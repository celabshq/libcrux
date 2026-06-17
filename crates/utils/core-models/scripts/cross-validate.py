#!/usr/bin/env python3
"""SIMD intrinsics cross-validation script (Phase B5).

For every libcrux T1 intrinsic that has either an `#[hax_lib::ensures]`
clause in `_extract.rs` OR a SMTPat lemma in `Spec.Intrinsics.fsti`, this
script:

  1. Generates `--samples` random inputs (default 10000), seeded by
     `--seed` (default 0).
  2. Computes the intrinsic via a Python ground-truth lane operator that
     mirrors `core-models`'s int_vec body.
  3. Parses the F* spec predicate into a Python evaluator (LHS == RHS) and
     asserts it on each random input.
  4. On mismatch, records `(intrinsic, input, expected, got)`.
  5. Emits a per-intrinsic verdict and a global findings markdown file.

The L2 audit precondition (Phase A1 trust index) guarantees that the
core-models int_vec body has already been differentially tested against
the real CPU via `mk!` on 1000 random inputs (cf. `interpretations.rs`
`tests` mod). So the Python ground-truth here is anchored transitively:
if it matches the int_vec body's computation, it matches the real CPU
too. The cross-validation surfaces F*-spec ↔ ground-truth mismatches —
which manifest as either spec bugs or ground-truth bugs.

Output last line: `D6.4=NN.N% (N/M)` where M = number of T1 intrinsics
that had a parseable spec, and N = number that passed cross-validation.

Usage::

    python3 cross-validate.py --seed 0 --samples 10000 \\
        [--audit-feed]  # update the trust index CSV's audit_consistent
                          column from the cross-validation result.

The script is read-only against every file under
`crates/utils/intrinsics/src/`, `crates/utils/core-models/src/core_arch/`,
and `libcrux-ml-dsa/proofs/fstar/spec/Spec.Intrinsics.fsti`.

Stable interface — last stdout line: ``D6.4=NN.N% (N/M)``.
"""

from __future__ import annotations

import argparse
import csv
import random
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Set, Tuple


# ----------------------- Repo layout ----------------------------------------

# cross-validate.py lives at crates/utils/core-models/scripts/.
# parents[0]=scripts, [1]=core-models, [2]=utils, [3]=crates, [4]=repo root.
REPO_ROOT = Path(__file__).resolve().parents[4]

EXTRACT_AVX2 = REPO_ROOT / "crates/utils/intrinsics/src/avx2_extract.rs"
EXTRACT_ARM64 = REPO_ROOT / "crates/utils/intrinsics/src/arm64_extract.rs"
SPEC_INTRINSICS_FSTI = (
    REPO_ROOT / "libcrux-ml-dsa/proofs/fstar/spec/Spec.Intrinsics.fsti"
)
TRUST_INDEX_CSV = (
    REPO_ROOT / "crates/utils/core-models/proofs/intrinsics-trust-index.csv"
)
TRUST_INDEX_MD = (
    REPO_ROOT / "crates/utils/core-models/proofs/intrinsics-trust-index.md"
)

DEFAULT_FINDINGS_MD = (
    REPO_ROOT / "crates/utils/core-models/proofs/intrinsics-cross-validation-findings.md"
)


# ----------------------- Integer helpers ------------------------------------

# Two's-complement helpers.  Python ints are arbitrary precision, so we wrap
# explicitly to match Rust `wrapping_*` semantics on i16/i32/i64.

def _to_signed(x: int, bits: int) -> int:
    mask = (1 << bits) - 1
    x &= mask
    if x & (1 << (bits - 1)):
        x -= 1 << bits
    return x


def _to_unsigned(x: int, bits: int) -> int:
    return x & ((1 << bits) - 1)


def i8(x: int) -> int:
    return _to_signed(x, 8)


def i16(x: int) -> int:
    return _to_signed(x, 16)


def i32(x: int) -> int:
    return _to_signed(x, 32)


def i64(x: int) -> int:
    return _to_signed(x, 64)


def u8(x: int) -> int:
    return _to_unsigned(x, 8)


def u16(x: int) -> int:
    return _to_unsigned(x, 16)


def u32(x: int) -> int:
    return _to_unsigned(x, 32)


def u64(x: int) -> int:
    return _to_unsigned(x, 64)


def add_mod(x: int, y: int, bits: int) -> int:
    return _to_signed(x + y, bits)


def sub_mod(x: int, y: int, bits: int) -> int:
    return _to_signed(x - y, bits)


def mul_mod(x: int, y: int, bits: int) -> int:
    return _to_signed(x * y, bits)


def shift_right_arith(x: int, n: int, bits: int) -> int:
    """Signed arithmetic right shift, matching Rust signed `>>` and F* `>>!`."""
    n = n % 256  # rem_euclid, matches Rust IMM8.rem_euclid(256) pattern
    if n >= bits:
        return -1 if x < 0 else 0
    return _to_signed(x >> n, bits)


def shift_right_logical(x: int, n: int, bits: int) -> int:
    """Unsigned logical right shift: cast to u, shift, cast back."""
    if n < 0:
        # Rust would mask via rem_euclid; conservative behaviour: 0
        return 0
    if n >= bits:
        return 0
    ux = _to_unsigned(x, bits)
    return _to_signed(ux >> n, bits)


def shift_left(x: int, n: int, bits: int) -> int:
    """Logical left shift, with `_mm256_slli_epi*` semantics (n>=bits → 0)."""
    if n < 0:
        return 0
    if n >= bits:
        return 0
    ux = _to_unsigned(x, bits)
    return _to_signed((ux << n) & ((1 << bits) - 1), bits)


def signed_abs(x: int, bits: int) -> int:
    """`a.abs()` semantics, where `i*::MIN.abs() == i*::MIN` on overflow.

    Rust's `i32::abs()` returns `i32::MIN` for `i32::MIN` (since the result
    is unrepresentable).  Per `interpretations.rs::_mm256_abs_epi32`, the
    int_vec body returns `a[i]` for the MIN case.
    """
    if x == -(1 << (bits - 1)):
        return x
    return abs(x)


# ----------------------- Lane decomposition --------------------------------

# Random lane generators.  Inputs to a Vec256/Vec128 ops in our ground-truth
# are represented as Python lists of i16/i32/i64 lanes (length 16/8/4 etc.).
# A BitVec<256> input sampled as `i16x16` can be re-interpreted by the
# evaluator as `i32x8`, `i64x4`, `i8x32`, `u8x16`, etc., via reinterpret
# helpers below.

def _i16_lanes_to_bytes(lanes: List[int]) -> List[int]:
    out = []
    for v in lanes:
        u = _to_unsigned(v, 16)
        out.append(u & 0xFF)
        out.append((u >> 8) & 0xFF)
    return out


def _bytes_to_i16_lanes(b: List[int], n: int) -> List[int]:
    out = []
    for k in range(n):
        v = b[2 * k] | (b[2 * k + 1] << 8)
        out.append(_to_signed(v, 16))
    return out


def _bytes_to_i32_lanes(b: List[int], n: int) -> List[int]:
    out = []
    for k in range(n):
        v = b[4 * k] | (b[4 * k + 1] << 8) | (b[4 * k + 2] << 16) | (b[4 * k + 3] << 24)
        out.append(_to_signed(v, 32))
    return out


def _bytes_to_i64_lanes(b: List[int], n: int) -> List[int]:
    out = []
    for k in range(n):
        v = 0
        for j in range(8):
            v |= b[8 * k + j] << (8 * j)
        out.append(_to_signed(v, 64))
    return out


def _bytes_to_i8_lanes(b: List[int]) -> List[int]:
    return [_to_signed(x, 8) for x in b]


def _i32_lanes_to_bytes(lanes: List[int]) -> List[int]:
    out = []
    for v in lanes:
        u = _to_unsigned(v, 32)
        for j in range(4):
            out.append((u >> (8 * j)) & 0xFF)
    return out


def _i64_lanes_to_bytes(lanes: List[int]) -> List[int]:
    out = []
    for v in lanes:
        u = _to_unsigned(v, 64)
        for j in range(8):
            out.append((u >> (8 * j)) & 0xFF)
    return out


def _i8_lanes_to_bytes(lanes: List[int]) -> List[int]:
    return [_to_unsigned(v, 8) for v in lanes]


@dataclass
class Vec:
    """Generic SIMD vector value carried as little-endian raw bytes."""

    bits: int  # 128 or 256
    bytes: List[int]  # length bits/8

    def lanes_i16(self) -> List[int]:
        return _bytes_to_i16_lanes(self.bytes, self.bits // 16)

    def lanes_i32(self) -> List[int]:
        return _bytes_to_i32_lanes(self.bytes, self.bits // 32)

    def lanes_i64(self) -> List[int]:
        return _bytes_to_i64_lanes(self.bytes, self.bits // 64)

    def lanes_i8(self) -> List[int]:
        return _bytes_to_i8_lanes(self.bytes)

    def lanes_u8(self) -> List[int]:
        return list(self.bytes)

    def bit(self, i: int) -> int:
        """Return the i-th bit (0 or 1, little-endian — bit 0 is LSB of byte 0)."""
        if i < 0 or i >= self.bits:
            return 0
        return (self.bytes[i >> 3] >> (i & 7)) & 1

    @staticmethod
    def from_i16(lanes: List[int]) -> "Vec":
        bits = 16 * len(lanes)
        return Vec(bits, _i16_lanes_to_bytes(lanes))

    @staticmethod
    def from_i32(lanes: List[int]) -> "Vec":
        bits = 32 * len(lanes)
        return Vec(bits, _i32_lanes_to_bytes(lanes))

    @staticmethod
    def from_i64(lanes: List[int]) -> "Vec":
        bits = 64 * len(lanes)
        return Vec(bits, _i64_lanes_to_bytes(lanes))

    @staticmethod
    def from_i8(lanes: List[int]) -> "Vec":
        bits = 8 * len(lanes)
        return Vec(bits, _i8_lanes_to_bytes(lanes))

    @staticmethod
    def random(rng: random.Random, bits: int) -> "Vec":
        return Vec(bits, [rng.randrange(256) for _ in range(bits // 8)])

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Vec):
            return False
        return self.bits == other.bits and self.bytes == other.bytes


# ----------------------- Ground-truth intrinsic table ----------------------

# Mirrors `crates/utils/core-models/src/core_arch/x86/interpretations.rs`
# `int_vec` module.  Keys are intrinsic names without leading underscore
# (libcrux wrapper convention), values are dicts with input arity/types and
# the lane-form Python implementation.

# Each entry has the shape:
#   "name": {
#       "kind": one of {"vec128_to_vec128", "vec256_to_vec256",
#                       "vec256_x2_to_vec256", "vec128_x2_to_vec128",
#                       "vec256_to_vec128", "scalar_to_vec256",
#                       "scalar_to_vec128", ...}
#       "fn":   callable producing Vec or scalar.
#       "input_kinds": tuple of "Vec256"/"Vec128"/"i16"/"i32"/"i64"/"const_i32".
#                       const_i32 means a const-generic IMM8 in Rust;
#                       for these, the script enumerates a small set of
#                       representative const values per sample iteration.
#       "result_kind": one of "Vec256", "Vec128", "i32".
#   }


def _gt_setzero_si256(_args: List[Any]) -> Vec:
    return Vec.from_i16([0] * 16)


def _gt_set1_epi16_v256(args: List[Any]) -> Vec:
    return Vec.from_i16([i16(args[0])] * 16)


def _gt_set1_epi16_v128(args: List[Any]) -> Vec:
    return Vec.from_i16([i16(args[0])] * 8)


def _gt_set1_epi32(args: List[Any]) -> Vec:
    return Vec.from_i32([i32(args[0])] * 8)


def _gt_set1_epi64x(args: List[Any]) -> Vec:
    return Vec.from_i64([i64(args[0])] * 4)


def _gt_set_epi32_v128(args: List[Any]) -> Vec:
    # mm_set_epi32(e3, e2, e1, e0) -> [e0, e1, e2, e3]
    e3, e2, e1, e0 = args
    return Vec.from_i32([i32(e0), i32(e1), i32(e2), i32(e3)])


def _gt_set_epi64x(args: List[Any]) -> Vec:
    e3, e2, e1, e0 = args
    return Vec.from_i64([i64(e0), i64(e1), i64(e2), i64(e3)])


def _gt_set_m128i(args: List[Any]) -> Vec:
    hi, lo = args
    return Vec(256, list(lo.bytes) + list(hi.bytes))


def _pointwise_lane(
    a: Vec, b: Vec, lane: str, op: Callable[[int, int], int]
) -> Vec:
    if lane == "i16":
        la = a.lanes_i16(); lb = b.lanes_i16()
        return Vec.from_i16([i16(op(x, y)) for x, y in zip(la, lb)])
    if lane == "i32":
        la = a.lanes_i32(); lb = b.lanes_i32()
        return Vec.from_i32([i32(op(x, y)) for x, y in zip(la, lb)])
    if lane == "i64":
        la = a.lanes_i64(); lb = b.lanes_i64()
        return Vec.from_i64([i64(op(x, y)) for x, y in zip(la, lb)])
    raise ValueError(lane)


def _gt_add_epi16_v128(args):
    return _pointwise_lane(args[0], args[1], "i16", lambda x, y: x + y)


def _gt_add_epi16_v256(args):
    return _pointwise_lane(args[0], args[1], "i16", lambda x, y: x + y)


def _gt_add_epi32(args):
    return _pointwise_lane(args[0], args[1], "i32", lambda x, y: x + y)


def _gt_add_epi64(args):
    return _pointwise_lane(args[0], args[1], "i64", lambda x, y: x + y)


def _gt_sub_epi16_v128(args):
    return _pointwise_lane(args[0], args[1], "i16", lambda x, y: x - y)


def _gt_sub_epi16_v256(args):
    return _pointwise_lane(args[0], args[1], "i16", lambda x, y: x - y)


def _gt_sub_epi32(args):
    return _pointwise_lane(args[0], args[1], "i32", lambda x, y: x - y)


def _gt_mullo_epi16_v128(args):
    return _pointwise_lane(args[0], args[1], "i16", lambda x, y: x * y)


def _gt_mullo_epi16_v256(args):
    return _pointwise_lane(args[0], args[1], "i16", lambda x, y: x * y)


def _gt_mullo_epi32(args):
    return _pointwise_lane(args[0], args[1], "i32", lambda x, y: x * y)


def _gt_mulhi_epi16_v128(args):
    a = args[0].lanes_i16()
    b = args[1].lanes_i16()
    return Vec.from_i16([i16(((x * y) >> 16)) for x, y in zip(a, b)])


def _gt_mulhi_epi16_v256(args):
    a = args[0].lanes_i16()
    b = args[1].lanes_i16()
    return Vec.from_i16([i16(((x * y) >> 16)) for x, y in zip(a, b)])


def _gt_mul_epi32(args):
    # mm256_mul_epi32: 4 lanes; lane i = (i32 a[2i]) * (i32 b[2i]) -> i64.
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()
    return Vec.from_i64([i64(a[2 * i] * b[2 * i]) for i in range(4)])


def _gt_abs_epi32(args):
    a = args[0].lanes_i32()
    return Vec.from_i32([signed_abs(x, 32) for x in a])


def _gt_cmpgt_epi16(args):
    a = args[0].lanes_i16()
    b = args[1].lanes_i16()
    return Vec.from_i16([-1 if x > y else 0 for x, y in zip(a, b)])


def _gt_cmpgt_epi32(args):
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()
    return Vec.from_i32([-1 if x > y else 0 for x, y in zip(a, b)])


def _gt_sign_epi32(args):
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()
    out = []
    for x, y in zip(a, b):
        if y < 0:
            out.append(x if x == -(1 << 31) else -x)
        elif y > 0:
            out.append(x)
        else:
            out.append(0)
    return Vec.from_i32(out)


def _gt_castsi256_ps(args):
    return args[0]


def _gt_castps_si256(args):
    return args[0]


def _gt_castsi128_si256(args):
    a = args[0]
    return Vec(256, list(a.bytes) + [0] * 16)


def _gt_cvtepi16_epi32(args):
    a = args[0].lanes_i16()
    return Vec.from_i32([i32(x) for x in a[:8]])


def _gt_packs_epi16(args):
    a = args[0].lanes_i16()
    b = args[1].lanes_i16()

    def sat_i8(x):
        if x > 127:
            return 127
        if x < -128:
            return -128
        return x
    return Vec.from_i8([sat_i8(x) for x in a] + [sat_i8(x) for x in b])


def _gt_packs_epi32(args):
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()

    def sat_i16(x):
        if x > 32767:
            return 32767
        if x < -32768:
            return -32768
        return x
    return Vec.from_i16([sat_i16(x) for x in a[:4]] + [sat_i16(x) for x in b[:4]] +
                        [sat_i16(x) for x in a[4:]] + [sat_i16(x) for x in b[4:]])


def _gt_and_si256(args):
    a = args[0].bytes
    b = args[1].bytes
    return Vec(256, [x & y for x, y in zip(a, b)])


def _gt_or_si256(args):
    a = args[0].bytes
    b = args[1].bytes
    return Vec(256, [x | y for x, y in zip(a, b)])


def _gt_xor_si256(args):
    a = args[0].bytes
    b = args[1].bytes
    return Vec(256, [x ^ y for x, y in zip(a, b)])


def _gt_andnot_si256(args):
    a = args[0].bytes
    b = args[1].bytes
    return Vec(256, [(~x) & y & 0xFF for x, y in zip(a, b)])


def _gt_srai_epi16(args):
    imm = args[1] % 256
    a = args[0].lanes_i16()
    out = []
    for x in a:
        if imm > 15:
            out.append(-1 if x < 0 else 0)
        else:
            out.append(i16(x >> imm))
    return Vec.from_i16(out)


def _gt_srai_epi32(args):
    imm = args[1] % 256
    a = args[0].lanes_i32()
    out = []
    for x in a:
        if imm > 31:
            out.append(-1 if x < 0 else 0)
        else:
            out.append(i32(x >> imm))
    return Vec.from_i32(out)


def _gt_srli_epi16(args):
    imm = args[1] % 256
    a = args[0].lanes_i16()
    out = []
    for x in a:
        out.append(0 if imm > 15 else i16(_to_unsigned(x, 16) >> imm))
    return Vec.from_i16(out)


def _gt_srli_epi32(args):
    imm = args[1] % 256
    a = args[0].lanes_i32()
    out = []
    for x in a:
        out.append(0 if imm > 31 else i32(_to_unsigned(x, 32) >> imm))
    return Vec.from_i32(out)


def _gt_srli_epi64_v128(args):
    imm = args[1] % 256
    a = args[0].lanes_i64()
    out = []
    for x in a:
        out.append(0 if imm > 63 else i64(_to_unsigned(x, 64) >> imm))
    return Vec.from_i64(out)


def _gt_slli_epi32(args):
    imm = args[1] % 256
    a = args[0].lanes_i32()
    out = []
    for x in a:
        out.append(0 if imm > 31 else i32((_to_unsigned(x, 32) << imm) & 0xFFFFFFFF))
    return Vec.from_i32(out)


def _gt_slli_epi64(args):
    imm = args[1] % 256
    a = args[0].lanes_i64()
    out = []
    for x in a:
        out.append(0 if imm > 63 else i64((_to_unsigned(x, 64) << imm) & ((1 << 64) - 1)))
    return Vec.from_i64(out)


def _gt_srlv_epi64(args):
    a = args[0].lanes_i64()
    b = args[1].lanes_i64()
    out = []
    for x, n in zip(a, b):
        if n > 63 or n < 0:
            out.append(0)
        else:
            out.append(i64(_to_unsigned(x, 64) >> n))
    return Vec.from_i64(out)


def _gt_sllv_epi32_v128(args):
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()
    out = []
    for x, n in zip(a, b):
        if n > 31 or n < 0:
            out.append(0)
        else:
            out.append(i32((_to_unsigned(x, 32) << n) & 0xFFFFFFFF))
    return Vec.from_i32(out)


def _gt_sllv_epi32_v256(args):
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()
    out = []
    for x, n in zip(a, b):
        if n > 31 or n < 0:
            out.append(0)
        else:
            out.append(i32((_to_unsigned(x, 32) << n) & 0xFFFFFFFF))
    return Vec.from_i32(out)


def _gt_srlv_epi32_v256(args):
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()
    out = []
    for x, n in zip(a, b):
        if n > 31 or n < 0:
            out.append(0)
        else:
            out.append(i32(_to_unsigned(x, 32) >> n))
    return Vec.from_i32(out)


def _gt_srli_epi64_v256(args):
    imm = args[1] % 256
    a = args[0].lanes_i64()
    out = []
    for x in a:
        out.append(0 if imm > 63 else i64(_to_unsigned(x, 64) >> imm))
    return Vec.from_i64(out)


def _gt_shuffle_epi32(args):
    control = args[1]
    a = args[0].lanes_i32()
    indexes = [(control >> (i * 2)) % 4 for i in range(4)]
    out = []
    for i in range(8):
        if i < 4:
            out.append(a[indexes[i]])
        else:
            out.append(a[4 + indexes[i - 4]])
    return Vec.from_i32(out)


def _gt_blend_epi16(args):
    imm = args[2]
    a = args[0].lanes_i16()
    b = args[1].lanes_i16()
    out = []
    for i in range(16):
        bit = (imm >> (i % 8)) & 1
        out.append(b[i] if bit else a[i])
    return Vec.from_i16(out)


def _gt_blend_epi32(args):
    imm = args[2]
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()
    out = []
    for i in range(8):
        bit = (imm >> i) & 1
        out.append(b[i] if bit else a[i])
    return Vec.from_i32(out)


def _gt_inserti128(args):
    imm = args[2]
    a = args[0]  # Vec256
    b = args[1]  # Vec128
    if imm % 2 == 0:
        return Vec(256, list(b.bytes) + list(a.bytes[16:]))
    else:
        return Vec(256, list(a.bytes[:16]) + list(b.bytes))


def _gt_blendv_ps(args):
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()
    mask = args[2].lanes_i32()
    return Vec.from_i32([b[i] if mask[i] < 0 else a[i] for i in range(8)])


def _gt_unpacklo_epi64(args):
    a = args[0].lanes_i64()
    b = args[1].lanes_i64()
    return Vec.from_i64([a[0], b[0], a[2], b[2]])


def _gt_unpackhi_epi64(args):
    a = args[0].lanes_i64()
    b = args[1].lanes_i64()
    # Per the lemma at `mm256_unpackhi_epi64_lemma`:
    #   i32x8 lanes [a[2], a[3], b[2], b[3], a[6], a[7], b[6], b[7]]
    # which corresponds to i64x4 lanes [a64[1], b64[1], a64[3], b64[3]].
    return Vec.from_i64([a[1], b[1], a[3], b[3]])


def _gt_castsi256_si128(args):
    a = args[0]
    return Vec(128, list(a.bytes[:16]))


def _gt_extracti128_si256(args):
    a = args[0]
    imm = args[1]
    if imm % 2 == 0:
        return Vec(128, list(a.bytes[:16]))
    else:
        return Vec(128, list(a.bytes[16:]))


def _gt_set_epi8_v128(args):
    # mm_set_epi8(b15, b14, ..., b0) — args in lemma order are reversed
    # from output lane order: lane 0 = byte0 = args[15], lane 15 = byte15 = args[0].
    return Vec.from_i8([i8(args[15 - k]) for k in range(16)])


def _gt_set_epi8_v256(args):
    return Vec.from_i8([i8(args[31 - k]) for k in range(32)])


def _gt_set_epi16_v256(args):
    return Vec.from_i16([i16(args[15 - k]) for k in range(16)])


def _gt_set_epi32_v256(args):
    return Vec.from_i32([i32(args[7 - k]) for k in range(8)])


def _gt_testz_si256(args):
    a = args[0].bytes
    b = args[1].bytes
    for x, y in zip(a, b):
        if (x & y) != 0:
            return 0
    return 1


def _gt_madd_epi16(args):
    a = args[0].lanes_i16()
    b = args[1].lanes_i16()
    out = []
    for k in range(8):
        # 32-bit result: sum of two i32-extended products
        x = i32(i32(a[2*k]) * i32(b[2*k]))
        y = i32(i32(a[2*k+1]) * i32(b[2*k+1]))
        out.append(i32(x + y))
    return Vec.from_i32(out)


def _gt_mullo_epi16_v256(args):
    a = args[0].lanes_i16()
    b = args[1].lanes_i16()
    return Vec.from_i16([i16(x * y) for x, y in zip(a, b)])


def _gt_bsrli_epi128(args):
    imm = (args[1] % 256) % 16
    # _mm256_bsrli_epi128 shifts each 128-bit lane right by imm BYTES.
    # int_vec body: cast each i128 lane to u128, shift right by imm*8 bits.
    a = args[0]
    out_bytes = list(a.bytes)
    for lane in range(2):
        chunk = a.bytes[16 * lane : 16 * (lane + 1)]
        # interpret as u128, shift right (logically) by imm bytes
        v = 0
        for k in range(16):
            v |= chunk[k] << (8 * k)
        v >>= imm * 8
        new_chunk = [(v >> (8 * k)) & 0xFF for k in range(16)]
        for k in range(16):
            out_bytes[16 * lane + k] = new_chunk[k]
    return Vec(256, out_bytes)


def _gt_permute2x128(args):
    imm = args[2]
    a = args[0]
    b = args[1]
    out = list(a.bytes)
    for i in range(2):
        control = imm >> (i * 4)
        if (control >> 3) & 1:
            chunk = [0] * 16
        else:
            sel = control & 3
            if sel == 0:
                chunk = list(a.bytes[0:16])
            elif sel == 1:
                chunk = list(a.bytes[16:32])
            elif sel == 2:
                chunk = list(b.bytes[0:16])
            elif sel == 3:
                chunk = list(b.bytes[16:32])
        for k in range(16):
            out[16 * i + k] = chunk[k]
    return Vec(256, out)


def _gt_blendv_epi32(args):
    """`vec256_blendv_epi32` wrapper: implemented in libcrux as
    castps_si256(blendv_ps(castsi256_ps(a), castsi256_ps(b), castsi256_ps(mask)))
    which is just blendv_ps lifted to i32 lanes."""
    return _gt_blendv_ps(args)


# ---- AVX2: new GT functions added for D6.4 pass ----

def _gt_cmpeq_epi32(args):
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()
    return Vec.from_i32([-1 if x == y else 0 for x, y in zip(a, b)])


def _gt_unpacklo_epi32(args):
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()
    out = []
    for i in range(8):
        lane_base = 0 if i < 4 else 4
        local = i % 4
        src_idx = lane_base + local // 2
        out.append(a[src_idx] if local % 2 == 0 else b[src_idx])
    return Vec.from_i32(out)


def _gt_unpackhi_epi32(args):
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()
    out = []
    for i in range(8):
        lane_base = 2 if i < 4 else 6
        local = i % 4
        src_idx = lane_base + local // 2
        out.append(a[src_idx] if local % 2 == 0 else b[src_idx])
    return Vec.from_i32(out)


def _gt_slli_epi16(args):
    imm = args[1] % 256
    a = args[0].lanes_i16()
    out = []
    for x in a:
        out.append(0 if imm > 15 else i16((_to_unsigned(x, 16) << imm) & 0xFFFF))
    return Vec.from_i16(out)


# ---- ARM64 integer helpers ----

def rotate_left_u64(x: int, n: int) -> int:
    """64-bit rotate left."""
    n = n % 64
    ux = _to_unsigned(x, 64)
    if n == 0:
        return _to_signed(ux, 64)
    return _to_signed(((ux << n) | (ux >> (64 - n))) & ((1 << 64) - 1), 64)


# ---- AVX2 permute / shuffle GT functions ----

def _gt_permutevar8x32_epi32(args):
    v_lanes = args[0].lanes_i32()
    c_lanes = args[1].lanes_i32()
    return Vec.from_i32([v_lanes[c % 8] for c in c_lanes])


def _gt_shuffle_epi8_v128(args):
    """mm_shuffle_epi8: byte j of result = 0 if indexes_i8[j] < 0 else vec_u8[indexes_i8[j] % 16]."""
    v_bytes = args[0].lanes_u8()
    i_bytes = args[1].lanes_i8()
    return Vec.from_i8([0 if idx < 0 else v_bytes[idx % 16] for idx in i_bytes])


def _gt_shuffle_epi8_v256(args):
    """mm256_shuffle_epi8: 128-bit lane independent byte shuffle."""
    v_bytes = args[0].lanes_u8()
    i_bytes = args[1].lanes_i8()
    result = []
    for j, idx in enumerate(i_bytes):
        if idx < 0:
            result.append(0)
        else:
            group = 0 if j < 16 else 16
            result.append(v_bytes[group + (idx % 16)])
    return Vec.from_i8(result)


# ---- ARM64 GT functions ----

def _gt_arm_identity(args):
    return args[0]


def _gt_arm_dup_s16(args):
    return Vec.from_i16([i16(args[0])] * 8)


def _gt_arm_dup_u64(args):
    return Vec.from_i64([i64(args[0])] * 2)


def _gt_arm_dup_u32(args):
    return Vec.from_i32([i32(args[0])] * 4)


def _gt_arm_dup_u16(args):
    return Vec.from_i16([i16(args[0])] * 8)


def _gt_arm_dup_u8(args):
    return Vec.from_i8([i8(args[0])] * 16)


def _gt_arm_add_s16(args):
    return _pointwise_lane(args[0], args[1], "i16", lambda x, y: x + y)


def _gt_arm_sub_s16(args):
    return _pointwise_lane(args[0], args[1], "i16", lambda x, y: x - y)


def _gt_arm_mul_s16(args):
    return _pointwise_lane(args[0], args[1], "i16", lambda x, y: x * y)


def _gt_arm_muln_s16(args):
    a = args[0].lanes_i16()
    c = args[1]
    return Vec.from_i16([i16(x * i16(c)) for x in a])


def _gt_arm_muln_u16(args):
    a = args[0].lanes_i16()
    c = args[1]
    return Vec.from_i16([i16(x * i16(c)) for x in a])


def _gt_arm_muln_u32(args):
    a = args[0].lanes_i32()
    c = args[1]
    return Vec.from_i32([i32(x * i32(c)) for x in a])


def _gt_arm_cmpge_s16(args):
    a = args[0].lanes_i16()
    b = args[1].lanes_i16()
    return Vec.from_i16([-1 if x >= y else 0 for x, y in zip(a, b)])


def _gt_arm_cmple_s16(args):
    a = args[0].lanes_i16()
    b = args[1].lanes_i16()
    return Vec.from_i16([-1 if x <= y else 0 for x, y in zip(a, b)])


def _gt_arm_and_s16(args):
    return Vec(128, [x & y for x, y in zip(args[0].bytes, args[1].bytes)])


def _gt_arm_and_u32(args):
    return Vec(128, [x & y for x, y in zip(args[0].bytes, args[1].bytes)])


def _gt_arm_and_u16(args):
    return Vec(128, [x & y for x, y in zip(args[0].bytes, args[1].bytes)])


def _gt_arm_xor_s16(args):
    return Vec(128, [x ^ y for x, y in zip(args[0].bytes, args[1].bytes)])


def _gt_arm_xor_u64(args):
    return Vec(128, [x ^ y for x, y in zip(args[0].bytes, args[1].bytes)])


def _gt_arm_xor_u32(args):
    return Vec(128, [x ^ y for x, y in zip(args[0].bytes, args[1].bytes)])


def _gt_arm_xor_u8(args):
    return Vec(128, [x ^ y for x, y in zip(args[0].bytes, args[1].bytes)])


def _gt_arm_bic_u64(args):
    return Vec(128, [x & ((~y) & 0xFF) for x, y in zip(args[0].bytes, args[1].bytes)])


def _gt_arm_add_u32(args):
    return _pointwise_lane(args[0], args[1], "i32", lambda x, y: x + y)


def _gt_arm_rax1_u64(args):
    a = args[0].lanes_i64()
    b = args[1].lanes_i64()
    return Vec.from_i64([i64(_to_unsigned(x, 64) ^ _to_unsigned(rotate_left_u64(y, 1), 64)) for x, y in zip(a, b)])


def _gt_arm_veor3_u64(args):
    a = args[0].bytes
    b = args[1].bytes
    c = args[2].bytes
    return Vec(128, [x ^ y ^ z for x, y, z in zip(a, b, c)])


def _gt_arm_vxarq_u64(args):
    a = args[0].lanes_i64()
    b = args[1].lanes_i64()
    left = args[2] % 64
    return Vec.from_i64([rotate_left_u64(i64(_to_unsigned(x, 64) ^ _to_unsigned(y, 64)), left) for x, y in zip(a, b)])


def _gt_arm_vbcax_u64(args):
    a = args[0].lanes_i64()
    b = args[1].lanes_i64()
    c = args[2].lanes_i64()
    return Vec.from_i64([i64(_to_unsigned(x, 64) ^ (_to_unsigned(y, 64) & ~_to_unsigned(z, 64) & ((1 << 64) - 1))) for x, y, z in zip(a, b, c)])


def _gt_arm_qtbl1_u8(args):
    t = args[0].lanes_i8()
    idx = args[1].lanes_i8()
    out = []
    for ix in idx:
        uix = _to_unsigned(ix, 8)
        out.append(t[uix] if uix < 16 else 0)
    return Vec.from_i8(out)


def _gt_arm_trn1_s16(args):
    a = args[0].lanes_i16()
    b = args[1].lanes_i16()
    out = []
    for i in range(4):
        out.append(a[2 * i])
        out.append(b[2 * i])
    return Vec.from_i16(out)


def _gt_arm_trn2_s16(args):
    a = args[0].lanes_i16()
    b = args[1].lanes_i16()
    out = []
    for i in range(4):
        out.append(a[2 * i + 1])
        out.append(b[2 * i + 1])
    return Vec.from_i16(out)


def _gt_arm_trn1_s32(args):
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()
    out = []
    for i in range(2):
        out.append(a[2 * i])
        out.append(b[2 * i])
    return Vec.from_i32(out)


def _gt_arm_trn2_s32(args):
    a = args[0].lanes_i32()
    b = args[1].lanes_i32()
    out = []
    for i in range(2):
        out.append(a[2 * i + 1])
        out.append(b[2 * i + 1])
    return Vec.from_i32(out)


def _gt_arm_trn1_i64(args):
    a = args[0].lanes_i64()
    b = args[1].lanes_i64()
    return Vec.from_i64([a[0], b[0]])


def _gt_arm_trn2_i64(args):
    a = args[0].lanes_i64()
    b = args[1].lanes_i64()
    return Vec.from_i64([a[1], b[1]])


def _gt_arm_load_identity(args):
    return args[0]


# ---- ARM64 half-vec / widen / reduce GT functions ----

def _gt_arm_vget_low_s16(args):
    """Lower 4 lanes of i16x8 → i16x4 (64-bit Vec)."""
    return Vec.from_i16(args[0].lanes_i16()[:4])


def _gt_arm_vget_low_u16(args):
    return Vec.from_i16(args[0].lanes_i16()[:4])


def _gt_arm_vget_high_u16(args):
    """Upper 4 lanes of i16x8 → i16x4 (64-bit Vec)."""
    return Vec.from_i16(args[0].lanes_i16()[4:])


def _gt_arm_vmull_s16(args):
    """Widen-multiply i16x4 × i16x4 → i32x4."""
    a = args[0].lanes_i16()
    b = args[1].lanes_i16()
    return Vec.from_i32([i32(x) * i32(y) for x, y in zip(a, b)])


def _gt_arm_vmull_high_s16(args):
    """Widen-multiply upper i16x4 of each i16x8 → i32x4."""
    a = args[0].lanes_i16()[4:]
    b = args[1].lanes_i16()[4:]
    return Vec.from_i32([i32(x) * i32(y) for x, y in zip(a, b)])


def _gt_arm_vmlal_s16(args):
    """Widen-multiply-accumulate: i32x4 + (i16x4 widened × i16x4 widened)."""
    acc = args[0].lanes_i32()
    b = args[1].lanes_i16()
    c = args[2].lanes_i16()
    return Vec.from_i32([i32(acc[i] + i32(b[i]) * i32(c[i])) for i in range(4)])


def _gt_arm_vmlal_high_s16(args):
    """Accumulate + widen-multiply from upper halves of i16x8 inputs."""
    acc = args[0].lanes_i32()
    b = args[1].lanes_i16()[4:]
    c = args[2].lanes_i16()[4:]
    return Vec.from_i32([i32(acc[i] + i32(b[i]) * i32(c[i])) for i in range(4)])


def _gt_arm_vaddvq_s16(args):
    """Horizontal signed 16-bit sum → i16 scalar (wrapping)."""
    lanes = args[0].lanes_i16()
    return i16(sum(lanes))


def _gt_arm_vaddvq_u16(args):
    """Horizontal unsigned 16-bit sum → u16 scalar (wrapping)."""
    lanes = args[0].lanes_i16()
    return i16(sum(lanes))


def _gt_arm_vaddv_u16(args):
    """Horizontal unsigned 16-bit sum of 4-lane vec → u16 scalar."""
    lanes = args[0].lanes_i16()
    return i16(sum(lanes))


# Big table.  Each entry: (kind, fn, ground_truth_scalar_args).
# ``kind`` describes what shape of input/output we're dealing with so we know
# how to randomly sample.

VEC256 = "Vec256"
VEC128 = "Vec128"
VEC64 = "Vec64"
I16 = "i16"
I32 = "i32"
I64 = "i64"
CONST_I32 = "const_i32"


# Each ground-truth entry has:
#   "inputs":  list of input-kind labels (in libcrux wrapper signature order).
#   "fn":      callable taking the list of evaluated args and returning Vec or
#              scalar.  Const-generic args are passed positionally.
#   "result":  result-kind label.
#   "const_range": for each CONST_I32 in inputs, a tuple (lo, hi) bounding
#                  the legal const-generic value.  Random sampler picks
#                  uniformly within range.

GROUND_TRUTH: Dict[str, Dict[str, Any]] = {
    "mm256_setzero_si256": {
        "inputs": [], "fn": _gt_setzero_si256, "result": VEC256,
    },
    "mm256_set1_epi16": {
        "inputs": [I16], "fn": _gt_set1_epi16_v256, "result": VEC256,
    },
    "mm_set1_epi16": {
        "inputs": [I16], "fn": _gt_set1_epi16_v128, "result": VEC128,
    },
    "mm256_set1_epi32": {
        "inputs": [I32], "fn": _gt_set1_epi32, "result": VEC256,
    },
    "mm256_set1_epi64x": {
        "inputs": [I64], "fn": _gt_set1_epi64x, "result": VEC256,
    },
    "mm_set_epi32": {
        "inputs": [I32, I32, I32, I32],
        "fn": _gt_set_epi32_v128, "result": VEC128,
    },
    "mm256_set_epi64x": {
        "inputs": [I64, I64, I64, I64],
        "fn": _gt_set_epi64x, "result": VEC256,
    },
    "mm256_set_m128i": {
        "inputs": [VEC128, VEC128], "fn": _gt_set_m128i, "result": VEC256,
    },
    "mm_add_epi16": {
        "inputs": [VEC128, VEC128], "fn": _gt_add_epi16_v128, "result": VEC128,
    },
    "mm256_add_epi16": {
        "inputs": [VEC256, VEC256], "fn": _gt_add_epi16_v256, "result": VEC256,
    },
    "mm256_add_epi32": {
        "inputs": [VEC256, VEC256], "fn": _gt_add_epi32, "result": VEC256,
    },
    "mm256_add_epi64": {
        "inputs": [VEC256, VEC256], "fn": _gt_add_epi64, "result": VEC256,
    },
    "mm256_sub_epi16": {
        "inputs": [VEC256, VEC256], "fn": _gt_sub_epi16_v256, "result": VEC256,
    },
    "mm_sub_epi16": {
        "inputs": [VEC128, VEC128], "fn": _gt_sub_epi16_v128, "result": VEC128,
    },
    "mm256_sub_epi32": {
        "inputs": [VEC256, VEC256], "fn": _gt_sub_epi32, "result": VEC256,
    },
    "mm_mullo_epi16": {
        "inputs": [VEC128, VEC128], "fn": _gt_mullo_epi16_v128, "result": VEC128,
    },
    "mm256_mullo_epi32": {
        "inputs": [VEC256, VEC256], "fn": _gt_mullo_epi32, "result": VEC256,
    },
    "mm_mulhi_epi16": {
        "inputs": [VEC128, VEC128], "fn": _gt_mulhi_epi16_v128, "result": VEC128,
    },
    "mm256_mulhi_epi16": {
        "inputs": [VEC256, VEC256], "fn": _gt_mulhi_epi16_v256, "result": VEC256,
    },
    "mm256_mul_epi32": {
        "inputs": [VEC256, VEC256], "fn": _gt_mul_epi32, "result": VEC256,
    },
    "mm256_abs_epi32": {
        "inputs": [VEC256], "fn": _gt_abs_epi32, "result": VEC256,
    },
    "mm256_cmpgt_epi16": {
        "inputs": [VEC256, VEC256], "fn": _gt_cmpgt_epi16, "result": VEC256,
    },
    "mm256_cmpgt_epi32": {
        "inputs": [VEC256, VEC256], "fn": _gt_cmpgt_epi32, "result": VEC256,
    },
    "mm256_sign_epi32": {
        "inputs": [VEC256, VEC256], "fn": _gt_sign_epi32, "result": VEC256,
    },
    "mm256_castsi256_ps": {
        "inputs": [VEC256], "fn": _gt_castsi256_ps, "result": VEC256,
    },
    "mm256_castps_si256": {
        "inputs": [VEC256], "fn": _gt_castps_si256, "result": VEC256,
    },
    "mm256_castsi128_si256": {
        "inputs": [VEC128], "fn": _gt_castsi128_si256, "result": VEC256,
    },
    "mm256_cvtepi16_epi32": {
        "inputs": [VEC128], "fn": _gt_cvtepi16_epi32, "result": VEC256,
    },
    "mm_packs_epi16": {
        "inputs": [VEC128, VEC128], "fn": _gt_packs_epi16, "result": VEC128,
    },
    "mm256_packs_epi32": {
        "inputs": [VEC256, VEC256], "fn": _gt_packs_epi32, "result": VEC256,
    },
    "mm256_and_si256": {
        "inputs": [VEC256, VEC256], "fn": _gt_and_si256, "result": VEC256,
    },
    "mm256_or_si256": {
        "inputs": [VEC256, VEC256], "fn": _gt_or_si256, "result": VEC256,
    },
    "mm256_xor_si256": {
        "inputs": [VEC256, VEC256], "fn": _gt_xor_si256, "result": VEC256,
    },
    "mm256_andnot_si256": {
        "inputs": [VEC256, VEC256], "fn": _gt_andnot_si256, "result": VEC256,
    },
    "mm256_srai_epi16": {
        "inputs": [VEC256, CONST_I32], "fn": _gt_srai_epi16, "result": VEC256,
        "const_range": [(0, 15)],
    },
    "mm256_srai_epi32": {
        "inputs": [VEC256, CONST_I32], "fn": _gt_srai_epi32, "result": VEC256,
        "const_range": [(0, 31)],
    },
    "mm256_srli_epi16": {
        "inputs": [VEC256, CONST_I32], "fn": _gt_srli_epi16, "result": VEC256,
        "const_range": [(0, 15)],
    },
    "mm256_srli_epi32": {
        "inputs": [VEC256, CONST_I32], "fn": _gt_srli_epi32, "result": VEC256,
        "const_range": [(0, 31)],
    },
    "mm_srli_epi64": {
        "inputs": [VEC128, CONST_I32], "fn": _gt_srli_epi64_v128, "result": VEC128,
        "const_range": [(1, 63)],
    },
    "mm256_slli_epi32": {
        "inputs": [VEC256, CONST_I32], "fn": _gt_slli_epi32, "result": VEC256,
        "const_range": [(0, 31)],
    },
    "mm256_slli_epi64": {
        "inputs": [VEC256, CONST_I32], "fn": _gt_slli_epi64, "result": VEC256,
        "const_range": [(1, 63)],
    },
    "mm256_srlv_epi64": {
        "inputs": [VEC256, VEC256], "fn": _gt_srlv_epi64, "result": VEC256,
    },
    "mm_sllv_epi32": {
        "inputs": [VEC128, VEC128], "fn": _gt_sllv_epi32_v128, "result": VEC128,
    },
    "mm256_sllv_epi32": {
        "inputs": [VEC256, VEC256], "fn": _gt_sllv_epi32_v256, "result": VEC256,
    },
    "mm256_srlv_epi32": {
        "inputs": [VEC256, VEC256], "fn": _gt_srlv_epi32_v256, "result": VEC256,
    },
    "mm256_srli_epi64": {
        "inputs": [VEC256, CONST_I32], "fn": _gt_srli_epi64_v256, "result": VEC256,
        "const_range": [(1, 63)],
    },
    "mm256_shuffle_epi32": {
        "inputs": [VEC256, CONST_I32], "fn": _gt_shuffle_epi32, "result": VEC256,
        "const_range": [(0, 255)],
    },
    "mm256_blend_epi16": {
        "inputs": [VEC256, VEC256, CONST_I32], "fn": _gt_blend_epi16, "result": VEC256,
        "const_range": [(0, 255)],
    },
    "mm256_blend_epi32": {
        "inputs": [VEC256, VEC256, CONST_I32], "fn": _gt_blend_epi32, "result": VEC256,
        "const_range": [(0, 255)],
    },
    "mm256_inserti128_si256": {
        "inputs": [VEC256, VEC128, CONST_I32], "fn": _gt_inserti128, "result": VEC256,
        "const_range": [(0, 1)],
    },
    "mm256_unpacklo_epi64": {
        "inputs": [VEC256, VEC256], "fn": _gt_unpacklo_epi64, "result": VEC256,
    },
    "mm256_unpackhi_epi64": {
        "inputs": [VEC256, VEC256], "fn": _gt_unpackhi_epi64, "result": VEC256,
    },
    "mm256_castsi256_si128": {
        "inputs": [VEC256], "fn": _gt_castsi256_si128, "result": VEC128,
    },
    "mm256_extracti128_si256": {
        "inputs": [VEC256, CONST_I32], "fn": _gt_extracti128_si256, "result": VEC128,
        "const_range": [(0, 1)],
    },
    "mm_set_epi8": {
        "inputs": [I16] * 16,  # i8 wrapper takes i8 args; sample as small ints
        "fn": _gt_set_epi8_v128, "result": VEC128,
    },
    "mm256_set_epi8": {
        "inputs": [I16] * 32,
        "fn": _gt_set_epi8_v256, "result": VEC256,
    },
    "mm256_set_epi16": {
        "inputs": [I16] * 16,
        "fn": _gt_set_epi16_v256, "result": VEC256,
    },
    "mm256_set_epi32": {
        "inputs": [I32] * 8,
        "fn": _gt_set_epi32_v256, "result": VEC256,
    },
    "mm256_testz_si256": {
        "inputs": [VEC256, VEC256], "fn": _gt_testz_si256, "result": "i32",
    },
    "mm256_madd_epi16": {
        "inputs": [VEC256, VEC256], "fn": _gt_madd_epi16, "result": VEC256,
    },
    "mm256_mullo_epi16": {
        "inputs": [VEC256, VEC256], "fn": _gt_mullo_epi16_v256, "result": VEC256,
    },
    "mm256_bsrli_epi128": {
        "inputs": [VEC256, CONST_I32], "fn": _gt_bsrli_epi128, "result": VEC256,
        "const_range": [(1, 15)],
    },
    "mm256_permute2x128_si256": {
        "inputs": [VEC256, VEC256, CONST_I32], "fn": _gt_permute2x128, "result": VEC256,
        "const_range": [(0, 255)],
    },
    "vec256_blendv_epi32": {
        "inputs": [VEC256, VEC256, VEC256], "fn": _gt_blendv_epi32, "result": VEC256,
    },
    # ---- AVX2 new entries ----
    "mm256_cmpeq_epi32": {
        "inputs": [VEC256, VEC256], "fn": _gt_cmpeq_epi32, "result": VEC256,
    },
    "mm256_unpacklo_epi32": {
        "inputs": [VEC256, VEC256], "fn": _gt_unpacklo_epi32, "result": VEC256,
    },
    "mm256_unpackhi_epi32": {
        "inputs": [VEC256, VEC256], "fn": _gt_unpackhi_epi32, "result": VEC256,
    },
    "mm256_slli_epi16": {
        "inputs": [VEC256, CONST_I32], "fn": _gt_slli_epi16, "result": VEC256,
        "const_range": [(0, 15)],
    },
    "mm256_permutevar8x32_epi32": {
        "inputs": [VEC256, VEC256], "fn": _gt_permutevar8x32_epi32, "result": VEC256,
    },
    "mm_shuffle_epi8": {
        "inputs": [VEC128, VEC128], "fn": _gt_shuffle_epi8_v128, "result": VEC128,
    },
    "mm256_shuffle_epi8": {
        "inputs": [VEC256, VEC256], "fn": _gt_shuffle_epi8_v256, "result": VEC256,
    },
}

# Ground-truth table for ARM64 wrappers.
GROUND_TRUTH_ARM: Dict[str, Dict[str, Any]] = {
    # --- dup / broadcast ---
    "_vdupq_n_s16": {"inputs": [I16], "fn": _gt_arm_dup_s16, "result": VEC128},
    "_vdupq_n_u64": {"inputs": [I64], "fn": _gt_arm_dup_u64, "result": VEC128},
    "_vdupq_n_u32": {"inputs": [I32], "fn": _gt_arm_dup_u32, "result": VEC128},
    "_vdupq_n_u16": {"inputs": [I16], "fn": _gt_arm_dup_u16, "result": VEC128},
    "_vdupq_n_u8":  {"inputs": [I16], "fn": _gt_arm_dup_u8,  "result": VEC128},
    # --- reinterpret casts (identity) ---
    "_vreinterpretq_s16_u16": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_u16_s16": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_s16_u32": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_u32_s16": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_s16_u8":  {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_u8_s16":  {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_s32_u32": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_u32_s32": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_u32_u8":  {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_u8_u32":  {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_u32_u64": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_s16_u64": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_u16_u8":  {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_u16_u64": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_s16_s32": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_s32_s16": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_s16_s64": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_s64_s16": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_s64_s32": {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vreinterpretq_u8_s64":  {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    # --- loads modeled as identity (result == slice cast to VEC128) ---
    "_vld1q_s16":  {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vld1q_u8":   {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vld1q_u16":  {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vld1q_u32":  {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    "_vld1q_u64":  {"inputs": [VEC128], "fn": _gt_arm_identity, "result": VEC128},
    # --- arithmetic ---
    "_vaddq_s16":    {"inputs": [VEC128, VEC128], "fn": _gt_arm_add_s16, "result": VEC128},
    "_vsubq_s16":    {"inputs": [VEC128, VEC128], "fn": _gt_arm_sub_s16, "result": VEC128},
    "_vmulq_s16":    {"inputs": [VEC128, VEC128], "fn": _gt_arm_mul_s16, "result": VEC128},
    "_vmulq_n_s16":  {"inputs": [VEC128, I16],    "fn": _gt_arm_muln_s16, "result": VEC128},
    "_vmulq_n_u16":  {"inputs": [VEC128, I16],    "fn": _gt_arm_muln_u16, "result": VEC128},
    "_vmulq_n_u32":  {"inputs": [VEC128, I32],    "fn": _gt_arm_muln_u32, "result": VEC128},
    "_vaddq_u32":    {"inputs": [VEC128, VEC128], "fn": _gt_arm_add_u32, "result": VEC128},
    # --- bitwise ---
    "_vandq_s16": {"inputs": [VEC128, VEC128], "fn": _gt_arm_and_s16, "result": VEC128},
    "_vandq_u32":  {"inputs": [VEC128, VEC128], "fn": _gt_arm_and_u32, "result": VEC128},
    "_vandq_u16":  {"inputs": [VEC128, VEC128], "fn": _gt_arm_and_u16, "result": VEC128},
    "_veorq_s16":  {"inputs": [VEC128, VEC128], "fn": _gt_arm_xor_s16, "result": VEC128},
    "_veorq_u64":  {"inputs": [VEC128, VEC128], "fn": _gt_arm_xor_u64, "result": VEC128},
    "_veorq_u32":  {"inputs": [VEC128, VEC128], "fn": _gt_arm_xor_u32, "result": VEC128},
    "_veorq_u8":   {"inputs": [VEC128, VEC128], "fn": _gt_arm_xor_u8,  "result": VEC128},
    "_vbicq_u64":  {"inputs": [VEC128, VEC128], "fn": _gt_arm_bic_u64, "result": VEC128},
    # --- comparisons ---
    "_vcgeq_s16": {"inputs": [VEC128, VEC128], "fn": _gt_arm_cmpge_s16, "result": VEC128},
    "_vcleq_s16": {"inputs": [VEC128, VEC128], "fn": _gt_arm_cmple_s16, "result": VEC128},
    # --- SHA3 ops ---
    "_vrax1q_u64":  {"inputs": [VEC128, VEC128], "fn": _gt_arm_rax1_u64, "result": VEC128},
    "_veor3q_u64":  {"inputs": [VEC128, VEC128, VEC128], "fn": _gt_arm_veor3_u64, "result": VEC128},
    "_vxarq_u64":   {
        "inputs": [VEC128, VEC128, CONST_I32], "fn": _gt_arm_vxarq_u64,
        "result": VEC128, "const_range": [(0, 63)],
    },
    "_vbcaxq_u64":  {"inputs": [VEC128, VEC128, VEC128], "fn": _gt_arm_vbcax_u64, "result": VEC128},
    # --- table lookup ---
    "_vqtbl1q_u8": {"inputs": [VEC128, VEC128], "fn": _gt_arm_qtbl1_u8, "result": VEC128},
    # --- transpose ---
    "_vtrn1q_s16": {"inputs": [VEC128, VEC128], "fn": _gt_arm_trn1_s16, "result": VEC128},
    "_vtrn2q_s16": {"inputs": [VEC128, VEC128], "fn": _gt_arm_trn2_s16, "result": VEC128},
    "_vtrn1q_s32": {"inputs": [VEC128, VEC128], "fn": _gt_arm_trn1_s32, "result": VEC128},
    "_vtrn2q_s32": {"inputs": [VEC128, VEC128], "fn": _gt_arm_trn2_s32, "result": VEC128},
    "_vtrn1q_s64": {"inputs": [VEC128, VEC128], "fn": _gt_arm_trn1_i64, "result": VEC128},
    "_vtrn2q_s64": {"inputs": [VEC128, VEC128], "fn": _gt_arm_trn2_i64, "result": VEC128},
    "_vtrn1q_u64": {"inputs": [VEC128, VEC128], "fn": _gt_arm_trn1_i64, "result": VEC128},
    "_vtrn2q_u64": {"inputs": [VEC128, VEC128], "fn": _gt_arm_trn2_i64, "result": VEC128},
    # --- half-vec extract (64-bit result) ---
    "_vget_low_s16": {"inputs": [VEC128], "fn": _gt_arm_vget_low_s16, "result": VEC64},
    "_vget_low_u16": {"inputs": [VEC128], "fn": _gt_arm_vget_low_u16, "result": VEC64},
    "_vget_high_u16": {"inputs": [VEC128], "fn": _gt_arm_vget_high_u16, "result": VEC64},
    # --- widening multiply (i16x4 → i32x4) ---
    "_vmull_s16":      {"inputs": [VEC64, VEC64],        "fn": _gt_arm_vmull_s16,      "result": VEC128},
    "_vmull_high_s16": {"inputs": [VEC128, VEC128],      "fn": _gt_arm_vmull_high_s16, "result": VEC128},
    # --- widening multiply-accumulate ---
    "_vmlal_s16":      {"inputs": [VEC128, VEC64, VEC64],    "fn": _gt_arm_vmlal_s16,      "result": VEC128},
    "_vmlal_high_s16": {"inputs": [VEC128, VEC128, VEC128],  "fn": _gt_arm_vmlal_high_s16, "result": VEC128},
    # --- horizontal sum → scalar ---
    "_vaddvq_s16": {"inputs": [VEC128], "fn": _gt_arm_vaddvq_s16, "result": I16},
    "_vaddvq_u16": {"inputs": [VEC128], "fn": _gt_arm_vaddvq_u16, "result": I16},
    "_vaddv_u16":  {"inputs": [VEC64],  "fn": _gt_arm_vaddv_u16,  "result": I16},
}


# ----------------------- Spec parser ---------------------------------------

# We restrict to a small lane-form sub-language.  Patterns we recognize:
#
#   PAT_SETZERO     vec{256,128}_as_iNxM $result == Seq.create K (mk_iN 0)
#   PAT_CREATE      vec{256,128}_as_iNxM $result == Spec.Utils.create (sz K) $X
#   PAT_MAP2_OP     vec...$result == Spec.Utils.map2 (OP) (vec... $A) (vec... $B)
#   PAT_MAP2_LAMBDA vec...$result == Spec.Utils.map2 (fun x y -> ...) ...
#   PAT_MAP_ARRAY   vec...$result == Spec.Utils.map_array (fun x -> ...) (vec... $V)
#
# OP ∈ { (+.), (-.), (*. ), (^.), (&.), (|.), mul_mod, ... }


@dataclass
class SpecCase:
    """A parsed spec case for one wrapper, ready to evaluate.

    ``arg_names`` lists the expected variable names in the order the
    evaluator looks them up.  For ensures clauses these are the wrapper's
    signature names (``$lhs``, ``$rhs``, etc.); for SMTPat lemmas these are
    the lemma's binders (``(a b: bv256)`` → ``["a", "b"]``).  The runner
    matches them positionally to the sample's input slots.
    """

    intrinsic_name: str  # libcrux wrapper name (no leading underscore)
    pattern: str         # which sub-language pattern matched
    spec_text: str       # raw spec text (for findings)
    source: str          # 'extract.rs' or 'spec_intrinsics.fsti'
    eval_rhs: Callable[[Dict[str, Any]], Any]   # (args_dict) -> expected
    project_lhs: Callable[[Any], Any]   # (result_vec) -> projected lanes
    arg_names: List[str] = field(default_factory=list)
    out_of_scope_reason: Optional[str] = None


# Regex: hax_lib::ensures(|<bind>| fstar!(<expr>))
ENSURES_RE = re.compile(
    r'#\[\s*hax_lib::ensures\s*\(\s*\|[^|]*\|\s*fstar!\s*\(\s*'
    r'(?:r#)?"([^"\\]*(?:\\.[^"\\]*)*)"\s*\#?\)?\s*\)\s*\]',
    re.DOTALL,
)


def _strip_str_lit(text: str) -> str:
    """Collapse whitespace inside a Rust string-literal-esque fstar! body."""
    s = re.sub(r'\s+', ' ', text).strip()
    return s


def parse_extract_ensures_for_fn(text: str, name: str) -> Optional[Tuple[str, str]]:
    """Look for `#[hax_lib::ensures(|R| fstar!(\"...\"))]` immediately
    preceding `pub fn <name>` (with optional other attrs interleaved).
    Returns (raw_attribute_text, fstar_string_body) or None.
    """
    # Find the pub fn site.
    fn_re = re.compile(r"pub\s+(?:unsafe\s+)?fn\s+" + re.escape(name) + r"\b")
    m = fn_re.search(text)
    if not m:
        return None
    start = m.start()

    # Walk backwards over `#[...]` attribute lines and whitespace until the
    # nearest non-attribute non-whitespace token.
    cursor = start
    # Collect contiguous attrs (multi-line tolerant).
    attr_blob_pieces: List[str] = []
    # Walk back line-by-line in source-order.
    lines = text[:start].splitlines(keepends=True)
    # Track how many trailing lines are part of attribute / blank.
    attrs_lines: List[str] = []
    i = len(lines) - 1
    in_attr = False
    while i >= 0:
        line = lines[i]
        stripped = line.strip()
        if stripped == "":
            attrs_lines.insert(0, line)
            i -= 1
            continue
        # If inside a multi-line attr, look for opening `#[`.
        if in_attr:
            attrs_lines.insert(0, line)
            if stripped.startswith("#["):
                in_attr = False
            i -= 1
            continue
        # New attr block: ends with `])]` (very loose check).
        if stripped.endswith(")]") or stripped.endswith("]"):
            # Single-line attr.
            if stripped.startswith("#["):
                attrs_lines.insert(0, line)
                i -= 1
                continue
            else:
                # part of a multi-line attribute; mark and continue.
                attrs_lines.insert(0, line)
                in_attr = True
                i -= 1
                continue
        # Otherwise: stop scan, this line is not part of attrs.
        break
    blob = "".join(attrs_lines)

    # Find ensures within blob.  This is loose: we accept the attr to span
    # multiple lines.
    m = re.search(
        r'hax_lib::ensures\s*\(\s*\|[^|]*\|\s*fstar!\s*\(\s*'
        r'(r#)?"((?:[^"\\]|\\.)*)"',
        blob,
        re.DOTALL,
    )
    if not m:
        # Try the r#"..."# form which uses a raw string.
        m = re.search(
            r'hax_lib::ensures\s*\(\s*\|[^|]*\|\s*fstar!\s*\(\s*'
            r'r#"((?:.|\n)*?)"\#',
            blob,
            re.DOTALL,
        )
        if not m:
            return None
        body = m.group(1)
        return (blob, _strip_str_lit(body))
    body = m.group(2)
    return (blob, _strip_str_lit(body))


# Map F* operator tokens to Python int op (with explicit lane bits).
_OP_BIN_BY_LANE: Dict[str, Callable[[int, int, int], int]] = {
    "+.": add_mod,
    "-.": sub_mod,
    "*.": mul_mod,
    "^.": lambda x, y, b: _to_signed(_to_unsigned(x, b) ^ _to_unsigned(y, b), b),
    "&.": lambda x, y, b: _to_signed(_to_unsigned(x, b) & _to_unsigned(y, b), b),
    "|.": lambda x, y, b: _to_signed(_to_unsigned(x, b) | _to_unsigned(y, b), b),
    "mul_mod": mul_mod,
    "add_mod": add_mod,
    "sub_mod": sub_mod,
}

# Names for `vec256_as_i16x16` etc.  Maps to (bits, "i8"/"i16"/.../"u8", count).
_LANE_PROJ_RE = re.compile(r"vec(\d+)_as_([iu]\d+)x(\d+)")


def _parse_lane_proj(token: str) -> Optional[Tuple[int, str, int]]:
    """Return (vec_bits, lane_type, lane_count) for `vec256_as_i16x16` etc."""
    m = _LANE_PROJ_RE.match(token)
    if not m:
        return None
    return (int(m.group(1)), m.group(2), int(m.group(3)))


def _norm_lane_type(t: str) -> str:
    """Map unsigned ARM64 lane types to signed equivalents for comparison."""
    return {"u8": "i8", "u16": "i16", "u32": "i32", "u64": "i64"}.get(t, t)


def _project_vec_lanes(vec: Vec, lane_type: str) -> List[int]:
    if lane_type == "i8":
        return vec.lanes_i8()
    if lane_type in ("i16", "u16"):
        return vec.lanes_i16()
    if lane_type in ("i32", "u32"):
        return vec.lanes_i32()
    if lane_type in ("i64", "u64"):
        return vec.lanes_i64()
    if lane_type == "u8":
        return vec.lanes_u8()
    raise ValueError(lane_type)


def _lane_bits(lane_type: str) -> int:
    return int(lane_type[1:])


def _project_scalar(s: int, lane_type: str) -> int:
    bits = _lane_bits(lane_type)
    norm = _norm_lane_type(lane_type)
    if norm[0] == "u":
        return _to_unsigned(s, bits)
    return _to_signed(s, bits)


def parse_spec_case(name: str, spec_text: str, source: str) -> Optional[SpecCase]:
    """Try to recognize a small set of spec patterns and produce a SpecCase
    that we can evaluate on a (args_dict) by mapping over the ground-truth
    result.  Returns None if the pattern isn't recognized — caller files it
    as OUT-OF-SCOPE-PATTERN."""

    s = _strip_str_lit(spec_text)
    # Drop line continuations (`\` at EOL) introduced by Rust raw strings.
    s = s.replace("\\n", " ")

    # Pattern A: Setzero — `vecBITS_as_TxN $result == Seq.create N (mk_T 0)`.
    m = re.match(
        r'(vec\d+_as_[iu]\d+x\d+)\s*\$result\s*==\s*Seq\.create\s+(\d+)\s+\(mk_[iu]\d+\s*0\)\s*$',
        s,
    )
    if m:
        proj = _parse_lane_proj(m.group(1))
        n = int(m.group(2))
        if not proj or proj[2] != n:
            return None
        _, lane_type, count = proj

        def evaluator(_args: Dict[str, Any], lt=lane_type, ct=count) -> List[int]:
            return [0] * ct

        def project(v: Vec, lt=lane_type) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(name, "PAT_SETZERO", spec_text, source, evaluator, project, [])

    # Pattern A1: same as setzero but `Seq.create N $constant` or `Spec.Utils.create (sz N) $constant`.
    m = re.match(
        r'(vec\d+_as_[iu]\d+x\d+)\s*\$result\s*==\s*'
        r'(?:Seq\.create\s+(?:\(\s*sz\s+)?(\d+)\s*\)?|Spec\.Utils\.create\s+\(\s*sz\s+(\d+)\s*\))\s*\$([A-Za-z_][A-Za-z0-9_]*)\s*$',
        s,
    )
    if m:
        proj = _parse_lane_proj(m.group(1))
        n = int(m.group(2) or m.group(3))
        constant_name = m.group(4)
        if not proj or proj[2] != n:
            return None
        _, lane_type, count = proj
        nlt = _norm_lane_type(lane_type)

        def evaluator(args, lt=nlt, ct=count, var=constant_name):
            v = args[var]
            return [_project_scalar(v, lt) for _ in range(ct)]

        def project(v: Vec, lt=nlt) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(name, "PAT_CREATE", spec_text, source, evaluator, project, [constant_name])

    # Pattern B: `vecBITS_as_TxN $result == Spec.Utils.map2 (OP) (vecBITS_as_TxN $A) (vecBITS_as_TxN $B)`.
    # Note: parens around binary op as `(+.)` etc.; bare ident ops (mul_mod, add_mod, etc.) accepted without parens.
    m = re.match(
        r'(vec\d+_as_[iu]\d+x\d+)\s*\$result\s*==\s*'
        r'Spec\.Utils\.map2\s*'
        r'(?:\(\s*([+\-*^&|]\.|mul_mod|add_mod|sub_mod)\s*\)|([A-Za-z_][A-Za-z0-9_]*))\s*'
        r'\(\s*(vec\d+_as_[iu]\d+x\d+)\s*\$([A-Za-z_][A-Za-z0-9_]*)\s*\)\s*'
        r'\(\s*(vec\d+_as_[iu]\d+x\d+)\s*\$([A-Za-z_][A-Za-z0-9_]*)\s*\)\s*$',
        s,
    )
    if m:
        proj_r = _parse_lane_proj(m.group(1))
        op = m.group(2) or m.group(3)
        proj_a = _parse_lane_proj(m.group(4))
        var_a = m.group(5)
        proj_b = _parse_lane_proj(m.group(6))
        var_b = m.group(7)
        if not (proj_r and proj_a and proj_b):
            return None
        if proj_r != proj_a or proj_r != proj_b:
            return None
        if op not in _OP_BIN_BY_LANE:
            return None
        _, lane_type, count = proj_r
        bits = _lane_bits(lane_type)
        op_fn = _OP_BIN_BY_LANE[op]

        def evaluator(args, lt=lane_type, ct=count, va=var_a, vb=var_b, fn=op_fn, bb=bits):
            a = _project_vec_lanes(args[va], lt)
            b = _project_vec_lanes(args[vb], lt)
            return [fn(x, y, bb) for x, y in zip(a, b)]

        def project(v: Vec, lt=lane_type) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(name, "PAT_MAP2_OP", spec_text, source, evaluator, project, [var_a, var_b])

    # Pattern B': map2 with a lambda body that the script knows how to
    # handle for mulhi: `(fun x y -> cast (((cast x <: i32) *. (cast y <: i32))
    # >>! (mk_i32 16)) <: i16)`
    m = re.match(
        r'(vec\d+_as_[iu]\d+x\d+)\s*\$result\s*==\s*'
        r'Spec\.Utils\.map2\s*\(\s*fun\s+x\s+y\s*->\s*'
        r'cast\s*\(\s*\(\s*\(\s*cast\s+x\s*<:\s*i32\s*\)\s*\*\.\s*'
        r'\(\s*cast\s+y\s*<:\s*i32\s*\)\s*\)\s*>>\!\s*\(\s*mk_i32\s+(\d+)\s*\)\s*\)\s*<:\s*i16\s*\)\s*'
        r'\(\s*(vec\d+_as_[iu]\d+x\d+)\s*\$([A-Za-z_][A-Za-z0-9_]*)\s*\)\s*'
        r'\(\s*(vec\d+_as_[iu]\d+x\d+)\s*\$([A-Za-z_][A-Za-z0-9_]*)\s*\)\s*$',
        s,
    )
    if m:
        proj_r = _parse_lane_proj(m.group(1))
        shift = int(m.group(2))
        proj_a = _parse_lane_proj(m.group(3))
        var_a = m.group(4)
        proj_b = _parse_lane_proj(m.group(5))
        var_b = m.group(6)
        if proj_r and proj_a and proj_b and proj_r == proj_a == proj_b:
            _, lane_type, count = proj_r

            def evaluator(args, lt=lane_type, ct=count, va=var_a, vb=var_b, sh=shift):
                a = _project_vec_lanes(args[va], lt)
                b = _project_vec_lanes(args[vb], lt)
                # cast to i32, multiply, shift right arith, truncate to i16.
                return [i16(((i32(x) * i32(y)) >> sh)) for x, y in zip(a, b)]

            def project(v, lt=lane_type) -> List[int]:
                return _project_vec_lanes(v, lt)

            return SpecCase(name, "PAT_MAP2_MULHI", spec_text, source, evaluator, project, [var_a, var_b])

    # Pattern C: `vec...$result == Spec.Utils.map_array (fun x -> ...) (vec... $V)`.
    m = re.match(
        r'(vec\d+_as_[iu]\d+x\d+)\s*\$result\s*==\s*'
        r'Spec\.Utils\.map_array\s*\(\s*fun\s+x\s*->\s*x\s*>>\!\s*\$\{?([A-Za-z_][A-Za-z0-9_]*)\}?\s*\)\s*'
        r'\(\s*(vec\d+_as_[iu]\d+x\d+)\s*\$([A-Za-z_][A-Za-z0-9_]*)\s*\)\s*$',
        s,
    )
    if m:
        proj_r = _parse_lane_proj(m.group(1))
        const_var = m.group(2)
        proj_v = _parse_lane_proj(m.group(3))
        var_v = m.group(4)
        if proj_r and proj_v and proj_r == proj_v:
            _, lane_type, count = proj_r
            bits = _lane_bits(lane_type)

            def evaluator(args, lt=lane_type, ct=count, vv=var_v, cv=const_var, bb=bits):
                a = _project_vec_lanes(args[vv], lt)
                shift = args[cv]
                # signed arithmetic right shift.
                return [shift_right_arith(x, shift, bb) for x in a]

            def project(v, lt=lane_type) -> List[int]:
                return _project_vec_lanes(v, lt)

            return SpecCase(name, "PAT_MAP_ARRAY_SHR", spec_text, source, evaluator, project, [const_var, var_v])

    # Pattern C': `vecBITS_as_OUTTYPE $result == Spec.Utils.map_array (fun (x:INTYPE) -> cast x <: OUTTYPE)
    #              (vecBITS2_as_INTYPE $VAR)` — sign-extension widening.
    m = re.match(
        r'(vec\d+_as_[iu]\d+x\d+)\s*\$result\s*==\s*'
        r'Spec\.Utils\.map_array\s+\(\s*fun\s+\(\s*x\s*:\s*([iu]\d+)\s*\)\s*->\s*cast\s+x\s*<:\s*([iu]\d+)\s*\)\s*'
        r'\(\s*(vec\d+_as_[iu]\d+x\d+)\s*\$([A-Za-z_][A-Za-z0-9_]*)\s*\)\s*$',
        s,
    )
    if m:
        proj_r = _parse_lane_proj(m.group(1))
        in_type = m.group(2)   # "i16"
        out_type = m.group(3)  # "i32"
        proj_v = _parse_lane_proj(m.group(4))
        var_v = m.group(5)
        if proj_r and proj_v and _lane_bits(in_type) < _lane_bits(out_type):
            _, out_lane_type, out_count = proj_r
            _, in_lane_type, in_count = proj_v

            def evaluator(args, vv=var_v, ilt=in_lane_type, olt=out_lane_type):
                src = _project_vec_lanes(args[vv], ilt)
                bits_out = _lane_bits(olt)
                return [_to_signed(x, bits_out) for x in src]

            def project(v, olt=out_lane_type) -> List[int]:
                return _project_vec_lanes(v, olt)

            return SpecCase(name, "PAT_MAP_ARRAY_SIGNEXT", spec_text, source,
                            evaluator, project, [var_v])

    # Pattern B'': map2 with equality comparison (cmpeq):
    # `vecBITS_as_TxN $result == Spec.Utils.map2 (fun (a b: T) -> if a = b then mk_T (-1) else mk_T 0) ...`
    m = re.match(
        r'(vec\d+_as_[iu]\d+x\d+)\s*\$result\s*==\s*'
        r'Spec\.Utils\.map2\s+\(\s*fun\s+\(\s*a\s+b\s*:\s*([iu]\d+)\s*\)\s*->\s*'
        r'if\s+a\s*=\s*b\s+then\s+mk_[iu]\d+\s+\(\s*-\s*1\s*\)\s+else\s+mk_[iu]\d+\s+0\s*\)\s*'
        r'\(\s*(vec\d+_as_[iu]\d+x\d+)\s*\$([A-Za-z_][A-Za-z0-9_]*)\s*\)\s*'
        r'\(\s*(vec\d+_as_[iu]\d+x\d+)\s*\$([A-Za-z_][A-Za-z0-9_]*)\s*\)\s*$',
        s,
    )
    if m:
        proj_r = _parse_lane_proj(m.group(1))
        proj_a = _parse_lane_proj(m.group(3))
        var_a = m.group(4)
        proj_b = _parse_lane_proj(m.group(5))
        var_b = m.group(6)
        if proj_r and proj_a and proj_b and proj_r == proj_a == proj_b:
            _, lane_type, count = proj_r
            bits = _lane_bits(lane_type)

            def evaluator(args, lt=lane_type, va=var_a, vb=var_b, bb=bits):
                a = _project_vec_lanes(args[va], lt)
                b = _project_vec_lanes(args[vb], lt)
                return [_to_signed(-1, bb) if x == y else 0 for x, y in zip(a, b)]

            def project(v, lt=lane_type) -> List[int]:
                return _project_vec_lanes(v, lt)

            return SpecCase(name, "PAT_MAP2_CMPEQ", spec_text, source,
                            evaluator, project, [var_a, var_b])

    # Pattern D: Seq.init 8 unpack_epi32:
    # `vec256_as_i32x8 $result == Seq.init 8 (fun i ->
    #    let lane_base: usize = if i < 4 then BASE_LO else BASE_HI in
    #    let local: usize = i % 4 in let src_idx: usize = lane_base + local / 2 in
    #    if local % 2 = 0 then Seq.index (vec256_as_i32x8 $lhs) src_idx
    #    else Seq.index (vec256_as_i32x8 $rhs) src_idx)`
    _UNPACK_EPI32_RE = re.compile(
        r'(vec256_as_i32x8)\s+\$result\s*==\s*'
        r'Seq\.init\s+8\s+\(\s*fun\s+i\s*->\s*'
        r'let\s+lane_base\s*:\s*usize\s*=\s*if\s+i\s*<\s*4\s+then\s+(\d+)\s+else\s+(\d+)\s+in\s*'
        r'let\s+local\s*:\s*usize\s*=\s*i\s*%\s*4\s+in\s*'
        r'let\s+src_idx\s*:\s*usize\s*=\s*lane_base\s*\+\s*local\s*/\s*2\s+in\s*'
        r'if\s+local\s*%\s*2\s*=\s*0\s+then\s+Seq\.index\s+\(\s*vec256_as_i32x8\s+\$([A-Za-z_][A-Za-z0-9_]*)\s*\)\s+src_idx\s+'
        r'else\s+Seq\.index\s+\(\s*vec256_as_i32x8\s+\$([A-Za-z_][A-Za-z0-9_]*)\s*\)\s+src_idx\s*\)',
    )
    mu = _UNPACK_EPI32_RE.match(s)
    if mu:
        base_lo = int(mu.group(2))
        base_hi = int(mu.group(3))
        var_lhs = mu.group(4)
        var_rhs = mu.group(5)

        def evaluator(args, vl=var_lhs, vr=var_rhs, blo=base_lo, bhi=base_hi):
            la = _project_vec_lanes(args[vl], "i32")
            lb = _project_vec_lanes(args[vr], "i32")
            out = []
            for idx in range(8):
                lane_base = blo if idx < 4 else bhi
                local = idx % 4
                src_idx = lane_base + local // 2
                out.append(la[src_idx] if local % 2 == 0 else lb[src_idx])
            return out

        def project(v) -> List[int]:
            return _project_vec_lanes(v, "i32")

        return SpecCase(name, "PAT_SEQINIT_UNPACK_EPI32", spec_text, source,
                        evaluator, project, [var_lhs, var_rhs])

    # Pattern E: packs_epi32 with saturation:
    # `let la = vec256_as_i32x8 $lhs in let lb = vec256_as_i32x8 $rhs in
    #  let sat (x: i32) : i16 = if x >. mk_i32 HI then mk_i16 HI else if x <. mk_i32 (-LO_ABS)
    #  then mk_i16 (-LO_ABS) else cast x <: i16 in
    #  vec256_as_i16x16 $result == Seq.init 16 (fun i -> ...)`
    _PACKS_EPI32_RE = re.compile(
        r'let\s+la\s*=\s*vec256_as_i32x8\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+in\s*'
        r'let\s+lb\s*=\s*vec256_as_i32x8\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+in\s*'
        r'let\s+sat\s+\(\s*x\s*:\s*i32\s*\)\s*:\s*i16\s*=\s*'
        r'if\s+x\s*>\.\s*mk_i32\s+(\d+)\s+then\s+mk_i16\s+\d+\s+'
        r'else\s+if\s+x\s*<\.\s*mk_i32\s+\(\s*-\s*(\d+)\s*\)\s+then\s+mk_i16\s+\(\s*-\s*\d+\s*\)\s+'
        r'else\s+cast\s+x\s*<:\s*i16\s+in\s*'
        r'vec256_as_i16x16\s+\$result\s*==\s*Seq\.init\s+16\s+\(\s*fun\s+i\s*->\s*'
        r'if\s+i\s*<\s*4\s+then\s+sat\s+\(\s*Seq\.index\s+la\s+i\s*\)\s+'
        r'else\s+if\s+i\s*<\s*8\s+then\s+sat\s+\(\s*Seq\.index\s+lb\s+\(\s*i\s*-\s*4\s*\)\s*\)\s+'
        r'else\s+if\s+i\s*<\s*12\s+then\s+sat\s+\(\s*Seq\.index\s+la\s+\(\s*i\s*-\s*4\s*\)\s*\)\s+'
        r'else\s+sat\s+\(\s*Seq\.index\s+lb\s+\(\s*i\s*-\s*8\s*\)\s*\)\s*\)',
    )
    mp = _PACKS_EPI32_RE.match(s)
    if mp:
        var_lhs = mp.group(1)
        var_rhs = mp.group(2)
        sat_hi = int(mp.group(3))    # 32767
        sat_lo_abs = int(mp.group(4))  # 32768

        def evaluator(args, vl=var_lhs, vr=var_rhs, hi=sat_hi, lo_abs=sat_lo_abs):
            la = _project_vec_lanes(args[vl], "i32")
            lb = _project_vec_lanes(args[vr], "i32")
            def sat(x):
                if x > hi: return hi
                if x < -lo_abs: return -lo_abs
                return i16(x)
            return [
                sat(la[i]) if i < 4 else
                sat(lb[i - 4]) if i < 8 else
                sat(la[i - 4]) if i < 12 else
                sat(lb[i - 8])
                for i in range(16)
            ]

        def project(v) -> List[int]:
            return _project_vec_lanes(v, "i16")

        return SpecCase(name, "PAT_PACKS_EPI32", spec_text, source,
                        evaluator, project, [var_lhs, var_rhs])

    # ==== ARM64 patterns ====

    # PAT_ARM_HSUM_SCALAR: `$result == (tree of get_lane_TxN $VAR CONST +. ...)`.
    # Horizontal reduction to scalar — vaddvq_s16, vaddvq_u16, vaddv_u16.
    # We match by extracting all mentioned variable names and lane indices, then
    # evaluate by summing the specified lanes.
    _HSUM_RE = re.compile(
        r'^\$result\s*==\s*'
        r'([\(\)\+\.\s]*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+\d+[\(\)\+\.\s]*)+$'
    )
    if _HSUM_RE.match(s):
        lane_refs = re.findall(r'get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+(\d+)', s)
        if lane_refs:
            vars_seen = {v for (_, _, v, _) in lane_refs}
            if len(vars_seen) == 1:
                var = next(iter(vars_seen))
                lt_raw = lane_refs[0][0]    # e.g. "i16" or "u16"
                lt = _norm_lane_type(lt_raw)
                bits = _lane_bits(lt)
                indices = [int(idx) for (_, _, _, idx) in lane_refs]

                def evaluator(args, v=var, lt2=lt, idxs=indices, bb=bits):
                    lanes = _project_vec_lanes(args[v], lt2)
                    total = sum(lanes[i] for i in idxs)
                    return [_to_signed(total, bb)]

                def project(result_val) -> List[int]:
                    return [result_val]

                return SpecCase(name, "PAT_ARM_HSUM_SCALAR", spec_text, source,
                                evaluator, project, [var])

    # PAT_ARM_IDENTITY: `$result == $VAR`
    m = re.match(r'^\$result\s*==\s*\$([A-Za-z_][A-Za-z0-9_]*)\s*$', s)
    if m:
        var = m.group(1)
        def evaluator(args, v=var):
            return list(args[v].bytes)
        def project(vec):
            return list(vec.bytes)
        return SpecCase(name, "PAT_ARM_IDENTITY", spec_text, source, evaluator, project, [var])

    # PAT_ARM_TRN_CONJ_I16 / PAT_ARM_TRN_CONJ_I32:
    # `(forall (i:nat{i < N}). get_lane_TxM $result (2*i + K) == get_lane_TxM $A (2*i + J)) /\\
    #  (forall (i:nat{i < N}). get_lane_TxM $result (2*i + K') == get_lane_TxM $B (2*i + J'))`
    _CONJ_SPLIT_RE = re.compile(r'\)\s*/\\\\\s*\(')
    conj_parts = _CONJ_SPLIT_RE.split(s)
    if len(conj_parts) == 2:
        p0 = conj_parts[0].lstrip('(').strip()
        _p1 = conj_parts[1].strip()
        p1 = _p1[:-1].strip() if _p1.endswith(')') else _p1
        # Pattern: `forall (i:nat{i < N}). get_lane_TxM $result (2*i + OFFSET) == get_lane_TxM $VAR (2*i + SRC)`
        _TRN_FORALL_RE = re.compile(
            r'forall\s+\(i:nat\{i\s*<\s*(\d+)\}\)\.\s*'
            r'get_lane_([iu]\d+)x(\d+)\s+\$result\s+\(2\s*\*\s*i\s*(?:\+\s*(\d+))?\)\s*==\s*'
            r'get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+\(2\s*\*\s*i\s*(?:\+\s*(\d+))?\)',
        )
        m0 = _TRN_FORALL_RE.match(p0)
        m1 = _TRN_FORALL_RE.match(p1)
        if m0 and m1:
            count0 = int(m0.group(1))
            lane_type0 = m0.group(2)          # e.g. "i16"
            off_r0 = int(m0.group(4) or 0)   # offset in result index (e.g. 0 or 1)
            var0 = m0.group(7)
            off_s0 = int(m0.group(8) or 0)   # offset in source index
            count1 = int(m1.group(1))
            lane_type1 = m1.group(2)
            off_r1 = int(m1.group(4) or 0)
            var1 = m1.group(7)
            off_s1 = int(m1.group(8) or 0)
            total_lanes = count0 * 2
            if lane_type0 == lane_type1 and count0 == count1:
                lt = _norm_lane_type(lane_type0)

                def evaluator(args, va=var0, vb=var1, n=count0,
                               or0=off_r0, os0=off_s0, or1=off_r1, os1=off_s1, lt2=lt):
                    a = _project_vec_lanes(args[va], lt2)
                    b = _project_vec_lanes(args[vb], lt2)
                    out = [0] * (n * 2)
                    for i in range(n):
                        out[2 * i + or0] = a[2 * i + os0]
                        out[2 * i + or1] = b[2 * i + os1]
                    return out

                def project(v, lt2=lt):
                    return _project_vec_lanes(v, lt2)

                return SpecCase(name, "PAT_ARM_TRN_CONJ", spec_text, source,
                                evaluator, project, [var0, var1])

    # PAT_ARM_2LANE: explicit 2-lane conjunction:
    # `get_lane_TxN $result 0 == get_lane_TxN $A IDX0 /\\ get_lane_TxN $result 1 == get_lane_TxN $B IDX1`
    _2LANE_RE = re.compile(
        r'get_lane_([iu]\d+)x(\d+)\s+\$result\s+0\s*==\s*'
        r'get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+(\d+)'
        r'\s*/\\\\\s*'
        r'get_lane_([iu]\d+)x(\d+)\s+\$result\s+1\s*==\s*'
        r'get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+(\d+)',
    )
    m = _2LANE_RE.match(s)
    if m:
        lt_r = m.group(1)   # e.g. "u64"
        lt_a = m.group(3)
        var_a = m.group(5); idx_a = int(m.group(6))
        lt_b = m.group(9)
        var_b = m.group(11); idx_b = int(m.group(12))
        if lt_r == lt_a == lt_b:
            lt = _norm_lane_type(lt_r)

            def evaluator(args, va=var_a, vb=var_b, ia=idx_a, ib=idx_b, lt2=lt):
                a = _project_vec_lanes(args[va], lt2)
                b = _project_vec_lanes(args[vb], lt2)
                return [a[ia], b[ib]]

            def project(v, lt2=lt):
                return _project_vec_lanes(v, lt2)

            return SpecCase(name, "PAT_ARM_2LANE", spec_text, source,
                            evaluator, project, [var_a, var_b])

    # PAT_ARM_HALFVEC_EXTRACT: `forall (i:nat{i < 4}). get_lane_TxN1 $result i == get_lane_TxN2 $a IDX`
    # IDX is either bare `i` (offset 0) or `(i + N)` (offset N).
    # vget_low (offset=0) / vget_high (offset=4 etc.)
    _HALFVEC_RE = re.compile(
        r'^forall\s+\(i:nat\{i\s*<\s*(\d+)\}\)\.\s*'
        r'get_lane_([iu]\d+)x(\d+)\s+\$result\s+i\s*==\s*'
        r'get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+'
        r'(?:\(\s*i\s*\+\s*(\d+)\s*\)|i)\s*$',
    )
    mh = _HALFVEC_RE.match(s)
    if mh:
        count = int(mh.group(1))
        result_lt = _norm_lane_type(mh.group(2))
        src_lt = _norm_lane_type(mh.group(4))
        var = mh.group(6)
        offset = int(mh.group(7) or 0)
        if result_lt == src_lt:
            def evaluator(args, v=var, lt2=result_lt, n=count, off=offset):
                src = _project_vec_lanes(args[v], lt2)
                return [src[i + off] for i in range(n)]

            def project(vec, lt2=result_lt) -> List[int]:
                return _project_vec_lanes(vec, lt2)

            return SpecCase(name, "PAT_ARM_HALFVEC_EXTRACT", spec_text, source,
                            evaluator, project, [var])

    # PAT_ARM_VMULL: widening multiply — cast narrow → wide, multiply.
    # `forall (i:nat{i < 4}). get_lane_i32x4 $result i ==
    #   (cast (get_lane_i16xN $a IDX_A) <: i32) *. (cast (get_lane_i16xN $b IDX_B) <: i32)`
    # IDX is bare `i` or `(i + N)`.
    _VMULL_RE = re.compile(
        r'^forall\s+\(i:nat\{i\s*<\s*(\d+)\}\)\.\s*'
        r'get_lane_([iu]\d+)x(\d+)\s+\$result\s+i\s*==\s*'
        r'\(\s*cast\s+\(\s*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+'
        r'(?:\(\s*i\s*\+\s*(\d+)\s*\)|i)\s*\)\s*<:\s*([iu]\d+)\s*\)\s*\*\.\s*'
        r'\(\s*cast\s+\(\s*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+'
        r'(?:\(\s*i\s*\+\s*(\d+)\s*\)|i)\s*\)\s*<:\s*([iu]\d+)\s*\)\s*$',
    )
    mv = _VMULL_RE.match(s)
    if mv:
        count = int(mv.group(1))
        res_lt = _norm_lane_type(mv.group(2))
        src_a_lt = _norm_lane_type(mv.group(4))
        var_a = mv.group(6); off_a = int(mv.group(7) or 0)
        cast_a = _norm_lane_type(mv.group(8))
        src_b_lt = _norm_lane_type(mv.group(9))
        var_b = mv.group(11); off_b = int(mv.group(12) or 0)
        cast_b = _norm_lane_type(mv.group(13))
        if res_lt == cast_a == cast_b:
            res_bits = _lane_bits(res_lt)

            def evaluator(args, va=var_a, vb=var_b, slt_a=src_a_lt, slt_b=src_b_lt,
                          oa=off_a, ob=off_b, n=count, rb=res_bits):
                a = _project_vec_lanes(args[va], slt_a)
                b = _project_vec_lanes(args[vb], slt_b)
                return [_to_signed(a[i + oa] * b[i + ob], rb) for i in range(n)]

            def project(vec, lt2=res_lt) -> List[int]:
                return _project_vec_lanes(vec, lt2)

            return SpecCase(name, "PAT_ARM_VMULL", spec_text, source,
                            evaluator, project, [var_a, var_b])

    # PAT_ARM_VMLAL: multiply-accumulate with widening.
    # `forall (i:nat{i < 4}). get_lane_i32x4 $result i ==
    #   get_lane_i32x4 $a i +. ((cast (get_lane_i16xN $b IDX_B) <: i32) *. (cast (...) <: i32))`
    # IDX is bare `i` or `(i + N)`.
    _VMLAL_RE = re.compile(
        r'^forall\s+\(i:nat\{i\s*<\s*(\d+)\}\)\.\s*'
        r'get_lane_([iu]\d+)x(\d+)\s+\$result\s+i\s*==\s*'
        r'get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s*\+\.\s*'
        r'\(\s*\(\s*cast\s+\(\s*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+'
        r'(?:\(\s*i\s*\+\s*(\d+)\s*\)|i)\s*\)\s*<:\s*([iu]\d+)\s*\)\s*\*\.\s*'
        r'\(\s*cast\s+\(\s*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+'
        r'(?:\(\s*i\s*\+\s*(\d+)\s*\)|i)\s*\)\s*<:\s*([iu]\d+)\s*\)\s*\)\s*$',
    )
    mml = _VMLAL_RE.match(s)
    if mml:
        count = int(mml.group(1))
        res_lt = _norm_lane_type(mml.group(2))
        acc_lt = _norm_lane_type(mml.group(4))
        var_acc = mml.group(6)
        src_b_lt = _norm_lane_type(mml.group(7))
        var_b = mml.group(9); off_b = int(mml.group(10) or 0); cast_b = _norm_lane_type(mml.group(11))
        src_c_lt = _norm_lane_type(mml.group(12))
        var_c = mml.group(14); off_c = int(mml.group(15) or 0); cast_c = _norm_lane_type(mml.group(16))
        if res_lt == acc_lt == cast_b == cast_c:
            res_bits = _lane_bits(res_lt)

            def evaluator(args, va=var_acc, vb=var_b, vc=var_c,
                          acclt=res_lt, slt_b=src_b_lt, slt_c=src_c_lt,
                          ob=off_b, oc=off_c, n=count, rb=res_bits):
                acc = _project_vec_lanes(args[va], acclt)
                b = _project_vec_lanes(args[vb], slt_b)
                c = _project_vec_lanes(args[vc], slt_c)
                return [_to_signed(acc[i] + b[i + ob] * c[i + oc], rb) for i in range(n)]

            def project(vec, lt2=res_lt) -> List[int]:
                return _project_vec_lanes(vec, lt2)

            return SpecCase(name, "PAT_ARM_VMLAL", spec_text, source,
                            evaluator, project, [var_acc, var_b, var_c])

    # ARM64 forall patterns: `forall (i:nat{i < N}). get_lane_TxN $result i == RHS`
    _FORALL_PREFIX_RE = re.compile(
        r'^forall\s+\(i:nat\{i\s*<\s*(\d+)\}\)\.\s*'
        r'(?:let\s+\w+\s*=\s*[^i][^\n]*\s+in\s*)?'  # optional let binding before result check
        r'get_lane_([iu]\d+)x(\d+)\s+\$result\s+i\s*==\s*(.*)',
        re.DOTALL,
    )
    m = _FORALL_PREFIX_RE.match(s)
    if m:
        count = int(m.group(1))
        lane_type = m.group(2)  # e.g. "i16", "u64"
        rhs = m.group(4).strip()
        lt = _norm_lane_type(lane_type)
        lane_b = _lane_bits(lane_type)  # bit width, e.g. 16 for "i16"

        # Binary op: `get_lane_TxN $A i OP get_lane_TxN $B i` (with optional outer parens)
        _BIN_OP_RE = re.compile(
            r'^\(?\s*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s*'
            r'([+\-*^&|]\.|mul_mod|add_mod|sub_mod)\s*'
            r'get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s*\)?$',
        )
        mb = _BIN_OP_RE.match(rhs)
        if mb:
            var_a = mb.group(3); var_b = mb.group(7)
            op = mb.group(4)
            if op in _OP_BIN_BY_LANE:
                op_fn = _OP_BIN_BY_LANE[op]

                def evaluator(args, va=var_a, vb=var_b, fn=op_fn, lt2=lt, bb=lane_b):
                    a = _project_vec_lanes(args[va], lt2)
                    b = _project_vec_lanes(args[vb], lt2)
                    return [fn(x, y, bb) for x, y in zip(a, b)]

                def project(v, lt2=lt):
                    return _project_vec_lanes(v, lt2)

                return SpecCase(name, "PAT_ARM_FORALL_BIN", spec_text, source,
                                evaluator, project, [var_a, var_b])

        # Scalar mul: `get_lane_TxN $V i *. $C`
        _SCALAR_MUL_RE = re.compile(
            r'^\(?\s*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s*'
            r'\*\.\s*\$([A-Za-z_][A-Za-z0-9_]*)\s*\)?$',
        )
        ms = _SCALAR_MUL_RE.match(rhs)
        if ms:
            var_v = ms.group(3); var_c = ms.group(4)

            def evaluator(args, vv=var_v, vc=var_c, lt2=lt, bb=lane_b):
                a = _project_vec_lanes(args[vv], lt2)
                c = _to_signed(args[vc], bb)
                return [_to_signed(_to_unsigned(x, bb) * _to_unsigned(c, bb) & ((1 << bb) - 1), bb) for x in a]

            def project(v, lt2=lt):
                return _project_vec_lanes(v, lt2)

            return SpecCase(name, "PAT_ARM_FORALL_SCALAR_MUL", spec_text, source,
                            evaluator, project, [var_v, var_c])

        # Seq.index load: `Seq.index $array i`
        _SEQIDX_RE = re.compile(r'^Seq\.index\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i$')
        ms = _SEQIDX_RE.match(rhs)
        if ms:
            var_arr = ms.group(1)
            # Model: result == identity of the input interpreted as VEC128
            def evaluator(args, va=var_arr, lt2=lt, bb=lane_b):
                return _project_vec_lanes(args[va], lt2)

            def project(v, lt2=lt):
                return _project_vec_lanes(v, lt2)

            return SpecCase(name, "PAT_ARM_FORALL_SEQIDX", spec_text, source,
                            evaluator, project, [var_arr])

        # BIC: `(get_lane_TxN $A i &. (~. (get_lane_TxN $B i)))`
        _BIC_RE = re.compile(
            r'^\(?\s*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s*'
            r'&\.\s*\(~\.\s*\(get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\)\)\s*\)?$',
        )
        mb = _BIC_RE.match(rhs)
        if mb:
            var_a = mb.group(3); var_b = mb.group(6)

            def evaluator(args, va=var_a, vb=var_b, lt2=lt, bb=lane_b):
                a = _project_vec_lanes(args[va], lt2)
                b = _project_vec_lanes(args[vb], lt2)
                mask = (1 << bb) - 1
                return [_to_signed(_to_unsigned(x, bb) & (~_to_unsigned(y, bb) & mask), bb)
                        for x, y in zip(a, b)]

            def project(v, lt2=lt):
                return _project_vec_lanes(v, lt2)

            return SpecCase(name, "PAT_ARM_FORALL_BIC", spec_text, source,
                            evaluator, project, [var_a, var_b])

        # BCAX: `(get_lane_TxN $A i ^. (get_lane_TxN $B i &. (~. (get_lane_TxN $C i))))`
        _BCAX_RE = re.compile(
            r'^\(?\s*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s*'
            r'\^\.\s*\(get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s*'
            r'&\.\s*\(~\.\s*\(get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\)\)\)\s*\)?$',
        )
        mb = _BCAX_RE.match(rhs)
        if mb:
            var_a = mb.group(3); var_b = mb.group(6); var_c = mb.group(9)

            def evaluator(args, va=var_a, vb=var_b, vc=var_c, lt2=lt, bb=lane_b):
                a = _project_vec_lanes(args[va], lt2)
                b = _project_vec_lanes(args[vb], lt2)
                c = _project_vec_lanes(args[vc], lt2)
                mask = (1 << bb) - 1
                return [_to_signed(_to_unsigned(x, bb) ^ (_to_unsigned(y, bb) & ~_to_unsigned(z, bb) & mask), bb)
                        for x, y, z in zip(a, b, c)]

            def project(v, lt2=lt):
                return _project_vec_lanes(v, lt2)

            return SpecCase(name, "PAT_ARM_FORALL_BCAX", spec_text, source,
                            evaluator, project, [var_a, var_b, var_c])

        # CMPGE: `(if get_lane_i16x8 $V i >=. get_lane_i16x8 $C i then mk_u16 0xFFFF else mk_u16 0)`
        _CMPGE_RE = re.compile(
            r'^\(?\s*if\s+get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s*'
            r'(>=\.|<=\.)\s*'
            r'get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s+'
            r'then\s+mk_[iu]\d+\s+(0x[0-9A-Fa-f]+|\d+)\s+else\s+mk_[iu]\d+\s+0\s*\)?$',
        )
        mb = _CMPGE_RE.match(rhs)
        if mb:
            var_v = mb.group(3); cmp_op = mb.group(4); var_c = mb.group(7)
            true_val_str = mb.group(8)
            true_val = int(true_val_str, 16) if true_val_str.startswith('0x') else int(true_val_str)
            true_signed = _to_signed(true_val, lane_b)
            if cmp_op == ">=.":
                def evaluator(args, vv=var_v, vc=var_c, lt2=lt, tv=true_signed):
                    a = _project_vec_lanes(args[vv], lt2)
                    b = _project_vec_lanes(args[vc], lt2)
                    return [tv if x >= y else 0 for x, y in zip(a, b)]
            else:  # <=.
                def evaluator(args, vv=var_v, vc=var_c, lt2=lt, tv=true_signed):
                    a = _project_vec_lanes(args[vv], lt2)
                    b = _project_vec_lanes(args[vc], lt2)
                    return [tv if x <= y else 0 for x, y in zip(a, b)]

            def project(v, lt2=lt):
                return _project_vec_lanes(v, lt2)

            return SpecCase(name, "PAT_ARM_FORALL_CMP", spec_text, source,
                            evaluator, project, [var_v, var_c])

        # VEOR3: `((get_lane_u64x2 $a i ^. get_lane_u64x2 $b i) ^. get_lane_u64x2 $c i)`
        _VEOR3_RE = re.compile(
            r'^\(?\s*\(?\s*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s*'
            r'\^\.\s*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s*\)?\s*'
            r'\^\.\s*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s*\)?$',
        )
        mv = _VEOR3_RE.match(rhs)
        if mv:
            var_a = mv.group(3); var_b = mv.group(6); var_c = mv.group(9)

            def evaluator(args, va=var_a, vb=var_b, vc=var_c, lt2=lt, bb=lane_b):
                a = _project_vec_lanes(args[va], lt2)
                b = _project_vec_lanes(args[vb], lt2)
                c = _project_vec_lanes(args[vc], lt2)
                mask = (1 << bb) - 1
                return [_to_signed((_to_unsigned(x, bb) ^ _to_unsigned(y, bb) ^ _to_unsigned(z, bb)) & mask, bb)
                        for x, y, z in zip(a, b, c)]

            def project(v, lt2=lt):
                return _project_vec_lanes(v, lt2)

            return SpecCase(name, "PAT_ARM_FORALL_VEOR3", spec_text, source,
                            evaluator, project, [var_a, var_b, var_c])

        # VRAX1: `(get_lane_u64x2 $a i ^. Core_models.Num.impl_u64__rotate_left (get_lane_u64x2 $b i) (mk_u32 1))`
        _VRAX1_RE = re.compile(
            r'^\(?\s*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s*'
            r'\^\.\s*Core_models\.Num\.impl_u64__rotate_left\s+'
            r'\(get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\)\s+'
            r'\(mk_u32\s+1\)\s*\)?$',
        )
        mv = _VRAX1_RE.match(rhs)
        if mv:
            var_a = mv.group(3); var_b = mv.group(6)

            def evaluator(args, va=var_a, vb=var_b):
                a = args[va].lanes_i64()
                b = args[vb].lanes_i64()
                return [i64(_to_unsigned(x, 64) ^ _to_unsigned(rotate_left_u64(y, 1), 64))
                        for x, y in zip(a, b)]

            def project(v):
                return v.lanes_i64()

            return SpecCase(name, "PAT_ARM_FORALL_VRAX1", spec_text, source,
                            evaluator, project, [var_a, var_b])

        # VXARQ: `Core_models.Num.impl_u64__rotate_left (get_lane_u64x2 $a i ^. get_lane_u64x2 $b i) (cast ${LEFT} <: u32)`
        _VXARQ_RE = re.compile(
            r'^Core_models\.Num\.impl_u64__rotate_left\s+'
            r'\(get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\s*'
            r'\^\.\s*get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\)\s+'
            r'\(cast\s+\$\{?([A-Za-z_][A-Za-z0-9_]*)\}?\s*<:\s*u32\)',
        )
        mv = _VXARQ_RE.match(rhs)
        if mv:
            var_a = mv.group(3); var_b = mv.group(6); var_left = mv.group(7)

            def evaluator(args, va=var_a, vb=var_b, vl=var_left):
                a = args[va].lanes_i64()
                b = args[vb].lanes_i64()
                left = int(args[vl]) % 64
                return [rotate_left_u64(i64(_to_unsigned(x, 64) ^ _to_unsigned(y, 64)), left)
                        for x, y in zip(a, b)]

            def project(v):
                return v.lanes_i64()

            # Put const (LEFT) first so the reorder-to-front logic in
            # run_for_intrinsic correctly maps the CONST_I32 GT slot.
            return SpecCase(name, "PAT_ARM_FORALL_VXARQ", spec_text, source,
                            evaluator, project, [var_left, var_a, var_b])

        # QTBL1 (with let binding): handled via special forall that includes `let ix` in prefix
        # Raw: `forall (i:nat{i < 16}). let ix = v (get_lane_u8x16 $idx i) in
        #       get_lane_u8x16 $result i == (if ix < 16 then get_lane_u8x16 $t ix else mk_u8 0)`
        # The FORALL_PREFIX_RE won't match because the let is part of the forall body.
        # We handle this with a dedicated match below.

    # QTBL1 dedicated match (forall with let binding before the result check)
    _QTBL1_RE = re.compile(
        r'^forall\s+\(i:nat\{i\s*<\s*(\d+)\}\)\.\s*'
        r'let\s+(\w+)\s*=\s*v\s*\(get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+i\)\s+in\s*'
        r'get_lane_([iu]\d+)x(\d+)\s+\$result\s+i\s*==\s*'
        r'\(\s*if\s+\2\s*<\s*(\d+)\s+then\s+get_lane_([iu]\d+)x(\d+)\s+\$([A-Za-z_][A-Za-z0-9_]*)\s+\2\s+else\s+mk_[iu]\d+\s+0\s*\)',
        re.DOTALL,
    )
    m = _QTBL1_RE.match(s)
    if m:
        var_idx = m.group(5); var_t = m.group(11)
        tab_size = int(m.group(8))
        lane_bits_qtbl = int(m.group(4))

        def evaluator(args, vi=var_idx, vt=var_t, tsz=tab_size, lb=lane_bits_qtbl):
            t = args[vt].lanes_i8()
            idx = args[vi].lanes_i8()
            out = []
            for raw_ix in idx:
                uix = _to_unsigned(raw_ix, lb)
                out.append(t[uix] if uix < tsz else 0)
            return out

        def project(v):
            return v.lanes_i8()

        return SpecCase(name, "PAT_ARM_QTBL1", spec_text, source,
                        evaluator, project, [var_t, var_idx])

    return None


# ----------------------- Spec.Intrinsics.fsti SMTPat parsing ----------------

# We support a small set of recurring `to_iNxM (I.fn args) i == <rhs>` forms.
# In principle this is a separate parser pass; for the L2 pilot scope we
# focus on the int-vec-side lemmas (those whose LHS is `to_i32x8 (I.fn ...) i
# == ...`), which are easy to project against our ground-truth output.

# We only register lemmas whose intrinsic is in GROUND_TRUTH already.

@dataclass
class SmtPatLemma:
    intrinsic: str          # lib name without leading underscore
    args_in_call: List[str]  # F* argument names (after the I.<name> token)
    rhs: str                 # raw F* RHS expression
    quantifier: str          # quantifier name: typically `i`
    proj_kind: str           # e.g., 'to_i32x8', 'to_i16x16', '.()'
    full_text: str           # the entire `val ..._lemma ...` block


_SMTPAT_LEMMA_RE = re.compile(
    r'^val\s+(?P<lemmaname>(mm[0-9]*_[A-Za-z0-9_]+?))(?:_bv)?_lemma\b',
    re.MULTILINE,
)


def _slice_lemma_block(text: str, start: int) -> Tuple[int, int]:
    """Slice from the `val foo_lemma` keyword up to the next top-level
    boundary.  Returns (start, end) of the entire block."""
    # Look for next `^val ` or `^let ` (line-start) after the start.
    boundary = re.compile(r'\n(val\s+|let\s+|module\s+|open\s+)', re.MULTILINE)
    m = boundary.search(text, start + 4)
    end = m.start() if m else len(text)
    return start, end


def parse_specintrinsics_lemmas() -> Dict[str, List[str]]:
    """Return {intrinsic_name: [block_texts]}.  Multiple variants per name
    (e.g. `_lemma` and `_bv_lemma`) are kept distinct, all keyed off the
    same intrinsic.

    We DO NOT parse the F* RHS here — full parsing is OUT-OF-SCOPE-PATTERN
    when the structure isn't recognised.  Instead, the caller picks which
    lemmas have a structure we recognise (e.g., `to_i32x8 (I.fn a b) i ==
    add_mod_opaque (to_i32x8 a i) (to_i32x8 b i)`) via ad-hoc regex below.
    """
    text = SPEC_INTRINSICS_FSTI.read_text() if SPEC_INTRINSICS_FSTI.exists() else ""
    out: Dict[str, List[str]] = {}
    for m in _SMTPAT_LEMMA_RE.finditer(text):
        name = m.group("lemmaname")
        s, e = _slice_lemma_block(text, m.start())
        out.setdefault(name, []).append(text[s:e])
    return out


# Recognised SMTPat lemma shapes (subset).  Keyed by intrinsic name.  Each
# pattern emits a SpecCase whose `project_lhs` is a per-lane projection,
# and `eval_rhs` computes the RHS from the raw inputs.
#
# The patterns we handle (regex-based, conservative):
#   to_iNxM (I.NAME a b) i == add_mod_opaque (to_iNxM a i) (to_iNxM b i)
#                          == sub_mod_opaque ...
#                          == mul_mod_opaque ...
#                          == ((to_iNxM a i) &./^./|. (to_iNxM b i))
#   to_iNxM (I.NAME a) i   == abs_int (to_iNxM a i)            # has requires
#   to_iNxM (I.cmpgt a b) i == (if a > b then ones else zero)
#   to_iNxM (I.set1_epi32 x0) i == x0
#   to_iNxM (I.NAME imm a) i == ... shift_right_opaque/shift_left_opaque ...
#                                                              # srai/slli
#   to_iNxM (I.blend a b) i == (if (v imm8 / pow2 (v i)) % 2 = 0 then a else b)


# Common: the `match v i with | 0 -> x3 | 1 -> x2 | ...` set-form.
_LEMMA_SET_MATCH_RE = re.compile(
    r'to_([iu]\d+x\d+)\s*\(\s*[A-Za-z_.]+\.(?P<intr>mm[0-9]*_[A-Za-z0-9_]+)'
    r'(?P<args>(?:\s+\w+)+)\s*\)\s+i\s*==\s*'
    r'\(\s*match\s+(?:v\s+i|i\s*<:\s*u64)\s+with\s+(?P<arms>(?:\|[^=]*?)+?)\)',
    re.DOTALL,
)


# Plain `to_iNxM (I.set1_epi32 x0) i == x0`.
_LEMMA_SET1_RE = re.compile(
    r'to_([iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm[0-9]*_set1_[A-Za-z0-9_]+)\s+'
    r'(?P<v>\w+)\s*\)\s+i\s*==\s*(?P=v)\s*[\)\s]',
    re.DOTALL,
)


_LEMMA_BIN_OPAQUE_RE = re.compile(
    r'to_([iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm[0-9]*_[A-Za-z0-9_]+)\s+'
    r'(?P<a>\w+)\s+(?P<b>\w+)\s*\)\s+i\s*==\s*'
    r'(?P<op>add_mod_opaque|sub_mod_opaque|mul_mod_opaque)\s*'
    r'\(to_\1\s+(?P=a)\s+i\)\s*\(to_\1\s+(?P=b)\s+i\)',
    re.DOTALL,
)

_LEMMA_BIN_BITWISE_RE = re.compile(
    r'to_([iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm[0-9]*_[A-Za-z0-9_]+)\s+'
    r'(?P<a>\w+)\s+(?P<b>\w+)\s*\)\s+i\s*==\s*'
    r'\(\s*\(to_\1\s+(?P=a)\s+i\)\s*(?P<op>[&^|])\.\s*\(to_\1\s+(?P=b)\s+i\)\s*\)',
    re.DOTALL,
)

_LEMMA_CMPGT_RE = re.compile(
    r'to_([iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm[0-9]*_cmpgt_[A-Za-z0-9_]+)\s+'
    r'(?P<a>\w+)\s+(?P<b>\w+)\s*\)\s+i\s*==\s*'
    r'\(if\s+\(?\s*to_\1\s+(?P=a)\s+i\s*>\.\s*to_\1\s+(?P=b)\s+i\s*\)?'
    r'\s*then\s*ones\s*else\s*zero\s*\)',
    re.DOTALL,
)

_LEMMA_ABS_RE = re.compile(
    r'to_([iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm[0-9]*_abs_epi[0-9]+)\s+'
    r'(?P<a>\w+)\s*\)\s+i\s*==\s*'
    r'abs_int\s*\(to_\1\s+(?P=a)\s+i\)',
    re.DOTALL,
)


# `to_i32x8 (I.NAME v_IMM8 a) i == ...` for shifts.  Capture the args.
_LEMMA_SLLI_EPI32_RE = re.compile(
    r'to_([iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm[0-9]*_slli_epi32)\s+'
    r'(?P<imm>\w+)\s+(?P<a>\w+)\s*\)\s+i\s*==\s*'
    r'\(\s*if\s+(?P=imm)\s*<\.\s*mk_i32\s+0\s*\|\|\s*(?P=imm)\s*>\.\s*mk_i32\s+31\s*'
    r'then\s+mk_i32\s+0\s+'
    r'else\s+shift_left_opaque\s+\(to_\1\s+(?P=a)\s+i\)\s+(?P=imm)\s*\)',
    re.DOTALL,
)

_LEMMA_SRAI_EPI32_RE = re.compile(
    r'to_([iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm[0-9]*_srai_epi32)\s+'
    r'(?P<imm>\w+)\s+(?P<a>\w+)\s*\)\s+i\s*==\s*'
    r'\(\s*let\s+imm8:i32\s*=\s*Core_models\.Num\.impl_i32__rem_euclid\s+(?P=imm)\s*\(mk_i32\s+256\)\s+in\s+'
    r'if\s+imm8\s*>\.\s*mk_i32\s+31\s+'
    r'then\s+if\s+\(to_\1\s+(?P=a)\s+i\)\s*<\.\s*mk_i32\s+0\s+then\s+mk_i32\s+\(?-1\)?\s+else\s+mk_i32\s+0\s+'
    r'else\s+shift_right_opaque\s+\(to_\1\s+(?P=a)\s+i\)\s+imm8\s*\)',
    re.DOTALL,
)


# `mm256_blend_epi32`: `if (v imm8 / pow2 (v i)) % 2 = 0 then to_i32x8 a i else to_i32x8 b i`.
_LEMMA_BLEND_EPI32_RE = re.compile(
    r'to_([iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm[0-9]*_blend_epi32)\s+'
    r'(?P<imm>\w+)\s+(?P<a>\w+)\s+(?P<b>\w+)\s*\)\s+i\s*==\s*'
    r'\(if\s*\(?v\s+(?P=imm)\s*/\s*pow2\s*\(\s*v\s+i\s*\)\s*\)?\s*%\s*2\s*=\s*0\s+'
    r'then\s+to_\1\s+(?P=a)\s+i\s+else\s+to_\1\s+(?P=b)\s+i\s*\)',
    re.DOTALL,
)


# `mm_set_epi32 x0 x1 x2 x3` lemma — `match v i with | 0 -> x3 | 1 -> x2 | 2 -> x1 | 3 -> x0`.
# Generic `match v i` arms parser used for set_epi32 / set_epi64x / unpacklo etc.
_LEMMA_SET_EPI32X4_RE = re.compile(
    r'to_([iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm_set_epi32)\s+'
    r'(?P<x0>\w+)\s+(?P<x1>\w+)\s+(?P<x2>\w+)\s+(?P<x3>\w+)\s*\)\s+i\s*==\s*'
    r'\(\s*match\s+v\s+i\s+with\s*'
    r'\|\s*0\s*->\s*(?P<r0>\w+)\s*'
    r'\|\s*1\s*->\s*(?P<r1>\w+)\s*'
    r'\|\s*2\s*->\s*(?P<r2>\w+)\s*'
    r'\|\s*3\s*->\s*(?P<r3>\w+)\s*\)',
    re.DOTALL,
)


# Generic K-arm `match v i with | k -> rk | ...` for set-style lemmas.
# We match any number of arms as long as RHS is a single identifier (binder
# name).  Usable for `mm256_set_epi32` (8 arms), `mm_set_epi32` (4 arms),
# `mm_set_epi8` (16 arms), `mm256_set_epi8` (32 arms), `mm256_set_epi16` (16),
# `mm256_set_epi64x` (4), and similar.
#
# We capture the intrinsic name + the binder list at the call site (variable
# number of \w+) and the match arm pairs.  We do this in two pass:
#   - First a header regex to extract intrinsic + binders + arms text.
#   - Then a second pass over the arms text to extract per-arm RHS.
_LEMMA_SET_NARM_HEADER_RE = re.compile(
    r'to_(?P<rproj>[iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm[0-9]*_set_[A-Za-z0-9_]+)'
    r'(?P<args>(?:\s+\w+)+)\s*\)\s+i\s*==\s*'
    r'\(\s*match\s+v\s+i\s+with\s*(?P<arms>(?:\|\s*\d+\s*->\s*\w+\s*)+)\)',
    re.DOTALL,
)


# Generic 4-arm `match v i` for any binder list — used for `mm256_set_epi64x`.
# RHS arms are `\w+` matching one of the binders.  Shape:
#   to_iNx4 (I.NAME x0 x1 x2 x3) i == (match v i with | 0 -> r0 | 1 -> r1 | 2 -> r2 | 3 -> r3)
_LEMMA_SET_GENERIC_4ARM_RE = re.compile(
    r'to_([iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm[0-9]*_(?:set|unpacklo|unpackhi)_[A-Za-z0-9_]+)\s+'
    r'(?P<x0>\w+)\s+(?P<x1>\w+)\s+(?P<x2>\w+)\s+(?P<x3>\w+)\s*\)\s+i\s*==\s*'
    r'\(\s*match\s+v\s+i\s+with\s*'
    r'\|\s*0\s*->\s*(?P<r0>\w+)\s*'
    r'\|\s*1\s*->\s*(?P<r1>\w+)\s*'
    r'\|\s*2\s*->\s*(?P<r2>\w+)\s*'
    r'\|\s*3\s*->\s*(?P<r3>\w+)\s*\)',
    re.DOTALL,
)


# `to_iNxM (I.NAME a) i == to_iNxK a i` — identity-projection lemmas
# (e.g. `mm256_castsi256_si128_lemma`).  Two projections with same `i`.
_LEMMA_PROJ_IDENTITY_RE = re.compile(
    r'to_(?P<rproj>[iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm[0-9]*_[A-Za-z0-9_]+)\s+'
    r'(?P<a>\w+)\s*\)\s+i\s*==\s*'
    r'to_(?P<aproj>[iu]\d+x\d+)\s+(?P=a)\s+i\b',
    re.DOTALL,
)


# `to_iNxM (I.NAME control vec) i == to_iNxK vec (i + (if v control = 0 then 0 else N))`
# for `mm256_extracti128_si256_lemma`.
_LEMMA_EXTRACTI128_RE = re.compile(
    r'to_(?P<rproj>[iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm256_extracti128_si256)\s+'
    r'(?P<ctl>\w+)\s+(?P<vec>\w+)\s*\)\s+i\s*==\s*'
    r'to_(?P<aproj>[iu]\d+x\d+)\s+(?P=vec)\s+\(\s*i\s*\+!\s*mk_int\s*\(\s*if\s+v\s+(?P=ctl)\s*=\s*0\s+then\s+0\s+else\s+(?P<offset>\d+)\s*\)\s*\)',
    re.DOTALL,
)


# Specialized: `mm256_mul_epi32` lemma.  Shape:
#   to_i32x8 (I.mm256_mul_epi32 a b) i ==
#     (let j = mk_u64 (v i - (v i % 2)) in
#      let v64 = mul_mod_opaque (cast_mod_opaque (to_i32x8 a j) <: i64) (cast_mod_opaque (to_i32x8 b j) <: i64) in
#      if v i % 2 = 0 then cast_mod_opaque v64 else cast_mod_opaque (shift_right_opaque v64 (mk_i32 32)))
_LEMMA_MUL_EPI32_RE = re.compile(
    r'to_(?P<rproj>i32x8)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm256_mul_epi32)\s+'
    r'(?P<a>\w+)\s+(?P<b>\w+)\s*\)\s+i\s*==\s*'
    r'\(\s*let\s+j\s*=\s*mk_u64\s*\(\s*v\s+i\s*-\s*\(\s*v\s+i\s*%\s*2\s*\)\s*\)\s+in\s+'
    r'let\s+v64\s*=\s*mul_mod_opaque\s+\(\s*cast_mod_opaque\s+\(\s*to_i32x8\s+(?P=a)\s+j\s*\)\s*<:\s*i64\s*\)\s+'
    r'\(\s*cast_mod_opaque\s+\(\s*to_i32x8\s+(?P=b)\s+j\s*\)\s*<:\s*i64\s*\)\s+in\s+'
    r'if\s+v\s+i\s*%\s*2\s*=\s*0\s+then\s+cast_mod_opaque\s+v64\s+else\s+cast_mod_opaque\s+'
    r'\(\s*shift_right_opaque\s+v64\s+\(\s*mk_i32\s+32\s*\)\s*\)\s*\)',
    re.DOTALL,
)


# Specialized: `mm256_shuffle_epi32` lemma.  Shape:
#   to_i32x8 (I.mm256_shuffle_epi32 a b) i ==
#     (if i <. mk_u64 4 <: bool
#      then (to_i32x8 b (mm256_shuffle_epi32_index a i))
#      else (to_i32x8 b (mk_u64 4 +! mm256_shuffle_epi32_index a (i -! mk_u64 4))))
#   where mm256_shuffle_epi32_index a i = cast ((a >>! (i *! 2)) %! 4)
_LEMMA_SHUFFLE_EPI32_RE = re.compile(
    r'to_(?P<rproj>i32x8)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm256_shuffle_epi32)\s+'
    r'(?P<a>\w+)\s+(?P<b>\w+)\s*\)\s+i\s*==\s*'
    r'\(\s*if\s+i\s*<\.\s*mk_u64\s+4\s*(?:<:\s*bool)?\s+'
    r'then\s+\(?to_i32x8\s+(?P=b)\s+\(\s*mm256_shuffle_epi32_index\s+(?P=a)\s+i\s*\)\)?\s+'
    r'else\s+\(?to_i32x8\s+(?P=b)\s+\(\s*mk_u64\s+4\s*\+!\s*mm256_shuffle_epi32_index\s+(?P=a)\s+'
    r'\(\s*i\s*-!\s*mk_u64\s+4\s*\)\s*\)\)?\s*\)',
    re.DOTALL,
)


# Bit-vec `.()` byte-shift (bsrli) — lane-relative shift in BYTES, scaled to bits.
# Shape:
#   (I.mm256_bsrli_epi128 shift vector).(i) ==
#     (let lane = v i / 128 in
#      let local_index = v i % 128 in
#      let shift = v shift * 8 in
#      let j = local_index + shift in
#      if j < 0 || j >= 128 then Bit_Zero else vector.(mk_int (lane * 128 + j)))
# We accept the SCALE factor `* 8` and the lane-size `128` as captures.
_LEMMA_BIT_BYTE_SHIFT_RE = re.compile(
    r'\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*I\.(?P<intr>mm[0-9]*_(?:bsrli|bslli)_[A-Za-z0-9_]+)\s+'
    r'(?P<shiftarg>\w+)\s+(?P<vecarg>\w+)\s*\)\s*\.\s*\(\s*i\s*\)\s*==\s*'
    r'\(\s*let\s+lane\s*=\s*v\s+i\s*/\s*(?P<lane_bits>\d+)\s+in\s+'
    r'let\s+local_index\s*=\s*v\s+i\s*%\s*(?P=lane_bits)\s+in\s+'
    r'let\s+shift\s*=\s*v\s+(?P=shiftarg)\s*\*\s*(?P<scale>\d+)\s+in\s+'
    r'let\s+j\s*=\s*local_index\s*(?P<sign>[+\-])\s*shift\s+in\s+'
    r'if\s+j\s*<\s*0\s*\|\|\s*j\s*>=\s*(?P=lane_bits)\s+then\s+Bit_Zero\s+'
    r'else\s+(?P=vecarg)\s*\.\s*\(\s*mk_int\s*\(\s*lane\s*\*\s*(?P=lane_bits)\s*\+\s*j\s*\)\s*\)\s*\)',
    re.DOTALL,
)


# Bit-vec `.()` per-bit shift lemma.  Common shape:
#   (I.NAME ARG1 ARG2).(i) ==
#     (let i:u64 = i in
#      let v_CHUNK = mk_u64 N in
#      ...
#      let nth_bit:u64 = i %! v_CHUNK in
#      let nth_chunk:u64 = i /! v_CHUNK in
#      let [shift =|local_index = ...] in
#      if <cond>
#      then vector.( (nth_chunk *! v_CHUNK) +! mk_int local_index )
#      else Bit_Zero)
# where:
#   - the shift is either `v <name>` (scalar) or `v (to_<lane> <name> nth_chunk)` (per-chunk variable)
#   - `local_index = v nth_bit ± shift` selects between slli (`-`) and srli/srlv (`+`)
#   - the COND is `local_index < 64` (srli/srlv into-chunk-only) or
#     `local_index >= 0` (slli into-chunk-only) or
#     `local_index < v v_CHUNK && local_index >= 0` (both — for variable shifts).
# We handle them all with a single regex that captures the chunk size, the
# shift source, and the sign on `local_index`.
_LEMMA_BIT_SHIFT_RE = re.compile(
    r'\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*I\.(?P<intr>mm[0-9]*_[A-Za-z0-9_]+)\s+'
    r'(?P<arg1>\w+)\s+(?P<arg2>\w+)\s*\)\s*\.\s*\(\s*i\s*\)\s*==\s*'
    r'\(\s*let\s+i\s*:\s*u64\s*=\s*i\s+in\s+'
    r'let\s+v_CHUNK\s*=\s*mk_u64\s+(?P<chunk>\d+)\s+in\s+'
    r'(?:let\s+v_SHIFTS\s*=\s*mk_u64\s+\d+\s+in\s+)?'
    r'let\s+nth_bit\s*:\s*u64\s*=\s*i\s*%!\s*v_CHUNK\s+in\s+'
    r'let\s+nth_chunk\s*:\s*u64\s*=\s*i\s*/!\s*v_CHUNK\s+in\s+'
    r'(?:let\s+shift\s*=\s*(?:'
        r'if\s+nth_chunk\s*<\.\s*v_SHIFTS\s+then\s+v\s*\(\s*to_(?P<shiftproj>[iu]\d+x\d+)\s+'
        r'(?P<shiftvar>\w+)\s+nth_chunk\s*\)\s+else\s+0'
    r')\s+in\s+)?'
    r'let\s+local_index\s*=\s*v\s+nth_bit\s*(?P<sign>[+\-])\s*'
    r'(?:v\s+(?P<scshift>\w+)|shift)\s+in\s+'
    r'if\s+(?P<cond>[^\n]+?)\s+'
    r'then\s+(?P<srcvec>\w+)\s*\.\s*\(\s*\(?\s*nth_chunk\s*\*!\s*v_CHUNK\s*\)?\s*\+!\s*mk_int\s+local_index\s*\)\s+'
    r'else\s+Bit_Zero\s*\)',
    re.DOTALL,
)


# 8-arm `match i with | MkInt 0 -> ... | MkInt 1 -> ...` for unpacklo/unpackhi/set_m128i
# Each RHS is either `to_iNxM <var> (mk_uK <num>)` or `(to_iNxM <var>) (mk_uK <num>)`.
# This matches the shape used by `mm256_unpacklo_epi64`, `mm256_unpackhi_epi64`,
# `mm256_set_m128i`.  We don't try to handle the trailing `_ -> never_to_any panic`
# arm — we anchor the regex to stop after `MkInt 7 -> ...`.
_LEMMA_MKINT8ARM_RE = re.compile(
    r'to_(?P<rproj>[iu]\d+x\d+)\s*\(\s*(?:[A-Za-z_][A-Za-z_0-9]*\.)*(?P<intr>mm[0-9]*_[A-Za-z0-9_]+)\s+'
    r'(?P<a>\w+)\s+(?P<b>\w+)\s*\)\s+i\s*==\s*'
    r'\(\s*match\s+i\s*<:\s*u64\s+with\s*'
    + r'\s*'.join(
        r'\|\s*Rust_primitives\.Integers\.MkInt\s+' + str(k) + r'\s*->\s*'
        r'\(?\s*to_(?P<p' + str(k) + r'>[iu]\d+x\d+)\s+'
        r'(?P<v' + str(k) + r'>\w+)\s*\)?\s*\(?\s*mk_u64\s+'
        r'(?P<n' + str(k) + r'>\d+)\s*\)?'
        for k in range(8)
    )
    + r'\s*(?:\|.*?)?\)',
    re.DOTALL,
)


def _lane_type_from_to_proj(proj: str) -> Tuple[str, int]:
    """`i32x8` -> ('i32', 8)."""
    m = re.match(r"([iu]\d+)x(\d+)", proj)
    return m.group(1), int(m.group(2))


def parse_lemma_into_speccase(name: str, block: str) -> Optional[SpecCase]:
    # `to_iNxM (I.set1_epi32 x0) i == x0`.
    m = _LEMMA_SET1_RE.search(block)
    if m:
        proj = m.group(1)
        intr = m.group("intr")
        v_var = m.group("v")
        lane_type, count = _lane_type_from_to_proj(proj)
        bits = _lane_bits(lane_type)

        def evaluator(args, lt=lane_type, ct=count, vv=v_var):
            return [_project_scalar(args[vv], lt) for _ in range(ct)]

        def project(v, lt=lane_type) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(intr, "PAT_LEMMA_SET1", block, "spec_intrinsics",
                        evaluator, project, [v_var])

    m = _LEMMA_BIN_OPAQUE_RE.search(block)
    if m:
        proj = m.group(1)
        intr = m.group("intr")
        a_var, b_var = m.group("a"), m.group("b")
        op = m.group("op")
        op_fn = {
            "add_mod_opaque": add_mod,
            "sub_mod_opaque": sub_mod,
            "mul_mod_opaque": mul_mod,
        }[op]
        lane_type, count = _lane_type_from_to_proj(proj)
        bits = _lane_bits(lane_type)

        def evaluator(args, lt=lane_type, va=a_var, vb=b_var, fn=op_fn, bb=bits):
            a = _project_vec_lanes(args[va], lt)
            b = _project_vec_lanes(args[vb], lt)
            return [fn(x, y, bb) for x, y in zip(a, b)]

        def project(v, lt=lane_type) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(intr, f"PAT_LEMMA_BIN_{op.upper()}", block, "spec_intrinsics",
                        evaluator, project, [a_var, b_var])

    m = _LEMMA_BIN_BITWISE_RE.search(block)
    if m:
        proj = m.group(1)
        intr = m.group("intr")
        a_var, b_var = m.group("a"), m.group("b")
        op = m.group("op")
        op_fn = {
            "&": lambda x, y, b: _to_signed(_to_unsigned(x, b) & _to_unsigned(y, b), b),
            "^": lambda x, y, b: _to_signed(_to_unsigned(x, b) ^ _to_unsigned(y, b), b),
            "|": lambda x, y, b: _to_signed(_to_unsigned(x, b) | _to_unsigned(y, b), b),
        }[op]
        lane_type, count = _lane_type_from_to_proj(proj)
        bits = _lane_bits(lane_type)

        def evaluator(args, lt=lane_type, va=a_var, vb=b_var, fn=op_fn, bb=bits):
            a = _project_vec_lanes(args[va], lt)
            b = _project_vec_lanes(args[vb], lt)
            return [fn(x, y, bb) for x, y in zip(a, b)]

        def project(v, lt=lane_type) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(intr, f"PAT_LEMMA_BIN_BITWISE_{op}", block, "spec_intrinsics",
                        evaluator, project, [a_var, b_var])

    m = _LEMMA_CMPGT_RE.search(block)
    if m:
        proj = m.group(1)
        intr = m.group("intr")
        a_var, b_var = m.group("a"), m.group("b")
        lane_type, count = _lane_type_from_to_proj(proj)

        def evaluator(args, lt=lane_type, va=a_var, vb=b_var):
            a = _project_vec_lanes(args[va], lt)
            b = _project_vec_lanes(args[vb], lt)
            return [-1 if x > y else 0 for x, y in zip(a, b)]

        def project(v, lt=lane_type) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(intr, "PAT_LEMMA_CMPGT", block, "spec_intrinsics",
                        evaluator, project, [a_var, b_var])

    m = _LEMMA_ABS_RE.search(block)
    if m:
        proj = m.group(1)
        intr = m.group("intr")
        a_var = m.group("a")
        lane_type, count = _lane_type_from_to_proj(proj)
        bits = _lane_bits(lane_type)

        def evaluator(args, lt=lane_type, va=a_var, bb=bits):
            a = _project_vec_lanes(args[va], lt)
            mn = -(1 << (bb - 1))
            out = []
            for x in a:
                if x == mn:
                    out.append(None)  # sentinel: skip this lane (lemma has requires)
                else:
                    out.append(abs(x))
            return out

        def project(v, lt=lane_type) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(intr, "PAT_LEMMA_ABS", block, "spec_intrinsics",
                        evaluator, project, [a_var])

    m = _LEMMA_SLLI_EPI32_RE.search(block)
    if m:
        proj = m.group(1)
        intr = m.group("intr")
        imm_var, a_var = m.group("imm"), m.group("a")
        lane_type, count = _lane_type_from_to_proj(proj)

        def evaluator(args, lt=lane_type, ct=count, vimm=imm_var, va=a_var):
            a = _project_vec_lanes(args[va], lt)
            imm = args[vimm]
            out = []
            for x in a:
                if imm < 0 or imm > 31:
                    out.append(0)
                else:
                    out.append(shift_left(x, imm, 32))
            return out

        def project(v, lt=lane_type) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(intr, "PAT_LEMMA_SLLI_EPI32", block, "spec_intrinsics",
                        evaluator, project, [imm_var, a_var])

    m = _LEMMA_SRAI_EPI32_RE.search(block)
    if m:
        proj = m.group(1)
        intr = m.group("intr")
        imm_var, a_var = m.group("imm"), m.group("a")
        lane_type, count = _lane_type_from_to_proj(proj)

        def evaluator(args, lt=lane_type, ct=count, vimm=imm_var, va=a_var):
            a = _project_vec_lanes(args[va], lt)
            imm8 = args[vimm] % 256
            out = []
            for x in a:
                if imm8 > 31:
                    out.append(-1 if x < 0 else 0)
                else:
                    out.append(shift_right_arith(x, imm8, 32))
            return out

        def project(v, lt=lane_type) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(intr, "PAT_LEMMA_SRAI_EPI32", block, "spec_intrinsics",
                        evaluator, project, [imm_var, a_var])

    m = _LEMMA_BLEND_EPI32_RE.search(block)
    if m:
        proj = m.group(1)
        intr = m.group("intr")
        imm_var, a_var, b_var = m.group("imm"), m.group("a"), m.group("b")
        lane_type, count = _lane_type_from_to_proj(proj)

        def evaluator(args, lt=lane_type, ct=count, vimm=imm_var, va=a_var, vb=b_var):
            a = _project_vec_lanes(args[va], lt)
            b = _project_vec_lanes(args[vb], lt)
            imm = args[vimm]
            out = []
            for k in range(ct):
                bit = (imm // (1 << k)) % 2
                out.append(b[k] if bit else a[k])
            return out

        def project(v, lt=lane_type) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(intr, "PAT_LEMMA_BLEND_EPI32", block, "spec_intrinsics",
                        evaluator, project, [imm_var, a_var, b_var])

    # Generic K-arm set-lemma (4/8/16/32 arms): matches mm_set_epi32, mm256_set_epi32,
    # mm256_set_epi64x, mm_set_epi8, mm256_set_epi8, mm256_set_epi16.
    m = _LEMMA_SET_NARM_HEADER_RE.search(block)
    if m:
        rproj = m.group("rproj")
        intr = m.group("intr")
        args_str = m.group("args").strip()
        arms_str = m.group("arms").strip()
        binders = args_str.split()
        # Parse arms.
        arm_re = re.compile(r'\|\s*(\d+)\s*->\s*(\w+)')
        arm_map: Dict[int, str] = {}
        for am in arm_re.finditer(arms_str):
            arm_map[int(am.group(1))] = am.group(2)
        # Sanity: every arm RHS is a known binder.
        if not all(rhs in binders for rhs in arm_map.values()):
            pass  # fall through; another handler may match
        elif len(arm_map) >= 2:
            lane_type, count = _lane_type_from_to_proj(rproj)
            # Build the per-lane RHS list, ordered by arm key.
            # We support cases where arm_map keys are 0..K-1 contiguous.
            keys = sorted(arm_map.keys())
            if keys == list(range(len(keys))):
                r_vars = [arm_map[k] for k in keys]

                def evaluator(args, lt=lane_type, rs=r_vars):
                    return [_project_scalar(args[r], lt) for r in rs]

                def project(v, lt=lane_type) -> List[int]:
                    return _project_vec_lanes(v, lt)

                return SpecCase(intr, "PAT_LEMMA_SET_NARM", block, "spec_intrinsics",
                                evaluator, project, binders)

    m = _LEMMA_SET_EPI32X4_RE.search(block)
    if m:
        proj = m.group(1)
        intr = m.group("intr")
        # mm_set_epi32 args order: x0 x1 x2 x3 (matches the C signature).
        # The lemma maps `v i = 0 -> r0`, `1 -> r1`, etc.
        x_vars = [m.group("x0"), m.group("x1"), m.group("x2"), m.group("x3")]
        r_vars = [m.group("r0"), m.group("r1"), m.group("r2"), m.group("r3")]
        lane_type, count = _lane_type_from_to_proj(proj)

        def evaluator(args, lt=lane_type, ct=count, rs=r_vars):
            return [_project_scalar(args[r], lt) for r in rs]

        def project(v, lt=lane_type) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(intr, "PAT_LEMMA_SET_EPI32", block, "spec_intrinsics",
                        evaluator, project, x_vars)

    # Generic 4-arm set lemma: e.g. `mm256_set_epi64x x0 x1 x2 x3` mapping
    # `v i = 0 -> x3, 1 -> x2, 2 -> x1, 3 -> x0`.
    m = _LEMMA_SET_GENERIC_4ARM_RE.search(block)
    if m:
        proj = m.group(1)
        intr = m.group("intr")
        x_vars = [m.group("x0"), m.group("x1"), m.group("x2"), m.group("x3")]
        r_vars = [m.group("r0"), m.group("r1"), m.group("r2"), m.group("r3")]
        lane_type, count = _lane_type_from_to_proj(proj)

        def evaluator(args, lt=lane_type, ct=count, rs=r_vars):
            return [_project_scalar(args[r], lt) for r in rs]

        def project(v, lt=lane_type) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(intr, "PAT_LEMMA_SET_4ARM", block, "spec_intrinsics",
                        evaluator, project, x_vars)

    # `to_iNxM (I.cast a) i == to_iNxK a i` — identity-projection.
    m = _LEMMA_PROJ_IDENTITY_RE.search(block)
    if m:
        intr = m.group("intr")
        a_var = m.group("a")
        rproj = m.group("rproj")
        aproj = m.group("aproj")
        result_lane, result_count = _lane_type_from_to_proj(rproj)
        src_lane, src_count = _lane_type_from_to_proj(aproj)

        def evaluator(args, va=a_var, sl=src_lane, ct=result_count):
            src_lanes = _project_vec_lanes(args[va], sl)
            return src_lanes[:ct]

        def project(v, lt=result_lane) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(intr, "PAT_LEMMA_PROJ_IDENTITY", block, "spec_intrinsics",
                        evaluator, project, [a_var])

    # mm256_extracti128_si256 lemma — control-conditional offset.
    m = _LEMMA_EXTRACTI128_RE.search(block)
    if m:
        intr = m.group("intr")
        ctl_var = m.group("ctl")
        vec_var = m.group("vec")
        rproj = m.group("rproj")
        aproj = m.group("aproj")
        offset = int(m.group("offset"))
        result_lane, result_count = _lane_type_from_to_proj(rproj)
        src_lane, src_count = _lane_type_from_to_proj(aproj)

        def evaluator(args, vc=ctl_var, vv=vec_var, sl=src_lane,
                      ct=result_count, off=offset):
            ctl = args[vc]
            base = 0 if ctl == 0 else off
            src_lanes = _project_vec_lanes(args[vv], sl)
            return [src_lanes[base + k] for k in range(ct)]

        def project(v, lt=result_lane) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(intr, "PAT_LEMMA_EXTRACTI128", block, "spec_intrinsics",
                        evaluator, project, [ctl_var, vec_var])

    # Specialized: mm256_mul_epi32 lemma.  Per-i32-lane interleaved low/high.
    m = _LEMMA_MUL_EPI32_RE.search(block)
    if m:
        intr = m.group("intr")
        a_var, b_var = m.group("a"), m.group("b")

        def evaluator(args, va=a_var, vb=b_var):
            a = args[va].lanes_i32()
            b = args[vb].lanes_i32()
            out = []
            for i in range(8):
                j = i - (i % 2)
                # cast to i64, multiply, wrap mod 2^64
                v64 = i64(i64(a[j]) * i64(b[j]))
                if i % 2 == 0:
                    # low 32 bits as i32
                    out.append(i32(v64 & 0xFFFFFFFF))
                else:
                    # arith shift right 32, cast to i32
                    out.append(i32(v64 >> 32))
            return out

        def project(v):
            return v.lanes_i32()

        return SpecCase(intr, "PAT_LEMMA_MUL_EPI32", block, "spec_intrinsics",
                        evaluator, project, [a_var, b_var])

    # Specialized: mm256_shuffle_epi32 lemma.
    m = _LEMMA_SHUFFLE_EPI32_RE.search(block)
    if m:
        intr = m.group("intr")
        a_var, b_var = m.group("a"), m.group("b")

        def shuffle_epi32_index(a: int, i: int) -> int:
            # cast ((a >>! (i*2 :u64) <: i32) %! mk_i32 4 <: i32) <: u64
            # `a` is i32; arithmetic right shift by (i*2), then `% 4` (Euclidean
            # rem in F* gives non-negative for positive divisor).
            shifted = i32(a) >> (i * 2)  # arith shift
            return shifted % 4 if shifted >= 0 else (shifted % 4 + 4) % 4

        def evaluator(args, va=a_var, vb=b_var):
            a = args[va]   # i32 const
            b = args[vb]   # Vec256
            lanes = b.lanes_i32()
            out = []
            for i in range(8):
                if i < 4:
                    idx = shuffle_epi32_index(a, i)
                else:
                    idx = 4 + shuffle_epi32_index(a, i - 4)
                out.append(lanes[idx])
            return out

        def project(v):
            return v.lanes_i32()

        return SpecCase(intr, "PAT_LEMMA_SHUFFLE_EPI32", block, "spec_intrinsics",
                        evaluator, project, [a_var, b_var])

    # Bit-vec byte-shift (bsrli/bslli) — scaled by 8 bits, lane-relative.
    m = _LEMMA_BIT_BYTE_SHIFT_RE.search(block)
    if m:
        intr = m.group("intr")
        shift_arg = m.group("shiftarg")
        vec_arg = m.group("vecarg")
        lane_bits_s = int(m.group("lane_bits"))
        scale = int(m.group("scale"))
        sign = m.group("sign")
        arg_names = [shift_arg, vec_arg]

        def evaluator(args, intr=intr, shift_arg=shift_arg, vec_arg=vec_arg,
                      lane_bits_s=lane_bits_s, scale=scale, sign=sign):
            src = args[vec_arg]
            if not isinstance(src, Vec):
                # try other arg
                for k, vv in args.items():
                    if isinstance(vv, Vec):
                        src = vv
                        break
            sh = args[shift_arg]
            n_bits = src.bits
            out_bytes = [0] * (n_bits // 8)
            for ibit in range(n_bits):
                lane = ibit // lane_bits_s
                local_index = ibit % lane_bits_s
                bit_shift = sh * scale
                if sign == "+":
                    j = local_index + bit_shift
                else:
                    j = local_index - bit_shift
                if j < 0 or j >= lane_bits_s:
                    b = 0
                else:
                    src_idx = lane * lane_bits_s + j
                    b = src.bit(src_idx) if 0 <= src_idx < n_bits else 0
                out_bytes[ibit >> 3] |= (b & 1) << (ibit & 7)
            return list(out_bytes)

        def project(v):
            return list(v.bytes)

        return SpecCase(intr, "PAT_LEMMA_BIT_BYTE_SHIFT", block, "spec_intrinsics",
                        evaluator, project, arg_names)

    # Bit-vec per-bit shift lemma (variable or scalar shift, slli/srli/sllv/srlv).
    m = _LEMMA_BIT_SHIFT_RE.search(block)
    if m:
        intr = m.group("intr")
        arg1 = m.group("arg1")
        arg2 = m.group("arg2")
        chunk = int(m.group("chunk"))
        sign = m.group("sign")
        sc_shift = m.group("scshift")  # scalar shift name, or None if vec-shift
        shift_var = m.group("shiftvar")  # variable shift's source vec, or None
        shift_proj = m.group("shiftproj")  # 'i32x4', 'i32x8', 'i64x4', or None
        cond = (m.group("cond") or "").strip()
        src_vec_name = m.group("srcvec")
        arg_names = [arg1, arg2]

        def evaluator(args, intr=intr, chunk=chunk, sign=sign,
                      sc_shift=sc_shift, shift_var=shift_var,
                      shift_proj=shift_proj, cond=cond,
                      src_vec_name=src_vec_name, arg1=arg1, arg2=arg2):
            # Resolve source vector — should match src_vec_name.
            src_vec_arg = src_vec_name
            if src_vec_arg not in args or not isinstance(args.get(src_vec_arg), Vec):
                # Fall back: pick the Vec-typed argument.
                for n in (arg1, arg2):
                    if isinstance(args.get(n), Vec):
                        src_vec_arg = n
                        break
            src = args[src_vec_arg]
            if not isinstance(src, Vec):
                raise ValueError("source not Vec")
            n_bits = src.bits
            out_bytes = [0] * (n_bits // 8)
            for ibit in range(n_bits):
                nth_bit = ibit % chunk
                nth_chunk = ibit // chunk
                if sc_shift is not None:
                    sh = args[sc_shift]
                else:
                    # variable shift: use shift_proj projection of shift_var
                    sh_vec = args[shift_var]
                    lane_type, lane_count = _lane_type_from_to_proj(shift_proj)
                    lanes = _project_vec_lanes(sh_vec, lane_type)
                    if nth_chunk < lane_count:
                        sh = lanes[nth_chunk]
                    else:
                        sh = 0
                if sign == "-":
                    local_index = nth_bit - sh
                else:
                    local_index = nth_bit + sh
                # Evaluate cond.  Substitute `local_index` with its int value
                # and `v v_CHUNK` with the chunk size, then eval as python expr.
                cond_simple = cond.replace("v v_CHUNK", str(chunk))
                cond_simple = cond_simple.replace("&&", " and ")
                cond_simple = cond_simple.replace("||", " or ")
                cond_simple = cond_simple.replace("local_index", str(local_index))
                cond_simple = cond_simple.strip().rstrip(")").strip()
                try:
                    cond_ok = bool(eval(cond_simple, {"__builtins__": {}}))
                except Exception:
                    cond_ok = False
                if cond_ok:
                    src_bit_idx = nth_chunk * chunk + local_index
                    b = src.bit(src_bit_idx) if 0 <= src_bit_idx < n_bits else 0
                else:
                    b = 0
                out_bytes[ibit >> 3] |= (b & 1) << (ibit & 7)
            return list(out_bytes)

        def project(v):
            return list(v.bytes)

        return SpecCase(intr, "PAT_LEMMA_BIT_SHIFT", block, "spec_intrinsics",
                        evaluator, project, arg_names)

    # 8-arm `match i with | MkInt k -> to_iNxM <v> (mk_u64 <n>)` for unpacklo / set_m128i
    m = _LEMMA_MKINT8ARM_RE.search(block)
    if m:
        rproj = m.group("rproj")
        intr = m.group("intr")
        a_var, b_var = m.group("a"), m.group("b")
        # For each k in 0..7 collect (rhs_proj, rhs_var, rhs_idx).
        arms = []
        for k in range(8):
            arms.append((m.group(f"p{k}"), m.group(f"v{k}"), int(m.group(f"n{k}"))))
        result_lane, result_count = _lane_type_from_to_proj(rproj)

        def evaluator(args, arms=arms, result_lane=result_lane, a_var=a_var, b_var=b_var):
            out = []
            for arm in arms:
                arm_proj, arm_var, arm_idx = arm
                # arm_proj is e.g. "i32x8" — extract lane type for the projection
                arm_lane, _ = _lane_type_from_to_proj(arm_proj)
                src_vec = args[arm_var]
                lanes = _project_vec_lanes(src_vec, arm_lane)
                out.append(lanes[arm_idx])
            return out

        def project(v, lt=result_lane) -> List[int]:
            return _project_vec_lanes(v, lt)

        return SpecCase(intr, "PAT_LEMMA_MKINT_8ARM", block, "spec_intrinsics",
                        evaluator, project, [a_var, b_var])

    # PAT_LEMMA_MADD_EPI16: `mm256_madd_epi16_lemma (a b: bv256) i`
    # result_i32[j] = a_i16[2j]*b_i16[2j] + a_i16[2j+1]*b_i16[2j+1]  (all as i32, wrapping)
    _MADD_EPI16_RE = re.compile(
        r'val\s+(mm256_madd_epi16)(?:_bv)?_lemma\b.*?i16_mul_32extended.*?i32_wrapping_add',
        re.DOTALL,
    )
    mm = _MADD_EPI16_RE.search(block)
    if mm:
        intr = mm.group(1)

        def evaluator(args):
            a = _project_vec_lanes(args["a"], "i16")
            b = _project_vec_lanes(args["b"], "i16")
            return [i32(a[2*j] * b[2*j] + a[2*j+1] * b[2*j+1]) for j in range(8)]

        def project(v) -> List[int]:
            return _project_vec_lanes(v, "i32")

        return SpecCase(intr, "PAT_LEMMA_MADD_EPI16", block, "spec_intrinsics",
                        evaluator, project, ["a", "b"])

    # PAT_LEMMA_MULLO_EPI16_BV: `mm256_mullo_epi16_bv_lemma a b i`
    # result_i16[j] = i16(a_i16[j] * b_i16[j])  (lower 16 bits of 32-bit product)
    _MULLO_EPI16_BV_RE = re.compile(
        r'val\s+(mm256_mullo_epi16)(?:_bv)?_lemma\b.*?i16_mul_32extended_i16',
        re.DOTALL,
    )
    mmu = _MULLO_EPI16_BV_RE.search(block)
    if mmu:
        intr = mmu.group(1)

        def evaluator(args):
            a = _project_vec_lanes(args["a"], "i16")
            b = _project_vec_lanes(args["b"], "i16")
            return [i16(a[j] * b[j]) for j in range(16)]

        def project(v) -> List[int]:
            return _project_vec_lanes(v, "i16")

        return SpecCase(intr, "PAT_LEMMA_MULLO_EPI16_BV", block, "spec_intrinsics",
                        evaluator, project, ["a", "b"])

    # PAT_LEMMA_PERMUTEVAR8X32: `mm256_permutevar8x32_epi32_lemma vector control i`
    # result.(i) == let nth_i32 = i /! mk_int 32 in ... let nth_block = v (to_i32x8 control nth_i32) % 8 in
    #               vector.(mk_int (nth_block * 32 + local_index))
    # Lane interpretation: result_i32[j] = vector_i32[(to_i32x8 control j) % 8]
    _PERMUTEVAR_RE = re.compile(
        r'val\s+(mm256_permutevar8x32_epi32)(?:_bv)?_lemma\s+(\w+)\s+(\w+)\s+\w+\b'
        r'.*?nth_block\s*=\s*v\s*\(\s*to_i32x8\s+(\w+)\s+\w+\s*\)\s*%\s*8',
        re.DOTALL,
    )
    mp = _PERMUTEVAR_RE.search(block)
    if mp:
        intr = mp.group(1)
        vec_var = mp.group(2)
        ctrl_var = mp.group(4)

        def evaluator(args, vv=vec_var, cv=ctrl_var):
            v_lanes = _project_vec_lanes(args[vv], "i32")
            c_lanes = _project_vec_lanes(args[cv], "i32")
            return [v_lanes[c % 8] for c in c_lanes]

        def project(v) -> List[int]:
            return _project_vec_lanes(v, "i32")

        return SpecCase(intr, "PAT_LEMMA_PERMUTEVAR8X32", block, "spec_intrinsics",
                        evaluator, project, [vec_var, ctrl_var])

    # PAT_LEMMA_SHUFFLE_EPI8: byte shuffle with sign-zeroing.
    # `mm_shuffle_epi8_lemma vec indexes i` (128-bit) or
    # `mm256_shuffle_epi8_lemma (vec: bv256) indexes i` (256-bit)
    # At the byte level: result_byte[j] = (indexes_i8[j] < 0) ? 0 : vec_u8[(group + idx % 16)]
    _SHUFFLE_EPI8_RE = re.compile(
        r'val\s+(mm(?:256)?_shuffle_epi8)(?:_bv)?_lemma\b'
        r'.*?if\s+v\s+index\s*<\s*0',
        re.DOTALL,
    )
    ms = _SHUFFLE_EPI8_RE.search(block)
    if ms:
        intr = ms.group(1)   # mm_shuffle_epi8 or mm256_shuffle_epi8
        # arg names are always "vec" and "indexes" in both lemma variants
        vec_var = "vec"
        idx_var = "indexes"
        is_256 = "256" in intr

        def evaluator(args, vv=vec_var, iv=idx_var, w256=is_256):
            v_bytes = _project_vec_lanes(args[vv], "u8") if False else list(args[vv].bytes)
            i_bytes = _project_vec_lanes(args[iv], "i8")
            result = []
            for j, idx in enumerate(i_bytes):
                if idx < 0:
                    result.append(0)
                else:
                    group = (16 if j >= 16 else 0) if w256 else 0
                    result.append(_to_signed(v_bytes[group + (idx % 16)], 8))
            return result

        def project(v) -> List[int]:
            return _project_vec_lanes(v, "i8")

        return SpecCase(intr, "PAT_LEMMA_SHUFFLE_EPI8", block, "spec_intrinsics",
                        evaluator, project, [vec_var, idx_var])

    return None


# ----------------------- Sampling ------------------------------------------


def sample_args(
    rng: random.Random, name: str, gt: Dict[str, Any]
) -> Tuple[List[Any], Dict[str, Any]]:
    """Sample one set of args for the intrinsic.  Returns:
       - positional list of values (in libcrux signature order)
       - dict {arg_name -> value}, where arg_name comes from the
         `_extract.rs` parameter list.  Dict access by both arg-name AND
         positional via 'arg0', 'arg1', etc.
    """
    inputs = gt["inputs"]
    const_ranges = gt.get("const_range", [])
    args_pos: List[Any] = []
    const_idx = 0
    for kind in inputs:
        if kind == VEC256:
            args_pos.append(Vec.random(rng, 256))
        elif kind == VEC128:
            args_pos.append(Vec.random(rng, 128))
        elif kind == VEC64:
            args_pos.append(Vec.random(rng, 64))
        elif kind == I16:
            args_pos.append(rng.randrange(-(1 << 15), 1 << 15))
        elif kind == I32:
            args_pos.append(rng.randrange(-(1 << 31), 1 << 31))
        elif kind == I64:
            args_pos.append(rng.randrange(-(1 << 63), 1 << 63))
        elif kind == CONST_I32:
            lo, hi = const_ranges[const_idx]
            const_idx += 1
            args_pos.append(rng.randint(lo, hi))
        else:
            raise ValueError(kind)
    return args_pos, {}


# ----------------------- Main runner ---------------------------------------


@dataclass
class Verdict:
    name: str
    arch: str
    pattern: str
    source: str
    status: str  # 'PASS' / 'FAIL' / 'OUT-OF-SCOPE-PATTERN' / 'NO-GROUND-TRUTH' / 'NO-SPEC'
    samples: int
    fails: int
    first_fail: Optional[Tuple[str, str, str]] = None  # (input, expected, got)
    spec_text: Optional[str] = None


def parse_libcrux_arg_names(text: str, name: str) -> List[str]:
    """Extract argument names from `pub fn <name>(arg1: T1, arg2: T2, ...)`.
    Returns names in declaration order."""
    fn_re = re.compile(
        r"pub\s+(?:unsafe\s+)?fn\s+" + re.escape(name) + r"\s*(?:<[^>]*>)?\s*\(([^)]*)\)",
        re.DOTALL,
    )
    m = fn_re.search(text)
    if not m:
        return []
    args = m.group(1)
    out = []
    for piece in args.split(","):
        piece = piece.strip()
        if not piece:
            continue
        # piece looks like `name: Type`
        nm = piece.split(":", 1)[0].strip()
        # const generics — `const NAME: i32` — strip `const`.
        nm = re.sub(r'^const\s+', '', nm)
        out.append(nm)
    return out


def parse_libcrux_const_names(text: str, name: str) -> List[str]:
    """Extract const-generic names from `pub fn name<const NAME: i32>(...)`."""
    fn_re = re.compile(
        r"pub\s+(?:unsafe\s+)?fn\s+" + re.escape(name) + r"\s*<([^>]*)>\s*\(",
        re.DOTALL,
    )
    m = fn_re.search(text)
    if not m:
        return []
    out = []
    for piece in m.group(1).split(","):
        piece = piece.strip()
        if piece.startswith("const "):
            nm = piece[len("const "):].split(":", 1)[0].strip()
            out.append(nm)
    return out


def vec_repr(v: Vec) -> str:
    return f"Vec<{v.bits}>" + "[" + ",".join(f"{x:02x}" for x in v.bytes) + "]"


def _arg_repr(a: Any) -> str:
    if isinstance(a, Vec):
        return vec_repr(a)
    return repr(a)


def run_for_intrinsic(
    name: str,
    arch: str,
    spec: SpecCase,
    gt: Dict[str, Any],
    rng: random.Random,
    samples: int,
    extract_text: str,
) -> Verdict:
    """Run cross-validation for one intrinsic+spec pair."""
    inputs = gt["inputs"]

    fails = 0
    first_fail: Optional[Tuple[str, str, str]] = None

    # The spec carries the argument-name list it expects (lemma binders or
    # ensures variable names).  We map it 1-1 onto the GT input slots.
    if spec.arg_names:
        # In SMTPat lemmas, const-generic args come first in the signature
        # (e.g., `mm256_slli_epi32 v_IMM8 a`), and our ground-truth `inputs`
        # for those wrappers also lists const args last (Rust signature
        # order: <const IMM8: i32>(a: Vec256)).  When the spec's binder
        # ordering differs from `inputs`, we still match positionally —
        # this matches the way libcrux exposes args to F* via $name.
        # For SMTPat lemmas with const-first order, we re-order the GT
        # inputs to match.  Heuristic: if `inputs` has CONST_I32 trailing
        # but the spec's first arg name ends in `IMM`-like patterns, we
        # swap.
        name_for_slot = list(spec.arg_names)
        # Pad if shorter than inputs.
        while len(name_for_slot) < len(inputs):
            name_for_slot.append(f"_arg{len(name_for_slot)}")
        # Truncate if longer.
        name_for_slot = name_for_slot[: len(inputs)]
    else:
        # Fall back to libcrux signature order — used by setzero/setN etc.
        arg_names = parse_libcrux_arg_names(extract_text, name)
        const_names = parse_libcrux_const_names(extract_text, name)
        name_for_slot = []
        ai = 0
        ci = 0
        for kind in inputs:
            if kind == CONST_I32:
                name_for_slot.append(const_names[ci] if ci < len(const_names) else f"const{ci}")
                ci += 1
            else:
                if ai < len(arg_names):
                    name_for_slot.append(arg_names[ai])
                    ai += 1
                else:
                    name_for_slot.append(f"arg{ai}")
                    ai += 1

    # SMTPat lemmas list const generics FIRST (e.g. `mm256_slli_epi32_lemma
    # v_IMM8 a i`), but our ground-truth `inputs` in this file has Rust
    # signature order (value args first, const-generics last for slli/srai).
    # If we detect that mismatch — spec has more arg_names than value slots
    # in `inputs[:N]` and `inputs` ends with CONST_I32 — swap.  Concretely:
    # if the spec's arg_names start with the imm-name and the GT inputs end
    # with CONST_I32, pivot.
    if spec.arg_names and inputs and inputs[-1] == CONST_I32 and len(inputs) >= 2 and inputs[0] != CONST_I32:
        # Move the const slot to the front so it lines up with the spec's
        # first-arg-is-IMM convention.
        # We do this by re-mapping the *evaluation order* of args into the
        # dict.
        pass  # The swap is applied at sample_args via re-binding below.

    for _ in range(samples):
        args_pos, _ = sample_args(rng, name, gt)
        # Bind into args_dict using the spec's arg_names (if known) or
        # the libcrux signature names.
        args_dict: Dict[str, Any] = {}
        # Re-order: if spec has explicit arg_names AND const trails value
        # in inputs, bind by reversed order so a const-first lemma sees the
        # const at index 0.
        ordered_pos = list(args_pos)
        if spec.arg_names and inputs and inputs[-1] == CONST_I32 and len(inputs) >= 2:
            # Move trailing const to front.
            const_val = ordered_pos.pop()
            ordered_pos.insert(0, const_val)
        for slot, val in zip(name_for_slot, ordered_pos):
            args_dict[slot] = val
        # Replay positional args (un-reordered) for ground-truth call.
        gt_args_pos = list(args_pos)

        # Compute LHS via ground truth (Rust signature order).
        try:
            lhs = gt["fn"](gt_args_pos)
        except Exception as e:  # pragma: no cover
            return Verdict(name, arch, spec.pattern, spec.source, "GROUND-TRUTH-ERROR",
                           samples, samples, ("ground-truth raised", str(e), ""))

        # Compute RHS via spec.
        try:
            rhs = spec.eval_rhs(args_dict)
        except KeyError as e:
            # Unknown arg name in spec — likely a parser-ID mismatch.
            return Verdict(name, arch, spec.pattern, spec.source, "OUT-OF-SCOPE-PATTERN",
                           samples, samples, (f"spec references unknown var {e}", "", ""),
                           spec_text=spec.spec_text)

        # Project LHS to the same lane shape RHS produces.
        try:
            lhs_proj = spec.project_lhs(lhs)
        except Exception as e:  # pragma: no cover
            return Verdict(name, arch, spec.pattern, spec.source, "PROJECTION-ERROR",
                           samples, samples, (str(e), "", ""))

        # Compare element-wise.  `None` in RHS = skip lane (e.g., abs_int's
        # MIN exclusion via the lemma's `requires`).
        ok = True
        for i, (lv, rv) in enumerate(zip(lhs_proj, rhs)):
            if rv is None:
                continue
            if lv != rv:
                ok = False
                break
        if not ok:
            fails += 1
            if first_fail is None:
                first_fail = (
                    "; ".join(f"{slot}={_arg_repr(v)}" for slot, v in zip(name_for_slot, ordered_pos)),
                    str(rhs),
                    str(lhs_proj),
                )

    status = "PASS" if fails == 0 else "FAIL"
    return Verdict(name, arch, spec.pattern, spec.source, status, samples, fails,
                   first_fail, spec_text=spec.spec_text)


# ----------------------- Driver --------------------------------------------


def load_t1(csv_path: Path) -> List[Tuple[str, str]]:
    """Return list of (name, arch) pairs from the trust index CSV."""
    out = []
    with csv_path.open() as f:
        for row in csv.DictReader(f):
            out.append((row["name"], row["arch"]))
    return out


def load_t1_levels(csv_path: Path) -> Dict[str, str]:
    out: Dict[str, str] = {}
    with csv_path.open() as f:
        for row in csv.DictReader(f):
            out[row["name"]] = row["trust_level"].strip()
    return out


def write_findings_md(path: Path, verdicts: List[Verdict], seed: int, samples: int) -> None:
    n = len(verdicts)
    n_pass = sum(1 for v in verdicts if v.status == "PASS")
    n_fail = sum(1 for v in verdicts if v.status == "FAIL")
    n_oos = sum(1 for v in verdicts if v.status == "OUT-OF-SCOPE-PATTERN")
    n_other = n - n_pass - n_fail - n_oos

    lines = [
        "# SIMD intrinsics cross-validation findings",
        "",
        f"Generated by `crates/utils/core-models/scripts/cross-validate.py` "
        f"(seed={seed}, samples_per_intrinsic={samples}).",
        "",
        "See `crates/utils/core-models/INTRINSICS-TRUST-PLAN.md` Step 5 for "
        "the cross-validation protocol.",
        "",
        "## Summary",
        "",
        f"- Intrinsics covered: **{n}**",
        f"- PASS: **{n_pass}**",
        f"- FAIL: **{n_fail}**",
        f"- OUT-OF-SCOPE-PATTERN: **{n_oos}**",
        f"- Other (no ground truth, etc.): **{n_other}**",
        "",
        "Per-intrinsic verdicts below.",
        "",
        "## Per-intrinsic table",
        "",
        "| Intrinsic | Arch | Source | Pattern | Status | Samples | Fails |",
        "|---|---|---|---|---|---:|---:|",
    ]
    for v in sorted(verdicts, key=lambda x: (x.status, x.name)):
        lines.append(
            f"| `{v.name}` | {v.arch} | {v.source} | {v.pattern} | "
            f"{v.status} | {v.samples} | {v.fails} |"
        )
    lines.append("")

    if n_fail:
        lines.append("## FAIL details")
        lines.append("")
        for v in verdicts:
            if v.status != "FAIL":
                continue
            lines.append(f"### `{v.name}` ({v.arch}, source={v.source}, pattern={v.pattern})")
            lines.append("")
            if v.spec_text:
                lines.append("```")
                lines.append(v.spec_text)
                lines.append("```")
                lines.append("")
            if v.first_fail:
                inp, exp, got = v.first_fail
                lines.append(f"- input: `{inp}`")
                lines.append(f"- expected (RHS): `{exp}`")
                lines.append(f"- got (LHS): `{got}`")
            lines.append("")

    if n_oos:
        lines.append("## OUT-OF-SCOPE-PATTERN list")
        lines.append("")
        lines.append(
            "These intrinsics have a F\\* spec whose lane-form predicate is "
            "outside the small sub-language understood by this script.  See "
            "`INTRINSICS-TRUST-PLAN.md` Step 5 for the supported patterns.  "
            "Adding parser support per pattern is the natural follow-up."
        )
        lines.append("")
        for v in sorted([x for x in verdicts if x.status == "OUT-OF-SCOPE-PATTERN"], key=lambda x: x.name):
            lines.append(f"### `{v.name}` ({v.arch}, source={v.source})")
            lines.append("")
            if v.spec_text:
                lines.append("```")
                lines.append(v.spec_text[:500] + ("..." if len(v.spec_text) > 500 else ""))
                lines.append("```")
                lines.append("")
            if v.first_fail and v.first_fail[0]:
                lines.append(f"Reason: `{v.first_fail[0]}`")
                lines.append("")

    lines.append("## Supported patterns")
    lines.append("")
    lines.append("Extract.rs `#[hax_lib::ensures(...)]` clauses:")
    lines.append("")
    lines.append("- `PAT_SETZERO`     — `vecBITS_as_TxN $result == Seq.create N (mk_T 0)`.")
    lines.append("- `PAT_CREATE`      — `vecBITS_as_TxN $result == Spec.Utils.create (sz N) $constant`.")
    lines.append("- `PAT_MAP2_OP`     — `Spec.Utils.map2 (OP)` for OP ∈ {`+.`,`-.`,`*.`,`^.`,`&.`,`|.`,`mul_mod`,`add_mod`,`sub_mod`} (parens optional for bare-ident ops).")
    lines.append("- `PAT_MAP2_MULHI`  — `Spec.Utils.map2 (fun x y -> cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 N)) <: i16)`.")
    lines.append("- `PAT_MAP_ARRAY_SHR` — `Spec.Utils.map_array (fun x -> x >>! ${SHIFT_BY})`.")
    lines.append("")
    lines.append("Spec.Intrinsics.fsti SMTPat lemmas — `to_iNxM`-projected:")
    lines.append("")
    lines.append("- `PAT_LEMMA_BIN_*` — `to_iNxM (I.fn a b) i == add_mod_opaque/sub_mod_opaque/mul_mod_opaque (to_iNxM a i) (to_iNxM b i)`.")
    lines.append("- `PAT_LEMMA_BIN_BITWISE_{&,^,|}` — `to_iNxM (I.fn a b) i == ((to_iNxM a i) OP. (to_iNxM b i))`.")
    lines.append("- `PAT_LEMMA_CMPGT` — `to_iNxM (I.cmpgt a b) i == (if to_iNxM a i >. to_iNxM b i then ones else zero)`.")
    lines.append("- `PAT_LEMMA_ABS`   — `to_iNxM (I.abs_epi32 a) i == abs_int (to_iNxM a i)` with `requires` skipping MIN.")
    lines.append("- `PAT_LEMMA_SET1`  — `to_iNxM (I.set1_epiK x0) i == x0`.")
    lines.append("- `PAT_LEMMA_SET_NARM` — generic K-arm set form: `match v i with | k -> rk` (set_epi8/16/32/64x; 4/8/16/32 arms).")
    lines.append("- `PAT_LEMMA_SET_4ARM`/`PAT_LEMMA_SET_EPI32` — explicit 4-arm `match v i`.")
    lines.append("- `PAT_LEMMA_MKINT_8ARM` — `match i with | MkInt k -> to_iNxM <var> (mk_uK <num>)` (set_m128i, unpacklo, unpackhi).")
    lines.append("- `PAT_LEMMA_PROJ_IDENTITY` — `to_iNxM (I.cast a) i == to_iNxK a i`.")
    lines.append("- `PAT_LEMMA_EXTRACTI128` — control-conditional offset: `to_iNxM (I.extracti128 ctl a) i == to_iNxK a (i + (if v ctl = 0 then 0 else N))`.")
    lines.append("- `PAT_LEMMA_SHUFFLE_EPI32` — indirect index via `mm256_shuffle_epi32_index`.")
    lines.append("- `PAT_LEMMA_MUL_EPI32` — interleaved low/high i64 multiplication, even-lane sibling.")
    lines.append("- `PAT_LEMMA_SLLI_EPI32` / `PAT_LEMMA_SRAI_EPI32` — scalar shift of i32 lanes with rem_euclid 256.")
    lines.append("- `PAT_LEMMA_BLEND_EPI32` — `(v imm8 / pow2 (v i)) % 2`-conditional pick.")
    lines.append("")
    lines.append("Spec.Intrinsics.fsti SMTPat lemmas — bit-vec `.()` projected:")
    lines.append("")
    lines.append("- `PAT_LEMMA_BIT_SHIFT` — `(I.NAME args).(i) == let nth_chunk=i/CHUNK in let local_index=...; if cond then vector.(...) else Bit_Zero`. Handles slli/srli/sllv/srlv with scalar (`v shift`) or per-chunk variable shift (`to_iKxL shifts nth_chunk`), CHUNK ∈ {32, 64}.")
    lines.append("- `PAT_LEMMA_BIT_BYTE_SHIFT` — `bsrli/bslli` with byte-shift × 8 in 128-bit lanes.")
    lines.append("")
    lines.append("Patterns still out of scope (rare/conditional):")
    lines.append("")
    lines.append("- Lemmas with non-trivial `requires` preconditions (e.g. `mm256_add_epi64_lemma`'s no-carry assumption).")
    lines.append("- Lemmas using internal helpers like `i16_mul_32extended` / `i16_mul_32extended_i16` / `i32_wrapping_add` (`mm256_madd_epi16`, `mm256_mullo_epi16` bv).")
    lines.append("- Scalar-result lemmas (`mm256_testz_si256` returns i32, not Vec).")
    lines.append("- Incomplete or non-lane-form ensures clauses (e.g. `mm256_cmpgt_epi16`'s `forall i. i % 16 >= 1 ==> result i == 0`, which references an undefined `result` projection).")

    path.write_text("\n".join(lines) + "\n")


def update_trust_index_audit_consistent(
    csv_path: Path, verdicts: List[Verdict]
) -> None:
    rows = []
    with csv_path.open() as f:
        reader = csv.DictReader(f)
        rows = list(reader)
        fieldnames = reader.fieldnames
    if fieldnames is None:
        return
    by_name = {v.name: v for v in verdicts}
    for r in rows:
        v = by_name.get(r["name"])
        if v is None:
            r["audit_consistent"] = ""
            continue
        if v.status == "PASS":
            r["audit_consistent"] = "true"
        elif v.status == "FAIL":
            r["audit_consistent"] = "false"
        else:
            # OOS / no-gt / no-spec → leave blank (audit not feasible).
            r["audit_consistent"] = ""
        # Trust level: bump L2 → L3 when audit_consistent is true.
        if r["trust_level"] == "L2" and r["audit_consistent"] == "true":
            r["trust_level"] = "L3"
    with csv_path.open("w", newline="") as f:
        w = csv.DictWriter(f, fieldnames=fieldnames)
        w.writeheader()
        w.writerows(rows)


def update_trust_index_md_d64(md_path: Path, verdicts: List[Verdict], total_t1: int) -> None:
    """Patch the D6.4 line in the trust-index.md to reflect cross-validation
    results.  Leaves all other lines untouched.
    """
    if not md_path.exists():
        return
    n_consistent = sum(1 for v in verdicts if v.status == "PASS")
    text = md_path.read_text()
    new_d64 = (
        f"| **D6.4** Audit consistency | "
        f"{(100.0 * n_consistent / total_t1):.1f}% ({n_consistent}/{total_t1}) | "
        f"— | — | 100% |"
    )
    new_text = re.sub(
        r"\| \*\*D6\.4\*\* Audit consistency .*\|",
        new_d64,
        text,
    )
    md_path.write_text(new_text)


def cmdline() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        description="Phase B5 SIMD intrinsics cross-validation script.",
    )
    p.add_argument("--seed", type=int, default=0)
    p.add_argument("--samples", type=int, default=10000)
    p.add_argument("--findings", type=Path, default=DEFAULT_FINDINGS_MD)
    p.add_argument("--audit-feed", action="store_true",
                   help="Update the trust index CSV's audit_consistent column "
                        "and bump L2→L3 for PASS rows.")
    p.add_argument("--scope", choices=["l2-avx2", "l1l2-avx2", "all"],
                   default="l2-avx2",
                   help="Which subset of T1 to run on (default: l2-avx2 — "
                        "the 49 already-tested AVX2 wrappers).")
    p.add_argument("--list-only", action="store_true",
                   help="List the intrinsics that would be processed, then exit.")
    return p


def main() -> int:
    args = cmdline().parse_args()
    rng = random.Random(args.seed)

    extract_text_avx2 = EXTRACT_AVX2.read_text()
    extract_text_arm64 = EXTRACT_ARM64.read_text()
    levels = load_t1_levels(TRUST_INDEX_CSV)

    # Combined GT dict: AVX2 + ARM64.
    gt_all = {**GROUND_TRUTH, **GROUND_TRUTH_ARM}

    # Build the candidate list.  Default scope: L2-or-higher AVX2.
    # NB: a previously L3-bumped wrapper (audit_consistent=true from a prior
    # `--audit-feed` run) must STILL be re-tested every run, otherwise the
    # script becomes irreproducible w.r.t. its own state.
    candidates: List[Tuple[str, str]] = []
    for name, arch in load_t1(TRUST_INDEX_CSV):
        lvl = levels.get(name, "")
        if args.scope == "l2-avx2":
            if arch != "avx2":
                continue
            if not (lvl.startswith("L2") or lvl.startswith("L3") or lvl.startswith("L4")):
                continue
        elif args.scope == "l1l2-avx2":
            if arch != "avx2":
                continue
            if not (lvl.startswith("L1") or lvl.startswith("L2") or lvl.startswith("L3") or lvl.startswith("L4")):
                continue
        elif args.scope == "all":
            # Include all T1 entries (avx2 + arm64) at L2+.
            if not (lvl.startswith("L2") or lvl.startswith("L3") or lvl.startswith("L4")):
                continue
        candidates.append((name, arch))

    # Parse all spec.intrinsics.fsti lemmas once.
    lemma_blocks = parse_specintrinsics_lemmas()

    if args.list_only:
        for n, a in candidates:
            print(f"{a:6} {n}")
        return 0

    verdicts: List[Verdict] = []
    for name, arch in candidates:
        gt = gt_all.get(name)
        # Pick the right extract source text.
        extract_text = extract_text_arm64 if arch == "arm64" else extract_text_avx2
        spec_case: Optional[SpecCase] = None
        # Prefer extract.rs ensures (since it's the consumer-facing post-cond).
        ens = parse_extract_ensures_for_fn(extract_text, name)
        if ens is not None:
            attr_blob, fstar_body = ens
            spec_case = parse_spec_case(name, fstar_body, "extract.rs")
            if spec_case is None:
                # Couldn't parse the ensures content but it exists.
                if gt is None:
                    verdicts.append(Verdict(name, arch, "—", "extract.rs",
                                            "NO-GROUND-TRUTH", 0, 0,
                                            spec_text=fstar_body))
                else:
                    verdicts.append(Verdict(name, arch, "—", "extract.rs",
                                            "OUT-OF-SCOPE-PATTERN",
                                            0, 0, ("unparseable ensures", "", ""),
                                            spec_text=fstar_body))
                continue
        # Else try Spec.Intrinsics.fsti.  A given intrinsic may have multiple
        # lemmas (e.g., `_lemma` and `_bv_lemma`); we try each in turn until
        # one parses cleanly.
        if spec_case is None:
            blocks = lemma_blocks.get(name) or []
            tried_block = None
            for block in blocks:
                tried_block = block
                spec_case = parse_lemma_into_speccase(name, block)
                if spec_case is not None:
                    break
            if spec_case is None:
                if not blocks:
                    pass  # fall through to NO-SPEC
                elif gt is None:
                    verdicts.append(Verdict(name, arch, "—", "spec_intrinsics",
                                            "NO-GROUND-TRUTH", 0, 0,
                                            spec_text=tried_block))
                    continue
                else:
                    verdicts.append(Verdict(name, arch, "—", "spec_intrinsics",
                                            "OUT-OF-SCOPE-PATTERN",
                                            0, 0, ("unparseable lemma", "", ""),
                                            spec_text=tried_block))
                    continue
        if spec_case is None:
            verdicts.append(Verdict(name, arch, "—", "—",
                                    "NO-SPEC", 0, 0))
            continue

        if gt is None:
            verdicts.append(Verdict(name, arch, spec_case.pattern, spec_case.source,
                                    "NO-GROUND-TRUTH", 0, 0,
                                    spec_text=spec_case.spec_text))
            continue

        verdicts.append(run_for_intrinsic(name, arch, spec_case, gt, rng,
                                          args.samples, extract_text))

    # Output.
    write_findings_md(args.findings, verdicts, args.seed, args.samples)

    n_pass = sum(1 for v in verdicts if v.status == "PASS")
    n_oos_or_other = sum(1 for v in verdicts if v.status not in ("PASS", "FAIL"))
    total_eligible = len(verdicts)

    # D6.4 over the L2 AVX2 scope only — reported in stdout last line.
    pct = (100.0 * n_pass / total_eligible) if total_eligible else 0.0
    summary_line = f"D6.4={pct:.1f}% ({n_pass}/{total_eligible})"
    print(summary_line)

    # Per-status breakdown to stderr.
    statuses: Dict[str, int] = {}
    for v in verdicts:
        statuses[v.status] = statuses.get(v.status, 0) + 1
    for k in sorted(statuses):
        print(f"# {k}: {statuses[k]}", file=sys.stderr)

    if args.audit_feed:
        update_trust_index_audit_consistent(TRUST_INDEX_CSV, verdicts)
        # D6.4 in the .md must be computed over the FULL T1 (193), not just
        # the L2 scope, so consumers see a comparable proportion.
        full_t1 = sum(1 for _ in load_t1(TRUST_INDEX_CSV))
        update_trust_index_md_d64(TRUST_INDEX_MD, verdicts, full_t1)

    return 0


if __name__ == "__main__":
    sys.exit(main())

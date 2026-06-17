#!/usr/bin/env python3
"""SIMD intrinsics trust-base audit script.

Phase A1 of the SIMD intrinsics trust-base sprint. See
``crates/utils/core-models/INTRINSICS-TRUST-PLAN.md``.

Computes the trust ladder (L0..L4, see plan) and the D6 sub-percentages
(D6.1 Rust-model coverage, D6.2 test coverage, D6.3 F* spec coverage,
D6.4 audit consistency, D6.5 F* spec proven) over

  T1 = pub fn in crates/utils/intrinsics/src/{avx2,arm64}.rs

Inputs (all read-only):
  - crates/utils/intrinsics/src/{avx2,arm64}.rs           — T1 wrappers
  - crates/utils/intrinsics/src/{avx2,arm64}_extract.rs   — F* axiom site
  - crates/utils/core-models/src/core_arch/                — T2 models
  - libcrux-ml-dsa/proofs/fstar/spec/Spec.Intrinsics.fsti  — T3 SMTPats

Outputs:
  - crates/utils/core-models/proofs/intrinsics-trust-index.md
  - crates/utils/core-models/proofs/intrinsics-trust-index.csv

Stable interface: D6.* percentages on stdout last line in the form
``D6.1=NN.N% D6.2=NN.N% D6.3=NN.N% D6.4=NN.N% D6.5=NN.N% T1=NNN``.
"""

from __future__ import annotations

import argparse
import csv
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional, Set, Tuple


# ----------------------- Repo layout ----------------------------------------

# intrinsics-audit.py lives at crates/utils/core-models/scripts/.
# parents[0]=scripts, [1]=core-models, [2]=utils, [3]=crates, [4]=repo root.
REPO_ROOT = Path(__file__).resolve().parents[4]

INTRINSICS_AVX2 = REPO_ROOT / "crates/utils/intrinsics/src/avx2.rs"
INTRINSICS_ARM64 = REPO_ROOT / "crates/utils/intrinsics/src/arm64.rs"
EXTRACT_AVX2 = REPO_ROOT / "crates/utils/intrinsics/src/avx2_extract.rs"
EXTRACT_ARM64 = REPO_ROOT / "crates/utils/intrinsics/src/arm64_extract.rs"
CORE_MODELS_X86 = REPO_ROOT / "crates/utils/core-models/src/core_arch/x86.rs"
CORE_MODELS_X86_INTERP = (
    REPO_ROOT / "crates/utils/core-models/src/core_arch/x86/interpretations.rs"
)
CORE_MODELS_ARM_DIR = REPO_ROOT / "crates/utils/core-models/src/core_arch/arm"
SPEC_INTRINSICS_FSTI = (
    REPO_ROOT / "libcrux-ml-dsa/proofs/fstar/spec/Spec.Intrinsics.fsti"
)

DEFAULT_OUT_MD = (
    REPO_ROOT / "crates/utils/core-models/proofs/intrinsics-trust-index.md"
)
DEFAULT_OUT_CSV = (
    REPO_ROOT / "crates/utils/core-models/proofs/intrinsics-trust-index.csv"
)


# ----------------------- Regexes --------------------------------------------

PUB_FN_RE = re.compile(
    r"^\s*pub\s+(?:unsafe\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*[(<]"
)
# Top-level pub fn (no leading whitespace) — used for T1 in libcrux/intrinsics.
TOP_PUB_FN_RE = re.compile(
    r"^pub\s+(?:unsafe\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*[(<]"
)

# Identifier patterns for `_mm[256]?_X` (x86) and `vXxxx` (NEON).
X86_INTRINSIC_RE = re.compile(r"\b(_mm(?:256|512)?_[a-z][a-zA-Z0-9_]*)\b")
ARM_INTRINSIC_RE = re.compile(r"\b(v[a-z][a-zA-Z0-9_]*q?[a-z0-9_]*)\b")

UNIMPL_RE = re.compile(r"\bunimplemented!\s*\(")
# Match `mk!(name(...))` and `mk!([N]name(...))` and `mk!(name{<...>}(...))`.
MK_INVOC_RE = re.compile(r"\bmk!\s*\(\s*(?:\[[0-9]+\])?\s*([A-Za-z_][A-Za-z0-9_]*)")
MK_LIFT_RE = re.compile(
    r"\bmk_lift_lemma!\s*\(\s*(?:\[[0-9]+\])?\s*([A-Za-z_][A-Za-z0-9_]*)"
)
# Hand-written `#[test] fn _mm…` / `fn vXxx…` differential tests (used when
# mk! cannot express the assertion shape, e.g. when the upstream takes raw
# pointers — the load/store family on both x86 and aarch64).
HAND_TEST_RE = re.compile(
    r"#\[test\]\s*(?:#\[[^\]]+\]\s*)*fn\s+("
    r"_mm(?:256|512)?_[a-z][a-zA-Z0-9_]*"
    r"|v[a-z][a-zA-Z0-9_]*"
    r")\s*\("
)


# ----------------------- Data model -----------------------------------------


@dataclass
class T1Entry:
    """One libcrux wrapper in T1."""

    name: str
    arch: str  # 'avx2' or 'arm64'
    line: int
    underlying: List[str] = field(default_factory=list)
    has_extract_ensures: bool = False
    has_extract_opaque: bool = False
    has_specintrinsics_lemma: bool = False
    has_body_via_underlying: bool = False
    has_mk_test_via_underlying: bool = False
    audit_consistent: Optional[bool] = None
    fstar_proven: bool = False

    @property
    def has_spec(self) -> bool:
        return self.has_extract_ensures or self.has_specintrinsics_lemma

    @property
    def trust_level(self) -> str:
        if not self.has_body_via_underlying:
            return "L0" if self.has_spec else "L0-nospec"
        if not self.has_mk_test_via_underlying:
            return "L1"
        if self.audit_consistent is True:
            return "L4" if self.fstar_proven else "L3"
        return "L2"


@dataclass
class T2Entry:
    """One intrinsic modeled in core-models."""

    name: str
    arch: str
    has_body: bool
    has_mk_test: bool
    has_lift_lemma: bool


# ----------------------- Parsers --------------------------------------------


def read(path: Path) -> str:
    if not path.exists():
        return ""
    return path.read_text()


def parse_top_pub_fns(path: Path) -> List[Tuple[str, int]]:
    """Top-level (no indent) `pub fn` names — used for T1."""
    out = []
    for i, line in enumerate(read(path).splitlines(), 1):
        m = TOP_PUB_FN_RE.match(line)
        if m:
            out.append((m.group(1), i))
    return out


def parse_any_pub_fns(path: Path) -> List[Tuple[str, int]]:
    """All `pub fn` names regardless of indent — used for T2 in core-models."""
    out = []
    text = read(path)
    if not text:
        return out
    for i, line in enumerate(text.splitlines(), 1):
        m = PUB_FN_RE.match(line)
        if m:
            out.append((m.group(1), i))
    return out


def find_fn_body(text: str, name: str) -> Optional[str]:
    """Locate `pub fn <name>(...) { ... }` and return the body text."""
    pat = re.compile(
        r"pub\s+(?:unsafe\s+)?fn\s+" + re.escape(name) + r"\b[^{]*\{",
        re.DOTALL,
    )
    m = pat.search(text)
    if not m:
        return None
    pos = m.end() - 1  # opening '{'
    depth = 0
    body_start = None
    for j in range(pos, len(text)):
        c = text[j]
        if c == "{":
            if depth == 0:
                body_start = j + 1
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return text[body_start:j]
    return None


def find_fn_attrs(text: str, name: str) -> str:
    """Return the contiguous attribute block immediately preceding `pub fn <name>`.

    Handles multi-line #[...] blocks such as:
        #[hax_lib::ensures(|r| fstar!("long
                                       expression"))]
    by scanning backwards from the fn declaration and tracking bracket depth.
    """
    fn_pat = re.compile(
        r"(?m)^[^\S\n]*pub\s+(?:unsafe\s+)?fn\s+" + re.escape(name) + r"\b"
    )
    m = fn_pat.search(text)
    if not m:
        return ""

    lines = text[: m.start()].split("\n")
    result: list[str] = []
    # depth > 0 means we are inside an open multi-line #[...] block (scanning upward).
    # Going backwards: each ']' increments depth (we need to find its '[');
    # each '[' decrements depth (matched one level of opening).
    depth = 0

    for line in reversed(lines):
        stripped = line.strip()
        opens = stripped.count("[")
        closes = stripped.count("]")

        if depth > 0:
            result.append(line)
            depth = max(0, depth + closes - opens)
        elif closes > opens:
            # Bottom line of a multi-line attribute (more ']' than '[').
            result.append(line)
            depth = closes - opens
        elif stripped.startswith("#[") or stripped.startswith("#!["):
            # Complete single-line attribute (opens == closes after counting).
            result.append(line)
        elif not stripped or stripped.startswith("//"):
            # Blank line or comment — skip without breaking.
            continue
        else:
            break

    return "\n".join(reversed(result))


def strip_rust_comments(text: str) -> str:
    """Remove `//` line comments and `/* ... */` block comments."""
    text = re.sub(r"//[^\n]*", "", text)
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.DOTALL)
    return text


def underlying_calls_in_body(body: str, arch: str) -> List[str]:
    """Heuristically extract names of intrinsics called inside a wrapper."""
    if body is None:
        return []
    body = strip_rust_comments(body)
    if arch == "avx2":
        return sorted(set(X86_INTRINSIC_RE.findall(body)))
    elif arch == "arm64":
        # NEON wrappers call `vfoo_xxx(...)` directly. Filter out obvious
        # non-intrinsic identifiers (very short, or starts with `vec`).
        candidates = ARM_INTRINSIC_RE.findall(body)
        out: Set[str] = set()
        for c in candidates:
            if len(c) < 4:
                continue
            if c.startswith("vec"):
                continue
            if c in {"vector", "vectors", "value", "values"}:
                continue
            out.add(c)
        return sorted(out)
    return []


def _scan_for_intrinsic_defs(
    text: str,
    name_regex: str,
) -> Dict[str, bool]:
    """Return {name: has_real_body} for every `pub fn <name>` matching the
    regex in `text`. `has_real_body` = body is non-empty and contains no
    `unimplemented!()`."""
    out: Dict[str, bool] = {}
    pat = re.compile(
        r"pub\s+(?:unsafe\s+)?fn\s+(" + name_regex + r")\b[^{]*\{",
        re.DOTALL,
    )
    for m in pat.finditer(text):
        fn_name = m.group(1)
        body = find_fn_body(text, fn_name)
        has_body = body is not None and not UNIMPL_RE.search(body)
        if fn_name not in out or has_body:
            out[fn_name] = has_body
    return out


def parse_core_models_t2(arch: str) -> Dict[str, T2Entry]:
    """Parse core-models for the set of modeled intrinsics.

    For x86, an intrinsic is "modeled" if either:
      - the bit-vector layer (`core_arch/x86.rs`) has a real body, OR
      - the integer-vector layer (`interpretations.rs::int_vec`) has a real
        body.

    The bit-vector layer is typically `#[hax_lib::opaque]` with an
    `unimplemented!()` stub body — the actual computational content lives
    at the int-vec layer, with `mk_lift_lemma!` connecting them."""
    out: Dict[str, T2Entry] = {}

    if arch == "avx2":
        bv_text = read(CORE_MODELS_X86)
        iv_text = read(CORE_MODELS_X86_INTERP)
        if not bv_text and not iv_text:
            return out

        bv_defs = _scan_for_intrinsic_defs(
            bv_text, r"_mm(?:256|512)?_[a-z][a-zA-Z0-9_]*"
        )
        iv_defs = _scan_for_intrinsic_defs(
            iv_text, r"_mm(?:256|512)?_[a-z][a-zA-Z0-9_]*"
        )
        all_names = set(bv_defs) | set(iv_defs)
        for name in all_names:
            has_body = bv_defs.get(name, False) or iv_defs.get(name, False)
            out[name] = T2Entry(
                name=name,
                arch="avx2",
                has_body=has_body,
                has_mk_test=False,
                has_lift_lemma=False,
            )

        for name in MK_INVOC_RE.findall(iv_text):
            if name in out:
                out[name].has_mk_test = True
        for name in HAND_TEST_RE.findall(iv_text):
            if name in out:
                out[name].has_mk_test = True
        for name in MK_LIFT_RE.findall(iv_text):
            if name in out:
                out[name].has_lift_lemma = True

    elif arch == "arm64":
        if not CORE_MODELS_ARM_DIR.exists():
            return out
        files: List[Tuple[Path, str]] = [
            (f, read(f)) for f in sorted(CORE_MODELS_ARM_DIR.rglob("*.rs"))
        ]
        # First pass: collect all defs across all files so `out` is fully
        # populated before tests/lemmas (which may live in different files
        # from the definitions) try to look intrinsic names up.
        for _, text in files:
            defs = _scan_for_intrinsic_defs(text, r"v[a-z][a-zA-Z0-9_]*")
            for name, has_body in defs.items():
                if name not in out:
                    out[name] = T2Entry(
                        name=name,
                        arch="arm64",
                        has_body=has_body,
                        has_mk_test=False,
                        has_lift_lemma=False,
                    )
                elif has_body and not out[name].has_body:
                    out[name].has_body = True
        # Second pass: scan tests/lemmas, now that every defined name is in
        # `out`.
        for _, text in files:
            for name in MK_INVOC_RE.findall(text):
                if name in out:
                    out[name].has_mk_test = True
            for name in HAND_TEST_RE.findall(text):
                if name in out:
                    out[name].has_mk_test = True
            for name in MK_LIFT_RE.findall(text):
                if name in out:
                    out[name].has_lift_lemma = True

    return out


def parse_specintrinsics_t3() -> Set[str]:
    """Names referenced in Spec.Intrinsics.fsti — both `I.<name>` and
    `<name>_lemma` patterns. Returns the *base* intrinsic name with
    any trailing `_bv` (a lemma flavour, not a separate intrinsic) stripped."""
    text = read(SPEC_INTRINSICS_FSTI)
    raw: Set[str] = set()
    # `I.mm256_X` — direct intrinsic citation.
    for m in re.finditer(
        r"\bI\.(mm(?:256|512)?_[a-z][a-zA-Z0-9_]*)\b", text
    ):
        raw.add(m.group(1))
    # SMTPat lemmas named `mm256_X_lemma` or `mm_X_bv_lemma`. Use non-greedy
    # to keep suffix capture from absorbing into the inner group.
    for m in re.finditer(
        r"\b(mm(?:256|512)?_[a-z][a-zA-Z0-9_]*?)_(?:bv_)?lemma\b", text
    ):
        raw.add(m.group(1))
    out: Set[str] = set()
    for name in raw:
        if name.endswith("_bv"):
            name = name[:-3]
        out.add(name)
    return out


# ----------------------- Builders -------------------------------------------


def build_t1() -> List[T1Entry]:
    """Construct T1 = libcrux wrappers in {avx2,arm64}.rs."""
    t1: List[T1Entry] = []
    for path, arch in [
        (INTRINSICS_AVX2, "avx2"),
        (INTRINSICS_ARM64, "arm64"),
    ]:
        text = read(path)
        for name, line in parse_top_pub_fns(path):
            body = find_fn_body(text, name)
            underlying = underlying_calls_in_body(body or "", arch)
            t1.append(T1Entry(name=name, arch=arch, line=line, underlying=underlying))
    return t1


def annotate_t1_with_extract(t1: List[T1Entry]) -> None:
    """Set has_extract_ensures / has_extract_opaque from {avx2,arm64}_extract.rs."""
    for path, arch in [
        (EXTRACT_AVX2, "avx2"),
        (EXTRACT_ARM64, "arm64"),
    ]:
        text = read(path)
        if not text:
            continue
        for entry in t1:
            if entry.arch != arch:
                continue
            attrs = find_fn_attrs(text, entry.name)
            if not attrs:
                # Fn might not exist at all in extract.rs — leave defaults.
                continue
            if (
                "hax_lib::ensures" in attrs
                or "hax_lib::fstar::replace" in attrs
                or "hax_lib::fstar::before" in attrs
            ):
                entry.has_extract_ensures = True
            if "hax_lib::opaque" in attrs:
                entry.has_extract_opaque = True


def annotate_t1_with_specintrinsics(t1: List[T1Entry], t3: Set[str]) -> None:
    """Set has_specintrinsics_lemma based on T3 set."""
    for entry in t1:
        if entry.arch != "avx2":
            continue
        # Wrapper name maps to Spec.Intrinsics.fsti's `I.<name>` directly when
        # the wrapper IS the underlying (rare for AVX2; common for NEON).
        # For AVX2 wrappers, also check whether any underlying _mm name has a
        # lemma whose stripped-prefix form matches.
        if entry.name in t3:
            entry.has_specintrinsics_lemma = True
            continue
        for u in entry.underlying:
            stripped = u.lstrip("_")
            if stripped in t3:
                entry.has_specintrinsics_lemma = True
                break


def annotate_t1_with_t2(t1: List[T1Entry], t2_map: Dict[str, T2Entry]) -> None:
    """Set has_body_via_underlying / has_mk_test_via_underlying."""
    for entry in t1:
        # All underlying calls must be modeled for has_body to be true.
        if not entry.underlying:
            # Wrapper is purely declarative (e.g., type alias usage). Treat
            # as no underlying to model — falls into has_body=False.
            continue
        # AVX2: underlying names are `_mm...`, look up directly in t2_map.
        # NEON: underlying names are `vXxxx`, also direct lookup.
        all_have_body = True
        any_has_mk = False
        for u in entry.underlying:
            t2e = t2_map.get(u)
            if not t2e or not t2e.has_body:
                all_have_body = False
            if t2e and t2e.has_mk_test:
                any_has_mk = True
        entry.has_body_via_underlying = all_have_body
        entry.has_mk_test_via_underlying = any_has_mk and all_have_body


# ----------------------- Output ---------------------------------------------


def pct(n: int, d: int) -> str:
    return f"{(100.0 * n / d) if d else 0.0:.1f}%"


def preserve_audit_consistent_from_csv(t1: List[T1Entry], csv_path: Path) -> None:
    """Read `audit_consistent` and `fstar_proven` from an existing CSV and
    propagate the values into the matching T1Entry objects in-place.

    The audit script regenerates the trust index from source files only;
    `audit_consistent` is set by an out-of-band `cross-validate.py --audit-feed`
    run.  Without this restore step, every regen would overwrite the column.
    """
    if not csv_path.exists():
        return
    by_name = {e.name: e for e in t1}
    with csv_path.open() as f:
        for row in csv.DictReader(f):
            entry = by_name.get(row.get("name", ""))
            if entry is None:
                continue
            v = (row.get("audit_consistent") or "").strip().lower()
            if v == "true":
                entry.audit_consistent = True
            elif v == "false":
                entry.audit_consistent = False
            # blank/None: leave the default (None / not-yet-audited)
            # `fstar_proven` is set externally too; restore it if non-default.
            fp = (row.get("fstar_proven") or "").strip().lower()
            if fp == "true":
                entry.fstar_proven = True


def write_csv(t1: List[T1Entry], path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="") as f:
        w = csv.writer(f)
        w.writerow(
            [
                "name",
                "arch",
                "in_T1",
                "underlying",
                "has_body",
                "has_mk_test",
                "has_extract_ensures",
                "has_extract_opaque",
                "has_specintrinsics_lemma",
                "audit_consistent",
                "fstar_proven",
                "trust_level",
            ]
        )
        for e in t1:
            w.writerow(
                [
                    e.name,
                    e.arch,
                    "true",
                    ";".join(e.underlying),
                    str(e.has_body_via_underlying).lower(),
                    str(e.has_mk_test_via_underlying).lower(),
                    str(e.has_extract_ensures).lower(),
                    str(e.has_extract_opaque).lower(),
                    str(e.has_specintrinsics_lemma).lower(),
                    "" if e.audit_consistent is None else str(e.audit_consistent).lower(),
                    str(e.fstar_proven).lower(),
                    e.trust_level,
                ]
            )


def write_md(
    t1: List[T1Entry],
    t2_map: Dict[str, T2Entry],
    t3: Set[str],
    path: Path,
) -> None:
    n = len(t1)
    n_avx2 = sum(1 for e in t1 if e.arch == "avx2")
    n_arm = sum(1 for e in t1 if e.arch == "arm64")

    n_body = sum(1 for e in t1 if e.has_body_via_underlying)
    n_mk = sum(1 for e in t1 if e.has_mk_test_via_underlying)
    n_spec = sum(1 for e in t1 if e.has_spec)
    n_audit = sum(1 for e in t1 if e.audit_consistent is True)
    n_proven = sum(1 for e in t1 if e.fstar_proven)

    n_body_avx2 = sum(1 for e in t1 if e.arch == "avx2" and e.has_body_via_underlying)
    n_mk_avx2 = sum(1 for e in t1 if e.arch == "avx2" and e.has_mk_test_via_underlying)
    n_spec_avx2 = sum(1 for e in t1 if e.arch == "avx2" and e.has_spec)
    n_body_arm = sum(1 for e in t1 if e.arch == "arm64" and e.has_body_via_underlying)
    n_mk_arm = sum(1 for e in t1 if e.arch == "arm64" and e.has_mk_test_via_underlying)
    n_spec_arm = sum(1 for e in t1 if e.arch == "arm64" and e.has_spec)

    # Difference sets.
    t1_names = {e.name for e in t1}
    t1_underlying_avx2: Set[str] = set()
    t1_underlying_arm: Set[str] = set()
    for e in t1:
        for u in e.underlying:
            (t1_underlying_avx2 if e.arch == "avx2" else t1_underlying_arm).add(u)

    # T2 \ T1: core-models intrinsics not used by libcrux (pruning candidates).
    # Use the underlying-set as the natural T1 surface in core-models terms.
    t2_avx2 = {n for n, e in t2_map.items() if e.arch == "avx2"}
    t2_arm = {n for n, e in t2_map.items() if e.arch == "arm64"}
    t2_minus_t1_avx2 = sorted(t2_avx2 - t1_underlying_avx2)
    t2_minus_t1_arm = sorted(t2_arm - t1_underlying_arm)
    # T1 \ T2: libcrux underlying not modeled (gaps to fill).
    t1_minus_t2_avx2 = sorted(t1_underlying_avx2 - t2_avx2)
    t1_minus_t2_arm = sorted(t1_underlying_arm - t2_arm)
    # T3 \ T1: orphan SMTPat lemmas (not referenced by any libcrux wrapper).
    t1_underlying_no_prefix = {u.lstrip("_") for u in t1_underlying_avx2}
    t3_minus_t1 = sorted(t3 - t1_underlying_no_prefix - t1_names)

    out = []
    out.append("# SIMD intrinsics trust index")
    out.append("")
    out.append(
        "Generated by `crates/utils/core-models/scripts/intrinsics-audit.py`."
    )
    out.append(
        "See `crates/utils/core-models/INTRINSICS-TRUST-PLAN.md` for the "
        "trust-ladder definition (L0..L4) and the D6 sub-percentages."
    )
    out.append("")
    out.append("## D6 sub-percentages over T1")
    out.append("")
    out.append(f"`T1 = {n}` (`T1_avx2 = {n_avx2}`, `T1_arm64 = {n_arm}`)")
    out.append("")
    out.append("| Sub-dim | Today (this run) | Avx2 | Arm64 | Target after sprint |")
    out.append("|---|---:|---:|---:|---:|")
    out.append(
        f"| **D6.1** Rust-model coverage | {pct(n_body, n)} ({n_body}/{n}) | "
        f"{pct(n_body_avx2, n_avx2)} ({n_body_avx2}/{n_avx2}) | "
        f"{pct(n_body_arm, n_arm)} ({n_body_arm}/{n_arm}) | 100% |"
    )
    out.append(
        f"| **D6.2** Test coverage | {pct(n_mk, n)} ({n_mk}/{n}) | "
        f"{pct(n_mk_avx2, n_avx2)} ({n_mk_avx2}/{n_avx2}) | "
        f"{pct(n_mk_arm, n_arm)} ({n_mk_arm}/{n_arm}) | 100% |"
    )
    out.append(
        f"| **D6.3** F\\* spec coverage | {pct(n_spec, n)} ({n_spec}/{n}) | "
        f"{pct(n_spec_avx2, n_avx2)} ({n_spec_avx2}/{n_avx2}) | "
        f"{pct(n_spec_arm, n_arm)} ({n_spec_arm}/{n_arm}) | 100% |"
    )
    out.append(
        f"| **D6.4** Audit consistency | {pct(n_audit, n)} ({n_audit}/{n}) | "
        f"— | — | 100% |"
    )
    out.append(
        f"| **D6.5** F\\* spec proven | {pct(n_proven, n)} ({n_proven}/{n}) | "
        f"— | — | 0% (deferred) |"
    )
    out.append("")
    out.append("## Trust-level distribution over T1")
    out.append("")
    levels: Dict[str, int] = {}
    for e in t1:
        levels[e.trust_level] = levels.get(e.trust_level, 0) + 1
    for lvl in ["L0-nospec", "L0", "L1", "L2", "L3", "L4"]:
        if lvl in levels:
            out.append(f"- `{lvl}`: {levels[lvl]} ({pct(levels[lvl], n)})")
    out.append("")

    out.append("## Difference sets")
    out.append("")
    out.append(
        f"- `|T1 \\ T2|_avx2 = {len(t1_minus_t2_avx2)}` — libcrux AVX2 underlying "
        f"intrinsics not modeled in core-models (gaps to fill in Phase B-AVX2):"
    )
    if t1_minus_t2_avx2:
        for name in t1_minus_t2_avx2[:80]:
            out.append(f"  - `{name}`")
        if len(t1_minus_t2_avx2) > 80:
            out.append(f"  - ... ({len(t1_minus_t2_avx2) - 80} more)")
    else:
        out.append("  - (none)")
    out.append("")
    out.append(
        f"- `|T1 \\ T2|_arm64 = {len(t1_minus_t2_arm)}` — libcrux NEON underlying "
        f"intrinsics not modeled (gaps to fill in Phase B-NEON):"
    )
    if t1_minus_t2_arm:
        for name in t1_minus_t2_arm[:80]:
            out.append(f"  - `{name}`")
        if len(t1_minus_t2_arm) > 80:
            out.append(f"  - ... ({len(t1_minus_t2_arm) - 80} more)")
    else:
        out.append("  - (none)")
    out.append("")
    out.append(
        f"- `|T2 \\ T1|_avx2 = {len(t2_minus_t1_avx2)}` — core-models AVX2 "
        f"intrinsics not used by libcrux (pruning candidates in Phase C1):"
    )
    if t2_minus_t1_avx2:
        for name in t2_minus_t1_avx2[:80]:
            out.append(f"  - `{name}`")
        if len(t2_minus_t1_avx2) > 80:
            out.append(f"  - ... ({len(t2_minus_t1_avx2) - 80} more)")
    else:
        out.append("  - (none)")
    out.append("")
    out.append(
        f"- `|T2 \\ T1|_arm64 = {len(t2_minus_t1_arm)}` — core-models NEON "
        f"intrinsics not used by libcrux (none expected pre-sprint):"
    )
    if t2_minus_t1_arm:
        for name in t2_minus_t1_arm[:80]:
            out.append(f"  - `{name}`")
        if len(t2_minus_t1_arm) > 80:
            out.append(f"  - ... ({len(t2_minus_t1_arm) - 80} more)")
    else:
        out.append("  - (none)")
    out.append("")
    out.append(
        f"- `|T3 \\ T1| = {len(t3_minus_t1)}` — Spec.Intrinsics.fsti SMTPat "
        f"lemmas not referenced by any libcrux underlying:"
    )
    if t3_minus_t1:
        for name in t3_minus_t1[:80]:
            out.append(f"  - `{name}`")
        if len(t3_minus_t1) > 80:
            out.append(f"  - ... ({len(t3_minus_t1) - 80} more)")
    else:
        out.append("  - (none)")
    out.append("")

    out.append("## Per-intrinsic CSV")
    out.append("")
    out.append(
        f"Companion CSV: `{path.with_suffix('.csv').name}` — one row per T1 wrapper."
    )
    out.append("")
    out.append("Columns:")
    out.append(
        "`name`, `arch`, `in_T1`, `underlying`, `has_body`, `has_mk_test`, "
        "`has_extract_ensures`, `has_extract_opaque`, `has_specintrinsics_lemma`, "
        "`audit_consistent`, `fstar_proven`, `trust_level`."
    )
    out.append("")

    out.append("## Notes / caveats")
    out.append("")
    out.append(
        "1. **`has_body` semantics**: a wrapper is `has_body=true` iff *every* "
        "intrinsic it calls has a non-`unimplemented!` body in core-models. A "
        "single missing leaf flips the wrapper to `false`."
    )
    out.append(
        "2. **`has_mk_test` semantics**: at least one underlying intrinsic has a "
        "`mk!(...)` invocation in `core_arch/x86/interpretations.rs` or under "
        "`core_arch/arm/`. `has_mk_test` requires `has_body` to also be true "
        "(testing an undefined model is meaningless)."
    )
    out.append(
        "3. **Underlying extraction is regex-based**: it scans the wrapper body "
        "for tokens matching `_mm[256|512]?_X` (AVX2) or `vXxxx` (NEON). False "
        "positives possible for variable names that resemble intrinsic names; "
        "false negatives possible if a wrapper calls a helper that calls an "
        "intrinsic. Phase B5 cross-validation will surface mismatches."
    )
    out.append(
        "4. **`audit_consistent` is `null` until Phase B5** (cross-validation "
        "script). D6.4 reads as 0% pre-Phase-B5."
    )
    out.append(
        "5. **`fstar_proven` is `false` for all entries pre-unification-plan**. "
        "D6.5 stays at 0% by sprint design."
    )
    out.append(
        "6. **`Spec.Intrinsics.fsti` matching is name-based**: an `I.foo` "
        "reference or a `foo_lemma` is recorded as T3 ∋ foo. The audit "
        "doesn't reason about whether the lemma's *content* matches the "
        "tested body — that's Phase B5's job."
    )

    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(out) + "\n")


def emit_summary_line(t1: List[T1Entry]) -> str:
    n = len(t1)
    n_body = sum(1 for e in t1 if e.has_body_via_underlying)
    n_mk = sum(1 for e in t1 if e.has_mk_test_via_underlying)
    n_spec = sum(1 for e in t1 if e.has_spec)
    n_audit = sum(1 for e in t1 if e.audit_consistent is True)
    n_proven = sum(1 for e in t1 if e.fstar_proven)
    return (
        f"D6.1={pct(n_body, n)} D6.2={pct(n_mk, n)} "
        f"D6.3={pct(n_spec, n)} D6.4={pct(n_audit, n)} "
        f"D6.5={pct(n_proven, n)} T1={n}"
    )


# ----------------------- Main -----------------------------------------------


def main() -> int:
    p = argparse.ArgumentParser(
        description="Audit SIMD intrinsics trust base (D6.* over T1).",
    )
    p.add_argument(
        "--output",
        "--md",
        dest="md",
        type=Path,
        default=DEFAULT_OUT_MD,
        help="Output markdown trust-index path.",
    )
    p.add_argument(
        "--csv",
        type=Path,
        default=DEFAULT_OUT_CSV,
        help="Output CSV path.",
    )
    p.add_argument(
        "--print-summary",
        action="store_true",
        help="Also print a one-line D6.* summary to stdout (always emitted as "
        "the last line regardless of this flag).",
    )
    args = p.parse_args()

    t1 = build_t1()
    t2_map_avx2 = parse_core_models_t2("avx2")
    t2_map_arm = parse_core_models_t2("arm64")
    t2_map = {**t2_map_avx2, **t2_map_arm}
    t3 = parse_specintrinsics_t3()

    annotate_t1_with_extract(t1)
    annotate_t1_with_specintrinsics(t1, t3)
    annotate_t1_with_t2(t1, t2_map)
    # Preserve any existing audit_consistent column in the CSV (populated by
    # `cross-validate.py --audit-feed`). Without this restore, regenerating
    # trust-index.md would zero D6.4.
    preserve_audit_consistent_from_csv(t1, args.csv)

    write_csv(t1, args.csv)
    write_md(t1, t2_map, t3, args.md)

    summary = emit_summary_line(t1)
    if args.print_summary:
        print(summary)
    else:
        print(summary)
    return 0


if __name__ == "__main__":
    sys.exit(main())

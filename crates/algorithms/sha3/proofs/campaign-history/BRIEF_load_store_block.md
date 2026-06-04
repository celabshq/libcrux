# Brief: discharge `load_block`/`store_block` body admits

**Goal**: replace each `hax_lib::fstar!("admit()")` in the four SIMD
`load_block`/`store_block` functions with a real proof. Net result: 4
local-trust admits removed.

These are independent of the squeeze proofs (see `BRIEF_squeeze_steps.md`)
and can be done in parallel.

## Inventory (4 admits)

| File | Line | Function | RATE width | Lanes |
|---|---|---|---|---|
| `crates/algorithms/sha3/src/simd/arm64.rs` | 123 | `load_block` | RATE bytes | 2 |
| `crates/algorithms/sha3/src/simd/arm64.rs` | 207 | `store_block` | RATE bytes | 2 |
| `crates/algorithms/sha3/src/simd/avx2.rs` | 97 | `load_block` | RATE bytes | 4 |
| `crates/algorithms/sha3/src/simd/avx2.rs` | 254 | `store_block` | RATE bytes | 4 |

Each function is small (50-150 lines). The math is concrete bit-vector
reasoning over the SIMD intrinsics' bit-level definitions plus the
per-u64-lane SMTPats added in earlier sessions
(`lemma_mm256_*_u64x4` / arm64 NEON intrinsic ensures).

## Why these are good for human work

- **Concrete and self-contained.** Each function's correctness is
  expressed as a per-u64-lane equality between the post-state and the
  XOR / load / store of bytes from the input/output buffer. No
  whole-function induction, no loop-invariant Z3 cascade.
- **The Portable analogue is fully proved** at N=1 (no admit).
  `crates/algorithms/sha3/src/simd/portable.rs:60-119` — same
  bit-twiddling structure, just at lane width 1 instead of 2/4.
- **Bit-level reasoning is hard to grind iteratively** at the cost of an
  F* verify cycle (~30s/iteration on this module). Humans navigate the
  search faster than I do.

## Reference proofs

| What you're proving | Working reference |
|---|---|
| Per-u64-element ensures of `load_block` | `crates/algorithms/sha3/src/simd/portable.rs:53-120` (whole `load_block` function with two-loop `state_flat` + xor structure, fully proved) |
| Per-u64-element ensures of `store_block` | `crates/algorithms/sha3/src/simd/portable.rs:165-` (Portable `store_block`, fully proved) |
| Per-lane semantics of XOR/load/store intrinsics at u64 granularity | `crates/utils/intrinsics/src/avx2_extract.rs:447-460` (the `lemma_mm256_*_u64x4` SMTPats for AVX2); `crates/utils/intrinsics/src/arm64_extract.rs` (the `e_v*q_u64` per-lane ensures for arm64) |
| The bit_vec definitions of the underlying intrinsics | `fstar-helpers/fstar-bitvec/BitVec.Intrinsics.fsti:88-101` |

## Existing ensures shape (reference)

The arm64 `load_block` (line 70-) already has a per-lane forall ensures
in its loop_invariant block (look at lines 78-100 of `arm64.rs`). The
function ends with `hax_lib::fstar!("admit()");` immediately followed by
the `remaining > 0` partial-block tail. **The admit is what's blocking
the second-loop-body verification**, not the loop_invariant itself.

The pattern to mirror from Portable is:
1. Define a ghost `state_flat` (or use the existing `_vld1q_bytes_u64` /
   `_veorq_u64` helpers' per-lane ensures).
2. Loop invariant: at iteration `i`, the state's 0-th lane and 1-st
   lane (or 0..3 for avx2) match `old_state[j]` xor'd with the input
   bytes from `blocks[lane][offset..offset+8]`.
3. Each `_veorq_u64`, `_vtrn1q_u64`, `_vtrn2q_u64` invocation has a
   per-u64-lane SMTPat that gives Z3 the needed equality.
4. After the loop, the final partial block (if `RATE % 16 != 0`)
   updates one more lane.

## Suggested order

1. **arm64 `load_block`** first (line 70 / admit at line 123). Smallest
   bit-twiddling, well-documented intrinsics. Gets the pattern locked in.
2. **arm64 `store_block`** (line 193 / admit at line 207). Mirrors
   load_block but in the other direction.
3. **avx2 `load_block`** (line 90 / admit at line 97). Port from arm64
   using AVX2 intrinsic equivalents (`mm256_xor_si256`,
   `mm256_loadu_si256_u8` etc.). Uses the `lemma_mm256_*_u64x4`
   SMTPats already in `avx2_extract.rs`.
4. **avx2 `store_block`** (line 245 / admit at line 254).

I (Claude) can do step 3 and step 4 once you've cracked steps 1 and 2 —
the AVX2 versions are mechanical ports of the arm64 versions at lane
width 4 instead of 2.

## Acceptance criteria

For each function:
- [ ] `hax_lib::fstar!("admit()")` removed.
- [ ] Function verifies clean (after `bash hax.sh extract`) at the
  module's existing rlimit (probably 600-800 with `--split_queries always`).
- [ ] Existing ensures (per-lane forall) still passes.
- [ ] No new admits introduced.
- [ ] Full equivalence `make verify` GREEN.

## Tips

- Each iteration of `bash hax.sh extract && make` for just
  `Libcrux_sha3.Simd.{Arm64,Avx2}.fst.checked` is ~5-10s extraction
  + ~30-60s F* — fast enough to iterate.
- `--log_queries --query_stats` will show which sub-query is the
  bottleneck if Z3 stalls.
- The two-loop pattern in Portable (`state_flat` first, then xor) was
  chosen specifically because combining the loops hurt verification. The
  SIMD versions inline the load and xor — that's fine for the existing
  ensures shape, but if you hit Z3 cascade, splitting via a ghost
  `state_flat` array is a known fallback.
- Don't touch the `loop_invariant!` if you can avoid it — it's been
  tuned in earlier sessions.

## What Claude does after you finish arm64

- Mechanical port arm64 → avx2 for both `load_block` and `store_block`.
  Same proof body, swap `_veorq_u64` → `mm256_xor_si256`,
  `_vld1q_bytes_u64` → `mm256_loadu_si256_u8`, etc.
- Re-extract and verify.
- Update `SHA3_STATUS.md` and `HANDOFF.md` admit inventory.

## Status / log

- Created: 2026-04-25 by Claude session (sha3-proofs-focused branch)
- Branch / HEAD at brief creation: `9d780a03e`
- Ready to hand off to user.

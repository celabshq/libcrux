# Hint-deletion robustness experiment — 2026-05-08

Test: how many ML-DSA proofs are reliant on F\*'s `.hints` cache to
land within rlimit budget?  Conducted against HEAD `9b5b75b4b` plus
the tactical `Ml_dsa_generic::generate_key_pair` body-admit (this
session); the keygen-cone opacification scaffolding and the recent
sprint closures (Matrix body proofs, opacification of bound predicates)
are all in tree.

## Method

1. Backed up `.fstar-cache/hints/Libcrux_ml_dsa.*.hints` to `/tmp/hints-backup-2026-05-08/`.
2. Cleared `Libcrux_ml_dsa.*.fst.checked` to force re-verification.
3. Ran `make -k -j4 verify ENABLE_HINTS=''` which drops both `--use_hints`
   and `--record_hints` from the F\* invocation.  F\* never reads or
   writes hints.
4. For functions that failed: wrapped the body with
   `--admit_smt_queries true` in their existing `#push-options` line
   (extracted-`.fst`-level edit; reverted by next `hax.sh extract`),
   cleared the affected `.checked` files, re-ran.

## Results — Iter 1 (no admits)

- 84 modules attempted, **54 verified, 3 hard failures**, 30 transitively unverified (deps blocked).
- Cumulative wall: ~30 min on this machine.

**Hard-failed without hints (3 functions)**:

| # | Function | Module | Failure |
|---|---|---|---|
| 1 | `deserialize_when_gamma1_is_2_pow_17_` | `Simd.Portable.Encoding.Gamma1` | rlimit-saturated; max query 219 s, total 462 s |
| 2 | `compute_matrix_x_mask` | `Matrix` | rlimit-saturated; max query 57 s, total 161 s |
| 3 | `deserialize` | `Simd.Portable.Encoding.T0` | rlimit-saturated; max query 87 s, total 137 s |

These are the proofs **most reliant on hint replay**.  Each closes
under hints in ≤8 s (total) but cannot close cold without them at
rlimit ≤ 600 (max set on these functions today).

**Transitively blocked (30 modules)**: every keygen-cone wrapper —
all `Ml_dsa_generic.Instantiations.{Avx2,Portable,Neon}.Ml_dsa_*_`,
all `Ml_dsa_generic.Multiplexing.Ml_dsa_*_`, all `Ml_dsa_generic.Ml_dsa_*_`,
all `Ml_dsa_*_.{Avx2,Portable,Neon}`, plus `Simd.Portable` and
`Matrix`'s downstream consumers.  These modules don't fail on their
own merits; they're waiting on the 3 hard-failures' `.checked`.

## Results — Iter 2 (with 3 surgical admits)

After admitting the 3 root failures:

- 84 modules attempted, **84 verified, 0 errors**.
- All transitively-blocked modules cleared.
- Combined cumulative wall: an additional ~12 min.

This confirms the only structural hint-reliance is the 3 functions
above — every other ML-DSA proof closes cold without hints (some
slowly: see "fragile" group below).

## Top-25 cold-no-hints per-function totals

(Combined log; "FAILED, rlimit-sat" entries reflect Iter 1's pre-admit
failures.  Functions also count their FAILED-then-OK retry costs.)

| # | Function | Module | total (s) | max query (ms) | queries | flags |
|---|---|---|---:|---:|---:|---|
| 1 | `deserialize_when_gamma1_is_2_pow_17_` | `Simd.Portable.Encoding.Gamma1` | 461.94 | 219410 | 171 | **HARD-FAIL**, rlimit-sat |
| 2 | `outer_3_plus` | `Simd.Portable.Invntt` | 311.95 | 14772 | 162 | survived (~10× hints baseline) |
| 3 | `ntt_dot_accumulate` | `Matrix` | 171.06 | 73645 | 572 | borderline (max 73 s) |
| 4 | `compute_matrix_x_mask` | `Matrix` | 161.39 | 57485 | 130 | **HARD-FAIL**, rlimit-sat |
| 5 | `ntt_at_layer_3_` | `Simd.Portable.Ntt` | 140.48 | 12661 | 648 | survived (~2× hints baseline) |
| 6 | `deserialize` | `Simd.Portable.Encoding.T0` | 137.05 | 86860 | 142 | **HARD-FAIL**, rlimit-sat |
| 7 | `serialize_when_eta_is_2_aux` | `Simd.Avx2.Encoding.Error` | 136.88 | 136869 | 2 | borderline (single 137 s query) |
| 8 | `invert_ntt_at_layer_3_` | `Simd.Portable.Invntt` | 132.36 | 14843 | 648 | survived |
| 9 | `montgomery_multiply_by_constant` | `Simd.Portable.Arithmetic` | 122.78 | 60103 | 62 | rlimit-sat retry-and-OK |
| 10 | `impl_1` | `Simd.Avx2` | 63.33 | 13203 | 633 | rlimit-sat retry-and-OK |
| 11 | `serialize_aux` | `Simd.Avx2.Encoding.T0` | 56.20 | 56196 | 2 | borderline |
| 12 | `invert_ntt_at_layer_0_` | `Simd.Portable.Invntt` | 47.54 | 2870 | 392 | — |
| 13 | `serialize_6_` | `Simd.Avx2.Encoding.Commitment` | 47.06 | 47053 | 2 | borderline |
| 14 | `invert_ntt_montgomery` | `Simd.Portable.Invntt` | 45.91 | 8174 | 63 | survived |
| 15 | `ntt_at_layer_0_` | `Simd.Portable.Ntt` | 45.68 | 1453 | 392 | — |
| 16 | `decompose_element` | `Simd.Portable.Arithmetic` | 36.97 | 6901 | 82 | — |
| 17 | `impl_1` | `Simd.Portable` | 36.02 | 7887 | 547 | rlimit-sat retry-and-OK |
| 18 | `compute_w_approx` | `Matrix` | 34.22 | 7553 | 204 | survived |
| 19 | `montgomery_multiply` | `Simd.Portable.Arithmetic` | 32.66 | 32661 | 1 | rlimit-sat (single query) |
| 20 | `butterfly_2_` | `Simd.Avx2.Ntt` | 28.81 | 19525 | 65 | rlimit-sat retry-and-OK |
| 21 | `ntt_at_layer_1_` | `Simd.Portable.Ntt` | 28.74 | 1216 | 264 | — |
| 22 | `invert_ntt_at_layer_1_` | `Simd.Portable.Invntt` | 27.71 | 1531 | 264 | — |
| 23 | `outer_3_plus` | `Simd.Portable.Ntt` | 25.67 | 1441 | 128 | — |
| 24 | `ntt_at_layer_2_` | `Simd.Portable.Ntt` | 22.98 | 1150 | 200 | — |
| 25 | `generate_serialized` | `Encoding.Signing_key` | 22.53 | 4462 | 144 | — |

## Categorization

### A. HARD-RELIANT on hints (3 functions)

These cannot close without their cached hints:

1. `Libcrux_ml_dsa.Simd.Portable.Encoding.Gamma1::deserialize_when_gamma1_is_2_pow_17_`
2. `Libcrux_ml_dsa.Matrix::compute_matrix_x_mask`
3. `Libcrux_ml_dsa.Simd.Portable.Encoding.T0::deserialize`

**Root cause hypothesis**:
- `compute_matrix_x_mask`: heavy ambient-context proof in nested loop
  (per Sprint 3 `5d538df99` close).  The body has Z3-saturating
  asserts that closed at rlimit 400 + split-queries with hint replay,
  but only marginally.
- `deserialize_when_gamma1_is_2_pow_17_`: the Gamma1 18-bit unpack
  proof has a known per-byte assertion chain.  Cold-cache cost
  exploded from ≤1 s with hints to 462 s without.
- `deserialize` (T0): the 13-byte unpack with the strict-lower
  refinement post is similar.

These are precisely candidates that the trait-opacity remediation
(audit Phase A items 6, 21, 22 — t0 / t1 / Gamma1 strict-lower-bound
cleanup) might help, but they may also need their own SMT
restructuring (factor lemmas, smaller per-byte queries, etc.).

### B. RLIMIT-SAT FAILED-THEN-OK without hints (≥6 functions)

Functions that hit rlimit on at least one initial query attempt and
survived only via F\*'s automatic retry-with-split or retry-without-hint:

- `montgomery_multiply_by_constant` (Portable.Arithmetic) — total 123 s
- `impl_1` (Avx2) — total 63 s
- `impl_1` (Portable) — total 36 s
- `butterfly_2_` (Avx2.Ntt) — total 29 s
- `montgomery_multiply` (Portable.Arithmetic) — single query 32.7 s, used full rlimit

These succeed but at high cost.  Without F\*'s retry mechanism (which
currently includes `--split_queries always` per-function options),
they would join group A.

### C. SLOW BUT SUCCESSFUL without hints (~15 functions)

Functions in 25–150 s range that succeed cleanly:

- `outer_3_plus` (Invntt) — 312 s, 162 queries — slowest non-failure
- `ntt_dot_accumulate` (Matrix) — 171 s, max query 73 s — borderline
- `ntt_at_layer_3_` / `invert_ntt_at_layer_3_` — 140 s / 132 s each
- `serialize_when_eta_is_2_aux` (Avx2.Encoding.Error) — 137 s in 2 queries
  (single 137 s sub-query — borderline cliff)
- Plus `serialize_aux` (Avx2.Encoding.T0) at 56 s, `serialize_6_`
  (Avx2.Encoding.Commitment) at 47 s — both have borderline-cliff
  single sub-queries.

### D. FAST (≤22 s) without hints (most functions)

Everything below entry 25 in the table above.  These proofs have
small, well-structured search spaces that close cold within the
per-function rlimit even with no hint guidance.  The bulk of the
proof effort is in this category.

## Slowdown vs warm-cache hints baseline

Comparison points (cold-cache, hints intact, from `fstar-perf-top20.md`
Snapshot 2026-05-08):

| Function | Hints | No-hints | Slowdown |
|---|---:|---:|---:|
| `ntt_at_layer_3_` | 71 s | 140 s | 2.0× |
| `invert_ntt_at_layer_3_` | 68 s | 132 s | 1.9× |
| `Simd.Avx2::impl_1` | 57 s | 63 s | 1.1× |
| `Simd.Portable::impl_1` | 40 s | 36 s | 0.9× |
| `decompose_element` | 4.3 s | 37 s | 8.6× |
| `compute_w_approx` | 0.3 s | 34 s | 113× |
| `compute_matrix_x_mask` | <1 s | **fails** | ∞ |
| `T0::deserialize` | 7.7 s | **fails** | ∞ |
| `Gamma1::deserialize_when_gamma1_is_2_pow_17_` | 0.5 s | **fails** | ∞ |

Three functions go from <10 s with hints to **infinity** without
(can't close at rlimit 300–600).  Several others see 100×+
slowdown but eventually close.  The rest are nearly hint-independent
(impl_1 modules slightly differ; some functions even slightly
faster without hints — measurement variance).

## Priorities for the trait-opacity remediation

Group A failures will be the most useful test of the audit's Phase A
fixes:
- `compute_matrix_x_mask` is in `Matrix.fst` — uses the trait's
  bare-`forall i<32` posts on `ntt`/`invert_ntt`/`reduce` (audit
  items 25–27).  A successful poly-array-opacity remediation should
  collapse its hint-reliance.
- `Encoding.T0::deserialize` and `Gamma1::deserialize_when_gamma1_is_2_pow_17_`
  are in extraction modules.  Audit items 6 (t0_serialize),
  21 (t0_serialize), 22 (t0_deserialize), 16 (gamma1_serialize),
  17 (gamma1_deserialize) might address structural unfolds in their
  posts, but the SMT structure of bit-pack/unpack proofs is largely
  orthogonal — they likely need their own restructuring.

After each audit-Phase-A change, re-running `make -k verify ENABLE_HINTS=''`
on the affected modules will measure how much hint-reliance the change
removed.  The goal: shrink Group A (hard-fail) toward zero and shrink
Group B (FAILED-then-OK) so the warm-cache / cold-cache delta
narrows.

## Recovery steps taken

1. `cp -f /tmp/hints-backup-2026-05-08/*.hints` → `.fstar-cache/hints/`
2. `./hax.sh extract` → reverts `.fst` admits (extracted files are
   regenerated from Rust source, which doesn't carry the admits).
3. Cleared the 4 affected `.checked` files; re-ran `JOBS=4 ./hax.sh prove`
   to verify HEAD is back to clean state.

Tree state after recovery:
- `src/` clean (no commits made)
- `proofs/fstar/extraction/` clean (extracted from Rust)
- `.fstar-cache/hints/` restored from backup
- `.fstar-cache/checked/` regenerated by the restore prove
- `proofs/agent-status/` gains this file plus the prior baseline
  (`fstar-perf-top20.md` Snapshot 2026-05-08, `qi-baseline-2026-05-08.md`,
  `abstraction-boundary-audit-2026-05-07.md`)

---

**Headline number for the trait-opacity remediation kickoff**:
**3 functions are hard-reliant on hints** (every other ML-DSA proof
closes without them, slowly or quickly).  The remediation's success
metric should include "shrink that 3 to 0 or 1".

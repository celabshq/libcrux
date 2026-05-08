# F* verification time — top-20 culprits

This file tracks the slowest F* verification queries observed during
ML-DSA proof builds.  Updated after each significant build (full
`./hax.sh prove` or repo-wide `make`).  Snapshots are appended; do
not delete prior entries — diffs across snapshots reveal regressions
and improvements.

**Producer:** `./extract-fstar-perf.sh [N=20] [LOG=../verification_result.txt]`
parses `Query-stats` lines from a build log and emits two ranked
tables (per-function totals, per-module totals).  Run after a full
build and paste the output below as a new dated snapshot.

**Why we track this:** F* verification time is the sprint-deadline
threat.  A full prove on this repo is dominated by a small number of
borderline queries (NTT-layer butterflies, decompose mask logic,
montgomery_reduce_element).  Knowing the top 20 lets us:
- (a) optimize the proofs that matter (factor lemmas, tighter bounds,
  per-iteration unfolds, lower fuel/ifuel),
- (b) bypass them with `#push-options "--admit_smt_queries true"`
  during experiments where they're not the focus,
- (c) catch regressions — a function that climbs from #15 to #2
  between snapshots is either a real proof breakage or a hint-cache
  miss that needs `--record_hints`.

Cap: keep at most the most recent ~10 snapshots; older entries can
be moved to `fstar-perf-top20.archive.md` if the file gets long.

---

## Snapshot 2026-05-08b (after `--split_queries always` on impl_1 + reduce_with_proof)

Source: `verification_result.txt` from `JOBS=4 ./hax.sh prove` after
`rm -f /Users/karthik/libcrux-ml-dsa-proofs/.fstar-cache/checked/Libcrux_ml_dsa.*.fst.checked`.
HEAD `9b5b75b4b` + uncommitted edits this session:
- `Ml_dsa_generic::generate_key_pair` body re-admitted
- `Operations::montgomery_multiply` post: dropped `mont_mul` clause
  (audit Phase A item 10)
- `impl Operations for AVX2SIMDUnit` annotated with
  `#[hax_lib::fstar::options("--split_queries always")]`
- `impl Operations for Coefficients` (Portable) same
- `reduce_with_proof` (Portable free fn) same

Build: 99 modules invoked (incl. transitive Spec.\*), [CHECK]=82, [ADMIT]=17,
**0 F\* errors, 0 make-level failures**.

### Headline deltas vs Snapshot 2026-05-08

| Function | 05-08 | 05-08b | Δ |
|---|---:|---:|---|
| `Simd.Avx2::impl_1` | 57.40s, FAILED-then-OK rlimit-sat | **11.80s, clean** | **-79%** |
| `Simd.Portable::impl_1` | 39.53s, FAILED-then-OK rlimit-sat | **0.52s, clean (4 queries)** | **-99%** |
| `Simd.Portable::reduce_with_proof` | 17.75s, FAILED-then-OK rlimit-sat | **1.72s, clean** | **-90%** |

The Portable impl_1 collapse from 547 queries to 4 confirms the Track B
`*_with_proof` free-fn split was load-bearing — once `--split_queries always`
let F\* check each method's contract independently, impl_1's function-level
VC reduces to a handful of dispatcher checks.  Same expected (modulo Track
B not being applied to Avx2 yet) for the Avx2 collapse.

### Top-25 per-function totals

| # | Function | Module | total (s) | max query (ms) | queries | flags |
|---|---|---|---:|---:|---:|---|
| 1 | `ntt_at_layer_3_` | `L_md.Simd.Portable.Ntt` | 73.74 | 8613 | 36 | — |
| 2 | `invert_ntt_at_layer_3_` | `L_md.Simd.Portable.Invntt` | 72.55 | 8879 | 36 | — |
| 3 | `impl_1` | `L_md.Simd.Avx2` | 11.80 | 4101 | 40 | — |
| 4 | `deserialize` | `L_md.Simd.Portable.Encoding.T0` | 8.40 | 7603 | 18 | — |
| 5 | `ntt_at_layer_0_` | `L_md.Simd.Portable.Ntt` | 5.33 | 197 | 64 | — |
| 6 | `invert_ntt_at_layer_0_` | `L_md.Simd.Portable.Invntt` | 4.82 | 169 | 64 | — |
| 7 | `decompose_element` | `L_md.Simd.Portable.Arithmetic` | 4.61 | 338 | 18 | — |
| 8 | `ntt_at_layer_4_` | `L_md.Simd.Portable.Ntt` | 4.53 | 903 | 20 | — |
| 9 | `invert_ntt_at_layer_4_` | `L_md.Simd.Portable.Invntt` | 4.52 | 829 | 20 | — |
| 10 | `ntt_at_layer_1_` | `L_md.Simd.Portable.Ntt` | 3.76 | 130 | 64 | — |
| 11 | `invert_ntt_at_layer_1_` | `L_md.Simd.Portable.Invntt` | 3.49 | 116 | 64 | — |
| 12 | `ntt_at_layer_2_` | `L_md.Simd.Portable.Ntt` | 3.04 | 107 | 64 | — |
| 13 | `invert_ntt_at_layer_2_` | `L_md.Simd.Portable.Invntt` | 2.77 | 92 | 64 | — |
| 14 | `reduce_with_proof` | `L_md.Simd.Portable` | 1.72 | 571 | 95 | — |
| 15 | `ntt_dot_accumulate` | `L_md.Matrix` | 1.12 | 116 | 41 | — |
| 16 | `inv_ntt_and_add_s2` | `L_md.Matrix` | 0.54 | 43 | 20 | — |
| 17 | `impl_1` | `L_md.Simd.Portable` | 0.52 | 418 | 4 | — |
| 18 | `deserialize_when_gamma1_is_2_pow_17_` | `L_md.Simd.Portable.Encoding.Gamma1` | 0.51 | 55 | 14 | — |
| 19 | `generate_key_pair` | `L_md.Ml_dsa_generic.Ml_dsa_87_` | 0.41 | 248 | 5 | admitted body |
| 20 | `generate_key_pair` | `L_md.Ml_dsa_generic.Ml_dsa_65_` | 0.40 | 241 | 5 | admitted body |
| 21 | `generate_key_pair` | `L_md.Ml_dsa_generic.Ml_dsa_44_` | 0.39 | 231 | 5 | admitted body |
| 22 | `compute_w_approx` | `L_md.Matrix` | 0.31 | 39 | 11 | — |
| 23 | `ntt_at_layer_5_` | `L_md.Simd.Portable.Ntt` | 0.30 | 47 | 11 | — |
| 24 | `invert_ntt_at_layer_5_` | `L_md.Simd.Portable.Invntt` | 0.27 | 43 | 11 | — |
| 25 | `outer_3_plus` | `L_md.Simd.Portable.Invntt` | 0.25 | 29 | 13 | — |

### Top module totals

| # | Module | total (s) |
|---|---|---:|
| 1 | `L_md.Simd.Portable.Ntt` | 91.17 |
| 2 | `L_md.Simd.Portable.Invntt` | 89.10 |
| 3 | `L_md.Simd.Avx2` | 11.80 |
| 4 | `L_md.Simd.Portable.Encoding.T0` | 8.40 |
| 5 | `L_md.Simd.Portable.Arithmetic` | 5.17 |
| 6 | `L_md.Simd.Portable` | 2.24 |
| 7 | `L_md.Matrix` | 2.07 |

### Untouched outliers (potential next targets)

The big remaining cliffs are now:
1. `Simd.Portable.Ntt::ntt_at_layer_3_` (74s, 36 queries) — the per-layer
   NTT body proof is the new top.  Pre-existing structural shape; not
   helped by split_queries on the trait impl since this is below the
   trait impl block.
2. `Simd.Portable.Invntt::invert_ntt_at_layer_3_` (73s, same shape).
3. `Simd.Portable.Encoding.T0::deserialize` (8.4s, 18 queries) — also
   one of the 3 hard-fail-without-hints functions.

The hard-fail-without-hints set (per `hint-deletion-experiment-2026-05-08.md`)
pre-edit was 3 functions: `compute_matrix_x_mask`, `T0::deserialize`,
`Gamma1::deserialize_when_gamma1_is_2_pow_17_`.  Re-running the no-hints
prove with the current edits is in progress to confirm whether any of
those changed status (most likely unchanged — they were not the targets
of these edits).

---

## Snapshot 2026-05-08 (cold-cache baseline before trait-opacity remediation)

Source: `verification_result.txt` from `JOBS=4 ./hax.sh prove` after
`rm -f /Users/karthik/libcrux-ml-dsa-proofs/.fstar-cache/checked/Libcrux_ml_dsa.*.fst.checked`.
HEAD `9b5b75b4b` + tactical body-admit on `Ml_dsa_generic::generate_key_pair`
(see `proofs/agent-status/abstraction-boundary-audit-2026-05-07.md`).

Build: 85 modules invoked, [CHECK]=68, [ADMIT]=17, **0 F\* errors,
0 make-level failures**.  Cold-cache wall: ~30 min.  This is the
baseline against which the trait-opacity remediations
(audit Phase A items 10, 11, 12, 13–15, 20, 23, 24, 25, 26, 27)
will be measured.

**Why this matters for the audit**: the q60 cliff in `generate_key_pair`
that motivated the audit is invisible here (rows 20–22, all 0.36–0.38s)
because the body is admitted.  The trait-opacity remediation needs to
keep the trait-impl modules (rows 1–11) at or below their current
totals while *also* being strong enough to discharge `generate_key_pair`'s
body when the admit is removed.  Re-profile after each opacity change
+ before un-admitting.

### Top-25 per-function totals

| # | Function | Module | total (s) | max query (ms) | queries | flags |
|---|---|---|---:|---:|---:|---|
| 1 | `ntt_at_layer_3_` | `L_md.Simd.Portable.Ntt` | 71.11 | 8273 | 36 | — |
| 2 | `invert_ntt_at_layer_3_` | `L_md.Simd.Portable.Invntt` | 67.95 | 8295 | 36 | — |
| 3 | `impl_1` | `L_md.Simd.Avx2` | 57.40 | 11669 | 633 | FAILED-then-OK, rlimit-sat |
| 4 | `impl_1` | `L_md.Simd.Portable` | 39.53 | 13561 | 547 | FAILED-then-OK, rlimit-sat |
| 5 | `reduce_with_proof` | `L_md.Simd.Portable` | 17.75 | 15918 | 105 | FAILED-then-OK, rlimit-sat |
| 6 | `deserialize` | `L_md.Simd.Portable.Encoding.T0` | 7.70 | 7014 | 18 | — |
| 7 | `ntt_at_layer_0_` | `L_md.Simd.Portable.Ntt` | 4.77 | 168 | 64 | — |
| 8 | `invert_ntt_at_layer_0_` | `L_md.Simd.Portable.Invntt` | 4.50 | 160 | 64 | — |
| 9 | `invert_ntt_at_layer_4_` | `L_md.Simd.Portable.Invntt` | 4.35 | 828 | 20 | — |
| 10 | `ntt_at_layer_4_` | `L_md.Simd.Portable.Ntt` | 4.34 | 869 | 20 | — |
| 11 | `decompose_element` | `L_md.Simd.Portable.Arithmetic` | 4.29 | 338 | 18 | — |
| 12 | `power2round` | `L_md.Simd.Avx2.Arithmetic` | 3.58 | 3576 | 1 | — |
| 13 | `ntt_at_layer_1_` | `L_md.Simd.Portable.Ntt` | 3.34 | 115 | 64 | — |
| 14 | `invert_ntt_at_layer_1_` | `L_md.Simd.Portable.Invntt` | 3.31 | 112 | 64 | — |
| 15 | `ntt_at_layer_2_` | `L_md.Simd.Portable.Ntt` | 2.74 | 97 | 64 | — |
| 16 | `invert_ntt_at_layer_2_` | `L_md.Simd.Portable.Invntt` | 2.66 | 89 | 64 | — |
| 17 | `ntt_dot_accumulate` | `L_md.Matrix` | 1.08 | 125 | 41 | — |
| 18 | `inv_ntt_and_add_s2` | `L_md.Matrix` | 0.49 | 40 | 20 | — |
| 19 | `deserialize_when_gamma1_is_2_pow_17_` | `L_md.Simd.Portable.Encoding.Gamma1` | 0.48 | 53 | 14 | — |
| 20 | `generate_key_pair` | `L_md.Ml_dsa_generic.Ml_dsa_65_` | 0.38 | 233 | 5 | admitted body |
| 21 | `generate_key_pair` | `L_md.Ml_dsa_generic.Ml_dsa_87_` | 0.37 | 229 | 5 | admitted body |
| 22 | `generate_key_pair` | `L_md.Ml_dsa_generic.Ml_dsa_44_` | 0.36 | 227 | 5 | admitted body |
| 23 | `compute_w_approx` | `L_md.Matrix` | 0.29 | 35 | 11 | — |
| 24 | `ntt_at_layer_5_` | `L_md.Simd.Portable.Ntt` | 0.28 | 43 | 11 | — |
| 25 | `invert_ntt_at_layer_5_` | `L_md.Simd.Portable.Invntt` | 0.25 | 39 | 11 | — |

### Top-25 module totals

| # | Module | total (s) | functions tracked |
|---|---|---:|---:|
| 1 | `L_md.Simd.Portable.Ntt` | 87.00 | 14 |
| 2 | `L_md.Simd.Portable.Invntt` | 83.68 | 13 |
| 3 | `L_md.Simd.Avx2` | 57.40 | 1 |
| 4 | `L_md.Simd.Portable` | 57.28 | 2 |
| 5 | `L_md.Simd.Portable.Encoding.T0` | 7.70 | 1 |
| 6 | `L_md.Simd.Portable.Arithmetic` | 4.79 | 4 |
| 7 | `L_md.Simd.Avx2.Arithmetic` | 3.68 | 6 |
| 8 | `L_md.Matrix` | 1.97 | 4 |
| 9 | `L_md.Simd.Portable.Encoding.Gamma1` | 0.61 | 2 |
| 10 | `L_md.Ml_dsa_generic.Ml_dsa_65_` | 0.38 | 1 |
| 11 | `L_md.Ml_dsa_generic.Ml_dsa_87_` | 0.37 | 1 |
| 12 | `L_md.Ml_dsa_generic.Ml_dsa_44_` | 0.36 | 1 |
| 13 | `L_md.Polynomial` | 0.24 | 1 |
| 14 | `L_md.Encoding.Signature` | 0.21 | 1 |
| 15 | `L_md.Encoding.Signing_key` | 0.20 | 1 |
| 16 | `L_md.Simd.Avx2.Invntt` | 0.17 | 1 |
| 17 | `L_md.Simd.Avx2.Ntt` | 0.14 | 3 |
| 18 | `L_md.Encoding.Verification_key` | 0.14 | 1 |
| 19 | `L_md.Simd.Avx2.Encoding.Error` | 0.06 | 1 |
| 20 | `L_md.Simd.Avx2.Encoding.Gamma1` | 0.05 | 2 |
| 21 | `L_md.Simd.Avx2.Encoding.T0` | 0.04 | 3 |

_Sample size: 2080 Query-stats lines, ~306s total Z3 time, across 64
functions in 21 modules._

### Observations vs 2026-04-29b

| Observation | Then (2026-04-29b) | Now (2026-05-08) |
|---|---|---|
| `ntt_at_layer_3_` total | 132.42 s, 684 queries | 71.11 s, **36 queries** |
| `invert_ntt_at_layer_3_` total | 132.42 s, 684 queries | 67.95 s, **36 queries** |
| `Simd.Avx2::impl_1` total | 50.35 s, 603 queries | 57.40 s, 633 queries |
| `Simd.Portable::impl_1` total | 35.66 s, 541 queries | 39.53 s, 547 queries |
| `reduce_with_proof` total | 20.84 s, 106 queries | 17.75 s, 105 queries |
| `decompose_element` total | 16.66 s, 100 queries | 4.29 s, 18 queries |

The `ntt_at_layer_3_` query count drop (684 → 36) is the headline
change.  Sprint 3+3.5+4 closures (compute_matrix_x_mask, opacify
range/slice, compute_w_approx) and the keygen-cone opacification
appear to have collapsed a quantifier branching factor in the NTT
trait impl.  The total time dropped accordingly (132 → 71 s).

**`decompose_element` saw a similar drop** (100 → 18 queries; 16.66 →
4.29 s).  Likely fallout from the per-lane `decompose_lane_post`
opacity work (Phase B item ⑤ in the audit).

`Simd.Avx2::impl_1` and `Simd.Portable::impl_1` and `reduce_with_proof`
remain at the same FAILED-then-OK rlimit-saturated pattern as
2026-04-29b — pre-existing, not a regression.

**New on this snapshot**: `Simd.Portable.Encoding.T0::deserialize`
at 7.70 s with max single query 7.0 s in just 18 queries — a single
borderline-cliff sub-query.  Cause unknown; not in 2026-04-29b.
Targeted qi.profile candidate.

### qi.profile targets (next step)

In order of expected leverage on the trait-opacity remediation:

1. `Simd.Avx2::impl_1` max query 11.7 s — touches the AVX2 trait
   impl whose post/pre is the audit's Phase A target.
2. `Simd.Portable::impl_1` max query 13.6 s — same shape as above.
3. `Simd.Portable::reduce_with_proof` max query 15.9 s — uses
   `reduce_lane_post` (audit Phase B item ④).
4. `Simd.Portable.Encoding.T0::deserialize` max query 7.0 s — new
   outlier; likely a stale-shape consumer of `t0_deserialize` post.
5. `Simd.Portable.Ntt::ntt_at_layer_3_` max query 8.3 s — touches
   the NTT trait pre/post (audit Phase A item 25).
6. `Simd.Portable.Invntt::invert_ntt_at_layer_3_` max query 8.3 s —
   same shape, audit Phase A item 26.
7. `Simd.Avx2.Arithmetic::power2round` total 3.6 s in **1 query** —
   single borderline query, audit Phase A item 12 candidate.

---

## Snapshot 2026-04-29c (Step 14 Track D-2 — `*_deserialize` trait body close, partial)

Source: `/tmp/d2-iter12.log` (Step 14 Track D-2 close commit batch
`62a50deeb`, `10b15d325`, `4ec0e9f50`).  Build invalidated only the
encoding modules whose ensures changed plus their direct dependents
(Portable.Encoding.{T1,T0,Gamma1}, Avx2.Encoding.{T1,T0,Gamma1},
Simd.Portable, Simd.Avx2).  NTT / Invntt cascade .checked files
were not invalidated and so this snapshot only reflects the affected
modules — comparison with 2026-04-29b is per-function, not aggregate.

Build: 77 modules invoked, [CHECK]=29, [ADMIT]=48, 0 F\* errors.
Six trait body admits removed (3 methods × 2 impls).

### Top per-function totals (affected modules only)

| # | Function | Module | total (s) | max query (ms) | queries | flags |
|---|---|---|---:|---:|---:|---|
| 1 | `impl_1` | `L_md.Simd.Portable` | 36.45 | 8943 | 555 | FAILED-then-OK, rlimit-sat |
| 2 | `reduce_with_proof` | `L_md.Simd.Portable` | 21.26 | 19014 | 105 | FAILED-then-OK, rlimit-sat |
| 3 | `deserialize_when_gamma1_is_2_pow_17_` | `L_md.Simd.Portable.Encoding.Gamma1` | 0.46 | 53 | 14 | — |
| 4 | `deserialize_when_gamma1_is_2_pow_19_` | `L_md.Simd.Portable.Encoding.Gamma1` | 0.14 | 29 | 7 | — |
| 5 | `deserialize` | `L_md.Simd.Portable.Encoding.Gamma1` | 0.02 | 19 | 1 | — |

The new `deserialize_when_gamma1_is_2_pow_*` proofs add ~0.5s to
gamma1 module total — well within budget.  No changes in the
Portable.{NTT,Invntt} or impl_1 / reduce_with_proof times relative
to 2026-04-29b (those .checked files were not invalidated this
build, so values not in this snapshot).

The persistent rlimit-saturation on `impl_1` (Portable trait impl
discharge) and `reduce_with_proof` are pre-existing — same shape as
2026-04-29b's #6 / #14.  No regression introduced by this Track D-2
work.

### Top module totals (affected modules)

| # | Module | total (s) | functions tracked |
|---|---|---:|---:|
| 1 | `L_md.Simd.Portable` | 57.71 | 2 |
| 2 | `L_md.Simd.Portable.Encoding.Gamma1` | 0.62 | 3 |

_Sample size: 996 Query-stats lines, ~58s total Z3 time, across
5 functions in 2 modules._

---

## Snapshot 2026-04-29b (Step 14 Track 0 final, full NTT/Invntt cascade)

Source: `/tmp/track0-final.log` after F-13 revert cherry-pick from
above-trait `341a93d4d`.  `Simd.Traits.fst` regenerated → full
NTT/Invntt cascade re-verified (snapshot is **complete**, not partial
like 2026-04-29a).

Build: 74 modules invoked, [CHECK]=27, [ADMIT]=47, 0 F\* errors,
0 make-level failures.  ([ADMIT] dropped from 48 → 47: the F-13
helper-lemma admit was removed, and below-trait `decompose` body now
discharges fully.)

### Top-20 per-function totals

| # | Function | Module | total (s) | max query (ms) | queries | flags |
|---|---|---|---:|---:|---:|---|
| 1 | `invert_ntt_at_layer_3_` | `L_md.Simd.Portable.Invntt` | 132.42 | 8893 | 684 | FAILED-then-OK |
| 2 | `ntt_at_layer_3_` | `L_md.Simd.Portable.Ntt` | 130.61 | 8390 | 684 | FAILED-then-OK |
| 3 | `impl_1` | `L_md.Simd.Avx2` | 50.35 | 12764 | 603 | FAILED-then-OK |
| 4 | `ntt_at_layer_0_` | `L_md.Simd.Portable.Ntt` | 46.20 | 562 | 456 | FAILED-then-OK |
| 5 | `invert_ntt_at_layer_0_` | `L_md.Simd.Portable.Invntt` | 45.86 | 547 | 456 | FAILED-then-OK |
| 6 | `impl_1` | `L_md.Simd.Portable` | 35.66 | 9488 | 541 | FAILED-then-OK |
| 7 | `invert_ntt_at_layer_4_` | `L_md.Simd.Portable.Invntt` | 33.49 | 892 | 348 | FAILED-then-OK |
| 8 | `ntt_at_layer_1_` | `L_md.Simd.Portable.Ntt` | 32.80 | 552 | 328 | FAILED-then-OK |
| 9 | `ntt_at_layer_4_` | `L_md.Simd.Portable.Ntt` | 32.68 | 864 | 348 | FAILED-then-OK |
| 10 | `invert_ntt_at_layer_1_` | `L_md.Simd.Portable.Invntt` | 32.54 | 548 | 328 | FAILED-then-OK |
| 11 | `outer_3_plus` | `L_md.Simd.Portable.Invntt` | 32.19 | 2414 | 175 | FAILED-then-OK |
| 12 | `ntt_at_layer_2_` | `L_md.Simd.Portable.Ntt` | 26.21 | 548 | 264 | FAILED-then-OK |
| 13 | `invert_ntt_at_layer_2_` | `L_md.Simd.Portable.Invntt` | 26.09 | 540 | 264 | FAILED-then-OK |
| 14 | `reduce_with_proof` | `L_md.Simd.Portable` | 20.84 | 18585 | 106 | FAILED-then-OK |
| 15 | `decompose_element` | `L_md.Simd.Portable.Arithmetic` | 16.66 | 1899 | 100 | FAILED-then-OK |
| 16 | `ntt_at_layer_5_` | `L_md.Simd.Portable.Ntt` | 14.85 | 108 | 179 | FAILED-then-OK |
| 17 | `invert_ntt_at_layer_5_` | `L_md.Simd.Portable.Invntt` | 14.63 | 106 | 179 | FAILED-then-OK |
| 18 | `montgomery_reduce_element` | `L_md.Simd.Portable.Arithmetic` | 13.49 | 266 | 144 | — |
| 19 | `outer_3_plus` | `L_md.Simd.Portable.Ntt` | 13.08 | 700 | 136 | FAILED-then-OK |
| 20 | `ntt_at_layer_6_` | `L_md.Simd.Portable.Ntt` | 7.80 | 100 | 93 | FAILED-then-OK |

### Top module totals

| # | Module | total (s) | functions tracked |
|---|---|---:|---:|
| 1 | `L_md.Simd.Portable.Invntt` | 349.94 | 27 |
| 2 | `L_md.Simd.Portable.Ntt` | 332.79 | 29 |
| 3 | `L_md.Simd.Portable` | 57.12 | 7 |
| 4 | `L_md.Simd.Avx2` | 51.99 | 6 |
| 5 | `L_md.Simd.Portable.Arithmetic` | 50.81 | 20 |
| 6 | `L_md.Simd.Avx2.Ntt` | 20.56 | 24 |
| 7 | `L_md.Simd.Traits` | 15.81 | 98 |
| 8 | `L_md.Simd.Avx2.Invntt` | 12.51 | 30 |
| 9 | `L_md.Simd.Avx2.Arithmetic` | 11.41 | 12 |
| 10 | `L_md.Simd.Portable.Encoding.Gamma1` | 10.37 | 12 |

_Sample size: 7654 Query-stats lines, 928s total Z3 time, across 289
functions in 15 modules.  Note: this snapshot does NOT include
`H_md.Commute.Chunk` (lemma_mont_mul_bound_and_mod_q dominated 2026-04-29a
at 283 s) — that module did not need to re-verify in this build because
its checked file remained valid through the traits.fst regen._

### Notes / actions

1. **NTT/Invntt cascade is the dominant cost** (~683 s combined across
   the two modules — 73% of total Z3 time).  This is the cost of
   re-verifying the ring-element-level proofs after touching
   `traits.fst`.  The `touch-unchanged-checked.sh` workflow normally
   skips this; here the F-13 cherry-pick was the *intended* trigger
   and the cascade is unavoidable.
2. **`reduce_with_proof` Portable max single query 18.5 s** (up from
   17.5 s in 2026-04-29a, +6%) — within noise for this rlimit.
3. **`decompose_element` 100 queries / 16.7 s** appears high but is
   `--split_queries always` per its existing options, so the count is
   inflated by design.  Per-query max 1.9 s.

---

## Snapshot 2026-04-29a (Step 14 Track 0 partial, post-cascade-skip)

Source: `/tmp/track0-attempt1.log` (intermediate, before F-13 revert).
After Track 0 c6c68bbca propagation
(F-13 filed, power2round + AVX2 t0_serialize closed strict-lower,
decompose strict-lower locally admitted).  `touch-unchanged-checked.sh`
skipped 102 unchanged modules (NTT/Invntt cascade not re-verified);
sample is the 35 functions across 3 modules that DID re-verify.

Build: 75 modules invoked, [CHECK]=27, [ADMIT]=48, 0 F\* errors,
0 make-level failures.

### Top-20 per-function totals

| # | Function | Module | total (s) | max query (ms) | queries | flags |
|---|---|---|---:|---:|---:|---|
| 1 | `lemma_mont_mul_bound_and_mod_q` | `H_md.Commute.Chunk` | 283.24 | 283235 | 1 | — |
| 2 | `impl_1` | `L_md.Simd.Avx2` | 56.18 | 13964 | 600 | FAILED-then-OK |
| 3 | `impl_1` | `L_md.Simd.Portable` | 35.49 | 9198 | 538 | FAILED-then-OK |
| 4 | `reduce_with_proof` | `L_md.Simd.Portable` | 19.95 | 17531 | 106 | FAILED-then-OK |
| 5 | `power2round_with_proof` | `L_md.Simd.Avx2` | 2.50 | 2364 | 2 | FAILED-then-OK |
| 6 | `lemma_use_hint_lane_commute_conditional` | `H_md.Commute.Chunk` | 2.47 | 2472 | 1 | — |
| 7 | `lemma_power2round_lane_commute` | `H_md.Commute.Chunk` | 1.04 | 1044 | 1 | — |
| 8 | `reduce_with_proof` | `L_md.Simd.Avx2` | 0.81 | 718 | 2 | — |
| 9 | `power2round_with_proof` | `L_md.Simd.Portable` | 0.61 | 313 | 2 | FAILED-then-OK |
| 10 | `lemma_barrett_red_bound_and_mod_q` | `H_md.Commute.Chunk` | 0.46 | 456 | 1 | — |
| 11 | `lemma_decompose_bridge` | `H_md.Commute.Chunk` | 0.33 | 332 | 1 | — |
| 12 | `lemma_spec_decompose_r1_bound` | `H_md.Commute.Chunk` | 0.24 | 238 | 1 | — |
| 13 | `lemma_compute_hint_bound_aux` | `H_md.Commute.Chunk` | 0.23 | 141 | 2 | — |
| 14 | `lemma_shift_left_then_reduce_lane_commute` | `H_md.Commute.Chunk` | 0.21 | 214 | 1 | — |
| 15 | `lemma_mod_pm_eq_mod_p` | `H_md.Commute.Chunk` | 0.21 | 211 | 1 | — |
| 16 | `lemma_shift_left_then_reduce_lane_commute_mod_q` | `H_md.Commute.Chunk` | 0.21 | 209 | 1 | — |
| 17 | `lemma_power2round_t1_bound` | `H_md.Commute.Chunk` | 0.17 | 175 | 1 | — |
| 18 | `lemma_compute_hint_bound` | `H_md.Commute.Chunk` | 0.17 | 87 | 2 | — |
| 19 | `infinity_norm_exceeds_with_proof` | `L_md.Simd.Avx2` | 0.16 | 163 | 1 | — |
| 20 | `infinity_norm_exceeds_with_proof` | `L_md.Simd.Portable` | 0.15 | 151 | 1 | — |

### Top module totals

| # | Module | total (s) | functions tracked |
|---|---|---:|---:|
| 1 | `H_md.Commute.Chunk` | 289.89 | 22 |
| 2 | `L_md.Simd.Avx2` | 59.87 | 6 |
| 3 | `L_md.Simd.Portable` | 56.53 | 7 |

_Sample size: 1281 Query-stats lines, 406s total Z3 time, across 35
functions in 3 modules.  The new lemma
`lemma_power2round_t0_strict_lower_bound` ran in 0.11 s with 1 query —
not in top 20.  Power2round_with_proof on AVX2 split into 2 queries
(was 1 prior to Track 0); first sub-query failed with "incomplete
quantifiers" then the split succeeded — 2.5 s total, used rlimit
21.6/80, safe margin._

### Notes / actions

1. **`lemma_mont_mul_bound_and_mod_q` is the single biggest cost
   (283 s, 1 query, max 283 s).**  This lemma has been at the top
   for several snapshots; remains the priority for future
   optimization (factor sub-lemmas, lower fuel/ifuel, or move to
   a hint-cached separate query).  Track 0 did not change this.
2. **AVX2 power2round_with_proof split-query overhead** (1 → 2
   queries for a +0.5 s cost) is the price of adding the strict-lower
   derivation via the new math lemma.  Acceptable.
3. **`reduce_with_proof` Portable max single query 17.5 s** is the
   second hot spot after mont_mul.  If this regresses, the mont_mul
   factor approach (separate lemma + single SMTPat) likely applies.

---

## Snapshot 2026-04-28 (Step 9 mid-cascade, partial)

Source: `/tmp/sltr-portable.log` (in-flight Portable rebuild for
shift_left_then_reduce; cascade from `Specs.fst` change still
re-verifying NTT/Invntt; Avx2 modules not yet reached).

**Caveat:** this is a *partial* sample (~100s Z3 time across 25 fns
in 3 Portable modules).  A full snapshot needs a complete
`./hax.sh prove` post-cascade.

### Top-20 per-function totals

| # | Function | Module | total (s) | max query (ms) | queries | flags |
|---|---|---|---:|---:|---:|---|
| 1 | `ntt_at_layer_3_` | `L_md.Simd.Portable.Ntt` | 68.33 | 8285 | 36 | — |
| 2 | `ntt_at_layer_0_` | `L_md.Simd.Portable.Ntt` | 4.84 | 170 | 64 | — |
| 3 | `ntt_at_layer_4_` | `L_md.Simd.Portable.Ntt` | 4.40 | 892 | 20 | — |
| 4 | `invert_ntt_at_layer_0_` | `L_md.Simd.Portable.Invntt` | 4.39 | 156 | 64 | — |
| 5 | `decompose_element` | `L_md.Simd.Portable.Arithmetic` | 4.12 | 292 | 18 | — |
| 6 | `ntt_at_layer_1_` | `L_md.Simd.Portable.Ntt` | 3.61 | 117 | 64 | — |
| 7 | `invert_ntt_at_layer_1_` | `L_md.Simd.Portable.Invntt` | 3.39 | 120 | 64 | — |
| 8 | `ntt_at_layer_2_` | `L_md.Simd.Portable.Ntt` | 2.87 | 99 | 64 | — |
| 9 | `invert_ntt_at_layer_2_` | `L_md.Simd.Portable.Invntt` | 2.71 | 89 | 64 | — |
| 10 | `outer_3_plus` | `L_md.Simd.Portable.Invntt` | 0.25 | 30 | 13 | — |
| 11 | `ntt_at_layer_5_` | `L_md.Simd.Portable.Ntt` | 0.23 | 35 | 11 | — |
| 12 | `power2round_element` | `L_md.Simd.Portable.Arithmetic` | 0.20 | 46 | 5 | — |
| 13 | `get_n_least_significant_bits` | `L_md.Simd.Portable.Arithmetic` | 0.19 | 32 | 13 | — |
| 14 | `outer_3_plus` | `L_md.Simd.Portable.Ntt` | 0.14 | 25 | 8 | — |
| 15 | `uuse_one_hint` | `L_md.Simd.Portable.Arithmetic` | 0.09 | 46 | 2 | — |
| 16 | `ntt_at_layer_6_` | `L_md.Simd.Portable.Ntt` | 0.06 | 17 | 5 | — |
| 17 | `simd_unit_ntt_at_layer_0_` | `L_md.Simd.Portable.Ntt` | 0.06 | 39 | 3 | — |
| 18 | `outer_3_plus__round` | `L_md.Simd.Portable.Ntt` | 0.05 | 18 | 4 | — |
| 19 | `simd_unit_invert_ntt_at_layer_0_` | `L_md.Simd.Portable.Invntt` | 0.05 | 37 | 3 | — |
| 20 | `simd_unit_ntt_at_layer_2_` | `L_md.Simd.Portable.Ntt` | 0.05 | 10 | 6 | — |

### Top module totals

| # | Module | total (s) | functions tracked |
|---|---|---:|---:|
| 1 | `L_md.Simd.Portable.Ntt` | 84.68 | 14 |
| 2 | `L_md.Simd.Portable.Invntt` | 10.86 | 7 |
| 3 | `L_md.Simd.Portable.Arithmetic` | 4.61 | 4 |

_Sample size: 549 Query-stats lines, 100s total Z3 time, across 25
functions in 3 modules.  Partial; full snapshot pending end-of-cascade._

### Notes / actions

1. **`ntt_at_layer_3_` dominates** — 68 s alone, max single query
   8.3 s.  This is 81% of all observed Z3 time in this slice.
   Optimization candidates (in order of effort): (a) split the
   function body into 4 per-zeta sub-functions to localize the wide
   context; (b) drop fuel from 0 to 0/ifuel 0; (c) factor any
   intermediate `assert (forall i. ...)` into a separate lemma so
   Z3 sees the conclusion at function-level without re-deriving.
2. **NTT layers 0-2 have stable budgets** (~3-5 s each, 64 queries) —
   suggests these are at a good local optimum; layer 3 is the
   outlier.
3. **`decompose_element`** comes in at 4.1 s — non-NTT but high.
   Worth checking if the corner-case `mask` reasoning could factor
   into a sub-lemma.
4. The current verification_result.txt at repo root is mostly
   ADMIT-mode emissions (no Query-stats) so this snapshot draws from
   the in-flight cascade log instead.

---

## Snapshot 2026-04-28 (Step 10 Track A+B close)

Source: `verification_result.txt` after the Step 10 sweep:
- Track A (impl post lifts) commit `202686c56`
- Track B (one-line wrapper refactor) commit (this one)
- F-2 trait fix (`v gamma2` cast) included in Track A.

Cache mostly warm; Query-stats reflect re-checked Simd.{Portable,Avx2}.fst
modules plus their newly-extracted `*_with_proof` wrapper functions.

### Top-10 per-function totals

| # | Function | Module | total (s) | max query (ms) | queries | flags |
|---|---|---|---:|---:|---:|---|
| 1 | `reduce_with_proof` | `L_md.Simd.Portable` | 18.86 | 16499 | 106 | FAILED, rlimit-sat |
| 2 | `impl_1` | `L_md.Simd.Avx2` | 4.47 | 4468 | 1 | — |
| 3 | `reduce_with_proof` | `L_md.Simd.Avx2` | 1.15 | 1125 | 2 | — |
| 4 | `power2round_with_proof` | `L_md.Simd.Avx2` | 0.81 | 812 | 1 | — |
| 5 | `power2round_with_proof` | `L_md.Simd.Portable` | 0.46 | 462 | 1 | — |
| 6 | `infinity_norm_exceeds_with_proof` | `L_md.Simd.Avx2` | 0.32 | 317 | 1 | — |
| 7 | `shift_left_then_reduce_with_proof` | `L_md.Simd.Portable` | 0.22 | 218 | 1 | — |
| 8 | `shift_left_then_reduce_with_proof` | `L_md.Simd.Avx2` | 0.12 | 122 | 1 | — |
| 9 | `infinity_norm_exceeds_with_proof` | `L_md.Simd.Portable` | 0.10 | 101 | 1 | — |
| 10 | `montgomery_multiply_with_proof` | `L_md.Simd.Portable` | 0.10 | 95 | 1 | — |

### Top module totals

| # | Module | total (s) | functions tracked |
|---|---|---:|---:|
| 1 | `L_md.Simd.Portable` | 19.74 | 5 |
| 2 | `L_md.Simd.Avx2` | 6.87 | 5 |

_Sample size: 116 Query-stats lines, 27s total Z3 time, across 10 functions in 2 modules.  Note: the prove was warm-cache so NTT/Invntt sub-modules don't appear in this slice._

### Notes / actions

1. **Track B effect on `impl_1`:** Portable `impl_1` dropped out of the
   top-10 entirely (was #3 at 9.24s in the Track A baseline; now
   absorbed by per-method wrapper VCs).  AVX2 `impl_1` dropped from
   46.42s/637q (Track A, before splitting) to 4.47s/1q (Track B).
   The 21 method bodies are now one-liner dispatches, so the
   record-level VC just checks signatures and trivially-implied
   posts — fast.
2. **Portable `reduce_with_proof` at #1 (18.86s, FAILED, rlimit-sat):**
   query #1 (the unsplit single-query attempt) failed at rlimit 80
   in 16.5s, but F\*'s `--split_queries always` retried as 105 split
   queries that all succeeded (max 70ms each, none rlimit-sat).
   Total verification passes (0 errors).  Future optimization:
   factor the after-loop `Classical.forall_intro pf` into a
   standalone lemma, or drop fuel/ifuel.  Not blocking.
3. **AVX2 `impl_1` still rank #2 at 4.47s:** this is the
   record-level VC for the 21 methods.  Most of the work moved
   to per-method wrappers (good); the residual is checking that
   the impl record's pred-fields satisfy the trait refinements.
   To drop further, would need to factor more of the per-record
   subtyping into per-method lemmas.  Not currently a bottleneck.
4. **`shift_left_then_reduce_with_proof` 0.22s/0.12s, 1 query each:**
   the wrapper extraction kept these tiny.  Same pattern for
   `infinity_norm_exceeds`, `montgomery_multiply`, `power2round`.
5. **NTT/Invntt sub-modules absent from this slice:** the prove
   was incremental, so NTT layers (which dominated the Step 9.3
   snapshot at 70+ s each) didn't re-run.  A cold-cache full prove
   would still surface them at the top.

---

## Snapshot 2026-04-28 (Step 9.3 close)

Source: combined `/tmp/sltr-portable.log` + `/tmp/sltr-avx2.log` +
`verification_result.txt` after Step 9.3 commit (`b20a993a6`).
650 Query-stats lines / 197 s total Z3 / 45 functions in 10 modules.

### Top-20 per-function totals

| # | Function | Module | total (s) | max query (ms) | queries | flags |
|---|---|---|---:|---:|---:|---|
| 1 | `invert_ntt_at_layer_3_` | `L_md.Simd.Portable.Invntt` | 70.97 | 8533 | 36 | — |
| 2 | `ntt_at_layer_3_` | `L_md.Simd.Portable.Ntt` | 68.33 | 8285 | 36 | — |
| 3 | `impl_1` | `L_md.Simd.Portable` | 11.12 | 11116 | 1 | FAILED, rlimit-sat |
| 4 | `impl_1` | `L_md.Simd.Avx2` | 9.52 | 9517 | 1 | rlimit-sat |
| 5 | `ntt_at_layer_0_` | `L_md.Simd.Portable.Ntt` | 4.84 | 170 | 64 | — |
| 6 | `ntt_at_layer_4_` | `L_md.Simd.Portable.Ntt` | 4.40 | 892 | 20 | — |
| 7 | `invert_ntt_at_layer_0_` | `L_md.Simd.Portable.Invntt` | 4.39 | 156 | 64 | — |
| 8 | `invert_ntt_at_layer_4_` | `L_md.Simd.Portable.Invntt` | 4.28 | 822 | 20 | — |
| 9 | `decompose_element` | `L_md.Simd.Portable.Arithmetic` | 4.12 | 292 | 18 | — |
| 10 | `ntt_at_layer_1_` | `L_md.Simd.Portable.Ntt` | 3.61 | 117 | 64 | — |
| 11 | `invert_ntt_at_layer_1_` | `L_md.Simd.Portable.Invntt` | 3.39 | 120 | 64 | — |
| 12 | `ntt_at_layer_2_` | `L_md.Simd.Portable.Ntt` | 2.87 | 99 | 64 | — |
| 13 | `invert_ntt_at_layer_2_` | `L_md.Simd.Portable.Invntt` | 2.71 | 89 | 64 | — |
| 14 | `outer_3_plus` | `L_md.Simd.Portable.Invntt` | 0.25 | 30 | 13 | — |
| 15 | `invert_ntt_at_layer_5_` | `L_md.Simd.Portable.Invntt` | 0.25 | 39 | 11 | — |
| 16 | `ntt_at_layer_5_` | `L_md.Simd.Portable.Ntt` | 0.23 | 35 | 11 | — |
| 17 | `impl_2__to_i32_array` | `L_md.Polynomial` | 0.21 | 215 | 1 | — |
| 18 | `power2round_element` | `L_md.Simd.Portable.Arithmetic` | 0.20 | 46 | 5 | — |
| 19 | `invert_ntt_montgomery` | `L_md.Simd.Portable.Invntt` | 0.19 | 20 | 13 | — |
| 20 | `get_n_least_significant_bits` | `L_md.Simd.Portable.Arithmetic` | 0.19 | 32 | 13 | — |

### Top module totals

| # | Module | total (s) | functions tracked |
|---|---|---:|---:|
| 1 | `L_md.Simd.Portable.Invntt` | 86.62 | 13 |
| 2 | `L_md.Simd.Portable.Ntt` | 84.68 | 14 |
| 3 | `L_md.Simd.Portable` | 11.12 | 1 |
| 4 | `L_md.Simd.Avx2` | 9.52 | 1 |
| 5 | `L_md.Simd.Portable.Arithmetic` | 4.61 | 4 |
| 6 | `L_md.Polynomial` | 0.21 | 1 |
| 7 | `H_md.Commute.Chunk` | 0.20 | 2 |
| 8 | `L_md.Simd.Avx2.Invntt` | 0.14 | 1 |
| 9 | `L_md.Simd.Avx2.Ntt` | 0.12 | 3 |
| 10 | `L_md.Simd.Avx2.Arithmetic` | 0.09 | 5 |

### Notes / actions

1. **`invert_ntt_at_layer_3_` and `ntt_at_layer_3_` are the
   undisputed culprits.** Together they consume 139 s of the 197 s
   sampled budget (70%). Each has 36 split queries with single-query
   max ~8.5 s. **Optimization candidates** (in increasing effort):
   (a) drop `--fuel 1` to `--fuel 0` if not needed for the inductive
   loop_invariant; (b) factor each layer's loop body into a separate
   per-zeta lemma and `Classical.forall_intro` over zeta indices —
   collapses 36 split queries into 8 lemma-call queries; (c) split
   the function into 4 sub-functions matching the 4-zeta SIMD parallel
   pattern (the same approach we already use for AVX2 NTT layer-1/2).
2. **`impl_1` rlimit-sat (rows 3-4)** — these are the trait-method
   `impl` definitions in `Simd.Portable.fst` and `Simd.Avx2.fst`,
   which now have ~30 method bodies with proof bodies inline. Single
   query, single `--fuel 0 --ifuel 1 --rlimit 80` — at 11 s and 9.5 s
   each they're saturating. Per-method `#push-options` to localize
   rlimit/fuel may help, but this is fundamentally because all 30
   methods share one function-level VC. Splitting `Simd.Portable.fst`
   per-method would help (one query per method = 30× shorter queries),
   but is a larger refactor.
3. **`decompose_element` (#9, 4.1 s)** — non-NTT, the bit-tricky
   corner-case mask. Worth checking if the corner-case branch could
   factor into a sub-lemma.
4. **vs prior snapshot:** `ntt_at_layer_3_` stable at 68 s; new entry
   `invert_ntt_at_layer_3_` at 71 s (now top culprit). Otherwise no
   regressions — improvements would only show after layer-3
   refactor.

<!-- Append future snapshots above this comment. -->

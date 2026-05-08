# Bridge-cascade refactor experiment status

Started: 2026-05-07 (epoch 1778173735)

## Plan

Refactor 3 bridge lemmas in `polynomial.rs` to eliminate Z3 quantifier-instantiation cascade
(`k!63` fires 624,588 times in baseline query 60 of `generate_key_pair`).

## Approach

Replace nested `Classical.forall_intro` chains with dual-SMTPat lookup + a single outer
forall_intro per bridge. Specifically:

- `lemma_lane_range_pos_to_bounded_poly` (2 nested forall_intros currently):
  Add per-`(p, j)` SMTPat helper `lemma_lane_range_pos_to_i32b_array_at` — fires on
  `is_lane_range_poly` ∧ `Seq.index p.f_simd_units j`. Then the bridge is one
  `Classical.forall_intro` over `j`.

- `lemma_lane_range_pos_to_bounded_poly_slice` (1 forall_intro currently, but each
  iteration fires the now-cleaner inner bridge): unchanged structurally.

- `lemma_lane_range_pos_to_pos_array_slice` (1 `forall_intro_2` + 1 inner forall_intro):
  Replace inner forall by re-using the per-`(p, j)` helper which already gives the
  per-lane facts; then call `lemma_is_pos_array_intro` directly. Single
  `forall_intro_2`.

## Iterations

### Iteration 0 — baseline

- HEAD: 9b5b75b4b
- Tree clean.
- baseline k!63 = 624,588 (from prior profiling, not re-measured here).

### Iteration 1 — DONE; result negative

Refactored 3 bridge lemmas in `polynomial.rs` to replace nested
`Classical.forall_intro` chains with `assert (is_i32b_array...)` /
`assert (is_pos_array...)` that exploit `lemma_is_lane_range_poly_lookup`'s
SMTPat to discharge the inner per-m universal. New helper
`lemma_lane_range_pos_to_i32b_array_at` factored out (no SMTPat — invoked
explicitly from inside the j-forall_intro of the `*_to_bounded_poly` bridge).

#### Results

- `Polynomial.Spec.fst` itself verifies clean in 5 sec (the new lemmas all
  succeed; total rlimit < 1 each).
- `Ml_dsa_44_.fst`: query 60 of `generate_key_pair` STILL TIMES OUT at
  rlimit 400, walltime 122-147 sec. ALL OTHER queries pass. Same failure
  mode as baseline.
- Standalone Z3 profile of the canceled query 60 only (rlimit 400M):
  k!63 = ~1.14M instances (vs user-reported baseline 624K). However,
  k!63 is an anonymous quantifier — its label is position-dependent and
  changes across SMT2-file structure refactors, so the absolute number
  isn't directly comparable.

#### Conclusion: FAILED

The F*/Z3 timing of the failing query did not improve. The refactor
shifted where the universal-instantiation work happens (from an explicit
`Classical.forall_intro` over m to an `is_i32b_array`-body unfold +
SMTPat), but the underlying cascade — driven by
`lemma_is_lane_range_poly_lookup`'s SMTPat firing on `Seq.index lane_arr m`
across many polynomials × many simd_units — is unchanged.

A more invasive fix would be to remove that SMTPat entirely and provide a
different instantiation handle, but that risks breaking many existing
call sites and is well beyond the experiment budget.

Reverting all changes.


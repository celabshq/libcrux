# Sprint 4 — `compute_w_approx` body proof — CLOSED

Started: 2026-05-05
Closed: 2026-05-06

## Outcome

`make check/Libcrux_ml_dsa.Matrix.fst` exit 0 ("Verified module:
Libcrux_ml_dsa.Matrix"); `cargo test --release --lib` 20 passed.
`compute_w_approx` body proof closes with **no admit**.

## Path that worked

The body proof closed via four stacked changes, in order:

1. **Wrap heavy 3-deep ∀ in opaque pred.**  Added `is_lane_range_poly`
   and `is_lane_range_poly_slice` (with intro/lookup lemmas) to
   `polynomial.rs::spec`.  These wrap the per-lane T1-decoded
   `forall j m. v(...) ∈ [0, 261631]` shape that previously fired
   `Tm_refine_e3c40b65` ~325k times in a single failing query.

2. **Replace bare ∀s in `compute_w_approx` pre/inv with opaque preds.**
   The function pre's 3-deep ∀ on `t1` and the outer/inner inv's `old_t1`
   ∀ both became `is_lane_range_poly_slice (mk_usize 0) (mk_usize 261631)`.
   The outer carryover `forall k < v $i. is_bounded_poly 4_211_177 t1[k]`
   became `is_bounded_poly_range (mk_usize 4211177) (mk_usize 0) $i $t1`
   (existing opaque pred).

3. **Factor the long sequential blob into `compose_w_approx_per_row`.**
   The 6+ sequential `update_at_usize` ops on `t1[i]` (shift_left →
   ntt → ntt_mul → subtract → assign → reduce → invert) now happen
   inside a helper that takes a single `&mut PolynomialRingElement`.
   This collapses the outer fold body's `Seq.upd t1 i ...` chain to
   ONE update on `t1`, dramatically shrinking the WP and removing the
   compounding per-update re-typing pressure.

4. **Iter-start snapshot + standalone bridge lemma.**  The aux lemma
   that re-establishes `is_bounded_poly_range 4_211_177 0 (i+1) t1`
   at iter end was timing out at `rlimit 800` on **trivial assertions**
   (`k = v $i`, `Seq.index t1 k == Seq.index t1 (v $i)`) because of
   cascade pollution from compute_w_approx's heavy ambient context.
   Fix: snapshot `iter_start_t1` at iter start, and extract the
   bridge into a **standalone lemma**
   `lemma_is_bounded_poly_range_extend_after_update` in `polynomial.rs`.
   The lemma is verified in its own clean context — Z3 has no
   compute_w_approx state to wade through, so the trivial assertions
   close in milliseconds.  compute_w_approx then just calls the lemma
   once per outer iter.

## Diagnostic notes — what didn't work

Iterating with `--log_queries --z3refresh` and `smt.qi.profile=true`
on the dumped `.smt2` was load-bearing:

- **Bumping rlimit to 800 (with split_queries)**: same 2 errors.  Z3
  hits the new ceiling exactly.  Diagnosis: not budget-bound.
- **Dropping `--split_queries always`**: F* retries-with-split, all
  175 sub-queries pass logically, but F*/Z3 IPC crashes after 176
  queries ("Parse error: </labels> not found").  Proof closes in
  spirit but `.checked` not written.  Confirmed proof was provable.
- **Hypothesis test — assume_val'd `ntt_multiply_montgomery_local`**
  with tighter post (no lane forall): same 2 errors, **disproved**
  the hypothesis that ntt's lane forall was the cascade source.
  Per the user's principle ("don't touch verified code without
  proving the pattern closes a hard lemma"), confirmed
  justification to NOT modify `ntt.rs`.
- **Inline aux + assert (k = v $i)**: trivial assertion timed out
  at rlimit 800 due to cascade pollution.  Triggered the move to
  standalone lemma.

## Files changed

- **`libcrux-ml-dsa/src/polynomial.rs`**:
  - Added `is_lane_range_poly` + `is_lane_range_poly_slice` opaque
    preds (with intro/lookup lemmas) — 2-deep and 3-deep T1-decoded
    range predicates.
  - Added `lemma_is_bounded_poly_range_extend_after_update` (via
    `hax_lib::fstar::after`) — standalone lemma to extend an opaque
    `is_bounded_poly_range` carryover by one more index after a
    Seq.upd at the new index.
- **`libcrux-ml-dsa/src/matrix.rs`**:
  - Pre/inv use `is_lane_range_poly_slice 0 261631 ...` for
    T1-decoded slices and `is_bounded_poly_range 4_211_177 0 i ...`
    for the carryover.
  - Added `compose_w_approx_per_row` helper for the per-row
    sequential blob.  Single `&mut PolynomialRingElement`, not a
    slice.
  - `compute_w_approx` body: snapshots `iter_start_t1`; calls the
    helper once per outer iter; calls
    `lemma_is_bounded_poly_range_extend_after_update` to close the
    inv re-establishment.
  - Removed the prior `hax_lib::fstar!("admit ()")`.
- **`libcrux-ml-dsa/proofs/fstar/extraction/Libcrux_ml_dsa.Matrix.fst`**:
  - Post-extraction typeclass patch (rename inner-fold's destructured
    `result` → `out`, add `(out <: t_Slice ...)` ascription) is
    unchanged from prior cycles.  Must be reapplied after every
    `./hax.sh extract` until codified in `hax.sh`.

## Reused infrastructure

- `is_bounded_poly_range` (existing opaque pred + intro/lookup lemmas
  in `polynomial.rs::spec`).
- `add_to_ring_element` / `subtract_to_ring_element` (already-bound-aware
  add/subtract wrappers in `matrix.rs`).
- The `compute_as1_plus_s2` helper-split pattern (Sprint 3.5) — same
  shape applied to a different axis (sequential-blob factoring vs
  two-fold split).

## Sprint A unblocked

Three Sprint A targets are now unblocked:

- `compute_as1_plus_s2` body proof (Sprint 3.5) — already done.
- `compute_matrix_x_mask` body proof — already done.
- `compute_w_approx` body proof — **this sprint**.

These three are the matrix-level cores for `generate_key_pair`,
`sign_internal`, and `verify_internal` panic-free flips.

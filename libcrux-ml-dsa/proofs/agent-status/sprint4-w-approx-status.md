# Sprint 4 — `compute_w_approx` body proof — WIP, 99% closed

Started: 2026-05-05
Paused: 2026-05-06 (well over 60-min per-fn debug budget; deferring per
`feedback_proof_debug_budget`)

## Outcome

`make check/Libcrux_ml_dsa.Matrix.fst` exit 0 with `hax_lib::fstar!("admit
()")` at the top of `compute_w_approx`'s body.  `cargo test --release
--lib` 20 passed.

The body proof is **drafted in full** in `src/matrix.rs` — pre/post
upgraded to `is_bounded_poly`/`is_bounded_poly_slice` form, outer +
inner loop invariants, all bridge lemmas in place.  Under
`--admit_except "Libcrux_ml_dsa.Matrix.compute_w_approx"`, the proof
discharges **207 / 209** sub-queries.  The two stragglers are post-body
bridges that re-establish the outer carryover; both hit the WP-collapse
pattern documented in `SKILL.md` §7.

## What's complete

### `src/matrix.rs` (committed)

- `add_to_ring_element` (pre-existing, unchanged).
- **NEW** `subtract_to_ring_element` — `subtract` analog mirroring the
  `add_to_ring_element` recipe; lifts `subtract_bounded`'s per-lane post
  to `is_bounded_poly` form.  `rhs` fixed at `FIELD_MAX = 8380416`.
- `compute_w_approx` body proof drafted with full annotations:
  - Pre upgraded to `is_bounded_poly_slice FIELD_MAX matrix /\
    is_bounded_poly_slice FIELD_MAX signer_response /\ is_bounded_poly
    FIELD_MAX verifier_challenge_as_ntt`, retaining the per-lane
    non-negative pre on `t1` (needed by `shift_left_then_reduce`).
  - Post upgraded to `is_bounded_poly_slice 4_211_177 t1_future`.
  - `#[cfg(hax)] let old_t1 = t1.to_vec().as_slice()` snapshot.
  - Outer loop_invariant: carryover `forall k < v $i.
    is_bounded_poly 4_211_177 t1[k]` + frame `forall k. v $i <= k ==>
    t1[k] == old_t1[k]` + non-neg pre on `old_t1` (3-deep forall).
  - Inner loop_invariant: `is_bounded_poly (j *! FIELD_MAX) inner_result`
    + carryover/frame on t1.
  - Body: `add_to_ring_element` and `subtract_to_ring_element`
    replacements; per-step bridge lemmas; `lemma_is_bounded_poly_higher`
    for reduce's pre; `lemma_is_bounded_poly_intro` for shift_left's
    per-lane post.
  - Final bridge from `is_bounded_poly_range 4_211_177 0 rows t1` to
    `is_bounded_poly_slice` via `Classical.forall_intro`.
- `hax_lib::fstar!("admit ()")` placed at body top with a comment
  pointing to this status file.

### `.fst-direct` experimental fixes (NOT in source — reverted on
re-extract)

These were validated via `make OTHERFLAGS='--admit_except ...'` but not
yet ported back to Rust:

1. **Opaque pred `is_t1_decoded_slice`** — wraps the 3-deep forall on
   the per-lane non-negative pre.  Reduces context-pollution noise in
   the loop_invariant from a triple-forall to a single opaque atom.
   With SMTPat lookup lemma + extract lemma.  This SOLVED the bulk of
   the proof — ~200 queries went from "fails at rlimit 400 within 100s"
   to "succeeds within 100ms".  Belongs in `Polynomial.Spec.fst` (as
   `is_bounded_poly_slice`'s analog), or in the matrix.rs file via
   `#[hax_lib::fstar::before(...)]` injection.

2. **Refinement-typed bound aliases** — `_shift_by`, `_fm`, `_cols_fm`,
   `_cols_plus_one_fm`, `_reduce_max`, `_post_inv_bound`, hoisted
   *outside* the outer fold with explicit `(x: usize {v x = ...})`
   refinements.  This unblocks Z3 on the trivial integer overflow
   checks (`columns_in_a *! mk_usize 8380416`) that were timing out at
   rlimit 400 inside the body.

## What's blocking

The two timing-out post-body assertions are:

```fstar
assert (forall (k: nat). k < v i + 1 ==>
  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 4211177)
    (Seq.index t1 k));
assert (forall (k: nat). v i + 1 <= k /\ k < Seq.length t1 ==>
  Seq.index t1 k == Seq.index old_t1 k)
```

Both require Z3 to chase frame propagation through ~6 `update_at_usize`
calls on index `v i`.  Conceptually trivial; in practice Z3 spins for
>100s on each.

**Why this is the WP-collapse pattern:** the outer-fold body has a
long sequential chain of mutations (shift_left → ntt → ntt_mul →
subtract → assignment → reduce → invert_ntt) plus an inner fold.  At
the function exit's WP, F* has to compose all of these.  Per `SKILL.md`
§7, sequential mutating calls in a fold collapse the function-level WP.
The fix is to split the function (the same recipe that closed
`compute_as1_plus_s2`).

## Recommended path forward

**Option A (recommended): split `compute_w_approx` per the SKILL.md
recipe.**  Mirroring the `compute_as1_plus_s2` split:

- `matrix_x_signer_inner_dot` (private, per-row): one inner fold over
  `0..cols`, computes `inner_result` for given `i`.  Pure NTT-domain
  accumulation; no t1 mutation.  Returns `inner_result`.
- `compose_w_approx_per_row` (private, per-row): takes `inner_result` +
  matrix index, mutates `t1[i]` through the
  shift_left/ntt/ntt_mul/subtract/reduce/invert chain.  Single-mutable
  on t1 like `compute_matrix_x_mask`.
- `compute_w_approx` (public): outer fold dispatches to the two
  helpers per row.

Each helper has at most one fold + sequential body, eliminating the
WP-collapse trigger.

**Option B: port the `.fst-direct` opaque-pred + refinement-typed bounds
back to Rust source via `hax_lib::fstar::before/after`.**  This won't
fix the WP collapse but might still get the proof through if combined
with explicit `Classical.forall_intro` chains for the post-body bridges.
Higher risk, less alignment with the SKILL.md prescribed recipe.

## Files changed (committed)

- `src/matrix.rs`:
  - Added `subtract_to_ring_element` wrapper.
  - Replaced `compute_w_approx` body with full proof annotations +
    `admit ()` at the top.
  - Pre upgraded; post upgraded to `is_bounded_poly_slice 4_211_177`.
- `proofs/agent-status/sprint4-w-approx-status.md`: this file.

The post-extraction typeclass patch (`result` → `out` rename + ascription
in `ntt_dot_accumulate`'s inner-fold lambda) must still be reapplied
after every `./hax.sh extract` until codified in `hax.sh`.  Per
sprint3.5 retro this is a TODO.

## Time

- Sprint 4 prompt ~5h elapsed.
- Direct .fst iteration unblocked from "n errors" → "2 errors" with the
  opaque-pred refactor and refinement-typed bound aliases.
- Final 2-error structural blocker (WP collapse) is the SKILL.md
  prescribed split-the-function pattern.

## TODOs

- [ ] Split `compute_w_approx` per SKILL.md §7 (Option A).  Tracked.
- [ ] Port `is_t1_decoded_slice` opaque pred + lookup/intro lemmas to
      `Polynomial.Spec.fst` via Rust `#[hax_lib::fstar::*]` injections.
- [ ] Codify the post-extraction typeclass patch
      (`result`→`out` rename + ascription) as a sed step in `hax.sh`.

# Sprint 3.5 follow-up — `compute_as1_plus_s2` body-proof — DONE

Started: 2026-05-05 16:10
Closed: 2026-05-05 ~21:30

## Outcome

`make check/Libcrux_ml_dsa.Matrix.fst` exit 0 (~4.5 min cold), `cargo
test --release --lib` 20 passed.  Body proof of
`compute_as1_plus_s2` closes.

## What it took

Three independent issues were stacked on top of each other; the
original Sprint 2 commit message attributed the failure to "hax
extraction" alone.

1. **F\* typeclass resolution failure on `result.[i]`** inside hax's
   tuple-state destructured lambda.  This is a hax/F\* interaction
   that fires whenever a Rust loop body mutates two `&mut` slices —
   hax extracts a `(s1, s2)` tuple-state fold, the lambda destructures
   it, and F\* can't resolve the `Index` typeclass on the second
   slice's `.[i]` access in that scope.  Fixes that worked: rename the
   inner-fold's `result` binder to `out` AND add an explicit
   `(out <: t_Slice (...))` ascription on the indexing operation.
   Currently a post-extraction patch on the .fst file (hax.sh sed
   not yet codified — TODO).

2. **SMT cascade** from the transparent `is_bounded_poly_range` /
   `is_bounded_poly_slice` predicates.  Closed in Sprint 3.5 (commit
   `5d538df99`) by making both opaque with dual-SMTPat lookup lemmas.

3. **Function-level WP composition through two sequential folds with
   let-shadowed mutable parameters.**  Even after (1) and (2), every
   individual loop invariant and bridging lemma discharged cleanly,
   yet the function-level postcondition would fail with "incomplete
   quantifiers" using <2 of 400 rlimit.  Adding `assert`s inside the
   body for each post conjunct succeeded individually, but the
   conjunction at the function exit failed.  `assume False` near the
   return passed.  This pointed to an F\* WP composition bug or
   trigger pathology when sequentially-composed folds shadow input
   parameters with their let-bound locals.

## The fix that worked

**Split `compute_as1_plus_s2` into two helper functions** so each
helper has a single fold:
- `ntt_dot_accumulate` (private): does the NTT-domain dot product
  (mutates `a_as_ntt` and `result`, returns tuple).  Post:
  `is_bounded_poly_slice (cols*FM) result`.
- `inv_ntt_and_add_s2` (private): does the per-row Barrett reduce →
  InvNTT → add s2 (single-mutable, mirrors `compute_matrix_x_mask`'s
  shape exactly).  Pre: `is_bounded_poly_slice (cols*FM) result`.
  Post: `is_bounded_poly_slice 16760832 result_future`.
- `compute_as1_plus_s2` (public, top-level): just calls the two
  helpers in sequence, with a slice-projection bridge for `s1_s2[cols..]`.

Function-call boundaries cleanly isolate each fold's WP.  The
problematic WP composition disappeared.

## Other Rust source-level changes

- Added precondition `is_bounded_poly_slice (mk_usize 0) $result` to
  `compute_as1_plus_s2`.  The only call site (`ml_dsa_generic.rs:111`)
  passes a freshly-allocated `[zero(); ROWS_IN_A]`, so this discharges
  trivially at the call site.
- Replaced `PolynomialRingElement::add(&mut a, &b)` calls with
  `add_to_ring_element(&mut a, &b, bound)` (Sprint 3's bound-aware
  wrapper).  The wrapper internalizes the per-lane add-precondition
  discharge.  Two sites: inner loop of `ntt_dot_accumulate`, body
  of `inv_ntt_and_add_s2`.

## Bridge lemma pattern (libcrux-wide reusable)

To bridge between the opaque `is_bounded_poly_slice` form and the
opaque `is_bounded_poly_range` form (or the `forall k. is_bounded_poly`
universal form), Z3's e-matching of the dual-SMTPat through a
universal hypothesis is unreliable.  Use `Classical.forall_intro` of
an explicit aux helper:

```fstar
let _:Prims.unit =
  let aux (k: nat{k < Seq.length result}) :
    Lemma (is_bounded_poly b (Seq.index result k)) =
    lemma_is_bounded_poly_slice_lookup b result k
  in
  Classical.forall_intro aux
in
lemma_is_bounded_poly_range_intro b lo hi result;  // or lemma_is_bounded_poly_slice_intro
```

This works when SMTPat alone doesn't.

## Files changed (commit-ready)

- `src/matrix.rs`: split into 3 fns, added precondition, all bridge
  lemmas as source-level `hax_lib::fstar!()` annotations.
- `proofs/fstar/extraction/Libcrux_ml_dsa.Matrix.fst`: typeclass
  patch (rename + ascribe) — applied post-extraction; TODO codify in
  `hax.sh`.

## TODO

- Codify the typeclass post-extraction patch in `hax.sh` (sed/patch).
- Consider upstreaming the typeclass workaround to hax (avoid the
  destructured-tuple `Index` resolution failure entirely).

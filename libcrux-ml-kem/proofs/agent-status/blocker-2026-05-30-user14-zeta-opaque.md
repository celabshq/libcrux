# USER-14 — zeta-opacity blocker RESOLVED via Option B (user-approved axiom); post-chain verified

**Date:** 2026-05-30 · worktree `/Users/karthik/libcrux-user14-bridge` · branch `agent/user-14-layer4-bridge`
**Status:** the table-form post is now *provably dischargeable*. Axiom + unfold lemma + end-to-end
chain test all machine-verified (full `fstar_build` of `Bridges.fst`, exit 0). Only the loop-invariant
accumulation of `cross_vec_hyp` remains.

## Original blocker (for the record)
`invert_ntt_at_layer_4_plus`'s post cites the table-building `IN.ntt_inverse_layer (...) layer`
(concrete `v_ZETAS`), but `Libcrux_ml_kem.Polynomial.zeta` is exposed cross-module as an `assume val`
with a BOUND-ONLY ensures (`result ∈ [-1664,1664]`; `polynomial.rs:29`). The high-half butterfly
`._2 = z·(b−a)` forces `mont_i16_to_spec_fe (zeta k) == v_ZETAS[k]`, unprovable from the bound alone.
(Low-half `._1 = a+b` needs no zeta. Only k∈1..15 are used.)

## Resolution: Option B (user-approved 2026-05-30) — assumed correspondence axiom
Added to `Bridges.fst` (verified full build, exit 0):

1. `lemma_zeta_eq_vzetas (k:usize) : Lemma (requires v k < 128)
      (ensures mont_i16_to_spec_fe (Poly.zeta k) == N.v_ZETAS.[k])` — **`assume val`**.
   True; runtime-validated by `ntt_matches_spec` / `full_ntt_multiply_chain_matches_spec` in `src/ntt.rs`.
   This is trust debt, explicitly approved. It ALSO unblocks USER-15.

2. `lemma_ntt_inverse_layer_unfold p layer zs` — structural unfold of the table form to
   `IN.ntt_inverse_layer_n 256 p (2^layer) zs`, for a caller `zs` matching the table.
   Proof: FACT 1 = `norm [delta_only ntt_inverse_layer; iota; zeta; primops]; trefl`;
   FACT 2 = point-wise `tbl_slice == zs` via `createi_lemma` + `FStar.Seq.Base.lemma_index_slice`,
   then congruence. (No zeta correspondence inside — spec-to-self only.)

3. `lemma_layer_4_plus_post_from_cross_vec` — **end-to-end chain test**: assuming
   `forall m l. cross_vec_hyp re_in.f_coefficients re_out.f_coefficients step_vec zs m l`
   (plus `zs[round]==v_ZETAS[2*groups-1-round]`, len/layer/length side-conditions), it proves the
   exact post `to_spec_poly_mont re_out == IN.ntt_inverse_layer (to_spec_poly_mont re_in) layer`
   by composing `lemma_layer_4_plus_cross_vector` → `lemma_ntt_inverse_layer_unfold` →
   `lemma_to_spec_poly_mont_unfold`. **This is precisely the post-loop wiring the body needs.**

## What remains (the laborious body step — not yet done)
Strengthen the double-loop invariants in `invert_ntt_at_layer_4_plus` (Rust `loop_invariant!` +
`fstar!` blocks; extracted as two nested `fold_range`s) to establish, after the loops,
`forall m l. cross_vec_hyp re_init.f_coefficients re_final.f_coefficients (len/16) zs m l`,
where `zs[round] = mont_i16_to_spec_fe (zeta (2*groups-1-round))` (= `v_ZETAS[...]` by the axiom).
Mechanics:
- Each vector m is written exactly once; pairs `(j, j+step_vec)` are disjoint, and reads are of
  iter-start values → snapshot `re_init` (`#[cfg(hax)] let _re_init = re.coefficients;`) and carry,
  per processed vector, its `cross_vec_hyp` relation to `re_init`; for unprocessed m, `re[m]==re_init[m]`.
- Per inner iteration the per-step bridge `lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec`
  (Bridges:1175) gives the (m,l) butterfly facts (low half→`._1`, high half→`._2`); block=round,
  zeta=`zeta_r`. Accumulate into `cross_vec_hyp` via post-loop `Classical.forall_intro` (Rule SD4: no
  global no-arg `reveal_opaque` in the loop body — targeted reveal / post-call assert only).
- Then call `lemma_layer_4_plus_post_from_cross_vec` (or inline its three lemma calls) to finish.
- Drop `--admit_smt_queries true`; re-extract; per-stage clean build of `Invert_ntt.fst` + `Bridges.fst`.

This is the same shape as layers 1-3's post-loop `forall_intro` (invert_ntt.rs:111-142) but cross-vector
with an iter-start snapshot (see skill "iter-start snapshot + standalone bridge lemma"); a standalone
carryover-extension lemma (like ml-dsa's `lemma_is_bounded_poly_range_extend_after_update`) will likely
be needed for the opaque `cross_vec_hyp` carry across `Seq.upd`.

## Verified-build evidence
- `lemma_ntt_inverse_layer_unfold`: `fstar_build --admit_except` exit 0.
- `lemma_layer_4_plus_post_from_cross_vec`: `fstar_build --admit_except` exit 0.
- Full `check/Hacspec_ml_kem.Commute.Bridges.fst` (no admit): exit 0, 0 failed modules, ~87s.

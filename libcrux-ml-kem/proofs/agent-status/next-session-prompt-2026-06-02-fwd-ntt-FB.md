# Next session — Forward NTT: close F-B (layer-4+ keystone) + discharge assumes

Worktree `/Users/karthik/libcrux-fwd-ntt`, branch `agent/fwd-ntt-mirror-inverse` (off 92de78a50).
Read `RESUME-fwd-ntt-2026-06-02.md` first (architecture + gotchas + per-lemma status).
Use the fstar-for-libcrux skill. hax 0.3.7 at ~/.opam/hax-0.3.7/bin.

STATE: `ntt_vector_u` proves `to_spec_poly_plain re_future == Hacspec_ml_kem.Ntt.ntt
(to_spec_poly_plain re)`; full Libcrux_ml_kem.Ntt.fst GREEN; durable in src/ntt.rs
(re-extract reproduces). Committed a1d82181a + beeac1ee2. Real-verified: Ntt_bridge.fst
mont→plain per_coeff, layers 1-3 bridges, compose_7, F-A wiring.

DO (in priority order):
1. F-B: replace the 4 `assume (Ntt_bridge.poly_step ... (mk_usize {7,6,5,4}))` in
   ntt_vector_u with a real `lemma_layer_4_plus_to_poly_step` in Ntt_bridge.fst —
   forward cross-vector keystone, MIRROR `Bridges.fst` USER-14
   (cross_vec_hyp / lemma_layer_4_plus_per_coeff / lemma_cross_vec_from_step /
   lemma_layer_4_plus_post_from_cross_vec) for the Cooley-Tukey butterfly + PLAIN
   output (per-coeff mont→plain via the Ntt_bridge Section-1 cores). Then strengthen
   ntt_at_layer_4_plus's ensures to the poly post + wire cross_vec_hyp in its loop body.
2. Restore the 2 TEMP-ADMIT-UNFOLD bodies in Ntt_bridge.fst (squash+norm@rlimit400; verified
   once, slow ~33-130s). Keep `primops` in the norm; do NOT add --using_facts_from on the unfold.
3. Discharge barrett-value assume: strengthen impl__poly_barrett_reduce's post (Polynomial.fst,
   shared module) to give barrett_reduce_post per chunk, then
   Chunk.lemma_poly_barrett_reduce_commute + lemma_poly_barrett_reduce_id.
4. Discharge bound assume: factor is_bounded_poly_higher(re,8*3328,28296) into a clean-context
   helper (the inline call saturates under the post-layer quantifier pollution).
5. Discharge abs(-1600)==1600 in ntt_at_layer_7_ (needs a Core_models.Num.impl_i16__abs lemma).
6. Re-extract, full build, confirm ntt module 1 lax -> 0, refresh verification_status.md.

BUILD: per_coeff ~136s, layer1 bridge ~138s, layers rlimit-800 each; full Ntt.fst ~5-7 min.
Use --admit_except per-lemma for iteration. Kill orphan builds by PID. Max 4 own fstar/z3.

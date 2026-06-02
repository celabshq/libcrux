# RESUME — Forward NTT closure (2026-06-02)

Worktree `/Users/karthik/libcrux-fwd-ntt`, branch `agent/fwd-ntt-mirror-inverse` off `92de78a50`.
Cache seeded from `/Users/karthik/libcrux-ml-kem-proofs` (warm, 92de78a50). hax 0.3.7 at
`~/.opam/hax-0.3.7/bin/cargo-hax` works → can re-extract via `libcrux-ml-kem/hax.py`.

Goal: close `ntt_vector_u` admit + `ntt_at_layer_7_` abs assume → ntt module 1 lax → 0.
DoD: `check/Libcrux_ml_kem.Ntt.fst` green, `to_spec_poly_plain re_future == Hacspec_ml_kem.Ntt.ntt (to_spec_poly_plain re)`.

## Architecture (DECIDED, validated)
Forward layer posts are MONT (`mont_i16_to_spec_array`); driver post is PLAIN (`to_spec_poly_plain`).
They differ by a 169-scaling (`mont_i16_to_spec_fe x = (x*169)%q`, `i16_to_spec_fe x = x%q`).
Reconcile mont→plain at the PER-COEFFICIENT level (butterfly homogeneity + 169-cancellation,
169⁻¹=2285), so the bridge composes in PLAIN form directly to the driver post — NO global
`ntt` homogeneity needed. All forward material in NEW file
`specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Ntt_bridge.fst` (don't touch
Bridges/Chunk/Invert_ntt_bridge .checked). Reuse from Bridges via `open`: lemma_cross_idx,
lemma_partner_idx_*, lemma_div_128_prod, lemma_zeta_eq_vzetas; from Chunk: to_spec_poly_plain[_arr],
mont_array_lane, zetas_{1,2,4}_lane, lemma_mont_* mod cores.

## Ntt_bridge.fst status (per-lemma, validated in isolation via --admit_except)
- Section 1 (lemma_cancel_169, lemma_bf_*_core, lemma_mont_to_plain_butterfly_{plus,minus}): VERIFY.
- Section 2 helpers (lemma_intra_*, tspp_arr_lane, lemma_ntt_layer_n_16_lane, lemma_ntt_layer_n_256_compose): VERIFY.
- lemma_ntt_layer_unfold_lo / lemma_ntt_layer_unfold: **TEMP-ADMITTED** (search "TEMP-ADMIT-UNFOLD").
  Verified squash+norm body at rlimit 400 (lo 33s; hi needs 400). Performance tar-pit: the
  createi-extensionality congruence `ntt_layer_n A == ntt_layer_n B` from `A==B` costs 30-130s,
  and `norm [...; primops]; trefl` REQUIRES primops (without it the tactic diverges). RESTORE the
  squash+norm bodies (in git history of this file / see earlier edits) before final, OR find a
  fast congruence (the `--using_facts_from '* -ntt_layer_n'` exclusion broke ntt_layer_n's
  precondition discharge AND ntt_layer's unfold — do NOT exclude on the unfold itself).
- pv_post / pv_post_intro / pv_post_elim (MONT, matches layer posts): VERIFY.
- lemma_intra_vec_per_coeff (MONT pv_post → PLAIN per-coeff butterfly, the novel core): VERIFY (136s).
- lemma_intra_vec_layer_to_poly (chainer → plain poly_step): expected verify (calls per_coeff+256_compose+unfold_lo).
- poly_step / lemma_poly_step_intro (PLAIN): VERIFY.
- lemma_compose_7 (7 plain poly_steps, layer order 7..1 → N.ntt): expected verify.
- lemma_layer1_to_poly_step: VERIFY (138s). layer2/3 expected verify (smaller).

## REMAINING WORK
F-A wiring: edit `src/ntt.rs` ntt_at_layer_1/2/3 — change ensures forall `mont_i16_to_spec_array ...`
  to `Hacspec_ml_kem.Commute.Ntt_bridge.pv_post #$:Vector ${re}.f_coefficients ${re}_future.f_coefficients
  (mk_usize LEN) (${zetas_N} ...) (v i)`; in the post-loop aux, change aux's Lemma type to that pv_post
  and APPEND `Hacspec_ml_kem.Commute.Ntt_bridge.pv_post_intro #v_Vector ${_re_init} re.f_coefficients
  (mk_usize LEN) (${zetas_N} ...) j` after `lemma_ntt_layer_N_step_to_hacspec`. (LEN: layer1=2,2=4,3=8.)
  Re-extract, verify Ntt.fst layer fns.
F-B: ntt_at_layer_4_plus poly post (HARD, mirror USER-14 Bridges keystone for butterfly+plain):
  forward cross_vec_hyp (MONT, butterfly), forward lemma_layer_4_plus_per_coeff (mont→plain like
  per_coeff), forward keystone (lemma_cross_vec_from_step etc.), post lemma producing
  `to_spec_poly_plain re_out == N.ntt_layer (to_spec_poly_plain re_in) layer`. Add these to Ntt_bridge.fst.
  Then strengthen ntt_at_layer_4_plus ensures (ntt.rs) + body wiring. Discharge `abs(-1600)==1600` assume
  in ntt_at_layer_7_ (Core_models.Num.impl_i16__abs — likely `assert_norm` or a Core_models lemma).
F-C: ntt_vector_u — #cfg(hax) re0..re7 snapshots; layer_4_plus(7..4) → lemma_poly_step_intro re_{k} re_{k+1} {7,6,5,4};
  layer_3/2/1 → lemma_layer{3,2,1}_to_poly_step; then lemma_compose_7 re0..re7; barrett_reduce is plain-identity
  (lemma_poly_barrett_reduce_id in Chunk) so post survives it. Remove `--admit_smt_queries true`. Re-extract.

## Build discipline
make-based builds are SLOW (per_coeff 136s, layer1 138s, unfolds 33-130s). Use --admit_except per-lemma
for iteration. Full Ntt_bridge build ~6-8 min. Native fstar_build caps at 60s → use curl /tmp/fp.sh
(max-time 600) or launch with wait_secs:30 and poll fstar_build_status(build_id). Kill orphan builds
by PID (never pkill by name — other agents share host). Max 4 own fstar/z3.

## UPDATE (08:45) — F-C driver composition VERIFIED (isolated)
`ntt_vector_u` verifies via `--admit_except` (42s): the post
`to_spec_poly_plain re_future == N.ntt (to_spec_poly_plain re) /\ is_bounded_poly 3328 re_future`
is PROVEN by the composition (compose_7 + 7 poly_steps). Driver structure (in extracted
Libcrux_ml_kem.Ntt.fst, backed up as agent-status/Libcrux_ml_kem.Ntt.fst.FWD-FC-driver-green-20260602):
re0=re; 7 layers capturing re1..re7; bounds-widen+barrett; 7 poly_step lemmas
(layer_4_plus stub for 7/6/5/4, lemma_layer{3,2,1}_to_poly_step for 3/2/1); lemma_compose_7.
push-options `--z3rlimit 400 --split_queries always` (NO context_pruning — it pruned numeric facts).
GOTCHA: `mk_usize 28296` range-check SATURATES under the 7 layer-post quantifier context
(cascade pollution) — pre-bind `let bnd28296 = mk_usize 28296` at function start (clean context).

## REMAINING TEMP-ADMITS (the honest gap list)
In Ntt_bridge.fst: 2 unfold lemmas (TEMP-ADMIT-UNFOLD; provable squash+norm@rlimit400, slow).
In Libcrux_ml_kem.Ntt.fst ntt_vector_u:
  - lemma_layer_4_plus_to_poly_step_TEMP (admit): **F-B** = the real remaining proof work
    (forward cross-vector keystone, mirror USER-14 Bridges cross_vec_hyp/per_coeff/keystone for
    butterfly+plain). Layers 4-7 poly_step.
  - barrett-value assume (`to_spec_poly_plain re_b == to_spec_poly_plain re7`): impl__poly_barrett_reduce
    post is bounds-only; real fix strengthens it to barrett_reduce_post per chunk (shared Polynomial
    module change) then Chunk.lemma_poly_barrett_reduce_commute + _id.
  - bound assume (is_bounded_poly 28296 re): pollution-blocked is_bounded_poly_higher; provable, factor.
Plus: discharge abs(-1600)==1600 assume in ntt_at_layer_7_ (Core_models.Num.impl_i16__abs; concrete).
Plus: backport extracted Ntt.fst/.fsti edits (F-A pv_post ensures+intro, F-C driver) to src/ntt.rs + re-extract.

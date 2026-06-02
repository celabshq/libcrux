# Forward NTT closure — design note (2026-06-01)

Worktree: /Users/karthik/libcrux-fwd-ntt (branch agent/fwd-ntt-mirror-inverse, off 92de78a50).
Goal: close `ntt_vector_u` admit + `ntt_at_layer_7_` abs assume → ntt module 1 lax → 0.

## Key reconciliation insight (mont vs plain)
- Forward layer posts (ntt_at_layer_1/2/3 + the layer_4+ keystone) are stated in
  `mont_i16_to_spec_array` / `to_spec_poly_mont` (de-Montgomery, `*169`).
- Driver post is `to_spec_poly_plain re_future == N.ntt (to_spec_poly_plain re)` (plain, `x mod q`).
- `i16_to_spec_fe x = x%q`; `mont_i16_to_spec_fe x = (x*169)%q`. So mont = scale-by-169 of plain.
- Butterfly `(z,a,b)↦(a+z·b, a−z·b)` is homogeneous in (a,b) for fixed z:
  `butterfly z (169•a) (169•b) = 169 • butterfly z a b`. 169 invertible (169·2285≡1 mod 3329).

## Architecture (avoids nested-createi ntt homogeneity)
Build `Hacspec_ml_kem.Commute.Ntt_bridge.fst` entirely in `to_spec_poly_plain`:
- `pv_post` MONT (matches what layers 1-3 prove via lemma_ntt_layer_N_step_to_hacspec).
- `per_coeff` converts mont per-vector → PLAIN per-coefficient butterfly relations, doing the
  169-cancellation PER ELEMENT (clean modular arith; no createi homogeneity).
- forward `lemma_ntt_layer_n_256_compose` (plain) + `lemma_ntt_layer_unfold*` → plain poly_step.
- `poly_step` PLAIN; `compose_7` → `to_spec_poly_plain re7 == N.ntt (to_spec_poly_plain re0)`.
- layer_4_plus keystone: mirror USER-14 `cross_vec_hyp` (mont) but per_coeff emits plain.

All forward material lives in Ntt_bridge.fst (don't touch Bridges/Chunk/Invert_ntt_bridge .checked).
Reuse direction-agnostic helpers from Bridges via `open`: lemma_cross_idx, lemma_partner_idx_*,
lemma_div_128_prod, lemma_zeta_eq_vzetas; and from Chunk: tspp via to_spec_poly_plain_arr,
mont_array_lane, lane_plain, zetas_{1,2,4}_lane, lemma_mont_* mod cores.

## Phases
F-A layers 1-3 bridge + wire ntt_at_layer_1/2/3 ensures→pv_post.
F-B layer_4_plus plain poly post via forward keystone; discharge abs(-1600)==1600 assume.
F-C driver ntt_vector_u: snapshot re0..re7, poly_step intros, compose_7; drop admit.

## Progress log (2026-06-02)
- Ntt_bridge.fst Sections 1-6 written. Section 1+2 (reconciliation + generic helpers) verify.
- KEY GOTCHA: forward ntt_layer slices v_ZETAS DIRECTLY (`v_ZETAS.[{groups,2groups}]`), and
  ntt_layer_n has a STRICT precondition `2*len(zetas)*len==N` (inverse ntt_inverse_layer_n
  lacks it). The unfold lemma must (a) NOT churn on the nonlinear precondition (use a
  case-split `squash` to ground it) and (b) NOT pay the createi-extensionality congruence
  cost (`ntt_layer_n A == ntt_layer_n B` from A==B). Fix: norm/trefl FACT1 (syntactic
  ntt_layer unfold) + `lemma_ntt_layer_n_cong` (excludes ntt_layer_n's def -> uninterpreted,
  congruence in ms) + transitivity. Earlier attempts: direct precondition assert churned
  900s; whole-lemma exclusion churned 11min (broke ntt_layer unfold).
- Sections 3-6 (pv_post mont, per_coeff mont->plain via lemma_mont_to_plain_butterfly_*,
  chainer, poly_step plain, compose_7 plain layer-order 7..1, layer1/2/3 bridges) written,
  under test. Layer1 bridge (64 groups/4 zetas) is the createi-cascade risk (mirror inverse recipe).

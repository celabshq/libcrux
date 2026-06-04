# PLAN — cross-chunk forward-NTT layers 3-7 (the "#1 next" after A3)

Goal: give `ntt_at_layer_{3,4,5,6,7}` the same functional ensures the within-chunk
drivers (0/1/2) now have —
`forall i<256. (simd_units_to_array (chunks_of_re re_future))[i] %q ==
               (Hacspec_ml_dsa.Ntt.ntt_layer (simd_units_to_array (chunks_of_re re)) k)[i] %q`
— then compose all 8 layers in the top `ntt` driver to get `== Hacspec_ml_dsa.Ntt.ntt`.

This is a genuine step up from A3 (which was mechanical): layers 3-7 are CROSS-UNIT
and driven by a LOOP, so both the spec bridge and the composition shape are new.

## Established facts (verified 2026-06-04)

- **Spec** `Hacspec_ml_dsa.Ntt.ntt_layer p layer` (extraction/Hacspec_ml_dsa.Ntt.fst:86):
  `len = 1<<layer`, `k = 128/len`; for flat `i`: `round = i/(2*len)`, `idx = i%(2*len)`,
  `z = v_ZETAS.[round+k]`; `idx<len` → `out[i] = mod_q(p[i] + mod_q(z*p[i+len]))`;
  else `out[i] = mod_q(p[i-len] - mod_q(z*p[i]))`.  `mod_q a = a % 8380417` (opaque).
- **Cross-unit geometry**: layers 3-7 have `len = 8*step_by`, `step_by ∈ {1,2,4,8,16}`
  for layers {3,4,5,6,7}.  Flat `i = 8u+l` (unit u, lane l).  Pair (i, i+len) =
  (unit u lane l, unit u+step_by lane l).  So the butterfly is between unit `u` and
  unit `u+step_by`, SAME lane, with ONE zeta shared across all 8 lanes.
- **Impl drivers** (src/simd/portable/ntt.rs): `ntt_at_layer_k` calls
  `outer_3_plus::<OFFSET, STEP_BY, ZETA>` N times (N = 16,8,4,2,1 for k=3,4,5,6,7),
  `STEP_BY = step_by` (in units), `OFFSET = c*STEP*2/8` for call c.
  `outer_3_plus<OFFSET,STEP_BY,ZETA>` runs `for j in OFFSET..OFFSET+STEP_BY { round(re,j,STEP_BY,ZETA) }`.
  `round(re,index,step_by,zeta)` = cross-unit butterfly on units (index, index+step_by):
  `tmp = mont_mul_by_constant(old re[index+step_by], zeta); re[index+step_by] = old re[index] - tmp; re[index] = old re[index] + tmp`.
- **Zeta for call c** = `v_ZETAS.[round_c + k]` where `round_c = OFFSET_c/(2*step_by)` and
  `k = 16/step_by`.  Concretely the impl literal = `zeta_r(round_c + k)`.  Layer 3:
  `zeta_r(c+16)` (verified zeta_r(16)=2725464=layer-3 call-0 literal); layer 4: `zeta_r(c+8)`;
  layer 5: `zeta_r(c+4)`; layer 6: `zeta_r(c+2)`; layer 7: `zeta_r(c+1)`.
- **Leaf arithmetic already proven** (no new impl FE posts needed for the *leaf*):
  - `montgomery_multiply_by_constant` (arithmetic.rs:174): post gives
    `is_i32b_array_opaque 8380416 result /\ (forall l. mod_q (v result[l]) == mod_q (v simd_unit[l] * v c * 8265825))`.
  - `add` / `subtract` (arithmetic.rs:19/46): `add_post` / `sub_post` opaque preds giving
    the exact per-lane value relations.
- `lemma_v_zetas_eq_zeta (i:nat{1<=i<256})` and `Spec.MLDSA.Ntt.zeta_r` (with its mod_q
  ensures) are PROVEN and reusable for the zeta congruence (A3a).
- **Only `Commute.Chunk.fst` exists** for ml-dsa (within-chunk layer-0/1/2 poly lemmas
  + simd_units_to_array + lemma_v_zetas_eq_zeta).  No cross-chunk bridge yet.  ml-kem's
  cross-chunk NTT functional architecture lives in `specs/ml-kem/proofs/fstar/commute/
  Hacspec_ml_kem.Commute.Chunk.fst` (1166 lines) — STUDY IT as the template (the
  investigation report `proofs/agent-status/ntt-functional-correctness-investigation-2026-06-03.md`
  maps its `pv_post`/`poly_step`/`lemma_compose_7` design).

## Phases

### Phase 0 — Study the ml-kem cross-chunk template (1 session-quarter)
Read `Hacspec_ml_kem.Commute.Chunk.fst` + the investigation report.  ml-kem proves
cross-chunk NTT == spec; identify which lemma corresponds to "per-unit-pair butterfly
forall ⟹ flat ntt_layer == spec" and how it dispatches the flat index.  Decide whether
to port one parameterized-by-`step_by` lemma or five per-layer lemmas.

### Phase 1 — Cross-chunk poly bridge in Commute.Chunk.fst (the new spec work)
Author `lemma_ntt_layer_cross_to_hacspec` (param by `len`/`step_by`, OR per-layer
`lemma_ntt_layer_{3..7}_step_to_hacspec_poly`):
- **Requires** (the shape the driver will establish): for every lo-unit `u` and lane `l<8`
  in the layer's pairing, the butterfly relations between `input.[u].[l]`,
  `input.[u+step_by].[l]`, `transformed.[u].[l]`, `transformed.[u+step_by].[l]` with a
  witness `t` and per-pair zeta `zm`, PLUS the zeta congruence
  `(v (zm ...)) %q == (v v_ZETAS.[round+k] * pow2 32) %q`.
- **Ensures**: `forall flat i<256. simd_units_to_array(transformed)[i] %q == ntt_layer(simd_units_to_array(input)) layer [i] %q`.
- **Proof**: per flat `i`, `u=i/8, l=i%8, idx=i%(2*len), round=i/(2*len)`; case lo (`idx<len`,
  i.e. `u` in the lo half of its 2*step_by-unit block) vs hi; pick the matching butterfly
  relation; reuse `lemma_layer_0_pair_spec`-style butterfly algebra + the zeta congruence.
  The flat→(unit,lane) index lemmas mirror `lemma_simd_units_to_array_reveal`.
- VALIDATE in a tiny scratch module first (recipe from A3); never iterate in the
  1948-line Commute.Chunk.fst.  Editing Commute.Chunk cascades ~20-min rebuilds of
  Simd.Portable.{Arithmetic,Ntt} — gate per-edit with a scratch module.

### Phase 2 — outer_3_plus round functional post (cross-unit FE atom)
Add to `outer_3_plus`'s inner `round` ensures an opaque atom
`unit_fe_post_cross (ci_lo ci_hi co_lo co_hi : t_Array i32 8) (zeta {is_i32b 4190208})`
= the 8-lane (`forall l<8` or ground×8) cross-unit butterfly
(`co_lo[l]=ci_lo[l]+t[l] /\ co_hi[l]=ci_lo[l]-t[l] /\ (v t[l])%q==(v ci_hi[l]*v zeta*8265825)%q`,
`t[l] = mont(ci_hi[l], zeta)`).  Discharge in the round body from the
`mont_mul_by_constant` + `add_post`/`sub_post` posts (reveal them + the atom).  Mirror the
A3 ground-atom + `lemma_atom_to_bf`-style unfold.

### Phase 3 — outer_3_plus loop invariant (THE new composition shape)
`outer_3_plus` is a `for j in OFFSET..OFFSET+STEP_BY` loop (NOT 32 explicit rounds).  Add
to its `loop_invariant!` (alongside the existing `modifies_range2_32` + bounds forall) the
per-processed-pair atom: `forall32 (fun u -> (u in [OFFSET,j)) ==> unit_fe_post_cross
(orig_re[u]) (orig_re[u+STEP_BY]) (re[u]) (re[u+STEP_BY]) ZETA)`.  After the loop, all
STEP_BY pairs carry the atom.  This loop-carried atom is the piece A3 did NOT exercise
(A3 composed ground concrete rounds).  Risk: the per-iteration WP must extend the atom
forall by one pair + frame the rest — opaque atom keeps it light (like the bounds
invariant already there).  Snapshot `orig_re` already exists in outer_3_plus.

### Phase 4 — ntt_at_layer_{3..7} driver wiring (per layer)
Each `ntt_at_layer_k` sequences N concrete `outer_3_plus` calls (N=16/8/4/2/1) — GROUND
composition like the A3 within-chunk drivers.  Snapshot `orig_re`; functional ensures
`== ntt_layer k`; a clean `lemma_lk_cross_compose` that assembles the `forall32` of
cross atoms from the N outer_3_plus posts (+ frame + zeta assert_norms) and feeds the
Phase-1 cross poly lemma.  N assert_norms link each `ZETA` literal to `zeta_r`.
Do layer 3 first (step_by=1, N=16, simplest) to de-risk; then 4-7.

### Phase 5 (= the "#2" capstone) — 8-layer compose → ntt == Hacspec_ml_dsa.Ntt.ntt
With all of `ntt_at_layer_{0..7}` carrying `== ntt_layer _ k`, the top `ntt` driver
(ntt.rs: ntt_at_layer_7;...;ntt_at_layer_0) chains them: `out == ntt_layer (... (ntt_layer
w 7) ...) 0 == Hacspec_ml_dsa.Ntt.ntt w` (the spec `ntt` is exactly that chain).  Port
ml-kem's `lemma_compose_7/8`.  Mind the mod-q vs ==: layers give per-coeff mod-q
congruence; the chain must thread mod-q through each `ntt_layer` (its inputs are read mod
nothing — but ntt_layer's output is already mod_q-reduced, so the chain may need a
"both sides mod_q" framing lemma per step).

## Gotchas carried from A3 (apply here too)
- hax hoists nested `round` ABOVE its outer fn's before-blocks → put cross-chunk helper
  before-blocks on an EARLY fn (the cluster is already on `simd_unit_ntt_step`; extend it).
- `recursion_limit=1024` already set in lib.rs; bump further if the before-block stack on
  one fn grows past it.
- Opaque-atom refined-zeta args (`{is_i32b 4190208 _}`) for the mont precond.
- Validate every new F* incantation in a scratch module (sub-second) before the ~20-min
  extract+build cycles; the proxy session DIES across `fstar_build`.
- Gate per-layer: rm the module .checked, full `fstar_build check/…Portable.Ntt.fst`, no
  `--admit_except`, exit 0 + Query-stats + 0 unretried failures.  Then full-crate
  `JOBS=2 ./hax.sh prove` after all layers.

## Risk ranking
1. Phase 1 cross-chunk poly lemma (new spec, flat-index dispatch) — HIGH; de-risk via
   ml-kem template + scratch.
2. Phase 3 loop-carried atom invariant — MEDIUM (new shape, but opaque atom should keep
   it light); de-risk on layer 3 (step_by=1, loop runs once → nearly the A3 ground case).
3. Phases 2/4 — LOW (mechanical, mirror A3).
4. Phase 5 mod-q chaining — MEDIUM (the inter-layer mod_q framing).

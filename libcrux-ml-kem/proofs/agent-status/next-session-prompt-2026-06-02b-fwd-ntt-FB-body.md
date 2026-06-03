# Next session — Forward NTT F-B: ntt_at_layer_4_plus body scaffold + discharge assumes

Worktree `/Users/karthik/libcrux-fwd-ntt`, branch `agent/fwd-ntt-mirror-inverse`.
**Read `RESUME-fwd-ntt-2026-06-02b-bridge-done.md` first** (full per-piece status +
adaptations + gotchas), then `RESUME-fwd-ntt-2026-06-02.md` (architecture).
Use the fstar-for-libcrux skill. hax 0.3.7 at `~/.opam/hax-0.3.7/bin` (checkout d8b5b3d).

STATE: the F-B **bridge core is DONE & committed (`7e7e6761e`)** —
`Hacspec_ml_kem.Commute.Ntt_bridge.fst` verifies admit-free, including
`lemma_layer_4_plus_to_poly_step` (cross_vec_hyp_fwd forall + ascending zeta → PLAIN
`poly_step` for layers 4-7) and its supports (cross_vec_hyp_fwd, per_coeff_fwd,
cross_vector_fwd, cross_vec_from_step_fwd, cross_vec_frame_fwd) + the restored
unfold lemmas. `ntt_vector_u` still proves `to_spec_poly_plain re_future == N.ntt (...)`
modulo: 4 `assume poly_step` (layers 7,6,5,4), 1 bound assume (28296), 1 barrett-value
assume, and an abs(-1600) assume in ntt_at_layer_7_.

DO (priority order):
1. **F-B body** — make `ntt_at_layer_4_plus` PROVE the cross_vec_hyp_fwd forall and emit a
   `poly_step` post, then replace the 4 `assume (Ntt_bridge.poly_step ...)` in `ntt_vector_u`
   (Ntt.fst lines 675/685/695/705) with the real flow. Port the inverse scaffold
   `Libcrux_ml_kem.Invert_ntt.fst` lines 363–1270 to forward. THREE real adaptations
   (rest is rename inv→fwd + swap the butterfly atom):
   a. `ntt_step_post` (forward butterfly, MONT) + strengthen `ntt_layer_int_vec_step`'s
      ensures to emit it. Per-lane: `Chunk.lemma_mont_mul_fe_commute_mont_mont` +
      `lemma_add_fe_commute_mont` + `lemma_sub_fe_commute_mont` (NO barrett — simpler than
      the inverse `inv_ntt_layer_int_vec_step_reduce`).
   b. `outer_inv`/`inner_inv` PARAMETERIZED by `e_initial_coefficient_bound`
      (PENDING = bound, DONE = bound+3328 — opposite polarity & variable vs the inverse's
      hardcoded 4·3328/3328). Match the bound shape already in the extracted body
      (Ntt.fst:404-448).
   c. `lemma_layer_numeric_facts`/`zs_of_fwd`: zeta_i INCREMENTS; `zeta_i_init == groups−1`;
      `zs_of_fwd = Seq.init groups (fun r -> mont(zeta(zeta_i_init+1+r)))` (ASCENDING).
   Body wiring mirrors `invert_ntt_at_layer_4_plus` but ends with
   `Ntt_bridge.lemma_layer_4_plus_to_poly_step` (PLAIN), not the inverse MONT post.
   Iterate via `--admit_except` per-fn; the keystone/inv lemmas live in the Ntt module
   (cite the module-local `ntt_step_post`).
2. **abs(-1600)** (Ntt.fst:531): replace the `assume` with
   `Spec.Utils.impl_i16__abs_value (mk_i16 (-1600))` (pre-existing library axiom;
   `abs_i16` is abstract so assert_norm can't compute it).
3. **28296 bound** (Ntt.fst:741): factor `is_bounded_poly_higher re 26624 28296` into a
   clean-context helper (inverse calls it directly at Invert_ntt.fst:1283+).
4. **barrett-value** (Ntt.fst:744): strengthen `Polynomial.impl__poly_barrett_reduce` post
   (shared module — do LAST, cascade risk) → `Chunk.lemma_poly_barrett_reduce_commute` +
   `lemma_poly_barrett_reduce_id` (plain identity).
5. Backport all .fst-direct edits to `src/ntt.rs` (mirror the F-A layers-1-3 pattern),
   re-extract (hax 0.3.7), full build `check/Libcrux_ml_kem.Ntt.fst` (NO `--admit_except`,
   rm the .checked first), confirm ntt module 1 lax → 0, refresh `verification_status.md`.

BUILD: bridge ≈5 min, driver ≈5-7 min. `--admit_except <Module>.<fn>` writes a TAINTED
.checked → for the final gate, `rm .fstar-cache/checked/Libcrux_ml_kem.Ntt.fst.checked`
then full build, require a real multi-min wall + "All verification conditions discharged".
Curl `/tmp/fp.sh fstar_build '{...,"wait_secs":580}'` to beat the 60 s MCP cap. Max 4 own
fstar/z3; serialize builds; kill orphans by PID (never pkill by name).

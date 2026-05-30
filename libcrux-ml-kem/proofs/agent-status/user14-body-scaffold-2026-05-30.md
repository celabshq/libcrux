# USER-14 body-proof scaffold for `invert_ntt_at_layer_4_plus`

All 6 supporting lemmas are verified & committed in `Bridges.fst` (tip `e947a206f`).
This file holds the in-progress body proof so it can be re-applied once the
extraction-dir `.depend` is restored (see the env blocker below).

## STATUS UPDATE (2026-05-31): env blocker FIXED; body now blocked on nested-fold WP
- The `.depend` env blocker below is **RESOLVED** (commit `064186f97`): removing the
  `hpke-rs` dependency dropped the published libcrux crates that pinned `hax-lib "=0.3.6"`,
  collapsing the tree to a single `hax-lib 0.3.7` (git). `.depend` regenerates cleanly and the
  extraction dir builds again.
- **New finding on the body build itself:** with Phase 1 applied (`--admit_except` on the fn),
  the function VC has ONE giant sub-query that **cancels at rlimit 800 (~185 s)** — the
  never-before-verified **nested double-fold bounds maintenance** collapsing the function-level
  WP (skill: "Sequential folds collapse the function-level WP — split functions"). Other
  sub-queries pass in ms. So before/with the `cross_vec_hyp` threading, the bounds + zeta_i
  post for this nested-loop fn must be made tractable.
  - **Recommended fix: split `invert_ntt_at_layer_4_plus` into per-round / per-step helpers**
    (single fold each), so the call boundary isolates each WP — exactly the ml-dsa
    `compute_as1_plus_s2` split pattern. Then apply the Phase-1/2 scaffold to the (smaller)
    helpers.
  - **Runaway caution:** do NOT `assume (forall round. … sz(2*v groups-1-round) …)` inside the
    fn — that quantifier sent z3 to a 10 GB / 75-min runaway (no rlimit cap caught it). Monolithic
    builds of this fn are runaway-prone; keep the fn small (split) and use `verify-to-position`
    where possible. Kill stray z3 with `pkill -f 'admit_except.*invert_ntt_at_layer_4_plus'`.

## Env blocker (RESOLVED — kept for context; why this wasn't built/committed earlier)
The extraction dir's `fstar --dep full` fails on dependency CYCLES in
hax-lib 0.3.6 / core-models 0.0.5 (this worktree's hax checkout `d8b5b3d`):
- without `--cmi`: `Error 308: Recursive dependency on Core_models.Iter.Traits.Iterator.fst`
  (self-loop + `.fst→.fst` adapter cycle; line 1915 of dep.graph).
- with `--cmi`: cycle `Monomorphized_update_at.fsti ↔ Core_models.Ops.Range.fsti`.
The Makefile's `.depend` rule redirects the failing command's output into `.depend`,
corrupting it. The worktree was seeded with a WORKING `.depend`; editing any transitive
dep of the extraction roots (e.g. committing the Bridges.fst lemmas) makes `make` regen
`.depend`, which destroys the seed. **The seed is not cold-regenerable here and is
gitignored.** No sibling worktree shares hash `d8b5b3d` to copy from.
**To resume:** restore a valid extraction-dir `.depend` (re-seed from the setup that
created the warm worktree, or pin a hax/core-models version whose `--dep` is acyclic),
then apply the scaffold below and build `check/Libcrux_ml_kem.Invert_ntt.fst`.
(The interactive proxy session bypasses `.depend` but dies within minutes on this heavy
module, so it wasn't a reliable workaround.)

## Phase plan
- **Phase 1 (below):** prologue (snapshot/zs/numeric facts) + post-loop assembly calling
  the verified `lemma_layer_4_plus_post_from_cross_vec`, with the loop's `cross_vec_hyp`
  `assume`d. Validates the outlet + that the bounds invariants verify un-admitted.
- **Phase 2:** replace the `assume` by strengthening BOTH `fold_range` invariants with the
  done/pending `cross_vec_hyp` partition (== the existing bounds clause's loose/tight split),
  calling `lemma_cross_vec_from_step` (per inner step, cin = re_init since j & j+step_vec are
  pending⟹==re_init pre-step) + `lemma_cross_vec_frame` (carry the rest across the Seq.upd).

## Phase 1 scaffold — replace the `#push-options "--admit_smt_queries true"` block

Options line becomes:
`#push-options "--z3rlimit 400 --ext context_pruning --split_queries always --fuel 1 --ifuel 1"`

Prologue (insert after the `let (#v_Vector…)(layer: usize) =` header, replacing the
existing `let e_zeta_i_init = zeta_i in let step = … in`):

```fstar
  let e_zeta_i_init:usize = zeta_i in
  let re0:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = re in
  let re_init:t_Array v_Vector (mk_usize 16) = re.Libcrux_ml_kem.Vector.f_coefficients in
  let step:usize = mk_usize 1 <<! layer in
  let groups:usize = mk_usize 128 >>! layer in
  let _:squash (v step == pow2 (v layer) /\ v e_zeta_i_init == 2 * v groups /\
                v groups >= 1 /\ v step == 16 * (v step / 16) /\ v step / 16 >= 1 /\
                v groups == 128 / pow2 (v layer)) =
    (match v layer with
      | 4 -> assert_norm (v (mk_usize 1 <<! mk_usize 4 <: usize) == 16 /\
                          v (mk_usize 128 >>! mk_usize 4 <: usize) == 8 /\ pow2 4 == 16)
      | 5 -> assert_norm (v (mk_usize 1 <<! mk_usize 5 <: usize) == 32 /\
                          v (mk_usize 128 >>! mk_usize 5 <: usize) == 4 /\ pow2 5 == 32)
      | 6 -> assert_norm (v (mk_usize 1 <<! mk_usize 6 <: usize) == 64 /\
                          v (mk_usize 128 >>! mk_usize 6 <: usize) == 2 /\ pow2 6 == 64)
      | 7 -> assert_norm (v (mk_usize 1 <<! mk_usize 7 <: usize) == 128 /\
                          v (mk_usize 128 >>! mk_usize 7 <: usize) == 1 /\ pow2 7 == 128)
      | _ -> ())
  in
  let step_vec_n:nat = v step / 16 in
  let zs:t_Slice Hacspec_ml_kem.Parameters.t_FieldElement =
    Seq.init (v groups)
      (fun (r: nat{r < v groups}) ->
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
            (Libcrux_ml_kem.Polynomial.zeta (e_zeta_i_init -! mk_usize 1 -! mk_usize r <: usize)
              <: i16))
  in
  let _:unit =
    let aux (round: nat)
        : Lemma
          (round < v groups ==>
            Seq.index zs round ==
            Hacspec_ml_kem.Ntt.v_ZETAS.[ sz (2 * v groups - 1 - round) ]) =
      if round < v groups
      then begin
        FStar.Seq.Base.lemma_index_create (v groups)
          (fun (r: nat{r < v groups}) ->
              Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Libcrux_ml_kem.Polynomial.zeta (e_zeta_i_init -! mk_usize 1 -! mk_usize r <: usize)))
          round;
        Hacspec_ml_kem.Commute.Bridges.lemma_zeta_eq_vzetas
          (e_zeta_i_init -! mk_usize 1 -! mk_usize round <: usize);
        assert (v (e_zeta_i_init -! mk_usize 1 -! mk_usize round <: usize) == 2 * v groups - 1 - round)
      end
    in
    Classical.forall_intro aux
  in
```

Epilogue (replace the final `zeta_i, re <: (usize & …)`):

```fstar
  (* PHASE 1: assume the loop produced cross_vec_hyp (Phase 2 proves via invariants). *)
  assume (forall (m: nat) (l: nat).
            Hacspec_ml_kem.Commute.Bridges.cross_vec_hyp #v_Vector re_init
              re.Libcrux_ml_kem.Vector.f_coefficients step_vec_n zs m l);
  Hacspec_ml_kem.Commute.Bridges.lemma_layer_4_plus_post_from_cross_vec #v_Vector
    re0 re layer step step_vec_n zs;
  zeta_i, re <: (usize & Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
```

The two `fold_range`s in between stay verbatim for Phase 1. (Caveats to check on first build:
`FStar.Seq.Base.lemma_index_create` is the right name for `Seq.init` indexing — if not, use the
correct `Seq` init-index lemma; and confirm the `squash` match's `_->()` branch closes from the
`requires` precondition.)

## Phase 2 — invariant strengthening (per the blocker doc recipe)
Add to BOTH fold invariants, alongside the existing bounds clause, for each `i < 16`:
- in the "4*3328 / loose / pending" position: `... /\ Seq.index re.f_coefficients (v i) == Seq.index re_init (v i)`
- in the "3328 / tight / done" position:    `... /\ (forall (l:nat). l < 16 ==> Hacspec_ml_kem.Commute.Bridges.cross_vec_hyp #v_Vector re_init re.f_coefficients (v step/16) zs (v i) l)`
- plus carry `v step == 16 \/ v step == 32 \/ v step == 64 \/ v step == 128` (so `v step/16 : pos`).
Inner step body: after the two `update_at_usize`, assert `re_before[j]==re_init[j]` and
`re_before[j+step_vec]==re_init[j+step_vec]` (from pending), invoke the per-step bridge
`lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec` (f_repr arrays of the step inputs/outputs),
then `lemma_cross_vec_from_step re_init re_after (v step/16) zs (v j) (zeta zeta_i)`; and for every
other vector `m'`, `lemma_cross_vec_frame` to carry its prior `cross_vec_hyp` across the two upds.
Then remove the Phase-1 `assume`. Finally backport the snapshot/zs/invariant/asserts to
`invert_ntt.rs` (`loop_invariant!` + `fstar!` blocks), re-extract, per-stage clean build.

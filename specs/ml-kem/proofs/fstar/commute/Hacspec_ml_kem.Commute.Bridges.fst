module Hacspec_ml_kem.Commute.Bridges
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models
open Libcrux_ml_kem.Vector.Traits.Spec
open Hacspec_ml_kem.Commute.Chunk

(* Function-form per-vector hacspec bridges (the slow ones — Z3 queries
   take multi-minutes on first verification without hint replay) for
   the trait's NTT/INTT step methods.  These lift the trait branch
   posts (per-lane FE equations under `mont_i16_to_spec_fe`) to a
   single per-vector function-form equation
       `mont_i16_to_spec_array(out) ==
         <hacspec_layer_fn> (mont_i16_to_spec_array(in)) ...`.

   Split out from `Hacspec_ml_kem.Commute.Chunk` to isolate the slow
   queries: editing this file does not invalidate Chunk.fst.checked.
   Helpers (`mont_array_lane`, `zetas_4_lane`,
   `lemma_ntt_layer_n_16_2_lane`, `lemma_ntt_inverse_layer_n_16_2_lane`)
   live in Chunk because Polynomial.fst/Invert_ntt.fst's proofs depend
   on those axioms transitively.

   Future layer 2/3 and layer-4_plus per-vector function-form bridges
   go in this file, NOT in Chunk.fst. *)

module P  = Hacspec_ml_kem.Parameters
module T  = Libcrux_ml_kem.Vector.Traits
module TS = Libcrux_ml_kem.Vector.Traits.Spec
module N  = Hacspec_ml_kem.Ntt
module IN = Hacspec_ml_kem.Invert_ntt

(*** Phase 7a (track A) — Inverse spec function unfold helper ***)

(* Per-lane unfold for `IN.ntt_inverse_layer_n (mk_usize 16) p (mk_usize 2) zs`
   at concrete lane `i ∈ [0, 16)`.  Mirror of `lemma_ntt_layer_n_16_2_lane`
   (in Chunk.fst) for the inverse direction.  Defined here in Bridges.fst
   (NOT in Chunk.fst) so that introducing it to the codebase doesn't
   change Polynomial.fst's transitive SMT context (which would regress
   `add_to_ring_element` Q60 with "incomplete quantifiers"). *)
#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"
let lemma_ntt_inverse_layer_n_16_2_lane
    (p: t_Array P.t_FieldElement (mk_usize 16))
    (zs: t_Array P.t_FieldElement (mk_usize 4))
    (i: nat {i < 16}) :
    Lemma
      (let result = IN.ntt_inverse_layer_n (mk_usize 16) p (mk_usize 2)
                                            (Rust_primitives.unsize zs) in
       let group : nat = i / 4 in
       let idx   : nat = i % 4 in
       (idx < 2 ==>
         i + 2 < 16 /\
         Seq.index result i ==
           (IN.inv_butterfly (Seq.index zs group)
                              (Seq.index p i)
                              (Seq.index p (i + 2)))._1) /\
       (idx >= 2 ==>
         i >= 2 /\
         Seq.index result i ==
           (IN.inv_butterfly (Seq.index zs group)
                              (Seq.index p (i - 2))
                              (Seq.index p i))._2))
  = let result = IN.ntt_inverse_layer_n (mk_usize 16) p (mk_usize 2)
                                         (Rust_primitives.unsize zs) in
    P.createi_lemma #P.t_FieldElement (mk_usize 16)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 16 }) ->
        let group:usize = j /! (mk_usize 2 *! mk_usize 2 <: usize) in
        let idx:usize = j %! (mk_usize 2 *! mk_usize 2 <: usize) in
        (if idx <. mk_usize 2 then
          (IN.inv_butterfly (Seq.index zs (v group))
                             (Seq.index p (v j))
                             (Seq.index p (v j + 2)))._1
        else
          (IN.inv_butterfly (Seq.index zs (v group))
                             (Seq.index p (v j - 2))
                             (Seq.index p (v j)))._2)
        <: P.t_FieldElement)
      (sz i)
#pop-options

(*** Phase 7b — Forward NTT layer 1 hacspec bridge ***)

#restart-solver

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Per-lane bridge for `f_ntt_layer_1_step`: produces the per-lane FE
   equation `out_fe.[i] == rhs.[i]` from the trait branch post and the
   `lemma_ntt_layer_n_16_2_lane` unfold helper.

   Key idea: lane `i ∈ [0, 16)` belongs to branch `b = i / 4`, position
   `idx = i % 4` within the branch.  The trait branch post exposes 4
   FE equalities at lanes `(4b, 4b+1, 4b+2, 4b+3)`.  The hacspec lane
   `i` matches:
     - if idx < 2 (lanes 4b or 4b+1): `result[i] = vec[i] + z*vec[i+2]`
       (first FE eq for `i = 4b`, third for `i = 4b+1`)
     - if idx >= 2 (lanes 4b+2 or 4b+3): `result[i] = vec[i-2] - z*vec[i]`
       (second FE eq for `i = 4b+2`, fourth for `i = 4b+3`)
   The N.butterfly._{1,2} structurally matches the branch post's
   add/sub by virtue of `mont_i16_to_spec_fe`'s linearity. *)
private
let lemma_ntt_layer_1_step_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16)
    (i: nat {i < 16}) :
  Lemma
    (requires
      TS.ntt_layer_1_step_post in_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = N.ntt_layer_n (mk_usize 16) p_fe (mk_usize 2)
                               (Rust_primitives.unsize zs) in
       Seq.index r_fe i == Seq.index rhs i))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    (* Branch b = i / 4 ∈ {0,1,2,3}; reveal post for that branch. *)
    let b : nat = i / 4 in
    assert (b < 4);
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_layer_1_step_branch_post bb in_arr zeta0 zeta1 zeta2 zeta3 out_arr));
    assert (TS.ntt_layer_1_step_branch_post b in_arr zeta0 zeta1 zeta2 zeta3 out_arr);
    reveal_opaque (`%TS.ntt_layer_1_step_branch_post)
                  (TS.ntt_layer_1_step_branch_post b in_arr zeta0 zeta1 zeta2 zeta3 out_arr);
    (* Now we have, for the right z (zeta0..3 picked by b), 4 FE equalities
       at lanes (4b, 4b+2, 4b+1, 4b+3). *)
    lemma_ntt_layer_n_16_2_lane p_fe zs i;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz b);
    (* Unfold per-array index helpers — these provide
       `(mont_i16_to_spec_array (sz 16) x).[i] == mont_i16_to_spec_fe x.[i]`. *)
    mont_array_lane out_arr (sz i);
    mont_array_lane in_arr (sz i);
    let idx : nat = i % 4 in
    if idx < 2 then begin
      assert (i + 2 < 16);
      mont_array_lane in_arr (sz (i + 2))
    end else begin
      assert (i >= 2);
      mont_array_lane in_arr (sz (i - 2))
    end

#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Per-vector hacspec bridge for `f_ntt_layer_1_step`.

   Composes the 16 per-lane bridges via `Classical.forall_intro` +
   `Seq.lemma_eq_intro`. *)
let lemma_ntt_layer_1_step_to_hacspec
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires TS.ntt_layer_1_step_pre (T.f_repr vec) zeta0 zeta1 zeta2 zeta3)
    (ensures
       (let r = T.f_ntt_layer_1_step vec zeta0 zeta1 zeta2 zeta3 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r) ==
          N.ntt_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (sz 16) (T.f_repr vec))
            (mk_usize 2)
            (Rust_primitives.unsize (zetas_4_ zeta0 zeta1 zeta2 zeta3))))
  = let r = T.f_ntt_layer_1_step vec zeta0 zeta1 zeta2 zeta3 in
    let in_arr = T.f_repr vec in
    let out_arr = T.f_repr r in
    let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    let rhs = N.ntt_layer_n (mk_usize 16) p_fe (mk_usize 2)
                            (Rust_primitives.unsize zs) in
    assert (TS.ntt_layer_1_step_post in_arr zeta0 zeta1 zeta2 zeta3 out_arr);
    let aux (j: nat) : Lemma (j < 16 ==> Seq.index r_fe j == Seq.index rhs j)
      = if j < 16 then
          lemma_ntt_layer_1_step_lane_bridge in_arr out_arr
            zeta0 zeta1 zeta2 zeta3 j
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro r_fe rhs

#pop-options


(*** Phase 7a (track A) — Inverse NTT layer 1 hacspec bridge ***)

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Per-lane bridge for `f_inv_ntt_layer_1_step`: produces the per-lane FE
   equation `out_fe.[i] == rhs.[i]` from the trait branch post and the
   `lemma_ntt_inverse_layer_n_16_2_lane` unfold helper.

   Mirror of `lemma_ntt_layer_1_step_lane_bridge` for the inverse
   direction.  Lane `i ∈ [0, 16)` belongs to branch `b = i / 4`,
   position `idx = i % 4`.  Trait branch post exposes 4 FE equalities
   at lanes `(4b, 4b+1, 4b+2, 4b+3)`.  The hacspec lane `i` matches:
     - if idx < 2 (lanes 4b or 4b+1): `result[i] = vec[i] + vec[i+2]`
       (SUM — matches `inv_butterfly._1` at `(i, i+2)`)
     - if idx ≥ 2 (lanes 4b+2 or 4b+3): `result[i] = z·(vec[i] − vec[i−2])`
       (MUL — matches `inv_butterfly._2` at `(i−2, i)`).
   `IN.inv_butterfly._{1,2}` structurally matches the branch post's
   `add`/`mul-of-sub` by virtue of `mont_i16_to_spec_fe`'s linearity. *)
private
let lemma_inv_ntt_layer_1_step_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16)
    (i: nat {i < 16}) :
  Lemma
    (requires
      TS.inv_ntt_layer_1_step_post in_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = IN.ntt_inverse_layer_n (mk_usize 16) p_fe (mk_usize 2)
                                         (Rust_primitives.unsize zs) in
       Seq.index r_fe i == Seq.index rhs i))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    let b : nat = i / 4 in
    assert (b < 4);
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.inv_ntt_layer_1_step_branch_post bb in_arr zeta0 zeta1 zeta2 zeta3 out_arr));
    assert (TS.inv_ntt_layer_1_step_branch_post b in_arr zeta0 zeta1 zeta2 zeta3 out_arr);
    reveal_opaque (`%TS.inv_ntt_layer_1_step_branch_post)
                  (TS.inv_ntt_layer_1_step_branch_post b in_arr zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_inverse_layer_n_16_2_lane p_fe zs i;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz b);
    mont_array_lane out_arr (sz i);
    mont_array_lane in_arr (sz i);
    let idx : nat = i % 4 in
    if idx < 2 then begin
      assert (i + 2 < 16);
      mont_array_lane in_arr (sz (i + 2))
    end else begin
      assert (i >= 2);
      mont_array_lane in_arr (sz (i - 2))
    end

#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Per-vector hacspec bridge for `f_inv_ntt_layer_1_step`.

   Mirror of `lemma_ntt_layer_1_step_to_hacspec`.  Composes the 16
   per-lane bridges via `Classical.forall_intro` + `Seq.lemma_eq_intro`.

   This is the function-form layer commute: the Mont-lifted output of
   one in-vector layer-1 inverse-NTT step equals
   `IN.ntt_inverse_layer_n` applied to the Mont-lifted input.  Caller
   chains 16 of these (one per chunk) to lift to a poly-level equation
   for `invert_ntt_at_layer_1`'s post (Step 4 of the Phase 7a plan). *)
let lemma_inv_ntt_layer_1_step_to_hacspec
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires TS.inv_ntt_layer_1_step_pre (T.f_repr vec) zeta0 zeta1 zeta2 zeta3)
    (ensures
       (let r = T.f_inv_ntt_layer_1_step vec zeta0 zeta1 zeta2 zeta3 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (sz 16) (T.f_repr vec))
            (mk_usize 2)
            (Rust_primitives.unsize (zetas_4_ zeta0 zeta1 zeta2 zeta3))))
  = let r = T.f_inv_ntt_layer_1_step vec zeta0 zeta1 zeta2 zeta3 in
    let in_arr = T.f_repr vec in
    let out_arr = T.f_repr r in
    let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    let rhs = IN.ntt_inverse_layer_n (mk_usize 16) p_fe (mk_usize 2)
                                       (Rust_primitives.unsize zs) in
    assert (TS.inv_ntt_layer_1_step_post in_arr zeta0 zeta1 zeta2 zeta3 out_arr);
    let aux (j: nat) : Lemma (j < 16 ==> Seq.index r_fe j == Seq.index rhs j)
      = if j < 16 then
          lemma_inv_ntt_layer_1_step_lane_bridge in_arr out_arr
            zeta0 zeta1 zeta2 zeta3 j
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro r_fe rhs

#pop-options

(*** Phase 7a (track A) — Inverse NTT layer 3 hacspec bridge ***)

(* Per-lane unfold helper for `zetas_1_`.  Layer-3 inverse uses a single
   zeta, so `Seq.index (zetas_1_ z0) 0 == mont_i16_to_spec_fe z0`. *)
let zetas_1_lane (z0: i16) (i: usize { v i < 1 }) :
    Lemma (Seq.index (zetas_1_ z0) (v i) == mont_i16_to_spec_fe z0)
  = P.createi_lemma #P.t_FieldElement (mk_usize 1)
      #(usize -> P.t_FieldElement)
      (fun (_: usize { _ <. mk_usize 1 }) ->
        (mont_i16_to_spec_fe z0 <: P.t_FieldElement))
      i

#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"

(* Per-lane unfold for `IN.ntt_inverse_layer_n (mk_usize 16) p (mk_usize 8) zs`
   at concrete lane `i ∈ [0, 16)`.  Mirror of
   `lemma_ntt_inverse_layer_n_16_2_lane` for layer-3 step parameters
   (`len = 8`, `groups = 1`).  Since `2 * len = 16` and `i < 16`, the
   `group = i / 16 = 0` always, and `idx = i % 16 = i`.  Result:
   - if `i < 8`: result[i] = inv_butterfly._1 at (i, i+8)
   - if `i ≥ 8`: result[i] = inv_butterfly._2 at (i-8, i)
   Defined here in Bridges.fst (NOT in Chunk.fst) for the same Polynomial.fst
   transitive-context reason as `lemma_ntt_inverse_layer_n_16_2_lane` above. *)
let lemma_ntt_inverse_layer_n_16_8_lane
    (p: t_Array P.t_FieldElement (mk_usize 16))
    (zs: t_Array P.t_FieldElement (mk_usize 1))
    (i: nat {i < 16}) :
    Lemma
      (let result = IN.ntt_inverse_layer_n (mk_usize 16) p (mk_usize 8)
                                            (Rust_primitives.unsize zs) in
       (i < 8 ==>
         i + 8 < 16 /\
         Seq.index result i ==
           (IN.inv_butterfly (Seq.index zs 0)
                              (Seq.index p i)
                              (Seq.index p (i + 8)))._1) /\
       (i >= 8 ==>
         i >= 8 /\
         Seq.index result i ==
           (IN.inv_butterfly (Seq.index zs 0)
                              (Seq.index p (i - 8))
                              (Seq.index p i))._2))
  = let result = IN.ntt_inverse_layer_n (mk_usize 16) p (mk_usize 8)
                                         (Rust_primitives.unsize zs) in
    P.createi_lemma #P.t_FieldElement (mk_usize 16)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 16 }) ->
        let group:usize = j /! (mk_usize 2 *! mk_usize 8 <: usize) in
        let idx:usize = j %! (mk_usize 2 *! mk_usize 8 <: usize) in
        (if idx <. mk_usize 8 then
          (IN.inv_butterfly (Seq.index zs (v group))
                             (Seq.index p (v j))
                             (Seq.index p (v j + 8)))._1
        else
          (IN.inv_butterfly (Seq.index zs (v group))
                             (Seq.index p (v j - 8))
                             (Seq.index p (v j)))._2)
        <: P.t_FieldElement)
      (sz i)
#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Per-lane bridge for `f_inv_ntt_layer_3_step`: produces the per-lane FE
   equation `out_fe.[i] == rhs.[i]` from the trait branch post and the
   `lemma_ntt_inverse_layer_n_16_8_lane` unfold helper.

   Layer-3 lane → branch mapping: lane `i ∈ [0, 16)` belongs to branch
   `b = (i mod 8) / 2`.  Branch `b` touches the four lanes
   `(2b, 2b+1, 2b+8, 2b+9) = (i1, i2, j1, j2)`.  Hacspec lane `i`:
     - if i < 8 (low half): `result[i] = vec[i] + vec[i+8]` — matches
       `inv_butterfly._1` at `(i, i+8)`.  Lane is `i1` if `i` even, `i2`
       if `i` odd.
     - if i ≥ 8 (high half): `result[i] = z·(vec[i] − vec[i-8])` —
       matches `inv_butterfly._2` at `(i-8, i)`.  Lane is `j1` if `i`
       even, `j2` if `i` odd.
   Single zeta for the whole vector — `zetas_1_lane` collapses
   `Seq.index zs 0` to `mont_i16_to_spec_fe zeta0`. *)
private
let lemma_inv_ntt_layer_3_step_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0: i16)
    (i: nat {i < 16}) :
  Lemma
    (requires
      TS.inv_ntt_layer_3_step_post in_arr zeta0 out_arr)
    (ensures
      (let zs = zetas_1_ zeta0 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = IN.ntt_inverse_layer_n (mk_usize 16) p_fe (mk_usize 8)
                                         (Rust_primitives.unsize zs) in
       Seq.index r_fe i == Seq.index rhs i))
  = let zs = zetas_1_ zeta0 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    let b : nat = (i % 8) / 2 in
    assert (b < 4);
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.inv_ntt_layer_3_step_branch_post bb in_arr zeta0 out_arr));
    assert (TS.inv_ntt_layer_3_step_branch_post b in_arr zeta0 out_arr);
    reveal_opaque (`%TS.inv_ntt_layer_3_step_branch_post)
                  (TS.inv_ntt_layer_3_step_branch_post b in_arr zeta0 out_arr);
    lemma_ntt_inverse_layer_n_16_8_lane p_fe zs i;
    zetas_1_lane zeta0 (sz 0);
    mont_array_lane out_arr (sz i);
    mont_array_lane in_arr (sz i);
    if i < 8 then begin
      assert (i + 8 < 16);
      mont_array_lane in_arr (sz (i + 8))
    end else begin
      assert (i >= 8);
      mont_array_lane in_arr (sz (i - 8))
    end

#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Per-vector hacspec bridge for `f_inv_ntt_layer_3_step`.

   Mirror of `lemma_inv_ntt_layer_1_step_to_hacspec` for layer 3.
   Composes the 16 per-lane bridges via `Classical.forall_intro` +
   `Seq.lemma_eq_intro`.

   Caller chains 16 of these (one per chunk) to lift to a poly-level
   equation for `invert_ntt_at_layer_3`'s post (Step 4 layer 3 of the
   Phase 7a plan). *)
let lemma_inv_ntt_layer_3_step_to_hacspec
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0: i16) :
  Lemma
    (requires TS.inv_ntt_layer_3_step_pre (T.f_repr vec) zeta0)
    (ensures
       (let r = T.f_inv_ntt_layer_3_step vec zeta0 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (sz 16) (T.f_repr vec))
            (mk_usize 8)
            (Rust_primitives.unsize (zetas_1_ zeta0))))
  = let r = T.f_inv_ntt_layer_3_step vec zeta0 in
    let in_arr = T.f_repr vec in
    let out_arr = T.f_repr r in
    let zs = zetas_1_ zeta0 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    let rhs = IN.ntt_inverse_layer_n (mk_usize 16) p_fe (mk_usize 8)
                                       (Rust_primitives.unsize zs) in
    assert (TS.inv_ntt_layer_3_step_post in_arr zeta0 out_arr);
    let aux (j: nat) : Lemma (j < 16 ==> Seq.index r_fe j == Seq.index rhs j)
      = if j < 16 then
          lemma_inv_ntt_layer_3_step_lane_bridge in_arr out_arr zeta0 j
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro r_fe rhs

#pop-options

(*** Phase 7a (track A) — Inverse NTT layer 2 hacspec bridge ***)

(* Per-lane unfold helper for `zetas_2_`. *)
let zetas_2_lane (z0 z1: i16) (i: usize { v i < 2 }) :
    Lemma (Seq.index (zetas_2_ z0 z1) (v i)
           == (if v i = 0 then mont_i16_to_spec_fe z0
               else mont_i16_to_spec_fe z1))
  = P.createi_lemma #P.t_FieldElement (mk_usize 2)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 2 }) ->
        (if v j = 0 then mont_i16_to_spec_fe z0
         else mont_i16_to_spec_fe z1) <: P.t_FieldElement)
      i

#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"

(* Per-lane unfold for `IN.ntt_inverse_layer_n (mk_usize 16) p (mk_usize 4) zs`
   at concrete lane `i ∈ [0, 16)`.  Layer-2 form: 2 zetas, partner stride 4,
   group selector `i / 8`.  Mirror of `lemma_ntt_inverse_layer_n_16_2_lane`
   for layer-2 step parameters. *)
let lemma_ntt_inverse_layer_n_16_4_lane
    (p: t_Array P.t_FieldElement (mk_usize 16))
    (zs: t_Array P.t_FieldElement (mk_usize 2))
    (i: nat {i < 16}) :
    Lemma
      (let result = IN.ntt_inverse_layer_n (mk_usize 16) p (mk_usize 4)
                                            (Rust_primitives.unsize zs) in
       let group : nat = i / 8 in
       let idx   : nat = i % 8 in
       (idx < 4 ==>
         i + 4 < 16 /\
         Seq.index result i ==
           (IN.inv_butterfly (Seq.index zs group)
                              (Seq.index p i)
                              (Seq.index p (i + 4)))._1) /\
       (idx >= 4 ==>
         i >= 4 /\
         Seq.index result i ==
           (IN.inv_butterfly (Seq.index zs group)
                              (Seq.index p (i - 4))
                              (Seq.index p i))._2))
  = let result = IN.ntt_inverse_layer_n (mk_usize 16) p (mk_usize 4)
                                         (Rust_primitives.unsize zs) in
    P.createi_lemma #P.t_FieldElement (mk_usize 16)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 16 }) ->
        let group:usize = j /! (mk_usize 2 *! mk_usize 4 <: usize) in
        let idx:usize = j %! (mk_usize 2 *! mk_usize 4 <: usize) in
        (if idx <. mk_usize 4 then
          (IN.inv_butterfly (Seq.index zs (v group))
                             (Seq.index p (v j))
                             (Seq.index p (v j + 4)))._1
        else
          (IN.inv_butterfly (Seq.index zs (v group))
                             (Seq.index p (v j - 4))
                             (Seq.index p (v j)))._2)
        <: P.t_FieldElement)
      (sz i)
#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Layer-2 inverse: per-branch lane bridge for branch b ∈ {0,1,2,3}.
   Each helper is at a CONCRETE b literal so the trait branch_post's
   nested if-ladder (`z = b<2 ? zeta0 : zeta1`, `base = b<2 ? 0 : 8`,
   `off = b∈{0,2} ? 0 : 2`) collapses to literal values, avoiding the
   ~2.7-min Z3 timeout that hit the symbolic-b approach (per the
   first-cut layer-2-forward attempt mitigation note).

   Branch lane assignments:
     b=0: lanes (0, 1, 4, 5),   z=zeta0, group=0
     b=1: lanes (2, 3, 6, 7),   z=zeta0, group=0
     b=2: lanes (8, 9, 12, 13), z=zeta1, group=1
     b=3: lanes (10,11,14,15),  z=zeta1, group=1 *)

private
let lemma_inv_ntt_layer_2_step_branch_0_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16) :
  Lemma
    (requires
      TS.inv_ntt_layer_2_step_post in_arr zeta0 zeta1 out_arr)
    (ensures
      (let zs = zetas_2_ zeta0 zeta1 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = IN.ntt_inverse_layer_n (mk_usize 16) p_fe (mk_usize 4)
                                         (Rust_primitives.unsize zs) in
       Seq.index r_fe 0 == Seq.index rhs 0 /\
       Seq.index r_fe 1 == Seq.index rhs 1 /\
       Seq.index r_fe 4 == Seq.index rhs 4 /\
       Seq.index r_fe 5 == Seq.index rhs 5))
  = let zs = zetas_2_ zeta0 zeta1 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    reveal_opaque (`%TS.inv_ntt_layer_2_step_branch_post)
                  (TS.inv_ntt_layer_2_step_branch_post 0 in_arr zeta0 zeta1 out_arr);
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 0;
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 1;
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 4;
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 5;
    zetas_2_lane zeta0 zeta1 (sz 0);
    mont_array_lane out_arr (sz 0);
    mont_array_lane out_arr (sz 1);
    mont_array_lane out_arr (sz 4);
    mont_array_lane out_arr (sz 5);
    mont_array_lane in_arr (sz 0);
    mont_array_lane in_arr (sz 1);
    mont_array_lane in_arr (sz 4);
    mont_array_lane in_arr (sz 5)

private
let lemma_inv_ntt_layer_2_step_branch_1_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16) :
  Lemma
    (requires
      TS.inv_ntt_layer_2_step_post in_arr zeta0 zeta1 out_arr)
    (ensures
      (let zs = zetas_2_ zeta0 zeta1 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = IN.ntt_inverse_layer_n (mk_usize 16) p_fe (mk_usize 4)
                                         (Rust_primitives.unsize zs) in
       Seq.index r_fe 2 == Seq.index rhs 2 /\
       Seq.index r_fe 3 == Seq.index rhs 3 /\
       Seq.index r_fe 6 == Seq.index rhs 6 /\
       Seq.index r_fe 7 == Seq.index rhs 7))
  = let zs = zetas_2_ zeta0 zeta1 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    reveal_opaque (`%TS.inv_ntt_layer_2_step_branch_post)
                  (TS.inv_ntt_layer_2_step_branch_post 1 in_arr zeta0 zeta1 out_arr);
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 2;
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 3;
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 6;
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 7;
    zetas_2_lane zeta0 zeta1 (sz 0);
    mont_array_lane out_arr (sz 2);
    mont_array_lane out_arr (sz 3);
    mont_array_lane out_arr (sz 6);
    mont_array_lane out_arr (sz 7);
    mont_array_lane in_arr (sz 2);
    mont_array_lane in_arr (sz 3);
    mont_array_lane in_arr (sz 6);
    mont_array_lane in_arr (sz 7)

private
let lemma_inv_ntt_layer_2_step_branch_2_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16) :
  Lemma
    (requires
      TS.inv_ntt_layer_2_step_post in_arr zeta0 zeta1 out_arr)
    (ensures
      (let zs = zetas_2_ zeta0 zeta1 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = IN.ntt_inverse_layer_n (mk_usize 16) p_fe (mk_usize 4)
                                         (Rust_primitives.unsize zs) in
       Seq.index r_fe 8 == Seq.index rhs 8 /\
       Seq.index r_fe 9 == Seq.index rhs 9 /\
       Seq.index r_fe 12 == Seq.index rhs 12 /\
       Seq.index r_fe 13 == Seq.index rhs 13))
  = let zs = zetas_2_ zeta0 zeta1 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    reveal_opaque (`%TS.inv_ntt_layer_2_step_branch_post)
                  (TS.inv_ntt_layer_2_step_branch_post 2 in_arr zeta0 zeta1 out_arr);
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 8;
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 9;
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 12;
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 13;
    zetas_2_lane zeta0 zeta1 (sz 1);
    mont_array_lane out_arr (sz 8);
    mont_array_lane out_arr (sz 9);
    mont_array_lane out_arr (sz 12);
    mont_array_lane out_arr (sz 13);
    mont_array_lane in_arr (sz 8);
    mont_array_lane in_arr (sz 9);
    mont_array_lane in_arr (sz 12);
    mont_array_lane in_arr (sz 13)

private
let lemma_inv_ntt_layer_2_step_branch_3_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16) :
  Lemma
    (requires
      TS.inv_ntt_layer_2_step_post in_arr zeta0 zeta1 out_arr)
    (ensures
      (let zs = zetas_2_ zeta0 zeta1 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = IN.ntt_inverse_layer_n (mk_usize 16) p_fe (mk_usize 4)
                                         (Rust_primitives.unsize zs) in
       Seq.index r_fe 10 == Seq.index rhs 10 /\
       Seq.index r_fe 11 == Seq.index rhs 11 /\
       Seq.index r_fe 14 == Seq.index rhs 14 /\
       Seq.index r_fe 15 == Seq.index rhs 15))
  = let zs = zetas_2_ zeta0 zeta1 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    reveal_opaque (`%TS.inv_ntt_layer_2_step_branch_post)
                  (TS.inv_ntt_layer_2_step_branch_post 3 in_arr zeta0 zeta1 out_arr);
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 10;
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 11;
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 14;
    lemma_ntt_inverse_layer_n_16_4_lane p_fe zs 15;
    zetas_2_lane zeta0 zeta1 (sz 1);
    mont_array_lane out_arr (sz 10);
    mont_array_lane out_arr (sz 11);
    mont_array_lane out_arr (sz 14);
    mont_array_lane out_arr (sz 15);
    mont_array_lane in_arr (sz 10);
    mont_array_lane in_arr (sz 11);
    mont_array_lane in_arr (sz 14);
    mont_array_lane in_arr (sz 15)

#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Per-lane bridge (wrapper) for `f_inv_ntt_layer_2_step`.

   Dispatches lane `i` to the appropriate per-branch helper.  Each
   call site has only 4 facts in scope (one branch's worth), so the
   SMT context stays small for `aux j` use in the per-vector bridge. *)
private
let lemma_inv_ntt_layer_2_step_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16)
    (i: nat {i < 16}) :
  Lemma
    (requires
      TS.inv_ntt_layer_2_step_post in_arr zeta0 zeta1 out_arr)
    (ensures
      (let zs = zetas_2_ zeta0 zeta1 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = IN.ntt_inverse_layer_n (mk_usize 16) p_fe (mk_usize 4)
                                         (Rust_primitives.unsize zs) in
       Seq.index r_fe i == Seq.index rhs i))
  = if i = 0 || i = 1 || i = 4 || i = 5 then
      lemma_inv_ntt_layer_2_step_branch_0_lane_bridge in_arr out_arr zeta0 zeta1
    else if i = 2 || i = 3 || i = 6 || i = 7 then
      lemma_inv_ntt_layer_2_step_branch_1_lane_bridge in_arr out_arr zeta0 zeta1
    else if i = 8 || i = 9 || i = 12 || i = 13 then
      lemma_inv_ntt_layer_2_step_branch_2_lane_bridge in_arr out_arr zeta0 zeta1
    else
      lemma_inv_ntt_layer_2_step_branch_3_lane_bridge in_arr out_arr zeta0 zeta1

#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1 --split_queries always"

(* Per-vector hacspec bridge for `f_inv_ntt_layer_2_step`.

   Composes the 16 per-lane bridges via `Classical.forall_intro` +
   `Seq.lemma_eq_intro`.  Mirror of the layer-1/3 inverse structure;
   each per-lane invocation has only 4 in-scope facts (one branch). *)
let lemma_inv_ntt_layer_2_step_to_hacspec
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0 zeta1: i16) :
  Lemma
    (requires TS.inv_ntt_layer_2_step_pre (T.f_repr vec) zeta0 zeta1)
    (ensures
       (let r = T.f_inv_ntt_layer_2_step vec zeta0 zeta1 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (sz 16) (T.f_repr vec))
            (mk_usize 4)
            (Rust_primitives.unsize (zetas_2_ zeta0 zeta1))))
  = let r = T.f_inv_ntt_layer_2_step vec zeta0 zeta1 in
    let in_arr = T.f_repr vec in
    let out_arr = T.f_repr r in
    let zs = zetas_2_ zeta0 zeta1 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    let rhs = IN.ntt_inverse_layer_n (mk_usize 16) p_fe (mk_usize 4)
                                       (Rust_primitives.unsize zs) in
    assert (TS.inv_ntt_layer_2_step_post in_arr zeta0 zeta1 out_arr);
    let aux (j: nat) : Lemma (j < 16 ==> Seq.index r_fe j == Seq.index rhs j)
      = if j < 16 then
          lemma_inv_ntt_layer_2_step_lane_bridge in_arr out_arr zeta0 zeta1 j
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro r_fe rhs

#pop-options
(*** Phase 7b — Forward NTT layer 2 hacspec bridge ***)

#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"

(* Per-lane unfold for `N.ntt_layer_n (mk_usize 16) p (mk_usize 4) zs` at
   concrete lane `i ∈ [0, 16)`.  Layer-2 forward form: 2 zetas, partner
   stride 4, group selector `i / 8`.  Mirror of
   `lemma_ntt_inverse_layer_n_16_4_lane` for forward butterfly. *)
let lemma_ntt_layer_n_16_4_lane
    (p: t_Array P.t_FieldElement (mk_usize 16))
    (zs: t_Array P.t_FieldElement (mk_usize 2))
    (i: nat {i < 16}) :
    Lemma
      (let result = N.ntt_layer_n (mk_usize 16) p (mk_usize 4)
                                    (Rust_primitives.unsize zs) in
       let group : nat = i / 8 in
       let idx   : nat = i % 8 in
       (idx < 4 ==>
         i + 4 < 16 /\
         Seq.index result i ==
           (N.butterfly (Seq.index zs group)
                         (Seq.index p i)
                         (Seq.index p (i + 4)))._1) /\
       (idx >= 4 ==>
         i >= 4 /\
         Seq.index result i ==
           (N.butterfly (Seq.index zs group)
                         (Seq.index p (i - 4))
                         (Seq.index p i))._2))
  = let result = N.ntt_layer_n (mk_usize 16) p (mk_usize 4)
                                 (Rust_primitives.unsize zs) in
    P.createi_lemma #P.t_FieldElement (mk_usize 16)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 16 }) ->
        let group:usize = j /! (mk_usize 2 *! mk_usize 4 <: usize) in
        let idx:usize = j %! (mk_usize 2 *! mk_usize 4 <: usize) in
        (if idx <. mk_usize 4 then
          (N.butterfly (Seq.index zs (v group))
                        (Seq.index p (v j))
                        (Seq.index p (v j + 4)))._1
        else
          (N.butterfly (Seq.index zs (v group))
                        (Seq.index p (v j - 4))
                        (Seq.index p (v j)))._2)
        <: P.t_FieldElement)
      (sz i)
#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Layer-2 forward: per-branch lane bridge for branch b ∈ {0,1,2,3}.
   Each helper at a CONCRETE b literal so the trait branch_post's
   nested if-ladder collapses to literal values, mirroring the
   layer-2 inverse pattern (which mitigated a Z3 timeout on
   symbolic-b — see `lemma_inv_ntt_layer_2_step_branch_*_lane_bridge`).

   Branch lane assignments are IDENTICAL to inverse layer 2 (the
   trait branch_post for forward and inverse share the same
   base/off/i1/j1/i2/j2 indexing); only the per-lane FE equation
   differs (forward butterfly = (a+z*b, a-z*b) vs inverse
   inv_butterfly = (a+b, (a-b)*z)).

   Branch lane assignments:
     b=0: lanes (0, 1, 4, 5),   z=zeta0, group=0
     b=1: lanes (2, 3, 6, 7),   z=zeta0, group=0
     b=2: lanes (8, 9, 12, 13), z=zeta1, group=1
     b=3: lanes (10,11,14,15),  z=zeta1, group=1 *)

private
let lemma_ntt_layer_2_step_branch_0_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16) :
  Lemma
    (requires
      TS.ntt_layer_2_step_post in_arr zeta0 zeta1 out_arr)
    (ensures
      (let zs = zetas_2_ zeta0 zeta1 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = N.ntt_layer_n (mk_usize 16) p_fe (mk_usize 4)
                                (Rust_primitives.unsize zs) in
       Seq.index r_fe 0 == Seq.index rhs 0 /\
       Seq.index r_fe 1 == Seq.index rhs 1 /\
       Seq.index r_fe 4 == Seq.index rhs 4 /\
       Seq.index r_fe 5 == Seq.index rhs 5))
  = let zs = zetas_2_ zeta0 zeta1 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    reveal_opaque (`%TS.ntt_layer_2_step_branch_post)
                  (TS.ntt_layer_2_step_branch_post 0 in_arr zeta0 zeta1 out_arr);
    lemma_ntt_layer_n_16_4_lane p_fe zs 0;
    lemma_ntt_layer_n_16_4_lane p_fe zs 1;
    lemma_ntt_layer_n_16_4_lane p_fe zs 4;
    lemma_ntt_layer_n_16_4_lane p_fe zs 5;
    zetas_2_lane zeta0 zeta1 (sz 0);
    mont_array_lane out_arr (sz 0);
    mont_array_lane out_arr (sz 1);
    mont_array_lane out_arr (sz 4);
    mont_array_lane out_arr (sz 5);
    mont_array_lane in_arr (sz 0);
    mont_array_lane in_arr (sz 1);
    mont_array_lane in_arr (sz 4);
    mont_array_lane in_arr (sz 5)

private
let lemma_ntt_layer_2_step_branch_1_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16) :
  Lemma
    (requires
      TS.ntt_layer_2_step_post in_arr zeta0 zeta1 out_arr)
    (ensures
      (let zs = zetas_2_ zeta0 zeta1 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = N.ntt_layer_n (mk_usize 16) p_fe (mk_usize 4)
                                (Rust_primitives.unsize zs) in
       Seq.index r_fe 2 == Seq.index rhs 2 /\
       Seq.index r_fe 3 == Seq.index rhs 3 /\
       Seq.index r_fe 6 == Seq.index rhs 6 /\
       Seq.index r_fe 7 == Seq.index rhs 7))
  = let zs = zetas_2_ zeta0 zeta1 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    reveal_opaque (`%TS.ntt_layer_2_step_branch_post)
                  (TS.ntt_layer_2_step_branch_post 1 in_arr zeta0 zeta1 out_arr);
    lemma_ntt_layer_n_16_4_lane p_fe zs 2;
    lemma_ntt_layer_n_16_4_lane p_fe zs 3;
    lemma_ntt_layer_n_16_4_lane p_fe zs 6;
    lemma_ntt_layer_n_16_4_lane p_fe zs 7;
    zetas_2_lane zeta0 zeta1 (sz 0);
    mont_array_lane out_arr (sz 2);
    mont_array_lane out_arr (sz 3);
    mont_array_lane out_arr (sz 6);
    mont_array_lane out_arr (sz 7);
    mont_array_lane in_arr (sz 2);
    mont_array_lane in_arr (sz 3);
    mont_array_lane in_arr (sz 6);
    mont_array_lane in_arr (sz 7)

private
let lemma_ntt_layer_2_step_branch_2_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16) :
  Lemma
    (requires
      TS.ntt_layer_2_step_post in_arr zeta0 zeta1 out_arr)
    (ensures
      (let zs = zetas_2_ zeta0 zeta1 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = N.ntt_layer_n (mk_usize 16) p_fe (mk_usize 4)
                                (Rust_primitives.unsize zs) in
       Seq.index r_fe 8 == Seq.index rhs 8 /\
       Seq.index r_fe 9 == Seq.index rhs 9 /\
       Seq.index r_fe 12 == Seq.index rhs 12 /\
       Seq.index r_fe 13 == Seq.index rhs 13))
  = let zs = zetas_2_ zeta0 zeta1 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    reveal_opaque (`%TS.ntt_layer_2_step_branch_post)
                  (TS.ntt_layer_2_step_branch_post 2 in_arr zeta0 zeta1 out_arr);
    lemma_ntt_layer_n_16_4_lane p_fe zs 8;
    lemma_ntt_layer_n_16_4_lane p_fe zs 9;
    lemma_ntt_layer_n_16_4_lane p_fe zs 12;
    lemma_ntt_layer_n_16_4_lane p_fe zs 13;
    zetas_2_lane zeta0 zeta1 (sz 1);
    mont_array_lane out_arr (sz 8);
    mont_array_lane out_arr (sz 9);
    mont_array_lane out_arr (sz 12);
    mont_array_lane out_arr (sz 13);
    mont_array_lane in_arr (sz 8);
    mont_array_lane in_arr (sz 9);
    mont_array_lane in_arr (sz 12);
    mont_array_lane in_arr (sz 13)

private
let lemma_ntt_layer_2_step_branch_3_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16) :
  Lemma
    (requires
      TS.ntt_layer_2_step_post in_arr zeta0 zeta1 out_arr)
    (ensures
      (let zs = zetas_2_ zeta0 zeta1 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = N.ntt_layer_n (mk_usize 16) p_fe (mk_usize 4)
                                (Rust_primitives.unsize zs) in
       Seq.index r_fe 10 == Seq.index rhs 10 /\
       Seq.index r_fe 11 == Seq.index rhs 11 /\
       Seq.index r_fe 14 == Seq.index rhs 14 /\
       Seq.index r_fe 15 == Seq.index rhs 15))
  = let zs = zetas_2_ zeta0 zeta1 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    reveal_opaque (`%TS.ntt_layer_2_step_branch_post)
                  (TS.ntt_layer_2_step_branch_post 3 in_arr zeta0 zeta1 out_arr);
    lemma_ntt_layer_n_16_4_lane p_fe zs 10;
    lemma_ntt_layer_n_16_4_lane p_fe zs 11;
    lemma_ntt_layer_n_16_4_lane p_fe zs 14;
    lemma_ntt_layer_n_16_4_lane p_fe zs 15;
    zetas_2_lane zeta0 zeta1 (sz 1);
    mont_array_lane out_arr (sz 10);
    mont_array_lane out_arr (sz 11);
    mont_array_lane out_arr (sz 14);
    mont_array_lane out_arr (sz 15);
    mont_array_lane in_arr (sz 10);
    mont_array_lane in_arr (sz 11);
    mont_array_lane in_arr (sz 14);
    mont_array_lane in_arr (sz 15)

#pop-options

#push-options "--z3rlimit 800 --fuel 0 --ifuel 1 --split_queries always"

(* Per-lane bridge (wrapper) for `f_ntt_layer_2_step`.

   Dispatches lane `i` to the appropriate per-branch helper.  Each
   call site has only 4 facts in scope (one branch's worth). *)
private
let lemma_ntt_layer_2_step_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16)
    (i: nat {i < 16}) :
  Lemma
    (requires
      TS.ntt_layer_2_step_post in_arr zeta0 zeta1 out_arr)
    (ensures
      (let zs = zetas_2_ zeta0 zeta1 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = N.ntt_layer_n (mk_usize 16) p_fe (mk_usize 4)
                                (Rust_primitives.unsize zs) in
       Seq.index r_fe i == Seq.index rhs i))
  = if i = 0 || i = 1 || i = 4 || i = 5 then
      lemma_ntt_layer_2_step_branch_0_lane_bridge in_arr out_arr zeta0 zeta1
    else if i = 2 || i = 3 || i = 6 || i = 7 then
      lemma_ntt_layer_2_step_branch_1_lane_bridge in_arr out_arr zeta0 zeta1
    else if i = 8 || i = 9 || i = 12 || i = 13 then
      lemma_ntt_layer_2_step_branch_2_lane_bridge in_arr out_arr zeta0 zeta1
    else
      lemma_ntt_layer_2_step_branch_3_lane_bridge in_arr out_arr zeta0 zeta1

#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1 --split_queries always"

(* Per-vector hacspec bridge for `f_ntt_layer_2_step`.

   Composes the 16 per-lane bridges via `Classical.forall_intro` +
   `Seq.lemma_eq_intro`.  Mirror of `lemma_inv_ntt_layer_2_step_to_hacspec`
   for the forward direction. *)
let lemma_ntt_layer_2_step_to_hacspec
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0 zeta1: i16) :
  Lemma
    (requires TS.ntt_layer_2_step_pre (T.f_repr vec) zeta0 zeta1)
    (ensures
       (let r = T.f_ntt_layer_2_step vec zeta0 zeta1 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r) ==
          N.ntt_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (sz 16) (T.f_repr vec))
            (mk_usize 4)
            (Rust_primitives.unsize (zetas_2_ zeta0 zeta1))))
  = let r = T.f_ntt_layer_2_step vec zeta0 zeta1 in
    let in_arr = T.f_repr vec in
    let out_arr = T.f_repr r in
    let zs = zetas_2_ zeta0 zeta1 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    let rhs = N.ntt_layer_n (mk_usize 16) p_fe (mk_usize 4)
                             (Rust_primitives.unsize zs) in
    assert (TS.ntt_layer_2_step_post in_arr zeta0 zeta1 out_arr);
    let aux (j: nat) : Lemma (j < 16 ==> Seq.index r_fe j == Seq.index rhs j)
      = if j < 16 then
          lemma_ntt_layer_2_step_lane_bridge in_arr out_arr zeta0 zeta1 j
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro r_fe rhs

#pop-options

(*** Phase 7b — Forward NTT layer 3 hacspec bridge ***)

#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"

(* Per-lane unfold for `N.ntt_layer_n (mk_usize 16) p (mk_usize 8) zs`
   at concrete lane `i ∈ [0, 16)`.  Mirror of
   `lemma_ntt_inverse_layer_n_16_8_lane` for forward butterfly.
   Layer-3 form: 1 zeta, partner stride 8, group selector always 0
   (since `2 * len = 16` ≥ N). *)
let lemma_ntt_layer_n_16_8_lane
    (p: t_Array P.t_FieldElement (mk_usize 16))
    (zs: t_Array P.t_FieldElement (mk_usize 1))
    (i: nat {i < 16}) :
    Lemma
      (let result = N.ntt_layer_n (mk_usize 16) p (mk_usize 8)
                                    (Rust_primitives.unsize zs) in
       (i < 8 ==>
         i + 8 < 16 /\
         Seq.index result i ==
           (N.butterfly (Seq.index zs 0)
                         (Seq.index p i)
                         (Seq.index p (i + 8)))._1) /\
       (i >= 8 ==>
         i >= 8 /\
         Seq.index result i ==
           (N.butterfly (Seq.index zs 0)
                         (Seq.index p (i - 8))
                         (Seq.index p i))._2))
  = let result = N.ntt_layer_n (mk_usize 16) p (mk_usize 8)
                                 (Rust_primitives.unsize zs) in
    P.createi_lemma #P.t_FieldElement (mk_usize 16)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 16 }) ->
        let group:usize = j /! (mk_usize 2 *! mk_usize 8 <: usize) in
        let idx:usize = j %! (mk_usize 2 *! mk_usize 8 <: usize) in
        (if idx <. mk_usize 8 then
          (N.butterfly (Seq.index zs (v group))
                        (Seq.index p (v j))
                        (Seq.index p (v j + 8)))._1
        else
          (N.butterfly (Seq.index zs (v group))
                        (Seq.index p (v j - 8))
                        (Seq.index p (v j)))._2)
        <: P.t_FieldElement)
      (sz i)
#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Per-lane bridge for `f_ntt_layer_3_step`: produces the per-lane FE
   equation `out_fe.[i] == rhs.[i]` from the trait branch post and
   the `lemma_ntt_layer_n_16_8_lane` unfold helper.

   Layer-3 lane → branch mapping (same as inverse): lane `i ∈ [0, 16)`
   belongs to branch `b = (i mod 8) / 2`.  Branch `b` touches the
   four lanes `(2b, 2b+1, 2b+8, 2b+9) = (i1, i2, j1, j2)`.  Hacspec
   lane `i`:
     - if i < 8 (low half): `result[i] = vec[i] + z * vec[i+8]` —
       matches `N.butterfly._1` at `(i, i+8)`.  Lane is `i1` if `i`
       even, `i2` if `i` odd.
     - if i ≥ 8 (high half): `result[i] = vec[i-8] - z * vec[i]` —
       matches `N.butterfly._2` at `(i-8, i)`.  Lane is `j1` if `i`
       even, `j2` if `i` odd.
   Single zeta for the whole vector — `zetas_1_lane` collapses
   `Seq.index zs 0` to `mont_i16_to_spec_fe zeta0`.

   Mirror of `lemma_inv_ntt_layer_3_step_lane_bridge`; only the
   per-lane FE equation differs (forward butterfly's `z * b`
   asymmetry vs inverse's `(a - b) * z` upper-half). *)
private
let lemma_ntt_layer_3_step_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0: i16)
    (i: nat {i < 16}) :
  Lemma
    (requires
      TS.ntt_layer_3_step_post in_arr zeta0 out_arr)
    (ensures
      (let zs = zetas_1_ zeta0 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = N.ntt_layer_n (mk_usize 16) p_fe (mk_usize 8)
                                (Rust_primitives.unsize zs) in
       Seq.index r_fe i == Seq.index rhs i))
  = let zs = zetas_1_ zeta0 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    let b : nat = (i % 8) / 2 in
    assert (b < 4);
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_layer_3_step_branch_post bb in_arr zeta0 out_arr));
    assert (TS.ntt_layer_3_step_branch_post b in_arr zeta0 out_arr);
    reveal_opaque (`%TS.ntt_layer_3_step_branch_post)
                  (TS.ntt_layer_3_step_branch_post b in_arr zeta0 out_arr);
    lemma_ntt_layer_n_16_8_lane p_fe zs i;
    zetas_1_lane zeta0 (sz 0);
    mont_array_lane out_arr (sz i);
    mont_array_lane in_arr (sz i);
    if i < 8 then begin
      assert (i + 8 < 16);
      mont_array_lane in_arr (sz (i + 8))
    end else begin
      assert (i >= 8);
      mont_array_lane in_arr (sz (i - 8))
    end

#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Per-vector hacspec bridge for `f_ntt_layer_3_step`.

   Mirror of `lemma_ntt_layer_1_step_to_hacspec` for layer 3.
   Composes the 16 per-lane bridges via `Classical.forall_intro` +
   `Seq.lemma_eq_intro`. *)
let lemma_ntt_layer_3_step_to_hacspec
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0: i16) :
  Lemma
    (requires TS.ntt_layer_3_step_pre (T.f_repr vec) zeta0)
    (ensures
       (let r = T.f_ntt_layer_3_step vec zeta0 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r) ==
          N.ntt_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (sz 16) (T.f_repr vec))
            (mk_usize 8)
            (Rust_primitives.unsize (zetas_1_ zeta0))))
  = let r = T.f_ntt_layer_3_step vec zeta0 in
    let in_arr = T.f_repr vec in
    let out_arr = T.f_repr r in
    let zs = zetas_1_ zeta0 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    let rhs = N.ntt_layer_n (mk_usize 16) p_fe (mk_usize 8)
                             (Rust_primitives.unsize zs) in
    assert (TS.ntt_layer_3_step_post in_arr zeta0 out_arr);
    let aux (j: nat) : Lemma (j < 16 ==> Seq.index r_fe j == Seq.index rhs j)
      = if j < 16 then
          lemma_ntt_layer_3_step_lane_bridge in_arr out_arr zeta0 j
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro r_fe rhs

#pop-options

(*** Phase 7a (track A) — Layer 4_plus chunk-pair hacspec bridge ***)

(* `inv_ntt_layer_int_vec_step_reduce` (above the trait, in
   `src/invert_ntt.rs`) operates on a CHUNK-PAIR (two `vV`-vectors) rather
   than within a single chunk.  Its strengthened post (Step 3.1) gives
   per-lane FE equations matching `IN.inv_butterfly`.  This bridge lifts
   those per-lane equations to the function-form `IN.inv_butterfly`
   citation lane-wise — a pure unfold of `inv_butterfly`.

   Structurally simpler than layers 1/2/3: no nested if-ladder branch_post,
   no per-branch helpers, no `--split_queries always`.  Caller chains 16
   per-lane uses across each chunk pair to build the polynomial-level
   `IN.ntt_inverse_layer_n 256` claim in
   `invert_ntt_at_layer_4_plus`'s post (Step 3.3 / Step 4 layer 4_plus). *)
let lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec
    (a_arr b_arr r0_arr r1_arr: t_Array i16 (mk_usize 16))
    (zeta_r: i16) :
  Lemma
    (requires
       (forall (i: nat). i < 16 ==>
          mont_i16_to_spec_fe (Seq.index r0_arr i) ==
          P.impl_FieldElement__add
            (mont_i16_to_spec_fe (Seq.index a_arr i))
            (mont_i16_to_spec_fe (Seq.index b_arr i))) /\
       (forall (i: nat). i < 16 ==>
          mont_i16_to_spec_fe (Seq.index r1_arr i) ==
          P.impl_FieldElement__mul
            (mont_i16_to_spec_fe zeta_r)
            (P.impl_FieldElement__sub
              (mont_i16_to_spec_fe (Seq.index b_arr i))
              (mont_i16_to_spec_fe (Seq.index a_arr i)))))
    (ensures
       (forall (i: nat). i < 16 ==>
          mont_i16_to_spec_fe (Seq.index r0_arr i) ==
          (IN.inv_butterfly (mont_i16_to_spec_fe zeta_r)
                             (mont_i16_to_spec_fe (Seq.index a_arr i))
                             (mont_i16_to_spec_fe (Seq.index b_arr i)))._1) /\
       (forall (i: nat). i < 16 ==>
          mont_i16_to_spec_fe (Seq.index r1_arr i) ==
          (IN.inv_butterfly (mont_i16_to_spec_fe zeta_r)
                             (mont_i16_to_spec_fe (Seq.index a_arr i))
                             (mont_i16_to_spec_fe (Seq.index b_arr i)))._2))
  = ()

(* ───── Layer 2 forward NTT bridge ─────
   STATUS: layer 1 forward + inverse, layer 2 inverse, and layer 3 inverse
   bridges are done above (track A, Phase 7a).  Layer 2 forward remains.
   Same pattern: 4 per-branch helper lemmas at concrete `b` to collapse
   the nested if-ladder in the trait branch_post, plus a per-vector
   composition via `Classical.forall_intro` + `Seq.lemma_eq_intro`. *)

(*** poly_to_spec = to_spec_poly_plain bridge
     Connects the extraction-side `Vector.Spec.poly_to_spec` (builds a flat
     i16 array via `f_to_i16_array` then lifts via `i16_to_spec_array`) to the
     commute-side `to_spec_poly_plain` (lifts directly via `f_repr` + `i16_to_spec_fe`).
     Both compute `createi 256 (fun j -> i16_to_spec_fe (f_repr(p.coeffs[j/16])[j%16]))`.
     Used by `deserialize_then_decompress_u` loop invariant maintenance:
     the invariant is stated with `poly_to_spec` but `ntt_vector_u` postcondition
     uses `to_spec_poly_plain`. ***)

module VS = Libcrux_ml_kem.Vector.Spec
module VV = Libcrux_ml_kem.Vector

(* Bridge: poly_to_spec and to_spec_poly_plain compute the same lift.
   Proof: per-lane, VS.poly_to_spec_index gives lhs[j] and poly_lane_plain
   gives rhs[j]; both equal i16_to_spec_fe (f_repr(p.coeffs[j/16])[j%16]). *)
#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"
let poly_to_spec_eq_to_spec_poly_plain
    (#vV: Type0) {| i0: T.t_Operations vV |}
    (p: VV.t_PolynomialRingElement vV)
  : Lemma
    (VS.poly_to_spec #vV p == to_spec_poly_plain #vV p)
  = let lhs = VS.poly_to_spec #vV p in
    let rhs = to_spec_poly_plain #vV p in
    let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
      VS.poly_to_spec_index #vV p j;  (* lhs[j] = i16_to_spec_fe (f_repr(p.coeffs[j/16])[j%16]) *)
      poly_lane_plain #vV #i0 p j     (* rhs[j] = same *)
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro lhs rhs
#pop-options

(*** USER-14 — Layer 4+ cross-vector inverse-NTT composition bridge ***)
(* Authored 2026-05-30.  Lifts the impl's per-vector-pair `inv_butterfly`
   operations (cross-vector partners j and j+step_vec) to the spec's
   per-coefficient 256-element `IN.ntt_inverse_layer_n`, for layers 4..7
   (len = 2^layer ∈ {16,32,64,128}, step_vec = len/16, groups = 128/len).
   This is the missing composition that `invert_ntt_at_layer_4_plus`'s
   strengthened post needs (and that the existing intra-vector
   `lemma_ntt_inverse_layer_n_16_*_lane` helpers do NOT provide, since
   for layers 1..3 the butterfly partner stays within a single 16-lane
   vector whereas for layers 4..7 it crosses vector boundaries).

   Index correspondence (re-derived & F*-checked in `lemma_cross_idx`):
   poly coeff i ∈ [0,256) lives in vector m = i/16, lane l = i%16;
   group = i/(2·len) = m/(2·step_vec); idx = i%(2·len) = 16·(m%(2·step_vec))+l;
   idx < len  ⟺  m%(2·step_vec) < step_vec;  partner i±len ↔ vector m±step_vec
   (same lane l).  Two levels:
     LEVEL A  lemma_ntt_inverse_layer_n_256_compose — array-level createi
              unfold (per-coefficient inv_butterfly relations ⇒
              q == ntt_inverse_layer_n 256 p len zetas).
     LEVEL B  lemma_layer_4_plus_cross_vector — the cross-vector bridge,
              reduces the per-vector hypothesis to Level A's per-coefficient
              form via the index algebra and discharges Level A.
   See also the (drafted, not-yet-validated) zeta-table reconstruction
   `lemma_ntt_inverse_layer_unfold` that connects ntt_inverse_layer_n to
   ntt_inverse_layer (the table-building 256 form the post cites). *)

(* === LEVEL A: array-level createi composition for ntt_inverse_layer_n 256 === *)
#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"
let lemma_ntt_inverse_layer_n_256_compose
    (p q: t_Array P.t_FieldElement (mk_usize 256))
    (len: usize)
    (zetas: t_Slice P.t_FieldElement)
  : Lemma
    (requires
      v len >= 1 /\ v len < 1024 /\
      Seq.length zetas < 1024 /\
      2 * Seq.length zetas * v len == 256 /\
      (forall (i: nat). i < 256 ==>
        (let group : nat = i / (2 * v len) in
         let idx   : nat = i % (2 * v len) in
         group < Seq.length zetas /\
         (idx < v len ==>
            i + v len < 256 /\
            Seq.index q i ==
              (IN.inv_butterfly (Seq.index zetas group) (Seq.index p i) (Seq.index p (i + v len)))._1) /\
         (idx >= v len ==>
            i >= v len /\
            Seq.index q i ==
              (IN.inv_butterfly (Seq.index zetas group) (Seq.index p (i - v len)) (Seq.index p i))._2))))
    (ensures
      q == IN.ntt_inverse_layer_n (mk_usize 256) p len zetas)
  = let rhs = IN.ntt_inverse_layer_n (mk_usize 256) p len zetas in
    let aux (i: nat) : Lemma (i < 256 ==> Seq.index q i == Seq.index rhs i)
      = if i < 256 then begin
          let group : nat = i / (2 * v len) in
          assert (group < Seq.length zetas);
          assert (Seq.index rhs i ==
            (let g : usize = (sz i) /! (mk_usize 2 *! len <: usize) in
             let idx : usize = (sz i) %! (mk_usize 2 *! len <: usize) in
             if idx <. len
             then (IN.inv_butterfly (Seq.index zetas (v g)) (Seq.index p i) (Seq.index p (i + v len)))._1
             else (IN.inv_butterfly (Seq.index zetas (v g)) (Seq.index p (i - v len)) (Seq.index p i))._2))
        end
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro q rhs
#pop-options

(* Per-lane unfold for to_spec_poly_mont_arr (mirror of mont_to_spec_poly_256_lane). *)
let tspm_arr_lane
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (a: t_Array vV (mk_usize 16)) (j: nat { j < 256 }) :
    Lemma (Seq.index (to_spec_poly_mont_arr #vV a) j
           == mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index a (j / 16))) (j % 16)))
  = P.createi_lemma #P.t_FieldElement (mk_usize 256)
      #(usize -> P.t_FieldElement)
      (fun (k: usize { k <. mk_usize 256 }) ->
        (mont_i16_to_spec_fe
          (Seq.index (T.f_repr (Seq.index a (v k / 16))) (v k % 16))
         <: P.t_FieldElement))
      (sz j)

(* === Index-decomposition helper (clean nonlinear context) === *)
#push-options "--z3rlimit 300 --fuel 0 --ifuel 0"
let lemma_cross_idx (i: nat{i < 256}) (s: pos{s == 1 \/ s == 2 \/ s == 4 \/ s == 8})
  : Lemma
    (let m = i / 16 in let l = i % 16 in let len = 16 * s in
     m < 16 /\ l < 16 /\ i == 16 * m + l /\
     i / (2 * len) == m / (2 * s) /\
     i % (2 * len) == 16 * (m % (2 * s)) + l /\
     ((i % (2 * len)) < len <==> (m % (2 * s)) < s))
  = let m = i / 16 in let l = i % 16 in let len = 16 * s in
    FStar.Math.Lemmas.euclidean_division_definition i 16;
    FStar.Math.Lemmas.euclidean_division_definition m (2 * s);
    let q = m / (2 * s) in
    let r = m % (2 * s) in
    assert (m == (2 * s) * q + r);
    assert (i == (32 * s) * q + (16 * r + l));
    assert (16 * r + l < 32 * s);
    FStar.Math.Lemmas.lemma_div_plus (16 * r + l) q (32 * s);
    FStar.Math.Lemmas.lemma_mod_plus (16 * r + l) q (32 * s);
    FStar.Math.Lemmas.small_div (16 * r + l) (32 * s);
    FStar.Math.Lemmas.small_mod (16 * r + l) (32 * s)
#pop-options

(* Partner-index helper: for i<256, l=i%16<16, s pos with m+s<16 (resp m>=s),
   (i + 16*s)/16 == i/16 + s and (i+16*s)%16 == i%16 (resp for subtraction). *)
#push-options "--z3rlimit 200 --fuel 0 --ifuel 0"
let lemma_partner_idx_add (i: nat) (s: pos)
  : Lemma (requires i % 16 < 16)
          (ensures (i + 16 * s) / 16 == i / 16 + s /\ (i + 16 * s) % 16 == i % 16)
  = FStar.Math.Lemmas.euclidean_division_definition i 16;
    FStar.Math.Lemmas.lemma_div_plus i s 16;
    FStar.Math.Lemmas.lemma_mod_plus i s 16
let lemma_partner_idx_sub (i: nat) (s: pos)
  : Lemma (requires i >= 16 * s)
          (ensures (i - 16 * s) / 16 == i / 16 - s /\ (i - 16 * s) % 16 == i % 16)
  = FStar.Math.Lemmas.euclidean_division_definition i 16;
    FStar.Math.Lemmas.lemma_div_plus (i - 16 * s) s 16;
    FStar.Math.Lemmas.lemma_mod_plus (i - 16 * s) s 16
#pop-options

(* Enumerated arithmetic helper: for the four layer lengths, 2*(128/x)*x==256. *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 0"
let lemma_div_128_prod (x: nat)
  : Lemma (requires x == 16 \/ x == 32 \/ x == 64 \/ x == 128)
          (ensures 2 * (128 / x) * x == 256)
  = ()
#pop-options

(* === LEVEL B: cross-vector index bridge ===
   step_vec = len/16, groups = 128/len; vector m lane l holds poly coeff 16m+l;
   block = m/(2*step_vec), pos = m%(2*step_vec).  Low half (pos<step_vec): SUM
   with partner m+step_vec; high half: MUL with partner m-step_vec; zeta zs[block].
   Concludes the FE-array equation against ntt_inverse_layer_n 256. *)
(* Flat per-vector requires predicate (no nested forall, NOT unfold so the
   requires forall stays compact — Z3 instantiates one flat quantifier at
   (i/16, i%16); the body is revealed only at the single instantiation site). *)
[@@ "opaque_to_smt"]
let cross_vec_hyp
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16)) (step_vec: pos) (zs: t_Slice P.t_FieldElement)
    (m: nat) (l: nat) : prop =
  (m < 16 /\ l < 16) ==>
    (let block : nat = m / (2 * step_vec) in
     let pos   : nat = m % (2 * step_vec) in
     block < Seq.length zs /\
     (pos < step_vec ==>
        m + step_vec < 16 /\
        mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cout m)) l) ==
          (IN.inv_butterfly (Seq.index zs block)
             (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin m)) l))
             (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin (m + step_vec))) l)))._1) /\
     (pos >= step_vec ==>
        m >= step_vec /\
        mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cout m)) l) ==
          (IN.inv_butterfly (Seq.index zs block)
             (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin (m - step_vec))) l))
             (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin m)) l)))._2))

(* Standalone bridge lemma (skill "standalone bridge lemma" pattern): builds
   exactly Level A's per-coefficient hypothesis from the per-vector cross_vec_hyp.
   Factored out of lemma_layer_4_plus_cross_vector so the Classical.forall_intro
   WP is isolated from the lemma_ntt_inverse_layer_n_256_compose call — the fused
   form was a ~6-8 min monolithic query.  This lemma's ensures IS Level A's
   requires, so the consumer's compose call discharges trivially from this post. *)
#push-options "--z3rlimit 300 --fuel 0 --ifuel 1"
let lemma_layer_4_plus_per_coeff
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16))
    (len: usize)
    (zs: t_Slice P.t_FieldElement)
  : Lemma
    (requires
      (v len == 16 \/ v len == 32 \/ v len == 64 \/ v len == 128) /\
      Seq.length zs == 128 / v len /\
      (forall (m: nat) (l: nat).
         cross_vec_hyp #vV cin cout (v len / 16) zs m l))
    (ensures
      (let p = to_spec_poly_mont_arr #vV cin in
       let q = to_spec_poly_mont_arr #vV cout in
       (forall (i: nat). i < 256 ==>
         (let group : nat = i / (2 * v len) in
          let idx   : nat = i % (2 * v len) in
          group < Seq.length zs /\
          (idx < v len ==>
             i + v len < 256 /\
             Seq.index q i ==
               (IN.inv_butterfly (Seq.index zs group) (Seq.index p i) (Seq.index p (i + v len)))._1) /\
          (idx >= v len ==>
             i >= v len /\
             Seq.index q i ==
               (IN.inv_butterfly (Seq.index zs group) (Seq.index p (i - v len)) (Seq.index p i))._2)))))
  = (* establish step_vec ∈ {1,2,4,8} from the len disjunction *)
    assert (v len / 16 == 1 \/ v len / 16 == 2 \/ v len / 16 == 4 \/ v len / 16 == 8);
    let step_vec : s:pos{s == 1 \/ s == 2 \/ s == 4 \/ s == 8} = v len / 16 in
    let p = to_spec_poly_mont_arr #vV cin in
    let q = to_spec_poly_mont_arr #vV cout in
    let aux (i: nat) : Lemma (i < 256 ==>
        (let group : nat = i / (2 * v len) in
         let idx   : nat = i % (2 * v len) in
         group < Seq.length zs /\
         (idx < v len ==>
            i + v len < 256 /\
            Seq.index q i ==
              (IN.inv_butterfly (Seq.index zs group) (Seq.index p i) (Seq.index p (i + v len)))._1) /\
         (idx >= v len ==>
            i >= v len /\
            Seq.index q i ==
              (IN.inv_butterfly (Seq.index zs group) (Seq.index p (i - v len)) (Seq.index p i))._2)))
      = if i < 256 then begin
          let m : nat = i / 16 in
          let l : nat = i % 16 in
          lemma_cross_idx i step_vec;
          (* now: m<16, l<16, i=16m+l, i/(2len)=m/(2s), i%(2len)<len <==> m%(2s)<s *)
          (* reveal the opaque hypothesis at this single (m,l) instantiation *)
          assert (cross_vec_hyp #vV cin cout step_vec zs m l);
          reveal_opaque (`%cross_vec_hyp)
            (cross_vec_hyp #vV cin cout step_vec zs m l);
          let block : nat = m / (2 * step_vec) in
          let pos   : nat = m % (2 * step_vec) in
          tspm_arr_lane #vV cout i;       (* q[i] = mont(cout[m])[l] *)
          tspm_arr_lane #vV cin i;        (* p[i] = mont(cin[m])[l] *)
          if pos < step_vec then begin
            (* m+step_vec<16 from the requires; partner = vector m+step_vec, lane l *)
            lemma_partner_idx_add i step_vec;   (* (i+len)/16 = m+s, (i+len)%16 = l *)
            tspm_arr_lane #vV cin (i + v len)
          end else begin
            lemma_partner_idx_sub i step_vec;   (* (i-len)/16 = m-s, (i-len)%16 = l *)
            tspm_arr_lane #vV cin (i - v len)
          end
        end
    in
    Classical.forall_intro aux
#pop-options

#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"
let lemma_layer_4_plus_cross_vector
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16))
    (len: usize)
    (zs: t_Slice P.t_FieldElement)
  : Lemma
    (requires
      (v len == 16 \/ v len == 32 \/ v len == 64 \/ v len == 128) /\
      Seq.length zs == 128 / v len /\
      (forall (m: nat) (l: nat).
         cross_vec_hyp #vV cin cout (v len / 16) zs m l))
    (ensures
      to_spec_poly_mont_arr #vV cout ==
        IN.ntt_inverse_layer_n (mk_usize 256) (to_spec_poly_mont_arr #vV cin) len zs)
  = let p = to_spec_poly_mont_arr #vV cin in
    let q = to_spec_poly_mont_arr #vV cout in
    (* standalone lemma supplies exactly Level A's per-coefficient requires forall *)
    lemma_layer_4_plus_per_coeff #vV cin cout len zs;
    (* len ∈ {16,32,64,128}; 2*(128/len)*len == 256 via the enumerated helper. *)
    lemma_div_128_prod (v len);
    lemma_ntt_inverse_layer_n_256_compose p q len zs
#pop-options

(* === USER-14 zeta correspondence axiom (user-approved Option B, 2026-05-30) ===
   The impl Montgomery zeta `Libcrux_ml_kem.Polynomial.zeta` is exposed to clients
   as an `assume val` with a BOUND-ONLY postcondition (result in [-1664,1664]); its
   concrete value is opaque cross-module.  This axiom records its correspondence to
   the spec plain zeta table `N.v_ZETAS`, which is validated at runtime by
   `ntt_matches_spec` / `full_ntt_multiply_chain_matches_spec` in `src/ntt.rs`.
   Needed by the table-form posts of `invert_ntt_at_layer_4_plus` and (downstream)
   `invert_ntt_montgomery` (USER-15), since `IN.ntt_inverse_layer` consumes concrete
   `v_ZETAS` whereas the impl butterflies use `mont_i16_to_spec_fe (zeta …)`. *)
assume
val lemma_zeta_eq_vzetas (k: usize)
  : Lemma (requires v k < 128)
          (ensures mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta k) == N.v_ZETAS.[ k ])

(* === USER-14 unfold lemma: table-building `IN.ntt_inverse_layer` -> explicit
   `IN.ntt_inverse_layer_n` for layers 4..7, against a caller-supplied zeta slice
   `zs` whose entries match the spec's internal table `v_ZETAS[2*groups-1-round]`.
   Purely structural (no zeta correspondence inside): unfolds the table-building
   definition and shows its internal slice equals `zs` point-wise (createi_lemma +
   lemma_index_slice), then concludes by congruence of `ntt_inverse_layer_n`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_ntt_inverse_layer_unfold
    (p: t_Array P.t_FieldElement (mk_usize 256))
    (layer: usize)
    (zs: t_Slice P.t_FieldElement)
  : Lemma
    (requires
      (v layer == 4 \/ v layer == 5 \/ v layer == 6 \/ v layer == 7) /\
      Seq.length zs == 128 / (pow2 (v layer)) /\
      (let groups = 128 / pow2 (v layer) in
       forall (round: nat). round < groups ==>
         Seq.index zs round == N.v_ZETAS.[ sz (2 * groups - 1 - round) ]))
    (ensures
      IN.ntt_inverse_layer p layer ==
        IN.ntt_inverse_layer_n (mk_usize 256) p (mk_usize 1 <<! layer) zs)
  = let len : usize = mk_usize 1 <<! layer in
    let groups : usize = mk_usize 128 /! len in
    assert (v len == pow2 (v layer));
    assert (v groups == 128 / pow2 (v layer));
    let zetas_tbl : t_Array P.t_FieldElement (mk_usize 128) =
      P.createi #P.t_FieldElement (mk_usize 128)
        #(usize -> P.t_FieldElement)
        (fun round ->
          if round <. groups
          then N.v_ZETAS.[ (mk_usize 2 *! groups -! mk_usize 1) -! round ]
          else P.impl_FieldElement__new (mk_u16 0))
    in
    let tbl_slice : t_Slice P.t_FieldElement =
      zetas_tbl.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                    Core_models.Ops.Range.f_end = groups } ] in
    (* FACT 1: ntt_inverse_layer unfolds (definitionally) to ntt_inverse_layer_n on
       the table slice.  Proved by normalization + reflexivity (no SMT search). *)
    assert (IN.ntt_inverse_layer p layer ==
            IN.ntt_inverse_layer_n (mk_usize 256) p len tbl_slice)
      by (FStar.Tactics.norm [delta_only [`%IN.ntt_inverse_layer]; iota; zeta; primops];
          FStar.Tactics.trefl ());
    (* FACT 2: tbl_slice == zs, proven point-wise (createi_lemma + lemma_index_slice).
       Then SMT concludes the goal by congruence of ntt_inverse_layer_n on tbl_slice == zs. *)
    assert (Seq.length tbl_slice == v groups);
    let aux (i: nat) : Lemma (i < v groups ==> Seq.index tbl_slice i == Seq.index zs i)
      = if i < v groups then begin
          FStar.Seq.Base.lemma_index_slice zetas_tbl 0 (v groups) i;
          assert (sz i <. groups);
          P.createi_lemma #P.t_FieldElement (mk_usize 128)
            #(usize -> P.t_FieldElement)
            (fun round ->
              ((if round <. groups
                then N.v_ZETAS.[ (mk_usize 2 *! groups -! mk_usize 1) -! round ]
                else P.impl_FieldElement__new (mk_u16 0)) <: P.t_FieldElement))
            (sz i);
          (* index arithmetic: the table index equals the zs requires-index *)
          assert (v ((mk_usize 2 *! groups -! mk_usize 1) -! sz i) == 2 * v groups - 1 - i)
        end
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro tbl_slice zs
#pop-options

(* === USER-14 end-to-end chain test (validates Option B): assuming the loop
   produced the per-vector cross_vec_hyp, the function's strengthened post
   (table-form `IN.ntt_inverse_layer`) follows by composing the verified
   lemmas: cross-vector bridge -> unfold -> to_spec_poly_mont_unfold.  This is
   exactly the post-loop wiring `invert_ntt_at_layer_4_plus` needs; the only
   remaining work is establishing the cross_vec_hyp hypothesis from the double
   loop's invariants. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_layer_4_plus_post_from_cross_vec
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
    (layer len: usize)
    (step_vec: pos)
    (zs: t_Slice P.t_FieldElement)
  : Lemma
    (requires
      (v layer == 4 \/ v layer == 5 \/ v layer == 6 \/ v layer == 7) /\
      v len == pow2 (v layer) /\
      v len == 16 * step_vec /\
      Seq.length zs == 128 / pow2 (v layer) /\
      (let groups = 128 / pow2 (v layer) in
       forall (round: nat). round < groups ==>
         Seq.index zs round == N.v_ZETAS.[ sz (2 * groups - 1 - round) ]) /\
      (forall (m: nat) (l: nat).
         cross_vec_hyp #vV re_in.VV.f_coefficients re_out.VV.f_coefficients step_vec zs m l))
    (ensures
      to_spec_poly_mont #vV re_out ==
        IN.ntt_inverse_layer (to_spec_poly_mont #vV re_in) layer)
  = assert_norm (pow2 4 == 16 /\ pow2 5 == 32 /\ pow2 6 == 64 /\ pow2 7 == 128);
    assert (v len == 16 \/ v len == 32 \/ v len == 64 \/ v len == 128);
    assert (v len / 16 == step_vec);
    lemma_to_spec_poly_mont_unfold #vV re_out;
    lemma_to_spec_poly_mont_unfold #vV re_in;
    lemma_layer_4_plus_cross_vector #vV re_in.VV.f_coefficients re_out.VV.f_coefficients len zs;
    lemma_ntt_inverse_layer_unfold (to_spec_poly_mont_arr #vV re_in.VV.f_coefficients) layer zs
#pop-options

(* === USER-14 keystone: from one inv_ntt_layer_int_vec_step_reduce step (vectors
   j and j+step_vec, low/high halves of block = j/(2*step_vec)), establish
   cross_vec_hyp for BOTH written vectors.  This is the per-inner-iteration fact
   the body's loop accumulates.  `cout` is `cin` updated at j and j+step_vec; the
   two requires foralls are exactly the per-step bridge's ensures (in f_repr form,
   with a = cin[j], b = cin[j+step_vec], r0 = cout[j], r1 = cout[j+step_vec]). *)
(* Clean (fuel0/ifuel0) nonlinear index helper: partner j+sv sits in the same
   block as j (when j is in the low half) and falls in the high half. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_vec_partner_hi (j: nat) (sv: pos)
  : Lemma (requires j % (2 * sv) < sv)
          (ensures (j + sv) / (2 * sv) == j / (2 * sv) /\
                   (j + sv) % (2 * sv) == j % (2 * sv) + sv /\
                   j % (2 * sv) + sv >= sv)
  = let block = j / (2 * sv) in
    let pos = j % (2 * sv) in
    FStar.Math.Lemmas.euclidean_division_definition j (2 * sv);
    assert (j + sv == block * (2 * sv) + (pos + sv));
    FStar.Math.Lemmas.small_div (pos + sv) (2 * sv);
    FStar.Math.Lemmas.small_mod (pos + sv) (2 * sv);
    FStar.Math.Lemmas.lemma_div_plus (pos + sv) block (2 * sv);
    FStar.Math.Lemmas.lemma_mod_plus (pos + sv) block (2 * sv)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_cross_vec_from_step
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16))
    (step_vec: pos)
    (zs: t_Slice P.t_FieldElement)
    (j: nat)
    (zeta_r: i16)
  : Lemma
    (requires
      j + step_vec < 16 /\
      j % (2 * step_vec) < step_vec /\
      j / (2 * step_vec) < Seq.length zs /\
      Seq.index zs (j / (2 * step_vec)) == mont_i16_to_spec_fe zeta_r /\
      (forall (l: nat). l < 16 ==>
         mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cout j)) l) ==
           (IN.inv_butterfly (mont_i16_to_spec_fe zeta_r)
              (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin j)) l))
              (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin (j + step_vec))) l)))._1) /\
      (forall (l: nat). l < 16 ==>
         mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cout (j + step_vec))) l) ==
           (IN.inv_butterfly (mont_i16_to_spec_fe zeta_r)
              (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin j)) l))
              (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin (j + step_vec))) l)))._2))
    (ensures
      (forall (l: nat). l < 16 ==> cross_vec_hyp #vV cin cout step_vec zs j l) /\
      (forall (l: nat). l < 16 ==> cross_vec_hyp #vV cin cout step_vec zs (j + step_vec) l))
  = (* low half (m = j): block = j/(2sv) < len, pos < sv, partner = j+step_vec
       referenced directly -- no nonlinear index reasoning needed. *)
    let aux_lo (l: nat) : Lemma (cross_vec_hyp #vV cin cout step_vec zs j l)
      = reveal_opaque (`%cross_vec_hyp) (cross_vec_hyp #vV cin cout step_vec zs j l)
    in
    Classical.forall_intro aux_lo;
    (* high half (m = j+step_vec): needs block'==block, pos'>=sv, m-sv==j. *)
    lemma_vec_partner_hi j step_vec;
    let aux_hi (l: nat) : Lemma (cross_vec_hyp #vV cin cout step_vec zs (j + step_vec) l)
      = reveal_opaque (`%cross_vec_hyp) (cross_vec_hyp #vV cin cout step_vec zs (j + step_vec) l)
    in
    Classical.forall_intro aux_hi
#pop-options

(* === USER-14 frame lemma: cross_vec_hyp reads `cout` only at index m, so an update
   to `cout` at indices other than m preserves it.  Lets the body's loop carry the
   already-done vectors' cross_vec_hyp across each Seq.upd of the two written vectors. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_cross_vec_frame
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout1 cout2: t_Array vV (mk_usize 16))
    (step_vec: pos)
    (zs: t_Slice P.t_FieldElement)
    (m l: nat)
  : Lemma
    (requires m < 16 /\ Seq.index cout1 m == Seq.index cout2 m)
    (ensures cross_vec_hyp #vV cin cout1 step_vec zs m l <==>
             cross_vec_hyp #vV cin cout2 step_vec zs m l)
  = reveal_opaque (`%cross_vec_hyp) (cross_vec_hyp #vV cin cout1 step_vec zs m l);
    reveal_opaque (`%cross_vec_hyp) (cross_vec_hyp #vV cin cout2 step_vec zs m l)
#pop-options

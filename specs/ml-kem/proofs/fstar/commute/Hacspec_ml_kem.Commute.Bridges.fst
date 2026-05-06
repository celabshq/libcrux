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

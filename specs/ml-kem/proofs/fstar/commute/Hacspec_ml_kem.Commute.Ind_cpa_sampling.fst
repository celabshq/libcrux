module Hacspec_ml_kem.Commute.Ind_cpa_sampling

/// Sampling bridges for the ind_cpa CBD samplers
/// (`sample_ring_element_cbd`, `sample_vector_cbd_then_ntt`).
///
/// The impl loops `error_1[i] = sample_from_binomial_distribution eta prf_outputs[i]`
/// where `prf_outputs == Spec.Utils.v_PRFxN K (eta*64) prf_inputs` and each
/// `prf_inputs[i]` is `prf_input.clone()` with byte 32 bumped to `ds0 + i`
/// (the `prf_input_inc` post).  The spec is
///   `sample_vector_cbd K eta seed ds0 = createi K (fun i ->
///        sample_secret eta (concat_byte (try_into seed) (ds0 + cast i)))`
/// and `sample_secret eta prf = sample_poly_cbd (eta*64) (eta*512) eta (HF.v_PRF (eta*64) prf)`.
///
/// PER-INDEX chain (proven by `lemma_per_index_cbd`, for index i):
///   poly_to_spec error_1[i]
///     == sample_poly_cbd (eta*64) (eta*512) eta prf_outputs[i]              [CBD post — hypothesis]
///     == sample_poly_cbd (eta*64) (eta*512) eta (v_PRFxN K (eta*64) prf_inputs)[i]
///                                                                            [v_PRFxN trait post — hypothesis]
///     == sample_poly_cbd (eta*64) (eta*512) eta (SU.v_PRF (eta*64) prf_inputs[i])
///                                                                            [Prf_bridge.lemma_prfxn_pointwise]
///     == sample_poly_cbd (eta*64) (eta*512) eta (HF.v_PRF (eta*64) prf_inputs[i])
///                                                                            [Prf_bridge.lemma_prf_identification]
///     == sample_secret eta prf_inputs[i]                                     [def sample_secret]
///     == sample_secret eta (concat_byte (try_into seed) (ds0 + cast i))      [33-byte extensionality]
///     == (sample_vector_cbd K eta seed ds0)[i]                               [createi index]
/// FINALIZE (`lemma_sample_vector_cbd_finalize`):
///   vector_to_spec error_1 == sample_vector_cbd K eta seed ds0
///     via vector_to_spec_index + Seq.lemma_eq_intro over the per-index facts.

#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

module HI  = Hacspec_ml_kem.Ind_cpa
module HS  = Hacspec_ml_kem.Sampling
module P   = Hacspec_ml_kem.Parameters
module HF  = Hacspec_ml_kem.Parameters.Hash_functions
module SU  = Spec.Utils
module VS  = Libcrux_ml_kem.Vector.Spec
module VV  = Libcrux_ml_kem.Vector
module PB  = Hacspec_ml_kem.Commute.Prf_bridge
module U   = Rust_primitives.Hax.Monomorphized_update_at
module N   = Hacspec_ml_kem.Ntt
module Br  = Hacspec_ml_kem.Commute.Bridges
module CH  = Hacspec_ml_kem.Commute.Chunk

(* ------------------------------------------------------------------ *)
(* STRUCTURAL: the prf_input_inc post + clone-is-identity + repeat      *)
(* gives, for every j<K, the per-index facts the CBD bridge consumes.   *)
(*  prf_inputs == fst (prf_input_inc K (repeat (clone prf_input) K) ds0) *)
(*  is captured here by taking the prf_input_inc post forall as a        *)
(*  hypothesis (it mentions `repeat (clone prf_input) K` as the pre).    *)
(* Proven in clean context so the heavy ind_cpa VC just calls it.        *)
(* ------------------------------------------------------------------ *)
#push-options "--z3rlimit 150"
let lemma_prf_inputs_struct
      (v_K: usize{v v_K <= 4})
      (prf_input: t_Array u8 (mk_usize 33))
      (prf_inputs: t_Array (t_Array u8 (mk_usize 33)) v_K)
      (ds0: u8)
    : Lemma
      (requires
        (forall (i: nat).
           i < v v_K ==>
           v (Seq.index (Seq.index prf_inputs i) 32) == v ds0 + i /\
           Seq.slice (Seq.index prf_inputs i) 0 32 ==
           Seq.slice (Seq.index (Rust_primitives.Hax.repeat
                        (Core_models.Clone.f_clone #(t_Array u8 (mk_usize 33)) prf_input) v_K) i)
                      0 32))
      (ensures
        (forall (j: nat).
           j < v v_K ==>
           v (Seq.index (Seq.index prf_inputs j) 32) == v ds0 + j /\
           Seq.slice (Seq.index prf_inputs j) 0 32 == Seq.slice prf_input 0 32))
  = let cl = Core_models.Clone.f_clone #(t_Array u8 (mk_usize 33)) prf_input in
    assert (cl == prf_input);
    let aux (j: nat{j < v v_K}) : Lemma
      (v (Seq.index (Seq.index prf_inputs j) 32) == v ds0 + j /\
       Seq.slice (Seq.index prf_inputs j) 0 32 == Seq.slice prf_input 0 32) =
      FStar.Seq.Base.lemma_index_create (v v_K) cl j
    in
    Classical.forall_intro aux
#pop-options

(* ------------------------------------------------------------------ *)
(* 33-byte extensionality: prf_inputs[i] == concat_byte 32 33 seed32 b *)
(* concat_byte builds a 33-byte array whose [0..32] = a and [32] = b.  *)
(* ------------------------------------------------------------------ *)
#push-options "--z3rlimit 150"
let lemma_concat_byte_eq
      (a: t_Array u8 (mk_usize 32))
      (b: u8)
      (x: t_Array u8 (mk_usize 33))
    : Lemma
      (requires
        Seq.slice x 0 32 == (a <: Seq.seq u8) /\
        Seq.index x 32 == b)
      (ensures x == HI.concat_byte (mk_usize 32) (mk_usize 33) a b)
  = let c = HI.concat_byte (mk_usize 32) (mk_usize 33) a b in
    (* c[0..32] = a, c[32] = b by construction *)
    assert (Seq.length c == 33);
    assert (Seq.length x == 33);
    assert (Seq.slice c 0 32 == (a <: Seq.seq u8));
    assert (Seq.index c 32 == b);
    let aux (k: nat{k < 33}) : Lemma (Seq.index x k == Seq.index c k) =
      if k < 32 then begin
        Seq.lemma_index_slice x 0 32 k;
        Seq.lemma_index_slice c 0 32 k
      end
      else ()
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro x c
#pop-options

(* ------------------------------------------------------------------ *)
(* glue: try_into of a 32-byte slice is the identity                   *)
(* (mirrors Ind_cca_bridge.lemma_slice_to_array_id_32, but takes the   *)
(*  already-known equality `slc == (a:Seq.seq u8)`).                   *)
(* ------------------------------------------------------------------ *)
let lemma_try_into_id_32 (slc: t_Slice u8)
    : Lemma (requires Seq.length slc == 32)
            (ensures
              (Core_models.Result.impl__unwrap
                  #(t_Array u8 (mk_usize 32))
                  #Core_models.Array.t_TryFromSliceError
                  (Core_models.Convert.f_try_into #(t_Slice u8)
                      #(t_Array u8 (mk_usize 32))
                      #FStar.Tactics.Typeclasses.solve
                      slc)
                <: t_Array u8 (mk_usize 32))
              == slc)
  = Hacspec_ml_kem.Commute.Ind_cca_bridge.lemma_slice_to_array_id_32 slc

(* ------------------------------------------------------------------ *)
(* PER-INDEX bridge.                                                   *)
(*                                                                      *)
(* Hypotheses captured by the impl loop body at iteration i:            *)
(*  (cbd)   poly_to_spec error_1_i == sample_poly_cbd (eta*64)(eta*512) *)
(*                                      eta prf_outputs.[i]              *)
(*          (the sample_from_binomial_distribution post)                *)
(*  (prfxn) prf_outputs == SU.v_PRFxN K (eta*64) prf_inputs             *)
(*          (the f_PRFxN trait post)                                    *)
(*  (inp32) v (Seq.index prf_inputs.[i] 32) == v ds0 + i  AND           *)
(*          Seq.slice prf_inputs.[i] 0 32 == (seed : Seq.seq u8)        *)
(*          (the prf_input_inc post + prf_inputs init = prf_input.clone) *)
(*                                                                      *)
(* Conclusion: poly_to_spec error_1_i == (sample_vector_cbd ...).[i].   *)
(* ------------------------------------------------------------------ *)
#push-options "--z3rlimit 300 --split_queries always"
let lemma_per_index_cbd
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K == 2 \/ v v_K == 3 \/ v v_K == 4})
      (eta: usize{v eta == 2 \/ v eta == 3})
      (seed: t_Slice u8{Seq.length seed == 32})
      (ds0: u8{v ds0 + v v_K < 256})
      (len_rand: usize{v len_rand == v eta * 64})
      (prf_inputs: t_Array (t_Array u8 (mk_usize 33)) v_K)
      (prf_outputs: t_Array (t_Array u8 len_rand) v_K)
      (error_1_i: VV.t_PolynomialRingElement vV)
      (i: nat{i < v v_K})
    : Lemma
      (requires
        VS.poly_to_spec #vV error_1_i ==
          HS.sample_poly_cbd (eta *! mk_usize 64) (eta *! mk_usize 512) eta
            (prf_outputs.[ mk_usize i ] <: t_Array u8 len_rand) /\
        prf_outputs == SU.v_PRFxN v_K len_rand prf_inputs /\
        v (Seq.index (prf_inputs.[ mk_usize i ] <: t_Array u8 (mk_usize 33)) 32) == v ds0 + i /\
        Seq.slice (prf_inputs.[ mk_usize i ] <: t_Array u8 (mk_usize 33)) 0 32 == (seed <: Seq.seq u8))
      (ensures
        VS.poly_to_spec #vV error_1_i ==
        Seq.index (HI.sample_vector_cbd v_K eta seed ds0) i)
  = let len = len_rand in
    assert (v len < pow2 32);
    assert (len == eta *! mk_usize 64);
    let pin_i : t_Array u8 (mk_usize 33) = prf_inputs.[ mk_usize i ] in
    (* prf_outputs.[i] == SU.v_PRFxN ... .[i] == SU.v_PRF len pin_i *)
    PB.lemma_prfxn_pointwise v_K len prf_inputs i;
    assert ((prf_outputs.[ mk_usize i ] <: t_Array u8 len) == SU.v_PRF len (pin_i <: t_Slice u8));
    (* SU.v_PRF == HF.v_PRF *)
    PB.lemma_prf_identification len (pin_i <: t_Slice u8);
    assert (SU.v_PRF len (pin_i <: t_Slice u8) == HF.v_PRF len (pin_i <: t_Slice u8));
    (* therefore poly_to_spec error_1_i == sample_poly_cbd len (eta*512) eta (HF.v_PRF len pin_i) *)
    assert (VS.poly_to_spec #vV error_1_i ==
            HS.sample_poly_cbd len (eta *! mk_usize 512) eta (HF.v_PRF len (pin_i <: t_Slice u8)));
    (* sample_secret eta pin_i == sample_poly_cbd len (eta*512) eta (HF.v_PRF len pin_i) *)
    assert (HI.sample_secret eta pin_i ==
            HS.sample_poly_cbd len (eta *! mk_usize 512) eta (HF.v_PRF len (pin_i <: t_Slice u8)));
    (* pin_i == concat_byte 32 33 (try_into seed) (ds0 + cast i) *)
    let b : u8 = ds0 +! (cast (mk_usize i <: usize) <: u8) in
    lemma_try_into_id_32 seed;
    let seed32 : t_Array u8 (mk_usize 32) =
      Core_models.Result.impl__unwrap
        #(t_Array u8 (mk_usize 32))
        #Core_models.Array.t_TryFromSliceError
        (Core_models.Convert.f_try_into #(t_Slice u8)
            #(t_Array u8 (mk_usize 32))
            #FStar.Tactics.Typeclasses.solve
            seed)
    in
    assert ((seed32 <: Seq.seq u8) == (seed <: Seq.seq u8));
    assert (Seq.slice pin_i 0 32 == (seed32 <: Seq.seq u8));
    assert (v (Seq.index pin_i 32) == v ds0 + i);
    assert (v b == v ds0 + i);
    assert (Seq.index pin_i 32 == b);
    lemma_concat_byte_eq seed32 b pin_i;
    assert (pin_i == HI.concat_byte (mk_usize 32) (mk_usize 33) seed32 b);
    (* (sample_vector_cbd K eta seed ds0).[i] == sample_secret eta (concat_byte ...) *)
    P.createi_lemma #(t_Array P.t_FieldElement (mk_usize 256)) v_K
      #(usize -> t_Array P.t_FieldElement (mk_usize 256))
      (fun (j:usize{j <. v_K}) ->
          let pj : t_Array u8 (mk_usize 33) =
            HI.concat_byte (mk_usize 32) (mk_usize 33)
              (Core_models.Result.impl__unwrap
                  #(t_Array u8 (mk_usize 32))
                  #Core_models.Array.t_TryFromSliceError
                  (Core_models.Convert.f_try_into #(t_Slice u8)
                      #(t_Array u8 (mk_usize 32))
                      #FStar.Tactics.Typeclasses.solve
                      seed))
              (ds0 +! (cast (j <: usize) <: u8) <: u8)
          in
          HI.sample_secret eta pj)
      (mk_usize i)
#pop-options

(* ================================================================== *)
(* AGGRESSIVE OPACITY: the loop invariant carries ONLY this opaque     *)
(* atom, which bundles BOTH the eta-bound and the spec-equality for one *)
(* row.  Its body (is_bounded_poly's 256-coeff forall + the createi-    *)
(* indexed sample_vector_cbd) is therefore NEVER exposed to the ind_cpa *)
(* function WP — `reveal_opaque` happens only inside the primitive      *)
(* intro / assemble lemmas below.  Mirrors Matrix_bridge.row_done.      *)
(* ================================================================== *)
[@@ "opaque_to_smt"]
let cbd_row_done
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize)
      (x: VV.t_PolynomialRingElement vV)
      (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (j: nat {j < v v_K})
    : Type0
  = Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3) x /\
    VS.poly_to_spec #vV x == Seq.index target j

(* intro: the two raw per-row facts (sample_from_binomial bound +       *)
(* lemma_per_index_cbd spec-eq) close the opaque atom.                  *)
let lemma_cbd_row_intro
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize)
      (x: VV.t_PolynomialRingElement vV)
      (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (j: nat {j < v v_K})
    : Lemma
      (requires
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3) x /\
        VS.poly_to_spec #vV x == Seq.index target j)
      (ensures cbd_row_done v_K x target j)
  = reveal_opaque (`%cbd_row_done) (cbd_row_done v_K x target j)

(* ------------------------------------------------------------------ *)
(* LOOP STEP (opaque-atom frame).  The invariant carries only the      *)
(* opaque atom, so this is pure Seq.upd index bookkeeping — NO reveal,  *)
(* NO createi, NO is_bounded unfold.  update_at_usize is transparent    *)
(* (= Seq.upd error_1_old (v i) x) so the upd lemmas fire; the atom     *)
(* passes through by congruence on the (proven-equal) row element.      *)
(* ------------------------------------------------------------------ *)
#push-options "--z3rlimit 100"
let lemma_cbd_loop_step
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K <= 4})
      (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (error_1_old: t_Array (VV.t_PolynomialRingElement vV) v_K)
      (i: usize{v i < v v_K})
      (x: VV.t_PolynomialRingElement vV)
    : Lemma
      (requires
        (forall (j: nat). j < v i ==> cbd_row_done v_K (Seq.index error_1_old j) target j) /\
        cbd_row_done v_K x target (v i))
      (ensures
        (let error_1_new = U.update_at_usize error_1_old i x in
         (forall (j: nat). j < v i + 1 ==> cbd_row_done v_K (Seq.index error_1_new j) target j)))
  = let error_1_new = U.update_at_usize error_1_old i x in
    let aux (j: nat{j < v i + 1}) : Lemma (cbd_row_done v_K (Seq.index error_1_new j) target j) =
      if j < v i then Seq.lemma_index_upd2 error_1_old (v i) x j
      else Seq.lemma_index_upd1 error_1_old (v i) x
    in
    Classical.forall_intro aux
#pop-options

(* ================================================================== *)
(* OPAQUE LOOP-INVARIANT ATOM.  The raw `forall j. cbd_row_done …` is   *)
(* what hax bakes into the fold-accumulator REFINEMENT TYPE; its        *)
(* refinement_interpretation then re-derives the inner `forall j` for    *)
(* every accumulator-typing query in the whole-function post VC →        *)
(* ~20k instantiations → "incomplete quantifiers".  Hiding the forall    *)
(* behind ONE opaque atom collapses the accumulator refinement to a      *)
(* single uninterpreted predicate, so the cascade disappears.            *)
(* (qi.profile: refinement_interpretation_Tm_refine_05b846… × 20753.)    *)
(* ================================================================== *)
[@@ "opaque_to_smt"]
let cbd_prefix_done
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize)
      (error_1: t_Array (VV.t_PolynomialRingElement vV) v_K)
      (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (n: nat{n <= v v_K})
    : Type0
  = forall (j: nat). j < n ==> cbd_row_done v_K (Seq.index error_1 j) target j

(* init: empty prefix is vacuously done. *)
let lemma_cbd_prefix_init
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize)
      (error_1: t_Array (VV.t_PolynomialRingElement vV) v_K)
      (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    : Lemma (cbd_prefix_done v_K error_1 target 0)
  = reveal_opaque (`%cbd_prefix_done) (cbd_prefix_done v_K error_1 target 0)

(* step: extend the opaque prefix by one freshly-proven row.  The index is *)
(* a nat (= v i) so the loop-invariant check matches by nat equality, not  *)
(* usize injectivity.                                                       *)
#push-options "--z3rlimit 100"
let lemma_cbd_prefix_step
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K <= 4})
      (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (error_1_old: t_Array (VV.t_PolynomialRingElement vV) v_K)
      (i: usize{v i < v v_K})
      (x: VV.t_PolynomialRingElement vV)
    : Lemma
      (requires
        cbd_prefix_done v_K error_1_old target (v i) /\
        cbd_row_done v_K x target (v i))
      (ensures cbd_prefix_done v_K (U.update_at_usize error_1_old i x) target (v i + 1))
  = reveal_opaque (`%cbd_prefix_done) (cbd_prefix_done v_K error_1_old target (v i));
    lemma_cbd_loop_step #vV v_K target error_1_old i x;
    reveal_opaque (`%cbd_prefix_done)
      (cbd_prefix_done v_K (U.update_at_usize error_1_old i x) target (v i + 1))
#pop-options

(* ------------------------------------------------------------------ *)
(* ASSEMBLE / ENSURES BRIDGE: from the loop-exit forall of opaque atoms *)
(* (j < K), reveal each and produce the exact ind_cpa `.fsti` ensures:  *)
(*   is_bounded_polynomial_vector K 3 error_1                          *)
(*   /\ b2t (vector_to_spec K error_1 =. sample_vector_cbd K eta seed ds0) *)
(* All reveals happen HERE (clean context); the function WP only sees   *)
(* the opaque-atom forall and this lemma's clean post.                  *)
(* ------------------------------------------------------------------ *)
(* ★ The full ind_cpa ensures, wrapped in ONE opaque predicate.  The function's .fsti ensures is
   just `sample_ring_element_cbd_post_pred …`; the establisher lemma produces exactly that atom, so
   the return-post discharges by a single atom-match — never re-deriving the 3-conjunct body (the
   residual that defeated separate asserts and the transparent combined post). *)
[@@ "opaque_to_smt"]
let sample_ring_element_cbd_post_pred
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K <= 4})
      (eta: usize{v eta == 2 \/ v eta == 3})
      (seed: t_Slice u8{Seq.length seed == 32})
      (ds0: u8{v ds0 + v v_K < 256})
      (ds_out: u8)
      (error_1: t_Array (VV.t_PolynomialRingElement vV) v_K)
    : Type0
  = b2t (ds_out =. (ds0 +! (cast (v_K <: usize) <: u8) <: u8)) /\
    Libcrux_ml_kem.Polynomial.Spec.is_bounded_polynomial_vector v_K #vV (mk_usize 3) error_1 /\
    b2t ((VS.vector_to_spec v_K #vV error_1
            <: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) =.
         (HI.sample_vector_cbd v_K eta seed ds0
            <: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K))

(* ★ FULL-POST establisher: produces the opaque post predicate as ONE atom, GIVEN the loop-exit
   opaque-atom forall + the ds increment value-fact. *)
#push-options "--z3rlimit 200"
let lemma_sample_ring_element_cbd_post
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K <= 4})
      (eta: usize{v eta == 2 \/ v eta == 3})
      (seed: t_Slice u8{Seq.length seed == 32})
      (ds0: u8{v ds0 + v v_K < 256})
      (ds_out: u8)
      (error_1: t_Array (VV.t_PolynomialRingElement vV) v_K)
    : Lemma
      (requires
        cbd_prefix_done v_K error_1 (HI.sample_vector_cbd v_K eta seed ds0) (v v_K) /\
        v ds_out == v ds0 + v v_K)
      (ensures
        b2t (ds_out =. (ds0 +! (cast (v_K <: usize) <: u8) <: u8)) /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_polynomial_vector v_K #vV (mk_usize 3) error_1 /\
        b2t ((VS.vector_to_spec v_K #vV error_1
                <: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) =.
             (HI.sample_vector_cbd v_K eta seed ds0
                <: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)))
  = let target = HI.sample_vector_cbd v_K eta seed ds0 in
    reveal_opaque (`%cbd_prefix_done) (cbd_prefix_done v_K error_1 target (v v_K));
    // reveal each row to recover the raw bound + spec-eq
    let aux (j: nat{j < v v_K}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3) (Seq.index error_1 j) /\
       VS.poly_to_spec #vV (Seq.index error_1 j) == Seq.index target j) =
      reveal_opaque (`%cbd_row_done) (cbd_row_done v_K (Seq.index error_1 j) target j)
    in
    Classical.forall_intro aux;
    // bound: forall j. is_bounded (error_1[j])  ==>  is_bounded_polynomial_vector
    let arr_bound (i: usize{v i < v v_K}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3)
        (error_1.[ i ] <: VV.t_PolynomialRingElement vV)) = () in
    Classical.forall_intro arr_bound;
    Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_intro v_K #vV error_1
      (mk_usize 3);
    // spec: vector_to_spec error_1 == target
    let auxv (j: nat{j < v v_K}) : Lemma
      (Seq.index (VS.vector_to_spec v_K #vV error_1) j == Seq.index target j) =
      VS.vector_to_spec_index v_K #vV error_1 j
    in
    Classical.forall_intro auxv;
    Seq.lemma_eq_intro (VS.vector_to_spec v_K #vV error_1) target
#pop-options

(* ★ SMTPat variant of the finalize establisher.  Fires on the fold's OWN exit-invariant
   hypothesis `cbd_prefix_done v_K error_1 target (v v_K)` (which F* states about the abstract
   fold-result in the SAME representation as the return-post goal), producing the two vector
   post conjuncts.  This avoids the explicit-lemma-call representation divergence (the call
   substitutes a STRIPPED fold-step term into the ensures, which Z3 cannot equate with the
   return goal's FULL fold-step term — see ind_cpa-sample_ring_element_cbd-status.md). *)
#push-options "--z3rlimit 200"
let lemma_cbd_prefix_done_post_smtpat
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K <= 4})
      (eta: usize{v eta == 2 \/ v eta == 3})
      (seed: t_Slice u8{Seq.length seed == 32})
      (ds0: u8{v ds0 + v v_K < 256})
      (error_1: t_Array (VV.t_PolynomialRingElement vV) v_K)
    : Lemma
      (requires
        cbd_prefix_done v_K error_1 (HI.sample_vector_cbd v_K eta seed ds0) (v v_K))
      (ensures
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_polynomial_vector v_K #vV (mk_usize 3) error_1 /\
        b2t ((VS.vector_to_spec v_K #vV error_1
                <: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) =.
             (HI.sample_vector_cbd v_K eta seed ds0
                <: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)))
      [SMTPat (cbd_prefix_done v_K error_1 (HI.sample_vector_cbd v_K eta seed ds0) (v v_K))]
  = let target = HI.sample_vector_cbd v_K eta seed ds0 in
    reveal_opaque (`%cbd_prefix_done) (cbd_prefix_done v_K error_1 target (v v_K));
    let aux (j: nat{j < v v_K}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3) (Seq.index error_1 j) /\
       VS.poly_to_spec #vV (Seq.index error_1 j) == Seq.index target j) =
      reveal_opaque (`%cbd_row_done) (cbd_row_done v_K (Seq.index error_1 j) target j)
    in
    Classical.forall_intro aux;
    let arr_bound (i: usize{v i < v v_K}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3)
        (error_1.[ i ] <: VV.t_PolynomialRingElement vV)) = () in
    Classical.forall_intro arr_bound;
    Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_intro v_K #vV error_1
      (mk_usize 3);
    let auxv (j: nat{j < v v_K}) : Lemma
      (Seq.index (VS.vector_to_spec v_K #vV error_1) j == Seq.index target j) =
      VS.vector_to_spec_index v_K #vV error_1 j
    in
    Classical.forall_intro auxv;
    Seq.lemma_eq_intro (VS.vector_to_spec v_K #vV error_1) target
#pop-options

(* ================================================================== *)
(* CBD + NTT per-row coupling for `sample_vector_cbd_then_ntt`.        *)
(*   spec = vector_ntt K (sample_vector_cbd K eta seed ds0), so        *)
(*   row i of the result is `ntt (cbd[i])`.                            *)
(*   impl: re[i] = ntt_binomially_sampled_ring_element                 *)
(*                   (sample_from_binomial_distribution prf_outputs[i]).*)
(* Mirror of the `cbd_*` family above, bound 3328, target =           *)
(*   sample_vector_cbd_then_ntt.                                       *)
(* ================================================================== *)

(* (vector_ntt K vec)[i] == ntt (vec[i]) — createi index. *)
let lemma_vector_ntt_index
      (v_K: usize{v v_K <= 4})
      (vec: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (i: nat{i < v v_K})
    : Lemma (Seq.index (N.vector_ntt v_K vec) i == N.ntt (vec.[ mk_usize i ]))
  = P.createi_lemma #(t_Array P.t_FieldElement (mk_usize 256)) v_K
      #(usize -> t_Array P.t_FieldElement (mk_usize 256))
      (fun (j: usize{j <. v_K}) -> N.ntt (vec.[ j ]))
      (mk_usize i)

(* PER-INDEX: poly_to_spec (ntt_binomially sampled_i) == sample_vector_cbd_then_ntt[i].
   Composes lemma_per_index_cbd (poly_to_spec sampled_i == cbd[i]) with the
   ntt_binomially functional post (to_spec_poly_plain re_i == N.ntt(to_spec_poly_plain
   sampled_i)), bridged across the poly_to_spec / to_spec_poly_plain reps. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_per_index_cbd_ntt
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K == 2 \/ v v_K == 3 \/ v v_K == 4})
      (eta: usize{v eta == 2 \/ v eta == 3})
      (seed: t_Slice u8{Seq.length seed == 32})
      (ds0: u8{v ds0 + v v_K < 256})
      (len_rand: usize{v len_rand == v eta * 64})
      (prf_inputs: t_Array (t_Array u8 (mk_usize 33)) v_K)
      (prf_outputs: t_Array (t_Array u8 len_rand) v_K)
      (sampled_i re_i: VV.t_PolynomialRingElement vV)
      (i: nat{i < v v_K})
    : Lemma
      (requires
        VS.poly_to_spec #vV sampled_i ==
          HS.sample_poly_cbd (eta *! mk_usize 64) (eta *! mk_usize 512) eta
            (prf_outputs.[ mk_usize i ] <: t_Array u8 len_rand) /\
        prf_outputs == SU.v_PRFxN v_K len_rand prf_inputs /\
        v (Seq.index (prf_inputs.[ mk_usize i ] <: t_Array u8 (mk_usize 33)) 32) == v ds0 + i /\
        Seq.slice (prf_inputs.[ mk_usize i ] <: t_Array u8 (mk_usize 33)) 0 32 == (seed <: Seq.seq u8) /\
        CH.to_spec_poly_plain #vV re_i == N.ntt (CH.to_spec_poly_plain #vV sampled_i))
      (ensures
        VS.poly_to_spec #vV re_i ==
        Seq.index (HI.sample_vector_cbd_then_ntt v_K eta seed ds0) i)
  = let cbd = HI.sample_vector_cbd v_K eta seed ds0 in
    lemma_per_index_cbd #vV v_K eta seed ds0 len_rand prf_inputs prf_outputs sampled_i i;
    assert (VS.poly_to_spec #vV sampled_i == Seq.index cbd i);
    Br.poly_to_spec_eq_to_spec_poly_plain #vV sampled_i;
    Br.poly_to_spec_eq_to_spec_poly_plain #vV re_i;
    assert (CH.to_spec_poly_plain #vV sampled_i == Seq.index cbd i);
    assert (CH.to_spec_poly_plain #vV re_i == N.ntt (Seq.index cbd i));
    lemma_vector_ntt_index v_K cbd i;
    assert (Seq.index (N.vector_ntt v_K cbd) i == N.ntt (cbd.[ mk_usize i ]));
    assert (cbd.[ mk_usize i ] == Seq.index cbd i);
    assert (HI.sample_vector_cbd_then_ntt v_K eta seed ds0 == N.vector_ntt v_K cbd)
#pop-options

(* ----- opaque per-row "done" atom (bound 3328, ntt target) ----- *)
[@@ "opaque_to_smt"]
let cbd_ntt_row_done
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize)
      (x: VV.t_PolynomialRingElement vV)
      (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (j: nat {j < v v_K})
    : Type0
  = Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) x /\
    VS.poly_to_spec #vV x == Seq.index target j

let lemma_cbd_ntt_row_intro
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize)
      (x: VV.t_PolynomialRingElement vV)
      (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (j: nat {j < v v_K})
    : Lemma
      (requires
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) x /\
        VS.poly_to_spec #vV x == Seq.index target j)
      (ensures cbd_ntt_row_done v_K x target j)
  = reveal_opaque (`%cbd_ntt_row_done) (cbd_ntt_row_done v_K x target j)

#push-options "--z3rlimit 100"
let lemma_cbd_ntt_loop_step
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K <= 4})
      (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (re_old: t_Array (VV.t_PolynomialRingElement vV) v_K)
      (i: usize{v i < v v_K})
      (x: VV.t_PolynomialRingElement vV)
    : Lemma
      (requires
        (forall (j: nat). j < v i ==> cbd_ntt_row_done v_K (Seq.index re_old j) target j) /\
        cbd_ntt_row_done v_K x target (v i))
      (ensures
        (let re_new = U.update_at_usize re_old i x in
         (forall (j: nat). j < v i + 1 ==> cbd_ntt_row_done v_K (Seq.index re_new j) target j)))
  = let re_new = U.update_at_usize re_old i x in
    let aux (j: nat{j < v i + 1}) : Lemma (cbd_ntt_row_done v_K (Seq.index re_new j) target j) =
      if j < v i then Seq.lemma_index_upd2 re_old (v i) x j
      else Seq.lemma_index_upd1 re_old (v i) x
    in
    Classical.forall_intro aux
#pop-options

[@@ "opaque_to_smt"]
let cbd_ntt_prefix_done
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize)
      (re_as_ntt: t_Array (VV.t_PolynomialRingElement vV) v_K)
      (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (n: nat{n <= v v_K})
    : Type0
  = forall (j: nat). j < n ==> cbd_ntt_row_done v_K (Seq.index re_as_ntt j) target j

let lemma_cbd_ntt_prefix_init
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize)
      (re_as_ntt: t_Array (VV.t_PolynomialRingElement vV) v_K)
      (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    : Lemma (cbd_ntt_prefix_done v_K re_as_ntt target 0)
  = reveal_opaque (`%cbd_ntt_prefix_done) (cbd_ntt_prefix_done v_K re_as_ntt target 0)

#push-options "--z3rlimit 100"
let lemma_cbd_ntt_prefix_step
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K <= 4})
      (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
      (re_old: t_Array (VV.t_PolynomialRingElement vV) v_K)
      (i: usize{v i < v v_K})
      (x: VV.t_PolynomialRingElement vV)
    : Lemma
      (requires
        cbd_ntt_prefix_done v_K re_old target (v i) /\
        cbd_ntt_row_done v_K x target (v i))
      (ensures cbd_ntt_prefix_done v_K (U.update_at_usize re_old i x) target (v i + 1))
  = reveal_opaque (`%cbd_ntt_prefix_done) (cbd_ntt_prefix_done v_K re_old target (v i));
    lemma_cbd_ntt_loop_step #vV v_K target re_old i x;
    reveal_opaque (`%cbd_ntt_prefix_done)
      (cbd_ntt_prefix_done v_K (U.update_at_usize re_old i x) target (v i + 1))
#pop-options

(* ----- full-post establishers (explicit + SMTPat tuple-wall fix) ----- *)
#push-options "--z3rlimit 200"
let lemma_sample_vector_cbd_then_ntt_post
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K <= 4})
      (eta: usize{v eta == 2 \/ v eta == 3})
      (seed: t_Slice u8{Seq.length seed == 32})
      (ds0: u8{v ds0 + v v_K < 256})
      (ds_out: u8)
      (re_as_ntt: t_Array (VV.t_PolynomialRingElement vV) v_K)
    : Lemma
      (requires
        cbd_ntt_prefix_done v_K re_as_ntt (HI.sample_vector_cbd_then_ntt v_K eta seed ds0) (v v_K) /\
        v ds_out == v ds0 + v v_K)
      (ensures
        b2t (ds_out =. (ds0 +! (cast (v_K <: usize) <: u8) <: u8)) /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_polynomial_vector v_K #vV (mk_usize 3328) re_as_ntt /\
        b2t ((VS.vector_to_spec v_K #vV re_as_ntt
                <: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) =.
             (HI.sample_vector_cbd_then_ntt v_K eta seed ds0
                <: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)))
  = let target = HI.sample_vector_cbd_then_ntt v_K eta seed ds0 in
    reveal_opaque (`%cbd_ntt_prefix_done) (cbd_ntt_prefix_done v_K re_as_ntt target (v v_K));
    let aux (j: nat{j < v v_K}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) (Seq.index re_as_ntt j) /\
       VS.poly_to_spec #vV (Seq.index re_as_ntt j) == Seq.index target j) =
      reveal_opaque (`%cbd_ntt_row_done) (cbd_ntt_row_done v_K (Seq.index re_as_ntt j) target j)
    in
    Classical.forall_intro aux;
    let arr_bound (i: usize{v i < v v_K}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3328)
        (re_as_ntt.[ i ] <: VV.t_PolynomialRingElement vV)) = () in
    Classical.forall_intro arr_bound;
    Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_intro v_K #vV re_as_ntt
      (mk_usize 3328);
    let auxv (j: nat{j < v v_K}) : Lemma
      (Seq.index (VS.vector_to_spec v_K #vV re_as_ntt) j == Seq.index target j) =
      VS.vector_to_spec_index v_K #vV re_as_ntt j
    in
    Classical.forall_intro auxv;
    Seq.lemma_eq_intro (VS.vector_to_spec v_K #vV re_as_ntt) target
#pop-options

#push-options "--z3rlimit 200"
let lemma_cbd_ntt_prefix_done_post_smtpat
      (#vV: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations vV)
      (v_K: usize{v v_K <= 4})
      (eta: usize{v eta == 2 \/ v eta == 3})
      (seed: t_Slice u8{Seq.length seed == 32})
      (ds0: u8{v ds0 + v v_K < 256})
      (re_as_ntt: t_Array (VV.t_PolynomialRingElement vV) v_K)
    : Lemma
      (requires
        cbd_ntt_prefix_done v_K re_as_ntt (HI.sample_vector_cbd_then_ntt v_K eta seed ds0) (v v_K))
      (ensures
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_polynomial_vector v_K #vV (mk_usize 3328) re_as_ntt /\
        b2t ((VS.vector_to_spec v_K #vV re_as_ntt
                <: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) =.
             (HI.sample_vector_cbd_then_ntt v_K eta seed ds0
                <: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)))
      [SMTPat (cbd_ntt_prefix_done v_K re_as_ntt (HI.sample_vector_cbd_then_ntt v_K eta seed ds0) (v v_K))]
  = let target = HI.sample_vector_cbd_then_ntt v_K eta seed ds0 in
    reveal_opaque (`%cbd_ntt_prefix_done) (cbd_ntt_prefix_done v_K re_as_ntt target (v v_K));
    let aux (j: nat{j < v v_K}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) (Seq.index re_as_ntt j) /\
       VS.poly_to_spec #vV (Seq.index re_as_ntt j) == Seq.index target j) =
      reveal_opaque (`%cbd_ntt_row_done) (cbd_ntt_row_done v_K (Seq.index re_as_ntt j) target j)
    in
    Classical.forall_intro aux;
    let arr_bound (i: usize{v i < v v_K}) : Lemma
      (Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly (mk_usize 3328)
        (re_as_ntt.[ i ] <: VV.t_PolynomialRingElement vV)) = () in
    Classical.forall_intro arr_bound;
    Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_intro v_K #vV re_as_ntt
      (mk_usize 3328);
    let auxv (j: nat{j < v v_K}) : Lemma
      (Seq.index (VS.vector_to_spec v_K #vV re_as_ntt) j == Seq.index target j) =
      VS.vector_to_spec_index v_K #vV re_as_ntt j
    in
    Classical.forall_intro auxv;
    Seq.lemma_eq_intro (VS.vector_to_spec v_K #vV re_as_ntt) target
#pop-options

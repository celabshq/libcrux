module Hacspec_ml_kem.Commute.Chunk
#set-options "--fuel 0 --ifuel 1 --z3rlimit 200"
open FStar.Mul
open Core_models
open Libcrux_ml_kem.Vector.Traits.Spec
open Hacspec_ml_kem.ModQ

module P  = Hacspec_ml_kem.Parameters
module L  = FStar.Math.Lemmas
module T  = Libcrux_ml_kem.Vector.Traits
module TS = Libcrux_ml_kem.Vector.Traits.Spec
module N  = Hacspec_ml_kem.Ntt
module IN = Hacspec_ml_kem.Invert_ntt
module CP = Hacspec_ml_kem.Compress
module MQ = Hacspec_ml_kem.ModQ
module HP = Hacspec_ml_kem.Polynomial
module V  = Libcrux_ml_kem.Vector


val lemma_add_fe_commute_mont (a b r: i16) :
    Lemma (requires v r == v a + v b)
          (ensures  P.impl_FieldElement__add
                        (mont_i16_to_spec_fe a) (mont_i16_to_spec_fe b)
                    == mont_i16_to_spec_fe r)

val lemma_sub_fe_commute_mont (a b r: i16) :
    Lemma (requires v r == v a - v b)
          (ensures  P.impl_FieldElement__sub
                        (mont_i16_to_spec_fe a) (mont_i16_to_spec_fe b)
                    == mont_i16_to_spec_fe r)

val lemma_impl_mul_v_val (x y: P.t_FieldElement) :
    Lemma (v (P.impl_FieldElement__mul x y).P.f_val
             == (v x.P.f_val * v y.P.f_val) % 3329)

val lemma_impl_add_v_val (x y: P.t_FieldElement) :
    Lemma (v (P.impl_FieldElement__add x y).P.f_val
             == (v x.P.f_val + v y.P.f_val) % 3329)

val lemma_impl_sub_v_val (x y: P.t_FieldElement) :
    Lemma (v (P.impl_FieldElement__sub x y).P.f_val
             == (v x.P.f_val - v y.P.f_val) % 3329)

val lemma_add_fe_commute_mont_mod (a b r: i16) :
    Lemma (requires v r % 3329 == (v a + v b) % 3329)
          (ensures  P.impl_FieldElement__add
                        (mont_i16_to_spec_fe a) (mont_i16_to_spec_fe b)
                    == mont_i16_to_spec_fe r)

val lemma_butterfly_pair_commute
    (vec result: t_Array i16 (mk_usize 16))
    (z: i16) (i: nat{i < 16}) (j: nat{j < 16}) :
  Lemma (requires
           v (Seq.index result i) % 3329
             == (v (Seq.index vec i) + v (Seq.index vec j) * v z * 169) % 3329 /\
           v (Seq.index result j) % 3329
             == (v (Seq.index vec i) - v (Seq.index vec j) * v z * 169) % 3329)
        (ensures
           mont_i16_to_spec_fe (Seq.index result i) ==
             P.impl_FieldElement__add
               (mont_i16_to_spec_fe (Seq.index vec i))
               (P.impl_FieldElement__mul (mont_i16_to_spec_fe z)
                                         (mont_i16_to_spec_fe (Seq.index vec j))) /\
           mont_i16_to_spec_fe (Seq.index result j) ==
             P.impl_FieldElement__sub
               (mont_i16_to_spec_fe (Seq.index vec i))
               (P.impl_FieldElement__mul (mont_i16_to_spec_fe z)
                                         (mont_i16_to_spec_fe (Seq.index vec j))))

val lemma_barrett_reduce_lane_post_to_mod_q_eq (a r: i16) :
    Lemma (requires TS.barrett_reduce_lane_post a r)
          (ensures  mod_q_eq (v r) (v a))

val lemma_montgomery_multiply_lane_post_to_mod_q_eq (a c r: i16) :
    Lemma (requires TS.montgomery_multiply_lane_post a c r)
          (ensures  mod_q_eq (v r) (v a * v c * 169))

val lemma_inv_butterfly_mont_lane_to_fe (a b zeta r b_minus_a: i16) :
    Lemma (requires TS.montgomery_multiply_lane_post b_minus_a zeta r
                  /\ v b_minus_a == v b - v a)
          (ensures  mont_i16_to_spec_fe r ==
                    P.impl_FieldElement__mul
                      (mont_i16_to_spec_fe zeta)
                      (P.impl_FieldElement__sub
                         (mont_i16_to_spec_fe b)
                         (mont_i16_to_spec_fe a)))

val lemma_inv_butterfly_pair_commute
    (vec result: t_Array i16 (mk_usize 16))
    (z: i16) (i: nat{i < 16}) (j: nat{j < 16}) :
  Lemma (requires
           v (Seq.index result i) % 3329
             == (v (Seq.index vec j) + v (Seq.index vec i)) % 3329 /\
           v (Seq.index result j) % 3329
             == ((v (Seq.index vec j) - v (Seq.index vec i)) * v z * 169) % 3329)
        (ensures
           mont_i16_to_spec_fe (Seq.index result i) ==
             P.impl_FieldElement__add
               (mont_i16_to_spec_fe (Seq.index vec i))
               (mont_i16_to_spec_fe (Seq.index vec j)) /\
           mont_i16_to_spec_fe (Seq.index result j) ==
             P.impl_FieldElement__mul
               (mont_i16_to_spec_fe z)
               (P.impl_FieldElement__sub
                 (mont_i16_to_spec_fe (Seq.index vec j))
                 (mont_i16_to_spec_fe (Seq.index vec i))))

val lemma_mont_mul_fe_commute_mont_mont (a b r: i16) :
    Lemma (requires v r % 3329 == (v a * v b * 169) % 3329)
          (ensures  P.impl_FieldElement__mul
                        (mont_i16_to_spec_fe a) (mont_i16_to_spec_fe b)
                    == mont_i16_to_spec_fe r)

val lemma_compress_message_coefficient_fe_commute (fe result: i16) :
  Lemma (requires v fe >= 0 /\ v fe < 3329 /\
                  v result == ((v fe * 4 + 3329) / 6658) % 2)
        (ensures Hacspec_ml_kem.Compress.compress_d
                   (i16_to_spec_fe fe) (mk_usize 1)
                 == i16_to_spec_fe result)

val lemma_compress_ciphertext_coefficient_fe_commute (fe result: i16) (d: usize) :
  Lemma (requires (v d == 4 \/ v d == 5 \/ v d == 10 \/ v d == 11) /\
                  v fe >= 0 /\ v fe < 3329 /\
                  v result == ((v fe * 2 * pow2 (v d) + 3329) / 6658) % pow2 (v d))
        (ensures Hacspec_ml_kem.Compress.compress_d
                   (i16_to_spec_fe fe) d
                 == i16_to_spec_fe result)

val lemma_decompress_1_fe_commute_int (a result: i16) :
  Lemma (requires v a >= 0 /\ v a <= 1 /\
                  v result == (2 * v a * 3329 + 2) / 4)
        (ensures Hacspec_ml_kem.Compress.decompress_d
                   (i16_to_spec_fe a) (mk_usize 1)
                 == i16_to_spec_fe result)

val lemma_decompress_ciphertext_coefficient_fe_commute
    (a result: i16) (d: usize) :
  Lemma (requires (v d == 4 \/ v d == 5 \/ v d == 10 \/ v d == 11) /\
                  v a >= 0 /\ v a < pow2 (v d) /\
                  v result == (2 * v a * 3329 + pow2 (v d)) / (pow2 (v d) * 2))
        (ensures Hacspec_ml_kem.Compress.decompress_d
                   (i16_to_spec_fe a) d
                 == i16_to_spec_fe result)

val lemma_ntt_layer_1_step_branch_0
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.ntt_spec vec (v zeta0) 0 2 result /\
                  Spec.Utils.ntt_spec vec (v zeta0) 1 3 result)
        (ensures TS.ntt_layer_1_step_branch_post 0 vec zeta0 zeta1 zeta2 zeta3 result)

val lemma_ntt_layer_1_step_branch_1
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.ntt_spec vec (v zeta1) 4 6 result /\
                  Spec.Utils.ntt_spec vec (v zeta1) 5 7 result)
        (ensures TS.ntt_layer_1_step_branch_post 1 vec zeta0 zeta1 zeta2 zeta3 result)

val lemma_ntt_layer_1_step_branch_2
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.ntt_spec vec (v zeta2) 8 10 result /\
                  Spec.Utils.ntt_spec vec (v zeta2) 9 11 result)
        (ensures TS.ntt_layer_1_step_branch_post 2 vec zeta0 zeta1 zeta2 zeta3 result)

val lemma_ntt_layer_1_step_branch_3
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.ntt_spec vec (v zeta3) 12 14 result /\
                  Spec.Utils.ntt_spec vec (v zeta3) 13 15 result)
        (ensures TS.ntt_layer_1_step_branch_post 3 vec zeta0 zeta1 zeta2 zeta3 result)

val lemma_inv_ntt_layer_1_step_branch_0
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.inv_ntt_spec vec (v zeta0) 0 2 result /\
                  Spec.Utils.inv_ntt_spec vec (v zeta0) 1 3 result)
        (ensures TS.inv_ntt_layer_1_step_branch_post 0 vec zeta0 zeta1 zeta2 zeta3 result)

val lemma_inv_ntt_layer_1_step_branch_1
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.inv_ntt_spec vec (v zeta1) 4 6 result /\
                  Spec.Utils.inv_ntt_spec vec (v zeta1) 5 7 result)
        (ensures TS.inv_ntt_layer_1_step_branch_post 1 vec zeta0 zeta1 zeta2 zeta3 result)

val lemma_inv_ntt_layer_1_step_branch_2
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.inv_ntt_spec vec (v zeta2) 8 10 result /\
                  Spec.Utils.inv_ntt_spec vec (v zeta2) 9 11 result)
        (ensures TS.inv_ntt_layer_1_step_branch_post 2 vec zeta0 zeta1 zeta2 zeta3 result)

val lemma_inv_ntt_layer_1_step_branch_3
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.inv_ntt_spec vec (v zeta3) 12 14 result /\
                  Spec.Utils.inv_ntt_spec vec (v zeta3) 13 15 result)
        (ensures TS.inv_ntt_layer_1_step_branch_post 3 vec zeta0 zeta1 zeta2 zeta3 result)

val lemma_ntt_multiply_branch_0
    (lhs rhs result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.is_i16b 1664 zeta0 /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (v zeta0) 0 result /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (- (v zeta0)) 1 result)
        (ensures TS.ntt_multiply_branch_post 0 lhs rhs zeta0 zeta1 zeta2 zeta3 result)

val lemma_ntt_multiply_branch_1
    (lhs rhs result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.is_i16b 1664 zeta1 /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (v zeta1) 2 result /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (- (v zeta1)) 3 result)
        (ensures TS.ntt_multiply_branch_post 1 lhs rhs zeta0 zeta1 zeta2 zeta3 result)

val lemma_ntt_multiply_branch_2
    (lhs rhs result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.is_i16b 1664 zeta2 /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (v zeta2) 4 result /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (- (v zeta2)) 5 result)
        (ensures TS.ntt_multiply_branch_post 2 lhs rhs zeta0 zeta1 zeta2 zeta3 result)

val lemma_ntt_multiply_branch_3
    (lhs rhs result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.is_i16b 1664 zeta3 /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (v zeta3) 6 result /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (- (v zeta3)) 7 result)
        (ensures TS.ntt_multiply_branch_post 3 lhs rhs zeta0 zeta1 zeta2 zeta3 result)

let to_spec_poly_plain
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p: V.t_PolynomialRingElement vV)
    : t_Array P.t_FieldElement (mk_usize 256)
  = P.createi #P.t_FieldElement (mk_usize 256)
        #(usize -> P.t_FieldElement)
        (fun (j: usize { j <. mk_usize 256 }) ->
          (i16_to_spec_fe
            (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (v j / 16)))
                       (v j % 16))
           <: P.t_FieldElement))

(* Mont-domain poly lift: each i16 lane is interpreted as `a*R mod q`
   with R = 2^16; `mont_i16_to_spec_fe` strips the R factor. *)

let to_spec_poly_mont
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p: V.t_PolynomialRingElement vV)
    : t_Array P.t_FieldElement (mk_usize 256)
  = P.createi #P.t_FieldElement (mk_usize 256)
        #(usize -> P.t_FieldElement)
        (fun (j: usize { j <. mk_usize 256 }) ->
          (mont_i16_to_spec_fe
            (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (v j / 16)))
                       (v j % 16))
           <: P.t_FieldElement))

(* Per-lane index helper for `to_spec_poly_plain`.  Wraps `createi_lemma`
   to accept a `nat` index, mirroring `lane_plain` for the per-vector
   lift.  Useful when peeling per-lane facts from the poly equation. *)

val poly_lane_plain
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p: V.t_PolynomialRingElement vV) (j: nat {j < 256}) :
    Lemma (Seq.index (to_spec_poly_plain p) j
           == i16_to_spec_fe
                (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (j / 16)))
                           (j % 16)))

(* Mont-domain twin of poly_lane_plain (consumed by Polynomial / Compute_u_bridge /
   Matrix_bilin2; was missing from the firewall surface). *)
val poly_lane_mont
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p: V.t_PolynomialRingElement vV) (j: nat {j < 256}) :
    Lemma (Seq.index (to_spec_poly_mont p) j
           == mont_i16_to_spec_fe
                (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (j / 16)))
                           (j % 16)))

val lemma_poly_barrett_reduce_id
    (p: t_Array P.t_FieldElement (mk_usize 256)) :
    Lemma (HP.poly_barrett_reduce p == p)

val lemma_poly_barrett_reduce_commute
    (#vV: Type0) {| i: T.t_Operations vV |}
    (myself: V.t_PolynomialRingElement vV)
    (result: V.t_PolynomialRingElement vV) :
  Lemma
    (requires
      forall (k: nat). k < 16 ==>
        TS.barrett_reduce_post
          (T.f_repr (Seq.index myself.V.f_coefficients k))
          (T.f_repr (Seq.index result.V.f_coefficients k)))
    (ensures
       to_spec_poly_plain result
         == HP.poly_barrett_reduce (to_spec_poly_plain myself))

val lemma_add_to_ring_element_commute
    (#vV: Type0) {| i: T.t_Operations vV |}
    (myself rhs result: V.t_PolynomialRingElement vV) :
  Lemma
    (requires
      forall (k: nat). k < 16 ==>
        TS.add_post
          (T.f_repr (Seq.index myself.V.f_coefficients k))
          (T.f_repr (Seq.index rhs.V.f_coefficients k))
          (T.f_repr (Seq.index result.V.f_coefficients k)))
    (ensures
       to_spec_poly_plain result
         == HP.add_to_ring_element
              (to_spec_poly_plain myself) (to_spec_poly_plain rhs))

let fe_1441 : P.t_FieldElement = P.impl_FieldElement__new (mk_u16 1441)

(* Opaque per-vector wrapper for the per-lane FE finalize relation.  Bundles
   16 per-lane equations into a single opaque atom; without opacity the
   inner forall pollutes loop-invariant subtyping checks (Z3 instantiates
   at every (j, k) pair, blowing rlimit). *)

val subtract_reduce_finalize_chunk
    (myself_chunk b_chunk _b_chunk: t_Array i16 (mk_usize 16)) : prop

val lemma_subtract_reduce_iter
    (myself_chunk b_chunk_in cnf_chunk diff_chunk red_chunk: t_Array i16 (mk_usize 16)) :
    Lemma
      (requires
        TS.montgomery_multiply_by_constant_post b_chunk_in (mk_i16 1441) cnf_chunk /\
        TS.sub_post myself_chunk cnf_chunk diff_chunk /\
        TS.barrett_reduce_post diff_chunk red_chunk)
      (ensures
        subtract_reduce_finalize_chunk myself_chunk red_chunk b_chunk_in)

val subtract_reduce_helper
    (a b: t_Array P.t_FieldElement (mk_usize 256))
    : t_Array P.t_FieldElement (mk_usize 256)

val lemma_subtract_reduce_eq_helper
    (a b: t_Array P.t_FieldElement (mk_usize 256)) :
    Lemma (HP.subtract_reduce a b == subtract_reduce_helper a b)

val lemma_subtract_reduce_commute
    (#vV: Type0) {| i: T.t_Operations vV |}
    (myself b_input b_post: V.t_PolynomialRingElement vV) :
    Lemma
      (requires
        forall (k: nat). k < 16 ==>
          subtract_reduce_finalize_chunk
            (T.f_repr (Seq.index myself.V.f_coefficients k))
            (T.f_repr (Seq.index b_post.V.f_coefficients k))
            (T.f_repr (Seq.index b_input.V.f_coefficients k)))
      (ensures
        to_spec_poly_plain b_post ==
        subtract_reduce_helper
          (to_spec_poly_plain myself)
          (P.createi #P.t_FieldElement (mk_usize 256)
             #(usize -> P.t_FieldElement)
             (fun (j: usize {j <. mk_usize 256}) ->
               P.impl_FieldElement__mul
                 (Seq.index (to_spec_poly_mont b_input) (v j))
                 fe_1441)))

val add_error_reduce_finalize_chunk
    (myself_chunk red_chunk error_chunk: t_Array i16 (mk_usize 16)) : prop

val lemma_add_error_reduce_iter
    (myself_chunk error_chunk cnf_chunk sum_chunk red_chunk: t_Array i16 (mk_usize 16)) :
    Lemma
      (requires
        TS.montgomery_multiply_by_constant_post myself_chunk (mk_i16 1441) cnf_chunk /\
        TS.add_post cnf_chunk error_chunk sum_chunk /\
        TS.barrett_reduce_post sum_chunk red_chunk)
      (ensures
        add_error_reduce_finalize_chunk myself_chunk red_chunk error_chunk)

val add_error_reduce_helper
    (a b: t_Array P.t_FieldElement (mk_usize 256))
    : t_Array P.t_FieldElement (mk_usize 256)

val lemma_add_error_reduce_eq_helper
    (a b: t_Array P.t_FieldElement (mk_usize 256)) :
    Lemma (HP.add_error_reduce a b == add_error_reduce_helper a b)

val lemma_add_error_reduce_commute
    (#vV: Type0) {| i: T.t_Operations vV |}
    (myself_input error myself_post: V.t_PolynomialRingElement vV) :
    Lemma
      (requires
        forall (k: nat). k < 16 ==>
          add_error_reduce_finalize_chunk
            (T.f_repr (Seq.index myself_input.V.f_coefficients k))
            (T.f_repr (Seq.index myself_post.V.f_coefficients k))
            (T.f_repr (Seq.index error.V.f_coefficients k)))
      (ensures
        to_spec_poly_plain myself_post ==
        add_error_reduce_helper
          (P.createi #P.t_FieldElement (mk_usize 256)
             #(usize -> P.t_FieldElement)
             (fun (j: usize {j <. mk_usize 256}) ->
               P.impl_FieldElement__mul
                 (Seq.index (to_spec_poly_mont myself_input) (v j))
                 fe_1441))
          (to_spec_poly_plain error))

val add_message_error_reduce_finalize_chunk
    (myself_chunk message_chunk red_chunk result_chunk: t_Array i16 (mk_usize 16)) : prop

val lemma_add_message_error_reduce_iter
    (myself_chunk message_chunk result_chunk cnf_chunk sum1_chunk sum2_chunk red_chunk:
        t_Array i16 (mk_usize 16)) :
    Lemma
      (requires
        TS.montgomery_multiply_by_constant_post result_chunk (mk_i16 1441) cnf_chunk /\
        TS.add_post myself_chunk message_chunk sum1_chunk /\
        TS.add_post cnf_chunk sum1_chunk sum2_chunk /\
        TS.barrett_reduce_post sum2_chunk red_chunk)
      (ensures
        add_message_error_reduce_finalize_chunk
          myself_chunk message_chunk red_chunk result_chunk)

val add_message_error_reduce_helper
    (a b c: t_Array P.t_FieldElement (mk_usize 256))
    : t_Array P.t_FieldElement (mk_usize 256)

val lemma_add_message_error_reduce_eq_helper
    (a b c: t_Array P.t_FieldElement (mk_usize 256)) :
    Lemma (HP.add_message_error_reduce a b c == add_message_error_reduce_helper a b c)

val lemma_add_message_error_reduce_commute
    (#vV: Type0) {| i: T.t_Operations vV |}
    (myself message result_input myself_post: V.t_PolynomialRingElement vV) :
    Lemma
      (requires
        forall (k: nat). k < 16 ==>
          add_message_error_reduce_finalize_chunk
            (T.f_repr (Seq.index myself.V.f_coefficients k))
            (T.f_repr (Seq.index message.V.f_coefficients k))
            (T.f_repr (Seq.index myself_post.V.f_coefficients k))
            (T.f_repr (Seq.index result_input.V.f_coefficients k)))
      (ensures
        to_spec_poly_plain myself_post ==
        add_message_error_reduce_helper
          (to_spec_poly_plain myself)
          (to_spec_poly_plain message)
          (P.createi #P.t_FieldElement (mk_usize 256)
             #(usize -> P.t_FieldElement)
             (fun (j: usize {j <. mk_usize 256}) ->
               P.impl_FieldElement__mul
                 (Seq.index (to_spec_poly_mont result_input) (v j))
                 fe_1441)))

val mont_form_lane
    (input_lane: i16) (plain_lane: P.t_FieldElement) : prop

(* Intro for the opaque `mont_form_lane`: given the raw mod-q equality,
   establish the atom (consumers that build `plain` from `std_i16_to_spec_fe`
   use this instead of a local reveal, which would re-prove the body and
   saturate). *)
val lemma_mont_form_lane_intro
    (input_lane: i16) (plain_lane: P.t_FieldElement) :
    Lemma (requires
            (v input_lane * 2285) % 3329 == v plain_lane.P.f_val % 3329)
          (ensures mont_form_lane input_lane plain_lane)

val lemma_add_standard_error_reduce_lane
    (myself_lane normal_lane error_lane sum_lane red_lane: i16)
    (plain: P.t_FieldElement) :
    Lemma (requires
            mont_form_lane myself_lane plain /\
            (* From `mont_mul(myself, 1353)` post: v normal % q == v myself * 1353 * 169 % q *)
            v normal_lane % 3329 == (v myself_lane * 1353 * 169) % 3329 /\
            v sum_lane == v normal_lane + v error_lane /\
            v red_lane % 3329 == v sum_lane % 3329)
          (ensures
            i16_to_spec_fe red_lane
              == P.impl_FieldElement__add plain
                   (i16_to_spec_fe error_lane))

val lemma_add_standard_error_reduce_commute
    (#vV: Type0) {| i: T.t_Operations vV |}
    (myself error result: V.t_PolynomialRingElement vV)
    (ntt_product: t_Array P.t_FieldElement (mk_usize 256)) :
  Lemma
    (requires
      (* Per-lane FE-add equation specialized to the slice of ntt_product
         at chunk k.  The caller's loop body proves this directly via
         `lemma_add_standard_error_reduce_lane` after each iteration. *)
      forall (k: nat) (l: nat). k < 16 /\ l < 16 ==>
        i16_to_spec_fe
          (Seq.index (T.f_repr (Seq.index result.V.f_coefficients k)) l)
          == P.impl_FieldElement__add
               (Seq.index
                 (Seq.slice ntt_product (k * 16) (k * 16 + 16)) l)
               (i16_to_spec_fe
                 (Seq.index (T.f_repr (Seq.index error.V.f_coefficients k)) l)))
    (ensures
       to_spec_poly_plain result
         == HP.add_standard_error_reduce ntt_product (to_spec_poly_plain error))

val mont_array_lane (#n: usize)
    (x: t_Array i16 n) (i: usize { v i < v n }) :
    Lemma (Seq.index (mont_i16_to_spec_array n x) (v i)
           == mont_i16_to_spec_fe (Seq.index x (v i)))

val zetas_4_lane (z0 z1 z2 z3: i16) (i: usize { v i < 4 }) :
    Lemma (Seq.index (zetas_4_ z0 z1 z2 z3) (v i)
           == (if v i = 0 then mont_i16_to_spec_fe z0
               else if v i = 1 then mont_i16_to_spec_fe z1
               else if v i = 2 then mont_i16_to_spec_fe z2
               else mont_i16_to_spec_fe z3))

val lemma_ntt_layer_n_16_2_lane
    (p: t_Array P.t_FieldElement (mk_usize 16))
    (zs: t_Array P.t_FieldElement (mk_usize 4))
    (i: nat {i < 16}) :
    Lemma
      (let result = N.ntt_layer_n (mk_usize 16) p (mk_usize 2)
                                  (Rust_primitives.unsize zs) in
       let group : nat = i / 4 in
       let idx   : nat = i % 4 in
       (idx < 2 ==>
         i + 2 < 16 /\
         Seq.index result i ==
           (N.butterfly (Seq.index zs group)
                        (Seq.index p i)
                        (Seq.index p (i + 2)))._1) /\
       (idx >= 2 ==>
         i >= 2 /\
         Seq.index result i ==
           (N.butterfly (Seq.index zs group)
                        (Seq.index p (i - 2))
                        (Seq.index p i))._2))

(* 5 private lane-bridge lemmas (lemma_ntt_layer_1_step_branch_{0..3}_lane_bridge,
   lemma_ntt_layer_1_step_lane_bridge) stay module-private in Chunk.fst --
   not exposed. *)

val lemma_ntt_layer_1_step_to_hacspec
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

let to_spec_poly_mont_arr
    (#vV: Type0) {| i: T.t_Operations vV |}
    (a: t_Array vV (mk_usize 16))
    : t_Array P.t_FieldElement (mk_usize 256)
  = P.createi #P.t_FieldElement (mk_usize 256)
        #(usize -> P.t_FieldElement)
        (fun (j: usize { j <. mk_usize 256 }) ->
          (mont_i16_to_spec_fe
            (Seq.index (T.f_repr (Seq.index a (v j / 16)))
                       (v j % 16))
           <: P.t_FieldElement))

let to_spec_poly_plain_arr
    (#vV: Type0) {| i: T.t_Operations vV |}
    (a: t_Array vV (mk_usize 16))
    : t_Array P.t_FieldElement (mk_usize 256)
  = P.createi #P.t_FieldElement (mk_usize 256)
        #(usize -> P.t_FieldElement)
        (fun (j: usize { j <. mk_usize 256 }) ->
          (i16_to_spec_fe
            (Seq.index (T.f_repr (Seq.index a (v j / 16)))
                       (v j % 16))
           <: P.t_FieldElement))

(* ntt_multiply per-lane + chunk-commute lemmas (consumed by Polynomial;
   were missing from the firewall surface). *)
val lemma_ntt_multiply_n_16_lane
    (p1 p2: t_Array P.t_FieldElement (mk_usize 16))
    (zs: t_Array P.t_FieldElement (mk_usize 4))
    (i: nat {i < 16}) :
    Lemma
      (let result = N.ntt_multiply_n (mk_usize 16) p1 p2
                                     (Rust_primitives.unsize zs) in
       let group : nat = i / 4 in
       let zeta = (if i % 4 < 2 then Seq.index zs group
                   else P.impl_FieldElement__neg (Seq.index zs group)) in
       (i % 2 = 0 ==>
         i + 1 < 16 /\
         Seq.index result i ==
           N.base_case_multiply_even (Seq.index p1 i) (Seq.index p1 (i + 1))
                                     (Seq.index p2 i) (Seq.index p2 (i + 1))
                                     zeta) /\
       (i % 2 = 1 ==>
         i >= 1 /\
         Seq.index result i ==
           N.base_case_multiply_odd (Seq.index p1 (i - 1)) (Seq.index p1 i)
                                    (Seq.index p2 (i - 1)) (Seq.index p2 i)))

val lemma_ntt_multiply_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (lhs rhs: vV) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires TS.ntt_multiply_pre (T.f_repr lhs) (T.f_repr rhs)
                                  zeta0 zeta1 zeta2 zeta3)
    (ensures
       (let r = T.f_ntt_multiply lhs rhs zeta0 zeta1 zeta2 zeta3 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r)
          == N.ntt_multiply_n (mk_usize 16)
               (mont_i16_to_spec_array (sz 16) (T.f_repr lhs))
               (mont_i16_to_spec_array (sz 16) (T.f_repr rhs))
               (zetas_4_ zeta0 zeta1 zeta2 zeta3)))

(* Unfold lemma: `to_spec_poly_mont` only consumes `p.f_coefficients`,
   so it must equal `to_spec_poly_mont_arr p.f_coefficients`.  The
   bodies are structurally identical except the record projection.
   Single-line proof via `Seq.lemma_eq_intro` on the createi outputs. *)

val lemma_to_spec_poly_mont_unfold
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p: V.t_PolynomialRingElement vV) :
    Lemma (to_spec_poly_mont p == to_spec_poly_mont_arr p.V.f_coefficients)

val lemma_to_spec_poly_plain_unfold
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p: V.t_PolynomialRingElement vV) :
    Lemma (to_spec_poly_plain p == to_spec_poly_plain_arr p.V.f_coefficients)

val lemma_subtract_reduce_scaled_eq
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p q: V.t_PolynomialRingElement vV) :
    Lemma (requires p.V.f_coefficients == q.V.f_coefficients)
          (ensures
            (P.createi #P.t_FieldElement (mk_usize 256)
               #(usize -> P.t_FieldElement)
               (fun (j: usize {j <. mk_usize 256}) ->
                 P.impl_FieldElement__mul
                   (Seq.index (to_spec_poly_mont p) (v j))
                   fe_1441))
            ==
            (P.createi #P.t_FieldElement (mk_usize 256)
               #(usize -> P.t_FieldElement)
               (fun (j: usize {j <. mk_usize 256}) ->
                 P.impl_FieldElement__mul
                   (Seq.index (to_spec_poly_mont q) (v j))
                   fe_1441)))

val lemma_add_error_reduce_scaled_eq
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p q: V.t_PolynomialRingElement vV) :
    Lemma (requires p.V.f_coefficients == q.V.f_coefficients)
          (ensures
            (P.createi #P.t_FieldElement (mk_usize 256)
               #(usize -> P.t_FieldElement)
               (fun (j: usize {j <. mk_usize 256}) ->
                 P.impl_FieldElement__mul
                   (Seq.index (to_spec_poly_mont p) (v j))
                   fe_1441))
            ==
            (P.createi #P.t_FieldElement (mk_usize 256)
               #(usize -> P.t_FieldElement)
               (fun (j: usize {j <. mk_usize 256}) ->
                 P.impl_FieldElement__mul
                   (Seq.index (to_spec_poly_mont q) (v j))
                   fe_1441)))

val lemma_add_message_error_reduce_scaled_eq
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p q: V.t_PolynomialRingElement vV) :
    Lemma (requires p.V.f_coefficients == q.V.f_coefficients)
          (ensures
            (P.createi #P.t_FieldElement (mk_usize 256)
               #(usize -> P.t_FieldElement)
               (fun (j: usize {j <. mk_usize 256}) ->
                 P.impl_FieldElement__mul
                   (Seq.index (to_spec_poly_mont p) (v j))
                   fe_1441))
            ==
            (P.createi #P.t_FieldElement (mk_usize 256)
               #(usize -> P.t_FieldElement)
               (fun (j: usize {j <. mk_usize 256}) ->
                 P.impl_FieldElement__mul
                   (Seq.index (to_spec_poly_mont q) (v j))
                   fe_1441)))

val lemma_compress_d_barrett_eq (fe: int) (d: nat)
  : Lemma (requires 0 <= fe /\ fe < 3329 /\ (d == 4 \/ d == 5 \/ d == 10 \/ d == 11))
          (ensures (((fe * pow2 d + 1664) * 10321340) / pow2 35) % pow2 d
                   == ((fe * 2 * pow2 d + 3329) / 6658) % pow2 d)

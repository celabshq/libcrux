module Hacspec_ml_kem.Commute.Matrix_zerolift
#set-options "--fuel 1 --ifuel 1 --z3rlimit 100"
open FStar.Mul
open Core_models
module P = Hacspec_ml_kem.Parameters
module T = Libcrux_ml_kem.Vector.Traits
module V = Libcrux_ml_kem.Vector.Traits.Spec
module VV = Libcrux_ml_kem.Vector
module Poly = Libcrux_ml_kem.Polynomial
module Spec = Libcrux_ml_kem.Polynomial.Spec
module ML = FStar.Math.Lemmas

(* The all-zero spec FieldElement. *)
let zero_fe : P.t_FieldElement = P.impl_FieldElement__new (mk_u16 0)

(* std_i16_to_spec_fe of a zero-valued i16 lane is the zero FieldElement.
   The post of std_i16_to_spec_fe gives v r.f_val == (v x * 2285) % 3329;
   with v x == 0 that is 0, so r.f_val = mk_u16 0 = zero_fe.f_val.  Since
   t_FieldElement is a single-field record, f_val equality forces the
   FieldElement equality. *)
#push-options "--z3rlimit 50 --fuel 0 --ifuel 0"
let lemma_std_fe_zero (x: i16)
  : Lemma (requires v x == 0)
          (ensures Poly.std_i16_to_spec_fe x == zero_fe)
  = let r = Poly.std_i16_to_spec_fe x in
    assert (v r.P.f_val == (v x * 2285) % 3329);
    assert (v r.P.f_val == 0);
    assert (v zero_fe.P.f_val == 0)
#pop-options

(* Per-index: with a zero-bounded poly, the standard lift at index j is
   the zero FieldElement. *)
#push-options "--z3rlimit 100 --fuel 1 --ifuel 1"
let lemma_zero_lift_index
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (z: VV.t_PolynomialRingElement vV)
    (j: usize)
  : Lemma
    (requires Spec.is_bounded_poly #vV (mk_usize 0) z /\ v j < 256)
    (ensures Seq.index (Poly.to_spec_poly_standard #vV z) (v j) == zero_fe)
  = (* Peel the createi for index j (mirrors lemma_ntt_product_slice). *)
    let chunk_idx : nat = v j / 16 in
    let lane_idx  : nat = v j % 16 in
    ML.lemma_div_lt_nat (v j) 8 4;       (* v j < 256 ==> v j / 16 < 16 *)
    assert (chunk_idx < 16);
    assert (lane_idx < 16);
    (* is_bounded_poly is transparent: forall i<16. is_bounded_vector 0 (coeffs.[sz i]) *)
    assert (Spec.is_bounded_vector #vV (mk_usize 0)
              (z.VV.f_coefficients.[ sz chunk_idx ] <: vV));
    (* coeffs.[sz chunk_idx] == Seq.index coeffs chunk_idx *)
    let vec : vV = Seq.index z.VV.f_coefficients chunk_idx in
    assert (z.VV.f_coefficients.[ sz chunk_idx ] == vec);
    (* Inlined bridge (mirrors Poly.lemma_is_i16b_repr_of_bounded, which is
       not exported in the .fsti):
         is_bounded_vector 0 vec
           == is_i16b_array_opaque 0 (f_to_i16_array vec)   [by def]
       and the trait law f_to_i16_array vec == f_repr vec (fired by calling
       f_to_i16_array) bridges to f_repr. *)
    reveal_opaque (`%V.is_i16b_array_opaque)
      (V.is_i16b_array_opaque 0 (T.f_to_i16_array #vV vec));
    let _ = T.f_to_i16_array #vV vec in
    reveal_opaque (`%V.is_i16b_array_opaque)
      (V.is_i16b_array_opaque 0 (T.f_repr #vV vec));
    assert (V.is_i16b_array_opaque 0 (T.f_repr #vV vec));
    reveal_opaque (`%V.is_i16b_array_opaque) (V.is_i16b_array_opaque 0 (T.f_repr #vV vec));
    (* Now is_i16b_array 0 (f_repr vec): every lane has |lane| <= 0, i.e == 0. *)
    let repr = T.f_repr #vV vec in
    assert (Seq.length repr == 16);
    assert (V.is_i16b 0 (Seq.index repr lane_idx));
    let lane : i16 = Seq.index repr lane_idx in
    assert (v lane == 0);
    (* createi peel: Seq.index (to_spec_poly_standard z) (v j)
                       == std_i16_to_spec_fe lane *)
    P.createi_lemma #P.t_FieldElement (mk_usize 256)
      #(usize -> P.t_FieldElement)
      (fun (jj: usize { jj <. mk_usize 256 }) ->
        (Poly.std_i16_to_spec_fe
          (Seq.index (T.f_repr #vV
                       (Seq.index z.VV.f_coefficients (v jj / 16)))
                     (v jj % 16))
         <: P.t_FieldElement))
      j;
    assert (Seq.index (Poly.to_spec_poly_standard #vV z) (v j)
              == Poly.std_i16_to_spec_fe lane);
    lemma_std_fe_zero lane
#pop-options

#push-options "--z3rlimit 100 --fuel 1 --ifuel 1"
let lemma_zero_lift
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (z: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires Spec.is_bounded_poly #vV (mk_usize 0) z)
    (ensures
      Poly.to_spec_poly_standard #vV z
      == Rust_primitives.Hax.repeat (P.impl_FieldElement__new (mk_u16 0)) (mk_usize 256))
  = let lhs = Poly.to_spec_poly_standard #vV z in
    let rhs = Rust_primitives.Hax.repeat zero_fe (mk_usize 256) in
    let aux (j: nat{j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
      lemma_zero_lift_index #vV z (sz j);
      assert (Seq.index lhs j == zero_fe);
      FStar.Seq.lemma_index_create 256 zero_fe j;
      assert (Seq.index rhs j == zero_fe)
    in
    Classical.forall_intro aux;
    assert (Seq.length lhs == 256);
    assert (Seq.length rhs == 256);
    Seq.lemma_eq_intro lhs rhs
#pop-options

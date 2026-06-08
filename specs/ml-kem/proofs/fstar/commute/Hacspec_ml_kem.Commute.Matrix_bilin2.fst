module Hacspec_ml_kem.Commute.Matrix_bilin2
#set-options "--fuel 0 --ifuel 1 --z3rlimit 50"
open FStar.Mul
open Core_models
module P = Hacspec_ml_kem.Parameters
module N = Hacspec_ml_kem.Ntt
module V = Libcrux_ml_kem.Vector.Traits.Spec
module T = Libcrux_ml_kem.Vector.Traits
module VV = Libcrux_ml_kem.Vector
module CH = Hacspec_ml_kem.Commute.Chunk
module HP = Hacspec_ml_kem.Polynomial
module Poly = Libcrux_ml_kem.Polynomial
module SB = Hacspec_ml_kem.Commute.Matrix_bilin
module ML = FStar.Math.Lemmas

let r2 : P.t_FieldElement = P.impl_FieldElement__mul SB.r_fe SB.r_fe

(* H1 (per-lane): std = R^2 * mont, by chaining std = R*plain and plain = R*mont. *)
let lemma_std_eq_R2_mont (x: i16) : Lemma
  (Poly.std_i16_to_spec_fe x == P.impl_FieldElement__mul r2 (V.mont_i16_to_spec_fe x))
= SB.lemma_std_eq_R_plain x;          (* std x == r_fe * (i16_to_spec_fe x) *)
  SB.lemma_plain_eq_R_mont x;         (* i16_to_spec_fe x == r_fe * (mont x) *)
  SB.mul_assoc SB.r_fe SB.r_fe (V.mont_i16_to_spec_fe x)

(* index helper for to_spec_poly_standard (mirror of CH.poly_lane_plain/mont) *)
let poly_lane_std
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (p: VV.t_PolynomialRingElement vV) (j: nat {j < 256}) :
    Lemma (Seq.index (Poly.to_spec_poly_standard #vV p) j
           == Poly.std_i16_to_spec_fe
                (Seq.index (T.f_repr (Seq.index p.VV.f_coefficients (j / 16))) (j % 16)))
= P.createi_lemma #P.t_FieldElement (mk_usize 256)
    #(usize -> P.t_FieldElement)
    (fun (jj: usize { jj <. mk_usize 256 }) ->
      (Poly.std_i16_to_spec_fe
        (Seq.index (T.f_repr (Seq.index p.VV.f_coefficients (v jj / 16))) (v jj % 16))
       <: P.t_FieldElement))
    (sz j)

(* array form: plain a == scale_poly r_fe (mont a) *)
#push-options "--z3rlimit 100"
let lemma_plain_arr
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (p: VV.t_PolynomialRingElement vV) :
    Lemma (CH.to_spec_poly_plain #vV p == SB.scale_poly SB.r_fe (CH.to_spec_poly_mont #vV p))
= let mont = CH.to_spec_poly_mont #vV p in
  let lhs  = CH.to_spec_poly_plain #vV p in
  let rhs  = SB.scale_poly SB.r_fe mont in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    CH.poly_lane_plain #vV p j;
    CH.poly_lane_mont #vV p j;
    SB.lemma_scale_poly_index SB.r_fe mont j;
    SB.lemma_plain_eq_R_mont (Seq.index (T.f_repr (Seq.index p.VV.f_coefficients (j / 16))) (j % 16))
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* array form: std prod == scale_poly r2 (mont prod) *)
#push-options "--z3rlimit 100"
let lemma_std_arr
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (p: VV.t_PolynomialRingElement vV) :
    Lemma (Poly.to_spec_poly_standard #vV p == SB.scale_poly r2 (CH.to_spec_poly_mont #vV p))
= let mont = CH.to_spec_poly_mont #vV p in
  let lhs  = Poly.to_spec_poly_standard #vV p in
  let rhs  = SB.scale_poly r2 mont in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    poly_lane_std #vV p j;
    CH.poly_lane_mont #vV p j;
    SB.lemma_scale_poly_index r2 mont j;
    lemma_std_eq_R2_mont (Seq.index (T.f_repr (Seq.index p.VV.f_coefficients (j / 16))) (j % 16))
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* array form of the per-lane bilinearity *)
#push-options "--z3rlimit 100"
let lemma_poly_bilin
    (p1 p2: t_Array P.t_FieldElement (mk_usize 256))
    (zs: t_Slice P.t_FieldElement) :
    Lemma
      (requires
        (Core_models.Slice.impl__len #P.t_FieldElement zs <: usize) <. mk_usize 1024 &&
        ((Core_models.Slice.impl__len #P.t_FieldElement zs <: usize) *! mk_usize 4 <: usize) =.
          mk_usize 256)
      (ensures
        N.ntt_multiply_n (mk_usize 256) (SB.scale_poly SB.r_fe p1) (SB.scale_poly SB.r_fe p2) zs
        == SB.scale_poly r2 (N.ntt_multiply_n (mk_usize 256) p1 p2 zs))
= let lhs  = N.ntt_multiply_n (mk_usize 256) (SB.scale_poly SB.r_fe p1) (SB.scale_poly SB.r_fe p2) zs in
  let base = N.ntt_multiply_n (mk_usize 256) p1 p2 zs in
  let rhs  = SB.scale_poly r2 base in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    SB.lemma_poly_bilin_lane p1 p2 zs j;
    SB.lemma_scale_poly_index r2 base j
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* HP.ntt_multiply bilinearity (folds in the v_ZETAS[64..128] slice) *)
#push-options "--z3rlimit 150"
let lemma_HP_bilin (p1 p2: t_Array P.t_FieldElement (mk_usize 256)) :
    Lemma (HP.ntt_multiply (SB.scale_poly SB.r_fe p1) (SB.scale_poly SB.r_fe p2)
           == SB.scale_poly r2 (HP.ntt_multiply p1 p2))
= let zmul = N.v_ZETAS.[ ({ Core_models.Ops.Range.f_start = mk_usize 64;
                            Core_models.Ops.Range.f_end   = mk_usize 128 }
                          <: Core_models.Ops.Range.t_Range usize) ] in
  assert ((Core_models.Slice.impl__len #P.t_FieldElement zmul <: usize) =. mk_usize 64);
  lemma_poly_bilin p1 p2 zmul
#pop-options

(* ★ bundled: standard-domain ntt_multiply output == plain-domain HP.ntt_multiply of plain inputs *)
#push-options "--z3rlimit 150"
let lemma_ntt_multiply_standard
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (a s prod: VV.t_PolynomialRingElement vV) :
    Lemma
      (requires
        CH.to_spec_poly_mont #vV prod
        == HP.ntt_multiply (CH.to_spec_poly_mont #vV a) (CH.to_spec_poly_mont #vV s))
      (ensures
        Poly.to_spec_poly_standard #vV prod
        == HP.ntt_multiply (CH.to_spec_poly_plain #vV a) (CH.to_spec_poly_plain #vV s))
= lemma_plain_arr #vV a;
  lemma_plain_arr #vV s;
  lemma_std_arr #vV prod;
  lemma_HP_bilin (CH.to_spec_poly_mont #vV a) (CH.to_spec_poly_mont #vV s)
#pop-options

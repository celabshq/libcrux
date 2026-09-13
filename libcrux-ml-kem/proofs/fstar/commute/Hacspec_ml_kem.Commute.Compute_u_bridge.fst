module Hacspec_ml_kem.Commute.Compute_u_bridge
#set-options "--fuel 1 --ifuel 1 --z3rlimit 100"
open FStar.Mul
open Core_models
module P   = Hacspec_ml_kem.Parameters
module N   = Hacspec_ml_kem.Ntt
module IN  = Hacspec_ml_kem.Invert_ntt
module MX  = Hacspec_ml_kem.Matrix
module HP  = Hacspec_ml_kem.Polynomial
module T   = Libcrux_ml_kem.Vector.Traits
module V   = Libcrux_ml_kem.Vector.Traits.Spec
module VV  = Libcrux_ml_kem.Vector
module VS  = Libcrux_ml_kem.Vector.Spec
module Poly = Libcrux_ml_kem.Polynomial
module CH  = Hacspec_ml_kem.Commute.Chunk
module SB  = Hacspec_ml_kem.Commute.Matrix_bilin
module SB2 = Hacspec_ml_kem.Commute.Matrix_bilin2
module MB  = Hacspec_ml_kem.Commute.Matrix_bridge
module IS  = Hacspec_ml_kem.Commute.Invert_scale
module Br  = Hacspec_ml_kem.Commute.Bridges
module ML  = FStar.Math.Lemmas

(* ════════════════════════════════════════════════════════════════════
   Finalize bridge for compute_vector_u's per-row INTT + add_error_reduce.
   Given the inner-dot result (std(t_pre) == part == multiply_matrix_by_column[i]),
   the invert_ntt_montgomery post (mont(re_future) == ntt_inverse_butterflies(mont(t_pre)))
   and the add_error_reduce post (plain(t_final) == HP.add_error_reduce (scaled_mont re_future)
   (plain error_i)), conclude poly_to_spec(t_final) == compute_vector_u(...)[i].
   The scalar mismatch between the standard-domain inner product and the Mont-domain
   inverse NTT is moved through ntt_inverse_butterflies by IS.lemma_*_scale (linearity).
   Key constants: std = scale fe_1353 mont (1353=R²); reduce_polynomial = scale 3303;
   3303·1353 ≡ 1441 (= fe_1441 used by the fused mont_mul finalize). *)

let q : pos = 3329
let fe_1353 : P.t_FieldElement = P.impl_FieldElement__new (mk_u16 1353)

(* the add_error_reduce ntt_product operand: the Mont lift of `re` scaled by fe_1441 *)
let scaled_mont (#vV: Type0) {| iop: T.t_Operations vV |}
    (re: VV.t_PolynomialRingElement vV)
  : t_Array P.t_FieldElement (mk_usize 256)
= P.createi #P.t_FieldElement (mk_usize 256) #(usize -> P.t_FieldElement)
    (fun (j: usize {j <. mk_usize 256}) ->
      P.impl_FieldElement__mul (Seq.index (CH.to_spec_poly_mont #vV re) (v j)) CH.fe_1441
      <: P.t_FieldElement)

(* ---- FE scalar facts ---- *)

let lemma_mul_comm (a b: P.t_FieldElement) : Lemma
  (P.impl_FieldElement__mul a b == P.impl_FieldElement__mul b a)
= SB.mul_val a b; SB.mul_val b a;
  assert (v a.f_val * v b.f_val == v b.f_val * v a.f_val);
  SB.fe_eq (P.impl_FieldElement__mul a b) (P.impl_FieldElement__mul b a)

let lemma_rfe_sq_eq_1353 () : Lemma (P.impl_FieldElement__mul SB.r_fe SB.r_fe == fe_1353)
= SB.mul_val SB.r_fe SB.r_fe;
  assert_norm ((2285 * 2285) % 3329 == 1353);
  SB.fe_eq (P.impl_FieldElement__mul SB.r_fe SB.r_fe) fe_1353

let lemma_3303_1353_eq_1441 () : Lemma
  (P.impl_FieldElement__mul IN.v_INVERSE_OF_128_ fe_1353 == CH.fe_1441)
= SB.mul_val IN.v_INVERSE_OF_128_ fe_1353;
  assert_norm ((3303 * 1353) % 3329 == 1441);
  SB.fe_eq (P.impl_FieldElement__mul IN.v_INVERSE_OF_128_ fe_1353) CH.fe_1441

let lemma_std_eq_1353_mont (c: i16) : Lemma
  (Poly.std_i16_to_spec_fe c == P.impl_FieldElement__mul fe_1353 (V.mont_i16_to_spec_fe c))
= SB.lemma_std_eq_R_plain c;     (* std == r_fe · plain *)
  SB.lemma_plain_eq_R_mont c;    (* plain == r_fe · mont *)
  SB.mul_assoc SB.r_fe SB.r_fe (V.mont_i16_to_spec_fe c);  (* (r·r)·m == r·(r·m) *)
  lemma_rfe_sq_eq_1353 ()        (* r·r == fe_1353 *)

(* ---- array-level scale lemmas ---- *)

(* std(re) == scale_poly fe_1353 (mont(re)) *)
#push-options "--z3rlimit 150"
let lemma_std_arr_mont (#vV: Type0) {| iop: T.t_Operations vV |}
    (re: VV.t_PolynomialRingElement vV) : Lemma
  (Poly.to_spec_poly_standard #vV re == SB.scale_poly fe_1353 (CH.to_spec_poly_mont #vV re))
= let lhs = Poly.to_spec_poly_standard #vV re in
  let mont = CH.to_spec_poly_mont #vV re in
  let rhs = SB.scale_poly fe_1353 mont in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    SB2.poly_lane_std #vV re j;
    CH.poly_lane_mont #vV re j;
    SB.lemma_scale_poly_index fe_1353 mont j;
    lemma_std_eq_1353_mont (Seq.index (T.f_repr (Seq.index re.VV.f_coefficients (j / 16))) (j % 16))
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* scaled_mont re == scale_poly fe_1441 (mont re)  (just mul-commute per lane) *)
#push-options "--z3rlimit 150"
let lemma_scaled_mont_eq_scale (#vV: Type0) {| iop: T.t_Operations vV |}
    (re: VV.t_PolynomialRingElement vV) : Lemma
  (scaled_mont #vV re == SB.scale_poly CH.fe_1441 (CH.to_spec_poly_mont #vV re))
= let mont = CH.to_spec_poly_mont #vV re in
  let lhs = scaled_mont #vV re in
  let rhs = SB.scale_poly CH.fe_1441 mont in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    P.createi_lemma #P.t_FieldElement (mk_usize 256) #(usize -> P.t_FieldElement)
      (fun (k: usize {k <. mk_usize 256}) ->
        P.impl_FieldElement__mul (Seq.index mont (v k)) CH.fe_1441 <: P.t_FieldElement)
      (sz j);
    SB.lemma_scale_poly_index CH.fe_1441 mont j;
    lemma_mul_comm (Seq.index mont j) CH.fe_1441
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* reduce_polynomial X == scale_poly v_INVERSE_OF_128_ X *)
#push-options "--z3rlimit 150"
let lemma_reduce_eq_scale (x: t_Array P.t_FieldElement (mk_usize 256)) : Lemma
  (IN.reduce_polynomial x == SB.scale_poly IN.v_INVERSE_OF_128_ x)
= let lhs = IN.reduce_polynomial x in
  let rhs = SB.scale_poly IN.v_INVERSE_OF_128_ x in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    P.createi_lemma #P.t_FieldElement (mk_usize 256) #(usize -> P.t_FieldElement)
      (fun (i: usize {i <. mk_usize 256}) ->
        P.impl_FieldElement__mul (Seq.index x (v i)) IN.v_INVERSE_OF_128_ <: P.t_FieldElement)
      (sz j);
    SB.lemma_scale_poly_index IN.v_INVERSE_OF_128_ x j;
    lemma_mul_comm (Seq.index x j) IN.v_INVERSE_OF_128_
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* scale composition: scale a (scale b p) == scale (a*b) p *)
#push-options "--z3rlimit 150"
let lemma_scale_compose (a b: P.t_FieldElement) (p: t_Array P.t_FieldElement (mk_usize 256)) : Lemma
  (SB.scale_poly a (SB.scale_poly b p) == SB.scale_poly (P.impl_FieldElement__mul a b) p)
= let lhs = SB.scale_poly a (SB.scale_poly b p) in
  let rhs = SB.scale_poly (P.impl_FieldElement__mul a b) p in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    SB.lemma_scale_poly_index a (SB.scale_poly b p) j;
    SB.lemma_scale_poly_index b p j;
    SB.lemma_scale_poly_index (P.impl_FieldElement__mul a b) p j;
    SB.mul_assoc a b (Seq.index p j)
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* HP.add_error_reduce == MX.add_polynomials  (identical FE-level body) *)
let lemma_add_error_lane (x y: t_Array P.t_FieldElement (mk_usize 256)) (j: nat {j < 256}) : Lemma
  (Seq.index (HP.add_error_reduce x y) j == P.impl_FieldElement__add (Seq.index x j) (Seq.index y j))
= P.createi_lemma #P.t_FieldElement (mk_usize 256) #(usize -> P.t_FieldElement)
    (fun (i: usize {i <. mk_usize 256}) ->
       P.impl_FieldElement__new (cast (((cast ((x.[ i ] <: P.t_FieldElement).P.f_val <: u16) <: u32) +!
             (cast ((y.[ i ] <: P.t_FieldElement).P.f_val <: u16) <: u32) <: u32) %!
           (cast (P.v_FIELD_MODULUS <: u16) <: u32) <: u32) <: u16)
       <: P.t_FieldElement)
    (sz j)

#push-options "--z3rlimit 150"
let lemma_add_error_eq_add_poly (x y: t_Array P.t_FieldElement (mk_usize 256)) : Lemma
  (HP.add_error_reduce x y == MX.add_polynomials x y)
= let lhs = HP.add_error_reduce x y in
  let rhs = MX.add_polynomials x y in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    lemma_add_error_lane x y j;
    MB.lemma_add_poly_lane x y (sz j)
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* ---- the scaled operand equals the spec inverse NTT of the std inner product ---- *)
#push-options "--z3rlimit 250"
let lemma_scaled_operand_eq_ntt_inverse
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (t_pre re_future: VV.t_PolynomialRingElement vV) : Lemma
  (requires
    CH.to_spec_poly_mont #vV re_future
      == IN.ntt_inverse_butterflies (CH.to_spec_poly_mont #vV t_pre))
  (ensures
    scaled_mont #vV re_future == IN.ntt_inverse (Poly.to_spec_poly_standard #vV t_pre))
= let mont_pre = CH.to_spec_poly_mont #vV t_pre in
  let mont_re  = CH.to_spec_poly_mont #vV re_future in
  let std_pre  = Poly.to_spec_poly_standard #vV t_pre in
  (* operand == scale fe_1441 mont_re *)
  lemma_scaled_mont_eq_scale #vV re_future;
  (* std_pre == scale 1353 mont_pre *)
  lemma_std_arr_mont #vV t_pre;
  (* NIB(scale 1353 mont_pre) == scale 1353 (NIB mont_pre) == scale 1353 mont_re *)
  IS.lemma_ntt_inverse_butterflies_scale fe_1353 mont_pre;
  (* ntt_inverse std_pre == reduce(NIB std_pre) == scale 3303 (NIB std_pre) *)
  lemma_reduce_eq_scale (IN.ntt_inverse_butterflies std_pre);
  (* scale 3303 (scale 1353 mont_re) == scale (3303*1353) mont_re == scale 1441 mont_re *)
  lemma_scale_compose IN.v_INVERSE_OF_128_ fe_1353 mont_re;
  lemma_3303_1353_eq_1441 ()
#pop-options

(* ---- spec unfold of compute_vector_u[i] ---- *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_compute_vector_u_index
    (#v_K: usize)
    (a_as_ntt: t_Array (t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) v_K)
    (r_as_ntt error_1_: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    (i: usize {v i < v v_K}) : Lemma
  (Seq.index (MX.compute_vector_u v_K a_as_ntt r_as_ntt error_1_) (v i)
   == MX.add_polynomials
        (IN.ntt_inverse (Seq.index (MX.multiply_matrix_by_column v_K
                                      (MX.transpose v_K a_as_ntt) r_as_ntt) (v i)))
        (Seq.index error_1_ (v i)))
= ()
#pop-options

(* ════ MAIN finalize: per-row poly_to_spec(t_final) == compute_vector_u(...)[i] ════ *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_u_row_finalize
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (a_as_ntt: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
    (r error: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (t_pre re_future t_final: VV.t_PolynomialRingElement vV)
    (i: usize {v i < v v_K}) : Lemma
  (requires
    MB.inner_done t_pre a_as_ntt r i (v v_K) /\
    CH.to_spec_poly_mont #vV re_future
      == IN.ntt_inverse_butterflies (CH.to_spec_poly_mont #vV t_pre) /\
    CH.to_spec_poly_plain #vV t_final
      == HP.add_error_reduce (scaled_mont #vV re_future)
                             (CH.to_spec_poly_plain #vV (Seq.index error (v i))))
  (ensures
    VS.poly_to_spec #vV t_final
      == Seq.index (MX.compute_vector_u v_K (VS.matrix_to_spec v_K #vV a_as_ntt)
                                            (VS.vector_to_spec v_K #vV r)
                                            (VS.vector_to_spec v_K #vV error)) (v i))
= let mmbc_i = Seq.index (MX.multiply_matrix_by_column v_K
                            (MX.transpose v_K (VS.matrix_to_spec v_K #vV a_as_ntt))
                            (VS.vector_to_spec v_K #vV r)) (v i) in
  (* std(t_pre) == part == mmbc_i *)
  reveal_opaque (`%MB.inner_done) (MB.inner_done t_pre a_as_ntt r i (v v_K));
  MB.lemma_part_eq_mmbc a_as_ntt r i;
  (* scaled operand == ntt_inverse (std t_pre) == ntt_inverse mmbc_i *)
  lemma_scaled_operand_eq_ntt_inverse #vV t_pre re_future;
  (* plain(t_final) == add_error_reduce (ntt_inverse mmbc_i) (plain error_i) == add_polynomials ... *)
  lemma_add_error_eq_add_poly (IN.ntt_inverse mmbc_i)
                              (CH.to_spec_poly_plain #vV (Seq.index error (v i)));
  (* unfold compute_vector_u[i] *)
  lemma_compute_vector_u_index #v_K (VS.matrix_to_spec v_K #vV a_as_ntt)
    (VS.vector_to_spec v_K #vV r) (VS.vector_to_spec v_K #vV error) i;
  (* (vts error)[i] == poly_to_spec error_i == plain error_i *)
  VS.vector_to_spec_index v_K #vV error (v i);
  Br.poly_to_spec_eq_to_spec_poly_plain #vV (Seq.index error (v i));
  Br.poly_to_spec_eq_to_spec_poly_plain #vV t_final
#pop-options

(* ════ wrapper producing the opaque row_done atom (mirror Matrix_bridge.lemma_row_done_finalize)
   so the compute_vector_u loop body wiring is identical to compute_As_plus_e's. ════ *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_u_row_done_finalize
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (a_as_ntt: t_Array (t_Array (VV.t_PolynomialRingElement vV) v_K) v_K)
    (r error: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (t_pre re_future t_final: VV.t_PolynomialRingElement vV)
    (i: usize {v i < v v_K})
    (target: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K) : Lemma
  (requires
    MB.inner_done t_pre a_as_ntt r i (v v_K) /\
    CH.to_spec_poly_mont #vV re_future
      == IN.ntt_inverse_butterflies (CH.to_spec_poly_mont #vV t_pre) /\
    Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 7) (Seq.index error (v i)) /\
    t_final == Poly.impl__add_error_reduce #vV re_future (Seq.index error (v i)) /\
    target == MX.compute_vector_u v_K (VS.matrix_to_spec v_K #vV a_as_ntt)
                                      (VS.vector_to_spec v_K #vV r)
                                      (VS.vector_to_spec v_K #vV error))
  (ensures MB.row_done t_final target (v i))
= (* bridge the impl__ wrapper to the free add_error_reduce functional post; createi_lemma is
     available in this module's (unpruned) context, so the bridge's inlined createi operand and
     `scaled_mont re_future` are matched here (NOT in Matrix.fst, where createi_lemma is excluded). *)
  Poly.lemma_impl_add_error_reduce_spec #vV re_future (Seq.index error (v i));
  Seq.lemma_eq_intro (scaled_mont #vV re_future)
    (Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)
       #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
       (fun (j: usize {j <. mk_usize 256}) ->
          Hacspec_ml_kem.Parameters.impl_FieldElement__mul
            (Seq.index (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #vV re_future) (v j))
            Hacspec_ml_kem.Commute.Chunk.fe_1441));
  lemma_u_row_finalize #vV a_as_ntt r error t_pre re_future t_final i;
  reveal_opaque (`%MB.row_done) (MB.row_done t_final target (v i))
#pop-options

module Hacspec_ml_kem.Commute.Compute_dot_bridge
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
module MB  = Hacspec_ml_kem.Commute.Matrix_bridge
module CU  = Hacspec_ml_kem.Commute.Compute_u_bridge
module MZ  = Hacspec_ml_kem.Commute.Matrix_zerolift
module Br  = Hacspec_ml_kem.Commute.Bridges
module F   = Rust_primitives.Hax.Folds
module ML  = FStar.Math.Lemmas

(* ════════════════════════════════════════════════════════════════════
   Vector-dot inner bridge.  compute_message / compute_ring_element_v share a
   SINGLE-accumulator dot product `Σⱼ x[j]·y[j]` (hacspec `multiply_vectors`),
   unlike compute_As_plus_e / compute_vector_u which are per-ROW (matrix×col).
   The accumulator is a single shadowed poly (no `update_at`, no row frame), so
   this is structurally SIMPLER than Matrix_bridge.inner_done.

   We reuse `MB.part` (already generic over arow/scol) with arow = vts x,
   scol = vts y, and the FE-level step `MB.lemma_inner_step`.  Only the spec-side
   fold fusion (multiply_vectors == part) is genuinely new vs the matrix case
   (no transpose). ════════════════════════════════════════════════════════════ *)

(* ════════════════ Part 1: spec-side fold (multiply_vectors) fusion to `part` ════════════════
   Mirror of mmbc_inv/mmbc_step/lemma_mmbc_aux/lemma_mmbc_index but with v1/v2
   indexed DIRECTLY (no transpose, arow=v1 scol=v2 so the per-step term matches
   `part` without a bridge). multiply_vectors is NOT createi-wrapped (it returns
   the fold directly) so lemma_mv_index needs no createi_lemma rewrite. *)

(* named inv/step for multiply_vectors's inner fold — byte-match the inline
   lambdas in Hacspec_ml_kem.Matrix.multiply_vectors. *)
let mv_inv (result: t_Array P.t_FieldElement (mk_usize 256)) (temp_1_: usize) : Type0 =
  let result:t_Array P.t_FieldElement (mk_usize 256) = result in
  let _:usize = temp_1_ in
  true

let mv_step
    (#v_K: usize)
    (v1 v2: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    (result: t_Array P.t_FieldElement (mk_usize 256))
    (j: usize {v j < v v_K})
  : t_Array P.t_FieldElement (mk_usize 256)
  = let result:t_Array P.t_FieldElement (mk_usize 256) = result in
    let j:usize = j in
    let product:t_Array P.t_FieldElement (mk_usize 256) =
      N.multiply_ntts (v1.[ j ] <: t_Array P.t_FieldElement (mk_usize 256))
        (v2.[ j ] <: t_Array P.t_FieldElement (mk_usize 256))
    in
    let result:t_Array P.t_FieldElement (mk_usize 256) =
      MX.add_polynomials result product
    in
    result

(* upward recursion: fold from k onward over (part k) == part v_K *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100 --using_facts_from '* -Hacspec_ml_kem.Ntt -Hacspec_ml_kem.Matrix'"
let rec lemma_mv_aux
    (#v_K: usize)
    (v1 v2: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    (k: nat {k <= v v_K})
  : Lemma
    (ensures
      F.fold_range (sz k) v_K mv_inv (MB.part #v_K v1 v2 k) (mv_step #v_K v1 v2)
      == MB.part #v_K v1 v2 (v v_K))
    (decreases (v v_K - k))
= if k = v v_K then ()
  else begin
    assert (k < v v_K);
    assert ((sz k) +! mk_usize 1 == sz (k + 1));
    (* fold step at k == part (k+1): part unfold (fuel 1) + literal arow=v1/scol=v2 *)
    assert (mv_step #v_K v1 v2 (MB.part #v_K v1 v2 k) (sz k) == MB.part #v_K v1 v2 (k + 1));
    MB.lemma_fold_range_step #(t_Array P.t_FieldElement (mk_usize 256))
      (sz k) v_K mv_inv (MB.part #v_K v1 v2 k) (mv_step #v_K v1 v2);
    lemma_mv_aux #v_K v1 v2 (k + 1)
  end
#pop-options

(* multiply_vectors == my named fold_range (trefl delta-unfold + fold congruence) *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_mv_index
    (#v_K: usize)
    (v1 v2: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
  : Lemma
    (ensures
      MX.multiply_vectors v_K v1 v2
      == F.fold_range (mk_usize 0) v_K mv_inv MB.zero_poly (mv_step #v_K v1 v2))
= let spec_inv : (t_Array P.t_FieldElement (mk_usize 256))
                 -> (j:usize{F.fold_range_wf_index (mk_usize 0) v_K false (v j)}) -> Type0 =
    (fun result temp_1_ ->
        let result:t_Array P.t_FieldElement (mk_usize 256) = result in
        let _:usize = temp_1_ in
        b2t true) in
  let spec_step
      : (acc:(t_Array P.t_FieldElement (mk_usize 256))
         -> j:usize {v j <= v v_K /\ F.fold_range_wf_index (mk_usize 0) v_K true (v j) /\ spec_inv acc j}
         -> acc':(t_Array P.t_FieldElement (mk_usize 256)) {spec_inv acc' (mk_int (v j + 1))}) =
    (fun result j ->
        let result:t_Array P.t_FieldElement (mk_usize 256) = result in
        let j:usize = j in
        let product:t_Array P.t_FieldElement (mk_usize 256) =
          N.multiply_ntts (v1.[ j ] <: t_Array P.t_FieldElement (mk_usize 256))
            (v2.[ j ] <: t_Array P.t_FieldElement (mk_usize 256))
        in
        let result:t_Array P.t_FieldElement (mk_usize 256) =
          MX.add_polynomials result product
        in
        result) in
  (* Step 1: multiply_vectors == spec inline fold, by trefl (no createi wrapper). *)
  assert (MX.multiply_vectors v_K v1 v2
          == F.fold_range (mk_usize 0) v_K spec_inv MB.zero_poly spec_step)
    by (FStar.Tactics.norm [delta_only [`%MX.multiply_vectors];
                            zeta; iota; primops];
        FStar.Tactics.trefl ());
  (* Step 2: spec fold == mv fold, by output congruence (pointwise step eq). *)
  let aux (acc: t_Array P.t_FieldElement (mk_usize 256))
          (jj: usize{v jj >= 0 /\ v jj < v v_K})
    : Lemma (spec_step acc jj == mv_step #v_K v1 v2 acc jj)
    = () in
  Classical.forall_intro_2 aux;
  MB.lemma_fold_range_cong #(t_Array P.t_FieldElement (mk_usize 256))
    (mk_usize 0) v_K spec_inv mv_inv MB.zero_poly spec_step (mv_step #v_K v1 v2)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_part_eq_multiply_vectors
    (#v_K: usize)
    (v1 v2: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
  : Lemma
    (ensures MB.part #v_K v1 v2 (v v_K) == MX.multiply_vectors v_K v1 v2)
= lemma_mv_aux #v_K v1 v2 0;
  assert (MB.part #v_K v1 v2 0 == MB.zero_poly);
  lemma_mv_index #v_K v1 v2;
  assert (F.fold_range (mk_usize 0) v_K mv_inv MB.zero_poly (mv_step #v_K v1 v2)
          == MB.part #v_K v1 v2 (v v_K))
#pop-options

(* ════════════════ Part 2: single-accumulator vdot machinery ════════════════ *)

(* core maintenance: std(acc)==part j  ⟹  std(acc_next)==part (j+1).
   Mirror of MB.lemma_inner_maintain with arow=vts x, scol=vts y directly. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_vdot_maintain
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (x y: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (j: usize {v j < v v_K})
    (acc product acc_next: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      Poly.to_spec_poly_standard #vV acc
        == MB.part #v_K (VS.vector_to_spec v_K #vV x) (VS.vector_to_spec v_K #vV y) (v j) /\
      CH.to_spec_poly_mont #vV product
        == HP.ntt_multiply (CH.to_spec_poly_mont #vV (Seq.index x (v j)))
                           (CH.to_spec_poly_mont #vV (Seq.index y (v j))) /\
      CH.to_spec_poly_plain #vV acc_next
        == HP.add_to_ring_element (CH.to_spec_poly_plain #vV acc) (CH.to_spec_poly_plain #vV product))
    (ensures
      Poly.to_spec_poly_standard #vV acc_next
        == MB.part #v_K (VS.vector_to_spec v_K #vV x) (VS.vector_to_spec v_K #vV y) (v j + 1))
= let arow = VS.vector_to_spec v_K #vV x in
  let scol = VS.vector_to_spec v_K #vV y in
  let a_j = Seq.index x (v j) in
  let s_j = Seq.index y (v j) in
  MB.lemma_inner_step #vV acc product acc_next a_j s_j (MB.part #v_K arow scol (v j));
  VS.vector_to_spec_index v_K #vV x (v j);
  VS.vector_to_spec_index v_K #vV y (v j);
  Br.poly_to_spec_eq_to_spec_poly_plain #vV a_j;
  Br.poly_to_spec_eq_to_spec_poly_plain #vV s_j;
  assert (Seq.index arow (v j) == CH.to_spec_poly_plain #vV a_j);
  assert (Seq.index scol (v j) == CH.to_spec_poly_plain #vV s_j);
  assert (MB.part #v_K arow scol (v j + 1)
          == MX.add_polynomials (MB.part #v_K arow scol (v j))
               (N.multiply_ntts (Seq.index arow (v j)) (Seq.index scol (v j))))
#pop-options

(* opaque atom: acc is the partial dot product (vts x)·(vts y) up to index j *)
[@@ "opaque_to_smt"]
let vdot_done
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (acc: VV.t_PolynomialRingElement vV)
    (x y: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (j: nat {j <= v v_K})
  : Type0
  = Poly.to_spec_poly_standard #vV acc
    == MB.part #v_K (VS.vector_to_spec v_K #vV x) (VS.vector_to_spec v_K #vV y) j

(* base: a zero-bounded acc satisfies vdot_done at j=0 *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_vdot_base
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (x y: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (acc: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 0) acc)
    (ensures vdot_done acc x y 0)
= MZ.lemma_zero_lift #vV acc;
  reveal_opaque (`%vdot_done) (vdot_done acc x y 0)
#pop-options

(* step: vdot_done j + per-step mont/plain posts ⟹ vdot_done (j+1).
   The two impl__ bridge calls are made INSIDE this lemma so the createi-bearing
   functional posts never enter the caller's (compute_message's) pruned context. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_vdot_step_full
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (x y: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (j: usize {v j < v v_K})
    (acc product acc_next: VV.t_PolynomialRingElement vV) (e_b: usize)
  : Lemma
    (requires
      vdot_done acc x y (v j) /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 4096) (Seq.index x (v j)) /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) (Seq.index y (v j)) /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV e_b acc /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) product /\
      (e_b <=. (mk_usize 4 *! mk_usize 3328 <: usize)) /\
      product == Poly.impl__ntt_multiply (Seq.index x (v j)) (Seq.index y (v j)) /\
      acc_next == Poly.impl__add_to_ring_element acc product e_b)
    (ensures vdot_done acc_next x y (v j + 1))
= Poly.lemma_impl_ntt_multiply_spec (Seq.index x (v j)) (Seq.index y (v j));
  Poly.lemma_impl_add_to_ring_element_spec acc product e_b;
  reveal_opaque (`%vdot_done) (vdot_done acc x y (v j));
  lemma_vdot_maintain #vV x y j acc product acc_next;
  reveal_opaque (`%vdot_done) (vdot_done acc_next x y (v j + 1))
#pop-options

(* ════════════════ Part 3: compute_message finalize ════════════════ *)

(* HP.subtract_reduce == MX.sub_polynomials (identical FE-level createi body) *)
#push-options "--z3rlimit 150"
let lemma_sub_reduce_eq_sub_poly (a b: t_Array P.t_FieldElement (mk_usize 256)) : Lemma
  (HP.subtract_reduce a b == MX.sub_polynomials a b)
= let lhs = HP.subtract_reduce a b in
  let rhs = MX.sub_polynomials a b in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    P.createi_lemma #P.t_FieldElement (mk_usize 256) #(usize -> P.t_FieldElement)
      (fun (i: usize{i <. mk_usize 256}) ->
         P.impl_FieldElement__new (cast ((((cast ((a.[ i ] <: P.t_FieldElement).P.f_val <: u16) <: u32) +!
               (cast (P.v_FIELD_MODULUS <: u16) <: u32) <: u32) -!
             (cast ((b.[ i ] <: P.t_FieldElement).P.f_val <: u16) <: u32) <: u32) %!
           (cast (P.v_FIELD_MODULUS <: u16) <: u32) <: u32) <: u16)
         <: P.t_FieldElement)
      (sz j);
    P.createi_lemma #P.t_FieldElement (mk_usize 256) #(usize -> P.t_FieldElement)
      (fun (i: usize{i <. mk_usize 256}) ->
         P.impl_FieldElement__new (cast ((((cast ((a.[ i ] <: P.t_FieldElement).P.f_val <: u16) <: u32) +!
               (cast (P.v_FIELD_MODULUS <: u16) <: u32) <: u32) -!
             (cast ((b.[ i ] <: P.t_FieldElement).P.f_val <: u16) <: u32) <: u32) %!
           (cast (P.v_FIELD_MODULUS <: u16) <: u32) <: u32) <: u16)
         <: P.t_FieldElement)
      (sz j)
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* spec unfold of compute_message *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_compute_message_unfold
    (#v_K: usize)
    (vp: t_Array P.t_FieldElement (mk_usize 256))
    (secret u: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
  : Lemma
    (MX.compute_message v_K vp secret u
     == MX.sub_polynomials vp (IN.ntt_inverse (MX.multiply_vectors v_K secret u)))
= ()
#pop-options

(* MAIN finalize: takes the subtract_reduce FUNCTIONAL post as a hypothesis.
   plain(t_final) == HP.subtract_reduce (plain vp) (scaled_mont re_future). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_message_finalize
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (vp: VV.t_PolynomialRingElement vV)
    (secret u: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (t_pre re_future t_final: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      vdot_done t_pre secret u (v v_K) /\
      CH.to_spec_poly_mont #vV re_future
        == IN.ntt_inverse_butterflies (CH.to_spec_poly_mont #vV t_pre) /\
      CH.to_spec_poly_plain #vV t_final
        == HP.subtract_reduce (CH.to_spec_poly_plain #vV vp) (CU.scaled_mont #vV re_future))
    (ensures
      VS.poly_to_spec #vV t_final
        == MX.compute_message v_K (VS.poly_to_spec #vV vp)
                                  (VS.vector_to_spec v_K #vV secret)
                                  (VS.vector_to_spec v_K #vV u))
= let mv = MX.multiply_vectors v_K (VS.vector_to_spec v_K #vV secret) (VS.vector_to_spec v_K #vV u) in
  (* std(t_pre) == part == multiply_vectors *)
  reveal_opaque (`%vdot_done) (vdot_done t_pre secret u (v v_K));
  lemma_part_eq_multiply_vectors #v_K (VS.vector_to_spec v_K #vV secret) (VS.vector_to_spec v_K #vV u);
  (* scaled operand == ntt_inverse (std t_pre) == ntt_inverse mv *)
  CU.lemma_scaled_operand_eq_ntt_inverse #vV t_pre re_future;
  (* subtract_reduce == sub_polynomials *)
  lemma_sub_reduce_eq_sub_poly (CH.to_spec_poly_plain #vV vp) (IN.ntt_inverse mv);
  (* unfold compute_message *)
  lemma_compute_message_unfold #v_K (VS.poly_to_spec #vV vp)
    (VS.vector_to_spec v_K #vV secret) (VS.vector_to_spec v_K #vV u);
  (* poly_to_spec vp == plain vp ; poly_to_spec t_final == plain t_final *)
  Br.poly_to_spec_eq_to_spec_poly_plain #vV vp;
  Br.poly_to_spec_eq_to_spec_poly_plain #vV t_final
#pop-options

(* ════ impl-form wrapper: takes t_final == impl__subtract_reduce vp re_future, calls the impl__
   bridge INTERNALLY (createi_lemma available in this unpruned context) + bridges the inlined
   createi operand to scaled_mont, so the compute_message wiring is a single call (mirror
   Compute_u_bridge.lemma_u_row_done_finalize). ════ *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_message_done_finalize
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (vp: VV.t_PolynomialRingElement vV)
    (secret u: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (t_pre re_future t_final: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      vdot_done t_pre secret u (v v_K) /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 4095) vp /\
      CH.to_spec_poly_mont #vV re_future
        == IN.ntt_inverse_butterflies (CH.to_spec_poly_mont #vV t_pre) /\
      t_final == Poly.impl__subtract_reduce #vV vp re_future)
    (ensures
      VS.poly_to_spec #vV t_final
        == MX.compute_message v_K (VS.poly_to_spec #vV vp)
                                  (VS.vector_to_spec v_K #vV secret)
                                  (VS.vector_to_spec v_K #vV u))
= Poly.lemma_impl_subtract_reduce_spec #vV vp re_future;
  Seq.lemma_eq_intro (CU.scaled_mont #vV re_future)
    (Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)
       #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
       (fun (j: usize {j <. mk_usize 256}) ->
          Hacspec_ml_kem.Parameters.impl_FieldElement__mul
            (Seq.index (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #vV re_future) (v j))
            Hacspec_ml_kem.Commute.Chunk.fe_1441));
  lemma_message_finalize #vV vp secret u t_pre re_future t_final
#pop-options

(* ════════════════ Part 4: compute_ring_element_v finalize ════════════════ *)

(* per-lane value of HP.add_message_error_reduce: single mod of the 3-sum *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_add_msg_err_lane (e2 m ip: t_Array P.t_FieldElement (mk_usize 256)) (j: nat {j < 256}) : Lemma
  (v (Seq.index (HP.add_message_error_reduce e2 m ip) j).P.f_val
     == ((v (Seq.index e2 j).P.f_val + v (Seq.index m j).P.f_val) + v (Seq.index ip j).P.f_val) % 3329)
= P.createi_lemma #P.t_FieldElement (mk_usize 256) #(usize -> P.t_FieldElement)
    (fun (i: usize {i <. mk_usize 256}) ->
       P.impl_FieldElement__new (cast ((((cast ((e2.[ i ] <: P.t_FieldElement).P.f_val <: u16) <: u32) +!
             (cast ((m.[ i ] <: P.t_FieldElement).P.f_val <: u16) <: u32) <: u32) +!
             (cast ((ip.[ i ] <: P.t_FieldElement).P.f_val <: u16) <: u32) <: u32) %!
           (cast (P.v_FIELD_MODULUS <: u16) <: u32) <: u32) <: u16)
       <: P.t_FieldElement)
    (sz j)
#pop-options

(* 3-operand mod-assoc identity (needs NO bounds; holds for arbitrary FE arrays):
   HP.add_message_error_reduce e2 m ip == add_polynomials (add_polynomials ip e2) m *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 150"
let lemma_add_msg_err_eq_add_poly (e2 m ip: t_Array P.t_FieldElement (mk_usize 256)) : Lemma
  (HP.add_message_error_reduce e2 m ip == MX.add_polynomials (MX.add_polynomials ip e2) m)
= let lhs = HP.add_message_error_reduce e2 m ip in
  let rhs = MX.add_polynomials (MX.add_polynomials ip e2) m in
  let aux (j: nat {j < 256}) : Lemma (Seq.index lhs j == Seq.index rhs j) =
    let a = v (Seq.index e2 j).P.f_val in
    let b = v (Seq.index m j).P.f_val in
    let c = v (Seq.index ip j).P.f_val in
    lemma_add_msg_err_lane e2 m ip j;
    (* v(lhs[j]) == (a+b+c)%3329 *)
    MB.lemma_add_poly_lane (MX.add_polynomials ip e2) m (sz j);
    MB.lemma_add_poly_lane ip e2 (sz j);
    SB.add_val (Seq.index ip j) (Seq.index e2 j);
    SB.add_val (Seq.index (MX.add_polynomials ip e2) j) (Seq.index m j);
    (* v(rhs[j]) == ((c+a)%3329 + b)%3329 *)
    ML.lemma_mod_add_distr b (c + a) 3329;
    (* ((c+a)%q + b)%q == (c+a+b)%q == (a+b+c)%q *)
    SB.fe_eq (Seq.index lhs j) (Seq.index rhs j)
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* spec unfold of compute_ring_element_v *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 150"
let lemma_compute_ring_element_v_unfold
    (#v_K: usize)
    (tt r: t_Array (t_Array P.t_FieldElement (mk_usize 256)) v_K)
    (e2 m: t_Array P.t_FieldElement (mk_usize 256))
  : Lemma
    (MX.compute_ring_element_v v_K tt r e2 m
     == MX.add_polynomials (MX.add_polynomials (IN.ntt_inverse (MX.multiply_vectors v_K tt r)) e2) m)
= ()
#pop-options

(* MAIN finalize: takes the add_message_error_reduce FUNCTIONAL post as a hypothesis.
   plain(t_final) == HP.add_message_error_reduce (plain error_2) (plain message) (scaled_mont re_future). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_v_finalize
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (tt r: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (error_2 message t_pre re_future t_final: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      vdot_done t_pre tt r (v v_K) /\
      CH.to_spec_poly_mont #vV re_future
        == IN.ntt_inverse_butterflies (CH.to_spec_poly_mont #vV t_pre) /\
      CH.to_spec_poly_plain #vV t_final
        == HP.add_message_error_reduce (CH.to_spec_poly_plain #vV error_2)
                                       (CH.to_spec_poly_plain #vV message)
                                       (CU.scaled_mont #vV re_future))
    (ensures
      VS.poly_to_spec #vV t_final
        == MX.compute_ring_element_v v_K (VS.vector_to_spec v_K #vV tt)
                                         (VS.vector_to_spec v_K #vV r)
                                         (VS.poly_to_spec #vV error_2)
                                         (VS.poly_to_spec #vV message))
= let mv = MX.multiply_vectors v_K (VS.vector_to_spec v_K #vV tt) (VS.vector_to_spec v_K #vV r) in
  (* std(t_pre) == part == multiply_vectors *)
  reveal_opaque (`%vdot_done) (vdot_done t_pre tt r (v v_K));
  lemma_part_eq_multiply_vectors #v_K (VS.vector_to_spec v_K #vV tt) (VS.vector_to_spec v_K #vV r);
  (* scaled operand == ntt_inverse (std t_pre) == ntt_inverse mv *)
  CU.lemma_scaled_operand_eq_ntt_inverse #vV t_pre re_future;
  (* 3-op identity *)
  lemma_add_msg_err_eq_add_poly (CH.to_spec_poly_plain #vV error_2)
                                (CH.to_spec_poly_plain #vV message)
                                (IN.ntt_inverse mv);
  (* unfold compute_ring_element_v *)
  lemma_compute_ring_element_v_unfold #v_K (VS.vector_to_spec v_K #vV tt) (VS.vector_to_spec v_K #vV r)
                                            (VS.poly_to_spec #vV error_2) (VS.poly_to_spec #vV message);
  (* poly_to_spec == plain for error_2, message, t_final *)
  Br.poly_to_spec_eq_to_spec_poly_plain #vV error_2;
  Br.poly_to_spec_eq_to_spec_poly_plain #vV message;
  Br.poly_to_spec_eq_to_spec_poly_plain #vV t_final
#pop-options

(* ════ impl-form wrapper for compute_ring_element_v's finalize: takes
   t_final == impl__add_message_error_reduce error_2 message re_future, calls the impl__ bridge
   INTERNALLY (createi_lemma available here) + bridges the inlined createi to scaled_mont
   (mirror lemma_message_done_finalize / Compute_u_bridge.lemma_u_row_done_finalize). ════ *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_v_done_finalize
    (#vV: Type0) {| iop: T.t_Operations vV |} (#v_K: usize)
    (tt r: t_Array (VV.t_PolynomialRingElement vV) v_K)
    (error_2 message t_pre re_future t_final: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      vdot_done t_pre tt r (v v_K) /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) error_2 /\
      Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #vV (mk_usize 3328) message /\
      CH.to_spec_poly_mont #vV re_future
        == IN.ntt_inverse_butterflies (CH.to_spec_poly_mont #vV t_pre) /\
      t_final == Poly.impl__add_message_error_reduce #vV error_2 message re_future)
    (ensures
      VS.poly_to_spec #vV t_final
        == MX.compute_ring_element_v v_K (VS.vector_to_spec v_K #vV tt) (VS.vector_to_spec v_K #vV r)
                                         (VS.poly_to_spec #vV error_2) (VS.poly_to_spec #vV message))
= Poly.lemma_impl_add_message_error_reduce_spec #vV error_2 message re_future;
  Seq.lemma_eq_intro (CU.scaled_mont #vV re_future)
    (Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)
       #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
       (fun (j: usize {j <. mk_usize 256}) ->
          Hacspec_ml_kem.Parameters.impl_FieldElement__mul
            (Seq.index (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #vV re_future) (v j))
            Hacspec_ml_kem.Commute.Chunk.fe_1441));
  lemma_v_finalize #vV tt r error_2 message t_pre re_future t_final
#pop-options

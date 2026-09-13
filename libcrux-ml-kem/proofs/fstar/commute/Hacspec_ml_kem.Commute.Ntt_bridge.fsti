module Hacspec_ml_kem.Commute.Ntt_bridge

(* Abstract-interface firewall for the Ntt_bridge commute module (forward NTT).

   Sole consumer: Libcrux_ml_kem.Ntt (.fst + .fsti; no open, no spec companion).
   Firewalling Ntt_bridge shrinks Ntt's SMT context (the per-vector forward-layer
   chainer + cross-vector machinery do not leak in).

   Surface = 12 abstract vals: 3 opaque predicates (pv_post, poly_step,
   cross_vec_hyp_fwd) + the 9 lemmas Ntt drives (pv_post_intro, lemma_compose_7,
   lemma_layer{1,2,3}_to_poly_step, lemma_layer_4_plus_to_poly_step,
   lemma_cross_vec_from_step_fwd, lemma_cross_vec_frame_fwd, lemma_zeta1_val).
   Zero transparent lets (all three predicates are opaque_to_smt; consumers use
   them as atoms via the intro/frame lemmas), zero exposed SMTPats (Ntt_bridge.fst
   has none).  Val order mirrors the .fst let order.  zetas_{4,2,1}_ /
   mont_i16_to_spec_{fe,array} are external (Vector.Traits.Spec, opened);
   to_spec_poly_plain is transparent via the Chunk.fsti firewall. *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"

open FStar.Mul
open Core_models
open Libcrux_ml_kem.Vector.Traits.Spec
open Hacspec_ml_kem.Commute.Chunk
open Hacspec_ml_kem.Commute.Bridges

module P  = Hacspec_ml_kem.Parameters
module T  = Libcrux_ml_kem.Vector.Traits
module N  = Hacspec_ml_kem.Ntt
module VV = Libcrux_ml_kem.Vector

/// Sealed per-vector forward-NTT-layer post at index m (opaque to consumers).
val pv_post (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16))
    (len: usize {v len == 2 \/ v len == 4 \/ v len == 8})
    (pvm: t_Array P.t_FieldElement (mk_usize (8 / v len))) (m: nat) : prop

val pv_post_intro (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16))
    (len: usize {v len == 2 \/ v len == 4 \/ v len == 8})
    (pvm: t_Array P.t_FieldElement (mk_usize (8 / v len))) (m: nat)
  : Lemma
    (requires m < 16 ==>
      mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index cout m)) ==
        N.ntt_layer_n (mk_usize 16)
          (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index cin m)))
          len (Rust_primitives.unsize pvm))
    (ensures pv_post #vV cin cout len pvm m)

/// Sealed poly-level forward-NTT-layer step (opaque to consumers).
val poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
    (layer: usize {v layer >= 1 /\ v layer <= 7}) : prop

val lemma_compose_7 (#vV: Type0) {| iop: T.t_Operations vV |}
    (re0 re1 re2 re3 re4 re5 re6 re7: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      poly_step #vV re0 re1 (mk_usize 7) /\ poly_step #vV re1 re2 (mk_usize 6) /\
      poly_step #vV re2 re3 (mk_usize 5) /\ poly_step #vV re3 re4 (mk_usize 4) /\
      poly_step #vV re4 re5 (mk_usize 3) /\ poly_step #vV re5 re6 (mk_usize 2) /\
      poly_step #vV re6 re7 (mk_usize 1))
    (ensures
      to_spec_poly_plain #vV re7 == N.ntt (to_spec_poly_plain #vV re0))

val lemma_layer3_to_poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      (forall (i: usize). v i < 16 ==>
        pv_post #vV re_in.VV.f_coefficients re_out.VV.f_coefficients (mk_usize 8)
          (zetas_1_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! i)))
          (v i)))
    (ensures poly_step #vV re_in re_out (mk_usize 3))

val lemma_layer2_to_poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      (forall (i: usize). v i < 16 ==>
        pv_post #vV re_in.VV.f_coefficients re_out.VV.f_coefficients (mk_usize 4)
          (zetas_2_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! i)))
          (v i)))
    (ensures poly_step #vV re_in re_out (mk_usize 2))

val lemma_layer1_to_poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      (forall (i: usize). v i < 16 ==>
        pv_post #vV re_in.VV.f_coefficients re_out.VV.f_coefficients (mk_usize 2)
          (zetas_4_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! i)))
          (v i)))
    (ensures poly_step #vV re_in re_out (mk_usize 1))

(* Cross-vector forward-layer hypothesis (MONT).  TRANSPARENT (opaque_to_smt let,
   not an abstract val): the Ntt consumer's lemma_postloop_cross_vec_fwd calls
   `reveal_opaque cross_vec_hyp_fwd` directly (the out-of-range m>=16/l>=16 case),
   which needs the body through the interface — mirrors Bridges.fsti's transparent
   cross_vec_hyp.  Stays opaque_to_smt so it's an atom unless revealed. *)
[@@ "opaque_to_smt"]
let cross_vec_hyp_fwd
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
          (N.butterfly (Seq.index zs block)
             (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin m)) l))
             (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin (m + step_vec))) l)))._1) /\
     (pos >= step_vec ==>
        m >= step_vec /\
        mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cout m)) l) ==
          (N.butterfly (Seq.index zs block)
             (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin (m - step_vec))) l))
             (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin m)) l)))._2))

val lemma_layer_4_plus_to_poly_step
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
      Seq.length zs == 128 / v len /\
      ((Seq.length zs) * 2) * v len == 256 /\
      (let groups = 128 / v len in
       forall (round: nat). round < groups ==>
         Seq.index zs round == N.v_ZETAS.[ sz (groups + round) ]) /\
      (forall (m: nat) (l: nat).
         cross_vec_hyp_fwd #vV re_in.VV.f_coefficients re_out.VV.f_coefficients step_vec zs m l))
    (ensures poly_step #vV re_in re_out layer)

val lemma_cross_vec_from_step_fwd
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
           (N.butterfly (mont_i16_to_spec_fe zeta_r)
              (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin j)) l))
              (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin (j + step_vec))) l)))._1) /\
      (forall (l: nat). l < 16 ==>
         mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cout (j + step_vec))) l) ==
           (N.butterfly (mont_i16_to_spec_fe zeta_r)
              (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin j)) l))
              (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin (j + step_vec))) l)))._2))
    (ensures
      (forall (l: nat). l < 16 ==> cross_vec_hyp_fwd #vV cin cout step_vec zs j l) /\
      (forall (l: nat). l < 16 ==> cross_vec_hyp_fwd #vV cin cout step_vec zs (j + step_vec) l))

val lemma_cross_vec_frame_fwd
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout1 cout2: t_Array vV (mk_usize 16))
    (step_vec: pos)
    (zs: t_Slice P.t_FieldElement)
    (m l: nat)
  : Lemma
    (requires m < 16 /\ Seq.index cout1 m == Seq.index cout2 m)
    (ensures cross_vec_hyp_fwd #vV cin cout1 step_vec zs m l <==>
             cross_vec_hyp_fwd #vV cin cout2 step_vec zs m l)

val lemma_zeta1_val (_:unit)
  : Lemma ((v (Libcrux_ml_kem.Polynomial.zeta (mk_usize 1)) * 169) % 3329 == 1729)

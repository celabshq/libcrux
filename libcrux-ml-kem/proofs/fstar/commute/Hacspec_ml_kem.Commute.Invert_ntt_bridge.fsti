module Hacspec_ml_kem.Commute.Invert_ntt_bridge

(* Abstract-interface firewall for the Invert_ntt_bridge commute module.

   Sole consumer: Libcrux_ml_kem.Invert_ntt.  Surface = the 7 symbols it cites
   (pv_post, pv_post_intro, poly_step, lemma_poly_step_intro,
   lemma_layer{1,2,3}_to_poly_step, lemma_compose_7) plus the two opaque
   predicates named in their ensures (pv_post, poly_step).  All the layer 1-3
   per-vector chainer machinery (and the stale "layer 1 SMT ADMITTED" episode —
   long since closed, 0 admits) stays module-private.

   All vals ABSTRACT, 0 transparent lets (the consumer only introduces/consumes
   the opaque atoms via the intro lemmas), 0 SMTPats.  Val order mirrors the .fst
   let order (Error 233).  zetas_{4,2,1}_/mont_i16_to_spec_array are external
   (Vector.Traits.Spec, opened); to_spec_poly_mont is transparent via Chunk.fsti. *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"

open FStar.Mul
open Core_models
open Libcrux_ml_kem.Vector.Traits.Spec
open Hacspec_ml_kem.Commute.Chunk
open Hacspec_ml_kem.Commute.Bridges

module P  = Hacspec_ml_kem.Parameters
module T  = Libcrux_ml_kem.Vector.Traits
module TS = Libcrux_ml_kem.Vector.Traits.Spec
module IN = Hacspec_ml_kem.Invert_ntt
module VV = Libcrux_ml_kem.Vector

/// Sealed per-vector inverse-NTT-layer post at index m (opaque to consumers).
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
        IN.ntt_inverse_layer_n (mk_usize 16)
          (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index cin m)))
          len (Rust_primitives.unsize pvm))
    (ensures pv_post #vV cin cout len pvm m)

/// Sealed poly-level inverse-NTT-layer step (opaque to consumers).
val poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
    (layer: usize {v layer >= 1 /\ v layer <= 7}) : prop

val lemma_poly_step_intro (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
    (layer: usize {v layer >= 1 /\ v layer <= 7})
  : Lemma
    (requires
      to_spec_poly_mont #vV re_out == IN.ntt_inverse_layer (to_spec_poly_mont #vV re_in) layer)
    (ensures poly_step #vV re_in re_out layer)

val lemma_layer1_to_poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      (forall (i: usize). v i < 16 ==>
        pv_post #vV re_in.VV.f_coefficients re_out.VV.f_coefficients (mk_usize 2)
          (zetas_4_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 127 -! mk_usize 4 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 126 -! mk_usize 4 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 125 -! mk_usize 4 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 124 -! mk_usize 4 *! i)))
          (v i)))
    (ensures poly_step #vV re_in re_out (mk_usize 1))

val lemma_layer2_to_poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      (forall (i: usize). v i < 16 ==>
        pv_post #vV re_in.VV.f_coefficients re_out.VV.f_coefficients (mk_usize 4)
          (zetas_2_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 63 -! mk_usize 2 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 62 -! mk_usize 2 *! i)))
          (v i)))
    (ensures poly_step #vV re_in re_out (mk_usize 2))

val lemma_layer3_to_poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      (forall (i: usize). v i < 16 ==>
        pv_post #vV re_in.VV.f_coefficients re_out.VV.f_coefficients (mk_usize 8)
          (zetas_1_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 31 -! i)))
          (v i)))
    (ensures poly_step #vV re_in re_out (mk_usize 3))

val lemma_compose_7 (#vV: Type0) {| iop: T.t_Operations vV |}
    (re0 re1 re2 re3 re4 re5 re6 re7: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      poly_step #vV re0 re1 (mk_usize 1) /\ poly_step #vV re1 re2 (mk_usize 2) /\
      poly_step #vV re2 re3 (mk_usize 3) /\ poly_step #vV re3 re4 (mk_usize 4) /\
      poly_step #vV re4 re5 (mk_usize 5) /\ poly_step #vV re5 re6 (mk_usize 6) /\
      poly_step #vV re6 re7 (mk_usize 7))
    (ensures
      to_spec_poly_mont #vV re7 == IN.ntt_inverse_butterflies (to_spec_poly_mont #vV re0))

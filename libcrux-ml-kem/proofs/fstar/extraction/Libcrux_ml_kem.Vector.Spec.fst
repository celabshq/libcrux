module Libcrux_ml_kem.Vector.Spec
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_ml_kem.Vector.Traits in
  ()

let poly_to_spec
      (#v_V: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_V)
      (p: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_V)
     =
  let (flat: t_Array i16 (mk_usize 256)):t_Array i16 (mk_usize 256) =
    Hacspec_ml_kem.Parameters.createi #i16
      (mk_usize 256)
      #(usize -> i16)
      (fun i ->
          let i:usize = i in
          let chunk:t_Array i16 (mk_usize 16) =
            Libcrux_ml_kem.Vector.Traits.f_to_i16_array #v_V
              #FStar.Tactics.Typeclasses.solve
              (p.Libcrux_ml_kem.Vector.f_coefficients.[ i /! mk_usize 16 <: usize ] <: v_V)
          in
          chunk.[ i %! mk_usize 16 <: usize ])
  in
  Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_array (mk_usize 256) flat

let vector_to_spec
      (v_RANK: usize)
      (#v_V: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_V)
      (v: t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_V) v_RANK)
     =
  Hacspec_ml_kem.Parameters.createi #(t_Array Hacspec_ml_kem.Parameters.t_FieldElement
        (mk_usize 256))
    v_RANK
    #(usize -> t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    (fun i ->
        let i:usize = i in
        poly_to_spec #v_V (v.[ i ] <: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_V)
        <:
        t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))

let matrix_to_spec
      (v_RANK: usize)
      (#v_V: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_V)
      (m: t_Array (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_V) v_RANK) v_RANK)
     =
  Hacspec_ml_kem.Parameters.createi #(t_Array
        (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    v_RANK
    #(usize -> t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    (fun i ->
        let i:usize = i in
        vector_to_spec v_RANK
          #v_V
          (m.[ i ] <: t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_V) v_RANK)
        <:
        t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)

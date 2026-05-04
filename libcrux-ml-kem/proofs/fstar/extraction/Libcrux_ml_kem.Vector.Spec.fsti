module Libcrux_ml_kem.Vector.Spec
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_ml_kem.Vector.Traits in
  ()

/// Lift one impl `PolynomialRingElement<V>` (16 chunks × 16 lanes)
/// to the spec `[FieldElement; 256]` polynomial.  Each `i16`
/// coefficient is reduced via `i16_to_spec_fe` (Euclidean mod q).
val poly_to_spec
      (#v_V: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_V |}
      (p: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_V)
    : Prims.Pure (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      Prims.l_True
      (fun _ -> Prims.l_True)

/// Lift a rank-K array of impl polynomials to the spec
/// `[Polynomial; K]` vector.  Used by libcrux-side ensures that
/// state per-vector functional correctness against the Hacspec
/// reference (e.g. `serialize_public_key`).
val vector_to_spec
      (v_RANK: usize)
      (#v_V: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_V |}
      (v: t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_V) v_RANK)
    : Prims.Pure (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
      Prims.l_True
      (fun _ -> Prims.l_True)

/// Lift a K×K matrix of impl polynomials to the spec
/// `[[Polynomial; K]; K]` matrix.
val matrix_to_spec
      (v_RANK: usize)
      (#v_V: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_V |}
      (m: t_Array (t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_V) v_RANK) v_RANK)
    : Prims.Pure
      (t_Array (t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
          v_RANK) Prims.l_True (fun _ -> Prims.l_True)

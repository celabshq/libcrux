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

/// Per-lane index fact for `poly_to_spec`: the j-th coefficient equals
/// `i16_to_spec_fe` applied to the (j%16)-th lane of the (j/16)-th chunk.
/// Needed to bridge `poly_to_spec` (opaque outside this module) to
/// `to_spec_poly_plain` in `Hacspec_ml_kem.Commute.Chunk`.
val poly_to_spec_index
      (#v_V: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_V |}
      (p: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_V)
      (j: nat)
    : Lemma
      (requires j < 256)
      (ensures
        Seq.index (poly_to_spec #v_V p) j ==
        Libcrux_ml_kem.Vector.Traits.Spec.i16_to_spec_fe
          (Seq.index
            (Libcrux_ml_kem.Vector.Traits.f_repr #v_V
              (Seq.index p.Libcrux_ml_kem.Vector.f_coefficients (j / 16)))
            (j % 16)))

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

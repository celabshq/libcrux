module Libcrux_ml_kem.Polynomial
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_ml_kem.Vector.Traits in
  ()

let v_ZETAS_TIMES_MONTGOMERY_R: t_Array i16 (mk_usize 128) =
  let _:Prims.unit = assert_norm (pow2 16 == 65536) in
  let list =
    [
      mk_i16 (-1044); mk_i16 (-758); mk_i16 (-359); mk_i16 (-1517); mk_i16 1493; mk_i16 1422;
      mk_i16 287; mk_i16 202; mk_i16 (-171); mk_i16 622; mk_i16 1577; mk_i16 182; mk_i16 962;
      mk_i16 (-1202); mk_i16 (-1474); mk_i16 1468; mk_i16 573; mk_i16 (-1325); mk_i16 264;
      mk_i16 383; mk_i16 (-829); mk_i16 1458; mk_i16 (-1602); mk_i16 (-130); mk_i16 (-681);
      mk_i16 1017; mk_i16 732; mk_i16 608; mk_i16 (-1542); mk_i16 411; mk_i16 (-205); mk_i16 (-1571);
      mk_i16 1223; mk_i16 652; mk_i16 (-552); mk_i16 1015; mk_i16 (-1293); mk_i16 1491;
      mk_i16 (-282); mk_i16 (-1544); mk_i16 516; mk_i16 (-8); mk_i16 (-320); mk_i16 (-666);
      mk_i16 (-1618); mk_i16 (-1162); mk_i16 126; mk_i16 1469; mk_i16 (-853); mk_i16 (-90);
      mk_i16 (-271); mk_i16 830; mk_i16 107; mk_i16 (-1421); mk_i16 (-247); mk_i16 (-951);
      mk_i16 (-398); mk_i16 961; mk_i16 (-1508); mk_i16 (-725); mk_i16 448; mk_i16 (-1065);
      mk_i16 677; mk_i16 (-1275); mk_i16 (-1103); mk_i16 430; mk_i16 555; mk_i16 843; mk_i16 (-1251);
      mk_i16 871; mk_i16 1550; mk_i16 105; mk_i16 422; mk_i16 587; mk_i16 177; mk_i16 (-235);
      mk_i16 (-291); mk_i16 (-460); mk_i16 1574; mk_i16 1653; mk_i16 (-246); mk_i16 778; mk_i16 1159;
      mk_i16 (-147); mk_i16 (-777); mk_i16 1483; mk_i16 (-602); mk_i16 1119; mk_i16 (-1590);
      mk_i16 644; mk_i16 (-872); mk_i16 349; mk_i16 418; mk_i16 329; mk_i16 (-156); mk_i16 (-75);
      mk_i16 817; mk_i16 1097; mk_i16 603; mk_i16 610; mk_i16 1322; mk_i16 (-1285); mk_i16 (-1465);
      mk_i16 384; mk_i16 (-1215); mk_i16 (-136); mk_i16 1218; mk_i16 (-1335); mk_i16 (-874);
      mk_i16 220; mk_i16 (-1187); mk_i16 (-1659); mk_i16 (-1185); mk_i16 (-1530); mk_i16 (-1278);
      mk_i16 794; mk_i16 (-1510); mk_i16 (-854); mk_i16 (-870); mk_i16 478; mk_i16 (-108);
      mk_i16 (-308); mk_i16 996; mk_i16 991; mk_i16 958; mk_i16 (-1460); mk_i16 1522; mk_i16 1628
    ]
  in
  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 128);
  Rust_primitives.Hax.array_of_list 128 list

val zeta (i: usize)
    : Prims.Pure i16
      (requires i <. mk_usize 128)
      (ensures
        fun result ->
          let result:i16 = result in
          result >=. mk_i16 (-1664) && result <=. mk_i16 1664)

val add_bounded
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (vec1: v_Vector)
      (e_b1: usize)
      (vec2: v_Vector)
      (e_b2: usize)
    : Prims.Pure v_Vector
      (requires
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector e_b1 vec1 /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector e_b2 vec2 /\
        b2t
        ((e_b1 <. mk_usize 32768 <: bool) && (e_b2 <. mk_usize 32768 <: bool) &&
          ((e_b1 +! e_b2 <: usize) <. mk_usize 32768 <: bool)))
      (ensures
        fun result ->
          let result:v_Vector = result in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector (e_b1 +! e_b2 <: usize) result /\
          Libcrux_ml_kem.Vector.Traits.Spec.add_post (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                #FStar.Tactics.Typeclasses.solve
                vec1
              <:
              t_Array i16 (mk_usize 16))
            (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector #FStar.Tactics.Typeclasses.solve vec2
              <:
              t_Array i16 (mk_usize 16))
            (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector #FStar.Tactics.Typeclasses.solve result
              <:
              t_Array i16 (mk_usize 16)))

val sub_bounded
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (vec1: v_Vector)
      (e_b1: usize)
      (vec2: v_Vector)
      (e_b2: usize)
    : Prims.Pure v_Vector
      (requires
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector e_b1 vec1 /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector e_b2 vec2 /\
        b2t
        ((e_b1 <. mk_usize 32768 <: bool) && (e_b2 <. mk_usize 32768 <: bool) &&
          ((e_b1 +! e_b2 <: usize) <. mk_usize 32768 <: bool)))
      (ensures
        fun result ->
          let result:v_Vector = result in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector (e_b1 +! e_b2 <: usize) result /\
          Libcrux_ml_kem.Vector.Traits.Spec.sub_post (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                #FStar.Tactics.Typeclasses.solve
                vec1
              <:
              t_Array i16 (mk_usize 16))
            (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector #FStar.Tactics.Typeclasses.solve vec2
              <:
              t_Array i16 (mk_usize 16))
            (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector #FStar.Tactics.Typeclasses.solve result
              <:
              t_Array i16 (mk_usize 16)))

val multiply_by_constant_bounded
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (vec: v_Vector)
      (e_b: usize)
      (c: i16)
    : Prims.Pure v_Vector
      (requires
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector e_b vec /\
        b2t
        ((c >. mk_i16 (-32768) <: bool) &&
          (((Rust_primitives.Hax.Int.from_machine e_b <: Hax_lib.Int.t_Int) *
              (Rust_primitives.Hax.Int.from_machine (Core_models.Num.impl_i16__abs c <: i16)
                <:
                Hax_lib.Int.t_Int)
              <:
              Hax_lib.Int.t_Int) <
            (Rust_primitives.Hax.Int.from_machine (mk_i32 32768) <: Hax_lib.Int.t_Int)
            <:
            bool)))
      (ensures
        fun result ->
          let result:v_Vector = result in
          let abs_c = Core_models.Num.impl_i16__abs c in
          b2t (abs_c >=. mk_i16 0 && abs_c <=. mk_i16 32767) /\
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
            (mk_usize (v e_b * v abs_c))
            result)

val v_ZERO:
    #v_Vector: Type0 ->
    {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |} ->
    Prims.unit
  -> Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      Prims.l_True
      (ensures
        fun result ->
          let result:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = result in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 0) result)

val from_i16_array
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (a: t_Slice i16)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires
        (Libcrux_ml_kem.Vector.v_VECTORS_IN_RING_ELEMENT *! mk_usize 16 <: usize) <=.
        (Core_models.Slice.impl__len #i16 a <: usize))
      (fun _ -> Prims.l_True)

val to_i16_array
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (out: t_Slice i16)
    : Prims.Pure (t_Slice i16)
      (requires
        (Core_models.Slice.impl__len #i16 out <: usize) >=.
        (Libcrux_ml_kem.Vector.v_VECTORS_IN_RING_ELEMENT *! mk_usize 16 <: usize))
      (fun _ -> Prims.l_True)

val from_bytes
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (bytes: t_Slice u8)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires
        ((Libcrux_ml_kem.Vector.v_VECTORS_IN_RING_ELEMENT *! mk_usize 16 <: usize) *! mk_usize 2
          <:
          usize) <=.
        (Core_models.Slice.impl__len #u8 bytes <: usize))
      (fun _ -> Prims.l_True)

val to_bytes
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (re: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (out: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Libcrux_ml_kem.Vector.v_VECTORS_IN_RING_ELEMENT *! mk_usize 32 <: usize) <=.
        (Core_models.Slice.impl__len #u8 out <: usize))
      (ensures
        fun out_future ->
          let out_future:t_Slice u8 = out_future in
          (Core_models.Slice.impl__len #u8 out_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out <: usize))

/// Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
/// sum of their constituent coefficients.
val add_to_ring_element
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (myself rhs: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (e_bound: usize)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires
        b2t (e_bound <=. (mk_usize 4 *! mk_usize 3328 <: usize) <: bool) /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector e_bound myself /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) rhs)
      (ensures
        fun myself_future ->
          let myself_future:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            myself_future
          in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector
            (e_bound +! mk_usize 3328)
            myself_future /\
          Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector myself_future ==
          Hacspec_ml_kem.Polynomial.add_to_ring_element (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain
                #v_Vector
                myself)
            (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector rhs))

val poly_barrett_reduce
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (myself: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 28296) myself)
      (ensures
        fun myself_future ->
          let myself_future:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            myself_future
          in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) myself_future /\
          Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector myself_future ==
          Hacspec_ml_kem.Polynomial.poly_barrett_reduce (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain
                #v_Vector
                myself))

/// Compute `myself - InvNTT(b)` lane-wise, with the `· 128⁻¹` finalize of
/// the inverse NTT fused into the per-lane `mont_mul(_, 1441)` here (saving
/// ~80 SIMD ops per call vs running invert_ntt_montgomery\'s finalize +
/// a separate scale).
/// Scaling on entry:
/// - `myself` is **plain** (`v c ≡ α mod q`) — caller deserialized v from
///   the ciphertext via `decompress_then_deserialize_ring_element_v`,
///   which produces plain coefficients.
/// - `b` is in libcrux\'s **`·R⁻¹` form** (`v c ≡ β · R⁻¹ mod q`) — output
///   of `invert_ntt_montgomery`, which preserves the `·R⁻¹` form set by
///   `ntt_multiply` (each `mont_mul` carries one `·R⁻¹` factor through).
/// The fused `mont_mul(b, 1441)` step:
///   `mont_mul(b, 1441) = (β · R⁻¹) · 1441 · R⁻¹ = β · R²/128 · R⁻²
///                       = β · 1/128 (mod q)`
/// where `1441 = R²/128 mod q` per `pq-crystals/kyber/main/ref/ntt.c:106`.
/// This simultaneously discharges the missing `· 128⁻¹` from
/// `invert_ntt_montgomery`\'s 7-layer GS chain AND brings the lane back
/// to plain form (`v r ≡ β · 128⁻¹ mod q`), so `myself - mont_mul(b, 1441)`
/// is plain-form arithmetic.  Result is plain post-Barrett.
/// See `src/invert_ntt.rs` (above `invert_ntt_montgomery`) for the
/// upstream chain doc.
val subtract_reduce
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (myself b: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 4095) myself)
      (ensures
        fun result ->
          let result:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = result in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) result /\
          Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #v_Vector result ==
          Hacspec_ml_kem.Polynomial.subtract_reduce (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain
                #v_Vector
                myself)
            (Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
                (mk_usize 256)
                #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
                (fun (j: usize{j <. mk_usize 256}) ->
                    Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Seq.index (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont
                              #v_Vector
                              b)
                          (v j))
                      Hacspec_ml_kem.Commute.Chunk.fe_1441)))

/// Compute `myself + message + InvNTT(result)` lane-wise — same fused
/// `mont_mul(result, 1441)` pattern as `subtract_reduce`.
/// Scaling on entry:
/// - `myself` (= `error_2` at the call site `compute_ring_element_v` in
///   matrix.rs): plain (`v ≡ α mod q`).
/// - `message`: plain (output of `deserialize_then_decompress_message`).
/// - `result`: `·R⁻¹` form (output of `invert_ntt_montgomery`).
/// `mont_mul(result, 1441)` brings `result` to plain (per the fused-finalize
/// algebra documented above `subtract_reduce`), then plain + plain + plain
/// → plain, Barrett-reduced into `[0, q)`.
val add_message_error_reduce
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (myself message result: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) myself /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) message)
      (ensures
        fun output ->
          let output:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = output in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) output)

/// Compute `InvNTT(myself) + error` lane-wise — fused `mont_mul(myself, 1441)`
/// pattern as `subtract_reduce`.
/// Scaling on entry:
/// - `myself`: `·R⁻¹` form (output of `invert_ntt_montgomery` at the call
///   site `compute_vector_u` in matrix.rs).
/// - `error` (= `error_1[i]`): plain, small (`is_bounded_poly(7)` — direct
///   from CBD sampling).
/// `mont_mul(myself, 1441)` brings `myself` to plain (fused finalize), then
/// `plain + plain → plain`, Barrett-reduced.
val add_error_reduce
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (myself error: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 7) error)
      (ensures
        fun myself_future ->
          let myself_future:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            myself_future
          in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) myself_future)

val to_standard_domain
      (#v_T: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_T |}
      (vector: v_T)
    : Prims.Pure v_T
      Prims.l_True
      (ensures
        fun result ->
          let result:v_T = result in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_T (mk_usize 3328) result /\
          Spec.Utils.forall16 (fun (i: nat{i < 16}) ->
                Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_lane_post (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr
                          #v_T
                          vector)
                      i)
                  Libcrux_ml_kem.Vector.Traits.v_MONTGOMERY_R_SQUARED_MOD_FIELD_MODULUS
                  (Seq.index (Libcrux_ml_kem.Vector.Traits.f_repr #v_T result) i)))

/// Compute `to_standard_domain(myself) + error` lane-wise — different
/// fused-finalize pattern from the three INTT-track reduce fns above.
/// Scaling on entry:
/// - `myself` (= `t_as_ntt[i]` at the call site `compute_As_plus_e` in
///   matrix.rs): `·R⁻¹` form (output of accumulated `ntt_multiply` chain).
///   This is the **post-matrix-multiply** path — there is NO inverse NTT
///   upstream here; `myself` is in the NTT domain still.
/// - `error` (= `error_as_ntt[i]`): plain (sampled CBD then NTT\'d; NTT
///   preserves plain input scaling — see `src/ntt.rs` cross-spec test
///   `ntt_matches_spec`).
/// `to_standard_domain(myself) = mont_mul(myself, R²) = mont_mul(myself, 1353)`
/// applies a single `· R²` factor: `(α · R⁻¹) · 1353 · R⁻¹ = α · R² · R⁻²
/// = α (mod q)`, bringing `myself` to plain form.  Result is plain
/// post-Barrett.  Note `1353 = R² mod q` ≠ `1441 = R²/128 mod q` — the
/// distinction is the missing `· 128⁻¹` factor that ONLY applies to the
/// INTT track (where `invert_ntt_montgomery` skips its FIPS-203 finalize).
val add_standard_error_reduce
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (myself error: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) error)
      (ensures
        fun myself_future ->
          let myself_future:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            myself_future
          in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) myself_future)

/// Given two `KyberPolynomialRingElement`s in their NTT representations,
/// compute their product. Given two polynomials in the NTT domain `f^` and `ĵ`,
/// the `iᵗʰ` coefficient of the product `k\u{302}` is determined by the calculation:
/// ```plaintext
/// ĥ[2·i] + ĥ[2·i + 1]X = (f^[2·i] + f^[2·i + 1]X)·(ĝ[2·i] + ĝ[2·i + 1]X) mod (X² - ζ^(2·BitRev₇(i) + 1))
/// ```
/// This function almost implements <strong>Algorithm 10</strong> of the
/// NIST FIPS 203 standard, which is reproduced below:
/// ```plaintext
/// Input: Two arrays fˆ ∈ ℤ₂₅₆ and ĝ ∈ ℤ₂₅₆.
/// Output: An array ĥ ∈ ℤq.
/// for(i ← 0; i < 128; i++)
///     (ĥ[2i], ĥ[2i+1]) ← BaseCaseMultiply(fˆ[2i], fˆ[2i+1], ĝ[2i], ĝ[2i+1], ζ^(2·BitRev₇(i) + 1))
/// end for
/// return ĥ
/// ```
/// We say \"almost\" because the coefficients of the ring element output by
/// this function are in the Montgomery domain.
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
val ntt_multiply
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (myself rhs: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) myself /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) rhs)
      (ensures
        fun result ->
          let result:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = result in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) result)

val impl__ZERO:
    #v_Vector: Type0 ->
    {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |} ->
    Prims.unit
  -> Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      Prims.l_True
      (ensures
        fun result ->
          let result:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = result in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 0) result)

/// Size of a ring element in bytes.
val impl__num_bytes:
    #v_Vector: Type0 ->
    {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |} ->
    Prims.unit
  -> Prims.Pure usize
      Prims.l_True
      (ensures
        fun result ->
          let result:usize = result in
          result =. mk_usize 512)

/// The length of a vector of ring elements in bytes
val vec_len_bytes:
    v_K: usize ->
    #v_Vector: Type0 ->
    {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |} ->
    Prims.unit
  -> Prims.Pure usize
      (requires v_K <=. mk_usize 4)
      (ensures
        fun result ->
          let result:usize = result in
          result =. (v_K *! mk_usize 512 <: usize))

val impl__from_i16_array
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (a: t_Slice i16)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires
        (Libcrux_ml_kem.Vector.v_VECTORS_IN_RING_ELEMENT *! mk_usize 16 <: usize) <=.
        (Core_models.Slice.impl__len #i16 a <: usize))
      (fun _ -> Prims.l_True)

val impl__to_i16_array
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (self: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (out: t_Slice i16)
    : Prims.Pure (t_Slice i16)
      (requires
        (Libcrux_ml_kem.Vector.v_VECTORS_IN_RING_ELEMENT *! mk_usize 16 <: usize) <=.
        (Core_models.Slice.impl__len #i16 out <: usize))
      (fun _ -> Prims.l_True)

val impl__from_bytes
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (bytes: t_Slice u8)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires
        ((Libcrux_ml_kem.Vector.v_VECTORS_IN_RING_ELEMENT *! mk_usize 16 <: usize) *! mk_usize 2
          <:
          usize) <=.
        (Core_models.Slice.impl__len #u8 bytes <: usize))
      (fun _ -> Prims.l_True)

/// Build a vector of ring elements from `bytes`.
val vec_from_bytes
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (bytes: t_Slice u8)
      (out: t_Slice (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector))
    : Prims.Pure (t_Slice (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector))
      (requires
        (Core_models.Slice.impl__len #(Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) out
          <:
          usize) <=.
        mk_usize 4 &&
        (mk_usize 512 *!
          (Core_models.Slice.impl__len #(Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) out
            <:
            usize)
          <:
          usize) <=.
        (Core_models.Slice.impl__len #u8 bytes <: usize))
      (ensures
        fun out_future ->
          let out_future:t_Slice (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) =
            out_future
          in
          (Core_models.Slice.impl__len #(Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
              out_future
            <:
            usize) =.
          (Core_models.Slice.impl__len #(Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) out
            <:
            usize))

val impl__to_bytes
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (self: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (out: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        ((Libcrux_ml_kem.Vector.v_VECTORS_IN_RING_ELEMENT *! mk_usize 16 <: usize) *! mk_usize 2
          <:
          usize) <=.
        (Core_models.Slice.impl__len #u8 out <: usize))
      (ensures
        fun out_future ->
          let out_future:t_Slice u8 = out_future in
          (Core_models.Slice.impl__len #u8 out_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out <: usize))

/// Get the bytes of the vector of ring elements in `re` and write them to `out`.
val vec_to_bytes
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (re: t_Slice (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector))
      (out: t_Slice u8)
    : Prims.Pure (t_Slice u8)
      (requires
        (Core_models.Slice.impl__len #(Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) re
          <:
          usize) <=.
        mk_usize 4 &&
        (mk_usize 512 *!
          (Core_models.Slice.impl__len #(Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector) re
            <:
            usize)
          <:
          usize) <=.
        (Core_models.Slice.impl__len #u8 out <: usize))
      (ensures
        fun out_future ->
          let out_future:t_Slice u8 = out_future in
          (Core_models.Slice.impl__len #u8 out_future <: usize) =.
          (Core_models.Slice.impl__len #u8 out <: usize))

/// Given two polynomial ring elements `lhs` and `rhs`, compute the pointwise
/// sum of their constituent coefficients.
val impl__add_to_ring_element
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (self rhs: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (e_b: usize)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires
        b2t (e_b <=. (mk_usize 4 *! mk_usize 3328 <: usize) <: bool) /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector e_b self /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) rhs)
      (ensures
        fun self_e_future ->
          let self_e_future:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            self_e_future
          in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector
            (e_b +! mk_usize 3328 <: usize)
            self_e_future)

val impl__poly_barrett_reduce
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (self: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 28296) self)
      (ensures
        fun self_e_future ->
          let self_e_future:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            self_e_future
          in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) self_e_future)

val impl__subtract_reduce
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (self b: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 4095) self)
      (ensures
        fun result ->
          let result:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = result in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) result)

val impl__add_message_error_reduce
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (self message result: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) self /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) message)
      (ensures
        fun output ->
          let output:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = output in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) output)

val impl__add_error_reduce
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (self error: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 7) error)
      (ensures
        fun self_e_future ->
          let self_e_future:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            self_e_future
          in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) self_e_future)

val impl__add_standard_error_reduce
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (self error: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) error)
      (ensures
        fun self_e_future ->
          let self_e_future:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector =
            self_e_future
          in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) self_e_future)

val impl__ntt_multiply
      (#v_Vector: Type0)
      {| i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector |}
      (self rhs: Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
    : Prims.Pure (Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector)
      (requires
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) self /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) rhs)
      (ensures
        fun result ->
          let result:Libcrux_ml_kem.Vector.t_PolynomialRingElement v_Vector = result in
          Libcrux_ml_kem.Polynomial.Spec.is_bounded_poly #v_Vector (mk_usize 3328) result)

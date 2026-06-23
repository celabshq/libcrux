module Hacspec_ml_kem.Invert_ntt
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let v_INVERSE_OF_128_: Hacspec_ml_kem.Parameters.t_FieldElement =
  Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 3303)

/// Use the Gentleman-Sande butterfly to invert, in-place, the NTT representation
/// of a `Polynomial`.
/// This function implements <strong>Algorithm 9</strong> of the NIST FIPS 203 standard, which
/// is reproduced below:
/// ```plaintext
/// Input: array fˆ ∈ ℤ₂₅₆.
/// Output: array f ∈ ℤ₂₅₆.
/// f ← fˆ
/// k ← 127
/// for (len ← 2; len ≤ 128; len ← 2·len)
///     for (start ← 0; start < 256; start ← start + 2·len)
///         zeta ← ζ^(BitRev₇(k)) mod q
///         k ← k − 1
///         for (j ← start; j < start + len; j++)
///             t ← f[j]
///             f[j] ← t + f[j + len]
///             f[j + len] ← zeta·(f[j+len] − t)
///         end for
///     end for
/// end for
/// f ← f·3303 mod q
/// return f
/// ```
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
/// Gentleman–Sande butterfly: `(a, b, ζ) ↦ (a + b, ζ·(b − a))`.
/// Used in the inverse NTT.
let inv_butterfly (zeta a b: Hacspec_ml_kem.Parameters.t_FieldElement)
    : (Hacspec_ml_kem.Parameters.t_FieldElement & Hacspec_ml_kem.Parameters.t_FieldElement) =
  Hacspec_ml_kem.Parameters.impl_FieldElement__add a b,
  Hacspec_ml_kem.Parameters.impl_FieldElement__mul zeta
    (Hacspec_ml_kem.Parameters.impl_FieldElement__sub b a
      <:
      Hacspec_ml_kem.Parameters.t_FieldElement)
  <:
  (Hacspec_ml_kem.Parameters.t_FieldElement & Hacspec_ml_kem.Parameters.t_FieldElement)

#push-options "--z3rlimit 150"

/// One layer of the inverse NTT, generic over the array length `N`.
/// As in `ntt_layer_n`, `N = 2·groups·len` where `groups = zetas.len()`.
/// Each group spans `2·len` coefficients and uses one zeta for its `len`
/// Gentleman–Sande butterflies.
/// This is FIPS 203 Algorithm 9 lines 3-8 applied once at butterfly
/// half-size `len`.
let ntt_inverse_layer_n
      (v_N: usize)
      (p: t_Array Hacspec_ml_kem.Parameters.t_FieldElement v_N)
      (len: usize)
      (zetas: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
    : Prims.Pure (t_Array Hacspec_ml_kem.Parameters.t_FieldElement v_N)
      (requires
        len >=. mk_usize 1 && len <. mk_usize 1024 &&
        (Core_models.Slice.impl__len #Hacspec_ml_kem.Parameters.t_FieldElement zetas <: usize) <.
        mk_usize 1024 &&
        (((Core_models.Slice.impl__len #Hacspec_ml_kem.Parameters.t_FieldElement zetas <: usize) *!
            mk_usize 2
            <:
            usize) *!
          len
          <:
          usize) =.
        v_N)
      (fun _ -> Prims.l_True) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    v_N
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun i ->
        let i:usize = i in
        let group:usize = i /! (mk_usize 2 *! len <: usize) in
        let idx:usize = i %! (mk_usize 2 *! len <: usize) in
        if idx <. len
        then
          (inv_butterfly (zetas.[ group ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
              (p.[ i ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
              (p.[ i +! len <: usize ] <: Hacspec_ml_kem.Parameters.t_FieldElement))
            ._1
        else
          (inv_butterfly (zetas.[ group ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
              (p.[ i -! len <: usize ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
              (p.[ i ] <: Hacspec_ml_kem.Parameters.t_FieldElement))
            ._2)

#pop-options

#push-options "--z3rlimit 150"

/// One layer of the 256-coefficient inverse NTT.
/// Follows FIPS 203 Algorithm 9.  Butterfly half-size `len = 2^layer`,
/// groups = `128 / len`, zetas used = `ZETAS[groups .. 2·groups]` reversed
/// (the inverse NTT consumes the zeta table top-down).
let ntt_inverse_layer
      (p: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (layer: usize)
    : Prims.Pure (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (requires layer >=. mk_usize 1 && layer <=. mk_usize 7)
      (fun _ -> Prims.l_True) =
  let len:usize = mk_usize 1 <<! layer in
  let groups:usize = mk_usize 128 /! len in
  let (zetas: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 128)):t_Array
    Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 128) =
    Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
      (mk_usize 128)
      #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
      (fun round ->
          let round:usize = round in
          if round <. groups <: bool
          then
            Hacspec_ml_kem.Ntt.v_ZETAS.[ ((mk_usize 2 *! groups <: usize) -! mk_usize 1 <: usize) -!
              round
              <:
              usize ]
            <:
            Hacspec_ml_kem.Parameters.t_FieldElement
          else
            Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 0)
            <:
            Hacspec_ml_kem.Parameters.t_FieldElement)
  in
  ntt_inverse_layer_n (mk_usize 256)
    p
    len
    (zetas.[ { Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = groups }
        <:
        Core_models.Ops.Range.t_Range usize ]
      <:
      t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)

#pop-options

let reduce_polynomial (p: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun i ->
        let i:usize = i in
        Hacspec_ml_kem.Parameters.impl_FieldElement__mul (p.[ i ]
            <:
            Hacspec_ml_kem.Parameters.t_FieldElement)
          v_INVERSE_OF_128_
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)

#push-options "--z3rlimit 150"

/// FIPS 203 Algorithm 9 lines 3-8 only — the seven layers of
/// Gentleman–Sande inverse butterflies, *without* the final
/// `f ← f · 3303 mod q` finalization (the `· 128⁻¹` factor).
/// This is the natural intermediate form of the inverse NTT and
/// matches the impl's `invert_ntt_montgomery`, which deliberately
/// omits the `· 3303` finalization because every call site fuses it
/// with the next per-element operation (see `polynomial.rs::subtract_reduce`,
/// `add_error_reduce`, `add_message_error_reduce`'s `mont_mul(b, 1441)`
/// where `1441 ≡ R²/128 mod q`).  Reference:
/// `pq-crystals/kyber/ref/ntt.c` line 106 (the `1441 = mont²/128` comment).
/// The fully-finalized FIPS-203 INTT (`ntt_inverse` above) factors as
/// `reduce_polynomial ∘ ntt_inverse_butterflies`.
let ntt_inverse_butterflies (p: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  let p:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    ntt_inverse_layer p (mk_usize 1)
  in
  let p:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    ntt_inverse_layer p (mk_usize 2)
  in
  let p:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    ntt_inverse_layer p (mk_usize 3)
  in
  let p:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    ntt_inverse_layer p (mk_usize 4)
  in
  let p:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    ntt_inverse_layer p (mk_usize 5)
  in
  let p:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    ntt_inverse_layer p (mk_usize 6)
  in
  ntt_inverse_layer p (mk_usize 7)

#pop-options

#push-options "--z3rlimit 150"

let ntt_inverse (p: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  reduce_polynomial (ntt_inverse_butterflies p
      <:
      t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))

#pop-options

/// Inverse NTT applied to each polynomial in a vector.
let vector_inverse_ntt
      (v_RANK: usize)
      (vector_as_ntt:
          t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    : t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
  Hacspec_ml_kem.Parameters.createi #(t_Array Hacspec_ml_kem.Parameters.t_FieldElement
        (mk_usize 256))
    v_RANK
    #(usize -> t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    (fun i ->
        let i:usize = i in
        ntt_inverse (vector_as_ntt.[ i ]
            <:
            t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
        <:
        t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))

/// Performs Barrett reduction on all coefficients of a polynomial.
/// This is the spec equivalent of `poly_barrett_reduce` in the implementation.
let poly_barrett_reduce (p: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun i ->
        let i:usize = i in
        Hacspec_ml_kem.Parameters.impl_FieldElement__new ((p.[ i ]
              <:
              Hacspec_ml_kem.Parameters.t_FieldElement)
              .Hacspec_ml_kem.Parameters.f_val %!
            Hacspec_ml_kem.Parameters.v_FIELD_MODULUS
            <:
            u16)
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)

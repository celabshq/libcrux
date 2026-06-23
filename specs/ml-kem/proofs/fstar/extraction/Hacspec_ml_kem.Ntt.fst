module Hacspec_ml_kem.Ntt
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

let v_ZETA: Hacspec_ml_kem.Parameters.t_FieldElement =
  Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 17)

/// Montgomery constant R = 2^16 mod q.
/// In the implementation, coefficients are stored in Montgomery form (a * R mod q).
/// In the spec, we use plain modular arithmetic, so R is conceptually 1.
/// This constant documents the correspondence.
let v_MONTGOMERY_R: i32 = mk_i32 1

/// Montgomery domain conversion: identity in the spec.
/// In the implementation, `to_standard_domain(a)` converts from Montgomery form
/// by computing `a * MONTGOMERY_R_INV mod q`. Since the spec uses plain arithmetic
/// (effectively MONTGOMERY_R = 1), this is an identity operation.
/// Documenting this correspondence enables function-by-function verification by
/// showing that the implementation's Montgomery conversions compose to identity.
let to_standard_domain (a: Hacspec_ml_kem.Parameters.t_FieldElement)
    : Hacspec_ml_kem.Parameters.t_FieldElement = a

/// Montgomery multiplication: identity wrapper in the spec.
/// In the implementation, `montgomery_multiply_by_constant(a, c)` computes
/// `a * c * R^{-1} mod q`. In the spec, this simplifies to `a * c mod q` since R = 1.
let montgomery_multiply_by_constant (a c: Hacspec_ml_kem.Parameters.t_FieldElement)
    : Hacspec_ml_kem.Parameters.t_FieldElement =
  Hacspec_ml_kem.Parameters.impl_FieldElement__new (cast (((cast (a.Hacspec_ml_kem.Parameters.f_val
                  <:
                  u16)
              <:
              u32) *!
            (cast (c.Hacspec_ml_kem.Parameters.f_val <: u16) <: u32)
            <:
            u32) %!
          (cast (Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: u16) <: u32)
          <:
          u32)
      <:
      u16)

/// Convert a field element to its unsigned representative in [0, q).
/// Corresponds to `to_unsigned_field_modulus` / `Vector::to_unsigned_representative`
/// in the implementation.
/// In the spec, field elements are already non-negative after reduction, so this
/// is a plain modular reduction.
let to_unsigned_field_modulus (a: Hacspec_ml_kem.Parameters.t_FieldElement)
    : Hacspec_ml_kem.Parameters.t_FieldElement =
  Hacspec_ml_kem.Parameters.impl_FieldElement__new (a.Hacspec_ml_kem.Parameters.f_val %!
      Hacspec_ml_kem.Parameters.v_FIELD_MODULUS
      <:
      u16)

let bit_rev_7_ (x: usize) : usize =
  let result:usize = mk_usize 0 in
  let result:usize =
    Rust_primitives.Hax.Folds.fold_range (mk_i32 0)
      (mk_i32 7)
      (fun result temp_1_ ->
          let result:usize = result in
          let _:i32 = temp_1_ in
          true)
      result
      (fun result i ->
          let result:usize = result in
          let i:i32 = i in
          if ((x >>! i <: usize) &. mk_usize 1 <: usize) =. mk_usize 1 <: bool
          then
            let result:usize = result |. (mk_usize 1 <<! (mk_i32 6 -! i <: i32) <: usize) in
            result
          else result)
  in
  result

/// Use the Cooley–Tukey butterfly to compute an in-place NTT representation
/// of a `Polynomial`.
/// Given a `Polynomial` `f`, the NTT representation `f^` is:
/// ```plaintext
/// f^ := (f mod(X² - ζ^(2*BitRev₇(0) + 1), ..., f mod (X² − ζ^(2·BitRev₇(127) + 1))
/// ```
/// This function implements <strong>Algorithm 8</strong> of the NIST FIPS 203 standard, which
/// is reproduced below:
/// ```plaintext
/// Input: array f ∈ ℤ₂₅₆.
/// Output: array fˆ ∈ ℤ₂₅₆.
/// fˆ ← f
/// k ← 1
/// for (len ← 128; len ≥ 2; len ← len/2)
///     for (start ← 0; start < 256; start ← start + 2·len)
///         zeta ← ζ^(BitRev₇(k)) mod q
///         k ← k + 1
///         for (j ← start; j < start + len; j++)
///             t ← zeta·fˆ[j+len]
///             fˆ[j+len] ← fˆ[j] − t
///             fˆ[j] ← fˆ[j] + t
///         end for
///     end for
/// end for
/// return fˆ
/// ```
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
let v_ZETAS: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 128) =
  let list =
    [
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1729);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2580);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 3289);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2642);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 630);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1897);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 848);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1062);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1919);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 193);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 797);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2786);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 3260);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 569);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1746);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 296);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2447);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1339);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1476);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 3046);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 56);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2240);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1333);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1426);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2094);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 535);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2882);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2393);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2879);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1974);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 821);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 289);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 331);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 3253);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1756);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1197);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2304);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2277);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2055);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 650);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1977);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2513);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 632);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2865);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 33);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1320);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1915);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2319);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1435);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 807);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 452);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1438);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2868);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1534);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2402);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2647);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2617);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1481);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 648);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2474);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 3110);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1227);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 910);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 17);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2761);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 583);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2649);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1637);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 723);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2288);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1100);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1409);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2662);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 3281);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 233);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 756);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2156);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 3015);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 3050);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1703);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1651);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2789);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1789);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1847);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 952);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1461);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2687);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 939);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2308);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2437);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2388);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 733);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2337);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 268);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 641);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1584);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2298);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2037);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 3220);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 375);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2549);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2090);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1645);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1063);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 319);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2773);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 757);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2099);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 561);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2466);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2594);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2804);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1092);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 403);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1026);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1143);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2150);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2775);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 886);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1722);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1212);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1874);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 1029);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2110);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2935);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 885);
      Hacspec_ml_kem.Parameters.impl_FieldElement__new (mk_u16 2154)
    ]
  in
  FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 128);
  Rust_primitives.Hax.array_of_list 128 list

/// In the implementation, zetas are pre-multiplied by Montgomery R.
/// In the spec, ZETAS are plain values, so ZETAS_TIMES_MONTGOMERY_R == ZETAS.
/// This alias documents the correspondence with the implementation's `ZETAS_TIMES_MONTGOMERY_R`.
let v_ZETAS_TIMES_MONTGOMERY_R: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 128) =
  v_ZETAS

let get_zeta (i: usize)
    : Prims.Pure Hacspec_ml_kem.Parameters.t_FieldElement
      (requires i <. mk_usize 128)
      (fun _ -> Prims.l_True) = v_ZETAS.[ i ]

/// Cooley–Tukey butterfly: `(a, b, ζ) ↦ (a + ζ·b, a − ζ·b)`.
/// Used in the forward NTT.
let butterfly (zeta a b: Hacspec_ml_kem.Parameters.t_FieldElement)
    : (Hacspec_ml_kem.Parameters.t_FieldElement & Hacspec_ml_kem.Parameters.t_FieldElement) =
  let t:Hacspec_ml_kem.Parameters.t_FieldElement =
    Hacspec_ml_kem.Parameters.impl_FieldElement__mul zeta b
  in
  Hacspec_ml_kem.Parameters.impl_FieldElement__add a t,
  Hacspec_ml_kem.Parameters.impl_FieldElement__sub a t
  <:
  (Hacspec_ml_kem.Parameters.t_FieldElement & Hacspec_ml_kem.Parameters.t_FieldElement)

#push-options "--z3rlimit 150"

/// One layer of the NTT, generic over the array length `N`.
/// The layer is characterised by `(len, groups)` where `groups = zetas.len()`
/// and `N = 2·groups·len`.  Each of the `groups` butterfly groups spans `2·len`
/// consecutive coefficients, uses one zeta, and runs `len` independent
/// butterflies.
/// This is FIPS 203 Algorithm 8 lines 3-8 applied once at butterfly half-size
/// `len`.  The within-chunk case (N = 16, len ∈ {2, 4, 8}) corresponds to the
/// trait\'s `ntt_layer_{1,2,3}_step`; the full-polynomial case (N = 256,
/// len = 2^layer) is what `ntt_layer` below instantiates.
let ntt_layer_n
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
          (butterfly (zetas.[ group ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
              (p.[ i ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
              (p.[ i +! len <: usize ] <: Hacspec_ml_kem.Parameters.t_FieldElement))
            ._1
        else
          (butterfly (zetas.[ group ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
              (p.[ i -! len <: usize ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
              (p.[ i ] <: Hacspec_ml_kem.Parameters.t_FieldElement))
            ._2)

#pop-options

#push-options "--z3rlimit 150"

/// One layer of the 256-coefficient NTT.  Thin wrapper over `ntt_layer_n`
/// that selects the zeta slice for this layer out of the global `ZETAS`
/// table.
/// Follows FIPS 203 Algorithm 8.  Butterfly half-size `len = 2^layer`,
/// groups = `128 / len`, zetas used = `ZETAS[groups .. 2·groups]`.
let ntt_layer (p: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) (layer: usize)
    : Prims.Pure (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (requires layer >=. mk_usize 1 && layer <=. mk_usize 7)
      (fun _ -> Prims.l_True) =
  let len:usize = mk_usize 1 <<! layer in
  let groups:usize = mk_usize 128 /! len in
  ntt_layer_n (mk_usize 256)
    p
    len
    (v_ZETAS.[ {
          Core_models.Ops.Range.f_start = groups;
          Core_models.Ops.Range.f_end = mk_usize 2 *! groups <: usize
        }
        <:
        Core_models.Ops.Range.t_Range usize ]
      <:
      t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)

#pop-options

let ntt (p: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  let p:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    ntt_layer p (mk_usize 7)
  in
  let p:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    ntt_layer p (mk_usize 6)
  in
  let p:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    ntt_layer p (mk_usize 5)
  in
  let p:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    ntt_layer p (mk_usize 4)
  in
  let p:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    ntt_layer p (mk_usize 3)
  in
  let p:t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
    ntt_layer p (mk_usize 2)
  in
  ntt_layer p (mk_usize 1)

#push-options "--z3rlimit 150"

/// Compute the product of two `KyberBinomial`s with respect to the
/// modulus `X² - zeta`.
/// This function implements <strong>Algorithm 11</strong> of the NIST FIPS 203 standard, which
/// is reproduced below:
/// ```plaintext
/// Input:  a₀, a₁, b₀, b₁ ∈ ℤq.
/// Input: γ ∈ ℤq.
/// Output: c₀, c₁ ∈ ℤq.
/// c₀ ← a₀·b₀ + a₁·b₁·γ
/// c₁ ← a₀·b₁ + a₁·b₀
/// return c₀, c₁
/// ```
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
let base_case_multiply_even (a0 a1 b0 b1 zeta: Hacspec_ml_kem.Parameters.t_FieldElement)
    : Hacspec_ml_kem.Parameters.t_FieldElement =
  Hacspec_ml_kem.Parameters.impl_FieldElement__add (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        a0
        b0
      <:
      Hacspec_ml_kem.Parameters.t_FieldElement)
    (Hacspec_ml_kem.Parameters.impl_FieldElement__mul (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
            a1
            b1
          <:
          Hacspec_ml_kem.Parameters.t_FieldElement)
        zeta
      <:
      Hacspec_ml_kem.Parameters.t_FieldElement)

#pop-options

let base_case_multiply_odd (a0 a1 b0 b1: Hacspec_ml_kem.Parameters.t_FieldElement)
    : Hacspec_ml_kem.Parameters.t_FieldElement =
  Hacspec_ml_kem.Parameters.impl_FieldElement__add (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        a0
        b1
      <:
      Hacspec_ml_kem.Parameters.t_FieldElement)
    (Hacspec_ml_kem.Parameters.impl_FieldElement__mul a1 b0
      <:
      Hacspec_ml_kem.Parameters.t_FieldElement)

#push-options "--z3rlimit 150"

/// Given two `Polynomial`s in their NTT representations,
/// compute their product. Given two polynomials in the NTT domain `f^` and `ĵ`,
/// the `iᵗʰ` coefficient of the product `k\u{302}` is determined by the calculation:
/// ```plaintext
/// ĥ[2·i] + ĥ[2·i + 1]X = (f^[2·i] + f^[2·i + 1]X)·(ĝ[2·i] + ĝ[2·i + 1]X) mod (X² - ζ^(2·BitRev₇(i) + 1))
/// ```
/// This function implements <strong>Algorithm 10</strong> of the NIST FIPS 203 standard, which
/// is reproduced below:
/// ```plaintext
/// Input: Two arrays fˆ ∈ ℤ₂₅₆ and ĝ ∈ ℤ₂₅₆.
/// Output: An array ĥ ∈ ℤq.
/// for(i ← 0; i < 128; i++)
///     (ĥ[2i], ĥ[2i+1]) ← BaseCaseMultiply(fˆ[2i], fˆ[2i+1], ĝ[2i], ĝ[2i+1], ζ^(2·BitRev₇(i) + 1))
/// end for
/// return ĥ
/// ```
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
/// Pointwise polynomial multiplication in the NTT domain, generic over
/// the array length `N`.
/// The input is two NTT-domain arrays of `N` coefficients and an array of
/// `N/4` zetas.  Consecutive 4-coefficient groups are treated as two
/// quadratic polynomials multiplied modulo `X² − ζ`: the first pair uses
/// `+ζ`, the second pair uses `−ζ`.  This is the trait-compatible
/// restriction of FIPS 203 Algorithm 10.
/// When instantiated at N=256 with `zetas = ZETAS[64..128]` this is the
/// full `multiply_ntts` below.  When instantiated at N=16 with 4 zetas
/// this is the trait\'s `ntt_multiply(lhs, rhs, z0, z1, z2, z3)`.
let ntt_multiply_n
      (v_N: usize)
      (p1 p2: t_Array Hacspec_ml_kem.Parameters.t_FieldElement v_N)
      (zetas: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
    : Prims.Pure (t_Array Hacspec_ml_kem.Parameters.t_FieldElement v_N)
      (requires
        (Core_models.Slice.impl__len #Hacspec_ml_kem.Parameters.t_FieldElement zetas <: usize) <.
        mk_usize 1024 &&
        ((Core_models.Slice.impl__len #Hacspec_ml_kem.Parameters.t_FieldElement zetas <: usize) *!
          mk_usize 4
          <:
          usize) =.
        v_N)
      (fun _ -> Prims.l_True) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    v_N
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun i ->
        let i:usize = i in
        let group:usize = i /! mk_usize 4 in
        let zeta:Hacspec_ml_kem.Parameters.t_FieldElement =
          if (i %! mk_usize 4 <: usize) <. mk_usize 2
          then zetas.[ group ]
          else
            Hacspec_ml_kem.Parameters.impl_FieldElement__neg (zetas.[ group ]
                <:
                Hacspec_ml_kem.Parameters.t_FieldElement)
        in
        if (i %! mk_usize 2 <: usize) =. mk_usize 0
        then
          base_case_multiply_even (p1.[ i ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
            (p1.[ i +! mk_usize 1 <: usize ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
            (p2.[ i ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
            (p2.[ i +! mk_usize 1 <: usize ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
            zeta
        else
          base_case_multiply_odd (p1.[ i -! mk_usize 1 <: usize ]
              <:
              Hacspec_ml_kem.Parameters.t_FieldElement)
            (p1.[ i ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
            (p2.[ i -! mk_usize 1 <: usize ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
            (p2.[ i ] <: Hacspec_ml_kem.Parameters.t_FieldElement))

#pop-options

let multiply_ntts (p1 p2: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    : t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256) =
  ntt_multiply_n (mk_usize 256)
    p1
    p2
    (v_ZETAS.[ {
          Core_models.Ops.Range.f_start = mk_usize 64;
          Core_models.Ops.Range.f_end = mk_usize 128
        }
        <:
        Core_models.Ops.Range.t_Range usize ]
      <:
      t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)

let vector_ntt
      (v_RANK: usize)
      (vector: t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK)
    : t_Array (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256)) v_RANK =
  Hacspec_ml_kem.Parameters.createi #(t_Array Hacspec_ml_kem.Parameters.t_FieldElement
        (mk_usize 256))
    v_RANK
    #(usize -> t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
    (fun i ->
        let i:usize = i in
        ntt (vector.[ i ] <: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
        <:
        t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))

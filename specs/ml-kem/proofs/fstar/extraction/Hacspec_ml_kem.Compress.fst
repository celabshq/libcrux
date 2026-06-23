module Hacspec_ml_kem.Compress
#set-options "--fuel 0 --ifuel 1 --z3rlimit 15"
open FStar.Mul
open Core_models

#push-options "--z3rlimit 1500"

/// This function implements the `Compress` function specified in the NIST FIPS
/// 203 standard (Page 18, Expression 4.5), which is defined as:
/// ```plaintext
/// Compress_d: ℤq -> ℤ_{2ᵈ}
/// Compress_d(x) = ⌈(2ᵈ/q)·x⌋
/// ```
/// Since `⌈x⌋ = ⌊x + 1/2⌋` we have:
/// ```plaintext
/// Compress_d(x) = ⌊(2ᵈ/q)·x + 1/2⌋
///               = ⌊(2^{d+1}·x + q) / 2q⌋
/// ```
/// this latter expression is what the code computes, since it enables us to
/// avoid the use of floating point computations as required by the standard.
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
let compress_d (fe: Hacspec_ml_kem.Parameters.t_FieldElement) (to_bit_size: usize)
    : Prims.Pure Hacspec_ml_kem.Parameters.t_FieldElement
      (requires to_bit_size <. mk_usize 12)
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let two_pow_bit_size:u32 =
    Core_models.Num.impl_u32__pow (mk_u32 2) (cast (to_bit_size <: usize) <: u32)
  in
  let compressed:u32 =
    ((((cast (fe.Hacspec_ml_kem.Parameters.f_val <: u16) <: u32) *! mk_u32 2 <: u32) *!
        two_pow_bit_size
        <:
        u32) +!
      (cast (Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: u16) <: u32)
      <:
      u32) /!
    (mk_u32 2 *! (cast (Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: u16) <: u32) <: u32)
  in
  Hacspec_ml_kem.Parameters.impl_FieldElement__new (cast (compressed %! two_pow_bit_size <: u32)
      <:
      u16)

#pop-options

#push-options "--z3rlimit 150"

/// According to the NIST FIPS 203 standard (Page 10, Lines 536 - 539),
/// compressing a polynomial ring element is accomplished by `compress()`ing its
/// constituent field coefficients.
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
let compress
      (re: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (bits_per_compressed_coefficient: usize)
    : Prims.Pure (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (requires bits_per_compressed_coefficient <. mk_usize 12)
      (fun _ -> Prims.l_True) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun i ->
        let i:usize = i in
        compress_d (re.[ i ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
          bits_per_compressed_coefficient
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)

#pop-options

#push-options "--z3rlimit 150"

/// This function implements the `Decompress` function specified in the NIST FIPS
/// 203 standard (Page 18, Expression 4.6), which is defined as:
/// ```plaintext
/// Decompress_d: ℤ_{2ᵈ} -> ℤq
/// Decompress_d(y) = ⌈(q/2ᵈ)·y⌋
/// ```
/// Since `⌈x⌋ = ⌊x + 1/2⌋` we have:
/// ```plaintext
/// Decompress_d(y) = ⌊(q/2ᵈ)·y + 1/2⌋
///                 = ⌊(2·y·q + 2ᵈ) / 2^{d+1})⌋
/// ```
/// this latter expression is what the code computes, since it enables us to
/// avoid the use of floating point computations as required by the standard.
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
let decompress_d (fe: Hacspec_ml_kem.Parameters.t_FieldElement) (to_bit_size: usize)
    : Prims.Pure Hacspec_ml_kem.Parameters.t_FieldElement
      (requires
        to_bit_size <. mk_usize 12 &&
        fe.Hacspec_ml_kem.Parameters.f_val <. (mk_u16 1 <<! to_bit_size <: u16))
      (fun _ -> Prims.l_True) =
  let _:Prims.unit = () <: Prims.unit in
  let two_pow_bit_size:u32 =
    Core_models.Num.impl_u32__pow (mk_u32 2) (cast (to_bit_size <: usize) <: u32)
  in
  let numerator:u32 =
    ((mk_u32 2 *! (cast (fe.Hacspec_ml_kem.Parameters.f_val <: u16) <: u32) <: u32) *!
      (cast (Hacspec_ml_kem.Parameters.v_FIELD_MODULUS <: u16) <: u32)
      <:
      u32) +!
    two_pow_bit_size
  in
  let decompressed:u32 = numerator /! (two_pow_bit_size *! mk_u32 2 <: u32) in
  Hacspec_ml_kem.Parameters.impl_FieldElement__new (cast (decompressed <: u32) <: u16)

#pop-options

#push-options "--z3rlimit 150"

/// According to the NIST FIPS 203 standard (Page 10, Lines 536 - 539),
/// compressing a polynomial ring element is accomplished by `decompress()`ing
/// its constituent field coefficients.
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
let decompress
      (re: t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (bits_per_compressed_coefficient: usize)
    : Prims.Pure (t_Array Hacspec_ml_kem.Parameters.t_FieldElement (mk_usize 256))
      (requires
        b2t (bits_per_compressed_coefficient <. mk_usize 12 <: bool) /\
        (forall (i: usize).
            b2t (i <. mk_usize 256 <: bool) ==>
            b2t
            ((re.[ i ] <: Hacspec_ml_kem.Parameters.t_FieldElement).Hacspec_ml_kem.Parameters.f_val <.
              (mk_u16 1 <<! bits_per_compressed_coefficient <: u16)
              <:
              bool)))
      (fun _ -> Prims.l_True) =
  Hacspec_ml_kem.Parameters.createi #Hacspec_ml_kem.Parameters.t_FieldElement
    (mk_usize 256)
    #(usize -> Hacspec_ml_kem.Parameters.t_FieldElement)
    (fun i ->
        let i:usize = i in
        decompress_d (re.[ i ] <: Hacspec_ml_kem.Parameters.t_FieldElement)
          bits_per_compressed_coefficient
        <:
        Hacspec_ml_kem.Parameters.t_FieldElement)

#pop-options

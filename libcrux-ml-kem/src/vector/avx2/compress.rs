use super::*;
use crate::vector::FIELD_MODULUS;

// Multiply the 32-bit numbers contained in |lhs| and |rhs|, and store only
// the upper 32 bits of the resulting product.
// This implementation was taken from:
// https://ei1333.github.io/library/math/combinatorics/vectorize-mod-int.hpp.html
//
// TODO: Optimize this implementation if performance numbers suggest doing so.
#[inline(always)]
fn mulhi_mm256_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    let prod02 = mm256_mul_epu32(lhs, rhs);
    let prod13 = mm256_mul_epu32(
        mm256_shuffle_epi32::<0b11_11_01_01>(lhs),
        mm256_shuffle_epi32::<0b11_11_01_01>(rhs),
    );

    mm256_unpackhi_epi64(
        mm256_unpacklo_epi32(prod02, prod13),
        mm256_unpackhi_epi32(prod02, prod13),
    )
}

#[inline(always)]
pub(crate) fn compress_message_coefficient(vector: Vec256) -> Vec256 {
    let field_modulus_halved = mm256_set1_epi16((FIELD_MODULUS - 1) / 2);
    let field_modulus_quartered = mm256_set1_epi16((FIELD_MODULUS - 1) / 4);

    let shifted = mm256_sub_epi16(field_modulus_halved, vector);
    let mask = mm256_srai_epi16::<15>(shifted);

    let shifted_to_positive = mm256_xor_si256(mask, shifted);
    let shifted_to_positive_in_range =
        mm256_sub_epi16(shifted_to_positive, field_modulus_quartered);

    mm256_srli_epi16::<15>(shifted_to_positive_in_range)
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"v $COEFFICIENT_BITS >= 0 /\ v $COEFFICIENT_BITS < bits i32_inttype /\
    range (v ((mk_i32 1) <<! $COEFFICIENT_BITS) - 1) i32_inttype"#)))]
pub(crate) fn compress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(
    vector: Vec256,
) -> Vec256 {
    let field_modulus_halved = mm256_set1_epi32(((FIELD_MODULUS as i32) - 1) / 2);
    let compression_factor = mm256_set1_epi32(10_321_340);
    let coefficient_bits_mask = mm256_set1_epi32((1 << COEFFICIENT_BITS) - 1);

    // ---- Compress the first 8 coefficients ----

    // Take the bottom 128 bits, i.e. the first 8 16-bit coefficients
    let coefficients_low = mm256_castsi256_si128(vector);

    // If:
    //
    // coefficients_low[0:15] = A
    // coefficients_low[16:31] = B
    // coefficients_low[32:63] = C
    // and so on ...
    //
    // after this step:
    //
    // coefficients_low[0:31] = A
    // coefficients_low[32:63] = B
    // and so on ...
    let coefficients_low = mm256_cvtepi16_epi32(coefficients_low);

    let compressed_low = mm256_slli_epi32::<{ COEFFICIENT_BITS }>(coefficients_low);
    let compressed_low = mm256_add_epi32(compressed_low, field_modulus_halved);

    let compressed_low = mulhi_mm256_epi32(compressed_low, compression_factor);

    // Due to the mulhi_mm256_epi32 we've already shifted right by 32 bits, we
    // just need to shift right by 35 - 32 = 3 more.
    let compressed_low = mm256_srli_epi32::<3>(compressed_low);

    let compressed_low = mm256_and_si256(compressed_low, coefficient_bits_mask);

    // ---- Compress the next 8 coefficients ----

    // Take the upper 128 bits, i.e. the next 8 16-bit coefficients
    let coefficients_high = mm256_extracti128_si256::<1>(vector);
    let coefficients_high = mm256_cvtepi16_epi32(coefficients_high);

    let compressed_high = mm256_slli_epi32::<{ COEFFICIENT_BITS }>(coefficients_high);
    let compressed_high = mm256_add_epi32(compressed_high, field_modulus_halved);

    let compressed_high = mulhi_mm256_epi32(compressed_high, compression_factor);
    let compressed_high = mm256_srli_epi32::<3>(compressed_high);
    let compressed_high = mm256_and_si256(compressed_high, coefficient_bits_mask);

    // Combining them, and grouping each set of 64-bits, this function results
    // in:
    //
    // 0: low low low low | 1: high high high high | 2: low low low low | 3: high high high high
    //
    // where each |low| and |high| is a 16-bit element
    let compressed = mm256_packs_epi32(compressed_low, compressed_high);

    // To be in the right order, we need to move the |low|s above in position 2 to
    // position 1 and the |high|s in position 1 to position 2, and leave the
    // rest unchanged.
    mm256_permute4x64_epi64::<0b11_01_10_00>(compressed)
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"forall i. let x = Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i in 
                                      (x == mk_i16 0 \/ x == mk_i16 1)"#)))]
pub fn decompress_1(a: Vec256) -> Vec256 {
    let z = mm256_setzero_si256();

    #[cfg(hax)]
    hax_lib::fstar!(
        r#"
        assert(Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $z == Seq.create 16 (mk_i16 0));
        assert(forall i. Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $z) i == mk_i16 0);
        assert(forall i. let x = Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i in 
                                      ((0 - v x) == 0 \/ (0 - v x) == -1));
        assert(forall i. i < 16 ==>
                        Spec.Utils.is_intb (pow2 15 - 1) 
                        (0 - v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $a) i)))
    "#
    );

    let s = arithmetic::sub(z, a);

    #[cfg(hax)]
    hax_lib::fstar!(
        r#"assert(forall i. Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $s) i == mk_i16 0 \/ 
                            Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 $s) i == mk_i16 (-1))"#
    );

    arithmetic::bitwise_and_with_constant(s, 1665)
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"v $COEFFICIENT_BITS >= 0 /\ v $COEFFICIENT_BITS < bits i32_inttype"#)))]
pub(crate) fn decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(
    vector: Vec256,
) -> Vec256 {
    let field_modulus = mm256_set1_epi32(FIELD_MODULUS as i32);
    let two_pow_coefficient_bits = mm256_set1_epi32(1 << COEFFICIENT_BITS);

    // ---- Compress the first 8 coefficients ----
    let coefficients_low = mm256_castsi256_si128(vector);
    let coefficients_low = mm256_cvtepi16_epi32(coefficients_low);

    let decompressed_low = mm256_mullo_epi32(coefficients_low, field_modulus);
    let decompressed_low = mm256_slli_epi32::<1>(decompressed_low);
    let decompressed_low = mm256_add_epi32(decompressed_low, two_pow_coefficient_bits);

    // We can't shift in one go by (COEFFICIENT_BITS + 1) due to the lack
    // of support for const generic expressions.
    let decompressed_low = mm256_srli_epi32::<{ COEFFICIENT_BITS }>(decompressed_low);
    let decompressed_low = mm256_srli_epi32::<1>(decompressed_low);

    // ---- Compress the next 8 coefficients ----
    let coefficients_high = mm256_extracti128_si256::<1>(vector);
    let coefficients_high = mm256_cvtepi16_epi32(coefficients_high);

    let decompressed_high = mm256_mullo_epi32(coefficients_high, field_modulus);
    let decompressed_high = mm256_slli_epi32::<1>(decompressed_high);
    let decompressed_high = mm256_add_epi32(decompressed_high, two_pow_coefficient_bits);

    // We can't shift in one go by (COEFFICIENT_BITS + 1) due to the lack
    // of support for const generic expressions.
    let decompressed_high = mm256_srli_epi32::<{ COEFFICIENT_BITS }>(decompressed_high);
    let decompressed_high = mm256_srli_epi32::<1>(decompressed_high);

    // Combining them, and grouping each set of 64-bits, this function results
    // in:
    //
    // 0: low low low low | 1: high high high high | 2: low low low low | 3: high high high high
    //
    // where each |low| and |high| is a 16-bit element
    let compressed = mm256_packs_epi32(decompressed_low, decompressed_high);

    // To be in the right order, we need to move the |low|s above in position 2 to
    // position 1 and the |high|s in position 1 to position 2, and leave the
    // rest unchanged.
    mm256_permute4x64_epi64::<0b11_01_10_00>(compressed)
}

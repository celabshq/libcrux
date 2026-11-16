use super::vector_type::*;
use crate::vector::FIELD_MODULUS;
use libcrux_intrinsics::arm64::*;

#[inline(always)]
pub(crate) fn compress_1(mut v: SIMD128Vector) -> SIMD128Vector {
    // This is what we are trying to do in portable:
    // let shifted: i16 = 1664 - (fe as i16);
    // let mask = shifted >> 15;
    // let shifted_to_positive = mask ^ shifted;
    // let shifted_positive_in_range = shifted_to_positive - 832;
    // ((shifted_positive_in_range >> 15) & 1) as u8

    let half = _vdupq_n_s16(1664);
    let quarter = _vdupq_n_s16(832);

    let shifted = _vsubq_s16(half, v.low);
    let mask = _vshrq_n_s16::<15>(shifted);
    let shifted_to_positive = _veorq_s16(mask, shifted);
    let shifted_positive_in_range = _vsubq_s16(shifted_to_positive, quarter);
    v.low = _vreinterpretq_s16_u16(_vshrq_n_u16::<15>(_vreinterpretq_u16_s16(
        shifted_positive_in_range,
    )));

    let shifted = _vsubq_s16(half, v.high);
    let mask = _vshrq_n_s16::<15>(shifted);
    let shifted_to_positive = _veorq_s16(mask, shifted);
    let shifted_positive_in_range = _vsubq_s16(shifted_to_positive, quarter);
    v.high = _vreinterpretq_s16_u16(_vshrq_n_u16::<15>(_vreinterpretq_u16_s16(
        shifted_positive_in_range,
    )));

    v
}

#[inline(always)]
fn mask_n_least_significant_bits(coefficient_bits: i16) -> i16 {
    match coefficient_bits {
        4 => 0x0f,
        5 => 0x1f,
        10 => 0x3ff,
        11 => 0x7ff,
        x => (1 << x) - 1,
    }
}

#[inline(always)]
fn compress_int32x4_t<const COEFFICIENT_BITS: i32>(v: _uint32x4_t) -> _uint32x4_t {
    // This is what we are trying to do in portable:
    // let mut compressed = (fe as u64) << coefficient_bits;
    // compressed += 1664 as u64;
    // compressed *= 10_321_340;
    // compressed >>= 35;
    // get_n_least_significant_bits(coefficient_bits, compressed as u32) as FieldElement
    let half = _vdupq_n_u32(1664);
    let compressed = _vshlq_n_u32::<COEFFICIENT_BITS>(v);
    let compressed = _vaddq_u32(compressed, half);
    let compressed = _vreinterpretq_u32_s32(_vqdmulhq_n_s32(
        _vreinterpretq_s32_u32(compressed),
        10_321_340,
    ));
    let compressed = _vshrq_n_u32::<4>(compressed);
    compressed
}

#[inline(always)]
pub(crate) fn compress<const COEFFICIENT_BITS: i32>(mut v: SIMD128Vector) -> SIMD128Vector {
    // This is what we are trying to do in portable:
    // let mut compressed = (fe as u64) << coefficient_bits;
    // compressed += 1664 as u64;
    // compressed *= 10_321_340;
    // compressed >>= 35;
    // get_n_least_significant_bits(coefficient_bits, compressed as u32) as FieldElement

    let mask = _vdupq_n_s16(mask_n_least_significant_bits(COEFFICIENT_BITS as i16));
    let mask16 = _vdupq_n_u32(0xffff);

    let low0 = _vandq_u32(_vreinterpretq_u32_s16(v.low), mask16); //a0, a2, a4, a6
    let low1 = _vshrq_n_u32::<16>(_vreinterpretq_u32_s16(v.low)); //a1, a3, a5, a7
    let high0 = _vandq_u32(_vreinterpretq_u32_s16(v.high), mask16); //a0, a2, a4, a6
    let high1 = _vshrq_n_u32::<16>(_vreinterpretq_u32_s16(v.high)); //a1, a3, a5, a7

    let low0 = compress_int32x4_t::<COEFFICIENT_BITS>(low0);
    let low1 = compress_int32x4_t::<COEFFICIENT_BITS>(low1);
    let high0 = compress_int32x4_t::<COEFFICIENT_BITS>(high0);
    let high1 = compress_int32x4_t::<COEFFICIENT_BITS>(high1);

    let low = _vtrn1q_s16(_vreinterpretq_s16_u32(low0), _vreinterpretq_s16_u32(low1));
    let high = _vtrn1q_s16(_vreinterpretq_s16_u32(high0), _vreinterpretq_s16_u32(high1));

    v.low = _vandq_s16(low, mask);
    v.high = _vandq_s16(high, mask);
    v
}

#[inline(always)]
fn decompress_uint32x4_t<const COEFFICIENT_BITS: i32>(v: _uint32x4_t) -> _uint32x4_t {
    let coeff = _vdupq_n_u32(1 << (COEFFICIENT_BITS - 1));
    let decompressed = _vmulq_n_u32(v, FIELD_MODULUS as u32);
    let decompressed = _vaddq_u32(decompressed, coeff);
    let decompressed = _vshrq_n_u32::<COEFFICIENT_BITS>(decompressed);

    decompressed
}

#[inline(always)]
pub fn decompress_1(a: SIMD128Vector) -> SIMD128Vector {
    let z = ZERO();
    let s = super::arithmetic::sub(z, &a);
    super::arithmetic::bitwise_and_with_constant(s, 1665)
}

#[inline(always)]
pub(crate) fn decompress_ciphertext_coefficient<const COEFFICIENT_BITS: i32>(
    mut v: SIMD128Vector,
) -> SIMD128Vector {
    let mask16 = _vdupq_n_u32(0xffff);
    let low0 = _vandq_u32(_vreinterpretq_u32_s16(v.low), mask16);
    let low1 = _vshrq_n_u32::<16>(_vreinterpretq_u32_s16(v.low));
    let high0 = _vandq_u32(_vreinterpretq_u32_s16(v.high), mask16);
    let high1 = _vshrq_n_u32::<16>(_vreinterpretq_u32_s16(v.high));

    let low0 = decompress_uint32x4_t::<COEFFICIENT_BITS>(low0);
    let low1 = decompress_uint32x4_t::<COEFFICIENT_BITS>(low1);
    let high0 = decompress_uint32x4_t::<COEFFICIENT_BITS>(high0);
    let high1 = decompress_uint32x4_t::<COEFFICIENT_BITS>(high1);

    v.low = _vtrn1q_s16(_vreinterpretq_s16_u32(low0), _vreinterpretq_s16_u32(low1));
    v.high = _vtrn1q_s16(_vreinterpretq_s16_u32(high0), _vreinterpretq_s16_u32(high1));
    v
}

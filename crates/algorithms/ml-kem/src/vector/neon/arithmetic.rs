use super::vector_type::*;
use crate::vector::{traits::INVERSE_OF_MODULUS_MOD_MONTGOMERY_R, FIELD_MODULUS};
use libcrux_intrinsics::arm64::*;

#[inline(always)]
pub(crate) fn add(mut lhs: SIMD128Vector, rhs: &SIMD128Vector) -> SIMD128Vector {
    lhs.low = _vaddq_s16(lhs.low, rhs.low);
    lhs.high = _vaddq_s16(lhs.high, rhs.high);
    lhs
}

#[inline(always)]
pub(crate) fn sub(mut lhs: SIMD128Vector, rhs: &SIMD128Vector) -> SIMD128Vector {
    lhs.low = _vsubq_s16(lhs.low, rhs.low);
    lhs.high = _vsubq_s16(lhs.high, rhs.high);
    lhs
}

#[inline(always)]
pub(crate) fn multiply_by_constant(mut v: SIMD128Vector, c: i16) -> SIMD128Vector {
    v.low = _vmulq_n_s16(v.low, c);
    v.high = _vmulq_n_s16(v.high, c);
    v
}

#[inline(always)]
pub(crate) fn bitwise_and_with_constant(mut v: SIMD128Vector, c: i16) -> SIMD128Vector {
    let c = _vdupq_n_s16(c);
    v.low = _vandq_s16(v.low, c);
    v.high = _vandq_s16(v.high, c);
    v
}

#[inline(always)]
pub(crate) fn shift_right<const SHIFT_BY: i32>(mut v: SIMD128Vector) -> SIMD128Vector {
    // Should find special cases of this
    // e.g when doing a right shift just to propagate signed bits, use vclezq_s32 instead
    v.low = _vshrq_n_s16::<SHIFT_BY>(v.low);
    v.high = _vshrq_n_s16::<SHIFT_BY>(v.high);
    v
}

// #[inline(always)]
// pub(crate) fn shift_left<const SHIFT_BY: i32>(mut lhs: SIMD128Vector) -> SIMD128Vector {
//     lhs.low = _vshlq_n_s16::<SHIFT_BY>(lhs.low);
//     lhs.high = _vshlq_n_s16::<SHIFT_BY>(lhs.high);
//     lhs
// }

#[inline(always)]
pub(crate) fn cond_subtract_3329(mut v: SIMD128Vector) -> SIMD128Vector {
    let c = _vdupq_n_s16(3329);
    let m0 = _vcgeq_s16(v.low, c);
    let m1 = _vcgeq_s16(v.high, c);
    let c0 = _vandq_s16(c, _vreinterpretq_s16_u16(m0));
    let c1 = _vandq_s16(c, _vreinterpretq_s16_u16(m1));
    v.low = _vsubq_s16(v.low, c0);
    v.high = _vsubq_s16(v.high, c1);
    v
}

const BARRETT_MULTIPLIER: i16 = 20159;

#[inline(always)]
pub(crate) fn barrett_reduce_int16x8_t(v: _int16x8_t) -> _int16x8_t {
    // This is what we are trying to do in portable:
    // let t = (value as i32 * BARRETT_MULTIPLIER) + (BARRETT_R >> 1);
    // let quotient = (t >> BARRETT_SHIFT) as i16;
    // let result = value - (quotient * FIELD_MODULUS);

    let adder = _vdupq_n_s16(1024);
    let vec = _vqdmulhq_n_s16(v, BARRETT_MULTIPLIER as i16);
    let vec = _vaddq_s16(vec, adder);
    let quotient = _vshrq_n_s16::<11>(vec);
    let sub = _vmulq_n_s16(quotient, FIELD_MODULUS);
    _vsubq_s16(v, sub)
}

#[inline(always)]
pub(crate) fn barrett_reduce(mut v: SIMD128Vector) -> SIMD128Vector {
    //let pv = crate::simd::portable::from_i16_array(to_i16_array(v));
    //from_i16_array(crate::simd::portable::to_i16_array(crate::simd::portable::barrett_reduce(pv)))

    // This is what we are trying to do in portable:
    // let t = (value as i32 * BARRETT_MULTIPLIER) + (BARRETT_R >> 1);
    // let quotient = (t >> BARRETT_SHIFT) as i16;
    // let result = value - (quotient * FIELD_MODULUS);

    v.low = barrett_reduce_int16x8_t(v.low);
    v.high = barrett_reduce_int16x8_t(v.high);
    v
}

#[inline(always)]
pub(crate) fn montgomery_reduce_int16x8_t(low: _int16x8_t, high: _int16x8_t) -> _int16x8_t {
    // This is what we are trying to do in portable:
    // let k = low as i16 * INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;
    // let k_times_modulus = (k as i16 as i16) * (FIELD_MODULUS as i16);
    // let c = (k_times_modulus >> MONTGOMERY_SHIFT) as i16;
    // high - c

    let k = _vreinterpretq_s16_u16(_vmulq_n_u16(
        _vreinterpretq_u16_s16(low),
        INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as u16,
    ));
    let c = _vshrq_n_s16::<1>(_vqdmulhq_n_s16(k, FIELD_MODULUS as i16));
    _vsubq_s16(high, c)
}

#[inline(always)]
pub(crate) fn montgomery_multiply_by_constant_int16x8_t(v: _int16x8_t, c: i16) -> _int16x8_t {
    // This is what we are trying to do in portable:
    // let value = v as i16 * c
    // let k = (value as i16) as i16 * INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;
    // let k_times_modulus = (k as i16 as i16) * (FIELD_MODULUS as i16);
    // let c = (k_times_modulus >> MONTGOMERY_SHIFT) as i16;
    // let value_high = (value >> MONTGOMERY_SHIFT) as i16;
    // value_high - c

    let v_low = _vmulq_n_s16(v, c);
    let v_high = _vshrq_n_s16::<1>(_vqdmulhq_n_s16(v, c));
    montgomery_reduce_int16x8_t(v_low, v_high)
}

#[inline(always)]
pub(crate) fn montgomery_multiply_int16x8_t(v: _int16x8_t, c: _int16x8_t) -> _int16x8_t {
    // This is what we are trying to do in portable:
    // let value = v as i16 * c
    // let k = (value as i16) as i16 * INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;
    // let k_times_modulus = (k as i16 as i16) * (FIELD_MODULUS as i16);
    // let c = (k_times_modulus >> MONTGOMERY_SHIFT) as i16;
    // let value_high = (value >> MONTGOMERY_SHIFT) as i16;
    // value_high - c

    let v_low = _vmulq_s16(v, c);
    let v_high = _vshrq_n_s16::<1>(_vqdmulhq_s16(v, c));
    montgomery_reduce_int16x8_t(v_low, v_high)
}

#[inline(always)]
pub(crate) fn montgomery_multiply_by_constant(mut v: SIMD128Vector, c: i16) -> SIMD128Vector {
    v.low = montgomery_multiply_by_constant_int16x8_t(v.low, c);
    v.high = montgomery_multiply_by_constant_int16x8_t(v.high, c);
    v
}

#[inline(always)]
pub(crate) fn to_unsigned_representative(a: SIMD128Vector) -> SIMD128Vector {
    let t = shift_right::<15>(a);
    let fm = bitwise_and_with_constant(t, FIELD_MODULUS);
    add(a, &fm)
}

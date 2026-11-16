#![forbid(unsafe_code)]

use super::intrinsics::*;
use crate::vector::rej_sample_table::REJECTION_SAMPLE_SHUFFLE_TABLE;

#[inline(always)]
pub(crate) fn rej_sample(a: &[u8], out: &mut [i16]) -> usize {
    let neon_bits: [u16; 8] = [0x1, 0x2, 0x4, 0x8, 0x10, 0x20, 0x40, 0x80];
    let bits = _vld1q_u16(&neon_bits);
    let fm = _vdupq_n_s16(3328);

    let input = super::simd128ops::deserialize_12(a);
    let mask0 = _vcleq_s16(input.low, fm);
    let mask1 = _vcleq_s16(input.high, fm);
    let masked = _vandq_u16(mask0, bits);
    let used0 = _vaddvq_u16(masked);
    let masked = _vandq_u16(mask1, bits);
    let used1 = _vaddvq_u16(masked);
    let pick0 = used0.count_ones();
    let pick1 = used1.count_ones();

    // XXX: the indices used0 and used1 must be < 256.
    let index_vec0 = _vld1q_u8(&REJECTION_SAMPLE_SHUFFLE_TABLE[(used0 as u8) as usize]);
    let shifted0 = _vreinterpretq_s16_u8(_vqtbl1q_u8(_vreinterpretq_u8_s16(input.low), index_vec0));
    let index_vec1 = _vld1q_u8(&REJECTION_SAMPLE_SHUFFLE_TABLE[(used1 as u8) as usize]);
    let shifted1 =
        _vreinterpretq_s16_u8(_vqtbl1q_u8(_vreinterpretq_u8_s16(input.high), index_vec1));

    let idx0 = pick0 as usize;
    _vst1q_s16(&mut out[0..8], shifted0);
    _vst1q_s16(&mut out[idx0..idx0 + 8], shifted1);
    (pick0 + pick1) as usize
}

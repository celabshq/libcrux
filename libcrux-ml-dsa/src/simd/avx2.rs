use crate::{
    constants::{Eta, Gamma2},
    simd::{avx2::arithmetic::barrett_reduce_simd_unit, traits::*},
};

mod arithmetic;
mod encoding;
mod invntt;
mod ntt;
mod rejection_sample;
mod vector_type;

use arithmetic::shift_left_then_reduce;
pub(crate) use vector_type::{AVX2RingElement, Vec256 as AVX2SIMDUnit};

#[cfg(hax)]
impl Repr for AVX2SIMDUnit {
    fn repr(&self) -> [i32; COEFFICIENTS_IN_SIMD_UNIT] {
        let mut result = [0i32; COEFFICIENTS_IN_SIMD_UNIT];
        vector_type::to_coefficient_array(self, &mut result);
        result
    }
}

#[cfg(not(hax))]
impl Repr for AVX2SIMDUnit {}

/// Implementing the [`Operations`] for AVX2.
impl Operations for AVX2SIMDUnit {
    #[inline(always)]
    fn zero() -> Self {
        vector_type::zero()
    }

    #[inline(always)]
    fn from_coefficient_array(coefficient_array: &[i32], out: &mut Self) {
        vector_type::from_coefficient_array(coefficient_array, out)
    }

    #[inline(always)]
    fn to_coefficient_array(value: &Self, out: &mut [i32]) {
        vector_type::to_coefficient_array(value, out)
    }

    #[inline(always)]
    fn add(lhs: &mut Self, rhs: &Self) {
        arithmetic::add(&mut lhs.value, &rhs.value)
    }

    #[inline(always)]
    fn subtract(lhs: &mut Self, rhs: &Self) {
        arithmetic::subtract(&mut lhs.value, &rhs.value)
    }

    #[inline(always)]
    fn infinity_norm_exceeds(simd_unit: &Self, bound: i32) -> bool {
        arithmetic::infinity_norm_exceeds(&simd_unit.value, bound)
    }

    #[inline(always)]
    fn decompose(gamma2: Gamma2, simd_unit: &Self, low: &mut Self, high: &mut Self) {
        arithmetic::decompose(gamma2, &simd_unit.value, &mut low.value, &mut high.value);
    }

    #[inline(always)]
    fn compute_hint(low: &Self, high: &Self, gamma2: i32, hint: &mut Self) -> usize {
        arithmetic::compute_hint(&low.value, &high.value, gamma2, &mut hint.value)
    }

    #[inline(always)]
    fn use_hint(gamma2: Gamma2, simd_unit: &Self, hint: &mut Self) {
        arithmetic::use_hint(gamma2, &simd_unit.value, &mut hint.value);
    }

    #[inline(always)]
    fn montgomery_multiply(lhs: &mut Self, rhs: &Self) {
        arithmetic::montgomery_multiply(&mut lhs.value, &rhs.value);
    }

    #[inline(always)]
    fn shift_left_then_reduce<const SHIFT_BY: i32>(simd_unit: &mut Self) {
        shift_left_then_reduce::<SHIFT_BY>(&mut simd_unit.value)
    }

    #[inline(always)]
    fn power2round(t0: &mut Self, t1: &mut Self) {
        arithmetic::power2round(&mut t0.value, &mut t1.value);
    }

    #[inline(always)]
    fn rejection_sample_less_than_field_modulus(randomness: &[u8], out: &mut [i32]) -> usize {
        rejection_sample::less_than_field_modulus::sample(randomness, out)
    }

    #[inline(always)]
    fn rejection_sample_less_than_eta_equals_2(randomness: &[u8], out: &mut [i32]) -> usize {
        rejection_sample::less_than_eta::sample::<2>(randomness, out)
    }

    #[inline(always)]
    fn rejection_sample_less_than_eta_equals_4(randomness: &[u8], out: &mut [i32]) -> usize {
        rejection_sample::less_than_eta::sample::<4>(randomness, out)
    }

    #[inline(always)]
    fn gamma1_serialize(simd_unit: &Self, serialized: &mut [u8], gamma1_exponent: usize) {
        encoding::gamma1::serialize(&simd_unit.value, serialized, gamma1_exponent)
    }
    #[inline(always)]
    fn gamma1_deserialize(serialized: &[u8], out: &mut Self, gamma1_exponent: usize) {
        encoding::gamma1::deserialize(serialized, &mut out.value, gamma1_exponent);
    }

    #[inline(always)]
    fn commitment_serialize(simd_unit: &Self, serialized: &mut [u8]) {
        encoding::commitment::serialize(&simd_unit.value, serialized)
    }

    #[inline(always)]
    fn error_serialize(eta: Eta, simd_unit: &Self, serialized: &mut [u8]) {
        encoding::error::serialize(eta, &simd_unit.value, serialized)
    }

    #[inline(always)]
    fn error_deserialize(eta: Eta, serialized: &[u8], out: &mut Self) {
        encoding::error::deserialize(eta, serialized, &mut out.value);
    }

    #[inline(always)]
    fn t0_serialize(simd_unit: &Self, out: &mut [u8]) {
        // out len 13
        encoding::t0::serialize(&simd_unit.value, out);
    }
    #[inline(always)]
    fn t0_deserialize(serialized: &[u8], out: &mut Self) {
        encoding::t0::deserialize(serialized, &mut out.value);
    }

    #[inline(always)]
    fn t1_serialize(simd_unit: &Self, out: &mut [u8]) {
        encoding::t1::serialize(&simd_unit.value, out);
    }

    #[inline(always)]
    fn t1_deserialize(serialized: &[u8], out: &mut Self) {
        encoding::t1::deserialize(serialized, &mut out.value);
    }

    #[inline(always)]
    fn ntt(simd_units: &mut AVX2RingElement) {
        ntt::ntt(simd_units);
    }

    #[inline(always)]
    fn invert_ntt_montgomery(simd_units: &mut AVX2RingElement) {
        invntt::invert_ntt_montgomery(simd_units);
    }

    #[inline(always)]
    fn barrett_reduce_simd_unit(simd_unit: &mut Self) {
        barrett_reduce_simd_unit(&mut simd_unit.value);
    }
}

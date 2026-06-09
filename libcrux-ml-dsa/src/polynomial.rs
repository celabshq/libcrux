#[cfg(hax)]
use crate::simd::traits::specs::*;
use crate::simd::traits::{Operations, COEFFICIENTS_IN_SIMD_UNIT, SIMD_UNITS_IN_RING_ELEMENT};

#[derive(Clone, Copy)]
#[hax_lib::fstar::after("open Libcrux_ml_dsa.Simd.Traits.Specs")]
pub(crate) struct PolynomialRingElement<SIMDUnit: Operations> {
    pub(crate) simd_units: [SIMDUnit; SIMD_UNITS_IN_RING_ELEMENT],
}

#[hax_lib::attributes]
impl<SIMDUnit: Operations> PolynomialRingElement<SIMDUnit> {
    #[inline(always)]
    // Barrett reduce all coefficients.
    pub(crate) fn barrett_reduce(&mut self) {
        for i in 0..self.simd_units.len() {
            SIMDUnit::barrett_reduce_simd_unit(&mut self.simd_units[i]);
        }
    }

    pub(crate) fn zero() -> Self {
        Self {
            simd_units: [SIMDUnit::zero(); SIMD_UNITS_IN_RING_ELEMENT],
        }
    }

    // This is used in `make_hint` and for tests
    pub(crate) fn to_i32_array(&self) -> [i32; 256] {
        let mut result = [0i32; 256];

        for i in 0..self.simd_units.len() {
            SIMDUnit::to_coefficient_array(
                &self.simd_units[i],
                &mut result[i * COEFFICIENTS_IN_SIMD_UNIT..(i + 1) * COEFFICIENTS_IN_SIMD_UNIT],
            );
        }

        result
    }

    #[hax_lib::requires(array.len() == 256)]
    pub(crate) fn from_i32_array(array: &[i32], result: &mut Self) {
        for i in 0..SIMD_UNITS_IN_RING_ELEMENT {
            SIMDUnit::from_coefficient_array(
                &array[i * COEFFICIENTS_IN_SIMD_UNIT..(i + 1) * COEFFICIENTS_IN_SIMD_UNIT],
                &mut result.simd_units[i],
            );
        }
    }

    #[cfg(test)]
    pub(crate) fn from_i32_array_test(array: &[i32]) -> Self {
        let mut result = PolynomialRingElement::zero();
        Self::from_i32_array(array, &mut result);
        result
    }

    #[inline(always)]
    #[hax_lib::requires(fstar!(r#"v $bound > 0 /\ 
        (forall i. Spec.Utils.is_i32b_array_opaque 
            (v ${FIELD_MAX}) 
            (i0._super_i2.f_repr (Seq.index self.f_simd_units i)))"#))]
    /// CAUTION: This function must only be called with inputs for
    /// which it is safe to leak the index of a violating coefficient.
    ///
    /// For all norm checks during ML-DSA signature generation it is
    /// safe to leak the index of a violating coefficient.
    pub(crate) fn infinity_norm_exceeds(&self, bound: i32) -> bool {
        let mut result = false;
        for i in 0..self.simd_units.len() {
            let coeff_exceeds = SIMDUnit::infinity_norm_exceeds(&self.simd_units[i], bound);
            // Declassification: It is safe to leak the index of a
            // violating coefficient during ML-DSA signature
            // generation.
            //
            // See section 5.5 of the Dilithium Specification for
            // Round 3 of the NIST Post-Quantum Cryptography
            // Standardization.
            // (https://pq-crystals.org/dilithium/data/dilithium-specification-round3-20210208.pdf)
            libcrux_secrets::mem_requests::ct_declassify(&coeff_exceeds);
            // XXX: We can't use the non-short-circuiting
            // core::ops::BitOr here, because of an issue in hax-lib
            // v0.3.6.
            // (cf. https://github.com/cryspen/libcrux/issues/1437)
            //
            // Using the short-circuiting OR is safe, see comment on
            // declassification above.
            result = result || coeff_exceeds;
        }

        result
    }

    #[inline(always)]
    #[hax_lib::requires(fstar!(r#"forall i. 
        add_pre (i0._super_i2.f_repr (Seq.index self.f_simd_units i)) 
                (i0._super_i2.f_repr (Seq.index rhs.f_simd_units i))"#))]
    pub(crate) fn add(&mut self, rhs: &Self) {
        #[cfg(hax)]
        let old_self = self.clone();

        for i in 0..self.simd_units.len() {
            hax_lib::loop_invariant!(|i: usize| fstar!(
                r#"forall j. j >= v i ==> 
                            Seq.index self.f_simd_units j == 
                            Seq.index old_self.f_simd_units j"#
            ));

            SIMDUnit::add(&mut self.simd_units[i], &rhs.simd_units[i]);
        }
    }

    #[inline(always)]
    #[hax_lib::requires(fstar!(r#"forall i. 
        sub_pre (i0._super_i2.f_repr (Seq.index self.f_simd_units i)) 
                (i0._super_i2.f_repr (Seq.index rhs.f_simd_units i))"#))]
    pub(crate) fn subtract(&mut self, rhs: &Self) {
        #[cfg(hax)]
        let old_self = self.clone();

        for i in 0..self.simd_units.len() {
            hax_lib::loop_invariant!(|i: usize| fstar!(
                r#"forall j. j >= v i ==> 
                        Seq.index self.f_simd_units j == 
                        Seq.index old_self.f_simd_units j"#
            ));

            SIMDUnit::subtract(&mut self.simd_units[i], &rhs.simd_units[i]);
        }
    }
}

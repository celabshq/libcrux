use crate::simd::traits::COEFFICIENTS_IN_SIMD_UNIT;
/// Values having this type hold a representative 'x' of the ML-DSA field.
/// We use 'fe' as a shorthand for this type.
pub(crate) type FieldElement = i32;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Coefficients {
    pub(super) values: [FieldElement; COEFFICIENTS_IN_SIMD_UNIT],
}

#[inline(always)]
pub(crate) fn zero() -> Coefficients {
    Coefficients {
        values: [0i32; COEFFICIENTS_IN_SIMD_UNIT],
    }
}

#[inline(always)]
#[hax_lib::requires(array.len() == COEFFICIENTS_IN_SIMD_UNIT)]
pub(crate) fn from_coefficient_array(array: &[i32], out: &mut Coefficients) {
    out.values
        .copy_from_slice(&array[0..COEFFICIENTS_IN_SIMD_UNIT])
}

#[inline(always)]
#[hax_lib::requires(out.len() == COEFFICIENTS_IN_SIMD_UNIT)]
pub(crate) fn to_coefficient_array(
    value: &Coefficients,
    out: &mut [i32], // len: COEFFICIENTS_IN_SIMD_UNIT
) {
    out.copy_from_slice(&value.values);
}

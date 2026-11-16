use crate::{
    constants::{GAMMA2_V261_888, GAMMA2_V95_232},
    simd::traits::*,
};

fn test_decompose_generic<SIMDUnit: Operations>() {
    // When GAMMA2 = 95,232
    let mut input = SIMDUnit::zero();
    SIMDUnit::from_coefficient_array(
        &[
            5520769, 5416853, 180455, 8127421, 5159850, 5553986, 3391280, 3968290,
        ],
        &mut input,
    );

    let expected_low = [-2687, 83861, -10009, -62531, 17322, 30530, -37072, -31454];
    let expected_high = [29, 28, 1, 43, 27, 29, 18, 21];

    let (mut low, mut high) = (SIMDUnit::zero(), SIMDUnit::zero());
    SIMDUnit::decompose(GAMMA2_V95_232, &input, &mut low, &mut high);

    let mut out = [0i32; COEFFICIENTS_IN_SIMD_UNIT];
    SIMDUnit::to_coefficient_array(&low, &mut out);
    assert_eq!(out, expected_low);

    let mut out = [0i32; COEFFICIENTS_IN_SIMD_UNIT];
    SIMDUnit::to_coefficient_array(&high, &mut out);
    assert_eq!(out, expected_high);

    // When GAMMA2 = 261,888
    let mut input = SIMDUnit::zero();
    SIMDUnit::from_coefficient_array(
        &[
            2108939, 7162128, 6506792, 7957464, 2350341, 8333084, 496214, 2168929,
        ],
        &mut input,
    );

    let expected_low = [
        13835, -170736, 221480, 100824, 255237, -47333, -27562, 73825,
    ];
    let expected_high = [4, 14, 12, 15, 4, 0, 1, 4];

    SIMDUnit::decompose(GAMMA2_V261_888, &input, &mut low, &mut high);

    let mut out = [0i32; COEFFICIENTS_IN_SIMD_UNIT];
    SIMDUnit::to_coefficient_array(&low, &mut out);
    assert_eq!(out, expected_low);

    let mut out = [0i32; COEFFICIENTS_IN_SIMD_UNIT];
    SIMDUnit::to_coefficient_array(&high, &mut out);
    assert_eq!(out, expected_high);
}

fn test_power2round_generic<SIMDUnit: Operations>() {
    let mut input = SIMDUnit::zero();
    SIMDUnit::from_coefficient_array(
        &[
            6950677, 3362411, 5783989, 5909314, 6459529, 5751812, 864332, 3667708,
        ],
        &mut input,
    );

    let expected_low = [3861, 3691, 437, 2882, -3959, 1028, -4020, -2308];
    let expected_high = [848, 410, 706, 721, 789, 702, 106, 448];

    let mut high = SIMDUnit::zero();
    SIMDUnit::from_coefficient_array(&[0; 8], &mut high);
    SIMDUnit::power2round(&mut input, &mut high);
    let low = input;

    let mut out = [0i32; COEFFICIENTS_IN_SIMD_UNIT];
    SIMDUnit::to_coefficient_array(&low, &mut out);
    assert_eq!(out, expected_low);

    let mut out = [0i32; COEFFICIENTS_IN_SIMD_UNIT];
    SIMDUnit::to_coefficient_array(&high, &mut out);
    assert_eq!(out, expected_high);
}

#[cfg(not(feature = "simd256"))]
mod portable {
    use super::{test_decompose_generic, test_power2round_generic};

    #[test]
    fn test_decompose() {
        test_decompose_generic::<crate::simd::portable::PortableSIMDUnit>();
    }
    #[test]
    fn test_power2round() {
        test_power2round_generic::<crate::simd::portable::PortableSIMDUnit>();
    }
}

#[cfg(feature = "simd256")]
mod avx2 {
    use super::{test_decompose_generic, test_power2round_generic};

    #[test]
    fn test_decompose() {
        test_decompose_generic::<crate::simd::avx2::AVX2SIMDUnit>();
    }
    #[test]
    fn test_power2round() {
        test_power2round_generic::<crate::simd::avx2::AVX2SIMDUnit>();
    }
}

use crate::{helper::cloop, polynomial::PolynomialRingElement, simd::traits::Operations};

#[inline(always)]
fn serialize<SIMDUnit: Operations>(re: &PolynomialRingElement<SIMDUnit>, serialized: &mut [u8]) {
    let output_bytes_per_simd_unit = serialized.len() / (8 * 4);

    cloop! {
        for (i, simd_unit) in re.simd_units.iter().enumerate() {
            SIMDUnit::commitment_serialize(
                simd_unit,
                &mut serialized[i * output_bytes_per_simd_unit..(i + 1) * output_bytes_per_simd_unit],
            );
        }
    }
}

#[inline(always)]
pub(crate) fn serialize_vector<SIMDUnit: Operations>(
    ring_element_size: usize,
    vector: &[PolynomialRingElement<SIMDUnit>],
    serialized: &mut [u8],
) {
    let mut offset: usize = 0;

    cloop! {
        for ring_element in vector.iter() {
            serialize::<SIMDUnit>(ring_element, &mut serialized[offset..offset + ring_element_size]);
            offset += ring_element_size;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        polynomial::PolynomialRingElement,
        simd::{self, traits::Operations},
    };

    fn test_serialize_generic<SIMDUnit: Operations>() {
        // Test serialization when LOW_ORDER_ROUNDING_RANGE = 95_232
        let coefficients = [
            42, 38, 3, 37, 37, 40, 2, 36, 11, 43, 37, 40, 1, 39, 20, 41, 38, 24, 41, 32, 7, 10, 21,
            21, 25, 11, 21, 22, 33, 43, 8, 11, 20, 23, 24, 30, 22, 42, 11, 37, 31, 39, 9, 22, 27,
            14, 39, 11, 3, 17, 25, 17, 17, 20, 32, 43, 17, 20, 23, 2, 38, 19, 16, 14, 38, 34, 35,
            8, 39, 12, 9, 4, 4, 1, 21, 37, 22, 10, 20, 3, 36, 1, 42, 39, 18, 17, 3, 1, 38, 1, 5,
            20, 0, 21, 39, 20, 10, 42, 10, 26, 6, 22, 12, 1, 20, 1, 43, 37, 33, 37, 6, 24, 32, 8,
            42, 2, 32, 16, 13, 3, 33, 2, 0, 29, 4, 3, 23, 36, 6, 42, 1, 37, 7, 3, 12, 36, 19, 41,
            42, 20, 36, 12, 11, 39, 23, 35, 29, 9, 31, 11, 19, 11, 14, 1, 32, 5, 6, 31, 4, 30, 8,
            24, 22, 39, 8, 10, 26, 11, 25, 10, 36, 17, 43, 25, 20, 2, 37, 11, 21, 4, 24, 25, 5, 26,
            29, 39, 3, 10, 8, 15, 40, 28, 26, 4, 30, 42, 14, 17, 41, 27, 8, 19, 19, 0, 3, 5, 41,
            34, 39, 14, 1, 39, 9, 10, 41, 12, 24, 16, 2, 5, 33, 27, 27, 32, 4, 3, 9, 5, 37, 40, 38,
            43, 32, 27, 34, 27, 15, 24, 4, 2, 42, 15, 9, 3, 17, 35, 0, 22, 43, 13, 15, 6, 38, 10,
            20, 37,
        ];
        let re = PolynomialRingElement::<SIMDUnit>::from_i32_array_test(&coefficients);

        let serialized = [
            170, 57, 148, 37, 42, 144, 203, 90, 162, 193, 73, 165, 38, 150, 130, 135, 82, 85, 217,
            82, 89, 225, 138, 44, 212, 133, 121, 150, 186, 148, 223, 153, 88, 155, 115, 46, 67,
            148, 69, 17, 5, 174, 17, 117, 9, 230, 4, 57, 166, 56, 34, 39, 147, 16, 68, 80, 149,
            150, 66, 13, 100, 160, 158, 82, 52, 4, 102, 80, 80, 64, 117, 82, 138, 170, 104, 134,
            197, 4, 84, 176, 150, 97, 105, 96, 32, 162, 10, 32, 212, 12, 161, 0, 116, 196, 112,
            145, 134, 26, 148, 199, 192, 144, 83, 170, 82, 36, 179, 156, 215, 216, 37, 223, 50, 45,
            78, 0, 22, 198, 71, 120, 8, 102, 157, 136, 162, 45, 153, 66, 70, 107, 70, 9, 229, 82,
            17, 88, 86, 104, 221, 57, 40, 200, 131, 114, 26, 225, 169, 78, 148, 110, 200, 52, 1,
            67, 145, 138, 167, 19, 156, 137, 146, 50, 24, 36, 20, 225, 182, 129, 196, 144, 20, 37,
            106, 174, 224, 38, 110, 15, 70, 8, 234, 147, 12, 209, 8, 88, 107, 243, 24, 166, 66,
            149,
        ];

        let mut result = [0u8; 192];
        serialize::<SIMDUnit>(&re, &mut result);
        assert_eq!(result, serialized);

        // Test serialization when LOW_ORDER_ROUNDING_RANGE = 261,888
        let coefficients = [
            2, 4, 8, 3, 14, 3, 10, 7, 4, 15, 13, 3, 1, 2, 9, 12, 8, 11, 12, 4, 7, 14, 9, 4, 4, 2,
            5, 15, 14, 11, 6, 11, 10, 13, 3, 13, 9, 15, 10, 8, 14, 4, 8, 11, 11, 10, 13, 8, 4, 9,
            3, 8, 8, 3, 4, 5, 14, 9, 13, 12, 0, 4, 4, 2, 9, 11, 7, 11, 9, 14, 1, 7, 13, 12, 0, 15,
            14, 8, 6, 15, 15, 7, 11, 1, 11, 2, 4, 11, 10, 3, 15, 6, 7, 3, 1, 12, 0, 15, 7, 13, 13,
            1, 9, 14, 3, 5, 0, 8, 5, 7, 5, 8, 10, 13, 13, 11, 11, 13, 1, 4, 10, 14, 15, 14, 12, 6,
            13, 1, 7, 7, 15, 4, 2, 5, 6, 2, 7, 14, 2, 2, 4, 11, 7, 1, 5, 8, 9, 5, 4, 13, 8, 8, 13,
            13, 15, 5, 6, 11, 11, 4, 13, 7, 11, 15, 15, 3, 12, 4, 12, 14, 2, 6, 9, 10, 6, 13, 15,
            12, 11, 12, 2, 7, 6, 9, 9, 5, 6, 3, 4, 2, 8, 3, 10, 2, 8, 1, 13, 10, 12, 8, 14, 0, 5,
            12, 5, 3, 7, 15, 12, 13, 3, 4, 10, 1, 13, 3, 9, 6, 10, 13, 4, 4, 2, 9, 0, 4, 5, 7, 14,
            11, 2, 6, 3, 11, 6, 2, 0, 5, 8, 5, 9, 5, 9, 0, 2, 2, 3, 15, 0, 8, 11, 13, 2, 6, 11, 0,
        ];
        let re = PolynomialRingElement::<SIMDUnit>::from_i32_array_test(&coefficients);

        let serialized = [
            66, 56, 62, 122, 244, 61, 33, 201, 184, 76, 231, 73, 36, 245, 190, 182, 218, 211, 249,
            138, 78, 184, 171, 141, 148, 131, 56, 84, 158, 205, 64, 36, 185, 183, 233, 113, 205,
            240, 142, 246, 127, 27, 43, 180, 58, 111, 55, 193, 240, 215, 29, 233, 83, 128, 117,
            133, 218, 189, 219, 65, 234, 239, 108, 29, 119, 79, 82, 38, 231, 34, 180, 23, 133, 89,
            212, 136, 221, 95, 182, 75, 125, 251, 63, 76, 236, 98, 169, 214, 207, 203, 114, 150,
            89, 54, 36, 56, 42, 24, 173, 140, 14, 197, 53, 247, 220, 67, 26, 61, 105, 218, 68, 146,
            64, 117, 190, 98, 179, 38, 80, 88, 89, 9, 34, 243, 128, 219, 98, 11,
        ];

        let mut result = [0u8; 128];
        serialize::<SIMDUnit>(&re, &mut result);
        assert_eq!(result, serialized);
    }

    #[test]
    fn test_serialize_portable() {
        test_serialize_generic::<simd::portable::PortableSIMDUnit>();
    }

    #[cfg(feature = "simd256")]
    #[test]
    fn test_serialize_simd256() {
        test_serialize_generic::<simd::avx2::AVX2SIMDUnit>();
    }
}

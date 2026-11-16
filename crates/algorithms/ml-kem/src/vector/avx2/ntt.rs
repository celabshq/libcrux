use super::*;

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\ Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3"#)))]
pub(crate) fn ntt_layer_1_step(
    vector: Vec256,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> Vec256 {
    let zetas = mm256_set_epi16(
        -zeta3, -zeta3, zeta3, zeta3, -zeta2, -zeta2, zeta2, zeta2, -zeta1, -zeta1, zeta1, zeta1,
        -zeta0, -zeta0, zeta0, zeta0,
    );

    let rhs = mm256_shuffle_epi32::<0b11_11_01_01>(vector);
    let rhs = arithmetic::montgomery_multiply_by_constants(rhs, zetas);

    let lhs = mm256_shuffle_epi32::<0b10_10_00_00>(vector);

    mm256_add_epi16(lhs, rhs)
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1"#)))]
pub(crate) fn ntt_layer_2_step(vector: Vec256, zeta0: i16, zeta1: i16) -> Vec256 {
    let zetas = mm256_set_epi16(
        -zeta1, -zeta1, -zeta1, -zeta1, zeta1, zeta1, zeta1, zeta1, -zeta0, -zeta0, -zeta0, -zeta0,
        zeta0, zeta0, zeta0, zeta0,
    );

    let rhs = mm256_shuffle_epi32::<0b11_10_11_10>(vector);
    let rhs = arithmetic::montgomery_multiply_by_constants(rhs, zetas);

    let lhs = mm256_shuffle_epi32::<0b01_00_01_00>(vector);

    mm256_add_epi16(lhs, rhs)
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta"#)))]
pub(crate) fn ntt_layer_3_step(vector: Vec256, zeta: i16) -> Vec256 {
    let rhs = mm256_extracti128_si256::<1>(vector);
    let rhs = arithmetic::montgomery_multiply_m128i_by_constants(rhs, mm_set1_epi16(zeta));

    let lhs = mm256_castsi256_si128(vector);

    let lower_coefficients = mm_add_epi16(lhs, rhs);
    let upper_coefficients = mm_sub_epi16(lhs, rhs);

    let combined = mm256_castsi128_si256(lower_coefficients);
    let combined = mm256_inserti128_si256::<1>(combined, upper_coefficients);

    combined
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(lax))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
                            Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
                            Spec.Utils.is_i16b_array (4*3328) (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${vector})"#)))]
pub(crate) fn inv_ntt_layer_1_step(
    vector: Vec256,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> Vec256 {
    let lhs = mm256_shuffle_epi32::<0b11_11_01_01>(vector);

    let rhs = mm256_shuffle_epi32::<0b10_10_00_00>(vector);
    let rhs = mm256_mullo_epi16(
        rhs,
        mm256_set_epi16(-1, -1, 1, 1, -1, -1, 1, 1, -1, -1, 1, 1, -1, -1, 1, 1),
    );

    let sum = mm256_add_epi16(lhs, rhs);
    let sum_times_zetas = arithmetic::montgomery_multiply_by_constants(
        sum,
        mm256_set_epi16(
            zeta3, zeta3, 0, 0, zeta2, zeta2, 0, 0, zeta1, zeta1, 0, 0, zeta0, zeta0, 0, 0,
        ),
    );

    let sum = arithmetic::barrett_reduce(sum);

    mm256_blend_epi16::<0b1_1_0_0_1_1_0_0>(sum, sum_times_zetas)
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1"#)))]
pub(crate) fn inv_ntt_layer_2_step(vector: Vec256, zeta0: i16, zeta1: i16) -> Vec256 {
    let lhs = mm256_permute4x64_epi64::<0b11_11_01_01>(vector);

    let rhs = mm256_permute4x64_epi64::<0b10_10_00_00>(vector);
    let rhs = mm256_mullo_epi16(
        rhs,
        mm256_set_epi16(-1, -1, -1, -1, 1, 1, 1, 1, -1, -1, -1, -1, 1, 1, 1, 1),
    );

    let sum = mm256_add_epi16(lhs, rhs);
    let sum_times_zetas = arithmetic::montgomery_multiply_by_constants(
        sum,
        mm256_set_epi16(
            zeta1, zeta1, zeta1, zeta1, 0, 0, 0, 0, zeta0, zeta0, zeta0, zeta0, 0, 0, 0, 0,
        ),
    );

    mm256_blend_epi16::<0b1_1_1_1_0_0_0_0>(sum, sum_times_zetas)
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta"#)))]
pub(crate) fn inv_ntt_layer_3_step(vector: Vec256, zeta: i16) -> Vec256 {
    let lhs = mm256_extracti128_si256::<1>(vector);
    let rhs = mm256_castsi256_si128(vector);

    let lower_coefficients = mm_add_epi16(lhs, rhs);

    let upper_coefficients = mm_sub_epi16(lhs, rhs);
    let upper_coefficients =
        arithmetic::montgomery_multiply_m128i_by_constants(upper_coefficients, mm_set1_epi16(zeta));

    let combined = mm256_castsi128_si256(lower_coefficients);
    let combined = mm256_inserti128_si256::<1>(combined, upper_coefficients);

    combined
}

#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(lax))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\ Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3"#)))]
pub(crate) fn ntt_multiply(
    lhs: Vec256,
    rhs: Vec256,
    zeta0: i16,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
) -> Vec256 {
    // Compute the first term of the product
    let shuffle_with = mm256_set_epi8(
        15, 14, 11, 10, 7, 6, 3, 2, 13, 12, 9, 8, 5, 4, 1, 0, 15, 14, 11, 10, 7, 6, 3, 2, 13, 12,
        9, 8, 5, 4, 1, 0,
    );
    const PERMUTE_WITH: i32 = 0b11_01_10_00;

    // Prepare the left hand side
    let lhs_shuffled = mm256_shuffle_epi8(lhs, shuffle_with);
    let lhs_shuffled = mm256_permute4x64_epi64::<{ PERMUTE_WITH }>(lhs_shuffled);

    let lhs_evens = mm256_castsi256_si128(lhs_shuffled);
    let lhs_evens = mm256_cvtepi16_epi32(lhs_evens);

    let lhs_odds = mm256_extracti128_si256::<1>(lhs_shuffled);
    let lhs_odds = mm256_cvtepi16_epi32(lhs_odds);

    // Prepare the right hand side
    let rhs_shuffled = mm256_shuffle_epi8(rhs, shuffle_with);
    let rhs_shuffled = mm256_permute4x64_epi64::<{ PERMUTE_WITH }>(rhs_shuffled);

    let rhs_evens = mm256_castsi256_si128(rhs_shuffled);
    let rhs_evens = mm256_cvtepi16_epi32(rhs_evens);

    let rhs_odds = mm256_extracti128_si256::<1>(rhs_shuffled);
    let rhs_odds = mm256_cvtepi16_epi32(rhs_odds);

    // Start operating with them
    let left = mm256_mullo_epi32(lhs_evens, rhs_evens);

    let right = mm256_mullo_epi32(lhs_odds, rhs_odds);
    let right = arithmetic::montgomery_reduce_i32s(right);
    let right = mm256_mullo_epi32(
        right,
        mm256_set_epi32(
            -(zeta3 as i32),
            zeta3 as i32,
            -(zeta2 as i32),
            zeta2 as i32,
            -(zeta1 as i32),
            zeta1 as i32,
            -(zeta0 as i32),
            zeta0 as i32,
        ),
    );

    let products_left = mm256_add_epi32(left, right);
    let products_left = arithmetic::montgomery_reduce_i32s(products_left);

    // Compute the second term of the product
    let rhs_adjacent_swapped = mm256_shuffle_epi8(
        rhs,
        mm256_set_epi8(
            13, 12, 15, 14, 9, 8, 11, 10, 5, 4, 7, 6, 1, 0, 3, 2, 13, 12, 15, 14, 9, 8, 11, 10, 5,
            4, 7, 6, 1, 0, 3, 2,
        ),
    );
    let products_right = mm256_madd_epi16(lhs, rhs_adjacent_swapped);
    let products_right = arithmetic::montgomery_reduce_i32s(products_right);
    let products_right = mm256_slli_epi32::<16>(products_right);

    // Combine them into one vector
    mm256_blend_epi16::<0b1_0_1_0_1_0_1_0>(products_left, products_right)
}

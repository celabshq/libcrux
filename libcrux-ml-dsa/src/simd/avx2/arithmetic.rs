use libcrux_intrinsics::avx2::*;

use crate::{
    constants::{Gamma2, BITS_IN_LOWER_PART_OF_T, GAMMA2_V261_888, GAMMA2_V95_232},
    simd::traits::{FIELD_MODULUS, INVERSE_OF_MODULUS_MOD_MONTGOMERY_R},
};

#[inline]
#[hax_lib::fstar::before(r#"open Spec.Intrinsics"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(true)]
#[hax_lib::ensures(|result| fstar!(r#"
    forall i. if v (to_i32x8 $t i) < 0 
              then to_i32x8 $result i = to_i32x8 $t i +! $FIELD_MODULUS
              else to_i32x8 $result i = to_i32x8 $t i)) =
"#))]
fn to_unsigned_representatives_ret(t: &Vec256) -> Vec256 {
    hax_lib::fstar!("reveal_opaque_arithmetic_ops #i32_inttype)");

    let signs = mm256_srai_epi32::<31>(*t);
    let conditional_add_field_modulus = mm256_and_si256(signs, mm256_set1_epi32(FIELD_MODULUS));

    hax_lib::fstar!(r"logand_lemma $FIELD_MODULUS (mk_i32 0)");

    mm256_add_epi32(*t, conditional_add_field_modulus)
}

#[inline]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(true)]
#[hax_lib::ensures(|_| fstar!(r#"
    forall i. if v (to_i32x8 $t i) < 0 
              then to_i32x8 tt_future i = to_i32x8 $t i +! $FIELD_MODULUS
              else to_i32x8 tt_future i = to_i32x8 $t i)) =
"#))]
fn to_unsigned_representatives(t: &mut Vec256) {
    *t = to_unsigned_representatives_ret(t);
}

#[inline]
pub(super) fn add(lhs: &mut Vec256, rhs: &Vec256) {
    *lhs = mm256_add_epi32(*lhs, *rhs);
}

#[inline]
pub(super) fn subtract(lhs: &mut Vec256, rhs: &Vec256) {
    *lhs = mm256_sub_epi32(*lhs, *rhs)
}

// Not using inline always here regresses performance significantly.
#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|result| fstar!(r#"
    forall i. to_i32x8 ${result} i == 
              Spec.MLDSA.Math.mont_mul (to_i32x8 ${lhs} i) $constant
"#))]
pub(super) fn montgomery_multiply_by_constant(lhs: Vec256, constant: i32) -> Vec256 {
    hax_lib::fstar!("reveal_opaque (`%Spec.MLDSA.Math.mont_mul) (Spec.MLDSA.Math.mont_mul)");

    let rhs = mm256_set1_epi32(constant);
    let field_modulus = mm256_set1_epi32(FIELD_MODULUS);
    let inverse_of_modulus_mod_montgomery_r =
        mm256_set1_epi32(INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i32);

    let prod02 = mm256_mul_epi32(lhs, rhs);
    let prod13 = mm256_mul_epi32(
        mm256_shuffle_epi32::<0b11_11_01_01>(lhs),
        mm256_shuffle_epi32::<0b11_11_01_01>(rhs),
    );

    let k02 = mm256_mul_epi32(prod02, inverse_of_modulus_mod_montgomery_r);
    let k13 = mm256_mul_epi32(prod13, inverse_of_modulus_mod_montgomery_r);

    let c02 = mm256_mul_epi32(k02, field_modulus);
    let c13 = mm256_mul_epi32(k13, field_modulus);

    let res02 = mm256_sub_epi32(prod02, c02);
    let res13 = mm256_sub_epi32(prod13, c13);
    let res02_shifted = mm256_shuffle_epi32::<0b11_11_01_01>(res02);
    mm256_blend_epi32::<0b10101010>(res02_shifted, res13)
}

// Not using inline always here regresses performance significantly.
#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(
    hax_lib::eq(field_modulus, mm256_set1_epi32(FIELD_MODULUS)).and(hax_lib::eq(
        inverse_of_modulus_mod_montgomery_r,
        mm256_set1_epi32(INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i32),
    ))
)]
#[hax_lib::ensures(|_| fstar!(r#"
    forall i. to_i32x8 ${lhs}_future i == 
              Spec.MLDSA.Math.mont_mul (to_i32x8 ${lhs} i) (to_i32x8 ${rhs} i)
"#))]
pub(super) fn montgomery_multiply_aux(
    field_modulus: Vec256,
    inverse_of_modulus_mod_montgomery_r: Vec256,
    lhs: &mut Vec256,
    rhs: &Vec256,
) {
    hax_lib::fstar!("reveal_opaque (`%Spec.MLDSA.Math.mont_mul) (Spec.MLDSA.Math.mont_mul)");

    let prod02 = mm256_mul_epi32(*lhs, *rhs);
    let prod13 = mm256_mul_epi32(
        mm256_shuffle_epi32::<0b11_11_01_01>(*lhs),
        mm256_shuffle_epi32::<0b11_11_01_01>(*rhs),
    );
    let k02 = mm256_mul_epi32(prod02, inverse_of_modulus_mod_montgomery_r);
    let k13 = mm256_mul_epi32(prod13, inverse_of_modulus_mod_montgomery_r);

    let c02 = mm256_mul_epi32(k02, field_modulus);
    let c13 = mm256_mul_epi32(k13, field_modulus);

    let res02 = mm256_sub_epi32(prod02, c02);
    let res13 = mm256_sub_epi32(prod13, c13);
    let res02_shifted = mm256_shuffle_epi32::<0b11_11_01_01>(res02);
    *lhs = mm256_blend_epi32::<0b10101010>(res02_shifted, res13);
}

// Not using inline always here regresses performance significantly.
#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|_| fstar!(r#"
    forall i. to_i32x8 ${lhs}_future i == 
              Spec.MLDSA.Math.mont_mul (to_i32x8 ${lhs} i) (to_i32x8 ${rhs} i)
"#))]
pub(super) fn montgomery_multiply(lhs: &mut Vec256, rhs: &Vec256) {
    let field_modulus = mm256_set1_epi32(FIELD_MODULUS);
    let inverse_of_modulus_mod_montgomery_r =
        mm256_set1_epi32(INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i32);

    montgomery_multiply_aux(field_modulus, inverse_of_modulus_mod_montgomery_r, lhs, rhs);
}

#[inline]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"v $SHIFT_BY == 13"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    (forall i. to_i32x8 ${simd_unit}_future i ==
        Spec.MLDSA.Math.barrett_red (shift_left_opaque (to_i32x8 ${simd_unit} i) v_SHIFT_BY))"#))]
pub(super) fn shift_left_then_reduce<const SHIFT_BY: i32>(simd_unit: &mut Vec256) {
    hax_lib::fstar!("reveal_opaque (`%Spec.MLDSA.Math.barrett_red) (Spec.MLDSA.Math.barrett_red)");

    let mut shifted = mm256_slli_epi32::<SHIFT_BY>(*simd_unit);

    barrett_reduce_simd_unit(&mut shifted);
    *simd_unit = shifted;
}

#[inline]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|_| fstar!(r#"
    (forall i. to_i32x8 ${simd_unit}_future i ==
        Spec.MLDSA.Math.barrett_red (to_i32x8 ${simd_unit} i))"#))]
pub(super) fn barrett_reduce_simd_unit(simd_unit: &mut Vec256) {
    let quotient = mm256_add_epi32(*simd_unit, mm256_set1_epi32(1 << 22));
    let quotient = mm256_srai_epi32::<23>(quotient);

    let quotient_times_field_modulus =
        mm256_mullo_epi32(quotient, mm256_set1_epi32(FIELD_MODULUS as i32));

    *simd_unit = mm256_sub_epi32(*simd_unit, quotient_times_field_modulus);
}

#[inline]
#[hax_lib::fstar::options("--fuel 0 --ifuel 0 --z3rlimit 300")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"v $bound > 0 /\ 
    (forall i. Spec.Utils.is_i32b (v $FIELD_MODULUS - 1) (to_i32x8 ${simd_unit} i))"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    $result == false <==> 
        (forall i. Spec.Utils.is_i32b (v $bound - 1) (to_i32x8 ${simd_unit} i))"#))]
pub(super) fn infinity_norm_exceeds(simd_unit: &Vec256, bound: i32) -> bool {
    let absolute_values = mm256_abs_epi32(*simd_unit);

    // We will test if |simd_unit| > bound - 1, because if this is the case then
    // it follows that |simd_unit| >= bound
    let bound = mm256_set1_epi32(bound - 1);

    let compare_with_bound = mm256_cmpgt_epi32(absolute_values, bound);

    // If every lane of |result| is 0, all coefficients are <= bound - 1
    let result = mm256_testz_si256(compare_with_bound, compare_with_bound);

    hax_lib::fstar!(r"logand_lemma_forall #i32_inttype");

    result != 1
}

#[inline]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    forall i. Spec.Utils.is_i32b (v $FIELD_MODULUS - 1) (to_i32x8 $r0 i)"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    forall i. 
        let (t0, t1) = Spec.MLDSA.Math.power2round (v (to_i32x8 $r0 i)) in
        (to_i32x8 ${r0}_future i == mk_i32 t0 /\
         to_i32x8 ${r1}_future i == mk_i32 t1 /\
         Spec.Utils.is_i32b (pow2 (v $BITS_IN_LOWER_PART_OF_T - 1)) (to_i32x8 ${r0}_future i))"#))]
pub(super) fn power2round(r0: &mut Vec256, r1: &mut Vec256) {
    hax_lib::fstar!("reveal_opaque_arithmetic_ops #i32_inttype");

    to_unsigned_representatives(r0);

    *r1 = mm256_add_epi32(
        *r0,
        mm256_set1_epi32((1 << (BITS_IN_LOWER_PART_OF_T - 1)) - 1),
    );
    *r1 = mm256_srai_epi32::<{ BITS_IN_LOWER_PART_OF_T as i32 }>(*r1);

    let tmp = mm256_slli_epi32::<{ BITS_IN_LOWER_PART_OF_T as i32 }>(*r1);
    *r0 = mm256_sub_epi32(*r0, tmp);
}

// Not using inline always here regresses performance significantly.
#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"(v $gamma2 == v $GAMMA2_V261_888 \/ v $gamma2 == v $GAMMA2_V95_232) /\
    (forall i. Spec.Utils.is_i32b (v $FIELD_MODULUS - 1) (to_i32x8 $r i))"#))]
#[hax_lib::ensures(|(r0,r1)| fstar!(r#"
    forall i.
    let (r0_s, r1_s) = Spec.MLDSA.Math.decompose_spec $gamma2 (to_i32x8 $r i) in
    to_i32x8 ${r0}_future i = r0_s /\ 
    to_i32x8 ${r1}_future i = r1_s"#))]
pub(super) fn decompose(gamma2: Gamma2, r: &Vec256, r0: &mut Vec256, r1: &mut Vec256) {
    let r = to_unsigned_representatives_ret(r);

    let ceil_of_r_by_128 = mm256_add_epi32(r, mm256_set1_epi32(127));
    let ceil_of_r_by_128 = mm256_srai_epi32::<7>(ceil_of_r_by_128);

    match gamma2 {
        GAMMA2_V95_232 => {
            // We approximate 1 / 1488 as:
            // ⌊2²⁴ / 1488⌋ / 2²⁴ = 11,275 / 2²⁴
            let result = mm256_mullo_epi32(ceil_of_r_by_128, mm256_set1_epi32(11_275));
            let result = mm256_add_epi32(result, mm256_set1_epi32(1 << 23));
            let result = mm256_srai_epi32::<24>(result);

            // For the corner-case a₁ = (q-1)/α = 44, we have to set a₁=0.
            let mask = mm256_sub_epi32(mm256_set1_epi32(43), result);
            let mask = mm256_srai_epi32::<31>(mask);

            let not_result = mm256_xor_si256(result, mask);

            *r1 = mm256_and_si256(result, not_result);
        }

        GAMMA2_V261_888 => {
            // We approximate 1 / 4092 as:
            // ⌊2²² / 4092⌋ / 2²² = 1025 / 2²²
            let result = mm256_mullo_epi32(ceil_of_r_by_128, mm256_set1_epi32(1025));
            let result = mm256_add_epi32(result, mm256_set1_epi32(1 << 21));
            let result = mm256_srai_epi32::<22>(result);

            // For the corner-case a₁ = (q-1)/α = 16, we have to set a₁=0.
            *r1 = mm256_and_si256(result, mm256_set1_epi32(15));
        }

        _ => unreachable!(),
    }

    // In the corner-case, when we set a₁=0, we will incorrectly
    // have a₀ > (q-1)/2 and we'll need to subtract q.  As we
    // return a₀ + q, that comes down to adding q if a₀ < (q-1)/2.

    let alpha = gamma2 * 2;
    let r0_tmp = mm256_mullo_epi32(*r1, mm256_set1_epi32(alpha));
    let r0_tmp = mm256_sub_epi32(r, r0_tmp);

    let field_modulus_halved = mm256_set1_epi32((FIELD_MODULUS - 1) / 2);
    let mask = mm256_sub_epi32(field_modulus_halved, r0_tmp);
    let mask = mm256_srai_epi32::<31>(mask);

    let field_modulus_and_mask = mm256_and_si256(mask, mm256_set1_epi32(FIELD_MODULUS));

    *r0 = mm256_sub_epi32(r0_tmp, field_modulus_and_mask);
}

// Not using inline always here regresses performance significantly.
#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::verification_status(lax)]
pub(super) fn compute_hint(low: &Vec256, high: &Vec256, gamma2: i32, hint: &mut Vec256) -> usize {
    let minus_gamma2 = mm256_set1_epi32(-gamma2);
    let gamma2 = mm256_set1_epi32(gamma2);

    let low_within_bound = mm256_cmpgt_epi32(mm256_abs_epi32(*low), gamma2);
    let low_equals_minus_gamma2 = mm256_cmpeq_epi32(*low, minus_gamma2);

    // If a lane in |high| is 0, the corresponding output will be 0; the output
    // will have its most significant bit set to 1 otherwise.
    let low_equals_minus_gamma2_and_high_is_nonzero =
        mm256_sign_epi32(low_equals_minus_gamma2, *high);

    *hint = mm256_or_si256(
        low_within_bound,
        low_equals_minus_gamma2_and_high_is_nonzero,
    );

    let hints_mask = mm256_movemask_ps(mm256_castsi256_ps(*hint));
    *hint = mm256_and_si256(*hint, mm256_set1_epi32(0x1));

    hints_mask.count_ones() as usize
}

// Not using inline always here regresses performance significantly.
#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::verification_status(lax)]
pub(super) fn use_hint(gamma2: Gamma2, r: &Vec256, hint: &mut Vec256) {
    let (mut r0, mut r1) = (mm256_setzero_si256(), mm256_setzero_si256());
    decompose(gamma2, r, &mut r0, &mut r1);

    let all_zeros = mm256_setzero_si256();

    // If r0 is <=0, we have to subtract the hint, whereas if it is > 0,
    // we have to add the hint. We thus add signs to the hint vector accordingly:
    //
    // With this step, |negate_hints| will match |hint| in only those lanes in
    // which the corresponding r0 value is <= 0, and will be 0 elsewhere.
    let r0_gt_zero = mm256_cmpgt_epi32(r0, all_zeros);
    let negate_hints = vec256_blendv_epi32(*hint, all_zeros, r0_gt_zero);

    // If a lane in |negate_hints| is 1, it means the corresponding hint was 1,
    // and the lane value will be doubled. It will remain 0 otherwise.
    let negate_hints = mm256_slli_epi32::<1>(negate_hints);

    // Suppose |hints[0]| = 1, and |r0[0]| = 1, then this will set |hints[0]| = -1.
    // (we're indexing into an AVX2 vector, as it were).
    let hints = mm256_sub_epi32(*hint, negate_hints);

    // Now add the hints to r1
    let mut r1_plus_hints = mm256_add_epi32(r1, hints);

    match gamma2 {
        GAMMA2_V95_232 => {
            let max = mm256_set1_epi32(43);

            // If |r1_plus_hints[i]| is negative, it must be that |r1[i]| is
            // 0, in this case, we'd want to return |max|.
            r1_plus_hints = vec256_blendv_epi32(r1_plus_hints, max, r1_plus_hints);

            let greater_than_or_equal_to_max = mm256_cmpgt_epi32(r1_plus_hints, max);

            // If r1 is greater than equal to 43, we need to set the result to 0.
            *hint = vec256_blendv_epi32(r1_plus_hints, all_zeros, greater_than_or_equal_to_max);
        }
        GAMMA2_V261_888 => {
            *hint = mm256_and_si256(r1_plus_hints, mm256_set1_epi32(15));
        }

        _ => unreachable!(),
    }
}

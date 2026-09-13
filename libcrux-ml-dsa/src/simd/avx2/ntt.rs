use super::{arithmetic, AVX2RingElement, AVX2SIMDUnit};
use crate::simd::traits::COEFFICIENTS_IN_SIMD_UNIT;

use libcrux_intrinsics::avx2::*;

// Compute (a,b) ↦ (a + ζb, a - ζb) at layer 0 for 2 SIMD Units in one go.
#[inline(always)]
#[hax_lib::fstar::before(r"open Spec.MLDSA.NttConstants")]
#[hax_lib::fstar::before(r"open Spec.Intrinsics")]
// The theory for this file lives in the hand-written companion
// `Libcrux_ml_dsa.Simd.Avx2.Ntt_theory`.  The opens/alias below are NOT
// decoration: they establish exactly the scope the code beneath needs
// (F* module abbreviations are file-scoped and are not
// re-exported by `open`, and the Core_models/FStar.Mul re-opens take
// resolution precedence over the NttConstants/Intrinsics opens above).
#[hax_lib::fstar::before(
    r#"
open Core_models
open FStar.Mul
open Spec.MLDSA.Math
module C = Hacspec_ml_dsa.Commute.Chunk
open Libcrux_ml_dsa.Simd.Avx2.Ntt_theory
"#
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(index < 31)]
#[hax_lib::ensures(|_result| fstar!(r"
        Libcrux_ml_dsa.Simd.Avx2.Ntt_theory.butterfly_2_spec (Seq.index ${re} (v $index)).f_value
                         (Seq.index ${re} (v $index + 1)).f_value
                         $zeta_a0 $zeta_a1 $zeta_a2 $zeta_a3 $zeta_b0 $zeta_b1 $zeta_b2 $zeta_b3
                         (Seq.index ${re}_future (v $index)).f_value
                         (Seq.index ${re}_future (v $index + 1)).f_value /\
        Spec.Utils.modifies2_32 re ${re}_future $index ($index +! mk_int 1)
"))]
fn butterfly_2(
    re: &mut AVX2RingElement,
    index: usize,
    zeta_a0: i32,
    zeta_a1: i32,
    zeta_a2: i32,
    zeta_a3: i32,
    zeta_b0: i32,
    zeta_b1: i32,
    zeta_b2: i32,
    zeta_b3: i32,
) {
    // For proofs, the style that works best is to separate out the
    // stateful operations (reading and writing to mutable arrays)
    // from the core computation. So this and the following functions
    // have the pattern: read from array; compute; write to array.

    let re0 = re[index].value;
    let re1 = re[index + 1].value;

    // We shuffle the terms to group those that need to be multiplied
    // with zetas in the high QWORDS of the vectors, i.e. if the inputs are
    //   a = (a7, a6, a5, a4, a3, a2, a1, a0)
    //   b = (b7, b6, b5, b4, b3, b2, b1, b0)
    // after shuffling we have
    //   a_shuffled = ( a7, a5, a6, a4, a3, a1, a2, a0)
    //   b_shuffled = ( b7, b5, b6, b4, b3, b1, b2, b0)
    const SHUFFLE: i32 = 0b11_01_10_00;
    let a = mm256_shuffle_epi32::<SHUFFLE>(re0);
    let b = mm256_shuffle_epi32::<SHUFFLE>(re1);

    // Now we can use the same approach as for `butterfly_4`, only
    // zetas need to be adjusted.
    let summands = mm256_unpacklo_epi64(a, b);
    let mut zeta_products = mm256_unpackhi_epi64(a, b);
    let zetas = mm256_set_epi32(
        zeta_b3, zeta_b2, zeta_a3, zeta_a2, zeta_b1, zeta_b0, zeta_a1, zeta_a0,
    );

    arithmetic::montgomery_multiply(&mut zeta_products, &zetas);

    let sub_terms = mm256_sub_epi32(summands, zeta_products);
    let add_terms = mm256_add_epi32(summands, zeta_products);

    let a_terms_shuffled = mm256_unpacklo_epi64(add_terms, sub_terms);
    let b_terms_shuffled = mm256_unpackhi_epi64(add_terms, sub_terms);

    // Here, we undo the initial shuffle (it's self-inverse).
    let nre0 = mm256_shuffle_epi32::<SHUFFLE>(a_terms_shuffled);
    let nre1 = mm256_shuffle_epi32::<SHUFFLE>(b_terms_shuffled);

    // This assert allows all the SMT Patterns to kick in and prove correctness
    proof!(
        r#"assert (Libcrux_ml_dsa.Simd.Avx2.Ntt_theory.butterfly_2_spec 
                            $re0 $re1 $zeta_a0 $zeta_a1 $zeta_a2 $zeta_a3 
                            $zeta_b0 $zeta_b1 $zeta_b2 $zeta_b3 $nre0 $nre1)"#
    );

    re[index].value = nre0;
    re[index + 1].value = nre1;
}

// Compute (a,b) ↦ (a + ζb, a - ζb) at layer 1 for 2 SIMD Units in one go.
#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(index < 31)]
#[hax_lib::ensures(|_result| fstar!(r"
        Libcrux_ml_dsa.Simd.Avx2.Ntt_theory.butterfly_4_spec    (Seq.index ${re} (v $index)).f_value
                            (Seq.index ${re} (v $index + 1)).f_value
                            $zeta_a0 $zeta_a1 $zeta_b0 $zeta_b1
                            (Seq.index ${re}_future (v $index)).f_value
                            (Seq.index ${re}_future (v $index + 1)).f_value /\
        Spec.Utils.modifies2_32 $re ${re}_future $index ($index +! mk_int 1)
"))]
fn butterfly_4(
    re: &mut AVX2RingElement,
    index: usize,
    zeta_a0: i32,
    zeta_a1: i32,
    zeta_b0: i32,
    zeta_b1: i32,
) {
    let re0 = re[index].value;
    let re1 = re[index + 1].value;

    let summands = mm256_unpacklo_epi64(re0, re1);
    let mut zeta_products = mm256_unpackhi_epi64(re0, re1);

    let zetas = mm256_set_epi32(
        zeta_b1, zeta_b1, zeta_a1, zeta_a1, zeta_b0, zeta_b0, zeta_a0, zeta_a0,
    );
    arithmetic::montgomery_multiply(&mut zeta_products, &zetas);

    let sub_terms = mm256_sub_epi32(summands, zeta_products);
    let add_terms = mm256_add_epi32(summands, zeta_products);

    // Results are shuffled across the two SIMD registers.
    // We need to bring them in the right order.
    let nre0 = mm256_unpacklo_epi64(add_terms, sub_terms);
    let nre1 = mm256_unpackhi_epi64(add_terms, sub_terms);

    // This assert allows all the SMT Patterns to kick in and prove correctness
    proof!(
        r#"assert (Libcrux_ml_dsa.Simd.Avx2.Ntt_theory.butterfly_4_spec 
        $re0 $re1 $zeta_a0 $zeta_a1 $zeta_b0 $zeta_b1 $nre0 $nre1)"#
    );

    re[index].value = nre0;
    re[index + 1].value = nre1;
}

// Compute (a,b) ↦ (a + ζb, a - ζb) at layer 2 for 2 SIMD Units in one go.
#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(index < 31)]
#[hax_lib::ensures(|_result| fstar!(r"
        Libcrux_ml_dsa.Simd.Avx2.Ntt_theory.butterfly_8_spec    (Seq.index ${re} (v $index)).f_value
                            (Seq.index ${re} (v $index + 1)).f_value
                            $zeta0 $zeta1
                            (Seq.index ${re}_future (v $index)).f_value
                            (Seq.index ${re}_future (v $index + 1)).f_value /\
        Spec.Utils.modifies2_32 $re ${re}_future $index ($index +! mk_int 1)
"))]
fn butterfly_8(re: &mut AVX2RingElement, index: usize, zeta0: i32, zeta1: i32) {
    let re0 = re[index].value;
    let re1 = re[index + 1].value;

    let summands = mm256_set_m128i(mm256_castsi256_si128(re1), mm256_castsi256_si128(re0));
    let mut zeta_products = mm256_permute2x128_si256::<0b0001_0011>(re1, re0);

    let zetas = mm256_set_epi32(zeta1, zeta1, zeta1, zeta1, zeta0, zeta0, zeta0, zeta0);
    arithmetic::montgomery_multiply(&mut zeta_products, &zetas);

    let sub_terms = mm256_sub_epi32(summands, zeta_products);
    let add_terms = mm256_add_epi32(summands, zeta_products);

    let nre0 = mm256_set_m128i(
        mm256_castsi256_si128(sub_terms),
        mm256_castsi256_si128(add_terms),
    );
    let nre1 = mm256_permute2x128_si256::<0b0001_0011>(sub_terms, add_terms);

    // This assert allows all the SMT Patterns to kick in and prove correctness
    proof!(
        r#"assert (Libcrux_ml_dsa.Simd.Avx2.Ntt_theory.butterfly_8_spec 
         $re0 $re1 $zeta0 $zeta1 $nre0 $nre1)"#
    );

    re[index].value = nre0;
    re[index + 1].value = nre1;
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_poly_avx2 (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 7 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    is_i32b_poly_avx2 (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 8 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX) ${re}_future /\
    (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 ${re}) in
     let out_flat = C.simd_units_to_array (chunks_of_re_avx2 ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 0) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#))]
unsafe fn ntt_at_layer_0(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig_re = re.clone();

    butterfly_2(
        re, 0, 2091667, 3407706, 2316500, 3817976, -3342478, 2244091, -2446433, -3562462,
    );
    butterfly_2(
        re, 2, 266997, 2434439, -1235728, 3513181, -3520352, -3759364, -1197226, -3193378,
    );
    butterfly_2(
        re, 4, 900702, 1859098, 909542, 819034, 495491, -1613174, -43260, -522500,
    );
    butterfly_2(
        re, 6, -655327, -3122442, 2031748, 3207046, -3556995, -525098, -768622, -3595838,
    );
    butterfly_2(
        re, 8, 342297, 286988, -2437823, 4108315, 3437287, -3342277, 1735879, 203044,
    );
    butterfly_2(
        re, 10, 2842341, 2691481, -2590150, 1265009, 4055324, 1247620, 2486353, 1595974,
    );
    butterfly_2(
        re, 12, -3767016, 1250494, 2635921, -3548272, -2994039, 1869119, 1903435, -1050970,
    );
    butterfly_2(
        re, 14, -1333058, 1237275, -3318210, -1430225, -451100, 1312455, 3306115, -1962642,
    );
    butterfly_2(
        re, 16, -1279661, 1917081, -2546312, -1374803, 1500165, 777191, 2235880, 3406031,
    );
    butterfly_2(
        re, 18, -542412, -2831860, -1671176, -1846953, -2584293, -3724270, 594136, -3776993,
    );
    butterfly_2(
        re, 20, -2013608, 2432395, 2454455, -164721, 1957272, 3369112, 185531, -1207385,
    );
    butterfly_2(
        re, 22, -3183426, 162844, 1616392, 3014001, 810149, 1652634, -3694233, -1799107,
    );
    butterfly_2(
        re, 24, -3038916, 3523897, 3866901, 269760, 2213111, -975884, 1717735, 472078,
    );
    butterfly_2(
        re, 26, -426683, 1723600, -1803090, 1910376, -1667432, -1104333, -260646, -3833893,
    );
    butterfly_2(
        re, 28, -2939036, -2235985, -420899, -2286327, 183443, -976891, 1612842, -3545687,
    );
    butterfly_2(
        re, 30, -554416, 3919660, -48306, -1362209, 3937738, 1400424, -846154, 1976782,
    );

    proof!(
        r#"
assert (l0_post orig_re ${re});
lemma_l0post_to_sym orig_re ${re};
assert_norm (v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX == 8380416);
lemma_l0_full_avx2 orig_re ${re}
  (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 7 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX)
"#
    );
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_poly_avx2 (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 6 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    is_i32b_poly_avx2 (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 7 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX) ${re}_future /\
    (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 ${re}) in
     let out_flat = C.simd_units_to_array (chunks_of_re_avx2 ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 1) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#))]
unsafe fn ntt_at_layer_1(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig_re = re.clone();

    butterfly_4(re, 0, -3930395, -1528703, -3677745, -3041255);
    butterfly_4(re, 2, -1452451, 3475950, 2176455, -1585221);
    butterfly_4(re, 4, -1257611, 1939314, -4083598, -1000202);
    butterfly_4(re, 6, -3190144, -3157330, -3632928, 126922);
    butterfly_4(re, 8, 3412210, -983419, 2147896, 2715295);
    butterfly_4(re, 10, -2967645, -3693493, -411027, -2477047);
    butterfly_4(re, 12, -671102, -1228525, -22981, -1308169);
    butterfly_4(re, 14, -381987, 1349076, 1852771, -1430430);
    butterfly_4(re, 16, -3343383, 264944, 508951, 3097992);
    butterfly_4(re, 18, 44288, -1100098, 904516, 3958618);
    butterfly_4(re, 20, -3724342, -8578, 1653064, -3249728);
    butterfly_4(re, 22, 2389356, -210977, 759969, -1316856);
    butterfly_4(re, 24, 189548, -3553272, 3159746, -1851402);
    butterfly_4(re, 26, -2409325, -177440, 1315589, 1341330);
    butterfly_4(re, 28, 1285669, -1584928, -812732, -1439742);
    butterfly_4(re, 30, -3019102, -3881060, -3628969, 3839961);

    proof!(
        r#"
assert (l1_post orig_re ${re});
lemma_l1post_to_sym orig_re ${re};
assert_norm (v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX == 8380416);
lemma_l1_full_avx2 orig_re ${re}
  (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 6 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX)
"#
    );
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_poly_avx2 (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 5 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    is_i32b_poly_avx2 (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 6 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX) ${re}_future /\
    (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 ${re}) in
     let out_flat = C.simd_units_to_array (chunks_of_re_avx2 ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 2) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#))]
unsafe fn ntt_at_layer_2(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig_re = re.clone();

    butterfly_8(re, 0, 2706023, 95776);
    butterfly_8(re, 2, 3077325, 3530437);
    butterfly_8(re, 4, -1661693, -3592148);
    butterfly_8(re, 6, -2537516, 3915439);
    butterfly_8(re, 8, -3861115, -3043716);
    butterfly_8(re, 10, 3574422, -2867647);
    butterfly_8(re, 12, 3539968, -300467);
    butterfly_8(re, 14, 2348700, -539299);
    butterfly_8(re, 16, -1699267, -1643818);
    butterfly_8(re, 18, 3505694, -3821735);
    butterfly_8(re, 20, 3507263, -2140649);
    butterfly_8(re, 22, -1600420, 3699596);
    butterfly_8(re, 24, 811944, 531354);
    butterfly_8(re, 26, 954230, 3881043);
    butterfly_8(re, 28, 3900724, -2556880);
    butterfly_8(re, 30, 2071892, -2797779);

    proof!(
        r#"
assert (l2_post orig_re ${re});
lemma_l2post_to_sym orig_re ${re};
assert_norm (v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX == 8380416);
lemma_l2_full_avx2 orig_re ${re}
  (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 5 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX)
"#
    );
}

/// This is equivalent to the pqclean 0 and 1
///
/// This does 32 Montgomery multiplications (192 multiplications).
/// This is the same as in pqclean. The only difference is locality of registers.
// monolithic (NO --split_queries always): split clutters the comp_7_6_done sub-query
// with the L6/L7 value-foralls and saturates rlimit 400; monolithic lets Z3 discharge
// the opaque comp_7_6_done atom directly.
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 600"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Avx2NttTheory.is_i32b_poly_avx2 8380416 $re
"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Avx2NttTheory.comp_7_6_done $re ${re}_future /\
    Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 2*8380416) ${re}_future
"#))]
unsafe fn ntt_at_layer_7_and_6(re: &mut AVX2RingElement) {
    let field_modulus = mm256_set1_epi32(crate::simd::traits::FIELD_MODULUS);
    let inverse_of_modulus_mod_montgomery_r =
        mm256_set1_epi32(crate::simd::traits::INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i32);

    #[inline(always)]
    #[hax_lib::fstar::options(r#"--fuel 1 --ifuel 1 --z3rlimit 300"#)]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires({
        use crate::constants::FIELD_MODULUS;
        use crate::simd::traits::INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;
        use hax_lib::int::{ToInt, int};
        hax_lib::eq(field_modulus, mm256_set1_epi32(FIELD_MODULUS)).and(
            hax_lib::eq(inverse_of_modulus_mod_montgomery_r, mm256_set1_epi32(INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i32))
        ).and(index.to_int() + step_by.to_int() < int!(32)).and(step_by > 0)
    })]
    #[hax_lib::ensures(|result| fstar!(r#"
        let m = Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply
                  (Seq.index $re (v $index + v $step_by)).f_value $zeta in
        Spec.Utils.modifies2_32 $re ${re}_future $index ($index +! $step_by) /\
        Seq.index ${re}_future (v $index) ==
          ({ Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value =
               Libcrux_intrinsics.Avx2.mm256_add_epi32 (Seq.index $re (v $index)).f_value m }
           <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) /\
        Seq.index ${re}_future (v $index + v $step_by) ==
          ({ Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value =
               Libcrux_intrinsics.Avx2.mm256_sub_epi32 (Seq.index $re (v $index)).f_value m }
           <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
    "#))]
    fn mul(
        re: &mut AVX2RingElement,
        index: usize,
        zeta: Vec256,
        step_by: usize,
        field_modulus: Vec256,
        inverse_of_modulus_mod_montgomery_r: Vec256,
    ) {
        proof!(
            r#"
        reveal_opaque (`%Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply)
          (Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply);
        reveal_opaque (`%Libcrux_ml_dsa.Simd.Avx2.Arithmetic.add)
          (Libcrux_ml_dsa.Simd.Avx2.Arithmetic.add);
        reveal_opaque (`%Libcrux_ml_dsa.Simd.Avx2.Arithmetic.subtract)
          (Libcrux_ml_dsa.Simd.Avx2.Arithmetic.subtract)
    "#
        );
        let mut t = re[index + step_by].value;
        arithmetic::montgomery_multiply_aux(
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
            &mut t,
            &zeta,
        );
        re[index + step_by].value = re[index].value;
        arithmetic::subtract(&mut re[index + step_by].value, &t);
        arithmetic::add(&mut re[index].value, &t);
    }

    // Note: For proofs, it is better to use concrete constants instead of const expressions
    const STEP_BY_7: usize = 16; //2 * COEFFICIENTS_IN_SIMD_UNIT;
    const STEP_BY_6: usize = 8; //(1 << 6) / COEFFICIENTS_IN_SIMD_UNIT;

    // Per-pair REAL Montgomery butterfly: wraps `mul`; proves modifies + the two
    // output bounds + the bf_pair VALUE (via Avx2NttTheory.lemma_bf_pair_def).
    // Runtime-identical (#[inline(always)]).
    #[inline(always)]
    // The q76 standalone drivers here are deliberately host-side copies
    // (of Avx2NttTheory helpers) with #restart-solver: relocating them would shift
    // the solver-state placement the L7/6 composer needs.
    // proof-residence: clean-context
    #[hax_lib::fstar::before(
        r#"
(* ============================================================================
   IN-BODY 7_6: prove comp_7_6_done OVER THE REAL 32 Montgomery
   butterflies (ntt_at_layer_7_and_6___mul), no fstar::replace, mirroring
   Avx2NttTheory.build_out_impl + ntt_at_layer_7_and_6_interleaved_o in-body.

   Local proof scaffolding (NOT impl): q76_quadpost (abstract restatement of
   `quad`'s ensures) + two pure bf_pair-congruence substitution lemmas + the
   L6-value driver + the 32-unit bound driver, all copied from the theory's
   .fst-internal helpers (they only consume the abstract .fsti atoms bf_pair /
   is_i32b_unit_avx2, so they live legally in the consumer).  Then the real
   per-pair `mul_bf` (wraps ___mul) and the 4-pair `quad` (= Avx2NttTheory.quad's
   contract, real muls).  bf_pair value comes from the ADDITIVE exposure
   Avx2NttTheory.lemma_bf_pair_def. ============================================ *)

(* abstract restatement of Avx2NttTheory.quad's ensures (the 3 foralls). *)
unfold let q76_quadpost (re re_f: Avx2NttTheory.av32)
                        (base:nat) (sb:nat{sb > 0 /\ base + 4 + sb <= 32}) (bnd:nat) (zeta:i32) : Type0 =
  (forall (k:nat). k < 32 /\
      ~((k >= base /\ k < base + 4) \/ (k >= base + sb /\ k < base + 4 + sb))
      ==> Seq.index re_f k == Seq.index re k) /\
  (forall (k:nat). (k >= base /\ k < base + 4) \/ (k >= base + sb /\ k < base + 4 + sb) ==>
      Avx2NttTheory.is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_f k)) /\
  (forall (q:nat). q >= base /\ q < base + 4 ==>
      (let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index re q) (Seq.index re (q + sb)) zeta in
       Seq.index re_f q == nu0 /\ Seq.index re_f (q + sb) == nu1))

(* an L7-quad output array `a` at lo-unit q in [base,base+4) carries fst(bf_pair src[q] src[q+16] z1);
   if src agrees with orig at q,q+16 and mid==bf_pair-of-orig there, then a==mid there.  Pure
   bf_pair determinism (congruence), abstract bf_pair OK. *)
#restart-solver
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let q76_l7out_eq_mid (orig mid src a: Avx2NttTheory.av32) (q:nat)
    : Lemma
      (requires
        q < 16 /\
        Seq.index src q     == Seq.index orig q /\
        Seq.index src (q+16) == Seq.index orig (q+16) /\
        (let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index src q) (Seq.index src (q+16)) (mk_i32 (zeta_r 1)) in
         Seq.index a q == nu0 /\ Seq.index a (q+16) == nu1) /\
        (let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index orig q) (Seq.index orig (q+16)) (mk_i32 (zeta_r 1)) in
         Seq.index mid q == nu0 /\ Seq.index mid (q+16) == nu1))
      (ensures Seq.index a q == Seq.index mid q /\ Seq.index a (q+16) == Seq.index mid (q+16)) = ()
#pop-options

(* a quad's bf_pair value-post is about its INPUT `inp`; if `inp` agrees with `mid` on the read
   pair (q,q+8), the value-post is equivalently about `mid`.  Pure substitution. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let q76_l6_pair_subst_mid (inp mid out_q: Avx2NttTheory.av32) (q:nat{q+8 < 32}) (zL6:i32)
    : Lemma
      (requires
        Seq.index inp q == Seq.index mid q /\
        Seq.index inp (q+8) == Seq.index mid (q+8) /\
        (let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index inp q) (Seq.index inp (q+8)) zL6 in
         Seq.index out_q q == nu0 /\ Seq.index out_q (q+8) == nu1))
      (ensures
        (let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index mid q) (Seq.index mid (q+8)) zL6 in
         Seq.index out_q q == nu0 /\ Seq.index out_q (q+8) == nu1)) = ()
#pop-options

(* STANDALONE L6-value driver (clean context).  Preconditions = the 8 quad posts (as
   q76_quadpost abstract foralls, in IMPL order) + the L7 (`mid`) bf_pair-of-orig relation.
   Discharges out's bf_pair-of-MID per-L6-lo-unit post.  Copied from
   Avx2NttTheory.lemma_build_out_l6_value. *)
#restart-solver
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let q76_l6_value
      (orig mid qa0 qa1 qb0 qb1 qc0 qc1 qd0 out: Avx2NttTheory.av32)
    : Lemma
      (requires
        (let z1 = mk_i32 (zeta_r 1) in let z2 = mk_i32 (zeta_r 2) in let z3 = mk_i32 (zeta_r 3) in
         (forall (u:nat). u < 16 ==>
            (let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index orig u) (Seq.index orig (u+16)) z1 in
             Seq.index mid u == nu0 /\ Seq.index mid (u+16) == nu1)) /\
         q76_quadpost orig qa0 0  16 8380416            z1 /\
         q76_quadpost qa0  qa1 8  16 8380416            z1 /\
         q76_quadpost qa1  qb0 0  8  (8380416+8380416)  z2 /\
         q76_quadpost qb0  qb1 16 8  (8380416+8380416)  z3 /\
         q76_quadpost qb1  qc0 4  16 8380416            z1 /\
         q76_quadpost qc0  qc1 12 16 8380416            z1 /\
         q76_quadpost qc1  qd0 4  8  (8380416+8380416)  z2 /\
         q76_quadpost qd0  out 20 8  (8380416+8380416)  z3))
      (ensures
        (forall (u:nat). (u % 16 < 8) /\ u < 32 ==>
           (let zL6 = (if u < 16 then mk_i32 (zeta_r 2) else mk_i32 (zeta_r 3)) in
            let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index mid u) (Seq.index mid (u+8)) zL6 in
            Seq.index out u == nu0 /\ Seq.index out (u+8) == nu1)))
  = let z1 : i32 = mk_i32 (zeta_r 1) in
    let z2 : i32 = mk_i32 (zeta_r 2) in
    let z3 : i32 = mk_i32 (zeta_r 3) in
    assert_norm (zeta_r 1 == 25847);
    assert_norm (zeta_r 2 == (-2608894));
    assert_norm (zeta_r 3 == (-518909));
    let aux (u:nat{u<32}) : Lemma
        ((u % 16 < 8) ==>
           (let zL6 = (if u < 16 then z2 else z3) in
            let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index mid u) (Seq.index mid (u+8)) zL6 in
            Seq.index out u == nu0 /\ Seq.index out (u+8) == nu1))
      = if (u % 16 < 8) then begin
          if u < 4 then begin
            q76_l7out_eq_mid orig mid orig qa0 u;
            q76_l7out_eq_mid orig mid qa0  qa1 (u+8);
            assert (Seq.index qa1 u == Seq.index mid u);
            q76_l6_pair_subst_mid qa1 mid qb0 u z2;
            assert (Seq.index out u == Seq.index qb0 u);
            assert (Seq.index out (u+8) == Seq.index qb0 (u+8))
          end
          else if u < 8 then begin
            q76_l7out_eq_mid orig mid qb1 qc0 u;
            q76_l7out_eq_mid orig mid qc0 qc1 (u+8);
            assert (Seq.index qc1 u == Seq.index mid u);
            q76_l6_pair_subst_mid qc1 mid qd0 u z2;
            assert (Seq.index out u == Seq.index qd0 u);
            assert (Seq.index out (u+8) == Seq.index qd0 (u+8))
          end
          else if u < 20 then begin
            q76_l7out_eq_mid orig mid orig qa0 (u-16);
            q76_l7out_eq_mid orig mid qa0  qa1 (u-8);
            assert (Seq.index qb0 u == Seq.index mid u);
            assert (Seq.index qb0 (u+8) == Seq.index mid (u+8));
            q76_l6_pair_subst_mid qb0 mid qb1 u z3;
            assert (Seq.index out u == Seq.index qb1 u);
            assert (Seq.index out (u+8) == Seq.index qb1 (u+8))
          end
          else begin
            q76_l7out_eq_mid orig mid qb1 qc0 (u-16);
            q76_l7out_eq_mid orig mid qc0 qc1 (u-8);
            assert (Seq.index qd0 u == Seq.index mid u);
            assert (Seq.index qd0 (u+8) == Seq.index mid (u+8));
            q76_l6_pair_subst_mid qd0 mid out u z3
          end
        end
    in
    Classical.forall_intro aux
#pop-options

(* STANDALONE 32-unit bound driver (clean context).  Each unit's LAST touch is its L6 quad
   (qb0/qb1/qd0/out), bounding it to NTT_BASE+2; later quads frame it.  From the 8 quad posts. *)
#restart-solver
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let q76_bound
      (orig qa0 qa1 qb0 qb1 qc0 qc1 qd0 out: Avx2NttTheory.av32)
    : Lemma
      (requires
        (let z1 = mk_i32 (zeta_r 1) in let z2 = mk_i32 (zeta_r 2) in let z3 = mk_i32 (zeta_r 3) in
         q76_quadpost orig qa0 0  16 8380416            z1 /\
         q76_quadpost qa0  qa1 8  16 8380416            z1 /\
         q76_quadpost qa1  qb0 0  8  (8380416+8380416)  z2 /\
         q76_quadpost qb0  qb1 16 8  (8380416+8380416)  z3 /\
         q76_quadpost qb1  qc0 4  16 8380416            z1 /\
         q76_quadpost qc0  qc1 12 16 8380416            z1 /\
         q76_quadpost qc1  qd0 4  8  (8380416+8380416)  z2 /\
         q76_quadpost qd0  out 20 8  (8380416+8380416)  z3))
      (ensures
        (forall (k:nat). k < 32 ==>
           Avx2NttTheory.is_i32b_unit_avx2 (8380416 + 2*8380416) (Seq.index out k)))
  = let nb2 : nat = 8380416 + 2*8380416 in
    let aux (k:nat{k<32}) : Lemma
        (Avx2NttTheory.is_i32b_unit_avx2 nb2 (Seq.index out k))
      = if k < 4 || (k >= 8 && k < 12) then begin
          // bounded nb2 by qb0 (quad qa1 0 8); framed through qb1,qc0,qc1,qd0,out (all disjoint from k)
          assert (Avx2NttTheory.is_i32b_unit_avx2 nb2 (Seq.index qb0 k));
          assert (Seq.index out k == Seq.index qd0 k);
          assert (Seq.index qd0 k == Seq.index qc1 k);
          assert (Seq.index qc1 k == Seq.index qc0 k);
          assert (Seq.index qc0 k == Seq.index qb1 k);
          assert (Seq.index qb1 k == Seq.index qb0 k)
        end
        else if (k >= 16 && k < 20) || (k >= 24 && k < 28) then begin
          // bounded nb2 by qb1 (quad qb0 16 8); framed through qc0,qc1,qd0,out
          assert (Avx2NttTheory.is_i32b_unit_avx2 nb2 (Seq.index qb1 k));
          assert (Seq.index out k == Seq.index qd0 k);
          assert (Seq.index qd0 k == Seq.index qc1 k);
          assert (Seq.index qc1 k == Seq.index qc0 k);
          assert (Seq.index qc0 k == Seq.index qb1 k)
        end
        else if (k >= 4 && k < 8) || (k >= 12 && k < 16) then begin
          // bounded nb2 by qd0 (quad qc1 4 8); framed through out
          assert (Avx2NttTheory.is_i32b_unit_avx2 nb2 (Seq.index qd0 k));
          assert (Seq.index out k == Seq.index qd0 k)
        end
        else
          // {20-23,28-31}: bounded nb2 directly by out (quad qd0 20 8, last)
          assert (Avx2NttTheory.is_i32b_unit_avx2 nb2 (Seq.index out k))
    in
    Classical.forall_intro aux
#pop-options
"#
    )]
    #[hax_lib::fstar::before(r#"#restart-solver"#)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"#)]
    #[hax_lib::requires(fstar!(r#"
        v $bnd + 8380416 < pow2 31 /\
        v $step_by > 0 /\
        v $index + v $step_by < 32 /\
        Spec.Utils.is_i32b 4190208 $zeta /\
        $zeta_v == Libcrux_intrinsics.Avx2.mm256_set1_epi32 $zeta /\
        $field_modulus ==
          Libcrux_intrinsics.Avx2.mm256_set1_epi32 Libcrux_ml_dsa.Constants.v_FIELD_MODULUS /\
        $inverse_of_modulus_mod_montgomery_r ==
          Libcrux_intrinsics.Avx2.mm256_set1_epi32
            (cast (Libcrux_ml_dsa.Simd.Traits.v_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R <: u64) <: i32) /\
        Avx2NttTheory.is_i32b_unit_avx2 (v $bnd) (Seq.index $re (v $index)) /\
        Avx2NttTheory.is_i32b_unit_avx2 (v $bnd) (Seq.index $re (v $index + v $step_by))
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        Spec.Utils.modifies2_32 $re ${re}_future $index ($index +! $step_by) /\
        Avx2NttTheory.is_i32b_unit_avx2 (v $bnd + 8380416) (Seq.index ${re}_future (v $index)) /\
        Avx2NttTheory.is_i32b_unit_avx2 (v $bnd + 8380416) (Seq.index ${re}_future (v $index + v $step_by)) /\
        (let (nu0, nu1) =
              Avx2NttTheory.bf_pair (Seq.index $re (v $index)) (Seq.index $re (v $index + v $step_by)) $zeta in
         Seq.index ${re}_future (v $index) == nu0 /\ Seq.index ${re}_future (v $index + v $step_by) == nu1)
    "#))]
    fn mul_bf(
        re: &mut AVX2RingElement,
        index: usize,
        step_by: usize,
        zeta_v: Vec256,
        zeta: i32,
        bnd: usize,
        field_modulus: Vec256,
        inverse_of_modulus_mod_montgomery_r: Vec256,
    ) {
        #[cfg(hax)]
        let orig = re.clone();
        mul(
            re,
            index,
            zeta_v,
            step_by,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        // bf_pair VALUE: `mul` gives re[index]={f_value=add(orig[index].f_value, m)},
        //   re[index+sb]={f_value=sub(orig[index].f_value, m)}, m = montgomery_multiply orig[index+sb].f_value zeta_v;
        //   lemma_bf_pair_def gives bf_pair's value with m'=montgomery_multiply orig[index+sb].f_value (mm256_set1_epi32 zeta);
        //   zeta_v == mm256_set1_epi32 zeta -> m == m' -> equal.
        // BOUNDS: ntt_step forall8 (from `mul`'s add/sub form) -> lemma_cross_pair_relations_ws -> intro.
        proof!(
            r#"
        Avx2NttTheory.lemma_bf_pair_def (Seq.index orig (v $index)) (Seq.index orig (v $index + v $step_by)) $zeta;
        let re0 = (Seq.index orig (v $index)).f_value in
        let re1 = (Seq.index orig (v $index + v $step_by)).f_value in
        let nre0 = (Seq.index ${re} (v $index)).f_value in
        let nre1 = (Seq.index ${re} (v $index + v $step_by)).f_value in
        let lane (i:nat{i<8}) : Lemma
            ((to_i32x8 nre0 (mk_u64 i), to_i32x8 nre1 (mk_u64 i)) ==
             ntt_step (to_i32x8 $zeta_v (mk_int i))
               (to_i32x8 re0 (mk_u64 i), to_i32x8 re1 (mk_u64 i)))
          = assert (to_i32x8 $zeta_v (mk_int i) == $zeta);
            assert (to_i32x8 $zeta_v (mk_u64 i) == $zeta)
        in
        lane 0; lane 1; lane 2; lane 3; lane 4; lane 5; lane 6; lane 7;
        introduce forall (l:nat). l < 8 ==> to_i32x8 $zeta_v (mk_int l) == $zeta
        with (introduce l < 8 ==> to_i32x8 $zeta_v (mk_int l) == $zeta
              with _. assert (to_i32x8 $zeta_v (mk_int l) == $zeta));
        Avx2NttTheory.lemma_cross_pair_relations_ws orig ${re} (v $bnd) (v $index) (v $index + v $step_by) $zeta_v $zeta;
        Avx2NttTheory.lemma_is_i32b_unit_avx2_intro (v $bnd + 8380416) (Seq.index ${re} (v $index));
        Avx2NttTheory.lemma_is_i32b_unit_avx2_intro (v $bnd + 8380416) (Seq.index ${re} (v $index + v $step_by))
    "#
        );
    }

    // 4-pair REAL quad (= Avx2NttTheory.quad's contract; body = 4 real mul_bf).
    // lo-units base..base+3 paired with +sb.
    #[inline(always)]
    #[hax_lib::fstar::before(r#"#restart-solver"#)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"#)]
    #[hax_lib::requires(fstar!(r#"
        v $bnd + 8380416 < pow2 31 /\
        v $sb > 0 /\ v $base + 3 + v $sb < 32 /\ v $base + 4 <= v $base + v $sb /\
        Spec.Utils.is_i32b 4190208 $zeta /\
        $zeta_v == Libcrux_intrinsics.Avx2.mm256_set1_epi32 $zeta /\
        $field_modulus ==
          Libcrux_intrinsics.Avx2.mm256_set1_epi32 Libcrux_ml_dsa.Constants.v_FIELD_MODULUS /\
        $inverse_of_modulus_mod_montgomery_r ==
          Libcrux_intrinsics.Avx2.mm256_set1_epi32
            (cast (Libcrux_ml_dsa.Simd.Traits.v_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R <: u64) <: i32) /\
        (forall (k:nat). (k >= v $base /\ k < v $base + 4) \/
                         (k >= v $base + v $sb /\ k < v $base + 4 + v $sb) ==>
                         Avx2NttTheory.is_i32b_unit_avx2 (v $bnd) (Seq.index $re k))
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        q76_quadpost $re ${re}_future (v $base) (v $sb) (v $bnd) $zeta
    "#))]
    fn quad(
        re: &mut AVX2RingElement,
        base: usize,
        sb: usize,
        zeta_v: Vec256,
        zeta: i32,
        bnd: usize,
        field_modulus: Vec256,
        inverse_of_modulus_mod_montgomery_r: Vec256,
    ) {
        mul_bf(
            re,
            base,
            sb,
            zeta_v,
            zeta,
            bnd,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        mul_bf(
            re,
            base + 1,
            sb,
            zeta_v,
            zeta,
            bnd,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        mul_bf(
            re,
            base + 2,
            sb,
            zeta_v,
            zeta,
            bnd,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        mul_bf(
            re,
            base + 3,
            sb,
            zeta_v,
            zeta,
            bnd,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
    }

    // Factored 8-quad build-out (mirror of Avx2NttTheory.build_out_impl, but over the
    // REAL `quad` muls): the impl-order 8 quads (= the 32 `mul` butterflies) + the
    // standalone bound + L6-value drivers.  Kept SEPARATE so the 7_6 fn's VC stays
    // lean (just the layer-atom composition).
    #[inline(always)]
    #[hax_lib::fstar::before(r#"#restart-solver"#)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"#)]
    #[hax_lib::requires(fstar!(r#"
        Avx2NttTheory.is_i32b_poly_avx2 8380416 $re /\
        $field_modulus ==
          Libcrux_intrinsics.Avx2.mm256_set1_epi32 Libcrux_ml_dsa.Constants.v_FIELD_MODULUS /\
        $inverse_of_modulus_mod_montgomery_r ==
          Libcrux_intrinsics.Avx2.mm256_set1_epi32
            (cast (Libcrux_ml_dsa.Simd.Traits.v_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R <: u64) <: i32)
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        (let mid = Avx2NttTheory.build_mid_L7 $re in
         Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 2*8380416) ${re}_future /\
         (forall (u:nat). (u % 16 < 8) /\ u < 32 ==>
            (let zL6 = (if u < 16 then mk_i32 (zeta_r 2) else mk_i32 (zeta_r 3)) in
             let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index mid u) (Seq.index mid (u+8)) zL6 in
             Seq.index ${re}_future u == nu0 /\ Seq.index ${re}_future (u+8) == nu1)))
    "#))]
    fn build_out(
        re: &mut AVX2RingElement,
        field_modulus: Vec256,
        inverse_of_modulus_mod_montgomery_r: Vec256,
    ) {
        let zeta7 = mm256_set1_epi32(25847);
        let zeta60 = mm256_set1_epi32(-2608894);
        let zeta61 = mm256_set1_epi32(-518909);
        #[cfg(hax)]
        let orig = re.clone();
        proof!(
            r#"
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 1 == 25847);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 2 == (-2608894));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 3 == (-518909));
        Avx2NttTheory.lemma_poly_to_units 8380416 orig
    "#
        );
        quad(
            re,
            0,
            STEP_BY_7,
            zeta7,
            25847,
            8380416,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qa0 = re.clone();
        quad(
            re,
            8,
            STEP_BY_7,
            zeta7,
            25847,
            8380416,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qa1 = re.clone();
        quad(
            re,
            0,
            STEP_BY_6,
            zeta60,
            -2608894,
            16760832,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qb0 = re.clone();
        quad(
            re,
            16,
            STEP_BY_6,
            zeta61,
            -518909,
            16760832,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qb1 = re.clone();
        quad(
            re,
            4,
            STEP_BY_7,
            zeta7,
            25847,
            8380416,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qc0 = re.clone();
        quad(
            re,
            12,
            STEP_BY_7,
            zeta7,
            25847,
            8380416,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qc1 = re.clone();
        quad(
            re,
            4,
            STEP_BY_6,
            zeta60,
            -2608894,
            16760832,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qd0 = re.clone();
        quad(
            re,
            20,
            STEP_BY_6,
            zeta61,
            -518909,
            16760832,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        proof!(
            r#"
        q76_bound orig qa0 qa1 qb0 qb1 qc0 qc1 qd0 ${re};
        Avx2NttTheory.lemma_units_to_poly (8380416 + 2*8380416) ${re};
        let mid = Avx2NttTheory.build_mid_L7 orig in
        Avx2NttTheory.lemma_poly_to_units (8380416 + 8380416) mid;
        q76_l6_value orig mid qa0 qa1 qb0 qb1 qc0 qc1 qd0 ${re}
    "#
        );
    }

    #[cfg(hax)]
    let orig = re.clone();

    // IN-BODY over the REAL 32 interleaved Montgomery butterflies (8 quads of 4 `mul`
    // each, via `build_out`).  Ghost grouped-all-L7 `mid` feeds the layer atoms;
    // `build_out` gives the real out's bound + L6-value (bf_pair-of-mid); the layer
    // atoms chain into comp_7_6_done.  No fstar::replace; Avx2NttTheory.build_out_impl
    // is NOT called (its logic is replicated over the real muls).
    build_out(re, field_modulus, inverse_of_modulus_mod_montgomery_r);

    proof!(
        r#"
    let mid = Avx2NttTheory.build_mid_L7 orig in
    Avx2NttTheory.lemma_poly_to_units (8380416 + 8380416) mid;
    Avx2NttTheory.lemma_poly_to_units 8380416 orig;
    Avx2NttTheory.lemma_L7_atoms orig mid;
    Avx2NttTheory.lemma_L6_atoms mid ${re};
    Avx2NttTheory.lemma_compose_7_6_o orig mid ${re}
"#
    );
}

/// Layer 5, 4, 3
///
/// Each layer does 16 Montgomery multiplications -> 3*16 = 48 total
/// pqclean does 4 * 4 on each layer -> 48 total | plus 4 * 4 shuffles every time (48)
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 400"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 2*8380416) $re
"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Avx2NttTheory.comp_5_3_done $re ${re}_future /\
    Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 5*8380416) ${re}_future
"#))]
unsafe fn ntt_at_layer_5_to_3(re: &mut AVX2RingElement) {
    #[inline(always)]
    #[hax_lib::fstar::before(r#"#restart-solver"#)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"#)]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        (v v_STEP == 8 \/ v v_STEP == 16 \/ v v_STEP == 32) /\
        v v_STEP_BY == v v_STEP / 8 /\
        v $index < 128 / v v_STEP /\
        Spec.Utils.is_i32b 4190208 $zeta /\
        (let offset = ((v $index) * (v v_STEP) * 2) / 8 in
         offset + 2 * (v v_STEP_BY) <= 32 /\
         (Spec.Utils.forall32 (fun i ->
            (i >= offset /\ i < offset + 2 * (v v_STEP_BY)) ==>
            Avx2NttTheory.is_i32b_unit_avx2
              (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v v_STEP_BY)) * 8380416)
              (Seq.index $re i))))
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        let offset = ((v $index) * (v v_STEP) * 2) / 8 in
        Avx2NttTheory.round_post_avx2 $re ${re}_future offset (v v_STEP_BY) $zeta
    "#))]
    fn round<const STEP: usize, const STEP_BY: usize>(
        re: &mut AVX2RingElement,
        index: usize,
        zeta: i32,
    ) {
        // Per-pair butterfly: the windowed round's inner fold step, factored into its
        // own VC for proof performance (mirrors Portable.Ntt.outer_3_plus__round;
        // runtime-identical via #[inline(always)]).  Body = the REAL butterfly
        // (mont-mul on unit index+step_by, then sub/add cross) + the
        // unit_post_cross_avx2 producer proof.
        #[inline(always)]
        #[hax_lib::fstar::before(r#"#restart-solver"#)]
        #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"#)]
        #[hax_lib::requires(fstar!(r#"
            v $step_by > 0 /\
            v $index + v $step_by < 32 /\
            Spec.Utils.is_i32b 4190208 $zeta /\
            Avx2NttTheory.is_i32b_unit_avx2
              (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v $step_by)) * 8380416)
              (Seq.index $re (v $index)) /\
            Avx2NttTheory.is_i32b_unit_avx2
              (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v $step_by)) * 8380416)
              (Seq.index $re (v $index + v $step_by))
        "#))]
        #[hax_lib::ensures(|result| fstar!(r#"
            Spec.Utils.modifies2_32 $re ${re}_future $index ($index +! $step_by) /\
            Avx2NttTheory.is_i32b_unit_avx2
              (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v $step_by) + 1) * 8380416)
              (Seq.index ${re}_future (v $index)) /\
            Avx2NttTheory.is_i32b_unit_avx2
              (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v $step_by) + 1) * 8380416)
              (Seq.index ${re}_future (v $index + v $step_by)) /\
            Avx2NttTheory.unit_post_cross_avx2
              (Seq.index (Avx2NttTheory.chunks_of_re_avx2 $re) (v $index))
              (Seq.index (Avx2NttTheory.chunks_of_re_avx2 $re) (v $index + v $step_by))
              (Seq.index (Avx2NttTheory.chunks_of_re_avx2 ${re}_future) (v $index))
              (Seq.index (Avx2NttTheory.chunks_of_re_avx2 ${re}_future) (v $index + v $step_by))
              $zeta
        "#))]
        fn butterfly(re: &mut AVX2RingElement, index: usize, step_by: usize, zeta: i32) {
            let rhs = mm256_set1_epi32(zeta);
            #[cfg(hax)]
            let re_old = re.clone();
            // ---- REAL butterfly (the original round-loop step) ----
            arithmetic::montgomery_multiply(&mut re[index + step_by].value, &rhs);
            let tmp = mm256_sub_epi32(re[index].value, re[index + step_by].value);
            re[index] = AVX2SIMDUnit {
                value: mm256_add_epi32(re[index].value, re[index + step_by].value),
            };
            re[index + step_by] = AVX2SIMDUnit { value: tmp };
            // ---- per-pair proof: ntt_step fact -> cross-pair relations + output bounds ----
            proof!(
                r#"
            let bnd : nat = 8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v $step_by)) * 8380416 in
            let re0 = (Seq.index re_old (v $index)).f_value in
            let re1 = (Seq.index re_old (v $index + v $step_by)).f_value in
            let nre0 = (Seq.index ${re} (v $index)).f_value in
            let nre1 = (Seq.index ${re} (v $index + v $step_by)).f_value in
            let lane (i:nat{i<8}) : Lemma
                ((to_i32x8 nre0 (mk_u64 i), to_i32x8 nre1 (mk_u64 i)) ==
                 ntt_step (to_i32x8 $rhs (mk_int i))
                   (to_i32x8 re0 (mk_u64 i), to_i32x8 re1 (mk_u64 i)))
              = assert (to_i32x8 $rhs (mk_int i) == $zeta);
                assert (to_i32x8 $rhs (mk_u64 i) == $zeta)
            in
            lane 0; lane 1; lane 2; lane 3; lane 4; lane 5; lane 6; lane 7;
            introduce forall (l:nat). l < 8 ==> to_i32x8 $rhs (mk_int l) == $zeta
            with (introduce l < 8 ==> to_i32x8 $rhs (mk_int l) == $zeta
                  with _. assert (to_i32x8 $rhs (mk_int l) == $zeta));
            Avx2NttTheory.lemma_cross_pair_relations_ws re_old ${re} bnd (v $index) (v $index + v $step_by) $rhs $zeta;
            Avx2NttTheory.lemma_is_i32b_unit_avx2_intro (bnd + 8380416) (Seq.index ${re} (v $index));
            Avx2NttTheory.lemma_is_i32b_unit_avx2_intro (bnd + 8380416) (Seq.index ${re} (v $index + v $step_by))
        "#
            );
        }

        let offset = (index * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT;

        // raw forall32 input bound (matches the requires); no win_bounded reveal needed
        #[cfg(hax)]
        let orig_re = re.clone();

        for j in offset..offset + STEP_BY {
            hax_lib::loop_invariant!(|j: usize| fstar!(
                r#"
                (Spec.Utils.modifies_range2_32 $orig_re ${re}
                    $offset $j
                    ($offset +! v_STEP_BY) ($j +! v_STEP_BY)) /\
                (Spec.Utils.forall32 (fun i ->
                        ((i >= v $offset /\ i < v $j) \/
                          (i >= v $offset + v v_STEP_BY /\ i < v $j + v v_STEP_BY)) ==>
                        Avx2NttTheory.is_i32b_unit_avx2
                          (8380416 + ((Avx2NttTheory.layer_bound_factor_avx2 (v v_STEP_BY)) + 1) * 8380416)
                          (Seq.index ${re} i))) /\
                (Spec.Utils.forall32 (fun u ->
                        (u >= v $offset /\ u < v $j) ==>
                        Avx2NttTheory.unit_post_cross_avx2 (Seq.index (Avx2NttTheory.chunks_of_re_avx2 $orig_re) u)
                          (Seq.index (Avx2NttTheory.chunks_of_re_avx2 $orig_re) (u + v v_STEP_BY))
                          (Seq.index (Avx2NttTheory.chunks_of_re_avx2 ${re}) u)
                          (Seq.index (Avx2NttTheory.chunks_of_re_avx2 ${re}) (u + v v_STEP_BY))
                          $zeta))
            "#
            ));
            #[cfg(hax)]
            let re_old = re.clone();
            proof!(
                r#"
            assert (Seq.index re_old (v $j) == Seq.index $orig_re (v $j));
            assert (Seq.index re_old (v $j + v v_STEP_BY) == Seq.index $orig_re (v $j + v v_STEP_BY));
            assert (Avx2NttTheory.is_i32b_unit_avx2
                      (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v v_STEP_BY)) * 8380416)
                      (Seq.index re_old (v $j)));
            assert (Avx2NttTheory.is_i32b_unit_avx2
                      (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v v_STEP_BY)) * 8380416)
                      (Seq.index re_old (v $j + v v_STEP_BY)))
        "#
            );
            // ---- per-pair butterfly (factored; proves its cross post in its own VC) ----
            butterfly(re, j, STEP_BY, zeta);
            proof!(
                r#"
            Avx2NttTheory.lemma_round_ws_maintains $orig_re re_old ${re} $offset $j v_STEP_BY
              (Avx2NttTheory.layer_bound_factor_avx2 (v v_STEP_BY)) $zeta
        "#
            );
        }

        // ---- tail seal: completed transparent invariant -> round_post_avx2 ----
        proof!(
            r#"
        let lbf : nat = Avx2NttTheory.layer_bound_factor_avx2 (v v_STEP_BY) in
        let offset_n : nat = ((v $index) * (v v_STEP) * 2) / 8 in
        // (a) modifies: the two adjacent ranges union to [offset, offset+2*STEP_BY)
        assert (Spec.Utils.modifies_range_32 $orig_re ${re}
                  (mk_usize offset_n) (mk_usize (offset_n + 2 * (v v_STEP_BY))));
        Avx2NttTheory.lemma_range32_to_modwin $orig_re ${re}
          (mk_usize offset_n) (mk_usize (offset_n + 2 * (v v_STEP_BY)));
        // (b) win_bounded on the whole window [offset, offset+2*STEP_BY): the completed
        // invariant's disjoint two-half bound merges to the contiguous window.
        assert (Spec.Utils.forall32 (fun i ->
                  (i >= offset_n /\ i < offset_n + 2 * (v v_STEP_BY)) ==>
                  Avx2NttTheory.is_i32b_unit_avx2 (8380416 + (lbf + 1) * 8380416) (Seq.index ${re} i)));
        Avx2NttTheory.lemma_win_bounded_from_forall32 ${re} offset_n (2 * (v v_STEP_BY))
          (8380416 + (lbf + 1) * 8380416);
        // (c) win_cross on the lo-half [offset, offset+STEP_BY)
        Avx2NttTheory.lemma_win_cross_from_forall32 $orig_re ${re} offset_n (v v_STEP_BY) $zeta;
        Avx2NttTheory.lemma_rp_intro $orig_re ${re} offset_n (v v_STEP_BY) $zeta
    "#
        );
    }

    // Layer 5 block (window-scoped; rounds at offsets 0/8/16/24, zetas zeta_r(4..7))
    #[inline(always)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 800"#)]
    // proof-residence: clean-context — local block helper kept next to the
    // rlimit-800 layer-5 window fn whose ambient facts it needs.
    #[hax_lib::fstar::before(
        r#"
(* Local block helper: derive the real round's raw forall32 input bound over a
   window [lo,lo+width) from the block's whole-poly bound + the modifies frame.
   Replaces the theory's win_bounded-producing lemma_window_from_modwin, since the
   in-body round consumes the raw forall32 form (not the sealed win_bounded atom). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_window_forall32_from_modwin
      (orig cur: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (lo width bnd:nat)
    : Lemma
      (requires
        Avx2NttTheory.modifies_win orig cur 0 lo /\ lo + width <= 32 /\
        Avx2NttTheory.is_i32b_poly_avx2 bnd orig)
      (ensures
        Spec.Utils.forall32 (fun i -> (i >= lo /\ i < lo + width) ==>
          Avx2NttTheory.is_i32b_unit_avx2 bnd (Seq.index cur i))) =
  Avx2NttTheory.lemma_poly_to_units bnd orig;
  let aux (i:nat{i<32})
      : Lemma ((i >= lo /\ i < lo + width) ==>
               Avx2NttTheory.is_i32b_unit_avx2 bnd (Seq.index cur i)) =
    if i >= lo && i < lo + width
    then Avx2NttTheory.lemma_modwin_lookup orig cur 0 lo i
    else ()
  in
  Classical.forall_intro aux
#pop-options
"#
    )]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 2*8380416) $re
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        Avx2NttTheory.layer_done $re ${re}_future 5 /\
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 3*8380416) ${re}_future
    "#))]
    fn l5_block(re: &mut AVX2RingElement) {
        // 0: 0, 1, 2, 3
        // 1: 8, 9, 10, 11
        // 2: 16, 17, 18, 19
        // 3: 24, 25, 26, 27
        const STEP: usize = 1 << 5;
        const STEP_BY: usize = STEP / COEFFICIENTS_IN_SIMD_UNIT;

        #[cfg(hax)]
        let orig = re.clone();
        // round 0
        proof!(
            r#"
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 4 == 237124);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 5 == (-777960));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 6 == (-876248));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 7 == 466468);
        Avx2NttTheory.lemma_modwin_refl orig 0 0;
        lemma_window_forall32_from_modwin orig orig 0 8 (8380416 + 2*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 0, 237124);
        #[cfg(hax)]
        let s1 = re.clone();
        // round 1
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin orig s1 0 4 (mk_i32 237124);
        Avx2NttTheory.lemma_modwin_union orig orig s1 0 0 8;
        lemma_window_forall32_from_modwin orig s1 8 8 (8380416 + 2*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 1, -777960);
        #[cfg(hax)]
        let s2 = re.clone();
        // round 2
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s1 s2 8 4 (mk_i32 (-777960));
        Avx2NttTheory.lemma_modwin_union orig s1 s2 0 8 16;
        lemma_window_forall32_from_modwin orig s2 16 8 (8380416 + 2*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 2, -876248);
        #[cfg(hax)]
        let s3 = re.clone();
        // round 3
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s2 s3 16 4 (mk_i32 (-876248));
        Avx2NttTheory.lemma_modwin_union orig s2 s3 0 16 24;
        lemma_window_forall32_from_modwin orig s3 24 8 (8380416 + 2*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 3, 466468);
        proof!(r#"Avx2NttTheory.lemma_l5_assemble_o orig s1 s2 s3 ${re}"#);
    }

    // Layer 4 block (rounds at offsets 4k, zetas zeta_r(8..15))
    #[inline(always)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 800"#)]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 3*8380416) $re
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        Avx2NttTheory.layer_done $re ${re}_future 4 /\
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 4*8380416) ${re}_future
    "#))]
    fn l4_block(re: &mut AVX2RingElement) {
        // 0: 0, 1
        // 1: 4, 5
        // 2: 8, 9
        // 3: 12, 13
        // 4: 16, 17
        // 5: 20, 21
        // 6: 24, 25
        // 7: 28, 29
        const STEP: usize = 1 << 4;
        const STEP_BY: usize = STEP / COEFFICIENTS_IN_SIMD_UNIT;

        #[cfg(hax)]
        let orig = re.clone();
        // round 0
        proof!(
            r#"
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 8 == 1826347);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 9 == 2353451);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 10 == (-359251));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 11 == (-2091905));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 12 == 3119733);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 13 == (-2884855));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 14 == 3111497);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 15 == 2680103);
        Avx2NttTheory.lemma_modwin_refl orig 0 0;
        lemma_window_forall32_from_modwin orig orig 0 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 0, 1826347);
        #[cfg(hax)]
        let s1 = re.clone();
        // round 1
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin orig s1 0 2 (mk_i32 1826347);
        Avx2NttTheory.lemma_modwin_union orig orig s1 0 0 4;
        lemma_window_forall32_from_modwin orig s1 4 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 1, 2353451);
        #[cfg(hax)]
        let s2 = re.clone();
        // round 2
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s1 s2 4 2 (mk_i32 2353451);
        Avx2NttTheory.lemma_modwin_union orig s1 s2 0 4 8;
        lemma_window_forall32_from_modwin orig s2 8 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 2, -359251);
        #[cfg(hax)]
        let s3 = re.clone();
        // round 3
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s2 s3 8 2 (mk_i32 (-359251));
        Avx2NttTheory.lemma_modwin_union orig s2 s3 0 8 12;
        lemma_window_forall32_from_modwin orig s3 12 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 3, -2091905);
        #[cfg(hax)]
        let s4 = re.clone();
        // round 4
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s3 s4 12 2 (mk_i32 (-2091905));
        Avx2NttTheory.lemma_modwin_union orig s3 s4 0 12 16;
        lemma_window_forall32_from_modwin orig s4 16 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 4, 3119733);
        #[cfg(hax)]
        let s5 = re.clone();
        // round 5
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s4 s5 16 2 (mk_i32 3119733);
        Avx2NttTheory.lemma_modwin_union orig s4 s5 0 16 20;
        lemma_window_forall32_from_modwin orig s5 20 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 5, -2884855);
        #[cfg(hax)]
        let s6 = re.clone();
        // round 6
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s5 s6 20 2 (mk_i32 (-2884855));
        Avx2NttTheory.lemma_modwin_union orig s5 s6 0 20 24;
        lemma_window_forall32_from_modwin orig s6 24 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 6, 3111497);
        #[cfg(hax)]
        let s7 = re.clone();
        // round 7
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s6 s7 24 2 (mk_i32 3111497);
        Avx2NttTheory.lemma_modwin_union orig s6 s7 0 24 28;
        lemma_window_forall32_from_modwin orig s7 28 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 7, 2680103);
        proof!(r#"Avx2NttTheory.lemma_l4_assemble_o orig s1 s2 s3 s4 s5 s6 s7 ${re}"#);
    }

    // Layer 3 block (rounds at offsets 2k, zetas zeta_r(16..31))
    #[inline(always)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 800"#)]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 4*8380416) $re
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        Avx2NttTheory.layer_done $re ${re}_future 3 /\
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 5*8380416) ${re}_future
    "#))]
    fn l3_block(re: &mut AVX2RingElement) {
        // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
        const STEP: usize = 1 << 3;
        const STEP_BY: usize = STEP / COEFFICIENTS_IN_SIMD_UNIT;

        #[cfg(hax)]
        let orig = re.clone();
        // round 0
        proof!(
            r#"
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 16 == 2725464);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 17 == 1024112);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 18 == (-1079900));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 19 == 3585928);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 20 == (-549488));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 21 == (-1119584));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 22 == 2619752);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 23 == (-2108549));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 24 == (-2118186));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 25 == (-3859737));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 26 == (-1399561));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 27 == (-3277672));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 28 == 1757237);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 29 == (-19422));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 30 == 4010497);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 31 == 280005);
        Avx2NttTheory.lemma_modwin_refl orig 0 0;
        lemma_window_forall32_from_modwin orig orig 0 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 0, 2725464);
        #[cfg(hax)]
        let s1 = re.clone();
        // round 1
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin orig s1 0 1 (mk_i32 2725464);
        Avx2NttTheory.lemma_modwin_union orig orig s1 0 0 2;
        lemma_window_forall32_from_modwin orig s1 2 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 1, 1024112);
        #[cfg(hax)]
        let s2 = re.clone();
        // round 2
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s1 s2 2 1 (mk_i32 1024112);
        Avx2NttTheory.lemma_modwin_union orig s1 s2 0 2 4;
        lemma_window_forall32_from_modwin orig s2 4 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 2, -1079900);
        #[cfg(hax)]
        let s3 = re.clone();
        // round 3
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s2 s3 4 1 (mk_i32 (-1079900));
        Avx2NttTheory.lemma_modwin_union orig s2 s3 0 4 6;
        lemma_window_forall32_from_modwin orig s3 6 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 3, 3585928);
        #[cfg(hax)]
        let s4 = re.clone();
        // round 4
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s3 s4 6 1 (mk_i32 3585928);
        Avx2NttTheory.lemma_modwin_union orig s3 s4 0 6 8;
        lemma_window_forall32_from_modwin orig s4 8 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 4, -549488);
        #[cfg(hax)]
        let s5 = re.clone();
        // round 5
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s4 s5 8 1 (mk_i32 (-549488));
        Avx2NttTheory.lemma_modwin_union orig s4 s5 0 8 10;
        lemma_window_forall32_from_modwin orig s5 10 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 5, -1119584);
        #[cfg(hax)]
        let s6 = re.clone();
        // round 6
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s5 s6 10 1 (mk_i32 (-1119584));
        Avx2NttTheory.lemma_modwin_union orig s5 s6 0 10 12;
        lemma_window_forall32_from_modwin orig s6 12 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 6, 2619752);
        #[cfg(hax)]
        let s7 = re.clone();
        // round 7
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s6 s7 12 1 (mk_i32 2619752);
        Avx2NttTheory.lemma_modwin_union orig s6 s7 0 12 14;
        lemma_window_forall32_from_modwin orig s7 14 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 7, -2108549);
        #[cfg(hax)]
        let s8 = re.clone();
        // round 8
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s7 s8 14 1 (mk_i32 (-2108549));
        Avx2NttTheory.lemma_modwin_union orig s7 s8 0 14 16;
        lemma_window_forall32_from_modwin orig s8 16 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 8, -2118186);
        #[cfg(hax)]
        let s9 = re.clone();
        // round 9
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s8 s9 16 1 (mk_i32 (-2118186));
        Avx2NttTheory.lemma_modwin_union orig s8 s9 0 16 18;
        lemma_window_forall32_from_modwin orig s9 18 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 9, -3859737);
        #[cfg(hax)]
        let s10 = re.clone();
        // round 10
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s9 s10 18 1 (mk_i32 (-3859737));
        Avx2NttTheory.lemma_modwin_union orig s9 s10 0 18 20;
        lemma_window_forall32_from_modwin orig s10 20 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 10, -1399561);
        #[cfg(hax)]
        let s11 = re.clone();
        // round 11
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s10 s11 20 1 (mk_i32 (-1399561));
        Avx2NttTheory.lemma_modwin_union orig s10 s11 0 20 22;
        lemma_window_forall32_from_modwin orig s11 22 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 11, -3277672);
        #[cfg(hax)]
        let s12 = re.clone();
        // round 12
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s11 s12 22 1 (mk_i32 (-3277672));
        Avx2NttTheory.lemma_modwin_union orig s11 s12 0 22 24;
        lemma_window_forall32_from_modwin orig s12 24 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 12, 1757237);
        #[cfg(hax)]
        let s13 = re.clone();
        // round 13
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s12 s13 24 1 (mk_i32 1757237);
        Avx2NttTheory.lemma_modwin_union orig s12 s13 0 24 26;
        lemma_window_forall32_from_modwin orig s13 26 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 13, -19422);
        #[cfg(hax)]
        let s14 = re.clone();
        // round 14
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s13 s14 26 1 (mk_i32 (-19422));
        Avx2NttTheory.lemma_modwin_union orig s13 s14 0 26 28;
        lemma_window_forall32_from_modwin orig s14 28 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 14, 4010497);
        #[cfg(hax)]
        let s15 = re.clone();
        // round 15
        proof!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s14 s15 28 1 (mk_i32 4010497);
        Avx2NttTheory.lemma_modwin_union orig s14 s15 0 28 30;
        lemma_window_forall32_from_modwin orig s15 30 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 15, 280005);
        proof!(
            r#"Avx2NttTheory.lemma_l3_assemble_o orig s1 s2 s3 s4 s5 s6 s7 s8 s9 s10 s11 s12 s13 s14 s15 ${re}"#
        );
    }

    #[cfg(hax)]
    let orig = re.clone();
    l5_block(re);
    #[cfg(hax)]
    let s_l5 = re.clone();
    l4_block(re);
    #[cfg(hax)]
    let s_l4 = re.clone();
    l3_block(re);
    proof!(r#"Avx2NttTheory.lemma_compose_5_3_o orig s_l5 s_l4 ${re}"#);
}

#[allow(unsafe_code)]
#[inline(always)]
#[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 400"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Avx2NttTheory.is_i32b_poly_avx2 8380416 $re
"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 8*8380416) ${re}_future /\
    (let in_flat = C.simd_units_to_array (Avx2NttTheory.chunks_of_re_avx2 $re) in
     let out_flat = C.simd_units_to_array (Avx2NttTheory.chunks_of_re_avx2 ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt in_flat in
     forall (i: nat). i < 256 ==>
       v (Seq.index out_flat i) % 8380417 == v (Seq.index spec i) % 8380417)
"#))]
pub(crate) fn ntt(re: &mut AVX2RingElement) {
    #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
    #[hax_lib::fstar::before(r#"#restart-solver"#)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"#)]
    // proof-residence: clean-context — CHUNKS-EQ bridge between the LOCAL
    // chunks_of_re_avx2 and the theory's, tuned to the top-level ntt
    // composer's #restart-solver/split-queries context it sits in.
    #[hax_lib::fstar::before(
        r#"
(* CHUNKS-EQ bridge helper: the LOCAL `chunks_of_re_avx2` (Ntt.fst) and the
   theory's `Avx2NttTheory.chunks_of_re_avx2` are the SAME function — both
   project lane (b,l) to `to_i32x8 (Seq.index re b).f_value (mk_u64 l)`.  We
   prove pointwise equality from BOTH index lemmas, then Seq.lemma_eq_intro at
   the inner and outer levels.  This lets the L2/L1/L0 LOCAL congruences (over
   the local chunks) feed `Avx2NttTheory.lemma_layer_done_intro` (over the
   theory chunks). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_chunks_eq (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (Avx2NttTheory.chunks_of_re_avx2 re == chunks_of_re_avx2 re) =
  let cT = Avx2NttTheory.chunks_of_re_avx2 re in
  let cL = chunks_of_re_avx2 re in
  let aux (b: nat{b < 32}) : Lemma (Seq.index cT b == Seq.index cL b) =
    let auxl (l: nat{l < 8}) : Lemma (Seq.index (Seq.index cT b) l == Seq.index (Seq.index cL b) l) =
      Avx2NttTheory.lemma_chunks_of_re_avx2_index re b l;
      lemma_chunks_of_re_avx2_index re b l
    in
    Classical.forall_intro auxl;
    Seq.lemma_eq_intro (Seq.index cT b) (Seq.index cL b)
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro cT cL
#pop-options
"#
    )]
    #[hax_lib::requires(fstar!(r#"
        Avx2NttTheory.is_i32b_poly_avx2 8380416 $re
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 8*8380416) ${re}_future /\
        (let in_flat = C.simd_units_to_array (Avx2NttTheory.chunks_of_re_avx2 $re) in
         let out_flat = C.simd_units_to_array (Avx2NttTheory.chunks_of_re_avx2 ${re}_future) in
         let spec = Hacspec_ml_dsa.Ntt.ntt in_flat in
         forall (i: nat). i < 256 ==>
           v (Seq.index out_flat i) % 8380417 == v (Seq.index spec i) % 8380417)
    "#))]
    unsafe fn avx2_ntt(re: &mut AVX2RingElement) {
        proof!(
            r#"
        assert_norm (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND == 8380416);
        assert_norm (v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX == 8380416)
    "#
        );
        #[cfg(hax)]
        let orig = re.clone();
        ntt_at_layer_7_and_6(re);
        #[cfg(hax)]
        let s76 = re.clone();
        ntt_at_layer_5_to_3(re);
        #[cfg(hax)]
        let s53 = re.clone();
        // SEAM bridge: theory is_i32b_poly_avx2 (+5) s53 -> LOCAL is_i32b_poly_avx2 (+5) s53
        proof!(
            r#"
        let seam (u:nat{u<32}) (l:nat{l<8})
            : Lemma (Spec.Utils.is_i32b (8380416 + 5*8380416)
                       (to_i32x8 (Seq.index s53 u).f_value (mk_u64 l))) =
          Avx2NttTheory.lemma_is_i32b_poly_avx2_elim (8380416 + 5*8380416) s53 u l
        in
        Classical.forall_intro_2 seam;
        lemma_is_i32b_poly_avx2_intro (8380416 + 5*8380416) s53
    "#
        );
        ntt_at_layer_2(re);
        #[cfg(hax)]
        let s2 = re.clone();
        ntt_at_layer_1(re);
        #[cfg(hax)]
        let s1 = re.clone();
        ntt_at_layer_0(re);
        // CHUNKS bridge (3x): LOCAL congruence -> Avx2NttTheory.layer_done. The L2/L1/L0
        // ensures give the congruence over the LOCAL chunks; rewrite both sides to the
        // theory chunks via lemma_chunks_eq, then fire lemma_layer_done_intro.  Then the
        // top compose + reveal, and the FINAL bound bridge (LOCAL -> theory).
        proof!(
            r#"
        lemma_chunks_eq s53; lemma_chunks_eq s2; lemma_chunks_eq s1; lemma_chunks_eq ${re};
        Avx2NttTheory.lemma_layer_done_intro s53 s2 2;
        Avx2NttTheory.lemma_layer_done_intro s2 s1 1;
        Avx2NttTheory.lemma_layer_done_intro s1 ${re} 0;
        Avx2NttTheory.lemma_ntt_top_compose_o orig s76 s53 s2 s1 ${re};
        Avx2NttTheory.lemma_ntt_done_reveal orig ${re};
        let fin (u:nat{u<32}) (l:nat{l<8})
            : Lemma (Spec.Utils.is_i32b (8380416 + 8*8380416)
                       (to_i32x8 (Seq.index ${re} u).f_value (mk_u64 l))) =
          lemma_is_i32b_poly_avx2_elim (8380416 + 8*8380416) ${re} u l
        in
        Classical.forall_intro_2 fin;
        Avx2NttTheory.lemma_is_i32b_poly_avx2_intro (8380416 + 8*8380416) ${re}
    "#
        );
    }

    unsafe { avx2_ntt(re) }
}

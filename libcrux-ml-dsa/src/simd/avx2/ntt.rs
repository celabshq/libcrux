use libcrux_intrinsics::avx2::*;

use super::{arithmetic, AVX2RingElement, AVX2SIMDUnit};
use crate::simd::traits::COEFFICIENTS_IN_SIMD_UNIT;

// Compute (a,b) ↦ (a + ζb, a - ζb) at layer 0 for 2 SIMD Units in one go.
#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::before(r"open Spec.MLDSA.Ntt"))]
#[cfg_attr(hax, hax_lib::fstar::before(r"open Spec.Intrinsics"))]
#[cfg_attr(
    hax,
    hax_lib::fstar::before(
        r#"
let butterfly_2_spec re0 re1 zeta_a0 zeta_a1 zeta_a2 zeta_a3 
                     zeta_b0 zeta_b1 zeta_b2 zeta_b3 nre0 nre1 =
    (to_i32x8 nre0 (mk_u64 0), to_i32x8 nre0 (mk_u64 1)) ==
     ntt_step zeta_a0 (to_i32x8 re0 (mk_u64 0), to_i32x8 re0 (mk_u64 1)) /\
    (to_i32x8 nre0 (mk_u64 2), to_i32x8 nre0 (mk_u64 3)) ==
     ntt_step zeta_a1 (to_i32x8 re0 (mk_u64 2), to_i32x8 re0 (mk_u64 3)) /\
    (to_i32x8 nre0 (mk_u64 4), to_i32x8 nre0 (mk_u64 5)) ==
     ntt_step zeta_a2 (to_i32x8 re0 (mk_u64 4), to_i32x8 re0 (mk_u64 5)) /\
    (to_i32x8 nre0 (mk_u64 6), to_i32x8 nre0 (mk_u64 7)) ==
     ntt_step zeta_a3 (to_i32x8 re0 (mk_u64 6), to_i32x8 re0 (mk_u64 7)) /\
    (to_i32x8 nre1 (mk_u64 0), to_i32x8 nre1 (mk_u64 1)) ==
     ntt_step zeta_b0 (to_i32x8 re1 (mk_u64 0), to_i32x8 re1 (mk_u64 1)) /\
    (to_i32x8 nre1 (mk_u64 2), to_i32x8 nre1 (mk_u64 3)) ==
     ntt_step zeta_b1 (to_i32x8 re1 (mk_u64 2), to_i32x8 re1 (mk_u64 3)) /\
    (to_i32x8 nre1 (mk_u64 4), to_i32x8 nre1 (mk_u64 5)) ==
     ntt_step zeta_b2 (to_i32x8 re1 (mk_u64 4), to_i32x8 re1 (mk_u64 5)) /\
    (to_i32x8 nre1 (mk_u64 6), to_i32x8 nre1 (mk_u64 7)) ==
     ntt_step zeta_b3 (to_i32x8 re1 (mk_u64 6), to_i32x8 re1 (mk_u64 7))
"#
    )
)]
#[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
#[cfg_attr(hax, hax_lib::requires(index < 31))]
#[cfg_attr(hax, hax_lib::ensures(|_result| fstar!(r"
        butterfly_2_spec (Seq.index ${re} (v $index)).f_value
                         (Seq.index ${re} (v $index + 1)).f_value
                         $zeta_a0 $zeta_a1 $zeta_a2 $zeta_a3 $zeta_b0 $zeta_b1 $zeta_b2 $zeta_b3
                         (Seq.index ${re}_future (v $index)).f_value
                         (Seq.index ${re}_future (v $index + 1)).f_value /\
        Spec.Utils.modifies2_32 re ${re}_future $index ($index +! mk_int 1)
")))]
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
    #[cfg(hax)]
    hax_lib::fstar!(
        r#"assert (butterfly_2_spec 
                            $re0 $re1 $zeta_a0 $zeta_a1 $zeta_a2 $zeta_a3 
                            $zeta_b0 $zeta_b1 $zeta_b2 $zeta_b3 $nre0 $nre1)"#
    );

    re[index].value = nre0;
    re[index + 1].value = nre1;
}

// Compute (a,b) ↦ (a + ζb, a - ζb) at layer 1 for 2 SIMD Units in one go.
#[inline(always)]
#[cfg_attr(
    hax,
    hax_lib::fstar::before(
        r#"
let butterfly_4_spec re0 re1 zeta_a0 zeta_a1 zeta_b0 zeta_b1 nre0 nre1 =
    (to_i32x8 nre0 (mk_u64 0), to_i32x8 nre0 (mk_u64 2)) ==
    ntt_step zeta_a0 (to_i32x8 re0 (mk_u64 0), to_i32x8 re0 (mk_u64 2)) /\
    (to_i32x8 nre0 (mk_u64 1), to_i32x8 nre0 (mk_u64 3)) ==
    ntt_step zeta_a0 (to_i32x8 re0 (mk_u64 1), to_i32x8 re0 (mk_u64 3)) /\
    (to_i32x8 nre0 (mk_u64 4), to_i32x8 nre0 (mk_u64 6)) ==
    ntt_step zeta_a1 (to_i32x8 re0 (mk_u64 4), to_i32x8 re0 (mk_u64 6)) /\
    (to_i32x8 nre0 (mk_u64 5), to_i32x8 nre0 (mk_u64 7)) ==
    ntt_step zeta_a1 (to_i32x8 re0 (mk_u64 5), to_i32x8 re0 (mk_u64 7)) /\
    (to_i32x8 nre1 (mk_u64 0), to_i32x8 nre1 (mk_u64 2)) ==
    ntt_step zeta_b0 (to_i32x8 re1 (mk_u64 0), to_i32x8 re1 (mk_u64 2)) /\
    (to_i32x8 nre1 (mk_u64 1), to_i32x8 nre1 (mk_u64 3)) ==
    ntt_step zeta_b0 (to_i32x8 re1 (mk_u64 1), to_i32x8 re1 (mk_u64 3)) /\
    (to_i32x8 nre1 (mk_u64 4), to_i32x8 nre1 (mk_u64 6)) ==
    ntt_step zeta_b1 (to_i32x8 re1 (mk_u64 4), to_i32x8 re1 (mk_u64 6)) /\
    (to_i32x8 nre1 (mk_u64 5), to_i32x8 nre1 (mk_u64 7)) ==
    ntt_step zeta_b1 (to_i32x8 re1 (mk_u64 5), to_i32x8 re1 (mk_u64 7))
"#
    )
)]
#[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
#[cfg_attr(hax, hax_lib::requires(index < 31))]
#[cfg_attr(hax, hax_lib::ensures(|_result| fstar!(r"
        butterfly_4_spec    (Seq.index ${re} (v $index)).f_value
                            (Seq.index ${re} (v $index + 1)).f_value
                            $zeta_a0 $zeta_a1 $zeta_b0 $zeta_b1
                            (Seq.index ${re}_future (v $index)).f_value
                            (Seq.index ${re}_future (v $index + 1)).f_value /\
        Spec.Utils.modifies2_32 $re ${re}_future $index ($index +! mk_int 1)
")))]
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
    #[cfg(hax)]
    hax_lib::fstar!(
        r#"assert (butterfly_4_spec 
        $re0 $re1 $zeta_a0 $zeta_a1 $zeta_b0 $zeta_b1 $nre0 $nre1)"#
    );

    re[index].value = nre0;
    re[index + 1].value = nre1;
}

// Compute (a,b) ↦ (a + ζb, a - ζb) at layer 2 for 2 SIMD Units in one go.
#[inline(always)]
#[cfg_attr(
    hax,
    hax_lib::fstar::before(
        r#"
let butterfly_8_spec re0 re1 zeta0 zeta1 nre0 nre1 =
    (to_i32x8 nre0 (mk_u64 0), to_i32x8 nre0 (mk_u64 4)) ==
     ntt_step zeta0 (to_i32x8 re0 (mk_u64 0), to_i32x8 re0 (mk_u64 4)) /\
    (to_i32x8 nre0 (mk_u64 1), to_i32x8 nre0 (mk_u64 5)) ==
     ntt_step zeta0 (to_i32x8 re0 (mk_u64 1), to_i32x8 re0 (mk_u64 5)) /\
    (to_i32x8 nre0 (mk_u64 2), to_i32x8 nre0 (mk_u64 6)) ==
     ntt_step zeta0 (to_i32x8 re0 (mk_u64 2), to_i32x8 re0 (mk_u64 6)) /\
    (to_i32x8 nre0 (mk_u64 3), to_i32x8 nre0 (mk_u64 7)) ==
     ntt_step zeta0 (to_i32x8 re0 (mk_u64 3), to_i32x8 re0 (mk_u64 7)) /\
    (to_i32x8 nre1 (mk_u64 0), to_i32x8 nre1 (mk_u64 4)) ==
     ntt_step zeta1 (to_i32x8 re1 (mk_u64 0), to_i32x8 re1 (mk_u64 4)) /\
    (to_i32x8 nre1 (mk_u64 1), to_i32x8 nre1 (mk_u64 5)) ==
     ntt_step zeta1 (to_i32x8 re1 (mk_u64 1), to_i32x8 re1 (mk_u64 5)) /\
    (to_i32x8 nre1 (mk_u64 2), to_i32x8 nre1 (mk_u64 6)) ==
     ntt_step zeta1 (to_i32x8 re1 (mk_u64 2), to_i32x8 re1 (mk_u64 6)) /\
    (to_i32x8 nre1 (mk_u64 3), to_i32x8 nre1 (mk_u64 7)) ==
     ntt_step zeta1 (to_i32x8 re1 (mk_u64 3), to_i32x8 re1 (mk_u64 7))
"#
    )
)]
#[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
#[cfg_attr(hax, hax_lib::requires(index < 31))]
#[cfg_attr(hax, hax_lib::ensures(|_result| fstar!(r"
        butterfly_8_spec    (Seq.index ${re} (v $index)).f_value
                            (Seq.index ${re} (v $index + 1)).f_value
                            $zeta0 $zeta1
                            (Seq.index ${re}_future (v $index)).f_value
                            (Seq.index ${re}_future (v $index + 1)).f_value /\
        Spec.Utils.modifies2_32 $re ${re}_future $index ($index +! mk_int 1)
")))]
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
    #[cfg(hax)]
    hax_lib::fstar!(
        r#"assert (butterfly_8_spec 
         $re0 $re1 $zeta0 $zeta1 $nre0 $nre1)"#
    );

    re[index].value = nre0;
    re[index + 1].value = nre1;
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall4; `%Spec.Utils.forall16 ]] (
   Spec.Utils.forall16 (fun i ->
     let  nre = ${re}_future in
     let  re0 = Seq.index $re (i * 2) in
     let  re1 = Seq.index $re (i * 2 + 1) in
     let nre0 = Seq.index nre (i * 2) in
     let nre1 = Seq.index nre (i * 2 + 1) in
     Spec.Utils.forall4 (fun j ->
       let zeta0 = zeta_r (128 + i * 8 + j) in
       let zeta1 = zeta_r (128 + i * 8 + j + 4) in
       let j0 = j * 2 in
       let j1 = j0 + 1 in
       (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
        ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
       (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
        ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1))
     )
   )
)
"#)))]
unsafe fn ntt_at_layer_0(re: &mut AVX2RingElement) {
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
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall4; `%Spec.Utils.forall16 ]] (
   Spec.Utils.forall16 (fun i ->
     let  nre = ${re}_future in
     let  re0 = Seq.index $re (i * 2) in
     let  re1 = Seq.index $re (i * 2 + 1) in
     let nre0 = Seq.index nre (i * 2) in
     let nre1 = Seq.index nre (i * 2 + 1) in
     Spec.Utils.forall4 (fun j ->
         let zeta0 = zeta_r (64 + i * 4 + j / 2) in
         let zeta1 = zeta_r (64 + i * 4 + j / 2 + 2) in
         let j0 = match j with
           | 0 -> 0 | 1 -> 1
           | 2 -> 4 | 3 -> 5
         in
         let j1 = j0 + 2 in
         (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
          ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
         (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
          ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1))
     )
   )
)
"#)))]
unsafe fn ntt_at_layer_1(re: &mut AVX2RingElement) {
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
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
#[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall4; `%Spec.Utils.forall16 ]] (
   Spec.Utils.forall16 (fun i ->
     let  nre = ${re}_future in
     let  re0 = Seq.index $re (i * 2) in
     let  re1 = Seq.index $re (i * 2 + 1) in
     let nre0 = Seq.index nre (i * 2) in
     let nre1 = Seq.index nre (i * 2 + 1) in
     Spec.Utils.forall4 (fun j ->
        let zeta0 = zeta_r (32 + i * 2) in
        let zeta1 = zeta_r (32 + i * 2 + 1) in
        let j0 = j in
        let j1 = j0 + 4 in
        (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
        ntt_step (mk_int zeta0)
          (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
        (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
        ntt_step (mk_int zeta1)
          (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1))
     )
   )
)
"#)))]
unsafe fn ntt_at_layer_2(re: &mut AVX2RingElement) {
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
}

/// This is equivalent to the pqclean 0 and 1
///
/// This does 32 Montgomery multiplications (192 multiplications).
/// This is the same as in pqclean. The only difference is locality of registers.
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[cfg_attr(hax, hax_lib::fstar::options(r#"--fuel 0 --ifuel 0 --z3rlimit 200"#))]
#[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
unsafe fn ntt_at_layer_7_and_6(re: &mut AVX2RingElement) {
    let field_modulus = mm256_set1_epi32(crate::simd::traits::FIELD_MODULUS);
    let inverse_of_modulus_mod_montgomery_r =
        mm256_set1_epi32(crate::simd::traits::INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i32);

    #[inline(always)]
    #[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
    #[cfg_attr(hax, hax_lib::requires({
        use crate::constants::FIELD_MODULUS;
        use crate::simd::traits::INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;
        use hax_lib::int::{ToInt, int};
        hax_lib::eq(field_modulus, mm256_set1_epi32(FIELD_MODULUS)).and(
            hax_lib::eq(inverse_of_modulus_mod_montgomery_r, mm256_set1_epi32(INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i32))
        ).and(index.to_int() + step_by.to_int() < int!(32)).and(step_by > 0)
    }))]
    #[cfg_attr(hax, hax_lib::ensures(|result| fstar!(r#"
        let re0 = (Seq.index $re (v $index)).f_value in
        let re1 = (Seq.index $re (v $index + v $step_by)).f_value in
        let nre0 = (Seq.index ${re}_future (v $index)).f_value in
        let nre1 = (Seq.index ${re}_future (v $index + v $step_by)).f_value in
        Spec.Utils.modifies2_32 $re ${re}_future $index ($index +! $step_by) /\
        Spec.Utils.forall8 (fun i ->
           (to_i32x8 nre0 (mk_u64 i), to_i32x8 nre1 (mk_u64 i)) ==
           ntt_step (to_i32x8 zeta (mk_int i)) (to_i32x8 re0 (mk_u64 i), to_i32x8 re1 (mk_u64 i))
        )
    "#)))]
    fn mul(
        re: &mut AVX2RingElement,
        index: usize,
        zeta: Vec256,
        step_by: usize,
        field_modulus: Vec256,
        inverse_of_modulus_mod_montgomery_r: Vec256,
    ) {
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

    macro_rules! layer {
        ($start:literal, $zeta:expr, $step_by:expr) => {{
            mul(
                re,
                $start,
                $zeta,
                $step_by,
                field_modulus,
                inverse_of_modulus_mod_montgomery_r,
            );
            mul(
                re,
                $start + 1,
                $zeta,
                $step_by,
                field_modulus,
                inverse_of_modulus_mod_montgomery_r,
            );
            mul(
                re,
                $start + 2,
                $zeta,
                $step_by,
                field_modulus,
                inverse_of_modulus_mod_montgomery_r,
            );
            mul(
                re,
                $start + 3,
                $zeta,
                $step_by,
                field_modulus,
                inverse_of_modulus_mod_montgomery_r,
            );
        }};
    }

    // Note: For proofs, it is better to use concrete constants instead of const expressions
    const STEP_BY_7: usize = 16; //2 * COEFFICIENTS_IN_SIMD_UNIT;
    const STEP_BY_6: usize = 8; //(1 << 6) / COEFFICIENTS_IN_SIMD_UNIT;

    let zeta7 = mm256_set1_epi32(25847);
    let zeta60 = mm256_set1_epi32(-2608894);
    let zeta61 = mm256_set1_epi32(-518909);

    layer!(0, zeta7, STEP_BY_7);
    layer!(8, zeta7, STEP_BY_7);
    layer!(0, zeta60, STEP_BY_6);
    layer!(16, zeta61, STEP_BY_6);

    layer!(4, zeta7, STEP_BY_7);
    layer!(12, zeta7, STEP_BY_7);
    layer!(4, zeta60, STEP_BY_6);
    layer!(20, zeta61, STEP_BY_6);
}

/// Layer 5, 4, 3
///
/// Each layer does 16 Montgomery multiplications -> 3*16 = 48 total
/// pqclean does 4 * 4 on each layer -> 48 total | plus 4 * 4 shuffles every time (48)
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
unsafe fn ntt_at_layer_5_to_3(re: &mut AVX2RingElement) {
    #[inline(always)]
    #[cfg_attr(hax, hax_lib::requires(
        (STEP == 8 || STEP == 16 || STEP == 32) && STEP_BY == STEP / COEFFICIENTS_IN_SIMD_UNIT && index < 128 / STEP
    ))]
    fn round<const STEP: usize, const STEP_BY: usize>(
        re: &mut AVX2RingElement,
        index: usize,
        zeta: i32,
    ) {
        let rhs = mm256_set1_epi32(zeta);
        let offset = (index * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT;

        for j in offset..offset + STEP_BY {
            arithmetic::montgomery_multiply(&mut re[j + STEP_BY].value, &rhs);

            let tmp = mm256_sub_epi32(re[j].value, re[j + STEP_BY].value);
            re[j] = AVX2SIMDUnit {
                value: mm256_add_epi32(re[j].value, re[j + STEP_BY].value),
            };
            re[j + STEP_BY] = AVX2SIMDUnit { value: tmp };
        }
    }

    // Layer 5
    {
        // 0: 0, 1, 2, 3
        // 1: 8, 9, 10, 11
        // 2: 16, 17, 18, 19
        // 3: 24, 25, 26, 27
        const STEP: usize = 1 << 5;
        const STEP_BY: usize = STEP / COEFFICIENTS_IN_SIMD_UNIT;

        round::<STEP, STEP_BY>(re, 0, 237124);
        round::<STEP, STEP_BY>(re, 1, -777960);
        round::<STEP, STEP_BY>(re, 2, -876248);
        round::<STEP, STEP_BY>(re, 3, 466468);
    }

    // Layer 4
    {
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

        round::<STEP, STEP_BY>(re, 0, 1826347);
        round::<STEP, STEP_BY>(re, 1, 2353451);
        round::<STEP, STEP_BY>(re, 2, -359251);
        round::<STEP, STEP_BY>(re, 3, -2091905);
        round::<STEP, STEP_BY>(re, 4, 3119733);
        round::<STEP, STEP_BY>(re, 5, -2884855);
        round::<STEP, STEP_BY>(re, 6, 3111497);
        round::<STEP, STEP_BY>(re, 7, 2680103);
    }

    // Layer 3
    {
        // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
        const STEP: usize = 1 << 3;
        const STEP_BY: usize = STEP / COEFFICIENTS_IN_SIMD_UNIT;

        round::<STEP, STEP_BY>(re, 0, 2725464);
        round::<STEP, STEP_BY>(re, 1, 1024112);
        round::<STEP, STEP_BY>(re, 2, -1079900);
        round::<STEP, STEP_BY>(re, 3, 3585928);
        round::<STEP, STEP_BY>(re, 4, -549488);
        round::<STEP, STEP_BY>(re, 5, -1119584);
        round::<STEP, STEP_BY>(re, 6, 2619752);
        round::<STEP, STEP_BY>(re, 7, -2108549);
        round::<STEP, STEP_BY>(re, 8, -2118186);
        round::<STEP, STEP_BY>(re, 9, -3859737);
        round::<STEP, STEP_BY>(re, 10, -1399561);
        round::<STEP, STEP_BY>(re, 11, -3277672);
        round::<STEP, STEP_BY>(re, 12, 1757237);
        round::<STEP, STEP_BY>(re, 13, -19422);
        round::<STEP, STEP_BY>(re, 14, 4010497);
        round::<STEP, STEP_BY>(re, 15, 280005);
    }
    ()
}

#[allow(unsafe_code)]
#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#))]
pub(crate) fn ntt(re: &mut AVX2RingElement) {
    #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
    unsafe fn avx2_ntt(re: &mut AVX2RingElement) {
        ntt_at_layer_7_and_6(re);
        ntt_at_layer_5_to_3(re);
        ntt_at_layer_2(re);
        ntt_at_layer_1(re);
        ntt_at_layer_0(re);
    }

    unsafe { avx2_ntt(re) }
}

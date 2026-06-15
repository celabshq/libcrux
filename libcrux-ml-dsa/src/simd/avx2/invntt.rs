use libcrux_intrinsics::avx2::*;

use super::{arithmetic, AVX2RingElement};
use crate::simd::{avx2::AVX2SIMDUnit, traits::COEFFICIENTS_IN_SIMD_UNIT};

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::verification_status(lax)]
pub(crate) fn invert_ntt_montgomery(re: &mut AVX2RingElement) {
    #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
    #[allow(unsafe_code)]
    #[hax_lib::fstar::verification_status(lax)]
    unsafe fn inv_inner(re: &mut AVX2RingElement) {
        invert_ntt_at_layer_0(re);
        invert_ntt_at_layer_1(re);
        invert_ntt_at_layer_2(re);
        invert_ntt_at_layer_3(re);
        invert_ntt_at_layer_4(re);
        invert_ntt_at_layer_5(re);
        invert_ntt_at_layer_6(re);
        invert_ntt_at_layer_7(re);

        for i in 0..re.len() {
            // After invert_ntt_at_layer, elements are of the form a * MONTGOMERY_R^{-1}
            // we multiply by (MONTGOMERY_R^2) * (1/2^8) mod Q = 41,978 to both:
            //
            // - Divide the elements by 256 and
            // - Convert the elements form montgomery domain to the standard domain.
            const FACTOR: i32 = 41_978;
            re[i].value = arithmetic::montgomery_multiply_by_constant(re[i].value, FACTOR);
        }
    }

    unsafe { inv_inner(re) };
}

#[inline(always)]
#[hax_lib::fstar::before(r"open Spec.MLDSA.Ntt")]
#[hax_lib::fstar::before(r"open Spec.Intrinsics")]
#[hax_lib::fstar::before(r"open Spec.Utils")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|(a, b)| fstar!(r#"
let nre0, nre1 = ${a}.f_value, ${b}.f_value in
let re0, re1 = ${simd_unit0}, ${simd_unit1} in
(to_i32x8 nre0 (mk_u64 0), to_i32x8 nre0 (mk_u64 1)) ==
 inv_ntt_step $zeta00 (to_i32x8 re0 (mk_u64 0), to_i32x8 re0 (mk_u64 1)) /\
(to_i32x8 nre0 (mk_u64 2), to_i32x8 nre0 (mk_u64 3)) ==
 inv_ntt_step $zeta01 (to_i32x8 re0 (mk_u64 2), to_i32x8 re0 (mk_u64 3)) /\
(to_i32x8 nre0 (mk_u64 4), to_i32x8 nre0 (mk_u64 5)) ==
 inv_ntt_step $zeta02 (to_i32x8 re0 (mk_u64 4), to_i32x8 re0 (mk_u64 5)) /\
(to_i32x8 nre0 (mk_u64 6), to_i32x8 nre0 (mk_u64 7)) ==
 inv_ntt_step $zeta03 (to_i32x8 re0 (mk_u64 6), to_i32x8 re0 (mk_u64 7)) /\
(to_i32x8 nre1 (mk_u64 0), to_i32x8 nre1 (mk_u64 1)) ==
 inv_ntt_step $zeta10 (to_i32x8 re1 (mk_u64 0), to_i32x8 re1 (mk_u64 1)) /\
(to_i32x8 nre1 (mk_u64 2), to_i32x8 nre1 (mk_u64 3)) ==
 inv_ntt_step $zeta11 (to_i32x8 re1 (mk_u64 2), to_i32x8 re1 (mk_u64 3)) /\
(to_i32x8 nre1 (mk_u64 4), to_i32x8 nre1 (mk_u64 5)) ==
 inv_ntt_step $zeta12 (to_i32x8 re1 (mk_u64 4), to_i32x8 re1 (mk_u64 5)) /\
(to_i32x8 nre1 (mk_u64 6), to_i32x8 nre1 (mk_u64 7)) ==
 inv_ntt_step $zeta13 (to_i32x8 re1 (mk_u64 6), to_i32x8 re1 (mk_u64 7))
"#))]
fn simd_unit_invert_ntt_at_layer_0(
    simd_unit0: Vec256,
    simd_unit1: Vec256,
    zeta00: i32,
    zeta01: i32,
    zeta02: i32,
    zeta03: i32,
    zeta10: i32,
    zeta11: i32,
    zeta12: i32,
    zeta13: i32,
) -> (AVX2SIMDUnit, AVX2SIMDUnit) {
    const SHUFFLE: i32 = 0b11_01_10_00;
    let a_shuffled = mm256_shuffle_epi32::<SHUFFLE>(simd_unit0);
    let b_shuffled = mm256_shuffle_epi32::<SHUFFLE>(simd_unit1);

    let mut lo_values = mm256_unpacklo_epi64(a_shuffled, b_shuffled);
    let hi_values = mm256_unpackhi_epi64(a_shuffled, b_shuffled);

    let mut differences = hi_values;
    arithmetic::subtract(&mut differences, &lo_values);
    arithmetic::add(&mut lo_values, &hi_values);
    let sums = lo_values;

    let zetas = mm256_set_epi32(
        zeta13, zeta12, zeta03, zeta02, zeta11, zeta10, zeta01, zeta00,
    );
    arithmetic::montgomery_multiply(&mut differences, &zetas);

    let a_shuffled = mm256_unpacklo_epi64(sums, differences);
    let b_shuffled = mm256_unpackhi_epi64(sums, differences);

    let a = AVX2SIMDUnit {
        value: mm256_shuffle_epi32::<SHUFFLE>(a_shuffled),
    };
    let b = AVX2SIMDUnit {
        value: mm256_shuffle_epi32::<SHUFFLE>(b_shuffled),
    };

    (a, b)
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|(a, b)| fstar!(r#"
let nre0, nre1 = ${a}.f_value, ${b}.f_value in
let re0, re1 = ${simd_unit0}, ${simd_unit1} in
(to_i32x8 nre0 (mk_u64 0), to_i32x8 nre0 (mk_u64 2)) ==
inv_ntt_step zeta00 (to_i32x8 re0 (mk_u64 0), to_i32x8 re0 (mk_u64 2)) /\
(to_i32x8 nre0 (mk_u64 1), to_i32x8 nre0 (mk_u64 3)) ==
inv_ntt_step zeta00 (to_i32x8 re0 (mk_u64 1), to_i32x8 re0 (mk_u64 3)) /\
(to_i32x8 nre0 (mk_u64 4), to_i32x8 nre0 (mk_u64 6)) ==
inv_ntt_step zeta01 (to_i32x8 re0 (mk_u64 4), to_i32x8 re0 (mk_u64 6)) /\
(to_i32x8 nre0 (mk_u64 5), to_i32x8 nre0 (mk_u64 7)) ==
inv_ntt_step zeta01 (to_i32x8 re0 (mk_u64 5), to_i32x8 re0 (mk_u64 7)) /\
(to_i32x8 nre1 (mk_u64 0), to_i32x8 nre1 (mk_u64 2)) ==
inv_ntt_step zeta10 (to_i32x8 re1 (mk_u64 0), to_i32x8 re1 (mk_u64 2)) /\
(to_i32x8 nre1 (mk_u64 1), to_i32x8 nre1 (mk_u64 3)) ==
inv_ntt_step zeta10 (to_i32x8 re1 (mk_u64 1), to_i32x8 re1 (mk_u64 3)) /\
(to_i32x8 nre1 (mk_u64 4), to_i32x8 nre1 (mk_u64 6)) ==
inv_ntt_step zeta11 (to_i32x8 re1 (mk_u64 4), to_i32x8 re1 (mk_u64 6)) /\
(to_i32x8 nre1 (mk_u64 5), to_i32x8 nre1 (mk_u64 7)) ==
inv_ntt_step zeta11 (to_i32x8 re1 (mk_u64 5), to_i32x8 re1 (mk_u64 7))
"#))]
fn simd_unit_invert_ntt_at_layer_1(
    simd_unit0: Vec256,
    simd_unit1: Vec256,
    zeta00: i32,
    zeta01: i32,
    zeta10: i32,
    zeta11: i32,
) -> (AVX2SIMDUnit, AVX2SIMDUnit) {
    let mut lo_values = mm256_unpacklo_epi64(simd_unit0, simd_unit1);
    let hi_values = mm256_unpackhi_epi64(simd_unit0, simd_unit1);

    let mut differences = hi_values;
    arithmetic::subtract(&mut differences, &lo_values);
    arithmetic::add(&mut lo_values, &hi_values);
    let sums = lo_values;

    let zetas = mm256_set_epi32(
        zeta11, zeta11, zeta01, zeta01, zeta10, zeta10, zeta00, zeta00,
    );
    arithmetic::montgomery_multiply(&mut differences, &zetas);

    let a = AVX2SIMDUnit {
        value: mm256_unpacklo_epi64(sums, differences),
    };
    let b = AVX2SIMDUnit {
        value: mm256_unpackhi_epi64(sums, differences),
    };

    (a, b)
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|(a, b)| fstar!(r#"
let nre0, nre1 = ${a}.f_value, ${b}.f_value in
let re0, re1 = ${simd_unit0}, ${simd_unit1} in
(to_i32x8 nre0 (mk_u64 0), to_i32x8 nre0 (mk_u64 4)) ==
 inv_ntt_step zeta0 (to_i32x8 re0 (mk_u64 0), to_i32x8 re0 (mk_u64 4)) /\
(to_i32x8 nre0 (mk_u64 1), to_i32x8 nre0 (mk_u64 5)) ==
 inv_ntt_step zeta0 (to_i32x8 re0 (mk_u64 1), to_i32x8 re0 (mk_u64 5)) /\
(to_i32x8 nre0 (mk_u64 2), to_i32x8 nre0 (mk_u64 6)) ==
 inv_ntt_step zeta0 (to_i32x8 re0 (mk_u64 2), to_i32x8 re0 (mk_u64 6)) /\
(to_i32x8 nre0 (mk_u64 3), to_i32x8 nre0 (mk_u64 7)) ==
 inv_ntt_step zeta0 (to_i32x8 re0 (mk_u64 3), to_i32x8 re0 (mk_u64 7)) /\
(to_i32x8 nre1 (mk_u64 0), to_i32x8 nre1 (mk_u64 4)) ==
 inv_ntt_step zeta1 (to_i32x8 re1 (mk_u64 0), to_i32x8 re1 (mk_u64 4)) /\
(to_i32x8 nre1 (mk_u64 1), to_i32x8 nre1 (mk_u64 5)) ==
 inv_ntt_step zeta1 (to_i32x8 re1 (mk_u64 1), to_i32x8 re1 (mk_u64 5)) /\
(to_i32x8 nre1 (mk_u64 2), to_i32x8 nre1 (mk_u64 6)) ==
 inv_ntt_step zeta1 (to_i32x8 re1 (mk_u64 2), to_i32x8 re1 (mk_u64 6)) /\
(to_i32x8 nre1 (mk_u64 3), to_i32x8 nre1 (mk_u64 7)) ==
 inv_ntt_step zeta1 (to_i32x8 re1 (mk_u64 3), to_i32x8 re1 (mk_u64 7))
"#))]
fn simd_unit_invert_ntt_at_layer_2(
    simd_unit0: Vec256,
    simd_unit1: Vec256,
    zeta0: i32,
    zeta1: i32,
) -> (AVX2SIMDUnit, AVX2SIMDUnit) {
    let mut lo_values = mm256_permute2x128_si256::<0x20>(simd_unit0, simd_unit1);
    let hi_values = mm256_permute2x128_si256::<0x31>(simd_unit0, simd_unit1);

    let mut differences = hi_values;
    arithmetic::subtract(&mut differences, &lo_values);
    arithmetic::add(&mut lo_values, &hi_values);
    let sums = lo_values;

    let zetas = mm256_set_epi32(zeta1, zeta1, zeta1, zeta1, zeta0, zeta0, zeta0, zeta0);
    arithmetic::montgomery_multiply(&mut differences, &zetas);

    let a = AVX2SIMDUnit {
        value: mm256_permute2x128_si256::<0x20>(sums, differences),
    };
    let b = AVX2SIMDUnit {
        value: mm256_permute2x128_si256::<0x31>(sums, differences),
    };

    (a, b)
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall4; `%Spec.Utils.forall16 ]] (
   Spec.Utils.forall16 (fun i ->
     let  nre = ${re}_future in
     let  re0 = Seq.index $re (i * 2) in
     let  re1 = Seq.index $re (i * 2 + 1) in
     let nre0 = Seq.index nre (i * 2) in
     let nre1 = Seq.index nre (i * 2 + 1) in
     Spec.Utils.forall4 (fun j ->
       let zeta0 = zeta_r (255 - (i * 8 + j)) in
       let zeta1 = zeta_r (255 - (i * 8 + j + 4)) in
       let j0 = j * 2 in
       let j1 = j0 + 1 in
       (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
        inv_ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
       (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
        inv_ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1))
     )
   )
)
"#))]
unsafe fn invert_ntt_at_layer_0(re: &mut AVX2RingElement) {
    #[inline(always)]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(index < 31)]
    #[hax_lib::ensures(|result| fstar!(r#"
      let r = ${re}_future in
         modifies2_32 $re r $index ($index +! mk_int 1)
      /\ ( let (a, b) = simd_unit_invert_ntt_at_layer_0_ (Seq.index re (v $index)).f_value (Seq.index re (v $index + 1)).f_value 
                            $zeta00 $zeta01 $zeta02 $zeta03 $zeta10 $zeta11 $zeta12 $zeta13 in
           Seq.index r (v $index) == a /\ Seq.index r (v $index + 1) == b)
    "#))]
    fn round(
        re: &mut AVX2RingElement,
        index: usize,
        zeta00: i32,
        zeta01: i32,
        zeta02: i32,
        zeta03: i32,
        zeta10: i32,
        zeta11: i32,
        zeta12: i32,
        zeta13: i32,
    ) {
        (re[index], re[index + 1]) = simd_unit_invert_ntt_at_layer_0(
            re[index].value,
            re[index + 1].value,
            zeta00,
            zeta01,
            zeta02,
            zeta03,
            zeta10,
            zeta11,
            zeta12,
            zeta13,
        );
    }

    round(
        re, 0, 1976782, -846154, 1400424, 3937738, -1362209, -48306, 3919660, -554416,
    );
    round(
        re, 2, -3545687, 1612842, -976891, 183443, -2286327, -420899, -2235985, -2939036,
    );
    round(
        re, 4, -3833893, -260646, -1104333, -1667432, 1910376, -1803090, 1723600, -426683,
    );
    round(
        re, 6, 472078, 1717735, -975884, 2213111, 269760, 3866901, 3523897, -3038916,
    );
    round(
        re, 8, -1799107, -3694233, 1652634, 810149, 3014001, 1616392, 162844, -3183426,
    );
    round(
        re, 10, -1207385, 185531, 3369112, 1957272, -164721, 2454455, 2432395, -2013608,
    );
    round(
        re, 12, -3776993, 594136, -3724270, -2584293, -1846953, -1671176, -2831860, -542412,
    );
    round(
        re, 14, 3406031, 2235880, 777191, 1500165, -1374803, -2546312, 1917081, -1279661,
    );
    round(
        re, 16, -1962642, 3306115, 1312455, -451100, -1430225, -3318210, 1237275, -1333058,
    );
    round(
        re, 18, -1050970, 1903435, 1869119, -2994039, -3548272, 2635921, 1250494, -3767016,
    );
    round(
        re, 20, 1595974, 2486353, 1247620, 4055324, 1265009, -2590150, 2691481, 2842341,
    );
    round(
        re, 22, 203044, 1735879, -3342277, 3437287, 4108315, -2437823, 286988, 342297,
    );
    round(
        re, 24, -3595838, -768622, -525098, -3556995, 3207046, 2031748, -3122442, -655327,
    );
    round(
        re, 26, -522500, -43260, -1613174, 495491, 819034, 909542, 1859098, 900702,
    );
    round(
        re, 28, -3193378, -1197226, -3759364, -3520352, 3513181, -1235728, 2434439, 266997,
    );
    round(
        re, 30, -3562462, -2446433, 2244091, -3342478, 3817976, 2316500, 3407706, 2091667,
    );
}

#[allow(unsafe_code)]
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall4; `%Spec.Utils.forall16 ]] (
   Spec.Utils.forall16 (fun i ->
     let  nre = ${re}_future in
     let  re0 = Seq.index $re (i * 2) in
     let  re1 = Seq.index $re (i * 2 + 1) in
     let nre0 = Seq.index nre (i * 2) in
     let nre1 = Seq.index nre (i * 2 + 1) in
     Spec.Utils.forall4 (fun j ->
         let zeta0 = zeta_r (127 - (i * 4 + j / 2)) in
         let zeta1 = zeta_r (127 - (i * 4 + j / 2 + 2)) in
         let j0 = match j with
           | 0 -> 0 | 1 -> 1
           | 2 -> 4 | 3 -> 5
         in
         let j1 = j0 + 2 in
         (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
          inv_ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
         (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
          inv_ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1))
     )
   )
)
"#))]
unsafe fn invert_ntt_at_layer_1(re: &mut AVX2RingElement) {
    #[inline(always)]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(index < 31)]
    #[hax_lib::ensures(|result| fstar!(r#"
      let r = ${re}_future in
         modifies2_32 re r $index ($index +! mk_int 1)
      /\ ( let (a, b) = simd_unit_invert_ntt_at_layer_1_ (Seq.index re (v $index)).f_value (Seq.index re (v $index + 1)).f_value $zeta_00 $zeta_01 $zeta_10 $zeta_11 in
           Seq.index r (v $index) == a /\ Seq.index r (v $index + 1) == b)
    "#))]
    fn round(
        re: &mut AVX2RingElement,
        index: usize,
        zeta_00: i32,
        zeta_01: i32,
        zeta_10: i32,
        zeta_11: i32,
    ) {
        (re[index], re[index + 1]) = simd_unit_invert_ntt_at_layer_1(
            re[index].value,
            re[index + 1].value,
            zeta_00,
            zeta_01,
            zeta_10,
            zeta_11,
        );
    }

    round(re, 0, 3839961, -3628969, -3881060, -3019102);
    round(re, 2, -1439742, -812732, -1584928, 1285669);
    round(re, 4, 1341330, 1315589, -177440, -2409325);
    round(re, 6, -1851402, 3159746, -3553272, 189548);
    round(re, 8, -1316856, 759969, -210977, 2389356);
    round(re, 10, -3249728, 1653064, -8578, -3724342);
    round(re, 12, 3958618, 904516, -1100098, 44288);
    round(re, 14, 3097992, 508951, 264944, -3343383);
    round(re, 16, -1430430, 1852771, 1349076, -381987);
    round(re, 18, -1308169, -22981, -1228525, -671102);
    round(re, 20, -2477047, -411027, -3693493, -2967645);
    round(re, 22, 2715295, 2147896, -983419, 3412210);
    round(re, 24, 126922, -3632928, -3157330, -3190144);
    round(re, 26, -1000202, -4083598, 1939314, -1257611);
    round(re, 28, -1585221, 2176455, 3475950, -1452451);
    round(re, 30, -3041255, -3677745, -1528703, -3930395);
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall4; `%Spec.Utils.forall16 ]] (
   Spec.Utils.forall16 (fun i ->
     let  nre = ${re}_future in
     let  re0 = Seq.index $re (i * 2) in
     let  re1 = Seq.index $re (i * 2 + 1) in
     let nre0 = Seq.index nre (i * 2) in
     let nre1 = Seq.index nre (i * 2 + 1) in
     Spec.Utils.forall4 (fun j ->
        let zeta0 = zeta_r (63 - (i * 2)) in
        let zeta1 = zeta_r (63 - (i * 2 + 1)) in
        let j0 = j in
        let j1 = j0 + 4 in
        (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
        inv_ntt_step (mk_int zeta0)
          (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
        (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
        inv_ntt_step (mk_int zeta1)
          (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1))
     )
   )
)
"#))]
unsafe fn invert_ntt_at_layer_2(re: &mut AVX2RingElement) {
    #[inline(always)]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(index < 31)]
    #[hax_lib::ensures(|result| fstar!(r#"
      let r = ${re}_future in
         modifies2_32 re r $index ($index +! mk_int 1)
      /\ ( let (a, b) = simd_unit_invert_ntt_at_layer_2_ (Seq.index re (v $index)).f_value (Seq.index re (v $index + 1)).f_value $zeta1 $zeta2 in
           Seq.index r (v $index) == a /\ Seq.index r (v $index + 1) == b)
    "#))]
    fn round(re: &mut AVX2RingElement, index: usize, zeta1: i32, zeta2: i32) {
        (re[index], re[index + 1]) =
            simd_unit_invert_ntt_at_layer_2(re[index].value, re[index + 1].value, zeta1, zeta2);
    }

    round(re, 0, -2797779, 2071892);
    round(re, 2, -2556880, 3900724);
    round(re, 4, 3881043, 954230);
    round(re, 6, 531354, 811944);
    round(re, 8, 3699596, -1600420);
    round(re, 10, -2140649, 3507263);
    round(re, 12, -3821735, 3505694);
    round(re, 14, -1643818, -1699267);
    round(re, 16, -539299, 2348700);
    round(re, 18, -300467, 3539968);
    round(re, 20, -2867647, 3574422);
    round(re, 22, -3043716, -3861115);
    round(re, 24, 3915439, -2537516);
    round(re, 26, -3592148, -1661693);
    round(re, 28, 3530437, 3077325);
    round(re, 30, 95776, 2706023);
}

#[inline(always)]
#[hax_lib::fstar::before(
    r#"
unfold let (∈) (x: nat) ((l, r): (nat & nat)) = x >= l && x < r
unfold let outer_3_plus_inv_pointwise  (offset: nat) (step_by: nat {offset + step_by * 2 <= 32}) (zeta: i32)
    (current_j: nat {current_j ∈ (offset, offset + step_by + 1)})
    (re nre: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) (j: nat{j < 32})
= let interval1 = (offset, current_j) in
  let interval2 = (offset + step_by, current_j + step_by) in
  if j ∈ interval1 then 
    let  re_j = (Seq.index  re j).f_value in
    let nre_j = (Seq.index nre j).f_value in
    let  re_j'= (Seq.index  re (j + step_by)).f_value in
    let nre_j'= (Seq.index nre (j + step_by)).f_value in
    forall i. (to_i32x8 nre_j i, to_i32x8 nre_j' i) == inv_ntt_step zeta (to_i32x8 re_j i, to_i32x8 re_j' i)
  else if j ∈ interval2 then True
  else Seq.index nre j == Seq.index re j

let outer_3_plus_inv
    (offset: nat) (step_by: nat {offset + step_by * 2 <= 32}) (zeta: i32)
    (current_j: nat {current_j ∈ (offset, offset + step_by + 1)})
    (re nre: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
= forall j. outer_3_plus_inv_pointwise offset step_by zeta current_j re nre j
"#
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!("v $OFFSET + v $STEP_BY * 2 <= 32"))]
#[hax_lib::ensures(|result| fstar!(r#"
    outer_3_plus_inv (v $OFFSET) (v $STEP_BY) v_ZETA (v $OFFSET + v $STEP_BY) $re ${re}_future
"#))]
fn outer_3_plus<const OFFSET: usize, const STEP_BY: usize, const ZETA: i32>(
    re: &mut AVX2RingElement,
) {
    #[cfg(hax)]
    let _re0 = re.clone();
    for j in OFFSET..OFFSET + STEP_BY {
        hax_lib::loop_invariant!(|j: usize| fstar!(
            r#"outer_3_plus_inv (v $OFFSET) (v $STEP_BY) $ZETA (v $j) $_re0 $re"#
        ));
        let a_minus_b = mm256_sub_epi32(re[j + STEP_BY].value, re[j].value);
        re[j] = AVX2SIMDUnit {
            value: mm256_add_epi32(re[j].value, re[j + STEP_BY].value),
        };
        re[j + STEP_BY] = AVX2SIMDUnit {
            value: arithmetic::montgomery_multiply_by_constant(a_minus_b, ZETA),
        };
        hax_lib::fstar!("assert (outer_3_plus_inv_pointwise (v $OFFSET) (v $STEP_BY) $ZETA (v $OFFSET + v $STEP_BY) ${_re0} ${re} (v j + v $STEP_BY))");
        ()
    }
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"
let invert_ntt_outer_3_plus_spec
  (layer: nat {layer >= 3 && layer <= 7})
  (re nre: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
  = let zeta_rank = pow2 (8 - layer) - 1 in
    let step_by   = pow2 (layer - 3) in
    let gap       = pow2 (layer - 2) in
    let n         = pow2 (7 - layer) in
    Spec.Utils.forall32 (fun j -> j < n ==> begin
                    let zeta = mk_i32 (zeta_r (zeta_rank - j)) in
                    let j = j * gap in
                    let  re_j = (Seq.index  re j).f_value in
                    let nre_j = (Seq.index nre j).f_value in
                    let  re_j'= (Seq.index  re (j + step_by)).f_value in
                    let nre_j'= (Seq.index nre (j + step_by)).f_value in
                    forall i. (to_i32x8 nre_j i, to_i32x8 nre_j' i) == inv_ntt_step zeta (to_i32x8 re_j i, to_i32x8 re_j' i)
                  end)
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall32 ]] (invert_ntt_outer_3_plus_spec 3 $re ${re}_future)
"#))]
unsafe fn invert_ntt_at_layer_3(re: &mut AVX2RingElement) {
    const STEP: usize = 8; // 1 << LAYER;
    const STEP_BY: usize = 1; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 280005>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 4010497>(re);
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -19422>(re);
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1757237>(re);
    outer_3_plus::<{ (4 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -3277672>(re);
    outer_3_plus::<{ (5 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1399561>(re);
    outer_3_plus::<{ (6 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -3859737>(re);
    outer_3_plus::<{ (7 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2118186>(re);
    outer_3_plus::<{ (8 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2108549>(re);
    outer_3_plus::<{ (9 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2619752>(re);
    outer_3_plus::<{ (10 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1119584>(re);
    outer_3_plus::<{ (11 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -549488>(re);
    outer_3_plus::<{ (12 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3585928>(re);
    outer_3_plus::<{ (13 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1079900>(re);
    outer_3_plus::<{ (14 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1024112>(re);
    outer_3_plus::<{ (15 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2725464>(re);
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall32 ]] (invert_ntt_outer_3_plus_spec 4 $re ${re}_future)
"#))]
unsafe fn invert_ntt_at_layer_4(re: &mut AVX2RingElement) {
    const STEP: usize = 16; // 1 << LAYER;
    const STEP_BY: usize = 2; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2680103>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3111497>(re);
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2884855>(re);
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3119733>(re);
    outer_3_plus::<{ (4 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2091905>(re);
    outer_3_plus::<{ (5 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -359251>(re);
    outer_3_plus::<{ (6 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2353451>(re);
    outer_3_plus::<{ (7 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1826347>(re);
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall32 ]] (invert_ntt_outer_3_plus_spec 5 $re ${re}_future)
"#))]
unsafe fn invert_ntt_at_layer_5(re: &mut AVX2RingElement) {
    const STEP: usize = 32; // 1 << LAYER;
    const STEP_BY: usize = 4; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 466468>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -876248>(re);
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -777960>(re);
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 237124>(re);
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall32 ]] (invert_ntt_outer_3_plus_spec 6 $re ${re}_future)
"#))]
unsafe fn invert_ntt_at_layer_6(re: &mut AVX2RingElement) {
    const STEP: usize = 64; // 1 << LAYER;
    const STEP_BY: usize = 8; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -518909>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2608894>(re);
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall32 ]] (invert_ntt_outer_3_plus_spec 7 $re ${re}_future)
"#))]
unsafe fn invert_ntt_at_layer_7(re: &mut AVX2RingElement) {
    const STEP: usize = 128; // 1 << LAYER;
    const STEP_BY: usize = 16; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 25847>(re);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        polynomial::PolynomialRingElement,
        simd::traits::{Operations, FIELD_MODULUS, SIMD_UNITS_IN_RING_ELEMENT},
    };

    fn poly_reduce_from_value<SIMDUnit: Operations>(value: i32) -> PolynomialRingElement<SIMDUnit> {
        let mut re = PolynomialRingElement::<SIMDUnit>::zero();

        PolynomialRingElement::<SIMDUnit>::from_i32_array(
            &[value; SIMD_UNITS_IN_RING_ELEMENT * COEFFICIENTS_IN_SIMD_UNIT],
            &mut re,
        );

        re.barrett_reduce();

        let _ = core::hint::black_box(SIMDUnit::invert_ntt_montgomery(&mut re.simd_units));

        re
    }

    #[test]
    fn inv_ntt_unreduced_max() {
        let value = FIELD_MODULUS + (FIELD_MODULUS / 1024) + 6;
        let re_portable = poly_reduce_from_value::<crate::simd::portable::PortableSIMDUnit>(value);
        let re_avx2 = poly_reduce_from_value::<AVX2SIMDUnit>(value);
        assert_eq!(re_portable.to_i32_array(), re_avx2.to_i32_array());
    }

    #[test]
    fn inv_ntt_reduced() {
        let value = FIELD_MODULUS + (FIELD_MODULUS / 1024) + 7;
        let re_portable = poly_reduce_from_value::<crate::simd::portable::PortableSIMDUnit>(value);
        let re_avx2 = poly_reduce_from_value::<AVX2SIMDUnit>(value);
        assert_eq!(re_portable.to_i32_array(), re_avx2.to_i32_array());
    }

    #[test]
    fn inv_ntt_reduced_large() {
        let value = FIELD_MODULUS * 8;
        let re_portable = poly_reduce_from_value::<crate::simd::portable::PortableSIMDUnit>(value);
        let re_avx2 = poly_reduce_from_value::<AVX2SIMDUnit>(value);
        assert_eq!(re_portable.to_i32_array(), re_avx2.to_i32_array());
    }
}

use super::{arithmetic, AVX2RingElement};
use crate::simd::{avx2::AVX2SIMDUnit, traits::COEFFICIENTS_IN_SIMD_UNIT};

use libcrux_intrinsics::avx2::*;

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::verification_status(panic_free)]
pub(crate) fn invert_ntt_montgomery(re: &mut AVX2RingElement) {
    #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
    #[allow(unsafe_code)]
    #[hax_lib::fstar::verification_status(panic_free)]
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
#[hax_lib::fstar::before(r"open Spec.MLDSA.NttConstants")]
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
  = let zeta_rank = (if layer = 3 then 31 else if layer = 4 then 15 else if layer = 5 then 7 else if layer = 6 then 3 else 1) in
    let step_by   = (if layer = 3 then 1  else if layer = 4 then 2  else if layer = 5 then 4 else if layer = 6 then 8  else 16) in
    let gap       = (if layer = 3 then 2  else if layer = 4 then 4  else if layer = 5 then 8 else if layer = 6 then 16 else 32) in
    Spec.Utils.forall32 (fun j -> j < 16 ==> begin
                    let w = j / step_by in
                    let l = j % step_by in
                    let zeta = mk_i32 (zeta_r (zeta_rank - w)) in
                    let u = w * gap + l in
                    let  re_j = (Seq.index  re u).f_value in
                    let nre_j = (Seq.index nre u).f_value in
                    let  re_j'= (Seq.index  re (u + step_by)).f_value in
                    let nre_j'= (Seq.index nre (u + step_by)).f_value in
                    forall i. (to_i32x8 nre_j i, to_i32x8 nre_j' i) == inv_ntt_step zeta (to_i32x8 re_j i, to_i32x8 re_j' i)
                  end)
"#)]
#[hax_lib::fstar::before(r#"
(* Clean-context assembly: the 16 per-pair ground facts (proven in the function
   body) imply the layer-3 spec.  Hoisted out of the function so the assembly never
   runs inside the 16-let-shadowed heavy WP (which saturates rlimit 400). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 800 --z3refresh"
let lemma_inv_l3_avx2_assemble
      (orig fin: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        (forall i. (to_i32x8 (Seq.index fin 0).f_value i, to_i32x8 (Seq.index fin 1).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 31)) (to_i32x8 (Seq.index orig 0).f_value i, to_i32x8 (Seq.index orig 1).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 2).f_value i, to_i32x8 (Seq.index fin 3).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 30)) (to_i32x8 (Seq.index orig 2).f_value i, to_i32x8 (Seq.index orig 3).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 4).f_value i, to_i32x8 (Seq.index fin 5).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 29)) (to_i32x8 (Seq.index orig 4).f_value i, to_i32x8 (Seq.index orig 5).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 6).f_value i, to_i32x8 (Seq.index fin 7).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 28)) (to_i32x8 (Seq.index orig 6).f_value i, to_i32x8 (Seq.index orig 7).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 8).f_value i, to_i32x8 (Seq.index fin 9).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 27)) (to_i32x8 (Seq.index orig 8).f_value i, to_i32x8 (Seq.index orig 9).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 10).f_value i, to_i32x8 (Seq.index fin 11).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 26)) (to_i32x8 (Seq.index orig 10).f_value i, to_i32x8 (Seq.index orig 11).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 12).f_value i, to_i32x8 (Seq.index fin 13).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 25)) (to_i32x8 (Seq.index orig 12).f_value i, to_i32x8 (Seq.index orig 13).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 14).f_value i, to_i32x8 (Seq.index fin 15).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 24)) (to_i32x8 (Seq.index orig 14).f_value i, to_i32x8 (Seq.index orig 15).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 16).f_value i, to_i32x8 (Seq.index fin 17).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 23)) (to_i32x8 (Seq.index orig 16).f_value i, to_i32x8 (Seq.index orig 17).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 18).f_value i, to_i32x8 (Seq.index fin 19).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 22)) (to_i32x8 (Seq.index orig 18).f_value i, to_i32x8 (Seq.index orig 19).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 20).f_value i, to_i32x8 (Seq.index fin 21).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 21)) (to_i32x8 (Seq.index orig 20).f_value i, to_i32x8 (Seq.index orig 21).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 22).f_value i, to_i32x8 (Seq.index fin 23).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 20)) (to_i32x8 (Seq.index orig 22).f_value i, to_i32x8 (Seq.index orig 23).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 24).f_value i, to_i32x8 (Seq.index fin 25).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 19)) (to_i32x8 (Seq.index orig 24).f_value i, to_i32x8 (Seq.index orig 25).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 26).f_value i, to_i32x8 (Seq.index fin 27).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 18)) (to_i32x8 (Seq.index orig 26).f_value i, to_i32x8 (Seq.index orig 27).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 28).f_value i, to_i32x8 (Seq.index fin 29).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 17)) (to_i32x8 (Seq.index orig 28).f_value i, to_i32x8 (Seq.index orig 29).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 30).f_value i, to_i32x8 (Seq.index fin 31).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 16)) (to_i32x8 (Seq.index orig 30).f_value i, to_i32x8 (Seq.index orig 31).f_value i)))
      (ensures
        norm [primops; iota; delta_namespace [`%zeta_r; `%Spec.Utils.forall32]]
          (invert_ntt_outer_3_plus_spec 3 orig fin))
  = ()
#pop-options
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall32 ]] (invert_ntt_outer_3_plus_spec 3 $re ${re}_future)
"#))]
unsafe fn invert_ntt_at_layer_3(re: &mut AVX2RingElement) {
    const STEP: usize = 8; // 1 << LAYER;
    const STEP_BY: usize = 1; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

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

    hax_lib::fstar!(r#"
    assert_norm (pow2 0 == 1);
    assert_norm (pow2 1 == 2);
    assert_norm (pow2 4 == 16);
    assert_norm (pow2 5 == 32);
    assert_norm (zeta_r 31 == 280005);
    assert_norm (zeta_r 30 == 4010497);
    assert_norm (zeta_r 29 == (-19422));
    assert_norm (zeta_r 28 == 1757237);
    assert_norm (zeta_r 27 == (-3277672));
    assert_norm (zeta_r 26 == (-1399561));
    assert_norm (zeta_r 25 == (-3859737));
    assert_norm (zeta_r 24 == (-2118186));
    assert_norm (zeta_r 23 == (-2108549));
    assert_norm (zeta_r 22 == 2619752);
    assert_norm (zeta_r 21 == (-1119584));
    assert_norm (zeta_r 20 == (-549488));
    assert_norm (zeta_r 19 == 3585928);
    assert_norm (zeta_r 18 == (-1079900));
    assert_norm (zeta_r 17 == 1024112);
    assert_norm (zeta_r 16 == 2725464);
    assert (forall i. (to_i32x8 (Seq.index ${re} 0).f_value i, to_i32x8 (Seq.index ${re} 1).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 31)) (to_i32x8 (Seq.index ${orig_re} 0).f_value i, to_i32x8 (Seq.index ${orig_re} 1).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 2).f_value i, to_i32x8 (Seq.index ${re} 3).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 30)) (to_i32x8 (Seq.index ${orig_re} 2).f_value i, to_i32x8 (Seq.index ${orig_re} 3).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 4).f_value i, to_i32x8 (Seq.index ${re} 5).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 29)) (to_i32x8 (Seq.index ${orig_re} 4).f_value i, to_i32x8 (Seq.index ${orig_re} 5).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 6).f_value i, to_i32x8 (Seq.index ${re} 7).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 28)) (to_i32x8 (Seq.index ${orig_re} 6).f_value i, to_i32x8 (Seq.index ${orig_re} 7).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 8).f_value i, to_i32x8 (Seq.index ${re} 9).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 27)) (to_i32x8 (Seq.index ${orig_re} 8).f_value i, to_i32x8 (Seq.index ${orig_re} 9).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 10).f_value i, to_i32x8 (Seq.index ${re} 11).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 26)) (to_i32x8 (Seq.index ${orig_re} 10).f_value i, to_i32x8 (Seq.index ${orig_re} 11).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 12).f_value i, to_i32x8 (Seq.index ${re} 13).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 25)) (to_i32x8 (Seq.index ${orig_re} 12).f_value i, to_i32x8 (Seq.index ${orig_re} 13).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 14).f_value i, to_i32x8 (Seq.index ${re} 15).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 24)) (to_i32x8 (Seq.index ${orig_re} 14).f_value i, to_i32x8 (Seq.index ${orig_re} 15).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 16).f_value i, to_i32x8 (Seq.index ${re} 17).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 23)) (to_i32x8 (Seq.index ${orig_re} 16).f_value i, to_i32x8 (Seq.index ${orig_re} 17).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 18).f_value i, to_i32x8 (Seq.index ${re} 19).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 22)) (to_i32x8 (Seq.index ${orig_re} 18).f_value i, to_i32x8 (Seq.index ${orig_re} 19).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 20).f_value i, to_i32x8 (Seq.index ${re} 21).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 21)) (to_i32x8 (Seq.index ${orig_re} 20).f_value i, to_i32x8 (Seq.index ${orig_re} 21).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 22).f_value i, to_i32x8 (Seq.index ${re} 23).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 20)) (to_i32x8 (Seq.index ${orig_re} 22).f_value i, to_i32x8 (Seq.index ${orig_re} 23).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 24).f_value i, to_i32x8 (Seq.index ${re} 25).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 19)) (to_i32x8 (Seq.index ${orig_re} 24).f_value i, to_i32x8 (Seq.index ${orig_re} 25).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 26).f_value i, to_i32x8 (Seq.index ${re} 27).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 18)) (to_i32x8 (Seq.index ${orig_re} 26).f_value i, to_i32x8 (Seq.index ${orig_re} 27).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 28).f_value i, to_i32x8 (Seq.index ${re} 29).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 17)) (to_i32x8 (Seq.index ${orig_re} 28).f_value i, to_i32x8 (Seq.index ${orig_re} 29).f_value i));
    assert (forall i. (to_i32x8 (Seq.index ${re} 30).f_value i, to_i32x8 (Seq.index ${re} 31).f_value i) ==
      inv_ntt_step (mk_i32 (zeta_r 16)) (to_i32x8 (Seq.index ${orig_re} 30).f_value i, to_i32x8 (Seq.index ${orig_re} 31).f_value i));
    lemma_inv_l3_avx2_assemble ${orig_re} ${re}
    "#);
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"
(* Clean-context assembly: the 16 per-pair ground facts (proven in the function
   body) imply the layer-4 spec (gap 4, step_by 2: windows (4j, 4j+2)). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 800 --z3refresh"
let lemma_inv_l4_avx2_assemble
      (orig fin: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        (forall i. (to_i32x8 (Seq.index fin 0).f_value i, to_i32x8 (Seq.index fin 2).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 15)) (to_i32x8 (Seq.index orig 0).f_value i, to_i32x8 (Seq.index orig 2).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 1).f_value i, to_i32x8 (Seq.index fin 3).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 15)) (to_i32x8 (Seq.index orig 1).f_value i, to_i32x8 (Seq.index orig 3).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 4).f_value i, to_i32x8 (Seq.index fin 6).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 14)) (to_i32x8 (Seq.index orig 4).f_value i, to_i32x8 (Seq.index orig 6).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 5).f_value i, to_i32x8 (Seq.index fin 7).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 14)) (to_i32x8 (Seq.index orig 5).f_value i, to_i32x8 (Seq.index orig 7).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 8).f_value i, to_i32x8 (Seq.index fin 10).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 13)) (to_i32x8 (Seq.index orig 8).f_value i, to_i32x8 (Seq.index orig 10).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 9).f_value i, to_i32x8 (Seq.index fin 11).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 13)) (to_i32x8 (Seq.index orig 9).f_value i, to_i32x8 (Seq.index orig 11).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 12).f_value i, to_i32x8 (Seq.index fin 14).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 12)) (to_i32x8 (Seq.index orig 12).f_value i, to_i32x8 (Seq.index orig 14).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 13).f_value i, to_i32x8 (Seq.index fin 15).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 12)) (to_i32x8 (Seq.index orig 13).f_value i, to_i32x8 (Seq.index orig 15).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 16).f_value i, to_i32x8 (Seq.index fin 18).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 11)) (to_i32x8 (Seq.index orig 16).f_value i, to_i32x8 (Seq.index orig 18).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 17).f_value i, to_i32x8 (Seq.index fin 19).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 11)) (to_i32x8 (Seq.index orig 17).f_value i, to_i32x8 (Seq.index orig 19).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 20).f_value i, to_i32x8 (Seq.index fin 22).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 10)) (to_i32x8 (Seq.index orig 20).f_value i, to_i32x8 (Seq.index orig 22).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 21).f_value i, to_i32x8 (Seq.index fin 23).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 10)) (to_i32x8 (Seq.index orig 21).f_value i, to_i32x8 (Seq.index orig 23).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 24).f_value i, to_i32x8 (Seq.index fin 26).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 9)) (to_i32x8 (Seq.index orig 24).f_value i, to_i32x8 (Seq.index orig 26).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 25).f_value i, to_i32x8 (Seq.index fin 27).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 9)) (to_i32x8 (Seq.index orig 25).f_value i, to_i32x8 (Seq.index orig 27).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 28).f_value i, to_i32x8 (Seq.index fin 30).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 8)) (to_i32x8 (Seq.index orig 28).f_value i, to_i32x8 (Seq.index orig 30).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 29).f_value i, to_i32x8 (Seq.index fin 31).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 8)) (to_i32x8 (Seq.index orig 29).f_value i, to_i32x8 (Seq.index orig 31).f_value i)))
      (ensures
        norm [primops; iota; delta_namespace [`%zeta_r; `%Spec.Utils.forall32]]
          (invert_ntt_outer_3_plus_spec 4 orig fin))
  = ()
#pop-options
"#)]
#[hax_lib::fstar::before(r#"
(* Clean-context TRANSPORT for layer 4: from the eight raw outer_3_plus call posts
   (call w at offset 4w, step_by 2, with intermediate states s1..s7) derive the 16
   orig->fin per-pair facts.  The eight calls are DISJOINT (call w only touches units
   4w..4w+3); so for any unit u=4w+l (l<2) the orig->fin pair fact reduces to call
   w's own post pair (orig[..]==s_w[..] via earlier posts' else-branch frame;
   fin[..]==s_{w+1}[..] via later posts' else-branch frame).  Per-call step lemmas
   over a CONCRETE w each (parametric-w framing over the other 7 posts would have to
   decide the (∈)-ladder symbolically and saturates); each materialises both of its
   call's pairs.  fuel 1 unfolds outer_3_plus_inv to its `forall j`; ifuel 2 evaluates
   the (∈) if-ladder.  Monolithic 800 (mirror the assemble/L5/L6-transport lemmas). *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 800 --z3refresh"
let lemma_inv_l4_transport
      (orig s1 s2 s3 s4 s5 s6 s7 fin: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        outer_3_plus_inv 0  2 (mk_i32 2680103)    2  orig s1 /\
        outer_3_plus_inv 4  2 (mk_i32 3111497)    6  s1 s2 /\
        outer_3_plus_inv 8  2 (mk_i32 (-2884855)) 10 s2 s3 /\
        outer_3_plus_inv 12 2 (mk_i32 3119733)    14 s3 s4 /\
        outer_3_plus_inv 16 2 (mk_i32 (-2091905)) 18 s4 s5 /\
        outer_3_plus_inv 20 2 (mk_i32 (-359251))  22 s5 s6 /\
        outer_3_plus_inv 24 2 (mk_i32 2353451)    26 s6 s7 /\
        outer_3_plus_inv 28 2 (mk_i32 1826347)    30 s7 fin)
      (ensures
        (forall i. (to_i32x8 (Seq.index fin 0).f_value i, to_i32x8 (Seq.index fin 2).f_value i) ==
           inv_ntt_step (mk_i32 2680103) (to_i32x8 (Seq.index orig 0).f_value i, to_i32x8 (Seq.index orig 2).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 1).f_value i, to_i32x8 (Seq.index fin 3).f_value i) ==
           inv_ntt_step (mk_i32 2680103) (to_i32x8 (Seq.index orig 1).f_value i, to_i32x8 (Seq.index orig 3).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 4).f_value i, to_i32x8 (Seq.index fin 6).f_value i) ==
           inv_ntt_step (mk_i32 3111497) (to_i32x8 (Seq.index orig 4).f_value i, to_i32x8 (Seq.index orig 6).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 5).f_value i, to_i32x8 (Seq.index fin 7).f_value i) ==
           inv_ntt_step (mk_i32 3111497) (to_i32x8 (Seq.index orig 5).f_value i, to_i32x8 (Seq.index orig 7).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 8).f_value i, to_i32x8 (Seq.index fin 10).f_value i) ==
           inv_ntt_step (mk_i32 (-2884855)) (to_i32x8 (Seq.index orig 8).f_value i, to_i32x8 (Seq.index orig 10).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 9).f_value i, to_i32x8 (Seq.index fin 11).f_value i) ==
           inv_ntt_step (mk_i32 (-2884855)) (to_i32x8 (Seq.index orig 9).f_value i, to_i32x8 (Seq.index orig 11).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 12).f_value i, to_i32x8 (Seq.index fin 14).f_value i) ==
           inv_ntt_step (mk_i32 3119733) (to_i32x8 (Seq.index orig 12).f_value i, to_i32x8 (Seq.index orig 14).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 13).f_value i, to_i32x8 (Seq.index fin 15).f_value i) ==
           inv_ntt_step (mk_i32 3119733) (to_i32x8 (Seq.index orig 13).f_value i, to_i32x8 (Seq.index orig 15).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 16).f_value i, to_i32x8 (Seq.index fin 18).f_value i) ==
           inv_ntt_step (mk_i32 (-2091905)) (to_i32x8 (Seq.index orig 16).f_value i, to_i32x8 (Seq.index orig 18).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 17).f_value i, to_i32x8 (Seq.index fin 19).f_value i) ==
           inv_ntt_step (mk_i32 (-2091905)) (to_i32x8 (Seq.index orig 17).f_value i, to_i32x8 (Seq.index orig 19).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 20).f_value i, to_i32x8 (Seq.index fin 22).f_value i) ==
           inv_ntt_step (mk_i32 (-359251)) (to_i32x8 (Seq.index orig 20).f_value i, to_i32x8 (Seq.index orig 22).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 21).f_value i, to_i32x8 (Seq.index fin 23).f_value i) ==
           inv_ntt_step (mk_i32 (-359251)) (to_i32x8 (Seq.index orig 21).f_value i, to_i32x8 (Seq.index orig 23).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 24).f_value i, to_i32x8 (Seq.index fin 26).f_value i) ==
           inv_ntt_step (mk_i32 2353451) (to_i32x8 (Seq.index orig 24).f_value i, to_i32x8 (Seq.index orig 26).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 25).f_value i, to_i32x8 (Seq.index fin 27).f_value i) ==
           inv_ntt_step (mk_i32 2353451) (to_i32x8 (Seq.index orig 25).f_value i, to_i32x8 (Seq.index orig 27).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 28).f_value i, to_i32x8 (Seq.index fin 30).f_value i) ==
           inv_ntt_step (mk_i32 1826347) (to_i32x8 (Seq.index orig 28).f_value i, to_i32x8 (Seq.index orig 30).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 29).f_value i, to_i32x8 (Seq.index fin 31).f_value i) ==
           inv_ntt_step (mk_i32 1826347) (to_i32x8 (Seq.index orig 29).f_value i, to_i32x8 (Seq.index orig 31).f_value i))) =
  let elim (k: nat{k < 8}) (sk sk1: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) (zk: i32) (u: nat{u < 32})
      : Lemma (requires outer_3_plus_inv (4 * k) 2 zk (4 * k + 2) sk sk1)
              (ensures outer_3_plus_inv_pointwise (4 * k) 2 zk (4 * k + 2) sk sk1 u) =
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise (4 * k) 2 zk (4 * k + 2) sk sk1 j with u
  in
  let step0 (_: unit)
      : Lemma
        ((forall i. (to_i32x8 (Seq.index fin 0).f_value i, to_i32x8 (Seq.index fin 2).f_value i) ==
            inv_ntt_step (mk_i32 2680103) (to_i32x8 (Seq.index orig 0).f_value i, to_i32x8 (Seq.index orig 2).f_value i)) /\
         (forall i. (to_i32x8 (Seq.index fin 1).f_value i, to_i32x8 (Seq.index fin 3).f_value i) ==
            inv_ntt_step (mk_i32 2680103) (to_i32x8 (Seq.index orig 1).f_value i, to_i32x8 (Seq.index orig 3).f_value i))) =
    elim 0 orig s1 (mk_i32 2680103) 0; elim 0 orig s1 (mk_i32 2680103) 1; elim 0 orig s1 (mk_i32 2680103) 2; elim 0 orig s1 (mk_i32 2680103) 3;
    elim 1 s1 s2 (mk_i32 3111497) 0; elim 1 s1 s2 (mk_i32 3111497) 1; elim 1 s1 s2 (mk_i32 3111497) 2; elim 1 s1 s2 (mk_i32 3111497) 3;
    elim 2 s2 s3 (mk_i32 (-2884855)) 0; elim 2 s2 s3 (mk_i32 (-2884855)) 1; elim 2 s2 s3 (mk_i32 (-2884855)) 2; elim 2 s2 s3 (mk_i32 (-2884855)) 3;
    elim 3 s3 s4 (mk_i32 3119733) 0; elim 3 s3 s4 (mk_i32 3119733) 1; elim 3 s3 s4 (mk_i32 3119733) 2; elim 3 s3 s4 (mk_i32 3119733) 3;
    elim 4 s4 s5 (mk_i32 (-2091905)) 0; elim 4 s4 s5 (mk_i32 (-2091905)) 1; elim 4 s4 s5 (mk_i32 (-2091905)) 2; elim 4 s4 s5 (mk_i32 (-2091905)) 3;
    elim 5 s5 s6 (mk_i32 (-359251)) 0; elim 5 s5 s6 (mk_i32 (-359251)) 1; elim 5 s5 s6 (mk_i32 (-359251)) 2; elim 5 s5 s6 (mk_i32 (-359251)) 3;
    elim 6 s6 s7 (mk_i32 2353451) 0; elim 6 s6 s7 (mk_i32 2353451) 1; elim 6 s6 s7 (mk_i32 2353451) 2; elim 6 s6 s7 (mk_i32 2353451) 3;
    elim 7 s7 fin (mk_i32 1826347) 0; elim 7 s7 fin (mk_i32 1826347) 1; elim 7 s7 fin (mk_i32 1826347) 2; elim 7 s7 fin (mk_i32 1826347) 3
  in
  let step1 (_: unit)
      : Lemma
        ((forall i. (to_i32x8 (Seq.index fin 4).f_value i, to_i32x8 (Seq.index fin 6).f_value i) ==
            inv_ntt_step (mk_i32 3111497) (to_i32x8 (Seq.index orig 4).f_value i, to_i32x8 (Seq.index orig 6).f_value i)) /\
         (forall i. (to_i32x8 (Seq.index fin 5).f_value i, to_i32x8 (Seq.index fin 7).f_value i) ==
            inv_ntt_step (mk_i32 3111497) (to_i32x8 (Seq.index orig 5).f_value i, to_i32x8 (Seq.index orig 7).f_value i))) =
    elim 0 orig s1 (mk_i32 2680103) 4; elim 0 orig s1 (mk_i32 2680103) 5; elim 0 orig s1 (mk_i32 2680103) 6; elim 0 orig s1 (mk_i32 2680103) 7;
    elim 1 s1 s2 (mk_i32 3111497) 4; elim 1 s1 s2 (mk_i32 3111497) 5; elim 1 s1 s2 (mk_i32 3111497) 6; elim 1 s1 s2 (mk_i32 3111497) 7;
    elim 2 s2 s3 (mk_i32 (-2884855)) 4; elim 2 s2 s3 (mk_i32 (-2884855)) 5; elim 2 s2 s3 (mk_i32 (-2884855)) 6; elim 2 s2 s3 (mk_i32 (-2884855)) 7;
    elim 3 s3 s4 (mk_i32 3119733) 4; elim 3 s3 s4 (mk_i32 3119733) 5; elim 3 s3 s4 (mk_i32 3119733) 6; elim 3 s3 s4 (mk_i32 3119733) 7;
    elim 4 s4 s5 (mk_i32 (-2091905)) 4; elim 4 s4 s5 (mk_i32 (-2091905)) 5; elim 4 s4 s5 (mk_i32 (-2091905)) 6; elim 4 s4 s5 (mk_i32 (-2091905)) 7;
    elim 5 s5 s6 (mk_i32 (-359251)) 4; elim 5 s5 s6 (mk_i32 (-359251)) 5; elim 5 s5 s6 (mk_i32 (-359251)) 6; elim 5 s5 s6 (mk_i32 (-359251)) 7;
    elim 6 s6 s7 (mk_i32 2353451) 4; elim 6 s6 s7 (mk_i32 2353451) 5; elim 6 s6 s7 (mk_i32 2353451) 6; elim 6 s6 s7 (mk_i32 2353451) 7;
    elim 7 s7 fin (mk_i32 1826347) 4; elim 7 s7 fin (mk_i32 1826347) 5; elim 7 s7 fin (mk_i32 1826347) 6; elim 7 s7 fin (mk_i32 1826347) 7
  in
  let step2 (_: unit)
      : Lemma
        ((forall i. (to_i32x8 (Seq.index fin 8).f_value i, to_i32x8 (Seq.index fin 10).f_value i) ==
            inv_ntt_step (mk_i32 (-2884855)) (to_i32x8 (Seq.index orig 8).f_value i, to_i32x8 (Seq.index orig 10).f_value i)) /\
         (forall i. (to_i32x8 (Seq.index fin 9).f_value i, to_i32x8 (Seq.index fin 11).f_value i) ==
            inv_ntt_step (mk_i32 (-2884855)) (to_i32x8 (Seq.index orig 9).f_value i, to_i32x8 (Seq.index orig 11).f_value i))) =
    elim 0 orig s1 (mk_i32 2680103) 8; elim 0 orig s1 (mk_i32 2680103) 9; elim 0 orig s1 (mk_i32 2680103) 10; elim 0 orig s1 (mk_i32 2680103) 11;
    elim 1 s1 s2 (mk_i32 3111497) 8; elim 1 s1 s2 (mk_i32 3111497) 9; elim 1 s1 s2 (mk_i32 3111497) 10; elim 1 s1 s2 (mk_i32 3111497) 11;
    elim 2 s2 s3 (mk_i32 (-2884855)) 8; elim 2 s2 s3 (mk_i32 (-2884855)) 9; elim 2 s2 s3 (mk_i32 (-2884855)) 10; elim 2 s2 s3 (mk_i32 (-2884855)) 11;
    elim 3 s3 s4 (mk_i32 3119733) 8; elim 3 s3 s4 (mk_i32 3119733) 9; elim 3 s3 s4 (mk_i32 3119733) 10; elim 3 s3 s4 (mk_i32 3119733) 11;
    elim 4 s4 s5 (mk_i32 (-2091905)) 8; elim 4 s4 s5 (mk_i32 (-2091905)) 9; elim 4 s4 s5 (mk_i32 (-2091905)) 10; elim 4 s4 s5 (mk_i32 (-2091905)) 11;
    elim 5 s5 s6 (mk_i32 (-359251)) 8; elim 5 s5 s6 (mk_i32 (-359251)) 9; elim 5 s5 s6 (mk_i32 (-359251)) 10; elim 5 s5 s6 (mk_i32 (-359251)) 11;
    elim 6 s6 s7 (mk_i32 2353451) 8; elim 6 s6 s7 (mk_i32 2353451) 9; elim 6 s6 s7 (mk_i32 2353451) 10; elim 6 s6 s7 (mk_i32 2353451) 11;
    elim 7 s7 fin (mk_i32 1826347) 8; elim 7 s7 fin (mk_i32 1826347) 9; elim 7 s7 fin (mk_i32 1826347) 10; elim 7 s7 fin (mk_i32 1826347) 11
  in
  let step3 (_: unit)
      : Lemma
        ((forall i. (to_i32x8 (Seq.index fin 12).f_value i, to_i32x8 (Seq.index fin 14).f_value i) ==
            inv_ntt_step (mk_i32 3119733) (to_i32x8 (Seq.index orig 12).f_value i, to_i32x8 (Seq.index orig 14).f_value i)) /\
         (forall i. (to_i32x8 (Seq.index fin 13).f_value i, to_i32x8 (Seq.index fin 15).f_value i) ==
            inv_ntt_step (mk_i32 3119733) (to_i32x8 (Seq.index orig 13).f_value i, to_i32x8 (Seq.index orig 15).f_value i))) =
    elim 0 orig s1 (mk_i32 2680103) 12; elim 0 orig s1 (mk_i32 2680103) 13; elim 0 orig s1 (mk_i32 2680103) 14; elim 0 orig s1 (mk_i32 2680103) 15;
    elim 1 s1 s2 (mk_i32 3111497) 12; elim 1 s1 s2 (mk_i32 3111497) 13; elim 1 s1 s2 (mk_i32 3111497) 14; elim 1 s1 s2 (mk_i32 3111497) 15;
    elim 2 s2 s3 (mk_i32 (-2884855)) 12; elim 2 s2 s3 (mk_i32 (-2884855)) 13; elim 2 s2 s3 (mk_i32 (-2884855)) 14; elim 2 s2 s3 (mk_i32 (-2884855)) 15;
    elim 3 s3 s4 (mk_i32 3119733) 12; elim 3 s3 s4 (mk_i32 3119733) 13; elim 3 s3 s4 (mk_i32 3119733) 14; elim 3 s3 s4 (mk_i32 3119733) 15;
    elim 4 s4 s5 (mk_i32 (-2091905)) 12; elim 4 s4 s5 (mk_i32 (-2091905)) 13; elim 4 s4 s5 (mk_i32 (-2091905)) 14; elim 4 s4 s5 (mk_i32 (-2091905)) 15;
    elim 5 s5 s6 (mk_i32 (-359251)) 12; elim 5 s5 s6 (mk_i32 (-359251)) 13; elim 5 s5 s6 (mk_i32 (-359251)) 14; elim 5 s5 s6 (mk_i32 (-359251)) 15;
    elim 6 s6 s7 (mk_i32 2353451) 12; elim 6 s6 s7 (mk_i32 2353451) 13; elim 6 s6 s7 (mk_i32 2353451) 14; elim 6 s6 s7 (mk_i32 2353451) 15;
    elim 7 s7 fin (mk_i32 1826347) 12; elim 7 s7 fin (mk_i32 1826347) 13; elim 7 s7 fin (mk_i32 1826347) 14; elim 7 s7 fin (mk_i32 1826347) 15
  in
  let step4 (_: unit)
      : Lemma
        ((forall i. (to_i32x8 (Seq.index fin 16).f_value i, to_i32x8 (Seq.index fin 18).f_value i) ==
            inv_ntt_step (mk_i32 (-2091905)) (to_i32x8 (Seq.index orig 16).f_value i, to_i32x8 (Seq.index orig 18).f_value i)) /\
         (forall i. (to_i32x8 (Seq.index fin 17).f_value i, to_i32x8 (Seq.index fin 19).f_value i) ==
            inv_ntt_step (mk_i32 (-2091905)) (to_i32x8 (Seq.index orig 17).f_value i, to_i32x8 (Seq.index orig 19).f_value i))) =
    elim 0 orig s1 (mk_i32 2680103) 16; elim 0 orig s1 (mk_i32 2680103) 17; elim 0 orig s1 (mk_i32 2680103) 18; elim 0 orig s1 (mk_i32 2680103) 19;
    elim 1 s1 s2 (mk_i32 3111497) 16; elim 1 s1 s2 (mk_i32 3111497) 17; elim 1 s1 s2 (mk_i32 3111497) 18; elim 1 s1 s2 (mk_i32 3111497) 19;
    elim 2 s2 s3 (mk_i32 (-2884855)) 16; elim 2 s2 s3 (mk_i32 (-2884855)) 17; elim 2 s2 s3 (mk_i32 (-2884855)) 18; elim 2 s2 s3 (mk_i32 (-2884855)) 19;
    elim 3 s3 s4 (mk_i32 3119733) 16; elim 3 s3 s4 (mk_i32 3119733) 17; elim 3 s3 s4 (mk_i32 3119733) 18; elim 3 s3 s4 (mk_i32 3119733) 19;
    elim 4 s4 s5 (mk_i32 (-2091905)) 16; elim 4 s4 s5 (mk_i32 (-2091905)) 17; elim 4 s4 s5 (mk_i32 (-2091905)) 18; elim 4 s4 s5 (mk_i32 (-2091905)) 19;
    elim 5 s5 s6 (mk_i32 (-359251)) 16; elim 5 s5 s6 (mk_i32 (-359251)) 17; elim 5 s5 s6 (mk_i32 (-359251)) 18; elim 5 s5 s6 (mk_i32 (-359251)) 19;
    elim 6 s6 s7 (mk_i32 2353451) 16; elim 6 s6 s7 (mk_i32 2353451) 17; elim 6 s6 s7 (mk_i32 2353451) 18; elim 6 s6 s7 (mk_i32 2353451) 19;
    elim 7 s7 fin (mk_i32 1826347) 16; elim 7 s7 fin (mk_i32 1826347) 17; elim 7 s7 fin (mk_i32 1826347) 18; elim 7 s7 fin (mk_i32 1826347) 19
  in
  let step5 (_: unit)
      : Lemma
        ((forall i. (to_i32x8 (Seq.index fin 20).f_value i, to_i32x8 (Seq.index fin 22).f_value i) ==
            inv_ntt_step (mk_i32 (-359251)) (to_i32x8 (Seq.index orig 20).f_value i, to_i32x8 (Seq.index orig 22).f_value i)) /\
         (forall i. (to_i32x8 (Seq.index fin 21).f_value i, to_i32x8 (Seq.index fin 23).f_value i) ==
            inv_ntt_step (mk_i32 (-359251)) (to_i32x8 (Seq.index orig 21).f_value i, to_i32x8 (Seq.index orig 23).f_value i))) =
    elim 0 orig s1 (mk_i32 2680103) 20; elim 0 orig s1 (mk_i32 2680103) 21; elim 0 orig s1 (mk_i32 2680103) 22; elim 0 orig s1 (mk_i32 2680103) 23;
    elim 1 s1 s2 (mk_i32 3111497) 20; elim 1 s1 s2 (mk_i32 3111497) 21; elim 1 s1 s2 (mk_i32 3111497) 22; elim 1 s1 s2 (mk_i32 3111497) 23;
    elim 2 s2 s3 (mk_i32 (-2884855)) 20; elim 2 s2 s3 (mk_i32 (-2884855)) 21; elim 2 s2 s3 (mk_i32 (-2884855)) 22; elim 2 s2 s3 (mk_i32 (-2884855)) 23;
    elim 3 s3 s4 (mk_i32 3119733) 20; elim 3 s3 s4 (mk_i32 3119733) 21; elim 3 s3 s4 (mk_i32 3119733) 22; elim 3 s3 s4 (mk_i32 3119733) 23;
    elim 4 s4 s5 (mk_i32 (-2091905)) 20; elim 4 s4 s5 (mk_i32 (-2091905)) 21; elim 4 s4 s5 (mk_i32 (-2091905)) 22; elim 4 s4 s5 (mk_i32 (-2091905)) 23;
    elim 5 s5 s6 (mk_i32 (-359251)) 20; elim 5 s5 s6 (mk_i32 (-359251)) 21; elim 5 s5 s6 (mk_i32 (-359251)) 22; elim 5 s5 s6 (mk_i32 (-359251)) 23;
    elim 6 s6 s7 (mk_i32 2353451) 20; elim 6 s6 s7 (mk_i32 2353451) 21; elim 6 s6 s7 (mk_i32 2353451) 22; elim 6 s6 s7 (mk_i32 2353451) 23;
    elim 7 s7 fin (mk_i32 1826347) 20; elim 7 s7 fin (mk_i32 1826347) 21; elim 7 s7 fin (mk_i32 1826347) 22; elim 7 s7 fin (mk_i32 1826347) 23
  in
  let step6 (_: unit)
      : Lemma
        ((forall i. (to_i32x8 (Seq.index fin 24).f_value i, to_i32x8 (Seq.index fin 26).f_value i) ==
            inv_ntt_step (mk_i32 2353451) (to_i32x8 (Seq.index orig 24).f_value i, to_i32x8 (Seq.index orig 26).f_value i)) /\
         (forall i. (to_i32x8 (Seq.index fin 25).f_value i, to_i32x8 (Seq.index fin 27).f_value i) ==
            inv_ntt_step (mk_i32 2353451) (to_i32x8 (Seq.index orig 25).f_value i, to_i32x8 (Seq.index orig 27).f_value i))) =
    elim 0 orig s1 (mk_i32 2680103) 24; elim 0 orig s1 (mk_i32 2680103) 25; elim 0 orig s1 (mk_i32 2680103) 26; elim 0 orig s1 (mk_i32 2680103) 27;
    elim 1 s1 s2 (mk_i32 3111497) 24; elim 1 s1 s2 (mk_i32 3111497) 25; elim 1 s1 s2 (mk_i32 3111497) 26; elim 1 s1 s2 (mk_i32 3111497) 27;
    elim 2 s2 s3 (mk_i32 (-2884855)) 24; elim 2 s2 s3 (mk_i32 (-2884855)) 25; elim 2 s2 s3 (mk_i32 (-2884855)) 26; elim 2 s2 s3 (mk_i32 (-2884855)) 27;
    elim 3 s3 s4 (mk_i32 3119733) 24; elim 3 s3 s4 (mk_i32 3119733) 25; elim 3 s3 s4 (mk_i32 3119733) 26; elim 3 s3 s4 (mk_i32 3119733) 27;
    elim 4 s4 s5 (mk_i32 (-2091905)) 24; elim 4 s4 s5 (mk_i32 (-2091905)) 25; elim 4 s4 s5 (mk_i32 (-2091905)) 26; elim 4 s4 s5 (mk_i32 (-2091905)) 27;
    elim 5 s5 s6 (mk_i32 (-359251)) 24; elim 5 s5 s6 (mk_i32 (-359251)) 25; elim 5 s5 s6 (mk_i32 (-359251)) 26; elim 5 s5 s6 (mk_i32 (-359251)) 27;
    elim 6 s6 s7 (mk_i32 2353451) 24; elim 6 s6 s7 (mk_i32 2353451) 25; elim 6 s6 s7 (mk_i32 2353451) 26; elim 6 s6 s7 (mk_i32 2353451) 27;
    elim 7 s7 fin (mk_i32 1826347) 24; elim 7 s7 fin (mk_i32 1826347) 25; elim 7 s7 fin (mk_i32 1826347) 26; elim 7 s7 fin (mk_i32 1826347) 27
  in
  let step7 (_: unit)
      : Lemma
        ((forall i. (to_i32x8 (Seq.index fin 28).f_value i, to_i32x8 (Seq.index fin 30).f_value i) ==
            inv_ntt_step (mk_i32 1826347) (to_i32x8 (Seq.index orig 28).f_value i, to_i32x8 (Seq.index orig 30).f_value i)) /\
         (forall i. (to_i32x8 (Seq.index fin 29).f_value i, to_i32x8 (Seq.index fin 31).f_value i) ==
            inv_ntt_step (mk_i32 1826347) (to_i32x8 (Seq.index orig 29).f_value i, to_i32x8 (Seq.index orig 31).f_value i))) =
    elim 0 orig s1 (mk_i32 2680103) 28; elim 0 orig s1 (mk_i32 2680103) 29; elim 0 orig s1 (mk_i32 2680103) 30; elim 0 orig s1 (mk_i32 2680103) 31;
    elim 1 s1 s2 (mk_i32 3111497) 28; elim 1 s1 s2 (mk_i32 3111497) 29; elim 1 s1 s2 (mk_i32 3111497) 30; elim 1 s1 s2 (mk_i32 3111497) 31;
    elim 2 s2 s3 (mk_i32 (-2884855)) 28; elim 2 s2 s3 (mk_i32 (-2884855)) 29; elim 2 s2 s3 (mk_i32 (-2884855)) 30; elim 2 s2 s3 (mk_i32 (-2884855)) 31;
    elim 3 s3 s4 (mk_i32 3119733) 28; elim 3 s3 s4 (mk_i32 3119733) 29; elim 3 s3 s4 (mk_i32 3119733) 30; elim 3 s3 s4 (mk_i32 3119733) 31;
    elim 4 s4 s5 (mk_i32 (-2091905)) 28; elim 4 s4 s5 (mk_i32 (-2091905)) 29; elim 4 s4 s5 (mk_i32 (-2091905)) 30; elim 4 s4 s5 (mk_i32 (-2091905)) 31;
    elim 5 s5 s6 (mk_i32 (-359251)) 28; elim 5 s5 s6 (mk_i32 (-359251)) 29; elim 5 s5 s6 (mk_i32 (-359251)) 30; elim 5 s5 s6 (mk_i32 (-359251)) 31;
    elim 6 s6 s7 (mk_i32 2353451) 28; elim 6 s6 s7 (mk_i32 2353451) 29; elim 6 s6 s7 (mk_i32 2353451) 30; elim 6 s6 s7 (mk_i32 2353451) 31;
    elim 7 s7 fin (mk_i32 1826347) 28; elim 7 s7 fin (mk_i32 1826347) 29; elim 7 s7 fin (mk_i32 1826347) 30; elim 7 s7 fin (mk_i32 1826347) 31
  in
  step0 (); step1 (); step2 (); step3 (); step4 (); step5 (); step6 (); step7 ()
#pop-options
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall32 ]] (invert_ntt_outer_3_plus_spec 4 $re ${re}_future)
"#))]
unsafe fn invert_ntt_at_layer_4(re: &mut AVX2RingElement) {
    const STEP: usize = 16; // 1 << LAYER;
    const STEP_BY: usize = 2; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2680103>(re);
    #[cfg(hax)]
    let s1 = re.clone();
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3111497>(re);
    #[cfg(hax)]
    let s2 = re.clone();
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2884855>(re);
    #[cfg(hax)]
    let s3 = re.clone();
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3119733>(re);
    #[cfg(hax)]
    let s4 = re.clone();
    outer_3_plus::<{ (4 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2091905>(re);
    #[cfg(hax)]
    let s5 = re.clone();
    outer_3_plus::<{ (5 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -359251>(re);
    #[cfg(hax)]
    let s6 = re.clone();
    outer_3_plus::<{ (6 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2353451>(re);
    #[cfg(hax)]
    let s7 = re.clone();
    outer_3_plus::<{ (7 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1826347>(re);

    hax_lib::fstar!(r#"
    assert_norm (zeta_r 15 == 2680103);
    assert_norm (zeta_r 14 == 3111497);
    assert_norm (zeta_r 13 == (-2884855));
    assert_norm (zeta_r 12 == 3119733);
    assert_norm (zeta_r 11 == (-2091905));
    assert_norm (zeta_r 10 == (-359251));
    assert_norm (zeta_r 9 == 2353451);
    assert_norm (zeta_r 8 == 1826347);
    lemma_inv_l4_transport ${orig_re} ${s1} ${s2} ${s3} ${s4} ${s5} ${s6} ${s7} ${re};
    lemma_inv_l4_avx2_assemble ${orig_re} ${re}
    "#);
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"
(* Clean-context assembly: the 16 per-pair ground facts (proven in the function
   body) imply the layer-5 spec (gap 8, step_by 4: pairs (8w+l, 8w+l+4)). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 800 --z3refresh"
let lemma_inv_l5_avx2_assemble
      (orig fin: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        (forall i. (to_i32x8 (Seq.index fin 0).f_value i, to_i32x8 (Seq.index fin 4).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 7)) (to_i32x8 (Seq.index orig 0).f_value i, to_i32x8 (Seq.index orig 4).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 1).f_value i, to_i32x8 (Seq.index fin 5).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 7)) (to_i32x8 (Seq.index orig 1).f_value i, to_i32x8 (Seq.index orig 5).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 2).f_value i, to_i32x8 (Seq.index fin 6).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 7)) (to_i32x8 (Seq.index orig 2).f_value i, to_i32x8 (Seq.index orig 6).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 3).f_value i, to_i32x8 (Seq.index fin 7).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 7)) (to_i32x8 (Seq.index orig 3).f_value i, to_i32x8 (Seq.index orig 7).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 8).f_value i, to_i32x8 (Seq.index fin 12).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 6)) (to_i32x8 (Seq.index orig 8).f_value i, to_i32x8 (Seq.index orig 12).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 9).f_value i, to_i32x8 (Seq.index fin 13).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 6)) (to_i32x8 (Seq.index orig 9).f_value i, to_i32x8 (Seq.index orig 13).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 10).f_value i, to_i32x8 (Seq.index fin 14).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 6)) (to_i32x8 (Seq.index orig 10).f_value i, to_i32x8 (Seq.index orig 14).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 11).f_value i, to_i32x8 (Seq.index fin 15).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 6)) (to_i32x8 (Seq.index orig 11).f_value i, to_i32x8 (Seq.index orig 15).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 16).f_value i, to_i32x8 (Seq.index fin 20).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 5)) (to_i32x8 (Seq.index orig 16).f_value i, to_i32x8 (Seq.index orig 20).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 17).f_value i, to_i32x8 (Seq.index fin 21).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 5)) (to_i32x8 (Seq.index orig 17).f_value i, to_i32x8 (Seq.index orig 21).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 18).f_value i, to_i32x8 (Seq.index fin 22).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 5)) (to_i32x8 (Seq.index orig 18).f_value i, to_i32x8 (Seq.index orig 22).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 19).f_value i, to_i32x8 (Seq.index fin 23).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 5)) (to_i32x8 (Seq.index orig 19).f_value i, to_i32x8 (Seq.index orig 23).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 24).f_value i, to_i32x8 (Seq.index fin 28).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 4)) (to_i32x8 (Seq.index orig 24).f_value i, to_i32x8 (Seq.index orig 28).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 25).f_value i, to_i32x8 (Seq.index fin 29).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 4)) (to_i32x8 (Seq.index orig 25).f_value i, to_i32x8 (Seq.index orig 29).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 26).f_value i, to_i32x8 (Seq.index fin 30).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 4)) (to_i32x8 (Seq.index orig 26).f_value i, to_i32x8 (Seq.index orig 30).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 27).f_value i, to_i32x8 (Seq.index fin 31).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 4)) (to_i32x8 (Seq.index orig 27).f_value i, to_i32x8 (Seq.index orig 31).f_value i)))
      (ensures
        norm [primops; iota; delta_namespace [`%zeta_r; `%Spec.Utils.forall32]]
          (invert_ntt_outer_3_plus_spec 5 orig fin))
  = ()
#pop-options
"#)]
#[hax_lib::fstar::before(r#"
(* Clean-context TRANSPORT for layer 5: from the four raw outer_3_plus call posts
   (call w at offset 8w, step_by 4, with intermediate states s1..s3) derive the 16
   orig->fin per-pair facts.  The four calls are DISJOINT (call w only touches units
   8w..8w+7); so for any unit u=8w+l (l<4) the orig->fin pair fact reduces to call
   w's own post pair (orig[..]==s_w[..] via earlier posts' else-branch frame;
   fin[..]==s_{w+1}[..] via later posts' else-branch frame).  fuel 1 unfolds the
   plain-let outer_3_plus_inv to its `forall j`; ifuel 2 evaluates the (∈) if-ladder
   at parametric u.  Monolithic 800 (mirror the assemble/L6-transport lemmas). *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 800 --z3refresh"
let lemma_inv_l5_transport
      (orig s1 s2 s3 fin: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        outer_3_plus_inv 0  4 (mk_i32 466468)    4  orig s1 /\
        outer_3_plus_inv 8  4 (mk_i32 (-876248)) 12 s1 s2 /\
        outer_3_plus_inv 16 4 (mk_i32 (-777960)) 20 s2 s3 /\
        outer_3_plus_inv 24 4 (mk_i32 237124)    28 s3 fin)
      (ensures
        (forall i. (to_i32x8 (Seq.index fin 0).f_value i, to_i32x8 (Seq.index fin 4).f_value i) ==
           inv_ntt_step (mk_i32 466468) (to_i32x8 (Seq.index orig 0).f_value i, to_i32x8 (Seq.index orig 4).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 1).f_value i, to_i32x8 (Seq.index fin 5).f_value i) ==
           inv_ntt_step (mk_i32 466468) (to_i32x8 (Seq.index orig 1).f_value i, to_i32x8 (Seq.index orig 5).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 2).f_value i, to_i32x8 (Seq.index fin 6).f_value i) ==
           inv_ntt_step (mk_i32 466468) (to_i32x8 (Seq.index orig 2).f_value i, to_i32x8 (Seq.index orig 6).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 3).f_value i, to_i32x8 (Seq.index fin 7).f_value i) ==
           inv_ntt_step (mk_i32 466468) (to_i32x8 (Seq.index orig 3).f_value i, to_i32x8 (Seq.index orig 7).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 8).f_value i, to_i32x8 (Seq.index fin 12).f_value i) ==
           inv_ntt_step (mk_i32 (-876248)) (to_i32x8 (Seq.index orig 8).f_value i, to_i32x8 (Seq.index orig 12).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 9).f_value i, to_i32x8 (Seq.index fin 13).f_value i) ==
           inv_ntt_step (mk_i32 (-876248)) (to_i32x8 (Seq.index orig 9).f_value i, to_i32x8 (Seq.index orig 13).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 10).f_value i, to_i32x8 (Seq.index fin 14).f_value i) ==
           inv_ntt_step (mk_i32 (-876248)) (to_i32x8 (Seq.index orig 10).f_value i, to_i32x8 (Seq.index orig 14).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 11).f_value i, to_i32x8 (Seq.index fin 15).f_value i) ==
           inv_ntt_step (mk_i32 (-876248)) (to_i32x8 (Seq.index orig 11).f_value i, to_i32x8 (Seq.index orig 15).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 16).f_value i, to_i32x8 (Seq.index fin 20).f_value i) ==
           inv_ntt_step (mk_i32 (-777960)) (to_i32x8 (Seq.index orig 16).f_value i, to_i32x8 (Seq.index orig 20).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 17).f_value i, to_i32x8 (Seq.index fin 21).f_value i) ==
           inv_ntt_step (mk_i32 (-777960)) (to_i32x8 (Seq.index orig 17).f_value i, to_i32x8 (Seq.index orig 21).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 18).f_value i, to_i32x8 (Seq.index fin 22).f_value i) ==
           inv_ntt_step (mk_i32 (-777960)) (to_i32x8 (Seq.index orig 18).f_value i, to_i32x8 (Seq.index orig 22).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 19).f_value i, to_i32x8 (Seq.index fin 23).f_value i) ==
           inv_ntt_step (mk_i32 (-777960)) (to_i32x8 (Seq.index orig 19).f_value i, to_i32x8 (Seq.index orig 23).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 24).f_value i, to_i32x8 (Seq.index fin 28).f_value i) ==
           inv_ntt_step (mk_i32 237124) (to_i32x8 (Seq.index orig 24).f_value i, to_i32x8 (Seq.index orig 28).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 25).f_value i, to_i32x8 (Seq.index fin 29).f_value i) ==
           inv_ntt_step (mk_i32 237124) (to_i32x8 (Seq.index orig 25).f_value i, to_i32x8 (Seq.index orig 29).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 26).f_value i, to_i32x8 (Seq.index fin 30).f_value i) ==
           inv_ntt_step (mk_i32 237124) (to_i32x8 (Seq.index orig 26).f_value i, to_i32x8 (Seq.index orig 30).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 27).f_value i, to_i32x8 (Seq.index fin 31).f_value i) ==
           inv_ntt_step (mk_i32 237124) (to_i32x8 (Seq.index orig 27).f_value i, to_i32x8 (Seq.index orig 31).f_value i))) =
  let step0 (l: nat{l < 4})
      : Lemma
        (forall i. (to_i32x8 (Seq.index fin l).f_value i, to_i32x8 (Seq.index fin (l + 4)).f_value i) ==
           inv_ntt_step (mk_i32 466468)
             (to_i32x8 (Seq.index orig l).f_value i, to_i32x8 (Seq.index orig (l + 4)).f_value i)) =
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 0  4 (mk_i32 466468)    4  orig s1 j with l;
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 8  4 (mk_i32 (-876248)) 12 s1 s2 j with l;
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 8  4 (mk_i32 (-876248)) 12 s1 s2 j with (l + 4);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 16 4 (mk_i32 (-777960)) 20 s2 s3 j with l;
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 16 4 (mk_i32 (-777960)) 20 s2 s3 j with (l + 4);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 24 4 (mk_i32 237124)    28 s3 fin j with l;
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 24 4 (mk_i32 237124)    28 s3 fin j with (l + 4)
  in
  let step1 (l: nat{l < 4})
      : Lemma
        (forall i. (to_i32x8 (Seq.index fin (8 + l)).f_value i, to_i32x8 (Seq.index fin (12 + l)).f_value i) ==
           inv_ntt_step (mk_i32 (-876248))
             (to_i32x8 (Seq.index orig (8 + l)).f_value i, to_i32x8 (Seq.index orig (12 + l)).f_value i)) =
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 0  4 (mk_i32 466468)    4  orig s1 j with (8 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 0  4 (mk_i32 466468)    4  orig s1 j with (12 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 8  4 (mk_i32 (-876248)) 12 s1 s2 j with (8 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 16 4 (mk_i32 (-777960)) 20 s2 s3 j with (8 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 16 4 (mk_i32 (-777960)) 20 s2 s3 j with (12 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 24 4 (mk_i32 237124)    28 s3 fin j with (8 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 24 4 (mk_i32 237124)    28 s3 fin j with (12 + l)
  in
  let step2 (l: nat{l < 4})
      : Lemma
        (forall i. (to_i32x8 (Seq.index fin (16 + l)).f_value i, to_i32x8 (Seq.index fin (20 + l)).f_value i) ==
           inv_ntt_step (mk_i32 (-777960))
             (to_i32x8 (Seq.index orig (16 + l)).f_value i, to_i32x8 (Seq.index orig (20 + l)).f_value i)) =
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 0  4 (mk_i32 466468)    4  orig s1 j with (16 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 0  4 (mk_i32 466468)    4  orig s1 j with (20 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 8  4 (mk_i32 (-876248)) 12 s1 s2 j with (16 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 8  4 (mk_i32 (-876248)) 12 s1 s2 j with (20 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 16 4 (mk_i32 (-777960)) 20 s2 s3 j with (16 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 24 4 (mk_i32 237124)    28 s3 fin j with (16 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 24 4 (mk_i32 237124)    28 s3 fin j with (20 + l)
  in
  let step3 (l: nat{l < 4})
      : Lemma
        (forall i. (to_i32x8 (Seq.index fin (24 + l)).f_value i, to_i32x8 (Seq.index fin (28 + l)).f_value i) ==
           inv_ntt_step (mk_i32 237124)
             (to_i32x8 (Seq.index orig (24 + l)).f_value i, to_i32x8 (Seq.index orig (28 + l)).f_value i)) =
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 0  4 (mk_i32 466468)    4  orig s1 j with (24 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 0  4 (mk_i32 466468)    4  orig s1 j with (28 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 8  4 (mk_i32 (-876248)) 12 s1 s2 j with (24 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 8  4 (mk_i32 (-876248)) 12 s1 s2 j with (28 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 16 4 (mk_i32 (-777960)) 20 s2 s3 j with (24 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 16 4 (mk_i32 (-777960)) 20 s2 s3 j with (28 + l);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 24 4 (mk_i32 237124)    28 s3 fin j with (24 + l)
  in
  step0 0; step0 1; step0 2; step0 3;
  step1 0; step1 1; step1 2; step1 3;
  step2 0; step2 1; step2 2; step2 3;
  step3 0; step3 1; step3 2; step3 3
#pop-options
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall32 ]] (invert_ntt_outer_3_plus_spec 5 $re ${re}_future)
"#))]
unsafe fn invert_ntt_at_layer_5(re: &mut AVX2RingElement) {
    const STEP: usize = 32; // 1 << LAYER;
    const STEP_BY: usize = 4; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 466468>(re);
    #[cfg(hax)]
    let s1 = re.clone();
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -876248>(re);
    #[cfg(hax)]
    let s2 = re.clone();
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -777960>(re);
    #[cfg(hax)]
    let s3 = re.clone();
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 237124>(re);

    hax_lib::fstar!(r#"
    assert_norm (zeta_r 7 == 466468);
    assert_norm (zeta_r 6 == (-876248));
    assert_norm (zeta_r 5 == (-777960));
    assert_norm (zeta_r 4 == 237124);
    lemma_inv_l5_transport ${orig_re} ${s1} ${s2} ${s3} ${re};
    lemma_inv_l5_avx2_assemble ${orig_re} ${re}
    "#);
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"
(* Clean-context assembly: the 16 per-pair ground facts (proven in the function
   body) imply the layer-6 spec (gap 16, step_by 8: pairs (16w+l, 16w+l+8)). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 800 --z3refresh"
let lemma_inv_l6_avx2_assemble
      (orig fin: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        (forall i. (to_i32x8 (Seq.index fin 0).f_value i, to_i32x8 (Seq.index fin 8).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 3)) (to_i32x8 (Seq.index orig 0).f_value i, to_i32x8 (Seq.index orig 8).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 1).f_value i, to_i32x8 (Seq.index fin 9).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 3)) (to_i32x8 (Seq.index orig 1).f_value i, to_i32x8 (Seq.index orig 9).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 2).f_value i, to_i32x8 (Seq.index fin 10).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 3)) (to_i32x8 (Seq.index orig 2).f_value i, to_i32x8 (Seq.index orig 10).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 3).f_value i, to_i32x8 (Seq.index fin 11).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 3)) (to_i32x8 (Seq.index orig 3).f_value i, to_i32x8 (Seq.index orig 11).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 4).f_value i, to_i32x8 (Seq.index fin 12).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 3)) (to_i32x8 (Seq.index orig 4).f_value i, to_i32x8 (Seq.index orig 12).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 5).f_value i, to_i32x8 (Seq.index fin 13).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 3)) (to_i32x8 (Seq.index orig 5).f_value i, to_i32x8 (Seq.index orig 13).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 6).f_value i, to_i32x8 (Seq.index fin 14).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 3)) (to_i32x8 (Seq.index orig 6).f_value i, to_i32x8 (Seq.index orig 14).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 7).f_value i, to_i32x8 (Seq.index fin 15).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 3)) (to_i32x8 (Seq.index orig 7).f_value i, to_i32x8 (Seq.index orig 15).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 16).f_value i, to_i32x8 (Seq.index fin 24).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 2)) (to_i32x8 (Seq.index orig 16).f_value i, to_i32x8 (Seq.index orig 24).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 17).f_value i, to_i32x8 (Seq.index fin 25).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 2)) (to_i32x8 (Seq.index orig 17).f_value i, to_i32x8 (Seq.index orig 25).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 18).f_value i, to_i32x8 (Seq.index fin 26).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 2)) (to_i32x8 (Seq.index orig 18).f_value i, to_i32x8 (Seq.index orig 26).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 19).f_value i, to_i32x8 (Seq.index fin 27).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 2)) (to_i32x8 (Seq.index orig 19).f_value i, to_i32x8 (Seq.index orig 27).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 20).f_value i, to_i32x8 (Seq.index fin 28).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 2)) (to_i32x8 (Seq.index orig 20).f_value i, to_i32x8 (Seq.index orig 28).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 21).f_value i, to_i32x8 (Seq.index fin 29).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 2)) (to_i32x8 (Seq.index orig 21).f_value i, to_i32x8 (Seq.index orig 29).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 22).f_value i, to_i32x8 (Seq.index fin 30).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 2)) (to_i32x8 (Seq.index orig 22).f_value i, to_i32x8 (Seq.index orig 30).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 23).f_value i, to_i32x8 (Seq.index fin 31).f_value i) ==
           inv_ntt_step (mk_i32 (zeta_r 2)) (to_i32x8 (Seq.index orig 23).f_value i, to_i32x8 (Seq.index orig 31).f_value i)))
      (ensures
        norm [primops; iota; delta_namespace [`%zeta_r; `%Spec.Utils.forall32]]
          (invert_ntt_outer_3_plus_spec 6 orig fin))
  = ()
#pop-options
"#)]
#[hax_lib::fstar::before(r#"
(* Clean-context TRANSPORT: from the two raw outer_3_plus call posts (with the
   call-1 result named s1) derive the 16 orig->fin per-pair facts.  The width-8
   interval foralls in each call post cannot be ground-extracted inside the
   function's heavy WP (saturates rlimit 400); in this clean 2-hypothesis context
   they discharge.  Monolithic 800 (mirror the assemble lemmas). *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 800 --z3refresh"
let lemma_inv_l6_transport
      (orig s1 fin: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        outer_3_plus_inv 0 8 (mk_i32 (-518909)) 8 orig s1 /\
        outer_3_plus_inv 16 8 (mk_i32 (-2608894)) 24 s1 fin)
      (ensures
        (forall i. (to_i32x8 (Seq.index fin 0).f_value i, to_i32x8 (Seq.index fin 8).f_value i) ==
           inv_ntt_step (mk_i32 (-518909)) (to_i32x8 (Seq.index orig 0).f_value i, to_i32x8 (Seq.index orig 8).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 1).f_value i, to_i32x8 (Seq.index fin 9).f_value i) ==
           inv_ntt_step (mk_i32 (-518909)) (to_i32x8 (Seq.index orig 1).f_value i, to_i32x8 (Seq.index orig 9).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 2).f_value i, to_i32x8 (Seq.index fin 10).f_value i) ==
           inv_ntt_step (mk_i32 (-518909)) (to_i32x8 (Seq.index orig 2).f_value i, to_i32x8 (Seq.index orig 10).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 3).f_value i, to_i32x8 (Seq.index fin 11).f_value i) ==
           inv_ntt_step (mk_i32 (-518909)) (to_i32x8 (Seq.index orig 3).f_value i, to_i32x8 (Seq.index orig 11).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 4).f_value i, to_i32x8 (Seq.index fin 12).f_value i) ==
           inv_ntt_step (mk_i32 (-518909)) (to_i32x8 (Seq.index orig 4).f_value i, to_i32x8 (Seq.index orig 12).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 5).f_value i, to_i32x8 (Seq.index fin 13).f_value i) ==
           inv_ntt_step (mk_i32 (-518909)) (to_i32x8 (Seq.index orig 5).f_value i, to_i32x8 (Seq.index orig 13).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 6).f_value i, to_i32x8 (Seq.index fin 14).f_value i) ==
           inv_ntt_step (mk_i32 (-518909)) (to_i32x8 (Seq.index orig 6).f_value i, to_i32x8 (Seq.index orig 14).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 7).f_value i, to_i32x8 (Seq.index fin 15).f_value i) ==
           inv_ntt_step (mk_i32 (-518909)) (to_i32x8 (Seq.index orig 7).f_value i, to_i32x8 (Seq.index orig 15).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 16).f_value i, to_i32x8 (Seq.index fin 24).f_value i) ==
           inv_ntt_step (mk_i32 (-2608894)) (to_i32x8 (Seq.index orig 16).f_value i, to_i32x8 (Seq.index orig 24).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 17).f_value i, to_i32x8 (Seq.index fin 25).f_value i) ==
           inv_ntt_step (mk_i32 (-2608894)) (to_i32x8 (Seq.index orig 17).f_value i, to_i32x8 (Seq.index orig 25).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 18).f_value i, to_i32x8 (Seq.index fin 26).f_value i) ==
           inv_ntt_step (mk_i32 (-2608894)) (to_i32x8 (Seq.index orig 18).f_value i, to_i32x8 (Seq.index orig 26).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 19).f_value i, to_i32x8 (Seq.index fin 27).f_value i) ==
           inv_ntt_step (mk_i32 (-2608894)) (to_i32x8 (Seq.index orig 19).f_value i, to_i32x8 (Seq.index orig 27).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 20).f_value i, to_i32x8 (Seq.index fin 28).f_value i) ==
           inv_ntt_step (mk_i32 (-2608894)) (to_i32x8 (Seq.index orig 20).f_value i, to_i32x8 (Seq.index orig 28).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 21).f_value i, to_i32x8 (Seq.index fin 29).f_value i) ==
           inv_ntt_step (mk_i32 (-2608894)) (to_i32x8 (Seq.index orig 21).f_value i, to_i32x8 (Seq.index orig 29).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 22).f_value i, to_i32x8 (Seq.index fin 30).f_value i) ==
           inv_ntt_step (mk_i32 (-2608894)) (to_i32x8 (Seq.index orig 22).f_value i, to_i32x8 (Seq.index orig 30).f_value i)) /\
        (forall i. (to_i32x8 (Seq.index fin 23).f_value i, to_i32x8 (Seq.index fin 31).f_value i) ==
           inv_ntt_step (mk_i32 (-2608894)) (to_i32x8 (Seq.index orig 23).f_value i, to_i32x8 (Seq.index orig 31).f_value i))) =
  let step_lo (u: nat{u < 8})
      : Lemma
        (forall i. (to_i32x8 (Seq.index fin u).f_value i, to_i32x8 (Seq.index fin (u + 8)).f_value i) ==
           inv_ntt_step (mk_i32 (-518909))
             (to_i32x8 (Seq.index orig u).f_value i, to_i32x8 (Seq.index orig (u + 8)).f_value i)) =
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 0 8 (mk_i32 (-518909)) 8 orig s1 j with u;
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 16 8 (mk_i32 (-2608894)) 24 s1 fin j with u;
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 16 8 (mk_i32 (-2608894)) 24 s1 fin j with (u + 8)
  in
  let step_hi (u: nat{u < 8})
      : Lemma
        (forall i. (to_i32x8 (Seq.index fin (16 + u)).f_value i, to_i32x8 (Seq.index fin (24 + u)).f_value i) ==
           inv_ntt_step (mk_i32 (-2608894))
             (to_i32x8 (Seq.index orig (16 + u)).f_value i, to_i32x8 (Seq.index orig (24 + u)).f_value i)) =
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 16 8 (mk_i32 (-2608894)) 24 s1 fin j with (16 + u);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 0 8 (mk_i32 (-518909)) 8 orig s1 j with (16 + u);
    eliminate forall (j: nat{j < 32}). outer_3_plus_inv_pointwise 0 8 (mk_i32 (-518909)) 8 orig s1 j with (24 + u)
  in
  step_lo 0; step_lo 1; step_lo 2; step_lo 3; step_lo 4; step_lo 5; step_lo 6; step_lo 7;
  step_hi 0; step_hi 1; step_hi 2; step_hi 3; step_hi 4; step_hi 5; step_hi 6; step_hi 7
#pop-options
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall32 ]] (invert_ntt_outer_3_plus_spec 6 $re ${re}_future)
"#))]
unsafe fn invert_ntt_at_layer_6(re: &mut AVX2RingElement) {
    const STEP: usize = 64; // 1 << LAYER;
    const STEP_BY: usize = 8; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -518909>(re);
    #[cfg(hax)]
    let s1 = re.clone();
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2608894>(re);

    hax_lib::fstar!(r#"
    assert_norm (zeta_r 3 == (-518909));
    assert_norm (zeta_r 2 == (-2608894));
    lemma_inv_l6_transport ${orig_re} ${s1} ${re};
    lemma_inv_l6_avx2_assemble ${orig_re} ${re}
    "#);
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 800 --z3refresh")]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall32 ]] (invert_ntt_outer_3_plus_spec 7 $re ${re}_future)
"#))]
unsafe fn invert_ntt_at_layer_7(re: &mut AVX2RingElement) {
    const STEP: usize = 128; // 1 << LAYER;
    const STEP_BY: usize = 16; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 25847>(re);
}

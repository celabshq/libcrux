use super::{arithmetic, AVX2RingElement};
use crate::simd::{avx2::AVX2SIMDUnit, traits::COEFFICIENTS_IN_SIMD_UNIT};

use libcrux_intrinsics::avx2::*;

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 8380416 $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"
T.is_i32b_poly_avx2 4211177 ${re}_future /\
(let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 $re) in
 let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 ${re}_future) in
 forall (i: nat). i < 256 ==>
   (v (Seq.index out_flat i)) % 8380417 ==
   (v (Seq.index (PI.to_mont (Hacspec_ml_dsa.Ntt.intt in_flat)) i)) % 8380417)
"#))]
pub(crate) fn invert_ntt_montgomery(re: &mut AVX2RingElement) {
    #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
    #[allow(unsafe_code)]
    #[hax_lib::fstar::before(r#"#restart-solver"#)]
    #[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
    #[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 8380416 $re"#))]
    #[hax_lib::ensures(|result| fstar!(r#"
T.is_i32b_poly_avx2 4211177 ${re}_future /\
(let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 $re) in
 let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 ${re}_future) in
 forall (i: nat). i < 256 ==>
   (v (Seq.index out_flat i)) % 8380417 ==
   (v (Seq.index (PI.to_mont (Hacspec_ml_dsa.Ntt.intt in_flat)) i)) % 8380417)
"#))]
    unsafe fn inv_inner(re: &mut AVX2RingElement) {
        #[cfg(hax)]
        let s0 = re.clone();
        inv_run_layers_avx2(re);
        #[cfg(hax)]
        let s8 = re.clone();
        scale_montgomery_avx2(re);
        proof!(
            r#"PI.lemma_invert_top (C.simd_units_to_array (T.chunks_of_re_avx2 s0)) (C.simd_units_to_array (T.chunks_of_re_avx2 s8)) (C.simd_units_to_array (T.chunks_of_re_avx2 re))"#
        );
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
(* The hand-written F* theory for this file lives in the companion module
   Libcrux_ml_dsa.Simd.Avx2.Invntt_theory (git-tracked, NOT
   generated -- see proofs/fstar/extraction/).

   F* module abbreviations are file-scoped and are NOT re-exported by `open`, so
   every alias the theory and the impl fns below use is re-declared here, in the
   original order.  `open Spec.MLDSA.Math` likewise sits exactly where it did --
   after this file's earlier opens, so it keeps its resolution precedence -- and
   the companion is opened LAST so its decls win over the opened Spec modules,
   which is what the in-file definitions did before they were relocated. *)
module T = Avx2NttTheory
module C = Hacspec_ml_dsa.Commute.Chunk
module FN = Libcrux_ml_dsa.Simd.Avx2.Ntt_theory
module PI = Libcrux_ml_dsa.Simd.Portable.Invntt_theory

open Spec.MLDSA.Math

open Libcrux_ml_dsa.Simd.Avx2.Invntt_theory
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
        proof!("assert (outer_3_plus_inv_pointwise (v $OFFSET) (v $STEP_BY) $ZETA (v $OFFSET + v $STEP_BY) ${_re0} ${re} (v j + v $STEP_BY))");
        ()
    }
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall32 ]] (invert_ntt_outer_3_plus_spec 3 $re ${re}_future)
"#))]
unsafe fn invert_ntt_at_layer_3(re: &mut AVX2RingElement) {
    const STEP: usize = 8; // 1 << LAYER;
    const STEP_BY: usize = 1; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 280005>(re);
    #[cfg(hax)]
    let s1 = re.clone();
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 4010497>(re);
    #[cfg(hax)]
    let s2 = re.clone();
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -19422>(re);
    #[cfg(hax)]
    let s3 = re.clone();
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1757237>(re);
    #[cfg(hax)]
    let s4 = re.clone();
    outer_3_plus::<{ (4 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -3277672>(re);
    #[cfg(hax)]
    let s5 = re.clone();
    outer_3_plus::<{ (5 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1399561>(re);
    #[cfg(hax)]
    let s6 = re.clone();
    outer_3_plus::<{ (6 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -3859737>(re);
    #[cfg(hax)]
    let s7 = re.clone();
    outer_3_plus::<{ (7 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2118186>(re);
    #[cfg(hax)]
    let s8 = re.clone();
    outer_3_plus::<{ (8 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2108549>(re);
    #[cfg(hax)]
    let s9 = re.clone();
    outer_3_plus::<{ (9 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2619752>(re);
    #[cfg(hax)]
    let s10 = re.clone();
    outer_3_plus::<{ (10 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1119584>(re);
    #[cfg(hax)]
    let s11 = re.clone();
    outer_3_plus::<{ (11 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -549488>(re);
    #[cfg(hax)]
    let s12 = re.clone();
    outer_3_plus::<{ (12 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3585928>(re);
    #[cfg(hax)]
    let s13 = re.clone();
    outer_3_plus::<{ (13 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1079900>(re);
    #[cfg(hax)]
    let s14 = re.clone();
    outer_3_plus::<{ (14 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1024112>(re);
    #[cfg(hax)]
    let s15 = re.clone();
    outer_3_plus::<{ (15 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2725464>(re);

    proof!(
        r#"
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
    lemma_inv_l3_transport ${orig_re} ${s1} ${s2} ${s3} ${s4} ${s5} ${s6} ${s7} ${s8} ${s9} ${s10} ${s11} ${s12} ${s13} ${s14} ${s15} ${re};
    lemma_inv_l3_avx2_assemble ${orig_re} ${re}
    "#
    );
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
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

    proof!(
        r#"
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
    "#
    );
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
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

    proof!(
        r#"
    assert_norm (zeta_r 7 == 466468);
    assert_norm (zeta_r 6 == (-876248));
    assert_norm (zeta_r 5 == (-777960));
    assert_norm (zeta_r 4 == 237124);
    lemma_inv_l5_transport ${orig_re} ${s1} ${s2} ${s3} ${re};
    lemma_inv_l5_avx2_assemble ${orig_re} ${re}
    "#
    );
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
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

    proof!(
        r#"
    assert_norm (zeta_r 3 == (-518909));
    assert_norm (zeta_r 2 == (-2608894));
    lemma_inv_l6_transport ${orig_re} ${s1} ${re};
    lemma_inv_l6_avx2_assemble ${orig_re} ${re}
    "#
    );
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 800")]
#[hax_lib::ensures(|result| fstar!(r#"
norm [primops; iota; delta_namespace [ `%zeta_r; `%Spec.Utils.forall32 ]] (invert_ntt_outer_3_plus_spec 7 $re ${re}_future)
"#))]
unsafe fn invert_ntt_at_layer_7(re: &mut AVX2RingElement) {
    const STEP: usize = 128; // 1 << LAYER;
    const STEP_BY: usize = 16; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 25847>(re);
}

#[inline(always)]
#[allow(unsafe_code)]
// proof-residence: locked(own-const): scale_montgomery_avx2__v_FACTOR (hax-mangled per-fn const)
#[hax_lib::fstar::before(
    r#"
(* These two lemmas mention `scale_montgomery_avx2__v_FACTOR` -- hax's hoisting of
   the `const FACTOR` declared inside `scale_montgomery_avx2` below -- so they
   cannot travel to the Libcrux_ml_dsa.Simd.Avx2.Invntt_theory companion: that
   module is compiled ahead of this one, which is where the constant is defined.
   The rest of the inverse-NTT theory lives in the companion; these two stay, each
   in the balanced push/pop region it always had.  Both are used only by
   `scale_montgomery_avx2`'s own fstar! blocks, and no other theory decl depends
   on them.  (The Portable mirror has no such constant: its impl passes the
   literal 41_978, so Portable.Invntt_theory states these facts over mk_i32 41978.) *)

#restart-solver
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300"
(* Per-chunk establish: re[i] = mont_mul-by-FACTOR of orig_re[i].  The AVX2
   mont-by-const post gives lane equality to mont_mul; C.lemma_mont_mul_bound_and_mod_q
   turns each lane into the mod_q form chunk_scaled (via lemma_establish_chunk_scaled)
   consumes.  Mirrors the per-iter lemma_establish_chunk_scaled call in Portable
   scale_montgomery, lifted through chunks_of_re_avx2's index reveal.
   NOTE: monolithic (no --split_queries): the requires forall is pruned from
   split sub-queries -> incomplete quantifiers; the lemma is small enough cold. *)
let lemma_establish_chunk_scaled_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:nat{i<32})
    : Lemma
      (requires
        (forall (l:nat). l < 8 ==>
          to_i32x8 (Seq.index re i).f_value (mk_u64 l) ==
          Spec.MLDSA.Math.mont_mul (to_i32x8 (Seq.index orig_re i).f_value (mk_u64 l))
            scale_montgomery_avx2__v_FACTOR))
      (ensures
        PI.chunk_scaled (Seq.index (T.chunks_of_re_avx2 orig_re) i)
                        (Seq.index (T.chunks_of_re_avx2 re) i))
  = let ci = Seq.index (T.chunks_of_re_avx2 orig_re) i in
    let co = Seq.index (T.chunks_of_re_avx2 re) i in
    let aux (l:nat{l<8}) : Lemma
        (Spec.MLDSA.Math.mod_q (v (Seq.index co l)) ==
         Spec.MLDSA.Math.mod_q (v (Seq.index ci l) * 41978 * 8265825)) =
      reveal_opaque (`%Spec.MLDSA.Math.mod_q) Spec.MLDSA.Math.mod_q;
      let x = to_i32x8 (Seq.index orig_re i).f_value (mk_u64 l) in
      assert (to_i32x8 (Seq.index re i).f_value (mk_u64 l) ==
              Spec.MLDSA.Math.mont_mul x scale_montgomery_avx2__v_FACTOR);
      T.lemma_chunks_of_re_avx2_index orig_re i l;
      T.lemma_chunks_of_re_avx2_index re i l;
      C.lemma_mont_mul_bound_and_mod_q x scale_montgomery_avx2__v_FACTOR
    in Classical.forall_intro aux;
    PI.lemma_establish_chunk_scaled ci co
#pop-options

#restart-solver
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
(* Per-iteration scaling step: re is the post-update array (re[i] = mont_mul-by-FACTOR
   of orig_unit); establishes the new chunk_scaled atom + per-lane FM bound for index i.
   Mirrors the lemma_establish_chunk_scaled call inside Portable scale_montgomery's body. *)
let lemma_inv_scale_step
      (s8 re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:usize{v i < 32})
      (orig_unit: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
    : Lemma
      (requires
        Seq.index re (v i) ==
          ({ orig_unit with
             Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value =
               Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply_by_constant
                 orig_unit.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
                 scale_montgomery_avx2__v_FACTOR }
           <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) /\
        orig_unit == Seq.index s8 (v i) /\
        T.is_i32b_poly_avx2 (256 * 8380416) s8)
      (ensures
        (forall (l:nat). l < 8 ==>
           Spec.Utils.is_i32b 4211177 (to_i32x8 (Seq.index re (v i)).f_value (mk_u64 l))) /\
        PI.chunk_scaled (Seq.index (T.chunks_of_re_avx2 s8) (v i))
                        (Seq.index (T.chunks_of_re_avx2 re) (v i)))
  = (* Trigger the mont-by-const post (opaque_to_smt fn): its post fires on the
       application term, giving the per-lane mont_mul equality. *)
    assert_norm (v scale_montgomery_avx2__v_FACTOR == 41978);
    let mont_res = Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply_by_constant
                     orig_unit.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
                     scale_montgomery_avx2__v_FACTOR in
    assert (forall l. to_i32x8 mont_res l ==
              Spec.MLDSA.Math.mont_mul (to_i32x8 orig_unit.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value l)
                scale_montgomery_avx2__v_FACTOR);
    assert ((Seq.index re (v i)).f_value == mont_res);
    (* per-lane: tight 4211177 bound (from the 256*FIELD_MAX input) + mont_mul -> mod_q form *)
    let bridge (l:nat{l<8}) : Lemma
        (Spec.Utils.is_i32b 4211177 (to_i32x8 (Seq.index re (v i)).f_value (mk_u64 l)) /\
         to_i32x8 (Seq.index re (v i)).f_value (mk_u64 l) ==
         Spec.MLDSA.Math.mont_mul (to_i32x8 (Seq.index s8 (v i)).f_value (mk_u64 l))
           scale_montgomery_avx2__v_FACTOR) =
      C.lemma_mont_mul_bound_and_mod_q (to_i32x8 (Seq.index s8 (v i)).f_value (mk_u64 l))
        scale_montgomery_avx2__v_FACTOR;
      T.lemma_is_i32b_poly_avx2_elim (256 * 8380416) s8 (v i) l;
      lemma_mont_mul_tight_bound_256 (to_i32x8 (Seq.index s8 (v i)).f_value (mk_u64 l))
        scale_montgomery_avx2__v_FACTOR
    in Classical.forall_intro bridge;
    lemma_establish_chunk_scaled_avx2 s8 re (v i)
#pop-options
"#
)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
// Input bound 256*FIELD_MAX (the inverse-NTT layers' accumulated bound); the
// final ·41978 Montgomery multiply reduces it to the tight centered 4211177.
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (256 * 8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"
T.is_i32b_poly_avx2 4211177 ${re}_future /\
(let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 $re) in
 let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 ${re}_future) in
 forall (j: nat). j < 256 ==>
   (v (Seq.index out_flat j)) % 8380417 == (16382 * v (Seq.index in_flat j)) % 8380417)
"#))]
unsafe fn scale_montgomery_avx2(re: &mut AVX2RingElement) {
    const FACTOR: i32 = 41_978;
    #[cfg(hax)]
    let s8 = re.clone();
    for i in 0..re.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"
T.is_i32b_poly_avx2 (256 * 8380416) s8 /\
(forall (k:nat). k < v $i ==>
   (forall (l:nat). l < 8 ==> Spec.Utils.is_i32b 4211177 (to_i32x8 (Seq.index ${re} k).f_value (mk_u64 l))) /\
   PI.chunk_scaled (Seq.index (T.chunks_of_re_avx2 s8) k) (Seq.index (T.chunks_of_re_avx2 ${re}) k)) /\
(forall (k:nat). (k >= v $i /\ k < 32) ==> (Seq.index ${re} k) == (Seq.index s8 k))
"#
        ));
        #[cfg(hax)]
        let re_old = re.clone();
        #[cfg(hax)]
        let orig_unit = re[i];
        re[i].value = arithmetic::montgomery_multiply_by_constant(re[i].value, FACTOR);
        proof!(
            r#"lemma_inv_scale_step s8 re i orig_unit; lemma_inv_scale_carryover s8 re_old re i"#
        );
    }
    proof!(r#"lemma_inv_scale_finalize s8 re"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 8380416 $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (2*8380416) ${re}_future /\ inv_layer_done 0 $re ${re}_future"#))]
unsafe fn run_inv_layer_0(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_0(re);
    proof!(r#"lemma_inv_l0_post_to_sym orig re; lemma_inv_l0_sealed orig re 8380416"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (2*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (4*8380416) ${re}_future /\ inv_layer_done 1 $re ${re}_future"#))]
unsafe fn run_inv_layer_1(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_1(re);
    proof!(r#"lemma_inv_l1_post_to_sym orig re; lemma_inv_l1_sealed orig re (2*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (4*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (8*8380416) ${re}_future /\ inv_layer_done 2 $re ${re}_future"#))]
unsafe fn run_inv_layer_2(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_2(re);
    proof!(r#"lemma_inv_l2_post_to_sym orig re; lemma_inv_l2_sealed orig re (4*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (8*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (16*8380416) ${re}_future /\ inv_layer_done 3 $re ${re}_future"#))]
unsafe fn run_inv_layer_3(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_3(re);
    proof!(r#"lemma_inv_l3_sealed orig re (8*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (16*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (32*8380416) ${re}_future /\ inv_layer_done 4 $re ${re}_future"#))]
unsafe fn run_inv_layer_4(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_4(re);
    proof!(r#"lemma_inv_l4_sealed orig re (16*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (32*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (64*8380416) ${re}_future /\ inv_layer_done 5 $re ${re}_future"#))]
unsafe fn run_inv_layer_5(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_5(re);
    proof!(r#"lemma_inv_l5_sealed orig re (32*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (64*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (128*8380416) ${re}_future /\ inv_layer_done 6 $re ${re}_future"#))]
unsafe fn run_inv_layer_6(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_6(re);
    proof!(r#"lemma_inv_l6_sealed orig re (64*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (128*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (256*8380416) ${re}_future /\ inv_layer_done 7 $re ${re}_future"#))]
unsafe fn run_inv_layer_7(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_7(re);
    proof!(r#"lemma_inv_l7_sealed orig re (128*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 100")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 8380416 $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"
T.is_i32b_poly_avx2 (256*8380416) ${re}_future /\
(let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 $re) in
 let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 ${re}_future) in
 forall (i: nat). i < 256 ==>
   (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index (PI.intt_unscaled in_flat) i)) % 8380417)
"#))]
unsafe fn inv_run_layers_avx2(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let s0 = re.clone();
    run_inv_layer_0(re);
    #[cfg(hax)]
    let s1 = re.clone();
    run_inv_layer_1(re);
    #[cfg(hax)]
    let s2 = re.clone();
    run_inv_layer_2(re);
    #[cfg(hax)]
    let s3 = re.clone();
    run_inv_layer_3(re);
    #[cfg(hax)]
    let s4 = re.clone();
    run_inv_layer_4(re);
    #[cfg(hax)]
    let s5 = re.clone();
    run_inv_layer_5(re);
    #[cfg(hax)]
    let s6 = re.clone();
    run_inv_layer_6(re);
    #[cfg(hax)]
    let s7 = re.clone();
    run_inv_layer_7(re);
    #[cfg(hax)]
    let s8 = re.clone();
    proof!(r#"lemma_inv_compose_8_sealed s0 s1 s2 s3 s4 s5 s6 s7 s8"#);
}

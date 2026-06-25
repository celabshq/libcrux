use super::{arithmetic, AVX2RingElement};
use crate::simd::{avx2::AVX2SIMDUnit, traits::COEFFICIENTS_IN_SIMD_UNIT};

use libcrux_intrinsics::avx2::*;

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --z3refresh")]
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
    #[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --z3refresh")]
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
        hax_lib::fstar!(r#"PI.lemma_invert_top (C.simd_units_to_array (T.chunks_of_re_avx2 s0)) (C.simd_units_to_array (T.chunks_of_re_avx2 s8)) (C.simd_units_to_array (T.chunks_of_re_avx2 re))"#);
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
(* ============================================================================
   PHASE B (within-chunk inverse layers 0/1/2): flat `intt_layer` congruence
   bridge.  For each within-chunk inverse layer N we add
   `lemma_inv_lN_full_avx2`: from the input bound + the layer fn's per-pair
   `inv_ntt_step` post, derive the output bound (2*bnd) + the flat-poly
   congruence (out_flat == intt_layer in_flat N  mod q).

   Mirror of the FORWARD AVX2 machinery in Libcrux_ml_dsa.Simd.Avx2.Ntt
   (lemma_l{0,1,2}_full_avx2), with:
     - ntt_step  -> inv_ntt_step  (GS butterfly: co[even]=a+b, co[odd]=mont_mul(b-a,zeta))
     - zeta idx  -> L0: 255-(4b+p);  L1: 127-(2b+h);  L2: 63-b
     - bridge    -> Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_{0,1,2}_step_to_hacspec_poly
     - atom      -> Portable's unit_fe_post_inv_l{0,1,2} shape
   The chunk view + bound predicate + generic elim lemmas are REUSED from
   Avx2NttTheory / Libcrux_ml_dsa.Simd.Avx2.Ntt (not redefined). ===== *)
module T = Avx2NttTheory
module C = Hacspec_ml_dsa.Commute.Chunk
module FN = Libcrux_ml_dsa.Simd.Avx2.Ntt

open Spec.MLDSA.Math

(* ===== INVERSE LAYER 0 ====================================================
   Layer-0 fn post (symbolic-zeta form): for chunk b=2i (nre0)/b=2i+1 (nre1),
   pair (2p,2p+1) with zeta = zeta_r (255 - (4b+p)). ===== *)
unfold let inv_l0_post_sym (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  norm [
      primops; iota;
      delta_namespace [`%Spec.Utils.forall4; `%Spec.Utils.forall16]
    ]
    (Spec.Utils.forall16 (fun i ->
          let nre = re_fut in
          let re0 = Seq.index re (i * 2) in
          let re1 = Seq.index re (i * 2 + 1) in
          let nre0 = Seq.index nre (i * 2) in
          let nre1 = Seq.index nre (i * 2 + 1) in
          Spec.Utils.forall4 (fun j ->
                let zeta0 = zeta_r (255 - (i * 8 + j)) in
                let zeta1 = zeta_r (255 - (i * 8 + j + 4)) in
                let j0 = j * 2 in
                let j1 = j0 + 1 in
                (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
                inv_ntt_step (mk_int zeta0)
                  (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
                (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
                inv_ntt_step (mk_int zeta1)
                  (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))))

unfold let inv_l0_body (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                   (i:nat{i<16}) : Type0 =
  let re0 = Seq.index re (i*2) in
  let re1 = Seq.index re (i*2+1) in
  let nre0 = Seq.index re_fut (i*2) in
  let nre1 = Seq.index re_fut (i*2+1) in
  Spec.Utils.forall4 (fun j ->
        let zeta0 = zeta_r (255 - (i*8 + j)) in
        let zeta1 = zeta_r (255 - (i*8 + j + 4)) in
        let j0 = j*2 in let j1 = j0+1 in
        (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
          inv_ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
        (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
          inv_ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))

unfold let inv_body2 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                 (i:nat{i<16}) (j:nat{j<4}) : Type0 =
  let re0 = Seq.index re (i*2) in
  let re1 = Seq.index re (i*2+1) in
  let nre0 = Seq.index re_fut (i*2) in
  let nre1 = Seq.index re_fut (i*2+1) in
  let zeta0 = zeta_r (255 - (i*8 + j)) in
  let zeta1 = zeta_r (255 - (i*8 + j + 4)) in
  let j0 = j*2 in let j1 = j0+1 in
  (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
    inv_ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
  (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
    inv_ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1))

(* chunkfact: per-chunk b, pair p, the inv_ntt_step relation with chunk-flat
   zeta zeta_r (255 - (4b+p)). *)
unfold let inv_chunkfact (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                     (b:nat{b<32}) (p:nat{p<4}) : Type0 =
  let ci = T.chunks_of_re_avx2 re in
  let co = T.chunks_of_re_avx2 re_fut in
  (Seq.index (Seq.index co b) (2*p), Seq.index (Seq.index co b) (2*p+1)) ==
    inv_ntt_step (mk_int (zeta_r (255 - (4*b + p))))
      (Seq.index (Seq.index ci b) (2*p), Seq.index (Seq.index ci b) (2*p+1))

#push-options "--fuel 0 --ifuel 1 --z3rlimit 80"
let inv_lemma_lift2 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires inv_l0_post_sym re re_fut)
            (ensures forall (i:nat{i<16}) (j:nat{j<4}). inv_body2 re re_fut i j)
  = FN.forall16_elim_1d (inv_l0_body re re_fut);
    let aux (i:nat{i<16}) : Lemma (forall (j:nat{j<4}). inv_body2 re re_fut i j) =
      FN.forall4_elim_1d (fun (j:nat{j<4}) -> inv_body2 re re_fut i j)
    in Classical.forall_intro aux
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let inv_lemma_chunkfact_even
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:nat{i<16}) (p:nat{p<4})
    : Lemma (requires inv_body2 re re_fut i p) (ensures inv_chunkfact re re_fut (2*i) p)
  = T.lemma_chunks_of_re_avx2_index re (2*i) (2*p);
    T.lemma_chunks_of_re_avx2_index re (2*i) (2*p+1);
    T.lemma_chunks_of_re_avx2_index re_fut (2*i) (2*p);
    T.lemma_chunks_of_re_avx2_index re_fut (2*i) (2*p+1)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let inv_lemma_chunkfact_odd
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:nat{i<16}) (p:nat{p<4})
    : Lemma (requires inv_body2 re re_fut i p) (ensures inv_chunkfact re re_fut (2*i+1) p)
  = T.lemma_chunks_of_re_avx2_index re (2*i+1) (2*p);
    T.lemma_chunks_of_re_avx2_index re (2*i+1) (2*p+1);
    T.lemma_chunks_of_re_avx2_index re_fut (2*i+1) (2*p);
    T.lemma_chunks_of_re_avx2_index re_fut (2*i+1) (2*p+1)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let inv_lemma_chunkfacts_from_lift
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires inv_l0_post_sym re re_fut)
            (ensures forall (b:nat{b<32}) (p:nat{p<4}). inv_chunkfact re re_fut b p)
  = inv_lemma_lift2 re re_fut;
    let auxe (i:nat{i<16}) (p:nat{p<4}) : Lemma (inv_chunkfact re re_fut (2*i) p) =
      inv_lemma_chunkfact_even re re_fut i p
    in Classical.forall_intro_2 auxe;
    let auxo (i:nat{i<16}) (p:nat{p<4}) : Lemma (inv_chunkfact re re_fut (2*i+1) p) =
      inv_lemma_chunkfact_odd re re_fut i p
    in Classical.forall_intro_2 auxo;
    FN.reindex_32_from_16 (inv_chunkfact re re_fut)
#pop-options

(* CRUX: from the per-(b,p) inv_ntt_step post + input bound, derive the GS
   butterfly relations + output bound.  even lane: co=a+b (plain add); odd lane:
   co = mont_mul (b-a) zeta. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let inv_lemma_l0_pair_relations
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (b: nat{b < 32}) (p: nat{p < 4})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ inv_chunkfact re re_fut b p)
        (ensures
          (let ci = T.chunks_of_re_avx2 re in
           let co = T.chunks_of_re_avx2 re_fut in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (255 - (4*b + p)) ] in
           let zm : i32 = mk_int (zeta_r (255 - (4*b + p))) in
           v (Seq.index (Seq.index co b) (2*p)) ==
             v (Seq.index (Seq.index ci b) (2*p)) + v (Seq.index (Seq.index ci b) (2*p+1)) /\
           (v (Seq.index (Seq.index co b) (2*p+1))) % 8380417 ==
             ((v (Seq.index (Seq.index ci b) (2*p+1)) - v (Seq.index (Seq.index ci b) (2*p)))
                * v zm * 8265825) % 8380417 /\
           (v zm) % 8380417 == (v z * pow2 32) % 8380417 /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index (Seq.index co b) (2*p)) /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index (Seq.index co b) (2*p+1))))
  = let ci = T.chunks_of_re_avx2 re in
    let co = T.chunks_of_re_avx2 re_fut in
    let ci_e = Seq.index (Seq.index ci b) (2*p) in
    let ci_o = Seq.index (Seq.index ci b) (2*p+1) in
    let co_e = Seq.index (Seq.index co b) (2*p) in
    let co_o = Seq.index (Seq.index co b) (2*p+1) in
    let zm : i32 = mk_int (zeta_r (255 - (4*b + p))) in
    // inv_ntt_step unfolds: co_e = add_mod_opaque ci_e ci_o; co_o = mont_mul (sub_mod_opaque ci_o ci_e) zm
    assert (co_e == add_mod_opaque ci_e ci_o);
    assert (co_o == mont_mul (sub_mod_opaque ci_o ci_e) zm);
    // input bounds on the pair (via opaque-predicate elim)
    T.lemma_chunks_of_re_avx2_index re b (2*p);
    T.lemma_chunks_of_re_avx2_index re b (2*p+1);
    T.lemma_is_i32b_poly_avx2_elim bnd re b (2*p);
    T.lemma_is_i32b_poly_avx2_elim bnd re b (2*p+1);
    assert (Spec.Utils.is_i32b bnd ci_e);
    assert (Spec.Utils.is_i32b bnd ci_o);
    // add/sub exactness (no overflow): even lane add fits 2*bnd; diff fits 2*bnd<pow2 31
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    assert (v co_e == v ci_e + v ci_o);
    let d : i32 = sub_mod_opaque ci_o ci_e in
    assert (v d == v ci_o - v ci_e);
    // mont bound + mod-q (zeta_r bounded by 4190208 < FIELD_MAX)
    assert (Spec.Utils.is_i32b 8380416 zm);
    C.lemma_mont_mul_bound_and_mod_q d zm;
    assert (Spec.Utils.is_i32b 8380416 co_o);
    // (v co_o) %q == (v d * v zm * 8265825) %q == ((v ci_o - v ci_e) * v zm * 8265825) %q
    // zeta canonicalization
    let idx : nat = 255 - (4*b + p) in
    C.lemma_v_zetas_eq_zeta idx
#pop-options

(* ===== Opaque per-chunk inverse FE atom (mirror Portable unit_fe_post_inv_l0). ===== *)
[@@ "opaque_to_smt"]
let unit_post_inv_l0_avx2 (ci co: t_Array i32 (mk_usize 8))
      (zeta0 zeta1 zeta2 zeta3: i32{Spec.Utils.is_i32b 4190208 zeta0 /\ Spec.Utils.is_i32b 4190208 zeta1 /\ Spec.Utils.is_i32b 4190208 zeta2 /\ Spec.Utils.is_i32b 4190208 zeta3}) : Type0 =
  (v (Seq.index co 0) == v (Seq.index ci 0) + v (Seq.index ci 1) /\
   (v (Seq.index co 1)) % 8380417 == ((v (Seq.index ci 1) - v (Seq.index ci 0)) * v zeta0 * 8265825) % 8380417 /\
   v (Seq.index co 2) == v (Seq.index ci 2) + v (Seq.index ci 3) /\
   (v (Seq.index co 3)) % 8380417 == ((v (Seq.index ci 3) - v (Seq.index ci 2)) * v zeta1 * 8265825) % 8380417 /\
   v (Seq.index co 4) == v (Seq.index ci 4) + v (Seq.index ci 5) /\
   (v (Seq.index co 5)) % 8380417 == ((v (Seq.index ci 5) - v (Seq.index ci 4)) * v zeta2 * 8265825) % 8380417 /\
   v (Seq.index co 6) == v (Seq.index ci 6) + v (Seq.index ci 7) /\
   (v (Seq.index co 7)) % 8380417 == ((v (Seq.index ci 7) - v (Seq.index ci 6)) * v zeta3 * 8265825) % 8380417)

(* Per-chunk establishment: input bound + the 4 inv_chunkfacts -> the opaque
   atom + per-lane output bound (2*bnd). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l0_chunk_avx2
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (b: nat{b < 32})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ (forall (p:nat{p<4}). inv_chunkfact re re_fut b p))
        (ensures
          unit_post_inv_l0_avx2 (Seq.index (T.chunks_of_re_avx2 re) b) (Seq.index (T.chunks_of_re_avx2 re_fut) b)
            (mk_i32 (zeta_r (255 - (4*b + 0)))) (mk_i32 (zeta_r (255 - (4*b + 1))))
            (mk_i32 (zeta_r (255 - (4*b + 2)))) (mk_i32 (zeta_r (255 - (4*b + 3)))) /\
          (forall (l:nat). l < 8 ==>
            Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut b).f_value (mk_u64 l))))
  = eliminate forall (p:nat{p<4}). inv_chunkfact re re_fut b p with 0;
    eliminate forall (p:nat{p<4}). inv_chunkfact re re_fut b p with 1;
    eliminate forall (p:nat{p<4}). inv_chunkfact re re_fut b p with 2;
    eliminate forall (p:nat{p<4}). inv_chunkfact re re_fut b p with 3;
    inv_lemma_l0_pair_relations re re_fut bnd b 0;
    inv_lemma_l0_pair_relations re re_fut bnd b 1;
    inv_lemma_l0_pair_relations re re_fut bnd b 2;
    inv_lemma_l0_pair_relations re re_fut bnd b 3;
    reveal_opaque (`%unit_post_inv_l0_avx2) unit_post_inv_l0_avx2;
    introduce forall (l:nat{l<8}).
        Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut b).f_value (mk_u64 l))
    with (T.lemma_chunks_of_re_avx2_index re_fut b l)
#pop-options

(* Unfold one L0 opaque atom to the bridge's per-pair forall. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always --z3refresh"
let inv_lemma_atom_to_bf_l0_avx2 (ci co: t_Array i32 (mk_usize 8))
      (zf: (p: nat{p < 4}) -> (z: i32{Spec.Utils.is_i32b 4190208 z}))
    : Lemma (requires unit_post_inv_l0_avx2 ci co (zf 0) (zf 1) (zf 2) (zf 3))
            (ensures
              (forall (p: nat{p < 4}).
                 v (Seq.index co (2*p))   == v (Seq.index ci (2*p)) + v (Seq.index ci (2*p+1)) /\
                 (v (Seq.index co (2*p+1))) % 8380417 ==
                   ((v (Seq.index ci (2*p+1)) - v (Seq.index ci (2*p))) * v (zf p) * 8265825) % 8380417))
  = reveal_opaque (`%unit_post_inv_l0_avx2) unit_post_inv_l0_avx2;
    introduce forall (p: nat{p < 4}).
        (v (Seq.index co (2*p))   == v (Seq.index ci (2*p)) + v (Seq.index ci (2*p+1)) /\
         (v (Seq.index co (2*p+1))) % 8380417 ==
           ((v (Seq.index ci (2*p+1)) - v (Seq.index ci (2*p))) * v (zf p) * 8265825) % 8380417)
    with (match p with | 0 -> () | 1 -> () | 2 -> () | _ -> ())
#pop-options

(* Clean-context driver composition: forall32 opaque atoms -> intt_layer 0 congruence. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l0_driver_compose_avx2
      (orig fut: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : Lemma
        (requires
          forall32 (fun b ->
            unit_post_inv_l0_avx2 (Seq.index orig b) (Seq.index fut b)
              (mk_i32 (zeta_r (255 - (4*b + 0)))) (mk_i32 (zeta_r (255 - (4*b + 1))))
              (mk_i32 (zeta_r (255 - (4*b + 2)))) (mk_i32 (zeta_r (255 - (4*b + 3))))))
        (ensures
          (let in_flat = C.simd_units_to_array orig in
           let out_flat = C.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 0) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (b: nat{b < 32}) (p: nat{p < 4}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (zeta_r (255 - (4*b + p))) in
    FN.forall32_elim_1d (fun b -> unit_post_inv_l0_avx2 (Seq.index orig b) (Seq.index fut b)
                                 (mk_i32 (zeta_r (255 - (4*b + 0)))) (mk_i32 (zeta_r (255 - (4*b + 1))))
                                 (mk_i32 (zeta_r (255 - (4*b + 2)))) (mk_i32 (zeta_r (255 - (4*b + 3)))));
    (let aux (b: nat{b < 32}) (p: nat{p < 4}) : Lemma
       (let ci = Seq.index orig b in
        let co = Seq.index fut b in
        v (Seq.index co (2*p)) == v (Seq.index ci (2*p)) + v (Seq.index ci (2*p+1)) /\
        (v (Seq.index co (2*p+1))) % 8380417 ==
          ((v (Seq.index ci (2*p+1)) - v (Seq.index ci (2*p))) * v (zm b p) * 8265825) % 8380417 /\
        (v (zm b p)) % 8380417 ==
          (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (255 - (4*b + p)) ] <: i32) * pow2 32) % 8380417)
      = inv_lemma_atom_to_bf_l0_avx2 (Seq.index orig b) (Seq.index fut b) (fun p -> zm b p);
        reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
        let _ = zeta_r (255 - (4*b + p)) in
        C.lemma_v_zetas_eq_zeta (255 - (4*b + p))
     in Classical.forall_intro_2 aux);
    C.lemma_intt_layer_0_step_to_hacspec_poly orig fut zm
#pop-options

(* FULL L0 glue: input bound + symbolic L0 post -> output bound (2*bnd) + intt_layer 0 congruence. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_inv_l0_full_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd orig_re /\ inv_l0_post_sym orig_re re)
        (ensures
          T.is_i32b_poly_avx2 (2*bnd) re /\
          (let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 0) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = inv_lemma_chunkfacts_from_lift orig_re re;
    let aux (b:nat{b<32}) : Lemma
        (unit_post_inv_l0_avx2 (Seq.index (T.chunks_of_re_avx2 orig_re) b) (Seq.index (T.chunks_of_re_avx2 re) b)
           (mk_i32 (zeta_r (255 - (4*b + 0)))) (mk_i32 (zeta_r (255 - (4*b + 1))))
           (mk_i32 (zeta_r (255 - (4*b + 2)))) (mk_i32 (zeta_r (255 - (4*b + 3))))
         /\ (forall (l:nat). l<8 ==>
              Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re b).f_value (mk_u64 l))))
      = inv_lemma_l0_chunk_avx2 orig_re re bnd b
    in Classical.forall_intro aux;
    T.lemma_is_i32b_poly_avx2_intro (2*bnd) re;
    inv_lemma_l0_driver_compose_avx2 (T.chunks_of_re_avx2 orig_re) (T.chunks_of_re_avx2 re)
#pop-options
"#
)]
#[hax_lib::fstar::before(
    r#"
(* ===== INVERSE LAYER 1 ====================================================
   Layer-1 fn post (symbolic): chunk b=2i (nre0)/b=2i+1 (nre1), pairs
   (4h+j, 4h+j+2) with zeta = zeta_r (127 - (2b+h)).  AVX2 ensures uses
   j in 0..3 with j0 in {0,1,4,5}, j1=j0+2, h=j/2, j'=j%2. ===== *)
unfold let inv_l1_post_sym (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  norm [
      primops; iota;
      delta_namespace [`%Spec.Utils.forall4; `%Spec.Utils.forall16]
    ]
    (Spec.Utils.forall16 (fun i ->
          let nre = re_fut in
          let re0 = Seq.index re (i * 2) in
          let re1 = Seq.index re (i * 2 + 1) in
          let nre0 = Seq.index nre (i * 2) in
          let nre1 = Seq.index nre (i * 2 + 1) in
          Spec.Utils.forall4 (fun j ->
                let zeta0 = zeta_r (127 - (i * 4 + j / 2)) in
                let zeta1 = zeta_r (127 - (i * 4 + j / 2 + 2)) in
                let j0 = (match j with | 0 -> 0 | 1 -> 1 | 2 -> 4 | 3 -> 5) in
                let j1 = j0 + 2 in
                (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
                inv_ntt_step (mk_int zeta0)
                  (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
                (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
                inv_ntt_step (mk_int zeta1)
                  (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))))

unfold let inv_l1_body (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                   (i:nat{i<16}) : Type0 =
  let re0 = Seq.index re (i*2) in
  let re1 = Seq.index re (i*2+1) in
  let nre0 = Seq.index re_fut (i*2) in
  let nre1 = Seq.index re_fut (i*2+1) in
  Spec.Utils.forall4 (fun j ->
        let zeta0 = zeta_r (127 - (i*4 + j/2)) in
        let zeta1 = zeta_r (127 - (i*4 + j/2 + 2)) in
        let j0 = (match j with | 0 -> 0 | 1 -> 1 | 2 -> 4 | 3 -> 5) in
        let j1 = j0+2 in
        (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
          inv_ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
        (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
          inv_ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))

(* per (i,j): the same body conjunct (used by the (h,j) reindex). *)
unfold let inv_l1_body2 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                 (i:nat{i<16}) (j:nat{j<4}) : Type0 =
  let re0 = Seq.index re (i*2) in
  let re1 = Seq.index re (i*2+1) in
  let nre0 = Seq.index re_fut (i*2) in
  let nre1 = Seq.index re_fut (i*2+1) in
  let zeta0 = zeta_r (127 - (i*4 + j/2)) in
  let zeta1 = zeta_r (127 - (i*4 + j/2 + 2)) in
  let j0 = (match j with | 0 -> 0 | 1 -> 1 | 2 -> 4 | 3 -> 5) in
  let j1 = j0+2 in
  (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
    inv_ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
  (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
    inv_ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1))

(* L1 chunkfact: chunk b, sub-index (h,j) h<2 j<2, lanes (4h+j, 4h+j+2),
   zeta zeta_r (127-(2b+h)). *)
unfold let inv_chunkfact_l1 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                     (b:nat{b<32}) (h:nat{h<2}) (j:nat{j<2}) : Type0 =
  let ci = T.chunks_of_re_avx2 re in
  let co = T.chunks_of_re_avx2 re_fut in
  (Seq.index (Seq.index co b) (4*h+j), Seq.index (Seq.index co b) (4*h+j+2)) ==
    inv_ntt_step (mk_int (zeta_r (127 - (2*b + h))))
      (Seq.index (Seq.index ci b) (4*h+j), Seq.index (Seq.index ci b) (4*h+j+2))

#push-options "--fuel 0 --ifuel 1 --z3rlimit 80"
let inv_l1_lemma_lift2 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires inv_l1_post_sym re re_fut)
            (ensures forall (i:nat{i<16}) (j:nat{j<4}). inv_l1_body2 re re_fut i j)
  = FN.forall16_elim_1d (inv_l1_body re re_fut);
    let aux (i:nat{i<16}) : Lemma (forall (j:nat{j<4}). inv_l1_body2 re re_fut i j) =
      FN.forall4_elim_1d (fun (j:nat{j<4}) -> inv_l1_body2 re re_fut i j)
    in Classical.forall_intro aux
#pop-options

(* even chunk b=2i: (h,j) maps from j-index (j_idx = 2*h+j) of nre0. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let inv_l1_chunkfact_even
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:nat{i<16}) (h:nat{h<2}) (j:nat{j<2})
    : Lemma (requires inv_l1_body2 re re_fut i (2*h+j)) (ensures inv_chunkfact_l1 re re_fut (2*i) h j)
  = T.lemma_chunks_of_re_avx2_index re (2*i) (4*h+j);
    T.lemma_chunks_of_re_avx2_index re (2*i) (4*h+j+2);
    T.lemma_chunks_of_re_avx2_index re_fut (2*i) (4*h+j);
    T.lemma_chunks_of_re_avx2_index re_fut (2*i) (4*h+j+2)
#pop-options

(* odd chunk b=2i+1: from nre1 part. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let inv_l1_chunkfact_odd
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:nat{i<16}) (h:nat{h<2}) (j:nat{j<2})
    : Lemma (requires inv_l1_body2 re re_fut i (2*h+j)) (ensures inv_chunkfact_l1 re re_fut (2*i+1) h j)
  = T.lemma_chunks_of_re_avx2_index re (2*i+1) (4*h+j);
    T.lemma_chunks_of_re_avx2_index re (2*i+1) (4*h+j+2);
    T.lemma_chunks_of_re_avx2_index re_fut (2*i+1) (4*h+j);
    T.lemma_chunks_of_re_avx2_index re_fut (2*i+1) (4*h+j+2)
#pop-options

(* generic reindex (b<32,h<2,j<2): even/odd 16-foralls -> 32-forall. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let inv_l1_reindex_32 (q: (b:nat{b<32}) -> (h:nat{h<2}) -> (j:nat{j<2}) -> Type0)
    : Lemma (requires (forall (i:nat{i<16}) (h:nat{h<2}) (j:nat{j<2}). q (2*i) h j) /\
                      (forall (i:nat{i<16}) (h:nat{h<2}) (j:nat{j<2}). q (2*i+1) h j))
            (ensures forall (b:nat{b<32}) (h:nat{h<2}) (j:nat{j<2}). q b h j)
  = let aux (b:nat{b<32}) (h:nat{h<2}) (j:nat{j<2}) : Lemma (q b h j) =
      FStar.Math.Lemmas.euclidean_division_definition b 2;
      (if b % 2 = 0 then assert (q (2*(b/2)) h j) else assert (q (2*(b/2)+1) h j))
    in Classical.forall_intro_3 aux
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let inv_l1_chunkfacts_from_lift
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires inv_l1_post_sym re re_fut)
            (ensures forall (b:nat{b<32}) (h:nat{h<2}) (j:nat{j<2}). inv_chunkfact_l1 re re_fut b h j)
  = inv_l1_lemma_lift2 re re_fut;
    let auxe (i:nat{i<16}) (h:nat{h<2}) (j:nat{j<2}) : Lemma (inv_chunkfact_l1 re re_fut (2*i) h j) =
      inv_l1_chunkfact_even re re_fut i h j
    in Classical.forall_intro_3 auxe;
    let auxo (i:nat{i<16}) (h:nat{h<2}) (j:nat{j<2}) : Lemma (inv_chunkfact_l1 re re_fut (2*i+1) h j) =
      inv_l1_chunkfact_odd re re_fut i h j
    in Classical.forall_intro_3 auxo;
    inv_l1_reindex_32 (inv_chunkfact_l1 re re_fut)
#pop-options

(* CRUX L1: per (b,h,j) inv_ntt_step post + input bound -> GS facts + output bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let inv_l1_pair_relations
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (b: nat{b < 32}) (h:nat{h<2}) (j:nat{j<2})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ inv_chunkfact_l1 re re_fut b h j)
        (ensures
          (let ci = T.chunks_of_re_avx2 re in
           let co = T.chunks_of_re_avx2 re_fut in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (127 - (2*b + h)) ] in
           let zm : i32 = mk_int (zeta_r (127 - (2*b + h))) in
           v (Seq.index (Seq.index co b) (4*h+j)) ==
             v (Seq.index (Seq.index ci b) (4*h+j)) + v (Seq.index (Seq.index ci b) (4*h+j+2)) /\
           (v (Seq.index (Seq.index co b) (4*h+j+2))) % 8380417 ==
             ((v (Seq.index (Seq.index ci b) (4*h+j+2)) - v (Seq.index (Seq.index ci b) (4*h+j)))
                * v zm * 8265825) % 8380417 /\
           (v zm) % 8380417 == (v z * pow2 32) % 8380417 /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index (Seq.index co b) (4*h+j)) /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index (Seq.index co b) (4*h+j+2))))
  = let ci = T.chunks_of_re_avx2 re in
    let co = T.chunks_of_re_avx2 re_fut in
    let ci_e = Seq.index (Seq.index ci b) (4*h+j) in
    let ci_o = Seq.index (Seq.index ci b) (4*h+j+2) in
    let co_e = Seq.index (Seq.index co b) (4*h+j) in
    let co_o = Seq.index (Seq.index co b) (4*h+j+2) in
    let zm : i32 = mk_int (zeta_r (127 - (2*b + h))) in
    assert (co_e == add_mod_opaque ci_e ci_o);
    assert (co_o == mont_mul (sub_mod_opaque ci_o ci_e) zm);
    T.lemma_chunks_of_re_avx2_index re b (4*h+j);
    T.lemma_chunks_of_re_avx2_index re b (4*h+j+2);
    T.lemma_is_i32b_poly_avx2_elim bnd re b (4*h+j);
    T.lemma_is_i32b_poly_avx2_elim bnd re b (4*h+j+2);
    assert (Spec.Utils.is_i32b bnd ci_e);
    assert (Spec.Utils.is_i32b bnd ci_o);
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    assert (v co_e == v ci_e + v ci_o);
    let d : i32 = sub_mod_opaque ci_o ci_e in
    assert (v d == v ci_o - v ci_e);
    assert (Spec.Utils.is_i32b 8380416 zm);
    C.lemma_mont_mul_bound_and_mod_q d zm;
    assert (Spec.Utils.is_i32b 8380416 co_o);
    let idx : nat = 127 - (2*b + h) in
    C.lemma_v_zetas_eq_zeta idx
#pop-options

(* L1 opaque per-chunk inverse FE atom (mirror Portable unit_fe_post_inv_l1). *)
[@@ "opaque_to_smt"]
let unit_post_inv_l1_avx2 (ci co: t_Array i32 (mk_usize 8))
      (zeta0 zeta1: i32{Spec.Utils.is_i32b 4190208 zeta0 /\ Spec.Utils.is_i32b 4190208 zeta1}) : Type0 =
  (v (Seq.index co 0) == v (Seq.index ci 0) + v (Seq.index ci 2) /\
   (v (Seq.index co 2)) % 8380417 == ((v (Seq.index ci 2) - v (Seq.index ci 0)) * v zeta0 * 8265825) % 8380417 /\
   v (Seq.index co 1) == v (Seq.index ci 1) + v (Seq.index ci 3) /\
   (v (Seq.index co 3)) % 8380417 == ((v (Seq.index ci 3) - v (Seq.index ci 1)) * v zeta0 * 8265825) % 8380417 /\
   v (Seq.index co 4) == v (Seq.index ci 4) + v (Seq.index ci 6) /\
   (v (Seq.index co 6)) % 8380417 == ((v (Seq.index ci 6) - v (Seq.index ci 4)) * v zeta1 * 8265825) % 8380417 /\
   v (Seq.index co 5) == v (Seq.index ci 5) + v (Seq.index ci 7) /\
   (v (Seq.index co 7)) % 8380417 == ((v (Seq.index ci 7) - v (Seq.index ci 5)) * v zeta1 * 8265825) % 8380417)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l1_chunk_avx2
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (b: nat{b < 32})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ (forall (h:nat{h<2}) (j:nat{j<2}). inv_chunkfact_l1 re re_fut b h j))
        (ensures
          unit_post_inv_l1_avx2 (Seq.index (T.chunks_of_re_avx2 re) b) (Seq.index (T.chunks_of_re_avx2 re_fut) b)
            (mk_i32 (zeta_r (127 - (2*b + 0)))) (mk_i32 (zeta_r (127 - (2*b + 1)))) /\
          (forall (l:nat). l < 8 ==>
            Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut b).f_value (mk_u64 l))))
  = eliminate forall (h:nat{h<2}) (j:nat{j<2}). inv_chunkfact_l1 re re_fut b h j with 0 0;
    eliminate forall (h:nat{h<2}) (j:nat{j<2}). inv_chunkfact_l1 re re_fut b h j with 0 1;
    eliminate forall (h:nat{h<2}) (j:nat{j<2}). inv_chunkfact_l1 re re_fut b h j with 1 0;
    eliminate forall (h:nat{h<2}) (j:nat{j<2}). inv_chunkfact_l1 re re_fut b h j with 1 1;
    inv_l1_pair_relations re re_fut bnd b 0 0;
    inv_l1_pair_relations re re_fut bnd b 0 1;
    inv_l1_pair_relations re re_fut bnd b 1 0;
    inv_l1_pair_relations re re_fut bnd b 1 1;
    reveal_opaque (`%unit_post_inv_l1_avx2) unit_post_inv_l1_avx2;
    introduce forall (l:nat{l<8}).
        Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut b).f_value (mk_u64 l))
    with (T.lemma_chunks_of_re_avx2_index re_fut b l)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always --z3refresh"
let inv_lemma_atom_to_bf_l1_avx2 (ci co: t_Array i32 (mk_usize 8))
      (zf: (h: nat{h < 2}) -> (z: i32{Spec.Utils.is_i32b 4190208 z}))
    : Lemma (requires unit_post_inv_l1_avx2 ci co (zf 0) (zf 1))
            (ensures
              (forall (h: nat{h < 2}) (j: nat{j < 2}).
                 v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v (Seq.index ci (4*h+j+2)) /\
                 (v (Seq.index co (4*h+j+2))) % 8380417 ==
                   ((v (Seq.index ci (4*h+j+2)) - v (Seq.index ci (4*h+j))) * v (zf h) * 8265825) % 8380417))
  = reveal_opaque (`%unit_post_inv_l1_avx2) unit_post_inv_l1_avx2;
    introduce forall (h: nat{h < 2}) (j: nat{j < 2}).
        (v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v (Seq.index ci (4*h+j+2)) /\
         (v (Seq.index co (4*h+j+2))) % 8380417 ==
           ((v (Seq.index ci (4*h+j+2)) - v (Seq.index ci (4*h+j))) * v (zf h) * 8265825) % 8380417)
    with (match h with | 0 -> (match j with | 0 -> () | _ -> ()) | _ -> (match j with | 0 -> () | _ -> ()))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l1_driver_compose_avx2
      (orig fut: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : Lemma
        (requires
          forall32 (fun b ->
            unit_post_inv_l1_avx2 (Seq.index orig b) (Seq.index fut b)
              (mk_i32 (zeta_r (127 - (2*b + 0)))) (mk_i32 (zeta_r (127 - (2*b + 1))))))
        (ensures
          (let in_flat = C.simd_units_to_array orig in
           let out_flat = C.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 1) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (b: nat{b < 32}) (h: nat{h < 2}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (zeta_r (127 - (2*b + h))) in
    FN.forall32_elim_1d (fun b -> unit_post_inv_l1_avx2 (Seq.index orig b) (Seq.index fut b)
                                 (mk_i32 (zeta_r (127 - (2*b + 0)))) (mk_i32 (zeta_r (127 - (2*b + 1)))));
    (let aux_bf (b: nat{b < 32}) : Lemma
       (forall (h: nat{h < 2}) (j: nat{j < 2}).
         (let ci = Seq.index orig b in
          let co = Seq.index fut b in
          v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v (Seq.index ci (4*h+j+2)) /\
          (v (Seq.index co (4*h+j+2))) % 8380417 ==
            ((v (Seq.index ci (4*h+j+2)) - v (Seq.index ci (4*h+j))) * v (zm b h) * 8265825) % 8380417))
      = inv_lemma_atom_to_bf_l1_avx2 (Seq.index orig b) (Seq.index fut b) (fun h -> zm b h)
     in Classical.forall_intro aux_bf);
    (let aux_z (b: nat{b < 32}) (h: nat{h < 2}) : Lemma
       ((v (zm b h)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (127 - (2*b + h)) ] <: i32) * pow2 32) % 8380417)
      = reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
        let _ = zeta_r (127 - (2*b + h)) in
        C.lemma_v_zetas_eq_zeta (127 - (2*b + h))
     in Classical.forall_intro_2 aux_z);
    C.lemma_intt_layer_1_step_to_hacspec_poly orig fut zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_inv_l1_full_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd orig_re /\ inv_l1_post_sym orig_re re)
        (ensures
          T.is_i32b_poly_avx2 (2*bnd) re /\
          (let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 1) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = inv_l1_chunkfacts_from_lift orig_re re;
    let aux (b:nat{b<32}) : Lemma
        (unit_post_inv_l1_avx2 (Seq.index (T.chunks_of_re_avx2 orig_re) b) (Seq.index (T.chunks_of_re_avx2 re) b)
           (mk_i32 (zeta_r (127 - (2*b + 0)))) (mk_i32 (zeta_r (127 - (2*b + 1))))
         /\ (forall (l:nat). l<8 ==>
              Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re b).f_value (mk_u64 l))))
      = inv_lemma_l1_chunk_avx2 orig_re re bnd b
    in Classical.forall_intro aux;
    T.lemma_is_i32b_poly_avx2_intro (2*bnd) re;
    inv_lemma_l1_driver_compose_avx2 (T.chunks_of_re_avx2 orig_re) (T.chunks_of_re_avx2 re)
#pop-options

(* ===== INVERSE LAYER 2 ====================================================
   Layer-2 fn post (symbolic): chunk b=2i (nre0)/b=2i+1 (nre1), pairs (p,p+4)
   p<4, with single chunk zeta = zeta_r (63 - b).  AVX2 ensures: nre0 zeta0 =
   zeta_r (63 - (i*2)), nre1 zeta1 = zeta_r (63 - (i*2+1)). ===== *)
unfold let inv_l2_post_sym (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  norm [
      primops; iota;
      delta_namespace [`%Spec.Utils.forall4; `%Spec.Utils.forall16]
    ]
    (Spec.Utils.forall16 (fun i ->
          let nre = re_fut in
          let re0 = Seq.index re (i * 2) in
          let re1 = Seq.index re (i * 2 + 1) in
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
                  (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))))

unfold let inv_l2_body (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                   (i:nat{i<16}) : Type0 =
  let re0 = Seq.index re (i*2) in
  let re1 = Seq.index re (i*2+1) in
  let nre0 = Seq.index re_fut (i*2) in
  let nre1 = Seq.index re_fut (i*2+1) in
  Spec.Utils.forall4 (fun j ->
        let zeta0 = zeta_r (63 - (i*2)) in
        let zeta1 = zeta_r (63 - (i*2+1)) in
        let j0 = j in let j1 = j0+4 in
        (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
          inv_ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
        (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
          inv_ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))

unfold let inv_l2_body2 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                 (i:nat{i<16}) (j:nat{j<4}) : Type0 =
  let re0 = Seq.index re (i*2) in
  let re1 = Seq.index re (i*2+1) in
  let nre0 = Seq.index re_fut (i*2) in
  let nre1 = Seq.index re_fut (i*2+1) in
  let zeta0 = zeta_r (63 - (i*2)) in
  let zeta1 = zeta_r (63 - (i*2+1)) in
  let j0 = j in let j1 = j0+4 in
  (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
    inv_ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
  (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
    inv_ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1))

(* L2 chunkfact: chunk b, pair p<4, lanes (p,p+4), single zeta zeta_r (63-b). *)
unfold let inv_chunkfact_l2 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                     (b:nat{b<32}) (p:nat{p<4}) : Type0 =
  let ci = T.chunks_of_re_avx2 re in
  let co = T.chunks_of_re_avx2 re_fut in
  (Seq.index (Seq.index co b) p, Seq.index (Seq.index co b) (p+4)) ==
    inv_ntt_step (mk_int (zeta_r (63 - b)))
      (Seq.index (Seq.index ci b) p, Seq.index (Seq.index ci b) (p+4))

#push-options "--fuel 0 --ifuel 1 --z3rlimit 80"
let inv_l2_lemma_lift2 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires inv_l2_post_sym re re_fut)
            (ensures forall (i:nat{i<16}) (j:nat{j<4}). inv_l2_body2 re re_fut i j)
  = FN.forall16_elim_1d (inv_l2_body re re_fut);
    let aux (i:nat{i<16}) : Lemma (forall (j:nat{j<4}). inv_l2_body2 re re_fut i j) =
      FN.forall4_elim_1d (fun (j:nat{j<4}) -> inv_l2_body2 re re_fut i j)
    in Classical.forall_intro aux
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let inv_l2_chunkfact_even
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:nat{i<16}) (p:nat{p<4})
    : Lemma (requires inv_l2_body2 re re_fut i p) (ensures inv_chunkfact_l2 re re_fut (2*i) p)
  = T.lemma_chunks_of_re_avx2_index re (2*i) p;
    T.lemma_chunks_of_re_avx2_index re (2*i) (p+4);
    T.lemma_chunks_of_re_avx2_index re_fut (2*i) p;
    T.lemma_chunks_of_re_avx2_index re_fut (2*i) (p+4)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let inv_l2_chunkfact_odd
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:nat{i<16}) (p:nat{p<4})
    : Lemma (requires inv_l2_body2 re re_fut i p) (ensures inv_chunkfact_l2 re re_fut (2*i+1) p)
  = T.lemma_chunks_of_re_avx2_index re (2*i+1) p;
    T.lemma_chunks_of_re_avx2_index re (2*i+1) (p+4);
    T.lemma_chunks_of_re_avx2_index re_fut (2*i+1) p;
    T.lemma_chunks_of_re_avx2_index re_fut (2*i+1) (p+4)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let inv_l2_chunkfacts_from_lift
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires inv_l2_post_sym re re_fut)
            (ensures forall (b:nat{b<32}) (p:nat{p<4}). inv_chunkfact_l2 re re_fut b p)
  = inv_l2_lemma_lift2 re re_fut;
    let auxe (i:nat{i<16}) (p:nat{p<4}) : Lemma (inv_chunkfact_l2 re re_fut (2*i) p) =
      inv_l2_chunkfact_even re re_fut i p
    in Classical.forall_intro_2 auxe;
    let auxo (i:nat{i<16}) (p:nat{p<4}) : Lemma (inv_chunkfact_l2 re re_fut (2*i+1) p) =
      inv_l2_chunkfact_odd re re_fut i p
    in Classical.forall_intro_2 auxo;
    FN.reindex_32_from_16 (inv_chunkfact_l2 re re_fut)
#pop-options

(* CRUX L2: per (b,p) inv_ntt_step post + input bound -> GS facts + output bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let inv_l2_pair_relations
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (b: nat{b < 32}) (p: nat{p < 4})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ inv_chunkfact_l2 re re_fut b p)
        (ensures
          (let ci = T.chunks_of_re_avx2 re in
           let co = T.chunks_of_re_avx2 re_fut in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (63 - b) ] in
           let zm : i32 = mk_int (zeta_r (63 - b)) in
           v (Seq.index (Seq.index co b) p) ==
             v (Seq.index (Seq.index ci b) p) + v (Seq.index (Seq.index ci b) (p+4)) /\
           (v (Seq.index (Seq.index co b) (p+4))) % 8380417 ==
             ((v (Seq.index (Seq.index ci b) (p+4)) - v (Seq.index (Seq.index ci b) p))
                * v zm * 8265825) % 8380417 /\
           (v zm) % 8380417 == (v z * pow2 32) % 8380417 /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index (Seq.index co b) p) /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index (Seq.index co b) (p+4))))
  = let ci = T.chunks_of_re_avx2 re in
    let co = T.chunks_of_re_avx2 re_fut in
    let ci_e = Seq.index (Seq.index ci b) p in
    let ci_o = Seq.index (Seq.index ci b) (p+4) in
    let co_e = Seq.index (Seq.index co b) p in
    let co_o = Seq.index (Seq.index co b) (p+4) in
    let zm : i32 = mk_int (zeta_r (63 - b)) in
    assert (co_e == add_mod_opaque ci_e ci_o);
    assert (co_o == mont_mul (sub_mod_opaque ci_o ci_e) zm);
    T.lemma_chunks_of_re_avx2_index re b p;
    T.lemma_chunks_of_re_avx2_index re b (p+4);
    T.lemma_is_i32b_poly_avx2_elim bnd re b p;
    T.lemma_is_i32b_poly_avx2_elim bnd re b (p+4);
    assert (Spec.Utils.is_i32b bnd ci_e);
    assert (Spec.Utils.is_i32b bnd ci_o);
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    assert (v co_e == v ci_e + v ci_o);
    let d : i32 = sub_mod_opaque ci_o ci_e in
    assert (v d == v ci_o - v ci_e);
    assert (Spec.Utils.is_i32b 8380416 zm);
    C.lemma_mont_mul_bound_and_mod_q d zm;
    assert (Spec.Utils.is_i32b 8380416 co_o);
    let idx : nat = 63 - b in
    C.lemma_v_zetas_eq_zeta idx
#pop-options

(* L2 opaque per-chunk inverse FE atom (mirror Portable unit_fe_post_inv_l2). *)
[@@ "opaque_to_smt"]
let unit_post_inv_l2_avx2 (ci co: t_Array i32 (mk_usize 8))
      (zeta: i32{Spec.Utils.is_i32b 4190208 zeta}) : Type0 =
  (v (Seq.index co 0) == v (Seq.index ci 0) + v (Seq.index ci 4) /\
   (v (Seq.index co 4)) % 8380417 == ((v (Seq.index ci 4) - v (Seq.index ci 0)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co 1) == v (Seq.index ci 1) + v (Seq.index ci 5) /\
   (v (Seq.index co 5)) % 8380417 == ((v (Seq.index ci 5) - v (Seq.index ci 1)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co 2) == v (Seq.index ci 2) + v (Seq.index ci 6) /\
   (v (Seq.index co 6)) % 8380417 == ((v (Seq.index ci 6) - v (Seq.index ci 2)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co 3) == v (Seq.index ci 3) + v (Seq.index ci 7) /\
   (v (Seq.index co 7)) % 8380417 == ((v (Seq.index ci 7) - v (Seq.index ci 3)) * v zeta * 8265825) % 8380417)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l2_chunk_avx2
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (b: nat{b < 32})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ (forall (p:nat{p<4}). inv_chunkfact_l2 re re_fut b p))
        (ensures
          unit_post_inv_l2_avx2 (Seq.index (T.chunks_of_re_avx2 re) b) (Seq.index (T.chunks_of_re_avx2 re_fut) b)
            (mk_i32 (zeta_r (63 - b))) /\
          (forall (l:nat). l < 8 ==>
            Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut b).f_value (mk_u64 l))))
  = eliminate forall (p:nat{p<4}). inv_chunkfact_l2 re re_fut b p with 0;
    eliminate forall (p:nat{p<4}). inv_chunkfact_l2 re re_fut b p with 1;
    eliminate forall (p:nat{p<4}). inv_chunkfact_l2 re re_fut b p with 2;
    eliminate forall (p:nat{p<4}). inv_chunkfact_l2 re re_fut b p with 3;
    inv_l2_pair_relations re re_fut bnd b 0;
    inv_l2_pair_relations re re_fut bnd b 1;
    inv_l2_pair_relations re re_fut bnd b 2;
    inv_l2_pair_relations re re_fut bnd b 3;
    reveal_opaque (`%unit_post_inv_l2_avx2) unit_post_inv_l2_avx2;
    introduce forall (l:nat{l<8}).
        Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut b).f_value (mk_u64 l))
    with (T.lemma_chunks_of_re_avx2_index re_fut b l)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always --z3refresh"
let inv_lemma_atom_to_bf_l2_avx2 (ci co: t_Array i32 (mk_usize 8))
      (zeta: i32{Spec.Utils.is_i32b 4190208 zeta})
    : Lemma (requires unit_post_inv_l2_avx2 ci co zeta)
            (ensures
              (forall (p: nat{p < 4}).
                 v (Seq.index co p)     == v (Seq.index ci p) + v (Seq.index ci (p+4)) /\
                 (v (Seq.index co (p+4))) % 8380417 ==
                   ((v (Seq.index ci (p+4)) - v (Seq.index ci p)) * v zeta * 8265825) % 8380417))
  = reveal_opaque (`%unit_post_inv_l2_avx2) unit_post_inv_l2_avx2;
    introduce forall (p: nat{p < 4}).
        (v (Seq.index co p)     == v (Seq.index ci p) + v (Seq.index ci (p+4)) /\
         (v (Seq.index co (p+4))) % 8380417 ==
           ((v (Seq.index ci (p+4)) - v (Seq.index ci p)) * v zeta * 8265825) % 8380417)
    with (match p with | 0 -> () | 1 -> () | 2 -> () | _ -> ())
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l2_driver_compose_avx2
      (orig fut: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : Lemma
        (requires
          forall32 (fun b ->
            unit_post_inv_l2_avx2 (Seq.index orig b) (Seq.index fut b)
              (mk_i32 (zeta_r (63 - b)))))
        (ensures
          (let in_flat = C.simd_units_to_array orig in
           let out_flat = C.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 2) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (b: nat{b < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (zeta_r (63 - b)) in
    FN.forall32_elim_1d (fun b -> unit_post_inv_l2_avx2 (Seq.index orig b) (Seq.index fut b)
                                 (mk_i32 (zeta_r (63 - b))));
    (let aux_bf (b: nat{b < 32}) : Lemma
       (forall (p: nat{p < 4}).
         (let ci = Seq.index orig b in
          let co = Seq.index fut b in
          v (Seq.index co p)     == v (Seq.index ci p) + v (Seq.index ci (p+4)) /\
          (v (Seq.index co (p+4))) % 8380417 ==
            ((v (Seq.index ci (p+4)) - v (Seq.index ci p)) * v (zm b) * 8265825) % 8380417))
      = inv_lemma_atom_to_bf_l2_avx2 (Seq.index orig b) (Seq.index fut b) (zm b)
     in Classical.forall_intro aux_bf);
    (let aux_z (b: nat{b < 32}) : Lemma
       ((v (zm b)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (63 - b) ] <: i32) * pow2 32) % 8380417)
      = reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
        let _ = zeta_r (63 - b) in
        C.lemma_v_zetas_eq_zeta (63 - b)
     in Classical.forall_intro aux_z);
    C.lemma_intt_layer_2_step_to_hacspec_poly orig fut zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_inv_l2_full_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd orig_re /\ inv_l2_post_sym orig_re re)
        (ensures
          T.is_i32b_poly_avx2 (2*bnd) re /\
          (let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 2) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = inv_l2_chunkfacts_from_lift orig_re re;
    let aux (b:nat{b<32}) : Lemma
        (unit_post_inv_l2_avx2 (Seq.index (T.chunks_of_re_avx2 orig_re) b) (Seq.index (T.chunks_of_re_avx2 re) b)
           (mk_i32 (zeta_r (63 - b)))
         /\ (forall (l:nat). l<8 ==>
              Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re b).f_value (mk_u64 l))))
      = inv_lemma_l2_chunk_avx2 orig_re re bnd b
    in Classical.forall_intro aux;
    T.lemma_is_i32b_poly_avx2_intro (2*bnd) re;
    inv_lemma_l2_driver_compose_avx2 (T.chunks_of_re_avx2 orig_re) (T.chunks_of_re_avx2 re)
#pop-options
"#
)]
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


#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"
(* =========================================================================
   PHASE B CROSS LAYERS (N=3..7).  Mirror of Portable invntt.rs cross drivers
   (lemma_inv_lN_cross_driver_compose) plus the chunks_of_re_avx2 projection
   from the within-chunk AVX2 lemmas.  Geometry: GS butterfly on PAIRS OF CHUNKS
   (lo-unit u, hi-unit u+step_by).  step_by = pow2(N-3): L3=1,L4=2,L5=4,L6=8,L7=16.
   gap = 2*step_by.  Zeta walked DOWN: zeta_r(zeta_rank - u/gap) keyed on lo-unit.
   ========================================================================= *)

(* Shared opaque cross GS-FE atom (mirror Portable unit_fe_post_inv_cross). *)
[@@ "opaque_to_smt"]
let unit_post_inv_cross_avx2 (ci_lo ci_hi co_lo co_hi : t_Array i32 (mk_usize 8))
                             (zeta: i32{Spec.Utils.is_i32b 4190208 zeta}) : Type0 =
  (v (Seq.index co_lo 0) == v (Seq.index ci_lo 0) + v (Seq.index ci_hi 0) /\
   (v (Seq.index co_hi 0)) % 8380417 == ((v (Seq.index ci_hi 0) - v (Seq.index ci_lo 0)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 1) == v (Seq.index ci_lo 1) + v (Seq.index ci_hi 1) /\
   (v (Seq.index co_hi 1)) % 8380417 == ((v (Seq.index ci_hi 1) - v (Seq.index ci_lo 1)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 2) == v (Seq.index ci_lo 2) + v (Seq.index ci_hi 2) /\
   (v (Seq.index co_hi 2)) % 8380417 == ((v (Seq.index ci_hi 2) - v (Seq.index ci_lo 2)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 3) == v (Seq.index ci_lo 3) + v (Seq.index ci_hi 3) /\
   (v (Seq.index co_hi 3)) % 8380417 == ((v (Seq.index ci_hi 3) - v (Seq.index ci_lo 3)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 4) == v (Seq.index ci_lo 4) + v (Seq.index ci_hi 4) /\
   (v (Seq.index co_hi 4)) % 8380417 == ((v (Seq.index ci_hi 4) - v (Seq.index ci_lo 4)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 5) == v (Seq.index ci_lo 5) + v (Seq.index ci_hi 5) /\
   (v (Seq.index co_hi 5)) % 8380417 == ((v (Seq.index ci_hi 5) - v (Seq.index ci_lo 5)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 6) == v (Seq.index ci_lo 6) + v (Seq.index ci_hi 6) /\
   (v (Seq.index co_hi 6)) % 8380417 == ((v (Seq.index ci_hi 6) - v (Seq.index ci_lo 6)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 7) == v (Seq.index ci_lo 7) + v (Seq.index ci_hi 7) /\
   (v (Seq.index co_hi 7)) % 8380417 == ((v (Seq.index ci_hi 7) - v (Seq.index ci_lo 7)) * v zeta * 8265825) % 8380417)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always --z3refresh"
let inv_lemma_atom_to_bf_inv_cross_avx2 (ci_lo ci_hi co_lo co_hi : t_Array i32 (mk_usize 8))
                                        (zeta: i32{Spec.Utils.is_i32b 4190208 zeta})
    : Lemma (requires unit_post_inv_cross_avx2 ci_lo ci_hi co_lo co_hi zeta)
            (ensures
              (forall (l: nat{l < 8}).
                 v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
                 (v (Seq.index co_hi l)) % 8380417 ==
                   ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v zeta * 8265825) % 8380417))
  = reveal_opaque (`%unit_post_inv_cross_avx2) unit_post_inv_cross_avx2;
    introduce forall (l: nat{l < 8}).
        (v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
         (v (Seq.index co_hi l)) % 8380417 ==
           ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v zeta * 8265825) % 8380417)
    with (match l with | 0 -> () | 1 -> () | 2 -> () | 3 -> () | 4 -> () | 5 -> () | 6 -> () | _ -> ())
#pop-options

(* ===== INVERSE LAYER 3 (cross, step_by=1, gap=2, zeta_rank=31) ============= *)

(* Per (u,l) symbolic chunkfact extracted from invert_ntt_outer_3_plus_spec 3.
   u is the lo-unit (u%2==0); the post relates units u, u+1, lane l. *)
unfold let inv_l3_cross_chunkfact
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    (u:nat{u<32 /\ u%2==0}) (l:nat{l<8}) : Type0 =
  let re_lo = Seq.index re u in let re_hi = Seq.index re (u+1) in
  let nre_lo = Seq.index re_fut u in let nre_hi = Seq.index re_fut (u+1) in
  let zeta = zeta_r (31 - u/2) in
  (to_i32x8 nre_lo.f_value (mk_u64 l), to_i32x8 nre_hi.f_value (mk_u64 l)) ==
    inv_ntt_step (mk_int zeta) (to_i32x8 re_lo.f_value (mk_u64 l), to_i32x8 re_hi.f_value (mk_u64 l))

(* The layer-3 post body at index j (step_by=1: w=j, ll=0, uu=2*j). *)
unfold let inv_l3_post_body
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    (j:nat{j<32}) : Type0 =
  j < 16 ==>
    (let w = j / 1 in let ll = j % 1 in
     let zeta = mk_i32 (zeta_r (31 - w)) in
     let uu = w * 2 + ll in
     let  re_j = (Seq.index  re uu).f_value in
     let nre_j = (Seq.index re_fut uu).f_value in
     let  re_j'= (Seq.index  re (uu + 1)).f_value in
     let nre_j'= (Seq.index re_fut (uu + 1)).f_value in
     forall i. (to_i32x8 nre_j i, to_i32x8 nre_j' i) ==
                inv_ntt_step zeta (to_i32x8 re_j i, to_i32x8 re_j' i))

(* Lift the layer post (forall32 j<16) into per-(u,l) chunkfacts.  For each
   even u, instantiate the post at j=u/2 (w=j, ll=0, uu=u). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_l3_cross_chunkfacts_from_post
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
        (requires invert_ntt_outer_3_plus_spec 3 re re_fut)
        (ensures forall (u:nat{u<32 /\ u%2==0}) (l:nat{l<8}). inv_l3_cross_chunkfact re re_fut u l)
  = assert_norm (invert_ntt_outer_3_plus_spec 3 re re_fut ==
       Spec.Utils.forall32 (inv_l3_post_body re re_fut));
    FN.forall32_elim_1d (inv_l3_post_body re re_fut);
    let aux (u:nat{u<32 /\ u%2==0}) (l:nat{l<8}) : Lemma (inv_l3_cross_chunkfact re re_fut u l) =
      let j : nat = u / 2 in
      FStar.Math.Lemmas.lemma_div_mod u 2;
      assert (j < 16 /\ j / 1 == j /\ j % 1 == 0 /\ j * 2 + 0 == u);
      assert (inv_l3_post_body re re_fut j);
      assert (v (mk_u64 l) == l)
    in Classical.forall_intro_2 aux
#pop-options

(* Per (u,l): chunkfact + input bound -> GS facts + zeta fact + output bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let inv_l3_cross_pair_relations
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (u:nat{u<32 /\ u%2==0}) (l:nat{l<8})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ inv_l3_cross_chunkfact re re_fut u l)
        (ensures
          (let ci_lo = T.chunks_of_re_avx2 re in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (31 - u/2) ] in
           let zm : i32 = mk_int (zeta_r (31 - u/2)) in
           let cilo = Seq.index (T.chunks_of_re_avx2 re) u in
           let cihi = Seq.index (T.chunks_of_re_avx2 re) (u+1) in
           let colo = Seq.index (T.chunks_of_re_avx2 re_fut) u in
           let cohi = Seq.index (T.chunks_of_re_avx2 re_fut) (u+1) in
           v (Seq.index colo l) == v (Seq.index cilo l) + v (Seq.index cihi l) /\
           (v (Seq.index cohi l)) % 8380417 ==
             ((v (Seq.index cihi l) - v (Seq.index cilo l)) * v zm * 8265825) % 8380417 /\
           (v zm) % 8380417 == (v z * pow2 32) % 8380417 /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index colo l) /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index cohi l)))
  = let cilo = Seq.index (T.chunks_of_re_avx2 re) u in
    let cihi = Seq.index (T.chunks_of_re_avx2 re) (u+1) in
    let colo = Seq.index (T.chunks_of_re_avx2 re_fut) u in
    let cohi = Seq.index (T.chunks_of_re_avx2 re_fut) (u+1) in
    let ci_e = Seq.index cilo l in
    let ci_o = Seq.index cihi l in
    let co_e = Seq.index colo l in
    let co_o = Seq.index cohi l in
    let zm : i32 = mk_int (zeta_r (31 - u/2)) in
    T.lemma_chunks_of_re_avx2_index re u l;
    T.lemma_chunks_of_re_avx2_index re (u+1) l;
    T.lemma_chunks_of_re_avx2_index re_fut u l;
    T.lemma_chunks_of_re_avx2_index re_fut (u+1) l;
    assert (co_e == add_mod_opaque ci_e ci_o);
    assert (co_o == mont_mul (sub_mod_opaque ci_o ci_e) zm);
    T.lemma_is_i32b_poly_avx2_elim bnd re u l;
    T.lemma_is_i32b_poly_avx2_elim bnd re (u+1) l;
    assert (Spec.Utils.is_i32b bnd ci_e);
    assert (Spec.Utils.is_i32b bnd ci_o);
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    assert (v co_e == v ci_e + v ci_o);
    let d : i32 = sub_mod_opaque ci_o ci_e in
    assert (v d == v ci_o - v ci_e);
    assert (Spec.Utils.is_i32b 8380416 zm);
    C.lemma_mont_mul_bound_and_mod_q d zm;
    assert (Spec.Utils.is_i32b 8380416 co_o);
    let idx : nat = 31 - u/2 in
    C.lemma_v_zetas_eq_zeta idx
#pop-options

(* Pack the 8 lanes of a lo-unit pair into the opaque cross atom + bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l3_cross_chunk_avx2
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (u:nat{u<32 /\ u%2==0})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ (forall (l:nat{l<8}). inv_l3_cross_chunkfact re re_fut u l))
        (ensures
          unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+1))
            (Seq.index (T.chunks_of_re_avx2 re_fut) u) (Seq.index (T.chunks_of_re_avx2 re_fut) (u+1))
            (mk_i32 (zeta_r (31 - u/2))) /\
          (forall (l:nat). l < 8 ==>
            Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut u).f_value (mk_u64 l)) /\
            Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut (u+1)).f_value (mk_u64 l))))
  = eliminate forall (l:nat{l<8}). inv_l3_cross_chunkfact re re_fut u l with 0;
    eliminate forall (l:nat{l<8}). inv_l3_cross_chunkfact re re_fut u l with 1;
    eliminate forall (l:nat{l<8}). inv_l3_cross_chunkfact re re_fut u l with 2;
    eliminate forall (l:nat{l<8}). inv_l3_cross_chunkfact re re_fut u l with 3;
    eliminate forall (l:nat{l<8}). inv_l3_cross_chunkfact re re_fut u l with 4;
    eliminate forall (l:nat{l<8}). inv_l3_cross_chunkfact re re_fut u l with 5;
    eliminate forall (l:nat{l<8}). inv_l3_cross_chunkfact re re_fut u l with 6;
    eliminate forall (l:nat{l<8}). inv_l3_cross_chunkfact re re_fut u l with 7;
    inv_l3_cross_pair_relations re re_fut bnd u 0;
    inv_l3_cross_pair_relations re re_fut bnd u 1;
    inv_l3_cross_pair_relations re re_fut bnd u 2;
    inv_l3_cross_pair_relations re re_fut bnd u 3;
    inv_l3_cross_pair_relations re re_fut bnd u 4;
    inv_l3_cross_pair_relations re re_fut bnd u 5;
    inv_l3_cross_pair_relations re re_fut bnd u 6;
    inv_l3_cross_pair_relations re re_fut bnd u 7;
    reveal_opaque (`%unit_post_inv_cross_avx2) unit_post_inv_cross_avx2;
    introduce forall (l:nat{l<8}).
        (Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut u).f_value (mk_u64 l)) /\
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut (u+1)).f_value (mk_u64 l)))
    with (T.lemma_chunks_of_re_avx2_index re_fut u l;
          T.lemma_chunks_of_re_avx2_index re_fut (u+1) l)
#pop-options

(* Driver compose: plain refined-forall of cross atom -> intt_layer flat congruence.
   (A plain `forall (u:nat{u<32})` requires avoids the 32-way forall32 ground unroll
   that the opaque cross atom makes Z3 choke on.) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l3_cross_driver_compose_avx2
      (orig fut: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : Lemma
        (requires
          (forall (u:nat{u<32}). (u % 2 == 0) ==>
            unit_post_inv_cross_avx2 (Seq.index orig u) (Seq.index orig (u+1))
              (Seq.index fut u) (Seq.index fut (u+1))
              (mk_i32 (zeta_r (31 - u/2)))))
        (ensures
          (let in_flat = C.simd_units_to_array orig in
           let out_flat = C.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 3) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (zeta_r (31 - u/2)) in
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 2 == 0) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+1) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+1) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
          (v (Seq.index co_hi l)) % 8380417 ==
            ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm u) * 8265825) % 8380417))
      = if (u % 2 = 0) then
          inv_lemma_atom_to_bf_inv_cross_avx2 (Seq.index orig u) (Seq.index orig (u+1))
                                              (Seq.index fut u) (Seq.index fut (u+1)) (zm u)
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 2 == 0) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (31 - u/2) ] <: i32) * pow2 32) % 8380417)
      = if (u % 2 = 0) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = zeta_r (31 - u/2) in
          C.lemma_v_zetas_eq_zeta (31 - u/2)
        end
     in Classical.forall_intro aux_z);
    C.lemma_intt_layer_3_cross_to_hacspec_poly orig fut zm
#pop-options

(* Establish the (plain refined-forall) cross atom + per-unit bound from the
   chunkfacts.  The atom forall is dispatched per even lo-unit u via the chunk
   lemma; the bound forall is dispatched even/odd via the same chunk lemma at
   the even lo-unit u - u%2. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_l3_cross_atoms_and_bounds
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd orig_re /\
          (forall (u:nat{u<32 /\ u%2==0}) (l:nat{l<8}). inv_l3_cross_chunkfact orig_re re u l))
        (ensures
          (forall (u:nat{u<32}). (u % 2 == 0) ==>
            unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 orig_re) u) (Seq.index (T.chunks_of_re_avx2 orig_re) (u+1))
              (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+1))
              (mk_i32 (zeta_r (31 - u/2)))) /\
          (forall (u:nat) (l:nat). u<32 /\ l<8 ==>
             Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l))))
  = (let aux (u:nat{u<32}) : Lemma
        ((u % 2 == 0) ==>
          unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 orig_re) u) (Seq.index (T.chunks_of_re_avx2 orig_re) (u+1))
            (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+1))
            (mk_i32 (zeta_r (31 - u/2))))
      = if (u % 2 = 0) then inv_lemma_l3_cross_chunk_avx2 orig_re re bnd u
     in Classical.forall_intro aux);
    (let auxb (i:nat{i<16}) (l:nat{l<8}) : Lemma
        (Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re (2*i)).f_value (mk_u64 l)) /\
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re (2*i+1)).f_value (mk_u64 l)))
      = inv_lemma_l3_cross_chunk_avx2 orig_re re bnd (2*i)
     in Classical.forall_intro_2 auxb);
    introduce forall (u:nat) (l:nat). u<32 /\ l<8 ==>
        Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l))
    with (if u < 32 && l < 8 then begin
            let i : nat = u / 2 in
            FStar.Math.Lemmas.lemma_div_mod u 2;
            assert (i < 16 /\ (2*i == u \/ 2*i+1 == u))
          end)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_inv_l3_full_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd orig_re /\ invert_ntt_outer_3_plus_spec 3 orig_re re)
        (ensures
          T.is_i32b_poly_avx2 (2*bnd) re /\
          (let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 3) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = inv_l3_cross_chunkfacts_from_post orig_re re;
    inv_l3_cross_atoms_and_bounds orig_re re bnd;
    T.lemma_is_i32b_poly_avx2_intro (2*bnd) re;
    inv_lemma_l3_cross_driver_compose_avx2 (T.chunks_of_re_avx2 orig_re) (T.chunks_of_re_avx2 re)
#pop-options


(* ===== INVERSE LAYER 4 (cross, step_by=2, gap=4, zeta_rank=15) ============= *)

(* The layer-4 post body at index j (step_by=2: w=j/2, ll=j%2, uu=w*4+ll). *)
unfold let inv_l4_post_body
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    (j:nat{j<32}) : Type0 =
  j < 16 ==>
    (let w = j / 2 in let ll = j % 2 in
     let zeta = mk_i32 (zeta_r (15 - w)) in
     let uu = w * 4 + ll in
     let  re_j = (Seq.index  re uu).f_value in
     let nre_j = (Seq.index re_fut uu).f_value in
     let  re_j'= (Seq.index  re (uu + 2)).f_value in
     let nre_j'= (Seq.index re_fut (uu + 2)).f_value in
     forall i. (to_i32x8 nre_j i, to_i32x8 nre_j' i) ==
                inv_ntt_step zeta (to_i32x8 re_j i, to_i32x8 re_j' i))

(* Per (u,l) symbolic chunkfact: lo-unit u (u % 4 < 2), units u, u+2, lane l. *)
unfold let inv_l4_cross_chunkfact
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    (u:nat{u<32 /\ u % 4 < 2}) (l:nat{l<8}) : Type0 =
  let re_lo = Seq.index re u in let re_hi = Seq.index re (u+2) in
  let nre_lo = Seq.index re_fut u in let nre_hi = Seq.index re_fut (u+2) in
  let zeta = zeta_r (15 - u/4) in
  (to_i32x8 nre_lo.f_value (mk_u64 l), to_i32x8 nre_hi.f_value (mk_u64 l)) ==
    inv_ntt_step (mk_int zeta) (to_i32x8 re_lo.f_value (mk_u64 l), to_i32x8 re_hi.f_value (mk_u64 l))

(* Lift the layer post (forall32 j<16) into per-(u,l) chunkfacts.  For each lo-unit u,
   instantiate the post at j = (u/4)*2 + u%4 (so w=j/2=u/4, ll=j%2=u%4, uu=u). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_l4_cross_chunkfacts_from_post
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
        (requires invert_ntt_outer_3_plus_spec 4 re re_fut)
        (ensures forall (u:nat{u<32 /\ u % 4 < 2}) (l:nat{l<8}). inv_l4_cross_chunkfact re re_fut u l)
  = assert_norm (invert_ntt_outer_3_plus_spec 4 re re_fut ==
       Spec.Utils.forall32 (inv_l4_post_body re re_fut));
    FN.forall32_elim_1d (inv_l4_post_body re re_fut);
    let aux (u:nat{u<32 /\ u % 4 < 2}) (l:nat{l<8}) : Lemma (inv_l4_cross_chunkfact re re_fut u l) =
      let base : nat = u / 4 in let r : nat = u % 4 in
      FStar.Math.Lemmas.lemma_div_mod u 4;
      let j : nat = base * 2 + r in
      FStar.Math.Lemmas.lemma_div_mod j 2;
      FStar.Math.Lemmas.small_div r 2;
      FStar.Math.Lemmas.small_mod r 2;
      FStar.Math.Lemmas.lemma_div_plus r base 2;
      FStar.Math.Lemmas.lemma_mod_plus r base 2;
      assert (j < 16 /\ j / 2 == base /\ j % 2 == r /\ base * 4 + r == u);
      assert (inv_l4_post_body re re_fut j);
      assert (v (mk_u64 l) == l)
    in Classical.forall_intro_2 aux
#pop-options

(* Per (u,l): chunkfact + input bound -> GS facts + zeta fact + output bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let inv_l4_cross_pair_relations
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (u:nat{u<32 /\ u % 4 < 2}) (l:nat{l<8})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ inv_l4_cross_chunkfact re re_fut u l)
        (ensures
          (let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (15 - u/4) ] in
           let zm : i32 = mk_int (zeta_r (15 - u/4)) in
           let cilo = Seq.index (T.chunks_of_re_avx2 re) u in
           let cihi = Seq.index (T.chunks_of_re_avx2 re) (u+2) in
           let colo = Seq.index (T.chunks_of_re_avx2 re_fut) u in
           let cohi = Seq.index (T.chunks_of_re_avx2 re_fut) (u+2) in
           v (Seq.index colo l) == v (Seq.index cilo l) + v (Seq.index cihi l) /\
           (v (Seq.index cohi l)) % 8380417 ==
             ((v (Seq.index cihi l) - v (Seq.index cilo l)) * v zm * 8265825) % 8380417 /\
           (v zm) % 8380417 == (v z * pow2 32) % 8380417 /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index colo l) /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index cohi l)))
  = let cilo = Seq.index (T.chunks_of_re_avx2 re) u in
    let cihi = Seq.index (T.chunks_of_re_avx2 re) (u+2) in
    let colo = Seq.index (T.chunks_of_re_avx2 re_fut) u in
    let cohi = Seq.index (T.chunks_of_re_avx2 re_fut) (u+2) in
    let ci_e = Seq.index cilo l in
    let ci_o = Seq.index cihi l in
    let co_e = Seq.index colo l in
    let co_o = Seq.index cohi l in
    let zm : i32 = mk_int (zeta_r (15 - u/4)) in
    T.lemma_chunks_of_re_avx2_index re u l;
    T.lemma_chunks_of_re_avx2_index re (u+2) l;
    T.lemma_chunks_of_re_avx2_index re_fut u l;
    T.lemma_chunks_of_re_avx2_index re_fut (u+2) l;
    assert (co_e == add_mod_opaque ci_e ci_o);
    assert (co_o == mont_mul (sub_mod_opaque ci_o ci_e) zm);
    T.lemma_is_i32b_poly_avx2_elim bnd re u l;
    T.lemma_is_i32b_poly_avx2_elim bnd re (u+2) l;
    assert (Spec.Utils.is_i32b bnd ci_e);
    assert (Spec.Utils.is_i32b bnd ci_o);
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    assert (v co_e == v ci_e + v ci_o);
    let d : i32 = sub_mod_opaque ci_o ci_e in
    assert (v d == v ci_o - v ci_e);
    assert (Spec.Utils.is_i32b 8380416 zm);
    C.lemma_mont_mul_bound_and_mod_q d zm;
    assert (Spec.Utils.is_i32b 8380416 co_o);
    let idx : nat = 15 - u/4 in
    C.lemma_v_zetas_eq_zeta idx
#pop-options

(* Pack the 8 lanes of a lo-unit pair into the opaque cross atom + bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l4_cross_chunk_avx2
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (u:nat{u<32 /\ u % 4 < 2})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ (forall (l:nat{l<8}). inv_l4_cross_chunkfact re re_fut u l))
        (ensures
          unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+2))
            (Seq.index (T.chunks_of_re_avx2 re_fut) u) (Seq.index (T.chunks_of_re_avx2 re_fut) (u+2))
            (mk_i32 (zeta_r (15 - u/4))) /\
          (forall (l:nat). l < 8 ==>
            Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut u).f_value (mk_u64 l)) /\
            Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut (u+2)).f_value (mk_u64 l))))
  = eliminate forall (l:nat{l<8}). inv_l4_cross_chunkfact re re_fut u l with 0;
    eliminate forall (l:nat{l<8}). inv_l4_cross_chunkfact re re_fut u l with 1;
    eliminate forall (l:nat{l<8}). inv_l4_cross_chunkfact re re_fut u l with 2;
    eliminate forall (l:nat{l<8}). inv_l4_cross_chunkfact re re_fut u l with 3;
    eliminate forall (l:nat{l<8}). inv_l4_cross_chunkfact re re_fut u l with 4;
    eliminate forall (l:nat{l<8}). inv_l4_cross_chunkfact re re_fut u l with 5;
    eliminate forall (l:nat{l<8}). inv_l4_cross_chunkfact re re_fut u l with 6;
    eliminate forall (l:nat{l<8}). inv_l4_cross_chunkfact re re_fut u l with 7;
    inv_l4_cross_pair_relations re re_fut bnd u 0;
    inv_l4_cross_pair_relations re re_fut bnd u 1;
    inv_l4_cross_pair_relations re re_fut bnd u 2;
    inv_l4_cross_pair_relations re re_fut bnd u 3;
    inv_l4_cross_pair_relations re re_fut bnd u 4;
    inv_l4_cross_pair_relations re re_fut bnd u 5;
    inv_l4_cross_pair_relations re re_fut bnd u 6;
    inv_l4_cross_pair_relations re re_fut bnd u 7;
    reveal_opaque (`%unit_post_inv_cross_avx2) unit_post_inv_cross_avx2;
    introduce forall (l:nat{l<8}).
        (Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut u).f_value (mk_u64 l)) /\
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut (u+2)).f_value (mk_u64 l)))
    with (T.lemma_chunks_of_re_avx2_index re_fut u l;
          T.lemma_chunks_of_re_avx2_index re_fut (u+2) l)
#pop-options

(* Establish the (plain refined-forall) cross atom + per-unit bound from chunkfacts. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_l4_cross_atoms_and_bounds
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd orig_re /\
          (forall (u:nat{u<32 /\ u % 4 < 2}) (l:nat{l<8}). inv_l4_cross_chunkfact orig_re re u l))
        (ensures
          (forall (u:nat{u<32}). (u % 4 < 2) ==>
            unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 orig_re) u) (Seq.index (T.chunks_of_re_avx2 orig_re) (u+2))
              (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+2))
              (mk_i32 (zeta_r (15 - u/4)))) /\
          (forall (u:nat) (l:nat). u<32 /\ l<8 ==>
             Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l))))
  = (let aux (u:nat{u<32}) : Lemma
        ((u % 4 < 2) ==>
          unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 orig_re) u) (Seq.index (T.chunks_of_re_avx2 orig_re) (u+2))
            (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+2))
            (mk_i32 (zeta_r (15 - u/4))))
      = if (u % 4 < 2) then inv_lemma_l4_cross_chunk_avx2 orig_re re bnd u
     in Classical.forall_intro aux);
    (let auxb (i:nat{i<16}) (l:nat{l<8}) : Lemma
        (let u = ((i / 2) * 4) + (i % 2) in
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l)) /\
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re (u+2)).f_value (mk_u64 l)))
      = let u = ((i / 2) * 4) + (i % 2) in
        FStar.Math.Lemmas.lemma_div_mod i 2;
        assert (u < 32 /\ u % 4 < 2);
        inv_lemma_l4_cross_chunk_avx2 orig_re re bnd u
     in Classical.forall_intro_2 auxb);
    introduce forall (u:nat) (l:nat). u<32 /\ l<8 ==>
        Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l))
    with (if u < 32 && l < 8 then begin
            let base : nat = u / 4 in let r : nat = u % 4 in
            FStar.Math.Lemmas.lemma_div_mod u 4;
            let i : nat = base * 2 + (if r < 2 then r else r - 2) in
            FStar.Math.Lemmas.lemma_div_mod i 2;
            assert (i < 16);
            let ulo = ((i / 2) * 4) + (i % 2) in
            assert (ulo == u \/ ulo + 2 == u)
          end)
#pop-options

(* Driver compose: plain refined-forall of cross atom -> intt_layer flat congruence. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l4_cross_driver_compose_avx2
      (orig fut: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : Lemma
        (requires
          (forall (u:nat{u<32}). (u % 4 < 2) ==>
            unit_post_inv_cross_avx2 (Seq.index orig u) (Seq.index orig (u+2))
              (Seq.index fut u) (Seq.index fut (u+2))
              (mk_i32 (zeta_r (15 - u/4)))))
        (ensures
          (let in_flat = C.simd_units_to_array orig in
           let out_flat = C.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 4) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (zeta_r (15 - u/4)) in
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 4 < 2) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+2) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+2) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
          (v (Seq.index co_hi l)) % 8380417 ==
            ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm u) * 8265825) % 8380417))
      = if (u % 4 < 2) then
          inv_lemma_atom_to_bf_inv_cross_avx2 (Seq.index orig u) (Seq.index orig (u+2))
                                              (Seq.index fut u) (Seq.index fut (u+2)) (zm u)
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 4 < 2) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (15 - u/4) ] <: i32) * pow2 32) % 8380417)
      = if (u % 4 < 2) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = zeta_r (15 - u/4) in
          C.lemma_v_zetas_eq_zeta (15 - u/4)
        end
     in Classical.forall_intro aux_z);
    C.lemma_intt_layer_4_cross_to_hacspec_poly orig fut zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_inv_l4_full_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd orig_re /\ invert_ntt_outer_3_plus_spec 4 orig_re re)
        (ensures
          T.is_i32b_poly_avx2 (2*bnd) re /\
          (let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 4) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = inv_l4_cross_chunkfacts_from_post orig_re re;
    inv_l4_cross_atoms_and_bounds orig_re re bnd;
    T.lemma_is_i32b_poly_avx2_intro (2*bnd) re;
    inv_lemma_l4_cross_driver_compose_avx2 (T.chunks_of_re_avx2 orig_re) (T.chunks_of_re_avx2 re)
#pop-options

(* ===== INVERSE LAYER 5 (cross, step_by=4, gap=8, zeta_rank=7) ============= *)

(* The layer-5 post body at index j (step_by=4: w=j/4, ll=j%4, uu=w*8+ll). *)
unfold let inv_l5_post_body
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    (j:nat{j<32}) : Type0 =
  j < 16 ==>
    (let w = j / 4 in let ll = j % 4 in
     let zeta = mk_i32 (zeta_r (7 - w)) in
     let uu = w * 8 + ll in
     let  re_j = (Seq.index  re uu).f_value in
     let nre_j = (Seq.index re_fut uu).f_value in
     let  re_j'= (Seq.index  re (uu + 4)).f_value in
     let nre_j'= (Seq.index re_fut (uu + 4)).f_value in
     forall i. (to_i32x8 nre_j i, to_i32x8 nre_j' i) ==
                inv_ntt_step zeta (to_i32x8 re_j i, to_i32x8 re_j' i))

(* Per (u,l) symbolic chunkfact: lo-unit u (u % 8 < 4), units u, u+4, lane l. *)
unfold let inv_l5_cross_chunkfact
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    (u:nat{u<32 /\ u % 8 < 4}) (l:nat{l<8}) : Type0 =
  let re_lo = Seq.index re u in let re_hi = Seq.index re (u+4) in
  let nre_lo = Seq.index re_fut u in let nre_hi = Seq.index re_fut (u+4) in
  let zeta = zeta_r (7 - u/8) in
  (to_i32x8 nre_lo.f_value (mk_u64 l), to_i32x8 nre_hi.f_value (mk_u64 l)) ==
    inv_ntt_step (mk_int zeta) (to_i32x8 re_lo.f_value (mk_u64 l), to_i32x8 re_hi.f_value (mk_u64 l))

(* Lift the layer post (forall32 j<16) into per-(u,l) chunkfacts.  For each lo-unit u,
   instantiate the post at j = (u/8)*4 + u%8 (so w=j/4=u/8, ll=j%4=u%8, uu=u). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_l5_cross_chunkfacts_from_post
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
        (requires invert_ntt_outer_3_plus_spec 5 re re_fut)
        (ensures forall (u:nat{u<32 /\ u % 8 < 4}) (l:nat{l<8}). inv_l5_cross_chunkfact re re_fut u l)
  = assert_norm (invert_ntt_outer_3_plus_spec 5 re re_fut ==
       Spec.Utils.forall32 (inv_l5_post_body re re_fut));
    FN.forall32_elim_1d (inv_l5_post_body re re_fut);
    let aux (u:nat{u<32 /\ u % 8 < 4}) (l:nat{l<8}) : Lemma (inv_l5_cross_chunkfact re re_fut u l) =
      let base : nat = u / 8 in let r : nat = u % 8 in
      FStar.Math.Lemmas.lemma_div_mod u 8;
      let j : nat = base * 4 + r in
      FStar.Math.Lemmas.lemma_div_mod j 4;
      FStar.Math.Lemmas.small_div r 4;
      FStar.Math.Lemmas.small_mod r 4;
      FStar.Math.Lemmas.lemma_div_plus r base 4;
      FStar.Math.Lemmas.lemma_mod_plus r base 4;
      assert (j < 16 /\ j / 4 == base /\ j % 4 == r /\ base * 8 + r == u);
      assert (inv_l5_post_body re re_fut j);
      assert (v (mk_u64 l) == l)
    in Classical.forall_intro_2 aux
#pop-options

(* Per (u,l): chunkfact + input bound -> GS facts + zeta fact + output bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let inv_l5_cross_pair_relations
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (u:nat{u<32 /\ u % 8 < 4}) (l:nat{l<8})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ inv_l5_cross_chunkfact re re_fut u l)
        (ensures
          (let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (7 - u/8) ] in
           let zm : i32 = mk_int (zeta_r (7 - u/8)) in
           let cilo = Seq.index (T.chunks_of_re_avx2 re) u in
           let cihi = Seq.index (T.chunks_of_re_avx2 re) (u+4) in
           let colo = Seq.index (T.chunks_of_re_avx2 re_fut) u in
           let cohi = Seq.index (T.chunks_of_re_avx2 re_fut) (u+4) in
           v (Seq.index colo l) == v (Seq.index cilo l) + v (Seq.index cihi l) /\
           (v (Seq.index cohi l)) % 8380417 ==
             ((v (Seq.index cihi l) - v (Seq.index cilo l)) * v zm * 8265825) % 8380417 /\
           (v zm) % 8380417 == (v z * pow2 32) % 8380417 /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index colo l) /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index cohi l)))
  = let cilo = Seq.index (T.chunks_of_re_avx2 re) u in
    let cihi = Seq.index (T.chunks_of_re_avx2 re) (u+4) in
    let colo = Seq.index (T.chunks_of_re_avx2 re_fut) u in
    let cohi = Seq.index (T.chunks_of_re_avx2 re_fut) (u+4) in
    let ci_e = Seq.index cilo l in
    let ci_o = Seq.index cihi l in
    let co_e = Seq.index colo l in
    let co_o = Seq.index cohi l in
    let zm : i32 = mk_int (zeta_r (7 - u/8)) in
    T.lemma_chunks_of_re_avx2_index re u l;
    T.lemma_chunks_of_re_avx2_index re (u+4) l;
    T.lemma_chunks_of_re_avx2_index re_fut u l;
    T.lemma_chunks_of_re_avx2_index re_fut (u+4) l;
    assert (co_e == add_mod_opaque ci_e ci_o);
    assert (co_o == mont_mul (sub_mod_opaque ci_o ci_e) zm);
    T.lemma_is_i32b_poly_avx2_elim bnd re u l;
    T.lemma_is_i32b_poly_avx2_elim bnd re (u+4) l;
    assert (Spec.Utils.is_i32b bnd ci_e);
    assert (Spec.Utils.is_i32b bnd ci_o);
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    assert (v co_e == v ci_e + v ci_o);
    let d : i32 = sub_mod_opaque ci_o ci_e in
    assert (v d == v ci_o - v ci_e);
    assert (Spec.Utils.is_i32b 8380416 zm);
    C.lemma_mont_mul_bound_and_mod_q d zm;
    assert (Spec.Utils.is_i32b 8380416 co_o);
    let idx : nat = 7 - u/8 in
    C.lemma_v_zetas_eq_zeta idx
#pop-options

(* Pack the 8 lanes of a lo-unit pair into the opaque cross atom + bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l5_cross_chunk_avx2
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (u:nat{u<32 /\ u % 8 < 4})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ (forall (l:nat{l<8}). inv_l5_cross_chunkfact re re_fut u l))
        (ensures
          unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+4))
            (Seq.index (T.chunks_of_re_avx2 re_fut) u) (Seq.index (T.chunks_of_re_avx2 re_fut) (u+4))
            (mk_i32 (zeta_r (7 - u/8))) /\
          (forall (l:nat). l < 8 ==>
            Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut u).f_value (mk_u64 l)) /\
            Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut (u+4)).f_value (mk_u64 l))))
  = eliminate forall (l:nat{l<8}). inv_l5_cross_chunkfact re re_fut u l with 0;
    eliminate forall (l:nat{l<8}). inv_l5_cross_chunkfact re re_fut u l with 1;
    eliminate forall (l:nat{l<8}). inv_l5_cross_chunkfact re re_fut u l with 2;
    eliminate forall (l:nat{l<8}). inv_l5_cross_chunkfact re re_fut u l with 3;
    eliminate forall (l:nat{l<8}). inv_l5_cross_chunkfact re re_fut u l with 4;
    eliminate forall (l:nat{l<8}). inv_l5_cross_chunkfact re re_fut u l with 5;
    eliminate forall (l:nat{l<8}). inv_l5_cross_chunkfact re re_fut u l with 6;
    eliminate forall (l:nat{l<8}). inv_l5_cross_chunkfact re re_fut u l with 7;
    inv_l5_cross_pair_relations re re_fut bnd u 0;
    inv_l5_cross_pair_relations re re_fut bnd u 1;
    inv_l5_cross_pair_relations re re_fut bnd u 2;
    inv_l5_cross_pair_relations re re_fut bnd u 3;
    inv_l5_cross_pair_relations re re_fut bnd u 4;
    inv_l5_cross_pair_relations re re_fut bnd u 5;
    inv_l5_cross_pair_relations re re_fut bnd u 6;
    inv_l5_cross_pair_relations re re_fut bnd u 7;
    reveal_opaque (`%unit_post_inv_cross_avx2) unit_post_inv_cross_avx2;
    introduce forall (l:nat{l<8}).
        (Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut u).f_value (mk_u64 l)) /\
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut (u+4)).f_value (mk_u64 l)))
    with (T.lemma_chunks_of_re_avx2_index re_fut u l;
          T.lemma_chunks_of_re_avx2_index re_fut (u+4) l)
#pop-options

(* Establish the (plain refined-forall) cross atom + per-unit bound from chunkfacts. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_l5_cross_atoms_and_bounds
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd orig_re /\
          (forall (u:nat{u<32 /\ u % 8 < 4}) (l:nat{l<8}). inv_l5_cross_chunkfact orig_re re u l))
        (ensures
          (forall (u:nat{u<32}). (u % 8 < 4) ==>
            unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 orig_re) u) (Seq.index (T.chunks_of_re_avx2 orig_re) (u+4))
              (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+4))
              (mk_i32 (zeta_r (7 - u/8)))) /\
          (forall (u:nat) (l:nat). u<32 /\ l<8 ==>
             Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l))))
  = (let aux (u:nat{u<32}) : Lemma
        ((u % 8 < 4) ==>
          unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 orig_re) u) (Seq.index (T.chunks_of_re_avx2 orig_re) (u+4))
            (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+4))
            (mk_i32 (zeta_r (7 - u/8))))
      = if (u % 8 < 4) then inv_lemma_l5_cross_chunk_avx2 orig_re re bnd u
     in Classical.forall_intro aux);
    (let auxb (i:nat{i<16}) (l:nat{l<8}) : Lemma
        (let u = ((i / 4) * 8) + (i % 4) in
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l)) /\
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re (u+4)).f_value (mk_u64 l)))
      = let u = ((i / 4) * 8) + (i % 4) in
        FStar.Math.Lemmas.lemma_div_mod i 4;
        assert (u < 32 /\ u % 8 < 4);
        inv_lemma_l5_cross_chunk_avx2 orig_re re bnd u
     in Classical.forall_intro_2 auxb);
    introduce forall (u:nat) (l:nat). u<32 /\ l<8 ==>
        Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l))
    with (if u < 32 && l < 8 then begin
            let base : nat = u / 8 in let r : nat = u % 8 in
            FStar.Math.Lemmas.lemma_div_mod u 8;
            let i : nat = base * 4 + (if r < 4 then r else r - 4) in
            FStar.Math.Lemmas.lemma_div_mod i 4;
            assert (i < 16);
            let ulo = ((i / 4) * 8) + (i % 4) in
            assert (ulo == u \/ ulo + 4 == u)
          end)
#pop-options

(* Driver compose: plain refined-forall of cross atom -> intt_layer flat congruence. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l5_cross_driver_compose_avx2
      (orig fut: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : Lemma
        (requires
          (forall (u:nat{u<32}). (u % 8 < 4) ==>
            unit_post_inv_cross_avx2 (Seq.index orig u) (Seq.index orig (u+4))
              (Seq.index fut u) (Seq.index fut (u+4))
              (mk_i32 (zeta_r (7 - u/8)))))
        (ensures
          (let in_flat = C.simd_units_to_array orig in
           let out_flat = C.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 5) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (zeta_r (7 - u/8)) in
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 8 < 4) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+4) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+4) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
          (v (Seq.index co_hi l)) % 8380417 ==
            ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm u) * 8265825) % 8380417))
      = if (u % 8 < 4) then
          inv_lemma_atom_to_bf_inv_cross_avx2 (Seq.index orig u) (Seq.index orig (u+4))
                                              (Seq.index fut u) (Seq.index fut (u+4)) (zm u)
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 8 < 4) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (7 - u/8) ] <: i32) * pow2 32) % 8380417)
      = if (u % 8 < 4) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = zeta_r (7 - u/8) in
          C.lemma_v_zetas_eq_zeta (7 - u/8)
        end
     in Classical.forall_intro aux_z);
    C.lemma_intt_layer_5_cross_to_hacspec_poly orig fut zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_inv_l5_full_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd orig_re /\ invert_ntt_outer_3_plus_spec 5 orig_re re)
        (ensures
          T.is_i32b_poly_avx2 (2*bnd) re /\
          (let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 5) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = inv_l5_cross_chunkfacts_from_post orig_re re;
    inv_l5_cross_atoms_and_bounds orig_re re bnd;
    T.lemma_is_i32b_poly_avx2_intro (2*bnd) re;
    inv_lemma_l5_cross_driver_compose_avx2 (T.chunks_of_re_avx2 orig_re) (T.chunks_of_re_avx2 re)
#pop-options

(* ===== INVERSE LAYER 6 (cross, step_by=8, gap=16, zeta_rank=3) ============= *)

(* The layer-6 post body at index j (step_by=8: w=j/8, ll=j%8, uu=w*16+ll). *)
unfold let inv_l6_post_body
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    (j:nat{j<32}) : Type0 =
  j < 16 ==>
    (let w = j / 8 in let ll = j % 8 in
     let zeta = mk_i32 (zeta_r (3 - w)) in
     let uu = w * 16 + ll in
     let  re_j = (Seq.index  re uu).f_value in
     let nre_j = (Seq.index re_fut uu).f_value in
     let  re_j'= (Seq.index  re (uu + 8)).f_value in
     let nre_j'= (Seq.index re_fut (uu + 8)).f_value in
     forall i. (to_i32x8 nre_j i, to_i32x8 nre_j' i) ==
                inv_ntt_step zeta (to_i32x8 re_j i, to_i32x8 re_j' i))

(* Per (u,l) symbolic chunkfact: lo-unit u (u % 16 < 8), units u, u+8, lane l. *)
unfold let inv_l6_cross_chunkfact
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    (u:nat{u<32 /\ u % 16 < 8}) (l:nat{l<8}) : Type0 =
  let re_lo = Seq.index re u in let re_hi = Seq.index re (u+8) in
  let nre_lo = Seq.index re_fut u in let nre_hi = Seq.index re_fut (u+8) in
  let zeta = zeta_r (3 - u/16) in
  (to_i32x8 nre_lo.f_value (mk_u64 l), to_i32x8 nre_hi.f_value (mk_u64 l)) ==
    inv_ntt_step (mk_int zeta) (to_i32x8 re_lo.f_value (mk_u64 l), to_i32x8 re_hi.f_value (mk_u64 l))

(* Lift the layer post (forall32 j<16) into per-(u,l) chunkfacts.  For each lo-unit u,
   instantiate the post at j = (u/16)*8 + u%16 (so w=j/8=u/16, ll=j%8=u%16, uu=u). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_l6_cross_chunkfacts_from_post
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
        (requires invert_ntt_outer_3_plus_spec 6 re re_fut)
        (ensures forall (u:nat{u<32 /\ u % 16 < 8}) (l:nat{l<8}). inv_l6_cross_chunkfact re re_fut u l)
  = assert_norm (invert_ntt_outer_3_plus_spec 6 re re_fut ==
       Spec.Utils.forall32 (inv_l6_post_body re re_fut));
    FN.forall32_elim_1d (inv_l6_post_body re re_fut);
    let aux (u:nat{u<32 /\ u % 16 < 8}) (l:nat{l<8}) : Lemma (inv_l6_cross_chunkfact re re_fut u l) =
      let base : nat = u / 16 in let r : nat = u % 16 in
      FStar.Math.Lemmas.lemma_div_mod u 16;
      let j : nat = base * 8 + r in
      FStar.Math.Lemmas.lemma_div_mod j 8;
      FStar.Math.Lemmas.small_div r 8;
      FStar.Math.Lemmas.small_mod r 8;
      FStar.Math.Lemmas.lemma_div_plus r base 8;
      FStar.Math.Lemmas.lemma_mod_plus r base 8;
      assert (j < 16 /\ j / 8 == base /\ j % 8 == r /\ base * 16 + r == u);
      assert (inv_l6_post_body re re_fut j);
      assert (v (mk_u64 l) == l)
    in Classical.forall_intro_2 aux
#pop-options

(* Per (u,l): chunkfact + input bound -> GS facts + zeta fact + output bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let inv_l6_cross_pair_relations
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (u:nat{u<32 /\ u % 16 < 8}) (l:nat{l<8})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ inv_l6_cross_chunkfact re re_fut u l)
        (ensures
          (let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (3 - u/16) ] in
           let zm : i32 = mk_int (zeta_r (3 - u/16)) in
           let cilo = Seq.index (T.chunks_of_re_avx2 re) u in
           let cihi = Seq.index (T.chunks_of_re_avx2 re) (u+8) in
           let colo = Seq.index (T.chunks_of_re_avx2 re_fut) u in
           let cohi = Seq.index (T.chunks_of_re_avx2 re_fut) (u+8) in
           v (Seq.index colo l) == v (Seq.index cilo l) + v (Seq.index cihi l) /\
           (v (Seq.index cohi l)) % 8380417 ==
             ((v (Seq.index cihi l) - v (Seq.index cilo l)) * v zm * 8265825) % 8380417 /\
           (v zm) % 8380417 == (v z * pow2 32) % 8380417 /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index colo l) /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index cohi l)))
  = let cilo = Seq.index (T.chunks_of_re_avx2 re) u in
    let cihi = Seq.index (T.chunks_of_re_avx2 re) (u+8) in
    let colo = Seq.index (T.chunks_of_re_avx2 re_fut) u in
    let cohi = Seq.index (T.chunks_of_re_avx2 re_fut) (u+8) in
    let ci_e = Seq.index cilo l in
    let ci_o = Seq.index cihi l in
    let co_e = Seq.index colo l in
    let co_o = Seq.index cohi l in
    let zm : i32 = mk_int (zeta_r (3 - u/16)) in
    T.lemma_chunks_of_re_avx2_index re u l;
    T.lemma_chunks_of_re_avx2_index re (u+8) l;
    T.lemma_chunks_of_re_avx2_index re_fut u l;
    T.lemma_chunks_of_re_avx2_index re_fut (u+8) l;
    assert (co_e == add_mod_opaque ci_e ci_o);
    assert (co_o == mont_mul (sub_mod_opaque ci_o ci_e) zm);
    T.lemma_is_i32b_poly_avx2_elim bnd re u l;
    T.lemma_is_i32b_poly_avx2_elim bnd re (u+8) l;
    assert (Spec.Utils.is_i32b bnd ci_e);
    assert (Spec.Utils.is_i32b bnd ci_o);
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    assert (v co_e == v ci_e + v ci_o);
    let d : i32 = sub_mod_opaque ci_o ci_e in
    assert (v d == v ci_o - v ci_e);
    assert (Spec.Utils.is_i32b 8380416 zm);
    C.lemma_mont_mul_bound_and_mod_q d zm;
    assert (Spec.Utils.is_i32b 8380416 co_o);
    let idx : nat = 3 - u/16 in
    C.lemma_v_zetas_eq_zeta idx
#pop-options

(* Pack the 8 lanes of a lo-unit pair into the opaque cross atom + bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l6_cross_chunk_avx2
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (u:nat{u<32 /\ u % 16 < 8})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ (forall (l:nat{l<8}). inv_l6_cross_chunkfact re re_fut u l))
        (ensures
          unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+8))
            (Seq.index (T.chunks_of_re_avx2 re_fut) u) (Seq.index (T.chunks_of_re_avx2 re_fut) (u+8))
            (mk_i32 (zeta_r (3 - u/16))) /\
          (forall (l:nat). l < 8 ==>
            Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut u).f_value (mk_u64 l)) /\
            Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut (u+8)).f_value (mk_u64 l))))
  = eliminate forall (l:nat{l<8}). inv_l6_cross_chunkfact re re_fut u l with 0;
    eliminate forall (l:nat{l<8}). inv_l6_cross_chunkfact re re_fut u l with 1;
    eliminate forall (l:nat{l<8}). inv_l6_cross_chunkfact re re_fut u l with 2;
    eliminate forall (l:nat{l<8}). inv_l6_cross_chunkfact re re_fut u l with 3;
    eliminate forall (l:nat{l<8}). inv_l6_cross_chunkfact re re_fut u l with 4;
    eliminate forall (l:nat{l<8}). inv_l6_cross_chunkfact re re_fut u l with 5;
    eliminate forall (l:nat{l<8}). inv_l6_cross_chunkfact re re_fut u l with 6;
    eliminate forall (l:nat{l<8}). inv_l6_cross_chunkfact re re_fut u l with 7;
    inv_l6_cross_pair_relations re re_fut bnd u 0;
    inv_l6_cross_pair_relations re re_fut bnd u 1;
    inv_l6_cross_pair_relations re re_fut bnd u 2;
    inv_l6_cross_pair_relations re re_fut bnd u 3;
    inv_l6_cross_pair_relations re re_fut bnd u 4;
    inv_l6_cross_pair_relations re re_fut bnd u 5;
    inv_l6_cross_pair_relations re re_fut bnd u 6;
    inv_l6_cross_pair_relations re re_fut bnd u 7;
    reveal_opaque (`%unit_post_inv_cross_avx2) unit_post_inv_cross_avx2;
    introduce forall (l:nat{l<8}).
        (Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut u).f_value (mk_u64 l)) /\
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut (u+8)).f_value (mk_u64 l)))
    with (T.lemma_chunks_of_re_avx2_index re_fut u l;
          T.lemma_chunks_of_re_avx2_index re_fut (u+8) l)
#pop-options

(* Establish the (plain refined-forall) cross atom + per-unit bound from chunkfacts. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_l6_cross_atoms_and_bounds
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd orig_re /\
          (forall (u:nat{u<32 /\ u % 16 < 8}) (l:nat{l<8}). inv_l6_cross_chunkfact orig_re re u l))
        (ensures
          (forall (u:nat{u<32}). (u % 16 < 8) ==>
            unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 orig_re) u) (Seq.index (T.chunks_of_re_avx2 orig_re) (u+8))
              (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+8))
              (mk_i32 (zeta_r (3 - u/16)))) /\
          (forall (u:nat) (l:nat). u<32 /\ l<8 ==>
             Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l))))
  = (let aux (u:nat{u<32}) : Lemma
        ((u % 16 < 8) ==>
          unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 orig_re) u) (Seq.index (T.chunks_of_re_avx2 orig_re) (u+8))
            (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+8))
            (mk_i32 (zeta_r (3 - u/16))))
      = if (u % 16 < 8) then inv_lemma_l6_cross_chunk_avx2 orig_re re bnd u
     in Classical.forall_intro aux);
    (let auxb (i:nat{i<16}) (l:nat{l<8}) : Lemma
        (let u = ((i / 8) * 16) + (i % 8) in
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l)) /\
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re (u+8)).f_value (mk_u64 l)))
      = let u = ((i / 8) * 16) + (i % 8) in
        FStar.Math.Lemmas.lemma_div_mod i 8;
        assert (u < 32 /\ u % 16 < 8);
        inv_lemma_l6_cross_chunk_avx2 orig_re re bnd u
     in Classical.forall_intro_2 auxb);
    introduce forall (u:nat) (l:nat). u<32 /\ l<8 ==>
        Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l))
    with (if u < 32 && l < 8 then begin
            let base : nat = u / 16 in let r : nat = u % 16 in
            FStar.Math.Lemmas.lemma_div_mod u 16;
            let i : nat = base * 8 + (if r < 8 then r else r - 8) in
            FStar.Math.Lemmas.lemma_div_mod i 8;
            assert (i < 16);
            let ulo = ((i / 8) * 16) + (i % 8) in
            assert (ulo == u \/ ulo + 8 == u)
          end)
#pop-options

(* Driver compose: plain refined-forall of cross atom -> intt_layer flat congruence. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l6_cross_driver_compose_avx2
      (orig fut: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : Lemma
        (requires
          (forall (u:nat{u<32}). (u % 16 < 8) ==>
            unit_post_inv_cross_avx2 (Seq.index orig u) (Seq.index orig (u+8))
              (Seq.index fut u) (Seq.index fut (u+8))
              (mk_i32 (zeta_r (3 - u/16)))))
        (ensures
          (let in_flat = C.simd_units_to_array orig in
           let out_flat = C.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 6) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (zeta_r (3 - u/16)) in
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 16 < 8) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+8) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+8) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
          (v (Seq.index co_hi l)) % 8380417 ==
            ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm u) * 8265825) % 8380417))
      = if (u % 16 < 8) then
          inv_lemma_atom_to_bf_inv_cross_avx2 (Seq.index orig u) (Seq.index orig (u+8))
                                              (Seq.index fut u) (Seq.index fut (u+8)) (zm u)
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 16 < 8) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (3 - u/16) ] <: i32) * pow2 32) % 8380417)
      = if (u % 16 < 8) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = zeta_r (3 - u/16) in
          C.lemma_v_zetas_eq_zeta (3 - u/16)
        end
     in Classical.forall_intro aux_z);
    C.lemma_intt_layer_6_cross_to_hacspec_poly orig fut zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_inv_l6_full_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd orig_re /\ invert_ntt_outer_3_plus_spec 6 orig_re re)
        (ensures
          T.is_i32b_poly_avx2 (2*bnd) re /\
          (let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 6) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = inv_l6_cross_chunkfacts_from_post orig_re re;
    inv_l6_cross_atoms_and_bounds orig_re re bnd;
    T.lemma_is_i32b_poly_avx2_intro (2*bnd) re;
    inv_lemma_l6_cross_driver_compose_avx2 (T.chunks_of_re_avx2 orig_re) (T.chunks_of_re_avx2 re)
#pop-options

(* ===== INVERSE LAYER 7 (cross, step_by=16, gap=32, zeta_rank=1) ============= *)

(* The layer-7 post body at index j (step_by=16: w=j/16, ll=j%16, uu=w*32+ll). *)
unfold let inv_l7_post_body
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    (j:nat{j<32}) : Type0 =
  j < 16 ==>
    (let w = j / 16 in let ll = j % 16 in
     let zeta = mk_i32 (zeta_r (1 - w)) in
     let uu = w * 32 + ll in
     let  re_j = (Seq.index  re uu).f_value in
     let nre_j = (Seq.index re_fut uu).f_value in
     let  re_j'= (Seq.index  re (uu + 16)).f_value in
     let nre_j'= (Seq.index re_fut (uu + 16)).f_value in
     forall i. (to_i32x8 nre_j i, to_i32x8 nre_j' i) ==
                inv_ntt_step zeta (to_i32x8 re_j i, to_i32x8 re_j' i))

(* Per (u,l) symbolic chunkfact: lo-unit u (u % 32 < 16), units u, u+16, lane l. *)
unfold let inv_l7_cross_chunkfact
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    (u:nat{u<32 /\ u % 32 < 16}) (l:nat{l<8}) : Type0 =
  let re_lo = Seq.index re u in let re_hi = Seq.index re (u+16) in
  let nre_lo = Seq.index re_fut u in let nre_hi = Seq.index re_fut (u+16) in
  let zeta = zeta_r (1 - u/32) in
  (to_i32x8 nre_lo.f_value (mk_u64 l), to_i32x8 nre_hi.f_value (mk_u64 l)) ==
    inv_ntt_step (mk_int zeta) (to_i32x8 re_lo.f_value (mk_u64 l), to_i32x8 re_hi.f_value (mk_u64 l))

(* Lift the layer post (forall32 j<16) into per-(u,l) chunkfacts.  For each lo-unit u,
   instantiate the post at j = (u/32)*16 + u%32 (so w=j/16=u/32, ll=j%16=u%32, uu=u). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_l7_cross_chunkfacts_from_post
    (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
        (requires invert_ntt_outer_3_plus_spec 7 re re_fut)
        (ensures forall (u:nat{u<32 /\ u % 32 < 16}) (l:nat{l<8}). inv_l7_cross_chunkfact re re_fut u l)
  = assert_norm (invert_ntt_outer_3_plus_spec 7 re re_fut ==
       Spec.Utils.forall32 (inv_l7_post_body re re_fut));
    FN.forall32_elim_1d (inv_l7_post_body re re_fut);
    let aux (u:nat{u<32 /\ u % 32 < 16}) (l:nat{l<8}) : Lemma (inv_l7_cross_chunkfact re re_fut u l) =
      let base : nat = u / 32 in let r : nat = u % 32 in
      FStar.Math.Lemmas.lemma_div_mod u 32;
      let j : nat = base * 16 + r in
      FStar.Math.Lemmas.lemma_div_mod j 16;
      FStar.Math.Lemmas.small_div r 16;
      FStar.Math.Lemmas.small_mod r 16;
      FStar.Math.Lemmas.lemma_div_plus r base 16;
      FStar.Math.Lemmas.lemma_mod_plus r base 16;
      assert (j < 16 /\ j / 16 == base /\ j % 16 == r /\ base * 32 + r == u);
      assert (inv_l7_post_body re re_fut j);
      assert (v (mk_u64 l) == l)
    in Classical.forall_intro_2 aux
#pop-options

(* Per (u,l): chunkfact + input bound -> GS facts + zeta fact + output bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let inv_l7_cross_pair_relations
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (u:nat{u<32 /\ u % 32 < 16}) (l:nat{l<8})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ inv_l7_cross_chunkfact re re_fut u l)
        (ensures
          (let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (1 - u/32) ] in
           let zm : i32 = mk_int (zeta_r (1 - u/32)) in
           let cilo = Seq.index (T.chunks_of_re_avx2 re) u in
           let cihi = Seq.index (T.chunks_of_re_avx2 re) (u+16) in
           let colo = Seq.index (T.chunks_of_re_avx2 re_fut) u in
           let cohi = Seq.index (T.chunks_of_re_avx2 re_fut) (u+16) in
           v (Seq.index colo l) == v (Seq.index cilo l) + v (Seq.index cihi l) /\
           (v (Seq.index cohi l)) % 8380417 ==
             ((v (Seq.index cihi l) - v (Seq.index cilo l)) * v zm * 8265825) % 8380417 /\
           (v zm) % 8380417 == (v z * pow2 32) % 8380417 /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index colo l) /\
           Spec.Utils.is_i32b (2*bnd) (Seq.index cohi l)))
  = let cilo = Seq.index (T.chunks_of_re_avx2 re) u in
    let cihi = Seq.index (T.chunks_of_re_avx2 re) (u+16) in
    let colo = Seq.index (T.chunks_of_re_avx2 re_fut) u in
    let cohi = Seq.index (T.chunks_of_re_avx2 re_fut) (u+16) in
    let ci_e = Seq.index cilo l in
    let ci_o = Seq.index cihi l in
    let co_e = Seq.index colo l in
    let co_o = Seq.index cohi l in
    let zm : i32 = mk_int (zeta_r (1 - u/32)) in
    T.lemma_chunks_of_re_avx2_index re u l;
    T.lemma_chunks_of_re_avx2_index re (u+16) l;
    T.lemma_chunks_of_re_avx2_index re_fut u l;
    T.lemma_chunks_of_re_avx2_index re_fut (u+16) l;
    assert (co_e == add_mod_opaque ci_e ci_o);
    assert (co_o == mont_mul (sub_mod_opaque ci_o ci_e) zm);
    T.lemma_is_i32b_poly_avx2_elim bnd re u l;
    T.lemma_is_i32b_poly_avx2_elim bnd re (u+16) l;
    assert (Spec.Utils.is_i32b bnd ci_e);
    assert (Spec.Utils.is_i32b bnd ci_o);
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    assert (v co_e == v ci_e + v ci_o);
    let d : i32 = sub_mod_opaque ci_o ci_e in
    assert (v d == v ci_o - v ci_e);
    assert (Spec.Utils.is_i32b 8380416 zm);
    C.lemma_mont_mul_bound_and_mod_q d zm;
    assert (Spec.Utils.is_i32b 8380416 co_o);
    let idx : nat = 1 - u/32 in
    C.lemma_v_zetas_eq_zeta idx
#pop-options

(* Pack the 8 lanes of a lo-unit pair into the opaque cross atom + bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l7_cross_chunk_avx2
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
      (u:nat{u<32 /\ u % 32 < 16})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd re /\ (forall (l:nat{l<8}). inv_l7_cross_chunkfact re re_fut u l))
        (ensures
          unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+16))
            (Seq.index (T.chunks_of_re_avx2 re_fut) u) (Seq.index (T.chunks_of_re_avx2 re_fut) (u+16))
            (mk_i32 (zeta_r (1 - u/32))) /\
          (forall (l:nat). l < 8 ==>
            Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut u).f_value (mk_u64 l)) /\
            Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut (u+16)).f_value (mk_u64 l))))
  = eliminate forall (l:nat{l<8}). inv_l7_cross_chunkfact re re_fut u l with 0;
    eliminate forall (l:nat{l<8}). inv_l7_cross_chunkfact re re_fut u l with 1;
    eliminate forall (l:nat{l<8}). inv_l7_cross_chunkfact re re_fut u l with 2;
    eliminate forall (l:nat{l<8}). inv_l7_cross_chunkfact re re_fut u l with 3;
    eliminate forall (l:nat{l<8}). inv_l7_cross_chunkfact re re_fut u l with 4;
    eliminate forall (l:nat{l<8}). inv_l7_cross_chunkfact re re_fut u l with 5;
    eliminate forall (l:nat{l<8}). inv_l7_cross_chunkfact re re_fut u l with 6;
    eliminate forall (l:nat{l<8}). inv_l7_cross_chunkfact re re_fut u l with 7;
    inv_l7_cross_pair_relations re re_fut bnd u 0;
    inv_l7_cross_pair_relations re re_fut bnd u 1;
    inv_l7_cross_pair_relations re re_fut bnd u 2;
    inv_l7_cross_pair_relations re re_fut bnd u 3;
    inv_l7_cross_pair_relations re re_fut bnd u 4;
    inv_l7_cross_pair_relations re re_fut bnd u 5;
    inv_l7_cross_pair_relations re re_fut bnd u 6;
    inv_l7_cross_pair_relations re re_fut bnd u 7;
    reveal_opaque (`%unit_post_inv_cross_avx2) unit_post_inv_cross_avx2;
    introduce forall (l:nat{l<8}).
        (Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut u).f_value (mk_u64 l)) /\
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re_fut (u+16)).f_value (mk_u64 l)))
    with (T.lemma_chunks_of_re_avx2_index re_fut u l;
          T.lemma_chunks_of_re_avx2_index re_fut (u+16) l)
#pop-options

(* Establish the (plain refined-forall) cross atom + per-unit bound from chunkfacts. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_l7_cross_atoms_and_bounds
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd orig_re /\
          (forall (u:nat{u<32 /\ u % 32 < 16}) (l:nat{l<8}). inv_l7_cross_chunkfact orig_re re u l))
        (ensures
          (forall (u:nat{u<32}). (u % 32 < 16) ==>
            unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 orig_re) u) (Seq.index (T.chunks_of_re_avx2 orig_re) (u+16))
              (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+16))
              (mk_i32 (zeta_r (1 - u/32)))) /\
          (forall (u:nat) (l:nat). u<32 /\ l<8 ==>
             Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l))))
  = (let aux (u:nat{u<32}) : Lemma
        ((u % 32 < 16) ==>
          unit_post_inv_cross_avx2 (Seq.index (T.chunks_of_re_avx2 orig_re) u) (Seq.index (T.chunks_of_re_avx2 orig_re) (u+16))
            (Seq.index (T.chunks_of_re_avx2 re) u) (Seq.index (T.chunks_of_re_avx2 re) (u+16))
            (mk_i32 (zeta_r (1 - u/32))))
      = if (u % 32 < 16) then inv_lemma_l7_cross_chunk_avx2 orig_re re bnd u
     in Classical.forall_intro aux);
    (let auxb (i:nat{i<16}) (l:nat{l<8}) : Lemma
        (let u = ((i / 16) * 32) + (i % 16) in
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l)) /\
         Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re (u+16)).f_value (mk_u64 l)))
      = let u = ((i / 16) * 32) + (i % 16) in
        FStar.Math.Lemmas.lemma_div_mod i 16;
        assert (u < 32 /\ u % 32 < 16);
        inv_lemma_l7_cross_chunk_avx2 orig_re re bnd u
     in Classical.forall_intro_2 auxb);
    introduce forall (u:nat) (l:nat). u<32 /\ l<8 ==>
        Spec.Utils.is_i32b (2*bnd) (to_i32x8 (Seq.index re u).f_value (mk_u64 l))
    with (if u < 32 && l < 8 then begin
            let base : nat = u / 32 in let r : nat = u % 32 in
            FStar.Math.Lemmas.lemma_div_mod u 32;
            let i : nat = base * 16 + (if r < 16 then r else r - 16) in
            FStar.Math.Lemmas.lemma_div_mod i 16;
            assert (i < 16);
            let ulo = ((i / 16) * 32) + (i % 16) in
            assert (ulo == u \/ ulo + 16 == u)
          end)
#pop-options

(* Driver compose: plain refined-forall of cross atom -> intt_layer flat congruence. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let inv_lemma_l7_cross_driver_compose_avx2
      (orig fut: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : Lemma
        (requires
          (forall (u:nat{u<32}). (u % 32 < 16) ==>
            unit_post_inv_cross_avx2 (Seq.index orig u) (Seq.index orig (u+16))
              (Seq.index fut u) (Seq.index fut (u+16))
              (mk_i32 (zeta_r (1 - u/32)))))
        (ensures
          (let in_flat = C.simd_units_to_array orig in
           let out_flat = C.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 7) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (zeta_r (1 - u/32)) in
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 32 < 16) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+16) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+16) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
          (v (Seq.index co_hi l)) % 8380417 ==
            ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm u) * 8265825) % 8380417))
      = if (u % 32 < 16) then
          inv_lemma_atom_to_bf_inv_cross_avx2 (Seq.index orig u) (Seq.index orig (u+16))
                                              (Seq.index fut u) (Seq.index fut (u+16)) (zm u)
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 32 < 16) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (1 - u/32) ] <: i32) * pow2 32) % 8380417)
      = if (u % 32 < 16) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = zeta_r (1 - u/32) in
          C.lemma_v_zetas_eq_zeta (1 - u/32)
        end
     in Classical.forall_intro aux_z);
    C.lemma_intt_layer_7_cross_to_hacspec_poly orig fut zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_inv_l7_full_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2 * bnd < pow2 31 /\ bnd >= 8380416})
    : Lemma
        (requires T.is_i32b_poly_avx2 bnd orig_re /\ invert_ntt_outer_3_plus_spec 7 orig_re re)
        (ensures
          T.is_i32b_poly_avx2 (2*bnd) re /\
          (let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 7) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = inv_l7_cross_chunkfacts_from_post orig_re re;
    inv_l7_cross_atoms_and_bounds orig_re re bnd;
    T.lemma_is_i32b_poly_avx2_intro (2*bnd) re;
    inv_lemma_l7_cross_driver_compose_avx2 (T.chunks_of_re_avx2 orig_re) (T.chunks_of_re_avx2 re)
#pop-options

(* ===== PHASE C: scaling fold + top compose =================================
   AVX2 analogues of the Portable scaling machinery.  The generic
   chunk-level helpers (chunk_scaled / lemma_establish_chunk_scaled /
   lemma_scale_chunk / lemma_scale_flat) live in Portable.Invntt over plain
   t_Array i32 8 / t_Array (t_Array i32 8) 32, so they are reusable verbatim;
   only the chunks_of_re-specific driver needs an AVX2 mirror.            ===== *)
module PI = Libcrux_ml_dsa.Simd.Portable.Invntt

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --z3refresh"
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

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
(* AVX2 mirror of Portable lemma_scale_driver: 32 per-chunk chunk_scaled atoms
   (loop-invariant output) -> flat 16382-scaling congruence.  Reuses the generic
   PI.lemma_scale_flat (chunks_of_re-agnostic) after revealing each chunk atom. *)
let lemma_scale_driver_avx2
      (orig_re fut_re : t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires (forall (b:nat). b < 32 ==>
         PI.chunk_scaled (Seq.index (T.chunks_of_re_avx2 orig_re) b)
                         (Seq.index (T.chunks_of_re_avx2 fut_re) b)))
      (ensures
        (let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 orig_re) in
         let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 fut_re) in
         forall (j:nat). j < 256 ==>
           (v (Seq.index out_flat j)) % 8380417 == (16382 * v (Seq.index in_flat j)) % 8380417))
  = let ci = T.chunks_of_re_avx2 orig_re in
    let co = T.chunks_of_re_avx2 fut_re in
    let aux (b:nat{b<32}) : Lemma
        (forall (l:nat). l < 8 ==>
          (v (Seq.index (Seq.index co b) l)) % 8380417 ==
          (16382 * v (Seq.index (Seq.index ci b) l)) % 8380417) =
      reveal_opaque (`%PI.chunk_scaled)
        (PI.chunk_scaled (Seq.index ci b) (Seq.index co b))
    in Classical.forall_intro aux;
    PI.lemma_scale_flat ci co
#pop-options

(* OPAQUE seal for one inverse layer's flat congruence: flat(fout) ≡ intt_layer(flat(fin),n).
   Sealing each layer's congruence right after its _full call keeps inv_inner's WP small
   (8 cheap atoms instead of 8 transparent 256-foralls — the "WALL 2" the forward NTT hit;
   mirrors Avx2NttTheory.layer_done for the forward direction). *)
[@@ "opaque_to_smt"]
let inv_layer_done (n:nat{n<8})
      (fin fout: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  let in_flat  = C.simd_units_to_array (T.chunks_of_re_avx2 fin) in
  let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 fout) in
  forall (i:nat). i < 256 ==>
    (v (Seq.index out_flat i)) % 8380417 ==
    (v (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize n)) i)) % 8380417

#push-options "--fuel 0 --ifuel 1 --z3rlimit 50 --z3refresh"
let lemma_inv_layer_done_intro (n:nat{n<8})
      (fin fout: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        (let in_flat  = C.simd_units_to_array (T.chunks_of_re_avx2 fin) in
         let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 fout) in
         forall (i:nat). i < 256 ==>
           (v (Seq.index out_flat i)) % 8380417 ==
           (v (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize n)) i)) % 8380417))
      (ensures inv_layer_done n fin fout)
  = reveal_opaque (`%inv_layer_done) (inv_layer_done n fin fout)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --z3refresh"
(* Compose the 8 sealed inv_layer_done atoms into flat(s8) ≡ intt_unscaled(flat(s0)).
   Reveals each atom internally, then chains via the Portable compose lemma. *)
let lemma_inv_compose_8_sealed
      (s0 s1 s2 s3 s4 s5 s6 s7 s8 : t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        inv_layer_done 0 s0 s1 /\ inv_layer_done 1 s1 s2 /\ inv_layer_done 2 s2 s3 /\
        inv_layer_done 3 s3 s4 /\ inv_layer_done 4 s4 s5 /\ inv_layer_done 5 s5 s6 /\
        inv_layer_done 6 s6 s7 /\ inv_layer_done 7 s7 s8)
      (ensures
        (let f0 = C.simd_units_to_array (T.chunks_of_re_avx2 s0) in
         let f8 = C.simd_units_to_array (T.chunks_of_re_avx2 s8) in
         forall (i:nat). i < 256 ==> (v (Seq.index f8 i)) % 8380417 == (v (Seq.index (PI.intt_unscaled f0) i)) % 8380417))
  = reveal_opaque (`%inv_layer_done) (inv_layer_done 0 s0 s1);
    reveal_opaque (`%inv_layer_done) (inv_layer_done 1 s1 s2);
    reveal_opaque (`%inv_layer_done) (inv_layer_done 2 s2 s3);
    reveal_opaque (`%inv_layer_done) (inv_layer_done 3 s3 s4);
    reveal_opaque (`%inv_layer_done) (inv_layer_done 4 s4 s5);
    reveal_opaque (`%inv_layer_done) (inv_layer_done 5 s5 s6);
    reveal_opaque (`%inv_layer_done) (inv_layer_done 6 s6 s7);
    reveal_opaque (`%inv_layer_done) (inv_layer_done 7 s7 s8);
    PI.lemma_intt_compose_8
      (C.simd_units_to_array (T.chunks_of_re_avx2 s0))
      (C.simd_units_to_array (T.chunks_of_re_avx2 s1))
      (C.simd_units_to_array (T.chunks_of_re_avx2 s2))
      (C.simd_units_to_array (T.chunks_of_re_avx2 s3))
      (C.simd_units_to_array (T.chunks_of_re_avx2 s4))
      (C.simd_units_to_array (T.chunks_of_re_avx2 s5))
      (C.simd_units_to_array (T.chunks_of_re_avx2 s6))
      (C.simd_units_to_array (T.chunks_of_re_avx2 s7))
      (C.simd_units_to_array (T.chunks_of_re_avx2 s8))
#pop-options

(* ---- Sealed per-layer wrappers: call the _full lemma then immediately seal the flat
   congruence into the inv_layer_done atom.  The ensures exposes ONLY the opaque bound +
   the opaque atom (NOT the transparent 256-forall), so inv_inner's WP carries 8 cheap
   atoms instead of 8 soup-forming foralls. ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --z3refresh"
let lemma_inv_l0_sealed (fin fout: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2*bnd<pow2 31 /\ bnd>=8380416}) : Lemma
  (requires T.is_i32b_poly_avx2 bnd fin /\ inv_l0_post_sym fin fout)
  (ensures T.is_i32b_poly_avx2 (2*bnd) fout /\ inv_layer_done 0 fin fout)
  = lemma_inv_l0_full_avx2 fin fout bnd; lemma_inv_layer_done_intro 0 fin fout

let lemma_inv_l1_sealed (fin fout: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2*bnd<pow2 31 /\ bnd>=8380416}) : Lemma
  (requires T.is_i32b_poly_avx2 bnd fin /\ inv_l1_post_sym fin fout)
  (ensures T.is_i32b_poly_avx2 (2*bnd) fout /\ inv_layer_done 1 fin fout)
  = lemma_inv_l1_full_avx2 fin fout bnd; lemma_inv_layer_done_intro 1 fin fout

let lemma_inv_l2_sealed (fin fout: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2*bnd<pow2 31 /\ bnd>=8380416}) : Lemma
  (requires T.is_i32b_poly_avx2 bnd fin /\ inv_l2_post_sym fin fout)
  (ensures T.is_i32b_poly_avx2 (2*bnd) fout /\ inv_layer_done 2 fin fout)
  = lemma_inv_l2_full_avx2 fin fout bnd; lemma_inv_layer_done_intro 2 fin fout

let lemma_inv_l3_sealed (fin fout: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2*bnd<pow2 31 /\ bnd>=8380416}) : Lemma
  (requires T.is_i32b_poly_avx2 bnd fin /\ invert_ntt_outer_3_plus_spec 3 fin fout)
  (ensures T.is_i32b_poly_avx2 (2*bnd) fout /\ inv_layer_done 3 fin fout)
  = lemma_inv_l3_full_avx2 fin fout bnd; lemma_inv_layer_done_intro 3 fin fout

let lemma_inv_l4_sealed (fin fout: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2*bnd<pow2 31 /\ bnd>=8380416}) : Lemma
  (requires T.is_i32b_poly_avx2 bnd fin /\ invert_ntt_outer_3_plus_spec 4 fin fout)
  (ensures T.is_i32b_poly_avx2 (2*bnd) fout /\ inv_layer_done 4 fin fout)
  = lemma_inv_l4_full_avx2 fin fout bnd; lemma_inv_layer_done_intro 4 fin fout

let lemma_inv_l5_sealed (fin fout: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2*bnd<pow2 31 /\ bnd>=8380416}) : Lemma
  (requires T.is_i32b_poly_avx2 bnd fin /\ invert_ntt_outer_3_plus_spec 5 fin fout)
  (ensures T.is_i32b_poly_avx2 (2*bnd) fout /\ inv_layer_done 5 fin fout)
  = lemma_inv_l5_full_avx2 fin fout bnd; lemma_inv_layer_done_intro 5 fin fout

let lemma_inv_l6_sealed (fin fout: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2*bnd<pow2 31 /\ bnd>=8380416}) : Lemma
  (requires T.is_i32b_poly_avx2 bnd fin /\ invert_ntt_outer_3_plus_spec 6 fin fout)
  (ensures T.is_i32b_poly_avx2 (2*bnd) fout /\ inv_layer_done 6 fin fout)
  = lemma_inv_l6_full_avx2 fin fout bnd; lemma_inv_layer_done_intro 6 fin fout

let lemma_inv_l7_sealed (fin fout: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{2*bnd<pow2 31 /\ bnd>=8380416}) : Lemma
  (requires T.is_i32b_poly_avx2 bnd fin /\ invert_ntt_outer_3_plus_spec 7 fin fout)
  (ensures T.is_i32b_poly_avx2 (2*bnd) fout /\ inv_layer_done 7 fin fout)
  = lemma_inv_l7_full_avx2 fin fout bnd; lemma_inv_layer_done_intro 7 fin fout
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --z3refresh"
(* TIGHT per-lane bound for the final scale-back multiply.  The inverse-NTT
   layers leave each lane bounded by 256*FIELD_MAX; the `montgomery_multiply_by_constant(_, 41978)`
   then reduces it to the centered bound 4211177 = q/2 + ceil(256*FIELD_MAX*41978/2^32)
   via `Spec.MLDSA.Math.lemma_mont_red_bound_256_field_max_times_41978`. *)
let lemma_mont_mul_tight_bound_256 (x c: i32)
    : Lemma
        (requires Spec.Utils.is_i32b (256 * 8380416) x /\ v c == 41978)
        (ensures Spec.Utils.is_i32b 4211177 (mont_mul x c))
  = Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i64_inttype;
    Spec.Intrinsics.reveal_opaque_cast_ops #i32_inttype #i64_inttype;
    reveal_opaque (`%i32_mul) (i32_mul);
    let prod : int = v x * v c in
    assert_norm ((256 * 8380416) * 41978 < pow2 63);
    Spec.Utils.lemma_range_at_percent (v x) (pow2 64);
    Spec.Utils.lemma_range_at_percent (v c) (pow2 64);
    let cast_x : i64 = cast x <: i64 in
    let cast_y : i64 = cast c <: i64 in
    assert (v cast_x == v x /\ v cast_y == v c);
    let value : i64 = i32_mul x c in
    Spec.Utils.lemma_range_at_percent prod (pow2 64);
    assert (v value == prod);
    FStar.Math.Lemmas.lemma_abs_mul (v x) (v c);
    assert (Spec.Utils.is_i64b (256 * 8380416 * 41978) value);
    lemma_mont_red_bound_256_field_max_times_41978 value
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
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

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
(* After the scaling fold: lift the 32 per-chunk chunk_scaled atoms + per-lane bounds
   to the flat 16382-scaling congruence and the FM poly bound. *)
let lemma_inv_scale_finalize
      (s8 re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        (forall (k:nat). k < 32 ==>
           (forall (l:nat). l < 8 ==>
              Spec.Utils.is_i32b 4211177 (to_i32x8 (Seq.index re k).f_value (mk_u64 l))) /\
           PI.chunk_scaled (Seq.index (T.chunks_of_re_avx2 s8) k)
                           (Seq.index (T.chunks_of_re_avx2 re) k)))
      (ensures
        T.is_i32b_poly_avx2 4211177 re /\
        (let in_flat = C.simd_units_to_array (T.chunks_of_re_avx2 s8) in
         let out_flat = C.simd_units_to_array (T.chunks_of_re_avx2 re) in
         forall (j:nat). j < 256 ==>
           (v (Seq.index out_flat j)) % 8380417 == (16382 * v (Seq.index in_flat j)) % 8380417))
  = T.lemma_is_i32b_poly_avx2_intro 4211177 re;
    lemma_scale_driver_avx2 s8 re
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --z3refresh"
(* Frame: if re_new[k] == re_old[k] then their chunk views agree at k.  Proves the
   createi sub-array equality by extensionality + the lane index lemma — in CLEAN
   context, so the scaling fold's invariant maintenance never pays the createi cascade. *)
let lemma_cre_frame
      (re_old re_new: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (k:nat{k<32})
    : Lemma (requires Seq.index re_new k == Seq.index re_old k)
            (ensures Seq.index (T.chunks_of_re_avx2 re_new) k == Seq.index (T.chunks_of_re_avx2 re_old) k)
  = let co = Seq.index (T.chunks_of_re_avx2 re_old) k in
    let cn = Seq.index (T.chunks_of_re_avx2 re_new) k in
    let auxl (l:nat{l<8}) : Lemma (Seq.index cn l == Seq.index co l) =
      T.lemma_chunks_of_re_avx2_index re_old k l;
      T.lemma_chunks_of_re_avx2_index re_new k l
    in Classical.forall_intro auxl;
    Seq.lemma_eq_intro cn co
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
(* Standalone clean-context carryover: extend the scaling-fold invariant from index i
   to i+1 after the per-unit update at i.  Mirrors the Portable
   lemma_is_bounded_poly_range_extend_after_update pattern; isolates the frame so the
   fold body's WP stays tiny (the inline version saturated at rlimit 400). *)
let lemma_inv_scale_carryover
      (s8 re_old re_new: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:usize{v i < 32})
    : Lemma
      (requires
        (forall (k:nat). k < v i ==>
           (forall (l:nat). l < 8 ==>
              Spec.Utils.is_i32b 4211177 (to_i32x8 (Seq.index re_old k).f_value (mk_u64 l))) /\
           PI.chunk_scaled (Seq.index (T.chunks_of_re_avx2 s8) k) (Seq.index (T.chunks_of_re_avx2 re_old) k)) /\
        (forall (k:nat). (k >= v i /\ k < 32) ==> Seq.index re_old k == Seq.index s8 k) /\
        (forall (k:nat). k < 32 /\ k <> v i ==> Seq.index re_new k == Seq.index re_old k) /\
        (forall (l:nat). l < 8 ==>
           Spec.Utils.is_i32b 4211177 (to_i32x8 (Seq.index re_new (v i)).f_value (mk_u64 l))) /\
        PI.chunk_scaled (Seq.index (T.chunks_of_re_avx2 s8) (v i)) (Seq.index (T.chunks_of_re_avx2 re_new) (v i)))
      (ensures
        (forall (k:nat). k < v i + 1 ==>
           (forall (l:nat). l < 8 ==>
              Spec.Utils.is_i32b 4211177 (to_i32x8 (Seq.index re_new k).f_value (mk_u64 l))) /\
           PI.chunk_scaled (Seq.index (T.chunks_of_re_avx2 s8) k) (Seq.index (T.chunks_of_re_avx2 re_new) k)) /\
        (forall (k:nat). (k >= v i + 1 /\ k < 32) ==> Seq.index re_new k == Seq.index s8 k))
  = let aux_lo (k:nat{k < v i + 1}) : Lemma
        ((forall (l:nat). l < 8 ==>
            Spec.Utils.is_i32b 8380416 (to_i32x8 (Seq.index re_new k).f_value (mk_u64 l))) /\
         PI.chunk_scaled (Seq.index (T.chunks_of_re_avx2 s8) k) (Seq.index (T.chunks_of_re_avx2 re_new) k)) =
      if k < v i then lemma_cre_frame re_old re_new k
      else ()
    in Classical.forall_intro aux_lo;
    let aux_hi (k:nat{k >= v i + 1 /\ k < 32}) : Lemma (Seq.index re_new k == Seq.index s8 k) = ()
    in Classical.forall_intro aux_hi
#pop-options

"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --z3refresh")]
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
        hax_lib::loop_invariant!(|i: usize| fstar!(r#"
T.is_i32b_poly_avx2 (256 * 8380416) s8 /\
(forall (k:nat). k < v $i ==>
   (forall (l:nat). l < 8 ==> Spec.Utils.is_i32b 4211177 (to_i32x8 (Seq.index ${re} k).f_value (mk_u64 l))) /\
   PI.chunk_scaled (Seq.index (T.chunks_of_re_avx2 s8) k) (Seq.index (T.chunks_of_re_avx2 ${re}) k)) /\
(forall (k:nat). (k >= v $i /\ k < 32) ==> (Seq.index ${re} k) == (Seq.index s8 k))
"#));
        #[cfg(hax)]
        let re_old = re.clone();
        #[cfg(hax)]
        let orig_unit = re[i];
        re[i].value = arithmetic::montgomery_multiply_by_constant(re[i].value, FACTOR);
        hax_lib::fstar!(r#"lemma_inv_scale_step s8 re i orig_unit; lemma_inv_scale_carryover s8 re_old re i"#);
    }
    hax_lib::fstar!(r#"lemma_inv_scale_finalize s8 re"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"
unfold let inv_l0_post_lit (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  norm [
      primops; iota;
      delta_namespace [`%zeta_r; `%Spec.Utils.forall4; `%Spec.Utils.forall16]
    ]
    (Spec.Utils.forall16 (fun i ->
          let nre = re_fut in
          let re0 = Seq.index re (i * 2) in
          let re1 = Seq.index re (i * 2 + 1) in
          let nre0 = Seq.index nre (i * 2) in
          let nre1 = Seq.index nre (i * 2 + 1) in
          Spec.Utils.forall4 (fun j ->
                let zeta0 = zeta_r (255 - (i * 8 + j)) in
                let zeta1 = zeta_r (255 - (i * 8 + j + 4)) in
                let j0 = j * 2 in
                let j1 = j0 + 1 in
                (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
                inv_ntt_step (mk_int zeta0)
                  (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
                (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
                inv_ntt_step (mk_int zeta1)
                  (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))))

unfold let inv_l1_post_lit (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  norm [
      primops; iota;
      delta_namespace [`%zeta_r; `%Spec.Utils.forall4; `%Spec.Utils.forall16]
    ]
    (Spec.Utils.forall16 (fun i ->
          let nre = re_fut in
          let re0 = Seq.index re (i * 2) in
          let re1 = Seq.index re (i * 2 + 1) in
          let nre0 = Seq.index nre (i * 2) in
          let nre1 = Seq.index nre (i * 2 + 1) in
          Spec.Utils.forall4 (fun j ->
                let zeta0 = zeta_r (127 - (i * 4 + j / 2)) in
                let zeta1 = zeta_r (127 - (i * 4 + j / 2 + 2)) in
                let j0 = (match j with | 0 -> 0 | 1 -> 1 | 2 -> 4 | 3 -> 5) in
                let j1 = j0 + 2 in
                (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
                inv_ntt_step (mk_int zeta0)
                  (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
                (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
                inv_ntt_step (mk_int zeta1)
                  (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))))

unfold let inv_l2_post_lit (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  norm [
      primops; iota;
      delta_namespace [`%zeta_r; `%Spec.Utils.forall4; `%Spec.Utils.forall16]
    ]
    (Spec.Utils.forall16 (fun i ->
          let nre = re_fut in
          let re0 = Seq.index re (i * 2) in
          let re1 = Seq.index re (i * 2 + 1) in
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
                  (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))))

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --z3refresh"
let lemma_inv_l0_post_to_sym (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires inv_l0_post_lit re re_fut) (ensures inv_l0_post_sym re re_fut)
  =
    assert_norm (zeta_r 128 == 2091667);
    assert_norm (zeta_r 129 == 3407706);
    assert_norm (zeta_r 130 == 2316500);
    assert_norm (zeta_r 131 == 3817976);
    assert_norm (zeta_r 132 == (- 3342478));
    assert_norm (zeta_r 133 == 2244091);
    assert_norm (zeta_r 134 == (- 2446433));
    assert_norm (zeta_r 135 == (- 3562462));
    assert_norm (zeta_r 136 == 266997);
    assert_norm (zeta_r 137 == 2434439);
    assert_norm (zeta_r 138 == (- 1235728));
    assert_norm (zeta_r 139 == 3513181);
    assert_norm (zeta_r 140 == (- 3520352));
    assert_norm (zeta_r 141 == (- 3759364));
    assert_norm (zeta_r 142 == (- 1197226));
    assert_norm (zeta_r 143 == (- 3193378));
    assert_norm (zeta_r 144 == 900702);
    assert_norm (zeta_r 145 == 1859098);
    assert_norm (zeta_r 146 == 909542);
    assert_norm (zeta_r 147 == 819034);
    assert_norm (zeta_r 148 == 495491);
    assert_norm (zeta_r 149 == (- 1613174));
    assert_norm (zeta_r 150 == (- 43260));
    assert_norm (zeta_r 151 == (- 522500));
    assert_norm (zeta_r 152 == (- 655327));
    assert_norm (zeta_r 153 == (- 3122442));
    assert_norm (zeta_r 154 == 2031748);
    assert_norm (zeta_r 155 == 3207046);
    assert_norm (zeta_r 156 == (- 3556995));
    assert_norm (zeta_r 157 == (- 525098));
    assert_norm (zeta_r 158 == (- 768622));
    assert_norm (zeta_r 159 == (- 3595838));
    assert_norm (zeta_r 160 == 342297);
    assert_norm (zeta_r 161 == 286988);
    assert_norm (zeta_r 162 == (- 2437823));
    assert_norm (zeta_r 163 == 4108315);
    assert_norm (zeta_r 164 == 3437287);
    assert_norm (zeta_r 165 == (- 3342277));
    assert_norm (zeta_r 166 == 1735879);
    assert_norm (zeta_r 167 == 203044);
    assert_norm (zeta_r 168 == 2842341);
    assert_norm (zeta_r 169 == 2691481);
    assert_norm (zeta_r 170 == (- 2590150));
    assert_norm (zeta_r 171 == 1265009);
    assert_norm (zeta_r 172 == 4055324);
    assert_norm (zeta_r 173 == 1247620);
    assert_norm (zeta_r 174 == 2486353);
    assert_norm (zeta_r 175 == 1595974);
    assert_norm (zeta_r 176 == (- 3767016));
    assert_norm (zeta_r 177 == 1250494);
    assert_norm (zeta_r 178 == 2635921);
    assert_norm (zeta_r 179 == (- 3548272));
    assert_norm (zeta_r 180 == (- 2994039));
    assert_norm (zeta_r 181 == 1869119);
    assert_norm (zeta_r 182 == 1903435);
    assert_norm (zeta_r 183 == (- 1050970));
    assert_norm (zeta_r 184 == (- 1333058));
    assert_norm (zeta_r 185 == 1237275);
    assert_norm (zeta_r 186 == (- 3318210));
    assert_norm (zeta_r 187 == (- 1430225));
    assert_norm (zeta_r 188 == (- 451100));
    assert_norm (zeta_r 189 == 1312455);
    assert_norm (zeta_r 190 == 3306115);
    assert_norm (zeta_r 191 == (- 1962642));
    assert_norm (zeta_r 192 == (- 1279661));
    assert_norm (zeta_r 193 == 1917081);
    assert_norm (zeta_r 194 == (- 2546312));
    assert_norm (zeta_r 195 == (- 1374803));
    assert_norm (zeta_r 196 == 1500165);
    assert_norm (zeta_r 197 == 777191);
    assert_norm (zeta_r 198 == 2235880);
    assert_norm (zeta_r 199 == 3406031);
    assert_norm (zeta_r 200 == (- 542412));
    assert_norm (zeta_r 201 == (- 2831860));
    assert_norm (zeta_r 202 == (- 1671176));
    assert_norm (zeta_r 203 == (- 1846953));
    assert_norm (zeta_r 204 == (- 2584293));
    assert_norm (zeta_r 205 == (- 3724270));
    assert_norm (zeta_r 206 == 594136);
    assert_norm (zeta_r 207 == (- 3776993));
    assert_norm (zeta_r 208 == (- 2013608));
    assert_norm (zeta_r 209 == 2432395);
    assert_norm (zeta_r 210 == 2454455);
    assert_norm (zeta_r 211 == (- 164721));
    assert_norm (zeta_r 212 == 1957272);
    assert_norm (zeta_r 213 == 3369112);
    assert_norm (zeta_r 214 == 185531);
    assert_norm (zeta_r 215 == (- 1207385));
    assert_norm (zeta_r 216 == (- 3183426));
    assert_norm (zeta_r 217 == 162844);
    assert_norm (zeta_r 218 == 1616392);
    assert_norm (zeta_r 219 == 3014001);
    assert_norm (zeta_r 220 == 810149);
    assert_norm (zeta_r 221 == 1652634);
    assert_norm (zeta_r 222 == (- 3694233));
    assert_norm (zeta_r 223 == (- 1799107));
    assert_norm (zeta_r 224 == (- 3038916));
    assert_norm (zeta_r 225 == 3523897);
    assert_norm (zeta_r 226 == 3866901);
    assert_norm (zeta_r 227 == 269760);
    assert_norm (zeta_r 228 == 2213111);
    assert_norm (zeta_r 229 == (- 975884));
    assert_norm (zeta_r 230 == 1717735);
    assert_norm (zeta_r 231 == 472078);
    assert_norm (zeta_r 232 == (- 426683));
    assert_norm (zeta_r 233 == 1723600);
    assert_norm (zeta_r 234 == (- 1803090));
    assert_norm (zeta_r 235 == 1910376);
    assert_norm (zeta_r 236 == (- 1667432));
    assert_norm (zeta_r 237 == (- 1104333));
    assert_norm (zeta_r 238 == (- 260646));
    assert_norm (zeta_r 239 == (- 3833893));
    assert_norm (zeta_r 240 == (- 2939036));
    assert_norm (zeta_r 241 == (- 2235985));
    assert_norm (zeta_r 242 == (- 420899));
    assert_norm (zeta_r 243 == (- 2286327));
    assert_norm (zeta_r 244 == 183443);
    assert_norm (zeta_r 245 == (- 976891));
    assert_norm (zeta_r 246 == 1612842);
    assert_norm (zeta_r 247 == (- 3545687));
    assert_norm (zeta_r 248 == (- 554416));
    assert_norm (zeta_r 249 == 3919660);
    assert_norm (zeta_r 250 == (- 48306));
    assert_norm (zeta_r 251 == (- 1362209));
    assert_norm (zeta_r 252 == 3937738);
    assert_norm (zeta_r 253 == 1400424);
    assert_norm (zeta_r 254 == (- 846154));
    assert_norm (zeta_r 255 == 1976782)

let lemma_inv_l1_post_to_sym (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires inv_l1_post_lit re re_fut) (ensures inv_l1_post_sym re re_fut)
  =
    assert_norm (zeta_r 64 == (- 3930395));
    assert_norm (zeta_r 65 == (- 1528703));
    assert_norm (zeta_r 66 == (- 3677745));
    assert_norm (zeta_r 67 == (- 3041255));
    assert_norm (zeta_r 68 == (- 1452451));
    assert_norm (zeta_r 69 == 3475950);
    assert_norm (zeta_r 70 == 2176455);
    assert_norm (zeta_r 71 == (- 1585221));
    assert_norm (zeta_r 72 == (- 1257611));
    assert_norm (zeta_r 73 == 1939314);
    assert_norm (zeta_r 74 == (- 4083598));
    assert_norm (zeta_r 75 == (- 1000202));
    assert_norm (zeta_r 76 == (- 3190144));
    assert_norm (zeta_r 77 == (- 3157330));
    assert_norm (zeta_r 78 == (- 3632928));
    assert_norm (zeta_r 79 == 126922);
    assert_norm (zeta_r 80 == 3412210);
    assert_norm (zeta_r 81 == (- 983419));
    assert_norm (zeta_r 82 == 2147896);
    assert_norm (zeta_r 83 == 2715295);
    assert_norm (zeta_r 84 == (- 2967645));
    assert_norm (zeta_r 85 == (- 3693493));
    assert_norm (zeta_r 86 == (- 411027));
    assert_norm (zeta_r 87 == (- 2477047));
    assert_norm (zeta_r 88 == (- 671102));
    assert_norm (zeta_r 89 == (- 1228525));
    assert_norm (zeta_r 90 == (- 22981));
    assert_norm (zeta_r 91 == (- 1308169));
    assert_norm (zeta_r 92 == (- 381987));
    assert_norm (zeta_r 93 == 1349076);
    assert_norm (zeta_r 94 == 1852771);
    assert_norm (zeta_r 95 == (- 1430430));
    assert_norm (zeta_r 96 == (- 3343383));
    assert_norm (zeta_r 97 == 264944);
    assert_norm (zeta_r 98 == 508951);
    assert_norm (zeta_r 99 == 3097992);
    assert_norm (zeta_r 100 == 44288);
    assert_norm (zeta_r 101 == (- 1100098));
    assert_norm (zeta_r 102 == 904516);
    assert_norm (zeta_r 103 == 3958618);
    assert_norm (zeta_r 104 == (- 3724342));
    assert_norm (zeta_r 105 == (- 8578));
    assert_norm (zeta_r 106 == 1653064);
    assert_norm (zeta_r 107 == (- 3249728));
    assert_norm (zeta_r 108 == 2389356);
    assert_norm (zeta_r 109 == (- 210977));
    assert_norm (zeta_r 110 == 759969);
    assert_norm (zeta_r 111 == (- 1316856));
    assert_norm (zeta_r 112 == 189548);
    assert_norm (zeta_r 113 == (- 3553272));
    assert_norm (zeta_r 114 == 3159746);
    assert_norm (zeta_r 115 == (- 1851402));
    assert_norm (zeta_r 116 == (- 2409325));
    assert_norm (zeta_r 117 == (- 177440));
    assert_norm (zeta_r 118 == 1315589);
    assert_norm (zeta_r 119 == 1341330);
    assert_norm (zeta_r 120 == 1285669);
    assert_norm (zeta_r 121 == (- 1584928));
    assert_norm (zeta_r 122 == (- 812732));
    assert_norm (zeta_r 123 == (- 1439742));
    assert_norm (zeta_r 124 == (- 3019102));
    assert_norm (zeta_r 125 == (- 3881060));
    assert_norm (zeta_r 126 == (- 3628969));
    assert_norm (zeta_r 127 == 3839961)

let lemma_inv_l2_post_to_sym (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires inv_l2_post_lit re re_fut) (ensures inv_l2_post_sym re re_fut)
  =
    assert_norm (zeta_r 32 == 2706023);
    assert_norm (zeta_r 33 == 95776);
    assert_norm (zeta_r 34 == 3077325);
    assert_norm (zeta_r 35 == 3530437);
    assert_norm (zeta_r 36 == (- 1661693));
    assert_norm (zeta_r 37 == (- 3592148));
    assert_norm (zeta_r 38 == (- 2537516));
    assert_norm (zeta_r 39 == 3915439);
    assert_norm (zeta_r 40 == (- 3861115));
    assert_norm (zeta_r 41 == (- 3043716));
    assert_norm (zeta_r 42 == 3574422);
    assert_norm (zeta_r 43 == (- 2867647));
    assert_norm (zeta_r 44 == 3539968);
    assert_norm (zeta_r 45 == (- 300467));
    assert_norm (zeta_r 46 == 2348700);
    assert_norm (zeta_r 47 == (- 539299));
    assert_norm (zeta_r 48 == (- 1699267));
    assert_norm (zeta_r 49 == (- 1643818));
    assert_norm (zeta_r 50 == 3505694);
    assert_norm (zeta_r 51 == (- 3821735));
    assert_norm (zeta_r 52 == 3507263);
    assert_norm (zeta_r 53 == (- 2140649));
    assert_norm (zeta_r 54 == (- 1600420));
    assert_norm (zeta_r 55 == 3699596);
    assert_norm (zeta_r 56 == 811944);
    assert_norm (zeta_r 57 == 531354);
    assert_norm (zeta_r 58 == 954230);
    assert_norm (zeta_r 59 == 3881043);
    assert_norm (zeta_r 60 == 3900724);
    assert_norm (zeta_r 61 == (- 2556880));
    assert_norm (zeta_r 62 == 2071892);
    assert_norm (zeta_r 63 == (- 2797779))
#pop-options
"#)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 8380416 $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (2*8380416) ${re}_future /\ inv_layer_done 0 $re ${re}_future"#))]
unsafe fn run_inv_layer_0(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_0(re);
    hax_lib::fstar!(r#"lemma_inv_l0_post_to_sym orig re; lemma_inv_l0_sealed orig re 8380416"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (2*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (4*8380416) ${re}_future /\ inv_layer_done 1 $re ${re}_future"#))]
unsafe fn run_inv_layer_1(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_1(re);
    hax_lib::fstar!(r#"lemma_inv_l1_post_to_sym orig re; lemma_inv_l1_sealed orig re (2*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (4*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (8*8380416) ${re}_future /\ inv_layer_done 2 $re ${re}_future"#))]
unsafe fn run_inv_layer_2(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_2(re);
    hax_lib::fstar!(r#"lemma_inv_l2_post_to_sym orig re; lemma_inv_l2_sealed orig re (4*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (8*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (16*8380416) ${re}_future /\ inv_layer_done 3 $re ${re}_future"#))]
unsafe fn run_inv_layer_3(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_3(re);
    hax_lib::fstar!(r#"lemma_inv_l3_sealed orig re (8*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (16*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (32*8380416) ${re}_future /\ inv_layer_done 4 $re ${re}_future"#))]
unsafe fn run_inv_layer_4(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_4(re);
    hax_lib::fstar!(r#"lemma_inv_l4_sealed orig re (16*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (32*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (64*8380416) ${re}_future /\ inv_layer_done 5 $re ${re}_future"#))]
unsafe fn run_inv_layer_5(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_5(re);
    hax_lib::fstar!(r#"lemma_inv_l5_sealed orig re (32*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (64*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (128*8380416) ${re}_future /\ inv_layer_done 6 $re ${re}_future"#))]
unsafe fn run_inv_layer_6(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_6(re);
    hax_lib::fstar!(r#"lemma_inv_l6_sealed orig re (64*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::options("--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh")]
#[hax_lib::requires(fstar!(r#"T.is_i32b_poly_avx2 (128*8380416) $re"#))]
#[hax_lib::ensures(|result| fstar!(r#"T.is_i32b_poly_avx2 (256*8380416) ${re}_future /\ inv_layer_done 7 $re ${re}_future"#))]
unsafe fn run_inv_layer_7(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig = re.clone();
    invert_ntt_at_layer_7(re);
    hax_lib::fstar!(r#"lemma_inv_l7_sealed orig re (128*8380416)"#);
}

#[inline(always)]
#[allow(unsafe_code)]
#[hax_lib::fstar::options("--z3rlimit 100 --z3refresh")]
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
    hax_lib::fstar!(r#"lemma_inv_compose_8_sealed s0 s1 s2 s3 s4 s5 s6 s7 s8"#);
}

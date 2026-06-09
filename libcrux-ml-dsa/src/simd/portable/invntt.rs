use super::{
    arithmetic::{self, montgomery_multiply_fe_by_fer},
    vector_type::Coefficients,
};
#[cfg(hax)]
use crate::simd::traits::specs::*;
use crate::simd::traits::{COEFFICIENTS_IN_SIMD_UNIT, SIMD_UNITS_IN_RING_ELEMENT};

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::before(
    r#"
let simd_layer_factor (step:usize) =
    match step with
    | MkInt 1 -> 1
    | MkInt 2 -> 2
    | MkInt 4 -> 4
    | _ -> 5
"#
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    v $step <= 4 /\ v $index + v $step < 8 /\    
    Spec.Utils.is_i32b (simd_layer_factor $step * v $FIELD_MAX)
                    (Seq.index ${simd_unit}.f_values (v $index)) /\
    Spec.Utils.is_i32b (simd_layer_factor $step * v $FIELD_MAX)
                    (Seq.index ${simd_unit}.f_values (v $index + v $step)) /\
    Spec.Utils.is_i32b 4190208 $zeta 
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.modifies2_8 ${simd_unit}.f_values ${simd_unit}_future.f_values index (index +! step) /\
    Spec.Utils.is_i32b (2 * (simd_layer_factor $step)  * v $FIELD_MAX)
                    (Seq.index ${simd_unit}_future.f_values (v $index)) /\
    Spec.Utils.is_i32b (2 * (simd_layer_factor $step)  * v $FIELD_MAX)
                    (Seq.index ${simd_unit}_future.f_values (v $index + v $step))
"#) )]
fn simd_unit_inv_ntt_step(simd_unit: &mut Coefficients, zeta: i32, index: usize, step: usize) {
    let a_minus_b = simd_unit.values[index + step] - simd_unit.values[index];
    simd_unit.values[index] = simd_unit.values[index] + simd_unit.values[index + step];
    simd_unit.values[index + step] = montgomery_multiply_fe_by_fer(a_minus_b, zeta);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.is_i32b_array (v $FIELD_MAX) ${simd_unit}.f_values /\
    Spec.Utils.is_i32b 4190208 $zeta0 /\
    Spec.Utils.is_i32b 4190208 $zeta1 /\
    Spec.Utils.is_i32b 4190208 $zeta2 /\
    Spec.Utils.is_i32b 4190208 $zeta3
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_array (2 * v $FIELD_MAX) ${simd_unit}_future.f_values
"#) )]
pub fn simd_unit_invert_ntt_at_layer_0(
    simd_unit: &mut Coefficients,
    zeta0: i32,
    zeta1: i32,
    zeta2: i32,
    zeta3: i32,
) {
    simd_unit_inv_ntt_step(simd_unit, zeta0, 0, 1);
    simd_unit_inv_ntt_step(simd_unit, zeta1, 2, 1);
    simd_unit_inv_ntt_step(simd_unit, zeta2, 4, 1);
    simd_unit_inv_ntt_step(simd_unit, zeta3, 6, 1);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.is_i32b_array (2 * v $FIELD_MAX) ${simd_unit}.f_values /\
    Spec.Utils.is_i32b 4190208 $zeta0 /\
    Spec.Utils.is_i32b 4190208 $zeta1
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_array (4 * v $FIELD_MAX) ${simd_unit}_future.f_values
"#) )]
pub fn simd_unit_invert_ntt_at_layer_1(simd_unit: &mut Coefficients, zeta0: i32, zeta1: i32) {
    simd_unit_inv_ntt_step(simd_unit, zeta0, 0, 2);
    simd_unit_inv_ntt_step(simd_unit, zeta0, 1, 2);
    simd_unit_inv_ntt_step(simd_unit, zeta1, 4, 2);
    simd_unit_inv_ntt_step(simd_unit, zeta1, 5, 2);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.is_i32b_array (4 * v $FIELD_MAX) ${simd_unit}.f_values /\
    Spec.Utils.is_i32b 4190208 $zeta
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_array (8 * v $FIELD_MAX) ${simd_unit}_future.f_values
"#) )]
pub fn simd_unit_invert_ntt_at_layer_2(simd_unit: &mut Coefficients, zeta: i32) {
    simd_unit_inv_ntt_step(simd_unit, zeta, 0, 4);
    simd_unit_inv_ntt_step(simd_unit, zeta, 1, 4);
    simd_unit_inv_ntt_step(simd_unit, zeta, 2, 4);
    simd_unit_inv_ntt_step(simd_unit, zeta, 3, 4);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (2 * v $FIELD_MAX) ${re}_future
"#) )]
fn invert_ntt_at_layer_0(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[inline(always)]
    #[hax_lib::fstar::options("--z3rlimit 100")]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        v index < v $SIMD_UNITS_IN_RING_ELEMENT /\
        Spec.Utils.is_i32b_array_opaque (v $FIELD_MAX) 
            (Seq.index ${re} (v index)).f_values /\
        Spec.Utils.is_i32b 4190208 $zeta0 /\
        Spec.Utils.is_i32b 4190208 $zeta1 /\
        Spec.Utils.is_i32b 4190208 $zeta2 /\
        Spec.Utils.is_i32b 4190208 $zeta3
    "#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.modifies1_32 ${re} ${re}_future $index /\
        Spec.Utils.is_i32b_array_opaque (2* v $FIELD_MAX)
            (Seq.index ${re}_future (v index)).f_values
     "#))]
    fn round(
        re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT],
        index: usize,
        zeta0: i32,
        zeta1: i32,
        zeta2: i32,
        zeta3: i32,
    ) {
        hax_lib::fstar!(
            "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
        );
        simd_unit_invert_ntt_at_layer_0(&mut re[index], zeta0, zeta1, zeta2, zeta3);
    }

    round(re, 0, 1976782, -846154, 1400424, 3937738);
    round(re, 1, -1362209, -48306, 3919660, -554416);
    round(re, 2, -3545687, 1612842, -976891, 183443);
    round(re, 3, -2286327, -420899, -2235985, -2939036);
    round(re, 4, -3833893, -260646, -1104333, -1667432);
    round(re, 5, 1910376, -1803090, 1723600, -426683);
    round(re, 6, 472078, 1717735, -975884, 2213111);
    round(re, 7, 269760, 3866901, 3523897, -3038916);
    round(re, 8, -1799107, -3694233, 1652634, 810149);
    round(re, 9, 3014001, 1616392, 162844, -3183426);
    round(re, 10, -1207385, 185531, 3369112, 1957272);
    round(re, 11, -164721, 2454455, 2432395, -2013608);
    round(re, 12, -3776993, 594136, -3724270, -2584293);
    round(re, 13, -1846953, -1671176, -2831860, -542412);
    round(re, 14, 3406031, 2235880, 777191, 1500165);
    round(re, 15, -1374803, -2546312, 1917081, -1279661);
    round(re, 16, -1962642, 3306115, 1312455, -451100);
    round(re, 17, -1430225, -3318210, 1237275, -1333058);
    round(re, 18, -1050970, 1903435, 1869119, -2994039);
    round(re, 19, -3548272, 2635921, 1250494, -3767016);
    round(re, 20, 1595974, 2486353, 1247620, 4055324);
    round(re, 21, 1265009, -2590150, 2691481, 2842341);
    round(re, 22, 203044, 1735879, -3342277, 3437287);
    round(re, 23, 4108315, -2437823, 286988, 342297);
    round(re, 24, -3595838, -768622, -525098, -3556995);
    round(re, 25, 3207046, 2031748, -3122442, -655327);
    round(re, 26, -522500, -43260, -1613174, 495491);
    round(re, 27, 819034, 909542, 1859098, 900702);
    round(re, 28, -3193378, -1197226, -3759364, -3520352);
    round(re, 29, 3513181, -1235728, 2434439, 266997);
    round(re, 30, -3562462, -2446433, 2244091, -3342478);
    round(re, 31, 3817976, 2316500, 3407706, 2091667);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (2 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (4 * v $FIELD_MAX) ${re}_future
"#) )]
fn invert_ntt_at_layer_1(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[inline(always)]
    #[hax_lib::fstar::options("--z3rlimit 100")]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        v index < v $SIMD_UNITS_IN_RING_ELEMENT /\
        Spec.Utils.is_i32b_array_opaque (2 * v $FIELD_MAX) 
            (Seq.index ${re} (v index)).f_values /\
        Spec.Utils.is_i32b 4190208 $zeta_00 /\
        Spec.Utils.is_i32b 4190208 $zeta_01
    "#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.modifies1_32 ${re} ${re}_future $index /\
        Spec.Utils.is_i32b_array_opaque (4 * v $FIELD_MAX)
            (Seq.index ${re}_future (v $index)).f_values
     "#))]
    fn round(
        re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT],
        index: usize,
        zeta_00: i32,
        zeta_01: i32,
    ) {
        hax_lib::fstar!(
            "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
        );
        simd_unit_invert_ntt_at_layer_1(&mut re[index], zeta_00, zeta_01);
    }

    round(re, 0, 3839961, -3628969);
    round(re, 1, -3881060, -3019102);
    round(re, 2, -1439742, -812732);
    round(re, 3, -1584928, 1285669);
    round(re, 4, 1341330, 1315589);
    round(re, 5, -177440, -2409325);
    round(re, 6, -1851402, 3159746);
    round(re, 7, -3553272, 189548);
    round(re, 8, -1316856, 759969);
    round(re, 9, -210977, 2389356);
    round(re, 10, -3249728, 1653064);
    round(re, 11, -8578, -3724342);
    round(re, 12, 3958618, 904516);
    round(re, 13, -1100098, 44288);
    round(re, 14, 3097992, 508951);
    round(re, 15, 264944, -3343383);
    round(re, 16, -1430430, 1852771);
    round(re, 17, 1349076, -381987);
    round(re, 18, -1308169, -22981);
    round(re, 19, -1228525, -671102);
    round(re, 20, -2477047, -411027);
    round(re, 21, -3693493, -2967645);
    round(re, 22, 2715295, 2147896);
    round(re, 23, -983419, 3412210);
    round(re, 24, 126922, -3632928);
    round(re, 25, -3157330, -3190144);
    round(re, 26, -1000202, -4083598);
    round(re, 27, 1939314, -1257611);
    round(re, 28, -1585221, 2176455);
    round(re, 29, 3475950, -1452451);
    round(re, 30, -3041255, -3677745);
    round(re, 31, -1528703, -3930395);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (4 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (8 * v $FIELD_MAX) ${re}_future
"#) )]
fn invert_ntt_at_layer_2(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[inline(always)]
    #[hax_lib::fstar::options("--z3rlimit 100")]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        v index < v $SIMD_UNITS_IN_RING_ELEMENT /\
        Spec.Utils.is_i32b_array_opaque (4 * v $FIELD_MAX) 
            (Seq.index ${re} (v index)).f_values /\
        Spec.Utils.is_i32b 4190208 $zeta1
    "#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.modifies1_32 ${re} ${re}_future $index /\
        Spec.Utils.is_i32b_array_opaque (8 * v $FIELD_MAX)
            (Seq.index ${re}_future (v $index)).f_values
     "#))]
    fn round(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT], index: usize, zeta1: i32) {
        hax_lib::fstar!(
            "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
        );
        simd_unit_invert_ntt_at_layer_2(&mut re[index], zeta1);
    }

    round(re, 0, -2797779);
    round(re, 1, 2071892);
    round(re, 2, -2556880);
    round(re, 3, 3900724);
    round(re, 4, 3881043);
    round(re, 5, 954230);
    round(re, 6, 531354);
    round(re, 7, 811944);
    round(re, 8, 3699596);
    round(re, 9, -1600420);
    round(re, 10, -2140649);
    round(re, 11, 3507263);
    round(re, 12, -3821735);
    round(re, 13, 3505694);
    round(re, 14, -1643818);
    round(re, 15, -1699267);
    round(re, 16, -539299);
    round(re, 17, 2348700);
    round(re, 18, -300467);
    round(re, 19, 3539968);
    round(re, 20, -2867647);
    round(re, 21, 3574422);
    round(re, 22, -3043716);
    round(re, 23, -3861115);
    round(re, 24, 3915439);
    round(re, 25, -2537516);
    round(re, 26, -3592148);
    round(re, 27, -1661693);
    round(re, 28, 3530437);
    round(re, 29, 3077325);
    round(re, 30, 95776);
    round(re, 31, 2706023);
}

#[inline(always)]
#[hax_lib::fstar::before(
    r#"
let layer_bound_factor (step_by:usize) : n:nat{n <= 128} =
    match step_by with
    | MkInt 1 -> 8
    | MkInt 2 -> 16
    | MkInt 4 -> 32
    | MkInt 8 -> 64
    | MkInt 16 -> 128
    | _ -> 128"#
)]
#[hax_lib::fstar::options("--z3rlimit 600 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    (v $STEP_BY > 0) /\
    (v $OFFSET + v $STEP_BY < v $SIMD_UNITS_IN_RING_ELEMENT) /\
    (v $OFFSET + 2 * v $STEP_BY <= v $SIMD_UNITS_IN_RING_ELEMENT) /\
    (Spec.Utils.forall32 (fun i -> (i >= v $OFFSET /\ i < (v $OFFSET + 2 * v $STEP_BY)) ==>
              Spec.Utils.is_i32b_array_opaque 
                ((layer_bound_factor $STEP_BY) * v $FIELD_MAX)
                (Seq.index ${re} i).f_values)) /\
    Spec.Utils.is_i32b 4190208 $ZETA
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.modifies_range_32 ${re} ${re}_future $OFFSET (${OFFSET + STEP_BY + STEP_BY}) /\
    (Spec.Utils.forall32 (fun i -> (i >= v $OFFSET /\ i < (v $OFFSET + 2 * v $STEP_BY)) ==>
              Spec.Utils.is_i32b_array_opaque 
                (2 * (layer_bound_factor $STEP_BY) * v $FIELD_MAX)
                (Seq.index ${re}_future i).f_values))
"#))]
fn outer_3_plus<const OFFSET: usize, const STEP_BY: usize, const ZETA: i32>(
    re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT],
) {
    #[cfg(hax)]
    let orig_re = re.clone();

    for j in OFFSET..OFFSET + STEP_BY {
        hax_lib::loop_invariant!(|j: usize| fstar!(
            r#"
            (Spec.Utils.modifies_range2_32 $orig_re $re 
                $OFFSET $j ($OFFSET +! $STEP_BY) ($j +! $STEP_BY)) /\
            (Spec.Utils.forall32 (fun i -> ((i >= v $OFFSET /\ i < v $j) \/ 
                        (i >= v $OFFSET + v $STEP_BY /\ i < v $j + v $STEP_BY)) ==>
                Spec.Utils.is_i32b_array_opaque 
                    (2 * (layer_bound_factor $STEP_BY) * v $FIELD_MAX) 
                    (Seq.index ${re} i).f_values))
        "#
        ));

        let rej = re[j];
        let rejs = re[j + STEP_BY];
        arithmetic::add(&mut re[j], &rejs);
        arithmetic::subtract(&mut re[j + STEP_BY], &rej);
        arithmetic::montgomery_multiply_by_constant(&mut re[j + STEP_BY], ZETA);

        hax_lib::fstar!("Spec.Utils.is_i32b_array_larger 
            (v $FIELD_MAX) (2 * (layer_bound_factor $STEP_BY) * v $FIELD_MAX) (Seq.index re (v j + v v_STEP_BY)).f_values");
    }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (8 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (16 * v $FIELD_MAX) ${re}_future
"#) )]
fn invert_ntt_at_layer_3(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
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

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (16 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (32 * v $FIELD_MAX) ${re}_future
"#) )]
fn invert_ntt_at_layer_4(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
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

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (32 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (64 * v $FIELD_MAX) ${re}_future
"#) )]
fn invert_ntt_at_layer_5(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 32; // 1 << LAYER;
    const STEP_BY: usize = 4; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 466468>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -876248>(re);
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -777960>(re);
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 237124>(re);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (64 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (128 * v $FIELD_MAX) ${re}_future
"#) )]
fn invert_ntt_at_layer_6(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 64; // 1 << LAYER;
    const STEP_BY: usize = 8; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -518909>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2608894>(re);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (128 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (256 * v $FIELD_MAX) ${re}_future
"#) )]
fn invert_ntt_at_layer_7(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 128; // 1 << LAYER;
    const STEP_BY: usize = 16; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 25847>(re);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (v $FIELD_MAX) ${re}_future
"#) )]
pub(crate) fn invert_ntt_montgomery(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    invert_ntt_at_layer_0(re);
    invert_ntt_at_layer_1(re);
    invert_ntt_at_layer_2(re);
    invert_ntt_at_layer_3(re);
    invert_ntt_at_layer_4(re);
    invert_ntt_at_layer_5(re);
    invert_ntt_at_layer_6(re);
    invert_ntt_at_layer_7(re);

    for i in 0..re.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"
            (forall (k:nat).
              k < v $i ==>
              Spec.Utils.is_i32b_array_opaque (v $FIELD_MAX)
                (Seq.index $re k).f_values) /\
            (forall (k:nat).
              (k >= v $i /\ k < 32) ==>
              Spec.Utils.is_i32b_array_opaque (256 * v $FIELD_MAX)
                (Seq.index $re k).f_values))
        "#
        ));
        // After invert_ntt_at_layer, elements are of the form a * MONTGOMERY_R^{-1}
        // we multiply by (MONTGOMERY_R^2) * (1/2^8) mod Q = 41,978 to both:
        //
        // - Divide the elements by 256 and
        // - Convert the elements form montgomery domain to the standard domain.
        arithmetic::montgomery_multiply_by_constant(&mut re[i], 41_978);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        polynomial::PolynomialRingElement,
        simd::traits::{Operations, FIELD_MODULUS, SIMD_UNITS_IN_RING_ELEMENT},
    };

    fn poly_reduce_from_value<SIMDUnit: Operations>(
        value: i32,
        reduce: bool,
    ) -> PolynomialRingElement<SIMDUnit> {
        let mut re = PolynomialRingElement::<SIMDUnit>::zero();

        PolynomialRingElement::<SIMDUnit>::from_i32_array(
            &[value; SIMD_UNITS_IN_RING_ELEMENT * COEFFICIENTS_IN_SIMD_UNIT],
            &mut re,
        );

        if reduce {
            re.barrett_reduce();
        }

        let _ = core::hint::black_box(SIMDUnit::invert_ntt_montgomery(&mut re.simd_units));

        re
    }

    #[test]
    fn inv_ntt_unreduced_max() {
        let value = FIELD_MODULUS + (FIELD_MODULUS / 1024) + 6;
        let _re_portable =
            poly_reduce_from_value::<crate::simd::portable::PortableSIMDUnit>(value, true);
    }

    #[test]
    fn inv_ntt_reduced() {
        let value = FIELD_MODULUS + (FIELD_MODULUS / 1024) + 7;
        let _re_portable =
            poly_reduce_from_value::<crate::simd::portable::PortableSIMDUnit>(value, true);
    }

    #[test]
    #[should_panic]
    fn inv_ntt_reduced_panic() {
        let value = FIELD_MODULUS + (FIELD_MODULUS / 1024) + 7;
        // iNTT without reduction: In debug mode this will panic.
        let re1 = poly_reduce_from_value::<crate::simd::portable::PortableSIMDUnit>(value, false);

        let re2 = poly_reduce_from_value::<crate::simd::portable::PortableSIMDUnit>(value, true);

        // In release mode, one of the checks below will panic, since
        // the intermediate values silently overflowed, producing an
        // incorrect result.
        assert_eq!(re1.to_i32_array(), re2.to_i32_array());
    }

    #[test]
    fn inv_ntt_reduced_large() {
        let value = FIELD_MODULUS * 8;
        let _re_portable =
            poly_reduce_from_value::<crate::simd::portable::PortableSIMDUnit>(value, true);
    }

    #[test]
    #[should_panic]
    fn inv_ntt_reduced_large_panic() {
        let value = FIELD_MODULUS * 8;

        // iNTT without reduction: In debug mode this will panic.
        let re1 = poly_reduce_from_value::<crate::simd::portable::PortableSIMDUnit>(value, false);

        let re2 = poly_reduce_from_value::<crate::simd::portable::PortableSIMDUnit>(value, true);

        // In release mode, one of the checks below will panic, since
        // the intermediate values silently overflowed, producing an
        // incorrect result.
        assert_eq!(re1.to_i32_array(), re2.to_i32_array());
    }

    #[test]
    #[should_panic]
    fn inv_ntt_unreduced_panic() {
        let value = FIELD_MODULUS + (FIELD_MODULUS / 1024) + 7;

        // iNTT without reduction: In debug mode this will panic.
        let re1 = poly_reduce_from_value::<crate::simd::portable::PortableSIMDUnit>(value, false);

        let re2 = poly_reduce_from_value::<crate::simd::portable::PortableSIMDUnit>(value, true);

        // In release mode, one of the checks below will panic, since
        // the intermediate values silently overflowed, producing an
        // incorrect result.
        assert_eq!(re1.to_i32_array(), re2.to_i32_array());
    }
}

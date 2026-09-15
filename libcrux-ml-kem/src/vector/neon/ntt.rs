use super::arithmetic::*;
use super::vector_type::*;
use libcrux_intrinsics::arm64::*;

#[inline(always)]
#[hax_lib::fstar::before(
    interface,
    r#"unfold let repr = Libcrux_ml_kem.Vector.Neon.Vector_type.repr"#
)]
#[hax_lib::fstar::before(
    r#"
module NI = Libcrux_intrinsics.Arm64_ml_kem_views
module NS = Spec.Utils
module NA = Libcrux_ml_kem.Vector.Neon.Arithmetic
open Libcrux_ml_kem.Vector.Neon.Ntt_theory
"#
)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta1} /\ Spec.Utils.is_i16b 1664 ${zeta2} /\
    Spec.Utils.is_i16b 1664 ${zeta3} /\ Spec.Utils.is_i16b 1664 ${zeta4} /\
    Spec.Utils.is_i16b_array (7 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (8 * 3328) (repr ${result}) /\
    Spec.Utils.ntt_layer_1_butterfly_post (repr ${vec}) (repr ${result})
      ${zeta1} ${zeta2} ${zeta3} ${zeta4}"#))]
pub(crate) fn ntt_layer_1_step(
    vec: SIMD128Vector,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
    zeta4: i16,
) -> SIMD128Vector {
    // This is what we are trying to do, pointwise for every pair of elements:
    // let t = simd::Vector::montgomery_multiply_fe_by_fer(b, zeta_r);
    // b = simd::Vector::sub(a, &t);
    // a = simd::Vector::add(a, &t);

    let zetas = [zeta1, zeta1, zeta3, zeta3, zeta2, zeta2, zeta4, zeta4];
    let zeta = _vld1q_s16(&zetas);
    let dup_a = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(vec.low),
        _vreinterpretq_s32_s16(vec.high),
    ));
    let dup_b = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(vec.low),
        _vreinterpretq_s32_s16(vec.high),
    ));
    proof!(
        r#"assert (NI.get_lane_i16x8 ${zeta} 0 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 1 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 2 == ${zeta3} /\ NI.get_lane_i16x8 ${zeta} 3 == ${zeta3} /\
                NI.get_lane_i16x8 ${zeta} 4 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 5 == ${zeta2} /\
                NI.get_lane_i16x8 ${zeta} 6 == ${zeta4} /\ NI.get_lane_i16x8 ${zeta} 7 == ${zeta4});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i));
           lemma_trn1_s32_reinterpret ${vec}.f_low ${vec}.f_high;
           lemma_trn2_s32_reinterpret ${vec}.f_low ${vec}.f_high"#
    );
    let t = montgomery_multiply_int16x8_t(dup_b, zeta);
    let b = _vsubq_s16(dup_a, t);
    let a = _vaddq_s16(dup_a, t);

    let mut res = vec;
    res.low = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(a),
        _vreinterpretq_s32_s16(b),
    ));
    res.high = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(a),
        _vreinterpretq_s32_s16(b),
    ));
    proof!(
        r#"lemma_trn1_s32_reinterpret ${a} ${b};
           lemma_trn2_s32_reinterpret ${a} ${b};
           lemma_neon_fwd_l1_post (repr ${vec}) (repr ${res}) ${t} ${zeta1} ${zeta2} ${zeta3} ${zeta4}"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta1} /\ Spec.Utils.is_i16b 1664 ${zeta2} /\
    Spec.Utils.is_i16b_array (6 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (7 * 3328) (repr ${result}) /\
    Spec.Utils.ntt_layer_2_butterfly_post (repr ${vec}) (repr ${result}) ${zeta1} ${zeta2}"#))]
pub(crate) fn ntt_layer_2_step(vec: SIMD128Vector, zeta1: i16, zeta2: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    // let t = simd::Vector::montgomery_multiply_fe_by_fer(b, zeta_r);
    // b = simd::Vector::sub(a, &t);
    // a = simd::Vector::add(a, &t);

    let zetas = [zeta1, zeta1, zeta1, zeta1, zeta2, zeta2, zeta2, zeta2];
    let zeta = _vld1q_s16(&zetas);
    let dup_a = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(vec.low),
        _vreinterpretq_s64_s16(vec.high),
    ));
    let dup_b = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(vec.low),
        _vreinterpretq_s64_s16(vec.high),
    ));
    proof!(
        r#"assert (NI.get_lane_i16x8 ${zeta} 0 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 1 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 2 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 3 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 4 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 5 == ${zeta2} /\
                NI.get_lane_i16x8 ${zeta} 6 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 7 == ${zeta2});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i));
           lemma_trn1_s64_reinterpret ${vec}.f_low ${vec}.f_high;
           lemma_trn2_s64_reinterpret ${vec}.f_low ${vec}.f_high"#
    );
    let t = montgomery_multiply_int16x8_t(dup_b, zeta);
    let b = _vsubq_s16(dup_a, t);
    let a = _vaddq_s16(dup_a, t);

    let mut res = vec;
    res.low = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(a),
        _vreinterpretq_s64_s16(b),
    ));
    res.high = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(a),
        _vreinterpretq_s64_s16(b),
    ));
    proof!(
        r#"lemma_fwd_l2_resultv ${vec} ${res} ${dup_a} ${t} ${a} ${b};
           lemma_neon_fwd_l2_post (repr ${vec}) (repr ${res}) ${t} ${zeta1} ${zeta2}"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta_c} /\
    Spec.Utils.is_i16b_array (5 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (6 * 3328) (repr ${result}) /\
    Spec.Utils.ntt_layer_3_butterfly_post (repr ${vec}) (repr ${result}) ${zeta_c}"#))]
pub(crate) fn ntt_layer_3_step(vec: SIMD128Vector, zeta_c: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    // let t = simd::Vector::montgomery_multiply_fe_by_fer(b, zeta_r);
    // b = simd::Vector::sub(a, &t);
    // a = simd::Vector::add(a, &t);

    let zeta = _vdupq_n_s16(zeta_c);
    proof!(r#"assert (forall (i: nat{i < 8}). NI.get_lane_i16x8 ${zeta} i == ${zeta_c})"#);
    let t = montgomery_multiply_int16x8_t(vec.high, zeta);
    proof!(r#"assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i))"#);
    let mut res = vec;
    res.high = _vsubq_s16(vec.low, t);
    res.low = _vaddq_s16(res.low, t);
    proof!(
        r#"reveal_opaque (`%Spec.Utils.ntt_layer_3_butterfly_post) (Spec.Utils.ntt_layer_3_butterfly_post (repr ${vec}));
           lemma_modadd (v (Seq.index (repr ${vec}) 0)) (v (NI.get_lane_i16x8 ${t} 0)) (v (Seq.index (repr ${vec}) 8) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 0)) (v (NI.get_lane_i16x8 ${t} 0)) (v (Seq.index (repr ${vec}) 8) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 1)) (v (NI.get_lane_i16x8 ${t} 1)) (v (Seq.index (repr ${vec}) 9) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 1)) (v (NI.get_lane_i16x8 ${t} 1)) (v (Seq.index (repr ${vec}) 9) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 2)) (v (NI.get_lane_i16x8 ${t} 2)) (v (Seq.index (repr ${vec}) 10) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 2)) (v (NI.get_lane_i16x8 ${t} 2)) (v (Seq.index (repr ${vec}) 10) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 3)) (v (NI.get_lane_i16x8 ${t} 3)) (v (Seq.index (repr ${vec}) 11) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 3)) (v (NI.get_lane_i16x8 ${t} 3)) (v (Seq.index (repr ${vec}) 11) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 4)) (v (NI.get_lane_i16x8 ${t} 4)) (v (Seq.index (repr ${vec}) 12) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 4)) (v (NI.get_lane_i16x8 ${t} 4)) (v (Seq.index (repr ${vec}) 12) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 5)) (v (NI.get_lane_i16x8 ${t} 5)) (v (Seq.index (repr ${vec}) 13) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 5)) (v (NI.get_lane_i16x8 ${t} 5)) (v (Seq.index (repr ${vec}) 13) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 6)) (v (NI.get_lane_i16x8 ${t} 6)) (v (Seq.index (repr ${vec}) 14) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 6)) (v (NI.get_lane_i16x8 ${t} 6)) (v (Seq.index (repr ${vec}) 14) * v ${zeta_c} * 169);
           lemma_modadd (v (Seq.index (repr ${vec}) 7)) (v (NI.get_lane_i16x8 ${t} 7)) (v (Seq.index (repr ${vec}) 15) * v ${zeta_c} * 169);
           lemma_modsub (v (Seq.index (repr ${vec}) 7)) (v (NI.get_lane_i16x8 ${t} 7)) (v (Seq.index (repr ${vec}) 15) * v ${zeta_c} * 169);
           assert (Spec.Utils.is_i16b_array (6 * 3328) (repr ${res}))"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta1} /\ Spec.Utils.is_i16b 1664 ${zeta2} /\
    Spec.Utils.is_i16b 1664 ${zeta3} /\ Spec.Utils.is_i16b 1664 ${zeta4} /\
    Spec.Utils.is_i16b_array (4 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array 3328 (repr ${result}) /\
    Spec.Utils.inv_ntt_layer_1_butterfly_post (repr ${vec}) (repr ${result})
      ${zeta1} ${zeta2} ${zeta3} ${zeta4}"#))]
pub(crate) fn inv_ntt_layer_1_step(
    vec: SIMD128Vector,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
    zeta4: i16,
) -> SIMD128Vector {
    // This is what we are trying to do for every two elements:
    //let a_minus_b = simd::Vector::sub(b, &a);
    //a = simd::Vector::add(a, &b);
    //b = simd::Vector::montgomery_multiply_fe_by_fer(a_minus_b, zeta_r);
    //(a, b)

    let zetas = [zeta1, zeta1, zeta3, zeta3, zeta2, zeta2, zeta4, zeta4];
    let zeta = _vld1q_s16(&zetas);

    let aa = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(vec.low),
        _vreinterpretq_s32_s16(vec.high),
    ));
    let bb = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(vec.low),
        _vreinterpretq_s32_s16(vec.high),
    ));
    proof!(
        r#"assert (NI.get_lane_i16x8 ${zeta} 0 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 1 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 2 == ${zeta3} /\ NI.get_lane_i16x8 ${zeta} 3 == ${zeta3} /\
                NI.get_lane_i16x8 ${zeta} 4 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 5 == ${zeta2} /\
                NI.get_lane_i16x8 ${zeta} 6 == ${zeta4} /\ NI.get_lane_i16x8 ${zeta} 7 == ${zeta4});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i));
           lemma_trn1_s32_reinterpret ${vec}.f_low ${vec}.f_high;
           lemma_trn2_s32_reinterpret ${vec}.f_low ${vec}.f_high"#
    );

    let b_minus_a = _vsubq_s16(bb, aa);
    let asum_pre = _vaddq_s16(aa, bb);
    proof!(
        r#"assert (forall (i: nat{i < 8}). NS.is_i16b 28296 (NI.get_lane_i16x8 ${asum_pre} i))"#
    );
    let asum = barrett_reduce_int16x8_t(asum_pre);
    let bres = montgomery_multiply_int16x8_t(b_minus_a, zeta);

    let mut res = vec;
    res.low = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(asum),
        _vreinterpretq_s32_s16(bres),
    ));
    res.high = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(asum),
        _vreinterpretq_s32_s16(bres),
    ));
    proof!(
        r#"lemma_trn1_s32_reinterpret ${asum} ${bres};
           lemma_trn2_s32_reinterpret ${asum} ${bres};
           lemma_neon_inv_l1_post (repr ${vec}) (repr ${res}) ${asum} ${bres}
             ${zeta1} ${zeta2} ${zeta3} ${zeta4}"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta1} /\ Spec.Utils.is_i16b 1664 ${zeta2} /\
    Spec.Utils.is_i16b_array 3328 (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (2 * 3328) (repr ${result}) /\
    Spec.Utils.inv_ntt_layer_2_butterfly_post (repr ${vec}) (repr ${result}) ${zeta1} ${zeta2}"#))]
pub(crate) fn inv_ntt_layer_2_step(vec: SIMD128Vector, zeta1: i16, zeta2: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    //let a_minus_b = simd::Vector::sub(b, &a);
    //a = simd::Vector::add(a, &b);
    //b = simd::Vector::montgomery_multiply_fe_by_fer(a_minus_b, zeta_r);
    //(a, b)

    let zetas = [zeta1, zeta1, zeta1, zeta1, zeta2, zeta2, zeta2, zeta2];
    let zeta = _vld1q_s16(&zetas);

    let aa = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(vec.low),
        _vreinterpretq_s64_s16(vec.high),
    ));
    let bb = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(vec.low),
        _vreinterpretq_s64_s16(vec.high),
    ));
    proof!(
        r#"assert (NI.get_lane_i16x8 ${zeta} 0 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 1 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 2 == ${zeta1} /\ NI.get_lane_i16x8 ${zeta} 3 == ${zeta1} /\
                NI.get_lane_i16x8 ${zeta} 4 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 5 == ${zeta2} /\
                NI.get_lane_i16x8 ${zeta} 6 == ${zeta2} /\ NI.get_lane_i16x8 ${zeta} 7 == ${zeta2});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i));
           lemma_trn1_s64_reinterpret ${vec}.f_low ${vec}.f_high;
           lemma_trn2_s64_reinterpret ${vec}.f_low ${vec}.f_high"#
    );

    let b_minus_a = _vsubq_s16(bb, aa);
    let asum = _vaddq_s16(aa, bb);
    let bres = montgomery_multiply_int16x8_t(b_minus_a, zeta);

    let mut res = vec;
    res.low = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(asum),
        _vreinterpretq_s64_s16(bres),
    ));
    res.high = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(asum),
        _vreinterpretq_s64_s16(bres),
    ));
    proof!(
        r#"lemma_trn_s64_bound ${vec} ${aa} ${bb} 3328;
           lemma_vadd_bound ${aa} ${bb} ${asum} 3328;
           lemma_inv_l2_bdiff ${vec} ${aa} ${bb} ${b_minus_a};
           lemma_trn1_s64_reinterpret ${asum} ${bres};
           lemma_trn2_s64_reinterpret ${asum} ${bres};
           lemma_neon_inv_l2_post (repr ${vec}) (repr ${res}) ${asum} ${bres} ${zeta1} ${zeta2}"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta_c} /\
    Spec.Utils.is_i16b_array (2 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (4 * 3328) (repr ${result}) /\
    Spec.Utils.inv_ntt_layer_3_butterfly_post (repr ${vec}) (repr ${result}) ${zeta_c}"#))]
pub(crate) fn inv_ntt_layer_3_step(vec: SIMD128Vector, zeta_c: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    //let a_minus_b = simd::Vector::sub(b, &a);
    //a = simd::Vector::add(a, &b);
    //b = simd::Vector::montgomery_multiply_fe_by_fer(a_minus_b, zeta_r);
    //(a, b)

    let zeta = _vdupq_n_s16(zeta_c);
    proof!(
        r#"assert (forall (i: nat{i < 8}). NI.get_lane_i16x8 ${zeta} i == ${zeta_c});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i))"#
    );
    let b_minus_a = _vsubq_s16(vec.high, vec.low);
    let mut res = vec;
    res.low = _vaddq_s16(vec.low, vec.high);
    res.high = montgomery_multiply_int16x8_t(b_minus_a, zeta);
    proof!(
        r#"reveal_opaque (`%Spec.Utils.inv_ntt_layer_3_butterfly_post) (Spec.Utils.inv_ntt_layer_3_butterfly_post (repr ${vec}));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 0) == v (Seq.index (repr ${vec}) 8) - v (Seq.index (repr ${vec}) 0));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 1) == v (Seq.index (repr ${vec}) 9) - v (Seq.index (repr ${vec}) 1));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 2) == v (Seq.index (repr ${vec}) 10) - v (Seq.index (repr ${vec}) 2));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 3) == v (Seq.index (repr ${vec}) 11) - v (Seq.index (repr ${vec}) 3));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 4) == v (Seq.index (repr ${vec}) 12) - v (Seq.index (repr ${vec}) 4));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 5) == v (Seq.index (repr ${vec}) 13) - v (Seq.index (repr ${vec}) 5));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 6) == v (Seq.index (repr ${vec}) 14) - v (Seq.index (repr ${vec}) 6));
           assert (v (NI.get_lane_i16x8 ${b_minus_a} 7) == v (Seq.index (repr ${vec}) 15) - v (Seq.index (repr ${vec}) 7));
           assert (Spec.Utils.is_i16b_array (4 * 3328) (repr ${res}))"#
    );
    res
}

#[inline(always)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta1 /\ Spec.Utils.is_i16b 1664 zeta2 /\
                            Spec.Utils.is_i16b 1664 zeta3 /\ Spec.Utils.is_i16b 1664 zeta4 /\
                            Spec.Utils.is_i16b_array 4096 (repr ${lhs}) /\
                            Spec.Utils.is_i16b_array 4096 (repr ${rhs})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array 3328 (repr ${result}) /\
    Spec.Utils.ntt_multiply_butterfly_post (repr ${lhs}) (repr ${rhs}) (repr ${result})
      zeta1 zeta2 zeta3 zeta4"#))]
pub(crate) fn ntt_multiply(
    lhs: &SIMD128Vector,
    rhs: &SIMD128Vector,
    zeta1: i16,
    zeta2: i16,
    zeta3: i16,
    zeta4: i16,
) -> SIMD128Vector {
    // This is what we are trying to do for pairs of two elements:
    // montgomery_reduce(a0 * b0 + montgomery_reduce(a1 * b1) * zeta),
    // montgomery_reduce(a0 * b1 + a1 * b0)
    //let lhsp = crate::simd::portable::from_i16_array(to_i16_array(lhs.clone()));
    //let rhsp = crate::simd::portable::from_i16_array(to_i16_array(rhs.clone()));
    //let mulp = crate::simd::portable::ntt_multiply(&lhsp,&rhsp,zeta0,zeta1);
    //from_i16_array(crate::simd::portable::to_i16_array(mulp))

    let zetas: [i16; 8] = [zeta1, zeta3, -zeta1, -zeta3, zeta2, zeta4, -zeta2, -zeta4];
    let zeta = _vld1q_s16(&zetas);

    let a0 = _vtrn1q_s16(lhs.low, lhs.high); // a0, a8, a2, a10, ...
    let a1 = _vtrn2q_s16(lhs.low, lhs.high); // a1, a9, a3, a11, ...
    let b0 = _vtrn1q_s16(rhs.low, rhs.high); // b0, b8, b2, b10, ...
    let b1 = _vtrn2q_s16(rhs.low, rhs.high); // b1, b9, b3, b11, ...

    let a1b1 = montgomery_multiply_int16x8_t(a1, b1);
    let a1b1_low = _vmull_s16(_vget_low_s16(a1b1), _vget_low_s16(zeta)); // a1b1z, a9b9z, a3b3z, a11b11z
    let a1b1_high = _vmull_high_s16(a1b1, zeta); // a5b5z, a13b13z, a7b7z, a15b15z

    let fst_low =
        _vreinterpretq_s16_s32(_vmlal_s16(a1b1_low, _vget_low_s16(a0), _vget_low_s16(b0))); // 0, 8, 2, 10
    let fst_high = _vreinterpretq_s16_s32(_vmlal_high_s16(a1b1_high, a0, b0)); // 4, 12, 6, 14

    let a0b1_low = _vmull_s16(_vget_low_s16(a0), _vget_low_s16(b1));
    let a0b1_high = _vmull_high_s16(a0, b1);

    let snd_low =
        _vreinterpretq_s16_s32(_vmlal_s16(a0b1_low, _vget_low_s16(a1), _vget_low_s16(b0))); // 1, 9, 3, 11
    let snd_high = _vreinterpretq_s16_s32(_vmlal_high_s16(a0b1_high, a1, b0)); // 5, 13, 7, 15

    let fst_low16 = _vtrn1q_s16(fst_low, fst_high); // 0,4,8,12,2,6,10,14
    let fst_high16 = _vtrn2q_s16(fst_low, fst_high);
    let snd_low16 = _vtrn1q_s16(snd_low, snd_high); // 1,5,9,13,3,7,11,15
    let snd_high16 = _vtrn2q_s16(snd_low, snd_high);

    let fst = montgomery_reduce_int16x8_t(fst_low16, fst_high16); // 0,4,8,12,2,6,10,14
    let snd = montgomery_reduce_int16x8_t(snd_low16, snd_high16); // 1,5,9,13,3,7,11,15

    let low0 = _vreinterpretq_s32_s16(_vtrn1q_s16(fst, snd)); // 0,1,8,9,2,3,10,11
    let high0 = _vreinterpretq_s32_s16(_vtrn2q_s16(fst, snd)); // 4,5,12,13,6,7,14,15

    let low1 = _vreinterpretq_s16_s32(_vtrn1q_s32(low0, high0)); // 0,1,4,5,2,3,6,7
    let high1 = _vreinterpretq_s16_s32(_vtrn2q_s32(low0, high0)); // 8,9,12,13,10,11,14,15

    let indexes: [u8; 16] = [0, 1, 2, 3, 8, 9, 10, 11, 4, 5, 6, 7, 12, 13, 14, 15];
    let index = _vld1q_u8(&indexes);
    let low2 = _vreinterpretq_s16_u8(_vqtbl1q_u8(_vreinterpretq_u8_s16(low1), index));
    let high2 = _vreinterpretq_s16_u8(_vqtbl1q_u8(_vreinterpretq_u8_s16(high1), index));

    let res = SIMD128Vector {
        low: low2,
        high: high2,
    };
    proof!(
        r#"lemma_indexes_vals ${indexes};
    lemma_nttmul_index ${index} ${indexes};
    lemma_zetas_vals ${zetas} zeta1 zeta2 zeta3 zeta4;
    lemma_nttmul_zeta ${zeta} ${zetas} zeta1 zeta2 zeta3 zeta4;
    lemma_nttmul_compute ${lhs} ${rhs} ${zeta} ${index} zeta1 zeta2 zeta3 zeta4"#
    );
    res
}

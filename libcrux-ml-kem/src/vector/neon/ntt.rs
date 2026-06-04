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
module NI = Libcrux_intrinsics.Arm64_extract
module NS = Spec.Utils
module NA = Libcrux_ml_kem.Vector.Neon.Arithmetic

(* Mod-3329 congruence carries through the butterfly add/sub, exactly as the
   AVX2 ntt before-blocks (lemma_modadd). *)
let lemma_modadd (a r x:int) : Lemma
  (requires r % 3329 == x % 3329)
  (ensures (a + r) % 3329 == (a + x) % 3329)
  = FStar.Math.Lemmas.lemma_mod_add_distr a r 3329;
    FStar.Math.Lemmas.lemma_mod_add_distr a x 3329

let lemma_modsub (a r x:int) : Lemma
  (requires r % 3329 == x % 3329)
  (ensures (a - r) % 3329 == (a - x) % 3329)
  = FStar.Math.Lemmas.lemma_mod_sub_distr a r 3329;
    FStar.Math.Lemmas.lemma_mod_sub_distr a x 3329

(* Per-lane i16 add/sub are exact when the result is in range — the Neon
   analog of the AVX2 lemma_add_i_128 / lemma_sub_i_128 SMTPat lifters. *)
let lemma_neon_add_lane (lhs rhs: NI.t_e_int16x8_t) (i:nat{i < 8}) : Lemma
  (requires NS.is_intb (pow2 15 - 1)
              (v (NI.get_lane_i16x8 lhs i) + v (NI.get_lane_i16x8 rhs i)))
  (ensures v (NI.get_lane_i16x8 lhs i +. NI.get_lane_i16x8 rhs i) ==
           v (NI.get_lane_i16x8 lhs i) + v (NI.get_lane_i16x8 rhs i))
  [SMTPat (v (NI.get_lane_i16x8 lhs i +. NI.get_lane_i16x8 rhs i))]
  = ()

let lemma_neon_sub_lane (lhs rhs: NI.t_e_int16x8_t) (i:nat{i < 8}) : Lemma
  (requires NS.is_intb (pow2 15 - 1)
              (v (NI.get_lane_i16x8 lhs i) - v (NI.get_lane_i16x8 rhs i)))
  (ensures v (NI.get_lane_i16x8 lhs i -. NI.get_lane_i16x8 rhs i) ==
           v (NI.get_lane_i16x8 lhs i) - v (NI.get_lane_i16x8 rhs i))
  [SMTPat (v (NI.get_lane_i16x8 lhs i -. NI.get_lane_i16x8 rhs i))]
  = ()
"#
)]
pub(crate) fn ntt_layer_1_step(
    mut v: SIMD128Vector,
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
        _vreinterpretq_s32_s16(v.low),
        _vreinterpretq_s32_s16(v.high),
    ));
    let dup_b = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(v.low),
        _vreinterpretq_s32_s16(v.high),
    ));
    let t = montgomery_multiply_int16x8_t(dup_b, zeta);
    let b = _vsubq_s16(dup_a, t);
    let a = _vaddq_s16(dup_a, t);

    v.low = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(a),
        _vreinterpretq_s32_s16(b),
    ));
    v.high = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(a),
        _vreinterpretq_s32_s16(b),
    ));
    v
}

#[inline(always)]
pub(crate) fn ntt_layer_2_step(mut v: SIMD128Vector, zeta1: i16, zeta2: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    // let t = simd::Vector::montgomery_multiply_fe_by_fer(b, zeta_r);
    // b = simd::Vector::sub(a, &t);
    // a = simd::Vector::add(a, &t);

    let zetas = [zeta1, zeta1, zeta1, zeta1, zeta2, zeta2, zeta2, zeta2];
    let zeta = _vld1q_s16(&zetas);
    let dup_a = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(v.low),
        _vreinterpretq_s64_s16(v.high),
    ));
    let dup_b = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(v.low),
        _vreinterpretq_s64_s16(v.high),
    ));
    let t = montgomery_multiply_int16x8_t(dup_b, zeta);
    let b = _vsubq_s16(dup_a, t);
    let a = _vaddq_s16(dup_a, t);

    v.low = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(a),
        _vreinterpretq_s64_s16(b),
    ));
    v.high = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(a),
        _vreinterpretq_s64_s16(b),
    ));
    v
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta_c} /\
    Spec.Utils.is_i16b_array (5 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (6 * 3328) (repr ${result}) /\
    (forall (i: nat{i < 8}).
        (v (Seq.index (repr ${result}) i) % 3329 ==
          (v (Seq.index (repr ${vec}) i) +
            v (Seq.index (repr ${vec}) (i + 8)) * v ${zeta_c} * 169) % 3329) /\
        (v (Seq.index (repr ${result}) (i + 8)) % 3329 ==
          (v (Seq.index (repr ${vec}) i) -
            v (Seq.index (repr ${vec}) (i + 8)) * v ${zeta_c} * 169) % 3329))"#))]
pub(crate) fn ntt_layer_3_step(vec: SIMD128Vector, zeta_c: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    // let t = simd::Vector::montgomery_multiply_fe_by_fer(b, zeta_r);
    // b = simd::Vector::sub(a, &t);
    // a = simd::Vector::add(a, &t);

    let zeta = _vdupq_n_s16(zeta_c);
    hax_lib::fstar!(r#"assert (forall (i: nat{i < 8}). NI.get_lane_i16x8 ${zeta} i == ${zeta_c})"#);
    let t = montgomery_multiply_int16x8_t(vec.high, zeta);
    hax_lib::fstar!(r#"assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i))"#);
    let mut res = vec;
    res.high = _vsubq_s16(vec.low, t);
    res.low = _vaddq_s16(res.low, t);
    hax_lib::fstar!(
        r#"introduce forall (i: nat{i < 8}).
      (v (Seq.index (repr ${res}) i) % 3329 ==
        (v (Seq.index (repr ${vec}) i) +
          v (Seq.index (repr ${vec}) (i + 8)) * v ${zeta_c} * 169) % 3329) /\
      (v (Seq.index (repr ${res}) (i + 8)) % 3329 ==
        (v (Seq.index (repr ${vec}) i) -
          v (Seq.index (repr ${vec}) (i + 8)) * v ${zeta_c} * 169) % 3329)
    with (lemma_modadd (v (Seq.index (repr ${vec}) i)) (v (NI.get_lane_i16x8 ${t} i))
            (v (Seq.index (repr ${vec}) (i + 8)) * v ${zeta_c} * 169);
          lemma_modsub (v (Seq.index (repr ${vec}) i)) (v (NI.get_lane_i16x8 ${t} i))
            (v (Seq.index (repr ${vec}) (i + 8)) * v ${zeta_c} * 169));
        assert (Spec.Utils.is_i16b_array (6 * 3328) (repr ${res}))"#
    );
    res
}

#[inline(always)]
pub(crate) fn inv_ntt_layer_1_step(
    mut v: SIMD128Vector,
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

    let a = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(v.low),
        _vreinterpretq_s32_s16(v.high),
    ));
    let b = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(v.low),
        _vreinterpretq_s32_s16(v.high),
    ));

    let b_minus_a = _vsubq_s16(b, a);
    let a = _vaddq_s16(a, b);
    let a = barrett_reduce_int16x8_t(a);
    let b = montgomery_multiply_int16x8_t(b_minus_a, zeta);

    v.low = _vreinterpretq_s16_s32(_vtrn1q_s32(
        _vreinterpretq_s32_s16(a),
        _vreinterpretq_s32_s16(b),
    ));
    v.high = _vreinterpretq_s16_s32(_vtrn2q_s32(
        _vreinterpretq_s32_s16(a),
        _vreinterpretq_s32_s16(b),
    ));
    v
}

#[inline(always)]
pub(crate) fn inv_ntt_layer_2_step(mut v: SIMD128Vector, zeta1: i16, zeta2: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    //let a_minus_b = simd::Vector::sub(b, &a);
    //a = simd::Vector::add(a, &b);
    //b = simd::Vector::montgomery_multiply_fe_by_fer(a_minus_b, zeta_r);
    //(a, b)

    let zetas = [zeta1, zeta1, zeta1, zeta1, zeta2, zeta2, zeta2, zeta2];
    let zeta = _vld1q_s16(&zetas);

    let a = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(v.low),
        _vreinterpretq_s64_s16(v.high),
    ));
    let b = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(v.low),
        _vreinterpretq_s64_s16(v.high),
    ));

    let b_minus_a = _vsubq_s16(b, a);
    let a = _vaddq_s16(a, b);
    let b = montgomery_multiply_int16x8_t(b_minus_a, zeta);

    v.low = _vreinterpretq_s16_s64(_vtrn1q_s64(
        _vreinterpretq_s64_s16(a),
        _vreinterpretq_s64_s16(b),
    ));
    v.high = _vreinterpretq_s16_s64(_vtrn2q_s64(
        _vreinterpretq_s64_s16(a),
        _vreinterpretq_s64_s16(b),
    ));
    v
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${zeta_c} /\
    Spec.Utils.is_i16b_array (2 * 3328) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Spec.Utils.is_i16b_array (4 * 3328) (repr ${result}) /\
    (forall (i: nat{i < 8}).
        (v (Seq.index (repr ${result}) i) % 3329 ==
          (v (Seq.index (repr ${vec}) (i + 8)) + v (Seq.index (repr ${vec}) i)) % 3329) /\
        (v (Seq.index (repr ${result}) (i + 8)) % 3329 ==
          ((v (Seq.index (repr ${vec}) (i + 8)) - v (Seq.index (repr ${vec}) i)) *
            v ${zeta_c} * 169) % 3329))"#))]
pub(crate) fn inv_ntt_layer_3_step(vec: SIMD128Vector, zeta_c: i16) -> SIMD128Vector {
    // This is what we are trying to do for every four elements:
    //let a_minus_b = simd::Vector::sub(b, &a);
    //a = simd::Vector::add(a, &b);
    //b = simd::Vector::montgomery_multiply_fe_by_fer(a_minus_b, zeta_r);
    //(a, b)

    let zeta = _vdupq_n_s16(zeta_c);
    hax_lib::fstar!(
        r#"assert (forall (i: nat{i < 8}). NI.get_lane_i16x8 ${zeta} i == ${zeta_c});
           assert (forall (i: nat{i < 8}). NS.is_i16b 1664 (NI.get_lane_i16x8 ${zeta} i))"#
    );
    let b_minus_a = _vsubq_s16(vec.high, vec.low);
    let mut res = vec;
    res.low = _vaddq_s16(vec.low, vec.high);
    res.high = montgomery_multiply_int16x8_t(b_minus_a, zeta);
    hax_lib::fstar!(
        r#"introduce forall (i: nat{i < 8}).
      (v (Seq.index (repr ${res}) i) % 3329 ==
        (v (Seq.index (repr ${vec}) (i + 8)) + v (Seq.index (repr ${vec}) i)) % 3329) /\
      (v (Seq.index (repr ${res}) (i + 8)) % 3329 ==
        ((v (Seq.index (repr ${vec}) (i + 8)) - v (Seq.index (repr ${vec}) i)) *
          v ${zeta_c} * 169) % 3329)
    with (assert (v (NI.get_lane_i16x8 ${b_minus_a} i) ==
            v (Seq.index (repr ${vec}) (i + 8)) - v (Seq.index (repr ${vec}) i)));
        assert (Spec.Utils.is_i16b_array (4 * 3328) (repr ${res}))"#
    );
    res
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 zeta1 /\ Spec.Utils.is_i16b 1664 zeta2 /\
                            Spec.Utils.is_i16b 1664 zeta3 /\ Spec.Utils.is_i16b 1664 zeta4"#))]
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

    SIMD128Vector {
        low: low2,
        high: high2,
    }
}

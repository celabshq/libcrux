use super::vector_type::*;
use crate::vector::{traits::INVERSE_OF_MODULUS_MOD_MONTGOMERY_R, FIELD_MODULUS};
use libcrux_intrinsics::arm64::*;

#[cfg(hax)]
use crate::vector::traits::spec;

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::fstar::before(
    interface,
    r#"unfold let repr = Libcrux_ml_kem.Vector.Neon.Vector_type.repr"#
)]
#[hax_lib::requires(fstar!(r#"${spec::add_pre} (repr ${lhs}) (repr ${rhs})"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::add_post} (repr ${lhs}) (repr ${rhs}) (repr ${result})"#))]
pub(crate) fn add(mut lhs: SIMD128Vector, rhs: &SIMD128Vector) -> SIMD128Vector {
    #[cfg(hax)]
    let _lhs0 = lhs;

    lhs.low = _vaddq_s16(lhs.low, rhs.low);
    lhs.high = _vaddq_s16(lhs.high, rhs.high);
    lhs
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(fstar!(r#"${spec::sub_pre} (repr ${lhs}) (repr ${rhs})"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::sub_post} (repr ${lhs}) (repr ${rhs}) (repr ${result})"#))]
pub(crate) fn sub(mut lhs: SIMD128Vector, rhs: &SIMD128Vector) -> SIMD128Vector {
    #[cfg(hax)]
    let _lhs0 = lhs;

    lhs.low = _vsubq_s16(lhs.low, rhs.low);
    lhs.high = _vsubq_s16(lhs.high, rhs.high);
    lhs
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(fstar!(r#"${spec::multiply_by_constant_pre} (repr ${vec}) c"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::multiply_by_constant_post} (repr ${vec}) c (repr ${result})"#))]
pub(crate) fn multiply_by_constant(mut vec: SIMD128Vector, c: i16) -> SIMD128Vector {
    #[cfg(hax)]
    let _vec0 = vec;

    vec.low = _vmulq_n_s16(vec.low, c);
    vec.high = _vmulq_n_s16(vec.high, c);
    vec
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::ensures(|result| fstar!(r#"repr ${result} == Spec.Utils.map_array (fun x -> x &. $c) (repr ${vec})"#))]
pub(crate) fn bitwise_and_with_constant(mut vec: SIMD128Vector, c: i16) -> SIMD128Vector {
    #[cfg(hax)]
    let _vec0 = vec;

    let c_vec = _vdupq_n_s16(c);
    vec.low = _vandq_s16(vec.low, c_vec);
    vec.high = _vandq_s16(vec.high, c_vec);
    hax_lib::fstar!(
        r#"Seq.lemma_eq_intro (repr ${vec}) (Spec.Utils.map_array (fun x -> x &. c) (repr ${_vec0}))"#
    );
    vec
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(SHIFT_BY >= 0 && SHIFT_BY < 16)]
#[hax_lib::ensures(|result| fstar!(r#"(v_SHIFT_BY >=. (mk_i32 0) /\ v_SHIFT_BY <. (mk_i32 16)) ==>
        repr ${result} == Spec.Utils.map_array (fun x -> x >>! ${SHIFT_BY}) (repr ${vec})"#))]
pub(crate) fn shift_right<const SHIFT_BY: i32>(mut vec: SIMD128Vector) -> SIMD128Vector {
    // Should find special cases of this
    // e.g when doing a right shift just to propagate signed bits, use vclezq_s32 instead
    #[cfg(hax)]
    let _vec0 = vec;

    vec.low = _vshrq_n_s16::<SHIFT_BY>(vec.low);
    vec.high = _vshrq_n_s16::<SHIFT_BY>(vec.high);
    hax_lib::fstar!(
        r#"Seq.lemma_eq_intro (repr ${vec}) (Spec.Utils.map_array (fun x -> x >>! v_SHIFT_BY) (repr ${_vec0}))"#
    );
    vec
}

// #[inline(always)]
// pub(crate) fn shift_left<const SHIFT_BY: i32>(mut lhs: SIMD128Vector) -> SIMD128Vector {
//     lhs.low = _vshlq_n_s16::<SHIFT_BY>(lhs.low);
//     lhs.high = _vshlq_n_s16::<SHIFT_BY>(lhs.high);
//     lhs
// }

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b_array (pow2 12 - 1) (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall i.
    (let x = Seq.index (repr ${vec}) i in
     let y = Seq.index (repr ${result}) i in
     ((v y == v x - 3329 \/ v y == v x) /\
      (v y % 3329 == v x % 3329)))"#))]
pub(crate) fn cond_subtract_3329(mut vec: SIMD128Vector) -> SIMD128Vector {
    #[cfg(hax)]
    let _vec0 = vec;

    let c = _vdupq_n_s16(3329);
    let m0 = _vcgeq_s16(vec.low, c);
    let m1 = _vcgeq_s16(vec.high, c);
    let rm0 = _vreinterpretq_s16_u16(m0);
    let rm1 = _vreinterpretq_s16_u16(m1);
    let c0 = _vandq_s16(c, rm0);
    let c1 = _vandq_s16(c, rm1);
    vec.low = _vsubq_s16(vec.low, c0);
    vec.high = _vsubq_s16(vec.high, c1);
    // Per lane: the u16 compare mask reinterprets to -1/0 in the i16 view
    // (cast_mod post on _vreinterpretq_s16_u16); 3329 &. (-1|0) collapses via
    // logand_lemma; the subtraction is then exact under the 2^12 input bound.
    hax_lib::fstar!(
        r#"introduce forall (j: nat{j < 16}).
    (let x = Seq.index (repr ${_vec0}) j in
     let y = Seq.index (repr ${vec}) j in
     ((v y == v x - 3329 \/ v y == v x) /\
      (v y % 3329 == v x % 3329)))
with begin
  if j < 8 then begin
    Rust_primitives.Integers.logand_lemma (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${c} j) (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${rm0} j)
  end else begin
    Rust_primitives.Integers.logand_lemma (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${c} (j - 8)) (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${rm1} (j - 8))
  end
end"#
    );
    vec
}

const BARRETT_MULTIPLIER: i16 = 20159;

#[inline(always)]
pub(crate) fn barrett_reduce_int16x8_t(v: _int16x8_t) -> _int16x8_t {
    // This is what we are trying to do in portable:
    // let t = (value as i32 * BARRETT_MULTIPLIER) + (BARRETT_R >> 1);
    // let quotient = (t >> BARRETT_SHIFT) as i16;
    // let result = value - (quotient * FIELD_MODULUS);

    let adder = _vdupq_n_s16(1024);
    let vec = _vqdmulhq_n_s16(v, BARRETT_MULTIPLIER as i16);
    let vec = _vaddq_s16(vec, adder);
    let quotient = _vshrq_n_s16::<11>(vec);
    let sub = _vmulq_n_s16(quotient, FIELD_MODULUS);
    _vsubq_s16(v, sub)
}

#[inline(always)]
pub(crate) fn barrett_reduce(mut vec: SIMD128Vector) -> SIMD128Vector {
    //let pv = crate::simd::portable::from_i16_array(to_i16_array(v));
    //from_i16_array(crate::simd::portable::to_i16_array(crate::simd::portable::barrett_reduce(pv)))

    // This is what we are trying to do in portable:
    // let t = (value as i32 * BARRETT_MULTIPLIER) + (BARRETT_R >> 1);
    // let quotient = (t >> BARRETT_SHIFT) as i16;
    // let result = value - (quotient * FIELD_MODULUS);

    vec.low = barrett_reduce_int16x8_t(vec.low);
    vec.high = barrett_reduce_int16x8_t(vec.high);
    vec
}

#[inline(always)]
pub(crate) fn montgomery_reduce_int16x8_t(low: _int16x8_t, high: _int16x8_t) -> _int16x8_t {
    // This is what we are trying to do in portable:
    // let k = low as i16 * INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;
    // let k_times_modulus = (k as i16 as i16) * (FIELD_MODULUS as i16);
    // let c = (k_times_modulus >> MONTGOMERY_SHIFT) as i16;
    // high - c

    let k = _vreinterpretq_s16_u16(_vmulq_n_u16(
        _vreinterpretq_u16_s16(low),
        INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as u16,
    ));
    let c = _vshrq_n_s16::<1>(_vqdmulhq_n_s16(k, FIELD_MODULUS as i16));
    _vsubq_s16(high, c)
}

#[inline(always)]
pub(crate) fn montgomery_multiply_by_constant_int16x8_t(v: _int16x8_t, c: i16) -> _int16x8_t {
    // This is what we are trying to do in portable:
    // let value = v as i16 * c
    // let k = (value as i16) as i16 * INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;
    // let k_times_modulus = (k as i16 as i16) * (FIELD_MODULUS as i16);
    // let c = (k_times_modulus >> MONTGOMERY_SHIFT) as i16;
    // let value_high = (value >> MONTGOMERY_SHIFT) as i16;
    // value_high - c

    let v_low = _vmulq_n_s16(v, c);
    let v_high = _vshrq_n_s16::<1>(_vqdmulhq_n_s16(v, c));
    montgomery_reduce_int16x8_t(v_low, v_high)
}

#[inline(always)]
pub(crate) fn montgomery_multiply_int16x8_t(v: _int16x8_t, c: _int16x8_t) -> _int16x8_t {
    // This is what we are trying to do in portable:
    // let value = v as i16 * c
    // let k = (value as i16) as i16 * INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;
    // let k_times_modulus = (k as i16 as i16) * (FIELD_MODULUS as i16);
    // let c = (k_times_modulus >> MONTGOMERY_SHIFT) as i16;
    // let value_high = (value >> MONTGOMERY_SHIFT) as i16;
    // value_high - c

    let v_low = _vmulq_s16(v, c);
    let v_high = _vshrq_n_s16::<1>(_vqdmulhq_s16(v, c));
    montgomery_reduce_int16x8_t(v_low, v_high)
}

#[inline(always)]
pub(crate) fn montgomery_multiply_by_constant(mut vec: SIMD128Vector, c: i16) -> SIMD128Vector {
    vec.low = montgomery_multiply_by_constant_int16x8_t(vec.low, c);
    vec.high = montgomery_multiply_by_constant_int16x8_t(vec.high, c);
    vec
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b_array 3328 (repr ${a})"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall i.
    (let x = Seq.index (repr ${a}) i in
     let y = Seq.index (repr ${result}) i in
     (v y >= 0 /\ v y <= 3328 /\ (v y % 3329 == v x % 3329)))"#))]
pub(crate) fn to_unsigned_representative(a: SIMD128Vector) -> SIMD128Vector {
    let t = shift_right::<15>(a);
    hax_lib::fstar!(
        r#"assert (forall i. Seq.index (repr ${t}) i == ((Seq.index (repr ${a}) i) >>! (mk_i32 15)));
assert (forall i. Seq.index (repr ${a}) i >=. mk_i16 0 ==> Seq.index (repr ${t}) i == mk_i16 0);
assert (forall i. Seq.index (repr ${a}) i <. mk_i16 0 ==> Seq.index (repr ${t}) i == mk_i16 (-1))"#
    );
    let fm = bitwise_and_with_constant(t, FIELD_MODULUS);
    hax_lib::fstar!(
        r#"assert (forall i. Seq.index (repr ${fm}) i == (Seq.index (repr ${t}) i &. Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS));
assert (forall i. Seq.index (repr ${a}) i >=. mk_i16 0 ==> Seq.index (repr ${fm}) i == mk_i16 0);
assert (forall i. Seq.index (repr ${a}) i <. mk_i16 0 ==> Seq.index (repr ${fm}) i == mk_i16 3329)"#
    );
    add(a, &fm)
}

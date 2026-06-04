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

// Functional foundation for the Neon barrett/montgomery primitives.  The four
// helper lemmas bridge the saturating doubling-multiply-high (`_vqdmulhq_*`)
// model to the scalar `Spec.Utils.{barrett_red,mont_mul_red_i16}` reductions.
// The per-lane `int16x8_t` workers (`*_int16x8_t`) are `opaque_to_smt` so the
// SIMD128 wrappers compose only against their (genuine `forall i<8`) posts —
// without that, the wrappers' `pow2`/`barrett_red` machinery leaks into the
// composition VC and saturates Z3.
#[hax_lib::fstar::before(
    r#"
let lemma_neon_floor_collapse (p: int)
    : Lemma ((p / pow2 15 + pow2 10) / pow2 11 == (p / pow2 16 + pow2 9) / pow2 10) =
  FStar.Math.Lemmas.pow2_plus 10 15;
  FStar.Math.Lemmas.division_addition_lemma p (pow2 15) (pow2 10);
  FStar.Math.Lemmas.pow2_plus 15 11;
  FStar.Math.Lemmas.division_multiplication_lemma (p + pow2 25) (pow2 15) (pow2 11);
  FStar.Math.Lemmas.pow2_plus 9 16;
  FStar.Math.Lemmas.division_addition_lemma p (pow2 16) (pow2 9);
  FStar.Math.Lemmas.pow2_plus 16 10;
  FStar.Math.Lemmas.division_multiplication_lemma (p + pow2 25) (pow2 16) (pow2 10)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"

(* The Neon barrett lane chain (saturating doubling-mul-high + add 1024 + >>11)
   collapses to the scalar `Spec.Utils.barrett_red`: both quotients equal
   floor((x*20159 + 2^25) / 2^26). *)
let lemma_barrett_lane_eq (x: i16)
    : Lemma (requires Spec.Utils.is_i16b 28296 x)
      (ensures
        (let prod:i32 = ((cast x <: i32) *. (cast (mk_i16 20159) <: i32)) >>! (mk_i32 15) in
          let vec1:i16 =
            (if prod >. mk_i32 32767
              then mk_i16 32767
              else if prod <. mk_i32 (- 32768) then mk_i16 (- 32768) else (cast prod <: i16))
          in
          x -. (((vec1 +. mk_i16 1024) >>! (mk_i32 11)) *. mk_i16 3329) == Spec.Utils.barrett_red x)) =
  let xx:int = v x in
  assert (xx * 20159 <= 570419064 /\ xx * 20159 >= -570419064);
  let prod:i32 = ((cast x <: i32) *. (cast (mk_i16 20159) <: i32)) >>! (mk_i32 15) in
  assert (v prod == (xx * 20159) / pow2 15);
  FStar.Math.Lemmas.lemma_div_le (xx * 20159) 570419064 (pow2 15);
  FStar.Math.Lemmas.lemma_div_le (-570419064) (xx * 20159) (pow2 15);
  assert_norm (570419064 / pow2 15 == 17407);
  assert_norm ((-570419064) / pow2 15 == -17408);
  let vec1:i16 = (cast prod <: i16) in
  let vec2:i16 = vec1 +. mk_i16 1024 in
  let quotient:i16 = vec2 >>! (mk_i32 11) in
  lemma_neon_floor_collapse (xx * 20159);
  assert_norm (pow2 10 == 1024);
  assert_norm (pow2 9 == 512);
  assert (v quotient == ((xx * 20159) / pow2 16 + 512) / pow2 10);
  ()
#pop-options

#push-options "--z3rlimit 200"
(* The unsigned multiply-by-62209 detour reinterprets to a signed
   multiply-by-(-3327), since 62209 == -3327 (mod 2^16). *)
let lemma_u16_detour (a: i16)
    : Lemma
      (Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype
          #Rust_primitives.Integers.i16_inttype
          ((Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype
                #Rust_primitives.Integers.u16_inttype
                a) *.
            mk_u16 62209) ==
        a *. (mk_i16 (-3327))) =
  let aa = v a in
  FStar.Math.Lemmas.lemma_mod_mul_distr_l aa 62209 (pow2 16);
  FStar.Math.Lemmas.lemma_mod_plus (aa * (-3327)) aa (pow2 16);
  assert (aa * 62209 == aa * (-3327) + aa * pow2 16);
  assert (((aa % pow2 16) * 62209) % pow2 16 == (aa * (-3327)) % pow2 16);
  ()
#pop-options

#push-options "--z3rlimit 300"
(* The saturating doubling-mul-high `e_vqdmulhq_n_s16 m d` (model `(m*d)>>15`)
   then `>>1` equals the scalar high half `(m*d)>>16`, for products below 2^28
   (so no saturation and the i16 cast is exact). *)
let lemma_qdmulh_shift1 (m d: i16)
    : Lemma (requires Spec.Utils.is_intb (pow2 28) (v m * v d))
      (ensures
        (let prod:i32 = ((cast m <: i32) *. (cast d <: i32)) >>! (mk_i32 15) in
          let sat:i16 =
            (if prod >. mk_i32 32767
              then mk_i16 32767
              else if prod <. mk_i32 (- 32768) then mk_i16 (- 32768) else (cast prod <: i16))
          in
          (sat >>! (mk_i32 1)) ==
          (cast (((cast m <: i32) *. (cast d <: i32)) >>! (mk_i32 16)) <: i16))) =
  let p:int = v m * v d in
  assert (v ((cast m <: i32) *. (cast d <: i32)) == p);
  let prod:i32 = ((cast m <: i32) *. (cast d <: i32)) >>! (mk_i32 15) in
  assert (v prod == p / pow2 15);
  FStar.Math.Lemmas.lemma_div_le p (pow2 28) (pow2 15);
  FStar.Math.Lemmas.lemma_div_le (- pow2 28) p (pow2 15);
  assert_norm (pow2 28 / pow2 15 == pow2 13);
  assert_norm ((- pow2 28) / pow2 15 == - pow2 13);
  assert_norm (pow2 13 < 32767);
  let sat:i16 = (cast prod <: i16) in
  assert (v (sat >>! (mk_i32 1)) == (p / pow2 15) / pow2 1);
  FStar.Math.Lemmas.division_multiplication_lemma p (pow2 15) (pow2 1);
  FStar.Math.Lemmas.pow2_plus 15 1;
  ()
#pop-options
"#
)]
// Precondition stays `l_True` (the existing caller `inv_ntt_layer_1_step` in
// Neon.Ntt does not track the 28296 bound); the functional post is guarded by
// that bound as an antecedent.  The SIMD128 `barrett_reduce` wrapper discharges
// it from `barrett_reduce_pre`.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|result| fstar!(r#"(forall (i: nat{i < 8}).
      Spec.Utils.is_i16b 28296 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i)) ==>
    (forall (i: nat{i < 8}).
      Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) /\
      v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) % 3329 ==
      v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) % 3329)"#))]
pub(crate) fn barrett_reduce_int16x8_t(a: _int16x8_t) -> _int16x8_t {
    // This is what we are trying to do in portable:
    // let t = (value as i32 * BARRETT_MULTIPLIER) + (BARRETT_R >> 1);
    // let quotient = (t >> BARRETT_SHIFT) as i16;
    // let result = value - (quotient * FIELD_MODULUS);

    let adder = _vdupq_n_s16(1024);
    let prod = _vqdmulhq_n_s16(a, BARRETT_MULTIPLIER as i16);
    let summed = _vaddq_s16(prod, adder);
    let quotient = _vshrq_n_s16::<11>(summed);
    let sub = _vmulq_n_s16(quotient, FIELD_MODULUS);
    let result = _vsubq_s16(a, sub);
    hax_lib::fstar!(
        r#"introduce
  (forall (i: nat{i < 8}). Spec.Utils.is_i16b 28296 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i)) ==>
  (forall (i: nat{i < 8}).
      Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) /\
      v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) % 3329 ==
      v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) % 3329)
  with _h.
  introduce
  forall (i: nat{i < 8}).
      Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) /\
      v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) % 3329 ==
      v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) % 3329
  with (assert (Spec.Utils.is_i16b 28296 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i));
    lemma_barrett_lane_eq (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i);
    Spec.Utils.lemma_barrett_red (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i))"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::barrett_reduce_pre} (repr ${vec})"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::barrett_reduce_post} (repr ${vec}) (repr ${result})"#))]
pub(crate) fn barrett_reduce(mut vec: SIMD128Vector) -> SIMD128Vector {
    // This is what we are trying to do in portable:
    // let t = (value as i32 * BARRETT_MULTIPLIER) + (BARRETT_R >> 1);
    // let quotient = (t >> BARRETT_SHIFT) as i16;
    // let result = value - (quotient * FIELD_MODULUS);

    #[cfg(hax)]
    let _vec0 = vec;

    // Unfold the (opaque) input/output array bounds, then establish the per-half
    // input bounds (`lemma_repr_index` SMTPat bridges `repr` lanes to the
    // `get_lane_i16x8` of `.f_low`/`.f_high`) so each per-half call's guarded
    // post fires; the output bound folds back into the array post.
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 28296);
           reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328);
           assert (forall (i: nat{i < 8}). Seq.index (repr ${vec}) i == Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${vec}.f_low i);
           assert (forall (i: nat{i < 8}). Seq.index (repr ${vec}) (i + 8) == Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${vec}.f_high i);
           assert (forall (i: nat{i < 8}). Spec.Utils.is_i16b 28296 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${vec}.f_low i));
           assert (forall (i: nat{i < 8}). Spec.Utils.is_i16b 28296 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${vec}.f_high i))"#
    );
    vec.low = barrett_reduce_int16x8_t(vec.low);
    vec.high = barrett_reduce_int16x8_t(vec.high);
    // Compose the per-half facts into the per-lane opaque atom (pow2-free).
    hax_lib::fstar!(
        r#"introduce
  forall (j: nat{j < 16}).
      Libcrux_ml_kem.Vector.Traits.Spec.barrett_reduce_lane_post (Seq.index (repr ${_vec0}) j) (Seq.index (repr ${vec}) j)
  with (reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.barrett_reduce_lane_post) (Libcrux_ml_kem.Vector.Traits.Spec.barrett_reduce_lane_post (Seq.index (repr ${_vec0}) j) (Seq.index (repr ${vec}) j));
    Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro (v (Seq.index (repr ${vec}) j)) (v (Seq.index (repr ${_vec0}) j)))"#
    );
    vec
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 8}).
    Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i ==
    Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${high} i -.
    (cast (((cast (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${low} i *. (mk_i16 (-3327))) <: i32) *. (mk_i32 3329)) >>! (mk_i32 16)) <: i16)"#))]
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
    let result = _vsubq_s16(high, c);
    hax_lib::fstar!(
        r#"introduce
  forall (i: nat{i < 8}).
      Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i ==
      Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${high} i -.
      (cast (((cast (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${low} i *. (mk_i16 (-3327))) <: i32) *. (mk_i32 3329)) >>! (mk_i32 16)) <: i16)
  with (lemma_u16_detour (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${low} i);
    assert (Spec.Utils.is_intb (pow2 28) (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${k} i) * v Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS));
    lemma_qdmulh_shift1 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${k} i) Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS)"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 300")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"Spec.Utils.is_i16b 1664 ${c}"#))]
#[hax_lib::ensures(|result| fstar!(r#"forall (i: nat{i < 8}).
    Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) /\
    v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) % 3329 ==
    (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) * v ${c} * 169) % 3329"#))]
pub(crate) fn montgomery_multiply_by_constant_int16x8_t(a: _int16x8_t, c: i16) -> _int16x8_t {
    // This is what we are trying to do in portable:
    // let value = v as i16 * c
    // let k = (value as i16) as i16 * INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;
    // let k_times_modulus = (k as i16 as i16) * (FIELD_MODULUS as i16);
    // let c = (k_times_modulus >> MONTGOMERY_SHIFT) as i16;
    // let value_high = (value >> MONTGOMERY_SHIFT) as i16;
    // value_high - c

    let v_low = _vmulq_n_s16(a, c);
    let v_high = _vshrq_n_s16::<1>(_vqdmulhq_n_s16(a, c));
    let result = montgomery_reduce_int16x8_t(v_low, v_high);
    hax_lib::fstar!(
        r#"introduce
  forall (i: nat{i < 8}).
      Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) /\
      v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) % 3329 ==
      (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) * v ${c} * 169) % 3329
  with (assert (Spec.Utils.is_intb (pow2 28) (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) * v ${c}));
    lemma_qdmulh_shift1 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) ${c};
    assert (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i == Spec.Utils.mont_mul_red_i16 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) ${c});
    Spec.Utils.lemma_mont_mul_red_i16 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) ${c})"#
    );
    result
}

// Two-vector montgomery: same shape as the by-constant worker but a vector
// multiplier `c`.  Pre stays `l_True`; the functional post is guarded by the
// per-lane validity antecedent (`|c[i]| <= 1664` OR the product fits the
// montgomery bound) so the not-yet-functional ntt callers keep building (same
// implication-post device as `barrett_reduce_int16x8_t`).
#[inline(always)]
#[hax_lib::fstar::options("--fuel 1 --ifuel 1 --z3rlimit 300")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::ensures(|result| fstar!(r#"(forall (i: nat{i < 8}).
      Spec.Utils.is_i16b 1664 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${c} i) \/
      Spec.Utils.is_intb (3326 * pow2 15)
        (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) *
          v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${c} i))) ==>
    (forall (i: nat{i < 8}).
      Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) /\
      v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) % 3329 ==
      (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) *
        v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${c} i) * 169) % 3329)"#))]
pub(crate) fn montgomery_multiply_int16x8_t(a: _int16x8_t, c: _int16x8_t) -> _int16x8_t {
    // This is what we are trying to do in portable:
    // let value = v as i16 * c
    // let k = (value as i16) as i16 * INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;
    // let k_times_modulus = (k as i16 as i16) * (FIELD_MODULUS as i16);
    // let c = (k_times_modulus >> MONTGOMERY_SHIFT) as i16;
    // let value_high = (value >> MONTGOMERY_SHIFT) as i16;
    // value_high - c

    let v_low = _vmulq_s16(a, c);
    let v_high = _vshrq_n_s16::<1>(_vqdmulhq_s16(a, c));
    let result = montgomery_reduce_int16x8_t(v_low, v_high);
    hax_lib::fstar!(
        r#"introduce
  (forall (i: nat{i < 8}).
      Spec.Utils.is_i16b 1664 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${c} i) \/
      Spec.Utils.is_intb (3326 * pow2 15)
        (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) *
          v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${c} i))) ==>
  (forall (i: nat{i < 8}).
      Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) /\
      v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) % 3329 ==
      (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) *
        v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${c} i) * 169) % 3329)
  with _h.
  introduce forall (i: nat{i < 8}).
      Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) /\
      v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i) % 3329 ==
      (v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i) *
        v (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${c} i) * 169) % 3329
  with (let ai = Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${a} i in
    let ci = Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${c} i in
    assert (Spec.Utils.is_i16b 1664 ci \/ Spec.Utils.is_intb (3326 * pow2 15) (v ai * v ci));
    assert (Spec.Utils.is_intb (pow2 28) (v ai * v ci));
    lemma_qdmulh_shift1 ai ci;
    assert (Libcrux_intrinsics.Arm64_extract.get_lane_i16x8 ${result} i ==
        Spec.Utils.mont_mul_red_i16 ai ci);
    Spec.Utils.lemma_mont_mul_red_i16 ai ci)"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(fstar!(r#"${spec::montgomery_multiply_by_constant_pre} (repr ${vec}) ${c}"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::montgomery_multiply_by_constant_post} (repr ${vec}) ${c} (repr ${result})"#))]
pub(crate) fn montgomery_multiply_by_constant(mut vec: SIMD128Vector, c: i16) -> SIMD128Vector {
    #[cfg(hax)]
    let _vec0 = vec;

    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque) (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque 3328)"#
    );
    vec.low = montgomery_multiply_by_constant_int16x8_t(vec.low, c);
    vec.high = montgomery_multiply_by_constant_int16x8_t(vec.high, c);
    hax_lib::fstar!(
        r#"introduce
  forall (j: nat{j < 16}).
      Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_lane_post (Seq.index (repr ${_vec0}) j) ${c} (Seq.index (repr ${vec}) j)
  with (reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_lane_post) (Libcrux_ml_kem.Vector.Traits.Spec.montgomery_multiply_lane_post (Seq.index (repr ${_vec0}) j) ${c} (Seq.index (repr ${vec}) j));
    Hacspec_ml_kem.ModQ.lemma_mod_q_eq_intro (v (Seq.index (repr ${vec}) j)) (v (Seq.index (repr ${_vec0}) j) * v ${c} * 169))"#
    );
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

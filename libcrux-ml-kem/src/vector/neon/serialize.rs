use super::*;
use crate::vector::portable::PortableVector;
use libcrux_intrinsics::arm64::*;

#[cfg(hax)]
use crate::vector::traits::spec;

#[inline(always)]
#[hax_lib::fstar::before(
    interface,
    r#"unfold let repr = Libcrux_ml_kem.Vector.Neon.Vector_type.repr"#
)]
#[hax_lib::fstar::before(
    r#"
module NI = Libcrux_intrinsics.Arm64_ml_kem_views
module BV = Rust_primitives.BitVectors
module I = Rust_primitives.Integers
module VT = Libcrux_ml_kem.Vector.Neon.Vector_type
module Spec = Libcrux_ml_kem.Vector.Traits.Spec
open Libcrux_ml_kem.Vector.Neon.Serialize_theory
"#
)]
#[hax_lib::fstar::options("--z3rlimit 400")]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 1 (repr ${v})"#))]
#[hax_lib::ensures(|result| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 1 (repr ${v}) ${result}"#))]
pub(crate) fn serialize_1(v: SIMD128Vector) -> [u8; 2] {
    let shifter: [i16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let shift = _vld1q_s16(&shifter);
    let low = _vshlq_s16(v.low, shift);
    let high = _vshlq_s16(v.high, shift);
    let low = _vaddvq_s16(low);
    let high = _vaddvq_s16(high);
    let result = [low as u8, high as u8];
    proof!(
        r#"(* shift-amount facts from the concrete shifter (lane j shifts by j) *)
assert (forall (j: nat{j < 8}).
      Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 ${shift} j == Seq.index ${shifter} j);
assert (Seq.index ${shifter} 0 == mk_i16 0 /\ Seq.index ${shifter} 1 == mk_i16 1 /\
        Seq.index ${shifter} 2 == mk_i16 2 /\ Seq.index ${shifter} 3 == mk_i16 3 /\
        Seq.index ${shifter} 4 == mk_i16 4 /\ Seq.index ${shifter} 5 == mk_i16 5 /\
        Seq.index ${shifter} 6 == mk_i16 6 /\ Seq.index ${shifter} 7 == mk_i16 7);
lemma_ser1_shift_amounts ${shift} ${shifter};
assert (Seq.index ${result} 0 == (cast (${low} <: i16) <: u8));
assert (Seq.index ${result} 1 == (cast (${high} <: i16) <: u8));
lemma_serialize_1_post ${v} ${shift} ${result}"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(a.len() == 2)]
#[hax_lib::ensures(|result| fstar!(r#"${spec::deserialize_1_post} ${a} (repr ${result})"#))]
pub(crate) fn deserialize_1(a: &[u8]) -> SIMD128Vector {
    let one = _vdupq_n_s16(1);
    let low = _vdupq_n_s16(a[0] as i16);
    let high = _vdupq_n_s16(a[1] as i16);
    let shifter: [i16; 8] = [0, 0xff, -2, -3, -4, -5, -6, -7];
    let shift = _vld1q_s16(&shifter);
    let low = _vshlq_s16(low, shift);
    let high = _vshlq_s16(high, shift);
    let result = SIMD128Vector {
        low: _vandq_s16(low, one),
        high: _vandq_s16(high, one),
    };
    proof!(
        r#"
let rr : t_Array i16 (mk_usize 16) = Libcrux_ml_kem.Vector.Neon.Vector_type.repr ${result} in
let pre0 : i16 = cast (${a}.[ mk_usize 0 ] <: u8) <: i16 in
let pre1 : i16 = cast (${a}.[ mk_usize 1 ] <: u8) <: i16 in
assert (forall (m: nat{m < 8}). Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 ${one} m == mk_i16 1);
assert (forall (m: nat{m < 8}).
          Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 ${shift} m == Seq.index ${shifter} m);
let aux (i: nat{i < 16}) : Lemma
  (Rust_primitives.Integers.get_bit (Seq.index rr i) (mk_usize 0)
     == Rust_primitives.Integers.get_bit (Seq.index ${a} (i / 8)) (sz (i % 8))
   /\ Rust_primitives.BitVectors.bounded (Seq.index rr i) 1) =
  let k : nat = i % 8 in
  let byte = if i < 8 then (${a}.[ mk_usize 0 ] <: u8) else (${a}.[ mk_usize 1 ] <: u8) in
  let pre_lane : i16 = if i < 8 then pre0 else pre1 in
  let shk = Seq.index ${shifter} k in
  (match k with
   | 0 -> assert (v (shk %! mk_i16 256) == 0)
   | 1 -> assert (v (shk %! mk_i16 256) == 255)
   | 2 -> assert (v (shk %! mk_i16 256) == 254)
   | 3 -> assert (v (shk %! mk_i16 256) == 253)
   | 4 -> assert (v (shk %! mk_i16 256) == 252)
   | 5 -> assert (v (shk %! mk_i16 256) == 251)
   | 6 -> assert (v (shk %! mk_i16 256) == 250)
   | _ -> assert (v (shk %! mk_i16 256) == 249));
  assert (v (shk %! mk_i16 256) == (if k = 0 then 0 else 256 - k));
  (if i < 8
   then lemma_deser1_half_lane pre0 ${shift} ${one} ${shifter} k
   else lemma_deser1_half_lane pre1 ${shift} ${one} ${shifter} k);
  assert (Seq.index rr i
          == (Libcrux_intrinsics.Arm64_ml_kem_views.arm_sshl_i16 pre_lane shk &. mk_i16 1));
  assert (byte == Seq.index ${a} (i / 8));
  assert (k == i % 8);
  assert (cast byte == pre_lane);
  lemma_deser1_lane byte k shk
in
Classical.forall_intro aux;
let bv_in : Rust_primitives.BitVectors.bit_vec 16 =
  Rust_primitives.BitVectors.bit_vec_of_int_t_array #u8_inttype #(mk_usize 2) (${a} <: t_Array u8 (mk_usize 2)) 8 in
let bv_out : Rust_primitives.BitVectors.bit_vec 16 =
  Rust_primitives.BitVectors.bit_vec_of_int_t_array #i16_inttype #(mk_usize 16) rr 1 in
introduce forall (i: nat{i < 16}). bv_in i == bv_out i
with (assert (bv_in i == Rust_primitives.Integers.get_bit (Seq.index ${a} (i / 8)) (sz (i % 8)));
      assert (bv_out i == Rust_primitives.Integers.get_bit (Seq.index rr i) (mk_usize 0)));
BitVecEq.bit_vec_equal_intro bv_in (BitVecEq.retype bv_out)
"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400")]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 4 (repr ${v})"#))]
#[hax_lib::ensures(|result| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 4 (repr ${v}) ${result}"#))]
pub(crate) fn serialize_4(v: SIMD128Vector) -> [u8; 8] {
    let shifter: [i16; 8] = [0, 4, 8, 12, 0, 4, 8, 12];
    let shift = _vld1q_s16(&shifter);
    let lowt = _vshlq_u16(_vreinterpretq_u16_s16(v.low), shift);
    let hight = _vshlq_u16(_vreinterpretq_u16_s16(v.high), shift);
    let sum0 = _vaddv_u16(_vget_low_u16(lowt)) as u64;
    let sum1 = _vaddv_u16(_vget_high_u16(lowt)) as u64;
    let sum2 = _vaddv_u16(_vget_low_u16(hight)) as u64;
    let sum3 = _vaddv_u16(_vget_high_u16(hight)) as u64;
    let sum = sum0 | (sum1 << 16) | (sum2 << 32) | (sum3 << 48);
    let result = sum.to_le_bytes();
    proof!(
        r#"assert (forall (j: nat{j < 8}).
      Libcrux_intrinsics.Arm64_ml_kem_views.get_lane_i16x8 ${shift} j == Seq.index ${shifter} j);
assert (Seq.index ${shifter} 0 == mk_i16 0 /\ Seq.index ${shifter} 1 == mk_i16 4 /\
        Seq.index ${shifter} 2 == mk_i16 8 /\ Seq.index ${shifter} 3 == mk_i16 12 /\
        Seq.index ${shifter} 4 == mk_i16 0 /\ Seq.index ${shifter} 5 == mk_i16 4 /\
        Seq.index ${shifter} 6 == mk_i16 8 /\ Seq.index ${shifter} 7 == mk_i16 12);
lemma_ser4_shift_amounts ${shift} ${shifter};
lemma_serialize_4_post ${v} ${shift} ${sum0} ${sum1} ${sum2} ${sum3} ${sum} ${result}"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(v.len() == 8)]
#[hax_lib::ensures(|result| fstar!(r#"${spec::deserialize_4_post} ${v} (repr ${result})"#))]
pub(crate) fn deserialize_4(v: &[u8]) -> SIMD128Vector {
    let input = PortableVector::deserialize_4(v);
    let input_i16s = PortableVector::to_i16_array(input);
    let result = SIMD128Vector {
        low: _vld1q_s16(&input_i16s[0..8]),
        high: _vld1q_s16(&input_i16s[8..16]),
    };
    proof!(
        r#"
assert (${input_i16s}.[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                           Core_models.Ops.Range.f_end = mk_usize 8 }
                         <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${input_i16s} 0 8);
assert (${input_i16s}.[ ({ Core_models.Ops.Range.f_start = mk_usize 8;
                           Core_models.Ops.Range.f_end = mk_usize 16 }
                         <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${input_i16s} 8 16);
lemma_repr_of_two_loads ${input_i16s}
"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"${spec::serialize_5_pre} (repr ${v})"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::serialize_5_post} (repr ${v}) ${result}"#))]
pub(crate) fn serialize_5(v: SIMD128Vector) -> [u8; 10] {
    let out_i16s = to_i16_array(v);
    let out = PortableVector::from_i16_array(&out_i16s);
    PortableVector::serialize_5(out)
}

#[inline(always)]
#[hax_lib::requires(v.len() == 10)]
#[hax_lib::ensures(|result| fstar!(r#"${spec::deserialize_5_post} ${v} (repr ${result})"#))]
pub(crate) fn deserialize_5(v: &[u8]) -> SIMD128Vector {
    let output = PortableVector::deserialize_5(v);
    let array = PortableVector::to_i16_array(output);
    let result = SIMD128Vector {
        low: _vld1q_s16(&array[0..8]),
        high: _vld1q_s16(&array[8..16]),
    };
    proof!(
        r#"
assert (${array}.[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end = mk_usize 8 }
                    <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${array} 0 8);
assert (${array}.[ ({ Core_models.Ops.Range.f_start = mk_usize 8;
                      Core_models.Ops.Range.f_end = mk_usize 16 }
                    <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${array} 8 16);
lemma_repr_of_two_loads ${array}
"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 10 (repr ${v})"#))]
#[hax_lib::ensures(|result| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 10 (repr ${v}) ${result}"#))]
pub(crate) fn serialize_10(v: SIMD128Vector) -> [u8; 20] {
    let low0 = _vreinterpretq_s32_s16(_vtrn1q_s16(v.low, v.low)); // a0, a0, a2, a2, a4, a4, a6, a6
    let low1 = _vreinterpretq_s32_s16(_vtrn2q_s16(v.low, v.low)); // a1, a1, a3, a3, a5, a5, a7, a7
    let mixt = _vsliq_n_s32::<10>(low0, low1); // a1a0, a3a2, a5a4, a7a6

    let low0 = _vreinterpretq_s64_s32(_vtrn1q_s32(mixt, mixt)); // a1a0, a1a0, a5a4, a5a4
    let low1 = _vreinterpretq_s64_s32(_vtrn2q_s32(mixt, mixt)); // a3a2, a3a2, a7a6, a7a6
    let low_mix = _vsliq_n_s64::<20>(low0, low1); // a3a2a1a0, a7a6a5a4

    let high0 = _vreinterpretq_s32_s16(_vtrn1q_s16(v.high, v.high)); // a0, a0, a2, a2, a4, a4, a6, a6
    let high1 = _vreinterpretq_s32_s16(_vtrn2q_s16(v.high, v.high)); // a1, a1, a3, a3, a5, a5, a7, a7
    let mixt = _vsliq_n_s32::<10>(high0, high1); // a1a0, a3a2, a5a4, a7a6

    let high0 = _vreinterpretq_s64_s32(_vtrn1q_s32(mixt, mixt)); // a1a0, a1a0, a5a4, a5a4
    let high1 = _vreinterpretq_s64_s32(_vtrn2q_s32(mixt, mixt)); // a3a2, a3a2, a7a6, a7a6
    let high_mix = _vsliq_n_s64::<20>(high0, high1); // a3a2a1a0, a7a6a5a4

    let mut result32 = [0u8; 32];
    _vst1q_u8(&mut result32[0..16], _vreinterpretq_u8_s64(low_mix));
    _vst1q_u8(&mut result32[16..32], _vreinterpretq_u8_s64(high_mix));
    let mut result = [0u8; 20];
    result[0..5].copy_from_slice(&result32[0..5]);
    result[5..10].copy_from_slice(&result32[8..13]);
    result[10..15].copy_from_slice(&result32[16..21]);
    result[15..20].copy_from_slice(&result32[24..29]);
    proof!(
        r#"
    let _:squash (forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 0 16) k == NI.get_lane_u8x16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_u8_s64 ${low_mix}) k) =
      introduce forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 0 16) k == NI.get_lane_u8x16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_u8_s64 ${low_mix}) k
      with ()
    in
    let _:squash (forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 16 32) k == NI.get_lane_u8x16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_u8_s64 ${high_mix}) k) =
      introduce forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 16 32) k == NI.get_lane_u8x16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_u8_s64 ${high_mix}) k
      with ()
    in
    lemma_store32_to_bytes ${low_mix} ${high_mix} ${result32};
    assert (Seq.slice ${result} 0 5 == Seq.slice ${result32} 0 5);
    assert (Seq.slice ${result} 5 10 == Seq.slice ${result32} 8 13);
    assert (Seq.slice ${result} 10 15 == Seq.slice ${result32} 16 21);
    assert (Seq.slice ${result} 15 20 == Seq.slice ${result32} 24 29);
    lemma_result_mapping ${low_mix} ${high_mix} 5 ${result32} ${result};
    reveal_opaque (`%half_spine) (half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low 10);
    reveal_opaque (`%half_mixt) (half_mixt ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low 10);
    reveal_opaque (`%half_spine) (half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high 10);
    reveal_opaque (`%half_mixt) (half_mixt ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high 10);
    assert (${low_mix} == half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low 10);
    assert (${high_mix} == half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high 10);
    lemma_spine_r2 ${v} ${low_mix} ${high_mix} 10;
    lemma_serialize_post ${v} ${low_mix} ${high_mix} 5 ${result}
    "#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(v.len() == 20)]
#[hax_lib::ensures(|result| fstar!(r#"${spec::deserialize_10_post} ${v} (repr ${result})"#))]
pub(crate) fn deserialize_10(v: &[u8]) -> SIMD128Vector {
    let output = PortableVector::deserialize_10(v);
    let array = PortableVector::to_i16_array(output);
    let result = SIMD128Vector {
        low: _vld1q_s16(&array[0..8]),
        high: _vld1q_s16(&array[8..16]),
    };
    proof!(
        r#"
assert (${array}.[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end = mk_usize 8 }
                    <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${array} 0 8);
assert (${array}.[ ({ Core_models.Ops.Range.f_start = mk_usize 8;
                      Core_models.Ops.Range.f_end = mk_usize 16 }
                    <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${array} 8 16);
lemma_repr_of_two_loads ${array}
"#
    );
    result
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"${spec::serialize_11_pre} (repr ${v})"#))]
#[hax_lib::ensures(|result| fstar!(r#"${spec::serialize_11_post} (repr ${v}) ${result}"#))]
pub(crate) fn serialize_11(v: SIMD128Vector) -> [u8; 22] {
    let out_i16s = to_i16_array(v);
    let out = PortableVector::from_i16_array(&out_i16s);
    PortableVector::serialize_11(out)
}

#[inline(always)]
#[hax_lib::requires(v.len() == 22)]
#[hax_lib::ensures(|result| fstar!(r#"${spec::deserialize_11_post} ${v} (repr ${result})"#))]
pub(crate) fn deserialize_11(v: &[u8]) -> SIMD128Vector {
    let output = PortableVector::deserialize_11(v);
    let array = PortableVector::to_i16_array(output);
    let result = SIMD128Vector {
        low: _vld1q_s16(&array[0..8]),
        high: _vld1q_s16(&array[8..16]),
    };
    proof!(
        r#"
assert (${array}.[ ({ Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end = mk_usize 8 }
                    <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${array} 0 8);
assert (${array}.[ ({ Core_models.Ops.Range.f_start = mk_usize 8;
                      Core_models.Ops.Range.f_end = mk_usize 16 }
                    <: Core_models.Ops.Range.t_Range usize) ]
        == Seq.slice ${array} 8 16);
lemma_repr_of_two_loads ${array}
"#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_pre_N 12 (repr ${v})"#))]
#[hax_lib::ensures(|result| fstar!(r#"Libcrux_ml_kem.Vector.Traits.Spec.serialize_post_N 12 (repr ${v}) ${result}"#))]
pub(crate) fn serialize_12(v: SIMD128Vector) -> [u8; 24] {
    let low0 = _vreinterpretq_s32_s16(_vtrn1q_s16(v.low, v.low)); // a0, a0, a2, a2, a4, a4, a6, a6
    let low1 = _vreinterpretq_s32_s16(_vtrn2q_s16(v.low, v.low)); // a1, a1, a3, a3, a5, a5, a7, a7
    let mixt = _vsliq_n_s32::<12>(low0, low1); // a1a0, a3a2, a5a4, a7a6

    let low0 = _vreinterpretq_s64_s32(_vtrn1q_s32(mixt, mixt)); // a1a0, a1a0, a5a4, a5a4
    let low1 = _vreinterpretq_s64_s32(_vtrn2q_s32(mixt, mixt)); // a3a2, a3a2, a7a6, a7a6
    let low_mix = _vsliq_n_s64::<24>(low0, low1); // a3a2a1a0, a7a6a5a4

    let high0 = _vreinterpretq_s32_s16(_vtrn1q_s16(v.high, v.high)); // a0, a0, a2, a2, a4, a4, a6, a6
    let high1 = _vreinterpretq_s32_s16(_vtrn2q_s16(v.high, v.high)); // a1, a1, a3, a3, a5, a5, a7, a7
    let mixt = _vsliq_n_s32::<12>(high0, high1); // a1a0, a3a2, a5a4, a7a6

    let high0 = _vreinterpretq_s64_s32(_vtrn1q_s32(mixt, mixt)); // a1a0, a1a0, a5a4, a5a4
    let high1 = _vreinterpretq_s64_s32(_vtrn2q_s32(mixt, mixt)); // a3a2, a3a2, a7a6, a7a6
    let high_mix = _vsliq_n_s64::<24>(high0, high1); // a3a2a1a0, a7a6a5a4

    let mut result32 = [0u8; 32];
    _vst1q_u8(&mut result32[0..16], _vreinterpretq_u8_s64(low_mix));
    _vst1q_u8(&mut result32[16..32], _vreinterpretq_u8_s64(high_mix));
    let mut result = [0u8; 24];
    result[0..6].copy_from_slice(&result32[0..6]);
    result[6..12].copy_from_slice(&result32[8..14]);
    result[12..18].copy_from_slice(&result32[16..22]);
    result[18..24].copy_from_slice(&result32[24..30]);
    proof!(
        r#"
    let _:squash (forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 0 16) k == NI.get_lane_u8x16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_u8_s64 ${low_mix}) k) =
      introduce forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 0 16) k == NI.get_lane_u8x16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_u8_s64 ${low_mix}) k
      with ()
    in
    let _:squash (forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 16 32) k == NI.get_lane_u8x16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_u8_s64 ${high_mix}) k) =
      introduce forall (k: nat{k < 16}).
          Seq.index (Seq.slice ${result32} 16 32) k == NI.get_lane_u8x16 (Libcrux_intrinsics.Arm64.e_vreinterpretq_u8_s64 ${high_mix}) k
      with ()
    in
    lemma_store32_to_bytes ${low_mix} ${high_mix} ${result32};
    assert (Seq.slice ${result} 0 6 == Seq.slice ${result32} 0 6);
    assert (Seq.slice ${result} 6 12 == Seq.slice ${result32} 8 14);
    assert (Seq.slice ${result} 12 18 == Seq.slice ${result32} 16 22);
    assert (Seq.slice ${result} 18 24 == Seq.slice ${result32} 24 30);
    lemma_result_mapping ${low_mix} ${high_mix} 6 ${result32} ${result};
    reveal_opaque (`%half_spine) (half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low 12);
    reveal_opaque (`%half_mixt) (half_mixt ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low 12);
    reveal_opaque (`%half_spine) (half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high 12);
    reveal_opaque (`%half_mixt) (half_mixt ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high 12);
    assert (${low_mix} == half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_low 12);
    assert (${high_mix} == half_spine ${v}.Libcrux_ml_kem.Vector.Neon.Vector_type.f_high 12);
    lemma_spine_r2 ${v} ${low_mix} ${high_mix} 12;
    lemma_serialize_post ${v} ${low_mix} ${high_mix} 6 ${result}
    "#
    );
    result
}

#[inline(always)]
#[hax_lib::fstar::before(r#"#restart-solver"#)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::requires(v.len() == 24)]
#[hax_lib::ensures(|result| fstar!(r#"${spec::deserialize_12_post} ${v} (repr ${result})"#))]
pub(crate) fn deserialize_12(v: &[u8]) -> SIMD128Vector {
    let indexes: [u8; 16] = [0, 1, 1, 2, 3, 4, 4, 5, 6, 7, 7, 8, 9, 10, 10, 11];
    let index_vec = _vld1q_u8(&indexes);
    let shifts: [i16; 8] = [0, -4, 0, -4, 0, -4, 0, -4];
    let shift_vec = _vld1q_s16(&shifts);
    let mask12 = _vdupq_n_u16(0xfff);

    let mut input0 = [0u8; 16];
    input0[0..12].copy_from_slice(&v[0..12]);
    let input_vec0 = _vld1q_u8(&input0);

    let mut input1 = [0u8; 16];
    input1[0..12].copy_from_slice(&v[12..24]);
    let input_vec1 = _vld1q_u8(&input1);

    let moved0 = _vreinterpretq_u16_u8(_vqtbl1q_u8(input_vec0, index_vec));
    let shifted0 = _vshlq_u16(moved0, shift_vec);
    let low = _vreinterpretq_s16_u16(_vandq_u16(shifted0, mask12));

    let moved1 = _vreinterpretq_u16_u8(_vqtbl1q_u8(input_vec1, index_vec));
    let shifted1 = _vshlq_u16(moved1, shift_vec);
    let high = _vreinterpretq_s16_u16(_vandq_u16(shifted1, mask12));

    let result = SIMD128Vector { low, high };
    proof!(
        r#"(* mask lanes *)
assert (forall (i: nat{i < 8}). NI.get_lane_u16x8 ${mask12} i == mk_u16 4095);
(* ${index_vec} lanes + concrete index values *)
assert (forall (i: nat{i < 16}). NI.get_lane_u8x16 ${index_vec} i == Seq.index ${indexes} i);
lemma_deser12_index_lanes ${index_vec} ${indexes};
(* shift lanes + concrete shift values *)
assert (forall (i: nat{i < 8}). NI.get_lane_i16x8 ${shift_vec} i == Seq.index ${shifts} i);
lemma_deser12_shift_lanes ${shift_vec} ${shifts};
(* input0 / input1 vs ${v} *)
assert ((${v}.[ { Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = mk_usize 12 } <: Core_models.Ops.Range.t_Range usize ]) == Seq.slice ${v} 0 12);
assert ((${v}.[ { Core_models.Ops.Range.f_start = mk_usize 12; Core_models.Ops.Range.f_end = mk_usize 24 } <: Core_models.Ops.Range.t_Range usize ]) == Seq.slice ${v} 12 24);
Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range #u8
  (Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 16))
  ({ Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = mk_usize 12 } <: Core_models.Ops.Range.t_Range usize)
  (Seq.slice ${v} 0 12);
Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range #u8
  (Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 16))
  ({ Core_models.Ops.Range.f_start = mk_usize 0; Core_models.Ops.Range.f_end = mk_usize 12 } <: Core_models.Ops.Range.t_Range usize)
  (Seq.slice ${v} 12 24);
assert (forall (i: nat{i < 12}). NI.get_lane_u8x16 ${input_vec0} i == Seq.index ${v} i);
assert (forall (i: nat{i < 12}). NI.get_lane_u8x16 ${input_vec1} i == Seq.index ${v} (12 + i));
lemma_deser12_post ${v} ${result} ${index_vec} ${input_vec0} ${input_vec1} ${shift_vec} ${mask12}"#
    );
    result
}

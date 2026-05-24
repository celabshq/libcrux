//! This file does not contain correct function signatures!
//! Replace with a hand-written file after extraction.

#![allow(unused_variables, non_camel_case_types, dead_code)]

#[cfg(hax)]
use hax_lib::prop::*;

#[cfg(hax)]
#[derive(Clone, Copy, Debug)]
#[hax_lib::fstar::replace(
    interface,
    r#"
unfold type $:{Vec256} = bit_vec 256
val vec256_as_i16x16 (x: bit_vec 256) : t_Array i16 (sz 16)
let get_lane (v: bit_vec 256) (i:nat{i < 16}) = Seq.index (vec256_as_i16x16 v) i
val vec256_as_u64x4 (x: bit_vec 256) : t_Array u64 (sz 4)
let get_lane_u64x4 (v: bit_vec 256) (i: nat{i < 4}) : u64 =
  Seq.index (vec256_as_u64x4 v) i

(** Bridge admit: relates the [b]-th bit of the [lane]-th u64 lane to
    the corresponding bit of the underlying 256-bit vector.  This is
    the only "trust" axiom relating [get_lane_u64x4] (defined via the
    opaque [vec256_as_u64x4]) to the bit-level form.  All six
    [lemma_mm256_*_u64x4] discharges below derive from this bridge
    plus the per-bit operator semantics
    ([get_bit_and]/[get_bit_or]/[get_bit_xor]/[get_bit_cast]) and
    [Rust_primitives.Integers.lemma_int_t_eq_via_bits]. *)
val lemma_get_lane_u64x4_bit
      (vec: bit_vec 256) (lane: nat{lane < 4})
      (b: Rust_primitives.Integers.usize {Rust_primitives.Integers.v b < 64})
  : Lemma (Rust_primitives.Integers.get_bit (get_lane_u64x4 vec lane) b
           == vec (64 * lane + Rust_primitives.Integers.v b))
        [SMTPat (Rust_primitives.Integers.get_bit (get_lane_u64x4 vec lane) b)]
"#
)]
pub struct Vec256(u8);

#[cfg(hax)]
#[derive(Copy, Clone, Debug)]
#[hax_lib::fstar::replace(
    interface,
    r#"
unfold type $:{Vec128} = bit_vec 128
val vec128_as_i16x8 (x: bit_vec 128) : t_Array i16 (sz 8)
let get_lane128 (v: bit_vec 128) (i:nat{i < 8}) = Seq.index (vec128_as_i16x8 v) i
"#
)]
pub struct Vec128(u8);

#[cfg(not(hax))]
pub type Vec256 = u8;
#[cfg(not(hax))]
pub type Vec128 = u8;
pub type Vec256Float = u8;

// NB: the equality with `get_lane_u64x4` is exposed via a separate
// SMTPat lemma instead of an ensures-refinement on the return type.
// The refinement-interpretation of `Pure u64 (ensures result == ...)`
// fires on every value of the refined type and was triggering a
// quantifier cascade in load_block proofs (~1M instantiations).
// SMTPat-trigger fires only when `get_lane_u64 vec lane` actually
// appears in the goal — controlled instantiation, no cascade.
// Trust footprint unchanged: the lemma body is `admit ()` because
// `get_lane_u64` is `unimplemented!()` (axiomatic), same as before.
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::requires(lane < 4)]
#[hax_lib::fstar::after(
    interface,
    r#"
val get_lane_u64_post (vec: t_Vec256) (lane: usize{v lane < 4})
  : Lemma (get_lane_u64 vec lane == get_lane_u64x4 vec (v lane))
    [SMTPat (get_lane_u64 vec lane)]
"#
)]
#[hax_lib::fstar::after(
    r#"
let get_lane_u64_post (vec: t_Vec256) (lane: usize{v lane < 4})
  : Lemma (get_lane_u64 vec lane == get_lane_u64x4 vec (v lane))
    [SMTPat (get_lane_u64 vec lane)]
  = admit ()
"#
)]
pub fn get_lane_u64(vec: Vec256, lane: usize) -> u64 {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::requires(output.len() == 32)]
#[hax_lib::ensures(|()| (future(output).len() == output.len()).to_prop()
    & hax_lib::forall(|i: usize|
        if i < 4 {
            u64::from_le_bytes(future(output)[i*8..i*8+8].try_into().unwrap())
              == get_lane_u64(vector, i)
        } else { true }))]
#[hax_lib::fstar::after(
    r#"
let lemma_mm256_storeu_si256_u8_byte (output: t_Slice u8) (vector: t_Vec256) (k: nat)
  : Lemma
      (requires
        Seq.length output == 32 /\ k < 32)
      (ensures
        Seq.index (mm256_storeu_si256_u8 output vector <: t_Slice u8) k ==
        Seq.index
          (Core_models.Num.impl_u64__to_le_bytes (get_lane_u64 vector (mk_usize (k / 8))))
          (k % 8))
      [SMTPat (Seq.index (mm256_storeu_si256_u8 output vector <: t_Slice u8) k)]
  = admit ()
"#
)]
pub fn mm256_storeu_si256_u8(output: &mut [u8], vector: Vec256) {
    debug_assert_eq!(output.len(), 32);
    unimplemented!()
}

#[hax_lib::ensures(|()| future(output).len() == output.len())]
#[inline(always)]
pub fn mm256_storeu_si256_i16(output: &mut [i16], vector: Vec256) {
    debug_assert_eq!(output.len(), 16);
    unimplemented!()
}

#[inline(always)]
pub fn mm256_storeu_si256_i32(output: &mut [i32], vector: Vec256) {
    debug_assert_eq!(output.len(), 8);
    unimplemented!()
}

#[inline(always)]
#[hax_lib::requires(output.len() >= 8)]
#[hax_lib::ensures(|_| future(output).len() == output.len())]
pub fn mm_storeu_si128(output: &mut [i16], vector: Vec128) {
    debug_assert!(output.len() >= 8);
    unimplemented!()
}

#[inline(always)]
pub fn mm_storeu_si128_i32(output: &mut [i32], vector: Vec128) {
    debug_assert_eq!(output.len(), 4);
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm_storeu_bytes_si128}")]
#[inline(always)]
pub fn mm_storeu_bytes_si128(output: &mut [u8], vector: Vec128) {
    debug_assert_eq!(output.len(), 16);
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm_loadu_si128}")]
#[inline(always)]
pub fn mm_loadu_si128(input: &[u8]) -> Vec128 {
    debug_assert_eq!(input.len(), 16);
    unimplemented!()
}

#[inline(always)]
#[hax_lib::requires(input.len() == 32)]
#[hax_lib::ensures(|result| hax_lib::forall(|i: usize|
    if i < 4 {
        get_lane_u64(result, i)
          == u64::from_le_bytes(input[i*8..i*8+8].try_into().unwrap())
    } else { true }))]
pub fn mm256_loadu_si256_u8(input: &[u8]) -> Vec256 {
    debug_assert_eq!(input.len(), 32);
    unimplemented!()
}

#[inline(always)]
pub fn mm256_loadu_si256_i16(input: &[i16]) -> Vec256 {
    debug_assert_eq!(input.len(), 16);
    unimplemented!()
}

#[inline(always)]
pub fn mm256_loadu_si256_i32(input: &[i32]) -> Vec256 {
    debug_assert_eq!(input.len(), 8);
    unimplemented!()
}

#[inline(always)]
#[hax_lib::ensures(|result| fstar!("vec256_as_i16x16 $result == Seq.create 16 (mk_i16 0)"))]
pub fn mm256_setzero_si256() -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_set_m128i(hi: Vec128, lo: Vec128) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm_set_epi8}")]
#[inline(always)]
pub fn mm_set_epi8(
    byte15: i8,
    byte14: i8,
    byte13: i8,
    byte12: i8,
    byte11: i8,
    byte10: i8,
    byte9: i8,
    byte8: i8,
    byte7: i8,
    byte6: i8,
    byte5: i8,
    byte4: i8,
    byte3: i8,
    byte2: i8,
    byte1: i8,
    byte0: i8,
) -> Vec128 {
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm256_set_epi8}")]
#[inline(always)]
pub fn mm256_set_epi8(
    byte31: i8,
    byte30: i8,
    byte29: i8,
    byte28: i8,
    byte27: i8,
    byte26: i8,
    byte25: i8,
    byte24: i8,
    byte23: i8,
    byte22: i8,
    byte21: i8,
    byte20: i8,
    byte19: i8,
    byte18: i8,
    byte17: i8,
    byte16: i8,
    byte15: i8,
    byte14: i8,
    byte13: i8,
    byte12: i8,
    byte11: i8,
    byte10: i8,
    byte9: i8,
    byte8: i8,
    byte7: i8,
    byte6: i8,
    byte5: i8,
    byte4: i8,
    byte3: i8,
    byte2: i8,
    byte1: i8,
    byte0: i8,
) -> Vec256 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec256_as_i16x16 $result == 
                                    Spec.Utils.create (sz 16) $constant"))]
#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_set1_epi16 as ${mm256_set1_epi16}}
val lemma_mm256_set1_epi16 constant
  : Lemma (   vec256_as_i16x16 (mm256_set1_epi16 constant)
           == Spec.Utils.create (sz 16) constant
          )
          [SMTPat (vec256_as_i16x16 (mm256_set1_epi16 constant))]
"#
)]
#[inline(always)]
pub fn mm256_set1_epi16(constant: i16) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_set_epi16 as ${mm256_set_epi16}}
let lemma_mm256_set_epi16 v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0 :
    Lemma (vec256_as_i16x16 (${mm256_set_epi16} v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0) == 
            Spec.Utils.create16 v0 v1 v2 v3 v4 v5 v6 v7 v8 v9 v10 v11 v12 v13 v14 v15)
            [SMTPat (vec256_as_i16x16 (${mm256_set_epi16} v15 v14 v13 v12 v11 v10 v9 v8 v7 v6 v5 v4 v3 v2 v1 v0))] = admit()
"#
)]
pub fn mm256_set_epi16(
    input15: i16,
    input14: i16,
    input13: i16,
    input12: i16,
    input11: i16,
    input10: i16,
    input9: i16,
    input8: i16,
    input7: i16,
    input6: i16,
    input5: i16,
    input4: i16,
    input3: i16,
    input2: i16,
    input1: i16,
    input0: i16,
) -> Vec256 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec128_as_i16x8 $result == 
                                    Spec.Utils.create (sz 8) $constant"))]
#[inline(always)]
pub fn mm_set1_epi16(constant: i16) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_set1_epi32(constant: i32) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_set_epi32(input3: i32, input2: i32, input1: i32, input0: i32) -> Vec128 {
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm256_set_epi32}")]
#[inline(always)]
pub fn mm256_set_epi32(
    input7: i32,
    input6: i32,
    input5: i32,
    input4: i32,
    input3: i32,
    input2: i32,
    input1: i32,
    input0: i32,
) -> Vec256 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec128_as_i16x8 $result == 
            Spec.Utils.map2 (+.) (vec128_as_i16x8 $lhs) (vec128_as_i16x8 $rhs)"))]
#[inline(always)]
pub fn mm_add_epi16(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec128_as_i16x8 $result == 
            Spec.Utils.map2 (-.) (vec128_as_i16x8 $lhs) (vec128_as_i16x8 $rhs)"))]
#[inline(always)]
pub fn mm_sub_epi16(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec256_as_i16x16 $result == 
            Spec.Utils.map2 (+.) (vec256_as_i16x16 $lhs) (vec256_as_i16x16 $rhs)"))]
#[inline(always)]
pub fn mm256_add_epi16(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    "include BitVec.Intrinsics {mm256_madd_epi16 as ${mm256_madd_epi16}}"
)]
#[inline(always)]
pub fn mm256_madd_epi16(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}
#[inline(always)]
pub fn mm256_add_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec256_as_i16x16 $result == 
            Spec.Utils.map2 (-.) (vec256_as_i16x16 $lhs) (vec256_as_i16x16 $rhs)"))]
pub fn mm256_sub_epi16(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_add_epi64(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_abs_epi32(a: Vec256) -> Vec256 {
    unimplemented!()
}

pub fn mm256_sub_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_mullo_epi16 as ${mm256_mullo_epi16}}
let lemma_mm256_mullo_epi16 v1 v2 :
   Lemma (vec256_as_i16x16 (${mm256_mullo_epi16} v1 v2) == 
       Spec.Utils.map2 mul_mod (vec256_as_i16x16 v1) (vec256_as_i16x16 v2))
       [SMTPat (vec256_as_i16x16 (${mm256_mullo_epi16} v1 v2))] = admit()
"#
)]
pub fn mm256_mullo_epi16(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec128_as_i16x8 $result == 
            Spec.Utils.map2 mul_mod (vec128_as_i16x8 $lhs) (vec128_as_i16x8 $rhs)"))]
pub fn mm_mullo_epi16(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::ensures(|result| fstar!(r#"forall i. i % 16 >= 1 ==> result i == 0"#))]
pub fn mm256_cmpgt_epi16(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}
#[inline(always)]
pub fn mm256_cmpgt_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}
#[inline(always)]
pub fn mm256_cmpeq_epi32(a: Vec256, b: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_sign_epi32(a: Vec256, b: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_castsi256_ps(a: Vec256) -> Vec256Float {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_movemask_ps(a: Vec256Float) -> i32 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec128_as_i16x8 $result == 
            Spec.Utils.map2 (fun x y -> cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 16)) <: i16) 
                (vec128_as_i16x8 $lhs) (vec128_as_i16x8 $rhs)"))]
pub fn mm_mulhi_epi16(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}

pub fn mm256_mullo_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::ensures(|result| fstar!("vec256_as_i16x16 $result == 
            Spec.Utils.map2 (fun x y -> cast (((cast x <: i32) *. (cast y <: i32)) >>! (mk_i32 16)) <: i16) (vec256_as_i16x16 $lhs) (vec256_as_i16x16 $rhs)"))]
pub fn mm256_mulhi_epi16(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

pub fn mm256_mul_epu32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_mul_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_and_si256 as ${mm256_and_si256}}
val lemma_mm256_and_si256 lhs rhs
  : Lemma (   vec256_as_i16x16 (mm256_and_si256 lhs rhs)
           == Spec.Utils.map2 (&.) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs)
          )
          [SMTPat (vec256_as_i16x16 (mm256_and_si256 lhs rhs))]
"#
)]
#[inline(always)]
pub fn mm256_and_si256(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_or_si256 as mm256_or_si256}
let lemma_mm256_or_si256_u64x4 (a b: t_Vec256)
  : Lemma (forall (i: nat{i < 4}).
             get_lane_u64x4 (mm256_or_si256 a b) i ==
             (get_lane_u64x4 a i |. get_lane_u64x4 b i))
        [SMTPat (mm256_or_si256 a b)]
  = let aux (i: nat{i < 4})
      : Lemma (get_lane_u64x4 (mm256_or_si256 a b) i ==
               (get_lane_u64x4 a i |. get_lane_u64x4 b i)) =
      Rust_primitives.Integers.lemma_int_t_eq_via_bits
        (get_lane_u64x4 (mm256_or_si256 a b) i)
        (get_lane_u64x4 a i |. get_lane_u64x4 b i)
    in FStar.Classical.forall_intro aux
"#
)]
#[inline(always)]
pub fn mm256_or_si256(a: Vec256, b: Vec256) -> Vec256 {
    unimplemented!()
}

pub fn mm256_testz_si256(lhs: Vec256, rhs: Vec256) -> i32 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_xor_si256 as mm256_xor_si256}
let lemma_mm256_xor_si256_u64x4 (lhs rhs: t_Vec256)
  : Lemma (forall (i: nat{i < 4}).
             get_lane_u64x4 (mm256_xor_si256 lhs rhs) i ==
             (get_lane_u64x4 lhs i ^. get_lane_u64x4 rhs i))
        [SMTPat (mm256_xor_si256 lhs rhs)]
  = let aux (i: nat{i < 4})
      : Lemma (get_lane_u64x4 (mm256_xor_si256 lhs rhs) i ==
               (get_lane_u64x4 lhs i ^. get_lane_u64x4 rhs i)) =
      Rust_primitives.Integers.lemma_int_t_eq_via_bits
        (get_lane_u64x4 (mm256_xor_si256 lhs rhs) i)
        (get_lane_u64x4 lhs i ^. get_lane_u64x4 rhs i)
    in FStar.Classical.forall_intro aux
"#
)]
pub fn mm256_xor_si256(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::requires(SHIFT_BY >= 0 && SHIFT_BY < 16)]
#[hax_lib::ensures(|result| fstar!("vec256_as_i16x16 $result == 
            Spec.Utils.map_array (fun x -> x >>! ${SHIFT_BY}) (vec256_as_i16x16 $vector)"))]
pub fn mm256_srai_epi16<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 16);
    unimplemented!()
}
pub fn mm256_srai_epi32<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 32);
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    "include BitVec.Intrinsics {mm256_srli_epi16 as ${mm256_srli_epi16::<0>}}"
)]
pub fn mm256_srli_epi16<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 16);
    unimplemented!()
}
pub fn mm256_srli_epi32<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 32);
    unimplemented!()
}

pub fn mm_srli_epi64<const SHIFT_BY: i32>(vector: Vec128) -> Vec128 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 64);
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_srli_epi64 as ${mm256_srli_epi64::<0>}}
let lemma_mm256_srli_epi64_u64x4 (v_SHIFT_BY: i32) (vector: t_Vec256)
  : Lemma
      (requires v v_SHIFT_BY >= 0 /\ v v_SHIFT_BY < 64)
      (ensures
        forall (i: nat{i < 4}).
          get_lane_u64x4 (mm256_srli_epi64 v_SHIFT_BY vector) i ==
          (get_lane_u64x4 vector i >>! v_SHIFT_BY))
        [SMTPat (mm256_srli_epi64 v_SHIFT_BY vector)]
  = let aux (i: nat{i < 4})
      : Lemma (get_lane_u64x4 (mm256_srli_epi64 v_SHIFT_BY vector) i ==
               (get_lane_u64x4 vector i >>! v_SHIFT_BY)) =
      Rust_primitives.Integers.lemma_int_t_eq_via_bits
        (get_lane_u64x4 (mm256_srli_epi64 v_SHIFT_BY vector) i)
        (get_lane_u64x4 vector i >>! v_SHIFT_BY)
    in FStar.Classical.forall_intro aux
"#
)]
pub fn mm256_srli_epi64<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 64);
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    "include BitVec.Intrinsics {mm256_slli_epi16 as ${mm256_slli_epi16::<0>}}"
)]
pub fn mm256_slli_epi16<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 16);
    unimplemented!()
}

pub fn mm256_slli_epi32<const SHIFT_BY: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY >= 0 && SHIFT_BY < 32);
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm_shuffle_epi8}")]
pub fn mm_shuffle_epi8(vector: Vec128, control: Vec128) -> Vec128 {
    unimplemented!()
}
#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm256_shuffle_epi8}")]
pub fn mm256_shuffle_epi8(vector: Vec256, control: Vec256) -> Vec256 {
    unimplemented!()
}
pub fn mm256_shuffle_epi32<const CONTROL: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(CONTROL >= 0 && CONTROL < 256);
    unimplemented!()
}

pub fn mm256_permute4x64_epi64<const CONTROL: i32>(vector: Vec256) -> Vec256 {
    debug_assert!(CONTROL >= 0 && CONTROL < 256);
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_unpackhi_epi64 as mm256_unpackhi_epi64}
let lemma_mm256_unpackhi_epi64_u64x4 (lhs rhs: t_Vec256)
  : Lemma (
      get_lane_u64x4 (mm256_unpackhi_epi64 lhs rhs) 0 == get_lane_u64x4 lhs 1 /\
      get_lane_u64x4 (mm256_unpackhi_epi64 lhs rhs) 1 == get_lane_u64x4 rhs 1 /\
      get_lane_u64x4 (mm256_unpackhi_epi64 lhs rhs) 2 == get_lane_u64x4 lhs 3 /\
      get_lane_u64x4 (mm256_unpackhi_epi64 lhs rhs) 3 == get_lane_u64x4 rhs 3)
    [SMTPat (mm256_unpackhi_epi64 lhs rhs)]
  = let r = mm256_unpackhi_epi64 lhs rhs in
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 0) (get_lane_u64x4 lhs 1);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 1) (get_lane_u64x4 rhs 1);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 2) (get_lane_u64x4 lhs 3);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 3) (get_lane_u64x4 rhs 3)
"#
)]
pub fn mm256_unpackhi_epi64(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

pub fn mm256_unpacklo_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

pub fn mm256_unpackhi_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    "include BitVec.Intrinsics {mm256_castsi256_si128 as ${mm256_castsi256_si128}}"
)]
pub fn mm256_castsi256_si128(vector: Vec256) -> Vec128 {
    unimplemented!()
}
pub fn mm256_castsi128_si256(vector: Vec128) -> Vec256 {
    unimplemented!()
}

pub fn mm256_cvtepi16_epi32(vector: Vec128) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    "include BitVec.Intrinsics {mm_packs_epi16 as ${mm_packs_epi16}}"
)]
pub fn mm_packs_epi16(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}
pub fn mm256_packs_epi32(lhs: Vec256, rhs: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    "include BitVec.Intrinsics {mm256_extracti128_si256 as ${mm256_extracti128_si256::<0>}}"
)]
pub fn mm256_extracti128_si256<const CONTROL: i32>(vector: Vec256) -> Vec128 {
    debug_assert!(CONTROL == 0 || CONTROL == 1);
    unimplemented!()
}

pub fn mm256_inserti128_si256<const CONTROL: i32>(vector: Vec256, vector_i128: Vec128) -> Vec256 {
    debug_assert!(CONTROL == 0 || CONTROL == 1);
    unimplemented!()
}

#[inline(always)]
pub fn mm256_blend_epi16<const CONTROL: i32>(lhs: Vec256, rhs: Vec256) -> Vec256 {
    debug_assert!(CONTROL >= 0 && CONTROL < 256);
    unimplemented!()
}

#[inline(always)]
pub fn mm256_blend_epi32<const CONTROL: i32>(lhs: Vec256, rhs: Vec256) -> Vec256 {
    debug_assert!(CONTROL >= 0 && CONTROL < 256);
    unimplemented!()
}

// This is essentially _mm256_blendv_ps adapted for use with the Vec256 type.
// It is not offered by the AVX2 instruction set.
#[inline(always)]
pub fn vec256_blendv_epi32(a: Vec256, b: Vec256, mask: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    "include BitVec.Intrinsics {mm_movemask_epi8 as ${mm_movemask_epi8}}"
)]
#[inline(always)]
pub fn mm_movemask_epi8(vector: Vec128) -> i32 {
    unimplemented!()
}

#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm256_permutevar8x32_epi32}")]
#[inline(always)]
pub fn mm256_permutevar8x32_epi32(vector: Vec256, control: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_srlv_epi32(vector: Vec256, counts: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_srlv_epi64(vector: Vec256, counts: Vec256) -> Vec256 {
    unimplemented!()
}

pub fn mm_sllv_epi32(vector: Vec128, counts: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::fstar::replace(interface, "include BitVec.Intrinsics {mm256_sllv_epi32}")]
pub fn mm256_sllv_epi32(vector: Vec256, counts: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_slli_epi64 as ${mm256_slli_epi64::<0>}}
let lemma_mm256_slli_epi64_u64x4 (v_LEFT: i32) (x: t_Vec256)
  : Lemma
      (requires v v_LEFT >= 0 /\ v v_LEFT < 64)
      (ensures
        forall (i: nat{i < 4}).
          get_lane_u64x4 (mm256_slli_epi64 v_LEFT x) i ==
          (get_lane_u64x4 x i <<! v_LEFT))
        [SMTPat (mm256_slli_epi64 v_LEFT x)]
  = let aux (i: nat{i < 4})
      : Lemma (get_lane_u64x4 (mm256_slli_epi64 v_LEFT x) i ==
               (get_lane_u64x4 x i <<! v_LEFT)) =
      Rust_primitives.Integers.lemma_int_t_eq_via_bits
        (get_lane_u64x4 (mm256_slli_epi64 v_LEFT x) i)
        (get_lane_u64x4 x i <<! v_LEFT)
    in FStar.Classical.forall_intro aux
"#
)]
#[hax_lib::requires(LEFT >= 0 && LEFT <= 64)]
#[inline(always)]
pub fn mm256_slli_epi64<const LEFT: i32>(x: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm256_bsrli_epi128<const SHIFT_BY: i32>(x: Vec256) -> Vec256 {
    debug_assert!(SHIFT_BY > 0 && SHIFT_BY < 16);
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_andnot_si256 as mm256_andnot_si256}
let lemma_mm256_andnot_si256_u64x4 (a b: t_Vec256)
  : Lemma (forall (i: nat{i < 4}).
             get_lane_u64x4 (mm256_andnot_si256 a b) i ==
             (get_lane_u64x4 b i &. (~. (get_lane_u64x4 a i))))
        [SMTPat (mm256_andnot_si256 a b)]
  = let aux (i: nat{i < 4})
      : Lemma (get_lane_u64x4 (mm256_andnot_si256 a b) i ==
               (get_lane_u64x4 b i &. (~. (get_lane_u64x4 a i)))) =
      Rust_primitives.Integers.lemma_int_t_eq_via_bits
        (get_lane_u64x4 (mm256_andnot_si256 a b) i)
        (get_lane_u64x4 b i &. (~. (get_lane_u64x4 a i)))
    in FStar.Classical.forall_intro aux
"#
)]
#[inline(always)]
pub fn mm256_andnot_si256(a: Vec256, b: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_set1_epi64x as mm256_set1_epi64x}
let lemma_mm256_set1_epi64x_u64x4 (a: i64)
  : Lemma (forall (i: nat{i < 4}).
             get_lane_u64x4 (mm256_set1_epi64x a) i == (cast_mod #i64_inttype #u64_inttype a))
        [SMTPat (mm256_set1_epi64x a)]
  = let aux (i: nat{i < 4})
      : Lemma (get_lane_u64x4 (mm256_set1_epi64x a) i ==
               (cast_mod #i64_inttype #u64_inttype a)) =
      Rust_primitives.Integers.lemma_int_t_eq_via_bits
        (get_lane_u64x4 (mm256_set1_epi64x a) i)
        (cast_mod #i64_inttype #u64_inttype a)
    in FStar.Classical.forall_intro aux
"#
)]
#[inline(always)]
pub fn mm256_set1_epi64x(a: i64) -> Vec256 {
    unimplemented!()
}
#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_set_epi64x as mm256_set_epi64x}
let lemma_mm256_set_epi64x_u64x4 (input3 input2 input1 input0: i64)
  : Lemma (
      get_lane_u64x4 (mm256_set_epi64x input3 input2 input1 input0) 0 == cast_mod #i64_inttype #u64_inttype input0 /\
      get_lane_u64x4 (mm256_set_epi64x input3 input2 input1 input0) 1 == cast_mod #i64_inttype #u64_inttype input1 /\
      get_lane_u64x4 (mm256_set_epi64x input3 input2 input1 input0) 2 == cast_mod #i64_inttype #u64_inttype input2 /\
      get_lane_u64x4 (mm256_set_epi64x input3 input2 input1 input0) 3 == cast_mod #i64_inttype #u64_inttype input3)
    [SMTPat (mm256_set_epi64x input3 input2 input1 input0)]
  = let r = mm256_set_epi64x input3 input2 input1 input0 in
    Rust_primitives.Integers.lemma_int_t_eq_via_bits
      (get_lane_u64x4 r 0) (cast_mod #i64_inttype #u64_inttype input0);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits
      (get_lane_u64x4 r 1) (cast_mod #i64_inttype #u64_inttype input1);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits
      (get_lane_u64x4 r 2) (cast_mod #i64_inttype #u64_inttype input2);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits
      (get_lane_u64x4 r 3) (cast_mod #i64_inttype #u64_inttype input3)
"#
)]
#[inline(always)]
pub fn mm256_set_epi64x(input3: i64, input2: i64, input1: i64, input0: i64) -> Vec256 {
    unimplemented!()
}

#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_unpacklo_epi64 as mm256_unpacklo_epi64}
let lemma_mm256_unpacklo_epi64_u64x4 (a b: t_Vec256)
  : Lemma (
      get_lane_u64x4 (mm256_unpacklo_epi64 a b) 0 == get_lane_u64x4 a 0 /\
      get_lane_u64x4 (mm256_unpacklo_epi64 a b) 1 == get_lane_u64x4 b 0 /\
      get_lane_u64x4 (mm256_unpacklo_epi64 a b) 2 == get_lane_u64x4 a 2 /\
      get_lane_u64x4 (mm256_unpacklo_epi64 a b) 3 == get_lane_u64x4 b 2)
    [SMTPat (mm256_unpacklo_epi64 a b)]
  = let r = mm256_unpacklo_epi64 a b in
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 0) (get_lane_u64x4 a 0);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 1) (get_lane_u64x4 b 0);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 2) (get_lane_u64x4 a 2);
    Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 3) (get_lane_u64x4 b 2)
"#
)]
#[inline(always)]
pub fn mm256_unpacklo_epi64(a: Vec256, b: Vec256) -> Vec256 {
    unimplemented!()
}

#[hax_lib::requires(IMM8 == 0x20 || IMM8 == 0x31)]
#[hax_lib::fstar::replace(
    interface,
    r#"
include BitVec.Intrinsics {mm256_permute2x128_si256 as ${mm256_permute2x128_si256::<0>}}
let lemma_mm256_permute2x128_si256_u64x4 (v_IMM8: i32) (a b: t_Vec256)
  : Lemma
      (requires v v_IMM8 == 0x20 \/ v v_IMM8 == 0x31)
      (ensures
        (v v_IMM8 == 0x20 ==>
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 0 == get_lane_u64x4 a 0 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 1 == get_lane_u64x4 a 1 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 2 == get_lane_u64x4 b 0 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 3 == get_lane_u64x4 b 1) /\
        (v v_IMM8 == 0x31 ==>
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 0 == get_lane_u64x4 a 2 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 1 == get_lane_u64x4 a 3 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 2 == get_lane_u64x4 b 2 /\
          get_lane_u64x4 (mm256_permute2x128_si256 v_IMM8 a b) 3 == get_lane_u64x4 b 3))
    [SMTPat (mm256_permute2x128_si256 v_IMM8 a b)]
  = let r = mm256_permute2x128_si256 v_IMM8 a b in
    if v v_IMM8 = 0x20 then begin
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 0) (get_lane_u64x4 a 0);
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 1) (get_lane_u64x4 a 1);
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 2) (get_lane_u64x4 b 0);
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 3) (get_lane_u64x4 b 1)
    end else begin
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 0) (get_lane_u64x4 a 2);
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 1) (get_lane_u64x4 a 3);
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 2) (get_lane_u64x4 b 2);
      Rust_primitives.Integers.lemma_int_t_eq_via_bits (get_lane_u64x4 r 3) (get_lane_u64x4 b 3)
    end
"#
)]
#[inline(always)]
pub fn mm256_permute2x128_si256<const IMM8: i32>(a: Vec256, b: Vec256) -> Vec256 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_clmulepi64_si128<const IMM8: i32>(a: Vec128, b: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_aesenc_si128(a: Vec128, b: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_aesenclast_si128(a: Vec128, b: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_aeskeygenassist_si128<const RCON: i32>(a: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_slli_si128<const SHIFT_BY: i32>(vector: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_srli_si128<const SHIFT_BY: i32>(vector: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_unpackhi_epi64(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_unpacklo_epi64(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}
#[inline(always)]
pub fn mm_xor_si128(lhs: Vec128, rhs: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_setzero_si128() -> Vec128 {
    unimplemented!()
}
#[inline(always)]
pub fn mm_shuffle_epi32<const CONTROL: i32>(vector: Vec128) -> Vec128 {
    unimplemented!()
}

#[inline(always)]
pub fn mm_storeu_si128_u8(output: &mut [u8], vector: Vec128) {
    unimplemented!()
}

#[inline(always)]
pub fn mm_loadu_si128_u128(input: &u128) -> Vec128 {
    unimplemented!()
}
#[inline(always)]
pub fn mm_storeu_si128_u128(output: &mut u128, vector: Vec128) {
    unimplemented!()
}

//! Extraction-only stubs for ARM64 NEON intrinsic types and functions.
//!
//! Each NEON vector type is defined as a single-field struct with
//! `#[hax_lib::lean::replace(...)]` so the Lean backend directly emits the
//! correct `BitVec` width, and `#[hax_lib::fstar::replace(...)]` for F*.

#![allow(non_camel_case_types, unsafe_code, unused_variables)]

#[derive(Clone, Copy)]
#[hax_lib::lean::replace("abbrev _uint16x4_t := BitVec 64")]
#[hax_lib::fstar::replace(
    interface,
    r#"
unfold type $:{_uint16x4_t} = bit_vec 64
val vec64_as_u16x4 (x: $:{_uint16x4_t}) : t_Array u16 (sz 4)
let get_lane_u16x4 (v: $:{_uint16x4_t}) (i: nat{i < 4}) : u16 =
  Seq.index (vec64_as_u16x4 v) i
"#
)]
#[hax_lib::fstar::replace(
    r#"
assume val vec64_as_u16x4_axiom (x: $:{_uint16x4_t}) : t_Array u16 (sz 4)
let vec64_as_u16x4 = vec64_as_u16x4_axiom
"#
)]
pub struct _uint16x4_t(u8);

#[derive(Clone, Copy)]
#[hax_lib::lean::replace("abbrev _int16x4_t := BitVec 64")]
#[hax_lib::fstar::replace(
    interface,
    r#"
unfold type $:{_int16x4_t} = bit_vec 64
val vec64_as_i16x4 (x: $:{_int16x4_t}) : t_Array i16 (sz 4)
let get_lane_i16x4 (v: $:{_int16x4_t}) (i: nat{i < 4}) : i16 =
  Seq.index (vec64_as_i16x4 v) i
"#
)]
#[hax_lib::fstar::replace(
    r#"
assume val vec64_as_i16x4_axiom (x: $:{_int16x4_t}) : t_Array i16 (sz 4)
let vec64_as_i16x4 = vec64_as_i16x4_axiom
"#
)]
pub struct _int16x4_t(u8);

#[derive(Clone, Copy)]
#[hax_lib::lean::replace("abbrev _int16x8_t := BitVec 128")]
#[hax_lib::fstar::replace(
    interface,
    r#"
unfold type $:{_int16x8_t} = bit_vec 128
val vec128_as_i16x8 (x: $:{_int16x8_t}) : t_Array i16 (sz 8)
let get_lane_i16x8 (v: $:{_int16x8_t}) (i: nat{i < 8}) : i16 =
  Seq.index (vec128_as_i16x8 v) i
"#
)]
#[hax_lib::fstar::replace(
    r#"
assume val vec128_as_i16x8_axiom (x: $:{_int16x8_t}) : t_Array i16 (sz 8)
let vec128_as_i16x8 = vec128_as_i16x8_axiom
"#
)]
pub struct _int16x8_t(u8);

#[derive(Clone, Copy)]
#[hax_lib::lean::replace("abbrev _uint8x16_t := BitVec 128")]
#[hax_lib::fstar::replace(
    interface,
    r#"
unfold type $:{_uint8x16_t} = bit_vec 128
val vec128_as_u8x16 (x: $:{_uint8x16_t}) : t_Array u8 (sz 16)
let get_lane_u8x16 (v: $:{_uint8x16_t}) (i: nat{i < 16}) : u8 =
  Seq.index (vec128_as_u8x16 v) i
"#
)]
#[hax_lib::fstar::replace(
    r#"
assume val vec128_as_u8x16_axiom (x: $:{_uint8x16_t}) : t_Array u8 (sz 16)
let vec128_as_u8x16 = vec128_as_u8x16_axiom
"#
)]
pub struct _uint8x16_t(u8);

#[derive(Clone, Copy)]
#[hax_lib::lean::replace("abbrev _uint16x8_t := BitVec 128")]
#[hax_lib::fstar::replace(
    interface,
    r#"
unfold type $:{_uint16x8_t} = bit_vec 128
val vec128_as_u16x8 (x: $:{_uint16x8_t}) : t_Array u16 (sz 8)
let get_lane_u16x8 (v: $:{_uint16x8_t}) (i: nat{i < 8}) : u16 =
  Seq.index (vec128_as_u16x8 v) i
"#
)]
#[hax_lib::fstar::replace(
    r#"
assume val vec128_as_u16x8_axiom (x: $:{_uint16x8_t}) : t_Array u16 (sz 8)
let vec128_as_u16x8 = vec128_as_u16x8_axiom
"#
)]
pub struct _uint16x8_t(u8);

#[derive(Clone, Copy)]
#[hax_lib::lean::replace("abbrev _uint32x4_t := BitVec 128")]
#[hax_lib::fstar::replace(
    interface,
    r#"
unfold type $:{_uint32x4_t} = bit_vec 128
val vec128_as_u32x4 (x: $:{_uint32x4_t}) : t_Array u32 (sz 4)
let get_lane_u32x4 (v: $:{_uint32x4_t}) (i: nat{i < 4}) : u32 =
  Seq.index (vec128_as_u32x4 v) i
"#
)]
#[hax_lib::fstar::replace(
    r#"
assume val vec128_as_u32x4_axiom (x: $:{_uint32x4_t}) : t_Array u32 (sz 4)
let vec128_as_u32x4 = vec128_as_u32x4_axiom
"#
)]
pub struct _uint32x4_t(u8);

#[derive(Clone, Copy)]
#[hax_lib::lean::replace("abbrev _int32x4_t := BitVec 128")]
#[hax_lib::fstar::replace(
    interface,
    r#"
unfold type $:{_int32x4_t} = bit_vec 128
val vec128_as_i32x4 (x: $:{_int32x4_t}) : t_Array i32 (sz 4)
let get_lane_i32x4 (v: $:{_int32x4_t}) (i: nat{i < 4}) : i32 =
  Seq.index (vec128_as_i32x4 v) i
"#
)]
#[hax_lib::fstar::replace(
    r#"
assume val vec128_as_i32x4_axiom (x: $:{_int32x4_t}) : t_Array i32 (sz 4)
let vec128_as_i32x4 = vec128_as_i32x4_axiom
"#
)]
pub struct _int32x4_t(u8);

#[derive(Clone, Copy)]
#[hax_lib::lean::replace("abbrev _uint64x2_t := BitVec 128")]
#[hax_lib::fstar::replace(
    interface,
    r#"
unfold type $:{_uint64x2_t} = bit_vec 128
val vec128_as_u64x2 (x: $:{_uint64x2_t}) : t_Array u64 (sz 2)
let get_lane_u64x2 (v: $:{_uint64x2_t}) (i: nat{i < 2}) : u64 =
  Seq.index (vec128_as_u64x2 v) i
"#
)]
#[hax_lib::fstar::replace(
    r#"
assume val vec128_as_u64x2_axiom (x: $:{_uint64x2_t}) : t_Array u64 (sz 2)
let vec128_as_u64x2 = vec128_as_u64x2_axiom
"#
)]
pub struct _uint64x2_t(u8);

#[derive(Clone, Copy)]
#[hax_lib::lean::replace("abbrev _int64x2_t := BitVec 128")]
#[hax_lib::fstar::replace(
    interface,
    r#"
unfold type $:{_int64x2_t} = bit_vec 128
val vec128_as_i64x2 (x: $:{_int64x2_t}) : t_Array i64 (sz 2)
let get_lane_i64x2 (v: $:{_int64x2_t}) (i: nat{i < 2}) : i64 =
  Seq.index (vec128_as_i64x2 v) i
"#
)]
#[hax_lib::fstar::replace(
    r#"
assume val vec128_as_i64x2_axiom (x: $:{_int64x2_t}) : t_Array i64 (sz 2)
let vec128_as_i64x2 = vec128_as_i64x2_axiom
"#
)]
pub struct _int64x2_t(u8);

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("vec128_as_i16x8 $result == Seq.create 8 $i"))]
pub fn _vdupq_n_s16(i: i16) -> _int16x8_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("vec128_as_u64x2 $result == Seq.create 2 $i"))]
pub fn _vdupq_n_u64(i: u64) -> _uint64x2_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("()")]
#[hax_lib::requires(out.len() >= 8)]
#[hax_lib::ensures(|()| fstar!(
    "Seq.length (out_future <: t_Slice i16) == Seq.length ($out <: t_Slice i16) /\\
     (forall (i:nat{i < 8}).
        Seq.index (out_future <: t_Slice i16) i == get_lane_i16x8 $v i) /\\
     (forall (i:nat{i >= 8 /\\ i < Seq.length (out_future <: t_Slice i16)}).
        Seq.index (out_future <: t_Slice i16) i == Seq.index ($out <: t_Slice i16) i)"))]
pub fn _vst1q_s16(out: &mut [i16], v: _int16x8_t) {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("()")]
#[hax_lib::ensures(|()| future(out).len() == out.len())]
pub fn _vst1q_bytes(out: &mut [u8], v: _int16x8_t) {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::requires(bytes.len() >= 16)]
pub fn _vld1q_bytes(bytes: &[u8]) -> _int16x8_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::requires(array.len() >= 8)]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 8}). get_lane_i16x8 $result i == Seq.index $array i"))]
pub fn _vld1q_s16(array: &[i16]) -> _int16x8_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::requires(array.len() >= 16)]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 2}). get_lane_u64x2 $result i == \
     Core_models.Num.impl_u64__from_le_bytes \
       (Core_models.Result.impl__unwrap #(t_Array u8 (mk_usize 8)) \
                                        #Core_models.Array.t_TryFromSliceError \
          (Core_models.Convert.f_try_into #(t_Slice u8) \
                                          #(t_Array u8 (mk_usize 8)) \
                                          #FStar.Tactics.Typeclasses.solve \
             (Seq.slice $array (8*i) (8*i + 8))))"))]
pub fn _vld1q_bytes_u64(array: &[u8]) -> _uint64x2_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::requires(array.len() >= 2)]
#[hax_lib::ensures(|result| fstar!("forall (i:nat{i < 2}). get_lane_u64x2 $result i == Seq.index $array i"))]
pub fn _vld1q_u64(array: &[u64]) -> _uint64x2_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::ensures(|()| future(out).len() == out.len())]
#[hax_lib::lean::replace_body("()")]
pub fn _vst1q_u64(out: &mut [u64], v: _uint64x2_t) {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::requires(lane < 2)]
#[hax_lib::ensures(|result| fstar!("$result == get_lane_u64x2 $vec (v $lane)"))]
pub fn get_lane_u64(vec: _uint64x2_t, lane: usize) -> u64 {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::requires(out.len() >= 16)]
#[hax_lib::ensures(|()| fstar!(
    "Seq.length (out_future <: t_Slice u8) == Seq.length ($out <: t_Slice u8) /\\
     (forall (i:nat{i < 16}).
        Seq.index (out_future <: t_Slice u8) i ==
        Seq.index
          (Core_models.Num.impl_u64__to_le_bytes
             (get_lane_u64x2 $v (i / 8))) (i % 8)) /\\
     (forall (i:nat{i >= 16 /\\ i < Seq.length (out_future <: t_Slice u8)}).
        Seq.index (out_future <: t_Slice u8) i == Seq.index ($out <: t_Slice u8) i)"))]
#[hax_lib::lean::replace_body("()")]
pub fn _vst1q_bytes_u64(out: &mut [u8], v: _uint64x2_t) {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 8}). get_lane_i16x8 $result i == get_lane_i16x8 $lhs i +. get_lane_i16x8 $rhs i"))]
pub fn _vaddq_s16(lhs: _int16x8_t, rhs: _int16x8_t) -> _int16x8_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 8}). get_lane_i16x8 $result i == get_lane_i16x8 $lhs i -. get_lane_i16x8 $rhs i"))]
pub fn _vsubq_s16(lhs: _int16x8_t, rhs: _int16x8_t) -> _int16x8_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 8}). get_lane_i16x8 $result i == get_lane_i16x8 $v i *. $c"))]
pub fn _vmulq_n_s16(v: _int16x8_t, c: i16) -> _int16x8_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 8}). get_lane_u16x8 $result i == get_lane_u16x8 $v i *. $c"))]
pub fn _vmulq_n_u16(v: _uint16x8_t, c: u16) -> _uint16x8_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::requires(0 <= SHIFT_BY && SHIFT_BY < 16)]
#[hax_lib::ensures(|result| fstar!("forall (i:nat{i < 8}). get_lane_i16x8 $result i ==
    (get_lane_i16x8 $v i >>! ${SHIFT_BY})"))]
pub fn _vshrq_n_s16<const SHIFT_BY: i32>(v: _int16x8_t) -> _int16x8_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
// TODO: ensures needs `requires (v SHIFT_BY >= 0 /\ v SHIFT_BY < 16)` for >>! subtyping
pub fn _vshrq_n_u16<const SHIFT_BY: i32>(v: _uint16x8_t) -> _uint16x8_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::requires(0 <= SHIFT_BY && SHIFT_BY < 64)]
#[hax_lib::ensures(|result| fstar!("forall (i:nat{i < 2}). get_lane_u64x2 $result i ==
    (get_lane_u64x2 $v i >>! (cast ${SHIFT_BY} <: u32))"))]
pub fn _vshrq_n_u64<const SHIFT_BY: i32>(v: _uint64x2_t) -> _uint64x2_t {
    unimplemented!()
}

// Note: NO `#[hax_lib::lean::replace_body("sorry")]` — this is a real
// fallback body, not a stub, and we want both backends to extract it.
//
// The `before` block opens the rotate-decomposition lemma module so its
// SMTPat-tagged bridge lemma fires when F* sees `rotate_left ... LEFT`
// in the post.
#[cfg_attr(hax, hax_lib::fstar::before(r#"open Bitvec.U64Rotate"#))]
#[inline(always)]
#[hax_lib::requires(0 < LEFT && LEFT < 64 && 0 < RIGHT && RIGHT < 64 && LEFT + RIGHT == 64)]
#[hax_lib::ensures(|result| fstar!("forall (i:nat{i < 2}). get_lane_u64x2 $result i ==
    Core_models.Num.impl_u64__rotate_left (get_lane_u64x2 $a i ^. get_lane_u64x2 $b i) (cast ${LEFT} <: u32)"))]
pub fn _vxarq_u64<const LEFT: i32, const RIGHT: i32>(
    a: _uint64x2_t,
    b: _uint64x2_t,
) -> _uint64x2_t {
    // Manual fallback: VXAR is XOR-and-rotate-right by RIGHT, equivalent
    // to `(a XOR b) shl LEFT  XOR  (a XOR b) shr RIGHT` when LEFT+RIGHT=64.
    // The post's `rotate_left .. LEFT` is bridged to this composition
    // by `Bitvec.U64Rotate.lemma_u64_rotate_left_decomp` (SMTPat).
    let a_xor_b = _veorq_u64(a, b);
    _veorq_u64(
        _vshlq_n_u64::<LEFT>(a_xor_b),
        _vshrq_n_u64::<RIGHT>(a_xor_b),
    )
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::requires(0 <= SHIFT_BY && SHIFT_BY < 64)]
#[hax_lib::ensures(|result| fstar!("forall (i:nat{i < 2}). get_lane_u64x2 $result i ==
    (get_lane_u64x2 $v i <<! (cast ${SHIFT_BY} <: u32))"))]
pub fn _vshlq_n_u64<const SHIFT_BY: i32>(v: _uint64x2_t) -> _uint64x2_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
// TODO: ensures needs `requires (v SHIFT_BY >= 0 /\ v SHIFT_BY < 16)` for <<! subtyping
pub fn _vshlq_n_s16<const SHIFT_BY: i32>(v: _int16x8_t) -> _int16x8_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
// TODO: ensures needs `requires (v SHIFT_BY >= 0 /\ v SHIFT_BY < 32)` for <<! subtyping
pub fn _vshlq_n_u32<const SHIFT_BY: i32>(v: _uint32x4_t) -> _uint32x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
pub fn _vqdmulhq_n_s16(k: _int16x8_t, b: i16) -> _int16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
pub fn _vqdmulhq_s16(v: _int16x8_t, c: _int16x8_t) -> _int16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 8}). get_lane_u16x8 $result i == (if get_lane_i16x8 $v i >=. get_lane_i16x8 $c i then mk_u16 0xFFFF else mk_u16 0)"))]
pub fn _vcgeq_s16(v: _int16x8_t, c: _int16x8_t) -> _uint16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 8}). get_lane_i16x8 $result i == (get_lane_i16x8 $a i &. get_lane_i16x8 $b i)"))]
pub fn _vandq_s16(a: _int16x8_t, b: _int16x8_t) -> _int16x8_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 2}). get_lane_u64x2 $result i == (get_lane_u64x2 $a i &. (~. (get_lane_u64x2 $b i)))"))]
pub fn _vbicq_u64(a: _uint64x2_t, b: _uint64x2_t) -> _uint64x2_t {
    unimplemented!()
}

// Real fallback body — composition of `_veorq_u64` and `_vbicq_u64`.
#[inline(always)]
#[hax_lib::ensures(|result| fstar!("forall (i:nat{i < 2}). get_lane_u64x2 $result i ==
    (get_lane_u64x2 $a i ^. (get_lane_u64x2 $b i &. (~. (get_lane_u64x2 $c i))))"))]
pub fn _vbcaxq_u64(a: _uint64x2_t, b: _uint64x2_t, c: _uint64x2_t) -> _uint64x2_t {
    _veorq_u64(a, _vbicq_u64(b, c))
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $m0 /\\
    (forall (i:nat{i < 8}). get_lane_i16x8 $result i ==
       Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.u16_inttype #Rust_primitives.Integers.i16_inttype (get_lane_u16x8 $m0 i))"))]
pub fn _vreinterpretq_s16_u16(m0: _uint16x8_t) -> _int16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $m0 /\\
    (forall (i:nat{i < 8}). get_lane_u16x8 $result i ==
       Rust_primitives.Integers.cast_mod #Rust_primitives.Integers.i16_inttype #Rust_primitives.Integers.u16_inttype (get_lane_i16x8 $m0 i))"))]
pub fn _vreinterpretq_u16_s16(m0: _int16x8_t) -> _uint16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 8}). get_lane_i16x8 $result i == get_lane_i16x8 $v i *. get_lane_i16x8 $c i"))]
pub fn _vmulq_s16(v: _int16x8_t, c: _int16x8_t) -> _int16x8_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 8}). get_lane_i16x8 $result i == (get_lane_i16x8 $mask i ^. get_lane_i16x8 $shifted i)"))]
pub fn _veorq_s16(mask: _int16x8_t, shifted: _int16x8_t) -> _int16x8_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("forall (i:nat{i < 2}). get_lane_u64x2 $result i ==
    (get_lane_u64x2 $mask i ^. get_lane_u64x2 $shifted i)"))]
pub fn _veorq_u64(mask: _uint64x2_t, shifted: _uint64x2_t) -> _uint64x2_t {
    unimplemented!()
}

// Real fallback body — XOR-and-rotate-left-by-1.  The rotate is
// decomposed via `Bitvec.U64Rotate.lemma_u64_rotate_left_decomp`
// (SMTPat already in scope from `_vxarq_u64`'s before-block).
#[inline(always)]
#[hax_lib::ensures(|result| fstar!("forall (i:nat{i < 2}). get_lane_u64x2 $result i ==
    (get_lane_u64x2 $a i ^. Core_models.Num.impl_u64__rotate_left (get_lane_u64x2 $b i) (mk_u32 1))"))]
pub fn _vrax1q_u64(a: _uint64x2_t, b: _uint64x2_t) -> _uint64x2_t {
    _veorq_u64(a, _veorq_u64(_vshlq_n_u64::<1>(b), _vshrq_n_u64::<63>(b)))
}

// Real fallback body — triple XOR via two _veorq_u64 calls.
// Body is left-associative `(a XOR b) XOR c` to match the spec's parens
// (and the downstream `arm64_lc_xor5` equivalence lemma).  The libcrux
// arm64.rs fallback uses the right-associative form `a XOR (b XOR c)`;
// both are equivalent at runtime by XOR associativity, but the
// left-associative form lets F* discharge the post definitionally.
#[inline(always)]
#[hax_lib::ensures(|result| fstar!("forall (i:nat{i < 2}). get_lane_u64x2 $result i ==
    ((get_lane_u64x2 $a i ^. get_lane_u64x2 $b i) ^. get_lane_u64x2 $c i)"))]
pub fn _veor3q_u64(a: _uint64x2_t, b: _uint64x2_t, c: _uint64x2_t) -> _uint64x2_t {
    _veorq_u64(_veorq_u64(a, b), c)
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("vec128_as_u32x4 $result == Seq.create 4 $value"))]
pub fn _vdupq_n_u32(value: u32) -> _uint32x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 4}). get_lane_u32x4 $result i == get_lane_u32x4 $compressed i +. get_lane_u32x4 $half i"))]
pub fn _vaddq_u32(compressed: _uint32x4_t, half: _uint32x4_t) -> _uint32x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $compressed"))]
pub fn _vreinterpretq_s32_u32(compressed: _uint32x4_t) -> _int32x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
pub fn _vqdmulhq_n_s32(a: _int32x4_t, b: i32) -> _int32x4_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_u32_s32(a: _int32x4_t) -> _uint32x4_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
// TODO: ensures needs `requires (v N >= 0 /\ v N < 32)` for >>! subtyping
pub fn _vshrq_n_u32<const N: i32>(a: _uint32x4_t) -> _uint32x4_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 4}). get_lane_u32x4 $result i == (get_lane_u32x4 $a i &. get_lane_u32x4 $b i)"))]
pub fn _vandq_u32(a: _uint32x4_t, b: _uint32x4_t) -> _uint32x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_u32_s16(a: _int16x8_t) -> _uint32x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_s16_u32(a: _uint32x4_t) -> _int16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "(forall (i:nat{i < 4}). get_lane_i16x8 $result (2 * i) == get_lane_i16x8 $a (2 * i)) /\\
     (forall (i:nat{i < 4}). get_lane_i16x8 $result (2 * i + 1) == get_lane_i16x8 $b (2 * i))"))]
pub fn _vtrn1q_s16(a: _int16x8_t, b: _int16x8_t) -> _int16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "(forall (i:nat{i < 4}). get_lane_i16x8 $result (2 * i) == get_lane_i16x8 $a (2 * i + 1)) /\\
     (forall (i:nat{i < 4}). get_lane_i16x8 $result (2 * i + 1) == get_lane_i16x8 $b (2 * i + 1))"))]
pub fn _vtrn2q_s16(a: _int16x8_t, b: _int16x8_t) -> _int16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 4}). get_lane_u32x4 $result i == get_lane_u32x4 $a i *. $b"))]
pub fn _vmulq_n_u32(a: _uint32x4_t, b: u32) -> _uint32x4_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "(forall (i:nat{i < 2}). get_lane_i32x4 $result (2 * i) == get_lane_i32x4 $a (2 * i)) /\\
     (forall (i:nat{i < 2}). get_lane_i32x4 $result (2 * i + 1) == get_lane_i32x4 $b (2 * i))"))]
pub fn _vtrn1q_s32(a: _int32x4_t, b: _int32x4_t) -> _int32x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_s16_s32(a: _int32x4_t) -> _int16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_s32_s16(a: _int16x8_t) -> _int32x4_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "(forall (i:nat{i < 2}). get_lane_i32x4 $result (2 * i) == get_lane_i32x4 $a (2 * i + 1)) /\\
     (forall (i:nat{i < 2}). get_lane_i32x4 $result (2 * i + 1) == get_lane_i32x4 $b (2 * i + 1))"))]
pub fn _vtrn2q_s32(a: _int32x4_t, b: _int32x4_t) -> _int32x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "get_lane_i64x2 $result 0 == get_lane_i64x2 $a 0 /\\
     get_lane_i64x2 $result 1 == get_lane_i64x2 $b 0"))]
pub fn _vtrn1q_s64(a: _int64x2_t, b: _int64x2_t) -> _int64x2_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "get_lane_u64x2 $result 0 == get_lane_u64x2 $a 0 /\\
     get_lane_u64x2 $result 1 == get_lane_u64x2 $b 0"))]
pub fn _vtrn1q_u64(a: _uint64x2_t, b: _uint64x2_t) -> _uint64x2_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_s16_s64(a: _int64x2_t) -> _int16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_s64_s16(a: _int16x8_t) -> _int64x2_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "get_lane_i64x2 $result 0 == get_lane_i64x2 $a 1 /\\
     get_lane_i64x2 $result 1 == get_lane_i64x2 $b 1"))]
pub fn _vtrn2q_s64(a: _int64x2_t, b: _int64x2_t) -> _int64x2_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "get_lane_u64x2 $result 0 == get_lane_u64x2 $a 1 /\\
     get_lane_u64x2 $result 1 == get_lane_u64x2 $b 1"))]
pub fn _vtrn2q_u64(a: _uint64x2_t, b: _uint64x2_t) -> _uint64x2_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 4}). get_lane_i32x4 $result i == (cast (get_lane_i16x4 $a i) <: i32) *. (cast (get_lane_i16x4 $b i) <: i32)"))]
pub fn _vmull_s16(a: _int16x4_t, b: _int16x4_t) -> _int32x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 4}). get_lane_i16x4 $result i == get_lane_i16x8 $a i"))]
pub fn _vget_low_s16(a: _int16x8_t) -> _int16x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 4}). get_lane_i32x4 $result i == (cast (get_lane_i16x8 $a (i + 4)) <: i32) *. (cast (get_lane_i16x8 $b (i + 4)) <: i32)"))]
pub fn _vmull_high_s16(a: _int16x8_t, b: _int16x8_t) -> _int32x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 4}). get_lane_i32x4 $result i == get_lane_i32x4 $a i +. ((cast (get_lane_i16x4 $b i) <: i32) *. (cast (get_lane_i16x4 $c i) <: i32))"))]
pub fn _vmlal_s16(a: _int32x4_t, b: _int16x4_t, c: _int16x4_t) -> _int32x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 4}). get_lane_i32x4 $result i == get_lane_i32x4 $a i +. ((cast (get_lane_i16x8 $b (i + 4)) <: i32) *. (cast (get_lane_i16x8 $c (i + 4)) <: i32))"))]
pub fn _vmlal_high_s16(a: _int32x4_t, b: _int16x8_t, c: _int16x8_t) -> _int32x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::requires(ptr.len() >= 16)]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 16}). get_lane_u8x16 $result i == Seq.index $ptr i"))]
pub fn _vld1q_u8(ptr: &[u8]) -> _uint8x16_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_u8_s16(a: _int16x8_t) -> _uint8x16_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 16}).
       let ix = v (get_lane_u8x16 $idx i) in
       get_lane_u8x16 $result i == (if ix < 16 then get_lane_u8x16 $t ix else mk_u8 0)"))]
pub fn _vqtbl1q_u8(t: _uint8x16_t, idx: _uint8x16_t) -> _uint8x16_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_s16_u8(a: _uint8x16_t) -> _int16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
pub fn _vshlq_s16(a: _int16x8_t, b: _int16x8_t) -> _int16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
pub fn _vshlq_u16(a: _uint16x8_t, b: _int16x8_t) -> _uint16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "$result == ((get_lane_u16x4 $a 0 +. get_lane_u16x4 $a 1) +. (get_lane_u16x4 $a 2 +. get_lane_u16x4 $a 3))"))]
pub fn _vaddv_u16(a: _uint16x4_t) -> u16 {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 4}). get_lane_u16x4 $result i == get_lane_u16x8 $a i"))]
pub fn _vget_low_u16(a: _uint16x8_t) -> _uint16x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 4}). get_lane_u16x4 $result i == get_lane_u16x8 $a (i + 4)"))]
pub fn _vget_high_u16(a: _uint16x8_t) -> _uint16x4_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "$result == (((get_lane_i16x8 $a 0 +. get_lane_i16x8 $a 1) +. (get_lane_i16x8 $a 2 +. get_lane_i16x8 $a 3)) +. ((get_lane_i16x8 $a 4 +. get_lane_i16x8 $a 5) +. (get_lane_i16x8 $a 6 +. get_lane_i16x8 $a 7)))"))]
pub fn _vaddvq_s16(a: _int16x8_t) -> i16 {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
pub fn _vsliq_n_s32<const N: i32>(a: _int32x4_t, b: _int32x4_t) -> _int32x4_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_s64_s32(a: _int32x4_t) -> _int64x2_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
pub fn _vsliq_n_s64<const N: i32>(a: _int64x2_t, b: _int64x2_t) -> _int64x2_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_u8_s64(a: _int64x2_t) -> _uint8x16_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::ensures(|()| future(out).len() == out.len())]
#[hax_lib::lean::replace_body("()")]
pub fn _vst1q_u8(out: &mut [u8], v: _uint8x16_t) {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("vec128_as_u16x8 $result == Seq.create 8 $value"))]
pub fn _vdupq_n_u16(value: u16) -> _uint16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 8}). get_lane_u16x8 $result i == (get_lane_u16x8 $a i &. get_lane_u16x8 $b i)"))]
pub fn _vandq_u16(a: _uint16x8_t, b: _uint16x8_t) -> _uint16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_u16_u8(a: _uint8x16_t) -> _uint16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::requires(ptr.len() >= 8)]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 8}). get_lane_u16x8 $result i == Seq.index $ptr i"))]
pub fn _vld1q_u16(ptr: &[u16]) -> _uint16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 8}). get_lane_u16x8 $result i == (if get_lane_i16x8 $a i <=. get_lane_i16x8 $b i then mk_u16 0xFFFF else mk_u16 0)"))]
pub fn _vcleq_s16(a: _int16x8_t, b: _int16x8_t) -> _uint16x8_t {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "$result == (((get_lane_u16x8 $a 0 +. get_lane_u16x8 $a 1) +. (get_lane_u16x8 $a 2 +. get_lane_u16x8 $a 3)) +. ((get_lane_u16x8 $a 4 +. get_lane_u16x8 $a 5) +. (get_lane_u16x8 $a 6 +. get_lane_u16x8 $a 7)))"))]
pub fn _vaddvq_u16(a: _uint16x8_t) -> u16 {
    unimplemented!()
}
#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
pub fn _vmull_p64(a: u64, b: u64) -> u128 {
    unimplemented!()
}

#[inline]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 16}). get_lane_u8x16 $result i == (get_lane_u8x16 $a i ^. get_lane_u8x16 $b i)"))]
pub fn _veorq_u8(a: _uint8x16_t, b: _uint8x16_t) -> _uint8x16_t {
    unimplemented!()
}

#[inline]
#[hax_lib::lean::replace_body("sorry")]
pub fn _vaesmcq_u8(data: _uint8x16_t) -> _uint8x16_t {
    unimplemented!()
}

#[inline]
#[hax_lib::lean::replace_body("sorry")]
pub fn _vaeseq_u8(data: _uint8x16_t, key: _uint8x16_t) -> _uint8x16_t {
    unimplemented!()
}

#[inline]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("vec128_as_u8x16 $result == Seq.create 16 $value"))]
pub fn _vdupq_n_u8(value: u8) -> _uint8x16_t {
    unimplemented!()
}

#[inline]
#[hax_lib::lean::replace_body("sorry")]
// TODO: ensures needs `requires (v N >= 0 /\ v N < 4)` for index subtyping
pub fn _vdupq_laneq_u32<const N: i32>(a: _uint32x4_t) -> _uint32x4_t {
    unimplemented!()
}

#[inline]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 4}). get_lane_u32x4 $result i == (get_lane_u32x4 $a i ^. get_lane_u32x4 $b i)"))]
pub fn _veorq_u32(a: _uint32x4_t, b: _uint32x4_t) -> _uint32x4_t {
    unimplemented!()
}

#[inline]
#[hax_lib::lean::replace_body("sorry")]
// TODO: ensures needs `requires (v N >= 0 /\ v N < 4)` for index subtyping
pub fn _vextq_u32<const N: i32>(a: _uint32x4_t, b: _uint32x4_t) -> _uint32x4_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::requires(ptr.len() >= 4)]
#[hax_lib::ensures(|result| fstar!(
    "forall (i:nat{i < 4}). get_lane_u32x4 $result i == Seq.index $ptr i"))]
pub fn _vld1q_u32(ptr: &[u32]) -> _uint32x4_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_u32_u8(a: _uint8x16_t) -> _uint32x4_t {
    unimplemented!()
}

#[inline(always)]
#[hax_lib::lean::replace_body("sorry")]
#[hax_lib::ensures(|result| fstar!("$result == $a"))]
pub fn _vreinterpretq_u8_u32(a: _uint32x4_t) -> _uint8x16_t {
    unimplemented!()
}

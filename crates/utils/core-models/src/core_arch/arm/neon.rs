//! Bit-vector layer for NEON intrinsics.
//!
//! Every function here is `#[hax_lib::opaque]` with an `unimplemented!()`
//! stub body. Real computational content lives at the integer-vector layer
//! in `super::interpretations::int_vec`, with `mk_lift_lemma!` connecting
//! the two.
//!
//! Dropping the opacity attribute is **forbidden** without satisfying all
//! three justification clauses in `INTRINSICS-TRUST-PLAN.md`'s
//! "preserve `#[hax_lib::opaque]` on bit-vector layer" rule.
//!
//! # Source attribution
//!
//! Portions of this file are adapted from
//! `verify-rust-std/testable-simd-models/`, © Cryspen, Apache-2.0,
//! imported on 2026-05-02 for the libcrux SIMD intrinsics trust-base sprint.

#![allow(unused_variables)]

use super::*;

// --------- Arithmetic: add/sub/mul ----------------------------------------

/// [ARM intrinsics guide](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_s16)
#[hax_lib::opaque]
pub fn vaddq_s16(_a: int16x8_t, _b: int16x8_t) -> int16x8_t {
    unimplemented!()
}

/// [ARM intrinsics guide](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_u32)
#[hax_lib::opaque]
pub fn vaddq_u32(_a: uint32x4_t, _b: uint32x4_t) -> uint32x4_t {
    unimplemented!()
}

/// [ARM intrinsics guide](https://developer.arm.com/architectures/instruction-sets/intrinsics/vsubq_s16)
#[hax_lib::opaque]
pub fn vsubq_s16(_a: int16x8_t, _b: int16x8_t) -> int16x8_t {
    unimplemented!()
}

/// Across-vector add (signed 16-bit, full 128-bit register).
#[hax_lib::opaque]
pub fn vaddvq_s16(_a: int16x8_t) -> i16 {
    unimplemented!()
}

/// Across-vector add (unsigned 16-bit, full 128-bit register).
#[hax_lib::opaque]
pub fn vaddvq_u16(_a: uint16x8_t) -> u16 {
    unimplemented!()
}

/// Across-vector add (unsigned 16-bit, half 64-bit register).
#[hax_lib::opaque]
pub fn vaddv_u16(_a: uint16x4_t) -> u16 {
    unimplemented!()
}

/// Multiply by scalar.
#[hax_lib::opaque]
pub fn vmulq_n_s16(_a: int16x8_t, _b: i16) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vmulq_n_u16(_a: uint16x8_t, _b: u16) -> uint16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vmulq_n_u32(_a: uint32x4_t, _b: u32) -> uint32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vmulq_s16(_a: int16x8_t, _b: int16x8_t) -> int16x8_t {
    unimplemented!()
}

/// Signed widening multiply (low half).
#[hax_lib::opaque]
pub fn vmull_s16(_a: int16x4_t, _b: int16x4_t) -> int32x4_t {
    unimplemented!()
}

/// Signed widening multiply (high half).
#[hax_lib::opaque]
pub fn vmull_high_s16(_a: int16x8_t, _b: int16x8_t) -> int32x4_t {
    unimplemented!()
}

/// Signed widening multiply-accumulate (low half).
#[hax_lib::opaque]
pub fn vmlal_s16(_a: int32x4_t, _b: int16x4_t, _c: int16x4_t) -> int32x4_t {
    unimplemented!()
}

/// Signed widening multiply-accumulate (high half).
#[hax_lib::opaque]
pub fn vmlal_high_s16(_a: int32x4_t, _b: int16x8_t, _c: int16x8_t) -> int32x4_t {
    unimplemented!()
}

/// Saturating doubling multiply, returning the high half.
#[hax_lib::opaque]
pub fn vqdmulhq_s16(_a: int16x8_t, _b: int16x8_t) -> int16x8_t {
    unimplemented!()
}

/// Saturating doubling multiply by scalar, returning the high half.
#[hax_lib::opaque]
pub fn vqdmulhq_n_s16(_a: int16x8_t, _b: i16) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vqdmulhq_n_s32(_a: int32x4_t, _b: i32) -> int32x4_t {
    unimplemented!()
}

// --------- Bitwise --------------------------------------------------------

#[hax_lib::opaque]
pub fn vandq_s16(_a: int16x8_t, _b: int16x8_t) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vandq_u16(_a: uint16x8_t, _b: uint16x8_t) -> uint16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vandq_u32(_a: uint32x4_t, _b: uint32x4_t) -> uint32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vbicq_u64(_a: uint64x2_t, _b: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn veorq_s16(_a: int16x8_t, _b: int16x8_t) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn veorq_u32(_a: uint32x4_t, _b: uint32x4_t) -> uint32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn veorq_u64(_a: uint64x2_t, _b: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn veorq_u8(_a: uint8x16_t, _b: uint8x16_t) -> uint8x16_t {
    unimplemented!()
}

// --------- Comparisons ----------------------------------------------------

#[hax_lib::opaque]
pub fn vcgeq_s16(_a: int16x8_t, _b: int16x8_t) -> uint16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vcleq_s16(_a: int16x8_t, _b: int16x8_t) -> uint16x8_t {
    unimplemented!()
}

// --------- Duplicate / set / lane access ----------------------------------

#[hax_lib::opaque]
pub fn vdupq_n_s16(_a: i16) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vdupq_n_u16(_a: u16) -> uint16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vdupq_n_u32(_a: u32) -> uint32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vdupq_n_u64(_a: u64) -> uint64x2_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vdupq_n_u8(_a: u8) -> uint8x16_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vdupq_laneq_u32<const N: i32>(_a: uint32x4_t) -> uint32x4_t {
    unimplemented!()
}

/// Get the low half of a 128-bit signed 16-bit vector.
#[hax_lib::opaque]
pub fn vget_low_s16(_a: int16x8_t) -> int16x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vget_low_u16(_a: uint16x8_t) -> uint16x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vget_high_u16(_a: uint16x8_t) -> uint16x4_t {
    unimplemented!()
}

// --------- Loads / stores -------------------------------------------------
//
// The load/store intrinsics take raw pointers, which hax cannot extract;
// they are marked `#[hax_lib::exclude]`. At the model layer we provide
// real bodies (slice-from-ptr decode) so cargo tests can exercise them
// and the audit script sees `has_body=true`.

/// [ARM intrinsics guide](https://developer.arm.com/architectures/instruction-sets/intrinsics/vld1q_s16)
#[hax_lib::exclude]
pub unsafe fn vld1q_s16(ptr: *const i16) -> int16x8_t {
    let arr: &[i16] = unsafe { core::slice::from_raw_parts(ptr, 8) };
    BitVec::from_slice(arr, 16)
}

#[hax_lib::exclude]
pub unsafe fn vld1q_u16(ptr: *const u16) -> uint16x8_t {
    let arr: &[u16] = unsafe { core::slice::from_raw_parts(ptr, 8) };
    BitVec::from_slice(arr, 16)
}

#[hax_lib::exclude]
pub unsafe fn vld1q_u32(ptr: *const u32) -> uint32x4_t {
    let arr: &[u32] = unsafe { core::slice::from_raw_parts(ptr, 4) };
    BitVec::from_slice(arr, 32)
}

#[hax_lib::exclude]
pub unsafe fn vld1q_u64(ptr: *const u64) -> uint64x2_t {
    let arr: &[u64] = unsafe { core::slice::from_raw_parts(ptr, 2) };
    BitVec::from_slice(arr, 64)
}

#[hax_lib::exclude]
pub unsafe fn vld1q_u8(ptr: *const u8) -> uint8x16_t {
    let arr: &[u8] = unsafe { core::slice::from_raw_parts(ptr, 16) };
    BitVec::from_slice(arr, 8)
}

#[hax_lib::exclude]
pub unsafe fn vst1q_s16(ptr: *mut i16, v: int16x8_t) {
    let vec: Vec<i16> = v.to_vec();
    for i in 0..8 {
        unsafe {
            *ptr.add(i) = vec[i];
        }
    }
}

#[hax_lib::exclude]
pub unsafe fn vst1q_u64(ptr: *mut u64, v: uint64x2_t) {
    let vec: Vec<u64> = v.to_vec();
    for i in 0..2 {
        unsafe {
            *ptr.add(i) = vec[i];
        }
    }
}

#[hax_lib::exclude]
pub unsafe fn vst1q_u8(ptr: *mut u8, v: uint8x16_t) {
    let vec: Vec<u8> = v.to_vec();
    for i in 0..16 {
        unsafe {
            *ptr.add(i) = vec[i];
        }
    }
}

// --------- Reinterprets (semantic identity at the bit-vec layer) ---------

#[hax_lib::opaque]
pub fn vreinterpretq_s16_s32(_a: int32x4_t) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_s16_s64(_a: int64x2_t) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_s16_u16(_a: uint16x8_t) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_s16_u32(_a: uint32x4_t) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_s16_u8(_a: uint8x16_t) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_s32_s16(_a: int16x8_t) -> int32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_s32_u32(_a: uint32x4_t) -> int32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_s64_s16(_a: int16x8_t) -> int64x2_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_s64_s32(_a: int32x4_t) -> int64x2_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_u16_s16(_a: int16x8_t) -> uint16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_u16_u8(_a: uint8x16_t) -> uint16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_u32_s16(_a: int16x8_t) -> uint32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_u32_s32(_a: int32x4_t) -> uint32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_u32_u8(_a: uint8x16_t) -> uint32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_u8_s16(_a: int16x8_t) -> uint8x16_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_u8_s64(_a: int64x2_t) -> uint8x16_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vreinterpretq_u8_u32(_a: uint32x4_t) -> uint8x16_t {
    unimplemented!()
}

// --------- Shifts ---------------------------------------------------------

#[hax_lib::opaque]
pub fn vshlq_n_s16<const N: i32>(_a: int16x8_t) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vshlq_n_u32<const N: i32>(_a: uint32x4_t) -> uint32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vshlq_n_u64<const N: i32>(_a: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vshlq_s16(_a: int16x8_t, _b: int16x8_t) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vshlq_u16(_a: uint16x8_t, _b: int16x8_t) -> uint16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vshrq_n_s16<const N: i32>(_a: int16x8_t) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vshrq_n_u16<const N: i32>(_a: uint16x8_t) -> uint16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vshrq_n_u32<const N: i32>(_a: uint32x4_t) -> uint32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vshrq_n_u64<const N: i32>(_a: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vsliq_n_s32<const N: i32>(_a: int32x4_t, _b: int32x4_t) -> int32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vsliq_n_s64<const N: i32>(_a: int64x2_t, _b: int64x2_t) -> int64x2_t {
    unimplemented!()
}

// --------- Permutations / extracts ---------------------------------------

#[hax_lib::opaque]
pub fn vextq_u32<const N: i32>(_a: uint32x4_t, _b: uint32x4_t) -> uint32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vtrn1q_s16(_a: int16x8_t, _b: int16x8_t) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vtrn2q_s16(_a: int16x8_t, _b: int16x8_t) -> int16x8_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vtrn1q_s32(_a: int32x4_t, _b: int32x4_t) -> int32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vtrn2q_s32(_a: int32x4_t, _b: int32x4_t) -> int32x4_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vtrn1q_s64(_a: int64x2_t, _b: int64x2_t) -> int64x2_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vtrn2q_s64(_a: int64x2_t, _b: int64x2_t) -> int64x2_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vtrn1q_u64(_a: uint64x2_t, _b: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vtrn2q_u64(_a: uint64x2_t, _b: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

#[hax_lib::opaque]
pub fn vqtbl1q_u8(_t: uint8x16_t, _idx: uint8x16_t) -> uint8x16_t {
    unimplemented!()
}

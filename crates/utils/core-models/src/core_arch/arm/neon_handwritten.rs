//! Hand-written models for NEON intrinsics whose upstream `stdarch`
//! definitions go directly to `unsafe extern "C"` LLVM intrinsic leaves.
//!
//! This includes the SHA3 instructions (`vrax1q_u64`, `vbcaxq_u64`,
//! `veor3q_u64`, `vxarq_u64`), the AES instructions (`vaeseq_u8`,
//! `vaesmcq_u8`), and the polynomial multiply (`vmull_p64`).
//!
//! All entries here are `#[hax_lib::opaque]` stubs at the bit-vec layer;
//! computational content lives at the int-vec layer in
//! `super::interpretations::int_vec`.
//!
//! # Source attribution
//!
//! Portions of this file are adapted from
//! `verify-rust-std/testable-simd-models/`, © Cryspen, Apache-2.0,
//! imported on 2026-05-02 for the libcrux SIMD intrinsics trust-base sprint.

#![allow(unused_variables)]

use super::*;

/// SHA3 rotate-and-XOR (RAX1).
/// `result[i] = a[i] XOR rotate-left-1(b[i])` per 64-bit lane.
#[hax_lib::opaque]
pub fn vrax1q_u64(_a: uint64x2_t, _b: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

/// SHA3 bit-clear-and-XOR (BCAX). `result = a XOR (b AND NOT c)`.
#[hax_lib::opaque]
pub fn vbcaxq_u64(_a: uint64x2_t, _b: uint64x2_t, _c: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

/// SHA3 three-way XOR (EOR3). `result = a XOR b XOR c`.
#[hax_lib::opaque]
pub fn veor3q_u64(_a: uint64x2_t, _b: uint64x2_t, _c: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

/// SHA3 XOR-and-rotate (XAR). `result = rotate-right-N(a XOR b)` per 64-bit lane.
#[hax_lib::opaque]
pub fn vxarq_u64<const N: i32>(_a: uint64x2_t, _b: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

/// AES single round encryption: `AESE(data, key) = AddRoundKey(SubBytes(ShiftRows(data XOR key)))`.
/// (Note: the AArch64 AESE instruction does NOT include MixColumns; it does
/// XOR with key, ShiftRows, and SubBytes. MixColumns is a separate AESMC.)
#[hax_lib::opaque]
pub fn vaeseq_u8(_data: uint8x16_t, _key: uint8x16_t) -> uint8x16_t {
    unimplemented!()
}

/// AES MixColumns step.
#[hax_lib::opaque]
pub fn vaesmcq_u8(_data: uint8x16_t) -> uint8x16_t {
    unimplemented!()
}

/// Polynomial multiplication of two 64-bit elements over GF(2), producing
/// a 128-bit polynomial (carry-less multiply). The libcrux wrapper takes
/// scalar `u64` operands and returns `u128`; the arm64.rs wrapper passes
/// these directly to `core::arch::aarch64::vmull_p64`.
#[hax_lib::opaque]
pub fn vmull_p64(_a: u64, _b: u64) -> u128 {
    unimplemented!()
}

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
//! `verify-rust-std/testable-simd-models/`, © Cryspen, Apache-2.0.

#![allow(unused_variables)]

use super::*;

/// SHA3 rotate-and-XOR (RAX1).
/// `result[i] = a[i] XOR rotate-left-1(b[i])` per 64-bit lane.
/// [Arm Documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vrax1q_u64)
#[libcrux_macros::trusted(opaque, "validated-axiom: vrax1q_u64 hardware intrinsic; int-vec model + mk_lift_lemma! lift + mk! difftest")]
pub fn vrax1q_u64(_a: uint64x2_t, _b: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

/// SHA3 bit-clear-and-XOR (BCAX). `result = a XOR (b AND NOT c)`.
/// [Arm Documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vbcaxq_u64)
#[libcrux_macros::trusted(opaque, "validated-axiom: vbcaxq_u64 hardware intrinsic; int-vec model + mk_lift_lemma! lift + mk! difftest")]
pub fn vbcaxq_u64(_a: uint64x2_t, _b: uint64x2_t, _c: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

/// SHA3 three-way XOR (EOR3). `result = a XOR b XOR c`.
/// [Arm Documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/veor3q_u64)
#[libcrux_macros::trusted(opaque, "validated-axiom: veor3q_u64 hardware intrinsic; int-vec model + mk_lift_lemma! lift + mk! difftest")]
pub fn veor3q_u64(_a: uint64x2_t, _b: uint64x2_t, _c: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

/// SHA3 XOR-and-rotate (XAR). `result = rotate-right-N(a XOR b)` per 64-bit lane.
/// [Arm Documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vxarq_u64)
#[libcrux_macros::trusted(opaque, "validated-axiom: vxarq_u64 hardware intrinsic; int-vec model + mk_lift_lemma! lift + mk! difftest")]
pub fn vxarq_u64<const N: i32>(_a: uint64x2_t, _b: uint64x2_t) -> uint64x2_t {
    unimplemented!()
}

/// AES single round encryption: `AESE(data, key) = AddRoundKey(SubBytes(ShiftRows(data XOR key)))`.
/// (Note: the AArch64 AESE instruction does NOT include MixColumns; it does
/// XOR with key, ShiftRows, and SubBytes. MixColumns is a separate AESMC.)
/// [Arm Documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaeseq_u8)
#[libcrux_macros::trusted(opaque, "validated-axiom: vaeseq_u8 hardware intrinsic; int-vec model + mk_lift_lemma! lift + mk! difftest")]
pub fn vaeseq_u8(_data: uint8x16_t, _key: uint8x16_t) -> uint8x16_t {
    unimplemented!()
}

/// AES MixColumns step.
/// [Arm Documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaesmcq_u8)
#[libcrux_macros::trusted(opaque, "validated-axiom: vaesmcq_u8 hardware intrinsic; int-vec model + mk_lift_lemma! lift + mk! difftest")]
pub fn vaesmcq_u8(_data: uint8x16_t) -> uint8x16_t {
    unimplemented!()
}

/// Polynomial multiplication of two 64-bit elements over GF(2), producing
/// a 128-bit polynomial (carry-less multiply). The libcrux wrapper takes
/// scalar `u64` operands and returns `u128`; the arm64.rs wrapper passes
/// these directly to `core::arch::aarch64::vmull_p64`.
/// [Arm Documentation](https://developer.arm.com/architectures/instruction-sets/intrinsics/vmull_p64)
#[libcrux_macros::trusted(opaque, "validated-axiom: vmull_p64 hardware intrinsic; int-vec model + mk_lift_lemma! lift + mk! difftest")]
pub fn vmull_p64(_a: u64, _b: u64) -> u128 {
    unimplemented!()
}

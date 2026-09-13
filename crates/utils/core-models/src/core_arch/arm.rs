//! A (partial) Rust-based model of [`core::arch::aarch64`].
//!
//! Models 94 NEON intrinsics referenced from `crates/utils/intrinsics/src/arm64.rs`
//! (the `T1_arm64` set).
//!
//! # Layout
//!
//! Mirrors `core_arch/x86.rs`'s pattern:
//!
//! - **Bit-vector layer** (this file, module `neon`): every intrinsic is a
//!   `#[hax_lib::opaque]` stub returning `unimplemented!()`. The opacity
//!   attribute is **load-bearing** for downstream F* proofs (see
//!   `INTRINSICS-TRUST-PLAN.md`'s opacity rule).
//! - **Integer-vector layer** (`interpretations::int_vec`): real
//!   computational bodies, plus `mk_lift_lemma!` connecting bit-vec ↔ int-vec
//!   and `mk!` randomized differential tests against `core::arch::aarch64::*`.
//! - **Hand-written extern leaves** (`neon_handwritten`): models for
//!   intrinsics whose upstream `stdarch` definitions go directly to LLVM
//!   intrinsic leaves (e.g. `vaeseq_u8`, `vqtbl1q_u8`, `vmull_p64`).
//!
//! Tests are gated on `target_arch = "aarch64"` so they run natively on
//! Apple Silicon hosts and CI runners.
//!
//! # Source attribution
//!
//! Portions of this file are adapted from
//! `verify-rust-std/testable-simd-models/`, © Cryspen, Apache-2.0.
//! The const-generic of `BitVec<N>` is reconciled from `u32` (upstream) to
//! `u64` (libcrux core-models) per `INTRINSICS-TRUST-PLAN.md`.

#![allow(clippy::too_many_arguments)]
#![allow(non_camel_case_types)]

pub mod interpretations;
pub mod neon;
pub mod neon_handwritten;

pub use neon::*;
pub use neon_handwritten::*;

use crate::abstractions::bitvec::BitVec;

pub(crate) mod upstream {
    #[cfg(target_arch = "aarch64")]
    pub use core::arch::aarch64::*;
}

#[libcrux_macros::trusted(replace, "trusted-extern: NEON vector F* type aliases (hardware type model)")]
#[hax_lib::fstar::replace(
    r#"
    unfold type t_int8x16_t = $:{int8x16_t}
    unfold type t_int16x8_t = $:{int16x8_t}
    unfold type t_int32x4_t = $:{int32x4_t}
    unfold type t_int64x2_t = $:{int64x2_t}
    unfold type t_uint8x16_t = $:{uint8x16_t}
    unfold type t_uint16x8_t = $:{uint16x8_t}
    unfold type t_uint32x4_t = $:{uint32x4_t}
    unfold type t_uint64x2_t = $:{uint64x2_t}
    unfold type t_int8x8_t = $:{int8x8_t}
    unfold type t_int16x4_t = $:{int16x4_t}
    unfold type t_int32x2_t = $:{int32x2_t}
    unfold type t_int64x1_t = $:{int64x1_t}
    unfold type t_uint8x8_t = $:{uint8x8_t}
    unfold type t_uint16x4_t = $:{uint16x4_t}
    unfold type t_uint32x2_t = $:{uint32x2_t}
    unfold type t_uint64x1_t = $:{uint64x1_t}
"#
)]
const _: () = {};

/// 128-bit wide vector containing 16 signed 8-bit integers.
pub type int8x16_t = BitVec<128>;
/// 128-bit wide vector containing 8 signed 16-bit integers.
pub type int16x8_t = BitVec<128>;
/// 128-bit wide vector containing 4 signed 32-bit integers.
pub type int32x4_t = BitVec<128>;
/// 128-bit wide vector containing 2 signed 64-bit integers.
pub type int64x2_t = BitVec<128>;
/// 128-bit wide vector containing 16 unsigned 8-bit integers.
pub type uint8x16_t = BitVec<128>;
/// 128-bit wide vector containing 8 unsigned 16-bit integers.
pub type uint16x8_t = BitVec<128>;
/// 128-bit wide vector containing 4 unsigned 32-bit integers.
pub type uint32x4_t = BitVec<128>;
/// 128-bit wide vector containing 2 unsigned 64-bit integers.
pub type uint64x2_t = BitVec<128>;

/// 64-bit wide vector containing 8 signed 8-bit integers.
pub type int8x8_t = BitVec<64>;
/// 64-bit wide vector containing 4 signed 16-bit integers.
pub type int16x4_t = BitVec<64>;
/// 64-bit wide vector containing 2 signed 32-bit integers.
pub type int32x2_t = BitVec<64>;
/// 64-bit wide vector containing 1 signed 64-bit integer.
pub type int64x1_t = BitVec<64>;
/// 64-bit wide vector containing 8 unsigned 8-bit integers.
pub type uint8x8_t = BitVec<64>;
/// 64-bit wide vector containing 4 unsigned 16-bit integers.
pub type uint16x4_t = BitVec<64>;
/// 64-bit wide vector containing 2 unsigned 32-bit integers.
pub type uint32x2_t = BitVec<64>;
/// 64-bit wide vector containing 1 unsigned 64-bit integer.
pub type uint64x1_t = BitVec<64>;

/// `From` conversions between `BitVec<N>` and `<u/i>NxM_t` are direct identity
/// since the latter ARE bit-vectors at the model layer. Concrete conversions
/// to/from real `core::arch::aarch64` types are handled in `interpretations`.
#[hax_lib::exclude]
#[cfg(target_arch = "aarch64")]
mod conversions {
    use super::upstream::*;
    use crate::abstractions::bitvec::BitVec;

    macro_rules! bv_convert {
        ($($ty1:ident [$prim:ty ; $n:literal ; $bits:literal]),* $(,)?) => {
            $(
                impl From<$ty1> for BitVec<$bits> {
                    fn from(arg: $ty1) -> BitVec<$bits> {
                        let stuff = unsafe {
                            *(&arg as *const $ty1 as *const [$prim; $n])
                        };
                        BitVec::from_slice(&stuff[..], <$prim>::BITS as u64)
                    }
                }
                impl From<BitVec<$bits>> for $ty1 {
                    fn from(bv: BitVec<$bits>) -> $ty1 {
                        let v: Vec<$prim> = bv.to_vec();
                        let arr: [$prim; $n] = v.try_into().unwrap();
                        unsafe { *(arr.as_ptr() as *const $ty1) }
                    }
                }
            )*
        }
    }

    bv_convert!(
        int8x16_t [i8; 16; 128],
        int16x8_t [i16; 8; 128],
        int32x4_t [i32; 4; 128],
        int64x2_t [i64; 2; 128],
        uint8x16_t [u8; 16; 128],
        uint16x8_t [u16; 8; 128],
        uint32x4_t [u32; 4; 128],
        uint64x2_t [u64; 2; 128],
        int8x8_t [i8; 8; 64],
        int16x4_t [i16; 4; 64],
        int32x2_t [i32; 2; 64],
        int64x1_t [i64; 1; 64],
        uint8x8_t [u8; 8; 64],
        uint16x4_t [u16; 4; 64],
        uint32x2_t [u32; 2; 64],
        uint64x1_t [u64; 1; 64],
    );
}

/// Extractable slice-I/O models for the raw-pointer NEON load/store wrappers in
/// `libcrux-intrinsics`' `arm64.rs`.  Mirror of `x86::extra`: the upstream
/// `vld1q_*` / `vst1q_*` intrinsics are raw-pointer FFI with no memory model, so
/// under the hax cfg the wrappers delegate here, turning the slice-I/O bit/lane
/// semantics into concrete (provable) definitions rather than `assume val`s.
///
/// The models are TOTAL: loads read 0 past the end of a short slice, stores drop
/// lane-writes when the slice is shorter than the vector (one top-level `if
/// len >= N` guard + straight-line per-lane writes, so the extracted WP is a
/// ground `Seq.upd` spine, not a 2^N-path split).  Real callers always pass
/// slices of exactly the vector width (16 bytes / 8 i16 / 2 u64 / …).
///
/// A NEON register is `BitVec<128>`.  Byte stores/loads write/read all 16 bytes
/// (LSB-first); `vld1q_bytes`, `vld1q_u8`, and `vld1q_bytes_u64` all reduce to
/// the same 128-bit register (bit-identical), so they share `vld1q_bytes_model`;
/// likewise the three 16-byte stores share `vst1q_bytes_model`.
pub mod extra {
    use super::BitVec;
    use crate::abstractions::funarr::FunArray;

    /// `vld1q_u8` / `vld1q_bytes` / `vld1q_bytes_u64`: read 16 bytes LSB-first.
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    pub fn vld1q_bytes_model(input: &[u8]) -> BitVec<128> {
        BitVec::from_u8x16(FunArray::from_fn(|j| {
            if (j as usize) < input.len() {
                input[j as usize]
            } else {
                0
            }
        }))
    }

    /// `vld1q_s16`: read 8 i16 lanes.
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    pub fn vld1q_s16_model(input: &[i16]) -> BitVec<128> {
        BitVec::from_i16x8(FunArray::from_fn(|j| {
            if (j as usize) < input.len() {
                input[j as usize]
            } else {
                0
            }
        }))
    }

    /// `vld1q_u16`: read 8 u16 lanes.
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    pub fn vld1q_u16_model(input: &[u16]) -> BitVec<128> {
        BitVec::from_u16x8(FunArray::from_fn(|j| {
            if (j as usize) < input.len() {
                input[j as usize]
            } else {
                0
            }
        }))
    }

    /// `vld1q_u32`: read 4 u32 lanes.
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    pub fn vld1q_u32_model(input: &[u32]) -> BitVec<128> {
        BitVec::from_u32x4(FunArray::from_fn(|j| {
            if (j as usize) < input.len() {
                input[j as usize]
            } else {
                0
            }
        }))
    }

    /// `vld1q_u64`: read 2 u64 lanes.
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    pub fn vld1q_u64_model(input: &[u64]) -> BitVec<128> {
        BitVec::from_u64x2(FunArray::from_fn(|j| {
            if (j as usize) < input.len() {
                input[j as usize]
            } else {
                0
            }
        }))
    }

    /// `vst1q_u8` / `vst1q_bytes` / `vst1q_bytes_u64`: write 16 bytes LSB-first.
    #[hax_lib::ensures(|_r| future(output).len() == output.len())]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    pub fn vst1q_bytes_model(output: &mut [u8], vector: BitVec<128>) {
        let lanes = BitVec::to_u8x16(vector);
        if output.len() >= 16 {
            output[0] = lanes[0];
            output[1] = lanes[1];
            output[2] = lanes[2];
            output[3] = lanes[3];
            output[4] = lanes[4];
            output[5] = lanes[5];
            output[6] = lanes[6];
            output[7] = lanes[7];
            output[8] = lanes[8];
            output[9] = lanes[9];
            output[10] = lanes[10];
            output[11] = lanes[11];
            output[12] = lanes[12];
            output[13] = lanes[13];
            output[14] = lanes[14];
            output[15] = lanes[15];
        }
    }

    /// `vst1q_s16`: write 8 i16 lanes.
    #[hax_lib::ensures(|_r| future(output).len() == output.len())]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    pub fn vst1q_s16_model(output: &mut [i16], vector: BitVec<128>) {
        let lanes = BitVec::to_i16x8(vector);
        if output.len() >= 8 {
            output[0] = lanes[0];
            output[1] = lanes[1];
            output[2] = lanes[2];
            output[3] = lanes[3];
            output[4] = lanes[4];
            output[5] = lanes[5];
            output[6] = lanes[6];
            output[7] = lanes[7];
        }
    }

    /// `vst1q_u64`: write 2 u64 lanes.
    #[hax_lib::ensures(|_r| future(output).len() == output.len())]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    pub fn vst1q_u64_model(output: &mut [u64], vector: BitVec<128>) {
        let lanes = BitVec::to_u64x2(vector);
        if output.len() >= 2 {
            output[0] = lanes[0];
            output[1] = lanes[1];
        }
    }

    /// `get_lane_u64`: the 64-bit lane at `lane` (0..2) of a 128-bit vector.
    /// TOTAL: `lane >= 2` yields 0 (the wrapper would panic; real callers pass
    /// `lane < 2`).  NOTE: arm is u64x2 (2 lanes), unlike x86's u64x4.
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    pub fn get_lane_u64_model(vector: BitVec<128>, lane: usize) -> u64 {
        let lanes = BitVec::to_u64x2(vector);
        if lane < 2 {
            lanes[lane as u64]
        } else {
            0
        }
    }
}

/// Differential tests: each `extra::*_model` vs the real `core::arch::aarch64`
/// intrinsic on the same data.  Native on aarch64 (Apple Silicon / CI) — no
/// cross-compile, unlike the x86 side.  These ground every NEON slice-I/O lemma.
#[cfg(all(test, target_arch = "aarch64"))]
mod extra_tests {
    use super::extra;
    use super::upstream;
    use crate::abstractions::bitvec::BitVec;

    const N: usize = 1000;

    #[test]
    fn vld1q_bytes_model_diff() {
        for _ in 0..N {
            let bytes: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect();
            let hw = unsafe { upstream::vld1q_u8(bytes.as_ptr()) };
            let mut hw_out = [0u8; 16];
            unsafe { upstream::vst1q_u8(hw_out.as_mut_ptr(), hw) };
            let model_bytes: Vec<u8> = extra::vld1q_bytes_model(&bytes).to_vec();
            assert_eq!(&model_bytes[..], &hw_out[..]);
        }
    }

    #[test]
    fn vld1q_s16_model_diff() {
        for _ in 0..N {
            let data: Vec<i16> = (0..8).map(|_| rand::random::<i16>()).collect();
            let hw = unsafe { upstream::vld1q_s16(data.as_ptr()) };
            let mut hw_out = [0i16; 8];
            unsafe { upstream::vst1q_s16(hw_out.as_mut_ptr(), hw) };
            let model_lanes: Vec<i16> = extra::vld1q_s16_model(&data).to_vec();
            assert_eq!(&model_lanes[..], &hw_out[..]);
        }
    }

    #[test]
    fn vld1q_u16_model_diff() {
        for _ in 0..N {
            let data: Vec<u16> = (0..8).map(|_| rand::random::<u16>()).collect();
            let hw = unsafe { upstream::vld1q_u16(data.as_ptr()) };
            let mut hw_out = [0u16; 8];
            unsafe { upstream::vst1q_u16(hw_out.as_mut_ptr(), hw) };
            let model_lanes: Vec<u16> = extra::vld1q_u16_model(&data).to_vec();
            assert_eq!(&model_lanes[..], &hw_out[..]);
        }
    }

    #[test]
    fn vld1q_u32_model_diff() {
        for _ in 0..N {
            let data: Vec<u32> = (0..4).map(|_| rand::random::<u32>()).collect();
            let hw = unsafe { upstream::vld1q_u32(data.as_ptr()) };
            let mut hw_out = [0u32; 4];
            unsafe { upstream::vst1q_u32(hw_out.as_mut_ptr(), hw) };
            let model_lanes: Vec<u32> = extra::vld1q_u32_model(&data).to_vec();
            assert_eq!(&model_lanes[..], &hw_out[..]);
        }
    }

    #[test]
    fn vld1q_u64_model_diff() {
        for _ in 0..N {
            let data: Vec<u64> = (0..2).map(|_| rand::random::<u64>()).collect();
            let hw = unsafe { upstream::vld1q_u64(data.as_ptr()) };
            let mut hw_out = [0u64; 2];
            unsafe { upstream::vst1q_u64(hw_out.as_mut_ptr(), hw) };
            let model_lanes: Vec<u64> = extra::vld1q_u64_model(&data).to_vec();
            assert_eq!(&model_lanes[..], &hw_out[..]);
        }
    }

    #[test]
    fn vst1q_bytes_model_diff() {
        for _ in 0..N {
            let bv: BitVec<128> = BitVec::rand();
            let bytes: Vec<u8> = bv.to_vec();
            let hw = unsafe { upstream::vld1q_u8(bytes.as_ptr()) };
            let mut hw_out = [0u8; 16];
            unsafe { upstream::vst1q_u8(hw_out.as_mut_ptr(), hw) };
            let mut model_out = [0u8; 16];
            extra::vst1q_bytes_model(&mut model_out, bv);
            assert_eq!(&model_out[..], &hw_out[..]);
        }
    }

    #[test]
    fn vst1q_s16_model_diff() {
        for _ in 0..N {
            let bv: BitVec<128> = BitVec::rand();
            let bytes: Vec<u8> = bv.to_vec();
            let hw = unsafe { upstream::vld1q_u8(bytes.as_ptr()) };
            let hw = unsafe { upstream::vreinterpretq_s16_u8(hw) };
            let mut hw_out = [0i16; 8];
            unsafe { upstream::vst1q_s16(hw_out.as_mut_ptr(), hw) };
            let mut model_out = [0i16; 8];
            extra::vst1q_s16_model(&mut model_out, bv);
            assert_eq!(&model_out[..], &hw_out[..]);
        }
    }

    #[test]
    fn vst1q_u64_model_diff() {
        for _ in 0..N {
            let bv: BitVec<128> = BitVec::rand();
            let bytes: Vec<u8> = bv.to_vec();
            let hw = unsafe { upstream::vld1q_u8(bytes.as_ptr()) };
            let hw = unsafe { upstream::vreinterpretq_u64_u8(hw) };
            let mut hw_out = [0u64; 2];
            unsafe { upstream::vst1q_u64(hw_out.as_mut_ptr(), hw) };
            let mut model_out = [0u64; 2];
            extra::vst1q_u64_model(&mut model_out, bv);
            assert_eq!(&model_out[..], &hw_out[..]);
        }
    }

    #[test]
    fn get_lane_u64_model_diff() {
        for _ in 0..N {
            let bv: BitVec<128> = BitVec::rand();
            let bytes: Vec<u8> = bv.to_vec();
            let hw = unsafe { upstream::vld1q_u8(bytes.as_ptr()) };
            let hw = unsafe { upstream::vreinterpretq_u64_u8(hw) };
            let mut hw_lanes = [0u64; 2];
            unsafe { upstream::vst1q_u64(hw_lanes.as_mut_ptr(), hw) };
            for lane in 0..2 {
                assert_eq!(extra::get_lane_u64_model(bv, lane), hw_lanes[lane]);
            }
        }
    }
}

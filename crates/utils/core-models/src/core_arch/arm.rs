//! A (partial) Rust-based model of [`core::arch::aarch64`].
//!
//! Models 94 NEON intrinsics referenced from `crates/utils/intrinsics/src/arm64.rs`
//! (the `T1_arm64` set in the SIMD intrinsics trust-base sprint).
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
//! `verify-rust-std/testable-simd-models/`, © Cryspen, Apache-2.0,
//! imported on 2026-05-02 for the libcrux SIMD intrinsics trust-base sprint.
//! The const-generic of `BitVec<N>` was reconciled from `u32` (upstream) to
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

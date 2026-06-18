//! Arm64 (NEON) math wrappers, the `uint64x2_t` type alias, and the
//! `KeccakItem<2>` impl.

use libcrux_intrinsics::arm64::*;

use crate::traits::KeccakItem;

#[allow(non_camel_case_types)]
pub type uint64x2_t = _uint64x2_t;

#[inline(always)]
fn _veor5q_u64(
    a: uint64x2_t,
    b: uint64x2_t,
    c: uint64x2_t,
    d: uint64x2_t,
    e: uint64x2_t,
) -> uint64x2_t {
    _veor3q_u64(_veor3q_u64(a, b, c), d, e)
}

#[inline(always)]
fn _vrax1q_u64(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    libcrux_intrinsics::arm64::_vrax1q_u64(a, b)
}

#[inline(always)]
fn _vxarq_u64<const LEFT: i32, const RIGHT: i32>(a: uint64x2_t, b: uint64x2_t) -> uint64x2_t {
    libcrux_intrinsics::arm64::_vxarq_u64::<LEFT, RIGHT>(a, b)
}

#[inline(always)]
fn _vbcaxq_u64(a: uint64x2_t, b: uint64x2_t, c: uint64x2_t) -> uint64x2_t {
    libcrux_intrinsics::arm64::_vbcaxq_u64(a, b, c)
}

#[inline(always)]
fn _veorq_n_u64(a: uint64x2_t, c: u64) -> uint64x2_t {
    let c = _vdupq_n_u64(c);
    _veorq_u64(a, c)
}

impl KeccakItem<2> for uint64x2_t {
    #[inline(always)]
    fn zero() -> Self {
        _vdupq_n_u64(0)
    }
    #[inline(always)]
    fn xor5(a: Self, b: Self, c: Self, d: Self, e: Self) -> Self {
        _veor5q_u64(a, b, c, d, e)
    }
    #[inline(always)]
    fn rotate_left1_and_xor(a: Self, b: Self) -> Self {
        _vrax1q_u64(a, b)
    }
    #[inline(always)]
    fn xor_and_rotate<const LEFT: i32, const RIGHT: i32>(a: Self, b: Self) -> Self {
        _vxarq_u64::<LEFT, RIGHT>(a, b)
    }
    #[inline(always)]
    fn and_not_xor(a: Self, b: Self, c: Self) -> Self {
        _vbcaxq_u64(a, b, c)
    }
    #[inline(always)]
    fn xor_constant(a: Self, c: u64) -> Self {
        _veorq_n_u64(a, c)
    }
    #[inline(always)]
    fn xor(a: Self, b: Self) -> Self {
        _veorq_u64(a, b)
    }
}

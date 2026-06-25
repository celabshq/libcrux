//! AVX2 SIMD math wrappers + `KeccakItem<4>` trait impl.
//!
//! Lives in its own submodule (extracts to
//! `Libcrux_sha3.Simd.Avx2.Wrappers`) so that the parent
//! `Libcrux_sha3.Simd.Avx2` does NOT carry top-level body
//! definitions — that suppresses hax's `.Bundle.fst` collation,
//! keeping each load/store/wrapper module's F* SMT context minimal.

#[cfg(hax)]
use hax_lib::int::ToInt;

use libcrux_intrinsics::avx2::*;

use crate::traits::*;

#[inline(always)]
#[hax_lib::requires(0 <= LEFT && LEFT <= 64 && 0 <= RIGHT && RIGHT <= 64)]
fn rotate_left<const LEFT: i32, const RIGHT: i32>(x: Vec256) -> Vec256 {
    #[cfg(not(any(eurydice, hax)))]
    debug_assert!(LEFT + RIGHT == 64);
    // This could be done more efficiently, if the shift values are multiples of 8.
    // However, in SHA-3 this function is only called twice with such inputs (8/56).
    mm256_xor_si256(mm256_slli_epi64::<LEFT>(x), mm256_srli_epi64::<RIGHT>(x))
}

#[inline(always)]
fn _veor5q_u64(a: Vec256, b: Vec256, c: Vec256, d: Vec256, e: Vec256) -> Vec256 {
    // Left-associated to match the spec shape `(((a^b)^c)^d)^e` so
    // [avx2_lc_xor5] can compose lane-wise SMTPats without needing
    // assoc/comm of `^.` on u64.
    let ab = mm256_xor_si256(a, b);
    let abc = mm256_xor_si256(ab, c);
    let abcd = mm256_xor_si256(abc, d);
    mm256_xor_si256(abcd, e)
}

#[inline(always)]
fn _vrax1q_u64(a: Vec256, b: Vec256) -> Vec256 {
    mm256_xor_si256(a, rotate_left::<1, 63>(b))
}

#[inline(always)]
#[hax_lib::requires(0 <= LEFT && LEFT <= 64 && 0 <= RIGHT && RIGHT <= 64)]
fn _vxarq_u64<const LEFT: i32, const RIGHT: i32>(a: Vec256, b: Vec256) -> Vec256 {
    let ab = mm256_xor_si256(a, b);
    rotate_left::<LEFT, RIGHT>(ab)
}

#[inline(always)]
fn _vbcaxq_u64(a: Vec256, b: Vec256, c: Vec256) -> Vec256 {
    mm256_xor_si256(a, mm256_andnot_si256(c, b))
}

#[inline(always)]
fn _veorq_n_u64(a: Vec256, c: u64) -> Vec256 {
    // Casting here is required, doesn't change the value.
    let c = mm256_set1_epi64x(c as i64);
    mm256_xor_si256(a, c)
}

#[hax_lib::attributes]
impl KeccakItem<4> for Vec256 {
    #[inline(always)]
    fn zero() -> Self {
        mm256_set1_epi64x(0)
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
    #[hax_lib::requires(
        LEFT.to_int() + RIGHT.to_int() == 64.to_int() &&
        RIGHT > 0 &&
        RIGHT < 64
    )]
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
        mm256_xor_si256(a, b)
    }
}

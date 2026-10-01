use libcrux_intrinsics::arm64::*;

use crate::generic_keccak::KeccakState;
use crate::traits::{get_ij, Squeeze2};

use super::wrappers::uint64x2_t;

/// Per-iteration store wrapper for the `store_block` loop body.
#[inline(always)]
fn store_u64x2x2(
    out0: &mut [u8],
    out1: &mut [u8],
    s_2i: uint64x2_t,
    s_succ: uint64x2_t,
    start: usize,
    i: usize,
) {
    let v0 = _vtrn1q_u64(s_2i, s_succ);
    let v1 = _vtrn2q_u64(s_2i, s_succ);
    _vst1q_bytes_u64(&mut out0[start + 16 * i..start + 16 * (i + 1)], v0);
    _vst1q_bytes_u64(&mut out1[start + 16 * i..start + 16 * (i + 1)], v1);
}

/// Tail wrapper for the `remaining > 8` branch of `store_block`.
#[inline(always)]
fn store_tail_high(
    out0: &mut [u8],
    out1: &mut [u8],
    s_2i: uint64x2_t,
    s_succ: uint64x2_t,
    start: usize,
    q: usize,
    remaining: usize,
) {
    let v0 = _vtrn1q_u64(s_2i, s_succ);
    let v1 = _vtrn2q_u64(s_2i, s_succ);
    let mut out0_tmp = [0u8; 16];
    let mut out1_tmp = [0u8; 16];
    _vst1q_bytes_u64(&mut out0_tmp, v0);
    _vst1q_bytes_u64(&mut out1_tmp, v1);
    out0[start + 16 * q..start + 16 * q + remaining].copy_from_slice(&out0_tmp[0..remaining]);
    out1[start + 16 * q..start + 16 * q + remaining].copy_from_slice(&out1_tmp[0..remaining]);
}

/// Tail wrapper for the `remaining > 0 && remaining <= 8` branch of
/// `store_block`. A single 16-byte tmp materialized from one state
/// slot — its low half (`tmp[0..remaining]`) goes to `out0`, its high
/// half (`tmp[8..8+remaining]`) goes to `out1`.
#[inline(always)]
fn store_tail_low(
    out0: &mut [u8],
    out1: &mut [u8],
    s_2q: uint64x2_t,
    start: usize,
    q: usize,
    remaining: usize,
) {
    let mut out01 = [0u8; 16];
    _vst1q_bytes_u64(&mut out01, s_2q);
    out0[start + 16 * q..start + 16 * q + remaining].copy_from_slice(&out01[0..remaining]);
    out1[start + 16 * q..start + 16 * q + remaining].copy_from_slice(&out01[8..8 + remaining]);
}

/// Loop-only half of `store_block`. Iterates over `i in 0..q`, calling
/// `store_u64x2x2` per iteration to fill `out0[start..start+16q]` and
/// `out1[start..start+16q]` from state slots `s[2*i]` and `s[2*i+1]`.
#[inline(always)]
fn store_block_full(
    s: &[uint64x2_t; 25],
    out0: &mut [u8],
    out1: &mut [u8],
    start: usize,
    q: usize,
) {
    for i in 0..q {
        let i0 = (2 * i) / 5;
        let j0 = (2 * i) % 5;
        let i1 = (2 * i + 1) / 5;
        let j1 = (2 * i + 1) % 5;
        store_u64x2x2(out0, out1, *get_ij(s, i0, j0), *get_ij(s, i1, j1), start, i);
    }
}

/// Tail-only half of `store_block`. Dispatches to `store_tail_high`
/// (when `remaining > 8`) or `store_tail_low` (when
/// `0 < remaining <= 8`) to fill the partial window
/// `out0[start+16q..start+16q+remaining]` (likewise `out1`). When
/// `remaining == 0` the function is a no-op.
#[inline(always)]
fn store_block_tail(
    s: &[uint64x2_t; 25],
    out0: &mut [u8],
    out1: &mut [u8],
    start: usize,
    q: usize,
    remaining: usize,
) {
    if remaining > 8 {
        let i = 2 * q;
        let i0 = i / 5;
        let j0 = i % 5;
        let i1 = (i + 1) / 5;
        let j1 = (i + 1) % 5;
        let s_2i = *get_ij(s, i0, j0);
        let s_succ = *get_ij(s, i1, j1);
        store_tail_high(out0, out1, s_2i, s_succ, start, q, remaining);
    } else if remaining > 0 {
        let i = 2 * q;
        let s_2q = *get_ij(s, i / 5, i % 5);
        store_tail_low(out0, out1, s_2q, start, q, remaining);
    }
}

#[inline(always)]
pub(crate) fn store_block<const RATE: usize>(
    s: &[uint64x2_t; 25],
    out0: &mut [u8],
    out1: &mut [u8],
    start: usize,
    len: usize,
) {
    #[cfg(not(eurydice))]
    debug_assert!(len <= RATE && start + len <= out0.len() && out0.len() == out1.len());

    let q = len / 16;
    let remaining = len % 16;
    store_block_full(s, out0, out1, start, q);
    store_block_tail(s, out0, out1, start, q, remaining);
}

impl Squeeze2<uint64x2_t> for KeccakState<2, uint64x2_t> {
    fn squeeze2<const RATE: usize>(
        &self,
        out0: &mut [u8],
        out1: &mut [u8],
        start: usize,
        len: usize,
    ) {
        store_block::<RATE>(&self.st, out0, out1, start, len);
    }
}

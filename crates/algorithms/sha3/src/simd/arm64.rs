use libcrux_intrinsics::arm64::*;

use crate::{generic_keccak::KeccakState, traits::*};
use libcrux_secrets::Zeroize;

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

#[inline(always)]
pub(crate) fn load_block<const RATE: usize>(
    s: &mut [uint64x2_t; 25],
    blocks: &[&[u8]; 2],
    offset: usize,
) {
    #[cfg(not(eurydice))]
    debug_assert!(RATE <= blocks[0].len() && RATE % 8 == 0 && blocks[0].len() == blocks[1].len());
    for i in 0..RATE / 16 {
        let start = offset + 16 * i;
        let v0 = _vld1q_bytes_u64(&blocks[0][start..start + 16]);
        let v1 = _vld1q_bytes_u64(&blocks[1][start..start + 16]);
        let i0 = (2 * i) / 5;
        let j0 = (2 * i) % 5;
        let i1 = (2 * i + 1) / 5;
        let j1 = (2 * i + 1) % 5;
        set_ij(
            s,
            i0,
            j0,
            _veorq_u64(*get_ij(s, i0, j0), _vtrn1q_u64(v0, v1)),
        );
        set_ij(
            s,
            i1,
            j1,
            _veorq_u64(*get_ij(s, i1, j1), _vtrn2q_u64(v0, v1)),
        );
    }
    if RATE % 16 != 0 {
        let i = RATE / 8 - 1;
        let mut u = [0u64; 2];
        let start = offset + RATE - 8;
        u[0] = u64::from_le_bytes(blocks[0][start..start + 8].try_into().unwrap());
        u[1] = u64::from_le_bytes(blocks[1][start..start + 8].try_into().unwrap());
        let uvec = _vld1q_u64(&u);
        set_ij(s, i / 5, i % 5, _veorq_u64(*get_ij(s, i / 5, i % 5), uvec));
        // Not wiped: a copy of the input, which the caller still holds. Wiping
        // it here, once per block, costs up to 20% (see the README).
    }
}

#[inline(always)]
pub(crate) fn load_last<const RATE: usize, const DELIMITER: u8>(
    state: &mut [uint64x2_t; 25],
    blocks: &[&[u8]; 2],
    offset: usize,
    len: usize,
) {
    #[cfg(not(eurydice))]
    debug_assert!(offset + len <= blocks[0].len() && blocks[0].len() == blocks[1].len());

    let mut buffer0 = [0u8; RATE];
    buffer0[0..len].copy_from_slice(&blocks[0][offset..offset + len]);
    buffer0[len] = DELIMITER;
    buffer0[RATE - 1] |= 0x80;

    let mut buffer1 = [0u8; RATE];
    buffer1[0..len].copy_from_slice(&blocks[1][offset..offset + len]);
    buffer1[len] = DELIMITER;
    buffer1[RATE - 1] |= 0x80;

    load_block::<RATE>(state, &[&buffer0, &buffer1], 0);
    buffer0.zeroize();
    buffer1.zeroize();
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
    for i in 0..len / 16 {
        let i0 = (2 * i) / 5;
        let j0 = (2 * i) % 5;
        let i1 = (2 * i + 1) / 5;
        let j1 = (2 * i + 1) % 5;
        let v0 = _vtrn1q_u64(*get_ij(s, i0, j0), *get_ij(s, i1, j1));
        let v1 = _vtrn2q_u64(*get_ij(s, i0, j0), *get_ij(s, i1, j1));
        _vst1q_bytes_u64(&mut out0[start + 16 * i..start + 16 * (i + 1)], v0);
        _vst1q_bytes_u64(&mut out1[start + 16 * i..start + 16 * (i + 1)], v1);
    }
    let remaining = len % 16;
    if remaining > 8 {
        let mut out0_tmp = [0u8; 16];
        let mut out1_tmp = [0u8; 16];
        let i = 2 * (len / 16);
        let i0 = i / 5;
        let j0 = i % 5;
        let i1 = (i + 1) / 5;
        let j1 = (i + 1) % 5;
        let v0 = _vtrn1q_u64(*get_ij(s, i0, j0), *get_ij(s, i1, j1));
        let v1 = _vtrn2q_u64(*get_ij(s, i0, j0), *get_ij(s, i1, j1));
        _vst1q_bytes_u64(&mut out0_tmp, v0);
        _vst1q_bytes_u64(&mut out1_tmp, v1);
        out0[start + len - remaining..start + len].copy_from_slice(&out0_tmp[0..remaining]);
        out1[start + len - remaining..start + len].copy_from_slice(&out1_tmp[0..remaining]);
        out0_tmp.zeroize();
        out1_tmp.zeroize();
    } else if remaining > 0 {
        let mut out01 = [0u8; 16];
        let i = 2 * (len / 16);
        _vst1q_bytes_u64(&mut out01, *get_ij(s, i / 5, i % 5));
        out0[start + len - remaining..start + len].copy_from_slice(&out01[0..remaining]);
        out1[start + len - remaining..start + len].copy_from_slice(&out01[8..8 + remaining]);
        out01.zeroize();
    }
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

impl Absorb<2> for KeccakState<2, uint64x2_t> {
    fn load_block<const RATE: usize>(&mut self, input: &[&[u8]; 2], start: usize) {
        load_block::<RATE>(&mut self.st, input, start);
    }

    fn load_last<const RATE: usize, const DELIMITER: u8>(
        &mut self,
        input: &[&[u8]; 2],
        start: usize,
        len: usize,
    ) {
        load_last::<RATE, DELIMITER>(&mut self.st, input, start, len);
    }
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

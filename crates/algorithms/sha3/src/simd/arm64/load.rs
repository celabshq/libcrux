use libcrux_intrinsics::arm64::*;

use crate::generic_keccak::KeccakState;
use crate::traits::{get_ij, set_ij, Absorb};

use super::wrappers::uint64x2_t;

#[inline(always)]
fn load_u64x2(blocks: &[&[u8]; 2], offset: usize, i: usize, statei: uint64x2_t) -> uint64x2_t {
    let mut u = [0u64; 2];
    u[0] = u64::from_le_bytes(
        blocks[0][offset + 8 * i..offset + 8 * i + 8]
            .try_into()
            .unwrap(),
    );
    u[1] = u64::from_le_bytes(
        blocks[1][offset + 8 * i..offset + 8 * i + 8]
            .try_into()
            .unwrap(),
    );
    let uvec = _vld1q_u64(&u);
    _veorq_u64(statei, uvec)
}

#[inline(always)]
fn load_u64x2x2(
    blocks: &[&[u8]; 2],
    offset: usize,
    i: usize,
    in0: uint64x2_t,
    in1: uint64x2_t,
) -> (uint64x2_t, uint64x2_t) {
    let v0 = _vld1q_bytes_u64(&blocks[0][offset + 16 * i..offset + 16 * i + 16]);
    let v1 = _vld1q_bytes_u64(&blocks[1][offset + 16 * i..offset + 16 * i + 16]);
    (
        _veorq_u64(in0, _vtrn1q_u64(v0, v1)),
        _veorq_u64(in1, _vtrn2q_u64(v0, v1)),
    )
}

#[inline(always)]
pub(crate) fn load_block<const RATE: usize>(
    state: &mut [uint64x2_t; 25],
    blocks: &[&[u8]; 2],
    offset: usize,
) {
    for i in 0..RATE / 16 {
        let i0 = (2 * i) / 5;
        let j0 = (2 * i) % 5;
        let i1 = (2 * i + 1) / 5;
        let j1 = (2 * i + 1) % 5;
        let (v0, v1) = load_u64x2x2(
            blocks,
            offset,
            i,
            *get_ij(state, i0, j0),
            *get_ij(state, i1, j1),
        );
        set_ij(state, i0, j0, v0);
        set_ij(state, i1, j1, v1);
    }
    let remaining = RATE % 16;
    if remaining > 0 {
        let i = RATE / 8 - 1;
        let result = load_u64x2(blocks, offset, i, *get_ij(state, i / 5, i % 5));
        set_ij(state, i / 5, i % 5, result);
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

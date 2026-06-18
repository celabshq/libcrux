//! Arm64 (NEON) block loads and the `Absorb<2>` impl.

use libcrux_intrinsics::arm64::*;

use crate::generic_keccak::KeccakState;
use crate::traits::{get_ij, set_ij, Absorb};

use super::wrappers::uint64x2_t;

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

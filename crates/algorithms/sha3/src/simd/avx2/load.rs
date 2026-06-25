//! AVX2 block loads and the `Absorb<4>` impl.

use libcrux_intrinsics::avx2::*;

use crate::generic_keccak::KeccakState;
use crate::traits::{get_ij, set_ij, Absorb};

#[inline(always)]
pub(crate) fn load_block<const RATE: usize>(
    state: &mut [Vec256; 25],
    blocks: &[&[u8]; 4],
    offset: usize,
) {
    #[cfg(not(eurydice))]
    debug_assert!(RATE <= blocks[0].len() && RATE % 8 == 0 && (RATE % 32 == 8 || RATE % 32 == 16));
    for i in 0..RATE / 32 {
        let start = offset + 32 * i;
        let v0 = mm256_loadu_si256_u8(&blocks[0][start..start + 32]);
        let v1 = mm256_loadu_si256_u8(&blocks[1][start..start + 32]);
        let v2 = mm256_loadu_si256_u8(&blocks[2][start..start + 32]);
        let v3 = mm256_loadu_si256_u8(&blocks[3][start..start + 32]);

        let v0l = mm256_unpacklo_epi64(v0, v1); // 0 0 2 2
        let v1h = mm256_unpackhi_epi64(v0, v1); // 1 1 3 3
        let v2l = mm256_unpacklo_epi64(v2, v3); // 0 0 2 2
        let v3h = mm256_unpackhi_epi64(v2, v3); // 1 1 3 3

        let v0 = mm256_permute2x128_si256::<0x20>(v0l, v2l); // 0 0 0 0
        let v1 = mm256_permute2x128_si256::<0x20>(v1h, v3h); // 1 1 1 1
        let v2 = mm256_permute2x128_si256::<0x31>(v0l, v2l); // 2 2 2 2
        let v3 = mm256_permute2x128_si256::<0x31>(v1h, v3h); // 3 3 3 3

        let i0 = (4 * i) / 5;
        let j0 = (4 * i) % 5;
        let i1 = (4 * i + 1) / 5;
        let j1 = (4 * i + 1) % 5;
        let i2 = (4 * i + 2) / 5;
        let j2 = (4 * i + 2) % 5;
        let i3 = (4 * i + 3) / 5;
        let j3 = (4 * i + 3) % 5;

        set_ij(state, i0, j0, mm256_xor_si256(*get_ij(state, i0, j0), v0));
        set_ij(state, i1, j1, mm256_xor_si256(*get_ij(state, i1, j1), v1));
        set_ij(state, i2, j2, mm256_xor_si256(*get_ij(state, i2, j2), v2));
        set_ij(state, i3, j3, mm256_xor_si256(*get_ij(state, i3, j3), v3));
    }

    let rem = RATE % 32; // has to be 8 or 16
    let start = offset + 32 * (RATE / 32);
    let mut u8s = [0u8; 32];
    u8s[0..8].copy_from_slice(&blocks[0][start..start + 8]);
    u8s[8..16].copy_from_slice(&blocks[1][start..start + 8]);
    u8s[16..24].copy_from_slice(&blocks[2][start..start + 8]);
    u8s[24..32].copy_from_slice(&blocks[3][start..start + 8]);
    let u = mm256_loadu_si256_u8(u8s.as_slice());
    let i = (4 * (RATE / 32)) / 5;
    let j = (4 * (RATE / 32)) % 5;
    set_ij(state, i, j, mm256_xor_si256(*get_ij(state, i, j), u));
    if rem == 16 {
        let mut u8s = [0u8; 32];
        u8s[0..8].copy_from_slice(&blocks[0][start + 8..start + 16]);
        u8s[8..16].copy_from_slice(&blocks[1][start + 8..start + 16]);
        u8s[16..24].copy_from_slice(&blocks[2][start + 8..start + 16]);
        u8s[24..32].copy_from_slice(&blocks[3][start + 8..start + 16]);
        let u = mm256_loadu_si256_u8(u8s.as_slice());
        let i = (4 * (RATE / 32) + 1) / 5;
        let j = (4 * (RATE / 32) + 1) % 5;
        set_ij(state, i, j, mm256_xor_si256(*get_ij(state, i, j), u));
    }
}

#[inline(always)]
pub(crate) fn load_last<const RATE: usize, const DELIMITER: u8>(
    state: &mut [Vec256; 25],
    blocks: &[&[u8]; 4],
    start: usize,
    len: usize,
) {
    let mut buffers = [[0u8; RATE]; 4];
    for i in 0..4 {
        buffers[i][0..len].copy_from_slice(&blocks[i][start..start + len]);
        buffers[i][len] = DELIMITER;
        buffers[i][RATE - 1] |= 0x80;
    }

    load_block::<RATE>(
        state,
        &[
            &buffers[0] as &[u8],
            &buffers[1] as &[u8],
            &buffers[2] as &[u8],
            &buffers[3] as &[u8],
        ],
        0,
    );
}

impl Absorb<4> for KeccakState<4, Vec256> {
    fn load_block<const RATE: usize>(&mut self, input: &[&[u8]; 4], start: usize) {
        load_block::<RATE>(&mut self.st, input, start);
    }

    fn load_last<const RATE: usize, const DELIMITER: u8>(
        &mut self,
        input: &[&[u8]; 4],
        start: usize,
        len: usize,
    ) {
        load_last::<RATE, DELIMITER>(&mut self.st, input, start, len)
    }
}

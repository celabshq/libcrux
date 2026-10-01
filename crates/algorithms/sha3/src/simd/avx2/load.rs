use libcrux_intrinsics::avx2::*;

use crate::generic_keccak::KeccakState;
use crate::traits::{get_ij, set_ij, Absorb};

/// Bulk-block load helper (mirrors arm64::load_u64x2x2 at N=4).
/// Loads 32 bytes from each of the 4 blocks at `offset + 32*i`,
/// gathers them via unpack/permute into 4 Vec256s, each holding the
/// `(4*i + idx)`th u64 from each block in lane `lane`, then XORs
/// with the corresponding state inputs `inK`.
#[inline(always)]
fn load_u64x4x4(
    blocks: &[&[u8]; 4],
    offset: usize,
    i: usize,
    in0: Vec256,
    in1: Vec256,
    in2: Vec256,
    in3: Vec256,
) -> (Vec256, Vec256, Vec256, Vec256) {
    let start = offset + 32 * i;
    let v0 = mm256_loadu_si256_u8(&blocks[0][start..start + 32]);
    let v1 = mm256_loadu_si256_u8(&blocks[1][start..start + 32]);
    let v2 = mm256_loadu_si256_u8(&blocks[2][start..start + 32]);
    let v3 = mm256_loadu_si256_u8(&blocks[3][start..start + 32]);

    let v0l = mm256_unpacklo_epi64(v0, v1);
    let v1h = mm256_unpackhi_epi64(v0, v1);
    let v2l = mm256_unpacklo_epi64(v2, v3);
    let v3h = mm256_unpackhi_epi64(v2, v3);

    let g0 = mm256_permute2x128_si256::<0x20>(v0l, v2l);
    let g1 = mm256_permute2x128_si256::<0x20>(v1h, v3h);
    let g2 = mm256_permute2x128_si256::<0x31>(v0l, v2l);
    let g3 = mm256_permute2x128_si256::<0x31>(v1h, v3h);

    (
        mm256_xor_si256(in0, g0),
        mm256_xor_si256(in1, g1),
        mm256_xor_si256(in2, g2),
        mm256_xor_si256(in3, g3),
    )
}

/// Partial-block load helper (mirrors arm64::load_u64x2 at N=4).
/// Loads 8 bytes from each of the 4 blocks at `offset + 8*i`,
/// gathers them into a Vec256, and XORs with `statei`.
#[inline(always)]
fn load_u64x4(blocks: &[&[u8]; 4], offset: usize, i: usize, statei: Vec256) -> Vec256 {
    let v0 = u64::from_le_bytes(
        blocks[0][offset + 8 * i..offset + 8 * i + 8]
            .try_into()
            .unwrap(),
    ) as i64;
    let v1 = u64::from_le_bytes(
        blocks[1][offset + 8 * i..offset + 8 * i + 8]
            .try_into()
            .unwrap(),
    ) as i64;
    let v2 = u64::from_le_bytes(
        blocks[2][offset + 8 * i..offset + 8 * i + 8]
            .try_into()
            .unwrap(),
    ) as i64;
    let v3 = u64::from_le_bytes(
        blocks[3][offset + 8 * i..offset + 8 * i + 8]
            .try_into()
            .unwrap(),
    ) as i64;
    let u = mm256_set_epi64x(v3, v2, v1, v0);
    mm256_xor_si256(statei, u)
}

#[inline(always)]
pub(crate) fn load_block<const RATE: usize>(
    state: &mut [Vec256; 25],
    blocks: &[&[u8]; 4],
    offset: usize,
) {
    #[cfg(not(eurydice))]
    debug_assert!(
        RATE <= blocks[0].len()
            && RATE / 32 <= 6
            && 32 * (RATE / 32 - 1) + 32 <= RATE
            && RATE % 8 == 0
            && (RATE % 32 == 8 || RATE % 32 == 16)
    );
    for i in 0..RATE / 32 {
        let i0 = (4 * i) / 5;
        let j0 = (4 * i) % 5;
        let i1 = (4 * i + 1) / 5;
        let j1 = (4 * i + 1) % 5;
        let i2 = (4 * i + 2) / 5;
        let j2 = (4 * i + 2) % 5;
        let i3 = (4 * i + 3) / 5;
        let j3 = (4 * i + 3) % 5;
        let (g0, g1, g2, g3) = load_u64x4x4(
            blocks,
            offset,
            i,
            *get_ij(state, i0, j0),
            *get_ij(state, i1, j1),
            *get_ij(state, i2, j2),
            *get_ij(state, i3, j3),
        );
        set_ij(state, i0, j0, g0);
        set_ij(state, i1, j1, g1);
        set_ij(state, i2, j2, g2);
        set_ij(state, i3, j3, g3);
    }
    let rem = RATE % 32; // has to be 8 or 16
    let i = 4 * (RATE / 32);
    let result = load_u64x4(blocks, offset, i, *get_ij(state, i / 5, i % 5));
    set_ij(state, i / 5, i % 5, result);
    if rem == 16 {
        let i = 4 * (RATE / 32) + 1;
        let result = load_u64x4(blocks, offset, i, *get_ij(state, i / 5, i % 5));
        set_ij(state, i / 5, i % 5, result);
    }
}

#[inline(always)]
pub(crate) fn load_last<const RATE: usize, const DELIMITER: u8>(
    state: &mut [Vec256; 25],
    blocks: &[&[u8]; 4],
    start: usize,
    len: usize,
) {
    let mut buffer0 = [0u8; RATE];
    buffer0[0..len].copy_from_slice(&blocks[0][start..start + len]);
    buffer0[len] = DELIMITER;
    buffer0[RATE - 1] |= 0x80;

    let mut buffer1 = [0u8; RATE];
    buffer1[0..len].copy_from_slice(&blocks[1][start..start + len]);
    buffer1[len] = DELIMITER;
    buffer1[RATE - 1] |= 0x80;

    let mut buffer2 = [0u8; RATE];
    buffer2[0..len].copy_from_slice(&blocks[2][start..start + len]);
    buffer2[len] = DELIMITER;
    buffer2[RATE - 1] |= 0x80;

    let mut buffer3 = [0u8; RATE];
    buffer3[0..len].copy_from_slice(&blocks[3][start..start + len]);
    buffer3[len] = DELIMITER;
    buffer3[RATE - 1] |= 0x80;

    load_block::<RATE>(state, &[&buffer0, &buffer1, &buffer2, &buffer3], 0);
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

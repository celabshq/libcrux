//! AVX2 block stores and the `Squeeze4` impl.

use libcrux_intrinsics::avx2::*;

use crate::generic_keccak::KeccakState;
use crate::traits::{get_ij, Squeeze4};

#[inline(always)]
pub(crate) fn store_block<const RATE: usize>(
    s: &[Vec256; 25],
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    start: usize,
    len: usize,
) {
    let chunks = len / 32;
    for i in 0..chunks {
        let i0 = (4 * i) / 5;
        let j0 = (4 * i) % 5;
        let i1 = (4 * i + 1) / 5;
        let j1 = (4 * i + 1) % 5;
        let i2 = (4 * i + 2) / 5;
        let j2 = (4 * i + 2) % 5;
        let i3 = (4 * i + 3) / 5;
        let j3 = (4 * i + 3) % 5;

        let v0l = mm256_permute2x128_si256::<0x20>(*get_ij(s, i0, j0), *get_ij(s, i2, j2));
        // 0 0 2 2
        let v1h = mm256_permute2x128_si256::<0x20>(*get_ij(s, i1, j1), *get_ij(s, i3, j3)); // 1 1 3 3
        let v2l = mm256_permute2x128_si256::<0x31>(*get_ij(s, i0, j0), *get_ij(s, i2, j2)); // 0 0 2 2
        let v3h = mm256_permute2x128_si256::<0x31>(*get_ij(s, i1, j1), *get_ij(s, i3, j3)); // 1 1 3 3

        let v0 = mm256_unpacklo_epi64(v0l, v1h); // 0 1 2 3
        let v1 = mm256_unpackhi_epi64(v0l, v1h); // 0 1 2 3
        let v2 = mm256_unpacklo_epi64(v2l, v3h); // 0 1 2 3
        let v3 = mm256_unpackhi_epi64(v2l, v3h); // 0 1 2 3

        mm256_storeu_si256_u8(&mut out0[start + 32 * i..start + 32 * (i + 1)], v0);
        mm256_storeu_si256_u8(&mut out1[start + 32 * i..start + 32 * (i + 1)], v1);
        mm256_storeu_si256_u8(&mut out2[start + 32 * i..start + 32 * (i + 1)], v2);
        mm256_storeu_si256_u8(&mut out3[start + 32 * i..start + 32 * (i + 1)], v3);
    }

    let rem = len % 32;
    if rem > 0 {
        let offset = start + 32 * chunks;
        let mut u8s = [0u8; 32];
        let chunks8 = rem / 8;
        for k in 0..chunks8 {
            let i = (4 * chunks + k) / 5;
            let j = (4 * chunks + k) % 5;
            mm256_storeu_si256_u8(&mut u8s, *get_ij(s, i, j));
            out0[offset + 8 * k..offset + 8 * (k + 1)].copy_from_slice(&u8s[0..8]);
            out1[offset + 8 * k..offset + 8 * (k + 1)].copy_from_slice(&u8s[8..16]);
            out2[offset + 8 * k..offset + 8 * (k + 1)].copy_from_slice(&u8s[16..24]);
            out3[offset + 8 * k..offset + 8 * (k + 1)].copy_from_slice(&u8s[24..32]);
        }
        let rem8 = rem % 8;
        let offset_rem8 = offset + chunks8 * 8;
        if rem8 > 0 {
            let i = (4 * chunks + chunks8) / 5;
            let j = (4 * chunks + chunks8) % 5;
            mm256_storeu_si256_u8(&mut u8s, *get_ij(s, i, j));
            out0[offset_rem8..offset_rem8 + rem8].copy_from_slice(&u8s[0..rem8]);
            out1[offset_rem8..offset_rem8 + rem8].copy_from_slice(&u8s[8..8 + rem8]);
            out2[offset_rem8..offset_rem8 + rem8].copy_from_slice(&u8s[16..16 + rem8]);
            out3[offset_rem8..offset_rem8 + rem8].copy_from_slice(&u8s[24..24 + rem8]);
        }
    }
}

impl Squeeze4<Vec256> for KeccakState<4, Vec256> {
    fn squeeze4<const RATE: usize>(
        &self,
        out0: &mut [u8],
        out1: &mut [u8],
        out2: &mut [u8],
        out3: &mut [u8],
        start: usize,
        len: usize,
    ) {
        store_block::<RATE>(&self.st, out0, out1, out2, out3, start, len)
    }
}

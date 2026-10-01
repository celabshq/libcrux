use libcrux_intrinsics::avx2::*;

use crate::generic_keccak::KeccakState;
use crate::traits::{get_ij, Squeeze4};

/// Per-iteration store wrapper for `store_block_full_avx2`. Given the
/// four state vectors `s0..s3` (= `s[4*i + 0..4*i + 3]`), the four
/// permute2x128 + two unpacklo/unpackhi pass deinterleaves them into four
/// output streams `v_m`, each whose lane `k` corresponds to lane `m` of
/// `s_k`. Four `mm256_storeu_si256_u8` stores then write a 32-byte window
/// per buffer.
#[inline(always)]
#[allow(unused_variables)]
fn store_u64x4x4(
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    s: &[Vec256; 25],
    s0: Vec256,
    s1: Vec256,
    s2: Vec256,
    s3: Vec256,
    start: usize,
    i: usize,
) {
    let v0l = mm256_permute2x128_si256::<0x20>(s0, s2);
    let v1h = mm256_permute2x128_si256::<0x20>(s1, s3);
    let v2l = mm256_permute2x128_si256::<0x31>(s0, s2);
    let v3h = mm256_permute2x128_si256::<0x31>(s1, s3);
    let v0 = mm256_unpacklo_epi64(v0l, v1h);
    let v1 = mm256_unpackhi_epi64(v0l, v1h);
    let v2 = mm256_unpacklo_epi64(v2l, v3h);
    let v3 = mm256_unpackhi_epi64(v2l, v3h);
    mm256_storeu_si256_u8(&mut out0[start + 32 * i..start + 32 * (i + 1)], v0);
    mm256_storeu_si256_u8(&mut out1[start + 32 * i..start + 32 * (i + 1)], v1);
    mm256_storeu_si256_u8(&mut out2[start + 32 * i..start + 32 * (i + 1)], v2);
    mm256_storeu_si256_u8(&mut out3[start + 32 * i..start + 32 * (i + 1)], v3);
}

/// Inner-loop leaf (8-byte chunk) for the tail. Writes the 8-byte window
/// `[off, off+8)` (`off = start+32*q+8*k`) of each output from lane `m` of
/// `vec` (= `s[4*q+k]`).
#[inline(always)]
#[allow(unused_variables)]
fn store_chunk8x4(
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    vec: Vec256,
    s: &[Vec256; 25],
    start: usize,
    q: usize,
    k: usize,
) {
    let mut u8s = [0u8; 32];
    mm256_storeu_si256_u8(&mut u8s, vec);
    let off = start + 32 * q + 8 * k;
    out0[off..off + 8].copy_from_slice(&u8s[0..8]);
    out1[off..off + 8].copy_from_slice(&u8s[8..16]);
    out2[off..off + 8].copy_from_slice(&u8s[16..24]);
    out3[off..off + 8].copy_from_slice(&u8s[24..32]);
}

/// Ragged leaf for the tail's final `rem8 < 8` bytes. Writes
/// `[off, off+rem8)` (`off = start+32*q+8*chunks8`) of each output from
/// lane `m` of `vec` (= `s[4*q+chunks8]`).
#[inline(always)]
#[allow(unused_variables)]
fn store_tail_ragged_avx2(
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    vec: Vec256,
    s: &[Vec256; 25],
    start: usize,
    q: usize,
    chunks8: usize,
    rem8: usize,
) {
    let mut u8s = [0u8; 32];
    mm256_storeu_si256_u8(&mut u8s, vec);
    let off = start + 32 * q + 8 * chunks8;
    out0[off..off + rem8].copy_from_slice(&u8s[0..rem8]);
    out1[off..off + rem8].copy_from_slice(&u8s[8..8 + rem8]);
    out2[off..off + rem8].copy_from_slice(&u8s[16..16 + rem8]);
    out3[off..off + rem8].copy_from_slice(&u8s[24..24 + rem8]);
}

/// Outer-loop half of `store_block`: writes the full 32-byte windows
/// `[start, start+32*q)` by calling `store_u64x4x4` per iteration.
#[inline(always)]
fn store_block_full_avx2(
    s: &[Vec256; 25],
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    start: usize,
    q: usize,
) {
    for i in 0..q {
        store_u64x4x4(
            out0,
            out1,
            out2,
            out3,
            s,
            *get_ij(s, (4 * i) / 5, (4 * i) % 5),
            *get_ij(s, (4 * i + 1) / 5, (4 * i + 1) % 5),
            *get_ij(s, (4 * i + 2) / 5, (4 * i + 2) % 5),
            *get_ij(s, (4 * i + 3) / 5, (4 * i + 3) % 5),
            start,
            i,
        );
    }
}

/// Tail half of `store_block`: writes the partial window
/// `[start+32*q, start+32*q+rem)` (`rem < 32`) via the inner 8-byte
/// loop (`store_chunk8x4`) and the ragged remainder
/// (`store_tail_ragged_avx2`).
#[inline(always)]
fn store_block_tail_avx2(
    s: &[Vec256; 25],
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
    start: usize,
    q: usize,
    rem: usize,
) {
    let chunks8 = rem / 8;
    for k in 0..chunks8 {
        store_chunk8x4(
            out0,
            out1,
            out2,
            out3,
            *get_ij(s, (4 * q + k) / 5, (4 * q + k) % 5),
            s,
            start,
            q,
            k,
        );
    }
    let rem8 = rem % 8;
    if rem8 > 0 {
        store_tail_ragged_avx2(
            out0,
            out1,
            out2,
            out3,
            *get_ij(s, (4 * q + chunks8) / 5, (4 * q + chunks8) % 5),
            s,
            start,
            q,
            chunks8,
            rem8,
        );
    }
}

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
    let rem = len % 32;
    store_block_full_avx2(s, out0, out1, out2, out3, start, chunks);
    store_block_tail_avx2(s, out0, out1, out2, out3, start, chunks, rem);
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

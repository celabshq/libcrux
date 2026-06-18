//! Arm64 (NEON) block stores and the `Squeeze2` impl.

use libcrux_intrinsics::arm64::*;

use crate::generic_keccak::KeccakState;
use crate::traits::{get_ij, Squeeze2};

use super::wrappers::uint64x2_t;

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
    } else if remaining > 0 {
        let mut out01 = [0u8; 16];
        let i = 2 * (len / 16);
        _vst1q_bytes_u64(&mut out01, *get_ij(s, i / 5, i % 5));
        out0[start + len - remaining..start + len].copy_from_slice(&out01[0..remaining]);
        out1[start + len - remaining..start + len].copy_from_slice(&out01[8..8 + remaining]);
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

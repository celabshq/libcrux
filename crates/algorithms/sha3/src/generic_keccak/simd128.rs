use super::*;

use libcrux_intrinsics::arm64::_uint64x2_t;

/// Absorb phase of `keccak2`: initialise a two-lane Keccak state,
/// absorb all full rate-byte blocks of `data[0]` and `data[1]` in
/// parallel, then pad and absorb each lane's final partial block
/// with domain-separation byte `DELIM` and the pad10*1 terminator.
#[inline]
pub(crate) fn absorb2<const RATE: usize, const DELIM: u8>(
    data: &[&[u8]; 2],
) -> KeccakState<2, _uint64x2_t> {
    let mut s = KeccakState::<2, _uint64x2_t>::new();
    let data_len = data[0].len();
    let data_blocks = data_len / RATE;
    let rem = data_len % RATE;
    for i in 0..data_blocks {
        s.absorb_block::<RATE>(data, i * RATE);
    }
    s.absorb_final::<RATE, DELIM>(data, data_len - rem, rem);
    s
}

#[inline]
fn squeeze2_blocks<const RATE: usize>(
    s: &mut KeccakState<2, _uint64x2_t>,
    out0: &mut [u8],
    out1: &mut [u8],
    blocks: usize,
) {
    s.squeeze2::<RATE>(out0, out1, 0, RATE);
    for i in 1..blocks {
        s.keccakf1600();
        s.squeeze2::<RATE>(out0, out1, i * RATE, RATE);
    }
}

/// Squeeze phase of `keccak2`: extract `out0.len()` bytes from each
/// lane of `s` into `out0` and `out1`, applying Keccak-f between
/// each full rate-byte block of output.
#[inline]
pub(crate) fn squeeze2<const RATE: usize>(
    mut s: KeccakState<2, _uint64x2_t>,
    out0: &mut [u8],
    out1: &mut [u8],
) {
    let outlen = out0.len();
    let blocks = outlen / RATE;
    let last = outlen - (outlen % RATE);

    if blocks == 0 {
        s.squeeze2::<RATE>(out0, out1, 0, outlen);
    } else {
        squeeze2_blocks::<RATE>(&mut s, out0, out1, blocks);
        if last < outlen {
            s.keccakf1600();
            s.squeeze2::<RATE>(out0, out1, last, outlen - last);
        }
    }
}

#[inline]
pub(crate) fn keccak2<const RATE: usize, const DELIM: u8>(
    data: &[&[u8]; 2],
    out0: &mut [u8],
    out1: &mut [u8],
) {
    #[cfg(not(eurydice))]
    debug_assert!(out0.len() == out1.len());
    #[cfg(not(eurydice))]
    debug_assert!(data[0].len() == data[1].len());

    let s = absorb2::<RATE, DELIM>(data);
    squeeze2::<RATE>(s, out0, out1);
}

impl KeccakState<2, _uint64x2_t> {
    #[inline(always)]
    pub(crate) fn squeeze_next_block<const RATE: usize>(
        &mut self,
        out0: &mut [u8],
        out1: &mut [u8],
        start: usize,
    ) {
        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, start, RATE);
    }

    /// Write out the first block of Keccak output.
    ///
    /// This function MUST NOT be called after any of the other `squeeze_*`
    /// functions have been called, since that would result in a duplicate output
    /// block.
    pub(crate) fn squeeze_first_block<const RATE: usize>(&self, out0: &mut [u8], out1: &mut [u8]) {
        self.squeeze2::<RATE>(out0, out1, 0, RATE);
    }

    #[inline(always)]
    pub(crate) fn squeeze_first_three_blocks<const RATE: usize>(
        &mut self,
        out0: &mut [u8],
        out1: &mut [u8],
    ) {
        self.squeeze2::<RATE>(out0, out1, 0, RATE);

        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, RATE, RATE);

        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, 2 * RATE, RATE);
    }

    #[inline(always)]
    pub(crate) fn squeeze_first_five_blocks<const RATE: usize>(
        &mut self,
        out0: &mut [u8],
        out1: &mut [u8],
    ) {
        self.squeeze2::<RATE>(out0, out1, 0, RATE);

        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, RATE, RATE);

        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, 2 * RATE, RATE);

        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, 3 * RATE, RATE);

        self.keccakf1600();
        self.squeeze2::<RATE>(out0, out1, 4 * RATE, RATE);
    }
}

use super::*;

use libcrux_intrinsics::arm64::_uint64x2_t;

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

    let mut s = KeccakState::<2, _uint64x2_t>::new();
    let data_len = data[0].len();
    for i in 0..data_len / RATE {
        s.absorb_block::<RATE>(data, i * RATE);
    }
    let rem = data_len % RATE;
    s.absorb_final::<RATE, DELIM>(data, data_len - rem, rem);

    let outlen = out0.len();
    let blocks = outlen / RATE;
    let last = outlen - (outlen % RATE);

    if blocks == 0 {
        s.squeeze2::<RATE>(out0, out1, 0, outlen);
    } else {
        s.squeeze2::<RATE>(out0, out1, 0, RATE);
        for i in 1..blocks {
            s.keccakf1600();
            s.squeeze2::<RATE>(out0, out1, i * RATE, RATE);
        }
        if last < outlen {
            s.keccakf1600();
            s.squeeze2::<RATE>(out0, out1, last, outlen - last);
        }
    }
    // Not wiped: taking the address of `s` keeps LLVM from holding the state
    // in NEON registers, which made hashing up to ~20% slower (see the README).
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

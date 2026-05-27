use super::*;

use libcrux_intrinsics::arm64::_uint64x2_t;

#[cfg(hax)]
use crate::proof_utils::{lemma_mul_succ_le, valid_rate};

#[hax_lib::attributes]
impl KeccakState<2, _uint64x2_t> {
    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        start.to_int() + RATE.to_int() <= out0.len().to_int() &&
        out0.len() == out1.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len()
    )]
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
    #[hax_lib::requires(
        valid_rate(RATE) &&
        RATE <= out0.len() &&
        out0.len() == out1.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len()
    )]
    pub(crate) fn squeeze_first_block<const RATE: usize>(&self, out0: &mut [u8], out1: &mut [u8]) {
        self.squeeze2::<RATE>(out0, out1, 0, RATE);
    }

    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        3 * RATE <= out0.len() &&
        out0.len() == out1.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len()
    )]
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
    #[hax_lib::requires(
        valid_rate(RATE) &&
        5 * RATE <= out0.len() &&
        out0.len() == out1.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len()
    )]
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

#[hax_lib::requires(
    valid_rate(RATE) &&
    data[0].len() == data[1].len() &&
    out0.len() == out1.len()
)]
#[hax_lib::ensures(|_|
    future(out0).len() == out0.len() &&
    future(out1).len() == out1.len()
)]
#[hax_lib::fstar::options("--split_queries always --z3rlimit 300")]
#[inline]
pub(crate) fn keccak2<const RATE: usize, const DELIM: u8>(
    data: &[&[u8]; 2],
    out0: &mut [u8],
    out1: &mut [u8],
) {
    debug_assert!(data[0].len() == data[1].len());
    debug_assert!(out0.len() == out1.len());

    // Initialize Keccak state
    let mut s = KeccakState::<2, _uint64x2_t>::new();

    // Absorb input
    let input_len = data[0].len();
    let input_blocks = input_len / RATE;
    let input_rem = input_len % RATE;
    for i in 0..input_blocks {
        #[cfg(hax)]
        lemma_mul_succ_le(i, input_blocks, RATE);

        s.absorb_block::<RATE>(data, i * RATE);
    }
    s.absorb_final::<RATE, DELIM>(data, input_len - input_rem, input_rem);

    // Squeeze output
    let output_len = out0.len();
    let output_blocks = output_len / RATE;
    let output_rem = output_len % RATE;
    if output_blocks == 0 {
        s.squeeze2::<RATE>(out0, out1, 0, output_len);
    } else {
        s.squeeze2::<RATE>(out0, out1, 0, RATE);
        for i in 1..output_blocks {
            hax_lib::loop_invariant!(
                |_: usize| out0.len() == output_len && out0.len() == out1.len()
            );
            #[cfg(hax)]
            lemma_mul_succ_le(i, output_blocks, RATE);

            s.keccakf1600();
            s.squeeze2::<RATE>(out0, out1, i * RATE, RATE);
        }
        if output_rem != 0 {
            s.keccakf1600();
            s.squeeze2::<RATE>(out0, out1, output_len - output_rem, output_rem);
        }
    }
}

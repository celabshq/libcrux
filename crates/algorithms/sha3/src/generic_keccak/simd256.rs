use super::*;

use libcrux_intrinsics::avx2::Vec256;

#[cfg(hax)]
use crate::proof_utils::{lemma_mul_succ_le, valid_rate};

#[hax_lib::attributes]
impl KeccakState<4, Vec256> {
    #[hax_lib::requires(
        valid_rate(RATE) &&
        start.to_int() + RATE.to_int() <= out0.len().to_int() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len() &&
        future(out2).len() == out2.len() &&
        future(out3).len() == out3.len()
    )]
    #[inline(always)]
    pub(crate) fn squeeze_next_block<const RATE: usize>(
        &mut self,
        out0: &mut [u8],
        out1: &mut [u8],
        out2: &mut [u8],
        out3: &mut [u8],
        start: usize,
    ) {
        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, start, RATE);
    }

    #[hax_lib::requires(
        valid_rate(RATE) &&
        RATE <= out0.len() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len() &&
        future(out2).len() == out2.len() &&
        future(out3).len() == out3.len()
    )]
    #[inline(always)]
    pub(crate) fn squeeze_first_block<const RATE: usize>(
        &self,
        out0: &mut [u8],
        out1: &mut [u8],
        out2: &mut [u8],
        out3: &mut [u8],
    ) {
        self.squeeze4::<RATE>(out0, out1, out2, out3, 0, RATE);
    }

    #[hax_lib::requires(
        valid_rate(RATE) &&
        3 * RATE <= out0.len() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len() &&
        future(out2).len() == out2.len() &&
        future(out3).len() == out3.len()
    )]
    #[inline(always)]
    pub(crate) fn squeeze_first_three_blocks<const RATE: usize>(
        &mut self,
        out0: &mut [u8],
        out1: &mut [u8],
        out2: &mut [u8],
        out3: &mut [u8],
    ) {
        self.squeeze4::<RATE>(out0, out1, out2, out3, 0, RATE);

        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, RATE, RATE);

        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, 2 * RATE, RATE);
    }

    #[hax_lib::requires(
        valid_rate(RATE) &&
        5 * RATE <= out0.len() &&
        out0.len() == out1.len() &&
        out0.len() == out2.len() &&
        out0.len() == out3.len()
    )]
    #[hax_lib::ensures(|_|
        future(out0).len() == out0.len() &&
        future(out1).len() == out1.len() &&
        future(out2).len() == out2.len() &&
        future(out3).len() == out3.len()
    )]
    #[inline(always)]
    pub(crate) fn squeeze_first_five_blocks<const RATE: usize>(
        &mut self,
        out0: &mut [u8],
        out1: &mut [u8],
        out2: &mut [u8],
        out3: &mut [u8],
    ) {
        self.squeeze4::<RATE>(out0, out1, out2, out3, 0, RATE);

        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, RATE, RATE);

        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, 2 * RATE, RATE);

        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, 3 * RATE, RATE);

        self.keccakf1600();
        self.squeeze4::<RATE>(out0, out1, out2, out3, 4 * RATE, RATE);
    }
}

#[hax_lib::requires(
    valid_rate(RATE) &&
    data[0].len() == data[1].len() &&
    data[0].len() == data[2].len() &&
    data[0].len() == data[3].len() &&
    out0.len() == out1.len() &&
    out0.len() == out2.len() &&
    out0.len() == out3.len()
)]
#[hax_lib::ensures(|_|
    future(out0).len() == out0.len() &&
    future(out1).len() == out1.len() &&
    future(out2).len() == out2.len() &&
    future(out3).len() == out3.len()
)]
#[hax_lib::fstar::options("--split_queries always --z3rlimit 300")]
#[inline]
pub(crate) fn keccak4<const RATE: usize, const DELIM: u8>(
    data: &[&[u8]; 4],
    out0: &mut [u8],
    out1: &mut [u8],
    out2: &mut [u8],
    out3: &mut [u8],
) {
    debug_assert!(
        data[0].len() == data[1].len()
            && data[0].len() == data[2].len()
            && data[0].len() == data[3].len()
    );
    debug_assert!(out0.len() == out1.len() && out0.len() == out2.len() && out0.len() == out3.len());

    // Initialize Keccak state
    let mut s = KeccakState::<4, Vec256>::new();

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
        s.squeeze4::<RATE>(out0, out1, out2, out3, 0, output_len);
    } else {
        s.squeeze4::<RATE>(out0, out1, out2, out3, 0, RATE);
        for i in 1..output_blocks {
            hax_lib::loop_invariant!(|_: usize| out0.len() == output_len
                && out0.len() == out1.len()
                && out0.len() == out2.len()
                && out0.len() == out3.len());
            #[cfg(hax)]
            lemma_mul_succ_le(i, output_blocks, RATE);

            s.keccakf1600();
            s.squeeze4::<RATE>(out0, out1, out2, out3, i * RATE, RATE);
        }
        if output_rem != 0 {
            s.keccakf1600();
            s.squeeze4::<RATE>(out0, out1, out2, out3, output_len - output_rem, output_rem);
        }
    }
}

use super::*;

#[cfg(hax)]
use crate::proof_utils::{lemma_mul_succ_le, valid_rate};

#[hax_lib::attributes]
impl KeccakState<1, u64> {
    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        start.to_int() + RATE.to_int() <= out.len().to_int()
    )]
    #[hax_lib::ensures(|_| future(out).len() == out.len())]
    pub(crate) fn squeeze_next_block<const RATE: usize>(&mut self, out: &mut [u8], start: usize) {
        self.keccakf1600();
        self.squeeze::<RATE>(out, start, RATE);
    }

    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        RATE <= out.len()
    )]
    #[hax_lib::ensures(|_| future(out).len() == out.len())]
    pub(crate) fn squeeze_first_block<const RATE: usize>(&self, out: &mut [u8]) {
        self.squeeze::<RATE>(out, 0, RATE);
    }

    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        3 * RATE <= out.len()
    )]
    #[hax_lib::ensures(|_| future(out).len() == out.len())]
    pub(crate) fn squeeze_first_three_blocks<const RATE: usize>(&mut self, out: &mut [u8]) {
        self.squeeze::<RATE>(out, 0, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, RATE, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, 2 * RATE, RATE);
    }

    #[inline(always)]
    #[hax_lib::requires(
        valid_rate(RATE) &&
        5 * RATE <= out.len()
    )]
    #[hax_lib::ensures(|_| future(out).len() == out.len())]
    pub(crate) fn squeeze_first_five_blocks<const RATE: usize>(&mut self, out: &mut [u8]) {
        self.squeeze::<RATE>(out, 0, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, RATE, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, 2 * RATE, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, 3 * RATE, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, 4 * RATE, RATE);
    }
}

#[hax_lib::requires(valid_rate(RATE))]
#[hax_lib::ensures(|_| future(output).len() == output.len())]
#[hax_lib::fstar::options("--split_queries always --z3rlimit 300")]
#[inline]
pub(crate) fn keccak1<const RATE: usize, const DELIM: u8>(input: &[u8], output: &mut [u8]) {
    // Initialize Keccak state
    let mut s = KeccakState::<1, u64>::new();

    // Absorb input
    let input_len = input.len();
    let input_blocks = input_len / RATE;
    let input_rem = input_len % RATE;
    for i in 0..input_blocks {
        #[cfg(hax)]
        lemma_mul_succ_le(i, input_blocks, RATE);

        s.absorb_block::<RATE>(&[input], i * RATE);
    }
    s.absorb_final::<RATE, DELIM>(&[input], input_len - input_rem, input_rem);

    // Squeeze output
    let output_len = output.len();
    let output_blocks = output_len / RATE;
    let output_rem = output_len % RATE;
    if output_blocks == 0 {
        s.squeeze::<RATE>(output, 0, output_len);
    } else {
        s.squeeze::<RATE>(output, 0, RATE);
        for i in 1..output_blocks {
            hax_lib::loop_invariant!(|_: usize| output.len() == output_len);
            #[cfg(hax)]
            lemma_mul_succ_le(i, output_blocks, RATE);

            s.keccakf1600();
            s.squeeze::<RATE>(output, i * RATE, RATE);
        }
        if output_rem != 0 {
            s.keccakf1600();
            s.squeeze::<RATE>(output, output_len - output_rem, output_rem);
        }
    }
}

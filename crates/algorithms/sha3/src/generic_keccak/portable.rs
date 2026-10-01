use super::*;
#[cfg(hax)]
use crate::proof_utils::{lemma_mul_succ_le, valid_rate};

#[cfg_attr(hax, hax_lib::attributes)]
impl KeccakState<1, u64> {
    #[inline(always)]
    #[cfg_attr(hax, hax_lib::requires(
        valid_rate(RATE) &&
        start.to_int() + RATE.to_int() <= out.len().to_int()
    ))]
    #[cfg_attr(hax, hax_lib::ensures(|_| future(out).len() == out.len()))]
    pub(crate) fn squeeze_next_block<const RATE: usize>(&mut self, out: &mut [u8], start: usize) {
        self.keccakf1600();
        self.squeeze::<RATE>(out, start, RATE);
    }

    #[inline(always)]
    #[cfg_attr(hax, hax_lib::requires(
        valid_rate(RATE) &&
        RATE <= out.len()
    ))]
    #[cfg_attr(hax, hax_lib::ensures(|_| future(out).len() == out.len()))]
    pub(crate) fn squeeze_first_block<const RATE: usize>(&self, out: &mut [u8]) {
        self.squeeze::<RATE>(out, 0, RATE);
    }

    #[inline(always)]
    #[cfg_attr(hax, hax_lib::requires(
        valid_rate(RATE) &&
        3 * RATE <= out.len()
    ))]
    #[cfg_attr(hax, hax_lib::ensures(|_| future(out).len() == out.len()))]
    pub(crate) fn squeeze_first_three_blocks<const RATE: usize>(&mut self, out: &mut [u8]) {
        self.squeeze::<RATE>(out, 0, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, RATE, RATE);

        self.keccakf1600();
        self.squeeze::<RATE>(out, 2 * RATE, RATE);
    }

    /// Final partial-block step of the squeeze phase: if `output_rem != 0`,
    /// apply one Keccak-f permutation and then extract the trailing
    /// `output_rem` bytes of output into the tail of `out`; otherwise
    /// a no-op.
    #[inline(always)]
    pub(crate) fn squeeze_last<const RATE: usize>(&mut self, out: &mut [u8], output_rem: usize) {
        if output_rem != 0 {
            self.keccakf1600();
            self.squeeze::<RATE>(out, out.len() - output_rem, output_rem);
        }
    }

    #[inline(always)]
    #[cfg_attr(hax, hax_lib::requires(
        valid_rate(RATE) &&
        5 * RATE <= out.len()
    ))]
    #[cfg_attr(hax, hax_lib::ensures(|_| future(out).len() == out.len()))]
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

/// Absorb phase of `keccak1`: initialise a Keccak state, absorb all full
/// rate-byte blocks of `input`, then pad and absorb the final partial block
/// with domain-separation byte `DELIM` and the pad10*1 terminator.
#[inline]
pub(crate) fn absorb<const RATE: usize, const DELIM: u8>(input: &[u8]) -> KeccakState<1, u64> {
    let mut s = KeccakState::<1, u64>::new();
    let input_len = input.len();
    let input_blocks = input_len / RATE;
    let input_rem = input_len % RATE;
    for i in 0..input_blocks {
        #[cfg(hax)]
        lemma_mul_succ_le(i, input_blocks, RATE);

        s.absorb_block::<RATE>(&[input], i * RATE);
    }
    s.absorb_final::<RATE, DELIM>(&[input], input_len - input_rem, input_rem);
    s
}

fn squeeze_blocks<const RATE: usize>(
    s: &mut KeccakState<1, u64>,
    output: &mut [u8],
    output_blocks: usize,
) {
    s.squeeze::<RATE>(output, 0, RATE);

    for i in 1..output_blocks {
        #[cfg(hax)]
        lemma_mul_succ_le(i, output_blocks, RATE);

        s.keccakf1600();

        s.squeeze::<RATE>(output, i * RATE, RATE);
    }
}

/// Squeeze phase of `keccak1`: extract `output.len()` bytes from `s`,
/// applying Keccak-f between each full rate-byte block of output.
#[inline]
pub(crate) fn squeeze<const RATE: usize>(mut s: KeccakState<1, u64>, output: &mut [u8]) {
    let output_len = output.len();
    let output_blocks = output_len / RATE;
    let output_rem = output_len % RATE;

    if output_blocks == 0 {
        s.squeeze::<RATE>(output, 0, output_len);
    } else {
        squeeze_blocks::<RATE>(&mut s, output, output_blocks);
        s.squeeze_last::<RATE>(output, output_rem);
    }
}

#[cfg_attr(hax, hax_lib::requires(valid_rate(RATE)))]
#[cfg_attr(hax, hax_lib::ensures(|_| future(output).len() == output.len()))]
#[cfg_attr(hax, hax_lib::fstar::options("--split_queries always --z3rlimit 300"))]
#[inline]
pub(crate) fn keccak1<const RATE: usize, const DELIM: u8>(input: &[u8], output: &mut [u8]) {
    let s = absorb::<RATE, DELIM>(input);
    squeeze::<RATE>(s, output);
}

//! A portable SHA3 implementation using the generic implementation.

#[cfg(hax)]
use hax_lib::int::*;

#[cfg(hax)]
use hax_lib::prop::*;

#[cfg(hax)]
use crate::proof_utils::valid_rate;

use crate::{generic_keccak::KeccakState, traits::*};

#[inline(always)]
#[hax_lib::requires(
    LEFT.to_int() + RIGHT.to_int() == 64.to_int() &&
    RIGHT > 0
)]
fn rotate_left<const LEFT: i32, const RIGHT: i32>(x: u64) -> u64 {
    #[cfg(not(eurydice))]
    debug_assert!(LEFT + RIGHT == 64);
    x.rotate_left(LEFT as u32)
}

#[inline(always)]
fn _veor5q_u64(a: u64, b: u64, c: u64, d: u64, e: u64) -> u64 {
    a ^ b ^ c ^ d ^ e
}

#[inline(always)]
fn _vrax1q_u64(a: u64, b: u64) -> u64 {
    a ^ rotate_left::<1, 63>(b)
}

#[inline(always)]
#[hax_lib::requires(
    LEFT.to_int() + RIGHT.to_int() == 64.to_int() &&
    RIGHT > 0
)]
fn _vxarq_u64<const LEFT: i32, const RIGHT: i32>(a: u64, b: u64) -> u64 {
    rotate_left::<LEFT, RIGHT>(a ^ b)
}

#[inline(always)]
fn _vbcaxq_u64(a: u64, b: u64, c: u64) -> u64 {
    a ^ (b & !c)
}

#[inline(always)]
fn _veorq_n_u64(a: u64, c: u64) -> u64 {
    a ^ c
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(
    valid_rate(RATE) &&
    start.to_int() + RATE.to_int() <= blocks.len().to_int()
)]
#[hax_lib::ensures(|_| hax_lib::forall(|i: usize|
      if i < 25 {
          if i < RATE/8 {
              future(state)[i] == state[i] ^ u64::from_le_bytes(blocks[start + 8 * i..start + 8 * i + 8].try_into().unwrap())
          } else {
              future(state)[i] == state[i]
          }
      } else { true }
  ))]
pub(crate) fn load_block<const RATE: usize>(state: &mut [u64; 25], blocks: &[u8], start: usize) {
    #[cfg(not(eurydice))]
    {
        debug_assert!(start + RATE <= blocks.len());
        debug_assert!(RATE % 8 == 0);
    }

    // First load the block, then xor it with the state
    // Note: combining the two loops below reduces performance for large inputs,
    //       so we knowingly use two loops: one for loading, one for xor
    let mut state_flat = [0u64; 25];

    #[allow(clippy::needless_range_loop)]
    for i in 0..RATE / 8 {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| if j < RATE / 8 {
            if j < i {
                state_flat[j]
                    == u64::from_le_bytes(
                        blocks[start + 8 * j..start + 8 * j + 8].try_into().unwrap(),
                    )
            } else {
                state_flat[j] == 0
            }
        } else {
            true
        }));
        let offset = start + 8 * i;
        state_flat[i] = u64::from_le_bytes(blocks[offset..offset + 8].try_into().unwrap());
    }

    #[cfg(hax)]
    let _old_state = *state; // ghost variable

    #[allow(clippy::needless_range_loop)]
    for i in 0..RATE / 8 {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| if j < 25 {
            if j < i {
                state[j] == _old_state[j] ^ state_flat[j]
            } else {
                state[j] == _old_state[j]
            }
        } else {
            true
        }));
        set_ij(
            state,
            i / 5,
            i % 5,
            *get_ij(state, i / 5, i % 5) ^ state_flat[i],
        );
    }
}

#[inline(always)]
#[hax_lib::requires(
    valid_rate(RATE) &&
    len < RATE &&
    start.to_int() + len.to_int() <= blocks.len().to_int()
)]
pub(crate) fn load_last<const RATE: usize, const DELIMITER: u8>(
    state: &mut [u64; 25],
    blocks: &[u8],
    start: usize,
    len: usize,
) {
    #[cfg(not(eurydice))]
    debug_assert!(start + len <= blocks.len());

    let mut buffer = [0u8; RATE];
    buffer[0..len].copy_from_slice(&blocks[start..start + len]);
    buffer[len] = DELIMITER;
    buffer[RATE - 1] |= 0x80;

    load_block::<RATE>(state, &buffer, 0);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(
    valid_rate(RATE) &&
    len <= RATE &&
    start.to_int() + len.to_int() <= out.len().to_int()
)]
#[hax_lib::ensures(|_| (future(out).len() == out.len()).to_prop() &
    hax_lib::forall(|i: usize| if i < out.len() {
        if i < start {
            out[i] == future(out)[i]
        } else if i < start + len {
            future(out)[i] == s[(i-start)/8].to_le_bytes()[(i-start)%8]
        } else {
            out[i] == future(out)[i]
        }
    } else {
        true
    })
)]
pub(crate) fn store_block<const RATE: usize>(
    s: &[u64; 25],
    out: &mut [u8],
    start: usize,
    len: usize,
) {
    let octets = len / 8;

    #[cfg(hax)]
    let old_out = out.to_vec().as_slice(); // ghost variable

    for i in 0..octets {
        hax_lib::loop_invariant!(|i: usize| (out.len() == old_out.len()).to_prop()
            & hax_lib::forall(|j: usize| if j < out.len() {
                if j < start {
                    out[j] == old_out[j]
                } else if j < start + i * 8 {
                    out[j] == s[(j - start) / 8].to_le_bytes()[(j - start) % 8]
                } else {
                    out[j] == old_out[j]
                }
            } else {
                true
            }));

        let bytes = get_ij(s, i / 5, i % 5).to_le_bytes();
        let out_pos = start + 8 * i;
        hax_lib::fstar!(
            r#"
            Proof_Utils.Lemmas.lemma_index_update_at_range out (${out_pos..out_pos+8}) bytes
        "#
        );
        out[out_pos..out_pos + 8].copy_from_slice(&bytes);
    }

    let remaining = len % 8;

    if remaining > 0 {
        let bytes = get_ij(s, octets / 5, octets % 5).to_le_bytes();
        let out_pos = start + len - remaining;
        hax_lib::fstar!(
            r#"
            Proof_Utils.Lemmas.lemma_index_update_at_range out (${out_pos..out_pos+remaining}) (Seq.slice bytes 0 (v remaining))
        "#
        );
        out[out_pos..out_pos + remaining].copy_from_slice(&bytes[..remaining]);
    }
}

#[hax_lib::attributes]
impl KeccakItem<1> for u64 {
    #[inline(always)]
    fn zero() -> Self {
        0
    }
    #[inline(always)]
    fn xor5(a: Self, b: Self, c: Self, d: Self, e: Self) -> Self {
        _veor5q_u64(a, b, c, d, e)
    }
    #[inline(always)]
    fn rotate_left1_and_xor(a: Self, b: Self) -> Self {
        _vrax1q_u64(a, b)
    }
    #[inline(always)]
    #[hax_lib::requires(
        LEFT.to_int() + RIGHT.to_int() == 64.to_int() &&
        RIGHT > 0
    )]
    fn xor_and_rotate<const LEFT: i32, const RIGHT: i32>(a: Self, b: Self) -> Self {
        _vxarq_u64::<LEFT, RIGHT>(a, b)
    }
    #[inline(always)]
    fn and_not_xor(a: Self, b: Self, c: Self) -> Self {
        _vbcaxq_u64(a, b, c)
    }
    #[inline(always)]
    fn xor_constant(a: Self, c: u64) -> Self {
        _veorq_n_u64(a, c)
    }
    #[inline(always)]
    fn xor(a: Self, b: Self) -> Self {
        a ^ b
    }
}

#[hax_lib::attributes]
impl Absorb<1> for KeccakState<1, u64> {
    #[hax_lib::requires(
        valid_rate(RATE) &&
        start.to_int() + RATE.to_int() <= input[0].len().to_int()
    )]
    fn load_block<const RATE: usize>(&mut self, input: &[&[u8]; 1], start: usize) {
        load_block::<RATE>(&mut self.st, input[0], start);
    }

    #[hax_lib::requires(
        valid_rate(RATE) &&
        len < RATE &&
        start.to_int() + len.to_int() <= input[0].len().to_int()
    )]
    fn load_last<const RATE: usize, const DELIMITER: u8>(
        &mut self,
        input: &[&[u8]; 1],
        start: usize,
        len: usize,
    ) {
        load_last::<RATE, DELIMITER>(&mut self.st, input[0], start, len);
    }
}

// Workaround for hax#1698: `fstar::before`/`after` on impl blocks is silently
// dropped by the extractor.  Attach the push-options to a hax-only dummy
// function — which does extract properly.  No `#pop-options` is needed since
// the Squeeze impl is the last item in the module.
#[cfg(hax)]
#[hax_lib::fstar::before(r#"#push-options "--split_queries always --z3rlimit 300""#)]
fn _squeeze_impl_opts() {}

#[hax_lib::attributes]
impl Squeeze<u64> for KeccakState<1, u64> {
    #[hax_lib::requires(
        valid_rate(RATE) &&
        len <= RATE &&
        start.to_int() + len.to_int() <= out.len().to_int()
    )]
    // Postcondition well-formedness notes:
    //   - `start + len` is compared at `.to_int()` level to avoid a usize-overflow
    //     subtyping obligation (the trait-field elaboration has no access to
    //     `f_squeeze_pre`, so the overflow bound can't be discharged).
    //   - The indexed `self.st[(i - start)/8]` access is wrapped in a bounds guard
    //     `(i - start)/8 < 25` for the same reason.  When the preconditions hold
    //     (`valid_rate(RATE)` && `len <= RATE`) we have `len <= 168 < 200 = 8*25`,
    //     so the guard is always true at actual use sites — the wrapping only
    //     affects the type-elaboration obligation, not the proof.
    #[hax_lib::ensures(|_| (future(out).len() == out.len()).to_prop() &
        hax_lib::forall(|i: usize| if i < out.len() {
            if i < start {
                out[i] == future(out)[i]
            } else if i.to_int() < start.to_int() + len.to_int() {
                if (i - start)/8 < 25 {
                    future(out)[i] == self.st[(i - start)/8].to_le_bytes()[(i - start)%8]
                } else {
                    true
                }
            } else {
                out[i] == future(out)[i]
            }
        } else {
            true
        })
    )]
    fn squeeze<const RATE: usize>(&self, out: &mut [u8], start: usize, len: usize) {
        store_block::<RATE>(&self.st, out, start, len);
    }
}

#[cfg(hax)]
use hax_lib::int::*;

use crate::{
    generic_keccak::KeccakState,
    traits::{Absorb, KeccakItem, Squeeze},
};

#[cfg(hax)]
use crate::proof_utils::{keccak_xof_state_inv, valid_rate};

// TODO: Should not be needed. Use hax_lib::fstar::options("") below.
// Known bug: https://github.com/cryspen/hax/issues/1698
#[hax_lib::fstar::before(
    r#"
#push-options "--split_queries always --z3rlimit 300"
"#
)]

/// The internal keccak state that can also buffer inputs to absorb.
/// This is used in the general xof APIs.
pub(crate) struct KeccakXofState<
    const PARALLEL_LANES: usize,
    const RATE: usize,
    STATE: KeccakItem<PARALLEL_LANES>,
> {
    inner: KeccakState<PARALLEL_LANES, STATE>,

    // Buffer inputs on absorb.
    buf: [[u8; RATE]; PARALLEL_LANES],

    // Buffered length.
    // Note: pub(crate) so that portable.rs can access it for verification invariants
    pub(crate) buf_len: usize,

    // Needs sponge.
    sponge: bool,

    // Buffer holding the most recently squeezed full block of RATE bytes,
    // used so that callers may request output in arbitrary-sized chunks
    // (not just RATE-aligned ones) without losing bytes between calls.
    squeeze_buf: [u8; RATE],

    // Number of bytes already returned from `squeeze_buf`.
    // Invariant: `squeeze_pos <= RATE`.
    // - `squeeze_pos == RATE` means there are no buffered output bytes
    //   left to drain (initial state, or all leftover bytes have been
    //   consumed).
    // - `squeeze_pos < RATE` means `squeeze_buf[squeeze_pos..RATE]`
    //   still needs to be returned to the caller on the next squeeze.
    squeeze_pos: usize,
}

/// Note: This function exists to work around a hax bug where `core::array::from_fn`
/// is extracted with an incorrect explicit type parameter `#(usize -> t_Slice u8)`
/// instead of using the typeclass-based implicit parameter `#v_F` from
/// `Core_models.Array.from_fn`.
/// See: https://github.com/cryspen/hax/issues/1920
#[inline(always)]
#[hax_lib::fstar::replace(
    "let buf_to_slices
      (v_PARALLEL_LANES v_RATE: usize)
      (buf: t_Array (t_Array u8 v_RATE) v_PARALLEL_LANES)
    : t_Array (t_Slice u8) v_PARALLEL_LANES =
  Core_models.Array.from_fn #(t_Slice u8)
    v_PARALLEL_LANES
    (fun i -> Core_models.Array.impl_23__as_slice #u8 v_RATE (buf.[ i ]))
"
)]
fn buf_to_slices<const PARALLEL_LANES: usize, const RATE: usize>(
    buf: &[[u8; RATE]; PARALLEL_LANES],
) -> [&[u8]; PARALLEL_LANES] {
    core::array::from_fn(|i| buf[i].as_slice())
}

#[hax_lib::attributes]
#[hax_lib::fstar::options("--split_queries always --z3rlimit 300")] // TODO: Has no effect. See above.
impl<const PARALLEL_LANES: usize, const RATE: usize, STATE: KeccakItem<PARALLEL_LANES>>
    KeccakXofState<PARALLEL_LANES, RATE, STATE>
{
    /// An all zero block
    pub(crate) const fn zero_block() -> [u8; RATE] {
        [0u8; RATE]
    }

    /// Generate a new keccak xof state.
    #[hax_lib::requires(
        PARALLEL_LANES == 1 &&
        valid_rate(RATE)
    )]
    #[hax_lib::ensures(|result|
        keccak_xof_state_inv(RATE, result.buf_len) &&
        result.squeeze_pos == RATE
    )]
    pub(crate) fn new() -> Self {
        Self {
            inner: KeccakState::new(),
            buf: [Self::zero_block(); PARALLEL_LANES],
            buf_len: 0,
            sponge: false,
            squeeze_buf: Self::zero_block(),
            squeeze_pos: RATE,
        }
    }

    /// Try to complete the internal partial buffer by consuming the minimum required
    /// number of bytes from the provided `inputs` so that `self.buf` becomes exactly
    /// one full block of size `RATE`.
    ///
    /// Behaviour:
    /// - If `self.buf_len` is 0 (no buffered bytes) or already equal to `RATE`
    ///   (already a full block), or if the combined available bytes in `inputs` are
    ///   not enough to reach `RATE`, the function does nothing and returns 0.
    /// - If `0 < self.buf_len < RATE` and `inputs[..]` contain at least
    ///   `RATE - self.buf_len` bytes, the function copies exactly
    ///   `consumed = RATE - self.buf_len` bytes from each lane `inputs[i]` into
    ///   `self.buf[i]` starting at the current `self.buf_len` offset, sets
    ///   `self.buf_len = RATE`, and returns `consumed`.
    ///
    /// Returns the `consumed` bytes from `inputs` if there's enough buffered
    /// content to consume, and `0` otherwise.
    /// If `consumed > 0` is returned, `self.buf` contains a full block to be
    /// loaded.
    // Note: consciously not inlining this function to avoid using too much stack
    #[hax_lib::requires(
        PARALLEL_LANES == 1 &&
        keccak_xof_state_inv(RATE, self.buf_len)
    )]
    #[hax_lib::ensures(|consumed|
        keccak_xof_state_inv(RATE, future(self).buf_len) &&
        if consumed == 0 {
            future(self).buf_len == self.buf_len && (
                self.buf_len == 0 ||
                self.buf_len == RATE ||
                self.buf_len.to_int() + inputs[0].len().to_int() < RATE.to_int()
            )
        } else {
            future(self).buf_len == RATE &&
            consumed.to_int() == RATE.to_int() - self.buf_len.to_int()
        }
    )]
    pub(crate) fn fill_buffer(&mut self, inputs: &[&[u8]; PARALLEL_LANES]) -> usize {
        let input_len = inputs[0].len();

        // Check if we have enough data when combining the internal buffer and the input.
        // If the internal buffer is empty and we have enough to absorb do not use.
        if self.buf_len != 0 && input_len >= RATE - self.buf_len {
            let consumed = RATE - self.buf_len;

            #[cfg(hax)]
            let self_buf_len = self.buf_len; // ghost variable for F* proof

            #[allow(clippy::needless_range_loop)]
            for i in 0..PARALLEL_LANES {
                hax_lib::loop_invariant!(|_: usize| { self.buf_len == self_buf_len });

                self.buf[i][self.buf_len..].copy_from_slice(&inputs[i][..consumed]);
            }

            self.buf_len = RATE;
            consumed
        } else {
            0
        }
    }

    // Note: consciously not inlining this function to avoid using too much stack
    #[hax_lib::requires(
        PARALLEL_LANES == 1 &&
        keccak_xof_state_inv(RATE, self.buf_len)
    )]
    #[hax_lib::ensures(|remainder|
        keccak_xof_state_inv(RATE, future(self).buf_len) &&
        future(self).buf_len.to_int() + remainder.to_int() <= RATE.to_int()
    )]
    pub(crate) fn absorb_full(&mut self, inputs: &[&[u8]; PARALLEL_LANES]) -> usize
    where
        KeccakState<PARALLEL_LANES, STATE>: Absorb<PARALLEL_LANES>,
    {
        debug_assert!(PARALLEL_LANES > 0);
        debug_assert!(self.buf_len <= RATE);
        #[cfg(all(debug_assertions, not(eurydice), not(hax)))]
        {
            for block in inputs {
                debug_assert!(block.len() == inputs[0].len());
            }
        }

        // Check if there are buffered bytes to absorb first and consume them.
        let consumed = self.fill_buffer(inputs);

        if self.buf_len == RATE {
            // We have a full block in the local buffer now.
            // Convert self.buf to the right type for load_block
            let borrowed = buf_to_slices(&self.buf);

            self.inner.load_block::<RATE>(&borrowed, 0);
            self.inner.keccakf1600();

            // "empty" the local buffer
            self.buf_len = 0;
        }

        // We only need to consume the rest of the input.
        let input_to_consume = inputs[0].len() - consumed;

        // Consume the (rest of the) input ...
        let num_blocks = input_to_consume / RATE;
        let remainder = input_to_consume % RATE;

        #[cfg(hax)]
        let (self_buf_len, end) = {
            let end = consumed + num_blocks * RATE;
            #[cfg(hax)]
            hax_lib::assert!(end <= inputs[0].len());
            (self.buf_len, end)
        };

        for i in 0..num_blocks {
            hax_lib::loop_invariant!(|_: usize| self.buf_len == self_buf_len);
            #[cfg(hax)]
            crate::proof_utils::lemma_mul_succ_le(i, num_blocks, RATE);

            let start = i * RATE + consumed;

            #[cfg(hax)]
            hax_lib::assert!(start + RATE <= end);

            self.inner.load_block::<RATE>(inputs, start);
            self.inner.keccakf1600();
        }

        remainder
    }

    /// Absorb
    ///
    /// This function takes any number of bytes to absorb and buffers if it's not enough.
    /// The function assumes that all input slices in `inputs` have the same length.
    ///
    /// Only a multiple of `RATE` blocks are absorbed.
    /// For the remaining bytes [`absorb_final`] needs to be called.
    ///
    /// This works best with relatively small `inputs`.
    #[inline(always)]
    #[hax_lib::requires(
        PARALLEL_LANES == 1 &&
        keccak_xof_state_inv(RATE, self.buf_len)
    )]
    #[hax_lib::ensures(|_| keccak_xof_state_inv(RATE, future(self).buf_len))]
    pub(crate) fn absorb(&mut self, inputs: &[&[u8]; PARALLEL_LANES])
    where
        KeccakState<PARALLEL_LANES, STATE>: Absorb<PARALLEL_LANES>,
    {
        let remainder = self.absorb_full(inputs);

        // ... buffer the rest if there's not enough input (left).
        if remainder > 0 {
            #[cfg(not(eurydice))]
            debug_assert!(
                self.buf_len == 0 || // We consumed everything (or it was empty all along).
                self.buf_len + remainder <= RATE
            );

            #[cfg(hax)]
            hax_lib::assert!(remainder.to_int() + self.buf_len.to_int() <= RATE.to_int());

            let input_len = inputs[0].len();

            #[cfg(hax)]
            let self_buf_len = self.buf_len;

            #[allow(clippy::needless_range_loop)]
            for i in 0..PARALLEL_LANES {
                hax_lib::loop_invariant!(|_: usize| self.buf_len == self_buf_len);

                self.buf[i][self.buf_len..self.buf_len + remainder]
                    .copy_from_slice(&inputs[i][input_len - remainder..input_len]);
            }

            self.buf_len += remainder;
        }
    }

    /// Absorb a final block.
    ///
    /// The `inputs` block may be empty. Everything in the `inputs` block beyond
    /// `RATE` bytes is ignored.
    #[inline(always)]
    #[hax_lib::requires(
        PARALLEL_LANES == 1 &&
        keccak_xof_state_inv(RATE, self.buf_len)
    )]
    #[hax_lib::ensures(|_| keccak_xof_state_inv(RATE, future(self).buf_len))]
    pub(crate) fn absorb_final<const DELIMITER: u8>(&mut self, inputs: &[&[u8]; PARALLEL_LANES])
    where
        KeccakState<PARALLEL_LANES, STATE>: Absorb<PARALLEL_LANES>,
    {
        self.absorb(inputs);

        let borrowed = buf_to_slices(&self.buf);

        self.inner
            .load_last::<RATE, DELIMITER>(&borrowed, 0, self.buf_len);
        self.inner.keccakf1600();
    }
}

/// Squeeze we only implement for N = 1 right now.
/// This is because it's not needed for N > 1 right now, but also because hax
/// can't handle the required mutability for it.
#[hax_lib::attributes]
impl<const RATE: usize, STATE: KeccakItem<1>> KeccakXofState<1, RATE, STATE> {
    /// Squeeze output bytes into `out`.
    ///
    /// Supports arbitrary-sized requests across multiple calls. Bytes
    /// from the most recently squeezed RATE-byte block that exceed the
    /// caller's request are buffered internally so that the next call
    /// can resume from that block before permuting again. This avoids
    /// the previous restriction that all chunks except the last had to
    /// be a multiple of `RATE` bytes.
    /// See https://github.com/cryspen/libcrux/issues/1362.
    #[inline(always)]
    #[hax_lib::requires(
        keccak_xof_state_inv(RATE, self.buf_len) &&
        self.squeeze_pos <= RATE
    )]
    #[hax_lib::ensures(|_|
        keccak_xof_state_inv(RATE, future(self).buf_len) &&
        future(self).squeeze_pos <= RATE &&
        future(out).len() == out.len()
    )]
    pub(crate) fn squeeze(&mut self, out: &mut [u8])
    where
        KeccakState<1, STATE>: Squeeze<STATE>,
    {
        let out_len = out.len();

        if out_len == 0 {
            return;
        }

        // 1) Drain any leftover bytes from the previously squeezed block.
        //    `squeeze_pos < RATE` only after a previous partial squeeze
        //    that did not consume the full RATE-byte block.
        let mut out_offset = 0;
        if self.squeeze_pos < RATE {
            let avail = RATE - self.squeeze_pos;
            let take = if avail < out_len { avail } else { out_len };
            out[..take]
                .copy_from_slice(&self.squeeze_buf[self.squeeze_pos..self.squeeze_pos + take]);
            self.squeeze_pos += take;
            out_offset = take;
        }

        if out_offset == out_len {
            return;
        }

        // 2) Need more output: permute (unless this is the very first
        //    squeeze, in which case the absorb already permuted).
        if self.sponge {
            self.inner.keccakf1600();
        }
        self.sponge = true;

        let remaining = out_len - out_offset;
        let blocks = remaining / RATE;
        let last_full = out_offset + blocks * RATE;

        if blocks == 0 {
            // Sub-RATE request: extract a full block into our buffer
            // and copy out the requested prefix; the rest is preserved
            // for the next call.
            self.inner.squeeze::<RATE>(&mut self.squeeze_buf, 0, RATE);
            out[out_offset..out_len].copy_from_slice(&self.squeeze_buf[..remaining]);
            self.squeeze_pos = remaining;
        } else {
            // Extract the first remaining block from the current state.
            self.inner.squeeze::<RATE>(out, out_offset, RATE);

            #[cfg(hax)]
            let self_buf_len = self.buf_len;

            // Apply f then extract for each subsequent full block.
            for i in 1..blocks {
                hax_lib::loop_invariant!(
                    |_: usize| out.len() == out_len && self_buf_len == self.buf_len
                );
                #[cfg(hax)]
                hax_lib::assert!(
                    out_offset.to_int() + i.to_int() * RATE.to_int() <= out.len().to_int()
                );

                self.inner.keccakf1600();
                self.inner.squeeze::<RATE>(out, out_offset + i * RATE, RATE);
            }

            // Trailing partial block, if any: extract a full block
            // into our buffer and copy out the partial prefix.
            let trailing = out_len - last_full;
            if trailing > 0 {
                self.inner.keccakf1600();
                self.inner.squeeze::<RATE>(&mut self.squeeze_buf, 0, RATE);
                out[last_full..out_len].copy_from_slice(&self.squeeze_buf[..trailing]);
                self.squeeze_pos = trailing;
            } else {
                self.squeeze_pos = RATE;
            }
        }
    }
}

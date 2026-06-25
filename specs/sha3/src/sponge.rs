/// Sponge construction — FIPS 202, Algorithm 8 (KECCAK[c])
/// with pad10*1 padding — FIPS 202, Algorithm 9.
///
/// With the state stored as `state[5·y + x]` (FIPS 202 §3.1.2), byte-lane
/// `l` lives directly at `state[l]`, so no lane-index permutation is
/// needed here.
use crate::createi;
use crate::keccak_f::{keccak_f, State};

#[cfg(hax)]
use hax_lib::int::*;
#[cfg(hax)]
use hax_lib::prop::*;

/// XOR a block of message bytes into the state (little-endian, lane-interleaved).
///
/// Corresponds to the `S ⊕ (Pi || 0^c)` step of Algorithm 8.
#[hax_lib::requires(rate <= 200 && rate % 8 == 0 && block.len() >= rate)]
pub fn xor_block_into_state(state: State, block: &[u8], rate: usize) -> State {
    createi(|i| {
        if i < rate / 8 {
            state[i] ^ u64::from_le_bytes(block[8 * i..8 * i + 8].try_into().unwrap())
        } else {
            state[i]
        }
    })
}

/// Extract `len` bytes from the rate portion of the state (little-endian, lane-interleaved).
///
/// Corresponds to `Trunc_r(S)` in Algorithm 8.
#[hax_lib::requires(len <= 200 && output.len() >= len && out_offset <= output.len() - len)]
#[hax_lib::ensures(|result| (result.len() == output.len()).to_prop() &
	hax_lib::forall(|i:usize|
		if i < output.len() {
		   if i < out_offset {
 			result[i] == output[i]
		   } else if i < out_offset + len {
			result[i] == state[(i - out_offset) / 8].to_le_bytes()[(i - out_offset) % 8]
		   } else {
			result[i] == output[i]
		   }
		} else {true}))]
pub fn squeeze_state<const OUTPUT_LEN: usize>(
    state: &State,
    mut output: [u8; OUTPUT_LEN],
    out_offset: usize,
    len: usize,
) -> [u8; OUTPUT_LEN] {
    let bytes: [u8; 200] = createi(|i| state[i / 8].to_le_bytes()[i % 8]);
    hax_lib::fstar!(
        r#"
	 Proof_Utils.Lemmas.lemma_index_update_at_range output
	 	(${out_offset..out_offset+len}) (Seq.slice bytes 0 (v len))
    "#
    );
    output[out_offset..out_offset + len].copy_from_slice(&bytes[0..len]);
    output
}

/// Absorb one full block: XOR it into the state, then apply Keccak-f.
///
/// Corresponds to one iteration of the absorb loop in Algorithm 8 (step 6).
#[hax_lib::requires(rate <= 200 && rate % 8 == 0 && block.len() == rate)]
pub fn absorb_block(state: State, block: &[u8], rate: usize) -> State {
    let state = xor_block_into_state(state, block, rate);
    keccak_f(state)
}

/// Build the padded last block: copy remaining message bytes, add the
/// domain-separation byte `delim`, and set the final bit of pad10*1.
///
/// Returns a `rate`-byte buffer ready to be absorbed via `xor_block_into_state`.
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(rate > 0 && rate <= 200 && rate % 8 == 0 && remaining < rate
                     && msg_offset <= message.len() && remaining <= message.len() - msg_offset)]
pub fn pad_last_block(
    message: &[u8],
    msg_offset: usize,
    remaining: usize,
    rate: usize,
    delim: u8,
) -> [u8; 200] {
    let mut buffer = [0u8; 200];
    buffer[0..remaining].copy_from_slice(&message[msg_offset..msg_offset + remaining]);
    buffer[remaining] = delim;
    buffer[rate - 1] = buffer[rate - 1] | 0x80;
    buffer
}

/// Absorb the final (possibly partial) block: pad it, XOR into state, and
/// apply Keccak-f.
///
/// Combines `pad_last_block` + `absorb_block`.
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(rate > 0 && rate <= 200 && rate % 8 == 0 && remaining < rate
                     && msg_offset <= message.len() && remaining <= message.len() - msg_offset)]
pub fn absorb_final(
    state: State,
    message: &[u8],
    msg_offset: usize,
    remaining: usize,
    rate: usize,
    delim: u8,
) -> State {
    let block = pad_last_block(message, msg_offset, remaining, rate, delim);
    absorb_block(state, &block[0..rate], rate)
}

/// Recursively absorb the remaining bytes of `message`: peel off one full
/// `rate`-byte block, XOR it into the state, apply Keccak-f, then recurse on
/// the tail slice. Once fewer than `rate` bytes remain, pad and absorb the
/// partial final block.
///
/// This recursive form is chosen so the extracted F* definition lines up
/// block-for-block with the libcrux impl's absorb loop.
#[hax_lib::requires(rate > 0 && rate <= 200 && rate % 8 == 0)]
#[hax_lib::decreases(message.len().to_int())]
pub fn absorb_rec(state: State, rate: usize, delim: u8, message: &[u8]) -> State {
    if message.len() < rate {
        absorb_final(state, message, 0, message.len(), rate, delim)
    } else {
        let state = absorb_block(state, &message[0..rate], rate);
        absorb_rec(state, rate, delim, &message[rate..])
    }
}

/// Iterative middle-loop of [absorb]: for each block index `k ∈ [i, input_blocks)`,
/// XOR `input[k*rate..(k+1)*rate]` into the state and apply Keccak-f via
/// [absorb_block]. Returns the state after the final [absorb_block].
///
/// Shape chosen to mirror [squeeze_blocks], so the F* equivalence proof can line
/// up the libcrux impl's `fold_range 0 input_blocks` step-by-step against this
/// recursion via tail-extension lemmas, dodging the slice-of-slice reasoning
/// that triggers a Z3 4.13.3 LP-solver internal-assertion bug in the older
/// proof that used `absorb_rec` with progressive `message[rate..]` tails.
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(rate > 0 && rate <= 200 && rate % 8 == 0
                     && i <= input_blocks
                     && input_blocks <= input.len() / rate)]
#[hax_lib::decreases(input_blocks.to_int() - i.to_int())]
pub fn absorb_blocks(
    state: State,
    rate: usize,
    i: usize,
    input_blocks: usize,
    input: &[u8],
) -> State {
    if i < input_blocks {
        let state = absorb_block(state, &input[i * rate..i * rate + rate], rate);
        absorb_blocks(state, rate, i + 1, input_blocks, input)
    } else {
        state
    }
}

/// Absorb phase of the Keccak sponge (FIPS 202, Algorithm 8, step 6 combined
/// with the pad10*1 padding of Algorithm 9).
///
/// Splits `message` into `rate`-byte blocks, XORing each into the state and
/// applying Keccak-f. The final partial block is padded with the domain
/// separation byte `delim` and the pad10*1 terminator `0x80` before being
/// absorbed.
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(rate > 0 && rate <= 200 && rate % 8 == 0)]
pub fn absorb(rate: usize, delim: u8, message: &[u8]) -> State {
    absorb_rec([0u64; 25], rate, delim, message)
}

/// Apply Keccak-f to `state` exactly `n` times.  Right-add recursion
/// so `iterate_keccak_f(n+1, state) == keccak_f(iterate_keccak_f(n, state))`
/// is by definitional unfold at fuel 1 — consumers don't need a
/// separate lemma to peel one round off the right.
#[hax_lib::decreases(n.to_int())]
pub fn iterate_keccak_f(n: usize, state: State) -> State {
    if n == 0 {
        state
    } else {
        keccak_f(iterate_keccak_f(n - 1, state))
    }
}

/// Recursive middle-loop of [squeeze]: for each block index `i ∈ [i, output_blocks)`,
/// apply `keccak_f` and then extract `rate` bytes at `i * rate`. Returns the state
/// after the final `keccak_f`.
///
/// Shape chosen to mirror `absorb_rec`, so the F* equivalence proof can line up
/// the libcrux impl's `fold_range 1 output_blocks` step-by-step against this
/// recursion via `lemma_fold_range_step`, the same pattern used for absorb.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(rate > 0 && rate <= 200 && rate % 8 == 0
                     && i <= output_blocks
                     && output_blocks <= output.len() / rate)]
#[hax_lib::decreases(output_blocks.to_int() - i.to_int())]
pub fn squeeze_blocks<const OUTPUT_LEN: usize>(
    state: State,
    rate: usize,
    i: usize,
    output_blocks: usize,
    mut output: [u8; OUTPUT_LEN],
) -> (State, [u8; OUTPUT_LEN]) {
    if i < output_blocks {
        let state = keccak_f(state);
        output = squeeze_state(&state, output, i * rate, rate);
        squeeze_blocks(state, rate, i + 1, output_blocks, output)
    } else {
        (state, output)
    }
}

/// Final partial-block step of [squeeze]: if `output_rem != 0`, apply
/// one Keccak-f permutation and extract the trailing `output_rem`
/// bytes; otherwise a no-op.  Factored out so the libcrux impl's
/// corresponding helper can have a matching shape, keeping the
/// equivalence proof's final reconciliation local.
#[hax_lib::requires(rate > 0 && rate <= 200 && rate % 8 == 0
                     && output_rem < rate
                     && output_rem <= OUTPUT_LEN
                     && OUTPUT_LEN < usize::MAX - 200)]
pub fn squeeze_last<const OUTPUT_LEN: usize>(
    state: State,
    output: [u8; OUTPUT_LEN],
    rate: usize,
    output_rem: usize,
) -> (State, [u8; OUTPUT_LEN]) {
    let _ = rate;
    if output_rem != 0 {
        let state = keccak_f(state);
        let output = squeeze_state(&state, output, OUTPUT_LEN - output_rem, output_rem);
        (state, output)
    } else {
        (state, output)
    }
}

/// Squeeze phase of the Keccak sponge (FIPS 202, Algorithm 8, steps 8–9).
///
/// Extracts `OUTPUT_LEN` bytes from `state`, applying Keccak-f between each
/// `rate`-byte block of output.
///
/// Byteform definition: byte at position `k` lives in block `b = k / rate`
/// (or the trailing partial block if `b == OUTPUT_LEN / rate`); within a
/// block the offset is `j = k - b * rate`; the value is the `(j mod 8)`-th
/// little-endian byte of `iterate_keccak_f(b, state)`'s lane `(j / 8)`.
///
/// Equivalent to FIPS-202 Algorithm 8: for each full block apply keccak_f
/// and extract `rate` bytes; the trailing partial block uses one more
/// keccak_f before extracting `OUTPUT_LEN mod rate` bytes.
#[hax_lib::fstar::options("--z3rlimit 500")]
#[hax_lib::requires(rate > 0 && rate <= 200 && rate % 8 == 0 && OUTPUT_LEN < usize::MAX - 200)]
pub fn squeeze<const OUTPUT_LEN: usize>(state: State, rate: usize) -> [u8; OUTPUT_LEN] {
    createi(|k| {
        let b = k / rate;
        let j = k - b * rate;
        let state_b = iterate_keccak_f(b, state);
        state_b[j / 8].to_le_bytes()[j % 8]
    })
}

/// Keccak sponge — FIPS 202, Algorithm 8 combined with pad10*1 (Algorithm 9).
///
/// 1. Absorb: split `message` into `rate`-byte blocks, XOR each into the
///    state, and apply Keccak-f. The final partial block is padded with
///    the domain separation byte `delim` and the pad10*1 terminator `0x80`.
/// 2. Squeeze: extract `OUTPUT_LEN` bytes from the state, applying
///    Keccak-f between each `rate`-byte block of output.
///
/// The `OUTPUT_LEN < usize::MAX - 200` precondition is a Rust implementation
/// artifact to prevent arithmetic overflow; FIPS 202 places no upper bound
/// on the output length.
#[hax_lib::requires(rate > 0 && rate <= 200 && rate % 8 == 0 && OUTPUT_LEN < usize::MAX - 200)]
pub fn keccak<const OUTPUT_LEN: usize>(rate: usize, delim: u8, message: &[u8]) -> [u8; OUTPUT_LEN] {
    squeeze(absorb(rate, delim, message), rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every SHA-3 variant must satisfy `keccak == squeeze ∘ absorb`.
    /// This pins down the refactor that split `keccak` into its two
    /// phases so the F* equivalence proof can be structured per-phase.
    #[test]
    fn keccak_equals_squeeze_of_absorb() {
        fn check<const OUT: usize>(rate: usize, delim: u8, msg: &[u8]) {
            let via_keccak = keccak::<OUT>(rate, delim, msg);
            let via_split = squeeze(absorb(rate, delim, msg), rate);
            assert_eq!(
                via_keccak,
                via_split,
                "keccak != squeeze(absorb) for rate={rate}, delim={delim:#x}, msg.len()={}",
                msg.len()
            );
        }

        let empty: [u8; 0] = [];
        let short = b"hello world";
        let long: Vec<u8> = (0u8..200).collect();

        // SHA3-224: rate=144, delim=0x06, out=28
        check::<28>(144, 0x06, &empty);
        check::<28>(144, 0x06, short);
        check::<28>(144, 0x06, &long);
        // SHA3-256: rate=136, delim=0x06, out=32
        check::<32>(136, 0x06, &empty);
        check::<32>(136, 0x06, short);
        check::<32>(136, 0x06, &long);
        // SHA3-384: rate=104, delim=0x06, out=48
        check::<48>(104, 0x06, short);
        // SHA3-512: rate=72, delim=0x06, out=64
        check::<64>(72, 0x06, short);
        // SHAKE128: rate=168, delim=0x1f — short and long output exercise the squeeze loop.
        check::<16>(168, 0x1f, short);
        check::<200>(168, 0x1f, short);
        // SHAKE256: rate=136, delim=0x1f.
        check::<64>(136, 0x1f, short);
        check::<300>(136, 0x1f, short);
    }
}

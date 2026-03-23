//! Implementation of AES-CCM
use core::ops::Range;

use crate::{
    aes::{block_cipher, AES_BLOCK_LEN},
    ctr::{AesCtrContext, CcmInit, AES_CCM_CTR_LEN, AES_CCM_NONCE_START},
    platform::AESState,
    DecryptError, CCM_SHORT_TAG_LEN, NONCE_LEN, TAG_LEN,
};

const TWO_BYTE_ENCODING_RANGE: Range<usize> = 0..(1 << 16) - (1 << 8);
#[cfg(target_pointer_width = "64")]
const SIX_BYTE_ENCODING_RANGE: Range<usize> = (1 << 16) - (1 << 8)..(1 << 32);
#[cfg(target_pointer_width = "32")]
const SIX_BYTE_ENCODING_RANGE: Range<usize> = (1 << 16) - (1 << 8)..usize::MAX;
#[cfg(target_pointer_width = "64")]
const TEN_BYTE_ENCODING_RANGE: Range<usize> = (1 << 32)..usize::MAX;

impl<const TAG_LEN: usize, const NUM_KEYS: usize, T: AESState> super::State
    for State<TAG_LEN, NUM_KEYS, T>
where
    AesCtrContext<T, NUM_KEYS, AES_CCM_CTR_LEN, AES_CCM_NONCE_START>: CcmInit,
{
    /// Initialize the state, internally expanding subkeys for
    /// AES block cipher.
    fn init(key: &[u8]) -> Self {
        let accumulator = [0u8; AES_BLOCK_LEN];
        let aes_state = AesCtrContext::ccm_init(key);
        Self {
            aes_state,
            accumulator,
        }
    }

    /// Set the nonce for the AES-CTR and authentication
    /// states.
    fn set_nonce(&mut self, nonce: &[u8]) {
        debug_assert!(nonce.len() == NONCE_LEN);

        self.aes_state.aes_ctr_set_nonce(nonce);
        self.accumulator[1..1 + NONCE_LEN].copy_from_slice(nonce);
    }

    /// Encrypt and authenticate AAD and plaintext.
    fn encrypt(&mut self, aad: &[u8], plaintext: &[u8], ciphertext: &mut [u8], tag: &mut [u8]) {
        debug_assert_eq!(tag.len(), TAG_LEN);
        let mut tag_block = [0u8; AES_BLOCK_LEN];

        // fill accumulator with CBC-MAC of AAD and plaintext
        self.ccm_update_aad(aad, plaintext.len());
        self.ccm_update_plaintext(plaintext);

        // xor first key block to CBC-MAC
        self.aes_state
            .aes_ctr_update(0, &self.accumulator, &mut tag_block);

        // encrypt plaintext
        self.aes_state.aes_ctr_update(1, plaintext, ciphertext);

        // write out tag
        tag.copy_from_slice(&tag_block[..TAG_LEN]);
    }

    /// Verify authentication tag, and if valid decrypt
    /// plaintext from ciphertext.
    fn decrypt(
        &mut self,
        aad: &[u8],
        ciphertext: &[u8],
        tag: &[u8],
        plaintext: &mut [u8],
    ) -> Result<(), DecryptError> {
        debug_assert_eq!(tag.len(), TAG_LEN);
        let mut tag_block = [0u8; AES_BLOCK_LEN];

        // Feed accumulator with AAD.
        self.ccm_update_aad(aad, ciphertext.len());
        // Feed accumulator with ciphertext blocks.
        //
        // This decrypts ciphertext blocks on the fly
        // internally to accumulate decrypted plaintext blocks
        // into the candidate CBC-MAC without prematurely
        // writing out an unauthenticated decryption to the
        // output buffer.
        self.ccm_update_ciphertext(ciphertext);

        // xor first key block to CBC-MAC
        self.aes_state
            .aes_ctr_update(0, &self.accumulator, &mut tag_block);

        // Check that recomputed tag in accumulator agrees
        // with provided tag.
        let mut eq_mask = 0u8;
        for i in 0..TAG_LEN {
            eq_mask |= tag_block[i] ^ tag[i];
        }

        if eq_mask != 0 {
            return Err(DecryptError::InvalidTag);
        }

        // Decrypt and write out plaintext if tag was valid.
        self.aes_state.aes_ctr_update(1, ciphertext, plaintext);
        Ok(())
    }
}

// Length in bytes of the field encoding the message length in bytes.
const MSG_ENC_LEN: usize = 3;

/// The AES-CCM state.
pub(crate) struct State<const TAG_LEN: usize, const NUM_KEYS: usize, T: AESState> {
    /// Internal AES-CTR state for encryption/decryption.
    pub(crate) aes_state: AesCtrContext<T, NUM_KEYS, AES_CCM_CTR_LEN, 1>,
    /// Internal state for accumulating the authentication tag from AAD and
    /// message.
    pub(crate) accumulator: [u8; AES_BLOCK_LEN],
}

impl<const TAG_LEN: usize, const NUM_KEYS: usize, T: AESState> State<TAG_LEN, NUM_KEYS, T> {
    /// Update authentication state by accumulating AAD.
    ///
    /// The state needs to be initialized first to set the
    /// correct nonce in the initial state of the accumulator.
    #[inline]
    fn ccm_update_aad(&mut self, aad: &[u8], payload_len: usize) {
        // We need this to get the right slices from the end
        // of `x.len().to_be_bytes()` where `x` is a usize.
        const USIZE_LEN: usize = core::mem::size_of::<usize>();

        // `MSG_ENC_LEN` is 3, so this should always be the
        // case.
        debug_assert!(MSG_ENC_LEN <= USIZE_LEN);
        debug_assert!(MSG_ENC_LEN <= AES_BLOCK_LEN);
        debug_assert_eq!(15 - MSG_ENC_LEN, NONCE_LEN);

        // Byte 0 of initial accumulator value:
        // bit 7: `Reserved`, set to 0
        // bit 6: `Adata`, 1 if len(AAD) > 0, 0 otherwise
        // bits 5..=3: `(TAG_LEN - 2) / 2` encoded in three bytes
        // bits 2..=0: `(MSG_ENC_LEN - 1)` encoded in three bytes
        self.accumulator[0] =
            64 * (!aad.is_empty() as u8) + ((TAG_LEN as u8 - 2) / 2) * 8 + (MSG_ENC_LEN as u8) - 1;

        // Bytes 1..=15-MSG_ENC_LEN contain the nonce, which
        // is set in `set_nonce`.

        // Bytes 16-MSG_ENC_LEN..=15 contain the plaintext
        // length, encoded in `MSG_ENC_LEN` bytes.
        self.accumulator[AES_BLOCK_LEN - MSG_ENC_LEN as usize..]
            .copy_from_slice(&payload_len.to_be_bytes()[USIZE_LEN - MSG_ENC_LEN as usize..]);

        // Process the initial value
        let mut st = T::new();
        st.load_block(&self.accumulator);
        block_cipher(&mut st, &self.aes_state.extended_key);
        st.store_block(&mut self.accumulator);

        // The AAD is prepended with an encoding of its length
        // before accumulating it.

        // If len(AAD) == 0, nothing further is accumulated
        // and we move on to accumulating the plaintext.
        let aad_len = aad.len();
        if aad_len == 0 {
            return;
        }

        let mut current_block = [0u8; AES_BLOCK_LEN];

        // The AAD length encoding can be two, six, or ten
        // bytes long, depending on which range `len(AAD)`
        // falls into.
        let mut aad_len_encoding_len = 2;

        if TWO_BYTE_ENCODING_RANGE.contains(&aad_len) {
            // If 0 < len(AAD) < 2^16 - 2^8, len(AAD) is encoded
            // in two bytes.
            current_block[0..2].copy_from_slice(&aad_len.to_be_bytes()[USIZE_LEN - 2..]);
        } else if SIX_BYTE_ENCODING_RANGE.contains(&aad_len) {
            // If 2^16 - 2^8 <= len(AAD) < 2^32, len(AAD) is
            // encoded in four bytes and prefixed by the two
            // bytes 0xff, 0xfe.
            aad_len_encoding_len = 6;
            current_block[0] = 0xff;
            current_block[1] = 0xfe;
            current_block[2..6].copy_from_slice(&aad_len.to_be_bytes()[USIZE_LEN - 4..]);
        }

        // The ten byte encoding range is larger than we can
        // handle in 32-bits.
        #[cfg(target_pointer_width = "64")]
        if TEN_BYTE_ENCODING_RANGE.contains(&aad_len) {
            // If 2^32 <= len(AAD) < 2^64, len(AAD) is
            // encoded in 8 bytes and prefixed by the two
            // bytes 0xff, 0xff.
            aad_len_encoding_len = 10;
            current_block[0] = 0xff;
            current_block[1] = 0xff;
            current_block[2..10].copy_from_slice(&aad_len.to_be_bytes());
        }

        // We have checked in the traits API that the AAD
        // length does not exceed `usize::MAX - 10`, so this
        // addition should not overflow.
        if aad_len + aad_len_encoding_len <= AES_BLOCK_LEN {
            // If len(AAD) + aad_len_encoding_len does not fill a
            // full block, we write out the AAD into the current
            // block which is implicitly padded with zeroes, and
            // then accumulate.
            current_block[aad_len_encoding_len..aad_len + aad_len_encoding_len]
                .copy_from_slice(&aad);

            self.accumulate(current_block.as_slice());
        } else {
            // We have to incorporate the bytes used for the
            // encoding of len(AAD) into the computation of
            // full blocks to be accumulated.
            let full_blocks = (aad_len_encoding_len + aad_len) / AES_BLOCK_LEN;
            let remainder = (aad_len_encoding_len + aad_len) - full_blocks * AES_BLOCK_LEN;

            let initial_aad_chunk_len = AES_BLOCK_LEN - aad_len_encoding_len;

            for i in 0..full_blocks {
                if i == 0 {
                    // The first full block contains the
                    // encoding of len(AAD) at the beginning,
                    // so we can only include
                    // `initial_aad_chunk_len` bytes from the
                    // AAD here.
                    current_block[aad_len_encoding_len..]
                        .copy_from_slice(&aad[0..initial_aad_chunk_len]);
                } else {
                    let offset = initial_aad_chunk_len + (i - 1) * AES_BLOCK_LEN;
                    current_block.copy_from_slice(&aad[offset..offset + AES_BLOCK_LEN]);
                }

                self.accumulate(current_block.as_slice());
            }

            if remainder != 0 {
                current_block = [0u8; AES_BLOCK_LEN];
                current_block[..remainder].copy_from_slice(&aad[aad_len - remainder..]);

                self.accumulate(current_block.as_slice());
            }
        }
    }

    /// Update authentication state by accumulating plaintext.
    ///
    /// This needs to be called after `ccm_update_aad`.
    /// Afterwards, `self.accumulator` will contain the
    /// block-length CBC-MAC of AAD and message plaintext,
    /// which needs to be xor-ed with the first block of the
    /// CTR key stream and truncated to the final
    /// authentication tag length.
    fn ccm_update_plaintext(&mut self, payload: &[u8]) {
        let full_blocks = payload.len() / AES_BLOCK_LEN;
        let remainder = payload.len() - full_blocks * AES_BLOCK_LEN;

        for i in 0..full_blocks {
            let offset = i * AES_BLOCK_LEN;
            self.accumulate(&payload[offset..offset + AES_BLOCK_LEN]);
        }

        if remainder != 0 {
            self.accumulate(&payload[full_blocks * AES_BLOCK_LEN..]);
        }
    }

    /// Update authentication state by accumulating ciphertext
    /// blocks, decrypting on the fly.
    ///
    /// This needs to be called after `ccm_update_aad`.
    /// Afterwards, `self.accumulator` will contain the
    /// block-length CBC-MAC of AAD and message plaintext,
    /// which needs to be xor-ed with the first block of the
    /// CTR key stream and truncated to the final
    /// authentication tag length.
    fn ccm_update_ciphertext(&mut self, ciphertext: &[u8]) {
        let full_blocks = ciphertext.len() / AES_BLOCK_LEN;
        let remainder = ciphertext.len() - full_blocks * AES_BLOCK_LEN;

        let mut key_block = [0u8; AES_BLOCK_LEN];

        for i in 0..full_blocks {
            // The traits API will reject ciphertexts which
            // are longer than `u32::MAX - 2` full blocks
            // long, so this cast is safe.
            self.aes_state
                .aes_ctr_key_block((i + 1) as u32, &mut key_block);
            let offset = i * AES_BLOCK_LEN;
            for j in 0..AES_BLOCK_LEN {
                key_block[j] ^= ciphertext[offset + j]
            }

            self.accumulate(key_block.as_slice());
        }

        if remainder != 0 {
            self.aes_state
                .aes_ctr_key_block((full_blocks + 1) as u32, &mut key_block);
            let offset = full_blocks * AES_BLOCK_LEN;
            for j in 0..remainder {
                key_block[j] ^= ciphertext[offset + j]
            }

            self.accumulate(&key_block[0..remainder]);
        }
    }

    /// Accumulate an input of at most `AES_BLOCK_LEN` bytes
    /// into the authentication state.
    ///
    /// The input is implicitly zero-padded to
    /// `AES_BLOCK_LEN` bytes.
    ///
    /// self.accumulator = AES(self.accumulator ^ pad(input))
    fn accumulate(&mut self, input: &[u8]) {
        debug_assert!(input.len() <= AES_BLOCK_LEN);
        for j in 0..input.len() {
            self.accumulator[j] ^= input[j];
        }
        let mut st = T::new();
        st.load_block(&self.accumulator);
        block_cipher(&mut st, &self.aes_state.extended_key);
        st.store_block(&mut self.accumulator);
    }
}

pub(crate) type AesCcm128State<T> = State<TAG_LEN, 11, T>;
#[allow(non_camel_case_types)]
pub(crate) type AesCcm128_8_State<T> = State<CCM_SHORT_TAG_LEN, 11, T>;

pub(crate) type AesCcm256State<T> = State<TAG_LEN, 15, T>;
#[allow(non_camel_case_types)]
pub(crate) type AesCcm256_8_State<T> = State<CCM_SHORT_TAG_LEN, 15, T>;

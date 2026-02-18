//! Implementation of AES-CCM

use crate::{
    aes::{block_cipher, AES_BLOCK_LEN},
    ctr::{AesCcm128CtrContext, AesCcm256CtrContext, AesCtrContext},
    platform::AESState,
    DecryptError, CCM_SHORT_TAG_LEN, NONCE_LEN, TAG_LEN,
};
use core::ops::Range;

/// Macro to instantiate the AES state.
const TWO_BYTE_ENCODING_RANGE: Range<usize> = 0..(1 << 16) - (1 << 8);
const SIX_BYTE_ENCODING_RANGE: Range<usize> = (1 << 16) - (1 << 8)..(1 << 32);
const TEN_BYTE_ENCODING_RANGE: Range<usize> = (1 << 32)..usize::MAX;
/// This should really be replaced by using traits everywhere.
macro_rules! aesccm {
    ($state:ty, $ctr_context:ident, $key_len:literal) => {
        impl<T: AESState> super::State for $state {
            /// Initialize the state
            fn init(key: &[u8]) -> Self {
                debug_assert!(key.len() == $key_len);

                let nonce = [0u8; NONCE_LEN];
                let accumulator = [0u8; AES_BLOCK_LEN];

                let aes_state = $ctr_context::<T>::init(key, &nonce);

                Self {
                    aes_state,
                    accumulator,
                }
            }

            fn set_nonce(&mut self, nonce: &[u8]) {
                debug_assert!(nonce.len() == NONCE_LEN);

                self.aes_state.set_nonce(nonce);
                self.accumulator[1..1 + NONCE_LEN].copy_from_slice(nonce);
            }

            fn encrypt(
                &mut self,
                aad: &[u8],
                plaintext: &[u8],
                ciphertext: &mut [u8],
                tag: &mut [u8],
            ) {
                let mut tag_block = [0u8; AES_BLOCK_LEN];

                // fill accumulator with CBC-MAC of AAD and plaintext
                self.ccm_update_aad(aad, plaintext.len());
                self.ccm_update_plaintext(plaintext);

                // xor first key block to CBC-MAC
                self.aes_state.update(0, &self.accumulator, &mut tag_block);

                // encrypt plaintext
                self.aes_state.update(1, plaintext, ciphertext);

                // write out tag
                tag.copy_from_slice(&tag_block[..tag.len()]);
            }

            fn decrypt(
                &mut self,
                aad: &[u8],
                ciphertext: &[u8],
                tag: &[u8],
                plaintext: &mut [u8],
            ) -> Result<(), DecryptError> {
                let mut tag_block = [0u8; AES_BLOCK_LEN];

                // Feed accumulator with AAD
                self.ccm_update_aad(aad, ciphertext.len());
                // Feed accumulator with ciphertext blocks
                self.ccm_update_ciphertext(ciphertext);

                // xor first key block to CBC-MAC
                self.aes_state.update(0, &self.accumulator, &mut tag_block);

                // Check that recomputed tag in accumulator agrees with tag
                let mut eq_mask = 0u8;
                for i in 0..tag.len() {
                    eq_mask |= (tag_block[i] ^ tag[i]);
                }

                if eq_mask != 0 {
                    return Err(DecryptError::InvalidTag);
                }

                // Decrypt and write out plaintext if tag was valid
                self.aes_state.update(1, ciphertext, plaintext);
                Ok(())
            }
        }
    };
}

// Length in bytes of the field encoding the message length in bytes.
const MSG_ENC_LEN: usize = 3;
pub(crate) const AES_CCM_CTR_LEN: usize = 3;

/// The AES-CCM 128 state
pub(crate) struct State<const TAG_LEN: usize, const NUM_KEYS: usize, T: AESState> {
    // pub(crate) aes_state: AesCcm128CtrContext<T>,
    pub(crate) aes_state: AesCtrContext<T, NUM_KEYS, AES_CCM_CTR_LEN, 1>,
    pub(crate) accumulator: [u8; AES_BLOCK_LEN],
}

macro_rules! ccm_num_keys {
    ($num_keys:literal) => {
        impl<const TAG_LEN: usize, T: AESState> State<TAG_LEN, $num_keys, T> {
            #[inline]
            // Nonce must be set first
            pub fn ccm_update_aad(&mut self, aad: &[u8], payload_len: usize) {
                // First block
                // We need this to get the right slices from the end
                // of `x.len().to_be_bytes()` where `x` is a usize.
                const USIZE_LEN: usize = core::mem::size_of::<usize>();

                // `MSG_ENC_LEN` is 3, so this should always be the
                // case.
                debug_assert!(MSG_ENC_LEN <= USIZE_LEN);
                debug_assert!(MSG_ENC_LEN <= AES_BLOCK_LEN);
                self.accumulator[0] = 64 * (!aad.is_empty() as u8)
                    + ((TAG_LEN as u8 - 2) / 2) * 8
                    + (MSG_ENC_LEN as u8)
                    - 1;

                self.accumulator[AES_BLOCK_LEN - MSG_ENC_LEN as usize..].copy_from_slice(
                    &payload_len.to_be_bytes()[USIZE_LEN - MSG_ENC_LEN as usize..],
                );

                let mut st = T::new();
                st.load_block(&self.accumulator);
                block_cipher(&mut st, &self.aes_state.extended_key);

                st.store_block(&mut self.accumulator);

                // Encode AAD length
                let aad_len = aad.len();
                if aad_len == 0 {
                    return;
                }

                let mut current_block = [0u8; AES_BLOCK_LEN];

                let mut aad_len_encoding_len = 2;
                if TWO_BYTE_ENCODING_RANGE.contains(&aad_len) {
                    current_block[0..2].copy_from_slice(
                        &aad_len.to_be_bytes()[USIZE_LEN - aad_len_encoding_len..],
                    );
                }
                if SIX_BYTE_ENCODING_RANGE.contains(&aad_len) {
                    aad_len_encoding_len = 6;
                    current_block[0] = 0xff;
                    current_block[1] = 0xfe;
                    current_block[2..4].copy_from_slice(
                        &aad_len.to_be_bytes()[USIZE_LEN - aad_len_encoding_len + 2..],
                    );
                } else if TEN_BYTE_ENCODING_RANGE.contains(&aad_len) {
                    aad_len_encoding_len = 10;
                    current_block[0] = 0xff;
                    current_block[1] = 0xfe;
                    current_block[2..8].copy_from_slice(&aad_len.to_be_bytes());
                }

                if aad_len + aad_len_encoding_len <= AES_BLOCK_LEN {
                    current_block[aad_len_encoding_len..aad_len + aad_len_encoding_len]
                        .copy_from_slice(&aad);

                    self.accumulate(current_block.as_slice());
                } else {
                    let full_blocks = (aad_len_encoding_len + aad_len) / AES_BLOCK_LEN;
                    let remainder = (aad_len_encoding_len + aad_len) - full_blocks * AES_BLOCK_LEN;
                    let initial_aad_chunk_len = AES_BLOCK_LEN - aad_len_encoding_len;

                    for i in 0..full_blocks {
                        if i == 0 {
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

            pub fn ccm_update_plaintext(&mut self, payload: &[u8]) {
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

            fn ccm_update_ciphertext(&mut self, ciphertext: &[u8]) {
                let full_blocks = ciphertext.len() / AES_BLOCK_LEN;
                let remainder = ciphertext.len() - full_blocks * AES_BLOCK_LEN;

                let mut key_block = [0u8; AES_BLOCK_LEN];

                for i in 0..full_blocks {
                    self.aes_state.key_block((i + 1) as u32, &mut key_block);
                    let offset = i * AES_BLOCK_LEN;
                    for j in 0..AES_BLOCK_LEN {
                        key_block[j] ^= ciphertext[offset + j]
                    }

                    self.accumulate(key_block.as_slice());
                }

                if remainder != 0 {
                    self.aes_state
                        .key_block((full_blocks + 1) as u32, &mut key_block);
                    let offset = full_blocks * AES_BLOCK_LEN;
                    for j in 0..remainder {
                        key_block[j] ^= ciphertext[offset + j]
                    }

                    self.accumulate(&key_block[0..remainder]);
                }
            }

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
    };
}

ccm_num_keys!(11); // AES-128
ccm_num_keys!(15); // AES-256

pub(crate) type AesCcm128State<T> = State<TAG_LEN, 11, T>;
#[allow(non_camel_case_types)]
pub(crate) type AesCcm128_8_State<T> = State<CCM_SHORT_TAG_LEN, 11, T>;

pub(crate) type AesCcm256State<T> = State<TAG_LEN, 15, T>;
#[allow(non_camel_case_types)]
pub(crate) type AesCcm256_8_State<T> = State<CCM_SHORT_TAG_LEN, 15, T>;

aesccm!(AesCcm128State<T>, AesCcm128CtrContext, 16);
aesccm!(AesCcm128_8_State<T>, AesCcm128CtrContext, 16);

aesccm!(AesCcm256State<T>, AesCcm256CtrContext, 32);
aesccm!(AesCcm256_8_State<T>, AesCcm256CtrContext, 32);

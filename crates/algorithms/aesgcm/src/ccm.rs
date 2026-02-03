//! AES ctr mode implementation.
//!
//! This implementation is generic over the [`AESState`], which has different,
//! platform dependent implementations.
//!
//! This get's instantiated in [`aes128_ctr`] and [`aes256_ctr`].

use crate::{aes::*, platform::AESState};

/// The ctr nonce length. This is different from the AES nonce length
/// [`crate::NONCE_LEN`].
const NONCE_LEN: usize = 16;
const CCM_TAG_LEN: usize = 16;
const MSG_ENC_LEN: usize = 3;

pub(crate) type Aes128CcmContext<T> = AesCcmContext<T, 11>;

/// Generic AES CCM context.
pub(crate) struct AesCcmContext<T: AESState, const NUM_KEYS: usize> {
    pub(crate) extended_key: ExtendedKey<T, NUM_KEYS>,
    pub(crate) accumulator: [u8; AES_BLOCK_LEN],
}

impl<T: AESState, const NUM_KEYS: usize> AesCcmContext<T, NUM_KEYS> {
    #[inline]
    pub(crate) fn init(extended_key: &ExtendedKey<T, NUM_KEYS>) -> Self {
        Self {
            extended_key: extended_key.clone(),
            accumulator: [0u8; AES_BLOCK_LEN],
        }
    }

    #[inline]
    pub(crate) fn set_nonce(&mut self, nonce: &[u8]) {
        self.accumulator[1..16 - MSG_ENC_LEN as usize].copy_from_slice(nonce);
    }

    #[inline]
    // Nonce must be set first
    pub fn update_aad(&mut self, aad: &[u8], payload_len: usize) {
        // First block
        self.accumulator[0] |= (aad.is_empty() as u8) << 6;
        self.accumulator[0] |= ((CCM_TAG_LEN as u8 - 2) / 2) << 3;
        self.accumulator[0] |= MSG_ENC_LEN as u8 - 1;

        self.accumulator[16 - MSG_ENC_LEN as usize..]
            .copy_from_slice(&payload_len.to_be_bytes()[8 - MSG_ENC_LEN as usize..]);

        let mut st = T::new();
        st.load_block(&self.accumulator);
        block_cipher(&mut st, &self.extended_key);

        st.store_block(&mut self.accumulator);

        // Encode AAD length
        let aad_len = aad.len();
        if aad_len == 0 {
            return;
        }

        let mut current_block = [0u8; AES_BLOCK_LEN];

        let mut aad_len_encoding = [0u8; 10];
        let mut aad_len_encoding_len = 2;
        &aad_len_encoding[2..].copy_from_slice(&aad_len.to_be_bytes());

        if aad_len >= (1 << 16) - (1 << 8) && aad_len < (1 << 32) {
            aad_len_encoding_len = 6;
            aad_len_encoding[7] = 0xfe;
            aad_len_encoding[6] = 0xff;
        } else if aad_len >= (1 << 32) {
            aad_len_encoding_len = 10;
            aad_len_encoding[1] = 0xff;
            aad_len_encoding[0] = 0xff;
        }

        let aad_len_encoding = &aad_len_encoding[10 - aad_len_encoding_len..];
        current_block[..aad_len_encoding_len].copy_from_slice(aad_len_encoding);

        let mut current_block_index = aad_len_encoding_len as usize;
        let full_blocks = (aad_len_encoding_len + aad_len) / AES_BLOCK_LEN;
        let remainder = (aad_len_encoding_len + aad_len) * AES_BLOCK_LEN - full_blocks;

        for i in 0..full_blocks {
            current_block[current_block_index..].copy_from_slice(
                &aad[i * AES_BLOCK_LEN..(i + 1) * AES_BLOCK_LEN - current_block_index],
            );

            for j in 0..AES_BLOCK_LEN {
                self.accumulator[j] ^= current_block[j];
            }

            let mut st = T::new();
            st.load_block(&self.accumulator);
            block_cipher(&mut st, &self.extended_key);
            st.store_block(&mut self.accumulator);

            current_block_index = 0;
        }

        current_block = [0u8; AES_BLOCK_LEN];

        current_block[current_block_index..remainder]
            .copy_from_slice(&aad[(full_blocks * AES_BLOCK_LEN) - aad_len_encoding_len..]);

        for i in 0..AES_BLOCK_LEN {
            current_block[i] = current_block[i] ^ self.accumulator[i];
        }

        let mut st = T::new();
        st.load_block(&current_block);
        block_cipher(&mut st, &self.extended_key);
        st.store_block(&mut self.accumulator);
    }

    pub fn update_plaintext(&mut self, payload: &[u8]) {
        let full_blocks = payload.len() / AES_BLOCK_LEN;
        let remainder = payload.len() - full_blocks * AES_BLOCK_LEN;

        for i in 0..full_blocks {
            let offset = i * AES_BLOCK_LEN;
            for j in 0..AES_BLOCK_LEN {
                self.accumulator[j] ^= payload[offset + j];
            }
            let mut st = T::new();
            st.load_block(&self.accumulator);
            block_cipher(&mut st, &self.extended_key);
            st.store_block(&mut self.accumulator);
        }

        let mut final_block = [0u8; AES_BLOCK_LEN];
        final_block[..remainder].copy_from_slice(
            &payload[full_blocks * AES_BLOCK_LEN..full_blocks * AES_BLOCK_LEN + remainder],
        );

        for j in 0..AES_BLOCK_LEN {
            self.accumulator[j] ^= final_block[j];
        }
        let mut st = T::new();
        st.load_block(&self.accumulator);
        block_cipher(&mut st, &self.extended_key);
        st.store_block(&mut self.accumulator);
    }

    pub(crate) fn update_ciphertext(&mut self, ciphertext: &[u8]) {}

    pub(crate) fn update(&mut self, aad: &[u8], plaintext: &[u8]) {
        self.update_aad(aad, plaintext);
        self.update_plaintext(plaintext);
    }

    pub(crate) fn emit(&self, out: &mut [u8]) {
        out.copy_from_slice(&self.accumulator[..CCM_TAG_LEN]);
    }
}

use crate::{
    aes::{block_cipher, AES_BLOCK_LEN},
    aes_ccm::aesccm,
    ctr::{AesCcm128CtrContext, AesCtrContext, AesGcm128CtrContext, AesGcm256CtrContext},
    platform::AESState,
    DecryptError, NONCE_LEN as AES_NONCE, TAG_LEN,
};

/// AES-CCM 128 key length.
pub const KEY_LEN: usize = 16;

/// The ctr nonce length. This is different from the AES nonce length
/// [`crate::NONCE_LEN`].
const NONCE_LEN: usize = 12;
const CCM_TAG_LEN: usize = 16;
const MSG_ENC_LEN: usize = 3;
pub(crate) const AES_CCM_CTR_LEN: usize = 3;

/// The AES-CCM 128 state
pub(crate) struct State<const TAG_LEN: usize, T: AESState> {
    pub(crate) aes_state: AesCcm128CtrContext<T>,
    pub(crate) accumulator: [u8; AES_BLOCK_LEN],
}

impl<const TAG_LEN: usize, T: AESState> State<TAG_LEN, T> {
    #[inline]
    // Nonce must be set first
    pub fn ccm_update_aad(&mut self, aad: &[u8], payload_len: usize) {
        // First block
        self.accumulator[0] =
            64 * (!aad.is_empty() as u8) + ((CCM_TAG_LEN as u8 - 2) / 2) * 8 + (MSG_ENC_LEN as u8)
                - 1;

        // XXX: This assumes usize is 64 bits wide, should be made more robust.
        self.accumulator[AES_BLOCK_LEN - MSG_ENC_LEN as usize..]
            .copy_from_slice(&payload_len.to_be_bytes()[8 - MSG_ENC_LEN as usize..]);

        println!("B_0: {:?}", self.accumulator);
        let mut st = T::new();
        st.load_block(&self.accumulator);
        block_cipher(&mut st, &self.aes_state.extended_key);

        st.store_block(&mut self.accumulator);

        // Encode AAD length
        let aad_len = aad.len();
        if aad_len == 0 {
            println!("No AAD, returning");
            return;
        }

        let mut current_block = [0u8; AES_BLOCK_LEN];

        let mut aad_len_encoding_len = 2;
        if aad_len < (1 << 16) - (1 << 8) {
            current_block[0..2].copy_from_slice(&aad_len.to_be_bytes()[6..]);
        }
        if aad_len >= (1 << 16) - (1 << 8) && aad_len < (1 << 32) {
            aad_len_encoding_len = 6;
            current_block[0] = 0xff;
            current_block[1] = 0xfe;
            current_block[2..4].copy_from_slice(&aad_len.to_be_bytes()[4..]);
        } else if aad_len >= (1 << 32) {
            aad_len_encoding_len = 10;
            current_block[0] = 0xff;
            current_block[1] = 0xfe;
            current_block[2..8].copy_from_slice(&aad_len.to_be_bytes());
        }

        println!("AAD length: {aad_len}");
        println!("AAD length encoding: {:?}", current_block);

        if aad_len + aad_len_encoding_len <= AES_BLOCK_LEN {
            current_block[aad_len_encoding_len..aad_len + aad_len_encoding_len]
                .copy_from_slice(&aad);

            self.accumulate(current_block.as_slice());
        } else {
            let full_blocks = (aad_len_encoding_len + aad_len) / AES_BLOCK_LEN;
            let remainder = (aad_len_encoding_len + aad_len) - full_blocks * AES_BLOCK_LEN;
            let initial_aad_chunk_len = AES_BLOCK_LEN - aad_len_encoding_len;
            println!("AAD Full blocks: {full_blocks}, remaining bytes: {remainder}");

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
        println!("Payload Full blocks: {full_blocks}, remaining bytes: {remainder}");

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
            self.aes_state
                .key_block((i + 1) as u32, key_block.as_mut_slice());
            let offset = i * AES_BLOCK_LEN;
            for j in 0..AES_BLOCK_LEN {
                key_block[j] ^= ciphertext[offset + j]
            }

            self.accumulate(key_block.as_slice());
        }

        if remainder != 0 {
            self.aes_state
                .key_block((full_blocks + 1) as u32, key_block.as_mut_slice());
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

pub(crate) type AesCcm128State<T: AESState> = State<16, T>;

aesccm!(AesCcm128State<T>, AesCcm128CtrContext);

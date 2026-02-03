//! Implementation of AES-CCM

/// Macro to instantiate the AES state.
/// This should really be replaced by using traits everywhere.
macro_rules! aesccm {
    ($state:ty, $ctr_context:ident) => {
        impl<T: AESState> super::State for $state {
            /// Initialize the state
            fn init(key: &[u8]) -> Self {
                debug_assert!(key.len() == KEY_LEN);

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

pub(crate) use aesccm;

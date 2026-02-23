//! Implementation of AES-GCM

/// Macro to instantiate the AES state.
/// This should really be replaced by using traits everywhere.
macro_rules! aesgcm {
    ($state:ty, $context:ident) => {
        impl<T: AESState, U: GF128FieldElement> super::State for $state {
            /// Initialize the state
            fn init(key: &[u8]) -> Self {
                debug_assert!(key.len() == KEY_LEN);

                let nonce = [0u8; NONCE_LEN];
                let mut gcm_key = [0u8; GCM_KEY_LEN];
                let tag_mix = [0u8; TAG_LEN];

                let aes_state = $context::<T>::init(key, &nonce);
                aes_state.key_block(0, &mut gcm_key);
                let gcm_state = GF128State::init(&gcm_key);

                Self {
                    aes_state,
                    gcm_state,
                    tag_mix,
                }
            }

            fn set_nonce(&mut self, nonce: &[u8]) {
                debug_assert!(nonce.len() == NONCE_LEN);

                self.aes_state.set_nonce(nonce);
                self.aes_state.key_block(1, &mut self.tag_mix);
            }

            fn encrypt(
                &mut self,
                aad: &[u8],
                plaintext: &[u8],
                ciphertext: &mut [u8],
                tag: &mut [u8],
            ) {
                debug_assert!(ciphertext.len() == plaintext.len());
                debug_assert!(plaintext.len() / AES_BLOCK_LEN <= u32::MAX as usize);
                debug_assert!(tag.len() == TAG_LEN);

                self.aes_state.update(2, plaintext, ciphertext);

                self.gcm_state.update_padded(aad);
                self.gcm_state.update_padded(ciphertext);

                let mut last_block = [0u8; AES_BLOCK_LEN];
                last_block[0..8].copy_from_slice(&((aad.len() as u64) * 8).to_be_bytes());
                last_block[8..16].copy_from_slice(&((plaintext.len() as u64) * 8).to_be_bytes());

                self.gcm_state.update(&last_block);
                self.gcm_state.emit(tag);

                for i in 0..16 {
                    tag[i] ^= self.tag_mix[i];
                }
            }

            fn decrypt(
                &mut self,
                aad: &[u8],
                ciphertext: &[u8],
                tag: &[u8],
                plaintext: &mut [u8],
            ) -> Result<(), DecryptError> {
                debug_assert!(plaintext.len() == ciphertext.len());
                debug_assert!(ciphertext.len() / AES_BLOCK_LEN <= u32::MAX as usize);
                debug_assert!(tag.len() == TAG_LEN);

                self.gcm_state.update_padded(aad);
                self.gcm_state.update_padded(ciphertext);

                let mut last_block = [0u8; AES_BLOCK_LEN];
                last_block[0..8].copy_from_slice(&((aad.len() as u64) * 8).to_be_bytes());
                last_block[8..16].copy_from_slice(&((plaintext.len() as u64) * 8).to_be_bytes());

                self.gcm_state.update(&last_block);

                let mut computed_tag = [0u8; TAG_LEN];
                self.gcm_state.emit(&mut computed_tag);

                for i in 0..16 {
                    computed_tag[i] ^= self.tag_mix[i];
                }

                let mut eq_mask = 0u8;
                for i in 0..16 {
                    eq_mask |= computed_tag[i] ^ tag[i];
                }

                if eq_mask == 0 {
                    self.aes_state.update(2, ciphertext, plaintext);
                    Ok(())
                } else {
                    Err(DecryptError::InvalidTag)
                }
            }
        }
    };
}

pub(crate) use aesgcm;

/// Helper module for implementing platform-specific modules
macro_rules! type_aliases {
    ($implementation:ident, $alg_name:literal) => {
        pub use crate::implementations::$implementation;
        #[doc = concat!("An owned key for ",$alg_name, ".")]
        pub type Key = libcrux_traits::aead::typed_owned::Key<$implementation>;
        #[doc = concat!("An owned tag for ",$alg_name, ".")]
        pub type Tag = libcrux_traits::aead::typed_owned::Tag<$implementation>;
        #[doc = concat!("An owned nonce for ",$alg_name, ".")]
        pub type Nonce = libcrux_traits::aead::typed_owned::Nonce<$implementation>;
        #[doc = concat!("A reference to a key for ",$alg_name, ".")]
        pub type KeyRef<'a> = libcrux_traits::aead::typed_refs::KeyRef<'a, $implementation>;
        #[doc = concat!("A reference to a tag for ",$alg_name, ".")]
        pub type TagRef<'a> = libcrux_traits::aead::typed_refs::TagRef<'a, $implementation>;
        #[doc = concat!("A mutable reference to a tag for ",$alg_name, ".")]
        pub type TagMut<'a> = libcrux_traits::aead::typed_refs::TagMut<'a, $implementation>;
        #[doc = concat!("A reference to a nonce for ",$alg_name, ".")]
        pub type NonceRef<'a> = libcrux_traits::aead::typed_refs::NonceRef<'a, $implementation>;
    };
}

pub(crate) use type_aliases;

use libcrux_secrets::U8;
use libcrux_traits::aead::{
    self,
    slice::KeyGenError,
    typed_refs::{DecryptError, EncryptError, KeyMut, Multiplexes},
};

/// A multiplexed AEAD, allowing algorithm selection at run time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Aead {
    /// The ChaCha20Poly1305 AEAD algorithm.
    #[cfg(feature = "chacha20poly1305")]
    ChaCha20Poly1305,

    /// The XChaCha20Poly1305 AEAD algorithm.
    #[cfg(feature = "xchacha20poly1305")]
    XChaCha20Poly1305,

    /// The AES-GCM 128 AEAD algorithm.
    #[cfg(feature = "aesgcm128")]
    AesGcm128,

    /// The AES-GCM 256 AEAD algorithm.
    #[cfg(feature = "aesgcm256")]
    AesGcm256,
}

/// A reference to a multiplexed AEAD key.
pub type KeyRef<'a> = aead::typed_refs::KeyRef<'a, Aead>;
/// A reference to a multiplexed AEAD tag.
pub type TagRef<'a> = aead::typed_refs::TagRef<'a, Aead>;
/// A mutable reference to a multiplexed AEAD tag.
pub type TagMut<'a> = aead::typed_refs::TagMut<'a, Aead>;
/// A reference to a multiplexed AEAD nonce.
pub type NonceRef<'a> = aead::typed_refs::NonceRef<'a, Aead>;

#[cfg(feature = "chacha20poly1305")]
impl Multiplexes<crate::chacha20poly1305::ChaCha20Poly1305> for Aead {
    fn mux_algo(&self) -> Option<crate::chacha20poly1305::ChaCha20Poly1305> {
        matches!(self, Self::ChaCha20Poly1305).then_some(crate::chacha20poly1305::ChaCha20Poly1305)
    }
    fn wrap_algo(_algo: crate::chacha20poly1305::ChaCha20Poly1305) -> Self {
        Self::ChaCha20Poly1305
    }
}

#[cfg(feature = "xchacha20poly1305")]
impl Multiplexes<crate::xchacha20poly1305::XChaCha20Poly1305> for Aead {
    fn mux_algo(&self) -> Option<crate::xchacha20poly1305::XChaCha20Poly1305> {
        matches!(self, Self::XChaCha20Poly1305)
            .then_some(crate::xchacha20poly1305::XChaCha20Poly1305)
    }
    fn wrap_algo(_algo: crate::xchacha20poly1305::XChaCha20Poly1305) -> Self {
        Self::XChaCha20Poly1305
    }
}

#[cfg(feature = "aesgcm128")]
impl Multiplexes<crate::aesgcm128::AesGcm128> for Aead {
    fn mux_algo(&self) -> Option<crate::aesgcm128::AesGcm128> {
        matches!(self, Self::AesGcm128).then_some(crate::aesgcm128::AesGcm128)
    }
    fn wrap_algo(_algo: crate::aesgcm128::AesGcm128) -> Self {
        Self::AesGcm128
    }
}

#[cfg(feature = "aesgcm256")]
impl Multiplexes<crate::aesgcm256::AesGcm256> for Aead {
    fn mux_algo(&self) -> Option<crate::aesgcm256::AesGcm256> {
        matches!(self, Self::AesGcm256).then_some(crate::aesgcm256::AesGcm256)
    }
    fn wrap_algo(_algo: crate::aesgcm256::AesGcm256) -> Self {
        Self::AesGcm256
    }
}

impl aead::typed_refs::Aead for Aead {
    fn key_len(&self) -> usize {
        match *self {
            #[cfg(feature = "chacha20poly1305")]
            Aead::ChaCha20Poly1305 => 32,
            #[cfg(feature = "xchacha20poly1305")]
            Aead::XChaCha20Poly1305 => 32,
            #[cfg(feature = "aesgcm128")]
            Aead::AesGcm128 => 16,
            #[cfg(feature = "aesgcm256")]
            Aead::AesGcm256 => 32,
        }
    }

    fn tag_len(&self) -> usize {
        16
    }

    fn nonce_len(&self) -> usize {
        match *self {
            #[cfg(feature = "chacha20poly1305")]
            Aead::ChaCha20Poly1305 => 12,
            #[cfg(feature = "xchacha20poly1305")]
            Aead::XChaCha20Poly1305 => 24,
            #[cfg(feature = "aesgcm128")]
            Aead::AesGcm128 => 12,
            #[cfg(feature = "aesgcm256")]
            Aead::AesGcm256 => 12,
        }
    }

    fn keygen<'a>(&self, key: KeyMut<'a, Self>, rand: &[U8]) -> Result<(), KeyGenError> {
        match *self {
            #[cfg(feature = "chacha20poly1305")]
            Aead::ChaCha20Poly1305 => {
                use crate::chacha20poly1305::ChaCha20Poly1305;

                let key = Self::mux_key_mut(key).ok_or(KeyGenError::WrongKeyLength)?;
                ChaCha20Poly1305.keygen(key, rand)
            }
            #[cfg(feature = "xchacha20poly1305")]
            Aead::XChaCha20Poly1305 => {
                use crate::xchacha20poly1305::XChaCha20Poly1305;

                let key = Self::mux_key_mut(key).ok_or(KeyGenError::WrongKeyLength)?;
                XChaCha20Poly1305.keygen(key, rand)
            }
            #[cfg(feature = "aesgcm128")]
            Aead::AesGcm128 => {
                use crate::aesgcm128::AesGcm128;

                let key = Self::mux_key_mut(key).ok_or(KeyGenError::WrongKeyLength)?;

                AesGcm128.keygen(key, rand)
            }
            #[cfg(feature = "aesgcm256")]
            Aead::AesGcm256 => {
                use crate::aesgcm256::AesGcm256;

                let key = Self::mux_key_mut(key).ok_or(KeyGenError::WrongKeyLength)?;

                AesGcm256.keygen(key, rand)
            }
        }
    }

    fn encrypt<'a>(
        &self,
        ciphertext: &mut [u8],
        tag: TagMut<'a>,
        key: KeyRef<'a>,
        nonce: NonceRef<'a>,
        aad: &[u8],
        plaintext: &[U8],
    ) -> Result<(), aead::typed_refs::EncryptError> {
        match *self {
            #[cfg(feature = "chacha20poly1305")]
            Aead::ChaCha20Poly1305 => {
                use crate::chacha20poly1305::ChaCha20Poly1305;

                let key = Self::mux_key(key).ok_or(EncryptError::WrongKey)?;
                let tag = Self::mux_tag_mut(tag).ok_or(EncryptError::WrongTag)?;
                let nonce = Self::mux_nonce(nonce).ok_or(EncryptError::WrongNonce)?;
                ChaCha20Poly1305.encrypt(ciphertext, tag, key, nonce, aad, plaintext)
            }
            #[cfg(feature = "xchacha20poly1305")]
            Aead::XChaCha20Poly1305 => {
                use crate::xchacha20poly1305::XChaCha20Poly1305;

                let key = Self::mux_key(key).ok_or(EncryptError::WrongKey)?;
                let tag = Self::mux_tag_mut(tag).ok_or(EncryptError::WrongTag)?;
                let nonce = Self::mux_nonce(nonce).ok_or(EncryptError::WrongNonce)?;
                XChaCha20Poly1305.encrypt(ciphertext, tag, key, nonce, aad, plaintext)
            }
            #[cfg(feature = "aesgcm128")]
            Aead::AesGcm128 => {
                use crate::aesgcm128::AesGcm128;

                let key = Self::mux_key(key).ok_or(EncryptError::WrongKey)?;
                let tag = Self::mux_tag_mut(tag).ok_or(EncryptError::WrongTag)?;
                let nonce = Self::mux_nonce(nonce).ok_or(EncryptError::WrongNonce)?;
                AesGcm128.encrypt(ciphertext, tag, key, nonce, aad, plaintext)
            }
            #[cfg(feature = "aesgcm256")]
            Aead::AesGcm256 => {
                use crate::aesgcm256::AesGcm256;

                let key = Self::mux_key(key).ok_or(EncryptError::WrongKey)?;
                let tag = Self::mux_tag_mut(tag).ok_or(EncryptError::WrongTag)?;
                let nonce = Self::mux_nonce(nonce).ok_or(EncryptError::WrongNonce)?;
                AesGcm256.encrypt(ciphertext, tag, key, nonce, aad, plaintext)
            }
        }
    }

    fn decrypt<'a>(
        &self,
        plaintext: &mut [U8],
        key: KeyRef<'a>,
        nonce: NonceRef<'a>,
        aad: &[u8],
        ciphertext: &[u8],
        tag: TagRef<'a>,
    ) -> Result<(), aead::typed_refs::DecryptError> {
        match *self {
            #[cfg(feature = "chacha20poly1305")]
            Aead::ChaCha20Poly1305 => {
                use crate::chacha20poly1305::ChaCha20Poly1305;

                let key = Self::mux_key(key).ok_or(DecryptError::WrongKey)?;
                let tag = Self::mux_tag(tag).ok_or(DecryptError::WrongTag)?;
                let nonce = Self::mux_nonce(nonce).ok_or(DecryptError::WrongNonce)?;
                ChaCha20Poly1305.decrypt(plaintext, key, nonce, aad, ciphertext, tag)
            }
            #[cfg(feature = "xchacha20poly1305")]
            Aead::XChaCha20Poly1305 => {
                use crate::xchacha20poly1305::XChaCha20Poly1305;

                let key = Self::mux_key(key).ok_or(DecryptError::WrongKey)?;
                let tag = Self::mux_tag(tag).ok_or(DecryptError::WrongTag)?;
                let nonce = Self::mux_nonce(nonce).ok_or(DecryptError::WrongNonce)?;
                XChaCha20Poly1305.decrypt(plaintext, key, nonce, aad, ciphertext, tag)
            }
            #[cfg(feature = "aesgcm128")]
            Aead::AesGcm128 => {
                use crate::aesgcm128::AesGcm128;

                let key = Self::mux_key(key).ok_or(DecryptError::WrongKey)?;
                let tag = Self::mux_tag(tag).ok_or(DecryptError::WrongTag)?;
                let nonce = Self::mux_nonce(nonce).ok_or(DecryptError::WrongNonce)?;
                AesGcm128.decrypt(plaintext, key, nonce, aad, ciphertext, tag)
            }
            #[cfg(feature = "aesgcm256")]
            Aead::AesGcm256 => {
                use crate::aesgcm256::AesGcm256;

                let key = Self::mux_key(key).ok_or(DecryptError::WrongKey)?;
                let tag = Self::mux_tag(tag).ok_or(DecryptError::WrongTag)?;
                let nonce = Self::mux_nonce(nonce).ok_or(DecryptError::WrongNonce)?;
                AesGcm256.decrypt(plaintext, key, nonce, aad, ciphertext, tag)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use libcrux_traits::aead::typed_refs;
    use typed_refs::Aead as _;

    use super::Aead;

    #[test]
    #[cfg(feature = "chacha20poly1305")]
    fn test_key_centric_multiplexed_chachapoly() {
        use libcrux_traits::libcrux_secrets::{Classify, ClassifyRef, DeclassifyRef};

        let algo = Aead::ChaCha20Poly1305;

        algo.new_key(&[0.classify(); 33])
            .expect_err("length should mismatch");

        let mut tag_bytes = [0.classify(); 16];
        let key_bytes = [0.classify(); 32];
        let nonce_bytes = [0.classify(); 12];

        let key = algo.new_key(&key_bytes).expect("length should match");
        let nonce = algo.new_nonce(&nonce_bytes).expect("length should match");
        let tag = algo
            .new_tag_mut(&mut tag_bytes)
            .expect("length should match");

        let pt = b"the quick brown fox jumps over the lazy dog".classify_ref();
        let mut ct = [0; 43];
        let mut pt_out = [0.classify(); 43];

        key.encrypt(&mut ct, tag, nonce, b"", pt).unwrap();
        let tag = algo.new_tag(&tag_bytes).unwrap();
        key.decrypt(&mut pt_out, nonce, b"", &ct, tag).unwrap();
        assert_eq!(pt.declassify_ref(), pt_out.declassify_ref());
    }

    #[test]
    #[cfg(feature = "xchacha20poly1305")]
    fn test_key_centric_multiplexed_xchachapoly() {
        use libcrux_traits::libcrux_secrets::{Classify, ClassifyRef, DeclassifyRef};

        let algo = Aead::XChaCha20Poly1305;

        let wrong_length_key_bytes = [0.classify(); 33];
        algo.new_key(&wrong_length_key_bytes)
            .expect_err("length should mismatch");

        let mut tag_bytes = [0.classify(); 16];

        let key_bytes = [0.classify(); 32];
        let nonce_bytes = [0.classify(); 24];

        let key = algo.new_key(&key_bytes).expect("length should match");
        let nonce = algo.new_nonce(&nonce_bytes).expect("length should match");
        let tag = algo
            .new_tag_mut(&mut tag_bytes)
            .expect("length should match");

        let pt = b"the quick brown fox jumps over the lazy dog".classify_ref();
        let mut ct = [0; 43];
        let mut pt_out = [0.classify(); 43];

        key.encrypt(&mut ct, tag, nonce, b"", pt).unwrap();
        let tag = algo.new_tag(&tag_bytes).unwrap();
        key.decrypt(&mut pt_out, nonce, b"", &ct, tag).unwrap();
        assert_eq!(pt.declassify_ref(), pt_out.declassify_ref());
    }

    #[test]
    #[cfg(feature = "aesgcm128")]
    fn test_key_centric_multiplexed_aesgcm128() {
        use libcrux_traits::libcrux_secrets::{Classify, ClassifyRef, DeclassifyRef};

        let algo = Aead::AesGcm128;

        algo.new_key(&[0.classify(); 33])
            .expect_err("length should mismatch");

        let mut tag_bytes = [0.classify(); 16];
        let key_bytes = [0.classify(); 16];
        let nonce_bytes = [0.classify(); 12];

        let key = algo.new_key(&key_bytes).expect("length should match");
        let nonce = algo.new_nonce(&nonce_bytes).expect("length should match");
        let tag = algo
            .new_tag_mut(&mut tag_bytes)
            .expect("length should match");

        let pt = b"the quick brown fox jumps over the lazy dog".classify_ref();
        let mut ct = [0; 43];
        let mut pt_out = [0.classify(); 43];

        key.encrypt(&mut ct, tag, nonce, b"", pt).unwrap();
        let tag = algo.new_tag(&tag_bytes).unwrap();
        key.decrypt(&mut pt_out, nonce, b"", &ct, tag).unwrap();
        assert_eq!(pt.declassify_ref(), pt_out.declassify_ref());
    }

    #[test]
    #[cfg(feature = "aesgcm256")]
    fn test_key_centric_multiplexed_aesgcm256() {
        use libcrux_traits::libcrux_secrets::{Classify, ClassifyRef, DeclassifyRef};

        let algo = Aead::AesGcm256;

        algo.new_key(&[0.classify(); 33])
            .expect_err("length should mismatch");

        let mut tag_bytes = [0.classify(); 16];
        let key_bytes = [0.classify(); 32];
        let nonce_bytes = [0.classify(); 12];

        let key = algo.new_key(&key_bytes).expect("length should match");
        let nonce = algo.new_nonce(&nonce_bytes).expect("length should match");
        let tag = algo
            .new_tag_mut(&mut tag_bytes)
            .expect("length should match");

        let pt = b"the quick brown fox jumps over the lazy dog".classify_ref();
        let mut ct = [0; 43];
        let mut pt_out = [0.classify(); 43];

        key.encrypt(&mut ct, tag, nonce, b"", pt).unwrap();
        let tag = algo.new_tag(&tag_bytes).unwrap();
        key.decrypt(&mut pt_out, nonce, b"", &ct, tag).unwrap();
        assert_eq!(pt.declassify_ref(), pt_out.declassify_ref());
    }
}

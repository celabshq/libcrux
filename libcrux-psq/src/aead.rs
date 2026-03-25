//! # AEAD API
use libcrux_aes::AES_128_KEY_LEN as KEY_LEN_AES;
use libcrux_chacha20poly1305::{decrypt_detached, encrypt_detached, KEY_LEN as KEY_LEN_CHACHA};
use libcrux_traits::aead::arrayref::Aead;
use tls_codec::{
    Deserialize, Serialize, SerializeBytes, TlsDeserialize, TlsSerialize, TlsSerializeBytes,
    TlsSize,
};

/// Length of an AEAD nonce in bytes.
pub const NONCE_LEN: usize = 12;

#[cfg(not(feature = "nonce-control"))]
const NONCE_MAX: [u8; NONCE_LEN] = [0xff; NONCE_LEN];
const TAG_LEN: usize = 16;

#[derive(Clone, TlsSerialize, TlsDeserialize, TlsSerializeBytes, TlsSize)]
#[repr(u8)]
/// An AEAD key.
pub(crate) enum AEADKey {
    ChaChaPoly1305([u8; KEY_LEN_CHACHA]),
    AesGcm128([u8; KEY_LEN_AES]),
}

#[derive(Clone, TlsSerialize, TlsDeserialize, TlsSerializeBytes, TlsSize)]
/// An AEAD key.
pub(crate) struct AEADKeyNonce {
    key: AEADKey,
    #[tls_codec(skip)]
    nonce: [u8; NONCE_LEN],
    // We use this to enforce guarantees on using the key for
    // transport encryption. In particular, keys generated as part of
    // the handshake can only be used once, and the derived main
    // session key can never be used for transport encryption
    // directly.
    #[tls_codec(skip)]
    expired: bool,
}

impl std::fmt::Debug for AEADKeyNonce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AEADKey").field(&"***").finish()
    }
}

#[derive(Debug, Copy, Clone, TlsSerialize, TlsDeserialize, TlsSize)]
#[repr(u8)]
pub(crate) enum AeadType {
    ChaCha20Poly1305,
    AesGcm128,
}

/// Errors arising in the creation or use of AEAD keys
#[derive(Debug)]
pub(crate) enum AEADError {
    /// The key should not (anymore) be used for transport encryption,
    /// because it has reached its usage limit or because the nonce is
    /// maxed out.
    KeyExpired,
    /// An error occurred in the underlying AEAD implementation.
    CryptoError,
    /// An error during serialization.
    Serialize(tls_codec::Error),
    /// An error during deserialization.
    Deserialize(tls_codec::Error),
}

impl AEADKeyNonce {
    pub(crate) fn set_nonce(&mut self, nonce: &[u8; NONCE_LEN]) {
        self.nonce = *nonce;
    }

    pub(crate) fn nonce(&self) -> &[u8; NONCE_LEN] {
        &self.nonce
    }

    /// Set the expired flag on the key, indicating that it should not
    /// be used to encrypt any (more) transport messages.
    pub(crate) fn expire(&mut self) {
        self.expired = true;
    }

    /// Create a new key, immediately expiring it.
    ///
    /// Use this for main session keys.
    pub(crate) fn new_expired(
        ikm: &impl SerializeBytes,
        info: &impl SerializeBytes,
        aead_type: AeadType,
    ) -> Result<AEADKeyNonce, AEADError> {
        let mut new_key = Self::new(ikm, info, aead_type)?;
        new_key.expire();
        Ok(new_key)
    }

    pub(crate) fn new(
        ikm: &impl SerializeBytes,
        info: &impl SerializeBytes,
        aead_type: AeadType,
    ) -> Result<AEADKeyNonce, AEADError> {
        match aead_type {
            AeadType::ChaCha20Poly1305 => {
                let mut key = [0u8; KEY_LEN_CHACHA];
                libcrux_hkdf::sha2_256::hkdf(
                    &mut key,
                    &[],
                    &ikm.tls_serialize().map_err(AEADError::Serialize)?,
                    &info.tls_serialize().map_err(AEADError::Serialize)?,
                )
                .map_err(|_| AEADError::CryptoError)?;
                Ok(AEADKeyNonce {
                    key: AEADKey::ChaChaPoly1305(key),
                    nonce: [0u8; NONCE_LEN],
                    expired: false,
                })
            }
            AeadType::AesGcm128 => {
                let mut key = [0u8; KEY_LEN_AES];
                libcrux_hkdf::sha2_256::hkdf(
                    &mut key,
                    &[],
                    &ikm.tls_serialize().map_err(AEADError::Serialize)?,
                    &info.tls_serialize().map_err(AEADError::Serialize)?,
                )
                .map_err(|_| AEADError::CryptoError)?;
                Ok(AEADKeyNonce {
                    key: AEADKey::AesGcm128(key),
                    nonce: [0u8; NONCE_LEN],
                    expired: false,
                })
            }
        }
    }

    // Increment the nonce, treating it as a 12 byte big-endian
    // integer. This will generate an AEADError, if the nonce is
    // already at its maximum value.
    //
    // If feature `nonce-control` is enabled, the nonce will not be
    // incremented.
    //
    fn increment_nonce(&mut self) -> Result<(), AEADError> {
        #[cfg(feature = "nonce-control")]
        {
            return Ok(());
        }
        #[cfg(not(feature = "nonce-control"))]
        {
            if self.nonce == NONCE_MAX {
                self.expired = true;
                return Err(AEADError::KeyExpired);
            }

            let mut buf = [0u8; 16];
            buf[16 - NONCE_LEN..].copy_from_slice(self.nonce.as_slice());
            let mut nonce = u128::from_be_bytes(buf);
            nonce += 1;
            let buf = nonce.to_be_bytes();

            self.nonce.copy_from_slice(&buf[16 - NONCE_LEN..]);
            Ok(())
        }
    }

    pub(crate) fn encrypt(
        &mut self,
        payload: &[u8],
        aad: &[u8],
        ciphertext: &mut [u8],
    ) -> Result<[u8; 16], AEADError> {
        if self.expired {
            return Err(AEADError::KeyExpired);
        }
        // AES-GCM 128 and ChaCha20Poly1305 agree on tag length
        let mut tag = [0u8; TAG_LEN];

        // If feature `nonce-control` is enabled, this is a no-op.
        self.increment_nonce()?;

        match &self.key {
            AEADKey::ChaChaPoly1305(key) => {
                // XXX: We could do better if we'd have an inplace API here.
                encrypt_detached(key, payload, ciphertext, &mut tag, aad, &self.nonce)
                    .map_err(|_| AEADError::CryptoError)?;
            }
            AEADKey::AesGcm128(key) => {
                libcrux_aes::AesGcm128::encrypt(
                    ciphertext,
                    &mut tag,
                    key,
                    &self.nonce,
                    aad,
                    payload,
                )
                .map_err(|_| AEADError::CryptoError)?;
            }
        }

        Ok(tag)
    }

    fn serialize_encrypt<T: Serialize>(
        &mut self,
        payload: &T,
        aad: &[u8],
    ) -> Result<(Vec<u8>, [u8; 16]), AEADError> {
        let serialization_buffer = payload
            .tls_serialize_detached()
            .map_err(AEADError::Serialize)?;

        let mut ciphertext = vec![0u8; serialization_buffer.len()];
        let tag = self.encrypt(&serialization_buffer, aad, &mut ciphertext)?;

        Ok((ciphertext, tag))
    }

    pub(crate) fn handshake_encrypt<T: Serialize>(
        &mut self,
        payload: &T,
        aad: &[u8],
    ) -> Result<(Vec<u8>, [u8; 16]), AEADError> {
        let result = self.serialize_encrypt(payload, aad)?;
        self.expire();
        Ok(result)
    }

    pub(crate) fn decrypt(
        &mut self,
        ciphertext: &[u8],
        tag: &[u8; 16],
        aad: &[u8],
    ) -> Result<Vec<u8>, AEADError> {
        if self.expired {
            return Err(AEADError::KeyExpired);
        }

        // This is to reset the nonce, in case of a decryption
        // error. Crucially, we assume that a decryption error does not
        // reveal anything about the tag or the failed decryption.
        let old_nonce = *self.nonce();
        // If feature `nonce-control` is enabled, this is a no-op.
        self.increment_nonce()?;

        let mut plaintext = vec![0u8; ciphertext.len()];

        match &self.key {
            AEADKey::ChaChaPoly1305(key) => {
                if decrypt_detached(key, &mut plaintext, ciphertext, tag, aad, &self.nonce).is_err()
                {
                    self.set_nonce(&old_nonce);
                    return Err(AEADError::CryptoError);
                }
            }
            AEADKey::AesGcm128(key) => {
                if libcrux_aes::AesGcm128::decrypt(
                    &mut plaintext,
                    key,
                    &self.nonce,
                    aad,
                    ciphertext,
                    tag,
                )
                .is_err()
                {
                    self.set_nonce(&old_nonce);
                    return Err(AEADError::CryptoError);
                }
            }
        }

        Ok(plaintext)
    }

    pub(crate) fn decrypt_out(
        &mut self,
        ciphertext: &[u8],
        tag: &[u8; 16],
        aad: &[u8],
        plaintext: &mut [u8],
    ) -> Result<(), AEADError> {
        if self.expired {
            return Err(AEADError::KeyExpired);
        }

        // This is to reset the nonce, in case of a decryption
        // error. Crucially, we assume that a decryption error does not
        // reveal anything about the tag or the failed decryption.
        let old_nonce = *self.nonce();
        // If feature `nonce-control` is enabled, this is a no-op.
        self.increment_nonce()?;
        debug_assert!(ciphertext.len() <= plaintext.len());

        match &self.key {
            AEADKey::ChaChaPoly1305(key) => {
                if decrypt_detached(key, plaintext, ciphertext, tag, aad, &self.nonce).is_err() {
                    self.set_nonce(&old_nonce);
                    return Err(AEADError::CryptoError);
                }
            }
            AEADKey::AesGcm128(key) => {
                if libcrux_aes::AesGcm128::decrypt(
                    plaintext,
                    key,
                    &self.nonce,
                    aad,
                    ciphertext,
                    tag,
                )
                .is_err()
                {
                    self.set_nonce(&old_nonce);
                    return Err(AEADError::CryptoError);
                }
            }
        }

        Ok(())
    }

    fn decrypt_deserialize<T: Deserialize>(
        &mut self,
        ciphertext: &[u8],
        tag: &[u8; 16],
        aad: &[u8],
    ) -> Result<T, AEADError> {
        let payload_serialized_buf = self.decrypt(ciphertext, tag, aad)?;

        T::tls_deserialize_exact(&payload_serialized_buf).map_err(AEADError::Deserialize)
    }

    pub(crate) fn handshake_decrypt<T: Deserialize>(
        &mut self,
        ciphertext: &[u8],
        tag: &[u8; 16],
        aad: &[u8],
    ) -> Result<T, AEADError> {
        let result = self.decrypt_deserialize(ciphertext, tag, aad)?;
        self.expire();
        Ok(result)
    }
}

impl AsRef<[u8]> for AEADKeyNonce {
    fn as_ref(&self) -> &[u8] {
        match &self.key {
            AEADKey::ChaChaPoly1305(k) => k.as_ref(),
            AEADKey::AesGcm128(k) => k.as_ref(),
        }
    }
}

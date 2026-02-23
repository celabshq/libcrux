//! # AEAD API
use libcrux_chacha20poly1305::{decrypt_detached, encrypt_detached, KEY_LEN as KEY_LEN_CHACHA};
use libcrux_traits::aead::arrayref::Aead;
use tls_codec::{
    Deserialize, Serialize, SerializeBytes, TlsDeserialize, TlsSerialize, TlsSerializeBytes,
    TlsSize,
};

use libcrux_aes::AESGCM128_KEY_LEN as KEY_LEN_AES;

const NONCE_LEN: usize = 12;
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
    /// An error occurred in the underlying AEAD implementation.
    CryptoError,
    /// An error during serialization.
    Serialize(tls_codec::Error),
    /// An error during deserialization.
    Deserialize(tls_codec::Error),
}

impl AEADKeyNonce {
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
                })
            }
        }
    }

    fn increment_nonce(&mut self) -> Result<(), AEADError> {
        if self.nonce == NONCE_MAX {
            return Err(AEADError::CryptoError);
        }

        let mut buf = [0u8; 16];
        buf[16 - NONCE_LEN..].copy_from_slice(self.nonce.as_slice());
        let mut nonce = u128::from_be_bytes(buf);
        nonce += 1;
        let buf = nonce.to_be_bytes();

        self.nonce.copy_from_slice(&buf[16 - NONCE_LEN..]);
        Ok(())
    }

    pub(crate) fn encrypt(
        &mut self,
        payload: &[u8],
        aad: &[u8],
        ciphertext: &mut [u8],
    ) -> Result<[u8; 16], AEADError> {
        // AES-GCM 128 and ChaCha20Poly1305 agree on tag length
        let mut tag = [0u8; TAG_LEN];

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

    pub(crate) fn serialize_encrypt<T: Serialize>(
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

    pub(crate) fn decrypt(
        &mut self,
        ciphertext: &[u8],
        tag: &[u8; 16],
        aad: &[u8],
    ) -> Result<Vec<u8>, AEADError> {
        self.increment_nonce()?;
        let mut plaintext = vec![0u8; ciphertext.len()];

        match &self.key {
            AEADKey::ChaChaPoly1305(key) => {
                decrypt_detached(key, &mut plaintext, ciphertext, tag, aad, &self.nonce)
                    .map_err(|_| AEADError::CryptoError)?;
            }
            AEADKey::AesGcm128(key) => {
                libcrux_aes::AesGcm128::decrypt(
                    &mut plaintext,
                    key,
                    &self.nonce,
                    aad,
                    ciphertext,
                    tag,
                )
                .map_err(|_| AEADError::CryptoError)?;
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
        self.increment_nonce()?;
        debug_assert!(ciphertext.len() <= plaintext.len());

        match &self.key {
            AEADKey::ChaChaPoly1305(key) => {
                decrypt_detached(key, plaintext, ciphertext, tag, aad, &self.nonce)
                    .map_err(|_| AEADError::CryptoError)?;
            }
            AEADKey::AesGcm128(key) => {
                libcrux_aes::AesGcm128::decrypt(
                    plaintext,
                    key,
                    &self.nonce,
                    aad,
                    ciphertext,
                    tag,
                )
                .unwrap();
                // .map_err(|_| AEADError::CryptoError)?;
            }
        }

        Ok(())
    }

    pub(crate) fn decrypt_deserialize<T: Deserialize>(
        &mut self,
        ciphertext: &[u8],
        tag: &[u8; 16],
        aad: &[u8],
    ) -> Result<T, AEADError> {
        let payload_serialized_buf = self.decrypt(ciphertext, tag, aad)?;

        T::tls_deserialize_exact(&payload_serialized_buf).map_err(AEADError::Deserialize)
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

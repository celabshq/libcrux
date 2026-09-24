use alloc::{format, vec::Vec};

use aes_gcm::{Aes128Gcm as RC_Aes128Gcm, Aes256Gcm as RC_Aes256Gcm};
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    ChaCha20Poly1305 as RC_ChaCha20Poly1305,
};
use hpke_rs_crypto::{error::Error, types::AeadAlgorithm, HpkeCrypto};

use super::HpkeRustCrypto;

macro_rules! implement_aead {
    ($name_seal: ident, $name_open: ident, $name:ident, $algorithm:ident) => {
        pub(crate) fn $name_seal(
            key: &[u8],
            nonce: &[u8],
            aad: &[u8],
            msg: &[u8],
        ) -> Result<Vec<u8>, Error> {
            let nonce = nonce.try_into().map_err(|_| Error::AeadInvalidNonce)?;
            let cipher = $algorithm::new_from_slice(key)
                .map_err(|e| Error::CryptoLibraryError(format!("AEAD error: {:?}", e)))?;
            cipher
                .encrypt(nonce, Payload { msg, aad })
                .map_err(|e| Error::CryptoLibraryError(format!("AEAD error: {:?}", e)))
        }
        pub(crate) fn $name_open(
            alg: AeadAlgorithm,
            key: &[u8],
            nonce: &[u8],
            aad: &[u8],
            msg: &[u8],
        ) -> Result<Vec<u8>, Error> {
            let nonce = nonce.try_into().map_err(|_| Error::AeadInvalidNonce)?;
            let tag_length = HpkeRustCrypto::aead_tag_length(alg);
            if msg.len() < tag_length {
                return Err(Error::AeadInvalidCiphertext);
            }

            let cipher = $algorithm::new_from_slice(key)
                .map_err(|e| Error::CryptoLibraryError(format!("AEAD error: {:?}", e)))?;

            cipher
                .decrypt(nonce, Payload { msg, aad })
                .map_err(|_| Error::AeadOpenError)
        }
    };
}

implement_aead!(aes128_seal, aes128_open, AesGcm128, RC_Aes128Gcm);
implement_aead!(aes256_seal, aes256_open, AesGcm256, RC_Aes256Gcm);
implement_aead!(
    chacha_seal,
    chacha_open,
    ChaCha20Poly1305,
    RC_ChaCha20Poly1305
);

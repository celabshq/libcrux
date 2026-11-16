//! This module contains the trait and related errors for an Authenticated
//! Encryption with Associated Data (AEAD) scheme that takes array references
//! as arguments and writes outputs to mutable array references.

use libcrux_secrets::U8;

/// An Authenticated Encryption with Associated Data (AEAD) scheme. This trait
/// is low-level and is mostly used for implementing other, more usable APIs.
///
/// Some implementors of this trait may impose stronger restrictions on the inputs than described
/// here. Check the documentation of the types implementing this trait to make sure which inputs
/// are valid.
pub trait Aead<const KEY_LEN: usize, const TAG_LEN: usize, const NONCE_LEN: usize> {
    /// Generate a new key. Consumes the entire randomnes.
    fn keygen(key: &mut [U8; KEY_LEN], rand: &[U8; KEY_LEN]) -> Result<(), KeyGenError>;

    /// Encrypt a plaintext message, producing a ciphertext and an authentication tag.
    /// The arguments `plaintext` and `ciphertext` must have the same length.
    fn encrypt(
        ciphertext: &mut [u8],
        tag: &mut [U8; TAG_LEN],
        key: &[U8; KEY_LEN],
        nonce: &[U8; NONCE_LEN],
        aad: &[u8],
        plaintext: &[U8],
    ) -> Result<(), EncryptError>;

    /// Decrypt a ciphertext, verifying its authenticity.
    /// The arguments `plaintext` and `ciphertext` must have the same length.
    fn decrypt(
        plaintext: &mut [U8],
        key: &[U8; KEY_LEN],
        nonce: &[U8; NONCE_LEN],
        aad: &[u8],
        ciphertext: &[u8],
        tag: &[U8; TAG_LEN],
    ) -> Result<(), DecryptError>;
}

/// An error occurred during key generation
pub struct KeyGenError;

/// Error that can occur during encryption.
#[derive(Debug, PartialEq, Eq)]
pub enum EncryptError {
    /// The ciphertext buffer has the wrong length.
    WrongCiphertextLength,

    /// The plaintext is too long for this algorithm or implementation.
    PlaintextTooLong,

    /// The AAD is too long for this algorithm or implementation.
    AadTooLong,

    /// An unknown error occurred during encryption.
    Unknown,
}

/// Error that can occur during decryption.
#[derive(Debug, PartialEq, Eq)]
pub enum DecryptError {
    /// The authentication tag is invalid; the ciphertext has been tampered with
    /// or the key/nonce/aad is incorrect.
    InvalidTag,

    /// The plaintext buffer has the wrong length.
    WrongPlaintextLength,

    /// The plaintext is too long for this algorithm or implementation.
    PlaintextTooLong,

    /// The AAD is too long for this algorithm or implementation.
    AadTooLong,

    /// An unknown error occurred during decryption.
    Unknown,
}

impl core::fmt::Display for EncryptError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            EncryptError::WrongCiphertextLength => "ciphertext buffer has wrong length",
            EncryptError::PlaintextTooLong => {
                "plaintext is too long for algorithm or implementation"
            }
            EncryptError::AadTooLong => "aad is too long for algorithm or implementation",
            EncryptError::Unknown => "an unknown error occurred",
        };

        f.write_str(text)
    }
}

impl core::fmt::Display for DecryptError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            DecryptError::InvalidTag => "invalid authentication tag",
            DecryptError::WrongPlaintextLength => "plaintext buffer has wrong length",
            DecryptError::PlaintextTooLong => {
                "plaintext is too long for algorithm or implementation"
            }
            DecryptError::AadTooLong => "aad is too long for algorithm or implementation",
            DecryptError::Unknown => "an unknown error occurred",
        };

        f.write_str(text)
    }
}

#[cfg(feature = "error-in-core")]
mod error_in_core {
    impl core::error::Error for super::EncryptError {}
    impl core::error::Error for super::DecryptError {}
}

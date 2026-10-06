//! # HPKE Crypto Trait Errors
//!
//! Errors thrown by crypto functions implementing the [`crate::HpkeCrypto`] traits.

use alloc::string::String;
use core::fmt::Display;

/// Errors thrown by [`crate::HpkeCrypto`] trait implementations.
#[derive(Debug)]
pub enum Error {
    /// The output length is invalid (too large).
    HpkeInvalidOutputLength,

    /// Unknown or unsupported KDF algorithm.
    UnknownKdfAlgorithm,

    /// Invalid secret key for the KEM.
    KemInvalidSecretKey,

    /// Invalid public key for the KEM.
    KemInvalidPublicKey,

    /// Invalid ciphertext for the KEM.
    KemInvalidCiphertext,

    /// Unknown or unsupported KEM algorithm,
    UnknownKemAlgorithm,

    /// Unsupported operation for KEM algorithm.
    UnsupportedKemOperation,

    /// Unknown or unsupported AEAD algorithm.
    UnknownAeadAlgorithm,

    /// Invalid nonce for the AEAD algorithm.
    AeadInvalidNonce,

    /// Error opening an AEAD cipher text.
    AeadOpenError,

    /// Invalid cipher text for the AEAD algorithm.
    AeadInvalidCiphertext,

    /// Insufficient randomness to perform the operation.
    InsufficientRandomness,

    /// A crypto library error.
    CryptoLibraryError(String),
}

// `rand_core::TryRng::Error` which requires this.
impl core::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "HPKE Crypto Error: {:?}", self)
    }
}

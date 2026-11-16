//! This module contains the trait and related errors for a KEM that takes array references as
//! arguments and writes to outputs to mutable array references.

use libcrux_secrets::U8;

/// A Key Encapsulation Mechanism (KEM). This trait is the most low-level and mostly used in the
/// implementation of other, more usabe APIs on top.
pub trait Kem<
    const EK_LEN: usize,
    const DK_LEN: usize,
    const CT_LEN: usize,
    const SS_LEN: usize,
    const RAND_KEYGEN_LEN: usize,
    const RAND_ENCAPS_LEN: usize,
>
{
    /// Generate a pair of encapsulation and decapsulation keys.
    /// It is the responsibility of the caller to ensure  that the `rand` argument is actually
    /// random.
    fn keygen(
        ek: &mut [u8; EK_LEN],
        dk: &mut [U8; DK_LEN],
        rand: &[U8; RAND_KEYGEN_LEN],
    ) -> Result<(), KeyGenError>;

    /// Encapsulate a shared secret towards a given encapsulation key.
    /// It is the responsibility of the caller to ensure  that the `rand` argument is actually
    /// random.
    fn encaps(
        ct: &mut [u8; CT_LEN],
        ss: &mut [U8; SS_LEN],
        ek: &[u8; EK_LEN],
        rand: &[U8; RAND_ENCAPS_LEN],
    ) -> Result<(), EncapsError>;

    /// Decapsulate a shared secret.
    fn decaps(
        ss: &mut [U8; SS_LEN],
        ct: &[u8; CT_LEN],
        dk: &[U8; DK_LEN],
    ) -> Result<(), DecapsError>;
}

/// Error generating key with provided randomness
#[derive(Debug)]
pub enum KeyGenError {
    /// Error generating key with provided randomness
    InvalidRandomness,

    /// An unknown error occurred
    Unknown,
}

/// Error indicating that encapsulating failed
#[derive(Debug)]
pub enum EncapsError {
    /// Encapsulation key is invalid
    InvalidEncapsKey,

    /// Error encapsulating key with provided randomness
    InvalidRandomness,

    /// An unknown error occurred
    Unknown,
}

/// Error indicating that decapsulating failed
#[derive(Debug)]
pub enum DecapsError {
    /// Ciphertext key is invalid
    InvalidCiphertext,

    /// Decapsulation key is invalid
    InvalidDecapsKey,

    /// An unknown error occurred
    Unknown,
}

impl core::fmt::Display for KeyGenError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            KeyGenError::InvalidRandomness => "error generating key with provided randomness",
            KeyGenError::Unknown => "an unknown error occurred",
        };

        f.write_str(text)
    }
}

impl core::fmt::Display for EncapsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            EncapsError::InvalidEncapsKey => "encapsulation key is invalid",
            EncapsError::InvalidRandomness => "error generating key with provided randomness",
            EncapsError::Unknown => "an unknown error occurred",
        };

        f.write_str(text)
    }
}

impl core::fmt::Display for DecapsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            DecapsError::InvalidDecapsKey => "encapsulation key is invalid",
            DecapsError::InvalidCiphertext => "ciphertext is invalid",
            DecapsError::Unknown => "an unknown error occurred",
        };

        f.write_str(text)
    }
}

#[cfg(feature = "error-in-core")]
/// Here we implement the Error trait. This has only been added to core relatively recently, so we
/// are hiding that behind a feature.
mod error_in_core {
    impl core::error::Error for super::EncapsError {}
    impl core::error::Error for super::DecapsError {}
    impl core::error::Error for super::KeyGenError {}
}

//! This module contains the trait and related errors for an Authenticated
//! Encryption with Associated Data (AEAD) scheme that takes slices
//! as arguments and writes outputs to mutable slices.

use libcrux_secrets::U8;

/// An Authenticated Encryption with Associated Data (AEAD) scheme.
///
/// Some implementors of this trait may impose stronger restrictions on the inputs than described
/// here. Check the documentation of the types implementing this trait to make sure which inputs
/// are valid.
pub trait Aead {
    /// Generate a new key. Consumes the entire randomnes. It is the responsibility of the caller
    /// to ensure `rand` is random and long enough (usually as long as `key`).
    fn keygen(key: &mut [U8], rand: &[U8]) -> Result<(), KeyGenError>;

    /// Encrypt a plaintext message, producing a ciphertext and an authentication tag.
    /// The arguments `plaintext` and `ciphertext` must have the same length.
    fn encrypt(
        ciphertext: &mut [u8],
        tag: &mut [U8],
        key: &[U8],
        nonce: &[U8],
        aad: &[u8],
        plaintext: &[U8],
    ) -> Result<(), EncryptError>;

    /// Decrypt a ciphertext, verifying its authenticity.
    /// The arguments `plaintext` and `ciphertext` must have the same length.
    fn decrypt(
        plaintext: &mut [U8],
        key: &[U8],
        nonce: &[U8],
        aad: &[u8],
        ciphertext: &[u8],
        tag: &[U8],
    ) -> Result<(), DecryptError>;
}

/// Error that can orrur during key generation
pub enum KeyGenError {
    /// The provided key has the wrong length
    WrongKeyLength,

    /// The provided randomness is too short
    InsufficientRandomness,

    /// The provided randomness is not suitable. Usually this is resolved by trying again with
    /// different randomness (which would do rejection sampling).
    UnsuitableRandomness,
}

/// Error that can occur during encryption.
#[derive(Debug, PartialEq, Eq)]
pub enum EncryptError {
    /// The ciphertext buffer has the wrong length.
    WrongCiphertextLength,

    /// The plaintext is too long for this algorithm or implementation.
    PlaintextTooLong,

    /// The AAD is too long for this algorithm or implementation.
    AadTooLong,

    /// The key has the wrong length.
    WrongKeyLength,

    /// The tag has the wrong length.
    WrongTagLength,

    /// The nonce has the wrong length.
    WrongNonceLength,

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

    /// The key has the wrong length.
    WrongKeyLength,

    /// The tag has the wrong length.
    WrongTagLength,

    /// The nonce has the wrong length.
    WrongNonceLength,

    /// An unknown error occurred during decryption.
    Unknown,
}

impl From<super::arrayref::EncryptError> for EncryptError {
    fn from(value: super::arrayref::EncryptError) -> Self {
        match value {
            super::arrayref::EncryptError::WrongCiphertextLength => {
                EncryptError::WrongCiphertextLength
            }
            super::arrayref::EncryptError::PlaintextTooLong => EncryptError::PlaintextTooLong,
            super::arrayref::EncryptError::AadTooLong => EncryptError::AadTooLong,
            super::arrayref::EncryptError::Unknown => EncryptError::Unknown,
        }
    }
}

impl From<super::arrayref::DecryptError> for DecryptError {
    fn from(value: super::arrayref::DecryptError) -> Self {
        match value {
            super::arrayref::DecryptError::InvalidTag => DecryptError::InvalidTag,
            super::arrayref::DecryptError::WrongPlaintextLength => {
                DecryptError::WrongPlaintextLength
            }
            super::arrayref::DecryptError::PlaintextTooLong => DecryptError::PlaintextTooLong,
            super::arrayref::DecryptError::AadTooLong => DecryptError::AadTooLong,
            super::arrayref::DecryptError::Unknown => DecryptError::Unknown,
        }
    }
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
            EncryptError::WrongKeyLength => "key has wrong length",
            EncryptError::WrongTagLength => "tag has wrong length",
            EncryptError::WrongNonceLength => "nonce has wrong length",
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
            DecryptError::WrongKeyLength => "key has wrong length",
            DecryptError::WrongTagLength => "tag has wrong length",
            DecryptError::WrongNonceLength => "nonce has wrong length",
        };

        f.write_str(text)
    }
}

#[cfg(feature = "error-in-core")]
/// Here we implement the Error trait. This has only been added to core relatively recently, so we
/// are hiding that behind a feature.
mod error_in_core {
    impl core::error::Error for super::EncryptError {}
    impl core::error::Error for super::DecryptError {}
}

/// Implements [`slice::Aead`] based on [`arrayref::Aead`].
///
/// [`slice::Aead`]: Aead
/// [`arrayref::Aead`]: super::arrayref::Aead
#[macro_export]
macro_rules! impl_aead_slice_trait {
    ($type:ty => $key:expr, $tag:expr, $nonce:expr) => {
        impl $crate::aead::slice::Aead for $type {
            fn keygen(
                key: &mut [$crate::libcrux_secrets::U8],
                rand: &[$crate::libcrux_secrets::U8],
            ) -> Result<(), $crate::aead::slice::KeyGenError> {
                let key: &mut [$crate::libcrux_secrets::U8; $key] = key
                    .try_into()
                    .map_err(|_| $crate::aead::slice::KeyGenError::WrongKeyLength)?;
                if rand.len() < $key {
                    return Err($crate::aead::slice::KeyGenError::InsufficientRandomness);
                }

                Ok(key.copy_from_slice(rand))
            }

            fn encrypt(
                ciphertext: &mut [u8],
                tag: &mut [$crate::libcrux_secrets::U8],
                key: &[$crate::libcrux_secrets::U8],
                nonce: &[$crate::libcrux_secrets::U8],
                aad: &[u8],
                plaintext: &[$crate::libcrux_secrets::U8],
            ) -> Result<(), $crate::aead::slice::EncryptError> {
                let key: &[$crate::libcrux_secrets::U8; $key] = key
                    .try_into()
                    .map_err(|_| $crate::aead::slice::EncryptError::WrongKeyLength)?;
                let tag: &mut [$crate::libcrux_secrets::U8; $tag] = tag
                    .try_into()
                    .map_err(|_| $crate::aead::slice::EncryptError::WrongTagLength)?;
                let nonce: &[$crate::libcrux_secrets::U8; $nonce] = nonce
                    .try_into()
                    .map_err(|_| $crate::aead::slice::EncryptError::WrongNonceLength)?;

                <Self as $crate::aead::arrayref::Aead<$key, $tag, $nonce>>::encrypt(
                    ciphertext, tag, key, nonce, aad, plaintext,
                )
                .map_err($crate::aead::slice::EncryptError::from)
            }

            fn decrypt(
                plaintext: &mut [$crate::libcrux_secrets::U8],
                key: &[$crate::libcrux_secrets::U8],
                nonce: &[$crate::libcrux_secrets::U8],
                aad: &[u8],
                ciphertext: &[u8],
                tag: &[$crate::libcrux_secrets::U8],
            ) -> Result<(), $crate::aead::slice::DecryptError> {
                let key: &[$crate::libcrux_secrets::U8; $key] = key
                    .try_into()
                    .map_err(|_| $crate::aead::slice::DecryptError::WrongKeyLength)?;
                let tag: &[$crate::libcrux_secrets::U8; $tag] = tag
                    .try_into()
                    .map_err(|_| $crate::aead::slice::DecryptError::WrongTagLength)?;
                let nonce: &[$crate::libcrux_secrets::U8; $nonce] = nonce
                    .try_into()
                    .map_err(|_| $crate::aead::slice::DecryptError::WrongNonceLength)?;

                <Self as $crate::aead::arrayref::Aead<$key, $tag, $nonce>>::decrypt(
                    plaintext, key, nonce, aad, ciphertext, tag,
                )
                .map_err($crate::aead::slice::DecryptError::from)
            }
        }

        #[cfg(test)]
        #[test]
        fn simple_aead_test() {
            $crate::aead::tests::simple::<$key, $tag, $nonce, $type>();
        }
    };
}

pub use impl_aead_slice_trait;

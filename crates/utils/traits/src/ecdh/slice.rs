//! This module contains the trait and related errors for an ECDH
//! implementation that takes slices as arguments and writes outputs
//! to mutable slices.

use super::arrayref;
use libcrux_secrets::U8;

pub trait EcdhSlice {
    /// Generate a Diffie-Hellman secret value.
    /// It is the responsibility of the caller to ensure  that the `rand` argument is actually
    /// random.
    fn generate_secret(secret: &mut [U8], rand: &[U8]) -> Result<(), GenerateSecretError>;

    /// Derive a Diffie-Hellman public value from a secret value.
    fn secret_to_public(public: &mut [u8], secret: &[U8]) -> Result<(), SecretToPublicError>;

    /// Generate a Diffie-Hellman secret value and derive the
    /// corresponding public value in one step.
    fn generate_pair(
        public: &mut [u8],
        secret: &mut [U8],
        rand: &[U8],
    ) -> Result<(), GenerateSecretError> {
        Self::generate_secret(secret, rand)?;
        Self::secret_to_public(public, secret).map_err(|_| GenerateSecretError::Unknown)
    }

    /// Derive a Diffie-Hellman shared secret from a public and a
    /// secret value.
    ///
    /// This value is NOT (!) safe for use as a key and needs to be processed in a round of key
    /// derivation, to ensure both that the output is uniformly random and that unkown key share
    /// attacks can not happen.
    fn derive_ecdh(derived: &mut [U8], public: &[u8], secret: &[U8]) -> Result<(), DeriveError>;

    /// Check the validity of a Diffie-Hellman secret value.
    fn validate_secret(secret: &[U8]) -> Result<(), ValidateSecretError>;
}

/// Implements [`EcdhSlice`] for any `$ty : arrayref::EcdhArrayref`
/// with the given array bounds.
#[macro_export]
macro_rules! impl_ecdh_slice_trait {
    ($type:ty => $rand_len:expr, $sk_len:expr, $pk_len:expr) => {
        impl $crate::ecdh::slice::EcdhSlice for $type {
            fn generate_secret(secret: &mut [U8], rand: &[U8]) -> Result<(), $crate::ecdh::slice::GenerateSecretError> {
                let secret: &mut [U8; $sk_len] = secret
                .try_into()
                .map_err(|_|
                    $crate::ecdh::slice::GenerateSecretError::InvalidSecretLength)?;

                let rand: &[U8; $rand_len] = rand
                .try_into()
                .map_err(|_|
                    $crate::ecdh::slice::GenerateSecretError::InvalidRandomnessLength)?;

                <$type as $crate::ecdh::arrayref::EcdhArrayref<$rand_len, $sk_len, $pk_len>>::generate_secret(secret, rand)
                .map_err($crate::ecdh::slice::GenerateSecretError::from)
            }

            fn secret_to_public(public: &mut [u8], secret: &[U8]) -> Result<(), $crate::ecdh::slice::SecretToPublicError> {
                let secret: &[U8; $sk_len] = secret
                .try_into()
                .map_err(|_|
                    $crate::ecdh::slice::SecretToPublicError::InvalidSecretLength)?;

                let public: &mut [u8; $pk_len] = public
                .try_into()
                .map_err(|_|
                    $crate::ecdh::slice::SecretToPublicError::InvalidPublicLength)?;

                <$type as $crate::ecdh::arrayref::EcdhArrayref<$rand_len, $sk_len, $pk_len>>::secret_to_public(public, secret)
                .map_err($crate::ecdh::slice::SecretToPublicError::from)
            }

            fn derive_ecdh(derived: &mut [U8], public: &[u8], secret: &[U8]) -> Result<(), $crate::ecdh::slice::DeriveError> {
                let derived: &mut [U8; $pk_len] = derived
                .try_into()
                .map_err(|_|
                    $crate::ecdh::slice::DeriveError::InvalidDeriveLength)?;

                let secret: &[U8; $sk_len] = secret
                .try_into()
                .map_err(|_|
                    $crate::ecdh::slice::DeriveError::InvalidSecretLength)?;

                let public: &[u8; $pk_len] = public
                .try_into()
                .map_err(|_|
                    $crate::ecdh::slice::DeriveError::InvalidPublicLength)?;

                <$type as $crate::ecdh::arrayref::EcdhArrayref<$rand_len, $sk_len, $pk_len>>::derive_ecdh(derived, public, secret)
                .map_err($crate::ecdh::slice::DeriveError::from)
            }

            fn validate_secret(secret: &[U8]) -> Result<(), $crate::ecdh::slice::ValidateSecretError> {
                let secret: &[U8; $sk_len] = secret
                .try_into()
                .map_err(|_|
                    $crate::ecdh::slice::ValidateSecretError::InvalidSecretLength)?;

                <$type as $crate::ecdh::arrayref::EcdhArrayref<$rand_len, $sk_len, $pk_len>>::validate_secret(secret)
                .map_err($crate::ecdh::slice::ValidateSecretError::from)
            }
        }
    };
}

pub use impl_ecdh_slice_trait;

#[derive(Debug)]
/// An error during secret value generation.
pub enum GenerateSecretError {
    /// Error generating secret value with provided randomness
    InvalidRandomness,
    /// The provided secret value buffer has the wrong length
    InvalidSecretLength,
    /// The provided randomness has the wrong length
    InvalidRandomnessLength,
    /// An unknown error occurred
    Unknown,
}

#[derive(Debug)]
/// An error during derivation of a public value from a secret value.
pub enum SecretToPublicError {
    /// Secret value was invalid
    InvalidSecret,
    /// The provided secret value buffer has the wrong length
    InvalidSecretLength,
    /// The provided public value buffer has the wrong length
    InvalidPublicLength,
    /// An unknown error occurred
    Unknown,
}

#[derive(Debug)]
/// An error derivation of Diffie-Hellman shared secret.
pub enum DeriveError {
    /// Public value was invalid
    InvalidPublic,
    /// Secret value was invalid
    InvalidSecret,
    /// The provided secret value buffer has the wrong length
    InvalidSecretLength,
    /// The provided public value buffer has the wrong length
    InvalidPublicLength,
    /// The provided shared secret buffer has the wrong length
    InvalidDeriveLength,
    /// An unknown error occurred
    Unknown,
}

#[derive(Debug)]
/// A Diffie-Hellman secret value was found to be invalid.
pub enum ValidateSecretError {
    /// Secret value was invalid
    InvalidSecret,
    /// The provided secret value buffer has the wrong length
    InvalidSecretLength,
    /// An unknown error occurred
    Unknown,
}

impl From<arrayref::GenerateSecretError> for GenerateSecretError {
    fn from(value: arrayref::GenerateSecretError) -> Self {
        match value {
            arrayref::GenerateSecretError::InvalidRandomness => {
                GenerateSecretError::InvalidRandomness
            }
            arrayref::GenerateSecretError::Unknown => GenerateSecretError::Unknown,
        }
    }
}
impl From<arrayref::SecretToPublicError> for SecretToPublicError {
    fn from(value: arrayref::SecretToPublicError) -> Self {
        match value {
            arrayref::SecretToPublicError::InvalidSecret => SecretToPublicError::InvalidSecret,
            arrayref::SecretToPublicError::Unknown => SecretToPublicError::Unknown,
        }
    }
}
impl From<arrayref::DeriveError> for DeriveError {
    fn from(value: arrayref::DeriveError) -> Self {
        match value {
            arrayref::DeriveError::InvalidPublic => DeriveError::InvalidPublic,
            arrayref::DeriveError::InvalidSecret => DeriveError::InvalidSecret,
            arrayref::DeriveError::Unknown => DeriveError::Unknown,
        }
    }
}
impl From<arrayref::ValidateSecretError> for ValidateSecretError {
    fn from(value: arrayref::ValidateSecretError) -> Self {
        match value {
            arrayref::ValidateSecretError::InvalidSecret => ValidateSecretError::InvalidSecret,
            arrayref::ValidateSecretError::Unknown => ValidateSecretError::Unknown,
        }
    }
}

impl core::fmt::Display for GenerateSecretError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            GenerateSecretError::InvalidRandomness => {
                "error generating secret value with provided randomness"
            }
            GenerateSecretError::Unknown => "an unknown error occured",
            GenerateSecretError::InvalidRandomnessLength => {
                "the provided randomess buffer has the wrong length"
            }
            GenerateSecretError::InvalidSecretLength => {
                "the provided secret value buffer has the wrong length"
            }
        };

        f.write_str(text)
    }
}

impl core::fmt::Display for SecretToPublicError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            SecretToPublicError::InvalidSecret => "secret value is invalid",
            SecretToPublicError::Unknown => "an unknown error occured",
            SecretToPublicError::InvalidSecretLength => {
                "the provided secret value buffer has the wrong length"
            }
            SecretToPublicError::InvalidPublicLength => {
                "the provided public value buffer has the wrong length"
            }
        };

        f.write_str(text)
    }
}

impl core::fmt::Display for DeriveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            DeriveError::InvalidPublic => "public value is invalid",
            DeriveError::InvalidSecret => "secret value is invalid",
            DeriveError::Unknown => "an unknown error occured",
            DeriveError::InvalidSecretLength => {
                "the provided secret value buffer has the wrong length"
            }
            DeriveError::InvalidPublicLength => {
                "the provided public value buffer has the wrong length"
            }
            DeriveError::InvalidDeriveLength => {
                "the provided shared secret buffer has the wrong length"
            }
        };

        f.write_str(text)
    }
}

impl core::fmt::Display for ValidateSecretError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            ValidateSecretError::InvalidSecret => "secret value is invalid",
            ValidateSecretError::Unknown => "an unknown error occured",
            ValidateSecretError::InvalidSecretLength => {
                "the provided secret value buffer has the wrong length"
            }
        };

        f.write_str(text)
    }
}

#[cfg(feature = "error-in-core")]
/// Here we implement the Error trait. This has only been added to core relatively recently, so we
/// are hiding that behind a feature.
mod error_in_core {
    impl core::error::Error for super::GenerateSecretError {}
    impl core::error::Error for super::SecretToPublicError {}
    impl core::error::Error for super::DeriveError {}
    impl core::error::Error for super::ValidateSecretError {}
}

//! This module contains the trait and related errors for an ECDH
//! implementation that takes array references as arguments and writes
//! outputs to mutable array references.

use libcrux_secrets::U8;

/// An Elliptic Curve Diffie-Hellman (ECDH) key exchange. This trait is the most low-level and mostly used in the
/// implementation of other, more usabe APIs on top.
pub trait EcdhArrayref<const RAND_LEN: usize, const SECRET_LEN: usize, const PUBLIC_LEN: usize> {
    /// Generate a Diffie-Hellman secret value.
    /// It is the responsibility of the caller to ensure  that the `rand` argument is actually
    /// random.
    fn generate_secret(
        secret: &mut [U8; SECRET_LEN],
        rand: &[U8; RAND_LEN],
    ) -> Result<(), GenerateSecretError>;

    /// Derive a Diffie-Hellman public value from a secret value.
    fn secret_to_public(
        public: &mut [u8; PUBLIC_LEN],
        secret: &[U8; SECRET_LEN],
    ) -> Result<(), SecretToPublicError>;

    /// Generate a Diffie-Hellman secret value and derive the
    /// corresponding public value in one step.
    fn generate_pair(
        public: &mut [u8; PUBLIC_LEN],
        secret: &mut [U8; SECRET_LEN],
        rand: &[U8; RAND_LEN],
    ) -> Result<(), GenerateSecretError> {
        Self::generate_secret(secret, rand)?;
        Self::secret_to_public(public, secret).map_err(|_| GenerateSecretError::Unknown)
    }

    /// Derive a Diffie-Hellman shared secret from a public and a
    /// secret value.
    ///
    /// This value is NOT (!) safe for use as a key and needs to be processed in a round of key
    /// derivation, to ensure both that the output is uniformly random and that unknown key share
    /// attacks can not happen.
    fn derive_ecdh(
        derived: &mut [U8; PUBLIC_LEN],
        public: &[u8; PUBLIC_LEN],
        secret: &[U8; SECRET_LEN],
    ) -> Result<(), DeriveError>;

    /// Check the validity of a Diffie-Hellman secret value.
    fn validate_secret(secret: &[U8; SECRET_LEN]) -> Result<(), ValidateSecretError>;
}

#[derive(Debug)]
/// An error during secret value generation.
pub enum GenerateSecretError {
    /// Error generating secret value with provided randomness
    InvalidRandomness,

    /// An unknown error occurred
    Unknown,
}

#[derive(Debug)]
/// An error during derivation of a public value from a secret value.
pub enum SecretToPublicError {
    /// Secret value was invalid
    InvalidSecret,
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
    /// An unknown error occurred
    Unknown,
}

#[derive(Debug)]
/// A Diffie-Hellman secret value was found to be invalid.
pub enum ValidateSecretError {
    /// Secret value was invalid
    InvalidSecret,
    /// An unknown error occurred
    Unknown,
}

impl core::fmt::Display for GenerateSecretError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            GenerateSecretError::InvalidRandomness => {
                "error generating secret value with provided randomness"
            }
            GenerateSecretError::Unknown => "an unknown error occured",
        };

        f.write_str(text)
    }
}

impl core::fmt::Display for SecretToPublicError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            SecretToPublicError::InvalidSecret => "secret value is invalid",
            SecretToPublicError::Unknown => "an unknown error occured",
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
        };

        f.write_str(text)
    }
}

impl core::fmt::Display for ValidateSecretError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            ValidateSecretError::InvalidSecret => "secret value is invalid",
            ValidateSecretError::Unknown => "an unknown error occured",
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

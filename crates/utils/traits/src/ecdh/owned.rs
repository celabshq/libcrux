//! This module contains the trait and related errors for an ECDH
//! implementation that takes array references as arguments and
//! returns owned arrays.

use super::arrayref;
use super::arrayref::{DeriveError, GenerateSecretError, SecretToPublicError, ValidateSecretError};

use libcrux_secrets::{Classify, U8};

/// An Elliptic Curve Diffie-Hellman (ECDH) key exchange.
pub trait EcdhOwned<const RAND_LEN: usize, const SECRET_LEN: usize, const PUBLIC_LEN: usize> {
    /// Generate a Diffie-Hellman secret value.
    /// It is the responsibility of the caller to ensure  that the `rand` argument is actually
    /// random.
    fn generate_secret(rand: &[U8; RAND_LEN]) -> Result<[U8; SECRET_LEN], GenerateSecretError>;

    /// Derive a Diffie-Hellman public value from a secret value.
    fn secret_to_public(secret: &[U8; SECRET_LEN])
        -> Result<[u8; PUBLIC_LEN], SecretToPublicError>;

    /// Generate a Diffie-Hellman secret value and derive the
    /// corresponding public value in one step.
    fn generate_pair(
        rand: &[U8; RAND_LEN],
    ) -> Result<([u8; PUBLIC_LEN], [U8; SECRET_LEN]), GenerateSecretError> {
        let secret = Self::generate_secret(rand)?;
        let public = Self::secret_to_public(&secret).map_err(|_| GenerateSecretError::Unknown)?;
        Ok((public, secret))
    }

    /// Derive a Diffie-Hellman shared secret from a public and a
    /// secret value.
    ///
    /// This value is NOT (!) safe for use as a key and needs to be processed in a round of key
    /// derivation, to ensure both that the output is uniformly random and that unkown key share
    /// attacks can not happen.
    fn derive_ecdh(
        public: &[u8; PUBLIC_LEN],
        secret: &[U8; SECRET_LEN],
    ) -> Result<[U8; PUBLIC_LEN], DeriveError>;

    /// Check the validity of a Diffie-Hellman secret value.
    fn validate_secret(secret: &[U8; SECRET_LEN]) -> Result<(), ValidateSecretError>;
}

impl<
        const RAND_LEN: usize,
        const SECRET_LEN: usize,
        const PUBLIC_LEN: usize,
        T: arrayref::EcdhArrayref<RAND_LEN, SECRET_LEN, PUBLIC_LEN>,
    > EcdhOwned<RAND_LEN, SECRET_LEN, PUBLIC_LEN> for T
{
    fn generate_secret(rand: &[U8; RAND_LEN]) -> Result<[U8; SECRET_LEN], GenerateSecretError> {
        let mut secret = [0u8; SECRET_LEN].classify();
        <Self as arrayref::EcdhArrayref<RAND_LEN, SECRET_LEN, PUBLIC_LEN>>::generate_secret(
            &mut secret,
            rand,
        )?;
        Ok(secret)
    }

    fn secret_to_public(
        secret: &[U8; SECRET_LEN],
    ) -> Result<[u8; PUBLIC_LEN], SecretToPublicError> {
        let mut public = [0u8; PUBLIC_LEN];
        <Self as arrayref::EcdhArrayref<RAND_LEN, SECRET_LEN, PUBLIC_LEN>>::secret_to_public(
            &mut public,
            secret,
        )?;
        Ok(public)
    }

    fn derive_ecdh(
        public: &[u8; PUBLIC_LEN],
        secret: &[U8; SECRET_LEN],
    ) -> Result<[U8; PUBLIC_LEN], DeriveError> {
        let mut derived = [0u8; PUBLIC_LEN].classify();
        <Self as arrayref::EcdhArrayref<RAND_LEN, SECRET_LEN, PUBLIC_LEN>>::derive_ecdh(
            &mut derived,
            public,
            secret,
        )?;
        Ok(derived)
    }

    fn validate_secret(secret: &[U8; SECRET_LEN]) -> Result<(), ValidateSecretError> {
        <Self as arrayref::EcdhArrayref<RAND_LEN, SECRET_LEN, PUBLIC_LEN>>::validate_secret(secret)
    }
}

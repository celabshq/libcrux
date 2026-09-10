//! # ECDH
//!
//! ## x25519
//! For x25519 the portable HACL implementation is used.
//!
//! ## P256
//! For P256 the portable HACL implementation is used.
#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};

mod hacl;

#[derive(Debug, PartialEq, Eq)]
pub enum LowLevelError {
    Jasmin(String),
    Hacl(hacl::Error),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidPoint,
    InvalidScalar,
    UnknownAlgorithm,
    KeyGenError,
    Custom(String),
    Wrap(LowLevelError),
}

impl From<hacl::p256::Error> for Error {
    fn from(value: hacl::p256::Error) -> Self {
        Error::Wrap(LowLevelError::Hacl(hacl::Error::P256(value)))
    }
}

/// ECDH algorithm.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Algorithm {
    X25519,
    X448,
    P256,
    P384,
    P521,
}

/// The internal x25519 module
pub(crate) mod x25519;
pub use x25519::{
    derive as x25519_derive, generate_secret as x25519_generate_secret, key_gen as x25519_key_gen,
    secret_to_public as x25519_secret_to_public, PrivateKey as X25519PrivateKey,
    PublicKey as X25519PublicKey, SharedSecret as X25519SharedSecret,
};

pub mod curve25519 {
    use super::hacl;
    pub use hacl::curve25519::Error;
}

/// The internal p256 module
pub(crate) mod p256_internal;

pub mod p256 {
    use super::hacl;
    pub use hacl::p256::{
        compressed_to_coordinates, uncompressed_to_coordinates, validate_point,
        validate_scalar_slice, Error,
    };
}

pub use p256_internal::{
    generate_secret as p256_generate_secret, key_gen as p256_key_gen,
    secret_to_public as p256_secret_to_public, validate_scalar as p256_validate_scalar,
    PrivateKey as P256PrivateKey, PublicKey as P256PublicKey, SharedSecret as P256SharedSecret,
};

/// Derive the ECDH shared secret.
/// Returns `Ok(point * scalar)` on the provided curve [`Algorithm`] or an error.
pub fn derive(
    alg: Algorithm,
    point: impl AsRef<[u8]>,
    scalar: impl AsRef<[u8]>,
) -> Result<Vec<u8>, Error> {
    match alg {
        Algorithm::X25519 => {
            x25519::derive(&point.as_ref().try_into()?, &scalar.as_ref().try_into()?)
                .map(|r| r.0.into())
        }
        Algorithm::P256 => {
            let point = p256_internal::prepare_public_key(point.as_ref())?;
            let scalar = hacl::p256::validate_scalar_slice(scalar.as_ref())
                .map_err(|_| Error::InvalidScalar)?;

            p256_internal::derive(&point, &scalar).map(|r| r.0.into())
        }
        _ => Err(Error::UnknownAlgorithm),
    }
}

pub fn p256_derive(
    point: &p256_internal::PublicKey,
    scalar: &p256_internal::PrivateKey,
) -> Result<p256_internal::SharedSecret, Error> {
    p256_internal::validate_point(point)?;
    p256_internal::validate_scalar(scalar)?;

    p256_internal::derive(point, scalar)
}

/// Derive the public key for the provided secret key `scalar`.
pub fn secret_to_public(alg: Algorithm, scalar: impl AsRef<[u8]>) -> Result<Vec<u8>, Error> {
    match alg {
        Algorithm::X25519 => {
            x25519::secret_to_public(&scalar.as_ref().try_into()?).map(|r| r.0.into())
        }
        Algorithm::P256 => {
            p256_internal::secret_to_public(&scalar.as_ref().try_into()?).map(|r| r.0.into())
        }
        _ => Err(Error::UnknownAlgorithm),
    }
}

/// Validate a secret key.
pub fn validate_scalar(alg: Algorithm, s: impl AsRef<[u8]>) -> Result<(), Error> {
    match alg {
        Algorithm::X25519 => {
            if s.as_ref().iter().all(|&b| b == 0)
                || libcrux_curve25519::is_clamped(
                    s.as_ref().try_into().map_err(|_| Error::InvalidScalar)?,
                )
                .is_err()
            {
                Err(Error::InvalidScalar)
            } else {
                Ok(())
            }
        }
        Algorithm::P256 => p256_internal::validate_scalar(&s.as_ref().try_into()?),
        _ => Err(Error::UnknownAlgorithm),
    }
}

use rand::TryCryptoRng;

/// Generate a new private key scalar.
///
/// The function returns the new scalar or an [`Error::KeyGenError`] if it was unable to
/// generate a new key. If this happens, the provided `rng` is probably faulty.
pub fn generate_secret(alg: Algorithm, rng: &mut impl TryCryptoRng) -> Result<Vec<u8>, Error> {
    match alg {
        Algorithm::X25519 => x25519::generate_secret(rng).map(|k| k.0.to_vec()),
        Algorithm::P256 => p256_internal::generate_secret(rng).map(|k| k.0.to_vec()),
        _ => Err(Error::UnknownAlgorithm),
    }
}

/// Generate a fresh key pair.
///
/// The function returns the (secret key, public key) tuple, or an [`Error`].
pub fn key_gen(alg: Algorithm, rng: &mut impl TryCryptoRng) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let sk = generate_secret(alg, rng)?;
    let pk = secret_to_public(alg, &sk)?;
    Ok((sk, pk))
}

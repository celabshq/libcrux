//! # Libcrux P-384
//!
//! This crate implements ECDH for NIST curve P-384. It is based on P-384
//! base field arithmetic from [Bas Spitter's fork of
//! fiat-crypto](https://github.com/spitters/fiat-crypto/) and
//! [AU-Curves](https://github.com/AU-COBRA/AUCurves).
//!
//! ## Examples
//! ```
//! use libcrux_p384::{PublicKey, PrivateKey, SharedSecret};
//!
//! # let pk_bytes = [0x04u8,0x79,0x0a,0x6e,0x05,0x9e,0xf9,0xa5,0x94,0x01,0x63,0x18,0x3d,0x4a,0x78,0x09,0x13,0x5d,0x29,0x79,0x16,0x43,0xfc,0x43,0xa2,0xf1,0x7e,0xe8,0xbf,0x67,0x7a,0xb8,0x4f,0x79,0x1b,0x64,0xa6,0xbe,0x15,0x96,0x9f,0xfa,0x01,0x2d,0xd9,0x18,0x5d,0x87,0x96,0xd9,0xb9,0x54,0xba,0xa8,0xa7,0x5e,0x82,0xdf,0x71,0x1b,0x3b,0x56,0xea,0xdf,0xf6,0xb0,0xf6,0x68,0xc3,0xb2,0x6b,0x4b,0x1a,0xeb,0x30,0x8a,0x1f,0xcc,0x1c,0x68,0x0d,0x32,0x9a,0x67,0x05,0x02,0x5f,0x1c,0x98,0xa0,0xb5,0xe5,0xbf,0xcb,0x16,0x3c,0xaa].as_slice();
//! # let sk_bytes = [0x76,0x6e,0x61,0x42,0x5b,0x2d,0xa9,0xf8,0x46,0xc0,0x9f,0xc3,0x56,0x4b,0x93,0xa6,0xf8,0x60,0x3b,0x73,0x92,0xc7,0x85,0x16,0x5b,0xf2,0x0d,0xa9,0x48,0xc4,0x9f,0xd1,0xfb,0x1d,0xee,0x4e,0xdd,0x64,0x35,0x6b,0x9f,0x21,0xc5,0x88,0xb7,0x5d,0xfd,0x81].as_slice();
//!
//! // Construct key types
//! let pk = PublicKey::try_from(pk_bytes).unwrap();
//! let sk = PrivateKey::try_from(sk_bytes).unwrap();
//! let shared_secret: SharedSecret = pk.ecdh(&sk);
//!
//! // or derive the exported shared secret directly
//! let shared_secret_direct = libcrux_p384::derive_ecdh(sk_bytes, pk_bytes).unwrap();
//!
//! assert_eq!(shared_secret_direct.as_ref(), shared_secret.as_ref());
//! ```
//! ## Cargo Features
//!
//! - `rand`: This feature provides access to a private key generation
//!   API, which samples internally from a `rand::TryCryptoRng`.
//!
//! ## Secret Independence
//!
//! The code in this crate aims to be secret-independent on a source code
//! level.
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

// Base Field Arithmetic for P-384
pub(crate) mod field;
pub(crate) mod field_ops;

mod constants;
mod curve;
mod ecdh;
mod util;

pub use ecdh::{private::PrivateKey, PublicKey, SharedSecret};

#[derive(Copy, Clone, Debug)]
/// Errors that can occur during P-384 operations.
pub enum EcdhError {
    /// Error when deserializing a private key
    InvalidPrivateKey,
    /// Error when deserializing a public key
    InvalidPublicKey,
}

#[cfg(feature = "rand")]
#[derive(Copy, Clone, Debug)]
/// Error due to insufficient randomness during key generation.
pub struct RandomnessError;

/// Internal error type for debugging
pub(crate) enum InternalError {
    FieldElement,
    Uncompressed,
    Compressed,
}

/// Attempt to derive an P-384 ECDH shared secret, given SEC1
/// encodings of private and public keys.
pub fn derive_ecdh(sk_bytes: &[u8], pk_bytes: &[u8]) -> Result<SharedSecret, EcdhError> {
    let sk = PrivateKey::try_from(sk_bytes)?;
    let pk = PublicKey::try_from(pk_bytes)?;

    Ok(pk.ecdh(&sk))
}

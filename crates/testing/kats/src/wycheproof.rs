//! Wycheproof Known Answer Tests

#[cfg(feature = "chacha20poly1305")]
pub mod aead;

#[cfg(feature = "ecdh")]
pub mod ecdh;

#[cfg(feature = "ecdsa")]
pub mod ecdsa;

#[cfg(feature = "mldsa")]
pub mod mldsa;

#[cfg(feature = "mlkem")]
pub mod mlkem;

#[cfg(feature = "hmac")]
pub mod hmac;

#[cfg(feature = "kmac")]
pub mod kmac;

pub mod schema_common;
pub use schema_common::TestResult;

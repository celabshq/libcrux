//! Known Answer Tests
//!
//! ⚠️ **NOTE**: This crate serves as an internal testing dependency for other `libcrux`
//! crates, and is not intended to be used directly. Use at your own risk.

pub mod wycheproof;

pub mod acvp;

#[cfg(feature = "sha2")]
pub mod sha2;

#[cfg(feature = "sha3")]
pub mod sha3;

#[cfg(feature = "poly1305")]
pub mod poly1305;

#[cfg(feature = "cavp")]
pub mod cavp;

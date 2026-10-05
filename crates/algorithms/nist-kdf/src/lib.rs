#![no_std]
#![doc = include_str!("../README.md")]
use core::fmt::{Debug, Display};

pub mod feedback;
pub mod two_step;

/// Opaque error returned by [`feedback::kdf`] and [`two_step::kdf`].
///
/// Please refer to the documentation of the individual `kdf` functions for their
/// error conditions.
#[derive(Debug)]
pub struct KdfError;

impl Display for KdfError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("NIST KDF Error")
    }
}

impl core::error::Error for KdfError {}

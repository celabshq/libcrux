//! This module contains the trait and related errors for hashers that take slices as
//! arguments and write the results to mutable slices.

use super::arrayref;

/// A trait for oneshot hashing, where the output is written to a provided slice.
pub trait Hash {
    /// Writes the digest for the given input byte slice, into `digest` in immediate mode.
    fn hash(digest: &mut [u8], payload: &[u8]) -> Result<usize, HashError>;
}

/// A trait for incremental hashing, where the output is written to a provided slice.
pub trait DigestIncremental: super::DigestIncrementalBase {
    /// Writes the digest into `digest`, consuming the hasher.
    fn finish(state: Self::IncrementalState, digest: &mut [u8]) -> Result<usize, FinishError>;
}

/// Error indicating that finalizing failed.
#[derive(Debug, PartialEq)]
pub enum FinishError {
    /// The length of the provided digest buffer is invalid.
    InvalidDigestLength,
    /// Unknown error.
    Unknown,
}

impl core::fmt::Display for FinishError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            FinishError::InvalidDigestLength => {
                "the length of the provided digest buffer is invalid"
            }
            FinishError::Unknown => "indicates an unknown error",
        };

        f.write_str(text)
    }
}

/// Error indicating that hashing failed.
#[derive(Debug, PartialEq)]
pub enum HashError {
    /// The length of the provided digest buffer is invalid.
    InvalidDigestLength,
    /// The length of the provided payload is invalid.
    InvalidPayloadLength,
    /// Indicates an unknown error.
    Unknown,
}

impl core::fmt::Display for HashError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            HashError::InvalidDigestLength => "the length of the provided digest buffer is invalid",
            HashError::InvalidPayloadLength => "the length of the provided payload is invalid",
            HashError::Unknown => "indicates an unknown error",
        };

        f.write_str(text)
    }
}

#[cfg(feature = "error-in-core")]
mod error_in_core {

    impl core::error::Error for super::HashError {}
    impl core::error::Error for super::FinishError {}
}

impl From<arrayref::HashError> for HashError {
    fn from(e: arrayref::HashError) -> Self {
        match e {
            arrayref::HashError::InvalidPayloadLength => Self::InvalidPayloadLength,
            arrayref::HashError::Unknown => Self::Unknown,
        }
    }
}

#[macro_export]
/// Implements [`Hash`] for any [`arrayref::Hash`].
macro_rules! impl_hash_trait {
    ($type:ty => $len:expr) => {
        impl $crate::digest::slice::Hash for $type {
            fn hash(
                digest: &mut [u8],
                payload: &[u8],
            ) -> Result<usize, $crate::digest::slice::HashError> {
                let digest: &mut [u8; $len] = digest
                    .try_into()
                    .map_err(|_| $crate::digest::slice::HashError::InvalidDigestLength)?;
                <Self as $crate::digest::arrayref::Hash<$len>>::hash(digest, payload)
                    .map(|_| $len)
                    .map_err($crate::digest::slice::HashError::from)
            }
        }
    };
}

#[macro_export]
/// Implements [`DigestIncremental`] for any [`arrayref::DigestIncremental`].
macro_rules! impl_digest_incremental_trait {
    ($type:ty => $incremental_state:ty, $len:expr) => {
        impl $crate::digest::slice::DigestIncremental for $type {
            fn finish(
                state: Self::IncrementalState,
                digest: &mut [u8],
            ) -> Result<usize, $crate::digest::slice::FinishError> {
                let digest: &mut [u8; $len] = digest
                    .try_into()
                    .map_err(|_| $crate::digest::slice::FinishError::InvalidDigestLength)?;
                <Self as $crate::digest::arrayref::DigestIncremental<$len>>::finish(state, digest);

                Ok($len)
            }
        }
    };
}

pub use impl_digest_incremental_trait;
pub use impl_hash_trait;

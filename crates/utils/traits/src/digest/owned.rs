//! This module contains the traits and related errors for hashers that take array references as
//! arguments and return values as arrays.

pub use arrayref::HashError;

use super::arrayref;

/// A trait for oneshot hashing, where the output is returned as an array.
pub trait Hash<const OUTPUT_LEN: usize> {
    /// Returns the digest for the given input byte slice, as an array, in immediate mode.
    fn hash(payload: &[u8]) -> Result<[u8; OUTPUT_LEN], HashError>;
}

/// A trait for incremental hashing, where the output is returned as an array.
pub trait DigestIncremental<const OUTPUT_LEN: usize>: super::DigestIncrementalBase {
    /// Returns the digest as an array, consuming the hasher.
    fn finish(state: Self::IncrementalState) -> [u8; OUTPUT_LEN];
}

impl<const OUTPUT_LEN: usize, D: arrayref::Hash<OUTPUT_LEN>> Hash<OUTPUT_LEN> for D {
    fn hash(payload: &[u8]) -> Result<[u8; OUTPUT_LEN], HashError> {
        let mut digest = [0; OUTPUT_LEN];
        Self::hash(&mut digest, payload).map(|_| digest)
    }
}
impl<const OUTPUT_LEN: usize, D: arrayref::DigestIncremental<OUTPUT_LEN>>
    DigestIncremental<OUTPUT_LEN> for D
{
    fn finish(state: Self::IncrementalState) -> [u8; OUTPUT_LEN] {
        let mut digest = [0; OUTPUT_LEN];
        Self::finish(state, &mut digest);
        digest
    }
}

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

// NOTE: This Digest trait and the new `digest` trait APIs overlap to some extent.
// See issue #1039
/// A Hash algorithm returning hashes of length `HASH_LEN`.
pub trait Digest<const HASH_LEN: usize> {
    /// Writes the digest for the given input byte slice, into `digest` in immediate mode.
    fn hash(digest: &mut [u8], payload: &[u8]);

    /// Add the `payload` to the digest.
    fn update(&mut self, payload: &[u8]);

    /// Writes the digest into `digest`.
    ///
    /// Note that the digest state can be continued to be used, to extend the
    /// digest.
    fn finish(&self, digest: &mut [u8; HASH_LEN]);

    /// Reset the digest state.
    fn reset(&mut self);
}

pub mod aead;
pub mod digest;
pub mod ecdh;
pub mod kem;

pub use libcrux_secrets;

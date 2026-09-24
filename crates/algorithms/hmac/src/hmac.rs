//! HMAC
//!
//! This crate implements HMAC on SHA 2 (except for SHA 224) and SHA 3.
#![no_std]

#[cfg(not(feature = "expose-hacl"))]
mod hacl {
    pub(crate) mod hmac;
}

#[cfg(feature = "expose-hacl")]
pub mod hacl {
    pub mod hmac;
}

mod impl_hacl;
mod incremental;

pub use impl_hacl::*;
pub use incremental::*;

/// Streaming HMAC state.
///
/// Initialize with the concrete type's `new(key)` constructor, feed data
/// fragments with [`update`](HmacState::update), and obtain the tag with
/// [`finalize`](HmacState::finalize).
///
/// Implementations are available as [`HmacSha256`], [`HmacSha384`], and
/// [`HmacSha512`].
pub trait HmacState<const OUTLEN: usize> {
    /// Create a new [`HmacState`].
    fn new(key: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;

    /// Feed a data fragment into the HMAC computation.
    fn update(&mut self, data: &[u8]) -> Result<(), Error>;

    /// Finalize the HMAC and write the tag into `dst`.
    fn finalize(self, dst: &mut [u8; OUTLEN]);
}

/// HMAC Errors
#[derive(Debug)]
pub enum Error {
    InvalidInputLength,
}

/// The HMAC algorithm defining the used hash function.
///
/// * `Sha256`, `Sha384`, and `Sha512` are SHA-2.
/// * SHA-3 variants are prefixed with `Sha3_`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Algorithm {
    // Not implemented
    // Sha224
    Sha256,
    Sha384,
    Sha512,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
}

const SHA256_TAG_SIZE: usize = 32;
const SHA384_TAG_SIZE: usize = 48;
const SHA512_TAG_SIZE: usize = 64;
const SHA3_224_TAG_SIZE: usize = 28;
const SHA3_256_TAG_SIZE: usize = 32;
const SHA3_384_TAG_SIZE: usize = 48;
const SHA3_512_TAG_SIZE: usize = 64;

/// Get the tag size for a given algorithm.
pub const fn tag_size(alg: Algorithm) -> usize {
    match alg {
        Algorithm::Sha256 => SHA256_TAG_SIZE,
        Algorithm::Sha384 => SHA384_TAG_SIZE,
        Algorithm::Sha512 => SHA512_TAG_SIZE,
        Algorithm::Sha3_224 => SHA3_224_TAG_SIZE,
        Algorithm::Sha3_256 => SHA3_256_TAG_SIZE,
        Algorithm::Sha3_384 => SHA3_384_TAG_SIZE,
        Algorithm::Sha3_512 => SHA3_512_TAG_SIZE,
    }
}

/// Compute the HMAC value with the given `alg` and `key` on `data`.
/// Writes the tag into the provided output buffer `tag`.
///
/// If the output buffer is shorter than the native output length of
/// `alg` the tag will be truncated to the output buffer's length. If
/// the output buffer is longer than the native output length of
/// `alg`, the tag will be written to the beginning of the output
/// buffer, leaving the rest unchanged.  Panics if either `key` or
/// `data` are longer than `u32::MAX`.
pub fn hmac(alg: Algorithm, key: &[u8], data: &[u8], tag: &mut [u8]) {
    let tag_len = core::cmp::min(tag.len(), tag_size(alg));
    match alg {
        Algorithm::Sha256 => {
            let mut buf = [0u8; SHA256_TAG_SIZE];
            hmac_sha2_256(&mut buf, key, data);
            tag[..tag_len].copy_from_slice(&buf[..tag_len]);
        }
        Algorithm::Sha384 => {
            let mut buf = [0u8; SHA384_TAG_SIZE];
            hmac_sha2_384(&mut buf, key, data);
            tag[..tag_len].copy_from_slice(&buf[..tag_len]);
        }
        Algorithm::Sha512 => {
            let mut buf = [0u8; SHA512_TAG_SIZE];
            hmac_sha2_512(&mut buf, key, data);
            tag[..tag_len].copy_from_slice(&buf[..tag_len]);
        }
        Algorithm::Sha3_224 => {
            let mut buf = [0u8; SHA3_224_TAG_SIZE];
            hmac_sha3_224(&mut buf, key, data).expect("HMAC-SHA3 input too long");
            tag[..tag_len].copy_from_slice(&buf[..tag_len]);
        }
        Algorithm::Sha3_256 => {
            let mut buf = [0u8; SHA3_256_TAG_SIZE];
            hmac_sha3_256(&mut buf, key, data).expect("HMAC-SHA3 input too long");
            tag[..tag_len].copy_from_slice(&buf[..tag_len]);
        }
        Algorithm::Sha3_384 => {
            let mut buf = [0u8; SHA3_384_TAG_SIZE];
            hmac_sha3_384(&mut buf, key, data).expect("HMAC-SHA3 input too long");
            tag[..tag_len].copy_from_slice(&buf[..tag_len]);
        }
        Algorithm::Sha3_512 => {
            let mut buf = [0u8; SHA3_512_TAG_SIZE];
            hmac_sha3_512(&mut buf, key, data).expect("HMAC-SHA3 input too long");
            tag[..tag_len].copy_from_slice(&buf[..tag_len]);
        }
    };
}

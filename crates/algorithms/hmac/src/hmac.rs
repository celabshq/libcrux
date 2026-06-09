//! HMAC
//!
//! This crate implements HMAC on SHA 1 and SHA 2 (except for SHA 224).
#![no_std]

extern crate alloc;

use alloc::vec::Vec;

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
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Algorithm {
    // Not implemented
    // Sha224
    Sha256,
    Sha384,
    Sha512,
}

/// Get the tag size for a given algorithm.
pub const fn tag_size(alg: Algorithm) -> usize {
    match alg {
        Algorithm::Sha256 => 32,
        Algorithm::Sha384 => 48,
        Algorithm::Sha512 => 64,
    }
}

/// Compute the HMAC value with the given `alg` and `key` on `data` with an
/// output tag length of `tag_length`.
/// Returns a vector of length `tag_length`.
/// Panics if either `key` or `data` are longer than `u32::MAX`.
pub fn hmac(alg: Algorithm, key: &[u8], data: &[u8], tag_length: Option<usize>) -> Vec<u8> {
    let native_tag_length = tag_size(alg);
    let tag_length = match tag_length {
        Some(v) => v,
        None => native_tag_length,
    };
    let mut dst: Vec<_> = match alg {
        Algorithm::Sha256 => wrap_bufalloc(|buf| hmac_sha2_256(buf, key, data)),
        Algorithm::Sha384 => wrap_bufalloc(|buf| hmac_sha2_384(buf, key, data)),
        Algorithm::Sha512 => wrap_bufalloc(|buf| hmac_sha2_512(buf, key, data)),
    };
    dst.truncate(tag_length);
    dst
}

#[inline(always)]
fn wrap_bufalloc<const N: usize, F: Fn(&mut [u8; N])>(f: F) -> Vec<u8> {
    let mut buf = [0u8; N];
    f(&mut buf);
    buf.to_vec()
}

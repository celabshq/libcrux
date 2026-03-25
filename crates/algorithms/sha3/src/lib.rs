//! # SHA3
//!
//! A SHA3 implementation with optional simd optimisations.

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod simd;

mod generic_keccak;

#[cfg(not(any(hax, eurydice)))]
mod impl_digest_trait;
#[cfg(not(any(hax, eurydice)))]
pub use impl_digest_trait::*;

#[cfg(hax)]
use hax_lib::int::*;

mod traits;

/// Size in bytes of a SHA3 224 digest.
pub const SHA3_224_DIGEST_SIZE: usize = 28;
/// Size in bytes of a SHA3 256 digest.
pub const SHA3_256_DIGEST_SIZE: usize = 32;
/// Size in bytes of a SHA3 2384 digest.
pub const SHA3_384_DIGEST_SIZE: usize = 48;
/// Size in bytes of a SHA3 512 digest.
pub const SHA3_512_DIGEST_SIZE: usize = 64;

/// F* verification helper
#[cfg(hax)]
pub(crate) mod proof_utils;

/// The Digest Algorithm.
#[cfg_attr(not(eurydice), derive(Clone, Copy, Debug, PartialEq))]
#[repr(u32)]
pub enum Algorithm {
    /// SHA3 224
    Sha224 = 1,

    /// SHA3 256
    Sha256 = 2,

    /// SHA3 384
    Sha384 = 3,

    /// SHA3 512
    Sha512 = 4,
}

// Verification fails because of the panic
#[cfg(not(any(hax, eurydice)))]
impl From<u32> for Algorithm {
    fn from(v: u32) -> Algorithm {
        match v {
            1 => Algorithm::Sha224,
            2 => Algorithm::Sha256,
            3 => Algorithm::Sha384,
            4 => Algorithm::Sha512,
            _ => panic!("Invalid SHA3 Algorithm code"),
        }
    }
}

impl From<Algorithm> for u32 {
    fn from(v: Algorithm) -> u32 {
        match v {
            Algorithm::Sha224 => 1,
            Algorithm::Sha256 => 2,
            Algorithm::Sha384 => 3,
            Algorithm::Sha512 => 4,
        }
    }
}

/// Returns the output size of a digest.
pub const fn digest_size(mode: Algorithm) -> usize {
    match mode {
        Algorithm::Sha224 => SHA3_224_DIGEST_SIZE,
        Algorithm::Sha256 => SHA3_256_DIGEST_SIZE,
        Algorithm::Sha384 => SHA3_384_DIGEST_SIZE,
        Algorithm::Sha512 => SHA3_512_DIGEST_SIZE,
    }
}

/// SHA3
#[hax_lib::fstar::options("--split_queries always")]
#[hax_lib::requires(payload.len().to_int() <= u32::MAX.to_int())]
pub fn hash<const LEN: usize>(algorithm: Algorithm, payload: &[u8]) -> [u8; LEN] {
    #[cfg(not(eurydice))]
    debug_assert!(payload.len() <= u32::MAX as usize);

    let mut out = [0u8; LEN];
    match algorithm {
        Algorithm::Sha224 => portable::sha224(&mut out, payload),
        Algorithm::Sha256 => portable::sha256(&mut out, payload),
        Algorithm::Sha384 => portable::sha384(&mut out, payload),
        Algorithm::Sha512 => portable::sha512(&mut out, payload),
    }
    out
}

/// SHA3
pub use hash as sha3;

/// SHA3 224
#[cfg_attr(not(eurydice), inline(always))]
#[hax_lib::requires(
    data.len().to_int() <= u32::MAX.to_int()
)]
pub fn sha224(data: &[u8]) -> [u8; SHA3_224_DIGEST_SIZE] {
    let mut out = [0u8; SHA3_224_DIGEST_SIZE];
    sha224_ema(&mut out, data);
    out
}

/// SHA3 224
///
/// Preconditions:
/// - `digest.len() == 28`
#[cfg_attr(not(eurydice), inline(always))]
#[hax_lib::requires(
    payload.len().to_int() <= u32::MAX.to_int() &&
    digest.len().to_int() == int!(28)
)]
pub fn sha224_ema(digest: &mut [u8], payload: &[u8]) {
    debug_assert!(payload.len() <= u32::MAX as usize);
    debug_assert!(digest.len() == 28);

    portable::sha224(digest, payload)
}

/// SHA3 256
#[cfg_attr(not(eurydice), inline(always))]
#[hax_lib::requires(
    data.len().to_int() <= u32::MAX.to_int()
)]
pub fn sha256(data: &[u8]) -> [u8; SHA3_256_DIGEST_SIZE] {
    let mut out = [0u8; SHA3_256_DIGEST_SIZE];
    sha256_ema(&mut out, data);
    out
}

/// SHA3 256
#[cfg_attr(not(eurydice), inline(always))]
#[hax_lib::requires(
    payload.len().to_int() <= u32::MAX.to_int() &&
    digest.len().to_int() == int!(32)
)]
pub fn sha256_ema(digest: &mut [u8], payload: &[u8]) {
    debug_assert!(payload.len() <= u32::MAX as usize);
    debug_assert!(digest.len() == 32);

    portable::sha256(digest, payload)
}

/// SHA3 384
#[cfg_attr(not(eurydice), inline(always))]
#[hax_lib::requires(
    data.len().to_int() <= u32::MAX.to_int()
)]
pub fn sha384(data: &[u8]) -> [u8; SHA3_384_DIGEST_SIZE] {
    let mut out = [0u8; SHA3_384_DIGEST_SIZE];
    sha384_ema(&mut out, data);
    out
}

/// SHA3 384
#[cfg_attr(not(eurydice), inline(always))]
#[hax_lib::requires(
    payload.len().to_int() <= u32::MAX.to_int() &&
    digest.len().to_int() == int!(48)
)]
pub fn sha384_ema(digest: &mut [u8], payload: &[u8]) {
    debug_assert!(payload.len() <= u32::MAX as usize);
    debug_assert!(digest.len() == 48);

    portable::sha384(digest, payload)
}

/// SHA3 512
#[cfg_attr(not(eurydice), inline(always))]
#[hax_lib::requires(
    data.len().to_int() <= u32::MAX.to_int()
)]
pub fn sha512(data: &[u8]) -> [u8; SHA3_512_DIGEST_SIZE] {
    let mut out = [0u8; SHA3_512_DIGEST_SIZE];
    sha512_ema(&mut out, data);
    out
}

/// SHA3 512
#[cfg_attr(not(eurydice), inline(always))]
#[hax_lib::requires(
    payload.len().to_int() <= u32::MAX.to_int() &&
    digest.len().to_int() == int!(64)
)]
pub fn sha512_ema(digest: &mut [u8], payload: &[u8]) {
    debug_assert!(payload.len() <= u32::MAX as usize);
    debug_assert!(digest.len() == 64);

    portable::sha512(digest, payload)
}

/// SHAKE 128
///
/// Note that the output length `BYTES` must fit into 32 bit. If it is longer,
/// the output will only return `u32::MAX` bytes.
#[cfg_attr(not(eurydice), inline(always))]
pub fn shake128<const BYTES: usize>(data: &[u8]) -> [u8; BYTES] {
    let mut out = [0u8; BYTES];
    portable::shake128(&mut out, data);
    out
}

/// SHAKE 128
///
/// Writes `out.len()` bytes.
#[cfg_attr(not(eurydice), inline(always))]
pub fn shake128_ema(out: &mut [u8], data: &[u8]) {
    portable::shake128(out, data);
}

/// SHAKE 256
///
/// Note that the output length `BYTES` must fit into 32 bit. If it is longer,
/// the output will only return `u32::MAX` bytes.
#[cfg_attr(not(eurydice), inline(always))]
pub fn shake256<const BYTES: usize>(data: &[u8]) -> [u8; BYTES] {
    let mut out = [0u8; BYTES];
    portable::shake256(&mut out, data);
    out
}

/// SHAKE 256
///
/// Writes `out.len()` bytes.
#[cfg_attr(not(eurydice), inline(always))]
pub fn shake256_ema(out: &mut [u8], data: &[u8]) {
    portable::shake256(out, data);
}

//  === Instantiation === //

/// A portable SHA3 implementation without platform dependent optimisations.
pub mod portable;

/// A neon optimised implementation.
///
/// When this is compiled for non-neon architectures, the functions panic.
/// The caller must make sure to check for hardware feature before calling these
/// functions and compile them in.
///
/// Feature `simd128` enables the implementations in this module.
#[cfg(feature = "simd128")]
pub mod neon;

/// An AVX2 optimised implementation.
///
/// When this is compiled for non-neon architectures, the functions panic.
/// The caller must make sure to check for hardware feature before calling these
/// functions and compile them in.
///
/// Feature `simd256` enables the implementations in this module.
#[cfg(feature = "simd256")]
pub mod avx2;

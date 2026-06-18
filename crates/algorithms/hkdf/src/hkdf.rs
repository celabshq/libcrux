//! # HKDF
//!
//! This crate implements HKDF ([RFC 5869](https://tools.ietf.org/html/rfc5869)) on SHA2-256, SHA2-384, and SHA2-512.
//! The implementation is based on code extracted from verified crypto code from the [HACL* project](https://hacl-star.github.io).
//!
//! ## Examples
//!
//! ### Using the typed SHA2-256 API
//!
//! ```
//! use libcrux_hkdf::{Hkdf, Sha2_256};
//! use libcrux_secrets::{U8, Classify, ClassifyRef, DeclassifyRef};
//!
//! // Input key material and salt
//! let ikm = &[0x0b; 22].classify(); // 22 bytes of 0x0b
//! let salt = b"salt".classify_ref();
//!
//! // Extract phase: derive pseudorandom key
//! let mut prk = [0u8; 32].classify(); // SHA2-256 output length
//! Hkdf::<Sha2_256>::extract(&mut prk, salt, ikm).unwrap();
//!
//! // Expand phase: derive keys for different purposes
//! let mut encrypt_key = [0u8; 16].classify();
//! let mut mac_key = [0u8; 16].classify();
//!
//! Hkdf::<Sha2_256>::expand(&mut encrypt_key, &prk, b"encrypt").unwrap();
//! Hkdf::<Sha2_256>::expand(&mut mac_key, &prk, b"mac").unwrap();
//! ```
//!
//! ### Using the dynamic API
//!
//! ```
//! use libcrux_hkdf::{extract, expand, Algorithm};
//! use libcrux_secrets::{U8, Classify, ClassifyRef, DeclassifyRef};
//!
//! // Input key material and salt
//! let ikm = &[0x0b; 22].classify();
//! let salt = b"salt".classify_ref();
//!
//! // Extract phase using SHA2-512
//! let mut prk = [0u8; 64].classify(); // SHA2-512 output length
//! extract(Algorithm::Sha512, &mut prk, salt, ikm).unwrap();
//!
//! // Expand phase: derive keys for different purposes
//! let mut encrypt_key = [0u8; 32].classify();
//! let mut mac_key = [0u8; 32].classify();
//!
//! expand(Algorithm::Sha512, &mut encrypt_key, &prk, b"encrypt").unwrap();
//! expand(Algorithm::Sha512, &mut mac_key, &prk, b"mac").unwrap();
//! ```
#![no_std]

use core::marker::PhantomData;

use libcrux_secrets::{Classify, DeclassifyRef, DeclassifyRefMut, U8};

pub mod hacl;

/// The HKDF algorithm defining the used hash function. Only needed for the functions with dynamic
/// algorithm selection.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Algorithm {
    Sha256,
    Sha384,
    Sha512,
}

/// HKDF extract using the `salt` and the input key material `ikm`.
/// The result is written to `prk`.
/// The `algo` argument is used for dynamic algorithm selection.
///
/// Returns nothing on success.
/// Returns [`ExtractError::ArgumentTooLong`] if one of `ikm` or `salt` is longer than [`u32::MAX`]
/// bytes.
/// Returns [`ExtractError::PrkTooShort`] if `prk` is shorter than the hash length.
pub fn extract(
    algo: Algorithm,
    prk: &mut [U8],
    salt: &[U8],
    ikm: &[U8],
) -> Result<(), ExtractError> {
    match algo {
        Algorithm::Sha256 => sha2_256::extract(prk, salt, ikm),
        Algorithm::Sha384 => sha2_384::extract(prk, salt, ikm),
        Algorithm::Sha512 => sha2_512::extract(prk, salt, ikm),
    }
}

/// HKDF expand. The argument names match the specification.
/// The result is written to `okm`.
/// The `algo` argument is used for dynamic algorithm selection.
///
/// Returns nothing on success.
/// Returns [`ExpandError::ArgumentTooLong`] if one of `prk` or `info` is longer than
/// [`u32::MAX`] bytes.
/// Returns [`ExpandError::PrkTooShort`] if `okm` is shorter than hash length.
/// Returns [`ExpandError::OutputTooLong`] if `okm` is longer than 255 times the respective hash
/// length.
pub fn expand(algo: Algorithm, okm: &mut [U8], prk: &[U8], info: &[u8]) -> Result<(), ExpandError> {
    match algo {
        Algorithm::Sha256 => sha2_256::expand(okm, prk, info),
        Algorithm::Sha384 => sha2_384::expand(okm, prk, info),
        Algorithm::Sha512 => sha2_512::expand(okm, prk, info),
    }
}

/// Full HKDF, i.e. both extract and expand, using the `salt` and the input key material `ikm`.
/// The argument names match the specification. The result is written to `okm`.
/// The `algo` argument is used for dynamic algorithm selection.
///
/// Returns nothing on success.
/// Returns [`ExpandError::ArgumentTooLong`] if one of `prk` or `info` is longer than
/// [`u32::MAX`] bytes.
/// Returns [`ExpandError::PrkTooShort`] if `okm` is shorter than hash length.
/// Returns [`ExpandError::OutputTooLong`] if `okm` is longer than 255 times the respective hash
/// length.
pub fn hkdf(
    algo: Algorithm,
    okm: &mut [U8],
    salt: &[U8],
    ikm: &[U8],
    info: &[u8],
) -> Result<(), ExpandError> {
    match algo {
        Algorithm::Sha256 => sha2_256::hkdf(okm, salt, ikm, info),
        Algorithm::Sha384 => sha2_384::hkdf(okm, salt, ikm, info),
        Algorithm::Sha512 => sha2_512::hkdf(okm, salt, ikm, info),
    }
}

/// Type marker for SHA2-256 hash algorithm.
///
/// This struct is used as a type parameter for [`Hkdf<Sha2_256>`] to provide
/// compile-time selection of the SHA2-256 algorithm for HKDF operations.
/// SHA2-256 produces 32-byte (256-bit) hash outputs.
pub struct Sha2_256;

/// Type marker for SHA2-384 hash algorithm.
///
/// This struct is used as a type parameter for [`Hkdf<Sha2_384>`] to provide
/// compile-time selection of the SHA2-384 algorithm for HKDF operations.
/// SHA2-384 produces 48-byte (384-bit) hash outputs.
pub struct Sha2_384;

/// Type marker for SHA2-512 hash algorithm.
///
/// This struct is used as a type parameter for [`Hkdf<Sha2_512>`] to provide
/// compile-time selection of the SHA2-512 algorithm for HKDF operations.
/// SHA2-512 produces 64-byte (512-bit) hash outputs.
pub struct Sha2_512;

/// HKDF implementation with compile-time algorithm selection.
///
/// This struct provides type-safe HKDF operations for a specific hash algorithm
/// determined at compile time. The algorithm is specified using type markers
/// like [`Sha2_256`], [`Sha2_384`], or [`Sha2_512`].
///
/// The implementation follows RFC 5869 and uses verified cryptographic code
/// from the HACL* project.
///
/// # Type Parameters
///
/// * `Algo` - The hash algorithm type marker (e.g., [`Sha2_256`])
///
/// # Examples
///
/// ```
/// use libcrux_hkdf::{Hkdf, Sha2_256};
/// use libcrux_secrets::{U8, Classify, ClassifyRef};
///
/// let ikm = &[0x0b; 22].classify();
/// let salt = b"salt".classify_ref();
///
/// let mut prk = [0u8; 32].classify();
/// Hkdf::<Sha2_256>::extract(&mut prk, salt, ikm).unwrap();
/// ```
pub struct Hkdf<Algo>(PhantomData<Algo>);

impl Algorithm {
    /// Returns the digest length of the underlying hash function.
    pub const fn hash_len(self) -> usize {
        match self {
            Algorithm::Sha256 => 32,
            Algorithm::Sha384 => 48,
            Algorithm::Sha512 => 64,
        }
    }
}

/// Generates HKDF implementation modules for specific hash algorithms.
///
/// This macro creates a complete HKDF implementation module for a specific SHA2 algorithm,
/// including both typed API methods on the [`Hkdf`] struct and standalone module functions.
/// It generates implementations for extract, expand, and full HKDF operations with both
/// fixed-size array references and variable-size slice variants.
///
/// Parameters:
///
/// * `$struct_name` - The path to the hash algorithm type marker (e.g., `crate::Sha2_256`)
/// * `$name` - The module name to generate (e.g., `sha2_256`)
/// * `$string_name` - A string literal describing the algorithm (e.g., `"SHA2-256"`)
/// * `$mode` - The corresponding [`Algorithm`] enum variant (e.g., `Algorithm::Sha256`)
/// * `$extract` - The name of the HACL extract function (e.g., `extract_sha2_256`)
/// * `$expand` - The name of the HACL expand function (e.g., `expand_sha2_256`)
/// * `$hash_len` - The hash output length in bytes as a literal (e.g., `32`)
///
///
/// This generates the `sha2_256` module and implements all HKDF methods for `Hkdf<Sha2_256>`.
macro_rules! impl_hkdf {
    ($struct_name:path, $name:ident, $string_name:literal, $mode:path, $extract:ident, $expand:ident,$hash_len:literal) => {
        #[doc = concat!("HKDF implementation for ", $string_name, ".")]
        ///
        /// This module provides HKDF (HMAC-based Key Derivation Function) operations
        /// specifically for the underlying hash algorithm. It includes both standalone
        /// functions and methods on the typed [`Hkdf`] struct.
        ///
        /// The `_arrayref` variants work with compile-time known PRK sizes for better type safety,
        /// while the regular variants accept slices and perform runtime validation.
        pub mod $name {
            use libcrux_secrets::U8;

            use super::*;

            impl Hkdf<$struct_name> {
                /// HKDF extract using the `salt` and the input key material `ikm`.
                /// The result is written to `prk`.
                ///
                /// Returns nothing on success.
                /// Returns [`ExtractError::ArgumentTooLong`] if one of `ikm` or `salt` is longer than
                /// [`u32::MAX`] bytes.
                #[inline(always)]
                pub fn extract_arrayref(
                    prk: &mut [U8; $hash_len],
                    salt: &[U8],
                    ikm: &[U8],
                ) -> Result<(), ArrayReferenceExtractError> {
                    extract_arrayref(prk, salt, ikm)
                }

                /// HKDF extract using the `salt` and the input key material `ikm`.
                /// The result is written to `prk`.
                ///
                /// Returns nothing on success.
                /// Returns [`ExtractError::ArgumentTooLong`] if one of `ikm` or `salt` is longer than
                /// [`u32::MAX`] bytes.
                /// Returns [`ExtractError::PrkTooShort`] if `prk` is shorter than hash length.
                #[inline(always)]
                pub fn extract(
                    prk: &mut [U8],
                    salt: &[U8],
                    ikm: &[U8],
                ) -> Result<(), ExtractError> {
                    extract(prk, salt, ikm)
                }

                /// HKDF expand using the pre-key material `prk` and `info`.
                /// The output is written to `okm`.
                ///
                /// Returns nothing on success.
                /// Returns [`ExpandError::OutputTooLong`] if `okm` is too long (longer than
                /// `255 * hash_length`)
                /// Returns [`ExpandError::ArgumentTooLong`] if one of `prk` or `info` is longer than
                /// [`u32::MAX`] bytes.
                #[inline(always)]
                pub fn expand_arrayref(
                    okm: &mut [U8],
                    prk: &[U8; $hash_len],
                    info: &[u8],
                ) -> Result<(), ArrayReferenceExpandError> {
                    if okm.len() > 255 * $hash_len {
                        // Output size is too large. HACL doesn't catch this.
                        return Err(ArrayReferenceExpandError::OutputTooLong);
                    }

                    expand_arrayref(okm, prk, info)
                }

                /// HKDF expand using the pre-key material `prk` and `info`.
                /// The output is written to `okm`.
                ///
                /// Returns nothing on success.
                /// Returns [`ExpandError::OutputTooLong`] if `okm` is too long (longer than
                /// `255 * hash_length`)
                /// Returns [`ExpandError::ArgumentTooLong`] if one of `prk` or `info` is longer than
                /// [`u32::MAX`] bytes.
                /// Returns [`ExpandError::PrkTooShort`] if `prk` is shorter than hash length.
                #[inline(always)]
                pub fn expand(okm: &mut [U8], prk: &[U8], info: &[u8]) -> Result<(), ExpandError> {
                    expand(okm, prk, info)
                }

                /// Full HKDF using the `salt`, input key material `ikm`, `info`.
                /// The result is written to `okm`.
                /// The output length is defined through the length of `okm`.
                /// Calls `extract` and `expand` with the given input.
                ///
                /// Returns nothing on success.
                /// Returns [`ExpandError::OutputTooLong`] if `okm` is too long (longer than
                /// `255 * hash_length`)
                /// Returns [`ExpandError::ArgumentTooLong`] if one of `prk` or `info` is longer than
                /// [`u32::MAX`] bytes.
                #[inline(always)]
                pub fn hkdf(
                    okm: &mut [U8],
                    salt: &[U8],
                    ikm: &[U8],
                    info: &[u8],
                ) -> Result<(), ExpandError> {
                    hkdf(okm, salt, ikm, info)
                }
            }

            /// HKDF extract using the `salt` and the input key material `ikm`.
            /// The result is written to `prk`.
            ///
            /// Returns nothing on success.
            /// Returns [`ExtractError::ArgumentTooLong`] if one of `ikm` or `salt` is longer than
            /// [`u32::MAX`] bytes.
            #[inline(always)]
            pub fn extract_arrayref(
                prk: &mut [U8; $hash_len],
                salt: &[U8],
                ikm: &[U8],
            ) -> Result<(), ArrayReferenceExtractError> {
                Ok(crate::hacl::$extract(
                    prk.declassify_ref_mut(),
                    salt.declassify_ref(),
                    checked_u32(salt.len())?,
                    ikm.declassify_ref(),
                    checked_u32(ikm.len())?,
                ))
            }

            /// HKDF extract using the `salt` and the input key material `ikm`.
            /// The result is written to `prk`.
            ///
            /// Returns nothing on success.
            /// Returns [`ExtractError::ArgumentTooLong`] if one of `ikm` or `salt` is longer than
            /// [`u32::MAX`] bytes.
            /// Returns [`ExtractError::PrkTooShort`] if `prk` is shorter than hash length.
            #[inline(always)]
            pub fn extract(prk: &mut [U8], salt: &[U8], ikm: &[U8]) -> Result<(), ExtractError> {
                let (prk, _) = prk
                    .split_at_mut_checked($hash_len)
                    .ok_or(ExtractError::PrkTooShort)?;
                let prk: &mut [U8; $hash_len] =
                    prk.try_into().map_err(|_| ExtractError::Unknown)?;

                extract_arrayref(prk, salt, ikm).map_err(ExtractError::from)
            }

            /// HKDF expand using the pre-key material `prk` and `info`.
            /// The output is written to `okm`.
            ///
            /// Returns nothing on success.
            /// Returns [`ExpandError::OutputTooLong`] if `okm` is too long (longer than
            /// `255 * hash_length`)
            /// Returns [`ExpandError::ArgumentTooLong`] if one of `prk` or `info` is longer than
            /// [`u32::MAX`] bytes.
            #[inline(always)]
            pub fn expand_arrayref(
                okm: &mut [U8],
                prk: &[U8; $hash_len],
                info: &[u8],
            ) -> Result<(), ArrayReferenceExpandError> {
                let okm_len = okm.len();
                if okm_len > 255 * $hash_len {
                    // Output size is too large. HACL doesn't catch this.
                    return Err(ArrayReferenceExpandError::OutputTooLong);
                }

                Ok(crate::hacl::$expand(
                    okm.declassify_ref_mut(),
                    prk.declassify_ref(),
                    checked_u32(prk.len())?,
                    info,
                    checked_u32(info.len())?,
                    checked_u32(okm_len)?,
                ))
            }

            /// HKDF expand using the pre-key material `prk` and `info`.
            /// The output is written to `okm`.
            ///
            /// Returns nothing on success.
            /// Returns [`ExpandError::OutputTooLong`] if `okm` is too long (longer than
            /// `255 * hash_length`)
            /// Returns [`ExpandError::ArgumentTooLong`] if one of `prk` or `info` is longer than
            /// [`u32::MAX`] bytes.
            /// Returns [`ExpandError::PrkTooShort`] if `prk` is shorter than hash length.
            #[inline(always)]
            pub fn expand(okm: &mut [U8], prk: &[U8], info: &[u8]) -> Result<(), ExpandError> {
                let (prk, _) = prk
                    .split_at_checked($hash_len)
                    .ok_or(ExpandError::PrkTooShort)?;
                let prk: &[U8; $hash_len] = prk.try_into().map_err(|_| ExpandError::Unknown)?;

                expand_arrayref(okm, prk, info).map_err(ExpandError::from)
            }

            /// Full HKDF using the `salt`, input key material `ikm`, `info`.
            /// The result is written to `okm`.
            /// The output length is defined through the length of `okm`.
            /// Calls `extract` and `expand` with the given input.
            ///
            /// Returns nothing on success.
            /// Returns [`ExpandError::OutputTooLong`] if `okm` is too long (longer than
            /// `255 * hash_length`)
            /// Returns [`ExpandError::ArgumentTooLong`] if one of `prk` or `info` is longer than
            /// [`u32::MAX`] bytes.
            #[inline(always)]
            pub fn hkdf(
                okm: &mut [U8],
                salt: &[U8],
                ikm: &[U8],
                info: &[u8],
            ) -> Result<(), ExpandError> {
                let mut prk = [0u8; $hash_len].classify();
                extract(&mut prk, salt, ikm)?;
                expand(okm, &prk, info)
            }
        }
    };
}

impl_hkdf!(
    crate::Sha2_256,
    sha2_256,
    "SHA2-256",
    Algorithm::Sha256,
    extract_sha2_256,
    expand_sha2_256,
    32
);

impl_hkdf!(
    crate::Sha2_384,
    sha2_384,
    "SHA2-384",
    Algorithm::Sha384,
    extract_sha2_384,
    expand_sha2_384,
    48
);

impl_hkdf!(
    crate::Sha2_512,
    sha2_512,
    "SHA2-512",
    Algorithm::Sha512,
    extract_sha2_512,
    expand_sha2_512,
    64
);

fn checked_u32(num: usize) -> Result<u32, ArgumentsTooLongError> {
    num.try_into().map_err(|_| ArgumentsTooLongError)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArrayReferenceExtractError {
    ArgumentTooLong,
    Unknown,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ExtractError {
    PrkTooShort,
    ArgumentTooLong,
    Unknown,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ArrayReferenceExpandError {
    OutputTooLong,
    ArgumentTooLong,
    Unknown,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ExpandError {
    OutputTooLong,
    PrkTooShort,
    ArgumentTooLong,
    Unknown,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ArgumentsTooLongError;

impl From<ArrayReferenceExtractError> for ExtractError {
    fn from(err: ArrayReferenceExtractError) -> Self {
        match err {
            ArrayReferenceExtractError::ArgumentTooLong => ExtractError::ArgumentTooLong,
            ArrayReferenceExtractError::Unknown => ExtractError::Unknown,
        }
    }
}

impl From<ArrayReferenceExpandError> for ExpandError {
    fn from(err: ArrayReferenceExpandError) -> Self {
        match err {
            ArrayReferenceExpandError::OutputTooLong => ExpandError::OutputTooLong,
            ArrayReferenceExpandError::ArgumentTooLong => ExpandError::ArgumentTooLong,
            ArrayReferenceExpandError::Unknown => ExpandError::Unknown,
        }
    }
}

impl From<ExtractError> for ExpandError {
    fn from(err: ExtractError) -> Self {
        match err {
            ExtractError::PrkTooShort => ExpandError::PrkTooShort,
            ExtractError::ArgumentTooLong => ExpandError::ArgumentTooLong,
            ExtractError::Unknown => ExpandError::Unknown,
        }
    }
}

impl From<ArgumentsTooLongError> for ArrayReferenceExtractError {
    fn from(_: ArgumentsTooLongError) -> Self {
        ArrayReferenceExtractError::ArgumentTooLong
    }
}
impl From<ArgumentsTooLongError> for ArrayReferenceExpandError {
    fn from(_: ArgumentsTooLongError) -> Self {
        ArrayReferenceExpandError::ArgumentTooLong
    }
}
impl From<ArgumentsTooLongError> for ExtractError {
    fn from(_: ArgumentsTooLongError) -> Self {
        ExtractError::ArgumentTooLong
    }
}
impl From<ArgumentsTooLongError> for ExpandError {
    fn from(_: ArgumentsTooLongError) -> Self {
        ExpandError::ArgumentTooLong
    }
}

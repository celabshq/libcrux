//! # AES-based AEADs
//!
//! This crate implements authenticated encryption with authenticated data
//! (AEAD) based on the AES block cipher. The following modes of operation
//! are implemented:
//!
//! - AES-GCM-128
//! - AES-GCM-256
//! - AES-CCM 128
//! - AES-CCM 256
//!
//! The crate provides optimized implementations for ARM and x86_64
//! platforms with support for AES hardware acceleration, as well as a
//! bit-sliced portable implementation.
//!
//! For general use, we provide a platform-multiplexing API via the
//! [`AesGcm128Key`], [`AesGcm256Key`], [`AesCcm128Key`] and
//! [`AesCcm256Key`] structs, which select the most performant
//! implementation at runtime.
//!
//! Usage example for AES-GCM 128:
//!
//! ```rust
//! // Multiplexed owned API
//! use libcrux_aes::{
//!     AeadConsts as _, AesGcm128, AesGcm128Key, AesGcm128Nonce, AesGcm128Tag, NONCE_LEN, TAG_LEN,
//! };
//!
//! let k: AesGcm128Key = [0; AesGcm128::KEY_LEN].into();
//! let nonce: AesGcm128Nonce = [0; NONCE_LEN].into();
//! let mut tag: AesGcm128Tag = [0; TAG_LEN].into();
//!
//! let pt = b"the quick brown fox jumps over the lazy dog";
//! let mut ct = [0; 43];
//! let mut pt_out = [0; 43];
//!
//! k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
//! k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
//! assert_eq!(pt, &pt_out);
//! ```
//!
//! We also provide access to [lower-level AEAD
//! APIs](libcrux_traits::aead) for the platform-multiplexing
//! implementation with the [`AesGcm128`], [`AesGcm256`], [`AesCcm128`]
//! and [`AesCcm256`] structs.
//!
//! Users who want to use a platform-specific implementation directly can
//! access them in submodules following the path scheme
//! `aes_gcm_128::{portable, x64, neon}` and `aes_ccm_128::{portable, x64,
//! neon}`.
//!
//! ## Supported Lengths
//!
//! The crate supports the following values for authentication tag and
//! nonce lengths:
//!
//! | Algorithm             | Key Length | Tag Length | Nonce Length |
//! |-----------------------|------------|------------|--------------|
//! | AES-GCM 128           | 16 bytes   | 16 bytes   | 12 bytes     |
//! | AES-CCM 128           | 16 bytes   | 16 bytes   | 12 bytes     |
//! | AES-CCM 128 Short Tag | 16 bytes   | 8 bytes    | 12 bytes     |
//! | AES-GCM 256           | 32 bytes   | 16 bytes   | 12 bytes     |
//! | AES-CCM 256           | 32 bytes   | 16 bytes   | 12 bytes     |
//! | AES-CCM 256 Short Tag | 32 bytes   | 8 bytes    | 12 bytes     |
//!
//! Short tag variants of AES-CCM as defined in [RFC
//! 6655](https://datatracker.ietf.org/doc/html/rfc6655) can be found in
//! the `short_tag` submodules of `aes_ccm_128` and `aes_ccm_256`.
//!
//! For plaintext, ciphertext and AAD lengths, we have the following
//! limitations:
//!
//! | Algorithm                 | Plain-/Ciphertext Length | AAD Length      |
//! |---------------------------|--------------------------|-----------------|
//! | AES-GCM on 64-bit systems | 2^36 - 32 bytes           | 2^61 - 1 bytes  |
//! | AES-CCM on 64-bit systems | 2^24 - 1 bytes           | 2^64 - 10 bytes |
//! | AES-GCM on 32-bit systems | 2^32 - 1 bytes           | 2^32 - 1 bytes  |
//! | AES-CCM on 32-bit systems | 2^24 - 1 bytes           | 2^32 - 6 bytes  |

#![no_std]
#![deny(unsafe_code)]
#[cfg(feature = "std")]
extern crate std;

mod aes;
mod ctr;
mod gf128;
mod platform;

mod traits_api;

mod aes_ccm;
mod aes_gcm;

/// Implementations of AES-GCM 128
///
/// This module contains implementations of AES-GCM 128:
/// - [`AesGcm128`]: A platform-multiplexing implementation, which will at
/// runtime select the most performant implementation among the following
/// for the given architecture at runtime.
#[cfg_attr(
    feature = "simd256",
    doc = "- [`aes_gcm_128::x64::X64AesGcm128`]: An implementation optimized for x86_64 AES-NI \
           instruction sets."
)]
#[cfg_attr(
    feature = "simd128",
    doc = "- [`aes_gcm_128::neon::NeonAesGcm128`]: An implementation optimized for ARM NEON \
           instruction sets."
)]
/// - [`aes_gcm_128::portable::PortableAesGcm128`]: A portable, bit-sliced
///   implementation.
///
/// See [`EncryptError`],
/// [`DecryptError`](libcrux_traits::aead::arrayref::DecryptError) and
/// [`KeyGenError`](libcrux_traits::aead::arrayref::DecryptError) for
/// errors.
///
/// The [`libcrux_traits`](libcrux_traits) crate provides two typed APIs
/// for AEADs:
///
/// ## Owned key-centric API
/// This API operates on owned arrays for keys, nonces and tags:
/// ```rust
/// // Using the multiplexed implementation.
/// use libcrux_aes::{
///     aes_gcm_128::{AesGcm128, Key, Nonce, Tag},
///     AeadConsts as _, NONCE_LEN, TAG_LEN,
/// };
///
/// let k: Key = [0; AesGcm128::KEY_LEN].into();
/// let nonce: Nonce = [0; NONCE_LEN].into();
/// let mut tag: Tag = [0; TAG_LEN].into();
///
/// let pt = b"the quick brown fox jumps over the lazy dog";
/// let mut ct = [0; 43];
/// let mut pt_out = [0; 43];
///
/// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
/// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
/// assert_eq!(pt, &pt_out);
/// ```
///
/// ## Refs key-centric API
/// This API operates on array references for keys, nonces and tags:
/// ```rust
/// // Using the multiplexed API
/// use libcrux_aes::{aes_gcm_128::AesGcm128, Aead as _, AeadConsts as _, NONCE_LEN, TAG_LEN};
///
/// let algo = AesGcm128;
///
/// let mut tag_bytes = [0; TAG_LEN];
/// let tag = algo.new_tag_mut(&mut tag_bytes).unwrap();
///
/// let key = algo.new_key(&[0; AesGcm128::KEY_LEN]).unwrap();
/// let nonce = algo.new_nonce(&[0; NONCE_LEN]).unwrap();
///
/// let pt = b"the quick brown fox jumps over the lazy dog";
/// let mut ct = [0; 43];
/// let mut pt_out = [0; 43];
///
/// key.encrypt(&mut ct, tag, nonce, b"", pt).unwrap();
/// let tag = algo.new_tag(&tag_bytes).unwrap();
/// key.decrypt(&mut pt_out, nonce, b"", &ct, tag).unwrap();
/// assert_eq!(pt, &pt_out);
/// ```
pub mod aes_gcm_128;

/// Implementations of AES-GCM 256
///
/// This module contains implementations of AES-GCM 256:
/// - [`AesGcm256`]: A platform-multiplexing implementation, which will at
/// runtime select the most performant implementation among the following
/// for the given architecture at runtime.
#[cfg_attr(
    feature = "simd256",
    doc = "- [`aes_gcm_256::x64::X64AesGcm256`]: An implementation optimized for x86_64 AES-NI \
           instruction sets."
)]
#[cfg_attr(
    feature = "simd128",
    doc = "- [`aes_gcm_256::neon::NeonAesGcm256`]: An implementation optimized for ARM NEON \
           instruction sets."
)]
/// - [`aes_gcm_256::portable::PortableAesGcm256`]: A portable, bit-sliced
///   implementation.
///
/// See [`EncryptError`],
/// [`DecryptError`](libcrux_traits::aead::arrayref::DecryptError) and
/// [`KeyGenError`](libcrux_traits::aead::arrayref::DecryptError) for
/// errors.
///
/// The [`libcrux_traits`](libcrux_traits) crate provides two typed APIs
/// for AEADs:
///
/// ## Owned key-centric API
/// This API operates on owned arrays for keys, nonces and tags:
/// ```rust
/// // Using the multiplexed implementation.
/// use libcrux_aes::{
///     aes_gcm_256::{AesGcm256, Key, Nonce, Tag},
///     AeadConsts as _, NONCE_LEN, TAG_LEN,
/// };
///
/// let k: Key = [0; AesGcm256::KEY_LEN].into();
/// let nonce: Nonce = [0; NONCE_LEN].into();
/// let mut tag: Tag = [0; TAG_LEN].into();
///
/// let pt = b"the quick brown fox jumps over the lazy dog";
/// let mut ct = [0; 43];
/// let mut pt_out = [0; 43];
///
/// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
/// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
/// assert_eq!(pt, &pt_out);
/// ```
///
/// ## Refs key-centric API
/// This API operates on array references for keys, nonces and tags:
/// ```rust
/// // Using the multiplexed API
/// use libcrux_aes::{aes_gcm_256::AesGcm256, Aead as _, AeadConsts as _, NONCE_LEN, TAG_LEN};
///
/// let algo = AesGcm256;
///
/// let mut tag_bytes = [0; TAG_LEN];
/// let tag = algo.new_tag_mut(&mut tag_bytes).unwrap();
///
/// let key = algo.new_key(&[0; AesGcm256::KEY_LEN]).unwrap();
/// let nonce = algo.new_nonce(&[0; NONCE_LEN]).unwrap();
///
/// let pt = b"the quick brown fox jumps over the lazy dog";
/// let mut ct = [0; 43];
/// let mut pt_out = [0; 43];
///
/// key.encrypt(&mut ct, tag, nonce, b"", pt).unwrap();
/// let tag = algo.new_tag(&tag_bytes).unwrap();
/// key.decrypt(&mut pt_out, nonce, b"", &ct, tag).unwrap();
/// assert_eq!(pt, &pt_out);
/// ```
pub mod aes_gcm_256;

/// Implementations of AES-CCM 128
///
/// This module contains implementations of AES-CCM 128:
/// - [`AesCcm128`]: A platform-multiplexing implementation, which will at
/// runtime select the most performant implementation among the following
/// for the given architecture at runtime.
#[cfg_attr(
    feature = "simd256",
    doc = "- [`x64::X64AesCcm128`](aes_ccm_128::x64::X64AesCcm128): An implementation optimized \
           for x86_64 AES-NI instruction sets."
)]
#[cfg_attr(
    feature = "simd128",
    doc = "- [`neon::NeonAesCcm128`](aes_ccm_128::neon::NeonAesCcm128): An implementation \
           optimized for ARM NEON instruction sets."
)]
/// - [`portable::PortableAesCcm128`](aes_ccm_128::portable::PortableAesCcm128):
///   A portable, bit-sliced implementation.
///
/// The [`short_tag`](crate::aes_ccm_128::short_tag) module provides
/// implementations for AES-CCM 128 with reduced tag length as defined in
/// [RFC 6655](https://datatracker.ietf.org/doc/html/rfc6655).
///
/// See [`EncryptError`],
/// [`DecryptError`](libcrux_traits::aead::arrayref::DecryptError) and
/// [`KeyGenError`](libcrux_traits::aead::arrayref::DecryptError) for
/// errors.
///
/// The [`libcrux_traits`](libcrux_traits) crate provides two typed APIs
/// for AEADs:
///
/// ## Owned key-centric API
/// This API operates on owned arrays for keys, nonces and tags:
/// ```rust
/// // Using the multiplexed implementation.
/// use libcrux_aes::{
///     aes_ccm_128::{AesCcm128, Key, Nonce, Tag},
///     AeadConsts as _, NONCE_LEN, TAG_LEN,
/// };
///
/// let k: Key = [0; AesCcm128::KEY_LEN].into();
/// let nonce: Nonce = [0; NONCE_LEN].into();
/// let mut tag: Tag = [0; TAG_LEN].into();
///
/// let pt = b"the quick brown fox jumps over the lazy dog";
/// let mut ct = [0; 43];
/// let mut pt_out = [0; 43];
///
/// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
/// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
/// assert_eq!(pt, &pt_out);
/// ```
///
/// ## Refs key-centric API
/// This API operates on array references for keys, nonces and tags:
/// ```rust
/// // Using the multiplexed API
/// use libcrux_aes::{aes_ccm_128::AesCcm128, Aead as _, AeadConsts as _, NONCE_LEN, TAG_LEN};
///
/// let algo = AesCcm128;
///
/// let mut tag_bytes = [0; TAG_LEN];
/// let tag = algo.new_tag_mut(&mut tag_bytes).unwrap();
///
/// let key = algo.new_key(&[0; AesCcm128::KEY_LEN]).unwrap();
/// let nonce = algo.new_nonce(&[0; NONCE_LEN]).unwrap();
///
/// let pt = b"the quick brown fox jumps over the lazy dog";
/// let mut ct = [0; 43];
/// let mut pt_out = [0; 43];
///
/// key.encrypt(&mut ct, tag, nonce, b"", pt).unwrap();
/// let tag = algo.new_tag(&tag_bytes).unwrap();
/// key.decrypt(&mut pt_out, nonce, b"", &ct, tag).unwrap();
/// assert_eq!(pt, &pt_out);
/// ```
pub mod aes_ccm_128 {
    use crate::aes_gcm::type_aliases;
    type_aliases!(AesCcm128, "AES-CCM 128");

    /// # Portable implementation of AES-CCM 128
    ///
    /// To use the portable implementation, `Key`, `Nonce`, and `Tag` types
    /// must be explicitly parameterized by the portable implementation.
    ///
    /// Example:
    /// ```rust
    /// // Using the portable implementation.
    /// use libcrux_aes::{
    ///     aes_ccm_128::portable::{Key, Nonce, PortableAesCcm128, Tag},
    ///     AeadConsts as _, NONCE_LEN, TAG_LEN,
    /// };
    ///
    /// let k: Key<PortableAesCcm128> = [0; PortableAesCcm128::KEY_LEN].into();
    /// let nonce: Nonce<PortableAesCcm128> = [0; NONCE_LEN].into();
    /// let mut tag: Tag<PortableAesCcm128> = [0; TAG_LEN].into();
    ///
    /// let pt = b"the quick brown fox jumps over the lazy dog";
    /// let mut ct = [0; 43];
    /// let mut pt_out = [0; 43];
    ///
    /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
    /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
    /// assert_eq!(pt, &pt_out);
    /// ```
    pub mod portable {
        pub use libcrux_traits::aead::{
            typed_owned::{Key, Nonce, Tag},
            typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef},
        };

        pub use crate::implementations::PortableAesCcm128;
    }

    #[cfg(feature = "simd128")]
    /// ARM NEON-optimized implementation of AES-CCM 128
    ///
    /// To use the NEON-optimized implementation, `Key`, `Nonce`, and `Tag`
    /// types must be explicitly parameterized by the NEON implementation.
    ///
    /// Example:
    /// ```rust
    /// // Using the NEON implementation.
    /// use libcrux_aes::{
    ///     aes_ccm_128::neon::{Key, NeonAesCcm128, Nonce, Tag},
    ///     AeadConsts as _, NONCE_LEN, TAG_LEN,
    /// };
    ///
    /// let k: Key<NeonAesCcm128> = [0; NeonAesCcm128::KEY_LEN].into();
    /// let nonce: Nonce<NeonAesCcm128> = [0; NONCE_LEN].into();
    /// let mut tag: Tag<NeonAesCcm128> = [0; TAG_LEN].into();
    ///
    /// let pt = b"the quick brown fox jumps over the lazy dog";
    /// let mut ct = [0; 43];
    /// let mut pt_out = [0; 43];
    ///
    /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
    /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
    /// assert_eq!(pt, &pt_out);
    /// ```
    pub mod neon {
        pub use libcrux_traits::aead::{
            typed_owned::{Key, Nonce, Tag},
            typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef},
        };

        pub use crate::implementations::NeonAesCcm128;
    }

    #[cfg(feature = "simd256")]
    /// AES-NI-optimized implementation of AES-CCM 128
    ///
    /// To use the AES-NI-optimized implementation, `Key`, `Nonce`, and `Tag`
    /// types must be explicitly parameterized by the AES-NI implementation.
    ///
    /// Example:
    /// ```rust
    /// // Using the AES-NI implementation.
    /// use libcrux_aes::{
    ///     aes_ccm_128::x64::{Key, Nonce, Tag, X64AesCcm128},
    ///     AeadConsts as _, NONCE_LEN, TAG_LEN,
    /// };
    ///
    /// let k: Key<X64AesCcm128> = [0; X64AesCcm128::KEY_LEN].into();
    /// let nonce: Nonce<X64AesCcm128> = [0; NONCE_LEN].into();
    /// let mut tag: Tag<X64AesCcm128> = [0; TAG_LEN].into();
    ///
    /// let pt = b"the quick brown fox jumps over the lazy dog";
    /// let mut ct = [0; 43];
    /// let mut pt_out = [0; 43];
    ///
    /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
    /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
    /// assert_eq!(pt, &pt_out);
    /// ```
    pub mod x64 {
        pub use libcrux_traits::aead::{
            typed_owned::{Key, Nonce, Tag},
            typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef},
        };

        pub use crate::implementations::X64AesCcm128;
    }

    /// Implementations of AES-CCM 128 ([RFC 6655](https://datatracker.ietf.org/doc/html/rfc6655) Short Tag)
    ///
    /// This module contains implementations of AES-CCM 128 with short 8 byte
    /// tag as defined by [RFC
    /// 6655](https://datatracker.ietf.org/doc/html/rfc6655):
    /// - [`AesCcm128ShortTag`](crate::aes_ccm_128::short_tag::AesCcm128ShortTag):
    /// A platform-multiplexing implementation, which will at runtime select
    /// the most performant implementation among the following for the given
    /// architecture at runtime.
    #[cfg_attr(
        feature = "simd256",
        doc = "- [`x64::X64AesCcm128ShortTag`](crate::aes_ccm_128::short_tag::x64::X64AesCcm128ShortTag): An implementation optimized for x86_64 AES-NI instruction sets."
    )]
    #[cfg_attr(
        feature = "simd128",
        doc = "- [`neon::NeonAesCcm128ShortTag`](crate::aes_ccm_128::short_tag::neon::NeonAesCcm128ShortTag): An implementation optimized for ARM NEON instruction sets."
    )]
    /// - [`portable::PortableAesCcm128ShortTag`](crate::aes_ccm_128::short_tag::portable::PortableAesCcm128ShortTag):
    ///   A portable, bit-sliced implementation.
    ///
    /// See [`EncryptError`](`crate::EncryptError`),
    /// [`DecryptError`](libcrux_traits::aead::arrayref::DecryptError) and
    /// [`KeyGenError`](libcrux_traits::aead::arrayref::DecryptError) for
    /// errors.
    ///
    /// The [`libcrux_traits`](libcrux_traits) crate provides two typed APIs
    /// for AEADs:
    ///
    /// ## Owned key-centric API
    /// This API operates on owned arrays for keys, nonces and tags:
    /// ```rust
    /// // Using the multiplexed implementation.
    /// use libcrux_aes::{
    ///     aes_ccm_128::short_tag::{AesCcm128ShortTag, Key, Nonce, Tag},
    ///     AeadConsts as _, CCM_SHORT_TAG_LEN, NONCE_LEN,
    /// };
    ///
    /// let k: Key = [0; AesCcm128ShortTag::KEY_LEN].into();
    /// let nonce: Nonce = [0; NONCE_LEN].into();
    /// let mut tag: Tag = [0; CCM_SHORT_TAG_LEN].into();
    ///
    /// let pt = b"the quick brown fox jumps over the lazy dog";
    /// let mut ct = [0; 43];
    /// let mut pt_out = [0; 43];
    ///
    /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
    /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
    /// assert_eq!(pt, &pt_out);
    /// ```
    ///
    /// ## Refs key-centric API
    /// This API operates on array references for keys, nonces and tags:
    /// ```rust
    /// // Using the multiplexed API
    /// use libcrux_aes::{
    ///     aes_ccm_128::short_tag::AesCcm128ShortTag, Aead as _, AeadConsts as _, CCM_SHORT_TAG_LEN,
    ///     NONCE_LEN,
    /// };
    ///
    /// let algo = AesCcm128ShortTag;
    ///
    /// let mut tag_bytes = [0; CCM_SHORT_TAG_LEN];
    /// let tag = algo.new_tag_mut(&mut tag_bytes).unwrap();
    ///
    /// let key = algo.new_key(&[0; AesCcm128ShortTag::KEY_LEN]).unwrap();
    /// let nonce = algo.new_nonce(&[0; NONCE_LEN]).unwrap();
    ///
    /// let pt = b"the quick brown fox jumps over the lazy dog";
    /// let mut ct = [0; 43];
    /// let mut pt_out = [0; 43];
    ///
    /// key.encrypt(&mut ct, tag, nonce, b"", pt).unwrap();
    /// let tag = algo.new_tag(&tag_bytes).unwrap();
    /// key.decrypt(&mut pt_out, nonce, b"", &ct, tag).unwrap();
    /// assert_eq!(pt, &pt_out);
    /// ```
    pub mod short_tag {
        use crate::aes_gcm::type_aliases;
        type_aliases!(AesCcm128ShortTag, "AES-CCM 128 (8 octet tag)");

        /// # Portable implementation of AES-CCM 128 (Short Tag)
        ///
        /// To use the portable implementation, `Key`, `Nonce`, and `Tag` types
        /// must be explicitly parameterized by the portable implementation.
        ///
        /// Example:
        /// ```rust
        /// // Using the portable implementation.
        /// use libcrux_aes::{
        ///     aes_ccm_128::short_tag::portable::{Key, Nonce, PortableAesCcm128ShortTag, Tag},
        ///     AeadConsts as _, CCM_SHORT_TAG_LEN, NONCE_LEN,
        /// };
        ///
        /// let k: Key<PortableAesCcm128ShortTag> = [0; PortableAesCcm128ShortTag::KEY_LEN].into();
        /// let nonce: Nonce<PortableAesCcm128ShortTag> = [0; NONCE_LEN].into();
        /// let mut tag: Tag<PortableAesCcm128ShortTag> = [0; CCM_SHORT_TAG_LEN].into();
        ///
        /// let pt = b"the quick brown fox jumps over the lazy dog";
        /// let mut ct = [0; 43];
        /// let mut pt_out = [0; 43];
        ///
        /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
        /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
        /// assert_eq!(pt, &pt_out);
        /// ```
        pub mod portable {
            pub use libcrux_traits::aead::{
                typed_owned::{Key, Nonce, Tag},
                typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef},
            };

            pub use crate::implementations::PortableAesCcm128ShortTag;
        }

        #[cfg(feature = "simd128")]

        /// ARM NEON-optimized implementation of AES-CCM 128 (Short Tag)
        ///
        /// To use the NEON-optimized implementation, `Key`, `Nonce`, and `Tag`
        /// types must be explicitly parameterized by the NEON implementation.
        ///
        /// Example:
        /// ```rust
        /// // Using the NEON implementation.
        /// use libcrux_aes::{
        ///     aes_ccm_128::short_tag::neon::{Key, NeonAesCcm128ShortTag, Nonce, Tag},
        ///     AeadConsts as _, CCM_SHORT_TAG_LEN, NONCE_LEN,
        /// };
        ///
        /// let k: Key<NeonAesCcm128ShortTag> = [0; NeonAesCcm128ShortTag::KEY_LEN].into();
        /// let nonce: Nonce<NeonAesCcm128ShortTag> = [0; NONCE_LEN].into();
        /// let mut tag: Tag<NeonAesCcm128ShortTag> = [0; CCM_SHORT_TAG_LEN].into();
        ///
        /// let pt = b"the quick brown fox jumps over the lazy dog";
        /// let mut ct = [0; 43];
        /// let mut pt_out = [0; 43];
        ///
        /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
        /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
        /// assert_eq!(pt, &pt_out);
        /// ```
        pub mod neon {
            pub use libcrux_traits::aead::{
                typed_owned::{Key, Nonce, Tag},
                typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef},
            };

            pub use crate::implementations::NeonAesCcm128ShortTag;
        }

        #[cfg(feature = "simd256")]
        /// AES-NI-optimized implementation of AES-CCM 128 (Short Tag)
        ///
        /// To use the AES-NI-optimized implementation, `Key`, `Nonce`, and
        /// `Tag` types must be explicitly parameterized by the AES-NI
        /// implementation.
        ///
        /// Example:
        /// ```rust
        /// // Using the AES-NI implementation.
        /// use libcrux_aes::{
        ///     aes_ccm_128::short_tag::x64::{Key, Nonce, Tag, X64AesCcm128ShortTag},
        ///     AeadConsts as _, CCM_SHORT_TAG_LEN, NONCE_LEN,
        /// };
        ///
        /// let k: Key<X64AesCcm128ShortTag> = [0; X64AesCcm128ShortTag::KEY_LEN].into();
        /// let nonce: Nonce<X64AesCcm128ShortTag> = [0; NONCE_LEN].into();
        /// let mut tag: Tag<X64AesCcm128ShortTag> = [0; CCM_SHORT_TAG_LEN].into();
        ///
        /// let pt = b"the quick brown fox jumps over the lazy dog";
        /// let mut ct = [0; 43];
        /// let mut pt_out = [0; 43];
        ///
        /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
        /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
        /// assert_eq!(pt, &pt_out);
        /// ```
        pub mod x64 {
            pub use libcrux_traits::aead::{
                typed_owned::{Key, Nonce, Tag},
                typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef},
            };

            pub use crate::implementations::X64AesCcm128ShortTag;
        }
    }
}

/// Implementations of AES-CCM 256
///
/// This module contains implementations of AES-CCM 256:
/// - [`AesCcm256`]: A platform-multiplexing implementation, which will at
/// runtime select the most performant implementation among the following
/// for the given architecture at runtime.
#[cfg_attr(
    feature = "simd256",
    doc = "- [`x64::X64AesCcm256`](aes_ccm_256::x64::X64AesCcm256): An implementation optimized \
           for x86_64 AES-NI instruction sets."
)]
#[cfg_attr(
    feature = "simd128",
    doc = "- [`neon::NeonAesCcm256`](aes_ccm_256::neon::NeonAesCcm256): An implementation \
           optimized for ARM NEON instruction sets."
)]
/// - [`portable::PortableAesCcm256`](aes_ccm_256::portable::PortableAesCcm256):
///   A portable, bit-sliced implementation.
///
/// The [`short_tag`](crate::aes_ccm_256::short_tag) module provides
/// implementations for AES-CCM 256 with reduced tag length as defined in
/// [RFC 6655](https://datatracker.ietf.org/doc/html/rfc6655).
///
/// See [`EncryptError`],
/// [`DecryptError`](libcrux_traits::aead::arrayref::DecryptError) and
/// [`KeyGenError`](libcrux_traits::aead::arrayref::DecryptError) for
/// errors.
///
/// The [`libcrux_traits`](libcrux_traits) crate provides two typed APIs for
/// AEADs:
///
/// ## Owned key-centric API
/// This API operates on owned arrays for keys, nonces and tags:
/// ```rust
/// // Using the multiplexed implementation.
/// use libcrux_aes::{
///     aes_ccm_256::{AesCcm256, Key, Nonce, Tag},
///     AeadConsts as _, NONCE_LEN, TAG_LEN,
/// };
///
/// let k: Key = [0; AesCcm256::KEY_LEN].into();
/// let nonce: Nonce = [0; NONCE_LEN].into();
/// let mut tag: Tag = [0; TAG_LEN].into();
///
/// let pt = b"the quick brown fox jumps over the lazy dog";
/// let mut ct = [0; 43];
/// let mut pt_out = [0; 43];
///
/// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
/// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
/// assert_eq!(pt, &pt_out);
/// ```
///
/// ## Refs key-centric API
/// This API operates on array references for keys, nonces and tags:
/// ```rust
/// // Using the multiplexed API
/// use libcrux_aes::{aes_ccm_256::AesCcm256, Aead as _, AeadConsts as _, NONCE_LEN, TAG_LEN};
///
/// let algo = AesCcm256;
///
/// let mut tag_bytes = [0; TAG_LEN];
/// let tag = algo.new_tag_mut(&mut tag_bytes).unwrap();
///
/// let key = algo.new_key(&[0; AesCcm256::KEY_LEN]).unwrap();
/// let nonce = algo.new_nonce(&[0; NONCE_LEN]).unwrap();
///
/// let pt = b"the quick brown fox jumps over the lazy dog";
/// let mut ct = [0; 43];
/// let mut pt_out = [0; 43];
///
/// key.encrypt(&mut ct, tag, nonce, b"", pt).unwrap();
/// let tag = algo.new_tag(&tag_bytes).unwrap();
/// key.decrypt(&mut pt_out, nonce, b"", &ct, tag).unwrap();
/// assert_eq!(pt, &pt_out);
/// ```
pub mod aes_ccm_256 {
    use crate::aes_gcm::type_aliases;
    type_aliases!(AesCcm256, "AES-CCM 256");

    /// # Portable implementation of AES-CCM 256
    ///
    /// To use the portable implementation, `Key`, `Nonce`, and `Tag` types
    /// must be explicitly parameterized by the portable implementation.
    ///
    /// Example:
    /// ```rust
    /// // Using the portable implementation.
    /// use libcrux_aes::{
    ///     aes_ccm_256::portable::{Key, Nonce, PortableAesCcm256, Tag},
    ///     AeadConsts as _, NONCE_LEN, TAG_LEN,
    /// };
    ///
    /// let k: Key<PortableAesCcm256> = [0; PortableAesCcm256::KEY_LEN].into();
    /// let nonce: Nonce<PortableAesCcm256> = [0; NONCE_LEN].into();
    /// let mut tag: Tag<PortableAesCcm256> = [0; TAG_LEN].into();
    ///
    /// let pt = b"the quick brown fox jumps over the lazy dog";
    /// let mut ct = [0; 43];
    /// let mut pt_out = [0; 43];
    ///
    /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
    /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
    /// assert_eq!(pt, &pt_out);
    /// ```
    pub mod portable {
        pub use libcrux_traits::aead::{
            typed_owned::{Key, Nonce, Tag},
            typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef},
        };

        pub use crate::implementations::PortableAesCcm256;
    }

    #[cfg(feature = "simd128")]
    /// ARM NEON-optimized implementation of AES-CCM 256
    ///
    /// To use the NEON-optimized implementation, `Key`, `Nonce`, and `Tag`
    /// types must be explicitly parameterized by the NEON implementation.
    ///
    /// Example:
    /// ```rust
    /// // Using the NEON implementation.
    /// use libcrux_aes::{
    ///     aes_ccm_256::neon::{Key, NeonAesCcm256, Nonce, Tag},
    ///     AeadConsts as _, NONCE_LEN, TAG_LEN,
    /// };
    ///
    /// let k: Key<NeonAesCcm256> = [0; NeonAesCcm256::KEY_LEN].into();
    /// let nonce: Nonce<NeonAesCcm256> = [0; NONCE_LEN].into();
    /// let mut tag: Tag<NeonAesCcm256> = [0; TAG_LEN].into();
    ///
    /// let pt = b"the quick brown fox jumps over the lazy dog";
    /// let mut ct = [0; 43];
    /// let mut pt_out = [0; 43];
    ///
    /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
    /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
    /// assert_eq!(pt, &pt_out);
    /// ```
    pub mod neon {
        pub use libcrux_traits::aead::{
            typed_owned::{Key, Nonce, Tag},
            typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef},
        };

        pub use crate::implementations::NeonAesCcm256;
    }

    #[cfg(feature = "simd256")]
    /// AES-NI-optimized implementation of AES-CCM 256
    ///
    /// To use the AES-NI-optimized implementation, `Key`, `Nonce`, and `Tag`
    /// types must be explicitly parameterized by the AES-NI implementation.
    ///
    /// Example:
    /// ```rust
    /// // Using the AES-NI implementation.
    /// use libcrux_aes::{
    ///     aes_ccm_256::x64::{Key, Nonce, Tag, X64AesCcm256},
    ///     AeadConsts as _, NONCE_LEN, TAG_LEN,
    /// };
    ///
    /// let k: Key<X64AesCcm256> = [0; X64AesCcm256::KEY_LEN].into();
    /// let nonce: Nonce<X64AesCcm256> = [0; NONCE_LEN].into();
    /// let mut tag: Tag<X64AesCcm256> = [0; TAG_LEN].into();
    ///
    /// let pt = b"the quick brown fox jumps over the lazy dog";
    /// let mut ct = [0; 43];
    /// let mut pt_out = [0; 43];
    ///
    /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
    /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
    /// assert_eq!(pt, &pt_out);
    /// ```
    pub mod x64 {
        pub use libcrux_traits::aead::{
            typed_owned::{Key, Nonce, Tag},
            typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef},
        };

        pub use crate::implementations::X64AesCcm256;
    }

    /// Implementations of AES-CCM 256 ([RFC
    /// 6655](https://datatracker.ietf.org/doc/html/rfc6655) Short Tag)
    ///
    /// This module contains implementations of AES-CCM 256 with short 8 byte
    /// tag as defined by [RFC
    /// 6655](https://datatracker.ietf.org/doc/html/rfc6655):
    /// - [`AesCcm256ShortTag`](crate::aes_ccm_256::short_tag::AesCcm256ShortTag):
    /// A platform-multiplexing implementation, which will at runtime select
    /// the most performant implementation among the following for the given
    /// architecture at runtime.
    #[cfg_attr(
        feature = "simd256",
        doc = "- [`x64::X64AesCcm256ShortTag`](crate::aes_ccm_256::short_tag::x64::X64AesCcm256ShortTag): An implementation optimized for x86_64 AES-NI instruction sets."
    )]
    #[cfg_attr(
        feature = "simd128",
        doc = "- [`neon::NeonAesCcm256ShortTag`](crate::aes_ccm_256::short_tag::neon::NeonAesCcm256ShortTag): An implementation optimized for ARM NEON instruction sets."
    )]
    /// - [`portable::PortableAesCcm256ShortTag`](crate::aes_ccm_256::short_tag::portable::PortableAesCcm256ShortTag): A portable, bit-sliced implementation.
    ///
    /// See [`EncryptError`](`crate::EncryptError`),
    /// [`DecryptError`](libcrux_traits::aead::arrayref::DecryptError) and
    /// [`KeyGenError`](libcrux_traits::aead::arrayref::DecryptError) for
    /// errors.
    ///
    /// The [`libcrux_traits`](libcrux_traits) crate provides two typed APIs for
    /// AEADs:
    ///
    /// ## Owned key-centric API
    /// This API operates on owned arrays for keys, nonces and tags:
    /// ```rust
    /// // Using the multiplexed implementation.
    /// use libcrux_aes::{
    ///     aes_ccm_256::short_tag::{AesCcm256ShortTag, Key, Nonce, Tag},
    ///     AeadConsts as _, CCM_SHORT_TAG_LEN, NONCE_LEN,
    /// };
    ///
    /// let k: Key = [0; AesCcm256ShortTag::KEY_LEN].into();
    /// let nonce: Nonce = [0; NONCE_LEN].into();
    /// let mut tag: Tag = [0; CCM_SHORT_TAG_LEN].into();
    ///
    /// let pt = b"the quick brown fox jumps over the lazy dog";
    /// let mut ct = [0; 43];
    /// let mut pt_out = [0; 43];
    ///
    /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
    /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
    /// assert_eq!(pt, &pt_out);
    /// ```
    ///
    /// ## Refs key-centric API
    /// This API operates on array references for keys, nonces and tags:
    /// ```rust
    /// // Using the multiplexed API
    /// use libcrux_aes::{
    ///     aes_ccm_256::short_tag::AesCcm256ShortTag, Aead as _, AeadConsts as _, CCM_SHORT_TAG_LEN,
    ///     NONCE_LEN,
    /// };
    ///
    /// let algo = AesCcm256ShortTag;
    ///
    /// let mut tag_bytes = [0; CCM_SHORT_TAG_LEN];
    /// let tag = algo.new_tag_mut(&mut tag_bytes).unwrap();
    ///
    /// let key = algo.new_key(&[0; AesCcm256ShortTag::KEY_LEN]).unwrap();
    /// let nonce = algo.new_nonce(&[0; NONCE_LEN]).unwrap();
    ///
    /// let pt = b"the quick brown fox jumps over the lazy dog";
    /// let mut ct = [0; 43];
    /// let mut pt_out = [0; 43];
    ///
    /// key.encrypt(&mut ct, tag, nonce, b"", pt).unwrap();
    /// let tag = algo.new_tag(&tag_bytes).unwrap();
    /// key.decrypt(&mut pt_out, nonce, b"", &ct, tag).unwrap();
    /// assert_eq!(pt, &pt_out);
    /// ```
    pub mod short_tag {
        use crate::aes_gcm::type_aliases;
        type_aliases!(AesCcm256ShortTag, "AES-CCM 256 (8 octet tag)");

        /// # Portable implementation of AES-CCM 256 (Short Tag)
        ///
        /// To use the portable implementation, `Key`, `Nonce`, and `Tag` types
        /// must be explicitly parameterized by the portable implementation.
        ///
        /// Example:
        /// ```rust
        /// // Using the portable implementation.
        /// use libcrux_aes::{
        ///     aes_ccm_256::short_tag::portable::{Key, Nonce, PortableAesCcm256ShortTag, Tag},
        ///     AeadConsts as _, CCM_SHORT_TAG_LEN, NONCE_LEN,
        /// };
        ///
        /// let k: Key<PortableAesCcm256ShortTag> = [0; PortableAesCcm256ShortTag::KEY_LEN].into();
        /// let nonce: Nonce<PortableAesCcm256ShortTag> = [0; NONCE_LEN].into();
        /// let mut tag: Tag<PortableAesCcm256ShortTag> = [0; CCM_SHORT_TAG_LEN].into();
        ///
        /// let pt = b"the quick brown fox jumps over the lazy dog";
        /// let mut ct = [0; 43];
        /// let mut pt_out = [0; 43];
        ///
        /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
        /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
        /// assert_eq!(pt, &pt_out);
        /// ```
        pub mod portable {
            pub use libcrux_traits::aead::{
                typed_owned::{Key, Nonce, Tag},
                typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef},
            };

            pub use crate::implementations::PortableAesCcm256ShortTag;
        }

        #[cfg(feature = "simd128")]
        /// ARM NEON-optimized implementation of AES-CCM 256 (Short Tag)
        ///
        /// To use the NEON-optimized implementation, `Key`, `Nonce`, and `Tag`
        /// types must be explicitly parameterized by the NEON
        /// implementation.
        ///
        /// Example:
        /// ```rust
        /// // Using the NEON implementation.
        /// use libcrux_aes::{
        ///     aes_ccm_256::short_tag::neon::{Key, NeonAesCcm256ShortTag, Nonce, Tag},
        ///     AeadConsts as _, CCM_SHORT_TAG_LEN, NONCE_LEN,
        /// };
        ///
        /// let k: Key<NeonAesCcm256ShortTag> = [0; NeonAesCcm256ShortTag::KEY_LEN].into();
        /// let nonce: Nonce<NeonAesCcm256ShortTag> = [0; NONCE_LEN].into();
        /// let mut tag: Tag<NeonAesCcm256ShortTag> = [0; CCM_SHORT_TAG_LEN].into();
        ///
        /// let pt = b"the quick brown fox jumps over the lazy dog";
        /// let mut ct = [0; 43];
        /// let mut pt_out = [0; 43];
        ///
        /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
        /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
        /// assert_eq!(pt, &pt_out);
        /// ```
        pub mod neon {
            pub use libcrux_traits::aead::{
                typed_owned::{Key, Nonce, Tag},
                typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef},
            };

            pub use crate::implementations::NeonAesCcm256ShortTag;
        }

        #[cfg(feature = "simd256")]
        /// AES-NI-optimized implementation of AES-CCM 256 (Short Tag)
        ///
        /// To use the AES-NI-optimized implementation, `Key`, `Nonce`, and
        /// `Tag` types must be explicitly parameterized by the AES-NI
        /// implementation.
        ///
        /// Example:
        /// ```rust
        /// // Using the AES-NI implementation.
        /// use libcrux_aes::{
        ///     aes_ccm_256::short_tag::x64::{Key, Nonce, Tag, X64AesCcm256ShortTag},
        ///     AeadConsts as _, CCM_SHORT_TAG_LEN, NONCE_LEN,
        /// };
        ///
        /// let k: Key<X64AesCcm256ShortTag> = [0; X64AesCcm256ShortTag::KEY_LEN].into();
        /// let nonce: Nonce<X64AesCcm256ShortTag> = [0; NONCE_LEN].into();
        /// let mut tag: Tag<X64AesCcm256ShortTag> = [0; CCM_SHORT_TAG_LEN].into();
        ///
        /// let pt = b"the quick brown fox jumps over the lazy dog";
        /// let mut ct = [0; 43];
        /// let mut pt_out = [0; 43];
        ///
        /// k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
        /// k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
        /// assert_eq!(pt, &pt_out);
        /// ```
        pub mod x64 {
            pub use libcrux_traits::aead::{
                typed_owned::{Key, Nonce, Tag},
                typed_refs::{KeyMut, KeyRef, NonceRef, TagMut, TagRef},
            };

            pub use crate::implementations::X64AesCcm256ShortTag;
        }
    }
}

/// Trait for an AES State.
/// Implemented for 128 and 256.
pub(crate) trait State {
    fn init(key: &[u8]) -> Self;
    fn set_nonce(&mut self, nonce: &[u8]);
    fn encrypt(&mut self, aad: &[u8], plaintext: &[u8], ciphertext: &mut [u8], tag: &mut [u8]);
    fn decrypt(
        &mut self,
        aad: &[u8],
        ciphertext: &[u8],
        tag: &[u8],
        plaintext: &mut [u8],
    ) -> Result<(), DecryptError>;
}

pub(crate) mod implementations {

    #[cfg(doc)]
    use super::{aes_gcm_128, aes_gcm_256};

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for
    /// platform-multiplexed AES-GCM 128.
    ///
    /// The implementation used is determined automatically at runtime.
    /// - `x64`
    /// - `neon`
    /// - `portable`
    ///
    /// For more information on usage, see [`aes_gcm_128`].
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AesGcm128;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for portable
    /// AES-GCM 128.
    ///
    /// For more information on usage, see [`aes_gcm_128`].
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct PortableAesGcm128;

    #[cfg(feature = "simd128")]
    #[derive(Clone, Copy, PartialEq, Eq)]
    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for ARM Neon
    /// optimized AES-GCM 128.
    ///
    /// Should only be used directly after performing runtime checks for the
    /// necessary CPU features.
    ///
    /// For more information on usage, see [`aes_gcm_128`].
    pub struct NeonAesGcm128;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for x86_64
    /// AES-NI optimized AES-GCM 128.
    ///
    /// Should only be used directly after performing runtime checks for the
    /// necessary CPU features.
    ///
    /// For more information on usage, see [`aes_gcm_128`].
    #[cfg(feature = "simd256")]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct X64AesGcm128;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for
    /// platform-multiplexed AES-CCM 128.
    ///
    /// The implementation used is determined automatically at runtime.
    /// - `x64`
    /// - `neon`
    /// - `portable`
    ///
    /// For more information on usage, see [`aes_ccm_128`](crate::aes_ccm_128).
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AesCcm128;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for portable
    /// AES-CCM 128.
    ///
    /// For more information on usage, see [`aes_ccm_128`](crate::aes_ccm_128).
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct PortableAesCcm128;

    #[cfg(feature = "simd128")]
    #[derive(Clone, Copy, PartialEq, Eq)]
    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for ARM Neon
    /// optimized AES-CCM 128.
    ///
    /// Should only be used directly after performing runtime checks for the
    /// necessary CPU features.
    ///
    /// For more information on usage, see [`aes_ccm_128`](crate::aes_ccm_128).
    pub struct NeonAesCcm128;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for x86_64
    /// AES-NI optimized AES-CCM 128.
    ///
    /// Should only be used directly after performing runtime checks for the
    /// necessary CPU features.
    ///
    /// For more information on usage, see [`aes_ccm_128`](crate::aes_ccm_128).
    #[cfg(feature = "simd256")]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct X64AesCcm128;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for
    /// platform-multiplexed AES-CCM 128.
    ///
    /// The implementation used is determined automatically at runtime.
    /// - `x64`
    /// - `neon`
    /// - `portable`
    ///
    /// For more information on usage, see
    /// [`aes_ccm_128::short_tag`](crate::aes_ccm_128::short_tag).
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AesCcm128ShortTag;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for portable
    /// AES-CCM 128 (Short Tag).
    ///
    /// For more information on usage, see
    /// [`aes_ccm_128::short_tag`](crate::aes_ccm_128::short_tag).
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct PortableAesCcm128ShortTag;

    #[cfg(feature = "simd128")]
    #[derive(Clone, Copy, PartialEq, Eq)]
    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for ARM Neon
    /// optimized AES-CCM 128 (Short Tag).
    ///
    /// Should only be used directly after performing runtime checks for the
    /// necessary CPU features.
    ///
    /// For more information on usage, see
    /// [`aes_ccm_128::short_tag`](crate::aes_ccm_128::short_tag).
    pub struct NeonAesCcm128ShortTag;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for x86_64
    /// AES-NI optimized AES-CCM 128 (Short Tag).
    ///
    /// Should only be used directly after performing runtime checks for the
    /// necessary CPU features.
    ///
    /// For more information on usage, see
    /// [`aes_ccm_128::short_tag`](crate::aes_ccm_128::short_tag).
    #[cfg(feature = "simd256")]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct X64AesCcm128ShortTag;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for
    /// platform-multiplexed AES-GCM 256.
    ///
    /// The implementation used is determined automatically at runtime.
    /// - `x64`
    /// - `neon`
    /// - `portable`
    ///
    /// For more information on usage, see [`aes_gcm_256`].
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AesGcm256;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for portable
    /// AES-GCM 256.
    ///
    /// For more information on usage, see [`aes_gcm_256`].
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct PortableAesGcm256;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for ARM Neon
    /// optimized AES-GCM 256.
    ///
    /// For more information on usage, see [`aes_gcm_256`].
    #[cfg(feature = "simd128")]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct NeonAesGcm256;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for x86_64
    /// AES-NI optimized AES-GCM 256.
    ///
    /// For more information on usage, see [`aes_gcm_256`].
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[cfg(feature = "simd256")]
    pub struct X64AesGcm256;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for
    /// platform-multiplexed AES-CCM 256.
    ///
    /// The implementation used is determined automatically at runtime.
    /// - `x64`
    /// - `neon`
    /// - `portable`
    ///
    /// For more information on usage, see [`aes_ccm_256`](crate::aes_ccm_256).
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AesCcm256;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for portable
    /// AES-CCM 256.
    ///
    /// For more information on usage, see [`aes_ccm_256`](crate::aes_ccm_256).
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct PortableAesCcm256;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for ARM Neon
    /// optimized AES-CCM 256.
    ///
    /// For more information on usage, see [`aes_ccm_256`](crate::aes_ccm_256).
    #[cfg(feature = "simd128")]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct NeonAesCcm256;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for x86_64
    /// AES-NI optimized AES-CCM 256.
    ///
    /// For more information on usage, see [`aes_ccm_256`](crate::aes_ccm_256).
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[cfg(feature = "simd256")]
    pub struct X64AesCcm256;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for
    /// platform-multiplexed AES-CCM 256 (Short Tag).
    ///
    /// The implementation used is determined automatically at runtime.
    /// - `x64`
    /// - `neon`
    /// - `portable`
    ///
    /// For more information on usage, see
    /// [`aes_ccm_256::short_tag`](crate::aes_ccm_256::short_tag).
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AesCcm256ShortTag;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for portable
    /// AES-CCM 256 (Short Tag).
    ///
    /// For more information on usage, see
    /// [`aes_ccm_256::short_tag`](crate::aes_ccm_256::short_tag).
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct PortableAesCcm256ShortTag;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for ARM Neon
    /// optimized AES-CCM 256 (Short Tag).
    ///
    /// For more information on usage, see
    /// [`aes_ccm_256::short_tag`](crate::aes_ccm_256::short_tag).
    #[cfg(feature = "simd128")]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct NeonAesCcm256ShortTag;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for x86_64
    /// AES-NI optimized AES-CCM 256 (Short Tag).
    ///
    /// For more information on usage, see
    /// [`aes_ccm_256::short_tag`](crate::aes_ccm_256::short_tag).
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[cfg(feature = "simd256")]
    pub struct X64AesCcm256ShortTag;
}
// pub use implementations::*;

/// Tag length.
pub const TAG_LEN: usize = 16;

/// Reduced tag length for AES-CCM, as per [RFC 6655](https://datatracker.ietf.org/doc/html/rfc6655).
pub const CCM_SHORT_TAG_LEN: usize = 8;

/// Nonce length.
pub const NONCE_LEN: usize = 12;

#[doc(inline)]
pub use aes::AES_128_KEY_LEN;
#[doc(inline)]
pub use aes::AES_256_KEY_LEN;
pub use libcrux_traits::aead::arrayref::{DecryptError, EncryptError, KeyGenError};

/// Generic AES-based AEAD encrypt.
pub(crate) fn encrypt<S: State>(
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    plaintext: &[u8],
    ciphertext: &mut [u8],
    tag: &mut [u8],
) -> Result<(), EncryptError> {
    // This should only be reachable via the arrayref trait API which
    // checks the lengths.

    let mut st = S::init(key);
    st.set_nonce(nonce);
    st.encrypt(aad, plaintext, ciphertext, tag);

    Ok(())
}

/// Generic AES-based AEAD decrypt.
pub(crate) fn decrypt<S: State>(
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    ciphertext: &[u8],
    tag: &[u8],
    plaintext: &mut [u8],
) -> Result<(), DecryptError> {
    // This should only be reachable via the arrayref trait API which
    // checks the lengths.

    let mut st = S::init(key);
    st.set_nonce(nonce);
    st.decrypt(aad, ciphertext, tag, plaintext)
}

/// Macro to instantiate the different variants, both 128/256 and platforms.
macro_rules! pub_crate_mod {
    ($mod_name:ident, $key_len:literal, $state:ty, $variant_comment:literal) => {
        #[doc = $variant_comment]
        pub mod $mod_name {
            use crate::{platform, DecryptError, EncryptError};

            type State = $state;

            #[doc = $variant_comment]
            /// encrypt.
            pub fn encrypt(
                key: &[u8],
                nonce: &[u8],
                aad: &[u8],
                plaintext: &[u8],
                ciphertext: &mut [u8],
                tag: &mut [u8],
            ) -> Result<(), EncryptError> {
                debug_assert!(key.len() == $key_len);
                crate::encrypt::<State>(key, nonce, aad, plaintext, ciphertext, tag)
            }

            #[doc = $variant_comment]
            /// decrypt.
            pub fn decrypt(
                key: &[u8],
                nonce: &[u8],
                aad: &[u8],
                ciphertext: &[u8],
                tag: &[u8],
                plaintext: &mut [u8],
            ) -> Result<(), DecryptError> {
                debug_assert!(key.len() == $key_len);
                crate::decrypt::<State>(key, nonce, aad, ciphertext, tag, plaintext)
            }
        }
    };
}

pub(crate) mod portable {
    pub_crate_mod!(aes_gcm_128, 16, crate::aes_gcm_128::State<platform::portable::State, platform::portable::FieldElement>, r"AES-GCM 128 ");
    pub_crate_mod!(aes_gcm_256, 32, crate::aes_gcm_256::State<platform::portable::State, platform::portable::FieldElement>, r"AES-GCM 256 ");
    pub_crate_mod!(
        aes_ccm_128,
        16,
        crate::aes_ccm::AesCcm128State<platform::portable::State>,
        r"AES-CCM 128 "
    );
    pub_crate_mod!(
        aes_ccm_128_8,
        16,
        crate::aes_ccm::AesCcm128_8_State<platform::portable::State>,
        r"AES-CCM 128 (8-octet tag) "
    );

    pub_crate_mod!(
        aes_ccm_256,
        32,
        crate::aes_ccm::AesCcm256State<platform::portable::State>,
        r"AES-CCM 256 "
    );

    pub_crate_mod!(
        aes_ccm_256_8,
        32,
        crate::aes_ccm::AesCcm256_8_State<platform::portable::State>,
        r"AES-CCM 256 (8-octet tag) "
    );
}

#[cfg(feature = "simd128")]
pub(crate) mod neon {
    pub_crate_mod!(aes_gcm_128, 16, crate::aes_gcm_128::State<platform::neon::State, platform::neon::FieldElement>, r"AES-GCM 128 ");
    pub_crate_mod!(aes_gcm_256, 32, crate::aes_gcm_256::State<platform::neon::State, platform::neon::FieldElement>, r"AES-GCM 256 ");
    pub_crate_mod!(
        aes_ccm_128,
        16,
        crate::aes_ccm::AesCcm128State<platform::neon::State>,
        r"AES-CCM 128 "
    );
    pub_crate_mod!(
        aes_ccm_128_8,
        16,
        crate::aes_ccm::AesCcm128_8_State<platform::neon::State>,
        r"AES-CCM 128 (8-octet tag) "
    );

    pub_crate_mod!(
        aes_ccm_256,
        32,
        crate::aes_ccm::AesCcm256State<platform::neon::State>,
        r"AES-CCM 256 "
    );

    pub_crate_mod!(
        aes_ccm_256_8,
        32,
        crate::aes_ccm::AesCcm256_8_State<platform::neon::State>,
        r"AES-CCM 256 (8-octet tag) "
    );
}

#[cfg(feature = "simd256")]
pub(crate) mod x64 {
    // Here we don't use the `pub_crate_mod` macro because we need to add target
    // features onto the functions.
    macro_rules! x64_pub_crate_mod {
        ($variant_comment:literal, $mod_name:ident, $state:ty, $key_len:literal) => {
            #[doc = $variant_comment]
            pub mod $mod_name {
                use crate::{platform, DecryptError, EncryptError};

                type State = $state;

                #[doc = $variant_comment]
                /// encrypt.
                pub fn encrypt(
                    key: &[u8],
                    nonce: &[u8],
                    aad: &[u8],
                    plaintext: &[u8],
                    ciphertext: &mut [u8],
                    tag: &mut [u8],
                ) -> Result<(), EncryptError> {
                    debug_assert!(key.len() == $key_len);

                    // due to use of `target_feature`, unsafe is needed here
                    #[inline]
                    #[cfg_attr(not(hax), target_feature(enable = "avx2", enable = "aes"))]
                    #[allow(unsafe_code)]
                    unsafe fn inner(
                        key: &[u8],
                        nonce: &[u8],
                        aad: &[u8],
                        plaintext: &[u8],
                        ciphertext: &mut [u8],
                        tag: &mut [u8],
                    ) -> Result<(), EncryptError> {
                        crate::encrypt::<State>(key, nonce, aad, plaintext, ciphertext, tag)
                    }

                    #[allow(unsafe_code)]
                    unsafe {
                        inner(key, nonce, aad, plaintext, ciphertext, tag)
                    }
                }

                #[doc = $variant_comment]
                /// decrypt.
                pub fn decrypt(
                    key: &[u8],
                    nonce: &[u8],
                    aad: &[u8],
                    ciphertext: &[u8],
                    tag: &[u8],
                    plaintext: &mut [u8],
                ) -> Result<(), DecryptError> {
                    debug_assert!(key.len() == $key_len);

                    // due to use of `target_feature`, unsafe is needed here
                    #[inline]
                    #[cfg_attr(not(hax), target_feature(enable = "avx2", enable = "aes"))]
                    #[allow(unsafe_code)]
                    unsafe fn inner(
                        key: &[u8],
                        nonce: &[u8],
                        aad: &[u8],
                        ciphertext: &[u8],
                        tag: &[u8],
                        plaintext: &mut [u8],
                    ) -> Result<(), DecryptError> {
                        crate::decrypt::<State>(key, nonce, aad, ciphertext, tag, plaintext)
                    }

                    #[allow(unsafe_code)]
                    unsafe {
                        inner(key, nonce, aad, ciphertext, tag, plaintext)
                    }
                }
            }
        };
    }

    x64_pub_crate_mod!(r"AES-GCM 128 ", aes_gcm_128, crate::aes_gcm_128::State<platform::x64::State, platform::x64::FieldElement>, 16);

    x64_pub_crate_mod!(r"AES-GCM 256 ", aes_gcm_256, crate::aes_gcm_256::State<platform::x64::State, platform::x64::FieldElement>, 32);
    x64_pub_crate_mod!(
        r"AES-CCM 128 ",
        aes_ccm_128,
        crate::aes_ccm::AesCcm128State<platform::x64::State>,
        16
    );
    x64_pub_crate_mod!(
        r"AES-CCM 128 (8-octet tag) ",
        aes_ccm_128_8,
        crate::aes_ccm::AesCcm128_8_State<platform::x64::State>,
        16
    );
    x64_pub_crate_mod!(
        r"AES-CCM 256 ",
        aes_ccm_256,
        crate::aes_ccm::AesCcm256State<platform::x64::State>,
        32
    );
    x64_pub_crate_mod!(
        r"AES-CCM 256 (8-octet tag) ",
        aes_ccm_256_8,
        crate::aes_ccm::AesCcm256_8_State<platform::x64::State>,
        32
    );
}

// traits re-exports
#[doc(inline)]
pub use aes_ccm_128::Key as AesCcm128Key;
#[doc(inline)]
pub use aes_ccm_128::Nonce as AesCcm128Nonce;
#[doc(inline)]
pub use aes_ccm_128::Tag as AesCcm128Tag;
#[doc(inline)]
pub use aes_ccm_256::Key as AesCcm256Key;
#[doc(inline)]
pub use aes_ccm_256::Nonce as AesCcm256Nonce;
#[doc(inline)]
pub use aes_ccm_256::Tag as AesCcm256Tag;
#[doc(inline)]
pub use aes_gcm_128::Key as AesGcm128Key;
#[doc(inline)]
pub use aes_gcm_128::Nonce as AesGcm128Nonce;
#[doc(inline)]
pub use aes_gcm_128::Tag as AesGcm128Tag;
#[doc(inline)]
pub use aes_gcm_256::Key as AesGcm256Key;
#[doc(inline)]
pub use aes_gcm_256::Nonce as AesGcm256Nonce;
#[doc(inline)]
pub use aes_gcm_256::Tag as AesGcm256Tag;
pub use implementations::{AesCcm128, AesCcm256, AesGcm128, AesGcm256};
pub use libcrux_traits::aead::{consts::AeadConsts, typed_refs::Aead};

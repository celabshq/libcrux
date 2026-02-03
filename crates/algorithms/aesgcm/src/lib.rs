//! # AES-GCM
//!
//! This crate implements AES-GCM-128 and AES-GCM-256. The crate provides
//! optimized implementations for ARM and x86_64 platforms with support
//! for AES hardware acceleration, as well as a bit-sliced portable
//! implementation.
//!
//! For general use, we provide a platform-multiplexing API via the
//! [`AesGcm128Key`] and [`AesGcm256Key`] structs, which selects the most
//! performant implementation at runtime.
//!
//! Usage example:
//!
//! ```rust
//! // Multiplexed owned API
//! use libcrux_aesgcm::AeadConsts as _;
//! use libcrux_aesgcm::{AesGcm128, AesGcm128Key, AesGcm128Nonce, AesGcm128Tag, NONCE_LEN, TAG_LEN};
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
//! implementation with the [`AesGcm128`] and [`AesGcm256`] structs.
//!
//! Users who want to use a platform-specific implementation directly can
//! access them in the submodules `aes_gcm_128::{portable, x64, neon}`.
//!
//!

// #![no_std]
#![deny(unsafe_code)]
// #[cfg(feature = "std")]
// extern crate std;

mod aes;
// mod ccm;
mod ctr;
mod gf128;
mod platform;

mod traits_api;

mod aes_ccm;
pub mod aes_ccm_128;
mod aes_gcm;

/// Implementations of AES-GCM 128
///
/// This module contains implementations of AES-GCM 128:
/// - [`AesGcm128`]: A platform-multiplexing implementation, which will at
/// runtime select the most performant implementation among the following for the given
/// architecture at runtime.
#[cfg_attr(
    feature = "simd256",
    doc = "- [`aes_gcm_128::x64::X64AesGcm128`]: An implementation optimized for x86_64 AES-NI instruction sets."
)]
#[cfg_attr(
    feature = "simd128",
    doc = "- [`aes_gcm_128::neon::NeonAesGcm128`]: An implementation optimized for ARM NEON instruction sets."
)]
/// - [`aes_gcm_128::portable::PortableAesGcm128`]: A portable, bit-sliced implementation.
///
/// See [`EncryptError`],
/// [`DecryptError`](libcrux_traits::aead::arrayref::DecryptError) and
/// [`KeyGenError`](libcrux_traits::aead::arrayref::DecryptError) for
/// errors.
///
/// The [`libcrux_traits`](libcrux_traits) crate provides two typed APIs for AEADs:
///
/// ## Owned key-centric API
/// This API operates on owned arrays for keys, nonces and tags:
/// ```rust
/// // Using the multiplexed implementation.
/// use libcrux_aesgcm::AeadConsts as _;
/// use libcrux_aesgcm::{NONCE_LEN, TAG_LEN, aes_gcm_128::{AesGcm128, Key, Tag, Nonce}};
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
/// use libcrux_aesgcm::{AeadConsts as _, Aead as _};
/// use libcrux_aesgcm::{NONCE_LEN, TAG_LEN, aes_gcm_128::{AesGcm128}};
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
/// runtime select the most performant implementation among the following for the given
/// architecture at runtime.
#[cfg_attr(
    feature = "simd256",
    doc = "- [`aes_gcm_256::x64::X64AesGcm256`]: An implementation optimized for x86_64 AES-NI instruction sets."
)]
#[cfg_attr(
    feature = "simd128",
    doc = "- [`aes_gcm_256::neon::NeonAesGcm256`]: An implementation optimized for ARM NEON instruction sets."
)]
/// - [`aes_gcm_256::portable::PortableAesGcm256`]: A portable, bit-sliced implementation.
///
/// See [`EncryptError`],
/// [`DecryptError`](libcrux_traits::aead::arrayref::DecryptError) and
/// [`KeyGenError`](libcrux_traits::aead::arrayref::DecryptError) for
/// errors.
///
/// The [`libcrux_traits`](libcrux_traits) crate provides two typed APIs for AEADs:
///
/// ## Owned key-centric API
/// This API operates on owned arrays for keys, nonces and tags:
/// ```rust
/// // Using the multiplexed implementation.
/// use libcrux_aesgcm::AeadConsts as _;
/// use libcrux_aesgcm::{NONCE_LEN, TAG_LEN, aes_gcm_256::{AesGcm256, Key, Tag, Nonce}};
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
/// use libcrux_aesgcm::{AeadConsts as _, Aead as _};
/// use libcrux_aesgcm::{NONCE_LEN, TAG_LEN, aes_gcm_256::{AesGcm256}};
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

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for platform-multiplexed AES-GCM 128.
    ///
    /// The implementation used is determined automatically at runtime.
    /// - `x64`
    /// - `neon`
    /// - `portable`
    ///
    /// For more information on usage, see [`aes_gcm_128`].
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AesGcm128;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for portable AES-GCM 128.
    ///
    /// For more information on usage, see [`aes_gcm_128`].
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct PortableAesGcm128;

    #[cfg(feature = "simd128")]
    #[derive(Clone, Copy, PartialEq, Eq)]
    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for ARM Neon optimized AES-GCM 128.
    ///
    /// Should only be used directly after performing runtime checks for the necessary CPU
    /// features.
    ///
    /// For more information on usage, see [`aes_gcm_128`].
    pub struct NeonAesGcm128;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for x86_64 AES-NI optimized AES-GCM 128.
    ///
    /// Should only be used directly after performing runtime checks for the necessary CPU
    /// features.
    ///
    /// For more information on usage, see [`aes_gcm_128`].
    #[cfg(feature = "simd256")]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct X64AesGcm128;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for platform-multiplexed AES-GCM 256.
    ///
    /// The implementation used is determined automatically at runtime.
    /// - `x64`
    /// - `neon`
    /// - `portable`
    ///
    /// For more information on usage, see [`aes_gcm_256`].
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AesGcm256;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for portable AES-GCM 256.
    ///
    /// For more information on usage, see [`aes_gcm_256`].
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct PortableAesGcm256;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for ARM Neon optimized AES-GCM 256.
    ///
    /// For more information on usage, see [`aes_gcm_256`].
    #[cfg(feature = "simd128")]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct NeonAesGcm256;

    /// Access to [lower-level AEAD APIs](libcrux_traits::aead) for x86_64 AES-NI optimized AES-GCM 256.
    ///
    /// For more information on usage, see [`aes_gcm_256`].
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[cfg(feature = "simd256")]
    pub struct X64AesGcm256;
}
// pub use implementations::*;

/// Tag length.
pub const TAG_LEN: usize = 16;

/// Nonce length.
pub const NONCE_LEN: usize = 12;

#[doc(inline)]
pub use aes_gcm_128::KEY_LEN as AESGCM128_KEY_LEN;
#[doc(inline)]
pub use aes_gcm_256::KEY_LEN as AESGCM256_KEY_LEN;

pub use libcrux_traits::aead::arrayref::{DecryptError, EncryptError, KeyGenError};

/// Generic AES-GCM encrypt.
pub(crate) fn encrypt<S: State>(
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    plaintext: &[u8],
    ciphertext: &mut [u8],
    tag: &mut [u8],
) -> Result<(), EncryptError> {
    debug_assert!(nonce.len() == NONCE_LEN);
    debug_assert!(tag.len() == TAG_LEN);

    // plaintext length check
    if plaintext.len() / crate::aes::AES_BLOCK_LEN >= (u32::MAX - 1) as usize {
        return Err(EncryptError::PlaintextTooLong);
    }

    // ensure ciphertext and plaintext have same length
    if ciphertext.len() != plaintext.len() {
        return Err(EncryptError::WrongCiphertextLength);
    }

    let mut st = S::init(key);
    st.set_nonce(nonce);
    st.encrypt(aad, plaintext, ciphertext, tag);

    Ok(())
}

/// Generic AES-GCM decrypt.
pub(crate) fn decrypt<S: State>(
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    ciphertext: &[u8],
    tag: &[u8],
    plaintext: &mut [u8],
) -> Result<(), DecryptError> {
    debug_assert!(nonce.len() == NONCE_LEN);
    debug_assert!(tag.len() == TAG_LEN);

    // plaintext length check
    if plaintext.len() / crate::aes::AES_BLOCK_LEN >= (u32::MAX - 1) as usize {
        return Err(DecryptError::PlaintextTooLong);
    }

    // ensure ciphertext and plaintext have same length
    if ciphertext.len() != plaintext.len() {
        return Err(DecryptError::WrongPlaintextLength);
    }

    let mut st = S::init(key);
    st.set_nonce(nonce);
    st.decrypt(aad, ciphertext, tag, plaintext)
}

/// Macro to instantiate the different variants, both 128/256 and platforms.
macro_rules! pub_crate_mod {
    ($variant_comment:literal, $mod_name:ident, $state:ty) => {
        #[doc = $variant_comment]
        pub mod $mod_name {
            use crate::$mod_name::KEY_LEN;
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
                debug_assert!(key.len() == KEY_LEN);
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
                debug_assert!(key.len() == KEY_LEN);
                crate::decrypt::<State>(key, nonce, aad, ciphertext, tag, plaintext)
            }
        }
    };
}

pub mod aes_ccm_128_external {
    pub use crate::portable::aes_ccm_128::*;
}

pub(crate) mod portable {
    pub_crate_mod!(r"AES-GCM 128 ", aes_gcm_128, crate::aes_gcm_128::State<platform::portable::State, platform::portable::FieldElement>);
    pub_crate_mod!(
        r"AES-CCM 128 ",
        aes_ccm_128,
        crate::aes_ccm_128::AesCcm128State<platform::portable::State>
    );
    pub_crate_mod!(r"AES-GCM 256 ", aes_gcm_256, crate::aes_gcm_256::State<platform::portable::State, platform::portable::FieldElement>);
}

#[cfg(feature = "simd128")]
pub(crate) mod neon {
    pub_crate_mod!(r"AES-GCM 128 ", aes_gcm_128, crate::aes_gcm_128::State<platform::neon::State, platform::neon::FieldElement>);
    pub_crate_mod!(r"AES-GCM 256 ", aes_gcm_256, crate::aes_gcm_256::State<platform::neon::State, platform::neon::FieldElement>);
}

#[cfg(feature = "simd256")]
pub(crate) mod x64 {
    // Here we don't use the `pub_crate_mod` macro because we need to add target features
    // onto the functions.
    macro_rules! x64_pub_crate_mod {
        ($variant_comment:literal, $mod_name:ident, $state:ty) => {
            #[doc = $variant_comment]
            pub mod $mod_name {
                use crate::$mod_name::KEY_LEN;
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
                    debug_assert!(key.len() == KEY_LEN);

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
                    debug_assert!(key.len() == KEY_LEN);

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

    x64_pub_crate_mod!(r"AES-GCM 128 ", aes_gcm_128, crate::aes_gcm_128::State<platform::x64::State, platform::x64::FieldElement>);
    x64_pub_crate_mod!(r"AES-GCM 256 ", aes_gcm_256, crate::aes_gcm_256::State<platform::x64::State, platform::x64::FieldElement>);
}

// traits re-exports
pub use libcrux_traits::aead::consts::AeadConsts;
pub use libcrux_traits::aead::typed_refs::Aead;

pub use implementations::{AesGcm128, AesGcm256};

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

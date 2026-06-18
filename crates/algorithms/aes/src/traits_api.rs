use crate::{
    implementations::{
        AesCcm128, AesCcm128ShortTag, AesCcm256, AesCcm256ShortTag, AesGcm128, AesGcm256,
        PortableAesCcm128, PortableAesCcm128ShortTag, PortableAesCcm256, PortableAesCcm256ShortTag,
        PortableAesGcm128, PortableAesGcm256,
    },
    NONCE_LEN,
};

use libcrux_traits::aead::{
    arrayref::{self, DecryptError, EncryptError},
    consts, slice, typed_owned,
};

/// Internal error type for length checks
enum LengthError {
    /// The plaintext or ciphertext lengths exceed the AEAD-mode's limit
    PlaintextCiphertextTooLong,
    /// The AAD length exceeds the AEAD-modes's limit
    AadTooLong,
    /// The plaintext and ciphertext buffer lengths disagree
    LengthMismatch,
}

impl From<LengthError> for EncryptError {
    fn from(value: LengthError) -> Self {
        match value {
            LengthError::PlaintextCiphertextTooLong => EncryptError::PlaintextTooLong,
            LengthError::AadTooLong => EncryptError::AadTooLong,
            LengthError::LengthMismatch => EncryptError::WrongCiphertextLength,
        }
    }
}

impl From<LengthError> for DecryptError {
    fn from(value: LengthError) -> Self {
        match value {
            LengthError::PlaintextCiphertextTooLong => DecryptError::PlaintextTooLong,
            LengthError::AadTooLong => DecryptError::AadTooLong,
            LengthError::LengthMismatch => DecryptError::WrongPlaintextLength,
        }
    }
}

/// Macro to implement the libcrux_traits public API traits
///
/// For the blanket impl of `typed_refs::Aead` to take place,
/// the `$type` must implement `Copy` and `PartialEq`.
macro_rules! impl_traits_public_api {
    ($type:ty, $keylen:expr, $taglen:expr, $noncelen:expr ) => {
        // prerequisite for typed_owned::Aead
        impl consts::AeadConsts for $type {
            const KEY_LEN: usize = $keylen;
            const TAG_LEN: usize = $taglen;
            const NONCE_LEN: usize = $noncelen;
        }
        // implement typed_owned::Aead
        typed_owned::impl_aead_typed_owned!($type, $keylen, $taglen, $noncelen);
    };
}

/// Macro to implement the different structs and multiplexing.
macro_rules! api {
    ($mod_name:ident, $variant:ident, $multiplexing:ty, $portable:ident, $neon:ident, $x64:ident, $key_len:path, $tag_len:path, $aad_limit: expr, $ptxt_limit: expr) => {
        mod $mod_name {
            use super::*;
            use libcrux_secrets::U8;

            use libcrux_traits::aead::arrayref::{DecryptError, EncryptError, KeyGenError};

            use $key_len as KEY_LEN;
            use $tag_len as TAG_LEN;

            pub type Key = [u8; KEY_LEN];
            pub type Tag = [u8; TAG_LEN];
            pub type Nonce = [u8; NONCE_LEN];

            /// Check that AAD and plaintext are within AEAD-mode
            /// specific limits, and that plaintext and ciphertext
            /// buffer lengths agree.
            fn length_check(ciphertext: &[u8], plaintext: &[u8], aad: &[u8]) -> Result<(), LengthError> {
                // plaintext length check
                // AES-CTR has an internal bound of
                //
                // (2^32 - 1) * 128,
                //
                // but that is higher than either of the limits for of GCM (2^36 - 32) or
                // CCM (2^24 - 1).
                if plaintext.len() > $ptxt_limit {
                    return Err(LengthError::PlaintextCiphertextTooLong);
                }

                // ensure ciphertext and plaintext have same length
                if ciphertext.len() != plaintext.len() {
                    return Err(LengthError::LengthMismatch);
                }

                // ensure AAD length is within AEAD-mode-specific limit
                if aad.len() > $aad_limit {
                    return Err(LengthError::AadTooLong);
                }

                Ok(())
            }

            mod _libcrux_traits_apis_multiplex {
                use super::*;

                // implement `libcrux_traits` slice trait
                slice::impl_aead_slice_trait!($multiplexing => KEY_LEN, TAG_LEN, NONCE_LEN);

                // implement `libcrux_traits` public API traits
                impl_traits_public_api!($multiplexing, KEY_LEN, TAG_LEN, NONCE_LEN);

                /// The plaintext length must be equal to the ciphertext length.
                impl arrayref::Aead<KEY_LEN, TAG_LEN, NONCE_LEN> for $multiplexing {
                    fn keygen(key: &mut [u8; KEY_LEN], rand: &[u8; KEY_LEN]) -> Result<(), KeyGenError> {
                        *key = *rand;
                        Ok(())
                    }

                    fn encrypt(
                        ciphertext: &mut [u8],
                        tag: &mut Tag,
                        key: &Key,
                        nonce: &Nonce,
                        aad: &[u8],
                        plaintext: &[u8],
                    ) -> Result<(), EncryptError> {
                        length_check(ciphertext, plaintext, aad)?;

                        // SIMD256 needs to come first because SIMD128 is true for
                        // x64 as well, but we don't actually implement it.
                        if libcrux_platform::simd256_support() && libcrux_platform::aes_ni_support() {
                            $x64::encrypt(ciphertext, tag, key, nonce, aad, plaintext)
                        } else if libcrux_platform::simd128_support()
                            && libcrux_platform::aes_ni_support()
                        {
                            $neon::encrypt(ciphertext, tag, key, nonce, aad, plaintext)
                        } else {
                            $portable::encrypt(ciphertext, tag, key, nonce, aad, plaintext)
                        }
                    }

                    fn decrypt(
                        plaintext: &mut [u8],
                        key: &Key,
                        nonce: &Nonce,
                        aad: &[u8],
                        ciphertext: &[u8],
                        tag: &Tag,
                    ) -> Result<(), DecryptError> {
                        length_check(ciphertext, plaintext, aad)?;

                        // SIMD256 needs to come first because SIMD128 is true for
                        // x64 as well, but we don't actually implement it.
                        if libcrux_platform::simd256_support() && libcrux_platform::aes_ni_support() {
                            $x64::decrypt(plaintext, key, nonce, aad, ciphertext, tag)
                        } else if libcrux_platform::simd128_support()
                            && libcrux_platform::aes_ni_support()
                        {
                            $neon::decrypt(plaintext, key, nonce, aad, ciphertext, tag)
                        } else {
                            $portable::decrypt(plaintext, key, nonce, aad, ciphertext, tag)
                        }
                    }
                }
            }

            mod _libcrux_traits_apis_portable {
                use super::*;

                // implement `libcrux_traits` slice trait
                slice::impl_aead_slice_trait!($portable => KEY_LEN, TAG_LEN, NONCE_LEN);

                // implement `libcrux_traits` public API traits
                impl_traits_public_api!($portable, KEY_LEN, TAG_LEN, NONCE_LEN);

                /// The plaintext length must be equal to the ciphertext length.
                impl arrayref::Aead<KEY_LEN, TAG_LEN, NONCE_LEN> for $portable {
                    fn keygen(key: &mut [u8; KEY_LEN], rand: &[u8; KEY_LEN]) -> Result<(), KeyGenError> {
                        *key = *rand;
                        Ok(())
                    }

                    fn encrypt(
                        ciphertext: &mut [u8],
                        tag: &mut Tag,
                        key: &Key,
                        nonce: &Nonce,
                        aad: &[u8],
                        plaintext: &[u8],
                    ) -> Result<(), EncryptError> {
                        length_check(ciphertext, plaintext, aad)?;

                        crate::portable::$variant::encrypt(key, nonce, aad, plaintext, ciphertext, tag)
                    }

                    fn decrypt(
                        plaintext: &mut [u8],
                        key: &Key,
                        nonce: &Nonce,
                        aad: &[u8],
                        ciphertext: &[u8],
                        tag: &Tag,
                    ) -> Result<(), DecryptError> {
                        length_check(ciphertext, plaintext, aad)?;

                        crate::portable::$variant::decrypt(key, nonce, aad, ciphertext, tag, plaintext)
                    }
                }
            }

            #[cfg(feature = "simd128")]
            mod _libcrux_traits_apis_neon {
                use super::*;

                // implement `libcrux_traits` slice trait
                slice::impl_aead_slice_trait!($neon => KEY_LEN, TAG_LEN, NONCE_LEN);

                // implement `libcrux_traits` public API traits
                impl_traits_public_api!($neon, KEY_LEN, TAG_LEN, NONCE_LEN);

                /// The plaintext length must be equal to the ciphertext length.
                impl arrayref::Aead<KEY_LEN, TAG_LEN, NONCE_LEN> for $neon {
                    fn keygen(key: &mut [u8; KEY_LEN], rand: &[u8; KEY_LEN]) -> Result<(), KeyGenError> {
                        *key = *rand;
                        Ok(())
                    }

                    fn encrypt(
                        ciphertext: &mut [u8],
                        tag: &mut Tag,
                        key: &Key,
                        nonce: &Nonce,
                        aad: &[u8],
                        plaintext: &[u8],
                    ) -> Result<(), EncryptError> {
                        length_check(ciphertext, plaintext, aad)?;

                        crate::neon::$variant::encrypt(key, nonce, aad, plaintext, ciphertext, tag)
                    }

                    fn decrypt(
                        plaintext: &mut [u8],
                        key: &Key,
                        nonce: &Nonce,
                        aad: &[u8],
                        ciphertext: &[u8],
                        tag: &Tag,
                    ) -> Result<(), DecryptError> {
                        length_check(ciphertext, plaintext, aad)?;

                        crate::neon::$variant::decrypt(key, nonce, aad, ciphertext, tag, plaintext)
                    }
                }
            }

            #[cfg(feature = "simd256")]
            mod _libcrux_traits_api_x64 {
                use super::*;

                // implement `libcrux_traits` slice trait
                slice::impl_aead_slice_trait!($x64 => KEY_LEN, TAG_LEN, NONCE_LEN);

                // implement `libcrux_traits` public API traits
                impl_traits_public_api!($x64, KEY_LEN, TAG_LEN, NONCE_LEN);

                /// The plaintext length must be equal to the ciphertext length.
                impl arrayref::Aead<KEY_LEN, TAG_LEN, NONCE_LEN> for $x64 {
                    fn keygen(key: &mut [u8; KEY_LEN], rand: &[u8; KEY_LEN]) -> Result<(), KeyGenError> {
                        *key = *rand;
                        Ok(())
                    }

                    fn encrypt(
                        ciphertext: &mut [u8],
                        tag: &mut Tag,
                        key: &Key,
                        nonce: &Nonce,
                        aad: &[u8],
                        plaintext: &[u8],
                    ) -> Result<(), EncryptError> {
                        length_check(ciphertext, plaintext, aad)?;

                        crate::x64::$variant::encrypt(key, nonce, aad, plaintext, ciphertext, tag)
                    }

                    fn decrypt(
                        plaintext: &mut [u8],
                        key: &Key,
                        nonce: &Nonce,
                        aad: &[u8],
                        ciphertext: &[u8],
                        tag: &Tag,
                    ) -> Result<(), DecryptError> {
                        length_check(ciphertext, plaintext, aad)?;

                        crate::x64::$variant::decrypt(key, nonce, aad, ciphertext, tag, plaintext)
                    }
                }
            }
        }
    };
}

macro_rules! cfg {
    ($feature:literal $($it:item)*) => {
        $(
        #[cfg(feature = $feature)]
            $it
        )*
    }
}

macro_rules! not_cfg {
    ($feature:literal $($it:item)*) => {
        $(
        #[cfg(not(feature = $feature))]
            $it
        )*
    }
}

cfg!(
    "simd128"
    use crate::implementations::NeonAesGcm128;
    use crate::implementations::NeonAesGcm256;
    use crate::implementations::NeonAesCcm128;
    use crate::implementations::NeonAesCcm256;
    use crate::implementations::NeonAesCcm256ShortTag;
    use crate::implementations::NeonAesCcm128ShortTag;
);

cfg!(
    "simd256"
    use crate::implementations::X64AesGcm128;
    use crate::implementations::X64AesGcm256;
    use crate::implementations::X64AesCcm128;
    use crate::implementations::X64AesCcm256;
    use crate::implementations::X64AesCcm128ShortTag;
    use crate::implementations::X64AesCcm256ShortTag;
);

// If SIMD implementations are not available, fall back to portable.
not_cfg!(
    "simd128"
    use crate::implementations::PortableAesGcm128 as NeonAesGcm128;
    use crate::implementations::PortableAesGcm256 as NeonAesGcm256;
    use crate::implementations::PortableAesCcm128 as NeonAesCcm128;
    use crate::implementations::PortableAesCcm256 as NeonAesCcm256;
    use crate::implementations::PortableAesCcm128ShortTag as NeonAesCcm128ShortTag;
    use crate::implementations::PortableAesCcm256ShortTag as NeonAesCcm256ShortTag;
);

not_cfg!(
    "simd256"
    use crate::implementations::PortableAesGcm128 as X64AesGcm128;
    use crate::implementations::PortableAesGcm256 as X64AesGcm256;
    use crate::implementations::PortableAesCcm128 as X64AesCcm128;
    use crate::implementations::PortableAesCcm256 as X64AesCcm256;
    use crate::implementations::PortableAesCcm128ShortTag as X64AesCcm128ShortTag;
    use crate::implementations::PortableAesCcm256ShortTag as X64AesCcm256ShortTag;
);

// The following values are taken from RFC 5116.

#[cfg(target_pointer_width = "64")]
/// AAD and plain/ciphertext size limits for 64-bit systems.
mod limits {
    /// AES-GCM allows for AAD to be 2^61 - 1 octets long.
    pub(super) const GCM_AAD_MAX_LEN: usize = (1 << 61) - 1;

    /// AES-GCM allows the plaintext to be 2^36 - 32 octets long. This
    /// is also the maximum length of the ciphertext for us, since we
    /// store the tag separately.
    pub(super) const GCM_PTXT_MAX_LEN: usize = (1 << 36) - 32;

    /// AES-CCM allows for AAD to be of size `usize::MAX - 10`.
    pub(super) const CCM_AAD_MAX_LEN: usize = usize::MAX - 10;

    /// AES-CCM allows the plaintext to be 2^24 - 1 octets long, since
    /// the length has to be encoded in three bytes. This is also the
    /// maximum length of the ciphertext for us, since we store the
    /// tag separately.
    pub(super) const CCM_PTXT_MAX_LEN: usize = (1 << 24) - 1;
}

#[cfg(target_pointer_width = "32")]
/// AAD and plain/ciphertext size limits for 32-bit systems.
mod limits {
    /// AES-GCM allows for AAD to be 2^61 - 1 octets long, but on
    /// 32-bit systems our limit is 2^32 - 1.
    pub(super) const GCM_AAD_MAX_LEN: usize = usize::MAX;

    /// AES-GCM allows the plaintext to be 2^36 - 32 octets long, but
    /// on 32-bit systems our limit is 2^32 - 1.This is also the
    /// maximum length of the ciphertext for us, since we store the
    /// tag separately.
    pub(super) const GCM_PTXT_MAX_LEN: usize = usize::MAX;

    /// AES-CCM allows for AAD to be of size `usize::MAX - 6` octets
    /// on 32-bit systems.
    pub(super) const CCM_AAD_MAX_LEN: usize = usize::MAX - 6;

    /// AES-CCM allows the plaintext to be 2^24 - 1 octets long, since
    /// the length has to be encoded in three bytes. This is also the
    /// maximum length of the ciphertext for us, since we store the
    /// tag separately.
    pub(super) const CCM_PTXT_MAX_LEN: usize = (1 << 24) - 1;
}

api!(
    aes128gcm,
    aes_gcm_128,
    AesGcm128,
    PortableAesGcm128,
    NeonAesGcm128,
    X64AesGcm128,
    crate::aes::AES_128_KEY_LEN,
    crate::TAG_LEN,
    limits::GCM_AAD_MAX_LEN,
    limits::GCM_PTXT_MAX_LEN
);

api!(
    aes256gcm,
    aes_gcm_256,
    AesGcm256,
    PortableAesGcm256,
    NeonAesGcm256,
    X64AesGcm256,
    crate::aes::AES_256_KEY_LEN,
    crate::TAG_LEN,
    limits::GCM_AAD_MAX_LEN,
    limits::GCM_PTXT_MAX_LEN
);

api!(
    aes128ccm,
    aes_ccm_128,
    AesCcm128,
    PortableAesCcm128,
    NeonAesCcm128,
    X64AesCcm128,
    crate::aes::AES_128_KEY_LEN,
    crate::TAG_LEN,
    limits::CCM_AAD_MAX_LEN,
    limits::CCM_PTXT_MAX_LEN
);

api!(
    aes256ccm,
    aes_ccm_256,
    AesCcm256,
    PortableAesCcm256,
    NeonAesCcm256,
    X64AesCcm256,
    crate::aes::AES_256_KEY_LEN,
    crate::TAG_LEN,
    limits::CCM_AAD_MAX_LEN,
    limits::CCM_PTXT_MAX_LEN
);

api!(
    aes128ccm_short_tag,
    aes_ccm_128_8,
    AesCcm128ShortTag,
    PortableAesCcm128ShortTag,
    NeonAesCcm128ShortTag,
    X64AesCcm128ShortTag,
    crate::aes::AES_128_KEY_LEN,
    crate::CCM_SHORT_TAG_LEN,
    limits::CCM_AAD_MAX_LEN,
    limits::CCM_PTXT_MAX_LEN
);

api!(
    aes256ccm_short_tag,
    aes_ccm_256_8,
    AesCcm256ShortTag,
    PortableAesCcm256ShortTag,
    NeonAesCcm256ShortTag,
    X64AesCcm256ShortTag,
    crate::aes::AES_256_KEY_LEN,
    crate::CCM_SHORT_TAG_LEN,
    limits::CCM_AAD_MAX_LEN,
    limits::CCM_PTXT_MAX_LEN
);

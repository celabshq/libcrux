use super::*;
use libcrux_traits::Digest;

/// The different Sha2 algorithms.
#[derive(Clone, Copy, Debug)]
pub enum Algorithm {
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

impl Algorithm {
    // The length of the digest by algorithm.
    pub const fn hash_len(&self) -> usize {
        match self {
            Algorithm::Sha224 => SHA224_LENGTH,
            Algorithm::Sha256 => SHA256_LENGTH,
            Algorithm::Sha384 => SHA384_LENGTH,
            Algorithm::Sha512 => SHA512_LENGTH,
        }
    }
}

impl Algorithm {
    /// Sha2
    ///
    /// Write the Sha2 hash of `payload` into `digest`.
    pub fn hash(&self, payload: &[u8], digest: &mut [u8]) {
        match self {
            Algorithm::Sha224 => Sha224::hash(digest, payload),
            Algorithm::Sha256 => Sha256::hash(digest, payload),
            Algorithm::Sha384 => Sha384::hash(digest, payload),
            Algorithm::Sha512 => Sha512::hash(digest, payload),
        }
    }
}

/// SHA2 224
/// Will panic if `payload` is longer than `u32::MAX` to ensure that hacl-rs can
/// process it.
#[inline(always)]
pub fn sha224(payload: &[u8]) -> [u8; SHA224_LENGTH] {
    let mut digest = [0u8; SHA224_LENGTH];
    Sha224::hash(&mut digest, payload);
    digest
}

/// SHA2 256
/// Will panic if `payload` is longer than `u32::MAX` to ensure that hacl-rs can
/// process it.
#[inline(always)]
pub fn sha256(payload: &[u8]) -> [u8; SHA256_LENGTH] {
    let mut digest = [0u8; SHA256_LENGTH];
    Sha256::hash(&mut digest, payload);
    digest
}

/// SHA2 384
/// Will panic if `payload` is longer than `u32::MAX` to ensure that hacl-rs can
/// process it.
#[inline(always)]
pub fn sha384(payload: &[u8]) -> [u8; SHA384_LENGTH] {
    let mut digest = [0u8; SHA384_LENGTH];
    Sha384::hash(&mut digest, payload);
    digest
}

/// SHA2 512
/// Will panic if `payload` is longer than `u32::MAX` to ensure that hacl-rs can
/// process it.
#[inline(always)]
pub fn sha512(payload: &[u8]) -> [u8; SHA512_LENGTH] {
    let mut digest = [0u8; SHA512_LENGTH];
    Sha512::hash(&mut digest, payload);
    digest
}

// Streaming API - This is the recommended one.
// For implementations based on hacl_rs (over hacl-c)
macro_rules! impl_hash {
    ($name:ident, $digest_size:literal, $state:ty, $malloc:expr, $reset:expr, $update:expr, $finish:expr, $copy:expr, $hash:expr) => {
        #[allow(non_camel_case_types)]
        pub struct $name {
            state: $state,
        }

        impl $name {
            /// Initialize a new digest state for streaming use.
            pub fn new() -> $name {
                $name { state: $malloc() }
            }
        }

        impl libcrux_traits::Digest<$digest_size> for $name {
            /// Return the digest for the given input byte slice, in immediate mode.
            /// Will panic if `payload` is longer than `u32::MAX` to ensure that hacl-rs can
            /// process it.
            #[inline(always)]
            fn hash(digest: &mut [u8], payload: &[u8]) {
                debug_assert!(digest.len() == $digest_size);
                let payload_len = payload.len().try_into().unwrap();
                $hash(digest, payload, payload_len)
            }

            /// Add the `payload` to the digest.
            /// Will panic if `payload` is longer than `u32::MAX` to ensure that hacl-rs can
            /// process it.
            #[inline(always)]
            fn update(&mut self, payload: &[u8]) {
                let payload_len = payload.len().try_into().unwrap();
                $update(&mut self.state, payload, payload_len);
            }

            /// Get the digest.
            ///
            /// Note that the digest state can be continued to be used, to extend the
            /// digest.
            #[inline(always)]
            fn finish(&self, digest: &mut [u8; $digest_size]) {
                $finish(&self.state, digest);
            }

            /// Reset the digest state.
            #[inline(always)]
            fn reset(&mut self) {
                $reset(&mut self.state);
            }
        }

        impl Default for $name {
            #[inline(always)]
            fn default() -> Self {
                Self::new()
            }
        }

        impl Clone for $name {
            #[inline(always)]
            fn clone(&self) -> Self {
                Self {
                    state: $copy(&self.state),
                }
            }
        }
    };
}

impl_hash!(
    Sha256,
    32,
    libcrux_hacl_rs::streaming_types::state_32,
    crate::hacl::malloc_256,
    crate::hacl::reset_256,
    crate::hacl::update_256,
    crate::hacl::digest_256,
    crate::hacl::copy_256,
    crate::hacl::hash_256
);
impl_hash!(
    Sha224,
    28,
    libcrux_hacl_rs::streaming_types::state_32,
    crate::hacl::malloc_224,
    crate::hacl::reset_224,
    crate::hacl::update_224,
    crate::hacl::digest_224,
    crate::hacl::copy_256,
    crate::hacl::hash_224
);

impl_hash!(
    Sha512,
    64,
    libcrux_hacl_rs::streaming_types::state_64,
    crate::hacl::malloc_512,
    crate::hacl::reset_512,
    crate::hacl::update_512,
    crate::hacl::digest_512,
    crate::hacl::copy_512,
    crate::hacl::hash_512
);
impl_hash!(
    Sha384,
    48,
    libcrux_hacl_rs::streaming_types::state_64,
    crate::hacl::malloc_384,
    crate::hacl::reset_384,
    crate::hacl::update_384,
    crate::hacl::digest_384,
    crate::hacl::copy_512,
    crate::hacl::hash_384
);

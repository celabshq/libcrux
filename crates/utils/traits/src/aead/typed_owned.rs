//! This module contains the trait and related errors for an Authenticated
//! Encryption with Associated Data (AEAD) scheme that takes array references
//! as arguments and writes outputs to mutable array references.

use super::{arrayref::*, consts::AeadConsts};
use libcrux_secrets::U8;

// These are the types for the arguments to AEAD. I wonder if it makes sense to do stuff like
// implementing a random constructor for Nonce for all LEN in 24..64 or so
//
// There also is the question whether we want these to be byte-oriented or whether we want to just
// make these associated types. That would mean that we'd have to define them separately for all
// implementations.

#[repr(transparent)]
pub struct Key<Algo: Aead>(Algo::Key);
#[repr(transparent)]
pub struct Tag<Algo: Aead>(Algo::Tag);
#[repr(transparent)]
pub struct Nonce<Algo: Aead>(Algo::Nonce);

/// An Authenticated Encryption with Associated Data (AEAD) scheme. This trait
/// is low-level and is mostly used for implementing other, more usable APIs.
///
/// Some implementors of this trait may impose stronger restrictions on the inputs than described
/// here. Check the documentation of the types implementing this trait to make sure which inputs
/// are valid.
pub trait Aead: Sized + AeadConsts {
    /// The type containing the key data. Usually an array.
    type Key;
    /// The type containing the tag data. Usually an array.
    type Tag;
    /// The type containing the nonce data. Usually an array.
    type Nonce;
    /// The type containing the randomness to generate a new key. Usually an array.
    type Rand;

    /// Generate a new key. Consumes the entire randomnes.
    fn keygen(key: &mut Key<Self>, rand: &Self::Rand) -> Result<(), KeyGenError>;

    /// Encrypt a plaintext message, producing a ciphertext and an authentication tag.
    /// The arguments `plaintext` and `ciphertext` must have the same length.
    fn encrypt(
        ciphertext: &mut [u8],
        tag: &mut Tag<Self>,
        key: &Key<Self>,
        nonce: &Nonce<Self>,
        aad: &[u8],
        plaintext: &[U8],
    ) -> Result<(), EncryptError>;

    /// Decrypt a ciphertext, verifying its authenticity.
    /// The arguments `plaintext` and `ciphertext` must have the same length.
    fn decrypt(
        plaintext: &mut [U8],
        key: &Key<Self>,
        nonce: &Nonce<Self>,
        aad: &[u8],
        ciphertext: &[u8],
        tag: &Tag<Self>,
    ) -> Result<(), DecryptError>;
}

impl<Algo: Aead> Key<Algo> {
    pub fn encrypt(
        &self,
        ciphertext: &mut [u8],
        tag: &mut Tag<Algo>,
        nonce: &Nonce<Algo>,
        aad: &[u8],
        plaintext: &[U8],
    ) -> Result<(), EncryptError> {
        Algo::encrypt(ciphertext, tag, self, nonce, aad, plaintext)
    }

    pub fn decrypt(
        &self,
        plaintext: &mut [U8],
        nonce: &Nonce<Algo>,
        aad: &[u8],
        ciphertext: &[u8],
        tag: &Tag<Algo>,
    ) -> Result<(), DecryptError> {
        Algo::decrypt(plaintext, self, nonce, aad, ciphertext, tag)
    }
}

#[macro_export]
macro_rules! impl_aead_typed_owned {
    ($ty:ty, $keylen:expr, $taglen:expr, $noncelen:expr) => {
        impl $crate::aead::typed_owned::Aead for $ty {
            type Key = [$crate::libcrux_secrets::U8; $keylen];

            type Tag = [$crate::libcrux_secrets::U8; $taglen];

            type Nonce = [$crate::libcrux_secrets::U8; $noncelen];

            type Rand = [$crate::libcrux_secrets::U8; $keylen];

            fn keygen(
                key: &mut $crate::aead::typed_owned::Key<Self>,
                rand: &[U8; $keylen],
            ) -> Result<(), KeyGenError> {
                *key = $crate::aead::typed_owned::Key::<Self>::from(*rand);
                Ok(())
            }

            fn encrypt(
                ciphertext: &mut [u8],
                tag: &mut $crate::aead::typed_owned::Tag<Self>,
                key: &$crate::aead::typed_owned::Key<Self>,
                nonce: &$crate::aead::typed_owned::Nonce<Self>,
                aad: &[u8],
                plaintext: &[$crate::libcrux_secrets::U8],
            ) -> Result<(), EncryptError> {
                <$ty as $crate::aead::arrayref::Aead<$keylen, $taglen, $noncelen>>::encrypt(
                    ciphertext,
                    tag.as_mut(),
                    key.as_ref(),
                    nonce.as_ref(),
                    aad,
                    plaintext,
                )
            }

            fn decrypt(
                plaintext: &mut [$crate::libcrux_secrets::U8],
                key: &$crate::aead::typed_owned::Key<Self>,
                nonce: &$crate::aead::typed_owned::Nonce<Self>,
                aad: &[u8],
                ciphertext: &[u8],
                tag: &$crate::aead::typed_owned::Tag<Self>,
            ) -> Result<(), DecryptError> {
                <$ty as $crate::aead::arrayref::Aead<$keylen, $taglen, $noncelen>>::decrypt(
                    plaintext,
                    key.as_ref(),
                    nonce.as_ref(),
                    aad,
                    ciphertext,
                    tag.as_ref(),
                )
            }
        }
    };
}

pub use impl_aead_typed_owned;

impl<const L: usize, Algo: Aead<Key = [U8; L]>> From<[U8; L]> for Key<Algo> {
    fn from(bytes: Algo::Key) -> Self {
        Self(bytes)
    }
}

impl<const L: usize, Algo: Aead<Key = [U8; L]>> From<&[U8; L]> for &Key<Algo> {
    fn from(bytes: &Algo::Key) -> Self {
        unsafe { core::mem::transmute(bytes) }
    }
}

impl<Algo: Aead> AsRef<Algo::Key> for Key<Algo> {
    fn as_ref(&self) -> &Algo::Key {
        &self.0
    }
}

impl<Algo: Aead> AsMut<Algo::Key> for Key<Algo> {
    fn as_mut(&mut self) -> &mut Algo::Key {
        &mut self.0
    }
}

impl<const L: usize, Algo: Aead<Tag = [U8; L]>> From<[U8; L]> for Tag<Algo> {
    fn from(bytes: Algo::Tag) -> Self {
        Self(bytes)
    }
}

impl<const L: usize, Algo: Aead<Tag = [U8; L]>> From<&[U8; L]> for &Tag<Algo> {
    fn from(bytes: &Algo::Tag) -> Self {
        unsafe { core::mem::transmute(bytes) }
    }
}

impl<const L: usize, Algo: Aead<Tag = [U8; L]>> From<&mut [U8; L]> for &mut Tag<Algo> {
    fn from(bytes: &mut Algo::Tag) -> Self {
        unsafe { core::mem::transmute(bytes) }
    }
}

impl<Algo: Aead> AsRef<Algo::Tag> for Tag<Algo> {
    fn as_ref(&self) -> &Algo::Tag {
        &self.0
    }
}

impl<Algo: Aead> AsMut<Algo::Tag> for Tag<Algo> {
    fn as_mut(&mut self) -> &mut Algo::Tag {
        &mut self.0
    }
}

impl<const L: usize, Algo: Aead<Nonce = [U8; L]>> From<[U8; L]> for Nonce<Algo> {
    fn from(bytes: Algo::Nonce) -> Self {
        Self(bytes)
    }
}

impl<const L: usize, Algo: Aead<Nonce = [U8; L]>> From<&[U8; L]> for &Nonce<Algo> {
    fn from(bytes: &Algo::Nonce) -> Self {
        unsafe { core::mem::transmute(bytes) }
    }
}

impl<Algo: Aead> AsRef<Algo::Nonce> for Nonce<Algo> {
    fn as_ref(&self) -> &Algo::Nonce {
        &self.0
    }
}

impl<Algo: Aead> AsMut<Algo::Nonce> for Nonce<Algo> {
    fn as_mut(&mut self) -> &mut Algo::Nonce {
        &mut self.0
    }
}

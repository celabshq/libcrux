//! This crate implements the ChaCha20Poly1305 AEAD algorithm, as well as the extended nonce
//! variant XChaCha20Poly1305.
//!
//! To encrypt something using ChaCha20Poly1305, use:
//!
//! ```rust
//! # fn main(){
//! # use libcrux_secrets::{Classify, ClassifyRef, Declassify, DeclassifyRef, U8};
//! # let key_bytes = [0u8.classify(); 32];
//! # const MSG_LEN: usize = 19;
//! #
//! use libcrux_chacha20poly1305::*;
//! use libcrux_traits::aead::typed_owned::Aead as _;
//!
//! let msg: &[U8; MSG_LEN] = b"squeamish ossifrage".classify_ref();
//! let mut ciphertext = [0u8; MSG_LEN];
//! let mut tag = Tag::from([0u8.classify(); TAG_LEN]);
//!
//! let key = Key::from(key_bytes);
//! let nonce = Nonce::from([123u8.classify(); NONCE_LEN]);
//!
//! key.encrypt(&mut ciphertext, &mut tag, &nonce, &[/* no aad */], msg)
//!     .expect("Encryption error");
//!
//! // Ciphertext and tag contain encrypted data
//! assert_eq!(
//!     ciphertext,
//!     [ 181, 223,  66, 115, 105, 181,  98, 178,
//!       247, 139, 196, 238, 169, 225, 143,  94,
//!       174, 123, 232 ]
//! );
//! assert_eq!(
//!     tag.as_ref().declassify_ref(),
//!     &[ 155, 112, 155, 212, 133, 38, 145, 115,
//!         27, 221, 245, 237, 125, 28,  22, 101 ]
//! );
//! # }
//! ```
//!
//! and to decrypt, do
//!
//! ```rust
//! # fn main(){
//! # use libcrux_secrets::{Classify, Declassify, DeclassifyRef};
//! # let key_bytes  = [0u8; 32].classify();
//! # let ciphertext = [181, 223,  66, 115, 105, 181,  98, 178, 247, 139, 196, 238, 169, 225, 143,  94, 174, 123, 232];
//! # let tag_bytes  = [155, 112, 155, 212, 133,  38, 145, 115,  27, 221, 245, 237, 125,  28,  22, 101].classify();
//! # const MSG_LEN: usize = 19;
//! #
//! use libcrux_chacha20poly1305::*;
//! use libcrux_traits::aead::typed_owned::Aead as _;
//!
//! let mut plaintext = [0u8.classify(); MSG_LEN];
//! let mut tag = Tag::from(tag_bytes);
//!
//! let key = Key::from(key_bytes);
//! let nonce = Nonce::from([123u8.classify(); NONCE_LEN]);
//!
//! key.decrypt(&mut plaintext, &nonce, &[/* no aad */], &ciphertext, &tag)
//!     .expect("Encryption error");
//!
//! assert_eq!(plaintext.declassify_ref(), b"squeamish ossifrage");
//! # }
//! ```

#![no_std]

use libcrux_traits::aead::{typed_owned, typed_refs};

pub mod xchacha20_poly1305;

mod hacl {
    pub(crate) use libcrux_poly1305::hacl::mac_poly1305;

    pub(crate) mod aead_chacha20poly1305;
    pub(crate) mod chacha20;
}

/// Entry points for the valgrind constant-time tests in `ctgrind-test`.
#[cfg(valgrind_ct_test)]
#[doc(hidden)]
pub mod ct_test {
    /// The tag comparison of [`crate::decrypt`], without the branch on its result.
    #[inline(never)]
    pub fn tag_eq(computed_tag: &[u8; crate::TAG_LEN], tag: &[u8; crate::TAG_LEN]) -> bool {
        crate::hacl::aead_chacha20poly1305::tag_eq(computed_tag, tag)
    }
}

mod impl_aead_trait;
mod impl_hacl;

pub use impl_aead_trait::ChaCha20Poly1305;
pub use impl_hacl::*;

/// The length of ChaCha20-Poly1305 keys.
pub const KEY_LEN: usize = 32;

/// The length of Poly1305 MAC tags.
pub const TAG_LEN: usize = 16;

/// The length of ChaCha20-Poly1305 nonces.
pub const NONCE_LEN: usize = 12;

/// An owned ChaCha20Poly1305 key.
pub type Key = typed_owned::Key<ChaCha20Poly1305>;
/// An owned ChaCha20Poly1305 tag.
pub type Tag = typed_owned::Tag<ChaCha20Poly1305>;
/// An owned ChaCha20Poly1305 nonce.
pub type Nonce = typed_owned::Nonce<ChaCha20Poly1305>;

/// A reference to a ChaCha20Poly1305 key.
pub type KeyRef<'a> = typed_refs::KeyRef<'a, ChaCha20Poly1305>;
/// A reference to a ChaCha20Poly1305 tag.
pub type TagRef<'a> = typed_refs::TagRef<'a, ChaCha20Poly1305>;
/// A mutable reference to a ChaCha20Poly1305 tag.
pub type TagMut<'a> = typed_refs::TagMut<'a, ChaCha20Poly1305>;
/// A reference to a ChaCha20Poly1305 nonce.
pub type NonceRef<'a> = typed_refs::NonceRef<'a, ChaCha20Poly1305>;

/// Describes the error conditions of the  ChaCha20-Poly1305 AEAD.
#[derive(Debug)]
pub enum AeadError {
    /// Indicates that the plaintext argument is too large for the library to handle.
    PlaintextTooLarge,
    /// Indicates that the ciphertext argument is too large for the library to handle.
    CiphertextTooLarge,
    /// Indicates that the associated data argument is too large for the library to handle.
    AadTooLarge,
    /// This indicates that the provided destination ciphertext does not fit the ciphertext and tag.
    CiphertextTooShort,
    /// This indicates that the provided destination plaintext is shorter than `ctxt.len() - TAG_LEN`
    /// and thus will not fit the decrypted plaintext
    PlaintextTooShort,
    /// Indicates that the ciphertext is not a valid encryption under the given key and nonce.
    InvalidCiphertext,
}

impl core::fmt::Display for AeadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            AeadError::PlaintextTooLarge => {
                "The plaintext argument is too large for the library to handle"
            }
            AeadError::CiphertextTooLarge => {
                "The ciphertext argument is too large for the library to handle"
            }
            AeadError::AadTooLarge => {
                "The associated data argument is too large for the library to handle"
            }
            AeadError::CiphertextTooShort => {
                "The provided destination ciphertext does not fit the ciphertext and tag"
            }
            AeadError::PlaintextTooShort => {
                "The provided destination plaintext is too short to fit the decrypted plaintext"
            }
            AeadError::InvalidCiphertext => {
                "The ciphertext is not a valid encryption under the given key and nonce."
            }
        };

        f.write_str(msg)
    }
}

/// Describes the error conditions of the Poly1305 MAC.
#[derive(Debug)]
pub enum MacError {
    /// Indicates that the message argument is too large for the library to handle.
    MessageTooLarge,

    /// Indicates that the MAC tag is invalid for that key and message.
    InvalidMacTag,
}

impl core::fmt::Display for MacError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let msg = match self {
            MacError::MessageTooLarge => {
                "The message argument is too large for the library to handle"
            }
            MacError::InvalidMacTag => "The MAC tag is invalid for that key and message",
        };

        f.write_str(msg)
    }
}

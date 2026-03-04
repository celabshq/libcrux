//! This crate provides AEAD (Authenticated Encryption with Associate Data).
//!
//! We currently support four AEAD algorithms:
//!
//! - ChaCha20Poly1305
//! - XChaCha20Poly1305
//! - AES-GCM 128
//! - AES-GCM 256
//!
//! These can either be used directly, using the [`chacha20poly1305`],
//! [`xchacha20poly1305`], [`aesgcm128`] and [`aesgcm256`] submodules,
//! or using the multiplexed API, which allows chosing the used
//! algorithm dynamically at run time.
//!
//! ## Static API
//!
//! For example, to use the static API to encrypt something using ChaCha20Poly1305:
//!
//! ```rust
//! # fn main(){
//! use libcrux_aead::chacha20poly1305::*;
//! use libcrux_traits::aead::typed_owned::Aead as _;
//! use libcrux_secrets::{Classify, ClassifyRef, Declassify, DeclassifyRef, U8};
//! let key_bytes = [0u8; 32].classify();
//! let tag_bytes = [0u8; TAG_LEN].classify();
//! let nonce_bytes = [123u8; NONCE_LEN].classify();
//! # const MSG_LEN: usize = 19;
//! #
//!
//! let msg: &[U8; MSG_LEN] = b"squeamish ossifrage".classify_ref();
//! let mut ciphertext = [0u8; MSG_LEN];
//! let mut tag = Tag::from(tag_bytes);
//!
//! let key = Key::from(key_bytes);
//! let nonce = Nonce::from(nonce_bytes);
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
//! And, for decrypting:
//!
//! ```rust
//! # fn main(){
//! # use libcrux_aead::chacha20poly1305::*;
//! # use libcrux_traits::aead::typed_owned::Aead as _;
//! # use libcrux_secrets::{Classify, ClassifyRef, Declassify, DeclassifyRef};
//! # let nonce_bytes = [123u8; NONCE_LEN].classify();
//! # let key_bytes  = [0u8; 32].classify();
//! # let ciphertext = [ 181, 223,  66, 115, 105, 181,  98, 178, 247, 139, 196, 238, 169, 225, 143,  94, 174, 123, 232 ];
//! # let tag_bytes  = [ 155, 112, 155, 212, 133, 38, 145, 115, 27, 221, 245, 237, 125, 28,  22, 101 ].classify();
//! # const MSG_LEN: usize = 19;
//! #
//!
//! let mut plaintext = [0u8; MSG_LEN].classify();
//! let mut tag = Tag::from(tag_bytes);
//!
//! let key = Key::from(key_bytes);
//! let nonce = Nonce::from(nonce_bytes);
//!
//! key.decrypt(&mut plaintext, &nonce,  &[/* no aad */], &ciphertext, &tag)
//!     .expect("Decryption error");
//!
//! assert_eq!( plaintext.declassify_ref(), b"squeamish ossifrage");
//! # }
//! ```
//!
//! ## Multiplexed API
//!
//! If you need to select the AEAD algorithm at runtime, you can use the multiplexed API. Here, the
//! algorithm is selected through an enum variant, and the methods `new_key`, `new_nonce` etc.
//! check that the lengths match that of the algorithm.
//!
//! ```rust
//! # fn main(){
//! # use libcrux_secrets::{Classify, ClassifyRef, Declassify, DeclassifyRef, U8};
//! # let key_bytes = [0u8; 32].classify();
//! # let nonce_bytes = [123u8; chacha20poly1305::NONCE_LEN].classify();
//! # const MSG_LEN: usize = 19;
//! # extern crate libcrux_traits;
//! #
//! use libcrux_aead::*;
//! use libcrux_traits::aead::typed_refs::Aead as _;
//!
//! let msg: &[U8; MSG_LEN] = b"squeamish ossifrage".classify_ref();
//! let mut ciphertext = [0u8; MSG_LEN];
//! let mut tag_bytes = [0u8; chacha20poly1305::TAG_LEN].classify();
//!
//! let algo = Aead::ChaCha20Poly1305;
//! let key = algo.new_key(&key_bytes)
//!               .expect("key should have correct length");
//! let nonce = algo.new_nonce(&nonce_bytes)
//!                 .expect("nonce should have correct length");
//! let tag = algo.new_tag_mut(&mut tag_bytes)
//!               .expect("tag should have correct length");
//!
//! key.encrypt(&mut ciphertext, tag, nonce, &[/* no aad */], msg)
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
//!     tag_bytes.declassify(),
//!     [ 155, 112, 155, 212, 133, 38, 145, 115,
//!        27, 221, 245, 237, 125, 28,  22, 101 ]
//! );
//! # }
//! ```
//!
//! Decrypting works similar:
//!
//! ```rust
//! # fn main(){
//! # use libcrux_secrets::{Classify, ClassifyRef, Declassify, DeclassifyRef, U8};
//! # let key_bytes = [0u8; 32].classify();
//! # let nonce_bytes = [123u8; chacha20poly1305::NONCE_LEN].classify();
//! # let ciphertext= [ 181, 223,  66, 115, 105, 181,  98, 178, 247, 139, 196, 238, 169, 225, 143,  94, 174, 123, 232 ];
//! # let tag_bytes =  [ 155, 112, 155, 212, 133, 38, 145, 115, 27, 221, 245, 237, 125, 28,  22, 101 ].classify();
//! # const MSG_LEN: usize = 19;
//! # extern crate libcrux_traits;
//! #
//! use libcrux_aead::*;
//! use libcrux_traits::aead::typed_refs::Aead as _;
//!
//! let mut plaintext = [0u8; MSG_LEN].classify();
//!
//! let algo = Aead::ChaCha20Poly1305;
//! let key = algo.new_key(&key_bytes)
//!               .expect("key should have correct length");
//! let nonce = algo.new_nonce(&nonce_bytes)
//!                 .expect("nonce should have correct length");
//! let tag = algo.new_tag(& tag_bytes)
//!               .expect("tag should have correct length");
//!
//! key.decrypt(&mut plaintext, nonce, &[/* no aad */], &ciphertext, tag)
//!     .expect("Decryption error");
//!
//! // Ciphertext and tag contain encrypted data
//! assert_eq!(plaintext.declassify_ref(), b"squeamish ossifrage");
//! # }
//! ```
//!
//!

#![no_std]

mod multiplexed;

pub use multiplexed::*;

#[cfg(feature = "chacha20poly1305")]
pub mod chacha20poly1305 {
    pub use libcrux_chacha20poly1305::{
        ChaCha20Poly1305, Key, KeyRef, Nonce, NonceRef, Tag, TagMut, TagRef, KEY_LEN, NONCE_LEN,
        TAG_LEN,
    };
}

#[cfg(feature = "xchacha20poly1305")]
pub mod xchacha20poly1305 {
    pub use libcrux_chacha20poly1305::xchacha20_poly1305::{
        Key, KeyRef, Nonce, NonceRef, Tag, TagMut, TagRef, XChaCha20Poly1305, KEY_LEN, NONCE_LEN,
        TAG_LEN,
    };
}

#[cfg(feature = "aesgcm128")]
pub mod aesgcm128 {
    pub use libcrux_aes::aes_gcm_128::{KeyRef, NonceRef, TagMut, TagRef, KEY_LEN};
    pub use libcrux_aes::{
        AesGcm128, AesGcm128Key as Key, AesGcm128Nonce as Nonce, AesGcm128Tag as Tag, NONCE_LEN,
        TAG_LEN,
    };
}

#[cfg(feature = "aesgcm256")]
pub mod aesgcm256 {
    pub use libcrux_aes::aes_gcm_256::{KeyRef, NonceRef, TagMut, TagRef, KEY_LEN};
    pub use libcrux_aes::{
        AesGcm256, AesGcm256Key as Key, AesGcm256Nonce as Nonce, AesGcm256Tag as Tag, NONCE_LEN,
        TAG_LEN,
    };
}

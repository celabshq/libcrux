//! This crate provides digest implementations.
//!
//! - Blake2
//! - Sha2
//! - Sha3
//!
//! These can be used directly, using the `blake2`, `sha2` and `sha3` submodules.
//!
//! ## Basic API
//!
//! For example, to hash a payload using Blake2b (with [`Hash`]):
//! ```
//! fn main() {
//!     use libcrux_digest::blake2::*;
//!     use libcrux_digest::Hash as _;
//!     let mut digest = [0; 32];
//!     Blake2b::hash(&mut digest, b"test data").unwrap();
//! }
//!
//! ```

//! Or to hash a payload using Blake2b (with [`HashOwned`]):
//! ```
//! fn main() {
//!     use libcrux_digest::blake2::*;
//!     use libcrux_digest::HashOwned as _;
//!     let digest: [u8; 32] = Blake2b::hash(b"test data").unwrap();
//! }
//!
//! ```
//!
//! ## Incremental digest API
//!
//! To hash a payload using Blake2b:
//! ```
//! fn main() {
//!     use libcrux_digest::blake2::*;
//!     let mut digest = [0; 32];
//!     let mut hasher = Blake2bHasher::new();
//!     hasher.update(b"test data").unwrap();
//!     hasher.finish(&mut digest);
//! }
//!
//! ```

#![no_std]

#[cfg(any(feature = "sha2", feature = "sha3", feature = "blake2"))]
pub use libcrux_traits::digest::{arrayref::Hash, owned::Hash as HashOwned, Hasher};

#[cfg(feature = "blake2")]
pub mod blake2 {
    //! Blake2 digest implementations.
    //!
    //! Usage example for [`Blake2b`] with [`Hash`](crate::Hash):
    //! ```rust
    //! use libcrux_digest::blake2::*;
    //! use libcrux_digest::Hash as _;
    //! let mut digest = [0; 32];
    //! Blake2b::hash(&mut digest, b"test data").unwrap();
    //! ```
    //!
    //! Usage example for [`Blake2bHasher`]:
    //! ```rust
    //! use libcrux_digest::blake2::*;
    //! let mut digest = [0; 32];
    //! let mut hasher = Blake2bHasher::new();
    //! hasher.update(b"test data").unwrap();
    //! hasher.finish(&mut digest);
    //! ```
    //!
    //! The structs in this module are re-exported from [`libcrux_blake2`].
    pub use libcrux_blake2::{
        Blake2bHash as Blake2b, Blake2bHasher, Blake2sHash as Blake2s, Blake2sHasher,
    };
}

#[cfg(feature = "sha2")]
pub mod sha2 {
    //! Sha2 digest implementations.
    //!
    //! Usage example for [`Sha2_224`] with [`Hash`](crate::Hash):
    //! ```rust
    //! use libcrux_digest::sha2::*;
    //! use libcrux_digest::Hash as _;
    //! let mut digest = [0; 28];
    //! Sha2_224::hash(&mut digest, b"test data").unwrap();
    //! ```
    //!
    //! Usage example for [`Sha2_224Hasher`]:
    //! ```rust
    //! use libcrux_digest::sha2::Sha2_224Hasher;
    //! let mut digest = [0; 28];
    //! let mut hasher = Sha2_224Hasher::default();
    //! hasher.update(b"test data").unwrap();
    //! hasher.finish(&mut digest);
    //! ```
    //!
    //! The structs in this module are re-exported from [`libcrux_sha2`].
    pub use libcrux_sha2::{
        Sha224Hash as Sha2_224, Sha224Hasher as Sha2_224Hasher, Sha256Hash as Sha2_256,
        Sha256Hasher as Sha2_256Hasher, Sha384Hash as Sha2_384, Sha384Hasher as Sha2_384Hasher,
        Sha512Hash as Sha2_512, Sha512Hasher as Sha2_512Hasher,
    };
}

#[cfg(feature = "sha3")]
pub mod sha3 {
    //! Sha3 digest implementations.
    //!
    //! Usage example for [`Sha3_224`] with [`Hash`](crate::Hash):
    //! ```rust
    //! use libcrux_digest::sha3::*;
    //! use libcrux_digest::Hash as _;
    //! let mut digest = [0; 28];
    //! Sha3_224::hash(&mut digest, b"test data").unwrap();
    //! ```
    //!
    //! The structs in this module are re-exported from [`libcrux_sha3`].

    pub use libcrux_sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
}

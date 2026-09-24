#![no_std]
mod hacl {
    //! This module contains generated hacl code.

    pub(crate) mod hash_blake2b;
    pub(crate) mod hash_blake2s;
    pub(crate) mod impl_blake2_constants;
}

mod impl_hacl;

mod impl_digest_trait;

pub use impl_digest_trait::{Blake2bHash, Blake2bHasher, Blake2sHash, Blake2sHasher};
pub use impl_hacl::{
    Blake2b, Blake2bBuilder, Blake2s, Blake2sBuilder, Error, LengthBounds, SupportsKeyLen,
    SupportsOutLen,
};

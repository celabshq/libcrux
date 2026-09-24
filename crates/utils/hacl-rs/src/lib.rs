//! This crate contains hacl-generated utility modules for other hacl-generated code.
//! You most likely don't need to import this.
//!
//! hacl-star commit: efbf82f29190e2aecdac8899e4f42c8cb9defc98

#![no_std]

// Utility modules. In the generated hacl-rs, these are individual crates.
pub mod bignum;
pub mod fstar;
pub mod lowstar;
pub mod util;

// Utility modules that were modules of hacl in the generated code
pub mod bignum25519_51;
pub mod curve25519_51;
pub mod streaming_types;

pub mod prelude {
    #[cfg(feature = "alloc")]
    extern crate alloc;

    #[cfg(feature = "alloc")]
    pub use alloc::{boxed::Box, vec, vec::Vec};

    pub use crate::{bignum, fstar, lowstar, streaming_types, util as lib};
}

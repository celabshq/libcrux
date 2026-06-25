//! AVX2 SIMD backend for SHA-3.
//!
//! Module-declaration shim; all bodies live in the submodules:
//! - [`wrappers`] — math wrappers and the `KeccakItem<4>` impl.
//! - [`load`] — `load_block`, `load_last`, and the `Absorb<4>` impl.
//! - [`store`] — `store_block` and the `Squeeze4` impl.

pub(crate) mod load;
pub(crate) mod store;
pub(crate) mod wrappers;

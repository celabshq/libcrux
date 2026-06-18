//! Arm64 (NEON) SIMD backend for SHA-3.
//!
//! Module-declaration shim; all bodies live in the submodules:
//! - [`wrappers`] — math wrappers, the `uint64x2_t` type alias, and
//!   the `KeccakItem<2>` impl.
//! - [`load`] — `load_block`, `load_last`, and the `Absorb<2>` impl.
//! - [`store`] — `store_block` and the `Squeeze2` impl.

pub(crate) mod load;
pub(crate) mod store;
pub(crate) mod wrappers;

// Re-export `uint64x2_t` so callers (e.g. `neon.rs`) can keep
// referencing `crate::simd::arm64::uint64x2_t` exactly as before the
// split.
pub use wrappers::uint64x2_t;

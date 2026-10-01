//! Arm64 (NEON) SIMD backend for SHA-3.

pub(crate) mod load;
pub(crate) mod store;
pub(crate) mod wrappers;

pub use wrappers::uint64x2_t;

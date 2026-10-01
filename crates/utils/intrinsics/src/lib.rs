#![no_std]

#[cfg(feature = "simd128")]
pub mod arm64;

#[cfg(feature = "simd256")]
pub mod avx2;

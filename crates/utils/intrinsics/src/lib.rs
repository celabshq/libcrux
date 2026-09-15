#![no_std]

// Every crate now routes to the REAL intrinsics modules — under both a normal
// `cargo build` (`not(hax)`) and hax extraction (`hax`). During extraction the
// real `arm64`/`avx2` op bodies route through the differentially-tested
// core-models `Libcrux_intrinsics.{Arm64,Avx2}`. The historical `pre_core_models`
// / `pre_core_models_{arm64,avx2}` "bit_vec stub" cfg-gates (which selected the
// hand-written `{arm64,avx2}_extract.rs` modules) have been retired now that
// ml-kem, ml-dsa, sha3, and aes all extract off the real core-models path.
#[cfg(feature = "simd128")]
pub mod arm64;

#[cfg(feature = "simd256")]
pub mod avx2;

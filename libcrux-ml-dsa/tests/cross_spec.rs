//! Cross-comparison tests between the libcrux ML-DSA implementation and the
//! hacspec specification at `specs/ml-dsa/`. Each `Operations`-trait method
//! is exercised against its `hacspec_ml_dsa::*` equivalent.

#![cfg(feature = "cross-spec-tests")]

// Internal helpers and per-area test modules.
mod cross_spec {
    pub mod arithmetic;
    pub mod encoding;
    pub mod helpers;
    pub mod ntt;
    pub mod sampling;
}

#![no_std]
#![deny(unsafe_code)]
#![deny(unused_qualifications)]

#[cfg(feature = "std")]
extern crate std;

mod arithmetic;
mod constants;
mod encoding;
mod hash_functions;
mod helper;
mod matrix;
mod ml_dsa_generic;
mod ntt;
mod polynomial;
mod pre_hash;
mod sample;
mod samplex4;
mod simd;

#[cfg(hax)]
mod specs;

mod types;

// Public interface

pub use types::*;

pub use crate::constants::KEY_GENERATION_RANDOMNESS_SIZE;
pub use crate::constants::SIGNING_RANDOMNESS_SIZE;

/// Internal items re-exported for cross-spec testing only.
/// Gated behind the `cross-spec-tests` feature.
#[cfg(feature = "cross-spec-tests")]
pub mod test_utils {
    pub use crate::constants::Eta;

    /// SIMD per-lane arithmetic trait and the concrete impl types.
    pub mod simd {
        pub use crate::simd::portable::PortableSIMDUnit;
        pub use crate::simd::traits::{
            Operations, COEFFICIENTS_IN_SIMD_UNIT, SIMD_UNITS_IN_RING_ELEMENT,
        };

        #[cfg(feature = "simd256")]
        pub use crate::simd::avx2::AVX2SIMDUnit;
    }
}

#[cfg(feature = "mldsa44")]
pub mod ml_dsa_44;

#[cfg(feature = "mldsa65")]
pub mod ml_dsa_65;

#[cfg(feature = "mldsa87")]
pub mod ml_dsa_87;

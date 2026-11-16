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

#[cfg(feature = "mldsa44")]
pub mod ml_dsa_44;

#[cfg(feature = "mldsa65")]
pub mod ml_dsa_65;

#[cfg(feature = "mldsa87")]
pub mod ml_dsa_87;

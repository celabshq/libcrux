#![no_std]
// The NTT layer-0/1/2 functional-correctness proof annotations attach a dense
// stack of `#[hax_lib::fstar::before(...)]` blocks to ntt_at_layer_0, which
// overflows the default proc-macro recursion limit during `#[_hax::json]`
// expansion.  Compile-time only — does not affect extracted F* or runtime.
#![recursion_limit = "1024"]
#![deny(unsafe_code)]
// `warn` (not `deny`): the crate-wide `proof!` macro forwards to `hax_lib::fstar!`,
// and a newer toolchain misflags that load-bearing qualified path inside the
// `macro_rules!` transcriber as an unnecessary qualification. Denying it fails the
// hax `cargo build` and aborts extraction (ml-kem does not gate on this lint).
#![warn(unused_qualifications)]

#[cfg(feature = "std")]
extern crate std;

// Crate-wide `proof!` annotation marker (see proof_macros.rs). `#[macro_use]`
// makes it visible in every module below without per-module imports.
#[macro_use]
mod proof_macros;

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

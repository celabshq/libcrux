//! # libcrux
//!
//! A high-assurance cryptography library.

#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc as std;

pub mod algorithms;
pub mod primitives;
pub mod protocols;

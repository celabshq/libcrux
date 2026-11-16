//! # Low-level HACL APIs
//!
//! Unsafe API for HACL.
//!
//! |             | x86 | x86-64             | Arm32 | Arm64 | s390x |
//! | ----------- | --- | ------------------ | ----- | ----- | ----- |
//! | Portable C  | ✓   | ✓                  | ✓     | ✓     | ✓     |
//! | simd128     | -   | SSE2, SSE3, SSE4.1 | -     | NEON  | z14   |
//! | simd256     | -   | AVX, AVX2          | -     | -     | -     |

pub(crate) mod curve25519;
pub(crate) mod p256;

/// Unified error type.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    Curve25519(curve25519::Error),
    P256(p256::Error),
}

impl From<curve25519::Error> for Error {
    fn from(val: curve25519::Error) -> Self {
        Error::Curve25519(val)
    }
}

impl From<p256::Error> for Error {
    fn from(val: p256::Error) -> Self {
        Error::P256(val)
    }
}

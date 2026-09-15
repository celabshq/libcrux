//! Byte-oriented wrappers.
//!
//! The Standard's functions take and return bit strings; everything outside
//! takes and returns bytes. Appendix B.1 fixes the correspondence, so this
//! module is nothing but `h2b` in front and `b2h` behind. It is the only
//! place where a byte appears.

use crate::bits::{b2h, h2b_full};
use crate::sha3;

macro_rules! hash_over_bytes {
    ($(#[$meta:meta])* $name:ident, $inner:path, $bytes:expr) => {
        $(#[$meta])*
        pub fn $name(m: &[u8]) -> [u8; $bytes] {
            let digest = b2h(&$inner(&h2b_full(m)));
            let mut out = [0u8; $bytes];
            out.copy_from_slice(&digest);
            out
        }
    };
}

hash_over_bytes!(
    /// `SHA3-224` on byte-aligned input.
    sha3_224, sha3::sha3_224, 28);
hash_over_bytes!(
    /// `SHA3-256` on byte-aligned input.
    sha3_256, sha3::sha3_256, 32);
hash_over_bytes!(
    /// `SHA3-384` on byte-aligned input.
    sha3_384, sha3::sha3_384, 48);
hash_over_bytes!(
    /// `SHA3-512` on byte-aligned input.
    sha3_512, sha3::sha3_512, 64);

/// `SHAKE128` on byte-aligned input, with the output length given in bytes.
pub fn shake128(m: &[u8], out_bytes: usize) -> Vec<u8> {
    b2h(&sha3::shake128(&h2b_full(m), 8 * out_bytes))
}

/// `SHAKE256` on byte-aligned input, with the output length given in bytes.
pub fn shake256(m: &[u8], out_bytes: usize) -> Vec<u8> {
    b2h(&sha3::shake256(&h2b_full(m), 8 * out_bytes))
}

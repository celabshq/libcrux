//! The six SHA-3 functions — FIPS 202, Sec. 6.1 and 6.2.
//!
//! Each is `KECCAK[c]` on the message with a suffix appended: `01` for the
//! four hash functions, `1111` for the XOFs. Only part of the XOF suffix is
//! domain separation. Sec. 6.3 splits it as `M || 11 || 11`: the trailing pair
//! is what `RawSHAKE` appends, and is the domain separation that tells these
//! inputs from those of the hash functions; the leading pair, next to `M`, is
//! what `SHAKE` adds before calling `RawSHAKE`, for compatibility with the
//! Sakura coding scheme.

use crate::bits::{concat, Bit, BitString};
use crate::sponge::keccak_c;

/// The suffix `01` appended to `M` by the four hash functions — Sec. 6.1.
pub const HASH_SUFFIX: [Bit; 2] = [false, true];

/// The suffix `1111` appended to `M` by the two XOFs — Sec. 6.2.
pub const XOF_SUFFIX: [Bit; 4] = [true, true, true, true];

/// `SHA3-224(M) = KECCAK[448](M || 01, 224)`.
pub fn sha3_224(m: &[Bit]) -> BitString {
    keccak_c(448, &concat(m, &HASH_SUFFIX), 224)
}

/// `SHA3-256(M) = KECCAK[512](M || 01, 256)`.
pub fn sha3_256(m: &[Bit]) -> BitString {
    keccak_c(512, &concat(m, &HASH_SUFFIX), 256)
}

/// `SHA3-384(M) = KECCAK[768](M || 01, 384)`.
pub fn sha3_384(m: &[Bit]) -> BitString {
    keccak_c(768, &concat(m, &HASH_SUFFIX), 384)
}

/// `SHA3-512(M) = KECCAK[1024](M || 01, 512)`.
pub fn sha3_512(m: &[Bit]) -> BitString {
    keccak_c(1024, &concat(m, &HASH_SUFFIX), 512)
}

/// `SHAKE128(M, d) = KECCAK[256](M || 1111, d)`.
pub fn shake128(m: &[Bit], d: usize) -> BitString {
    keccak_c(256, &concat(m, &XOF_SUFFIX), d)
}

/// `SHAKE256(M, d) = KECCAK[512](M || 1111, d)`.
pub fn shake256(m: &[Bit], d: usize) -> BitString {
    keccak_c(512, &concat(m, &XOF_SUFFIX), d)
}

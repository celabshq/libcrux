/// Each field element needs floor(log_2(FIELD_MODULUS)) + 1 = 12 bits to represent
pub(crate) const BITS_PER_COEFFICIENT: usize = 12;

/// Coefficients per ring element
pub(crate) const COEFFICIENTS_IN_RING_ELEMENT: usize = 256;

/// Bits required per (uncompressed) ring element
pub(crate) const BITS_PER_RING_ELEMENT: usize = COEFFICIENTS_IN_RING_ELEMENT * 12;

/// Bytes required per (uncompressed) ring element
pub(crate) const BYTES_PER_RING_ELEMENT: usize = BITS_PER_RING_ELEMENT / 8;

/// The size of an ML-KEM shared secret.
pub const SHARED_SECRET_SIZE: usize = 32;

pub(crate) const CPA_PKE_KEY_GENERATION_SEED_SIZE: usize = 32;

// [hax]: hacspec/hacspec-v2#27 stealing error
//        Using these functions causes stealing errors in hax.
// /// Compute serialized length for output size of ByteEncode
// pub(crate) -> usize {
//     OUT_LEN * K
// }

// /// Compute block length for output block size of ByteEncode u (c1)
// pub(crate) -> usize {
//     (COEFFICIENTS_IN_RING_ELEMENT * FACTOR) / 8
// }

// XXX: Eurydice can't handle this.
// digest_size(Algorithm::Sha3_256);
/// SHA3 256 digest size
pub(crate) const H_DIGEST_SIZE: usize = 32;
/// SHA3 512 digest size
pub(crate) const G_DIGEST_SIZE: usize = 64;

/// K * BITS_PER_RING_ELEMENT / 8
///
/// [eurydice] Note that we can't use const generics here because that breaks
///            C extraction with eurydice.
#[cfg_attr(hax, hax_lib::requires(rank <= 4))]
#[cfg_attr(hax, hax_lib::ensures(|result| result == rank * BITS_PER_RING_ELEMENT / 8))]
pub(crate) const fn ranked_bytes_per_ring_element(rank: usize) -> usize {
    rank * BITS_PER_RING_ELEMENT / 8
}

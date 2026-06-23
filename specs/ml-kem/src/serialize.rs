use crate::compress::{compress, decompress};
use crate::ntt::vector_ntt;
use crate::parameters::*;

pub const MAX_BYTES: usize = 16384;

/// Converts a set of bytes in `bytes` into a set of bits.
///
/// This function implements <strong>Algorithm 3</strong> of the NIST FIPS 203
/// standard, which is reproduced below:
///
/// ```plaintext
/// Input: byte array B ∈ 𝔹ˡ.
/// Output: bit array b ∈ {0,1}⁸ˡ.
/// for (i ← 0; i < l; i++)
///     for(j ← 0; j < 8; j++)
///         b[8i + j] ← B[i] mod 2
///         B[i] ← ⌊B[i]/2⌋
///     end for
/// end for
/// return b
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[hax_lib::requires(N < 16384 && N8 == N * 8)]
pub fn bytes_to_bits<const N: usize, const N8: usize>(bytes: &[u8; N]) -> BitVector<N8> {
    hax_lib::debug_assert!(N8 == N * 8);
    createi(|i| (bytes[i / 8] >> (i % 8)) & 1 == 1)
}

/// Converts a bit string `bits` into an array of bytes. This function asserts
/// that `bits.len()` is a multiple of 8.
///
/// This function implements <strong>Algorithm 2</strong> of the NIST FIPS 203
/// standard, which is reproduced below:
///
/// ```plaintext
/// Input: bit array b ∈ {0,1}⁸ˡ.
/// Output: byte array B ∈ 𝔹ˡ.
///
/// B ← (0,...,0)
/// for (i ← 0; i < 8l; i++)
///     B[⌊i/8⌋] ← B[⌊i/8⌋] + b[i]·2^{i} mod 8
/// end for
/// return B
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[hax_lib::requires(N < 16384 && N8 == N * 8)]
pub fn bits_to_bytes<const N: usize, const N8: usize>(bv: &BitVector<N8>) -> [u8; N] {
    hax_lib::debug_assert!(N8 == N * 8);
    let result = createi(|i| {
        bv[8 * i] as u8
            | ((bv[8 * i + 1] as u8) << 1)
            | ((bv[8 * i + 2] as u8) << 2)
            | ((bv[8 * i + 3] as u8) << 3)
            | ((bv[8 * i + 4] as u8) << 4)
            | ((bv[8 * i + 5] as u8) << 5)
            | ((bv[8 * i + 6] as u8) << 6)
            | ((bv[8 * i + 7] as u8) << 7)
    });
    result
}

/// Convert the associated ring element into a vector of
/// `COEFFICIENTS_IN_RING_ELEMENT * bits_per_coefficient`
/// bits, and output this vector as a byte array such that the first 8 bits of
/// the vector represent the first byte of the output, the next 8 bits the
/// next byte of the output, and so on ...
///
/// N.B.: `byte_encode` is only the inverse of `byte_decode` when:
///
/// - each ring coefficient can fit into `bits_per_coefficient` (otherwise
///   lossy compression takes place)
/// - `bits_per_coefficient < BITS_PER_COEFFICIENT`, since
///   otherwise when `byte_decode` operates on 12 bits at a time,
///   it is not injective: the values 3329 + 1 and 1 for example both fit into
///   12 bits and map to the same `KyberFieldElement`
///
/// Otherwise `byte_decode` is not injective and therefore has no left inverse.
///
/// N.B.: This function asserts that `bits_per_coefficient <= 12`
///
/// This function implements <strong>Algorithm 4</strong> of the NIST FIPS 203 standard, which is
/// reproduced below:
///
/// ```plaintext
/// Input: integer array F ∈ ℤₘ²⁵⁶, where m = 2ᵈ if d < 12 and m = q if d = 12.
/// Output: byte array B ∈ 𝔹^{32d}.
/// for(i ← 0; i < 256; i++)
///     a ← F[i]
///     for(j ← 0; j < d; j++)
///         b[i·d + j] ← a mod 2
///         a ← (a − b[i·d + j])/2
///     end for
/// B ← BitsToBytes(b)
/// return B
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.

#[allow(non_upper_case_globals)]
#[hax_lib::requires(N < 16384 && d <= BITS_PER_COEFFICIENT && Nd == N * d)]
pub fn bitvector_from_bounded_ints<const N: usize, const Nd: usize>(
    input: &[u16; N],
    d: usize,
) -> BitVector<Nd> {
    hax_lib::debug_assert!(Nd == N * d);
    createi(|i| (input[i / d] >> (i % d)) & 1u16 == 1)
}

#[hax_lib::requires(d <= BITS_PER_COEFFICIENT && D32 == 32 * d && D256 == 256 * d)]
pub fn byte_encode<const D32: usize, const D256: usize>(p: Polynomial, d: usize) -> [u8; D32] {
    hax_lib::debug_assert!(d <= BITS_PER_COEFFICIENT && D32 == 32 * d && D256 == 256 * d);
    let p_raw: [u16; 256] = createi(|i| p[i].val);
    let bv = bitvector_from_bounded_ints::<256, D256>(&p_raw, d);
    bits_to_bytes(&bv)
}

/// Given a series of bytes representing a ring element in `re_bytes`,
/// first convert them into a vector of bits in little-endian order; i.e.
/// the least significant `bits_per_coefficient` of `re_bytes[0]`
/// are the first set of bits in the bitstream.
///
/// This vector is deserialized into a `Polynomial` structure.
/// The first `bits_per_coefficient` represent the first coefficient of
/// the ring element, the second `bits_per_coefficient` the second coefficient,
/// and so on.
///
/// N.B.: This function asserts that `bits_per_coefficient <= 12`
///
/// This function implements <strong>Algorithm 5</strong> of the NIST FIPS 203
/// standard, which is reproduced below:
///
/// ```plaintext
/// Input: byte array B ∈ 𝔹^{32d}.
/// Output: integer array F ∈ ℤₘ²⁵⁶, where m = 2ᵈ if d < 12 and m = q if d = 12.
///
/// b ← BytesToBits(B)
/// for (i ← 0; i < 256; i++)
///     F[i] ← ∑(j = 0 to d−1) b[i·d + j] · 2ʲ mod m
/// end for
/// return F
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
/// Per-index body of the `createi` in `bitvector_to_bounded_ints`,
/// hoisted to dodge Aeneas issue
/// https://github.com/AeneasVerif/aeneas/issues/924.
#[allow(non_upper_case_globals)]
fn bitvector_to_bounded_ints_at<const Nd: usize>(input: &BitVector<Nd>, d: usize, i: usize) -> u16 {
    let mut coefficient: u16 = 0;
    for j in 0..d {
        // Loop invariant: coefficient holds the value assembled
        // from the lower j bits, so it's strictly less than 2^j.
        // Using addition instead of bit-OR makes the bound
        // discharge tractable for Z3 (the bits are disjoint by
        // construction, so OR == +).
        hax_lib::loop_invariant!(|j: usize| coefficient < (1u16 << j));
        if input[i * d + j] {
            coefficient += 1u16 << j;
        }
    }
    coefficient
}

#[hax_lib::requires(N < 16384 && d <= BITS_PER_COEFFICIENT && Nd == N * d)]
#[hax_lib::ensures(|result|
    hax_lib::forall(|i: usize| hax_lib::implies(i < N, result[i] < (1u16 << d))))]
#[allow(non_upper_case_globals)]
pub fn bitvector_to_bounded_ints<const N: usize, const Nd: usize>(
    input: &BitVector<Nd>,
    d: usize,
) -> [u16; N] {
    hax_lib::debug_assert!(Nd == N * d);
    let result: [u16; N] = createi(|i| bitvector_to_bounded_ints_at::<Nd>(input, d, i));
    hax_lib::debug_assert!(*input == bitvector_from_bounded_ints(&result, d));
    result
}

#[hax_lib::requires(d > 0 && d <= BITS_PER_COEFFICIENT && N < 16384 / d && N < 16384 / 8 && N8 == N * 8 && Nd == N * d && Nd8 == Nd * 8)]
#[hax_lib::ensures(|result|
    hax_lib::forall(|i: usize| hax_lib::implies(i < N8, result[i] < (1u16 << d))))]
#[allow(non_upper_case_globals)]
pub fn byte_decode_generic<const N: usize, const N8: usize, const Nd: usize, const Nd8: usize>(
    b: &[u8; Nd],
    d: usize,
) -> [u16; N8] {
    hax_lib::debug_assert!(
        d <= BITS_PER_COEFFICIENT && N8 == N * 8 && Nd == N * d && Nd8 == Nd * 8
    );
    let bv: [bool; Nd8] = bytes_to_bits::<Nd, Nd8>(&b);
    bitvector_to_bounded_ints(&bv, d)
}

#[hax_lib::requires(d > 0 && d <= BITS_PER_COEFFICIENT && b.len() == 32 * d && D32 == 32 * d && D256 == 256 * d)]
#[hax_lib::ensures(|result| hax_lib::forall(|i: usize| hax_lib::implies(i < 256, result[i].val < (1u16 << d))))]
pub fn byte_decode<const D32: usize, const D256: usize>(b: &[u8; D32], d: usize) -> Polynomial {
    hax_lib::debug_assert!(
        d <= BITS_PER_COEFFICIENT && b.len() == 32 * d && D32 == 32 * d && D256 == 256 * d
    );
    let decoded = byte_decode_generic::<32, 256, D32, D256>(b, d);
    createi(|i| FieldElement::new(decoded[i] % FIELD_MODULUS))
}

/// Encode each polynomial of an NTT vector at 12 bits per coefficient,
/// writing the result into `out`.  Companion to value-returning
/// [`serialize_secret_key`].  Mirrors the existing
/// [`byte_encode_into`] convention: the `_into` form is the canonical
/// primitive; the value-returning form is a thin allocating wrapper.
#[hax_lib::requires(RANK <= 4 && out.len() == RANK * BYTES_PER_RING_ELEMENT)]
pub fn serialize_secret_key_into<const RANK: usize>(vector: &Vector<RANK>, out: &mut [u8]) {
    hax_lib::debug_assert!(out.len() == RANK * BYTES_PER_RING_ELEMENT);
    for i in 0..RANK {
        hax_lib::loop_invariant!(|_i: usize| out.len() == RANK * BYTES_PER_RING_ELEMENT);
        let encoded = byte_encode::<{ 32 * 12 }, { 256 * 12 }>(vector[i], 12);
        out[i * BYTES_PER_RING_ELEMENT..(i + 1) * BYTES_PER_RING_ELEMENT].copy_from_slice(&encoded);
    }
}

#[hax_lib::requires(RANK <= 4 && encoded.len() == RANK * BYTES_PER_RING_ELEMENT)]
pub fn vector_decode_12<const RANK: usize>(encoded: &[u8]) -> Vector<RANK> {
    hax_lib::debug_assert!(encoded.len() == RANK * BYTES_PER_RING_ELEMENT);
    createi(|i| {
        let start = i * BYTES_PER_RING_ELEMENT;
        let chunk: &[u8; 384] = encoded[start..start + 384].try_into().unwrap();
        byte_decode::<{ 32 * 12 }, { 256 * 12 }>(chunk, 12)
    })
}

#[hax_lib::requires((d == 1 || d == 4 || d == 5 || d == 10 || d == 11 || d == 12) && out.len() == 32 * d)]
pub fn byte_encode_into(p: Polynomial, d: usize, out: &mut [u8]) {
    hax_lib::debug_assert!(d <= BITS_PER_COEFFICIENT && out.len() == 32 * d);
    match d {
        1 => out.copy_from_slice(&byte_encode::<32, 256>(p, 1)),
        4 => out.copy_from_slice(&byte_encode::<128, 1024>(p, 4)),
        5 => out.copy_from_slice(&byte_encode::<160, 1280>(p, 5)),
        10 => out.copy_from_slice(&byte_encode::<320, 2560>(p, 10)),
        11 => out.copy_from_slice(&byte_encode::<352, 2816>(p, 11)),
        12 => out.copy_from_slice(&byte_encode::<384, 3072>(p, 12)),
        _ => panic!("unsupported d={}", d),
    }
}

#[hax_lib::requires((d == 1 || d == 4 || d == 5 || d == 10 || d == 11 || d == 12) && b.len() == 32 * d)]
pub fn byte_decode_dyn(b: &[u8], d: usize) -> Polynomial {
    hax_lib::debug_assert!(d <= BITS_PER_COEFFICIENT && b.len() == 32 * d);
    match d {
        1 => byte_decode::<32, 256>(b.try_into().unwrap(), 1),
        4 => byte_decode::<128, 1024>(b.try_into().unwrap(), 4),
        5 => byte_decode::<160, 1280>(b.try_into().unwrap(), 5),
        10 => byte_decode::<320, 2560>(b.try_into().unwrap(), 10),
        11 => byte_decode::<352, 2816>(b.try_into().unwrap(), 11),
        12 => byte_decode::<384, 3072>(b.try_into().unwrap(), 12),
        _ => panic!("unsupported d={}", d),
    }
}

// ── Fused compress+serialize / deserialize+decompress functions ──
// These match the implementation's decomposition in serialize.rs,
// composing existing spec operations.

/// Compress a polynomial to 1 bit per coefficient, then serialize to bytes.
/// Corresponds to `compress_then_serialize_message` in the implementation.
///
/// Used for encoding/decoding the message in K-PKE.
pub fn compress_then_serialize_message(re: Polynomial) -> [u8; 32] {
    byte_encode::<32, 256>(compress(re, 1), 1)
}

/// Deserialize bytes to a polynomial, then decompress from 1 bit per coefficient.
/// Corresponds to `deserialize_then_decompress_message` in the implementation.
pub fn deserialize_then_decompress_message(serialized: &[u8; 32]) -> Polynomial {
    decompress(byte_decode::<32, 256>(serialized, 1), 1)
}

/// Serialize a polynomial with 12-bit coefficients (no compression).
/// Corresponds to `serialize_uncompressed_ring_element` in the implementation.
pub fn serialize_uncompressed_ring_element(re: &Polynomial) -> [u8; BYTES_PER_RING_ELEMENT] {
    byte_encode::<{ 32 * 12 }, { 256 * 12 }>(*re, 12)
}

/// Deserialize bytes to a polynomial with 12-bit coefficients (no decompression).
/// Corresponds to `deserialize_to_uncompressed_ring_element` in the implementation.
pub fn deserialize_to_uncompressed_ring_element(
    serialized: &[u8; BYTES_PER_RING_ELEMENT],
) -> Polynomial {
    byte_decode::<{ 32 * 12 }, { 256 * 12 }>(serialized, 12)
}

/// Compress each polynomial in u to du bits, then serialize, writing
/// into `out`.  Companion to value-returning [`compress_then_serialize_u`].
/// Mirrors the [`byte_encode_into`] / [`serialize_secret_key_into`]
/// convention: the `_into` form is the canonical primitive.
#[hax_lib::requires(
    RANK <= 4
    && (du == 10 || du == 11)
    && out.len() == (RANK * COEFFICIENTS_IN_RING_ELEMENT * du) / 8
)]
pub fn compress_then_serialize_u_into<const RANK: usize>(
    u: &Vector<RANK>,
    du: usize,
    out: &mut [u8],
) {
    let du_poly_size = (COEFFICIENTS_IN_RING_ELEMENT * du) / 8;
    for i in 0..RANK {
        hax_lib::loop_invariant!(
            |_i: usize| out.len() == (RANK * COEFFICIENTS_IN_RING_ELEMENT * du) / 8
        );
        byte_encode_into(
            compress(u[i], du),
            du,
            &mut out[i * du_poly_size..(i + 1) * du_poly_size],
        );
    }
}

/// Compress each polynomial in u to du bits, then serialize.
/// Corresponds to `compress_then_serialize_ring_element_u` in the implementation.
/// Thin allocating wrapper around [`compress_then_serialize_u_into`].
///
/// Note: The implementation dispatches on the compression factor (10 or 11).
/// In the spec we use the generic compress + byte_encode path.
#[hax_lib::requires(RANK <= 4 && (du == 10 || du == 11) && U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * du) / 8)]
pub fn compress_then_serialize_u<const RANK: usize, const U_SIZE: usize>(
    u: &Vector<RANK>,
    du: usize,
) -> [u8; U_SIZE] {
    let mut out = [0u8; U_SIZE];
    compress_then_serialize_u_into::<RANK>(u, du, &mut out);
    out
}

/// Compress v to dv bits, then serialize.
/// Corresponds to `compress_then_serialize_ring_element_v` in the implementation.
#[hax_lib::requires((dv == 4 || dv == 5) && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * dv) / 8)]
pub fn compress_then_serialize_v<const V_SIZE: usize>(v: &Polynomial, dv: usize) -> [u8; V_SIZE] {
    let mut out = [0u8; V_SIZE];
    byte_encode_into(compress(*v, dv), dv, &mut out);
    out
}

/// Deserialize and decompress u from ciphertext bytes.
/// Corresponds to `deserialize_then_decompress_ring_element_u` in the implementation.
#[hax_lib::requires(RANK <= 4 && (du == 10 || du == 11) && ciphertext.len() == (RANK * COEFFICIENTS_IN_RING_ELEMENT * du) / 8)]
pub fn deserialize_then_decompress_u<const RANK: usize>(
    ciphertext: &[u8],
    du: usize,
) -> Vector<RANK> {
    let du_poly_size = (COEFFICIENTS_IN_RING_ELEMENT * du) / 8;
    createi(|i| {
        let start = i * du_poly_size;
        decompress(
            byte_decode_dyn(&ciphertext[start..start + du_poly_size], du),
            du,
        )
    })
}

/// Deserialize, decompress, then NTT-transform u from ciphertext bytes.
/// Composes [`deserialize_then_decompress_u`] with [`vector_ntt`].  Used
/// by `ind_cpa::decrypt` and matches the libcrux-impl
/// `deserialize_then_decompress_u` function (which fuses the NTT into
/// the per-element decompress loop).
#[hax_lib::requires(RANK <= 4 && (du == 10 || du == 11) && ciphertext.len() == (RANK * COEFFICIENTS_IN_RING_ELEMENT * du) / 8)]
pub fn deserialize_then_decompress_u_then_ntt<const RANK: usize>(
    ciphertext: &[u8],
    du: usize,
) -> Vector<RANK> {
    vector_ntt(deserialize_then_decompress_u::<RANK>(ciphertext, du))
}

/// Deserialize and decompress v from ciphertext bytes.
/// Corresponds to `deserialize_then_decompress_ring_element_v` in the implementation.
#[hax_lib::requires((dv == 4 || dv == 5) && serialized.len() == (COEFFICIENTS_IN_RING_ELEMENT * dv) / 8)]
pub fn deserialize_then_decompress_v(serialized: &[u8], dv: usize) -> Polynomial {
    decompress(byte_decode_dyn(serialized, dv), dv)
}

/// Deserialize ring elements from a byte slice, reducing mod q.
/// Corresponds to `deserialize_ring_elements_reduced` in the implementation.
///
/// This is equivalent to `vector_decode_12` but named to match the implementation.
#[hax_lib::requires(RANK <= 4 && encoded.len() == RANK * BYTES_PER_RING_ELEMENT)]
pub fn deserialize_ring_elements_reduced<const RANK: usize>(encoded: &[u8]) -> Vector<RANK> {
    vector_decode_12::<RANK>(encoded)
}

/// Serialize a vector of polynomials with 12-bit coefficients.
/// Corresponds to `serialize_secret_key` / `serialize_vector` in the implementation.
#[hax_lib::requires(RANK <= 4 && T_SIZE == RANK * BYTES_PER_RING_ELEMENT)]
pub fn serialize_secret_key<const RANK: usize, const T_SIZE: usize>(
    vector: &Vector<RANK>,
) -> [u8; T_SIZE] {
    createi(|k| {
        let i = k / BYTES_PER_RING_ELEMENT;
        let j = k % BYTES_PER_RING_ELEMENT;
        let encoded = byte_encode::<{ 32 * 12 }, { 256 * 12 }>(vector[i], 12);
        encoded[j]
    })
}

/// Serialize a public key: encode the NTT vector t̂ concatenated with the seed ρ.
/// Corresponds to `serialize_public_key` in the implementation's `ind_cpa.rs`.
///
/// Layout: bytes [0, RANK*384) hold byte_encode(t_as_ntt[i], 12) per polynomial,
/// bytes [RANK*384, EK_SIZE) hold seed_for_A[0..32].
/// Per-index body of the `createi` in `serialize_public_key`, hoisted
/// to dodge Aeneas issue
/// https://github.com/AeneasVerif/aeneas/issues/924.
#[allow(non_snake_case)]
fn serialize_public_key_at<const RANK: usize>(
    t_as_ntt: &Vector<RANK>,
    seed_for_A: &[u8],
    k: usize,
) -> u8 {
    if k < RANK * BYTES_PER_RING_ELEMENT {
        let i = k / BYTES_PER_RING_ELEMENT;
        let j = k % BYTES_PER_RING_ELEMENT;
        let encoded = byte_encode::<{ 32 * 12 }, { 256 * 12 }>(t_as_ntt[i], 12);
        encoded[j]
    } else {
        seed_for_A[k - RANK * BYTES_PER_RING_ELEMENT]
    }
}

#[allow(non_snake_case)]
#[hax_lib::requires(RANK <= 4 && EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32 && seed_for_A.len() >= 32)]
pub fn serialize_public_key<const RANK: usize, const EK_SIZE: usize>(
    t_as_ntt: &Vector<RANK>,
    seed_for_A: &[u8],
) -> [u8; EK_SIZE] {
    createi(|k| serialize_public_key_at::<RANK>(t_as_ntt, seed_for_A, k))
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::compress::tests::arb_ring_element;
    use crate::parameters::FIELD_MODULUS;

    #[test]
    fn bytes_to_bits_known_vector() {
        // 0xA5 = 10100101 -> bits in LE: [1,0,1,0,0,1,0,1]
        // 0x3C = 00111100 -> bits in LE: [0,0,1,1,1,1,0,0]
        let bytes = [0xA5u8, 0x3C];
        let bits: [bool; 16] = bytes_to_bits(&bytes);
        assert_eq!(
            bits[0..8],
            [true, false, true, false, false, true, false, true]
        );
        assert_eq!(
            bits[8..16],
            [false, false, true, true, true, true, false, false]
        );
    }

    #[test]
    fn bits_to_bytes_known_vector() {
        let mut bits = [false; 16];
        // Encode 0xA5 = LE bits [1,0,1,0,0,1,0,1]
        bits[0] = true;
        bits[2] = true;
        bits[5] = true;
        bits[7] = true;
        // Encode 0x3C = LE bits [0,0,1,1,1,1,0,0]
        bits[10] = true;
        bits[11] = true;
        bits[12] = true;
        bits[13] = true;

        let bytes: [u8; 2] = bits_to_bytes(&bits);
        assert_eq!(bytes, [0xA5, 0x3C]);
    }

    #[test]
    fn bytes_to_bits_all_zeros() {
        let bytes = [0u8; 4];
        let bits: [bool; 32] = bytes_to_bits(&bytes);
        assert!(bits.iter().all(|&b| !b));
    }

    #[test]
    fn bytes_to_bits_all_ones() {
        let bytes = [0xFFu8; 4];
        let bits: [bool; 32] = bytes_to_bits(&bytes);
        assert!(bits.iter().all(|&b| b));
    }

    #[test]
    fn bitvector_from_bounded_ints_known_vector() {
        // d=4, integers [5, 11] -> bits for 5=0101 and 11=1011
        let ints: [u16; 2] = [5, 11];
        let bits: [bool; 8] = bitvector_from_bounded_ints(&ints, 4);
        // 5 in LE bits: [1,0,1,0]
        assert_eq!(bits[0..4], [true, false, true, false]);
        // 11 in LE bits: [1,1,0,1]
        assert_eq!(bits[4..8], [true, true, false, true]);
    }

    #[test]
    fn bitvector_to_bounded_ints_known_vector() {
        // Reverse of above: bits for 5 and 11 with d=4
        let bits = [true, false, true, false, true, true, false, true];
        let ints: [u16; 2] = bitvector_to_bounded_ints(&bits, 4);
        assert_eq!(ints, [5, 11]);
    }

    #[test]
    fn bitvector_roundtrip() {
        // from_bounded -> to_bounded should recover original values
        let ints: [u16; 4] = [0, 7, 15, 3];
        let bits: [bool; 16] = bitvector_from_bounded_ints(&ints, 4);
        let recovered: [u16; 4] = bitvector_to_bounded_ints(&bits, 4);
        assert_eq!(recovered, ints);
    }

    #[test]
    fn byte_encode_decode_roundtrip_d1() {
        // d=1: coefficients are 0 or 1
        let poly: Polynomial = createi(|i| FieldElement::new((i % 2) as u16));
        let encoded: [u8; 32] = byte_encode::<32, 256>(poly, 1);
        let decoded: Polynomial = byte_decode::<32, 256>(&encoded, 1);
        assert_eq!(decoded, poly);
    }

    #[test]
    fn byte_encode_decode_roundtrip_d4() {
        let poly: Polynomial = createi(|i| FieldElement::new((i % 16) as u16));
        let encoded: [u8; 128] = byte_encode::<128, 1024>(poly, 4);
        let decoded: Polynomial = byte_decode::<128, 1024>(&encoded, 4);
        assert_eq!(decoded, poly);
    }

    #[test]
    fn byte_encode_decode_roundtrip_d10() {
        let poly: Polynomial = createi(|i| FieldElement::new((i % 1024) as u16));
        let encoded: [u8; 320] = byte_encode::<320, 2560>(poly, 10);
        let decoded: Polynomial = byte_decode::<320, 2560>(&encoded, 10);
        assert_eq!(decoded, poly);
    }

    #[test]
    fn byte_decode_d12_reduces_mod_q() {
        // Encode a polynomial with values in [0, q-1], decode it, verify reduction
        let poly: Polynomial = createi(|i| FieldElement::new((i as u16 * 13) % FIELD_MODULUS));
        let encoded: [u8; 384] = byte_encode::<384, 3072>(poly, 12);
        let decoded: Polynomial = byte_decode::<384, 3072>(&encoded, 12);
        // All decoded values should be in [0, q-1]
        for (i, coeff) in decoded.iter().enumerate() {
            assert!(
                coeff.val < FIELD_MODULUS,
                "decoded[{}] = {} not in [0, q)",
                i,
                coeff.val
            );
        }
        assert_eq!(decoded, poly);
    }

    #[test]
    fn byte_encode_known_vector_d1() {
        // All zeros
        let poly = [FieldElement::new(0); 256];
        let encoded: [u8; 32] = byte_encode::<32, 256>(poly, 1);
        assert_eq!(encoded, [0u8; 32]);

        // All ones
        let poly = [FieldElement::new(1); 256];
        let encoded: [u8; 32] = byte_encode::<32, 256>(poly, 1);
        assert_eq!(encoded, [0xFFu8; 32]);
    }

    #[test]
    fn byte_decode_known_vector_d1() {
        let bytes = [0u8; 32];
        let decoded: Polynomial = byte_decode::<32, 256>(&bytes, 1);
        assert!(decoded.iter().all(|c| c.val == 0));

        let bytes = [0xFFu8; 32];
        let decoded: Polynomial = byte_decode::<32, 256>(&bytes, 1);
        assert!(decoded.iter().all(|c| c.val == 1));
    }

    proptest! {
        #[test]
        fn bytes_to_bits_and_back_roundtrip(b0 in any::<u8>(), b1 in any::<u8>(), b2 in any::<u8>(), b3 in any::<u8>()) {
            let bytes = [b0, b1, b2, b3];
            let bits: [bool; 32] = bytes_to_bits(&bytes);
            let recovered: [u8; 4] = bits_to_bytes(&bits);
            assert_eq!(recovered, bytes);
        }

        #[test]
        fn deserialize_is_left_inverse_of_serialize_when_no_compression(ring_element in arb_ring_element(12)) {
            let ring_element_serialized: [u8; 32 * 12]= byte_encode::<{32 * 12}, {256 * 12}>(ring_element, 12);
            assert_eq!(ring_element, byte_decode::<{32 * 12}, {256 * 12}>(&ring_element_serialized, 12));
        }

        #[test]
        fn serialize_is_sometimes_left_inverse_of_deserialize_when_no_compression(ring_element in arb_ring_element(12)) {
            let ring_element_serialized: [u8; 32 * 12]= byte_encode::<{32 * 12}, {256 * 12}>(ring_element, 12);
            assert_eq!(ring_element_serialized, byte_encode::<{32 * 12}, {256 * 12}>(byte_decode::<{32 * 12}, {256 * 12}>(&ring_element_serialized, 12), 12));
        }
    }
}

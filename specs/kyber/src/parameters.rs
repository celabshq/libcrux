use hacspec_lib::{field::PrimeFieldElement, ring::PolynomialRingElement, vector::Vector};

/// Field modulus: 3329
pub(crate) const FIELD_MODULUS: u16 = 3329;

/// Each field element needs floor(log_2(FIELD_MODULUS)) + 1 = 12 bits to represent
pub(crate) const BITS_PER_COEFFICIENT: usize = 12;

/// Coefficients per ring element
pub(crate) const COEFFICIENTS_IN_RING_ELEMENT: usize = 256;

/// Bits required per (uncompressed) ring element
pub(crate) const BITS_PER_RING_ELEMENT: usize = COEFFICIENTS_IN_RING_ELEMENT * 12;

/// Bytes required per (uncompressed) ring element
pub(crate) const BYTES_PER_RING_ELEMENT: usize = BITS_PER_RING_ELEMENT / 8;

/// Seed size for rejection sampling.
///
/// See <https://eprint.iacr.org/2023/708> for some background regarding
/// this choice.
pub(crate) const REJECTION_SAMPLING_SEED_SIZE: usize = 168 * 5;

/// Rank
pub(crate) const RANK: usize = 3;

/// `T` NTT encoding size in bytes
pub(crate) const T_AS_NTT_ENCODED_SIZE: usize =
    (RANK * COEFFICIENTS_IN_RING_ELEMENT * BITS_PER_COEFFICIENT) / 8;

/// Compression factor for `U`
pub(crate) const VECTOR_U_COMPRESSION_FACTOR: usize = 10;

/// Compression factor for `V`
pub(crate) const VECTOR_V_COMPRESSION_FACTOR: usize = 4;

/// `U` encoding size in bytes
pub(crate) const VECTOR_U_ENCODED_SIZE: usize =
    (RANK * COEFFICIENTS_IN_RING_ELEMENT * VECTOR_U_COMPRESSION_FACTOR) / 8;

/// `V` encoding size in bytes
pub(crate) const VECTOR_V_ENCODED_SIZE: usize =
    (COEFFICIENTS_IN_RING_ELEMENT * VECTOR_V_COMPRESSION_FACTOR) / 8;

pub(crate) const CPA_PKE_KEY_GENERATION_SEED_SIZE: usize = 32;
pub(crate) const CPA_PKE_SECRET_KEY_SIZE: usize =
    (RANK * COEFFICIENTS_IN_RING_ELEMENT * BITS_PER_COEFFICIENT) / 8;
pub(crate) const CPA_PKE_PUBLIC_KEY_SIZE: usize = T_AS_NTT_ENCODED_SIZE + 32;
pub(crate) const CPA_PKE_CIPHERTEXT_SIZE: usize = VECTOR_U_ENCODED_SIZE + VECTOR_V_ENCODED_SIZE;
pub(crate) const CPA_PKE_MESSAGE_SIZE: usize = 32;
pub(crate) const CPA_SERIALIZED_KEY_LEN: usize = CPA_PKE_SECRET_KEY_SIZE
    + CPA_PKE_PUBLIC_KEY_SIZE
    + hash_functions::H_DIGEST_SIZE
    + CPA_PKE_MESSAGE_SIZE;

#[allow(non_snake_case)]
pub(crate) mod hash_functions {
    use libcrux::algorithms::sha3::{self, digest_size, Algorithm};

    pub(crate) fn G(input: &[u8]) -> [u8; digest_size(Algorithm::Sha512)] {
        sha3::sha512(input)
    }

    pub(crate) const H_DIGEST_SIZE: usize = digest_size(Algorithm::Sha256);
    pub(crate) fn H(input: &[u8]) -> [u8; H_DIGEST_SIZE] {
        sha3::sha256(input)
    }

    pub(crate) fn PRF<const LEN: usize>(input: &[u8]) -> [u8; LEN] {
        sha3::shake256::<LEN>(input)
    }

    pub(crate) fn XOF<const LEN: usize>(input: &[u8]) -> [u8; LEN] {
        sha3::shake128::<LEN>(input)
    }

    pub(crate) fn J<const LEN: usize>(input: &[u8]) -> [u8; LEN] {
        sha3::shake256::<LEN>(input)
    }
}

/// A Kyber field element.
pub(crate) type KyberFieldElement = PrimeFieldElement<FIELD_MODULUS>;

/// A Kyber ring element
pub(crate) type KyberPolynomialRingElement =
    PolynomialRingElement<KyberFieldElement, COEFFICIENTS_IN_RING_ELEMENT>;

/// A Kyber vector
pub(crate) type KyberVector = Vector<KyberFieldElement, COEFFICIENTS_IN_RING_ELEMENT, RANK>;

/// Hash function wrappers — FIPS 204, Section 3.7.
///
/// H(str, ℓ) = SHAKE256(str, 8ℓ)
/// G(str, ℓ) = SHAKE128(str, 8ℓ)

/// H: SHAKE256 of a single input, producing N bytes.
/// Kept opaque: cross-crate call to hacspec_sha3 does not extract.
#[hax_lib::opaque]
pub(crate) fn h<const N: usize>(input: &[u8]) -> [u8; N] {
    hacspec_sha3::shake256(input)
}

/// G: SHAKE128 of a single input, producing N bytes.
/// Kept opaque: cross-crate call to hacspec_sha3 does not extract.
#[hax_lib::opaque]
pub(crate) fn g<const N: usize>(input: &[u8]) -> [u8; N] {
    hacspec_sha3::shake128(input)
}

/// H applied to the concatenation of two byte strings.
/// Kept opaque: variable-length m_prime prevents fixed-size buffer.
#[hax_lib::opaque]
pub(crate) fn h2<const N: usize>(a: &[u8], b: &[u8]) -> [u8; N] {
    let mut input = a.to_vec();
    input.extend_from_slice(b);
    hacspec_sha3::shake256(&input)
}

/// H applied to the concatenation of [u8; 32] ++ [u8; 32] ++ [u8; 64] = 128 bytes.
pub(crate) fn h3<const N: usize>(a: &[u8; 32], b: &[u8; 32], c: &[u8; 64]) -> [u8; N] {
    let mut input = [0u8; 128];
    input[..32].copy_from_slice(a);
    input[32..64].copy_from_slice(b);
    input[64..128].copy_from_slice(c);
    h(&input)
}

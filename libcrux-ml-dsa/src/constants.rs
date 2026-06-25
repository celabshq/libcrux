pub(crate) const FIELD_MODULUS: i32 = 8_380_417;

pub(crate) const COEFFICIENTS_IN_RING_ELEMENT: usize = 256;

pub(crate) const FIELD_MODULUS_MINUS_ONE_BIT_LENGTH: usize = 23;

pub(crate) const BITS_IN_LOWER_PART_OF_T: usize = 13;
pub(crate) const RING_ELEMENT_OF_T0S_SIZE: usize =
    (BITS_IN_LOWER_PART_OF_T * COEFFICIENTS_IN_RING_ELEMENT) / 8;

pub(crate) const BITS_IN_UPPER_PART_OF_T: usize =
    FIELD_MODULUS_MINUS_ONE_BIT_LENGTH - BITS_IN_LOWER_PART_OF_T;
pub(crate) const RING_ELEMENT_OF_T1S_SIZE: usize =
    (BITS_IN_UPPER_PART_OF_T * COEFFICIENTS_IN_RING_ELEMENT) / 8;

pub(crate) const SEED_FOR_A_SIZE: usize = 32;
pub(crate) const SEED_FOR_ERROR_VECTORS_SIZE: usize = 64;
pub(crate) const BYTES_FOR_VERIFICATION_KEY_HASH: usize = 64;
pub(crate) const SEED_FOR_SIGNING_SIZE: usize = 32;

/// Number of bytes of entropy required for key generation.
pub const KEY_GENERATION_RANDOMNESS_SIZE: usize = 32;
/// Number of bytes of entropy required for signing.
pub const SIGNING_RANDOMNESS_SIZE: usize = 32;

pub(crate) const MESSAGE_REPRESENTATIVE_SIZE: usize = 64;
pub(crate) const MASK_SEED_SIZE: usize = 64;

pub(crate) const REJECTION_SAMPLE_BOUND_SIGN: usize = 814;

/// The length of `context` is serialized to a single `u8`.
pub(crate) const CONTEXT_MAX_LEN: usize = 255;

// Handling of enums in eurydice is very limited.
// We therefore don't sue them here in all the places we could.
// See
// - https://github.com/AeneasVerif/eurydice/issues/123
// - https://github.com/AeneasVerif/eurydice/issues/122

/// Eta values
#[derive(Clone, Copy)]
pub enum Eta {
    Two = 2,
    Four = 4,
}

/// Gamma2 values
pub type Gamma2 = i32;
pub(crate) const GAMMA2_V261_888: Gamma2 = 261_888;
pub(crate) const GAMMA2_V95_232: Gamma2 = 95_232;

/// ML-DSA-44-specific parameters
#[cfg(feature = "mldsa44")]
pub(crate) mod ml_dsa_44 {
    use super::Eta;
    use crate::constants::*;

    pub(crate) const ROWS_IN_A: usize = 4;
    pub(crate) const COLUMNS_IN_A: usize = 4;

    pub(crate) const ETA: Eta = Eta::Two;

    // To sample a value in the interval [-ETA, ETA], we can sample a value (say 'v')
    // in the interval [0, 2 * ETA] and then compute ETA - v. This can be done in
    // 3 bits when ETA is 2.
    pub(crate) const BITS_PER_ERROR_COEFFICIENT: usize = 3;

    pub(crate) const GAMMA1_EXPONENT: usize = 17;
    pub(crate) const GAMMA2: i32 = (FIELD_MODULUS - 1) / 88;

    // To sample a value in the interval [-(GAMMA - 1), GAMMA], we can sample a
    // value (say 'v') in the interval [0, (2 * GAMMA) - 1] and then compute
    // GAMMA - v. This can be done in 18 bits when GAMMA is 2^{17}.
    pub(crate) const BITS_PER_GAMMA1_COEFFICIENT: usize = 18;

    pub(crate) const MAX_ONES_IN_HINT: usize = 80;

    pub(crate) const ONES_IN_VERIFIER_CHALLENGE: usize = 39;

    pub(crate) const COMMITMENT_HASH_SIZE: usize = 32;

    // Commitment coefficients are in the interval: [0, ((FIELD_MODULUS − 1)/2γ2) − 1]
    // ((FIELD_MODULUS − 1)/2γ2) − 1 = 43, which means we need 6 bits to represent a
    // coefficient.
    pub(crate) const BITS_PER_COMMITMENT_COEFFICIENT: usize = 6;
}

/// ML-DSA-65-specific parameters
#[cfg(feature = "mldsa65")]
pub(crate) mod ml_dsa_65 {
    use super::Eta;
    use crate::constants::*;

    pub(crate) const ROWS_IN_A: usize = 6;
    pub(crate) const COLUMNS_IN_A: usize = 5;

    pub(crate) const ETA: Eta = Eta::Four;

    // To sample a value in the interval [-ETA, ETA], we can sample a value (say 'v')
    // in the interval [0, 2 * ETA] and then compute ETA - v. This can be done in
    // 4 bits when ETA is 4.
    pub(crate) const BITS_PER_ERROR_COEFFICIENT: usize = 4;

    pub(crate) const GAMMA1_EXPONENT: usize = 19;
    pub(crate) const GAMMA2: i32 = (FIELD_MODULUS - 1) / 32;

    // To sample a value in the interval [-(GAMMA - 1), GAMMA], we can sample a
    // value (say 'v') in the interval [0, (2 * GAMMA) - 1] and then compute
    // GAMMA - v. This can be done in 20 bits when GAMMA is 2^{19}.
    pub(crate) const BITS_PER_GAMMA1_COEFFICIENT: usize = 20;

    pub(crate) const MAX_ONES_IN_HINT: usize = 55;

    pub(crate) const ONES_IN_VERIFIER_CHALLENGE: usize = 49;

    pub(crate) const COMMITMENT_HASH_SIZE: usize = 48;

    // Commitment coefficients are in the interval: [0, ((FIELD_MODULUS − 1)/2γ2) − 1]
    // ((FIELD_MODULUS − 1)/2γ2) − 1 = 15, which means we need 4 bits to represent a
    // coefficient.
    pub(crate) const BITS_PER_COMMITMENT_COEFFICIENT: usize = 4;
}

/// ML-DSA-87-specific parameters
#[cfg(feature = "mldsa87")]
pub(crate) mod ml_dsa_87 {
    use super::Eta;
    use crate::constants::*;

    pub(crate) const ROWS_IN_A: usize = 8;
    pub(crate) const COLUMNS_IN_A: usize = 7;

    pub(crate) const ETA: Eta = Eta::Two;

    // To sample a value in the interval [-ETA, ETA], we can sample a value (say 'v')
    // in the interval [0, 2 * ETA] and then compute ETA - v. This can be done in
    // 3 bits when ETA is 2.
    pub(crate) const BITS_PER_ERROR_COEFFICIENT: usize = 3;

    pub(crate) const GAMMA1_EXPONENT: usize = 19;
    // To sample a value in the interval [-(GAMMA - 1), GAMMA], we can sample a
    // value (say 'v') in the interval [0, (2 * GAMMA) - 1] and then compute
    // GAMMA - v. This can be done in 20 bits when GAMMA is 2^{19}.
    pub(crate) const BITS_PER_GAMMA1_COEFFICIENT: usize = 20;

    pub(crate) const MAX_ONES_IN_HINT: usize = 75;

    pub(crate) const ONES_IN_VERIFIER_CHALLENGE: usize = 60;

    pub(crate) const GAMMA2: i32 = (FIELD_MODULUS - 1) / 32;

    // Commitment coefficients are in the interval: [0, ((FIELD_MODULUS − 1)/2γ2) − 1]
    // ((FIELD_MODULUS − 1)/2γ2) − 1 = 15, which means we need 4 bits to represent a
    // coefficient.
    pub(crate) const BITS_PER_COMMITMENT_COEFFICIENT: usize = 4;

    pub(crate) const COMMITMENT_HASH_SIZE: usize = 64;
}

#[hax_lib::requires(ones_in_verifier_challenge <= 60)]
pub(crate) const fn beta(ones_in_verifier_challenge: usize, eta: Eta) -> i32 {
    // [eurydice] can't handle conversion of enum into a usize
    let eta_val: usize = match eta {
        Eta::Two => 2,
        Eta::Four => 4,
    };
    (ones_in_verifier_challenge * eta_val) as i32
}

#[hax_lib::requires(bits_per_error_coefficient <= 4)]
pub(crate) const fn error_ring_element_size(bits_per_error_coefficient: usize) -> usize {
    (bits_per_error_coefficient * COEFFICIENTS_IN_RING_ELEMENT) / 8
}

#[hax_lib::requires(bits_per_gamma1_coefficient <= 20)]
pub(crate) const fn gamma1_ring_element_size(bits_per_gamma1_coefficient: usize) -> usize {
    (bits_per_gamma1_coefficient * COEFFICIENTS_IN_RING_ELEMENT) / 8
}

#[hax_lib::requires(bits_per_commitment_coefficient <= 6)]
pub(crate) const fn commitment_ring_element_size(bits_per_commitment_coefficient: usize) -> usize {
    (bits_per_commitment_coefficient * COEFFICIENTS_IN_RING_ELEMENT) / 8
}

#[hax_lib::requires(bits_per_commitment_coefficient <= 6 && rows_in_a <= 8)]
pub(crate) const fn commitment_vector_size(
    bits_per_commitment_coefficient: usize,
    rows_in_a: usize,
) -> usize {
    commitment_ring_element_size(bits_per_commitment_coefficient) * rows_in_a
}

#[hax_lib::requires(rows_in_a <= 8 && columns_in_a <= 7 && error_ring_element_size <= 128)]
pub(crate) const fn signing_key_size(
    rows_in_a: usize,
    columns_in_a: usize,
    error_ring_element_size: usize,
) -> usize {
    SEED_FOR_A_SIZE
        + SEED_FOR_SIGNING_SIZE
        + BYTES_FOR_VERIFICATION_KEY_HASH
        + (rows_in_a + columns_in_a) * error_ring_element_size
        + rows_in_a * RING_ELEMENT_OF_T0S_SIZE
}

#[hax_lib::requires(rows_in_a <= 8)]
pub(crate) const fn verification_key_size(rows_in_a: usize) -> usize {
    SEED_FOR_A_SIZE
        + (COEFFICIENTS_IN_RING_ELEMENT
            * rows_in_a
            * (FIELD_MODULUS_MINUS_ONE_BIT_LENGTH - BITS_IN_LOWER_PART_OF_T))
            / 8
}

#[hax_lib::requires(rows_in_a <= 8 && columns_in_a <= 7 && max_ones_in_hint <= 80 && commitment_hash_size <= 64 && bits_per_gamma1_coefficient <= 20)]
pub(crate) const fn signature_size(
    rows_in_a: usize,
    columns_in_a: usize,
    max_ones_in_hint: usize,
    commitment_hash_size: usize,
    bits_per_gamma1_coefficient: usize,
) -> usize {
    commitment_hash_size
        + (columns_in_a * gamma1_ring_element_size(bits_per_gamma1_coefficient))
        + max_ones_in_hint
        + rows_in_a
}

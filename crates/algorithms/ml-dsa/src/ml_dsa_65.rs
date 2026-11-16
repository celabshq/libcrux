use crate::ml_dsa_generic::ml_dsa_65::*;
use crate::{constants::*, types::*, SigningError, VerificationError};

pub use crate::ml_dsa_generic::ml_dsa_65::{
    MLDSA65KeyPair, MLDSA65Signature, MLDSA65SigningKey, MLDSA65VerificationKey,
};

// Instantiate the different functions.
macro_rules! instantiate {
    ($modp:ident, $doc:expr) => {
        #[doc = $doc]
        pub mod $modp {
            use super::*;

            /// Generate an ML-DSA-65 Key Pair
            pub fn generate_key_pair(
                randomness: [u8; KEY_GENERATION_RANDOMNESS_SIZE],
            ) -> MLDSA65KeyPair {
                let mut signing_key = [0u8; SIGNING_KEY_SIZE];
                let mut verification_key = [0u8; VERIFICATION_KEY_SIZE];
                crate::ml_dsa_generic::instantiations::$modp::ml_dsa_65::generate_key_pair(
                    randomness,
                    &mut signing_key,
                    &mut verification_key,
                );

                MLDSA65KeyPair {
                    signing_key: MLDSASigningKey::new(signing_key),
                    verification_key: MLDSAVerificationKey::new(verification_key),
                }
            }

            /// Generate an ML-DSA-65 Key Pair
            pub fn generate_key_pair_mut(
                randomness: [u8; KEY_GENERATION_RANDOMNESS_SIZE],
                signing_key: &mut [u8; SIGNING_KEY_SIZE],
                verification_key: &mut [u8; VERIFICATION_KEY_SIZE],
            ) {
                crate::ml_dsa_generic::instantiations::$modp::ml_dsa_65::generate_key_pair(
                    randomness,
                    signing_key,
                    verification_key,
                );
            }

            /// Generate an ML-DSA-65 Signature
            ///
            /// The parameter `context` is used for domain separation
            /// and is a byte string of length at most 255 bytes. It
            /// may also be empty.
            pub fn sign(
                signing_key: &MLDSA65SigningKey,
                message: &[u8],
                context: &[u8],
                randomness: [u8; SIGNING_RANDOMNESS_SIZE],
            ) -> Result<MLDSA65Signature, SigningError> {
                crate::ml_dsa_generic::instantiations::$modp::ml_dsa_65::sign(
                    signing_key.as_ref(),
                    message,
                    context,
                    randomness,
                )
            }

            /// Generate an ML-DSA-65 Signature
            ///
            /// The parameter `context` is used for domain separation
            /// and is a byte string of length at most 255 bytes. It
            /// may also be empty.
            pub fn sign_mut(
                signing_key: &[u8; SIGNING_KEY_SIZE],
                message: &[u8],
                context: &[u8],
                randomness: [u8; SIGNING_RANDOMNESS_SIZE],
                signature: &mut [u8; SIGNATURE_SIZE],
            ) -> Result<(), SigningError> {
                crate::ml_dsa_generic::instantiations::$modp::ml_dsa_65::sign_mut(
                    signing_key,
                    message,
                    context,
                    randomness,
                    signature,
                )
            }

            /// Generate an ML-DSA-65 Signature (Algorithm 7 in FIPS204)
            ///
            /// The message is assumed to be domain-separated.
            #[cfg(feature = "acvp")]
            pub fn sign_internal(
                signing_key: &MLDSA65SigningKey,
                message: &[u8],
                randomness: [u8; SIGNING_RANDOMNESS_SIZE],
            ) -> Result<MLDSA65Signature, SigningError> {
                crate::ml_dsa_generic::instantiations::$modp::ml_dsa_65::sign_internal(
                    signing_key.as_ref(),
                    message,
                    randomness,
                )
            }

            /// Verify an ML-DSA-65 Signature (Algorithm 8 in FIPS204)
            ///
            /// The message is assumed to be domain-separated.
            #[cfg(feature = "acvp")]
            pub fn verify_internal(
                verification_key: &MLDSA65VerificationKey,
                message: &[u8],
                signature: &MLDSA65Signature,
            ) -> Result<(), VerificationError> {
                crate::ml_dsa_generic::instantiations::$modp::ml_dsa_65::verify_internal(
                    verification_key.as_ref(),
                    message,
                    signature.as_ref(),
                )
            }

            /// Generate a HashML-DSA-65 Signature, with a SHAKE128 pre-hashing
            ///
            /// The parameter `context` is used for domain separation
            /// and is a byte string of length at most 255 bytes. It
            /// may also be empty.
            pub fn sign_pre_hashed_shake128(
                signing_key: &MLDSA65SigningKey,
                message: &[u8],
                context: &[u8],
                randomness: [u8; SIGNING_RANDOMNESS_SIZE],
            ) -> Result<MLDSA65Signature, SigningError> {
                let mut pre_hash_buffer = [0u8; 32];
                crate::ml_dsa_generic::instantiations::$modp::ml_dsa_65::sign_pre_hashed_shake128(
                    signing_key.as_ref(),
                    message,
                    context,
                    &mut pre_hash_buffer,
                    randomness,
                )
            }

            /// Verify an ML-DSA-65 Signature
            ///
            /// The parameter `context` is used for domain separation
            /// and is a byte string of length at most 255 bytes. It
            /// may also be empty.
            pub fn verify(
                verification_key: &MLDSA65VerificationKey,
                message: &[u8],
                context: &[u8],
                signature: &MLDSA65Signature,
            ) -> Result<(), VerificationError> {
                crate::ml_dsa_generic::instantiations::$modp::ml_dsa_65::verify(
                    verification_key.as_ref(),
                    message,
                    context,
                    signature.as_ref(),
                )
            }

            /// Verify a HashML-DSA-65 Signature, with a SHAKE128 pre-hashing
            ///
            /// The parameter `context` is used for domain separation
            /// and is a byte string of length at most 255 bytes. It
            /// may also be empty.
            pub fn verify_pre_hashed_shake128(
                verification_key: &MLDSA65VerificationKey,
                message: &[u8],
                context: &[u8],
                signature: &MLDSA65Signature,
            ) -> Result<(), VerificationError> {
                let mut pre_hash_buffer = [0u8; 32];
                crate::ml_dsa_generic::instantiations::$modp::ml_dsa_65::verify_pre_hashed_shake128(
                    verification_key.as_ref(),
                    message,
                    context,
                    &mut pre_hash_buffer,
                    signature.as_ref(),
                )
            }
        }
    };
}

// Instantiations
instantiate! {portable, "Portable ML-DSA 65"}
#[cfg(feature = "simd256")]
instantiate! {avx2, "AVX2 Optimised ML-DSA 65"}
#[cfg(feature = "simd128")]
instantiate! {neon, "Neon Optimised ML-DSA 65"}

/// Generate an ML-DSA 65 Key Pair
///
/// Generate an ML-DSA key pair. The input is a byte array of size
/// [`KEY_GENERATION_RANDOMNESS_SIZE`].
///
/// This function returns an [`MLDSA65KeyPair`].
#[cfg(not(eurydice))]
pub fn generate_key_pair(randomness: [u8; KEY_GENERATION_RANDOMNESS_SIZE]) -> MLDSA65KeyPair {
    let mut signing_key = [0u8; SIGNING_KEY_SIZE];
    let mut verification_key = [0u8; VERIFICATION_KEY_SIZE];
    crate::ml_dsa_generic::multiplexing::ml_dsa_65::generate_key_pair(
        randomness,
        &mut signing_key,
        &mut verification_key,
    );

    MLDSA65KeyPair {
        signing_key: MLDSASigningKey::new(signing_key),
        verification_key: MLDSAVerificationKey::new(verification_key),
    }
}

/// Sign with ML-DSA 65
///
/// Sign a `message` with the ML-DSA `signing_key`.
///
/// The parameter `context` is used for domain separation
/// and is a byte string of length at most 255 bytes. It
/// may also be empty.
///
/// This function returns an [`MLDSA65Signature`].
#[cfg(not(eurydice))]
pub fn sign(
    signing_key: &MLDSA65SigningKey,
    message: &[u8],
    context: &[u8],
    randomness: [u8; SIGNING_RANDOMNESS_SIZE],
) -> Result<MLDSA65Signature, SigningError> {
    crate::ml_dsa_generic::multiplexing::ml_dsa_65::sign(
        signing_key.as_ref(),
        message,
        context,
        randomness,
    )
}

/// Sign with ML-DSA 65 (Algorithm 7 in FIPS204)
///
/// Sign a `message` (assumed to be domain-separated) with the ML-DSA `signing_key`.
///
/// This function returns an [`MLDSA65Signature`].
#[cfg(all(not(eurydice), feature = "acvp"))]
pub fn sign_internal(
    signing_key: &MLDSA65SigningKey,
    message: &[u8],
    randomness: [u8; SIGNING_RANDOMNESS_SIZE],
) -> Result<MLDSA65Signature, SigningError> {
    crate::ml_dsa_generic::multiplexing::ml_dsa_65::sign_internal(
        signing_key.as_ref(),
        message,
        randomness,
    )
}

/// Verify an ML-DSA-65 Signature (Algorithm 8 in FIPS204)
///
/// Returns `Ok` when the `signature` is valid for the `message` (assumed to be domain-separated) and
/// `verification_key`, and a [`VerificationError`] otherwise.
#[cfg(all(not(eurydice), feature = "acvp"))]
pub fn verify_internal(
    verification_key: &MLDSA65VerificationKey,
    message: &[u8],
    signature: &MLDSA65Signature,
) -> Result<(), VerificationError> {
    crate::ml_dsa_generic::multiplexing::ml_dsa_65::verify_internal(
        verification_key.as_ref(),
        message,
        signature.as_ref(),
    )
}

/// Verify an ML-DSA-65 Signature
///
/// The parameter `context` is used for domain separation
/// and is a byte string of length at most 255 bytes. It
/// may also be empty.
///
/// Returns `Ok` when the `signature` is valid for the `message` and
/// `verification_key`, and a [`VerificationError`] otherwise.
#[cfg(not(eurydice))]
pub fn verify(
    verification_key: &MLDSA65VerificationKey,
    message: &[u8],
    context: &[u8],
    signature: &MLDSA65Signature,
) -> Result<(), VerificationError> {
    crate::ml_dsa_generic::multiplexing::ml_dsa_65::verify(
        verification_key.as_ref(),
        message,
        context,
        signature.as_ref(),
    )
}

/// Sign with HashML-DSA 65, with a SHAKE128 pre-hashing
///
/// Sign a digest of `message` derived using `pre_hash` with the
/// ML-DSA `signing_key`.
///
/// The parameter `context` is used for domain separation
/// and is a byte string of length at most 255 bytes. It
/// may also be empty.
///
/// This function returns an [`MLDSA65Signature`].
#[cfg(not(eurydice))]
pub fn sign_pre_hashed_shake128(
    signing_key: &MLDSA65SigningKey,
    message: &[u8],
    context: &[u8],
    randomness: [u8; SIGNING_RANDOMNESS_SIZE],
) -> Result<MLDSA65Signature, SigningError> {
    let mut pre_hash_buffer = [0u8; 32];
    crate::ml_dsa_generic::multiplexing::ml_dsa_65::sign_pre_hashed_shake128(
        signing_key.as_ref(),
        message,
        context,
        &mut pre_hash_buffer,
        randomness,
    )
}

/// Verify a HashML-DSA-65 Signature, with a SHAKE128 pre-hashing
///
/// The parameter `context` is used for domain separation
/// and is a byte string of length at most 255 bytes. It
/// may also be empty.
///
/// Returns `Ok` when the `signature` is valid for the `message` and
/// `verification_key`, and a [`VerificationError`] otherwise.
#[cfg(not(eurydice))]
pub fn verify_pre_hashed_shake128(
    verification_key: &MLDSA65VerificationKey,
    message: &[u8],
    context: &[u8],
    signature: &MLDSA65Signature,
) -> Result<(), VerificationError> {
    let mut pre_hash_buffer = [0u8; 32];
    crate::ml_dsa_generic::multiplexing::ml_dsa_65::verify_pre_hashed_shake128(
        verification_key.as_ref(),
        message,
        context,
        &mut pre_hash_buffer,
        signature.as_ref(),
    )
}

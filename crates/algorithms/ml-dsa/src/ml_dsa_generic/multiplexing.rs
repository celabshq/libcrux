use super::*;

macro_rules! parameter_set {
    ($parameter_module:ident, $feature:literal) => {
        #[cfg(feature = $feature)]
        pub mod $parameter_module {
            use super::*;
            use crate::ml_dsa_generic::$parameter_module::{
                SIGNATURE_SIZE, SIGNING_KEY_SIZE, VERIFICATION_KEY_SIZE,
            };

            #[cfg(all(feature = "simd256", feature = $feature))]
            use instantiations::avx2::$parameter_module::{
                generate_key_pair as generate_key_pair_avx2, sign as sign_avx2,
                sign_pre_hashed_shake128 as sign_pre_hashed_shake128_avx2, verify as verify_avx2,
                verify_pre_hashed_shake128 as verify_pre_hashed_shake128_avx2,
            };

            #[cfg(all(feature = "simd256", feature = "acvp", feature = $feature))]
            use instantiations::avx2::$parameter_module::{
                sign_internal as sign_internal_avx2, verify_internal as verify_internal_avx2,
            };

            #[cfg(all(feature = "simd128", feature = $feature))]
            use instantiations::neon::$parameter_module::{
                generate_key_pair as generate_key_pair_neon, sign as sign_neon,
                sign_pre_hashed_shake128 as sign_pre_hashed_shake128_neon, verify as verify_neon,
                verify_pre_hashed_shake128 as verify_pre_hashed_shake128_neon,
            };

            #[cfg(all(feature = "simd128", feature = "acvp", feature = $feature))]
            use instantiations::neon::$parameter_module::{
                sign_internal as sign_internal_neon, verify_internal as verify_internal_neon,
            };

            // For the case where we didn't compile with the simd128/simd256 features but
            // have a CPU that has it and thus tries to call the simd128/simd256 version,
            // we fall back to the portable version in this case.
            #[cfg(all(not(feature = "simd256"), feature = $feature))]
            use instantiations::portable::$parameter_module::{
                generate_key_pair as generate_key_pair_avx2, sign as sign_avx2,
                sign_pre_hashed_shake128 as sign_pre_hashed_shake128_avx2, verify as verify_avx2,
                verify_pre_hashed_shake128 as verify_pre_hashed_shake128_avx2,
            };

            #[cfg(all(not(feature = "simd256"), feature = "acvp", feature = $feature))]
            use instantiations::portable::$parameter_module::{
                sign_internal as sign_internal_avx2, verify_internal as verify_internal_avx2,
            };

            #[cfg(all(not(feature = "simd128"), feature = $feature))]
            use instantiations::portable::$parameter_module::{
                generate_key_pair as generate_key_pair_neon, sign as sign_neon,
                sign_pre_hashed_shake128 as sign_pre_hashed_shake128_neon, verify as verify_neon,
                verify_pre_hashed_shake128 as verify_pre_hashed_shake128_neon,
            };

            #[cfg(all(not(feature = "simd128"), feature = "acvp", feature = $feature))]
            use instantiations::portable::$parameter_module::{
                sign_internal as sign_internal_neon, verify_internal as verify_internal_neon,
            };

            pub(crate) fn generate_key_pair(
                randomness: [u8; KEY_GENERATION_RANDOMNESS_SIZE],
                signing_key: &mut [u8; SIGNING_KEY_SIZE],
                verification_key: &mut [u8; VERIFICATION_KEY_SIZE],
            ) {
                if libcrux_platform::simd256_support() {
                    generate_key_pair_avx2(randomness, signing_key, verification_key);
                } else if libcrux_platform::simd128_support() {
                    generate_key_pair_neon(randomness, signing_key, verification_key);
                } else {
                    instantiations::portable::$parameter_module::generate_key_pair(
                        randomness,
                        signing_key,
                        verification_key,
                    );
                }
            }

            #[cfg(feature = "acvp")]
            pub(crate) fn sign_internal(
                signing_key: &[u8; SIGNING_KEY_SIZE],
                message: &[u8],
                randomness: [u8; SIGNING_RANDOMNESS_SIZE],
            ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
                if libcrux_platform::simd256_support() {
                    sign_internal_avx2(signing_key, message, randomness)
                } else if libcrux_platform::simd128_support() {
                    sign_internal_neon(signing_key, message, randomness)
                } else {
                    instantiations::portable::$parameter_module::sign_internal(
                        signing_key,
                        message,
                        randomness,
                    )
                }
            }

            pub(crate) fn sign(
                signing_key: &[u8; SIGNING_KEY_SIZE],
                message: &[u8],
                context: &[u8],
                randomness: [u8; SIGNING_RANDOMNESS_SIZE],
            ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
                if libcrux_platform::simd256_support() {
                    sign_avx2(signing_key, message, context, randomness)
                } else if libcrux_platform::simd128_support() {
                    sign_neon(signing_key, message, context, randomness)
                } else {
                    instantiations::portable::$parameter_module::sign(
                        signing_key,
                        message,
                        context,
                        randomness,
                    )
                }
            }

            pub(crate) fn sign_pre_hashed_shake128(
                signing_key: &[u8; SIGNING_KEY_SIZE],
                message: &[u8],
                context: &[u8],
                pre_hash_buffer: &mut [u8],
                randomness: [u8; SIGNING_RANDOMNESS_SIZE],
            ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
                if libcrux_platform::simd256_support() {
                    sign_pre_hashed_shake128_avx2(
                        signing_key,
                        message,
                        context,
                        pre_hash_buffer,
                        randomness,
                    )
                } else if libcrux_platform::simd128_support() {
                    sign_pre_hashed_shake128_neon(
                        signing_key,
                        message,
                        context,
                        pre_hash_buffer,
                        randomness,
                    )
                } else {
                    instantiations::portable::$parameter_module::sign_pre_hashed_shake128(
                        signing_key,
                        message,
                        context,
                        pre_hash_buffer,
                        randomness,
                    )
                }
            }

            #[cfg(feature = "acvp")]
            pub(crate) fn verify_internal(
                verification_key_serialized: &[u8; VERIFICATION_KEY_SIZE],
                message: &[u8],
                signature_serialized: &[u8; SIGNATURE_SIZE],
            ) -> Result<(), VerificationError> {
                if libcrux_platform::simd256_support() {
                    verify_internal_avx2(verification_key_serialized, message, signature_serialized)
                } else if libcrux_platform::simd128_support() {
                    verify_internal_neon(verification_key_serialized, message, signature_serialized)
                } else {
                    instantiations::portable::$parameter_module::verify_internal(
                        verification_key_serialized,
                        message,
                        signature_serialized,
                    )
                }
            }

            pub(crate) fn verify(
                verification_key_serialized: &[u8; VERIFICATION_KEY_SIZE],
                message: &[u8],
                context: &[u8],
                signature_serialized: &[u8; SIGNATURE_SIZE],
            ) -> Result<(), VerificationError> {
                if libcrux_platform::simd256_support() {
                    verify_avx2(
                        verification_key_serialized,
                        message,
                        context,
                        signature_serialized,
                    )
                } else if libcrux_platform::simd128_support() {
                    verify_neon(
                        verification_key_serialized,
                        message,
                        context,
                        signature_serialized,
                    )
                } else {
                    instantiations::portable::$parameter_module::verify(
                        verification_key_serialized,
                        message,
                        context,
                        signature_serialized,
                    )
                }
            }

            pub(crate) fn verify_pre_hashed_shake128(
                verification_key_serialized: &[u8; VERIFICATION_KEY_SIZE],
                message: &[u8],
                context: &[u8],
                pre_hash_buffer: &mut [u8],
                signature_serialized: &[u8; SIGNATURE_SIZE],
            ) -> Result<(), VerificationError> {
                if libcrux_platform::simd256_support() {
                    verify_pre_hashed_shake128_avx2(
                        verification_key_serialized,
                        message,
                        context,
                        pre_hash_buffer,
                        signature_serialized,
                    )
                } else if libcrux_platform::simd128_support() {
                    verify_pre_hashed_shake128_neon(
                        verification_key_serialized,
                        message,
                        context,
                        pre_hash_buffer,
                        signature_serialized,
                    )
                } else {
                    instantiations::portable::$parameter_module::verify_pre_hashed_shake128(
                        verification_key_serialized,
                        message,
                        context,
                        pre_hash_buffer,
                        signature_serialized,
                    )
                }
            }
        }
    };
}

parameter_set!(ml_dsa_44, "mldsa44");
parameter_set!(ml_dsa_65, "mldsa65");
parameter_set!(ml_dsa_87, "mldsa87");

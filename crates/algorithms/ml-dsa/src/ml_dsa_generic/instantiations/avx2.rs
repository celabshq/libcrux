use crate::{
    constants::*,
    ml_dsa_generic::{SigningError, VerificationError},
    pre_hash::SHAKE128_PH,
    types::*,
};

macro_rules! parameter_set {
    ($parameter_module:ident, $feature:literal) => {
        #[cfg(feature = $feature)]
        pub(crate) mod $parameter_module {
            use super::*;
            use crate::ml_dsa_generic::$parameter_module::{
                SIGNATURE_SIZE, SIGNING_KEY_SIZE, VERIFICATION_KEY_SIZE,
            };

            #[allow(unsafe_code)]
            pub fn generate_key_pair(
                randomness: [u8; KEY_GENERATION_RANDOMNESS_SIZE],
                signing_key: &mut [u8],
                verification_key: &mut [u8],
            ) {
                /// Key Generation.
                #[allow(unsafe_code)]
                #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
                unsafe fn _inner(
                    randomness: [u8; KEY_GENERATION_RANDOMNESS_SIZE],
                    signing_key: &mut [u8],
                    verification_key: &mut [u8],
                ) {
                    crate::ml_dsa_generic::$parameter_module::generate_key_pair::<
                        crate::simd::avx2::AVX2SIMDUnit,
                        crate::samplex4::avx2::AVX2Sampler,
                        crate::hash_functions::simd256::Shake128x4,
                        crate::hash_functions::simd256::Shake256,
                        crate::hash_functions::portable::Shake256Xof,
                        crate::hash_functions::simd256::Shake256x4,
                    >(randomness, signing_key, verification_key);
                }

                unsafe { _inner(randomness, signing_key, verification_key) }
            }

            #[allow(unsafe_code)]
            /// Sign.
            pub fn sign(
                signing_key: &[u8; SIGNING_KEY_SIZE],
                message: &[u8],
                context: &[u8],
                randomness: [u8; SIGNING_RANDOMNESS_SIZE],
            ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
                #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
                #[allow(unsafe_code)]
                unsafe fn _inner(
                    signing_key: &[u8; SIGNING_KEY_SIZE],
                    message: &[u8],
                    context: &[u8],
                    randomness: [u8; SIGNING_RANDOMNESS_SIZE],
                ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
                    crate::ml_dsa_generic::$parameter_module::sign::<
                        crate::simd::avx2::AVX2SIMDUnit,
                        crate::samplex4::avx2::AVX2Sampler,
                        crate::hash_functions::simd256::Shake128x4,
                        crate::hash_functions::simd256::Shake256,
                        // We use the portable version here.
                        // It doesn' make sense to do these in parallel.
                        crate::hash_functions::portable::Shake256Xof,
                        crate::hash_functions::simd256::Shake256x4,
                    >(signing_key, message, context, randomness)
                }
                unsafe { _inner(signing_key, message, context, randomness) }
            }

            #[allow(unsafe_code)]
            /// Sign.
            pub fn sign_mut(
                signing_key: &[u8; SIGNING_KEY_SIZE],
                message: &[u8],
                context: &[u8],
                randomness: [u8; SIGNING_RANDOMNESS_SIZE],
                signature: &mut [u8; SIGNATURE_SIZE],
            ) -> Result<(), SigningError> {
                #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
                #[allow(unsafe_code)]
                unsafe fn _inner(
                    signing_key: &[u8; SIGNING_KEY_SIZE],
                    message: &[u8],
                    context: &[u8],
                    randomness: [u8; SIGNING_RANDOMNESS_SIZE],
                    signature: &mut [u8; SIGNATURE_SIZE],
                ) -> Result<(), SigningError> {
                    crate::ml_dsa_generic::$parameter_module::sign_mut::<
                        crate::simd::avx2::AVX2SIMDUnit,
                        crate::samplex4::avx2::AVX2Sampler,
                        crate::hash_functions::simd256::Shake128x4,
                        crate::hash_functions::simd256::Shake256,
                        // We use the portable version here.
                        // It doesn' make sense to do these in parallel.
                        crate::hash_functions::portable::Shake256Xof,
                        crate::hash_functions::simd256::Shake256x4,
                    >(signing_key, message, context, randomness, signature)
                }
                unsafe { _inner(signing_key, message, context, randomness, signature) }
            }

            /// Sign (internal API)
            #[allow(unsafe_code)]
            #[cfg(feature = "acvp")]
            pub fn sign_internal(
                signing_key: &[u8; SIGNING_KEY_SIZE],
                message: &[u8],
                randomness: [u8; SIGNING_RANDOMNESS_SIZE],
            ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
                #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
                #[allow(unsafe_code)]
                unsafe fn _inner(
                    signing_key: &[u8; SIGNING_KEY_SIZE],
                    message: &[u8],
                    randomness: [u8; SIGNING_RANDOMNESS_SIZE],
                    signature: &mut [u8; SIGNATURE_SIZE],
                ) -> Result<(), SigningError> {
                    crate::ml_dsa_generic::$parameter_module::sign_internal::<
                        crate::simd::avx2::AVX2SIMDUnit,
                        crate::samplex4::avx2::AVX2Sampler,
                        crate::hash_functions::simd256::Shake128x4,
                        crate::hash_functions::simd256::Shake256,
                        // We use the portable version here.
                        // It doesn' make sense to do these in parallel.
                        crate::hash_functions::portable::Shake256Xof,
                        crate::hash_functions::simd256::Shake256x4,
                    >(signing_key, message, None, randomness, signature)
                }

                let mut signature = MLDSASignature::zero();
                unsafe {
                    _inner(&signing_key, message, randomness, &mut signature.value)?;
                }

                Ok(signature)
            }

            /// Sign (pre-hashed).
            #[allow(unsafe_code)]
            pub fn sign_pre_hashed_shake128(
                signing_key: &[u8; SIGNING_KEY_SIZE],
                message: &[u8],
                context: &[u8],
                pre_hash_buffer: &mut [u8],
                randomness: [u8; SIGNING_RANDOMNESS_SIZE],
            ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
                #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
                #[allow(unsafe_code)]
                unsafe fn _inner(
                    signing_key: &[u8; SIGNING_KEY_SIZE],
                    message: &[u8],
                    context: &[u8],
                    pre_hash_buffer: &mut [u8],
                    randomness: [u8; SIGNING_RANDOMNESS_SIZE],
                ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
                    crate::ml_dsa_generic::$parameter_module::sign_pre_hashed::<
                        crate::simd::avx2::AVX2SIMDUnit,
                        crate::samplex4::avx2::AVX2Sampler,
                        // We use the portable version here.
                        // It doesn' make sense to do these in parallel.
                        crate::hash_functions::portable::Shake128,
                        crate::hash_functions::simd256::Shake128x4,
                        crate::hash_functions::simd256::Shake256,
                        // We use the portable version here.
                        // It doesn' make sense to do these in parallel.
                        crate::hash_functions::portable::Shake256Xof,
                        crate::hash_functions::simd256::Shake256x4,
                        SHAKE128_PH,
                    >(signing_key, message, context, pre_hash_buffer, randomness)
                }
                unsafe { _inner(signing_key, message, context, pre_hash_buffer, randomness) }
            }

            /// Verify.
            #[allow(unsafe_code)]
            pub fn verify(
                verification_key: &[u8; VERIFICATION_KEY_SIZE],
                message: &[u8],
                context: &[u8],
                signature: &[u8; SIGNATURE_SIZE],
            ) -> Result<(), VerificationError> {
                #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
                #[allow(unsafe_code)]
                unsafe fn _inner(
                    verification_key: &[u8; VERIFICATION_KEY_SIZE],
                    message: &[u8],
                    context: &[u8],
                    signature: &[u8; SIGNATURE_SIZE],
                ) -> Result<(), VerificationError> {
                    crate::ml_dsa_generic::$parameter_module::verify::<
                        crate::simd::avx2::AVX2SIMDUnit,
                        crate::samplex4::avx2::AVX2Sampler,
                        crate::hash_functions::simd256::Shake128x4,
                        crate::hash_functions::simd256::Shake256,
                        // We use the portable version here.
                        // It doesn' make sense to do these in parallel.
                        crate::hash_functions::portable::Shake256Xof,
                    >(verification_key, message, context, signature)
                }
                unsafe { _inner(verification_key, message, context, signature) }
            }

            /// Verify (internal API).
            #[cfg(feature = "acvp")]
            #[allow(unsafe_code)]
            pub fn verify_internal(
                verification_key: &[u8; VERIFICATION_KEY_SIZE],
                message: &[u8],
                signature: &[u8; SIGNATURE_SIZE],
            ) -> Result<(), VerificationError> {
                #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
                #[allow(unsafe_code)]
                unsafe fn _inner(
                    verification_key: &[u8; VERIFICATION_KEY_SIZE],
                    message: &[u8],
                    signature: &[u8; SIGNATURE_SIZE],
                ) -> Result<(), VerificationError> {
                    crate::ml_dsa_generic::$parameter_module::verify_internal::<
                        crate::simd::avx2::AVX2SIMDUnit,
                        crate::samplex4::avx2::AVX2Sampler,
                        crate::hash_functions::simd256::Shake128x4,
                        crate::hash_functions::simd256::Shake256,
                        // We use the portable version here.
                        // It doesn' make sense to do these in parallel.
                        crate::hash_functions::portable::Shake256Xof,
                    >(verification_key, message, None, signature)
                }
                unsafe { _inner(verification_key, message, signature) }
            }

            /// Verify (pre-hashed with SHAKE-128).
            #[allow(unsafe_code)]
            pub fn verify_pre_hashed_shake128(
                verification_key: &[u8; VERIFICATION_KEY_SIZE],
                message: &[u8],
                context: &[u8],
                pre_hash_buffer: &mut [u8],
                signature: &[u8; SIGNATURE_SIZE],
            ) -> Result<(), VerificationError> {
                #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
                #[allow(unsafe_code)]
                unsafe fn _inner(
                    verification_key: &[u8; VERIFICATION_KEY_SIZE],
                    message: &[u8],
                    context: &[u8],
                    pre_hash_buffer: &mut [u8],
                    signature: &[u8; SIGNATURE_SIZE],
                ) -> Result<(), VerificationError> {
                    crate::ml_dsa_generic::$parameter_module::verify_pre_hashed::<
                        crate::simd::avx2::AVX2SIMDUnit,
                        crate::samplex4::avx2::AVX2Sampler,
                        // We use the portable version here.
                        // It doesn' make sense to do these in parallel.
                        crate::hash_functions::portable::Shake128,
                        crate::hash_functions::simd256::Shake128x4,
                        crate::hash_functions::simd256::Shake256,
                        // We use the portable version here.
                        // It doesn' make sense to do these in parallel.
                        crate::hash_functions::portable::Shake256Xof,
                        SHAKE128_PH,
                    >(
                        verification_key,
                        message,
                        context,
                        pre_hash_buffer,
                        signature,
                    )
                }
                unsafe {
                    _inner(
                        verification_key,
                        message,
                        context,
                        pre_hash_buffer,
                        signature,
                    )
                }
            }
        }
    };
}

parameter_set!(ml_dsa_44, "mldsa44");
parameter_set!(ml_dsa_65, "mldsa65");
parameter_set!(ml_dsa_87, "mldsa87");

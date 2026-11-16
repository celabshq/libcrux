macro_rules! instantiate {
    (
        $platform:ident, // name for the module
        $simdunit:path, // paths to the platform specific implementations ...
        $shake128:path,
        $shake128x4:path,
        $shake256:path,
        $shake256xof:path,
        $shake256x4:path,
        $sampler:path
    ) => {
        pub mod $platform {
            use crate::{
                constants::*,
                pre_hash::SHAKE128_PH,
                types::*,
                types::{SigningError, VerificationError},
            };

            macro_rules! parameter_set {
                ($parameter_module:ident, $feature:literal) => {
                    #[cfg(feature = $feature)]
                    pub(crate) mod $parameter_module {
                        use super::*;
                        use crate::ml_dsa_generic::$parameter_module::{
                            SIGNATURE_SIZE, SIGNING_KEY_SIZE, VERIFICATION_KEY_SIZE,
                        };

                        /// Generate key pair.
                        pub fn generate_key_pair(
                            randomness: [u8; KEY_GENERATION_RANDOMNESS_SIZE],
                            signing_key: &mut [u8; SIGNING_KEY_SIZE],
                            verification_key: &mut [u8; VERIFICATION_KEY_SIZE],
                        ) {
                            crate::ml_dsa_generic::$parameter_module::generate_key_pair::<
                                $simdunit,
                                $sampler,
                                $shake128x4,
                                $shake256,
                                $shake256xof,
                                $shake256x4,
                            >(randomness, signing_key, verification_key)
                        }

                        /// Sign.
                        pub fn sign(
                            signing_key: &[u8; SIGNING_KEY_SIZE],
                            message: &[u8],
                            context: &[u8],
                            randomness: [u8; SIGNING_RANDOMNESS_SIZE],
                        ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
                            crate::ml_dsa_generic::$parameter_module::sign::<
                                $simdunit,
                                $sampler,
                                $shake128x4,
                                $shake256,
                                $shake256xof,
                                $shake256x4,
                            >(signing_key, message, context, randomness)
                        }

                        /// Sign.
                        pub fn sign_mut(
                            signing_key: &[u8; SIGNING_KEY_SIZE],
                            message: &[u8],
                            context: &[u8],
                            randomness: [u8; SIGNING_RANDOMNESS_SIZE],
                            signature: &mut [u8; SIGNATURE_SIZE],
                        ) -> Result<(), SigningError> {
                            crate::ml_dsa_generic::$parameter_module::sign_mut::<
                                $simdunit,
                                $sampler,
                                $shake128x4,
                                $shake256,
                                $shake256xof,
                                $shake256x4,
                            >(signing_key, message, context, randomness, signature)
                        }

                        #[cfg(feature = "acvp")]
                        pub fn sign_internal(
                            signing_key: &[u8; SIGNING_KEY_SIZE],
                            message: &[u8],
                            randomness: [u8; SIGNING_RANDOMNESS_SIZE],
                        ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
                            let mut signature = MLDSASignature::zero();

                            crate::ml_dsa_generic::$parameter_module::sign_internal::<
                                $simdunit,
                                $sampler,
                                $shake128x4,
                                $shake256,
                                $shake256xof,
                                $shake256x4,
                            >(signing_key, message, None, randomness, &mut signature.value)?;

                            Ok(signature)
                        }

                        /// Sign (pre-hashed).
                        pub(crate) fn sign_pre_hashed_shake128(
                            signing_key: &[u8; SIGNING_KEY_SIZE],
                            message: &[u8],
                            context: &[u8],
                            pre_hash_buffer: &mut [u8],
                            randomness: [u8; SIGNING_RANDOMNESS_SIZE],
                        ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
                            crate::ml_dsa_generic::$parameter_module::sign_pre_hashed::<
                                $simdunit,
                                $sampler,
                                $shake128,
                                $shake128x4,
                                $shake256,
                                $shake256xof,
                                $shake256x4,
                                SHAKE128_PH,
                            >(signing_key, message, context, pre_hash_buffer, randomness)
                        }

                        /// Verify.
                        pub(crate) fn verify(
                            verification_key: &[u8; VERIFICATION_KEY_SIZE],
                            message: &[u8],
                            context: &[u8],
                            signature: &[u8; SIGNATURE_SIZE],
                        ) -> Result<(), VerificationError> {
                            crate::ml_dsa_generic::$parameter_module::verify::<
                                $simdunit,
                                $sampler,
                                $shake128x4,
                                $shake256,
                                $shake256xof,
                            >(verification_key, message, context, signature)
                        }

                        /// Verify (internal API).
                        #[cfg(feature = "acvp")]
                        pub(crate) fn verify_internal(
                            verification_key: &[u8; VERIFICATION_KEY_SIZE],
                            message: &[u8],
                            signature: &[u8; SIGNATURE_SIZE],
                        ) -> Result<(), VerificationError> {
                            crate::ml_dsa_generic::$parameter_module::verify_internal::<
                                $simdunit,
                                $sampler,
                                $shake128x4,
                                $shake256,
                                $shake256xof,
                            >(verification_key, message, None, signature)
                        }

                        /// Verify (pre-hashed with SHAKE-128).
                        pub(crate) fn verify_pre_hashed_shake128(
                            verification_key: &[u8; VERIFICATION_KEY_SIZE],
                            message: &[u8],
                            context: &[u8],
                            pre_hash_buffer: &mut [u8],
                            signature: &[u8; SIGNATURE_SIZE],
                        ) -> Result<(), VerificationError> {
                            crate::ml_dsa_generic::$parameter_module::verify_pre_hashed::<
                                $simdunit,
                                $sampler,
                                $shake128,
                                $shake128x4,
                                $shake256,
                                $shake256xof,
                                SHAKE128_PH,
                            >(
                                verification_key,
                                message,
                                context,
                                pre_hash_buffer,
                                signature,
                            )
                        }
                    }
                };
            }

            parameter_set!(ml_dsa_44, "mldsa44");
            parameter_set!(ml_dsa_65, "mldsa65");
            parameter_set!(ml_dsa_87, "mldsa87");
        }
    };
}

// Portable generic implementations.
instantiate! {portable,
    crate::simd::portable::PortableSIMDUnit,
    crate::hash_functions::portable::Shake128,
    crate::hash_functions::portable::Shake128X4,
    crate::hash_functions::portable::Shake256,
    crate::hash_functions::portable::Shake256Xof,
    crate::hash_functions::portable::Shake256X4,
    crate::samplex4::portable::PortableSampler
}

// AVX2 generic implementation.
#[cfg(feature = "simd256")]
pub mod avx2;

// NEON generic implementation.
#[cfg(feature = "simd128")]
instantiate! {neon,
    crate::simd::portable::PortableSIMDUnit,
    crate::hash_functions::portable::Shake128,
    crate::hash_functions::neon::Shake128x4,
    crate::hash_functions::portable::Shake256,
    crate::hash_functions::portable::Shake256Xof,
    crate::hash_functions::neon::Shake256x4,
    crate::samplex4::neon::NeonSampler
}

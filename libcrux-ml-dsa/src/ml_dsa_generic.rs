use crate::{
    arithmetic::{
        decompose_vector, make_hint, power2round_vector, use_hint, vector_infinity_norm_exceeds,
    },
    constants::*,
    encoding::{self},
    hash_functions::{shake128, shake256},
    matrix::{
        add_vectors, compute_as1_plus_s2, compute_matrix_x_mask, compute_w_approx,
        subtract_vectors, vector_times_ring_element,
    },
    ntt::ntt,
    polynomial::PolynomialRingElement,
    pre_hash::{DomainSeparationContext, PreHash},
    sample::{sample_challenge_ring_element, sample_mask_vector},
    samplex4::{self, X4Sampler},
    simd::traits::Operations,
    types::*,
    MLDSASignature,
};

pub(crate) mod instantiations;

#[cfg(not(eurydice))]
pub(crate) mod multiplexing;

#[libcrux_macros::ml_dsa_parameter_sets(44, 65, 87)]
pub(crate) mod generic {
    use super::*;

    // Derived constants
    const ROW_COLUMN: usize = ROWS_IN_A + COLUMNS_IN_A;
    const ROW_X_COLUMN: usize = ROWS_IN_A * COLUMNS_IN_A;
    const ERROR_RING_ELEMENT_SIZE: usize = error_ring_element_size(BITS_PER_ERROR_COEFFICIENT);
    const GAMMA1_RING_ELEMENT_SIZE: usize = gamma1_ring_element_size(BITS_PER_GAMMA1_COEFFICIENT);
    const COMMITMENT_RING_ELEMENT_SIZE: usize =
        commitment_ring_element_size(BITS_PER_COMMITMENT_COEFFICIENT);

    const BETA: i32 = beta(ONES_IN_VERIFIER_CHALLENGE, ETA);
    const COMMITMENT_VECTOR_SIZE: usize =
        commitment_vector_size(BITS_PER_COMMITMENT_COEFFICIENT, ROWS_IN_A);
    pub(crate) const SIGNING_KEY_SIZE: usize =
        signing_key_size(ROWS_IN_A, COLUMNS_IN_A, ERROR_RING_ELEMENT_SIZE);
    pub(crate) const VERIFICATION_KEY_SIZE: usize = verification_key_size(ROWS_IN_A);
    pub(crate) const SIGNATURE_SIZE: usize = signature_size(
        ROWS_IN_A,
        COLUMNS_IN_A,
        MAX_ONES_IN_HINT,
        COMMITMENT_HASH_SIZE,
        BITS_PER_GAMMA1_COEFFICIENT,
    );

    #[inline(always)]
    pub(crate) fn generate_key_pair<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        Shake256X4: shake256::XofX4,
    >(
        randomness: [u8; KEY_GENERATION_RANDOMNESS_SIZE],
        signing_key: &mut [u8],
        verification_key: &mut [u8],
    ) {
        // Check key sizes
        #[cfg(not(eurydice))]
        debug_assert!(signing_key.len() == SIGNING_KEY_SIZE);
        #[cfg(not(eurydice))]
        debug_assert!(verification_key.len() == VERIFICATION_KEY_SIZE);

        // 128 = SEED_FOR_A_SIZE + SEED_FOR_ERROR_VECTORS_SIZE + SEED_FOR_SIGNING_SIZE
        let mut seed_expanded = [0; 128];
        {
            let mut shake = Shake256Xof::init();
            shake.absorb(&randomness);
            shake.absorb_final(&[ROWS_IN_A as u8, COLUMNS_IN_A as u8]);
            shake.squeeze(&mut seed_expanded);
        }

        let (seed_for_a, seed_expanded) = seed_expanded.split_at(SEED_FOR_A_SIZE);
        let (seed_for_error_vectors, seed_for_signing) =
            seed_expanded.split_at(SEED_FOR_ERROR_VECTORS_SIZE);

        let mut s1_s2 = [PolynomialRingElement::<SIMDUnit>::zero(); ROW_COLUMN];
        samplex4::sample_s1_and_s2::<SIMDUnit, Shake256X4>(ETA, seed_for_error_vectors, &mut s1_s2);

        let mut t0 = [PolynomialRingElement::<SIMDUnit>::zero(); ROWS_IN_A];
        {
            let mut a_as_ntt = [PolynomialRingElement::<SIMDUnit>::zero(); ROW_X_COLUMN];
            Sampler::matrix_flat::<SIMDUnit>(COLUMNS_IN_A, seed_for_a, &mut a_as_ntt);

            let mut s1_ntt = [PolynomialRingElement::<SIMDUnit>::zero(); COLUMNS_IN_A];
            s1_ntt.copy_from_slice(&s1_s2[0..COLUMNS_IN_A]);
            for i in 0..s1_ntt.len() {
                ntt(&mut s1_ntt[i]);
            }
            compute_as1_plus_s2::<SIMDUnit>(
                ROWS_IN_A,
                COLUMNS_IN_A,
                &mut a_as_ntt,
                &s1_ntt,
                &s1_s2,
                &mut t0,
            );
        }

        let mut t1 = [PolynomialRingElement::<SIMDUnit>::zero(); ROWS_IN_A];
        power2round_vector::<SIMDUnit>(&mut t0, &mut t1);

        // Write out the keys
        encoding::verification_key::generate_serialized::<SIMDUnit>(
            seed_for_a,
            &t1,
            verification_key,
        );
        encoding::signing_key::generate_serialized::<SIMDUnit, Shake256>(
            ETA,
            ERROR_RING_ELEMENT_SIZE,
            seed_for_a,
            seed_for_signing,
            verification_key,
            &s1_s2,
            &t0,
            signing_key,
        );
    }

    #[inline(always)]
    pub(crate) fn sign_internal<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        Shake256X4: shake256::XofX4,
    >(
        signing_key: &[u8],
        message: &[u8],
        domain_separation_context: Option<DomainSeparationContext>,
        randomness: [u8; SIGNING_RANDOMNESS_SIZE],
        signature: &mut [u8; SIGNATURE_SIZE],
    ) -> Result<(), SigningError> {
        // Split the signing key into its parts.
        let (seed_for_a, remaining_serialized) = signing_key.split_at(SEED_FOR_A_SIZE);
        let (seed_for_signing, remaining_serialized) =
            remaining_serialized.split_at(SEED_FOR_SIGNING_SIZE);
        let (verification_key_hash, remaining_serialized) =
            remaining_serialized.split_at(BYTES_FOR_VERIFICATION_KEY_HASH);

        let (s1_serialized, remaining_serialized) =
            remaining_serialized.split_at(ERROR_RING_ELEMENT_SIZE * COLUMNS_IN_A);
        let (s2_serialized, t0_serialized) =
            remaining_serialized.split_at(ERROR_RING_ELEMENT_SIZE * ROWS_IN_A);

        // Deserialize s1, s2, and t0.
        let mut s1_as_ntt = [PolynomialRingElement::zero(); COLUMNS_IN_A];
        let mut s2_as_ntt = [PolynomialRingElement::zero(); ROWS_IN_A];
        let mut t0_as_ntt = [PolynomialRingElement::zero(); ROWS_IN_A];

        encoding::error::deserialize_to_vector_then_ntt::<SIMDUnit>(
            ETA,
            ERROR_RING_ELEMENT_SIZE,
            s1_serialized,
            &mut s1_as_ntt,
        );
        encoding::error::deserialize_to_vector_then_ntt::<SIMDUnit>(
            ETA,
            ERROR_RING_ELEMENT_SIZE,
            s2_serialized,
            &mut s2_as_ntt,
        );
        encoding::t0::deserialize_to_vector_then_ntt::<SIMDUnit>(t0_serialized, &mut t0_as_ntt);

        // Sample matrix A.
        let mut matrix = [PolynomialRingElement::<SIMDUnit>::zero(); ROW_X_COLUMN];
        Sampler::matrix_flat::<SIMDUnit>(COLUMNS_IN_A, seed_for_a, &mut matrix);

        let mut message_representative = [0; MESSAGE_REPRESENTATIVE_SIZE];
        derive_message_representative::<Shake256Xof>(
            verification_key_hash,
            &domain_separation_context,
            message,
            &mut message_representative,
        );

        let mut mask_seed = [0; MASK_SEED_SIZE];
        {
            let mut shake = Shake256Xof::init();
            shake.absorb(seed_for_signing);
            shake.absorb(&randomness);
            shake.absorb_final(&message_representative);

            shake.squeeze(&mut mask_seed);
        }

        let mut domain_separator_for_mask: u16 = 0;
        let mut attempt = 0;

        // Return values.
        // Required because we can't return early.
        // See https://github.com/hacspec/hax/issues/1171
        let mut commitment_hash = None;
        let mut signer_response = None;
        let mut hint = None;

        // As specified in [FIPS 204, Appendix C], the minimum number of
        // attempts in this rejection sampling loop is 814. This puts the
        // probability of failure at 2⁻²⁵⁶ or less.
        //
        // [FIPS 204, Appendix C]: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.pdf#appendix.C
        while attempt < REJECTION_SAMPLE_BOUND_SIGN {
            attempt += 1;

            let mut mask = [PolynomialRingElement::zero(); COLUMNS_IN_A];
            let mut w0 = [PolynomialRingElement::zero(); ROWS_IN_A];
            let mut commitment = [PolynomialRingElement::zero(); ROWS_IN_A];

            sample_mask_vector::<SIMDUnit, Shake256, Shake256X4>(
                COLUMNS_IN_A,
                GAMMA1_EXPONENT,
                &mask_seed,
                &mut domain_separator_for_mask,
                &mut mask,
            );

            {
                let mut a_x_mask = [PolynomialRingElement::zero(); ROWS_IN_A];
                let mut mask_ntt = mask.clone();
                for i in 0..mask_ntt.len() {
                    ntt(&mut mask_ntt[i]);
                }
                compute_matrix_x_mask::<SIMDUnit>(
                    ROWS_IN_A,
                    COLUMNS_IN_A,
                    &matrix,
                    &mask_ntt,
                    &mut a_x_mask,
                );
                decompose_vector::<SIMDUnit>(
                    ROWS_IN_A,
                    GAMMA2,
                    &a_x_mask,
                    &mut w0,
                    &mut commitment,
                );
            }

            let mut commitment_hash_candidate = [0; COMMITMENT_HASH_SIZE];
            {
                let mut commitment_serialized = [0u8; COMMITMENT_VECTOR_SIZE];
                encoding::commitment::serialize_vector::<SIMDUnit>(
                    COMMITMENT_RING_ELEMENT_SIZE,
                    &commitment,
                    &mut commitment_serialized,
                );

                let mut shake = Shake256Xof::init();
                shake.absorb(&message_representative);
                shake.absorb_final(&commitment_serialized);

                shake.squeeze(&mut commitment_hash_candidate);
            }

            let mut verifier_challenge = PolynomialRingElement::zero();
            sample_challenge_ring_element::<SIMDUnit, Shake256>(
                &commitment_hash_candidate,
                ONES_IN_VERIFIER_CHALLENGE,
                &mut verifier_challenge,
            );
            ntt(&mut verifier_challenge);

            // We need to clone here in case we need s1_as_ntt or s2_as_ntt again in
            // another iteration of the loop.
            let mut challenge_times_s1 = s1_as_ntt.clone();
            let mut challenge_times_s2 = s2_as_ntt.clone();

            vector_times_ring_element::<SIMDUnit>(&mut challenge_times_s1, &verifier_challenge);
            vector_times_ring_element::<SIMDUnit>(&mut challenge_times_s2, &verifier_challenge);

            add_vectors::<SIMDUnit>(COLUMNS_IN_A, &mut mask, &challenge_times_s1);
            subtract_vectors::<SIMDUnit>(ROWS_IN_A, &mut w0, &challenge_times_s2);

            if vector_infinity_norm_exceeds::<SIMDUnit>(&mask, (1 << GAMMA1_EXPONENT) - BETA) {
                // XXX: https://github.com/hacspec/hax/issues/1171
                // continue;
            } else {
                if vector_infinity_norm_exceeds::<SIMDUnit>(&w0, GAMMA2 - BETA) {
                    // XXX: https://github.com/hacspec/hax/issues/1171
                    // continue;
                } else {
                    // We need to clone here in case we need t0_as_ntt again in another iteration
                    // of the loop.
                    let mut challenge_times_t0 = t0_as_ntt.clone();
                    vector_times_ring_element::<SIMDUnit>(
                        &mut challenge_times_t0,
                        &verifier_challenge,
                    );
                    if vector_infinity_norm_exceeds::<SIMDUnit>(&challenge_times_t0, GAMMA2) {
                        // XXX: https://github.com/hacspec/hax/issues/1171
                        // continue;
                    } else {
                        add_vectors::<SIMDUnit>(ROWS_IN_A, &mut w0, &challenge_times_t0);
                        let mut hint_candidate = [[0; COEFFICIENTS_IN_RING_ELEMENT]; ROWS_IN_A];
                        let ones_in_hint =
                            make_hint::<SIMDUnit>(&w0, &commitment, GAMMA2, &mut hint_candidate);

                        if ones_in_hint > MAX_ONES_IN_HINT {
                            // XXX: https://github.com/hacspec/hax/issues/1171
                            // continue;
                        } else {
                            attempt = REJECTION_SAMPLE_BOUND_SIGN; // exit loop now
                            commitment_hash = Some(commitment_hash_candidate);
                            signer_response = Some(mask);
                            hint = Some(hint_candidate);
                        }
                    }
                }
            }
        }

        let commitment_hash = match commitment_hash {
            Some(commitment_hash) => commitment_hash,
            None => return Err(SigningError::RejectionSamplingError),
        };

        let signer_response = match signer_response {
            Some(signer_response) => signer_response,
            None => return Err(SigningError::RejectionSamplingError),
        };

        let hint = match hint {
            Some(hint) => hint,
            None => return Err(SigningError::RejectionSamplingError),
        };

        encoding::signature::serialize::<SIMDUnit>(
            &commitment_hash,
            &signer_response,
            &hint,
            COMMITMENT_HASH_SIZE,
            COLUMNS_IN_A,
            ROWS_IN_A,
            GAMMA1_EXPONENT,
            GAMMA1_RING_ELEMENT_SIZE,
            MAX_ONES_IN_HINT,
            signature,
        );

        Ok(())
    }

    /// The internal verification API.
    ///
    /// If no `domain_separation_context` is supplied, it is assumed that
    /// `message` already contains the domain separation.
    #[allow(non_snake_case)]
    #[inline(always)]
    pub(crate) fn verify_internal<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
    >(
        verification_key: &[u8; VERIFICATION_KEY_SIZE],
        message: &[u8],
        domain_separation_context: Option<DomainSeparationContext>,
        signature_serialized: &[u8; SIGNATURE_SIZE],
    ) -> Result<(), VerificationError> {
        let (seed_for_a, t1_serialized) = verification_key.split_at(SEED_FOR_A_SIZE);
        let mut t1 = [PolynomialRingElement::<SIMDUnit>::zero(); ROWS_IN_A];
        encoding::verification_key::deserialize::<SIMDUnit>(
            ROWS_IN_A,
            VERIFICATION_KEY_SIZE,
            t1_serialized,
            &mut t1,
        );

        let mut deserialized_commitment_hash = [0u8; COMMITMENT_HASH_SIZE];
        let mut deserialized_signer_response = [PolynomialRingElement::zero(); COLUMNS_IN_A];
        let mut deserialized_hint = [[0i32; COEFFICIENTS_IN_RING_ELEMENT]; ROWS_IN_A];

        match encoding::signature::deserialize::<SIMDUnit>(
            COLUMNS_IN_A,
            ROWS_IN_A,
            COMMITMENT_HASH_SIZE,
            GAMMA1_EXPONENT,
            GAMMA1_RING_ELEMENT_SIZE,
            MAX_ONES_IN_HINT,
            SIGNATURE_SIZE,
            signature_serialized,
            &mut deserialized_commitment_hash,
            &mut deserialized_signer_response,
            &mut deserialized_hint,
        ) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        // We use if-else branches because early returns will not go through hax.
        if vector_infinity_norm_exceeds::<SIMDUnit>(
            &deserialized_signer_response,
            (1 << GAMMA1_EXPONENT) - BETA,
        ) {
            return Err(VerificationError::SignerResponseExceedsBoundError);
        }
        let mut matrix = [PolynomialRingElement::<SIMDUnit>::zero(); ROW_X_COLUMN];
        Sampler::matrix_flat::<SIMDUnit>(COLUMNS_IN_A, seed_for_a, &mut matrix);

        let mut verification_key_hash = [0; BYTES_FOR_VERIFICATION_KEY_HASH];
        Shake256::shake256(verification_key, &mut verification_key_hash);

        let mut message_representative = [0; MESSAGE_REPRESENTATIVE_SIZE];
        derive_message_representative::<Shake256Xof>(
            &verification_key_hash,
            &domain_separation_context,
            message,
            &mut message_representative,
        );

        let mut verifier_challenge = PolynomialRingElement::zero();
        sample_challenge_ring_element::<SIMDUnit, Shake256>(
            &deserialized_commitment_hash,
            ONES_IN_VERIFIER_CHALLENGE,
            &mut verifier_challenge,
        );
        ntt(&mut verifier_challenge);

        // Move signer response into ntt
        for i in 0..deserialized_signer_response.len() {
            ntt(&mut deserialized_signer_response[i]);
        }
        compute_w_approx::<SIMDUnit>(
            ROWS_IN_A,
            COLUMNS_IN_A,
            &matrix,
            &deserialized_signer_response,
            &verifier_challenge,
            &mut t1,
        );

        // Compute the commitment hash again to validate the signature.
        let mut recomputed_commitment_hash = [0; COMMITMENT_HASH_SIZE];
        {
            use_hint::<SIMDUnit>(GAMMA2, &deserialized_hint, &mut t1);
            let mut commitment_serialized = [0u8; COMMITMENT_VECTOR_SIZE];
            encoding::commitment::serialize_vector::<SIMDUnit>(
                COMMITMENT_RING_ELEMENT_SIZE,
                &t1,
                &mut commitment_serialized,
            );

            let mut shake = Shake256Xof::init();
            shake.absorb(&message_representative);
            shake.absorb_final(&commitment_serialized);

            shake.squeeze(&mut recomputed_commitment_hash);
        }

        // Check if this is a valid signature by comparing the hashes.
        if deserialized_commitment_hash == recomputed_commitment_hash {
            return Ok(());
        }

        return Err(VerificationError::CommitmentHashesDontMatchError);
    }

    #[inline(always)]
    pub(crate) fn sign_pre_hashed_mut<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128: shake128::Xof,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        Shake256X4: shake256::XofX4,
        PH: PreHash,
    >(
        signing_key: &[u8],
        message: &[u8],
        context: &[u8],
        pre_hash_buffer: &mut [u8],
        randomness: [u8; SIGNING_RANDOMNESS_SIZE],
        signature: &mut [u8; SIGNATURE_SIZE],
    ) -> Result<(), SigningError> {
        if context.len() > CONTEXT_MAX_LEN {
            return Err(SigningError::ContextTooLongError);
        }
        PH::hash::<Shake128>(message, pre_hash_buffer);
        let domain_separation_context = match DomainSeparationContext::new(context, Some(PH::oid()))
        {
            Ok(dsc) => dsc,
            Err(_) => return Err(SigningError::ContextTooLongError),
        };
        sign_internal::<SIMDUnit, Sampler, Shake128X4, Shake256, Shake256Xof, Shake256X4>(
            signing_key,
            pre_hash_buffer,
            Some(domain_separation_context),
            randomness,
            signature,
        )
    }

    #[inline(always)]
    pub(crate) fn sign_pre_hashed<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128: shake128::Xof,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        Shake256X4: shake256::XofX4,
        PH: PreHash,
    >(
        signing_key: &[u8],
        message: &[u8],
        context: &[u8],
        pre_hash_buffer: &mut [u8],
        randomness: [u8; SIGNING_RANDOMNESS_SIZE],
    ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
        let mut signature = MLDSASignature::zero();

        // [eurydice] doesn't support ?
        // https://github.com/AeneasVerif/eurydice/issues/105
        match sign_pre_hashed_mut::<
            SIMDUnit,
            Sampler,
            Shake128,
            Shake128X4,
            Shake256,
            Shake256Xof,
            Shake256X4,
            PH,
        >(
            signing_key,
            message,
            context,
            pre_hash_buffer,
            randomness,
            &mut signature.value,
        ) {
            Ok(_) => Ok(signature),
            Err(e) => Err(e),
        }
    }

    #[inline(always)]
    pub(crate) fn sign_mut<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        Shake256X4: shake256::XofX4,
    >(
        signing_key: &[u8],
        message: &[u8],
        context: &[u8],
        randomness: [u8; SIGNING_RANDOMNESS_SIZE],
        signature: &mut [u8; SIGNATURE_SIZE],
    ) -> Result<(), SigningError> {
        let domain_separation_context = match DomainSeparationContext::new(context, None) {
            Ok(dsc) => dsc,
            Err(_) => return Err(SigningError::ContextTooLongError),
        };
        sign_internal::<SIMDUnit, Sampler, Shake128X4, Shake256, Shake256Xof, Shake256X4>(
            signing_key,
            message,
            Some(domain_separation_context),
            randomness,
            signature,
        )
    }

    #[inline(always)]
    pub(crate) fn sign<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        Shake256X4: shake256::XofX4,
    >(
        signing_key: &[u8],
        message: &[u8],
        context: &[u8],
        randomness: [u8; SIGNING_RANDOMNESS_SIZE],
    ) -> Result<MLDSASignature<SIGNATURE_SIZE>, SigningError> {
        let mut signature = MLDSASignature::zero();

        // [eurydice] doesn't support ?
        // https://github.com/AeneasVerif/eurydice/issues/105
        match sign_mut::<SIMDUnit, Sampler, Shake128X4, Shake256, Shake256Xof, Shake256X4>(
            signing_key,
            message,
            context,
            randomness,
            &mut signature.value,
        ) {
            Ok(_) => Ok(signature),
            Err(e) => Err(e),
        }
    }

    #[inline(always)]
    pub(crate) fn verify<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
    >(
        verification_key_serialized: &[u8; VERIFICATION_KEY_SIZE],
        message: &[u8],
        context: &[u8],
        signature_serialized: &[u8; SIGNATURE_SIZE],
    ) -> Result<(), VerificationError> {
        // We manually do the matching here to make Eurydice happy.
        let domain_separation_context = match DomainSeparationContext::new(context, None) {
            Ok(dsc) => dsc,
            Err(_) => return Err(VerificationError::VerificationContextTooLongError),
        };
        verify_internal::<SIMDUnit, Sampler, Shake128X4, Shake256, Shake256Xof>(
            verification_key_serialized,
            message,
            Some(domain_separation_context),
            signature_serialized,
        )
    }

    #[inline(always)]
    pub(crate) fn verify_pre_hashed<
        SIMDUnit: Operations,
        Sampler: X4Sampler,
        Shake128: shake128::Xof,
        Shake128X4: shake128::XofX4,
        Shake256: shake256::DsaXof,
        Shake256Xof: shake256::Xof,
        PH: PreHash,
    >(
        verification_key_serialized: &[u8; VERIFICATION_KEY_SIZE],
        message: &[u8],
        context: &[u8],
        pre_hash_buffer: &mut [u8],
        signature_serialized: &[u8; SIGNATURE_SIZE],
    ) -> Result<(), VerificationError> {
        PH::hash::<Shake128>(message, pre_hash_buffer);
        let domain_separation_context = match DomainSeparationContext::new(context, Some(PH::oid()))
        {
            Ok(dsc) => dsc,
            Err(_) => return Err(VerificationError::VerificationContextTooLongError),
        };
        verify_internal::<SIMDUnit, Sampler, Shake128X4, Shake256, Shake256Xof>(
            verification_key_serialized,
            pre_hash_buffer,
            Some(domain_separation_context),
            signature_serialized,
        )
    }
}

/// This corresponds to line 6 in algorithm 7 in FIPS 204 (line 7 in algorithm
/// 8, resp.).
///
/// If `domain_separation_context` is supplied, applies domain
/// separation and length encoding to the context string,
/// before appending the message (in the regular variant) or the
/// pre-hash OID as well as the pre-hashed message digest. Otherwise,
/// it is assumed that `message` already contains domain separation
/// information.
///
/// In FIPS 204 M' is the concatenation of the domain separated context, any
/// potential pre-hash OID and the message (or the message pre-hash). We do not
/// explicitely construct the concatenation in memory since it is of statically unknown
/// length, but feed its components directly into the incremental XOF.
///
/// Refer to line 10 of Algorithm 2 (and line 5 of Algorithm 3, resp.) in [FIPS
/// 204](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.pdf#section.5)
/// for details on the domain separation for regular ML-DSA. Line
/// 23 of Algorithm 4 (and line 18 of Algorithm 5,resp.) describe domain separation for the HashMl-DSA
/// variant.
#[inline(always)]
fn derive_message_representative<Shake256Xof: shake256::Xof>(
    verification_key_hash: &[u8],
    domain_separation_context: &Option<DomainSeparationContext>,
    message: &[u8],
    message_representative: &mut [u8; 64],
) {
    #[cfg(not(eurydice))]
    debug_assert!(verification_key_hash.len() == 64);

    let mut shake = Shake256Xof::init();
    shake.absorb(verification_key_hash);
    if let Some(domain_separation_context) = domain_separation_context {
        shake.absorb(&[domain_separation_context.pre_hash_oid().is_some() as u8]);
        shake.absorb(&[domain_separation_context.context().len() as u8]);
        shake.absorb(domain_separation_context.context());
        if let Some(pre_hash_oid) = domain_separation_context.pre_hash_oid() {
            shake.absorb(pre_hash_oid)
        }
    }

    shake.absorb_final(message);
    shake.squeeze(message_representative);
}

use crate::{
    constants::COEFFICIENTS_IN_RING_ELEMENT, encoding, polynomial::PolynomialRingElement,
    simd::traits::Operations, VerificationError,
};

#[inline(always)]
pub(crate) fn serialize<SIMDUnit: Operations>(
    commitment_hash: &[u8],
    signer_response: &[PolynomialRingElement<SIMDUnit>],
    hint: &[[i32; COEFFICIENTS_IN_RING_ELEMENT]],
    commitment_hash_size: usize,
    columns_in_a: usize,
    rows_in_a: usize,
    gamma1_exponent: usize,
    gamma1_ring_element_size: usize,
    max_ones_in_hint: usize,
    signature: &mut [u8],
) {
    let mut offset = 0;

    signature[offset..offset + commitment_hash_size].copy_from_slice(commitment_hash);
    offset += commitment_hash_size;

    for i in 0..columns_in_a {
        encoding::gamma1::serialize::<SIMDUnit>(
            &signer_response[i],
            &mut signature[offset..offset + gamma1_ring_element_size],
            gamma1_exponent,
        );
        offset += gamma1_ring_element_size;
    }

    let mut true_hints_seen = 0;

    // Unfortunately the following does not go through hax:
    //
    //     let hint_serialized = &mut signature[offset..];
    //
    // Instead, we have to mutate signature[offset + ..] directly.
    for i in 0..rows_in_a {
        // for (j, hint) in self.hint[i].into_iter().enumerate() {
        for j in 0..hint[i].len() {
            if hint[i][j] == 1 {
                signature[offset + true_hints_seen] = j as u8;
                true_hints_seen += 1;
            }
        }
        signature[offset + max_ones_in_hint + i] = true_hints_seen as u8;
    }
}

#[inline(always)]
pub(crate) fn deserialize<SIMDUnit: Operations>(
    columns_in_a: usize,
    rows_in_a: usize,
    commitment_hash_size: usize,
    gamma1_exponent: usize,
    gamma1_ring_element_size: usize,
    max_ones_in_hint: usize,
    signature_size: usize,
    serialized: &[u8],
    out_commitment_hash: &mut [u8],
    out_signer_response: &mut [PolynomialRingElement<SIMDUnit>],
    out_hint: &mut [[i32; COEFFICIENTS_IN_RING_ELEMENT]],
) -> Result<(), VerificationError> {
    #[cfg(not(eurydice))]
    debug_assert!(serialized.len() == signature_size);

    let (commitment_hash, rest_of_serialized) = serialized.split_at(commitment_hash_size);
    out_commitment_hash[0..commitment_hash_size].copy_from_slice(commitment_hash);

    let (signer_response_serialized, hint_serialized) =
        rest_of_serialized.split_at(gamma1_ring_element_size * columns_in_a);

    for i in 0..columns_in_a {
        encoding::gamma1::deserialize::<SIMDUnit>(
            gamma1_exponent,
            &signer_response_serialized
                [i * gamma1_ring_element_size..(i + 1) * gamma1_ring_element_size],
            &mut out_signer_response[i],
        );
    }

    // While there are several ways to encode the same hint vector, we
    // allow only one such encoding, to ensure strong unforgeability.
    let mut previous_true_hints_seen = 0usize;

    // XXX: We would like to use early returns below, but doing so triggers a bug in
    // Eurdice leads to a bad extraction.
    let mut malformed_hint = false;

    for i in 0..rows_in_a {
        let current_true_hints_seen = hint_serialized[max_ones_in_hint + i] as usize;

        if (current_true_hints_seen < previous_true_hints_seen)
            || (current_true_hints_seen > max_ones_in_hint)
        {
            // the true hints seen should be increasing
            // return Err(VerificationError::MalformedHintError);
            malformed_hint = true;
            break;
        }

        for j in previous_true_hints_seen..current_true_hints_seen {
            if j > previous_true_hints_seen && hint_serialized[j] <= hint_serialized[j - 1] {
                // indices of true hints for a specific polynomial should be
                // increasing
                // return Err(VerificationError::MalformedHintError);
                malformed_hint = true;
                break;
            }

            set_hint(out_hint, i, hint_serialized[j] as usize);
        }

        if malformed_hint {
            break;
        } else {
            previous_true_hints_seen = current_true_hints_seen;
        }
    }

    for j in previous_true_hints_seen..max_ones_in_hint {
        if hint_serialized[j] != 0 {
            // ensures padding indices are zero
            // return Err(VerificationError::MalformedHintError);
            malformed_hint = true;
            break;
        }
    }

    if malformed_hint {
        Err(VerificationError::MalformedHintError)
    } else {
        Ok(())
    }
}

#[inline(always)]
fn set_hint(out_hint: &mut [[i32; 256]], i: usize, j: usize) {
    out_hint[i][j] = 1
}

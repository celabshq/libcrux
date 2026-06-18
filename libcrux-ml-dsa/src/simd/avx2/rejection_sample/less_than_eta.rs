use libcrux_intrinsics::avx2::*;
use libcrux_secrets::mem_requests::ct_declassify;

use crate::simd::avx2::{encoding, rejection_sample::shuffle_table::SHUFFLE_TABLE, Eta};

// TODO: This code seems to slow the implementation down, but stabilizes
// benchmarks. Revisit this once the other functions are vectorized.

#[inline(always)]
fn shift_interval<const ETA: usize>(coefficients: Vec256) -> Vec256 {
    match ETA as u8 {
        2 => {
            let quotient = mm256_mullo_epi32(coefficients, mm256_set1_epi32(26));
            let quotient = mm256_srai_epi32::<7>(quotient);
            let quotient = mm256_mullo_epi32(quotient, mm256_set1_epi32(5));

            let coefficients_mod_5 = mm256_sub_epi32(coefficients, quotient);

            mm256_sub_epi32(mm256_set1_epi32(ETA as i32), coefficients_mod_5)
        }

        4 => mm256_sub_epi32(mm256_set1_epi32(ETA as i32), coefficients),
        _ => unreachable!(),
    }
}

#[inline(always)]
pub(crate) fn sample<const ETA: usize>(input: &[u8], output: &mut [i32]) -> usize {
    // Whether or not ETA is 2 or 4, we always split the input bytestream into
    // values that are 4-bits wide.
    let potential_coefficients = encoding::error::deserialize_to_unsigned(Eta::Four, input);

    let interval_boundary: i32 = match ETA as u8 {
        2 => 15,
        4 => 9,
        _ => unreachable!(),
    };

    let compare_with_interval_boundary =
        mm256_cmpgt_epi32(mm256_set1_epi32(interval_boundary), potential_coefficients);

    // Declassification: The subsequent operation may leak the
    // rejection decision for these coefficients. The Dilithium
    // Specification for Round 3 of the NIST Post-Quantum
    // Cryptography Standardization Process states that:
    //
    // "When performing rejection sampling, our code reveals which of the
    // conditions was the reason for the rejection, ..."
    //
    // and that doing so is safe (Section 5.5,
    // https://pq-crystals.org/dilithium/data/dilithium-specification-round3-20210208.pdf).
    // However, there is some ambiguity whether this is referring
    // only to rejection sampling during signature
    // generation. Other implementations of ML-DSA, including the
    // reference implementation, do not go out of their way to
    // avoid this secret-dependent behaviour and are offered as
    // evidence that this is safe to do:
    //
    // - mldsa-native: https://github.com/pq-code-package/mldsa-native/blob/0591fe06832418f8a320d5a1533327df063185cf/dev/x86_64/meta.h#L130
    // - Dilithium reference implementation: https://github.com/pq-crystals/dilithium/blob/6e00625c5b29f516c6de973fe2ee2fbb150973f9/avx2/rejsample.c#L312
    //
    // It should be noted that the declassification of the comparison
    // result here does not (and must not) declassify the sampled
    // coefficients.
    //
    // While we do not consider protection against power
    // side-channels in scope for libcrux, at this point, a
    // hardened implementation of sampling mod p can be found in
    // Azouaoui et al, "Levelling Dilithium Against Leakage",
    // section 4.4
    // (https://csrc.nist.gov/csrc/media/Events/2022/fourth-pqc-standardization-conference/documents/papers/leveling-dilithium-against-leakage-pqc2022.pdf).
    ct_declassify(&compare_with_interval_boundary);

    // Since every bit in each lane is either 0 or all 1s, we only need one bit
    // from each lane to tell us what coefficients to keep and what to throw-away.
    // Combine all the bits (there are 8) into one byte.
    let good = mm256_movemask_ps(mm256_castsi256_ps(compare_with_interval_boundary));

    let good_lower_half = good & 0x0F;
    let good_upper_half = good >> 4;

    // Now move all the coefficients into the signed interval, some of the
    // coefficients will be rejected, so the calculations in some lanes might be
    // wasted, but this is probably faster than splitting into 2 128-bit registers,
    // rejecting, combining them back, moving the cofficients, and then combining
    // them back again.
    let shifted = shift_interval::<ETA>(potential_coefficients);

    // Each bit (and its corresponding position) represents an element we
    // want to sample. We'd like all such elements to be next to each other starting
    // at index 0, so that they can be read from the vector easily.
    // |REJECTION_SAMPLE_SHUFFLE_TABLE| encodes the byte-level shuffling indices
    // needed to make this happen.
    //
    // For e.g. if the lower 4 bits of good = 0b_0_0_1_0, we need to move the
    // element in the 2-nd 32-bit lane to the first. To do this, we need the
    // byte-level shuffle indices to be 2 3 4 5 X X ...
    let lower_shuffles = SHUFFLE_TABLE[good_lower_half as usize];

    // Shuffle the lower 4 32-bits accordingly ...
    let lower_shuffles = mm_loadu_si128(&lower_shuffles);
    let lower_coefficients = mm256_castsi256_si128(shifted);
    let lower_coefficients = mm_shuffle_epi8(lower_coefficients, lower_shuffles);

    // ... then write them out ...
    mm_storeu_si128_i32(&mut output[0..4], lower_coefficients);

    // ... and finally count the number of bits of |good_lower_half| so we know
    // how many were actually sampled
    let sampled_count = good_lower_half.count_ones() as usize;

    // Do the same for |good_upper_half|
    let upper_shuffles = SHUFFLE_TABLE[good_upper_half as usize];
    let upper_shuffles = mm_loadu_si128(&upper_shuffles);
    let upper_coefficients = mm256_extracti128_si256::<1>(shifted);
    let upper_coefficients = mm_shuffle_epi8(upper_coefficients, upper_shuffles);

    mm_storeu_si128_i32(
        &mut output[sampled_count..sampled_count + 4],
        upper_coefficients,
    );

    sampled_count + (good_upper_half.count_ones() as usize)
}

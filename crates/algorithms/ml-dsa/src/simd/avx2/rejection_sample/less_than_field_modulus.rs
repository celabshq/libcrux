use crate::simd::{avx2::rejection_sample::shuffle_table::SHUFFLE_TABLE, traits::FIELD_MODULUS};

use libcrux_intrinsics::avx2::*;

// Partition a stream of bytes into 24-bit values, and then clear the most
// significant bit to turn them into 23-bit ones.
#[inline(always)]
fn bytestream_to_potential_coefficients(serialized: &[u8]) -> Vec256 {
    #[cfg(not(eurydice))]
    debug_assert_eq!(serialized.len(), 24);

    let mut serialized_extended = [0u8; 32];
    serialized_extended[..24].copy_from_slice(serialized);

    const COEFFICIENT_MASK: i32 = (1 << 23) - 1;

    let coefficients = mm256_loadu_si256_u8(&serialized_extended);
    let coefficients =
        mm256_permutevar8x32_epi32(coefficients, mm256_set_epi32(0, 5, 4, 3, 0, 2, 1, 0));

    let coefficients = mm256_shuffle_epi8(
        coefficients,
        mm256_set_epi8(
            -1, 11, 10, 9, -1, 8, 7, 6, -1, 5, 4, 3, -1, 2, 1, 0, -1, 11, 10, 9, -1, 8, 7, 6, -1,
            5, 4, 3, -1, 2, 1, 0,
        ),
    );

    mm256_and_si256(coefficients, mm256_set1_epi32(COEFFICIENT_MASK))
}

#[inline(always)]
pub(crate) fn sample(input: &[u8], output: &mut [i32]) -> usize {
    let field_modulus = mm256_set1_epi32(FIELD_MODULUS);

    // The input bytes can be interpreted as a sequence of serialized
    // 23-bit (i.e. uncompressed) coefficients. Not all coefficients may be
    // less than FIELD_MODULUS though.
    let potential_coefficients = bytestream_to_potential_coefficients(input);

    // Suppose we view |potential_coefficients| as follows (clumping bits together
    // in groups of 32):
    //
    // A B C D | E F G H ....
    //
    // and A < |FIELD_MODULUS|, D < |FIELD_MODULUS| and H < |FIELD_MODULUS|, |compare_with_field_modulus| will look like:
    //
    // 0xFF..FF 0 0 0xFF..FF | 0 0 0 0xFF..FF | ...
    let compare_with_field_modulus = mm256_cmpgt_epi32(field_modulus, potential_coefficients);

    // Since every bit in each lane is either 0 or all 1s, we only need one bit
    // from each lane to tell us what coefficients to keep and what to throw-away.
    // Combine all the bits (there are 8) into one byte.
    let good = mm256_movemask_ps(mm256_castsi256_ps(compare_with_field_modulus));

    let good_lower_half = good & 0x0F;
    let good_upper_half = good >> 4;

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
    let lower_coefficients = mm256_castsi256_si128(potential_coefficients);
    let lower_coefficients = mm_shuffle_epi8(lower_coefficients, lower_shuffles);

    // ... then write them out ...
    mm_storeu_si128_i32(&mut output[0..4], lower_coefficients);

    // ... and finally count the number of bits of |good_lower_half| so we know
    // how many were actually sampled
    let sampled_count = good_lower_half.count_ones() as usize;

    // Do the same for |good_upper_half|
    let upper_shuffles = SHUFFLE_TABLE[good_upper_half as usize];
    let upper_shuffles = mm_loadu_si128(&upper_shuffles);
    let upper_coefficients = mm256_extracti128_si256::<1>(potential_coefficients);
    let upper_coefficients = mm_shuffle_epi8(upper_coefficients, upper_shuffles);

    mm_storeu_si128_i32(
        &mut output[sampled_count..sampled_count + 4],
        upper_coefficients,
    );

    sampled_count + (good_upper_half.count_ones() as usize)
}

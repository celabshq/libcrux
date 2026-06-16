use crate::simd::avx2::{encoding, rejection_sample::shuffle_table::SHUFFLE_TABLE, Eta};

use libcrux_intrinsics::avx2::*;

// TODO: This code seems to slow the implementation down, but stabilizes
// benchmarks. Revisit this once the other functions are vectorized.

#[inline(always)]
#[hax_lib::requires(ETA == 2 || ETA == 4)]
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
#[hax_lib::requires((ETA == 2 || ETA == 4) && input.len() == 4 && output.len() >= 8)]
#[hax_lib::ensures(|result| future(output).len() == output.len() && result <= 8)]
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

    // Since every bit in each lane is either 0 or all 1s, we only need one bit
    // from each lane to tell us what coefficients to keep and what to throw-away.
    // Combine all the bits (there are 8) into one byte.
    let good = mm256_movemask_ps(mm256_castsi256_ps(compare_with_interval_boundary));

    let good_lower_half = good & 0x0F;
    let good_upper_half = good >> 4;

    hax_lib::fstar!(
        r#"Libcrux_ml_dsa.Proof_utils.lemma_movemask_ps_bound (Libcrux_intrinsics.Avx2.mm256_castsi256_ps $compare_with_interval_boundary);
           logand_mask_lemma $good 4;
           assert (v $good_lower_half >= 0 /\ v $good_lower_half < 16);
           assert (v $good_upper_half >= 0 /\ v $good_upper_half < 16);
           Libcrux_ml_dsa.Proof_utils.lemma_count_ones_nibble $good_lower_half;
           Libcrux_ml_dsa.Proof_utils.lemma_count_ones_nibble $good_upper_half"#
    );

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

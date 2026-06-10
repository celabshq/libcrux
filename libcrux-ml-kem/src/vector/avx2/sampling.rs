use super::{
    super::{rej_sample_table::REJECTION_SAMPLE_SHUFFLE_TABLE, traits::FIELD_MODULUS},
    serialize::{deserialize_12, serialize_1},
    *,
};

#[hax_lib::fstar::before(
    r#"
(* Trusted axiom (Track I, 2026-06-10): semantics of the dynamic-mask byte
   shuffle. `BitVec.Intrinsics.mm_shuffle_epi8`'s tactic routes masks that are
   not `mm_set_epi8` literals (such as the `mm_loadu_si128`-loaded
   REJECTION_SAMPLE_SHUFFLE_TABLE rows below) to the uninterpreted
   `BitVec.Intrinsics.mm_shuffle_epi8_no_semantics`. This axiom gives that
   symbol the PSHUFB hardware semantics, transcribed from the executable
   core-models reference `crates/utils/core-models/src/core_arch/x86.rs`
   (`extra::mm_shuffle_epi8_u8_array`, the model behind
   `ssse3::_mm_shuffle_epi8`):

     result bit i = let nth = i / 8 in
                    let idx = byte `nth` of the mask (bits LSB-first) in
                    if idx > 127 then 0 else a ((idx % 16) * 8 + i % 8)

   Validated against core-models by the differential test
   `track_i_axiom_transcription_tests::shuffle_epi8_dynamic_mask_formula` in
   `crates/utils/core-models/src/core_arch/x86/interpretations.rs`. Kept
   ml-kem-local (not in the shared BitVec.Intrinsics.fsti) to avoid a
   stale-cascade into the sha3 / ml-dsa proof trees. *)
assume val mm_shuffle_epi8_no_semantics_lemma (a b: bit_vec 128) (i: nat{i < 128})
  : Lemma
    (BitVec.Intrinsics.mm_shuffle_epi8_no_semantics a b i ==
      (let nth = i / 8 in
       let idx: nat =
         b (8 * nth) + 2 * b (8 * nth + 1) + 4 * b (8 * nth + 2) + 8 * b (8 * nth + 3) +
         16 * b (8 * nth + 4) + 32 * b (8 * nth + 5) + 64 * b (8 * nth + 6) +
         128 * b (8 * nth + 7)
       in
       if idx > 127 then 0 else a ((idx % 16) * 8 + i % 8)))
"#
)]
#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(input.len() == 24 && output.len() == 16)]
#[hax_lib::ensures(|result| (future(output).len() == 16 && result <= 16).to_prop().and(
            hax_lib::forall(|j: usize|
                hax_lib::implies(j < result,
                    future(output)[j] >= 0 && future(output)[j] <= 3328))))]
pub(crate) fn rejection_sample(input: &[u8], output: &mut [i16]) -> usize {
    let field_modulus = mm256_set1_epi16(FIELD_MODULUS);

    // The input bytes can be interpreted as a sequence of serialized
    // 12-bit (i.e. uncompressed) coefficients. Not all coefficients may be
    // less than FIELD_MODULUS though.
    let potential_coefficients = deserialize_12(input);

    // Suppose we view |potential_coefficients| as follows (grouping 64-bit elements):
    //
    // A B C D | E F G H | ....
    //
    // and A < 3329, D < 3329 and H < 3329, |compare_with_field_modulus| will look like:
    //
    // 0xFF 0 0 0xFF | 0 0 0 0xFF | ...
    let compare_with_field_modulus = mm256_cmpgt_epi16(field_modulus, potential_coefficients);

    // Since every bit in each lane is either 0 or 1, we only need one bit from
    // each lane in the register to tell us what coefficients to keep and what
    // to throw-away. Combine all the bits (there are 16) into two bytes.
    let good = serialize_1(compare_with_field_modulus);

    // Each bit (and its corresponding position) represents an element we
    // want to sample. We'd like all such elements to be next to each other starting
    // at index 0, so that they can be read from the vector easily.
    // |REJECTION_SAMPLE_SHUFFLE_TABLE| encodes the byte-level shuffling indices
    // needed to make this happen.
    //
    // For e.g. if good[0] = 0b0_0_0_0_0_0_1_0, we need to move the element in
    // the 2-nd 16-bit lane to the first. To do this, we need the byte-level
    // shuffle indices to be 2 3 X X X X ...
    let lower_shuffles = REJECTION_SAMPLE_SHUFFLE_TABLE[good[0] as usize];

    // Shuffle the lower 8 16-bits accordingly ...
    let lower_shuffles = mm_loadu_si128(&lower_shuffles);
    let lower_coefficients = mm256_castsi256_si128(potential_coefficients);
    let lower_coefficients = mm_shuffle_epi8(lower_coefficients, lower_shuffles);

    // ... then write them out ...
    mm_storeu_si128(output, lower_coefficients);

    // ... and finally count the number of bits of |good[0]| so we know how many
    // were actually sampled
    let sampled_count = good[0].count_ones() as usize;
    // Do the same for |goood[1]|
    let upper_shuffles = REJECTION_SAMPLE_SHUFFLE_TABLE[good[1] as usize];
    let upper_shuffles = mm_loadu_si128(&upper_shuffles);
    let upper_coefficients = mm256_extracti128_si256::<1>(potential_coefficients);
    let upper_coefficients = mm_shuffle_epi8(upper_coefficients, upper_shuffles);

    mm_storeu_si128(
        &mut output[sampled_count..sampled_count + 8],
        upper_coefficients,
    );

    sampled_count + (good[1].count_ones() as usize)
}

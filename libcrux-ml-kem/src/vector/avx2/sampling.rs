use super::{
    super::{rej_sample_table::REJECTION_SAMPLE_SHUFFLE_TABLE, traits::FIELD_MODULUS},
    serialize::{deserialize_12, serialize_1},
    *,
};

#[hax_lib::fstar::before(
    r#"
open Libcrux_intrinsics.Avx2
open Libcrux_intrinsics.Avx2_ml_kem_views
open Libcrux_ml_kem.Vector.Avx2.Sampling_theory
"#
)]
#[inline(always)]
#[hax_lib::requires(input.len() == 24 && output.len() == 16)]
#[hax_lib::ensures(|result| (future(output).len() == 16 && result <= 16).to_prop().and(
            hax_lib::forall(|j: usize|
                hax_lib::implies(j < result,
                    future(output)[j] >= 0 && future(output)[j] <= 3328))))]
#[cfg_attr(
    hax,
    hax_lib::fstar::before(r#"#restart-solver"#),
    hax_lib::fstar::options("--z3rlimit 300 --split_queries always")
)]
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
    // (`_vec` / `_shuffled` names keep the table row and the pre-shuffle
    // coefficients nameable in the proof blocks below — rename only.)
    let lower_shuffles_vec = mm_loadu_si128(&lower_shuffles);
    let lower_coefficients = mm256_castsi256_si128(potential_coefficients);
    let lower_shuffled = mm_shuffle_epi8(lower_coefficients, lower_shuffles_vec);

    // ... then write them out ...
    // Proof-only snapshot of the buffer before the first store, so the store
    // bridge lemma below can name its pre-state operand (the `mm_storeu_si128`
    // call rebinds `output`).
    #[cfg(hax)]
    let output_before_lower: &[i16] = &*output;

    mm_storeu_si128(output, lower_shuffled);

    proof!(
        r#"
        let g0: nat = v (${good}.[ mk_usize 0 ] <: u8) in
        introduce forall (l: nat{l < 16}).
            Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${compare_with_field_modulus} (16 * l) ==
            (if 3329 > v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${potential_coefficients}) l)
             then 1 else 0)
        with (
          Libcrux_intrinsics.Avx2_ml_kem_views.lemma_bv_bit0_mm256_cmpgt_epi16 ${field_modulus} ${potential_coefficients} l;
          assert (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${field_modulus}) l ==
                  Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS)
        );
        Hacspec_ml_kem.Commute.Rej_table.lemma_good_bits ${good} ${compare_with_field_modulus} ${potential_coefficients} 0;
        Hacspec_ml_kem.Commute.Rej_table.intro_top_bits_clear ${potential_coefficients};
        assert (${lower_shuffles} ==
            Seq.index Libcrux_ml_kem.Vector.Rej_sample_table.v_REJECTION_SAMPLE_SHUFFLE_TABLE g0);
        lemma_half_done ${potential_coefficients} ${lower_coefficients} ${lower_shuffles_vec} ${lower_shuffled} ${lower_shuffles} 0 g0;
        (* The store bridge: lane j of the stored buffer IS lane j of the
           shuffled vector's i16x8 view.  Over core-models the store is a
           modeled per-lane spine, so this is a proven companion lemma. *)
        Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm_storeu_si128 ${output_before_lower} ${lower_shuffled};
        assert (forall (j: nat{j < 8}).
            j < Hacspec_ml_kem.Commute.Rej_table.popcount8 g0 ==>
            (v (Seq.index ${output} j) >= 0 /\ v (Seq.index ${output} j) <= 3328))
    "#
    );

    // ... and finally count the number of bits of |good[0]| so we know how many
    // were actually sampled
    let sampled_count = good[0].count_ones() as usize;
    // Do the same for |goood[1]|
    let upper_shuffles = REJECTION_SAMPLE_SHUFFLE_TABLE[good[1] as usize];
    let upper_shuffles_vec = mm_loadu_si128(&upper_shuffles);
    let upper_coefficients = mm256_extracti128_si256::<1>(potential_coefficients);
    let upper_shuffled = mm_shuffle_epi8(upper_coefficients, upper_shuffles_vec);

    // Proof-only snapshot of the buffer between the two stores.
    #[cfg(hax)]
    let output_after_lower: &[i16] = &*output;

    mm_storeu_si128(
        &mut output[sampled_count..sampled_count + 8],
        upper_shuffled,
    );

    let result = sampled_count + (good[1].count_ones() as usize);

    proof!(
        r#"
        let g0: nat = v (${good}.[ mk_usize 0 ] <: u8) in
        let g1: nat = v (${good}.[ mk_usize 1 ] <: u8) in
        count_ones_u8_popcount8 (${good}.[ mk_usize 0 ] <: u8);
        count_ones_u8_popcount8 (${good}.[ mk_usize 1 ] <: u8);
        Hacspec_ml_kem.Commute.Rej_table.lemma_popcount8_u8 g0;
        Hacspec_ml_kem.Commute.Rej_table.lemma_popcount8_u8 g1;
        let c0: nat = Hacspec_ml_kem.Commute.Rej_table.popcount8 g0 in
        let c1: nat = Hacspec_ml_kem.Commute.Rej_table.popcount8 g1 in
        assert (v ${sampled_count} == c0);
        assert (v ${result} == c0 + c1);
        (* upper-half driver *)
        introduce forall (l: nat{l < 16}).
            Libcrux_intrinsics.Avx2_ml_kem_views.bv_bit ${compare_with_field_modulus} (16 * l) ==
            (if 3329 > v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${potential_coefficients}) l)
             then 1 else 0)
        with (
          Libcrux_intrinsics.Avx2_ml_kem_views.lemma_bv_bit0_mm256_cmpgt_epi16 ${field_modulus} ${potential_coefficients} l;
          assert (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 ${field_modulus}) l ==
                  Libcrux_ml_kem.Vector.Traits.v_FIELD_MODULUS)
        );
        Hacspec_ml_kem.Commute.Rej_table.lemma_good_bits ${good} ${compare_with_field_modulus} ${potential_coefficients} 1;
        Hacspec_ml_kem.Commute.Rej_table.intro_top_bits_clear ${potential_coefficients};
        assert (${upper_shuffles} ==
            Seq.index Libcrux_ml_kem.Vector.Rej_sample_table.v_REJECTION_SAMPLE_SHUFFLE_TABLE g1);
        lemma_half_done ${potential_coefficients} ${upper_coefficients} ${upper_shuffles_vec} ${upper_shuffled} ${upper_shuffles} 1 g1;
        (* output indexing through the second store *)
        let range = ({ Core_models.Ops.Range.f_start = ${sampled_count};
                      Core_models.Ops.Range.f_end = ${sampled_count} +! mk_usize 8 }
                      <: Core_models.Ops.Range.t_Range usize) in
        let s' = Libcrux_intrinsics.Avx2.mm_storeu_si128
                   ((${output_after_lower}.[ range ] <: t_Slice i16)) ${upper_shuffled} in
        assert (${output} ==
            Rust_primitives.Hax.Monomorphized_update_at.update_at_range ${output_after_lower} range s');
        assert (Seq.length ${output} == 16);
        Hacspec_ml_kem.Commute.Rej_table.lemma_popcount8_le 8 g0;
        Hacspec_ml_kem.Commute.Rej_table.lemma_popcount8_le 8 g1;
        (* Frame the second store: below c0 the buffer is untouched, inside
           [c0, c0+8) it is s'.  Then the store bridge turns s' into the
           upper shuffled vector's i16x8 view, whose kept lanes lemma_half_done
           has already bounded. *)
        Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range
            ${output_after_lower} range s';
        Libcrux_intrinsics.Avx2_ml_kem_views.lemma_mm_storeu_si128
            ((${output_after_lower}.[ range ] <: t_Slice i16)) ${upper_shuffled};
        introduce forall (j: nat{j < v ${result}}).
            (v (Seq.index ${output} j) >= 0 /\ v (Seq.index ${output} j) <= 3328)
        with begin
          if j < c0 then () else assert (j - c0 < c1)
        end
    "#
    );

    result
}

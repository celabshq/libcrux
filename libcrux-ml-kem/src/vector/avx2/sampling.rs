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

(* Trusted axiom (Track I M2, 2026-06-10): `u8::count_ones` counts set bits.
   `Rust_primitives.Arithmetic.count_ones_u8` is an uninterpreted `val` in
   hax-lib (only `v r <= 8` is known); this axiom gives it popcount
   semantics via the bit recursion `popcount8 g = if g = 0 then 0 else
   g % 2 + popcount8 (g / 2)` (Hacspec_ml_kem.Commute.Rej_table.popcount8).
   Validated exhaustively (x in 0..=255) against the executable
   `u8::count_ones` by the core-models test
   `track_i_axiom_transcription_tests::count_ones_popcount8_formula` in
   `crates/utils/core-models/src/core_arch/x86/interpretations.rs`. *)
assume val count_ones_u8_popcount8 (x: u8)
  : Lemma (v (Rust_primitives.Arithmetic.count_ones_u8 x) ==
           Hacspec_ml_kem.Commute.Rej_table.popcount8 (v x))

(* Seal the trusted shuffle semantics into the
   Hacspec_ml_kem.Commute.Rej_table.shuffle_semantics atom in its own
   context (the raw per-bit forall must not leak into any consumer VC). *)
let lemma_shuffle_semantics_of_axiom (a mask res: bit_vec 128)
  : Lemma
    (requires res == BitVec.Intrinsics.mm_shuffle_epi8_no_semantics a mask)
    (ensures Hacspec_ml_kem.Commute.Rej_table.shuffle_semantics a mask res)
  = Classical.forall_intro (mm_shuffle_epi8_no_semantics_lemma a mask);
    Hacspec_ml_kem.Commute.Rej_table.intro_shuffle_semantics a mask res

(* Driver: every kept lane (j < popcount8 g) of a shuffled half is in
   [0, 3328].  Establishes the sealed atoms of
   Hacspec_ml_kem.Commute.Rej_table (shuffle_semantics via the axiom
   above, mask/row/half links from the term equalities) and composes the
   clean-context per-lane lemma lemma_half_lane_bounded. *)
#push-options "--z3rlimit 300 --split_queries always --z3refresh"
let lemma_half_done
    (potential: bit_vec 256) (a mask res: bit_vec 128)
    (row: t_Array u8 (mk_usize 16)) (half: nat{half <= 1}) (g: nat{g < 256})
  : Lemma
    (requires
      res == BitVec.Intrinsics.mm_shuffle_epi8_no_semantics a mask /\
      mask == BitVec.Intrinsics.mm_loadu_si128 row /\
      row ==
      Seq.index Libcrux_ml_kem.Vector.Rej_sample_table.v_REJECTION_SAMPLE_SHUFFLE_TABLE g /\
      (half == 0 ==> a == BitVec.Intrinsics.mm256_castsi256_si128 potential) /\
      (half == 1 ==> a == BitVec.Intrinsics.mm256_extracti128_si256 (mk_i32 1) potential) /\
      Hacspec_ml_kem.Commute.Rej_table.top_bits_clear potential /\
      Hacspec_ml_kem.Commute.Rej_table.good_bits g potential half)
    (ensures
      forall (j: nat{j < 8}).
        j < Hacspec_ml_kem.Commute.Rej_table.popcount8 g ==>
        (v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 res) j) >= 0 /\
         v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 res) j) <= 3328))
  = lemma_shuffle_semantics_of_axiom a mask res;
    Hacspec_ml_kem.Commute.Rej_table.lemma_mask_of_row_loadu mask row;
    Hacspec_ml_kem.Commute.Rej_table.intro_row_of_table row g;
    Hacspec_ml_kem.Commute.Rej_table.lemma_half_of_cast a potential half;
    introduce forall (j: nat{j < 8}).
        j < Hacspec_ml_kem.Commute.Rej_table.popcount8 g ==>
        (v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 res) j) >= 0 /\
         v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 res) j) <= 3328)
    with introduce j < Hacspec_ml_kem.Commute.Rej_table.popcount8 g ==>
        (v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 res) j) >= 0 /\
         v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 res) j) <= 3328)
    with _. Hacspec_ml_kem.Commute.Rej_table.lemma_half_lane_bounded potential a mask res row half g j
#pop-options
"#
)]
#[inline(always)]
#[hax_lib::requires(input.len() == 24 && output.len() == 16)]
#[hax_lib::ensures(|result| (future(output).len() == 16 && result <= 16).to_prop().and(
            hax_lib::forall(|j: usize|
                hax_lib::implies(j < result,
                    future(output)[j] >= 0 && future(output)[j] <= 3328))))]
#[cfg_attr(hax, hax_lib::fstar::options("--z3rlimit 300 --split_queries always --z3refresh"))]
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
    mm_storeu_si128(output, lower_shuffled);

    hax_lib::fstar!(
        r#"
        let g0: nat = v (${good}.[ mk_usize 0 ] <: u8) in
        assert (forall (i: nat{i < 256}).
            ${compare_with_field_modulus} i ==
            (if 3329 > v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${potential_coefficients}) (i / 16))
             then 1 else 0));
        Hacspec_ml_kem.Commute.Rej_table.lemma_good_bits ${good} ${compare_with_field_modulus} ${potential_coefficients} 0;
        Hacspec_ml_kem.Commute.Rej_table.intro_top_bits_clear ${potential_coefficients};
        assert (${lower_shuffles} ==
            Seq.index Libcrux_ml_kem.Vector.Rej_sample_table.v_REJECTION_SAMPLE_SHUFFLE_TABLE g0);
        assert (${lower_shuffled} ==
            BitVec.Intrinsics.mm_shuffle_epi8_no_semantics ${lower_coefficients} ${lower_shuffles_vec});
        lemma_half_done ${potential_coefficients} ${lower_coefficients} ${lower_shuffles_vec} ${lower_shuffled} ${lower_shuffles} 0 g0;
        introduce forall (j: nat{j < 8}).
            Seq.index ${output} j == Seq.index (Libcrux_intrinsics.Avx2_extract.vec128_as_i16x8 ${lower_shuffled}) j
        with FStar.Seq.lemma_index_slice ${output} 0 8 j;
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

    hax_lib::fstar!(
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
        assert (forall (i: nat{i < 256}).
            ${compare_with_field_modulus} i ==
            (if 3329 > v (Seq.index (Libcrux_intrinsics.Avx2_extract.vec256_as_i16x16 ${potential_coefficients}) (i / 16))
             then 1 else 0));
        Hacspec_ml_kem.Commute.Rej_table.lemma_good_bits ${good} ${compare_with_field_modulus} ${potential_coefficients} 1;
        Hacspec_ml_kem.Commute.Rej_table.intro_top_bits_clear ${potential_coefficients};
        assert (${upper_shuffles} ==
            Seq.index Libcrux_ml_kem.Vector.Rej_sample_table.v_REJECTION_SAMPLE_SHUFFLE_TABLE g1);
        assert (${upper_shuffled} ==
            BitVec.Intrinsics.mm_shuffle_epi8_no_semantics ${upper_coefficients} ${upper_shuffles_vec});
        lemma_half_done ${potential_coefficients} ${upper_coefficients} ${upper_shuffles_vec} ${upper_shuffled} ${upper_shuffles} 1 g1;
        (* output indexing through the second store *)
        let range = { Core_models.Ops.Range.f_start = ${sampled_count};
                      Core_models.Ops.Range.f_end = ${sampled_count} +! mk_usize 8 } in
        let s' = Libcrux_intrinsics.Avx2_extract.mm_storeu_si128
                   ((${output_after_lower}.[ range ] <: t_Slice i16)) ${upper_shuffled} in
        assert (${output} ==
            Rust_primitives.Hax.Monomorphized_update_at.update_at_range ${output_after_lower} range s');
        assert (Seq.length ${output} == 16);
        introduce forall (j: nat{j < v ${result}}).
            (v (Seq.index ${output} j) >= 0 /\ v (Seq.index ${output} j) <= 3328)
        with begin
          if j < c0
          then begin
            FStar.Seq.lemma_index_slice ${output} 0 c0 j;
            FStar.Seq.lemma_index_slice ${output_after_lower} 0 c0 j
          end
          else begin
            FStar.Seq.lemma_index_slice ${output} c0 (c0 + 8) (j - c0);
            FStar.Seq.lemma_index_slice s' 0 8 (j - c0)
          end
        end
    "#
    );

    result
}

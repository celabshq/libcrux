use crate::simd::{avx2::rejection_sample::shuffle_table::SHUFFLE_TABLE, traits::FIELD_MODULUS};

use libcrux_intrinsics::avx2::*;

// Partition a stream of bytes into 24-bit values, and then clear the most
// significant bit to turn them into 23-bit ones.
#[inline(always)]
#[hax_lib::requires(serialized.len() == 24)]
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
#[hax_lib::fstar::options("--z3rlimit 400 --fuel 1 --ifuel 2")]
#[hax_lib::fstar::before(r#"
#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"
open FStar.Mul
open Core_models
open Spec.Intrinsics
open Libcrux_core_models.Abstractions.Bit
open Libcrux_ml_dsa.Simd.Avx2.Rejection_sample.Proof_helpers

module I = Libcrux_intrinsics.Avx2
module ST = Libcrux_ml_dsa.Simd.Avx2.Rejection_sample.Shuffle_table
module R = Core_models.Ops.Range
module M = Spec.MLDSA.Math

(* =============== Layer A (field_modulus): bytestream gather == spec coeff =============== *)
(* the leaf's two control vectors *)
unfold let cpm : bv256 = I.mm256_set_epi32 (mk_i32 0) (mk_i32 5) (mk_i32 4) (mk_i32 3)
                                           (mk_i32 0) (mk_i32 2) (mk_i32 1) (mk_i32 0)
unfold let csh : bv256 =
  I.mm256_set_epi8 (mk_i8 (-1)) (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 (-1)) (mk_i8 8) (mk_i8 7) (mk_i8 6)
                   (mk_i8 (-1)) (mk_i8 5) (mk_i8 4) (mk_i8 3) (mk_i8 (-1)) (mk_i8 2) (mk_i8 1) (mk_i8 0)
                   (mk_i8 (-1)) (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 (-1)) (mk_i8 8) (mk_i8 7) (mk_i8 6)
                   (mk_i8 (-1)) (mk_i8 5) (mk_i8 4) (mk_i8 3) (mk_i8 (-1)) (mk_i8 2) (mk_i8 1) (mk_i8 0)

let gathered (c0: bv256) : bv256 =
  I.mm256_shuffle_epi8 (I.mm256_permutevar8x32_epi32 c0 cpm) csh

(* gather core: coeff_2 bit (32j+8r+s) == c0 bit (24j+8r+s), all lanes j<8 *)
#push-options "--z3rlimit 400 --ifuel 3"
let lemma_gather (c0: bv256) (j: nat{j<8}) (r: nat{r<3}) (s: nat{s<8})
  : Lemma ((gathered c0).(mk_int (32*j+8*r+s)) == c0.(mk_int (24*j+8*r+s))) =
  ()
#pop-options

let v_MASK : i32 = (mk_i32 1 <<! mk_i32 23 <: i32) -! mk_i32 1

(* Layer A core: the gathered+masked vector's lane j == the spec coefficient, given se's bytes. *)
#push-options "--z3rlimit 600 --ifuel 2 --fuel 1"
let lemma_layerA_se (se: t_Array u8 (mk_usize 32)) (input: t_Slice u8) (j:nat{j<8})
  : Lemma
    (requires Seq.length input == 24 /\
              (forall (k:nat). k<24 ==> Seq.index se k == Seq.index input k))
    (ensures to_i32x8 (I.mm256_and_si256 (gathered (I.mm256_loadu_si256_u8 (se <: t_Slice u8)))
                                          (I.mm256_set1_epi32 v_MASK)) (mk_u64 j)
             == M.rejection_sample_coefficient input (sz j)) =
  let c0 = I.mm256_loadu_si256_u8 (se <: t_Slice u8) in
  let result = I.mm256_and_si256 (gathered c0) (I.mm256_set1_epi32 v_MASK) in
  M.rejection_sample_coefficient_lemma input (sz j);
  let aux (i:u64{v i<32})
    : Lemma (i32_to_bv (to_i32x8 result (mk_u64 j)) i
             == i32_to_bv (M.rejection_sample_coefficient input (sz j)) i) =
    coeff_gather_bv_lemma (Seq.index input (3*j)) (Seq.index input (3*j+1)) (Seq.index input (3*j+2)) i;
    if v i < 24 then begin
      let r = v i / 8 in let s = v i % 8 in
      FStar.Math.Lemmas.euclidean_division_definition (v i) 8;
      lemma_gather c0 j r s
    end else ()
  in
  FStar.Classical.forall_intro aux;
  i32_to_bv_ext (to_i32x8 result (mk_u64 j)) (M.rejection_sample_coefficient input (sz j))
#pop-options

(* the padded 32-byte buffer the leaf builds; its first 24 bytes == input *)
#push-options "--z3rlimit 400 --ifuel 2 --fuel 2"
let lemma_se_bytes (input: t_Slice u8)
  : Lemma (requires Seq.length input == 24)
          (ensures (
            let se0 : t_Array u8 (mk_usize 32) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32) in
            let se = Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to se0
                       ({ R.f_end = mk_usize 24 } <: R.t_RangeTo usize)
                       (Core_models.Slice.impl__copy_from_slice
                          (se0.[ ({ R.f_end = mk_usize 24 } <: R.t_RangeTo usize) ] <: t_Slice u8) input) in
            forall (k:nat). k<24 ==> Seq.index se k == Seq.index input k)) =
  let se0 : t_Array u8 (mk_usize 32) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32) in
  let dst : t_Slice u8 = se0.[ ({ R.f_end = mk_usize 24 } <: R.t_RangeTo usize) ] in
  let cpy : t_Slice u8 = Core_models.Slice.impl__copy_from_slice dst input in
  assert (cpy == input);
  let se = Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to se0
             ({ R.f_end = mk_usize 24 } <: R.t_RangeTo usize) cpy in
  assert (Seq.slice se 0 24 == input);
  let aux (k:nat) : Lemma (k<24 ==> Seq.index se k == Seq.index input k) =
    if k < 24 then Seq.lemma_index_slice se 0 24 k else ()
  in
  FStar.Classical.forall_intro aux
#pop-options

(* full Layer A: the real bytestream gather == the spec coefficient *)
#push-options "--z3rlimit 600 --ifuel 2 --fuel 2"
let lemma_layerA (input: t_Slice u8) (j:nat{j<8})
  : Lemma (requires Seq.length input == 24)
          (ensures to_i32x8 (bytestream_to_potential_coefficients input) (mk_u64 j)
                   == M.rejection_sample_coefficient input (sz j)) =
  let se0 : t_Array u8 (mk_usize 32) = Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32) in
  let se = Rust_primitives.Hax.Monomorphized_update_at.update_at_range_to se0
             ({ R.f_end = mk_usize 24 } <: R.t_RangeTo usize)
             (Core_models.Slice.impl__copy_from_slice
                (se0.[ ({ R.f_end = mk_usize 24 } <: R.t_RangeTo usize) ] <: t_Slice u8) input) in
  assert (bytestream_to_potential_coefficients input
          == I.mm256_and_si256 (gathered (I.mm256_loadu_si256_u8 (se <: t_Slice u8)))
                               (I.mm256_set1_epi32 v_MASK));
  lemma_se_bytes input;
  lemma_layerA_se se input j
#pop-options

(* =============== Layer D + structural assembly (field_modulus) =============== *)
(* filt8 of the gathered candidates == the field-modulus spec sequence *)
#push-options "--z3rlimit 300 --fuel 1 --ifuel 2"
let lemma_filt8_is_spec (input: t_Slice u8)
  : Lemma (requires Seq.length input == 24)
          (ensures (let potential = bytestream_to_potential_coefficients input in
                    filt8 (cand8v potential) (acc8b potential (mk_i32 8380417))
                    == M.rejection_sample_field_modulus input)) =
  let potential = bytestream_to_potential_coefficients input in
  let aux (j:nat{j<8}) : Lemma (cand8v potential j == M.rejection_sample_coefficient input (sz j) /\
                                acc8b potential (mk_i32 8380417) j
                                  == (cand8v potential j <. mk_i32 8380417)) =
    lemma_layerA input j
  in
  Classical.forall_intro aux;
  lemma_filt8_eq_spec input (cand8v potential) (acc8b potential (mk_i32 8380417))
#pop-options

(* full structural+spec leaf result: out[0..result] == rejection_sample_field_modulus input *)
#push-options "--z3rlimit 400 --fuel 1 --ifuel 2"
let lemma_leaf_post (input: t_Slice u8) (output: t_Slice i32)
  (good_lower good_upper: i32) (nlo nup: nat)
  : Lemma
    (requires
       Seq.length input == 24 /\ Seq.length output >= 8 /\
       v good_lower >= 0 /\ v good_lower < 16 /\ v good_upper >= 0 /\ v good_upper < 16 /\
       nlo <= 4 /\ nup <= 4 /\
       (let potential = bytestream_to_potential_coefficients input in
        let good = I.mm256_movemask_ps (I.mm256_castsi256_ps
                     (I.mm256_cmpgt_epi32 (I.mm256_set1_epi32 (mk_i32 8380417)) potential)) in
        good_lower == (good &. mk_i32 15) /\ good_upper == (good >>! mk_i32 4)) /\
       nlo == v (Core_models.Num.impl_i32__count_ones good_lower) /\
       nup == v (Core_models.Num.impl_i32__count_ones good_upper))
    (ensures (
       let potential = bytestream_to_potential_coefficients input in
       let lc = I.mm_shuffle_epi8 (I.mm256_castsi256_si128 potential)
                  (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE (v good_lower) <: t_Slice u8)) in
       let uc = I.mm_shuffle_epi8 (I.mm256_extracti128_si256 (mk_i32 1) potential)
                  (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE (v good_upper) <: t_Slice u8)) in
       let o1 = Rust_primitives.Hax.Monomorphized_update_at.update_at_range output
                  ({ R.f_start = mk_usize 0; R.f_end = mk_usize 4 } <: R.t_Range usize)
                  (I.mm_storeu_si128_i32 (Seq.slice output 0 4) lc) in
       let o2 = Rust_primitives.Hax.Monomorphized_update_at.update_at_range o1
                  ({ R.f_start = mk_usize nlo; R.f_end = mk_usize (nlo+4) } <: R.t_Range usize)
                  (I.mm_storeu_si128_i32 (Seq.slice o1 nlo (nlo+4)) uc) in
       Seq.slice o2 0 (nlo+nup) == M.rejection_sample_field_modulus input)) =
  let potential = bytestream_to_potential_coefficients input in
  lemma_leaf_structural_g potential potential (mk_i32 8380417) output good_lower good_upper nlo nup;
  lemma_filt8_is_spec input
#pop-options

(* =============== spec value bound (for the dispatcher bounds-only post) =============== *)
#push-options "--z3rlimit 300 --fuel 1 --ifuel 1"
let lemma_coeff_bounds (input: t_Slice u8) (j:nat{j<8})
  : Lemma (requires Seq.length input == 24)
          (ensures 0 <= v (M.rejection_sample_coefficient input (sz j)) /\
                   v (M.rejection_sample_coefficient input (sz j)) < 8388608) =
  M.rejection_sample_coefficient_lemma input (sz j);
  let b0 = cast (Seq.index input (j * 3)) <: i32 in
  let b1 = cast (Seq.index input (j * 3 + 1)) <: i32 in
  let b2 = cast (Seq.index input (j * 3 + 2)) <: i32 in
  logand_mask_lemma (((b2 <<! mk_i32 16 <: i32) |. (b1 <<! mk_i32 8 <: i32) <: i32) |. b0 <: i32) 23
#pop-options

#push-options "--z3rlimit 400 --fuel 1 --ifuel 2"
let lemma_spec_fm_bound (input: t_Slice u8)
  : Lemma (requires Seq.length input == 24)
          (ensures (let s = M.rejection_sample_field_modulus input in
                    forall (i:nat). i < Seq.length s ==>
                      (v (Seq.index s i) >= 0 /\ v (Seq.index s i) < 8380417))) =
  let potential = bytestream_to_potential_coefficients input in
  let aux (j:nat{j<8}) : Lemma (acc8b potential (mk_i32 8380417) j ==>
                                 (0 <= v (cand8v potential j) /\ v (cand8v potential j) <= 8380416)) =
    lemma_layerA input j;
    lemma_coeff_bounds input j
  in
  Classical.forall_intro aux;
  lemma_filt8_bound (cand8v potential) (acc8b potential (mk_i32 8380417)) 0 8380416;
  lemma_filt8_is_spec input
#pop-options
"#)]
#[hax_lib::requires(input.len() == 24 && output.len() >= 8)]
#[hax_lib::ensures(|r| fstar!(r#"
    Libcrux_ml_dsa.Specs.Simd.Portable.Sample.rejection_sample_less_than_field_modulus_post $input ${output}_future $r /\
    Seq.length ${output}_future == Seq.length $output /\
    v $r <= Seq.length $input / 3 /\
    (forall (i:nat{i < Seq.length ${output}_future}). i < v $r ==>
       v (Seq.index ${output}_future i) >= 0 /\ v (Seq.index ${output}_future i) < 8380417)"#))]
pub(crate) fn sample(input: &[u8], output: &mut [i32]) -> usize {
    #[cfg(hax)]
    let output_init = output.to_vec();

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

    hax_lib::fstar!(
        r#"Libcrux_ml_dsa.Proof_utils.lemma_movemask_ps_bound (Libcrux_intrinsics.Avx2.mm256_castsi256_ps $compare_with_field_modulus);
           logand_mask_lemma $good 4;
           assert (v $good_lower_half >= 0 /\ v $good_lower_half < 16);
           assert (v $good_upper_half >= 0 /\ v $good_upper_half < 16);
           Libcrux_ml_dsa.Proof_utils.lemma_count_ones_nibble $good_lower_half;
           Libcrux_ml_dsa.Proof_utils.lemma_count_ones_nibble $good_upper_half"#
    );

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

    let result = sampled_count + (good_upper_half.count_ones() as usize);

    hax_lib::fstar!(
        r#"lemma_leaf_post $input (Alloc.Vec.impl_1__as_slice output_init <: t_Slice i32) $good_lower_half $good_upper_half
             (v (Core_models.Num.impl_i32__count_ones $good_lower_half))
             (v (Core_models.Num.impl_i32__count_ones $good_upper_half));
           lemma_spec_fm_bound $input;
           (let s = Spec.MLDSA.Math.rejection_sample_field_modulus $input in
            introduce forall (i:nat{i < Seq.length ${output}}). i < v $result ==>
               (v (Seq.index $output i) >= 0 /\ v (Seq.index $output i) < 8380417)
            with (if i < v $result then Seq.lemma_index_slice $output 0 (v $result) i else ()))"#
    );

    result
}

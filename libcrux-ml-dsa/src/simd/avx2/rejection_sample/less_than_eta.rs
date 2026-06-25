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
#[hax_lib::fstar::options("--z3rlimit 400 --fuel 1 --ifuel 2")]
#[hax_lib::fstar::before(
    r#"
#set-options "--fuel 1 --ifuel 2 --z3rlimit 400"
open FStar.Mul
open Core_models
open Spec.Intrinsics
open Libcrux_core_models.Abstractions.Bit
open Libcrux_ml_dsa.Simd.Avx2.Rejection_sample.Proof_helpers

module I = Libcrux_intrinsics.Avx2
module ST = Libcrux_ml_dsa.Simd.Avx2.Rejection_sample.Shuffle_table
module R = Core_models.Ops.Range
module E = Libcrux_ml_dsa.Simd.Avx2.Encoding.Error
module C = Libcrux_ml_dsa.Constants
module M = Spec.MLDSA.Math

(* =============== Layer A_eta(a): deserialize lane == cast nibble =============== *)
(* lane 2m holds byte m's low nibble (try_0 = byte & 15);
   lane 2m+1 holds byte m's high nibble (try_1 = byte >> 4) *)
let nibble_u8 (input: t_Slice u8{Seq.length input == 4}) (j: nat{j<8}) : u8 =
  if j % 2 = 0 then (Seq.index input (j/2)) &. mk_u8 15
  else (Seq.index input (j/2)) >>! mk_u8 4

#push-options "--z3rlimit 600 --ifuel 2 --fuel 1"
let lemma_layerA_eta_a (input: t_Slice u8) (j: nat{j<8})
  : Lemma (requires Seq.length input == 4)
          (ensures to_i32x8 (E.deserialize_to_unsigned C.Eta_Four input) (mk_u64 j)
                   == (cast (nibble_u8 input j) <: i32)) =
  let p = E.deserialize_to_unsigned C.Eta_Four input in
  let nb = nibble_u8 input j in
  let aux (b: u64{v b < 32})
    : Lemma (i32_to_bv (to_i32x8 p (mk_u64 j)) b == i32_to_bv (cast nb <: i32) b) =
    if v b < 4 then begin
      let k = 4 * j + v b in
      assert (k < 32);
      FStar.Math.Lemmas.lemma_div_plus (v b) j 4;
      FStar.Math.Lemmas.lemma_mod_plus (v b) j 4;
      assert (k / 4 == j /\ k % 4 == v b);
      if j % 2 = 0 then begin
        u8_to_bv_logand15_lemma (Seq.index input (j/2)) b;
        assert (k / 8 == j / 2 /\ k % 8 == v b)
      end else begin
        u8_to_bv_shr4_lemma (Seq.index input (j/2)) b;
        assert (k / 8 == j / 2 /\ k % 8 == v b + 4)
      end
    end else begin
      assert ((j*32 + v b) % 32 == v b);
      if v b < 8 then
        (if j % 2 = 0 then u8_to_bv_logand15_lemma (Seq.index input (j/2)) b
         else u8_to_bv_shr4_lemma (Seq.index input (j/2)) b)
      else ()
    end
  in
  FStar.Classical.forall_intro aux;
  i32_to_bv_ext (to_i32x8 p (mk_u64 j)) (cast nb <: i32)
#pop-options

(* =============== Layer A_eta(b): shift_interval lane == eta coefficient =============== *)
let lemma_div5_trick (n: int)
  : Lemma (requires 0 <= n /\ n < 16)
          (ensures (26*n)/128 == n/5 /\ n - 5*((26*n)/128) == n % 5) =
  ()

#push-options "--z3rlimit 600 --ifuel 2 --fuel 1"
let lemma_shift_interval_lane (eta: usize) (potential: bv256) (j: nat{j<8})
  : Lemma (requires (v eta == 2 \/ v eta == 4) /\
                    v (to_i32x8 potential (mk_u64 j)) >= 0 /\
                    v (to_i32x8 potential (mk_u64 j)) < 16)
          (ensures to_i32x8 (shift_interval eta potential) (mk_u64 j) ==
            (if v eta = 2
             then mk_i32 2 -. ((to_i32x8 potential (mk_u64 j)) %! mk_i32 5)
             else mk_i32 4 -. (to_i32x8 potential (mk_u64 j)))) =
  reveal_opaque_arithmetic_ops #i32_inttype;
  let c = to_i32x8 potential (mk_u64 j) in
  if v eta = 2 then lemma_div5_trick (v c) else ()
#pop-options

(* =============== eta_2 leaf post =============== *)
#push-options "--z3rlimit 600 --ifuel 2 --fuel 1"
let lemma_eta2_byte_facts (input: t_Slice u8{Seq.length input == 4}) (m: nat{m<4})
  : Lemma
    (ensures (
      let p = E.deserialize_to_unsigned C.Eta_Four input in
      let sh = shift_interval (mk_usize 2) p in
      let byte = Seq.index input m in
      let try_0 = byte &. mk_u8 15 in
      let try_1 = byte >>! mk_u8 4 in
      acc8b p (mk_i32 15) (2*m)   == (try_0 <. mk_u8 15) /\
      acc8b p (mk_i32 15) (2*m+1) == (try_1 <. mk_u8 15) /\
      cand8v sh (2*m)   == (mk_i32 2 -. ((cast try_0 <: i32) %! mk_i32 5)) /\
      cand8v sh (2*m+1) == (mk_i32 2 -. ((cast try_1 <: i32) %! mk_i32 5)))) =
  let p = E.deserialize_to_unsigned C.Eta_Four input in
  let byte = Seq.index input m in
  logand_mask_lemma byte 4;
  lemma_layerA_eta_a input (2*m);
  lemma_layerA_eta_a input (2*m+1);
  lemma_shift_interval_lane (mk_usize 2) p (2*m);
  lemma_shift_interval_lane (mk_usize 2) p (2*m+1)
#pop-options

#push-options "--z3rlimit 400 --ifuel 2 --fuel 1"
let lemma_eta2_filt8_is_spec (input: t_Slice u8{Seq.length input == 4})
  : Lemma (ensures (
      let p = E.deserialize_to_unsigned C.Eta_Four input in
      let sh = shift_interval (mk_usize 2) p in
      filt8 (cand8v sh) (acc8b p (mk_i32 15)) == M.rejection_sample_eta_2 input)) =
  let p = E.deserialize_to_unsigned C.Eta_Four input in
  let sh = shift_interval (mk_usize 2) p in
  let aux (m:nat{m<4}) : Lemma (
      let byte = Seq.index input m in
      let try_0 = byte &. mk_u8 15 in
      let try_1 = byte >>! mk_u8 4 in
      acc8b p (mk_i32 15) (2*m) == (try_0 <. mk_u8 15) /\
      acc8b p (mk_i32 15) (2*m+1) == (try_1 <. mk_u8 15) /\
      cand8v sh (2*m) == (mk_i32 2 -. ((cast try_0 <: i32) %! mk_i32 5)) /\
      cand8v sh (2*m+1) == (mk_i32 2 -. ((cast try_1 <: i32) %! mk_i32 5))) =
    lemma_eta2_byte_facts input m
  in
  Classical.forall_intro aux;
  lemma_filt8_eq_eta2 input (cand8v sh) (acc8b p (mk_i32 15))
#pop-options

#push-options "--z3rlimit 400 --fuel 1 --ifuel 2"
let lemma_eta2_leaf_post (input: t_Slice u8) (output: t_Slice i32)
  (good_lower good_upper: i32) (nlo nup: nat)
  : Lemma
    (requires
       Seq.length input == 4 /\ Seq.length output >= 8 /\
       v good_lower >= 0 /\ v good_lower < 16 /\ v good_upper >= 0 /\ v good_upper < 16 /\
       nlo <= 4 /\ nup <= 4 /\
       (let p = E.deserialize_to_unsigned C.Eta_Four input in
        let good = I.mm256_movemask_ps (I.mm256_castsi256_ps
                     (I.mm256_cmpgt_epi32 (I.mm256_set1_epi32 (mk_i32 15)) p)) in
        good_lower == (good &. mk_i32 15) /\ good_upper == (good >>! mk_i32 4)) /\
       nlo == v (Core_models.Num.impl_i32__count_ones good_lower) /\
       nup == v (Core_models.Num.impl_i32__count_ones good_upper))
    (ensures (
       let p = E.deserialize_to_unsigned C.Eta_Four input in
       let sh = shift_interval (mk_usize 2) p in
       let lc = I.mm_shuffle_epi8 (I.mm256_castsi256_si128 sh)
                  (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE (v good_lower) <: t_Slice u8)) in
       let uc = I.mm_shuffle_epi8 (I.mm256_extracti128_si256 (mk_i32 1) sh)
                  (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE (v good_upper) <: t_Slice u8)) in
       let o1 = Rust_primitives.Hax.Monomorphized_update_at.update_at_range output
                  ({ R.f_start = mk_usize 0; R.f_end = mk_usize 4 } <: R.t_Range usize)
                  (I.mm_storeu_si128_i32 (Seq.slice output 0 4) lc) in
       let o2 = Rust_primitives.Hax.Monomorphized_update_at.update_at_range o1
                  ({ R.f_start = mk_usize nlo; R.f_end = mk_usize (nlo+4) } <: R.t_Range usize)
                  (I.mm_storeu_si128_i32 (Seq.slice o1 nlo (nlo+4)) uc) in
       Seq.slice o2 0 (nlo+nup) == M.rejection_sample_eta_2 input)) =
  let p = E.deserialize_to_unsigned C.Eta_Four input in
  let sh = shift_interval (mk_usize 2) p in
  lemma_leaf_structural_g sh p (mk_i32 15) output good_lower good_upper nlo nup;
  lemma_eta2_filt8_is_spec input
#pop-options

(* =============== eta_4 leaf post =============== *)
#push-options "--z3rlimit 600 --ifuel 2 --fuel 1"
let lemma_eta4_byte_facts (input: t_Slice u8{Seq.length input == 4}) (m: nat{m<4})
  : Lemma
    (ensures (
      let p = E.deserialize_to_unsigned C.Eta_Four input in
      let sh = shift_interval (mk_usize 4) p in
      let byte = Seq.index input m in
      let try_0 = byte &. mk_u8 15 in
      let try_1 = byte >>! mk_u8 4 in
      acc8b p (mk_i32 9) (2*m)   == (try_0 <. mk_u8 9) /\
      acc8b p (mk_i32 9) (2*m+1) == (try_1 <. mk_u8 9) /\
      cand8v sh (2*m)   == (mk_i32 4 -. (cast try_0 <: i32)) /\
      cand8v sh (2*m+1) == (mk_i32 4 -. (cast try_1 <: i32)))) =
  let p = E.deserialize_to_unsigned C.Eta_Four input in
  let byte = Seq.index input m in
  logand_mask_lemma byte 4;
  lemma_layerA_eta_a input (2*m);
  lemma_layerA_eta_a input (2*m+1);
  lemma_shift_interval_lane (mk_usize 4) p (2*m);
  lemma_shift_interval_lane (mk_usize 4) p (2*m+1)
#pop-options

#push-options "--z3rlimit 400 --ifuel 2 --fuel 1"
let lemma_eta4_filt8_is_spec (input: t_Slice u8{Seq.length input == 4})
  : Lemma (ensures (
      let p = E.deserialize_to_unsigned C.Eta_Four input in
      let sh = shift_interval (mk_usize 4) p in
      filt8 (cand8v sh) (acc8b p (mk_i32 9)) == M.rejection_sample_eta_4 input)) =
  let p = E.deserialize_to_unsigned C.Eta_Four input in
  let sh = shift_interval (mk_usize 4) p in
  let aux (m:nat{m<4}) : Lemma (
      let byte = Seq.index input m in
      let try_0 = byte &. mk_u8 15 in
      let try_1 = byte >>! mk_u8 4 in
      acc8b p (mk_i32 9) (2*m) == (try_0 <. mk_u8 9) /\
      acc8b p (mk_i32 9) (2*m+1) == (try_1 <. mk_u8 9) /\
      cand8v sh (2*m) == (mk_i32 4 -. (cast try_0 <: i32)) /\
      cand8v sh (2*m+1) == (mk_i32 4 -. (cast try_1 <: i32))) =
    lemma_eta4_byte_facts input m
  in
  Classical.forall_intro aux;
  lemma_filt8_eq_eta4 input (cand8v sh) (acc8b p (mk_i32 9))
#pop-options

#push-options "--z3rlimit 400 --fuel 1 --ifuel 2"
let lemma_eta4_leaf_post (input: t_Slice u8) (output: t_Slice i32)
  (good_lower good_upper: i32) (nlo nup: nat)
  : Lemma
    (requires
       Seq.length input == 4 /\ Seq.length output >= 8 /\
       v good_lower >= 0 /\ v good_lower < 16 /\ v good_upper >= 0 /\ v good_upper < 16 /\
       nlo <= 4 /\ nup <= 4 /\
       (let p = E.deserialize_to_unsigned C.Eta_Four input in
        let good = I.mm256_movemask_ps (I.mm256_castsi256_ps
                     (I.mm256_cmpgt_epi32 (I.mm256_set1_epi32 (mk_i32 9)) p)) in
        good_lower == (good &. mk_i32 15) /\ good_upper == (good >>! mk_i32 4)) /\
       nlo == v (Core_models.Num.impl_i32__count_ones good_lower) /\
       nup == v (Core_models.Num.impl_i32__count_ones good_upper))
    (ensures (
       let p = E.deserialize_to_unsigned C.Eta_Four input in
       let sh = shift_interval (mk_usize 4) p in
       let lc = I.mm_shuffle_epi8 (I.mm256_castsi256_si128 sh)
                  (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE (v good_lower) <: t_Slice u8)) in
       let uc = I.mm_shuffle_epi8 (I.mm256_extracti128_si256 (mk_i32 1) sh)
                  (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE (v good_upper) <: t_Slice u8)) in
       let o1 = Rust_primitives.Hax.Monomorphized_update_at.update_at_range output
                  ({ R.f_start = mk_usize 0; R.f_end = mk_usize 4 } <: R.t_Range usize)
                  (I.mm_storeu_si128_i32 (Seq.slice output 0 4) lc) in
       let o2 = Rust_primitives.Hax.Monomorphized_update_at.update_at_range o1
                  ({ R.f_start = mk_usize nlo; R.f_end = mk_usize (nlo+4) } <: R.t_Range usize)
                  (I.mm_storeu_si128_i32 (Seq.slice o1 nlo (nlo+4)) uc) in
       Seq.slice o2 0 (nlo+nup) == M.rejection_sample_eta_4 input)) =
  let p = E.deserialize_to_unsigned C.Eta_Four input in
  let sh = shift_interval (mk_usize 4) p in
  lemma_leaf_structural_g sh p (mk_i32 9) output good_lower good_upper nlo nup;
  lemma_eta4_filt8_is_spec input
#pop-options

(* =============== spec value bounds (for the dispatcher bounds-only posts) =============== *)
#push-options "--z3rlimit 200 --fuel 1 --ifuel 1"
let lemma_nibble_bound (input: t_Slice u8{Seq.length input == 4}) (j:nat{j<8})
  : Lemma (v (nibble_u8 input j) < 16) =
  if j % 2 = 0 then logand_mask_lemma (Seq.index input (j/2)) 4 else ()
#pop-options

#push-options "--z3rlimit 400 --fuel 1 --ifuel 2"
let lemma_spec_eta2_bound (input: t_Slice u8)
  : Lemma (requires Seq.length input == 4)
          (ensures (let s = M.rejection_sample_eta_2 input in
                    forall (i:nat). i < Seq.length s ==>
                      (v (Seq.index s i) >= -2 /\ v (Seq.index s i) <= 2))) =
  let p = E.deserialize_to_unsigned C.Eta_Four input in
  let sh = shift_interval (mk_usize 2) p in
  let aux (j:nat{j<8}) : Lemma (acc8b p (mk_i32 15) j ==>
                                 (v (cand8v sh j) >= -2 /\ v (cand8v sh j) <= 2)) =
    lemma_layerA_eta_a input j;
    lemma_nibble_bound input j;
    lemma_shift_interval_lane (mk_usize 2) p j
  in
  Classical.forall_intro aux;
  lemma_filt8_bound (cand8v sh) (acc8b p (mk_i32 15)) (-2) 2;
  lemma_eta2_filt8_is_spec input
#pop-options

#push-options "--z3rlimit 400 --fuel 1 --ifuel 2"
let lemma_spec_eta4_bound (input: t_Slice u8)
  : Lemma (requires Seq.length input == 4)
          (ensures (let s = M.rejection_sample_eta_4 input in
                    forall (i:nat). i < Seq.length s ==>
                      (v (Seq.index s i) >= -4 /\ v (Seq.index s i) <= 4))) =
  let p = E.deserialize_to_unsigned C.Eta_Four input in
  let sh = shift_interval (mk_usize 4) p in
  let aux (j:nat{j<8}) : Lemma (acc8b p (mk_i32 9) j ==>
                                 (v (cand8v sh j) >= -4 /\ v (cand8v sh j) <= 4)) =
    lemma_layerA_eta_a input j;
    lemma_nibble_bound input j;
    lemma_shift_interval_lane (mk_usize 4) p j
  in
  Classical.forall_intro aux;
  lemma_filt8_bound (cand8v sh) (acc8b p (mk_i32 9)) (-4) 4;
  lemma_eta4_filt8_is_spec input
#pop-options
"#
)]
#[hax_lib::requires((ETA == 2 || ETA == 4) && input.len() == 4 && output.len() >= 8)]
#[hax_lib::ensures(|r| fstar!(r#"
    (v $ETA == 2 ==> Libcrux_ml_dsa.Specs.Simd.Portable.Sample.rejection_sample_less_than_eta_equals_2_post $input ${output}_future $r) /\
    (v $ETA == 4 ==> Libcrux_ml_dsa.Specs.Simd.Portable.Sample.rejection_sample_less_than_eta_equals_4_post $input ${output}_future $r) /\
    Seq.length ${output}_future == Seq.length $output /\
    v $r <= Seq.length $input * 2 /\
    (v $ETA == 2 ==> (forall (i:nat{i < Seq.length ${output}_future}). i < v $r ==>
       v (Seq.index ${output}_future i) >= -2 /\ v (Seq.index ${output}_future i) <= 2)) /\
    (v $ETA == 4 ==> (forall (i:nat{i < Seq.length ${output}_future}). i < v $r ==>
       v (Seq.index ${output}_future i) >= -4 /\ v (Seq.index ${output}_future i) <= 4))"#))]
pub(crate) fn sample<const ETA: usize>(input: &[u8], output: &mut [i32]) -> usize {
    #[cfg(hax)]
    let output_init = output.to_vec();

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

    let result = sampled_count + (good_upper_half.count_ones() as usize);

    hax_lib::fstar!(
        r#"(if v $ETA = 2
            then begin
              assert (interval_boundary == mk_i32 15);
              lemma_eta2_leaf_post $input (Alloc.Vec.impl_1__as_slice output_init <: t_Slice i32) $good_lower_half $good_upper_half
                (v (Core_models.Num.impl_i32__count_ones $good_lower_half))
                (v (Core_models.Num.impl_i32__count_ones $good_upper_half));
              lemma_spec_eta2_bound $input;
              (let s = Spec.MLDSA.Math.rejection_sample_eta_2 $input in
               introduce forall (i:nat{i < Seq.length ${output}}). i < v $result ==>
                  (v (Seq.index $output i) >= -2 /\ v (Seq.index $output i) <= 2)
               with (if i < v $result then Seq.lemma_index_slice $output 0 (v $result) i else ()))
            end
            else begin
              assert (interval_boundary == mk_i32 9);
              lemma_eta4_leaf_post $input (Alloc.Vec.impl_1__as_slice output_init <: t_Slice i32) $good_lower_half $good_upper_half
                (v (Core_models.Num.impl_i32__count_ones $good_lower_half))
                (v (Core_models.Num.impl_i32__count_ones $good_upper_half));
              lemma_spec_eta4_bound $input;
              (let s = Spec.MLDSA.Math.rejection_sample_eta_4 $input in
               introduce forall (i:nat{i < Seq.length ${output}}). i < v $result ==>
                  (v (Seq.index $output i) >= -4 /\ v (Seq.index $output i) <= 4)
               with (if i < v $result then Seq.lemma_index_slice $output 0 (v $result) i else ()))
            end)"#
    );

    result
}

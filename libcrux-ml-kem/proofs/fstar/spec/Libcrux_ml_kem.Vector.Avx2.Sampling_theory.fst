module Libcrux_ml_kem.Vector.Avx2.Sampling_theory
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

module AVX = Libcrux_intrinsics.Avx2_ml_kem_views
module I = Libcrux_intrinsics.Avx2

(* Hand-written proof theory relocated from src/vector/avx2/sampling.rs
   `hax_lib::fstar::before` blocks (byte-exact raw-string contents, verified verbatim
   against the green extracted module). Consumed only by that module. *)

(* `u8::count_ones` counts set bits.  This was a trusted axiom while
   `Rust_primitives.Arithmetic.count_ones_u8` was an uninterpreted `val` in
   hax-lib (only `v r <= 8` was known).  hax now models `count_ones` as a sum
   over the bits and discharges the popcount contract in
   `Core_models.Num.Count_ones_spec`, so this is a proof.

   `Count_ones_spec.popcount` and `Rej_table.popcount8` are the same bit
   recursion declared independently on each side; the bridge relates them. *)
let count_ones_u8_popcount8 (x: u8)
  : Lemma (v (Core_models.Num.impl_u8__count_ones x) ==
           Hacspec_ml_kem.Commute.Rej_table.popcount8 (v x))
  = Core_models.Num.Count_ones_spec.count_ones_u8_popcount x;
    Hacspec_ml_kem.Commute.Rej_table.lemma_popcount8_agrees (v x)

(* Seal the PROVEN shuffle semantics into the
   Hacspec_ml_kem.Commute.Rej_table.shuffle_semantics atom in its own
   context (the raw per-bit forall must not leak into any consumer VC). *)
let lemma_shuffle_semantics_of_axiom (a mask res: AVX.t_Vec128)
  : Lemma
    (requires res == I.mm_shuffle_epi8 a mask)
    (ensures Hacspec_ml_kem.Commute.Rej_table.shuffle_semantics a mask res)
  = Classical.forall_intro (AVX.lemma_bv_bit_mm_shuffle_epi8 a mask);
    Hacspec_ml_kem.Commute.Rej_table.intro_shuffle_semantics a mask res

(* Driver: every kept lane (j < popcount8 g) of a shuffled half is in
   [0, 3328].  Establishes the sealed atoms of
   Hacspec_ml_kem.Commute.Rej_table (shuffle_semantics via the axiom
   above, mask/row/half links from the term equalities) and composes the
   clean-context per-lane lemma lemma_half_lane_bounded. *)
#restart-solver
#push-options "--z3rlimit 300 --split_queries always"
let lemma_half_done
    (potential: AVX.t_Vec256) (a mask res: AVX.t_Vec128)
    (row: t_Array u8 (mk_usize 16)) (half: nat{half <= 1}) (g: nat{g < 256})
  : Lemma
    (requires
      res == I.mm_shuffle_epi8 a mask /\
      mask == I.mm_loadu_si128 (row <: t_Slice u8) /\
      row ==
      Seq.index Libcrux_ml_kem.Vector.Rej_sample_table.v_REJECTION_SAMPLE_SHUFFLE_TABLE g /\
      (half == 0 ==> a == I.mm256_castsi256_si128 potential) /\
      (half == 1 ==> a == I.mm256_extracti128_si256 (mk_i32 1) potential) /\
      Hacspec_ml_kem.Commute.Rej_table.top_bits_clear potential /\
      Hacspec_ml_kem.Commute.Rej_table.good_bits g potential half)
    (ensures
      forall (j: nat{j < 8}).
        j < Hacspec_ml_kem.Commute.Rej_table.popcount8 g ==>
        (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 res) j) >= 0 /\
         v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 res) j) <= 3328))
  = lemma_shuffle_semantics_of_axiom a mask res;
    Hacspec_ml_kem.Commute.Rej_table.lemma_mask_of_row_loadu mask row;
    Hacspec_ml_kem.Commute.Rej_table.intro_row_of_table row g;
    Hacspec_ml_kem.Commute.Rej_table.lemma_half_of_cast a potential half;
    introduce forall (j: nat{j < 8}).
        j < Hacspec_ml_kem.Commute.Rej_table.popcount8 g ==>
        (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 res) j) >= 0 /\
         v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 res) j) <= 3328)
    with introduce j < Hacspec_ml_kem.Commute.Rej_table.popcount8 g ==>
        (v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 res) j) >= 0 /\
         v (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 res) j) <= 3328)
    with _. Hacspec_ml_kem.Commute.Rej_table.lemma_half_lane_bounded potential a mask res row half g j
#pop-options

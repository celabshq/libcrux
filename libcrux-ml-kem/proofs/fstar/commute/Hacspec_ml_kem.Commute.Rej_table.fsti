module Hacspec_ml_kem.Commute.Rej_table

(* Abstract-interface firewall for the Rej_table commute module.

   Consumers (all three): Libcrux_ml_kem.Vector.Avx2.Sampling (extraction),
   Libcrux_ml_kem.Vector.Avx2.Sampling_theory (spec companion), and a benign
   comment in Avx2_ml_kem_views.  Surface = the 16 symbols they cite: 6 opaque
   predicates (shuffle_semantics, mask_of_row, row_of_table, half_of,
   top_bits_clear, good_bits) + popcount8 + its two bound lemmas + the intro /
   bridge lemmas the sampling proof drives.  Everything else in the 305-decl .fst
   — including the 256-entry Rej_sample_table (`RT`) shuffle-table normalizer
   machinery consumed only internally — stays module-private.

   All vals ABSTRACT (0 transparent lets: the consumers reason about popcount8
   via the bound lemmas + the count_ones bridge, and about the shuffle/mask/row
   predicates only through the intro/consume lemmas), and carry ZERO SMTPats
   (Rej_table.fst has none).  Val order mirrors the .fst let order (F* Error 233).
   bit_vec_of_int_t_array / get_bit / AVX.* are external; RT is referenced only in
   intro_row_of_table's requires. *)

#set-options "--fuel 0 --ifuel 0 --z3rlimit 80"

open FStar.Mul
open Core_models
open Rust_primitives.Integers
open Rust_primitives.BitVectors

module RT  = Libcrux_ml_kem.Vector.Rej_sample_table
module AVX = Libcrux_intrinsics.Avx2_ml_kem_views
module I   = Libcrux_intrinsics.Avx2

/// Population count of the low 8 bits of `g` (LSB-first).
val popcount8 (g: nat) : nat

val lemma_popcount8_le (n: nat) (g: nat{g < pow2 n})
  : Lemma (ensures popcount8 g <= n)

/// Sealed: `res` is `a` byte-permuted by the per-nibble index encoded in `mask`.
val shuffle_semantics (a mask res: AVX.t_Vec128) : prop

val intro_shuffle_semantics (a mask res: AVX.t_Vec128)
  : Lemma
      (requires
        forall (i: nat{i < 128}).
          AVX.bv_bit res i ==
          (let nth = i / 8 in
            let idx: nat =
              AVX.bv_bit mask (8 * nth) + 2 * AVX.bv_bit mask (8 * nth + 1) + 4 * AVX.bv_bit mask (8 * nth + 2) +
              8 * AVX.bv_bit mask (8 * nth + 3) + 16 * AVX.bv_bit mask (8 * nth + 4) + 32 * AVX.bv_bit mask (8 * nth + 5) +
              64 * AVX.bv_bit mask (8 * nth + 6) + 128 * AVX.bv_bit mask (8 * nth + 7)
            in
            if idx > 127 then 0 else AVX.bv_bit a ((idx % 16) * 8 + i % 8)))
      (ensures shuffle_semantics a mask res)

/// Sealed: `mask`'s bits are byte `row`'s bits (little-endian per byte).
val mask_of_row (mask: AVX.t_Vec128) (row: t_Array u8 (mk_usize 16)) : prop

/// Sealed: `row` is entry `g` of the rejection-sample shuffle table.
val row_of_table (row: t_Array u8 (mk_usize 16)) (g: nat{g < 256}) : prop

val intro_row_of_table (row: t_Array u8 (mk_usize 16)) (g: nat{g < 256})
  : Lemma (requires row == Seq.index RT.v_REJECTION_SAMPLE_SHUFFLE_TABLE g)
          (ensures row_of_table row g)

/// Sealed: `a` is the `half`-th 128-bit lane of `potential`.
val half_of (a: AVX.t_Vec128) (potential: AVX.t_Vec256) (half: nat{half <= 1}) : prop

/// Sealed: every 16-bit block's top nibble bits (>=12) of `potential` are clear.
val top_bits_clear (potential: AVX.t_Vec256) : prop

val intro_top_bits_clear (potential: AVX.t_Vec256)
  : Lemma (requires forall (i: nat{i < 256}). i % 16 >= 12 ==> AVX.bv_bit potential i == 0)
          (ensures top_bits_clear potential)

/// Sealed: bit k of byte `g` is set iff lane `8*half + k` is a kept (< 3329) coeff.
val good_bits (g: nat) (potential: AVX.t_Vec256) (half: nat{half <= 1}) : prop

val lemma_half_lane_bounded
      (potential: AVX.t_Vec256)
      (a mask res: AVX.t_Vec128)
      (row: t_Array u8 (mk_usize 16))
      (half: nat{half <= 1})
      (g: nat{g < 256})
      (j: nat{j < 8 /\ j < popcount8 g})
  : Lemma
      (requires
        shuffle_semantics a mask res /\ mask_of_row mask row /\ row_of_table row g /\
        half_of a potential half /\ top_bits_clear potential /\ good_bits g potential half)
      (ensures
        v (Seq.index (AVX.vec128_as_i16x8 res) j) >= 0 /\
        v (Seq.index (AVX.vec128_as_i16x8 res) j) <= 3328)

val lemma_popcount8_u8 (g: nat{g < 256})
  : Lemma (popcount8 g <= 8)

val lemma_good_bits
    (good: t_Array u8 (mk_usize 2)) (cmp potential: AVX.t_Vec256) (half: nat{half <= 1})
  : Lemma
      (requires
        (forall (i: nat{i < 16}). bit_vec_of_int_t_array good 8 i == AVX.bv_bit cmp (i * 16)) /\
        (forall (l: nat{l < 16}).
            AVX.bv_bit cmp (16 * l) ==
            (if 3329 > v (Seq.index (AVX.vec256_as_i16x16 potential) l) then 1 else 0)))
      (ensures good_bits (v (Seq.index good half)) potential half)

val lemma_mask_of_row_loadu (mask: AVX.t_Vec128) (row: t_Array u8 (mk_usize 16))
  : Lemma (requires mask == I.mm_loadu_si128 (row <: t_Slice u8))
          (ensures mask_of_row mask row)

val lemma_half_of_cast (a: AVX.t_Vec128) (potential: AVX.t_Vec256) (half: nat{half <= 1})
  : Lemma
      (requires
        (half == 0 ==> a == I.mm256_castsi256_si128 potential) /\
        (half == 1 ==> a == I.mm256_extracti128_si256 (mk_i32 1) potential))
      (ensures half_of a potential half)

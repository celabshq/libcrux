module Hacspec_ml_kem.Commute.Rej_table

(* Track I M2 (2026-06-10): ground machinery for the AVX2 rejection-sampling
   shuffle table.  The extracted
   `Libcrux_ml_kem.Vector.Rej_sample_table.v_REJECTION_SAMPLE_SHUFFLE_TABLE`
   is a `Seq` built by `array_of_list`/`seq_of_list`; `Seq.index` is
   interface-abstract, so `assert_norm` cannot evaluate lookups into it
   (Track I M1 cliff note).  This module carries a DUPLICATE `list (list u8)`
   literal of the table (generated mechanically from
   `libcrux-ml-kem/src/vector/rej_sample_table.rs`), bridges it to the
   extracted Seq once by normalization-reflexivity (`lemma_table_eq`), proves
   all per-row facts over `List.Tot` (fully normalizable), and exports:

   - `popcount8` / `nth_set_bit` and their arithmetic lemmas;
   - `lemma_table_entry`: for `j < popcount8 g`, row `g` holds the byte pair
     `(2*k, 2*k+1)` at positions `(2*j, 2*j+1)` where `k = nth_set_bit g j`;
   - `lemma_u8_bits_value` (u8 = LSB-first sum of its bits);
   - `lemma_shuffled_lane` / `lemma_half_lane_bounded`: clean-context
     per-lane composition for `rejection_sample`'s values clause. *)

#set-options "--fuel 0 --ifuel 0 --z3rlimit 80"
open FStar.Mul
open Core_models
open Rust_primitives.Integers
open Rust_primitives.BitVectors

module L = FStar.List.Tot
module ML = FStar.Math.Lemmas
module RT = Libcrux_ml_kem.Vector.Rej_sample_table
module AVX = Libcrux_intrinsics.Avx2_extract

(* ===================================================================== *)
(* popcount8 / nth_set_bit                                               *)
(* ===================================================================== *)

#push-options "--fuel 1 --ifuel 0"

(* Number of set bits.  Matches the recursion validated (exhaustively over
   0..=255 against `u8::count_ones`) by the core-models test
   `track_i_axiom_transcription_tests::count_ones_popcount8_formula`. *)
let rec popcount8 (g: nat) : Tot nat (decreases g) =
  if g = 0 then 0 else g % 2 + popcount8 (g / 2)

let rec lemma_popcount8_le (n: nat) (g: nat{g < pow2 n})
  : Lemma (ensures popcount8 g <= n) (decreases n)
  = if g = 0 then ()
    else begin
      assert_norm (pow2 0 == 1);
      ML.pow2_plus 1 (n - 1);
      lemma_popcount8_le (n - 1) (g / 2)
    end

(* Index of the j-th set bit of g (LSB-first, 0-based). *)
let rec nth_set_bit (g: nat) (j: nat{j < popcount8 g}) : Tot nat (decreases g) =
  if g % 2 = 1
  then (if j = 0 then 0 else 1 + nth_set_bit (g / 2) (j - 1))
  else 1 + nth_set_bit (g / 2) j

let rec lemma_nth_set_bit_lt (n: nat) (g: nat{g < pow2 n}) (j: nat{j < popcount8 g})
  : Lemma (ensures nth_set_bit g j < n) (decreases n)
  = assert_norm (pow2 0 == 1);
    ML.pow2_plus 1 (n - 1);
    if g % 2 = 1 && j = 0 then ()
    else begin
      let j' = if g % 2 = 1 then j - 1 else j in
      lemma_nth_set_bit_lt (n - 1) (g / 2) j'
    end

let rec lemma_nth_set_bit_is_set (g: nat) (j: nat{j < popcount8 g})
  : Lemma (ensures (g / pow2 (nth_set_bit g j)) % 2 == 1) (decreases g)
  = if g % 2 = 1 && j = 0 then assert_norm (pow2 0 == 1)
    else begin
      let j' = if g % 2 = 1 then j - 1 else j in
      lemma_nth_set_bit_is_set (g / 2) j';
      let r = nth_set_bit (g / 2) j' in
      ML.pow2_plus 1 r;
      ML.division_multiplication_lemma g 2 (pow2 r)
    end

#pop-options

(* ===================================================================== *)
(* u8 bit decomposition                                                  *)
(* ===================================================================== *)

let lemma_get_bit_u8 (x: u8) (k: nat{k < 8})
  : Lemma (get_bit x (sz k) == (v x / pow2 k) % 2)
  = reveal_opaque (`%get_bit) (get_bit #U8)

#push-options "--z3rlimit 150"
(* A u8 is the LSB-first weighted sum of its bits. *)
let lemma_u8_bits_value (x: u8)
  : Lemma (v x ==
             get_bit x (sz 0) + 2 * get_bit x (sz 1) + 4 * get_bit x (sz 2) +
             8 * get_bit x (sz 3) + 16 * get_bit x (sz 4) + 32 * get_bit x (sz 5) +
             64 * get_bit x (sz 6) + 128 * get_bit x (sz 7))
  = let n = v x in
    lemma_get_bit_u8 x 0; lemma_get_bit_u8 x 1; lemma_get_bit_u8 x 2; lemma_get_bit_u8 x 3;
    lemma_get_bit_u8 x 4; lemma_get_bit_u8 x 5; lemma_get_bit_u8 x 6; lemma_get_bit_u8 x 7;
    assert_norm (pow2 0 == 1 /\ pow2 1 == 2 /\ pow2 2 == 4 /\ pow2 3 == 8 /\
                 pow2 4 == 16 /\ pow2 5 == 32 /\ pow2 6 == 64 /\ pow2 7 == 128 /\ pow2 8 == 256);
    ML.division_multiplication_lemma n 1 2;
    ML.division_multiplication_lemma n 2 2;
    ML.division_multiplication_lemma n 4 2;
    ML.division_multiplication_lemma n 8 2;
    ML.division_multiplication_lemma n 16 2;
    ML.division_multiplication_lemma n 32 2;
    ML.division_multiplication_lemma n 64 2;
    ML.small_div n 256;
    ML.euclidean_division_definition n 2;
    ML.euclidean_division_definition (n / 2) 2;
    ML.euclidean_division_definition (n / 4) 2;
    ML.euclidean_division_definition (n / 8) 2;
    ML.euclidean_division_definition (n / 16) 2;
    ML.euclidean_division_definition (n / 32) 2;
    ML.euclidean_division_definition (n / 64) 2;
    ML.euclidean_division_definition (n / 128) 2
#pop-options

(* ===================================================================== *)
(* Duplicate table literal (mechanically generated — DO NOT hand-edit;   *)
(* regenerate from libcrux-ml-kem/src/vector/rej_sample_table.rs)        *)
(* ===================================================================== *)

[@@ "opaque_to_smt"]
let row_0: list u8 = [mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_1: list u8 = [mk_u8 0; mk_u8 1; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_2: list u8 = [mk_u8 2; mk_u8 3; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_3: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_4: list u8 = [mk_u8 4; mk_u8 5; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_5: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_6: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_7: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_8: list u8 = [mk_u8 6; mk_u8 7; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_9: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_10: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_11: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_12: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_13: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_14: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_15: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_16: list u8 = [mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_17: list u8 = [mk_u8 0; mk_u8 1; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_18: list u8 = [mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_19: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_20: list u8 = [mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_21: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_22: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_23: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_24: list u8 = [mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_25: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_26: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_27: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_28: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_29: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_30: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_31: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_32: list u8 = [mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_33: list u8 = [mk_u8 0; mk_u8 1; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_34: list u8 = [mk_u8 2; mk_u8 3; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_35: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_36: list u8 = [mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_37: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_38: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_39: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_40: list u8 = [mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_41: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_42: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_43: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_44: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_45: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_46: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_47: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_48: list u8 = [mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_49: list u8 = [mk_u8 0; mk_u8 1; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_50: list u8 = [mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_51: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_52: list u8 = [mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_53: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_54: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_55: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_56: list u8 = [mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_57: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_58: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_59: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_60: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_61: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_62: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_63: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_64: list u8 = [mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_65: list u8 = [mk_u8 0; mk_u8 1; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_66: list u8 = [mk_u8 2; mk_u8 3; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_67: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_68: list u8 = [mk_u8 4; mk_u8 5; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_69: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_70: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_71: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_72: list u8 = [mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_73: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_74: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_75: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_76: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_77: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_78: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_79: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_80: list u8 = [mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_81: list u8 = [mk_u8 0; mk_u8 1; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_82: list u8 = [mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_83: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_84: list u8 = [mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_85: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_86: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_87: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_88: list u8 = [mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_89: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_90: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_91: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_92: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_93: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_94: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_95: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_96: list u8 = [mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_97: list u8 = [mk_u8 0; mk_u8 1; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_98: list u8 = [mk_u8 2; mk_u8 3; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_99: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_100: list u8 = [mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_101: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_102: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_103: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_104: list u8 = [mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_105: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_106: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_107: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_108: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_109: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_110: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_111: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_112: list u8 = [mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_113: list u8 = [mk_u8 0; mk_u8 1; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_114: list u8 = [mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_115: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_116: list u8 = [mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_117: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_118: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_119: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_120: list u8 = [mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_121: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_122: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_123: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_124: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_125: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_126: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_127: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_128: list u8 = [mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_129: list u8 = [mk_u8 0; mk_u8 1; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_130: list u8 = [mk_u8 2; mk_u8 3; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_131: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_132: list u8 = [mk_u8 4; mk_u8 5; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_133: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_134: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_135: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_136: list u8 = [mk_u8 6; mk_u8 7; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_137: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_138: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_139: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_140: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_141: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_142: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_143: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_144: list u8 = [mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_145: list u8 = [mk_u8 0; mk_u8 1; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_146: list u8 = [mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_147: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_148: list u8 = [mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_149: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_150: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_151: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_152: list u8 = [mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_153: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_154: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_155: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_156: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_157: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_158: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_159: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_160: list u8 = [mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_161: list u8 = [mk_u8 0; mk_u8 1; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_162: list u8 = [mk_u8 2; mk_u8 3; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_163: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_164: list u8 = [mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_165: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_166: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_167: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_168: list u8 = [mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_169: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_170: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_171: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_172: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_173: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_174: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_175: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_176: list u8 = [mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_177: list u8 = [mk_u8 0; mk_u8 1; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_178: list u8 = [mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_179: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_180: list u8 = [mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_181: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_182: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_183: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_184: list u8 = [mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_185: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_186: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_187: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_188: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_189: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_190: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_191: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_192: list u8 = [mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_193: list u8 = [mk_u8 0; mk_u8 1; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_194: list u8 = [mk_u8 2; mk_u8 3; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_195: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_196: list u8 = [mk_u8 4; mk_u8 5; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_197: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_198: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_199: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_200: list u8 = [mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_201: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_202: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_203: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_204: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_205: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_206: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_207: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_208: list u8 = [mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_209: list u8 = [mk_u8 0; mk_u8 1; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_210: list u8 = [mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_211: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_212: list u8 = [mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_213: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_214: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_215: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_216: list u8 = [mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_217: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_218: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_219: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_220: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_221: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_222: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_223: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_224: list u8 = [mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_225: list u8 = [mk_u8 0; mk_u8 1; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_226: list u8 = [mk_u8 2; mk_u8 3; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_227: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_228: list u8 = [mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_229: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_230: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_231: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_232: list u8 = [mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_233: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_234: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_235: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_236: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_237: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_238: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_239: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_240: list u8 = [mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_241: list u8 = [mk_u8 0; mk_u8 1; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_242: list u8 = [mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_243: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_244: list u8 = [mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_245: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_246: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_247: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_248: list u8 = [mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_249: list u8 = [mk_u8 0; mk_u8 1; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_250: list u8 = [mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_251: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_252: list u8 = [mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_253: list u8 = [mk_u8 0; mk_u8 1; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_254: list u8 = [mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15; mk_u8 255; mk_u8 255]
[@@ "opaque_to_smt"]
let row_255: list u8 = [mk_u8 0; mk_u8 1; mk_u8 2; mk_u8 3; mk_u8 4; mk_u8 5; mk_u8 6; mk_u8 7; mk_u8 8; mk_u8 9; mk_u8 10; mk_u8 11; mk_u8 12; mk_u8 13; mk_u8 14; mk_u8 15]

[@@ "opaque_to_smt"]
let table_rows: list (list u8) =
  [row_0; row_1; row_2; row_3; row_4; row_5; row_6; row_7;
   row_8; row_9; row_10; row_11; row_12; row_13; row_14; row_15;
   row_16; row_17; row_18; row_19; row_20; row_21; row_22; row_23;
   row_24; row_25; row_26; row_27; row_28; row_29; row_30; row_31;
   row_32; row_33; row_34; row_35; row_36; row_37; row_38; row_39;
   row_40; row_41; row_42; row_43; row_44; row_45; row_46; row_47;
   row_48; row_49; row_50; row_51; row_52; row_53; row_54; row_55;
   row_56; row_57; row_58; row_59; row_60; row_61; row_62; row_63;
   row_64; row_65; row_66; row_67; row_68; row_69; row_70; row_71;
   row_72; row_73; row_74; row_75; row_76; row_77; row_78; row_79;
   row_80; row_81; row_82; row_83; row_84; row_85; row_86; row_87;
   row_88; row_89; row_90; row_91; row_92; row_93; row_94; row_95;
   row_96; row_97; row_98; row_99; row_100; row_101; row_102; row_103;
   row_104; row_105; row_106; row_107; row_108; row_109; row_110; row_111;
   row_112; row_113; row_114; row_115; row_116; row_117; row_118; row_119;
   row_120; row_121; row_122; row_123; row_124; row_125; row_126; row_127;
   row_128; row_129; row_130; row_131; row_132; row_133; row_134; row_135;
   row_136; row_137; row_138; row_139; row_140; row_141; row_142; row_143;
   row_144; row_145; row_146; row_147; row_148; row_149; row_150; row_151;
   row_152; row_153; row_154; row_155; row_156; row_157; row_158; row_159;
   row_160; row_161; row_162; row_163; row_164; row_165; row_166; row_167;
   row_168; row_169; row_170; row_171; row_172; row_173; row_174; row_175;
   row_176; row_177; row_178; row_179; row_180; row_181; row_182; row_183;
   row_184; row_185; row_186; row_187; row_188; row_189; row_190; row_191;
   row_192; row_193; row_194; row_195; row_196; row_197; row_198; row_199;
   row_200; row_201; row_202; row_203; row_204; row_205; row_206; row_207;
   row_208; row_209; row_210; row_211; row_212; row_213; row_214; row_215;
   row_216; row_217; row_218; row_219; row_220; row_221; row_222; row_223;
   row_224; row_225; row_226; row_227; row_228; row_229; row_230; row_231;
   row_232; row_233; row_234; row_235; row_236; row_237; row_238; row_239;
   row_240; row_241; row_242; row_243; row_244; row_245; row_246; row_247;
   row_248; row_249; row_250; row_251; row_252; row_253; row_254; row_255]

(* ===================================================================== *)
(* List-side machinery and the Seq bridge                                *)
(* ===================================================================== *)

#push-options "--fuel 1 --ifuel 1"

let rec all_len16 (l: list (list u8)) : Tot bool (decreases l) =
  match l with
  | [] -> true
  | hd :: tl -> (L.length hd = 16) && all_len16 tl

let rec lemma_len16_index (l: list (list u8)) (g: nat{g < L.length l})
  : Lemma (requires all_len16 l == true)
          (ensures L.length (L.index l g) == 16)
          (decreases g)
  = match l with
    | hd :: tl -> if g = 0 then () else lemma_len16_index tl (g - 1)

let dummy_row : t_Array u8 (mk_usize 16) = Seq.create 16 (mk_u8 0)

let to_row (l: list u8) : t_Array u8 (mk_usize 16) =
  if L.length l = 16 then Seq.seq_of_list l else dummy_row

let rec seqify (l: list (list u8)) : Tot (list (t_Array u8 (mk_usize 16))) (decreases l) =
  match l with
  | [] -> []
  | hd :: tl -> to_row hd :: seqify tl

let rec lemma_seqify_length (l: list (list u8))
  : Lemma (ensures L.length (seqify l) == L.length l) (decreases l)
  = match l with
    | [] -> ()
    | _ :: tl -> lemma_seqify_length tl

let rec lemma_seqify_index (l: list (list u8)) (g: nat{g < L.length l})
  : Lemma (ensures (lemma_seqify_length l; L.index (seqify l) g == to_row (L.index l g)))
          (decreases g)
  = lemma_seqify_length l;
    match l with
    | hd :: tl -> if g = 0 then () else lemma_seqify_index tl (g - 1)

#pop-options

(* THE refl-bridge: the extracted Seq-built table equals the duplicate
   list literal, lifted by `seqify`.  Both sides normalize to the same
   `Seq.seq_of_list [Seq.seq_of_list [...]; ...]` term (`array_of_list`
   is an `unfold` for `seq_of_list`), so this is pure reflexivity after
   normalization — no `Seq.index` evaluation involved. *)
let lemma_table_eq ()
  : Lemma (RT.v_REJECTION_SAMPLE_SHUFFLE_TABLE == Seq.seq_of_list (seqify table_rows))
  = assert_norm (RT.v_REJECTION_SAMPLE_SHUFFLE_TABLE == Seq.seq_of_list (seqify table_rows))

(* Per-lookup bridge: Seq-side table lookups equal List.Tot-side lookups. *)
let lemma_table_lookup (g: nat{g < 256}) (m: nat{m < 16})
  : Lemma (L.length table_rows == 256 /\
           L.length (L.index table_rows g) == 16 /\
           Seq.index (Seq.index RT.v_REJECTION_SAMPLE_SHUFFLE_TABLE g) m ==
             L.index (L.index table_rows g) m)
  = lemma_table_eq ();
    assert_norm (L.length table_rows == 256);
    assert_norm (all_len16 table_rows == true);
    lemma_len16_index table_rows g;
    lemma_seqify_length table_rows;
    FStar.Seq.Properties.lemma_seq_of_list_index (seqify table_rows) g;
    lemma_seqify_index table_rows g;
    FStar.Seq.Properties.lemma_seq_of_list_index (L.index table_rows g) m

(* ===================================================================== *)
(* Ground sweep: every row satisfies the popcount/nth_set_bit layout     *)
(* ===================================================================== *)

(* Byte m of `row` as an int, total (guards instead of proofs). *)
let row_byte (row: list u8) (m: nat) : int =
  if m < L.length row then v (L.index row m) else (-1)

let entry_ok (row: list u8) (g: nat) (j: nat{j < 8}) : bool =
  if j < popcount8 g
  then
    (let k = nth_set_bit g j in
     row_byte row (2 * j) = 2 * k && row_byte row (2 * j + 1) = 2 * k + 1)
  else row_byte row (2 * j) = 255 && row_byte row (2 * j + 1) = 255

let row_ok (row: list u8) (g: nat) : bool =
  entry_ok row g 0 && entry_ok row g 1 && entry_ok row g 2 && entry_ok row g 3 &&
  entry_ok row g 4 && entry_ok row g 5 && entry_ok row g 6 && entry_ok row g 7

#push-options "--fuel 1 --ifuel 1"

let rec rows_ok (l: list (list u8)) (base: nat) : Tot bool (decreases l) =
  match l with
  | [] -> true
  | row :: tl -> row_ok row base && rows_ok tl (base + 1)

let rec lemma_rows_ok_index (l: list (list u8)) (base: nat) (g: nat{base <= g /\ g < base + L.length l})
  : Lemma (requires rows_ok l base == true)
          (ensures row_ok (L.index l (g - base)) g == true)
          (decreases l)
  = match l with
    | row :: tl -> if g = base then () else lemma_rows_ok_index tl (base + 1) g

#pop-options

(* The 256-row ground sweep — one normalization, all over List.Tot. *)
let lemma_table_rows_ok ()
  : Lemma (rows_ok table_rows 0 == true)
  = assert_norm (rows_ok table_rows 0 == true)

let lemma_row_ok_entry (row: list u8) (g: nat) (j: nat{j < 8})
  : Lemma (requires row_ok row g == true)
          (ensures entry_ok row g j == true)
  = if j = 0 then () else if j = 1 then () else if j = 2 then () else if j = 3 then ()
    else if j = 4 then () else if j = 5 then () else if j = 6 then () else ()

(* ===================================================================== *)
(* Exported table fact                                                   *)
(* ===================================================================== *)

(* For j < popcount8 g, row g of the shuffle table moves source lane
   k = nth_set_bit g j to output lane j: bytes (2j, 2j+1) are (2k, 2k+1).
   For j >= popcount8 g the bytes are 0xff (MSB set => shuffle yields 0). *)
let lemma_table_entry (g: nat{g < 256}) (j: nat{j < 8})
  : Lemma
      ((j < popcount8 g ==>
          (nth_set_bit g j < 8 /\
           v (Seq.index (Seq.index RT.v_REJECTION_SAMPLE_SHUFFLE_TABLE g) (2 * j)) ==
             2 * nth_set_bit g j /\
           v (Seq.index (Seq.index RT.v_REJECTION_SAMPLE_SHUFFLE_TABLE g) (2 * j + 1)) ==
             2 * nth_set_bit g j + 1)) /\
       (j >= popcount8 g ==>
          (v (Seq.index (Seq.index RT.v_REJECTION_SAMPLE_SHUFFLE_TABLE g) (2 * j)) == 255 /\
           v (Seq.index (Seq.index RT.v_REJECTION_SAMPLE_SHUFFLE_TABLE g) (2 * j + 1)) == 255)))
  = lemma_table_rows_ok ();
    lemma_table_lookup g (2 * j);
    lemma_table_lookup g (2 * j + 1);
    let row = L.index table_rows g in
    lemma_rows_ok_index table_rows 0 g;
    lemma_row_ok_entry row g j;
    if j < popcount8 g then (assert_norm (pow2 8 == 256); lemma_nth_set_bit_lt 8 g j) else ()

(* ===================================================================== *)
(* SIMD glue (clean-context per-lane lemmas for rejection_sample)        *)
(* ===================================================================== *)

(* Helper: byte/bit index split. *)
let lemma_byte_split (nth: nat) (s: nat{s < 8})
  : Lemma ((8 * nth + s) / 8 == nth /\ (8 * nth + s) % 8 == s)
  = ML.lemma_div_plus s nth 8;
    ML.lemma_mod_plus s nth 8;
    ML.small_div s 8;
    ML.small_mod s 8

(* Helper: lane/bit index split (16-bit lanes). *)
let lemma_lane_split (lane: nat) (t: nat{t < 16})
  : Lemma ((16 * lane + t) / 16 == lane /\ (16 * lane + t) % 16 == t)
  = ML.lemma_div_plus t lane 16;
    ML.lemma_mod_plus t lane 16;
    ML.small_div t 16;
    ML.small_mod t 16

(* If two 16-bit lanes (of a vec128 and a vec256 resp.) agree bitwise,
   their i16 values agree. *)
#push-options "--z3rlimit 200"
let lemma_vec128_lane_eq_bits
      (w: bit_vec 128) (src: bit_vec 256) (lane: nat{lane < 8}) (src_lane: nat{src_lane < 16})
  : Lemma
      (requires forall (t: nat{t < 16}). w (16 * lane + t) == src (16 * src_lane + t))
      (ensures
        Seq.index (AVX.vec128_as_i16x8 w) lane ==
        Seq.index (AVX.vec256_as_i16x16 src) src_lane)
  = let a = Seq.index (AVX.vec128_as_i16x8 w) lane in
    let b = Seq.index (AVX.vec256_as_i16x16 src) src_lane in
    introduce forall (t: usize{v t < 16}). get_bit a t == get_bit b t
    with begin
      AVX.bit_vec_of_int_t_array_vec128_as_i16x8_lemma w 16 (16 * lane + v t);
      AVX.bit_vec_of_int_t_array_vec256_as_i16x16_lemma src 16 (16 * src_lane + v t);
      lemma_lane_split lane (v t);
      lemma_lane_split src_lane (v t)
    end;
    lemma_int_t_eq_via_bits a b

(* Per-lane bound for a vec256 whose per-lane top bits are clear
   (copy of the local recipe in Vector.Avx2 / Vector.Avx2.Serialize —
   neither exports it). *)
let lemma_vec256_lane_bounded (vec: bit_vec 256) (n: nat{n > 0 /\ n <= 16}) (i: nat{i < 16})
  : Lemma
      (requires forall (b: nat{b < 16}). b >= n ==> vec (i * 16 + b) == 0)
      (ensures bounded (Seq.index (AVX.vec256_as_i16x16 vec) i) n)
  = let arr = AVX.vec256_as_i16x16 vec in
    let lane = Seq.index arr i in
    let aux (b: usize{v b < 16}) : Lemma (v b > n ==> get_bit lane b == 0)
      = if v b > n then begin
          AVX.bit_vec_of_int_t_array_vec256_as_i16x16_lemma vec 16 (i * 16 + v b);
          ML.lemma_mod_plus (v b) i 16;
          ML.lemma_div_plus (v b) i 16
        end
        else ()
    in
    Classical.forall_intro aux;
    lemma_get_bit_bounded' lane n
#pop-options

(* ===================================================================== *)
(* Opaque per-half atoms.  Each heavy hypothesis (the PSHUFB bit formula,
   the mask/row/table/half links, the lane-compare bits) is sealed in an
   opaque prop atom: consumers carry the ATOM (zero SMT instantiation
   cost), and only the leaf lemma that needs a forall reveals its own.
   Without this, the ite + div-sum formula hypothesis is re-instantiated
   in every split sub-query and saturates Z3 on trivial arithmetic
   (build d43a3f6c / 99987b26: rlimit_used 200.000, qi-profile flat).    *)
(* ===================================================================== *)

[@@ "opaque_to_smt"]
let shuffle_semantics (a mask res: bit_vec 128) : prop =
  forall (i: nat{i < 128}).
    res i ==
    (let nth = i / 8 in
      let idx: nat =
        mask (8 * nth) + 2 * mask (8 * nth + 1) + 4 * mask (8 * nth + 2) +
        8 * mask (8 * nth + 3) + 16 * mask (8 * nth + 4) + 32 * mask (8 * nth + 5) +
        64 * mask (8 * nth + 6) + 128 * mask (8 * nth + 7)
      in
      if idx > 127 then 0 else a ((idx % 16) * 8 + i % 8))

let intro_shuffle_semantics (a mask res: bit_vec 128)
  : Lemma
      (requires
        forall (i: nat{i < 128}).
          res i ==
          (let nth = i / 8 in
            let idx: nat =
              mask (8 * nth) + 2 * mask (8 * nth + 1) + 4 * mask (8 * nth + 2) +
              8 * mask (8 * nth + 3) + 16 * mask (8 * nth + 4) + 32 * mask (8 * nth + 5) +
              64 * mask (8 * nth + 6) + 128 * mask (8 * nth + 7)
            in
            if idx > 127 then 0 else a ((idx % 16) * 8 + i % 8)))
      (ensures shuffle_semantics a mask res)
  = reveal_opaque (`%shuffle_semantics) (shuffle_semantics a mask res)

[@@ "opaque_to_smt"]
let mask_of_row (mask: bit_vec 128) (row: t_Array u8 (mk_usize 16)) : prop =
  forall (i: nat{i < 128}). mask i == get_bit (Seq.index row (i / 8)) (sz (i % 8))

let intro_mask_of_row (mask: bit_vec 128) (row: t_Array u8 (mk_usize 16))
  : Lemma
      (requires forall (i: nat{i < 128}). mask i == get_bit (Seq.index row (i / 8)) (sz (i % 8)))
      (ensures mask_of_row mask row)
  = reveal_opaque (`%mask_of_row) (mask_of_row mask row)

[@@ "opaque_to_smt"]
let row_of_table (row: t_Array u8 (mk_usize 16)) (g: nat{g < 256}) : prop =
  forall (m: nat{m < 16}).
    Seq.index row m == Seq.index (Seq.index RT.v_REJECTION_SAMPLE_SHUFFLE_TABLE g) m

let intro_row_of_table (row: t_Array u8 (mk_usize 16)) (g: nat{g < 256})
  : Lemma (requires row == Seq.index RT.v_REJECTION_SAMPLE_SHUFFLE_TABLE g)
          (ensures row_of_table row g)
  = reveal_opaque (`%row_of_table) (row_of_table row g)

[@@ "opaque_to_smt"]
let half_of (a: bit_vec 128) (potential: bit_vec 256) (half: nat{half <= 1}) : prop =
  forall (i: nat{i < 128}). a i == potential (128 * half + i)

let intro_half_of (a: bit_vec 128) (potential: bit_vec 256) (half: nat{half <= 1})
  : Lemma (requires forall (i: nat{i < 128}). a i == potential (128 * half + i))
          (ensures half_of a potential half)
  = reveal_opaque (`%half_of) (half_of a potential half)

[@@ "opaque_to_smt"]
let top_bits_clear (potential: bit_vec 256) : prop =
  forall (i: nat{i < 256}). i % 16 >= 12 ==> potential i == 0

let intro_top_bits_clear (potential: bit_vec 256)
  : Lemma (requires forall (i: nat{i < 256}). i % 16 >= 12 ==> potential i == 0)
          (ensures top_bits_clear potential)
  = reveal_opaque (`%top_bits_clear) (top_bits_clear potential)

(* Bit k of g is set iff vec256 lane `8*half + k` is a kept (< 3329)
   coefficient. *)
[@@ "opaque_to_smt"]
let good_bits (g: nat) (potential: bit_vec 256) (half: nat{half <= 1}) : prop =
  forall (k: nat{k < 8}).
    (g / pow2 k) % 2 ==
    (if v (Seq.index (AVX.vec256_as_i16x16 potential) (8 * half + k)) < 3329 then 1 else 0)

(* ===================================================================== *)
(* Leaf lemmas (each reveals only its own atom; clean context)           *)
(* ===================================================================== *)

(* idx decoding: the 8 mask bits of byte `nth` sum (LSB-first weighted) to
   the byte's value. *)
#push-options "--z3rlimit 200"
let lemma_mask_byte_value (mask: bit_vec 128) (row: t_Array u8 (mk_usize 16)) (nth: nat{nth < 16})
  : Lemma
      (requires mask_of_row mask row)
      (ensures
        mask (8 * nth) + 2 * mask (8 * nth + 1) + 4 * mask (8 * nth + 2) +
        8 * mask (8 * nth + 3) + 16 * mask (8 * nth + 4) + 32 * mask (8 * nth + 5) +
        64 * mask (8 * nth + 6) + 128 * mask (8 * nth + 7) ==
        v (Seq.index row nth))
  = reveal_opaque (`%mask_of_row) (mask_of_row mask row);
    lemma_byte_split nth 0; lemma_byte_split nth 1; lemma_byte_split nth 2; lemma_byte_split nth 3;
    lemma_byte_split nth 4; lemma_byte_split nth 5; lemma_byte_split nth 6; lemma_byte_split nth 7;
    lemma_u8_bits_value (Seq.index row nth)
#pop-options

(* Table row facts at output lane j, via the sealed table link. *)
let lemma_row_bytes (row: t_Array u8 (mk_usize 16)) (g: nat{g < 256}) (j: nat{j < 8 /\ j < popcount8 g})
  : Lemma
      (requires row_of_table row g)
      (ensures
        nth_set_bit g j < 8 /\
        v (Seq.index row (2 * j)) == 2 * nth_set_bit g j /\
        v (Seq.index row (2 * j + 1)) == 2 * nth_set_bit g j + 1)
  = reveal_opaque (`%row_of_table) (row_of_table row g);
    assert_norm (pow2 8 == 256);
    lemma_nth_set_bit_lt 8 g j;
    lemma_table_entry g j

(* PSHUFB byte select: if mask byte `nth` has value idx <= 15, output byte
   `nth` of the shuffle is source byte `idx`, bit for bit. *)
#push-options "--z3rlimit 200 --split_queries always --z3refresh"
let lemma_shuffle_byte
      (a mask res: bit_vec 128) (row: t_Array u8 (mk_usize 16))
      (nth: nat{nth < 16}) (idx: nat{idx <= 15})
  : Lemma
      (requires
        shuffle_semantics a mask res /\ mask_of_row mask row /\
        v (Seq.index row nth) == idx)
      (ensures forall (s: nat{s < 8}). res (8 * nth + s) == a (8 * idx + s))
  = reveal_opaque (`%shuffle_semantics) (shuffle_semantics a mask res);
    lemma_mask_byte_value mask row nth;
    ML.small_mod idx 16;
    introduce forall (s: nat{s < 8}). res (8 * nth + s) == a (8 * idx + s)
    with lemma_byte_split nth s
#pop-options

(* The PSHUFB step: output lane j (for j < popcount8 g) is source lane
   `nth_set_bit g j`, bit for bit. *)
#push-options "--z3rlimit 200 --split_queries always --z3refresh"
let lemma_shuffled_lane
      (a mask res: bit_vec 128)
      (row: t_Array u8 (mk_usize 16))
      (g: nat{g < 256})
      (j: nat{j < 8 /\ j < popcount8 g})
  : Lemma
      (requires shuffle_semantics a mask res /\ mask_of_row mask row /\ row_of_table row g)
      (ensures
        nth_set_bit g j < 8 /\
        (forall (t: nat{t < 16}). res (16 * j + t) == a (16 * nth_set_bit g j + t)))
  = lemma_row_bytes row g j;
    let k = nth_set_bit g j in
    lemma_shuffle_byte a mask res row (2 * j) (2 * k);
    lemma_shuffle_byte a mask res row (2 * j + 1) (2 * k + 1);
    introduce forall (t: nat{t < 16}). res (16 * j + t) == a (16 * k + t)
    with begin
      if t < 8
      then assert (res (8 * (2 * j) + t) == a (8 * (2 * k) + t))
      else assert (res (8 * (2 * j + 1) + (t - 8)) == a (8 * (2 * k + 1) + (t - 8)))
    end
#pop-options

(* Half-finalize: output lane j of the shuffled half is a kept coefficient,
   hence in [0, 3328]. `half = 0` is the lower 128 bits (vec256 lanes 0..7),
   `half = 1` the upper (lanes 8..15). *)
#push-options "--z3rlimit 200 --split_queries always --z3refresh"
let lemma_half_lane_bounded
      (potential: bit_vec 256)
      (a mask res: bit_vec 128)
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
  = lemma_shuffled_lane a mask res row g j;
    let k = nth_set_bit g j in
    let src_lane = 8 * half + k in
    reveal_opaque (`%half_of) (half_of a potential half);
    introduce forall (t: nat{t < 16}). res (16 * j + t) == potential (16 * src_lane + t)
    with begin
      assert (res (16 * j + t) == a (16 * k + t));
      assert (a (16 * k + t) == potential (128 * half + (16 * k + t)))
    end;
    lemma_vec128_lane_eq_bits res potential j src_lane;
    (* the kept lane is < 3329: bit k of g is set *)
    reveal_opaque (`%good_bits) (good_bits g potential half);
    lemma_nth_set_bit_is_set g j;
    assert (v (Seq.index (AVX.vec256_as_i16x16 potential) src_lane) < 3329);
    (* and >= 0 (12-bit value): top 4 bits of the lane are clear *)
    reveal_opaque (`%top_bits_clear) (top_bits_clear potential);
    introduce forall (b: nat{b < 16}). b >= 12 ==> potential (src_lane * 16 + b) == 0
    with introduce b >= 12 ==> potential (src_lane * 16 + b) == 0
    with _. lemma_lane_split src_lane b;
    lemma_vec256_lane_bounded potential 12 src_lane
#pop-options

(* popcount8 of a byte value is at most 8 (re-export at the use shape). *)
let lemma_popcount8_u8 (g: nat{g < 256})
  : Lemma (popcount8 g <= 8)
  = assert_norm (pow2 8 == 256);
    lemma_popcount8_le 8 g

(* Bridge from serialize_1's bit-level post + cmpgt's lane-compare post to
   the sealed good_bits atom: bit k of byte `half` of `good` is set iff
   vec256 lane `8*half + k` is < 3329. *)
#push-options "--z3rlimit 200"
let lemma_good_bits
    (good: t_Array u8 (mk_usize 2)) (cmp potential: bit_vec 256) (half: nat{half <= 1})
  : Lemma
      (requires
        (forall (i: nat{i < 16}). bit_vec_of_int_t_array good 8 i == cmp (i * 16)) /\
        (forall (i: nat{i < 256}).
            cmp i ==
            (if 3329 > v (Seq.index (AVX.vec256_as_i16x16 potential) (i / 16)) then 1 else 0)))
      (ensures good_bits (v (Seq.index good half)) potential half)
  = reveal_opaque (`%good_bits) (good_bits (v (Seq.index good half)) potential half);
    introduce forall (k: nat{k < 8}).
        (v (Seq.index good half) / pow2 k) % 2 ==
        (if v (Seq.index (AVX.vec256_as_i16x16 potential) (8 * half + k)) < 3329 then 1 else 0)
    with begin
      lemma_byte_split half k;
      lemma_get_bit_u8 (Seq.index good half) k;
      ML.cancel_mul_div (8 * half + k) 16;
      assert (bit_vec_of_int_t_array good 8 (8 * half + k) == get_bit (Seq.index good half) (sz k))
    end
#pop-options

(* Term-level intro helpers: establish the sealed atoms directly from the
   BitVec.Intrinsics term equalities, each in its own clean context so the
   underlying foralls never leak into a consumer's VC. *)
#push-options "--z3rlimit 200"
let lemma_mask_of_row_loadu (mask: bit_vec 128) (row: t_Array u8 (mk_usize 16))
  : Lemma (requires mask == BitVec.Intrinsics.mm_loadu_si128 row)
          (ensures mask_of_row mask row)
  = intro_mask_of_row mask row

let lemma_half_of_cast (a: bit_vec 128) (potential: bit_vec 256) (half: nat{half <= 1})
  : Lemma
      (requires
        (half == 0 ==> a == BitVec.Intrinsics.mm256_castsi256_si128 potential) /\
        (half == 1 ==> a == BitVec.Intrinsics.mm256_extracti128_si256 (mk_i32 1) potential))
      (ensures half_of a potential half)
  = intro_half_of a potential half
#pop-options

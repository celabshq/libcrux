//! Trusted-base + proven proof helpers for the AVX2 rejection_sample functional
//! proof (F*-only). The generic combinatorial machinery (SHUFFLE_TABLE compaction,
//! per-lane masks, the `filt8`/`step8` repeati-filter model, the two-half store
//! assembly, the generic structural leaf result `lemma_leaf_structural_g`, the eta
//! spec bridges, and the per-element `lemma_filt8_bound`) is emitted as standalone
//! F* into `Libcrux_ml_dsa.Simd.Avx2.Rejection_sample.Proof_helpers`. Model axioms
//! live in `Spec.Intrinsics`; `lemma_count_ones_nibble_exact` is the one trusted
//! `assume val` here (count_ones has no F* model in-tree). The marker fn below just
//! gives hax an item to hang the module on (whole module is `#[cfg(hax)]`).
#[hax_lib::fstar::before(r#"
#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"
open FStar.Mul
open Core_models
open Spec.Intrinsics
open Libcrux_core_models.Abstractions.Bit

module I = Libcrux_intrinsics.Avx2
module ST = Libcrux_ml_dsa.Simd.Avx2.Rejection_sample.Shuffle_table
module SU = Spec.Utils
module M = Spec.MLDSA.Math
module R = Core_models.Ops.Range
module UL = Rust_primitives.Hax.Monomorphized_update_at_Lemmas

(* ===== from ScratchRej PART2/3/4 (shuffle_lane + table + shuffle_table_lane) ===== *)
(* ===================================================================== *)
(* PART 2 — shuffle-lane preservation (PROVEN, the hard bit-level core)   *)
(* ===================================================================== *)

#push-options "--z3rlimit 200"
let shuffle_lane_aux (vec ctrl: bv128) (t: u64{v t<4}) (j:u64{v j<4}) (b:u64{v b<32})
  : Lemma (requires (forall (r:nat{r<4}). v (to_i8x16 ctrl (mk_u64 (4 * v t + r))) == 4 * v j + r))
          (ensures i32_to_bv (to_i32x4 (I.mm_shuffle_epi8 vec ctrl) t) b == i32_to_bv (to_i32x4 vec j) b) =
  let r0 : nat = v b / 8 in
  FStar.Math.Lemmas.lemma_div_plus (v b) (4 * v t) 8;
  FStar.Math.Lemmas.lemma_mod_plus (v b) (4 * v t) 8;
  FStar.Math.Lemmas.euclidean_division_definition (v b) 8;
  assert (v (to_i8x16 ctrl (mk_u64 (4 * v t + r0))) == 4 * v j + r0);
  ()
#pop-options

(* If output lane t's 4 control bytes point at input lane j, output lane t == input lane j. *)
let shuffle_lane_lemma (vec ctrl: bv128) (t: u64{v t<4}) (j:u64{v j<4})
  : Lemma (requires (forall (r:nat{r<4}). v (to_i8x16 ctrl (mk_u64 (4 * v t + r))) == 4 * v j + r))
          (ensures to_i32x4 (I.mm_shuffle_epi8 vec ctrl) t == to_i32x4 vec j) =
  let res = I.mm_shuffle_epi8 vec ctrl in
  let aux (b:u64{v b<32}) : Lemma (i32_to_bv (to_i32x4 res t) b == i32_to_bv (to_i32x4 vec j) b) =
    shuffle_lane_aux vec ctrl t j b
  in
  FStar.Classical.forall_intro aux;
  i32_to_bv_ext (to_i32x4 res t) (to_i32x4 vec j)

(* ===================================================================== *)
(* PART 3 — table indexing via the proven zeta-recipe (lemma_seq_of_list_index) *)
(* ===================================================================== *)

let r0 : list u8 = [mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r1 : list u8 = [mk_u8 0;mk_u8 1;mk_u8 2;mk_u8 3;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r2 : list u8 = [mk_u8 4;mk_u8 5;mk_u8 6;mk_u8 7;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r3 : list u8 = [mk_u8 0;mk_u8 1;mk_u8 2;mk_u8 3;mk_u8 4;mk_u8 5;mk_u8 6;mk_u8 7;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r4 : list u8 = [mk_u8 8;mk_u8 9;mk_u8 10;mk_u8 11;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r5 : list u8 = [mk_u8 0;mk_u8 1;mk_u8 2;mk_u8 3;mk_u8 8;mk_u8 9;mk_u8 10;mk_u8 11;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r6 : list u8 = [mk_u8 4;mk_u8 5;mk_u8 6;mk_u8 7;mk_u8 8;mk_u8 9;mk_u8 10;mk_u8 11;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r7 : list u8 = [mk_u8 0;mk_u8 1;mk_u8 2;mk_u8 3;mk_u8 4;mk_u8 5;mk_u8 6;mk_u8 7;mk_u8 8;mk_u8 9;mk_u8 10;mk_u8 11;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r8 : list u8 = [mk_u8 12;mk_u8 13;mk_u8 14;mk_u8 15;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r9 : list u8 = [mk_u8 0;mk_u8 1;mk_u8 2;mk_u8 3;mk_u8 12;mk_u8 13;mk_u8 14;mk_u8 15;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r10 : list u8 = [mk_u8 4;mk_u8 5;mk_u8 6;mk_u8 7;mk_u8 12;mk_u8 13;mk_u8 14;mk_u8 15;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r11 : list u8 = [mk_u8 0;mk_u8 1;mk_u8 2;mk_u8 3;mk_u8 4;mk_u8 5;mk_u8 6;mk_u8 7;mk_u8 12;mk_u8 13;mk_u8 14;mk_u8 15;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r12 : list u8 = [mk_u8 8;mk_u8 9;mk_u8 10;mk_u8 11;mk_u8 12;mk_u8 13;mk_u8 14;mk_u8 15;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r13 : list u8 = [mk_u8 0;mk_u8 1;mk_u8 2;mk_u8 3;mk_u8 8;mk_u8 9;mk_u8 10;mk_u8 11;mk_u8 12;mk_u8 13;mk_u8 14;mk_u8 15;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r14 : list u8 = [mk_u8 4;mk_u8 5;mk_u8 6;mk_u8 7;mk_u8 8;mk_u8 9;mk_u8 10;mk_u8 11;mk_u8 12;mk_u8 13;mk_u8 14;mk_u8 15;mk_u8 255;mk_u8 255;mk_u8 255;mk_u8 255]
let r15 : list u8 = [mk_u8 0;mk_u8 1;mk_u8 2;mk_u8 3;mk_u8 4;mk_u8 5;mk_u8 6;mk_u8 7;mk_u8 8;mk_u8 9;mk_u8 10;mk_u8 11;mk_u8 12;mk_u8 13;mk_u8 14;mk_u8 15]

#push-options "--fuel 20"
let table_rows : list (t_Array u8 (sz 16)) =
  [Seq.seq_of_list r0; Seq.seq_of_list r1; Seq.seq_of_list r2; Seq.seq_of_list r3;
   Seq.seq_of_list r4; Seq.seq_of_list r5; Seq.seq_of_list r6; Seq.seq_of_list r7;
   Seq.seq_of_list r8; Seq.seq_of_list r9; Seq.seq_of_list r10; Seq.seq_of_list r11;
   Seq.seq_of_list r12; Seq.seq_of_list r13; Seq.seq_of_list r14; Seq.seq_of_list r15]

let lemma_table_unfold () : Lemma (ST.v_SHUFFLE_TABLE == Seq.seq_of_list table_rows) =
  assert (ST.v_SHUFFLE_TABLE == Seq.seq_of_list table_rows)
    by (FStar.Tactics.norm [delta_only [`%ST.v_SHUFFLE_TABLE; `%Rust_primitives.Hax.array_of_list;
                                        `%table_rows; `%r0;`%r1;`%r2;`%r3;`%r4;`%r5;`%r6;`%r7;
                                        `%r8;`%r9;`%r10;`%r11;`%r12;`%r13;`%r14;`%r15];
                            iota; zeta; primops];
        FStar.Tactics.trefl ())

(* extract three bytes of row 5 the zeta way, 2-level *)
let lemma_row5_bytes () : Lemma (v (Seq.index (Seq.index ST.v_SHUFFLE_TABLE 5) 0) == 0 /\
                                 v (Seq.index (Seq.index ST.v_SHUFFLE_TABLE 5) 4) == 8 /\
                                 v (Seq.index (Seq.index ST.v_SHUFFLE_TABLE 5) 8) == 255) =
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 5;
  assert_norm (List.Tot.index table_rows 5 == Seq.seq_of_list r5);
  FStar.Seq.Properties.lemma_seq_of_list_index r5 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 4;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 8;
  assert_norm (v (List.Tot.index r5 0) == 0);
  assert_norm (v (List.Tot.index r5 4) == 8);
  assert_norm (v (List.Tot.index r5 8) == 255)
#pop-options

(* ===================================================================== *)
(* PART 4 — per-mask compaction bridge (table bytes -> shuffled lane).     *)
(* ===================================================================== *)

(* Reusable: given that the 4 control bytes of row m for output lane t point at
   input lane j, the shuffled output i32-lane t equals input i32-lane j. *)
#push-options "--z3rlimit 100"
let shuffle_table_lane (cand128: bv128) (m: nat{m<16}) (t: nat{t<4}) (j: nat{j<4})
  : Lemma (requires (forall (r:nat{r<4}). v (Seq.index (Seq.index ST.v_SHUFFLE_TABLE m) (4*t+r)) == 4*j+r))
          (ensures to_i32x4 (I.mm_shuffle_epi8 cand128
                              (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE m <: t_Slice u8)))
                            (mk_u64 t)
                   == to_i32x4 cand128 (mk_u64 j))
  = shuffle_lane_lemma cand128 (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE m <: t_Slice u8))
        (mk_u64 t) (mk_u64 j)
#pop-options

(* Concrete template: mask m=5 (lanes 0 and 2 accepted) — lower-half compaction.
   row5 = [0,1,2,3, 8,9,10,11, 255..]; out lane0 <- in lane0, out lane1 <- in lane2. *)
#push-options "--z3rlimit 100"
let lemma_compact_m5 (cand128: bv128)
  : Lemma (let ctrl = I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 5 <: t_Slice u8) in
           let sh = I.mm_shuffle_epi8 cand128 ctrl in
           to_i32x4 sh (mk_u64 0) == to_i32x4 cand128 (mk_u64 0) /\
           to_i32x4 sh (mk_u64 1) == to_i32x4 cand128 (mk_u64 2)) =
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 5;
  assert_norm (List.Tot.index table_rows 5 == Seq.seq_of_list r5);
  FStar.Seq.Properties.lemma_seq_of_list_index r5 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 3;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 4;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 5;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 6;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 7;
  assert_norm (v (List.Tot.index r5 0) == 0); assert_norm (v (List.Tot.index r5 1) == 1);
  assert_norm (v (List.Tot.index r5 2) == 2); assert_norm (v (List.Tot.index r5 3) == 3);
  assert_norm (v (List.Tot.index r5 4) == 8); assert_norm (v (List.Tot.index r5 5) == 9);
  assert_norm (v (List.Tot.index r5 6) == 10); assert_norm (v (List.Tot.index r5 7) == 11);
  shuffle_table_lane cand128 5 0 0;
  shuffle_table_lane cand128 5 1 2
#pop-options

(* ===== from ScratchRej2 (combinators + per-mask + lemma_compact_lane) ===== *)
(* ===================================================================== *)
(* Combinators: nibble popcount + nth-set-bit (pure, reduce on concrete)  *)
(* ===================================================================== *)
let bitj (m:nat) (j:nat) : n:nat{n<=1} = (match j with | 0 -> m%2 | 1 -> (m/2)%2 | 2 -> (m/4)%2 | _ -> (m/8)%2)
let popcount4 (m:nat{m<16}) : p:nat{p<=4} = (bitj m 0) + (bitj m 1) + (bitj m 2) + (bitj m 3)
let rec nth_set_bit_from (m:nat{m<16}) (t:nat) (j:nat{j<=4}) : Tot (n:nat{n<=4}) (decreases (4 - j)) =
  if j >= 4 then 4
  else if bitj m j = 1 then (if t = 0 then j else nth_set_bit_from m (t-1) (j+1))
  else nth_set_bit_from m t (j+1)
let nth_set_bit (m:nat{m<16}) (t:nat) : n:nat{n<4} =
  let x = nth_set_bit_from m t 0 in if x < 4 then x else 0

(* ===================================================================== *)
(* Per-mask compaction lemmas (generic nth_set_bit form).                 *)
(*   to_i32x4 (shuffle cand128 loadu(TABLE[m])) t == cand128 lane (nth_set_bit m t)  *)
(* The SHUFFLE_TABLE row m front-packs the set bits of m: for accepted    *)
(* lane t (t<popcount4 m), control bytes 4t..4t+3 = 4j..4j+3 (j=nth bit). *)
(* ===================================================================== *)

#push-options "--z3rlimit 150"

let lemma_cl_0 (cand128: bv128) (t: nat{t < popcount4 0})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 0 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 0 t))) =
  assert_norm (popcount4 0 == 0)

let lemma_cl_1 (cand128: bv128) (t: nat{t < popcount4 1})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 1 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 1 t))) =
  assert_norm (popcount4 1 == 1);
  assert_norm (nth_set_bit 1 0 == 0);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 1;
  assert_norm (List.Tot.index table_rows 1 == Seq.seq_of_list r1);
  FStar.Seq.Properties.lemma_seq_of_list_index r1 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r1 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r1 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r1 3;
  assert_norm (v (List.Tot.index r1 0) == 0); assert_norm (v (List.Tot.index r1 1) == 1);
  assert_norm (v (List.Tot.index r1 2) == 2); assert_norm (v (List.Tot.index r1 3) == 3);
  shuffle_table_lane cand128 1 0 0

let lemma_cl_2 (cand128: bv128) (t: nat{t < popcount4 2})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 2 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 2 t))) =
  assert_norm (popcount4 2 == 1);
  assert_norm (nth_set_bit 2 0 == 1);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 2;
  assert_norm (List.Tot.index table_rows 2 == Seq.seq_of_list r2);
  FStar.Seq.Properties.lemma_seq_of_list_index r2 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r2 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r2 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r2 3;
  assert_norm (v (List.Tot.index r2 0) == 4); assert_norm (v (List.Tot.index r2 1) == 5);
  assert_norm (v (List.Tot.index r2 2) == 6); assert_norm (v (List.Tot.index r2 3) == 7);
  shuffle_table_lane cand128 2 0 1

let lemma_cl_3 (cand128: bv128) (t: nat{t < popcount4 3})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 3 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 3 t))) =
  assert_norm (popcount4 3 == 2);
  assert_norm (nth_set_bit 3 0 == 0 /\ nth_set_bit 3 1 == 1);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 3;
  assert_norm (List.Tot.index table_rows 3 == Seq.seq_of_list r3);
  FStar.Seq.Properties.lemma_seq_of_list_index r3 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r3 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r3 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r3 3;
  FStar.Seq.Properties.lemma_seq_of_list_index r3 4;
  FStar.Seq.Properties.lemma_seq_of_list_index r3 5;
  FStar.Seq.Properties.lemma_seq_of_list_index r3 6;
  FStar.Seq.Properties.lemma_seq_of_list_index r3 7;
  assert_norm (v (List.Tot.index r3 0) == 0); assert_norm (v (List.Tot.index r3 1) == 1);
  assert_norm (v (List.Tot.index r3 2) == 2); assert_norm (v (List.Tot.index r3 3) == 3);
  assert_norm (v (List.Tot.index r3 4) == 4); assert_norm (v (List.Tot.index r3 5) == 5);
  assert_norm (v (List.Tot.index r3 6) == 6); assert_norm (v (List.Tot.index r3 7) == 7);
  shuffle_table_lane cand128 3 0 0;
  shuffle_table_lane cand128 3 1 1

let lemma_cl_4 (cand128: bv128) (t: nat{t < popcount4 4})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 4 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 4 t))) =
  assert_norm (popcount4 4 == 1);
  assert_norm (nth_set_bit 4 0 == 2);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 4;
  assert_norm (List.Tot.index table_rows 4 == Seq.seq_of_list r4);
  FStar.Seq.Properties.lemma_seq_of_list_index r4 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r4 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r4 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r4 3;
  assert_norm (v (List.Tot.index r4 0) == 8); assert_norm (v (List.Tot.index r4 1) == 9);
  assert_norm (v (List.Tot.index r4 2) == 10); assert_norm (v (List.Tot.index r4 3) == 11);
  shuffle_table_lane cand128 4 0 2

let lemma_cl_5 (cand128: bv128) (t: nat{t < popcount4 5})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 5 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 5 t))) =
  assert_norm (popcount4 5 == 2);
  assert_norm (nth_set_bit 5 0 == 0 /\ nth_set_bit 5 1 == 2);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 5;
  assert_norm (List.Tot.index table_rows 5 == Seq.seq_of_list r5);
  FStar.Seq.Properties.lemma_seq_of_list_index r5 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 3;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 4;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 5;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 6;
  FStar.Seq.Properties.lemma_seq_of_list_index r5 7;
  assert_norm (v (List.Tot.index r5 0) == 0); assert_norm (v (List.Tot.index r5 1) == 1);
  assert_norm (v (List.Tot.index r5 2) == 2); assert_norm (v (List.Tot.index r5 3) == 3);
  assert_norm (v (List.Tot.index r5 4) == 8); assert_norm (v (List.Tot.index r5 5) == 9);
  assert_norm (v (List.Tot.index r5 6) == 10); assert_norm (v (List.Tot.index r5 7) == 11);
  shuffle_table_lane cand128 5 0 0;
  shuffle_table_lane cand128 5 1 2

let lemma_cl_6 (cand128: bv128) (t: nat{t < popcount4 6})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 6 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 6 t))) =
  assert_norm (popcount4 6 == 2);
  assert_norm (nth_set_bit 6 0 == 1 /\ nth_set_bit 6 1 == 2);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 6;
  assert_norm (List.Tot.index table_rows 6 == Seq.seq_of_list r6);
  FStar.Seq.Properties.lemma_seq_of_list_index r6 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r6 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r6 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r6 3;
  FStar.Seq.Properties.lemma_seq_of_list_index r6 4;
  FStar.Seq.Properties.lemma_seq_of_list_index r6 5;
  FStar.Seq.Properties.lemma_seq_of_list_index r6 6;
  FStar.Seq.Properties.lemma_seq_of_list_index r6 7;
  assert_norm (v (List.Tot.index r6 0) == 4); assert_norm (v (List.Tot.index r6 1) == 5);
  assert_norm (v (List.Tot.index r6 2) == 6); assert_norm (v (List.Tot.index r6 3) == 7);
  assert_norm (v (List.Tot.index r6 4) == 8); assert_norm (v (List.Tot.index r6 5) == 9);
  assert_norm (v (List.Tot.index r6 6) == 10); assert_norm (v (List.Tot.index r6 7) == 11);
  shuffle_table_lane cand128 6 0 1;
  shuffle_table_lane cand128 6 1 2

let lemma_cl_7 (cand128: bv128) (t: nat{t < popcount4 7})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 7 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 7 t))) =
  assert_norm (popcount4 7 == 3);
  assert_norm (nth_set_bit 7 0 == 0 /\ nth_set_bit 7 1 == 1 /\ nth_set_bit 7 2 == 2);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 7;
  assert_norm (List.Tot.index table_rows 7 == Seq.seq_of_list r7);
  FStar.Seq.Properties.lemma_seq_of_list_index r7 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r7 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r7 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r7 3;
  FStar.Seq.Properties.lemma_seq_of_list_index r7 4;
  FStar.Seq.Properties.lemma_seq_of_list_index r7 5;
  FStar.Seq.Properties.lemma_seq_of_list_index r7 6;
  FStar.Seq.Properties.lemma_seq_of_list_index r7 7;
  FStar.Seq.Properties.lemma_seq_of_list_index r7 8;
  FStar.Seq.Properties.lemma_seq_of_list_index r7 9;
  FStar.Seq.Properties.lemma_seq_of_list_index r7 10;
  FStar.Seq.Properties.lemma_seq_of_list_index r7 11;
  assert_norm (v (List.Tot.index r7 0) == 0); assert_norm (v (List.Tot.index r7 1) == 1);
  assert_norm (v (List.Tot.index r7 2) == 2); assert_norm (v (List.Tot.index r7 3) == 3);
  assert_norm (v (List.Tot.index r7 4) == 4); assert_norm (v (List.Tot.index r7 5) == 5);
  assert_norm (v (List.Tot.index r7 6) == 6); assert_norm (v (List.Tot.index r7 7) == 7);
  assert_norm (v (List.Tot.index r7 8) == 8); assert_norm (v (List.Tot.index r7 9) == 9);
  assert_norm (v (List.Tot.index r7 10) == 10); assert_norm (v (List.Tot.index r7 11) == 11);
  shuffle_table_lane cand128 7 0 0;
  shuffle_table_lane cand128 7 1 1;
  shuffle_table_lane cand128 7 2 2

let lemma_cl_8 (cand128: bv128) (t: nat{t < popcount4 8})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 8 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 8 t))) =
  assert_norm (popcount4 8 == 1);
  assert_norm (nth_set_bit 8 0 == 3);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 8;
  assert_norm (List.Tot.index table_rows 8 == Seq.seq_of_list r8);
  FStar.Seq.Properties.lemma_seq_of_list_index r8 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r8 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r8 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r8 3;
  assert_norm (v (List.Tot.index r8 0) == 12); assert_norm (v (List.Tot.index r8 1) == 13);
  assert_norm (v (List.Tot.index r8 2) == 14); assert_norm (v (List.Tot.index r8 3) == 15);
  shuffle_table_lane cand128 8 0 3

let lemma_cl_9 (cand128: bv128) (t: nat{t < popcount4 9})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 9 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 9 t))) =
  assert_norm (popcount4 9 == 2);
  assert_norm (nth_set_bit 9 0 == 0 /\ nth_set_bit 9 1 == 3);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 9;
  assert_norm (List.Tot.index table_rows 9 == Seq.seq_of_list r9);
  FStar.Seq.Properties.lemma_seq_of_list_index r9 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r9 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r9 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r9 3;
  FStar.Seq.Properties.lemma_seq_of_list_index r9 4;
  FStar.Seq.Properties.lemma_seq_of_list_index r9 5;
  FStar.Seq.Properties.lemma_seq_of_list_index r9 6;
  FStar.Seq.Properties.lemma_seq_of_list_index r9 7;
  assert_norm (v (List.Tot.index r9 0) == 0); assert_norm (v (List.Tot.index r9 1) == 1);
  assert_norm (v (List.Tot.index r9 2) == 2); assert_norm (v (List.Tot.index r9 3) == 3);
  assert_norm (v (List.Tot.index r9 4) == 12); assert_norm (v (List.Tot.index r9 5) == 13);
  assert_norm (v (List.Tot.index r9 6) == 14); assert_norm (v (List.Tot.index r9 7) == 15);
  shuffle_table_lane cand128 9 0 0;
  shuffle_table_lane cand128 9 1 3

let lemma_cl_10 (cand128: bv128) (t: nat{t < popcount4 10})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 10 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 10 t))) =
  assert_norm (popcount4 10 == 2);
  assert_norm (nth_set_bit 10 0 == 1 /\ nth_set_bit 10 1 == 3);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 10;
  assert_norm (List.Tot.index table_rows 10 == Seq.seq_of_list r10);
  FStar.Seq.Properties.lemma_seq_of_list_index r10 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r10 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r10 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r10 3;
  FStar.Seq.Properties.lemma_seq_of_list_index r10 4;
  FStar.Seq.Properties.lemma_seq_of_list_index r10 5;
  FStar.Seq.Properties.lemma_seq_of_list_index r10 6;
  FStar.Seq.Properties.lemma_seq_of_list_index r10 7;
  assert_norm (v (List.Tot.index r10 0) == 4); assert_norm (v (List.Tot.index r10 1) == 5);
  assert_norm (v (List.Tot.index r10 2) == 6); assert_norm (v (List.Tot.index r10 3) == 7);
  assert_norm (v (List.Tot.index r10 4) == 12); assert_norm (v (List.Tot.index r10 5) == 13);
  assert_norm (v (List.Tot.index r10 6) == 14); assert_norm (v (List.Tot.index r10 7) == 15);
  shuffle_table_lane cand128 10 0 1;
  shuffle_table_lane cand128 10 1 3

let lemma_cl_11 (cand128: bv128) (t: nat{t < popcount4 11})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 11 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 11 t))) =
  assert_norm (popcount4 11 == 3);
  assert_norm (nth_set_bit 11 0 == 0 /\ nth_set_bit 11 1 == 1 /\ nth_set_bit 11 2 == 3);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 11;
  assert_norm (List.Tot.index table_rows 11 == Seq.seq_of_list r11);
  FStar.Seq.Properties.lemma_seq_of_list_index r11 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r11 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r11 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r11 3;
  FStar.Seq.Properties.lemma_seq_of_list_index r11 4;
  FStar.Seq.Properties.lemma_seq_of_list_index r11 5;
  FStar.Seq.Properties.lemma_seq_of_list_index r11 6;
  FStar.Seq.Properties.lemma_seq_of_list_index r11 7;
  FStar.Seq.Properties.lemma_seq_of_list_index r11 8;
  FStar.Seq.Properties.lemma_seq_of_list_index r11 9;
  FStar.Seq.Properties.lemma_seq_of_list_index r11 10;
  FStar.Seq.Properties.lemma_seq_of_list_index r11 11;
  assert_norm (v (List.Tot.index r11 0) == 0); assert_norm (v (List.Tot.index r11 1) == 1);
  assert_norm (v (List.Tot.index r11 2) == 2); assert_norm (v (List.Tot.index r11 3) == 3);
  assert_norm (v (List.Tot.index r11 4) == 4); assert_norm (v (List.Tot.index r11 5) == 5);
  assert_norm (v (List.Tot.index r11 6) == 6); assert_norm (v (List.Tot.index r11 7) == 7);
  assert_norm (v (List.Tot.index r11 8) == 12); assert_norm (v (List.Tot.index r11 9) == 13);
  assert_norm (v (List.Tot.index r11 10) == 14); assert_norm (v (List.Tot.index r11 11) == 15);
  shuffle_table_lane cand128 11 0 0;
  shuffle_table_lane cand128 11 1 1;
  shuffle_table_lane cand128 11 2 3

let lemma_cl_12 (cand128: bv128) (t: nat{t < popcount4 12})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 12 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 12 t))) =
  assert_norm (popcount4 12 == 2);
  assert_norm (nth_set_bit 12 0 == 2 /\ nth_set_bit 12 1 == 3);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 12;
  assert_norm (List.Tot.index table_rows 12 == Seq.seq_of_list r12);
  FStar.Seq.Properties.lemma_seq_of_list_index r12 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r12 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r12 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r12 3;
  FStar.Seq.Properties.lemma_seq_of_list_index r12 4;
  FStar.Seq.Properties.lemma_seq_of_list_index r12 5;
  FStar.Seq.Properties.lemma_seq_of_list_index r12 6;
  FStar.Seq.Properties.lemma_seq_of_list_index r12 7;
  assert_norm (v (List.Tot.index r12 0) == 8); assert_norm (v (List.Tot.index r12 1) == 9);
  assert_norm (v (List.Tot.index r12 2) == 10); assert_norm (v (List.Tot.index r12 3) == 11);
  assert_norm (v (List.Tot.index r12 4) == 12); assert_norm (v (List.Tot.index r12 5) == 13);
  assert_norm (v (List.Tot.index r12 6) == 14); assert_norm (v (List.Tot.index r12 7) == 15);
  shuffle_table_lane cand128 12 0 2;
  shuffle_table_lane cand128 12 1 3

let lemma_cl_13 (cand128: bv128) (t: nat{t < popcount4 13})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 13 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 13 t))) =
  assert_norm (popcount4 13 == 3);
  assert_norm (nth_set_bit 13 0 == 0 /\ nth_set_bit 13 1 == 2 /\ nth_set_bit 13 2 == 3);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 13;
  assert_norm (List.Tot.index table_rows 13 == Seq.seq_of_list r13);
  FStar.Seq.Properties.lemma_seq_of_list_index r13 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r13 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r13 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r13 3;
  FStar.Seq.Properties.lemma_seq_of_list_index r13 4;
  FStar.Seq.Properties.lemma_seq_of_list_index r13 5;
  FStar.Seq.Properties.lemma_seq_of_list_index r13 6;
  FStar.Seq.Properties.lemma_seq_of_list_index r13 7;
  FStar.Seq.Properties.lemma_seq_of_list_index r13 8;
  FStar.Seq.Properties.lemma_seq_of_list_index r13 9;
  FStar.Seq.Properties.lemma_seq_of_list_index r13 10;
  FStar.Seq.Properties.lemma_seq_of_list_index r13 11;
  assert_norm (v (List.Tot.index r13 0) == 0); assert_norm (v (List.Tot.index r13 1) == 1);
  assert_norm (v (List.Tot.index r13 2) == 2); assert_norm (v (List.Tot.index r13 3) == 3);
  assert_norm (v (List.Tot.index r13 4) == 8); assert_norm (v (List.Tot.index r13 5) == 9);
  assert_norm (v (List.Tot.index r13 6) == 10); assert_norm (v (List.Tot.index r13 7) == 11);
  assert_norm (v (List.Tot.index r13 8) == 12); assert_norm (v (List.Tot.index r13 9) == 13);
  assert_norm (v (List.Tot.index r13 10) == 14); assert_norm (v (List.Tot.index r13 11) == 15);
  shuffle_table_lane cand128 13 0 0;
  shuffle_table_lane cand128 13 1 2;
  shuffle_table_lane cand128 13 2 3

let lemma_cl_14 (cand128: bv128) (t: nat{t < popcount4 14})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 14 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 14 t))) =
  assert_norm (popcount4 14 == 3);
  assert_norm (nth_set_bit 14 0 == 1 /\ nth_set_bit 14 1 == 2 /\ nth_set_bit 14 2 == 3);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 14;
  assert_norm (List.Tot.index table_rows 14 == Seq.seq_of_list r14);
  FStar.Seq.Properties.lemma_seq_of_list_index r14 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r14 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r14 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r14 3;
  FStar.Seq.Properties.lemma_seq_of_list_index r14 4;
  FStar.Seq.Properties.lemma_seq_of_list_index r14 5;
  FStar.Seq.Properties.lemma_seq_of_list_index r14 6;
  FStar.Seq.Properties.lemma_seq_of_list_index r14 7;
  FStar.Seq.Properties.lemma_seq_of_list_index r14 8;
  FStar.Seq.Properties.lemma_seq_of_list_index r14 9;
  FStar.Seq.Properties.lemma_seq_of_list_index r14 10;
  FStar.Seq.Properties.lemma_seq_of_list_index r14 11;
  assert_norm (v (List.Tot.index r14 0) == 4); assert_norm (v (List.Tot.index r14 1) == 5);
  assert_norm (v (List.Tot.index r14 2) == 6); assert_norm (v (List.Tot.index r14 3) == 7);
  assert_norm (v (List.Tot.index r14 4) == 8); assert_norm (v (List.Tot.index r14 5) == 9);
  assert_norm (v (List.Tot.index r14 6) == 10); assert_norm (v (List.Tot.index r14 7) == 11);
  assert_norm (v (List.Tot.index r14 8) == 12); assert_norm (v (List.Tot.index r14 9) == 13);
  assert_norm (v (List.Tot.index r14 10) == 14); assert_norm (v (List.Tot.index r14 11) == 15);
  shuffle_table_lane cand128 14 0 1;
  shuffle_table_lane cand128 14 1 2;
  shuffle_table_lane cand128 14 2 3

let lemma_cl_15 (cand128: bv128) (t: nat{t < popcount4 15})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE 15 <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit 15 t))) =
  assert_norm (popcount4 15 == 4);
  assert_norm (nth_set_bit 15 0 == 0 /\ nth_set_bit 15 1 == 1 /\ nth_set_bit 15 2 == 2 /\ nth_set_bit 15 3 == 3);
  lemma_table_unfold ();
  FStar.Seq.Properties.lemma_seq_of_list_index table_rows 15;
  assert_norm (List.Tot.index table_rows 15 == Seq.seq_of_list r15);
  FStar.Seq.Properties.lemma_seq_of_list_index r15 0;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 1;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 2;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 3;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 4;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 5;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 6;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 7;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 8;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 9;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 10;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 11;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 12;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 13;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 14;
  FStar.Seq.Properties.lemma_seq_of_list_index r15 15;
  assert_norm (v (List.Tot.index r15 0) == 0); assert_norm (v (List.Tot.index r15 1) == 1);
  assert_norm (v (List.Tot.index r15 2) == 2); assert_norm (v (List.Tot.index r15 3) == 3);
  assert_norm (v (List.Tot.index r15 4) == 4); assert_norm (v (List.Tot.index r15 5) == 5);
  assert_norm (v (List.Tot.index r15 6) == 6); assert_norm (v (List.Tot.index r15 7) == 7);
  assert_norm (v (List.Tot.index r15 8) == 8); assert_norm (v (List.Tot.index r15 9) == 9);
  assert_norm (v (List.Tot.index r15 10) == 10); assert_norm (v (List.Tot.index r15 11) == 11);
  assert_norm (v (List.Tot.index r15 12) == 12); assert_norm (v (List.Tot.index r15 13) == 13);
  assert_norm (v (List.Tot.index r15 14) == 14); assert_norm (v (List.Tot.index r15 15) == 15);
  shuffle_table_lane cand128 15 0 0;
  shuffle_table_lane cand128 15 1 1;
  shuffle_table_lane cand128 15 2 2;
  shuffle_table_lane cand128 15 3 3

#pop-options

(* Generic dispatcher: lane t (t<popcount4 m) of the shuffled compaction == cand lane (nth_set_bit m t). *)
let lemma_compact_lane (cand128: bv128) (m: nat{m<16}) (t: nat{t < popcount4 m})
  : Lemma (to_i32x4 (I.mm_shuffle_epi8 cand128
                      (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE m <: t_Slice u8)))
                    (mk_u64 t)
           == to_i32x4 cand128 (mk_u64 (nth_set_bit m t))) =
  if m = 0 then lemma_cl_0 cand128 t
  else if m = 1 then lemma_cl_1 cand128 t
  else if m = 2 then lemma_cl_2 cand128 t
  else if m = 3 then lemma_cl_3 cand128 t
  else if m = 4 then lemma_cl_4 cand128 t
  else if m = 5 then lemma_cl_5 cand128 t
  else if m = 6 then lemma_cl_6 cand128 t
  else if m = 7 then lemma_cl_7 cand128 t
  else if m = 8 then lemma_cl_8 cand128 t
  else if m = 9 then lemma_cl_9 cand128 t
  else if m = 10 then lemma_cl_10 cand128 t
  else if m = 11 then lemma_cl_11 cand128 t
  else if m = 12 then lemma_cl_12 cand128 t
  else if m = 13 then lemma_cl_13 cand128 t
  else if m = 14 then lemma_cl_14 cand128 t
  else lemma_cl_15 cand128 t

(* ===== from ScratchRej3 (mask4 / filt4 window bridge) ===== *)
(* ===================================================================== *)
(* Layer D combinatorial bridge: the (popcount4, nth_set_bit) compaction  *)
(* representation (Layer C output) == the repeati-filter (spec building   *)
(* block), at the 4-lane window level.                                    *)
(* ===================================================================== *)

(* 4-bit mask of an accept predicate over lanes 0..3 *)
let mask4 (acc: (j:nat{j<4}) -> bool) : m:nat{m<16} =
  (if acc 0 then 1 else 0) + (if acc 1 then 2 else 0) + (if acc 2 then 4 else 0) + (if acc 3 then 8 else 0)

(* bit-decomposition bridge: linchpin relating mask4 back to acc, bit by bit *)
#push-options "--ifuel 4"
let lemma_mask4_bitj (acc: (j:nat{j<4}) -> bool) (j:nat{j<4})
  : Lemma (bitj (mask4 acc) j == (if acc j then 1 else 0)) = ()
let lemma_mask4_popcount (acc: (j:nat{j<4}) -> bool)
  : Lemma (popcount4 (mask4 acc) ==
           (if acc 0 then 1 else 0) + (if acc 1 then 1 else 0) +
           (if acc 2 then 1 else 0) + (if acc 3 then 1 else 0)) =
  lemma_mask4_bitj acc 0; lemma_mask4_bitj acc 1; lemma_mask4_bitj acc 2; lemma_mask4_bitj acc 3
#pop-options

(* window repeati-filter over 4 lanes (mirrors rejection_sample_*_inner restricted to a window) *)
let step4 (cand:(j:nat{j<4})->i32) (acc:(j:nat{j<4})->bool) (i:usize{v i<4}) (s:Seq.seq i32) : Seq.seq i32 =
  if acc (v i) then Seq.append s (Seq.create 1 (cand (v i))) else s
let filt4 (cand:(j:nat{j<4})->i32) (acc:(j:nat{j<4})->bool) : Seq.seq i32 =
  SU.repeati (sz 4) (step4 cand acc) Seq.empty

#push-options "--ifuel 4 --z3rlimit 200"
let lemma_filt4_unfold (cand:(j:nat{j<4})->i32) (acc:(j:nat{j<4})->bool)
  : Lemma (filt4 cand acc ==
           step4 cand acc (sz 3) (step4 cand acc (sz 2)
             (step4 cand acc (sz 1) (step4 cand acc (sz 0) Seq.empty)))) =
  let f = step4 cand acc in
  SU.eq_repeati0 (sz 4) f Seq.empty;
  SU.unfold_repeati (sz 4) f Seq.empty (sz 0);
  SU.unfold_repeati (sz 4) f Seq.empty (sz 1);
  SU.unfold_repeati (sz 4) f Seq.empty (sz 2);
  SU.unfold_repeati (sz 4) f Seq.empty (sz 3)

(* result count: the window filter length == popcount of the accept mask *)
let lemma_filt4_length (cand:(j:nat{j<4})->i32) (acc:(j:nat{j<4})->bool)
  : Lemma (Seq.length (filt4 cand acc) == popcount4 (mask4 acc)) =
  lemma_filt4_unfold cand acc;
  lemma_mask4_popcount acc
#pop-options

(* element t of the window filter == cand at the t-th accepted lane *)
#push-options "--fuel 6 --ifuel 4 --z3rlimit 400"
let lemma_filt4_index (cand:(j:nat{j<4})->i32) (acc:(j:nat{j<4})->bool) (t:nat{t < popcount4 (mask4 acc)})
  : Lemma (requires Seq.length (filt4 cand acc) == popcount4 (mask4 acc))
          (ensures Seq.index (filt4 cand acc) t == cand (nth_set_bit (mask4 acc) t)) =
  lemma_filt4_unfold cand acc
#pop-options

(* ===== from ScratchRej4 (step8 / filt8 / split / filt8_eq_spec field_modulus) ===== *)
(* ===================================================================== *)
(* Layer D two-half assembly: the AVX2 leaf processes 8 coefficients in   *)
(* one call. The spec is repeati (sz 8) inner empty. We prove the         *)
(* repeati-8 filter splits as the append of the two 4-lane window         *)
(* filters (filt4, proven in ScratchRej3). Pure / model-free.            *)
(* Key trick: rephrase the step as `append s (seg j)` so the split is     *)
(* pure Seq.append associativity (no 256-way accept-bool case blast).     *)
(* ===================================================================== *)

(* 8-lane generic step / filter (mirrors rejection_sample_*_inner) *)
let step8 (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool) (i:usize{v i<8}) (s:Seq.seq i32) : Seq.seq i32 =
  if acc (v i) then Seq.append s (Seq.create 1 (cand (v i))) else s
let filt8 (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool) : Seq.seq i32 =
  SU.repeati (sz 8) (step8 cand acc) Seq.empty

(* segment forms: a step appends a (possibly empty) one-element segment *)
let seg8 (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool) (j:nat{j<8}) : Seq.seq i32 =
  if acc j then Seq.create 1 (cand j) else Seq.empty
let seg4 (cand:(j:nat{j<4})->i32) (acc:(j:nat{j<4})->bool) (j:nat{j<4}) : Seq.seq i32 =
  if acc j then Seq.create 1 (cand j) else Seq.empty

let lemma_step8_seg (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool) (i:usize{v i<8}) (s:Seq.seq i32)
  : Lemma (step8 cand acc i s == Seq.append s (seg8 cand acc (v i))) =
  if acc (v i) then () else Seq.append_empty_r s
let lemma_step4_seg (cand:(j:nat{j<4})->i32) (acc:(j:nat{j<4})->bool) (i:usize{v i<4}) (s:Seq.seq i32)
  : Lemma (step4 cand acc i s == Seq.append s (seg4 cand acc (v i))) =
  if acc (v i) then () else Seq.append_empty_r s

(* unfold filt8 to its explicit 8-step form *)
#push-options "--ifuel 4 --z3rlimit 300"
let lemma_filt8_unfold (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool)
  : Lemma (filt8 cand acc ==
           step8 cand acc (sz 7) (step8 cand acc (sz 6) (step8 cand acc (sz 5)
            (step8 cand acc (sz 4) (step8 cand acc (sz 3) (step8 cand acc (sz 2)
             (step8 cand acc (sz 1) (step8 cand acc (sz 0) Seq.empty)))))))) =
  let f = step8 cand acc in
  SU.eq_repeati0 (sz 8) f Seq.empty;
  SU.unfold_repeati (sz 8) f Seq.empty (sz 0);
  SU.unfold_repeati (sz 8) f Seq.empty (sz 1);
  SU.unfold_repeati (sz 8) f Seq.empty (sz 2);
  SU.unfold_repeati (sz 8) f Seq.empty (sz 3);
  SU.unfold_repeati (sz 8) f Seq.empty (sz 4);
  SU.unfold_repeati (sz 8) f Seq.empty (sz 5);
  SU.unfold_repeati (sz 8) f Seq.empty (sz 6);
  SU.unfold_repeati (sz 8) f Seq.empty (sz 7)
#pop-options

(* filt8 in nested-append-of-segments form *)
let lemma_filt8_segform (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool)
  : Lemma (filt8 cand acc ==
      Seq.append (Seq.append (Seq.append (Seq.append (Seq.append (Seq.append (Seq.append (Seq.append
        Seq.empty (seg8 cand acc 0)) (seg8 cand acc 1)) (seg8 cand acc 2)) (seg8 cand acc 3))
        (seg8 cand acc 4)) (seg8 cand acc 5)) (seg8 cand acc 6)) (seg8 cand acc 7)) =
  lemma_filt8_unfold cand acc;
  let e = Seq.empty #i32 in
  let s0 = step8 cand acc (sz 0) e in
  let s1 = step8 cand acc (sz 1) s0 in
  let s2 = step8 cand acc (sz 2) s1 in
  let s3 = step8 cand acc (sz 3) s2 in
  let s4 = step8 cand acc (sz 4) s3 in
  let s5 = step8 cand acc (sz 5) s4 in
  let s6 = step8 cand acc (sz 6) s5 in
  lemma_step8_seg cand acc (sz 0) e;
  lemma_step8_seg cand acc (sz 1) s0;
  lemma_step8_seg cand acc (sz 2) s1;
  lemma_step8_seg cand acc (sz 3) s2;
  lemma_step8_seg cand acc (sz 4) s3;
  lemma_step8_seg cand acc (sz 5) s4;
  lemma_step8_seg cand acc (sz 6) s5;
  lemma_step8_seg cand acc (sz 7) s6

(* filt4 in nested-append-of-segments form (generic 4-lane) *)
let lemma_filt4_segform (cand:(j:nat{j<4})->i32) (acc:(j:nat{j<4})->bool)
  : Lemma (filt4 cand acc ==
      Seq.append (Seq.append (Seq.append (Seq.append
        Seq.empty (seg4 cand acc 0)) (seg4 cand acc 1)) (seg4 cand acc 2)) (seg4 cand acc 3)) =
  lemma_filt4_unfold cand acc;
  let e = Seq.empty #i32 in
  let s0 = step4 cand acc (sz 0) e in
  let s1 = step4 cand acc (sz 1) s0 in
  let s2 = step4 cand acc (sz 2) s1 in
  lemma_step4_seg cand acc (sz 0) e;
  lemma_step4_seg cand acc (sz 1) s0;
  lemma_step4_seg cand acc (sz 2) s1;
  lemma_step4_seg cand acc (sz 3) s2

(* ============ the split ============ *)
#push-options "--z3rlimit 300"
let lemma_filt8_split
  (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool)
  (cl:(j:nat{j<4})->i32) (al:(j:nat{j<4})->bool)
  (cu:(j:nat{j<4})->i32) (au:(j:nat{j<4})->bool)
  : Lemma
    (requires (forall (k:nat{k<4}). cl k == cand k /\ al k == acc k /\
                                    cu k == cand (4+k) /\ au k == acc (4+k)))
    (ensures filt8 cand acc == Seq.append (filt4 cl al) (filt4 cu au)) =
  lemma_filt8_segform cand acc;
  lemma_filt4_segform cl al;
  lemma_filt4_segform cu au;
  (* under the requires: seg4 cl al k == seg8 cand acc k, seg4 cu au k == seg8 cand acc (4+k) *)
  let s0 = seg8 cand acc 0 in let s1 = seg8 cand acc 1 in
  let s2 = seg8 cand acc 2 in let s3 = seg8 cand acc 3 in
  let s4 = seg8 cand acc 4 in let s5 = seg8 cand acc 5 in
  let s6 = seg8 cand acc 6 in let s7 = seg8 cand acc 7 in
  let e = Seq.empty #i32 in
  let bigL = Seq.append (Seq.append (Seq.append (Seq.append e s0) s1) s2) s3 in
  (* L = filt4 cl al = bigL ; assoc rearrangement on the upper four segments *)
  Seq.append_empty_l s4;
  Seq.append_assoc bigL s4 s5;
  Seq.append_assoc bigL (Seq.append s4 s5) s6;
  Seq.append_assoc bigL (Seq.append (Seq.append s4 s5) s6) s7
#pop-options

(* ============ spec connection: filt8 == rejection_sample_field_modulus ============ *)
(* per-lane: step8 (with cand/acc matching the spec coefficients) == the spec inner *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_step8_eq_inner
  (randomness: Seq.seq u8) (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool)
  (k:nat{k<8}) (s:Seq.seq i32)
  : Lemma
    (requires Seq.length randomness == 24 /\
       cand k == Spec.MLDSA.Math.rejection_sample_coefficient randomness (sz k) /\
       acc k == (cand k <. mk_i32 8380417))
    (ensures step8 cand acc (sz k) s ==
             Spec.MLDSA.Math.rejection_sample_field_modulus_inner randomness (sz k) s) =
  ()
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let lemma_filt8_eq_spec
  (randomness: Seq.seq u8) (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool)
  : Lemma
    (requires Seq.length randomness == 24 /\
       (forall (j:nat{j<8}). cand j == Spec.MLDSA.Math.rejection_sample_coefficient randomness (sz j) /\
                             acc j == (cand j <. mk_i32 8380417)))
    (ensures filt8 cand acc == Spec.MLDSA.Math.rejection_sample_field_modulus randomness) =
  let inner = Spec.MLDSA.Math.rejection_sample_field_modulus_inner randomness in
  assert (Seq.length randomness / 3 == 8);
  (* unfold the spec repeati 8 *)
  SU.eq_repeati0 (sz 8) inner Seq.empty;
  SU.unfold_repeati (sz 8) inner Seq.empty (sz 0);
  SU.unfold_repeati (sz 8) inner Seq.empty (sz 1);
  SU.unfold_repeati (sz 8) inner Seq.empty (sz 2);
  SU.unfold_repeati (sz 8) inner Seq.empty (sz 3);
  SU.unfold_repeati (sz 8) inner Seq.empty (sz 4);
  SU.unfold_repeati (sz 8) inner Seq.empty (sz 5);
  SU.unfold_repeati (sz 8) inner Seq.empty (sz 6);
  SU.unfold_repeati (sz 8) inner Seq.empty (sz 7);
  (* unfold filt8 *)
  lemma_filt8_unfold cand acc;
  (* per-step equalities, bottom-up so accumulators are matched by congruence *)
  let e = Seq.empty #i32 in
  let t0 = inner (sz 0) e in
  let t1 = inner (sz 1) t0 in
  let t2 = inner (sz 2) t1 in
  let t3 = inner (sz 3) t2 in
  let t4 = inner (sz 4) t3 in
  let t5 = inner (sz 5) t4 in
  let t6 = inner (sz 6) t5 in
  lemma_step8_eq_inner randomness cand acc 0 e;
  lemma_step8_eq_inner randomness cand acc 1 t0;
  lemma_step8_eq_inner randomness cand acc 2 t1;
  lemma_step8_eq_inner randomness cand acc 3 t2;
  lemma_step8_eq_inner randomness cand acc 4 t3;
  lemma_step8_eq_inner randomness cand acc 5 t4;
  lemma_step8_eq_inner randomness cand acc 6 t5;
  lemma_step8_eq_inner randomness cand acc 7 t6
#pop-options

(* ===== from ScratchRej6 (count_ones_exact + store_two_halves) ===== *)
assume val lemma_count_ones_nibble_exact (x: i32)
  : Lemma (requires v x >= 0 /\ v x < 16)
          (ensures v (Core_models.Num.impl_i32__count_ones x) == popcount4 (v x))

(* candidate lane functions of `potential` *)

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"
let lemma_store_two_halves
      (output: t_Slice i32) (lc uc: bv128) (lo up: Seq.seq i32) (nlo nup: nat)
  : Lemma
    (requires
       Seq.length output >= 8 /\ nlo <= 4 /\ nup <= 4 /\
       Seq.length lo == nlo /\ Seq.length up == nup /\
       (forall (t:nat). t < nlo ==> to_i32x4 lc (mk_u64 t) == Seq.index lo t) /\
       (forall (t:nat). t < nup ==> to_i32x4 uc (mk_u64 t) == Seq.index up t))
    (ensures (
       let o1 = Rust_primitives.Hax.Monomorphized_update_at.update_at_range output
                  ({ R.f_start = mk_usize 0; R.f_end = mk_usize 4 } <: R.t_Range usize)
                  (I.mm_storeu_si128_i32 (Seq.slice output 0 4) lc) in
       let o2 = Rust_primitives.Hax.Monomorphized_update_at.update_at_range o1
                  ({ R.f_start = mk_usize nlo; R.f_end = mk_usize (nlo+4) } <: R.t_Range usize)
                  (I.mm_storeu_si128_i32 (Seq.slice o1 nlo (nlo+4)) uc) in
       Seq.slice o2 0 (nlo+nup) == Seq.append lo up)) =
  let stored_lo = I.mm_storeu_si128_i32 (Seq.slice output 0 4) lc in
  let o1 = Rust_primitives.Hax.Monomorphized_update_at.update_at_range output
             ({ R.f_start = mk_usize 0; R.f_end = mk_usize 4 } <: R.t_Range usize) stored_lo in
  UL.lemma_index_update_at_range output
    ({ R.f_start = mk_usize 0; R.f_end = mk_usize 4 } <: R.t_Range usize) stored_lo;
  let stored_up = I.mm_storeu_si128_i32 (Seq.slice o1 nlo (nlo+4)) uc in
  let o2 = Rust_primitives.Hax.Monomorphized_update_at.update_at_range o1
             ({ R.f_start = mk_usize nlo; R.f_end = mk_usize (nlo+4) } <: R.t_Range usize) stored_up in
  UL.lemma_index_update_at_range o1
    ({ R.f_start = mk_usize nlo; R.f_end = mk_usize (nlo+4) } <: R.t_Range usize) stored_up;
  let res = Seq.append lo up in
  let sl = Seq.slice o2 0 (nlo+nup) in
  let aux (i:nat{i < nlo+nup}) : Lemma (Seq.index sl i == Seq.index res i) =
    Seq.lemma_index_slice o2 0 (nlo+nup) i;
    if i < nlo then begin
      mm_storeu_si128_i32_lemma (Seq.slice output 0 4) lc i;
      Seq.lemma_index_app1 lo up i
    end
    else begin
      mm_storeu_si128_i32_lemma (Seq.slice o1 nlo (nlo+4)) uc (i - nlo);
      Seq.lemma_index_app2 lo up i
    end
  in
  FStar.Classical.forall_intro aux;
  Seq.lemma_eq_intro sl res
#pop-options

(* ===== from ScratchRejEta (generic Layer B/C + structural_g) ===== *)
(* generic accept (good-vector gv compared against boundary), candidate (compact-vector cv) *)
let accept_b (gv: bv256) (boundary: i32) (j:nat{j<8}) : bool = to_i32x8 gv (mk_u64 j) <. boundary
let alb (gv: bv256) (boundary: i32) : (j:nat{j<4}) -> bool = fun j -> accept_b gv boundary j
let aub (gv: bv256) (boundary: i32) : (j:nat{j<4}) -> bool = fun j -> accept_b gv boundary (4+j)
let clv (cv: bv256) : (j:nat{j<4}) -> i32 = fun j -> to_i32x8 cv (mk_u64 j)
let cuv (cv: bv256) : (j:nat{j<4}) -> i32 = fun j -> to_i32x8 cv (mk_u64 (4+j))
let cand8v (cv: bv256) : (j:nat{j<8}) -> i32 = fun j -> to_i32x8 cv (mk_u64 j)
let acc8b (gv: bv256) (boundary: i32) : (j:nat{j<8}) -> bool = fun j -> accept_b gv boundary j

(* ===== Layer B (generic boundary): good nibbles == mask4 of accept ===== *)
let lemma_cmp_sign_b (gv: bv256) (boundary: i32) (j:u64{v j<8})
  : Lemma ((to_i32x8 (I.mm256_cmpgt_epi32 (I.mm256_set1_epi32 boundary) gv) j <. mk_i32 0)
           == accept_b gv boundary (v j)) = ()

#push-options "--z3rlimit 200"
let lemma_good_value_b (gv: bv256) (boundary: i32)
  : Lemma (let cmp = I.mm256_cmpgt_epi32 (I.mm256_set1_epi32 boundary) gv in
           let good = I.mm256_movemask_ps (I.mm256_castsi256_ps cmp) in
           v good ==
           (if accept_b gv boundary 0 then 1 else 0) + (if accept_b gv boundary 1 then 2 else 0) +
           (if accept_b gv boundary 2 then 4 else 0) + (if accept_b gv boundary 3 then 8 else 0) +
           (if accept_b gv boundary 4 then 16 else 0) + (if accept_b gv boundary 5 then 32 else 0) +
           (if accept_b gv boundary 6 then 64 else 0) + (if accept_b gv boundary 7 then 128 else 0)) =
  let cmp = I.mm256_cmpgt_epi32 (I.mm256_set1_epi32 boundary) gv in
  mm256_movemask_ps_lemma cmp;
  lemma_cmp_sign_b gv boundary (mk_u64 0); lemma_cmp_sign_b gv boundary (mk_u64 1);
  lemma_cmp_sign_b gv boundary (mk_u64 2); lemma_cmp_sign_b gv boundary (mk_u64 3);
  lemma_cmp_sign_b gv boundary (mk_u64 4); lemma_cmp_sign_b gv boundary (mk_u64 5);
  lemma_cmp_sign_b gv boundary (mk_u64 6); lemma_cmp_sign_b gv boundary (mk_u64 7)
#pop-options

#push-options "--z3rlimit 200 --fuel 1"
let lemma_good_lower_b (gv: bv256) (boundary: i32)
  : Lemma (let cmp = I.mm256_cmpgt_epi32 (I.mm256_set1_epi32 boundary) gv in
           let good = I.mm256_movemask_ps (I.mm256_castsi256_ps cmp) in
           v (good &. mk_i32 15) == mask4 (alb gv boundary)) =
  lemma_good_value_b gv boundary;
  let good = I.mm256_movemask_ps (I.mm256_castsi256_ps (I.mm256_cmpgt_epi32 (I.mm256_set1_epi32 boundary) gv)) in
  logand_mask_lemma good 4

let lemma_good_upper_b (gv: bv256) (boundary: i32)
  : Lemma (let cmp = I.mm256_cmpgt_epi32 (I.mm256_set1_epi32 boundary) gv in
           let good = I.mm256_movemask_ps (I.mm256_castsi256_ps cmp) in
           v (good >>! mk_i32 4) == mask4 (aub gv boundary)) =
  lemma_good_value_b gv boundary
#pop-options

(* ===== Layer C (generic): per-half compaction of compact-vec cv, accept arbitrary ===== *)
#push-options "--z3rlimit 300 --fuel 1 --ifuel 2"
let lemma_lower_compact_g (cv: bv256) (al: (j:nat{j<4})->bool) (m: nat{m<16})
      (lo: Seq.seq i32) (t:nat{t < 4 /\ t < Seq.length lo})
  : Lemma
    (requires m == mask4 al /\ lo == filt4 (clv cv) al)
    (ensures
       to_i32x4 (I.mm_shuffle_epi8 (I.mm256_castsi256_si128 cv)
                  (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE m <: t_Slice u8)))
                (mk_u64 t)
       == Seq.index lo t) =
  lemma_filt4_length (clv cv) al;
  assert (t < popcount4 (mask4 al));
  assert (popcount4 m == popcount4 (mask4 al));
  assert (t < popcount4 m);
  lemma_compact_lane (I.mm256_castsi256_si128 cv) m t;
  lemma_filt4_index (clv cv) al t

let lemma_upper_compact_g (cv: bv256) (au: (j:nat{j<4})->bool) (m: nat{m<16})
      (up: Seq.seq i32) (t:nat{t < 4 /\ t < Seq.length up})
  : Lemma
    (requires m == mask4 au /\ up == filt4 (cuv cv) au)
    (ensures
       to_i32x4 (I.mm_shuffle_epi8 (I.mm256_extracti128_si256 (mk_i32 1) cv)
                  (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE m <: t_Slice u8)))
                (mk_u64 t)
       == Seq.index up t) =
  lemma_filt4_length (cuv cv) au;
  assert (t < popcount4 (mask4 au));
  assert (popcount4 m == popcount4 (mask4 au));
  assert (t < popcount4 m);
  lemma_compact_lane (I.mm256_extracti128_si256 (mk_i32 1) cv) m t;
  lemma_filt4_index (cuv cv) au t
#pop-options

(* per-half equalities as foralls (consumed by lemma_store_two_halves) *)
#push-options "--z3rlimit 300 --fuel 1 --ifuel 2"
let lemma_lower_forall_g (cv: bv256) (al: (j:nat{j<4})->bool) (m: nat{m<16}) (lo: Seq.seq i32) (nlo:nat{nlo<=4})
  : Lemma (requires m == mask4 al /\ lo == filt4 (clv cv) al /\ Seq.length lo == nlo)
          (ensures (forall (t:nat). t < nlo ==>
                      to_i32x4 (I.mm_shuffle_epi8 (I.mm256_castsi256_si128 cv)
                                 (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE m <: t_Slice u8))) (mk_u64 t)
                      == Seq.index lo t)) =
  let aux (t:nat) : Lemma (t < nlo ==>
      to_i32x4 (I.mm_shuffle_epi8 (I.mm256_castsi256_si128 cv)
                 (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE m <: t_Slice u8))) (mk_u64 t)
      == Seq.index lo t) =
    if t < nlo then lemma_lower_compact_g cv al m lo t else ()
  in
  Classical.forall_intro aux

let lemma_upper_forall_g (cv: bv256) (au: (j:nat{j<4})->bool) (m: nat{m<16}) (up: Seq.seq i32) (nup:nat{nup<=4})
  : Lemma (requires m == mask4 au /\ up == filt4 (cuv cv) au /\ Seq.length up == nup)
          (ensures (forall (t:nat). t < nup ==>
                      to_i32x4 (I.mm_shuffle_epi8 (I.mm256_extracti128_si256 (mk_i32 1) cv)
                                 (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE m <: t_Slice u8))) (mk_u64 t)
                      == Seq.index up t)) =
  let aux (t:nat) : Lemma (t < nup ==>
      to_i32x4 (I.mm_shuffle_epi8 (I.mm256_extracti128_si256 (mk_i32 1) cv)
                 (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE m <: t_Slice u8))) (mk_u64 t)
      == Seq.index up t) =
    if t < nup then lemma_upper_compact_g cv au m up t else ()
  in
  Classical.forall_intro aux
#pop-options

(* generic structural leaf result: out[0..result] == filt8 (cand8v cv)(acc8b gv boundary) *)
#push-options "--z3rlimit 400 --fuel 1 --ifuel 2"
let lemma_leaf_structural_g
  (cv gv: bv256) (boundary: i32) (output: t_Slice i32)
  (good_lower good_upper: i32) (nlo nup: nat)
  : Lemma
    (requires
       Seq.length output >= 8 /\
       v good_lower >= 0 /\ v good_lower < 16 /\ v good_upper >= 0 /\ v good_upper < 16 /\
       nlo <= 4 /\ nup <= 4 /\
       (let good = I.mm256_movemask_ps (I.mm256_castsi256_ps
                     (I.mm256_cmpgt_epi32 (I.mm256_set1_epi32 boundary) gv)) in
        good_lower == (good &. mk_i32 15) /\ good_upper == (good >>! mk_i32 4)) /\
       nlo == v (Core_models.Num.impl_i32__count_ones good_lower) /\
       nup == v (Core_models.Num.impl_i32__count_ones good_upper))
    (ensures (
       let lc = I.mm_shuffle_epi8 (I.mm256_castsi256_si128 cv)
                  (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE (v good_lower) <: t_Slice u8)) in
       let uc = I.mm_shuffle_epi8 (I.mm256_extracti128_si256 (mk_i32 1) cv)
                  (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE (v good_upper) <: t_Slice u8)) in
       let o1 = Rust_primitives.Hax.Monomorphized_update_at.update_at_range output
                  ({ R.f_start = mk_usize 0; R.f_end = mk_usize 4 } <: R.t_Range usize)
                  (I.mm_storeu_si128_i32 (Seq.slice output 0 4) lc) in
       let o2 = Rust_primitives.Hax.Monomorphized_update_at.update_at_range o1
                  ({ R.f_start = mk_usize nlo; R.f_end = mk_usize (nlo+4) } <: R.t_Range usize)
                  (I.mm_storeu_si128_i32 (Seq.slice o1 nlo (nlo+4)) uc) in
       Seq.slice o2 0 (nlo+nup) == filt8 (cand8v cv) (acc8b gv boundary))) =
  let lo = filt4 (clv cv) (alb gv boundary) in
  let up = filt4 (cuv cv) (aub gv boundary) in
  lemma_good_lower_b gv boundary;
  lemma_good_upper_b gv boundary;
  lemma_filt4_length (clv cv) (alb gv boundary);
  lemma_filt4_length (cuv cv) (aub gv boundary);
  lemma_count_ones_nibble_exact good_lower;
  lemma_count_ones_nibble_exact good_upper;
  assert (v good_lower == mask4 (alb gv boundary));
  assert (v good_upper == mask4 (aub gv boundary));
  assert (Seq.length lo == nlo);
  assert (Seq.length up == nup);
  let lc = I.mm_shuffle_epi8 (I.mm256_castsi256_si128 cv)
             (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE (v good_lower) <: t_Slice u8)) in
  let uc = I.mm_shuffle_epi8 (I.mm256_extracti128_si256 (mk_i32 1) cv)
             (I.mm_loadu_si128 (Seq.index ST.v_SHUFFLE_TABLE (v good_upper) <: t_Slice u8)) in
  lemma_lower_forall_g cv (alb gv boundary) (v good_lower) lo nlo;
  lemma_upper_forall_g cv (aub gv boundary) (v good_upper) up nup;
  lemma_store_two_halves output lc uc lo up nlo nup;
  lemma_filt8_split (cand8v cv) (acc8b gv boundary)
                    (clv cv) (alb gv boundary) (cuv cv) (aub gv boundary)
#pop-options

(* ===== from ScratchRejEtaD (eta spec bridges) ===== *)

(* ===================================================================== *)
(* Layer D_eta: filt8 cand acc == rejection_sample_eta_{2,4} input.        *)
(* The eta spec is repeati (sz 4) inner empty, where each inner step       *)
(* appends try_0 THEN try_1 (up to 2 elements per byte). We relate this    *)
(* repeati-4-of-pairs to the repeati-8 filt8: byte m's two appends are     *)
(* exactly step8 at lanes 2m (try_0) and 2m+1 (try_1).                     *)
(* Lane 2m <-> byte m low nibble (try_0); lane 2m+1 <-> high nibble (try_1).*)
(* ===================================================================== *)

(* ---- eta_2 ---- *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_step8_eq_inner_eta2 (input: t_Slice u8{Seq.length input == 4})
  (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool) (m:nat{m<4}) (s:Seq.seq i32)
  : Lemma
    (requires (
      let byte = Seq.index input m in
      let try_0 = byte &. mk_u8 15 in
      let try_1 = byte >>! mk_u8 4 in
      acc (2*m)   == (try_0 <. mk_u8 15) /\
      acc (2*m+1) == (try_1 <. mk_u8 15) /\
      cand (2*m)   == (mk_i32 2 -. ((cast try_0 <: i32) %! mk_i32 5)) /\
      cand (2*m+1) == (mk_i32 2 -. ((cast try_1 <: i32) %! mk_i32 5))))
    (ensures M.rejection_sample_eta_2_inner input (sz m) s
             == step8 cand acc (sz (2*m+1)) (step8 cand acc (sz (2*m)) s)) =
  ()
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let lemma_filt8_eq_eta2 (input: t_Slice u8{Seq.length input == 4})
  (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool)
  : Lemma
    (requires (forall (m:nat{m<4}).
      (let byte = Seq.index input m in
       let try_0 = byte &. mk_u8 15 in
       let try_1 = byte >>! mk_u8 4 in
       acc (2*m) == (try_0 <. mk_u8 15) /\ acc (2*m+1) == (try_1 <. mk_u8 15) /\
       cand (2*m) == (mk_i32 2 -. ((cast try_0 <: i32) %! mk_i32 5)) /\
       cand (2*m+1) == (mk_i32 2 -. ((cast try_1 <: i32) %! mk_i32 5)))))
    (ensures filt8 cand acc == M.rejection_sample_eta_2 input) =
  let inner = M.rejection_sample_eta_2_inner input in
  SU.eq_repeati0 (sz 4) inner Seq.empty;
  SU.unfold_repeati (sz 4) inner Seq.empty (sz 0);
  SU.unfold_repeati (sz 4) inner Seq.empty (sz 1);
  SU.unfold_repeati (sz 4) inner Seq.empty (sz 2);
  SU.unfold_repeati (sz 4) inner Seq.empty (sz 3);
  lemma_filt8_unfold cand acc;
  let e = Seq.empty #i32 in
  let t0 = inner (sz 0) e in
  let t1 = inner (sz 1) t0 in
  let t2 = inner (sz 2) t1 in
  lemma_step8_eq_inner_eta2 input cand acc 0 e;
  lemma_step8_eq_inner_eta2 input cand acc 1 t0;
  lemma_step8_eq_inner_eta2 input cand acc 2 t1;
  lemma_step8_eq_inner_eta2 input cand acc 3 t2
#pop-options

(* ---- eta_4 ---- *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_step8_eq_inner_eta4 (input: t_Slice u8{Seq.length input == 4})
  (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool) (m:nat{m<4}) (s:Seq.seq i32)
  : Lemma
    (requires (
      let byte = Seq.index input m in
      let try_0 = byte &. mk_u8 15 in
      let try_1 = byte >>! mk_u8 4 in
      acc (2*m)   == (try_0 <. mk_u8 9) /\
      acc (2*m+1) == (try_1 <. mk_u8 9) /\
      cand (2*m)   == (mk_i32 4 -. (cast try_0 <: i32)) /\
      cand (2*m+1) == (mk_i32 4 -. (cast try_1 <: i32))))
    (ensures M.rejection_sample_eta_4_inner input (sz m) s
             == step8 cand acc (sz (2*m+1)) (step8 cand acc (sz (2*m)) s)) =
  ()
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let lemma_filt8_eq_eta4 (input: t_Slice u8{Seq.length input == 4})
  (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool)
  : Lemma
    (requires (forall (m:nat{m<4}).
      (let byte = Seq.index input m in
       let try_0 = byte &. mk_u8 15 in
       let try_1 = byte >>! mk_u8 4 in
       acc (2*m) == (try_0 <. mk_u8 9) /\ acc (2*m+1) == (try_1 <. mk_u8 9) /\
       cand (2*m) == (mk_i32 4 -. (cast try_0 <: i32)) /\
       cand (2*m+1) == (mk_i32 4 -. (cast try_1 <: i32)))))
    (ensures filt8 cand acc == M.rejection_sample_eta_4 input) =
  let inner = M.rejection_sample_eta_4_inner input in
  SU.eq_repeati0 (sz 4) inner Seq.empty;
  SU.unfold_repeati (sz 4) inner Seq.empty (sz 0);
  SU.unfold_repeati (sz 4) inner Seq.empty (sz 1);
  SU.unfold_repeati (sz 4) inner Seq.empty (sz 2);
  SU.unfold_repeati (sz 4) inner Seq.empty (sz 3);
  lemma_filt8_unfold cand acc;
  let e = Seq.empty #i32 in
  let t0 = inner (sz 0) e in
  let t1 = inner (sz 1) t0 in
  let t2 = inner (sz 2) t1 in
  lemma_step8_eq_inner_eta4 input cand acc 0 e;
  lemma_step8_eq_inner_eta4 input cand acc 1 t0;
  lemma_step8_eq_inner_eta4 input cand acc 2 t1;
  lemma_step8_eq_inner_eta4 input cand acc 3 t2
#pop-options

(* ===== generic per-element bound for filt8 (for the dispatcher's bounds-only post) ===== *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let step8_preserves_bound (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool) (lo hi: int)
      (i:usize{v i<8}) (s:Seq.seq i32)
  : Lemma (requires (forall (k:nat). k < Seq.length s ==>
                       (lo <= v (Seq.index s k) /\ v (Seq.index s k) <= hi)) /\
                    (acc (v i) ==> (lo <= v (cand (v i)) /\ v (cand (v i)) <= hi)))
          (ensures (forall (k:nat). k < Seq.length (step8 cand acc i s) ==>
                      (lo <= v (Seq.index (step8 cand acc i s) k) /\
                       v (Seq.index (step8 cand acc i s) k) <= hi))) =
  if acc (v i) then begin
    let seg = Seq.create 1 (cand (v i)) in
    let app = Seq.append s seg in
    let aux (k:nat{k < Seq.length app}) : Lemma (lo <= v (Seq.index app k) /\ v (Seq.index app k) <= hi) =
      if k < Seq.length s then Seq.lemma_index_app1 s seg k
      else (Seq.lemma_index_app2 s seg k; Seq.lemma_index_create 1 (cand (v i)) (k - Seq.length s))
    in Classical.forall_intro aux
  end else ()
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let lemma_filt8_bound (cand:(j:nat{j<8})->i32) (acc:(j:nat{j<8})->bool) (lo hi: int)
  : Lemma (requires (forall (j:nat{j<8}). acc j ==> (lo <= v (cand j) /\ v (cand j) <= hi)))
          (ensures (forall (k:nat). k < Seq.length (filt8 cand acc) ==>
                      (lo <= v (Seq.index (filt8 cand acc) k) /\
                       v (Seq.index (filt8 cand acc) k) <= hi))) =
  lemma_filt8_unfold cand acc;
  let e = Seq.empty #i32 in
  let t0 = step8 cand acc (sz 0) e in
  let t1 = step8 cand acc (sz 1) t0 in
  let t2 = step8 cand acc (sz 2) t1 in
  let t3 = step8 cand acc (sz 3) t2 in
  let t4 = step8 cand acc (sz 4) t3 in
  let t5 = step8 cand acc (sz 5) t4 in
  let t6 = step8 cand acc (sz 6) t5 in
  step8_preserves_bound cand acc lo hi (sz 0) e;
  step8_preserves_bound cand acc lo hi (sz 1) t0;
  step8_preserves_bound cand acc lo hi (sz 2) t1;
  step8_preserves_bound cand acc lo hi (sz 3) t2;
  step8_preserves_bound cand acc lo hi (sz 4) t3;
  step8_preserves_bound cand acc lo hi (sz 5) t4;
  step8_preserves_bound cand acc lo hi (sz 6) t5;
  step8_preserves_bound cand acc lo hi (sz 7) t6
#pop-options
"#)]
pub(crate) fn proof_helpers_module_marker() -> bool {
    true
}

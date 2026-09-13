module Libcrux_ml_kem.Vector.Avx2.Byteperm_theory
#set-options "--fuel 0 --ifuel 1 --z3rlimit 50"
open FStar.Mul
open Core_models
open Libcrux_intrinsics.Avx2
open Libcrux_intrinsics.Avx2_ml_kem_views

module Funarr = Libcrux_core_models.Abstractions.Funarr
module Canon  = Libcrux_core_models.Intrinsics_views
module IV     = Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec
module IVi    = Libcrux_core_models.Abstractions.Bitvec.Int_vec_interp

(* ============================================================================
   256-bit BYTE/DWORD PERMUTATION bit semantics — `mm256_shuffle_epi8` (VPSHUFB)
   and `mm256_permutevar8x32_epi32` (VPERMD), in `bv_bit` form.

   These are the two ops the AVX2 serialize widths use to gather the packed
   bytes after `mm256_concat_pairs_n`, and neither had a bit-level fact: the
   companion carries only the 128-bit `lemma_bv_bit_mm_shuffle_epi8`, and the
   canonical `Libcrux_core_models.Intrinsics_views` carries the 256-bit SELECT
   branch (`lemma_iv_shuffle_epi8_sel`) but not the 256-bit ZEROING branch.
   That last one is proven here (mirror of the canonical 128-bit
   `lemma_iv_mm_shuffle_epi8_neg`) — developed in the consumer per
   `feedback_develop_locally_upstream_once`, to be upstreamed to core-models
   once the width sweep has exercised it.

   Own module, not appended to `Libcrux_intrinsics.Avx2_ml_kem_views`, because a
   2000-line host costs these proofs an order of magnitude.

   SELECT INDEX AS A FREE PARAMETER.  Both lemmas take the resolved source
   index (`sel`) as a parameter constrained in the `requires`, rather than
   recomputing it in the `ensures`.  Callers pass ground masks
   (`mm256_set_epi8 …`/`mm256_set_epi32 …`), so `sel` is a literal at the call
   site and the index algebra never enters the consumer's context — the
   ground-literal discipline of skill §7. *)

(* Byte / dword of a 256-bit vector, in the canonical FunArray view. Plain
   (transparent) definitions: they must stay delta-equal to what the canonical
   op lemmas produce. *)
let vec256_byte (bv: t_Vec256) (k: nat{k < 32}) : i8 =
  Funarr.impl_5__get (mk_u64 32) #i8 (Canon.to_i8x32 bv) (mk_u64 k)

let vec256_dword (bv: t_Vec256) (j: nat{j < 8}) : i32 =
  Funarr.impl_5__get (mk_u64 8) #i32 (Canon.to_i32x8 bv) (mk_u64 j)

(* ── VPSHUFB, zeroing branch, at the Int_vec layer ────────────────────────────
   The 256-bit twin of the canonical `lemma_iv_mm_shuffle_epi8_neg`: a negative
   index byte has its high bit set once wrapped to u8, so the op takes the
   zeroing branch. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_iv256_shuffle_epi8_neg (a b: Funarr.t_FunArray (mk_u64 32) i8) (i: nat{i < 32})
  : Lemma (requires v (Funarr.impl_5__get (mk_u64 32) #i8 b (mk_u64 i)) < 0)
          (ensures Funarr.impl_5__get (mk_u64 32) #i8 (IV.e_mm256_shuffle_epi8 a b) (mk_u64 i) ==
                   mk_i8 0) =
  let bi = Funarr.impl_5__get (mk_u64 32) #i8 b (mk_u64 i) in
  let idx: u8 = cast bi <: u8 in
  assert (v idx >= 128);
  Canon.lemma_u8_high_bit_set idx;
  assert (Funarr.impl_5__get (mk_u64 32) #i8 (IV.e_mm256_shuffle_epi8 a b) (mk_u64 i) == mk_i8 0)
    by (FStar.Tactics.norm [delta_only [`%Libcrux_core_models.Core_arch.X86.Interpretations.Int_vec.e_mm256_shuffle_epi8];
                            iota; zeta; primops];
        FStar.Tactics.smt ())
#pop-options

(* ── VPSHUFB, select branch, in bv_bit form ───────────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_mm256_shuffle_epi8_sel (a b: t_Vec256) (i: nat{i < 256}) (sel: nat{sel < 32})
  : Lemma (requires v (vec256_byte b (i / 8)) >= 0 /\
                    sel == 16 * ((i / 8) / 16) + (v (vec256_byte b (i / 8))) % 16)
          (ensures bv_bit (mm256_shuffle_epi8 a b) i == bv_bit a (8 * sel + i % 8)) =
  reveal_opaque (`%mm256_shuffle_epi8) mm256_shuffle_epi8;
  Canon.lemma_mm256_shuffle_epi8 a b;
  let nth = i / 8 in
  let sb = i % 8 in
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  let r = mm256_shuffle_epi8 a b in
  Canon.lemma_iv_shuffle_epi8_sel (Canon.to_i8x32 a) (Canon.to_i8x32 b) nth;
  Canon.lemma_readback Rust_primitives.Integers.I8 (mk_u64 256) (mk_u64 32) r (mk_u64 nth) sb;
  lemma_bv_bit_reader 8 r nth sb;
  Canon.lemma_readback Rust_primitives.Integers.I8 (mk_u64 256) (mk_u64 32) a (mk_u64 sel) sb;
  lemma_bv_bit_reader 8 a sel sb
#pop-options

(* ── VPSHUFB, zeroing branch, in bv_bit form ──────────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_mm256_shuffle_epi8_neg (a b: t_Vec256) (i: nat{i < 256})
  : Lemma (requires v (vec256_byte b (i / 8)) < 0)
          (ensures bv_bit (mm256_shuffle_epi8 a b) i == 0) =
  reveal_opaque (`%mm256_shuffle_epi8) mm256_shuffle_epi8;
  Canon.lemma_mm256_shuffle_epi8 a b;
  let nth = i / 8 in
  let sb = i % 8 in
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  let r = mm256_shuffle_epi8 a b in
  lemma_iv256_shuffle_epi8_neg (Canon.to_i8x32 a) (Canon.to_i8x32 b) nth;
  Canon.lemma_readback Rust_primitives.Integers.I8 (mk_u64 256) (mk_u64 32) r (mk_u64 nth) sb;
  lemma_bv_bit_reader 8 r nth sb;
  reveal_opaque (`%Rust_primitives.Integers.get_bit)
                (Rust_primitives.Integers.get_bit #Rust_primitives.Integers.I8)
#pop-options

(* ── VPERMD, in bv_bit form ───────────────────────────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_mm256_permutevar8x32 (a b: t_Vec256) (i: nat{i < 256}) (sel: nat{sel < 8})
  : Lemma (requires v (vec256_dword b (i / 32)) >= 0 /\
                    sel == (v (vec256_dword b (i / 32))) % 8)
          (ensures bv_bit (mm256_permutevar8x32_epi32 a b) i == bv_bit a (32 * sel + i % 32)) =
  reveal_opaque (`%mm256_permutevar8x32_epi32) mm256_permutevar8x32_epi32;
  Canon.lemma_mm256_permutevar8x32_epi32 a b;
  let j = i / 32 in
  let sb = i % 32 in
  FStar.Math.Lemmas.euclidean_division_definition i 32;
  let r = mm256_permutevar8x32_epi32 a b in
  Canon.lemma_iv_permutevar8x32 (Canon.to_i32x8 a) (Canon.to_i32x8 b) j;
  assert ((cast (vec256_dword b j) <: u64) %! mk_u64 8 == mk_u64 sel);
  Canon.lemma_readback Rust_primitives.Integers.I32 (mk_u64 256) (mk_u64 8) r (mk_u64 j) sb;
  lemma_bv_bit_reader 32 r j sb;
  Canon.lemma_readback Rust_primitives.Integers.I32 (mk_u64 256) (mk_u64 8) a (mk_u64 sel) sb;
  lemma_bv_bit_reader 32 a sel sb
#pop-options

(* ============================================================================
   THE serialize_4 GATHER CHAIN

   After `mm256_concat_pairs_n 4`, serialize_4 runs
     shuffle_epi8 <mask>  ->  permutevar8x32 <ctrl>  ->  castsi256_si128
   to collect the 4 low bytes of each 128-bit half into the low 64 bits.
   Stated in two stages, per the session-7 item-1 lesson: stage 1 is the pure
   BIT-VIEW step (ground-dispatched over the 8 live mask bytes), stage 2 is the
   pure INDEX ALGEBRA.  Neither ever runs in the other's context.
   ========================================================================== *)

unfold let ser4_mask =
  mm256_set_epi8 (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 12) (mk_i8 8) (mk_i8 4) (mk_i8 0)
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 12) (mk_i8 8) (mk_i8 4) (mk_i8 0)

unfold let ser4_ctrl =
  mm256_set_epi32 (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 0)
    (mk_i32 4) (mk_i32 0)

(* the 8 LIVE mask bytes (the other 24 are -1 = zeroing, and unreachable here) *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_ser4_mask_bytes (nth: nat{nth < 32})
  : Lemma (requires nth < 4 \/ (nth >= 16 /\ nth < 20))
          (ensures v (vec256_byte ser4_mask nth) == 4 * (nth % 16)) =
  reveal_opaque (`%mm256_set_epi8) mm256_set_epi8;
  Canon.lemma_mm256_set_epi8 (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 12) (mk_i8 8) (mk_i8 4) (mk_i8 0)
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 12) (mk_i8 8) (mk_i8 4) (mk_i8 0);
  Canon.lemma_iv_set_epi8 (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 12) (mk_i8 8) (mk_i8 4) (mk_i8 0)
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 12) (mk_i8 8) (mk_i8 4) (mk_i8 0) nth
#pop-options

(* the 2 LIVE control dwords *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_ser4_ctrl_dwords (j: nat{j < 2})
  : Lemma (ensures v (vec256_dword ser4_ctrl j) == 4 * j) =
  reveal_opaque (`%mm256_set_epi32) mm256_set_epi32;
  Canon.lemma_mm256_set_epi32 (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 0)
    (mk_i32 4) (mk_i32 0);
  Canon.lemma_iv_set_epi32 (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 0)
    (mk_i32 4) (mk_i32 0) j
#pop-options

(* stage 1a — the VPSHUFB step at the 8 live byte positions, ground-dispatched
   on `nth` so the mask byte is a literal in every arm. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_ser4_shuffle_bit (y: t_Vec256) (k: nat{k < 256})
  : Lemma (requires k < 32 \/ (k >= 128 /\ k < 160))
          (ensures bv_bit (mm256_shuffle_epi8 y ser4_mask) k ==
                   bv_bit y (128 * (k / 128) + 32 * ((k / 8) % 16) + k % 8)) =
  let nth = k / 8 in
  let sb = k % 8 in
  FStar.Math.Lemmas.euclidean_division_definition k 8;
  lemma_ser4_mask_bytes nth;
  let sel: nat = 16 * (nth / 16) + (v (vec256_byte ser4_mask nth)) % 16 in
  assert (v (vec256_byte ser4_mask nth) == 4 * (nth % 16));
  assert (nth % 16 < 4);
  FStar.Math.Lemmas.small_mod (4 * (nth % 16)) 16;
  assert (sel == 16 * (nth / 16) + 4 * (nth % 16));
  assert (sel < 32);
  lemma_bv_bit_mm256_shuffle_epi8_sel y ser4_mask k sel;
  assert (8 * sel + sb == 128 * (nth / 16) + 32 * (nth % 16) + sb);
  assert (nth / 16 == k / 128)
#pop-options

(* stage 1b — the VPERMD + cast steps, and the full gather at bit `i < 64`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_ser4_gather_bit (y: t_Vec256) (i: nat{i < 64})
  : Lemma (ensures
             bv_bit (mm256_castsi256_si128
                       (mm256_permutevar8x32_epi32 (mm256_shuffle_epi8 y ser4_mask) ser4_ctrl)) i ==
             bv_bit y (128 * (i / 32) + 32 * ((i / 8) % 4) + i % 8)) =
  let s = mm256_shuffle_epi8 y ser4_mask in
  let p = mm256_permutevar8x32_epi32 s ser4_ctrl in
  lemma_bv_bit_castsi256_si128 p i;
  let j = i / 32 in
  lemma_ser4_ctrl_dwords j;
  FStar.Math.Lemmas.small_mod (4 * j) 8;
  lemma_bv_bit_mm256_permutevar8x32 s ser4_ctrl i (4 * j);
  let k = 32 * (4 * j) + i % 32 in
  assert (k == 128 * j + i % 32);
  FStar.Math.Lemmas.euclidean_division_definition i 32;
  assert (k < 32 \/ (k >= 128 /\ k < 160));
  lemma_ser4_shuffle_bit y k;
  assert (k / 128 == j);
  assert (k % 8 == i % 8);
  assert ((k / 8) % 16 == (i / 8) % 4)
#pop-options

(* stage 2 — the index algebra: composing the `concat_pairs_n 4` post at the
   gathered index yields exactly `(i/4)*16 + i%4`.  Pure integers, no vectors. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 300 --split_queries always"
let lemma_ser4_index (i: nat{i < 64})
  : Lemma (ensures
             (let g = 128 * (i / 32) + 32 * ((i / 8) % 4) + i % 8 in
              g < 256 /\ g % 32 == i % 8 /\ g / 32 == 4 * (i / 32) + (i / 8) % 4 /\
              (i % 8 < 4 ==> (g / 32) * 32 + g % 32 == (i / 4) * 16 + i % 4) /\
              (i % 8 >= 4 ==> (g / 32) * 32 + 16 + (g % 32 - 4) == (i / 4) * 16 + i % 4))) =
  FStar.Math.Lemmas.euclidean_division_definition i 32;
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  FStar.Math.Lemmas.euclidean_division_definition i 4;
  assert (i / 8 == 4 * (i / 32) + (i / 8) % 4);
  assert (i == 32 * (i / 32) + 8 * ((i / 8) % 4) + i % 8)
#pop-options

(* THE serialize_4 obligation, per output bit.  `y` is the `concat_pairs_n 4`
   result, threaded as a FREE parameter with its bit post as a hypothesis, so
   the caller links by congruence and this module never mentions Serialize. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_serialize_4_gather_bits (x y: t_Vec256) (i: nat{i < 64})
  : Lemma
      (requires (forall (k: nat{k < 256}).
                   bv_bit y k ==
                   (if k % 32 < 4 then bv_bit x ((k / 32) * 32 + k % 32)
                    else if k % 32 < 8 then bv_bit x ((k / 32) * 32 + 16 + (k % 32 - 4))
                    else 0)))
      (ensures bv_bit (mm256_castsi256_si128
                         (mm256_permutevar8x32_epi32 (mm256_shuffle_epi8 y ser4_mask) ser4_ctrl)) i ==
               bv_bit x ((i / 4) * 16 + i % 4)) =
  lemma_ser4_gather_bit y i;
  lemma_ser4_index i
#pop-options

module RI = Rust_primitives.Integers

(* ============================================================================
   VARIABLE 32-BIT LEFT SHIFT / IMMEDIATE 64-BIT RIGHT SHIFT, in `bv_bit` form.

   `mm256_sllv_epi32` + `mm256_srli_epi64` are the pair every "pack adjacent
   N-combined into adjacent 2N-combined" step of serialize_5/10/12 runs: the
   sllv pushes the low dword of a 64-bit lane up so that the srli64 slides the
   high dword down onto it.  Neither had a bit-level fact.

   Split per the session-7 keystone lesson: the PURE u32/u64 shift arithmetic
   (`lemma_shl_bit32` / `lemma_shr_bit64`) is proven with no vector terms in
   scope; the lane-view lemmas below do nothing but a readback and one call.
   ========================================================================== *)

(* qword accessor — the 64-bit twin of `vec256_dword`. *)
let vec256_qword (bv: t_Vec256) (q: nat{q < 4}) : i64 =
  Funarr.impl_5__get (mk_u64 4) #i64 (Canon.to_i64x4 bv) (mk_u64 q)

(* ── pure bit arithmetic: u32 left shift ──────────────────────────────────── *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 400 --split_queries always"
let lemma_shl_bit32 (x: i32) (s: i32{v s >= 0 /\ v s < 32}) (t: nat{t < 32})
  : Lemma (RI.get_bit (cast ((cast x <: u32) <<! s <: u32) <: i32) (sz t) ==
           (if t < v s then 0 else RI.get_bit x (sz (t - v s)))) =
  assert_norm (pow2 32 == 4294967296);
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.I32);
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.U32);
  let n32 = pow2 32 in
  let xu: u32 = cast x <: u32 in
  let sh: u32 = xu <<! s in
  let r: i32 = cast sh <: i32 in
  assert (v xu == (v x) % n32);
  assert (v sh == ((v xu) * pow2 (v s)) % n32);
  assert ((v r) % n32 == v sh);
  let a = (v xu) * pow2 (v s) in
  (* ((a % 2^32) / 2^t) % 2 == (a / 2^t) % 2 *)
  FStar.Math.Lemmas.pow2_modulo_division_lemma_1 a t 32;
  FStar.Math.Lemmas.pow2_plus 1 (32 - t - 1);
  FStar.Math.Lemmas.modulo_modulo_lemma (a / pow2 t) 2 (pow2 (32 - t - 1));
  if t < v s then begin
    (* a / 2^t == xu * 2^(s-t), which is even *)
    FStar.Math.Lemmas.pow2_plus t (v s - t);
    FStar.Math.Lemmas.cancel_mul_div ((v xu) * pow2 (v s - t)) (pow2 t);
    FStar.Math.Lemmas.pow2_plus 1 (v s - t - 1);
    FStar.Math.Lemmas.multiple_modulo_lemma ((v xu) * pow2 (v s - t - 1)) 2
  end
  else begin
    (* a / 2^t == xu / 2^(t-s) *)
    FStar.Math.Lemmas.pow2_plus (v s) (t - v s);
    FStar.Math.Lemmas.division_multiplication_lemma a (pow2 (v s)) (pow2 (t - v s));
    FStar.Math.Lemmas.cancel_mul_div (v xu) (pow2 (v s))
  end
#pop-options

(* ── pure bit arithmetic: u64 right shift ─────────────────────────────────── *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 400 --split_queries always"
let lemma_shr_bit64 (x: i64) (s: i32{v s >= 0 /\ v s < 64}) (t: nat{t < 64})
  : Lemma (RI.get_bit (cast ((cast x <: u64) >>! s <: u64) <: i64) (sz t) ==
           (if t + v s < 64 then RI.get_bit x (sz (t + v s)) else 0)) =
  assert_norm (pow2 64 == 18446744073709551616);
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.I64);
  reveal_opaque (`%RI.get_bit) (RI.get_bit #RI.U64);
  let n64 = pow2 64 in
  let xu: u64 = cast x <: u64 in
  let sh: u64 = xu >>! s in
  let r: i64 = cast sh <: i64 in
  assert (v xu == (v x) % n64);
  assert (v sh == (v xu) / pow2 (v s));
  FStar.Math.Lemmas.lemma_div_lt_nat (v xu) 64 (v s);
  assert ((v r) % n64 == v sh);
  (* (xu / 2^s) / 2^t == xu / 2^(s+t) *)
  FStar.Math.Lemmas.pow2_plus (v s) t;
  FStar.Math.Lemmas.division_multiplication_lemma (v xu) (pow2 (v s)) (pow2 t);
  if t + v s < 64 then ()
  else begin
    FStar.Math.Lemmas.pow2_le_compat (t + v s) 64;
    FStar.Math.Lemmas.small_division_lemma_1 (v xu) (pow2 (t + v s))
  end
#pop-options

(* ── VPSLLVD, in bv_bit form ──────────────────────────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_mm256_sllv_epi32 (a b: t_Vec256) (i: nat{i < 256}) (s: nat{s < 32})
  : Lemma (requires v (vec256_dword b (i / 32)) == s)
          (ensures bv_bit (mm256_sllv_epi32 a b) i ==
                   (if i % 32 < s then 0 else bv_bit a (32 * (i / 32) + (i % 32 - s)))) =
  reveal_opaque (`%mm256_sllv_epi32) mm256_sllv_epi32;
  Canon.lemma_mm256_sllv_epi32 a b;
  let j = i / 32 in
  let t = i % 32 in
  FStar.Math.Lemmas.euclidean_division_definition i 32;
  let r = mm256_sllv_epi32 a b in
  Canon.lemma_iv_sllv_epi32 (Canon.to_i32x8 a) (Canon.to_i32x8 b) j;
  lemma_shl_bit32 (vec256_dword a j) (vec256_dword b j) t;
  Canon.lemma_readback Rust_primitives.Integers.I32 (mk_u64 256) (mk_u64 8) r (mk_u64 j) t;
  lemma_bv_bit_reader 32 r j t;
  if t >= s then begin
    Canon.lemma_readback Rust_primitives.Integers.I32 (mk_u64 256) (mk_u64 8) a (mk_u64 j) (t - s);
    lemma_bv_bit_reader 32 a j (t - s)
  end
#pop-options

(* ── VPSRLQ (immediate), in bv_bit form ───────────────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_bv_bit_mm256_srli_epi64 (imm: i32{v imm > 0 /\ v imm < 64}) (a: t_Vec256)
      (i: nat{i < 256})
  : Lemma (ensures bv_bit (mm256_srli_epi64 imm a) i ==
                   (if i % 64 + v imm < 64
                    then bv_bit a (64 * (i / 64) + (i % 64 + v imm))
                    else 0)) =
  reveal_opaque (`%mm256_srli_epi64) mm256_srli_epi64;
  Canon.lemma_mm256_srli_epi64 imm a;
  let q = i / 64 in
  let t = i % 64 in
  FStar.Math.Lemmas.euclidean_division_definition i 64;
  let r = mm256_srli_epi64 imm a in
  Canon.lemma_iv_srli64 imm (Canon.to_i64x4 a) q;
  lemma_shr_bit64 (vec256_qword a q) imm t;
  Canon.lemma_readback Rust_primitives.Integers.I64 (mk_u64 256) (mk_u64 4) r (mk_u64 q) t;
  lemma_bv_bit_reader 64 r q t;
  if t + v imm < 64 then begin
    Canon.lemma_readback Rust_primitives.Integers.I64 (mk_u64 256) (mk_u64 4) a (mk_u64 q)
      (t + v imm);
    lemma_bv_bit_reader 64 a q (t + v imm)
  end
#pop-options

(* ============================================================================
   THE serialize_5 GATHER CHAIN

   `serialize_5_vec` runs, on the `concat_pairs_n 5` result `y` (10 live bits at
   the bottom of every 32-bit dword):

     sllv <22 on even dwords>  -> srli64 22     (2-combined -> 4-combined: 20
                                                 live bits at the bottom of
                                                 dwords 0 and 2 of each half)
     shuffle_epi8 <mask>                        (dword 2 of each half -> dword 1)
     sllv <12 on dwords = 0 mod 4> -> srli64 12 (4-combined -> 8-combined: 40
                                                 live bits at the bottom of each
                                                 128-bit half)
     castsi256_si128 / extracti128_si256 1

   Every step is stated as a pure INDEX SHIFT on `bv_bit` — the cheapest
   possible conclusion shape, with no arithmetic under the `bv_bit`.  The
   four-way ground dispatch lives only in `lemma_ser5_gather_bit`, and the
   `(u/5)*16 + u%5` algebra is a separate pure-integer lemma with zero vector
   terms.  Three contexts, per the session-7 keystone lesson.
   ========================================================================== *)

unfold let ser5_sh1 =
  mm256_set_epi32 (mk_i32 0) (mk_i32 22) (mk_i32 0) (mk_i32 22)
                  (mk_i32 0) (mk_i32 22) (mk_i32 0) (mk_i32 22)

unfold let ser5_sh2 =
  mm256_set_epi32 (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 12)
                  (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 12)

unfold let ser5_mask =
  mm256_set_epi8 (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 8)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 8)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)

(* the whole register chain, exactly as serialize_5_vec composes it *)
unfold let ser5_chain (y: t_Vec256) : t_Vec256 =
  mm256_srli_epi64 (mk_i32 12)
    (mm256_sllv_epi32
       (mm256_shuffle_epi8
          (mm256_srli_epi64 (mk_i32 22) (mm256_sllv_epi32 y ser5_sh1))
          ser5_mask)
       ser5_sh2)

(* ── the ground shift-count dwords ────────────────────────────────────────── *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_ser5_sh1_dwords (j: nat{j < 8})
  : Lemma (ensures v (vec256_dword ser5_sh1 j) == (if j % 2 = 0 then 22 else 0)) =
  reveal_opaque (`%mm256_set_epi32) mm256_set_epi32;
  Canon.lemma_mm256_set_epi32 (mk_i32 0) (mk_i32 22) (mk_i32 0) (mk_i32 22)
    (mk_i32 0) (mk_i32 22) (mk_i32 0) (mk_i32 22);
  Canon.lemma_iv_set_epi32 (mk_i32 0) (mk_i32 22) (mk_i32 0) (mk_i32 22)
    (mk_i32 0) (mk_i32 22) (mk_i32 0) (mk_i32 22) j
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_ser5_sh2_dwords (j: nat{j < 8})
  : Lemma (ensures v (vec256_dword ser5_sh2 j) == (if j % 4 = 0 then 12 else 0)) =
  reveal_opaque (`%mm256_set_epi32) mm256_set_epi32;
  Canon.lemma_mm256_set_epi32 (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 12)
    (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 12);
  Canon.lemma_iv_set_epi32 (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 12)
    (mk_i32 0) (mk_i32 0) (mk_i32 0) (mk_i32 12) j
#pop-options

(* the 14 LIVE mask bytes (7 per 128-bit half): bytes 0..3 of a half select
   dword 0 of that half, bytes 4..6 select the bottom 3 bytes of dword 2. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_ser5_mask_bytes (nth: nat{nth < 32})
  : Lemma (requires nth % 16 < 8)
          (ensures v (vec256_byte ser5_mask nth) ==
                   (if nth % 16 < 4 then nth % 16 else nth % 16 + 4)) =
  reveal_opaque (`%mm256_set_epi8) mm256_set_epi8;
  Canon.lemma_mm256_set_epi8 (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 8)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 8)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0);
  Canon.lemma_iv_set_epi8 (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 8)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 8)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0) nth
#pop-options

(* ── step 1: the 22-shift pair, as an index shift ─────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_ser5_shift1_bit (y: t_Vec256) (i: nat{i < 256})
  : Lemma (requires i % 64 < 20)
          (ensures bv_bit (mm256_srli_epi64 (mk_i32 22) (mm256_sllv_epi32 y ser5_sh1)) i ==
                   bv_bit y (if i % 64 < 10 then i else i + 22)) =
  let q = i / 64 in
  let u = i % 64 in
  FStar.Math.Lemmas.euclidean_division_definition i 64;
  lemma_bv_bit_mm256_srli_epi64 (mk_i32 22) (mm256_sllv_epi32 y ser5_sh1) i;
  if u < 10 then begin
    (* index 64q + u + 22 sits in dword 2q (shift 22), at bit u + 22 *)
    FStar.Math.Lemmas.small_division_lemma_1 (u + 22) 32;
    FStar.Math.Lemmas.lemma_div_plus (u + 22) (2 * q) 32;
    FStar.Math.Lemmas.lemma_mod_plus (u + 22) (2 * q) 32;
    FStar.Math.Lemmas.small_mod (u + 22) 32;
    lemma_ser5_sh1_dwords (2 * q);
    lemma_bv_bit_mm256_sllv_epi32 y ser5_sh1 (64 * q + u + 22) 22
  end
  else begin
    (* index 64q + u + 22 sits in dword 2q+1 (shift 0), at bit u - 10 *)
    FStar.Math.Lemmas.small_division_lemma_1 (u - 10) 32;
    FStar.Math.Lemmas.lemma_div_plus (u - 10) (2 * q + 1) 32;
    FStar.Math.Lemmas.lemma_mod_plus (u - 10) (2 * q + 1) 32;
    FStar.Math.Lemmas.small_mod (u - 10) 32;
    lemma_ser5_sh1_dwords (2 * q + 1);
    lemma_bv_bit_mm256_sllv_epi32 y ser5_sh1 (64 * q + u + 22) 0
  end
#pop-options

(* ── step 2: VPSHUFB, as an index shift ───────────────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_ser5_shuffle_bit (b: t_Vec256) (k: nat{k < 256})
  : Lemma (requires k % 128 < 56)
          (ensures bv_bit (mm256_shuffle_epi8 b ser5_mask) k ==
                   bv_bit b (if k % 128 < 32 then k else k + 32)) =
  let h = k / 128 in
  let u = k % 128 in
  let nth = k / 8 in
  let sb = k % 8 in
  FStar.Math.Lemmas.euclidean_division_definition k 128;
  FStar.Math.Lemmas.euclidean_division_definition k 8;
  FStar.Math.Lemmas.euclidean_division_definition u 8;
  (* nth == 16h + u/8, and u/8 < 7, so nth % 16 == u/8 and nth / 16 == h *)
  FStar.Math.Lemmas.small_division_lemma_1 (u / 8) 16;
  FStar.Math.Lemmas.lemma_div_plus (u / 8) h 16;
  FStar.Math.Lemmas.lemma_mod_plus (u / 8) h 16;
  FStar.Math.Lemmas.small_mod (u / 8) 16;
  lemma_ser5_mask_bytes nth;
  let m = v (vec256_byte ser5_mask nth) in
  FStar.Math.Lemmas.small_mod m 16;
  lemma_bv_bit_mm256_shuffle_epi8_sel b ser5_mask k (16 * (nth / 16) + m % 16);
  assert (8 * (16 * (nth / 16) + m % 16) + sb == 128 * h + 8 * m + sb)
#pop-options

(* ── step 3: the 12-shift pair, as an index shift ─────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_ser5_shift2_bit (c: t_Vec256) (i: nat{i < 256})
  : Lemma (requires i % 128 < 40)
          (ensures bv_bit (mm256_srli_epi64 (mk_i32 12) (mm256_sllv_epi32 c ser5_sh2)) i ==
                   bv_bit c (if i % 128 < 20 then i else i + 12)) =
  let h = i / 128 in
  let u = i % 128 in
  FStar.Math.Lemmas.euclidean_division_definition i 128;
  (* u < 40 < 64, so i / 64 == 2h and i % 64 == u *)
  FStar.Math.Lemmas.small_division_lemma_1 u 64;
  FStar.Math.Lemmas.lemma_div_plus u (2 * h) 64;
  FStar.Math.Lemmas.lemma_mod_plus u (2 * h) 64;
  FStar.Math.Lemmas.small_mod u 64;
  lemma_bv_bit_mm256_srli_epi64 (mk_i32 12) (mm256_sllv_epi32 c ser5_sh2) i;
  if u < 20 then begin
    (* index 128h + u + 12 sits in dword 4h (shift 12), at bit u + 12 *)
    FStar.Math.Lemmas.small_division_lemma_1 (u + 12) 32;
    FStar.Math.Lemmas.lemma_div_plus (u + 12) (4 * h) 32;
    FStar.Math.Lemmas.lemma_mod_plus (u + 12) (4 * h) 32;
    FStar.Math.Lemmas.small_mod (u + 12) 32;
    lemma_ser5_sh2_dwords (4 * h);
    lemma_bv_bit_mm256_sllv_epi32 c ser5_sh2 (128 * h + u + 12) 12
  end
  else begin
    (* index 128h + u + 12 sits in dword 4h+1 (shift 0), at bit u - 20 *)
    FStar.Math.Lemmas.small_division_lemma_1 (u - 20) 32;
    FStar.Math.Lemmas.lemma_div_plus (u - 20) (4 * h + 1) 32;
    FStar.Math.Lemmas.lemma_mod_plus (u - 20) (4 * h + 1) 32;
    FStar.Math.Lemmas.small_mod (u - 20) 32;
    lemma_ser5_sh2_dwords (4 * h + 1);
    lemma_bv_bit_mm256_sllv_epi32 c ser5_sh2 (128 * h + u + 12) 0
  end
#pop-options

(* ── the index algebra: pure integers, zero vector terms ──────────────────── *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 300 --split_queries always"
let lemma_ser5_index (u: nat{u < 40})
  : Lemma (ensures
            (u % 20) % 10 == u % 10 /\
            64 * (u / 20) + 32 * ((u % 20) / 10) == 32 * (u / 10) /\
            (u % 10 < 5 ==> (u / 5) * 16 + u % 5 == 32 * (u / 10) + u % 10) /\
            (u % 10 >= 5 ==> (u / 5) * 16 + u % 5 == 32 * (u / 10) + 16 + (u % 10 - 5))) =
  (* u = (u%20) + 20*(u/20), so u/10 = 2*(u/20) + (u%20)/10 and u%10 = (u%20)%10 *)
  FStar.Math.Lemmas.euclidean_division_definition u 20;
  FStar.Math.Lemmas.lemma_div_plus (u % 20) (2 * (u / 20)) 10;
  FStar.Math.Lemmas.lemma_mod_plus (u % 20) (2 * (u / 20)) 10;
  (* u = (u%10) + 10*(u/10), so u/5 = 2*(u/10) + (u%10)/5 and u%5 = (u%10)%5 *)
  FStar.Math.Lemmas.euclidean_division_definition u 10;
  FStar.Math.Lemmas.lemma_div_plus (u % 10) (2 * (u / 10)) 5;
  FStar.Math.Lemmas.lemma_mod_plus (u % 10) (2 * (u / 10)) 5;
  if u % 10 < 5 then begin
    FStar.Math.Lemmas.small_division_lemma_1 (u % 10) 5;
    FStar.Math.Lemmas.small_mod (u % 10) 5
  end
  else begin
    FStar.Math.Lemmas.lemma_div_plus (u % 10 - 5) 1 5;
    FStar.Math.Lemmas.lemma_mod_plus (u % 10 - 5) 1 5;
    FStar.Math.Lemmas.small_division_lemma_1 (u % 10 - 5) 5;
    FStar.Math.Lemmas.small_mod (u % 10 - 5) 5
  end
#pop-options

(* ── the gather, ground-dispatched over the four 10-bit windows ───────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_ser5_gather_bit (y: t_Vec256) (i: nat{i < 256})
  : Lemma (requires i % 128 < 40)
          (ensures bv_bit (ser5_chain y) i ==
                   bv_bit y (128 * (i / 128) + 64 * ((i % 128) / 20) +
                             32 * (((i % 128) % 20) / 10) + (i % 128) % 10)) =
  let h = i / 128 in
  let u = i % 128 in
  FStar.Math.Lemmas.euclidean_division_definition i 128;
  let b = mm256_srli_epi64 (mk_i32 22) (mm256_sllv_epi32 y ser5_sh1) in
  let c = mm256_shuffle_epi8 b ser5_mask in
  lemma_ser5_shift2_bit c i;
  if u < 20 then begin
    (* shift2 stays at i; i % 128 == u < 32 so the shuffle stays at i too *)
    lemma_ser5_shuffle_bit b i;
    (* i % 64 == u < 20 *)
    FStar.Math.Lemmas.small_division_lemma_1 u 64;
    FStar.Math.Lemmas.lemma_div_plus u (2 * h) 64;
    FStar.Math.Lemmas.lemma_mod_plus u (2 * h) 64;
    FStar.Math.Lemmas.small_mod u 64;
    lemma_ser5_shift1_bit y i
  end
  else begin
    (* shift2 moves to i+12, whose (i+12) % 128 == u+12 lies in [32, 52) *)
    FStar.Math.Lemmas.lemma_div_plus (u + 12) h 128;
    FStar.Math.Lemmas.lemma_mod_plus (u + 12) h 128;
    FStar.Math.Lemmas.small_division_lemma_1 (u + 12) 128;
    FStar.Math.Lemmas.small_mod (u + 12) 128;
    lemma_ser5_shuffle_bit b (i + 12);
    (* the shuffle moves to i+44 == 64*(2h+1) + (u-20), so (i+44) % 64 == u-20 *)
    FStar.Math.Lemmas.small_division_lemma_1 (u - 20) 64;
    FStar.Math.Lemmas.lemma_div_plus (u - 20) (2 * h + 1) 64;
    FStar.Math.Lemmas.lemma_mod_plus (u - 20) (2 * h + 1) 64;
    FStar.Math.Lemmas.small_mod (u - 20) 64;
    lemma_ser5_shift1_bit y (i + 44)
  end
#pop-options

(* THE serialize_5 obligation, per output bit of the 256-bit chain result.  `y`
   is the `concat_pairs_n 5` result, threaded as a FREE parameter carrying its
   bit post as a hypothesis, so this module never mentions Serialize. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_serialize_5_gather_bits (x y: t_Vec256) (i: nat{i < 256})
  : Lemma
      (requires
        i % 128 < 40 /\
        (forall (k: nat{k < 256}).
           bv_bit y k ==
           (if k % 32 < 5 then bv_bit x ((k / 32) * 32 + k % 32)
            else if k % 32 < 10 then bv_bit x ((k / 32) * 32 + 16 + (k % 32 - 5))
            else 0)))
      (ensures bv_bit (ser5_chain y) i ==
               bv_bit x (128 * (i / 128) + ((i % 128) / 5) * 16 + (i % 128) % 5)) =
  let h = i / 128 in
  let u = i % 128 in
  lemma_ser5_gather_bit y i;
  lemma_ser5_index u;
  (* the y-index is 32 * (4h + u/10) + u%10, so its %32 is u%10 and /32 is 4h+u/10 *)
  FStar.Math.Lemmas.small_division_lemma_1 (u % 10) 32;
  FStar.Math.Lemmas.lemma_div_plus (u % 10) (4 * h + u / 10) 32;
  FStar.Math.Lemmas.lemma_mod_plus (u % 10) (4 * h + u / 10) 32;
  FStar.Math.Lemmas.small_mod (u % 10) 32
#pop-options

(* the two consumer-facing forms: the low half via `castsi256_si128`, the high
   half via `extracti128_si256 1`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_serialize_5_lower_bits (x y: t_Vec256) (i: nat{i < 40})
  : Lemma
      (requires (forall (k: nat{k < 256}).
                   bv_bit y k ==
                   (if k % 32 < 5 then bv_bit x ((k / 32) * 32 + k % 32)
                    else if k % 32 < 10 then bv_bit x ((k / 32) * 32 + 16 + (k % 32 - 5))
                    else 0)))
      (ensures bv_bit (mm256_castsi256_si128 (ser5_chain y)) i ==
               bv_bit x ((i / 5) * 16 + i % 5)) =
  FStar.Math.Lemmas.small_division_lemma_1 i 128;
  FStar.Math.Lemmas.small_mod i 128;
  lemma_bv_bit_castsi256_si128 (ser5_chain y) i;
  lemma_serialize_5_gather_bits x y i
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_serialize_5_upper_bits (x y: t_Vec256) (i: nat{i < 40})
  : Lemma
      (requires (forall (k: nat{k < 256}).
                   bv_bit y k ==
                   (if k % 32 < 5 then bv_bit x ((k / 32) * 32 + k % 32)
                    else if k % 32 < 10 then bv_bit x ((k / 32) * 32 + 16 + (k % 32 - 5))
                    else 0)))
      (ensures bv_bit (mm256_extracti128_si256 (mk_i32 1) (ser5_chain y)) i ==
               bv_bit x (128 + (i / 5) * 16 + i % 5)) =
  FStar.Math.Lemmas.small_division_lemma_1 i 128;
  FStar.Math.Lemmas.lemma_div_plus i 1 128;
  FStar.Math.Lemmas.lemma_mod_plus i 1 128;
  FStar.Math.Lemmas.small_mod i 128;
  lemma_bv_bit_extracti128_si256_1 (ser5_chain y) i;
  lemma_serialize_5_gather_bits x y (i + 128)
#pop-options

(* ── the two-store byte glue, shared by serialize_5 / _10 / _12 ─────────────
   Each of those widths writes `lower_8` into bytes [0,16) of a 32-byte scratch
   buffer, then `upper_8` into bytes [off, off+16) with off = 5 / 10 / 12, and
   returns the first 2*off bytes.  The second store CLOBBERS bytes [off,16) of
   the first — sound because only the low 8*off bits of `lower_8` are live, and
   those sit in bytes [0,off).

   Stated over the per-byte facts the caller obtains from
   `lemma_index_update_at_range`, so neither the store spine nor
   `update_at_range` appears in this context — the caller links by supplying
   the four hypotheses.  Pure index algebra otherwise. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_store_glue_bits
      (fin: t_Array u8 (mk_usize 32))
      (o1 o2: t_Array u8 (mk_usize 16))
      (lo hi: t_Vec128)
      (off: nat{1 <= off /\ off <= 16})
      (i: nat{i < 16 * off})
  : Lemma
      (requires
        (forall (j: nat{j < 128}).
           Rust_primitives.BitVectors.bit_vec_of_int_t_array o1 8 j == bv_bit lo j) /\
        (forall (j: nat{j < 128}).
           Rust_primitives.BitVectors.bit_vec_of_int_t_array o2 8 j == bv_bit hi j) /\
        (forall (k: nat{k < off}). Seq.index fin k == Seq.index o1 k) /\
        (forall (k: nat{off <= k /\ k < off + 16}). Seq.index fin k == Seq.index o2 (k - off)))
      (ensures
        Rust_primitives.BitVectors.bit_vec_of_int_t_array fin 8 i ==
        (if i < 8 * off then bv_bit lo i else bv_bit hi (i - 8 * off))) =
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  if i < 8 * off then begin
    (* byte i/8 lies in [0, off) — untouched by the second store *)
    FStar.Math.Lemmas.lemma_div_lt_nat i (8 * off) 8;
    assert (i / 8 < off);
    assert (Seq.index fin (i / 8) == Seq.index o1 (i / 8));
    assert (Rust_primitives.BitVectors.bit_vec_of_int_t_array o1 8 i == bv_bit lo i)
  end
  else begin
    (* byte i/8 lies in [off, 2*off) subset [off, off+16) — from the second store *)
    let j = i - 8 * off in
    FStar.Math.Lemmas.lemma_div_plus j off 8;
    FStar.Math.Lemmas.lemma_mod_plus j off 8;
    assert (i / 8 == j / 8 + off);
    assert (i % 8 == j % 8);
    FStar.Math.Lemmas.lemma_div_lt_nat j (8 * off) 8;
    assert (j < 8 * off /\ j < 128);
    assert (Seq.index fin (i / 8) == Seq.index o2 (j / 8));
    assert (Rust_primitives.BitVectors.bit_vec_of_int_t_array o2 8 j == bv_bit hi j)
  end
#pop-options

(* ── the two-store SPINE, in clean context ─────────────────────────────────
   `lemma_store_glue_bits` above consumes the per-byte frame facts.  Supplying
   those in the CONSUMER drags `update_at_range`, the two slice reads and the
   unbounded `forall (j: nat)` of `lemma_index_update_at_range` into the
   consumer's WP — measured to saturate `serialize_5_` at 400.000 even with
   `#restart-solver`, i.e. structural, not solver state.

   This lemma swallows the whole spine instead.  The caller passes the two
   array snapshots plus the two store results and discharges the two `==`
   hypotheses from its own let-equations (ground congruence); everything
   about `update_at_range` stays here, and what comes back is the finished
   per-bit `forall`.  Shared verbatim by serialize_5 / _10 / _12, which differ
   only in `off` (5 / 10 / 12) and hence in the returned prefix width. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let lemma_store_glue_two_writes
      (ser0 ser1 fin: t_Array u8 (mk_usize 32))
      (o1 o2: t_Array u8 (mk_usize 16))
      (lo hi: t_Vec128)
      (off: nat{1 <= off /\ off <= 16})
      (r0:
        Core_models.Ops.Range.t_Range usize
          { v r0.Core_models.Ops.Range.f_start == 0 /\
            v r0.Core_models.Ops.Range.f_end == 16 })
      (r1:
        Core_models.Ops.Range.t_Range usize
          { v r1.Core_models.Ops.Range.f_start == off /\
            v r1.Core_models.Ops.Range.f_end == off + 16 })
  : Lemma
      (requires
        ser1 == Rust_primitives.Hax.Monomorphized_update_at.update_at_range
                  (ser0 <: t_Slice u8) r0 (o1 <: t_Slice u8) /\
        fin == Rust_primitives.Hax.Monomorphized_update_at.update_at_range
                 (ser1 <: t_Slice u8) r1 (o2 <: t_Slice u8) /\
        (forall (j: nat{j < 128}).
           Rust_primitives.BitVectors.bit_vec_of_int_t_array o1 8 j == bv_bit lo j) /\
        (forall (j: nat{j < 128}).
           Rust_primitives.BitVectors.bit_vec_of_int_t_array o2 8 j == bv_bit hi j))
      (ensures
        forall (i: nat{i < 16 * off}).
          Rust_primitives.BitVectors.bit_vec_of_int_t_array fin 8 i ==
          (if i < 8 * off then bv_bit lo i else bv_bit hi (i - 8 * off))) =
  Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range
    (ser0 <: t_Slice u8) r0 (o1 <: t_Slice u8);
  Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range
    (ser1 <: t_Slice u8) r1 (o2 <: t_Slice u8);
  assert (forall (k: nat{k < off}). Seq.index fin k == Seq.index o1 k);
  assert (forall (k: nat{off <= k /\ k < off + 16}).
            Seq.index fin k == Seq.index o2 (k - off));
  let aux (i: nat{i < 16 * off})
    : Lemma
        (Rust_primitives.BitVectors.bit_vec_of_int_t_array fin 8 i ==
         (if i < 8 * off then bv_bit lo i else bv_bit hi (i - 8 * off))) =
    lemma_store_glue_bits fin o1 o2 lo hi off i
  in
  FStar.Classical.forall_intro aux
#pop-options

(* ============================================================================
   THE serialize_10 AND serialize_12 GATHER CHAINS

   Both are STRICTLY SHORTER than serialize_5: after `concat_pairs_n n` each
   64-bit lane holds 2n live bits, and for n = 10 / 12 that is already
   byte-aligned (40 = 5 bytes, 48 = 6 bytes), so ONE sllv/srli64 pair suffices
   and the single VPSHUFB gathers the bytes.  serialize_5 needed a second pair
   only because 2*5 = 10 bits is not byte-aligned.

   Consequently these two widths add NO new op-level machinery — they reuse
   `lemma_bv_bit_mm256_sllv_epi32`, `lemma_bv_bit_mm256_srli_epi64`,
   `lemma_bv_bit_mm256_shuffle_epi8_sel`, `lemma_bv_bit_castsi256_si128` and
   `lemma_bv_bit_extracti128_si256_1` unchanged.  Per width the only new
   content is: two ground constants, one shift-pair index shift, one shuffle
   index shift, the gather dispatch, and the two consumer forms.
   ========================================================================== *)

(* ── the width-generic index algebra, shared by every serialize_N ──────────
   After the gather, output bit `u` reads y-bit `32*(u/2n) + u%2n`; composing
   the `concat_pairs_n n` post with that yields exactly `(u/n)*16 + u%n`.
   Pure integers, no vector terms.  (serialize_5's `lemma_ser5_index` carries
   the same fact at n=5 plus the extra two-stage decomposition it needs.) *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 300 --split_queries always"
let lemma_pack_index (n: nat{n >= 1}) (u: nat)
  : Lemma (ensures
            (u % (2 * n) < n ==> (u / n) * 16 + u % n == 32 * (u / (2 * n)) + u % (2 * n)) /\
            (u % (2 * n) >= n ==>
               (u / n) * 16 + u % n == 32 * (u / (2 * n)) + 16 + (u % (2 * n) - n))) =
  (* u = (u % 2n) + 2n*(u/2n), so u/n = 2*(u/2n) + (u%2n)/n and u%n = (u%2n)%n *)
  FStar.Math.Lemmas.euclidean_division_definition u (2 * n);
  FStar.Math.Lemmas.lemma_div_plus (u % (2 * n)) (2 * (u / (2 * n))) n;
  FStar.Math.Lemmas.lemma_mod_plus (u % (2 * n)) (2 * (u / (2 * n))) n;
  if u % (2 * n) < n then begin
    FStar.Math.Lemmas.small_division_lemma_1 (u % (2 * n)) n;
    FStar.Math.Lemmas.small_mod (u % (2 * n)) n
  end
  else begin
    FStar.Math.Lemmas.lemma_div_plus (u % (2 * n) - n) 1 n;
    FStar.Math.Lemmas.lemma_mod_plus (u % (2 * n) - n) 1 n;
    FStar.Math.Lemmas.small_division_lemma_1 (u % (2 * n) - n) n;
    FStar.Math.Lemmas.small_mod (u % (2 * n) - n) n
  end
#pop-options

(* ────────────────────────────── serialize_10 ─────────────────────────────── *)

unfold let ser10_sh =
  mm256_set_epi32 (mk_i32 0) (mk_i32 12) (mk_i32 0) (mk_i32 12)
                  (mk_i32 0) (mk_i32 12) (mk_i32 0) (mk_i32 12)

unfold let ser10_mask =
  mm256_set_epi8 (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 12) (mk_i8 11)
    (mk_i8 10) (mk_i8 9) (mk_i8 8) (mk_i8 4)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 12) (mk_i8 11)
    (mk_i8 10) (mk_i8 9) (mk_i8 8) (mk_i8 4)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)

(* the whole register chain, exactly as serialize_10_vec composes it *)
unfold let ser10_chain (y: t_Vec256) : t_Vec256 =
  mm256_shuffle_epi8 (mm256_srli_epi64 (mk_i32 12) (mm256_sllv_epi32 y ser10_sh)) ser10_mask

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_ser10_sh_dwords (j: nat{j < 8})
  : Lemma (ensures v (vec256_dword ser10_sh j) == (if j % 2 = 0 then 12 else 0)) =
  reveal_opaque (`%mm256_set_epi32) mm256_set_epi32;
  Canon.lemma_mm256_set_epi32 (mk_i32 0) (mk_i32 12) (mk_i32 0) (mk_i32 12)
    (mk_i32 0) (mk_i32 12) (mk_i32 0) (mk_i32 12);
  Canon.lemma_iv_set_epi32 (mk_i32 0) (mk_i32 12) (mk_i32 0) (mk_i32 12)
    (mk_i32 0) (mk_i32 12) (mk_i32 0) (mk_i32 12) j
#pop-options

(* the 10 LIVE mask bytes per 128-bit half: bytes 0..4 select bytes 0..4 (the
   40 bits of 64-bit lane 0), bytes 5..9 select bytes 8..12 (lane 1). *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_ser10_mask_bytes (nth: nat{nth < 32})
  : Lemma (requires nth % 16 < 10)
          (ensures v (vec256_byte ser10_mask nth) ==
                   (if nth % 16 < 5 then nth % 16 else nth % 16 + 3)) =
  reveal_opaque (`%mm256_set_epi8) mm256_set_epi8;
  Canon.lemma_mm256_set_epi8 (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 12) (mk_i8 11)
    (mk_i8 10) (mk_i8 9) (mk_i8 8) (mk_i8 4)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 12) (mk_i8 11)
    (mk_i8 10) (mk_i8 9) (mk_i8 8) (mk_i8 4)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0);
  Canon.lemma_iv_set_epi8 (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 12) (mk_i8 11)
    (mk_i8 10) (mk_i8 9) (mk_i8 8) (mk_i8 4)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 12) (mk_i8 11)
    (mk_i8 10) (mk_i8 9) (mk_i8 8) (mk_i8 4)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0) nth
#pop-options

(* ── step 1: the 12-shift pair, as an index shift ─────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_ser10_shift_bit (y: t_Vec256) (i: nat{i < 256})
  : Lemma (requires i % 64 < 40)
          (ensures bv_bit (mm256_srli_epi64 (mk_i32 12) (mm256_sllv_epi32 y ser10_sh)) i ==
                   bv_bit y (if i % 64 < 20 then i else i + 12)) =
  let q = i / 64 in
  let u = i % 64 in
  FStar.Math.Lemmas.euclidean_division_definition i 64;
  lemma_bv_bit_mm256_srli_epi64 (mk_i32 12) (mm256_sllv_epi32 y ser10_sh) i;
  if u < 20 then begin
    (* index 64q + u + 12 sits in dword 2q (shift 12), at bit u + 12 *)
    FStar.Math.Lemmas.small_division_lemma_1 (u + 12) 32;
    FStar.Math.Lemmas.lemma_div_plus (u + 12) (2 * q) 32;
    FStar.Math.Lemmas.lemma_mod_plus (u + 12) (2 * q) 32;
    FStar.Math.Lemmas.small_mod (u + 12) 32;
    lemma_ser10_sh_dwords (2 * q);
    lemma_bv_bit_mm256_sllv_epi32 y ser10_sh (64 * q + u + 12) 12
  end
  else begin
    (* index 64q + u + 12 sits in dword 2q+1 (shift 0), at bit u - 20 *)
    FStar.Math.Lemmas.small_division_lemma_1 (u - 20) 32;
    FStar.Math.Lemmas.lemma_div_plus (u - 20) (2 * q + 1) 32;
    FStar.Math.Lemmas.lemma_mod_plus (u - 20) (2 * q + 1) 32;
    FStar.Math.Lemmas.small_mod (u - 20) 32;
    lemma_ser10_sh_dwords (2 * q + 1);
    lemma_bv_bit_mm256_sllv_epi32 y ser10_sh (64 * q + u + 12) 0
  end
#pop-options

(* ── step 2: VPSHUFB, as an index shift ───────────────────────────────────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_ser10_shuffle_bit (b: t_Vec256) (k: nat{k < 256})
  : Lemma (requires k % 128 < 80)
          (ensures bv_bit (mm256_shuffle_epi8 b ser10_mask) k ==
                   bv_bit b (if k % 128 < 40 then k else k + 24)) =
  let h = k / 128 in
  let u = k % 128 in
  let nth = k / 8 in
  let sb = k % 8 in
  FStar.Math.Lemmas.euclidean_division_definition k 128;
  FStar.Math.Lemmas.euclidean_division_definition k 8;
  FStar.Math.Lemmas.euclidean_division_definition u 8;
  (* nth == 16h + u/8, and u/8 < 10, so nth % 16 == u/8 and nth / 16 == h *)
  FStar.Math.Lemmas.small_division_lemma_1 (u / 8) 16;
  FStar.Math.Lemmas.lemma_div_plus (u / 8) h 16;
  FStar.Math.Lemmas.lemma_mod_plus (u / 8) h 16;
  FStar.Math.Lemmas.small_mod (u / 8) 16;
  lemma_ser10_mask_bytes nth;
  let m = v (vec256_byte ser10_mask nth) in
  FStar.Math.Lemmas.small_mod m 16;
  lemma_bv_bit_mm256_shuffle_epi8_sel b ser10_mask k (16 * (nth / 16) + m % 16);
  assert (8 * (16 * (nth / 16) + m % 16) + sb == 128 * h + 8 * m + sb)
#pop-options

(* ── the gather, dispatched over the two 40-bit windows of each half ──────── *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_ser10_gather_bit (y: t_Vec256) (i: nat{i < 256})
  : Lemma (requires i % 128 < 80)
          (ensures bv_bit (ser10_chain y) i ==
                   bv_bit y (128 * (i / 128) + 32 * ((i % 128) / 20) + (i % 128) % 20)) =
  let h = i / 128 in
  let u = i % 128 in
  FStar.Math.Lemmas.euclidean_division_definition i 128;
  let b = mm256_srli_epi64 (mk_i32 12) (mm256_sllv_epi32 y ser10_sh) in
  lemma_ser10_shuffle_bit b i;
  if u < 40 then begin
    (* the shuffle stays at i, whose i % 64 == u < 40 *)
    FStar.Math.Lemmas.small_division_lemma_1 u 64;
    FStar.Math.Lemmas.lemma_div_plus u (2 * h) 64;
    FStar.Math.Lemmas.lemma_mod_plus u (2 * h) 64;
    FStar.Math.Lemmas.small_mod u 64;
    lemma_ser10_shift_bit y i
  end
  else begin
    (* the shuffle moves to i+24 == 64*(2h+1) + (u-40), so (i+24) % 64 == u-40 *)
    FStar.Math.Lemmas.small_division_lemma_1 (u - 40) 64;
    FStar.Math.Lemmas.lemma_div_plus (u - 40) (2 * h + 1) 64;
    FStar.Math.Lemmas.lemma_mod_plus (u - 40) (2 * h + 1) 64;
    FStar.Math.Lemmas.small_mod (u - 40) 64;
    lemma_ser10_shift_bit y (i + 24)
  end
#pop-options

(* THE serialize_10 obligation, per output bit of the 256-bit chain result. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_serialize_10_gather_bits (x y: t_Vec256) (i: nat{i < 256})
  : Lemma
      (requires
        i % 128 < 80 /\
        (forall (k: nat{k < 256}).
           bv_bit y k ==
           (if k % 32 < 10 then bv_bit x ((k / 32) * 32 + k % 32)
            else if k % 32 < 20 then bv_bit x ((k / 32) * 32 + 16 + (k % 32 - 10))
            else 0)))
      (ensures bv_bit (ser10_chain y) i ==
               bv_bit x (128 * (i / 128) + ((i % 128) / 10) * 16 + (i % 128) % 10)) =
  let h = i / 128 in
  let u = i % 128 in
  lemma_ser10_gather_bit y i;
  lemma_pack_index 10 u;
  (* the y-index is 32 * (4h + u/20) + u%20, so its %32 is u%20 and /32 is 4h+u/20 *)
  FStar.Math.Lemmas.small_division_lemma_1 (u % 20) 32;
  FStar.Math.Lemmas.lemma_div_plus (u % 20) (4 * h + u / 20) 32;
  FStar.Math.Lemmas.lemma_mod_plus (u % 20) (4 * h + u / 20) 32;
  FStar.Math.Lemmas.small_mod (u % 20) 32
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_serialize_10_lower_bits (x y: t_Vec256) (i: nat{i < 80})
  : Lemma
      (requires (forall (k: nat{k < 256}).
                   bv_bit y k ==
                   (if k % 32 < 10 then bv_bit x ((k / 32) * 32 + k % 32)
                    else if k % 32 < 20 then bv_bit x ((k / 32) * 32 + 16 + (k % 32 - 10))
                    else 0)))
      (ensures bv_bit (mm256_castsi256_si128 (ser10_chain y)) i ==
               bv_bit x ((i / 10) * 16 + i % 10)) =
  FStar.Math.Lemmas.small_division_lemma_1 i 128;
  FStar.Math.Lemmas.small_mod i 128;
  lemma_bv_bit_castsi256_si128 (ser10_chain y) i;
  lemma_serialize_10_gather_bits x y i
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_serialize_10_upper_bits (x y: t_Vec256) (i: nat{i < 80})
  : Lemma
      (requires (forall (k: nat{k < 256}).
                   bv_bit y k ==
                   (if k % 32 < 10 then bv_bit x ((k / 32) * 32 + k % 32)
                    else if k % 32 < 20 then bv_bit x ((k / 32) * 32 + 16 + (k % 32 - 10))
                    else 0)))
      (ensures bv_bit (mm256_extracti128_si256 (mk_i32 1) (ser10_chain y)) i ==
               bv_bit x (128 + (i / 10) * 16 + i % 10)) =
  FStar.Math.Lemmas.small_division_lemma_1 i 128;
  FStar.Math.Lemmas.lemma_div_plus i 1 128;
  FStar.Math.Lemmas.lemma_mod_plus i 1 128;
  FStar.Math.Lemmas.small_mod i 128;
  lemma_bv_bit_extracti128_si256_1 (ser10_chain y) i;
  lemma_serialize_10_gather_bits x y (i + 128)
#pop-options

(* ────────────────────────────── serialize_12 ─────────────────────────────── *)

unfold let ser12_sh =
  mm256_set_epi32 (mk_i32 0) (mk_i32 8) (mk_i32 0) (mk_i32 8)
                  (mk_i32 0) (mk_i32 8) (mk_i32 0) (mk_i32 8)

unfold let ser12_mask =
  mm256_set_epi8 (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 13) (mk_i8 12) (mk_i8 11) (mk_i8 10)
    (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 13) (mk_i8 12) (mk_i8 11) (mk_i8 10)
    (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)

unfold let ser12_chain (y: t_Vec256) : t_Vec256 =
  mm256_shuffle_epi8 (mm256_srli_epi64 (mk_i32 8) (mm256_sllv_epi32 y ser12_sh)) ser12_mask

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_ser12_sh_dwords (j: nat{j < 8})
  : Lemma (ensures v (vec256_dword ser12_sh j) == (if j % 2 = 0 then 8 else 0)) =
  reveal_opaque (`%mm256_set_epi32) mm256_set_epi32;
  Canon.lemma_mm256_set_epi32 (mk_i32 0) (mk_i32 8) (mk_i32 0) (mk_i32 8)
    (mk_i32 0) (mk_i32 8) (mk_i32 0) (mk_i32 8);
  Canon.lemma_iv_set_epi32 (mk_i32 0) (mk_i32 8) (mk_i32 0) (mk_i32 8)
    (mk_i32 0) (mk_i32 8) (mk_i32 0) (mk_i32 8) j
#pop-options

(* the 12 LIVE mask bytes per 128-bit half: bytes 0..5 select bytes 0..5 (the
   48 bits of 64-bit lane 0), bytes 6..11 select bytes 8..13 (lane 1). *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_ser12_mask_bytes (nth: nat{nth < 32})
  : Lemma (requires nth % 16 < 12)
          (ensures v (vec256_byte ser12_mask nth) ==
                   (if nth % 16 < 6 then nth % 16 else nth % 16 + 2)) =
  reveal_opaque (`%mm256_set_epi8) mm256_set_epi8;
  Canon.lemma_mm256_set_epi8 (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 13) (mk_i8 12) (mk_i8 11) (mk_i8 10)
    (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 13) (mk_i8 12) (mk_i8 11) (mk_i8 10)
    (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0);
  Canon.lemma_iv_set_epi8 (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 13) (mk_i8 12) (mk_i8 11) (mk_i8 10)
    (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1)) (mk_i8 (-1))
    (mk_i8 13) (mk_i8 12) (mk_i8 11) (mk_i8 10)
    (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
    (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0) nth
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_ser12_shift_bit (y: t_Vec256) (i: nat{i < 256})
  : Lemma (requires i % 64 < 48)
          (ensures bv_bit (mm256_srli_epi64 (mk_i32 8) (mm256_sllv_epi32 y ser12_sh)) i ==
                   bv_bit y (if i % 64 < 24 then i else i + 8)) =
  let q = i / 64 in
  let u = i % 64 in
  FStar.Math.Lemmas.euclidean_division_definition i 64;
  lemma_bv_bit_mm256_srli_epi64 (mk_i32 8) (mm256_sllv_epi32 y ser12_sh) i;
  if u < 24 then begin
    (* index 64q + u + 8 sits in dword 2q (shift 8), at bit u + 8 *)
    FStar.Math.Lemmas.small_division_lemma_1 (u + 8) 32;
    FStar.Math.Lemmas.lemma_div_plus (u + 8) (2 * q) 32;
    FStar.Math.Lemmas.lemma_mod_plus (u + 8) (2 * q) 32;
    FStar.Math.Lemmas.small_mod (u + 8) 32;
    lemma_ser12_sh_dwords (2 * q);
    lemma_bv_bit_mm256_sllv_epi32 y ser12_sh (64 * q + u + 8) 8
  end
  else begin
    (* index 64q + u + 8 sits in dword 2q+1 (shift 0), at bit u - 24 *)
    FStar.Math.Lemmas.small_division_lemma_1 (u - 24) 32;
    FStar.Math.Lemmas.lemma_div_plus (u - 24) (2 * q + 1) 32;
    FStar.Math.Lemmas.lemma_mod_plus (u - 24) (2 * q + 1) 32;
    FStar.Math.Lemmas.small_mod (u - 24) 32;
    lemma_ser12_sh_dwords (2 * q + 1);
    lemma_bv_bit_mm256_sllv_epi32 y ser12_sh (64 * q + u + 8) 0
  end
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_ser12_shuffle_bit (b: t_Vec256) (k: nat{k < 256})
  : Lemma (requires k % 128 < 96)
          (ensures bv_bit (mm256_shuffle_epi8 b ser12_mask) k ==
                   bv_bit b (if k % 128 < 48 then k else k + 16)) =
  let h = k / 128 in
  let u = k % 128 in
  let nth = k / 8 in
  let sb = k % 8 in
  FStar.Math.Lemmas.euclidean_division_definition k 128;
  FStar.Math.Lemmas.euclidean_division_definition k 8;
  FStar.Math.Lemmas.euclidean_division_definition u 8;
  (* nth == 16h + u/8, and u/8 < 12, so nth % 16 == u/8 and nth / 16 == h *)
  FStar.Math.Lemmas.small_division_lemma_1 (u / 8) 16;
  FStar.Math.Lemmas.lemma_div_plus (u / 8) h 16;
  FStar.Math.Lemmas.lemma_mod_plus (u / 8) h 16;
  FStar.Math.Lemmas.small_mod (u / 8) 16;
  lemma_ser12_mask_bytes nth;
  let m = v (vec256_byte ser12_mask nth) in
  FStar.Math.Lemmas.small_mod m 16;
  lemma_bv_bit_mm256_shuffle_epi8_sel b ser12_mask k (16 * (nth / 16) + m % 16);
  assert (8 * (16 * (nth / 16) + m % 16) + sb == 128 * h + 8 * m + sb)
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_ser12_gather_bit (y: t_Vec256) (i: nat{i < 256})
  : Lemma (requires i % 128 < 96)
          (ensures bv_bit (ser12_chain y) i ==
                   bv_bit y (128 * (i / 128) + 32 * ((i % 128) / 24) + (i % 128) % 24)) =
  let h = i / 128 in
  let u = i % 128 in
  FStar.Math.Lemmas.euclidean_division_definition i 128;
  let b = mm256_srli_epi64 (mk_i32 8) (mm256_sllv_epi32 y ser12_sh) in
  lemma_ser12_shuffle_bit b i;
  if u < 48 then begin
    (* the shuffle stays at i, whose i % 64 == u < 48 *)
    FStar.Math.Lemmas.small_division_lemma_1 u 64;
    FStar.Math.Lemmas.lemma_div_plus u (2 * h) 64;
    FStar.Math.Lemmas.lemma_mod_plus u (2 * h) 64;
    FStar.Math.Lemmas.small_mod u 64;
    lemma_ser12_shift_bit y i
  end
  else begin
    (* the shuffle moves to i+16 == 64*(2h+1) + (u-48), so (i+16) % 64 == u-48 *)
    FStar.Math.Lemmas.small_division_lemma_1 (u - 48) 64;
    FStar.Math.Lemmas.lemma_div_plus (u - 48) (2 * h + 1) 64;
    FStar.Math.Lemmas.lemma_mod_plus (u - 48) (2 * h + 1) 64;
    FStar.Math.Lemmas.small_mod (u - 48) 64;
    lemma_ser12_shift_bit y (i + 16)
  end
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_serialize_12_gather_bits (x y: t_Vec256) (i: nat{i < 256})
  : Lemma
      (requires
        i % 128 < 96 /\
        (forall (k: nat{k < 256}).
           bv_bit y k ==
           (if k % 32 < 12 then bv_bit x ((k / 32) * 32 + k % 32)
            else if k % 32 < 24 then bv_bit x ((k / 32) * 32 + 16 + (k % 32 - 12))
            else 0)))
      (ensures bv_bit (ser12_chain y) i ==
               bv_bit x (128 * (i / 128) + ((i % 128) / 12) * 16 + (i % 128) % 12)) =
  let h = i / 128 in
  let u = i % 128 in
  lemma_ser12_gather_bit y i;
  lemma_pack_index 12 u;
  (* the y-index is 32 * (4h + u/24) + u%24, so its %32 is u%24 and /32 is 4h+u/24 *)
  FStar.Math.Lemmas.small_division_lemma_1 (u % 24) 32;
  FStar.Math.Lemmas.lemma_div_plus (u % 24) (4 * h + u / 24) 32;
  FStar.Math.Lemmas.lemma_mod_plus (u % 24) (4 * h + u / 24) 32;
  FStar.Math.Lemmas.small_mod (u % 24) 32
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_serialize_12_lower_bits (x y: t_Vec256) (i: nat{i < 96})
  : Lemma
      (requires (forall (k: nat{k < 256}).
                   bv_bit y k ==
                   (if k % 32 < 12 then bv_bit x ((k / 32) * 32 + k % 32)
                    else if k % 32 < 24 then bv_bit x ((k / 32) * 32 + 16 + (k % 32 - 12))
                    else 0)))
      (ensures bv_bit (mm256_castsi256_si128 (ser12_chain y)) i ==
               bv_bit x ((i / 12) * 16 + i % 12)) =
  FStar.Math.Lemmas.small_division_lemma_1 i 128;
  FStar.Math.Lemmas.small_mod i 128;
  lemma_bv_bit_castsi256_si128 (ser12_chain y) i;
  lemma_serialize_12_gather_bits x y i
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_serialize_12_upper_bits (x y: t_Vec256) (i: nat{i < 96})
  : Lemma
      (requires (forall (k: nat{k < 256}).
                   bv_bit y k ==
                   (if k % 32 < 12 then bv_bit x ((k / 32) * 32 + k % 32)
                    else if k % 32 < 24 then bv_bit x ((k / 32) * 32 + 16 + (k % 32 - 12))
                    else 0)))
      (ensures bv_bit (mm256_extracti128_si256 (mk_i32 1) (ser12_chain y)) i ==
               bv_bit x (128 + (i / 12) * 16 + i % 12)) =
  FStar.Math.Lemmas.small_division_lemma_1 i 128;
  FStar.Math.Lemmas.lemma_div_plus i 1 128;
  FStar.Math.Lemmas.lemma_mod_plus i 1 128;
  FStar.Math.Lemmas.small_mod i 128;
  lemma_bv_bit_extracti128_si256_1 (ser12_chain y) i;
  lemma_serialize_12_gather_bits x y (i + 128)
#pop-options

(* ── deserialize_5: the 256-bit gather shuffle ───────────────────────────────
   `deserialize_5_vec` duplicates its single 128-bit input into BOTH halves and
   then gathers with one 256-bit VPSHUFB.  Within each 128-bit lane byte b
   reads source byte (b/4)*2 + b%2, and the high lane's map is that shifted by
   8 — the 5-bits-per-coefficient stride packs 5 source bytes per 8
   coefficients, so each 4-byte output group re-reads a 2-byte source window.

   Stated in the piecewise form the function's own `ensures` uses (`c_byte`),
   so the composition downstream is a pure INDEX SHIFT with no arithmetic to
   re-derive under the `bv_bit`. *)
unfold let deser5_bytemap (b: nat{b < 32}) : nat =
  if b < 16 then (b / 4) * 2 + b % 2 else ((b - 16) / 4) * 2 + (b - 16) % 2 + 8

unfold let deser5_mask =
  mm256_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 15) (mk_i8 14) (mk_i8 13) (mk_i8 12) (mk_i8 13)
                 (mk_i8 12) (mk_i8 11) (mk_i8 10) (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 8)
                 (mk_i8 9) (mk_i8 8) (mk_i8 7) (mk_i8 6) (mk_i8 7) (mk_i8 6) (mk_i8 5) (mk_i8 4)
                 (mk_i8 5) (mk_i8 4) (mk_i8 3) (mk_i8 2) (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
                 (mk_i8 1) (mk_i8 0)

#push-options "--fuel 1 --ifuel 2 --z3rlimit 300"
let lemma_deser5_mask_bytes (nth: nat{nth < 32})
  : Lemma (ensures v (vec256_byte deser5_mask nth) == deser5_bytemap nth) =
  reveal_opaque (`%mm256_set_epi8) mm256_set_epi8;
  Canon.lemma_mm256_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 15) (mk_i8 14) (mk_i8 13) (mk_i8 12)
    (mk_i8 13) (mk_i8 12) (mk_i8 11) (mk_i8 10) (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 8)
    (mk_i8 9) (mk_i8 8) (mk_i8 7) (mk_i8 6) (mk_i8 7) (mk_i8 6) (mk_i8 5) (mk_i8 4)
    (mk_i8 5) (mk_i8 4) (mk_i8 3) (mk_i8 2) (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 1) (mk_i8 0);
  Canon.lemma_iv_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 15) (mk_i8 14) (mk_i8 13) (mk_i8 12)
    (mk_i8 13) (mk_i8 12) (mk_i8 11) (mk_i8 10) (mk_i8 11) (mk_i8 10) (mk_i8 9) (mk_i8 8)
    (mk_i8 9) (mk_i8 8) (mk_i8 7) (mk_i8 6) (mk_i8 7) (mk_i8 6) (mk_i8 5) (mk_i8 4)
    (mk_i8 5) (mk_i8 4) (mk_i8 3) (mk_i8 2) (mk_i8 3) (mk_i8 2) (mk_i8 1) (mk_i8 0)
    (mk_i8 1) (mk_i8 0) nth
#pop-options

(* the shuffle, as a pure index shift.  The `16 * (nth / 16)` of the sel form
   is the per-128-lane restriction of VPSHUFB; it survives as the
   `128 * (i / 128)` below. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser5_shuffle_bit (a: t_Vec256) (i: nat{i < 256})
  : Lemma (ensures bv_bit (mm256_shuffle_epi8 a deser5_mask) i ==
                   bv_bit a (128 * (i / 128) + 8 * deser5_bytemap (i / 8) + i % 8)) =
  let nth = i / 8 in
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  FStar.Math.Lemmas.division_multiplication_lemma i 8 16;
  lemma_deser5_mask_bytes nth;
  FStar.Math.Lemmas.small_mod (deser5_bytemap nth) 16;
  lemma_bv_bit_mm256_shuffle_epi8_sel a deser5_mask i (16 * (nth / 16) + deser5_bytemap nth)
#pop-options

(* ── the gather, with the 128+128 duplication collapsed ──────────────────────
   `coefficients_loaded` is `mm256_si256_from_two_si128 c c`, so BOTH halves
   are `c` and the `128 * (i / 128)` lane offset above cancels against the
   concat's own `k - 128`: every bit lands on the same index of `c`. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_deser5_gather_bit (c: t_Vec128) (co: t_Vec256) (i: nat{i < 256})
  : Lemma (requires
             forall (k: nat{k < 256}).
               bv_bit co k == (if k < 128 then bv_bit c k else bv_bit c (k - 128)))
          (ensures bv_bit (mm256_shuffle_epi8 co deser5_mask) i ==
                   bv_bit c (8 * deser5_bytemap (i / 8) + i % 8)) =
  FStar.Math.Lemmas.euclidean_division_definition i 8;
  lemma_deser5_shuffle_bit co i;
  if i < 128 then assert (i / 128 == 0) else assert (i / 128 == 1)
#pop-options

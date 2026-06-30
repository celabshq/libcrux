module Libcrux_intrinsics.Avx2_ml_kem_views
#set-options "--fuel 0 --ifuel 1 --z3rlimit 50"
open FStar.Mul
open Core_models
open Libcrux_intrinsics.Avx2_extract

(* ml-kem-only AVX2 intrinsic view-axioms, RELOCATED out of the shared
   `Libcrux_intrinsics.Avx2_extract` interface (2026-06-30).

   These five lemmas carry global `[SMTPat]`s on the i16/byte/lane32 views of
   AVX2 ops that ml-kem's Compress/Serialize/(top) Vector.Avx2 proofs need, but
   that sha3 never takes. Leaving them in the shared `Avx2_extract` interface
   that sha3 also `open`s bloated it (624->959 lines, +5 global SMTPats) and
   saturated sha3's byte-window discharges (Store.fst store_u64x4x4 /
   lemma_window_modifies). They cannot be context_pruning-pruned for sha3
   because their SMTPat triggers (mm256_storeu_si256_u8 / mm256_loadu_si256_u8 /
   mm256_unpackhi_epi64 / vec256_as_i16x16 ...) DO form in sha3's Keccak store.

   Relocating them here — a module ONLY ml-kem opens, NEVER sha3 — keeps sha3's
   interface lean while preserving ml-kem's trust footprint exactly: each was an
   assumed `val`/`admit ()` trust-axiom before (validated by the core-models
   differential + transcription tests), and stays an `admit ()` here. No new
   trust. This module lives in `proofs/fstar/spec/` (hand-maintained, NOT the
   hax-extraction dir) so `cargo hax into` does not clobber it on re-extraction,
   and it is on ml-kem's include path but not sha3's. See ~/hax-fstar-mcp/
   libcrux-notes/agent-status/sha3-avx2-regression-rootcause-2026-06-30.md. *)

let lemma_mm256_storeu_si256_u8_bit_vec (output: t_Slice u8) (vector: t_Vec256)
  : Lemma (requires Core_models.Slice.impl__len #u8 output == mk_usize 32)
          (ensures
      (let output_future = mm256_storeu_si256_u8 output vector in
       Core_models.Slice.impl__len #u8 output_future ==
         Core_models.Slice.impl__len #u8 output /\
       (let output_arr: t_Array u8 (sz 32) = output_future in
        BitVecEq.bit_vec_equal
          (Rust_primitives.BitVectors.bit_vec_of_int_t_array output_arr 8) vector)))
    [SMTPat (mm256_storeu_si256_u8 output vector)]
  = admit ()

let lemma_mm256_loadu_si256_u8_bit_vec (input: t_Slice u8)
    : Lemma (requires Core_models.Slice.impl__len #u8 input == mk_usize 32)
            (ensures (let input_arr: t_Array u8 (sz 32) = input in
              BitVecEq.bit_vec_equal (mm256_loadu_si256_u8 input)
                (Rust_primitives.BitVectors.bit_vec_of_int_t_array input_arr 8)))
            [SMTPat (mm256_loadu_si256_u8 input)]
  = admit ()

(* ml-kem i16-view characterization (called explicitly by
   Libcrux_ml_kem.Vector.Avx2.Compress; also SMTPat). Coexists with the
   `lemma_mm256_xor_si256_u64x4` kept in Avx2_extract: the two describe
   disjoint lane views (i16 vs u64) of the same value. *)
let lemma_mm256_xor_si256 (lhs rhs: t_Vec256)
  : Lemma (   vec256_as_i16x16 (mm256_xor_si256 lhs rhs)
           == Spec.Utils.map2 (^.) (vec256_as_i16x16 lhs) (vec256_as_i16x16 rhs)
          )
          [SMTPat (vec256_as_i16x16 (mm256_xor_si256 lhs rhs))]
  = admit ()

(* ml-kem i16-view characterization (called explicitly by
   Libcrux_ml_kem.Vector.Avx2.Compress, e.g. lemma_mm256_srli_epi16_15; also
   SMTPat). *)
let lemma_mm256_srli_epi16 (v_SHIFT_BY: i32 {v v_SHIFT_BY >= 0 /\ v v_SHIFT_BY < 16}) (vector: t_Vec256)
  : Lemma (   vec256_as_i16x16 (mm256_srli_epi16 v_SHIFT_BY vector)
           == Spec.Utils.map_array (fun (x:i16) ->
                  cast ((cast x <: u16) >>! v_SHIFT_BY) <: i16)
                (vec256_as_i16x16 vector)
          )
          [SMTPat (vec256_as_i16x16 (mm256_srli_epi16 v_SHIFT_BY vector))]
  = admit ()

(* ml-kem i16-view (lane32) of the qword permutation, used by
   Libcrux_ml_kem.Vector.Avx2.Compress's mulhi composite lemma (SMTPat). The
   `lemma_mm256_unpackhi_epi64_u64x4` u64x4-view stays in Avx2_extract for
   sha3; this lane32-view is ml-kem-only. *)
let lemma_mm256_unpackhi_epi64_lane32 (lhs rhs: t_Vec256)
  : Lemma (ensures forall (j: nat). j < 8 ==>
            lane32 (mm256_unpackhi_epi64 lhs rhs) j ==
            (match j with
              | 0 -> lane32 lhs 2 | 1 -> lane32 lhs 3
              | 2 -> lane32 rhs 2 | 3 -> lane32 rhs 3
              | 4 -> lane32 lhs 6 | 5 -> lane32 lhs 7
              | 6 -> lane32 rhs 6 | _ -> lane32 rhs 7))
    [SMTPat (mm256_unpackhi_epi64 lhs rhs)]
  = admit ()

module EquivImplSpec.Keccakf.Avx2

(* ================================================================
   AVX2 (N=4, v_T=t_Vec256) instantiation of the generic keccak_f
   equivalence proof.

   Mirrors the structure of [EquivImplSpec.Keccakf.Arm64] but at N=4.

   STRUCTURAL DIFFERENCE FROM Arm64:
   The Arm64 [t_e_uint64x2_t] is a refined Rust SIMD type with per-lane
   primitives (e_veorq_u64, e_vrax1q_u64, ...) whose F* extractions
   carry per-lane [ensures] clauses on [get_lane_u64x2].  The AVX2
   [t_Vec256 = bit_vec 256] uses bit_vec-level intrinsics
   (mm256_xor_si256, mm256_set1_epi64x, ...).  Their per-u64-lane
   semantics are exposed via the [lemma_mm256_*_u64x4] SMTPats in
   [Libcrux_intrinsics.Avx2_extract] (the AVX2 analogue of the NEON
   per-lane ensures), so the seven [lane_correctness] field proofs are
   discharged here by SMT — same external trust boundary as Arm64.

   With the seven primitives in hand, the main theorem
     [lemma_keccakf1600_avx2 ks l ==
        keccak_f (extract_lane lc_avx2 ks.f_st l)]
   follows from the generic [G.lemma_keccakf1600_to_spec].
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"

open FStar.Mul
open Core_models

module G = EquivImplSpec.Keccakf.Generic
module I = Libcrux_intrinsics.Avx2_extract

(* Bring the AVX2 typeclass instance into scope so
   t_KeccakItem t_Vec256 (mk_usize 4) resolves to
   [Libcrux_sha3.Simd.Avx2.impl]. *)
let _ =
  let open Libcrux_intrinsics.Avx2_extract in
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Avx2 in
  ()

(* ================================================================
   AVX2 lane extraction

   Mirrors the arm64 pattern: rather than reasoning at the bit_vec
   level, we use [get_lane_u64x4] (declared in
   [Libcrux_intrinsics.Avx2_extract] alongside an opaque
   [vec256_as_u64x4]) and rely on the [lemma_mm256_*_u64x4] SMTPats
   on the AVX2 intrinsics for per-u64-lane semantics.  The lane
   primitives are axiomatized (admit'd) in [Libcrux_intrinsics.Avx2_extract.fst]
   — the external trust boundary; this module then closes the [lc_*]
   proofs by unfolding [avx2_lane] onto those per-lane SMTPats (like arm64).
   ================================================================ *)

[@@ "opaque_to_smt"]
let avx2_lane (vec: I.t_Vec256) (l: nat{l < 4}) : u64 =
  I.get_lane_u64x4 vec l

(* Reveal helper for callers that need to unfold [avx2_lane] under SMT. *)
let lemma_avx2_lane_unfold (vec: I.t_Vec256) (l: nat{l < 4})
  : Lemma (avx2_lane vec l == I.get_lane_u64x4 vec l)
  = reveal_opaque (`%avx2_lane) (avx2_lane vec l)

(* ================================================================
   Lane-correctness field proofs.

   Each is discharged by unfolding the typeclass-dispatched [f_*] to the
   concrete [Simd.Avx2.{e_*, _v*}] helpers and pushing [avx2_lane]
   through the per-u64-lane semantics of the underlying AVX2 intrinsics
   (mm256_xor_si256, mm256_set1_epi64x, mm256_andnot_si256,
   mm256_slli_epi64, mm256_srli_epi64), which are exposed via the
   [lemma_mm256_*_u64x4] SMTPats in [Libcrux_intrinsics.Avx2_extract].
   That intrinsic layer is the (external) trust boundary, exactly as for
   the NEON per-lane ensures in [Arm64_extract].
   ================================================================ *)

(* The seven proofs below mirror the arm64 lane-correctness lemmas
   structurally — they unfold the typeclass-dispatched [f_*] to the
   concrete [Simd.Avx2.{e_*, _v*}] helpers, which call AVX2
   intrinsics whose per-u64-lane semantics are now SMTPat'd via
   [lemma_mm256_*_u64x4] (admitted in [Libcrux_intrinsics.Avx2_extract.fst]).
   Same trust boundary as arm64, where [Arm64_extract] declares
   [vec128_as_u64x2] + per-lane [ensures] clauses on the NEON
   intrinsics. *)

let avx2_lc_zero (l: nat{l < 4})
  : Lemma (avx2_lane (Libcrux_sha3.Traits.f_zero
             #I.t_Vec256 #(mk_usize 4)
             #FStar.Tactics.Typeclasses.solve ()) l == mk_u64 0)
  = let _ = I.lemma_mm256_set1_epi64x_u64x4 (mk_i64 0) in
    lemma_avx2_lane_unfold
      (Libcrux_sha3.Traits.f_zero
        #I.t_Vec256 #(mk_usize 4)
        #FStar.Tactics.Typeclasses.solve ()) l

let avx2_lc_xor5 (a b c d e: I.t_Vec256) (l: nat{l < 4})
  : Lemma (avx2_lane (Libcrux_sha3.Traits.f_xor5
             #I.t_Vec256 #(mk_usize 4)
             #FStar.Tactics.Typeclasses.solve a b c d e) l ==
           (((avx2_lane a l ^. avx2_lane b l) ^. avx2_lane c l)
             ^. avx2_lane d l) ^. avx2_lane e l)
  = lemma_avx2_lane_unfold a l;
    lemma_avx2_lane_unfold b l;
    lemma_avx2_lane_unfold c l;
    lemma_avx2_lane_unfold d l;
    lemma_avx2_lane_unfold e l;
    lemma_avx2_lane_unfold
      (Libcrux_sha3.Traits.f_xor5
        #I.t_Vec256 #(mk_usize 4)
        #FStar.Tactics.Typeclasses.solve a b c d e) l

(* Bridge: the AVX2 implementation of rotate_left as
   (x <<! LEFT) ^. (x >>! RIGHT) (when LEFT + RIGHT == 64) yields
   the per-lane Core_models rotate_left.  Proven in
   [Libcrux_sha3.Proof_utils.Lemmas.lemma_shl_xor_shr_is_rotate_left]. *)
module PUL = Libcrux_sha3.Proof_utils.Lemmas

#push-options "--z3rlimit 200"
let avx2_lc_rotate_left1_and_xor (a b: I.t_Vec256) (l: nat{l < 4})
  : Lemma (avx2_lane (Libcrux_sha3.Traits.f_rotate_left1_and_xor
             #I.t_Vec256 #(mk_usize 4)
             #FStar.Tactics.Typeclasses.solve a b) l ==
           avx2_lane a l ^.
           Core_models.Num.impl_u64__rotate_left (avx2_lane b l) (mk_u32 1))
  = (* e_vrax1q_u64 a b = mm256_xor_si256 a (rotate_left 1 63 b)
       where rotate_left 1 63 b = mm256_xor_si256 (mm256_slli_epi64 1 b) (mm256_srli_epi64 63 b).
       SMTPats give per-lane shifts/xor; bridge lemma equates
       (x <<! 1) ^. (x >>! 63) with impl_u64__rotate_left x 1. *)
    lemma_avx2_lane_unfold a l;
    lemma_avx2_lane_unfold b l;
    lemma_avx2_lane_unfold
      (Libcrux_sha3.Traits.f_rotate_left1_and_xor
        #I.t_Vec256 #(mk_usize 4)
        #FStar.Tactics.Typeclasses.solve a b) l;
    PUL.lemma_shl_xor_shr_is_rotate_left (avx2_lane b l) (mk_i32 1) (mk_i32 63)
#pop-options

#push-options "--z3rlimit 200"
let avx2_lc_xor_and_rotate (v_LEFT v_RIGHT: i32) (a b: I.t_Vec256) (l: nat{l < 4})
  : Lemma
      (requires
        ((Rust_primitives.Hax.Int.from_machine v_LEFT <: Hax_lib.Int.t_Int) +
         (Rust_primitives.Hax.Int.from_machine v_RIGHT <: Hax_lib.Int.t_Int)) =
        (Rust_primitives.Hax.Int.from_machine (mk_i32 64) <: Hax_lib.Int.t_Int) /\
        v_RIGHT >. mk_i32 0 /\
        v_RIGHT <. mk_i32 64)
      (ensures
        avx2_lane (Libcrux_sha3.Traits.f_xor_and_rotate
          #I.t_Vec256 #(mk_usize 4)
          #FStar.Tactics.Typeclasses.solve v_LEFT v_RIGHT a b) l ==
        Core_models.Num.impl_u64__rotate_left
          (avx2_lane a l ^. avx2_lane b l) (cast (v_LEFT <: i32) <: u32))
  = (* e_vxarq_u64 LEFT RIGHT a b = rotate_left LEFT RIGHT (mm256_xor_si256 a b).
       Per-lane: avx2_lane(xor a b) l = avx2_lane a l ^. avx2_lane b l, then
       rotate_left over that lane via lemma_shl_xor_shr_is_rotate_left. *)
    lemma_avx2_lane_unfold a l;
    lemma_avx2_lane_unfold b l;
    lemma_avx2_lane_unfold
      (Libcrux_sha3.Traits.f_xor_and_rotate
        #I.t_Vec256 #(mk_usize 4)
        #FStar.Tactics.Typeclasses.solve v_LEFT v_RIGHT a b) l;
    let xab_lane = avx2_lane a l ^. avx2_lane b l in
    PUL.lemma_shl_xor_shr_is_rotate_left xab_lane v_LEFT v_RIGHT
#pop-options

let avx2_lc_and_not_xor (a b c: I.t_Vec256) (l: nat{l < 4})
  : Lemma (avx2_lane (Libcrux_sha3.Traits.f_and_not_xor
             #I.t_Vec256 #(mk_usize 4)
             #FStar.Tactics.Typeclasses.solve a b c) l ==
           avx2_lane a l ^. (avx2_lane b l &. (~. (avx2_lane c l))))
  = lemma_avx2_lane_unfold a l;
    lemma_avx2_lane_unfold b l;
    lemma_avx2_lane_unfold c l;
    lemma_avx2_lane_unfold
      (Libcrux_sha3.Traits.f_and_not_xor
        #I.t_Vec256 #(mk_usize 4)
        #FStar.Tactics.Typeclasses.solve a b c) l

#push-options "--z3rlimit 200"
let avx2_lc_xor_constant (a: I.t_Vec256) (c: u64) (l: nat{l < 4})
  : Lemma (avx2_lane (Libcrux_sha3.Traits.f_xor_constant
             #I.t_Vec256 #(mk_usize 4)
             #FStar.Tactics.Typeclasses.solve a c) l ==
           avx2_lane a l ^. c)
  = (* e_veorq_n_u64 a c = mm256_xor_si256 a (mm256_set1_epi64x (cast c)).
       SMTPats give:
         avx2_lane (mm256_xor_si256 a x) l = avx2_lane a l ^. avx2_lane x l
         avx2_lane (mm256_set1_epi64x (cast c <: i64)) l = cast_mod (cast c <: i64) <: u64
       The second cast_mod (i64 -> u64) of (cast c : i64) round-trips back to c. *)
    lemma_avx2_lane_unfold a l;
    lemma_avx2_lane_unfold
      (Libcrux_sha3.Traits.f_xor_constant
        #I.t_Vec256 #(mk_usize 4)
        #FStar.Tactics.Typeclasses.solve a c) l
#pop-options

let avx2_lc_xor (a b: I.t_Vec256) (l: nat{l < 4})
  : Lemma (avx2_lane (Libcrux_sha3.Traits.f_xor
             #I.t_Vec256 #(mk_usize 4)
             #FStar.Tactics.Typeclasses.solve a b) l ==
           avx2_lane a l ^. avx2_lane b l)
  = lemma_avx2_lane_unfold a l;
    lemma_avx2_lane_unfold b l;
    lemma_avx2_lane_unfold
      (Libcrux_sha3.Traits.f_xor
        #I.t_Vec256 #(mk_usize 4)
        #FStar.Tactics.Typeclasses.solve a b) l

(* ================================================================
   Assemble the [lane_correctness] record
   ================================================================ *)

let lc_avx2 : G.lane_correctness (mk_usize 4) #I.t_Vec256 =
  {
    lane = avx2_lane;
    lc_zero = avx2_lc_zero;
    lc_xor5 = avx2_lc_xor5;
    lc_rotate_left1_and_xor = avx2_lc_rotate_left1_and_xor;
    lc_xor_and_rotate = avx2_lc_xor_and_rotate;
    lc_and_not_xor = avx2_lc_and_not_xor;
    lc_xor_constant = avx2_lc_xor_constant;
    lc_xor = avx2_lc_xor;
  }

(* ================================================================
   MAIN THEOREM: extract_lane commutes with keccakf1600 on AVX2.

   Direct specialization of [lemma_keccakf1600_to_spec], via the seven
   lane-correctness primitives above (which rest on the AVX2 intrinsic
   SMTPats in [Libcrux_intrinsics.Avx2_extract] — external trust).
   ================================================================ *)

let lemma_keccakf1600_avx2
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
      (l: nat{l < 4})
  : Lemma
      (G.extract_lane (mk_usize 4) lc_avx2
         (Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4)
            #I.t_Vec256 ks)
           .Libcrux_sha3.Generic_keccak.f_st l
       ==
       Hacspec_sha3.Keccak_f.keccak_f
         (G.extract_lane (mk_usize 4) lc_avx2
            ks.Libcrux_sha3.Generic_keccak.f_st l))
  = G.lemma_keccakf1600_to_spec (mk_usize 4) lc_avx2 ks l


(* ================================================================
   Initial-state lane extraction: at the zero SIMD state produced by
   [impl_2__new], extracting any lane yields the scalar all-zero
   state [repeat 0u64 25].
   ================================================================ *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_extract_lane_zero_avx2 (l: nat{l < 4})
  : Lemma
      (G.extract_lane (mk_usize 4) lc_avx2
         (Rust_primitives.Hax.repeat
           (Libcrux_sha3.Traits.f_zero
             #I.t_Vec256 #(mk_usize 4)
             #FStar.Tactics.Typeclasses.solve ()) (mk_usize 25)) l
       ==
       Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25))
  = let zeros_simd : t_Array I.t_Vec256 (mk_usize 25) =
      Rust_primitives.Hax.repeat
        (Libcrux_sha3.Traits.f_zero
          #I.t_Vec256 #(mk_usize 4)
          #FStar.Tactics.Typeclasses.solve ()) (mk_usize 25) in
    let zeros_u64 : t_Array u64 (mk_usize 25) =
      Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
    let lhs = G.extract_lane (mk_usize 4) lc_avx2 zeros_simd l in
    let aux (i: nat{i < 25}) : Lemma (Seq.index lhs i == Seq.index zeros_u64 i) =
      let ii : usize = mk_usize i in
      assert (lhs.[ii] == lc_avx2.G.lane zeros_simd.[ii] l);
      assert (zeros_simd.[ii] ==
                Libcrux_sha3.Traits.f_zero
                  #I.t_Vec256 #(mk_usize 4)
                  #FStar.Tactics.Typeclasses.solve ());
      avx2_lc_zero l;
      lemma_avx2_lane_unfold zeros_simd.[ii] l
    in
    FStar.Classical.forall_intro aux;
    Seq.lemma_eq_intro (lhs <: Seq.seq u64) (zeros_u64 <: Seq.seq u64)
#pop-options

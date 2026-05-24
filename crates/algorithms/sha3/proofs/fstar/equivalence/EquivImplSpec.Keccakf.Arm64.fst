module EquivImplSpec.Keccakf.Arm64

(* ================================================================
   NEON (Arm64, N=2, v_T=t_e_uint64x2_t) instantiation of the generic
   keccak_f equivalence proof.

   This module constructs the 7-field [lane_correctness] record for
   [Libcrux_sha3.Simd.Arm64.impl] using the per-lane postconditions
   declared on the 7 Keccak-critical intrinsics in
   [Libcrux_intrinsics.Arm64_extract]:

     e_vdupq_n_u64, e_veorq_u64, e_veor3q_u64, e_vrax1q_u64,
     e_vxarq_u64, e_vbcaxq_u64

   Derived theorem:
     extract_lane lc_arm64 (keccakf1600 ks).f_st l
       == keccak_f (extract_lane lc_arm64 ks.f_st l)
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"

open FStar.Mul
open Core_models

module G = EquivImplSpec.Keccakf.Generic
module I = Libcrux_intrinsics.Arm64_extract

(* Bring the Arm64 typeclass instance into scope so
   t_KeccakItem t_e_uint64x2_t (mk_usize 2) resolves to
   [Libcrux_sha3.Simd.Arm64.impl]. *)
let _ =
  let open Libcrux_intrinsics.Arm64_extract in
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Arm64 in
  ()

(* ================================================================
   Arm64 lane extraction

   For N=2, a [t_e_uint64x2_t] has two u64 lanes; the [lane] field
   of [lane_correctness] is simply [get_lane_u64x2].
   ================================================================ *)

[@@ "opaque_to_smt"]
let arm64_lane (v: I.t_e_uint64x2_t) (l: nat{l < 2}) : u64 =
  I.get_lane_u64x2 v l

(* Reveal helper for callers that need to unfold [arm64_lane] under SMT. *)
let lemma_arm64_lane_unfold (v: I.t_e_uint64x2_t) (l: nat{l < 2})
  : Lemma (arm64_lane v l == I.get_lane_u64x2 v l)
  = reveal_opaque (`%arm64_lane) (arm64_lane v l)

(* ================================================================
   Lane-correctness field proofs

   Each proof is a short unfolding:
     1. typeclass resolution expands [f_*] to the concrete [e_*]
        helper in [Libcrux_sha3.Simd.Arm64],
     2. that helper expands to the intrinsic in [I],
     3. the intrinsic's [ensures] clause (forall-over-lane) supplies
        the per-lane equation.
   ================================================================ *)

let arm64_lc_zero (l: nat{l < 2})
  : Lemma (arm64_lane (Libcrux_sha3.Traits.f_zero
             #I.t_e_uint64x2_t #(mk_usize 2)
             #FStar.Tactics.Typeclasses.solve ()) l == mk_u64 0)
  = reveal_opaque (`%arm64_lane)
      (arm64_lane (Libcrux_sha3.Traits.f_zero
                     #I.t_e_uint64x2_t #(mk_usize 2)
                     #FStar.Tactics.Typeclasses.solve ()) l)

let arm64_lc_xor5 (a b c d e: I.t_e_uint64x2_t) (l: nat{l < 2})
  : Lemma (arm64_lane (Libcrux_sha3.Traits.f_xor5
             #I.t_e_uint64x2_t #(mk_usize 2)
             #FStar.Tactics.Typeclasses.solve a b c d e) l ==
           (((arm64_lane a l ^. arm64_lane b l) ^. arm64_lane c l)
             ^. arm64_lane d l) ^. arm64_lane e l)
  = lemma_arm64_lane_unfold a l;
    lemma_arm64_lane_unfold b l;
    lemma_arm64_lane_unfold c l;
    lemma_arm64_lane_unfold d l;
    lemma_arm64_lane_unfold e l;
    lemma_arm64_lane_unfold
      (Libcrux_sha3.Traits.f_xor5
        #I.t_e_uint64x2_t #(mk_usize 2)
        #FStar.Tactics.Typeclasses.solve a b c d e) l

let arm64_lc_rotate_left1_and_xor (a b: I.t_e_uint64x2_t) (l: nat{l < 2})
  : Lemma (arm64_lane (Libcrux_sha3.Traits.f_rotate_left1_and_xor
             #I.t_e_uint64x2_t #(mk_usize 2)
             #FStar.Tactics.Typeclasses.solve a b) l ==
           arm64_lane a l ^.
           Core_models.Num.impl_u64__rotate_left (arm64_lane b l) (mk_u32 1))
  = lemma_arm64_lane_unfold a l;
    lemma_arm64_lane_unfold b l;
    lemma_arm64_lane_unfold
      (Libcrux_sha3.Traits.f_rotate_left1_and_xor
        #I.t_e_uint64x2_t #(mk_usize 2)
        #FStar.Tactics.Typeclasses.solve a b) l

let arm64_lc_xor_and_rotate (v_LEFT v_RIGHT: i32) (a b: I.t_e_uint64x2_t) (l: nat{l < 2})
  : Lemma
      (requires
        ((Rust_primitives.Hax.Int.from_machine v_LEFT <: Hax_lib.Int.t_Int) +
         (Rust_primitives.Hax.Int.from_machine v_RIGHT <: Hax_lib.Int.t_Int)) =
        (Rust_primitives.Hax.Int.from_machine (mk_i32 64) <: Hax_lib.Int.t_Int) /\
        v_RIGHT >. mk_i32 0 /\
        v_RIGHT <. mk_i32 64)
      (ensures
        arm64_lane (Libcrux_sha3.Traits.f_xor_and_rotate
          #I.t_e_uint64x2_t #(mk_usize 2)
          #FStar.Tactics.Typeclasses.solve v_LEFT v_RIGHT a b) l ==
        Core_models.Num.impl_u64__rotate_left
          (arm64_lane a l ^. arm64_lane b l) (cast (v_LEFT <: i32) <: u32))
  = lemma_arm64_lane_unfold a l;
    lemma_arm64_lane_unfold b l;
    lemma_arm64_lane_unfold
      (Libcrux_sha3.Traits.f_xor_and_rotate
        #I.t_e_uint64x2_t #(mk_usize 2)
        #FStar.Tactics.Typeclasses.solve v_LEFT v_RIGHT a b) l

let arm64_lc_and_not_xor (a b c: I.t_e_uint64x2_t) (l: nat{l < 2})
  : Lemma (arm64_lane (Libcrux_sha3.Traits.f_and_not_xor
             #I.t_e_uint64x2_t #(mk_usize 2)
             #FStar.Tactics.Typeclasses.solve a b c) l ==
           arm64_lane a l ^. (arm64_lane b l &. (~. (arm64_lane c l))))
  = lemma_arm64_lane_unfold a l;
    lemma_arm64_lane_unfold b l;
    lemma_arm64_lane_unfold c l;
    lemma_arm64_lane_unfold
      (Libcrux_sha3.Traits.f_and_not_xor
        #I.t_e_uint64x2_t #(mk_usize 2)
        #FStar.Tactics.Typeclasses.solve a b c) l

let arm64_lc_xor_constant (a: I.t_e_uint64x2_t) (c: u64) (l: nat{l < 2})
  : Lemma (arm64_lane (Libcrux_sha3.Traits.f_xor_constant
             #I.t_e_uint64x2_t #(mk_usize 2)
             #FStar.Tactics.Typeclasses.solve a c) l ==
           arm64_lane a l ^. c)
  = lemma_arm64_lane_unfold a l;
    lemma_arm64_lane_unfold
      (Libcrux_sha3.Traits.f_xor_constant
        #I.t_e_uint64x2_t #(mk_usize 2)
        #FStar.Tactics.Typeclasses.solve a c) l

let arm64_lc_xor (a b: I.t_e_uint64x2_t) (l: nat{l < 2})
  : Lemma (arm64_lane (Libcrux_sha3.Traits.f_xor
             #I.t_e_uint64x2_t #(mk_usize 2)
             #FStar.Tactics.Typeclasses.solve a b) l ==
           arm64_lane a l ^. arm64_lane b l)
  = lemma_arm64_lane_unfold a l;
    lemma_arm64_lane_unfold b l;
    lemma_arm64_lane_unfold
      (Libcrux_sha3.Traits.f_xor
        #I.t_e_uint64x2_t #(mk_usize 2)
        #FStar.Tactics.Typeclasses.solve a b) l

(* ================================================================
   Assemble the [lane_correctness] record
   ================================================================ *)

let lc_arm64 : G.lane_correctness (mk_usize 2) #I.t_e_uint64x2_t =
  {
    lane = arm64_lane;
    lc_zero = arm64_lc_zero;
    lc_xor5 = arm64_lc_xor5;
    lc_rotate_left1_and_xor = arm64_lc_rotate_left1_and_xor;
    lc_xor_and_rotate = arm64_lc_xor_and_rotate;
    lc_and_not_xor = arm64_lc_and_not_xor;
    lc_xor_constant = arm64_lc_xor_constant;
    lc_xor = arm64_lc_xor;
  }

(* ================================================================
   MAIN THEOREM: extract_lane commutes with keccakf1600 on Arm64.

   Direct specialization of [lemma_keccakf1600_to_spec].
   ================================================================ *)

let lemma_keccakf1600_arm64
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
      (l: nat{l < 2})
  : Lemma
      (G.extract_lane (mk_usize 2) lc_arm64
         (Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 2)
            #I.t_e_uint64x2_t ks)
           .Libcrux_sha3.Generic_keccak.f_st l
       ==
       Hacspec_sha3.Keccak_f.keccak_f
         (G.extract_lane (mk_usize 2) lc_arm64
            ks.Libcrux_sha3.Generic_keccak.f_st l))
  = G.lemma_keccakf1600_to_spec (mk_usize 2) lc_arm64 ks l


(* ================================================================
   Initial-state lane extraction: at the zero SIMD state produced by
   [impl_2__new], extracting any lane yields the scalar all-zero
   state [repeat 0u64 25].  Used by the Arm64 absorb2 inline proof
   to establish the loop invariant at i=0.
   ================================================================ *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_extract_lane_zero_arm64 (l: nat{l < 2})
  : Lemma
      (G.extract_lane (mk_usize 2) lc_arm64
         (Rust_primitives.Hax.repeat
           (Libcrux_sha3.Traits.f_zero
             #I.t_e_uint64x2_t #(mk_usize 2)
             #FStar.Tactics.Typeclasses.solve ()) (mk_usize 25)) l
       ==
       Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25))
  = let zeros_simd : t_Array I.t_e_uint64x2_t (mk_usize 25) =
      Rust_primitives.Hax.repeat
        (Libcrux_sha3.Traits.f_zero
          #I.t_e_uint64x2_t #(mk_usize 2)
          #FStar.Tactics.Typeclasses.solve ()) (mk_usize 25) in
    let zeros_u64 : t_Array u64 (mk_usize 25) =
      Rust_primitives.Hax.repeat (mk_u64 0) (mk_usize 25) in
    let lhs = G.extract_lane (mk_usize 2) lc_arm64 zeros_simd l in
    let aux (i: nat{i < 25}) : Lemma (Seq.index lhs i == Seq.index zeros_u64 i) =
      let ii : usize = mk_usize i in
      (* Trigger lemma_extract_lane_index SMTPat: lhs.[ii] == lc.lane zeros_simd.[ii] l *)
      assert (lhs.[ii] == lc_arm64.G.lane zeros_simd.[ii] l);
      (* zeros_simd.[ii] == f_zero ()  (repeat element access) *)
      assert (zeros_simd.[ii] ==
                Libcrux_sha3.Traits.f_zero
                  #I.t_e_uint64x2_t #(mk_usize 2)
                  #FStar.Tactics.Typeclasses.solve ());
      (* arm64_lane (f_zero ()) l == 0u64 *)
      arm64_lc_zero l;
      lemma_arm64_lane_unfold zeros_simd.[ii] l
    in
    FStar.Classical.forall_intro aux;
    Seq.lemma_eq_intro (lhs <: Seq.seq u64) (zeros_u64 <: Seq.seq u64)
#pop-options

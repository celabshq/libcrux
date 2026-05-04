module EquivImplSpec.Keccakf.Generic

(* ================================================================
   Generic keccak_f equivalence via lane-wise `to_spec` commutativity.

   MAIN THEOREM (lemma_keccakf1600_to_spec):

     extract_lane lc (keccakf1600 v_N #v_T ks).f_st l ==
       keccak_f (extract_lane lc ks.f_st l)

   i.e., extracting any lane from the SIMD keccakf1600 result equals
   running the scalar spec keccak_f on that lane of the input.

   ARCHITECTURE:

   1. `lane_correctness` record — 7 hypotheses any KeccakItem backend
      must satisfy (lc_xor5, lc_rotate_left1_and_xor, lc_xor_and_rotate,
      lc_and_not_xor, lc_xor_constant, lc_xor, lc_zero). These say
      that extracting a lane commutes with each typeclass operation.

   2. `extract_lane lc state l` — maps `t_Array v_T 25` to
      `t_Array u64 25` by applying `lc.lane` pointwise:
        (extract_lane state l).[i] = lc.lane state.[i] l

   3. Per-step commutativity lemmas — for each keccak step (theta+rho,
      pi, chi, iota), prove:
        extract_lane (impl_step state) l == spec_step (extract_lane state l)

   4. Composition — chain per-step commutativity into one-round, then
      induction over 24 rounds.

   PROOF STATUS:

   Proven (= ()):
   - All generic impl-side lemmas (Phase 1): lemma_theta_generic,
     lemma_rho_{0..4}_generic, lemma_pi_{0..4}_generic,
     lemma_rho_unfold_generic, lemma_pi_unfold_generic.
     These express impl results in terms of abstract KeccakItem ops.
   - Operation-level lane commutativity: lane_xor5, lane_xor, etc.
     (trivial wrappers around lane_correctness fields)
   - One-round and multi-round composition (assuming per-step lemmas)

   Admitted (library-level, same as portable proof):
   - [Proof_Utils.Lemmas.lemma_rotate_left_zero]: rotate_left(x, 0) == x
   - [Proof_Utils.Lemmas.logand_commutative]: (a &. b) == (b &. a)
   - lemma_rho_offsets_values: RHO_OFFSETS array element values
   - lemma_keccakf1600_is_rounds: fold_range bridge (impl side)
   - lemma_keccak_f_is_rounds: fold_range bridge (spec side)

   PROOF STRATEGY for the to_spec admits:

   Each to_spec lemma follows the same pattern:
   1. Use the generic impl-side lemma to know what each slot contains
      (e.g., lemma_rho_0_generic says r.[1] == f_xor_and_rotate ... s.[1] d.[0])
   2. Apply lane_* helpers to convert from abstract v_T ops to scalar u64 ops
      (e.g., lane_xor_and_rotate says lc.lane(f_xor_and_rotate ... a b) l == rotl(lane a l ^. lane b l, LEFT))
   3. Use spec-side reduction (e.g., lemma_rho_theta_spec from the portable proof)
   4. Conclude with Rust_primitives.Arrays.eq_intro

   The tricky part is getting Z3 to see pointwise equality across all 25
   indices. The portable proof handles this with explicit per-index asserts;
   the generic proof needs the same but with lane extraction in between.

   INSTANTIATION (future files):
   - Portable (N=1, u64): lane(x,0) = x. All lc_* are `= ()`.
   - NEON (N=2, uint64x2_t): lane = get_lane64, lc_* from arm64_extract.rs specs
   - AVX2 (N=4, Vec256): lane = get_lane, lc_* from avx2_extract.rs specs
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models
open Proof_Utils.NatFold   (* fold_range_nat, lemma_fold_range_is_range_nat *)
module Lemmas = Proof_Utils.Lemmas
module ChiFold = EquivImplSpec.Keccakf.ChiFold
module SpecRounds = EquivImplSpec.Keccakf.SpecRounds

let _ =
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Portable in
  ()

(* ================================================================
   Lane-correctness specification
   ================================================================ *)

noeq type lane_correctness
  (v_N: usize)
  (#v_T: Type0)
  {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |} = {

  lane: v_T -> l:nat{l < v v_N} -> u64;

  lc_zero: (l:nat{l < v v_N}) ->
    Lemma (lane (Libcrux_sha3.Traits.f_zero #v_T #v_N #inst ()) l == mk_u64 0);

  lc_xor5: (a:v_T) -> (b:v_T) -> (c:v_T) -> (d:v_T) -> (e:v_T) -> (l:nat{l < v v_N}) ->
    Lemma (lane (Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst a b c d e) l ==
           (((lane a l ^. lane b l) ^. lane c l) ^. lane d l) ^. lane e l);

  lc_rotate_left1_and_xor: (a:v_T) -> (b:v_T) -> (l:nat{l < v v_N}) ->
    Lemma (lane (Libcrux_sha3.Traits.f_rotate_left1_and_xor #v_T #v_N #inst a b) l ==
           lane a l ^. Core_models.Num.impl_u64__rotate_left (lane b l) (mk_u32 1));

  lc_xor_and_rotate: (v_LEFT:i32) -> (v_RIGHT:i32) -> (a:v_T) -> (b:v_T) -> (l:nat{l < v v_N}) ->
    Lemma
      (requires
        ((Rust_primitives.Hax.Int.from_machine v_LEFT <: Hax_lib.Int.t_Int) +
         (Rust_primitives.Hax.Int.from_machine v_RIGHT <: Hax_lib.Int.t_Int)) =
        (Rust_primitives.Hax.Int.from_machine (mk_i32 64) <: Hax_lib.Int.t_Int) /\
        v_RIGHT >. mk_i32 0 /\
        v_RIGHT <. mk_i32 64)
      (ensures
        lane (Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst v_LEFT v_RIGHT a b) l ==
        Core_models.Num.impl_u64__rotate_left (lane a l ^. lane b l) (cast (v_LEFT <: i32) <: u32));

  lc_and_not_xor: (a:v_T) -> (b:v_T) -> (c:v_T) -> (l:nat{l < v v_N}) ->
    Lemma (lane (Libcrux_sha3.Traits.f_and_not_xor #v_T #v_N #inst a b c) l ==
           lane a l ^. (lane b l &. (~. (lane c l))));

  lc_xor_constant: (a:v_T) -> (c:u64) -> (l:nat{l < v v_N}) ->
    Lemma (lane (Libcrux_sha3.Traits.f_xor_constant #v_T #v_N #inst a c) l ==
           lane a l ^. c);

  lc_xor: (a:v_T) -> (b:v_T) -> (l:nat{l < v v_N}) ->
    Lemma (lane (Libcrux_sha3.Traits.f_xor #v_T #v_N #inst a b) l ==
           lane a l ^. lane b l);
}

(* ================================================================
   extract_lane: maps SIMD state to a scalar spec state for lane l
   ================================================================ *)

[@ "opaque_to_smt"]
let extract_lane
      (#v_T: Type0) (v_N: usize)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (state: t_Array v_T (mk_usize 25))
      (l: nat{l < v v_N})
  : t_Array u64 (mk_usize 25) =
  Rust_primitives.Arrays.createi (mk_usize 25) (fun i -> lc.lane state.[i] l)

let lemma_extract_lane_index
      (#v_T: Type0) (v_N: usize)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (state: t_Array v_T (mk_usize 25))
      (l: nat{l < v v_N})
      (i: usize{i <. mk_usize 25})
  : Lemma
      ((extract_lane v_N lc state l).[i] == lc.lane state.[i] l)
      [SMTPat ((extract_lane v_N lc state l).[i])]
  = assert_norm ((extract_lane v_N lc state l).[i] == lc.lane state.[i] l)

(* Shorthand for the spec's state type *)
let spec_state = SpecRounds.spec_state

(* Shorthand for the impl's state type *)
let impl_state (v_N: usize) (v_T: Type0)
  {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |} =
  Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T

(* ================================================================
   Operation-level commutativity with extract_lane

   These show that applying a KeccakItem operation to SIMD elements
   and then extracting a lane equals extracting lanes first and then
   applying the scalar operation. Each is a direct consequence of the
   corresponding lane_correctness hypothesis.
   ================================================================ *)

let lane_xor5
      (#v_T: Type0) (v_N: usize)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (a b c d e: v_T) (l: nat{l < v v_N})
  : Lemma (lc.lane (Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst a b c d e) l ==
           (((lc.lane a l ^. lc.lane b l) ^. lc.lane c l) ^. lc.lane d l) ^. lc.lane e l)
  = lc.lc_xor5 a b c d e l

let lane_rotate_left1_and_xor
      (#v_T: Type0) (v_N: usize)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (a b: v_T) (l: nat{l < v v_N})
  : Lemma (lc.lane (Libcrux_sha3.Traits.f_rotate_left1_and_xor #v_T #v_N #inst a b) l ==
           lc.lane a l ^. Core_models.Num.impl_u64__rotate_left (lc.lane b l) (mk_u32 1))
  = lc.lc_rotate_left1_and_xor a b l

let lane_xor_and_rotate
      (#v_T: Type0) (v_N: usize)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (v_LEFT v_RIGHT: i32) (a b: v_T) (l: nat{l < v v_N})
  : Lemma
      (requires
        ((Rust_primitives.Hax.Int.from_machine v_LEFT <: Hax_lib.Int.t_Int) +
         (Rust_primitives.Hax.Int.from_machine v_RIGHT <: Hax_lib.Int.t_Int)) =
        (Rust_primitives.Hax.Int.from_machine (mk_i32 64) <: Hax_lib.Int.t_Int) /\
        v_RIGHT >. mk_i32 0 /\
        v_RIGHT <. mk_i32 64)
      (ensures
        lc.lane (Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst v_LEFT v_RIGHT a b) l ==
        Core_models.Num.impl_u64__rotate_left (lc.lane a l ^. lc.lane b l) (cast (v_LEFT <: i32) <: u32))
  = lc.lc_xor_and_rotate v_LEFT v_RIGHT a b l

let lane_and_not_xor
      (#v_T: Type0) (v_N: usize)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (a b c: v_T) (l: nat{l < v v_N})
  : Lemma (lc.lane (Libcrux_sha3.Traits.f_and_not_xor #v_T #v_N #inst a b c) l ==
           lc.lane a l ^. (lc.lane b l &. (~. (lc.lane c l))))
  = lc.lc_and_not_xor a b c l

let lane_xor_constant
      (#v_T: Type0) (v_N: usize)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (a: v_T) (c: u64) (l: nat{l < v v_N})
  : Lemma (lc.lane (Libcrux_sha3.Traits.f_xor_constant #v_T #v_N #inst a c) l ==
           lc.lane a l ^. c)
  = lc.lc_xor_constant a c l

let lane_xor
      (#v_T: Type0) (v_N: usize)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (a b: v_T) (l: nat{l < v v_N})
  : Lemma (lc.lane (Libcrux_sha3.Traits.f_xor #v_T #v_N #inst a b) l ==
           lc.lane a l ^. lc.lane b l)
  = lc.lc_xor a b l

(* ================================================================
   Phase 1: Generic impl-side rho lemmas (abstract v_T)

   These capture what each array slot of the rho result contains,
   expressed in terms of abstract typeclass operations.
   They should be `= ()` because they depend only on array update
   semantics, not on the concrete type v_T.
   ================================================================ *)

(** Abbreviation for rotate_left with i32 cast. *)
let rotl (x: u64) (n: i32) : u64 =
  Core_models.Num.impl_u64__rotate_left x (cast (n <: i32) <: u32)

(** Theta: state is unchanged, d matches column parities.
    Under the FIPS-native layout [get_ij(arr, i, j) = arr[5*i + j]] with
    impl [(i, j) = (y, x)], column [x] corresponds to flat indices
    [x, x+5, x+10, x+15, x+20] (stride 5). *)
#push-options "--z3rlimit 100"
let lemma_theta_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let ks', d = Libcrux_sha3.Generic_keccak.impl_2__theta v_N #v_T ks in
       ks'.Libcrux_sha3.Generic_keccak.f_st == s /\
       d.[mk_usize 0] == Libcrux_sha3.Traits.f_rotate_left1_and_xor #v_T #v_N #inst
         (Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst s.[mk_usize 4] s.[mk_usize 9] s.[mk_usize 14] s.[mk_usize 19] s.[mk_usize 24])
         (Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst s.[mk_usize 1] s.[mk_usize 6] s.[mk_usize 11] s.[mk_usize 16] s.[mk_usize 21]) /\
       d.[mk_usize 1] == Libcrux_sha3.Traits.f_rotate_left1_and_xor #v_T #v_N #inst
         (Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst s.[mk_usize 0] s.[mk_usize 5] s.[mk_usize 10] s.[mk_usize 15] s.[mk_usize 20])
         (Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst s.[mk_usize 2] s.[mk_usize 7] s.[mk_usize 12] s.[mk_usize 17] s.[mk_usize 22]) /\
       d.[mk_usize 2] == Libcrux_sha3.Traits.f_rotate_left1_and_xor #v_T #v_N #inst
         (Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst s.[mk_usize 1] s.[mk_usize 6] s.[mk_usize 11] s.[mk_usize 16] s.[mk_usize 21])
         (Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst s.[mk_usize 3] s.[mk_usize 8] s.[mk_usize 13] s.[mk_usize 18] s.[mk_usize 23]) /\
       d.[mk_usize 3] == Libcrux_sha3.Traits.f_rotate_left1_and_xor #v_T #v_N #inst
         (Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst s.[mk_usize 2] s.[mk_usize 7] s.[mk_usize 12] s.[mk_usize 17] s.[mk_usize 22])
         (Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst s.[mk_usize 4] s.[mk_usize 9] s.[mk_usize 14] s.[mk_usize 19] s.[mk_usize 24]) /\
       d.[mk_usize 4] == Libcrux_sha3.Traits.f_rotate_left1_and_xor #v_T #v_N #inst
         (Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst s.[mk_usize 3] s.[mk_usize 8] s.[mk_usize 13] s.[mk_usize 18] s.[mk_usize 23])
         (Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst s.[mk_usize 0] s.[mk_usize 5] s.[mk_usize 10] s.[mk_usize 15] s.[mk_usize 20]))
  = ()
#pop-options

(** rho_0_: under FIPS-native layout, updates cells where [x=0]
    (flat indices [0, 5, 10, 15, 20]); preserves the rest.
    The [y=0, x=0] cell uses f_xor; the other four use f_xor_and_rotate. *)
#push-options "--z3rlimit 200"
let lemma_rho_0_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let r = (Libcrux_sha3.Generic_keccak.impl_2__rho_0_ v_N #v_T ks d)
                .Libcrux_sha3.Generic_keccak.f_st in
       r.[mk_usize 0] == Libcrux_sha3.Traits.f_xor #v_T #v_N #inst s.[mk_usize 0] d.[mk_usize 0] /\
       r.[mk_usize 5] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 36) (mk_i32 28) s.[mk_usize 5] d.[mk_usize 0] /\
       r.[mk_usize 10] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 3) (mk_i32 61) s.[mk_usize 10] d.[mk_usize 0] /\
       r.[mk_usize 15] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 41) (mk_i32 23) s.[mk_usize 15] d.[mk_usize 0] /\
       r.[mk_usize 20] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 18) (mk_i32 46) s.[mk_usize 20] d.[mk_usize 0] /\
       r.[mk_usize 1] == s.[mk_usize 1] /\ r.[mk_usize 2] == s.[mk_usize 2] /\
       r.[mk_usize 3] == s.[mk_usize 3] /\ r.[mk_usize 4] == s.[mk_usize 4] /\
       r.[mk_usize 6] == s.[mk_usize 6] /\ r.[mk_usize 7] == s.[mk_usize 7] /\
       r.[mk_usize 8] == s.[mk_usize 8] /\ r.[mk_usize 9] == s.[mk_usize 9] /\
       r.[mk_usize 11] == s.[mk_usize 11] /\ r.[mk_usize 12] == s.[mk_usize 12] /\
       r.[mk_usize 13] == s.[mk_usize 13] /\ r.[mk_usize 14] == s.[mk_usize 14] /\
       r.[mk_usize 16] == s.[mk_usize 16] /\ r.[mk_usize 17] == s.[mk_usize 17] /\
       r.[mk_usize 18] == s.[mk_usize 18] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 21] == s.[mk_usize 21] /\ r.[mk_usize 22] == s.[mk_usize 22] /\
       r.[mk_usize 23] == s.[mk_usize 23] /\ r.[mk_usize 24] == s.[mk_usize 24])
  = ()
#pop-options

(** rho_1_: updates cells where [x=1] (flat [1, 6, 11, 16, 21]). *)
#push-options "--z3rlimit 200"
let lemma_rho_1_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let r = (Libcrux_sha3.Generic_keccak.impl_2__rho_1_ v_N #v_T ks d)
                .Libcrux_sha3.Generic_keccak.f_st in
       r.[mk_usize 1] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 1) (mk_i32 63) s.[mk_usize 1] d.[mk_usize 1] /\
       r.[mk_usize 6] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 44) (mk_i32 20) s.[mk_usize 6] d.[mk_usize 1] /\
       r.[mk_usize 11] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 10) (mk_i32 54) s.[mk_usize 11] d.[mk_usize 1] /\
       r.[mk_usize 16] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 45) (mk_i32 19) s.[mk_usize 16] d.[mk_usize 1] /\
       r.[mk_usize 21] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 2) (mk_i32 62) s.[mk_usize 21] d.[mk_usize 1] /\
       r.[mk_usize 0] == s.[mk_usize 0] /\ r.[mk_usize 2] == s.[mk_usize 2] /\
       r.[mk_usize 3] == s.[mk_usize 3] /\ r.[mk_usize 4] == s.[mk_usize 4] /\
       r.[mk_usize 5] == s.[mk_usize 5] /\ r.[mk_usize 7] == s.[mk_usize 7] /\
       r.[mk_usize 8] == s.[mk_usize 8] /\ r.[mk_usize 9] == s.[mk_usize 9] /\
       r.[mk_usize 10] == s.[mk_usize 10] /\ r.[mk_usize 12] == s.[mk_usize 12] /\
       r.[mk_usize 13] == s.[mk_usize 13] /\ r.[mk_usize 14] == s.[mk_usize 14] /\
       r.[mk_usize 15] == s.[mk_usize 15] /\ r.[mk_usize 17] == s.[mk_usize 17] /\
       r.[mk_usize 18] == s.[mk_usize 18] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 20] == s.[mk_usize 20] /\ r.[mk_usize 22] == s.[mk_usize 22] /\
       r.[mk_usize 23] == s.[mk_usize 23] /\ r.[mk_usize 24] == s.[mk_usize 24])
  = ()
#pop-options

(** rho_2_: updates cells where [x=2] (flat [2, 7, 12, 17, 22]). *)
#push-options "--z3rlimit 200"
let lemma_rho_2_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let r = (Libcrux_sha3.Generic_keccak.impl_2__rho_2_ v_N #v_T ks d)
                .Libcrux_sha3.Generic_keccak.f_st in
       r.[mk_usize 2] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 62) (mk_i32 2) s.[mk_usize 2] d.[mk_usize 2] /\
       r.[mk_usize 7] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 6) (mk_i32 58) s.[mk_usize 7] d.[mk_usize 2] /\
       r.[mk_usize 12] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 43) (mk_i32 21) s.[mk_usize 12] d.[mk_usize 2] /\
       r.[mk_usize 17] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 15) (mk_i32 49) s.[mk_usize 17] d.[mk_usize 2] /\
       r.[mk_usize 22] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 61) (mk_i32 3) s.[mk_usize 22] d.[mk_usize 2] /\
       r.[mk_usize 0] == s.[mk_usize 0] /\ r.[mk_usize 1] == s.[mk_usize 1] /\
       r.[mk_usize 3] == s.[mk_usize 3] /\ r.[mk_usize 4] == s.[mk_usize 4] /\
       r.[mk_usize 5] == s.[mk_usize 5] /\ r.[mk_usize 6] == s.[mk_usize 6] /\
       r.[mk_usize 8] == s.[mk_usize 8] /\ r.[mk_usize 9] == s.[mk_usize 9] /\
       r.[mk_usize 10] == s.[mk_usize 10] /\ r.[mk_usize 11] == s.[mk_usize 11] /\
       r.[mk_usize 13] == s.[mk_usize 13] /\ r.[mk_usize 14] == s.[mk_usize 14] /\
       r.[mk_usize 15] == s.[mk_usize 15] /\ r.[mk_usize 16] == s.[mk_usize 16] /\
       r.[mk_usize 18] == s.[mk_usize 18] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 20] == s.[mk_usize 20] /\ r.[mk_usize 21] == s.[mk_usize 21] /\
       r.[mk_usize 23] == s.[mk_usize 23] /\ r.[mk_usize 24] == s.[mk_usize 24])
  = ()
#pop-options

(** rho_3_: updates cells where [x=3] (flat [3, 8, 13, 18, 23]). *)
#push-options "--z3rlimit 200"
let lemma_rho_3_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let r = (Libcrux_sha3.Generic_keccak.impl_2__rho_3_ v_N #v_T ks d)
                .Libcrux_sha3.Generic_keccak.f_st in
       r.[mk_usize 3] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 28) (mk_i32 36) s.[mk_usize 3] d.[mk_usize 3] /\
       r.[mk_usize 8] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 55) (mk_i32 9) s.[mk_usize 8] d.[mk_usize 3] /\
       r.[mk_usize 13] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 25) (mk_i32 39) s.[mk_usize 13] d.[mk_usize 3] /\
       r.[mk_usize 18] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 21) (mk_i32 43) s.[mk_usize 18] d.[mk_usize 3] /\
       r.[mk_usize 23] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 56) (mk_i32 8) s.[mk_usize 23] d.[mk_usize 3] /\
       r.[mk_usize 0] == s.[mk_usize 0] /\ r.[mk_usize 1] == s.[mk_usize 1] /\
       r.[mk_usize 2] == s.[mk_usize 2] /\ r.[mk_usize 4] == s.[mk_usize 4] /\
       r.[mk_usize 5] == s.[mk_usize 5] /\ r.[mk_usize 6] == s.[mk_usize 6] /\
       r.[mk_usize 7] == s.[mk_usize 7] /\ r.[mk_usize 9] == s.[mk_usize 9] /\
       r.[mk_usize 10] == s.[mk_usize 10] /\ r.[mk_usize 11] == s.[mk_usize 11] /\
       r.[mk_usize 12] == s.[mk_usize 12] /\ r.[mk_usize 14] == s.[mk_usize 14] /\
       r.[mk_usize 15] == s.[mk_usize 15] /\ r.[mk_usize 16] == s.[mk_usize 16] /\
       r.[mk_usize 17] == s.[mk_usize 17] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 20] == s.[mk_usize 20] /\ r.[mk_usize 21] == s.[mk_usize 21] /\
       r.[mk_usize 22] == s.[mk_usize 22] /\ r.[mk_usize 24] == s.[mk_usize 24])
  = ()
#pop-options

(** rho_4_: updates cells where [x=4] (flat [4, 9, 14, 19, 24]). *)
#push-options "--z3rlimit 200"
let lemma_rho_4_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let r = (Libcrux_sha3.Generic_keccak.impl_2__rho_4_ v_N #v_T ks d)
                .Libcrux_sha3.Generic_keccak.f_st in
       r.[mk_usize 4] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 27) (mk_i32 37) s.[mk_usize 4] d.[mk_usize 4] /\
       r.[mk_usize 9] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 20) (mk_i32 44) s.[mk_usize 9] d.[mk_usize 4] /\
       r.[mk_usize 14] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 39) (mk_i32 25) s.[mk_usize 14] d.[mk_usize 4] /\
       r.[mk_usize 19] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 8) (mk_i32 56) s.[mk_usize 19] d.[mk_usize 4] /\
       r.[mk_usize 24] == Libcrux_sha3.Traits.f_xor_and_rotate #v_T #v_N #inst (mk_i32 14) (mk_i32 50) s.[mk_usize 24] d.[mk_usize 4] /\
       r.[mk_usize 0] == s.[mk_usize 0] /\ r.[mk_usize 1] == s.[mk_usize 1] /\
       r.[mk_usize 2] == s.[mk_usize 2] /\ r.[mk_usize 3] == s.[mk_usize 3] /\
       r.[mk_usize 5] == s.[mk_usize 5] /\ r.[mk_usize 6] == s.[mk_usize 6] /\
       r.[mk_usize 7] == s.[mk_usize 7] /\ r.[mk_usize 8] == s.[mk_usize 8] /\
       r.[mk_usize 10] == s.[mk_usize 10] /\ r.[mk_usize 11] == s.[mk_usize 11] /\
       r.[mk_usize 12] == s.[mk_usize 12] /\ r.[mk_usize 13] == s.[mk_usize 13] /\
       r.[mk_usize 15] == s.[mk_usize 15] /\ r.[mk_usize 16] == s.[mk_usize 16] /\
       r.[mk_usize 17] == s.[mk_usize 17] /\ r.[mk_usize 18] == s.[mk_usize 18] /\
       r.[mk_usize 20] == s.[mk_usize 20] /\ r.[mk_usize 21] == s.[mk_usize 21] /\
       r.[mk_usize 22] == s.[mk_usize 22] /\ r.[mk_usize 23] == s.[mk_usize 23])
  = ()
#pop-options

(** rho unfolds to rho_0_ through rho_4_ chain. *)
#push-options "--z3rlimit 100"
let lemma_rho_unfold_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
  : Lemma
      (let open Libcrux_sha3.Generic_keccak in
       impl_2__rho v_N #v_T ks d ==
       (let ks0 = impl_2__rho_0_ v_N #v_T ks d in
        let ks1 = impl_2__rho_1_ v_N #v_T ks0 d in
        let ks2 = impl_2__rho_2_ v_N #v_T ks1 d in
        let ks3 = impl_2__rho_3_ v_N #v_T ks2 d in
        impl_2__rho_4_ v_N #v_T ks3 d))
  = ()
#pop-options

(* ================================================================
   Phase 1b: Generic impl-side pi lemmas (abstract v_T)
   ================================================================ *)

(** pi_0_: updates cells where [x=0] except [(0,0)] (flat [5, 10, 15, 20]). *)
#push-options "--z3rlimit 200"
let lemma_pi_0_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks old: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let o = old.Libcrux_sha3.Generic_keccak.f_st in
       let r = (Libcrux_sha3.Generic_keccak.impl_2__pi_0_ v_N #v_T ks old)
                .Libcrux_sha3.Generic_keccak.f_st in
       r.[mk_usize 5] == o.[mk_usize 3] /\
       r.[mk_usize 10] == o.[mk_usize 1] /\
       r.[mk_usize 15] == o.[mk_usize 4] /\
       r.[mk_usize 20] == o.[mk_usize 2] /\
       r.[mk_usize 0] == s.[mk_usize 0] /\ r.[mk_usize 1] == s.[mk_usize 1] /\
       r.[mk_usize 2] == s.[mk_usize 2] /\ r.[mk_usize 3] == s.[mk_usize 3] /\
       r.[mk_usize 4] == s.[mk_usize 4] /\ r.[mk_usize 6] == s.[mk_usize 6] /\
       r.[mk_usize 7] == s.[mk_usize 7] /\ r.[mk_usize 8] == s.[mk_usize 8] /\
       r.[mk_usize 9] == s.[mk_usize 9] /\ r.[mk_usize 11] == s.[mk_usize 11] /\
       r.[mk_usize 12] == s.[mk_usize 12] /\ r.[mk_usize 13] == s.[mk_usize 13] /\
       r.[mk_usize 14] == s.[mk_usize 14] /\ r.[mk_usize 16] == s.[mk_usize 16] /\
       r.[mk_usize 17] == s.[mk_usize 17] /\ r.[mk_usize 18] == s.[mk_usize 18] /\
       r.[mk_usize 19] == s.[mk_usize 19] /\ r.[mk_usize 21] == s.[mk_usize 21] /\
       r.[mk_usize 22] == s.[mk_usize 22] /\ r.[mk_usize 23] == s.[mk_usize 23] /\
       r.[mk_usize 24] == s.[mk_usize 24])
  = ()
#pop-options

(** pi_1_: updates cells where [x=1] (flat [1, 6, 11, 16, 21]). *)
#push-options "--z3rlimit 200"
let lemma_pi_1_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks old: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let o = old.Libcrux_sha3.Generic_keccak.f_st in
       let r = (Libcrux_sha3.Generic_keccak.impl_2__pi_1_ v_N #v_T ks old)
                .Libcrux_sha3.Generic_keccak.f_st in
       r.[mk_usize 1] == o.[mk_usize 6] /\
       r.[mk_usize 6] == o.[mk_usize 9] /\
       r.[mk_usize 11] == o.[mk_usize 7] /\
       r.[mk_usize 16] == o.[mk_usize 5] /\
       r.[mk_usize 21] == o.[mk_usize 8] /\
       r.[mk_usize 0] == s.[mk_usize 0] /\ r.[mk_usize 2] == s.[mk_usize 2] /\
       r.[mk_usize 3] == s.[mk_usize 3] /\ r.[mk_usize 4] == s.[mk_usize 4] /\
       r.[mk_usize 5] == s.[mk_usize 5] /\ r.[mk_usize 7] == s.[mk_usize 7] /\
       r.[mk_usize 8] == s.[mk_usize 8] /\ r.[mk_usize 9] == s.[mk_usize 9] /\
       r.[mk_usize 10] == s.[mk_usize 10] /\ r.[mk_usize 12] == s.[mk_usize 12] /\
       r.[mk_usize 13] == s.[mk_usize 13] /\ r.[mk_usize 14] == s.[mk_usize 14] /\
       r.[mk_usize 15] == s.[mk_usize 15] /\ r.[mk_usize 17] == s.[mk_usize 17] /\
       r.[mk_usize 18] == s.[mk_usize 18] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 20] == s.[mk_usize 20] /\ r.[mk_usize 22] == s.[mk_usize 22] /\
       r.[mk_usize 23] == s.[mk_usize 23] /\ r.[mk_usize 24] == s.[mk_usize 24])
  = ()
#pop-options

(** pi_2_: updates cells where [x=2] (flat [2, 7, 12, 17, 22]). *)
#push-options "--z3rlimit 200"
let lemma_pi_2_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks old: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let o = old.Libcrux_sha3.Generic_keccak.f_st in
       let r = (Libcrux_sha3.Generic_keccak.impl_2__pi_2_ v_N #v_T ks old)
                .Libcrux_sha3.Generic_keccak.f_st in
       r.[mk_usize 2] == o.[mk_usize 12] /\
       r.[mk_usize 7] == o.[mk_usize 10] /\
       r.[mk_usize 12] == o.[mk_usize 13] /\
       r.[mk_usize 17] == o.[mk_usize 11] /\
       r.[mk_usize 22] == o.[mk_usize 14] /\
       r.[mk_usize 0] == s.[mk_usize 0] /\ r.[mk_usize 1] == s.[mk_usize 1] /\
       r.[mk_usize 3] == s.[mk_usize 3] /\ r.[mk_usize 4] == s.[mk_usize 4] /\
       r.[mk_usize 5] == s.[mk_usize 5] /\ r.[mk_usize 6] == s.[mk_usize 6] /\
       r.[mk_usize 8] == s.[mk_usize 8] /\ r.[mk_usize 9] == s.[mk_usize 9] /\
       r.[mk_usize 10] == s.[mk_usize 10] /\ r.[mk_usize 11] == s.[mk_usize 11] /\
       r.[mk_usize 13] == s.[mk_usize 13] /\ r.[mk_usize 14] == s.[mk_usize 14] /\
       r.[mk_usize 15] == s.[mk_usize 15] /\ r.[mk_usize 16] == s.[mk_usize 16] /\
       r.[mk_usize 18] == s.[mk_usize 18] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 20] == s.[mk_usize 20] /\ r.[mk_usize 21] == s.[mk_usize 21] /\
       r.[mk_usize 23] == s.[mk_usize 23] /\ r.[mk_usize 24] == s.[mk_usize 24])
  = ()
#pop-options

(** pi_3_: updates cells where [x=3] (flat [3, 8, 13, 18, 23]). *)
#push-options "--z3rlimit 200"
let lemma_pi_3_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks old: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let o = old.Libcrux_sha3.Generic_keccak.f_st in
       let r = (Libcrux_sha3.Generic_keccak.impl_2__pi_3_ v_N #v_T ks old)
                .Libcrux_sha3.Generic_keccak.f_st in
       r.[mk_usize 3] == o.[mk_usize 18] /\
       r.[mk_usize 8] == o.[mk_usize 16] /\
       r.[mk_usize 13] == o.[mk_usize 19] /\
       r.[mk_usize 18] == o.[mk_usize 17] /\
       r.[mk_usize 23] == o.[mk_usize 15] /\
       r.[mk_usize 0] == s.[mk_usize 0] /\ r.[mk_usize 1] == s.[mk_usize 1] /\
       r.[mk_usize 2] == s.[mk_usize 2] /\ r.[mk_usize 4] == s.[mk_usize 4] /\
       r.[mk_usize 5] == s.[mk_usize 5] /\ r.[mk_usize 6] == s.[mk_usize 6] /\
       r.[mk_usize 7] == s.[mk_usize 7] /\ r.[mk_usize 9] == s.[mk_usize 9] /\
       r.[mk_usize 10] == s.[mk_usize 10] /\ r.[mk_usize 11] == s.[mk_usize 11] /\
       r.[mk_usize 12] == s.[mk_usize 12] /\ r.[mk_usize 14] == s.[mk_usize 14] /\
       r.[mk_usize 15] == s.[mk_usize 15] /\ r.[mk_usize 16] == s.[mk_usize 16] /\
       r.[mk_usize 17] == s.[mk_usize 17] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 20] == s.[mk_usize 20] /\ r.[mk_usize 21] == s.[mk_usize 21] /\
       r.[mk_usize 22] == s.[mk_usize 22] /\ r.[mk_usize 24] == s.[mk_usize 24])
  = ()
#pop-options

(** pi_4_: updates cells where [x=4] (flat [4, 9, 14, 19, 24]). *)
#push-options "--z3rlimit 200"
let lemma_pi_4_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks old: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let o = old.Libcrux_sha3.Generic_keccak.f_st in
       let r = (Libcrux_sha3.Generic_keccak.impl_2__pi_4_ v_N #v_T ks old)
                .Libcrux_sha3.Generic_keccak.f_st in
       r.[mk_usize 4] == o.[mk_usize 24] /\
       r.[mk_usize 9] == o.[mk_usize 22] /\
       r.[mk_usize 14] == o.[mk_usize 20] /\
       r.[mk_usize 19] == o.[mk_usize 23] /\
       r.[mk_usize 24] == o.[mk_usize 21] /\
       r.[mk_usize 0] == s.[mk_usize 0] /\ r.[mk_usize 1] == s.[mk_usize 1] /\
       r.[mk_usize 2] == s.[mk_usize 2] /\ r.[mk_usize 3] == s.[mk_usize 3] /\
       r.[mk_usize 5] == s.[mk_usize 5] /\ r.[mk_usize 6] == s.[mk_usize 6] /\
       r.[mk_usize 7] == s.[mk_usize 7] /\ r.[mk_usize 8] == s.[mk_usize 8] /\
       r.[mk_usize 10] == s.[mk_usize 10] /\ r.[mk_usize 11] == s.[mk_usize 11] /\
       r.[mk_usize 12] == s.[mk_usize 12] /\ r.[mk_usize 13] == s.[mk_usize 13] /\
       r.[mk_usize 15] == s.[mk_usize 15] /\ r.[mk_usize 16] == s.[mk_usize 16] /\
       r.[mk_usize 17] == s.[mk_usize 17] /\ r.[mk_usize 18] == s.[mk_usize 18] /\
       r.[mk_usize 20] == s.[mk_usize 20] /\ r.[mk_usize 21] == s.[mk_usize 21] /\
       r.[mk_usize 22] == s.[mk_usize 22] /\ r.[mk_usize 23] == s.[mk_usize 23])
  = ()
#pop-options

(** pi unfolds to pi_0_ through pi_4_ chain. *)
#push-options "--z3rlimit 100"
let lemma_pi_unfold_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
  : Lemma
      (let open Libcrux_sha3.Generic_keccak in
       impl_2__pi v_N #v_T ks ==
       (let old = ks in
        let ks0 = impl_2__pi_0_ v_N #v_T ks old in
        let ks1 = impl_2__pi_1_ v_N #v_T ks0 old in
        let ks2 = impl_2__pi_2_ v_N #v_T ks1 old in
        let ks3 = impl_2__pi_3_ v_N #v_T ks2 old in
        impl_2__pi_4_ v_N #v_T ks3 old))
  = ()
#pop-options

(* ================================================================
   Phase 2: Spec-side helpers (reused from the portable proof)
   ================================================================ *)

let spec_c (state: spec_state) (x: usize{x <. mk_usize 5}) : u64 =
  ((((Hacspec_sha3.Keccak_f.get state x (mk_usize 0)) ^.
     (Hacspec_sha3.Keccak_f.get state x (mk_usize 1))) ^.
    (Hacspec_sha3.Keccak_f.get state x (mk_usize 2))) ^.
   (Hacspec_sha3.Keccak_f.get state x (mk_usize 3))) ^.
  (Hacspec_sha3.Keccak_f.get state x (mk_usize 4))

let spec_d (state: spec_state) (x: usize{x <. mk_usize 5}) : u64 =
  (spec_c state ((x +! mk_usize 4) %! mk_usize 5)) ^.
  (Core_models.Num.impl_u64__rotate_left
    (spec_c state ((x +! mk_usize 1) %! mk_usize 5))
    (mk_u32 1))

(* ================================================================
   Phase 3: to_spec commutativity — theta+rho

   Goal: extract_lane lc (theta_rho impl_state) l == rho(theta(extract_lane lc impl_state l))

   Strategy:
   1. Use generic rho lemmas to know what each slot of the impl result
      contains (in terms of abstract v_T typeclass ops)
   2. Apply lane-correctness to convert to scalar u64 ops
   3. Match against spec rho(theta(...))
   ================================================================ *)

(** Spec-side: RHO_OFFSETS values (FIPS-native layout, indexed as
    RHO_OFFSETS[5*y + x]). Proved by reducing [v_RHO_OFFSETS] to the
    concrete literal array, then reading off each index via a single
    [assert_norm] on the concrete Seq-of-list. *)
#push-options "--z3rlimit 100"
let lemma_rho_offsets_values (_: unit)
  : Lemma (
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 0]  == mk_u32 0  /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 1]  == mk_u32 1  /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 2]  == mk_u32 62 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 3]  == mk_u32 28 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 4]  == mk_u32 27 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 5]  == mk_u32 36 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 6]  == mk_u32 44 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 7]  == mk_u32 6  /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 8]  == mk_u32 55 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 9]  == mk_u32 20 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 10] == mk_u32 3  /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 11] == mk_u32 10 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 12] == mk_u32 43 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 13] == mk_u32 25 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 14] == mk_u32 39 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 15] == mk_u32 41 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 16] == mk_u32 45 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 17] == mk_u32 15 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 18] == mk_u32 21 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 19] == mk_u32 8  /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 20] == mk_u32 18 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 21] == mk_u32 2  /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 22] == mk_u32 61 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 23] == mk_u32 56 /\
  Hacspec_sha3.Keccak_f.v_RHO_OFFSETS.[mk_usize 24] == mk_u32 14)
  = let rho_list : list u32 = [
        mk_u32 0;  mk_u32 1;  mk_u32 62; mk_u32 28; mk_u32 27;
        mk_u32 36; mk_u32 44; mk_u32 6;  mk_u32 55; mk_u32 20;
        mk_u32 3;  mk_u32 10; mk_u32 43; mk_u32 25; mk_u32 39;
        mk_u32 41; mk_u32 45; mk_u32 15; mk_u32 21; mk_u32 8;
        mk_u32 18; mk_u32 2;  mk_u32 61; mk_u32 56; mk_u32 14 ]
    in
    assert_norm(List.Tot.length rho_list == 25);
    assert (forall i. Seq.index (Hacspec_sha3.Keccak_f.v_RHO_OFFSETS) i ==
            List.Tot.index rho_list i);
    assert_norm (  List.Tot.index rho_list 0 == mk_u32 0  /\
  List.Tot.index rho_list  1 == mk_u32 1  /\
  List.Tot.index rho_list  2 == mk_u32 62 /\
  List.Tot.index rho_list  3 == mk_u32 28 /\
  List.Tot.index rho_list  4 == mk_u32 27 /\
  List.Tot.index rho_list  5 == mk_u32 36 /\
  List.Tot.index rho_list  6 == mk_u32 44 /\
  List.Tot.index rho_list  7 == mk_u32 6  /\
  List.Tot.index rho_list  8 == mk_u32 55 /\
  List.Tot.index rho_list  9 == mk_u32 20 /\
  List.Tot.index rho_list  10 == mk_u32 3  /\
  List.Tot.index rho_list  11 == mk_u32 10 /\
  List.Tot.index rho_list  12 == mk_u32 43 /\
  List.Tot.index rho_list  13 == mk_u32 25 /\
  List.Tot.index rho_list  14 == mk_u32 39 /\
  List.Tot.index rho_list  15 == mk_u32 41 /\
  List.Tot.index rho_list  16 == mk_u32 45 /\
  List.Tot.index rho_list  17 == mk_u32 15 /\
  List.Tot.index rho_list  18 == mk_u32 21 /\
  List.Tot.index rho_list  19 == mk_u32 8  /\
  List.Tot.index rho_list  20 == mk_u32 18 /\
  List.Tot.index rho_list  21 == mk_u32 2  /\
  List.Tot.index rho_list  22 == mk_u32 61 /\
  List.Tot.index rho_list  23 == mk_u32 56 /\
  List.Tot.index rho_list  24 == mk_u32 14)
#pop-options

(** Round constants equivalence. *)
#push-options "--z3rlimit 200"
let lemma_round_constants_equal (i: usize)
  : Lemma (requires i <. mk_usize 24)
          (ensures  Libcrux_sha3.Generic_keccak.Constants.v_ROUNDCONSTANTS.[i] ==
                    Hacspec_sha3.Keccak_f.v_ROUND_CONSTANTS.[i])
  = assert_norm (Libcrux_sha3.Generic_keccak.Constants.v_ROUNDCONSTANTS ==
                 Hacspec_sha3.Keccak_f.v_ROUND_CONSTANTS)
#pop-options

let rotl_spec (x: u64) (n: u32) : u64 =
  Core_models.Num.impl_u64__rotate_left x n

(** Under the FIPS-native layout, theta XORs [state.[k]] with [d[k % 5]]
    (column index is [k % 5], not [k / 5] as in the old transposed layout). *)
#push-options "--z3rlimit 400"
let lemma_rho_theta_spec (state: spec_state)
  : Lemma
      (let r = Hacspec_sha3.Keccak_f.rho (Hacspec_sha3.Keccak_f.theta state) in
       r.[mk_usize 0]  == rotl_spec (state.[mk_usize 0]  ^. spec_d state (mk_usize 0)) (mk_u32 0)  /\
       r.[mk_usize 1]  == rotl_spec (state.[mk_usize 1]  ^. spec_d state (mk_usize 1)) (mk_u32 1)  /\
       r.[mk_usize 2]  == rotl_spec (state.[mk_usize 2]  ^. spec_d state (mk_usize 2)) (mk_u32 62) /\
       r.[mk_usize 3]  == rotl_spec (state.[mk_usize 3]  ^. spec_d state (mk_usize 3)) (mk_u32 28) /\
       r.[mk_usize 4]  == rotl_spec (state.[mk_usize 4]  ^. spec_d state (mk_usize 4)) (mk_u32 27) /\
       r.[mk_usize 5]  == rotl_spec (state.[mk_usize 5]  ^. spec_d state (mk_usize 0)) (mk_u32 36) /\
       r.[mk_usize 6]  == rotl_spec (state.[mk_usize 6]  ^. spec_d state (mk_usize 1)) (mk_u32 44) /\
       r.[mk_usize 7]  == rotl_spec (state.[mk_usize 7]  ^. spec_d state (mk_usize 2)) (mk_u32 6)  /\
       r.[mk_usize 8]  == rotl_spec (state.[mk_usize 8]  ^. spec_d state (mk_usize 3)) (mk_u32 55) /\
       r.[mk_usize 9]  == rotl_spec (state.[mk_usize 9]  ^. spec_d state (mk_usize 4)) (mk_u32 20) /\
       r.[mk_usize 10] == rotl_spec (state.[mk_usize 10] ^. spec_d state (mk_usize 0)) (mk_u32 3)  /\
       r.[mk_usize 11] == rotl_spec (state.[mk_usize 11] ^. spec_d state (mk_usize 1)) (mk_u32 10) /\
       r.[mk_usize 12] == rotl_spec (state.[mk_usize 12] ^. spec_d state (mk_usize 2)) (mk_u32 43) /\
       r.[mk_usize 13] == rotl_spec (state.[mk_usize 13] ^. spec_d state (mk_usize 3)) (mk_u32 25) /\
       r.[mk_usize 14] == rotl_spec (state.[mk_usize 14] ^. spec_d state (mk_usize 4)) (mk_u32 39) /\
       r.[mk_usize 15] == rotl_spec (state.[mk_usize 15] ^. spec_d state (mk_usize 0)) (mk_u32 41) /\
       r.[mk_usize 16] == rotl_spec (state.[mk_usize 16] ^. spec_d state (mk_usize 1)) (mk_u32 45) /\
       r.[mk_usize 17] == rotl_spec (state.[mk_usize 17] ^. spec_d state (mk_usize 2)) (mk_u32 15) /\
       r.[mk_usize 18] == rotl_spec (state.[mk_usize 18] ^. spec_d state (mk_usize 3)) (mk_u32 21) /\
       r.[mk_usize 19] == rotl_spec (state.[mk_usize 19] ^. spec_d state (mk_usize 4)) (mk_u32 8)  /\
       r.[mk_usize 20] == rotl_spec (state.[mk_usize 20] ^. spec_d state (mk_usize 0)) (mk_u32 18) /\
       r.[mk_usize 21] == rotl_spec (state.[mk_usize 21] ^. spec_d state (mk_usize 1)) (mk_u32 2)  /\
       r.[mk_usize 22] == rotl_spec (state.[mk_usize 22] ^. spec_d state (mk_usize 2)) (mk_u32 61) /\
       r.[mk_usize 23] == rotl_spec (state.[mk_usize 23] ^. spec_d state (mk_usize 3)) (mk_u32 56) /\
       r.[mk_usize 24] == rotl_spec (state.[mk_usize 24] ^. spec_d state (mk_usize 4)) (mk_u32 14))
  = lemma_rho_offsets_values ();
    let ts = Hacspec_sha3.Keccak_f.theta state in
    assert (ts.[mk_usize 0] == state.[mk_usize 0] ^. spec_d state (mk_usize 0));
    assert (ts.[mk_usize 1] == state.[mk_usize 1] ^. spec_d state (mk_usize 1));
    assert (ts.[mk_usize 2] == state.[mk_usize 2] ^. spec_d state (mk_usize 2));
    assert (ts.[mk_usize 3] == state.[mk_usize 3] ^. spec_d state (mk_usize 3));
    assert (ts.[mk_usize 4] == state.[mk_usize 4] ^. spec_d state (mk_usize 4));
    assert (ts.[mk_usize 5] == state.[mk_usize 5] ^. spec_d state (mk_usize 0));
    assert (ts.[mk_usize 6] == state.[mk_usize 6] ^. spec_d state (mk_usize 1));
    assert (ts.[mk_usize 7] == state.[mk_usize 7] ^. spec_d state (mk_usize 2));
    assert (ts.[mk_usize 8] == state.[mk_usize 8] ^. spec_d state (mk_usize 3));
    assert (ts.[mk_usize 9] == state.[mk_usize 9] ^. spec_d state (mk_usize 4));
    assert (ts.[mk_usize 10] == state.[mk_usize 10] ^. spec_d state (mk_usize 0));
    assert (ts.[mk_usize 11] == state.[mk_usize 11] ^. spec_d state (mk_usize 1));
    assert (ts.[mk_usize 12] == state.[mk_usize 12] ^. spec_d state (mk_usize 2));
    assert (ts.[mk_usize 13] == state.[mk_usize 13] ^. spec_d state (mk_usize 3));
    assert (ts.[mk_usize 14] == state.[mk_usize 14] ^. spec_d state (mk_usize 4));
    assert (ts.[mk_usize 15] == state.[mk_usize 15] ^. spec_d state (mk_usize 0));
    assert (ts.[mk_usize 16] == state.[mk_usize 16] ^. spec_d state (mk_usize 1));
    assert (ts.[mk_usize 17] == state.[mk_usize 17] ^. spec_d state (mk_usize 2));
    assert (ts.[mk_usize 18] == state.[mk_usize 18] ^. spec_d state (mk_usize 3));
    assert (ts.[mk_usize 19] == state.[mk_usize 19] ^. spec_d state (mk_usize 4));
    assert (ts.[mk_usize 20] == state.[mk_usize 20] ^. spec_d state (mk_usize 0));
    assert (ts.[mk_usize 21] == state.[mk_usize 21] ^. spec_d state (mk_usize 1));
    assert (ts.[mk_usize 22] == state.[mk_usize 22] ^. spec_d state (mk_usize 2));
    assert (ts.[mk_usize 23] == state.[mk_usize 23] ^. spec_d state (mk_usize 3));
    assert (ts.[mk_usize 24] == state.[mk_usize 24] ^. spec_d state (mk_usize 4))
#pop-options

#push-options "--z3rlimit 400"
let lemma_pi_spec (state: spec_state)
  : Lemma
      (let p = Hacspec_sha3.Keccak_f.pi state in
       p.[mk_usize 0] == state.[mk_usize 0] /\
       p.[mk_usize 1] == state.[mk_usize 6] /\
       p.[mk_usize 2] == state.[mk_usize 12] /\
       p.[mk_usize 3] == state.[mk_usize 18] /\
       p.[mk_usize 4] == state.[mk_usize 24] /\
       p.[mk_usize 5] == state.[mk_usize 3] /\
       p.[mk_usize 6] == state.[mk_usize 9] /\
       p.[mk_usize 7] == state.[mk_usize 10] /\
       p.[mk_usize 8] == state.[mk_usize 16] /\
       p.[mk_usize 9] == state.[mk_usize 22] /\
       p.[mk_usize 10] == state.[mk_usize 1] /\
       p.[mk_usize 11] == state.[mk_usize 7] /\
       p.[mk_usize 12] == state.[mk_usize 13] /\
       p.[mk_usize 13] == state.[mk_usize 19] /\
       p.[mk_usize 14] == state.[mk_usize 20] /\
       p.[mk_usize 15] == state.[mk_usize 4] /\
       p.[mk_usize 16] == state.[mk_usize 5] /\
       p.[mk_usize 17] == state.[mk_usize 11] /\
       p.[mk_usize 18] == state.[mk_usize 17] /\
       p.[mk_usize 19] == state.[mk_usize 23] /\
       p.[mk_usize 20] == state.[mk_usize 2] /\
       p.[mk_usize 21] == state.[mk_usize 8] /\
       p.[mk_usize 22] == state.[mk_usize 14] /\
       p.[mk_usize 23] == state.[mk_usize 15] /\
       p.[mk_usize 24] == state.[mk_usize 21])
  = let p = normalize_term (Hacspec_sha3.Keccak_f.pi state) in
    assert_norm (p.[mk_usize 0] == state.[mk_usize 0]);
    assert_norm (p.[mk_usize 1] == state.[mk_usize 6]);
    assert_norm (p.[mk_usize 2] == state.[mk_usize 12]);
    assert_norm (p.[mk_usize 3] == state.[mk_usize 18]);
    assert_norm (p.[mk_usize 4] == state.[mk_usize 24]);
    assert_norm (p.[mk_usize 5] == state.[mk_usize 3]);
    assert_norm (p.[mk_usize 6] == state.[mk_usize 9]);
    assert_norm (p.[mk_usize 7] == state.[mk_usize 10]);
    assert_norm (p.[mk_usize 8] == state.[mk_usize 16]);
    assert_norm (p.[mk_usize 9] == state.[mk_usize 22]);
    assert_norm (p.[mk_usize 10] == state.[mk_usize 1]);
    assert_norm (p.[mk_usize 11] == state.[mk_usize 7]);
    assert_norm (p.[mk_usize 12] == state.[mk_usize 13]);
    assert_norm (p.[mk_usize 13] == state.[mk_usize 19]);
    assert_norm (p.[mk_usize 14] == state.[mk_usize 20]);
    assert_norm (p.[mk_usize 15] == state.[mk_usize 4]);
    assert_norm (p.[mk_usize 16] == state.[mk_usize 5]);
    assert_norm (p.[mk_usize 17] == state.[mk_usize 11]);
    assert_norm (p.[mk_usize 18] == state.[mk_usize 17]);
    assert_norm (p.[mk_usize 19] == state.[mk_usize 23]);
    assert_norm (p.[mk_usize 20] == state.[mk_usize 2]);
    assert_norm (p.[mk_usize 21] == state.[mk_usize 8]);
    assert_norm (p.[mk_usize 22] == state.[mk_usize 14]);
    assert_norm (p.[mk_usize 23] == state.[mk_usize 15]);
    assert_norm (p.[mk_usize 24] == state.[mk_usize 21])
#pop-options

(* ================================================================
   Phase 4: to_spec commutativity for each step

   Core lemmas: extract_lane after impl step == spec step after extract_lane
   ================================================================ *)

let d_matches_spec
      (#v_T: Type0) (v_N: usize)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (d: t_Array v_T (mk_usize 5))
      (state: spec_state)
      (l: nat{l < v v_N})
  : Type0 =
  lc.lane d.[mk_usize 0] l == spec_d state (mk_usize 0) /\
  lc.lane d.[mk_usize 1] l == spec_d state (mk_usize 1) /\
  lc.lane d.[mk_usize 2] l == spec_d state (mk_usize 2) /\
  lc.lane d.[mk_usize 3] l == spec_d state (mk_usize 3) /\
  lc.lane d.[mk_usize 4] l == spec_d state (mk_usize 4)

#push-options "--z3rlimit 800"
let lemma_theta_extract_lane
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (l: nat{l < v v_N})
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let ks', d = Libcrux_sha3.Generic_keccak.impl_2__theta v_N #v_T ks in
       let state = extract_lane v_N lc s l in
       ks'.Libcrux_sha3.Generic_keccak.f_st == s /\
       d_matches_spec v_N lc d state l)
  = let open Libcrux_sha3.Generic_keccak in
    let s = ks.f_st in
    let ks', d = impl_2__theta v_N #v_T ks in
    lemma_theta_generic v_N ks;
    let c0 = Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst
      s.[mk_usize 0] s.[mk_usize 5] s.[mk_usize 10] s.[mk_usize 15] s.[mk_usize 20] in
    let c1 = Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst
      s.[mk_usize 1] s.[mk_usize 6] s.[mk_usize 11] s.[mk_usize 16] s.[mk_usize 21] in
    let c2 = Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst
      s.[mk_usize 2] s.[mk_usize 7] s.[mk_usize 12] s.[mk_usize 17] s.[mk_usize 22] in
    let c3 = Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst
      s.[mk_usize 3] s.[mk_usize 8] s.[mk_usize 13] s.[mk_usize 18] s.[mk_usize 23] in
    let c4 = Libcrux_sha3.Traits.f_xor5 #v_T #v_N #inst
      s.[mk_usize 4] s.[mk_usize 9] s.[mk_usize 14] s.[mk_usize 19] s.[mk_usize 24] in
    lane_xor5 v_N lc s.[mk_usize 0] s.[mk_usize 5] s.[mk_usize 10] s.[mk_usize 15] s.[mk_usize 20] l;
    lane_xor5 v_N lc s.[mk_usize 1] s.[mk_usize 6] s.[mk_usize 11] s.[mk_usize 16] s.[mk_usize 21] l;
    lane_xor5 v_N lc s.[mk_usize 2] s.[mk_usize 7] s.[mk_usize 12] s.[mk_usize 17] s.[mk_usize 22] l;
    lane_xor5 v_N lc s.[mk_usize 3] s.[mk_usize 8] s.[mk_usize 13] s.[mk_usize 18] s.[mk_usize 23] l;
    lane_xor5 v_N lc s.[mk_usize 4] s.[mk_usize 9] s.[mk_usize 14] s.[mk_usize 19] s.[mk_usize 24] l;
    let state = extract_lane v_N lc s l in
    assert (lc.lane c0 l == spec_c state (mk_usize 0));
    assert (lc.lane c1 l == spec_c state (mk_usize 1));
    assert (lc.lane c2 l == spec_c state (mk_usize 2));
    assert (lc.lane c3 l == spec_c state (mk_usize 3));
    assert (lc.lane c4 l == spec_c state (mk_usize 4));
    lane_rotate_left1_and_xor v_N lc c4 c1 l;
    lane_rotate_left1_and_xor v_N lc c0 c2 l;
    lane_rotate_left1_and_xor v_N lc c1 c3 l;
    lane_rotate_left1_and_xor v_N lc c2 c4 l;
    lane_rotate_left1_and_xor v_N lc c3 c0 l;
    assert (lc.lane d.[mk_usize 0] l == spec_d state (mk_usize 0));
    assert (lc.lane d.[mk_usize 1] l == spec_d state (mk_usize 1));
    assert (lc.lane d.[mk_usize 2] l == spec_d state (mk_usize 2));
    assert (lc.lane d.[mk_usize 3] l == spec_d state (mk_usize 3));
    assert (lc.lane d.[mk_usize 4] l == spec_d state (mk_usize 4))
#pop-options

#push-options "--z3rlimit 800"
let lemma_rho_0_extract_lane
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
      (l: nat{l < v v_N})
  : Lemma
      (let s = extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l in
       let r = extract_lane v_N lc
          (Libcrux_sha3.Generic_keccak.impl_2__rho_0_ v_N #v_T ks d).Libcrux_sha3.Generic_keccak.f_st l in
       r.[mk_usize 0] == s.[mk_usize 0] ^. lc.lane d.[mk_usize 0] l /\
       r.[mk_usize 5] == rotl_spec (s.[mk_usize 5] ^. lc.lane d.[mk_usize 0] l) (mk_u32 36) /\
       r.[mk_usize 10] == rotl_spec (s.[mk_usize 10] ^. lc.lane d.[mk_usize 0] l) (mk_u32 3) /\
       r.[mk_usize 15] == rotl_spec (s.[mk_usize 15] ^. lc.lane d.[mk_usize 0] l) (mk_u32 41) /\
       r.[mk_usize 20] == rotl_spec (s.[mk_usize 20] ^. lc.lane d.[mk_usize 0] l) (mk_u32 18) /\
       r.[mk_usize 1] == s.[mk_usize 1] /\ r.[mk_usize 2] == s.[mk_usize 2] /\
       r.[mk_usize 3] == s.[mk_usize 3] /\ r.[mk_usize 4] == s.[mk_usize 4] /\
       r.[mk_usize 6] == s.[mk_usize 6] /\ r.[mk_usize 7] == s.[mk_usize 7] /\
       r.[mk_usize 8] == s.[mk_usize 8] /\ r.[mk_usize 9] == s.[mk_usize 9] /\
       r.[mk_usize 11] == s.[mk_usize 11] /\ r.[mk_usize 12] == s.[mk_usize 12] /\
       r.[mk_usize 13] == s.[mk_usize 13] /\ r.[mk_usize 14] == s.[mk_usize 14] /\
       r.[mk_usize 16] == s.[mk_usize 16] /\ r.[mk_usize 17] == s.[mk_usize 17] /\
       r.[mk_usize 18] == s.[mk_usize 18] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 21] == s.[mk_usize 21] /\ r.[mk_usize 22] == s.[mk_usize 22] /\
       r.[mk_usize 23] == s.[mk_usize 23] /\ r.[mk_usize 24] == s.[mk_usize 24])
  = let open Libcrux_sha3.Generic_keccak in
    lemma_rho_0_generic v_N ks d;
    lane_xor v_N lc ks.f_st.[mk_usize 0] d.[mk_usize 0] l;
    lane_xor_and_rotate v_N lc (mk_i32 36) (mk_i32 28) ks.f_st.[mk_usize 5] d.[mk_usize 0] l;
    lane_xor_and_rotate v_N lc (mk_i32 3) (mk_i32 61) ks.f_st.[mk_usize 10] d.[mk_usize 0] l;
    lane_xor_and_rotate v_N lc (mk_i32 41) (mk_i32 23) ks.f_st.[mk_usize 15] d.[mk_usize 0] l;
    lane_xor_and_rotate v_N lc (mk_i32 18) (mk_i32 46) ks.f_st.[mk_usize 20] d.[mk_usize 0] l
#pop-options

#push-options "--z3rlimit 800"
let lemma_rho_1_extract_lane
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
      (l: nat{l < v v_N})
  : Lemma
      (let s = extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l in
       let r = extract_lane v_N lc
          (Libcrux_sha3.Generic_keccak.impl_2__rho_1_ v_N #v_T ks d).Libcrux_sha3.Generic_keccak.f_st l in
       r.[mk_usize 1] == rotl_spec (s.[mk_usize 1] ^. lc.lane d.[mk_usize 1] l) (mk_u32 1) /\
       r.[mk_usize 6] == rotl_spec (s.[mk_usize 6] ^. lc.lane d.[mk_usize 1] l) (mk_u32 44) /\
       r.[mk_usize 11] == rotl_spec (s.[mk_usize 11] ^. lc.lane d.[mk_usize 1] l) (mk_u32 10) /\
       r.[mk_usize 16] == rotl_spec (s.[mk_usize 16] ^. lc.lane d.[mk_usize 1] l) (mk_u32 45) /\
       r.[mk_usize 21] == rotl_spec (s.[mk_usize 21] ^. lc.lane d.[mk_usize 1] l) (mk_u32 2) /\
       r.[mk_usize 0] == s.[mk_usize 0] /\ r.[mk_usize 2] == s.[mk_usize 2] /\
       r.[mk_usize 3] == s.[mk_usize 3] /\ r.[mk_usize 4] == s.[mk_usize 4] /\
       r.[mk_usize 5] == s.[mk_usize 5] /\ r.[mk_usize 7] == s.[mk_usize 7] /\
       r.[mk_usize 8] == s.[mk_usize 8] /\ r.[mk_usize 9] == s.[mk_usize 9] /\
       r.[mk_usize 10] == s.[mk_usize 10] /\ r.[mk_usize 12] == s.[mk_usize 12] /\
       r.[mk_usize 13] == s.[mk_usize 13] /\ r.[mk_usize 14] == s.[mk_usize 14] /\
       r.[mk_usize 15] == s.[mk_usize 15] /\ r.[mk_usize 17] == s.[mk_usize 17] /\
       r.[mk_usize 18] == s.[mk_usize 18] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 20] == s.[mk_usize 20] /\ r.[mk_usize 22] == s.[mk_usize 22] /\
       r.[mk_usize 23] == s.[mk_usize 23] /\ r.[mk_usize 24] == s.[mk_usize 24])
  = let open Libcrux_sha3.Generic_keccak in
    lemma_rho_1_generic v_N ks d;
    lane_xor_and_rotate v_N lc (mk_i32 1) (mk_i32 63) ks.f_st.[mk_usize 1] d.[mk_usize 1] l;
    lane_xor_and_rotate v_N lc (mk_i32 44) (mk_i32 20) ks.f_st.[mk_usize 6] d.[mk_usize 1] l;
    lane_xor_and_rotate v_N lc (mk_i32 10) (mk_i32 54) ks.f_st.[mk_usize 11] d.[mk_usize 1] l;
    lane_xor_and_rotate v_N lc (mk_i32 45) (mk_i32 19) ks.f_st.[mk_usize 16] d.[mk_usize 1] l;
    lane_xor_and_rotate v_N lc (mk_i32 2) (mk_i32 62) ks.f_st.[mk_usize 21] d.[mk_usize 1] l
#pop-options

#push-options "--z3rlimit 800"
let lemma_rho_2_extract_lane
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
      (l: nat{l < v v_N})
  : Lemma
      (let s = extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l in
       let r = extract_lane v_N lc
          (Libcrux_sha3.Generic_keccak.impl_2__rho_2_ v_N #v_T ks d).Libcrux_sha3.Generic_keccak.f_st l in
       r.[mk_usize 2] == rotl_spec (s.[mk_usize 2] ^. lc.lane d.[mk_usize 2] l) (mk_u32 62) /\
       r.[mk_usize 7] == rotl_spec (s.[mk_usize 7] ^. lc.lane d.[mk_usize 2] l) (mk_u32 6) /\
       r.[mk_usize 12] == rotl_spec (s.[mk_usize 12] ^. lc.lane d.[mk_usize 2] l) (mk_u32 43) /\
       r.[mk_usize 17] == rotl_spec (s.[mk_usize 17] ^. lc.lane d.[mk_usize 2] l) (mk_u32 15) /\
       r.[mk_usize 22] == rotl_spec (s.[mk_usize 22] ^. lc.lane d.[mk_usize 2] l) (mk_u32 61) /\
       r.[mk_usize 0] == s.[mk_usize 0] /\ r.[mk_usize 1] == s.[mk_usize 1] /\
       r.[mk_usize 3] == s.[mk_usize 3] /\ r.[mk_usize 4] == s.[mk_usize 4] /\
       r.[mk_usize 5] == s.[mk_usize 5] /\ r.[mk_usize 6] == s.[mk_usize 6] /\
       r.[mk_usize 8] == s.[mk_usize 8] /\ r.[mk_usize 9] == s.[mk_usize 9] /\
       r.[mk_usize 10] == s.[mk_usize 10] /\ r.[mk_usize 11] == s.[mk_usize 11] /\
       r.[mk_usize 13] == s.[mk_usize 13] /\ r.[mk_usize 14] == s.[mk_usize 14] /\
       r.[mk_usize 15] == s.[mk_usize 15] /\ r.[mk_usize 16] == s.[mk_usize 16] /\
       r.[mk_usize 18] == s.[mk_usize 18] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 20] == s.[mk_usize 20] /\ r.[mk_usize 21] == s.[mk_usize 21] /\
       r.[mk_usize 23] == s.[mk_usize 23] /\ r.[mk_usize 24] == s.[mk_usize 24])
  = let open Libcrux_sha3.Generic_keccak in
    lemma_rho_2_generic v_N ks d;
    lane_xor_and_rotate v_N lc (mk_i32 62) (mk_i32 2) ks.f_st.[mk_usize 2] d.[mk_usize 2] l;
    lane_xor_and_rotate v_N lc (mk_i32 6) (mk_i32 58) ks.f_st.[mk_usize 7] d.[mk_usize 2] l;
    lane_xor_and_rotate v_N lc (mk_i32 43) (mk_i32 21) ks.f_st.[mk_usize 12] d.[mk_usize 2] l;
    lane_xor_and_rotate v_N lc (mk_i32 15) (mk_i32 49) ks.f_st.[mk_usize 17] d.[mk_usize 2] l;
    lane_xor_and_rotate v_N lc (mk_i32 61) (mk_i32 3) ks.f_st.[mk_usize 22] d.[mk_usize 2] l
#pop-options

#push-options "--z3rlimit 800"
let lemma_rho_3_extract_lane
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
      (l: nat{l < v v_N})
  : Lemma
      (let s = extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l in
       let r = extract_lane v_N lc
          (Libcrux_sha3.Generic_keccak.impl_2__rho_3_ v_N #v_T ks d).Libcrux_sha3.Generic_keccak.f_st l in
       r.[mk_usize 3] == rotl_spec (s.[mk_usize 3] ^. lc.lane d.[mk_usize 3] l) (mk_u32 28) /\
       r.[mk_usize 8] == rotl_spec (s.[mk_usize 8] ^. lc.lane d.[mk_usize 3] l) (mk_u32 55) /\
       r.[mk_usize 13] == rotl_spec (s.[mk_usize 13] ^. lc.lane d.[mk_usize 3] l) (mk_u32 25) /\
       r.[mk_usize 18] == rotl_spec (s.[mk_usize 18] ^. lc.lane d.[mk_usize 3] l) (mk_u32 21) /\
       r.[mk_usize 23] == rotl_spec (s.[mk_usize 23] ^. lc.lane d.[mk_usize 3] l) (mk_u32 56) /\
       r.[mk_usize 0] == s.[mk_usize 0] /\ r.[mk_usize 1] == s.[mk_usize 1] /\
       r.[mk_usize 2] == s.[mk_usize 2] /\ r.[mk_usize 4] == s.[mk_usize 4] /\
       r.[mk_usize 5] == s.[mk_usize 5] /\ r.[mk_usize 6] == s.[mk_usize 6] /\
       r.[mk_usize 7] == s.[mk_usize 7] /\ r.[mk_usize 9] == s.[mk_usize 9] /\
       r.[mk_usize 10] == s.[mk_usize 10] /\ r.[mk_usize 11] == s.[mk_usize 11] /\
       r.[mk_usize 12] == s.[mk_usize 12] /\ r.[mk_usize 14] == s.[mk_usize 14] /\
       r.[mk_usize 15] == s.[mk_usize 15] /\ r.[mk_usize 16] == s.[mk_usize 16] /\
       r.[mk_usize 17] == s.[mk_usize 17] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 20] == s.[mk_usize 20] /\ r.[mk_usize 21] == s.[mk_usize 21] /\
       r.[mk_usize 22] == s.[mk_usize 22] /\ r.[mk_usize 24] == s.[mk_usize 24])
  = let open Libcrux_sha3.Generic_keccak in
    lemma_rho_3_generic v_N ks d;
    lane_xor_and_rotate v_N lc (mk_i32 28) (mk_i32 36) ks.f_st.[mk_usize 3] d.[mk_usize 3] l;
    lane_xor_and_rotate v_N lc (mk_i32 55) (mk_i32 9) ks.f_st.[mk_usize 8] d.[mk_usize 3] l;
    lane_xor_and_rotate v_N lc (mk_i32 25) (mk_i32 39) ks.f_st.[mk_usize 13] d.[mk_usize 3] l;
    lane_xor_and_rotate v_N lc (mk_i32 21) (mk_i32 43) ks.f_st.[mk_usize 18] d.[mk_usize 3] l;
    lane_xor_and_rotate v_N lc (mk_i32 56) (mk_i32 8) ks.f_st.[mk_usize 23] d.[mk_usize 3] l
#pop-options

#push-options "--z3rlimit 800"
let lemma_rho_4_extract_lane
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
      (l: nat{l < v v_N})
  : Lemma
      (let s = extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l in
       let r = extract_lane v_N lc
          (Libcrux_sha3.Generic_keccak.impl_2__rho_4_ v_N #v_T ks d).Libcrux_sha3.Generic_keccak.f_st l in
       r.[mk_usize 4] == rotl_spec (s.[mk_usize 4] ^. lc.lane d.[mk_usize 4] l) (mk_u32 27) /\
       r.[mk_usize 9] == rotl_spec (s.[mk_usize 9] ^. lc.lane d.[mk_usize 4] l) (mk_u32 20) /\
       r.[mk_usize 14] == rotl_spec (s.[mk_usize 14] ^. lc.lane d.[mk_usize 4] l) (mk_u32 39) /\
       r.[mk_usize 19] == rotl_spec (s.[mk_usize 19] ^. lc.lane d.[mk_usize 4] l) (mk_u32 8) /\
       r.[mk_usize 24] == rotl_spec (s.[mk_usize 24] ^. lc.lane d.[mk_usize 4] l) (mk_u32 14) /\
       r.[mk_usize 0] == s.[mk_usize 0] /\ r.[mk_usize 1] == s.[mk_usize 1] /\
       r.[mk_usize 2] == s.[mk_usize 2] /\ r.[mk_usize 3] == s.[mk_usize 3] /\
       r.[mk_usize 5] == s.[mk_usize 5] /\ r.[mk_usize 6] == s.[mk_usize 6] /\
       r.[mk_usize 7] == s.[mk_usize 7] /\ r.[mk_usize 8] == s.[mk_usize 8] /\
       r.[mk_usize 10] == s.[mk_usize 10] /\ r.[mk_usize 11] == s.[mk_usize 11] /\
       r.[mk_usize 12] == s.[mk_usize 12] /\ r.[mk_usize 13] == s.[mk_usize 13] /\
       r.[mk_usize 15] == s.[mk_usize 15] /\ r.[mk_usize 16] == s.[mk_usize 16] /\
       r.[mk_usize 17] == s.[mk_usize 17] /\ r.[mk_usize 18] == s.[mk_usize 18] /\
       r.[mk_usize 20] == s.[mk_usize 20] /\ r.[mk_usize 21] == s.[mk_usize 21] /\
       r.[mk_usize 22] == s.[mk_usize 22] /\ r.[mk_usize 23] == s.[mk_usize 23])
  = let open Libcrux_sha3.Generic_keccak in
    lemma_rho_4_generic v_N ks d;
    lane_xor_and_rotate v_N lc (mk_i32 27) (mk_i32 37) ks.f_st.[mk_usize 4] d.[mk_usize 4] l;
    lane_xor_and_rotate v_N lc (mk_i32 20) (mk_i32 44) ks.f_st.[mk_usize 9] d.[mk_usize 4] l;
    lane_xor_and_rotate v_N lc (mk_i32 39) (mk_i32 25) ks.f_st.[mk_usize 14] d.[mk_usize 4] l;
    lane_xor_and_rotate v_N lc (mk_i32 8) (mk_i32 56) ks.f_st.[mk_usize 19] d.[mk_usize 4] l;
    lane_xor_and_rotate v_N lc (mk_i32 14) (mk_i32 50) ks.f_st.[mk_usize 24] d.[mk_usize 4] l
#pop-options

(** Cumulative rho lemmas: each [lemma_rho_thru_N_extract_lane] describes
    the state after composing [rho_0_; rho_1_; ...; rho_N_] on the same
    input [ks] and [d]. The final [lemma_rho_thru_4_extract_lane] gives
    all 25 positions of [impl_2__rho ks d] in closed form, which lets
    [lemma_theta_rho_to_spec] finish via a single [eq_intro]. *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 1200"
let lemma_rho_thru_1_extract_lane
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
      (l: nat{l < v v_N})
  : Lemma
      (let s = extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l in
       let ks0 = Libcrux_sha3.Generic_keccak.impl_2__rho_0_ v_N #v_T ks d in
       let ks1 = Libcrux_sha3.Generic_keccak.impl_2__rho_1_ v_N #v_T ks0 d in
       let r = extract_lane v_N lc ks1.Libcrux_sha3.Generic_keccak.f_st l in
       (* Column 0 final *)
       r.[mk_usize 0]  == s.[mk_usize 0]  ^. lc.lane d.[mk_usize 0] l /\
       r.[mk_usize 5]  == rotl_spec (s.[mk_usize 5]  ^. lc.lane d.[mk_usize 0] l) (mk_u32 36) /\
       r.[mk_usize 10] == rotl_spec (s.[mk_usize 10] ^. lc.lane d.[mk_usize 0] l) (mk_u32 3)  /\
       r.[mk_usize 15] == rotl_spec (s.[mk_usize 15] ^. lc.lane d.[mk_usize 0] l) (mk_u32 41) /\
       r.[mk_usize 20] == rotl_spec (s.[mk_usize 20] ^. lc.lane d.[mk_usize 0] l) (mk_u32 18) /\
       (* Column 1 final *)
       r.[mk_usize 1]  == rotl_spec (s.[mk_usize 1]  ^. lc.lane d.[mk_usize 1] l) (mk_u32 1)  /\
       r.[mk_usize 6]  == rotl_spec (s.[mk_usize 6]  ^. lc.lane d.[mk_usize 1] l) (mk_u32 44) /\
       r.[mk_usize 11] == rotl_spec (s.[mk_usize 11] ^. lc.lane d.[mk_usize 1] l) (mk_u32 10) /\
       r.[mk_usize 16] == rotl_spec (s.[mk_usize 16] ^. lc.lane d.[mk_usize 1] l) (mk_u32 45) /\
       r.[mk_usize 21] == rotl_spec (s.[mk_usize 21] ^. lc.lane d.[mk_usize 1] l) (mk_u32 2)  /\
       (* Columns 2, 3, 4 unchanged *)
       r.[mk_usize 2]  == s.[mk_usize 2]  /\ r.[mk_usize 3]  == s.[mk_usize 3]  /\
       r.[mk_usize 4]  == s.[mk_usize 4]  /\ r.[mk_usize 7]  == s.[mk_usize 7]  /\
       r.[mk_usize 8]  == s.[mk_usize 8]  /\ r.[mk_usize 9]  == s.[mk_usize 9]  /\
       r.[mk_usize 12] == s.[mk_usize 12] /\ r.[mk_usize 13] == s.[mk_usize 13] /\
       r.[mk_usize 14] == s.[mk_usize 14] /\ r.[mk_usize 17] == s.[mk_usize 17] /\
       r.[mk_usize 18] == s.[mk_usize 18] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 22] == s.[mk_usize 22] /\ r.[mk_usize 23] == s.[mk_usize 23] /\
       r.[mk_usize 24] == s.[mk_usize 24])
  = let ks0 = Libcrux_sha3.Generic_keccak.impl_2__rho_0_ v_N #v_T ks d in
    lemma_rho_0_extract_lane v_N lc ks d l;
    lemma_rho_1_extract_lane v_N lc ks0 d l
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 1200"
let lemma_rho_thru_2_extract_lane
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
      (l: nat{l < v v_N})
  : Lemma
      (let s = extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l in
       let ks0 = Libcrux_sha3.Generic_keccak.impl_2__rho_0_ v_N #v_T ks d in
       let ks1 = Libcrux_sha3.Generic_keccak.impl_2__rho_1_ v_N #v_T ks0 d in
       let ks2 = Libcrux_sha3.Generic_keccak.impl_2__rho_2_ v_N #v_T ks1 d in
       let r = extract_lane v_N lc ks2.Libcrux_sha3.Generic_keccak.f_st l in
       (* Column 0 final *)
       r.[mk_usize 0]  == s.[mk_usize 0]  ^. lc.lane d.[mk_usize 0] l /\
       r.[mk_usize 5]  == rotl_spec (s.[mk_usize 5]  ^. lc.lane d.[mk_usize 0] l) (mk_u32 36) /\
       r.[mk_usize 10] == rotl_spec (s.[mk_usize 10] ^. lc.lane d.[mk_usize 0] l) (mk_u32 3)  /\
       r.[mk_usize 15] == rotl_spec (s.[mk_usize 15] ^. lc.lane d.[mk_usize 0] l) (mk_u32 41) /\
       r.[mk_usize 20] == rotl_spec (s.[mk_usize 20] ^. lc.lane d.[mk_usize 0] l) (mk_u32 18) /\
       (* Column 1 final *)
       r.[mk_usize 1]  == rotl_spec (s.[mk_usize 1]  ^. lc.lane d.[mk_usize 1] l) (mk_u32 1)  /\
       r.[mk_usize 6]  == rotl_spec (s.[mk_usize 6]  ^. lc.lane d.[mk_usize 1] l) (mk_u32 44) /\
       r.[mk_usize 11] == rotl_spec (s.[mk_usize 11] ^. lc.lane d.[mk_usize 1] l) (mk_u32 10) /\
       r.[mk_usize 16] == rotl_spec (s.[mk_usize 16] ^. lc.lane d.[mk_usize 1] l) (mk_u32 45) /\
       r.[mk_usize 21] == rotl_spec (s.[mk_usize 21] ^. lc.lane d.[mk_usize 1] l) (mk_u32 2)  /\
       (* Column 2 final *)
       r.[mk_usize 2]  == rotl_spec (s.[mk_usize 2]  ^. lc.lane d.[mk_usize 2] l) (mk_u32 62) /\
       r.[mk_usize 7]  == rotl_spec (s.[mk_usize 7]  ^. lc.lane d.[mk_usize 2] l) (mk_u32 6)  /\
       r.[mk_usize 12] == rotl_spec (s.[mk_usize 12] ^. lc.lane d.[mk_usize 2] l) (mk_u32 43) /\
       r.[mk_usize 17] == rotl_spec (s.[mk_usize 17] ^. lc.lane d.[mk_usize 2] l) (mk_u32 15) /\
       r.[mk_usize 22] == rotl_spec (s.[mk_usize 22] ^. lc.lane d.[mk_usize 2] l) (mk_u32 61) /\
       (* Columns 3, 4 unchanged *)
       r.[mk_usize 3]  == s.[mk_usize 3]  /\ r.[mk_usize 4]  == s.[mk_usize 4]  /\
       r.[mk_usize 8]  == s.[mk_usize 8]  /\ r.[mk_usize 9]  == s.[mk_usize 9]  /\
       r.[mk_usize 13] == s.[mk_usize 13] /\ r.[mk_usize 14] == s.[mk_usize 14] /\
       r.[mk_usize 18] == s.[mk_usize 18] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 23] == s.[mk_usize 23] /\ r.[mk_usize 24] == s.[mk_usize 24])
  = let ks0 = Libcrux_sha3.Generic_keccak.impl_2__rho_0_ v_N #v_T ks d in
    let ks1 = Libcrux_sha3.Generic_keccak.impl_2__rho_1_ v_N #v_T ks0 d in
    lemma_rho_thru_1_extract_lane v_N lc ks d l;
    lemma_rho_2_extract_lane v_N lc ks1 d l
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 1600"
let lemma_rho_thru_3_extract_lane
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
      (l: nat{l < v v_N})
  : Lemma
      (let s = extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l in
       let ks0 = Libcrux_sha3.Generic_keccak.impl_2__rho_0_ v_N #v_T ks d in
       let ks1 = Libcrux_sha3.Generic_keccak.impl_2__rho_1_ v_N #v_T ks0 d in
       let ks2 = Libcrux_sha3.Generic_keccak.impl_2__rho_2_ v_N #v_T ks1 d in
       let ks3 = Libcrux_sha3.Generic_keccak.impl_2__rho_3_ v_N #v_T ks2 d in
       let r = extract_lane v_N lc ks3.Libcrux_sha3.Generic_keccak.f_st l in
       (* Column 0 final *)
       r.[mk_usize 0]  == s.[mk_usize 0]  ^. lc.lane d.[mk_usize 0] l /\
       r.[mk_usize 5]  == rotl_spec (s.[mk_usize 5]  ^. lc.lane d.[mk_usize 0] l) (mk_u32 36) /\
       r.[mk_usize 10] == rotl_spec (s.[mk_usize 10] ^. lc.lane d.[mk_usize 0] l) (mk_u32 3)  /\
       r.[mk_usize 15] == rotl_spec (s.[mk_usize 15] ^. lc.lane d.[mk_usize 0] l) (mk_u32 41) /\
       r.[mk_usize 20] == rotl_spec (s.[mk_usize 20] ^. lc.lane d.[mk_usize 0] l) (mk_u32 18) /\
       (* Column 1 final *)
       r.[mk_usize 1]  == rotl_spec (s.[mk_usize 1]  ^. lc.lane d.[mk_usize 1] l) (mk_u32 1)  /\
       r.[mk_usize 6]  == rotl_spec (s.[mk_usize 6]  ^. lc.lane d.[mk_usize 1] l) (mk_u32 44) /\
       r.[mk_usize 11] == rotl_spec (s.[mk_usize 11] ^. lc.lane d.[mk_usize 1] l) (mk_u32 10) /\
       r.[mk_usize 16] == rotl_spec (s.[mk_usize 16] ^. lc.lane d.[mk_usize 1] l) (mk_u32 45) /\
       r.[mk_usize 21] == rotl_spec (s.[mk_usize 21] ^. lc.lane d.[mk_usize 1] l) (mk_u32 2)  /\
       (* Column 2 final *)
       r.[mk_usize 2]  == rotl_spec (s.[mk_usize 2]  ^. lc.lane d.[mk_usize 2] l) (mk_u32 62) /\
       r.[mk_usize 7]  == rotl_spec (s.[mk_usize 7]  ^. lc.lane d.[mk_usize 2] l) (mk_u32 6)  /\
       r.[mk_usize 12] == rotl_spec (s.[mk_usize 12] ^. lc.lane d.[mk_usize 2] l) (mk_u32 43) /\
       r.[mk_usize 17] == rotl_spec (s.[mk_usize 17] ^. lc.lane d.[mk_usize 2] l) (mk_u32 15) /\
       r.[mk_usize 22] == rotl_spec (s.[mk_usize 22] ^. lc.lane d.[mk_usize 2] l) (mk_u32 61) /\
       (* Column 3 final *)
       r.[mk_usize 3]  == rotl_spec (s.[mk_usize 3]  ^. lc.lane d.[mk_usize 3] l) (mk_u32 28) /\
       r.[mk_usize 8]  == rotl_spec (s.[mk_usize 8]  ^. lc.lane d.[mk_usize 3] l) (mk_u32 55) /\
       r.[mk_usize 13] == rotl_spec (s.[mk_usize 13] ^. lc.lane d.[mk_usize 3] l) (mk_u32 25) /\
       r.[mk_usize 18] == rotl_spec (s.[mk_usize 18] ^. lc.lane d.[mk_usize 3] l) (mk_u32 21) /\
       r.[mk_usize 23] == rotl_spec (s.[mk_usize 23] ^. lc.lane d.[mk_usize 3] l) (mk_u32 56) /\
       (* Column 4 unchanged *)
       r.[mk_usize 4]  == s.[mk_usize 4]  /\ r.[mk_usize 9]  == s.[mk_usize 9]  /\
       r.[mk_usize 14] == s.[mk_usize 14] /\ r.[mk_usize 19] == s.[mk_usize 19] /\
       r.[mk_usize 24] == s.[mk_usize 24])
  = let ks0 = Libcrux_sha3.Generic_keccak.impl_2__rho_0_ v_N #v_T ks d in
    let ks1 = Libcrux_sha3.Generic_keccak.impl_2__rho_1_ v_N #v_T ks0 d in
    let ks2 = Libcrux_sha3.Generic_keccak.impl_2__rho_2_ v_N #v_T ks1 d in
    lemma_rho_thru_2_extract_lane v_N lc ks d l;
    lemma_rho_3_extract_lane v_N lc ks2 d l
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 1600"
let lemma_rho_thru_4_extract_lane
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (d: t_Array v_T (mk_usize 5))
      (l: nat{l < v v_N})
  : Lemma
      (let s = extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l in
       let r = extract_lane v_N lc
         (Libcrux_sha3.Generic_keccak.impl_2__rho v_N #v_T ks d)
           .Libcrux_sha3.Generic_keccak.f_st l in
       (* All 25 positions final: rotl_spec(s.[k] ^. d.[k % 5]_l, RHO_OFFSETS[k]) *)
       r.[mk_usize 0]  == s.[mk_usize 0]  ^. lc.lane d.[mk_usize 0] l /\
       r.[mk_usize 1]  == rotl_spec (s.[mk_usize 1]  ^. lc.lane d.[mk_usize 1] l) (mk_u32 1)  /\
       r.[mk_usize 2]  == rotl_spec (s.[mk_usize 2]  ^. lc.lane d.[mk_usize 2] l) (mk_u32 62) /\
       r.[mk_usize 3]  == rotl_spec (s.[mk_usize 3]  ^. lc.lane d.[mk_usize 3] l) (mk_u32 28) /\
       r.[mk_usize 4]  == rotl_spec (s.[mk_usize 4]  ^. lc.lane d.[mk_usize 4] l) (mk_u32 27) /\
       r.[mk_usize 5]  == rotl_spec (s.[mk_usize 5]  ^. lc.lane d.[mk_usize 0] l) (mk_u32 36) /\
       r.[mk_usize 6]  == rotl_spec (s.[mk_usize 6]  ^. lc.lane d.[mk_usize 1] l) (mk_u32 44) /\
       r.[mk_usize 7]  == rotl_spec (s.[mk_usize 7]  ^. lc.lane d.[mk_usize 2] l) (mk_u32 6)  /\
       r.[mk_usize 8]  == rotl_spec (s.[mk_usize 8]  ^. lc.lane d.[mk_usize 3] l) (mk_u32 55) /\
       r.[mk_usize 9]  == rotl_spec (s.[mk_usize 9]  ^. lc.lane d.[mk_usize 4] l) (mk_u32 20) /\
       r.[mk_usize 10] == rotl_spec (s.[mk_usize 10] ^. lc.lane d.[mk_usize 0] l) (mk_u32 3)  /\
       r.[mk_usize 11] == rotl_spec (s.[mk_usize 11] ^. lc.lane d.[mk_usize 1] l) (mk_u32 10) /\
       r.[mk_usize 12] == rotl_spec (s.[mk_usize 12] ^. lc.lane d.[mk_usize 2] l) (mk_u32 43) /\
       r.[mk_usize 13] == rotl_spec (s.[mk_usize 13] ^. lc.lane d.[mk_usize 3] l) (mk_u32 25) /\
       r.[mk_usize 14] == rotl_spec (s.[mk_usize 14] ^. lc.lane d.[mk_usize 4] l) (mk_u32 39) /\
       r.[mk_usize 15] == rotl_spec (s.[mk_usize 15] ^. lc.lane d.[mk_usize 0] l) (mk_u32 41) /\
       r.[mk_usize 16] == rotl_spec (s.[mk_usize 16] ^. lc.lane d.[mk_usize 1] l) (mk_u32 45) /\
       r.[mk_usize 17] == rotl_spec (s.[mk_usize 17] ^. lc.lane d.[mk_usize 2] l) (mk_u32 15) /\
       r.[mk_usize 18] == rotl_spec (s.[mk_usize 18] ^. lc.lane d.[mk_usize 3] l) (mk_u32 21) /\
       r.[mk_usize 19] == rotl_spec (s.[mk_usize 19] ^. lc.lane d.[mk_usize 4] l) (mk_u32 8)  /\
       r.[mk_usize 20] == rotl_spec (s.[mk_usize 20] ^. lc.lane d.[mk_usize 0] l) (mk_u32 18) /\
       r.[mk_usize 21] == rotl_spec (s.[mk_usize 21] ^. lc.lane d.[mk_usize 1] l) (mk_u32 2)  /\
       r.[mk_usize 22] == rotl_spec (s.[mk_usize 22] ^. lc.lane d.[mk_usize 2] l) (mk_u32 61) /\
       r.[mk_usize 23] == rotl_spec (s.[mk_usize 23] ^. lc.lane d.[mk_usize 3] l) (mk_u32 56) /\
       r.[mk_usize 24] == rotl_spec (s.[mk_usize 24] ^. lc.lane d.[mk_usize 4] l) (mk_u32 14))
  = let ks0 = Libcrux_sha3.Generic_keccak.impl_2__rho_0_ v_N #v_T ks d in
    let ks1 = Libcrux_sha3.Generic_keccak.impl_2__rho_1_ v_N #v_T ks0 d in
    let ks2 = Libcrux_sha3.Generic_keccak.impl_2__rho_2_ v_N #v_T ks1 d in
    let ks3 = Libcrux_sha3.Generic_keccak.impl_2__rho_3_ v_N #v_T ks2 d in
    lemma_rho_unfold_generic v_N ks d;
    lemma_rho_thru_3_extract_lane v_N lc ks d l;
    lemma_rho_4_extract_lane v_N lc ks3 d l
#pop-options

(** Theta+Rho commutativity:
    extract_lane lc (rho(theta(ks))).f_st l == rho(theta(extract_lane lc ks.f_st l))

    The cumulative [lemma_rho_thru_4_extract_lane] carries all 25 positions
    of [impl_2__rho ks' d] in closed form. Combined with [lemma_theta_extract_lane]
    (which shows [ks'.f_st == s] and [d_matches_spec]) and [lemma_rho_theta_spec]
    (spec-side 25-position result with matching offsets), the goal reduces to
    pointwise equality + [eq_intro]. *)

(* Theta+Rho commutativity (2026-05-04: factored as 5 row-helpers + a dispatcher).

   The cumulative [lemma_rho_thru_4_extract_lane] supplies all 25 impl-side
   per-index equalities; [lemma_rho_theta_spec] supplies the matching 25
   spec-side ones; [lemma_theta_extract_lane] bridges the two via
   [d_matches_spec]. The remaining work is just lifting 25 in-scope
   pointwise equalities to a forall for [eq_intro].

   Prior attempts that fed all 25 asserts to a single [eq_intro] timed out
   on the forall-precondition consolidation (see proof_milestones.md
   Note A). The fix is to split the post into 5 row-shaped 5-conjunct
   sub-goals (each closes monolithically) and assemble with a 5-way
   case-split on [i / 5]. *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_theta_rho_row_0_to_spec
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (l: nat{l < v v_N})
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let ks', d = Libcrux_sha3.Generic_keccak.impl_2__theta v_N #v_T ks in
       let ks'' = Libcrux_sha3.Generic_keccak.impl_2__rho v_N #v_T ks' d in
       let lhs = extract_lane v_N lc ks''.Libcrux_sha3.Generic_keccak.f_st l in
       let rhs = Hacspec_sha3.Keccak_f.rho
                   (Hacspec_sha3.Keccak_f.theta (extract_lane v_N lc s l)) in
       lhs.[mk_usize 0] == rhs.[mk_usize 0] /\
       lhs.[mk_usize 1] == rhs.[mk_usize 1] /\
       lhs.[mk_usize 2] == rhs.[mk_usize 2] /\
       lhs.[mk_usize 3] == rhs.[mk_usize 3] /\
       lhs.[mk_usize 4] == rhs.[mk_usize 4])
  = let open Libcrux_sha3.Generic_keccak in
    let s = ks.f_st in
    let ks', d = impl_2__theta v_N #v_T ks in
    let state = extract_lane v_N lc s l in
    lemma_theta_extract_lane v_N lc ks l;
    lemma_rho_thru_4_extract_lane v_N lc ks' d l;
    lemma_rho_theta_spec state;
    Lemmas.lemma_rotate_left_zero (state.[mk_usize 0] ^. spec_d state (mk_usize 0))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_theta_rho_row_1_to_spec
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (l: nat{l < v v_N})
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let ks', d = Libcrux_sha3.Generic_keccak.impl_2__theta v_N #v_T ks in
       let ks'' = Libcrux_sha3.Generic_keccak.impl_2__rho v_N #v_T ks' d in
       let lhs = extract_lane v_N lc ks''.Libcrux_sha3.Generic_keccak.f_st l in
       let rhs = Hacspec_sha3.Keccak_f.rho
                   (Hacspec_sha3.Keccak_f.theta (extract_lane v_N lc s l)) in
       lhs.[mk_usize 5] == rhs.[mk_usize 5] /\
       lhs.[mk_usize 6] == rhs.[mk_usize 6] /\
       lhs.[mk_usize 7] == rhs.[mk_usize 7] /\
       lhs.[mk_usize 8] == rhs.[mk_usize 8] /\
       lhs.[mk_usize 9] == rhs.[mk_usize 9])
  = let open Libcrux_sha3.Generic_keccak in
    let s = ks.f_st in
    let ks', d = impl_2__theta v_N #v_T ks in
    let state = extract_lane v_N lc s l in
    lemma_theta_extract_lane v_N lc ks l;
    lemma_rho_thru_4_extract_lane v_N lc ks' d l;
    lemma_rho_theta_spec state
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_theta_rho_row_2_to_spec
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (l: nat{l < v v_N})
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let ks', d = Libcrux_sha3.Generic_keccak.impl_2__theta v_N #v_T ks in
       let ks'' = Libcrux_sha3.Generic_keccak.impl_2__rho v_N #v_T ks' d in
       let lhs = extract_lane v_N lc ks''.Libcrux_sha3.Generic_keccak.f_st l in
       let rhs = Hacspec_sha3.Keccak_f.rho
                   (Hacspec_sha3.Keccak_f.theta (extract_lane v_N lc s l)) in
       lhs.[mk_usize 10] == rhs.[mk_usize 10] /\
       lhs.[mk_usize 11] == rhs.[mk_usize 11] /\
       lhs.[mk_usize 12] == rhs.[mk_usize 12] /\
       lhs.[mk_usize 13] == rhs.[mk_usize 13] /\
       lhs.[mk_usize 14] == rhs.[mk_usize 14])
  = let open Libcrux_sha3.Generic_keccak in
    let s = ks.f_st in
    let ks', d = impl_2__theta v_N #v_T ks in
    let state = extract_lane v_N lc s l in
    lemma_theta_extract_lane v_N lc ks l;
    lemma_rho_thru_4_extract_lane v_N lc ks' d l;
    lemma_rho_theta_spec state
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_theta_rho_row_3_to_spec
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (l: nat{l < v v_N})
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let ks', d = Libcrux_sha3.Generic_keccak.impl_2__theta v_N #v_T ks in
       let ks'' = Libcrux_sha3.Generic_keccak.impl_2__rho v_N #v_T ks' d in
       let lhs = extract_lane v_N lc ks''.Libcrux_sha3.Generic_keccak.f_st l in
       let rhs = Hacspec_sha3.Keccak_f.rho
                   (Hacspec_sha3.Keccak_f.theta (extract_lane v_N lc s l)) in
       lhs.[mk_usize 15] == rhs.[mk_usize 15] /\
       lhs.[mk_usize 16] == rhs.[mk_usize 16] /\
       lhs.[mk_usize 17] == rhs.[mk_usize 17] /\
       lhs.[mk_usize 18] == rhs.[mk_usize 18] /\
       lhs.[mk_usize 19] == rhs.[mk_usize 19])
  = let open Libcrux_sha3.Generic_keccak in
    let s = ks.f_st in
    let ks', d = impl_2__theta v_N #v_T ks in
    let state = extract_lane v_N lc s l in
    lemma_theta_extract_lane v_N lc ks l;
    lemma_rho_thru_4_extract_lane v_N lc ks' d l;
    lemma_rho_theta_spec state
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_theta_rho_row_4_to_spec
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (l: nat{l < v v_N})
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let ks', d = Libcrux_sha3.Generic_keccak.impl_2__theta v_N #v_T ks in
       let ks'' = Libcrux_sha3.Generic_keccak.impl_2__rho v_N #v_T ks' d in
       let lhs = extract_lane v_N lc ks''.Libcrux_sha3.Generic_keccak.f_st l in
       let rhs = Hacspec_sha3.Keccak_f.rho
                   (Hacspec_sha3.Keccak_f.theta (extract_lane v_N lc s l)) in
       lhs.[mk_usize 20] == rhs.[mk_usize 20] /\
       lhs.[mk_usize 21] == rhs.[mk_usize 21] /\
       lhs.[mk_usize 22] == rhs.[mk_usize 22] /\
       lhs.[mk_usize 23] == rhs.[mk_usize 23] /\
       lhs.[mk_usize 24] == rhs.[mk_usize 24])
  = let open Libcrux_sha3.Generic_keccak in
    let s = ks.f_st in
    let ks', d = impl_2__theta v_N #v_T ks in
    let state = extract_lane v_N lc s l in
    lemma_theta_extract_lane v_N lc ks l;
    lemma_rho_thru_4_extract_lane v_N lc ks' d l;
    lemma_rho_theta_spec state
#pop-options

let forall25 (p:(i:nat{i < 25} -> Type0)):
  Lemma (requires (p 0 /\ p 1 /\ p 2 /\ p 3 /\ p 4 /\
                   p 5 /\ p 6 /\ p 7 /\ p 8 /\ p 9 /\
                   p 10 /\ p 11 /\ p 12 /\ p 13 /\ p 14 /\
                   p 15 /\ p 16 /\ p 17 /\ p 18 /\ p 19 /\
                   p 20 /\ p 21 /\ p 22 /\ p 23 /\ p 24))
        (ensures (forall (i:nat{i < 25}). p i)) = ()
        
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_theta_rho_to_spec
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (l: nat{l < v v_N})
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let ks', d = Libcrux_sha3.Generic_keccak.impl_2__theta v_N #v_T ks in
       let ks'' = Libcrux_sha3.Generic_keccak.impl_2__rho v_N #v_T ks' d in
       extract_lane v_N lc ks''.Libcrux_sha3.Generic_keccak.f_st l ==
       Hacspec_sha3.Keccak_f.rho (Hacspec_sha3.Keccak_f.theta (extract_lane v_N lc s l)))
  = lemma_theta_rho_row_0_to_spec v_N lc ks l;
    lemma_theta_rho_row_1_to_spec v_N lc ks l;
    lemma_theta_rho_row_2_to_spec v_N lc ks l;
    lemma_theta_rho_row_3_to_spec v_N lc ks l;
    lemma_theta_rho_row_4_to_spec v_N lc ks l;
    let s = ks.Libcrux_sha3.Generic_keccak.f_st in
    let ks', d = Libcrux_sha3.Generic_keccak.impl_2__theta v_N #v_T ks in
    let ks'' = Libcrux_sha3.Generic_keccak.impl_2__rho v_N #v_T ks' d in
    let lhs = extract_lane v_N lc ks''.Libcrux_sha3.Generic_keccak.f_st l in
    let rhs = Hacspec_sha3.Keccak_f.rho (Hacspec_sha3.Keccak_f.theta (extract_lane v_N lc s l)) in
    forall25 (fun i -> Seq.index lhs i == Seq.index rhs i);
    eq_intro lhs rhs
#pop-options

(** Pi extract_lane: states all 25 indices of pi result at u64 level.
    Chains the 5 sub-step generics + SMTPat conversion to extract_lane.
    Pi is a pure permutation: r.[k] == state.[pi_perm(k)]. *)

#push-options "--z3rlimit 1200"
let lemma_pi_extract_lane
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (l: nat{l < v v_N})
  : Lemma
      (let state = extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l in
       let r = extract_lane v_N lc
                (Libcrux_sha3.Generic_keccak.impl_2__pi v_N #v_T ks)
                  .Libcrux_sha3.Generic_keccak.f_st l in
       r.[mk_usize 0] == state.[mk_usize 0] /\
       r.[mk_usize 1] == state.[mk_usize 6] /\
       r.[mk_usize 2] == state.[mk_usize 12] /\
       r.[mk_usize 3] == state.[mk_usize 18] /\
       r.[mk_usize 4] == state.[mk_usize 24] /\
       r.[mk_usize 5] == state.[mk_usize 3] /\
       r.[mk_usize 6] == state.[mk_usize 9] /\
       r.[mk_usize 7] == state.[mk_usize 10] /\
       r.[mk_usize 8] == state.[mk_usize 16] /\
       r.[mk_usize 9] == state.[mk_usize 22] /\
       r.[mk_usize 10] == state.[mk_usize 1] /\
       r.[mk_usize 11] == state.[mk_usize 7] /\
       r.[mk_usize 12] == state.[mk_usize 13] /\
       r.[mk_usize 13] == state.[mk_usize 19] /\
       r.[mk_usize 14] == state.[mk_usize 20] /\
       r.[mk_usize 15] == state.[mk_usize 4] /\
       r.[mk_usize 16] == state.[mk_usize 5] /\
       r.[mk_usize 17] == state.[mk_usize 11] /\
       r.[mk_usize 18] == state.[mk_usize 17] /\
       r.[mk_usize 19] == state.[mk_usize 23] /\
       r.[mk_usize 20] == state.[mk_usize 2] /\
       r.[mk_usize 21] == state.[mk_usize 8] /\
       r.[mk_usize 22] == state.[mk_usize 14] /\
       r.[mk_usize 23] == state.[mk_usize 15] /\
       r.[mk_usize 24] == state.[mk_usize 21])
  = let open Libcrux_sha3.Generic_keccak in
    let old = ks in
    let ks0 = impl_2__pi_0_ v_N #v_T ks old in
    lemma_pi_0_generic v_N ks old;
    let ks1 = impl_2__pi_1_ v_N #v_T ks0 old in
    lemma_pi_1_generic v_N ks0 old;
    let ks2 = impl_2__pi_2_ v_N #v_T ks1 old in
    lemma_pi_2_generic v_N ks1 old;
    let ks3 = impl_2__pi_3_ v_N #v_T ks2 old in
    lemma_pi_3_generic v_N ks2 old;
    let ks4 = impl_2__pi_4_ v_N #v_T ks3 old in
    lemma_pi_4_generic v_N ks3 old;
    lemma_pi_unfold_generic v_N ks
#pop-options

(** Pi commutativity:
    extract_lane lc (pi(ks)).f_st l == pi(extract_lane lc ks.f_st l)

    lemma_pi_extract_lane provides u64-level facts via extract_lane,
    lemma_pi_spec provides the spec side, eq_intro closes. *)

#push-options "--z3rlimit 800"
let lemma_pi_to_spec
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (l: nat{l < v v_N})
  : Lemma
      (extract_lane v_N lc
        (Libcrux_sha3.Generic_keccak.impl_2__pi v_N #v_T ks)
          .Libcrux_sha3.Generic_keccak.f_st l ==
       Hacspec_sha3.Keccak_f.pi (extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l))
  = let open Libcrux_sha3.Generic_keccak in
    lemma_pi_extract_lane v_N lc ks l;
    let state = extract_lane v_N lc ks.f_st l in
    lemma_pi_spec state;
    let lhs = extract_lane v_N lc (impl_2__pi v_N #v_T ks).f_st l in
    let rhs = Hacspec_sha3.Keccak_f.pi state in
    forall25 (fun i -> Seq.index lhs i == Seq.index rhs i);
    Rust_primitives.Arrays.eq_intro lhs rhs
#pop-options

(** Chi extract_lane: states all 25 indices of chi result at u64 level.

    Strategy:
      1. [ChiFold.lemma_chi_val_i] gives, for any flat index [k < 25]:
           (impl_2__chi v_N #v_T ks).f_st.[k] == chi_inner_val ks (k/5) (k%5)
         Under the FIPS-native layout [get_ij(arr, i, j) = arr[5*i + j]],
         flat index [k] corresponds to impl-[(i, j) = (k/5, k%5)] which
         is FIPS [(y, x) = (k/5, k%5)], i.e. [x = k%5, y = k/5].
         [chi_inner_val] is a transparent [let] that unfolds to
         [f_and_not_xor] of three indices along the [x] axis at fixed [y].
      2. [lane_and_not_xor] (operation-level commutativity above) lifts
         that equality through [lc.lane].
      3. [logand_commutative] swaps `(b &. ~.c)` to `(~.c &. b)` to
         match the spec orientation. *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_chi_extract_lane_aux
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (l: nat{l < v v_N})
      (k: usize{v k < 25})
  : Lemma
      (let s = ks.Libcrux_sha3.Generic_keccak.f_st in
       let i = k /! sz 5 in
       let j = k %! sz 5 in
       lc.lane (Libcrux_sha3.Generic_keccak.impl_2__chi v_N #v_T ks)
                 .Libcrux_sha3.Generic_keccak.f_st.[k] l ==
         lc.lane s.[k] l ^.
           ((~. (lc.lane s.[ (mk_usize 5 *! i) +! ((j +! mk_usize 1) %! mk_usize 5) ] l)) &.
             lc.lane s.[ (mk_usize 5 *! i) +! ((j +! mk_usize 2) %! mk_usize 5) ] l))
  = let i = k /! sz 5 in
    let j = k %! sz 5 in
    let s = ks.Libcrux_sha3.Generic_keccak.f_st in
    assert (k == sz 5 *! i +! j);
    ChiFold.lemma_chi_val_i v_N #v_T ks k;
    lane_and_not_xor v_N lc
      (ks.[ i, j <: (usize & usize) ] <: v_T)
      (ks.[ i, ((j +! mk_usize 2) %! mk_usize 5) <: (usize & usize) ] <: v_T)
      (ks.[ i, ((j +! mk_usize 1) %! mk_usize 5) <: (usize & usize) ] <: v_T)
      l;
    Lemmas.logand_commutative
      (lc.lane s.[ (mk_usize 5 *! i) +! ((j +! mk_usize 2) %! mk_usize 5) ] l)
      (~. (lc.lane s.[ (mk_usize 5 *! i) +! ((j +! mk_usize 1) %! mk_usize 5) ] l))
#pop-options

(** Chi commutativity:
    extract_lane lc (chi(ks)).f_st l == chi(extract_lane lc ks.f_st l)

    Direct pointwise proof: [lemma_chi_extract_lane_aux] gives the
    per-index equality at the u64 level, and [Hacspec_sha3.createi_lemma]
    is an SMTPat that unfolds [(chi state).[k]] on the spec side. We
    introduce the universal pointwise fact via [Classical.forall_intro]
    and conclude with array extensionality. *)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_chi_to_spec
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (l: nat{l < v v_N})
  : Lemma
      (extract_lane v_N lc
        (Libcrux_sha3.Generic_keccak.impl_2__chi v_N #v_T ks)
          .Libcrux_sha3.Generic_keccak.f_st l ==
       Hacspec_sha3.Keccak_f.chi (extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l))
  = let open Libcrux_sha3.Generic_keccak in
    let lhs = extract_lane v_N lc (impl_2__chi v_N #v_T ks).f_st l in
    let state = extract_lane v_N lc ks.f_st l in
    let rhs = Hacspec_sha3.Keccak_f.chi state in
    let aux (i: nat{i < 25}) : Lemma (Seq.index lhs i == Seq.index rhs i) =
      let k : usize = mk_usize i in
      assert (v k == i);
      lemma_chi_extract_lane_aux v_N #v_T lc ks l k;
      assert (lhs.[k] == rhs.[k]);
      assert (lhs.[k] == Seq.index lhs i);
      assert (rhs.[k] == Seq.index rhs i)
    in
    Classical.forall_intro aux;
    Rust_primitives.Arrays.eq_intro lhs rhs
#pop-options

(** Iota spec-side: unfold iota at each index.
    Index 0: state[0] ^. RC[round].  Indices 1-24: unchanged. *)

let lemma_iota_spec (state: spec_state) (round: usize)
  : Lemma
      (requires round <. mk_usize 24)
      (ensures
        (let r = Hacspec_sha3.Keccak_f.iota state round in
         r.[mk_usize 0] == (state.[mk_usize 0] <: u64) ^. (Hacspec_sha3.Keccak_f.v_ROUND_CONSTANTS.[round] <: u64) /\
         r.[mk_usize 1] == state.[mk_usize 1] /\
         r.[mk_usize 2] == state.[mk_usize 2] /\
         r.[mk_usize 3] == state.[mk_usize 3] /\
         r.[mk_usize 4] == state.[mk_usize 4] /\
         r.[mk_usize 5] == state.[mk_usize 5] /\
         r.[mk_usize 6] == state.[mk_usize 6] /\
         r.[mk_usize 7] == state.[mk_usize 7] /\
         r.[mk_usize 8] == state.[mk_usize 8] /\
         r.[mk_usize 9] == state.[mk_usize 9] /\
         r.[mk_usize 10] == state.[mk_usize 10] /\
         r.[mk_usize 11] == state.[mk_usize 11] /\
         r.[mk_usize 12] == state.[mk_usize 12] /\
         r.[mk_usize 13] == state.[mk_usize 13] /\
         r.[mk_usize 14] == state.[mk_usize 14] /\
         r.[mk_usize 15] == state.[mk_usize 15] /\
         r.[mk_usize 16] == state.[mk_usize 16] /\
         r.[mk_usize 17] == state.[mk_usize 17] /\
         r.[mk_usize 18] == state.[mk_usize 18] /\
         r.[mk_usize 19] == state.[mk_usize 19] /\
         r.[mk_usize 20] == state.[mk_usize 20] /\
         r.[mk_usize 21] == state.[mk_usize 21] /\
         r.[mk_usize 22] == state.[mk_usize 22] /\
         r.[mk_usize 23] == state.[mk_usize 23] /\
         r.[mk_usize 24] == state.[mk_usize 24]))
  = ()

(** Iota extract_lane: only index 0 changes (via lane_xor_constant),
    indices 1-24 are preserved. *)

let lemma_iota_extract_lane
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (round: usize)
      (l: nat{l < v v_N})
  : Lemma
      (requires round <. mk_usize 24)
      (ensures
        (let state = extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l in
         let r = extract_lane v_N lc
                   (Libcrux_sha3.Generic_keccak.impl_2__iota v_N #v_T ks round)
                     .Libcrux_sha3.Generic_keccak.f_st l in
         r.[mk_usize 0] == (state.[mk_usize 0] <: u64) ^.
           (Libcrux_sha3.Generic_keccak.Constants.v_ROUNDCONSTANTS.[round] <: u64) /\
         r.[mk_usize 1] == state.[mk_usize 1] /\
         r.[mk_usize 2] == state.[mk_usize 2] /\
         r.[mk_usize 3] == state.[mk_usize 3] /\
         r.[mk_usize 4] == state.[mk_usize 4] /\
         r.[mk_usize 5] == state.[mk_usize 5] /\
         r.[mk_usize 6] == state.[mk_usize 6] /\
         r.[mk_usize 7] == state.[mk_usize 7] /\
         r.[mk_usize 8] == state.[mk_usize 8] /\
         r.[mk_usize 9] == state.[mk_usize 9] /\
         r.[mk_usize 10] == state.[mk_usize 10] /\
         r.[mk_usize 11] == state.[mk_usize 11] /\
         r.[mk_usize 12] == state.[mk_usize 12] /\
         r.[mk_usize 13] == state.[mk_usize 13] /\
         r.[mk_usize 14] == state.[mk_usize 14] /\
         r.[mk_usize 15] == state.[mk_usize 15] /\
         r.[mk_usize 16] == state.[mk_usize 16] /\
         r.[mk_usize 17] == state.[mk_usize 17] /\
         r.[mk_usize 18] == state.[mk_usize 18] /\
         r.[mk_usize 19] == state.[mk_usize 19] /\
         r.[mk_usize 20] == state.[mk_usize 20] /\
         r.[mk_usize 21] == state.[mk_usize 21] /\
         r.[mk_usize 22] == state.[mk_usize 22] /\
         r.[mk_usize 23] == state.[mk_usize 23] /\
         r.[mk_usize 24] == state.[mk_usize 24]))
  = lane_xor_constant v_N lc
      ks.Libcrux_sha3.Generic_keccak.f_st.[mk_usize 0]
      (Libcrux_sha3.Generic_keccak.Constants.v_ROUNDCONSTANTS.[round])
      l

(** Iota commutativity:
    extract_lane lc (iota(ks, round)).f_st l == iota(extract_lane lc ks.f_st l, round)

    lemma_iota_extract_lane provides u64-level facts via extract_lane,
    lemma_iota_spec provides the spec side, eq_intro closes. *)

#push-options "--z3rlimit 200"
let lemma_iota_to_spec
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (round: usize)
      (l: nat{l < v v_N})
  : Lemma
      (requires round <. mk_usize 24)
      (ensures
        extract_lane v_N lc
          (Libcrux_sha3.Generic_keccak.impl_2__iota v_N #v_T ks round)
            .Libcrux_sha3.Generic_keccak.f_st l ==
        Hacspec_sha3.Keccak_f.iota (extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l) round)
  = let open Libcrux_sha3.Generic_keccak in
    let state = extract_lane v_N lc ks.f_st l in
    lemma_round_constants_equal round;
    lemma_iota_extract_lane v_N lc ks round l;
    lemma_iota_spec state round;
    Rust_primitives.Arrays.eq_intro
      (extract_lane v_N lc (impl_2__iota v_N #v_T ks round).f_st l)
      (Hacspec_sha3.Keccak_f.iota state round)
#pop-options

(* ================================================================
   Phase 5: One-round and full keccakf1600 commutativity
   ================================================================ *)

let impl_one_round
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (i: usize)
  : Pure (Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (requires i <. mk_usize 24) (fun _ -> True) =
  let open Libcrux_sha3.Generic_keccak in
  let tmp0, t = impl_2__theta v_N #v_T ks in
  let ks1 = impl_2__rho v_N #v_T tmp0 t in
  let ks2 = impl_2__pi v_N #v_T ks1 in
  let ks3 = impl_2__chi v_N #v_T ks2 in
  impl_2__iota v_N #v_T ks3 i

let spec_one_round = SpecRounds.spec_one_round

(** One-round commutativity: composition of per-step commutativity. *)
#push-options "--z3rlimit 200"
let lemma_one_round_to_spec
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (round: usize)
      (l: nat{l < v v_N})
  : Lemma
      (requires round <. mk_usize 24)
      (ensures
        extract_lane v_N lc (impl_one_round v_N ks round)
          .Libcrux_sha3.Generic_keccak.f_st l ==
        spec_one_round (extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l) round)
  = let open Libcrux_sha3.Generic_keccak in
    let s = ks.f_st in
    let ks', d = impl_2__theta v_N #v_T ks in
    let ks1 = impl_2__rho v_N #v_T ks' d in
    lemma_theta_rho_to_spec v_N lc ks l;
    let spec_after_rho = Hacspec_sha3.Keccak_f.rho (Hacspec_sha3.Keccak_f.theta (extract_lane v_N lc s l)) in
    assert (extract_lane v_N lc ks1.f_st l == spec_after_rho);
    let ks2 = impl_2__pi v_N #v_T ks1 in
    lemma_pi_to_spec v_N lc ks1 l;
    let spec_after_pi = Hacspec_sha3.Keccak_f.pi spec_after_rho in
    assert (extract_lane v_N lc ks2.f_st l == spec_after_pi);
    let ks3 = impl_2__chi v_N #v_T ks2 in
    lemma_chi_to_spec v_N lc ks2 l;
    let spec_after_chi = Hacspec_sha3.Keccak_f.chi spec_after_pi in
    assert (extract_lane v_N lc ks3.f_st l == spec_after_chi);
    lemma_iota_to_spec v_N lc ks3 round l
#pop-options

(** Recursive helpers for multi-round iteration. *)
let rec impl_rounds
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (r: usize)
  : Pure (Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (requires r <=. mk_usize 24) (fun _ -> True)
      (decreases (v (mk_usize 24) - v r)) =
  if r =. mk_usize 24 then ks
  else impl_rounds v_N (impl_one_round v_N ks r) (r +! mk_usize 1)

let spec_rounds = SpecRounds.spec_rounds

(** Induction: impl_rounds and spec_rounds commute with extract_lane. *)
#push-options "--fuel 1 --z3rlimit 200"
let rec lemma_rounds_to_spec
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (r: usize)
      (l: nat{l < v v_N})
  : Lemma
      (requires r <=. mk_usize 24)
      (ensures
        extract_lane v_N lc (impl_rounds v_N ks r).Libcrux_sha3.Generic_keccak.f_st l ==
        spec_rounds (extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l) r)
      (decreases (v (mk_usize 24) - v r))
  = if r =. mk_usize 24 then ()
    else begin
      lemma_one_round_to_spec v_N lc ks r l;
      lemma_rounds_to_spec v_N lc (impl_one_round v_N ks r) (r +! mk_usize 1) l
    end
#pop-options

(** Named fold body — matches the extracted lambda body in impl_2__keccakf1600
    (modulo identity let-bindings that normalize away). *)
let keccakf_body
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (self: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (i: usize{v i < 24})
  : Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T =
  let open Libcrux_sha3.Generic_keccak in
  let (tmp0: t_KeccakState v_N v_T), (out: t_Array v_T (mk_usize 5)) =
    impl_2__theta v_N #v_T self
  in
  let self: t_KeccakState v_N v_T = tmp0 in
  let t: t_Array v_T (mk_usize 5) = out in
  let self: t_KeccakState v_N v_T = impl_2__rho v_N #v_T self t in
  let self: t_KeccakState v_N v_T = impl_2__pi v_N #v_T self in
  let self: t_KeccakState v_N v_T = impl_2__chi v_N #v_T self in
  let self: t_KeccakState v_N v_T = impl_2__iota v_N #v_T self i in
  self

(** Fold wrapper with local bindings — amenable to lemma_fold_range_step. *)
let keccakf_fold_local
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (i: usize{v i <= 24})
  : Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T =
  let inv (_: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T) (_: usize) : Type0 = True in
  let f (self: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
        (j: usize{v j < 24}) : Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T =
    keccakf_body v_N self j
  in
  Rust_primitives.Hax.Folds.fold_range i (mk_usize 24) inv ks f

(** Recursive bridge: keccakf_fold_local == impl_rounds. *)
#push-options "--fuel 1 --z3rlimit 200"
let rec lemma_keccakf_fold_local_is_rounds
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (i: usize)
  : Lemma
      (requires i <=. mk_usize 24)
      (ensures keccakf_fold_local v_N ks i == impl_rounds v_N ks i)
      (decreases (v (mk_usize 24) - v i))
  = if i =. mk_usize 24 then ()
    else begin
      let inv (_: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T) (_: usize) : Type0 = True in
      let f (self: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
            (j: usize{v j < 24}) : Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T =
        keccakf_body v_N self j
      in
      Proof_Utils.FoldRange.lemma_fold_range_step i (mk_usize 24) inv ks f;
      lemma_keccakf_fold_local_is_rounds v_N (f ks i) (i +! mk_usize 1)
    end
#pop-options

(** Nat-indexed body matching [keccakf_body] (lifted to nat index). *)
let keccakf_body_rnat
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (self: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (j: nat{0 <= j /\ j < 24})
    : Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T =
  keccakf_body v_N #v_T self (mk_usize j)

(** [keccakf_body] equals [impl_one_round]. Both unfold to the same
    sequence of theta/rho/pi/chi/iota calls; extractor's extra identity
    let-bindings in [keccakf_body] normalize away via zeta/iota. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_keccakf_body_is_one_round
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (i: usize{v i < 24})
    : Lemma (keccakf_body v_N #v_T ks i == impl_one_round v_N #v_T ks i)
    = ()
#pop-options

(** Inductive bridge: the [fold_range_nat] iteration of [keccakf_body_rnat]
    equals [impl_rounds]. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let rec lemma_fold_range_nat_is_impl_rounds
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (i: nat{0 <= i /\ i <= 24})
    : Lemma
        (ensures fold_range_nat 0 24 i ks (keccakf_body_rnat v_N #v_T) ==
                 impl_rounds v_N #v_T ks (mk_usize i))
        (decreases 24 - i)
    = if i = 24 then ()
      else begin
        lemma_keccakf_body_is_one_round v_N #v_T ks (mk_usize i);
        lemma_fold_range_nat_is_impl_rounds v_N #v_T
          (keccakf_body_rnat v_N #v_T ks i) (i + 1)
      end
#pop-options

(** Bridge lemma: the extracted [impl_2__keccakf1600] (a refined [fold_range]
    with inline lambda body) equals the recursive [impl_rounds] helper.

    Two-step proof:
      (A) Apply [lemma_fold_range_is_range_nat] with the SAME inline lambdas
          the extractor produces — F* matches syntactically. This rewrites
          the refined [fold_range] as [fold_range_nat 0 24 0 ks body_rnat].
      (B) Apply [lemma_fold_range_nat_is_impl_rounds] to relate the
          nat-indexed fold to [impl_rounds]. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let lemma_keccakf1600_is_rounds
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
  : Lemma (Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 v_N #v_T ks ==
           impl_rounds v_N ks (mk_usize 0))
  =
    (* (A) Rewrite the extracted fold_range as a fold_range_nat via the
       bridge. Inline lambdas must match the extractor's shape verbatim. *)
    lemma_fold_range_is_range_nat
      #(Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T) #USIZE
      (mk_usize 0) (mk_usize 24) (mk_usize 0)
      (fun self temp_1_ ->
          let self: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T = self in
          let _: usize = temp_1_ in
          true)
      ks
      (fun self i ->
          let self: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T = self in
          let i: usize = i in
          let (tmp0: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T),
              (out: t_Array v_T (mk_usize 5)) =
            Libcrux_sha3.Generic_keccak.impl_2__theta v_N #v_T self
          in
          let self: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T = tmp0 in
          let t: t_Array v_T (mk_usize 5) = out in
          let self: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T =
            Libcrux_sha3.Generic_keccak.impl_2__rho v_N #v_T self t in
          let self: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T =
            Libcrux_sha3.Generic_keccak.impl_2__pi v_N #v_T self in
          let self: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T =
            Libcrux_sha3.Generic_keccak.impl_2__chi v_N #v_T self in
          let self: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T =
            Libcrux_sha3.Generic_keccak.impl_2__iota v_N #v_T self i in
          self)
      (keccakf_body_rnat v_N #v_T)
      (fun acc i -> ());   (* pointwise: both sides β-reduce to keccakf_body v_N acc i *)
    (* (B) Relate the nat-fold to impl_rounds. *)
    lemma_fold_range_nat_is_impl_rounds v_N #v_T ks 0
#pop-options

(** Bridge: spec's [keccak_f] equals [spec_rounds]. Re-exported from
    [EquivImplSpec.Keccakf.SpecRounds] which isolates the fragile [fuel 25]
    setting from the surrounding SMT context here. *)
let lemma_keccak_f_is_rounds = SpecRounds.lemma_keccak_f_is_rounds

(* ================================================================
   MAIN THEOREM: Generic keccak_f lane-wise equivalence.

   For any KeccakItem implementation satisfying lane_correctness,
   extracting lane l from keccakf1600 equals running the scalar
   keccak_f on lane l of the input.
   ================================================================ *)

let lemma_keccakf1600_to_spec
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (lc: lane_correctness v_N #v_T)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (l: nat{l < v v_N})
  : Lemma
      (extract_lane v_N lc
        (Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 v_N #v_T ks)
          .Libcrux_sha3.Generic_keccak.f_st l ==
       Hacspec_sha3.Keccak_f.keccak_f
        (extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l))
  = lemma_keccakf1600_is_rounds v_N ks;
    lemma_keccak_f_is_rounds (extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l);
    lemma_rounds_to_spec v_N lc ks (mk_usize 0) l

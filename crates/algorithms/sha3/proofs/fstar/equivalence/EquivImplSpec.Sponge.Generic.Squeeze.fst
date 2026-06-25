module EquivImplSpec.Sponge.Generic.Squeeze

(* ================================================================
   Generic squeeze-phase proof for any KeccakItem backend.

   Provides:
   - spec_squeeze_loop: recursive spec-side squeeze loop (scalar)
   - Per-step lemmas for squeeze commutativity via extract_lane:
     * lemma_squeeze_once_generic: single squeeze (no keccakf)
     * lemma_squeeze_kf_step_generic: keccakf then squeeze
   - Inductive loop equivalence via lockstep induction
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models
open FStar.Tactics.Typeclasses

module G = EquivImplSpec.Keccakf.Generic
module SC = EquivImplSpec.Sponge.Generic.Core

(* Bring typeclass instances into scope *)
let _ =
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Portable in
  ()


(* ================================================================
   Spec-side recursive squeeze loop (scalar).
   Each step: keccak_f then squeeze_state.
   Starts at block index i, goes to output_blocks.
   ================================================================ *)

let rec spec_squeeze_loop
      (state: SC.spec_state)
      (output: t_Slice u8)
      (rate: usize)
      (i output_blocks: usize)
  : Pure (t_Slice u8 & SC.spec_state)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v i <= v output_blocks /\
        v output_blocks * v rate <= Seq.length output)
      (ensures fun res ->
        let output', _ = res in
        Seq.length output' == Seq.length output)
      (decreases (v output_blocks - v i))
  = if i =. output_blocks then (output, state)
    else
      let _: Prims.unit =
        Libcrux_sha3.Proof_utils.Lemmas.lemma_mul_succ_le i output_blocks rate
      in
      let state' = Hacspec_sha3.Keccak_f.keccak_f state in
      let output_len = Core_models.Slice.impl__len #u8 output in
      let output' =
        Hacspec_sha3.Sponge.squeeze_state output_len state'
          (output <: t_Array u8 output_len) (i *! rate) rate
      in
      spec_squeeze_loop state' output' rate (i +! mk_usize 1) output_blocks


(* ================================================================
   Per-step: single squeeze (no keccakf) commutes with extract_lane.

   sq_lane(rate, state, outputs, start, len, l)
     == squeeze_state(extract_lane(state, l), outputs[l], start, len)

   This is a direct consequence of sc.sc_store_block.
   ================================================================ *)

let lemma_squeeze_once_generic
      (v_N: usize) (#v_T: Type0)
      {| ki: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      {| ab: Libcrux_sha3.Traits.t_Absorb
               (Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T) v_N |}
      (lc: G.lane_correctness v_N #v_T)
      (sc: SC.sponge_correctness v_N #v_T lc)
      (state: t_Array v_T (mk_usize 25))
      (outputs: t_Array (t_Slice u8) v_N)
      (rate: usize)
      (start len: usize)
      (l: nat{l < v v_N})
  : Lemma
      (requires
        v v_N > 0 /\
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 (outputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len v_N outputs)
      (ensures
        sc.sq_lane rate state outputs start len l
        ==
        Hacspec_sha3.Sponge.squeeze_state
          (Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]))
          (G.extract_lane v_N lc state l)
          (outputs.[ mk_usize l ] <: t_Array u8 _)
          start
          len)
  = sc.sc_store_block rate state outputs start len l


(* ================================================================
   Per-step: keccakf then squeeze commutes with extract_lane.

   For lane l:
     extract_lane(keccakf(ks).f_st, l)  == keccak_f(extract_lane(ks.f_st, l))
     sq_lane(rate, keccakf(ks).f_st, outputs, start, len, l)
       == squeeze_state(keccak_f(extract_lane(ks.f_st, l)), outputs[l], start, len)
   ================================================================ *)

#push-options "--fuel 0 --z3rlimit 200"
let lemma_squeeze_kf_step_generic
      (v_N: usize) (#v_T: Type0)
      {| ki: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      {| ab: Libcrux_sha3.Traits.t_Absorb
               (Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T) v_N |}
      (lc: G.lane_correctness v_N #v_T)
      (sc: SC.sponge_correctness v_N #v_T lc)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
      (outputs: t_Array (t_Slice u8) v_N)
      (rate: usize)
      (start len: usize)
      (l: nat{l < v v_N})
  : Lemma
      (requires
        v v_N > 0 /\
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 (outputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len v_N outputs)
      (ensures (
        let ks' = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 v_N #v_T ks in
        (* State commutativity *)
        G.extract_lane v_N lc ks'.Libcrux_sha3.Generic_keccak.f_st l
        ==
        Hacspec_sha3.Keccak_f.keccak_f
          (G.extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l)
        /\
        (* Squeeze commutativity *)
        sc.sq_lane rate ks'.Libcrux_sha3.Generic_keccak.f_st outputs start len l
        ==
        Hacspec_sha3.Sponge.squeeze_state
          (Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]))
          (Hacspec_sha3.Keccak_f.keccak_f
            (G.extract_lane v_N lc ks.Libcrux_sha3.Generic_keccak.f_st l))
          (outputs.[ mk_usize l ] <: t_Array u8 _)
          start
          len))
  =
  (* Step 1: keccakf1600 commutes with extract_lane *)
  G.lemma_keccakf1600_to_spec v_N lc ks l;
  (* Step 2: squeeze commutes with extract_lane on the post-keccakf state *)
  let ks' = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 v_N #v_T ks in
  sc.sc_store_block rate ks'.Libcrux_sha3.Generic_keccak.f_st outputs start len l
#pop-options

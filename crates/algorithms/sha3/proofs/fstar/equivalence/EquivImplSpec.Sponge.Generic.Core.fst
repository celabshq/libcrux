module EquivImplSpec.Sponge.Generic.Core

(* ================================================================
   Generic sponge-layer equivalence for any KeccakItem backend.

   Builds on:
   - EquivImplSpec.Keccakf.Generic.lemma_keccakf1600_to_spec:
       extract_lane(keccakf1600(ks).f_st, l) == keccak_f(extract_lane(ks.f_st, l))
   - Hacspec_sha3.Sponge spec functions (scalar, on t_Array u64 25)

   This file defines:
   1. Generic type aliases
   2. The `sponge_correctness` record — per-lane commutativity hypotheses
      for the Absorb and Squeeze trait operations, analogous to
      `lane_correctness` for KeccakItem operations.
   3. Reusable helpers (lane_index, rate constants, etc.)

   ARCHITECTURE:

   The sponge_correctness record has four fields:

   sq_lane:       per-lane extraction of the backend squeeze output
   sc_load_block: extract_lane after f_load_block == xor_block_into_state on extracted lane
   sc_load_last:  extract_lane after f_load_last  == pad_last_block + xor_block_into_state
   sc_store_block: sq_lane matches scalar squeeze_state on extracted lane

   Each backend (Portable, Arm64, AVX2) instantiates this record once.
   The generic absorb/squeeze proofs thread these through the loop
   structure via lockstep induction (fuel 1).
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models
open FStar.Tactics.Typeclasses

module G = EquivImplSpec.Keccakf.Generic

(* Bring typeclass instances into scope for resolution *)
let _ =
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Portable in
  ()


(* ================================================================
   Type aliases
   ================================================================ *)

let spec_state = t_Array u64 (mk_usize 25)


(* ================================================================
   Lane index equivalence (reused from EquivImplSpec.Sponge.Core, N-independent)

   After the FIPS-native layout flip, [lane_index] was removed from the
   Rust spec (it collapsed to the identity and was inlined). The impl
   writes lane [i] at flat index [5*(i/5) + i%5 = i] under
   [get_ij(N, state, i/5, i%5) = state[5*(i/5) + i%5]].

   [lemma_lane_index_is_impl_index] now reflects that identity:
   spec-side lane [i] sits at impl flat index [i], i.e.
   [5*(i/5) + i%5 = i]. Consumers that formerly called
   [Hacspec_sha3.Sponge.lane_index i] must now use [i] directly. *)

let lemma_lane_index_is_impl_index (i: usize)
  : Lemma (requires v i < 25)
          (ensures  i ==
                    (mk_usize 5 *! (i /! mk_usize 5 <: usize) <: usize) +!
                    (i %! mk_usize 5 <: usize))
  = ()


(* ================================================================
   Rate and Delimiter Constant Equivalence (N-independent)
   ================================================================ *)

let lemma_sha3_224_rate  (_:unit) : Lemma (mk_usize 144 == Hacspec_sha3.Sha3.v_SHA3_224_RATE)  = ()
let lemma_sha3_256_rate  (_:unit) : Lemma (mk_usize 136 == Hacspec_sha3.Sha3.v_SHA3_256_RATE)  = ()
let lemma_sha3_384_rate  (_:unit) : Lemma (mk_usize 104 == Hacspec_sha3.Sha3.v_SHA3_384_RATE)  = ()
let lemma_sha3_512_rate  (_:unit) : Lemma (mk_usize 72  == Hacspec_sha3.Sha3.v_SHA3_512_RATE)  = ()
let lemma_shake128_rate  (_:unit) : Lemma (mk_usize 168 == Hacspec_sha3.Sha3.v_SHAKE128_RATE)  = ()
let lemma_shake256_rate  (_:unit) : Lemma (mk_usize 136 == Hacspec_sha3.Sha3.v_SHAKE256_RATE)  = ()
let lemma_sha3_delim     (_:unit) : Lemma (mk_u8 6  == Hacspec_sha3.Sha3.v_SHA3_DELIM)         = ()
let lemma_shake_delim    (_:unit) : Lemma (mk_u8 31 == Hacspec_sha3.Sha3.v_SHAKE_DELIM)        = ()


(* ================================================================
   Sponge correctness record

   Abstracts over per-lane commutativity of the sponge operations:
   - f_load_block (Absorb trait)
   - f_load_last  (Absorb trait)
   - f_squeeze / f_squeeze2 (Squeeze / Squeeze2 traits)

   This is the sponge-layer analogue of lane_correctness.

   The squeeze API differs per backend:
   - N=1 (Portable): f_squeeze takes one output slice
   - N=2 (Arm64):    f_squeeze2 takes two output slices
   - N=4 (AVX2):     (future)

   The `sq_lane` field abstracts over this difference: each backend
   provides a function that, given the SIMD state and the array of
   all N output slices, returns the updated output for lane l after
   squeezing.  The `sc_store_block` lemma then proves this matches
   the scalar `squeeze_state` on the extracted lane.
   ================================================================ *)

noeq type sponge_correctness
  (v_N: usize)
  (#v_T: Type0)
  {| ki: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
  {| ab: Libcrux_sha3.Traits.t_Absorb
           (Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T) v_N |}
  (lc: G.lane_correctness v_N #v_T) = {

  (* ----- sq_lane: per-lane squeeze extraction ----- *)

  (* Each backend defines this concretely:
     - Portable (N=1): sq_lane rate state outputs start len 0
         = f_squeeze(rate, {f_st=state}, outputs.[0], start, len)
     - Arm64 (N=2):    sq_lane rate state outputs start len l
         = (f_squeeze2(rate, {f_st=state}, outputs.[0], outputs.[1], start, len)).l
  *)
  sq_lane:
    (rate: usize) ->
    (state: t_Array v_T (mk_usize 25)) ->
    (outputs: t_Array (t_Slice u8) v_N) ->
    (start: usize) ->
    (len: usize) ->
    (l: nat{l < v v_N}) ->
    Pure (t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 (outputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len v_N outputs)
      (ensures fun r -> Seq.length #u8 r == Seq.length #u8 (outputs.[ mk_usize l ]));


  (* ----- load_block: absorb one full rate-byte block ----- *)

  (* Per-lane: after f_load_block, the extracted lane equals the scalar
     xor_block_into_state applied to that lane's input slice.

     impl: f_load_block(rate, {f_st=state}, inputs, start) -- modifies all N lanes
     spec: xor_block_into_state(state_l, inputs[l][start..start+rate], rate)
  *)
  sc_load_block:
    (rate: usize) ->
    (state: t_Array v_T (mk_usize 25)) ->
    (inputs: t_Array (t_Slice u8) v_N) ->
    (start: usize) ->
    (l: nat{l < v v_N}) ->
    Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v start + v rate <= Seq.length #u8 (inputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len v_N inputs)
      (ensures
        G.extract_lane v_N lc
          (Libcrux_sha3.Traits.f_load_block #_ #_ #ab rate
             ({ Libcrux_sha3.Generic_keccak.f_st = state }
               <: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
             inputs
             start)
            .Libcrux_sha3.Generic_keccak.f_st
          l
        ==
        Hacspec_sha3.Sponge.xor_block_into_state
          (G.extract_lane v_N lc state l)
          (inputs.[ mk_usize l ].[ {
              Core_models.Ops.Range.f_start = start;
              Core_models.Ops.Range.f_end   = start +! rate } <:
            Core_models.Ops.Range.t_Range usize ])
          rate);


  (* ----- load_last: absorb the final partial block with padding ----- *)

  (* Per-lane: after f_load_last, the extracted lane equals
     xor_block_into_state(state_l, pad_last_block(input_l, start, len, rate, delim)[0..rate], rate)
     The [0..rate] slice matches the spec-side [absorb_final]'s call shape. *)
  sc_load_last:
    (rate: usize) ->
    (delim: u8) ->
    (state: t_Array v_T (mk_usize 25)) ->
    (inputs: t_Array (t_Slice u8) v_N) ->
    (start: usize) ->
    (len: usize) ->
    (l: nat{l < v v_N}) ->
    Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len < v rate /\
        v start + v len <= Seq.length #u8 (inputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len v_N inputs)
      (ensures (
        let padded: t_Array u8 (mk_usize 200) =
          Hacspec_sha3.Sponge.pad_last_block
            (inputs.[ mk_usize l ]) start len rate delim
        in
        G.extract_lane v_N lc
          (Libcrux_sha3.Traits.f_load_last #_ #_ #ab rate delim
             ({ Libcrux_sha3.Generic_keccak.f_st = state }
               <: Libcrux_sha3.Generic_keccak.t_KeccakState v_N v_T)
             inputs
             start
             len)
            .Libcrux_sha3.Generic_keccak.f_st
          l
        ==
        Hacspec_sha3.Sponge.xor_block_into_state
          (G.extract_lane v_N lc state l)
          (padded.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end   = rate } <:
                    Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
          rate));


  (* ----- store_block: squeeze bytes from state ----- *)

  (* Per-lane: the backend squeeze output for lane l (given by sq_lane)
     matches the scalar squeeze_state on the extracted lane.
  *)
  sc_store_block:
    (rate: usize) ->
    (state: t_Array v_T (mk_usize 25)) ->
    (outputs: t_Array (t_Slice u8) v_N) ->
    (start: usize) ->
    (len: usize) ->
    (l: nat{l < v v_N}) ->
    Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 (outputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len v_N outputs)
      (ensures
        sq_lane rate state outputs start len l
        ==
        Hacspec_sha3.Sponge.squeeze_state
          (Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]))
          (G.extract_lane v_N lc state l)
          (outputs.[ mk_usize l ] <: t_Array u8 _)
          start
          len);
}

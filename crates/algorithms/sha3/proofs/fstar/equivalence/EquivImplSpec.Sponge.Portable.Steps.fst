module EquivImplSpec.Sponge.Portable.Steps

(* ================================================================
   Per-step equivalences for the Portable (N=1, v_T=u64) backend.

   Four step lemmas connecting single-block impl operations to the
   scalar Hacspec spec:

   - [lemma_absorb_block_portable] : impl_2__absorb_block ≡ spec absorb_block
   - [lemma_absorb_last_portable]  : impl_2__absorb_final ≡ spec absorb_final
   - [lemma_squeeze_block_portable]: keccakf1600 ; f_squeeze (len=rate) ≡
                                     keccak_f ; squeeze_state (len=rate)
   - [lemma_squeeze_last_portable] : keccakf1600 ; f_squeeze (len<rate) ≡
                                     keccak_f ; squeeze_state (len<rate)

   Absorb-side proofs ([lemma_absorb_block_portable],
   [lemma_absorb_last_portable]) use the pointwise [load_block] ensures
   (proved on the Rust side via hax_lib::ensures) together with the
   [createi_lemma] SMT pattern on the [createi]-form spec
   [xor_block_into_state]. No admit is reached on this side.

   Squeeze-side proofs still compose the admitted [portable_sc_store_block]
   at lane l=0 with the N=1 extract-lane identity and
   [lemma_keccakf1600_portable].
   ================================================================ *)

#set-options "--fuel 1 --ifuel 1 --z3rlimit 150"

open FStar.Mul
open Core_models

module G  = EquivImplSpec.Keccakf.Generic
module KP = EquivImplSpec.Keccakf.Portable
module SP = EquivImplSpec.Sponge.Portable

(* Bring Portable typeclass instances into scope so
   t_KeccakItem u64 1 / t_Absorb / t_Squeeze resolve. *)
let _ =
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Portable in
  ()


(* ================================================================
   Step 1: impl_2__absorb_block ≡ spec absorb_block.
   ================================================================ *)
#push-options "--z3rlimit 200"
let lemma_absorb_block_portable
      (rate: usize)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (inputs: t_Array (t_Slice u8) (mk_usize 1))
      (start: usize)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v start + v rate <= Seq.length #u8 (inputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 1) inputs)
      (ensures
        (Libcrux_sha3.Generic_keccak.impl_2__absorb_block
           (mk_usize 1) #u64 rate ks inputs start)
          .Libcrux_sha3.Generic_keccak.f_st
        ==
        Hacspec_sha3.Sponge.absorb_block
          ks.Libcrux_sha3.Generic_keccak.f_st
          (inputs.[ mk_usize 0 ].[ {
              Core_models.Ops.Range.f_start = start;
              Core_models.Ops.Range.f_end   = start +! rate } <:
            Core_models.Ops.Range.t_Range usize ])
          rate)
  = let state = ks.Libcrux_sha3.Generic_keccak.f_st in
    let input0 = inputs.[ mk_usize 0 ] in
    SP.lemma_load_block_eq_xor_block_into_state rate state input0 start;
    let s1 =
      Libcrux_sha3.Traits.f_load_block
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        #(mk_usize 1)
        #FStar.Tactics.Typeclasses.solve
        rate ks inputs start in
    KP.lemma_keccakf1600_portable s1
#pop-options


(* ================================================================
   Step 2: impl_2__absorb_final ≡ spec absorb_final.

   impl_2__absorb_final 1 #u64 rate delim ks inputs start len
     = f_load_last ... ks inputs start len
     |> impl_2__keccakf1600 1 #u64

   [load_last] internally builds a [rate]-sized padded buffer and calls
   [load_block rate state buffer 0]; the spec's [pad_last_block] builds
   a 200-sized buffer with identical bytes at positions [0..rate].
   Since [xor_block_into_state] only reads positions [< rate], the two
   padded buffers agree on the bytes actually consumed.

   Spec:
     absorb_final state input off len rate delim
     = keccak_f (xor_block_into_state state (pad_last_block ...) rate)

   Proof: same pointwise strategy as [lemma_absorb_block_portable], but
   we step through the internal [load_block] call on the padded buffer.
   ================================================================ *)
(* ================================================================
   Step 2: impl_2__absorb_final ≡ spec absorb_final.
   ================================================================ *)
#push-options "--z3rlimit 200"
let lemma_absorb_last_portable
      (rate: usize)
      (delim: u8)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (inputs: t_Array (t_Slice u8) (mk_usize 1))
      (start: usize)
      (len: usize)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len < v rate /\
        v start + v len <= Seq.length #u8 (inputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 1) inputs)
      (ensures
        (Libcrux_sha3.Generic_keccak.impl_2__absorb_final
           (mk_usize 1) #u64 rate delim ks inputs start len)
          .Libcrux_sha3.Generic_keccak.f_st
        ==
        Hacspec_sha3.Sponge.absorb_final
          ks.Libcrux_sha3.Generic_keccak.f_st
          (inputs.[ mk_usize 0 ])
          start len rate delim)
  = let state = ks.Libcrux_sha3.Generic_keccak.f_st in
    let input0 = inputs.[ mk_usize 0 ] in
    SP.lemma_load_last_eq_xor_block_into_state_padded rate delim state input0 start len;
    let s1 =
      Libcrux_sha3.Traits.f_load_last
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        #(mk_usize 1)
        #FStar.Tactics.Typeclasses.solve
        rate delim ks inputs start len in
    KP.lemma_keccakf1600_portable s1
#pop-options


(* ================================================================
   Step 3: a full-rate squeeze block preceded by a permutation.

   Matches one iteration of the [1 .. output_blocks) middle-loop in
   both [Libcrux_sha3.Generic_keccak.Portable.squeeze] and
   [Hacspec_sha3.Sponge.squeeze]:

     impl side: keccakf1600 ; f_squeeze (start, RATE)
     spec side: keccak_f   ; squeeze_state (start, RATE)

   Both sides produce the same new state (after keccak_f) and the
   same output slice.
   ================================================================ *)
let lemma_squeeze_block_portable
      (rate: usize)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (out: t_Slice u8)
      (start: usize)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v start + v rate <= Seq.length #u8 out)
      (ensures (
        let ks' = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
                    (mk_usize 1) #u64 ks in
        let out' = Libcrux_sha3.Traits.f_squeeze
                     #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
                     #u64
                     #FStar.Tactics.Typeclasses.solve
                     rate ks' out start rate in
        let state' = Hacspec_sha3.Keccak_f.keccak_f
                       ks.Libcrux_sha3.Generic_keccak.f_st in
        ks'.Libcrux_sha3.Generic_keccak.f_st == state' /\
        out' == Hacspec_sha3.Sponge.squeeze_state
                  (Core_models.Slice.impl__len #u8 out) state'
                  (out <: t_Array u8 _) start rate))
  = let state  = ks.Libcrux_sha3.Generic_keccak.f_st in
    KP.lemma_keccakf1600_portable ks;
    let ks' = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
                (mk_usize 1) #u64 ks in
    let state' = ks'.Libcrux_sha3.Generic_keccak.f_st in
    (* store_block at l=0 via the Sponge.Portable admit (collapses to identity). *)
    let outputs : t_Array (t_Slice u8) (mk_usize 1) =
      let list = [out] in
      FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
      Rust_primitives.Hax.array_of_list 1 list in
    assert (outputs.[ mk_usize 0 ] == out);
    SP.portable_sc_store_block rate state' outputs start rate 0;
    KP.lemma_extract_lane_portable_identity state'


(* ================================================================
   Step 4: a partial-rate trailing squeeze preceded by a permutation.

   Matches the [output_rem ≠ 0] tail branch in both
   [Libcrux_sha3.Generic_keccak.Portable.squeeze] and
   [Hacspec_sha3.Sponge.squeeze]:

     impl side: keccakf1600 ; f_squeeze (start, len)    with len < rate
     spec side: keccak_f   ; squeeze_state (start, len) with len < rate
   ================================================================ *)
let lemma_squeeze_last_portable
      (rate: usize)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (out: t_Slice u8)
      (start: usize)
      (len: usize)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 out)
      (ensures (
        let ks' = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
                    (mk_usize 1) #u64 ks in
        let out' = Libcrux_sha3.Traits.f_squeeze
                     #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
                     #u64
                     #FStar.Tactics.Typeclasses.solve
                     rate ks' out start len in
        let state' = Hacspec_sha3.Keccak_f.keccak_f
                       ks.Libcrux_sha3.Generic_keccak.f_st in
        ks'.Libcrux_sha3.Generic_keccak.f_st == state' /\
        out' == Hacspec_sha3.Sponge.squeeze_state
                  (Core_models.Slice.impl__len #u8 out) state'
                  (out <: t_Array u8 _) start len))
  = KP.lemma_keccakf1600_portable ks;
    let ks' = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
                (mk_usize 1) #u64 ks in
    let state' = ks'.Libcrux_sha3.Generic_keccak.f_st in
    let outputs : t_Array (t_Slice u8) (mk_usize 1) =
      let list = [out] in
      FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
      Rust_primitives.Hax.array_of_list 1 list in
    assert (outputs.[ mk_usize 0 ] == out);
    SP.portable_sc_store_block rate state' outputs start len 0;
    KP.lemma_extract_lane_portable_identity state'


(* [portable_squeeze_composed] — DELETED with the byteform migration
   (Note C in proof_milestones.md).  Was a sugar for the recursive
   [squeeze_blocks] / [squeeze_last] composition; consumers now cite
   [Hacspec_sha3.Sponge.squeeze] (byteform) directly. *)

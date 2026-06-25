module EquivImplSpec.Sponge.Portable

(* ================================================================
   Portable (N=1, v_T=u64) instantiation of the generic sponge-layer
   equivalence proof.

   Builds a [sponge_correctness] record for
   [Libcrux_sha3.Simd.Portable.{impl_1, impl_2}]: the t_Absorb and
   t_Squeeze typeclass instances at N=1.

   The three per-backend lemmas — sc_load_block, sc_load_last,
   sc_store_block — are proven here (real [let]s below). They reduce to:
     f_load_block / f_load_last  ≡ scalar xor_block_into_state
     f_squeeze                   ≡ scalar squeeze_state
   under the N=1 identity for extract_lane, by composing the closed
   bridge lemmas (reused from [EquivImplSpec.Sponge.Portable.Steps]) with
   [lemma_extract_lane_portable_identity].
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models

module G  = EquivImplSpec.Keccakf.Generic
module P  = EquivImplSpec.Keccakf.Portable
module SC = EquivImplSpec.Sponge.Generic.Core

(* Bring Portable typeclass instances into scope. *)
let _ =
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Portable in
  ()

(* ================================================================
   sq_lane for N=1: single-lane squeeze via t_Squeeze.f_squeeze.
   ================================================================ *)

let sq_lane_portable
      (rate: usize)
      (state: t_Array u64 (mk_usize 25))
      (outputs: t_Array (t_Slice u8) (mk_usize 1))
      (start: usize)
      (len: usize)
      (l: nat{l < 1})
  : Pure (t_Slice u8)
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 (outputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 1) outputs)
      (ensures fun r -> Seq.length #u8 r == Seq.length #u8 (outputs.[ mk_usize l ]))
  = Libcrux_sha3.Traits.f_squeeze
      #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      #u64
      #FStar.Tactics.Typeclasses.solve
      rate
      ({ Libcrux_sha3.Generic_keccak.f_st = state }
         <: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (outputs.[ mk_usize 0 ])
      start
      len

(* ================================================================
   Bridge lemmas: pointwise equivalence of the Portable [load_block]
   and [load_last] with the scalar spec [xor_block_into_state] (a
   [createi] on the extraction side).

   These closed lemmas are reused from [Sponge.Portable.Steps.fst] and
   by the [sc_load_block] / [sc_load_last] proofs below.
   ================================================================ *)

(* Slice-of-slice byte equality:
   (blocks[start..start+rate])[8k..8k+8] == blocks[start+8k..start+8k+8] *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_subslice_bytes_eq
      (blocks: t_Slice u8) (start: usize) (rate: usize) (k: usize)
  : Lemma
      (requires
        v start + v rate <= Seq.length #u8 blocks /\
        v rate % 8 == 0 /\
        v k < v rate / 8 /\
        v start + 8 * v k + 8 <= Seq.length #u8 blocks)
      (ensures (
        let block : t_Slice u8 =
          blocks.[ { Core_models.Ops.Range.f_start = start;
                     Core_models.Ops.Range.f_end   = start +! rate } <:
                   Core_models.Ops.Range.t_Range usize ] in
        let lhs : t_Slice u8 =
          block.[ { Core_models.Ops.Range.f_start = mk_usize 8 *! k;
                    Core_models.Ops.Range.f_end   =
                      (mk_usize 8 *! k <: usize) +! mk_usize 8 } <:
                  Core_models.Ops.Range.t_Range usize ] in
        let rhs : t_Slice u8 =
          blocks.[ { Core_models.Ops.Range.f_start =
                       start +! (mk_usize 8 *! k <: usize);
                     Core_models.Ops.Range.f_end   =
                       (start +! (mk_usize 8 *! k <: usize) <: usize)
                       +! mk_usize 8 } <:
                   Core_models.Ops.Range.t_Range usize ] in
        lhs == rhs))
  = let block : t_Slice u8 =
      blocks.[ { Core_models.Ops.Range.f_start = start;
                 Core_models.Ops.Range.f_end   = start +! rate } <:
               Core_models.Ops.Range.t_Range usize ] in
    let lhs : t_Slice u8 =
      block.[ { Core_models.Ops.Range.f_start = mk_usize 8 *! k;
                Core_models.Ops.Range.f_end   =
                  (mk_usize 8 *! k <: usize) +! mk_usize 8 } <:
              Core_models.Ops.Range.t_Range usize ] in
    let rhs : t_Slice u8 =
      blocks.[ { Core_models.Ops.Range.f_start =
                   start +! (mk_usize 8 *! k <: usize);
                 Core_models.Ops.Range.f_end   =
                   (start +! (mk_usize 8 *! k <: usize) <: usize)
                   +! mk_usize 8 } <:
               Core_models.Ops.Range.t_Range usize ] in
    let aux (m: nat{m < 8}) : Lemma (Seq.index lhs m == Seq.index rhs m) = () in
    Classical.forall_intro aux;
    assert (Seq.equal lhs rhs)
#pop-options

(* Core pointwise equivalence: [load_block] ≡ [xor_block_into_state]. *)
#push-options "--z3rlimit 200"
let lemma_load_block_eq_xor_block_into_state
      (rate: usize)
      (state: t_Array u64 (mk_usize 25))
      (blocks: t_Slice u8)
      (start: usize)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v start + v rate <= Seq.length #u8 blocks)
      (ensures (
        let block : t_Slice u8 =
          blocks.[ { Core_models.Ops.Range.f_start = start;
                     Core_models.Ops.Range.f_end   = start +! rate } <:
                   Core_models.Ops.Range.t_Range usize ] in
        Libcrux_sha3.Simd.Portable.load_block rate state blocks start
        == Hacspec_sha3.Sponge.xor_block_into_state state block rate))
  = let block : t_Slice u8 =
      blocks.[ { Core_models.Ops.Range.f_start = start;
                 Core_models.Ops.Range.f_end   = start +! rate } <:
               Core_models.Ops.Range.t_Range usize ] in
    let lb = Libcrux_sha3.Simd.Portable.load_block rate state blocks start in
    let spec_state = Hacspec_sha3.Sponge.xor_block_into_state state block rate in
    let byte_eq (i: nat{i < 25}) : Lemma (Seq.index lb i == Seq.index spec_state i) =
      let ii = mk_usize i in
      if v ii < v (rate /! mk_usize 8 <: usize)
      then lemma_subslice_bytes_eq blocks start rate ii
    in
    Classical.forall_intro byte_eq;
    Rust_primitives.Arrays.eq_intro lb spec_state
#pop-options

(* Helper: [load_last] = [load_block] on [pad_last_block]. *)
#push-options "--z3rlimit 200 --split_queries always"
let lemma_load_last_equals_load_block_on_padded
      (rate: usize)
      (delim: u8)
      (state: t_Array u64 (mk_usize 25))
      (blocks: t_Slice u8)
      (start: usize)
      (len: usize)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len < v rate /\
        v start + v len <= Seq.length #u8 blocks)
      (ensures
        Libcrux_sha3.Simd.Portable.load_last rate delim state blocks start len
        ==
        Libcrux_sha3.Simd.Portable.load_block rate state
          (Seq.slice 
            ((Hacspec_sha3.Sponge.pad_last_block blocks start len rate delim)
             <: t_Slice u8) 0 (v rate))
          (mk_usize 0))
  = 
    let copy_src : t_Slice u8 =
      blocks.[ { Core_models.Ops.Range.f_start = start;
                 Core_models.Ops.Range.f_end   = start +! len } <:
               Core_models.Ops.Range.t_Range usize ] in
    let b0 : t_Array u8 rate = Rust_primitives.Hax.repeat (mk_u8 0) rate in
    let b1 : t_Array u8 rate =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_range b0
        ({ Core_models.Ops.Range.f_start = mk_usize 0;
           Core_models.Ops.Range.f_end   = len } <:
         Core_models.Ops.Range.t_Range usize)
        (Core_models.Slice.impl__copy_from_slice #u8
            (b0.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                    Core_models.Ops.Range.f_end   = len } <:
                  Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
            copy_src <: t_Slice u8) in
    Proof_Utils.Lemmas.lemma_index_update_at_range b0 
      ({ Core_models.Ops.Range.f_start = mk_usize 0;
           Core_models.Ops.Range.f_end   = len } <:
         Core_models.Ops.Range.t_Range usize)
      copy_src;
    let b2 : t_Array u8 rate =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize b1 len delim in
    let b3 : t_Array u8 rate =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize b2
        (rate -! mk_usize 1 <: usize)
        ((b2.[ rate -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8) in
    let s0 : t_Array u8 (mk_usize 200) =
      Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 200) in
    let s1 : t_Array u8 (mk_usize 200) =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_range s0
        ({ Core_models.Ops.Range.f_start = mk_usize 0;
           Core_models.Ops.Range.f_end   = len } <:
         Core_models.Ops.Range.t_Range usize)
        (Core_models.Slice.impl__copy_from_slice #u8
            (s0.[ { Core_models.Ops.Range.f_end = len } <:
                  Core_models.Ops.Range.t_RangeTo usize ] <: t_Slice u8)
            copy_src <: t_Slice u8) in
    Proof_Utils.Lemmas.lemma_index_update_at_range s0 
      ({ Core_models.Ops.Range.f_start = mk_usize 0;
           Core_models.Ops.Range.f_end   = len } <:
         Core_models.Ops.Range.t_Range usize)
      copy_src;
    let s2 : t_Array u8 (mk_usize 200) =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s1 len delim in
    let s3 : t_Array u8 (mk_usize 200) =
      Rust_primitives.Hax.Monomorphized_update_at.update_at_usize s2
        (rate -! mk_usize 1 <: usize)
        ((s2.[ rate -! mk_usize 1 <: usize ] <: u8) |. mk_u8 128 <: u8) in
    let s4 : t_Slice u8 = Seq.slice s3 0 (v rate) in
    let b3_s3_byte_eq (m: nat{m < v rate})
      : Lemma (Seq.index (b3 <: Seq.seq u8) m == Seq.index (s3 <: Seq.seq u8) m)
      = () in
    Classical.forall_intro b3_s3_byte_eq;
    Rust_primitives.Arrays.eq_intro b3 s4;
    let impl_out = Libcrux_sha3.Simd.Portable.load_block rate state
                     (b3 <: t_Slice u8) (mk_usize 0) in
    let spec_out = Libcrux_sha3.Simd.Portable.load_block rate state
                     (s4 <: t_Slice u8) (mk_usize 0) in
    assert (Libcrux_sha3.Simd.Portable.load_last rate delim state blocks start len ==
            impl_out);
    assert(b3 == s4)
#pop-options

(* Compose: [load_last] ≡ [xor_block_into_state] on [pad_last_block[0..rate]].
   The [0..rate] slice matches [sc_load_last]'s sliced-buffer contract,
   which in turn matches the spec's [absorb_final] calling
   [absorb_block state padded[0..rate] rate]. *)
#push-options "--z3rlimit 400"
let lemma_load_last_eq_xor_block_into_state_padded
      (rate: usize)
      (delim: u8)
      (state: t_Array u64 (mk_usize 25))
      (blocks: t_Slice u8)
      (start: usize)
      (len: usize)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len < v rate /\
        v start + v len <= Seq.length #u8 blocks)
      (ensures (
        let padded : t_Array u8 (mk_usize 200) =
          Hacspec_sha3.Sponge.pad_last_block blocks start len rate delim in
        Libcrux_sha3.Simd.Portable.load_last rate delim state blocks start len
        ==
        Hacspec_sha3.Sponge.xor_block_into_state state
          (padded.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end   = rate } <:
                    Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
          rate))
  = let spec_buffer : t_Array u8 (mk_usize 200) =
      Hacspec_sha3.Sponge.pad_last_block blocks start len rate delim in
    let prefix : t_Slice u8 =
      spec_buffer.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end   = rate } <:
                    Core_models.Ops.Range.t_Range usize ] in
    (* lemma_load_last_equals_load_block_on_padded's ensures is already
       stated in terms of [Seq.slice padded 0 (v rate)] — which is the
       same underlying Seq.seq as [prefix]. Bridge through xor_block_into_state. *)
    lemma_load_last_equals_load_block_on_padded rate delim state blocks start len;
    lemma_load_block_eq_xor_block_into_state rate state prefix (mk_usize 0)
#pop-options


(* Core pointwise equivalence: [store_block] ≡ [squeeze_state].

   Both sides are characterized pointwise:
   - [store_block]'s Rust-level ensures gives the three-way case split
     (below [start] / in range / above [start+len]) per output byte.
   - [squeeze_state] is [update_at_range] on a [createi]-built source:
     [Proof_Utils.Lemmas.lemma_index_update_at_range] + the [createi_lemma]
     SMTPat reduce it to the same three-way pointwise form. *)
#push-options "--z3rlimit 300"
let lemma_store_block_eq_squeeze_state
      (rate: usize)
      (state: t_Array u64 (mk_usize 25))
      (out: t_Slice u8)
      (start: usize)
      (len: usize)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 out)
      (ensures
        Libcrux_sha3.Simd.Portable.store_block rate state out start len
        ==
        Hacspec_sha3.Sponge.squeeze_state
          (Core_models.Slice.impl__len #u8 out) state
          (out <: t_Array u8 _) start len)
  = let impl_out =
      Libcrux_sha3.Simd.Portable.store_block rate state out start len in
    let out_len = Core_models.Slice.impl__len #u8 out in
    let spec_out =
      Hacspec_sha3.Sponge.squeeze_state out_len state
        (out <: t_Array u8 out_len) start len in
    let aux (i:nat{i < Seq.length out}):
      Lemma(Seq.index impl_out i == Seq.index spec_out i) = 
      let sz_i = sz i in
      assert (v sz_i < Seq.length out);
      assert(Seq.index impl_out (v sz_i) == Seq.index spec_out (v sz_i))
    in
    Classical.forall_intro aux;
    Rust_primitives.Arrays.eq_intro impl_out spec_out
#pop-options 


(* ================================================================
   Per-backend lemmas — reduce f_load_block / f_load_last / f_squeeze
   on the Portable impl to the scalar spec operations.
   ================================================================ *)

let portable_sc_load_block
      (rate: usize)
      (state: t_Array u64 (mk_usize 25))
      (inputs: t_Array (t_Slice u8) (mk_usize 1))
      (start: usize)
      (l: nat{l < 1})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v start + v rate <= Seq.length #u8 (inputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 1) inputs)
      (ensures
        G.extract_lane (mk_usize 1) P.lc_portable
          (Libcrux_sha3.Traits.f_load_block
             #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
             #(mk_usize 1)
             #FStar.Tactics.Typeclasses.solve
             rate
             ({ Libcrux_sha3.Generic_keccak.f_st = state }
                <: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
             inputs
             start)
            .Libcrux_sha3.Generic_keccak.f_st
          l
        ==
        Hacspec_sha3.Sponge.xor_block_into_state
          (G.extract_lane (mk_usize 1) P.lc_portable state l)
          (inputs.[ mk_usize l ].[ {
              Core_models.Ops.Range.f_start = start;
              Core_models.Ops.Range.f_end   = start +! rate } <:
            Core_models.Ops.Range.t_Range usize ])
          rate)
  = let input0 = inputs.[ mk_usize 0 ] in
    lemma_load_block_eq_xor_block_into_state rate state input0 start;
    P.lemma_extract_lane_portable_identity state;
    let ks : Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
      { Libcrux_sha3.Generic_keccak.f_st = state } in
    let result =
      Libcrux_sha3.Traits.f_load_block
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        #(mk_usize 1)
        #FStar.Tactics.Typeclasses.solve
        rate ks inputs start in
    P.lemma_extract_lane_portable_identity
      result.Libcrux_sha3.Generic_keccak.f_st

let portable_sc_load_last
      (rate: usize)
      (delim: u8)
      (state: t_Array u64 (mk_usize 25))
      (inputs: t_Array (t_Slice u8) (mk_usize 1))
      (start: usize)
      (len: usize)
      (l: nat{l < 1})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len < v rate /\
        v start + v len <= Seq.length #u8 (inputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 1) inputs)
      (ensures (
        let padded: t_Array u8 (mk_usize 200) =
          Hacspec_sha3.Sponge.pad_last_block
            (inputs.[ mk_usize l ]) start len rate delim
        in
        G.extract_lane (mk_usize 1) P.lc_portable
          (Libcrux_sha3.Traits.f_load_last
             #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
             #(mk_usize 1)
             #FStar.Tactics.Typeclasses.solve
             rate delim
             ({ Libcrux_sha3.Generic_keccak.f_st = state }
                <: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
             inputs
             start
             len)
            .Libcrux_sha3.Generic_keccak.f_st
          l
        ==
        Hacspec_sha3.Sponge.xor_block_into_state
          (G.extract_lane (mk_usize 1) P.lc_portable state l)
          (padded.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                      Core_models.Ops.Range.f_end   = rate } <:
                    Core_models.Ops.Range.t_Range usize ] <: t_Slice u8)
          rate))
  = let input0 = inputs.[ mk_usize 0 ] in
    lemma_load_last_eq_xor_block_into_state_padded
      rate delim state input0 start len;
    P.lemma_extract_lane_portable_identity state;
    let ks : Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
      { Libcrux_sha3.Generic_keccak.f_st = state } in
    let result =
      Libcrux_sha3.Traits.f_load_last
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        #(mk_usize 1)
        #FStar.Tactics.Typeclasses.solve
        rate delim ks inputs start len in
    P.lemma_extract_lane_portable_identity
      result.Libcrux_sha3.Generic_keccak.f_st

let portable_sc_store_block
      (rate: usize)
      (state: t_Array u64 (mk_usize 25))
      (outputs: t_Array (t_Slice u8) (mk_usize 1))
      (start: usize)
      (len: usize)
      (l: nat{l < 1})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 (outputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 1) outputs)
      (ensures
        sq_lane_portable rate state outputs start len l
        ==
        Hacspec_sha3.Sponge.squeeze_state
          (Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]))
          (G.extract_lane (mk_usize 1) P.lc_portable state l)
          (outputs.[ mk_usize l ] <: t_Array u8 _)
          start
          len)
  = let out0 = outputs.[ mk_usize 0 ] in
    lemma_store_block_eq_squeeze_state rate state out0 start len;
    P.lemma_extract_lane_portable_identity state

(* ================================================================
   Assemble the [sponge_correctness] record for Portable.
   ================================================================ *)

let sc_portable : SC.sponge_correctness (mk_usize 1) #u64 P.lc_portable =
  {
    sq_lane        = sq_lane_portable;
    sc_load_block  = portable_sc_load_block;
    sc_load_last   = portable_sc_load_last;
    sc_store_block = portable_sc_store_block;
  }

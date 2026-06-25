module EquivImplSpec.Sponge.Portable.SqueezeAPI

(* ================================================================
   Portable squeeze driver equivalence.

   Proves: Libcrux_sha3.Generic_keccak.Portable.squeeze ≡
           Hacspec_sha3.Sponge.squeeze

   Structure: mirrors [lemma_absorb_portable_aux].  The spec-side
   [Hacspec_sha3.Sponge.squeeze] delegates its middle loop to a
   recursive helper [Hacspec_sha3.Sponge.squeeze_blocks]; the
   equivalence pairs the impl's [fold_range 1 output_blocks]
   iteration-for-iteration against that recursion via
   [Proof_Utils.FoldRange.lemma_fold_range_step].
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models

module SP = EquivImplSpec.Sponge.Portable
module KP = EquivImplSpec.Keccakf.Portable
module Steps = EquivImplSpec.Sponge.Portable.Steps
module HS = Hacspec_sha3.Sponge

(* Bring Portable typeclass instances into scope. *)
let _ =
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Portable in
  ()


(* ================================================================
   Step 1: f_squeeze (no keccakf) ≡ squeeze_state.
   ================================================================ *)
#push-options "--z3rlimit 100"
let lemma_squeeze_once_portable
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
        let out' = Libcrux_sha3.Traits.f_squeeze
                     #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
                     #u64
                     #FStar.Tactics.Typeclasses.solve
                     rate ks out start len in
        out' == HS.squeeze_state
                  (Core_models.Slice.impl__len #u8 out)
                  ks.Libcrux_sha3.Generic_keccak.f_st
                  (out <: t_Array u8 _) start len))
  = let outputs : t_Array (t_Slice u8) (mk_usize 1) =
      let list = [out] in
      FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 1);
      Rust_primitives.Hax.array_of_list 1 list in
    assert (outputs.[ mk_usize 0 ] == out);
    SP.portable_sc_store_block rate ks.Libcrux_sha3.Generic_keccak.f_st
      outputs start len 0;
    KP.lemma_extract_lane_portable_identity ks.Libcrux_sha3.Generic_keccak.f_st
#pop-options


(* NOTE: the former middle-loop lockstep induction [lemma_squeeze_portable_aux]
   and the re-exports of [lemma_squeeze_blocks_unfold]/[lemma_squeeze_blocks_tail]
   have been removed.

   The lockstep induction has been pushed into the Rust-side [squeeze]'s ensures
   clause (see [Libcrux_sha3.Generic_keccak.Portable.squeeze]): its postcondition
   now asserts equality with [EquivImplSpec.Sponge.Portable.Steps.portable_squeeze_composed]
   directly, with the fold-range → [squeeze_blocks] bridge discharged inline by
   [Hacspec_sha3.Sponge.Lemmas.lemma_squeeze_blocks_tail] and
   [Steps.lemma_squeeze_block_portable]. The remaining work — reconciling
   [portable_squeeze_composed] with [Hacspec_sha3.Sponge.squeeze] itself —
   is done by [EquivImplSpec.Sponge.Portable.API.lemma_squeeze_portable]. *)


(* ================================================================
   Per-block driver-lemma equivalences (Layer 2, milestone 8).

   These lemmas surface functional equivalence for the four
   per-block helpers in [Libcrux_sha3.Generic_keccak.Portable]
   ([impl__squeeze_first_block], [impl__squeeze_next_block],
   [impl__squeeze_first_three_blocks],
   [impl__squeeze_first_five_blocks]) whose Rust-side
   [#[hax_lib::ensures]] is bounds-only (length preservation).
   The functional spec is established here via separate lemmas
   chained on top of [lemma_squeeze_once_portable] and
   [KP.lemma_keccakf1600_portable] — same audit pattern as
   [keccakf1600] (Layer 1, milestone 1, surfaced via
   [EquivImplSpec.Keccakf.Portable.lemma_keccakf1600_portable]).
   ================================================================ *)

#push-options "--z3rlimit 100"
let lemma_squeeze_first_block_portable
      (v_RATE: usize)
      (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (out: t_Slice u8)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE /\
        v v_RATE <= Seq.length #u8 out)
      (ensures (
        let out' = Libcrux_sha3.Generic_keccak.Portable.impl__squeeze_first_block
                     v_RATE self out in
        out' == HS.squeeze_state
                  (Core_models.Slice.impl__len #u8 out)
                  self.Libcrux_sha3.Generic_keccak.f_st
                  (out <: t_Array u8 _) (mk_usize 0) v_RATE))
  = lemma_squeeze_once_portable v_RATE self out (mk_usize 0) v_RATE
#pop-options

#push-options "--z3rlimit 100"
let lemma_squeeze_next_block_portable
      (v_RATE: usize)
      (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (out: t_Slice u8)
      (start: usize)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE /\
        v start + v v_RATE <= Seq.length #u8 out)
      (ensures (
        let self', out' = Libcrux_sha3.Generic_keccak.Portable.impl__squeeze_next_block
                            v_RATE self out start in
        let new_st = Hacspec_sha3.Keccak_f.keccak_f
                       self.Libcrux_sha3.Generic_keccak.f_st in
        self'.Libcrux_sha3.Generic_keccak.f_st == new_st /\
        out' == HS.squeeze_state
                  (Core_models.Slice.impl__len #u8 out)
                  new_st
                  (out <: t_Array u8 _) start v_RATE))
  = let self_perm = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
                      (mk_usize 1) #u64 self in
    KP.lemma_keccakf1600_portable self;
    lemma_squeeze_once_portable v_RATE self_perm out start v_RATE
#pop-options

(* For the [first_three_blocks] / [first_five_blocks] composition, each
   subsequent [lemma_squeeze_once_portable] call needs a slice argument
   that was previously updated by [f_squeeze] (so its [out_len]
   coincides with the original).  [f_squeeze]'s postcondition includes
   the length-preservation [length out_future == length out_initial],
   which is enough to close the chain via Z3 — no explicit re-cast is
   needed inside the proof body. *)

#push-options "--z3rlimit 200 --split_queries always"
let lemma_squeeze_first_three_blocks_portable
      (v_RATE: usize)
      (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (out: t_Slice u8)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE /\
        3 * v v_RATE <= Seq.length #u8 out)
      (ensures (
        let self', out' =
          Libcrux_sha3.Generic_keccak.Portable.impl__squeeze_first_three_blocks
            v_RATE self out in
        let s0 = self.Libcrux_sha3.Generic_keccak.f_st in
        let s1 = Hacspec_sha3.Keccak_f.keccak_f s0 in
        let s2 = Hacspec_sha3.Keccak_f.keccak_f s1 in
        let out_len = Core_models.Slice.impl__len #u8 out in
        let out0 = HS.squeeze_state out_len s0
                     (out <: t_Array u8 _) (mk_usize 0) v_RATE in
        let out1 = HS.squeeze_state out_len s1
                     out0 v_RATE v_RATE in
        let out2 = HS.squeeze_state out_len s2
                     out1 (mk_usize 2 *! v_RATE) v_RATE in
        self'.Libcrux_sha3.Generic_keccak.f_st == s2 /\
        out' == out2))
  = let out_after_0 =
      Libcrux_sha3.Traits.f_squeeze
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        #u64 #FStar.Tactics.Typeclasses.solve
        v_RATE self out (mk_usize 0) v_RATE in
    lemma_squeeze_once_portable v_RATE self out (mk_usize 0) v_RATE;
    let self_1 = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
                   (mk_usize 1) #u64 self in
    KP.lemma_keccakf1600_portable self;
    let out_after_1 =
      Libcrux_sha3.Traits.f_squeeze
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        #u64 #FStar.Tactics.Typeclasses.solve
        v_RATE self_1 out_after_0 v_RATE v_RATE in
    lemma_squeeze_once_portable v_RATE self_1 out_after_0 v_RATE v_RATE;
    let self_2 = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
                   (mk_usize 1) #u64 self_1 in
    KP.lemma_keccakf1600_portable self_1;
    lemma_squeeze_once_portable v_RATE self_2 out_after_1 (mk_usize 2 *! v_RATE) v_RATE
#pop-options

#push-options "--z3rlimit 400 --split_queries always"
let lemma_squeeze_first_five_blocks_portable
      (v_RATE: usize)
      (self: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
      (out: t_Slice u8)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate v_RATE /\
        5 * v v_RATE <= Seq.length #u8 out)
      (ensures (
        let self', out' =
          Libcrux_sha3.Generic_keccak.Portable.impl__squeeze_first_five_blocks
            v_RATE self out in
        let s0 = self.Libcrux_sha3.Generic_keccak.f_st in
        let s1 = Hacspec_sha3.Keccak_f.keccak_f s0 in
        let s2 = Hacspec_sha3.Keccak_f.keccak_f s1 in
        let s3 = Hacspec_sha3.Keccak_f.keccak_f s2 in
        let s4 = Hacspec_sha3.Keccak_f.keccak_f s3 in
        let out_len = Core_models.Slice.impl__len #u8 out in
        let out0 = HS.squeeze_state out_len s0
                     (out <: t_Array u8 _) (mk_usize 0) v_RATE in
        let out1 = HS.squeeze_state out_len s1 out0 v_RATE v_RATE in
        let out2 = HS.squeeze_state out_len s2 out1
                     (mk_usize 2 *! v_RATE) v_RATE in
        let out3 = HS.squeeze_state out_len s3 out2
                     (mk_usize 3 *! v_RATE) v_RATE in
        let out4 = HS.squeeze_state out_len s4 out3
                     (mk_usize 4 *! v_RATE) v_RATE in
        self'.Libcrux_sha3.Generic_keccak.f_st == s4 /\
        out' == out4))
  = let out_after_0 =
      Libcrux_sha3.Traits.f_squeeze
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        #u64 #FStar.Tactics.Typeclasses.solve
        v_RATE self out (mk_usize 0) v_RATE in
    lemma_squeeze_once_portable v_RATE self out (mk_usize 0) v_RATE;
    let self_1 = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
                   (mk_usize 1) #u64 self in
    KP.lemma_keccakf1600_portable self;
    let out_after_1 =
      Libcrux_sha3.Traits.f_squeeze
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        #u64 #FStar.Tactics.Typeclasses.solve
        v_RATE self_1 out_after_0 v_RATE v_RATE in
    lemma_squeeze_once_portable v_RATE self_1 out_after_0 v_RATE v_RATE;
    let self_2 = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
                   (mk_usize 1) #u64 self_1 in
    KP.lemma_keccakf1600_portable self_1;
    let out_after_2 =
      Libcrux_sha3.Traits.f_squeeze
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        #u64 #FStar.Tactics.Typeclasses.solve
        v_RATE self_2 out_after_1 (mk_usize 2 *! v_RATE) v_RATE in
    lemma_squeeze_once_portable v_RATE self_2 out_after_1 (mk_usize 2 *! v_RATE) v_RATE;
    let self_3 = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
                   (mk_usize 1) #u64 self_2 in
    KP.lemma_keccakf1600_portable self_2;
    let out_after_3 =
      Libcrux_sha3.Traits.f_squeeze
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64)
        #u64 #FStar.Tactics.Typeclasses.solve
        v_RATE self_3 out_after_2 (mk_usize 3 *! v_RATE) v_RATE in
    lemma_squeeze_once_portable v_RATE self_3 out_after_2 (mk_usize 3 *! v_RATE) v_RATE;
    let self_4 = Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
                   (mk_usize 1) #u64 self_3 in
    KP.lemma_keccakf1600_portable self_3;
    lemma_squeeze_once_portable v_RATE self_4 out_after_3 (mk_usize 4 *! v_RATE) v_RATE
#pop-options

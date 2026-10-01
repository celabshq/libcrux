module EquivImplSpec.Sponge.Avx2.OneStepByte

(* ================================================================
   Per-index byteform equality for ONE avx2 (N=4) squeeze step, in its own
   module so its (monolithic, LP-crash-avoiding) encoding does NOT bloat the
   context of the [forall_intro] dispatch in [EquivImplSpec.Sponge.Avx2.OneStep]
   (which then imports only this lemma's ensures).  N=4 analog of arm64's
   [OneStep.lemma_one_step_byte_arm64] (arm64 keeps both in one module because
   its byte lemma is split-400, light enough not to pollute the dispatch; the
   avx2 byte lemma must be MONOLITHIC to dodge the Z3 4.13.3 LP-solver crash
   (Error 276), and monolithic is heavy — hence the separate module).
   ================================================================ *)

#set-options "--fuel 1 --ifuel 1 --z3rlimit 150"

open FStar.Mul
open Core_models

module G   = EquivImplSpec.Keccakf.Generic
module KA  = EquivImplSpec.Keccakf.Avx2
module SA  = EquivImplSpec.Sponge.Avx2
module HSL = Hacspec_sha3.Sponge.Lemmas
module I   = Libcrux_intrinsics.Avx2_sha3_views

(* Delegates: [lemma_sq_lane_byte_eq_avx2] (outX'.[k] == squeeze_state state_i .[k]),
   [lemma_squeeze_state_index] (squeeze_state value), [lemma_squeeze_index]
   (squeeze value); b == i + the state equality make the two [to_le_bytes] byte
   values coincide.  MONOLITHIC (no --split_queries): at N=4 the split path trips
   the Z3 4.13.3 LP-solver crash (Error 276) on this byteform+arithmetic VC. *)
#restart-solver
#push-options "--z3rlimit 800 --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid -Libcrux_sha3.Simd.Avx2.Store.store_block_full_avx2 -Libcrux_sha3.Simd.Avx2.Store.store_block_tail_avx2 -Libcrux_sha3.Simd.Avx2.Store.store_chunk8x4 -Libcrux_sha3.Simd.Avx2.Store.store_u64x4x4 -Libcrux_sha3.Simd.Avx2.Store.store_tail_ragged_avx2'"
let lemma_one_step_byte_avx2
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (st_post: t_Array I.t_Vec256 (mk_usize 25))
      (lane_st_init: t_Array u64 (mk_usize 25))
      (outputs_pre: t_Array (t_Slice u8) (mk_usize 4))
      (i: usize)
      (l: nat{l < 4})
      (k: nat)
  : Lemma
      (requires (
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        v i >= 1 /\ v i * v rate + v rate <= v outlen /\
        v outlen < v Core_models.Num.impl_usize__MAX - 200 /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) outputs_pre /\
        v i * v rate <= k /\ k < (v i + 1) * v rate /\
        G.extract_lane (mk_usize 4) KA.lc_avx2 st_post l ==
          Hacspec_sha3.Sponge.iterate_keccak_f i lane_st_init))
      (ensures (
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        Seq.index
          (SA.sq_lane_avx2 rate st_post outputs_pre (i *! rate) rate l <: Seq.seq u8) k ==
        Seq.index
          (Hacspec_sha3.Sponge.squeeze outlen lane_st_init rate <: Seq.seq u8) k))
  = let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
    let kk : usize = mk_usize k in
    assert (v kk == k);
    let state_i = G.extract_lane (mk_usize 4) KA.lc_avx2 st_post l in
    FStar.Math.Lemmas.distributivity_add_left (v i) 1 (v rate);
    assert ((v i + 1) * v rate == v i * v rate + v rate);
    assert (k - v i * v rate < v rate);
    FStar.Math.Lemmas.small_div (k - v i * v rate) (v rate);
    FStar.Math.Lemmas.lemma_div_plus (k - v i * v rate) (v i) (v rate);
    let b : usize = kk /! rate in
    assert (v b == v i);
    assert (b == i);
    let j : usize = kk -! (b *! rate) in
    assert (v j == k - v i * v rate);
    assert (v j / 8 < 25);
    SA.lemma_sq_lane_byte_eq_avx2 rate st_post outputs_pre (i *! rate) rate l k;
    HSL.lemma_squeeze_state_index outlen state_i
      (outputs_pre.[ mk_usize l ] <: t_Array u8 outlen) (i *! rate) rate kk;
    HSL.lemma_squeeze_index outlen lane_st_init rate kk
#pop-options

(* Per-index FRAME equality for the OUT-of-range prefix [k < i*rate] of ONE
   squeeze step (N=4), clean context.  For k before the written block, the store
   preserves the pre-array and [squeeze_state]'s else-branch returns it, so
   [outX'.[k] == outputs_pre.[l].[k]]; the caller supplies the loop-invariant
   [outputs_pre.[l].[k] == squeeze.[k]], giving [outX'.[k] == squeeze.[k]].  No
   state-eq / distributivity needed (out-of-range is state-independent).  Kept out
   of the [Avx2.OneStep] dispatch's context so its forall_intro WP stays light. *)
#restart-solver
#push-options "--z3rlimit 400 --split_queries always --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid -Libcrux_sha3.Simd.Avx2.Store.store_block_full_avx2 -Libcrux_sha3.Simd.Avx2.Store.store_block_tail_avx2 -Libcrux_sha3.Simd.Avx2.Store.store_chunk8x4 -Libcrux_sha3.Simd.Avx2.Store.store_u64x4x4 -Libcrux_sha3.Simd.Avx2.Store.store_tail_ragged_avx2'"
let lemma_one_step_frame_avx2
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (st_post: t_Array I.t_Vec256 (mk_usize 25))
      (lane_st_init: t_Array u64 (mk_usize 25))
      (outputs_pre: t_Array (t_Slice u8) (mk_usize 4))
      (i: usize)
      (l: nat{l < 4})
      (k: nat)
  : Lemma
      (requires (
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        v i >= 1 /\ v i * v rate + v rate <= v outlen /\
        v outlen < v Core_models.Num.impl_usize__MAX - 200 /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) outputs_pre /\
        k < v i * v rate /\
        Seq.index (outputs_pre.[ mk_usize l ] <: Seq.seq u8) k ==
          Seq.index (Hacspec_sha3.Sponge.squeeze outlen lane_st_init rate <: Seq.seq u8) k))
      (ensures (
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        Seq.index
          (SA.sq_lane_avx2 rate st_post outputs_pre (i *! rate) rate l <: Seq.seq u8) k ==
        Seq.index
          (Hacspec_sha3.Sponge.squeeze outlen lane_st_init rate <: Seq.seq u8) k))
  = let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
    let kk : usize = mk_usize k in
    assert (v kk == k);
    assert (k < v outlen);
    let state_i = G.extract_lane (mk_usize 4) KA.lc_avx2 st_post l in
    (* outX'.[k] == squeeze_state state_i .[k] (per-index, any k). *)
    SA.lemma_sq_lane_byte_eq_avx2 rate st_post outputs_pre (i *! rate) rate l k;
    (* out-of-range (k < i*rate = out_offset): squeeze_state .[kk] == outputs_pre.[l].[kk]. *)
    HSL.lemma_squeeze_state_index outlen state_i
      (outputs_pre.[ mk_usize l ] <: t_Array u8 outlen) (i *! rate) rate kk
#pop-options

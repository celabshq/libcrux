module EquivImplSpec.Sponge.Arm64.OneStep

(* ================================================================
   [lemma_squeeze_one_step_arm64] (N=2 byteform squeeze-step) lives in
   this lean module, separate from [EquivImplSpec.Sponge.Arm64.Steps].

   Rationale: the lemma's [forall_intro] dispatch sub-query passes in
   isolation (admit_except, split-800) but SATURATES inside [Arm64.Steps] —
   its SMT context accumulates every prior sibling step-lemma's inline proof
   encoding.  Here it imports [Arm64.Steps] as an interface (only the
   exported [lemma_squeeze_block_arm64] ensures, no sibling bodies), so the
   context is isolation-sized and the dispatch closes.  See fstar-for-libcrux
   §7 "Abstract companion .fsti to escape a heavy-host context cascade".
   ================================================================ *)

#set-options "--fuel 1 --ifuel 1 --z3rlimit 150"

open FStar.Mul
open Core_models

module G     = EquivImplSpec.Keccakf.Generic
module KA    = EquivImplSpec.Keccakf.Arm64
module SA    = EquivImplSpec.Sponge.Arm64
module Steps = EquivImplSpec.Sponge.Arm64.Steps
module I     = Libcrux_intrinsics.Arm64_sha3_views
module HSL   = Hacspec_sha3.Sponge.Lemmas

(* Bring Arm64 typeclass instances into scope so t_KeccakItem /
   t_Absorb / t_Squeeze2 at N=2 resolve. *)
let _ =
  let open Libcrux_intrinsics.Arm64_sha3_views in
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Arm64 in
  ()

(* Per-index byteform equality for ONE squeeze step, in a clean context so the
   final [to_le_bytes]-congruence stays cheap.  Delegates: [lemma_sq_lane_byte_eq_arm64]
   (outX'.[k] == squeeze_state state_i .[k]), [lemma_squeeze_state_index]
   (squeeze_state value), [lemma_squeeze_index] (squeeze value); b == i plus the
   state equality make the two [to_le_bytes] byte values coincide.  The store body
   helpers are excluded via [--using_facts_from] (Z3 uses the cheap opaque
   sq_lane_arm64/stored posts).  The last saturation here was NONLINEAR
   distributivity [(i+1)*rate], discharged by [distributivity_add_left]. *)
#restart-solver
#push-options "--z3rlimit 400 --split_queries always --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid -Libcrux_sha3.Simd.Arm64.Store.store_block_full -Libcrux_sha3.Simd.Arm64.Store.store_block_tail -Libcrux_sha3.Simd.Arm64.Store.store_tail_high -Libcrux_sha3.Simd.Arm64.Store.store_tail_low -Libcrux_sha3.Simd.Arm64.Store.store_u64x2x2'"
let lemma_one_step_byte_arm64
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (st_post: t_Array I.t_e_uint64x2_t (mk_usize 25))
      (lane_st_init: t_Array u64 (mk_usize 25))
      (outputs_pre: t_Array (t_Slice u8) (mk_usize 2))
      (i: usize)
      (l: nat{l < 2})
      (k: nat)
  : Lemma
      (requires (
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        v i >= 1 /\ v i * v rate + v rate <= v outlen /\
        v outlen < v Core_models.Num.impl_usize__MAX - 200 /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) outputs_pre /\
        v i * v rate <= k /\ k < (v i + 1) * v rate /\
        G.extract_lane (mk_usize 2) KA.lc_arm64 st_post l ==
          Hacspec_sha3.Sponge.iterate_keccak_f i lane_st_init))
      (ensures (
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        Seq.index
          (SA.sq_lane_arm64 rate st_post outputs_pre (i *! rate) rate l <: Seq.seq u8) k ==
        Seq.index
          (Hacspec_sha3.Sponge.squeeze outlen lane_st_init rate <: Seq.seq u8) k))
  = let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
    let kk : usize = mk_usize k in
    assert (v kk == k);
    let state_i = G.extract_lane (mk_usize 2) KA.lc_arm64 st_post l in
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
    SA.lemma_sq_lane_byte_eq_arm64 rate st_post outputs_pre (i *! rate) rate l k;
    HSL.lemma_squeeze_state_index outlen state_i
      (outputs_pre.[ mk_usize l ] <: t_Array u8 outlen) (i *! rate) rate kk;
    HSL.lemma_squeeze_index outlen lane_st_init rate kk
#pop-options

(* Per-index FRAME equality for the OUT-of-range prefix [k < i*rate] (N=2), clean
   context — mirrors the avx2 twin.  outX'.[k] == outputs_pre.[l].[k] (the store
   preserves the pre-array; [squeeze_state]'s else-branch returns it), then the
   caller-supplied loop-invariant gives == squeeze.[k].  No state-eq / distributivity
   (out-of-range is state-independent). *)
#restart-solver
#push-options "--z3rlimit 400 --split_queries always --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid -Libcrux_sha3.Simd.Arm64.Store.store_block_full -Libcrux_sha3.Simd.Arm64.Store.store_block_tail -Libcrux_sha3.Simd.Arm64.Store.store_tail_high -Libcrux_sha3.Simd.Arm64.Store.store_tail_low -Libcrux_sha3.Simd.Arm64.Store.store_u64x2x2'"
let lemma_one_step_frame_arm64
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (st_post: t_Array I.t_e_uint64x2_t (mk_usize 25))
      (lane_st_init: t_Array u64 (mk_usize 25))
      (outputs_pre: t_Array (t_Slice u8) (mk_usize 2))
      (i: usize)
      (l: nat{l < 2})
      (k: nat)
  : Lemma
      (requires (
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        v i >= 1 /\ v i * v rate + v rate <= v outlen /\
        v outlen < v Core_models.Num.impl_usize__MAX - 200 /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) outputs_pre /\
        k < v i * v rate /\
        Seq.index (outputs_pre.[ mk_usize l ] <: Seq.seq u8) k ==
          Seq.index (Hacspec_sha3.Sponge.squeeze outlen lane_st_init rate <: Seq.seq u8) k))
      (ensures (
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        Seq.index
          (SA.sq_lane_arm64 rate st_post outputs_pre (i *! rate) rate l <: Seq.seq u8) k ==
        Seq.index
          (Hacspec_sha3.Sponge.squeeze outlen lane_st_init rate <: Seq.seq u8) k))
  = let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
    let kk : usize = mk_usize k in
    assert (v kk == k);
    assert (k < v outlen);
    let state_i = G.extract_lane (mk_usize 2) KA.lc_arm64 st_post l in
    SA.lemma_sq_lane_byte_eq_arm64 rate st_post outputs_pre (i *! rate) rate l k;
    HSL.lemma_squeeze_state_index outlen state_i
      (outputs_pre.[ mk_usize l ] <: t_Array u8 outlen) (i *! rate) rate kk
#pop-options

(* Byteform step lemma (N=2): thin per-index dispatch — in-range to
   [lemma_one_step_byte_arm64], out-of-range to [lemma_one_step_frame_arm64], both
   clean-context.  MONOLITHIC (no --split_queries): the thin aux closes in one
   query, and avoiding the split keeps the fstar process's z3 ask_count low, which
   dodges the Z3 4.13.3 IPC crash ("</labels> not found") that the ~130 split
   sub-queries triggered when this was split-800. *)
#restart-solver
#push-options "--z3rlimit 800 --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid -Libcrux_sha3.Simd.Arm64.Store.store_block_full -Libcrux_sha3.Simd.Arm64.Store.store_block_tail -Libcrux_sha3.Simd.Arm64.Store.store_tail_high -Libcrux_sha3.Simd.Arm64.Store.store_tail_low -Libcrux_sha3.Simd.Arm64.Store.store_u64x2x2'"
let lemma_squeeze_one_step_arm64
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (s_init_st: t_Array I.t_e_uint64x2_t (mk_usize 25))
      (ks_pre: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
      (outputs_pre: t_Array (t_Slice u8) (mk_usize 2))
      (i: usize)
      (l: nat{l < 2})
  : Lemma
      (requires (
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        v i >= 1 /\
        v i * v rate + v rate <= v outlen /\
        v outlen < v Core_models.Num.impl_usize__MAX - 200 /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) outputs_pre /\
        (let lane_st_init =
            G.extract_lane (mk_usize 2) KA.lc_arm64 s_init_st l in
         G.extract_lane (mk_usize 2) KA.lc_arm64
           ks_pre.Libcrux_sha3.Generic_keccak.f_st l
         == Hacspec_sha3.Sponge.iterate_keccak_f (i -! mk_usize 1) lane_st_init /\
         (forall (k: nat). k < v i * v rate /\ k < v outlen ==>
            Seq.index (outputs_pre.[ mk_usize l ] <: Seq.seq u8) k ==
            Seq.index
              (Hacspec_sha3.Sponge.squeeze outlen lane_st_init rate <: Seq.seq u8) k))))
      (ensures (
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        let ks_post =
            Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
              (mk_usize 2) #I.t_e_uint64x2_t ks_pre in
        let outX' =
            SA.sq_lane_arm64 rate ks_post.Libcrux_sha3.Generic_keccak.f_st
              outputs_pre (i *! rate) rate l in
        let lane_st_init =
            G.extract_lane (mk_usize 2) KA.lc_arm64 s_init_st l in
        G.extract_lane (mk_usize 2) KA.lc_arm64
          ks_post.Libcrux_sha3.Generic_keccak.f_st l
        == Hacspec_sha3.Sponge.iterate_keccak_f i lane_st_init /\
        (forall (k: nat). k < (v i + 1) * v rate /\ k < v outlen ==>
            Seq.index (outX' <: Seq.seq u8) k ==
            Seq.index
              (Hacspec_sha3.Sponge.squeeze outlen lane_st_init rate <: Seq.seq u8) k)))
  = let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
    let lane_st_init =
        G.extract_lane (mk_usize 2) KA.lc_arm64 s_init_st l in
    (* State step: keccak_f (iterate_keccak_f (v i - 1) lane_st_init)
       == iterate_keccak_f (v i) lane_st_init. *)
    Steps.lemma_squeeze_block_arm64 rate ks_pre outputs_pre (i *! rate) l;
    let ks_post =
        Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
          (mk_usize 2) #I.t_e_uint64x2_t ks_pre in
    let outX' =
        SA.sq_lane_arm64 rate ks_post.Libcrux_sha3.Generic_keccak.f_st
          outputs_pre (i *! rate) rate l in
    FStar.Math.Lemmas.distributivity_add_left (v i) 1 (v rate);
    let state_i =
        G.extract_lane (mk_usize 2) KA.lc_arm64
          ks_post.Libcrux_sha3.Generic_keccak.f_st l in
    assert (state_i == Hacspec_sha3.Sponge.iterate_keccak_f i lane_st_init);
    let aux (k: nat{k < v outlen})
      : Lemma
        (k < (v i + 1) * v rate ==>
          Seq.index (outX' <: Seq.seq u8) k ==
          Seq.index
            (Hacspec_sha3.Sponge.squeeze outlen lane_st_init rate <: Seq.seq u8) k) =
      if k < (v i + 1) * v rate then begin
        if k < v i * v rate then begin
          (* out-of-range prefix: instantiate the loop-invariant forall at k, then
             delegate to the clean-context frame lemma. *)
          assert (Seq.index (outputs_pre.[ mk_usize l ] <: Seq.seq u8) k ==
                  Seq.index (Hacspec_sha3.Sponge.squeeze outlen lane_st_init rate <: Seq.seq u8) k);
          lemma_one_step_frame_arm64 rate
            ks_post.Libcrux_sha3.Generic_keccak.f_st lane_st_init outputs_pre i l k
        end
        else lemma_one_step_byte_arm64 rate
               ks_post.Libcrux_sha3.Generic_keccak.f_st lane_st_init outputs_pre i l k
      end
    in
    FStar.Classical.forall_intro aux
#pop-options

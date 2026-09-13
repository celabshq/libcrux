module EquivImplSpec.Sponge.Avx2.OneStep

(* ================================================================
   [lemma_squeeze_one_step_avx2] (N=4 byteform squeeze-step) lives in
   this lean module, separate from [EquivImplSpec.Sponge.Avx2.SqueezeDriver].

   Rationale: the lemma's [forall_intro] dispatch sub-query passes in
   isolation (admit_except, split-800) but SATURATES inside SqueezeDriver —
   its SMT context accumulates the sibling step/driver lemmas' inline proof
   encodings.  Here it imports [Avx2.Steps] as an interface (only
   [lemma_squeeze_block_avx2]'s ensures) so the context is isolation-sized
   and the dispatch closes.  Mirrors [EquivImplSpec.Sponge.Arm64.OneStep]
   at N=2.  fstar-for-libcrux §7 "Abstract companion .fsti to escape a
   heavy-host context cascade".
   ================================================================ *)

#set-options "--fuel 1 --ifuel 1 --z3rlimit 150"

open FStar.Mul
open Core_models

module G           = EquivImplSpec.Keccakf.Generic
module KA          = EquivImplSpec.Keccakf.Avx2
module SA          = EquivImplSpec.Sponge.Avx2
module Steps       = EquivImplSpec.Sponge.Avx2.Steps
module OneStepByte = EquivImplSpec.Sponge.Avx2.OneStepByte
module HS          = Hacspec_sha3.Sponge
module I           = Libcrux_intrinsics.Avx2_sha3_views

(* Bring AVX2 typeclass instances into scope so t_Squeeze4 at N=4 resolves. *)
let _ =
  let open Libcrux_intrinsics.Avx2_sha3_views in
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Avx2 in
  ()

(* [lemma_one_step_byte_avx2] lives in [EquivImplSpec.Sponge.Avx2.OneStepByte] so its
   heavy monolithic encoding does not bloat this dispatch's context. *)

(* Byteform step lemma (N=4): thin per-index dispatch to the clean-context
   [OneStepByte.lemma_one_step_byte_avx2] + the state step from [Steps.lemma_squeeze_block_avx2].
   ⚠️ CLIFF (Z3 4.13.3): this N=4 forall_intro dispatch has NO working config —
   split-800 saturates one sub-query (~132), monolithic LP-crashes (Error 276).
   Passes in the arm64 N=2 twin (split-800, 156.5).  Needs a Z3 version bump or a
   per-lane reformulation of the dispatch forall.  Left at split-800 (least-bad:
   only one sub-query saturates). *)
#restart-solver
#push-options "--z3rlimit 800 --split_queries always --using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid -Libcrux_sha3.Simd.Avx2.Store.store_block_full_avx2 -Libcrux_sha3.Simd.Avx2.Store.store_block_tail_avx2 -Libcrux_sha3.Simd.Avx2.Store.store_chunk8x4 -Libcrux_sha3.Simd.Avx2.Store.store_u64x4x4 -Libcrux_sha3.Simd.Avx2.Store.store_tail_ragged_avx2'"
let lemma_squeeze_one_step_avx2
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (s_init_st: t_Array I.t_Vec256 (mk_usize 25))
      (ks_pre: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
      (outputs_pre: t_Array (t_Slice u8) (mk_usize 4))
      (i: usize)
      (l: nat{l < 4})
  : Lemma
      (requires (
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        v i >= 1 /\
        v i * v rate + v rate <= v outlen /\
        v outlen < v Core_models.Num.impl_usize__MAX - 200 /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) outputs_pre /\
        (let lane_st_init = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st l in
         G.extract_lane (mk_usize 4) KA.lc_avx2
           ks_pre.Libcrux_sha3.Generic_keccak.f_st l
         == HS.iterate_keccak_f (i -! mk_usize 1) lane_st_init /\
         (forall (k: nat). k < v i * v rate /\ k < v outlen ==>
            Seq.index (outputs_pre.[ mk_usize l ] <: Seq.seq u8) k ==
            Seq.index
              (HS.squeeze outlen lane_st_init rate <: Seq.seq u8) k))))
      (ensures (
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        let ks_post =
          Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
            (mk_usize 4) #I.t_Vec256 ks_pre in
        let outX' =
          SA.sq_lane_avx2 rate ks_post.Libcrux_sha3.Generic_keccak.f_st
            outputs_pre (i *! rate) rate l in
        let lane_st_init = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st l in
        G.extract_lane (mk_usize 4) KA.lc_avx2
          ks_post.Libcrux_sha3.Generic_keccak.f_st l
        == HS.iterate_keccak_f i lane_st_init /\
        (forall (k: nat). k < (v i + 1) * v rate /\ k < v outlen ==>
            Seq.index (outX' <: Seq.seq u8) k ==
            Seq.index
              (HS.squeeze outlen lane_st_init rate <: Seq.seq u8) k)))
  = let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
    let lane_st_init = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st l in
    assert (v i * v rate + v rate <= v outlen);
    assert (Seq.length #u8 (outputs_pre.[ mk_usize 0 ]) == v outlen);
    let start : usize = i *! rate in
    assert (v start == v i * v rate);
    assert (v start + v rate <= Seq.length #u8 (outputs_pre.[ mk_usize 0 ]));
    Steps.lemma_squeeze_block_avx2 rate ks_pre outputs_pre start l;
    let ks_post =
      Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
        (mk_usize 4) #I.t_Vec256 ks_pre in
    let outX' =
      SA.sq_lane_avx2 rate ks_post.Libcrux_sha3.Generic_keccak.f_st
        outputs_pre start rate l in
    FStar.Math.Lemmas.distributivity_add_left (v i) 1 (v rate);
    let state_i =
      G.extract_lane (mk_usize 4) KA.lc_avx2
        ks_post.Libcrux_sha3.Generic_keccak.f_st l in
    assert (state_i == Hacspec_sha3.Sponge.iterate_keccak_f i lane_st_init);
    let aux (k: nat{k < v outlen})
      : Lemma
        (k < (v i + 1) * v rate ==>
          Seq.index (outX' <: Seq.seq u8) k ==
          Seq.index
            (HS.squeeze outlen lane_st_init rate <: Seq.seq u8) k) =
      if k < (v i + 1) * v rate then begin
        if k < v i * v rate then begin
          (* out-of-range prefix: instantiate the loop-invariant forall at k, then
             delegate to the clean-context frame lemma (keeps the dispatch WP thin). *)
          assert (Seq.index (outputs_pre.[ mk_usize l ] <: Seq.seq u8) k ==
                  Seq.index (HS.squeeze outlen lane_st_init rate <: Seq.seq u8) k);
          OneStepByte.lemma_one_step_frame_avx2 rate
            ks_post.Libcrux_sha3.Generic_keccak.f_st lane_st_init outputs_pre i l k
        end
        else OneStepByte.lemma_one_step_byte_avx2 rate
               ks_post.Libcrux_sha3.Generic_keccak.f_st lane_st_init outputs_pre i l k
      end
    in
    FStar.Classical.forall_intro aux
#pop-options

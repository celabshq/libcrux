module EquivImplSpec.Sponge.ByteForm

(* ================================================================
   ByteForm squeeze spec — experimental, sandbox.

   Replaces the recursive [Hacspec_sha3.Sponge.squeeze] with a direct
   per-byte denotational form.  Block [b]'s bytes use
   [iterate_keccak_f b state_init].  Per-byte equality with the impl
   side (after writing block [b] from a state that has had [b]
   keccak_f's applied) is one direct equation, no per-byte case-split
   between "in spec write range" / "preserved tail".

   GOAL: experiment whether this collapses the per-byte forall_intro
   compose with the loop invariant's per-block forall.  If yes, we
   replace [Hacspec_sha3.Sponge.squeeze] wholesale and remove the old
   recursive helpers ([squeeze_blocks], [squeeze_last],
   [lemma_squeeze_blocks_*], etc.).

   The bridge to the existing [Hacspec_sha3.Sponge.squeeze] is
   currently ADMITTED ([assume val lemma_squeeze_eq_byteform])
   pending the experiment's outcome.
   ================================================================ *)

#set-options "--fuel 1 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models

module HK = Hacspec_sha3.Keccak_f


(* n iterations of [keccak_f].  Right-add definition: the unfold
   [iterate_keccak_f (n+1) state == keccak_f (iterate_keccak_f n state)]
   is by definitional reduction at fuel 1, no separate lemma needed. *)
let rec iterate_keccak_f (n: nat) (state: t_Array u64 (mk_usize 25))
  : Tot (t_Array u64 (mk_usize 25)) (decreases n)
  = if n = 0 then state
    else HK.keccak_f (iterate_keccak_f (n - 1) state)


(* ByteForm squeeze: every output byte is determined by its block
   index and within-block offset.

   For byte [k] in [0, outlen):
     b := k / rate
     j := k - b * rate
     state_b := iterate_keccak_f b state_init
     byte_k := (to_le_bytes state_b.[j/8]).[j%8]

   Block [b] for [b ∈ [0, outlen/rate)] uses the state after [b]
   keccak_f's.  The trailing partial block (when outlen % rate > 0)
   uses the state after [outlen / rate] keccak_f's.  Since
   [valid_rate rate] forces [rate < 200], every [j < rate] satisfies
   [j/8 < 25], so the lane index is always in range. *)
let squeeze_byteform
      (outlen: usize{
        v outlen < v Core_models.Num.impl_usize__MAX - 200})
      (state_init: t_Array u64 (mk_usize 25))
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
  : Tot (t_Array u8 outlen)
  = Hacspec_sha3.createi #u8 outlen #(usize -> u8)
      (fun k ->
        let b : usize = k /! rate in
        let j : usize = k -! (b *! rate) in
        let state_b = iterate_keccak_f (v b) state_init in
        (Core_models.Num.impl_u64__to_le_bytes
           (state_b.[j /! mk_usize 8] <: u64) <: t_Array u8 (mk_usize 8))
           .[j %! mk_usize 8])


(* ============================================================
   Bridge to the existing [Hacspec_sha3.Sponge.squeeze].

   ADMITTED — to be discharged once the experiment confirms the
   byteform shape is the right call (then we replace
   [Hacspec_sha3.Sponge.squeeze] wholesale rather than keep both).

   Proof sketch: induct on [output_blocks = outlen / rate].
   Use [Hacspec_sha3.Sponge.Lemmas.lemma_squeeze_blocks_unfold] to
   peel one iteration at a time; each iteration matches one [b] in
   the byteform.  Trailing partial block aligns with [b = output_blocks].
   ============================================================ *)
assume val lemma_squeeze_eq_byteform
      (outlen: usize{v outlen < v Core_models.Num.impl_usize__MAX - 200})
      (state_init: t_Array u64 (mk_usize 25))
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
  : Lemma
      (ensures
        Hacspec_sha3.Sponge.squeeze outlen state_init rate
        ==
        squeeze_byteform outlen state_init rate)


(* ============================================================
   EXPERIMENT: per-iteration step lemma against the byteform spec.

   Mirrors [EquivImplSpec.Sponge.Arm64.Steps.lemma_squeeze_one_step_arm64]
   but states the loop invariant against [squeeze_byteform] instead of
   the recursive [Hacspec_sha3.Sponge.squeeze_blocks] + [output_initial]
   threading.

   Compare:
     - Old-spec version (Arm64.Steps): 84 s wall, 239 sub-queries,
       ~150 lines, 4-clause invariant including spec_out_pre /
       outputs_initial threading.
     - This byteform version: target <30 s wall, fewer sub-queries,
       <80 lines, 2-clause invariant.

   Key simplifications under byteform:
     - No spec_out_pre / spec_out_post pair to thread.
     - No tail-preservation forall (bytes [(i+1)*rate, outlen) are
       unconstrained until written; consumers re-establish via
       byteform ensures, no separate tail clause needed).
     - State condition: [extract_lane ks.f_st l == iterate_keccak_f
       (v i - 1) state_init_l].  Right-add unfold gives us
       [iterate_keccak_f (v i) state_init_l] after one keccakf1600
       by definitional reduction (fuel 1).
     - Per-byte aux: just one assert per byte (lane-bound check),
       no case-split with 4 branches.
   ============================================================ *)

module HK_Generic = EquivImplSpec.Keccakf.Generic
module KA_Arm64 = EquivImplSpec.Keccakf.Arm64
module SA_Arm64 = EquivImplSpec.Sponge.Arm64
module I_Arm64 = Libcrux_intrinsics.Arm64_extract

(* Bring Arm64 typeclass instances into scope. *)
let _ =
  let open Libcrux_intrinsics.Arm64_extract in
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Arm64 in
  ()

#push-options "--z3rlimit 400 --split_queries always"
let lemma_squeeze_one_step_arm64_byteform
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (s_init_st: t_Array I_Arm64.t_e_uint64x2_t (mk_usize 25))
      (ks_pre: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I_Arm64.t_e_uint64x2_t)
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
            HK_Generic.extract_lane (mk_usize 2) KA_Arm64.lc_arm64 s_init_st l in
         HK_Generic.extract_lane (mk_usize 2) KA_Arm64.lc_arm64
           ks_pre.Libcrux_sha3.Generic_keccak.f_st l
         == iterate_keccak_f (v i - 1) lane_st_init /\
         (forall (k: nat). k < v i * v rate /\ k < v outlen ==>
            Seq.index (outputs_pre.[ mk_usize l ] <: Seq.seq u8) k ==
            Seq.index
              (squeeze_byteform outlen lane_st_init rate <: Seq.seq u8) k))))
      (ensures (
        let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
        let ks_post =
            Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
              (mk_usize 2) #I_Arm64.t_e_uint64x2_t ks_pre in
        let outX' =
            SA_Arm64.sq_lane_arm64 rate ks_post.Libcrux_sha3.Generic_keccak.f_st
              outputs_pre (i *! rate) rate l in
        let lane_st_init =
            HK_Generic.extract_lane (mk_usize 2) KA_Arm64.lc_arm64 s_init_st l in
        HK_Generic.extract_lane (mk_usize 2) KA_Arm64.lc_arm64
          ks_post.Libcrux_sha3.Generic_keccak.f_st l
        == iterate_keccak_f (v i) lane_st_init /\
        (forall (k: nat). k < (v i + 1) * v rate /\ k < v outlen ==>
            Seq.index (outX' <: Seq.seq u8) k ==
            Seq.index
              (squeeze_byteform outlen lane_st_init rate <: Seq.seq u8) k)))
  = let outlen = Core_models.Slice.impl__len #u8 (outputs_pre.[ mk_usize l ]) in
    let lane_st_init =
        HK_Generic.extract_lane (mk_usize 2) KA_Arm64.lc_arm64 s_init_st l in
    (* State step: keccak_f (iterate_keccak_f (v i - 1) lane_st_init)
       == iterate_keccak_f (v i) lane_st_init by the right-add
       definitional unfold of iterate_keccak_f at fuel 1. *)
    EquivImplSpec.Sponge.Arm64.Steps.lemma_squeeze_block_arm64
      rate ks_pre outputs_pre (i *! rate) l;
    let ks_post =
        Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
          (mk_usize 2) #I_Arm64.t_e_uint64x2_t ks_pre in
    let outX' =
        SA_Arm64.sq_lane_arm64 rate ks_post.Libcrux_sha3.Generic_keccak.f_st
          outputs_pre (i *! rate) rate l in
    FStar.Math.Lemmas.distributivity_add_left (v i) 1 (v rate);
    let aux (k: nat{k < v outlen})
      : Lemma
        (k < (v i + 1) * v rate ==>
          Seq.index (outX' <: Seq.seq u8) k ==
          Seq.index
            (squeeze_byteform outlen lane_st_init rate <: Seq.seq u8) k) =
      if k < (v i + 1) * v rate then begin
        let kk : usize = mk_usize k in
        assert (v kk == k);
        if k < v i * v rate then ()
        else begin
          assert (v i * v rate <= k);
          assert ((v i + 1) * v rate == v i * v rate + v rate);
          assert (k - v i * v rate < v rate);
          assert ((k - v i * v rate) / 8 < 25);
          (* Trigger byteform's createi_lemma SMTPat at index kk:
             byteform[k] uses block b = k / rate = v i; offset
             j = k - v i*rate; state iterate_keccak_f i lane_st_init.

             For Z3 to see [k / v rate == v i] from
             [v i * v rate <= k < (v i + 1) * v rate], use
             [lemma_div_plus] + [small_div]:
               [k = (k - v i * v rate) + v i * v rate]
               [k / v rate = (k - v i * v rate) / v rate + v i = 0 + v i] *)
          FStar.Math.Lemmas.small_div (k - v i * v rate) (v rate);
          FStar.Math.Lemmas.lemma_div_plus
            (k - v i * v rate) (v i) (v rate);
          let b : usize = kk /! rate in
          assert (v b == v i);
          let j : usize = kk -! (b *! rate) in
          assert (v j == k - v i * v rate);
          assert (v j / 8 < 25);
          ()
        end
      end
    in
    FStar.Classical.forall_intro aux
#pop-options

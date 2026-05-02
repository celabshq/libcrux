module EquivImplSpec.Sponge.Arm64.Steps

(* ================================================================
   Per-step equivalences for the NEON / Arm64 backend
   (N=2, v_T=t_e_uint64x2_t).

   Four step lemmas, each parameterised over a lane [l : nat{l < 2}]:

   - [lemma_absorb_block_arm64] : extract_lane (impl_2__absorb_block …) l
                                  ≡ spec absorb_block (extract_lane … l)
   - [lemma_absorb_last_arm64]  : extract_lane (impl_2__absorb_final …) l
                                  ≡ spec absorb_final (extract_lane … l)
   - [lemma_squeeze_block_arm64]: per-lane state equality after keccakf1600
                                  + per-lane output equality from
                                  sq_lane_arm64 ≡ spec squeeze_state, at
                                  len=rate
   - [lemma_squeeze_last_arm64] : same shape, at len≤rate

   All four are proven by composing:
   - the admitted primitive equivalences in [EquivImplSpec.Sponge.Arm64]
     (arm64_sc_load_block / arm64_sc_load_last / arm64_sc_store_block),
   - the lane-wise keccakf1600 theorem [lemma_keccakf1600_arm64].

   The N=2 extract_lane is NOT an identity (unlike Portable), so it is
   carried through the statements rather than collapsed.

   The top-level [keccak2] driver in [src/generic_keccak/simd128.rs]
   has no F* counterpart yet, so loop-level composition is still out
   of scope; these step lemmas give callers the per-block primitives.
   ================================================================ *)

#set-options "--fuel 1 --ifuel 1 --z3rlimit 150"

open FStar.Mul
open Core_models

module G  = EquivImplSpec.Keccakf.Generic
module KA = EquivImplSpec.Keccakf.Arm64
module SA = EquivImplSpec.Sponge.Arm64
module I  = Libcrux_intrinsics.Arm64_extract

(* Bring Arm64 typeclass instances into scope so t_KeccakItem /
   t_Absorb / t_Squeeze2 at N=2 resolve. *)
let _ =
  let open Libcrux_intrinsics.Arm64_extract in
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Arm64 in
  ()


(* ================================================================
   Step 1: impl_2__absorb_block ≡ spec absorb_block, lane-wise.
   ================================================================ *)
let lemma_absorb_block_arm64
      (rate: usize)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
      (inputs: t_Array (t_Slice u8) (mk_usize 2))
      (start: usize)
      (l: nat{l < 2})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v start + v rate <= Seq.length #u8 (inputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) inputs)
      (ensures
        G.extract_lane (mk_usize 2) KA.lc_arm64
          (Libcrux_sha3.Generic_keccak.impl_2__absorb_block
             (mk_usize 2) #I.t_e_uint64x2_t rate ks inputs start)
            .Libcrux_sha3.Generic_keccak.f_st
          l
        ==
        Hacspec_sha3.Sponge.absorb_block
          (G.extract_lane (mk_usize 2) KA.lc_arm64
             ks.Libcrux_sha3.Generic_keccak.f_st l)
          (inputs.[ mk_usize l ].[ {
              Core_models.Ops.Range.f_start = start;
              Core_models.Ops.Range.f_end   = start +! rate } <:
            Core_models.Ops.Range.t_Range usize ])
          rate)
  = SA.arm64_sc_load_block rate ks.Libcrux_sha3.Generic_keccak.f_st inputs start l;
    let s1 =
      Libcrux_sha3.Traits.f_load_block
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
        #(mk_usize 2) #FStar.Tactics.Typeclasses.solve
        rate ks inputs start in
    KA.lemma_keccakf1600_arm64 s1 l


(* ================================================================
   Step 2: impl_2__absorb_final ≡ spec absorb_final, lane-wise.
   ================================================================ *)
let lemma_absorb_last_arm64
      (rate: usize)
      (delim: u8)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
      (inputs: t_Array (t_Slice u8) (mk_usize 2))
      (start: usize)
      (len: usize)
      (l: nat{l < 2})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len < v rate /\
        v start + v len <= Seq.length #u8 (inputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) inputs)
      (ensures
        G.extract_lane (mk_usize 2) KA.lc_arm64
          (Libcrux_sha3.Generic_keccak.impl_2__absorb_final
             (mk_usize 2) #I.t_e_uint64x2_t rate delim ks inputs start len)
            .Libcrux_sha3.Generic_keccak.f_st
          l
        ==
        Hacspec_sha3.Sponge.absorb_final
          (G.extract_lane (mk_usize 2) KA.lc_arm64
             ks.Libcrux_sha3.Generic_keccak.f_st l)
          (inputs.[ mk_usize l ])
          start len rate delim)
  = SA.arm64_sc_load_last rate delim
      ks.Libcrux_sha3.Generic_keccak.f_st inputs start len l;
    let s1 =
      Libcrux_sha3.Traits.f_load_last
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
        #(mk_usize 2) #FStar.Tactics.Typeclasses.solve
        rate delim ks inputs start len in
    KA.lemma_keccakf1600_arm64 s1 l


(* ================================================================
   Step 3: a full-rate squeeze block preceded by a permutation.

   At lane l:
     impl side (after permutation) : extract_lane ks'.f_st l
     impl side (output byte-stream): sq_lane_arm64 rate ks'.f_st outputs start rate l
     spec side                     : squeeze_state state'_l outputs[l] start rate
                                     where state'_l = keccak_f (extract_lane ks.f_st l)
   ================================================================ *)
let lemma_squeeze_block_arm64
      (rate: usize)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
      (outputs: t_Array (t_Slice u8) (mk_usize 2))
      (start: usize)
      (l: nat{l < 2})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v start + v rate <= Seq.length #u8 (outputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) outputs)
      (ensures (
        let ks' =
          Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
            (mk_usize 2) #I.t_e_uint64x2_t ks in
        let state_l' =
          Hacspec_sha3.Keccak_f.keccak_f
            (G.extract_lane (mk_usize 2) KA.lc_arm64
               ks.Libcrux_sha3.Generic_keccak.f_st l) in
        G.extract_lane (mk_usize 2) KA.lc_arm64
          ks'.Libcrux_sha3.Generic_keccak.f_st l
        == state_l' /\
        SA.sq_lane_arm64 rate ks'.Libcrux_sha3.Generic_keccak.f_st
          outputs start rate l
        ==
        Hacspec_sha3.Sponge.squeeze_state
          (Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]))
          state_l'
          (outputs.[ mk_usize l ] <: t_Array u8 _) start rate))
  = KA.lemma_keccakf1600_arm64 ks l;
    let ks' =
      Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
        (mk_usize 2) #I.t_e_uint64x2_t ks in
    SA.arm64_sc_store_block rate
      ks'.Libcrux_sha3.Generic_keccak.f_st outputs start rate l


(* ================================================================
   Step 4: a partial-rate trailing squeeze preceded by a permutation.

   Same shape as step 3 but with [len ≤ rate] instead of [len = rate].
   ================================================================ *)
let lemma_squeeze_last_arm64
      (rate: usize)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
      (outputs: t_Array (t_Slice u8) (mk_usize 2))
      (start: usize)
      (len: usize)
      (l: nat{l < 2})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 (outputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) outputs)
      (ensures (
        let ks' =
          Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
            (mk_usize 2) #I.t_e_uint64x2_t ks in
        let state_l' =
          Hacspec_sha3.Keccak_f.keccak_f
            (G.extract_lane (mk_usize 2) KA.lc_arm64
               ks.Libcrux_sha3.Generic_keccak.f_st l) in
        G.extract_lane (mk_usize 2) KA.lc_arm64
          ks'.Libcrux_sha3.Generic_keccak.f_st l
        == state_l' /\
        SA.sq_lane_arm64 rate ks'.Libcrux_sha3.Generic_keccak.f_st
          outputs start len l
        ==
        Hacspec_sha3.Sponge.squeeze_state
          (Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]))
          state_l'
          (outputs.[ mk_usize l ] <: t_Array u8 _) start len))
  = KA.lemma_keccakf1600_arm64 ks l;
    let ks' =
      Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
        (mk_usize 2) #I.t_e_uint64x2_t ks in
    SA.arm64_sc_store_block rate
      ks'.Libcrux_sha3.Generic_keccak.f_st outputs start len l


(* ================================================================
   Step 5: per-lane loop-invariant preservation across one
   (keccakf1600 ; squeeze trait call) iteration, byteform.

   Building block for the Arm64 driver-level [lemma_squeeze2_arm64]
   proof: the multi-block squeeze loop in
   [Libcrux_sha3.Generic_keccak.Simd128.squeeze2] preserves the
   per-lane invariant against the byteform [Hacspec_sha3.Sponge.squeeze].

   Pre-step invariant at iteration [i] (lane [l]):
     - extract_lane ks_pre.f_st l == iterate_keccak_f (i-1) lane_st_init
     - outputs_pre[l][k] == squeeze lane_st_init rate [k]  for k < i*rate

   Step:
     - ks_post = keccakf1600 ks_pre
     - outX'   = sq_lane_arm64 rate ks_post.f_st outputs_pre (i*rate) rate l

   Post-step invariant at iteration [i+1] (lane [l]):
     - extract_lane ks_post.f_st l == iterate_keccak_f i lane_st_init
     - outX'[k] == squeeze lane_st_init rate [k]           for k < (i+1)*rate

   3.2x faster than the prior recursive-spec form (Note C);
   84 s -> 26 s on cold cache, 239 -> 117 sub-queries, ~150 -> ~80 lines.
   ================================================================ *)
#push-options "--z3rlimit 400 --split_queries always"
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
       == iterate_keccak_f (v i) lane_st_init by right-add definitional
       unfold of iterate_keccak_f at fuel 1. *)
    lemma_squeeze_block_arm64 rate ks_pre outputs_pre (i *! rate) l;
    let ks_post =
        Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
          (mk_usize 2) #I.t_e_uint64x2_t ks_pre in
    let outX' =
        SA.sq_lane_arm64 rate ks_post.Libcrux_sha3.Generic_keccak.f_st
          outputs_pre (i *! rate) rate l in
    FStar.Math.Lemmas.distributivity_add_left (v i) 1 (v rate);
    let aux (k: nat{k < v outlen})
      : Lemma
        (k < (v i + 1) * v rate ==>
          Seq.index (outX' <: Seq.seq u8) k ==
          Seq.index
            (Hacspec_sha3.Sponge.squeeze outlen lane_st_init rate <: Seq.seq u8) k) =
      if k < (v i + 1) * v rate then begin
        let kk : usize = mk_usize k in
        assert (v kk == k);
        if k < v i * v rate then ()
        else begin
          assert (v i * v rate <= k);
          assert ((v i + 1) * v rate == v i * v rate + v rate);
          assert (k - v i * v rate < v rate);
          assert ((k - v i * v rate) / 8 < 25);
          (* For k in [i*rate, (i+1)*rate): byteform[k] uses block b = i. *)
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

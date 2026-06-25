module EquivImplSpec.Sponge.Avx2.SqueezeDriver

(* ================================================================
   Opaque-predicate scaffolding for the AVX2 (N=4) driver-level
   [squeeze4] functional proof (src/generic_keccak/simd256.rs).

   Mechanical N=2 -> N=4 transfer of
   [EquivImplSpec.Sponge.Arm64.SqueezeDriver].  The Z3-killer in the
   four-lane squeeze loop is the four-lane x all-output-bytes per-byte
   [forall] dragged through the block-loop invariant and step VC.  We
   seal it behind the opaque predicate [squeezed_upto] (mirrors
   [stored]/[modifies_range] from the AVX2 store_block closure),
   parameterised over the *already computed* spec array so the
   predicate carries no [Pure] precondition.

   The step lemmas restate the proven per-iteration engines
   ([Steps.lemma_squeeze_block_avx2], [Steps.lemma_squeeze_last_avx2],
   [avx2_sc_store_block]) into [squeezed_upto] terms, with the bare
   [forall] / [reveal_opaque] confined to each lemma's own body.  The
   driver's loop-step VC then only ever sees the opaque atom.

   [lemma_squeeze_one_step_avx2] below is the N=4 analog of arm64's
   [Steps.lemma_squeeze_one_step_arm64] (which lives in the arm64 Steps
   module); the AVX2 Steps module only ships the per-block
   [lemma_squeeze_block_avx2], so its forall-form composition is done
   here.
   ================================================================ *)

#set-options "--fuel 1 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models

module G     = EquivImplSpec.Keccakf.Generic
module KA    = EquivImplSpec.Keccakf.Avx2
module SA    = EquivImplSpec.Sponge.Avx2
module Steps = EquivImplSpec.Sponge.Avx2.Steps
module HS    = Hacspec_sha3.Sponge
module I     = Libcrux_intrinsics.Avx2_extract

(* Bring AVX2 typeclass instances into scope so t_Squeeze4 at N=4 resolves. *)
let _ =
  let open Libcrux_intrinsics.Avx2_extract in
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Avx2 in
  ()

(* ================================================================
   The opaque range predicate: "[out] agrees with [spec_out] on the
   prefix [0, hi)".
   ================================================================ *)
[@@ "opaque_to_smt"]
let squeezed_upto (out spec_out: Seq.seq u8) (hi: int) : prop =
  forall (k: nat). (k < hi /\ k < Seq.length out /\ k < Seq.length spec_out) ==>
    Seq.index out k == Seq.index spec_out k

(* [iterate_keccak_f 0] is the identity.  Proven here at fuel 1 so the
   driver (which runs at fuel 0 to keep [iterate_keccak_f] from unfolding
   into a recursive cascade) can cite it for the loop base case. *)
let lemma_iterate_keccak_f_zero (st: t_Array u64 (mk_usize 25))
  : Lemma (HS.iterate_keccak_f (mk_usize 0) st == st)
  = ()

(* Close: a full-length [squeezed_upto] + equal lengths => Seq equality.
   Single reveal point that discharges the driver's full-Seq ensures. *)
let lemma_squeezed_upto_full (out spec_out: Seq.seq u8)
  : Lemma
      (requires
        squeezed_upto out spec_out (Seq.length out) /\
        Seq.length out == Seq.length spec_out)
      (ensures out == spec_out)
  = reveal_opaque (`%squeezed_upto) squeezed_upto;
    Seq.lemma_eq_intro out spec_out

(* The spec [squeeze] returns [t_Array u8 OUTPUT_LEN]; its length is
   [OUTPUT_LEN] by the return type.  The driver excludes [squeeze] from
   its [using_facts_from] (to keep [iterate_keccak_f] from unfolding into
   a recursive cascade), which also drops [squeeze]'s return-type length
   refinement.  This lemma re-exposes the length as a citable fact so the
   driver can discharge [lemma_squeezed_upto_full]'s equal-length premise. *)
let lemma_squeeze_length
      (v_OUTPUT_LEN: usize)
      (state: t_Array u64 (mk_usize 25))
      (rate: usize)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v v_OUTPUT_LEN < v Core_models.Num.impl_usize__MAX - 200)
      (ensures
        Seq.length (HS.squeeze v_OUTPUT_LEN state rate <: Seq.seq u8) == v v_OUTPUT_LEN)
  = ()

(* ================================================================
   First block (offset 0, length rate, NO preceding keccakf).
   Establishes [squeezed_upto .. rate] off the initial state.
   ================================================================ *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_squeeze_first_step_avx2
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (s_init_st: t_Array I.t_Vec256 (mk_usize 25))
      (outputs: t_Array (t_Slice u8) (mk_usize 4))
      (len: usize)
      (l: nat{l < 4})
  : Lemma
      (requires (
        let outlen = Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]) in
        v len <= v rate /\
        v len <= v outlen /\
        v outlen < v Core_models.Num.impl_usize__MAX - 200 /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) outputs))
      (ensures (
        let outlen = Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]) in
        let outX' = SA.sq_lane_avx2 rate s_init_st outputs (mk_usize 0) len l in
        let lane_st_init = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st l in
        squeezed_upto (outX' <: Seq.seq u8)
                      (HS.squeeze outlen lane_st_init rate <: Seq.seq u8)
                      (v len)))
  = reveal_opaque (`%squeezed_upto) squeezed_upto;
    let outlen = Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]) in
    let lane_st_init = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st l in
    SA.avx2_sc_store_block rate s_init_st outputs (mk_usize 0) len l;
    let outX' = SA.sq_lane_avx2 rate s_init_st outputs (mk_usize 0) len l in
    let spec = HS.squeeze outlen lane_st_init rate in
    let aux (k: nat{k < v len})
      : Lemma (Seq.index (outX' <: Seq.seq u8) k == Seq.index (spec <: Seq.seq u8) k) =
      let kk : usize = mk_usize k in
      assert (v kk == k);
      assert (v kk / 8 < 25);
      FStar.Math.Lemmas.small_div k (v rate);
      assert (k / v rate == 0)
    in
    FStar.Classical.forall_intro aux
#pop-options

(* ================================================================
   Per-lane loop-invariant preservation across one
   (keccakf1600 ; squeeze trait call) iteration, byteform.  N=4 analog
   of arm64's [Steps.lemma_squeeze_one_step_arm64] (the AVX2 Steps
   module only exposes the per-block [lemma_squeeze_block_avx2]).
   ================================================================ *)
#push-options "--z3rlimit 400 --split_queries always"
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
    (* Harden the [i *! rate] offset + its bounds up front, deterministically,
       so the heavy requires context (recursive [squeeze] + per-byte [forall],
       no [using_facts_from] filter here) does not poison the overflow /
       sq_lane precondition check at the [lemma_squeeze_block_avx2] call site.
       Cf. squeeze2 plan: name the nonlinear product first. *)
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
    let aux (k: nat{k < v outlen})
      : Lemma
        (k < (v i + 1) * v rate ==>
          Seq.index (outX' <: Seq.seq u8) k ==
          Seq.index
            (HS.squeeze outlen lane_st_init rate <: Seq.seq u8) k) =
      if k < (v i + 1) * v rate then begin
        let kk : usize = mk_usize k in
        assert (v kk == k);
        if k < v i * v rate then ()
        else begin
          assert (v i * v rate <= k);
          assert ((v i + 1) * v rate == v i * v rate + v rate);
          assert (k - v i * v rate < v rate);
          assert ((k - v i * v rate) / 8 < 25);
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

(* ================================================================
   Middle block (offset i*rate, length rate, keccakf first).
   Thin restatement of [lemma_squeeze_one_step_avx2] into
   [squeezed_upto] terms: reveal confined to this body.
   ================================================================ *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_squeeze_mid_step_avx2
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
         squeezed_upto (outputs_pre.[ mk_usize l ] <: Seq.seq u8)
                       (HS.squeeze outlen lane_st_init rate <: Seq.seq u8)
                       (v i * v rate))))
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
        squeezed_upto (outX' <: Seq.seq u8)
                      (HS.squeeze outlen lane_st_init rate <: Seq.seq u8)
                      ((v i + 1) * v rate)))
  = reveal_opaque (`%squeezed_upto) squeezed_upto;
    lemma_squeeze_one_step_avx2 rate s_init_st ks_pre outputs_pre i l
#pop-options

(* ================================================================
   Trailing partial block (offset blocks*rate, length outlen-last,
   keccakf first).  Extends [squeezed_upto] to the full [outlen].
   ================================================================ *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_squeeze_tail_step_avx2
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (s_init_st: t_Array I.t_Vec256 (mk_usize 25))
      (ks_pre: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
      (outputs: t_Array (t_Slice u8) (mk_usize 4))
      (blocks: usize)
      (l: nat{l < 4})
  : Lemma
      (requires (
        let outlen = Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]) in
        v blocks >= 1 /\
        v blocks * v rate < v outlen /\
        v outlen - v blocks * v rate < v rate /\
        v outlen < v Core_models.Num.impl_usize__MAX - 200 /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) outputs /\
        (let lane_st_init = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st l in
         G.extract_lane (mk_usize 4) KA.lc_avx2
           ks_pre.Libcrux_sha3.Generic_keccak.f_st l
         == HS.iterate_keccak_f (blocks -! mk_usize 1) lane_st_init /\
         squeezed_upto (outputs.[ mk_usize l ] <: Seq.seq u8)
                       (HS.squeeze outlen lane_st_init rate <: Seq.seq u8)
                       (v blocks * v rate))))
      (ensures (
        let outlen = Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]) in
        let ks_post =
          Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
            (mk_usize 4) #I.t_Vec256 ks_pre in
        let last = blocks *! rate in
        let outX' =
          SA.sq_lane_avx2 rate ks_post.Libcrux_sha3.Generic_keccak.f_st
            outputs last (outlen -! last) l in
        let lane_st_init = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st l in
        squeezed_upto (outX' <: Seq.seq u8)
                      (HS.squeeze outlen lane_st_init rate <: Seq.seq u8)
                      (v outlen)))
  = reveal_opaque (`%squeezed_upto) squeezed_upto;
    let outlen = Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]) in
    let lane_st_init = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st l in
    let last = blocks *! rate in
    Steps.lemma_squeeze_last_avx2 rate ks_pre outputs last (outlen -! last) l;
    let ks_post =
      Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
        (mk_usize 4) #I.t_Vec256 ks_pre in
    let outX' =
      SA.sq_lane_avx2 rate ks_post.Libcrux_sha3.Generic_keccak.f_st
        outputs last (outlen -! last) l in
    let spec = HS.squeeze outlen lane_st_init rate in
    let aux (k: nat{k < v outlen})
      : Lemma (Seq.index (outX' <: Seq.seq u8) k == Seq.index (spec <: Seq.seq u8) k) =
      let kk : usize = mk_usize k in
      assert (v kk == k);
      if k < v last then ()
      else begin
        assert (v rate > 0);
        assert (v blocks * v rate <= k);
        assert (k - v blocks * v rate < v rate);
        assert ((k - v blocks * v rate) / 8 < 25);
        (* k / rate == blocks, deterministically: name the nonlinear product so
           the (k - blocks*rate) + blocks*rate == k step is linear, then read off
           the quotient via small_div + lemma_div_plus. *)
        FStar.Math.Lemmas.small_div (k - v blocks * v rate) (v rate);
        FStar.Math.Lemmas.lemma_div_plus (k - v blocks * v rate) (v blocks) (v rate);
        assert ((k - v blocks * v rate) + v blocks * v rate == k);
        assert (k / v rate == v blocks);
        let b : usize = kk /! rate in
        assert (v b == k / v rate);
        assert (v b == v blocks);
        let j : usize = kk -! (b *! rate) in
        assert (v j == k - v blocks * v rate);
        assert (v j / 8 < 25)
      end
    in
    FStar.Classical.forall_intro aux
#pop-options

(* ================================================================
   Bridge: the driver calls the trait method [f_squeeze4] (a 4-tuple of
   output slices); [sq_lane_avx2] is *defined* as that same call
   projected to a lane.  Package [out0..out3] into a 4-array and expose
   the per-lane equalities so the driver can phrase its proof in
   [f_squeeze4] terms while the step lemmas speak [sq_lane_avx2].
   ================================================================ *)
#push-options "--z3rlimit 200"
let lemma_sq_lane_is_f_squeeze4
      (rate: usize)
      (s: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
      (out0 out1 out2 out3: t_Slice u8)
      (outputs: t_Array (t_Slice u8) (mk_usize 4))
      (start len: usize)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 out0 /\
        Seq.length #u8 out0 == Seq.length #u8 out1 /\
        Seq.length #u8 out0 == Seq.length #u8 out2 /\
        Seq.length #u8 out0 == Seq.length #u8 out3 /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) outputs /\
        (outputs.[ mk_usize 0 ] <: t_Slice u8) == out0 /\
        (outputs.[ mk_usize 1 ] <: t_Slice u8) == out1 /\
        (outputs.[ mk_usize 2 ] <: t_Slice u8) == out2 /\
        (outputs.[ mk_usize 3 ] <: t_Slice u8) == out3)
      (ensures (
        let tup =
          Libcrux_sha3.Traits.f_squeeze4
            #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
            #I.t_Vec256 #FStar.Tactics.Typeclasses.solve
            rate s out0 out1 out2 out3 start len in
        let r0, r1, r2, r3 = tup in
        (r0 <: t_Slice u8)
          == SA.sq_lane_avx2 rate s.Libcrux_sha3.Generic_keccak.f_st outputs start len 0 /\
        (r1 <: t_Slice u8)
          == SA.sq_lane_avx2 rate s.Libcrux_sha3.Generic_keccak.f_st outputs start len 1 /\
        (r2 <: t_Slice u8)
          == SA.sq_lane_avx2 rate s.Libcrux_sha3.Generic_keccak.f_st outputs start len 2 /\
        (r3 <: t_Slice u8)
          == SA.sq_lane_avx2 rate s.Libcrux_sha3.Generic_keccak.f_st outputs start len 3))
  = assert (s == ({ Libcrux_sha3.Generic_keccak.f_st = s.Libcrux_sha3.Generic_keccak.f_st }
                  <: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256))
#pop-options

(* ================================================================
   Driver-facing wrappers: phrased over the [f_squeeze4] result 4-tuple
   that the extracted driver actually produces, for ALL FOUR lanes at once.
   Each bundles the [f_squeeze4]<->[sq_lane] bridge + the per-lane step.
   ================================================================ *)

(* First block (offset 0, length [len] <= rate, NO keccakf).  Used both
   for the blocks==0 whole-output case (len=outlen) and the blocks>0
   first block (len=rate). *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_squeeze_first_driver_avx2
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (s: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
      (out0 out1 out2 out3: t_Slice u8)
      (len: usize)
  : Lemma
      (requires (
        let outlen = Core_models.Slice.impl__len #u8 out0 in
        v len <= v rate /\
        v len <= v outlen /\
        v outlen < v Core_models.Num.impl_usize__MAX - 200 /\
        Core_models.Slice.impl__len #u8 out0 == Core_models.Slice.impl__len #u8 out1 /\
        Core_models.Slice.impl__len #u8 out0 == Core_models.Slice.impl__len #u8 out2 /\
        Core_models.Slice.impl__len #u8 out0 == Core_models.Slice.impl__len #u8 out3))
      (ensures (
        let tup =
          Libcrux_sha3.Traits.f_squeeze4
            #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
            #I.t_Vec256 #FStar.Tactics.Typeclasses.solve
            rate s out0 out1 out2 out3 (mk_usize 0) len in
        let r0, r1, r2, r3 = tup in
        let l0 = G.extract_lane (mk_usize 4) KA.lc_avx2 s.Libcrux_sha3.Generic_keccak.f_st 0 in
        let l1 = G.extract_lane (mk_usize 4) KA.lc_avx2 s.Libcrux_sha3.Generic_keccak.f_st 1 in
        let l2 = G.extract_lane (mk_usize 4) KA.lc_avx2 s.Libcrux_sha3.Generic_keccak.f_st 2 in
        let l3 = G.extract_lane (mk_usize 4) KA.lc_avx2 s.Libcrux_sha3.Generic_keccak.f_st 3 in
        squeezed_upto (r0 <: Seq.seq u8)
          (HS.squeeze (Core_models.Slice.impl__len #u8 out0) l0 rate <: Seq.seq u8) (v len) /\
        squeezed_upto (r1 <: Seq.seq u8)
          (HS.squeeze (Core_models.Slice.impl__len #u8 out1) l1 rate <: Seq.seq u8) (v len) /\
        squeezed_upto (r2 <: Seq.seq u8)
          (HS.squeeze (Core_models.Slice.impl__len #u8 out2) l2 rate <: Seq.seq u8) (v len) /\
        squeezed_upto (r3 <: Seq.seq u8)
          (HS.squeeze (Core_models.Slice.impl__len #u8 out3) l3 rate <: Seq.seq u8) (v len)))
  = let outputs : t_Array (t_Slice u8) (mk_usize 4) =
      let list = [out0; out1; out2; out3] in
      FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 4);
      Rust_primitives.Hax.array_of_list 4 list in
    assert ((outputs.[ mk_usize 0 ] <: t_Slice u8) == out0);
    assert ((outputs.[ mk_usize 1 ] <: t_Slice u8) == out1);
    assert ((outputs.[ mk_usize 2 ] <: t_Slice u8) == out2);
    assert ((outputs.[ mk_usize 3 ] <: t_Slice u8) == out3);
    lemma_sq_lane_is_f_squeeze4 rate s out0 out1 out2 out3 outputs (mk_usize 0) len;
    lemma_squeeze_first_step_avx2 rate s.Libcrux_sha3.Generic_keccak.f_st outputs len 0;
    lemma_squeeze_first_step_avx2 rate s.Libcrux_sha3.Generic_keccak.f_st outputs len 1;
    lemma_squeeze_first_step_avx2 rate s.Libcrux_sha3.Generic_keccak.f_st outputs len 2;
    lemma_squeeze_first_step_avx2 rate s.Libcrux_sha3.Generic_keccak.f_st outputs len 3
#pop-options

(* Middle block (offset i*rate, length rate, keccakf first). *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_squeeze_mid_driver_avx2
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (s_init_st: t_Array I.t_Vec256 (mk_usize 25))
      (s: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
      (out0 out1 out2 out3: t_Slice u8)
      (i: usize)
  : Lemma
      (requires (
        let outlen = Core_models.Slice.impl__len #u8 out0 in
        v i >= 1 /\
        v i * v rate + v rate <= v outlen /\
        v outlen < v Core_models.Num.impl_usize__MAX - 200 /\
        Core_models.Slice.impl__len #u8 out0 == Core_models.Slice.impl__len #u8 out1 /\
        Core_models.Slice.impl__len #u8 out0 == Core_models.Slice.impl__len #u8 out2 /\
        Core_models.Slice.impl__len #u8 out0 == Core_models.Slice.impl__len #u8 out3 /\
        (let l0 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 0 in
         let l1 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 1 in
         let l2 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 2 in
         let l3 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 3 in
         G.extract_lane (mk_usize 4) KA.lc_avx2 s.Libcrux_sha3.Generic_keccak.f_st 0
           == HS.iterate_keccak_f (i -! mk_usize 1) l0 /\
         G.extract_lane (mk_usize 4) KA.lc_avx2 s.Libcrux_sha3.Generic_keccak.f_st 1
           == HS.iterate_keccak_f (i -! mk_usize 1) l1 /\
         G.extract_lane (mk_usize 4) KA.lc_avx2 s.Libcrux_sha3.Generic_keccak.f_st 2
           == HS.iterate_keccak_f (i -! mk_usize 1) l2 /\
         G.extract_lane (mk_usize 4) KA.lc_avx2 s.Libcrux_sha3.Generic_keccak.f_st 3
           == HS.iterate_keccak_f (i -! mk_usize 1) l3 /\
         squeezed_upto (out0 <: Seq.seq u8)
           (HS.squeeze (Core_models.Slice.impl__len #u8 out0) l0 rate <: Seq.seq u8) (v i * v rate) /\
         squeezed_upto (out1 <: Seq.seq u8)
           (HS.squeeze (Core_models.Slice.impl__len #u8 out1) l1 rate <: Seq.seq u8) (v i * v rate) /\
         squeezed_upto (out2 <: Seq.seq u8)
           (HS.squeeze (Core_models.Slice.impl__len #u8 out2) l2 rate <: Seq.seq u8) (v i * v rate) /\
         squeezed_upto (out3 <: Seq.seq u8)
           (HS.squeeze (Core_models.Slice.impl__len #u8 out3) l3 rate <: Seq.seq u8) (v i * v rate))))
      (ensures (
        let ks_post =
          Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4) #I.t_Vec256 s in
        let tup =
          Libcrux_sha3.Traits.f_squeeze4
            #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
            #I.t_Vec256 #FStar.Tactics.Typeclasses.solve
            rate ks_post out0 out1 out2 out3 (i *! rate) rate in
        let r0, r1, r2, r3 = tup in
        let l0 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 0 in
        let l1 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 1 in
        let l2 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 2 in
        let l3 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 3 in
        G.extract_lane (mk_usize 4) KA.lc_avx2 ks_post.Libcrux_sha3.Generic_keccak.f_st 0
          == HS.iterate_keccak_f i l0 /\
        G.extract_lane (mk_usize 4) KA.lc_avx2 ks_post.Libcrux_sha3.Generic_keccak.f_st 1
          == HS.iterate_keccak_f i l1 /\
        G.extract_lane (mk_usize 4) KA.lc_avx2 ks_post.Libcrux_sha3.Generic_keccak.f_st 2
          == HS.iterate_keccak_f i l2 /\
        G.extract_lane (mk_usize 4) KA.lc_avx2 ks_post.Libcrux_sha3.Generic_keccak.f_st 3
          == HS.iterate_keccak_f i l3 /\
        squeezed_upto (r0 <: Seq.seq u8)
          (HS.squeeze (Core_models.Slice.impl__len #u8 out0) l0 rate <: Seq.seq u8) ((v i + 1) * v rate) /\
        squeezed_upto (r1 <: Seq.seq u8)
          (HS.squeeze (Core_models.Slice.impl__len #u8 out1) l1 rate <: Seq.seq u8) ((v i + 1) * v rate) /\
        squeezed_upto (r2 <: Seq.seq u8)
          (HS.squeeze (Core_models.Slice.impl__len #u8 out2) l2 rate <: Seq.seq u8) ((v i + 1) * v rate) /\
        squeezed_upto (r3 <: Seq.seq u8)
          (HS.squeeze (Core_models.Slice.impl__len #u8 out3) l3 rate <: Seq.seq u8) ((v i + 1) * v rate)))
  = let ks_post =
      Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4) #I.t_Vec256 s in
    let outputs : t_Array (t_Slice u8) (mk_usize 4) =
      let list = [out0; out1; out2; out3] in
      FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 4);
      Rust_primitives.Hax.array_of_list 4 list in
    assert ((outputs.[ mk_usize 0 ] <: t_Slice u8) == out0);
    assert ((outputs.[ mk_usize 1 ] <: t_Slice u8) == out1);
    assert ((outputs.[ mk_usize 2 ] <: t_Slice u8) == out2);
    assert ((outputs.[ mk_usize 3 ] <: t_Slice u8) == out3);
    lemma_sq_lane_is_f_squeeze4 rate ks_post out0 out1 out2 out3 outputs (i *! rate) rate;
    lemma_squeeze_mid_step_avx2 rate s_init_st s outputs i 0;
    lemma_squeeze_mid_step_avx2 rate s_init_st s outputs i 1;
    lemma_squeeze_mid_step_avx2 rate s_init_st s outputs i 2;
    lemma_squeeze_mid_step_avx2 rate s_init_st s outputs i 3
#pop-options

(* Trailing partial block (offset blocks*rate, length outlen-last, keccakf first). *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_squeeze_tail_driver_avx2
      (rate: usize{Libcrux_sha3.Proof_utils.valid_rate rate})
      (s_init_st: t_Array I.t_Vec256 (mk_usize 25))
      (s: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
      (out0 out1 out2 out3: t_Slice u8)
      (blocks: usize)
  : Lemma
      (requires (
        let outlen = Core_models.Slice.impl__len #u8 out0 in
        v blocks >= 1 /\
        v blocks * v rate < v outlen /\
        v outlen - v blocks * v rate < v rate /\
        v outlen < v Core_models.Num.impl_usize__MAX - 200 /\
        Core_models.Slice.impl__len #u8 out0 == Core_models.Slice.impl__len #u8 out1 /\
        Core_models.Slice.impl__len #u8 out0 == Core_models.Slice.impl__len #u8 out2 /\
        Core_models.Slice.impl__len #u8 out0 == Core_models.Slice.impl__len #u8 out3 /\
        (let l0 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 0 in
         let l1 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 1 in
         let l2 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 2 in
         let l3 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 3 in
         G.extract_lane (mk_usize 4) KA.lc_avx2 s.Libcrux_sha3.Generic_keccak.f_st 0
           == HS.iterate_keccak_f (blocks -! mk_usize 1) l0 /\
         G.extract_lane (mk_usize 4) KA.lc_avx2 s.Libcrux_sha3.Generic_keccak.f_st 1
           == HS.iterate_keccak_f (blocks -! mk_usize 1) l1 /\
         G.extract_lane (mk_usize 4) KA.lc_avx2 s.Libcrux_sha3.Generic_keccak.f_st 2
           == HS.iterate_keccak_f (blocks -! mk_usize 1) l2 /\
         G.extract_lane (mk_usize 4) KA.lc_avx2 s.Libcrux_sha3.Generic_keccak.f_st 3
           == HS.iterate_keccak_f (blocks -! mk_usize 1) l3 /\
         squeezed_upto (out0 <: Seq.seq u8)
           (HS.squeeze (Core_models.Slice.impl__len #u8 out0) l0 rate <: Seq.seq u8) (v blocks * v rate) /\
         squeezed_upto (out1 <: Seq.seq u8)
           (HS.squeeze (Core_models.Slice.impl__len #u8 out1) l1 rate <: Seq.seq u8) (v blocks * v rate) /\
         squeezed_upto (out2 <: Seq.seq u8)
           (HS.squeeze (Core_models.Slice.impl__len #u8 out2) l2 rate <: Seq.seq u8) (v blocks * v rate) /\
         squeezed_upto (out3 <: Seq.seq u8)
           (HS.squeeze (Core_models.Slice.impl__len #u8 out3) l3 rate <: Seq.seq u8) (v blocks * v rate))))
      (ensures (
        let outlen = Core_models.Slice.impl__len #u8 out0 in
        let ks_post =
          Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4) #I.t_Vec256 s in
        let last = blocks *! rate in
        let tup =
          Libcrux_sha3.Traits.f_squeeze4
            #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
            #I.t_Vec256 #FStar.Tactics.Typeclasses.solve
            rate ks_post out0 out1 out2 out3 last (outlen -! last) in
        let r0, r1, r2, r3 = tup in
        let l0 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 0 in
        let l1 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 1 in
        let l2 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 2 in
        let l3 = G.extract_lane (mk_usize 4) KA.lc_avx2 s_init_st 3 in
        squeezed_upto (r0 <: Seq.seq u8)
          (HS.squeeze (Core_models.Slice.impl__len #u8 out0) l0 rate <: Seq.seq u8) (v outlen) /\
        squeezed_upto (r1 <: Seq.seq u8)
          (HS.squeeze (Core_models.Slice.impl__len #u8 out1) l1 rate <: Seq.seq u8) (v outlen) /\
        squeezed_upto (r2 <: Seq.seq u8)
          (HS.squeeze (Core_models.Slice.impl__len #u8 out2) l2 rate <: Seq.seq u8) (v outlen) /\
        squeezed_upto (r3 <: Seq.seq u8)
          (HS.squeeze (Core_models.Slice.impl__len #u8 out3) l3 rate <: Seq.seq u8) (v outlen)))
  = let outlen = Core_models.Slice.impl__len #u8 out0 in
    let ks_post =
      Libcrux_sha3.Generic_keccak.impl_2__keccakf1600 (mk_usize 4) #I.t_Vec256 s in
    let last = blocks *! rate in
    let outputs : t_Array (t_Slice u8) (mk_usize 4) =
      let list = [out0; out1; out2; out3] in
      FStar.Pervasives.assert_norm (Prims.eq2 (List.Tot.length list) 4);
      Rust_primitives.Hax.array_of_list 4 list in
    assert ((outputs.[ mk_usize 0 ] <: t_Slice u8) == out0);
    assert ((outputs.[ mk_usize 1 ] <: t_Slice u8) == out1);
    assert ((outputs.[ mk_usize 2 ] <: t_Slice u8) == out2);
    assert ((outputs.[ mk_usize 3 ] <: t_Slice u8) == out3);
    lemma_sq_lane_is_f_squeeze4 rate ks_post out0 out1 out2 out3 outputs last (outlen -! last);
    lemma_squeeze_tail_step_avx2 rate s_init_st s outputs blocks 0;
    lemma_squeeze_tail_step_avx2 rate s_init_st s outputs blocks 1;
    lemma_squeeze_tail_step_avx2 rate s_init_st s outputs blocks 2;
    lemma_squeeze_tail_step_avx2 rate s_init_st s outputs blocks 3
#pop-options

(* ================================================================
   Driver-arithmetic bridges (placed LAST so they never enter the SMT
   context of the step lemmas above — their div/mod assertions are
   fragile and adding lemmas earlier perturbed them).

   See [EquivImplSpec.Sponge.Arm64.SqueezeDriver] for the rationale.
   [last] = [outlen -! (outlen %! rate)] = [(outlen/rate)*rate]. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
(* else branch (exact multiple): [~(last < outlen)] => blocks*rate == outlen *)
let lemma_exact_multiple (outlen rate: usize)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        ~(v (outlen -! (outlen %! rate <: usize)) < v outlen))
      (ensures v (outlen /! rate) * v rate == v outlen)
  = FStar.Math.Lemmas.lemma_div_mod (v outlen) (v rate)

(* tail branch (partial block): hand the composer the div_mod EQUALITIES. *)
let lemma_blocks_rate_split (outlen rate: usize)
  : Lemma
      (requires Libcrux_sha3.Proof_utils.valid_rate rate)
      (ensures
        v (outlen /! rate) * v rate + v (outlen %! rate) == v outlen /\
        v (outlen -! (outlen %! rate <: usize)) == v (outlen /! rate) * v rate /\
        v (outlen %! rate) < v rate)
  = FStar.Math.Lemmas.lemma_div_mod (v outlen) (v rate)
#pop-options

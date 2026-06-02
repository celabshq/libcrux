module EquivImplSpec.Sponge.Avx2.Steps

(* ================================================================
   Per-step equivalences for the AVX2 backend
   (N=4, v_T=t_Vec256).

   Mirrors [EquivImplSpec.Sponge.Arm64.Steps] at N=4.

   Four step lemmas, each parameterised over a lane [l : nat{l < 4}]:

   - [lemma_absorb_block_avx2] : extract_lane (impl_2__absorb_block …) l
                                 ≡ spec absorb_block (extract_lane … l)
   - [lemma_absorb_last_avx2]  : extract_lane (impl_2__absorb_final …) l
                                 ≡ spec absorb_final (extract_lane … l)
   - [lemma_squeeze_block_avx2]: per-lane state equality after keccakf1600
                                 + per-lane output equality from
                                 sq_lane_avx2 ≡ spec squeeze_state, at
                                 len=rate
   - [lemma_squeeze_last_avx2] : same shape, at len≤rate

   All four are proven by composing:
   - the per-backend primitive equivalences in [EquivImplSpec.Sponge.Avx2]
     (avx2_sc_load_block / avx2_sc_load_last / avx2_sc_store_block),
   - the lane-wise keccakf1600 theorem [lemma_keccakf1600_avx2]
     (which itself rests on the seven lane-correctness primitives in
     [EquivImplSpec.Keccakf.Avx2], discharged via the AVX2 intrinsic
     SMTPats in [Libcrux_intrinsics.Avx2_extract] — external trust).

   The N=4 extract_lane is NOT an identity, so it is carried through
   the statements rather than collapsed.
   ================================================================ *)

#set-options "--fuel 1 --ifuel 1 --z3rlimit 150"

open FStar.Mul
open Core_models

module G  = EquivImplSpec.Keccakf.Generic
module KA = EquivImplSpec.Keccakf.Avx2
module SA = EquivImplSpec.Sponge.Avx2
module I  = Libcrux_intrinsics.Avx2_extract

(* Bring AVX2 typeclass instances into scope so t_KeccakItem /
   t_Absorb / t_Squeeze4 at N=4 resolve. *)
let _ =
  let open Libcrux_intrinsics.Avx2_extract in
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Avx2 in
  ()


(* ================================================================
   Step 1: impl_2__absorb_block ≡ spec absorb_block, lane-wise.
   ================================================================ *)
let lemma_absorb_block_avx2
      (rate: usize)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
      (inputs: t_Array (t_Slice u8) (mk_usize 4))
      (start: usize)
      (l: nat{l < 4})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v start + v rate <= Seq.length #u8 (inputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) inputs)
      (ensures
        G.extract_lane (mk_usize 4) KA.lc_avx2
          (Libcrux_sha3.Generic_keccak.impl_2__absorb_block
             (mk_usize 4) #I.t_Vec256 rate ks inputs start)
            .Libcrux_sha3.Generic_keccak.f_st
          l
        ==
        Hacspec_sha3.Sponge.absorb_block
          (G.extract_lane (mk_usize 4) KA.lc_avx2
             ks.Libcrux_sha3.Generic_keccak.f_st l)
          (inputs.[ mk_usize l ].[ {
              Core_models.Ops.Range.f_start = start;
              Core_models.Ops.Range.f_end   = start +! rate } <:
            Core_models.Ops.Range.t_Range usize ])
          rate)
  = SA.avx2_sc_load_block rate ks.Libcrux_sha3.Generic_keccak.f_st inputs start l;
    let s1 =
      Libcrux_sha3.Traits.f_load_block
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
        #(mk_usize 4) #FStar.Tactics.Typeclasses.solve
        rate ks inputs start in
    KA.lemma_keccakf1600_avx2 s1 l


(* ================================================================
   Step 2: impl_2__absorb_final ≡ spec absorb_final, lane-wise.
   ================================================================ *)
let lemma_absorb_last_avx2
      (rate: usize)
      (delim: u8)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
      (inputs: t_Array (t_Slice u8) (mk_usize 4))
      (start: usize)
      (len: usize)
      (l: nat{l < 4})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len < v rate /\
        v start + v len <= Seq.length #u8 (inputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) inputs)
      (ensures
        G.extract_lane (mk_usize 4) KA.lc_avx2
          (Libcrux_sha3.Generic_keccak.impl_2__absorb_final
             (mk_usize 4) #I.t_Vec256 rate delim ks inputs start len)
            .Libcrux_sha3.Generic_keccak.f_st
          l
        ==
        Hacspec_sha3.Sponge.absorb_final
          (G.extract_lane (mk_usize 4) KA.lc_avx2
             ks.Libcrux_sha3.Generic_keccak.f_st l)
          (inputs.[ mk_usize l ])
          start len rate delim)
  = SA.avx2_sc_load_last rate delim
      ks.Libcrux_sha3.Generic_keccak.f_st inputs start len l;
    let s1 =
      Libcrux_sha3.Traits.f_load_last
        #(Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
        #(mk_usize 4) #FStar.Tactics.Typeclasses.solve
        rate delim ks inputs start len in
    KA.lemma_keccakf1600_avx2 s1 l


(* ================================================================
   Step 3: a full-rate squeeze block preceded by a permutation.
   ================================================================ *)
let lemma_squeeze_block_avx2
      (rate: usize)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
      (outputs: t_Array (t_Slice u8) (mk_usize 4))
      (start: usize)
      (l: nat{l < 4})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v start + v rate <= Seq.length #u8 (outputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) outputs)
      (ensures (
        let ks' =
          Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
            (mk_usize 4) #I.t_Vec256 ks in
        let state_l' =
          Hacspec_sha3.Keccak_f.keccak_f
            (G.extract_lane (mk_usize 4) KA.lc_avx2
               ks.Libcrux_sha3.Generic_keccak.f_st l) in
        G.extract_lane (mk_usize 4) KA.lc_avx2
          ks'.Libcrux_sha3.Generic_keccak.f_st l
        == state_l' /\
        SA.sq_lane_avx2 rate ks'.Libcrux_sha3.Generic_keccak.f_st
          outputs start rate l
        ==
        Hacspec_sha3.Sponge.squeeze_state
          (Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]))
          state_l'
          (outputs.[ mk_usize l ] <: t_Array u8 _) start rate))
  = KA.lemma_keccakf1600_avx2 ks l;
    let ks' =
      Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
        (mk_usize 4) #I.t_Vec256 ks in
    SA.avx2_sc_store_block rate
      ks'.Libcrux_sha3.Generic_keccak.f_st outputs start rate l


(* ================================================================
   Step 4: a partial-rate trailing squeeze preceded by a permutation.
   ================================================================ *)
let lemma_squeeze_last_avx2
      (rate: usize)
      (ks: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
      (outputs: t_Array (t_Slice u8) (mk_usize 4))
      (start: usize)
      (len: usize)
      (l: nat{l < 4})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        v len <= v rate /\
        v start + v len <= Seq.length #u8 (outputs.[ mk_usize 0 ]) /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) outputs)
      (ensures (
        let ks' =
          Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
            (mk_usize 4) #I.t_Vec256 ks in
        let state_l' =
          Hacspec_sha3.Keccak_f.keccak_f
            (G.extract_lane (mk_usize 4) KA.lc_avx2
               ks.Libcrux_sha3.Generic_keccak.f_st l) in
        G.extract_lane (mk_usize 4) KA.lc_avx2
          ks'.Libcrux_sha3.Generic_keccak.f_st l
        == state_l' /\
        SA.sq_lane_avx2 rate ks'.Libcrux_sha3.Generic_keccak.f_st
          outputs start len l
        ==
        Hacspec_sha3.Sponge.squeeze_state
          (Core_models.Slice.impl__len #u8 (outputs.[ mk_usize l ]))
          state_l'
          (outputs.[ mk_usize l ] <: t_Array u8 _) start len))
  = KA.lemma_keccakf1600_avx2 ks l;
    let ks' =
      Libcrux_sha3.Generic_keccak.impl_2__keccakf1600
        (mk_usize 4) #I.t_Vec256 ks in
    SA.avx2_sc_store_block rate
      ks'.Libcrux_sha3.Generic_keccak.f_st outputs start len l

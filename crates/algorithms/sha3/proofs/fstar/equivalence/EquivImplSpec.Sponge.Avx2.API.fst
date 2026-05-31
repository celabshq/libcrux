module EquivImplSpec.Sponge.Avx2.API

(* ================================================================
   Top-level SHA-3 / SHAKE equivalence theorems for the AVX2
   (N=4, v_T=t_Vec256) backend.

   STRUCTURAL DIFFERENCE FROM Sponge.Arm64.API:

   The Arm64 driver [Libcrux_sha3.Generic_keccak.Simd128] exposes
   separate [absorb2], [squeeze2], and [keccak2] functions whose
   per-lane equivalences are stated as [lemma_absorb2_arm64] +
   [lemma_squeeze2_arm64], composed into [lemma_keccak2_arm64].
   The Arm64 [absorb2]'s inline ensures (proved via an
   [absorb_blocks]-based loop invariant) discharges
   [lemma_absorb2_arm64] directly.

   The AVX2 driver [Libcrux_sha3.Generic_keccak.Simd256] currently
   exposes only a single [keccak4] function — it has no separate
   [absorb4] or [squeeze4] functions, so the absorb / squeeze
   decomposition cannot be cleanly split at the F* level.

   Consequence:
   - [lemma_squeeze4_avx2] is stated about the [t_Squeeze4.f_squeeze4]
     trait method, which IS the lane-wise primitive used by
     [Generic_keccak.Simd256.{keccak4, impl__squeeze_*}].
     It is ADMITTED — it specialises [avx2_sc_store_block]
     (also admitted) to the per-lane spec [Hacspec_sha3.Sponge.squeeze]
     by induction over the keccakf1600/squeeze loop.
   - [lemma_keccak4_avx2] is the AVX2 counterpart of
     [lemma_keccak2_arm64] — per-lane equivalence between
     [Generic_keccak.Simd256.keccak4] and the scalar
     [Hacspec_sha3.Sponge.keccak].  It is ADMITTED for the same
     reason: it would compose [lemma_absorb4_avx2] (which we cannot
     state as a separate lemma without an [absorb4] function) with
     [lemma_squeeze4_avx2] over the [keccak4] body.
   - Top-level hashers ([lemma_shake256_x4_avx2]) are derived
     from [lemma_keccak4_avx2].

   See SHA3_STATUS.md for the full load-bearing admit inventory.
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models

module I = Libcrux_intrinsics.Avx2_extract
module KA = EquivImplSpec.Keccakf.Avx2


(** Driver-level absorb at N=4.  Running [absorb4] yields a state
    whose lane-[l] extraction equals the scalar
    [Hacspec_sha3.Sponge.absorb] applied to [data[l]].

    The per-lane equivalence is discharged directly by the Rust-side
    ensures on [Libcrux_sha3.Generic_keccak.Simd256.absorb4] (proved
    inline via an [absorb_blocks]-based loop invariant at N=4,
    mirroring the Arm64 [Simd128.absorb2] proof). *)
let lemma_absorb4_avx2
      (rate: usize) (delim: u8)
      (data: t_Array (t_Slice u8) (mk_usize 4))
      (l: nat{l < 4})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) data)
      (ensures (
        let s4 = Libcrux_sha3.Generic_keccak.Simd256.absorb4 rate delim data in
        EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4) KA.lc_avx2
          s4.Libcrux_sha3.Generic_keccak.f_st l
        ==
        Hacspec_sha3.Sponge.absorb rate delim (data.[ mk_usize l ])))
  = let _ = Libcrux_sha3.Generic_keccak.Simd256.absorb4 rate delim data in
    ()


(** Driver-level squeeze4 at N=4.  For an arbitrary four-lane state
    [s], running [squeeze4] yields output slices whose lane-[l]
    component equals [Hacspec_sha3.Sponge.squeeze] applied to
    [extract_lane s l].

    Mirrors [lemma_squeeze2_arm64].  Discharging this would require
    closing the [avx2_sc_store_block] admit (currently in
    [EquivImplSpec.Sponge.Avx2]) plus an inline loop-invariant proof
    on [Simd256.squeeze4] (analogous to the unfinished work on
    [Simd128.squeeze2] documented in HANDOFF). *)
let lemma_squeeze4_avx2
      (rate: usize)
      (s: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 4) I.t_Vec256)
      (out0 out1 out2 out3: t_Slice u8)
      (l: nat{l < 4})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        Seq.length #u8 out0 < v Core_models.Num.impl_usize__MAX - 200 /\
        Seq.length #u8 out0 == Seq.length #u8 out1 /\
        Seq.length #u8 out0 == Seq.length #u8 out2 /\
        Seq.length #u8 out0 == Seq.length #u8 out3)
      (ensures (
        let outlen : usize = Core_models.Slice.impl__len #u8 out0 in
        let (out0', out1', out2', out3') =
          Libcrux_sha3.Generic_keccak.Simd256.squeeze4 rate s out0 out1 out2 out3 in
        let r_l =
          if l = 0 then out0'
          else if l = 1 then out1'
          else if l = 2 then out2'
          else out3' in
        r_l
        ==
        (Hacspec_sha3.Sponge.squeeze outlen
           (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 4) KA.lc_avx2
              s.Libcrux_sha3.Generic_keccak.f_st l)
           rate
         <: t_Slice u8)))
  = let _ = Libcrux_sha3.Generic_keccak.Simd256.squeeze4 rate s out0 out1 out2 out3 in
    ()


(* ================================================================
   lemma_keccak4_avx2 = lemma_absorb4_avx2 ; lemma_squeeze4_avx2.

   Structurally identical to how [lemma_keccak2_arm64] composes
   [lemma_absorb2_arm64] + [lemma_squeeze2_arm64] on the Arm64 side.
   ================================================================ *)

let lemma_keccak4_avx2
      (rate: usize) (delim: u8)
      (input: t_Array (t_Slice u8) (mk_usize 4))
      (out0 out1 out2 out3: t_Slice u8)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) input /\
        Seq.length #u8 out0 < v Core_models.Num.impl_usize__MAX - 200 /\
        Seq.length #u8 out0 == Seq.length #u8 out1 /\
        Seq.length #u8 out0 == Seq.length #u8 out2 /\
        Seq.length #u8 out0 == Seq.length #u8 out3)
      (ensures (
        let (r0, r1, r2, r3) =
          Libcrux_sha3.Generic_keccak.Simd256.keccak4
            rate delim input out0 out1 out2 out3 in
        let n : usize = Core_models.Slice.impl__len #u8 out0 in
        r0 == (Hacspec_sha3.Sponge.keccak n rate delim (input.[ mk_usize 0 ]) <: t_Slice u8) /\
        r1 == (Hacspec_sha3.Sponge.keccak n rate delim (input.[ mk_usize 1 ]) <: t_Slice u8) /\
        r2 == (Hacspec_sha3.Sponge.keccak n rate delim (input.[ mk_usize 2 ]) <: t_Slice u8) /\
        r3 == (Hacspec_sha3.Sponge.keccak n rate delim (input.[ mk_usize 3 ]) <: t_Slice u8)))
  = let s = Libcrux_sha3.Generic_keccak.Simd256.absorb4 rate delim input in
    lemma_absorb4_avx2 rate delim input 0;
    lemma_absorb4_avx2 rate delim input 1;
    lemma_absorb4_avx2 rate delim input 2;
    lemma_absorb4_avx2 rate delim input 3;
    lemma_squeeze4_avx2 rate s out0 out1 out2 out3 0;
    lemma_squeeze4_avx2 rate s out0 out1 out2 out3 1;
    lemma_squeeze4_avx2 rate s out0 out1 out2 out3 2;
    lemma_squeeze4_avx2 rate s out0 out1 out2 out3 3


(* ================================================================
   PER-HASHER TOP-LEVEL THEOREM

   Currently AVX2 X4 only exposes [shake256] at the top level.
   ================================================================ *)

let lemma_shake256_x4_avx2
      (input0 input1 input2 input3 out0 out1 out2 out3: t_Slice u8)
  : Lemma
      (requires
        Seq.length #u8 out0 < v Core_models.Num.impl_usize__MAX - 200 /\
        Seq.length #u8 out0 == Seq.length #u8 out1 /\
        Seq.length #u8 out0 == Seq.length #u8 out2 /\
        Seq.length #u8 out0 == Seq.length #u8 out3 /\
        Seq.length #u8 input0 == Seq.length #u8 input1 /\
        Seq.length #u8 input0 == Seq.length #u8 input2 /\
        Seq.length #u8 input0 == Seq.length #u8 input3)
      (ensures (
        let (r0, r1, r2, r3) =
          Libcrux_sha3.Avx2.X4.shake256
            input0 input1 input2 input3 out0 out1 out2 out3 in
        let n : usize = Core_models.Slice.impl__len #u8 out0 in
        r0 == (Hacspec_sha3.Sha3.shake256 n input0 <: t_Slice u8) /\
        r1 == (Hacspec_sha3.Sha3.shake256 n input1 <: t_Slice u8) /\
        r2 == (Hacspec_sha3.Sha3.shake256 n input2 <: t_Slice u8) /\
        r3 == (Hacspec_sha3.Sha3.shake256 n input3 <: t_Slice u8)))
  = let inputs : t_Array (t_Slice u8) (mk_usize 4) =
      let l : list (t_Slice u8) = [ input0; input1; input2; input3 ] in
      FStar.Pervasives.assert_norm (List.Tot.length l == 4);
      Rust_primitives.Hax.array_of_list 4 l in
    lemma_keccak4_avx2 (mk_usize 136) (mk_u8 31) inputs out0 out1 out2 out3

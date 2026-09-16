module EquivImplSpec.Sponge.Avx2.Driver

(* ================================================================
   Driver-level (absorb4, squeeze4, keccak4) equivalence theorems for
   the AVX2 (N=4, v_T=t_Vec256) backend, factored out of
   [EquivImplSpec.Correctness.Avx2] to break a dependency cycle:

       Libcrux_sha3.Avx2.X4  -- needs lemma_keccak4_avx2 to wire its
                                function-level [Hacspec_sha3.Sponge.keccak]
                                ensures (in [Libcrux_sha3.Avx2.X4.fst]'s
                                body proof of shake256).
       Libcrux_sha3.Avx2.X4  <- referenced by the per-hasher lemma
                                [lemma_shake256_x4_avx2] in
                                [EquivImplSpec.Correctness.Avx2].

   Splitting the driver lemmas into their own (X4-free) module breaks
   the cycle, exactly as [EquivImplSpec.Sponge.Arm64.Driver] does for
   [Libcrux_sha3.Neon] on the Arm64 side.

   LAYER STRUCTURE:

     lemma_absorb4_avx2 + lemma_squeeze4_avx2
       ↓
     lemma_keccak4_avx2  : per-lane keccak4 ≡ scalar keccak

   The AVX2 driver [Libcrux_sha3.Generic_keccak.Simd256] exposes
   separate [absorb4], [squeeze4], and [keccak4] functions (mirroring
   the Arm64 [Simd128] split), so the absorb / squeeze decomposition
   splits cleanly at the F* level.  All are proven (no admits); the
   only external trust is the AVX2 intrinsic layer inherited via
   [lemma_keccakf1600_avx2].
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models

module I = Libcrux_intrinsics.Avx2_sha3_views
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

    Mirrors [lemma_squeeze2_arm64]: discharged directly by the Rust-side
    ensures on [Simd256.squeeze4] (an inline loop-invariant proof at N=4,
    the AVX2 analogue of [Simd128.squeeze2]). *)
#push-options "--z3rlimit 400 --using_facts_from '* -Hacspec_sha3.Sponge.squeeze -EquivImplSpec.Keccakf.Generic.extract_lane -Libcrux_sha3.Generic_keccak.Simd256.squeeze4_blocks'"
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
    (* squeeze4's post gives out{0,1,2,3}' == squeeze(extract_lane s {0,1,2,3}); pick the
       concrete lane so the symbolic `if l=0 ...` if-ladder collapses to the matching
       conjunct.  Mirrors lemma_squeeze2_arm64: without pinning the lane, the symbolic
       if forces a case split that cascades and saturates the whole lemma. *)
    (match l with
     | 0 -> ()
     | 1 -> ()
     | 2 -> ()
     | _ -> ())
#pop-options


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


(* `slices_same_len` quantifies over an unbounded usize, so at a symbolic index
   the projection cannot reduce. Dispatch the four in-range indices. *)
let lemma_slices_same_len4 (arr: t_Array (t_Slice u8) (mk_usize 4))
  : Lemma
      (requires
        Seq.length #u8 (arr.[ mk_usize 0 ]) == Seq.length #u8 (arr.[ mk_usize 1 ]) /\
        Seq.length #u8 (arr.[ mk_usize 0 ]) == Seq.length #u8 (arr.[ mk_usize 2 ]) /\
        Seq.length #u8 (arr.[ mk_usize 0 ]) == Seq.length #u8 (arr.[ mk_usize 3 ]))
      (ensures Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 4) arr)
  = introduce forall (i: usize).
        b2t (i <. mk_usize 4 <: bool) ==>
        b2t ((Core_models.Slice.impl__len #u8 (arr.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
             (Core_models.Slice.impl__len #u8 (arr.[ i ] <: t_Slice u8) <: usize) <: bool)
    with introduce _ ==> _
    with _. (if v i = 0 then assert (i == mk_usize 0)
             else if v i = 1 then assert (i == mk_usize 1)
             else if v i = 2 then assert (i == mk_usize 2)
             else assert (i == mk_usize 3))

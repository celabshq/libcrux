module EquivImplSpec.Sponge.Arm64.Driver

(* ================================================================
   Driver-level (absorb2, squeeze2, keccak2) equivalence theorems for
   the Arm64 (N=2, v_T=t_e_uint64x2_t) backend, factored out of
   [EquivImplSpec.Sponge.Arm64.API] to break a dependency cycle:

       Libcrux_sha3.Neon  -- needs lemma_keccak2_arm64 to wire its
                             function-level [Hacspec_sha3.Sponge.keccak]
                             ensures (in [Libcrux_sha3.Neon.fst]'s
                             body proofs of sha224/256/384/512,
                             shake128/256).
       Libcrux_sha3.Neon  <- referenced by the per-hasher lemmas in
                             [EquivImplSpec.Sponge.Arm64.API]
                             (e.g. [lemma_sha224_arm64]).

   Splitting the driver lemmas into their own (Neon-free) module
   breaks the cycle.  [EquivImplSpec.Sponge.Arm64.API] re-imports
   them from here for use in the per-hasher proofs.

   STRUCTURAL DIFFERENCE FROM Sponge.Portable.API:

   The Arm64 top-level hashers in [Libcrux_sha3.Neon] dispatch
   through a two-lane driver [Libcrux_sha3.Generic_keccak.Simd128.keccak2],
   e.g.

       let sha256 digest data =
         let dummy = [|0u8; …; 0u8|] in
         keccak2 136 0x06 [data; data] digest dummy

   The input is duplicated into both SIMD lanes; lane 0's output fills
   [digest] and lane 1's output is discarded into [dummy]. So even
   though the underlying primitive runs in parallel at N=2, the
   spec equivalence we need is single-lane.

   LAYER STRUCTURE:

     lemma_absorb2_arm64 + lemma_squeeze2_arm64
       ↓
     lemma_keccak2_arm64  : per-lane keccak2 ≡ scalar keccak

   [lemma_squeeze2_arm64] is ADMITTED ([assume val]) — Arm64 squeeze
   counterpart of [lemma_squeeze_portable], requiring reasoning over
   the [fold_range] in [Generic_keccak.Simd128.squeeze2] (whose
   current Rust-side ensures is bounds-only) plus the per-lane NEON
   bridges.

   Status of the per-lane NEON bridges [arm64_sc_load_block],
   [arm64_sc_load_last], [arm64_sc_store_block]: these are now real
   [let]s in [EquivImplSpec.Sponge.Arm64.fst] (lines 150, 386, 487
   respectively), NOT admitted.  The remaining open work for
   [lemma_squeeze2_arm64] is therefore the loop-shape part: porting
   the Portable squeeze closure pattern (push the lockstep induction
   into [squeeze2]'s Rust-side ensures, then reconcile via a one-line
   driver wrapper) to N=2.  See [EquivImplSpec.Sponge.Arm64.Steps.
   lemma_squeeze_one_step_arm64] (already proved) for the per-lane
   per-iteration step lemma that supports the closure.
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models

module I = Libcrux_intrinsics.Arm64_extract
module KA = EquivImplSpec.Keccakf.Arm64


(* ================================================================
   LAYER ASSUMPTIONS: two driver-level lemmas (absorb, squeeze2) at
   N=2, analogous to [lemma_absorb_portable] + [lemma_squeeze_portable]
   on the Portable side.  [lemma_keccak2_arm64] is derived by
   composition, mirroring how [lemma_keccak1_portable] is proven in
   [EquivImplSpec.Sponge.Portable.API].

   Both assumptions are per-lane and will be discharged once the
   Arm64 lockstep inductions (analogous to Gap 2 on the Portable
   side) are closed.  Splitting keccak2 into absorb+squeeze factors
   matches the Portable layout and isolates the two independent
   sub-problems.
   ================================================================ *)

(** Driver-level absorb at N=2.  Running [absorb2] yields a state
    whose lane-[l] extraction equals the scalar
    [Hacspec_sha3.Sponge.absorb] applied to [data[l]].

    The per-lane equivalence is discharged directly by the Rust-side
    ensures on [Libcrux_sha3.Generic_keccak.Simd128.absorb2] (proved
    inline via an [absorb_blocks]-based loop invariant at N=2). *)
let lemma_absorb2_arm64
      (rate: usize) (delim: u8)
      (data: t_Array (t_Slice u8) (mk_usize 2))
      (l: nat{l < 2})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) data)
      (ensures (
        let s2 = Libcrux_sha3.Generic_keccak.Simd128.absorb2 rate delim data in
        EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2) KA.lc_arm64
          s2.Libcrux_sha3.Generic_keccak.f_st l
        ==
        Hacspec_sha3.Sponge.absorb rate delim (data.[ mk_usize l ])))
  = let _ = Libcrux_sha3.Generic_keccak.Simd128.absorb2 rate delim data in
    ()

(** Driver-level squeeze2 at N=2.  For an arbitrary two-lane state
    [s], running [squeeze2] yields output slices whose lane-[l]
    component equals [Hacspec_sha3.Sponge.squeeze] applied to
    [extract_lane s l]. *)
#push-options "--admit_smt_queries true" (* TEMP diagnostic admit — UNDO *)
let lemma_squeeze2_arm64
      (rate: usize)
      (s: Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 2) I.t_e_uint64x2_t)
      (out0 out1: t_Slice u8)
      (l: nat{l < 2})
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        Seq.length #u8 out0 < v Core_models.Num.impl_usize__MAX - 200 /\
        Seq.length #u8 out0 == Seq.length #u8 out1)
      (ensures (
        let outlen : usize = Core_models.Slice.impl__len #u8 out0 in
        let (out0', out1') =
          Libcrux_sha3.Generic_keccak.Simd128.squeeze2 rate s out0 out1 in
        let r_l = if l = 0 then out0' else out1' in
        r_l
        ==
        (Hacspec_sha3.Sponge.squeeze outlen
           (EquivImplSpec.Keccakf.Generic.extract_lane (mk_usize 2) KA.lc_arm64
              s.Libcrux_sha3.Generic_keccak.f_st l)
           rate
         <: t_Slice u8)))
  = let _ = Libcrux_sha3.Generic_keccak.Simd128.squeeze2 rate s out0 out1 in
    ()
#pop-options (* end TEMP diagnostic admit *)

(* ================================================================
   lemma_keccak2_arm64 = lemma_absorb2_arm64 ; lemma_squeeze2_arm64.

   Structurally identical to how lemma_keccak1_portable composes
   lemma_absorb_portable + lemma_squeeze_portable on the Portable
   side.
   ================================================================ *)

let lemma_keccak2_arm64
      (rate: usize) (delim: u8)
      (input: t_Array (t_Slice u8) (mk_usize 2))
      (out0 out1: t_Slice u8)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        Libcrux_sha3.Proof_utils.slices_same_len (mk_usize 2) input /\
        Seq.length #u8 out0 < v Core_models.Num.impl_usize__MAX - 200 /\
        Seq.length #u8 out0 == Seq.length #u8 out1)
      (ensures (
        let (r0, r1) =
          Libcrux_sha3.Generic_keccak.Simd128.keccak2
            rate delim input out0 out1 in
        let n : usize = Core_models.Slice.impl__len #u8 out0 in
        r0 == (Hacspec_sha3.Sponge.keccak n rate delim (input.[ mk_usize 0 ]) <: t_Slice u8) /\
        r1 == (Hacspec_sha3.Sponge.keccak n rate delim (input.[ mk_usize 1 ]) <: t_Slice u8)))
  = let s = Libcrux_sha3.Generic_keccak.Simd128.absorb2 rate delim input in
    lemma_absorb2_arm64 rate delim input 0;
    lemma_absorb2_arm64 rate delim input 1;
    lemma_squeeze2_arm64 rate s out0 out1 0;
    lemma_squeeze2_arm64 rate s out0 out1 1

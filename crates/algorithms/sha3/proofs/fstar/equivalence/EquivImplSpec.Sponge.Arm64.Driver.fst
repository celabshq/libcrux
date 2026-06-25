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

   [lemma_squeeze2_arm64] is now PROVEN (no admit): it reads lane [l]
   off [Generic_keccak.Simd128.squeeze2]'s strong per-lane functional
   post via [match l].  Under hax-lib 0.3.7 the key is the facts filter
   `-Libcrux_sha3.Generic_keccak.Simd128.squeeze2_blocks` (plus -squeeze
   / -extract_lane): it stops F* unfolding squeeze2's loop body and forces
   use of squeeze2's opaque post atomically — 2.9 rlimit instead of a
   ground-explosion cascade.  The per-lane NEON bridges
   [arm64_sc_load_block]/[arm64_sc_load_last]/[arm64_sc_store_block] are
   real [let]s in [EquivImplSpec.Sponge.Arm64.fst], also NOT admitted.
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
#push-options "--z3rlimit 400 --using_facts_from '* -Hacspec_sha3.Sponge.squeeze -EquivImplSpec.Keccakf.Generic.extract_lane -Libcrux_sha3.Generic_keccak.Simd128.squeeze2_blocks'"
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
    (* squeeze2's post gives out0'/out1' == squeeze(extract_lane s 0/1); pick the
       concrete lane so the if-ladder collapses to the matching conjunct (the
       symbolic `if l=0` forced a case split that cascaded hint-free). *)
    (match l with
     | 0 -> ()
     | _ -> ())
#pop-options

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

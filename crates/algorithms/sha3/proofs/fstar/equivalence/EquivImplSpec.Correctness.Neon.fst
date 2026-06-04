module EquivImplSpec.Correctness.Neon

(* ================================================================
   MAIN THEOREMS — Neon / Arm64 backend (N=2, v_T=t_e_uint64x2_t).
   See proofs/README.md.

   One top-level correctness lemma per algorithm; each says the public
   Rust API result equals the Hacspec spec:
     lemma_sha224/256/384/512_arm64 : Libcrux_sha3.Neon.shaXXX digest data
                                      == Hacspec_sha3.Sha3.sha3_XXX_ data
     lemma_shake128/256_arm64       : ... == Hacspec_sha3.Sha3.shakeXXX n data
   Each is the lane-0 specialisation of the two-lane driver
   [D.lemma_keccak2_arm64] (in EquivImplSpec.Sponge.Arm64.Driver).

   STRUCTURAL DIFFERENCE FROM Correctness.Portable:

   The Arm64 top-level hashers in [Libcrux_sha3.Neon] dispatch
   through a two-lane driver [Libcrux_sha3.Generic_keccak.Simd128.keccak2],
   e.g.

       let sha256 digest data =
         let dummy = [|0u8; …; 0u8|] in
         keccak2 136 0x06 [data; data] digest dummy

   The input is duplicated into both SIMD lanes; lane 0's output fills
   [digest] and lane 1's output is discarded into [dummy]. So even
   though the underlying primitive runs in parallel at N=2, the spec
   equivalence we need is single-lane: [digest] == scalar SHA3-256 of
   [data].

   The driver-level lemmas ([lemma_absorb2_arm64], [lemma_squeeze2_arm64],
   [lemma_keccak2_arm64]) live in [EquivImplSpec.Sponge.Arm64.Driver]
   to break a dependency cycle with [Libcrux_sha3.Neon] (which now
   cites [lemma_keccak2_arm64] from its function bodies).

   This module hosts only the per-hasher top-level theorems.
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models

module D = EquivImplSpec.Sponge.Arm64.Driver

(* Re-export driver-level lemmas under their historical names so that
   downstream consumers (and existing F* call-sites) keep compiling. *)
let lemma_absorb2_arm64 = D.lemma_absorb2_arm64
let lemma_squeeze2_arm64 = D.lemma_squeeze2_arm64
let lemma_keccak2_arm64 = D.lemma_keccak2_arm64


(* ================================================================
   PER-HASHER TOP-LEVEL THEOREMS

   Each hasher duplicates [data] into both lanes, so lane 0 of
   [keccak2] gives the scalar hash of [data] — which is exactly
   [Hacspec_sha3.Sha3.sha*_ data].
   ================================================================ *)

let lemma_sha224_arm64 (digest data: t_Slice u8)
  : Lemma
      (requires Seq.length #u8 digest == 28)
      (ensures Libcrux_sha3.Neon.sha224 digest data
               == (Hacspec_sha3.Sha3.sha3_224_ data <: t_Slice u8))
  = let inputs : t_Array (t_Slice u8) (mk_usize 2) =
      let l : list (t_Slice u8) = [ data; data ] in
      FStar.Pervasives.assert_norm (List.Tot.length l == 2);
      Rust_primitives.Hax.array_of_list 2 l in
    let dummy : t_Array u8 (mk_usize 28) =
      Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 28) in
    D.lemma_keccak2_arm64 (mk_usize 144) (mk_u8 6) inputs digest (dummy <: t_Slice u8)

let lemma_sha256_arm64 (digest data: t_Slice u8)
  : Lemma
      (requires Seq.length #u8 digest == 32)
      (ensures Libcrux_sha3.Neon.sha256 digest data
               == (Hacspec_sha3.Sha3.sha3_256_ data <: t_Slice u8))
  = let inputs : t_Array (t_Slice u8) (mk_usize 2) =
      let l : list (t_Slice u8) = [ data; data ] in
      FStar.Pervasives.assert_norm (List.Tot.length l == 2);
      Rust_primitives.Hax.array_of_list 2 l in
    let dummy : t_Array u8 (mk_usize 32) =
      Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 32) in
    D.lemma_keccak2_arm64 (mk_usize 136) (mk_u8 6) inputs digest (dummy <: t_Slice u8)

let lemma_sha384_arm64 (digest data: t_Slice u8)
  : Lemma
      (requires Seq.length #u8 digest == 48)
      (ensures Libcrux_sha3.Neon.sha384 digest data
               == (Hacspec_sha3.Sha3.sha3_384_ data <: t_Slice u8))
  = let inputs : t_Array (t_Slice u8) (mk_usize 2) =
      let l : list (t_Slice u8) = [ data; data ] in
      FStar.Pervasives.assert_norm (List.Tot.length l == 2);
      Rust_primitives.Hax.array_of_list 2 l in
    let dummy : t_Array u8 (mk_usize 48) =
      Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 48) in
    D.lemma_keccak2_arm64 (mk_usize 104) (mk_u8 6) inputs digest (dummy <: t_Slice u8)

let lemma_sha512_arm64 (digest data: t_Slice u8)
  : Lemma
      (requires Seq.length #u8 digest == 64)
      (ensures Libcrux_sha3.Neon.sha512 digest data
               == (Hacspec_sha3.Sha3.sha3_512_ data <: t_Slice u8))
  = let inputs : t_Array (t_Slice u8) (mk_usize 2) =
      let l : list (t_Slice u8) = [ data; data ] in
      FStar.Pervasives.assert_norm (List.Tot.length l == 2);
      Rust_primitives.Hax.array_of_list 2 l in
    let dummy : t_Array u8 (mk_usize 64) =
      Rust_primitives.Hax.repeat (mk_u8 0) (mk_usize 64) in
    D.lemma_keccak2_arm64 (mk_usize 72) (mk_u8 6) inputs digest (dummy <: t_Slice u8)

let lemma_shake128_arm64 (v_LEN: usize) (digest: t_Array u8 v_LEN) (data: t_Slice u8)
  : Lemma
      (requires v v_LEN < v Core_models.Num.impl_usize__MAX - 200)
      (ensures
        Libcrux_sha3.Neon.shake128 v_LEN digest data
        == (Hacspec_sha3.Sha3.shake128 v_LEN data <: t_Array u8 v_LEN))
  = let inputs : t_Array (t_Slice u8) (mk_usize 2) =
      let l : list (t_Slice u8) = [ data; data ] in
      FStar.Pervasives.assert_norm (List.Tot.length l == 2);
      Rust_primitives.Hax.array_of_list 2 l in
    let dummy : t_Array u8 v_LEN = Rust_primitives.Hax.repeat (mk_u8 0) v_LEN in
    D.lemma_keccak2_arm64 (mk_usize 168) (mk_u8 31) inputs
      (digest <: t_Slice u8) (dummy <: t_Slice u8)

let lemma_shake256_arm64 (v_LEN: usize) (digest: t_Array u8 v_LEN) (data: t_Slice u8)
  : Lemma
      (requires v v_LEN < v Core_models.Num.impl_usize__MAX - 200)
      (ensures
        Libcrux_sha3.Neon.shake256 v_LEN digest data
        == (Hacspec_sha3.Sha3.shake256 v_LEN data <: t_Array u8 v_LEN))
  = let inputs : t_Array (t_Slice u8) (mk_usize 2) =
      let l : list (t_Slice u8) = [ data; data ] in
      FStar.Pervasives.assert_norm (List.Tot.length l == 2);
      Rust_primitives.Hax.array_of_list 2 l in
    let dummy : t_Array u8 v_LEN = Rust_primitives.Hax.repeat (mk_u8 0) v_LEN in
    D.lemma_keccak2_arm64 (mk_usize 136) (mk_u8 31) inputs
      (digest <: t_Slice u8) (dummy <: t_Slice u8)

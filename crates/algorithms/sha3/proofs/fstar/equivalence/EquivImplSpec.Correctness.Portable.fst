module EquivImplSpec.Correctness.Portable

(* ================================================================
   MAIN THEOREMS — Portable backend (N=1, v_T=u64).  See proofs/README.md.

   One top-level correctness lemma per algorithm; each says the public
   Rust API result equals the Hacspec spec:
     lemma_sha224_portable  : Libcrux_sha3.Portable.sha224 digest data
                              == Hacspec_sha3.Sha3.sha3_224_ data
     lemma_sha256/384/512_portable  : ... == Hacspec_sha3.Sha3.sha3_{256,384,512}_ data
     lemma_shake128/256_portable    : ... == Hacspec_sha3.Sha3.shake{128,256} n data
   Each reduces, via [lemma_keccak1_portable] (= absorb · squeeze) with the
   per-algorithm rate/delimiter, to the spec keccak.

   Proven top-down from two layer lemmas about the Portable absorb
   and squeeze drivers:

     [lemma_absorb_portable]   : Generic_keccak.Portable.absorb ≡
                                 Hacspec_sha3.Sponge.absorb
     [lemma_squeeze_portable]  : Generic_keccak.Portable.squeeze ≡
                                 Hacspec_sha3.Sponge.squeeze

   Both are discharged directly by the Rust-side ensures on the
   respective functions in [crates/algorithms/sha3/src/generic_keccak/
   portable.rs], proved inline via loop invariants on
   [Hacspec_sha3.Sponge.absorb_blocks] / [...squeeze_blocks].

   The top-level theorems — [lemma_sha224_portable], [_sha256_],
   [_sha384_], [_sha512_], [_shake128_], [_shake256_] — then follow
   by unfolding and constant-matching.

   Matches the dispatch performed by [Libcrux_sha3.hash] and its
   public API ([sha224..512], [shake128/256]) to the spec hashers
   in [Hacspec_sha3.Sha3].
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models

module SC = EquivImplSpec.Sponge.Generic.Core
module Steps = EquivImplSpec.Sponge.Portable.Steps
module SqueezeAPI = EquivImplSpec.Sponge.Portable.SqueezeAPI
module KP = EquivImplSpec.Keccakf.Portable
module HS = Hacspec_sha3.Sponge
module SL = Hacspec_sha3.Sponge.Lemmas
module GK = Libcrux_sha3.Generic_keccak
module NF = Proof_Utils.NatFold

(* Bring Portable typeclass instances into scope so
   t_KeccakItem u64 1 / t_Absorb / t_Squeeze resolve. *)
let _ =
  let open Libcrux_sha3.Traits in
  let open Libcrux_sha3.Simd.Portable in
  ()

(* ================================================================
   LAYER LEMMAS

   Per-backend absorb / squeeze equivalence at N=1.  Both are
   trivial wrappers that invoke the Rust-side function to bring its
   ensures into context.  The ensures is proved inline in Rust via a
   loop invariant on [absorb_blocks] / [squeeze_blocks].
   ================================================================ *)

(** Portable absorb commutes with the scalar spec absorb: the state
    returned by [Generic_keccak.Portable.absorb] equals (at the
    [f_st] field) the 25-lane state returned by
    [Hacspec_sha3.Sponge.absorb].

    The equivalence is discharged directly by the Rust-side ensures
    on [Libcrux_sha3.Generic_keccak.Portable.absorb], proved inline
    via an [absorb_blocks]-based loop invariant (see
    [generic_keccak/portable.rs]).  We invoke the function here so
    its ensures is brought into context. *)
let lemma_absorb_portable
      (rate: usize) (delim: u8) (input: t_Slice u8)
  : Lemma
      (requires Libcrux_sha3.Proof_utils.valid_rate rate)
      (ensures
        (Libcrux_sha3.Generic_keccak.Portable.absorb rate delim input)
          .Libcrux_sha3.Generic_keccak.f_st
        ==
        Hacspec_sha3.Sponge.absorb rate delim input)
  = let _ = Libcrux_sha3.Generic_keccak.Portable.absorb rate delim input in
    ()


(** Portable squeeze commutes with the scalar spec squeeze: the
    byte-stream returned by [Generic_keccak.Portable.squeeze] agrees
    with [Hacspec_sha3.Sponge.squeeze] taken as a slice.

    [Generic_keccak.Portable.squeeze]'s strong postcondition factors
    its result as [portable_squeeze_composed rate ks output] — a
    direct mirror of [Hacspec_sha3.Sponge.squeeze]'s structure with
    [f_squeeze]/[keccakf1600] in place of [squeeze_state]/[keccak_f]
    and the external [output] buffer in place of the spec's zero
    initialisation. Per-primitive bridge lemmas
    ([lemma_squeeze_once_portable], [lemma_squeeze_block_portable],
    [lemma_squeeze_last_portable], [KP.lemma_keccakf1600_portable])
    close the primitive gap; the buffer-initialisation gap closes
    because each branch's squeeze steps cumulatively overwrite
    [0..output_len), so initial bytes never survive to the result. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 800 --split_queries always"
let lemma_squeeze_portable
      (rate: usize)
      (state: t_Array u64 (mk_usize 25))
      (output: t_Slice u8)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        Seq.length #u8 output < v Core_models.Num.impl_usize__MAX - 200)
      (ensures (
        let ks : Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
          { Libcrux_sha3.Generic_keccak.f_st = state } in
        let output_len : usize = Core_models.Slice.impl__len #u8 output in
        Libcrux_sha3.Generic_keccak.Portable.squeeze rate ks output
        ==
        (Hacspec_sha3.Sponge.squeeze output_len state rate <: t_Slice u8)))
  = (* The equivalence is discharged directly by the postcondition of
       [Libcrux_sha3.Generic_keccak.Portable.squeeze], whose ensures
       states [result == Hacspec_sha3.Sponge.squeeze ...].  We invoke
       it here so the ensures is brought into context. *)
    let ks : Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
      { Libcrux_sha3.Generic_keccak.f_st = state } in
    let _ = Libcrux_sha3.Generic_keccak.Portable.squeeze rate ks output in
    ()
#pop-options


(* ================================================================
   KECCAK1 EQUIVALENCE

   The single-lane portable driver [keccak1] = absorb · squeeze.
   Composition of the two layer lemmas.
   ================================================================ *)

let lemma_keccak1_portable
      (rate: usize) (delim: u8)
      (input output: t_Slice u8)
  : Lemma
      (requires
        Libcrux_sha3.Proof_utils.valid_rate rate /\
        Seq.length #u8 output < v Core_models.Num.impl_usize__MAX - 200)
      (ensures (
        let output_len : usize = Core_models.Slice.impl__len #u8 output in
        Libcrux_sha3.Generic_keccak.Portable.keccak1 rate delim input output
        ==
        (Hacspec_sha3.Sponge.keccak output_len rate delim input <: t_Slice u8)))
  = let s : Libcrux_sha3.Generic_keccak.t_KeccakState (mk_usize 1) u64 =
      Libcrux_sha3.Generic_keccak.Portable.absorb rate delim input in
    lemma_absorb_portable rate delim input;
    lemma_squeeze_portable rate s.Libcrux_sha3.Generic_keccak.f_st output


(* ================================================================
   TOP-LEVEL SHA-3 HASHERS
   ================================================================ *)

let lemma_sha224_portable (digest data: t_Slice u8)
  : Lemma
      (requires Seq.length #u8 digest == 28)
      (ensures
        Libcrux_sha3.Portable.sha224 digest data
        ==
        (Hacspec_sha3.Sha3.sha3_224_ data <: t_Slice u8))
  = lemma_keccak1_portable (mk_usize 144) (mk_u8 6) data digest

let lemma_sha256_portable (digest data: t_Slice u8)
  : Lemma
      (requires Seq.length #u8 digest == 32)
      (ensures
        Libcrux_sha3.Portable.sha256 digest data
        ==
        (Hacspec_sha3.Sha3.sha3_256_ data <: t_Slice u8))
  = lemma_keccak1_portable (mk_usize 136) (mk_u8 6) data digest

let lemma_sha384_portable (digest data: t_Slice u8)
  : Lemma
      (requires Seq.length #u8 digest == 48)
      (ensures
        Libcrux_sha3.Portable.sha384 digest data
        ==
        (Hacspec_sha3.Sha3.sha3_384_ data <: t_Slice u8))
  = lemma_keccak1_portable (mk_usize 104) (mk_u8 6) data digest

let lemma_sha512_portable (digest data: t_Slice u8)
  : Lemma
      (requires Seq.length #u8 digest == 64)
      (ensures
        Libcrux_sha3.Portable.sha512 digest data
        ==
        (Hacspec_sha3.Sha3.sha3_512_ data <: t_Slice u8))
  = lemma_keccak1_portable (mk_usize 72) (mk_u8 6) data digest


(* ================================================================
   TOP-LEVEL SHAKE XOFs
   ================================================================ *)

let lemma_shake128_portable (digest data: t_Slice u8)
  : Lemma
      (requires
        Seq.length #u8 digest < v Core_models.Num.impl_usize__MAX - 200)
      (ensures (
        let n : usize = Core_models.Slice.impl__len #u8 digest in
        Libcrux_sha3.Portable.shake128 digest data
        ==
        (Hacspec_sha3.Sha3.shake128 n data <: t_Slice u8)))
  = lemma_keccak1_portable (mk_usize 168) (mk_u8 31) data digest

let lemma_shake256_portable (digest data: t_Slice u8)
  : Lemma
      (requires
        Seq.length #u8 digest < v Core_models.Num.impl_usize__MAX - 200)
      (ensures (
        let n : usize = Core_models.Slice.impl__len #u8 digest in
        Libcrux_sha3.Portable.shake256 digest data
        ==
        (Hacspec_sha3.Sha3.shake256 n data <: t_Slice u8)))
  = lemma_keccak1_portable (mk_usize 136) (mk_u8 31) data digest

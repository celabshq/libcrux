module EquivImplSpec.Correctness.Avx2

(* ================================================================
   MAIN THEOREMS — AVX2 x4 backend (N=4, v_T=t_Vec256).
   See proofs/README.md.

   Top-level 4-way SHA-3 / SHAKE correctness: for each lane l < 4 the
   public Rust x4 API result equals the Hacspec spec applied to that
   lane's input, e.g.
     lemma_sha256_x4_avx2 / lemma_shake{128,256}_x4_avx2 :
       digests[l] == Hacspec_sha3.Sha3.sha3_256_ / shakeXXX (data[l])

   STRUCTURE (mirrors Correctness.Neon):

   The driver-level lemmas — [lemma_absorb4_avx2], [lemma_squeeze4_avx2]
   and their composition [lemma_keccak4_avx2] (the AVX2 counterpart of
   [lemma_keccak2_arm64]) — live in [EquivImplSpec.Sponge.Avx2.Driver].
   They are X4-free, so [Libcrux_sha3.Avx2.X4] can call
   [lemma_keccak4_avx2] to discharge its own function-level
   [Hacspec_sha3.Sponge.keccak] ensures without a dependency cycle,
   exactly as [Libcrux_sha3.Neon] calls [lemma_keccak2_arm64].

   Each per-hasher theorem below is then the thin step from that
   interface ensures ([== Hacspec_sha3.Sponge.keccak n 136 31 inputᵢ])
   to the named XOF spec ([Hacspec_sha3.Sha3.shake256]), mirroring
   [lemma_shake256_portable] / [lemma_shake256_arm64].

   All are proven (no admits); the only external trust is the AVX2
   intrinsic layer inherited via [lemma_keccakf1600_avx2].
   ================================================================ *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models


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
  = let _ =
      Libcrux_sha3.Avx2.X4.shake256 input0 input1 input2 input3 out0 out1 out2 out3
    in
    ()

module Hacspec_ml_kem.Commute.Prf_bridge

/// ============================================================================
/// PROVEN bridges (HF-pin).  The impl-side hash symbols (`Spec.Utils.v_PRF` /
/// `v_PRFxN`, computed by every backend) and the spec-side PRF
/// (`Hacspec_ml_kem.Parameters.Hash_functions.v_PRF`) both denote SHAKE256
/// (`Hacspec_sha3.Sponge.keccak _ 136 31`).  Now that HF.v_* are DEFINED as the
/// verified hacspec sha3 spec (extracted `[@@ "opaque_to_smt"]`), these are
/// PROVEN — no longer trust axioms: `lemma_prf_identification` by revealing the
/// HF body (→ `shake256` → `keccak`, meeting `Spec.Utils.v_PRF`'s keccak);
/// `lemma_prfxn_pointwise` is SU-internal (`v_PRFxN = map_array (keccak _ 136 31)`,
/// `lemma_createi_index` SMTPat).  Bounds tightened `pow2 32 → usize::MAX - 200`
/// to match `keccak`'s squeeze-overflow precond.
/// ============================================================================

open Core_models
open FStar.Mul

module SU = Spec.Utils
module HF = Hacspec_ml_kem.Parameters.Hash_functions
module HR = Hacspec_ml_kem.Commute.Hash_reveal

#set-options "--fuel 0 --ifuel 1 --z3rlimit 40"

/// SU.v_PRF and HF.v_PRF both denote SHAKE256 (keccak _ 136 31): `HR.lemma_v_PRF_eq`
/// gives `SU.v_PRF = keccak`, revealing HF.v_PRF's opaque_to_smt body gives
/// `HF.v_PRF = shake256 = keccak`.
let lemma_prf_identification (len: usize {v len < v Core_models.Num.impl_usize__MAX - 200})
      (input: t_Slice u8)
    : Lemma (ensures SU.v_PRF len input == HF.v_PRF len input)
  = HR.lemma_v_PRF_eq len input;
    reveal_opaque (`%HF.v_PRF) (HF.v_PRF len input)

/// The K-batched PRF equals the per-element PRF at every index — SU-internal
/// (`v_PRFxN = map_array (fun row -> keccak len 136 31 row)`; the `createi`
/// index property gives the pointwise equality against `v_PRF`).
let lemma_prfxn_pointwise
      (r: usize {v r == 2 \/ v r == 3 \/ v r == 4})
      (len: usize {v len < v Core_models.Num.impl_usize__MAX - 200})
      (input: t_Array (t_Array u8 (mk_usize 33)) r)
      (i: nat {i < v r})
    : Lemma
      (ensures
        (SU.v_PRFxN r len input).[ mk_usize i ] ==
        SU.v_PRF len ((input.[ mk_usize i ] <: t_Array u8 (mk_usize 33)) <: t_Slice u8))
  = HR.lemma_v_PRFxN_pointwise r len input i

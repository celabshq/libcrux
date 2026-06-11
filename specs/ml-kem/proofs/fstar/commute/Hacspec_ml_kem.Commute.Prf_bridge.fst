module Hacspec_ml_kem.Commute.Prf_bridge

/// ============================================================================
/// PROTOTYPE — PENDING USER SIGN-OFF (2026-06-10).  DO NOT COMMIT / WIRE until
/// the trust-model decision is approved.  See xof-hash-functions-audit.md
/// §"what ind_cpa needs", gaps (1) and (2).
///
/// Two TRUST AXIOMS bridging the impl-side abstract hash symbols
/// (`Spec.Utils.v_PRF` / `v_PRFxN`, computed by every backend) to the
/// spec-side abstract PRF (`Hacspec_ml_kem.Parameters.Hash_functions.v_PRF`).
/// Both are true of all three real backends (Portable / Avx2 / Neon), which
/// implement the same SHAKE256 PRF and loop/parallelize it for the xN variant.
/// This mirrors the user-approved admitted-FO-glue trust base used to close
/// ind_cca (Ind_cca_bridge.lemma_v_{G,H}_bridge).
/// ============================================================================

open Core_models
open FStar.Mul

module SU = Spec.Utils
module HF = Hacspec_ml_kem.Parameters.Hash_functions

#set-options "--fuel 0 --ifuel 0 --z3rlimit 20"

/// TRUST AXIOM 1 (identification).  The impl-side PRF symbol and the spec-side
/// PRF symbol denote the same function (SHAKE256 truncated to `len` bytes).
/// Audit gap (1).
let lemma_prf_identification (len: usize {v len < pow2 32}) (input: t_Slice u8)
    : Lemma (ensures SU.v_PRF len input == HF.v_PRF len input)
  = admit () (* TRUST: both = SHAKE256; cross-backend identification axiom *)

/// TRUST AXIOM 2 (pointwise).  The K-batched PRF equals the per-element PRF at
/// every index.  True of all backends (they loop/parallelize PRF over the K
/// inputs).  Stated index-wise to avoid `createi` entanglement at consumers.
/// Audit gap (2).
let lemma_prfxn_pointwise
      (r: usize {v r == 2 \/ v r == 3 \/ v r == 4})
      (len: usize {v len < pow2 32})
      (input: t_Array (t_Array u8 (mk_usize 33)) r)
      (i: nat {i < v r})
    : Lemma
      (ensures
        (SU.v_PRFxN r len input).[ mk_usize i ] ==
        SU.v_PRF len ((input.[ mk_usize i ] <: t_Array u8 (mk_usize 33)) <: t_Slice u8))
  = admit () (* TRUST: xN backend = K independent PRF calls; pointwise axiom *)

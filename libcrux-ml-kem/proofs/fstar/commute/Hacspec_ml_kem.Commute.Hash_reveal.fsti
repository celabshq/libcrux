module Hacspec_ml_kem.Commute.Hash_reveal

/// Reveal companion: exposes the (abstract in `Spec.Utils.fsti`) impl-side hash
/// symbols as their SHA-3 `keccak` definitions.  The equalities are proven in the
/// `.fst` via `friend Spec.Utils` (which sees the concrete `let v_* = keccak ...`
/// bodies).  Confining the `friend` to THIS small module keeps the heavy
/// `Spec.Utils.fst` cone out of every consumer (the commute bridges depend only
/// on this light interface + `Spec.Utils.fsti`).  Rate/delimiter params mirror
/// `Spec.Utils` (SHA3-512 = 72/6, SHA3-256 = 136/6, SHAKE256 = 136/31).

open Core_models
open FStar.Mul

module SU = Spec.Utils
module SP = Hacspec_sha3.Sponge

val lemma_v_G_eq (input: t_Slice u8)
  : Lemma (ensures SU.v_G input == SP.keccak (sz 64) (sz 72) (mk_u8 6) input)

val lemma_v_H_eq (input: t_Slice u8)
  : Lemma (ensures SU.v_H input == SP.keccak (sz 32) (sz 136) (mk_u8 6) input)

val lemma_v_PRF_eq (len: usize {v len < v Core_models.Num.impl_usize__MAX - 200})
      (input: t_Slice u8)
  : Lemma (ensures SU.v_PRF len input == SP.keccak len (sz 136) (mk_u8 31) input)

val lemma_v_J_eq (input: t_Slice u8)
  : Lemma (ensures SU.v_J input == SP.keccak (sz 32) (sz 136) (mk_u8 31) input)

/// `v_PRFxN` is `map_array (fun row -> keccak len 136 31 row)`, so its `i`-th
/// entry equals `v_PRF` of the `i`-th input (SU-internal pointwise identity).
val lemma_v_PRFxN_pointwise
      (r: usize {v r == 2 \/ v r == 3 \/ v r == 4})
      (len: usize {v len < v Core_models.Num.impl_usize__MAX - 200})
      (input: t_Array (t_Array u8 (sz 33)) r)
      (i: nat {i < v r})
  : Lemma (ensures
      (SU.v_PRFxN r len input).[ mk_usize i ] ==
      SU.v_PRF len ((input.[ mk_usize i ] <: t_Array u8 (sz 33)) <: t_Slice u8))

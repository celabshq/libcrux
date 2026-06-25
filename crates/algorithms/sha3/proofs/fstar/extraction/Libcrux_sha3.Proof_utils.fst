module Libcrux_sha3.Proof_utils
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

/// Checks if all slices in an array have the same length.
let slices_same_len (v_N: usize) (slices: t_Array (t_Slice u8) v_N) : Hax_lib.Prop.t_Prop =
  forall (i: usize).
    b2t (i <. v_N <: bool) ==>
    b2t
    ((Core_models.Slice.impl__len #u8 (slices.[ mk_usize 0 ] <: t_Slice u8) <: usize) =.
      (Core_models.Slice.impl__len #u8 (slices.[ i ] <: t_Slice u8) <: usize)
      <:
      bool)

[@@ "opaque_to_smt"]

/// `modifies_range a fa lo hi`: `fa` has the same length as `a` and is
/// equal to `a` at every index outside the half-open range `[lo, hi)`.
/// Opaque so that composition proofs manipulate it via the frame
/// lemmas (`lemma_modifies_range_union`) without unfolding the byte
/// content — the central trick for the SHA-3 store-block composition.
let modifies_range (a fa: t_Slice u8) (lo hi: usize) : Hax_lib.Prop.t_Prop =
  (forall (k: usize).
      b2t
      ((k <. (Core_models.Slice.impl__len #u8 a <: usize) <: bool) &&
        (k <. (Core_models.Slice.impl__len #u8 fa <: usize) <: bool) &&
        ((k <. lo <: bool) || (k >=. hi <: bool))) ==>
      b2t ((a.[ k ] <: u8) =. (fa.[ k ] <: u8) <: bool)) /\
  b2t
  ((Core_models.Slice.impl__len #u8 a <: usize) =. (Core_models.Slice.impl__len #u8 fa <: usize)
    <:
    bool)

let lemma_modifies_range_union (a b c: t_Slice u8) (lo mid hi: usize)
  : Lemma
    (requires
      modifies_range a b lo mid /\ modifies_range b c mid hi /\
      v lo <= v mid /\ v mid <= v hi)
    (ensures modifies_range a c lo hi)
  = reveal_opaque (`%modifies_range) modifies_range

let lemma_modifies_range_refl (a: t_Slice u8) (lo hi: usize)
  : Lemma (ensures modifies_range a a lo hi)
  = reveal_opaque (`%modifies_range) modifies_range

let lemma_modifies_range_intro (a b: t_Slice u8) (lo hi: usize)
  : Lemma
    (requires
      Seq.length a == Seq.length b /\
      (forall (k: nat). (k < Seq.length a /\ (k < v lo \/ k >= v hi)) ==>
        Seq.index b k == Seq.index a k))
    (ensures modifies_range a b lo hi)
  = reveal_opaque (`%modifies_range) modifies_range

let valid_rate (rate: usize) : bool =
  rate >. mk_usize 32 && rate <. mk_usize 200 && (rate %! mk_usize 8 <: usize) =. mk_usize 0 &&
  ((rate %! mk_usize 32 <: usize) =. mk_usize 8 || (rate %! mk_usize 32 <: usize) =. mk_usize 16)

/// XOF state invariant: validates that buffer length and rate are valid.
let keccak_xof_state_inv (rate buf_len: usize) : bool = valid_rate rate && buf_len <=. rate

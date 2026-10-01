module Proof_Utils.Lemmas

(** Library-level lemmas that bridge to upstream hax-lib /
    core-models proofs.  The underlying lemmas live in hax-lib's
    [Rust_primitives.Integers] and companion
    [Rust_primitives.Hax.Monomorphized_update_at_Lemmas]; this file
    wraps those upstream lemmas. *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 50"

open FStar.Mul
open Core_models
open Rust_primitives.Integers

(** Bitwise AND commutativity. *)
let logand_commutative (#t: inttype) (a b: int_t t)
  : Lemma ((a &. b) == (b &. a))
  = Rust_primitives.Integers.logand_commutative a b

(** [rotate_left(x, 0) == x]: [Core_models.Num.impl_u64__rotate_left] is
    [@@ "opaque_to_smt"] (atom by default; see hax#2235), so we [reveal_opaque]
    its shifts+xor body — with [n = 0] the [n mod 64 == 0] guard selects the
    identity branch. *)
let lemma_rotate_left_zero (x: u64)
  : Lemma (Core_models.Num.impl_u64__rotate_left x (mk_u32 0) == x)
  = reveal_opaque (`%Core_models.Num.impl_u64__rotate_left)
                  Core_models.Num.impl_u64__rotate_left

(* Update at Range Indexing Property *)
(* This is a more useful spec than the one in Monomorphized_update_at *)
let lemma_index_update_at_range #t (s:t_Slice t) (i:Core_models.Ops.Range.t_Range usize) (x:t_Slice t):
  Lemma
  (requires
    v i.f_start >= 0 /\ v i.f_start <= Seq.length s /\
    v i.f_end <= Seq.length s /\
    Seq.length x == v i.f_end - v i.f_start)
  (ensures (
  let open Core_models.Ops.Range in
  let out = Rust_primitives.Hax.Monomorphized_update_at.update_at_range s i x in
  Seq.length out == Seq.length s /\
 (forall (j:nat). if j < v i.f_start then
    Seq.index out j == Seq.index s j
  else if j >= v i.f_start && j < v i.f_end then
    Seq.index out j == Seq.index x (j - v i.f_start)
  else if j >= v i.f_end && j < Seq.length out then
    Seq.index out j == Seq.index s j
  else True))) =
  Rust_primitives.Hax.Monomorphized_update_at_Lemmas.lemma_index_update_at_range s i x

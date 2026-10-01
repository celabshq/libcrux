(*
 * Bitvec.U64Rotate — bridge lemma between u64 rotate-left and the
 * shift-and-XOR decomposition used by ARM's manual `_vxarq_u64`
 * fallback.
 *
 *   rotate_left x sh == (x <<! sh) ^. (x >>! (64 - sh))     (0 < sh < 64)
 *
 * This is a standard bitvec identity (see Hacker's Delight §2-15 or any
 * SMT-LIB rotate-left semantics).  We state it once with an `SMTPat` so
 * downstream callers — `_vxarq_u64`, `_vrax1q_u64`, etc. — discharge
 * their post-condition without further plumbing.
 *
 * The lemma is assumed; the trust footprint is one bit-width-specific
 * identity.
 *)
module Bitvec.U64Rotate

open Core_models

let in_range64 (sh: u32) : prop =
  Rust_primitives.Integers.v sh > 0 /\ Rust_primitives.Integers.v sh < 64

(* Bridge lemma — see module header. *)
assume
val lemma_u64_rotate_left_decomp (x: u64) (sh: u32)
  : Lemma
    (requires in_range64 sh)
    (ensures
       Core_models.Num.impl_u64__rotate_left x sh
       == (x <<! sh) ^. (x >>! (mk_u32 64 -! sh)))
    [SMTPat (Core_models.Num.impl_u64__rotate_left x sh)]

(* TODO: discharge `lemma_u64_rotate_left_decomp` from
   FStar.UInt / FStar.BV.  Sketch:
     - Decompose x = (x / 2^(64-sh)) * 2^(64-sh) + (x mod 2^(64-sh))
     - Show (x <<! sh) lane == (x mod 2^(64-sh)) * 2^sh   (UInt.shift_left_value_lemma)
     - Show (x >>! (64-sh)) lane == x / 2^(64-sh)         (UInt.shift_right_value_lemma)
     - Disjoint-bit XOR collapses to + (UInt.logor_disjoint + logxor==logor on disjoint)
     - Recombine as rotate-left definition (an assume in Core_models.Num;
       needs a parallel definition lemma there).
*)

module Proof_Utils.FoldRange

(** One-step unfolding of [Rust_primitives.Hax.Folds.fold_range]: peel off
    the first iteration. Layout-independent utility lemma, extracted here
    so it can be shared between [EquivImplSpec.Keccakf.Generic] and the
    [EquivImplSpec.Keccakf] top-level module without creating a dependency
    cycle. *)

#set-options "--fuel 1 --ifuel 1 --z3rlimit 50"

open FStar.Mul
open Core_models
open Rust_primitives.Integers

let lemma_fold_range_step
      (#acc_t: Type0)
      (start end_: usize)
      (inv: acc_t -> (i:usize{Rust_primitives.Hax.Folds.fold_range_wf_index start end_ false (v i)}) -> Type0)
      (init: acc_t {~(Rust_primitives.Hax.Folds.range_empty start end_) ==> inv init start})
      (f: (acc:acc_t -> i:usize {v i <= v end_ /\ Rust_primitives.Hax.Folds.fold_range_wf_index start end_ true (v i) /\ inv acc i}
                     -> acc':acc_t {(inv acc' (mk_int (v i + 1)))}))
  : Lemma
      (requires v start < v end_)
      (ensures Rust_primitives.Hax.Folds.fold_range start end_ inv init f ==
               Rust_primitives.Hax.Folds.fold_range (start +! mk_usize 1) end_ inv (f init start) f)
  = ()

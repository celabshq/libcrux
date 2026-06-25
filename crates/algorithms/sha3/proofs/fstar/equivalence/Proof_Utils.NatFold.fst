module Proof_Utils.NatFold

(** Utility module for bridging refined [Rust_primitives.Hax.Folds.fold_range]
    calls (as produced by hax extraction) to nat-indexed folds, which SMT
    can reason about without closure-equality friction.

    ----------------------------------------------------------------
    Background
    ----------------------------------------------------------------

    Hax extracts Rust `for i in start..end { body }` loops as refined
    [Rust_primitives.Hax.Folds.fold_range] calls, typically as inline
    lambdas whose argument type carries a [fold_range_wf_index start end_]
    refinement. Two such folds can be propositionally equal yet fail to
    unify under SMT because F*'s encoding of closures does not identify
    α/β/η-equivalent inline lambdas whose refinement types differ only
    syntactically.

    Earlier attempts to discharge these equalities via [fold_range_ext]
    (see [Libcrux_sha3.Proof_utils.Folds]) ran into the same wall: the
    pointwise hypothesis of [fold_range_ext] is a ∀-quantified closure
    equality, which SMT cannot prove for hax-extracted lambdas.

    ----------------------------------------------------------------
    Solution
    ----------------------------------------------------------------

    Convert the refined [fold_range] to a *nat-indexed* fold by providing
    a plain-nat body [g] that agrees pointwise with the refined body [f].
    Crucially, the pointwise equality is supplied as a *Lemma argument*,
    not a ∀-quantified hypothesis — so F* can discharge it at each call
    site individually (usually with [()] when both bodies β-reduce to the
    same expression).

    Two nat-fold shapes are provided:

    - [fold_nat_range]: body type is [j:nat{j < end_}]. Simplest form;
      forces the body to be total on [0, end_).

    - [fold_range_nat]: fixed [start]/[end_], explicit iteration counter
      [i], body type is [j:nat{start <= j /\ j < end_}]. The body's
      refinement mirrors [fold_range_wf_index] directly, which is useful
      when the body is meaningfully partial over the window.

    Each shape has a corresponding bridge lemma:
    [lemma_fold_range_is_nat] and [lemma_fold_range_is_range_nat].

    ----------------------------------------------------------------
    Usage pattern
    ----------------------------------------------------------------

    1. Define a nat-indexed body [g] matching the extractor's inline lambda.
    2. Call the bridge lemma with inline lambdas for [inv]/[f] matching
       the fold_range target's syntactic shape.
    3. Discharge the [pointwise] Lemma with [(fun acc i -> ())] when both
       bodies β-reduce to the same expression.
    4. Either:
         - Unfold [fold_nat_range] / [fold_range_nat] via [T.norm] +
           [T.smt ()] for concrete small bounds; or
         - Relate the nat-fold to a recursive helper via structural
           induction.

    See [Test_Norm_Plain] and [Test_Keccakf_NatFold] for worked examples.
    See also [EquivImplSpec.Keccakf.Generic.lemma_keccakf1600_is_rounds] for
    a real-world application. *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models
open Rust_primitives.Integers

(** Nat-indexed fold. Body is total over [[0, end_)]. *)
let rec fold_nat_range
      (#acc_t: Type0)
      (start end_: nat)
      (init: acc_t)
      (f: acc_t -> (i: nat{i < end_}) -> acc_t)
  : Tot acc_t (decreases end_ - start)
  = if start < end_
    then fold_nat_range (start + 1) end_ (f init start) f
    else init

(** Nat-indexed fold with fixed start/end and an explicit iteration counter.
    The body is defined on [[start, end_)], mirroring
    [Rust_primitives.Hax.Folds.fold_range_wf_index start end_]. *)
let rec fold_range_nat
      (#acc_t: Type0)
      (start end_: nat)
      (i: nat{start <= i /\ i <= end_})
      (acc: acc_t)
      (f: acc_t -> (j: nat{start <= j /\ j < end_}) -> acc_t)
  : Tot acc_t (decreases end_ - i)
  = if i < end_
    then fold_range_nat start end_ (i + 1) (f acc i) f
    else acc

(** Bridge: refined [fold_range start end_ inv init f] equals
    [fold_nat_range (v start) (v end_) init g] whenever [f] and [g] agree
    pointwise on the iteration domain.

    The [pointwise] argument is a Lemma — supplied at the call site — not a
    ∀-hypothesis, so the closure-equality problem that blocks
    [fold_range_ext] does not arise. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let rec lemma_fold_range_is_nat
      (#acc_t: Type0) (#u: uinttype)
      (start end_: int_t u)
      (inv: acc_t -> (i:int_t u{Rust_primitives.Hax.Folds.fold_range_wf_index start end_ false (v i)}) -> Type0)
      (init: acc_t {~(Rust_primitives.Hax.Folds.range_empty start end_) ==> inv init start})
      (f: (acc:acc_t -> i:int_t u {v i <= v end_ /\ Rust_primitives.Hax.Folds.fold_range_wf_index start end_ true (v i) /\ inv acc i}
                     -> acc':acc_t {inv acc' (mk_int (v i + 1))}))
      (g: acc_t -> (i: nat{i < v end_}) -> acc_t)
      (pointwise:
        (acc: acc_t)
        -> (i: int_t u {v i <= v end_ /\ Rust_primitives.Hax.Folds.fold_range_wf_index start end_ true (v i) /\ inv acc i})
        -> Lemma (f acc i == g acc (v i)))
  : Lemma (ensures Rust_primitives.Hax.Folds.fold_range start end_ inv init f ==
                   fold_nat_range (v start) (v end_) init g)
          (decreases v end_ - v start)
  = if v start < v end_
    then begin
      pointwise init start;
      lemma_fold_range_is_nat
        (start +! mk_int 1) end_ inv (f init start) f g
        (fun acc i -> pointwise acc i)
    end
    else ()
#pop-options

(** Bridge: refined [fold_range i end_ inv acc f] equals
    [fold_range_nat (v start) (v end_) (v i) acc g] whenever [f] and [g]
    agree pointwise on [[start, end_)].

    Unlike [lemma_fold_range_is_nat], [start] and [end_] stay fixed across
    recursion — only [i] advances. This keeps the body's refinement
    [fold_range_wf_index start end_] constant, which can simplify the
    pointwise proof when the body uses [j - start], [j - i], modular
    arithmetic, etc. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let rec lemma_fold_range_is_range_nat
      (#acc_t: Type0) (#u: uinttype)
      (start end_: int_t u)
      (i: int_t u {v start <= v i /\ v i <= v end_})
      (inv: acc_t -> (j:int_t u{Rust_primitives.Hax.Folds.fold_range_wf_index i end_ false (v j)}) -> Type0)
      (acc: acc_t {~(Rust_primitives.Hax.Folds.range_empty i end_) ==> inv acc i})
      (f: (a:acc_t -> j:int_t u {v j <= v end_ /\ Rust_primitives.Hax.Folds.fold_range_wf_index i end_ true (v j) /\ inv a j}
                    -> a':acc_t {inv a' (mk_int (v j + 1))}))
      (g: acc_t -> (j: nat{v start <= j /\ j < v end_}) -> acc_t)
      (pointwise:
        (a: acc_t)
        -> (j: int_t u {v j <= v end_ /\ Rust_primitives.Hax.Folds.fold_range_wf_index i end_ true (v j) /\ inv a j})
        -> Lemma (f a j == g a (v j)))
  : Lemma (ensures Rust_primitives.Hax.Folds.fold_range i end_ inv acc f ==
                   fold_range_nat (v start) (v end_) (v i) acc g)
          (decreases v end_ - v i)
  = if v i < v end_
    then begin
      pointwise acc i;
      lemma_fold_range_is_range_nat
        start end_ (i +! mk_int 1) inv (f acc i) f g
        (fun a j -> pointwise a j)
    end
    else ()
#pop-options

(** Direct unrolling of a [fold_range 0 5]: the fold equals five sequential
    applications of the body.

    Useful when the body is a refined inline lambda that would be painful
    to bridge via [lemma_fold_range_is_nat] / [lemma_fold_range_is_range_nat].
    Instead, specialize [f] to the extracted lambda: F* β-reduces each
    application, yielding a direct expression the SMT can chain through
    with a small fuel budget. *)
#push-options "--fuel 6 --ifuel 2 --z3rlimit 200"
let lemma_fold_range_unroll_5
      (#acc_t: Type0) (#u: uinttype)
      (inv: acc_t -> (i:int_t u{Rust_primitives.Hax.Folds.fold_range_wf_index (mk_int #u 0) (mk_int #u 5) false (v i)}) -> Type0)
      (init: acc_t {inv init (mk_int #u 0)})
      (f: (acc:acc_t -> i:int_t u {v i <= 5 /\ Rust_primitives.Hax.Folds.fold_range_wf_index (mk_int #u 0) (mk_int #u 5) true (v i) /\ inv acc i}
                     -> acc':acc_t {inv acc' (mk_int #u (v i + 1))}))
  : Lemma (Rust_primitives.Hax.Folds.fold_range #acc_t #u (mk_int #u 0) (mk_int #u 5) inv init f ==
           f (f (f (f (f init (mk_int #u 0)) (mk_int #u 1)) (mk_int #u 2)) (mk_int #u 3)) (mk_int #u 4))
  = ()
#pop-options

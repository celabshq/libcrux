module EquivImplSpec.Keccakf.ChiFold

(** Chi-step fold-unfolding utilities for the generic keccak_f
    equivalence proof.

    The extracted [Libcrux_sha3.Generic_keccak.impl_2__chi] is a 5x5
    nested [fold_range] whose body writes [f_and_not_xor] into each
    cell. Direct SMT reasoning across both folds is too closure-heavy.

    This module establishes the per-position equality

      (impl_2__chi v_N #v_T ks).f_st.[k] == chi_inner_val ks (k/5) (k%5)

    Under the FIPS-native layout [get_ij(arr, i, j) = arr[5*i + j]],
    flat index [k] corresponds to [(i, j) = (k/5, k%5)] (impl-side
    [(i, j)] is FIPS [(y, x)]).

    The equality is proved via a loop-invariant argument on a named
    [chi_unrolled] form, bridged to [impl_2__chi] by a fuel-6
    [fold_range] unroll.

    The single export [lemma_chi_val_i] is consumed by
    [EquivImplSpec.Keccakf.Generic.lemma_chi_extract_lane] together with a
    lane-correctness wrapper around [chi_inner_val]. *)

#set-options "--fuel 0 --ifuel 1 --z3rlimit 100"

open FStar.Mul
open Core_models
open Rust_primitives.Integers
open Libcrux_sha3.Generic_keccak

(* ================================================================
   Per-position chi value: applied with [old = ks] this is what
   chi writes at position (i, j).
   ================================================================ *)

let chi_inner_val
  (#v_N: usize) (#v_T: Type0)
  {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
  (old: t_KeccakState v_N v_T)
  (i: usize{v i < 5}) (j: usize{v j < 5}) =
    (Libcrux_sha3.Traits.f_and_not_xor #v_T #v_N
       (old.[ i, j <: (usize & usize) ] <: v_T)
       (old.[ i, ((j +! mk_usize 2) %! mk_usize 5) <: (usize & usize) ] <: v_T)
       (old.[ i, ((j +! mk_usize 1) %! mk_usize 5) <: (usize & usize) ] <: v_T))

(* ================================================================
   Inner-loop invariant + step lemma.
   ================================================================ *)

let chi_inner_inv
      (#v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (old s: t_KeccakState v_N v_T)
      (i: usize{v i < 5}) (j: usize{v j <= 5}) =
    (forall (ii:usize) (jj:usize).
      (v ii < 5 /\ v jj < 5 /\
       (v ii < v i \/ (v ii == v i /\ v jj < v j))) ==>
       s.[ ii, jj ] == chi_inner_val old ii jj) /\
    (forall (ii:usize) (jj:usize).
      (v ii < 5 /\ v jj < 5 /\
       (v ii > v i \/ (v ii == v i /\ v jj >= v j))) ==>
       s.[ ii,jj ] == old.[ ii,jj ])

#push-options "--z3rlimit 200 --split_queries always"
let chi_inner_body
      (#v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (old s: t_KeccakState v_N v_T)
      (i: usize{v i < 5}) (j: usize{v j < 5})
  : Pure (t_KeccakState v_N v_T)
    (requires (chi_inner_inv old s i j))
    (ensures fun r -> (chi_inner_inv old r i (j +! sz 1))) =
  let s' =
  impl_2__set v_N #v_T s i j
    (Libcrux_sha3.Traits.f_and_not_xor #v_T #v_N
       (s.[ i, j <: (usize & usize) ] <: v_T)
       (old.[ i, ((j +! mk_usize 2) %! mk_usize 5) <: (usize & usize) ] <: v_T)
       (old.[ i, ((j +! mk_usize 1) %! mk_usize 5) <: (usize & usize) ] <: v_T))
  in
  assert (s'.[ i, j <: usize & usize ] == chi_inner_val old i j);
  assert (forall (ii:usize) (jj:usize).
      (v ii < 5 /\ v jj < 5 /\
       (v ii < v i \/ (v ii == v i /\ v jj < v j + 1))) ==>
       s'.[ ii, jj ] == chi_inner_val old ii jj);
  assert (forall (ii:usize) (jj:usize).
      (v ii < 5 /\ v jj < 5 /\
       (v ii > v i \/ (v ii == v i /\ v jj >= v j + 1))) ==>
       s.[ ii,jj ] == old.[ ii,jj ]);
  assert(chi_inner_inv old s' i (j +! sz 1));
  s'
#pop-options

(* ================================================================
   Outer-loop invariant + body.
   ================================================================ *)

let chi_outer_inv
      (#v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (old s: t_KeccakState v_N v_T)
      (i: usize{v i <= 5}) =
    (forall (ii:usize) (jj:usize).
      (v ii < 5 /\ v jj < 5 /\
       v ii < v i) ==>
       s.[ ii, jj ] == chi_inner_val old ii jj) /\
    (forall (ii:usize) (jj:usize).
      (v ii < 5 /\ v jj < 5 /\
       v ii >= v i) ==>
       s.[ ii,jj ] == old.[ ii,jj ])

let chi_outer_body
      (#v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (old s: t_KeccakState v_N v_T)
      (i: usize{v i < 5})
  : Pure (t_KeccakState v_N v_T)
      (requires chi_outer_inv old s i)
      (ensures fun r -> chi_outer_inv old r (i +! sz 1)) =
  let s = chi_inner_body #v_N old s i (mk_usize 0) in
  let s = chi_inner_body #v_N old s i (mk_usize 1) in
  let s = chi_inner_body #v_N old s i (mk_usize 2) in
  let s = chi_inner_body #v_N old s i (mk_usize 3) in
  chi_inner_body #v_N old s i (mk_usize 4)

(* ================================================================
   Fully unrolled chi with the outer postcondition.
   ================================================================ *)

#push-options "--z3rlimit 400"
let chi_unrolled
      (#v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: t_KeccakState v_N v_T)
  : Pure (t_KeccakState v_N v_T)
     (requires True)
     (ensures fun r -> chi_outer_inv ks r (sz 5))
  =
  let old = ks in
  let s = chi_outer_body #v_N old ks (mk_usize 0) in
  let s = chi_outer_body #v_N old s  (mk_usize 1) in
  let s = chi_outer_body #v_N old s  (mk_usize 2) in
  let s = chi_outer_body #v_N old s  (mk_usize 3) in
  let s = chi_outer_body #v_N old s  (mk_usize 4) in
  s
#pop-options

(* ================================================================
   Fold-range bridge: [impl_2__chi == chi_unrolled].
   Fuel 6 lets Z3 unfold both nested 0..5 fold_ranges step by step.
   ================================================================ *)

#push-options "--fuel 6 --ifuel 0 --z3rlimit 800"
let lemma_chi_outer_unfolds_generic
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: t_KeccakState v_N v_T)
  : Lemma (impl_2__chi v_N #v_T ks == chi_unrolled #v_N #v_T ks)
  = ()
#pop-options

(* ================================================================
   Top-level export: per-position equality at any flat index.
   ================================================================ *)

#push-options "--z3rlimit 400 --split_queries always"
let lemma_chi_val_i
      (v_N: usize) (#v_T: Type0)
      {| inst: Libcrux_sha3.Traits.t_KeccakItem v_T v_N |}
      (ks: t_KeccakState v_N v_T)
      (k: usize{v k < 25})
  : Lemma ((impl_2__chi v_N #v_T ks).f_st.[ k <: usize ] ==
           chi_inner_val ks (k /! sz 5) (k %! sz 5))
  = let i = k /! sz 5 in
    let j = k %! sz 5 in
    assert (v i = v k / 5);
    assert (v j = v k % 5);
    assert (v i < 5);
    assert (v j < 5);
    assert (v k == 5 * v i + v j);
    assert (k == sz 5 *! i +! j);
    let s = chi_unrolled #v_N #v_T ks in
    lemma_chi_outer_unfolds_generic v_N ks;
    assert (chi_outer_inv ks s (sz 5));
    assert (s.[ i,j ] == chi_inner_val ks i j);
    assert (s.[ i,j ] == Libcrux_sha3.Traits.get_ij v_N s.f_st i j)
#pop-options

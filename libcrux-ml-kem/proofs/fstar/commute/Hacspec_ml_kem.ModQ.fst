module Hacspec_ml_kem.ModQ
#set-options "--fuel 0 --ifuel 1 --z3rlimit 20"
open FStar.Mul

(* Opaque mod-q abstractions to keep raw `% 3329` from leaking above the
   trait boundary.  Above-trait code consumes `mod_q_eq` (an opaque
   relation) instead of raw integer modular equations.  Bridge lemmas
   below allow controlled unfolding when needed. *)

[@@ "opaque_to_smt"]
let mod_q (x: int) : (n: nat { n < 3329 })
  = let r = x % 3329 in
    FStar.Math.Lemmas.modulo_range_lemma x 3329;
    r

[@@ "opaque_to_smt"]
let mod_q_eq (x y: int) : prop = mod_q x == mod_q y

(* Reveal helpers — consumers that need the raw mod-q form invoke these. *)

let lemma_mod_q_unfold (x: int) :
    Lemma (mod_q x == x % 3329)
  = reveal_opaque (`%mod_q) (mod_q x)

let lemma_mod_q_eq_unfold (x y: int) :
    Lemma (mod_q_eq x y <==> (x % 3329 == y % 3329))
  = reveal_opaque (`%mod_q_eq) (mod_q_eq x y);
    reveal_opaque (`%mod_q) (mod_q x);
    reveal_opaque (`%mod_q) (mod_q y)

(* Intro helpers — impl-side body proofs produce `x % 3329 == y % 3329`
   directly; this folds them into the opaque relation. *)

let lemma_mod_q_eq_intro (x y: int) :
    Lemma (requires x % 3329 == y % 3329) (ensures mod_q_eq x y)
  = reveal_opaque (`%mod_q_eq) (mod_q_eq x y);
    reveal_opaque (`%mod_q) (mod_q x);
    reveal_opaque (`%mod_q) (mod_q y)

(* Algebra of the relation — reflexivity, symmetry, transitivity.  Useful
   for chaining without unfolding. *)

let lemma_mod_q_eq_refl (x: int) :
    Lemma (mod_q_eq x x)
  = reveal_opaque (`%mod_q_eq) (mod_q_eq x x)

let lemma_mod_q_eq_sym (x y: int) :
    Lemma (requires mod_q_eq x y) (ensures mod_q_eq y x)
  = reveal_opaque (`%mod_q_eq) (mod_q_eq x y);
    reveal_opaque (`%mod_q_eq) (mod_q_eq y x)

let lemma_mod_q_eq_trans (x y z: int) :
    Lemma (requires mod_q_eq x y /\ mod_q_eq y z) (ensures mod_q_eq x z)
  = reveal_opaque (`%mod_q_eq) (mod_q_eq x y);
    reveal_opaque (`%mod_q_eq) (mod_q_eq y z);
    reveal_opaque (`%mod_q_eq) (mod_q_eq x z)

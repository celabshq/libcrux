module Hacspec_ml_kem.Commute.Chunk
#set-options "--fuel 0 --ifuel 1 --z3rlimit 200"
open FStar.Mul
open Core_models
open Libcrux_ml_kem.Vector.Traits.Spec
open Hacspec_ml_kem.ModQ

(* Layer 0 — field-element scalar commute lemmas.
   Each consumes an existing impl post of the form `v r % 3329 == <eqn>`
   and produces a `t_FieldElement`-level equation via `i16_to_spec_fe` or
   `mont_i16_to_spec_fe`.  These are the primitive bridges from the impl's
   raw mod-q arithmetic to the hacspec FE algebra. *)

module P  = Hacspec_ml_kem.Parameters
module L  = FStar.Math.Lemmas
module T  = Libcrux_ml_kem.Vector.Traits
module TS = Libcrux_ml_kem.Vector.Traits.Spec
module N  = Hacspec_ml_kem.Ntt
module IN = Hacspec_ml_kem.Invert_ntt
module CP = Hacspec_ml_kem.Compress
module MQ = Hacspec_ml_kem.ModQ

(* Basic sanity: the two lifts produce valid FEs by construction. *)
let lemma_i16_to_spec_fe_bounded (x: i16) :
    Lemma ((i16_to_spec_fe x).P.f_val <. P.v_FIELD_MODULUS)
  = ()

let lemma_mont_i16_to_spec_fe_bounded (x: i16) :
    Lemma ((mont_i16_to_spec_fe x).P.f_val <. P.v_FIELD_MODULUS)
  = ()

(* Addition commutes with `i16_to_spec_fe` whenever the impl post states
   the algebraic identity at the int level.  Since `add` is linear, the
   same lemma holds when both sides are lifted via `mont_i16_to_spec_fe`. *)
let lemma_add_fe_commute_plain (a b r: i16) :
    Lemma (requires v r == v a + v b)
          (ensures  P.impl_FieldElement__add (i16_to_spec_fe a) (i16_to_spec_fe b)
                    == i16_to_spec_fe r)
  = ()

let lemma_add_fe_commute_mont (a b r: i16) :
    Lemma (requires v r == v a + v b)
          (ensures  P.impl_FieldElement__add
                        (mont_i16_to_spec_fe a) (mont_i16_to_spec_fe b)
                    == mont_i16_to_spec_fe r)
  = ()

let lemma_sub_fe_commute_plain (a b r: i16) :
    Lemma (requires v r == v a - v b)
          (ensures  P.impl_FieldElement__sub (i16_to_spec_fe a) (i16_to_spec_fe b)
                    == i16_to_spec_fe r)
  = ()

let lemma_sub_fe_commute_mont (a b r: i16) :
    Lemma (requires v r == v a - v b)
          (ensures  P.impl_FieldElement__sub
                        (mont_i16_to_spec_fe a) (mont_i16_to_spec_fe b)
                    == mont_i16_to_spec_fe r)
  = ()

(* Barrett reduction preserves value mod q, i.e. is identity under the
   plain lift. *)
let lemma_barrett_fe_commute (a r: i16) :
    Lemma (requires v r % 3329 == v a % 3329)
          (ensures  i16_to_spec_fe r == i16_to_spec_fe a)
  = ()

(* Zeta cancellation: the impl stores zetas pre-multiplied by R, so
   `mont_i16_to_spec_fe zeta_mont` recovers the plain abstract zeta.
   Its post exactly matches the canonical nonneg form of both lifts. *)
let lemma_mont_zeta_cancel (zeta_mont zeta_plain: i16) :
    Lemma (requires (v zeta_mont * 169) % 3329 == v zeta_plain % 3329)
          (ensures  mont_i16_to_spec_fe zeta_mont == i16_to_spec_fe zeta_plain)
  = ()

(* The `v .f_val` equalities for `i16_to_spec_fe` / `mont_i16_to_spec_fe`
   are now delivered by each lift's return refinement, so the only helper
   we still need is the one that exposes the f_val of `impl_FieldElement__mul`. *)
let lemma_impl_mul_v_val (x y: P.t_FieldElement) :
    Lemma (v (P.impl_FieldElement__mul x y).P.f_val
             == (v x.P.f_val * v y.P.f_val) % 3329)
  = ()

let lemma_impl_add_v_val (x y: P.t_FieldElement) :
    Lemma (v (P.impl_FieldElement__add x y).P.f_val
             == (v x.P.f_val + v y.P.f_val) % 3329)
  = ()

let lemma_impl_sub_v_val (x y: P.t_FieldElement) :
    Lemma (v (P.impl_FieldElement__sub x y).P.f_val
             == (v x.P.f_val - v y.P.f_val) % 3329)
  = ()

(* Scalar modular-arithmetic cores.  Proved at the `int` level so the
   wrapping lemmas only have to line up `v`-value equalities. *)

let lemma_mont_mul_mod_core (a b r: int) :
    Lemma (requires r % 3329 == (a * b * 169) % 3329)
          (ensures  (((a * 169) % 3329) * ((b * 169) % 3329)) % 3329
                    == (r * 169) % 3329)
  = let q : pos = 3329 in
    L.lemma_mod_mul_distr_l (a * 169) ((b * 169) % q) q;
    L.lemma_mod_mul_distr_r (a * 169) (b * 169) q;
    assert ((a * 169) * (b * 169) == a * b * 169 * 169);
    L.lemma_mod_mul_distr_l (a * b * 169) 169 q;
    L.lemma_mod_mul_distr_l r 169 q

let lemma_mont_plain_mul_mod_core (a b r: int) :
    Lemma (requires r % 3329 == (a * b * 169) % 3329)
          (ensures  (((a * 169) % 3329) * (b % 3329)) % 3329 == r % 3329)
  = let q : pos = 3329 in
    L.lemma_mod_mul_distr_l (a * 169) (b % q) q;
    L.lemma_mod_mul_distr_r (a * 169) b q;
    assert ((a * 169) * b == a * b * 169)

let lemma_mul_const_mod_core (a c r: int) :
    Lemma (requires r == a * c)
          (ensures  ((a % 3329) * (c % 3329)) % 3329 == r % 3329)
  = let q : pos = 3329 in
    L.lemma_mod_mul_distr_l a (c % q) q;
    L.lemma_mod_mul_distr_r a c q

(* Integer-level modular cores for the mod-aware add / sub lifts.
   Each shows that under the mont lift, residue equality at the i16 level
   transfers to f_val equality. *)

let lemma_add_mont_mod_core (a b r: int) :
    Lemma (requires r % 3329 == (a + b) % 3329)
          (ensures  (((a * 169) % 3329) + ((b * 169) % 3329)) % 3329
                    == (r * 169) % 3329)
  = let q : pos = 3329 in
    L.lemma_mod_add_distr ((a * 169) % q) (b * 169) q;
    L.lemma_mod_add_distr (b * 169) (a * 169) q;
    assert (a * 169 + b * 169 == (a + b) * 169);
    L.lemma_mod_mul_distr_l (a + b) 169 q;
    L.lemma_mod_mul_distr_l r 169 q

let lemma_sub_mont_mod_core (a b r: int) :
    Lemma (requires r % 3329 == (a - b) % 3329)
          (ensures  (((a * 169) % 3329) - ((b * 169) % 3329)) % 3329
                    == (r * 169) % 3329)
  = let q : pos = 3329 in
    L.lemma_mod_sub_distr ((a * 169) % q) (b * 169) q;
    L.lemma_mod_add_distr (- ((b * 169) % q)) (a * 169) q;
    assert (a * 169 - b * 169 == (a - b) * 169);
    L.lemma_mod_mul_distr_l (a - b) 169 q;
    L.lemma_mod_mul_distr_l r 169 q

(* Mod-aware addition: same hypothesis shape as
   `ntt_spec` / `inv_ntt_spec` give for the butterfly outputs. *)
let lemma_add_fe_commute_mont_mod (a b r: i16) :
    Lemma (requires v r % 3329 == (v a + v b) % 3329)
          (ensures  P.impl_FieldElement__add
                        (mont_i16_to_spec_fe a) (mont_i16_to_spec_fe b)
                    == mont_i16_to_spec_fe r)
  = lemma_impl_add_v_val (mont_i16_to_spec_fe a) (mont_i16_to_spec_fe b);
    lemma_add_mont_mod_core (v a) (v b) (v r)

let lemma_sub_fe_commute_mont_mod (a b r: i16) :
    Lemma (requires v r % 3329 == (v a - v b) % 3329)
          (ensures  P.impl_FieldElement__sub
                        (mont_i16_to_spec_fe a) (mont_i16_to_spec_fe b)
                    == mont_i16_to_spec_fe r)
  = lemma_impl_sub_v_val (mont_i16_to_spec_fe a) (mont_i16_to_spec_fe b);
    lemma_sub_mont_mod_core (v a) (v b) (v r)

(* Butterfly commute lemmas.  `Spec.Utils.ntt_spec` (and `inv_ntt_spec`)
   produce exactly one of these residue hypotheses per lane; the
   portable impl's `ntt_layer_1_step` wrapper folds them across 4
   butterfly pairs to discharge `forall4`-pointwise FE equalities. *)

let lemma_butterfly_mod_core (a b z r: int) :
    Lemma (requires r % 3329 == (a + b * z * 169) % 3329)
          (ensures  ((a * 169) % 3329
                     + (((z * 169) % 3329) * ((b * 169) % 3329)) % 3329) % 3329
                    == (r * 169) % 3329)
  = let q : pos = 3329 in
    L.lemma_mod_mul_distr_l (z * 169) ((b * 169) % q) q;
    L.lemma_mod_mul_distr_r (z * 169) (b * 169) q;
    assert ((z * 169) * (b * 169) == b * z * 169 * 169);
    L.lemma_mod_add_distr ((a * 169) % q) (b * z * 169 * 169) q;
    L.lemma_mod_add_distr (b * z * 169 * 169) (a * 169) q;
    assert (a * 169 + b * z * 169 * 169 == (a + b * z * 169) * 169);
    L.lemma_mod_mul_distr_l (a + b * z * 169) 169 q;
    L.lemma_mod_mul_distr_l r 169 q

let lemma_butterfly_fe_commute_plus (vec_i vec_j zeta result_i: i16) :
  Lemma (requires v result_i % 3329
                  == (v vec_i + v vec_j * v zeta * 169) % 3329)
        (ensures  mont_i16_to_spec_fe result_i ==
                  P.impl_FieldElement__add
                    (mont_i16_to_spec_fe vec_i)
                    (P.impl_FieldElement__mul
                      (mont_i16_to_spec_fe zeta)
                      (mont_i16_to_spec_fe vec_j)))
  = let prod_fe = P.impl_FieldElement__mul (mont_i16_to_spec_fe zeta)
                                            (mont_i16_to_spec_fe vec_j) in
    lemma_impl_mul_v_val (mont_i16_to_spec_fe zeta) (mont_i16_to_spec_fe vec_j);
    lemma_impl_add_v_val (mont_i16_to_spec_fe vec_i) prod_fe;
    lemma_butterfly_mod_core (v vec_i) (v vec_j) (v zeta) (v result_i)

let lemma_butterfly_sub_mod_core (a b z r: int) :
    Lemma (requires r % 3329 == (a - b * z * 169) % 3329)
          (ensures  ((a * 169) % 3329
                     - (((z * 169) % 3329) * ((b * 169) % 3329)) % 3329) % 3329
                    == (r * 169) % 3329)
  = let q : pos = 3329 in
    L.lemma_mod_mul_distr_l (z * 169) ((b * 169) % q) q;
    L.lemma_mod_mul_distr_r (z * 169) (b * 169) q;
    assert ((z * 169) * (b * 169) == b * z * 169 * 169);
    L.lemma_mod_sub_distr ((a * 169) % q) (b * z * 169 * 169) q;
    L.lemma_mod_add_distr (- (b * z * 169 * 169)) (a * 169) q;
    assert (a * 169 - b * z * 169 * 169 == (a - b * z * 169) * 169);
    L.lemma_mod_mul_distr_l (a - b * z * 169) 169 q;
    L.lemma_mod_mul_distr_l r 169 q

let lemma_butterfly_fe_commute_minus (vec_i vec_j zeta result_j: i16) :
  Lemma (requires v result_j % 3329
                  == (v vec_i - v vec_j * v zeta * 169) % 3329)
        (ensures  mont_i16_to_spec_fe result_j ==
                  P.impl_FieldElement__sub
                    (mont_i16_to_spec_fe vec_i)
                    (P.impl_FieldElement__mul
                      (mont_i16_to_spec_fe zeta)
                      (mont_i16_to_spec_fe vec_j)))
  = let prod_fe = P.impl_FieldElement__mul (mont_i16_to_spec_fe zeta)
                                            (mont_i16_to_spec_fe vec_j) in
    lemma_impl_mul_v_val (mont_i16_to_spec_fe zeta) (mont_i16_to_spec_fe vec_j);
    lemma_impl_sub_v_val (mont_i16_to_spec_fe vec_i) prod_fe;
    lemma_butterfly_sub_mod_core (v vec_i) (v vec_j) (v zeta) (v result_j)

(* Combined plus/minus: a single call produces both output-lane FE
   equations for one butterfly pair from its two `ntt_spec` residues. *)
let lemma_butterfly_pair_commute
    (vec result: t_Array i16 (mk_usize 16))
    (z: i16) (i: nat{i < 16}) (j: nat{j < 16}) :
  Lemma (requires
           v (Seq.index result i) % 3329
             == (v (Seq.index vec i) + v (Seq.index vec j) * v z * 169) % 3329 /\
           v (Seq.index result j) % 3329
             == (v (Seq.index vec i) - v (Seq.index vec j) * v z * 169) % 3329)
        (ensures
           mont_i16_to_spec_fe (Seq.index result i) ==
             P.impl_FieldElement__add
               (mont_i16_to_spec_fe (Seq.index vec i))
               (P.impl_FieldElement__mul (mont_i16_to_spec_fe z)
                                         (mont_i16_to_spec_fe (Seq.index vec j))) /\
           mont_i16_to_spec_fe (Seq.index result j) ==
             P.impl_FieldElement__sub
               (mont_i16_to_spec_fe (Seq.index vec i))
               (P.impl_FieldElement__mul (mont_i16_to_spec_fe z)
                                         (mont_i16_to_spec_fe (Seq.index vec j))))
  = lemma_butterfly_fe_commute_plus (Seq.index vec i) (Seq.index vec j) z (Seq.index result i);
    lemma_butterfly_fe_commute_minus (Seq.index vec i) (Seq.index vec j) z (Seq.index result j)

(* Inverse-butterfly (Gentleman–Sande) commute.  `inv_ntt_spec vec z i j
   result` gives:
     v result[i] % 3329 == (v vec[j] + v vec[i]) % 3329
     v result[j] % 3329 == ((v vec[j] − v vec[i]) · z · 169) % 3329
   and in FE algebra:
     mont_fe result[i] == mont_fe vec[i] + mont_fe vec[j]
     mont_fe result[j] == mont_fe z · (mont_fe vec[j] − mont_fe vec[i]). *)

let lemma_inv_butterfly_mul_diff_core (a b z r: int) :
    Lemma (requires r % 3329 == ((b - a) * z * 169) % 3329)
          (ensures  (((z * 169) % 3329)
                     * (((b * 169) % 3329 - (a * 169) % 3329) % 3329)) % 3329
                    == (r * 169) % 3329)
  = let q : pos = 3329 in
    L.lemma_mod_sub_distr ((b * 169) % q) (a * 169) q;
    L.lemma_mod_add_distr (- (a * 169)) (b * 169) q;
    assert (b * 169 - a * 169 == (b - a) * 169);
    L.lemma_mod_mul_distr_l (z * 169) (((b - a) * 169) % q) q;
    L.lemma_mod_mul_distr_r (z * 169) ((b - a) * 169) q;
    assert ((z * 169) * ((b - a) * 169) == (b - a) * z * 169 * 169);
    L.lemma_mod_mul_distr_l ((b - a) * z * 169) 169 q;
    L.lemma_mod_mul_distr_l r 169 q

let lemma_inv_butterfly_fe_commute_mul_diff (vec_i vec_j zeta result_j: i16) :
  Lemma (requires v result_j % 3329
                  == ((v vec_j - v vec_i) * v zeta * 169) % 3329)
        (ensures  mont_i16_to_spec_fe result_j ==
                  P.impl_FieldElement__mul
                    (mont_i16_to_spec_fe zeta)
                    (P.impl_FieldElement__sub
                      (mont_i16_to_spec_fe vec_j)
                      (mont_i16_to_spec_fe vec_i)))
  = let diff_fe = P.impl_FieldElement__sub (mont_i16_to_spec_fe vec_j)
                                            (mont_i16_to_spec_fe vec_i) in
    lemma_impl_sub_v_val (mont_i16_to_spec_fe vec_j) (mont_i16_to_spec_fe vec_i);
    lemma_impl_mul_v_val (mont_i16_to_spec_fe zeta) diff_fe;
    lemma_inv_butterfly_mul_diff_core (v vec_i) (v vec_j) (v zeta) (v result_j)

(* Lane-post → mod_q_eq bridges.

   The trait posts for `barrett_reduce` and `montgomery_multiply_by_constant`
   wrap each per-lane equation in an opaque atom (`barrett_reduce_lane_post`
   / `montgomery_multiply_lane_post`) so above-trait consumers don't see
   the raw `mod_q_eq` (and the non-linear `* v c * 169` for Montgomery).
   These two thin lemmas reveal the atom inside the lemma body and yield
   the underlying `mod_q_eq` fact, so consumers can chain into existing
   `lemma_*_fe_commute_*_mod` lemmas without revealing at the call site. *)

let lemma_barrett_reduce_lane_post_to_mod_q_eq (a r: i16) :
    Lemma (requires TS.barrett_reduce_lane_post a r)
          (ensures  mod_q_eq (v r) (v a))
  = reveal_opaque (`%TS.barrett_reduce_lane_post) (TS.barrett_reduce_lane_post)

let lemma_montgomery_multiply_lane_post_to_mod_q_eq (a c r: i16) :
    Lemma (requires TS.montgomery_multiply_lane_post a c r)
          (ensures  mod_q_eq (v r) (v a * v c * 169))
  = reveal_opaque (`%TS.montgomery_multiply_lane_post) (TS.montgomery_multiply_lane_post)

(* Fused lane-post → FE-equation bridges.  Bundle the lane-atom reveal,
   `mod_q_eq` unfold, and the FE-commute step into one chunk-side lemma
   so consumers (e.g. `inv_ntt_layer_int_vec_step_reduce`'s aux1) make a
   single lemma call per lane.  This keeps the L7+L8 abstraction intact
   (consumers never see the raw `mod_q_eq` or `% 3329`) and reduces the
   Z3 quantifier-instantiation budget the consumer must spend. *)

let lemma_inv_butterfly_mont_lane_to_fe (a b zeta r b_minus_a: i16) :
    Lemma (requires TS.montgomery_multiply_lane_post b_minus_a zeta r
                  /\ v b_minus_a == v b - v a)
          (ensures  mont_i16_to_spec_fe r ==
                    P.impl_FieldElement__mul
                      (mont_i16_to_spec_fe zeta)
                      (P.impl_FieldElement__sub
                         (mont_i16_to_spec_fe b)
                         (mont_i16_to_spec_fe a)))
  = lemma_montgomery_multiply_lane_post_to_mod_q_eq b_minus_a zeta r;
    lemma_mod_q_eq_unfold (v r) (v b_minus_a * v zeta * 169);
    lemma_inv_butterfly_fe_commute_mul_diff a b zeta r

(* `base_case_multiply_even` commute.  The impl's `ntt_multiply_binomials_post`
   residue for the even lane is
     v r % q == ((a0*b0 + a1*b1*z*169) * 169) % q
   and the hacspec FE form is
     mont_fe r == (mont_fe a0 · mont_fe b0) + ((mont_fe a1 · mont_fe b1) · mont_fe z).

   Pending — C4e-followup.  Content is a direct analog of
   `lemma_butterfly_mod_core` with one extra Montgomery-unwrap level, and
   in isolation satisfies the ensures under a full mod-distr chain
   (`lemma_mod_mul_distr_l/r`, `lemma_mod_add_distr`).  Z3 does not
   converge on this encoding at r-limit 300 with `--split_queries
   always`: queries 1-4 succeed in 14, 54, 123, 642 ms but query 5 runs
   15+ minutes before timing out.  The auto-retry has been observed to
   succeed in 369 ms (one-off), confirming the math.  Deferred to a
   future pass — replace with the `butterfly` / `mont-mul` composition
   and/or per-level helper lemmas, or split the residue into two
   sub-lemmas. *)
#push-options "--z3rlimit 400"
let lemma_base_case_mult_even_mod_core (a0 a1 b0 b1 z r: int) :
    Lemma (requires r % 3329 == ((a0 * b0 + a1 * b1 * z * 169) * 169) % 3329)
          (ensures  ((((a0 * 169) % 3329 * (b0 * 169) % 3329) % 3329)
                     + ((((a1 * 169) % 3329 * (b1 * 169) % 3329) % 3329)
                        * ((z * 169) % 3329)) % 3329) % 3329
                    == (r * 169) % 3329)
  = let result = ((((a0 * 169) % 3329 * (b0 * 169) % 3329) % 3329)
                     + ((((a1 * 169) % 3329 * (b1 * 169) % 3329) % 3329)
                        * ((z * 169) % 3329)) % 3329) % 3329 in
    calc (==) {
      ((a0 * 169) % 3329) * (b0 * 169) % 3329;
      (==) {FStar.Math.Lemmas.lemma_mod_mul_distr_l
            (a0 * 169) (b0 * 169) 3329}
      ((a0 * 169) * (b0 * 169)) % 3329;
    };
    FStar.Math.Lemmas.lemma_mod_mod
      (((a0 * 169) * (b0 * 169)) % 3329)
      ((a0 * 169) * (b0 * 169))
      3329;
    calc (==) {
      ((a1 * 169) % 3329) * (b1 * 169) % 3329;
      (==) {FStar.Math.Lemmas.lemma_mod_mul_distr_l
            (a1 * 169) (b1 * 169) 3329}
      ((a1 * 169) * (b1 * 169)) % 3329;
    };
    FStar.Math.Lemmas.lemma_mod_mod
      (((a1 * 169) * (b1 * 169)) % 3329)
      ((a1 * 169) * (b1 * 169))
      3329;
    calc (==) {
     ((((a1 * 169) % 3329 * (b1 * 169) % 3329) % 3329)
                        * ((z * 169) % 3329)) % 3329;
     (==) { () }
     (((a1 * 169) * (b1 * 169)) % 3329
      * ((z * 169) % 3329)) % 3329;
     (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l
          ((a1 * 169) * (b1 * 169))
          ((z * 169) % 3329)
          3329}
     (((a1 * 169) * (b1 * 169))
      * ((z * 169) % 3329)) % 3329;
     (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_r
          ((a1 * 169) * (b1 * 169))
          (z * 169)
          3329}
     (((a1 * 169) * (b1 * 169)) * (z * 169)) % 3329;
    };
    calc (==) {
      result;
      (==) { () }
      (((a0 * 169) * (b0 * 169)) % 3329 +
       (((a1 * 169) * (b1 * 169)) * (z * 169)) % 3329) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_plus_distr_l
        ((a0 * 169) * (b0 * 169))
        ((((a1 * 169) * (b1 * 169)) * (z * 169)) % 3329)
        3329
        }
      (((a0 * 169) * (b0 * 169)) +
       (((a1 * 169) * (b1 * 169)) * (z * 169)) % 3329) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_plus_distr_r
        ((a0 * 169) * (b0 * 169))
        ((((a1 * 169) * (b1 * 169)) * (z * 169)))
        3329
        }
      (((a0 * 169) * (b0 * 169)) +
       (((a1 * 169) * (b1 * 169)) * (z * 169))) % 3329;
    };
    calc (==) {
      (r * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l r 169 3329}
      (r % 3329 * 169) % 3329;
      (==) { () }
      ((((a0 * b0 + a1 * b1 * z * 169) * 169) % 3329) * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l
      ((a0 * b0 + a1 * b1 * z * 169) * 169)
      169 3329}
      (((a0 * b0 + a1 * b1 * z * 169) * 169) * 169) % 3329;
    };
    assert ((r * 169) % 3329 ==
      (((a0 * b0 + a1 * b1 * z * 169) * 169) * 169) % 3329)
#pop-options

(* Int-level FE-chain bridge for A1.  The chain
   `impl_add (impl_mul (mont a0) (mont b0)) (impl_mul (impl_mul (mont a1)
   (mont b1)) (mont z))` produces an f_val with both factors of each
   inner product inner-modded (`((a*169)%q) * ((b*169)%q)`).  A1's
   ensures has only the first factor inner-modded.  This wrapper does
   the two `lemma_mod_mul_distr_r` absorptions and invokes A1, so the
   FE-level lemma A2 is a clean chain. *)
#push-options "--z3rlimit 400"
let lemma_base_case_mult_even_mod_core_fe_form
    (a0 a1 b0 b1 z r: int) :
    Lemma (requires r % 3329 == ((a0 * b0 + a1 * b1 * z * 169) * 169) % 3329)
          (ensures  ((((a0 * 169) % 3329) * ((b0 * 169) % 3329)) % 3329
                     + ((((a1 * 169) % 3329) * ((b1 * 169) % 3329)) % 3329
                        * ((z * 169) % 3329)) % 3329) % 3329
                    == (r * 169) % 3329)
  = L.lemma_mod_mul_distr_r ((a0 * 169) % 3329) (b0 * 169) 3329;
    L.lemma_mod_mul_distr_r ((a1 * 169) % 3329) (b1 * 169) 3329;
    lemma_base_case_mult_even_mod_core a0 a1 b0 b1 z r
#pop-options

let lemma_base_case_mult_even_fe_commute
    (a0 a1 b0 b1 zeta result: i16) :
  Lemma (requires v result % 3329
                  == ((v a0 * v b0 + v a1 * v b1 * v zeta * 169) * 169) % 3329)
        (ensures
           mont_i16_to_spec_fe result ==
             P.impl_FieldElement__add
               (P.impl_FieldElement__mul (mont_i16_to_spec_fe a0) (mont_i16_to_spec_fe b0))
               (P.impl_FieldElement__mul
                 (P.impl_FieldElement__mul (mont_i16_to_spec_fe a1) (mont_i16_to_spec_fe b1))
                 (mont_i16_to_spec_fe zeta)))
  = let a0_fe = mont_i16_to_spec_fe a0 in
    let b0_fe = mont_i16_to_spec_fe b0 in
    let a1_fe = mont_i16_to_spec_fe a1 in
    let b1_fe = mont_i16_to_spec_fe b1 in
    let z_fe  = mont_i16_to_spec_fe zeta in
    let prod_a  = P.impl_FieldElement__mul a0_fe b0_fe in
    let prod_b  = P.impl_FieldElement__mul a1_fe b1_fe in
    let prod_bz = P.impl_FieldElement__mul prod_b z_fe in
    lemma_impl_mul_v_val a0_fe b0_fe;
    lemma_impl_mul_v_val a1_fe b1_fe;
    lemma_impl_mul_v_val prod_b z_fe;
    lemma_impl_add_v_val prod_a prod_bz;
    lemma_base_case_mult_even_mod_core_fe_form
      (v a0) (v a1) (v b0) (v b1) (v zeta) (v result)

(* `base_case_multiply_odd` has no zeta: residue
     v r % q == ((a0*b1 + a1*b0) * 169) % q
   FE form: mont_fe r == (mont_fe a0 · mont_fe b1) + (mont_fe a1 · mont_fe b0).
   Same calc structure as `lemma_base_case_mult_even_mod_core`, minus the
   z·169 factor: just two mul terms then add then pull-out. *)
#push-options "--z3rlimit 400"
let lemma_base_case_mult_odd_mod_core (a0 a1 b0 b1 r: int) :
    Lemma (requires r % 3329 == ((a0 * b1 + a1 * b0) * 169) % 3329)
          (ensures  ((((a0 * 169) % 3329 * (b1 * 169) % 3329) % 3329)
                     + (((a1 * 169) % 3329 * (b0 * 169) % 3329) % 3329)) % 3329
                    == (r * 169) % 3329)
  = let result = ((((a0 * 169) % 3329 * (b1 * 169) % 3329) % 3329)
                     + (((a1 * 169) % 3329 * (b0 * 169) % 3329) % 3329)) % 3329 in
    calc (==) {
      ((a0 * 169) % 3329) * (b1 * 169) % 3329;
      (==) {FStar.Math.Lemmas.lemma_mod_mul_distr_l
            (a0 * 169) (b1 * 169) 3329}
      ((a0 * 169) * (b1 * 169)) % 3329;
    };
    FStar.Math.Lemmas.lemma_mod_mod
      (((a0 * 169) * (b1 * 169)) % 3329)
      ((a0 * 169) * (b1 * 169))
      3329;
    calc (==) {
      ((a1 * 169) % 3329) * (b0 * 169) % 3329;
      (==) {FStar.Math.Lemmas.lemma_mod_mul_distr_l
            (a1 * 169) (b0 * 169) 3329}
      ((a1 * 169) * (b0 * 169)) % 3329;
    };
    FStar.Math.Lemmas.lemma_mod_mod
      (((a1 * 169) * (b0 * 169)) % 3329)
      ((a1 * 169) * (b0 * 169))
      3329;
    calc (==) {
      result;
      (==) { () }
      (((a0 * 169) * (b1 * 169)) % 3329 +
       ((a1 * 169) * (b0 * 169)) % 3329) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_plus_distr_l
        ((a0 * 169) * (b1 * 169))
        (((a1 * 169) * (b0 * 169)) % 3329)
        3329
        }
      (((a0 * 169) * (b1 * 169)) +
       ((a1 * 169) * (b0 * 169)) % 3329) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_plus_distr_r
        ((a0 * 169) * (b1 * 169))
        ((a1 * 169) * (b0 * 169))
        3329
        }
      (((a0 * 169) * (b1 * 169)) +
       ((a1 * 169) * (b0 * 169))) % 3329;
    };
    calc (==) {
      (r * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l r 169 3329}
      (r % 3329 * 169) % 3329;
      (==) { }
      ((((a0 * b1 + a1 * b0) * 169) % 3329) * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l
      ((a0 * b1 + a1 * b0) * 169)
      169 3329}
      (((a0 * b1 + a1 * b0) * 169) * 169) % 3329;
    };
    assert ((r * 169) % 3329 ==
      (((a0 * b1 + a1 * b0) * 169) * 169) % 3329);
    ()
#pop-options

(* Int-level FE-chain bridge for A3, analogous to A1's `_fe_form`.
   Two `lemma_mod_mul_distr_r` absorb the redundant inner mods, then A3. *)
#push-options "--z3rlimit 400"
let lemma_base_case_mult_odd_mod_core_fe_form
    (a0 a1 b0 b1 r: int) :
    Lemma (requires r % 3329 == ((a0 * b1 + a1 * b0) * 169) % 3329)
          (ensures  ((((a0 * 169) % 3329) * ((b1 * 169) % 3329)) % 3329
                     + (((a1 * 169) % 3329) * ((b0 * 169) % 3329)) % 3329) % 3329
                    == (r * 169) % 3329)
  = L.lemma_mod_mul_distr_r ((a0 * 169) % 3329) (b1 * 169) 3329;
    L.lemma_mod_mul_distr_r ((a1 * 169) % 3329) (b0 * 169) 3329;
    lemma_base_case_mult_odd_mod_core a0 a1 b0 b1 r
#pop-options

let lemma_base_case_mult_odd_fe_commute
    (a0 a1 b0 b1 result: i16) :
  Lemma (requires v result % 3329
                  == ((v a0 * v b1 + v a1 * v b0) * 169) % 3329)
        (ensures
           mont_i16_to_spec_fe result ==
             P.impl_FieldElement__add
               (P.impl_FieldElement__mul (mont_i16_to_spec_fe a0) (mont_i16_to_spec_fe b1))
               (P.impl_FieldElement__mul (mont_i16_to_spec_fe a1) (mont_i16_to_spec_fe b0)))
  = let a0_fe = mont_i16_to_spec_fe a0 in
    let b1_fe = mont_i16_to_spec_fe b1 in
    let a1_fe = mont_i16_to_spec_fe a1 in
    let b0_fe = mont_i16_to_spec_fe b0 in
    let prod_a = P.impl_FieldElement__mul a0_fe b1_fe in
    let prod_b = P.impl_FieldElement__mul a1_fe b0_fe in
    lemma_impl_mul_v_val a0_fe b1_fe;
    lemma_impl_mul_v_val a1_fe b0_fe;
    lemma_impl_add_v_val prod_a prod_b;
    lemma_base_case_mult_odd_mod_core_fe_form
      (v a0) (v a1) (v b0) (v b1) (v result)

(* Combined base-case multiply per binomial pair: takes both residue
   equations from `ntt_multiply_binomials_post` at binomial k, produces
   both FE equations for lanes 2k and 2k+1. *)
let lemma_base_case_mult_pair_commute
    (a b result: t_Array i16 (mk_usize 16))
    (zeta: i16) (k: nat{2 * k + 1 < 16}) :
  Lemma (requires
           v (Seq.index result (2 * k)) % 3329
             == ((v (Seq.index a (2 * k)) * v (Seq.index b (2 * k))
                  + v (Seq.index a (2 * k + 1)) * v (Seq.index b (2 * k + 1)) * v zeta * 169)
                 * 169) % 3329 /\
           v (Seq.index result (2 * k + 1)) % 3329
             == ((v (Seq.index a (2 * k)) * v (Seq.index b (2 * k + 1))
                  + v (Seq.index a (2 * k + 1)) * v (Seq.index b (2 * k)))
                 * 169) % 3329)
        (ensures
           mont_i16_to_spec_fe (Seq.index result (2 * k)) ==
             P.impl_FieldElement__add
               (P.impl_FieldElement__mul
                 (mont_i16_to_spec_fe (Seq.index a (2 * k)))
                 (mont_i16_to_spec_fe (Seq.index b (2 * k))))
               (P.impl_FieldElement__mul
                 (P.impl_FieldElement__mul
                   (mont_i16_to_spec_fe (Seq.index a (2 * k + 1)))
                   (mont_i16_to_spec_fe (Seq.index b (2 * k + 1))))
                 (mont_i16_to_spec_fe zeta)) /\
           mont_i16_to_spec_fe (Seq.index result (2 * k + 1)) ==
             P.impl_FieldElement__add
               (P.impl_FieldElement__mul
                 (mont_i16_to_spec_fe (Seq.index a (2 * k)))
                 (mont_i16_to_spec_fe (Seq.index b (2 * k + 1))))
               (P.impl_FieldElement__mul
                 (mont_i16_to_spec_fe (Seq.index a (2 * k + 1)))
                 (mont_i16_to_spec_fe (Seq.index b (2 * k)))))
  = lemma_base_case_mult_even_fe_commute
      (Seq.index a (2 * k)) (Seq.index a (2 * k + 1))
      (Seq.index b (2 * k)) (Seq.index b (2 * k + 1))
      zeta (Seq.index result (2 * k));
    lemma_base_case_mult_odd_fe_commute
      (Seq.index a (2 * k)) (Seq.index a (2 * k + 1))
      (Seq.index b (2 * k)) (Seq.index b (2 * k + 1))
      (Seq.index result (2 * k + 1))

(* One inv-butterfly pair from its two `inv_ntt_spec` residues. *)
let lemma_inv_butterfly_pair_commute
    (vec result: t_Array i16 (mk_usize 16))
    (z: i16) (i: nat{i < 16}) (j: nat{j < 16}) :
  Lemma (requires
           v (Seq.index result i) % 3329
             == (v (Seq.index vec j) + v (Seq.index vec i)) % 3329 /\
           v (Seq.index result j) % 3329
             == ((v (Seq.index vec j) - v (Seq.index vec i)) * v z * 169) % 3329)
        (ensures
           mont_i16_to_spec_fe (Seq.index result i) ==
             P.impl_FieldElement__add
               (mont_i16_to_spec_fe (Seq.index vec i))
               (mont_i16_to_spec_fe (Seq.index vec j)) /\
           mont_i16_to_spec_fe (Seq.index result j) ==
             P.impl_FieldElement__mul
               (mont_i16_to_spec_fe z)
               (P.impl_FieldElement__sub
                 (mont_i16_to_spec_fe (Seq.index vec j))
                 (mont_i16_to_spec_fe (Seq.index vec i))))
  = lemma_add_fe_commute_mont_mod (Seq.index vec i) (Seq.index vec j) (Seq.index result i);
    lemma_inv_butterfly_fe_commute_mul_diff
      (Seq.index vec i) (Seq.index vec j) z (Seq.index result j)

(* Montgomery multiplication of two Montgomery-form operands: the impl
   computes `r = a * b * R^{-1}` with `R^{-1} = 169`.  Under the
   Montgomery lift this is `fe(a) * fe(b)` in the plain FE algebra —
   the impl-side Montgomery factor cancels against the lift's R-stripping. *)
let lemma_mont_mul_fe_commute_mont_mont (a b r: i16) :
    Lemma (requires v r % 3329 == (v a * v b * 169) % 3329)
          (ensures  P.impl_FieldElement__mul
                        (mont_i16_to_spec_fe a) (mont_i16_to_spec_fe b)
                    == mont_i16_to_spec_fe r)
  = lemma_impl_mul_v_val (mont_i16_to_spec_fe a) (mont_i16_to_spec_fe b);
    lemma_mont_mul_mod_core (v a) (v b) (v r)

(* Mixed mode: `a` Montgomery, `b` plain; result is plain. *)
let lemma_mont_mul_fe_commute_mont_plain (a b r: i16) :
    Lemma (requires v r % 3329 == (v a * v b * 169) % 3329)
          (ensures  P.impl_FieldElement__mul
                        (mont_i16_to_spec_fe a) (i16_to_spec_fe b)
                    == i16_to_spec_fe r)
  = lemma_impl_mul_v_val (mont_i16_to_spec_fe a) (i16_to_spec_fe b);
    lemma_mont_plain_mul_mod_core (v a) (v b) (v r)

(* Plain multiplication by a constant coefficient. *)
let lemma_mul_const_fe_commute_plain (a c r: i16) :
    Lemma (requires v r == v a * v c)
          (ensures  P.impl_FieldElement__mul (i16_to_spec_fe a) (i16_to_spec_fe c)
                    == i16_to_spec_fe r)
  = lemma_impl_mul_v_val (i16_to_spec_fe a) (i16_to_spec_fe c);
    lemma_mul_const_mod_core (v a) (v c) (v r)

(* Layer 1 — 16-lane chunk commute lemmas.
   Each statement is quantified over `{| T.t_Operations vV |}` so the
   single lemma instance covers portable, avx2, and (once admitted-out
   neon is revisited) every backend.  The pre-condition is the trait's
   own `TS.<op>_pre` — anything already holding the impl pre gets the
   commute statement for free.  Proofs are deferred to C3b: each folds
   its corresponding Layer-0 lemma 16 times over the lanes via
   `createi_lemma` (SMTPat, from Hacspec_ml_kem.Parameters).

   ────────────  Linear ops  ────────────
   `add`/`sub` are R-linear so the same impl equation lifts through
   either `i16_to_spec_fe` or `mont_i16_to_spec_fe`; two lemmas per op. *)

(* ────────────  Per-lane index helpers  ────────────
   Expose `Seq.index (i16_to_spec_array (sz 16) x) j` as `i16_to_spec_fe (Seq.index x j)`
   for a `nat` index `j` (the trait's `_post` predicates quantify over nat).
   The underlying `createi_lemma` SMTPat only fires for `(v i)` with `i: usize`,
   so we wrap it once and use the nat variant throughout Layer 1. *)

let lane_plain (#n: usize) (x: t_Array i16 n) (j: nat {j < v n}) :
    Lemma (Seq.index (i16_to_spec_array n x) j
           == i16_to_spec_fe (Seq.index x j))
  = P.createi_lemma #P.t_FieldElement n #(usize -> P.t_FieldElement)
      (fun k -> (i16_to_spec_fe (Seq.index x (v k)) <: P.t_FieldElement)) (sz j)

let lane_mont (#n: usize) (x: t_Array i16 n) (j: nat {j < v n}) :
    Lemma (Seq.index (mont_i16_to_spec_array n x) j
           == mont_i16_to_spec_fe (Seq.index x j))
  = P.createi_lemma #P.t_FieldElement n #(usize -> P.t_FieldElement)
      (fun k -> (mont_i16_to_spec_fe (Seq.index x (v k)) <: P.t_FieldElement)) (sz j)

let lemma_add_chunk_commutes_plain
    (#vV: Type0) {| i: T.t_Operations vV |}
    (lhs rhs: vV) :
  Lemma
    (requires TS.add_pre (T.f_repr lhs) (T.f_repr rhs))
    (ensures
       (let r = T.f_add lhs rhs in
        forall (j: nat). j < 16 ==>
          Seq.index (i16_to_spec_array (sz 16) (T.f_repr r)) j
            == P.impl_FieldElement__add
                 (Seq.index (i16_to_spec_array (sz 16) (T.f_repr lhs)) j)
                 (Seq.index (i16_to_spec_array (sz 16) (T.f_repr rhs)) j)))
  = let r = T.f_add lhs rhs in
    let lhs_arr = T.f_repr lhs in
    let rhs_arr = T.f_repr rhs in
    let r_arr = T.f_repr r in
    let aux (j: nat) : Lemma (j < 16 ==>
        Seq.index (i16_to_spec_array (sz 16) r_arr) j
          == P.impl_FieldElement__add
               (Seq.index (i16_to_spec_array (sz 16) lhs_arr) j)
               (Seq.index (i16_to_spec_array (sz 16) rhs_arr) j))
      = if j < 16 then begin
          lane_plain r_arr j;
          lane_plain lhs_arr j;
          lane_plain rhs_arr j;
          lemma_add_fe_commute_plain (Seq.index lhs_arr j) (Seq.index rhs_arr j) (Seq.index r_arr j)
        end in
    Classical.forall_intro aux

let lemma_add_chunk_commutes_mont
    (#vV: Type0) {| i: T.t_Operations vV |}
    (lhs rhs: vV) :
  Lemma
    (requires TS.add_pre (T.f_repr lhs) (T.f_repr rhs))
    (ensures
       (let r = T.f_add lhs rhs in
        forall (j: nat). j < 16 ==>
          Seq.index (mont_i16_to_spec_array (sz 16) (T.f_repr r)) j
            == P.impl_FieldElement__add
                 (Seq.index (mont_i16_to_spec_array (sz 16) (T.f_repr lhs)) j)
                 (Seq.index (mont_i16_to_spec_array (sz 16) (T.f_repr rhs)) j)))
  = let r = T.f_add lhs rhs in
    let lhs_arr = T.f_repr lhs in
    let rhs_arr = T.f_repr rhs in
    let r_arr = T.f_repr r in
    let aux (j: nat) : Lemma (j < 16 ==>
        Seq.index (mont_i16_to_spec_array (sz 16) r_arr) j
          == P.impl_FieldElement__add
               (Seq.index (mont_i16_to_spec_array (sz 16) lhs_arr) j)
               (Seq.index (mont_i16_to_spec_array (sz 16) rhs_arr) j))
      = if j < 16 then begin
          lane_mont r_arr j;
          lane_mont lhs_arr j;
          lane_mont rhs_arr j;
          lemma_add_fe_commute_mont (Seq.index lhs_arr j) (Seq.index rhs_arr j) (Seq.index r_arr j)
        end in
    Classical.forall_intro aux

let lemma_sub_chunk_commutes_plain
    (#vV: Type0) {| i: T.t_Operations vV |}
    (lhs rhs: vV) :
  Lemma
    (requires TS.sub_pre (T.f_repr lhs) (T.f_repr rhs))
    (ensures
       (let r = T.f_sub lhs rhs in
        forall (j: nat). j < 16 ==>
          Seq.index (i16_to_spec_array (sz 16) (T.f_repr r)) j
            == P.impl_FieldElement__sub
                 (Seq.index (i16_to_spec_array (sz 16) (T.f_repr lhs)) j)
                 (Seq.index (i16_to_spec_array (sz 16) (T.f_repr rhs)) j)))
  = let r = T.f_sub lhs rhs in
    let lhs_arr = T.f_repr lhs in
    let rhs_arr = T.f_repr rhs in
    let r_arr = T.f_repr r in
    let aux (j: nat) : Lemma (j < 16 ==>
        Seq.index (i16_to_spec_array (sz 16) r_arr) j
          == P.impl_FieldElement__sub
               (Seq.index (i16_to_spec_array (sz 16) lhs_arr) j)
               (Seq.index (i16_to_spec_array (sz 16) rhs_arr) j))
      = if j < 16 then begin
          lane_plain r_arr j;
          lane_plain lhs_arr j;
          lane_plain rhs_arr j;
          lemma_sub_fe_commute_plain (Seq.index lhs_arr j) (Seq.index rhs_arr j) (Seq.index r_arr j)
        end in
    Classical.forall_intro aux

let lemma_sub_chunk_commutes_mont
    (#vV: Type0) {| i: T.t_Operations vV |}
    (lhs rhs: vV) :
  Lemma
    (requires TS.sub_pre (T.f_repr lhs) (T.f_repr rhs))
    (ensures
       (let r = T.f_sub lhs rhs in
        forall (j: nat). j < 16 ==>
          Seq.index (mont_i16_to_spec_array (sz 16) (T.f_repr r)) j
            == P.impl_FieldElement__sub
                 (Seq.index (mont_i16_to_spec_array (sz 16) (T.f_repr lhs)) j)
                 (Seq.index (mont_i16_to_spec_array (sz 16) (T.f_repr rhs)) j)))
  = let r = T.f_sub lhs rhs in
    let lhs_arr = T.f_repr lhs in
    let rhs_arr = T.f_repr rhs in
    let r_arr = T.f_repr r in
    let aux (j: nat) : Lemma (j < 16 ==>
        Seq.index (mont_i16_to_spec_array (sz 16) r_arr) j
          == P.impl_FieldElement__sub
               (Seq.index (mont_i16_to_spec_array (sz 16) lhs_arr) j)
               (Seq.index (mont_i16_to_spec_array (sz 16) rhs_arr) j))
      = if j < 16 then begin
          lane_mont r_arr j;
          lane_mont lhs_arr j;
          lane_mont rhs_arr j;
          lemma_sub_fe_commute_mont (Seq.index lhs_arr j) (Seq.index rhs_arr j) (Seq.index r_arr j)
        end in
    Classical.forall_intro aux

(* ────────────  Scalar-multiplication ops  ────────────
   Plain × plain, Mont × Mont (→ Mont), and Mont × plain (→ plain). *)

let lemma_multiply_by_constant_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (c: i16) :
  Lemma
    (requires TS.multiply_by_constant_pre (T.f_repr vec) c)
    (ensures
       (let r = T.f_multiply_by_constant vec c in
        forall (j: nat). j < 16 ==>
          Seq.index (i16_to_spec_array (sz 16) (T.f_repr r)) j
            == P.impl_FieldElement__mul
                 (Seq.index (i16_to_spec_array (sz 16) (T.f_repr vec)) j)
                 (i16_to_spec_fe c)))
  = let r = T.f_multiply_by_constant vec c in
    let vec_arr = T.f_repr vec in
    let r_arr = T.f_repr r in
    let aux (j: nat) : Lemma (j < 16 ==>
        Seq.index (i16_to_spec_array (sz 16) r_arr) j
          == P.impl_FieldElement__mul
               (Seq.index (i16_to_spec_array (sz 16) vec_arr) j)
               (i16_to_spec_fe c))
      = if j < 16 then begin
          lane_plain r_arr j;
          lane_plain vec_arr j;
          lemma_mul_const_fe_commute_plain (Seq.index vec_arr j) c (Seq.index r_arr j)
        end in
    Classical.forall_intro aux

let lemma_montgomery_multiply_by_constant_chunk_commutes_mont_mont
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (c: i16) :
  Lemma
    (requires TS.montgomery_multiply_by_constant_pre (T.f_repr vec) c)
    (ensures
       (let r = T.f_montgomery_multiply_by_constant vec c in
        forall (j: nat). j < 16 ==>
          Seq.index (mont_i16_to_spec_array (sz 16) (T.f_repr r)) j
            == P.impl_FieldElement__mul
                 (Seq.index (mont_i16_to_spec_array (sz 16) (T.f_repr vec)) j)
                 (mont_i16_to_spec_fe c)))
  = (* Reveal the lane-form opaque so the per-lane `mod_q_eq` from the
       trait post is in scope below. *)
    reveal_opaque (`%TS.montgomery_multiply_lane_post) (TS.montgomery_multiply_lane_post);
    let r = T.f_montgomery_multiply_by_constant vec c in
    let vec_arr = T.f_repr vec in
    let r_arr = T.f_repr r in
    let aux (j: nat) : Lemma (j < 16 ==>
        Seq.index (mont_i16_to_spec_array (sz 16) r_arr) j
          == P.impl_FieldElement__mul
               (Seq.index (mont_i16_to_spec_array (sz 16) vec_arr) j)
               (mont_i16_to_spec_fe c))
      = if j < 16 then begin
          lane_mont r_arr j;
          lane_mont vec_arr j;
          lemma_mod_q_eq_unfold (v (Seq.index r_arr j)) (v (Seq.index vec_arr j) * v c * 169);
          lemma_mont_mul_fe_commute_mont_mont (Seq.index vec_arr j) c (Seq.index r_arr j)
        end in
    Classical.forall_intro aux

let lemma_montgomery_multiply_by_constant_chunk_commutes_mont_plain
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (c: i16) :
  Lemma
    (requires TS.montgomery_multiply_by_constant_pre (T.f_repr vec) c)
    (ensures
       (let r = T.f_montgomery_multiply_by_constant vec c in
        forall (j: nat). j < 16 ==>
          Seq.index (i16_to_spec_array (sz 16) (T.f_repr r)) j
            == P.impl_FieldElement__mul
                 (Seq.index (mont_i16_to_spec_array (sz 16) (T.f_repr vec)) j)
                 (i16_to_spec_fe c)))
  = (* Reveal the lane-form opaque so the per-lane `mod_q_eq` from the
       trait post is in scope below. *)
    reveal_opaque (`%TS.montgomery_multiply_lane_post) (TS.montgomery_multiply_lane_post);
    let r = T.f_montgomery_multiply_by_constant vec c in
    let vec_arr = T.f_repr vec in
    let r_arr = T.f_repr r in
    let aux (j: nat) : Lemma (j < 16 ==>
        Seq.index (i16_to_spec_array (sz 16) r_arr) j
          == P.impl_FieldElement__mul
               (Seq.index (mont_i16_to_spec_array (sz 16) vec_arr) j)
               (i16_to_spec_fe c))
      = if j < 16 then begin
          lane_plain r_arr j;
          lane_mont vec_arr j;
          lemma_mod_q_eq_unfold (v (Seq.index r_arr j)) (v (Seq.index vec_arr j) * v c * 169);
          lemma_mont_mul_fe_commute_mont_plain (Seq.index vec_arr j) c (Seq.index r_arr j)
        end in
    Classical.forall_intro aux

(* ────────────  Identity-on-lift ops  ────────────
   `barrett_reduce`, `cond_subtract_3329`, `to_unsigned_representative`
   all preserve the residue class mod q, so they are the identity on the
   plain FE lift. *)

let lemma_barrett_reduce_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) :
  Lemma
    (requires TS.barrett_reduce_pre (T.f_repr vec))
    (ensures
       (let r = T.f_barrett_reduce vec in
        i16_to_spec_array (sz 16) (T.f_repr r) == i16_to_spec_array (sz 16) (T.f_repr vec)))
  = (* Reveal the lane-form opaque so the per-lane `mod_q_eq` from the
       trait post is in scope below. *)
    reveal_opaque (`%TS.barrett_reduce_lane_post) (TS.barrett_reduce_lane_post);
    let r = T.f_barrett_reduce vec in
    let vec_arr = T.f_repr vec in
    let r_arr = T.f_repr r in
    let aux (j: nat) : Lemma (j < 16 ==>
        Seq.index (i16_to_spec_array (sz 16) r_arr) j
          == Seq.index (i16_to_spec_array (sz 16) vec_arr) j)
      = if j < 16 then begin
          lane_plain r_arr j;
          lane_plain vec_arr j;
          lemma_mod_q_eq_unfold (v (Seq.index r_arr j)) (v (Seq.index vec_arr j));
          lemma_barrett_fe_commute (Seq.index vec_arr j) (Seq.index r_arr j)
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (i16_to_spec_array (sz 16) r_arr) (i16_to_spec_array (sz 16) vec_arr)

let lemma_cond_subtract_3329_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) :
  Lemma
    (requires TS.cond_subtract_3329_pre (T.f_repr vec))
    (ensures
       (let r = T.f_cond_subtract_3329_ vec in
        i16_to_spec_array (sz 16) (T.f_repr r) == i16_to_spec_array (sz 16) (T.f_repr vec)))
  = let r = T.f_cond_subtract_3329_ vec in
    let vec_arr = T.f_repr vec in
    let r_arr = T.f_repr r in
    let aux (j: nat) : Lemma (j < 16 ==>
        Seq.index (i16_to_spec_array (sz 16) r_arr) j
          == Seq.index (i16_to_spec_array (sz 16) vec_arr) j)
      = if j < 16 then begin
          lane_plain r_arr j;
          lane_plain vec_arr j;
          lemma_mod_q_eq_unfold (v (Seq.index r_arr j)) (v (Seq.index vec_arr j));
          lemma_barrett_fe_commute (Seq.index vec_arr j) (Seq.index r_arr j)
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (i16_to_spec_array (sz 16) r_arr) (i16_to_spec_array (sz 16) vec_arr)

let lemma_to_unsigned_representative_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) :
  Lemma
    (requires TS.to_unsigned_representative_pre (T.f_repr vec))
    (ensures
       (let r = T.f_to_unsigned_representative vec in
        i16_to_spec_array (sz 16) (T.f_repr r) == i16_to_spec_array (sz 16) (T.f_repr vec)))
  = let r = T.f_to_unsigned_representative vec in
    let vec_arr = T.f_repr vec in
    let r_arr = T.f_repr r in
    assert (TS.to_unsigned_representative_post vec_arr r_arr);
    let aux (j: nat) : Lemma (j < 16 ==>
        Seq.index (i16_to_spec_array (sz 16) r_arr) j
          == Seq.index (i16_to_spec_array (sz 16) vec_arr) j)
      = if j < 16 then begin
          let x = Seq.index vec_arr j in
          let y = Seq.index r_arr j in
          lemma_mod_q_eq_unfold (v y) (v x);
          assert (v y >= 0 /\ v y <= 3328 /\ (v y % 3329 == v x % 3329));
          lane_plain r_arr j;
          lane_plain vec_arr j;
          lemma_barrett_fe_commute x y
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (i16_to_spec_array (sz 16) r_arr) (i16_to_spec_array (sz 16) vec_arr)

(* ────────────  Compress / decompress per-lane fe_commute  ────────────
   Per-lane lifts from the impl's integer formulas (as exposed by the
   Vector.{Portable,Avx2}.Compress primitive posts) up to the FE form
   `i16_to_spec_fe r == compress_d (i16_to_spec_fe fe) d`.  The trait
   posts use these via `compress_post_N` / `decompress_post_N`. *)

(* A5/A6/A7 ADMITTED — F* segfaults during type-checking when these
   lemmas have `= ()` bodies even with appropriate rlimits.  Likely an
   F* bug in handling the Hacspec_ml_kem.Compress dependency at this
   scale.  Statements are correct (3-case integer split for A5,
   2-value enumeration for A6, formula congruence for A7 — see
   `proofs/manual-proof-targets.md` and inline comments below).
   Left for the user; signatures preserved so consumers can call them. *)

let lemma_compress_message_coefficient_fe_commute (fe result: i16) :
  Lemma (requires v fe >= 0 /\ v fe < 3329 /\
                  v result == ((v fe * 4 + 3329) / 6658) % 2)
        (ensures Hacspec_ml_kem.Compress.compress_d
                   (i16_to_spec_fe fe) (mk_usize 1)
                 == i16_to_spec_fe result)
  = assert(v (i16_to_spec_fe fe).f_val == v fe);
    assert(v (i16_to_spec_fe result).f_val == v result);
    ()

(* A8 (parameterized compress): closed Phase 2 / B6 (Wave-A coordinator,
   2026-04-29).  Statement matches the integer-form post of
   `compress<D>` in portable/compress.rs.  Each D ∈ {4, 5, 10, 11}
   reduces compress_d's u32 body to the integer formula in `requires`
   once `pow2 (v d)` is concretized; the two `f_val` asserts pin
   `i16_to_spec_fe` for both inputs.  Mirrors A5's (D=1) shape. *)
#push-options "--z3rlimit 400 --fuel 1 --ifuel 1"
let lemma_compress_ciphertext_coefficient_fe_commute (fe result: i16) (d: usize) :
  Lemma (requires (v d == 4 \/ v d == 5 \/ v d == 10 \/ v d == 11) /\
                  v fe >= 0 /\ v fe < 3329 /\
                  v result == ((v fe * 2 * pow2 (v d) + 3329) / 6658) % pow2 (v d))
        (ensures Hacspec_ml_kem.Compress.compress_d
                   (i16_to_spec_fe fe) d
                 == i16_to_spec_fe result)
  = assert (v (i16_to_spec_fe fe).P.f_val == v fe);
    assert (v (i16_to_spec_fe result).P.f_val == v result);
    assert (pow2 4 == 16);
    assert (pow2 5 == 32);
    assert (pow2 10 == 1024);
    assert (pow2 11 == 2048)
#pop-options

let lemma_decompress_1_fe_commute (a result: i16) :
  Lemma (requires v a >= 0 /\ v a <= 1 /\
                  v result == (if v a = 0 then 0 else 1665))
        (ensures Hacspec_ml_kem.Compress.decompress_d
                   (i16_to_spec_fe a) (mk_usize 1)
                 == i16_to_spec_fe result)
  = ()

(* A6' (integer form): matches the per-element primitive post on
   portable's `decompress_1` directly, without the case-split. *)
let lemma_decompress_1_fe_commute_int (a result: i16) :
  Lemma (requires v a >= 0 /\ v a <= 1 /\
                  v result == (2 * v a * 3329 + 2) / 4)
        (ensures Hacspec_ml_kem.Compress.decompress_d
                   (i16_to_spec_fe a) (mk_usize 1)
                 == i16_to_spec_fe result)
  = ()

let lemma_decompress_ciphertext_coefficient_fe_commute
    (a result: i16) (d: usize) :
  Lemma (requires (v d == 4 \/ v d == 5 \/ v d == 10 \/ v d == 11) /\
                  v a >= 0 /\ v a < pow2 (v d) /\
                  v result == (2 * v a * 3329 + pow2 (v d)) / (pow2 (v d) * 2))
        (ensures Hacspec_ml_kem.Compress.decompress_d
                   (i16_to_spec_fe a) d
                 == i16_to_spec_fe result)
  = ()

(* ────────────  Compress / decompress  ────────────
   Reuse the array-length-generic predicates already defined in
   Traits.Spec so Layer 2 at N = 256 can cite the same shape. *)

(* The trait field's post is now in `Spec.Utils.forall16` form (faster
   for callers per the C4-era forall benchmark — Form 1 was 44× slower
   at N=16).  These chunk-level lemmas mirror that shape so any caller
   that used to project from compress_post_N (Form 1) now gets the
   16-conjunction form, consumable lane-by-lane without quantifier
   instantiation. *)
let lemma_compress_1_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) :
  Lemma
    (requires TS.compress_1_pre (T.f_repr vec))
    (ensures
       (let r = T.f_compress_1_ vec in
        Spec.Utils.forall16 (fun (j: nat{j < 16}) ->
          TS.i16_to_spec_fe (Seq.index (T.f_repr r) j) ==
          Hacspec_ml_kem.Compress.compress_d
            (TS.i16_to_spec_fe (Seq.index (T.f_repr vec) j)) (mk_usize 1))))
  = reveal_opaque (`%TS.compress_1_lane_post) TS.compress_1_lane_post

let lemma_compress_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (coefficient_bits: i32) (vec: vV) :
  Lemma
    (requires
      (v coefficient_bits == 4 \/ v coefficient_bits == 5 \/
       v coefficient_bits == 10 \/ v coefficient_bits == 11) /\
      TS.compress_pre (T.f_repr vec) coefficient_bits)
    (ensures
       (let r = T.f_compress coefficient_bits vec in
        Spec.Utils.forall16 (fun (j: nat{j < 16}) ->
          TS.i16_to_spec_fe (Seq.index (T.f_repr r) j) ==
          Hacspec_ml_kem.Compress.compress_d
            (TS.i16_to_spec_fe (Seq.index (T.f_repr vec) j))
            (mk_usize (v coefficient_bits)))))
  = reveal_opaque (`%TS.compress_d_lane_post) TS.compress_d_lane_post

(* Decompress chunk lemmas: kept admitted for now (TODO).  Their
   ensures cite `TS.decompress_1_post` / `TS.decompress_ciphertext_coefficient_post`
   directly (which already have the input-bound implication wrapper
   needed to type-check `decompress_d`).  Earlier attempts to spell
   the forall16 inline tripped Z3 on a single specific lane query
   even with `bounded_i16_array` revealed. *)
let lemma_decompress_1_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) :
  Lemma
    (requires TS.decompress_1_pre (T.f_repr vec))
    (ensures
       (let r = T.f_decompress_1_ vec in
        TS.decompress_1_post (T.f_repr vec) (T.f_repr r)))
  = admit ()

let lemma_decompress_ciphertext_coefficient_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (coefficient_bits: i32) (vec: vV) :
  Lemma
    (requires
      (v coefficient_bits == 4 \/ v coefficient_bits == 5 \/
       v coefficient_bits == 10 \/ v coefficient_bits == 11) /\
      TS.decompress_ciphertext_coefficient_pre (T.f_repr vec) coefficient_bits)
    (ensures
       (let r = T.f_decompress_ciphertext_coefficient coefficient_bits vec in
        TS.decompress_ciphertext_coefficient_post
          (T.f_repr vec) coefficient_bits (T.f_repr r)))
  = admit ()

(* ────────────  NTT-layer ops  ────────────
   Hacspec's `ntt_layer_n` at N = 16 takes half-size `len` and a zeta
   slice of length `N / (2·len)`.  The three trait steps instantiate:
     `ntt_layer_1_step`   len = 2, 4 zetas  (zetas_4_)
     `ntt_layer_2_step`   len = 4, 2 zetas  (zetas_2_)
     `ntt_layer_3_step`   len = 8, 1 zeta   (zetas_1_)
   Symmetric layout for the inverse NTT via `ntt_inverse_layer_n`. *)

(* ────────────  Per-branch (concrete-b) layer-1 NTT helpers  ────────────
   Phase 6 follow-up (agent A2).  At call sites in
   `vector/portable.rs::op_ntt_layer_1_step`, the per-branch predicate
   `ntt_layer_1_step_branch_post` selects one of {zeta0..zeta3} via a
   4-way `if b=0 then zeta0 else if b=1 ... else zeta3` ladder.  When the
   wrapper asserts `p_layer_1 b` for symbolic-but-concrete `b ∈ {0,1,2,3}`,
   Z3 case-splits on the ladder for every per-lane FE-algebra fact, which
   blew up at rlimit 800 (single sub-query > 10 min).
   Refactor: 4 per-branch lemmas, each with `b` literal so the if-ladder
   collapses to the right zeta on both pre and post sides.  Each lemma:
   - takes the 2 `ntt_spec` residues for that branch's lane pairs,
   - calls `lemma_butterfly_pair_commute` twice,
   - reveals the opaque `ntt_layer_1_step_branch_post`,
   - concludes the per-branch post for its concrete `b`.
   Mirror set provided for `inv_ntt_layer_1_step_branch_post`. *)

#push-options "--z3rlimit 80 --fuel 0 --ifuel 0"

(* Helper: produce the 4 FE equalities for one zeta-pair group from the
   2 ntt_spec residues.  This isolates the integer/Mont arithmetic
   side, leaving the top-level branch wrapper a trivial composition. *)
private
let lemma_ntt_layer_1_butterfly_to_fe
    (vec result: t_Array i16 (mk_usize 16))
    (z: i16) (i1 j1 i2 j2: nat) :
  Lemma (requires i1 < 16 /\ j1 < 16 /\ i2 < 16 /\ j2 < 16 /\
                  Spec.Utils.ntt_spec vec (v z) i1 j1 result /\
                  Spec.Utils.ntt_spec vec (v z) i2 j2 result)
        (ensures
          mont_i16_to_spec_fe (Seq.index result i1) ==
            P.impl_FieldElement__add
              (mont_i16_to_spec_fe (Seq.index vec i1))
              (P.impl_FieldElement__mul (mont_i16_to_spec_fe z)
                                        (mont_i16_to_spec_fe (Seq.index vec j1))) /\
          mont_i16_to_spec_fe (Seq.index result j1) ==
            P.impl_FieldElement__sub
              (mont_i16_to_spec_fe (Seq.index vec i1))
              (P.impl_FieldElement__mul (mont_i16_to_spec_fe z)
                                        (mont_i16_to_spec_fe (Seq.index vec j1))) /\
          mont_i16_to_spec_fe (Seq.index result i2) ==
            P.impl_FieldElement__add
              (mont_i16_to_spec_fe (Seq.index vec i2))
              (P.impl_FieldElement__mul (mont_i16_to_spec_fe z)
                                        (mont_i16_to_spec_fe (Seq.index vec j2))) /\
          mont_i16_to_spec_fe (Seq.index result j2) ==
            P.impl_FieldElement__sub
              (mont_i16_to_spec_fe (Seq.index vec i2))
              (P.impl_FieldElement__mul (mont_i16_to_spec_fe z)
                                        (mont_i16_to_spec_fe (Seq.index vec j2))))
  = lemma_butterfly_pair_commute vec result z i1 j1;
    lemma_butterfly_pair_commute vec result z i2 j2

(* Mirror for the inverse (Gentleman-Sande) butterfly. *)
private
let lemma_inv_ntt_layer_1_butterfly_to_fe
    (vec result: t_Array i16 (mk_usize 16))
    (z: i16) (i1 j1 i2 j2: nat) :
  Lemma (requires i1 < 16 /\ j1 < 16 /\ i2 < 16 /\ j2 < 16 /\
                  Spec.Utils.inv_ntt_spec vec (v z) i1 j1 result /\
                  Spec.Utils.inv_ntt_spec vec (v z) i2 j2 result)
        (ensures
          mont_i16_to_spec_fe (Seq.index result i1) ==
            P.impl_FieldElement__add
              (mont_i16_to_spec_fe (Seq.index vec i1))
              (mont_i16_to_spec_fe (Seq.index vec j1)) /\
          mont_i16_to_spec_fe (Seq.index result j1) ==
            P.impl_FieldElement__mul
              (mont_i16_to_spec_fe z)
              (P.impl_FieldElement__sub
                (mont_i16_to_spec_fe (Seq.index vec j1))
                (mont_i16_to_spec_fe (Seq.index vec i1))) /\
          mont_i16_to_spec_fe (Seq.index result i2) ==
            P.impl_FieldElement__add
              (mont_i16_to_spec_fe (Seq.index vec i2))
              (mont_i16_to_spec_fe (Seq.index vec j2)) /\
          mont_i16_to_spec_fe (Seq.index result j2) ==
            P.impl_FieldElement__mul
              (mont_i16_to_spec_fe z)
              (P.impl_FieldElement__sub
                (mont_i16_to_spec_fe (Seq.index vec j2))
                (mont_i16_to_spec_fe (Seq.index vec i2))))
  = lemma_inv_butterfly_pair_commute vec result z i1 j1;
    lemma_inv_butterfly_pair_commute vec result z i2 j2

#pop-options

(* ──────  Per-branch concrete-`b` layer-1 step lemmas  ──────
   Each lemma fixes `b` to a literal so the 4-way `if b=0 .. else zeta3`
   ladder in `ntt_layer_1_step_branch_post` collapses to the branch's
   zeta at proof time, in a clean SMT context (no cross-branch fact
   pollution).  Body = reveal the opaque branch_post at this concrete
   application + the already-proven `_butterfly_to_fe` arithmetic.
   `op_ntt_layer_1_step` / `op_inv_ntt_layer_1_step` close the per-vector
   `forall4` post by calling these four and conjoining the opaque atoms. *)

#push-options "--z3rlimit 100 --fuel 0 --ifuel 1"

let lemma_ntt_layer_1_step_branch_0
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.ntt_spec vec (v zeta0) 0 2 result /\
                  Spec.Utils.ntt_spec vec (v zeta0) 1 3 result)
        (ensures TS.ntt_layer_1_step_branch_post 0 vec zeta0 zeta1 zeta2 zeta3 result)
  = reveal_opaque (`%TS.ntt_layer_1_step_branch_post)
                  (TS.ntt_layer_1_step_branch_post 0 vec zeta0 zeta1 zeta2 zeta3 result);
    lemma_ntt_layer_1_butterfly_to_fe vec result zeta0 0 2 1 3

let lemma_ntt_layer_1_step_branch_1
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.ntt_spec vec (v zeta1) 4 6 result /\
                  Spec.Utils.ntt_spec vec (v zeta1) 5 7 result)
        (ensures TS.ntt_layer_1_step_branch_post 1 vec zeta0 zeta1 zeta2 zeta3 result)
  = reveal_opaque (`%TS.ntt_layer_1_step_branch_post)
                  (TS.ntt_layer_1_step_branch_post 1 vec zeta0 zeta1 zeta2 zeta3 result);
    lemma_ntt_layer_1_butterfly_to_fe vec result zeta1 4 6 5 7

let lemma_ntt_layer_1_step_branch_2
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.ntt_spec vec (v zeta2) 8 10 result /\
                  Spec.Utils.ntt_spec vec (v zeta2) 9 11 result)
        (ensures TS.ntt_layer_1_step_branch_post 2 vec zeta0 zeta1 zeta2 zeta3 result)
  = reveal_opaque (`%TS.ntt_layer_1_step_branch_post)
                  (TS.ntt_layer_1_step_branch_post 2 vec zeta0 zeta1 zeta2 zeta3 result);
    lemma_ntt_layer_1_butterfly_to_fe vec result zeta2 8 10 9 11

let lemma_ntt_layer_1_step_branch_3
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.ntt_spec vec (v zeta3) 12 14 result /\
                  Spec.Utils.ntt_spec vec (v zeta3) 13 15 result)
        (ensures TS.ntt_layer_1_step_branch_post 3 vec zeta0 zeta1 zeta2 zeta3 result)
  = reveal_opaque (`%TS.ntt_layer_1_step_branch_post)
                  (TS.ntt_layer_1_step_branch_post 3 vec zeta0 zeta1 zeta2 zeta3 result);
    lemma_ntt_layer_1_butterfly_to_fe vec result zeta3 12 14 13 15

(* Inverse (Gentleman–Sande) mirror — same concrete-`b` collapse. *)

let lemma_inv_ntt_layer_1_step_branch_0
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.inv_ntt_spec vec (v zeta0) 0 2 result /\
                  Spec.Utils.inv_ntt_spec vec (v zeta0) 1 3 result)
        (ensures TS.inv_ntt_layer_1_step_branch_post 0 vec zeta0 zeta1 zeta2 zeta3 result)
  = reveal_opaque (`%TS.inv_ntt_layer_1_step_branch_post)
                  (TS.inv_ntt_layer_1_step_branch_post 0 vec zeta0 zeta1 zeta2 zeta3 result);
    lemma_inv_ntt_layer_1_butterfly_to_fe vec result zeta0 0 2 1 3

let lemma_inv_ntt_layer_1_step_branch_1
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.inv_ntt_spec vec (v zeta1) 4 6 result /\
                  Spec.Utils.inv_ntt_spec vec (v zeta1) 5 7 result)
        (ensures TS.inv_ntt_layer_1_step_branch_post 1 vec zeta0 zeta1 zeta2 zeta3 result)
  = reveal_opaque (`%TS.inv_ntt_layer_1_step_branch_post)
                  (TS.inv_ntt_layer_1_step_branch_post 1 vec zeta0 zeta1 zeta2 zeta3 result);
    lemma_inv_ntt_layer_1_butterfly_to_fe vec result zeta1 4 6 5 7

let lemma_inv_ntt_layer_1_step_branch_2
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.inv_ntt_spec vec (v zeta2) 8 10 result /\
                  Spec.Utils.inv_ntt_spec vec (v zeta2) 9 11 result)
        (ensures TS.inv_ntt_layer_1_step_branch_post 2 vec zeta0 zeta1 zeta2 zeta3 result)
  = reveal_opaque (`%TS.inv_ntt_layer_1_step_branch_post)
                  (TS.inv_ntt_layer_1_step_branch_post 2 vec zeta0 zeta1 zeta2 zeta3 result);
    lemma_inv_ntt_layer_1_butterfly_to_fe vec result zeta2 8 10 9 11

let lemma_inv_ntt_layer_1_step_branch_3
    (vec result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.inv_ntt_spec vec (v zeta3) 12 14 result /\
                  Spec.Utils.inv_ntt_spec vec (v zeta3) 13 15 result)
        (ensures TS.inv_ntt_layer_1_step_branch_post 3 vec zeta0 zeta1 zeta2 zeta3 result)
  = reveal_opaque (`%TS.inv_ntt_layer_1_step_branch_post)
                  (TS.inv_ntt_layer_1_step_branch_post 3 vec zeta0 zeta1 zeta2 zeta3 result);
    lemma_inv_ntt_layer_1_butterfly_to_fe vec result zeta3 12 14 13 15

#pop-options

(* `ntt_layer_1_step_branch_post` is opaque; `op_ntt_layer_1_step` in
   `Libcrux_ml_kem.Vector.Portable` discharges the per-vector `forall4`
   post by calling the four `lemma_ntt_layer_1_step_branch_{0..3}` above
   (each closes one opaque branch atom) after revealing
   `ntt_layer_1_butterfly_post` to expose the `ntt_spec` residues. *)

(* The trait post of `f_ntt_layer_1_step` is exactly this predicate —
   4 groups of 4 FE equalities, wrapped in `Spec.Utils.forall4`.  The
   lemma's conclusion is the post itself, so the body is `= ()`.  Layer 2
   is where the `Seq.lemma_eq_intro` + `N.ntt_layer_n` hacspec
   aggregation will happen. *)
let lemma_ntt_layer_1_step_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires TS.ntt_layer_1_step_pre (T.f_repr vec) zeta0 zeta1 zeta2 zeta3)
    (ensures
       (let r = T.f_ntt_layer_1_step vec zeta0 zeta1 zeta2 zeta3 in
        TS.ntt_layer_1_step_post (T.f_repr vec) zeta0 zeta1 zeta2 zeta3 (T.f_repr r)))
  = ()

let lemma_ntt_layer_2_step_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0 zeta1: i16) :
  Lemma
    (requires TS.ntt_layer_2_step_pre (T.f_repr vec) zeta0 zeta1)
    (ensures
       (let r = T.f_ntt_layer_2_step vec zeta0 zeta1 in
        TS.ntt_layer_2_step_post (T.f_repr vec) zeta0 zeta1 (T.f_repr r)))
  = ()

let lemma_ntt_layer_3_step_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0: i16) :
  Lemma
    (requires TS.ntt_layer_3_step_pre (T.f_repr vec) zeta0)
    (ensures
       (let r = T.f_ntt_layer_3_step vec zeta0 in
        TS.ntt_layer_3_step_post (T.f_repr vec) zeta0 (T.f_repr r)))
  = ()

let lemma_inv_ntt_layer_1_step_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires TS.inv_ntt_layer_1_step_pre (T.f_repr vec) zeta0 zeta1 zeta2 zeta3)
    (ensures
       (let r = T.f_inv_ntt_layer_1_step vec zeta0 zeta1 zeta2 zeta3 in
        TS.inv_ntt_layer_1_step_post (T.f_repr vec) zeta0 zeta1 zeta2 zeta3 (T.f_repr r)))
  = ()

let lemma_inv_ntt_layer_2_step_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0 zeta1: i16) :
  Lemma
    (requires TS.inv_ntt_layer_2_step_pre (T.f_repr vec) zeta0 zeta1)
    (ensures
       (let r = T.f_inv_ntt_layer_2_step vec zeta0 zeta1 in
        TS.inv_ntt_layer_2_step_post (T.f_repr vec) zeta0 zeta1 (T.f_repr r)))
  = ()

let lemma_inv_ntt_layer_3_step_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0: i16) :
  Lemma
    (requires TS.inv_ntt_layer_3_step_pre (T.f_repr vec) zeta0)
    (ensures
       (let r = T.f_inv_ntt_layer_3_step vec zeta0 in
        TS.inv_ntt_layer_3_step_post (T.f_repr vec) zeta0 (T.f_repr r)))
  = ()

(* ────────────  NTT multiply  ────────────
   `ntt_multiply_n` at N = 16 consumes four zetas (N / 4).
   The chunk-commute lemma itself (`lemma_ntt_multiply_chunk_commutes`,
   formerly an assume val here) lives in the Phase-7b region below,
   after the `mont_array_lane` / `zetas_4_lane` unfold helpers it needs.

   Wrapper-side branch lemmas: derive the opaque trait
   `ntt_multiply_branch_post b` from the impl's `ntt_multiply_spec`
   residue pairs at binomials 2b (zeta = +z_b) and 2b+1 (zeta = −z_b).
   Mirrors `lemma_ntt_layer_1_step_branch_*`: one concrete `b` per
   lemma.  The `is_i16b 1664` bound makes `neg_i16` the true negation. *)

let lemma_ntt_multiply_branch_0
    (lhs rhs result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.is_i16b 1664 zeta0 /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (v zeta0) 0 result /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (- (v zeta0)) 1 result)
        (ensures TS.ntt_multiply_branch_post 0 lhs rhs zeta0 zeta1 zeta2 zeta3 result)
  = reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 0 lhs rhs zeta0 zeta1 zeta2 zeta3 result);
    assert (v (Spec.Utils.neg_i16 zeta0) == - (v zeta0));
    lemma_base_case_mult_pair_commute lhs rhs result zeta0 0;
    lemma_base_case_mult_pair_commute lhs rhs result (Spec.Utils.neg_i16 zeta0) 1

let lemma_ntt_multiply_branch_1
    (lhs rhs result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.is_i16b 1664 zeta1 /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (v zeta1) 2 result /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (- (v zeta1)) 3 result)
        (ensures TS.ntt_multiply_branch_post 1 lhs rhs zeta0 zeta1 zeta2 zeta3 result)
  = reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 1 lhs rhs zeta0 zeta1 zeta2 zeta3 result);
    assert (v (Spec.Utils.neg_i16 zeta1) == - (v zeta1));
    lemma_base_case_mult_pair_commute lhs rhs result zeta1 2;
    lemma_base_case_mult_pair_commute lhs rhs result (Spec.Utils.neg_i16 zeta1) 3

let lemma_ntt_multiply_branch_2
    (lhs rhs result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.is_i16b 1664 zeta2 /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (v zeta2) 4 result /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (- (v zeta2)) 5 result)
        (ensures TS.ntt_multiply_branch_post 2 lhs rhs zeta0 zeta1 zeta2 zeta3 result)
  = reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 2 lhs rhs zeta0 zeta1 zeta2 zeta3 result);
    assert (v (Spec.Utils.neg_i16 zeta2) == - (v zeta2));
    lemma_base_case_mult_pair_commute lhs rhs result zeta2 4;
    lemma_base_case_mult_pair_commute lhs rhs result (Spec.Utils.neg_i16 zeta2) 5

let lemma_ntt_multiply_branch_3
    (lhs rhs result: t_Array i16 (mk_usize 16)) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma (requires Spec.Utils.is_i16b 1664 zeta3 /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (v zeta3) 6 result /\
                  Spec.Utils.ntt_multiply_spec lhs rhs (- (v zeta3)) 7 result)
        (ensures TS.ntt_multiply_branch_post 3 lhs rhs zeta0 zeta1 zeta2 zeta3 result)
  = reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 3 lhs rhs zeta0 zeta1 zeta2 zeta3 result);
    assert (v (Spec.Utils.neg_i16 zeta3) == - (v zeta3));
    lemma_base_case_mult_pair_commute lhs rhs result zeta3 6;
    lemma_base_case_mult_pair_commute lhs rhs result (Spec.Utils.neg_i16 zeta3) 7

(*** Phase 7a Tier-1 commute lemmas — Polynomial ***)

(* These lemmas lift per-vector trait posts (already established by the
   trait's `op_*` proofs) to the per-polynomial hacspec equation form,
   so callers in Libcrux_ml_kem.Polynomial.fst can cite
   `Hacspec_ml_kem.Polynomial.<f>` directly in their `ensures`.

   Lift: a `PolynomialRingElement v_Vector` is 16 vectors of 16 i16s
   each.  The hacspec polynomial is `t_Array t_FieldElement 256`.
   Lane `i` (0 ≤ i < 256) maps to the lane `i % 16` of the i-th-divided
   vector, then `i16_to_spec_fe` (plain domain) or `mont_i16_to_spec_fe`
   (Mont domain — for NTT-domain polys). *)

module HP = Hacspec_ml_kem.Polynomial
module V  = Libcrux_ml_kem.Vector

(* Plain-domain poly lift: each i16 lane is reduced mod q via the
   `i16_to_spec_fe` refinement. *)
let to_spec_poly_plain
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p: V.t_PolynomialRingElement vV)
    : t_Array P.t_FieldElement (mk_usize 256)
  = P.createi #P.t_FieldElement (mk_usize 256)
        #(usize -> P.t_FieldElement)
        (fun (j: usize { j <. mk_usize 256 }) ->
          (i16_to_spec_fe
            (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (v j / 16)))
                       (v j % 16))
           <: P.t_FieldElement))

(* Mont-domain poly lift: each i16 lane is interpreted as `a*R mod q`
   with R = 2^16; `mont_i16_to_spec_fe` strips the R factor. *)
let to_spec_poly_mont
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p: V.t_PolynomialRingElement vV)
    : t_Array P.t_FieldElement (mk_usize 256)
  = P.createi #P.t_FieldElement (mk_usize 256)
        #(usize -> P.t_FieldElement)
        (fun (j: usize { j <. mk_usize 256 }) ->
          (mont_i16_to_spec_fe
            (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (v j / 16)))
                       (v j % 16))
           <: P.t_FieldElement))

(* Per-lane index helper for `to_spec_poly_plain`.  Wraps `createi_lemma`
   to accept a `nat` index, mirroring `lane_plain` for the per-vector
   lift.  Useful when peeling per-lane facts from the poly equation. *)
let poly_lane_plain
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p: V.t_PolynomialRingElement vV) (j: nat {j < 256}) :
    Lemma (Seq.index (to_spec_poly_plain p) j
           == i16_to_spec_fe
                (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (j / 16)))
                           (j % 16)))
  = P.createi_lemma #P.t_FieldElement (mk_usize 256)
      #(usize -> P.t_FieldElement)
      (fun (k: usize { k <. mk_usize 256 }) ->
        (i16_to_spec_fe
          (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (v k / 16)))
                     (v k % 16))
         <: P.t_FieldElement))
      (sz j)

(* Same for `to_spec_poly_mont`. *)
let poly_lane_mont
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p: V.t_PolynomialRingElement vV) (j: nat {j < 256}) :
    Lemma (Seq.index (to_spec_poly_mont p) j
           == mont_i16_to_spec_fe
                (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (j / 16)))
                           (j % 16)))
  = P.createi_lemma #P.t_FieldElement (mk_usize 256)
      #(usize -> P.t_FieldElement)
      (fun (k: usize { k <. mk_usize 256 }) ->
        (mont_i16_to_spec_fe
          (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (v k / 16)))
                     (v k % 16))
         <: P.t_FieldElement))
      (sz j)

(* `poly_barrett_reduce` is the identity on FE polynomials.
   Hacspec's body is `createi 256 (i -> FE.new (p.[i].f_val % q))`; since
   `p.[i] : t_FieldElement` is refined by `f_val < FIELD_MODULUS`, the
   inner `% FIELD_MODULUS` is a no-op and `FE.new (p.[i].f_val) == p.[i]`. *)
let lemma_poly_barrett_reduce_id
    (p: t_Array P.t_FieldElement (mk_usize 256)) :
    Lemma (HP.poly_barrett_reduce p == p)
  = let lhs = HP.poly_barrett_reduce p in
    let aux (j: nat) : Lemma (j < 256 ==> Seq.index lhs j == Seq.index p j)
      = if j < 256 then begin
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (i: usize { i <. mk_usize 256 }) ->
              P.impl_FieldElement__new
                ((Seq.index p (v i)).P.f_val %! P.v_FIELD_MODULUS)
              <: P.t_FieldElement)
            (sz j)
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro lhs p

(* ----- E1: poly_barrett_reduce -----
   The trait post for each vector chunk is
     `forall i. v r.[i] % 3329 == v vec.[i] % 3329`,
   i.e. `barrett_reduce_post vec r`.  Pointwise this lifts to
     `i16_to_spec_fe r.[i] == i16_to_spec_fe vec.[i]`
   via `lemma_barrett_fe_commute`.  Composing across the 16 chunks gives
   `to_spec_poly_plain result == to_spec_poly_plain myself`, and
   `poly_barrett_reduce` is the identity (above). *)
let lemma_poly_barrett_reduce_commute
    (#vV: Type0) {| i: T.t_Operations vV |}
    (myself: V.t_PolynomialRingElement vV)
    (result: V.t_PolynomialRingElement vV) :
  Lemma
    (requires
      forall (k: nat). k < 16 ==>
        TS.barrett_reduce_post
          (T.f_repr (Seq.index myself.V.f_coefficients k))
          (T.f_repr (Seq.index result.V.f_coefficients k)))
    (ensures
       to_spec_poly_plain result
         == HP.poly_barrett_reduce (to_spec_poly_plain myself))
  = (* Reveal the lane-form opaque so the per-lane `mod_q_eq` extracted
       from the trait post is in scope below. *)
    reveal_opaque (`%TS.barrett_reduce_lane_post) (TS.barrett_reduce_lane_post);
    let aux (j: nat) : Lemma (j < 256 ==>
        Seq.index (to_spec_poly_plain result) j
          == Seq.index (to_spec_poly_plain myself) j)
      = if j < 256 then begin
          let k : nat = j / 16 in
          let l : nat = j % 16 in
          let m_arr = T.f_repr (Seq.index myself.V.f_coefficients k) in
          let r_arr = T.f_repr (Seq.index result.V.f_coefficients k) in
          (* From requires (instantiated at k): `barrett_reduce_post m_arr r_arr`,
             whose lane-form (revealed above) gives
             `mod_q_eq (v r_arr.[l]) (v m_arr.[l])`. *)
          assert (TS.barrett_reduce_post m_arr r_arr);
          lemma_mod_q_eq_unfold (v (Seq.index r_arr l)) (v (Seq.index m_arr l));
          assert (v (Seq.index r_arr l) % 3329 == v (Seq.index m_arr l) % 3329);
          lemma_barrett_fe_commute (Seq.index m_arr l) (Seq.index r_arr l);
          poly_lane_plain myself j;
          poly_lane_plain result j
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (to_spec_poly_plain result) (to_spec_poly_plain myself);
    lemma_poly_barrett_reduce_id (to_spec_poly_plain myself)

(* ----- E2: add_to_ring_element -----
   The trait post `add_post lhs rhs r` is `forall i. v r.[i] == v lhs.[i] + v rhs.[i]`.
   Per-lane via `lemma_add_fe_commute_plain`:
     `impl_FieldElement__add (i16_to_spec_fe lhs.[i]) (i16_to_spec_fe rhs.[i])
        == i16_to_spec_fe r.[i]`.
   Applied across 256 lanes, we get
     `to_spec_poly_plain result.[j]
        == impl_FieldElement__add (to_spec_poly_plain myself).[j]
                                  (to_spec_poly_plain rhs).[j]`.
   The hacspec `add_to_ring_element lhs rhs .[j]` is structurally the
   same FE.new ((lhs[j].val + rhs[j].val) % q) pattern as
   `impl_FieldElement__add lhs[j] rhs[j]`, so the two are equal lane-wise. *)
let lemma_add_to_ring_element_commute
    (#vV: Type0) {| i: T.t_Operations vV |}
    (myself rhs result: V.t_PolynomialRingElement vV) :
  Lemma
    (requires
      forall (k: nat). k < 16 ==>
        TS.add_post
          (T.f_repr (Seq.index myself.V.f_coefficients k))
          (T.f_repr (Seq.index rhs.V.f_coefficients k))
          (T.f_repr (Seq.index result.V.f_coefficients k)))
    (ensures
       to_spec_poly_plain result
         == HP.add_to_ring_element
              (to_spec_poly_plain myself) (to_spec_poly_plain rhs))
  = let lhs_poly = to_spec_poly_plain myself in
    let rhs_poly = to_spec_poly_plain rhs in
    let r_poly = to_spec_poly_plain result in
    let hp = HP.add_to_ring_element lhs_poly rhs_poly in
    let aux (j: nat) : Lemma (j < 256 ==>
        Seq.index r_poly j == Seq.index hp j)
      = if j < 256 then begin
          let k : nat = j / 16 in
          let l : nat = j % 16 in
          let m_arr = T.f_repr (Seq.index myself.V.f_coefficients k) in
          let s_arr = T.f_repr (Seq.index rhs.V.f_coefficients k) in
          let r_arr = T.f_repr (Seq.index result.V.f_coefficients k) in
          assert (TS.add_post m_arr s_arr r_arr);
          assert (v (Seq.index r_arr l)
                  == v (Seq.index m_arr l) + v (Seq.index s_arr l));
          lemma_add_fe_commute_plain
            (Seq.index m_arr l) (Seq.index s_arr l) (Seq.index r_arr l);
          poly_lane_plain myself j;
          poly_lane_plain rhs j;
          poly_lane_plain result j;
          (* Unfold the `createi` in `HP.add_to_ring_element` at index `j`. *)
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (jj: usize { jj <. mk_usize 256 }) ->
              P.impl_FieldElement__new
                (cast (((cast ((Seq.index lhs_poly (v jj)).P.f_val <: u16) <: u32) +!
                        (cast ((Seq.index rhs_poly (v jj)).P.f_val <: u16) <: u32)
                        <: u32)
                      %! (cast (P.v_FIELD_MODULUS <: u16) <: u32)
                      <: u32)
                <: u16)
              <: P.t_FieldElement)
            (sz j)
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro r_poly hp

(*** Phase 7b — INTT-Mont finalization (agent F) ***)

(* The constants `1441` (the impl's "fused finalization" mont_mul factor)
   and `1353 = R^2 mod q` and `2285 = R mod q` and `169 = R^{-1} mod q`
   appear in the chain that turns `mont_mul(b, 1441)` (post-INTT) back
   into the plain abstract value.  The structural identity, per
   `pq-crystals/kyber/ref/ntt.c` line 106 ("1441 = mont^2/128"), is
   `(128 * 1441) mod q == R^2 mod q`. *)

let lemma_1441_eq_RR_div_128 () :
    Lemma ((128 * 1441) % 3329 == 1353 /\
           (2285 * 2285) % 3329 == 1353 /\
           (2285 * 169) % 3329 == 1)
  = assert_norm ((128 * 1441) % 3329 == 1353);
    assert_norm ((2285 * 2285) % 3329 == 1353);
    assert_norm ((2285 * 169) % 3329 == 1)

(* Per-element INTT-Mont finalization lemma.

   Given the INTT-Mont form precondition on `b` (the impl's
   pre-finalization i16 coefficient lane):
     `(v b * R) mod q == (v b_real_val * 128) mod q`
   (i.e., the impl's `b` represents Mont(spec_butterflies_value); per
   the audit, when the input to `invert_ntt_montgomery` is in Mont
   form, after 7 layers of inverse butterflies the lane stores
   `f_real * 128 * R^{-1} mod q`, equivalently `v b * R ≡ f_real * 128
   (mod q)` where `f_real * 128` is exactly `ntt_inverse_butterflies`
   applied to the spec input.)

   And given the trait's `mont_mul(b, 1441)` post:
     `v r mod q == (v b * 1441 * 169) mod q`
   (mont_mul-by-constant produces the value `b * c * R^{-1} mod q`;
   1441 is the impl's chosen finalization constant since
   `1441 = R^2/128 mod q`).

   Conclusion: `v r ≡ b_real_val (mod q)`, equivalently
   `i16_to_spec_fe r == FE(b_real_val mod q)`.

   This is the per-lane core that lets callers (subtract_reduce et al.)
   chain `i16_to_spec_fe (mont_mul(b, 1441) result) == b_real_fe`
   under the INTT-Mont form precondition. *)

let lemma_intt_mont_form_post (b r: i16) (b_real_val: int) :
    Lemma
    (requires
      (v b * 2285) % 3329 == (b_real_val * 128) % 3329 /\
      v r % 3329 == (v b * 1441 * 169) % 3329)
    (ensures v r % 3329 == b_real_val % 3329)
  = let q : pos = 3329 in
    (* Show (v b * 1441 * 169) ≡ b_real (mod q). *)
    (* Step 1: v b * 1441 * 169 == v b * (1441 * 169) (associativity). *)
    assert (v b * 1441 * 169 == v b * (1441 * 169));
    (* Step 2: 1441 * 169 ≡ ?
       Numerically: 1441 * 169 = 243529; 243529 % 3329 = 243529 - 73*3329
                                = 243529 - 243017 = 512.
       So 1441 * 169 ≡ 512 (mod q). *)
    assert_norm ((1441 * 169) % 3329 == 512);
    L.lemma_mod_mul_distr_r (v b) (1441 * 169) q;
    (* Now: (v b * 1441 * 169) % q == (v b * 512) % q. *)
    assert ((v b * 1441 * 169) % q == (v b * 512) % q);
    (* Step 3: 512 ≡ ? in terms of R = 2285 and 169.
       Note: 512 = (2 * 2285 * 169) / 169... let me think differently.
       Goal: relate (v b * 512) % q to (b_real * 128 * 169) % q via
       the precondition (v b * 2285) % q == (b_real * 128) % q.
       Multiply both sides by 169:
         (v b * 2285 * 169) % q == (b_real * 128 * 169) % q.
       And 2285 * 169 ≡ 1 (mod q) (R * R^{-1}).
       So (v b * 1) % q == (b_real * 128 * 169) % q,
          i.e. v b % q == (b_real * 128 * 169) % q. *)
    L.lemma_mod_mul_distr_l (v b * 2285) 169 q;
    L.lemma_mod_mul_distr_l (b_real_val * 128) 169 q;
    assert ((v b * 2285 * 169) % q == ((v b * 2285) % q * 169) % q);
    assert ((b_real_val * 128 * 169) % q == ((b_real_val * 128) % q * 169) % q);
    assert ((v b * 2285 * 169) % q == (b_real_val * 128 * 169) % q);
    (* And 2285 * 169 == 386165; 386165 % 3329 == 1. *)
    assert_norm ((2285 * 169) % 3329 == 1);
    (* So (v b * 2285 * 169) % q == (v b * 1) % q == v b % q. *)
    assert (v b * 2285 * 169 == v b * (2285 * 169));
    L.lemma_mod_mul_distr_r (v b) (2285 * 169) q;
    assert ((v b * (2285 * 169)) % q == (v b * ((2285 * 169) % q)) % q);
    assert ((v b * 1) % q == v b % q);
    (* So v b % q == (b_real * 128 * 169) % q. *)
    assert (v b % q == (b_real_val * 128 * 169) % q);
    (* Now relate this to (v b * 512) % q.
       128 * 169 == 21632; 21632 % 3329 == 21632 - 6*3329 = 21632 - 19974 = 1658.
       Hmm — that doesn't match 512. Let me try a different chain.
       We have:
         (v b * 1441 * 169) % q == (v b * 512) % q                  -- (A)
         v b % q == (b_real * 128 * 169) % q                         -- (B)
       Want: (v b * 512) % q == b_real % q.
       From (B), multiply by 512: (v b * 512) % q == (b_real * 128 * 169 * 512) % q.
       Compute 128 * 169 * 512 mod q:
         128 * 169 = 21632; 21632 mod 3329 = 1658.
         1658 * 512 = 848896; 848896 mod 3329: 848896 / 3329 ≈ 254.99; 254 * 3329 = 845566; 848896 - 845566 = 3330; 3330 mod 3329 = 1.
       So (b_real * 128 * 169 * 512) ≡ b_real (mod q). *)
    L.lemma_mod_mul_distr_l (v b) 512 q;
    L.lemma_mod_mul_distr_l (b_real_val * 128 * 169) 512 q;
    assert ((v b * 512) % q == ((v b % q) * 512) % q);
    assert (((v b % q) * 512) % q == (((b_real_val * 128 * 169) % q) * 512) % q);
    assert ((((b_real_val * 128 * 169) % q) * 512) % q == (b_real_val * 128 * 169 * 512) % q);
    assert_norm ((128 * 169 * 512) % 3329 == 1);
    assert (b_real_val * 128 * 169 * 512 == b_real_val * (128 * 169 * 512));
    L.lemma_mod_mul_distr_r b_real_val (128 * 169 * 512) q;
    assert ((b_real_val * (128 * 169 * 512)) % q == (b_real_val * ((128 * 169 * 512) % q)) % q);
    assert ((b_real_val * 1) % q == b_real_val % q)

(* INTT-Mont form opaque per-lane predicate.

   Input lane: the i16 stored in the impl's polynomial after 7 layers
   of inverse butterflies (`invert_ntt_montgomery`'s output).
   `hacspec_butterflies_lane`: the corresponding spec FE value, i.e.,
   the lane of `Hacspec_ml_kem.Invert_ntt.ntt_inverse_butterflies` applied
   to the spec lift of the function's input.

   Body: `(v input_lane * R) mod q == (val(hacspec) * 128) mod q`,
   capturing that the impl coefficient is in `f_real · 128 · R^{-1}`
   form (Mont-domain post-INTT-without-finalization).

   Marked opaque so callers see only the structural per-lane predicate;
   the raw mod arithmetic stays hidden. *)

[@@ "opaque_to_smt"]
let intt_mont_form_lane
    (input_lane: i16) (hacspec_butterflies_lane: P.t_FieldElement) : prop =
  (v input_lane * 2285) % 3329 == (v hacspec_butterflies_lane.P.f_val * 128) % 3329

(* Reveal-on-demand helper for the per-lane predicate.  No SMTPat —
   call explicitly inside Tier-2 lemmas that need the unfolded form
   (style §3.3 of proof-style-guide.md). *)
let lemma_intt_mont_form_lane_reveal
    (input_lane: i16) (hacspec_butterflies_lane: P.t_FieldElement) :
    Lemma (requires intt_mont_form_lane input_lane hacspec_butterflies_lane)
          (ensures
            (v input_lane * 2285) % 3329 ==
            (v hacspec_butterflies_lane.P.f_val * 128) % 3329)
  = reveal_opaque (`%intt_mont_form_lane)
                  (intt_mont_form_lane input_lane hacspec_butterflies_lane)

(* Intro direction: build `intt_mont_form_lane` from the unfolded body. *)
let lemma_intt_mont_form_lane_intro
    (input_lane: i16) (hacspec_butterflies_lane: P.t_FieldElement) :
    Lemma (requires
            (v input_lane * 2285) % 3329 ==
            (v hacspec_butterflies_lane.P.f_val * 128) % 3329)
          (ensures intt_mont_form_lane input_lane hacspec_butterflies_lane)
  = reveal_opaque (`%intt_mont_form_lane)
                  (intt_mont_form_lane input_lane hacspec_butterflies_lane)

(* Per-chunk wrap (matches `forall16` pattern in trait posts).
   The body inlines to a 16-conjunction of opaque atoms, so callers
   see only the structural iteration. *)
let intt_mont_form_chunk
    (input_chunk: t_Array i16 (mk_usize 16))
    (hacspec_butterflies_chunk: t_Array P.t_FieldElement (mk_usize 16)) : prop =
  Spec.Utils.forall16 (fun (i: nat {i < 16}) ->
    intt_mont_form_lane (Seq.index input_chunk i)
                        (Seq.index hacspec_butterflies_chunk i))

(* Per-lane consumer lemma: given the INTT-Mont form on lane `b` and the
   trait's `montgomery_multiply_by_constant(b, 1441)` post (delivering
   `v r % q == (v b * 1441 * 169) % q`), conclude that `r` carries the
   plain abstract value `b_real`.

   This is the per-element bridge that callers like `subtract_reduce`,
   `add_error_reduce`, and `add_message_error_reduce` need: after the
   `mont_mul(coefficient, 1441)` and Barrett reduction, the lane is back
   to plain `f_real` form. *)

let lemma_intt_mont_finalize_fe
    (b r: i16) (b_real: P.t_FieldElement) :
    Lemma (requires
            intt_mont_form_lane b b_real /\
            v r % 3329 == (v b * 1441 * 169) % 3329)
          (ensures i16_to_spec_fe r == b_real)
  = reveal_opaque (`%intt_mont_form_lane) (intt_mont_form_lane b b_real);
    lemma_intt_mont_form_post b r (v b_real.P.f_val);
    (* Now `v r % 3329 == v b_real.f_val % 3329`.  Since `b_real`'s
       refinement gives `v b_real.f_val < 3329`, that's `v b_real.f_val`. *)
    assert (v r % 3329 == v b_real.P.f_val % 3329);
    assert (v b_real.P.f_val % 3329 == v b_real.P.f_val);
    (* `i16_to_spec_fe r` has `f_val = (v r) % 3329`. *)
    ()

(* ----- Per-lane bridge for `subtract_reduce`-family fused finalize -----
   Given the trait posts of `mont_mul(b, 1441)` (giving `cnf`), `sub`
   (giving `diff = myself - cnf`), and `barrett` (giving `red ≡ diff
   mod q`), conclude:
     i16_to_spec_fe red == myself_lift `sub_FE` (mont_lift_b `mul_FE` 1441)
   where `1441 = R²/128 mod q` is the impl's mont-mul-by constant.
   This is consumed in poly-level form by `subtract_reduce`,
   `add_message_error_reduce`, `add_error_reduce` after composing 256
   lanes via `Seq.lemma_eq_intro`. *)

let fe_1441 : P.t_FieldElement = P.impl_FieldElement__new (mk_u16 1441)

(* Opaque per-vector wrapper for the per-lane FE finalize relation.  Bundles
   16 per-lane equations into a single opaque atom; without opacity the
   inner forall pollutes loop-invariant subtyping checks (Z3 instantiates
   at every (j, k) pair, blowing rlimit). *)
[@@ "opaque_to_smt"]
let subtract_reduce_finalize_chunk
    (myself_chunk b_chunk _b_chunk: t_Array i16 (mk_usize 16)) : prop =
  forall (k: nat). k < 16 ==>
    i16_to_spec_fe (Seq.index b_chunk k) ==
      P.impl_FieldElement__sub
        (i16_to_spec_fe (Seq.index myself_chunk k))
        (P.impl_FieldElement__mul (mont_i16_to_spec_fe (Seq.index _b_chunk k)) fe_1441)

let lemma_subtract_reduce_finalize_chunk_reveal
    (myself_chunk b_chunk _b_chunk: t_Array i16 (mk_usize 16)) :
    Lemma (requires subtract_reduce_finalize_chunk myself_chunk b_chunk _b_chunk)
          (ensures
            forall (k: nat). k < 16 ==>
              i16_to_spec_fe (Seq.index b_chunk k) ==
                P.impl_FieldElement__sub
                  (i16_to_spec_fe (Seq.index myself_chunk k))
                  (P.impl_FieldElement__mul (mont_i16_to_spec_fe (Seq.index _b_chunk k)) fe_1441))
  = reveal_opaque (`%subtract_reduce_finalize_chunk)
                  (subtract_reduce_finalize_chunk myself_chunk b_chunk _b_chunk)

let lemma_subtract_reduce_finalize_chunk_intro
    (myself_chunk b_chunk _b_chunk: t_Array i16 (mk_usize 16)) :
    Lemma (requires
            forall (k: nat). k < 16 ==>
              i16_to_spec_fe (Seq.index b_chunk k) ==
                P.impl_FieldElement__sub
                  (i16_to_spec_fe (Seq.index myself_chunk k))
                  (P.impl_FieldElement__mul (mont_i16_to_spec_fe (Seq.index _b_chunk k)) fe_1441))
          (ensures subtract_reduce_finalize_chunk myself_chunk b_chunk _b_chunk)
  = reveal_opaque (`%subtract_reduce_finalize_chunk)
                  (subtract_reduce_finalize_chunk myself_chunk b_chunk _b_chunk)

let lemma_subtract_reduce_finalize_fe
    (myself b cnf diff red: i16) :
    Lemma
      (requires
        v cnf % 3329 == (v b * 1441 * 169) % 3329 /\
        v diff == v myself - v cnf /\
        v red % 3329 == v diff % 3329)
      (ensures
        i16_to_spec_fe red ==
          P.impl_FieldElement__sub
            (i16_to_spec_fe myself)
            (P.impl_FieldElement__mul (mont_i16_to_spec_fe b) fe_1441))
  = let q : pos = 3329 in
    let myself_lift = i16_to_spec_fe myself in
    let b_lift = mont_i16_to_spec_fe b in
    let mul = P.impl_FieldElement__mul b_lift fe_1441 in
    let res = P.impl_FieldElement__sub myself_lift mul in
    (* Step 1: v mul.f_val == (v b * 1441 * 169) % q *)
    lemma_impl_mul_v_val b_lift fe_1441;
    assert (v fe_1441.P.f_val == 1441);
    assert (v b_lift.P.f_val == (v b * 169) % q);
    L.lemma_mod_mul_distr_l (v b * 169) 1441 q;
    assert (v mul.P.f_val == (v b * 169 * 1441) % q);
    assert (v b * 169 * 1441 == v b * 1441 * 169);
    assert (v mul.P.f_val == v cnf % q);
    (* Step 2: v res.f_val == (v myself - v cnf) % q *)
    lemma_impl_sub_v_val myself_lift mul;
    L.lemma_mod_add_distr (v myself % q) (- v cnf) q;
    L.lemma_mod_add_distr (- v cnf) (v myself) q;
    assert (v res.P.f_val == (v myself - v cnf) % q);
    (* Step 3: that's v diff % q == v red % q == (i16_to_spec_fe red).f_val *)
    ()

(* Per-iteration helper for subtract_reduce body proof.  Takes the three
   trait posts (mont_mul, sub, barrett) at the i16-array (chunk) level and
   produces the opaque chunk-level finalize predicate.  Encapsulates the
   per-lane Classical.forall_intro + chunk-intro into one call so the
   subtract_reduce loop body's Z3 context stays small. *)
let lemma_subtract_reduce_iter
    (myself_chunk b_chunk_in cnf_chunk diff_chunk red_chunk: t_Array i16 (mk_usize 16)) :
    Lemma
      (requires
        TS.montgomery_multiply_by_constant_post b_chunk_in (mk_i16 1441) cnf_chunk /\
        TS.sub_post myself_chunk cnf_chunk diff_chunk /\
        TS.barrett_reduce_post diff_chunk red_chunk)
      (ensures
        subtract_reduce_finalize_chunk myself_chunk red_chunk b_chunk_in)
  = (* Reveal the lane-form opaque atoms so the per-lane `mod_q_eq` facts
       extracted from the trait posts are in scope below. *)
    reveal_opaque (`%TS.montgomery_multiply_lane_post) (TS.montgomery_multiply_lane_post);
    reveal_opaque (`%TS.barrett_reduce_lane_post) (TS.barrett_reduce_lane_post);
    let aux (k: nat) : Lemma (k < 16 ==>
        i16_to_spec_fe (Seq.index red_chunk k) ==
          P.impl_FieldElement__sub
            (i16_to_spec_fe (Seq.index myself_chunk k))
            (P.impl_FieldElement__mul (mont_i16_to_spec_fe (Seq.index b_chunk_in k)) fe_1441))
      = if k < 16 then begin
          (* Unfold mod_q_eq from the trait posts into raw `% 3329` so the
             inner finalize lemma's preconditions match. *)
          lemma_mod_q_eq_unfold
            (v (Seq.index cnf_chunk k))
            (v (Seq.index b_chunk_in k) * v (mk_i16 1441) * 169);
          lemma_mod_q_eq_unfold
            (v (Seq.index red_chunk k))
            (v (Seq.index diff_chunk k));
          lemma_subtract_reduce_finalize_fe
            (Seq.index myself_chunk k)
            (Seq.index b_chunk_in k)
            (Seq.index cnf_chunk k)
            (Seq.index diff_chunk k)
            (Seq.index red_chunk k)
        end
    in
    Classical.forall_intro aux;
    lemma_subtract_reduce_finalize_chunk_intro myself_chunk red_chunk b_chunk_in

(* Helper form of HP.subtract_reduce that avoids the heavy createi-with-
   inline-cast-arithmetic body.  Each lane is `impl_FE__sub a[j] b[j]`
   directly.  This and HP.subtract_reduce produce the same array (the
   bodies unfold equally), captured by `lemma_subtract_reduce_eq_helper`
   below.  Working through the helper bypasses Z3's struggle with
   createi_lemma on HP.subtract_reduce's body. *)
let subtract_reduce_helper
    (a b: t_Array P.t_FieldElement (mk_usize 256))
    : t_Array P.t_FieldElement (mk_usize 256) =
  P.createi #P.t_FieldElement (mk_usize 256)
    #(usize -> P.t_FieldElement)
    (fun (j: usize {j <. mk_usize 256}) ->
      P.impl_FieldElement__sub (Seq.index a (v j)) (Seq.index b (v j)))

#push-options "--z3rlimit 200 --ext context_pruning"
let lemma_subtract_reduce_eq_helper
    (a b: t_Array P.t_FieldElement (mk_usize 256)) :
    Lemma (HP.subtract_reduce a b == subtract_reduce_helper a b)
  = let lhs = HP.subtract_reduce a b in
    let rhs = subtract_reduce_helper a b in
    let aux (j: nat) : Lemma (j < 256 ==> Seq.index lhs j == Seq.index rhs j)
      = if j < 256 then begin
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (jj: usize {jj <. mk_usize 256}) ->
              P.impl_FieldElement__sub (Seq.index a (v jj)) (Seq.index b (v jj)))
            (sz j);
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (jj: usize { jj <. mk_usize 256 }) ->
              P.impl_FieldElement__new
                (cast ((((cast ((Seq.index a (v jj)).P.f_val <: u16) <: u32) +!
                         (cast P.v_FIELD_MODULUS <: u32)
                         <: u32) -!
                        (cast ((Seq.index b (v jj)).P.f_val <: u16) <: u32)
                        <: u32) %!
                       (cast P.v_FIELD_MODULUS <: u32)
                       <: u32)
                 <: u16)
              <: P.t_FieldElement)
            (sz j)
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro lhs rhs
#pop-options

(* Per-poly commute lemma for subtract_reduce.  Produces the polynomial-
   level equation in helper-form.  Pair with `lemma_subtract_reduce_eq_helper`
   to chain to the `HP.subtract_reduce` form when needed. *)
#push-options "--z3rlimit 200 --ext context_pruning --split_queries always"
let lemma_subtract_reduce_commute
    (#vV: Type0) {| i: T.t_Operations vV |}
    (myself b_input b_post: V.t_PolynomialRingElement vV) :
    Lemma
      (requires
        forall (k: nat). k < 16 ==>
          subtract_reduce_finalize_chunk
            (T.f_repr (Seq.index myself.V.f_coefficients k))
            (T.f_repr (Seq.index b_post.V.f_coefficients k))
            (T.f_repr (Seq.index b_input.V.f_coefficients k)))
      (ensures
        to_spec_poly_plain b_post ==
        subtract_reduce_helper
          (to_spec_poly_plain myself)
          (P.createi #P.t_FieldElement (mk_usize 256)
             #(usize -> P.t_FieldElement)
             (fun (j: usize {j <. mk_usize 256}) ->
               P.impl_FieldElement__mul
                 (Seq.index (to_spec_poly_mont b_input) (v j))
                 fe_1441)))
  = let myself_lift = to_spec_poly_plain myself in
    let mont_b = to_spec_poly_mont b_input in
    let scaled_b : t_Array P.t_FieldElement (mk_usize 256) =
      P.createi #P.t_FieldElement (mk_usize 256)
        #(usize -> P.t_FieldElement)
        (fun (j: usize {j <. mk_usize 256}) ->
          P.impl_FieldElement__mul (Seq.index mont_b (v j)) fe_1441) in
    let r_poly = to_spec_poly_plain b_post in
    let hp = subtract_reduce_helper myself_lift scaled_b in
    let aux (j: nat) : Lemma (j < 256 ==>
        Seq.index r_poly j == Seq.index hp j)
      = if j < 256 then begin
          let chunk : nat = j / 16 in
          let lane  : nat = j % 16 in
          let m_arr = T.f_repr (Seq.index myself.V.f_coefficients chunk) in
          let bin_arr = T.f_repr (Seq.index b_input.V.f_coefficients chunk) in
          let bp_arr = T.f_repr (Seq.index b_post.V.f_coefficients chunk) in
          lemma_subtract_reduce_finalize_chunk_reveal m_arr bp_arr bin_arr;
          poly_lane_plain b_post j;
          poly_lane_plain myself j;
          poly_lane_mont b_input j;
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (jj: usize { jj <. mk_usize 256 }) ->
              P.impl_FieldElement__mul (Seq.index mont_b (v jj)) fe_1441)
            (sz j);
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (jj: usize { jj <. mk_usize 256 }) ->
              P.impl_FieldElement__sub (Seq.index myself_lift (v jj)) (Seq.index scaled_b (v jj)))
            (sz j)
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro r_poly hp
#pop-options

(* ===================================================================== *)
(* add_error_reduce family — mirror of subtract_reduce (SUB → ADD).        *)
(*                                                                         *)
(* `add_error_reduce(myself, error)` computes, lane-wise:                  *)
(*   red = barrett(mont_mul(myself, 1441) + error)                          *)
(* where `myself` is the MONT-scaled operand (·R⁻¹ form, gets the fused    *)
(* `mont_mul(_, 1441)` finalize) and `error` is the PLAIN operand.  This   *)
(* is the OPPOSITE operand-role from subtract_reduce (where `b` is scaled  *)
(* and `myself` is plain).  Everything else mirrors the subtract_reduce    *)
(* family exactly, swapping the FE-`sub` for FE-`add`.                      *)
(* ===================================================================== *)

(* Opaque per-vector wrapper for the per-lane FE finalize relation.  Bundles
   16 per-lane equations into a single opaque atom (mirror of
   subtract_reduce_finalize_chunk; SUB → ADD; the scaled operand is
   `myself` and the plain operand is `error`). *)
[@@ "opaque_to_smt"]
let add_error_reduce_finalize_chunk
    (myself_chunk red_chunk error_chunk: t_Array i16 (mk_usize 16)) : prop =
  forall (k: nat). k < 16 ==>
    i16_to_spec_fe (Seq.index red_chunk k) ==
      P.impl_FieldElement__add
        (P.impl_FieldElement__mul (mont_i16_to_spec_fe (Seq.index myself_chunk k)) fe_1441)
        (i16_to_spec_fe (Seq.index error_chunk k))

let lemma_add_error_reduce_finalize_chunk_reveal
    (myself_chunk red_chunk error_chunk: t_Array i16 (mk_usize 16)) :
    Lemma (requires add_error_reduce_finalize_chunk myself_chunk red_chunk error_chunk)
          (ensures
            forall (k: nat). k < 16 ==>
              i16_to_spec_fe (Seq.index red_chunk k) ==
                P.impl_FieldElement__add
                  (P.impl_FieldElement__mul (mont_i16_to_spec_fe (Seq.index myself_chunk k)) fe_1441)
                  (i16_to_spec_fe (Seq.index error_chunk k)))
  = reveal_opaque (`%add_error_reduce_finalize_chunk)
                  (add_error_reduce_finalize_chunk myself_chunk red_chunk error_chunk)

let lemma_add_error_reduce_finalize_chunk_intro
    (myself_chunk red_chunk error_chunk: t_Array i16 (mk_usize 16)) :
    Lemma (requires
            forall (k: nat). k < 16 ==>
              i16_to_spec_fe (Seq.index red_chunk k) ==
                P.impl_FieldElement__add
                  (P.impl_FieldElement__mul (mont_i16_to_spec_fe (Seq.index myself_chunk k)) fe_1441)
                  (i16_to_spec_fe (Seq.index error_chunk k)))
          (ensures add_error_reduce_finalize_chunk myself_chunk red_chunk error_chunk)
  = reveal_opaque (`%add_error_reduce_finalize_chunk)
                  (add_error_reduce_finalize_chunk myself_chunk red_chunk error_chunk)

let lemma_add_error_reduce_finalize_fe
    (myself error cnf sum red: i16) :
    Lemma
      (requires
        v cnf % 3329 == (v myself * 1441 * 169) % 3329 /\
        v sum == v cnf + v error /\
        v red % 3329 == v sum % 3329)
      (ensures
        i16_to_spec_fe red ==
          P.impl_FieldElement__add
            (P.impl_FieldElement__mul (mont_i16_to_spec_fe myself) fe_1441)
            (i16_to_spec_fe error))
  = let q : pos = 3329 in
    let myself_lift = mont_i16_to_spec_fe myself in
    let error_lift = i16_to_spec_fe error in
    let mul = P.impl_FieldElement__mul myself_lift fe_1441 in
    let res = P.impl_FieldElement__add mul error_lift in
    (* Step 1: v mul.f_val == (v myself * 1441 * 169) % q == v cnf % q *)
    lemma_impl_mul_v_val myself_lift fe_1441;
    assert (v fe_1441.P.f_val == 1441);
    assert (v myself_lift.P.f_val == (v myself * 169) % q);
    L.lemma_mod_mul_distr_l (v myself * 169) 1441 q;
    assert (v mul.P.f_val == (v myself * 169 * 1441) % q);
    assert (v myself * 169 * 1441 == v myself * 1441 * 169);
    assert (v mul.P.f_val == v cnf % q);
    (* Step 2: v res.f_val == (v cnf + v error) % q == v sum % q *)
    lemma_impl_add_v_val mul error_lift;
    assert (v error_lift.P.f_val == v error % q);
    L.lemma_mod_add_distr (v error) (v cnf) q;
    L.lemma_mod_add_distr (v cnf) (v error) q;
    assert (v res.P.f_val == (v cnf + v error) % q);
    (* Step 3: v sum == v cnf + v error, and v red % q == v sum % q *)
    ()

(* Per-iteration helper for add_error_reduce body proof.  Takes the three
   trait posts (mont_mul, add, barrett) at the i16-array (chunk) level and
   produces the opaque chunk-level finalize predicate.  Mirror of
   lemma_subtract_reduce_iter (SUB → ADD; uses `add_post` instead of
   `sub_post`; the scaled operand is `myself`, the plain operand is
   `error`). *)
let lemma_add_error_reduce_iter
    (myself_chunk error_chunk cnf_chunk sum_chunk red_chunk: t_Array i16 (mk_usize 16)) :
    Lemma
      (requires
        TS.montgomery_multiply_by_constant_post myself_chunk (mk_i16 1441) cnf_chunk /\
        TS.add_post cnf_chunk error_chunk sum_chunk /\
        TS.barrett_reduce_post sum_chunk red_chunk)
      (ensures
        add_error_reduce_finalize_chunk myself_chunk red_chunk error_chunk)
  = reveal_opaque (`%TS.montgomery_multiply_lane_post) (TS.montgomery_multiply_lane_post);
    reveal_opaque (`%TS.barrett_reduce_lane_post) (TS.barrett_reduce_lane_post);
    let aux (k: nat) : Lemma (k < 16 ==>
        i16_to_spec_fe (Seq.index red_chunk k) ==
          P.impl_FieldElement__add
            (P.impl_FieldElement__mul (mont_i16_to_spec_fe (Seq.index myself_chunk k)) fe_1441)
            (i16_to_spec_fe (Seq.index error_chunk k)))
      = if k < 16 then begin
          (* mont_mul post: v cnf % 3329 == (v myself * 1441 * 169) % 3329. *)
          lemma_mod_q_eq_unfold
            (v (Seq.index cnf_chunk k))
            (v (Seq.index myself_chunk k) * v (mk_i16 1441) * 169);
          (* add post: v sum == v cnf + v error (a plain equality, not mod_q). *)
          (* barrett post: v red % 3329 == v sum % 3329. *)
          lemma_mod_q_eq_unfold
            (v (Seq.index red_chunk k))
            (v (Seq.index sum_chunk k));
          lemma_add_error_reduce_finalize_fe
            (Seq.index myself_chunk k)
            (Seq.index error_chunk k)
            (Seq.index cnf_chunk k)
            (Seq.index sum_chunk k)
            (Seq.index red_chunk k)
        end
    in
    Classical.forall_intro aux;
    lemma_add_error_reduce_finalize_chunk_intro myself_chunk red_chunk error_chunk

(* Helper form of HP.add_error_reduce that avoids the heavy createi-with-
   inline-cast-arithmetic body.  Each lane is `impl_FE__add a[j] b[j]`
   directly.  Mirror of subtract_reduce_helper. *)
let add_error_reduce_helper
    (a b: t_Array P.t_FieldElement (mk_usize 256))
    : t_Array P.t_FieldElement (mk_usize 256) =
  P.createi #P.t_FieldElement (mk_usize 256)
    #(usize -> P.t_FieldElement)
    (fun (j: usize {j <. mk_usize 256}) ->
      P.impl_FieldElement__add (Seq.index a (v j)) (Seq.index b (v j)))

#push-options "--z3rlimit 200 --ext context_pruning"
let lemma_add_error_reduce_eq_helper
    (a b: t_Array P.t_FieldElement (mk_usize 256)) :
    Lemma (HP.add_error_reduce a b == add_error_reduce_helper a b)
  = let lhs = HP.add_error_reduce a b in
    let rhs = add_error_reduce_helper a b in
    let aux (j: nat) : Lemma (j < 256 ==> Seq.index lhs j == Seq.index rhs j)
      = if j < 256 then begin
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (jj: usize {jj <. mk_usize 256}) ->
              P.impl_FieldElement__add (Seq.index a (v jj)) (Seq.index b (v jj)))
            (sz j);
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (jj: usize { jj <. mk_usize 256 }) ->
              P.impl_FieldElement__new
                (cast (((cast ((Seq.index a (v jj)).P.f_val <: u16) <: u32) +!
                        (cast ((Seq.index b (v jj)).P.f_val <: u16) <: u32)
                        <: u32) %!
                       (cast P.v_FIELD_MODULUS <: u32)
                       <: u32)
                 <: u16)
              <: P.t_FieldElement)
            (sz j)
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro lhs rhs
#pop-options

(* Per-poly commute lemma for add_error_reduce.  Produces the polynomial-
   level equation in helper-form.  Mirror of lemma_subtract_reduce_commute:
   the SCALED operand is `myself` (lifted via to_spec_poly_mont then scaled
   by fe_1441) and the PLAIN operand is `error` (lifted via to_spec_poly_plain). *)
#push-options "--z3rlimit 200 --ext context_pruning --split_queries always"
let lemma_add_error_reduce_commute
    (#vV: Type0) {| i: T.t_Operations vV |}
    (myself_input error myself_post: V.t_PolynomialRingElement vV) :
    Lemma
      (requires
        forall (k: nat). k < 16 ==>
          add_error_reduce_finalize_chunk
            (T.f_repr (Seq.index myself_input.V.f_coefficients k))
            (T.f_repr (Seq.index myself_post.V.f_coefficients k))
            (T.f_repr (Seq.index error.V.f_coefficients k)))
      (ensures
        to_spec_poly_plain myself_post ==
        add_error_reduce_helper
          (P.createi #P.t_FieldElement (mk_usize 256)
             #(usize -> P.t_FieldElement)
             (fun (j: usize {j <. mk_usize 256}) ->
               P.impl_FieldElement__mul
                 (Seq.index (to_spec_poly_mont myself_input) (v j))
                 fe_1441))
          (to_spec_poly_plain error))
  = let error_lift = to_spec_poly_plain error in
    let mont_m = to_spec_poly_mont myself_input in
    let scaled_m : t_Array P.t_FieldElement (mk_usize 256) =
      P.createi #P.t_FieldElement (mk_usize 256)
        #(usize -> P.t_FieldElement)
        (fun (j: usize {j <. mk_usize 256}) ->
          P.impl_FieldElement__mul (Seq.index mont_m (v j)) fe_1441) in
    let r_poly = to_spec_poly_plain myself_post in
    let hp = add_error_reduce_helper scaled_m error_lift in
    let aux (j: nat) : Lemma (j < 256 ==>
        Seq.index r_poly j == Seq.index hp j)
      = if j < 256 then begin
          let chunk : nat = j / 16 in
          let lane  : nat = j % 16 in
          let m_arr = T.f_repr (Seq.index myself_input.V.f_coefficients chunk) in
          let e_arr = T.f_repr (Seq.index error.V.f_coefficients chunk) in
          let mp_arr = T.f_repr (Seq.index myself_post.V.f_coefficients chunk) in
          lemma_add_error_reduce_finalize_chunk_reveal m_arr mp_arr e_arr;
          poly_lane_plain myself_post j;
          poly_lane_plain error j;
          poly_lane_mont myself_input j;
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (jj: usize { jj <. mk_usize 256 }) ->
              P.impl_FieldElement__mul (Seq.index mont_m (v jj)) fe_1441)
            (sz j);
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (jj: usize { jj <. mk_usize 256 }) ->
              P.impl_FieldElement__add (Seq.index scaled_m (v jj)) (Seq.index error_lift (v jj)))
            (sz j)
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro r_poly hp
#pop-options

(* (lemma_add_error_reduce_scaled_eq is defined at end of file, after
   lemma_subtract_reduce_scaled_eq which it mirrors.) *)

(*** Phase 7a Step 7 — to_standard_domain finalization (matrix-mul track) ***)

(* The standard-domain track is the post-matrix-multiply analogue of the
   INTT-Mont finalize.  After `t_as_ntt[i] = add_to_ring_element ∘
   ntt_multiply` (over k accumulators), each lane stores `α · R⁻¹ mod q`
   where α is the FIPS-203 plain spec value.  `to_standard_domain` =
   `mont_mul(_, 1353)` with `1353 = R² mod q` produces a single `· R²`
   factor: lane stores `(α · R⁻¹) · 1353 · R⁻¹ = α · R² · R⁻² = α (mod q)`.

   Note `1353 = R² mod q ≠ 1441 = R²/128 mod q` — the distinction is the
   missing `· 128⁻¹` factor that ONLY applies to the INTT track (where
   `invert_ntt_montgomery` skips the FIPS-203 `· 128⁻¹` finalize).

   The keystone numeric identity: `1353 · 169 ≡ R · R⁻¹ · R ≡ R ≡ 2285 (mod q)`.
   So `mont_mul(myself, 1353) ≡ v myself · 2285 ≡ α · R⁻¹ · R ≡ α (mod q)` ✓. *)

let lemma_1353_eq_R () :
    Lemma ((1353 * 169) % 3329 == 2285 /\
           (2285 * 169) % 3329 == 1)
  = assert_norm ((1353 * 169) % 3329 == 2285);
    assert_norm ((2285 * 169) % 3329 == 1)

(* Per-element standard-domain finalization core.

   Given the standard-domain form precondition on `myself` (the impl's
   pre-`to_standard_domain` i16 coefficient lane):
     `(v myself * 2285) mod q == plain_real_val mod q`
   (i.e., the impl's `myself` represents `α · R⁻¹` with α = plain_real_val).

   And given the trait's `mont_mul(myself, 1353)` post:
     `v r mod q == (v myself * 1353 * 169) mod q`.

   Conclusion: `v r mod q == plain_real_val mod q`, i.e., `r` carries the
   plain abstract value at the int level. *)

let lemma_mont_form_post (myself r: i16) (plain_real_val: int) :
    Lemma
    (requires
      (v myself * 2285) % 3329 == plain_real_val % 3329 /\
      v r % 3329 == (v myself * 1353 * 169) % 3329)
    (ensures v r % 3329 == plain_real_val % 3329)
  = let q : pos = 3329 in
    (* Step 1: `1353 * 169 ≡ 2285 (mod q)`. *)
    assert_norm ((1353 * 169) % 3329 == 2285);
    (* Step 2: factor `v myself * 1353 * 169 == v myself * (1353 * 169)`. *)
    assert (v myself * 1353 * 169 == v myself * (1353 * 169));
    L.lemma_mod_mul_distr_r (v myself) (1353 * 169) q;
    (* Now: `(v myself * 1353 * 169) % q == (v myself * 2285) % q`. *)
    assert ((v myself * (1353 * 169)) % q
            == (v myself * ((1353 * 169) % q)) % q);
    assert ((v myself * 1353 * 169) % q == (v myself * 2285) % q);
    (* Combine with the precondition:
         `(v myself * 2285) % q == plain_real_val % q`. *)
    assert ((v myself * 1353 * 169) % q == plain_real_val % q);
    (* And `v r % q == (v myself * 1353 * 169) % q`. *)
    assert (v r % q == plain_real_val % q)

(* Standard-domain (matrix-mul track) opaque per-lane predicate.

   Input lane: the i16 stored in the impl's polynomial after the
   `add_to_ring_element ∘ ntt_multiply` chain (e.g., `t_as_ntt[i]` at
   the call site `compute_As_plus_e` in matrix.rs).
   `plain_lane`: the corresponding spec FE value, i.e., the lane of
   `α` plain (the value `to_standard_domain(myself)` is meant to recover).

   Body: `(v input_lane * 2285) mod q == plain_lane.f_val mod q`,
   capturing that the impl coefficient is in `α · R⁻¹` form.

   Marked opaque so callers see only the structural per-lane predicate;
   the raw mod arithmetic stays hidden. *)

[@@ "opaque_to_smt"]
let mont_form_lane
    (input_lane: i16) (plain_lane: P.t_FieldElement) : prop =
  (v input_lane * 2285) % 3329 == v plain_lane.P.f_val % 3329

(* Reveal-on-demand helper.  No SMTPat — call explicitly inside Tier-2
   lemmas that need the unfolded form. *)
let lemma_mont_form_lane_reveal
    (input_lane: i16) (plain_lane: P.t_FieldElement) :
    Lemma (requires mont_form_lane input_lane plain_lane)
          (ensures
            (v input_lane * 2285) % 3329 == v plain_lane.P.f_val % 3329)
  = reveal_opaque (`%mont_form_lane)
                  (mont_form_lane input_lane plain_lane)

(* Intro direction. *)
let lemma_mont_form_lane_intro
    (input_lane: i16) (plain_lane: P.t_FieldElement) :
    Lemma (requires
            (v input_lane * 2285) % 3329 == v plain_lane.P.f_val % 3329)
          (ensures mont_form_lane input_lane plain_lane)
  = reveal_opaque (`%mont_form_lane)
                  (mont_form_lane input_lane plain_lane)

(* Per-chunk wrap (matches `forall16` pattern in trait posts). *)
let mont_form_chunk
    (input_chunk: t_Array i16 (mk_usize 16))
    (plain_chunk: t_Array P.t_FieldElement (mk_usize 16)) : prop =
  Spec.Utils.forall16 (fun (i: nat {i < 16}) ->
    mont_form_lane (Seq.index input_chunk i)
                   (Seq.index plain_chunk i))

(* Per-lane consumer lemma: given the standard-domain form on lane
   `myself` and the trait's `montgomery_multiply_by_constant(myself, 1353)`
   post, conclude that `r` carries the plain abstract value `plain`.

   This is the per-element bridge for `add_standard_error_reduce`: after
   the `mont_mul(coefficient, 1353)` step (= `to_standard_domain`), the
   lane is in plain form. *)

let lemma_to_standard_domain_finalize_fe
    (myself r: i16) (plain: P.t_FieldElement) :
    Lemma (requires
            mont_form_lane myself plain /\
            v r % 3329 == (v myself * 1353 * 169) % 3329)
          (ensures i16_to_spec_fe r == plain)
  = reveal_opaque (`%mont_form_lane) (mont_form_lane myself plain);
    lemma_mont_form_post myself r (v plain.P.f_val);
    assert (v r % 3329 == v plain.P.f_val % 3329);
    assert (v plain.P.f_val % 3329 == v plain.P.f_val);
    (* `i16_to_spec_fe r` has `f_val = (v r) % 3329`. *)
    ()

(* Per-lane bridge for the loop body of `add_standard_error_reduce`.

   Given:
     - `mont_form_lane myself plain` — standard-domain form of `myself`
     - `montgomery_multiply_by_constant_post(myself_arr, 1353, normal_arr)` at lane `l`
     - `add_post(normal_arr, error_arr, sum_arr)` at lane `l`
     - `barrett_reduce_post(sum_arr, red_arr)` at lane `l`

   Conclude:
     `i16_to_spec_fe red_arr.[l]
        == FE.new ((plain.f_val + i16_to_spec_fe error_arr.[l]).f_val) mod q)`,
   i.e., the impl chain produces the lane-wise plain spec
   `(plain.f_val + error_lane.f_val) mod q`. *)

let lemma_add_standard_error_reduce_lane
    (myself_lane normal_lane error_lane sum_lane red_lane: i16)
    (plain: P.t_FieldElement) :
    Lemma (requires
            mont_form_lane myself_lane plain /\
            (* From `mont_mul(myself, 1353)` post: v normal % q == v myself * 1353 * 169 % q *)
            v normal_lane % 3329 == (v myself_lane * 1353 * 169) % 3329 /\
            v sum_lane == v normal_lane + v error_lane /\
            v red_lane % 3329 == v sum_lane % 3329)
          (ensures
            i16_to_spec_fe red_lane
              == P.impl_FieldElement__add plain
                   (i16_to_spec_fe error_lane))
  = (* Step 1: per-lane to_standard_domain finalize: `i16_to_spec_fe normal_lane == plain`. *)
    lemma_to_standard_domain_finalize_fe myself_lane normal_lane plain;
    (* Step 2: barrett: i16_to_spec_fe red == i16_to_spec_fe sum. *)
    lemma_barrett_fe_commute sum_lane red_lane;
    (* Step 3: lift the int sum to the FE-add equation. *)
    lemma_add_fe_commute_plain normal_lane error_lane sum_lane

(* Closed-form variant of `lemma_add_standard_error_reduce_lane`.
   Combines the 3 trait-post preconditions (mont_mul + add + barrett) into
   a single composed mod-q identity:
     `v red % q == (v myself_pre * 1353 * 169 + v error) % q`.

   Used by `add_standard_error_reduce`'s loop invariant (Option B): track
   only this closed form for processed chunks (no need to store
   intermediate `normal`/`sum` values in the invariant via existentials).
   Caller then invokes this lemma per lane in a post-loop forall_intro to
   discharge the FE-add equation parameterized by ANY plain that has
   `mont_form_lane` on the input. *)
let lemma_add_standard_error_reduce_lane_closed
    (myself_pre red error_lane: i16) (plain: P.t_FieldElement) :
    Lemma (requires
            mont_form_lane myself_pre plain /\
            v red % 3329 ==
              (v myself_pre * 1353 * 169 + v error_lane) % 3329)
          (ensures
            i16_to_spec_fe red
              == P.impl_FieldElement__add plain
                   (i16_to_spec_fe error_lane))
  = let q : pos = 3329 in
    reveal_opaque (`%mont_form_lane) (mont_form_lane myself_pre plain);
    (* mont_form_lane: (v myself_pre * 2285) % q == v plain.f_val % q. *)
    (* Bridge `1353 · 169 ≡ 2285 (mod q)` (already proved in lemma_1353_eq_R). *)
    assert_norm ((1353 * 169) % q == 2285);
    L.lemma_mod_mul_distr_r (v myself_pre) (1353 * 169) q;
    assert (v myself_pre * 1353 * 169 == v myself_pre * (1353 * 169));
    (* Hence (v myself_pre * 1353 * 169) % q == (v myself_pre * 2285) % q
                                              == v plain.f_val % q. *)
    assert ((v myself_pre * 1353 * 169) % q == v plain.P.f_val % q);
    (* Now: v red % q == (v myself_pre * 1353 * 169 + v error_lane) % q
                       == (v plain.f_val + v error_lane) % q  (by mod-add-compat) *)
    L.lemma_mod_add_distr (v error_lane) (v myself_pre * 1353 * 169) q;
    (* Apply the existing FE-add bridge to recover the spec form.  Use a
       virtual sum_int = v plain.f_val + v error_lane to feed
       lemma_add_fe_commute_plain via the impl_FieldElement__add unfold. *)
    let p_v : nat = v plain.P.f_val in
    let e_v : int = v error_lane in
    assert (v red % q == (p_v + e_v) % q);
    (* Goal: i16_to_spec_fe red == FE.add plain (i16_to_spec_fe error_lane).
       i16_to_spec_fe red has f_val = (v red) % q.
       FE.add plain (i16_to_spec_fe error_lane) has f_val =
         ((v plain.f_val) + (v error_lane % q)) % q
         = (v plain.f_val + v error_lane) % q  (by lemma_mod_add_distr).
       So both sides have f_val ≡ (p_v + e_v) % q. *)
    L.lemma_mod_add_distr (v plain.P.f_val) (v error_lane) q;
    ()

(* ----- Phase 7a Step 7: poly-level commute for `add_standard_error_reduce` -----

   Tier-1 lemma assembling the per-chunk FE-add equation (lifted by the
   caller via `lemma_add_standard_error_reduce_lane` per processed chunk)
   into the hacspec function equation, parameterized by a ghost
   `ntt_product` array.

   The caller (Rust `add_standard_error_reduce`) discharges the per-chunk
   FE-add equation in its loop invariant.  The lemma below composes 16
   per-chunk × 16 per-lane = 256 lane-level FE equations into the
   `to_spec_poly_plain (future_myself) == HP.add_standard_error_reduce ...`
   identity. *)

#push-options "--z3rlimit 400 --split_queries always"

(* Tier-1 commute for `add_standard_error_reduce`.  The caller is expected
   to discharge per-lane FE-add equations specialized to the per-chunk
   slice of `ntt_product` (via `lemma_add_standard_error_reduce_lane`
   inside the loop body); this lemma assembles 256 such equations into
   the hacspec polynomial-level identity. *)
let lemma_add_standard_error_reduce_commute
    (#vV: Type0) {| i: T.t_Operations vV |}
    (myself error result: V.t_PolynomialRingElement vV)
    (ntt_product: t_Array P.t_FieldElement (mk_usize 256)) :
  Lemma
    (requires
      (* Per-lane FE-add equation specialized to the slice of ntt_product
         at chunk k.  The caller's loop body proves this directly via
         `lemma_add_standard_error_reduce_lane` after each iteration. *)
      forall (k: nat) (l: nat). k < 16 /\ l < 16 ==>
        i16_to_spec_fe
          (Seq.index (T.f_repr (Seq.index result.V.f_coefficients k)) l)
          == P.impl_FieldElement__add
               (Seq.index
                 (Seq.slice ntt_product (k * 16) (k * 16 + 16)) l)
               (i16_to_spec_fe
                 (Seq.index (T.f_repr (Seq.index error.V.f_coefficients k)) l)))
    (ensures
       to_spec_poly_plain result
         == HP.add_standard_error_reduce ntt_product (to_spec_poly_plain error))
  = let lhs_poly = to_spec_poly_plain result in
    let err_poly = to_spec_poly_plain error in
    let hp = HP.add_standard_error_reduce ntt_product err_poly in
    let aux (j: nat) : Lemma (j < 256 ==>
        Seq.index lhs_poly j == Seq.index hp j)
      = if j < 256 then begin
          let k : nat = j / 16 in
          let l : nat = j % 16 in
          let e_arr  = T.f_repr (Seq.index error.V.f_coefficients k) in
          let r_arr  = T.f_repr (Seq.index result.V.f_coefficients k) in
          let np_slice = Seq.slice ntt_product (k * 16) (k * 16 + 16) in
          (* Apply the FE-add equation at (k, l). *)
          assert (i16_to_spec_fe (Seq.index r_arr l)
                    == P.impl_FieldElement__add
                         (Seq.index np_slice l)
                         (i16_to_spec_fe (Seq.index e_arr l)));
          (* Slice index: `np_slice.[l] == ntt_product.[j]`. *)
          Seq.lemma_index_slice ntt_product (k * 16) (k * 16 + 16) l;
          (* Unfold to_spec_poly_plain at index j. *)
          poly_lane_plain result j;
          poly_lane_plain error j;
          (* Unfold the createi in HP.add_standard_error_reduce at j. *)
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (jj: usize { jj <. mk_usize 256 }) ->
              P.impl_FieldElement__new
                (cast (((cast ((Seq.index ntt_product (v jj)).P.f_val <: u16) <: u32) +!
                        (cast ((Seq.index err_poly (v jj)).P.f_val <: u16) <: u32)
                        <: u32)
                      %! (cast (P.v_FIELD_MODULUS <: u16) <: u32)
                      <: u32)
                <: u16)
              <: P.t_FieldElement)
            (sz j)
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro lhs_poly hp

#pop-options

(*** Phase 7b — Forward NTT layer commute (target #1: ntt_at_layer_1) ***)

(* Per-lane unfold helper for `mont_i16_to_spec_array`.  Wraps
   `createi_lemma` to surface the per-lane FE for an i16 array. *)
let mont_array_lane (#n: usize)
    (x: t_Array i16 n) (i: usize { v i < v n }) :
    Lemma (Seq.index (mont_i16_to_spec_array n x) (v i)
           == mont_i16_to_spec_fe (Seq.index x (v i)))
  = P.createi_lemma #P.t_FieldElement n
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. n }) ->
        (mont_i16_to_spec_fe (Seq.index x (v j)) <: P.t_FieldElement))
      i

(* Per-lane unfold helper for `zetas_4_`. *)
let zetas_4_lane (z0 z1 z2 z3: i16) (i: usize { v i < 4 }) :
    Lemma (Seq.index (zetas_4_ z0 z1 z2 z3) (v i)
           == (if v i = 0 then mont_i16_to_spec_fe z0
               else if v i = 1 then mont_i16_to_spec_fe z1
               else if v i = 2 then mont_i16_to_spec_fe z2
               else mont_i16_to_spec_fe z3))
  = P.createi_lemma #P.t_FieldElement (mk_usize 4)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 4 }) ->
        (if v j = 0 then mont_i16_to_spec_fe z0
         else if v j = 1 then mont_i16_to_spec_fe z1
         else if v j = 2 then mont_i16_to_spec_fe z2
         else mont_i16_to_spec_fe z3) <: P.t_FieldElement)
      i

#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"

(* Per-lane unfold for `N.ntt_layer_n (mk_usize 16) p (mk_usize 2) zs`
   at concrete lane `i ∈ [0, 16)`.  Computes the body of the createi at
   index `i`: `group = i / 4`, `idx = i % 4`, then either
   `butterfly._1` (if idx < 2: lanes i, i+2) or `_2` (else: lanes i-2, i).

   Surfaces the result at lane `i` as a concrete `butterfly` expression
   so per-lane matching against the trait branch post is reducible. *)
let lemma_ntt_layer_n_16_2_lane
    (p: t_Array P.t_FieldElement (mk_usize 16))
    (zs: t_Array P.t_FieldElement (mk_usize 4))
    (i: nat {i < 16}) :
    Lemma
      (let result = N.ntt_layer_n (mk_usize 16) p (mk_usize 2)
                                  (Rust_primitives.unsize zs) in
       let group : nat = i / 4 in
       let idx   : nat = i % 4 in
       (idx < 2 ==>
         i + 2 < 16 /\
         Seq.index result i ==
           (N.butterfly (Seq.index zs group)
                        (Seq.index p i)
                        (Seq.index p (i + 2)))._1) /\
       (idx >= 2 ==>
         i >= 2 /\
         Seq.index result i ==
           (N.butterfly (Seq.index zs group)
                        (Seq.index p (i - 2))
                        (Seq.index p i))._2))
  = let result = N.ntt_layer_n (mk_usize 16) p (mk_usize 2)
                                (Rust_primitives.unsize zs) in
    P.createi_lemma #P.t_FieldElement (mk_usize 16)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 16 }) ->
        let group:usize = j /! (mk_usize 2 *! mk_usize 2 <: usize) in
        let idx:usize = j %! (mk_usize 2 *! mk_usize 2 <: usize) in
        (if idx <. mk_usize 2 then
          (N.butterfly (Seq.index zs (v group))
                       (Seq.index p (v j))
                       (Seq.index p (v j + 2)))._1
        else
          (N.butterfly (Seq.index zs (v group))
                       (Seq.index p (v j - 2))
                       (Seq.index p (v j)))._2)
        <: P.t_FieldElement)
      (sz i)

#pop-options

#restart-solver

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Per-lane bridge for `f_ntt_layer_1_step`: produces the per-lane FE
   equation `out_fe.[i] == rhs.[i]` from the trait branch post and the
   `lemma_ntt_layer_n_16_2_lane` unfold helper.

   Key idea: lane `i ∈ [0, 16)` belongs to branch `b = i / 4`, position
   `idx = i % 4` within the branch.  The trait branch post exposes 4
   FE equalities at lanes `(4b, 4b+1, 4b+2, 4b+3)`.  The hacspec lane
   `i` matches:
     - if idx < 2 (lanes 4b or 4b+1): `result[i] = vec[i] + z*vec[i+2]`
       (first FE eq for `i = 4b`, third for `i = 4b+1`)
     - if idx >= 2 (lanes 4b+2 or 4b+3): `result[i] = vec[i-2] - z*vec[i]`
       (second FE eq for `i = 4b+2`, fourth for `i = 4b+3`)
   The N.butterfly._{1,2} structurally matches the branch post's
   add/sub by virtue of `mont_i16_to_spec_fe`'s linearity. *)
private
let lemma_ntt_layer_1_step_lane_bridge
    (in_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16)
    (i: nat {i < 16}) :
  Lemma
    (requires
      TS.ntt_layer_1_step_post in_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let rhs = N.ntt_layer_n (mk_usize 16) p_fe (mk_usize 2)
                               (Rust_primitives.unsize zs) in
       Seq.index r_fe i == Seq.index rhs i))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    (* Branch b = i / 4 ∈ {0,1,2,3}; reveal post for that branch. *)
    let b : nat = i / 4 in
    assert (b < 4);
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_layer_1_step_branch_post bb in_arr zeta0 zeta1 zeta2 zeta3 out_arr));
    assert (TS.ntt_layer_1_step_branch_post b in_arr zeta0 zeta1 zeta2 zeta3 out_arr);
    reveal_opaque (`%TS.ntt_layer_1_step_branch_post)
                  (TS.ntt_layer_1_step_branch_post b in_arr zeta0 zeta1 zeta2 zeta3 out_arr);
    (* Now we have, for the right z (zeta0..3 picked by b), 4 FE equalities
       at lanes (4b, 4b+2, 4b+1, 4b+3). *)
    lemma_ntt_layer_n_16_2_lane p_fe zs i;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz b);
    (* Unfold per-array index helpers — these provide
       `(mont_i16_to_spec_array (sz 16) x).[i] == mont_i16_to_spec_fe x.[i]`. *)
    mont_array_lane out_arr (sz i);
    mont_array_lane in_arr (sz i);
    let idx : nat = i % 4 in
    if idx < 2 then begin
      assert (i + 2 < 16);
      mont_array_lane in_arr (sz (i + 2))
    end else begin
      assert (i >= 2);
      mont_array_lane in_arr (sz (i - 2))
    end

#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Per-vector hacspec bridge for `f_ntt_layer_1_step`.

   Composes the 16 per-lane bridges via `Classical.forall_intro` +
   `Seq.lemma_eq_intro`. *)
let lemma_ntt_layer_1_step_to_hacspec
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires TS.ntt_layer_1_step_pre (T.f_repr vec) zeta0 zeta1 zeta2 zeta3)
    (ensures
       (let r = T.f_ntt_layer_1_step vec zeta0 zeta1 zeta2 zeta3 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r) ==
          N.ntt_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (sz 16) (T.f_repr vec))
            (mk_usize 2)
            (Rust_primitives.unsize (zetas_4_ zeta0 zeta1 zeta2 zeta3))))
  = let r = T.f_ntt_layer_1_step vec zeta0 zeta1 zeta2 zeta3 in
    let in_arr = T.f_repr vec in
    let out_arr = T.f_repr r in
    let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let p_fe = mont_i16_to_spec_array (sz 16) in_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) out_arr in
    let rhs = N.ntt_layer_n (mk_usize 16) p_fe (mk_usize 2)
                            (Rust_primitives.unsize zs) in
    assert (TS.ntt_layer_1_step_post in_arr zeta0 zeta1 zeta2 zeta3 out_arr);
    let aux (j: nat) : Lemma (j < 16 ==> Seq.index r_fe j == Seq.index rhs j)
      = if j < 16 then
          lemma_ntt_layer_1_step_lane_bridge in_arr out_arr
            zeta0 zeta1 zeta2 zeta3 j
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro r_fe rhs

#pop-options

(* ────────────  NTT multiply — chunk commute  ────────────
   Same architecture as `lemma_ntt_layer_1_step_to_hacspec` above. *)

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Per-lane unfold for `N.ntt_multiply_n (mk_usize 16) p1 p2 zs` at
   concrete lane `i ∈ [0, 16)`.  Mirrors `lemma_ntt_layer_n_16_2_lane`:
   computes the createi body at index `i` — `group = i / 4`, zeta is
   `zs[group]` (idx < 2) or its FE negation (idx ≥ 2), and the lane is
   `base_case_multiply_even` (i even, partner i+1) or `_odd` (i odd,
   partner i−1). *)
let lemma_ntt_multiply_n_16_lane
    (p1 p2: t_Array P.t_FieldElement (mk_usize 16))
    (zs: t_Array P.t_FieldElement (mk_usize 4))
    (i: nat {i < 16}) :
    Lemma
      (let result = N.ntt_multiply_n (mk_usize 16) p1 p2
                                     (Rust_primitives.unsize zs) in
       let group : nat = i / 4 in
       let zeta = (if i % 4 < 2 then Seq.index zs group
                   else P.impl_FieldElement__neg (Seq.index zs group)) in
       (i % 2 = 0 ==>
         i + 1 < 16 /\
         Seq.index result i ==
           N.base_case_multiply_even (Seq.index p1 i) (Seq.index p1 (i + 1))
                                     (Seq.index p2 i) (Seq.index p2 (i + 1))
                                     zeta) /\
       (i % 2 = 1 ==>
         i >= 1 /\
         Seq.index result i ==
           N.base_case_multiply_odd (Seq.index p1 (i - 1)) (Seq.index p1 i)
                                    (Seq.index p2 (i - 1)) (Seq.index p2 i)))
  = P.createi_lemma #P.t_FieldElement (mk_usize 16)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 16 }) ->
        let group:usize = j /! mk_usize 4 in
        let zeta:P.t_FieldElement =
          if (j %! mk_usize 4 <: usize) <. mk_usize 2
          then Seq.index zs (v group)
          else P.impl_FieldElement__neg (Seq.index zs (v group))
        in
        (if (j %! mk_usize 2 <: usize) =. mk_usize 0
         then
           N.base_case_multiply_even (Seq.index p1 (v j)) (Seq.index p1 (v j + 1))
                                     (Seq.index p2 (v j)) (Seq.index p2 (v j + 1))
                                     zeta
         else
           N.base_case_multiply_odd (Seq.index p1 (v j - 1)) (Seq.index p1 (v j))
                                    (Seq.index p2 (v j - 1)) (Seq.index p2 (v j)))
        <: P.t_FieldElement)
      (sz i)

#pop-options

(* The Montgomery lift commutes with FE negation: for |z| ≤ 1664 (so
   `neg_i16 z` is the true integer negation), lifting `−z` equals the
   FE additive inverse of lifting `z`. *)
let lemma_mont_fe_neg (z: i16) :
  Lemma (requires Spec.Utils.is_i16b 1664 z)
        (ensures mont_i16_to_spec_fe (Spec.Utils.neg_i16 z)
                 == P.impl_FieldElement__neg (mont_i16_to_spec_fe z))
  = (* LHS f_val = ((−v z) · 169) % q;  RHS f_val = (q − (v z · 169) % q) % q. *)
    L.lemma_mod_sub_distr 3329 (v z * 169) 3329;
    (*  (3329 − (vz·169) % q) % q == (3329 − vz·169) % q  *)
    L.lemma_mod_plus ((- (v z)) * 169) 1 3329
    (*  (−vz·169 + 3329) % q == (−vz·169) % q  *)

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Per-lane bridge for `f_ntt_multiply`: produces the per-lane FE
   equation from the trait branch post and the per-lane unfold of
   `N.ntt_multiply_n`.  Lane `i` belongs to branch `b = i / 4`; the
   branch post exposes 4 FE equalities at lanes (4b, 4b+1, 4b+2, 4b+3):
     - i % 4 = 0: even base-case at binomial 2b   (zeta = +z_b)
     - i % 4 = 1: odd  base-case at binomial 2b   (no zeta)
     - i % 4 = 2: even base-case at binomial 2b+1 (zeta = −z_b)
     - i % 4 = 3: odd  base-case at binomial 2b+1 (no zeta)
   The N-side zeta for idx ≥ 2 is `impl_FieldElement__neg zs[b]`,
   bridged to the branch post's `mont (neg_i16 z_b)` by `lemma_mont_fe_neg`. *)
(* Per-lane bridges at concrete lanes — one top-level lemma per lane so
   each verifies in a minimal context (the single shared-context dispatch
   version saturated Z3 in 5 of its 121 split branches, 4-17 min cancels). *)
private
let lemma_ntt_multiply_lane_bridge_0
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 0 == Seq.index nrhs 0))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 0 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 0;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 0);
    mont_array_lane out_arr (sz 0);
    mont_array_lane lhs_arr (sz 0);
    mont_array_lane rhs_arr (sz 0);
    mont_array_lane lhs_arr (sz 1);
    mont_array_lane rhs_arr (sz 1)

private
let lemma_ntt_multiply_lane_bridge_1
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 1 == Seq.index nrhs 1))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 0 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 1;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 0);
    mont_array_lane out_arr (sz 1);
    mont_array_lane lhs_arr (sz 1);
    mont_array_lane rhs_arr (sz 1);
    mont_array_lane lhs_arr (sz 0);
    mont_array_lane rhs_arr (sz 0)

private
let lemma_ntt_multiply_lane_bridge_2
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 2 == Seq.index nrhs 2))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 0 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 2;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 0);
    lemma_mont_fe_neg zeta0;
    mont_array_lane out_arr (sz 2);
    mont_array_lane lhs_arr (sz 2);
    mont_array_lane rhs_arr (sz 2);
    mont_array_lane lhs_arr (sz 3);
    mont_array_lane rhs_arr (sz 3)

private
let lemma_ntt_multiply_lane_bridge_3
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 3 == Seq.index nrhs 3))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 0 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 3;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 0);
    lemma_mont_fe_neg zeta0;
    mont_array_lane out_arr (sz 3);
    mont_array_lane lhs_arr (sz 3);
    mont_array_lane rhs_arr (sz 3);
    mont_array_lane lhs_arr (sz 2);
    mont_array_lane rhs_arr (sz 2)

private
let lemma_ntt_multiply_lane_bridge_4
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 4 == Seq.index nrhs 4))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 1 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 4;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 1);
    mont_array_lane out_arr (sz 4);
    mont_array_lane lhs_arr (sz 4);
    mont_array_lane rhs_arr (sz 4);
    mont_array_lane lhs_arr (sz 5);
    mont_array_lane rhs_arr (sz 5)

private
let lemma_ntt_multiply_lane_bridge_5
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 5 == Seq.index nrhs 5))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 1 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 5;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 1);
    mont_array_lane out_arr (sz 5);
    mont_array_lane lhs_arr (sz 5);
    mont_array_lane rhs_arr (sz 5);
    mont_array_lane lhs_arr (sz 4);
    mont_array_lane rhs_arr (sz 4)

private
let lemma_ntt_multiply_lane_bridge_6
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 6 == Seq.index nrhs 6))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 1 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 6;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 1);
    lemma_mont_fe_neg zeta1;
    mont_array_lane out_arr (sz 6);
    mont_array_lane lhs_arr (sz 6);
    mont_array_lane rhs_arr (sz 6);
    mont_array_lane lhs_arr (sz 7);
    mont_array_lane rhs_arr (sz 7)

private
let lemma_ntt_multiply_lane_bridge_7
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 7 == Seq.index nrhs 7))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 1 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 7;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 1);
    lemma_mont_fe_neg zeta1;
    mont_array_lane out_arr (sz 7);
    mont_array_lane lhs_arr (sz 7);
    mont_array_lane rhs_arr (sz 7);
    mont_array_lane lhs_arr (sz 6);
    mont_array_lane rhs_arr (sz 6)

private
let lemma_ntt_multiply_lane_bridge_8
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 8 == Seq.index nrhs 8))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 2 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 8;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 2);
    mont_array_lane out_arr (sz 8);
    mont_array_lane lhs_arr (sz 8);
    mont_array_lane rhs_arr (sz 8);
    mont_array_lane lhs_arr (sz 9);
    mont_array_lane rhs_arr (sz 9)

private
let lemma_ntt_multiply_lane_bridge_9
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 9 == Seq.index nrhs 9))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 2 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 9;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 2);
    mont_array_lane out_arr (sz 9);
    mont_array_lane lhs_arr (sz 9);
    mont_array_lane rhs_arr (sz 9);
    mont_array_lane lhs_arr (sz 8);
    mont_array_lane rhs_arr (sz 8)

private
let lemma_ntt_multiply_lane_bridge_10
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 10 == Seq.index nrhs 10))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 2 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 10;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 2);
    lemma_mont_fe_neg zeta2;
    mont_array_lane out_arr (sz 10);
    mont_array_lane lhs_arr (sz 10);
    mont_array_lane rhs_arr (sz 10);
    mont_array_lane lhs_arr (sz 11);
    mont_array_lane rhs_arr (sz 11)

private
let lemma_ntt_multiply_lane_bridge_11
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 11 == Seq.index nrhs 11))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 2 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 11;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 2);
    lemma_mont_fe_neg zeta2;
    mont_array_lane out_arr (sz 11);
    mont_array_lane lhs_arr (sz 11);
    mont_array_lane rhs_arr (sz 11);
    mont_array_lane lhs_arr (sz 10);
    mont_array_lane rhs_arr (sz 10)

private
let lemma_ntt_multiply_lane_bridge_12
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 12 == Seq.index nrhs 12))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 3 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 12;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 3);
    mont_array_lane out_arr (sz 12);
    mont_array_lane lhs_arr (sz 12);
    mont_array_lane rhs_arr (sz 12);
    mont_array_lane lhs_arr (sz 13);
    mont_array_lane rhs_arr (sz 13)

private
let lemma_ntt_multiply_lane_bridge_13
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 13 == Seq.index nrhs 13))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 3 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 13;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 3);
    mont_array_lane out_arr (sz 13);
    mont_array_lane lhs_arr (sz 13);
    mont_array_lane rhs_arr (sz 13);
    mont_array_lane lhs_arr (sz 12);
    mont_array_lane rhs_arr (sz 12)

private
let lemma_ntt_multiply_lane_bridge_14
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 14 == Seq.index nrhs 14))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 3 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 14;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 3);
    lemma_mont_fe_neg zeta3;
    mont_array_lane out_arr (sz 14);
    mont_array_lane lhs_arr (sz 14);
    mont_array_lane rhs_arr (sz 14);
    mont_array_lane lhs_arr (sz 15);
    mont_array_lane rhs_arr (sz 15)

private
let lemma_ntt_multiply_lane_bridge_15
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe 15 == Seq.index nrhs 15))
  = let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    assert (Spec.Utils.forall4 (fun (bb: nat{bb < 4}) ->
              TS.ntt_multiply_branch_post bb lhs_arr rhs_arr
                zeta0 zeta1 zeta2 zeta3 out_arr));
    reveal_opaque (`%TS.ntt_multiply_branch_post)
                  (TS.ntt_multiply_branch_post 3 lhs_arr rhs_arr
                     zeta0 zeta1 zeta2 zeta3 out_arr);
    lemma_ntt_multiply_n_16_lane l_fe r_fe zs 15;
    zetas_4_lane zeta0 zeta1 zeta2 zeta3 (sz 3);
    lemma_mont_fe_neg zeta3;
    mont_array_lane out_arr (sz 15);
    mont_array_lane lhs_arr (sz 15);
    mont_array_lane rhs_arr (sz 15);
    mont_array_lane lhs_arr (sz 14);
    mont_array_lane rhs_arr (sz 14)

private
let lemma_ntt_multiply_lane_bridge
    (lhs_arr rhs_arr out_arr: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16)
    (i: nat {i < 16}) :
  Lemma
    (requires
      Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
      Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
      TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr)
    (ensures
      (let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
       let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
       let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
       let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
       let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                   (Rust_primitives.unsize zs) in
       Seq.index o_fe i == Seq.index nrhs i))
  = match i with
    | 0 -> lemma_ntt_multiply_lane_bridge_0 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 1 -> lemma_ntt_multiply_lane_bridge_1 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 2 -> lemma_ntt_multiply_lane_bridge_2 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 3 -> lemma_ntt_multiply_lane_bridge_3 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 4 -> lemma_ntt_multiply_lane_bridge_4 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 5 -> lemma_ntt_multiply_lane_bridge_5 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 6 -> lemma_ntt_multiply_lane_bridge_6 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 7 -> lemma_ntt_multiply_lane_bridge_7 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 8 -> lemma_ntt_multiply_lane_bridge_8 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 9 -> lemma_ntt_multiply_lane_bridge_9 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 10 -> lemma_ntt_multiply_lane_bridge_10 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 11 -> lemma_ntt_multiply_lane_bridge_11 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 12 -> lemma_ntt_multiply_lane_bridge_12 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 13 -> lemma_ntt_multiply_lane_bridge_13 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | 14 -> lemma_ntt_multiply_lane_bridge_14 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
    | _ -> lemma_ntt_multiply_lane_bridge_15 lhs_arr rhs_arr out_arr zeta0 zeta1 zeta2 zeta3
#pop-options

#push-options "--z3rlimit 400 --fuel 0 --ifuel 1"

(* Chunk commute for `f_ntt_multiply` — closes the former assume val.
   Composes the 16 per-lane bridges via `Classical.forall_intro` +
   `Seq.lemma_eq_intro`, mirroring `lemma_ntt_layer_1_step_to_hacspec`. *)
let lemma_ntt_multiply_chunk_commutes
    (#vV: Type0) {| i: T.t_Operations vV |}
    (lhs rhs: vV) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires TS.ntt_multiply_pre (T.f_repr lhs) (T.f_repr rhs)
                                  zeta0 zeta1 zeta2 zeta3)
    (ensures
       (let r = T.f_ntt_multiply lhs rhs zeta0 zeta1 zeta2 zeta3 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r)
          == N.ntt_multiply_n (mk_usize 16)
               (mont_i16_to_spec_array (sz 16) (T.f_repr lhs))
               (mont_i16_to_spec_array (sz 16) (T.f_repr rhs))
               (zetas_4_ zeta0 zeta1 zeta2 zeta3)))
  = let r = T.f_ntt_multiply lhs rhs zeta0 zeta1 zeta2 zeta3 in
    let lhs_arr = T.f_repr lhs in
    let rhs_arr = T.f_repr rhs in
    let out_arr = T.f_repr r in
    let zs = zetas_4_ zeta0 zeta1 zeta2 zeta3 in
    let l_fe = mont_i16_to_spec_array (sz 16) lhs_arr in
    let r_fe = mont_i16_to_spec_array (sz 16) rhs_arr in
    let o_fe = mont_i16_to_spec_array (sz 16) out_arr in
    let nrhs = N.ntt_multiply_n (mk_usize 16) l_fe r_fe
                                (Rust_primitives.unsize zs) in
    assert (TS.ntt_multiply_post lhs_arr rhs_arr zeta0 zeta1 zeta2 zeta3 out_arr);
    let aux (j: nat) : Lemma (j < 16 ==> Seq.index o_fe j == Seq.index nrhs j)
      = if j < 16 then
          lemma_ntt_multiply_lane_bridge lhs_arr rhs_arr out_arr
            zeta0 zeta1 zeta2 zeta3 j
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro o_fe nrhs

#pop-options

(* ───── Layer 2 / 3 forward NTT bridges and inverse NTT bridges ─────
   STATUS: not delivered in this session.

   Same pattern as `lemma_ntt_layer_1_step_to_hacspec` above; each layer
   needs a `lemma_ntt_layer_n_16_<2*len>_lane` createi unfold, a
   `zetas_<groups>_lane` zetas unfold, a per-lane bridge that reveals
   the right branch post for `b = lane → branch` and matches against
   the trait branch post's per-lane FE equations, and a top-level
   `Classical.forall_intro` + `Seq.lemma_eq_intro` composition.

   First-cut implementation of layer 2 forward (lanes-to-branches
   mapping `b = (i / 8) * 2 + ((i % 4) / 2)`) verified the per-lane
   unfold and zetas helpers, but the lane bridge itself ran Z3 past
   2.7 minutes on a single sub-query without reaching any failure or
   success — likely because Z3 was case-splitting heavily on the
   layer-2 branch post's nested `if`-ladder for `base`/`off`/`z`.

   Recommended approach for follow-up:
     (a) Profile via `--query_stats --split_queries always` to identify
         which sub-query stalls;
     (b) Either explicitly enumerate `i ∈ {0..15}` to remove the
         nested arithmetic in `b = ...`, OR
     (c) Restructure the trait branch post so `b` is consumed by a
         flat case-split (no nested ifs).

   Inverse NTT layers (`f_inv_ntt_layer_{1,2,3}_step`) follow the same
   pattern with `IN.inv_butterfly` and `IN.ntt_inverse_layer_n` in
   place of the forward equivalents.

   Estimated remaining work: 1-2 hours per layer × 5 remaining layers
   = 5-10 hours.  Out of scope for this session. *)

(*** Phase 7a / lane A3 additions ***)

(* Goal: bridge `to_spec_poly_mont (param b)` (record-form) to the array
   form used at the body call site of `subtract_reduce`.  The body has
   `let e_b = b.f_coefficients` and after the loop reconstructs a
   `b_input = { f_coefficients = e_b }`.  Z3 has trouble equating the
   parameter-record's `to_spec_poly_mont` to the reconstructed-record's
   `to_spec_poly_mont` due to the `createi`-of-record-field structure.

   Resolution: define `to_spec_poly_mont_arr` (resp. `_plain_arr`) over a
   `t_Array vV 16` directly.  Both record forms unfold to the same array
   form via a one-line lemma — Z3 sees `array == array` directly, no
   record-projection traversal needed.

   See MLKEM_STATUS.md USER-7 hypothesis (b). *)

let to_spec_poly_mont_arr
    (#vV: Type0) {| i: T.t_Operations vV |}
    (a: t_Array vV (mk_usize 16))
    : t_Array P.t_FieldElement (mk_usize 256)
  = P.createi #P.t_FieldElement (mk_usize 256)
        #(usize -> P.t_FieldElement)
        (fun (j: usize { j <. mk_usize 256 }) ->
          (mont_i16_to_spec_fe
            (Seq.index (T.f_repr (Seq.index a (v j / 16)))
                       (v j % 16))
           <: P.t_FieldElement))

let to_spec_poly_plain_arr
    (#vV: Type0) {| i: T.t_Operations vV |}
    (a: t_Array vV (mk_usize 16))
    : t_Array P.t_FieldElement (mk_usize 256)
  = P.createi #P.t_FieldElement (mk_usize 256)
        #(usize -> P.t_FieldElement)
        (fun (j: usize { j <. mk_usize 256 }) ->
          (i16_to_spec_fe
            (Seq.index (T.f_repr (Seq.index a (v j / 16)))
                       (v j % 16))
           <: P.t_FieldElement))

(* Unfold lemma: `to_spec_poly_mont` only consumes `p.f_coefficients`,
   so it must equal `to_spec_poly_mont_arr p.f_coefficients`.  The
   bodies are structurally identical except the record projection.
   Single-line proof via `Seq.lemma_eq_intro` on the createi outputs. *)
let lemma_to_spec_poly_mont_unfold
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p: V.t_PolynomialRingElement vV) :
    Lemma (to_spec_poly_mont p == to_spec_poly_mont_arr p.V.f_coefficients)
  = let lhs = to_spec_poly_mont p in
    let rhs = to_spec_poly_mont_arr p.V.f_coefficients in
    let aux (j: nat) : Lemma (j < 256 ==> Seq.index lhs j == Seq.index rhs j)
      = if j < 256 then begin
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (k: usize { k <. mk_usize 256 }) ->
              (mont_i16_to_spec_fe
                (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (v k / 16)))
                           (v k % 16))
               <: P.t_FieldElement))
            (sz j);
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (k: usize { k <. mk_usize 256 }) ->
              (mont_i16_to_spec_fe
                (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (v k / 16)))
                           (v k % 16))
               <: P.t_FieldElement))
            (sz j)
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro lhs rhs

(* Same for plain. *)
let lemma_to_spec_poly_plain_unfold
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p: V.t_PolynomialRingElement vV) :
    Lemma (to_spec_poly_plain p == to_spec_poly_plain_arr p.V.f_coefficients)
  = let lhs = to_spec_poly_plain p in
    let rhs = to_spec_poly_plain_arr p.V.f_coefficients in
    let aux (j: nat) : Lemma (j < 256 ==> Seq.index lhs j == Seq.index rhs j)
      = if j < 256 then begin
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (k: usize { k <. mk_usize 256 }) ->
              (i16_to_spec_fe
                (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (v k / 16)))
                           (v k % 16))
               <: P.t_FieldElement))
            (sz j);
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (k: usize { k <. mk_usize 256 }) ->
              (i16_to_spec_fe
                (Seq.index (T.f_repr (Seq.index p.V.f_coefficients (v k / 16)))
                           (v k % 16))
               <: P.t_FieldElement))
            (sz j)
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro lhs rhs

(* Record-extensionality flavoured: when two ring elements share their
   `f_coefficients` array, their `to_spec_poly_mont` lifts coincide.
   Composes the two unfold lemmas for direct use at body call sites. *)
let lemma_to_spec_poly_mont_eq_of_coeffs
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p q: V.t_PolynomialRingElement vV) :
    Lemma (requires p.V.f_coefficients == q.V.f_coefficients)
          (ensures to_spec_poly_mont p == to_spec_poly_mont q)
  = lemma_to_spec_poly_mont_unfold p;
    lemma_to_spec_poly_mont_unfold q

let lemma_to_spec_poly_plain_eq_of_coeffs
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p q: V.t_PolynomialRingElement vV) :
    Lemma (requires p.V.f_coefficients == q.V.f_coefficients)
          (ensures to_spec_poly_plain p == to_spec_poly_plain q)
  = lemma_to_spec_poly_plain_unfold p;
    lemma_to_spec_poly_plain_unfold q

(* Createi-equality bridge for the `subtract_reduce`-family post.
   Given two ring elements that share `f_coefficients`, the per-lane
   createi expressions (used by both the lemma_subtract_reduce_commute
   conclusion and the function's post-condition) coincide.  This is
   exactly the bridge the body-level `subtract_reduce` closing query
   needs — Z3 doesn't auto-derive equality of createis from equality
   of the per-lane functions. *)
#push-options "--z3rlimit 100"
let lemma_subtract_reduce_scaled_eq
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p q: V.t_PolynomialRingElement vV) :
    Lemma (requires p.V.f_coefficients == q.V.f_coefficients)
          (ensures
            (P.createi #P.t_FieldElement (mk_usize 256)
               #(usize -> P.t_FieldElement)
               (fun (j: usize {j <. mk_usize 256}) ->
                 P.impl_FieldElement__mul
                   (Seq.index (to_spec_poly_mont p) (v j))
                   fe_1441))
            ==
            (P.createi #P.t_FieldElement (mk_usize 256)
               #(usize -> P.t_FieldElement)
               (fun (j: usize {j <. mk_usize 256}) ->
                 P.impl_FieldElement__mul
                   (Seq.index (to_spec_poly_mont q) (v j))
                   fe_1441)))
  = lemma_to_spec_poly_mont_eq_of_coeffs p q;
    let lhs = P.createi #P.t_FieldElement (mk_usize 256)
                #(usize -> P.t_FieldElement)
                (fun (j: usize {j <. mk_usize 256}) ->
                  P.impl_FieldElement__mul
                    (Seq.index (to_spec_poly_mont p) (v j))
                    fe_1441) in
    let rhs = P.createi #P.t_FieldElement (mk_usize 256)
                #(usize -> P.t_FieldElement)
                (fun (j: usize {j <. mk_usize 256}) ->
                  P.impl_FieldElement__mul
                    (Seq.index (to_spec_poly_mont q) (v j))
                    fe_1441) in
    let aux (j: nat) : Lemma (j < 256 ==> Seq.index lhs j == Seq.index rhs j)
      = if j < 256 then begin
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (k: usize {k <. mk_usize 256}) ->
              P.impl_FieldElement__mul
                (Seq.index (to_spec_poly_mont p) (v k))
                fe_1441)
            (sz j);
          P.createi_lemma #P.t_FieldElement (mk_usize 256)
            #(usize -> P.t_FieldElement)
            (fun (k: usize {k <. mk_usize 256}) ->
              P.impl_FieldElement__mul
                (Seq.index (to_spec_poly_mont q) (v k))
                fe_1441)
            (sz j)
        end in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro lhs rhs
#pop-options

(* Createi-equality bridge for the add_error_reduce-family post.  Given two
   ring elements that share `f_coefficients`, the per-lane scaled-createi
   expressions coincide.  The createi shape is identical to the
   subtract_reduce post's scaled operand, so this delegates to
   lemma_subtract_reduce_scaled_eq. *)
let lemma_add_error_reduce_scaled_eq
    (#vV: Type0) {| i: T.t_Operations vV |}
    (p q: V.t_PolynomialRingElement vV) :
    Lemma (requires p.V.f_coefficients == q.V.f_coefficients)
          (ensures
            (P.createi #P.t_FieldElement (mk_usize 256)
               #(usize -> P.t_FieldElement)
               (fun (j: usize {j <. mk_usize 256}) ->
                 P.impl_FieldElement__mul
                   (Seq.index (to_spec_poly_mont p) (v j))
                   fe_1441))
            ==
            (P.createi #P.t_FieldElement (mk_usize 256)
               #(usize -> P.t_FieldElement)
               (fun (j: usize {j <. mk_usize 256}) ->
                 P.impl_FieldElement__mul
                   (Seq.index (to_spec_poly_mont q) (v j))
                   fe_1441)))
  = lemma_subtract_reduce_scaled_eq p q

(* ===================================================================== *)
(* Phase 7a / lane A5 additions — polynomial-level lift + zetas helpers   *)
(* for `invert_ntt_montgomery` Step 5 strengthening.                       *)
(*                                                                         *)
(* `mont_to_spec_poly_256`: flattens a 16-chunk × 16-lane impl polynomial  *)
(* into a 256-element abstract FE polynomial, the canonical input shape    *)
(* of `Hacspec_ml_kem.Invert_ntt.ntt_inverse_butterflies`.                  *)
(*                                                                         *)
(* `zetas_8 / zetas_4_inv / zetas_2_inv / zetas_1_inv`: layer-N zetas      *)
(* slices for layers 4..7 (inverse direction).  Layer 4 has 8 zetas,       *)
(* layer 5 has 4, layer 6 has 2, layer 7 has 1.  Lifted via                 *)
(* `mont_i16_to_spec_fe` so the slice operates on plain abstract zetas      *)
(* (matching `IN.ntt_inverse_layer_n`'s zetas argument convention).        *)
(* ===================================================================== *)

#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"

(* Helper: for `idx: usize` with `v idx < 256`, the body of the createi
   uses `coeffs.[v idx / 16]` (chunk) then `chunk.[v idx % 16]` (lane).
   Factored out so the createi body is a single direct expression. *)
let mont_to_spec_poly_256_body
    (#vV: Type0) {| i: T.t_Operations vV |}
    (coeffs: t_Array vV (mk_usize 16))
    (idx: usize { v idx < 256 })
    : P.t_FieldElement =
  let ci : nat = v idx / 16 in
  let li : nat = v idx % 16 in
  assert (ci < 16);
  assert (li < 16);
  let chunk_arr : t_Array i16 (mk_usize 16) =
    T.f_repr (Seq.index coeffs ci) in
  mont_i16_to_spec_fe (Seq.index chunk_arr li)

let mont_to_spec_poly_256
    (#vV: Type0) {| i: T.t_Operations vV |}
    (coeffs: t_Array vV (mk_usize 16))
    : t_Array P.t_FieldElement (mk_usize 256) =
  P.createi #P.t_FieldElement (mk_usize 256)
    #(usize -> P.t_FieldElement)
    (fun (idx: usize { idx <. mk_usize 256 }) ->
      (mont_to_spec_poly_256_body #vV coeffs idx
        <: P.t_FieldElement))

(* Per-lane unfold helper for `mont_to_spec_poly_256`.  The createi body
   reduces the index to (chunk = i/16, lane = i%16) and projects through
   `mont_i16_to_spec_fe`.  Mirrors `mont_array_lane`'s shape. *)
let mont_to_spec_poly_256_lane
    (#vV: Type0) {| i: T.t_Operations vV |}
    (coeffs: t_Array vV (mk_usize 16))
    (idx: usize { v idx < 256 }) :
    Lemma (Seq.index (mont_to_spec_poly_256 #vV coeffs) (v idx)
           == mont_i16_to_spec_fe
                (Seq.index (T.f_repr (Seq.index coeffs (v idx / 16)))
                            (v idx % 16)))
  = P.createi_lemma #P.t_FieldElement (mk_usize 256)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 256 }) ->
        (mont_to_spec_poly_256_body #vV coeffs j
          <: P.t_FieldElement))
      idx

#pop-options

(* Layer 4 inverse zetas: 8 i16 zetas → 8 abstract FEs lifted via
   `mont_i16_to_spec_fe`.  Matches `IN.ntt_inverse_layer 256 p 4`'s
   internal `groups = 128/16 = 8` zetas slice. *)
let zetas_8 (z0 z1 z2 z3 z4 z5 z6 z7: i16)
    : t_Array P.t_FieldElement (mk_usize 8) =
  P.createi #P.t_FieldElement (mk_usize 8)
    #(usize -> P.t_FieldElement)
    (fun (i: usize) ->
      ((if v i = 0 then mont_i16_to_spec_fe z0
        else if v i = 1 then mont_i16_to_spec_fe z1
        else if v i = 2 then mont_i16_to_spec_fe z2
        else if v i = 3 then mont_i16_to_spec_fe z3
        else if v i = 4 then mont_i16_to_spec_fe z4
        else if v i = 5 then mont_i16_to_spec_fe z5
        else if v i = 6 then mont_i16_to_spec_fe z6
        else mont_i16_to_spec_fe z7)
        <: P.t_FieldElement))

(* Layer 5..7 inverse: shared shapes with the existing `zetas_1_/2/4`
   in `Vector.Traits.Spec`.  Layer 5: 4 zetas (zetas_4_); Layer 6: 2
   (zetas_2_); Layer 7: 1 (zetas_1_).  Provided by Spec module already.    *)

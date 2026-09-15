module Hacspec_ml_kem.Commute.Bridges
#set-options "--fuel 0 --ifuel 1 --z3rlimit 200"
open FStar.Mul
open Core_models
open Libcrux_ml_kem.Vector.Traits.Spec
open Hacspec_ml_kem.Commute.Chunk

module P  = Hacspec_ml_kem.Parameters
module T  = Libcrux_ml_kem.Vector.Traits
module TS = Libcrux_ml_kem.Vector.Traits.Spec
module N  = Hacspec_ml_kem.Ntt
module IN = Hacspec_ml_kem.Invert_ntt
module VS = Libcrux_ml_kem.Vector.Spec
module VV = Libcrux_ml_kem.Vector

(* ============================================================================
   ABSTRACT INTERFACE for the commute-layer NTT/INTT hacspec bridges.

   Purpose: consumers of this module typecheck against this cheap, stable
   `.fsti.checked` rather than the heavy `Bridges.fst.checked`, whose per-lane
   `lemma_*_lane_bridge` proofs are not robustly provable (symbolic i/4,
   i%4 + SIMD-helper opaque-reveal chains).

   Consumers are the extraction impl (Libcrux_ml_kem.Ntt, Libcrux_ml_kem.Invert_ntt)
   AND the sibling commute bridges (Commute.Ntt_bridge, Commute.Invert_ntt_bridge,
   Commute.Matrix_bridge) which `open` this module and use its helpers bare.  The
   24 declarations below are exactly the union referenced across all five; the
   ~19 per-lane lane-bridge helpers (lemma_*_lane_bridge, lemma_*_16_*_lane) stay
   module-private (absent here).

   Declaration order MUST match the `let` order in the `.fst` (interface `val`
   order gates the implementation match, Error 233 otherwise).  None of these
   carry an SMTPat, so order is a pure implementation-match concern; the two
   `.fsti`-defined items (cross_vec_hyp let, lemma_zeta_eq_vzetas is a `val` with
   an `= admit ()` body in the `.fst`) are placed at their `.fst` positions.
   ============================================================================ *)

val lemma_ntt_layer_1_step_to_hacspec
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

val lemma_inv_ntt_layer_1_step_to_hacspec
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0 zeta1 zeta2 zeta3: i16) :
  Lemma
    (requires TS.inv_ntt_layer_1_step_pre (T.f_repr vec) zeta0 zeta1 zeta2 zeta3)
    (ensures
       (let r = T.f_inv_ntt_layer_1_step vec zeta0 zeta1 zeta2 zeta3 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (sz 16) (T.f_repr vec))
            (mk_usize 2)
            (Rust_primitives.unsize (zetas_4_ zeta0 zeta1 zeta2 zeta3))))

val zetas_1_lane (z0: i16) (i: usize { v i < 1 }) :
    Lemma (Seq.index (zetas_1_ z0) (v i) == mont_i16_to_spec_fe z0)

val lemma_inv_ntt_layer_3_step_to_hacspec
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0: i16) :
  Lemma
    (requires TS.inv_ntt_layer_3_step_pre (T.f_repr vec) zeta0)
    (ensures
       (let r = T.f_inv_ntt_layer_3_step vec zeta0 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (sz 16) (T.f_repr vec))
            (mk_usize 8)
            (Rust_primitives.unsize (zetas_1_ zeta0))))

val zetas_2_lane (z0 z1: i16) (i: usize { v i < 2 }) :
    Lemma (Seq.index (zetas_2_ z0 z1) (v i)
           == (if v i = 0 then mont_i16_to_spec_fe z0
               else mont_i16_to_spec_fe z1))

val lemma_inv_ntt_layer_2_step_to_hacspec
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0 zeta1: i16) :
  Lemma
    (requires TS.inv_ntt_layer_2_step_pre (T.f_repr vec) zeta0 zeta1)
    (ensures
       (let r = T.f_inv_ntt_layer_2_step vec zeta0 zeta1 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (sz 16) (T.f_repr vec))
            (mk_usize 4)
            (Rust_primitives.unsize (zetas_2_ zeta0 zeta1))))

val lemma_ntt_layer_2_step_to_hacspec
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0 zeta1: i16) :
  Lemma
    (requires TS.ntt_layer_2_step_pre (T.f_repr vec) zeta0 zeta1)
    (ensures
       (let r = T.f_ntt_layer_2_step vec zeta0 zeta1 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r) ==
          N.ntt_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (sz 16) (T.f_repr vec))
            (mk_usize 4)
            (Rust_primitives.unsize (zetas_2_ zeta0 zeta1))))

val lemma_ntt_layer_3_step_to_hacspec
    (#vV: Type0) {| i: T.t_Operations vV |}
    (vec: vV) (zeta0: i16) :
  Lemma
    (requires TS.ntt_layer_3_step_pre (T.f_repr vec) zeta0)
    (ensures
       (let r = T.f_ntt_layer_3_step vec zeta0 in
        mont_i16_to_spec_array (sz 16) (T.f_repr r) ==
          N.ntt_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (sz 16) (T.f_repr vec))
            (mk_usize 8)
            (Rust_primitives.unsize (zetas_1_ zeta0))))

val lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec
    (a_arr b_arr r0_arr r1_arr: t_Array i16 (mk_usize 16))
    (zeta_r: i16) :
  Lemma
    (requires
       (forall (i: nat). i < 16 ==>
          mont_i16_to_spec_fe (Seq.index r0_arr i) ==
          P.impl_FieldElement__add
            (mont_i16_to_spec_fe (Seq.index a_arr i))
            (mont_i16_to_spec_fe (Seq.index b_arr i))) /\
       (forall (i: nat). i < 16 ==>
          mont_i16_to_spec_fe (Seq.index r1_arr i) ==
          P.impl_FieldElement__mul
            (mont_i16_to_spec_fe zeta_r)
            (P.impl_FieldElement__sub
              (mont_i16_to_spec_fe (Seq.index b_arr i))
              (mont_i16_to_spec_fe (Seq.index a_arr i)))))
    (ensures
       (forall (i: nat). i < 16 ==>
          mont_i16_to_spec_fe (Seq.index r0_arr i) ==
          (IN.inv_butterfly (mont_i16_to_spec_fe zeta_r)
                             (mont_i16_to_spec_fe (Seq.index a_arr i))
                             (mont_i16_to_spec_fe (Seq.index b_arr i)))._1) /\
       (forall (i: nat). i < 16 ==>
          mont_i16_to_spec_fe (Seq.index r1_arr i) ==
          (IN.inv_butterfly (mont_i16_to_spec_fe zeta_r)
                             (mont_i16_to_spec_fe (Seq.index a_arr i))
                             (mont_i16_to_spec_fe (Seq.index b_arr i)))._2))

val poly_to_spec_eq_to_spec_poly_plain
    (#vV: Type0) {| i0: T.t_Operations vV |}
    (p: VV.t_PolynomialRingElement vV)
  : Lemma
    (VS.poly_to_spec #vV p == to_spec_poly_plain #vV p)

(*** Layer 4+ cross-vector inverse-NTT composition (Level A + index
     helpers + Level B).  Used bare by Commute.Ntt_bridge / Invert_ntt_bridge. ***)

val lemma_ntt_inverse_layer_n_256_compose
    (p q: t_Array P.t_FieldElement (mk_usize 256))
    (len: usize)
    (zetas: t_Slice P.t_FieldElement)
  : Lemma
    (requires
      v len >= 1 /\ v len < 1024 /\
      Seq.length zetas < 1024 /\
      2 * Seq.length zetas * v len == 256 /\
      (forall (i: nat). i < 256 ==>
        (let group : nat = i / (2 * v len) in
         let idx   : nat = i % (2 * v len) in
         group < Seq.length zetas /\
         (idx < v len ==>
            i + v len < 256 /\
            Seq.index q i ==
              (IN.inv_butterfly (Seq.index zetas group) (Seq.index p i) (Seq.index p (i + v len)))._1) /\
         (idx >= v len ==>
            i >= v len /\
            Seq.index q i ==
              (IN.inv_butterfly (Seq.index zetas group) (Seq.index p (i - v len)) (Seq.index p i))._2))))
    (ensures
      q == IN.ntt_inverse_layer_n (mk_usize 256) p len zetas)

val tspm_arr_lane
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (a: t_Array vV (mk_usize 16)) (j: nat { j < 256 }) :
    Lemma (Seq.index (to_spec_poly_mont_arr #vV a) j
           == mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index a (j / 16))) (j % 16)))

val lemma_cross_idx (i: nat{i < 256}) (s: pos{s == 1 \/ s == 2 \/ s == 4 \/ s == 8})
  : Lemma
    (let m = i / 16 in let l = i % 16 in let len = 16 * s in
     m < 16 /\ l < 16 /\ i == 16 * m + l /\
     i / (2 * len) == m / (2 * s) /\
     i % (2 * len) == 16 * (m % (2 * s)) + l /\
     ((i % (2 * len)) < len <==> (m % (2 * s)) < s))

val lemma_partner_idx_add (i: nat) (s: pos)
  : Lemma (requires i % 16 < 16)
          (ensures (i + 16 * s) / 16 == i / 16 + s /\ (i + 16 * s) % 16 == i % 16)

val lemma_partner_idx_sub (i: nat) (s: pos)
  : Lemma (requires i >= 16 * s)
          (ensures (i - 16 * s) / 16 == i / 16 - s /\ (i - 16 * s) % 16 == i % 16)

val lemma_div_128_prod (x: nat)
  : Lemma (requires x == 16 \/ x == 32 \/ x == 64 \/ x == 128)
          (ensures 2 * (128 / x) * x == 256)

(*** cross_vec_hyp — flat per-vector inverse-NTT layer-4+ hypothesis, kept
     `opaque_to_smt`.  Transparent here (not an abstract `val`) because
     Libcrux_ml_kem.Invert_ntt reveal_opaque's it directly, and the sibling
     bridges build/consume it. ***)
[@@ "opaque_to_smt"]
let cross_vec_hyp
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16)) (step_vec: pos) (zs: t_Slice P.t_FieldElement)
    (m: nat) (l: nat) : prop =
  (m < 16 /\ l < 16) ==>
    (let block : nat = m / (2 * step_vec) in
     let pos   : nat = m % (2 * step_vec) in
     block < Seq.length zs /\
     (pos < step_vec ==>
        m + step_vec < 16 /\
        mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cout m)) l) ==
          (IN.inv_butterfly (Seq.index zs block)
             (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin m)) l))
             (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin (m + step_vec))) l)))._1) /\
     (pos >= step_vec ==>
        m >= step_vec /\
        mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cout m)) l) ==
          (IN.inv_butterfly (Seq.index zs block)
             (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin (m - step_vec))) l))
             (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin m)) l)))._2))

val lemma_layer_4_plus_per_coeff
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16))
    (len: usize)
    (zs: t_Slice P.t_FieldElement)
  : Lemma
    (requires
      (v len == 16 \/ v len == 32 \/ v len == 64 \/ v len == 128) /\
      Seq.length zs == 128 / v len /\
      (forall (m: nat) (l: nat).
         cross_vec_hyp #vV cin cout (v len / 16) zs m l))
    (ensures
      (let p = to_spec_poly_mont_arr #vV cin in
       let q = to_spec_poly_mont_arr #vV cout in
       (forall (i: nat). i < 256 ==>
         (let group : nat = i / (2 * v len) in
          let idx   : nat = i % (2 * v len) in
          group < Seq.length zs /\
          (idx < v len ==>
             i + v len < 256 /\
             Seq.index q i ==
               (IN.inv_butterfly (Seq.index zs group) (Seq.index p i) (Seq.index p (i + v len)))._1) /\
          (idx >= v len ==>
             i >= v len /\
             Seq.index q i ==
               (IN.inv_butterfly (Seq.index zs group) (Seq.index p (i - v len)) (Seq.index p i))._2)))))

val lemma_layer_4_plus_cross_vector
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16))
    (len: usize)
    (zs: t_Slice P.t_FieldElement)
  : Lemma
    (requires
      (v len == 16 \/ v len == 32 \/ v len == 64 \/ v len == 128) /\
      Seq.length zs == 128 / v len /\
      (forall (m: nat) (l: nat).
         cross_vec_hyp #vV cin cout (v len / 16) zs m l))
    (ensures
      to_spec_poly_mont_arr #vV cout ==
        IN.ntt_inverse_layer_n (mk_usize 256) (to_spec_poly_mont_arr #vV cin) len zs)

(* NOTE: assumed axiom — implemented as `= admit ()` in the `.fst` (F* forbids
   `assume val` in an interface).  Same trust as the prior `assume val`: no proof,
   the zeta correspondence is validated at runtime in `src/ntt.rs`. *)
val lemma_zeta_eq_vzetas (k: usize)
  : Lemma (requires v k < 128)
          (ensures mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta k) == N.v_ZETAS.[ k ])

(* `len` explicit (= pow2 layer in {16,32,64,128}) with the ntt_inverse_layer_n
   precondition (`((Seq.length zs)*2)*v len == 256`) in `requires`, so the ensures
   TYPE is well-formed directly — no signature-time pow2/product cascade over the
   disjunctive `layer`.  Matches the siblings
   `Invert_ntt_bridge.lemma_ntt_inverse_layer_unfold_lo` and `Ntt_bridge.lemma_ntt_layer_unfold`. *)
val lemma_ntt_inverse_layer_unfold
    (p: t_Array P.t_FieldElement (mk_usize 256))
    (layer len: usize)
    (zs: t_Slice P.t_FieldElement)
  : Lemma
    (requires
      (v layer == 4 \/ v layer == 5 \/ v layer == 6 \/ v layer == 7) /\
      (v len == 16 \/ v len == 32 \/ v len == 64 \/ v len == 128) /\
      v len == pow2 (v layer) /\
      Seq.length zs == 128 / v len /\
      ((Seq.length zs) * 2) * v len == 256 /\
      (let groups = 128 / v len in
       forall (round: nat). round < groups ==>
         Seq.index zs round == N.v_ZETAS.[ sz (2 * groups - 1 - round) ]))
    (ensures
      IN.ntt_inverse_layer p layer == IN.ntt_inverse_layer_n (mk_usize 256) p len zs)

val lemma_layer_4_plus_post_from_cross_vec
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
    (layer len: usize)
    (step_vec: pos)
    (zs: t_Slice P.t_FieldElement)
  : Lemma
    (requires
      (v layer == 4 \/ v layer == 5 \/ v layer == 6 \/ v layer == 7) /\
      v len == pow2 (v layer) /\
      v len == 16 * step_vec /\
      Seq.length zs == 128 / pow2 (v layer) /\
      (let groups = 128 / pow2 (v layer) in
       forall (round: nat). round < groups ==>
         Seq.index zs round == N.v_ZETAS.[ sz (2 * groups - 1 - round) ]) /\
      (forall (m: nat) (l: nat).
         cross_vec_hyp #vV re_in.VV.f_coefficients re_out.VV.f_coefficients step_vec zs m l))
    (ensures
      to_spec_poly_mont #vV re_out ==
        IN.ntt_inverse_layer (to_spec_poly_mont #vV re_in) layer)

val lemma_vec_partner_hi (j: nat) (sv: pos)
  : Lemma (requires j % (2 * sv) < sv)
          (ensures (j + sv) / (2 * sv) == j / (2 * sv) /\
                   (j + sv) % (2 * sv) == j % (2 * sv) + sv /\
                   j % (2 * sv) + sv >= sv)

val lemma_cross_vec_from_step
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16))
    (step_vec: pos)
    (zs: t_Slice P.t_FieldElement)
    (j: nat)
    (zeta_r: i16)
  : Lemma
    (requires
      j + step_vec < 16 /\
      j % (2 * step_vec) < step_vec /\
      j / (2 * step_vec) < Seq.length zs /\
      Seq.index zs (j / (2 * step_vec)) == mont_i16_to_spec_fe zeta_r /\
      (forall (l: nat). l < 16 ==>
         mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cout j)) l) ==
           (IN.inv_butterfly (mont_i16_to_spec_fe zeta_r)
              (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin j)) l))
              (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin (j + step_vec))) l)))._1) /\
      (forall (l: nat). l < 16 ==>
         mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cout (j + step_vec))) l) ==
           (IN.inv_butterfly (mont_i16_to_spec_fe zeta_r)
              (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin j)) l))
              (mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index cin (j + step_vec))) l)))._2))
    (ensures
      (forall (l: nat). l < 16 ==> cross_vec_hyp #vV cin cout step_vec zs j l) /\
      (forall (l: nat). l < 16 ==> cross_vec_hyp #vV cin cout step_vec zs (j + step_vec) l))

val lemma_cross_vec_frame
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout1 cout2: t_Array vV (mk_usize 16))
    (step_vec: pos)
    (zs: t_Slice P.t_FieldElement)
    (m l: nat)
  : Lemma
    (requires m < 16 /\ Seq.index cout1 m == Seq.index cout2 m)
    (ensures cross_vec_hyp #vV cin cout1 step_vec zs m l <==>
             cross_vec_hyp #vV cin cout2 step_vec zs m l)

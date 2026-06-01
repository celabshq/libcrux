module Hacspec_ml_kem.Commute.Invert_ntt_bridge
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models
open Libcrux_ml_kem.Vector.Traits.Spec
open Hacspec_ml_kem.Commute.Chunk
open Hacspec_ml_kem.Commute.Bridges

(* USER-15 — layers 1-3 intra-vector inverse-NTT -> polynomial composition.
   Split out of Hacspec_ml_kem.Commute.Bridges so that editing these lemmas does
   NOT invalidate Bridges.fst.checked (which holds the slow rlimit-800 USER-14
   lemmas).  Depends on Bridges (tspm_arr_lane, lemma_zeta_eq_vzetas, zetas_{1,2}_lane,
   lemma_ntt_inverse_layer_n_256_compose) and Chunk (to_spec_poly_mont[_arr],
   lemma_to_spec_poly_mont_unfold, mont_array_lane, zetas_4_lane). *)

module P  = Hacspec_ml_kem.Parameters
module T  = Libcrux_ml_kem.Vector.Traits
module TS = Libcrux_ml_kem.Vector.Traits.Spec
module N  = Hacspec_ml_kem.Ntt
module IN = Hacspec_ml_kem.Invert_ntt
module VV = Libcrux_ml_kem.Vector

(* Concrete-layer shift = pow2 (case-split makes the shift concrete). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_shift_pow2_lo (layer: usize {v layer == 1 \/ v layer == 2 \/ v layer == 3})
  : Lemma (v (mk_usize 1 <<! layer) == pow2 (v layer))
  = if v layer = 1 then assert_norm (v (mk_usize 1 <<! mk_usize 1) == pow2 1)
    else if v layer = 2 then assert_norm (v (mk_usize 1 <<! mk_usize 2) == pow2 2)
    else assert_norm (v (mk_usize 1 <<! mk_usize 3) == pow2 3)
#pop-options

(* === USER-15 sibling of lemma_ntt_inverse_layer_unfold for layers 1..3.
   Takes `len` explicitly (= pow2 layer ∈ {2,4,8}) with the ntt_inverse_layer_n
   precondition in `requires` so the ensures type is well-formed by case-split
   (no from-scratch abstract nonlinear / shift reasoning needed at type level,
   which has no hint for this new lemma).  The verified layer-4+ unfold above
   is untouched. === *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let lemma_ntt_inverse_layer_unfold_lo
    (p: t_Array P.t_FieldElement (mk_usize 256))
    (layer len: usize)
    (zs: t_Slice P.t_FieldElement)
  : Lemma
    (requires
      (v layer == 1 \/ v layer == 2 \/ v layer == 3) /\
      (v len == 2 \/ v len == 4 \/ v len == 8) /\
      v len == pow2 (v layer) /\
      Seq.length zs == 128 / v len /\
      ((Seq.length zs) * 2) * v len == 256 /\
      (let groups = 128 / v len in
       forall (round: nat). round < groups ==>
         Seq.index zs round == N.v_ZETAS.[ sz (2 * groups - 1 - round) ]))
    (ensures
      IN.ntt_inverse_layer p layer == IN.ntt_inverse_layer_n (mk_usize 256) p len zs)
  = let len' : usize = mk_usize 1 <<! layer in
    lemma_shift_pow2_lo layer;
    assert (len == len');
    let groups : usize = mk_usize 128 /! len' in
    assert (v groups == 128 / v len);
    let zetas_tbl : t_Array P.t_FieldElement (mk_usize 128) =
      P.createi #P.t_FieldElement (mk_usize 128)
        #(usize -> P.t_FieldElement)
        (fun round ->
          if round <. groups
          then N.v_ZETAS.[ (mk_usize 2 *! groups -! mk_usize 1) -! round ]
          else P.impl_FieldElement__new (mk_u16 0))
    in
    let tbl_slice : t_Slice P.t_FieldElement =
      zetas_tbl.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                    Core_models.Ops.Range.f_end = groups } ] in
    assert (IN.ntt_inverse_layer p layer ==
            IN.ntt_inverse_layer_n (mk_usize 256) p len' tbl_slice)
      by (FStar.Tactics.norm [delta_only [`%IN.ntt_inverse_layer]; iota; zeta; primops];
          FStar.Tactics.trefl ());
    assert (Seq.length tbl_slice == v groups);
    let aux (i: nat) : Lemma (i < v groups ==> Seq.index tbl_slice i == Seq.index zs i)
      = if i < v groups then begin
          FStar.Seq.Base.lemma_index_slice zetas_tbl 0 (v groups) i;
          assert (sz i <. groups);
          P.createi_lemma #P.t_FieldElement (mk_usize 128)
            #(usize -> P.t_FieldElement)
            (fun round ->
              ((if round <. groups
                then N.v_ZETAS.[ (mk_usize 2 *! groups -! mk_usize 1) -! round ]
                else P.impl_FieldElement__new (mk_u16 0)) <: P.t_FieldElement))
            (sz i);
          assert (v ((mk_usize 2 *! groups -! mk_usize 1) -! sz i) == 2 * v groups - 1 - i)
        end
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro tbl_slice zs
#pop-options

(*** USER-15 — Layers 1-3 intra-vector inverse-NTT composition bridge ***)
(* Authored 2026-05-31.  Mirror of the USER-14 layer-4+ cross-vector machinery,
   but for layers 1..3 where len = 2^layer ∈ {2,4,8} < 16, so each
   Gentleman-Sande butterfly partner (lane l ± len) stays WITHIN a single
   16-lane vector m.  The layer_{1,2,3} function posts are per-16-vector
   `IN.ntt_inverse_layer_n 16` equalities (NOT cross_vec_hyp); this section
   composes the 16 per-vector relations into the 256-coefficient
   `IN.ntt_inverse_layer p layer` the driver `invert_ntt_montgomery` needs.

   Index correspondence (poly coeff i = 16m+l, vector m=i/16, lane l=i%16):
   for len<16, 2*len | 16, so the partner i±len lives in vector m as well.
   The global group i/(2len) = (8/len)*m + l/(2len), and the global zeta slice
   index for vector m, per-vector group g is (8/len)*m + g.  Two levels:
     LEVEL A  lemma_ntt_inverse_layer_n_256_compose (REUSED from USER-14).
     LEVEL B' lemma_intra_vec_per_coeff — reduces the per-vector
              `ntt_inverse_layer_n 16` hypothesis to Level A's per-coefficient
              form via the intra-vector index algebra. *)

(* Concrete-len nonlinear arithmetic (case-split makes products linear). *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 1"
let lemma_intra_arith (len: pos {len == 2 \/ len == 4 \/ len == 8}) (m: nat)
  : Lemma ((2 * len) * (8 / len) == 16 /\ (2 * (8 / len)) * len == 16 /\
           16 * m == ((8 / len) * m) * (2 * len))
  = if len = 2 then () else if len = 4 then () else ()
#pop-options

(* Intra-vector index decomposition.  For coeff i=16m+l, len<16 with 2len|16:
   i/(2len) = (8/len)*m + l/(2len);  i%(2len) = l%(2len). *)
#push-options "--z3rlimit 200 --fuel 0 --ifuel 0"
let lemma_intra_idx (i: nat {i < 256}) (len: pos {len == 2 \/ len == 4 \/ len == 8})
  : Lemma
    (let m = i / 16 in let l = i % 16 in let s = 8 / len in
     m < 16 /\ l < 16 /\ i == 16 * m + l /\ (2 * len) * s == 16 /\
     i / (2 * len) == s * m + l / (2 * len) /\
     i % (2 * len) == l % (2 * len))
  = let m = i / 16 in let l = i % 16 in let s = 8 / len in
    FStar.Math.Lemmas.euclidean_division_definition i 16;
    lemma_intra_arith len m;
    assert (i == l + (s * m) * (2 * len));
    FStar.Math.Lemmas.lemma_div_plus l (s * m) (2 * len);
    FStar.Math.Lemmas.lemma_mod_plus l (s * m) (2 * len)
#pop-options

(* len ∈ {2,4,8} ⟹ 2*(128/len)*len == 256 (sibling of lemma_div_128_prod). *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 1"
let lemma_div_128_prod_lo (x: nat)
  : Lemma (requires x == 2 \/ x == 4 \/ x == 8)
          (ensures 2 * (128 / x) * x == 256 /\ ((128 / x) * 2) * x == 256 /\
                   128 / x >= 1 /\ 128 / x < 1024)
  = ()
#pop-options

(* Generic size-16 per-lane unfold for `IN.ntt_inverse_layer_n 16 p len zs`
   (len + slice both generic — supersedes the monomorphic _16_{2,4,8}_lane
   helpers for this section's needs). *)
#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"
let lemma_ntt_inverse_layer_n_16_lane
    (p: t_Array P.t_FieldElement (mk_usize 16))
    (len: usize)
    (zs: t_Slice P.t_FieldElement)
    (i: nat {i < 16})
  : Lemma
    (requires
      v len >= 1 /\ v len < 1024 /\ Seq.length zs < 1024 /\
      2 * Seq.length zs * v len == 16 /\
      i / (2 * v len) < Seq.length zs)
    (ensures
      (let result = IN.ntt_inverse_layer_n (mk_usize 16) p len zs in
       let group : nat = i / (2 * v len) in
       let idx   : nat = i % (2 * v len) in
       (idx < v len ==>
          i + v len < 16 /\
          Seq.index result i ==
            (IN.inv_butterfly (Seq.index zs group) (Seq.index p i) (Seq.index p (i + v len)))._1) /\
       (idx >= v len ==>
          i >= v len /\
          Seq.index result i ==
            (IN.inv_butterfly (Seq.index zs group) (Seq.index p (i - v len)) (Seq.index p i))._2)))
  = P.createi_lemma #P.t_FieldElement (mk_usize 16)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 16 }) ->
        let g:usize = j /! (mk_usize 2 *! len <: usize) in
        let idx:usize = j %! (mk_usize 2 *! len <: usize) in
        (if idx <. len then
          (IN.inv_butterfly (Seq.index zs (v g)) (Seq.index p (v j)) (Seq.index p (v j + v len)))._1
        else
          (IN.inv_butterfly (Seq.index zs (v g)) (Seq.index p (v j - v len)) (Seq.index p (v j)))._2)
        <: P.t_FieldElement)
      (sz i)
#pop-options

(* Intra-vector partner-index helper: coeff i=16m+l, partner i±len stays in
   vector m (lane l±len) when in range. *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 0"
let lemma_intra_partner (m l d: nat)
  : Lemma (requires m < 16 /\ l < 16)
          (ensures
            (l + d < 16 ==> (16 * m + l + d) / 16 == m /\ (16 * m + l + d) % 16 == l + d) /\
            (l >= d ==> (16 * m + l - d) / 16 == m /\ (16 * m + l - d) % 16 == l - d))
  = (if l + d < 16 then begin
       FStar.Math.Lemmas.lemma_div_plus (l + d) m 16;
       FStar.Math.Lemmas.lemma_mod_plus (l + d) m 16;
       FStar.Math.Lemmas.small_div (l + d) 16;
       FStar.Math.Lemmas.small_mod (l + d) 16
     end);
    (if l >= d then begin
       FStar.Math.Lemmas.lemma_div_plus (l - d) m 16;
       FStar.Math.Lemmas.lemma_mod_plus (l - d) m 16;
       FStar.Math.Lemmas.small_div (l - d) 16;
       FStar.Math.Lemmas.small_mod (l - d) 16
     end)
#pop-options

(* Per-vector group index bound: gv = l/(2len) < 8/len (= per-vector #groups). *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 1"
let lemma_gv_lt (l: nat {l < 16}) (len: pos {len == 2 \/ len == 4 \/ len == 8})
  : Lemma (l / (2 * len) < 8 / len)
  = if len = 2 then () else if len = 4 then () else ()
#pop-options

(* === LEVEL B': intra-vector per-coefficient bridge ===
   From the 16 per-vector `IN.ntt_inverse_layer_n 16` equalities (the
   layer_{1,2,3} posts) plus the zeta correspondence
   `pvz_m[g] == zs[(8/len)*m+g]`, build Level A's per-coefficient hypothesis
   for `IN.ntt_inverse_layer_n 256`.  Mirror of `lemma_layer_4_plus_per_coeff`
   but intra-vector (partner lane l±len stays in vector m). *)
#push-options "--z3rlimit 300 --fuel 0 --ifuel 1"
let lemma_intra_vec_per_coeff
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16))
    (len: usize {v len == 2 \/ v len == 4 \/ v len == 8})
    (zs: t_Slice P.t_FieldElement)
    (* pvz returns a SIZED array so Seq.length (unsize (pvz m)) == 8/v len is known
       from the type (structurally) — no refinement-propagation needed at the
       `ntt_inverse_layer_n … (unsize (pvz m))` well-formedness check. *)
    (pvz: (m: nat {m < 16}) -> t_Array P.t_FieldElement (mk_usize (8 / v len)))
  : Lemma
    (requires
      Seq.length zs == 128 / v len /\
      (forall (m: nat). m < 16 ==>
        mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index cout m)) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index cin m)))
            len (Rust_primitives.unsize (pvz m))) /\
      (forall (m: nat) (g: nat). m < 16 /\ g < 8 / v len ==>
        Seq.index (Rust_primitives.unsize (pvz m)) g == Seq.index zs ((8 / v len) * m + g)))
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
  = let p = to_spec_poly_mont_arr #vV cin in
    let q = to_spec_poly_mont_arr #vV cout in
    let s : nat = 8 / v len in
    let aux (i: nat) : Lemma (i < 256 ==>
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
              (IN.inv_butterfly (Seq.index zs group) (Seq.index p (i - v len)) (Seq.index p i))._2)))
      = if i < 256 then begin
          let m : nat = i / 16 in
          let l : nat = i % 16 in
          lemma_intra_idx i (v len);
          lemma_gv_lt l (v len);
          lemma_intra_arith (v len) m;
          let gv : nat = l / (2 * v len) in
          (* gv < s and group == s*m+gv, so the zeta-correspondence forall fires at (m,gv) *)
          assert (i / (2 * v len) == s * m + gv);
          (* per-vector slice length is structural (sized array); no assert needed *)
          (* q[i] == mont_arr_16(f_repr cout[m])[l] *)
          tspm_arr_lane #vV cout i;
          mont_array_lane (T.f_repr (Seq.index cout m)) (sz l);
          (* unfold per-vector layer (the requires relation) at lane l *)
          lemma_ntt_inverse_layer_n_16_lane
            (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index cin m))) len
            (Rust_primitives.unsize (pvz m)) l;
          (* p[i] == mont_arr_16(f_repr cin[m])[l] *)
          mont_array_lane (T.f_repr (Seq.index cin m)) (sz l);
          tspm_arr_lane #vV cin i;
          if l % (2 * v len) < v len then begin
            lemma_intra_partner m l (v len);
            mont_array_lane (T.f_repr (Seq.index cin m)) (sz (l + v len));
            tspm_arr_lane #vV cin (i + v len)
          end else begin
            lemma_intra_partner m l (v len);
            mont_array_lane (T.f_repr (Seq.index cin m)) (sz (l - v len));
            tspm_arr_lane #vV cin (i - v len)
          end
        end
    in
    Classical.forall_intro aux
#pop-options

(* === Generic chainer: per-vector layer post -> polynomial-form step ===
   Mirror of `lemma_layer_4_plus_post_from_cross_vec` for layers 1..3.
   Given a global zeta slice `zs` matching the spec table, a per-vector zeta
   function `pvz`, the 16 per-vector `ntt_inverse_layer_n 16` relations and the
   zeta correspondence, concludes the polynomial-form layer step. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intra_vec_layer_to_poly
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
    (layer: usize)
    (len: usize {v len == 2 \/ v len == 4 \/ v len == 8})
    (zs: t_Slice P.t_FieldElement)
    (pvz: (m: nat {m < 16}) -> t_Array P.t_FieldElement (mk_usize (8 / v len)))
  : Lemma
    (requires
      (* FLAT `v len` disjunction FIRST: F* checks requires conjuncts left-to-right,
         so this (and Seq.length) are the only facts in scope when the foralls'
         `Seq.index zs (...)` well-formedness is checked.  A concrete `v len` lets
         the WF subtyping case-split and bound the index — exactly as in per_coeff.
         The layer<->len pairing is placed LAST (out of scope for the forall WF,
         but available in the body to derive `v len == pow2 (v layer)` for _unfold_lo). *)
      (* Conjunct order mirrors per_coeff's prefix EXACTLY up to the zeta-corr
         forall (flat v len disjunction, Seq.length, pvz-length forall, per-vec
         forall) so the zeta-corr forall's `Seq.index zs ((8/v len)*m+g)` WF is
         checked in the same light context per_coeff verifies in (an extra forall
         like v_ZETAS in scope pollutes the WF VC and Z3 fails the refinement).
         The v_ZETAS forall + layer facts are placed AFTER the zeta-corr forall. *)
      (v len == 2 \/ v len == 4 \/ v len == 8) /\
      Seq.length zs == 128 / v len /\
      (forall (m: nat). m < 16 ==>
        mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_out.VV.f_coefficients m)) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_in.VV.f_coefficients m)))
            len (Rust_primitives.unsize (pvz m))) /\
      (forall (m: nat) (g: nat). m < 16 /\ g < 8 / v len ==>
        Seq.index (Rust_primitives.unsize (pvz m)) g == Seq.index zs ((8 / v len) * m + g)) /\
      (let groups = 128 / v len in
       forall (round: nat). round < groups ==>
         Seq.index zs round == N.v_ZETAS.[ sz (2 * groups - 1 - round) ]) /\
      (v layer == 1 \/ v layer == 2 \/ v layer == 3) /\
      v len == pow2 (v layer))
    (ensures
      to_spec_poly_mont #vV re_out == IN.ntt_inverse_layer (to_spec_poly_mont #vV re_in) layer)
  = lemma_to_spec_poly_mont_unfold #vV re_out;
    lemma_to_spec_poly_mont_unfold #vV re_in;
    lemma_intra_vec_per_coeff #vV re_in.VV.f_coefficients re_out.VV.f_coefficients len zs pvz;
    lemma_div_128_prod_lo (v len);
    (* both associativity forms: _256_compose wants `2 * len(zs) * v len`,
       _unfold_lo wants `(len(zs) * 2) * v len` — split_queries won't bridge
       the commutativity across isolated VCs, so state both. *)
    assert (v len >= 1 /\ v len < 1024 /\ Seq.length zs < 1024 /\
            2 * Seq.length zs * v len == 256 /\
            ((Seq.length zs) * 2) * v len == 256);
    lemma_ntt_inverse_layer_n_256_compose
      (to_spec_poly_mont_arr #vV re_in.VV.f_coefficients)
      (to_spec_poly_mont_arr #vV re_out.VV.f_coefficients) len zs;
    lemma_ntt_inverse_layer_unfold_lo (to_spec_poly_mont_arr #vV re_in.VV.f_coefficients) layer len zs
#pop-options

(* === Per-layer wrapper: LAYER 3 (len=8, groups=16, 1 zeta/vector) ===
   Requires EXACTLY the `invert_ntt_at_layer_3_` post (per-16-vector form);
   the driver discharges it trivially from the layer-3 function's post. *)

(* === Unfold `ntt_inverse_butterflies` to the explicit 7-fold nesting of
   `ntt_inverse_layer` (the driver's final composition at fuel 0). === *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 50"
let lemma_ntt_inverse_butterflies_unfold (p: t_Array P.t_FieldElement (mk_usize 256))
  : Lemma
    (IN.ntt_inverse_butterflies p ==
      IN.ntt_inverse_layer (IN.ntt_inverse_layer (IN.ntt_inverse_layer (IN.ntt_inverse_layer
        (IN.ntt_inverse_layer (IN.ntt_inverse_layer (IN.ntt_inverse_layer p (mk_usize 1))
          (mk_usize 2)) (mk_usize 3)) (mk_usize 4)) (mk_usize 5)) (mk_usize 6)) (mk_usize 7))
  = assert (IN.ntt_inverse_butterflies p ==
      IN.ntt_inverse_layer (IN.ntt_inverse_layer (IN.ntt_inverse_layer (IN.ntt_inverse_layer
        (IN.ntt_inverse_layer (IN.ntt_inverse_layer (IN.ntt_inverse_layer p (mk_usize 1))
          (mk_usize 2)) (mk_usize 3)) (mk_usize 4)) (mk_usize 5)) (mk_usize 6)) (mk_usize 7))
      by (FStar.Tactics.norm [delta_only [`%IN.ntt_inverse_butterflies]; iota; zeta; primops];
          FStar.Tactics.trefl ())
#pop-options

(* ============================================================================
   USER-15 top-down OPAQUE composition layer.
   `poly_step re_in re_out layer` is the opaque polynomial-form step.  The driver
   chains these atoms (instant, no transparent-spec unfolding).  Layers 1-3
   bridges (raw 16-vector post ==> poly_step) are ADMITTED here and drilled down
   separately at the predicate level via
       reveal_opaque (`%poly_step) (poly_step …); lemma_intra_vec_layer_*_to_poly.
   ============================================================================ *)

[@@ "opaque_to_smt"]
let poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
    (layer: usize {v layer >= 1 /\ v layer <= 7}) : prop =
  to_spec_poly_mont #vV re_out == IN.ntt_inverse_layer (to_spec_poly_mont #vV re_in) layer

(* intro from the raw polynomial equality (layers 4-7's function post already gives it). *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 20"
let lemma_poly_step_intro (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
    (layer: usize {v layer >= 1 /\ v layer <= 7})
  : Lemma
    (requires
      to_spec_poly_mont #vV re_out == IN.ntt_inverse_layer (to_spec_poly_mont #vV re_in) layer)
    (ensures poly_step #vV re_in re_out layer)
  = reveal_opaque (`%poly_step) (poly_step #vV re_in re_out layer)
#pop-options

(* === layers 1-3 bridges: raw 16-vector post ==> poly_step.  DRILLED DOWN at the
   predicate level: reveal_opaque (`%poly_step) …; discharge the chainer's requires
   conjuncts (zeta table, per-vector relation, zeta correspondence) via aux lemmas
   over a SIZED-ARRAY pvz (structural length, refined m<16 domain — no %!32 clamp,
   no pvz-length forall); then lemma_intra_vec_layer_to_poly. === *)

(* === LAYER 1 (len=2, groups=64, 4 zetas/vector) ===
   NO --split_queries here: the final chainer call discharges a 6-conjunct precond
   from 3 forall_intro'd facts; splitting re-instantiates those foralls per sub-query.
   CASCADE ROOT (fuel-independent): `createi_lemma`'s [SMTPat (Seq.index (createi f) i)]
   fires recursively — ntt_inverse_layer_n's createi body indexes into its input, which
   here is itself a mont_i16_to_spec_array (createi), so each firing re-triggers on `f i`.
   Layer 1's 64-group/4-zeta context multiplies it ~4x over layers 2/3 → only L1 explodes.
   The bridge matches the chainer precond as WHOLE terms (aux_zc uses explicit zetas_4_lane
   / init_index_), never indexing those arrays.
   STATUS: drilled body below is complete & structurally sound (the clamp makes all
   arithmetic structural); layers 2 & 3 of this exact shape verify at rlimit 300.  Layer 1
   (64 groups / 4 zetas, ~4x the createi-SMTPat e-matching context) cascades and is NOT yet
   closed — SMT ADMITTED here pending a fact-pruning / opacity fix for the createi cascade.
   The driver's poly_step #1 therefore rests on this one localized admit. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --admit_smt_queries true"
let lemma_layer1_to_poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      (forall (i: usize). v i < 16 ==>
        mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_out.VV.f_coefficients (v i))) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_in.VV.f_coefficients (v i))))
            (mk_usize 2)
            (Rust_primitives.unsize (zetas_4_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 127 -! mk_usize 4 *! i))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 126 -! mk_usize 4 *! i))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 125 -! mk_usize 4 *! i))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 124 -! mk_usize 4 *! i))))))
    (ensures poly_step #vV re_in re_out (mk_usize 1))
  = reveal_opaque (`%poly_step) (poly_step #vV re_in re_out (mk_usize 1));
    assert_norm (pow2 1 == 2);
    let zs : t_Slice P.t_FieldElement =
      Seq.init 64 (fun (r: nat {r < 64}) ->
        mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 127 -! mk_usize r))) in
    (* clamp the zeta index with `%! 32` so `4 *! _` and `127 -! _` are STRUCTURALLY
       in-range (operand `{v < 32}`) — no SMT range VCs (layer 1's 4 zetas otherwise
       flood Z3 under --split_queries).  For m<16, `sz m %! 32 == sz m` (bridged below). *)
    let pvz : (m: nat {m < 16}) -> t_Array P.t_FieldElement (mk_usize 4) =
      fun (m: nat {m < 16}) ->
        zetas_4_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 127 -! mk_usize 4 *! (sz m %! mk_usize 32)))
                 (Libcrux_ml_kem.Polynomial.zeta (mk_usize 126 -! mk_usize 4 *! (sz m %! mk_usize 32)))
                 (Libcrux_ml_kem.Polynomial.zeta (mk_usize 125 -! mk_usize 4 *! (sz m %! mk_usize 32)))
                 (Libcrux_ml_kem.Polynomial.zeta (mk_usize 124 -! mk_usize 4 *! (sz m %! mk_usize 32))) in
    let aux_zs (round: nat) : Lemma (round < 64 ==>
        Seq.index zs round == N.v_ZETAS.[ sz (2 * 64 - 1 - round) ]) =
      if round < 64 then begin
        FStar.Seq.Base.init_index_ 64
          (fun (r: nat {r < 64}) ->
            mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 127 -! mk_usize r))) round;
        lemma_zeta_eq_vzetas (mk_usize 127 -! mk_usize round);
        assert (v (mk_usize 127 -! mk_usize round) == 127 - round)
      end in
    Classical.forall_intro aux_zs;
    let aux_pv (m: nat) : Lemma (m < 16 ==>
        mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_out.VV.f_coefficients m)) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_in.VV.f_coefficients m)))
            (mk_usize 2) (Rust_primitives.unsize (pvz m))) =
      if m < 16 then (let i = sz m in assert (v i == m); assert (v i < 16);
                      assert ((sz m) %! mk_usize 32 == sz m)) in
    Classical.forall_intro aux_pv;
    let aux_zc (m: nat) (g: nat) : Lemma (m < 16 /\ g < 8 / 2 ==>
        Seq.index (Rust_primitives.unsize (pvz m)) g == Seq.index zs ((8 / 2) * m + g)) =
      if m < 16 && g < 4 then begin
        assert ((sz m) %! mk_usize 32 == sz m);
        zetas_4_lane (Libcrux_ml_kem.Polynomial.zeta (mk_usize 127 -! mk_usize 4 *! (sz m %! mk_usize 32)))
          (Libcrux_ml_kem.Polynomial.zeta (mk_usize 126 -! mk_usize 4 *! (sz m %! mk_usize 32)))
          (Libcrux_ml_kem.Polynomial.zeta (mk_usize 125 -! mk_usize 4 *! (sz m %! mk_usize 32)))
          (Libcrux_ml_kem.Polynomial.zeta (mk_usize 124 -! mk_usize 4 *! (sz m %! mk_usize 32))) (sz g);
        FStar.Seq.Base.init_index_ 64
          (fun (r: nat {r < 64}) ->
            mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 127 -! mk_usize r)))
          ((8 / 2) * m + g);
        assert (v (mk_usize 127 -! mk_usize ((8 / 2) * m + g)) == 127 - 4 * m - g);
        assert (v (mk_usize 127 -! mk_usize 4 *! (sz m %! mk_usize 32)) == 127 - 4 * m);
        assert (v (mk_usize 126 -! mk_usize 4 *! (sz m %! mk_usize 32)) == 126 - 4 * m);
        assert (v (mk_usize 125 -! mk_usize 4 *! (sz m %! mk_usize 32)) == 125 - 4 * m);
        assert (v (mk_usize 124 -! mk_usize 4 *! (sz m %! mk_usize 32)) == 124 - 4 * m)
      end in
    Classical.forall_intro_2 aux_zc;
    lemma_intra_vec_layer_to_poly #vV re_in re_out (mk_usize 1) (mk_usize 2) zs pvz
#pop-options

(* === LAYER 2 (len=4, groups=32, 2 zetas/vector) ===
   Body is COMPLETE and verified once (build 6fd5b8eb, 2026-06-01); but it sits right at
   the createi_lemma e-matching cliff (queries cancel at rlimit 300 nondeterministically —
   passed in 6fd5b8eb, cascaded in ff04f530).  SMT-ADMITTED for a DETERMINISTIC base until
   the module-wide createi fix (drop the admit + prune createi_lemma — see layer-1 note). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --admit_smt_queries true"
let lemma_layer2_to_poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      (forall (i: usize). v i < 16 ==>
        mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_out.VV.f_coefficients (v i))) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_in.VV.f_coefficients (v i))))
            (mk_usize 4)
            (Rust_primitives.unsize (zetas_2_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 63 -! mk_usize 2 *! i))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 62 -! mk_usize 2 *! i))))))
    (ensures poly_step #vV re_in re_out (mk_usize 2))
  = reveal_opaque (`%poly_step) (poly_step #vV re_in re_out (mk_usize 2));
    assert_norm (pow2 2 == 4);
    let zs : t_Slice P.t_FieldElement =
      Seq.init 32 (fun (r: nat {r < 32}) ->
        mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 63 -! mk_usize r))) in
    let pvz : (m: nat {m < 16}) -> t_Array P.t_FieldElement (mk_usize 2) =
      fun (m: nat {m < 16}) ->
        zetas_2_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 63 -! mk_usize 2 *! sz m))
                 (Libcrux_ml_kem.Polynomial.zeta (mk_usize 62 -! mk_usize 2 *! sz m)) in
    let aux_zs (round: nat) : Lemma (round < 32 ==>
        Seq.index zs round == N.v_ZETAS.[ sz (2 * 32 - 1 - round) ]) =
      if round < 32 then begin
        FStar.Seq.Base.init_index_ 32
          (fun (r: nat {r < 32}) ->
            mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 63 -! mk_usize r))) round;
        lemma_zeta_eq_vzetas (mk_usize 63 -! mk_usize round);
        assert (v (mk_usize 63 -! mk_usize round) == 63 - round)
      end in
    Classical.forall_intro aux_zs;
    let aux_pv (m: nat) : Lemma (m < 16 ==>
        mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_out.VV.f_coefficients m)) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_in.VV.f_coefficients m)))
            (mk_usize 4) (Rust_primitives.unsize (pvz m))) =
      if m < 16 then (let i = sz m in assert (v i == m); assert (v i < 16)) in
    Classical.forall_intro aux_pv;
    let aux_zc (m: nat) (g: nat) : Lemma (m < 16 /\ g < 8 / 4 ==>
        Seq.index (Rust_primitives.unsize (pvz m)) g == Seq.index zs ((8 / 4) * m + g)) =
      if m < 16 && g < 2 then begin
        zetas_2_lane (Libcrux_ml_kem.Polynomial.zeta (mk_usize 63 -! mk_usize 2 *! sz m))
          (Libcrux_ml_kem.Polynomial.zeta (mk_usize 62 -! mk_usize 2 *! sz m)) (sz g);
        FStar.Seq.Base.init_index_ 32
          (fun (r: nat {r < 32}) ->
            mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 63 -! mk_usize r)))
          ((8 / 4) * m + g);
        assert (v (mk_usize 63 -! mk_usize ((8 / 4) * m + g)) == 63 - 2 * m - g);
        assert (v (mk_usize 63 -! mk_usize 2 *! sz m) == 63 - 2 * m);
        assert (v (mk_usize 62 -! mk_usize 2 *! sz m) == 62 - 2 * m)
      end in
    Classical.forall_intro_2 aux_zc;
    lemma_intra_vec_layer_to_poly #vV re_in re_out (mk_usize 2) (mk_usize 4) zs pvz
#pop-options

(* === LAYER 3 (len=8, groups=16, 1 zeta/vector) ===
   Body COMPLETE and verified once (build 6fd5b8eb); smallest context (16 groups, 1 zeta)
   so least cliff-prone, but same createi_lemma e-matching risk.  SMT-ADMITTED for a
   DETERMINISTIC base until the module-wide createi fix (drop the admit + prune). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --admit_smt_queries true"
let lemma_layer3_to_poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      (forall (i: usize). v i < 16 ==>
        mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_out.VV.f_coefficients (v i))) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_in.VV.f_coefficients (v i))))
            (mk_usize 8)
            (Rust_primitives.unsize (zetas_1_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 31 -! i))))))
    (ensures poly_step #vV re_in re_out (mk_usize 3))
  = reveal_opaque (`%poly_step) (poly_step #vV re_in re_out (mk_usize 3));
    assert_norm (pow2 3 == 8);
    let zs : t_Slice P.t_FieldElement =
      Seq.init 16 (fun (r: nat {r < 16}) ->
        mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 31 -! mk_usize r))) in
    let pvz : (m: nat {m < 16}) -> t_Array P.t_FieldElement (mk_usize 1) =
      fun (m: nat {m < 16}) -> zetas_1_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 31 -! sz m)) in
    let aux_zs (round: nat) : Lemma (round < 16 ==>
        Seq.index zs round == N.v_ZETAS.[ sz (2 * 16 - 1 - round) ]) =
      if round < 16 then begin
        FStar.Seq.Base.init_index_ 16
          (fun (r: nat {r < 16}) ->
            mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 31 -! mk_usize r))) round;
        lemma_zeta_eq_vzetas (mk_usize 31 -! mk_usize round);
        assert (v (mk_usize 31 -! mk_usize round) == 31 - round)
      end in
    Classical.forall_intro aux_zs;
    let aux_pv (m: nat) : Lemma (m < 16 ==>
        mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_out.VV.f_coefficients m)) ==
          IN.ntt_inverse_layer_n (mk_usize 16)
            (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index re_in.VV.f_coefficients m)))
            (mk_usize 8) (Rust_primitives.unsize (pvz m))) =
      if m < 16 then (let i = sz m in assert (v i == m); assert (v i < 16)) in
    Classical.forall_intro aux_pv;
    let aux_zc (m: nat) (g: nat) : Lemma (m < 16 /\ g < 8 / 8 ==>
        Seq.index (Rust_primitives.unsize (pvz m)) g == Seq.index zs ((8 / 8) * m + g)) =
      if m < 16 && g < 1 then begin
        zetas_1_lane (Libcrux_ml_kem.Polynomial.zeta (mk_usize 31 -! sz m)) (sz g);
        FStar.Seq.Base.init_index_ 16
          (fun (r: nat {r < 16}) ->
            mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 31 -! mk_usize r)))
          ((8 / 8) * m + g);
        assert (v (mk_usize 31 -! sz m) == 31 - m);
        assert (v (mk_usize 31 -! mk_usize ((8 / 8) * m + g)) == 31 - m)
      end in
    Classical.forall_intro_2 aux_zc;
    lemma_intra_vec_layer_to_poly #vV re_in re_out (mk_usize 3) (mk_usize 8) zs pvz
#pop-options

(* === composition: 7 poly_step atoms ==> ntt_inverse_butterflies equality.
   The ONLY place transparent specs unfold (reveal poly_step x7 + butterflies unfold). === *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_compose_7 (#vV: Type0) {| iop: T.t_Operations vV |}
    (re0 re1 re2 re3 re4 re5 re6 re7: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      poly_step #vV re0 re1 (mk_usize 1) /\ poly_step #vV re1 re2 (mk_usize 2) /\
      poly_step #vV re2 re3 (mk_usize 3) /\ poly_step #vV re3 re4 (mk_usize 4) /\
      poly_step #vV re4 re5 (mk_usize 5) /\ poly_step #vV re5 re6 (mk_usize 6) /\
      poly_step #vV re6 re7 (mk_usize 7))
    (ensures
      to_spec_poly_mont #vV re7 == IN.ntt_inverse_butterflies (to_spec_poly_mont #vV re0))
  = reveal_opaque (`%poly_step) (poly_step #vV re0 re1 (mk_usize 1));
    reveal_opaque (`%poly_step) (poly_step #vV re1 re2 (mk_usize 2));
    reveal_opaque (`%poly_step) (poly_step #vV re2 re3 (mk_usize 3));
    reveal_opaque (`%poly_step) (poly_step #vV re3 re4 (mk_usize 4));
    reveal_opaque (`%poly_step) (poly_step #vV re4 re5 (mk_usize 5));
    reveal_opaque (`%poly_step) (poly_step #vV re5 re6 (mk_usize 6));
    reveal_opaque (`%poly_step) (poly_step #vV re6 re7 (mk_usize 7));
    lemma_ntt_inverse_butterflies_unfold (to_spec_poly_mont #vV re0)
#pop-options

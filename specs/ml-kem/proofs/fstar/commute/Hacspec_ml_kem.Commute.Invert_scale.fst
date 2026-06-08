module Hacspec_ml_kem.Commute.Invert_scale
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models
module P  = Hacspec_ml_kem.Parameters
module IN = Hacspec_ml_kem.Invert_ntt
module SB = Hacspec_ml_kem.Commute.Matrix_bilin
module ML = FStar.Math.Lemmas

(* ════════════════════════════════════════════════════════════════════
   Linearity (scalar-homogeneity) of the inverse-NTT butterflies:
     ntt_inverse_butterflies (scale_poly c p) == scale_poly c (ntt_inverse_butterflies p)
   Each Gentleman–Sande inverse butterfly `inv_butterfly zeta a b = (a+b, zeta·(b-a))`
   is jointly homogeneous in (a,b) (zeta fixed); ntt_inverse_layer_n is a createi over
   these butterflies; ntt_inverse_butterflies composes 7 layers.
   This is the keystone for the INTT-track finalize (compute_vector_u / ring_element_v /
   message): the impl computes the inverse NTT in Mont domain (to_spec_poly_mont) while the
   spec's `ntt_inverse` is applied to the standard-domain dot product `product`, which differ
   by a scalar (R²); linearity moves that scalar through the inverse NTT. *)

let q : pos = 3329

(* ---- FE value lemma for sub (mirror Matrix_bilin's mul_val/add_val) ---- *)
let sub_val (a b: P.t_FieldElement) : Lemma
  (v (P.impl_FieldElement__sub a b).f_val
     == (v a.f_val + q - v b.f_val) % q)
= ()

(* i < d*n ==> i/d < n  (avoids guessing a stdlib lemma name) *)
let lemma_group_lt (i: nat) (d: pos) (n: nat) : Lemma
  (requires i < d * n) (ensures i / d < n)
= if i / d >= n then begin
    ML.lemma_mult_le_left d n (i / d);   (* d*n <= d*(i/d) *)
    ML.lemma_div_mod i d;                (* i == (i/d)*d + i%d *)
    assert (d * (i / d) <= i);
    assert (d * n <= i);
    assert False
  end

(* len = 1<<layer and the layer_n size facts, layer in 1..7 (case-split) *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 200"
let lemma_len_groups (layer: usize {v layer >= 1 /\ v layer <= 7}) : Lemma
  (let len = mk_usize 1 <<! layer in
   v len == pow2 (v layer) /\ v len >= 2 /\ v len <= 128 /\
   v (mk_usize 128 /! len <: usize) >= 1 /\ v (mk_usize 128 /! len <: usize) <= 64 /\
   2 * v (mk_usize 128 /! len <: usize) * v len == 256)
= match v layer with
  | 1 -> assert_norm (v (mk_usize 1 <<! mk_usize 1) == pow2 1);
        assert_norm (v (mk_usize 128 /! (mk_usize 1 <<! mk_usize 1)) == 64)
  | 2 -> assert_norm (v (mk_usize 1 <<! mk_usize 2) == pow2 2);
        assert_norm (v (mk_usize 128 /! (mk_usize 1 <<! mk_usize 2)) == 32)
  | 3 -> assert_norm (v (mk_usize 1 <<! mk_usize 3) == pow2 3);
        assert_norm (v (mk_usize 128 /! (mk_usize 1 <<! mk_usize 3)) == 16)
  | 4 -> assert_norm (v (mk_usize 1 <<! mk_usize 4) == pow2 4);
        assert_norm (v (mk_usize 128 /! (mk_usize 1 <<! mk_usize 4)) == 8)
  | 5 -> assert_norm (v (mk_usize 1 <<! mk_usize 5) == pow2 5);
        assert_norm (v (mk_usize 128 /! (mk_usize 1 <<! mk_usize 5)) == 4)
  | 6 -> assert_norm (v (mk_usize 1 <<! mk_usize 6) == pow2 6);
        assert_norm (v (mk_usize 128 /! (mk_usize 1 <<! mk_usize 6)) == 2)
  | 7 -> assert_norm (v (mk_usize 1 <<! mk_usize 7) == pow2 7);
        assert_norm (v (mk_usize 128 /! (mk_usize 1 <<! mk_usize 7)) == 1)
#pop-options

(* ---- per-FE building blocks for butterfly homogeneity ---- *)

(* distributivity of scale over sub: c·(b−a) on each side *)
let sub_distrib (c x y: P.t_FieldElement) : Lemma
  (P.impl_FieldElement__sub (P.impl_FieldElement__mul c x) (P.impl_FieldElement__mul c y)
   == P.impl_FieldElement__mul c (P.impl_FieldElement__sub x y))
= let vc = v c.f_val in let vx = v x.f_val in let vy = v y.f_val in
  SB.mul_val c x; SB.mul_val c y;
  sub_val (P.impl_FieldElement__mul c x) (P.impl_FieldElement__mul c y);
  sub_val x y; SB.mul_val c (P.impl_FieldElement__sub x y);
  (* LHS.f_val = ((vc*vx)%q + q - (vc*vy)%q) % q ; RHS.f_val = (vc * ((vx+q-vy)%q)) % q *)
  ML.lemma_mod_mul_distr_r vc (vx + q - vy) q;
  ML.lemma_mod_add_distr (vc * vx) (- (vc * vy)) q;   (* push the (vc*vx)%q out *)
  ML.lemma_mod_add_distr ((vc * vx) % q) (q - vc * vy) q;
  assert (vc * (vx + q - vy) == vc * vx + vc * q - vc * vy);
  ML.lemma_mod_add_distr (vc * vx - vc * vy) (vc * q) q;
  assert ((vc * q) % q == 0);
  SB.fe_eq
    (P.impl_FieldElement__sub (P.impl_FieldElement__mul c x) (P.impl_FieldElement__mul c y))
    (P.impl_FieldElement__mul c (P.impl_FieldElement__sub x y))

(* move a scalar past a fixed multiplier: zeta·(c·w) == c·(zeta·w) *)
let mul_swap (zeta c w: P.t_FieldElement) : Lemma
  (P.impl_FieldElement__mul zeta (P.impl_FieldElement__mul c w)
   == P.impl_FieldElement__mul c (P.impl_FieldElement__mul zeta w))
= let vz = v zeta.f_val in let vc = v c.f_val in let vw = v w.f_val in
  SB.mul_val c w; SB.mul_val zeta (P.impl_FieldElement__mul c w);
  SB.mul_val zeta w; SB.mul_val c (P.impl_FieldElement__mul zeta w);
  ML.lemma_mod_mul_distr_r vz (vc * vw) q;
  ML.lemma_mod_mul_distr_r vc (vz * vw) q;
  assert (vz * (vc * vw) == vc * (vz * vw));
  SB.fe_eq
    (P.impl_FieldElement__mul zeta (P.impl_FieldElement__mul c w))
    (P.impl_FieldElement__mul c (P.impl_FieldElement__mul zeta w))

(* joint homogeneity of the inverse butterfly (zeta unscaled) *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_inv_butterfly_scale (c zeta a b: P.t_FieldElement) : Lemma
  (ensures
    (IN.inv_butterfly zeta (P.impl_FieldElement__mul c a) (P.impl_FieldElement__mul c b))._1
      == P.impl_FieldElement__mul c ((IN.inv_butterfly zeta a b)._1) /\
    (IN.inv_butterfly zeta (P.impl_FieldElement__mul c a) (P.impl_FieldElement__mul c b))._2
      == P.impl_FieldElement__mul c ((IN.inv_butterfly zeta a b)._2))
= (* _1 : (c·a) + (c·b) == c·(a+b) *)
  SB.left_distrib c a b;
  (* _2 : zeta·((c·b) − (c·a)) == c·(zeta·(b−a)) *)
  sub_distrib c b a;
  mul_swap zeta c (P.impl_FieldElement__sub b a)
#pop-options

(* ---- per-lane unfold of ntt_inverse_layer_n at size 256 (mirror of the
        size-16 lemma_ntt_inverse_layer_n_16_lane in Invert_ntt_bridge) ---- *)
#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"
let lemma_ntt_inverse_layer_n_256_lane
    (p: t_Array P.t_FieldElement (mk_usize 256))
    (len: usize)
    (zs: t_Slice P.t_FieldElement)
    (i: nat {i < 256})
  : Lemma
    (requires
      v len >= 1 /\ v len < 1024 /\ Seq.length zs < 1024 /\
      2 * Seq.length zs * v len == 256 /\
      i / (2 * v len) < Seq.length zs)
    (ensures
      (let result = IN.ntt_inverse_layer_n (mk_usize 256) p len zs in
       let group : nat = i / (2 * v len) in
       let idx   : nat = i % (2 * v len) in
       (idx < v len ==>
          i + v len < 256 /\
          Seq.index result i ==
            (IN.inv_butterfly (Seq.index zs group) (Seq.index p i) (Seq.index p (i + v len)))._1) /\
       (idx >= v len ==>
          i >= v len /\
          Seq.index result i ==
            (IN.inv_butterfly (Seq.index zs group) (Seq.index p (i - v len)) (Seq.index p i))._2)))
  = P.createi_lemma #P.t_FieldElement (mk_usize 256)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 256 }) ->
        let g:usize = j /! (mk_usize 2 *! len <: usize) in
        let idx:usize = j %! (mk_usize 2 *! len <: usize) in
        (if idx <. len then
          (IN.inv_butterfly (Seq.index zs (v g)) (Seq.index p (v j)) (Seq.index p (v j + v len)))._1
        else
          (IN.inv_butterfly (Seq.index zs (v g)) (Seq.index p (v j - v len)) (Seq.index p (v j)))._2)
        <: P.t_FieldElement)
      (sz i)
#pop-options

(* ---- homogeneity of one ntt_inverse_layer_n (256) ---- *)
#push-options "--z3rlimit 200 --fuel 1 --ifuel 1"
let lemma_inv_layer_n_scale
    (c: P.t_FieldElement)
    (p: t_Array P.t_FieldElement (mk_usize 256))
    (len: usize)
    (zs: t_Slice P.t_FieldElement)
  : Lemma
    (requires
      v len >= 1 /\ v len < 1024 /\ Seq.length zs < 1024 /\
      2 * Seq.length zs * v len == 256)
    (ensures
      IN.ntt_inverse_layer_n (mk_usize 256) (SB.scale_poly c p) len zs
      == SB.scale_poly c (IN.ntt_inverse_layer_n (mk_usize 256) p len zs))
= let lhs = IN.ntt_inverse_layer_n (mk_usize 256) (SB.scale_poly c p) len zs in
  let nilp = IN.ntt_inverse_layer_n (mk_usize 256) p len zs in
  let rhs = SB.scale_poly c nilp in
  let aux (i: nat {i < 256}) : Lemma (Seq.index lhs i == Seq.index rhs i) =
    (* group < |zs|, since 2*|zs|*len == 256 and i < 256 *)
    assert ((2 * v len) * Seq.length zs == 2 * Seq.length zs * v len);
    lemma_group_lt i (2 * v len) (Seq.length zs);
    lemma_ntt_inverse_layer_n_256_lane (SB.scale_poly c p) len zs i;
    lemma_ntt_inverse_layer_n_256_lane p len zs i;
    SB.lemma_scale_poly_index c nilp i;
    let group : nat = i / (2 * v len) in
    let idx   : nat = i % (2 * v len) in
    if idx < v len then begin
      SB.lemma_scale_poly_index c p i;
      SB.lemma_scale_poly_index c p (i + v len);
      lemma_inv_butterfly_scale c (Seq.index zs group) (Seq.index p i) (Seq.index p (i + v len))
    end
    else begin
      SB.lemma_scale_poly_index c p i;
      SB.lemma_scale_poly_index c p (i - v len);
      lemma_inv_butterfly_scale c (Seq.index zs group) (Seq.index p (i - v len)) (Seq.index p i)
    end
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro lhs rhs
#pop-options

(* ---- homogeneity of one ntt_inverse_layer (the table-building 256 form) ----
   Reduce both `ntt_inverse_layer ? layer` applications to `ntt_inverse_layer_n 256 ? len tbl`
   with the SAME (p-independent) zeta table, then apply lemma_inv_layer_n_scale. *)
#push-options "--z3rlimit 200 --fuel 1 --ifuel 1"
let lemma_inv_layer_scale
    (c: P.t_FieldElement)
    (p: t_Array P.t_FieldElement (mk_usize 256))
    (layer: usize {v layer >= 1 /\ v layer <= 7})
  : Lemma
    (ensures
      IN.ntt_inverse_layer (SB.scale_poly c p) layer
      == SB.scale_poly c (IN.ntt_inverse_layer p layer))
= let len : usize = mk_usize 1 <<! layer in
  let groups : usize = mk_usize 128 /! len in
  let zetas_tbl : t_Array P.t_FieldElement (mk_usize 128) =
    P.createi #P.t_FieldElement (mk_usize 128)
      #(usize -> P.t_FieldElement)
      (fun round ->
        if round <. groups
        then Hacspec_ml_kem.Ntt.v_ZETAS.[ (mk_usize 2 *! groups -! mk_usize 1) -! round ]
        else P.impl_FieldElement__new (mk_u16 0))
  in
  let tbl_slice : t_Slice P.t_FieldElement =
    zetas_tbl.[ { Core_models.Ops.Range.f_start = mk_usize 0;
                  Core_models.Ops.Range.f_end = groups } ] in
  (* shift / divisibility facts for v len == pow2 (v layer), layer in 1..7 *)
  lemma_len_groups layer;
  assert (v groups == v (mk_usize 128 /! len <: usize));
  assert (Seq.length tbl_slice == v groups);
  assert (2 * Seq.length tbl_slice * v len == 256);
  assert (IN.ntt_inverse_layer (SB.scale_poly c p) layer
          == IN.ntt_inverse_layer_n (mk_usize 256) (SB.scale_poly c p) len tbl_slice)
    by (FStar.Tactics.norm [delta_only [`%IN.ntt_inverse_layer]; iota; zeta; primops];
        FStar.Tactics.trefl ());
  assert (IN.ntt_inverse_layer p layer
          == IN.ntt_inverse_layer_n (mk_usize 256) p len tbl_slice)
    by (FStar.Tactics.norm [delta_only [`%IN.ntt_inverse_layer]; iota; zeta; primops];
        FStar.Tactics.trefl ());
  lemma_inv_layer_n_scale c p len tbl_slice
#pop-options

(* ---- homogeneity of the full 7-layer ntt_inverse_butterflies ---- *)
#push-options "--z3rlimit 200 --fuel 1 --ifuel 1"
let lemma_ntt_inverse_butterflies_scale
    (c: P.t_FieldElement)
    (p: t_Array P.t_FieldElement (mk_usize 256))
  : Lemma
    (ensures
      IN.ntt_inverse_butterflies (SB.scale_poly c p)
      == SB.scale_poly c (IN.ntt_inverse_butterflies p))
= let q1 = IN.ntt_inverse_layer p (mk_usize 1) in
  let q2 = IN.ntt_inverse_layer q1 (mk_usize 2) in
  let q3 = IN.ntt_inverse_layer q2 (mk_usize 3) in
  let q4 = IN.ntt_inverse_layer q3 (mk_usize 4) in
  let q5 = IN.ntt_inverse_layer q4 (mk_usize 5) in
  let q6 = IN.ntt_inverse_layer q5 (mk_usize 6) in
  lemma_inv_layer_scale c p  (mk_usize 1);
  lemma_inv_layer_scale c q1 (mk_usize 2);
  lemma_inv_layer_scale c q2 (mk_usize 3);
  lemma_inv_layer_scale c q3 (mk_usize 4);
  lemma_inv_layer_scale c q4 (mk_usize 5);
  lemma_inv_layer_scale c q5 (mk_usize 6);
  lemma_inv_layer_scale c q6 (mk_usize 7);
  (* unfold both sides to the 7-fold ntt_inverse_layer composition; the 7 layer
     equalities above then chain to the conclusion. *)
  assert (IN.ntt_inverse_butterflies (SB.scale_poly c p)
          == IN.ntt_inverse_layer (IN.ntt_inverse_layer (IN.ntt_inverse_layer
               (IN.ntt_inverse_layer (IN.ntt_inverse_layer (IN.ntt_inverse_layer
                 (IN.ntt_inverse_layer (SB.scale_poly c p) (mk_usize 1)) (mk_usize 2)) (mk_usize 3))
                 (mk_usize 4)) (mk_usize 5)) (mk_usize 6)) (mk_usize 7))
    by (FStar.Tactics.norm [delta_only [`%IN.ntt_inverse_butterflies]; iota; zeta; primops];
        FStar.Tactics.trefl ());
  assert (IN.ntt_inverse_butterflies p
          == IN.ntt_inverse_layer (IN.ntt_inverse_layer (IN.ntt_inverse_layer
               (IN.ntt_inverse_layer (IN.ntt_inverse_layer (IN.ntt_inverse_layer
                 (IN.ntt_inverse_layer p (mk_usize 1)) (mk_usize 2)) (mk_usize 3))
                 (mk_usize 4)) (mk_usize 5)) (mk_usize 6)) (mk_usize 7))
    by (FStar.Tactics.norm [delta_only [`%IN.ntt_inverse_butterflies]; iota; zeta; primops];
        FStar.Tactics.trefl ())
#pop-options

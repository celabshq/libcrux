module Hacspec_ml_kem.Commute.Matrix_bilin
#set-options "--fuel 0 --ifuel 1 --z3rlimit 50"
open FStar.Mul
open Core_models
module P = Hacspec_ml_kem.Parameters
module N = Hacspec_ml_kem.Ntt
module V = Libcrux_ml_kem.Vector.Traits.Spec
module Poly = Libcrux_ml_kem.Polynomial
module ML = FStar.Math.Lemmas

let q : pos = 3329

(* ---- value-level characterisation of the FE ops ---- *)

let mul_val (a b: P.t_FieldElement) : Lemma
  (v (P.impl_FieldElement__mul a b).f_val
     == (v a.f_val * v b.f_val) % q)
= ()

let add_val (a b: P.t_FieldElement) : Lemma
  (v (P.impl_FieldElement__add a b).f_val
     == (v a.f_val + v b.f_val) % q)
= ()

(* FE equality reduces to f_val equality (single-field record). *)
let fe_eq (a b: P.t_FieldElement) : Lemma
  (requires v a.f_val == v b.f_val)
  (ensures a == b)
= ()

(* ---- FE ring lemmas ---- *)

let mul_assoc (a b c: P.t_FieldElement) : Lemma
  (P.impl_FieldElement__mul (P.impl_FieldElement__mul a b) c
   == P.impl_FieldElement__mul a (P.impl_FieldElement__mul b c))
= let va = v a.f_val in let vb = v b.f_val in let vc = v c.f_val in
  mul_val a b; mul_val (P.impl_FieldElement__mul a b) c;
  mul_val b c; mul_val a (P.impl_FieldElement__mul b c);
  ML.lemma_mod_mul_distr_l (va*vb) vc q;
  ML.lemma_mod_mul_distr_r va (vb*vc) q;
  assert ((va*vb)*vc == va*(vb*vc));
  fe_eq (P.impl_FieldElement__mul (P.impl_FieldElement__mul a b) c)
        (P.impl_FieldElement__mul a (P.impl_FieldElement__mul b c))

let left_distrib (c u w: P.t_FieldElement) : Lemma
  (P.impl_FieldElement__mul c (P.impl_FieldElement__add u w)
   == P.impl_FieldElement__add (P.impl_FieldElement__mul c u) (P.impl_FieldElement__mul c w))
= let vc = v c.f_val in let vu = v u.f_val in let vw = v w.f_val in
  add_val u w; mul_val c (P.impl_FieldElement__add u w);
  mul_val c u; mul_val c w; add_val (P.impl_FieldElement__mul c u) (P.impl_FieldElement__mul c w);
  ML.lemma_mod_mul_distr_r vc (vu+vw) q;
  ML.lemma_mod_add_distr (vc*vu) (vc*vw) q;  (* placeholder name; fix if wrong *)
  assert (vc*(vu+vw) == vc*vu + vc*vw);
  fe_eq (P.impl_FieldElement__mul c (P.impl_FieldElement__add u w))
        (P.impl_FieldElement__add (P.impl_FieldElement__mul c u) (P.impl_FieldElement__mul c w))

(* homogeneity of FE mul: (c*a)*(c*b) = (c*c)*(a*b) *)
let mul_homog (c a b: P.t_FieldElement) : Lemma
  (P.impl_FieldElement__mul (P.impl_FieldElement__mul c a) (P.impl_FieldElement__mul c b)
   == P.impl_FieldElement__mul (P.impl_FieldElement__mul c c) (P.impl_FieldElement__mul a b))
= let vc = v c.f_val in let va = v a.f_val in let vb = v b.f_val in
  mul_val c a; mul_val c b;
  mul_val (P.impl_FieldElement__mul c a) (P.impl_FieldElement__mul c b);
  mul_val c c; mul_val a b;
  mul_val (P.impl_FieldElement__mul c c) (P.impl_FieldElement__mul a b);
  ML.lemma_mod_mul_distr_l (vc*va) ((vc*vb)%q) q;
  ML.lemma_mod_mul_distr_r (vc*va) (vc*vb) q;
  ML.lemma_mod_mul_distr_l (vc*vc) ((va*vb)%q) q;
  ML.lemma_mod_mul_distr_r (vc*vc) (va*vb) q;
  assert ((vc*va)*(vc*vb) == (vc*vc)*(va*vb));
  fe_eq (P.impl_FieldElement__mul (P.impl_FieldElement__mul c a) (P.impl_FieldElement__mul c b))
        (P.impl_FieldElement__mul (P.impl_FieldElement__mul c c) (P.impl_FieldElement__mul a b))

(* ---- per-pair base_case_multiply homogeneity (scale a's,b's by c; zeta unscaled) ---- *)

let bcm_even_homog (c a0 a1 b0 b1 zeta: P.t_FieldElement) : Lemma
  (ensures
    N.base_case_multiply_even (P.impl_FieldElement__mul c a0) (P.impl_FieldElement__mul c a1)
                              (P.impl_FieldElement__mul c b0) (P.impl_FieldElement__mul c b1) zeta
    == P.impl_FieldElement__mul (P.impl_FieldElement__mul c c)
                                (N.base_case_multiply_even a0 a1 b0 b1 zeta))
= mul_homog c a0 b0;
  mul_homog c a1 b1;
  mul_assoc (P.impl_FieldElement__mul c c) (P.impl_FieldElement__mul a1 b1) zeta;
  left_distrib (P.impl_FieldElement__mul c c)
               (P.impl_FieldElement__mul a0 b0)
               (P.impl_FieldElement__mul (P.impl_FieldElement__mul a1 b1) zeta)

let bcm_odd_homog (c a0 a1 b0 b1: P.t_FieldElement) : Lemma
  (ensures
    N.base_case_multiply_odd (P.impl_FieldElement__mul c a0) (P.impl_FieldElement__mul c a1)
                             (P.impl_FieldElement__mul c b0) (P.impl_FieldElement__mul c b1)
    == P.impl_FieldElement__mul (P.impl_FieldElement__mul c c)
                                (N.base_case_multiply_odd a0 a1 b0 b1))
= mul_homog c a0 b1;
  mul_homog c a1 b0;
  left_distrib (P.impl_FieldElement__mul c c)
               (P.impl_FieldElement__mul a0 b1)
               (P.impl_FieldElement__mul a1 b0)

(* ---- lift bridges: plain = R*mont, std = R*plain (R as FE = 2285) ---- *)

let r_fe : P.t_FieldElement = P.impl_FieldElement__new (mk_u16 2285)

(* plain = R *_FE mont   (uses 2285 * 169 = 386165 == 1 (mod 3329)) *)
let lemma_plain_eq_R_mont (x: i16) : Lemma
  (V.i16_to_spec_fe x == P.impl_FieldElement__mul r_fe (V.mont_i16_to_spec_fe x))
= let m = V.mont_i16_to_spec_fe x in
  let plain = V.i16_to_spec_fe x in
  let vx = v x in
  mul_val r_fe m;                                   (* v(mul r_fe m).f_val == (2285 * ((vx*169)%q)) % q *)
  ML.lemma_mod_mul_distr_r 2285 (vx * 169) q;       (* == (2285 * (vx*169)) % q *)
  assert_norm (2285 * 169 == 386165);
  assert (2285 * (vx * 169) == 386165 * vx);
  ML.lemma_mod_mul_distr_l 386165 vx q;             (* (386165*vx)%q == ((386165%q)*vx)%q *)
  assert_norm (386165 % 3329 == 1);
  fe_eq plain (P.impl_FieldElement__mul r_fe m)

(* std = R *_FE plain *)
let lemma_std_eq_R_plain (x: i16) : Lemma
  (Poly.std_i16_to_spec_fe x == P.impl_FieldElement__mul r_fe (V.i16_to_spec_fe x))
= let plain = V.i16_to_spec_fe x in
  let std = Poly.std_i16_to_spec_fe x in
  let vx = v x in
  mul_val r_fe plain;                               (* v(mul r_fe plain).f_val == (2285 * (vx % q)) % q *)
  ML.lemma_mod_mul_distr_r 2285 vx q;               (* == (2285 * vx) % q  == v std.f_val *)
  fe_eq std (P.impl_FieldElement__mul r_fe plain)

(* ---- poly-level bilinearity (per-lane) ---- *)

(* 256-lane unfold of ntt_multiply_n (mirror of the .fst-private
   Libcrux_ml_kem.Polynomial.lemma_ntt_multiply_n_256_lane). *)
#push-options "--z3rlimit 150"
let lane_unfold_256
    (p1 p2: t_Array P.t_FieldElement (mk_usize 256))
    (zs: t_Slice P.t_FieldElement)
    (i: nat {i < 256})
  : Lemma
    (requires
      (Core_models.Slice.impl__len #P.t_FieldElement zs <: usize) <. mk_usize 1024 &&
      ((Core_models.Slice.impl__len #P.t_FieldElement zs <: usize) *! mk_usize 4 <: usize) =.
        mk_usize 256)
    (ensures
      (let result = N.ntt_multiply_n (mk_usize 256) p1 p2 zs in
       let group : nat = i / 4 in
       let zeta = (if i % 4 < 2 then Seq.index zs group
                   else P.impl_FieldElement__neg (Seq.index zs group)) in
       (i % 2 = 0 ==>
         i + 1 < 256 /\
         Seq.index result i ==
           N.base_case_multiply_even (Seq.index p1 i) (Seq.index p1 (i + 1))
                                     (Seq.index p2 i) (Seq.index p2 (i + 1)) zeta) /\
       (i % 2 = 1 ==>
         i >= 1 /\
         Seq.index result i ==
           N.base_case_multiply_odd (Seq.index p1 (i - 1)) (Seq.index p1 i)
                                    (Seq.index p2 (i - 1)) (Seq.index p2 i))))
  = P.createi_lemma #P.t_FieldElement (mk_usize 256)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 256 }) ->
        let group:usize = j /! mk_usize 4 in
        let zeta:P.t_FieldElement =
          if (j %! mk_usize 4 <: usize) <. mk_usize 2
          then Seq.index zs (v group)
          else P.impl_FieldElement__neg (Seq.index zs (v group))
        in
        (if (j %! mk_usize 2 <: usize) =. mk_usize 0
         then
           N.base_case_multiply_even (Seq.index p1 (v j)) (Seq.index p1 (v j + 1))
                                     (Seq.index p2 (v j)) (Seq.index p2 (v j + 1)) zeta
         else
           N.base_case_multiply_odd (Seq.index p1 (v j - 1)) (Seq.index p1 (v j))
                                    (Seq.index p2 (v j - 1)) (Seq.index p2 (v j)))
        <: P.t_FieldElement)
      (sz i)
#pop-options

let scale_poly (c: P.t_FieldElement) (p: t_Array P.t_FieldElement (mk_usize 256))
  : t_Array P.t_FieldElement (mk_usize 256)
= P.createi #P.t_FieldElement (mk_usize 256) #(usize -> P.t_FieldElement)
    (fun (j: usize {j <. mk_usize 256}) -> P.impl_FieldElement__mul c (Seq.index p (v j)) <: P.t_FieldElement)

let lemma_scale_poly_index (c: P.t_FieldElement) (p: t_Array P.t_FieldElement (mk_usize 256)) (i: nat{i<256})
  : Lemma (Seq.index (scale_poly c p) i == P.impl_FieldElement__mul c (Seq.index p i))
= P.createi_lemma #P.t_FieldElement (mk_usize 256) #(usize -> P.t_FieldElement)
    (fun (j: usize {j <. mk_usize 256}) -> P.impl_FieldElement__mul c (Seq.index p (v j)) <: P.t_FieldElement)
    (sz i)

(* per-lane poly bilinearity: scaling both inputs by r_fe scales the lane by r_fe*r_fe *)
#push-options "--z3rlimit 200"
let lemma_poly_bilin_lane
    (p1 p2: t_Array P.t_FieldElement (mk_usize 256))
    (zs: t_Slice P.t_FieldElement)
    (i: nat {i < 256})
  : Lemma
    (requires
      (Core_models.Slice.impl__len #P.t_FieldElement zs <: usize) <. mk_usize 1024 &&
      ((Core_models.Slice.impl__len #P.t_FieldElement zs <: usize) *! mk_usize 4 <: usize) =.
        mk_usize 256)
    (ensures
      Seq.index (N.ntt_multiply_n (mk_usize 256) (scale_poly r_fe p1) (scale_poly r_fe p2) zs) i
      == P.impl_FieldElement__mul (P.impl_FieldElement__mul r_fe r_fe)
           (Seq.index (N.ntt_multiply_n (mk_usize 256) p1 p2 zs) i))
= lane_unfold_256 (scale_poly r_fe p1) (scale_poly r_fe p2) zs i;
  lane_unfold_256 p1 p2 zs i;
  lemma_scale_poly_index r_fe p1 i;
  lemma_scale_poly_index r_fe p2 i;
  let group : nat = i / 4 in
  let zeta = (if i % 4 < 2 then Seq.index zs group
              else P.impl_FieldElement__neg (Seq.index zs group)) in
  if i % 2 = 0 then begin
    lemma_scale_poly_index r_fe p1 (i+1);
    lemma_scale_poly_index r_fe p2 (i+1);
    bcm_even_homog r_fe (Seq.index p1 i) (Seq.index p1 (i+1))
                        (Seq.index p2 i) (Seq.index p2 (i+1)) zeta
  end else begin
    lemma_scale_poly_index r_fe p1 (i-1);
    lemma_scale_poly_index r_fe p2 (i-1);
    bcm_odd_homog r_fe (Seq.index p1 (i-1)) (Seq.index p1 i)
                       (Seq.index p2 (i-1)) (Seq.index p2 i)
  end
#pop-options

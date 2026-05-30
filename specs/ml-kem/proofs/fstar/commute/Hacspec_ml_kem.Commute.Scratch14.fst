module Hacspec_ml_kem.Commute.Scratch14
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models
open Libcrux_ml_kem.Vector.Traits.Spec
open Hacspec_ml_kem.Commute.Chunk
open Hacspec_ml_kem.Commute.Bridges

module P  = Hacspec_ml_kem.Parameters
module T  = Libcrux_ml_kem.Vector.Traits
module TS = Libcrux_ml_kem.Vector.Traits.Spec
module N  = Hacspec_ml_kem.Ntt
module IN = Hacspec_ml_kem.Invert_ntt

(* === LEVEL A: array-level createi composition for ntt_inverse_layer_n 256 === *)
#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"
let lemma_ntt_inverse_layer_n_256_compose
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
  = let rhs = IN.ntt_inverse_layer_n (mk_usize 256) p len zetas in
    let aux (i: nat) : Lemma (i < 256 ==> Seq.index q i == Seq.index rhs i)
      = if i < 256 then begin
          let group : nat = i / (2 * v len) in
          assert (group < Seq.length zetas);
          assert (Seq.index rhs i ==
            (let g : usize = (sz i) /! (mk_usize 2 *! len <: usize) in
             let idx : usize = (sz i) %! (mk_usize 2 *! len <: usize) in
             if idx <. len
             then (IN.inv_butterfly (Seq.index zetas (v g)) (Seq.index p i) (Seq.index p (i + v len)))._1
             else (IN.inv_butterfly (Seq.index zetas (v g)) (Seq.index p (i - v len)) (Seq.index p i))._2))
        end
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro q rhs
#pop-options

(* Per-lane unfold for to_spec_poly_mont_arr (mirror of mont_to_spec_poly_256_lane). *)
let tspm_arr_lane
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (a: t_Array vV (mk_usize 16)) (j: nat { j < 256 }) :
    Lemma (Seq.index (to_spec_poly_mont_arr #vV a) j
           == mont_i16_to_spec_fe (Seq.index (T.f_repr (Seq.index a (j / 16))) (j % 16)))
  = P.createi_lemma #P.t_FieldElement (mk_usize 256)
      #(usize -> P.t_FieldElement)
      (fun (k: usize { k <. mk_usize 256 }) ->
        (mont_i16_to_spec_fe
          (Seq.index (T.f_repr (Seq.index a (v k / 16))) (v k % 16))
         <: P.t_FieldElement))
      (sz j)

(* === Index-decomposition helper (clean nonlinear context) === *)
#push-options "--z3rlimit 300 --fuel 0 --ifuel 0"
let lemma_cross_idx (i: nat{i < 256}) (s: pos{s == 1 \/ s == 2 \/ s == 4 \/ s == 8})
  : Lemma
    (let m = i / 16 in let l = i % 16 in let len = 16 * s in
     m < 16 /\ l < 16 /\ i == 16 * m + l /\
     i / (2 * len) == m / (2 * s) /\
     i % (2 * len) == 16 * (m % (2 * s)) + l /\
     ((i % (2 * len)) < len <==> (m % (2 * s)) < s))
  = let m = i / 16 in let l = i % 16 in let len = 16 * s in
    FStar.Math.Lemmas.euclidean_division_definition i 16;
    FStar.Math.Lemmas.euclidean_division_definition m (2 * s);
    let q = m / (2 * s) in
    let r = m % (2 * s) in
    assert (m == (2 * s) * q + r);
    assert (i == (32 * s) * q + (16 * r + l));
    assert (16 * r + l < 32 * s);
    FStar.Math.Lemmas.lemma_div_plus (16 * r + l) q (32 * s);
    FStar.Math.Lemmas.lemma_mod_plus (16 * r + l) q (32 * s);
    FStar.Math.Lemmas.small_div (16 * r + l) (32 * s);
    FStar.Math.Lemmas.small_mod (16 * r + l) (32 * s)
#pop-options

(* Partner-index helper: for i<256, l=i%16<16, s pos with m+s<16 (resp m>=s),
   (i + 16*s)/16 == i/16 + s and (i+16*s)%16 == i%16 (resp for subtraction). *)
#push-options "--z3rlimit 200 --fuel 0 --ifuel 0"
let lemma_partner_idx_add (i: nat) (s: pos)
  : Lemma (requires i % 16 < 16)
          (ensures (i + 16 * s) / 16 == i / 16 + s /\ (i + 16 * s) % 16 == i % 16)
  = FStar.Math.Lemmas.euclidean_division_definition i 16;
    FStar.Math.Lemmas.lemma_div_plus i s 16;
    FStar.Math.Lemmas.lemma_mod_plus i s 16
let lemma_partner_idx_sub (i: nat) (s: pos)
  : Lemma (requires i >= 16 * s)
          (ensures (i - 16 * s) / 16 == i / 16 - s /\ (i - 16 * s) % 16 == i % 16)
  = FStar.Math.Lemmas.euclidean_division_definition i 16;
    FStar.Math.Lemmas.lemma_div_plus (i - 16 * s) s 16;
    FStar.Math.Lemmas.lemma_mod_plus (i - 16 * s) s 16
#pop-options

(* Enumerated arithmetic helper: for the four layer lengths, 2*(128/x)*x==256. *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 0"
let lemma_div_128_prod (x: nat)
  : Lemma (requires x == 16 \/ x == 32 \/ x == 64 \/ x == 128)
          (ensures 2 * (128 / x) * x == 256)
  = ()
#pop-options

(* === LEVEL B: cross-vector index bridge ===
   step_vec = len/16, groups = 128/len; vector m lane l holds poly coeff 16m+l;
   block = m/(2*step_vec), pos = m%(2*step_vec).  Low half (pos<step_vec): SUM
   with partner m+step_vec; high half: MUL with partner m-step_vec; zeta zs[block].
   Concludes the FE-array equation against ntt_inverse_layer_n 256. *)
(* Flat per-vector requires predicate (no nested forall, NOT unfold so the
   requires forall stays compact — Z3 instantiates one flat quantifier at
   (i/16, i%16); the body is revealed only at the single instantiation site). *)
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

#push-options "--z3rlimit 300 --fuel 0 --ifuel 1"
let lemma_layer_4_plus_cross_vector
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
  = (* establish step_vec ∈ {1,2,4,8} from the len disjunction *)
    assert (v len / 16 == 1 \/ v len / 16 == 2 \/ v len / 16 == 4 \/ v len / 16 == 8);
    let step_vec : s:pos{s == 1 \/ s == 2 \/ s == 4 \/ s == 8} = v len / 16 in
    let p = to_spec_poly_mont_arr #vV cin in
    let q = to_spec_poly_mont_arr #vV cout in
    (* establish per-coefficient hypothesis of Level A *)
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
          lemma_cross_idx i step_vec;
          (* now: m<16, l<16, i=16m+l, i/(2len)=m/(2s), i%(2len)<len <==> m%(2s)<s *)
          (* reveal the opaque hypothesis at this single (m,l) instantiation *)
          assert (cross_vec_hyp #vV cin cout step_vec zs m l);
          reveal_opaque (`%cross_vec_hyp)
            (cross_vec_hyp #vV cin cout step_vec zs m l);
          let block : nat = m / (2 * step_vec) in
          let pos   : nat = m % (2 * step_vec) in
          tspm_arr_lane #vV cout i;       (* q[i] = mont(cout[m])[l] *)
          tspm_arr_lane #vV cin i;        (* p[i] = mont(cin[m])[l] *)
          if pos < step_vec then begin
            (* m+step_vec<16 from the requires; partner = vector m+step_vec, lane l *)
            lemma_partner_idx_add i step_vec;   (* (i+len)/16 = m+s, (i+len)%16 = l *)
            tspm_arr_lane #vV cin (i + v len)
          end else begin
            lemma_partner_idx_sub i step_vec;   (* (i-len)/16 = m-s, (i-len)%16 = l *)
            tspm_arr_lane #vV cin (i - v len)
          end
        end
    in
    Classical.forall_intro aux;
    (* len ∈ {16,32,64,128}; 2*(128/len)*len == 256 via the enumerated helper. *)
    assert (Seq.length zs == 128 / v len);
    lemma_div_128_prod (v len);
    lemma_ntt_inverse_layer_n_256_compose p q len zs
#pop-options

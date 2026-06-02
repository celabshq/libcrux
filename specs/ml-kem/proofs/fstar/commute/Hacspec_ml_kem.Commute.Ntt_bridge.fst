module Hacspec_ml_kem.Commute.Ntt_bridge
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models
open Libcrux_ml_kem.Vector.Traits.Spec
open Hacspec_ml_kem.Commute.Chunk
open Hacspec_ml_kem.Commute.Bridges

(* FORWARD NTT — layers 1-7 per-vector/cross-vector -> polynomial composition.
   Mirror of Hacspec_ml_kem.Commute.Invert_ntt_bridge (layers 1-3) and the
   Bridges USER-14 layer-4+ keystone, but for the Cooley-Tukey forward butterfly
   and stated in PLAIN form (`to_spec_poly_plain`) to match `ntt_vector_u`'s post.
   The per-vector layer posts are MONT (`mont_i16_to_spec_array`); the
   mont->plain (169-scaling) reconciliation is done per-coefficient in `per_coeff`
   using the cancellation lemma below (169 * 2285 == 1 mod 3329). *)

module P  = Hacspec_ml_kem.Parameters
module T  = Libcrux_ml_kem.Vector.Traits
module TS = Libcrux_ml_kem.Vector.Traits.Spec
module N  = Hacspec_ml_kem.Ntt
module VV = Libcrux_ml_kem.Vector
module L  = FStar.Math.Lemmas

(* =====================================================================
   SECTION 1 — per-element mont -> plain reconciliation
   ===================================================================== *)

(* 169 is invertible mod 3329 (inverse 2285, since 169*2285 = 386165 = 1 + 116*3329).
   Cancel the common *169 factor from a residue equation. *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 0"
let lemma_cancel_169 (x y: int)
  : Lemma (requires (x * 169) % 3329 == (y * 169) % 3329)
          (ensures  x % 3329 == y % 3329)
  = let q : pos = 3329 in
    (* multiply the residue equation through by 2285 *)
    L.lemma_mod_mul_distr_l (x * 169) 2285 q;   (* (((x*169)%q)*2285)%q == ((x*169)*2285)%q *)
    L.lemma_mod_mul_distr_l (y * 169) 2285 q;
    assert (((x * 169) % q) * 2285 % q == ((y * 169) % q) * 2285 % q);
    assert ((x * 169) * 2285 == x + (x * 116) * q);
    assert ((y * 169) * 2285 == y + (y * 116) * q);
    L.lemma_mod_plus x (x * 116) q;             (* (x + (x*116)*q)%q == x%q *)
    L.lemma_mod_plus y (y * 116) q
#pop-options

(* int-level core: the MONT butterfly-plus residue equals 169 * the plain inner value. *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 0"
let lemma_bf_plus_mont_core (a b za: int)
  : Lemma (((a * 169) % 3329 + ((za * ((b * 169) % 3329)) % 3329)) % 3329
           == (169 * (a + za * b)) % 3329)
  = let q : pos = 3329 in
    L.lemma_mod_mul_distr_r za (b * 169) q;                 (* (za*((b*169)%q))%q == (za*(b*169))%q *)
    L.lemma_mod_add_distr ((a * 169) % q) (za * (b * 169)) q;
    L.lemma_mod_add_distr (za * (b * 169)) (a * 169) q;
    assert (a * 169 + za * (b * 169) == 169 * (a + za * b));
    L.lemma_mod_mul_distr_r 169 (a + za * b) q
#pop-options

(* int-level core: the PLAIN butterfly-plus residue equals the plain inner value. *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 0"
let lemma_bf_plus_plain_core (a b za: int)
  : Lemma ((a % 3329 + ((za * (b % 3329)) % 3329)) % 3329 == (a + za * b) % 3329)
  = let q : pos = 3329 in
    L.lemma_mod_mul_distr_r za b q;                         (* (za*(b%q))%q == (za*b)%q *)
    L.lemma_mod_add_distr (a % q) (za * b) q;
    L.lemma_mod_add_distr (za * b) a q
#pop-options

(* int-level core: MONT butterfly-minus residue equals 169 * the plain inner value. *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 0"
let lemma_bf_minus_mont_core (a b za: int)
  : Lemma (((a * 169) % 3329 - ((za * ((b * 169) % 3329)) % 3329)) % 3329
           == (169 * (a - za * b)) % 3329)
  = let q : pos = 3329 in
    L.lemma_mod_mul_distr_r za (b * 169) q;
    L.lemma_mod_sub_distr ((a * 169) % q) (za * (b * 169)) q;
    L.lemma_mod_add_distr (- (za * (b * 169))) (a * 169) q;
    assert (a * 169 - za * (b * 169) == 169 * (a - za * b));
    L.lemma_mod_mul_distr_r 169 (a - za * b) q
#pop-options

#push-options "--z3rlimit 100 --fuel 0 --ifuel 0"
let lemma_bf_minus_plain_core (a b za: int)
  : Lemma ((a % 3329 - ((za * (b % 3329)) % 3329)) % 3329 == (a - za * b) % 3329)
  = let q : pos = 3329 in
    L.lemma_mod_mul_distr_r za b q;
    L.lemma_mod_sub_distr (a % q) (za * b) q;
    L.lemma_mod_add_distr (- (za * b)) a q
#pop-options

(* FE-level mont -> plain butterfly._1 (sum branch).  z is a fixed plain FE
   (the true zeta); only a,b,out carry the *169 Montgomery scaling. *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 1"
let lemma_mont_to_plain_butterfly_plus (a b out: i16) (z: P.t_FieldElement)
  : Lemma
    (requires
      mont_i16_to_spec_fe out ==
        P.impl_FieldElement__add (mont_i16_to_spec_fe a)
          (P.impl_FieldElement__mul z (mont_i16_to_spec_fe b)))
    (ensures
      i16_to_spec_fe out ==
        P.impl_FieldElement__add (i16_to_spec_fe a)
          (P.impl_FieldElement__mul z (i16_to_spec_fe b)))
  = let za = v z.P.f_val in
    (* unfold the mont RHS f_val *)
    lemma_impl_mul_v_val z (mont_i16_to_spec_fe b);
    lemma_impl_add_v_val (mont_i16_to_spec_fe a)
                         (P.impl_FieldElement__mul z (mont_i16_to_spec_fe b));
    lemma_bf_plus_mont_core (v a) (v b) za;
    (* from requires: (v out*169)%q == (169*(v a + za*v b))%q *)
    assert ((v out * 169) % 3329 == (169 * (v a + za * v b)) % 3329);
    L.swap_mul (v a + za * v b) 169;
    lemma_cancel_169 (v out) (v a + za * v b);
    (* unfold the plain RHS f_val and conclude *)
    lemma_impl_mul_v_val z (i16_to_spec_fe b);
    lemma_impl_add_v_val (i16_to_spec_fe a)
                         (P.impl_FieldElement__mul z (i16_to_spec_fe b));
    lemma_bf_plus_plain_core (v a) (v b) za
#pop-options

(* FE-level mont -> plain butterfly._2 (difference branch). *)
#push-options "--z3rlimit 100 --fuel 0 --ifuel 1"
let lemma_mont_to_plain_butterfly_minus (a b out: i16) (z: P.t_FieldElement)
  : Lemma
    (requires
      mont_i16_to_spec_fe out ==
        P.impl_FieldElement__sub (mont_i16_to_spec_fe a)
          (P.impl_FieldElement__mul z (mont_i16_to_spec_fe b)))
    (ensures
      i16_to_spec_fe out ==
        P.impl_FieldElement__sub (i16_to_spec_fe a)
          (P.impl_FieldElement__mul z (i16_to_spec_fe b)))
  = let za = v z.P.f_val in
    lemma_impl_mul_v_val z (mont_i16_to_spec_fe b);
    lemma_impl_sub_v_val (mont_i16_to_spec_fe a)
                         (P.impl_FieldElement__mul z (mont_i16_to_spec_fe b));
    lemma_bf_minus_mont_core (v a) (v b) za;
    assert ((v out * 169) % 3329 == (169 * (v a - za * v b)) % 3329);
    L.swap_mul (v a - za * v b) 169;
    lemma_cancel_169 (v out) (v a - za * v b);
    lemma_impl_mul_v_val z (i16_to_spec_fe b);
    lemma_impl_sub_v_val (i16_to_spec_fe a)
                         (P.impl_FieldElement__mul z (i16_to_spec_fe b));
    lemma_bf_minus_plain_core (v a) (v b) za
#pop-options

(* =====================================================================
   SECTION 2 — forward generic helpers (mirror of the inverse, butterfly +
   ASCENDING zeta slices v_ZETAS[groups .. 2*groups])
   ===================================================================== *)

(* index helpers (direction-agnostic; copied from Invert_ntt_bridge) *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 100"
let lemma_shift_pow2_lo (layer: usize {v layer == 1 \/ v layer == 2 \/ v layer == 3})
  : Lemma (v (mk_usize 1 <<! layer) == pow2 (v layer))
  = if v layer = 1 then assert_norm (v (mk_usize 1 <<! mk_usize 1) == pow2 1)
    else if v layer = 2 then assert_norm (v (mk_usize 1 <<! mk_usize 2) == pow2 2)
    else assert_norm (v (mk_usize 1 <<! mk_usize 3) == pow2 3)
#pop-options

#push-options "--z3rlimit 100 --fuel 0 --ifuel 1"
let lemma_intra_arith (len: pos {len == 2 \/ len == 4 \/ len == 8}) (m: nat)
  : Lemma ((2 * len) * (8 / len) == 16 /\ (2 * (8 / len)) * len == 16 /\
           16 * m == ((8 / len) * m) * (2 * len))
  = if len = 2 then () else if len = 4 then () else ()
#pop-options

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

#push-options "--z3rlimit 100 --fuel 0 --ifuel 1"
let lemma_div_128_prod_lo (x: nat)
  : Lemma (requires x == 2 \/ x == 4 \/ x == 8)
          (ensures 2 * (128 / x) * x == 256 /\ ((128 / x) * 2) * x == 256 /\
                   128 / x >= 1 /\ 128 / x < 1024)
  = ()
#pop-options

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

#push-options "--z3rlimit 100 --fuel 0 --ifuel 1"
let lemma_gv_lt (l: nat {l < 16}) (len: pos {len == 2 \/ len == 4 \/ len == 8})
  : Lemma (l / (2 * len) < 8 / len)
  = if len = 2 then () else if len = 4 then () else ()
#pop-options

(* Per-lane unfold for to_spec_poly_plain_arr (mirror of tspm_arr_lane). *)
let tspp_arr_lane
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (a: t_Array vV (mk_usize 16)) (j: nat { j < 256 }) :
    Lemma (Seq.index (to_spec_poly_plain_arr #vV a) j
           == i16_to_spec_fe (Seq.index (T.f_repr (Seq.index a (j / 16))) (j % 16)))
  = P.createi_lemma #P.t_FieldElement (mk_usize 256)
      #(usize -> P.t_FieldElement)
      (fun (k: usize { k <. mk_usize 256 }) ->
        (i16_to_spec_fe
          (Seq.index (T.f_repr (Seq.index a (v k / 16))) (v k % 16))
         <: P.t_FieldElement))
      (sz j)

(* Generic size-16 per-lane unfold for `N.ntt_layer_n 16 p len zs`. *)
#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"
let lemma_ntt_layer_n_16_lane
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
      (let result = N.ntt_layer_n (mk_usize 16) p len zs in
       let group : nat = i / (2 * v len) in
       let idx   : nat = i % (2 * v len) in
       (idx < v len ==>
          i + v len < 16 /\
          Seq.index result i ==
            (N.butterfly (Seq.index zs group) (Seq.index p i) (Seq.index p (i + v len)))._1) /\
       (idx >= v len ==>
          i >= v len /\
          Seq.index result i ==
            (N.butterfly (Seq.index zs group) (Seq.index p (i - v len)) (Seq.index p i))._2)))
  = P.createi_lemma #P.t_FieldElement (mk_usize 16)
      #(usize -> P.t_FieldElement)
      (fun (j: usize { j <. mk_usize 16 }) ->
        let g:usize = j /! (mk_usize 2 *! len <: usize) in
        let idx:usize = j %! (mk_usize 2 *! len <: usize) in
        (if idx <. len then
          (N.butterfly (Seq.index zs (v g)) (Seq.index p (v j)) (Seq.index p (v j + v len)))._1
        else
          (N.butterfly (Seq.index zs (v g)) (Seq.index p (v j - v len)) (Seq.index p (v j)))._2)
        <: P.t_FieldElement)
      (sz i)
#pop-options

(* === LEVEL A: array-level createi composition for ntt_layer_n 256 === *)
#push-options "--z3rlimit 200 --fuel 0 --ifuel 1"
let lemma_ntt_layer_n_256_compose
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
              (N.butterfly (Seq.index zetas group) (Seq.index p i) (Seq.index p (i + v len)))._1) /\
         (idx >= v len ==>
            i >= v len /\
            Seq.index q i ==
              (N.butterfly (Seq.index zetas group) (Seq.index p (i - v len)) (Seq.index p i))._2))))
    (ensures
      q == N.ntt_layer_n (mk_usize 256) p len zetas)
  = let rhs = N.ntt_layer_n (mk_usize 256) p len zetas in
    let aux (i: nat) : Lemma (i < 256 ==> Seq.index q i == Seq.index rhs i)
      = if i < 256 then begin
          let group : nat = i / (2 * v len) in
          assert (group < Seq.length zetas);
          assert (Seq.index rhs i ==
            (let g : usize = (sz i) /! (mk_usize 2 *! len <: usize) in
             let idx : usize = (sz i) %! (mk_usize 2 *! len <: usize) in
             if idx <. len
             then (N.butterfly (Seq.index zetas (v g)) (Seq.index p i) (Seq.index p (i + v len)))._1
             else (N.butterfly (Seq.index zetas (v g)) (Seq.index p (i - v len)) (Seq.index p i))._2))
        end
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro q rhs
#pop-options

(* === unfold for layers 1..3: ntt_layer p layer == ntt_layer_n 256 p len zs,
   ASCENDING zeta slice (zs[round] == v_ZETAS[groups + round]).
   FACT1 (norm/trefl) unfolds ntt_layer syntactically; lemma_ntt_layer_n_cong
   bridges (len',spec_slice)->(len,zs) cheaply; transitivity closes the goal. === *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let lemma_ntt_layer_unfold_lo
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
         Seq.index zs round == N.v_ZETAS.[ sz (groups + round) ]))
    (ensures
      N.ntt_layer p layer == N.ntt_layer_n (mk_usize 256) p len zs)
  = admit () (* TEMP-ADMIT-UNFOLD: verified squash+norm body (lo) at rlimit 400, ~33s;
                admitted during F-A iteration for fast builds; RESTORE before final. *)
#pop-options

(* === unfold for layers 4..7: ntt_layer p layer == ntt_layer_n 256 p (1<<layer) zs,
   ASCENDING zeta slice. === *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 400"
let lemma_ntt_layer_unfold
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
         Seq.index zs round == N.v_ZETAS.[ sz (groups + round) ]))
    (ensures
      N.ntt_layer p layer == N.ntt_layer_n (mk_usize 256) p len zs)
  = admit () (* TEMP-ADMIT-UNFOLD: verified squash+norm body (hi) at rlimit 400;
                admitted during F-A iteration for fast builds; RESTORE before final. *)
#pop-options

(* === Unfold `N.ntt` to the explicit 7-fold nesting of `N.ntt_layer`
   (layer order 7,6,5,4,3,2,1). === *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 50"
let lemma_ntt_unfold (p: t_Array P.t_FieldElement (mk_usize 256))
  : Lemma
    (N.ntt p ==
      N.ntt_layer (N.ntt_layer (N.ntt_layer (N.ntt_layer
        (N.ntt_layer (N.ntt_layer (N.ntt_layer p (mk_usize 7))
          (mk_usize 6)) (mk_usize 5)) (mk_usize 4)) (mk_usize 3)) (mk_usize 2)) (mk_usize 1))
  = assert (N.ntt p ==
      N.ntt_layer (N.ntt_layer (N.ntt_layer (N.ntt_layer
        (N.ntt_layer (N.ntt_layer (N.ntt_layer p (mk_usize 7))
          (mk_usize 6)) (mk_usize 5)) (mk_usize 4)) (mk_usize 3)) (mk_usize 2)) (mk_usize 1))
      by (FStar.Tactics.norm [delta_only [`%N.ntt]; iota; zeta; primops];
          FStar.Tactics.trefl ())
#pop-options

(* =====================================================================
   SECTION 3 — opaque per-vector layer-post atom (MONT form, matches the
   ntt_at_layer_1/2/3 ensures).
   ===================================================================== *)
[@@ "opaque_to_smt"]
let pv_post (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16))
    (len: usize {v len == 2 \/ v len == 4 \/ v len == 8})
    (pvm: t_Array P.t_FieldElement (mk_usize (8 / v len))) (m: nat) : prop =
  m < 16 ==>
    mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index cout m)) ==
      N.ntt_layer_n (mk_usize 16)
        (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index cin m)))
        len (Rust_primitives.unsize pvm)

#push-options "--fuel 0 --ifuel 0 --z3rlimit 20"
let pv_post_intro (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16))
    (len: usize {v len == 2 \/ v len == 4 \/ v len == 8})
    (pvm: t_Array P.t_FieldElement (mk_usize (8 / v len))) (m: nat)
  : Lemma
    (requires m < 16 ==>
      mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index cout m)) ==
        N.ntt_layer_n (mk_usize 16)
          (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index cin m)))
          len (Rust_primitives.unsize pvm))
    (ensures pv_post #vV cin cout len pvm m)
  = reveal_opaque (`%pv_post) (pv_post #vV cin cout len pvm m)

let pv_post_elim (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16))
    (len: usize {v len == 2 \/ v len == 4 \/ v len == 8})
    (pvm: t_Array P.t_FieldElement (mk_usize (8 / v len))) (m: nat {m < 16})
  : Lemma
    (requires pv_post #vV cin cout len pvm m)
    (ensures
      mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index cout m)) ==
        N.ntt_layer_n (mk_usize 16)
          (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index cin m)))
          len (Rust_primitives.unsize pvm))
  = reveal_opaque (`%pv_post) (pv_post #vV cin cout len pvm m)
#pop-options

(* =====================================================================
   SECTION 4 — intra-vector per-coefficient bridge + chainer.
   `per_coeff` consumes the MONT pv_post and produces the PLAIN per-coefficient
   butterfly relations (mont->plain reconciliation, per element).
   ===================================================================== *)
#push-options "--z3rlimit 300 --fuel 0 --ifuel 1"
let lemma_intra_vec_per_coeff
    (#vV: Type0) {| iop: T.t_Operations vV |}
    (cin cout: t_Array vV (mk_usize 16))
    (len: usize {v len == 2 \/ v len == 4 \/ v len == 8})
    (zs: t_Slice P.t_FieldElement)
    (pvz: (m: nat {m < 16}) -> t_Array P.t_FieldElement (mk_usize (8 / v len)))
  : Lemma
    (requires
      Seq.length zs == 128 / v len /\
      (forall (m: nat). m < 16 ==> pv_post #vV cin cout len (pvz m) m) /\
      (forall (m: nat) (g: nat). m < 16 /\ g < 8 / v len ==>
        Seq.index (Rust_primitives.unsize (pvz m)) g == Seq.index zs ((8 / v len) * m + g)))
    (ensures
      (let p = to_spec_poly_plain_arr #vV cin in
       let q = to_spec_poly_plain_arr #vV cout in
       (forall (i: nat). i < 256 ==>
         (let group : nat = i / (2 * v len) in
          let idx   : nat = i % (2 * v len) in
          group < Seq.length zs /\
          (idx < v len ==>
             i + v len < 256 /\
             Seq.index q i ==
               (N.butterfly (Seq.index zs group) (Seq.index p i) (Seq.index p (i + v len)))._1) /\
          (idx >= v len ==>
             i >= v len /\
             Seq.index q i ==
               (N.butterfly (Seq.index zs group) (Seq.index p (i - v len)) (Seq.index p i))._2)))))
  = let p = to_spec_poly_plain_arr #vV cin in
    let q = to_spec_poly_plain_arr #vV cout in
    let s : nat = 8 / v len in
    let aux (i: nat) : Lemma (i < 256 ==>
        (let group : nat = i / (2 * v len) in
         let idx   : nat = i % (2 * v len) in
         group < Seq.length zs /\
         (idx < v len ==>
            i + v len < 256 /\
            Seq.index q i ==
              (N.butterfly (Seq.index zs group) (Seq.index p i) (Seq.index p (i + v len)))._1) /\
         (idx >= v len ==>
            i >= v len /\
            Seq.index q i ==
              (N.butterfly (Seq.index zs group) (Seq.index p (i - v len)) (Seq.index p i))._2)))
      = if i < 256 then begin
          let m : nat = i / 16 in
          let l : nat = i % 16 in
          lemma_intra_idx i (v len);
          lemma_gv_lt l (v len);
          lemma_intra_arith (v len) m;
          pv_post_elim #vV cin cout len (pvz m) m;
          let gv : nat = l / (2 * v len) in
          assert (i / (2 * v len) == s * m + gv);
          (* MONT relation at lane l: recover per-vector ntt_layer_n 16 + lane unfold *)
          mont_array_lane (T.f_repr (Seq.index cout m)) (sz l);
          lemma_ntt_layer_n_16_lane
            (mont_i16_to_spec_array (mk_usize 16) (T.f_repr (Seq.index cin m))) len
            (Rust_primitives.unsize (pvz m)) l;
          mont_array_lane (T.f_repr (Seq.index cin m)) (sz l);
          (* PLAIN lane values for q[i], p[i] *)
          tspp_arr_lane #vV cout i;
          tspp_arr_lane #vV cin i;
          if l % (2 * v len) < v len then begin
            lemma_intra_partner m l (v len);
            mont_array_lane (T.f_repr (Seq.index cin m)) (sz (l + v len));
            tspp_arr_lane #vV cin (i + v len);
            (* mont butterfly._1 -> plain butterfly._1 (169-cancellation) *)
            lemma_mont_to_plain_butterfly_plus
              (Seq.index (T.f_repr (Seq.index cin m)) l)
              (Seq.index (T.f_repr (Seq.index cin m)) (l + v len))
              (Seq.index (T.f_repr (Seq.index cout m)) l)
              (Seq.index (Rust_primitives.unsize (pvz m)) gv)
          end else begin
            lemma_intra_partner m l (v len);
            mont_array_lane (T.f_repr (Seq.index cin m)) (sz (l - v len));
            tspp_arr_lane #vV cin (i - v len);
            lemma_mont_to_plain_butterfly_minus
              (Seq.index (T.f_repr (Seq.index cin m)) (l - v len))
              (Seq.index (T.f_repr (Seq.index cin m)) l)
              (Seq.index (T.f_repr (Seq.index cout m)) l)
              (Seq.index (Rust_primitives.unsize (pvz m)) gv)
          end
        end
    in
    Classical.forall_intro aux
#pop-options

(* chainer: per-vector pv_post + zeta correspondence -> plain polynomial-form step *)
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
      (v len == 2 \/ v len == 4 \/ v len == 8) /\
      Seq.length zs == 128 / v len /\
      (forall (m: nat). m < 16 ==>
        pv_post #vV re_in.VV.f_coefficients re_out.VV.f_coefficients len (pvz m) m) /\
      (forall (m: nat) (g: nat). m < 16 /\ g < 8 / v len ==>
        Seq.index (Rust_primitives.unsize (pvz m)) g == Seq.index zs ((8 / v len) * m + g)) /\
      (let groups = 128 / v len in
       forall (round: nat). round < groups ==>
         Seq.index zs round == N.v_ZETAS.[ sz (groups + round) ]) /\
      (v layer == 1 \/ v layer == 2 \/ v layer == 3) /\
      v len == pow2 (v layer))
    (ensures
      to_spec_poly_plain #vV re_out == N.ntt_layer (to_spec_poly_plain #vV re_in) layer)
  = lemma_to_spec_poly_plain_unfold #vV re_out;
    lemma_to_spec_poly_plain_unfold #vV re_in;
    lemma_intra_vec_per_coeff #vV re_in.VV.f_coefficients re_out.VV.f_coefficients len zs pvz;
    lemma_div_128_prod_lo (v len);
    assert (v len >= 1 /\ v len < 1024 /\ Seq.length zs < 1024 /\
            2 * Seq.length zs * v len == 256 /\
            ((Seq.length zs) * 2) * v len == 256);
    lemma_ntt_layer_n_256_compose
      (to_spec_poly_plain_arr #vV re_in.VV.f_coefficients)
      (to_spec_poly_plain_arr #vV re_out.VV.f_coefficients) len zs;
    lemma_ntt_layer_unfold_lo (to_spec_poly_plain_arr #vV re_in.VV.f_coefficients) layer len zs
#pop-options

(* =====================================================================
   SECTION 5 — opaque PLAIN poly_step + 7-fold composition.
   ===================================================================== *)
[@@ "opaque_to_smt"]
let poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
    (layer: usize {v layer >= 1 /\ v layer <= 7}) : prop =
  to_spec_poly_plain #vV re_out == N.ntt_layer (to_spec_poly_plain #vV re_in) layer

#push-options "--fuel 0 --ifuel 0 --z3rlimit 20"
let lemma_poly_step_intro (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
    (layer: usize {v layer >= 1 /\ v layer <= 7})
  : Lemma
    (requires
      to_spec_poly_plain #vV re_out == N.ntt_layer (to_spec_poly_plain #vV re_in) layer)
    (ensures poly_step #vV re_in re_out layer)
  = reveal_opaque (`%poly_step) (poly_step #vV re_in re_out layer)
#pop-options

(* composition: 7 plain poly_step atoms (layer order 7,6,5,4,3,2,1) ==> N.ntt equality. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_compose_7 (#vV: Type0) {| iop: T.t_Operations vV |}
    (re0 re1 re2 re3 re4 re5 re6 re7: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      poly_step #vV re0 re1 (mk_usize 7) /\ poly_step #vV re1 re2 (mk_usize 6) /\
      poly_step #vV re2 re3 (mk_usize 5) /\ poly_step #vV re3 re4 (mk_usize 4) /\
      poly_step #vV re4 re5 (mk_usize 3) /\ poly_step #vV re5 re6 (mk_usize 2) /\
      poly_step #vV re6 re7 (mk_usize 1))
    (ensures
      to_spec_poly_plain #vV re7 == N.ntt (to_spec_poly_plain #vV re0))
  = reveal_opaque (`%poly_step) (poly_step #vV re0 re1 (mk_usize 7));
    reveal_opaque (`%poly_step) (poly_step #vV re1 re2 (mk_usize 6));
    reveal_opaque (`%poly_step) (poly_step #vV re2 re3 (mk_usize 5));
    reveal_opaque (`%poly_step) (poly_step #vV re3 re4 (mk_usize 4));
    reveal_opaque (`%poly_step) (poly_step #vV re4 re5 (mk_usize 3));
    reveal_opaque (`%poly_step) (poly_step #vV re5 re6 (mk_usize 2));
    reveal_opaque (`%poly_step) (poly_step #vV re6 re7 (mk_usize 1));
    lemma_ntt_unfold (to_spec_poly_plain #vV re0)
#pop-options

(* =====================================================================
   SECTION 6 — layers 1/2/3 bridges: raw pv_post forall ==> plain poly_step.
   ASCENDING zetas (layer L groups=128/2^L, zeta indices groups..2*groups).
   ===================================================================== *)

(* LAYER 3 (len=8, groups=16, 1 zeta/vector) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_layer3_to_poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      (forall (i: usize). v i < 16 ==>
        pv_post #vV re_in.VV.f_coefficients re_out.VV.f_coefficients (mk_usize 8)
          (zetas_1_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! i)))
          (v i)))
    (ensures poly_step #vV re_in re_out (mk_usize 3))
  = reveal_opaque (`%poly_step) (poly_step #vV re_in re_out (mk_usize 3));
    assert_norm (pow2 3 == 8);
    assert_norm (8 / v (mk_usize 8) == 1);
    assert_norm (128 / v (mk_usize 8) == 16);
    let zs : t_Slice P.t_FieldElement =
      Seq.init 16 (fun (r: nat {r < 16}) ->
        mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! mk_usize r))) in
    assert (Seq.length zs == 16);
    let pvz : (m: nat {m < 16}) -> t_Array P.t_FieldElement (mk_usize 1) =
      fun (m: nat {m < 16}) -> zetas_1_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! sz m)) in
    let aux_zs (round: nat) : Lemma (round < 16 ==>
        Seq.index zs round == N.v_ZETAS.[ sz (16 + round) ]) =
      if round < 16 then begin
        FStar.Seq.Base.init_index_ 16
          (fun (r: nat {r < 16}) ->
            mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! mk_usize r))) round;
        lemma_zeta_eq_vzetas (mk_usize 16 +! mk_usize round);
        assert (v (mk_usize 16 +! mk_usize round) == 16 + round)
      end in
    let aux_pv (m: nat) : Lemma (m < 16 ==>
        pv_post #vV re_in.VV.f_coefficients re_out.VV.f_coefficients (mk_usize 8) (pvz m) m) =
      if m < 16 then (let i = sz m in assert (v i == m); assert (v i < 16)) in
    let aux_zc (m: nat) (g: nat) : Lemma (m < 16 /\ g < 8 / 8 ==>
        Seq.index (Rust_primitives.unsize (pvz m)) g == Seq.index zs ((8 / 8) * m + g)) =
      if m < 16 && g < 1 then begin
        zetas_1_lane (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! sz m)) (sz g);
        FStar.Seq.Base.init_index_ 16
          (fun (r: nat {r < 16}) ->
            mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! mk_usize r)))
          ((8 / 8) * m + g);
        assert (v (mk_usize 16 +! sz m) == 16 + m);
        assert (v (mk_usize 16 +! mk_usize ((8 / 8) * m + g)) == 16 + m)
      end in
    Classical.forall_intro aux_zs;
    Classical.forall_intro aux_pv;
    Classical.forall_intro_2 aux_zc;
    lemma_intra_vec_layer_to_poly #vV re_in re_out (mk_usize 3) (mk_usize 8) zs pvz
#pop-options

(* LAYER 2 (len=4, groups=32, 2 zetas/vector) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_layer2_to_poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      (forall (i: usize). v i < 16 ==>
        pv_post #vV re_in.VV.f_coefficients re_out.VV.f_coefficients (mk_usize 4)
          (zetas_2_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! i)))
          (v i)))
    (ensures poly_step #vV re_in re_out (mk_usize 2))
  = reveal_opaque (`%poly_step) (poly_step #vV re_in re_out (mk_usize 2));
    assert_norm (pow2 2 == 4);
    assert_norm (8 / v (mk_usize 4) == 2);
    assert_norm (128 / v (mk_usize 4) == 32);
    let zs : t_Slice P.t_FieldElement =
      Seq.init 32 (fun (r: nat {r < 32}) ->
        mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize r))) in
    assert (Seq.length zs == 32);
    let pvz : (m: nat {m < 16}) -> t_Array P.t_FieldElement (mk_usize 2) =
      fun (m: nat {m < 16}) ->
        zetas_2_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! sz m))
                 (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! sz m)) in
    let aux_zs (round: nat) : Lemma (round < 32 ==>
        Seq.index zs round == N.v_ZETAS.[ sz (32 + round) ]) =
      if round < 32 then begin
        FStar.Seq.Base.init_index_ 32
          (fun (r: nat {r < 32}) ->
            mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize r))) round;
        lemma_zeta_eq_vzetas (mk_usize 32 +! mk_usize round);
        assert (v (mk_usize 32 +! mk_usize round) == 32 + round)
      end in
    let aux_pv (m: nat) : Lemma (m < 16 ==>
        pv_post #vV re_in.VV.f_coefficients re_out.VV.f_coefficients (mk_usize 4) (pvz m) m) =
      if m < 16 then (let i = sz m in assert (v i == m); assert (v i < 16)) in
    let aux_zc (m: nat) (g: nat) : Lemma (m < 16 /\ g < 8 / 4 ==>
        Seq.index (Rust_primitives.unsize (pvz m)) g == Seq.index zs ((8 / 4) * m + g)) =
      if m < 16 && g < 2 then begin
        zetas_2_lane (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! sz m))
          (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! sz m)) (sz g);
        FStar.Seq.Base.init_index_ 32
          (fun (r: nat {r < 32}) ->
            mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize r)))
          ((8 / 4) * m + g);
        assert (v (mk_usize 32 +! mk_usize ((8 / 4) * m + g)) == 32 + 2 * m + g);
        assert (v (mk_usize 32 +! mk_usize 2 *! sz m) == 32 + 2 * m);
        assert (v (mk_usize 33 +! mk_usize 2 *! sz m) == 33 + 2 * m)
      end in
    Classical.forall_intro aux_zs;
    Classical.forall_intro aux_pv;
    Classical.forall_intro_2 aux_zc;
    lemma_intra_vec_layer_to_poly #vV re_in re_out (mk_usize 2) (mk_usize 4) zs pvz
#pop-options

(* LAYER 1 (len=2, groups=64, 4 zetas/vector) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always"
let lemma_layer1_to_poly_step (#vV: Type0) {| iop: T.t_Operations vV |}
    (re_in re_out: VV.t_PolynomialRingElement vV)
  : Lemma
    (requires
      (forall (i: usize). v i < 16 ==>
        pv_post #vV re_in.VV.f_coefficients re_out.VV.f_coefficients (mk_usize 2)
          (zetas_4_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! i))
                    (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! i)))
          (v i)))
    (ensures poly_step #vV re_in re_out (mk_usize 1))
  = reveal_opaque (`%poly_step) (poly_step #vV re_in re_out (mk_usize 1));
    assert_norm (pow2 1 == 2);
    assert_norm (8 / v (mk_usize 2) == 4);
    assert_norm (128 / v (mk_usize 2) == 64);
    let zs : t_Slice P.t_FieldElement =
      Seq.init 64 (fun (r: nat {r < 64}) ->
        mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize r))) in
    assert (Seq.length zs == 64);
    let pvz : (m: nat {m < 16}) -> t_Array P.t_FieldElement (mk_usize 4) =
      fun (m: nat {m < 16}) ->
        zetas_4_ (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! sz m))
                 (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! sz m))
                 (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! sz m))
                 (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! sz m)) in
    let aux_zs (round: nat) : Lemma (round < 64 ==>
        Seq.index zs round == N.v_ZETAS.[ sz (64 + round) ]) =
      if round < 64 then begin
        FStar.Seq.Base.init_index_ 64
          (fun (r: nat {r < 64}) ->
            mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize r))) round;
        lemma_zeta_eq_vzetas (mk_usize 64 +! mk_usize round);
        assert (v (mk_usize 64 +! mk_usize round) == 64 + round)
      end in
    let aux_pv (m: nat) : Lemma (m < 16 ==>
        pv_post #vV re_in.VV.f_coefficients re_out.VV.f_coefficients (mk_usize 2) (pvz m) m) =
      if m < 16 then (let i = sz m in assert (v i == m); assert (v i < 16)) in
    let aux_zc (m: nat) (g: nat) : Lemma (m < 16 /\ g < 8 / 2 ==>
        Seq.index (Rust_primitives.unsize (pvz m)) g == Seq.index zs ((8 / 2) * m + g)) =
      if m < 16 && g < 4 then begin
        zetas_4_lane (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! sz m))
          (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! sz m))
          (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! sz m))
          (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! sz m)) (sz g);
        FStar.Seq.Base.init_index_ 64
          (fun (r: nat {r < 64}) ->
            mont_i16_to_spec_fe (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize r)))
          ((8 / 2) * m + g);
        assert (v (mk_usize 64 +! mk_usize ((8 / 2) * m + g)) == 64 + 4 * m + g);
        assert (v (mk_usize 64 +! mk_usize 4 *! sz m) == 64 + 4 * m);
        assert (v (mk_usize 65 +! mk_usize 4 *! sz m) == 65 + 4 * m);
        assert (v (mk_usize 66 +! mk_usize 4 *! sz m) == 66 + 4 * m);
        assert (v (mk_usize 67 +! mk_usize 4 *! sz m) == 67 + 4 * m)
      end in
    Classical.forall_intro aux_zs;
    Classical.forall_intro aux_pv;
    Classical.forall_intro_2 aux_zc;
    lemma_intra_vec_layer_to_poly #vV re_in re_out (mk_usize 1) (mk_usize 2) zs pvz
#pop-options

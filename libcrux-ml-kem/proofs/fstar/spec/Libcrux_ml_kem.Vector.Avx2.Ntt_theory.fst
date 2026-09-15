module Libcrux_ml_kem.Vector.Avx2.Ntt_theory
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

(* Hand-written proof theory relocated from libcrux-ml-kem/src/vector/avx2/ntt.rs
   `hax_lib::fstar::before` blocks (byte-exact from the green extracted
   Libcrux_ml_kem.Vector.Avx2.Ntt.fst). Consumed only by that module. *)

(* ── Forward-NTT layer-1 lemma library.  Same within-128-bit-lane shuffle_epi32
   architecture as the inverse layers, but the "add" operand is a montgomery
   residue (only known mod 3329), so the post-helper (lemma_fwd_l1_post)
   bridges via mod-add-distributivity (lemma_modadd); the subtraction in
   ntt_spec comes from the negated zetas entries.  fwd_shuffle_* /
   lemma_fwd_l1_* are this layer's copies (distinct names so they don't
   collide with the inverse-layer blocks below). ── *)

module FI = Libcrux_intrinsics.Avx2
module FS = Spec.Utils
module FA = Libcrux_ml_kem.Vector.Avx2.Arithmetic

// Per-lane lifting lemmas: confine the map2/createi reasoning to these two
// once-proven bodies (verify at module-default rlimit, no cascade), so the
// *_sums helpers' lane foralls reason over clean `+.`/`mul_mod` equalities.
// NO SMTPat — a global trigger here causes "incomplete quantifiers" on the
// forward ntt_layer_1/2 leaves; these are called explicitly from the
// forall-aux inside lemma_inv_l2_sums instead (createi paid once, here).
let lemma_get_lane_add (a b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (i:nat{i < 16}) : Lemma
  (ensures Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_add_epi16 a b) i == (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a i) +. (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b i))
  = ()

let lemma_get_lane_mullo (a b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (i:nat{i < 16}) : Lemma
  (ensures Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_mullo_epi16 a b) i == mul_mod (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a i) (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b i))
  = ()

// ───────────────────────────────────────────────────────────────────────────
// B' lane-permutation value lemmas: each reveals the (opaque) crate index
// helper in a clean context and pins the per-field result for one concrete
// control via a literal match.  Consumers call these to obtain ground
// shuffle32_src/permute64_src/blend_sel facts, then the op-ensures forall
// (pattern (get_lane result k)) maps each output lane to its source lane.
// ───────────────────────────────────────────────────────────────────────────
#push-options "--fuel 2 --ifuel 2 --z3rlimit 50"
let shuf245 (j:nat{j<8}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src (mk_i32 245) j ==
   (if j=0 then 1 else if j=1 then 1 else if j=2 then 3 else if j=3 then 3
    else if j=4 then 5 else if j=5 then 5 else if j=6 then 7 else 7))
  = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src) (Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src (mk_i32 245));
    (match j with |0->()|1->()|2->()|3->()|4->()|5->()|6->()|7->()|_->())

let shuf160 (j:nat{j<8}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src (mk_i32 160) j ==
   (if j=0 then 0 else if j=1 then 0 else if j=2 then 2 else if j=3 then 2
    else if j=4 then 4 else if j=5 then 4 else if j=6 then 6 else 6))
  = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src) (Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src (mk_i32 160));
    (match j with |0->()|1->()|2->()|3->()|4->()|5->()|6->()|7->()|_->())

let shuf238 (j:nat{j<8}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src (mk_i32 238) j ==
   (if j=0 then 2 else if j=1 then 3 else if j=2 then 2 else if j=3 then 3
    else if j=4 then 6 else if j=5 then 7 else if j=6 then 6 else 7))
  = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src) (Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src (mk_i32 238));
    (match j with |0->()|1->()|2->()|3->()|4->()|5->()|6->()|7->()|_->())

let shuf68 (j:nat{j<8}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src (mk_i32 68) j ==
   (if j=0 then 0 else if j=1 then 1 else if j=2 then 0 else if j=3 then 1
    else if j=4 then 4 else if j=5 then 5 else if j=6 then 4 else 5))
  = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src) (Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src (mk_i32 68));
    (match j with |0->()|1->()|2->()|3->()|4->()|5->()|6->()|7->()|_->())

let perm245 (q:nat{q<4}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src (mk_i32 245) q == (if q=0 then 1 else if q=1 then 1 else if q=2 then 3 else 3))
  = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src) (Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src (mk_i32 245));
    (match q with |0->()|1->()|2->()|3->()|_->())

let perm160 (q:nat{q<4}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src (mk_i32 160) q == (if q=0 then 0 else if q=1 then 0 else if q=2 then 2 else 2))
  = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src) (Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src (mk_i32 160));
    (match q with |0->()|1->()|2->()|3->()|_->())

let perm216 (q:nat{q<4}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src (mk_i32 216) q == (if q=0 then 0 else if q=1 then 2 else if q=2 then 1 else 3))
  = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src) (Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src (mk_i32 216));
    (match q with |0->()|1->()|2->()|3->()|_->())

let blendsel204 (k:nat{k<16}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.blend_sel (mk_i32 204) k == (k%8=2 || k%8=3 || k%8=6 || k%8=7))
  = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.blend_sel) (Libcrux_intrinsics.Avx2_ml_kem_views.blend_sel (mk_i32 204));
    (match k%8 with |0->()|1->()|2->()|3->()|4->()|5->()|6->()|7->()|_->())

let blendsel240 (k:nat{k<16}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.blend_sel (mk_i32 240) k == (k%8=4 || k%8=5 || k%8=6 || k%8=7))
  = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.blend_sel) (Libcrux_intrinsics.Avx2_ml_kem_views.blend_sel (mk_i32 240));
    (match k%8 with |0->()|1->()|2->()|3->()|4->()|5->()|6->()|7->()|_->())

let blendsel170 (k:nat{k<16}) : Lemma (Libcrux_intrinsics.Avx2_ml_kem_views.blend_sel (mk_i32 170) k == (k%2=1))
  = reveal_opaque (`%Libcrux_intrinsics.Avx2_ml_kem_views.blend_sel) (Libcrux_intrinsics.Avx2_ml_kem_views.blend_sel (mk_i32 170));
    (match k%8 with |0->()|1->()|2->()|3->()|4->()|5->()|6->()|7->()|_->())
#pop-options

let fwd_shuffle_245 (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) vv in
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 11 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 11 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 15 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 15))
  = let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) vv in
    shuf245 0; shuf245 1; shuf245 2; shuf245 3; shuf245 4; shuf245 5; shuf245 6; shuf245 7

let fwd_shuffle_160 (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 160) vv in
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 13 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 13))
  = let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 160) vv in
    shuf160 0; shuf160 1; shuf160 2; shuf160 3; shuf160 4; shuf160 5; shuf160 6; shuf160 7

(* `c` is symbolic here, so the shuffle fact's immediate-range side condition has
   to be taken as a hypothesis; both call sites pass a literal (245 / 160). *)
let fwd_shuffle_preserves_bound (c: i32) (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (b: nat) : Lemma
  (requires v c >= 0 /\ v c < 256 /\
            FS.is_i16b_array b (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vv))
  (ensures FS.is_i16b_array b (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 c vv)))
  = let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 c vv in
    let aux (i:nat{i<16}) : Lemma (FS.is_i16b b (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 r) i)) =
      assert (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r i == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv (2 * Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src c (i/2) + i%2)) in
    Classical.forall_intro aux

#push-options "--z3rlimit 400 --split_queries always"
let lemma_fwd_l1_add (lhs rhs result: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
     result == Libcrux_intrinsics.Avx2.mm256_add_epi16 lhs rhs /\
     FS.is_i16b_array (7*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 lhs) /\
     FS.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 rhs))
  (ensures
     FS.is_i16b_array (8*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 result) /\
     (forall (i:nat). i < 16 ==>
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result i) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs i) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs i)))
  = ()
#pop-options

#push-options "--z3rlimit 300 --split_queries always"
let lemma_fwd_l1_resultv (vector lhs rhs result: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
     result == Libcrux_intrinsics.Avx2.mm256_add_epi16 lhs rhs /\
     FS.is_i16b_array (7*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 lhs) /\
     FS.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 rhs) /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 13 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 13)
  (ensures
     FS.is_i16b_array (8*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 result) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 0)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 0) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 1)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 1) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 2)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 2) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 3)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 3) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 4)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 4)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 4) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 5)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 5)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 5) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 6)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 4)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 6) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 7)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 5)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 7) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 8)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 8) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 9)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 9) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 10) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 10) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 11) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 11) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 12) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 12) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 12) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 13) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 13) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 13) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 14) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 12) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 14) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 15) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 13) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 15))
  = lemma_fwd_l1_add lhs rhs result
#pop-options

let lemma_modadd (a r x:int) : Lemma
  (requires r % 3329 == x % 3329)
  (ensures (a + r) % 3329 == (a + x) % 3329)
  = FStar.Math.Lemmas.lemma_mod_add_distr a r 3329;
    FStar.Math.Lemmas.lemma_mod_add_distr a x 3329

#push-options "--z3rlimit 300 --split_queries always"
let lemma_fwd_l1_post
    (vec rhs zetas result: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16)
  : Lemma
    (requires
      v (Seq.index result 0)  == v (Seq.index vec 0)  + v (Seq.index rhs 0) /\
      v (Seq.index result 1)  == v (Seq.index vec 1)  + v (Seq.index rhs 1) /\
      v (Seq.index result 2)  == v (Seq.index vec 0)  + v (Seq.index rhs 2) /\
      v (Seq.index result 3)  == v (Seq.index vec 1)  + v (Seq.index rhs 3) /\
      v (Seq.index result 4)  == v (Seq.index vec 4)  + v (Seq.index rhs 4) /\
      v (Seq.index result 5)  == v (Seq.index vec 5)  + v (Seq.index rhs 5) /\
      v (Seq.index result 6)  == v (Seq.index vec 4)  + v (Seq.index rhs 6) /\
      v (Seq.index result 7)  == v (Seq.index vec 5)  + v (Seq.index rhs 7) /\
      v (Seq.index result 8)  == v (Seq.index vec 8)  + v (Seq.index rhs 8) /\
      v (Seq.index result 9)  == v (Seq.index vec 9)  + v (Seq.index rhs 9) /\
      v (Seq.index result 10) == v (Seq.index vec 8)  + v (Seq.index rhs 10) /\
      v (Seq.index result 11) == v (Seq.index vec 9)  + v (Seq.index rhs 11) /\
      v (Seq.index result 12) == v (Seq.index vec 12) + v (Seq.index rhs 12) /\
      v (Seq.index result 13) == v (Seq.index vec 13) + v (Seq.index rhs 13) /\
      v (Seq.index result 14) == v (Seq.index vec 12) + v (Seq.index rhs 14) /\
      v (Seq.index result 15) == v (Seq.index vec 13) + v (Seq.index rhs 15) /\
      v (Seq.index rhs 0)  % 3329 == (v (Seq.index vec 2)  * v zeta0 * 169) % 3329 /\
      v (Seq.index rhs 1)  % 3329 == (v (Seq.index vec 3)  * v zeta0 * 169) % 3329 /\
      v (Seq.index rhs 2)  % 3329 == (v (Seq.index vec 2)  * (- v zeta0) * 169) % 3329 /\
      v (Seq.index rhs 3)  % 3329 == (v (Seq.index vec 3)  * (- v zeta0) * 169) % 3329 /\
      v (Seq.index rhs 4)  % 3329 == (v (Seq.index vec 6)  * v zeta1 * 169) % 3329 /\
      v (Seq.index rhs 5)  % 3329 == (v (Seq.index vec 7)  * v zeta1 * 169) % 3329 /\
      v (Seq.index rhs 6)  % 3329 == (v (Seq.index vec 6)  * (- v zeta1) * 169) % 3329 /\
      v (Seq.index rhs 7)  % 3329 == (v (Seq.index vec 7)  * (- v zeta1) * 169) % 3329 /\
      v (Seq.index rhs 8)  % 3329 == (v (Seq.index vec 10) * v zeta2 * 169) % 3329 /\
      v (Seq.index rhs 9)  % 3329 == (v (Seq.index vec 11) * v zeta2 * 169) % 3329 /\
      v (Seq.index rhs 10) % 3329 == (v (Seq.index vec 10) * (- v zeta2) * 169) % 3329 /\
      v (Seq.index rhs 11) % 3329 == (v (Seq.index vec 11) * (- v zeta2) * 169) % 3329 /\
      v (Seq.index rhs 12) % 3329 == (v (Seq.index vec 14) * v zeta3 * 169) % 3329 /\
      v (Seq.index rhs 13) % 3329 == (v (Seq.index vec 15) * v zeta3 * 169) % 3329 /\
      v (Seq.index rhs 14) % 3329 == (v (Seq.index vec 14) * (- v zeta3) * 169) % 3329 /\
      v (Seq.index rhs 15) % 3329 == (v (Seq.index vec 15) * (- v zeta3) * 169) % 3329 /\
      FS.is_i16b_array (8*3328) result)
    (ensures
      FS.is_i16b_array (8*3328) result /\
      FS.ntt_layer_1_butterfly_post vec result zeta0 zeta1 zeta2 zeta3)
  =
  lemma_modadd (v (Seq.index vec 0)) (v (Seq.index rhs 0)) (v (Seq.index vec 2) * v zeta0 * 169);
  lemma_modadd (v (Seq.index vec 1)) (v (Seq.index rhs 1)) (v (Seq.index vec 3) * v zeta0 * 169);
  lemma_modadd (v (Seq.index vec 0)) (v (Seq.index rhs 2)) (v (Seq.index vec 2) * (- v zeta0) * 169);
  lemma_modadd (v (Seq.index vec 1)) (v (Seq.index rhs 3)) (v (Seq.index vec 3) * (- v zeta0) * 169);
  lemma_modadd (v (Seq.index vec 4)) (v (Seq.index rhs 4)) (v (Seq.index vec 6) * v zeta1 * 169);
  lemma_modadd (v (Seq.index vec 5)) (v (Seq.index rhs 5)) (v (Seq.index vec 7) * v zeta1 * 169);
  lemma_modadd (v (Seq.index vec 4)) (v (Seq.index rhs 6)) (v (Seq.index vec 6) * (- v zeta1) * 169);
  lemma_modadd (v (Seq.index vec 5)) (v (Seq.index rhs 7)) (v (Seq.index vec 7) * (- v zeta1) * 169);
  lemma_modadd (v (Seq.index vec 8)) (v (Seq.index rhs 8)) (v (Seq.index vec 10) * v zeta2 * 169);
  lemma_modadd (v (Seq.index vec 9)) (v (Seq.index rhs 9)) (v (Seq.index vec 11) * v zeta2 * 169);
  lemma_modadd (v (Seq.index vec 8)) (v (Seq.index rhs 10)) (v (Seq.index vec 10) * (- v zeta2) * 169);
  lemma_modadd (v (Seq.index vec 9)) (v (Seq.index rhs 11)) (v (Seq.index vec 11) * (- v zeta2) * 169);
  lemma_modadd (v (Seq.index vec 12)) (v (Seq.index rhs 12)) (v (Seq.index vec 14) * v zeta3 * 169);
  lemma_modadd (v (Seq.index vec 13)) (v (Seq.index rhs 13)) (v (Seq.index vec 15) * v zeta3 * 169);
  lemma_modadd (v (Seq.index vec 12)) (v (Seq.index rhs 14)) (v (Seq.index vec 14) * (- v zeta3) * 169);
  lemma_modadd (v (Seq.index vec 13)) (v (Seq.index rhs 15)) (v (Seq.index vec 15) * (- v zeta3) * 169);
  reveal_opaque (`%FS.ntt_layer_1_butterfly_post)
    (FS.ntt_layer_1_butterfly_post vec)
#pop-options

let fwd2_shuffle_238 (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 238) vv in
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 13 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 15 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 13 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 15))
  = let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 238) vv in
    shuf238 0; shuf238 1; shuf238 2; shuf238 3; shuf238 4; shuf238 5; shuf238 6; shuf238 7

let fwd2_shuffle_68 (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 68) vv in
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 11 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 11))
  = let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 68) vv in
    shuf68 0; shuf68 1; shuf68 2; shuf68 3; shuf68 4; shuf68 5; shuf68 6; shuf68 7

(* `c` is symbolic here, so the shuffle/permute fact's immediate-range side
   condition has to be taken as a hypothesis; every call site passes a literal. *)
let fwd2_shuffle_preserves_bound (c: i32) (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (b: nat) : Lemma
  (requires v c >= 0 /\ v c < 256 /\ FS.is_i16b_array b (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vv))
  (ensures FS.is_i16b_array b (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 c vv)))
  = let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 c vv in
    let aux (i:nat{i<16}) : Lemma (FS.is_i16b b (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 r) i)) =
      assert (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r i == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv (2 * Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src c (i/2) + i%2)) in
    Classical.forall_intro aux

#push-options "--z3rlimit 400 --split_queries always"
let lemma_fwd_l2_add (lhs rhs result: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
     result == Libcrux_intrinsics.Avx2.mm256_add_epi16 lhs rhs /\
     FS.is_i16b_array (6*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 lhs) /\
     FS.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 rhs))
  (ensures
     FS.is_i16b_array (7*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 result) /\
     (forall (i:nat). i < 16 ==>
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result i) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs i) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs i)))
  = ()
#pop-options

#push-options "--z3rlimit 300 --split_queries always"
let lemma_fwd_l2_resultv (vector lhs rhs result: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
     result == Libcrux_intrinsics.Avx2.mm256_add_epi16 lhs rhs /\
     FS.is_i16b_array (6*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 lhs) /\
     FS.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 rhs) /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 11 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 11)
  (ensures
     FS.is_i16b_array (7*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 result) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 0)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 0) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 1)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 1) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 2)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 2)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 2) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 3)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 3)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 3) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 4)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 4) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 5)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 5) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 6)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 2)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 6) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 7)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 3)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 7) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 8)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 8) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 9)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 9) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 10) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 10) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 10) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 11) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 11) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 11) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 12) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 12) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 13) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 13) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 14) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 10) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 14) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane result 15) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 11) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 15))
  = lemma_fwd_l2_add lhs rhs result
#pop-options

#push-options "--z3rlimit 300 --split_queries always"
let lemma_fwd_l2_post
    (vec rhs zetas result: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16)
  : Lemma
    (requires
      v (Seq.index result 0)  == v (Seq.index vec 0)  + v (Seq.index rhs 0) /\
      v (Seq.index result 1)  == v (Seq.index vec 1)  + v (Seq.index rhs 1) /\
      v (Seq.index result 2)  == v (Seq.index vec 2)  + v (Seq.index rhs 2) /\
      v (Seq.index result 3)  == v (Seq.index vec 3)  + v (Seq.index rhs 3) /\
      v (Seq.index result 4)  == v (Seq.index vec 0)  + v (Seq.index rhs 4) /\
      v (Seq.index result 5)  == v (Seq.index vec 1)  + v (Seq.index rhs 5) /\
      v (Seq.index result 6)  == v (Seq.index vec 2)  + v (Seq.index rhs 6) /\
      v (Seq.index result 7)  == v (Seq.index vec 3)  + v (Seq.index rhs 7) /\
      v (Seq.index result 8)  == v (Seq.index vec 8)  + v (Seq.index rhs 8) /\
      v (Seq.index result 9)  == v (Seq.index vec 9)  + v (Seq.index rhs 9) /\
      v (Seq.index result 10) == v (Seq.index vec 10) + v (Seq.index rhs 10) /\
      v (Seq.index result 11) == v (Seq.index vec 11) + v (Seq.index rhs 11) /\
      v (Seq.index result 12) == v (Seq.index vec 8)  + v (Seq.index rhs 12) /\
      v (Seq.index result 13) == v (Seq.index vec 9)  + v (Seq.index rhs 13) /\
      v (Seq.index result 14) == v (Seq.index vec 10) + v (Seq.index rhs 14) /\
      v (Seq.index result 15) == v (Seq.index vec 11) + v (Seq.index rhs 15) /\
      v (Seq.index rhs 0)  % 3329 == (v (Seq.index vec 4)  * v zeta0 * 169) % 3329 /\
      v (Seq.index rhs 1)  % 3329 == (v (Seq.index vec 5)  * v zeta0 * 169) % 3329 /\
      v (Seq.index rhs 2)  % 3329 == (v (Seq.index vec 6)  * v zeta0 * 169) % 3329 /\
      v (Seq.index rhs 3)  % 3329 == (v (Seq.index vec 7)  * v zeta0 * 169) % 3329 /\
      v (Seq.index rhs 4)  % 3329 == (v (Seq.index vec 4)  * (- v zeta0) * 169) % 3329 /\
      v (Seq.index rhs 5)  % 3329 == (v (Seq.index vec 5)  * (- v zeta0) * 169) % 3329 /\
      v (Seq.index rhs 6)  % 3329 == (v (Seq.index vec 6)  * (- v zeta0) * 169) % 3329 /\
      v (Seq.index rhs 7)  % 3329 == (v (Seq.index vec 7)  * (- v zeta0) * 169) % 3329 /\
      v (Seq.index rhs 8)  % 3329 == (v (Seq.index vec 12) * v zeta1 * 169) % 3329 /\
      v (Seq.index rhs 9)  % 3329 == (v (Seq.index vec 13) * v zeta1 * 169) % 3329 /\
      v (Seq.index rhs 10) % 3329 == (v (Seq.index vec 14) * v zeta1 * 169) % 3329 /\
      v (Seq.index rhs 11) % 3329 == (v (Seq.index vec 15) * v zeta1 * 169) % 3329 /\
      v (Seq.index rhs 12) % 3329 == (v (Seq.index vec 12) * (- v zeta1) * 169) % 3329 /\
      v (Seq.index rhs 13) % 3329 == (v (Seq.index vec 13) * (- v zeta1) * 169) % 3329 /\
      v (Seq.index rhs 14) % 3329 == (v (Seq.index vec 14) * (- v zeta1) * 169) % 3329 /\
      v (Seq.index rhs 15) % 3329 == (v (Seq.index vec 15) * (- v zeta1) * 169) % 3329 /\
      FS.is_i16b_array (7*3328) result)
    (ensures
      FS.is_i16b_array (7*3328) result /\
      FS.ntt_layer_2_butterfly_post vec result zeta0 zeta1)
  =
  lemma_modadd (v (Seq.index vec 0)) (v (Seq.index rhs 0)) (v (Seq.index vec 4) * v zeta0 * 169);
  lemma_modadd (v (Seq.index vec 1)) (v (Seq.index rhs 1)) (v (Seq.index vec 5) * v zeta0 * 169);
  lemma_modadd (v (Seq.index vec 2)) (v (Seq.index rhs 2)) (v (Seq.index vec 6) * v zeta0 * 169);
  lemma_modadd (v (Seq.index vec 3)) (v (Seq.index rhs 3)) (v (Seq.index vec 7) * v zeta0 * 169);
  lemma_modadd (v (Seq.index vec 0)) (v (Seq.index rhs 4)) (v (Seq.index vec 4) * (- v zeta0) * 169);
  lemma_modadd (v (Seq.index vec 1)) (v (Seq.index rhs 5)) (v (Seq.index vec 5) * (- v zeta0) * 169);
  lemma_modadd (v (Seq.index vec 2)) (v (Seq.index rhs 6)) (v (Seq.index vec 6) * (- v zeta0) * 169);
  lemma_modadd (v (Seq.index vec 3)) (v (Seq.index rhs 7)) (v (Seq.index vec 7) * (- v zeta0) * 169);
  lemma_modadd (v (Seq.index vec 8)) (v (Seq.index rhs 8)) (v (Seq.index vec 12) * v zeta1 * 169);
  lemma_modadd (v (Seq.index vec 9)) (v (Seq.index rhs 9)) (v (Seq.index vec 13) * v zeta1 * 169);
  lemma_modadd (v (Seq.index vec 10)) (v (Seq.index rhs 10)) (v (Seq.index vec 14) * v zeta1 * 169);
  lemma_modadd (v (Seq.index vec 11)) (v (Seq.index rhs 11)) (v (Seq.index vec 15) * v zeta1 * 169);
  lemma_modadd (v (Seq.index vec 8)) (v (Seq.index rhs 12)) (v (Seq.index vec 12) * (- v zeta1) * 169);
  lemma_modadd (v (Seq.index vec 9)) (v (Seq.index rhs 13)) (v (Seq.index vec 13) * (- v zeta1) * 169);
  lemma_modadd (v (Seq.index vec 10)) (v (Seq.index rhs 14)) (v (Seq.index vec 14) * (- v zeta1) * 169);
  lemma_modadd (v (Seq.index vec 11)) (v (Seq.index rhs 15)) (v (Seq.index vec 15) * (- v zeta1) * 169);
  reveal_opaque (`%FS.ntt_layer_2_butterfly_post)
    (FS.ntt_layer_2_butterfly_post vec)
#pop-options

(* ── Generic SIMD lane lemmas.  These bridge bit-level Vec256/Vec128 ops to
   their i16-array views.  The four mm256_* lemmas admit the abstract
   vec256_as_i16x16 / vec128_as_i16x8 definition (declared val in
   Avx2_extract.fsti); the two mm_* add/sub lemmas hold trivially. ── *)

let lemma_mm256_castsi256_si128 (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (forall (i: nat). i < 8 ==>
    Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8
                 (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 v)) i ==
    Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) i))
  = let aux (i: nat {i < 8})
      : Lemma (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8
                            (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 v)) i ==
               Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) i) =
      let a = Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8
                           (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 v)) i in
      let b = Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) i in
      let auxb (nth: usize {Rust_primitives.Integers.v nth < 16})
        : Lemma (get_bit a nth == get_bit b nth) =
        let nthv = Rust_primitives.Integers.v nth in
        FStar.Math.Lemmas.lemma_mult_le_left 16 i 7;
        let k : nat = 16 * i + nthv in
        assert (k < 128);
        FStar.Math.Lemmas.small_div nthv 16;
        FStar.Math.Lemmas.small_mod nthv 16;
        FStar.Math.Lemmas.lemma_div_plus nthv i 16;
        FStar.Math.Lemmas.lemma_mod_plus nthv i 16;
        Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec128_as_i16x8_lemma
          (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 v) 16 k;
        Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 16 k;
        assert (k / 16 == i);
        assert (k % 16 == nthv);
        (* Over core-models this step is the PROVEN companion bit fact. *)
        Libcrux_intrinsics.Avx2_ml_kem_views.lemma_bv_bit_castsi256_si128 v k
      in
      Classical.forall_intro auxb;
      Rust_primitives.Integers.lemma_int_t_eq_via_bits a b
    in
    Classical.forall_intro aux

let lemma_mm256_extracti128_si256_1 (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (forall (i: nat). i < 8 ==>
    Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8
                 (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) v)) i ==
    Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) (i + 8)))
  = let aux (i: nat {i < 8})
      : Lemma (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8
                            (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) v)) i ==
               Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) (i + 8)) =
      let a = Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8
                           (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) v)) i in
      let b = Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 v) (i + 8) in
      let auxb (nth: usize {Rust_primitives.Integers.v nth < 16})
        : Lemma (get_bit a nth == get_bit b nth) =
        let nthv = Rust_primitives.Integers.v nth in
        FStar.Math.Lemmas.lemma_mult_le_left 16 i 7;
        FStar.Math.Lemmas.lemma_mult_le_left 16 (i + 8) 15;
        let k : nat = 16 * i + nthv in
        let k' : nat = 16 * (i + 8) + nthv in
        assert (k < 128);
        assert (k' < 256);
        assert (k' == k + 128);
        FStar.Math.Lemmas.small_div nthv 16;
        FStar.Math.Lemmas.small_mod nthv 16;
        FStar.Math.Lemmas.lemma_div_plus nthv i 16;
        FStar.Math.Lemmas.lemma_mod_plus nthv i 16;
        FStar.Math.Lemmas.lemma_div_plus nthv (i + 8) 16;
        FStar.Math.Lemmas.lemma_mod_plus nthv (i + 8) 16;
        Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec128_as_i16x8_lemma
          (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) v) 16 k;
        Libcrux_intrinsics.Avx2_ml_kem_views.bit_vec_of_int_t_array_vec256_as_i16x16_lemma v 16 k';
        assert (k / 16 == i);
        assert (k % 16 == nthv);
        assert (k' / 16 == i + 8);
        assert (k' % 16 == nthv);
        (* Over core-models this step is the PROVEN companion bit fact. *)
        Libcrux_intrinsics.Avx2_ml_kem_views.lemma_bv_bit_extracti128_si256_1 v k
      in
      Classical.forall_intro auxb;
      Rust_primitives.Integers.lemma_int_t_eq_via_bits a b
    in
    Classical.forall_intro aux

let lemma_mm256_castsi128_si256_lo (v: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec128) : Lemma
  (ensures (forall (i: nat). i < 8 ==>
    Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16
                 (Libcrux_intrinsics.Avx2.mm256_castsi128_si256 v)) i ==
    Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 v) i))
  = let r = Libcrux_intrinsics.Avx2.mm256_castsi128_si256 v in
    let aux (i:nat{i<8}) : Lemma
      (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 r) i == Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 v) i) =
      assert (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r i == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 v i) in
    Classical.forall_intro aux

let lemma_mm256_inserti128_si256_1
    (a: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256)
    (b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec128) : Lemma
  (ensures
    (forall (i: nat). i < 8 ==>
      Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16
                   (Libcrux_intrinsics.Avx2.mm256_inserti128_si256 (mk_i32 1) a b)) i ==
      Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 a) i) /\
    (forall (i: nat). i < 8 ==>
      Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16
                   (Libcrux_intrinsics.Avx2.mm256_inserti128_si256 (mk_i32 1) a b)) (i + 8) ==
      Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 b) i))
  = let r = Libcrux_intrinsics.Avx2.mm256_inserti128_si256 (mk_i32 1) a b in
    let aux1 (i:nat{i<8}) : Lemma
      (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 r) i == Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 a) i) =
      assert (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r i == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a i) in
    Classical.forall_intro aux1;
    let aux2 (i:nat{i<8}) : Lemma
      (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 r) (i + 8) == Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec128_as_i16x8 b) i) =
      assert (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r (i + 8) == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 b ((i + 8) - 8)) in
    Classical.forall_intro aux2

let lemma_add_i_128
    (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec128) (i: nat) : Lemma
  (requires i < 8 /\ Spec.Utils.is_intb (pow2 15 - 1)
                       (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 lhs i) +
                        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 rhs i)))
  (ensures v (add_mod (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 lhs i)
                      (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 rhs i)) ==
           v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 lhs i) +
           v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 rhs i))
  [SMTPat (v (add_mod (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 lhs i)
                      (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 rhs i)))]
  = ()

let lemma_sub_i_128
    (lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec128) (i: nat) : Lemma
  (requires i < 8 /\ Spec.Utils.is_intb (pow2 15 - 1)
                       (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 lhs i) -
                        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 rhs i)))
  (ensures v (sub_mod (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 lhs i)
                      (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 rhs i)) ==
           v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 lhs i) -
           v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 rhs i))
  [SMTPat (v (sub_mod (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 lhs i)
                      (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 rhs i)))]
  = ()


(* ── Inverse-NTT layer-1 lemma library.  The leaf uses within-128-bit-lane
   mm256_shuffle_epi32 + mm256_mullo_epi16 + mm256_blend_epi16, none of
   which have an F* functional model — lemma_shuffle_{245,160} /
   lemma_shuffle_preserves_bound / lemma_blend_204 axiomatize their per-lane
   behaviour (analogous to the layer-3 mm256_* lemmas above).  The map2/createi
   cost of the mullo+add chain is isolated to lemma_inv_l1_sums (paid ONCE,
   uniform forall); lemma_inv_l1_sums_v substitutes the shuffle/mult facts to
   express the sum lanes in terms of vector; lemma_inv_l1_post derives the
   trait inv_ntt_layer_1_butterfly_post over plain i16 arrays (no mm256
   terms, hence cascade-free). ── *)

module ZI = Libcrux_intrinsics.Avx2
module ZS = Spec.Utils
module ZA = Libcrux_ml_kem.Vector.Avx2.Arithmetic

let lemma_shuffle_245 (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) vv in
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 11 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 11 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 15 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 15))
  = let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 245) vv in
    shuf245 0; shuf245 1; shuf245 2; shuf245 3; shuf245 4; shuf245 5; shuf245 6; shuf245 7

let lemma_shuffle_160 (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 160) vv in
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 13 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 13))
  = let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 (mk_i32 160) vv in
    shuf160 0; shuf160 1; shuf160 2; shuf160 3; shuf160 4; shuf160 5; shuf160 6; shuf160 7

(* `c` is symbolic here, so the shuffle/permute fact's immediate-range side
   condition has to be taken as a hypothesis; every call site passes a literal. *)
let lemma_shuffle_preserves_bound (c: i32) (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (b: nat) : Lemma
  (requires v c >= 0 /\ v c < 256 /\ ZS.is_i16b_array b (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vv))
  (ensures ZS.is_i16b_array b (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 (Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 c vv)))
  = let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi32 c vv in
    let aux (i:nat{i<16}) : Lemma (ZS.is_i16b b (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 r) i)) =
      assert (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r i == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv (2 * Libcrux_intrinsics.Avx2_ml_kem_views.shuffle32_src c (i/2) + i%2)) in
    Classical.forall_intro aux

let lemma_blend_204 (a b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (let r = Libcrux_intrinsics.Avx2.mm256_blend_epi16 (mk_i32 204) a b in
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 11 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 13 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 15))
  = let r = Libcrux_intrinsics.Avx2.mm256_blend_epi16 (mk_i32 204) a b in
    blendsel204 0; blendsel204 1; blendsel204 2; blendsel204 3;
    blendsel204 4; blendsel204 5; blendsel204 6; blendsel204 7;
    blendsel204 8; blendsel204 9; blendsel204 10; blendsel204 11;
    blendsel204 12; blendsel204 13; blendsel204 14; blendsel204 15

#push-options "--z3rlimit 400 --split_queries always"
let lemma_inv_l1_sums (lhs rhs0 mult rhs sum: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
     rhs == Libcrux_intrinsics.Avx2.mm256_mullo_epi16 rhs0 mult /\
     sum == Libcrux_intrinsics.Avx2.mm256_add_epi16 lhs rhs /\
     ZS.is_i16b_array (4*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 lhs) /\
     ZS.is_i16b_array (4*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 rhs0) /\
     (forall (i:nat). i < 16 ==> (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult i) == 1 \/ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult i) == -1)))
  (ensures
     ZS.is_i16b_array 28296 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 sum) /\
     (forall (i:nat). i < 16 ==>
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum i) ==
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs i) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 i) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult i)))
  = ()
#pop-options

#push-options "--z3rlimit 300 --split_queries always"
let lemma_inv_l1_sums_v (vector lhs rhs0 mult rhs sum: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
     rhs == Libcrux_intrinsics.Avx2.mm256_mullo_epi16 rhs0 mult /\
     sum == Libcrux_intrinsics.Avx2.mm256_add_epi16 lhs rhs /\
     ZS.is_i16b_array (4*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 lhs) /\
     ZS.is_i16b_array (4*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 rhs0) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 0) == 1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 1) == 1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 2) == -1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 3) == -1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 4) == 1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 5) == 1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 6) == -1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 7) == -1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 8) == 1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 9) == 1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 10) == -1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 11) == -1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 12) == 1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 13) == 1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 14) == -1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 15) == -1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 11 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 11 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 15 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 15 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 13 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 13)
  (ensures
     ZS.is_i16b_array 28296 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 sum) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 0)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 2)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 1)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 3)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 2)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 2)  - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 3)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 3)  - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 4)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 6)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 4) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 5)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 7)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 5) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 6)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 6)  - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 4) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 7)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 7)  - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 5) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 8)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 10) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 9)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 11) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 10) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 10) - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 11) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 11) - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 12) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 14) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 12) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 13) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 15) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 13) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 14) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 14) - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 12) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 15) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 15) - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 13))
  = lemma_inv_l1_sums lhs rhs0 mult rhs sum
#pop-options

#push-options "--z3rlimit 200 --split_queries always"
let lemma_inv_l1_post
    (vec sum sbar stz zetas res: t_Array i16 (mk_usize 16))
    (zeta0 zeta1 zeta2 zeta3: i16)
  : Lemma
    (requires
      v (Seq.index sum 0)  == v (Seq.index vec 2)  + v (Seq.index vec 0) /\
      v (Seq.index sum 1)  == v (Seq.index vec 3)  + v (Seq.index vec 1) /\
      v (Seq.index sum 2)  == v (Seq.index vec 2)  - v (Seq.index vec 0) /\
      v (Seq.index sum 3)  == v (Seq.index vec 3)  - v (Seq.index vec 1) /\
      v (Seq.index sum 4)  == v (Seq.index vec 6)  + v (Seq.index vec 4) /\
      v (Seq.index sum 5)  == v (Seq.index vec 7)  + v (Seq.index vec 5) /\
      v (Seq.index sum 6)  == v (Seq.index vec 6)  - v (Seq.index vec 4) /\
      v (Seq.index sum 7)  == v (Seq.index vec 7)  - v (Seq.index vec 5) /\
      v (Seq.index sum 8)  == v (Seq.index vec 10) + v (Seq.index vec 8) /\
      v (Seq.index sum 9)  == v (Seq.index vec 11) + v (Seq.index vec 9) /\
      v (Seq.index sum 10) == v (Seq.index vec 10) - v (Seq.index vec 8) /\
      v (Seq.index sum 11) == v (Seq.index vec 11) - v (Seq.index vec 9) /\
      v (Seq.index sum 12) == v (Seq.index vec 14) + v (Seq.index vec 12) /\
      v (Seq.index sum 13) == v (Seq.index vec 15) + v (Seq.index vec 13) /\
      v (Seq.index sum 14) == v (Seq.index vec 14) - v (Seq.index vec 12) /\
      v (Seq.index sum 15) == v (Seq.index vec 15) - v (Seq.index vec 13) /\
      (forall (i:nat). i < 16 ==> v (Seq.index sbar i) % 3329 == v (Seq.index sum i) % 3329) /\
      (forall (i:nat). i < 16 ==>
         v (Seq.index stz i) % 3329 == (v (Seq.index sum i) * v (Seq.index zetas i) * 169) % 3329) /\
      v (Seq.index zetas 2) == v zeta0 /\ v (Seq.index zetas 3) == v zeta0 /\
      v (Seq.index zetas 6) == v zeta1 /\ v (Seq.index zetas 7) == v zeta1 /\
      v (Seq.index zetas 10) == v zeta2 /\ v (Seq.index zetas 11) == v zeta2 /\
      v (Seq.index zetas 14) == v zeta3 /\ v (Seq.index zetas 15) == v zeta3 /\
      Seq.index res 0 == Seq.index sbar 0 /\ Seq.index res 1 == Seq.index sbar 1 /\
      Seq.index res 2 == Seq.index stz 2 /\ Seq.index res 3 == Seq.index stz 3 /\
      Seq.index res 4 == Seq.index sbar 4 /\ Seq.index res 5 == Seq.index sbar 5 /\
      Seq.index res 6 == Seq.index stz 6 /\ Seq.index res 7 == Seq.index stz 7 /\
      Seq.index res 8 == Seq.index sbar 8 /\ Seq.index res 9 == Seq.index sbar 9 /\
      Seq.index res 10 == Seq.index stz 10 /\ Seq.index res 11 == Seq.index stz 11 /\
      Seq.index res 12 == Seq.index sbar 12 /\ Seq.index res 13 == Seq.index sbar 13 /\
      Seq.index res 14 == Seq.index stz 14 /\ Seq.index res 15 == Seq.index stz 15 /\
      ZS.is_i16b_array 3328 sbar /\ ZS.is_i16b_array 3328 stz)
    (ensures
      ZS.is_i16b_array 3328 res /\
      ZS.inv_ntt_layer_1_butterfly_post vec res zeta0 zeta1 zeta2 zeta3)
  =
  reveal_opaque (`%ZS.inv_ntt_layer_1_butterfly_post)
    (ZS.inv_ntt_layer_1_butterfly_post vec)
#pop-options

(* ── Inverse-NTT layer-2 lemma library — same architecture as layer-1, with
   cross-128-bit-lane mm256_permute4x64_epi64 (modelless: axiomatized per
   control by lemma_permute_{245,160}) and NO Barrett (the "add" outputs are
   the raw sum, bounded 2*3328).  lemma_inv_l2_sums pays map2/createi once;
   lemma_inv_l2_post derives inv_ntt_layer_2_butterfly_post over plain i16
   arrays. ── *)

let lemma_permute_245 (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (let r = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 245) vv in
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 13 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 15 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 13 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 15))
  = let r = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 245) vv in
    perm245 0; perm245 1; perm245 2; perm245 3

let lemma_permute_160 (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (let r = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 160) vv in
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 11 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv 11))
  = let r = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 160) vv in
    perm160 0; perm160 1; perm160 2; perm160 3

(* `c` is symbolic here, so the shuffle/permute fact's immediate-range side
   condition has to be taken as a hypothesis; every call site passes a literal. *)
let lemma_permute_preserves_bound (c: i32) (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (b: nat) : Lemma
  (requires v c >= 0 /\ v c < 256 /\ ZS.is_i16b_array b (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 vv))
  (ensures ZS.is_i16b_array b (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 (Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 c vv)))
  = let r = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 c vv in
    let aux (i:nat{i<16}) : Lemma (ZS.is_i16b b (Seq.index (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 r) i)) =
      assert (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r i == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv (4 * Libcrux_intrinsics.Avx2_ml_kem_views.permute64_src c (i/4) + i%4)) in
    Classical.forall_intro aux

let lemma_blend_240 (a b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (ensures (let r = Libcrux_intrinsics.Avx2.mm256_blend_epi16 (mk_i32 240) a b in
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a 11 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 13 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane r 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b 15))
  = let r = Libcrux_intrinsics.Avx2.mm256_blend_epi16 (mk_i32 240) a b in
    blendsel240 0; blendsel240 1; blendsel240 2; blendsel240 3;
    blendsel240 4; blendsel240 5; blendsel240 6; blendsel240 7;
    blendsel240 8; blendsel240 9; blendsel240 10; blendsel240 11;
    blendsel240 12; blendsel240 13; blendsel240 14; blendsel240 15

// Createi-free: a per-lane forall-aux calls lemma_get_lane_mullo /
// lemma_get_lane_add (above) explicitly, paying the map2/createi reasoning once
// inside those two bodies (no SMTPat — see the note at their definition); with
// the per-lane bounds in hand, Z3's native i16 model lifts `mul_mod`/`+.` to
// integer `*`/`+` (same as lemma_inv_l1_sums). Classical.forall_intro assembles
// the lane forall; the array bound follows per-lane.
#push-options "--z3rlimit 400 --split_queries always"
let lemma_inv_l2_sums (lhs rhs0 mult rhs sum: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
     rhs == Libcrux_intrinsics.Avx2.mm256_mullo_epi16 rhs0 mult /\
     sum == Libcrux_intrinsics.Avx2.mm256_add_epi16 lhs rhs /\
     ZS.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 lhs) /\
     ZS.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 rhs0) /\
     (forall (i:nat). i < 16 ==> (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult i) == 1 \/ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult i) == -1)))
  (ensures
     ZS.is_i16b_array (2*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 sum) /\
     (forall (i:nat). i < 16 ==>
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum i) ==
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs i) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 i) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult i)))
  = let aux (i:nat{i < 16}) : Lemma
        (ensures
           ZS.is_intb (2*3328) (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum i)) /\
           v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum i) ==
             v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs i) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 i) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult i)) =
      // per-lane mullo/add facts (createi confined to these two bodies, no SMTPat):
      lemma_get_lane_mullo rhs0 mult i;   // get_lane rhs i == mul_mod (get_lane rhs0 i) (get_lane mult i)
      lemma_get_lane_add lhs rhs i;       // get_lane sum i == get_lane lhs i +. get_lane rhs i
      // per-lane bounds: get_lane _ i == Seq.index (vec256_as_i16x16 _) i (definitional),
      // so the requires' is_i16b_array foralls instantiate here; with these bounds Z3's
      // native i16 model lifts mul_mod / +. to integer * / + (same as lemma_inv_l1_sums).
      assert (ZS.is_intb 3328 (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs i)));
      assert (ZS.is_intb 3328 (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 i)));
      // re-assert the mult requires-forall: under --split_queries always this
      // disjunction-bodied forall is otherwise pruned from this sub-query's
      // hypotheses, so the instantiation at `i` would not fire.
      assert (forall (j:nat). j < 16 ==> (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult j) == 1 \/ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult j) == -1));
      assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult i) == 1 \/ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult i) == -1)
    in
    Classical.forall_intro aux
#pop-options

#push-options "--z3rlimit 300 --split_queries always"
let lemma_inv_l2_sums_v (vector lhs rhs0 mult rhs sum: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
     rhs == Libcrux_intrinsics.Avx2.mm256_mullo_epi16 rhs0 mult /\
     sum == Libcrux_intrinsics.Avx2.mm256_add_epi16 lhs rhs /\
     ZS.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 lhs) /\
     ZS.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 rhs0) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 0) == 1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 1) == 1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 2) == 1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 3) == 1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 4) == -1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 5) == -1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 6) == -1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 7) == -1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 8) == 1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 9) == 1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 10) == 1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 11) == 1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 12) == -1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 13) == -1 /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 14) == -1 /\ v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult 15) == -1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 4 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 6 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 13 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 15 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 12 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 13 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 14 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 15 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 2 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 11 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 10 /\ Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 11)
  (ensures
     ZS.is_i16b_array (2*3328) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 sum) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 0)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 4)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 1)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 5)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 2)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 6)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 2) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 3)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 7)  + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 3) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 4)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 4)  - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 0) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 5)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 5)  - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 1) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 6)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 6)  - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 2) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 7)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 7)  - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 3) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 8)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 12) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 9)  == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 13) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 10) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 14) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 10) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 11) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 15) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 11) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 12) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 12) - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 8) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 13) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 13) - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 9) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 14) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 14) - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 10) /\
     v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum 15) == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 15) - v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vector 11))
  = lemma_inv_l2_sums lhs rhs0 mult rhs sum;
    // re-assert l2_sums' post forall to pull it into each per-lane sub-query
    // under --split_queries always (else the lane instantiations do not fire).
    assert (forall (i:nat). i < 16 ==>
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sum i) ==
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs i) + v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs0 i) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane mult i))
#pop-options

#push-options "--z3rlimit 200 --split_queries always"
let lemma_inv_l2_post
    (vec sum stz zetas res: t_Array i16 (mk_usize 16))
    (zeta0 zeta1: i16)
  : Lemma
    (requires
      v (Seq.index sum 0)  == v (Seq.index vec 4)  + v (Seq.index vec 0) /\
      v (Seq.index sum 1)  == v (Seq.index vec 5)  + v (Seq.index vec 1) /\
      v (Seq.index sum 2)  == v (Seq.index vec 6)  + v (Seq.index vec 2) /\
      v (Seq.index sum 3)  == v (Seq.index vec 7)  + v (Seq.index vec 3) /\
      v (Seq.index sum 4)  == v (Seq.index vec 4)  - v (Seq.index vec 0) /\
      v (Seq.index sum 5)  == v (Seq.index vec 5)  - v (Seq.index vec 1) /\
      v (Seq.index sum 6)  == v (Seq.index vec 6)  - v (Seq.index vec 2) /\
      v (Seq.index sum 7)  == v (Seq.index vec 7)  - v (Seq.index vec 3) /\
      v (Seq.index sum 8)  == v (Seq.index vec 12) + v (Seq.index vec 8) /\
      v (Seq.index sum 9)  == v (Seq.index vec 13) + v (Seq.index vec 9) /\
      v (Seq.index sum 10) == v (Seq.index vec 14) + v (Seq.index vec 10) /\
      v (Seq.index sum 11) == v (Seq.index vec 15) + v (Seq.index vec 11) /\
      v (Seq.index sum 12) == v (Seq.index vec 12) - v (Seq.index vec 8) /\
      v (Seq.index sum 13) == v (Seq.index vec 13) - v (Seq.index vec 9) /\
      v (Seq.index sum 14) == v (Seq.index vec 14) - v (Seq.index vec 10) /\
      v (Seq.index sum 15) == v (Seq.index vec 15) - v (Seq.index vec 11) /\
      (forall (i:nat). i < 16 ==>
         v (Seq.index stz i) % 3329 == (v (Seq.index sum i) * v (Seq.index zetas i) * 169) % 3329) /\
      v (Seq.index zetas 4) == v zeta0 /\ v (Seq.index zetas 5) == v zeta0 /\
      v (Seq.index zetas 6) == v zeta0 /\ v (Seq.index zetas 7) == v zeta0 /\
      v (Seq.index zetas 12) == v zeta1 /\ v (Seq.index zetas 13) == v zeta1 /\
      v (Seq.index zetas 14) == v zeta1 /\ v (Seq.index zetas 15) == v zeta1 /\
      Seq.index res 0 == Seq.index sum 0 /\ Seq.index res 1 == Seq.index sum 1 /\
      Seq.index res 2 == Seq.index sum 2 /\ Seq.index res 3 == Seq.index sum 3 /\
      Seq.index res 4 == Seq.index stz 4 /\ Seq.index res 5 == Seq.index stz 5 /\
      Seq.index res 6 == Seq.index stz 6 /\ Seq.index res 7 == Seq.index stz 7 /\
      Seq.index res 8 == Seq.index sum 8 /\ Seq.index res 9 == Seq.index sum 9 /\
      Seq.index res 10 == Seq.index sum 10 /\ Seq.index res 11 == Seq.index sum 11 /\
      Seq.index res 12 == Seq.index stz 12 /\ Seq.index res 13 == Seq.index stz 13 /\
      Seq.index res 14 == Seq.index stz 14 /\ Seq.index res 15 == Seq.index stz 15 /\
      ZS.is_i16b_array (2*3328) sum /\ ZS.is_i16b_array 3328 stz)
    (ensures
      ZS.is_i16b_array (2*3328) res /\
      ZS.inv_ntt_layer_2_butterfly_post vec res zeta0 zeta1)
  =
  reveal_opaque (`%ZS.inv_ntt_layer_2_butterfly_post)
    (ZS.inv_ntt_layer_2_butterfly_post vec)
#pop-options

#push-options "--z3rlimit 400 --split_queries always"

// ─────────────────────────────────────────────────────────────────────────
// ntt_multiply lemma library — ground-literal architecture, v12.
//
// Per-lane admitted axioms (quantifier-free); helpers and one main Lemma
// (plain-implication encoding — large Pure bodies bury hypotheses behind a
// unit-refinement chain Z3 cannot thread).  The grouping shuffle mask is
// threaded everywhere as a FREE parameter `m` with `m == <inline mask>`
// as a requires: BitVec closure terms admit no first-order congruence, so
// the mask equality must only ever be PROPAGATED as a hypothesis, never
// re-derived; the function instantiates `m := shuffle_with`, putting every
// conclusion directly on its own spine terms.  Cross-validated against a
// bit-exact simulation (2000 random trials).
// ─────────────────────────────────────────────────────────────────────────

(* ── shuffle_epi8 (256-bit PSHUFB) ─────────────────────────────────────────────
   core-models MODELS the op (`Int_vec.e_mm256_shuffle_epi8`) and the model is
   differentially tested against the real intrinsic, so the per-lane bridge below
   is PROVEN
   from `Canon.lemma_mm256_shuffle_epi8` (the tested lift + the proven codec
   round-trip), `Canon.lemma_iv_shuffle_epi8_sel` (the model's select branch) and
   `Canon.lemma_i16_from_i8_pair` (two agreeing i8 sub-lanes give an agreeing i16
   lane).  Net: one untested axiom removed from the TCB. *)

(* The unsigned value of mask byte `n` — the PSHUFB index byte. *)
let mbv (g: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (n: nat{n < 32}) : nat =
  (Rust_primitives.Integers.v
     (Libcrux_core_models.Abstractions.Funarr.impl_5__get (mk_u64 32) #i8
        (Libcrux_core_models.Intrinsics_views.to_i8x32 g) (mk_u64 n))) % 256

(* Generic per-lane bridge: if mask bytes 2k/2k+1 select (within k's half) the
   two bytes of input i16-lane `sg`, then output i16-lane k == input lane sg.
   No `match k` (symbolic k, minimal context) — the dispatchers below pin k. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300"
let lemma_shuffle8_lane (vv m: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (k: nat{k<16}) (sg: nat{sg<16}) : Lemma
  (requires
     (let h = k/8 in
      mbv m (2*k) <= 127 /\ mbv m (2*k+1) <= 127 /\
      mbv m (2*k) % 16 == 2*(sg - h*8) /\ mbv m (2*k+1) % 16 == 2*(sg - h*8) + 1 /\
      sg / 8 == h))
  (ensures Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 vv m) k
           == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv sg)
  = reveal_opaque (`%Libcrux_intrinsics.Avx2.mm256_shuffle_epi8)
                  Libcrux_intrinsics.Avx2.mm256_shuffle_epi8;
    Libcrux_core_models.Intrinsics_views.lemma_mm256_shuffle_epi8 vv m;
    let r = Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 vv m in
    let h = k/8 in
    let mvv = Libcrux_core_models.Intrinsics_views.to_i8x32 m in
    (* `mbv <= 127` on an i8 lane is exactly "the index byte is non-negative",
       i.e. PSHUFB takes the select branch rather than zeroing. *)
    assert (Rust_primitives.Integers.v
              (Libcrux_core_models.Abstractions.Funarr.impl_5__get (mk_u64 32) #i8 mvv (mk_u64 (2*k))) >= 0);
    assert (Rust_primitives.Integers.v
              (Libcrux_core_models.Abstractions.Funarr.impl_5__get (mk_u64 32) #i8 mvv (mk_u64 (2*k+1))) >= 0);
    Libcrux_core_models.Intrinsics_views.lemma_iv_shuffle_epi8_sel
      (Libcrux_core_models.Intrinsics_views.to_i8x32 vv) mvv (2*k);
    Libcrux_core_models.Intrinsics_views.lemma_iv_shuffle_epi8_sel
      (Libcrux_core_models.Intrinsics_views.to_i8x32 vv) mvv (2*k+1);
    assert ((2*k) / 16 == h);
    assert ((2*k+1) / 16 == h);
    assert (16 * h + (mbv m (2*k)) % 16 == 2 * sg);
    assert (16 * h + (mbv m (2*k+1)) % 16 == 2 * sg + 1);
    Libcrux_core_models.Intrinsics_views.lemma_i16_from_i8_pair r vv k sg
#pop-options

let group_mask8 : Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256 =
  Libcrux_intrinsics.Avx2.mm256_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
  (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
  (mk_i8 1) (mk_i8 0) (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
  (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
  (mk_i8 1) (mk_i8 0)

let swap_mask8 : Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256 =
  Libcrux_intrinsics.Avx2.mm256_set_epi8 (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
  (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
  (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
  (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
  (mk_i8 3) (mk_i8 2)

(* Byte values of the two concrete PSHUFB masks, from the canonical `set_epi8`
   op-lemma + its per-lane interpretation lemma.  (Over core-models the lane view
   is a codec, so the values come from lemmas rather than normalisation.) *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let lemma_group_mask8_byte (n: nat{n < 32})
  : Lemma (mbv group_mask8 n ==
           (match n % 16 with
             | 0 -> 0
             | 1 -> 1
             | 2 -> 4
             | 3 -> 5
             | 4 -> 8
             | 5 -> 9
             | 6 -> 12
             | 7 -> 13
             | 8 -> 2
             | 9 -> 3
             | 10 -> 6
             | 11 -> 7
             | 12 -> 10
             | 13 -> 11
             | 14 -> 14
             | _ -> 15))
  = reveal_opaque (`%Libcrux_intrinsics.Avx2.mm256_set_epi8) Libcrux_intrinsics.Avx2.mm256_set_epi8;
    Libcrux_core_models.Intrinsics_views.lemma_mm256_set_epi8
      (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6) (mk_i8 3) (mk_i8 2)
      (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4) (mk_i8 1) (mk_i8 0)
      (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6) (mk_i8 3) (mk_i8 2)
      (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4) (mk_i8 1) (mk_i8 0);
    Libcrux_core_models.Intrinsics_views.lemma_iv_set_epi8
      (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6) (mk_i8 3) (mk_i8 2)
      (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4) (mk_i8 1) (mk_i8 0)
      (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6) (mk_i8 3) (mk_i8 2)
      (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4) (mk_i8 1) (mk_i8 0) n
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 400"
let lemma_swap_mask8_byte (n: nat{n < 32})
  : Lemma (mbv swap_mask8 n ==
           (match n % 16 with
             | 0 -> 2
             | 1 -> 3
             | 2 -> 0
             | 3 -> 1
             | 4 -> 6
             | 5 -> 7
             | 6 -> 4
             | 7 -> 5
             | 8 -> 10
             | 9 -> 11
             | 10 -> 8
             | 11 -> 9
             | 12 -> 14
             | 13 -> 15
             | 14 -> 12
             | _ -> 13))
  = reveal_opaque (`%Libcrux_intrinsics.Avx2.mm256_set_epi8) Libcrux_intrinsics.Avx2.mm256_set_epi8;
    Libcrux_core_models.Intrinsics_views.lemma_mm256_set_epi8
      (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8) (mk_i8 11) (mk_i8 10)
      (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0) (mk_i8 3) (mk_i8 2)
      (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8) (mk_i8 11) (mk_i8 10)
      (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0) (mk_i8 3) (mk_i8 2);
    Libcrux_core_models.Intrinsics_views.lemma_iv_set_epi8
      (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8) (mk_i8 11) (mk_i8 10)
      (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0) (mk_i8 3) (mk_i8 2)
      (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8) (mk_i8 11) (mk_i8 10)
      (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0) (mk_i8 3) (mk_i8 2) n
#pop-options

(* Evens/odds grouping shuffle at the concrete mask (passed as `m`):
   out half-lane k takes input half-lane sigma(k), sigma = [0,2,4,6,1,3,5,7]. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_nttmul_shuffle_group_lane (vv m: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (k: nat{k < 16}) : Lemma
  (requires m ==
      (Libcrux_intrinsics.Avx2.mm256_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0) (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0)))
  (ensures Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 vv m) k ==
   Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv (if k < 4 then 2*k
                   else if k < 8 then 2*(k-4)+1
                   else if k < 12 then 2*(k-8)+8
                   else 2*(k-12)+9))
  = assert (m == group_mask8);
    (match k with
     | 0  -> lemma_group_mask8_byte 0;  lemma_group_mask8_byte 1;  lemma_shuffle8_lane vv m 0 0
     | 1  -> lemma_group_mask8_byte 2;  lemma_group_mask8_byte 3;  lemma_shuffle8_lane vv m 1 2
     | 2  -> lemma_group_mask8_byte 4;  lemma_group_mask8_byte 5;  lemma_shuffle8_lane vv m 2 4
     | 3  -> lemma_group_mask8_byte 6; lemma_group_mask8_byte 7; lemma_shuffle8_lane vv m 3 6
     | 4  -> lemma_group_mask8_byte 8;  lemma_group_mask8_byte 9;  lemma_shuffle8_lane vv m 4 1
     | 5  -> lemma_group_mask8_byte 10; lemma_group_mask8_byte 11; lemma_shuffle8_lane vv m 5 3
     | 6  -> lemma_group_mask8_byte 12;lemma_group_mask8_byte 13;lemma_shuffle8_lane vv m 6 5
     | 7  -> lemma_group_mask8_byte 14;lemma_group_mask8_byte 15;lemma_shuffle8_lane vv m 7 7
     | 8  -> lemma_group_mask8_byte 16; lemma_group_mask8_byte 17; lemma_shuffle8_lane vv m 8 8
     | 9  -> lemma_group_mask8_byte 18; lemma_group_mask8_byte 19; lemma_shuffle8_lane vv m 9 10
     | 10 -> lemma_group_mask8_byte 20; lemma_group_mask8_byte 21; lemma_shuffle8_lane vv m 10 12
     | 11 -> lemma_group_mask8_byte 22;lemma_group_mask8_byte 23;lemma_shuffle8_lane vv m 11 14
     | 12 -> lemma_group_mask8_byte 24; lemma_group_mask8_byte 25; lemma_shuffle8_lane vv m 12 9
     | 13 -> lemma_group_mask8_byte 26; lemma_group_mask8_byte 27; lemma_shuffle8_lane vv m 13 11
     | 14 -> lemma_group_mask8_byte 28;lemma_group_mask8_byte 29;lemma_shuffle8_lane vv m 14 13
     | 15 -> lemma_group_mask8_byte 30;lemma_group_mask8_byte 31;lemma_shuffle8_lane vv m 15 15
     | _ -> ())
#pop-options

(* Adjacent-pair swap (mask passed as `m`): out lane k = in lane (k xor 1). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 150 --split_queries always"
let lemma_nttmul_swap_lane (vv m: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (k: nat{k < 16}) : Lemma
  (requires m ==
      (Libcrux_intrinsics.Avx2.mm256_set_epi8 (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2)))
  (ensures Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 vv m) k ==
   Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv (if k % 2 = 0 then k+1 else k-1))
  = assert (m == swap_mask8);
    (match k with
     | 0  -> lemma_swap_mask8_byte 0;  lemma_swap_mask8_byte 1;  lemma_shuffle8_lane vv m 0 1
     | 1  -> lemma_swap_mask8_byte 2;  lemma_swap_mask8_byte 3;  lemma_shuffle8_lane vv m 1 0
     | 2  -> lemma_swap_mask8_byte 4;  lemma_swap_mask8_byte 5;  lemma_shuffle8_lane vv m 2 3
     | 3  -> lemma_swap_mask8_byte 6;  lemma_swap_mask8_byte 7;  lemma_shuffle8_lane vv m 3 2
     | 4  -> lemma_swap_mask8_byte 8; lemma_swap_mask8_byte 9; lemma_shuffle8_lane vv m 4 5
     | 5  -> lemma_swap_mask8_byte 10; lemma_swap_mask8_byte 11; lemma_shuffle8_lane vv m 5 4
     | 6  -> lemma_swap_mask8_byte 12;lemma_swap_mask8_byte 13;lemma_shuffle8_lane vv m 6 7
     | 7  -> lemma_swap_mask8_byte 14;lemma_swap_mask8_byte 15;lemma_shuffle8_lane vv m 7 6
     | 8  -> lemma_swap_mask8_byte 16; lemma_swap_mask8_byte 17; lemma_shuffle8_lane vv m 8 9
     | 9  -> lemma_swap_mask8_byte 18; lemma_swap_mask8_byte 19; lemma_shuffle8_lane vv m 9 8
     | 10 -> lemma_swap_mask8_byte 20; lemma_swap_mask8_byte 21; lemma_shuffle8_lane vv m 10 11
     | 11 -> lemma_swap_mask8_byte 22; lemma_swap_mask8_byte 23; lemma_shuffle8_lane vv m 11 10
     | 12 -> lemma_swap_mask8_byte 24;lemma_swap_mask8_byte 25;lemma_shuffle8_lane vv m 12 13
     | 13 -> lemma_swap_mask8_byte 26; lemma_swap_mask8_byte 27; lemma_shuffle8_lane vv m 13 12
     | 14 -> lemma_swap_mask8_byte 28;lemma_swap_mask8_byte 29;lemma_shuffle8_lane vv m 14 15
     | 15 -> lemma_swap_mask8_byte 30;lemma_swap_mask8_byte 31;lemma_shuffle8_lane vv m 15 14
     | _ -> ())
#pop-options

(* 64-bit qword permute, control 0xD8 = [q0, q2, q1, q3]. *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 100"
let lemma_nttmul_permute_d8_lane (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (k: nat{k < 16}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216) vv) k ==
   Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv (if k < 4 then k else if k < 8 then k+4 else if k < 12 then k-4 else k))
  = let r = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216) vv in
    perm216 0; perm216 1; perm216 2; perm216 3;
    (match k/4 with |0->()|1->()|2->()|3->()|_->())
#pop-options

let lemma_nttmul_cast_lane (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 vv) j == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv j)
  = lemma_mm256_castsi256_si128 vv

let lemma_nttmul_extract1_lane (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) vv) j == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv (j + 8))
  = lemma_mm256_extracti128_si256_1 vv

#push-options "--fuel 1 --ifuel 1 --z3rlimit 60"
let lemma_nttmul_cvt_lane (x: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec128) (j: nat{j < 8}) : Lemma
  (ZA.lane32 (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 x) j == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 x j))
  = let cx = Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 x in
    let lo = Libcrux_intrinsics.Avx2_ml_kem_views.get_lane128 x j in
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane cx (2*j) == lo /\
            Libcrux_intrinsics.Avx2_ml_kem_views.get_lane cx (2*j+1) == (if v lo < 0 then mk_i16 (-1) else mk_i16 0));
    (if v lo >= 0
     then FStar.Math.Lemmas.small_mod (v lo) 65536
     else (FStar.Math.Lemmas.lemma_mod_plus (v lo) 1 65536;
           FStar.Math.Lemmas.small_mod (v lo + 65536) 65536));
    assert (ZA.lane32 cx j ==
            (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane cx (2*j)) % 65536) + 65536 * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane cx (2*j+1)))
        by (FStar.Tactics.norm [delta_only [`%ZA.lane32]]; FStar.Tactics.trefl ())
#pop-options

#push-options "--fuel 1 --ifuel 1 --z3rlimit 60"
let lemma_at_percent_id_32 (k:int) : Lemma (requires -2147483648 <= k /\ k < 2147483648)
                                          (ensures (k @% 4294967296) == k) =
  if k >= 0 then FStar.Math.Lemmas.small_mod k 4294967296
  else (FStar.Math.Lemmas.lemma_mod_plus k 1 4294967296;
        FStar.Math.Lemmas.small_mod (k + 4294967296) 4294967296)

(* The crate's lane32 (Libcrux_intrinsics.Avx2_extract) and the ml-kem-side
   lane32 (ZA) have identical bodies; bridge by unfolding both. *)
let lemma_lane32_eq (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (j: nat{j < 8}) : Lemma (ZA.lane32 vv j == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 vv j)
  = assert (ZA.lane32 vv j == Libcrux_intrinsics.Avx2_ml_kem_views.lane32 vv j)
      by (FStar.Tactics.norm [delta_only [`%ZA.lane32; `%Libcrux_intrinsics.Avx2_ml_kem_views.lane32]]; FStar.Tactics.trefl ())

let lemma_nttmul_mullo32_lane (a b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (bnd_a bnd_b: nat) (j: nat{j < 8}) : Lemma
  (requires bnd_a * bnd_b < pow2 31 /\
            Spec.Utils.is_intb bnd_a (ZA.lane32 a j) /\
            Spec.Utils.is_intb bnd_b (ZA.lane32 b j))
  (ensures ZA.lane32 (Libcrux_intrinsics.Avx2.mm256_mullo_epi32 a b) j == ZA.lane32 a j * ZA.lane32 b j /\
           Spec.Utils.is_intb (bnd_a * bnd_b) (ZA.lane32 (Libcrux_intrinsics.Avx2.mm256_mullo_epi32 a b) j))
  = let r = Libcrux_intrinsics.Avx2.mm256_mullo_epi32 a b in
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 r j == (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a j * Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b j) @% 4294967296);
    lemma_lane32_eq r j;
    lemma_lane32_eq a j;
    lemma_lane32_eq b j;
    Spec.Utils.lemma_mul_intb bnd_a bnd_b (ZA.lane32 a j) (ZA.lane32 b j);
    assert_norm (pow2 31 == 2147483648);
    lemma_at_percent_id_32 (ZA.lane32 a j * ZA.lane32 b j)

let lemma_nttmul_add32_lane (a b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (bnd_a bnd_b: nat) (j: nat{j < 8}) : Lemma
  (requires bnd_a + bnd_b < pow2 31 /\
            Spec.Utils.is_intb bnd_a (ZA.lane32 a j) /\
            Spec.Utils.is_intb bnd_b (ZA.lane32 b j))
  (ensures ZA.lane32 (Libcrux_intrinsics.Avx2.mm256_add_epi32 a b) j == ZA.lane32 a j + ZA.lane32 b j /\
           Spec.Utils.is_intb (bnd_a + bnd_b) (ZA.lane32 (Libcrux_intrinsics.Avx2.mm256_add_epi32 a b) j))
  = let r = Libcrux_intrinsics.Avx2.mm256_add_epi32 a b in
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 r j == (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 a j + Libcrux_intrinsics.Avx2_ml_kem_views.lane32 b j) @% 4294967296);
    lemma_lane32_eq r j;
    lemma_lane32_eq a j;
    lemma_lane32_eq b j;
    assert_norm (pow2 31 == 2147483648);
    lemma_at_percent_id_32 (ZA.lane32 a j + ZA.lane32 b j)
#pop-options

let lemma_nttmul_madd_lane (a b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (bnd_a bnd_b: nat) (j: nat{j < 8}) : Lemma
  (requires 2 * (bnd_a * bnd_b) < pow2 31 /\
            Spec.Utils.is_i16b bnd_a (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a (2*j)) /\
            Spec.Utils.is_i16b bnd_a (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a (2*j+1)) /\
            Spec.Utils.is_i16b bnd_b (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b (2*j)) /\
            Spec.Utils.is_i16b bnd_b (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b (2*j+1)))
  (ensures ZA.lane32 (Libcrux_intrinsics.Avx2.mm256_madd_epi16 a b) j ==
             v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a (2*j)) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b (2*j)) +
             v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a (2*j+1)) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b (2*j+1)) /\
           Spec.Utils.is_intb (2 * (bnd_a * bnd_b)) (ZA.lane32 (Libcrux_intrinsics.Avx2.mm256_madd_epi16 a b) j))
  = let r = Libcrux_intrinsics.Avx2.mm256_madd_epi16 a b in
    Libcrux_intrinsics.Avx2_ml_kem_views.lemma_madd_epi16_lane32 a b;
    assert (Libcrux_intrinsics.Avx2_ml_kem_views.lane32 r j ==
            (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a (2*j)) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b (2*j)) +
             v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a (2*j+1)) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b (2*j+1))) @% 4294967296);
    lemma_lane32_eq r j;
    Spec.Utils.lemma_mul_intb bnd_a bnd_b (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a (2*j))) (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b (2*j)));
    Spec.Utils.lemma_mul_intb bnd_a bnd_b (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a (2*j+1))) (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b (2*j+1)));
    assert_norm (pow2 31 == 2147483648);
    lemma_at_percent_id_32 (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a (2*j)) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b (2*j)) +
                            v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a (2*j+1)) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b (2*j+1)))

#push-options "--fuel 1 --ifuel 1 --z3rlimit 50"
let lemma_set32_at_percent_mod (a:int) : Lemma ((a @% 65536) % 65536 == a % 65536) =
  FStar.Math.Lemmas.small_mod (a % 65536) 65536;
  FStar.Math.Lemmas.lemma_mod_plus (a % 65536) (-1) 65536

let lemma_set32_at_percent_id (k:int) : Lemma (requires -32768 <= k /\ k <= 32767)
                                        (ensures (k @% 65536) == k) =
  if k >= 0 then FStar.Math.Lemmas.small_mod k 65536
  else (FStar.Math.Lemmas.lemma_mod_plus k 1 65536;
        FStar.Math.Lemmas.small_mod (k + 65536) 65536)

(* An i32 x equals (low16 unsigned) + 65536*(high16 signed), where lo/hi are
   i16 lanes that agree bit-for-bit with x's two halves.  This is exactly the
   `lane32` reconstruction once lo/hi are the two i16 sub-lanes of a 32-bit
   lane carrying value x. *)
let lemma_set32_i16x2_to_i32 (lo hi: i16) (x: i32)
  : Lemma (requires (forall (n:nat{n < 16}). get_bit lo (sz n) == get_bit x (sz n)) /\
                    (forall (n:nat{n < 16}). get_bit hi (sz n) == get_bit x (sz (16+n))))
          (ensures (v lo % 65536) + 65536 * v hi == v x)
  = let lo' : i16 = Rust_primitives.Integers.cast_mod #_ #_ x in
    let hi' : i16 = Rust_primitives.Integers.cast_mod #_ #_ (x >>! mk_i32 16) in
    let aux_lo (i: usize {v i < 16}) : Lemma (get_bit lo i == get_bit lo' i) =
      assert (sz (v i) == i);
      assert (get_bit lo (sz (v i)) == get_bit x (sz (v i))) in
    Classical.forall_intro aux_lo;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits lo lo';
    let aux_hi (i: usize {v i < 16}) : Lemma (get_bit hi i == get_bit hi' i) =
      assert (sz (v i) == i);
      assert (get_bit hi (sz (v i)) == get_bit x (sz (16 + v i)));
      Rust_primitives.Integers.get_bit_shr #_ #_ x (mk_i32 16) i in
    Classical.forall_intro aux_hi;
    Rust_primitives.Integers.lemma_int_t_eq_via_bits hi hi';
    assert_norm (pow2 16 == 65536);
    assert (v lo == v x @% 65536);
    assert (v hi == (v x / 65536) @% 65536);
    lemma_set32_at_percent_mod (v x);
    lemma_set32_at_percent_id (v x / 65536);
    FStar.Math.Lemmas.euclidean_division_definition (v x) 65536
#pop-options

(* PROVEN from the canonical set_epi32 op-lemma + its per-lane interpretation
   lemma + the lane32<->to_i32x8 bridge.  (Over core-models the i32 lane view IS
   the codec, so the fact is direct.) *)
#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_nttmul_set32_lane (e7 e6 e5 e4 e3 e2 e1 e0: i32) (j: nat{j < 8}) : Lemma
  (ZA.lane32 (Libcrux_intrinsics.Avx2.mm256_set_epi32 e7 e6 e5 e4 e3 e2 e1 e0) j ==
   v (if j = 0 then e0 else if j = 1 then e1 else if j = 2 then e2
      else if j = 3 then e3 else if j = 4 then e4 else if j = 5 then e5
      else if j = 6 then e6 else e7))
  = reveal_opaque (`%Libcrux_intrinsics.Avx2.mm256_set_epi32)
                  Libcrux_intrinsics.Avx2.mm256_set_epi32;
    let vv = Libcrux_intrinsics.Avx2.mm256_set_epi32 e7 e6 e5 e4 e3 e2 e1 e0 in
    Libcrux_core_models.Intrinsics_views.lemma_mm256_set_epi32 e7 e6 e5 e4 e3 e2 e1 e0;
    Libcrux_intrinsics.Avx2_ml_kem_views.lemma_lane32_eq_to_i32x8 vv j;
    Libcrux_core_models.Intrinsics_views.lemma_iv_set_epi32 e7 e6 e5 e4 e3 e2 e1 e0 j;
    (* `ZA.lane32` is an `opaque_to_smt` COPY of the companion's `lane32`; unfold
       it so the companion's bridge fact lands on the goal. *)
    assert (ZA.lane32 vv j ==
            (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv (2*j)) % 65536) +
            65536 * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv (2*j+1)))
      by (FStar.Tactics.norm [delta_only [`%ZA.lane32]]; FStar.Tactics.trefl ())
#pop-options

let lemma_nttmul_slli16_lane (vv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (k: nat{k < 16}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_slli_epi32 (mk_i32 16) vv) k ==
   (if k % 2 = 0 then mk_i16 0 else Libcrux_intrinsics.Avx2_ml_kem_views.get_lane vv (k-1)))
  = let r = Libcrux_intrinsics.Avx2.mm256_slli_epi32 (mk_i32 16) vv in ()

let lemma_nttmul_blend_aa_lane (a b: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (k: nat{k < 16}) : Lemma
  (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_blend_epi16 (mk_i32 170) a b) k ==
   (if k % 2 = 0 then Libcrux_intrinsics.Avx2_ml_kem_views.get_lane a k else Libcrux_intrinsics.Avx2_ml_kem_views.get_lane b k))
  = let r = Libcrux_intrinsics.Avx2.mm256_blend_epi16 (mk_i32 170) a b in
    blendsel170 k

#push-options "--z3rlimit 200"
let lemma_nttmul_even_chain (p r z ab: int) : Lemma
  (requires r % 3329 == (ab * 169) % 3329)
  (ensures ((p + r * z) * 169) % 3329 == ((p + ab * z * 169) * 169) % 3329)
  = calc (==) {
      ((p + r * z) * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l (p + r * z) 169 3329 }
      ((p + r * z) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_add_distr p (r * z) 3329 }
      ((p + (r * z) % 3329) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l r z 3329 }
      ((p + (r % 3329 * z) % 3329) % 3329 * 169) % 3329;
      (==) { () }
      ((p + ((ab * 169) % 3329 * z) % 3329) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l (ab * 169) z 3329 }
      ((p + (ab * 169 * z) % 3329) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_add_distr p (ab * 169 * z) 3329 }
      ((p + ab * 169 * z) % 3329 * 169) % 3329;
      (==) { FStar.Math.Lemmas.lemma_mod_mul_distr_l (p + ab * 169 * z) 169 3329 }
      ((p + ab * 169 * z) * 169) % 3329;
      (==) { assert (ab * 169 * z == ab * z * 169) }
      ((p + ab * z * 169) * 169) % 3329;
    }
#pop-options

#push-options "--z3rlimit 400 --split_queries always"

let lemma_nttmul_prep_evens (orig m sh2 ev: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (ev128: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec128) : Lemma
  (requires
     m ==
      (Libcrux_intrinsics.Avx2.mm256_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0) (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0)) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 0) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 1) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 2) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 3) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 4) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 5) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 6) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 7) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 8) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 9) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 10) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 11) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 12) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 13) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 14) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 15) /\
     sh2 == Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 orig m) /\
     ev128 == Libcrux_intrinsics.Avx2.mm256_castsi256_si128 sh2 /\
     ev == Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 ev128)
  (ensures
     ZA.lane32 ev 0 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 0) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 0) /\
     ZA.lane32 ev 1 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 2) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 1) /\
     ZA.lane32 ev 2 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 4) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 2) /\
     ZA.lane32 ev 3 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 6) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 3) /\
     ZA.lane32 ev 4 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 8) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 4) /\
     ZA.lane32 ev 5 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 10) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 5) /\
     ZA.lane32 ev 6 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 12) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 6) /\
     ZA.lane32 ev 7 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 14) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 7))
  = let sh1 = Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 orig m in
    lemma_nttmul_cvt_lane ev128 0;
    lemma_nttmul_cast_lane sh2 0;
    lemma_nttmul_cvt_lane ev128 1;
    lemma_nttmul_cast_lane sh2 1;
    lemma_nttmul_cvt_lane ev128 2;
    lemma_nttmul_cast_lane sh2 2;
    lemma_nttmul_cvt_lane ev128 3;
    lemma_nttmul_cast_lane sh2 3;
    lemma_nttmul_cvt_lane ev128 4;
    lemma_nttmul_cast_lane sh2 4;
    lemma_nttmul_cvt_lane ev128 5;
    lemma_nttmul_cast_lane sh2 5;
    lemma_nttmul_cvt_lane ev128 6;
    lemma_nttmul_cast_lane sh2 6;
    lemma_nttmul_cvt_lane ev128 7;
    lemma_nttmul_cast_lane sh2 7;
    lemma_nttmul_permute_d8_lane sh1 0;
    lemma_nttmul_permute_d8_lane sh1 1;
    lemma_nttmul_permute_d8_lane sh1 2;
    lemma_nttmul_permute_d8_lane sh1 3;
    lemma_nttmul_permute_d8_lane sh1 4;
    lemma_nttmul_permute_d8_lane sh1 5;
    lemma_nttmul_permute_d8_lane sh1 6;
    lemma_nttmul_permute_d8_lane sh1 7;
    lemma_nttmul_permute_d8_lane sh1 8;
    lemma_nttmul_permute_d8_lane sh1 9;
    lemma_nttmul_permute_d8_lane sh1 10;
    lemma_nttmul_permute_d8_lane sh1 11;
    lemma_nttmul_permute_d8_lane sh1 12;
    lemma_nttmul_permute_d8_lane sh1 13;
    lemma_nttmul_permute_d8_lane sh1 14;
    lemma_nttmul_permute_d8_lane sh1 15;
    lemma_nttmul_shuffle_group_lane orig m 0;
    lemma_nttmul_shuffle_group_lane orig m 1;
    lemma_nttmul_shuffle_group_lane orig m 2;
    lemma_nttmul_shuffle_group_lane orig m 3;
    lemma_nttmul_shuffle_group_lane orig m 4;
    lemma_nttmul_shuffle_group_lane orig m 5;
    lemma_nttmul_shuffle_group_lane orig m 6;
    lemma_nttmul_shuffle_group_lane orig m 7;
    lemma_nttmul_shuffle_group_lane orig m 8;
    lemma_nttmul_shuffle_group_lane orig m 9;
    lemma_nttmul_shuffle_group_lane orig m 10;
    lemma_nttmul_shuffle_group_lane orig m 11;
    lemma_nttmul_shuffle_group_lane orig m 12;
    lemma_nttmul_shuffle_group_lane orig m 13;
    lemma_nttmul_shuffle_group_lane orig m 14;
    lemma_nttmul_shuffle_group_lane orig m 15;
    ()

let lemma_nttmul_prep_odds (orig m sh2 ev: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (ev128: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec128) : Lemma
  (requires
     m ==
      (Libcrux_intrinsics.Avx2.mm256_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0) (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0)) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 0) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 1) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 2) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 3) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 4) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 5) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 6) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 7) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 8) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 9) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 10) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 11) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 12) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 13) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 14) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 15) /\
     sh2 == Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 orig m) /\
     ev128 == Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) sh2 /\
     ev == Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 ev128)
  (ensures
     ZA.lane32 ev 0 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 1) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 0) /\
     ZA.lane32 ev 1 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 3) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 1) /\
     ZA.lane32 ev 2 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 5) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 2) /\
     ZA.lane32 ev 3 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 7) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 3) /\
     ZA.lane32 ev 4 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 9) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 4) /\
     ZA.lane32 ev 5 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 11) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 5) /\
     ZA.lane32 ev 6 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 13) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 6) /\
     ZA.lane32 ev 7 == v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 15) /\
     Spec.Utils.is_intb 4096 (ZA.lane32 ev 7))
  = let sh1 = Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 orig m in
    lemma_nttmul_cvt_lane ev128 0;
    lemma_nttmul_extract1_lane sh2 0;
    lemma_nttmul_cvt_lane ev128 1;
    lemma_nttmul_extract1_lane sh2 1;
    lemma_nttmul_cvt_lane ev128 2;
    lemma_nttmul_extract1_lane sh2 2;
    lemma_nttmul_cvt_lane ev128 3;
    lemma_nttmul_extract1_lane sh2 3;
    lemma_nttmul_cvt_lane ev128 4;
    lemma_nttmul_extract1_lane sh2 4;
    lemma_nttmul_cvt_lane ev128 5;
    lemma_nttmul_extract1_lane sh2 5;
    lemma_nttmul_cvt_lane ev128 6;
    lemma_nttmul_extract1_lane sh2 6;
    lemma_nttmul_cvt_lane ev128 7;
    lemma_nttmul_extract1_lane sh2 7;
    lemma_nttmul_permute_d8_lane sh1 0;
    lemma_nttmul_permute_d8_lane sh1 1;
    lemma_nttmul_permute_d8_lane sh1 2;
    lemma_nttmul_permute_d8_lane sh1 3;
    lemma_nttmul_permute_d8_lane sh1 4;
    lemma_nttmul_permute_d8_lane sh1 5;
    lemma_nttmul_permute_d8_lane sh1 6;
    lemma_nttmul_permute_d8_lane sh1 7;
    lemma_nttmul_permute_d8_lane sh1 8;
    lemma_nttmul_permute_d8_lane sh1 9;
    lemma_nttmul_permute_d8_lane sh1 10;
    lemma_nttmul_permute_d8_lane sh1 11;
    lemma_nttmul_permute_d8_lane sh1 12;
    lemma_nttmul_permute_d8_lane sh1 13;
    lemma_nttmul_permute_d8_lane sh1 14;
    lemma_nttmul_permute_d8_lane sh1 15;
    lemma_nttmul_shuffle_group_lane orig m 0;
    lemma_nttmul_shuffle_group_lane orig m 1;
    lemma_nttmul_shuffle_group_lane orig m 2;
    lemma_nttmul_shuffle_group_lane orig m 3;
    lemma_nttmul_shuffle_group_lane orig m 4;
    lemma_nttmul_shuffle_group_lane orig m 5;
    lemma_nttmul_shuffle_group_lane orig m 6;
    lemma_nttmul_shuffle_group_lane orig m 7;
    lemma_nttmul_shuffle_group_lane orig m 8;
    lemma_nttmul_shuffle_group_lane orig m 9;
    lemma_nttmul_shuffle_group_lane orig m 10;
    lemma_nttmul_shuffle_group_lane orig m 11;
    lemma_nttmul_shuffle_group_lane orig m 12;
    lemma_nttmul_shuffle_group_lane orig m 13;
    lemma_nttmul_shuffle_group_lane orig m 14;
    lemma_nttmul_shuffle_group_lane orig m 15;
    ()

let lemma_nttmul_swap_facts (orig m sw: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
     m ==
      (Libcrux_intrinsics.Avx2.mm256_set_epi8 (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2)) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 0) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 1) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 2) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 3) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 4) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 5) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 6) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 7) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 8) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 9) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 10) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 11) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 12) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 13) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 14) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 15) /\
     sw == Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 orig m)
  (ensures
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 0 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 1 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 1 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 0 /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 0) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 1) /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 2 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 3 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 3 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 2 /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 2) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 3) /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 4 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 5 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 5 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 4 /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 4) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 5) /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 6 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 7 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 7 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 6 /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 6) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 7) /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 8 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 9 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 9 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 8 /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 8) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 9) /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 10 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 11 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 11 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 10 /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 10) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 11) /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 12 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 13 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 13 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 12 /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 12) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 13) /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 14 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 15 /\
     Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 15 == Libcrux_intrinsics.Avx2_ml_kem_views.get_lane orig 14 /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 14) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane sw 15))
  =
    lemma_nttmul_swap_lane orig m 0;
    lemma_nttmul_swap_lane orig m 1;
    lemma_nttmul_swap_lane orig m 2;
    lemma_nttmul_swap_lane orig m 3;
    lemma_nttmul_swap_lane orig m 4;
    lemma_nttmul_swap_lane orig m 5;
    lemma_nttmul_swap_lane orig m 6;
    lemma_nttmul_swap_lane orig m 7;
    lemma_nttmul_swap_lane orig m 8;
    lemma_nttmul_swap_lane orig m 9;
    lemma_nttmul_swap_lane orig m 10;
    lemma_nttmul_swap_lane orig m 11;
    lemma_nttmul_swap_lane orig m 12;
    lemma_nttmul_swap_lane orig m 13;
    lemma_nttmul_swap_lane orig m 14;
    lemma_nttmul_swap_lane orig m 15;
    ()

let lemma_nttmul_out_bounds (pl prsh: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane pl 0) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane pl 2) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane pl 4) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane pl 6) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane pl 8) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane pl 10) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane pl 12) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane pl 14) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane prsh 1) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane prsh 3) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane prsh 5) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane prsh 7) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane prsh 9) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane prsh 11) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane prsh 13) /\
     Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane prsh 15))
  (ensures ZS.is_i16b_array 3328
     (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 (Libcrux_intrinsics.Avx2.mm256_blend_epi16 (mk_i32 170) pl prsh)))
  = let aux (k: nat{k < 16}) : Lemma
      (Spec.Utils.is_i16b 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane (Libcrux_intrinsics.Avx2.mm256_blend_epi16 (mk_i32 170) pl prsh) k)) =
      lemma_nttmul_blend_aa_lane pl prsh k;
      (if k = 0 then () else if k = 1 then () else if k = 2 then () else if k = 3 then () else if k = 4 then () else if k = 5 then () else if k = 6 then () else if k = 7 then () else if k = 8 then () else if k = 9 then () else if k = 10 then () else if k = 11 then () else if k = 12 then () else if k = 13 then () else if k = 14 then () else ())
    in
    Classical.forall_intro aux

#pop-options

(* `neg (cast (zeta <: i16) <: i32)` needs `range (0 - v (cast zeta)) I32`; in the
   impl's heavy whole-spine VC that ground i16->i32 cast bound gets pruned, so
   re-supply it by SMTPat.  (left/main are zv-parameterized and carry no cast, so
   this never fires there.) *)
let lemma_nttmul_cast_i16_i32_neg (x: i16) : Lemma
  (ensures Rust_primitives.Integers.range (0 - v (cast x <: i32)) Rust_primitives.Integers.I32)
  [SMTPat (cast x <: i32)]
  = ()

(* zv-fact helper: prove the per-lane facts about the zeta multiplier vector
   `zv = set_epi32 (neg (cast zeta_i)) (cast zeta_i) ...` in a LIGHT context, so the
   impl can stay a thin two-call wrapper and the `neg` subtyping discharges here
   rather than in the impl's heavy whole-spine VC. *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_nttmul_zv (zeta0 zeta1 zeta2 zeta3: i16) (zv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
     Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
     Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
     zv == (Libcrux_intrinsics.Avx2.mm256_set_epi32 (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32)
      (cast (zeta3 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32)
      (cast (zeta2 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32)
      (cast (zeta1 <: i16) <: i32)
      (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32)
      (cast (zeta0 <: i16) <: i32)))
  (ensures
     ZA.lane32 zv 0 == (v zeta0) /\
     ZA.lane32 zv 1 == (- (v zeta0)) /\
     ZA.lane32 zv 2 == (v zeta1) /\
     ZA.lane32 zv 3 == (- (v zeta1)) /\
     ZA.lane32 zv 4 == (v zeta2) /\
     ZA.lane32 zv 5 == (- (v zeta2)) /\
     ZA.lane32 zv 6 == (v zeta3) /\
     ZA.lane32 zv 7 == (- (v zeta3)) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 0) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 1) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 2) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 3) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 4) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 5) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 6) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 7))
  = lemma_nttmul_set32_lane (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32) (cast (zeta3 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32) (cast (zeta2 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32) (cast (zeta1 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32) (cast (zeta0 <: i16) <: i32) 0;
    lemma_nttmul_set32_lane (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32) (cast (zeta3 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32) (cast (zeta2 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32) (cast (zeta1 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32) (cast (zeta0 <: i16) <: i32) 1;
    lemma_nttmul_set32_lane (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32) (cast (zeta3 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32) (cast (zeta2 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32) (cast (zeta1 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32) (cast (zeta0 <: i16) <: i32) 2;
    lemma_nttmul_set32_lane (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32) (cast (zeta3 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32) (cast (zeta2 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32) (cast (zeta1 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32) (cast (zeta0 <: i16) <: i32) 3;
    lemma_nttmul_set32_lane (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32) (cast (zeta3 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32) (cast (zeta2 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32) (cast (zeta1 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32) (cast (zeta0 <: i16) <: i32) 4;
    lemma_nttmul_set32_lane (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32) (cast (zeta3 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32) (cast (zeta2 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32) (cast (zeta1 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32) (cast (zeta0 <: i16) <: i32) 5;
    lemma_nttmul_set32_lane (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32) (cast (zeta3 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32) (cast (zeta2 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32) (cast (zeta1 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32) (cast (zeta0 <: i16) <: i32) 6;
    lemma_nttmul_set32_lane (Rust_primitives.Arithmetic.neg (cast (zeta3 <: i16) <: i32) <: i32) (cast (zeta3 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta2 <: i16) <: i32) <: i32) (cast (zeta2 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta1 <: i16) <: i32) <: i32) (cast (zeta1 <: i16) <: i32) (Rust_primitives.Arithmetic.neg (cast (zeta0 <: i16) <: i32) <: i32) (cast (zeta0 <: i16) <: i32) 7;
    ()
#pop-options

(* LEFT phase of avx2 ntt_multiply: even output lanes.  Split out of a
   monolithic lemma_nttmul_main so each half verifies at the 4096 input
   bound.  The zeta multiplier vector `zv` is a FREE PARAMETER (the impl passes
   its `neg (cast zeta)` set_epi32 and proves the per-lane facts there); keeping
   `neg` out of this lemma avoids a cold subtyping cascade under split_queries. *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_nttmul_left (m lhs rhs zv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (zeta0 zeta1 zeta2 zeta3: i16) : Lemma
  (requires
     m ==
      (Libcrux_intrinsics.Avx2.mm256_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0) (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0)) /\
     Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
     Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 0) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 1) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 2) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 3) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 4) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 5) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 6) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 7) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 8) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 9) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 10) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 11) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 12) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 13) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 14) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 15) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 0) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 1) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 2) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 3) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 4) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 5) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 6) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 7) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 8) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 9) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 10) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 11) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 12) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 13) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 14) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 15) /\
     ZA.lane32 zv 0 == (v zeta0) /\
     ZA.lane32 zv 1 == (- (v zeta0)) /\
     ZA.lane32 zv 2 == (v zeta1) /\
     ZA.lane32 zv 3 == (- (v zeta1)) /\
     ZA.lane32 zv 4 == (v zeta2) /\
     ZA.lane32 zv 5 == (- (v zeta2)) /\
     ZA.lane32 zv 6 == (v zeta3) /\
     ZA.lane32 zv 7 == (- (v zeta3)) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 0) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 1) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 2) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 3) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 4) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 5) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 6) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 7))
  (ensures
     (let lhs_grouped = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 lhs m) in
      let rhs_grouped = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 rhs m) in
      let left = Libcrux_intrinsics.Avx2.mm256_mullo_epi32
                   (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 lhs_grouped))
                   (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 rhs_grouped)) in
      let odd_products = Libcrux_intrinsics.Avx2.mm256_mullo_epi32
                   (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) lhs_grouped))
                   (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) rhs_grouped)) in
      let odd_products_reduced = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s odd_products in
      let right = Libcrux_intrinsics.Avx2.mm256_mullo_epi32 odd_products_reduced zv in
      let products_left_raw = Libcrux_intrinsics.Avx2.mm256_add_epi32 left right in
      let products_left = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s products_left_raw in
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 0) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 1) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 2) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 3) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 4) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 5) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 6) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 7) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 0) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 1) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 2) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 3) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 4) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 5) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 6) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 7) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 0) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 0) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 0) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 1) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 1) * (v zeta0) * 169) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 2) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 2) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 2) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 3) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 3) * (- (v zeta0)) * 169) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 4) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 4) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 4) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 5) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 5) * (v zeta1) * 169) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 6) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 6) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 6) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 7) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 7) * (- (v zeta1)) * 169) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 8) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 8) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 8) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 9) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 9) * (v zeta2) * 169) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 10) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 10) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 10) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 11) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 11) * (- (v zeta2)) * 169) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 12) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 12) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 12) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 13) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 13) * (v zeta3) * 169) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 14) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 14) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 14) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 15) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 15) * (- (v zeta3)) * 169) * 169) % 3329)))
  = assert_norm (pow2 15 == 32768);
    assert_norm (pow2 31 == 2147483648);
    let lhs_grouped = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 lhs m) in
    let rhs_grouped = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 rhs m) in
    let lhs_ev128 = Libcrux_intrinsics.Avx2.mm256_castsi256_si128 lhs_grouped in
    let lhs_evens = Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 lhs_ev128 in
    let lhs_od128 = Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) lhs_grouped in
    let lhs_odds = Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 lhs_od128 in
    let rhs_ev128 = Libcrux_intrinsics.Avx2.mm256_castsi256_si128 rhs_grouped in
    let rhs_evens = Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 rhs_ev128 in
    let rhs_od128 = Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) rhs_grouped in
    let rhs_odds = Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 rhs_od128 in
    lemma_nttmul_prep_evens lhs m lhs_grouped lhs_evens lhs_ev128;
    lemma_nttmul_prep_odds lhs m lhs_grouped lhs_odds lhs_od128;
    lemma_nttmul_prep_evens rhs m rhs_grouped rhs_evens rhs_ev128;
    lemma_nttmul_prep_odds rhs m rhs_grouped rhs_odds rhs_od128;
    let left = Libcrux_intrinsics.Avx2.mm256_mullo_epi32 lhs_evens rhs_evens in
    let odd_products = Libcrux_intrinsics.Avx2.mm256_mullo_epi32 lhs_odds rhs_odds in
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 4096 4096 0;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 4096 4096 1;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 4096 4096 2;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 4096 4096 3;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 4096 4096 4;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 4096 4096 5;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 4096 4096 6;
    lemma_nttmul_mullo32_lane lhs_evens rhs_evens 4096 4096 7;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 4096 4096 0;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 4096 4096 1;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 4096 4096 2;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 4096 4096 3;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 4096 4096 4;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 4096 4096 5;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 4096 4096 6;
    lemma_nttmul_mullo32_lane lhs_odds rhs_odds 4096 4096 7;
    let odd_products_reduced = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s odd_products in
    lemma_nttmul_even_chain
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 0) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 0))
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane odd_products_reduced 0)) (v zeta0)
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 1) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 1));
    lemma_nttmul_even_chain
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 2) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 2))
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane odd_products_reduced 2)) (- (v zeta0))
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 3) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 3));
    lemma_nttmul_even_chain
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 4) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 4))
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane odd_products_reduced 4)) (v zeta1)
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 5) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 5));
    lemma_nttmul_even_chain
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 6) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 6))
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane odd_products_reduced 6)) (- (v zeta1))
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 7) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 7));
    lemma_nttmul_even_chain
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 8) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 8))
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane odd_products_reduced 8)) (v zeta2)
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 9) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 9));
    lemma_nttmul_even_chain
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 10) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 10))
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane odd_products_reduced 10)) (- (v zeta2))
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 11) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 11));
    lemma_nttmul_even_chain
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 12) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 12))
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane odd_products_reduced 12)) (v zeta3)
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 13) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 13));
    lemma_nttmul_even_chain
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 14) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 14))
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane odd_products_reduced 14)) (- (v zeta3))
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 15) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 15));
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 0;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 1;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 2;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 3;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 4;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 5;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 6;
    lemma_nttmul_mullo32_lane odd_products_reduced zv 3328 1664 7;
    let right = Libcrux_intrinsics.Avx2.mm256_mullo_epi32 odd_products_reduced zv in
    lemma_nttmul_add32_lane left right (4096 * 4096) (3328 * 1664) 0;
    lemma_nttmul_add32_lane left right (4096 * 4096) (3328 * 1664) 1;
    lemma_nttmul_add32_lane left right (4096 * 4096) (3328 * 1664) 2;
    lemma_nttmul_add32_lane left right (4096 * 4096) (3328 * 1664) 3;
    lemma_nttmul_add32_lane left right (4096 * 4096) (3328 * 1664) 4;
    lemma_nttmul_add32_lane left right (4096 * 4096) (3328 * 1664) 5;
    lemma_nttmul_add32_lane left right (4096 * 4096) (3328 * 1664) 6;
    lemma_nttmul_add32_lane left right (4096 * 4096) (3328 * 1664) 7;
    let products_left_raw = Libcrux_intrinsics.Avx2.mm256_add_epi32 left right in
    let products_left = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s products_left_raw in
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 0) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 0) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 0) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 1) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 1) * (v zeta0) * 169) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 2) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 2) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 2) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 3) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 3) * (- (v zeta0)) * 169) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 4) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 4) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 4) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 5) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 5) * (v zeta1) * 169) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 6) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 6) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 6) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 7) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 7) * (- (v zeta1)) * 169) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 8) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 8) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 8) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 9) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 9) * (v zeta2) * 169) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 10) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 10) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 10) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 11) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 11) * (- (v zeta2)) * 169) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 12) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 12) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 12) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 13) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 13) * (v zeta3) * 169) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_left 14) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 14) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 14) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 15) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 15) * (- (v zeta3)) * 169) * 169) % 3329);
    ()
#pop-options

(* RIGHT phase of avx2 ntt_multiply: odd output lanes.  Proves the
   products_right_raw is_intb bounds + the 8 products_right_reduced residues
   that feed the odd half of ntt_multiply_butterfly_post. *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_nttmul_right (ms lhs rhs: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) : Lemma
  (requires
     ms ==
      (Libcrux_intrinsics.Avx2.mm256_set_epi8 (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2)) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 0) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 1) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 2) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 3) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 4) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 5) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 6) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 7) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 8) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 9) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 10) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 11) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 12) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 13) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 14) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 15) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 0) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 1) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 2) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 3) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 4) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 5) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 6) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 7) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 8) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 9) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 10) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 11) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 12) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 13) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 14) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 15))
  (ensures
     (let rhs_adjacent_swapped = Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 rhs ms in
      let products_right_raw = Libcrux_intrinsics.Avx2.mm256_madd_epi16 lhs rhs_adjacent_swapped in
      let products_right_reduced = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s products_right_raw in
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 0) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 1) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 2) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 3) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 4) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 5) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 6) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 7) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 0) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 0) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 1) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 1) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 0)) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 2) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 2) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 3) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 3) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 2)) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 4) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 4) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 5) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 5) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 4)) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 6) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 6) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 7) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 7) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 6)) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 8) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 8) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 9) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 9) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 8)) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 10) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 10) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 11) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 11) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 10)) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 12) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 12) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 13) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 13) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 12)) * 169) % 3329) /\
      (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 14) % 3329 ==
        ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 14) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 15) +
          v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 15) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 14)) * 169) % 3329)))
  = assert_norm (pow2 15 == 32768);
    assert_norm (pow2 31 == 2147483648);
    let rhs_adjacent_swapped = Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 rhs ms in
    lemma_nttmul_swap_facts rhs ms rhs_adjacent_swapped;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 4096 4096 0;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 4096 4096 1;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 4096 4096 2;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 4096 4096 3;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 4096 4096 4;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 4096 4096 5;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 4096 4096 6;
    lemma_nttmul_madd_lane lhs rhs_adjacent_swapped 4096 4096 7;
    let products_right_raw = Libcrux_intrinsics.Avx2.mm256_madd_epi16 lhs rhs_adjacent_swapped in
    let products_right_reduced = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s products_right_raw in
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 0) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 0) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 1) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 1) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 0)) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 2) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 2) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 3) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 3) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 2)) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 4) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 4) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 5) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 5) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 4)) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 6) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 6) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 7) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 7) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 6)) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 8) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 8) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 9) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 9) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 8)) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 10) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 10) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 11) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 11) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 10)) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 12) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 12) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 13) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 13) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 12)) * 169) % 3329);
    assert (v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane products_right_reduced 14) % 3329 ==
      ((v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 14) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 15) +
        v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 15) * v (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 14)) * 169) % 3329);
    ()
#pop-options

(* Thin composer: delegates the even/odd halves to lemma_nttmul_left /
   lemma_nttmul_right, then blends + reveals ntt_multiply_butterfly_post.
   `zv` (the neg/cast zeta vector) is a free parameter, supplied + justified by
   the ntt_multiply impl, so this lemma stays free of `neg` subtyping. *)
#push-options "--z3rlimit 400 --split_queries always"
let lemma_nttmul_main (m ms lhs rhs zv: Libcrux_intrinsics.Avx2_ml_kem_views.t_Vec256) (zeta0 zeta1 zeta2 zeta3: i16) : Lemma
  (requires
     ms ==
      (Libcrux_intrinsics.Avx2.mm256_set_epi8 (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 15) (mk_i8 14) (mk_i8 9) (mk_i8 8)
      (mk_i8 11) (mk_i8 10) (mk_i8 5) (mk_i8 4) (mk_i8 7) (mk_i8 6) (mk_i8 1) (mk_i8 0)
      (mk_i8 3) (mk_i8 2)) /\
     m ==
      (Libcrux_intrinsics.Avx2.mm256_set_epi8 (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0) (mk_i8 15) (mk_i8 14) (mk_i8 11) (mk_i8 10) (mk_i8 7) (mk_i8 6)
      (mk_i8 3) (mk_i8 2) (mk_i8 13) (mk_i8 12) (mk_i8 9) (mk_i8 8) (mk_i8 5) (mk_i8 4)
      (mk_i8 1) (mk_i8 0)) /\
     Spec.Utils.is_i16b 1664 zeta0 /\ Spec.Utils.is_i16b 1664 zeta1 /\
     Spec.Utils.is_i16b 1664 zeta2 /\ Spec.Utils.is_i16b 1664 zeta3 /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 0) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 1) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 2) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 3) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 4) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 5) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 6) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 7) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 8) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 9) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 10) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 11) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 12) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 13) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 14) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane lhs 15) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 0) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 1) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 2) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 3) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 4) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 5) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 6) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 7) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 8) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 9) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 10) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 11) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 12) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 13) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 14) /\
     Spec.Utils.is_i16b 4096 (Libcrux_intrinsics.Avx2_ml_kem_views.get_lane rhs 15) /\
     ZA.lane32 zv 0 == (v zeta0) /\
     ZA.lane32 zv 1 == (- (v zeta0)) /\
     ZA.lane32 zv 2 == (v zeta1) /\
     ZA.lane32 zv 3 == (- (v zeta1)) /\
     ZA.lane32 zv 4 == (v zeta2) /\
     ZA.lane32 zv 5 == (- (v zeta2)) /\
     ZA.lane32 zv 6 == (v zeta3) /\
     ZA.lane32 zv 7 == (- (v zeta3)) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 0) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 1) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 2) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 3) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 4) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 5) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 6) /\
     Spec.Utils.is_intb 1664 (ZA.lane32 zv 7))
  (ensures
     (let lhs_grouped = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 lhs m) in
      let rhs_grouped = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 rhs m) in
      let left = Libcrux_intrinsics.Avx2.mm256_mullo_epi32
                   (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 lhs_grouped))
                   (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 rhs_grouped)) in
      let odd_products = Libcrux_intrinsics.Avx2.mm256_mullo_epi32
                   (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) lhs_grouped))
                   (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) rhs_grouped)) in
      let odd_products_reduced = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s odd_products in
      let right = Libcrux_intrinsics.Avx2.mm256_mullo_epi32 odd_products_reduced zv in
      let products_left_raw = Libcrux_intrinsics.Avx2.mm256_add_epi32 left right in
      let products_left = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s products_left_raw in
      let rhs_adjacent_swapped = Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 rhs ms in
      let products_right_raw = Libcrux_intrinsics.Avx2.mm256_madd_epi16 lhs rhs_adjacent_swapped in
      let products_right_reduced = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s products_right_raw in
      let products_right = Libcrux_intrinsics.Avx2.mm256_slli_epi32 (mk_i32 16) products_right_reduced in
      let out = Libcrux_intrinsics.Avx2.mm256_blend_epi16 (mk_i32 170) products_left products_right in
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 0) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 1) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 2) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 3) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 4) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 5) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 6) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 odd_products 7) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 0) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 1) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 2) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 3) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 4) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 5) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 6) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_left_raw 7) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 0) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 1) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 2) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 3) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 4) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 5) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 6) /\
      Spec.Utils.is_intb (3328 * pow2 15) (ZA.lane32 products_right_raw 7) /\
      ZS.is_i16b_array 3328 (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 out) /\
      Spec.Utils.ntt_multiply_butterfly_post
        (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 lhs) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 rhs)
        (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 out) zeta0 zeta1 zeta2 zeta3))
  = assert_norm (pow2 15 == 32768);
    assert_norm (pow2 31 == 2147483648);
    lemma_nttmul_left m lhs rhs zv zeta0 zeta1 zeta2 zeta3;
    lemma_nttmul_right ms lhs rhs;
    let lhs_grouped = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 lhs m) in
    let rhs_grouped = Libcrux_intrinsics.Avx2.mm256_permute4x64_epi64 (mk_i32 216) (Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 rhs m) in
    let left = Libcrux_intrinsics.Avx2.mm256_mullo_epi32
                 (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 lhs_grouped))
                 (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 (Libcrux_intrinsics.Avx2.mm256_castsi256_si128 rhs_grouped)) in
    let odd_products = Libcrux_intrinsics.Avx2.mm256_mullo_epi32
                 (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) lhs_grouped))
                 (Libcrux_intrinsics.Avx2.mm256_cvtepi16_epi32 (Libcrux_intrinsics.Avx2.mm256_extracti128_si256 (mk_i32 1) rhs_grouped)) in
    let odd_products_reduced = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s odd_products in
    let right = Libcrux_intrinsics.Avx2.mm256_mullo_epi32 odd_products_reduced zv in
    let products_left_raw = Libcrux_intrinsics.Avx2.mm256_add_epi32 left right in
    let products_left = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s products_left_raw in
    let rhs_adjacent_swapped = Libcrux_intrinsics.Avx2.mm256_shuffle_epi8 rhs ms in
    let products_right_raw = Libcrux_intrinsics.Avx2.mm256_madd_epi16 lhs rhs_adjacent_swapped in
    let products_right_reduced = Libcrux_ml_kem.Vector.Avx2.Arithmetic.montgomery_reduce_i32s products_right_raw in
    lemma_nttmul_slli16_lane products_right_reduced 1;
    lemma_nttmul_slli16_lane products_right_reduced 3;
    lemma_nttmul_slli16_lane products_right_reduced 5;
    lemma_nttmul_slli16_lane products_right_reduced 7;
    lemma_nttmul_slli16_lane products_right_reduced 9;
    lemma_nttmul_slli16_lane products_right_reduced 11;
    lemma_nttmul_slli16_lane products_right_reduced 13;
    lemma_nttmul_slli16_lane products_right_reduced 15;
    let products_right = Libcrux_intrinsics.Avx2.mm256_slli_epi32 (mk_i32 16) products_right_reduced in
    lemma_nttmul_blend_aa_lane products_left products_right 0;
    lemma_nttmul_blend_aa_lane products_left products_right 1;
    lemma_nttmul_blend_aa_lane products_left products_right 2;
    lemma_nttmul_blend_aa_lane products_left products_right 3;
    lemma_nttmul_blend_aa_lane products_left products_right 4;
    lemma_nttmul_blend_aa_lane products_left products_right 5;
    lemma_nttmul_blend_aa_lane products_left products_right 6;
    lemma_nttmul_blend_aa_lane products_left products_right 7;
    lemma_nttmul_blend_aa_lane products_left products_right 8;
    lemma_nttmul_blend_aa_lane products_left products_right 9;
    lemma_nttmul_blend_aa_lane products_left products_right 10;
    lemma_nttmul_blend_aa_lane products_left products_right 11;
    lemma_nttmul_blend_aa_lane products_left products_right 12;
    lemma_nttmul_blend_aa_lane products_left products_right 13;
    lemma_nttmul_blend_aa_lane products_left products_right 14;
    lemma_nttmul_blend_aa_lane products_left products_right 15;
    lemma_nttmul_out_bounds products_left products_right;
    let out = Libcrux_intrinsics.Avx2.mm256_blend_epi16 (mk_i32 170) products_left products_right in
    reveal_opaque (`%Spec.Utils.ntt_multiply_butterfly_post)
      (Spec.Utils.ntt_multiply_butterfly_post
        (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 lhs) (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 rhs)
        (Libcrux_intrinsics.Avx2_ml_kem_views.vec256_as_i16x16 out) zeta0 zeta1 zeta2 zeta3)
#pop-options

#pop-options

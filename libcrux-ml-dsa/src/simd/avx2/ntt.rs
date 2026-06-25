use super::{arithmetic, AVX2RingElement, AVX2SIMDUnit};
use crate::simd::traits::COEFFICIENTS_IN_SIMD_UNIT;

use libcrux_intrinsics::avx2::*;

// Compute (a,b) ↦ (a + ζb, a - ζb) at layer 0 for 2 SIMD Units in one go.
#[inline(always)]
#[hax_lib::fstar::before(r"open Spec.MLDSA.NttConstants")]
#[hax_lib::fstar::before(r"open Spec.Intrinsics")]
#[hax_lib::fstar::before(
    r#"
let butterfly_2_spec re0 re1 zeta_a0 zeta_a1 zeta_a2 zeta_a3 
                     zeta_b0 zeta_b1 zeta_b2 zeta_b3 nre0 nre1 =
    (to_i32x8 nre0 (mk_u64 0), to_i32x8 nre0 (mk_u64 1)) ==
     ntt_step zeta_a0 (to_i32x8 re0 (mk_u64 0), to_i32x8 re0 (mk_u64 1)) /\
    (to_i32x8 nre0 (mk_u64 2), to_i32x8 nre0 (mk_u64 3)) ==
     ntt_step zeta_a1 (to_i32x8 re0 (mk_u64 2), to_i32x8 re0 (mk_u64 3)) /\
    (to_i32x8 nre0 (mk_u64 4), to_i32x8 nre0 (mk_u64 5)) ==
     ntt_step zeta_a2 (to_i32x8 re0 (mk_u64 4), to_i32x8 re0 (mk_u64 5)) /\
    (to_i32x8 nre0 (mk_u64 6), to_i32x8 nre0 (mk_u64 7)) ==
     ntt_step zeta_a3 (to_i32x8 re0 (mk_u64 6), to_i32x8 re0 (mk_u64 7)) /\
    (to_i32x8 nre1 (mk_u64 0), to_i32x8 nre1 (mk_u64 1)) ==
     ntt_step zeta_b0 (to_i32x8 re1 (mk_u64 0), to_i32x8 re1 (mk_u64 1)) /\
    (to_i32x8 nre1 (mk_u64 2), to_i32x8 nre1 (mk_u64 3)) ==
     ntt_step zeta_b1 (to_i32x8 re1 (mk_u64 2), to_i32x8 re1 (mk_u64 3)) /\
    (to_i32x8 nre1 (mk_u64 4), to_i32x8 nre1 (mk_u64 5)) ==
     ntt_step zeta_b2 (to_i32x8 re1 (mk_u64 4), to_i32x8 re1 (mk_u64 5)) /\
    (to_i32x8 nre1 (mk_u64 6), to_i32x8 nre1 (mk_u64 7)) ==
     ntt_step zeta_b3 (to_i32x8 re1 (mk_u64 6), to_i32x8 re1 (mk_u64 7))
"#
)]
#[hax_lib::fstar::before(r#"
open Core_models
open FStar.Mul
open Spec.MLDSA.Math
module C = Hacspec_ml_dsa.Commute.Chunk
#push-options "--fuel 0 --ifuel 1 --z3rlimit 80"

(* AVX2 analog of Portable's `chunks_of_re`: project the 32 Vec256 SIMD
   units to the flat-chunk view the Commute.Chunk poly lemmas consume.
   Lane access on AVX2 is the bitvec projection `to_i32x8 vec (mk_u64 l)`,
   not the array index `.f_values.[l]` Portable uses. *)
let chunks_of_re_avx2
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32)
  = Hacspec_ml_dsa.createi #(t_Array i32 (mk_usize 8)) (mk_usize 32)
      #(usize -> t_Array i32 (mk_usize 8))
      (fun (b: usize{b <. mk_usize 32}) ->
         Hacspec_ml_dsa.createi #i32 (mk_usize 8)
           #(usize -> i32)
           (fun (l: usize{l <. mk_usize 8}) ->
              to_i32x8 (Seq.index re (v b)).f_value (mk_u64 (v l))))

(* Index reveal: `chunks_of_re_avx2 re` at chunk b, lane l is the AVX2
   lane projection of unit b.  Two createi_lemma applications. *)
let lemma_chunks_of_re_avx2_index
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (b: nat{b < 32}) (l: nat{l < 8})
    : Lemma (Seq.index (Seq.index (chunks_of_re_avx2 re) b) l ==
             to_i32x8 (Seq.index re b).f_value (mk_u64 l))
  = assert (v (mk_usize b) == b);
    assert (v (mk_usize l) == l);
    let inner = Hacspec_ml_dsa.createi #i32 (mk_usize 8)
                  #(usize -> i32)
                  (fun (l: usize{l <. mk_usize 8}) ->
                     to_i32x8 (Seq.index re b).f_value (mk_u64 (v l))) in
    Hacspec_ml_dsa.createi_lemma #(t_Array i32 (mk_usize 8)) (mk_usize 32)
      #(usize -> t_Array i32 (mk_usize 8))
      (fun (b: usize{b <. mk_usize 32}) ->
         Hacspec_ml_dsa.createi #i32 (mk_usize 8)
           #(usize -> i32)
           (fun (l: usize{l <. mk_usize 8}) ->
              to_i32x8 (Seq.index re (v b)).f_value (mk_u64 (v l))))
      (mk_usize b);
    Hacspec_ml_dsa.createi_lemma #i32 (mk_usize 8)
      #(usize -> i32)
      (fun (l: usize{l <. mk_usize 8}) ->
         to_i32x8 (Seq.index re b).f_value (mk_u64 (v l)))
      (mk_usize l)

(* Sanity: the flat view of chunks_of_re_avx2 re, at flat index 8b+l, is
   the AVX2 lane projection — this is what the drivers will rely on to
   bridge AVX2 per-lane posts to the Commute.Chunk simd_units_to_array view. *)
let lemma_flat_avx2_index
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (b: nat{b < 32}) (l: nat{l < 8})
    : Lemma (v (Seq.index (C.simd_units_to_array (chunks_of_re_avx2 re)) (8*b + l)) ==
             v (to_i32x8 (Seq.index re b).f_value (mk_u64 l)))
  = C.lemma_simd_units_to_array_reveal (chunks_of_re_avx2 re) b l;
    lemma_chunks_of_re_avx2_index re b l

(* Direct AVX2 per-lane bound predicate.  OPAQUE (mirrors Portable's
   is_i32b_array_opaque / is_i32b_polynomial discipline) so the driver WP never
   expands the 256 per-lane facts; reveal only inside the leaf lemmas. *)
[@@ "opaque_to_smt"]
let is_i32b_poly_avx2 (bnd:nat)
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  forall (u:nat) (l:nat). u < 32 /\ l < 8 ==>
    Spec.Utils.is_i32b bnd (to_i32x8 (Seq.index re u).f_value (mk_u64 l))

(* intro/elim for the opaque bound predicate — consumers cite these, never reveal. *)
let lemma_is_i32b_poly_avx2_elim (bnd:nat)
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (u:nat{u<32}) (l:nat{l<8})
    : Lemma (requires is_i32b_poly_avx2 bnd re)
            (ensures Spec.Utils.is_i32b bnd (to_i32x8 (Seq.index re u).f_value (mk_u64 l)))
  = reveal_opaque (`%is_i32b_poly_avx2) is_i32b_poly_avx2

let lemma_is_i32b_poly_avx2_intro (bnd:nat)
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires forall (u:nat) (l:nat). u<32 /\ l<8 ==>
               Spec.Utils.is_i32b bnd (to_i32x8 (Seq.index re u).f_value (mk_u64 l)))
            (ensures is_i32b_poly_avx2 bnd re)
  = reveal_opaque (`%is_i32b_poly_avx2) is_i32b_poly_avx2

(* CRUX (the genuinely-new AVX2 logic): from the per-(b,p) `ntt_step` post +
   zeta bound + input bound, derive the 4 butterfly relations that
   `lemma_ntt_layer_0_step_to_hacspec_poly` consumes.  Input bound only needed
   for add/sub no-overflow exactness (`bnd + FIELD_MAX < pow2 31`); the mont
   mod-q + FIELD_MAX bound hold for ANY input (zeta is bounded by 4190208). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_l0_pair_relations
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{bnd + 8380416 < pow2 31})
      (b: nat{b < 32}) (p: nat{p < 4})
    : Lemma
        (requires
          is_i32b_poly_avx2 bnd re /\
          (let ci = chunks_of_re_avx2 re in
           let co = chunks_of_re_avx2 re_fut in
           (Seq.index (Seq.index co b) (2*p), Seq.index (Seq.index co b) (2*p+1)) ==
             ntt_step (mk_int (zeta_r (128 + 4*b + p)))
               (Seq.index (Seq.index ci b) (2*p), Seq.index (Seq.index ci b) (2*p+1))))
        (ensures
          (let ci = chunks_of_re_avx2 re in
           let co = chunks_of_re_avx2 re_fut in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (4*b + p + 128) ] in
           let zm : i32 = mk_int (zeta_r (128 + 4*b + p)) in
           let t : i32 = mont_mul (Seq.index (Seq.index ci b) (2*p+1)) zm in
           v (Seq.index (Seq.index co b) (2*p))   == v (Seq.index (Seq.index ci b) (2*p)) + v t /\
           v (Seq.index (Seq.index co b) (2*p+1)) == v (Seq.index (Seq.index ci b) (2*p)) - v t /\
           (v t) % 8380417 ==
             (v (Seq.index (Seq.index ci b) (2*p+1)) * v zm * 8265825) % 8380417 /\
           (v zm) % 8380417 == (v z * pow2 32) % 8380417 /\
           // output bound (drives the per-layer bound accumulation chain)
           Spec.Utils.is_i32b (bnd + 8380416) (Seq.index (Seq.index co b) (2*p)) /\
           Spec.Utils.is_i32b (bnd + 8380416) (Seq.index (Seq.index co b) (2*p+1))))
  = let ci = chunks_of_re_avx2 re in
    let co = chunks_of_re_avx2 re_fut in
    let ci_lo = Seq.index (Seq.index ci b) (2*p) in
    let ci_hi = Seq.index (Seq.index ci b) (2*p+1) in
    let co_lo = Seq.index (Seq.index co b) (2*p) in
    let co_hi = Seq.index (Seq.index co b) (2*p+1) in
    let zm : i32 = mk_int (zeta_r (128 + 4*b + p)) in
    let t : i32 = mont_mul ci_hi zm in
    // ntt_step unfolds (non-opaque):
    assert (co_lo == add_mod_opaque ci_lo t);
    assert (co_hi == sub_mod_opaque ci_lo t);
    // input bound on ci_lo (via the opaque-predicate elim, not a raw reveal)
    lemma_chunks_of_re_avx2_index re b (2*p);
    lemma_is_i32b_poly_avx2_elim bnd re b (2*p);
    assert (Spec.Utils.is_i32b bnd ci_lo);
    // mont bound + mod-q (zeta_r bounded by 4190208 < FIELD_MAX)
    assert (Spec.Utils.is_i32b 8380416 zm);
    C.lemma_mont_mul_bound_and_mod_q ci_hi zm;
    assert (Spec.Utils.is_i32b 8380416 t);
    // add/sub exactness (no overflow)
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    assert (v co_lo == v ci_lo + v t);
    assert (v co_hi == v ci_lo - v t);
    // zeta canonicalization
    let idx : nat = 128 + 4*b + p in
    C.lemma_v_zetas_eq_zeta idx
#pop-options

(* ===== Dispatch probe: extract per-(b,p) chunk ntt_step fact from the
   verbatim L0 post (norm[..](forall16(forall4 ..))).  Tests even-parity,
   odd-parity, and a non-zero pair, to confirm the 32x4 dispatch leaf shape
   before generating the full driver. ===== *)
unfold let l0_post (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  norm [
      primops; iota;
      delta_namespace [`%zeta_r; `%Spec.Utils.forall4; `%Spec.Utils.forall16]
    ]
    (Spec.Utils.forall16 (fun i ->
          let nre = re_fut in
          let re0 = Seq.index re (i * 2) in
          let re1 = Seq.index re (i * 2 + 1) in
          let nre0 = Seq.index nre (i * 2) in
          let nre1 = Seq.index nre (i * 2 + 1) in
          Spec.Utils.forall4 (fun j ->
                let zeta0 = zeta_r (128 + i * 8 + j) in
                let zeta1 = zeta_r (128 + i * 8 + j + 4) in
                let j0 = j * 2 in
                let j1 = j0 + 1 in
                (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
                ntt_step (mk_int zeta0)
                  (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
                (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
                ntt_step (mk_int zeta1)
                  (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))))

unfold let chunkfact (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                     (b:nat{b<32}) (p:nat{p<4}) : Type0 =
  let ci = chunks_of_re_avx2 re in
  let co = chunks_of_re_avx2 re_fut in
  (Seq.index (Seq.index co b) (2*p), Seq.index (Seq.index co b) (2*p+1)) ==
    ntt_step (mk_int (zeta_r (128 + 4*b + p)))
      (Seq.index (Seq.index ci b) (2*p), Seq.index (Seq.index ci b) (2*p+1))


(* ===== ARCHITECTURE TEST: cheap 16-arm forall16-elim against the SYMBOLIC
   post (zeta_r NOT norm-evaluated).  Each arm is a direct-conjunct match
   (forall32_elim_1d style) — should be fast, unlike the 128-leaf search. ===== *)
unfold let l0_post_sym (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  norm [
      primops; iota;
      delta_namespace [`%Spec.Utils.forall4; `%Spec.Utils.forall16]
    ]
    (Spec.Utils.forall16 (fun i ->
          let nre = re_fut in
          let re0 = Seq.index re (i * 2) in
          let re1 = Seq.index re (i * 2 + 1) in
          let nre0 = Seq.index nre (i * 2) in
          let nre1 = Seq.index nre (i * 2 + 1) in
          Spec.Utils.forall4 (fun j ->
                let zeta0 = zeta_r (128 + i * 8 + j) in
                let zeta1 = zeta_r (128 + i * 8 + j + 4) in
                let j0 = j * 2 in
                let j1 = j0 + 1 in
                (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
                ntt_step (mk_int zeta0)
                  (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
                (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
                ntt_step (mk_int zeta1)
                  (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))))

unfold let l0_body (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                   (i:nat{i<16}) : Type0 =
  let re0 = Seq.index re (i*2) in
  let re1 = Seq.index re (i*2+1) in
  let nre0 = Seq.index re_fut (i*2) in
  let nre1 = Seq.index re_fut (i*2+1) in
  Spec.Utils.forall4 (fun j ->
        let zeta0 = zeta_r (128 + i*8 + j) in
        let zeta1 = zeta_r (128 + i*8 + j + 4) in
        let j0 = j*2 in let j1 = j0+1 in
        (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
          ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
        (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
          ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))

(* Generic forall16-elim with ABSTRACT r (mirrors Portable forall32_elim_1d):
   each arm is a cheap direct-conjunct match because `r i` is opaque — no heavy
   body reduction.  The expensive l0_body is only substituted at the call. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 40"
let forall16_elim_1d (r: (i: nat{i < 16}) -> Type0)
    : Lemma (requires Spec.Utils.forall16 r) (ensures forall (i: nat{i < 16}). r i)
  = let aux (i: nat{i < 16}) : Lemma (r i) =
      (match i with
       | 0 -> () | 1 -> () | 2 -> () | 3 -> () | 4 -> () | 5 -> () | 6 -> () | 7 -> ()
       | 8 -> () | 9 -> () | 10 -> () | 11 -> () | 12 -> () | 13 -> () | 14 -> () | _ -> ())
    in Classical.forall_intro aux
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 60"
let lemma_lift (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires l0_post_sym re re_fut)
            (ensures forall (i:nat{i<16}). l0_body re re_fut i)
  = forall16_elim_1d (l0_body re re_fut)
#pop-options

(* ===== Final L0 glue: forall i. l0_body i  ==>  forall (b,p). chunkfact b p.
   Per-(b,p): instantiate i=b/2 (Euclidean), parity split (nre0/nre1), index lemmas. ===== *)
unfold let body2 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                 (i:nat{i<16}) (j:nat{j<4}) : Type0 =
  let re0 = Seq.index re (i*2) in
  let re1 = Seq.index re (i*2+1) in
  let nre0 = Seq.index re_fut (i*2) in
  let nre1 = Seq.index re_fut (i*2+1) in
  let zeta0 = zeta_r (128 + i*8 + j) in
  let zeta1 = zeta_r (128 + i*8 + j + 4) in
  let j0 = j*2 in let j1 = j0+1 in
  (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
    ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
  (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
    ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1))

#push-options "--fuel 0 --ifuel 1 --z3rlimit 40"
let forall4_elim_1d (r: (j: nat{j < 4}) -> Type0)
    : Lemma (requires Spec.Utils.forall4 r) (ensures forall (j: nat{j < 4}). r j)
  = let aux (j: nat{j < 4}) : Lemma (r j) =
      (match j with | 0 -> () | 1 -> () | 2 -> () | _ -> ())
    in Classical.forall_intro aux
#pop-options

(* l0_body i is definitionally forall4 (fun j -> body2 i j); lift to forall i j. body2. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 80"
let lemma_lift2 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires l0_post_sym re re_fut)
            (ensures forall (i:nat{i<16}) (j:nat{j<4}). body2 re re_fut i j)
  = lemma_lift re re_fut;
    let aux (i:nat{i<16}) : Lemma (forall (j:nat{j<4}). body2 re re_fut i j) =
      forall4_elim_1d (fun (j:nat{j<4}) -> body2 re re_fut i j)
    in Classical.forall_intro aux
#pop-options

(* ===== From the symbolic L0 post (16x4 ntt_step facts) to the 32x4 chunk
   ntt_step facts the bridge consumes.  Per (b,p): instantiate i=b/2, parity of b
   selects nre0/nre1, index lemmas bridge to_i32x8 <-> chunks_of_re_avx2. ===== *)
(* Even chunk b=2i: chunkfact (2i) p comes from body2 i p's nre0 part. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunkfact_even
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:nat{i<16}) (p:nat{p<4})
    : Lemma (requires body2 re re_fut i p) (ensures chunkfact re re_fut (2*i) p)
  = lemma_chunks_of_re_avx2_index re (2*i) (2*p);
    lemma_chunks_of_re_avx2_index re (2*i) (2*p+1);
    lemma_chunks_of_re_avx2_index re_fut (2*i) (2*p);
    lemma_chunks_of_re_avx2_index re_fut (2*i) (2*p+1)
#pop-options

(* Odd chunk b=2i+1: chunkfact (2i+1) p comes from body2 i p's nre1 part. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunkfact_odd
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:nat{i<16}) (p:nat{p<4})
    : Lemma (requires body2 re re_fut i p) (ensures chunkfact re re_fut (2*i+1) p)
  = lemma_chunks_of_re_avx2_index re (2*i+1) (2*p);
    lemma_chunks_of_re_avx2_index re (2*i+1) (2*p+1);
    lemma_chunks_of_re_avx2_index re_fut (2*i+1) (2*p);
    lemma_chunks_of_re_avx2_index re_fut (2*i+1) (2*p+1)
#pop-options

(* Generic createi-free re-index: even/odd 16-foralls -> 32-forall.  ABSTRACT q
   so no chunkfact/createi term enters this VC (kills the asymmetric odd-branch
   cascade). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let reindex_32_from_16 (q: (b:nat{b<32}) -> (p:nat{p<4}) -> Type0)
    : Lemma (requires (forall (i:nat{i<16}) (p:nat{p<4}). q (2*i) p) /\
                      (forall (i:nat{i<16}) (p:nat{p<4}). q (2*i+1) p))
            (ensures forall (b:nat{b<32}) (p:nat{p<4}). q b p)
  = let aux (b:nat{b<32}) (p:nat{p<4}) : Lemma (q b p) =
      FStar.Math.Lemmas.euclidean_division_definition b 2;
      (if b % 2 = 0 then assert (q (2*(b/2)) p) else assert (q (2*(b/2)+1) p))
    in Classical.forall_intro_2 aux
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunkfacts_from_lift
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires l0_post_sym re re_fut)
            (ensures forall (b:nat{b<32}) (p:nat{p<4}). chunkfact re re_fut b p)
  = lemma_lift2 re re_fut;
    (* even chunks 2i: instantiate body2 at CLEAN i (no b/2 mess) *)
    let auxe (i:nat{i<16}) (p:nat{p<4}) : Lemma (chunkfact re re_fut (2*i) p) =
      lemma_chunkfact_even re re_fut i p
    in Classical.forall_intro_2 auxe;
    (* odd chunks 2i+1 *)
    let auxo (i:nat{i<16}) (p:nat{p<4}) : Lemma (chunkfact re re_fut (2*i+1) p) =
      lemma_chunkfact_odd re re_fut i p
    in Classical.forall_intro_2 auxo;
    reindex_32_from_16 (chunkfact re re_fut)
#pop-options

(* ===== L0 opaque per-chunk FE atom (AVX2 form: t_p = mont_mul (ci[2p+1]) z_p).
   Mirror of Portable's unit_fe_post_l0; opaque so the driver composes 32 of
   them like the bounds post, never expanding 256 facts into the WP. ===== *)
[@@ "opaque_to_smt"]
let unit_post_l0_avx2 (ci co: t_Array i32 (mk_usize 8))
      (zeta0 zeta1 zeta2 zeta3: i32{Spec.Utils.is_i32b 4190208 zeta0 /\ Spec.Utils.is_i32b 4190208 zeta1 /\ Spec.Utils.is_i32b 4190208 zeta2 /\ Spec.Utils.is_i32b 4190208 zeta3}) : Type0 =
  (let t0 = mont_mul (Seq.index ci 1) zeta0 in
   let t1 = mont_mul (Seq.index ci 3) zeta1 in
   let t2 = mont_mul (Seq.index ci 5) zeta2 in
   let t3 = mont_mul (Seq.index ci 7) zeta3 in
   v (Seq.index co 0) == v (Seq.index ci 0) + v t0 /\
   v (Seq.index co 1) == v (Seq.index ci 0) - v t0 /\
   (v t0) % 8380417 == (v (Seq.index ci 1) * v zeta0 * 8265825) % 8380417 /\
   v (Seq.index co 2) == v (Seq.index ci 2) + v t1 /\
   v (Seq.index co 3) == v (Seq.index ci 2) - v t1 /\
   (v t1) % 8380417 == (v (Seq.index ci 3) * v zeta1 * 8265825) % 8380417 /\
   v (Seq.index co 4) == v (Seq.index ci 4) + v t2 /\
   v (Seq.index co 5) == v (Seq.index ci 4) - v t2 /\
   (v t2) % 8380417 == (v (Seq.index ci 5) * v zeta2 * 8265825) % 8380417 /\
   v (Seq.index co 6) == v (Seq.index ci 6) + v t3 /\
   v (Seq.index co 7) == v (Seq.index ci 6) - v t3 /\
   (v t3) % 8380417 == (v (Seq.index ci 7) * v zeta3 * 8265825) % 8380417)

(* Standalone: unfold one L0 opaque atom to the bridge's per-pair forall. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always --z3refresh"
let lemma_atom_to_bf_l0_avx2 (ci co: t_Array i32 (mk_usize 8))
      (zf: (p: nat{p < 4}) -> (z: i32{Spec.Utils.is_i32b 4190208 z}))
    : Lemma (requires unit_post_l0_avx2 ci co (zf 0) (zf 1) (zf 2) (zf 3))
            (ensures
              (forall (p: nat{p < 4}).
                (let t = mont_mul (Seq.index ci (2*p+1)) (zf p) in
                 v (Seq.index co (2*p))   == v (Seq.index ci (2*p)) + v t /\
                 v (Seq.index co (2*p+1)) == v (Seq.index ci (2*p)) - v t /\
                 (v t) % 8380417 == (v (Seq.index ci (2*p+1)) * v (zf p) * 8265825) % 8380417)))
  = reveal_opaque (`%unit_post_l0_avx2) unit_post_l0_avx2;
    introduce forall (p: nat{p < 4}).
        (let t = mont_mul (Seq.index ci (2*p+1)) (zf p) in
         v (Seq.index co (2*p))   == v (Seq.index ci (2*p)) + v t /\
         v (Seq.index co (2*p+1)) == v (Seq.index ci (2*p)) - v t /\
         (v t) % 8380417 == (v (Seq.index ci (2*p+1)) * v (zf p) * 8265825) % 8380417)
    with (match p with | 0 -> () | 1 -> () | 2 -> () | _ -> ())
#pop-options

(* ===== Per-chunk establishment: from the input bound + the 4 chunk ntt_step
   facts, build the opaque atom for chunk b AND the per-lane output bound.
   The genuinely-new AVX2 logic lives in lemma_l0_pair_relations (already
   validated); this just packages 4 pairs into the atom + bound. ===== *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l0_chunk_avx2
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{bnd + 8380416 < pow2 31})
      (b: nat{b < 32})
    : Lemma
        (requires is_i32b_poly_avx2 bnd re /\ (forall (p:nat{p<4}). chunkfact re re_fut b p))
        (ensures
          unit_post_l0_avx2 (Seq.index (chunks_of_re_avx2 re) b) (Seq.index (chunks_of_re_avx2 re_fut) b)
            (mk_i32 (zeta_r (4*b + 0 + 128))) (mk_i32 (zeta_r (4*b + 1 + 128)))
            (mk_i32 (zeta_r (4*b + 2 + 128))) (mk_i32 (zeta_r (4*b + 3 + 128))) /\
          (forall (l:nat). l < 8 ==>
            Spec.Utils.is_i32b (bnd + 8380416) (to_i32x8 (Seq.index re_fut b).f_value (mk_u64 l))))
  = lemma_l0_pair_relations re re_fut bnd b 0;
    lemma_l0_pair_relations re re_fut bnd b 1;
    lemma_l0_pair_relations re re_fut bnd b 2;
    lemma_l0_pair_relations re re_fut bnd b 3;
    reveal_opaque (`%unit_post_l0_avx2) unit_post_l0_avx2;
    introduce forall (l:nat{l<8}).
        Spec.Utils.is_i32b (bnd + 8380416) (to_i32x8 (Seq.index re_fut b).f_value (mk_u64 l))
    with (lemma_chunks_of_re_avx2_index re_fut b l)
#pop-options

(* Generic 1D ground->symbolic forall lift for 32 (mirror of forall16_elim_1d). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 40"
let forall32_elim_1d (r: (b: nat{b < 32}) -> Type0)
    : Lemma (requires Spec.Utils.forall32 r) (ensures forall (b: nat{b < 32}). r b)
  = let aux (b: nat{b < 32}) : Lemma (r b) =
      (match b with
       | 0 -> () | 1 -> () | 2 -> () | 3 -> () | 4 -> () | 5 -> () | 6 -> () | 7 -> ()
       | 8 -> () | 9 -> () | 10 -> () | 11 -> () | 12 -> () | 13 -> () | 14 -> () | 15 -> ()
       | 16 -> () | 17 -> () | 18 -> () | 19 -> () | 20 -> () | 21 -> () | 22 -> () | 23 -> ()
       | 24 -> () | 25 -> () | 26 -> () | 27 -> () | 28 -> () | 29 -> () | 30 -> () | _ -> ())
    in Classical.forall_intro aux
#pop-options

(* ===== Clean-context driver composition for L0 (chunk arrays): from the
   forall32 of opaque atoms, feed the Commute.Chunk poly lemma.  Mirror of
   Portable lemma_l0_driver_compose with mont_mul + the AVX2 atom. ===== *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l0_driver_compose_avx2
      (orig fut: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : Lemma
        (requires
          forall32 (fun b ->
            unit_post_l0_avx2 (Seq.index orig b) (Seq.index fut b)
              (mk_i32 (zeta_r (4*b + 0 + 128))) (mk_i32 (zeta_r (4*b + 1 + 128)))
              (mk_i32 (zeta_r (4*b + 2 + 128))) (mk_i32 (zeta_r (4*b + 3 + 128)))))
        (ensures
          (let in_flat = C.simd_units_to_array orig in
           let out_flat = C.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 0) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (b: nat{b < 32}) (p: nat{p < 4}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (zeta_r (4*b + p + 128)) in
    let t (b: nat{b < 32}) (p: nat{p < 4}) : i32 =
      mont_mul (Seq.index (Seq.index orig b) (2*p+1)) (zm b p) in
    forall32_elim_1d (fun b -> unit_post_l0_avx2 (Seq.index orig b) (Seq.index fut b)
                                 (mk_i32 (zeta_r (4*b + 0 + 128))) (mk_i32 (zeta_r (4*b + 1 + 128)))
                                 (mk_i32 (zeta_r (4*b + 2 + 128))) (mk_i32 (zeta_r (4*b + 3 + 128))));
    (let aux (b: nat{b < 32}) (p: nat{p < 4}) : Lemma
       (let ci = Seq.index orig b in
        let co = Seq.index fut b in
        v (Seq.index co (2*p))   == v (Seq.index ci (2*p)) + v (t b p) /\
        v (Seq.index co (2*p+1)) == v (Seq.index ci (2*p)) - v (t b p) /\
        (v (t b p)) % 8380417 == (v (Seq.index ci (2*p+1)) * v (zm b p) * 8265825) % 8380417 /\
        (v (zm b p)) % 8380417 ==
          (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (4*b + p + 128) ] <: i32) * pow2 32) % 8380417)
      = lemma_atom_to_bf_l0_avx2 (Seq.index orig b) (Seq.index fut b) (fun p -> zm b p);
        reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
        let _ = zeta_r (4*b + p + 128) in
        C.lemma_v_zetas_eq_zeta (4*b + p + 128)
     in Classical.forall_intro_2 aux);
    C.lemma_ntt_layer_0_step_to_hacspec_poly orig fut t zm
#pop-options

(* ===== FULL L0 body glue: from input bound + the symbolic L0 post, derive the
   complete layer-fn post (output bound + functional congruence).  This is what
   the ntt.rs body tail calls (after establishing l0_post_sym from the butterfly
   facts via assert_norm zeta literals). ===== *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l0_full_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{bnd + 8380416 < pow2 31})
    : Lemma
        (requires is_i32b_poly_avx2 bnd orig_re /\ l0_post_sym orig_re re)
        (ensures
          is_i32b_poly_avx2 (bnd + 8380416) re /\
          (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 0) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = lemma_chunkfacts_from_lift orig_re re;
    let aux (b:nat{b<32}) : Lemma
        (unit_post_l0_avx2 (Seq.index (chunks_of_re_avx2 orig_re) b) (Seq.index (chunks_of_re_avx2 re) b)
           (mk_i32 (zeta_r (4*b + 0 + 128))) (mk_i32 (zeta_r (4*b + 1 + 128)))
           (mk_i32 (zeta_r (4*b + 2 + 128))) (mk_i32 (zeta_r (4*b + 3 + 128)))
         /\ (forall (l:nat). l<8 ==>
              Spec.Utils.is_i32b (bnd + 8380416) (to_i32x8 (Seq.index re b).f_value (mk_u64 l))))
      = lemma_l0_chunk_avx2 orig_re re bnd b
    in Classical.forall_intro aux;
    lemma_is_i32b_poly_avx2_intro (bnd + 8380416) re;
    lemma_l0_driver_compose_avx2 (chunks_of_re_avx2 orig_re) (chunks_of_re_avx2 re)
#pop-options

(* ===== Bridge: the body-natural literal-zeta L0 post (l0_post) implies the
   symbolic-zeta form (l0_post_sym) the lift machinery consumes.  128 zeta_r
   literal assert_norms.  This is what the ntt.rs body calls after the
   butterflies establish l0_post. ===== *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_l0post_to_sym (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires l0_post re re_fut) (ensures l0_post_sym re re_fut)
  = 
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 128 == 2091667);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 129 == 3407706);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 130 == 2316500);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 131 == 3817976);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 132 == (- 3342478));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 133 == 2244091);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 134 == (- 2446433));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 135 == (- 3562462));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 136 == 266997);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 137 == 2434439);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 138 == (- 1235728));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 139 == 3513181);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 140 == (- 3520352));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 141 == (- 3759364));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 142 == (- 1197226));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 143 == (- 3193378));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 144 == 900702);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 145 == 1859098);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 146 == 909542);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 147 == 819034);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 148 == 495491);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 149 == (- 1613174));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 150 == (- 43260));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 151 == (- 522500));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 152 == (- 655327));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 153 == (- 3122442));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 154 == 2031748);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 155 == 3207046);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 156 == (- 3556995));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 157 == (- 525098));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 158 == (- 768622));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 159 == (- 3595838));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 160 == 342297);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 161 == 286988);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 162 == (- 2437823));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 163 == 4108315);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 164 == 3437287);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 165 == (- 3342277));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 166 == 1735879);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 167 == 203044);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 168 == 2842341);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 169 == 2691481);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 170 == (- 2590150));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 171 == 1265009);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 172 == 4055324);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 173 == 1247620);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 174 == 2486353);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 175 == 1595974);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 176 == (- 3767016));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 177 == 1250494);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 178 == 2635921);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 179 == (- 3548272));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 180 == (- 2994039));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 181 == 1869119);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 182 == 1903435);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 183 == (- 1050970));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 184 == (- 1333058));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 185 == 1237275);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 186 == (- 3318210));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 187 == (- 1430225));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 188 == (- 451100));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 189 == 1312455);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 190 == 3306115);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 191 == (- 1962642));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 192 == (- 1279661));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 193 == 1917081);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 194 == (- 2546312));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 195 == (- 1374803));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 196 == 1500165);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 197 == 777191);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 198 == 2235880);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 199 == 3406031);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 200 == (- 542412));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 201 == (- 2831860));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 202 == (- 1671176));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 203 == (- 1846953));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 204 == (- 2584293));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 205 == (- 3724270));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 206 == 594136);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 207 == (- 3776993));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 208 == (- 2013608));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 209 == 2432395);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 210 == 2454455);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 211 == (- 164721));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 212 == 1957272);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 213 == 3369112);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 214 == 185531);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 215 == (- 1207385));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 216 == (- 3183426));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 217 == 162844);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 218 == 1616392);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 219 == 3014001);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 220 == 810149);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 221 == 1652634);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 222 == (- 3694233));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 223 == (- 1799107));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 224 == (- 3038916));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 225 == 3523897);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 226 == 3866901);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 227 == 269760);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 228 == 2213111);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 229 == (- 975884));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 230 == 1717735);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 231 == 472078);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 232 == (- 426683));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 233 == 1723600);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 234 == (- 1803090));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 235 == 1910376);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 236 == (- 1667432));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 237 == (- 1104333));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 238 == (- 260646));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 239 == (- 3833893));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 240 == (- 2939036));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 241 == (- 2235985));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 242 == (- 420899));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 243 == (- 2286327));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 244 == 183443);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 245 == (- 976891));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 246 == 1612842);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 247 == (- 3545687));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 248 == (- 554416));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 249 == 3919660);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 250 == (- 48306));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 251 == (- 1362209));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 252 == 3937738);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 253 == 1400424);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 254 == (- 846154));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 255 == 1976782)
#pop-options
(* CRUX: from the per-(b,p) ntt_step post (pair (p,p+4), zeta = zeta_r(b+32))
   + zeta bound + input bound, derive the butterfly relations + output bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_l2_pair_relations
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{bnd + 8380416 < pow2 31})
      (b: nat{b < 32}) (p: nat{p < 4})
    : Lemma
        (requires
          is_i32b_poly_avx2 bnd re /\
          (let ci = chunks_of_re_avx2 re in
           let co = chunks_of_re_avx2 re_fut in
           (Seq.index (Seq.index co b) p, Seq.index (Seq.index co b) (p+4)) ==
             ntt_step (mk_int (zeta_r (b + 32)))
               (Seq.index (Seq.index ci b) p, Seq.index (Seq.index ci b) (p+4))))
        (ensures
          (let ci = chunks_of_re_avx2 re in
           let co = chunks_of_re_avx2 re_fut in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (b + 32) ] in
           let zm : i32 = mk_int (zeta_r (b + 32)) in
           let t : i32 = mont_mul (Seq.index (Seq.index ci b) (p+4)) zm in
           v (Seq.index (Seq.index co b) p)     == v (Seq.index (Seq.index ci b) p) + v t /\
           v (Seq.index (Seq.index co b) (p+4)) == v (Seq.index (Seq.index ci b) p) - v t /\
           (v t) % 8380417 ==
             (v (Seq.index (Seq.index ci b) (p+4)) * v zm * 8265825) % 8380417 /\
           (v zm) % 8380417 == (v z * pow2 32) % 8380417 /\
           Spec.Utils.is_i32b (bnd + 8380416) (Seq.index (Seq.index co b) p) /\
           Spec.Utils.is_i32b (bnd + 8380416) (Seq.index (Seq.index co b) (p+4))))
  = let ci = chunks_of_re_avx2 re in
    let co = chunks_of_re_avx2 re_fut in
    let ci_lo = Seq.index (Seq.index ci b) p in
    let ci_hi = Seq.index (Seq.index ci b) (p+4) in
    let co_lo = Seq.index (Seq.index co b) p in
    let co_hi = Seq.index (Seq.index co b) (p+4) in
    let zm : i32 = mk_int (zeta_r (b + 32)) in
    let t : i32 = mont_mul ci_hi zm in
    assert (co_lo == add_mod_opaque ci_lo t);
    assert (co_hi == sub_mod_opaque ci_lo t);
    lemma_chunks_of_re_avx2_index re b p;
    lemma_is_i32b_poly_avx2_elim bnd re b p;
    assert (Spec.Utils.is_i32b bnd ci_lo);
    assert (Spec.Utils.is_i32b 8380416 zm);
    C.lemma_mont_mul_bound_and_mod_q ci_hi zm;
    assert (Spec.Utils.is_i32b 8380416 t);
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    assert (v co_lo == v ci_lo + v t);
    assert (v co_hi == v ci_lo - v t);
    let idx : nat = b + 32 in
    C.lemma_v_zetas_eq_zeta idx
#pop-options

(* Verbatim literal-zeta L2 post (matches ntt_at_layer_2_'s ensures). *)
unfold let l2_post (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  norm [
      primops; iota;
      delta_namespace [`%zeta_r; `%Spec.Utils.forall4; `%Spec.Utils.forall16]
    ]
    (Spec.Utils.forall16 (fun i ->
          let nre = re_fut in
          let re0 = Seq.index re (i * 2) in
          let re1 = Seq.index re (i * 2 + 1) in
          let nre0 = Seq.index nre (i * 2) in
          let nre1 = Seq.index nre (i * 2 + 1) in
          Spec.Utils.forall4 (fun j ->
                let zeta0 = zeta_r (32 + i * 2) in
                let zeta1 = zeta_r (32 + i * 2 + 1) in
                let j0 = j in
                let j1 = j0 + 4 in
                (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
                ntt_step (mk_int zeta0)
                  (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
                (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
                ntt_step (mk_int zeta1)
                  (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))))

unfold let chunkfact_l2 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                        (b:nat{b<32}) (p:nat{p<4}) : Type0 =
  let ci = chunks_of_re_avx2 re in
  let co = chunks_of_re_avx2 re_fut in
  (Seq.index (Seq.index co b) p, Seq.index (Seq.index co b) (p+4)) ==
    ntt_step (mk_int (zeta_r (b + 32)))
      (Seq.index (Seq.index ci b) p, Seq.index (Seq.index ci b) (p+4))

(* Symbolic-zeta L2 post (zeta_r NOT norm-evaluated). *)
unfold let l2_post_sym (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  norm [
      primops; iota;
      delta_namespace [`%Spec.Utils.forall4; `%Spec.Utils.forall16]
    ]
    (Spec.Utils.forall16 (fun i ->
          let nre = re_fut in
          let re0 = Seq.index re (i * 2) in
          let re1 = Seq.index re (i * 2 + 1) in
          let nre0 = Seq.index nre (i * 2) in
          let nre1 = Seq.index nre (i * 2 + 1) in
          Spec.Utils.forall4 (fun j ->
                let zeta0 = zeta_r (32 + i * 2) in
                let zeta1 = zeta_r (32 + i * 2 + 1) in
                let j0 = j in
                let j1 = j0 + 4 in
                (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
                ntt_step (mk_int zeta0)
                  (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
                (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
                ntt_step (mk_int zeta1)
                  (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))))

unfold let l2_body (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                   (i:nat{i<16}) : Type0 =
  let re0 = Seq.index re (i*2) in
  let re1 = Seq.index re (i*2+1) in
  let nre0 = Seq.index re_fut (i*2) in
  let nre1 = Seq.index re_fut (i*2+1) in
  Spec.Utils.forall4 (fun j ->
        let zeta0 = zeta_r (32 + i*2) in
        let zeta1 = zeta_r (32 + i*2 + 1) in
        let j0 = j in let j1 = j0+4 in
        (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
          ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
        (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
          ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))

#push-options "--fuel 0 --ifuel 1 --z3rlimit 60"
let lemma_lift_l2 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires l2_post_sym re re_fut)
            (ensures forall (i:nat{i<16}). l2_body re re_fut i)
  = forall16_elim_1d (l2_body re re_fut)
#pop-options

unfold let body2_l2 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                    (i:nat{i<16}) (j:nat{j<4}) : Type0 =
  let re0 = Seq.index re (i*2) in
  let re1 = Seq.index re (i*2+1) in
  let nre0 = Seq.index re_fut (i*2) in
  let nre1 = Seq.index re_fut (i*2+1) in
  let zeta0 = zeta_r (32 + i*2) in
  let zeta1 = zeta_r (32 + i*2 + 1) in
  let j0 = j in let j1 = j0+4 in
  (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
    ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
  (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
    ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1))

#push-options "--fuel 0 --ifuel 1 --z3rlimit 80"
let lemma_lift2_l2 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires l2_post_sym re re_fut)
            (ensures forall (i:nat{i<16}) (j:nat{j<4}). body2_l2 re re_fut i j)
  = lemma_lift_l2 re re_fut;
    let aux (i:nat{i<16}) : Lemma (forall (j:nat{j<4}). body2_l2 re re_fut i j) =
      forall4_elim_1d (fun (j:nat{j<4}) -> body2_l2 re re_fut i j)
    in Classical.forall_intro aux
#pop-options

(* Even chunk b=2i: chunkfact_l2 (2i) p from body2_l2 i p's nre0 part. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunkfact_l2_even
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:nat{i<16}) (p:nat{p<4})
    : Lemma (requires body2_l2 re re_fut i p) (ensures chunkfact_l2 re re_fut (2*i) p)
  = lemma_chunks_of_re_avx2_index re (2*i) p;
    lemma_chunks_of_re_avx2_index re (2*i) (p+4);
    lemma_chunks_of_re_avx2_index re_fut (2*i) p;
    lemma_chunks_of_re_avx2_index re_fut (2*i) (p+4)
#pop-options

(* Odd chunk b=2i+1: chunkfact_l2 (2i+1) p from body2_l2 i p's nre1 part. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunkfact_l2_odd
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:nat{i<16}) (p:nat{p<4})
    : Lemma (requires body2_l2 re re_fut i p) (ensures chunkfact_l2 re re_fut (2*i+1) p)
  = lemma_chunks_of_re_avx2_index re (2*i+1) p;
    lemma_chunks_of_re_avx2_index re (2*i+1) (p+4);
    lemma_chunks_of_re_avx2_index re_fut (2*i+1) p;
    lemma_chunks_of_re_avx2_index re_fut (2*i+1) (p+4)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunkfacts_from_lift_l2
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires l2_post_sym re re_fut)
            (ensures forall (b:nat{b<32}) (p:nat{p<4}). chunkfact_l2 re re_fut b p)
  = lemma_lift2_l2 re re_fut;
    let auxe (i:nat{i<16}) (p:nat{p<4}) : Lemma (chunkfact_l2 re re_fut (2*i) p) =
      lemma_chunkfact_l2_even re re_fut i p
    in Classical.forall_intro_2 auxe;
    let auxo (i:nat{i<16}) (p:nat{p<4}) : Lemma (chunkfact_l2 re re_fut (2*i+1) p) =
      lemma_chunkfact_l2_odd re re_fut i p
    in Classical.forall_intro_2 auxo;
    reindex_32_from_16 (chunkfact_l2 re re_fut)
#pop-options

(* L2 opaque per-chunk FE atom: ONE zeta, pairs (p,p+4), t_p = mont_mul (ci[p+4]) zeta. *)
[@@ "opaque_to_smt"]
let unit_post_l2_avx2 (ci co: t_Array i32 (mk_usize 8))
      (zeta: i32{Spec.Utils.is_i32b 4190208 zeta}) : Type0 =
  (let t0 = mont_mul (Seq.index ci 4) zeta in
   let t1 = mont_mul (Seq.index ci 5) zeta in
   let t2 = mont_mul (Seq.index ci 6) zeta in
   let t3 = mont_mul (Seq.index ci 7) zeta in
   v (Seq.index co 0) == v (Seq.index ci 0) + v t0 /\
   v (Seq.index co 4) == v (Seq.index ci 0) - v t0 /\
   (v t0) % 8380417 == (v (Seq.index ci 4) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co 1) == v (Seq.index ci 1) + v t1 /\
   v (Seq.index co 5) == v (Seq.index ci 1) - v t1 /\
   (v t1) % 8380417 == (v (Seq.index ci 5) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co 2) == v (Seq.index ci 2) + v t2 /\
   v (Seq.index co 6) == v (Seq.index ci 2) - v t2 /\
   (v t2) % 8380417 == (v (Seq.index ci 6) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co 3) == v (Seq.index ci 3) + v t3 /\
   v (Seq.index co 7) == v (Seq.index ci 3) - v t3 /\
   (v t3) % 8380417 == (v (Seq.index ci 7) * v zeta * 8265825) % 8380417)

(* Standalone: unfold one L2 opaque atom to the bridge's per-pair forall. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always --z3refresh"
let lemma_atom_to_bf_l2_avx2 (ci co: t_Array i32 (mk_usize 8))
      (zeta: i32{Spec.Utils.is_i32b 4190208 zeta})
    : Lemma (requires unit_post_l2_avx2 ci co zeta)
            (ensures
              (forall (p: nat{p < 4}).
                (let t = mont_mul (Seq.index ci (p+4)) zeta in
                 v (Seq.index co p)     == v (Seq.index ci p) + v t /\
                 v (Seq.index co (p+4)) == v (Seq.index ci p) - v t /\
                 (v t) % 8380417 == (v (Seq.index ci (p+4)) * v zeta * 8265825) % 8380417)))
  = reveal_opaque (`%unit_post_l2_avx2) unit_post_l2_avx2;
    introduce forall (p: nat{p < 4}).
        (let t = mont_mul (Seq.index ci (p+4)) zeta in
         v (Seq.index co p)     == v (Seq.index ci p) + v t /\
         v (Seq.index co (p+4)) == v (Seq.index ci p) - v t /\
         (v t) % 8380417 == (v (Seq.index ci (p+4)) * v zeta * 8265825) % 8380417)
    with (match p with | 0 -> () | 1 -> () | 2 -> () | _ -> ())
#pop-options

(* Per-chunk establishment: from input bound + 4 chunk ntt_step facts, build the
   opaque atom for chunk b AND the per-lane output bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l2_chunk_avx2
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{bnd + 8380416 < pow2 31})
      (b: nat{b < 32})
    : Lemma
        (requires is_i32b_poly_avx2 bnd re /\ (forall (p:nat{p<4}). chunkfact_l2 re re_fut b p))
        (ensures
          unit_post_l2_avx2 (Seq.index (chunks_of_re_avx2 re) b) (Seq.index (chunks_of_re_avx2 re_fut) b)
            (mk_i32 (zeta_r (b + 32))) /\
          (forall (l:nat). l < 8 ==>
            Spec.Utils.is_i32b (bnd + 8380416) (to_i32x8 (Seq.index re_fut b).f_value (mk_u64 l))))
  = let h : squash (forall (p:nat{p<4}). chunkfact_l2 re re_fut b p) = () in
    eliminate forall (p:nat{p<4}). chunkfact_l2 re re_fut b p with 0;
    eliminate forall (p:nat{p<4}). chunkfact_l2 re re_fut b p with 1;
    eliminate forall (p:nat{p<4}). chunkfact_l2 re re_fut b p with 2;
    eliminate forall (p:nat{p<4}). chunkfact_l2 re re_fut b p with 3;
    lemma_l2_pair_relations re re_fut bnd b 0;
    lemma_l2_pair_relations re re_fut bnd b 1;
    lemma_l2_pair_relations re re_fut bnd b 2;
    lemma_l2_pair_relations re re_fut bnd b 3;
    reveal_opaque (`%unit_post_l2_avx2) unit_post_l2_avx2;
    introduce forall (l:nat{l<8}).
        Spec.Utils.is_i32b (bnd + 8380416) (to_i32x8 (Seq.index re_fut b).f_value (mk_u64 l))
    with (lemma_chunks_of_re_avx2_index re_fut b l)
#pop-options

(* Clean-context driver composition for L2: from the forall32 of opaque atoms,
   feed the Commute.Chunk poly lemma (ntt_layer mk_usize 2). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l2_driver_compose_avx2
      (orig fut: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : Lemma
        (requires
          forall32 (fun b ->
            unit_post_l2_avx2 (Seq.index orig b) (Seq.index fut b)
              (mk_i32 (zeta_r (b + 32)))))
        (ensures
          (let in_flat = C.simd_units_to_array orig in
           let out_flat = C.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 2) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (b: nat{b < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (zeta_r (b + 32)) in
    let t (b: nat{b < 32}) (p: nat{p < 4}) : i32 =
      mont_mul (Seq.index (Seq.index orig b) (p+4)) (zm b) in
    forall32_elim_1d (fun b -> unit_post_l2_avx2 (Seq.index orig b) (Seq.index fut b)
                                 (mk_i32 (zeta_r (b + 32))));
    (let aux_bf (b: nat{b < 32}) : Lemma
       (forall (p: nat{p < 4}).
         (let ci = Seq.index orig b in
          let co = Seq.index fut b in
          v (Seq.index co p)     == v (Seq.index ci p) + v (t b p) /\
          v (Seq.index co (p+4)) == v (Seq.index ci p) - v (t b p) /\
          (v (t b p)) % 8380417 == (v (Seq.index ci (p+4)) * v (zm b) * 8265825) % 8380417))
      = lemma_atom_to_bf_l2_avx2 (Seq.index orig b) (Seq.index fut b) (zm b)
     in Classical.forall_intro aux_bf);
    (let aux_z (b: nat{b < 32}) : Lemma
       ((v (zm b)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (b + 32) ] <: i32) * pow2 32) % 8380417)
      = reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
        let _ = zeta_r (b + 32) in
        C.lemma_v_zetas_eq_zeta (b + 32)
     in Classical.forall_intro aux_z);
    C.lemma_ntt_layer_2_step_to_hacspec_poly orig fut t zm
#pop-options

(* FULL L2 body glue: from input bound + symbolic L2 post, derive output bound +
   functional congruence. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l2_full_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{bnd + 8380416 < pow2 31})
    : Lemma
        (requires is_i32b_poly_avx2 bnd orig_re /\ l2_post_sym orig_re re)
        (ensures
          is_i32b_poly_avx2 (bnd + 8380416) re /\
          (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 2) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = lemma_chunkfacts_from_lift_l2 orig_re re;
    let aux (b:nat{b<32}) : Lemma
        (unit_post_l2_avx2 (Seq.index (chunks_of_re_avx2 orig_re) b) (Seq.index (chunks_of_re_avx2 re) b)
           (mk_i32 (zeta_r (b + 32)))
         /\ (forall (l:nat). l<8 ==>
              Spec.Utils.is_i32b (bnd + 8380416) (to_i32x8 (Seq.index re b).f_value (mk_u64 l))))
      = lemma_l2_chunk_avx2 orig_re re bnd b
    in Classical.forall_intro aux;
    lemma_is_i32b_poly_avx2_intro (bnd + 8380416) re;
    lemma_l2_driver_compose_avx2 (chunks_of_re_avx2 orig_re) (chunks_of_re_avx2 re)
#pop-options

(* Bridge: literal-zeta L2 post (l2_post) implies symbolic-zeta form (l2_post_sym).
   32 zeta_r literal assert_norms (idx 32..63, all of zeta_r(32+i*2)/zeta_r(32+i*2+1)). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_l2post_to_sym (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires l2_post re re_fut) (ensures l2_post_sym re re_fut)
  =
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 32 == 2706023);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 33 == 95776);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 34 == 3077325);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 35 == 3530437);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 36 == (- 1661693));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 37 == (- 3592148));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 38 == (- 2537516));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 39 == 3915439);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 40 == (- 3861115));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 41 == (- 3043716));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 42 == 3574422);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 43 == (- 2867647));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 44 == 3539968);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 45 == (- 300467));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 46 == 2348700);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 47 == (- 539299));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 48 == (- 1699267));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 49 == (- 1643818));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 50 == 3505694);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 51 == (- 3821735));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 52 == 3507263);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 53 == (- 2140649));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 54 == (- 1600420));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 55 == 3699596);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 56 == 811944);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 57 == 531354);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 58 == 954230);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 59 == 3881043);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 60 == 3900724);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 61 == (- 2556880));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 62 == 2071892);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 63 == (- 2797779))
#pop-options
(* ===================================================================== *)
(* ============================ LAYER 1 ================================= *)
(* ===================================================================== *)
(* L1 (len=2, within-chunk): per chunk b, TWO zetas (one per half h in {0,1}).
   The 4 butterflies act on lane pairs indexed by p in 0..3 with
   h = p/2, j' = p%2, lanes (4h+j', 4h+j'+2).  Zeta for half h of chunk b is
   zeta_r (2*b + h + 64).  The Commute bridge consumes the (b,h,j') indexing;
   we collapse (h,j') to p (p/2=h, p%2=j') so reindex_32_from_16 applies. *)

(* The post's j0-match equals the clean formula 4*(j/2)+(j%2). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 30"
let lemma_j0_l1 (j:nat{j<4})
    : Lemma ((match j with | 0 -> 0 | 1 -> 1 | 2 -> 4 | _ -> 5) == 4*(j/2)+(j%2))
  = (match j with | 0 -> () | 1 -> () | 2 -> () | _ -> ())
#pop-options

(* CRUX (genuinely-new AVX2 logic for L1): from the per-(b,p) ntt_step post +
   zeta bound + input bound, derive the butterfly relations the L1 bridge
   consumes.  Lane pair (4h+j', 4h+j'+2) with h=p/2,j'=p%2; zeta_r(2b+h+64). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_l1_pair_relations
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{bnd + 8380416 < pow2 31})
      (b: nat{b < 32}) (p: nat{p < 4})
    : Lemma
        (requires
          is_i32b_poly_avx2 bnd re /\
          (let ci = chunks_of_re_avx2 re in
           let co = chunks_of_re_avx2 re_fut in
           let lo : nat = 4*(p/2)+(p%2) in
           (Seq.index (Seq.index co b) lo, Seq.index (Seq.index co b) (lo+2)) ==
             ntt_step (mk_int (zeta_r (2*b + p/2 + 64)))
               (Seq.index (Seq.index ci b) lo, Seq.index (Seq.index ci b) (lo+2))))
        (ensures
          (let ci = chunks_of_re_avx2 re in
           let co = chunks_of_re_avx2 re_fut in
           let lo : nat = 4*(p/2)+(p%2) in
           let z : i32 = Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (2*b + p/2 + 64) ] in
           let zm : i32 = mk_int (zeta_r (2*b + p/2 + 64)) in
           let t : i32 = mont_mul (Seq.index (Seq.index ci b) (lo+2)) zm in
           v (Seq.index (Seq.index co b) lo)     == v (Seq.index (Seq.index ci b) lo) + v t /\
           v (Seq.index (Seq.index co b) (lo+2)) == v (Seq.index (Seq.index ci b) lo) - v t /\
           (v t) % 8380417 ==
             (v (Seq.index (Seq.index ci b) (lo+2)) * v zm * 8265825) % 8380417 /\
           (v zm) % 8380417 == (v z * pow2 32) % 8380417 /\
           Spec.Utils.is_i32b (bnd + 8380416) (Seq.index (Seq.index co b) lo) /\
           Spec.Utils.is_i32b (bnd + 8380416) (Seq.index (Seq.index co b) (lo+2))))
  = let ci = chunks_of_re_avx2 re in
    let co = chunks_of_re_avx2 re_fut in
    let lo : nat = 4*(p/2)+(p%2) in
    assert (lo < 6);
    let ci_lo = Seq.index (Seq.index ci b) lo in
    let ci_hi = Seq.index (Seq.index ci b) (lo+2) in
    let co_lo = Seq.index (Seq.index co b) lo in
    let co_hi = Seq.index (Seq.index co b) (lo+2) in
    let zm : i32 = mk_int (zeta_r (2*b + p/2 + 64)) in
    let t : i32 = mont_mul ci_hi zm in
    // ntt_step unfolds (non-opaque):
    assert (co_lo == add_mod_opaque ci_lo t);
    assert (co_hi == sub_mod_opaque ci_lo t);
    // input bound on ci_lo (via the opaque-predicate elim)
    lemma_chunks_of_re_avx2_index re b lo;
    lemma_is_i32b_poly_avx2_elim bnd re b lo;
    assert (Spec.Utils.is_i32b bnd ci_lo);
    // mont bound + mod-q (zeta_r bounded by 4190208 < FIELD_MAX)
    assert (Spec.Utils.is_i32b 8380416 zm);
    C.lemma_mont_mul_bound_and_mod_q ci_hi zm;
    assert (Spec.Utils.is_i32b 8380416 t);
    // add/sub exactness (no overflow)
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    assert (v co_lo == v ci_lo + v t);
    assert (v co_hi == v ci_lo - v t);
    // zeta canonicalization
    let idx : nat = 2*b + p/2 + 64 in
    C.lemma_v_zetas_eq_zeta idx
#pop-options

(* L1 verbatim post (literal-zeta, matches ntt_at_layer_1_'s ensures). *)
unfold let l1_post (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  norm [
      primops; iota;
      delta_namespace [`%zeta_r; `%Spec.Utils.forall4; `%Spec.Utils.forall16]
    ]
    (Spec.Utils.forall16 (fun i ->
          let nre = re_fut in
          let re0 = Seq.index re (i * 2) in
          let re1 = Seq.index re (i * 2 + 1) in
          let nre0 = Seq.index nre (i * 2) in
          let nre1 = Seq.index nre (i * 2 + 1) in
          Spec.Utils.forall4 (fun j ->
                let zeta0 = zeta_r (64 + i * 4 + j / 2) in
                let zeta1 = zeta_r (64 + i * 4 + j / 2 + 2) in
                let j0 = (match j with | 0 -> 0 | 1 -> 1 | 2 -> 4 | _ -> 5) in
                let j1 = j0 + 2 in
                (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
                ntt_step (mk_int zeta0)
                  (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
                (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
                ntt_step (mk_int zeta1)
                  (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))))

(* Per-(b,p) chunk ntt_step fact: lane pair (lo, lo+2), zeta_r(2b+p/2+64). *)
unfold let chunkfact_l1 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                        (b:nat{b<32}) (p:nat{p<4}) : Type0 =
  let ci = chunks_of_re_avx2 re in
  let co = chunks_of_re_avx2 re_fut in
  let lo : nat = 4*(p/2)+(p%2) in
  (Seq.index (Seq.index co b) lo, Seq.index (Seq.index co b) (lo+2)) ==
    ntt_step (mk_int (zeta_r (2*b + p/2 + 64)))
      (Seq.index (Seq.index ci b) lo, Seq.index (Seq.index ci b) (lo+2))

(* Symbolic L1 post (zeta_r NOT norm-evaluated). *)
unfold let l1_post_sym (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)) : Type0 =
  norm [
      primops; iota;
      delta_namespace [`%Spec.Utils.forall4; `%Spec.Utils.forall16]
    ]
    (Spec.Utils.forall16 (fun i ->
          let nre = re_fut in
          let re0 = Seq.index re (i * 2) in
          let re1 = Seq.index re (i * 2 + 1) in
          let nre0 = Seq.index nre (i * 2) in
          let nre1 = Seq.index nre (i * 2 + 1) in
          Spec.Utils.forall4 (fun j ->
                let zeta0 = zeta_r (64 + i * 4 + j / 2) in
                let zeta1 = zeta_r (64 + i * 4 + j / 2 + 2) in
                let j0 = (match j with | 0 -> 0 | 1 -> 1 | 2 -> 4 | _ -> 5) in
                let j1 = j0 + 2 in
                (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
                ntt_step (mk_int zeta0)
                  (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
                (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
                ntt_step (mk_int zeta1)
                  (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))))

unfold let l1_body (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                   (i:nat{i<16}) : Type0 =
  let re0 = Seq.index re (i*2) in
  let re1 = Seq.index re (i*2+1) in
  let nre0 = Seq.index re_fut (i*2) in
  let nre1 = Seq.index re_fut (i*2+1) in
  Spec.Utils.forall4 (fun j ->
        let zeta0 = zeta_r (64 + i*4 + j/2) in
        let zeta1 = zeta_r (64 + i*4 + j/2 + 2) in
        let j0 = (match j with | 0 -> 0 | 1 -> 1 | 2 -> 4 | _ -> 5) in let j1 = j0+2 in
        (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
          ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
        (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
          ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1)))

(* l1_post_sym ==> forall i<16. l1_body i  (reuse shared forall16_elim_1d). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 60"
let lemma_lift_l1 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires l1_post_sym re re_fut)
            (ensures forall (i:nat{i<16}). l1_body re re_fut i)
  = forall16_elim_1d (l1_body re re_fut)
#pop-options

unfold let body2_l1 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                    (i:nat{i<16}) (j:nat{j<4}) : Type0 =
  let re0 = Seq.index re (i*2) in
  let re1 = Seq.index re (i*2+1) in
  let nre0 = Seq.index re_fut (i*2) in
  let nre1 = Seq.index re_fut (i*2+1) in
  let zeta0 = zeta_r (64 + i*4 + j/2) in
  let zeta1 = zeta_r (64 + i*4 + j/2 + 2) in
  let j0 = (match j with | 0 -> 0 | 1 -> 1 | 2 -> 4 | _ -> 5) in let j1 = j0+2 in
  (to_i32x8 nre0.f_value (mk_u64 j0), to_i32x8 nre0.f_value (mk_u64 j1)) ==
    ntt_step (mk_int zeta0) (to_i32x8 re0.f_value (mk_u64 j0), to_i32x8 re0.f_value (mk_u64 j1)) /\
  (to_i32x8 nre1.f_value (mk_u64 j0), to_i32x8 nre1.f_value (mk_u64 j1)) ==
    ntt_step (mk_int zeta1) (to_i32x8 re1.f_value (mk_u64 j0), to_i32x8 re1.f_value (mk_u64 j1))

(* l1_body i is definitionally forall4 (fun j -> body2_l1 i j); lift to forall i j. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 80"
let lemma_lift2_l1 (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires l1_post_sym re re_fut)
            (ensures forall (i:nat{i<16}) (j:nat{j<4}). body2_l1 re re_fut i j)
  = lemma_lift_l1 re re_fut;
    let aux (i:nat{i<16}) : Lemma (forall (j:nat{j<4}). body2_l1 re re_fut i j) =
      forall4_elim_1d (fun (j:nat{j<4}) -> body2_l1 re re_fut i j)
    in Classical.forall_intro aux
#pop-options

(* Even chunk b=2i: chunkfact_l1 (2i) p from body2_l1 i p's nre0 part.
   The post's j0 = 4*(p/2)+(p%2) = lo; zeta_r(64+4i+p/2) = zeta_r(2*(2i)+p/2+64). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunkfact_l1_even
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:nat{i<16}) (p:nat{p<4})
    : Lemma (requires body2_l1 re re_fut i p) (ensures chunkfact_l1 re re_fut (2*i) p)
  = lemma_j0_l1 p;
    let lo : nat = 4*(p/2)+(p%2) in
    lemma_chunks_of_re_avx2_index re (2*i) lo;
    lemma_chunks_of_re_avx2_index re (2*i) (lo+2);
    lemma_chunks_of_re_avx2_index re_fut (2*i) lo;
    lemma_chunks_of_re_avx2_index re_fut (2*i) (lo+2)
#pop-options

(* Odd chunk b=2i+1: chunkfact_l1 (2i+1) p from body2_l1 i p's nre1 part.
   zeta_r(64+4i+p/2+2) = zeta_r(2*(2i+1)+p/2+64). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunkfact_l1_odd
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (i:nat{i<16}) (p:nat{p<4})
    : Lemma (requires body2_l1 re re_fut i p) (ensures chunkfact_l1 re re_fut (2*i+1) p)
  = lemma_j0_l1 p;
    let lo : nat = 4*(p/2)+(p%2) in
    lemma_chunks_of_re_avx2_index re (2*i+1) lo;
    lemma_chunks_of_re_avx2_index re (2*i+1) (lo+2);
    lemma_chunks_of_re_avx2_index re_fut (2*i+1) lo;
    lemma_chunks_of_re_avx2_index re_fut (2*i+1) (lo+2)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunkfacts_from_lift_l1
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires l1_post_sym re re_fut)
            (ensures forall (b:nat{b<32}) (p:nat{p<4}). chunkfact_l1 re re_fut b p)
  = lemma_lift2_l1 re re_fut;
    let auxe (i:nat{i<16}) (p:nat{p<4}) : Lemma (chunkfact_l1 re re_fut (2*i) p) =
      lemma_chunkfact_l1_even re re_fut i p
    in Classical.forall_intro_2 auxe;
    let auxo (i:nat{i<16}) (p:nat{p<4}) : Lemma (chunkfact_l1 re re_fut (2*i+1) p) =
      lemma_chunkfact_l1_odd re re_fut i p
    in Classical.forall_intro_2 auxo;
    reindex_32_from_16 (chunkfact_l1 re re_fut)
#pop-options

(* L1 opaque per-chunk FE atom: TWO zetas (one per half h), pairs (4h+j',4h+j'+2).
   Mirror of Portable unit_fe_post_l1 with mont_mul.  Lane layout:
   half 0 -> pairs (0,2),(1,3) with zeta0; half 1 -> pairs (4,6),(5,7) with zeta1. *)
[@@ "opaque_to_smt"]
let unit_post_l1_avx2 (ci co: t_Array i32 (mk_usize 8))
      (zeta0 zeta1: i32{Spec.Utils.is_i32b 4190208 zeta0 /\ Spec.Utils.is_i32b 4190208 zeta1}) : Type0 =
  (let t00 = mont_mul (Seq.index ci 2) zeta0 in
   let t01 = mont_mul (Seq.index ci 3) zeta0 in
   let t10 = mont_mul (Seq.index ci 6) zeta1 in
   let t11 = mont_mul (Seq.index ci 7) zeta1 in
   v (Seq.index co 0) == v (Seq.index ci 0) + v t00 /\
   v (Seq.index co 2) == v (Seq.index ci 0) - v t00 /\
   (v t00) % 8380417 == (v (Seq.index ci 2) * v zeta0 * 8265825) % 8380417 /\
   v (Seq.index co 1) == v (Seq.index ci 1) + v t01 /\
   v (Seq.index co 3) == v (Seq.index ci 1) - v t01 /\
   (v t01) % 8380417 == (v (Seq.index ci 3) * v zeta0 * 8265825) % 8380417 /\
   v (Seq.index co 4) == v (Seq.index ci 4) + v t10 /\
   v (Seq.index co 6) == v (Seq.index ci 4) - v t10 /\
   (v t10) % 8380417 == (v (Seq.index ci 6) * v zeta1 * 8265825) % 8380417 /\
   v (Seq.index co 5) == v (Seq.index ci 5) + v t11 /\
   v (Seq.index co 7) == v (Seq.index ci 5) - v t11 /\
   (v t11) % 8380417 == (v (Seq.index ci 7) * v zeta1 * 8265825) % 8380417)

(* Standalone: unfold one L1 opaque atom to the bridge's per-(h,j) forall. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always --z3refresh"
let lemma_atom_to_bf_l1_avx2 (ci co: t_Array i32 (mk_usize 8))
      (zf: (h: nat{h < 2}) -> (z: i32{Spec.Utils.is_i32b 4190208 z}))
    : Lemma (requires unit_post_l1_avx2 ci co (zf 0) (zf 1))
            (ensures
              (forall (h: nat{h < 2}) (j: nat{j < 2}).
                (let t = mont_mul (Seq.index ci (4*h+j+2)) (zf h) in
                 v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v t /\
                 v (Seq.index co (4*h+j+2)) == v (Seq.index ci (4*h+j)) - v t /\
                 (v t) % 8380417 == (v (Seq.index ci (4*h+j+2)) * v (zf h) * 8265825) % 8380417)))
  = reveal_opaque (`%unit_post_l1_avx2) unit_post_l1_avx2;
    introduce forall (h: nat{h < 2}) (j: nat{j < 2}).
        (let t = mont_mul (Seq.index ci (4*h+j+2)) (zf h) in
         v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v t /\
         v (Seq.index co (4*h+j+2)) == v (Seq.index ci (4*h+j)) - v t /\
         (v t) % 8380417 == (v (Seq.index ci (4*h+j+2)) * v (zf h) * 8265825) % 8380417)
    with (match h with | 0 -> (match j with | 0 -> () | _ -> ()) | _ -> (match j with | 0 -> () | _ -> ()))
#pop-options

(* Generic createi-free dispatch: plain forall over p<4 -> 4 ground facts.
   ABSTRACT q so no chunkfact/createi term enters this VC (the chunkfact_l1
   body's p/2,p%2 lane formula is a poor SMT trigger; this forces the 4
   instances by ground match). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 40"
let forall4_inst (q: (p:nat{p<4}) -> Type0)
    : Lemma (requires forall (p:nat{p<4}). q p)
            (ensures q 0 /\ q 1 /\ q 2 /\ q 3)
  = ()
#pop-options

(* Per-chunk establishment: from input bound + 4 chunk ntt_step facts, build the
   L1 opaque atom for chunk b AND the per-lane output bound.  Maps the (b,p)
   chunkfacts (p in 0..3) to the atom's (h,j') lane pairs. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l1_chunk_avx2
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{bnd + 8380416 < pow2 31})
      (b: nat{b < 32})
    : Lemma
        (requires is_i32b_poly_avx2 bnd re /\ (forall (p:nat{p<4}). chunkfact_l1 re re_fut b p))
        (ensures
          unit_post_l1_avx2 (Seq.index (chunks_of_re_avx2 re) b) (Seq.index (chunks_of_re_avx2 re_fut) b)
            (mk_i32 (zeta_r (2*b + 0 + 64))) (mk_i32 (zeta_r (2*b + 1 + 64))) /\
          (forall (l:nat). l < 8 ==>
            Spec.Utils.is_i32b (bnd + 8380416) (to_i32x8 (Seq.index re_fut b).f_value (mk_u64 l))))
  = // materialize the chunkfact hypothesis at each literal p (createi-free dispatch)
    forall4_inst (chunkfact_l1 re re_fut b);
    lemma_l1_pair_relations re re_fut bnd b 0;
    lemma_l1_pair_relations re re_fut bnd b 1;
    lemma_l1_pair_relations re re_fut bnd b 2;
    lemma_l1_pair_relations re re_fut bnd b 3;
    reveal_opaque (`%unit_post_l1_avx2) unit_post_l1_avx2;
    introduce forall (l:nat{l<8}).
        Spec.Utils.is_i32b (bnd + 8380416) (to_i32x8 (Seq.index re_fut b).f_value (mk_u64 l))
    with (lemma_chunks_of_re_avx2_index re_fut b l)
#pop-options

(* Clean-context driver composition for L1: from forall32 of opaque atoms, feed
   the Commute.Chunk L1 poly lemma.  Mirror of Portable lemma_l1_driver_compose
   with mont_mul + the AVX2 atom + the SEPARATE zeta-cong forall. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l1_driver_compose_avx2
      (orig fut: t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32))
    : Lemma
        (requires
          forall32 (fun b ->
            unit_post_l1_avx2 (Seq.index orig b) (Seq.index fut b)
              (mk_i32 (zeta_r (2*b + 0 + 64))) (mk_i32 (zeta_r (2*b + 1 + 64)))))
        (ensures
          (let in_flat = C.simd_units_to_array orig in
           let out_flat = C.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 1) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (b: nat{b < 32}) (h: nat{h < 2}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (zeta_r (2*b + h + 64)) in
    let t (b: nat{b < 32}) (h: nat{h < 2}) (j: nat{j < 2}) : i32 =
      mont_mul (Seq.index (Seq.index orig b) (4*h+j+2)) (zm b h) in
    forall32_elim_1d (fun b -> unit_post_l1_avx2 (Seq.index orig b) (Seq.index fut b)
                                 (mk_i32 (zeta_r (2*b + 0 + 64))) (mk_i32 (zeta_r (2*b + 1 + 64))));
    (let aux_bf (b: nat{b < 32}) : Lemma
       (forall (h: nat{h < 2}) (j: nat{j < 2}).
         (let ci = Seq.index orig b in
          let co = Seq.index fut b in
          v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v (t b h j) /\
          v (Seq.index co (4*h+j+2)) == v (Seq.index ci (4*h+j)) - v (t b h j) /\
          (v (t b h j)) % 8380417 == (v (Seq.index ci (4*h+j+2)) * v (zm b h) * 8265825) % 8380417))
      = lemma_atom_to_bf_l1_avx2 (Seq.index orig b) (Seq.index fut b) (fun h -> zm b h)
     in Classical.forall_intro aux_bf);
    (let aux_z (b: nat{b < 32}) (h: nat{h < 2}) : Lemma
       ((v (zm b h)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (2*b + h + 64) ] <: i32) * pow2 32) % 8380417)
      = reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
        let _ = zeta_r (2*b + h + 64) in
        C.lemma_v_zetas_eq_zeta (2*b + h + 64)
     in Classical.forall_intro_2 aux_z);
    C.lemma_ntt_layer_1_step_to_hacspec_poly orig fut t zm
#pop-options

(* FULL L1 body glue: from input bound + symbolic L1 post, derive the complete
   layer-fn post (output bound + functional congruence to ntt_layer .. (mk_usize 1)). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l1_full_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (bnd:nat{bnd + 8380416 < pow2 31})
    : Lemma
        (requires is_i32b_poly_avx2 bnd orig_re /\ l1_post_sym orig_re re)
        (ensures
          is_i32b_poly_avx2 (bnd + 8380416) re /\
          (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 1) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = lemma_chunkfacts_from_lift_l1 orig_re re;
    let aux (b:nat{b<32}) : Lemma
        (unit_post_l1_avx2 (Seq.index (chunks_of_re_avx2 orig_re) b) (Seq.index (chunks_of_re_avx2 re) b)
           (mk_i32 (zeta_r (2*b + 0 + 64))) (mk_i32 (zeta_r (2*b + 1 + 64)))
         /\ (forall (l:nat). l<8 ==>
              Spec.Utils.is_i32b (bnd + 8380416) (to_i32x8 (Seq.index re b).f_value (mk_u64 l))))
      = lemma_l1_chunk_avx2 orig_re re bnd b
    in Classical.forall_intro aux;
    lemma_is_i32b_poly_avx2_intro (bnd + 8380416) re;
    lemma_l1_driver_compose_avx2 (chunks_of_re_avx2 orig_re) (chunks_of_re_avx2 re)
#pop-options

(* Bridge: literal-zeta L1 post (l1_post) implies symbolic-zeta form (l1_post_sym).
   L1 chunk zetas are zeta_r(2b+h+64), b=0..31,h=0..1 -> idx 64..127 (64 literals). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_l1post_to_sym (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires l1_post re re_fut) (ensures l1_post_sym re re_fut)
  =
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 64 == (- 3930395));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 65 == (- 1528703));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 66 == (- 3677745));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 67 == (- 3041255));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 68 == (- 1452451));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 69 == 3475950);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 70 == 2176455);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 71 == (- 1585221));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 72 == (- 1257611));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 73 == 1939314);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 74 == (- 4083598));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 75 == (- 1000202));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 76 == (- 3190144));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 77 == (- 3157330));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 78 == (- 3632928));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 79 == 126922);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 80 == 3412210);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 81 == (- 983419));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 82 == 2147896);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 83 == 2715295);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 84 == (- 2967645));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 85 == (- 3693493));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 86 == (- 411027));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 87 == (- 2477047));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 88 == (- 671102));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 89 == (- 1228525));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 90 == (- 22981));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 91 == (- 1308169));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 92 == (- 381987));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 93 == 1349076);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 94 == 1852771);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 95 == (- 1430430));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 96 == (- 3343383));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 97 == 264944);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 98 == 508951);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 99 == 3097992);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 100 == 44288);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 101 == (- 1100098));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 102 == 904516);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 103 == 3958618);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 104 == (- 3724342));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 105 == (- 8578));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 106 == 1653064);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 107 == (- 3249728));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 108 == 2389356);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 109 == (- 210977));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 110 == 759969);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 111 == (- 1316856));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 112 == 189548);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 113 == (- 3553272));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 114 == 3159746);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 115 == (- 1851402));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 116 == (- 2409325));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 117 == (- 177440));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 118 == 1315589);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 119 == 1341330);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 120 == 1285669);
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 121 == (- 1584928));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 122 == (- 812732));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 123 == (- 1439742));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 124 == (- 3019102));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 125 == (- 3881060));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 126 == (- 3628969));
    assert_norm (Spec.MLDSA.NttConstants.zeta_r 127 == 3839961)
#pop-options
#pop-options
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(index < 31)]
#[hax_lib::ensures(|_result| fstar!(r"
        butterfly_2_spec (Seq.index ${re} (v $index)).f_value
                         (Seq.index ${re} (v $index + 1)).f_value
                         $zeta_a0 $zeta_a1 $zeta_a2 $zeta_a3 $zeta_b0 $zeta_b1 $zeta_b2 $zeta_b3
                         (Seq.index ${re}_future (v $index)).f_value
                         (Seq.index ${re}_future (v $index + 1)).f_value /\
        Spec.Utils.modifies2_32 re ${re}_future $index ($index +! mk_int 1)
"))]
fn butterfly_2(
    re: &mut AVX2RingElement,
    index: usize,
    zeta_a0: i32,
    zeta_a1: i32,
    zeta_a2: i32,
    zeta_a3: i32,
    zeta_b0: i32,
    zeta_b1: i32,
    zeta_b2: i32,
    zeta_b3: i32,
) {
    // For proofs, the style that works best is to separate out the
    // stateful operations (reading and writing to mutable arrays)
    // from the core computation. So this and the following functions
    // have the pattern: read from array; compute; write to array.

    let re0 = re[index].value;
    let re1 = re[index + 1].value;

    // We shuffle the terms to group those that need to be multiplied
    // with zetas in the high QWORDS of the vectors, i.e. if the inputs are
    //   a = (a7, a6, a5, a4, a3, a2, a1, a0)
    //   b = (b7, b6, b5, b4, b3, b2, b1, b0)
    // after shuffling we have
    //   a_shuffled = ( a7, a5, a6, a4, a3, a1, a2, a0)
    //   b_shuffled = ( b7, b5, b6, b4, b3, b1, b2, b0)
    const SHUFFLE: i32 = 0b11_01_10_00;
    let a = mm256_shuffle_epi32::<SHUFFLE>(re0);
    let b = mm256_shuffle_epi32::<SHUFFLE>(re1);

    // Now we can use the same approach as for `butterfly_4`, only
    // zetas need to be adjusted.
    let summands = mm256_unpacklo_epi64(a, b);
    let mut zeta_products = mm256_unpackhi_epi64(a, b);
    let zetas = mm256_set_epi32(
        zeta_b3, zeta_b2, zeta_a3, zeta_a2, zeta_b1, zeta_b0, zeta_a1, zeta_a0,
    );

    arithmetic::montgomery_multiply(&mut zeta_products, &zetas);

    let sub_terms = mm256_sub_epi32(summands, zeta_products);
    let add_terms = mm256_add_epi32(summands, zeta_products);

    let a_terms_shuffled = mm256_unpacklo_epi64(add_terms, sub_terms);
    let b_terms_shuffled = mm256_unpackhi_epi64(add_terms, sub_terms);

    // Here, we undo the initial shuffle (it's self-inverse).
    let nre0 = mm256_shuffle_epi32::<SHUFFLE>(a_terms_shuffled);
    let nre1 = mm256_shuffle_epi32::<SHUFFLE>(b_terms_shuffled);

    // This assert allows all the SMT Patterns to kick in and prove correctness
    hax_lib::fstar!(
        r#"assert (butterfly_2_spec 
                            $re0 $re1 $zeta_a0 $zeta_a1 $zeta_a2 $zeta_a3 
                            $zeta_b0 $zeta_b1 $zeta_b2 $zeta_b3 $nre0 $nre1)"#
    );

    re[index].value = nre0;
    re[index + 1].value = nre1;
}

// Compute (a,b) ↦ (a + ζb, a - ζb) at layer 1 for 2 SIMD Units in one go.
#[inline(always)]
#[hax_lib::fstar::before(
    r#"
let butterfly_4_spec re0 re1 zeta_a0 zeta_a1 zeta_b0 zeta_b1 nre0 nre1 =
    (to_i32x8 nre0 (mk_u64 0), to_i32x8 nre0 (mk_u64 2)) ==
    ntt_step zeta_a0 (to_i32x8 re0 (mk_u64 0), to_i32x8 re0 (mk_u64 2)) /\
    (to_i32x8 nre0 (mk_u64 1), to_i32x8 nre0 (mk_u64 3)) ==
    ntt_step zeta_a0 (to_i32x8 re0 (mk_u64 1), to_i32x8 re0 (mk_u64 3)) /\
    (to_i32x8 nre0 (mk_u64 4), to_i32x8 nre0 (mk_u64 6)) ==
    ntt_step zeta_a1 (to_i32x8 re0 (mk_u64 4), to_i32x8 re0 (mk_u64 6)) /\
    (to_i32x8 nre0 (mk_u64 5), to_i32x8 nre0 (mk_u64 7)) ==
    ntt_step zeta_a1 (to_i32x8 re0 (mk_u64 5), to_i32x8 re0 (mk_u64 7)) /\
    (to_i32x8 nre1 (mk_u64 0), to_i32x8 nre1 (mk_u64 2)) ==
    ntt_step zeta_b0 (to_i32x8 re1 (mk_u64 0), to_i32x8 re1 (mk_u64 2)) /\
    (to_i32x8 nre1 (mk_u64 1), to_i32x8 nre1 (mk_u64 3)) ==
    ntt_step zeta_b0 (to_i32x8 re1 (mk_u64 1), to_i32x8 re1 (mk_u64 3)) /\
    (to_i32x8 nre1 (mk_u64 4), to_i32x8 nre1 (mk_u64 6)) ==
    ntt_step zeta_b1 (to_i32x8 re1 (mk_u64 4), to_i32x8 re1 (mk_u64 6)) /\
    (to_i32x8 nre1 (mk_u64 5), to_i32x8 nre1 (mk_u64 7)) ==
    ntt_step zeta_b1 (to_i32x8 re1 (mk_u64 5), to_i32x8 re1 (mk_u64 7))
"#
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(index < 31)]
#[hax_lib::ensures(|_result| fstar!(r"
        butterfly_4_spec    (Seq.index ${re} (v $index)).f_value
                            (Seq.index ${re} (v $index + 1)).f_value
                            $zeta_a0 $zeta_a1 $zeta_b0 $zeta_b1
                            (Seq.index ${re}_future (v $index)).f_value
                            (Seq.index ${re}_future (v $index + 1)).f_value /\
        Spec.Utils.modifies2_32 $re ${re}_future $index ($index +! mk_int 1)
"))]
fn butterfly_4(
    re: &mut AVX2RingElement,
    index: usize,
    zeta_a0: i32,
    zeta_a1: i32,
    zeta_b0: i32,
    zeta_b1: i32,
) {
    let re0 = re[index].value;
    let re1 = re[index + 1].value;

    let summands = mm256_unpacklo_epi64(re0, re1);
    let mut zeta_products = mm256_unpackhi_epi64(re0, re1);

    let zetas = mm256_set_epi32(
        zeta_b1, zeta_b1, zeta_a1, zeta_a1, zeta_b0, zeta_b0, zeta_a0, zeta_a0,
    );
    arithmetic::montgomery_multiply(&mut zeta_products, &zetas);

    let sub_terms = mm256_sub_epi32(summands, zeta_products);
    let add_terms = mm256_add_epi32(summands, zeta_products);

    // Results are shuffled across the two SIMD registers.
    // We need to bring them in the right order.
    let nre0 = mm256_unpacklo_epi64(add_terms, sub_terms);
    let nre1 = mm256_unpackhi_epi64(add_terms, sub_terms);

    // This assert allows all the SMT Patterns to kick in and prove correctness
    hax_lib::fstar!(
        r#"assert (butterfly_4_spec 
        $re0 $re1 $zeta_a0 $zeta_a1 $zeta_b0 $zeta_b1 $nre0 $nre1)"#
    );

    re[index].value = nre0;
    re[index + 1].value = nre1;
}

// Compute (a,b) ↦ (a + ζb, a - ζb) at layer 2 for 2 SIMD Units in one go.
#[inline(always)]
#[hax_lib::fstar::before(
    r#"
let butterfly_8_spec re0 re1 zeta0 zeta1 nre0 nre1 =
    (to_i32x8 nre0 (mk_u64 0), to_i32x8 nre0 (mk_u64 4)) ==
     ntt_step zeta0 (to_i32x8 re0 (mk_u64 0), to_i32x8 re0 (mk_u64 4)) /\
    (to_i32x8 nre0 (mk_u64 1), to_i32x8 nre0 (mk_u64 5)) ==
     ntt_step zeta0 (to_i32x8 re0 (mk_u64 1), to_i32x8 re0 (mk_u64 5)) /\
    (to_i32x8 nre0 (mk_u64 2), to_i32x8 nre0 (mk_u64 6)) ==
     ntt_step zeta0 (to_i32x8 re0 (mk_u64 2), to_i32x8 re0 (mk_u64 6)) /\
    (to_i32x8 nre0 (mk_u64 3), to_i32x8 nre0 (mk_u64 7)) ==
     ntt_step zeta0 (to_i32x8 re0 (mk_u64 3), to_i32x8 re0 (mk_u64 7)) /\
    (to_i32x8 nre1 (mk_u64 0), to_i32x8 nre1 (mk_u64 4)) ==
     ntt_step zeta1 (to_i32x8 re1 (mk_u64 0), to_i32x8 re1 (mk_u64 4)) /\
    (to_i32x8 nre1 (mk_u64 1), to_i32x8 nre1 (mk_u64 5)) ==
     ntt_step zeta1 (to_i32x8 re1 (mk_u64 1), to_i32x8 re1 (mk_u64 5)) /\
    (to_i32x8 nre1 (mk_u64 2), to_i32x8 nre1 (mk_u64 6)) ==
     ntt_step zeta1 (to_i32x8 re1 (mk_u64 2), to_i32x8 re1 (mk_u64 6)) /\
    (to_i32x8 nre1 (mk_u64 3), to_i32x8 nre1 (mk_u64 7)) ==
     ntt_step zeta1 (to_i32x8 re1 (mk_u64 3), to_i32x8 re1 (mk_u64 7))
"#
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(index < 31)]
#[hax_lib::ensures(|_result| fstar!(r"
        butterfly_8_spec    (Seq.index ${re} (v $index)).f_value
                            (Seq.index ${re} (v $index + 1)).f_value
                            $zeta0 $zeta1
                            (Seq.index ${re}_future (v $index)).f_value
                            (Seq.index ${re}_future (v $index + 1)).f_value /\
        Spec.Utils.modifies2_32 $re ${re}_future $index ($index +! mk_int 1)
"))]
fn butterfly_8(re: &mut AVX2RingElement, index: usize, zeta0: i32, zeta1: i32) {
    let re0 = re[index].value;
    let re1 = re[index + 1].value;

    let summands = mm256_set_m128i(mm256_castsi256_si128(re1), mm256_castsi256_si128(re0));
    let mut zeta_products = mm256_permute2x128_si256::<0b0001_0011>(re1, re0);

    let zetas = mm256_set_epi32(zeta1, zeta1, zeta1, zeta1, zeta0, zeta0, zeta0, zeta0);
    arithmetic::montgomery_multiply(&mut zeta_products, &zetas);

    let sub_terms = mm256_sub_epi32(summands, zeta_products);
    let add_terms = mm256_add_epi32(summands, zeta_products);

    let nre0 = mm256_set_m128i(
        mm256_castsi256_si128(sub_terms),
        mm256_castsi256_si128(add_terms),
    );
    let nre1 = mm256_permute2x128_si256::<0b0001_0011>(sub_terms, add_terms);

    // This assert allows all the SMT Patterns to kick in and prove correctness
    hax_lib::fstar!(
        r#"assert (butterfly_8_spec 
         $re0 $re1 $zeta0 $zeta1 $nre0 $nre1)"#
    );

    re[index].value = nre0;
    re[index + 1].value = nre1;
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_poly_avx2 (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 7 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    is_i32b_poly_avx2 (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 8 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX) ${re}_future /\
    (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 ${re}) in
     let out_flat = C.simd_units_to_array (chunks_of_re_avx2 ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 0) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#))]
unsafe fn ntt_at_layer_0(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig_re = re.clone();

    butterfly_2(
        re, 0, 2091667, 3407706, 2316500, 3817976, -3342478, 2244091, -2446433, -3562462,
    );
    butterfly_2(
        re, 2, 266997, 2434439, -1235728, 3513181, -3520352, -3759364, -1197226, -3193378,
    );
    butterfly_2(
        re, 4, 900702, 1859098, 909542, 819034, 495491, -1613174, -43260, -522500,
    );
    butterfly_2(
        re, 6, -655327, -3122442, 2031748, 3207046, -3556995, -525098, -768622, -3595838,
    );
    butterfly_2(
        re, 8, 342297, 286988, -2437823, 4108315, 3437287, -3342277, 1735879, 203044,
    );
    butterfly_2(
        re, 10, 2842341, 2691481, -2590150, 1265009, 4055324, 1247620, 2486353, 1595974,
    );
    butterfly_2(
        re, 12, -3767016, 1250494, 2635921, -3548272, -2994039, 1869119, 1903435, -1050970,
    );
    butterfly_2(
        re, 14, -1333058, 1237275, -3318210, -1430225, -451100, 1312455, 3306115, -1962642,
    );
    butterfly_2(
        re, 16, -1279661, 1917081, -2546312, -1374803, 1500165, 777191, 2235880, 3406031,
    );
    butterfly_2(
        re, 18, -542412, -2831860, -1671176, -1846953, -2584293, -3724270, 594136, -3776993,
    );
    butterfly_2(
        re, 20, -2013608, 2432395, 2454455, -164721, 1957272, 3369112, 185531, -1207385,
    );
    butterfly_2(
        re, 22, -3183426, 162844, 1616392, 3014001, 810149, 1652634, -3694233, -1799107,
    );
    butterfly_2(
        re, 24, -3038916, 3523897, 3866901, 269760, 2213111, -975884, 1717735, 472078,
    );
    butterfly_2(
        re, 26, -426683, 1723600, -1803090, 1910376, -1667432, -1104333, -260646, -3833893,
    );
    butterfly_2(
        re, 28, -2939036, -2235985, -420899, -2286327, 183443, -976891, 1612842, -3545687,
    );
    butterfly_2(
        re, 30, -554416, 3919660, -48306, -1362209, 3937738, 1400424, -846154, 1976782,
    );

    hax_lib::fstar!(r#"
assert (l0_post orig_re ${re});
lemma_l0post_to_sym orig_re ${re};
assert_norm (v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX == 8380416);
lemma_l0_full_avx2 orig_re ${re}
  (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 7 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX)
"#);
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_poly_avx2 (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 6 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    is_i32b_poly_avx2 (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 7 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX) ${re}_future /\
    (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 ${re}) in
     let out_flat = C.simd_units_to_array (chunks_of_re_avx2 ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 1) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#))]
unsafe fn ntt_at_layer_1(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig_re = re.clone();

    butterfly_4(re, 0, -3930395, -1528703, -3677745, -3041255);
    butterfly_4(re, 2, -1452451, 3475950, 2176455, -1585221);
    butterfly_4(re, 4, -1257611, 1939314, -4083598, -1000202);
    butterfly_4(re, 6, -3190144, -3157330, -3632928, 126922);
    butterfly_4(re, 8, 3412210, -983419, 2147896, 2715295);
    butterfly_4(re, 10, -2967645, -3693493, -411027, -2477047);
    butterfly_4(re, 12, -671102, -1228525, -22981, -1308169);
    butterfly_4(re, 14, -381987, 1349076, 1852771, -1430430);
    butterfly_4(re, 16, -3343383, 264944, 508951, 3097992);
    butterfly_4(re, 18, 44288, -1100098, 904516, 3958618);
    butterfly_4(re, 20, -3724342, -8578, 1653064, -3249728);
    butterfly_4(re, 22, 2389356, -210977, 759969, -1316856);
    butterfly_4(re, 24, 189548, -3553272, 3159746, -1851402);
    butterfly_4(re, 26, -2409325, -177440, 1315589, 1341330);
    butterfly_4(re, 28, 1285669, -1584928, -812732, -1439742);
    butterfly_4(re, 30, -3019102, -3881060, -3628969, 3839961);

    hax_lib::fstar!(r#"
assert (l1_post orig_re ${re});
lemma_l1post_to_sym orig_re ${re};
assert_norm (v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX == 8380416);
lemma_l1_full_avx2 orig_re ${re}
  (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 6 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX)
"#);
}

#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_poly_avx2 (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 5 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    is_i32b_poly_avx2 (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 6 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX) ${re}_future /\
    (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 ${re}) in
     let out_flat = C.simd_units_to_array (chunks_of_re_avx2 ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 2) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#))]
unsafe fn ntt_at_layer_2(re: &mut AVX2RingElement) {
    #[cfg(hax)]
    let orig_re = re.clone();

    butterfly_8(re, 0, 2706023, 95776);
    butterfly_8(re, 2, 3077325, 3530437);
    butterfly_8(re, 4, -1661693, -3592148);
    butterfly_8(re, 6, -2537516, 3915439);
    butterfly_8(re, 8, -3861115, -3043716);
    butterfly_8(re, 10, 3574422, -2867647);
    butterfly_8(re, 12, 3539968, -300467);
    butterfly_8(re, 14, 2348700, -539299);
    butterfly_8(re, 16, -1699267, -1643818);
    butterfly_8(re, 18, 3505694, -3821735);
    butterfly_8(re, 20, 3507263, -2140649);
    butterfly_8(re, 22, -1600420, 3699596);
    butterfly_8(re, 24, 811944, 531354);
    butterfly_8(re, 26, 954230, 3881043);
    butterfly_8(re, 28, 3900724, -2556880);
    butterfly_8(re, 30, 2071892, -2797779);

    hax_lib::fstar!(r#"
assert (l2_post orig_re ${re});
lemma_l2post_to_sym orig_re ${re};
assert_norm (v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX == 8380416);
lemma_l2_full_avx2 orig_re ${re}
  (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND + 5 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX)
"#);
}

/// This is equivalent to the pqclean 0 and 1
///
/// This does 32 Montgomery multiplications (192 multiplications).
/// This is the same as in pqclean. The only difference is locality of registers.
// monolithic (NO --split_queries always): split clutters the comp_7_6_done sub-query
// with the L6/L7 value-foralls and saturates rlimit 400; monolithic lets Z3 discharge
// the opaque comp_7_6_done atom directly.
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 600 --z3refresh"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Avx2NttTheory.is_i32b_poly_avx2 8380416 $re
"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Avx2NttTheory.comp_7_6_done $re ${re}_future /\
    Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 2*8380416) ${re}_future
"#))]
unsafe fn ntt_at_layer_7_and_6(re: &mut AVX2RingElement) {
    let field_modulus = mm256_set1_epi32(crate::simd::traits::FIELD_MODULUS);
    let inverse_of_modulus_mod_montgomery_r =
        mm256_set1_epi32(crate::simd::traits::INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i32);

    #[inline(always)]
    #[hax_lib::fstar::options(r#"--fuel 1 --ifuel 1 --z3rlimit 300"#)]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires({
        use crate::constants::FIELD_MODULUS;
        use crate::simd::traits::INVERSE_OF_MODULUS_MOD_MONTGOMERY_R;
        use hax_lib::int::{ToInt, int};
        hax_lib::eq(field_modulus, mm256_set1_epi32(FIELD_MODULUS)).and(
            hax_lib::eq(inverse_of_modulus_mod_montgomery_r, mm256_set1_epi32(INVERSE_OF_MODULUS_MOD_MONTGOMERY_R as i32))
        ).and(index.to_int() + step_by.to_int() < int!(32)).and(step_by > 0)
    })]
    #[hax_lib::ensures(|result| fstar!(r#"
        let m = Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply
                  (Seq.index $re (v $index + v $step_by)).f_value $zeta in
        Spec.Utils.modifies2_32 $re ${re}_future $index ($index +! $step_by) /\
        Seq.index ${re}_future (v $index) ==
          ({ Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value =
               Libcrux_intrinsics.Avx2.mm256_add_epi32 (Seq.index $re (v $index)).f_value m }
           <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) /\
        Seq.index ${re}_future (v $index + v $step_by) ==
          ({ Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value =
               Libcrux_intrinsics.Avx2.mm256_sub_epi32 (Seq.index $re (v $index)).f_value m }
           <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
    "#))]
    fn mul(
        re: &mut AVX2RingElement,
        index: usize,
        zeta: Vec256,
        step_by: usize,
        field_modulus: Vec256,
        inverse_of_modulus_mod_montgomery_r: Vec256,
    ) {
        hax_lib::fstar!(
            r#"
        reveal_opaque (`%Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply)
          (Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply);
        reveal_opaque (`%Libcrux_ml_dsa.Simd.Avx2.Arithmetic.add)
          (Libcrux_ml_dsa.Simd.Avx2.Arithmetic.add);
        reveal_opaque (`%Libcrux_ml_dsa.Simd.Avx2.Arithmetic.subtract)
          (Libcrux_ml_dsa.Simd.Avx2.Arithmetic.subtract)
    "#
        );
        let mut t = re[index + step_by].value;
        arithmetic::montgomery_multiply_aux(
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
            &mut t,
            &zeta,
        );
        re[index + step_by].value = re[index].value;
        arithmetic::subtract(&mut re[index + step_by].value, &t);
        arithmetic::add(&mut re[index].value, &t);
    }

    // Note: For proofs, it is better to use concrete constants instead of const expressions
    const STEP_BY_7: usize = 16; //2 * COEFFICIENTS_IN_SIMD_UNIT;
    const STEP_BY_6: usize = 8; //(1 << 6) / COEFFICIENTS_IN_SIMD_UNIT;

    // Per-pair REAL Montgomery butterfly: wraps `mul`; proves modifies + the two
    // output bounds + the bf_pair VALUE (via Avx2NttTheory.lemma_bf_pair_def).
    // Runtime-identical (#[inline(always)]).
    #[inline(always)]
    #[hax_lib::fstar::before(
        r#"
(* ============================================================================
   IN-BODY 7_6 (Phase C): prove comp_7_6_done OVER THE REAL 32 Montgomery
   butterflies (ntt_at_layer_7_and_6___mul), no fstar::replace, mirroring
   Avx2NttTheory.build_out_impl + ntt_at_layer_7_and_6_interleaved_o in-body.

   Local proof scaffolding (NOT impl): q76_quadpost (abstract restatement of
   `quad`'s ensures) + two pure bf_pair-congruence substitution lemmas + the
   L6-value driver + the 32-unit bound driver, all copied from the theory's
   .fst-internal helpers (they only consume the abstract .fsti atoms bf_pair /
   is_i32b_unit_avx2, so they live legally in the consumer).  Then the real
   per-pair `mul_bf` (wraps ___mul) and the 4-pair `quad` (= Avx2NttTheory.quad's
   contract, real muls).  bf_pair value comes from the ADDITIVE exposure
   Avx2NttTheory.lemma_bf_pair_def. ============================================ *)

(* abstract restatement of Avx2NttTheory.quad's ensures (the 3 foralls). *)
unfold let q76_quadpost (re re_f: Avx2NttTheory.av32)
                        (base:nat) (sb:nat{sb > 0 /\ base + 4 + sb <= 32}) (bnd:nat) (zeta:i32) : Type0 =
  (forall (k:nat). k < 32 /\
      ~((k >= base /\ k < base + 4) \/ (k >= base + sb /\ k < base + 4 + sb))
      ==> Seq.index re_f k == Seq.index re k) /\
  (forall (k:nat). (k >= base /\ k < base + 4) \/ (k >= base + sb /\ k < base + 4 + sb) ==>
      Avx2NttTheory.is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_f k)) /\
  (forall (q:nat). q >= base /\ q < base + 4 ==>
      (let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index re q) (Seq.index re (q + sb)) zeta in
       Seq.index re_f q == nu0 /\ Seq.index re_f (q + sb) == nu1))

(* an L7-quad output array `a` at lo-unit q in [base,base+4) carries fst(bf_pair src[q] src[q+16] z1);
   if src agrees with orig at q,q+16 and mid==bf_pair-of-orig there, then a==mid there.  Pure
   bf_pair determinism (congruence), abstract bf_pair OK. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let q76_l7out_eq_mid (orig mid src a: Avx2NttTheory.av32) (q:nat)
    : Lemma
      (requires
        q < 16 /\
        Seq.index src q     == Seq.index orig q /\
        Seq.index src (q+16) == Seq.index orig (q+16) /\
        (let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index src q) (Seq.index src (q+16)) (mk_i32 (zeta_r 1)) in
         Seq.index a q == nu0 /\ Seq.index a (q+16) == nu1) /\
        (let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index orig q) (Seq.index orig (q+16)) (mk_i32 (zeta_r 1)) in
         Seq.index mid q == nu0 /\ Seq.index mid (q+16) == nu1))
      (ensures Seq.index a q == Seq.index mid q /\ Seq.index a (q+16) == Seq.index mid (q+16)) = ()
#pop-options

(* a quad's bf_pair value-post is about its INPUT `inp`; if `inp` agrees with `mid` on the read
   pair (q,q+8), the value-post is equivalently about `mid`.  Pure substitution. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let q76_l6_pair_subst_mid (inp mid out_q: Avx2NttTheory.av32) (q:nat{q+8 < 32}) (zL6:i32)
    : Lemma
      (requires
        Seq.index inp q == Seq.index mid q /\
        Seq.index inp (q+8) == Seq.index mid (q+8) /\
        (let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index inp q) (Seq.index inp (q+8)) zL6 in
         Seq.index out_q q == nu0 /\ Seq.index out_q (q+8) == nu1))
      (ensures
        (let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index mid q) (Seq.index mid (q+8)) zL6 in
         Seq.index out_q q == nu0 /\ Seq.index out_q (q+8) == nu1)) = ()
#pop-options

(* STANDALONE L6-value driver (clean context).  Preconditions = the 8 quad posts (as
   q76_quadpost abstract foralls, in IMPL order) + the L7 (`mid`) bf_pair-of-orig relation.
   Discharges out's bf_pair-of-MID per-L6-lo-unit post.  Copied from
   Avx2NttTheory.lemma_build_out_l6_value. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --z3refresh"
let q76_l6_value
      (orig mid qa0 qa1 qb0 qb1 qc0 qc1 qd0 out: Avx2NttTheory.av32)
    : Lemma
      (requires
        (let z1 = mk_i32 (zeta_r 1) in let z2 = mk_i32 (zeta_r 2) in let z3 = mk_i32 (zeta_r 3) in
         (forall (u:nat). u < 16 ==>
            (let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index orig u) (Seq.index orig (u+16)) z1 in
             Seq.index mid u == nu0 /\ Seq.index mid (u+16) == nu1)) /\
         q76_quadpost orig qa0 0  16 8380416            z1 /\
         q76_quadpost qa0  qa1 8  16 8380416            z1 /\
         q76_quadpost qa1  qb0 0  8  (8380416+8380416)  z2 /\
         q76_quadpost qb0  qb1 16 8  (8380416+8380416)  z3 /\
         q76_quadpost qb1  qc0 4  16 8380416            z1 /\
         q76_quadpost qc0  qc1 12 16 8380416            z1 /\
         q76_quadpost qc1  qd0 4  8  (8380416+8380416)  z2 /\
         q76_quadpost qd0  out 20 8  (8380416+8380416)  z3))
      (ensures
        (forall (u:nat). (u % 16 < 8) /\ u < 32 ==>
           (let zL6 = (if u < 16 then mk_i32 (zeta_r 2) else mk_i32 (zeta_r 3)) in
            let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index mid u) (Seq.index mid (u+8)) zL6 in
            Seq.index out u == nu0 /\ Seq.index out (u+8) == nu1)))
  = let z1 : i32 = mk_i32 (zeta_r 1) in
    let z2 : i32 = mk_i32 (zeta_r 2) in
    let z3 : i32 = mk_i32 (zeta_r 3) in
    assert_norm (zeta_r 1 == 25847);
    assert_norm (zeta_r 2 == (-2608894));
    assert_norm (zeta_r 3 == (-518909));
    let aux (u:nat{u<32}) : Lemma
        ((u % 16 < 8) ==>
           (let zL6 = (if u < 16 then z2 else z3) in
            let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index mid u) (Seq.index mid (u+8)) zL6 in
            Seq.index out u == nu0 /\ Seq.index out (u+8) == nu1))
      = if (u % 16 < 8) then begin
          if u < 4 then begin
            q76_l7out_eq_mid orig mid orig qa0 u;
            q76_l7out_eq_mid orig mid qa0  qa1 (u+8);
            assert (Seq.index qa1 u == Seq.index mid u);
            q76_l6_pair_subst_mid qa1 mid qb0 u z2;
            assert (Seq.index out u == Seq.index qb0 u);
            assert (Seq.index out (u+8) == Seq.index qb0 (u+8))
          end
          else if u < 8 then begin
            q76_l7out_eq_mid orig mid qb1 qc0 u;
            q76_l7out_eq_mid orig mid qc0 qc1 (u+8);
            assert (Seq.index qc1 u == Seq.index mid u);
            q76_l6_pair_subst_mid qc1 mid qd0 u z2;
            assert (Seq.index out u == Seq.index qd0 u);
            assert (Seq.index out (u+8) == Seq.index qd0 (u+8))
          end
          else if u < 20 then begin
            q76_l7out_eq_mid orig mid orig qa0 (u-16);
            q76_l7out_eq_mid orig mid qa0  qa1 (u-8);
            assert (Seq.index qb0 u == Seq.index mid u);
            assert (Seq.index qb0 (u+8) == Seq.index mid (u+8));
            q76_l6_pair_subst_mid qb0 mid qb1 u z3;
            assert (Seq.index out u == Seq.index qb1 u);
            assert (Seq.index out (u+8) == Seq.index qb1 (u+8))
          end
          else begin
            q76_l7out_eq_mid orig mid qb1 qc0 (u-16);
            q76_l7out_eq_mid orig mid qc0 qc1 (u-8);
            assert (Seq.index qd0 u == Seq.index mid u);
            assert (Seq.index qd0 (u+8) == Seq.index mid (u+8));
            q76_l6_pair_subst_mid qd0 mid out u z3
          end
        end
    in
    Classical.forall_intro aux
#pop-options

(* STANDALONE 32-unit bound driver (clean context).  Each unit's LAST touch is its L6 quad
   (qb0/qb1/qd0/out), bounding it to NTT_BASE+2; later quads frame it.  From the 8 quad posts. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --z3refresh"
let q76_bound
      (orig qa0 qa1 qb0 qb1 qc0 qc1 qd0 out: Avx2NttTheory.av32)
    : Lemma
      (requires
        (let z1 = mk_i32 (zeta_r 1) in let z2 = mk_i32 (zeta_r 2) in let z3 = mk_i32 (zeta_r 3) in
         q76_quadpost orig qa0 0  16 8380416            z1 /\
         q76_quadpost qa0  qa1 8  16 8380416            z1 /\
         q76_quadpost qa1  qb0 0  8  (8380416+8380416)  z2 /\
         q76_quadpost qb0  qb1 16 8  (8380416+8380416)  z3 /\
         q76_quadpost qb1  qc0 4  16 8380416            z1 /\
         q76_quadpost qc0  qc1 12 16 8380416            z1 /\
         q76_quadpost qc1  qd0 4  8  (8380416+8380416)  z2 /\
         q76_quadpost qd0  out 20 8  (8380416+8380416)  z3))
      (ensures
        (forall (k:nat). k < 32 ==>
           Avx2NttTheory.is_i32b_unit_avx2 (8380416 + 2*8380416) (Seq.index out k)))
  = let nb2 : nat = 8380416 + 2*8380416 in
    let aux (k:nat{k<32}) : Lemma
        (Avx2NttTheory.is_i32b_unit_avx2 nb2 (Seq.index out k))
      = if k < 4 || (k >= 8 && k < 12) then begin
          // bounded nb2 by qb0 (quad qa1 0 8); framed through qb1,qc0,qc1,qd0,out (all disjoint from k)
          assert (Avx2NttTheory.is_i32b_unit_avx2 nb2 (Seq.index qb0 k));
          assert (Seq.index out k == Seq.index qd0 k);
          assert (Seq.index qd0 k == Seq.index qc1 k);
          assert (Seq.index qc1 k == Seq.index qc0 k);
          assert (Seq.index qc0 k == Seq.index qb1 k);
          assert (Seq.index qb1 k == Seq.index qb0 k)
        end
        else if (k >= 16 && k < 20) || (k >= 24 && k < 28) then begin
          // bounded nb2 by qb1 (quad qb0 16 8); framed through qc0,qc1,qd0,out
          assert (Avx2NttTheory.is_i32b_unit_avx2 nb2 (Seq.index qb1 k));
          assert (Seq.index out k == Seq.index qd0 k);
          assert (Seq.index qd0 k == Seq.index qc1 k);
          assert (Seq.index qc1 k == Seq.index qc0 k);
          assert (Seq.index qc0 k == Seq.index qb1 k)
        end
        else if (k >= 4 && k < 8) || (k >= 12 && k < 16) then begin
          // bounded nb2 by qd0 (quad qc1 4 8); framed through out
          assert (Avx2NttTheory.is_i32b_unit_avx2 nb2 (Seq.index qd0 k));
          assert (Seq.index out k == Seq.index qd0 k)
        end
        else
          // {20-23,28-31}: bounded nb2 directly by out (quad qd0 20 8, last)
          assert (Avx2NttTheory.is_i32b_unit_avx2 nb2 (Seq.index out k))
    in
    Classical.forall_intro aux
#pop-options
"#
    )]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"#)]
    #[hax_lib::requires(fstar!(r#"
        v $bnd + 8380416 < pow2 31 /\
        v $step_by > 0 /\
        v $index + v $step_by < 32 /\
        Spec.Utils.is_i32b 4190208 $zeta /\
        $zeta_v == Libcrux_intrinsics.Avx2.mm256_set1_epi32 $zeta /\
        $field_modulus ==
          Libcrux_intrinsics.Avx2.mm256_set1_epi32 Libcrux_ml_dsa.Constants.v_FIELD_MODULUS /\
        $inverse_of_modulus_mod_montgomery_r ==
          Libcrux_intrinsics.Avx2.mm256_set1_epi32
            (cast (Libcrux_ml_dsa.Simd.Traits.v_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R <: u64) <: i32) /\
        Avx2NttTheory.is_i32b_unit_avx2 (v $bnd) (Seq.index $re (v $index)) /\
        Avx2NttTheory.is_i32b_unit_avx2 (v $bnd) (Seq.index $re (v $index + v $step_by))
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        Spec.Utils.modifies2_32 $re ${re}_future $index ($index +! $step_by) /\
        Avx2NttTheory.is_i32b_unit_avx2 (v $bnd + 8380416) (Seq.index ${re}_future (v $index)) /\
        Avx2NttTheory.is_i32b_unit_avx2 (v $bnd + 8380416) (Seq.index ${re}_future (v $index + v $step_by)) /\
        (let (nu0, nu1) =
              Avx2NttTheory.bf_pair (Seq.index $re (v $index)) (Seq.index $re (v $index + v $step_by)) $zeta in
         Seq.index ${re}_future (v $index) == nu0 /\ Seq.index ${re}_future (v $index + v $step_by) == nu1)
    "#))]
    fn mul_bf(
        re: &mut AVX2RingElement,
        index: usize,
        step_by: usize,
        zeta_v: Vec256,
        zeta: i32,
        bnd: usize,
        field_modulus: Vec256,
        inverse_of_modulus_mod_montgomery_r: Vec256,
    ) {
        #[cfg(hax)]
        let orig = re.clone();
        mul(
            re,
            index,
            zeta_v,
            step_by,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        // bf_pair VALUE: `mul` gives re[index]={f_value=add(orig[index].f_value, m)},
        //   re[index+sb]={f_value=sub(orig[index].f_value, m)}, m = montgomery_multiply orig[index+sb].f_value zeta_v;
        //   lemma_bf_pair_def gives bf_pair's value with m'=montgomery_multiply orig[index+sb].f_value (mm256_set1_epi32 zeta);
        //   zeta_v == mm256_set1_epi32 zeta -> m == m' -> equal.
        // BOUNDS: ntt_step forall8 (from `mul`'s add/sub form) -> lemma_cross_pair_relations_ws -> intro.
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_bf_pair_def (Seq.index orig (v $index)) (Seq.index orig (v $index + v $step_by)) $zeta;
        let re0 = (Seq.index orig (v $index)).f_value in
        let re1 = (Seq.index orig (v $index + v $step_by)).f_value in
        let nre0 = (Seq.index ${re} (v $index)).f_value in
        let nre1 = (Seq.index ${re} (v $index + v $step_by)).f_value in
        let lane (i:nat{i<8}) : Lemma
            ((to_i32x8 nre0 (mk_u64 i), to_i32x8 nre1 (mk_u64 i)) ==
             ntt_step (to_i32x8 $zeta_v (mk_int i))
               (to_i32x8 re0 (mk_u64 i), to_i32x8 re1 (mk_u64 i)))
          = assert (to_i32x8 $zeta_v (mk_int i) == $zeta);
            assert (to_i32x8 $zeta_v (mk_u64 i) == $zeta)
        in
        lane 0; lane 1; lane 2; lane 3; lane 4; lane 5; lane 6; lane 7;
        introduce forall (l:nat). l < 8 ==> to_i32x8 $zeta_v (mk_int l) == $zeta
        with (introduce l < 8 ==> to_i32x8 $zeta_v (mk_int l) == $zeta
              with _. assert (to_i32x8 $zeta_v (mk_int l) == $zeta));
        Avx2NttTheory.lemma_cross_pair_relations_ws orig ${re} (v $bnd) (v $index) (v $index + v $step_by) $zeta_v $zeta;
        Avx2NttTheory.lemma_is_i32b_unit_avx2_intro (v $bnd + 8380416) (Seq.index ${re} (v $index));
        Avx2NttTheory.lemma_is_i32b_unit_avx2_intro (v $bnd + 8380416) (Seq.index ${re} (v $index + v $step_by))
    "#
        );
    }

    // 4-pair REAL quad (= Avx2NttTheory.quad's contract; body = 4 real mul_bf).
    // lo-units base..base+3 paired with +sb.
    #[inline(always)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"#)]
    #[hax_lib::requires(fstar!(r#"
        v $bnd + 8380416 < pow2 31 /\
        v $sb > 0 /\ v $base + 3 + v $sb < 32 /\ v $base + 4 <= v $base + v $sb /\
        Spec.Utils.is_i32b 4190208 $zeta /\
        $zeta_v == Libcrux_intrinsics.Avx2.mm256_set1_epi32 $zeta /\
        $field_modulus ==
          Libcrux_intrinsics.Avx2.mm256_set1_epi32 Libcrux_ml_dsa.Constants.v_FIELD_MODULUS /\
        $inverse_of_modulus_mod_montgomery_r ==
          Libcrux_intrinsics.Avx2.mm256_set1_epi32
            (cast (Libcrux_ml_dsa.Simd.Traits.v_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R <: u64) <: i32) /\
        (forall (k:nat). (k >= v $base /\ k < v $base + 4) \/
                         (k >= v $base + v $sb /\ k < v $base + 4 + v $sb) ==>
                         Avx2NttTheory.is_i32b_unit_avx2 (v $bnd) (Seq.index $re k))
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        q76_quadpost $re ${re}_future (v $base) (v $sb) (v $bnd) $zeta
    "#))]
    fn quad(
        re: &mut AVX2RingElement,
        base: usize,
        sb: usize,
        zeta_v: Vec256,
        zeta: i32,
        bnd: usize,
        field_modulus: Vec256,
        inverse_of_modulus_mod_montgomery_r: Vec256,
    ) {
        mul_bf(
            re,
            base,
            sb,
            zeta_v,
            zeta,
            bnd,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        mul_bf(
            re,
            base + 1,
            sb,
            zeta_v,
            zeta,
            bnd,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        mul_bf(
            re,
            base + 2,
            sb,
            zeta_v,
            zeta,
            bnd,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        mul_bf(
            re,
            base + 3,
            sb,
            zeta_v,
            zeta,
            bnd,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
    }

    // Factored 8-quad build-out (mirror of Avx2NttTheory.build_out_impl, but over the
    // REAL `quad` muls): the impl-order 8 quads (= the 32 `mul` butterflies) + the
    // standalone bound + L6-value drivers.  Kept SEPARATE so the 7_6 fn's VC stays
    // lean (just the layer-atom composition).
    #[inline(always)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"#)]
    #[hax_lib::requires(fstar!(r#"
        Avx2NttTheory.is_i32b_poly_avx2 8380416 $re /\
        $field_modulus ==
          Libcrux_intrinsics.Avx2.mm256_set1_epi32 Libcrux_ml_dsa.Constants.v_FIELD_MODULUS /\
        $inverse_of_modulus_mod_montgomery_r ==
          Libcrux_intrinsics.Avx2.mm256_set1_epi32
            (cast (Libcrux_ml_dsa.Simd.Traits.v_INVERSE_OF_MODULUS_MOD_MONTGOMERY_R <: u64) <: i32)
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        (let mid = Avx2NttTheory.build_mid_L7 $re in
         Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 2*8380416) ${re}_future /\
         (forall (u:nat). (u % 16 < 8) /\ u < 32 ==>
            (let zL6 = (if u < 16 then mk_i32 (zeta_r 2) else mk_i32 (zeta_r 3)) in
             let (nu0,nu1) = Avx2NttTheory.bf_pair (Seq.index mid u) (Seq.index mid (u+8)) zL6 in
             Seq.index ${re}_future u == nu0 /\ Seq.index ${re}_future (u+8) == nu1)))
    "#))]
    fn build_out(
        re: &mut AVX2RingElement,
        field_modulus: Vec256,
        inverse_of_modulus_mod_montgomery_r: Vec256,
    ) {
        let zeta7 = mm256_set1_epi32(25847);
        let zeta60 = mm256_set1_epi32(-2608894);
        let zeta61 = mm256_set1_epi32(-518909);
        #[cfg(hax)]
        let orig = re.clone();
        hax_lib::fstar!(
            r#"
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 1 == 25847);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 2 == (-2608894));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 3 == (-518909));
        Avx2NttTheory.lemma_poly_to_units 8380416 orig
    "#
        );
        quad(
            re,
            0,
            STEP_BY_7,
            zeta7,
            25847,
            8380416,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qa0 = re.clone();
        quad(
            re,
            8,
            STEP_BY_7,
            zeta7,
            25847,
            8380416,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qa1 = re.clone();
        quad(
            re,
            0,
            STEP_BY_6,
            zeta60,
            -2608894,
            16760832,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qb0 = re.clone();
        quad(
            re,
            16,
            STEP_BY_6,
            zeta61,
            -518909,
            16760832,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qb1 = re.clone();
        quad(
            re,
            4,
            STEP_BY_7,
            zeta7,
            25847,
            8380416,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qc0 = re.clone();
        quad(
            re,
            12,
            STEP_BY_7,
            zeta7,
            25847,
            8380416,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qc1 = re.clone();
        quad(
            re,
            4,
            STEP_BY_6,
            zeta60,
            -2608894,
            16760832,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        #[cfg(hax)]
        let qd0 = re.clone();
        quad(
            re,
            20,
            STEP_BY_6,
            zeta61,
            -518909,
            16760832,
            field_modulus,
            inverse_of_modulus_mod_montgomery_r,
        );
        hax_lib::fstar!(
            r#"
        q76_bound orig qa0 qa1 qb0 qb1 qc0 qc1 qd0 ${re};
        Avx2NttTheory.lemma_units_to_poly (8380416 + 2*8380416) ${re};
        let mid = Avx2NttTheory.build_mid_L7 orig in
        Avx2NttTheory.lemma_poly_to_units (8380416 + 8380416) mid;
        q76_l6_value orig mid qa0 qa1 qb0 qb1 qc0 qc1 qd0 ${re}
    "#
        );
    }

    #[cfg(hax)]
    let orig = re.clone();

    // IN-BODY over the REAL 32 interleaved Montgomery butterflies (8 quads of 4 `mul`
    // each, via `build_out`).  Ghost grouped-all-L7 `mid` feeds the layer atoms;
    // `build_out` gives the real out's bound + L6-value (bf_pair-of-mid); the layer
    // atoms chain into comp_7_6_done.  No fstar::replace; Avx2NttTheory.build_out_impl
    // is NOT called (its logic is replicated over the real muls).
    build_out(re, field_modulus, inverse_of_modulus_mod_montgomery_r);

    hax_lib::fstar!(
        r#"
    let mid = Avx2NttTheory.build_mid_L7 orig in
    Avx2NttTheory.lemma_poly_to_units (8380416 + 8380416) mid;
    Avx2NttTheory.lemma_poly_to_units 8380416 orig;
    Avx2NttTheory.lemma_L7_atoms orig mid;
    Avx2NttTheory.lemma_L6_atoms mid ${re};
    Avx2NttTheory.lemma_compose_7_6_o orig mid ${re}
"#
    );
}

/// Layer 5, 4, 3
///
/// Each layer does 16 Montgomery multiplications -> 3*16 = 48 total
/// pqclean does 4 * 4 on each layer -> 48 total | plus 4 * 4 shuffles every time (48)
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[allow(unsafe_code)]
#[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 400"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 2*8380416) $re
"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Avx2NttTheory.comp_5_3_done $re ${re}_future /\
    Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 5*8380416) ${re}_future
"#))]
unsafe fn ntt_at_layer_5_to_3(re: &mut AVX2RingElement) {
    #[inline(always)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"#)]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        (v v_STEP == 8 \/ v v_STEP == 16 \/ v v_STEP == 32) /\
        v v_STEP_BY == v v_STEP / 8 /\
        v $index < 128 / v v_STEP /\
        Spec.Utils.is_i32b 4190208 $zeta /\
        (let offset = ((v $index) * (v v_STEP) * 2) / 8 in
         offset + 2 * (v v_STEP_BY) <= 32 /\
         (Spec.Utils.forall32 (fun i ->
            (i >= offset /\ i < offset + 2 * (v v_STEP_BY)) ==>
            Avx2NttTheory.is_i32b_unit_avx2
              (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v v_STEP_BY)) * 8380416)
              (Seq.index $re i))))
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        let offset = ((v $index) * (v v_STEP) * 2) / 8 in
        Avx2NttTheory.round_post_avx2 $re ${re}_future offset (v v_STEP_BY) $zeta
    "#))]
    fn round<const STEP: usize, const STEP_BY: usize>(
        re: &mut AVX2RingElement,
        index: usize,
        zeta: i32,
    ) {
        // Per-pair butterfly: the windowed round's inner fold step, factored into its
        // own VC for proof performance (mirrors Portable.Ntt.outer_3_plus__round;
        // runtime-identical via #[inline(always)]).  Body = the REAL butterfly
        // (mont-mul on unit index+step_by, then sub/add cross) + the
        // unit_post_cross_avx2 producer proof.
        #[inline(always)]
        #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"#)]
        #[hax_lib::requires(fstar!(r#"
            v $step_by > 0 /\
            v $index + v $step_by < 32 /\
            Spec.Utils.is_i32b 4190208 $zeta /\
            Avx2NttTheory.is_i32b_unit_avx2
              (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v $step_by)) * 8380416)
              (Seq.index $re (v $index)) /\
            Avx2NttTheory.is_i32b_unit_avx2
              (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v $step_by)) * 8380416)
              (Seq.index $re (v $index + v $step_by))
        "#))]
        #[hax_lib::ensures(|result| fstar!(r#"
            Spec.Utils.modifies2_32 $re ${re}_future $index ($index +! $step_by) /\
            Avx2NttTheory.is_i32b_unit_avx2
              (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v $step_by) + 1) * 8380416)
              (Seq.index ${re}_future (v $index)) /\
            Avx2NttTheory.is_i32b_unit_avx2
              (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v $step_by) + 1) * 8380416)
              (Seq.index ${re}_future (v $index + v $step_by)) /\
            Avx2NttTheory.unit_post_cross_avx2
              (Seq.index (Avx2NttTheory.chunks_of_re_avx2 $re) (v $index))
              (Seq.index (Avx2NttTheory.chunks_of_re_avx2 $re) (v $index + v $step_by))
              (Seq.index (Avx2NttTheory.chunks_of_re_avx2 ${re}_future) (v $index))
              (Seq.index (Avx2NttTheory.chunks_of_re_avx2 ${re}_future) (v $index + v $step_by))
              $zeta
        "#))]
        fn butterfly(re: &mut AVX2RingElement, index: usize, step_by: usize, zeta: i32) {
            let rhs = mm256_set1_epi32(zeta);
            #[cfg(hax)]
            let re_old = re.clone();
            // ---- REAL butterfly (the original round-loop step) ----
            arithmetic::montgomery_multiply(&mut re[index + step_by].value, &rhs);
            let tmp = mm256_sub_epi32(re[index].value, re[index + step_by].value);
            re[index] = AVX2SIMDUnit {
                value: mm256_add_epi32(re[index].value, re[index + step_by].value),
            };
            re[index + step_by] = AVX2SIMDUnit { value: tmp };
            // ---- per-pair proof: ntt_step fact -> cross-pair relations + output bounds ----
            hax_lib::fstar!(
                r#"
            let bnd : nat = 8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v $step_by)) * 8380416 in
            let re0 = (Seq.index re_old (v $index)).f_value in
            let re1 = (Seq.index re_old (v $index + v $step_by)).f_value in
            let nre0 = (Seq.index ${re} (v $index)).f_value in
            let nre1 = (Seq.index ${re} (v $index + v $step_by)).f_value in
            let lane (i:nat{i<8}) : Lemma
                ((to_i32x8 nre0 (mk_u64 i), to_i32x8 nre1 (mk_u64 i)) ==
                 ntt_step (to_i32x8 $rhs (mk_int i))
                   (to_i32x8 re0 (mk_u64 i), to_i32x8 re1 (mk_u64 i)))
              = assert (to_i32x8 $rhs (mk_int i) == $zeta);
                assert (to_i32x8 $rhs (mk_u64 i) == $zeta)
            in
            lane 0; lane 1; lane 2; lane 3; lane 4; lane 5; lane 6; lane 7;
            introduce forall (l:nat). l < 8 ==> to_i32x8 $rhs (mk_int l) == $zeta
            with (introduce l < 8 ==> to_i32x8 $rhs (mk_int l) == $zeta
                  with _. assert (to_i32x8 $rhs (mk_int l) == $zeta));
            Avx2NttTheory.lemma_cross_pair_relations_ws re_old ${re} bnd (v $index) (v $index + v $step_by) $rhs $zeta;
            Avx2NttTheory.lemma_is_i32b_unit_avx2_intro (bnd + 8380416) (Seq.index ${re} (v $index));
            Avx2NttTheory.lemma_is_i32b_unit_avx2_intro (bnd + 8380416) (Seq.index ${re} (v $index + v $step_by))
        "#
            );
        }

        let offset = (index * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT;

        // raw forall32 input bound (matches the requires); no win_bounded reveal needed
        #[cfg(hax)]
        let orig_re = re.clone();

        for j in offset..offset + STEP_BY {
            hax_lib::loop_invariant!(|j: usize| fstar!(
                r#"
                (Spec.Utils.modifies_range2_32 $orig_re ${re}
                    $offset $j
                    ($offset +! v_STEP_BY) ($j +! v_STEP_BY)) /\
                (Spec.Utils.forall32 (fun i ->
                        ((i >= v $offset /\ i < v $j) \/
                          (i >= v $offset + v v_STEP_BY /\ i < v $j + v v_STEP_BY)) ==>
                        Avx2NttTheory.is_i32b_unit_avx2
                          (8380416 + ((Avx2NttTheory.layer_bound_factor_avx2 (v v_STEP_BY)) + 1) * 8380416)
                          (Seq.index ${re} i))) /\
                (Spec.Utils.forall32 (fun u ->
                        (u >= v $offset /\ u < v $j) ==>
                        Avx2NttTheory.unit_post_cross_avx2 (Seq.index (Avx2NttTheory.chunks_of_re_avx2 $orig_re) u)
                          (Seq.index (Avx2NttTheory.chunks_of_re_avx2 $orig_re) (u + v v_STEP_BY))
                          (Seq.index (Avx2NttTheory.chunks_of_re_avx2 ${re}) u)
                          (Seq.index (Avx2NttTheory.chunks_of_re_avx2 ${re}) (u + v v_STEP_BY))
                          $zeta))
            "#
            ));
            #[cfg(hax)]
            let re_old = re.clone();
            hax_lib::fstar!(
                r#"
            assert (Seq.index re_old (v $j) == Seq.index $orig_re (v $j));
            assert (Seq.index re_old (v $j + v v_STEP_BY) == Seq.index $orig_re (v $j + v v_STEP_BY));
            assert (Avx2NttTheory.is_i32b_unit_avx2
                      (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v v_STEP_BY)) * 8380416)
                      (Seq.index re_old (v $j)));
            assert (Avx2NttTheory.is_i32b_unit_avx2
                      (8380416 + (Avx2NttTheory.layer_bound_factor_avx2 (v v_STEP_BY)) * 8380416)
                      (Seq.index re_old (v $j + v v_STEP_BY)))
        "#
            );
            // ---- per-pair butterfly (factored; proves its cross post in its own VC) ----
            butterfly(re, j, STEP_BY, zeta);
            hax_lib::fstar!(
                r#"
            Avx2NttTheory.lemma_round_ws_maintains $orig_re re_old ${re} $offset $j v_STEP_BY
              (Avx2NttTheory.layer_bound_factor_avx2 (v v_STEP_BY)) $zeta
        "#
            );
        }

        // ---- tail seal: completed transparent invariant -> round_post_avx2 ----
        hax_lib::fstar!(
            r#"
        let lbf : nat = Avx2NttTheory.layer_bound_factor_avx2 (v v_STEP_BY) in
        let offset_n : nat = ((v $index) * (v v_STEP) * 2) / 8 in
        // (a) modifies: the two adjacent ranges union to [offset, offset+2*STEP_BY)
        assert (Spec.Utils.modifies_range_32 $orig_re ${re}
                  (mk_usize offset_n) (mk_usize (offset_n + 2 * (v v_STEP_BY))));
        Avx2NttTheory.lemma_range32_to_modwin $orig_re ${re}
          (mk_usize offset_n) (mk_usize (offset_n + 2 * (v v_STEP_BY)));
        // (b) win_bounded on the whole window [offset, offset+2*STEP_BY): the completed
        // invariant's disjoint two-half bound merges to the contiguous window.
        assert (Spec.Utils.forall32 (fun i ->
                  (i >= offset_n /\ i < offset_n + 2 * (v v_STEP_BY)) ==>
                  Avx2NttTheory.is_i32b_unit_avx2 (8380416 + (lbf + 1) * 8380416) (Seq.index ${re} i)));
        Avx2NttTheory.lemma_win_bounded_from_forall32 ${re} offset_n (2 * (v v_STEP_BY))
          (8380416 + (lbf + 1) * 8380416);
        // (c) win_cross on the lo-half [offset, offset+STEP_BY)
        Avx2NttTheory.lemma_win_cross_from_forall32 $orig_re ${re} offset_n (v v_STEP_BY) $zeta;
        Avx2NttTheory.lemma_rp_intro $orig_re ${re} offset_n (v v_STEP_BY) $zeta
    "#
        );
    }

    // Layer 5 block (window-scoped; rounds at offsets 0/8/16/24, zetas zeta_r(4..7))
    #[inline(always)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 800"#)]
    #[hax_lib::fstar::before(
        r#"
(* Local block helper: derive the real round's raw forall32 input bound over a
   window [lo,lo+width) from the block's whole-poly bound + the modifies frame.
   Replaces the theory's win_bounded-producing lemma_window_from_modwin, since the
   in-body round consumes the raw forall32 form (not the sealed win_bounded atom). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_window_forall32_from_modwin
      (orig cur: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (lo width bnd:nat)
    : Lemma
      (requires
        Avx2NttTheory.modifies_win orig cur 0 lo /\ lo + width <= 32 /\
        Avx2NttTheory.is_i32b_poly_avx2 bnd orig)
      (ensures
        Spec.Utils.forall32 (fun i -> (i >= lo /\ i < lo + width) ==>
          Avx2NttTheory.is_i32b_unit_avx2 bnd (Seq.index cur i))) =
  Avx2NttTheory.lemma_poly_to_units bnd orig;
  let aux (i:nat{i<32})
      : Lemma ((i >= lo /\ i < lo + width) ==>
               Avx2NttTheory.is_i32b_unit_avx2 bnd (Seq.index cur i)) =
    if i >= lo && i < lo + width
    then Avx2NttTheory.lemma_modwin_lookup orig cur 0 lo i
    else ()
  in
  Classical.forall_intro aux
#pop-options
"#
    )]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 2*8380416) $re
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        Avx2NttTheory.layer_done $re ${re}_future 5 /\
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 3*8380416) ${re}_future
    "#))]
    fn l5_block(re: &mut AVX2RingElement) {
        // 0: 0, 1, 2, 3
        // 1: 8, 9, 10, 11
        // 2: 16, 17, 18, 19
        // 3: 24, 25, 26, 27
        const STEP: usize = 1 << 5;
        const STEP_BY: usize = STEP / COEFFICIENTS_IN_SIMD_UNIT;

        #[cfg(hax)]
        let orig = re.clone();
        // round 0
        hax_lib::fstar!(
            r#"
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 4 == 237124);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 5 == (-777960));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 6 == (-876248));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 7 == 466468);
        Avx2NttTheory.lemma_modwin_refl orig 0 0;
        lemma_window_forall32_from_modwin orig orig 0 8 (8380416 + 2*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 0, 237124);
        #[cfg(hax)]
        let s1 = re.clone();
        // round 1
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin orig s1 0 4 (mk_i32 237124);
        Avx2NttTheory.lemma_modwin_union orig orig s1 0 0 8;
        lemma_window_forall32_from_modwin orig s1 8 8 (8380416 + 2*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 1, -777960);
        #[cfg(hax)]
        let s2 = re.clone();
        // round 2
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s1 s2 8 4 (mk_i32 (-777960));
        Avx2NttTheory.lemma_modwin_union orig s1 s2 0 8 16;
        lemma_window_forall32_from_modwin orig s2 16 8 (8380416 + 2*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 2, -876248);
        #[cfg(hax)]
        let s3 = re.clone();
        // round 3
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s2 s3 16 4 (mk_i32 (-876248));
        Avx2NttTheory.lemma_modwin_union orig s2 s3 0 16 24;
        lemma_window_forall32_from_modwin orig s3 24 8 (8380416 + 2*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 3, 466468);
        hax_lib::fstar!(r#"Avx2NttTheory.lemma_l5_assemble_o orig s1 s2 s3 ${re}"#);
    }

    // Layer 4 block (rounds at offsets 4k, zetas zeta_r(8..15))
    #[inline(always)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 800"#)]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 3*8380416) $re
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        Avx2NttTheory.layer_done $re ${re}_future 4 /\
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 4*8380416) ${re}_future
    "#))]
    fn l4_block(re: &mut AVX2RingElement) {
        // 0: 0, 1
        // 1: 4, 5
        // 2: 8, 9
        // 3: 12, 13
        // 4: 16, 17
        // 5: 20, 21
        // 6: 24, 25
        // 7: 28, 29
        const STEP: usize = 1 << 4;
        const STEP_BY: usize = STEP / COEFFICIENTS_IN_SIMD_UNIT;

        #[cfg(hax)]
        let orig = re.clone();
        // round 0
        hax_lib::fstar!(
            r#"
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 8 == 1826347);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 9 == 2353451);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 10 == (-359251));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 11 == (-2091905));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 12 == 3119733);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 13 == (-2884855));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 14 == 3111497);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 15 == 2680103);
        Avx2NttTheory.lemma_modwin_refl orig 0 0;
        lemma_window_forall32_from_modwin orig orig 0 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 0, 1826347);
        #[cfg(hax)]
        let s1 = re.clone();
        // round 1
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin orig s1 0 2 (mk_i32 1826347);
        Avx2NttTheory.lemma_modwin_union orig orig s1 0 0 4;
        lemma_window_forall32_from_modwin orig s1 4 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 1, 2353451);
        #[cfg(hax)]
        let s2 = re.clone();
        // round 2
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s1 s2 4 2 (mk_i32 2353451);
        Avx2NttTheory.lemma_modwin_union orig s1 s2 0 4 8;
        lemma_window_forall32_from_modwin orig s2 8 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 2, -359251);
        #[cfg(hax)]
        let s3 = re.clone();
        // round 3
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s2 s3 8 2 (mk_i32 (-359251));
        Avx2NttTheory.lemma_modwin_union orig s2 s3 0 8 12;
        lemma_window_forall32_from_modwin orig s3 12 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 3, -2091905);
        #[cfg(hax)]
        let s4 = re.clone();
        // round 4
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s3 s4 12 2 (mk_i32 (-2091905));
        Avx2NttTheory.lemma_modwin_union orig s3 s4 0 12 16;
        lemma_window_forall32_from_modwin orig s4 16 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 4, 3119733);
        #[cfg(hax)]
        let s5 = re.clone();
        // round 5
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s4 s5 16 2 (mk_i32 3119733);
        Avx2NttTheory.lemma_modwin_union orig s4 s5 0 16 20;
        lemma_window_forall32_from_modwin orig s5 20 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 5, -2884855);
        #[cfg(hax)]
        let s6 = re.clone();
        // round 6
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s5 s6 20 2 (mk_i32 (-2884855));
        Avx2NttTheory.lemma_modwin_union orig s5 s6 0 20 24;
        lemma_window_forall32_from_modwin orig s6 24 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 6, 3111497);
        #[cfg(hax)]
        let s7 = re.clone();
        // round 7
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s6 s7 24 2 (mk_i32 3111497);
        Avx2NttTheory.lemma_modwin_union orig s6 s7 0 24 28;
        lemma_window_forall32_from_modwin orig s7 28 4 (8380416 + 3*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 7, 2680103);
        hax_lib::fstar!(r#"Avx2NttTheory.lemma_l4_assemble_o orig s1 s2 s3 s4 s5 s6 s7 ${re}"#);
    }

    // Layer 3 block (rounds at offsets 2k, zetas zeta_r(16..31))
    #[inline(always)]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 800"#)]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 4*8380416) $re
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        Avx2NttTheory.layer_done $re ${re}_future 3 /\
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 5*8380416) ${re}_future
    "#))]
    fn l3_block(re: &mut AVX2RingElement) {
        // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
        const STEP: usize = 1 << 3;
        const STEP_BY: usize = STEP / COEFFICIENTS_IN_SIMD_UNIT;

        #[cfg(hax)]
        let orig = re.clone();
        // round 0
        hax_lib::fstar!(
            r#"
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 16 == 2725464);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 17 == 1024112);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 18 == (-1079900));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 19 == 3585928);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 20 == (-549488));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 21 == (-1119584));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 22 == 2619752);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 23 == (-2108549));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 24 == (-2118186));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 25 == (-3859737));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 26 == (-1399561));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 27 == (-3277672));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 28 == 1757237);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 29 == (-19422));
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 30 == 4010497);
        assert_norm (Spec.MLDSA.NttConstants.zeta_r 31 == 280005);
        Avx2NttTheory.lemma_modwin_refl orig 0 0;
        lemma_window_forall32_from_modwin orig orig 0 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 0, 2725464);
        #[cfg(hax)]
        let s1 = re.clone();
        // round 1
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin orig s1 0 1 (mk_i32 2725464);
        Avx2NttTheory.lemma_modwin_union orig orig s1 0 0 2;
        lemma_window_forall32_from_modwin orig s1 2 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 1, 1024112);
        #[cfg(hax)]
        let s2 = re.clone();
        // round 2
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s1 s2 2 1 (mk_i32 1024112);
        Avx2NttTheory.lemma_modwin_union orig s1 s2 0 2 4;
        lemma_window_forall32_from_modwin orig s2 4 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 2, -1079900);
        #[cfg(hax)]
        let s3 = re.clone();
        // round 3
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s2 s3 4 1 (mk_i32 (-1079900));
        Avx2NttTheory.lemma_modwin_union orig s2 s3 0 4 6;
        lemma_window_forall32_from_modwin orig s3 6 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 3, 3585928);
        #[cfg(hax)]
        let s4 = re.clone();
        // round 4
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s3 s4 6 1 (mk_i32 3585928);
        Avx2NttTheory.lemma_modwin_union orig s3 s4 0 6 8;
        lemma_window_forall32_from_modwin orig s4 8 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 4, -549488);
        #[cfg(hax)]
        let s5 = re.clone();
        // round 5
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s4 s5 8 1 (mk_i32 (-549488));
        Avx2NttTheory.lemma_modwin_union orig s4 s5 0 8 10;
        lemma_window_forall32_from_modwin orig s5 10 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 5, -1119584);
        #[cfg(hax)]
        let s6 = re.clone();
        // round 6
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s5 s6 10 1 (mk_i32 (-1119584));
        Avx2NttTheory.lemma_modwin_union orig s5 s6 0 10 12;
        lemma_window_forall32_from_modwin orig s6 12 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 6, 2619752);
        #[cfg(hax)]
        let s7 = re.clone();
        // round 7
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s6 s7 12 1 (mk_i32 2619752);
        Avx2NttTheory.lemma_modwin_union orig s6 s7 0 12 14;
        lemma_window_forall32_from_modwin orig s7 14 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 7, -2108549);
        #[cfg(hax)]
        let s8 = re.clone();
        // round 8
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s7 s8 14 1 (mk_i32 (-2108549));
        Avx2NttTheory.lemma_modwin_union orig s7 s8 0 14 16;
        lemma_window_forall32_from_modwin orig s8 16 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 8, -2118186);
        #[cfg(hax)]
        let s9 = re.clone();
        // round 9
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s8 s9 16 1 (mk_i32 (-2118186));
        Avx2NttTheory.lemma_modwin_union orig s8 s9 0 16 18;
        lemma_window_forall32_from_modwin orig s9 18 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 9, -3859737);
        #[cfg(hax)]
        let s10 = re.clone();
        // round 10
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s9 s10 18 1 (mk_i32 (-3859737));
        Avx2NttTheory.lemma_modwin_union orig s9 s10 0 18 20;
        lemma_window_forall32_from_modwin orig s10 20 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 10, -1399561);
        #[cfg(hax)]
        let s11 = re.clone();
        // round 11
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s10 s11 20 1 (mk_i32 (-1399561));
        Avx2NttTheory.lemma_modwin_union orig s10 s11 0 20 22;
        lemma_window_forall32_from_modwin orig s11 22 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 11, -3277672);
        #[cfg(hax)]
        let s12 = re.clone();
        // round 12
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s11 s12 22 1 (mk_i32 (-3277672));
        Avx2NttTheory.lemma_modwin_union orig s11 s12 0 22 24;
        lemma_window_forall32_from_modwin orig s12 24 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 12, 1757237);
        #[cfg(hax)]
        let s13 = re.clone();
        // round 13
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s12 s13 24 1 (mk_i32 1757237);
        Avx2NttTheory.lemma_modwin_union orig s12 s13 0 24 26;
        lemma_window_forall32_from_modwin orig s13 26 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 13, -19422);
        #[cfg(hax)]
        let s14 = re.clone();
        // round 14
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s13 s14 26 1 (mk_i32 (-19422));
        Avx2NttTheory.lemma_modwin_union orig s13 s14 0 26 28;
        lemma_window_forall32_from_modwin orig s14 28 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 14, 4010497);
        #[cfg(hax)]
        let s15 = re.clone();
        // round 15
        hax_lib::fstar!(
            r#"
        Avx2NttTheory.lemma_rp_modwin s14 s15 28 1 (mk_i32 4010497);
        Avx2NttTheory.lemma_modwin_union orig s14 s15 0 28 30;
        lemma_window_forall32_from_modwin orig s15 30 2 (8380416 + 4*8380416)
    "#
        );
        round::<STEP, STEP_BY>(re, 15, 280005);
        hax_lib::fstar!(
            r#"Avx2NttTheory.lemma_l3_assemble_o orig s1 s2 s3 s4 s5 s6 s7 s8 s9 s10 s11 s12 s13 s14 s15 ${re}"#
        );
    }

    #[cfg(hax)]
    let orig = re.clone();
    l5_block(re);
    #[cfg(hax)]
    let s_l5 = re.clone();
    l4_block(re);
    #[cfg(hax)]
    let s_l4 = re.clone();
    l3_block(re);
    hax_lib::fstar!(r#"Avx2NttTheory.lemma_compose_5_3_o orig s_l5 s_l4 ${re}"#);
}

#[allow(unsafe_code)]
#[inline(always)]
#[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 400"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Avx2NttTheory.is_i32b_poly_avx2 8380416 $re
"#))]
#[hax_lib::ensures(|result| fstar!(r#"
    Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 8*8380416) ${re}_future /\
    (let in_flat = C.simd_units_to_array (Avx2NttTheory.chunks_of_re_avx2 $re) in
     let out_flat = C.simd_units_to_array (Avx2NttTheory.chunks_of_re_avx2 ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt in_flat in
     forall (i: nat). i < 256 ==>
       v (Seq.index out_flat i) % 8380417 == v (Seq.index spec i) % 8380417)
"#))]
pub(crate) fn ntt(re: &mut AVX2RingElement) {
    #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
    #[hax_lib::fstar::options(r#"--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"#)]
    #[hax_lib::fstar::before(
        r#"
(* CHUNKS-EQ bridge helper: the LOCAL `chunks_of_re_avx2` (Ntt.fst) and the
   theory's `Avx2NttTheory.chunks_of_re_avx2` are the SAME function — both
   project lane (b,l) to `to_i32x8 (Seq.index re b).f_value (mk_u64 l)`.  We
   prove pointwise equality from BOTH index lemmas, then Seq.lemma_eq_intro at
   the inner and outer levels.  This lets the L2/L1/L0 LOCAL congruences (over
   the local chunks) feed `Avx2NttTheory.lemma_layer_done_intro` (over the
   theory chunks). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_chunks_eq (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (Avx2NttTheory.chunks_of_re_avx2 re == chunks_of_re_avx2 re) =
  let cT = Avx2NttTheory.chunks_of_re_avx2 re in
  let cL = chunks_of_re_avx2 re in
  let aux (b: nat{b < 32}) : Lemma (Seq.index cT b == Seq.index cL b) =
    let auxl (l: nat{l < 8}) : Lemma (Seq.index (Seq.index cT b) l == Seq.index (Seq.index cL b) l) =
      Avx2NttTheory.lemma_chunks_of_re_avx2_index re b l;
      lemma_chunks_of_re_avx2_index re b l
    in
    Classical.forall_intro auxl;
    Seq.lemma_eq_intro (Seq.index cT b) (Seq.index cL b)
  in
  Classical.forall_intro aux;
  Seq.lemma_eq_intro cT cL
#pop-options
"#
    )]
    #[hax_lib::requires(fstar!(r#"
        Avx2NttTheory.is_i32b_poly_avx2 8380416 $re
    "#))]
    #[hax_lib::ensures(|result| fstar!(r#"
        Avx2NttTheory.is_i32b_poly_avx2 (8380416 + 8*8380416) ${re}_future /\
        (let in_flat = C.simd_units_to_array (Avx2NttTheory.chunks_of_re_avx2 $re) in
         let out_flat = C.simd_units_to_array (Avx2NttTheory.chunks_of_re_avx2 ${re}_future) in
         let spec = Hacspec_ml_dsa.Ntt.ntt in_flat in
         forall (i: nat). i < 256 ==>
           v (Seq.index out_flat i) % 8380417 == v (Seq.index spec i) % 8380417)
    "#))]
    unsafe fn avx2_ntt(re: &mut AVX2RingElement) {
        hax_lib::fstar!(
            r#"
        assert_norm (v Libcrux_ml_dsa.Simd.Traits.Specs.v_NTT_BASE_BOUND == 8380416);
        assert_norm (v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX == 8380416)
    "#
        );
        #[cfg(hax)]
        let orig = re.clone();
        ntt_at_layer_7_and_6(re);
        #[cfg(hax)]
        let s76 = re.clone();
        ntt_at_layer_5_to_3(re);
        #[cfg(hax)]
        let s53 = re.clone();
        // SEAM bridge: theory is_i32b_poly_avx2 (+5) s53 -> LOCAL is_i32b_poly_avx2 (+5) s53
        hax_lib::fstar!(
            r#"
        let seam (u:nat{u<32}) (l:nat{l<8})
            : Lemma (Spec.Utils.is_i32b (8380416 + 5*8380416)
                       (to_i32x8 (Seq.index s53 u).f_value (mk_u64 l))) =
          Avx2NttTheory.lemma_is_i32b_poly_avx2_elim (8380416 + 5*8380416) s53 u l
        in
        Classical.forall_intro_2 seam;
        lemma_is_i32b_poly_avx2_intro (8380416 + 5*8380416) s53
    "#
        );
        ntt_at_layer_2(re);
        #[cfg(hax)]
        let s2 = re.clone();
        ntt_at_layer_1(re);
        #[cfg(hax)]
        let s1 = re.clone();
        ntt_at_layer_0(re);
        // CHUNKS bridge (3x): LOCAL congruence -> Avx2NttTheory.layer_done. The L2/L1/L0
        // ensures give the congruence over the LOCAL chunks; rewrite both sides to the
        // theory chunks via lemma_chunks_eq, then fire lemma_layer_done_intro.  Then the
        // top compose + reveal, and the FINAL bound bridge (LOCAL -> theory).
        hax_lib::fstar!(
            r#"
        lemma_chunks_eq s53; lemma_chunks_eq s2; lemma_chunks_eq s1; lemma_chunks_eq ${re};
        Avx2NttTheory.lemma_layer_done_intro s53 s2 2;
        Avx2NttTheory.lemma_layer_done_intro s2 s1 1;
        Avx2NttTheory.lemma_layer_done_intro s1 ${re} 0;
        Avx2NttTheory.lemma_ntt_top_compose_o orig s76 s53 s2 s1 ${re};
        Avx2NttTheory.lemma_ntt_done_reveal orig ${re};
        let fin (u:nat{u<32}) (l:nat{l<8})
            : Lemma (Spec.Utils.is_i32b (8380416 + 8*8380416)
                       (to_i32x8 (Seq.index ${re} u).f_value (mk_u64 l))) =
          lemma_is_i32b_poly_avx2_elim (8380416 + 8*8380416) ${re} u l
        in
        Classical.forall_intro_2 fin;
        Avx2NttTheory.lemma_is_i32b_poly_avx2_intro (8380416 + 8*8380416) ${re}
    "#
        );
    }

    unsafe { avx2_ntt(re) }
}

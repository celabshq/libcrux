module Avx2NttTheory
(* ABSTRACT interface for the AVX2 NTT opaque-atom theory.  Consumers (the block
   fns / layer fns / top compose in the extracted Ntt.fst) open this and see ONLY
   abstract `val` atoms + lemma signatures — so their module context stays small
   and the t_Array/t_Vec256 refinement reasoning over 16 array snapshots does NOT
   cascade (validated: rlimit ~0.7 vs 800-saturate when the defs are in-module).
   The definitions + lemma bodies live in Avx2NttTheory.fst. *)
#set-options "--fuel 0 --ifuel 1 --z3rlimit 50"
open FStar.Mul
open Core_models
open Spec.Intrinsics
open Spec.MLDSA.NttConstants
open Spec.MLDSA.Math

module C = Hacspec_ml_dsa.Commute.Chunk

unfold let av32 = t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32)

(* transparent: consumers need its concrete values (1->4, 2->3, 4->2) to match bounds. *)
let layer_bound_factor_avx2 (step_by:nat) : n:nat{n <= 4} =
  if step_by = 1 then 4 else if step_by = 2 then 3 else if step_by = 4 then 2 else 0

(* ============================================================================
   PHASE 4 BACKPORT — additional theory decls the extracted Ntt.fst consumer
   calls.  All are EXISTING `let`s in Avx2NttTheory.fst; exposed here as abstract
   `val`s (bodies stay in the .fst).  Each `val` is placed so its relative order
   matches the `let` order in the .fst (F* Error 233 otherwise).
   ============================================================================ *)

(* chunk view (precedes is_i32b_poly_avx2 in the .fst). *)
val chunks_of_re_avx2
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32)

val lemma_chunks_of_re_avx2_index
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (b: nat{b < 32}) (l: nat{l < 8})
    : Lemma (Seq.index (Seq.index (chunks_of_re_avx2 re) b) l ==
             to_i32x8 (Seq.index re b).f_value (mk_u64 l))

(* ---- abstract opaque atoms ---- *)
val is_i32b_poly_avx2 : nat -> av32 -> Type0

(* poly/unit bound intro/elim + cross atom (all .fst-ordered between
   is_i32b_poly_avx2 and modifies_win). *)
val lemma_is_i32b_poly_avx2_elim (bnd:nat)
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (u:nat{u<32}) (l:nat{l<8})
    : Lemma (requires is_i32b_poly_avx2 bnd re)
            (ensures Spec.Utils.is_i32b bnd (to_i32x8 (Seq.index re u).f_value (mk_u64 l)))

val lemma_is_i32b_poly_avx2_intro (bnd:nat)
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires forall (u:nat) (l:nat). u<32 /\ l<8 ==>
               Spec.Utils.is_i32b bnd (to_i32x8 (Seq.index re u).f_value (mk_u64 l)))
            (ensures is_i32b_poly_avx2 bnd re)

val unit_post_cross_avx2 (ci_lo ci_hi co_lo co_hi : t_Array i32 (mk_usize 8))
                         (zeta: i32{Spec.Utils.is_i32b 4190208 zeta}) : Type0

val lemma_atom_to_bf_cross_avx2 (ci_lo ci_hi co_lo co_hi : t_Array i32 (mk_usize 8))
                                (zeta: i32{Spec.Utils.is_i32b 4190208 zeta})
    : Lemma (requires unit_post_cross_avx2 ci_lo ci_hi co_lo co_hi zeta)
            (ensures
              (forall (l: nat{l < 8}).
                (let t = mont_mul (Seq.index ci_hi l) zeta in
                 v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v t /\
                 v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v t /\
                 (v t) % 8380417 == (v (Seq.index ci_hi l) * v zeta * 8265825) % 8380417)))

val is_i32b_unit_avx2 (bnd:nat)
      (u: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) : Type0

val lemma_is_i32b_unit_avx2_elim (bnd:nat)
      (u: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) (l:nat{l<8})
    : Lemma (requires is_i32b_unit_avx2 bnd u)
            (ensures Spec.Utils.is_i32b bnd (to_i32x8 u.f_value (mk_u64 l)))

val lemma_is_i32b_unit_avx2_intro (bnd:nat)
      (u: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
    : Lemma (requires forall (l:nat). l < 8 ==>
               Spec.Utils.is_i32b bnd (to_i32x8 u.f_value (mk_u64 l)))
            (ensures is_i32b_unit_avx2 bnd u)

val lemma_poly_to_units (bnd:nat)
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires is_i32b_poly_avx2 bnd re)
            (ensures forall (u:nat). u < 32 ==> is_i32b_unit_avx2 bnd (Seq.index re u))

val lemma_units_to_poly (bnd:nat)
      (re: av32)
    : Lemma
      (requires (forall (u:nat). u < 32 ==> is_i32b_unit_avx2 bnd (Seq.index re u)))
      (ensures is_i32b_poly_avx2 bnd re)

(* ---- per-step ntt_step facts -> the round_ws__round pair post (cross atom +
   output bounds).  Exposed so the in-body 5_to_3 round fold can establish the
   per-butterfly cross atom from the inline ntt_step forall8 fact. ---- *)
val lemma_cross_pair_relations_ws
      (re re_fut: av32)
      (bnd:nat{bnd + 8380416 < pow2 31})
      (ulo uhi: nat{ulo < 32 /\ uhi < 32})
      (zeta_bv: Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256))
      (zeta_val: i32{Spec.Utils.is_i32b 4190208 zeta_val})
    : Lemma
        (requires
          is_i32b_unit_avx2 bnd (Seq.index re ulo) /\
          is_i32b_unit_avx2 bnd (Seq.index re uhi) /\
          (forall (l:nat). l < 8 ==> to_i32x8 zeta_bv (mk_int l) == zeta_val) /\
          (let re0 = (Seq.index re ulo).f_value in
           let re1 = (Seq.index re uhi).f_value in
           let nre0 = (Seq.index re_fut ulo).f_value in
           let nre1 = (Seq.index re_fut uhi).f_value in
           Spec.Utils.forall8 (fun i ->
             (to_i32x8 nre0 (mk_u64 i), to_i32x8 nre1 (mk_u64 i)) ==
             ntt_step (to_i32x8 zeta_bv (mk_int i))
               (to_i32x8 re0 (mk_u64 i), to_i32x8 re1 (mk_u64 i)))))
        (ensures
          (let ci = chunks_of_re_avx2 re in
           let co = chunks_of_re_avx2 re_fut in
           unit_post_cross_avx2 (Seq.index ci ulo) (Seq.index ci uhi)
                                (Seq.index co ulo) (Seq.index co uhi) zeta_val) /\
          (forall (l:nat). l < 8 ==>
            Spec.Utils.is_i32b (bnd + 8380416) (to_i32x8 (Seq.index re_fut ulo).f_value (mk_u64 l)) /\
            Spec.Utils.is_i32b (bnd + 8380416) (to_i32x8 (Seq.index re_fut uhi).f_value (mk_u64 l))))

(* ---- KEYSTONE maintain: entry fold-invariant at j + the per-butterfly pair post
   on (j, j+step_by) -> exit fold-invariant at j+1.  Called once per fold step. ---- *)
val lemma_round_ws_maintains
      (orig_re re_old re_new: av32)
      (offset j step_by: usize)
      (lbf:nat{lbf <= 4})
      (zeta:i32{Spec.Utils.is_i32b 4190208 zeta})
    : Lemma
      (requires
        v step_by > 0 /\ v offset <= v j /\ v j < v offset + v step_by /\
        v offset + 2 * v step_by <= 32 /\
        Spec.Utils.modifies_range2_32 orig_re re_old offset j (offset +! step_by) (j +! step_by) /\
        (Spec.Utils.forall32 (fun i ->
            ((i >= v offset /\ i < v j) \/ (i >= v offset + v step_by /\ i < v j + v step_by)) ==>
            is_i32b_unit_avx2 (8380416 + (lbf + 1) * 8380416) (Seq.index re_old i))) /\
        (Spec.Utils.forall32 (fun u ->
            (u >= v offset /\ u < v j) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig_re) u)
              (Seq.index (chunks_of_re_avx2 orig_re) (u + v step_by))
              (Seq.index (chunks_of_re_avx2 re_old) u)
              (Seq.index (chunks_of_re_avx2 re_old) (u + v step_by)) zeta)) /\
        Spec.Utils.modifies2_32 re_old re_new j (j +! step_by) /\
        is_i32b_unit_avx2 (8380416 + (lbf + 1) * 8380416) (Seq.index re_new (v j)) /\
        is_i32b_unit_avx2 (8380416 + (lbf + 1) * 8380416) (Seq.index re_new (v j + v step_by)) /\
        unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 re_old) (v j))
          (Seq.index (chunks_of_re_avx2 re_old) (v j + v step_by))
          (Seq.index (chunks_of_re_avx2 re_new) (v j))
          (Seq.index (chunks_of_re_avx2 re_new) (v j + v step_by)) zeta)
      (ensures
        Spec.Utils.modifies_range2_32 orig_re re_new offset (j +! mk_usize 1)
          (offset +! step_by) ((j +! mk_usize 1) +! step_by) /\
        (Spec.Utils.forall32 (fun i ->
            ((i >= v offset /\ i < v j + 1) \/ (i >= v offset + v step_by /\ i < v j + 1 + v step_by)) ==>
            is_i32b_unit_avx2 (8380416 + (lbf + 1) * 8380416) (Seq.index re_new i))) /\
        (Spec.Utils.forall32 (fun u ->
            (u >= v offset /\ u < v j + 1) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig_re) u)
              (Seq.index (chunks_of_re_avx2 orig_re) (u + v step_by))
              (Seq.index (chunks_of_re_avx2 re_new) u)
              (Seq.index (chunks_of_re_avx2 re_new) (u + v step_by)) zeta)))

val modifies_win : av32 -> av32 -> nat -> nat -> Type0
val win_bounded : av32 -> nat -> nat -> nat -> Type0
val win_cross : a:av32 -> b:av32 -> offset:nat{offset < 32} ->
  step_by:nat{step_by > 0 /\ offset + 2*step_by <= 32} -> zeta:i32 -> Type0
val round_post_avx2 :
  a:av32 -> b:av32 -> offset:nat{offset < 32} ->
  step_by:nat{step_by > 0 /\ offset + 2*step_by <= 32} -> zeta:i32 -> Type0
val layer_done : av32 -> av32 -> (layer:nat{layer < 8}) -> Type0

(* ---- modifies frame algebra ---- *)
val lemma_modwin_refl (a: av32) (lo hi:nat)
    : Lemma (requires lo <= hi /\ hi <= 32) (ensures modifies_win a a lo hi)

val lemma_modwin_lookup (a b: av32) (lo hi:nat) (u:nat{u<32})
    : Lemma (requires modifies_win a b lo hi /\ (u < lo \/ u >= hi))
            (ensures Seq.index a u == Seq.index b u)

val lemma_modwin_union (a b c: av32) (lo mid hi:nat)
    : Lemma (requires modifies_win a b lo mid /\ modifies_win b c mid hi /\
                      lo <= mid /\ mid <= hi /\ hi <= 32)
            (ensures modifies_win a c lo hi)

(* ---- modifies_range_32 (transparent) -> modifies_win (opaque) seal bridge ---- *)
val lemma_range32_to_modwin (a b: av32) (i j:usize{v i < 32 /\ v j <= 32 /\ v i <= v j})
    : Lemma (requires Spec.Utils.modifies_range_32 a b i j)
            (ensures modifies_win a b (v i) (v j))

(* ---- window-bound discharge ---- *)
val lemma_window_from_modwin (orig cur: av32) (lo width bnd:nat)
    : Lemma (requires modifies_win orig cur 0 lo /\ lo + width <= 32 /\
                      is_i32b_poly_avx2 bnd orig)
            (ensures win_bounded cur lo width bnd)

(* layer_done intro (.fst-ordered after lemma_window_from_modwin, before lemma_rp_modwin). *)
val lemma_layer_done_intro (a b: av32) (layer:nat{layer < 8})
    : Lemma
      (requires
        (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 a) in
         let out_flat = C.simd_units_to_array (chunks_of_re_avx2 b) in
         let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize layer) in
         forall (i: nat). i < 256 ==>
           (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
      (ensures layer_done a b layer)

(* ---- round post intro from the 3 sealed sub-atoms (in-body round seal) ---- *)
val lemma_rp_intro
      (a b: av32)
      (offset:nat{offset < 32}) (step_by:nat{step_by > 0 /\ offset + 2*step_by <= 32})
      (zeta:i32)
    : Lemma
      (requires
        modifies_win a b offset (offset + 2*step_by) /\
        win_bounded b offset (2*step_by)
          (8380416 + (layer_bound_factor_avx2 step_by + 1) * 8380416) /\
        win_cross a b offset step_by zeta)
      (ensures round_post_avx2 a b offset step_by zeta)

(* ---- round post extraction ---- *)
val lemma_rp_modwin
      (a b: av32)
      (offset:nat{offset < 32}) (step_by:nat{step_by > 0 /\ offset + 2*step_by <= 32})
      (zeta:i32)
    : Lemma (requires round_post_avx2 a b offset step_by zeta)
            (ensures modifies_win a b offset (offset + 2*step_by))

(* ---- the producer: one window butterfly, sealed into a round_post atom ---- *)
val round_ws_sealed
      (v_STEP v_STEP_BY: usize)
      (re: av32)
      (index: usize)
      (zeta: i32)
    : Prims.Pure av32
      (requires
        (v v_STEP == 8 \/ v v_STEP == 16 \/ v v_STEP == 32) /\
        v v_STEP_BY == v v_STEP / 8 /\
        v index < 128 / v v_STEP /\
        Spec.Utils.is_i32b 4190208 zeta /\
        (let offset = ((v index) * (v v_STEP) * 2) / 8 in
         offset + 2 * (v v_STEP_BY) <= 32 /\
         win_bounded re offset (2 * (v v_STEP_BY))
           (8380416 + (layer_bound_factor_avx2 (v v_STEP_BY)) * 8380416)))
      (ensures
        fun re_future ->
          let offset = ((v index) * (v v_STEP) * 2) / 8 in
          round_post_avx2 re re_future offset (v v_STEP_BY) zeta)

(* r3o : the L3 round atom (offset 2k, step_by 1, zeta_r(k+16)). *)
unfold let r3o (a b: av32) (k:nat{k<16}) : Type0 =
  round_post_avx2 a b (2*k) 1 (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (k + 16)))

(* L3 assemble: 16 round posts -> the layer congruence atom + whole-poly bound. *)
val lemma_l3_assemble_o
      (orig s1 s2 s3 s4 s5 s6 s7 s8 s9 s10 s11 s12 s13 s14 s15 s16: av32)
    : Lemma
      (requires
        is_i32b_poly_avx2 (8380416 + 4*8380416) orig /\
        r3o orig s1 0 /\ r3o s1 s2 1 /\ r3o s2 s3 2 /\ r3o s3 s4 3 /\
        r3o s4 s5 4 /\ r3o s5 s6 5 /\ r3o s6 s7 6 /\ r3o s7 s8 7 /\
        r3o s8 s9 8 /\ r3o s9 s10 9 /\ r3o s10 s11 10 /\ r3o s11 s12 11 /\
        r3o s12 s13 12 /\ r3o s13 s14 13 /\ r3o s14 s15 14 /\ r3o s15 s16 15)
      (ensures layer_done orig s16 3 /\ is_i32b_poly_avx2 (8380416 + 5*8380416) s16)

(* r5o : the L5 round atom (offset 8k, step_by 4, zeta_r(k+4)). *)
unfold let r5o (a b: av32) (k:nat{k<4}) : Type0 =
  round_post_avx2 a b (8*k) 4 (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (k + 4)))

(* L5 assemble: 4 round posts -> layer congruence atom + whole-poly bound. *)
val lemma_l5_assemble_o
      (orig s1 s2 s3 s4: av32)
    : Lemma
      (requires
        is_i32b_poly_avx2 (8380416 + 2*8380416) orig /\
        r5o orig s1 0 /\ r5o s1 s2 1 /\ r5o s2 s3 2 /\ r5o s3 s4 3)
      (ensures layer_done orig s4 5 /\ is_i32b_poly_avx2 (8380416 + 3*8380416) s4)

(* r4o : the L4 round atom (offset 4k, step_by 2, zeta_r(k+8)). *)
unfold let r4o (a b: av32) (k:nat{k<8}) : Type0 =
  round_post_avx2 a b (4*k) 2 (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (k + 8)))

(* L4 assemble: 8 round posts -> layer congruence atom + whole-poly bound. *)
val lemma_l4_assemble_o
      (orig s1 s2 s3 s4 s5 s6 s7 s8: av32)
    : Lemma
      (requires
        is_i32b_poly_avx2 (8380416 + 3*8380416) orig /\
        r4o orig s1 0 /\ r4o s1 s2 1 /\ r4o s2 s3 2 /\ r4o s3 s4 3 /\
        r4o s4 s5 4 /\ r4o s5 s6 5 /\ r4o s6 s7 6 /\ r4o s7 s8 7)
      (ensures layer_done orig s8 4 /\ is_i32b_poly_avx2 (8380416 + 4*8380416) s8)

(* ---- 5->3 chaining: a 3-layer composite atom (layers 5,4,3 applied in order).
   Produced by ntt_at_layer_5_to_3 from the 3 block layer_done atoms; consumed by
   the top compose. ---- *)
val comp_5_3_done : av32 -> av32 -> Type0

val lemma_compose_5_3_o (orig sL5 sL4 sL3: av32)
    : Lemma
      (requires layer_done orig sL5 5 /\ layer_done sL5 sL4 4 /\ layer_done sL4 sL3 3)
      (ensures comp_5_3_done orig sL3)

(* one-pair cross butterfly producer (.fst-ordered after lemma_compose_5_3_o,
   before comp_7_6_done). *)
val round_cross_sealed
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (index step_by: usize)
      (bnd:nat{bnd + 8380416 < pow2 31})
      (zeta: i32)
    : Prims.Pure (t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (requires
        v step_by > 0 /\
        v index + v step_by < 32 /\
        Spec.Utils.is_i32b 4190208 zeta /\
        is_i32b_unit_avx2 bnd (Seq.index re (v index)) /\
        is_i32b_unit_avx2 bnd (Seq.index re (v index + v step_by)))
      (ensures
        fun re_future ->
          Spec.Utils.modifies2_32 re re_future index (index +! step_by) /\
          is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_future (v index)) /\
          is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_future (v index + v step_by)) /\
          unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 re) (v index))
                               (Seq.index (chunks_of_re_avx2 re) (v index + v step_by))
                               (Seq.index (chunks_of_re_avx2 re_future) (v index))
                               (Seq.index (chunks_of_re_avx2 re_future) (v index + v step_by))
                               zeta)

(* ---- Phase 2: 7_6 two-layer composite (layers 7 then 6).  Order MUST match the
   .fst: comp_7_6_done + lemma_compose_7_6_o precede the block fns. ---- *)

(* the 2-layer composite atom + its compose lemma (mirror comp_5_3_done). *)
val comp_7_6_done : av32 -> av32 -> Type0

val lemma_compose_7_6_o (orig sL7 sL6: av32)
    : Lemma
      (requires layer_done orig sL7 7 /\ layer_done sL7 sL6 6)
      (ensures comp_7_6_done orig sL6)

(* L7 block: full layer 7 (step_by 16).  Input NTT_BASE, output NTT_BASE+1. *)
val ntt_l7_block_o (orig: av32)
    : Prims.Pure av32
      (requires is_i32b_poly_avx2 8380416 orig)
      (ensures fun mid ->
        layer_done orig mid 7 /\ is_i32b_poly_avx2 (8380416 + 8380416) mid)

(* L6 block: full layer 6 (step_by 8, two windows).  Input NTT_BASE+1, output NTT_BASE+2. *)
val ntt_l6_block_o (mid: av32)
    : Prims.Pure av32
      (requires is_i32b_poly_avx2 (8380416 + 8380416) mid)
      (ensures fun out ->
        layer_done mid out 6 /\ is_i32b_poly_avx2 (8380416 + 2*8380416) out)

(* ---- Phase 3: TOP COMPOSE.  ntt_done = the whole-forward-NTT functional atom
   ("flat(b) == Hacspec_ml_dsa.Ntt.ntt (flat a) mod q"); lemma_ntt_top_compose_o
   chains the 7_6 + 5_3 composites and the L2/L1/L0 within-chunk layer_done atoms
   into it (via Avx2NttCompose.lemma_ntt_compose_avx2).  The backport reveal
   (lemma_ntt_done_reveal) stays .fst-internal. ---- *)
val ntt_done : av32 -> av32 -> Type0

val lemma_ntt_top_compose_o (orig s76 s53 s2 s1 ffinal: av32)
    : Lemma
      (requires
        comp_7_6_done orig s76 /\ comp_5_3_done s76 s53 /\
        layer_done s53 s2 2 /\ layer_done s2 s1 1 /\ layer_done s1 ffinal 0)
      (ensures ntt_done orig ffinal)

(* ---- tail block (.fst order: lemma_ntt_done_reveal, bf_pair, quad, build_mid_L7,
   lemma_L7_atoms, lemma_L6_atoms, build_out_impl). ---- *)

val lemma_ntt_done_reveal (a b: av32)
    : Lemma
      (requires ntt_done a b)
      (ensures
        (let in_flat  = C.simd_units_to_array (chunks_of_re_avx2 a) in
         let out_flat = C.simd_units_to_array (chunks_of_re_avx2 b) in
         let spec = Hacspec_ml_dsa.Ntt.ntt in_flat in
         forall (i: nat). i < 256 ==>
           (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))

val bf_pair (u0 u1: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) (zeta: i32)
    : (Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 &
       Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)

val quad (re: av32) (base sb:usize)
         (bnd:nat{bnd + 8380416 < pow2 31}) (zeta:i32)
    : Prims.Pure av32
      (requires
        v sb > 0 /\ v base + 3 + v sb < 32 /\ v base + 4 <= v base + v sb /\
        Spec.Utils.is_i32b 4190208 zeta /\
        (forall (k:nat). (k >= v base /\ k < v base + 4) \/
                         (k >= v base + v sb /\ k < v base + 4 + v sb) ==>
                         is_i32b_unit_avx2 bnd (Seq.index re k)))
      (ensures fun re_f ->
        (forall (k:nat). k < 32 /\
            ~((k >= v base /\ k < v base + 4) \/ (k >= v base + v sb /\ k < v base + 4 + v sb))
            ==> Seq.index re_f k == Seq.index re k) /\
        (forall (k:nat). (k >= v base /\ k < v base + 4) \/
                         (k >= v base + v sb /\ k < v base + 4 + v sb) ==>
                         is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_f k)) /\
        (forall (q:nat). q >= v base /\ q < v base + 4 ==>
            (let (nu0,nu1) = bf_pair (Seq.index re q) (Seq.index re (q + v sb)) zeta in
             Seq.index re_f q == nu0 /\ Seq.index re_f (q + v sb) == nu1)))

val build_mid_L7 (orig: av32)
    : Prims.Pure av32
      (requires is_i32b_poly_avx2 8380416 orig)
      (ensures fun mid ->
        is_i32b_poly_avx2 (8380416 + 8380416) mid /\
        (forall (u:nat). u < 16 ==>
           (let (nu0,nu1) = bf_pair (Seq.index orig u) (Seq.index orig (u+16))
                                    (mk_i32 (zeta_r 1)) in
            Seq.index mid u == nu0 /\ Seq.index mid (u+16) == nu1)))

val lemma_L7_atoms (orig mid: av32)
    : Lemma
      (requires
        (forall (k:nat). k < 32 ==> is_i32b_unit_avx2 8380416 (Seq.index orig k)) /\
        (forall (u:nat). u < 16 ==>
           (let (nu0,nu1) = bf_pair (Seq.index orig u) (Seq.index orig (u+16))
                                    (mk_i32 (zeta_r 1)) in
            Seq.index mid u == nu0 /\ Seq.index mid (u+16) == nu1)))
      (ensures layer_done orig mid 7)

val lemma_L6_atoms (mid out: av32)
    : Lemma
      (requires
        (forall (k:nat). k < 32 ==> is_i32b_unit_avx2 (8380416 + 8380416) (Seq.index mid k)) /\
        (forall (u:nat). (u % 16 < 8) /\ u < 32 ==>
           (let zL6 = (if u < 16 then mk_i32 (zeta_r 2) else mk_i32 (zeta_r 3)) in
            let (nu0,nu1) = bf_pair (Seq.index mid u) (Seq.index mid (u+8)) zL6 in
            Seq.index out u == nu0 /\ Seq.index out (u+8) == nu1)))
      (ensures layer_done mid out 6)

val build_out_impl (orig mid: av32)
    : Prims.Pure av32
      (requires
        is_i32b_poly_avx2 8380416 orig /\
        (forall (k:nat). k < 32 ==> is_i32b_unit_avx2 (8380416 + 8380416) (Seq.index mid k)) /\
        (forall (u:nat). u < 16 ==>
           (let (nu0,nu1) = bf_pair (Seq.index orig u) (Seq.index orig (u+16))
                                    (mk_i32 (zeta_r 1)) in
            Seq.index mid u == nu0 /\ Seq.index mid (u+16) == nu1)))
      (ensures fun out ->
        is_i32b_poly_avx2 (8380416 + 2*8380416) out /\
        (forall (u:nat). (u % 16 < 8) /\ u < 32 ==>
           (let zL6 = (if u < 16 then mk_i32 (zeta_r 2) else mk_i32 (zeta_r 3)) in
            let (nu0,nu1) = bf_pair (Seq.index mid u) (Seq.index mid (u+8)) zL6 in
            Seq.index out u == nu0 /\ Seq.index out (u+8) == nu1)))

(* ---- in-body round seal helpers (forall32 sub-facts -> opaque sealed atoms) ---- *)
val lemma_win_bounded_from_forall32 (re: av32) (lo width bnd:nat)
    : Lemma
      (requires
        Spec.Utils.forall32 (fun i -> (i >= lo /\ i < lo + width) ==>
          is_i32b_unit_avx2 bnd (Seq.index re i)))
      (ensures win_bounded re lo width bnd)

val lemma_win_cross_from_forall32
      (a b: av32) (offset:nat{offset < 32})
      (step_by:nat{step_by > 0 /\ offset + 2*step_by <= 32}) (zeta:i32)
    : Lemma
      (requires
        Spec.Utils.is_i32b 4190208 zeta /\
        Spec.Utils.forall32 (fun u -> (u >= offset /\ u < offset + step_by) ==>
          unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 a) u)
                               (Seq.index (chunks_of_re_avx2 a) (u + step_by))
                               (Seq.index (chunks_of_re_avx2 b) u)
                               (Seq.index (chunks_of_re_avx2 b) (u + step_by))
                               zeta))
      (ensures win_cross a b offset step_by zeta)

(* ---- in-body bf_pair value exposure (ADDITIVE; non-weakening).  bf_pair is otherwise
   an abstract `val`; this exposes its defining value so the IN-BODY 7_6 consumer can match
   the REAL 32-mul (___mul) explicit add/sub output to bf_pair.  .fst body = () (it is
   bf_pair's definition verbatim).  Needed because lemma_atom_to_bf_cross_avx2 only yields
   lane-level relations, not the Vec256 bf_pair record equality that lemma_L6/L7_atoms need. *)
val lemma_bf_pair_def
      (u0 u1: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) (zeta: i32)
    : Lemma
      (ensures
        bf_pair u0 u1 zeta ==
        (({ u0 with Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value =
                Libcrux_intrinsics.Avx2.mm256_add_epi32
                  u0.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
                  (Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply
                    u1.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
                    (Libcrux_intrinsics.Avx2.mm256_set1_epi32 zeta)) }
          <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256),
         ({ Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value =
                Libcrux_intrinsics.Avx2.mm256_sub_epi32
                  u0.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
                  (Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply
                    u1.Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
                    (Libcrux_intrinsics.Avx2.mm256_set1_epi32 zeta)) }
          <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)))

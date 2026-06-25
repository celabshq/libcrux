module Avx2NttTheory
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models
open Spec.Intrinsics
open Spec.MLDSA.NttConstants
open Spec.MLDSA.Math

module C = Hacspec_ml_dsa.Commute.Chunk

(* AVX2 analog of Portable's `chunks_of_re`: project the 32 Vec256 SIMD
   units to the flat-chunk view the Commute.Chunk poly lemmas consume.
   Lane access on AVX2 is the bitvec projection `to_i32x8 vec (mk_u64 l)`,
   not the array index `.f_values.[l]` Portable uses. *)
[@@ "opaque_to_smt"]
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
  = reveal_opaque (`%chunks_of_re_avx2) chunks_of_re_avx2;
    assert (v (mk_usize b) == b);
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

(* ============================================================================
   CROSS LAYERS (L3..L7): pair WHOLE simd units lane-wise with a broadcast zeta.
   For cross layer L: step_by S = 2^(L-3), lo-unit u has u % (2*S) < S, partner
   u+S, broadcast zeta = zeta_r(u/(2*S) + 2^(7-L)).  Mirror of Portable's
   unit_fe_post_cross / lemma_atom_to_bf_cross / lemma_lL_cross_driver_compose,
   but lane access is `to_i32x8 .f_value (mk_u64 l)` and the chunk-arrays are
   the WHOLE units (chunks_of_re_avx2 re).[u] / .[u+S].
   ============================================================================ *)

(* ===== Opaque per-cross-pair FE atom (AVX2 form, mont_mul, 8 lanes).
   ci_lo/ci_hi/co_lo/co_hi are the FOUR chunk-arrays of the two paired units. ===== *)
[@@ "opaque_to_smt"]
let unit_post_cross_avx2 (ci_lo ci_hi co_lo co_hi : t_Array i32 (mk_usize 8))
                         (zeta: i32{Spec.Utils.is_i32b 4190208 zeta}) : Type0 =
  (let t0 = mont_mul (Seq.index ci_hi 0) zeta in
   let t1 = mont_mul (Seq.index ci_hi 1) zeta in
   let t2 = mont_mul (Seq.index ci_hi 2) zeta in
   let t3 = mont_mul (Seq.index ci_hi 3) zeta in
   let t4 = mont_mul (Seq.index ci_hi 4) zeta in
   let t5 = mont_mul (Seq.index ci_hi 5) zeta in
   let t6 = mont_mul (Seq.index ci_hi 6) zeta in
   let t7 = mont_mul (Seq.index ci_hi 7) zeta in
   v (Seq.index co_lo 0) == v (Seq.index ci_lo 0) + v t0 /\
   v (Seq.index co_hi 0) == v (Seq.index ci_lo 0) - v t0 /\
   (v t0) % 8380417 == (v (Seq.index ci_hi 0) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 1) == v (Seq.index ci_lo 1) + v t1 /\
   v (Seq.index co_hi 1) == v (Seq.index ci_lo 1) - v t1 /\
   (v t1) % 8380417 == (v (Seq.index ci_hi 1) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 2) == v (Seq.index ci_lo 2) + v t2 /\
   v (Seq.index co_hi 2) == v (Seq.index ci_lo 2) - v t2 /\
   (v t2) % 8380417 == (v (Seq.index ci_hi 2) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 3) == v (Seq.index ci_lo 3) + v t3 /\
   v (Seq.index co_hi 3) == v (Seq.index ci_lo 3) - v t3 /\
   (v t3) % 8380417 == (v (Seq.index ci_hi 3) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 4) == v (Seq.index ci_lo 4) + v t4 /\
   v (Seq.index co_hi 4) == v (Seq.index ci_lo 4) - v t4 /\
   (v t4) % 8380417 == (v (Seq.index ci_hi 4) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 5) == v (Seq.index ci_lo 5) + v t5 /\
   v (Seq.index co_hi 5) == v (Seq.index ci_lo 5) - v t5 /\
   (v t5) % 8380417 == (v (Seq.index ci_hi 5) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 6) == v (Seq.index ci_lo 6) + v t6 /\
   v (Seq.index co_hi 6) == v (Seq.index ci_lo 6) - v t6 /\
   (v t6) % 8380417 == (v (Seq.index ci_hi 6) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 7) == v (Seq.index ci_lo 7) + v t7 /\
   v (Seq.index co_hi 7) == v (Seq.index ci_lo 7) - v t7 /\
   (v t7) % 8380417 == (v (Seq.index ci_hi 7) * v zeta * 8265825) % 8380417)

(* Standalone: unfold one cross atom to the bridge's per-lane forall (mirror of
   Portable lemma_atom_to_bf_cross). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always --z3refresh"
let lemma_atom_to_bf_cross_avx2 (ci_lo ci_hi co_lo co_hi : t_Array i32 (mk_usize 8))
                                (zeta: i32{Spec.Utils.is_i32b 4190208 zeta})
    : Lemma (requires unit_post_cross_avx2 ci_lo ci_hi co_lo co_hi zeta)
            (ensures
              (forall (l: nat{l < 8}).
                (let t = mont_mul (Seq.index ci_hi l) zeta in
                 v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v t /\
                 v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v t /\
                 (v t) % 8380417 == (v (Seq.index ci_hi l) * v zeta * 8265825) % 8380417)))
  = reveal_opaque (`%unit_post_cross_avx2) unit_post_cross_avx2;
    let lane (l:nat{l<8}) : Lemma
        (let t = mont_mul (Seq.index ci_hi l) zeta in
         v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v t /\
         v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v t /\
         (v t) % 8380417 == (v (Seq.index ci_hi l) * v zeta * 8265825) % 8380417)
      = match l with | 0 -> () | 1 -> () | 2 -> () | 3 -> () | 4 -> () | 5 -> () | 6 -> () | 7 -> ()
    in
    lane 0; lane 1; lane 2; lane 3; lane 4; lane 5; lane 6; lane 7;
    Classical.forall_intro lane
#pop-options

(* ===== CRUX: from the AVX2 mul cross post (forall8 of ntt_step with broadcast
   zeta) + input bound, derive the opaque cross atom for the pair (ulo, uhi) AND
   the per-lane output bound (+FIELD_MAX).  ulo/uhi are arbitrary unit indices
   (the lo unit and its partner); zeta is the SAME value in all 8 lanes
   (broadcast: to_i32x8 zeta_bv (mk_int l) == zeta_val for all l).
   This mirrors lemma_l0_pair_relations but for whole-unit cross pairs. ===== *)

(* ===== Clean-context driver compose for each cross layer L3..L7.  Mirror of
   Portable lemma_lL_cross_driver_compose: take UNCHUNKED orig_re/re (AVX2 units),
   the forall32 of cross atoms over lo-units, feed the (backend-agnostic) Commute
   cross bridge.  The chunks_of_re_avx2 bridge runs HERE in clean context. ===== *)

(* ----- LAYER 3 (S=1, lo: u%2==0, zeta_r(u/2 + 16)) ----- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l3_cross_driver_compose_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 2 == 0) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig_re) u)
                                 (Seq.index (chunks_of_re_avx2 orig_re) (u+1))
                                 (Seq.index (chunks_of_re_avx2 re) u)
                                 (Seq.index (chunks_of_re_avx2 re) (u+1))
                                 (mk_i32 (zeta_r (u / 2 + 16)))))
        (ensures
          (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 3) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = chunks_of_re_avx2 orig_re in
    let fut = chunks_of_re_avx2 re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (zeta_r (u / 2 + 16)) in
    let t (u: nat{u < 32}) (l: nat{l < 8}) : i32 =
        mont_mul (Seq.index (Seq.index orig ((u + 1) % 32)) l) (zm u) in
    forall32_elim_1d (fun u -> (u % 2 == 0) ==>
        unit_post_cross_avx2 (Seq.index orig u) (Seq.index orig (u+1))
                             (Seq.index fut u) (Seq.index fut (u+1)) (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 2 == 0) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+1) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+1) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t u l) /\
          v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t u l) /\
          (v (t u l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm u) * 8265825) % 8380417))
      = if (u % 2 = 0) then begin
          C.lemma_cross_idx 1 u 0;
          FStar.Math.Lemmas.small_mod (u + 1) 32;
          lemma_atom_to_bf_cross_avx2 (Seq.index orig u) (Seq.index orig (u+1))
                                      (Seq.index fut u) (Seq.index fut (u+1)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 2 == 0) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (u / 2 + 16) ] <: i32) * pow2 32) % 8380417)
      = if (u % 2 = 0) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = zeta_r (u / 2 + 16) in
          C.lemma_v_zetas_eq_zeta (u / 2 + 16)
        end
     in Classical.forall_intro aux_z);
    C.lemma_ntt_layer_3_cross_to_hacspec_poly orig fut t zm
#pop-options

(* ----- LAYER 4 (S=2, lo: u%4<2, zeta_r(u/4 + 8)) ----- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l4_cross_driver_compose_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 4 < 2) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig_re) u)
                                 (Seq.index (chunks_of_re_avx2 orig_re) (u+2))
                                 (Seq.index (chunks_of_re_avx2 re) u)
                                 (Seq.index (chunks_of_re_avx2 re) (u+2))
                                 (mk_i32 (zeta_r (u / 4 + 8)))))
        (ensures
          (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 4) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = chunks_of_re_avx2 orig_re in
    let fut = chunks_of_re_avx2 re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (zeta_r (u / 4 + 8)) in
    let t (u: nat{u < 32}) (l: nat{l < 8}) : i32 =
        mont_mul (Seq.index (Seq.index orig ((u + 2) % 32)) l) (zm u) in
    forall32_elim_1d (fun u -> (u % 4 < 2) ==>
        unit_post_cross_avx2 (Seq.index orig u) (Seq.index orig (u+2))
                             (Seq.index fut u) (Seq.index fut (u+2)) (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 4 < 2) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+2) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+2) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t u l) /\
          v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t u l) /\
          (v (t u l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm u) * 8265825) % 8380417))
      = if (u % 4 < 2) then begin
          C.lemma_cross_idx 2 u 0;
          FStar.Math.Lemmas.small_mod (u + 2) 32;
          lemma_atom_to_bf_cross_avx2 (Seq.index orig u) (Seq.index orig (u+2))
                                      (Seq.index fut u) (Seq.index fut (u+2)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 4 < 2) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (u / 4 + 8) ] <: i32) * pow2 32) % 8380417)
      = if (u % 4 < 2) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = zeta_r (u / 4 + 8) in
          C.lemma_v_zetas_eq_zeta (u / 4 + 8)
        end
     in Classical.forall_intro aux_z);
    C.lemma_ntt_layer_4_cross_to_hacspec_poly orig fut t zm
#pop-options

(* ----- LAYER 5 (S=4, lo: u%8<4, zeta_r(u/8 + 4)) ----- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l5_cross_driver_compose_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 8 < 4) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig_re) u)
                                 (Seq.index (chunks_of_re_avx2 orig_re) (u+4))
                                 (Seq.index (chunks_of_re_avx2 re) u)
                                 (Seq.index (chunks_of_re_avx2 re) (u+4))
                                 (mk_i32 (zeta_r (u / 8 + 4)))))
        (ensures
          (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 5) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = chunks_of_re_avx2 orig_re in
    let fut = chunks_of_re_avx2 re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (zeta_r (u / 8 + 4)) in
    let t (u: nat{u < 32}) (l: nat{l < 8}) : i32 =
        mont_mul (Seq.index (Seq.index orig ((u + 4) % 32)) l) (zm u) in
    forall32_elim_1d (fun u -> (u % 8 < 4) ==>
        unit_post_cross_avx2 (Seq.index orig u) (Seq.index orig (u+4))
                             (Seq.index fut u) (Seq.index fut (u+4)) (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 8 < 4) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+4) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+4) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t u l) /\
          v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t u l) /\
          (v (t u l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm u) * 8265825) % 8380417))
      = if (u % 8 < 4) then begin
          C.lemma_cross_idx 4 u 0;
          FStar.Math.Lemmas.small_mod (u + 4) 32;
          lemma_atom_to_bf_cross_avx2 (Seq.index orig u) (Seq.index orig (u+4))
                                      (Seq.index fut u) (Seq.index fut (u+4)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 8 < 4) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (u / 8 + 4) ] <: i32) * pow2 32) % 8380417)
      = if (u % 8 < 4) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = zeta_r (u / 8 + 4) in
          C.lemma_v_zetas_eq_zeta (u / 8 + 4)
        end
     in Classical.forall_intro aux_z);
    C.lemma_ntt_layer_5_cross_to_hacspec_poly orig fut t zm
#pop-options

(* ----- LAYER 6 (S=8, lo: u%16<8, zeta_r(u/16 + 2)) ----- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l6_cross_driver_compose_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 16 < 8) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig_re) u)
                                 (Seq.index (chunks_of_re_avx2 orig_re) (u+8))
                                 (Seq.index (chunks_of_re_avx2 re) u)
                                 (Seq.index (chunks_of_re_avx2 re) (u+8))
                                 (mk_i32 (zeta_r (u / 16 + 2)))))
        (ensures
          (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 6) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = chunks_of_re_avx2 orig_re in
    let fut = chunks_of_re_avx2 re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (zeta_r (u / 16 + 2)) in
    let t (u: nat{u < 32}) (l: nat{l < 8}) : i32 =
        mont_mul (Seq.index (Seq.index orig ((u + 8) % 32)) l) (zm u) in
    forall32_elim_1d (fun u -> (u % 16 < 8) ==>
        unit_post_cross_avx2 (Seq.index orig u) (Seq.index orig (u+8))
                             (Seq.index fut u) (Seq.index fut (u+8)) (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 16 < 8) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+8) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+8) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t u l) /\
          v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t u l) /\
          (v (t u l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm u) * 8265825) % 8380417))
      = if (u % 16 < 8) then begin
          C.lemma_cross_idx 8 u 0;
          FStar.Math.Lemmas.small_mod (u + 8) 32;
          lemma_atom_to_bf_cross_avx2 (Seq.index orig u) (Seq.index orig (u+8))
                                      (Seq.index fut u) (Seq.index fut (u+8)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 16 < 8) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (u / 16 + 2) ] <: i32) * pow2 32) % 8380417)
      = if (u % 16 < 8) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = zeta_r (u / 16 + 2) in
          C.lemma_v_zetas_eq_zeta (u / 16 + 2)
        end
     in Classical.forall_intro aux_z);
    C.lemma_ntt_layer_6_cross_to_hacspec_poly orig fut t zm
#pop-options

(* ----- LAYER 7 (S=16, lo: u%32<16, zeta_r(u/32 + 1)) ----- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l7_cross_driver_compose_avx2
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 32 < 16) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig_re) u)
                                 (Seq.index (chunks_of_re_avx2 orig_re) (u+16))
                                 (Seq.index (chunks_of_re_avx2 re) u)
                                 (Seq.index (chunks_of_re_avx2 re) (u+16))
                                 (mk_i32 (zeta_r (u / 32 + 1)))))
        (ensures
          (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig_re) in
           let out_flat = C.simd_units_to_array (chunks_of_re_avx2 re) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 7) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = chunks_of_re_avx2 orig_re in
    let fut = chunks_of_re_avx2 re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (zeta_r (u / 32 + 1)) in
    let t (u: nat{u < 32}) (l: nat{l < 8}) : i32 =
        mont_mul (Seq.index (Seq.index orig ((u + 16) % 32)) l) (zm u) in
    forall32_elim_1d (fun u -> (u % 32 < 16) ==>
        unit_post_cross_avx2 (Seq.index orig u) (Seq.index orig (u+16))
                             (Seq.index fut u) (Seq.index fut (u+16)) (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 32 < 16) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+16) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+16) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t u l) /\
          v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t u l) /\
          (v (t u l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm u) * 8265825) % 8380417))
      = if (u % 32 < 16) then begin
          C.lemma_cross_idx 16 u 0;
          FStar.Math.Lemmas.small_mod (u + 16) 32;
          lemma_atom_to_bf_cross_avx2 (Seq.index orig u) (Seq.index orig (u+16))
                                      (Seq.index fut u) (Seq.index fut (u+16)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 32 < 16) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (u / 32 + 1) ] <: i32) * pow2 32) % 8380417)
      = if (u % 32 < 16) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = zeta_r (u / 32 + 1) in
          C.lemma_v_zetas_eq_zeta (u / 32 + 1)
        end
     in Classical.forall_intro aux_z);
    C.lemma_ntt_layer_7_cross_to_hacspec_poly orig fut t zm
#pop-options


(* ---- the forall32 wrapper -> driver call.  Takes the per-unit atoms already as a
   plain `forall u<32` (which the orchestrator assembles by calling
   lemma_cross_atom_to_layer per lo-unit), bridges to Spec.Utils.forall32, and
   invokes lemma_l5_cross_driver_compose_avx2 -> the L5 flat congruence.  This
   isolates the (cheap) forall32 intro from the (saturating) per-unit work, so the
   orchestrator's body stays thin.  L4/L3 analogs: swap the mask u%8<4 -> u%4<2 /
   u%2==0, the partner +4 -> +2 / +1, zeta_r(u/8+4) -> zeta_r(u/4+8) / zeta_r(u/2+16),
   and the driver -> lemma_l4_/l3_cross_driver_compose_avx2. ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l5_forall32_to_compose
      (orig fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 8 < 4) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig) u)
                                 (Seq.index (chunks_of_re_avx2 orig) (u+4))
                                 (Seq.index (chunks_of_re_avx2 fut) u)
                                 (Seq.index (chunks_of_re_avx2 fut) (u+4))
                                 (mk_i32 (zeta_r (u / 8 + 4)))))
        (ensures
          (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig) in
           let out_flat = C.simd_units_to_array (chunks_of_re_avx2 fut) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 5) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = lemma_l5_cross_driver_compose_avx2 orig fut
#pop-options

(* L4 analog of lemma_l5_forall32_to_compose (mask u%4<2, partner +2, zeta_r(u/4+8)). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l4_forall32_to_compose
      (orig fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 4 < 2) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig) u)
                                 (Seq.index (chunks_of_re_avx2 orig) (u+2))
                                 (Seq.index (chunks_of_re_avx2 fut) u)
                                 (Seq.index (chunks_of_re_avx2 fut) (u+2))
                                 (mk_i32 (zeta_r (u / 4 + 8)))))
        (ensures
          (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig) in
           let out_flat = C.simd_units_to_array (chunks_of_re_avx2 fut) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 4) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = lemma_l4_cross_driver_compose_avx2 orig fut
#pop-options

(* L3 analog (mask u%2==0, partner +1, zeta_r(u/2+16)). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l3_forall32_to_compose
      (orig fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 2 == 0) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig) u)
                                 (Seq.index (chunks_of_re_avx2 orig) (u+1))
                                 (Seq.index (chunks_of_re_avx2 fut) u)
                                 (Seq.index (chunks_of_re_avx2 fut) (u+1))
                                 (mk_i32 (zeta_r (u / 2 + 16)))))
        (ensures
          (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig) in
           let out_flat = C.simd_units_to_array (chunks_of_re_avx2 fut) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 3) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = lemma_l3_cross_driver_compose_avx2 orig fut
#pop-options

(* ============================================================================
   WINDOW-SCOPED MACHINERY (mirror of Portable outer_3_plus / outer_3_plus__round).
   The PENDING/DONE inner_inv_cross machinery above uses WHOLE-POLY bounds and is
   bound-broken (28 rounds bump the whole-poly bound +FIELD_MAX each).  Everything
   below is window-scoped so a layer's 16/8/4 windows each bump only their own 2
   units, giving the correct +FIELD_MAX-per-LAYER bound.
   ============================================================================ *)

(* ---- Deliverable 1: per-UNIT i32 bound (opaque), + poly<->unit bridges. ---- *)
[@@ "opaque_to_smt"]
let is_i32b_unit_avx2 (bnd:nat)
      (u: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) : Type0 =
  forall (l:nat). l < 8 ==> Spec.Utils.is_i32b bnd (to_i32x8 u.f_value (mk_u64 l))

let lemma_is_i32b_unit_avx2_elim (bnd:nat)
      (u: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) (l:nat{l<8})
    : Lemma (requires is_i32b_unit_avx2 bnd u)
            (ensures Spec.Utils.is_i32b bnd (to_i32x8 u.f_value (mk_u64 l)))
  = reveal_opaque (`%is_i32b_unit_avx2) is_i32b_unit_avx2

let lemma_is_i32b_unit_avx2_intro (bnd:nat)
      (u: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
    : Lemma (requires forall (l:nat). l < 8 ==>
               Spec.Utils.is_i32b bnd (to_i32x8 u.f_value (mk_u64 l)))
            (ensures is_i32b_unit_avx2 bnd u)
  = reveal_opaque (`%is_i32b_unit_avx2) is_i32b_unit_avx2

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_poly_to_units (bnd:nat)
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires is_i32b_poly_avx2 bnd re)
            (ensures forall (u:nat). u < 32 ==> is_i32b_unit_avx2 bnd (Seq.index re u))
  = reveal_opaque (`%is_i32b_poly_avx2) is_i32b_poly_avx2;
    reveal_opaque (`%is_i32b_unit_avx2) is_i32b_unit_avx2
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_units_to_poly (bnd:nat)
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma (requires forall (u:nat). u < 32 ==> is_i32b_unit_avx2 bnd (Seq.index re u))
            (ensures is_i32b_poly_avx2 bnd re)
  = reveal_opaque (`%is_i32b_poly_avx2) is_i32b_poly_avx2;
    reveal_opaque (`%is_i32b_unit_avx2) is_i32b_unit_avx2
#pop-options

(* ---- Deliverable 2: PAIR-SCOPED variant of lemma_cross_pair_relations.
   Replace the whole-poly `is_i32b_poly_avx2 bnd re` requires with just the two
   pair units bounded; identical ensures.  This is what the window driver needs
   (the window requires only bounds the 2*STEP_BY units in scope, not the poly). ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_cross_pair_relations_ws
      (re re_fut: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
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
  = let ci = chunks_of_re_avx2 re in
    let co = chunks_of_re_avx2 re_fut in
    let ci_lo = Seq.index ci ulo in
    let ci_hi = Seq.index ci uhi in
    let co_lo = Seq.index co ulo in
    let co_hi = Seq.index co uhi in
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    let lane (l:nat{l<8}) : Lemma
        (let cilo = Seq.index ci_lo l in
         let cihi = Seq.index ci_hi l in
         let colo = Seq.index co_lo l in
         let cohi = Seq.index co_hi l in
         let t = mont_mul cihi zeta_val in
         v colo == v cilo + v t /\
         v cohi == v cilo - v t /\
         (v t) % 8380417 == (v cihi * v zeta_val * 8265825) % 8380417 /\
         Spec.Utils.is_i32b (bnd + 8380416) colo /\
         Spec.Utils.is_i32b (bnd + 8380416) cohi)
      = lemma_chunks_of_re_avx2_index re ulo l;
        lemma_chunks_of_re_avx2_index re uhi l;
        lemma_chunks_of_re_avx2_index re_fut ulo l;
        lemma_chunks_of_re_avx2_index re_fut uhi l;
        let cilo = Seq.index ci_lo l in
        let cihi = Seq.index ci_hi l in
        assert (cilo == to_i32x8 (Seq.index re ulo).f_value (mk_u64 l));
        assert (cihi == to_i32x8 (Seq.index re uhi).f_value (mk_u64 l));
        assert (to_i32x8 zeta_bv (mk_int l) == zeta_val);
        let t = mont_mul cihi zeta_val in
        assert (to_i32x8 (Seq.index re_fut ulo).f_value (mk_u64 l) == add_mod_opaque cilo t);
        assert (to_i32x8 (Seq.index re_fut uhi).f_value (mk_u64 l) == sub_mod_opaque cilo t);
        // input bound on cilo, cihi  (PAIR-SCOPED: unit elim, not poly elim)
        lemma_is_i32b_unit_avx2_elim bnd (Seq.index re ulo) l;
        lemma_is_i32b_unit_avx2_elim bnd (Seq.index re uhi) l;
        assert (Spec.Utils.is_i32b bnd cilo);
        assert (Spec.Utils.is_i32b 8380416 zeta_val);
        C.lemma_mont_mul_bound_and_mod_q cihi zeta_val;
        assert (Spec.Utils.is_i32b 8380416 t);
        assert (v (Seq.index co_lo l) == v cilo + v t);
        assert (v (Seq.index co_hi l) == v cilo - v t)
    in
    lane 0; lane 1; lane 2; lane 3; lane 4; lane 5; lane 6; lane 7;
    reveal_opaque (`%unit_post_cross_avx2) unit_post_cross_avx2;
    introduce forall (l:nat{l<8}).
        Spec.Utils.is_i32b (bnd + 8380416) (to_i32x8 (Seq.index re_fut ulo).f_value (mk_u64 l)) /\
        Spec.Utils.is_i32b (bnd + 8380416) (to_i32x8 (Seq.index re_fut uhi).f_value (mk_u64 l))
    with (lemma_chunks_of_re_avx2_index re_fut ulo l;
          lemma_chunks_of_re_avx2_index re_fut uhi l)
#pop-options


(* ---- Deliverable 3: THE KEYSTONE.  Mirror of Portable outer_3_plus +
   outer_3_plus__round.  The per-step butterfly proof lives in a SEPARATE
   opaque Pure fn (round_ws__round) so the fold body only sees its clean
   ensures (modifies2 + bound + cross atom), not the SMTPat arithmetic. ---- *)

(* Framing: chunk u of chunks_of_re_avx2 depends ONLY on unit u, so unit equality
   gives chunk equality.  Needed because chunks_of_re_avx2 is a createi projection
   (vs Portable's raw .f_values) — F* will not auto-frame it under update_at. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_chunks_frame
      (a b: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (u:nat{u<32})
    : Lemma (requires Seq.index a u == Seq.index b u)
            (ensures Seq.index (chunks_of_re_avx2 a) u == Seq.index (chunks_of_re_avx2 b) u)
  = let ca = Seq.index (chunks_of_re_avx2 a) u in
    let cb = Seq.index (chunks_of_re_avx2 b) u in
    introduce forall (l:nat{l<8}). Seq.index ca l == Seq.index cb l
    with (lemma_chunks_of_re_avx2_index a u l;
          lemma_chunks_of_re_avx2_index b u l);
    assert (Seq.length ca == 8);
    assert (Seq.length cb == 8);
    Seq.lemma_eq_intro ca cb
#pop-options

(* ---- 3a: per-step butterfly = one Vec256 GS pair (mirror outer_3_plus__round).
   Body is the extracted ntt_at_layer_5_to_3___round inner step VERBATIM. ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"
let round_ws__round
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (index step_by: usize)
      (zeta: i32)
    : Prims.Pure (t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (requires
        v step_by > 0 /\
        v index + v step_by < 32 /\
        Spec.Utils.is_i32b 4190208 zeta /\
        is_i32b_unit_avx2 (8380416 + (layer_bound_factor_avx2 (v step_by)) * 8380416)
          (Seq.index re (v index)) /\
        is_i32b_unit_avx2 (8380416 + (layer_bound_factor_avx2 (v step_by)) * 8380416)
          (Seq.index re (v index + v step_by)))
      (ensures
        fun re_future ->
          Spec.Utils.modifies2_32 re re_future index (index +! step_by) /\
          is_i32b_unit_avx2 (8380416 + (layer_bound_factor_avx2 (v step_by) + 1) * 8380416)
            (Seq.index re_future (v index)) /\
          is_i32b_unit_avx2 (8380416 + (layer_bound_factor_avx2 (v step_by) + 1) * 8380416)
            (Seq.index re_future (v index + v step_by)) /\
          unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 re) (v index))
                               (Seq.index (chunks_of_re_avx2 re) (v index + v step_by))
                               (Seq.index (chunks_of_re_avx2 re_future) (v index))
                               (Seq.index (chunks_of_re_avx2 re_future) (v index + v step_by))
                               zeta) =
  let bnd : nat = 8380416 + (layer_bound_factor_avx2 (v step_by)) * 8380416 in
  let rhs:Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256) =
    Libcrux_intrinsics.Avx2.mm256_set1_epi32 zeta
  in
  let re_in:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) = re in
  // ---- butterfly (verbatim from extracted ntt_at_layer_5_to_3___round step) ----
  let re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re
      (index +! step_by <: usize)
      ({
          (re.[ index +! step_by <: usize ] <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) with
          Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
          =
          Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply (re.[ index +! step_by <: usize ]
              <:
              Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
              .Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value rhs
          <:
          Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)
        }
        <:
        Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
  in
  let tmp:Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256) =
    Libcrux_intrinsics.Avx2.mm256_sub_epi32 (re.[ index ]
        <:
        Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
        .Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
      (re.[ index +! step_by <: usize ] <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
        .Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
  in
  let re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re
      index
      ({
          Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
          =
          Libcrux_intrinsics.Avx2.mm256_add_epi32 (re.[ index ]
              <:
              Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
              .Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
            (re.[ index +! step_by <: usize ] <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
              .Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
          <:
          Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)
        }
        <:
        Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
  in
  let re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re
      (index +! step_by <: usize)
      ({ Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value = tmp }
        <:
        Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
  in
  // ---- proof: establish ntt_step fact for pair (index, index+step_by) then call ws lemma ----
  let _:Prims.unit =
    let re0 = (Seq.index re_in (v index)).f_value in
    let re1 = (Seq.index re_in (v index + v step_by)).f_value in
    let nre0 = (Seq.index re (v index)).f_value in
    let nre1 = (Seq.index re (v index + v step_by)).f_value in
    // per-lane: the ntt_step fact (broadcast set1 + mont + add/sub SMTPats fire per concrete lane)
    let lane (i:nat{i<8}) : Lemma
        ((to_i32x8 nre0 (mk_u64 i), to_i32x8 nre1 (mk_u64 i)) ==
         ntt_step (to_i32x8 rhs (mk_int i))
           (to_i32x8 re0 (mk_u64 i), to_i32x8 re1 (mk_u64 i)))
      = assert (to_i32x8 rhs (mk_int i) == zeta);
        assert (to_i32x8 rhs (mk_u64 i) == zeta)
    in
    lane 0; lane 1; lane 2; lane 3; lane 4; lane 5; lane 6; lane 7;
    assert (Spec.Utils.forall8 (fun i ->
             (to_i32x8 nre0 (mk_u64 i), to_i32x8 nre1 (mk_u64 i)) ==
             ntt_step (to_i32x8 rhs (mk_int i))
               (to_i32x8 re0 (mk_u64 i), to_i32x8 re1 (mk_u64 i))));
    // broadcast fact for the lemma's zeta-broadcast requires
    introduce forall (l:nat). l < 8 ==> to_i32x8 rhs (mk_int l) == zeta
    with (introduce l < 8 ==> to_i32x8 rhs (mk_int l) == zeta
          with _. assert (to_i32x8 rhs (mk_int l) == zeta));
    lemma_cross_pair_relations_ws re_in re bnd (v index) (v index + v step_by) rhs zeta;
    // lift the per-lane output bounds (bnd+FIELD_MAX) to is_i32b_unit_avx2 on both result units
    lemma_is_i32b_unit_avx2_intro (bnd + 8380416) (Seq.index re (v index));
    lemma_is_i32b_unit_avx2_intro (bnd + 8380416) (Seq.index re (v index + v step_by))
  in
  re
#pop-options

(* ---- 3a': clean-context fold-invariant maintenance for round_ws.  Given the
   entry invariant at j + the round_ws__round post on (re_old -> re_new) for the
   pair (j, j+STEP_BY), derive the exit invariant at j+1.  The createi-projection
   framing (chunks frame lemma) runs HERE in a clean context so the fold body's
   VC stays thin.  bumped = 8380416+(lbf+1)*8380416. ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_round_ws_maintains
      (orig_re re_old re_new: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (offset j step_by: usize)
      (lbf:nat{lbf <= 4})
      (zeta:i32{Spec.Utils.is_i32b 4190208 zeta})
    : Lemma
      (requires
        v step_by > 0 /\ v offset <= v j /\ v j < v offset + v step_by /\
        v offset + 2 * v step_by <= 32 /\
        // ENTRY invariant at j
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
        // round_ws__round POST on the pair (j, j+step_by)
        Spec.Utils.modifies2_32 re_old re_new j (j +! step_by) /\
        is_i32b_unit_avx2 (8380416 + (lbf + 1) * 8380416) (Seq.index re_new (v j)) /\
        is_i32b_unit_avx2 (8380416 + (lbf + 1) * 8380416) (Seq.index re_new (v j + v step_by)) /\
        unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 re_old) (v j))
          (Seq.index (chunks_of_re_avx2 re_old) (v j + v step_by))
          (Seq.index (chunks_of_re_avx2 re_new) (v j))
          (Seq.index (chunks_of_re_avx2 re_new) (v j + v step_by)) zeta)
      (ensures
        // EXIT invariant at j+1
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
  = let sb = v step_by in
    let jj = v j in
    // (A) frame: re_new agrees with re_old on every unit except j, j+sb (modifies2)
    // (B) the new-pair cross atom: re_old[j]==orig_re[j] (untouched in entry frame), so reindex
    lemma_chunks_frame orig_re re_old jj;
    lemma_chunks_frame orig_re re_old (jj + sb);
    // (C) old atoms (u<j): both units u, u+sb untouched by round_ws__round => chunks preserved
    introduce forall (u:nat{u<32}).
        (u >= v offset /\ u < jj + 1) ==>
        unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig_re) u)
          (Seq.index (chunks_of_re_avx2 orig_re) (u + sb))
          (Seq.index (chunks_of_re_avx2 re_new) u)
          (Seq.index (chunks_of_re_avx2 re_new) (u + sb)) zeta
    with (introduce (u >= v offset /\ u < jj + 1) ==> _
          with _. (
            if u = jj then (
              // new pair: round_ws__round atom on (re_old[j],re_old[j+sb],re_new..) +
              // pre-established chunks(orig)[j]==chunks(re_old)[j] (and +sb) rewrite to orig
              ()
            ) else (
              // old pair u<j: u and u+sb untouched by round_ws__round (only j,j+sb modified)
              lemma_chunks_frame re_old re_new u;
              lemma_chunks_frame re_old re_new (u + sb)
            )))
#pop-options

(* ---- 3b: round_ws — the window-scoped single-window driver (mirror outer_3_plus).
   fold_range offset (offset+STEP_BY), each step a round_ws__round butterfly. ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"
let round_ws
      (v_STEP v_STEP_BY: usize)
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (index: usize)
      (zeta: i32)
    : Prims.Pure (t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (requires
        (v v_STEP == 8 \/ v v_STEP == 16 \/ v v_STEP == 32) /\
        v v_STEP_BY == v v_STEP / 8 /\
        v index < 128 / v v_STEP /\
        Spec.Utils.is_i32b 4190208 zeta /\
        (let offset = ((v index) * (v v_STEP) * 2) / 8 in
         offset + 2 * (v v_STEP_BY) <= 32 /\
         (Spec.Utils.forall32 (fun i ->
            (i >= offset /\ i < offset + 2 * (v v_STEP_BY)) ==>
            is_i32b_unit_avx2 (8380416 + (layer_bound_factor_avx2 (v v_STEP_BY)) * 8380416)
              (Seq.index re i)))))
      (ensures
        fun re_future ->
          let offset = ((v index) * (v v_STEP) * 2) / 8 in
          Spec.Utils.modifies_range_32 re re_future
            (mk_usize offset) (mk_usize (offset + 2 * (v v_STEP_BY))) /\
          (Spec.Utils.forall32 (fun i ->
             (i >= offset /\ i < offset + 2 * (v v_STEP_BY)) ==>
             is_i32b_unit_avx2 (8380416 + (layer_bound_factor_avx2 (v v_STEP_BY) + 1) * 8380416)
               (Seq.index re_future i))) /\
          (Spec.Utils.forall32 (fun u ->
             (u >= offset /\ u < offset + (v v_STEP_BY)) ==>
             unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 re) u)
                                  (Seq.index (chunks_of_re_avx2 re) (u + v v_STEP_BY))
                                  (Seq.index (chunks_of_re_avx2 re_future) u)
                                  (Seq.index (chunks_of_re_avx2 re_future) (u + v v_STEP_BY))
                                  zeta))) =
  let lbf : nat = layer_bound_factor_avx2 (v v_STEP_BY) in
  let bnd : nat = 8380416 + lbf * 8380416 in
  let offset:usize =
    ((index *! v_STEP <: usize) *! mk_usize 2 <: usize) /!
    Libcrux_ml_dsa.Simd.Traits.v_COEFFICIENTS_IN_SIMD_UNIT
  in
  let orig_re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) =
    Core_models.Clone.f_clone #(t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      #FStar.Tactics.Typeclasses.solve re
  in
  let re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) =
    Rust_primitives.Hax.Folds.fold_range offset
      (offset +! v_STEP_BY <: usize)
      (fun re j ->
          let re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) = re in
          let j:usize = j in
          (Spec.Utils.modifies_range2_32 orig_re re
              offset j
              (offset +! v_STEP_BY) (j +! v_STEP_BY)) /\
          (Spec.Utils.forall32 (fun i ->
                  ((i >= v offset /\ i < v j) \/
                    (i >= v offset + v v_STEP_BY /\ i < v j + v v_STEP_BY)) ==>
                  is_i32b_unit_avx2 (8380416 + (lbf + 1) * 8380416) (Seq.index re i))) /\
          (Spec.Utils.forall32 (fun u ->
                  (u >= v offset /\ u < v j) ==>
                  unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig_re) u)
                    (Seq.index (chunks_of_re_avx2 orig_re) (u + v v_STEP_BY))
                    (Seq.index (chunks_of_re_avx2 re) u)
                    (Seq.index (chunks_of_re_avx2 re) (u + v v_STEP_BY))
                    zeta)))
      re
      (fun re j ->
          let re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) = re in
          let j:usize = j in
          let re_old:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) = re in
          // pre-butterfly: j and j+STEP_BY untouched => == orig_re => bounded at bnd (from requires)
          let _:Prims.unit =
            assert (Seq.index re_old (v j) == Seq.index orig_re (v j));
            assert (Seq.index re_old (v j + v v_STEP_BY) == Seq.index orig_re (v j + v v_STEP_BY));
            assert (is_i32b_unit_avx2 bnd (Seq.index re_old (v j)));
            assert (is_i32b_unit_avx2 bnd (Seq.index re_old (v j + v v_STEP_BY)))
          in
          let re_new:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) =
            round_ws__round re_old j v_STEP_BY zeta
          in
          // re-establish the fold invariant at j+1 in a clean context
          let _:Prims.unit =
            lemma_round_ws_maintains orig_re re_old re_new offset j v_STEP_BY lbf zeta
          in
          re_new)
  in
  re
#pop-options

(* ============================================================================
   Deliverable 4: SAMPLE L5 layer body-tail assembly (framing demo).
   L5: v_STEP=32, v_STEP_BY=4 => 4 disjoint windows, offsets 0,8,16,24; window k
   uses zeta_r(k+4).  Given the 4 round_ws posts (snapshots s0=orig,s1,s2,s3,s4),
   derive the 16 unit_post_cross_avx2 atoms between orig and s4 (via the modifies
   frames: orig agrees with s_k on window k because earlier windows are disjoint;
   s4 agrees with s_{k+1} on window k because later windows are disjoint) and feed
   lemma_l5_forall32_to_compose -> the L5 flat congruence.  Mirror of Portable
   ntt_at_layer_3_ body-tail.  The 4 round posts are stated as `requires` (the
   orchestrator gets them by 4 actual round_ws calls + snapshots). ============ *)

(* the window-k cross post shape produced by round_ws at offset 8k, STEP_BY 4. *)
unfold let l5_round_post
      (a b: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (k:nat{k<4}) : Type0 =
  Spec.Utils.modifies_range_32 a b (mk_usize (8*k)) (mk_usize (8*k+8)) /\
  (Spec.Utils.forall32 (fun u ->
      (u >= 8*k /\ u < 8*k + 4) ==>
      unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 a) u)
        (Seq.index (chunks_of_re_avx2 a) (u + 4))
        (Seq.index (chunks_of_re_avx2 b) u)
        (Seq.index (chunks_of_re_avx2 b) (u + 4))
        (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (k + 4)))))

#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let l5_body_tail_demo
      (orig s1 s2 s3 s4: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        l5_round_post orig s1 0 /\ l5_round_post s1 s2 1 /\
        l5_round_post s2 s3 2 /\ l5_round_post s3 s4 3)
      (ensures
        (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig) in
         let out_flat = C.simd_units_to_array (chunks_of_re_avx2 s4) in
         let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 5) in
         forall (i: nat). i < 256 ==>
           (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)) =
  // snapshots in order: orig=s0 -> s1 -> s2 -> s3 -> s4
  // window k modifies units [8k,8k+8); windows are pairwise disjoint, so:
  //   orig agrees with s_k on [8k,8k+8)  (earlier windows 0..k-1 are below 8k)
  //   s4 agrees with s_{k+1} on [8k,8k+8) (later windows k+1..3 are at/above 8(k+1))
  // For each lo-unit u in [8k,8k+4): frame chunks(orig)[u],[u+4] == chunks(s_k)[u],[u+4]
  //   and chunks(s4)[u],[u+4] == chunks(s_{k+1})[u],[u+4]; rewrite the round atom.
  let frame_unit (a b: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                 (u:nat{u<32}) : Lemma (requires Seq.index a u == Seq.index b u)
                                       (ensures Seq.index (chunks_of_re_avx2 a) u ==
                                                Seq.index (chunks_of_re_avx2 b) u)
    = lemma_chunks_frame a b u in
  // establish the 16 target atoms (orig -> s4) per lo-unit, window by window
  let win (k:nat{k<4}) (u:nat{8*k <= u /\ u < 8*k+4}) : Lemma
      (unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig) u)
         (Seq.index (chunks_of_re_avx2 orig) (u + 4))
         (Seq.index (chunks_of_re_avx2 s4) u)
         (Seq.index (chunks_of_re_avx2 s4) (u + 4))
         (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (k + 4)))) =
    // pick the right snapshots: a_k = s_k (entry), b_k = s_{k+1} (exit)
    let s_entry = (match k with | 0 -> orig | 1 -> s1 | 2 -> s2 | _ -> s3) in
    let s_exit  = (match k with | 0 -> s1   | 1 -> s2 | 2 -> s3 | _ -> s4) in
    // orig agrees with s_entry on u, u+4  (u,u+4 in [8k,8k+8), untouched by windows 0..k-1)
    frame_unit orig s_entry u; frame_unit orig s_entry (u+4);
    // s4 agrees with s_exit on u, u+4    (untouched by windows k+1..3)
    frame_unit s_exit s4 u; frame_unit s_exit s4 (u+4)
  in
  // window 0
  win 0 0; win 0 1; win 0 2; win 0 3;
  // window 1
  win 1 8; win 1 9; win 1 10; win 1 11;
  // window 2
  win 2 16; win 2 17; win 2 18; win 2 19;
  // window 3
  win 3 24; win 3 25; win 3 26; win 3 27;
  // assemble the forall32 (mask u%8<4, partner +4, zeta_r(u/8+4)) and compose
  assert (Spec.Utils.forall32 (fun u ->
            (u % 8 < 4) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig) u)
                                 (Seq.index (chunks_of_re_avx2 orig) (u+4))
                                 (Seq.index (chunks_of_re_avx2 s4) u)
                                 (Seq.index (chunks_of_re_avx2 s4) (u+4))
                                 (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 8 + 4)))));
  lemma_l5_forall32_to_compose orig s4
#pop-options

(* ============================================================================
   LAYER-ASSEMBLY lemmas for ntt_at_layer_5_to_3 (layers 5,4,3).
   Each `lemma_lX_assemble` takes the per-window round_ws posts (with the BOUND
   conjunct added, vs the cross-only lX_round_post) and produces BOTH halves:
   (a) the whole-poly flat congruence to ntt_layer (cross half, = lX_body_tail
       framing), and (b) the whole-poly output bound is_i32b_poly_avx2 NTT_BASE+f.
   ============================================================================ *)

(* ---- Deliverable 1: L5 (STEP=32, STEP_BY=4, 4 windows at offsets 0,8,16,24,
   zeta_r(k+4)).  `r5 a b k` = round_ws's ensures specialized to window k:
   modifies_range [8k,8k+8) + per-unit OUTPUT bound NTT_BASE+3 on the window +
   the 4 cross atoms.  (l5_round_post is the cross-only subset.) ---- *)
unfold let r5
      (a b: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (k:nat{k<4}) : Type0 =
  Spec.Utils.modifies_range_32 a b (mk_usize (8*k)) (mk_usize (8*k+8)) /\
  (Spec.Utils.forall32 (fun i ->
      (i >= 8*k /\ i < 8*k + 8) ==>
      is_i32b_unit_avx2 (8380416 + 3*8380416) (Seq.index b i))) /\
  (Spec.Utils.forall32 (fun u ->
      (u >= 8*k /\ u < 8*k + 4) ==>
      unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 a) u)
        (Seq.index (chunks_of_re_avx2 a) (u + 4))
        (Seq.index (chunks_of_re_avx2 b) u)
        (Seq.index (chunks_of_re_avx2 b) (u + 4))
        (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (k + 4)))))

(* BOUND half for L5, STANDALONE (clean context: NO chunks/createi/cross).  Takes
   just the per-window modifies + OUTPUT-bound conjuncts (the subset of r5 needed)
   and yields the whole-poly output bound.  Symbolic-u dispatch is robust here
   because the context has only the 4 modifies posts + 4 window bounds. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_l5_bound
      (orig s1 s2 s3 s4: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        Spec.Utils.modifies_range_32 orig s1 (mk_usize 0)  (mk_usize 8)  /\
        Spec.Utils.modifies_range_32 s1   s2 (mk_usize 8)  (mk_usize 16) /\
        Spec.Utils.modifies_range_32 s2   s3 (mk_usize 16) (mk_usize 24) /\
        Spec.Utils.modifies_range_32 s3   s4 (mk_usize 24) (mk_usize 32) /\
        (Spec.Utils.forall32 (fun i -> (i >= 0  /\ i < 8 ) ==> is_i32b_unit_avx2 (8380416 + 3*8380416) (Seq.index s1 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 8  /\ i < 16) ==> is_i32b_unit_avx2 (8380416 + 3*8380416) (Seq.index s2 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 16 /\ i < 24) ==> is_i32b_unit_avx2 (8380416 + 3*8380416) (Seq.index s3 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 24 /\ i < 32) ==> is_i32b_unit_avx2 (8380416 + 3*8380416) (Seq.index s4 i))))
      (ensures is_i32b_poly_avx2 (8380416 + 3*8380416) s4) =
  let bound_unit (u:nat{u<32}) : Lemma
      (is_i32b_unit_avx2 (8380416 + 3*8380416) (Seq.index s4 u)) =
    // pick the exit snapshot of window u/8; later windows preserve u.
    let s_exit = (if u<8 then s1 else if u<16 then s2 else if u<24 then s3 else s4) in
    assert (Seq.index s_exit u == Seq.index s4 u) in
  Classical.forall_intro bound_unit;
  lemma_units_to_poly (8380416 + 3*8380416) s4
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_l5_assemble
      (orig s1 s2 s3 s4: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        is_i32b_poly_avx2 (8380416 + 2*8380416) orig /\
        r5 orig s1 0 /\ r5 s1 s2 1 /\ r5 s2 s3 2 /\ r5 s3 s4 3)
      (ensures
        (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig) in
         let out_flat = C.simd_units_to_array (chunks_of_re_avx2 s4) in
         let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 5) in
         forall (i: nat). i < 256 ==>
           (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417) /\
        is_i32b_poly_avx2 (8380416 + 3*8380416) s4) =
  // CROSS half: r5 implies l5_round_post (the cross subset), so reuse the demo.
  l5_body_tail_demo orig s1 s2 s3 s4;
  // BOUND half: delegate to the standalone clean-context lemma (r5's modifies +
  // output-bound conjuncts discharge its requires directly).
  lemma_l5_bound orig s1 s2 s3 s4
#pop-options

(* ---- Deliverable 2: L4 (STEP=16, STEP_BY=2, 8 windows at offsets 4k,
   zeta_r(k+8)).  Input NTT_BASE+3, output NTT_BASE+4; cross mask u%4<2,
   partner +2, zeta_r(u/4+8). ---- *)
unfold let r4
      (a b: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (k:nat{k<8}) : Type0 =
  Spec.Utils.modifies_range_32 a b (mk_usize (4*k)) (mk_usize (4*k+4)) /\
  (Spec.Utils.forall32 (fun i ->
      (i >= 4*k /\ i < 4*k + 4) ==>
      is_i32b_unit_avx2 (8380416 + 4*8380416) (Seq.index b i))) /\
  (Spec.Utils.forall32 (fun u ->
      (u >= 4*k /\ u < 4*k + 2) ==>
      unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 a) u)
        (Seq.index (chunks_of_re_avx2 a) (u + 2))
        (Seq.index (chunks_of_re_avx2 b) u)
        (Seq.index (chunks_of_re_avx2 b) (u + 2))
        (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (k + 8)))))

(* CROSS half for L4 (mirror of l5_body_tail_demo): 8 windows, frame each lo-unit
   cross atom orig->s8 via the modifies posts, compose. Cross-only subset of r4. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_l4_cross
      (orig s1 s2 s3 s4 s5 s6 s7 s8:
        t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        r4 orig s1 0 /\ r4 s1 s2 1 /\ r4 s2 s3 2 /\ r4 s3 s4 3 /\
        r4 s4 s5 4 /\ r4 s5 s6 5 /\ r4 s6 s7 6 /\ r4 s7 s8 7)
      (ensures
        (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig) in
         let out_flat = C.simd_units_to_array (chunks_of_re_avx2 s8) in
         let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 4) in
         forall (i: nat). i < 256 ==>
           (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)) =
  let frame_unit (a b: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                 (u:nat{u<32}) : Lemma (requires Seq.index a u == Seq.index b u)
                                       (ensures Seq.index (chunks_of_re_avx2 a) u ==
                                                Seq.index (chunks_of_re_avx2 b) u)
    = lemma_chunks_frame a b u in
  let win (k:nat{k<8}) (u:nat{4*k <= u /\ u < 4*k+2}) : Lemma
      (unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig) u)
         (Seq.index (chunks_of_re_avx2 orig) (u + 2))
         (Seq.index (chunks_of_re_avx2 s8) u)
         (Seq.index (chunks_of_re_avx2 s8) (u + 2))
         (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (k + 8)))) =
    let s_entry = (match k with
      | 0 -> orig | 1 -> s1 | 2 -> s2 | 3 -> s3 | 4 -> s4 | 5 -> s5 | 6 -> s6 | _ -> s7) in
    let s_exit  = (match k with
      | 0 -> s1   | 1 -> s2 | 2 -> s3 | 3 -> s4 | 4 -> s5 | 5 -> s6 | 6 -> s7 | _ -> s8) in
    frame_unit orig s_entry u; frame_unit orig s_entry (u+2);
    frame_unit s_exit s8 u; frame_unit s_exit s8 (u+2)
  in
  win 0 0; win 0 1; win 1 4; win 1 5; win 2 8; win 2 9; win 3 12; win 3 13;
  win 4 16; win 4 17; win 5 20; win 5 21; win 6 24; win 6 25; win 7 28; win 7 29;
  assert (Spec.Utils.forall32 (fun u ->
            (u % 4 < 2) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig) u)
                                 (Seq.index (chunks_of_re_avx2 orig) (u+2))
                                 (Seq.index (chunks_of_re_avx2 s8) u)
                                 (Seq.index (chunks_of_re_avx2 s8) (u+2))
                                 (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 4 + 8)))));
  lemma_l4_forall32_to_compose orig s8
#pop-options

(* BOUND half for L4, STANDALONE clean context (NO chunks/cross): 8 windows of 4. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_l4_bound
      (orig s1 s2 s3 s4 s5 s6 s7 s8:
        t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        Spec.Utils.modifies_range_32 orig s1 (mk_usize 0)  (mk_usize 4)  /\
        Spec.Utils.modifies_range_32 s1   s2 (mk_usize 4)  (mk_usize 8)  /\
        Spec.Utils.modifies_range_32 s2   s3 (mk_usize 8)  (mk_usize 12) /\
        Spec.Utils.modifies_range_32 s3   s4 (mk_usize 12) (mk_usize 16) /\
        Spec.Utils.modifies_range_32 s4   s5 (mk_usize 16) (mk_usize 20) /\
        Spec.Utils.modifies_range_32 s5   s6 (mk_usize 20) (mk_usize 24) /\
        Spec.Utils.modifies_range_32 s6   s7 (mk_usize 24) (mk_usize 28) /\
        Spec.Utils.modifies_range_32 s7   s8 (mk_usize 28) (mk_usize 32) /\
        (Spec.Utils.forall32 (fun i -> (i >= 0  /\ i < 4 ) ==> is_i32b_unit_avx2 (8380416 + 4*8380416) (Seq.index s1 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 4  /\ i < 8 ) ==> is_i32b_unit_avx2 (8380416 + 4*8380416) (Seq.index s2 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 8  /\ i < 12) ==> is_i32b_unit_avx2 (8380416 + 4*8380416) (Seq.index s3 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 12 /\ i < 16) ==> is_i32b_unit_avx2 (8380416 + 4*8380416) (Seq.index s4 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 16 /\ i < 20) ==> is_i32b_unit_avx2 (8380416 + 4*8380416) (Seq.index s5 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 20 /\ i < 24) ==> is_i32b_unit_avx2 (8380416 + 4*8380416) (Seq.index s6 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 24 /\ i < 28) ==> is_i32b_unit_avx2 (8380416 + 4*8380416) (Seq.index s7 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 28 /\ i < 32) ==> is_i32b_unit_avx2 (8380416 + 4*8380416) (Seq.index s8 i))))
      (ensures is_i32b_poly_avx2 (8380416 + 4*8380416) s8) =
  let bound_unit (u:nat{u<32}) : Lemma
      (is_i32b_unit_avx2 (8380416 + 4*8380416) (Seq.index s8 u)) =
    let s_exit = (if u<4 then s1 else if u<8 then s2 else if u<12 then s3
                  else if u<16 then s4 else if u<20 then s5 else if u<24 then s6
                  else if u<28 then s7 else s8) in
    assert (Seq.index s_exit u == Seq.index s8 u) in
  Classical.forall_intro bound_unit;
  lemma_units_to_poly (8380416 + 4*8380416) s8
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_l4_assemble
      (orig s1 s2 s3 s4 s5 s6 s7 s8:
        t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        is_i32b_poly_avx2 (8380416 + 3*8380416) orig /\
        r4 orig s1 0 /\ r4 s1 s2 1 /\ r4 s2 s3 2 /\ r4 s3 s4 3 /\
        r4 s4 s5 4 /\ r4 s5 s6 5 /\ r4 s6 s7 6 /\ r4 s7 s8 7)
      (ensures
        (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig) in
         let out_flat = C.simd_units_to_array (chunks_of_re_avx2 s8) in
         let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 4) in
         forall (i: nat). i < 256 ==>
           (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417) /\
        is_i32b_poly_avx2 (8380416 + 4*8380416) s8) =
  lemma_l4_cross orig s1 s2 s3 s4 s5 s6 s7 s8;
  lemma_l4_bound orig s1 s2 s3 s4 s5 s6 s7 s8
#pop-options

(* ---- Deliverable 3: L3 (STEP=8, STEP_BY=1, 16 windows at offsets 2k,
   zeta_r(k+16)).  Input NTT_BASE+4, output NTT_BASE+5; cross mask u%2==0,
   partner +1, zeta_r(u/2+16).  Each window has ONE lo-unit (u=2k). ---- *)
unfold let r3
      (a b: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (k:nat{k<16}) : Type0 =
  Spec.Utils.modifies_range_32 a b (mk_usize (2*k)) (mk_usize (2*k+2)) /\
  (Spec.Utils.forall32 (fun i ->
      (i >= 2*k /\ i < 2*k + 2) ==>
      is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index b i))) /\
  (Spec.Utils.forall32 (fun u ->
      (u >= 2*k /\ u < 2*k + 1) ==>
      unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 a) u)
        (Seq.index (chunks_of_re_avx2 a) (u + 1))
        (Seq.index (chunks_of_re_avx2 b) u)
        (Seq.index (chunks_of_re_avx2 b) (u + 1))
        (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (k + 16)))))

(* CROSS half for L3: 16 windows, one lo-unit (u=2k) each, frame orig->s16. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_l3_cross
      (orig s1 s2 s3 s4 s5 s6 s7 s8 s9 s10 s11 s12 s13 s14 s15 s16:
        t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        r3 orig s1 0 /\ r3 s1 s2 1 /\ r3 s2 s3 2 /\ r3 s3 s4 3 /\
        r3 s4 s5 4 /\ r3 s5 s6 5 /\ r3 s6 s7 6 /\ r3 s7 s8 7 /\
        r3 s8 s9 8 /\ r3 s9 s10 9 /\ r3 s10 s11 10 /\ r3 s11 s12 11 /\
        r3 s12 s13 12 /\ r3 s13 s14 13 /\ r3 s14 s15 14 /\ r3 s15 s16 15)
      (ensures
        (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig) in
         let out_flat = C.simd_units_to_array (chunks_of_re_avx2 s16) in
         let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 3) in
         forall (i: nat). i < 256 ==>
           (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)) =
  let frame_unit (a b: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
                 (u:nat{u<32}) : Lemma (requires Seq.index a u == Seq.index b u)
                                       (ensures Seq.index (chunks_of_re_avx2 a) u ==
                                                Seq.index (chunks_of_re_avx2 b) u)
    = lemma_chunks_frame a b u in
  let win (k:nat{k<16}) : Lemma
      (let u = 2*k in
       unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig) u)
         (Seq.index (chunks_of_re_avx2 orig) (u + 1))
         (Seq.index (chunks_of_re_avx2 s16) u)
         (Seq.index (chunks_of_re_avx2 s16) (u + 1))
         (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (k + 16)))) =
    let u = 2*k in
    let s_entry = (match k with
      | 0 -> orig | 1 -> s1 | 2 -> s2 | 3 -> s3 | 4 -> s4 | 5 -> s5 | 6 -> s6 | 7 -> s7
      | 8 -> s8 | 9 -> s9 | 10 -> s10 | 11 -> s11 | 12 -> s12 | 13 -> s13 | 14 -> s14 | _ -> s15) in
    let s_exit  = (match k with
      | 0 -> s1 | 1 -> s2 | 2 -> s3 | 3 -> s4 | 4 -> s5 | 5 -> s6 | 6 -> s7 | 7 -> s8
      | 8 -> s9 | 9 -> s10 | 10 -> s11 | 11 -> s12 | 12 -> s13 | 13 -> s14 | 14 -> s15 | _ -> s16) in
    frame_unit orig s_entry u; frame_unit orig s_entry (u+1);
    frame_unit s_exit s16 u; frame_unit s_exit s16 (u+1)
  in
  win 0; win 1; win 2; win 3; win 4; win 5; win 6; win 7;
  win 8; win 9; win 10; win 11; win 12; win 13; win 14; win 15;
  assert (Spec.Utils.forall32 (fun u ->
            (u % 2 == 0) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig) u)
                                 (Seq.index (chunks_of_re_avx2 orig) (u+1))
                                 (Seq.index (chunks_of_re_avx2 s16) u)
                                 (Seq.index (chunks_of_re_avx2 s16) (u+1))
                                 (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 2 + 16)))));
  lemma_l3_forall32_to_compose orig s16
#pop-options

(* BOUND half for L3, STANDALONE clean context: 16 windows of 2. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_l3_bound
      (orig s1 s2 s3 s4 s5 s6 s7 s8 s9 s10 s11 s12 s13 s14 s15 s16:
        t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        Spec.Utils.modifies_range_32 orig s1 (mk_usize 0)  (mk_usize 2)  /\
        Spec.Utils.modifies_range_32 s1   s2 (mk_usize 2)  (mk_usize 4)  /\
        Spec.Utils.modifies_range_32 s2   s3 (mk_usize 4)  (mk_usize 6)  /\
        Spec.Utils.modifies_range_32 s3   s4 (mk_usize 6)  (mk_usize 8)  /\
        Spec.Utils.modifies_range_32 s4   s5 (mk_usize 8)  (mk_usize 10) /\
        Spec.Utils.modifies_range_32 s5   s6 (mk_usize 10) (mk_usize 12) /\
        Spec.Utils.modifies_range_32 s6   s7 (mk_usize 12) (mk_usize 14) /\
        Spec.Utils.modifies_range_32 s7   s8 (mk_usize 14) (mk_usize 16) /\
        Spec.Utils.modifies_range_32 s8   s9 (mk_usize 16) (mk_usize 18) /\
        Spec.Utils.modifies_range_32 s9   s10 (mk_usize 18) (mk_usize 20) /\
        Spec.Utils.modifies_range_32 s10  s11 (mk_usize 20) (mk_usize 22) /\
        Spec.Utils.modifies_range_32 s11  s12 (mk_usize 22) (mk_usize 24) /\
        Spec.Utils.modifies_range_32 s12  s13 (mk_usize 24) (mk_usize 26) /\
        Spec.Utils.modifies_range_32 s13  s14 (mk_usize 26) (mk_usize 28) /\
        Spec.Utils.modifies_range_32 s14  s15 (mk_usize 28) (mk_usize 30) /\
        Spec.Utils.modifies_range_32 s15  s16 (mk_usize 30) (mk_usize 32) /\
        (Spec.Utils.forall32 (fun i -> (i >= 0  /\ i < 2 ) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s1 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 2  /\ i < 4 ) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s2 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 4  /\ i < 6 ) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s3 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 6  /\ i < 8 ) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s4 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 8  /\ i < 10) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s5 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 10 /\ i < 12) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s6 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 12 /\ i < 14) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s7 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 14 /\ i < 16) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s8 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 16 /\ i < 18) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s9 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 18 /\ i < 20) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s10 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 20 /\ i < 22) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s11 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 22 /\ i < 24) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s12 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 24 /\ i < 26) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s13 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 26 /\ i < 28) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s14 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 28 /\ i < 30) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s15 i))) /\
        (Spec.Utils.forall32 (fun i -> (i >= 30 /\ i < 32) ==> is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s16 i))))
      (ensures is_i32b_poly_avx2 (8380416 + 5*8380416) s16) =
  let bound_unit (u:nat{u<32}) : Lemma
      (is_i32b_unit_avx2 (8380416 + 5*8380416) (Seq.index s16 u)) =
    let s_exit = (if u<2 then s1 else if u<4 then s2 else if u<6 then s3 else if u<8 then s4
                  else if u<10 then s5 else if u<12 then s6 else if u<14 then s7 else if u<16 then s8
                  else if u<18 then s9 else if u<20 then s10 else if u<22 then s11 else if u<24 then s12
                  else if u<26 then s13 else if u<28 then s14 else if u<30 then s15 else s16) in
    assert (Seq.index s_exit u == Seq.index s16 u) in
  Classical.forall_intro bound_unit;
  lemma_units_to_poly (8380416 + 5*8380416) s16
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_l3_assemble
      (orig s1 s2 s3 s4 s5 s6 s7 s8 s9 s10 s11 s12 s13 s14 s15 s16:
        t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires
        is_i32b_poly_avx2 (8380416 + 4*8380416) orig /\
        r3 orig s1 0 /\ r3 s1 s2 1 /\ r3 s2 s3 2 /\ r3 s3 s4 3 /\
        r3 s4 s5 4 /\ r3 s5 s6 5 /\ r3 s6 s7 6 /\ r3 s7 s8 7 /\
        r3 s8 s9 8 /\ r3 s9 s10 9 /\ r3 s10 s11 10 /\ r3 s11 s12 11 /\
        r3 s12 s13 12 /\ r3 s13 s14 13 /\ r3 s14 s15 14 /\ r3 s15 s16 15)
      (ensures
        (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig) in
         let out_flat = C.simd_units_to_array (chunks_of_re_avx2 s16) in
         let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 3) in
         forall (i: nat). i < 256 ==>
           (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417) /\
        is_i32b_poly_avx2 (8380416 + 5*8380416) s16) =
  lemma_l3_cross orig s1 s2 s3 s4 s5 s6 s7 s8 s9 s10 s11 s12 s13 s14 s15 s16;
  lemma_l3_bound orig s1 s2 s3 s4 s5 s6 s7 s8 s9 s10 s11 s12 s13 s14 s15 s16
#pop-options

(* ============================================================================
   Deliverable 4: THE KEY VALIDATION.  ntt_5to3_l5block_proto actually CALLS
   round_ws 4 times (the L5 windows k=0,1,2,3 at offsets 0,8,16,24) snapshotting
   s0=orig,s1,s2,s3,s4, then calls lemma_l5_assemble.  The POINT: show each
   round_ws call's WINDOW requires (the 8 window units bounded at NTT_BASE+2)
   discharges — for k=0 directly from orig's poly bound; for k>0 because the
   window [8k,8k+8) is untouched by the earlier rounds (which modified [0,8k)),
   so still == orig there.  Recipe ported to the real Avx2.Ntt.ntt_at_layer_5_to_3.
   ============================================================================ *)
(* Window-discharge helper (clean context): given orig's whole-poly NTT_BASE+2
   bound and the chain of EARLIER rounds' modifies posts (each modifying a range
   strictly below `lo`), the window [lo,lo+8) of `cur` is untouched => == orig =>
   still bounded NTT_BASE+2 (the source for the NEXT round_ws call's requires).
   Stated per concrete window via the modifies posts actually in scope. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_l5_proto_w1   // window [8,16) of s1 (after round 0 modified [0,8))
      (orig s1: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires (forall (u:nat). u < 32 ==> is_i32b_unit_avx2 (8380416 + 2*8380416) (Seq.index orig u)) /\
                Spec.Utils.modifies_range_32 orig s1 (mk_usize 0) (mk_usize 8))
      (ensures Spec.Utils.forall32 (fun i -> (i >= 8 /\ i < 16) ==>
                 is_i32b_unit_avx2 (8380416 + 2*8380416) (Seq.index s1 i))) = ()

let lemma_l5_proto_w2   // window [16,24) of s2 (rounds 0,1 modified [0,16))
      (orig s1 s2: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires (forall (u:nat). u < 32 ==> is_i32b_unit_avx2 (8380416 + 2*8380416) (Seq.index orig u)) /\
                Spec.Utils.modifies_range_32 orig s1 (mk_usize 0) (mk_usize 8) /\
                Spec.Utils.modifies_range_32 s1   s2 (mk_usize 8) (mk_usize 16))
      (ensures Spec.Utils.forall32 (fun i -> (i >= 16 /\ i < 24) ==>
                 is_i32b_unit_avx2 (8380416 + 2*8380416) (Seq.index s2 i))) = ()

let lemma_l5_proto_w3   // window [24,32) of s3 (rounds 0,1,2 modified [0,24))
      (orig s1 s2 s3: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Lemma
      (requires (forall (u:nat). u < 32 ==> is_i32b_unit_avx2 (8380416 + 2*8380416) (Seq.index orig u)) /\
                Spec.Utils.modifies_range_32 orig s1 (mk_usize 0)  (mk_usize 8)  /\
                Spec.Utils.modifies_range_32 s1   s2 (mk_usize 8)  (mk_usize 16) /\
                Spec.Utils.modifies_range_32 s2   s3 (mk_usize 16) (mk_usize 24))
      (ensures Spec.Utils.forall32 (fun i -> (i >= 24 /\ i < 32) ==>
                 is_i32b_unit_avx2 (8380416 + 2*8380416) (Seq.index s3 i))) = ()
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let ntt_5to3_l5block_proto
      (orig: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
    : Prims.Pure (t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (requires is_i32b_poly_avx2 (8380416 + 2*8380416) orig)
      (ensures fun s4 ->
        (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 orig) in
         let out_flat = C.simd_units_to_array (chunks_of_re_avx2 s4) in
         let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 5) in
         forall (i: nat). i < 256 ==>
           (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417) /\
        is_i32b_poly_avx2 (8380416 + 3*8380416) s4) =
  // bridge the concrete zeta literals to the symbolic zeta_r the r5 posts mention.
  assert_norm (Spec.MLDSA.NttConstants.zeta_r 4 == 237124);
  assert_norm (Spec.MLDSA.NttConstants.zeta_r 5 == (-777960));
  assert_norm (Spec.MLDSA.NttConstants.zeta_r 6 == (-876248));
  assert_norm (Spec.MLDSA.NttConstants.zeta_r 7 == 466468);
  // all 32 units of orig are bounded NTT_BASE+2 (the round window requires source).
  lemma_poly_to_units (8380416 + 2*8380416) orig;
  // window 0 requires: [0,8) bounded from orig directly (in the forall above).
  let s1 = round_ws (mk_usize 32) (mk_usize 4) orig (mk_usize 0) (mk_i32 237124) in
  lemma_l5_proto_w1 orig s1;            // discharge window-1 requires
  let s2 = round_ws (mk_usize 32) (mk_usize 4) s1 (mk_usize 1) (mk_i32 (-777960)) in
  lemma_l5_proto_w2 orig s1 s2;         // discharge window-2 requires
  let s3 = round_ws (mk_usize 32) (mk_usize 4) s2 (mk_usize 2) (mk_i32 (-876248)) in
  lemma_l5_proto_w3 orig s1 s2 s3;      // discharge window-3 requires
  let s4 = round_ws (mk_usize 32) (mk_usize 4) s3 (mk_usize 3) (mk_i32 466468) in
  // the 4 round posts are exactly r5 orig s1 0 .. r5 s3 s4 3 (zeta literals == zeta_r).
  lemma_l5_assemble orig s1 s2 s3 s4;
  s4
#pop-options

(* ============================================================================
   PHASE 0 REDESIGN (2026-06-08): opaque-atom round-post theory.

   Defeats the block-fn accumulation cliff (ntt_l4_block / ntt_l3_block > rlimit
   800).  ROOT CAUSE: round_ws's whole post (modifies_range_32 [32] + bound
   forall32 [32] + cross forall32 [32 opaque atoms]) is TRANSPARENT, so a block
   fn that chains N rounds piles ~96*N raw conjuncts into its WP.

   FIX (memory feedback_opaque_predicate_lemma_api): seal the whole round post
   into ONE opaque atom `round_post_avx2`; handle window-untouched reasoning via
   an opaque frame algebra `modifies_win`.  The opaque predicate + its FIXED
   lemma set IS its interface; `reveal_opaque` is confined to the primitive
   theory lemmas below.  Block fns / assemble / compose reason ONLY via the
   lemma API and NEVER reveal inline -> each block fn's WP carries N small
   opaque ATOMS, not N*96 conjuncts.  Window-scoped bounds are preserved.
   ============================================================================ *)

(* ---- 0.1  modifies frame ALGEBRA (opaque predicate + lookup/union/refl) ----
   `modifies_win a b lo hi` : b agrees with a everywhere OUTSIDE [lo,hi).
   The block fn accumulates "prefix [0,lo) touched" as ONE opaque atom and grows
   it by range-union, never carrying the 32-conjunct modifies_range_32 in its WP. *)
[@@ "opaque_to_smt"]
let modifies_win (a b: av32) (lo hi:nat) : Type0 =
  lo <= hi /\ hi <= 32 /\
  (forall (u:nat). u < 32 /\ (u < lo \/ u >= hi) ==> Seq.index a u == Seq.index b u)

(* window-bound OPAQUE atom: units [lo,lo+width) of `re` are bounded at `bnd`.
   THIS is the keystone of the rlimit-4 block fn: round_ws_sealed's window
   requires is this sealed atom, NOT a transparent forall32 (32 conjuncts).
   Carrying 16 win_bounded atoms instead of 16*32 conjuncts is what collapses
   the block WP from churning-70min to 4/800. *)
[@@ "opaque_to_smt"]
let win_bounded (re: av32) (lo width bnd:nat) : Type0 =
  Spec.Utils.forall32 (fun i -> (i >= lo /\ i < lo + width) ==>
    is_i32b_unit_avx2 bnd (Seq.index re i))

(* window-cross OPAQUE atom: the per-window butterfly cross relation, sealed so
   round_post_avx2's body references it as ONE atom (NOT a transparent forall32 of
   unit_post_cross + createi chunks).  This is THE fix: round_post's body must be
   3 opaque sub-atoms, else the gated-def body re-introduces the forall32+createi
   soup at every round_post occurrence (16x) and the block WP cascades. *)
[@@ "opaque_to_smt"]
let win_cross (a b: av32) (offset:nat{offset < 32})
      (step_by:nat{step_by > 0 /\ offset + 2*step_by <= 32}) (zeta:i32) : Type0 =
  Spec.Utils.is_i32b 4190208 zeta ==>
  Spec.Utils.forall32 (fun u -> (u >= offset /\ u < offset + step_by) ==>
    unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 a) u)
                         (Seq.index (chunks_of_re_avx2 a) (u + step_by))
                         (Seq.index (chunks_of_re_avx2 b) u)
                         (Seq.index (chunks_of_re_avx2 b) (u + step_by))
                         zeta)

(* ---- 0.2  round_post_avx2 : ONE opaque atom = round_ws's whole ensures, BUT its
   body references only the 3 OPAQUE sub-atoms (modifies_win + win_bounded + win_cross).
   CRITICAL: the body must NOT contain transparent modifies_range_32 / forall32 /
   createi — those re-introduce the 96-conj soup at every round_post occurrence (16x
   in a block) and the WP cascades (rlimit 800 saturate -> 70min).  With this LIGHT
   body the L3 block fn is rlimit ~0.7 (validated in ScratchPhase0e). *)
[@@ "opaque_to_smt"]
let round_post_avx2
      (a b: av32)
      (offset:nat{offset < 32}) (step_by:nat{step_by > 0 /\ offset + 2*step_by <= 32})
      (zeta:i32) : Type0 =
  modifies_win a b offset (offset + 2*step_by) /\
  win_bounded b offset (2*step_by)
    (8380416 + (layer_bound_factor_avx2 step_by + 1) * 8380416) /\
  win_cross a b offset step_by zeta

(* ---- 0.1c  layer_done : OPAQUE functional-post atom = "flat(b) == ntt_layer(flat a) layer
   mod q".  Sealing this is the SECOND half of the rlimit-4 block fn: the raw
   congruence mentions chunks_of_re_avx2 (a nested createi) + a forall-over-256 +
   ntt_layer; leaving it transparent in the block fn's ensures re-triggers the
   createi_lemma SMTPat cascade.  The block fns + assemble produce this atom; the
   layer-chaining / top compose reveals it via lemma_layer_done_reveal. ---- *)
[@@ "opaque_to_smt"]
let layer_done (a b: av32) (layer:nat{layer < 8}) : Type0 =
  (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 a) in
   let out_flat = C.simd_units_to_array (chunks_of_re_avx2 b) in
   let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize layer) in
   forall (i: nat). i < 256 ==>
     (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)

let lemma_modwin_refl (a: av32) (lo hi:nat)
    : Lemma (requires lo <= hi /\ hi <= 32) (ensures modifies_win a a lo hi)
  = reveal_opaque (`%modifies_win) modifies_win

let lemma_modwin_lookup (a b: av32) (lo hi:nat) (u:nat{u<32})
    : Lemma (requires modifies_win a b lo hi /\ (u < lo \/ u >= hi))
            (ensures Seq.index a u == Seq.index b u)
  = reveal_opaque (`%modifies_win) modifies_win

(* range-union: a->b touches [lo,mid), b->c touches [mid,hi) ==> a->c touches [lo,hi). *)
let lemma_modwin_union (a b c: av32) (lo mid hi:nat)
    : Lemma (requires modifies_win a b lo mid /\ modifies_win b c mid hi /\
                      lo <= mid /\ mid <= hi /\ hi <= 32)
            (ensures modifies_win a c lo hi)
  = reveal_opaque (`%modifies_win) modifies_win

(* bridges modifies_win (opaque) <-> Spec.Utils.modifies_range_32 (transparent 32-conj).
   The 32-conj expansion lives ONLY in these two leaf VCs, never in a consumer. *)
let lemma_range32_to_modwin (a b: av32) (i j:usize{v i < 32 /\ v j <= 32 /\ v i <= v j})
    : Lemma (requires Spec.Utils.modifies_range_32 a b i j)
            (ensures modifies_win a b (v i) (v j))
  = reveal_opaque (`%modifies_win) modifies_win

let lemma_modwin_to_range32 (a b: av32) (lo hi:nat{lo < 32 /\ hi <= 32 /\ lo <= hi})
    : Lemma (requires modifies_win a b lo hi)
            (ensures Spec.Utils.modifies_range_32 a b (mk_usize lo) (mk_usize hi))
  = reveal_opaque (`%modifies_win) modifies_win

(* window-bound discharge: if cur agrees with orig on [lo,32) (= modifies_win
   orig cur 0 lo) and orig's whole poly is bounded at `bnd`, then cur's window
   [lo,lo+width) is bounded -> the (opaque) next round_ws_sealed window requires.
   The forall32 (32 conj) is expanded HERE, in this leaf's clean VC, not the block. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
let lemma_window_from_modwin (orig cur: av32) (lo width bnd:nat)
    : Lemma (requires modifies_win orig cur 0 lo /\ lo + width <= 32 /\
                      is_i32b_poly_avx2 bnd orig)
            (ensures win_bounded cur lo width bnd)
  = reveal_opaque (`%modifies_win) modifies_win;
    reveal_opaque (`%win_bounded) win_bounded;
    reveal_opaque (`%is_i32b_poly_avx2) is_i32b_poly_avx2;
    reveal_opaque (`%is_i32b_unit_avx2) is_i32b_unit_avx2
#pop-options

let lemma_layer_done_intro (a b: av32) (layer:nat{layer < 8})
    : Lemma
      (requires
        (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 a) in
         let out_flat = C.simd_units_to_array (chunks_of_re_avx2 b) in
         let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize layer) in
         forall (i: nat). i < 256 ==>
           (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
      (ensures layer_done a b layer)
  = reveal_opaque (`%layer_done) layer_done

let lemma_layer_done_reveal (a b: av32) (layer:nat{layer < 8})
    : Lemma
      (requires layer_done a b layer)
      (ensures
        (let in_flat = C.simd_units_to_array (chunks_of_re_avx2 a) in
         let out_flat = C.simd_units_to_array (chunks_of_re_avx2 b) in
         let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize layer) in
         forall (i: nat). i < 256 ==>
           (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = reveal_opaque (`%layer_done) layer_done

(* intro from the 3 sub-atoms (used by round_ws_sealed). *)
let lemma_rp_intro
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
  = reveal_opaque (`%round_post_avx2) round_post_avx2

(* extractions: round post -> each opaque sub-atom (reveal of round_post ONLY;
   the sub-atoms stay opaque, so consumers carry small atoms). *)
let lemma_rp_modwin
      (a b: av32)
      (offset:nat{offset < 32}) (step_by:nat{step_by > 0 /\ offset + 2*step_by <= 32})
      (zeta:i32)
    : Lemma (requires round_post_avx2 a b offset step_by zeta)
            (ensures modifies_win a b offset (offset + 2*step_by))
  = reveal_opaque (`%round_post_avx2) round_post_avx2

let lemma_rp_bound
      (a b: av32)
      (offset:nat{offset < 32}) (step_by:nat{step_by > 0 /\ offset + 2*step_by <= 32})
      (zeta:i32)
    : Lemma (requires round_post_avx2 a b offset step_by zeta)
            (ensures win_bounded b offset (2*step_by)
                       (8380416 + (layer_bound_factor_avx2 step_by + 1) * 8380416))
  = reveal_opaque (`%round_post_avx2) round_post_avx2

let lemma_rp_cross
      (a b: av32)
      (offset:nat{offset < 32}) (step_by:nat{step_by > 0 /\ offset + 2*step_by <= 32})
      (zeta:i32)
    : Lemma (requires round_post_avx2 a b offset step_by zeta)
            (ensures win_cross a b offset step_by zeta)
  = reveal_opaque (`%round_post_avx2) round_post_avx2

(* ---- 0.3  round_ws_sealed : the producer.  Calls round_ws (transparent post),
   seals into the opaque atom.  The block fn calls THIS, gets one atom per round. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let round_ws_sealed
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
          round_post_avx2 re re_future offset (v v_STEP_BY) zeta) =
  let offset_n : nat = ((v index) * (v v_STEP) * 2) / 8 in
  // win_bounded reveal: (a) discharge round_ws's window-requires forall32 from the
  // input win_bounded atom, AND (b) re-seal the output bound from round_ws's post.
  // win_cross reveal: re-seal the cross post from round_ws's forall32 cross.
  reveal_opaque (`%win_bounded) win_bounded;
  reveal_opaque (`%win_cross) win_cross;
  let re_future = round_ws v_STEP v_STEP_BY re index zeta in
  lemma_range32_to_modwin re re_future
    (mk_usize offset_n) (mk_usize (offset_n + 2 * (v v_STEP_BY)));
  lemma_rp_intro re re_future offset_n (v v_STEP_BY) zeta;
  re_future
#pop-options

(* assemble over opaque atoms: reveal once (-> 16 r3 bodies) then call the
   already-validated transparent lemma_l3_assemble.  The heavy reveal lives in
   THIS theory lemma, not the block fn. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_l3_assemble_o
      (orig s1 s2 s3 s4 s5 s6 s7 s8 s9 s10 s11 s12 s13 s14 s15 s16: av32)
    : Lemma
      (requires
        is_i32b_poly_avx2 (8380416 + 4*8380416) orig /\
        r3o orig s1 0 /\ r3o s1 s2 1 /\ r3o s2 s3 2 /\ r3o s3 s4 3 /\
        r3o s4 s5 4 /\ r3o s5 s6 5 /\ r3o s6 s7 6 /\ r3o s7 s8 7 /\
        r3o s8 s9 8 /\ r3o s9 s10 9 /\ r3o s10 s11 10 /\ r3o s11 s12 11 /\
        r3o s12 s13 12 /\ r3o s13 s14 13 /\ r3o s14 s15 14 /\ r3o s15 s16 15)
      (ensures layer_done orig s16 3 /\ is_i32b_poly_avx2 (8380416 + 5*8380416) s16) =
  // unseal the 16 light round posts back to transparent r3 (modifies_range_32 +
  // forall32 bound + forall32 cross) for the already-validated lemma_l3_assemble.
  // Heavy, but ONCE in this theory lemma's VC, never in a block fn.
  reveal_opaque (`%round_post_avx2) round_post_avx2;
  reveal_opaque (`%win_bounded) win_bounded;
  reveal_opaque (`%win_cross) win_cross;
  lemma_modwin_to_range32 orig s1 0 2;   lemma_modwin_to_range32 s1 s2 2 4;
  lemma_modwin_to_range32 s2 s3 4 6;     lemma_modwin_to_range32 s3 s4 6 8;
  lemma_modwin_to_range32 s4 s5 8 10;    lemma_modwin_to_range32 s5 s6 10 12;
  lemma_modwin_to_range32 s6 s7 12 14;   lemma_modwin_to_range32 s7 s8 14 16;
  lemma_modwin_to_range32 s8 s9 16 18;   lemma_modwin_to_range32 s9 s10 18 20;
  lemma_modwin_to_range32 s10 s11 20 22; lemma_modwin_to_range32 s11 s12 22 24;
  lemma_modwin_to_range32 s12 s13 24 26; lemma_modwin_to_range32 s13 s14 26 28;
  lemma_modwin_to_range32 s14 s15 28 30; lemma_modwin_to_range32 s15 s16 30 32;
  lemma_l3_assemble orig s1 s2 s3 s4 s5 s6 s7 s8 s9 s10 s11 s12 s13 s14 s15 s16;
  lemma_layer_done_intro orig s16 3
#pop-options

(* L5 assemble over opaque atoms: reveal once (-> 4 r5 bodies) then call the
   already-validated transparent lemma_l5_assemble. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_l5_assemble_o (orig s1 s2 s3 s4: av32)
    : Lemma
      (requires is_i32b_poly_avx2 (8380416 + 2*8380416) orig /\
                r5o orig s1 0 /\ r5o s1 s2 1 /\ r5o s2 s3 2 /\ r5o s3 s4 3)
      (ensures layer_done orig s4 5 /\ is_i32b_poly_avx2 (8380416 + 3*8380416) s4) =
  reveal_opaque (`%round_post_avx2) round_post_avx2;
  reveal_opaque (`%win_bounded) win_bounded;
  reveal_opaque (`%win_cross) win_cross;
  lemma_modwin_to_range32 orig s1 0 8;   lemma_modwin_to_range32 s1 s2 8 16;
  lemma_modwin_to_range32 s2 s3 16 24;   lemma_modwin_to_range32 s3 s4 24 32;
  lemma_l5_assemble orig s1 s2 s3 s4;
  lemma_layer_done_intro orig s4 5
#pop-options

(* L4 assemble over opaque atoms: reveal once (-> 8 r4 bodies) then call the
   already-validated transparent lemma_l4_assemble. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400"
let lemma_l4_assemble_o (orig s1 s2 s3 s4 s5 s6 s7 s8: av32)
    : Lemma
      (requires is_i32b_poly_avx2 (8380416 + 3*8380416) orig /\
                r4o orig s1 0 /\ r4o s1 s2 1 /\ r4o s2 s3 2 /\ r4o s3 s4 3 /\
                r4o s4 s5 4 /\ r4o s5 s6 5 /\ r4o s6 s7 6 /\ r4o s7 s8 7)
      (ensures layer_done orig s8 4 /\ is_i32b_poly_avx2 (8380416 + 4*8380416) s8) =
  reveal_opaque (`%round_post_avx2) round_post_avx2;
  reveal_opaque (`%win_bounded) win_bounded;
  reveal_opaque (`%win_cross) win_cross;
  lemma_modwin_to_range32 orig s1 0 4;   lemma_modwin_to_range32 s1 s2 4 8;
  lemma_modwin_to_range32 s2 s3 8 12;    lemma_modwin_to_range32 s3 s4 12 16;
  lemma_modwin_to_range32 s4 s5 16 20;   lemma_modwin_to_range32 s5 s6 20 24;
  lemma_modwin_to_range32 s6 s7 24 28;   lemma_modwin_to_range32 s7 s8 28 32;
  lemma_l4_assemble orig s1 s2 s3 s4 s5 s6 s7 s8;
  lemma_layer_done_intro orig s8 4
#pop-options

(* ---- 5->3 chaining: compose 3 sealed layer_done atoms (layers 5,4,3) into a
   3-layer composite atom.  The reveal of layer_done (-> flat congruence) + the
   flat-level cong chaining (Avx2NttCompose.lemma_ntt_layer_{3,4}_cong) live in
   THIS theory lemma; the consumer ntt_at_layer_5_to_3 stays light (3 block calls
   + this one lemma).  Mirrors lemma_ntt_compose_avx2's ghost-state style. ---- *)
[@@ "opaque_to_smt"]
let comp_5_3_done (a b: av32) : Type0 =
  (let in_flat  = C.simd_units_to_array (chunks_of_re_avx2 a) in
   let out_flat = C.simd_units_to_array (chunks_of_re_avx2 b) in
   let spec = Hacspec_ml_dsa.Ntt.ntt_layer
                (Hacspec_ml_dsa.Ntt.ntt_layer
                   (Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 5)) (mk_usize 4)) (mk_usize 3) in
   forall (i: nat). i < 256 ==>
     (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_compose_5_3_o (orig sL5 sL4 sL3: av32)
    : Lemma
      (requires layer_done orig sL5 5 /\ layer_done sL5 sL4 4 /\ layer_done sL4 sL3 3)
      (ensures comp_5_3_done orig sL3) =
  lemma_layer_done_reveal orig sL5 5;
  lemma_layer_done_reveal sL5 sL4 4;
  lemma_layer_done_reveal sL4 sL3 3;
  let f0  = C.simd_units_to_array (chunks_of_re_avx2 orig) in
  let fL5 = C.simd_units_to_array (chunks_of_re_avx2 sL5) in
  let fL4 = C.simd_units_to_array (chunks_of_re_avx2 sL4) in
  let fL3 = C.simd_units_to_array (chunks_of_re_avx2 sL3) in
  // H1: fL5 ≡ ntt_layer f0 5
  let g5 = Hacspec_ml_dsa.Ntt.ntt_layer f0 (mk_usize 5) in
  assert (forall (i:nat). i < 256 ==> (v (Seq.index fL5 i)) % 8380417 == (v (Seq.index g5 i)) % 8380417);
  Avx2NttCompose.lemma_ntt_layer_4_cong fL5 g5;
  let g4 = Hacspec_ml_dsa.Ntt.ntt_layer g5 (mk_usize 4) in
  assert (Hacspec_ml_dsa.Ntt.ntt_layer fL5 (mk_usize 4) == g4);
  // H2: fL4 ≡ ntt_layer fL5 4 == g4
  assert (forall (i:nat). i < 256 ==> (v (Seq.index fL4 i)) % 8380417 == (v (Seq.index g4 i)) % 8380417);
  Avx2NttCompose.lemma_ntt_layer_3_cong fL4 g4;
  let g3 = Hacspec_ml_dsa.Ntt.ntt_layer g4 (mk_usize 3) in
  assert (Hacspec_ml_dsa.Ntt.ntt_layer fL4 (mk_usize 3) == g3);
  // H3: fL3 ≡ ntt_layer fL4 3 == g3 = comp spec
  assert (forall (i:nat). i < 256 ==> (v (Seq.index fL3 i)) % 8380417 == (v (Seq.index g3 i)) % 8380417);
  reveal_opaque (`%comp_5_3_done) comp_5_3_done
#pop-options


(* ============================================================================
   PHASE 2 — 7_6 two-layer composite (layers 7 then 6).  The impl fuses L7+L6 into
   ONE fn with 32 interleaved vector-pair muls.  Architecture:
   - round_cross_sealed : bnd-PARAMETRIC per-PAIR producer (mirror of round_ws__round
     but for the cross step_by 8/16, where layer_bound_factor is 0 so the within-chunk
     producer's bound threading doesn't fit).  Post = modifies2_32 + per-unit out-bound
     + unit_post_cross_avx2 on the pair.
   - lemma_l7_pairs_to_layer / lemma_l6_pairs_to_layer : take the forall32 of pair cross
     atoms (already framed to (orig,mid) / (mid,out)) -> layer_done via the cross drivers.
   - comp_7_6_done + lemma_compose_7_6_o : 2-layer composite atom (mirror comp_5_3_done).
   ============================================================================ *)

(* per-PAIR producer for the cross layers.  bnd explicit (NTT_BASE for L7, NTT_BASE+1
   for L6); step_by is 16 (L7) or 8 (L6).  Verbatim butterfly from round_ws__round. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"
let round_cross_sealed
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
                               zeta) =
  let rhs:Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256) =
    Libcrux_intrinsics.Avx2.mm256_set1_epi32 zeta
  in
  let re_in:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) = re in
  // ---- butterfly (verbatim from round_ws__round / extracted ___mul) ----
  let re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re
      (index +! step_by <: usize)
      ({
          (re.[ index +! step_by <: usize ] <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) with
          Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
          =
          Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply (re.[ index +! step_by <: usize ]
              <:
              Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
              .Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value rhs
          <:
          Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)
        }
        <:
        Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
  in
  let tmp:Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256) =
    Libcrux_intrinsics.Avx2.mm256_sub_epi32 (re.[ index ]
        <:
        Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
        .Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
      (re.[ index +! step_by <: usize ] <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
        .Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
  in
  let re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re
      index
      ({
          Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
          =
          Libcrux_intrinsics.Avx2.mm256_add_epi32 (re.[ index ]
              <:
              Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
              .Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
            (re.[ index +! step_by <: usize ] <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
              .Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value
          <:
          Libcrux_core_models.Abstractions.Bitvec.t_BitVec (mk_u64 256)
        }
        <:
        Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
  in
  let re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) =
    Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re
      (index +! step_by <: usize)
      ({ Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value = tmp }
        <:
        Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256)
  in
  let _:Prims.unit =
    let re0 = (Seq.index re_in (v index)).f_value in
    let re1 = (Seq.index re_in (v index + v step_by)).f_value in
    let nre0 = (Seq.index re (v index)).f_value in
    let nre1 = (Seq.index re (v index + v step_by)).f_value in
    let lane (i:nat{i<8}) : Lemma
        ((to_i32x8 nre0 (mk_u64 i), to_i32x8 nre1 (mk_u64 i)) ==
         ntt_step (to_i32x8 rhs (mk_int i))
           (to_i32x8 re0 (mk_u64 i), to_i32x8 re1 (mk_u64 i)))
      = assert (to_i32x8 rhs (mk_int i) == zeta);
        assert (to_i32x8 rhs (mk_u64 i) == zeta)
    in
    lane 0; lane 1; lane 2; lane 3; lane 4; lane 5; lane 6; lane 7;
    assert (Spec.Utils.forall8 (fun i ->
             (to_i32x8 nre0 (mk_u64 i), to_i32x8 nre1 (mk_u64 i)) ==
             ntt_step (to_i32x8 rhs (mk_int i))
               (to_i32x8 re0 (mk_u64 i), to_i32x8 re1 (mk_u64 i))));
    introduce forall (l:nat). l < 8 ==> to_i32x8 rhs (mk_int l) == zeta
    with (introduce l < 8 ==> to_i32x8 rhs (mk_int l) == zeta
          with _. assert (to_i32x8 rhs (mk_int l) == zeta));
    lemma_cross_pair_relations_ws re_in re bnd (v index) (v index + v step_by) rhs zeta;
    lemma_is_i32b_unit_avx2_intro (bnd + 8380416) (Seq.index re (v index));
    lemma_is_i32b_unit_avx2_intro (bnd + 8380416) (Seq.index re (v index + v step_by))
  in
  re
#pop-options

(* L7 pairs -> layer_done 7.  Takes the forall32 of pair cross atoms about (orig,mid)
   and the input bound; produces layer_done orig mid 7 + the +1 output bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_l7_pairs_to_layer (orig mid: av32)
    : Lemma
      (requires
        Spec.Utils.forall32 (fun u ->
          (u % 32 < 16) ==>
          unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig) u)
                               (Seq.index (chunks_of_re_avx2 orig) (u+16))
                               (Seq.index (chunks_of_re_avx2 mid) u)
                               (Seq.index (chunks_of_re_avx2 mid) (u+16))
                               (mk_i32 (zeta_r (u / 32 + 1)))))
      (ensures layer_done orig mid 7) =
  lemma_l7_cross_driver_compose_avx2 orig mid;
  lemma_layer_done_intro orig mid 7
#pop-options

(* L6 pairs -> layer_done 6.  Takes the forall32 of pair cross atoms about (mid,out). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_l6_pairs_to_layer (mid out: av32)
    : Lemma
      (requires
        Spec.Utils.forall32 (fun u ->
          (u % 16 < 8) ==>
          unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 mid) u)
                               (Seq.index (chunks_of_re_avx2 mid) (u+8))
                               (Seq.index (chunks_of_re_avx2 out) u)
                               (Seq.index (chunks_of_re_avx2 out) (u+8))
                               (mk_i32 (zeta_r (u / 16 + 2)))))
      (ensures layer_done mid out 6) =
  lemma_l6_cross_driver_compose_avx2 mid out;
  lemma_layer_done_intro mid out 6
#pop-options

(* ---- 7->6 chaining: compose 2 sealed layer_done atoms (layers 7,6) into a 2-layer
   composite atom.  Mirror of lemma_compose_5_3_o (one fewer layer; no layer_7 cong
   needed — only layer_6 cong to push fL7 through ntt_layer 6). ---- *)
[@@ "opaque_to_smt"]
let comp_7_6_done (a b: av32) : Type0 =
  (let in_flat  = C.simd_units_to_array (chunks_of_re_avx2 a) in
   let out_flat = C.simd_units_to_array (chunks_of_re_avx2 b) in
   let spec = Hacspec_ml_dsa.Ntt.ntt_layer
                (Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 7)) (mk_usize 6) in
   forall (i: nat). i < 256 ==>
     (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)

#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_compose_7_6_o (orig sL7 sL6: av32)
    : Lemma
      (requires layer_done orig sL7 7 /\ layer_done sL7 sL6 6)
      (ensures comp_7_6_done orig sL6) =
  lemma_layer_done_reveal orig sL7 7;
  lemma_layer_done_reveal sL7 sL6 6;
  let f0  = C.simd_units_to_array (chunks_of_re_avx2 orig) in
  let fL7 = C.simd_units_to_array (chunks_of_re_avx2 sL7) in
  let fL6 = C.simd_units_to_array (chunks_of_re_avx2 sL6) in
  // H1: fL7 ≡ ntt_layer f0 7
  let g7 = Hacspec_ml_dsa.Ntt.ntt_layer f0 (mk_usize 7) in
  assert (forall (i:nat). i < 256 ==> (v (Seq.index fL7 i)) % 8380417 == (v (Seq.index g7 i)) % 8380417);
  Avx2NttCompose.lemma_ntt_layer_6_cong fL7 g7;
  let g6 = Hacspec_ml_dsa.Ntt.ntt_layer g7 (mk_usize 6) in
  assert (Hacspec_ml_dsa.Ntt.ntt_layer fL7 (mk_usize 6) == g6);
  // H2: fL6 ≡ ntt_layer fL7 6 == g6 = comp spec
  assert (forall (i:nat). i < 256 ==> (v (Seq.index fL6 i)) % 8380417 == (v (Seq.index g6 i)) % 8380417);
  reveal_opaque (`%comp_7_6_done) comp_7_6_done
#pop-options

(* bnd-EXPLICIT clean-context maintainer (mirror of lemma_round_ws_maintains but
   bnd-parametric — the cross layers L6 take NTT_BASE+1 input, not what
   layer_bound_factor gives).  Entry inv at j + round_cross_sealed post on pair
   (j,j+step_by) -> exit inv at j+1. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_round_cross_maintains
      (orig_re re_old re_new: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (offset j step_by: usize)
      (bnd:nat{bnd + 8380416 < pow2 31})
      (zeta:i32{Spec.Utils.is_i32b 4190208 zeta})
    : Lemma
      (requires
        v step_by > 0 /\ v offset <= v j /\ v j < v offset + v step_by /\
        v offset + 2 * v step_by <= 32 /\
        Spec.Utils.modifies_range2_32 orig_re re_old offset j (offset +! step_by) (j +! step_by) /\
        (Spec.Utils.forall32 (fun i ->
            ((i >= v offset /\ i < v j) \/ (i >= v offset + v step_by /\ i < v j + v step_by)) ==>
            is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_old i))) /\
        (Spec.Utils.forall32 (fun u ->
            (u >= v offset /\ u < v j) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig_re) u)
              (Seq.index (chunks_of_re_avx2 orig_re) (u + v step_by))
              (Seq.index (chunks_of_re_avx2 re_old) u)
              (Seq.index (chunks_of_re_avx2 re_old) (u + v step_by)) zeta)) /\
        Spec.Utils.modifies2_32 re_old re_new j (j +! step_by) /\
        is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_new (v j)) /\
        is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_new (v j + v step_by)) /\
        unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 re_old) (v j))
          (Seq.index (chunks_of_re_avx2 re_old) (v j + v step_by))
          (Seq.index (chunks_of_re_avx2 re_new) (v j))
          (Seq.index (chunks_of_re_avx2 re_new) (v j + v step_by)) zeta)
      (ensures
        Spec.Utils.modifies_range2_32 orig_re re_new offset (j +! mk_usize 1)
          (offset +! step_by) ((j +! mk_usize 1) +! step_by) /\
        (Spec.Utils.forall32 (fun i ->
            ((i >= v offset /\ i < v j + 1) \/ (i >= v offset + v step_by /\ i < v j + 1 + v step_by)) ==>
            is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_new i))) /\
        (Spec.Utils.forall32 (fun u ->
            (u >= v offset /\ u < v j + 1) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig_re) u)
              (Seq.index (chunks_of_re_avx2 orig_re) (u + v step_by))
              (Seq.index (chunks_of_re_avx2 re_new) u)
              (Seq.index (chunks_of_re_avx2 re_new) (u + v step_by)) zeta)))
  = let sb = v step_by in
    let jj = v j in
    lemma_chunks_frame orig_re re_old jj;
    lemma_chunks_frame orig_re re_old (jj + sb);
    introduce forall (u:nat{u<32}).
        (u >= v offset /\ u < jj + 1) ==>
        unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig_re) u)
          (Seq.index (chunks_of_re_avx2 orig_re) (u + sb))
          (Seq.index (chunks_of_re_avx2 re_new) u)
          (Seq.index (chunks_of_re_avx2 re_new) (u + sb)) zeta
    with (introduce (u >= v offset /\ u < jj + 1) ==> _
          with _. (
            if u = jj then ()
            else (
              lemma_chunks_frame re_old re_new u;
              lemma_chunks_frame re_old re_new (u + sb)
            )))
#pop-options

(* ---- round_cross : the CROSS-window single-window driver (mirror of round_ws but
   for the cross layers L7/L6 where step_by is 16/8, outside round_ws's STEP in
   {8,16,32} support).  bnd-PARAMETRIC (NTT_BASE for L7, NTT_BASE+1 for L6);
   fold_range over [offset, offset+step_by) of round_cross_sealed butterflies,
   accumulating the window's lo-unit cross atoms.  Window covers [offset, offset+2*step_by). ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"
let round_cross
      (re: t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (offset step_by: usize)
      (bnd:nat{bnd + 8380416 < pow2 31})
      (zeta: i32)
    : Prims.Pure (t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      (requires
        v step_by > 0 /\
        v offset + 2 * (v step_by) <= 32 /\
        Spec.Utils.is_i32b 4190208 zeta /\
        (Spec.Utils.forall32 (fun i ->
            (i >= v offset /\ i < v offset + 2 * (v step_by)) ==>
            is_i32b_unit_avx2 bnd (Seq.index re i))))
      (ensures
        fun re_future ->
          Spec.Utils.modifies_range_32 re re_future
            offset (offset +! (mk_usize 2 *! step_by)) /\
          (Spec.Utils.forall32 (fun i ->
             (i >= v offset /\ i < v offset + 2 * (v step_by)) ==>
             is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_future i))) /\
          (Spec.Utils.forall32 (fun u ->
             (u >= v offset /\ u < v offset + (v step_by)) ==>
             unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 re) u)
                                  (Seq.index (chunks_of_re_avx2 re) (u + v step_by))
                                  (Seq.index (chunks_of_re_avx2 re_future) u)
                                  (Seq.index (chunks_of_re_avx2 re_future) (u + v step_by))
                                  zeta))) =
  let orig_re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) =
    Core_models.Clone.f_clone #(t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32))
      #FStar.Tactics.Typeclasses.solve re
  in
  let re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) =
    Rust_primitives.Hax.Folds.fold_range offset
      (offset +! step_by <: usize)
      (fun re j ->
          let re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) = re in
          let j:usize = j in
          (Spec.Utils.modifies_range2_32 orig_re re
              offset j
              (offset +! step_by) (j +! step_by)) /\
          (Spec.Utils.forall32 (fun i ->
                  ((i >= v offset /\ i < v j) \/
                    (i >= v offset + v step_by /\ i < v j + v step_by)) ==>
                  is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re i))) /\
          (Spec.Utils.forall32 (fun u ->
                  (u >= v offset /\ u < v j) ==>
                  unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig_re) u)
                    (Seq.index (chunks_of_re_avx2 orig_re) (u + v step_by))
                    (Seq.index (chunks_of_re_avx2 re) u)
                    (Seq.index (chunks_of_re_avx2 re) (u + v step_by))
                    zeta)))
      re
      (fun re j ->
          let re:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) = re in
          let j:usize = j in
          let re_old:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) = re in
          let _:Prims.unit =
            assert (Seq.index re_old (v j) == Seq.index orig_re (v j));
            assert (Seq.index re_old (v j + v step_by) == Seq.index orig_re (v j + v step_by));
            assert (is_i32b_unit_avx2 bnd (Seq.index re_old (v j)));
            assert (is_i32b_unit_avx2 bnd (Seq.index re_old (v j + v step_by)))
          in
          let re_new:t_Array Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 (mk_usize 32) =
            round_cross_sealed re_old j step_by bnd zeta
          in
          let _:Prims.unit =
            lemma_round_cross_maintains orig_re re_old re_new offset j step_by bnd zeta
          in
          re_new)
  in
  re
#pop-options

(* ---- L6 two-window merge: L6 = two windows (offset 0 lo-units [0,8) zeta_r 2;
   offset 16 lo-units [16,24) zeta_r 3).  round_cross on mid (off 0) -> s1 gives the
   [0,8) cross atoms about (mid,s1) + modifies [0,16); round_cross on s1 (off 16) ->
   out gives the [16,24) cross atoms about (s1,out) + modifies [16,32).  Merge into the
   single L6 forall32 (mask u%16<8, partner +8, zeta_r(u/16+2)) about (mid,out):
   - w1 atoms (mid,s1) -> (mid,out): u,u+8 in [0,16), disjoint from w2's modifies [16,32).
   - w2 atoms (s1,out) -> (mid,out): u,u+8 in [16,32), disjoint from w1's modifies [0,16),
     so mid agrees with s1 there. ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"
let lemma_l6_two_windows (mid s1 out: av32)
    : Lemma
      (requires
        // w1: round_cross mid 0 8 -> s1
        Spec.Utils.modifies_range_32 mid s1 (mk_usize 0) (mk_usize 16) /\
        (Spec.Utils.forall32 (fun u ->
            (u >= 0 /\ u < 8) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 mid) u)
              (Seq.index (chunks_of_re_avx2 mid) (u+8))
              (Seq.index (chunks_of_re_avx2 s1) u)
              (Seq.index (chunks_of_re_avx2 s1) (u+8))
              (mk_i32 (zeta_r 2)))) /\
        // w2: round_cross s1 16 8 -> out
        Spec.Utils.modifies_range_32 s1 out (mk_usize 16) (mk_usize 32) /\
        (Spec.Utils.forall32 (fun u ->
            (u >= 16 /\ u < 24) ==>
            unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 s1) u)
              (Seq.index (chunks_of_re_avx2 s1) (u+8))
              (Seq.index (chunks_of_re_avx2 out) u)
              (Seq.index (chunks_of_re_avx2 out) (u+8))
              (mk_i32 (zeta_r 3)))))
      (ensures
        Spec.Utils.forall32 (fun u ->
          (u % 16 < 8) ==>
          unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 mid) u)
            (Seq.index (chunks_of_re_avx2 mid) (u+8))
            (Seq.index (chunks_of_re_avx2 out) u)
            (Seq.index (chunks_of_re_avx2 out) (u+8))
            (mk_i32 (zeta_r (u / 16 + 2))))) =
  let aux (u:nat{u<32}) : Lemma
      ((u % 16 < 8) ==>
       unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 mid) u)
         (Seq.index (chunks_of_re_avx2 mid) (u+8))
         (Seq.index (chunks_of_re_avx2 out) u)
         (Seq.index (chunks_of_re_avx2 out) (u+8))
         (mk_i32 (zeta_r (u / 16 + 2))))
    = if (u % 16 < 8) then begin
        if u < 8 then begin
          // w1 group: u,u+8 in [0,16); out agrees with s1 there (w2 modifies [16,32))
          assert (Seq.index s1 u == Seq.index out u);
          assert (Seq.index s1 (u+8) == Seq.index out (u+8));
          lemma_chunks_frame s1 out u;
          lemma_chunks_frame s1 out (u+8);
          assert (u / 16 + 2 == 2)
        end else begin
          // u in [16,24): w2 group; mid agrees with s1 there (w1 modifies [0,16))
          assert (Seq.index mid u == Seq.index s1 u);
          assert (Seq.index mid (u+8) == Seq.index s1 (u+8));
          lemma_chunks_frame mid s1 u;
          lemma_chunks_frame mid s1 (u+8);
          assert (u / 16 + 2 == 3)
        end
      end
  in
  // explicit per-index discharge (the forall32 is a 32-conjunction; ground calls
  // make each conjunct available without relying on universal-instantiation hints
  // that go stale on a cold build).
  aux 0;  aux 1;  aux 2;  aux 3;  aux 4;  aux 5;  aux 6;  aux 7;
  aux 8;  aux 9;  aux 10; aux 11; aux 12; aux 13; aux 14; aux 15;
  aux 16; aux 17; aux 18; aux 19; aux 20; aux 21; aux 22; aux 23;
  aux 24; aux 25; aux 26; aux 27; aux 28; aux 29; aux 30; aux 31;
  Classical.forall_intro aux
#pop-options

(* ---- L7 block: full layer 7 (step_by 16, one window [0,32), zeta_r 1).
   Input NTT_BASE, output NTT_BASE+1.  Wraps round_cross + the zeta/mask bridge +
   pair-to-layer, exposing layer_done + poly bound (keeps the consumer thin). ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"
let ntt_l7_block_o (orig: av32)
    : Prims.Pure av32
      (requires is_i32b_poly_avx2 8380416 orig)
      (ensures fun mid ->
        layer_done orig mid 7 /\ is_i32b_poly_avx2 (8380416 + 8380416) mid) =
  let nb : nat = 8380416 in
  let nb1 : nat = 8380416 + 8380416 in
  lemma_poly_to_units nb orig;
  let mid = round_cross orig (mk_usize 0) (mk_usize 16) nb
              (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)) in
  // poly bound FIRST (clean context: round_cross's [0,32) output bound forall feeds it).
  let _:Prims.unit = lemma_units_to_poly nb1 mid in
  // layer_done: bridge round_cross's window cross post (u in [0,16), zeta_r 1) to the
  // L7 driver's form (mask u%32<16, per-u zeta_r(u/32+1)), then pair-to-layer.
  let _:Prims.unit =
    let _:Prims.unit =
      introduce forall (u:nat{u<32}).
          (u % 32 < 16) ==>
          unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig) u)
            (Seq.index (chunks_of_re_avx2 orig) (u+16))
            (Seq.index (chunks_of_re_avx2 mid) u)
            (Seq.index (chunks_of_re_avx2 mid) (u+16))
            (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 32 + 1)))
      with (introduce (u % 32 < 16) ==> _ with _. (assert (u / 32 + 1 == 1)))
    in
    lemma_l7_pairs_to_layer orig mid
  in
  mid
#pop-options

(* ---- L6 block: full layer 6 (step_by 8, two windows offsets 0/16, zeta_r 2/3).
   Input NTT_BASE+1, output NTT_BASE+2.  Wraps round_cross x2 + lemma_l6_two_windows
   + pair-to-layer + bound framing, exposing layer_done + poly bound. ---- *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"
let ntt_l6_block_o (mid: av32)
    : Prims.Pure av32
      (requires is_i32b_poly_avx2 (8380416 + 8380416) mid)
      (ensures fun out ->
        layer_done mid out 6 /\ is_i32b_poly_avx2 (8380416 + 2*8380416) out) =
  let nb1 : nat = 8380416 + 8380416 in
  let nb2 : nat = 8380416 + 2*8380416 in
  // window 1: offset 0, zeta_r 2
  lemma_poly_to_units nb1 mid;
  let s1 = round_cross mid (mk_usize 0) (mk_usize 8) nb1
             (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 2)) in
  // window 2: offset 16, zeta_r 3.  s1's [16,32) unchanged from mid (w1 modifies [0,16)).
  let _:Prims.unit =
    introduce forall (i:nat{i<32}).
        (i >= 16 /\ i < 32) ==> is_i32b_unit_avx2 nb1 (Seq.index s1 i)
    with (introduce (i >= 16 /\ i < 32) ==> _ with _. (
            assert (Seq.index mid i == Seq.index s1 i)))
  in
  let out = round_cross s1 (mk_usize 16) (mk_usize 8) nb1
              (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 3)) in
  // poly bound (its own block): out[0,16)==s1[0,16) (w2 modifies [16,32)); out[16,32) NTT_BASE+2.
  let _:Prims.unit =
    let _:Prims.unit =
      introduce forall (i:nat{i<32}). is_i32b_unit_avx2 nb2 (Seq.index out i)
      with (if i < 16 then assert (Seq.index s1 i == Seq.index out i) else ())
    in
    lemma_units_to_poly nb2 out
  in
  // layer_done (its own block): merge the 2 windows -> L6 forall32 -> layer_done.
  let _:Prims.unit =
    lemma_l6_two_windows mid s1 out;
    lemma_l6_pairs_to_layer mid out
  in
  out
#pop-options

(* ============================================================================
   PHASE 3 — TOP COMPOSE.  The keystone: compose the whole forward NTT into the
   single functional-correctness atom ntt_done = "flat(out) == ntt(flat orig) mod q".
   Mirror of lemma_compose_5_3_o (one more composite + the two within-chunk layers),
   feeding the flat-level top compose Avx2NttCompose.lemma_ntt_compose_avx2.
   (Placed at EOF, AFTER ntt_l7_block_o/ntt_l6_block_o, to match the .fsti val order.)

   Input chain (bound factors over NTT_BASE=8380416):
     orig (NTT_BASE) --7_6--> s76 (+2) --5_3--> s53 (+5) --L2--> s2 (+6)
        --L1--> s1 (+7) --L0--> ffinal (+8).
   ============================================================================ *)

(* ---- ntt_done : OPAQUE top-level functional-post atom = "flat(b) == ntt(flat a)
   mod q".  Mirror of comp_5_3_done/comp_7_6_done but the spec is the WHOLE forward NTT
   (Hacspec_ml_dsa.Ntt.ntt), not a layer chain.  The impl `ntt` fn reveals this (via
   lemma_ntt_done_reveal) for its own ensures in the Phase-4 backport. ---- *)
[@@ "opaque_to_smt"]
let ntt_done (a b: av32) : Type0 =
  (let in_flat  = C.simd_units_to_array (chunks_of_re_avx2 a) in
   let out_flat = C.simd_units_to_array (chunks_of_re_avx2 b) in
   let spec = Hacspec_ml_dsa.Ntt.ntt in_flat in
   forall (i: nat). i < 256 ==>
     (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)

(* ---- the top compose lemma.  Takes the 7_6 composite + 5_3 composite + the three
   within-chunk layer_done atoms (L2/L1/L0), reveals each to its flat congruence, and
   chains them via Avx2NttCompose.lemma_ntt_compose_avx2 (the flat-level top compose,
   which itself uses the per-layer cong lemmas).  Produces ntt_done.  Mirror of
   lemma_compose_5_3_o's ghost-state style. ---- *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_ntt_top_compose_o (orig s76 s53 s2 s1 ffinal: av32)
    : Lemma
      (requires
        comp_7_6_done orig s76 /\ comp_5_3_done s76 s53 /\
        layer_done s53 s2 2 /\ layer_done s2 s1 1 /\ layer_done s1 ffinal 0)
      (ensures ntt_done orig ffinal) =
  // reveal the composite/layer atoms -> the five flat congruences the top compose wants.
  reveal_opaque (`%comp_7_6_done) comp_7_6_done;
  reveal_opaque (`%comp_5_3_done) comp_5_3_done;
  lemma_layer_done_reveal s53 s2 2;
  lemma_layer_done_reveal s2 s1 1;
  lemma_layer_done_reveal s1 ffinal 0;
  let f0      = C.simd_units_to_array (chunks_of_re_avx2 orig)   in
  let f76     = C.simd_units_to_array (chunks_of_re_avx2 s76)    in
  let f53     = C.simd_units_to_array (chunks_of_re_avx2 s53)    in
  let f2      = C.simd_units_to_array (chunks_of_re_avx2 s2)     in
  let f1      = C.simd_units_to_array (chunks_of_re_avx2 s1)     in
  let ffinalf = C.simd_units_to_array (chunks_of_re_avx2 ffinal) in
  // the five flat hyps required by lemma_ntt_compose_avx2:
  //   f76 ≡ ntt_layer(ntt_layer f0 7) 6        (from comp_7_6_done)
  //   f53 ≡ ntt_layer(ntt_layer(ntt_layer f76 5) 4) 3  (from comp_5_3_done)
  //   f2  ≡ ntt_layer f53 2 ; f1 ≡ ntt_layer f2 1 ; ffinalf ≡ ntt_layer f1 0
  Avx2NttCompose.lemma_ntt_compose_avx2 f0 f76 f53 f2 f1 ffinalf;
  // Avx2NttCompose post: ffinalf ≡ ntt f0.  Seal into ntt_done.
  reveal_opaque (`%ntt_done) ntt_done
#pop-options

(* ---- backport reveal: unseal ntt_done -> the raw flat congruence the impl `ntt` fn
   needs for its own ensures.  .fst-only (mentions chunks_of_re_avx2); NOT in the .fsti. ---- *)
let lemma_ntt_done_reveal (a b: av32)
    : Lemma
      (requires ntt_done a b)
      (ensures
        (let in_flat  = C.simd_units_to_array (chunks_of_re_avx2 a) in
         let out_flat = C.simd_units_to_array (chunks_of_re_avx2 b) in
         let spec = Hacspec_ml_dsa.Ntt.ntt in_flat in
         forall (i: nat). i < 256 ==>
           (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = reveal_opaque (`%ntt_done) ntt_done

(* ============================================================================
   PHASE 4 — 7_6 INTERLEAVE COMMUTE.  Prove that the IMPL's exact interleaved 32-mul
   order yields comp_7_6_done (only the GROUPED order was proven before, via
   ntt_at_layer_7_and_6_o).  USER decision (b): PRESERVE the interleaved impl, PROVE
   the commute (do NOT reorder the impl).

   IMPL ORDER (extracted from Libcrux_ml_dsa.Simd.Avx2.Ntt.ntt_at_layer_7_and_6_):
     half A:  L7[0,1,2,3] L7[8,9,10,11]  (sb16, zeta_r 1)
              L6[0,1,2,3] (sb8, zeta_r 2)  L6[16,17,18,19] (sb8, zeta_r 3)
     half B:  L7[4,5,6,7] L7[12,13,14,15] (sb16, zeta_r 1)
              L6[4,5,6,7] (sb8, zeta_r 2)  L6[20,21,22,23] (sb8, zeta_r 3)
   The two halves touch DISJOINT units:
     half A units {0-3, 8-11, 16-19, 24-27} ; half B units {4-7, 12-15, 20-23, 28-31}.
   Each L7 pair (i,i+16); each L6 pair (j,j+8).  Each unit is written by exactly one L7
   then one L6 mul; the schedule is dependency-respecting (every L6 pair reads units
   already L7-processed, never since overwritten).

   APPROACH (ii) — order-AGNOSTIC drivers.  The cross drivers lemma_l{6,7}_cross_driver_
   compose_avx2 take a forall32 of per-pair cross atoms relating (in, out) and produce the
   layer congruence REGARDLESS of how `out` was produced.  So:
     mid := ntt_l7_block_o orig  (independent grouped all-L7; gives layer_done orig mid 7
            + bound +2; and mid's units are the canonical L7 transforms).
     out := the 32 impl-order round_cross_sealed muls.
   The L7 muls' outputs equal mid on the L7-written units; each L6 mul reads mid-equal
   units (dependency-respecting) and writes them; out keeps them (disjoint halves).
   Frame every L6 mul's cross atom to be about (mid, out) -> lemma_l6_pairs_to_layer
   -> layer_done mid out 6 -> lemma_compose_7_6_o orig mid out -> comp_7_6_done. ---- *)

(* Per-pair cross-atom FRAMING: a cross atom about (a,b) on pair (ulo,uhi) survives
   re-anchoring `a` to a' that agrees with a on ulo,uhi, and `b` to b' agreeing on
   ulo,uhi.  (unit_post_cross_avx2 reads only chunks at ulo,uhi of each side.) *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_cross_atom_reanchor
      (a a' b b': av32) (ulo uhi:nat{ulo<32 /\ uhi<32})
      (zeta:i32{Spec.Utils.is_i32b 4190208 zeta})
    : Lemma
      (requires
        Seq.index a ulo == Seq.index a' ulo /\ Seq.index a uhi == Seq.index a' uhi /\
        Seq.index b ulo == Seq.index b' ulo /\ Seq.index b uhi == Seq.index b' uhi /\
        unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 a) ulo)
                             (Seq.index (chunks_of_re_avx2 a) uhi)
                             (Seq.index (chunks_of_re_avx2 b) ulo)
                             (Seq.index (chunks_of_re_avx2 b) uhi) zeta)
      (ensures
        unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 a') ulo)
                             (Seq.index (chunks_of_re_avx2 a') uhi)
                             (Seq.index (chunks_of_re_avx2 b') ulo)
                             (Seq.index (chunks_of_re_avx2 b') uhi) zeta)
  = lemma_chunks_frame a a' ulo; lemma_chunks_frame a a' uhi;
    lemma_chunks_frame b b' ulo; lemma_chunks_frame b b' uhi
#pop-options

(* one impl-order mul = round_cross_sealed wrapper bundling the per-pair
   requires-discharge into a clean clause set.  bnd-parametric. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let imul (re: av32) (index step_by:usize)
         (bnd:nat{bnd + 8380416 < pow2 31}) (zeta:i32)
    : Prims.Pure av32
      (requires
        v step_by > 0 /\ v index + v step_by < 32 /\
        Spec.Utils.is_i32b 4190208 zeta /\
        is_i32b_unit_avx2 bnd (Seq.index re (v index)) /\
        is_i32b_unit_avx2 bnd (Seq.index re (v index + v step_by)))
      (ensures fun re_f ->
        Spec.Utils.modifies2_32 re re_f index (index +! step_by) /\
        is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_f (v index)) /\
        is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_f (v index + v step_by)) /\
        unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 re) (v index))
                             (Seq.index (chunks_of_re_avx2 re) (v index + v step_by))
                             (Seq.index (chunks_of_re_avx2 re_f) (v index))
                             (Seq.index (chunks_of_re_avx2 re_f) (v index + v step_by))
                             zeta) =
  round_cross_sealed re index step_by bnd zeta
#pop-options

(* ---- pure pair-butterfly: the (index, index+step_by) GS pair transform exactly as
   round_cross_sealed's body computes it, as a function of ONLY the two input units.
   `re_in[index] = u0`, `re_in[index+sb] = u1`.  Returns (new u0, new u1). ---- *)
let bf_pair (u0 u1: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) (zeta: i32)
    : (Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256 &
       Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) =
  let rhs = Libcrux_intrinsics.Avx2.mm256_set1_epi32 zeta in
  let m   = Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply u1.f_value rhs in
  let nu0 = { u0 with Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value =
                Libcrux_intrinsics.Avx2.mm256_add_epi32 u0.f_value m } in
  let nu1 = ({ Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value =
                Libcrux_intrinsics.Avx2.mm256_sub_epi32 u0.f_value m }
             <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256) in
  (nu0, nu1)

(* round_cross_sealed agrees with bf_pair at the pair: result[index] = fst (bf_pair ...),
   result[index+sb] = snd (bf_pair ...).  Proven by replicating the body's update_at chain. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --z3refresh"
let lemma_imul_eq_bf_pair (re: av32) (index step_by:usize)
         (bnd:nat{bnd + 8380416 < pow2 31}) (zeta:i32)
    : Lemma
      (requires
        v step_by > 0 /\ v index + v step_by < 32 /\
        Spec.Utils.is_i32b 4190208 zeta /\
        is_i32b_unit_avx2 bnd (Seq.index re (v index)) /\
        is_i32b_unit_avx2 bnd (Seq.index re (v index + v step_by)))
      (ensures
        (let re_f = round_cross_sealed re index step_by bnd zeta in
         let (nu0, nu1) = bf_pair (Seq.index re (v index)) (Seq.index re (v index + v step_by)) zeta in
         Seq.index re_f (v index) == nu0 /\
         Seq.index re_f (v index + v step_by) == nu1)) =
  let i = v index in let s = v step_by in
  let u0 = Seq.index re i in
  let u1 = Seq.index re (i + s) in
  let rhs = Libcrux_intrinsics.Avx2.mm256_set1_epi32 zeta in
  let m = Libcrux_ml_dsa.Simd.Avx2.Arithmetic.montgomery_multiply u1.f_value rhs in
  // re1: re[index+sb] <- mont_mul(re[index+sb], rhs)  (= m)
  let re1 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re (index +! step_by) ({ u1 with Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value = m }) in
  assert (Seq.index re1 i == u0);
  // tmp = sub(re1[index], re1[index+sb]) = sub(u0, m)
  let tmp = Libcrux_intrinsics.Avx2.mm256_sub_epi32 u0.f_value m in
  // re2: re1[index] <- add(re1[index], re1[index+sb]) = add(u0, m)
  let re2 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re1 index ({ u0 with Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value = Libcrux_intrinsics.Avx2.mm256_add_epi32 u0.f_value m }) in
  // re3: re2[index+sb] <- tmp
  let re3 = Rust_primitives.Hax.Monomorphized_update_at.update_at_usize re2 (index +! step_by) ({ Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value = tmp }) in
  assert (Seq.index re3 i == ({ u0 with Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value = Libcrux_intrinsics.Avx2.mm256_add_epi32 u0.f_value m }));
  assert (Seq.index re3 (i + s) == ({ Libcrux_ml_dsa.Simd.Avx2.Vector_type.f_value = tmp }
             <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256))
#pop-options

(* per-unit value via bf_pair, lifted to imul: imul output at the pair determined by
   input pair units.  Wraps lemma_imul_eq_bf_pair for the value-determinism we thread. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let imul_val (re: av32) (index step_by:usize)
         (bnd:nat{bnd + 8380416 < pow2 31}) (zeta:i32)
    : Lemma
      (requires
        v step_by > 0 /\ v index + v step_by < 32 /\
        Spec.Utils.is_i32b 4190208 zeta /\
        is_i32b_unit_avx2 bnd (Seq.index re (v index)) /\
        is_i32b_unit_avx2 bnd (Seq.index re (v index + v step_by)))
      (ensures
        (let re_f = imul re index step_by bnd zeta in
         let (nu0, nu1) = bf_pair (Seq.index re (v index)) (Seq.index re (v index + v step_by)) zeta in
         Seq.index re_f (v index) == nu0 /\ Seq.index re_f (v index + v step_by) == nu1)) =
  lemma_imul_eq_bf_pair re index step_by bnd zeta
#pop-options

(* ---- per-unit value of a disjoint-pair sweep, threaded by bf_pair determinism.
   A "qpair" run of 4 disjoint pairs (lo-units q,q+1,q+2,q+3 ; partner +sb) on `re`.
   Output unit values determined by bf_pair of the (orig-equal) input pair units. ---- *)

(* run 4 disjoint pairs (base..base+3, partner +sb); each reads `re`-equal (untouched
   within this call) units, writes bf_pair.  Post = the 4 bf_pair value-equalities about
   the ORIGINAL `re` + modifies only the 8 touched units + touched-unit bounds. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"
let quad (re: av32) (base sb:usize)
         (bnd:nat{bnd + 8380416 < pow2 31}) (zeta:i32)
    : Prims.Pure av32
      (requires
        v sb > 0 /\ v base + 3 + v sb < 32 /\ v base + 4 <= v base + v sb /\
        Spec.Utils.is_i32b 4190208 zeta /\
        (forall (k:nat). (k >= v base /\ k < v base + 4) \/
                         (k >= v base + v sb /\ k < v base + 4 + v sb) ==>
                         is_i32b_unit_avx2 bnd (Seq.index re k)))
      (ensures fun re_f ->
        // modifies: only the 8 touched units {base..base+3, base+sb..base+3+sb}
        (forall (k:nat). k < 32 /\
            ~((k >= v base /\ k < v base + 4) \/ (k >= v base + v sb /\ k < v base + 4 + v sb))
            ==> Seq.index re_f k == Seq.index re k) /\
        // touched-unit bounds
        (forall (k:nat). (k >= v base /\ k < v base + 4) \/
                         (k >= v base + v sb /\ k < v base + 4 + v sb) ==>
                         is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_f k)) /\
        // per-pair bf_pair value about the ORIGINAL re
        (forall (q:nat). q >= v base /\ q < v base + 4 ==>
            (let (nu0,nu1) = bf_pair (Seq.index re q) (Seq.index re (q + v sb)) zeta in
             Seq.index re_f q == nu0 /\ Seq.index re_f (q + v sb) == nu1))) =
  let b = v base in let s = v sb in
  let r0 = imul re   base               sb bnd zeta in
  imul_val re   base               sb bnd zeta;
  let r1 = imul r0   (base +! mk_usize 1) sb bnd zeta in
  imul_val r0   (base +! mk_usize 1) sb bnd zeta;
  let r2 = imul r1   (base +! mk_usize 2) sb bnd zeta in
  imul_val r1   (base +! mk_usize 2) sb bnd zeta;
  let r3 = imul r2   (base +! mk_usize 3) sb bnd zeta in
  imul_val r2   (base +! mk_usize 3) sb bnd zeta;
  r3
#pop-options

(* CROSS-ATOM FRAMING for two arrays a,b that agree on a pair (ulo,uhi) with a known
   bf_pair value relation: if b[ulo]==fst(bf_pair a[ulo] a[uhi] z), b[uhi]==snd(...),
   and a[ulo],a[uhi] bounded, then the cross atom holds about (a,b) on (ulo,uhi).
   Built by running imul on `a` at (ulo,uhi): imul's cross atom is about (a, imul a ...),
   and imul a's pair == bf_pair a-pair == b's pair, so re-anchor to b. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --split_queries always --z3refresh"
let lemma_pair_cross_from_bf
      (a b: av32) (ulo uhi:usize)
      (bnd:nat{bnd + 8380416 < pow2 31}) (zeta:i32{Spec.Utils.is_i32b 4190208 zeta})
    : Lemma
      (requires
        v ulo < v uhi /\ v uhi < 32 /\
        is_i32b_unit_avx2 bnd (Seq.index a (v ulo)) /\
        is_i32b_unit_avx2 bnd (Seq.index a (v uhi)) /\
        (let (nu0,nu1) = bf_pair (Seq.index a (v ulo)) (Seq.index a (v uhi)) zeta in
         Seq.index b (v ulo) == nu0 /\ Seq.index b (v uhi) == nu1))
      (ensures
        is_i32b_unit_avx2 (bnd + 8380416) (Seq.index b (v ulo)) /\
        is_i32b_unit_avx2 (bnd + 8380416) (Seq.index b (v uhi)) /\
        unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 a) (v ulo))
                             (Seq.index (chunks_of_re_avx2 a) (v uhi))
                             (Seq.index (chunks_of_re_avx2 b) (v ulo))
                             (Seq.index (chunks_of_re_avx2 b) (v uhi)) zeta) =
  let sb = uhi -! ulo in
  // imul a on pair (ulo, ulo+sb=uhi): cross atom about (a, re_f) + bounds; value == bf_pair.
  let re_f = imul a ulo sb bnd zeta in
  imul_val a ulo sb bnd zeta;
  // re_f[ulo]==nu0==b[ulo], re_f[uhi]==nu1==b[uhi]  -> frame chunks re_f -> chunks b.
  lemma_chunks_frame re_f b (v ulo);
  lemma_chunks_frame re_f b (v uhi)
#pop-options

(* mid = grouped all-L7 over orig, built as 4 disjoint quads (lo-units 0-3,4-7,8-11,12-15;
   partner +16; zeta_r 1).  Post: bound +1 poly, and EVERY lo-unit u<16 carries
   mid[u]==fst(bf_pair orig[u] orig[u+16] z1), mid[u+16]==snd(...).  Disjoint quads -> each
   reads orig (framed). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 600 --split_queries always --z3refresh"
let build_mid_L7 (orig: av32)
    : Prims.Pure av32
      (requires is_i32b_poly_avx2 8380416 orig)
      (ensures fun mid ->
        is_i32b_poly_avx2 (8380416 + 8380416) mid /\
        (forall (u:nat). u < 16 ==>
           (let (nu0,nu1) = bf_pair (Seq.index orig u) (Seq.index orig (u+16))
                                    (mk_i32 (zeta_r 1)) in
            Seq.index mid u == nu0 /\ Seq.index mid (u+16) == nu1))) =
  let nb : nat = 8380416 in let nb1 : nat = 8380416 + 8380416 in
  let z1 : i32 = mk_i32 (zeta_r 1) in
  assert_norm (zeta_r 1 == 25847);
  assert (Spec.Utils.is_i32b 4190208 z1);
  lemma_poly_to_units nb orig;
  let m0 = quad orig (mk_usize 0)  (mk_usize 16) nb z1 in   // lo 0-3
  let m1 = quad m0   (mk_usize 4)  (mk_usize 16) nb z1 in   // lo 4-7
  let m2 = quad m1   (mk_usize 8)  (mk_usize 16) nb z1 in   // lo 8-11
  let mid = quad m2  (mk_usize 12) (mk_usize 16) nb z1 in   // lo 12-15
  // each quad reads orig-equal units (disjoint from prior quads' touched sets):
  //   m0 touches {0-3,16-19}; m1 needs {4-7,20-23} (untouched in m0); etc.
  // bf_pair values about orig follow since each quad's input == orig on its pair units.
  lemma_units_to_poly nb1 mid;
  mid
#pop-options

(* L7 cross atoms about (orig,mid) from: orig units bounded nb, and per-lo-unit u<16
   mid[u]/mid[u+16] == bf_pair orig[u] orig[u+16] z1.  Standalone (bounded WP). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 600 --split_queries always --z3refresh"
let lemma_L7_atoms (orig mid: av32)
    : Lemma
      (requires
        (forall (k:nat). k < 32 ==> is_i32b_unit_avx2 8380416 (Seq.index orig k)) /\
        (forall (u:nat). u < 16 ==>
           (let (nu0,nu1) = bf_pair (Seq.index orig u) (Seq.index orig (u+16))
                                    (mk_i32 (zeta_r 1)) in
            Seq.index mid u == nu0 /\ Seq.index mid (u+16) == nu1)))
      (ensures layer_done orig mid 7) =
  let z1 = mk_i32 (zeta_r 1) in
  assert_norm (zeta_r 1 == 25847);
  assert (Spec.Utils.is_i32b 4190208 z1);
  // ground per-unit dispatch (avoid introduce-forall over the heavy lemma_pair_cross_from_bf;
  // mirror lemma_l6_two_windows: each aux call is a clean tiny VC).
  let aux (u:nat{u<32}) : Lemma
      ((u % 32 < 16) ==>
       unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 orig) u)
         (Seq.index (chunks_of_re_avx2 orig) (u+16))
         (Seq.index (chunks_of_re_avx2 mid) u)
         (Seq.index (chunks_of_re_avx2 mid) (u+16))
         (mk_i32 (zeta_r (u / 32 + 1))))
    = if (u % 32 < 16) then begin
        assert (u / 32 + 1 == 1);
        lemma_pair_cross_from_bf orig mid (mk_usize u) (mk_usize (u+16)) 8380416 z1
      end
  in
  aux 0;  aux 1;  aux 2;  aux 3;  aux 4;  aux 5;  aux 6;  aux 7;
  aux 8;  aux 9;  aux 10; aux 11; aux 12; aux 13; aux 14; aux 15;
  aux 16; aux 17; aux 18; aux 19; aux 20; aux 21; aux 22; aux 23;
  aux 24; aux 25; aux 26; aux 27; aux 28; aux 29; aux 30; aux 31;
  Classical.forall_intro aux;
  lemma_l7_pairs_to_layer orig mid
#pop-options

(* L6 cross atoms about (mid,out) from: mid units bounded nb1, and per-lo-unit u (u%16<8)
   out[u]/out[u+8] == bf_pair mid[u] mid[u+8] (z2 if u<16 else z3).  Standalone. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 600 --split_queries always --z3refresh"
let lemma_L6_atoms (mid out: av32)
    : Lemma
      (requires
        (forall (k:nat). k < 32 ==> is_i32b_unit_avx2 (8380416 + 8380416) (Seq.index mid k)) /\
        (forall (u:nat). (u % 16 < 8) /\ u < 32 ==>
           (let zL6 = (if u < 16 then mk_i32 (zeta_r 2) else mk_i32 (zeta_r 3)) in
            let (nu0,nu1) = bf_pair (Seq.index mid u) (Seq.index mid (u+8)) zL6 in
            Seq.index out u == nu0 /\ Seq.index out (u+8) == nu1)))
      (ensures layer_done mid out 6) =
  let z2 = mk_i32 (zeta_r 2) in let z3 = mk_i32 (zeta_r 3) in
  assert_norm (zeta_r 2 == (-2608894));
  assert_norm (zeta_r 3 == (-518909));
  assert (Spec.Utils.is_i32b 4190208 z2);
  assert (Spec.Utils.is_i32b 4190208 z3);
  let aux (u:nat{u<32}) : Lemma
      ((u % 16 < 8) ==>
       unit_post_cross_avx2 (Seq.index (chunks_of_re_avx2 mid) u)
         (Seq.index (chunks_of_re_avx2 mid) (u+8))
         (Seq.index (chunks_of_re_avx2 out) u)
         (Seq.index (chunks_of_re_avx2 out) (u+8))
         (mk_i32 (zeta_r (u / 16 + 2))))
    = if (u % 16 < 8) then begin
        let zL6 = if u < 16 then z2 else z3 in
        assert (mk_i32 (zeta_r (u / 16 + 2)) == zL6);
        lemma_pair_cross_from_bf mid out (mk_usize u) (mk_usize (u+8)) (8380416 + 8380416) zL6
      end
  in
  aux 0;  aux 1;  aux 2;  aux 3;  aux 4;  aux 5;  aux 6;  aux 7;
  aux 8;  aux 9;  aux 10; aux 11; aux 12; aux 13; aux 14; aux 15;
  aux 16; aux 17; aux 18; aux 19; aux 20; aux 21; aux 22; aux 23;
  aux 24; aux 25; aux 26; aux 27; aux 28; aux 29; aux 30; aux 31;
  Classical.forall_intro aux;
  lemma_l6_pairs_to_layer mid out
#pop-options

(* impl-order out = 8 quads in EXACT impl order.  `mid` (= grouped all-L7) is passed with
   its bf_pair-of-orig per-pair post.  Proves: bound nb2 + per-L6-lo-unit out value ==
   bf_pair of mid (the L6-input==mid bridge: impl L7 outputs == bf_pair orig == mid; disjoint
   halves frame).

   The 8-quad BODY (impl-faithful order) + bound is proven; the L6-value ensures (L6-input==mid
   bridge) currently SATURATES as one monolithic post-forall (rlimit 800 canceled — Z3 cannot
   auto-chain the bf_pair-determinism through the 8-quad modifies frames; needs a per-unit
   standalone bridge lemma — see fstar_note cliff).  Exposed as assume val so the COMMUTE main
   fn proves comp_7_6_done on top of it.  The body below documents the exact impl order. *)
(* per-unit substitution: a quad's bf_pair value-post is about its INPUT `inp`; if `inp`
   agrees with `mid` on the read pair (q, q+8), the value-post is equivalently about `mid`.
   Pure substitution (bf_pair is a function of its two unit args), no search. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_l6_pair_subst_mid (inp mid out_q: av32) (q:nat{q+8 < 32}) (zL6:i32)
    : Lemma
      (requires
        Seq.index inp q == Seq.index mid q /\
        Seq.index inp (q+8) == Seq.index mid (q+8) /\
        (let (nu0,nu1) = bf_pair (Seq.index inp q) (Seq.index inp (q+8)) zL6 in
         Seq.index out_q q == nu0 /\ Seq.index out_q (q+8) == nu1))
      (ensures
        (let (nu0,nu1) = bf_pair (Seq.index mid q) (Seq.index mid (q+8)) zL6 in
         Seq.index out_q q == nu0 /\ Seq.index out_q (q+8) == nu1)) = ()
#pop-options

(* ---- per-L6-quad cross-array framing lemmas (CLEAN CONTEXT, standalone).
   Each takes only the two concrete arrays involved + the bounds it needs, and discharges
   exactly one Seq.index agreement via a single quad modifies/value post.  Kept tiny so the
   driver `lemma_build_out_l6_value` aux branches are just a handful of clean lemma calls
   (mirrors lemma_L6_atoms calling lemma_pair_cross_from_bf — no inline framing in a heavy VC). *)

(* an L7-quad output array `a` (= quad src base 16 z1) carries, at its lo-unit q in [base,base+4),
   the value fst(bf_pair src[q] src[q+16] z1); if additionally src agrees with `orig` at q,q+16,
   and mid[q]==fst(bf_pair orig[q] orig[q+16] z1) (the requires), then a[q]==mid[q].  Pure
   determinism of bf_pair + the value-post of one quad. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always --z3refresh"
let lemma_l7out_eq_mid (orig mid src a: av32) (q:nat)
    : Lemma
      (requires
        q < 16 /\
        Seq.index src q     == Seq.index orig q /\
        Seq.index src (q+16) == Seq.index orig (q+16) /\
        (let (nu0,nu1) = bf_pair (Seq.index src q) (Seq.index src (q+16)) (mk_i32 (zeta_r 1)) in
         Seq.index a q == nu0 /\ Seq.index a (q+16) == nu1) /\
        (let (nu0,nu1) = bf_pair (Seq.index orig q) (Seq.index orig (q+16)) (mk_i32 (zeta_r 1)) in
         Seq.index mid q == nu0 /\ Seq.index mid (q+16) == nu1))
      (ensures Seq.index a q == Seq.index mid q /\ Seq.index a (q+16) == Seq.index mid (q+16)) = ()
#pop-options

(* abstract restatement of `quad`'s ensures (the 3 foralls: modifies-frame outside the 8
   touched units, touched-unit bound, per-pair bf_pair value about the INPUT array).  Used as
   the precondition shape of the standalone L6-value driver lemma so it reasons in a CLEAN
   context (abstract av32 params, no concrete t_Array refinement pollution). *)
unfold let quadpost (re re_f: av32) (base:nat) (sb:nat{sb > 0 /\ base + 4 + sb <= 32}) (bnd:nat) (zeta:i32) : Type0 =
  (forall (k:nat). k < 32 /\
      ~((k >= base /\ k < base + 4) \/ (k >= base + sb /\ k < base + 4 + sb))
      ==> Seq.index re_f k == Seq.index re k) /\
  (forall (k:nat). (k >= base /\ k < base + 4) \/ (k >= base + sb /\ k < base + 4 + sb) ==>
      is_i32b_unit_avx2 (bnd + 8380416) (Seq.index re_f k)) /\
  (forall (q:nat). q >= base /\ q < base + 4 ==>
      (let (nu0,nu1) = bf_pair (Seq.index re q) (Seq.index re (q + sb)) zeta in
       Seq.index re_f q == nu0 /\ Seq.index re_f (q + sb) == nu1))

(* STANDALONE L6-value driver for build_out_impl, proven in a CLEAN context.  Preconditions =
   the 8 quad posts (as `quadpost` abstract foralls) + the L7 (`mid`) bf_pair-of-orig relation.
   The per-unit ground aux 0..31 (copied verbatim from build_out_impl) is the saturating part;
   isolating it here (no concrete t_Array refinement) drops the cost dramatically. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --z3refresh"
let lemma_build_out_l6_value
      (orig mid qa0 qa1 qb0 qb1 qc0 qc1 qd0 out: av32)
    : Lemma
      (requires
        (let z1 = mk_i32 (zeta_r 1) in let z2 = mk_i32 (zeta_r 2) in let z3 = mk_i32 (zeta_r 3) in
         (forall (u:nat). u < 16 ==>
            (let (nu0,nu1) = bf_pair (Seq.index orig u) (Seq.index orig (u+16)) z1 in
             Seq.index mid u == nu0 /\ Seq.index mid (u+16) == nu1)) /\
         quadpost orig qa0 0  16 8380416            z1 /\
         quadpost qa0  qa1 8  16 8380416            z1 /\
         quadpost qa1  qb0 0  8  (8380416+8380416)  z2 /\
         quadpost qb0  qb1 16 8  (8380416+8380416)  z3 /\
         quadpost qb1  qc0 4  16 8380416            z1 /\
         quadpost qc0  qc1 12 16 8380416            z1 /\
         quadpost qc1  qd0 4  8  (8380416+8380416)  z2 /\
         quadpost qd0  out 20 8  (8380416+8380416)  z3))
      (ensures
        (forall (u:nat). (u % 16 < 8) /\ u < 32 ==>
           (let zL6 = (if u < 16 then mk_i32 (zeta_r 2) else mk_i32 (zeta_r 3)) in
            let (nu0,nu1) = bf_pair (Seq.index mid u) (Seq.index mid (u+8)) zL6 in
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
            let (nu0,nu1) = bf_pair (Seq.index mid u) (Seq.index mid (u+8)) zL6 in
            Seq.index out u == nu0 /\ Seq.index out (u+8) == nu1))
      = if (u % 16 < 8) then begin
          if u < 4 then begin
            // qb0 = quad qa1 0 8 z2 reads qa1[u],qa1[u+8].
            lemma_l7out_eq_mid orig mid orig qa0 u;       // qa0[u]==mid[u], qa0[u+16]==mid[u+16]
            lemma_l7out_eq_mid orig mid qa0  qa1 (u+8);   // qa1[u+8]==mid[u+8] (uses qa0==orig on u+8,u+24)
            assert (Seq.index qa1 u == Seq.index mid u);  // qa1[u]==qa0[u]==mid[u] (qa1 base8 frame)
            lemma_l6_pair_subst_mid qa1 mid qb0 u z2;
            assert (Seq.index out u == Seq.index qb0 u);  // survival: qb1/qc0/qc1/qd0/out disjoint from {0-3}
            assert (Seq.index out (u+8) == Seq.index qb0 (u+8))   //          ... and from {8-11}
          end
          else if u < 8 then begin
            // qd0 = quad qc1 4 8 z2 reads qc1[u],qc1[u+8].
            lemma_l7out_eq_mid orig mid qb1 qc0 u;        // qc0[u]==mid[u]   (qc0 L7 base4; qb1==orig on u,u+16)
            lemma_l7out_eq_mid orig mid qc0 qc1 (u+8);    // qc1[u+8]==mid[u+8] (qc1 L7 base12; qc0==orig on u+8,u+24)
            assert (Seq.index qc1 u == Seq.index mid u);  // qc1[u]==qc0[u]==mid[u] (qc1 base12 frame)
            lemma_l6_pair_subst_mid qc1 mid qd0 u z2;
            assert (Seq.index out u == Seq.index qd0 u);  // survival: out quad (base20) disjoint from {4-7,12-15}
            assert (Seq.index out (u+8) == Seq.index qd0 (u+8))
          end
          else if u < 20 then begin
            // qb1 = quad qb0 16 8 z3 reads qb0[u],qb0[u+8]   (u in 16..19, u+8 in 24..27)
            lemma_l7out_eq_mid orig mid orig qa0 (u-16);  // qa0[u]==mid[u] (hi half of L7 base0 pair u-16)
            lemma_l7out_eq_mid orig mid qa0  qa1 (u-8);   // qa1[u+8]==mid[u+8] (hi half of L7 base8 pair u-8)
            assert (Seq.index qb0 u == Seq.index mid u);  // qb0[u]==qa1[u]==mid[u] (qb0 base0 frame)
            assert (Seq.index qb0 (u+8) == Seq.index mid (u+8)); // qb0[u+8]==qa1[u+8]==mid[u+8] (qb0 base0 frame)
            lemma_l6_pair_subst_mid qb0 mid qb1 u z3;
            assert (Seq.index out u == Seq.index qb1 u);  // survival: qc0/qc1/qd0/out disjoint from {16-19,24-27}
            assert (Seq.index out (u+8) == Seq.index qb1 (u+8))
          end
          else begin
            // out = quad qd0 20 8 z3 reads qd0[u],qd0[u+8]   (u in 20..23, u+8 in 28..31)
            lemma_l7out_eq_mid orig mid qb1 qc0 (u-16);   // qc0[u]==mid[u] (hi half of L7 base4 pair u-16)
            lemma_l7out_eq_mid orig mid qc0 qc1 (u-8);    // qc1[u+8]==mid[u+8] (hi half of L7 base12 pair u-8)
            assert (Seq.index qd0 u == Seq.index mid u);  // qd0[u]==qc1[u]==mid[u] (qd0 base4 frame: {4-7,12-15} disjoint from {20-23})
            assert (Seq.index qd0 (u+8) == Seq.index mid (u+8));
            lemma_l6_pair_subst_mid qd0 mid out u z3      // out written directly here
          end
        end
    in
    Classical.forall_intro aux
#pop-options

(* build_out_impl: the impl-faithful 8-quad body proving BOTH the bound (build_out_impl_witness
   recipe: lemma_units_to_poly over the 8 quads) AND the L6-value ensures.  The L6-value forall
   is discharged by the standalone clean-context driver lemma_build_out_l6_value (whose
   quadpost preconditions match the 8 quad posts in scope by hypothesis), keeping this fn's own
   VC cheap (just the bound chain + the lemma call). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 400 --z3refresh"
let build_out_impl (orig mid: av32)
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
            Seq.index out u == nu0 /\ Seq.index out (u+8) == nu1))) =
  let nb  : nat = 8380416 in
  let nb1 : nat = 8380416 + 8380416 in
  let nb2 : nat = 8380416 + 2*8380416 in
  let z1 : i32 = mk_i32 (zeta_r 1) in
  let z2 : i32 = mk_i32 (zeta_r 2) in
  let z3 : i32 = mk_i32 (zeta_r 3) in
  assert_norm (zeta_r 1 == 25847);
  assert_norm (zeta_r 2 == (-2608894));
  assert_norm (zeta_r 3 == (-518909));
  assert (Spec.Utils.is_i32b 4190208 z1);
  assert (Spec.Utils.is_i32b 4190208 z2);
  assert (Spec.Utils.is_i32b 4190208 z3);
  lemma_poly_to_units nb orig;
  let qa0 = quad orig (mk_usize 0)  (mk_usize 16) nb  z1 in
  let qa1 = quad qa0  (mk_usize 8)  (mk_usize 16) nb  z1 in
  let qb0 = quad qa1  (mk_usize 0)  (mk_usize 8)  nb1 z2 in
  let qb1 = quad qb0  (mk_usize 16) (mk_usize 8)  nb1 z3 in
  let qc0 = quad qb1  (mk_usize 4)  (mk_usize 16) nb  z1 in
  let qc1 = quad qc0  (mk_usize 12) (mk_usize 16) nb  z1 in
  let qd0 = quad qc1  (mk_usize 4)  (mk_usize 8)  nb1 z2 in
  let out = quad qd0  (mk_usize 20) (mk_usize 8)  nb1 z3 in
  lemma_units_to_poly nb2 out;
  // L6-value ensures: discharged by the standalone clean-context driver.  The 8 `quad` calls'
  // ensures are in scope here (each matches a `quadpost ...` precondition by hypothesis), and
  // mid's bf_pair-of-orig relation is build_out_impl's own precondition.
  lemma_build_out_l6_value orig mid qa0 qa1 qb0 qb1 qc0 qc1 qd0 out;
  out
#pop-options

(* the impl-faithful 8-quad body for build_out_impl (the bound part verifies; the L6-value
   ensures is the saturating bridge).  Kept as a private witness fn proving only the bound. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 800 --split_queries always --z3refresh"
let build_out_impl_witness (orig mid: av32)
    : Prims.Pure av32
      (requires
        is_i32b_poly_avx2 8380416 orig /\
        (forall (k:nat). k < 32 ==> is_i32b_unit_avx2 (8380416 + 8380416) (Seq.index mid k)))
      (ensures fun out -> is_i32b_poly_avx2 (8380416 + 2*8380416) out) =
  let nb  : nat = 8380416 in
  let nb1 : nat = 8380416 + 8380416 in
  let nb2 : nat = 8380416 + 2*8380416 in
  let z1 : i32 = mk_i32 (zeta_r 1) in
  let z2 : i32 = mk_i32 (zeta_r 2) in
  let z3 : i32 = mk_i32 (zeta_r 3) in
  assert_norm (zeta_r 1 == 25847);
  assert_norm (zeta_r 2 == (-2608894));
  assert_norm (zeta_r 3 == (-518909));
  assert (Spec.Utils.is_i32b 4190208 z1);
  assert (Spec.Utils.is_i32b 4190208 z2);
  assert (Spec.Utils.is_i32b 4190208 z3);
  lemma_poly_to_units nb orig;
  let qa0 = quad orig (mk_usize 0)  (mk_usize 16) nb  z1 in
  let qa1 = quad qa0  (mk_usize 8)  (mk_usize 16) nb  z1 in
  let qb0 = quad qa1  (mk_usize 0)  (mk_usize 8)  nb1 z2 in
  let qb1 = quad qb0  (mk_usize 16) (mk_usize 8)  nb1 z3 in
  let qc0 = quad qb1  (mk_usize 4)  (mk_usize 16) nb  z1 in
  let qc1 = quad qc0  (mk_usize 12) (mk_usize 16) nb  z1 in
  let qd0 = quad qc1  (mk_usize 4)  (mk_usize 8)  nb1 z2 in
  let out = quad qd0  (mk_usize 20) (mk_usize 8)  nb1 z3 in
  lemma_units_to_poly nb2 out;
  out
#pop-options

(* ---- THE COMMUTE (main).  The impl's EXACT interleaved 32-mul order (8 quads in
   build_out_impl) yields comp_7_6_done.  build_out_impl now carries BOTH the bound and the
   L6-value (bf_pair-of-mid) post, so the driver lemmas chain admit-free.
   NB: NO --split_queries always — one split sub-query (the final comp_7_6_done/bound ensures)
   saturated at 400 because the big L6/L7 value-foralls from build_mid_L7/build_out_impl clutter
   each capped sub-query; monolithic lets Z3 discharge the opaque comp_7_6_done atom directly. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 600 --z3refresh"
let ntt_at_layer_7_and_6_interleaved_o (orig: av32)
    : Prims.Pure av32
      (requires is_i32b_poly_avx2 8380416 orig)
      (ensures fun out ->
        comp_7_6_done orig out /\ is_i32b_poly_avx2 (8380416 + 2*8380416) out) =
  let mid = build_mid_L7 orig in              // grouped all-L7, bf_pair-of-orig per-pair post
  lemma_poly_to_units (8380416 + 8380416) mid;
  lemma_poly_to_units 8380416 orig;
  let out = build_out_impl orig mid in        // 8 IMPL-ORDER quads; bound + L6-value (bf_pair-of-mid)
  lemma_L7_atoms orig mid;                    // -> layer_done orig mid 7
  lemma_L6_atoms mid out;                     // -> layer_done mid out 6
  lemma_compose_7_6_o orig mid out;           // -> comp_7_6_done orig out
  out
#pop-options

(* ---- in-body round seal helpers: forall32-shaped sub-facts -> the sealed
   opaque atoms (win_bounded / win_cross), so the in-body 5_to_3 round can seal
   its completed transparent loop invariant into round_post_avx2 (via lemma_rp_intro)
   without revealing the opaque atoms in the consumer.  Trivial reveal-wrappers,
   one VC each. ---- *)
let lemma_win_bounded_from_forall32 (re: av32) (lo width bnd:nat)
    : Lemma
      (requires
        Spec.Utils.forall32 (fun i -> (i >= lo /\ i < lo + width) ==>
          is_i32b_unit_avx2 bnd (Seq.index re i)))
      (ensures win_bounded re lo width bnd)
  = reveal_opaque (`%win_bounded) win_bounded

let lemma_win_cross_from_forall32
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
  = reveal_opaque (`%win_cross) win_cross

(* ADDITIVE bf_pair value exposure (non-weakening).  bf_pair is a transparent `let` here,
   so the ensures holds by definitional unfolding; the lemma simply re-exports the value
   across the abstract interface to the in-body 7_6 consumer. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 50"
let lemma_bf_pair_def
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
          <: Libcrux_ml_dsa.Simd.Avx2.Vector_type.t_Vec256))) =
  ()
#pop-options


module Libcrux_ml_dsa.Simd.Portable.Invntt_theory
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_ml_dsa.Simd.Portable.Vector_type in
  ()

(* ---------------------------------------------------------------------------
   Relocated from `src/simd/portable/invntt.rs` `fstar::before` theory blocks
   (annotation-compaction campaign). Consumers (`Invntt.fst`) `open` this module.
     - `simd_layer_factor` (bound helper);
     - the inverse cross-chunk GS-FE theory (`layer_bound_factor`,
       `unit_fe_post_inv_cross`, the cross bridges + `lemma_inv_l{3..7}` drivers);
     - the (16382) scaling stack (`lemma_scale_flat` .. `lemma_scale_driver`);
     - the intt-congruence + to_mont finalize stack (`lemma_modq_eq` ..
       `lemma_invert_top`, incl. `to_mont` referenced cross-module by avx2/portable).
   The layer-0/1/2 per-chunk atoms stay inline in `Invntt.fst` next to their impls.
   --------------------------------------------------------------------------- *)

let simd_layer_factor (step:usize) =
    match step with
    | MkInt 1 -> 1
    | MkInt 2 -> 2
    | MkInt 4 -> 4
    | _ -> 5

let layer_bound_factor (step_by:usize) : n:nat{n <= 128} =
    match step_by with
    | MkInt 1 -> 8
    | MkInt 2 -> 16
    | MkInt 4 -> 32
    | MkInt 8 -> 64
    | MkInt 16 -> 128
    | _ -> 128

(* ---- INVERSE cross-chunk GS-FE atom + bridge lemmas.
   GS butterfly: co_lo = ci_lo + ci_hi (plain add); co_hi is the mont result of
   (ci_hi - ci_lo), a direct mod-q relation (NO separate `t` witness). *)
[@@ "opaque_to_smt"]
let unit_fe_post_inv_cross (ci_lo ci_hi co_lo co_hi : t_Array i32 (sz 8))
                           (zeta: i32{Spec.Utils.is_i32b 4190208 zeta}) : Type0 =
  (v (Seq.index co_lo 0) == v (Seq.index ci_lo 0) + v (Seq.index ci_hi 0) /\
   (v (Seq.index co_hi 0)) % 8380417 == ((v (Seq.index ci_hi 0) - v (Seq.index ci_lo 0)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 1) == v (Seq.index ci_lo 1) + v (Seq.index ci_hi 1) /\
   (v (Seq.index co_hi 1)) % 8380417 == ((v (Seq.index ci_hi 1) - v (Seq.index ci_lo 1)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 2) == v (Seq.index ci_lo 2) + v (Seq.index ci_hi 2) /\
   (v (Seq.index co_hi 2)) % 8380417 == ((v (Seq.index ci_hi 2) - v (Seq.index ci_lo 2)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 3) == v (Seq.index ci_lo 3) + v (Seq.index ci_hi 3) /\
   (v (Seq.index co_hi 3)) % 8380417 == ((v (Seq.index ci_hi 3) - v (Seq.index ci_lo 3)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 4) == v (Seq.index ci_lo 4) + v (Seq.index ci_hi 4) /\
   (v (Seq.index co_hi 4)) % 8380417 == ((v (Seq.index ci_hi 4) - v (Seq.index ci_lo 4)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 5) == v (Seq.index ci_lo 5) + v (Seq.index ci_hi 5) /\
   (v (Seq.index co_hi 5)) % 8380417 == ((v (Seq.index ci_hi 5) - v (Seq.index ci_lo 5)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 6) == v (Seq.index ci_lo 6) + v (Seq.index ci_hi 6) /\
   (v (Seq.index co_hi 6)) % 8380417 == ((v (Seq.index ci_hi 6) - v (Seq.index ci_lo 6)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co_lo 7) == v (Seq.index ci_lo 7) + v (Seq.index ci_hi 7) /\
   (v (Seq.index co_hi 7)) % 8380417 == ((v (Seq.index ci_hi 7) - v (Seq.index ci_lo 7)) * v zeta * 8265825) % 8380417)

(* Round-body discharge: bridge the leaf posts into the ground inverse cross atom.
   Impl outer_3_plus loop: add(re[j], rejs) => add_post ci_lo ci_hi co_lo (co_lo=lo+hi);
   subtract(re[j+STEP_BY], rej) => sub_post ci_hi ci_lo tmp (tmp=hi-lo, NOTE order b-a);
   montgomery_multiply_by_constant(re[j+STEP_BY], zeta) => co_hi = mont_mul(tmp, zeta). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always --using_facts_from '* -Hacspec_ml_dsa'"
let lemma_round_inv_cross_intro
    (ci_lo ci_hi co_lo co_hi tmp : t_Array i32 (sz 8))
    (zeta : i32{Spec.Utils.is_i32b 4190208 zeta})
  : Lemma
      (requires
        Libcrux_ml_dsa.Simd.Traits.Specs.add_post ci_lo ci_hi co_lo /\
        Libcrux_ml_dsa.Simd.Traits.Specs.sub_post ci_hi ci_lo tmp /\
        (forall (i:nat). i < 8 ==>
          Seq.index co_hi i == Spec.MLDSA.Math.mont_mul (Seq.index tmp i) zeta) /\
        (forall (i:nat). i < 8 ==>
          Spec.MLDSA.Math.mod_q (v (Seq.index co_hi i)) ==
          Spec.MLDSA.Math.mod_q (v (Seq.index tmp i) * v zeta * 8265825)))
      (ensures unit_fe_post_inv_cross ci_lo ci_hi co_lo co_hi zeta)
  = reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.add_post) (Libcrux_ml_dsa.Simd.Traits.Specs.add_post);
    reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.sub_post) (Libcrux_ml_dsa.Simd.Traits.Specs.sub_post);
    reveal_opaque (`%unit_fe_post_inv_cross) unit_fe_post_inv_cross;
    reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
    let lane (l:nat{l<8}) : Lemma
        (v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
         v (Seq.index tmp l) == v (Seq.index ci_hi l) - v (Seq.index ci_lo l) /\
         (v (Seq.index co_hi l)) % 8380417 ==
           ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v zeta * 8265825) % 8380417) =
      assert (v (mk_usize l) == l);
      assert (v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l));
      assert (v (Seq.index tmp l) == v (Seq.index ci_hi l) - v (Seq.index ci_lo l))
    in
    lane 0; lane 1; lane 2; lane 3; lane 4; lane 5; lane 6; lane 7
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always --using_facts_from '* -Hacspec_ml_dsa'"
let lemma_atom_to_bf_inv_cross (ci_lo ci_hi co_lo co_hi : t_Array i32 (sz 8))
                               (zeta: i32{Spec.Utils.is_i32b 4190208 zeta})
    : Lemma (requires unit_fe_post_inv_cross ci_lo ci_hi co_lo co_hi zeta)
            (ensures
              (forall (l: nat{l < 8}).
                 v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
                 (v (Seq.index co_hi l)) % 8380417 ==
                   ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v zeta * 8265825) % 8380417))
  = reveal_opaque (`%unit_fe_post_inv_cross) unit_fe_post_inv_cross;
    introduce forall (l: nat{l < 8}).
        (v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
         (v (Seq.index co_hi l)) % 8380417 ==
           ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v zeta * 8265825) % 8380417)
    with (match l with | 0 -> () | 1 -> () | 2 -> () | 3 -> () | 4 -> () | 5 -> () | 6 -> () | _ -> ())
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_inv_l3_cross_driver_compose
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 2 == 0) ==>
            unit_fe_post_inv_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+1)).f_values
                                   (Seq.index re u).f_values (Seq.index re (u+1)).f_values
                                   (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (31 - u / 2)))))
        (ensures
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 3) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re orig_re in
    let fut = Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r (31 - u / 2)) in
    Libcrux_ml_dsa.Simd.Portable.Ntt_theory.forall32_elim_1d (fun u -> (u % 2 == 0) ==>
        unit_fe_post_inv_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+1)).f_values
                               (Seq.index re u).f_values (Seq.index re (u+1)).f_values (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 2 == 0) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+1) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+1) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
          (v (Seq.index co_hi l)) % 8380417 ==
            ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm u) * 8265825) % 8380417))
      = if (u % 2 = 0) then begin
          Hacspec_ml_dsa.Commute.Chunk.lemma_cross_idx 1 u 0;
          FStar.Math.Lemmas.small_mod (u + 1) 32;
          assert (v (mk_usize u) == u);
          assert (v (mk_usize (u+1)) == u+1);
          assert (Seq.index orig u == (Seq.index orig_re u).f_values);
          assert (Seq.index orig (u+1) == (Seq.index orig_re (u+1)).f_values);
          assert (Seq.index fut u == (Seq.index re u).f_values);
          assert (Seq.index fut (u+1) == (Seq.index re (u+1)).f_values);
          lemma_atom_to_bf_inv_cross (Seq.index orig u) (Seq.index orig (u+1))
                                     (Seq.index fut u) (Seq.index fut (u+1)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 2 == 0) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (31 - u / 2) ] <: i32) * pow2 32) % 8380417)
      = if (u % 2 = 0) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = Spec.MLDSA.NttConstants.zeta_r (31 - u / 2) in
          Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (31 - u / 2)
        end
     in Classical.forall_intro aux_z);
    Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_3_cross_to_hacspec_poly orig fut zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_inv_l4_cross_driver_compose
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 4 < 2) ==>
            unit_fe_post_inv_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+2)).f_values
                                   (Seq.index re u).f_values (Seq.index re (u+2)).f_values
                                   (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (15 - u / 4)))))
        (ensures
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 4) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re orig_re in
    let fut = Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r (15 - u / 4)) in
    Libcrux_ml_dsa.Simd.Portable.Ntt_theory.forall32_elim_1d (fun u -> (u % 4 < 2) ==>
        unit_fe_post_inv_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+2)).f_values
                               (Seq.index re u).f_values (Seq.index re (u+2)).f_values (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 4 < 2) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+2) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+2) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
          (v (Seq.index co_hi l)) % 8380417 ==
            ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm u) * 8265825) % 8380417))
      = if (u % 4 < 2) then begin
          Hacspec_ml_dsa.Commute.Chunk.lemma_cross_idx 2 u 0;
          FStar.Math.Lemmas.small_mod (u + 2) 32;
          assert (v (mk_usize u) == u);
          assert (v (mk_usize (u+2)) == u+2);
          assert (Seq.index orig u == (Seq.index orig_re u).f_values);
          assert (Seq.index orig (u+2) == (Seq.index orig_re (u+2)).f_values);
          assert (Seq.index fut u == (Seq.index re u).f_values);
          assert (Seq.index fut (u+2) == (Seq.index re (u+2)).f_values);
          lemma_atom_to_bf_inv_cross (Seq.index orig u) (Seq.index orig (u+2))
                                     (Seq.index fut u) (Seq.index fut (u+2)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 4 < 2) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (15 - u / 4) ] <: i32) * pow2 32) % 8380417)
      = if (u % 4 < 2) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = Spec.MLDSA.NttConstants.zeta_r (15 - u / 4) in
          Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (15 - u / 4)
        end
     in Classical.forall_intro aux_z);
    Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_4_cross_to_hacspec_poly orig fut zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_inv_l5_cross_driver_compose
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 8 < 4) ==>
            unit_fe_post_inv_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+4)).f_values
                                   (Seq.index re u).f_values (Seq.index re (u+4)).f_values
                                   (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (7 - u / 8)))))
        (ensures
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 5) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re orig_re in
    let fut = Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r (7 - u / 8)) in
    Libcrux_ml_dsa.Simd.Portable.Ntt_theory.forall32_elim_1d (fun u -> (u % 8 < 4) ==>
        unit_fe_post_inv_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+4)).f_values
                               (Seq.index re u).f_values (Seq.index re (u+4)).f_values (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 8 < 4) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+4) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+4) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
          (v (Seq.index co_hi l)) % 8380417 ==
            ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm u) * 8265825) % 8380417))
      = if (u % 8 < 4) then begin
          Hacspec_ml_dsa.Commute.Chunk.lemma_cross_idx 4 u 0;
          FStar.Math.Lemmas.small_mod (u + 4) 32;
          assert (v (mk_usize u) == u);
          assert (v (mk_usize (u+4)) == u+4);
          assert (Seq.index orig u == (Seq.index orig_re u).f_values);
          assert (Seq.index orig (u+4) == (Seq.index orig_re (u+4)).f_values);
          assert (Seq.index fut u == (Seq.index re u).f_values);
          assert (Seq.index fut (u+4) == (Seq.index re (u+4)).f_values);
          lemma_atom_to_bf_inv_cross (Seq.index orig u) (Seq.index orig (u+4))
                                     (Seq.index fut u) (Seq.index fut (u+4)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 8 < 4) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (7 - u / 8) ] <: i32) * pow2 32) % 8380417)
      = if (u % 8 < 4) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = Spec.MLDSA.NttConstants.zeta_r (7 - u / 8) in
          Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (7 - u / 8)
        end
     in Classical.forall_intro aux_z);
    Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_5_cross_to_hacspec_poly orig fut zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_inv_l6_cross_driver_compose
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 16 < 8) ==>
            unit_fe_post_inv_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+8)).f_values
                                   (Seq.index re u).f_values (Seq.index re (u+8)).f_values
                                   (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (3 - u / 16)))))
        (ensures
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 6) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re orig_re in
    let fut = Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r (3 - u / 16)) in
    Libcrux_ml_dsa.Simd.Portable.Ntt_theory.forall32_elim_1d (fun u -> (u % 16 < 8) ==>
        unit_fe_post_inv_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+8)).f_values
                               (Seq.index re u).f_values (Seq.index re (u+8)).f_values (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 16 < 8) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+8) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+8) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
          (v (Seq.index co_hi l)) % 8380417 ==
            ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm u) * 8265825) % 8380417))
      = if (u % 16 < 8) then begin
          Hacspec_ml_dsa.Commute.Chunk.lemma_cross_idx 8 u 0;
          FStar.Math.Lemmas.small_mod (u + 8) 32;
          assert (v (mk_usize u) == u);
          assert (v (mk_usize (u+8)) == u+8);
          assert (Seq.index orig u == (Seq.index orig_re u).f_values);
          assert (Seq.index orig (u+8) == (Seq.index orig_re (u+8)).f_values);
          assert (Seq.index fut u == (Seq.index re u).f_values);
          assert (Seq.index fut (u+8) == (Seq.index re (u+8)).f_values);
          lemma_atom_to_bf_inv_cross (Seq.index orig u) (Seq.index orig (u+8))
                                     (Seq.index fut u) (Seq.index fut (u+8)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 16 < 8) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (3 - u / 16) ] <: i32) * pow2 32) % 8380417)
      = if (u % 16 < 8) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = Spec.MLDSA.NttConstants.zeta_r (3 - u / 16) in
          Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (3 - u / 16)
        end
     in Classical.forall_intro aux_z);
    Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_6_cross_to_hacspec_poly orig fut zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_inv_l7_cross_driver_compose
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 32 < 16) ==>
            unit_fe_post_inv_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+16)).f_values
                                   (Seq.index re u).f_values (Seq.index re (u+16)).f_values
                                   (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1))))
        (ensures
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 7) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re orig_re in
    let fut = Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1) in
    Libcrux_ml_dsa.Simd.Portable.Ntt_theory.forall32_elim_1d (fun u -> (u % 32 < 16) ==>
        unit_fe_post_inv_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+16)).f_values
                               (Seq.index re u).f_values (Seq.index re (u+16)).f_values (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 32 < 16) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+16) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+16) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index ci_hi l) /\
          (v (Seq.index co_hi l)) % 8380417 ==
            ((v (Seq.index ci_hi l) - v (Seq.index ci_lo l)) * v (zm u) * 8265825) % 8380417))
      = if (u % 32 < 16) then begin
          Hacspec_ml_dsa.Commute.Chunk.lemma_cross_idx 16 u 0;
          FStar.Math.Lemmas.small_mod (u + 16) 32;
          assert (v (mk_usize u) == u);
          assert (v (mk_usize (u+16)) == u+16);
          assert (Seq.index orig u == (Seq.index orig_re u).f_values);
          assert (Seq.index orig (u+16) == (Seq.index orig_re (u+16)).f_values);
          assert (Seq.index fut u == (Seq.index re u).f_values);
          assert (Seq.index fut (u+16) == (Seq.index re (u+16)).f_values);
          lemma_atom_to_bf_inv_cross (Seq.index orig u) (Seq.index orig (u+16))
                                     (Seq.index fut u) (Seq.index fut (u+16)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 32 < 16) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (1 - u / 32) ] <: i32) * pow2 32) % 8380417)
      = if (u % 32 < 16) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          assert (1 - u / 32 == 1);
          let _ = Spec.MLDSA.NttConstants.zeta_r 1 in
          Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta 1
        end
     in Classical.forall_intro aux_z);
    Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_7_cross_to_hacspec_poly orig fut zm
#pop-options

#push-options "--z3rlimit 400 --split_queries always"
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
(* per-chunk (16382) scaling lifts to flat-poly scaling *)
let lemma_scale_flat
    (orig fut : t_Array (t_Array i32 (mk_usize 8)) (mk_usize 32)) : Lemma
  (requires
    (forall (b:nat) (l:nat). b < 32 /\ l < 8 ==>
      (v (Seq.index (Seq.index fut b) l)) % 8380417 ==
      (16382 * v (Seq.index (Seq.index orig b) l)) % 8380417))
  (ensures
    (forall (j:nat). j < 256 ==>
      (v (Seq.index (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array fut) j)) % 8380417 ==
      (16382 * v (Seq.index (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array orig) j)) % 8380417))
  = let aux (j:nat{j < 256}) : Lemma
        ((v (Seq.index (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array fut) j)) % 8380417 ==
         (16382 * v (Seq.index (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array orig) j)) % 8380417) =
      let b : nat = j / 8 in
      let l : nat = j % 8 in
      FStar.Math.Lemmas.lemma_div_mod j 8;
      assert (b < 32 /\ l < 8 /\ 8*b + l == j);
      Hacspec_ml_dsa.Commute.Chunk.lemma_simd_units_to_array_reveal fut b l;
      Hacspec_ml_dsa.Commute.Chunk.lemma_simd_units_to_array_reveal orig b l
    in Classical.forall_intro aux
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
(* STANDALONE clean-context arithmetic (plain ints, NO forall): mod_q a == mod_q (x*41978*8265825)
   ==> a ≡ 16382*x (mod q).  41978*8265825 % q == 16382 (R^2/2^8 * R^{-1} collapse). *)
let lemma_modq_scale_one (a x : int) : Lemma
  (requires Spec.MLDSA.Math.mod_q a == Spec.MLDSA.Math.mod_q (x * 41978 * 8265825))
  (ensures a % 8380417 == (16382 * x) % 8380417)
  = reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
    FStar.Math.Lemmas.paren_mul_right x 41978 8265825;
    assert_norm (41978 * 8265825 == 346982801850);
    FStar.Math.Lemmas.lemma_mod_mul_distr_r x 346982801850 8380417;
    assert_norm (346982801850 % 8380417 == 16382)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
(* montgomery_multiply_by_constant's per-lane Spec.MLDSA.Math.mod_q post -> 16382 %q form.
   The forall instantiates while mod_q is STILL opaque (trigger intact); the helper
   reveals in isolation -- revealing here would break the requires-forall trigger. *)
let lemma_scale_chunk (ci co : t_Array i32 (mk_usize 8)) : Lemma
  (requires (forall (l:nat). l < 8 ==>
     Spec.MLDSA.Math.mod_q (v (Seq.index co l)) ==
     Spec.MLDSA.Math.mod_q (v (Seq.index ci l) * 41978 * 8265825)))
  (ensures (forall (l:nat). l < 8 ==>
     (v (Seq.index co l)) % 8380417 == (16382 * v (Seq.index ci l)) % 8380417))
  = let aux (l:nat{l<8}) : Lemma
        ((v (Seq.index co l)) % 8380417 == (16382 * v (Seq.index ci l)) % 8380417) =
      lemma_modq_scale_one (v (Seq.index co l)) (v (Seq.index ci l))
    in Classical.forall_intro aux
#pop-options

(* OPAQUE per-chunk scaling atom: keeps the scale_montgomery loop invariant's WP
   small + deterministic (the raw nested forall makes a high-variance VC that Z3
   sometimes solves in <2s, sometimes blows past rlimit 400).  Same atom shape as
   Spec.Utils.is_i32b_array_opaque already used in this invariant. *)
[@@ "opaque_to_smt"]
let chunk_scaled (orig_chunk cur_chunk : t_Array i32 (mk_usize 8)) : Type0 =
  forall (l:nat). l < 8 ==>
    (v (Seq.index cur_chunk l)) % 8380417 == (16382 * v (Seq.index orig_chunk l)) % 8380417

#restart-solver
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
(* TIGHT per-lane bound for the final scale-back multiply.  The inverse-NTT
   layers leave each lane bounded by 256*FIELD_MAX; the
   `montgomery_multiply_by_constant(_, 41978)` then reduces it to the centered
   bound 4211177 = q/2 + ceil(256*FIELD_MAX*41978/2^32) via
   `Spec.MLDSA.Math.lemma_mont_red_bound_256_field_max_times_41978`.  Mirror of
   the AVX2 `lemma_mont_mul_tight_bound_256`. *)
let lemma_mont_mul_tight_bound_256 (x c: i32)
    : Lemma
        (requires Spec.Utils.is_i32b (256 * 8380416) x /\ v c == 41978)
        (ensures Spec.Utils.is_i32b 4211177 (Spec.MLDSA.Math.mont_mul x c))
  = Spec.Intrinsics.reveal_opaque_arithmetic_ops #i32_inttype;
    Spec.Intrinsics.reveal_opaque_arithmetic_ops #i64_inttype;
    Spec.Intrinsics.reveal_opaque_cast_ops #i32_inttype #i64_inttype;
    reveal_opaque (`%Spec.MLDSA.Math.i32_mul) (Spec.MLDSA.Math.i32_mul);
    let prod : int = v x * v c in
    assert_norm ((256 * 8380416) * 41978 < pow2 63);
    Spec.Utils.lemma_range_at_percent (v x) (pow2 64);
    Spec.Utils.lemma_range_at_percent (v c) (pow2 64);
    let cast_x : i64 = cast x <: i64 in
    let cast_y : i64 = cast c <: i64 in
    assert (v cast_x == v x /\ v cast_y == v c);
    let value : i64 = Spec.MLDSA.Math.i32_mul x c in
    Spec.Utils.lemma_range_at_percent prod (pow2 64);
    assert (v value == prod);
    FStar.Math.Lemmas.lemma_abs_mul (v x) (v c);
    assert (Spec.Utils.is_i64b (256 * 8380416 * 41978) value);
    Spec.MLDSA.Math.lemma_mont_red_bound_256_field_max_times_41978 value
#pop-options

#restart-solver
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
(* Lift the per-lane tight bound to a whole chunk: from the 256*FIELD_MAX input
   bound and the per-lane mont_mul-by-41978 equality (montgomery_multiply_by_constant's
   post), each output lane is bounded by 4211177. *)
let lemma_scale_chunk_tight_bound (orig_chunk cur_chunk : t_Array i32 (mk_usize 8)) : Lemma
  (requires
    Spec.Utils.is_i32b_array_opaque (256 * v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX) orig_chunk /\
    (forall (l:nat). l < 8 ==>
       Seq.index cur_chunk l == Spec.MLDSA.Math.mont_mul (Seq.index orig_chunk l) (mk_i32 41978)))
  (ensures Spec.Utils.is_i32b_array_opaque 4211177 cur_chunk)
  = assert_norm (v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX == 8380416);
    reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque);
    let aux (l:nat{l<8}) : Lemma (Spec.Utils.is_i32b 4211177 (Seq.index cur_chunk l)) =
      lemma_mont_mul_tight_bound_256 (Seq.index orig_chunk l) (mk_i32 41978)
    in Classical.forall_intro aux
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
(* establish the opaque atom from montgomery_multiply_by_constant's mod_q post *)
let lemma_establish_chunk_scaled (ci co : t_Array i32 (mk_usize 8)) : Lemma
  (requires (forall (l:nat). l < 8 ==>
     Spec.MLDSA.Math.mod_q (v (Seq.index co l)) ==
     Spec.MLDSA.Math.mod_q (v (Seq.index ci l) * 41978 * 8265825)))
  (ensures chunk_scaled ci co)
  = lemma_scale_chunk ci co;
    reveal_opaque (`%chunk_scaled) (chunk_scaled ci co)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
(* consume the OPAQUE chunk_scaled atom (loop-invariant output), reveal per-chunk,
   bridge chunks_of_re, lift to flat. *)
let lemma_scale_driver
    (orig_re fut_re : t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (mk_usize 32)) : Lemma
  (requires (forall (b:nat). b < 32 ==>
     chunk_scaled (Seq.index orig_re b).f_values (Seq.index fut_re b).f_values))
  (ensures
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re orig_re) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re fut_re) in
     forall (j:nat). j < 256 ==>
       (v (Seq.index out_flat j)) % 8380417 == (16382 * v (Seq.index in_flat j)) % 8380417))
  = let ci = Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re orig_re in
    let co = Libcrux_ml_dsa.Simd.Portable.Ntt_theory.chunks_of_re fut_re in
    let aux (b:nat{b<32}) : Lemma
        (forall (l:nat). l < 8 ==>
          (v (Seq.index (Seq.index co b) l)) % 8380417 ==
          (16382 * v (Seq.index (Seq.index ci b) l)) % 8380417) =
      reveal_opaque (`%chunk_scaled) (chunk_scaled (Seq.index orig_re b).f_values (Seq.index fut_re b).f_values);
      Hacspec_ml_dsa.createi_lemma #(t_Array i32 (mk_usize 8)) (mk_usize 32) #(usize -> t_Array i32 (mk_usize 8))
        (fun (bb:usize{bb <. mk_usize 32}) -> (Seq.index fut_re (v bb)).f_values) (mk_usize b);
      Hacspec_ml_dsa.createi_lemma #(t_Array i32 (mk_usize 8)) (mk_usize 32) #(usize -> t_Array i32 (mk_usize 8))
        (fun (bb:usize{bb <. mk_usize 32}) -> (Seq.index orig_re (v bb)).f_values) (mk_usize b)
    in Classical.forall_intro aux;
    lemma_scale_flat ci co
#pop-options
#pop-options

#push-options "--z3rlimit 400 --split_queries always"
let lemma_modq_eq (xa xb : i64) : Lemma
    (requires (v xa) % 8380417 == (v xb) % 8380417)
    (ensures Hacspec_ml_dsa.Arithmetic.mod_q xa == Hacspec_ml_dsa.Arithmetic.mod_q xb)
  = Hacspec_ml_dsa.Commute.Chunk.lemma_mod_q_v xa; Hacspec_ml_dsa.Commute.Chunk.lemma_mod_q_v xb

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_inv_bf_even_cong (x y x' y': i32) : Lemma
    (requires (v x) % 8380417 == (v x') % 8380417 /\ (v y) % 8380417 == (v y') % 8380417)
    (ensures
      Hacspec_ml_dsa.Arithmetic.mod_q ((cast x <: i64) +! (cast y <: i64)) ==
      Hacspec_ml_dsa.Arithmetic.mod_q ((cast x' <: i64) +! (cast y' <: i64)))
  = FStar.Math.Lemmas.modulo_distributivity (v x) (v y) 8380417;
    FStar.Math.Lemmas.modulo_distributivity (v x') (v y') 8380417;
    lemma_modq_eq ((cast x <: i64) +! (cast y <: i64)) ((cast x' <: i64) +! (cast y' <: i64))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_inv_bf_odd_cong (z: i64) (x y x' y': i32) : Lemma
    (requires (v z) >= -2147483648 /\ (v z) <= 2147483647 /\
              (v x) % 8380417 == (v x') % 8380417 /\ (v y) % 8380417 == (v y') % 8380417)
    (ensures
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! ((cast x <: i64) -! (cast y <: i64))) ==
      Hacspec_ml_dsa.Arithmetic.mod_q (z *! ((cast x' <: i64) -! (cast y' <: i64))))
  = FStar.Math.Lemmas.modulo_distributivity (v x) (- v y) 8380417;
    FStar.Math.Lemmas.modulo_distributivity (v x') (- v y') 8380417;
    FStar.Math.Lemmas.lemma_mod_sub_distr 0 (v y) 8380417;
    FStar.Math.Lemmas.lemma_mod_sub_distr 0 (v y') 8380417;
    FStar.Math.Lemmas.lemma_mod_mul_distr_r (v z) (v x - v y) 8380417;
    FStar.Math.Lemmas.lemma_mod_mul_distr_r (v z) (v x' - v y') 8380417;
    lemma_modq_eq (z *! ((cast x <: i64) -! (cast y <: i64))) (z *! ((cast x' <: i64) -! (cast y' <: i64)))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_inv_layer_0_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.intt_layer_0_lane a ii == Hacspec_ml_dsa.Commute.Chunk.intt_layer_0_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 2 in
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 255 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) in
    assert (v z >= 0 /\ v z < 8380417);
    FStar.Math.Lemmas.lemma_mod_lt i 2;
    FStar.Math.Lemmas.lemma_div_mod i 2;
    let parity : (n:nat{n < 2}) = i % 2 in
    assert (v (ii %! mk_usize 2) == parity);
    if parity < 1 then begin
      assert (ii %! mk_usize 2 <. mk_usize 1);
      assert (i + 1 < 256);
      lemma_inv_bf_even_cong (Seq.index a i) (Seq.index a (i + 1))
                             (Seq.index b i) (Seq.index b (i + 1))
    end else begin
      assert (~(ii %! mk_usize 2 <. mk_usize 1));
      assert (i >= 1);
      lemma_inv_bf_odd_cong z (Seq.index a (i - 1)) (Seq.index a i)
                              (Seq.index b (i - 1)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_inv_layer_1_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.intt_layer_1_lane a ii == Hacspec_ml_dsa.Commute.Chunk.intt_layer_1_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 4 in
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 127 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) in
    assert (v z >= 0 /\ v z < 8380417);
    FStar.Math.Lemmas.lemma_mod_lt i 4;
    FStar.Math.Lemmas.lemma_div_mod i 4;
    let parity : (n:nat{n < 4}) = i % 4 in
    assert (v (ii %! mk_usize 4) == parity);
    if parity < 2 then begin
      assert (ii %! mk_usize 4 <. mk_usize 2);
      assert (i + 2 < 256);
      lemma_inv_bf_even_cong (Seq.index a i) (Seq.index a (i + 2))
                             (Seq.index b i) (Seq.index b (i + 2))
    end else begin
      assert (~(ii %! mk_usize 4 <. mk_usize 2));
      assert (i >= 2);
      lemma_inv_bf_odd_cong z (Seq.index a (i - 2)) (Seq.index a i)
                              (Seq.index b (i - 2)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_inv_layer_2_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.intt_layer_2_lane a ii == Hacspec_ml_dsa.Commute.Chunk.intt_layer_2_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 8 in
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 63 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) in
    assert (v z >= 0 /\ v z < 8380417);
    FStar.Math.Lemmas.lemma_mod_lt i 8;
    FStar.Math.Lemmas.lemma_div_mod i 8;
    let parity : (n:nat{n < 8}) = i % 8 in
    assert (v (ii %! mk_usize 8) == parity);
    if parity < 4 then begin
      assert (ii %! mk_usize 8 <. mk_usize 4);
      assert (i + 4 < 256);
      lemma_inv_bf_even_cong (Seq.index a i) (Seq.index a (i + 4))
                             (Seq.index b i) (Seq.index b (i + 4))
    end else begin
      assert (~(ii %! mk_usize 8 <. mk_usize 4));
      assert (i >= 4);
      lemma_inv_bf_odd_cong z (Seq.index a (i - 4)) (Seq.index a i)
                              (Seq.index b (i - 4)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_inv_layer_3_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.intt_layer_3_lane a ii == Hacspec_ml_dsa.Commute.Chunk.intt_layer_3_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 16 in
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 31 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) in
    assert (v z >= 0 /\ v z < 8380417);
    FStar.Math.Lemmas.lemma_mod_lt i 16;
    FStar.Math.Lemmas.lemma_div_mod i 16;
    let parity : (n:nat{n < 16}) = i % 16 in
    assert (v (ii %! mk_usize 16) == parity);
    if parity < 8 then begin
      assert (ii %! mk_usize 16 <. mk_usize 8);
      assert (i + 8 < 256);
      lemma_inv_bf_even_cong (Seq.index a i) (Seq.index a (i + 8))
                             (Seq.index b i) (Seq.index b (i + 8))
    end else begin
      assert (~(ii %! mk_usize 16 <. mk_usize 8));
      assert (i >= 8);
      lemma_inv_bf_odd_cong z (Seq.index a (i - 8)) (Seq.index a i)
                              (Seq.index b (i - 8)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_inv_layer_4_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.intt_layer_4_lane a ii == Hacspec_ml_dsa.Commute.Chunk.intt_layer_4_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 32 in
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 15 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) in
    assert (v z >= 0 /\ v z < 8380417);
    FStar.Math.Lemmas.lemma_mod_lt i 32;
    FStar.Math.Lemmas.lemma_div_mod i 32;
    let parity : (n:nat{n < 32}) = i % 32 in
    assert (v (ii %! mk_usize 32) == parity);
    if parity < 16 then begin
      assert (ii %! mk_usize 32 <. mk_usize 16);
      assert (i + 16 < 256);
      lemma_inv_bf_even_cong (Seq.index a i) (Seq.index a (i + 16))
                             (Seq.index b i) (Seq.index b (i + 16))
    end else begin
      assert (~(ii %! mk_usize 32 <. mk_usize 16));
      assert (i >= 16);
      lemma_inv_bf_odd_cong z (Seq.index a (i - 16)) (Seq.index a i)
                              (Seq.index b (i - 16)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_inv_layer_5_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.intt_layer_5_lane a ii == Hacspec_ml_dsa.Commute.Chunk.intt_layer_5_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 64 in
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 7 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) in
    assert (v z >= 0 /\ v z < 8380417);
    FStar.Math.Lemmas.lemma_mod_lt i 64;
    FStar.Math.Lemmas.lemma_div_mod i 64;
    let parity : (n:nat{n < 64}) = i % 64 in
    assert (v (ii %! mk_usize 64) == parity);
    if parity < 32 then begin
      assert (ii %! mk_usize 64 <. mk_usize 32);
      assert (i + 32 < 256);
      lemma_inv_bf_even_cong (Seq.index a i) (Seq.index a (i + 32))
                             (Seq.index b i) (Seq.index b (i + 32))
    end else begin
      assert (~(ii %! mk_usize 64 <. mk_usize 32));
      assert (i >= 32);
      lemma_inv_bf_odd_cong z (Seq.index a (i - 32)) (Seq.index a i)
                              (Seq.index b (i - 32)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_inv_layer_6_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.intt_layer_6_lane a ii == Hacspec_ml_dsa.Commute.Chunk.intt_layer_6_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 128 in
    FStar.Math.Lemmas.lemma_div_mod i 128;  (* cold-stable: v round = i/128 <= 1, for the (mk_usize 3 -! round) subtyping *)
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 3 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) in
    assert (v z >= 0 /\ v z < 8380417);
    FStar.Math.Lemmas.lemma_mod_lt i 128;
    FStar.Math.Lemmas.lemma_div_mod i 128;
    let parity : (n:nat{n < 128}) = i % 128 in
    assert (v (ii %! mk_usize 128) == parity);
    if parity < 64 then begin
      assert (ii %! mk_usize 128 <. mk_usize 64);
      assert (i + 64 < 256);
      lemma_inv_bf_even_cong (Seq.index a i) (Seq.index a (i + 64))
                             (Seq.index b i) (Seq.index b (i + 64))
    end else begin
      assert (~(ii %! mk_usize 128 <. mk_usize 64));
      assert (i >= 64);
      lemma_inv_bf_odd_cong z (Seq.index a (i - 64)) (Seq.index a i)
                              (Seq.index b (i - 64)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_inv_layer_7_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.intt_layer_7_lane a ii == Hacspec_ml_dsa.Commute.Chunk.intt_layer_7_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 256 in
    let z:i64 =
      ((cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) -!
        (cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize 1 -! round <: usize ] <: i32) <: i64)
        <:
        i64) %!
      (cast (Hacspec_ml_dsa.Parameters.v_Q <: i32) <: i64) in
    assert (v z >= 0 /\ v z < 8380417);
    FStar.Math.Lemmas.lemma_mod_lt i 256;
    FStar.Math.Lemmas.lemma_div_mod i 256;
    let parity : (n:nat{n < 256}) = i % 256 in
    assert (v (ii %! mk_usize 256) == parity);
    if parity < 128 then begin
      assert (ii %! mk_usize 256 <. mk_usize 128);
      assert (i + 128 < 256);
      lemma_inv_bf_even_cong (Seq.index a i) (Seq.index a (i + 128))
                             (Seq.index b i) (Seq.index b (i + 128))
    end else begin
      assert (~(ii %! mk_usize 256 <. mk_usize 128));
      assert (i >= 128);
      lemma_inv_bf_odd_cong z (Seq.index a (i - 128)) (Seq.index a i)
                              (Seq.index b (i - 128)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_0_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 0) == Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 0))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 0)) i == Seq.index (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 0)) i) =
      let ii:usize = mk_usize i in
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_0_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_0_lane b ii;
      lemma_inv_layer_0_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 0)) (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 0))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_1_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 1) == Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 1))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 1)) i == Seq.index (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 1)) i) =
      let ii:usize = mk_usize i in
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_1_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_1_lane b ii;
      lemma_inv_layer_1_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 1)) (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 1))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_2_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 2) == Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 2))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 2)) i == Seq.index (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 2)) i) =
      let ii:usize = mk_usize i in
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_2_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_2_lane b ii;
      lemma_inv_layer_2_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 2)) (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 2))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_3_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 3) == Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 3))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 3)) i == Seq.index (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 3)) i) =
      let ii:usize = mk_usize i in
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_3_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_3_lane b ii;
      lemma_inv_layer_3_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 3)) (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 3))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_4_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 4) == Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 4))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 4)) i == Seq.index (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 4)) i) =
      let ii:usize = mk_usize i in
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_4_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_4_lane b ii;
      lemma_inv_layer_4_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 4)) (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 4))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_5_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 5) == Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 5))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 5)) i == Seq.index (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 5)) i) =
      let ii:usize = mk_usize i in
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_5_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_5_lane b ii;
      lemma_inv_layer_5_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 5)) (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 5))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_6_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 6) == Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 6))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 6)) i == Seq.index (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 6)) i) =
      let ii:usize = mk_usize i in
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_6_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_6_lane b ii;
      lemma_inv_layer_6_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 6)) (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 6))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_layer_7_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 7) == Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 7))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 7)) i == Seq.index (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 7)) i) =
      let ii:usize = mk_usize i in
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_7_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_7_lane b ii;
      lemma_inv_layer_7_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.intt_layer a (mk_usize 7)) (Hacspec_ml_dsa.Ntt.intt_layer b (mk_usize 7))
#pop-options

[@@ "opaque_to_smt"]
let intt_unscaled (w: t_Array i32 (mk_usize 256)) : t_Array i32 (mk_usize 256) =
  let p:t_Array i32 (mk_usize 256) = Hacspec_ml_dsa.Ntt.intt_layer w (mk_usize 0) in
  let p:t_Array i32 (mk_usize 256) = Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 1) in
  let p:t_Array i32 (mk_usize 256) = Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 2) in
  let p:t_Array i32 (mk_usize 256) = Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 3) in
  let p:t_Array i32 (mk_usize 256) = Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 4) in
  let p:t_Array i32 (mk_usize 256) = Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 5) in
  let p:t_Array i32 (mk_usize 256) = Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 6) in
  Hacspec_ml_dsa.Ntt.intt_layer p (mk_usize 7)

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_intt_compose_8 (f0 f1 f2 f3 f4 f5 f6 f7 ffinal : t_Array i32 (mk_usize 256)) : Lemma
    (requires
      (forall (i:nat). i < 256 ==> (v (Seq.index f1 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer f0 (mk_usize 0)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index f2 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer f1 (mk_usize 1)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index f3 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer f2 (mk_usize 2)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index f4 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer f3 (mk_usize 3)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index f5 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer f4 (mk_usize 4)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index f6 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer f5 (mk_usize 5)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index f7 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer f6 (mk_usize 6)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index ffinal i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.intt_layer f7 (mk_usize 7)) i)) % 8380417))
    (ensures
      (forall (i:nat). i < 256 ==> (v (Seq.index ffinal i)) % 8380417 == (v (Seq.index (intt_unscaled f0) i)) % 8380417))
  = let g0 = Hacspec_ml_dsa.Ntt.intt_layer f0 (mk_usize 0) in
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f1 i)) % 8380417 == (v (Seq.index g0 i)) % 8380417);
    lemma_intt_layer_1_cong f1 g0;
    let g1 = Hacspec_ml_dsa.Ntt.intt_layer g0 (mk_usize 1) in
    assert (Hacspec_ml_dsa.Ntt.intt_layer f1 (mk_usize 1) == g1);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f2 i)) % 8380417 == (v (Seq.index g1 i)) % 8380417);
    lemma_intt_layer_2_cong f2 g1;
    let g2 = Hacspec_ml_dsa.Ntt.intt_layer g1 (mk_usize 2) in
    assert (Hacspec_ml_dsa.Ntt.intt_layer f2 (mk_usize 2) == g2);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f3 i)) % 8380417 == (v (Seq.index g2 i)) % 8380417);
    lemma_intt_layer_3_cong f3 g2;
    let g3 = Hacspec_ml_dsa.Ntt.intt_layer g2 (mk_usize 3) in
    assert (Hacspec_ml_dsa.Ntt.intt_layer f3 (mk_usize 3) == g3);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f4 i)) % 8380417 == (v (Seq.index g3 i)) % 8380417);
    lemma_intt_layer_4_cong f4 g3;
    let g4 = Hacspec_ml_dsa.Ntt.intt_layer g3 (mk_usize 4) in
    assert (Hacspec_ml_dsa.Ntt.intt_layer f4 (mk_usize 4) == g4);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f5 i)) % 8380417 == (v (Seq.index g4 i)) % 8380417);
    lemma_intt_layer_5_cong f5 g4;
    let g5 = Hacspec_ml_dsa.Ntt.intt_layer g4 (mk_usize 5) in
    assert (Hacspec_ml_dsa.Ntt.intt_layer f5 (mk_usize 5) == g5);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f6 i)) % 8380417 == (v (Seq.index g5 i)) % 8380417);
    lemma_intt_layer_6_cong f6 g5;
    let g6 = Hacspec_ml_dsa.Ntt.intt_layer g5 (mk_usize 6) in
    assert (Hacspec_ml_dsa.Ntt.intt_layer f6 (mk_usize 6) == g6);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f7 i)) % 8380417 == (v (Seq.index g6 i)) % 8380417);
    lemma_intt_layer_7_cong f7 g6;
    let g7 = Hacspec_ml_dsa.Ntt.intt_layer g6 (mk_usize 7) in
    assert (Hacspec_ml_dsa.Ntt.intt_layer f7 (mk_usize 7) == g7);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index ffinal i)) % 8380417 == (v (Seq.index g7 i)) % 8380417);
    reveal_opaque (`%intt_unscaled) intt_unscaled;
    assert (intt_unscaled f0 == g7)
#pop-options

(* ---- scaling wrapper.  out ≡ to_mont(intt in) (mod q), with
   to_mont x = mod_q(R·x), R = 2^32 mod q = 4193792.  The impl stays in the
   Montgomery domain (mont_mul by 41978 = R·256^{-1}), so it is off the clean
   intt by R. *)
let to_mont (p: t_Array i32 (mk_usize 256)) : t_Array i32 (mk_usize 256) =
  Hacspec_ml_dsa.createi #i32 (mk_usize 256) #(usize -> i32)
    (fun i -> Hacspec_ml_dsa.Arithmetic.mod_q (mk_i64 4193792 *! (cast (p.[i] <: i32) <: i64)))

#restart-solver
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200"
(* STANDALONE clean-context arithmetic: a == mod_q(8347681*b) ==> mod_q(R*a) ≡ 16382*b (mod q).
   --z3refresh: cold-gate stability (full-build query-state accumulation causes "incomplete
   quantifiers" that admit_except doesn't see). *)
let lemma_scale_arith (a b : i32) : Lemma
  (requires a == Hacspec_ml_dsa.Arithmetic.mod_q (mk_i64 8347681 *! (cast b <: i64)))
  (ensures (v (Hacspec_ml_dsa.Arithmetic.mod_q (mk_i64 4193792 *! (cast a <: i64)))) % 8380417
           == (16382 * v b) % 8380417)
  = Hacspec_ml_dsa.Commute.Chunk.lemma_mod_q_v (mk_i64 4193792 *! (cast a <: i64));
    Hacspec_ml_dsa.Commute.Chunk.lemma_mod_q_v (mk_i64 8347681 *! (cast b <: i64));
    FStar.Math.Lemmas.lemma_mod_mul_distr_r 4193792 (v a) 8380417;
    FStar.Math.Lemmas.lemma_mod_mul_distr_r 4193792 (8347681 * v b) 8380417;
    assert_norm ((4193792 * 8347681) % 8380417 == 16382);
    (* cold-stable: left-factor distribution links (4193792*8347681*vb)%q to (16382*vb)%q *)
    FStar.Math.Lemmas.lemma_mod_mul_distr_l (4193792 * 8347681) (v b) 8380417;
    FStar.Math.Lemmas.lemma_mod_mul_distr_r 16382 (v b) 8380417
#pop-options

#restart-solver
#push-options "--z3rlimit 300 --split_queries always"
(* to_mont(intt p)[i] ≡ 16382 * intt_unscaled(p)[i]  (intt = reduce_polynomial o intt_unscaled) *)
let lemma_to_mont_intt (p: t_Array i32 (mk_usize 256)) (i: nat{i < 256}) : Lemma
  (ensures
    (v (Seq.index (to_mont (Hacspec_ml_dsa.Ntt.intt p)) i)) % 8380417 ==
    (16382 * v (Seq.index (intt_unscaled p) i)) % 8380417)
  = reveal_opaque (`%intt_unscaled) intt_unscaled;
    let ii : usize = mk_usize i in
    let iu = intt_unscaled p in
    let a : i32 = Seq.index (Hacspec_ml_dsa.Ntt.intt p) i in
    let b : i32 = Seq.index iu i in
    Hacspec_ml_dsa.createi_lemma #i32 (mk_usize 256) #(usize -> i32)
      (fun j -> Hacspec_ml_dsa.Arithmetic.mod_q (mk_i64 4193792 *! (cast ((Hacspec_ml_dsa.Ntt.intt p).[j] <: i32) <: i64))) ii;
    Hacspec_ml_dsa.createi_lemma #i32 (mk_usize 256) #(usize -> i32)
      (fun j -> Hacspec_ml_dsa.Arithmetic.mod_q (mk_i64 8347681 *! (cast (iu.[j] <: i32) <: i64))) ii;
    assert (a == Hacspec_ml_dsa.Arithmetic.mod_q (mk_i64 8347681 *! (cast b <: i64)));
    lemma_scale_arith a b
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
(* congruence lifts through *16382 *)
let lemma_cong_mul16382 (a b : int) : Lemma
  (requires a % 8380417 == b % 8380417)
  (ensures (16382 * a) % 8380417 == (16382 * b) % 8380417)
  = FStar.Math.Lemmas.lemma_mod_mul_distr_r 16382 a 8380417;
    FStar.Math.Lemmas.lemma_mod_mul_distr_r 16382 b 8380417
#pop-options

#restart-solver
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
(* top chain: scale post (out ≡ 16382·s8) + compose post (s8 ≡ intt_unscaled s0)
   -> out ≡ to_mont(intt s0) (mod q) *)
let lemma_invert_top (s0flat s8flat refut : t_Array i32 (mk_usize 256)) : Lemma
  (requires
     (forall (i:nat). i < 256 ==>
        (v (Seq.index refut i)) % 8380417 == (16382 * v (Seq.index s8flat i)) % 8380417) /\
     (forall (i:nat). i < 256 ==>
        (v (Seq.index s8flat i)) % 8380417 == (v (Seq.index (intt_unscaled s0flat) i)) % 8380417))
  (ensures
     (forall (i:nat). i < 256 ==>
        (v (Seq.index refut i)) % 8380417 ==
        (v (Seq.index (to_mont (Hacspec_ml_dsa.Ntt.intt s0flat)) i)) % 8380417))
  = let aux (i:nat{i<256}) : Lemma
        ((v (Seq.index refut i)) % 8380417 ==
         (v (Seq.index (to_mont (Hacspec_ml_dsa.Ntt.intt s0flat)) i)) % 8380417) =
       lemma_cong_mul16382 (v (Seq.index s8flat i)) (v (Seq.index (intt_unscaled s0flat) i));
       lemma_to_mont_intt s0flat i
    in Classical.forall_intro aux
#pop-options
#pop-options

(* --- Relocated from `src/simd/portable.rs` (`invert_ntt_with_proof`
   fstar::before; annotation-uniformity sweep Batch 1).  `to_mont` here is
   byte-identical to -- hence defeq with -- `Spec.MLDSA.Math.to_mont`; this
   bridge makes that explicit for the `invert_func_post` intro.  Consumer:
   Simd.Portable.fst (invert_ntt_with_proof proof! block). --- *)
let lemma_to_mont_eq (y: t_Array i32 (mk_usize 256))
    : Lemma (Libcrux_ml_dsa.Simd.Portable.Invntt_theory.to_mont y == Spec.MLDSA.Math.to_mont y)
  = ()

(* ===========================================================================
   Relocated from `src/simd/portable/invntt.rs` fstar::before blocks
   (annotation-uniformity sweep Batch 3): the INVERSE layer-0/1/2 per-chunk
   theory -- opaque GS-FE atoms `unit_fe_post_inv_l{0,1,2}`, the atom->bf
   bridges, and the per-layer driver-compose lemmas.  Consumers: Invntt.fst
   (layer fn contracts + proof! blocks) via qualified names.
   =========================================================================== *)

(* ---- INVERSE Layer 0: opaque per-chunk GS-FE atom (4 zetas/chunk, pairs (2p,2p+1)).
   GS butterfly: co[2p] = ci[2p] + ci[2p+1] (plain add); the odd lane co[2p+1] is a
   direct mod-q relation (= mont_mul(ci[2p+1]-ci[2p], zeta), NO separate `t` witness). *)
[@@ "opaque_to_smt"]
let unit_fe_post_inv_l0 (ci co: t_Array i32 (sz 8))
                    (zeta0 zeta1 zeta2 zeta3: i32{Spec.Utils.is_i32b 4190208 zeta0 /\ Spec.Utils.is_i32b 4190208 zeta1 /\ Spec.Utils.is_i32b 4190208 zeta2 /\ Spec.Utils.is_i32b 4190208 zeta3}) : Type0 =
  (v (Seq.index co 0) == v (Seq.index ci 0) + v (Seq.index ci 1) /\
   (v (Seq.index co 1)) % 8380417 == ((v (Seq.index ci 1) - v (Seq.index ci 0)) * v zeta0 * 8265825) % 8380417 /\
   v (Seq.index co 2) == v (Seq.index ci 2) + v (Seq.index ci 3) /\
   (v (Seq.index co 3)) % 8380417 == ((v (Seq.index ci 3) - v (Seq.index ci 2)) * v zeta1 * 8265825) % 8380417 /\
   v (Seq.index co 4) == v (Seq.index ci 4) + v (Seq.index ci 5) /\
   (v (Seq.index co 5)) % 8380417 == ((v (Seq.index ci 5) - v (Seq.index ci 4)) * v zeta2 * 8265825) % 8380417 /\
   v (Seq.index co 6) == v (Seq.index ci 6) + v (Seq.index ci 7) /\
   (v (Seq.index co 7)) % 8380417 == ((v (Seq.index ci 7) - v (Seq.index ci 6)) * v zeta3 * 8265825) % 8380417)


#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always"
let lemma_atom_to_bf_inv_l0 (ci co: t_Array i32 (sz 8))
                        (zf: (p: nat{p < 4}) -> (z: i32{Spec.Utils.is_i32b 4190208 z}))
    : Lemma (requires unit_fe_post_inv_l0 ci co (zf 0) (zf 1) (zf 2) (zf 3))
            (ensures
              (forall (p: nat{p < 4}).
                 v (Seq.index co (2*p))   == v (Seq.index ci (2*p)) + v (Seq.index ci (2*p+1)) /\
                 (v (Seq.index co (2*p+1))) % 8380417 ==
                   ((v (Seq.index ci (2*p+1)) - v (Seq.index ci (2*p))) * v (zf p) * 8265825) % 8380417))
  = reveal_opaque (`%unit_fe_post_inv_l0) unit_fe_post_inv_l0;
    introduce forall (p: nat{p < 4}).
        (v (Seq.index co (2*p))   == v (Seq.index ci (2*p)) + v (Seq.index ci (2*p+1)) /\
         (v (Seq.index co (2*p+1))) % 8380417 ==
           ((v (Seq.index ci (2*p+1)) - v (Seq.index ci (2*p))) * v (zf p) * 8265825) % 8380417)
    with (match p with | 0 -> () | 1 -> () | 2 -> () | _ -> ())
#pop-options


#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_inv_l0_driver_compose
      (orig fut: t_Array (t_Array i32 (sz 8)) (sz 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun b ->
            unit_fe_post_inv_l0 (Seq.index orig b) (Seq.index fut b)
                            (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (255 - (4*b + 0))))
                            (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (255 - (4*b + 1))))
                            (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (255 - (4*b + 2))))
                            (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (255 - (4*b + 3))))))
        (ensures
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array orig in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 0) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (b: nat{b < 32}) (p: nat{p < 4}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (Spec.MLDSA.NttConstants.zeta_r (255 - (4*b + p))) in
    Libcrux_ml_dsa.Simd.Portable.Ntt_theory.forall32_elim_1d (fun b -> unit_fe_post_inv_l0 (Seq.index orig b) (Seq.index fut b)
                                 (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (255 - (4*b + 0))))
                                 (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (255 - (4*b + 1))))
                                 (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (255 - (4*b + 2))))
                                 (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (255 - (4*b + 3)))));
    (let aux (b: nat{b < 32}) (p: nat{p < 4}) : Lemma
       (let ci = Seq.index orig b in
        let co = Seq.index fut b in
        v (Seq.index co (2*p)) == v (Seq.index ci (2*p)) + v (Seq.index ci (2*p+1)) /\
        (v (Seq.index co (2*p+1))) % 8380417 ==
          ((v (Seq.index ci (2*p+1)) - v (Seq.index ci (2*p))) * v (zm b p) * 8265825) % 8380417 /\
        (v (zm b p)) % 8380417 ==
          (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (255 - (4*b + p)) ] <: i32) * pow2 32) % 8380417)
      = lemma_atom_to_bf_inv_l0 (Seq.index orig b) (Seq.index fut b) (fun p -> zm b p);
        reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
        let _ = Spec.MLDSA.NttConstants.zeta_r (255 - (4*b + p)) in
        Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (255 - (4*b + p))
     in Classical.forall_intro_2 aux);
    Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_0_step_to_hacspec_poly orig fut zm
#pop-options


(* ---- INVERSE Layer 1: opaque per-chunk GS-FE atom (2 zetas/chunk, pairs (4h+j,4h+j+2)). *)
[@@ "opaque_to_smt"]
let unit_fe_post_inv_l1 (ci co: t_Array i32 (sz 8))
                    (zeta0 zeta1: i32{Spec.Utils.is_i32b 4190208 zeta0 /\ Spec.Utils.is_i32b 4190208 zeta1}) : Type0 =
  (v (Seq.index co 0) == v (Seq.index ci 0) + v (Seq.index ci 2) /\
   (v (Seq.index co 2)) % 8380417 == ((v (Seq.index ci 2) - v (Seq.index ci 0)) * v zeta0 * 8265825) % 8380417 /\
   v (Seq.index co 1) == v (Seq.index ci 1) + v (Seq.index ci 3) /\
   (v (Seq.index co 3)) % 8380417 == ((v (Seq.index ci 3) - v (Seq.index ci 1)) * v zeta0 * 8265825) % 8380417 /\
   v (Seq.index co 4) == v (Seq.index ci 4) + v (Seq.index ci 6) /\
   (v (Seq.index co 6)) % 8380417 == ((v (Seq.index ci 6) - v (Seq.index ci 4)) * v zeta1 * 8265825) % 8380417 /\
   v (Seq.index co 5) == v (Seq.index ci 5) + v (Seq.index ci 7) /\
   (v (Seq.index co 7)) % 8380417 == ((v (Seq.index ci 7) - v (Seq.index ci 5)) * v zeta1 * 8265825) % 8380417)


#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always"
let lemma_atom_to_bf_inv_l1 (ci co: t_Array i32 (sz 8))
                        (zf: (h: nat{h < 2}) -> (z: i32{Spec.Utils.is_i32b 4190208 z}))
    : Lemma (requires unit_fe_post_inv_l1 ci co (zf 0) (zf 1))
            (ensures
              (forall (h: nat{h < 2}) (j: nat{j < 2}).
                 v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v (Seq.index ci (4*h+j+2)) /\
                 (v (Seq.index co (4*h+j+2))) % 8380417 ==
                   ((v (Seq.index ci (4*h+j+2)) - v (Seq.index ci (4*h+j))) * v (zf h) * 8265825) % 8380417))
  = reveal_opaque (`%unit_fe_post_inv_l1) unit_fe_post_inv_l1;
    introduce forall (h: nat{h < 2}) (j: nat{j < 2}).
        (v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v (Seq.index ci (4*h+j+2)) /\
         (v (Seq.index co (4*h+j+2))) % 8380417 ==
           ((v (Seq.index ci (4*h+j+2)) - v (Seq.index ci (4*h+j))) * v (zf h) * 8265825) % 8380417)
    with (match h with | 0 -> (match j with | 0 -> () | _ -> ()) | _ -> (match j with | 0 -> () | _ -> ()))
#pop-options


#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_inv_l1_driver_compose
      (orig fut: t_Array (t_Array i32 (sz 8)) (sz 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun b ->
            unit_fe_post_inv_l1 (Seq.index orig b) (Seq.index fut b)
                            (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (127 - (2*b + 0))))
                            (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (127 - (2*b + 1))))))
        (ensures
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array orig in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 1) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (b: nat{b < 32}) (h: nat{h < 2}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (Spec.MLDSA.NttConstants.zeta_r (127 - (2*b + h))) in
    Libcrux_ml_dsa.Simd.Portable.Ntt_theory.forall32_elim_1d (fun b -> unit_fe_post_inv_l1 (Seq.index orig b) (Seq.index fut b)
                                 (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (127 - (2*b + 0))))
                                 (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (127 - (2*b + 1)))));
    (let aux_bf (b: nat{b < 32}) : Lemma
       (forall (h: nat{h < 2}) (j: nat{j < 2}).
         (let ci = Seq.index orig b in
          let co = Seq.index fut b in
          v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v (Seq.index ci (4*h+j+2)) /\
          (v (Seq.index co (4*h+j+2))) % 8380417 ==
            ((v (Seq.index ci (4*h+j+2)) - v (Seq.index ci (4*h+j))) * v (zm b h) * 8265825) % 8380417))
      = lemma_atom_to_bf_inv_l1 (Seq.index orig b) (Seq.index fut b) (fun h -> zm b h)
     in Classical.forall_intro aux_bf);
    (let aux_z (b: nat{b < 32}) (h: nat{h < 2}) : Lemma
       ((v (zm b h)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (127 - (2*b + h)) ] <: i32) * pow2 32) % 8380417)
      = reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
        let _ = Spec.MLDSA.NttConstants.zeta_r (127 - (2*b + h)) in
        Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (127 - (2*b + h))
     in Classical.forall_intro_2 aux_z);
    Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_1_step_to_hacspec_poly orig fut zm
#pop-options


(* ---- INVERSE Layer 2: opaque per-chunk GS-FE atom (1 zeta/chunk, pairs (p,p+4)). *)
[@@ "opaque_to_smt"]
let unit_fe_post_inv_l2 (ci co: t_Array i32 (sz 8))
                    (zeta: i32{Spec.Utils.is_i32b 4190208 zeta}) : Type0 =
  (v (Seq.index co 0) == v (Seq.index ci 0) + v (Seq.index ci 4) /\
   (v (Seq.index co 4)) % 8380417 == ((v (Seq.index ci 4) - v (Seq.index ci 0)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co 1) == v (Seq.index ci 1) + v (Seq.index ci 5) /\
   (v (Seq.index co 5)) % 8380417 == ((v (Seq.index ci 5) - v (Seq.index ci 1)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co 2) == v (Seq.index ci 2) + v (Seq.index ci 6) /\
   (v (Seq.index co 6)) % 8380417 == ((v (Seq.index ci 6) - v (Seq.index ci 2)) * v zeta * 8265825) % 8380417 /\
   v (Seq.index co 3) == v (Seq.index ci 3) + v (Seq.index ci 7) /\
   (v (Seq.index co 7)) % 8380417 == ((v (Seq.index ci 7) - v (Seq.index ci 3)) * v zeta * 8265825) % 8380417)


#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always"
let lemma_atom_to_bf_inv_l2 (ci co: t_Array i32 (sz 8))
                        (zeta: i32{Spec.Utils.is_i32b 4190208 zeta})
    : Lemma (requires unit_fe_post_inv_l2 ci co zeta)
            (ensures
              (forall (p: nat{p < 4}).
                 v (Seq.index co p)     == v (Seq.index ci p) + v (Seq.index ci (p+4)) /\
                 (v (Seq.index co (p+4))) % 8380417 ==
                   ((v (Seq.index ci (p+4)) - v (Seq.index ci p)) * v zeta * 8265825) % 8380417))
  = reveal_opaque (`%unit_fe_post_inv_l2) unit_fe_post_inv_l2;
    introduce forall (p: nat{p < 4}).
        (v (Seq.index co p)     == v (Seq.index ci p) + v (Seq.index ci (p+4)) /\
         (v (Seq.index co (p+4))) % 8380417 ==
           ((v (Seq.index ci (p+4)) - v (Seq.index ci p)) * v zeta * 8265825) % 8380417)
    with (match p with | 0 -> () | 1 -> () | 2 -> () | _ -> ())
#pop-options


#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_inv_l2_driver_compose
      (orig fut: t_Array (t_Array i32 (sz 8)) (sz 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun b ->
            unit_fe_post_inv_l2 (Seq.index orig b) (Seq.index fut b)
                            (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (63 - b)))))
        (ensures
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array orig in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array fut in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 2) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let zm (b: nat{b < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
      mk_i32 (Spec.MLDSA.NttConstants.zeta_r (63 - b)) in
    Libcrux_ml_dsa.Simd.Portable.Ntt_theory.forall32_elim_1d (fun b -> unit_fe_post_inv_l2 (Seq.index orig b) (Seq.index fut b)
                                 (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (63 - b))));
    (let aux_bf (b: nat{b < 32}) : Lemma
       (forall (p: nat{p < 4}).
         (let ci = Seq.index orig b in
          let co = Seq.index fut b in
          v (Seq.index co p)     == v (Seq.index ci p) + v (Seq.index ci (p+4)) /\
          (v (Seq.index co (p+4))) % 8380417 ==
            ((v (Seq.index ci (p+4)) - v (Seq.index ci p)) * v (zm b) * 8265825) % 8380417))
      = lemma_atom_to_bf_inv_l2 (Seq.index orig b) (Seq.index fut b) (zm b)
     in Classical.forall_intro aux_bf);
    (let aux_z (b: nat{b < 32}) : Lemma
       ((v (zm b)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (63 - b) ] <: i32) * pow2 32) % 8380417)
      = reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
        let _ = Spec.MLDSA.NttConstants.zeta_r (63 - b) in
        Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (63 - b)
     in Classical.forall_intro aux_z);
    Hacspec_ml_dsa.Commute.Chunk.lemma_intt_layer_2_step_to_hacspec_poly orig fut zm
#pop-options

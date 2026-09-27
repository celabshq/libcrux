module Libcrux_ml_dsa.Simd.Portable.Ntt_theory
#set-options "--fuel 0 --ifuel 1 --z3rlimit 80"
open FStar.Mul
open Core_models

let _ =
  (* This module has implicit dependencies, here we make them explicit. *)
  (* The implicit dependencies arise from typeclasses instances. *)
  let open Libcrux_ml_dsa.Simd.Portable.Vector_type in
  ()

(* ============================================================================
   Hand-written companion: the F* theory formerly inlined in
   `src/simd/portable/ntt.rs` `#[hax_lib::fstar::before(...)]` blocks.
   Decls are copied BYTE-EXACT from the green extracted
   Libcrux_ml_dsa.Simd.Portable.Ntt.fst; the `#push-options` wrappers below are
   reconstructed because in the extracted module each outer push also spanned an
   IMPL fn (simd_unit_ntt_step / ntt_at_layer_0_ / ntt) and so could not travel
   with the theory.  This module is NOT generated — edit it directly.
   ========================================================================== *)

(* --- Region A: was under the `simd_unit_ntt_step` fn options push --------- *)
#push-options "--z3rlimit 300 --split_queries always"
(* Project the 32 SIMD units to the flat-chunk view the Commute.Chunk
       poly lemmas consume: chunk b = re.[b].f_values (t_Array i32 8). *)
    let chunks_of_re (re:t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32))
        : t_Array (t_Array i32 (sz 8)) (sz 32)
      = Hacspec_ml_dsa.createi #(t_Array i32 (sz 8)) (sz 32)
          #(usize -> t_Array i32 (sz 8))
          (fun (b: usize{b <. sz 32}) -> (Seq.index re (v b)).f_values)

(* Generic 1D ground->symbolic forall lift: forall32 unfolds to a 32-way
       conjunction (the driver's natural WP, exactly like the bounds post);
       pinning b to each literal lifts it to a symbolic forall. *)
    let forall32_elim_1d (r: (b: nat{b < 32}) -> Type0)
        : Lemma (requires Spec.Utils.forall32 r) (ensures forall (b: nat{b < 32}). r b)
      = let aux (b: nat{b < 32}) : Lemma (r b) =
          (match b with
           | 0 -> () | 1 -> () | 2 -> () | 3 -> () | 4 -> () | 5 -> () | 6 -> () | 7 -> ()
           | 8 -> () | 9 -> () | 10 -> () | 11 -> () | 12 -> () | 13 -> () | 14 -> () | 15 -> ()
           | 16 -> () | 17 -> () | 18 -> () | 19 -> () | 20 -> () | 21 -> () | 22 -> () | 23 -> ()
           | 24 -> () | 25 -> () | 26 -> () | 27 -> () | 28 -> () | 29 -> () | 30 -> () | _ -> ())
        in
        Classical.forall_intro aux

(* Opaque per-chunk FE atom for layer 2: the 4-pair butterfly relations as a
       GROUND 12-conjunction (matches simd_unit_ntt_at_layer_2's post exactly, so
       the round body proves it by a plain reveal).  Opaque so the driver
       composes it like the bounds post (atomic + frame), keeping the raw
       arithmetic out of the polluted 32-round WP. *)
    [@@ "opaque_to_smt"]
    let unit_fe_post_l2 (ci co: t_Array i32 (sz 8))
                        (zeta: i32{Spec.Utils.is_i32b 4190208 zeta}) : Type0 =
      (let t0 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 4) zeta in
       let t1 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 5) zeta in
       let t2 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 6) zeta in
       let t3 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 7) zeta in
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

(* Standalone: unfold one opaque FE atom to the poly lemma's per-pair forall.
       Context-free, so the reveal + 4-way p dispatch stay clean. *)
    #push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always"
    let lemma_atom_to_bf (ci co: t_Array i32 (sz 8))
                         (zeta: i32{Spec.Utils.is_i32b 4190208 zeta})
        : Lemma (requires unit_fe_post_l2 ci co zeta)
                (ensures
                  (forall (p: nat{p < 4}).
                    (let t = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci (p + 4)) zeta in
                     v (Seq.index co p)       == v (Seq.index ci p) + v t /\
                     v (Seq.index co (p + 4)) == v (Seq.index ci p) - v t /\
                     (v t) % 8380417 == (v (Seq.index ci (p + 4)) * v zeta * 8265825) % 8380417)))
      = reveal_opaque (`%unit_fe_post_l2) unit_fe_post_l2;
        introduce forall (p: nat{p < 4}).
            (let t = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci (p + 4)) zeta in
             v (Seq.index co p)       == v (Seq.index ci p) + v t /\
             v (Seq.index co (p + 4)) == v (Seq.index ci p) - v t /\
             (v t) % 8380417 == (v (Seq.index ci (p + 4)) * v zeta * 8265825) % 8380417)
        with (match p with | 0 -> () | 1 -> () | 2 -> () | _ -> ())
    #pop-options

(* Clean-context driver composition for layer 2: from the forall32 of
       opaque FE atoms (which the driver establishes lightly, like bounds),
       unfold + feed the Commute.Chunk poly lemma.  All heavy logical work
       lives here, NOT in the polluted driver body. *)
    #push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
    let lemma_l2_driver_compose
          (orig fut: t_Array (t_Array i32 (sz 8)) (sz 32))
        : Lemma
            (requires
              Spec.Utils.forall32 (fun b ->
                unit_fe_post_l2 (Seq.index orig b) (Seq.index fut b)
                                (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (b + 32)))))
            (ensures
              (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array orig in
               let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array fut in
               let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 2) in
               forall (i: nat). i < 256 ==>
                 (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
      = let zm (b: nat{b < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
          mk_i32 (Spec.MLDSA.NttConstants.zeta_r (b + 32)) in
        let t (b: nat{b < 32}) (p: nat{p < 4}) : i32 =
          Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer
            (Seq.index (Seq.index orig b) (p + 4)) (zm b) in
        forall32_elim_1d (fun b -> unit_fe_post_l2 (Seq.index orig b) (Seq.index fut b)
                                     (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (b + 32))));
        (let aux_bf (b: nat{b < 32}) : Lemma
           (forall (p: nat{p < 4}).
             (let ci = Seq.index orig b in
              let co = Seq.index fut b in
              v (Seq.index co p)       == v (Seq.index ci p) + v (t b p) /\
              v (Seq.index co (p + 4)) == v (Seq.index ci p) - v (t b p) /\
              (v (t b p)) % 8380417 == (v (Seq.index ci (p + 4)) * v (zm b) * 8265825) % 8380417))
          = lemma_atom_to_bf (Seq.index orig b) (Seq.index fut b) (zm b)
         in Classical.forall_intro aux_bf);
        (let aux_z (b: nat{b < 32}) : Lemma
           ((v (zm b)) % 8380417 ==
            (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (b + 32) ] <: i32) * pow2 32) % 8380417)
          = reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
            let _ = Spec.MLDSA.NttConstants.zeta_r (b + 32) in
            Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (b + 32)
         in Classical.forall_intro aux_z);
        Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_2_step_to_hacspec_poly orig fut t zm
    #pop-options

(* ---- Layer 1: opaque per-chunk FE atom (2 zetas/chunk, pairs (4h+j,4h+j+2)) ---- *)
    [@@ "opaque_to_smt"]
    let unit_fe_post_l1 (ci co: t_Array i32 (sz 8))
                        (zeta0 zeta1: i32{Spec.Utils.is_i32b 4190208 zeta0 /\ Spec.Utils.is_i32b 4190208 zeta1}) : Type0 =
      (let t00 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 2) zeta0 in
       let t01 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 3) zeta0 in
       let t10 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 6) zeta1 in
       let t11 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 7) zeta1 in
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

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always"
    let lemma_atom_to_bf_l1 (ci co: t_Array i32 (sz 8))
                            (zf: (h: nat{h < 2}) -> (z: i32{Spec.Utils.is_i32b 4190208 z}))
        : Lemma (requires unit_fe_post_l1 ci co (zf 0) (zf 1))
                (ensures
                  (forall (h: nat{h < 2}) (j: nat{j < 2}).
                    (let t = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci (4*h+j+2)) (zf h) in
                     v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v t /\
                     v (Seq.index co (4*h+j+2)) == v (Seq.index ci (4*h+j)) - v t /\
                     (v t) % 8380417 == (v (Seq.index ci (4*h+j+2)) * v (zf h) * 8265825) % 8380417)))
      = reveal_opaque (`%unit_fe_post_l1) unit_fe_post_l1;
        introduce forall (h: nat{h < 2}) (j: nat{j < 2}).
            (let t = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci (4*h+j+2)) (zf h) in
             v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v t /\
             v (Seq.index co (4*h+j+2)) == v (Seq.index ci (4*h+j)) - v t /\
             (v t) % 8380417 == (v (Seq.index ci (4*h+j+2)) * v (zf h) * 8265825) % 8380417)
        with (match h with | 0 -> (match j with | 0 -> () | _ -> ()) | _ -> (match j with | 0 -> () | _ -> ()))
    #pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
    let lemma_l1_driver_compose
          (orig fut: t_Array (t_Array i32 (sz 8)) (sz 32))
        : Lemma
            (requires
              Spec.Utils.forall32 (fun b ->
                unit_fe_post_l1 (Seq.index orig b) (Seq.index fut b)
                                (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (2*b + 0 + 64)))
                                (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (2*b + 1 + 64)))))
            (ensures
              (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array orig in
               let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array fut in
               let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 1) in
               forall (i: nat). i < 256 ==>
                 (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
      = let zm (b: nat{b < 32}) (h: nat{h < 2}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
          mk_i32 (Spec.MLDSA.NttConstants.zeta_r (2*b + h + 64)) in
        let t (b: nat{b < 32}) (h: nat{h < 2}) (j: nat{j < 2}) : i32 =
          Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer
            (Seq.index (Seq.index orig b) (4*h+j+2)) (zm b h) in
        forall32_elim_1d (fun b -> unit_fe_post_l1 (Seq.index orig b) (Seq.index fut b)
                                     (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (2*b + 0 + 64)))
                                     (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (2*b + 1 + 64))));
        (let aux_bf (b: nat{b < 32}) : Lemma
           (forall (h: nat{h < 2}) (j: nat{j < 2}).
             (let ci = Seq.index orig b in
              let co = Seq.index fut b in
              v (Seq.index co (4*h+j))   == v (Seq.index ci (4*h+j)) + v (t b h j) /\
              v (Seq.index co (4*h+j+2)) == v (Seq.index ci (4*h+j)) - v (t b h j) /\
              (v (t b h j)) % 8380417 == (v (Seq.index ci (4*h+j+2)) * v (zm b h) * 8265825) % 8380417))
          = lemma_atom_to_bf_l1 (Seq.index orig b) (Seq.index fut b) (fun h -> zm b h)
         in Classical.forall_intro aux_bf);
        (let aux_z (b: nat{b < 32}) (h: nat{h < 2}) : Lemma
           ((v (zm b h)) % 8380417 ==
            (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (2*b + h + 64) ] <: i32) * pow2 32) % 8380417)
          = reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
            let _ = Spec.MLDSA.NttConstants.zeta_r (2*b + h + 64) in
            Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (2*b + h + 64)
         in Classical.forall_intro_2 aux_z);
        Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_1_step_to_hacspec_poly orig fut t zm
    #pop-options

(* ---- Layer 0: opaque per-chunk FE atom (4 zetas/chunk, pairs (2p,2p+1)) ---- *)
    [@@ "opaque_to_smt"]
    let unit_fe_post_l0 (ci co: t_Array i32 (sz 8))
                        (zeta0 zeta1 zeta2 zeta3: i32{Spec.Utils.is_i32b 4190208 zeta0 /\ Spec.Utils.is_i32b 4190208 zeta1 /\ Spec.Utils.is_i32b 4190208 zeta2 /\ Spec.Utils.is_i32b 4190208 zeta3}) : Type0 =
      (let t0 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 1) zeta0 in
       let t1 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 3) zeta1 in
       let t2 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 5) zeta2 in
       let t3 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 7) zeta3 in
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

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always"
    let lemma_atom_to_bf_l0 (ci co: t_Array i32 (sz 8))
                            (zf: (p: nat{p < 4}) -> (z: i32{Spec.Utils.is_i32b 4190208 z}))
        : Lemma (requires unit_fe_post_l0 ci co (zf 0) (zf 1) (zf 2) (zf 3))
                (ensures
                  (forall (p: nat{p < 4}).
                    (let t = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci (2*p+1)) (zf p) in
                     v (Seq.index co (2*p))   == v (Seq.index ci (2*p)) + v t /\
                     v (Seq.index co (2*p+1)) == v (Seq.index ci (2*p)) - v t /\
                     (v t) % 8380417 == (v (Seq.index ci (2*p+1)) * v (zf p) * 8265825) % 8380417)))
      = reveal_opaque (`%unit_fe_post_l0) unit_fe_post_l0;
        introduce forall (p: nat{p < 4}).
            (let t = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci (2*p+1)) (zf p) in
             v (Seq.index co (2*p))   == v (Seq.index ci (2*p)) + v t /\
             v (Seq.index co (2*p+1)) == v (Seq.index ci (2*p)) - v t /\
             (v t) % 8380417 == (v (Seq.index ci (2*p+1)) * v (zf p) * 8265825) % 8380417)
        with (match p with | 0 -> () | 1 -> () | 2 -> () | _ -> ())
    #pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
    let lemma_l0_driver_compose
          (orig fut: t_Array (t_Array i32 (sz 8)) (sz 32))
        : Lemma
            (requires
              Spec.Utils.forall32 (fun b ->
                unit_fe_post_l0 (Seq.index orig b) (Seq.index fut b)
                                (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (4*b + 0 + 128)))
                                (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (4*b + 1 + 128)))
                                (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (4*b + 2 + 128)))
                                (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (4*b + 3 + 128)))))
            (ensures
              (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array orig in
               let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array fut in
               let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 0) in
               forall (i: nat). i < 256 ==>
                 (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
      = let zm (b: nat{b < 32}) (p: nat{p < 4}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
          mk_i32 (Spec.MLDSA.NttConstants.zeta_r (4*b + p + 128)) in
        let t (b: nat{b < 32}) (p: nat{p < 4}) : i32 =
          Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer
            (Seq.index (Seq.index orig b) (2*p+1)) (zm b p) in
        forall32_elim_1d (fun b -> unit_fe_post_l0 (Seq.index orig b) (Seq.index fut b)
                                     (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (4*b + 0 + 128)))
                                     (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (4*b + 1 + 128)))
                                     (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (4*b + 2 + 128)))
                                     (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (4*b + 3 + 128))));
        (let aux (b: nat{b < 32}) (p: nat{p < 4}) : Lemma
           (let ci = Seq.index orig b in
            let co = Seq.index fut b in
            v (Seq.index co (2*p))   == v (Seq.index ci (2*p)) + v (t b p) /\
            v (Seq.index co (2*p+1)) == v (Seq.index ci (2*p)) - v (t b p) /\
            (v (t b p)) % 8380417 == (v (Seq.index ci (2*p+1)) * v (zm b p) * 8265825) % 8380417 /\
            (v (zm b p)) % 8380417 ==
              (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (4*b + p + 128) ] <: i32) * pow2 32) % 8380417)
          = lemma_atom_to_bf_l0 (Seq.index orig b) (Seq.index fut b) (fun p -> zm b p);
            reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
            let _ = Spec.MLDSA.NttConstants.zeta_r (4*b + p + 128) in
            Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (4*b + p + 128)
         in Classical.forall_intro_2 aux);
        Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_0_step_to_hacspec_poly orig fut t zm
    #pop-options

[@@ "opaque_to_smt"]
let unit_fe_post_cross (ci_lo ci_hi co_lo co_hi : t_Array i32 (sz 8))
                       (zeta: i32{Spec.Utils.is_i32b 4190208 zeta}) : Type0 =
  (let t0 = Spec.MLDSA.Math.mont_mul (Seq.index ci_hi 0) zeta in
   let t1 = Spec.MLDSA.Math.mont_mul (Seq.index ci_hi 1) zeta in
   let t2 = Spec.MLDSA.Math.mont_mul (Seq.index ci_hi 2) zeta in
   let t3 = Spec.MLDSA.Math.mont_mul (Seq.index ci_hi 3) zeta in
   let t4 = Spec.MLDSA.Math.mont_mul (Seq.index ci_hi 4) zeta in
   let t5 = Spec.MLDSA.Math.mont_mul (Seq.index ci_hi 5) zeta in
   let t6 = Spec.MLDSA.Math.mont_mul (Seq.index ci_hi 6) zeta in
   let t7 = Spec.MLDSA.Math.mont_mul (Seq.index ci_hi 7) zeta in
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

(* Round-body discharge: bridge the leaf posts (add_post/sub_post are usize/Int
   foralls; the mmbc post is a nat-indexed forall over mont_mul + mod_q) into the
   ground cross atom.  add/sub need the `v (mk_usize l) == l` e-match seed (the
   mmbc nat-foralls match the literal lanes directly).  Mirrors bounded_add_post. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_round_cross_intro
    (ci_lo ci_hi co_lo co_hi tmp : t_Array i32 (sz 8))
    (zeta : i32{Spec.Utils.is_i32b 4190208 zeta})
  : Lemma
      (requires
        Libcrux_ml_dsa.Simd.Traits.Specs.add_post ci_lo tmp co_lo /\
        Libcrux_ml_dsa.Simd.Traits.Specs.sub_post ci_lo tmp co_hi /\
        (forall (i:nat). i < 8 ==>
          Seq.index tmp i == Spec.MLDSA.Math.mont_mul (Seq.index ci_hi i) zeta) /\
        (forall (i:nat). i < 8 ==>
          Spec.MLDSA.Math.mod_q (v (Seq.index tmp i)) ==
          Spec.MLDSA.Math.mod_q (v (Seq.index ci_hi i) * v zeta * 8265825)))
      (ensures unit_fe_post_cross ci_lo ci_hi co_lo co_hi zeta)
  = reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.add_post) (Libcrux_ml_dsa.Simd.Traits.Specs.add_post);
    reveal_opaque (`%Libcrux_ml_dsa.Simd.Traits.Specs.sub_post) (Libcrux_ml_dsa.Simd.Traits.Specs.sub_post);
    reveal_opaque (`%unit_fe_post_cross) unit_fe_post_cross;
    reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
    let lane (l:nat{l<8}) : Lemma
        (v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index tmp l) /\
         v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (Seq.index tmp l)) =
      assert (v (mk_usize l) == l);
      assert (v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (Seq.index tmp l));
      assert (v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (Seq.index tmp l))
    in
    lane 0; lane 1; lane 2; lane 3; lane 4; lane 5; lane 6; lane 7
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --split_queries always"
let lemma_atom_to_bf_cross (ci_lo ci_hi co_lo co_hi : t_Array i32 (sz 8))
                           (zeta: i32{Spec.Utils.is_i32b 4190208 zeta})
    : Lemma (requires unit_fe_post_cross ci_lo ci_hi co_lo co_hi zeta)
            (ensures
              (forall (l: nat{l < 8}).
                (let t = Spec.MLDSA.Math.mont_mul (Seq.index ci_hi l) zeta in
                 v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v t /\
                 v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v t /\
                 (v t) % 8380417 == (v (Seq.index ci_hi l) * v zeta * 8265825) % 8380417)))
  = reveal_opaque (`%unit_fe_post_cross) unit_fe_post_cross;
    introduce forall (l: nat{l < 8}).
        (let t = Spec.MLDSA.Math.mont_mul (Seq.index ci_hi l) zeta in
         v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v t /\
         v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v t /\
         (v t) % 8380417 == (v (Seq.index ci_hi l) * v zeta * 8265825) % 8380417)
    with (match l with | 0 -> () | 1 -> () | 2 -> () | 3 -> () | 4 -> () | 5 -> () | 6 -> () | _ -> ())
#pop-options

(* Driver compose: takes UNCHUNKED orig_re/re so the requires atoms match the
   outer_3_plus posts EXACTLY (about re.[u].f_values) — the driver discharges it by
   FRAME only (no chunks_of_re / createi at the driver, avoiding the createi_lemma
   SMTPat cascade that saturated query 674).  The chunks_of_re bridge runs HERE, in
   clean context: createi_lemma fires per-u inside aux_bf to equate
   (chunks_of_re orig_re).[u] == orig_re.[u].f_values. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_l3_cross_driver_compose
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 2 == 0) ==>
            unit_fe_post_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+1)).f_values
                               (Seq.index re u).f_values (Seq.index re (u+1)).f_values
                               (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 2 + 16)))))
        (ensures
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 3) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = chunks_of_re orig_re in
    let fut = chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 2 + 16)) in
    let t (u: nat{u < 32}) (l: nat{l < 8}) : i32 =
        Spec.MLDSA.Math.mont_mul (Seq.index (Seq.index orig ((u + 1) % 32)) l) (zm u) in
    forall32_elim_1d (fun u -> (u % 2 == 0) ==>
        unit_fe_post_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+1)).f_values
                           (Seq.index re u).f_values (Seq.index re (u+1)).f_values (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 2 == 0) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+1) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+1) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t u l) /\
          v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t u l) /\
          (v (t u l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm u) * 8265825) % 8380417))
      = if (u % 2 = 0) then begin
          Hacspec_ml_dsa.Commute.Chunk.lemma_cross_idx 1 u 0;
          FStar.Math.Lemmas.small_mod (u + 1) 32;
          // createi bridge (per-u, clean context): seed v(mk_usize _)==_ so the
          // createi_lemma SMTPat (trigger Seq.index (createi f) (v i)) fires at the nat index.
          assert (v (mk_usize u) == u);
          assert (v (mk_usize (u+1)) == u+1);
          assert (Seq.index orig u == (Seq.index orig_re u).f_values);
          assert (Seq.index orig (u+1) == (Seq.index orig_re (u+1)).f_values);
          assert (Seq.index fut u == (Seq.index re u).f_values);
          assert (Seq.index fut (u+1) == (Seq.index re (u+1)).f_values);
          lemma_atom_to_bf_cross (Seq.index orig u) (Seq.index orig (u+1))
                                 (Seq.index fut u) (Seq.index fut (u+1)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 2 == 0) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (u / 2 + 16) ] <: i32) * pow2 32) % 8380417)
      = if (u % 2 = 0) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = Spec.MLDSA.NttConstants.zeta_r (u / 2 + 16) in
          Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (u / 2 + 16)
        end
     in Classical.forall_intro aux_z);
    Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_3_cross_to_hacspec_poly orig fut t zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_l4_cross_driver_compose
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 4 < 2) ==>
            unit_fe_post_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+2)).f_values
                               (Seq.index re u).f_values (Seq.index re (u+2)).f_values
                               (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 4 + 8)))))
        (ensures
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 4) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = chunks_of_re orig_re in
    let fut = chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 4 + 8)) in
    let t (u: nat{u < 32}) (l: nat{l < 8}) : i32 =
        Spec.MLDSA.Math.mont_mul (Seq.index (Seq.index orig ((u + 2) % 32)) l) (zm u) in
    forall32_elim_1d (fun u -> (u % 4 < 2) ==>
        unit_fe_post_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+2)).f_values
                           (Seq.index re u).f_values (Seq.index re (u+2)).f_values (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 4 < 2) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+2) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+2) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t u l) /\
          v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t u l) /\
          (v (t u l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm u) * 8265825) % 8380417))
      = if (u % 4 < 2) then begin
          Hacspec_ml_dsa.Commute.Chunk.lemma_cross_idx 2 u 0;
          FStar.Math.Lemmas.small_mod (u + 2) 32;
          assert (v (mk_usize u) == u);
          assert (v (mk_usize (u+2)) == u+2);
          assert (Seq.index orig u == (Seq.index orig_re u).f_values);
          assert (Seq.index orig (u+2) == (Seq.index orig_re (u+2)).f_values);
          assert (Seq.index fut u == (Seq.index re u).f_values);
          assert (Seq.index fut (u+2) == (Seq.index re (u+2)).f_values);
          lemma_atom_to_bf_cross (Seq.index orig u) (Seq.index orig (u+2))
                                 (Seq.index fut u) (Seq.index fut (u+2)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 4 < 2) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (u / 4 + 8) ] <: i32) * pow2 32) % 8380417)
      = if (u % 4 < 2) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = Spec.MLDSA.NttConstants.zeta_r (u / 4 + 8) in
          Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (u / 4 + 8)
        end
     in Classical.forall_intro aux_z);
    Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_4_cross_to_hacspec_poly orig fut t zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_l5_cross_driver_compose
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 8 < 4) ==>
            unit_fe_post_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+4)).f_values
                               (Seq.index re u).f_values (Seq.index re (u+4)).f_values
                               (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 8 + 4)))))
        (ensures
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 5) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = chunks_of_re orig_re in
    let fut = chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 8 + 4)) in
    let t (u: nat{u < 32}) (l: nat{l < 8}) : i32 =
        Spec.MLDSA.Math.mont_mul (Seq.index (Seq.index orig ((u + 4) % 32)) l) (zm u) in
    forall32_elim_1d (fun u -> (u % 8 < 4) ==>
        unit_fe_post_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+4)).f_values
                           (Seq.index re u).f_values (Seq.index re (u+4)).f_values (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 8 < 4) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+4) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+4) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t u l) /\
          v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t u l) /\
          (v (t u l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm u) * 8265825) % 8380417))
      = if (u % 8 < 4) then begin
          Hacspec_ml_dsa.Commute.Chunk.lemma_cross_idx 4 u 0;
          FStar.Math.Lemmas.small_mod (u + 4) 32;
          assert (v (mk_usize u) == u);
          assert (v (mk_usize (u+4)) == u+4);
          assert (Seq.index orig u == (Seq.index orig_re u).f_values);
          assert (Seq.index orig (u+4) == (Seq.index orig_re (u+4)).f_values);
          assert (Seq.index fut u == (Seq.index re u).f_values);
          assert (Seq.index fut (u+4) == (Seq.index re (u+4)).f_values);
          lemma_atom_to_bf_cross (Seq.index orig u) (Seq.index orig (u+4))
                                 (Seq.index fut u) (Seq.index fut (u+4)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 8 < 4) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (u / 8 + 4) ] <: i32) * pow2 32) % 8380417)
      = if (u % 8 < 4) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = Spec.MLDSA.NttConstants.zeta_r (u / 8 + 4) in
          Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (u / 8 + 4)
        end
     in Classical.forall_intro aux_z);
    Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_5_cross_to_hacspec_poly orig fut t zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_l6_cross_driver_compose
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 16 < 8) ==>
            unit_fe_post_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+8)).f_values
                               (Seq.index re u).f_values (Seq.index re (u+8)).f_values
                               (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 16 + 2)))))
        (ensures
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 6) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = chunks_of_re orig_re in
    let fut = chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 16 + 2)) in
    let t (u: nat{u < 32}) (l: nat{l < 8}) : i32 =
        Spec.MLDSA.Math.mont_mul (Seq.index (Seq.index orig ((u + 8) % 32)) l) (zm u) in
    forall32_elim_1d (fun u -> (u % 16 < 8) ==>
        unit_fe_post_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+8)).f_values
                           (Seq.index re u).f_values (Seq.index re (u+8)).f_values (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 16 < 8) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+8) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+8) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t u l) /\
          v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t u l) /\
          (v (t u l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm u) * 8265825) % 8380417))
      = if (u % 16 < 8) then begin
          Hacspec_ml_dsa.Commute.Chunk.lemma_cross_idx 8 u 0;
          FStar.Math.Lemmas.small_mod (u + 8) 32;
          assert (v (mk_usize u) == u);
          assert (v (mk_usize (u+8)) == u+8);
          assert (Seq.index orig u == (Seq.index orig_re u).f_values);
          assert (Seq.index orig (u+8) == (Seq.index orig_re (u+8)).f_values);
          assert (Seq.index fut u == (Seq.index re u).f_values);
          assert (Seq.index fut (u+8) == (Seq.index re (u+8)).f_values);
          lemma_atom_to_bf_cross (Seq.index orig u) (Seq.index orig (u+8))
                                 (Seq.index fut u) (Seq.index fut (u+8)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 16 < 8) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (u / 16 + 2) ] <: i32) * pow2 32) % 8380417)
      = if (u % 16 < 8) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = Spec.MLDSA.NttConstants.zeta_r (u / 16 + 2) in
          Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (u / 16 + 2)
        end
     in Classical.forall_intro aux_z);
    Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_6_cross_to_hacspec_poly orig fut t zm
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_l7_cross_driver_compose
      (orig_re re: t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32))
    : Lemma
        (requires
          Spec.Utils.forall32 (fun u ->
            (u % 32 < 16) ==>
            unit_fe_post_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+16)).f_values
                               (Seq.index re u).f_values (Seq.index re (u+16)).f_values
                               (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 32 + 1)))))
        (ensures
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 7) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = chunks_of_re orig_re in
    let fut = chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r (u / 32 + 1)) in
    let t (u: nat{u < 32}) (l: nat{l < 8}) : i32 =
        Spec.MLDSA.Math.mont_mul (Seq.index (Seq.index orig ((u + 16) % 32)) l) (zm u) in
    forall32_elim_1d (fun u -> (u % 32 < 16) ==>
        unit_fe_post_cross (Seq.index orig_re u).f_values (Seq.index orig_re (u+16)).f_values
                           (Seq.index re u).f_values (Seq.index re (u+16)).f_values (zm u));
    (let aux_bf (u: nat{u < 32}) : Lemma
       (forall (l: nat{l < 8}). (u % 32 < 16) ==>
         (let ci_lo = Seq.index orig u in let ci_hi = Seq.index orig (u+16) in
          let co_lo = Seq.index fut u in let co_hi = Seq.index fut (u+16) in
          v (Seq.index co_lo l) == v (Seq.index ci_lo l) + v (t u l) /\
          v (Seq.index co_hi l) == v (Seq.index ci_lo l) - v (t u l) /\
          (v (t u l)) % 8380417 == (v (Seq.index ci_hi l) * v (zm u) * 8265825) % 8380417))
      = if (u % 32 < 16) then begin
          Hacspec_ml_dsa.Commute.Chunk.lemma_cross_idx 16 u 0;
          FStar.Math.Lemmas.small_mod (u + 16) 32;
          assert (v (mk_usize u) == u);
          assert (v (mk_usize (u+16)) == u+16);
          assert (Seq.index orig u == (Seq.index orig_re u).f_values);
          assert (Seq.index orig (u+16) == (Seq.index orig_re (u+16)).f_values);
          assert (Seq.index fut u == (Seq.index re u).f_values);
          assert (Seq.index fut (u+16) == (Seq.index re (u+16)).f_values);
          lemma_atom_to_bf_cross (Seq.index orig u) (Seq.index orig (u+16))
                                 (Seq.index fut u) (Seq.index fut (u+16)) (zm u)
        end
     in Classical.forall_intro aux_bf);
    (let aux_z (u: nat{u < 32}) : Lemma
       ((u % 32 < 16) ==>
        (v (zm u)) % 8380417 ==
        (v (Hacspec_ml_dsa.Ntt.v_ZETAS.[ mk_usize (u / 32 + 1) ] <: i32) * pow2 32) % 8380417)
      = if (u % 32 < 16) then begin
          reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q);
          let _ = Spec.MLDSA.NttConstants.zeta_r (u / 32 + 1) in
          Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (u / 32 + 1)
        end
     in Classical.forall_intro aux_z);
    Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_7_cross_to_hacspec_poly orig fut t zm
#pop-options

let simd_layer_factor (step:usize) =
    match step with
    | MkInt 1 -> 7
    | MkInt 2 -> 6
    | MkInt 4 -> 5
    | _ -> 5
#pop-options

(* --- Region B: was under the `ntt_at_layer_0_` fn options push ------------ *)
#push-options "--z3rlimit 400 --split_queries always"
let is_i32b_polynomial (b:nat) (re:t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32)) =
        Spec.Utils.forall32 (fun x -> Spec.Utils.is_i32b_array_opaque b (Seq.index re x).f_values)
#pop-options

(* --- Region C: module-default options (no enclosing push) ----------------- *)
let layer_bound_factor (step_by:usize) : n:nat{n <= 4} =
        match step_by with
        | MkInt 1 -> 4
        | MkInt 2 -> 3
        | MkInt 4 -> 2
        | MkInt 8 -> 1
        | MkInt 16 -> 0
        | _ -> 0

(* --- Region D: was under the `ntt` fn options push ------------------------ *)
(* `--z3refresh` is ADDED here (it was not on the original `ntt` fn options).
   In Ntt.fst these lemmas rode on recorded hints; a fresh companion has none, so
   they verify COLD.  Cold, `lemma_bf_even_cong`'s ensures-level no-overflow
   subtyping VC (`range (v z * v (cast y <: i64)) I64`, nonlinear) fails at
   0.879/100 with "unknown" — Z3 bailing EARLY, derailed by the query state
   accumulated over the ~700 preceding lines of theory.  The same decl under
   `--admit_except` verifies at 15.234/100, which pins the cause as accumulated
   state rather than a logic gap or a budget shortfall.  The `#restart-solver`
   below gives this region a fresh solver at the decl boundary, reproducing that
   clean state (it replaced `--z3refresh`, whose per-attempt respawn+re-transmit
   dominated wall time).  Options-only: no statement, contract, or proof is
   weakened. *)
#restart-solver
#push-options "--z3rlimit 400 --split_queries always"
let lemma_modq_eq (xa xb : i64) : Lemma
    (requires (v xa) % 8380417 == (v xb) % 8380417)
    (ensures Hacspec_ml_dsa.Arithmetic.mod_q xa == Hacspec_ml_dsa.Arithmetic.mod_q xb)
  = Hacspec_ml_dsa.Commute.Chunk.lemma_mod_q_v xa; Hacspec_ml_dsa.Commute.Chunk.lemma_mod_q_v xb

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_bf_even_cong (z: i64) (x y x' y': i32) : Lemma
    (requires (v z) >= -2147483648 /\ (v z) <= 2147483647 /\
              (v x) % 8380417 == (v x') % 8380417 /\ (v y) % 8380417 == (v y') % 8380417)
    (ensures
      Hacspec_ml_dsa.Arithmetic.mod_q ((cast x <: i64) +! (cast (Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y <: i64))) <: i64)) ==
      Hacspec_ml_dsa.Arithmetic.mod_q ((cast x' <: i64) +! (cast (Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y' <: i64))) <: i64)))
  = FStar.Math.Lemmas.lemma_mod_mul_distr_r (v z) (v y) 8380417;
    FStar.Math.Lemmas.lemma_mod_mul_distr_r (v z) (v y') 8380417;
    lemma_modq_eq (z *! (cast y <: i64)) (z *! (cast y' <: i64));
    let ta = Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y <: i64)) in
    let tb = Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y' <: i64)) in
    assert (ta == tb);
    FStar.Math.Lemmas.lemma_mod_add_distr (v (cast ta <: i64)) (v x) 8380417;
    FStar.Math.Lemmas.lemma_mod_add_distr (v (cast tb <: i64)) (v x') 8380417;
    lemma_modq_eq ((cast x <: i64) +! (cast ta <: i64)) ((cast x' <: i64) +! (cast tb <: i64))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_bf_odd_cong (z: i64) (x y x' y': i32) : Lemma
    (requires (v z) >= -2147483648 /\ (v z) <= 2147483647 /\
              (v x) % 8380417 == (v x') % 8380417 /\ (v y) % 8380417 == (v y') % 8380417)
    (ensures
      Hacspec_ml_dsa.Arithmetic.mod_q ((cast x <: i64) -! (cast (Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y <: i64))) <: i64)) ==
      Hacspec_ml_dsa.Arithmetic.mod_q ((cast x' <: i64) -! (cast (Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y' <: i64))) <: i64)))
  = FStar.Math.Lemmas.lemma_mod_mul_distr_r (v z) (v y) 8380417;
    FStar.Math.Lemmas.lemma_mod_mul_distr_r (v z) (v y') 8380417;
    lemma_modq_eq (z *! (cast y <: i64)) (z *! (cast y' <: i64));
    let ta = Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y <: i64)) in
    let tb = Hacspec_ml_dsa.Arithmetic.mod_q (z *! (cast y' <: i64)) in
    assert (ta == tb);
    FStar.Math.Lemmas.lemma_mod_sub_distr (v x) (v (cast ta <: i64)) 8380417;
    FStar.Math.Lemmas.lemma_mod_sub_distr (v x') (v (cast tb <: i64)) 8380417;
    lemma_modq_eq ((cast x <: i64) -! (cast ta <: i64)) ((cast x' <: i64) -! (cast tb <: i64))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_layer_0_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_0_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_0_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 2 in
    assert (v round < 128);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 128 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 2;
    FStar.Math.Lemmas.lemma_div_mod i 2;
    let parity : (n:nat{n < 2}) = i % 2 in
    assert (v (ii %! mk_usize 2) == parity);
    if parity < 1 then begin
      assert (ii %! mk_usize 2 <. mk_usize 1);
      assert (i + 1 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 1))
                           (Seq.index b i) (Seq.index b (i + 1))
    end else begin
      assert (~(ii %! mk_usize 2 <. mk_usize 1));
      assert (i >= 1);
      lemma_bf_odd_cong z (Seq.index a (i - 1)) (Seq.index a i)
                          (Seq.index b (i - 1)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_0_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 0) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 0))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 0)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 0)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_0_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_0_lane b ii;
      lemma_layer_0_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 0)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 0))
#pop-options

#push-options "--fuel 1 --ifuel 2 --z3rlimit 200"
let lemma_layer_1_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_1_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_1_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 4 in
    assert (v round < 64);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 64 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 4;
    FStar.Math.Lemmas.lemma_div_mod i 4;
    let parity : (n:nat{n < 4}) = i % 4 in
    assert (v (ii %! mk_usize 4) == parity);
    if parity < 2 then begin
      assert (ii %! mk_usize 4 <. mk_usize 2);
      assert (i + 2 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 2))
                           (Seq.index b i) (Seq.index b (i + 2))
    end else begin
      assert (~(ii %! mk_usize 4 <. mk_usize 2));
      assert (i >= 2);
      lemma_bf_odd_cong z (Seq.index a (i - 2)) (Seq.index a i)
                          (Seq.index b (i - 2)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_1_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 1) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 1))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 1)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 1)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_1_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_1_lane b ii;
      lemma_layer_1_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 1)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 1))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_layer_2_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_2_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_2_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 8 in
    assert (v round < 32);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 32 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 8;
    FStar.Math.Lemmas.lemma_div_mod i 8;
    let parity : (n:nat{n < 8}) = i % 8 in
    assert (v (ii %! mk_usize 8) == parity);
    if parity < 4 then begin
      assert (ii %! mk_usize 8 <. mk_usize 4);
      assert (i + 4 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 4))
                           (Seq.index b i) (Seq.index b (i + 4))
    end else begin
      assert (~(ii %! mk_usize 8 <. mk_usize 4));
      assert (i >= 4);
      lemma_bf_odd_cong z (Seq.index a (i - 4)) (Seq.index a i)
                          (Seq.index b (i - 4)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_2_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 2) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 2))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 2)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 2)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_2_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_2_lane b ii;
      lemma_layer_2_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 2)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 2))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_layer_3_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_3_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_3_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 16 in
    assert (v round < 16);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 16 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 16;
    FStar.Math.Lemmas.lemma_div_mod i 16;
    let parity : (n:nat{n < 16}) = i % 16 in
    assert (v (ii %! mk_usize 16) == parity);
    if parity < 8 then begin
      assert (ii %! mk_usize 16 <. mk_usize 8);
      assert (i + 8 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 8))
                           (Seq.index b i) (Seq.index b (i + 8))
    end else begin
      assert (~(ii %! mk_usize 16 <. mk_usize 8));
      assert (i >= 8);
      lemma_bf_odd_cong z (Seq.index a (i - 8)) (Seq.index a i)
                          (Seq.index b (i - 8)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_3_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 3) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 3))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 3)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 3)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_3_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_3_lane b ii;
      lemma_layer_3_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 3)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 3))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_layer_4_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_4_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_4_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 32 in
    assert (v round < 8);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 8 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 32;
    FStar.Math.Lemmas.lemma_div_mod i 32;
    let parity : (n:nat{n < 32}) = i % 32 in
    assert (v (ii %! mk_usize 32) == parity);
    if parity < 16 then begin
      assert (ii %! mk_usize 32 <. mk_usize 16);
      assert (i + 16 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 16))
                           (Seq.index b i) (Seq.index b (i + 16))
    end else begin
      assert (~(ii %! mk_usize 32 <. mk_usize 16));
      assert (i >= 16);
      lemma_bf_odd_cong z (Seq.index a (i - 16)) (Seq.index a i)
                          (Seq.index b (i - 16)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_4_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 4) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 4))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 4)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 4)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_4_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_4_lane b ii;
      lemma_layer_4_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 4)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 4))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_layer_5_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_5_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_5_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 64 in
    assert (v round < 4);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 4 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 64;
    FStar.Math.Lemmas.lemma_div_mod i 64;
    let parity : (n:nat{n < 64}) = i % 64 in
    assert (v (ii %! mk_usize 64) == parity);
    if parity < 32 then begin
      assert (ii %! mk_usize 64 <. mk_usize 32);
      assert (i + 32 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 32))
                           (Seq.index b i) (Seq.index b (i + 32))
    end else begin
      assert (~(ii %! mk_usize 64 <. mk_usize 32));
      assert (i >= 32);
      lemma_bf_odd_cong z (Seq.index a (i - 32)) (Seq.index a i)
                          (Seq.index b (i - 32)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_5_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 5) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 5))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 5)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 5)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_5_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_5_lane b ii;
      lemma_layer_5_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 5)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 5))
#pop-options

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
let lemma_layer_6_lane_cong (a b : t_Array i32 (mk_usize 256)) (ii : usize{v ii < 256})
    : Lemma
        (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
        (ensures Hacspec_ml_dsa.Commute.Chunk.layer_6_lane a ii == Hacspec_ml_dsa.Commute.Chunk.layer_6_lane b ii)
  = let i : nat = v ii in
    let round:usize = ii /! mk_usize 128 in
    assert (v round < 2);
    let z:i64 = cast (Hacspec_ml_dsa.Ntt.v_ZETAS.[ round +! mk_usize 2 <: usize ] <: i32) <: i64 in
    FStar.Math.Lemmas.lemma_mod_lt i 128;
    FStar.Math.Lemmas.lemma_div_mod i 128;
    let parity : (n:nat{n < 128}) = i % 128 in
    assert (v (ii %! mk_usize 128) == parity);
    if parity < 64 then begin
      assert (ii %! mk_usize 128 <. mk_usize 64);
      assert (i + 64 < 256);
      lemma_bf_even_cong z (Seq.index a i) (Seq.index a (i + 64))
                           (Seq.index b i) (Seq.index b (i + 64))
    end else begin
      assert (~(ii %! mk_usize 128 <. mk_usize 64));
      assert (i >= 64);
      lemma_bf_odd_cong z (Seq.index a (i - 64)) (Seq.index a i)
                          (Seq.index b (i - 64)) (Seq.index b i)
    end
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_layer_6_cong (a b : t_Array i32 (mk_usize 256)) : Lemma
    (requires (forall (j: nat). j < 256 ==> (v (Seq.index a j)) % 8380417 == (v (Seq.index b j)) % 8380417))
    (ensures Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 6) == Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 6))
  = let aux (i: nat{i < 256}) : Lemma
        (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 6)) i == Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 6)) i) =
      let ii:usize = mk_usize i in
      assert (v ii == i);
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_6_lane a ii;
      Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_6_lane b ii;
      lemma_layer_6_lane_cong a b ii
    in
    Classical.forall_intro aux;
    Seq.lemma_eq_intro (Hacspec_ml_dsa.Ntt.ntt_layer a (mk_usize 6)) (Hacspec_ml_dsa.Ntt.ntt_layer b (mk_usize 6))
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_ntt_compose_8 (f0 f7 f6 f5 f4 f3 f2 f1 ffinal : t_Array i32 (mk_usize 256)) : Lemma
    (requires
      (forall (i:nat). i < 256 ==> (v (Seq.index f7 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer f0 (mk_usize 7)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index f6 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer f7 (mk_usize 6)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index f5 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer f6 (mk_usize 5)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index f4 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer f5 (mk_usize 4)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index f3 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer f4 (mk_usize 3)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index f2 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer f3 (mk_usize 2)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index f1 i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer f2 (mk_usize 1)) i)) % 8380417) /\
      (forall (i:nat). i < 256 ==> (v (Seq.index ffinal i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt_layer f1 (mk_usize 0)) i)) % 8380417))
    (ensures
      (forall (i:nat). i < 256 ==> (v (Seq.index ffinal i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt f0) i)) % 8380417))
  = let g7 = Hacspec_ml_dsa.Ntt.ntt_layer f0 (mk_usize 7) in
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f7 i)) % 8380417 == (v (Seq.index g7 i)) % 8380417);
    lemma_ntt_layer_6_cong f7 g7;
    let g6 = Hacspec_ml_dsa.Ntt.ntt_layer g7 (mk_usize 6) in
    assert (Hacspec_ml_dsa.Ntt.ntt_layer f7 (mk_usize 6) == g6);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f6 i)) % 8380417 == (v (Seq.index g6 i)) % 8380417);
    lemma_ntt_layer_5_cong f6 g6;
    let g5 = Hacspec_ml_dsa.Ntt.ntt_layer g6 (mk_usize 5) in
    assert (Hacspec_ml_dsa.Ntt.ntt_layer f6 (mk_usize 5) == g5);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f5 i)) % 8380417 == (v (Seq.index g5 i)) % 8380417);
    lemma_ntt_layer_4_cong f5 g5;
    let g4 = Hacspec_ml_dsa.Ntt.ntt_layer g5 (mk_usize 4) in
    assert (Hacspec_ml_dsa.Ntt.ntt_layer f5 (mk_usize 4) == g4);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f4 i)) % 8380417 == (v (Seq.index g4 i)) % 8380417);
    lemma_ntt_layer_3_cong f4 g4;
    let g3 = Hacspec_ml_dsa.Ntt.ntt_layer g4 (mk_usize 3) in
    assert (Hacspec_ml_dsa.Ntt.ntt_layer f4 (mk_usize 3) == g3);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f3 i)) % 8380417 == (v (Seq.index g3 i)) % 8380417);
    lemma_ntt_layer_2_cong f3 g3;
    let g2 = Hacspec_ml_dsa.Ntt.ntt_layer g3 (mk_usize 2) in
    assert (Hacspec_ml_dsa.Ntt.ntt_layer f3 (mk_usize 2) == g2);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f2 i)) % 8380417 == (v (Seq.index g2 i)) % 8380417);
    lemma_ntt_layer_1_cong f2 g2;
    let g1 = Hacspec_ml_dsa.Ntt.ntt_layer g2 (mk_usize 1) in
    assert (Hacspec_ml_dsa.Ntt.ntt_layer f2 (mk_usize 1) == g1);
    assert (forall (i:nat). i < 256 ==> (v (Seq.index f1 i)) % 8380417 == (v (Seq.index g1 i)) % 8380417);
    lemma_ntt_layer_0_cong f1 g1;
    let g0 = Hacspec_ml_dsa.Ntt.ntt_layer g1 (mk_usize 0) in
    assert (Hacspec_ml_dsa.Ntt.ntt_layer f1 (mk_usize 0) == g0);
    assert (Hacspec_ml_dsa.Ntt.ntt f0 == g0)
#pop-options
#pop-options

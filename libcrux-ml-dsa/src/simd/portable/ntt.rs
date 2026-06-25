use super::arithmetic::{self, montgomery_multiply_by_constant, montgomery_multiply_fe_by_fer};
use super::vector_type::Coefficients;
use crate::simd::traits::{COEFFICIENTS_IN_SIMD_UNIT, SIMD_UNITS_IN_RING_ELEMENT};

#[cfg(hax)]
use crate::simd::traits::specs::*;

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::before(r#"
    (* Project the 32 SIMD units to the flat-chunk view the Commute.Chunk
       poly lemmas consume: chunk b = re.[b].f_values (t_Array i32 8). *)
    let chunks_of_re (re:t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32))
        : t_Array (t_Array i32 (sz 8)) (sz 32)
      = Hacspec_ml_dsa.createi #(t_Array i32 (sz 8)) (sz 32)
          #(usize -> t_Array i32 (sz 8))
          (fun (b: usize{b <. sz 32}) -> (Seq.index re (v b)).f_values)
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(
    r#"
let simd_layer_factor (step:usize) =
    match step with
    | MkInt 1 -> 7
    | MkInt 2 -> 6
    | MkInt 4 -> 5
    | _ -> 5
"#
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    1 <= v step /\ v step <= 4 /\ v index + v step < 8 /\
    Spec.Utils.is_i32b
        (v $NTT_BASE_BOUND + (simd_layer_factor $step * v $FIELD_MAX))
        (Seq.index ${simd_unit}.f_values (v $index)) /\
    Spec.Utils.is_i32b 
        (v $NTT_BASE_BOUND + (simd_layer_factor $step * v $FIELD_MAX))
        (Seq.index ${simd_unit}.f_values (v $index + v $step)) /\
    Spec.Utils.is_i32b 4190208 $zeta 
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.modifies2_8 ${simd_unit}.f_values ${simd_unit}_future.f_values index (index +! step) /\
    Spec.Utils.is_i32b
        (v $NTT_BASE_BOUND + ((simd_layer_factor $step + 1)  * v $FIELD_MAX))
        (Seq.index ${simd_unit}_future.f_values (v $index)) /\
    Spec.Utils.is_i32b
        (v $NTT_BASE_BOUND + ((simd_layer_factor $step + 1)  * v $FIELD_MAX))
        (Seq.index ${simd_unit}_future.f_values (v $index + v $step)) /\
    (let t = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer
               (Seq.index ${simd_unit}.f_values (v $index + v $step)) $zeta in
     v (Seq.index ${simd_unit}_future.f_values (v $index)) ==
       v (Seq.index ${simd_unit}.f_values (v $index)) + v t /\
     v (Seq.index ${simd_unit}_future.f_values (v $index + v $step)) ==
       v (Seq.index ${simd_unit}.f_values (v $index)) - v t /\
     (v t) % 8380417 ==
       (v (Seq.index ${simd_unit}.f_values (v $index + v $step)) * v $zeta * 8265825) % 8380417)
"#) )]
fn simd_unit_ntt_step(simd_unit: &mut Coefficients, zeta: i32, index: usize, step: usize) {
    let t = montgomery_multiply_fe_by_fer(simd_unit.values[index + step], zeta);
    simd_unit.values[index + step] = simd_unit.values[index] - t;
    simd_unit.values[index] = simd_unit.values[index] + t;
    hax_lib::fstar!(r#"reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q)"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.is_i32b_array (v $NTT_BASE_BOUND + 7 * v $FIELD_MAX) ${simd_unit}.f_values /\
    Spec.Utils.is_i32b 4190208 $zeta0 /\
    Spec.Utils.is_i32b 4190208 $zeta1 /\
    Spec.Utils.is_i32b 4190208 $zeta2 /\
    Spec.Utils.is_i32b 4190208 $zeta3
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_array (v $NTT_BASE_BOUND + 8 * v $FIELD_MAX) ${simd_unit}_future.f_values /\
    (let ci = ${simd_unit}.f_values in
     let co = ${simd_unit}_future.f_values in
     let t0 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 1) $zeta0 in
     let t1 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 3) $zeta1 in
     let t2 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 5) $zeta2 in
     let t3 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 7) $zeta3 in
     v (Seq.index co 0) == v (Seq.index ci 0) + v t0 /\
     v (Seq.index co 1) == v (Seq.index ci 0) - v t0 /\
     (v t0) % 8380417 == (v (Seq.index ci 1) * v $zeta0 * 8265825) % 8380417 /\
     v (Seq.index co 2) == v (Seq.index ci 2) + v t1 /\
     v (Seq.index co 3) == v (Seq.index ci 2) - v t1 /\
     (v t1) % 8380417 == (v (Seq.index ci 3) * v $zeta1 * 8265825) % 8380417 /\
     v (Seq.index co 4) == v (Seq.index ci 4) + v t2 /\
     v (Seq.index co 5) == v (Seq.index ci 4) - v t2 /\
     (v t2) % 8380417 == (v (Seq.index ci 5) * v $zeta2 * 8265825) % 8380417 /\
     v (Seq.index co 6) == v (Seq.index ci 6) + v t3 /\
     v (Seq.index co 7) == v (Seq.index ci 6) - v t3 /\
     (v t3) % 8380417 == (v (Seq.index ci 7) * v $zeta3 * 8265825) % 8380417)
"#) )]
pub fn simd_unit_ntt_at_layer_0(
    simd_unit: &mut Coefficients,
    zeta0: i32,
    zeta1: i32,
    zeta2: i32,
    zeta3: i32,
) {
    simd_unit_ntt_step(simd_unit, zeta0, 0, 1);
    simd_unit_ntt_step(simd_unit, zeta1, 2, 1);
    simd_unit_ntt_step(simd_unit, zeta2, 4, 1);
    simd_unit_ntt_step(simd_unit, zeta3, 6, 1);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.is_i32b_array (v $NTT_BASE_BOUND + 6 * v $FIELD_MAX) ${simd_unit}.f_values /\
    Spec.Utils.is_i32b 4190208 $zeta1 /\
    Spec.Utils.is_i32b 4190208 $zeta2
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_array (v $NTT_BASE_BOUND + 7 * v $FIELD_MAX) ${simd_unit}_future.f_values /\
    (let ci = ${simd_unit}.f_values in
     let co = ${simd_unit}_future.f_values in
     let t00 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 2) $zeta1 in
     let t01 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 3) $zeta1 in
     let t10 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 6) $zeta2 in
     let t11 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 7) $zeta2 in
     v (Seq.index co 0) == v (Seq.index ci 0) + v t00 /\
     v (Seq.index co 2) == v (Seq.index ci 0) - v t00 /\
     (v t00) % 8380417 == (v (Seq.index ci 2) * v $zeta1 * 8265825) % 8380417 /\
     v (Seq.index co 1) == v (Seq.index ci 1) + v t01 /\
     v (Seq.index co 3) == v (Seq.index ci 1) - v t01 /\
     (v t01) % 8380417 == (v (Seq.index ci 3) * v $zeta1 * 8265825) % 8380417 /\
     v (Seq.index co 4) == v (Seq.index ci 4) + v t10 /\
     v (Seq.index co 6) == v (Seq.index ci 4) - v t10 /\
     (v t10) % 8380417 == (v (Seq.index ci 6) * v $zeta2 * 8265825) % 8380417 /\
     v (Seq.index co 5) == v (Seq.index ci 5) + v t11 /\
     v (Seq.index co 7) == v (Seq.index ci 5) - v t11 /\
     (v t11) % 8380417 == (v (Seq.index ci 7) * v $zeta2 * 8265825) % 8380417)
"#) )]
pub fn simd_unit_ntt_at_layer_1(simd_unit: &mut Coefficients, zeta1: i32, zeta2: i32) {
    simd_unit_ntt_step(simd_unit, zeta1, 0, 2);
    simd_unit_ntt_step(simd_unit, zeta1, 1, 2);
    simd_unit_ntt_step(simd_unit, zeta2, 4, 2);
    simd_unit_ntt_step(simd_unit, zeta2, 5, 2);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.is_i32b_array (v $NTT_BASE_BOUND + 5 * v $FIELD_MAX) ${simd_unit}.f_values /\
    Spec.Utils.is_i32b 4190208 $zeta
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_array (v $NTT_BASE_BOUND + 6 * v $FIELD_MAX) ${simd_unit}_future.f_values /\
    (let ci = ${simd_unit}.f_values in
     let co = ${simd_unit}_future.f_values in
     let t0 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 4) $zeta in
     let t1 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 5) $zeta in
     let t2 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 6) $zeta in
     let t3 = Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer (Seq.index ci 7) $zeta in
     v (Seq.index co 0) == v (Seq.index ci 0) + v t0 /\
     v (Seq.index co 4) == v (Seq.index ci 0) - v t0 /\
     (v t0) % 8380417 == (v (Seq.index ci 4) * v $zeta * 8265825) % 8380417 /\
     v (Seq.index co 1) == v (Seq.index ci 1) + v t1 /\
     v (Seq.index co 5) == v (Seq.index ci 1) - v t1 /\
     (v t1) % 8380417 == (v (Seq.index ci 5) * v $zeta * 8265825) % 8380417 /\
     v (Seq.index co 2) == v (Seq.index ci 2) + v t2 /\
     v (Seq.index co 6) == v (Seq.index ci 2) - v t2 /\
     (v t2) % 8380417 == (v (Seq.index ci 6) * v $zeta * 8265825) % 8380417 /\
     v (Seq.index co 3) == v (Seq.index ci 3) + v t3 /\
     v (Seq.index co 7) == v (Seq.index ci 3) - v t3 /\
     (v t3) % 8380417 == (v (Seq.index ci 7) * v $zeta * 8265825) % 8380417)
"#) )]
pub fn simd_unit_ntt_at_layer_2(simd_unit: &mut Coefficients, zeta: i32) {
    simd_unit_ntt_step(simd_unit, zeta, 0, 4);
    simd_unit_ntt_step(simd_unit, zeta, 1, 4);
    simd_unit_ntt_step(simd_unit, zeta, 2, 4);
    simd_unit_ntt_step(simd_unit, zeta, 3, 4);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"
    let is_i32b_polynomial (b:nat) (re:t_Array Libcrux_ml_dsa.Simd.Portable.Vector_type.t_Coefficients (sz 32)) =
        Spec.Utils.forall32 (fun x -> Spec.Utils.is_i32b_array_opaque b (Seq.index re x).f_values)
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 7 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 8 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 0) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn ntt_at_layer_0(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[inline(always)]
    #[hax_lib::fstar::options("--z3rlimit 100")]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        v index < v $SIMD_UNITS_IN_RING_ELEMENT /\
        Spec.Utils.is_i32b_array_opaque (v $NTT_BASE_BOUND + 7 * v $FIELD_MAX) 
            (Seq.index ${re} (v index)).f_values /\
        Spec.Utils.is_i32b 4190208 $zeta_0 /\
        Spec.Utils.is_i32b 4190208 $zeta_1 /\
        Spec.Utils.is_i32b 4190208 $zeta_2 /\
        Spec.Utils.is_i32b 4190208 $zeta_3
    "#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.modifies1_32 ${re} ${re}_future $index /\
        Spec.Utils.is_i32b_array_opaque (v $NTT_BASE_BOUND + 8 * v $FIELD_MAX)
            (Seq.index ${re}_future (v index)).f_values /\
        unit_fe_post_l0 (Seq.index ${re} (v $index)).f_values
                        (Seq.index ${re}_future (v $index)).f_values
                        $zeta_0 $zeta_1 $zeta_2 $zeta_3
     "#))]
    fn round(
        re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT],
        index: usize,
        zeta_0: i32,
        zeta_1: i32,
        zeta_2: i32,
        zeta_3: i32,
    ) {
        hax_lib::fstar!(
            "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
        );

        simd_unit_ntt_at_layer_0(&mut re[index], zeta_0, zeta_1, zeta_2, zeta_3);
        hax_lib::fstar!("reveal_opaque (`%unit_fe_post_l0) unit_fe_post_l0");
    }

    #[cfg(hax)]
    let orig_re = re.clone();

    round(re, 0, 2091667, 3407706, 2316500, 3817976);
    round(re, 1, -3342478, 2244091, -2446433, -3562462);
    round(re, 2, 266997, 2434439, -1235728, 3513181);
    round(re, 3, -3520352, -3759364, -1197226, -3193378);
    round(re, 4, 900702, 1859098, 909542, 819034);
    round(re, 5, 495491, -1613174, -43260, -522500);
    round(re, 6, -655327, -3122442, 2031748, 3207046);
    round(re, 7, -3556995, -525098, -768622, -3595838);
    round(re, 8, 342297, 286988, -2437823, 4108315);
    round(re, 9, 3437287, -3342277, 1735879, 203044);
    round(re, 10, 2842341, 2691481, -2590150, 1265009);
    round(re, 11, 4055324, 1247620, 2486353, 1595974);
    round(re, 12, -3767016, 1250494, 2635921, -3548272);
    round(re, 13, -2994039, 1869119, 1903435, -1050970);
    round(re, 14, -1333058, 1237275, -3318210, -1430225);
    round(re, 15, -451100, 1312455, 3306115, -1962642);
    round(re, 16, -1279661, 1917081, -2546312, -1374803);
    round(re, 17, 1500165, 777191, 2235880, 3406031);
    round(re, 18, -542412, -2831860, -1671176, -1846953);
    round(re, 19, -2584293, -3724270, 594136, -3776993);
    round(re, 20, -2013608, 2432395, 2454455, -164721);
    round(re, 21, 1957272, 3369112, 185531, -1207385);
    round(re, 22, -3183426, 162844, 1616392, 3014001);
    round(re, 23, 810149, 1652634, -3694233, -1799107);
    round(re, 24, -3038916, 3523897, 3866901, 269760);
    round(re, 25, 2213111, -975884, 1717735, 472078);
    round(re, 26, -426683, 1723600, -1803090, 1910376);
    round(re, 27, -1667432, -1104333, -260646, -3833893);
    round(re, 28, -2939036, -2235985, -420899, -2286327);
    round(re, 29, 183443, -976891, 1612842, -3545687);
    round(re, 30, -554416, 3919660, -48306, -1362209);
    round(re, 31, 3937738, 1400424, -846154, 1976782);

    hax_lib::fstar!(r#"
assert_norm (Spec.MLDSA.NttConstants.zeta_r 128 == 2091667);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 129 == 3407706);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 130 == 2316500);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 131 == 3817976);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 132 == (-3342478));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 133 == 2244091);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 134 == (-2446433));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 135 == (-3562462));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 136 == 266997);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 137 == 2434439);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 138 == (-1235728));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 139 == 3513181);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 140 == (-3520352));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 141 == (-3759364));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 142 == (-1197226));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 143 == (-3193378));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 144 == 900702);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 145 == 1859098);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 146 == 909542);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 147 == 819034);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 148 == 495491);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 149 == (-1613174));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 150 == (-43260));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 151 == (-522500));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 152 == (-655327));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 153 == (-3122442));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 154 == 2031748);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 155 == 3207046);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 156 == (-3556995));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 157 == (-525098));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 158 == (-768622));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 159 == (-3595838));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 160 == 342297);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 161 == 286988);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 162 == (-2437823));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 163 == 4108315);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 164 == 3437287);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 165 == (-3342277));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 166 == 1735879);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 167 == 203044);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 168 == 2842341);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 169 == 2691481);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 170 == (-2590150));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 171 == 1265009);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 172 == 4055324);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 173 == 1247620);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 174 == 2486353);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 175 == 1595974);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 176 == (-3767016));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 177 == 1250494);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 178 == 2635921);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 179 == (-3548272));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 180 == (-2994039));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 181 == 1869119);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 182 == 1903435);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 183 == (-1050970));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 184 == (-1333058));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 185 == 1237275);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 186 == (-3318210));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 187 == (-1430225));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 188 == (-451100));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 189 == 1312455);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 190 == 3306115);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 191 == (-1962642));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 192 == (-1279661));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 193 == 1917081);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 194 == (-2546312));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 195 == (-1374803));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 196 == 1500165);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 197 == 777191);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 198 == 2235880);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 199 == 3406031);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 200 == (-542412));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 201 == (-2831860));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 202 == (-1671176));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 203 == (-1846953));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 204 == (-2584293));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 205 == (-3724270));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 206 == 594136);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 207 == (-3776993));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 208 == (-2013608));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 209 == 2432395);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 210 == 2454455);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 211 == (-164721));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 212 == 1957272);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 213 == 3369112);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 214 == 185531);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 215 == (-1207385));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 216 == (-3183426));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 217 == 162844);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 218 == 1616392);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 219 == 3014001);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 220 == 810149);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 221 == 1652634);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 222 == (-3694233));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 223 == (-1799107));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 224 == (-3038916));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 225 == 3523897);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 226 == 3866901);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 227 == 269760);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 228 == 2213111);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 229 == (-975884));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 230 == 1717735);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 231 == 472078);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 232 == (-426683));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 233 == 1723600);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 234 == (-1803090));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 235 == 1910376);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 236 == (-1667432));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 237 == (-1104333));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 238 == (-260646));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 239 == (-3833893));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 240 == (-2939036));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 241 == (-2235985));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 242 == (-420899));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 243 == (-2286327));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 244 == 183443);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 245 == (-976891));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 246 == 1612842);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 247 == (-3545687));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 248 == (-554416));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 249 == 3919660);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 250 == (-48306));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 251 == (-1362209));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 252 == 3937738);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 253 == 1400424);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 254 == (-846154));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 255 == 1976782);
lemma_l0_driver_compose (chunks_of_re ${orig_re}) (chunks_of_re ${re})
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 6 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 7 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 1) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn ntt_at_layer_1(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[inline(always)]
    #[hax_lib::fstar::options("--z3rlimit 100")]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        v index < v $SIMD_UNITS_IN_RING_ELEMENT /\
        Spec.Utils.is_i32b_array_opaque (v $NTT_BASE_BOUND + 6 * v $FIELD_MAX)
                                 (Seq.index ${re} (v index)).f_values /\
        Spec.Utils.is_i32b 4190208 $zeta_0 /\
        Spec.Utils.is_i32b 4190208 $zeta_1
    "#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.modifies1_32 ${re} ${re}_future $index /\
        Spec.Utils.is_i32b_array_opaque (v $NTT_BASE_BOUND + 7 * v $FIELD_MAX)
            (Seq.index ${re}_future (v index)).f_values /\
        unit_fe_post_l1 (Seq.index ${re} (v $index)).f_values
                        (Seq.index ${re}_future (v $index)).f_values $zeta_0 $zeta_1
     "#))]
    fn round(
        re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT],
        index: usize,
        zeta_0: i32,
        zeta_1: i32,
    ) {
        hax_lib::fstar!(
            "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
        );

        simd_unit_ntt_at_layer_1(&mut re[index], zeta_0, zeta_1);
        hax_lib::fstar!("reveal_opaque (`%unit_fe_post_l1) unit_fe_post_l1");
    }

    #[cfg(hax)]
    let orig_re = re.clone();

    round(re, 0, -3930395, -1528703);
    round(re, 1, -3677745, -3041255);
    round(re, 2, -1452451, 3475950);
    round(re, 3, 2176455, -1585221);
    round(re, 4, -1257611, 1939314);
    round(re, 5, -4083598, -1000202);
    round(re, 6, -3190144, -3157330);
    round(re, 7, -3632928, 126922);
    round(re, 8, 3412210, -983419);
    round(re, 9, 2147896, 2715295);
    round(re, 10, -2967645, -3693493);
    round(re, 11, -411027, -2477047);
    round(re, 12, -671102, -1228525);
    round(re, 13, -22981, -1308169);
    round(re, 14, -381987, 1349076);
    round(re, 15, 1852771, -1430430);
    round(re, 16, -3343383, 264944);
    round(re, 17, 508951, 3097992);
    round(re, 18, 44288, -1100098);
    round(re, 19, 904516, 3958618);
    round(re, 20, -3724342, -8578);
    round(re, 21, 1653064, -3249728);
    round(re, 22, 2389356, -210977);
    round(re, 23, 759969, -1316856);
    round(re, 24, 189548, -3553272);
    round(re, 25, 3159746, -1851402);
    round(re, 26, -2409325, -177440);
    round(re, 27, 1315589, 1341330);
    round(re, 28, 1285669, -1584928);
    round(re, 29, -812732, -1439742);
    round(re, 30, -3019102, -3881060);
    round(re, 31, -3628969, 3839961);

    hax_lib::fstar!(r#"
assert_norm (Spec.MLDSA.NttConstants.zeta_r 64 == (-3930395));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 65 == (-1528703));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 66 == (-3677745));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 67 == (-3041255));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 68 == (-1452451));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 69 == 3475950);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 70 == 2176455);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 71 == (-1585221));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 72 == (-1257611));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 73 == 1939314);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 74 == (-4083598));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 75 == (-1000202));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 76 == (-3190144));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 77 == (-3157330));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 78 == (-3632928));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 79 == 126922);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 80 == 3412210);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 81 == (-983419));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 82 == 2147896);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 83 == 2715295);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 84 == (-2967645));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 85 == (-3693493));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 86 == (-411027));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 87 == (-2477047));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 88 == (-671102));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 89 == (-1228525));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 90 == (-22981));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 91 == (-1308169));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 92 == (-381987));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 93 == 1349076);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 94 == 1852771);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 95 == (-1430430));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 96 == (-3343383));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 97 == 264944);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 98 == 508951);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 99 == 3097992);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 100 == 44288);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 101 == (-1100098));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 102 == 904516);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 103 == 3958618);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 104 == (-3724342));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 105 == (-8578));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 106 == 1653064);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 107 == (-3249728));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 108 == 2389356);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 109 == (-210977));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 110 == 759969);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 111 == (-1316856));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 112 == 189548);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 113 == (-3553272));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 114 == 3159746);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 115 == (-1851402));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 116 == (-2409325));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 117 == (-177440));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 118 == 1315589);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 119 == 1341330);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 120 == 1285669);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 121 == (-1584928));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 122 == (-812732));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 123 == (-1439742));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 124 == (-3019102));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 125 == (-3881060));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 126 == (-3628969));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 127 == 3839961);
lemma_l1_driver_compose (chunks_of_re ${orig_re}) (chunks_of_re ${re})
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 5 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 6 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 2) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn ntt_at_layer_2(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[inline(always)]
    #[hax_lib::fstar::options("--z3rlimit 200")]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        v index < v $SIMD_UNITS_IN_RING_ELEMENT /\
        Spec.Utils.is_i32b_array_opaque (v $NTT_BASE_BOUND + 5 * v $FIELD_MAX) 
                                        (Seq.index ${re} (v index)).f_values /\
        Spec.Utils.is_i32b 4190208 $zeta
    "#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.modifies1_32 ${re} ${re}_future $index /\
        Spec.Utils.is_i32b_array_opaque (v $NTT_BASE_BOUND + 6 * v $FIELD_MAX)
            (Seq.index ${re}_future (v index)).f_values /\
        unit_fe_post_l2 (Seq.index ${re} (v $index)).f_values
                        (Seq.index ${re}_future (v $index)).f_values $zeta
    "#))]
    fn round(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT], index: usize, zeta: i32) {
        hax_lib::fstar!(
            "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
        );

        simd_unit_ntt_at_layer_2(&mut re[index], zeta);
        hax_lib::fstar!("reveal_opaque (`%unit_fe_post_l2) unit_fe_post_l2");
    }

    #[cfg(hax)]
    let orig_re = re.clone();

    round(re, 0, 2706023);
    round(re, 1, 95776);
    round(re, 2, 3077325);
    round(re, 3, 3530437);
    round(re, 4, -1661693);
    round(re, 5, -3592148);
    round(re, 6, -2537516);
    round(re, 7, 3915439);
    round(re, 8, -3861115);
    round(re, 9, -3043716);
    round(re, 10, 3574422);
    round(re, 11, -2867647);
    round(re, 12, 3539968);
    round(re, 13, -300467);
    round(re, 14, 2348700);
    round(re, 15, -539299);
    round(re, 16, -1699267);
    round(re, 17, -1643818);
    round(re, 18, 3505694);
    round(re, 19, -3821735);
    round(re, 20, 3507263);
    round(re, 21, -2140649);
    round(re, 22, -1600420);
    round(re, 23, 3699596);
    round(re, 24, 811944);
    round(re, 25, 531354);
    round(re, 26, 954230);
    round(re, 27, 3881043);
    round(re, 28, 3900724);
    round(re, 29, -2556880);
    round(re, 30, 2071892);
    round(re, 31, -2797779);

    hax_lib::fstar!(r#"
assert_norm (Spec.MLDSA.NttConstants.zeta_r 32 == 2706023);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 33 == 95776);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 34 == 3077325);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 35 == 3530437);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 36 == (-1661693));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 37 == (-3592148));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 38 == (-2537516));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 39 == 3915439);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 40 == (-3861115));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 41 == (-3043716));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 42 == 3574422);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 43 == (-2867647));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 44 == 3539968);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 45 == (-300467));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 46 == 2348700);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 47 == (-539299));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 48 == (-1699267));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 49 == (-1643818));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 50 == 3505694);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 51 == (-3821735));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 52 == 3507263);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 53 == (-2140649));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 54 == (-1600420));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 55 == 3699596);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 56 == 811944);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 57 == 531354);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 58 == 954230);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 59 == 3881043);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 60 == 3900724);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 61 == (-2556880));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 62 == 2071892);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 63 == (-2797779));
lemma_l2_driver_compose (chunks_of_re ${orig_re}) (chunks_of_re ${re})
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 600 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    (v $STEP_BY > 0) /\
    (v $OFFSET + v $STEP_BY < v $SIMD_UNITS_IN_RING_ELEMENT) /\
    (v $OFFSET + 2 * v $STEP_BY <= v $SIMD_UNITS_IN_RING_ELEMENT) /\
    (Spec.Utils.forall32 (fun i -> (i >= v $OFFSET /\ i < (v $OFFSET + 2 * v $STEP_BY)) ==>
              Spec.Utils.is_i32b_array_opaque 
                (v $NTT_BASE_BOUND + ((layer_bound_factor $STEP_BY) * v $FIELD_MAX)) 
                (Seq.index ${re} i).f_values)) /\
    Spec.Utils.is_i32b 4190208 $ZETA
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.modifies_range_32 ${re} ${re}_future $OFFSET (${OFFSET + STEP_BY + STEP_BY}) /\
    (Spec.Utils.forall32 (fun i -> (i >= v $OFFSET /\ i < (v $OFFSET + 2 * v $STEP_BY)) ==>
              Spec.Utils.is_i32b_array_opaque
                (v $NTT_BASE_BOUND + ((layer_bound_factor $STEP_BY + 1) * v $FIELD_MAX))
                (Seq.index ${re}_future i).f_values)) /\
    (Spec.Utils.forall32 (fun u -> (u >= v $OFFSET /\ u < v $OFFSET + v $STEP_BY) ==>
              unit_fe_post_cross (Seq.index ${re} u).f_values
                                 (Seq.index ${re} (u + v $STEP_BY)).f_values
                                 (Seq.index ${re}_future u).f_values
                                 (Seq.index ${re}_future (u + v $STEP_BY)).f_values
                                 $ZETA))
"#))]
fn outer_3_plus<const OFFSET: usize, const STEP_BY: usize, const ZETA: i32>(
    re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT],
) {
    // Refactoring the code to have the loop body separately verified is good for proof performance.
    // So we factor out the loop body in a `round` function similarly to the other NTT layers.
    #[inline(always)]
    #[hax_lib::fstar::before(
        r#"
    let layer_bound_factor (step_by:usize) : n:nat{n <= 4} =
        match step_by with
        | MkInt 1 -> 4
        | MkInt 2 -> 3
        | MkInt 4 -> 2
        | MkInt 8 -> 1
        | MkInt 16 -> 0
        | _ -> 0
    "#
    )]
    #[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        v $step_by > 0 /\
        v $index + v $step_by < v $SIMD_UNITS_IN_RING_ELEMENT /\
        Spec.Utils.is_i32b_array_opaque 
                    (v $NTT_BASE_BOUND + ((layer_bound_factor $step_by) * v $FIELD_MAX)) 
                    (Seq.index ${re} (v $index)).f_values /\
        Spec.Utils.is_i32b_array_opaque 
                    (v $NTT_BASE_BOUND + ((layer_bound_factor $step_by) * v $FIELD_MAX)) 
                    (Seq.index ${re} (v $index + v $step_by)).f_values /\
        Spec.Utils.is_i32b 4190208 $zeta
    "#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.modifies2_32 ${re} ${re}_future $index (${index + step_by}) /\
        Spec.Utils.is_i32b_array_opaque
                    (v $NTT_BASE_BOUND + ((layer_bound_factor $step_by + 1) * v $FIELD_MAX))
                    (Seq.index ${re}_future (v $index)).f_values /\
        Spec.Utils.is_i32b_array_opaque
                    (v $NTT_BASE_BOUND + ((layer_bound_factor $step_by + 1) * v $FIELD_MAX))
                    (Seq.index ${re}_future (v $index + v step_by)).f_values /\
        unit_fe_post_cross (Seq.index ${re} (v $index)).f_values
                           (Seq.index ${re} (v $index + v $step_by)).f_values
                           (Seq.index ${re}_future (v $index)).f_values
                           (Seq.index ${re}_future (v $index + v $step_by)).f_values
                           $zeta
    "#))]
    fn round(
        re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT],
        index: usize,
        step_by: usize,
        zeta: i32,
    ) {
        hax_lib::fstar!(
            "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
        );
        #[cfg(hax)]
        let re_in = re.clone();
        let mut tmp = re[index + step_by];
        montgomery_multiply_by_constant(&mut tmp, zeta);

        re[index + step_by] = re[index];

        arithmetic::subtract(&mut re[index + step_by], &tmp);
        arithmetic::add(&mut re[index], &tmp);
        // Discharge the cross-unit FE atom via the clean bridge lemma: ci_lo/ci_hi
        // = re_in[index]/[index+step_by]; co_lo/co_hi = re[index]/[index+step_by];
        // tmp = mmbc(ci_hi, zeta).  add/sub posts + mmbc post satisfy its requires.
        hax_lib::fstar!(
            r#"lemma_round_cross_intro
                 (Seq.index $re_in (v $index)).f_values
                 (Seq.index $re_in (v $index + v $step_by)).f_values
                 (Seq.index ${re} (v $index)).f_values
                 (Seq.index ${re} (v $index + v $step_by)).f_values
                 ${tmp}.f_values
                 $zeta"#
        );
    }

    #[cfg(hax)]
    let orig_re = re.clone();

    for j in OFFSET..OFFSET + STEP_BY {
        hax_lib::loop_invariant!(|j: usize| fstar!(
            r#"
            (Spec.Utils.modifies_range2_32 $orig_re $re
                $OFFSET $j ($OFFSET +! $STEP_BY) ($j +! $STEP_BY)) /\
            (Spec.Utils.forall32 (fun i -> ((i >= v $OFFSET /\ i < v $j) \/
                        (i >= v $OFFSET + v $STEP_BY /\ i < v $j + v $STEP_BY)) ==>
                Spec.Utils.is_i32b_array_opaque
                    (v $NTT_BASE_BOUND + ((layer_bound_factor $STEP_BY + 1) * v $FIELD_MAX))
                    (Seq.index ${re} i).f_values)) /\
            (Spec.Utils.forall32 (fun u -> (u >= v $OFFSET /\ u < v $j) ==>
                unit_fe_post_cross (Seq.index $orig_re u).f_values
                                   (Seq.index $orig_re (u + v $STEP_BY)).f_values
                                   (Seq.index ${re} u).f_values
                                   (Seq.index ${re} (u + v $STEP_BY)).f_values
                                   $ZETA))
        "#
        ));
        round(re, j, STEP_BY, ZETA);
    }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 4 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 5 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 3) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn ntt_at_layer_3(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 8; // 1 << LAYER;
    const STEP_BY: usize = 1; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2725464>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1024112>(re);
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1079900>(re);
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3585928>(re);
    outer_3_plus::<{ (4 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -549488>(re);
    outer_3_plus::<{ (5 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1119584>(re);
    outer_3_plus::<{ (6 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2619752>(re);
    outer_3_plus::<{ (7 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2108549>(re);
    outer_3_plus::<{ (8 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2118186>(re);
    outer_3_plus::<{ (9 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -3859737>(re);
    outer_3_plus::<{ (10 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1399561>(re);
    outer_3_plus::<{ (11 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -3277672>(re);
    outer_3_plus::<{ (12 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1757237>(re);
    outer_3_plus::<{ (13 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -19422>(re);
    outer_3_plus::<{ (14 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 4010497>(re);
    outer_3_plus::<{ (15 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 280005>(re);

    hax_lib::fstar!(r#"
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
// Flat-asserts: discharge each even-u cross atom (orig_re vs final re) in
// ISOLATION (one frame each, ~rlimit 11) so the compose lemma's forall32
// precondition is assembled from 16 ground facts instead of a
// forall32-of-forall32 e-matching cascade (mirrors the A3 flat composition).
assert (unit_fe_post_cross (Seq.index ${orig_re} 0).f_values (Seq.index ${orig_re} 1).f_values (Seq.index ${re} 0).f_values (Seq.index ${re} 1).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (0 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 2).f_values (Seq.index ${orig_re} 3).f_values (Seq.index ${re} 2).f_values (Seq.index ${re} 3).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (2 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 4).f_values (Seq.index ${orig_re} 5).f_values (Seq.index ${re} 4).f_values (Seq.index ${re} 5).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (4 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 6).f_values (Seq.index ${orig_re} 7).f_values (Seq.index ${re} 6).f_values (Seq.index ${re} 7).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (6 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 8).f_values (Seq.index ${orig_re} 9).f_values (Seq.index ${re} 8).f_values (Seq.index ${re} 9).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (8 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 10).f_values (Seq.index ${orig_re} 11).f_values (Seq.index ${re} 10).f_values (Seq.index ${re} 11).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (10 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 12).f_values (Seq.index ${orig_re} 13).f_values (Seq.index ${re} 12).f_values (Seq.index ${re} 13).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (12 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 14).f_values (Seq.index ${orig_re} 15).f_values (Seq.index ${re} 14).f_values (Seq.index ${re} 15).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (14 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 16).f_values (Seq.index ${orig_re} 17).f_values (Seq.index ${re} 16).f_values (Seq.index ${re} 17).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (16 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 18).f_values (Seq.index ${orig_re} 19).f_values (Seq.index ${re} 18).f_values (Seq.index ${re} 19).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (18 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 20).f_values (Seq.index ${orig_re} 21).f_values (Seq.index ${re} 20).f_values (Seq.index ${re} 21).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (20 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 22).f_values (Seq.index ${orig_re} 23).f_values (Seq.index ${re} 22).f_values (Seq.index ${re} 23).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (22 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 24).f_values (Seq.index ${orig_re} 25).f_values (Seq.index ${re} 24).f_values (Seq.index ${re} 25).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (24 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 26).f_values (Seq.index ${orig_re} 27).f_values (Seq.index ${re} 26).f_values (Seq.index ${re} 27).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (26 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 28).f_values (Seq.index ${orig_re} 29).f_values (Seq.index ${re} 28).f_values (Seq.index ${re} 29).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (28 / 2 + 16))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 30).f_values (Seq.index ${orig_re} 31).f_values (Seq.index ${re} 30).f_values (Seq.index ${re} 31).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (30 / 2 + 16))));
lemma_l3_cross_driver_compose ${orig_re} ${re}
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 3 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 4 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 4) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn ntt_at_layer_4(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 16; // 1 << LAYER;
    const STEP_BY: usize = 2; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1826347>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2353451>(re);
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -359251>(re);
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2091905>(re);
    outer_3_plus::<{ (4 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3119733>(re);
    outer_3_plus::<{ (5 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2884855>(re);
    outer_3_plus::<{ (6 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3111497>(re);
    outer_3_plus::<{ (7 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2680103>(re);

    hax_lib::fstar!(r#"
assert_norm (Spec.MLDSA.NttConstants.zeta_r 8 == 1826347);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 9 == 2353451);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 10 == (-359251));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 11 == (-2091905));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 12 == 3119733);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 13 == (-2884855));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 14 == 3111497);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 15 == 2680103);
assert (unit_fe_post_cross (Seq.index ${orig_re} 0).f_values (Seq.index ${orig_re} 2).f_values (Seq.index ${re} 0).f_values (Seq.index ${re} 2).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (0 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 1).f_values (Seq.index ${orig_re} 3).f_values (Seq.index ${re} 1).f_values (Seq.index ${re} 3).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (1 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 4).f_values (Seq.index ${orig_re} 6).f_values (Seq.index ${re} 4).f_values (Seq.index ${re} 6).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (4 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 5).f_values (Seq.index ${orig_re} 7).f_values (Seq.index ${re} 5).f_values (Seq.index ${re} 7).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (5 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 8).f_values (Seq.index ${orig_re} 10).f_values (Seq.index ${re} 8).f_values (Seq.index ${re} 10).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (8 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 9).f_values (Seq.index ${orig_re} 11).f_values (Seq.index ${re} 9).f_values (Seq.index ${re} 11).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (9 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 12).f_values (Seq.index ${orig_re} 14).f_values (Seq.index ${re} 12).f_values (Seq.index ${re} 14).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (12 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 13).f_values (Seq.index ${orig_re} 15).f_values (Seq.index ${re} 13).f_values (Seq.index ${re} 15).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (13 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 16).f_values (Seq.index ${orig_re} 18).f_values (Seq.index ${re} 16).f_values (Seq.index ${re} 18).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (16 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 17).f_values (Seq.index ${orig_re} 19).f_values (Seq.index ${re} 17).f_values (Seq.index ${re} 19).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (17 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 20).f_values (Seq.index ${orig_re} 22).f_values (Seq.index ${re} 20).f_values (Seq.index ${re} 22).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (20 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 21).f_values (Seq.index ${orig_re} 23).f_values (Seq.index ${re} 21).f_values (Seq.index ${re} 23).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (21 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 24).f_values (Seq.index ${orig_re} 26).f_values (Seq.index ${re} 24).f_values (Seq.index ${re} 26).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (24 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 25).f_values (Seq.index ${orig_re} 27).f_values (Seq.index ${re} 25).f_values (Seq.index ${re} 27).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (25 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 28).f_values (Seq.index ${orig_re} 30).f_values (Seq.index ${re} 28).f_values (Seq.index ${re} 30).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (28 / 4 + 8))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 29).f_values (Seq.index ${orig_re} 31).f_values (Seq.index ${re} 29).f_values (Seq.index ${re} 31).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (29 / 4 + 8))));
lemma_l4_cross_driver_compose ${orig_re} ${re}
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 2 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 3 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 5) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn ntt_at_layer_5(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 32; // 1 << LAYER;
    const STEP_BY: usize = 4; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 237124>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -777960>(re);
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -876248>(re);
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 466468>(re);

    hax_lib::fstar!(r#"
assert_norm (Spec.MLDSA.NttConstants.zeta_r 4 == 237124);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 5 == (-777960));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 6 == (-876248));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 7 == 466468);
assert (unit_fe_post_cross (Seq.index ${orig_re} 0).f_values (Seq.index ${orig_re} 4).f_values (Seq.index ${re} 0).f_values (Seq.index ${re} 4).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (0 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 1).f_values (Seq.index ${orig_re} 5).f_values (Seq.index ${re} 1).f_values (Seq.index ${re} 5).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (1 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 2).f_values (Seq.index ${orig_re} 6).f_values (Seq.index ${re} 2).f_values (Seq.index ${re} 6).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (2 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 3).f_values (Seq.index ${orig_re} 7).f_values (Seq.index ${re} 3).f_values (Seq.index ${re} 7).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (3 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 8).f_values (Seq.index ${orig_re} 12).f_values (Seq.index ${re} 8).f_values (Seq.index ${re} 12).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (8 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 9).f_values (Seq.index ${orig_re} 13).f_values (Seq.index ${re} 9).f_values (Seq.index ${re} 13).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (9 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 10).f_values (Seq.index ${orig_re} 14).f_values (Seq.index ${re} 10).f_values (Seq.index ${re} 14).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (10 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 11).f_values (Seq.index ${orig_re} 15).f_values (Seq.index ${re} 11).f_values (Seq.index ${re} 15).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (11 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 16).f_values (Seq.index ${orig_re} 20).f_values (Seq.index ${re} 16).f_values (Seq.index ${re} 20).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (16 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 17).f_values (Seq.index ${orig_re} 21).f_values (Seq.index ${re} 17).f_values (Seq.index ${re} 21).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (17 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 18).f_values (Seq.index ${orig_re} 22).f_values (Seq.index ${re} 18).f_values (Seq.index ${re} 22).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (18 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 19).f_values (Seq.index ${orig_re} 23).f_values (Seq.index ${re} 19).f_values (Seq.index ${re} 23).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (19 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 24).f_values (Seq.index ${orig_re} 28).f_values (Seq.index ${re} 24).f_values (Seq.index ${re} 28).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (24 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 25).f_values (Seq.index ${orig_re} 29).f_values (Seq.index ${re} 25).f_values (Seq.index ${re} 29).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (25 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 26).f_values (Seq.index ${orig_re} 30).f_values (Seq.index ${re} 26).f_values (Seq.index ${re} 30).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (26 / 8 + 4))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 27).f_values (Seq.index ${orig_re} 31).f_values (Seq.index ${re} 27).f_values (Seq.index ${re} 31).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (27 / 8 + 4))));
lemma_l5_cross_driver_compose ${orig_re} ${re}
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 1 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 2 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 6) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn ntt_at_layer_6(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 64; // 1 << LAYER;
    const STEP_BY: usize = 8; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2608894>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -518909>(re);

    hax_lib::fstar!(r#"
assert_norm (Spec.MLDSA.NttConstants.zeta_r 2 == (-2608894));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 3 == (-518909));
assert (unit_fe_post_cross (Seq.index ${orig_re} 0).f_values (Seq.index ${orig_re} 8).f_values (Seq.index ${re} 0).f_values (Seq.index ${re} 8).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (0 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 1).f_values (Seq.index ${orig_re} 9).f_values (Seq.index ${re} 1).f_values (Seq.index ${re} 9).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (1 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 2).f_values (Seq.index ${orig_re} 10).f_values (Seq.index ${re} 2).f_values (Seq.index ${re} 10).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (2 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 3).f_values (Seq.index ${orig_re} 11).f_values (Seq.index ${re} 3).f_values (Seq.index ${re} 11).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (3 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 4).f_values (Seq.index ${orig_re} 12).f_values (Seq.index ${re} 4).f_values (Seq.index ${re} 12).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (4 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 5).f_values (Seq.index ${orig_re} 13).f_values (Seq.index ${re} 5).f_values (Seq.index ${re} 13).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (5 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 6).f_values (Seq.index ${orig_re} 14).f_values (Seq.index ${re} 6).f_values (Seq.index ${re} 14).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (6 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 7).f_values (Seq.index ${orig_re} 15).f_values (Seq.index ${re} 7).f_values (Seq.index ${re} 15).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (7 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 16).f_values (Seq.index ${orig_re} 24).f_values (Seq.index ${re} 16).f_values (Seq.index ${re} 24).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (16 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 17).f_values (Seq.index ${orig_re} 25).f_values (Seq.index ${re} 17).f_values (Seq.index ${re} 25).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (17 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 18).f_values (Seq.index ${orig_re} 26).f_values (Seq.index ${re} 18).f_values (Seq.index ${re} 26).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (18 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 19).f_values (Seq.index ${orig_re} 27).f_values (Seq.index ${re} 19).f_values (Seq.index ${re} 27).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (19 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 20).f_values (Seq.index ${orig_re} 28).f_values (Seq.index ${re} 20).f_values (Seq.index ${re} 28).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (20 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 21).f_values (Seq.index ${orig_re} 29).f_values (Seq.index ${re} 21).f_values (Seq.index ${re} 29).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (21 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 22).f_values (Seq.index ${orig_re} 30).f_values (Seq.index ${re} 22).f_values (Seq.index ${re} 30).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (22 / 16 + 2))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 23).f_values (Seq.index ${orig_re} 31).f_values (Seq.index ${re} 23).f_values (Seq.index ${re} 31).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (23 / 16 + 2))));
lemma_l6_cross_driver_compose ${orig_re} ${re}
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND) $re
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 1 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 7) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn ntt_at_layer_7(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 128; // 1 << LAYER;
    const STEP_BY: usize = 16; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 25847>(re);

    hax_lib::fstar!(r#"
assert_norm (Spec.MLDSA.NttConstants.zeta_r 1 == 25847);
assert (unit_fe_post_cross (Seq.index ${orig_re} 0).f_values (Seq.index ${orig_re} 16).f_values (Seq.index ${re} 0).f_values (Seq.index ${re} 16).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (0 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 1).f_values (Seq.index ${orig_re} 17).f_values (Seq.index ${re} 1).f_values (Seq.index ${re} 17).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (1 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 2).f_values (Seq.index ${orig_re} 18).f_values (Seq.index ${re} 2).f_values (Seq.index ${re} 18).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (2 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 3).f_values (Seq.index ${orig_re} 19).f_values (Seq.index ${re} 3).f_values (Seq.index ${re} 19).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (3 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 4).f_values (Seq.index ${orig_re} 20).f_values (Seq.index ${re} 4).f_values (Seq.index ${re} 20).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (4 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 5).f_values (Seq.index ${orig_re} 21).f_values (Seq.index ${re} 5).f_values (Seq.index ${re} 21).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (5 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 6).f_values (Seq.index ${orig_re} 22).f_values (Seq.index ${re} 6).f_values (Seq.index ${re} 22).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (6 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 7).f_values (Seq.index ${orig_re} 23).f_values (Seq.index ${re} 7).f_values (Seq.index ${re} 23).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (7 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 8).f_values (Seq.index ${orig_re} 24).f_values (Seq.index ${re} 8).f_values (Seq.index ${re} 24).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (8 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 9).f_values (Seq.index ${orig_re} 25).f_values (Seq.index ${re} 9).f_values (Seq.index ${re} 25).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (9 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 10).f_values (Seq.index ${orig_re} 26).f_values (Seq.index ${re} 10).f_values (Seq.index ${re} 26).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (10 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 11).f_values (Seq.index ${orig_re} 27).f_values (Seq.index ${re} 11).f_values (Seq.index ${re} 27).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (11 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 12).f_values (Seq.index ${orig_re} 28).f_values (Seq.index ${re} 12).f_values (Seq.index ${re} 28).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (12 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 13).f_values (Seq.index ${orig_re} 29).f_values (Seq.index ${re} 13).f_values (Seq.index ${re} 29).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (13 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 14).f_values (Seq.index ${orig_re} 30).f_values (Seq.index ${re} 14).f_values (Seq.index ${re} 30).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (14 / 32 + 1))));
assert (unit_fe_post_cross (Seq.index ${orig_re} 15).f_values (Seq.index ${orig_re} 31).f_values (Seq.index ${re} 15).f_values (Seq.index ${re} 31).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r (15 / 32 + 1))));
lemma_l7_cross_driver_compose ${orig_re} ${re}
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"
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

#push-options "--fuel 0 --ifuel 2 --z3rlimit 200"
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
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 8 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}_future) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index (Hacspec_ml_dsa.Ntt.ntt in_flat) i)) % 8380417)
"#) )]
pub(crate) fn ntt(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[cfg(hax)]
    let s0 = re.clone();
    ntt_at_layer_7(re);
    #[cfg(hax)]
    let s7 = re.clone();
    ntt_at_layer_6(re);
    #[cfg(hax)]
    let s6 = re.clone();
    ntt_at_layer_5(re);
    #[cfg(hax)]
    let s5 = re.clone();
    ntt_at_layer_4(re);
    #[cfg(hax)]
    let s4 = re.clone();
    ntt_at_layer_3(re);
    #[cfg(hax)]
    let s3 = re.clone();
    ntt_at_layer_2(re);
    #[cfg(hax)]
    let s2 = re.clone();
    ntt_at_layer_1(re);
    #[cfg(hax)]
    let s1 = re.clone();
    ntt_at_layer_0(re);
    hax_lib::fstar!(r#"
lemma_ntt_compose_8
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${s0}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${s7}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${s6}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${s5}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${s4}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${s3}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${s2}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${s1}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (chunks_of_re ${re}))
"#);
}

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
                                (mk_i32 (Spec.MLDSA.Ntt.zeta_r (b + 32)))))
            (ensures
              (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array orig in
               let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array fut in
               let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 2) in
               forall (i: nat). i < 256 ==>
                 (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
      = let zm (b: nat{b < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
          mk_i32 (Spec.MLDSA.Ntt.zeta_r (b + 32)) in
        let t (b: nat{b < 32}) (p: nat{p < 4}) : i32 =
          Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer
            (Seq.index (Seq.index orig b) (p + 4)) (zm b) in
        forall32_elim_1d (fun b -> unit_fe_post_l2 (Seq.index orig b) (Seq.index fut b)
                                     (mk_i32 (Spec.MLDSA.Ntt.zeta_r (b + 32))));
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
            let _ = Spec.MLDSA.Ntt.zeta_r (b + 32) in
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
                                (mk_i32 (Spec.MLDSA.Ntt.zeta_r (2*b + 0 + 64)))
                                (mk_i32 (Spec.MLDSA.Ntt.zeta_r (2*b + 1 + 64)))))
            (ensures
              (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array orig in
               let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array fut in
               let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 1) in
               forall (i: nat). i < 256 ==>
                 (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
      = let zm (b: nat{b < 32}) (h: nat{h < 2}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
          mk_i32 (Spec.MLDSA.Ntt.zeta_r (2*b + h + 64)) in
        let t (b: nat{b < 32}) (h: nat{h < 2}) (j: nat{j < 2}) : i32 =
          Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer
            (Seq.index (Seq.index orig b) (4*h+j+2)) (zm b h) in
        forall32_elim_1d (fun b -> unit_fe_post_l1 (Seq.index orig b) (Seq.index fut b)
                                     (mk_i32 (Spec.MLDSA.Ntt.zeta_r (2*b + 0 + 64)))
                                     (mk_i32 (Spec.MLDSA.Ntt.zeta_r (2*b + 1 + 64))));
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
            let _ = Spec.MLDSA.Ntt.zeta_r (2*b + h + 64) in
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
                                (mk_i32 (Spec.MLDSA.Ntt.zeta_r (4*b + 0 + 128)))
                                (mk_i32 (Spec.MLDSA.Ntt.zeta_r (4*b + 1 + 128)))
                                (mk_i32 (Spec.MLDSA.Ntt.zeta_r (4*b + 2 + 128)))
                                (mk_i32 (Spec.MLDSA.Ntt.zeta_r (4*b + 3 + 128)))))
            (ensures
              (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array orig in
               let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array fut in
               let spec = Hacspec_ml_dsa.Ntt.ntt_layer in_flat (mk_usize 0) in
               forall (i: nat). i < 256 ==>
                 (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
      = let zm (b: nat{b < 32}) (p: nat{p < 4}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
          mk_i32 (Spec.MLDSA.Ntt.zeta_r (4*b + p + 128)) in
        let t (b: nat{b < 32}) (p: nat{p < 4}) : i32 =
          Libcrux_ml_dsa.Simd.Portable.Arithmetic.montgomery_multiply_fe_by_fer
            (Seq.index (Seq.index orig b) (2*p+1)) (zm b p) in
        forall32_elim_1d (fun b -> unit_fe_post_l0 (Seq.index orig b) (Seq.index fut b)
                                     (mk_i32 (Spec.MLDSA.Ntt.zeta_r (4*b + 0 + 128)))
                                     (mk_i32 (Spec.MLDSA.Ntt.zeta_r (4*b + 1 + 128)))
                                     (mk_i32 (Spec.MLDSA.Ntt.zeta_r (4*b + 2 + 128)))
                                     (mk_i32 (Spec.MLDSA.Ntt.zeta_r (4*b + 3 + 128))));
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
            let _ = Spec.MLDSA.Ntt.zeta_r (4*b + p + 128) in
            Hacspec_ml_dsa.Commute.Chunk.lemma_v_zetas_eq_zeta (4*b + p + 128)
         in Classical.forall_intro_2 aux);
        Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_layer_0_step_to_hacspec_poly orig fut t zm
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
assert_norm (Spec.MLDSA.Ntt.zeta_r 128 == 2091667);
assert_norm (Spec.MLDSA.Ntt.zeta_r 129 == 3407706);
assert_norm (Spec.MLDSA.Ntt.zeta_r 130 == 2316500);
assert_norm (Spec.MLDSA.Ntt.zeta_r 131 == 3817976);
assert_norm (Spec.MLDSA.Ntt.zeta_r 132 == (-3342478));
assert_norm (Spec.MLDSA.Ntt.zeta_r 133 == 2244091);
assert_norm (Spec.MLDSA.Ntt.zeta_r 134 == (-2446433));
assert_norm (Spec.MLDSA.Ntt.zeta_r 135 == (-3562462));
assert_norm (Spec.MLDSA.Ntt.zeta_r 136 == 266997);
assert_norm (Spec.MLDSA.Ntt.zeta_r 137 == 2434439);
assert_norm (Spec.MLDSA.Ntt.zeta_r 138 == (-1235728));
assert_norm (Spec.MLDSA.Ntt.zeta_r 139 == 3513181);
assert_norm (Spec.MLDSA.Ntt.zeta_r 140 == (-3520352));
assert_norm (Spec.MLDSA.Ntt.zeta_r 141 == (-3759364));
assert_norm (Spec.MLDSA.Ntt.zeta_r 142 == (-1197226));
assert_norm (Spec.MLDSA.Ntt.zeta_r 143 == (-3193378));
assert_norm (Spec.MLDSA.Ntt.zeta_r 144 == 900702);
assert_norm (Spec.MLDSA.Ntt.zeta_r 145 == 1859098);
assert_norm (Spec.MLDSA.Ntt.zeta_r 146 == 909542);
assert_norm (Spec.MLDSA.Ntt.zeta_r 147 == 819034);
assert_norm (Spec.MLDSA.Ntt.zeta_r 148 == 495491);
assert_norm (Spec.MLDSA.Ntt.zeta_r 149 == (-1613174));
assert_norm (Spec.MLDSA.Ntt.zeta_r 150 == (-43260));
assert_norm (Spec.MLDSA.Ntt.zeta_r 151 == (-522500));
assert_norm (Spec.MLDSA.Ntt.zeta_r 152 == (-655327));
assert_norm (Spec.MLDSA.Ntt.zeta_r 153 == (-3122442));
assert_norm (Spec.MLDSA.Ntt.zeta_r 154 == 2031748);
assert_norm (Spec.MLDSA.Ntt.zeta_r 155 == 3207046);
assert_norm (Spec.MLDSA.Ntt.zeta_r 156 == (-3556995));
assert_norm (Spec.MLDSA.Ntt.zeta_r 157 == (-525098));
assert_norm (Spec.MLDSA.Ntt.zeta_r 158 == (-768622));
assert_norm (Spec.MLDSA.Ntt.zeta_r 159 == (-3595838));
assert_norm (Spec.MLDSA.Ntt.zeta_r 160 == 342297);
assert_norm (Spec.MLDSA.Ntt.zeta_r 161 == 286988);
assert_norm (Spec.MLDSA.Ntt.zeta_r 162 == (-2437823));
assert_norm (Spec.MLDSA.Ntt.zeta_r 163 == 4108315);
assert_norm (Spec.MLDSA.Ntt.zeta_r 164 == 3437287);
assert_norm (Spec.MLDSA.Ntt.zeta_r 165 == (-3342277));
assert_norm (Spec.MLDSA.Ntt.zeta_r 166 == 1735879);
assert_norm (Spec.MLDSA.Ntt.zeta_r 167 == 203044);
assert_norm (Spec.MLDSA.Ntt.zeta_r 168 == 2842341);
assert_norm (Spec.MLDSA.Ntt.zeta_r 169 == 2691481);
assert_norm (Spec.MLDSA.Ntt.zeta_r 170 == (-2590150));
assert_norm (Spec.MLDSA.Ntt.zeta_r 171 == 1265009);
assert_norm (Spec.MLDSA.Ntt.zeta_r 172 == 4055324);
assert_norm (Spec.MLDSA.Ntt.zeta_r 173 == 1247620);
assert_norm (Spec.MLDSA.Ntt.zeta_r 174 == 2486353);
assert_norm (Spec.MLDSA.Ntt.zeta_r 175 == 1595974);
assert_norm (Spec.MLDSA.Ntt.zeta_r 176 == (-3767016));
assert_norm (Spec.MLDSA.Ntt.zeta_r 177 == 1250494);
assert_norm (Spec.MLDSA.Ntt.zeta_r 178 == 2635921);
assert_norm (Spec.MLDSA.Ntt.zeta_r 179 == (-3548272));
assert_norm (Spec.MLDSA.Ntt.zeta_r 180 == (-2994039));
assert_norm (Spec.MLDSA.Ntt.zeta_r 181 == 1869119);
assert_norm (Spec.MLDSA.Ntt.zeta_r 182 == 1903435);
assert_norm (Spec.MLDSA.Ntt.zeta_r 183 == (-1050970));
assert_norm (Spec.MLDSA.Ntt.zeta_r 184 == (-1333058));
assert_norm (Spec.MLDSA.Ntt.zeta_r 185 == 1237275);
assert_norm (Spec.MLDSA.Ntt.zeta_r 186 == (-3318210));
assert_norm (Spec.MLDSA.Ntt.zeta_r 187 == (-1430225));
assert_norm (Spec.MLDSA.Ntt.zeta_r 188 == (-451100));
assert_norm (Spec.MLDSA.Ntt.zeta_r 189 == 1312455);
assert_norm (Spec.MLDSA.Ntt.zeta_r 190 == 3306115);
assert_norm (Spec.MLDSA.Ntt.zeta_r 191 == (-1962642));
assert_norm (Spec.MLDSA.Ntt.zeta_r 192 == (-1279661));
assert_norm (Spec.MLDSA.Ntt.zeta_r 193 == 1917081);
assert_norm (Spec.MLDSA.Ntt.zeta_r 194 == (-2546312));
assert_norm (Spec.MLDSA.Ntt.zeta_r 195 == (-1374803));
assert_norm (Spec.MLDSA.Ntt.zeta_r 196 == 1500165);
assert_norm (Spec.MLDSA.Ntt.zeta_r 197 == 777191);
assert_norm (Spec.MLDSA.Ntt.zeta_r 198 == 2235880);
assert_norm (Spec.MLDSA.Ntt.zeta_r 199 == 3406031);
assert_norm (Spec.MLDSA.Ntt.zeta_r 200 == (-542412));
assert_norm (Spec.MLDSA.Ntt.zeta_r 201 == (-2831860));
assert_norm (Spec.MLDSA.Ntt.zeta_r 202 == (-1671176));
assert_norm (Spec.MLDSA.Ntt.zeta_r 203 == (-1846953));
assert_norm (Spec.MLDSA.Ntt.zeta_r 204 == (-2584293));
assert_norm (Spec.MLDSA.Ntt.zeta_r 205 == (-3724270));
assert_norm (Spec.MLDSA.Ntt.zeta_r 206 == 594136);
assert_norm (Spec.MLDSA.Ntt.zeta_r 207 == (-3776993));
assert_norm (Spec.MLDSA.Ntt.zeta_r 208 == (-2013608));
assert_norm (Spec.MLDSA.Ntt.zeta_r 209 == 2432395);
assert_norm (Spec.MLDSA.Ntt.zeta_r 210 == 2454455);
assert_norm (Spec.MLDSA.Ntt.zeta_r 211 == (-164721));
assert_norm (Spec.MLDSA.Ntt.zeta_r 212 == 1957272);
assert_norm (Spec.MLDSA.Ntt.zeta_r 213 == 3369112);
assert_norm (Spec.MLDSA.Ntt.zeta_r 214 == 185531);
assert_norm (Spec.MLDSA.Ntt.zeta_r 215 == (-1207385));
assert_norm (Spec.MLDSA.Ntt.zeta_r 216 == (-3183426));
assert_norm (Spec.MLDSA.Ntt.zeta_r 217 == 162844);
assert_norm (Spec.MLDSA.Ntt.zeta_r 218 == 1616392);
assert_norm (Spec.MLDSA.Ntt.zeta_r 219 == 3014001);
assert_norm (Spec.MLDSA.Ntt.zeta_r 220 == 810149);
assert_norm (Spec.MLDSA.Ntt.zeta_r 221 == 1652634);
assert_norm (Spec.MLDSA.Ntt.zeta_r 222 == (-3694233));
assert_norm (Spec.MLDSA.Ntt.zeta_r 223 == (-1799107));
assert_norm (Spec.MLDSA.Ntt.zeta_r 224 == (-3038916));
assert_norm (Spec.MLDSA.Ntt.zeta_r 225 == 3523897);
assert_norm (Spec.MLDSA.Ntt.zeta_r 226 == 3866901);
assert_norm (Spec.MLDSA.Ntt.zeta_r 227 == 269760);
assert_norm (Spec.MLDSA.Ntt.zeta_r 228 == 2213111);
assert_norm (Spec.MLDSA.Ntt.zeta_r 229 == (-975884));
assert_norm (Spec.MLDSA.Ntt.zeta_r 230 == 1717735);
assert_norm (Spec.MLDSA.Ntt.zeta_r 231 == 472078);
assert_norm (Spec.MLDSA.Ntt.zeta_r 232 == (-426683));
assert_norm (Spec.MLDSA.Ntt.zeta_r 233 == 1723600);
assert_norm (Spec.MLDSA.Ntt.zeta_r 234 == (-1803090));
assert_norm (Spec.MLDSA.Ntt.zeta_r 235 == 1910376);
assert_norm (Spec.MLDSA.Ntt.zeta_r 236 == (-1667432));
assert_norm (Spec.MLDSA.Ntt.zeta_r 237 == (-1104333));
assert_norm (Spec.MLDSA.Ntt.zeta_r 238 == (-260646));
assert_norm (Spec.MLDSA.Ntt.zeta_r 239 == (-3833893));
assert_norm (Spec.MLDSA.Ntt.zeta_r 240 == (-2939036));
assert_norm (Spec.MLDSA.Ntt.zeta_r 241 == (-2235985));
assert_norm (Spec.MLDSA.Ntt.zeta_r 242 == (-420899));
assert_norm (Spec.MLDSA.Ntt.zeta_r 243 == (-2286327));
assert_norm (Spec.MLDSA.Ntt.zeta_r 244 == 183443);
assert_norm (Spec.MLDSA.Ntt.zeta_r 245 == (-976891));
assert_norm (Spec.MLDSA.Ntt.zeta_r 246 == 1612842);
assert_norm (Spec.MLDSA.Ntt.zeta_r 247 == (-3545687));
assert_norm (Spec.MLDSA.Ntt.zeta_r 248 == (-554416));
assert_norm (Spec.MLDSA.Ntt.zeta_r 249 == 3919660);
assert_norm (Spec.MLDSA.Ntt.zeta_r 250 == (-48306));
assert_norm (Spec.MLDSA.Ntt.zeta_r 251 == (-1362209));
assert_norm (Spec.MLDSA.Ntt.zeta_r 252 == 3937738);
assert_norm (Spec.MLDSA.Ntt.zeta_r 253 == 1400424);
assert_norm (Spec.MLDSA.Ntt.zeta_r 254 == (-846154));
assert_norm (Spec.MLDSA.Ntt.zeta_r 255 == 1976782);
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
assert_norm (Spec.MLDSA.Ntt.zeta_r 64 == (-3930395));
assert_norm (Spec.MLDSA.Ntt.zeta_r 65 == (-1528703));
assert_norm (Spec.MLDSA.Ntt.zeta_r 66 == (-3677745));
assert_norm (Spec.MLDSA.Ntt.zeta_r 67 == (-3041255));
assert_norm (Spec.MLDSA.Ntt.zeta_r 68 == (-1452451));
assert_norm (Spec.MLDSA.Ntt.zeta_r 69 == 3475950);
assert_norm (Spec.MLDSA.Ntt.zeta_r 70 == 2176455);
assert_norm (Spec.MLDSA.Ntt.zeta_r 71 == (-1585221));
assert_norm (Spec.MLDSA.Ntt.zeta_r 72 == (-1257611));
assert_norm (Spec.MLDSA.Ntt.zeta_r 73 == 1939314);
assert_norm (Spec.MLDSA.Ntt.zeta_r 74 == (-4083598));
assert_norm (Spec.MLDSA.Ntt.zeta_r 75 == (-1000202));
assert_norm (Spec.MLDSA.Ntt.zeta_r 76 == (-3190144));
assert_norm (Spec.MLDSA.Ntt.zeta_r 77 == (-3157330));
assert_norm (Spec.MLDSA.Ntt.zeta_r 78 == (-3632928));
assert_norm (Spec.MLDSA.Ntt.zeta_r 79 == 126922);
assert_norm (Spec.MLDSA.Ntt.zeta_r 80 == 3412210);
assert_norm (Spec.MLDSA.Ntt.zeta_r 81 == (-983419));
assert_norm (Spec.MLDSA.Ntt.zeta_r 82 == 2147896);
assert_norm (Spec.MLDSA.Ntt.zeta_r 83 == 2715295);
assert_norm (Spec.MLDSA.Ntt.zeta_r 84 == (-2967645));
assert_norm (Spec.MLDSA.Ntt.zeta_r 85 == (-3693493));
assert_norm (Spec.MLDSA.Ntt.zeta_r 86 == (-411027));
assert_norm (Spec.MLDSA.Ntt.zeta_r 87 == (-2477047));
assert_norm (Spec.MLDSA.Ntt.zeta_r 88 == (-671102));
assert_norm (Spec.MLDSA.Ntt.zeta_r 89 == (-1228525));
assert_norm (Spec.MLDSA.Ntt.zeta_r 90 == (-22981));
assert_norm (Spec.MLDSA.Ntt.zeta_r 91 == (-1308169));
assert_norm (Spec.MLDSA.Ntt.zeta_r 92 == (-381987));
assert_norm (Spec.MLDSA.Ntt.zeta_r 93 == 1349076);
assert_norm (Spec.MLDSA.Ntt.zeta_r 94 == 1852771);
assert_norm (Spec.MLDSA.Ntt.zeta_r 95 == (-1430430));
assert_norm (Spec.MLDSA.Ntt.zeta_r 96 == (-3343383));
assert_norm (Spec.MLDSA.Ntt.zeta_r 97 == 264944);
assert_norm (Spec.MLDSA.Ntt.zeta_r 98 == 508951);
assert_norm (Spec.MLDSA.Ntt.zeta_r 99 == 3097992);
assert_norm (Spec.MLDSA.Ntt.zeta_r 100 == 44288);
assert_norm (Spec.MLDSA.Ntt.zeta_r 101 == (-1100098));
assert_norm (Spec.MLDSA.Ntt.zeta_r 102 == 904516);
assert_norm (Spec.MLDSA.Ntt.zeta_r 103 == 3958618);
assert_norm (Spec.MLDSA.Ntt.zeta_r 104 == (-3724342));
assert_norm (Spec.MLDSA.Ntt.zeta_r 105 == (-8578));
assert_norm (Spec.MLDSA.Ntt.zeta_r 106 == 1653064);
assert_norm (Spec.MLDSA.Ntt.zeta_r 107 == (-3249728));
assert_norm (Spec.MLDSA.Ntt.zeta_r 108 == 2389356);
assert_norm (Spec.MLDSA.Ntt.zeta_r 109 == (-210977));
assert_norm (Spec.MLDSA.Ntt.zeta_r 110 == 759969);
assert_norm (Spec.MLDSA.Ntt.zeta_r 111 == (-1316856));
assert_norm (Spec.MLDSA.Ntt.zeta_r 112 == 189548);
assert_norm (Spec.MLDSA.Ntt.zeta_r 113 == (-3553272));
assert_norm (Spec.MLDSA.Ntt.zeta_r 114 == 3159746);
assert_norm (Spec.MLDSA.Ntt.zeta_r 115 == (-1851402));
assert_norm (Spec.MLDSA.Ntt.zeta_r 116 == (-2409325));
assert_norm (Spec.MLDSA.Ntt.zeta_r 117 == (-177440));
assert_norm (Spec.MLDSA.Ntt.zeta_r 118 == 1315589);
assert_norm (Spec.MLDSA.Ntt.zeta_r 119 == 1341330);
assert_norm (Spec.MLDSA.Ntt.zeta_r 120 == 1285669);
assert_norm (Spec.MLDSA.Ntt.zeta_r 121 == (-1584928));
assert_norm (Spec.MLDSA.Ntt.zeta_r 122 == (-812732));
assert_norm (Spec.MLDSA.Ntt.zeta_r 123 == (-1439742));
assert_norm (Spec.MLDSA.Ntt.zeta_r 124 == (-3019102));
assert_norm (Spec.MLDSA.Ntt.zeta_r 125 == (-3881060));
assert_norm (Spec.MLDSA.Ntt.zeta_r 126 == (-3628969));
assert_norm (Spec.MLDSA.Ntt.zeta_r 127 == 3839961);
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
assert_norm (Spec.MLDSA.Ntt.zeta_r 32 == 2706023);
assert_norm (Spec.MLDSA.Ntt.zeta_r 33 == 95776);
assert_norm (Spec.MLDSA.Ntt.zeta_r 34 == 3077325);
assert_norm (Spec.MLDSA.Ntt.zeta_r 35 == 3530437);
assert_norm (Spec.MLDSA.Ntt.zeta_r 36 == (-1661693));
assert_norm (Spec.MLDSA.Ntt.zeta_r 37 == (-3592148));
assert_norm (Spec.MLDSA.Ntt.zeta_r 38 == (-2537516));
assert_norm (Spec.MLDSA.Ntt.zeta_r 39 == 3915439);
assert_norm (Spec.MLDSA.Ntt.zeta_r 40 == (-3861115));
assert_norm (Spec.MLDSA.Ntt.zeta_r 41 == (-3043716));
assert_norm (Spec.MLDSA.Ntt.zeta_r 42 == 3574422);
assert_norm (Spec.MLDSA.Ntt.zeta_r 43 == (-2867647));
assert_norm (Spec.MLDSA.Ntt.zeta_r 44 == 3539968);
assert_norm (Spec.MLDSA.Ntt.zeta_r 45 == (-300467));
assert_norm (Spec.MLDSA.Ntt.zeta_r 46 == 2348700);
assert_norm (Spec.MLDSA.Ntt.zeta_r 47 == (-539299));
assert_norm (Spec.MLDSA.Ntt.zeta_r 48 == (-1699267));
assert_norm (Spec.MLDSA.Ntt.zeta_r 49 == (-1643818));
assert_norm (Spec.MLDSA.Ntt.zeta_r 50 == 3505694);
assert_norm (Spec.MLDSA.Ntt.zeta_r 51 == (-3821735));
assert_norm (Spec.MLDSA.Ntt.zeta_r 52 == 3507263);
assert_norm (Spec.MLDSA.Ntt.zeta_r 53 == (-2140649));
assert_norm (Spec.MLDSA.Ntt.zeta_r 54 == (-1600420));
assert_norm (Spec.MLDSA.Ntt.zeta_r 55 == 3699596);
assert_norm (Spec.MLDSA.Ntt.zeta_r 56 == 811944);
assert_norm (Spec.MLDSA.Ntt.zeta_r 57 == 531354);
assert_norm (Spec.MLDSA.Ntt.zeta_r 58 == 954230);
assert_norm (Spec.MLDSA.Ntt.zeta_r 59 == 3881043);
assert_norm (Spec.MLDSA.Ntt.zeta_r 60 == 3900724);
assert_norm (Spec.MLDSA.Ntt.zeta_r 61 == (-2556880));
assert_norm (Spec.MLDSA.Ntt.zeta_r 62 == 2071892);
assert_norm (Spec.MLDSA.Ntt.zeta_r 63 == (-2797779));
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
                (Seq.index ${re}_future i).f_values))
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
                    (Seq.index ${re}_future (v $index + v step_by)).f_values
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
        let mut tmp = re[index + step_by];
        montgomery_multiply_by_constant(&mut tmp, zeta);

        re[index + step_by] = re[index];

        arithmetic::subtract(&mut re[index + step_by], &tmp);
        arithmetic::add(&mut re[index], &tmp);
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
                    (Seq.index ${re} i).f_values))
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
    is_i32b_polynomial (v $NTT_BASE_BOUND + 5 * v $FIELD_MAX) ${re}_future
"#) )]
fn ntt_at_layer_3(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 8; // 1 << LAYER;
    const STEP_BY: usize = 1; // step / COEFFICIENTS_IN_SIMD_UNIT;

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
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 3 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 4 * v $FIELD_MAX) ${re}_future
"#) )]
fn ntt_at_layer_4(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 16; // 1 << LAYER;
    const STEP_BY: usize = 2; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1826347>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2353451>(re);
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -359251>(re);
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2091905>(re);
    outer_3_plus::<{ (4 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3119733>(re);
    outer_3_plus::<{ (5 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2884855>(re);
    outer_3_plus::<{ (6 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3111497>(re);
    outer_3_plus::<{ (7 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2680103>(re);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 2 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 3 * v $FIELD_MAX) ${re}_future
"#) )]
fn ntt_at_layer_5(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 32; // 1 << LAYER;
    const STEP_BY: usize = 4; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 237124>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -777960>(re);
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -876248>(re);
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 466468>(re);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 1 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 2 * v $FIELD_MAX) ${re}_future
"#) )]
fn ntt_at_layer_6(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 64; // 1 << LAYER;
    const STEP_BY: usize = 8; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2608894>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -518909>(re);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND) $re
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 1 * v $FIELD_MAX) ${re}_future
"#) )]
fn ntt_at_layer_7(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 128; // 1 << LAYER;
    const STEP_BY: usize = 16; // step / COEFFICIENTS_IN_SIMD_UNIT;

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 25847>(re);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    is_i32b_polynomial (v $NTT_BASE_BOUND + 8 * v $FIELD_MAX) ${re}_future
"#) )]
pub(crate) fn ntt(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    ntt_at_layer_7(re);
    ntt_at_layer_6(re);
    ntt_at_layer_5(re);
    ntt_at_layer_4(re);
    ntt_at_layer_3(re);
    ntt_at_layer_2(re);
    ntt_at_layer_1(re);
    ntt_at_layer_0(re);
}

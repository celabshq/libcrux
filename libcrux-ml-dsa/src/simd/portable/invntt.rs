use super::arithmetic::{self, montgomery_multiply_fe_by_fer};
use super::vector_type::Coefficients;
use crate::simd::traits::{COEFFICIENTS_IN_SIMD_UNIT, SIMD_UNITS_IN_RING_ELEMENT};

#[cfg(hax)]
use crate::simd::traits::specs::*;

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::before(
    r#"
let simd_layer_factor (step:usize) =
    match step with
    | MkInt 1 -> 1
    | MkInt 2 -> 2
    | MkInt 4 -> 4
    | _ -> 5
"#
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    1 <= v $step /\ v $step <= 4 /\ v $index + v $step < 8 /\
    Spec.Utils.is_i32b (simd_layer_factor $step * v $FIELD_MAX)
                    (Seq.index ${simd_unit}.f_values (v $index)) /\
    Spec.Utils.is_i32b (simd_layer_factor $step * v $FIELD_MAX)
                    (Seq.index ${simd_unit}.f_values (v $index + v $step)) /\
    Spec.Utils.is_i32b 4190208 $zeta
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.modifies2_8 ${simd_unit}.f_values ${simd_unit}_future.f_values index (index +! step) /\
    Spec.Utils.is_i32b (2 * (simd_layer_factor $step)  * v $FIELD_MAX)
                    (Seq.index ${simd_unit}_future.f_values (v $index)) /\
    Spec.Utils.is_i32b (2 * (simd_layer_factor $step)  * v $FIELD_MAX)
                    (Seq.index ${simd_unit}_future.f_values (v $index + v $step)) /\
    (let ci = ${simd_unit}.f_values in
     let co = ${simd_unit}_future.f_values in
     v (Seq.index co (v $index)) ==
       v (Seq.index ci (v $index)) + v (Seq.index ci (v $index + v $step)) /\
     (v (Seq.index co (v $index + v $step))) % 8380417 ==
       ((v (Seq.index ci (v $index + v $step)) - v (Seq.index ci (v $index))) * v $zeta * 8265825) % 8380417)
"#) )]
fn simd_unit_inv_ntt_step(simd_unit: &mut Coefficients, zeta: i32, index: usize, step: usize) {
    let a_minus_b = simd_unit.values[index + step] - simd_unit.values[index];
    simd_unit.values[index] = simd_unit.values[index] + simd_unit.values[index + step];
    simd_unit.values[index + step] = montgomery_multiply_fe_by_fer(a_minus_b, zeta);
    hax_lib::fstar!(r#"reveal_opaque (`%Spec.MLDSA.Math.mod_q) (Spec.MLDSA.Math.mod_q)"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
        Libcrux_ml_dsa.Simd.Portable.Ntt.forall32_elim_1d (fun b -> unit_fe_post_inv_l0 (Seq.index orig b) (Seq.index fut b)
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
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.is_i32b_array (v $FIELD_MAX) ${simd_unit}.f_values /\
    Spec.Utils.is_i32b 4190208 $zeta0 /\
    Spec.Utils.is_i32b 4190208 $zeta1 /\
    Spec.Utils.is_i32b 4190208 $zeta2 /\
    Spec.Utils.is_i32b 4190208 $zeta3
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_array (2 * v $FIELD_MAX) ${simd_unit}_future.f_values /\
    (let ci = ${simd_unit}.f_values in
     let co = ${simd_unit}_future.f_values in
     v (Seq.index co 0) == v (Seq.index ci 0) + v (Seq.index ci 1) /\
     (v (Seq.index co 1)) % 8380417 == ((v (Seq.index ci 1) - v (Seq.index ci 0)) * v $zeta0 * 8265825) % 8380417 /\
     v (Seq.index co 2) == v (Seq.index ci 2) + v (Seq.index ci 3) /\
     (v (Seq.index co 3)) % 8380417 == ((v (Seq.index ci 3) - v (Seq.index ci 2)) * v $zeta1 * 8265825) % 8380417 /\
     v (Seq.index co 4) == v (Seq.index ci 4) + v (Seq.index ci 5) /\
     (v (Seq.index co 5)) % 8380417 == ((v (Seq.index ci 5) - v (Seq.index ci 4)) * v $zeta2 * 8265825) % 8380417 /\
     v (Seq.index co 6) == v (Seq.index ci 6) + v (Seq.index ci 7) /\
     (v (Seq.index co 7)) % 8380417 == ((v (Seq.index ci 7) - v (Seq.index ci 6)) * v $zeta3 * 8265825) % 8380417)
"#) )]
pub fn simd_unit_invert_ntt_at_layer_0(
    simd_unit: &mut Coefficients,
    zeta0: i32,
    zeta1: i32,
    zeta2: i32,
    zeta3: i32,
) {
    simd_unit_inv_ntt_step(simd_unit, zeta0, 0, 1);
    simd_unit_inv_ntt_step(simd_unit, zeta1, 2, 1);
    simd_unit_inv_ntt_step(simd_unit, zeta2, 4, 1);
    simd_unit_inv_ntt_step(simd_unit, zeta3, 6, 1);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
        Libcrux_ml_dsa.Simd.Portable.Ntt.forall32_elim_1d (fun b -> unit_fe_post_inv_l1 (Seq.index orig b) (Seq.index fut b)
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
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.is_i32b_array (2 * v $FIELD_MAX) ${simd_unit}.f_values /\
    Spec.Utils.is_i32b 4190208 $zeta0 /\
    Spec.Utils.is_i32b 4190208 $zeta1
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_array (4 * v $FIELD_MAX) ${simd_unit}_future.f_values /\
    (let ci = ${simd_unit}.f_values in
     let co = ${simd_unit}_future.f_values in
     v (Seq.index co 0) == v (Seq.index ci 0) + v (Seq.index ci 2) /\
     (v (Seq.index co 2)) % 8380417 == ((v (Seq.index ci 2) - v (Seq.index ci 0)) * v $zeta0 * 8265825) % 8380417 /\
     v (Seq.index co 1) == v (Seq.index ci 1) + v (Seq.index ci 3) /\
     (v (Seq.index co 3)) % 8380417 == ((v (Seq.index ci 3) - v (Seq.index ci 1)) * v $zeta0 * 8265825) % 8380417 /\
     v (Seq.index co 4) == v (Seq.index ci 4) + v (Seq.index ci 6) /\
     (v (Seq.index co 6)) % 8380417 == ((v (Seq.index ci 6) - v (Seq.index ci 4)) * v $zeta1 * 8265825) % 8380417 /\
     v (Seq.index co 5) == v (Seq.index ci 5) + v (Seq.index ci 7) /\
     (v (Seq.index co 7)) % 8380417 == ((v (Seq.index ci 7) - v (Seq.index ci 5)) * v $zeta1 * 8265825) % 8380417)
"#) )]
pub fn simd_unit_invert_ntt_at_layer_1(simd_unit: &mut Coefficients, zeta0: i32, zeta1: i32) {
    simd_unit_inv_ntt_step(simd_unit, zeta0, 0, 2);
    simd_unit_inv_ntt_step(simd_unit, zeta0, 1, 2);
    simd_unit_inv_ntt_step(simd_unit, zeta1, 4, 2);
    simd_unit_inv_ntt_step(simd_unit, zeta1, 5, 2);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
        Libcrux_ml_dsa.Simd.Portable.Ntt.forall32_elim_1d (fun b -> unit_fe_post_inv_l2 (Seq.index orig b) (Seq.index fut b)
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
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Spec.Utils.is_i32b_array (4 * v $FIELD_MAX) ${simd_unit}.f_values /\
    Spec.Utils.is_i32b 4190208 $zeta
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.is_i32b_array (8 * v $FIELD_MAX) ${simd_unit}_future.f_values /\
    (let ci = ${simd_unit}.f_values in
     let co = ${simd_unit}_future.f_values in
     v (Seq.index co 0) == v (Seq.index ci 0) + v (Seq.index ci 4) /\
     (v (Seq.index co 4)) % 8380417 == ((v (Seq.index ci 4) - v (Seq.index ci 0)) * v $zeta * 8265825) % 8380417 /\
     v (Seq.index co 1) == v (Seq.index ci 1) + v (Seq.index ci 5) /\
     (v (Seq.index co 5)) % 8380417 == ((v (Seq.index ci 5) - v (Seq.index ci 1)) * v $zeta * 8265825) % 8380417 /\
     v (Seq.index co 2) == v (Seq.index ci 2) + v (Seq.index ci 6) /\
     (v (Seq.index co 6)) % 8380417 == ((v (Seq.index ci 6) - v (Seq.index ci 2)) * v $zeta * 8265825) % 8380417 /\
     v (Seq.index co 3) == v (Seq.index ci 3) + v (Seq.index ci 7) /\
     (v (Seq.index co 7)) % 8380417 == ((v (Seq.index ci 7) - v (Seq.index ci 3)) * v $zeta * 8265825) % 8380417)
"#) )]
pub fn simd_unit_invert_ntt_at_layer_2(simd_unit: &mut Coefficients, zeta: i32) {
    simd_unit_inv_ntt_step(simd_unit, zeta, 0, 4);
    simd_unit_inv_ntt_step(simd_unit, zeta, 1, 4);
    simd_unit_inv_ntt_step(simd_unit, zeta, 2, 4);
    simd_unit_inv_ntt_step(simd_unit, zeta, 3, 4);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (2 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 0) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn invert_ntt_at_layer_0(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[inline(always)]
    #[hax_lib::fstar::options("--z3rlimit 100")]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        v index < v $SIMD_UNITS_IN_RING_ELEMENT /\
        Spec.Utils.is_i32b_array_opaque (v $FIELD_MAX) 
            (Seq.index ${re} (v index)).f_values /\
        Spec.Utils.is_i32b 4190208 $zeta0 /\
        Spec.Utils.is_i32b 4190208 $zeta1 /\
        Spec.Utils.is_i32b 4190208 $zeta2 /\
        Spec.Utils.is_i32b 4190208 $zeta3
    "#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.modifies1_32 ${re} ${re}_future $index /\
        Spec.Utils.is_i32b_array_opaque (2* v $FIELD_MAX)
            (Seq.index ${re}_future (v index)).f_values /\
        unit_fe_post_inv_l0 (Seq.index ${re} (v $index)).f_values
                        (Seq.index ${re}_future (v $index)).f_values
                        $zeta0 $zeta1 $zeta2 $zeta3
     "#))]
    fn round(
        re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT],
        index: usize,
        zeta0: i32,
        zeta1: i32,
        zeta2: i32,
        zeta3: i32,
    ) {
        hax_lib::fstar!(
            "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
        );
        simd_unit_invert_ntt_at_layer_0(&mut re[index], zeta0, zeta1, zeta2, zeta3);
        hax_lib::fstar!("reveal_opaque (`%unit_fe_post_inv_l0) unit_fe_post_inv_l0");
    }

    #[cfg(hax)]
    let orig_re = re.clone();

    round(re, 0, 1976782, -846154, 1400424, 3937738);
    round(re, 1, -1362209, -48306, 3919660, -554416);
    round(re, 2, -3545687, 1612842, -976891, 183443);
    round(re, 3, -2286327, -420899, -2235985, -2939036);
    round(re, 4, -3833893, -260646, -1104333, -1667432);
    round(re, 5, 1910376, -1803090, 1723600, -426683);
    round(re, 6, 472078, 1717735, -975884, 2213111);
    round(re, 7, 269760, 3866901, 3523897, -3038916);
    round(re, 8, -1799107, -3694233, 1652634, 810149);
    round(re, 9, 3014001, 1616392, 162844, -3183426);
    round(re, 10, -1207385, 185531, 3369112, 1957272);
    round(re, 11, -164721, 2454455, 2432395, -2013608);
    round(re, 12, -3776993, 594136, -3724270, -2584293);
    round(re, 13, -1846953, -1671176, -2831860, -542412);
    round(re, 14, 3406031, 2235880, 777191, 1500165);
    round(re, 15, -1374803, -2546312, 1917081, -1279661);
    round(re, 16, -1962642, 3306115, 1312455, -451100);
    round(re, 17, -1430225, -3318210, 1237275, -1333058);
    round(re, 18, -1050970, 1903435, 1869119, -2994039);
    round(re, 19, -3548272, 2635921, 1250494, -3767016);
    round(re, 20, 1595974, 2486353, 1247620, 4055324);
    round(re, 21, 1265009, -2590150, 2691481, 2842341);
    round(re, 22, 203044, 1735879, -3342277, 3437287);
    round(re, 23, 4108315, -2437823, 286988, 342297);
    round(re, 24, -3595838, -768622, -525098, -3556995);
    round(re, 25, 3207046, 2031748, -3122442, -655327);
    round(re, 26, -522500, -43260, -1613174, 495491);
    round(re, 27, 819034, 909542, 1859098, 900702);
    round(re, 28, -3193378, -1197226, -3759364, -3520352);
    round(re, 29, 3513181, -1235728, 2434439, 266997);
    round(re, 30, -3562462, -2446433, 2244091, -3342478);
    round(re, 31, 3817976, 2316500, 3407706, 2091667);
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
lemma_inv_l0_driver_compose (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${orig_re}) (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re})
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (2 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (4 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 1) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn invert_ntt_at_layer_1(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[inline(always)]
    #[hax_lib::fstar::options("--z3rlimit 100")]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        v index < v $SIMD_UNITS_IN_RING_ELEMENT /\
        Spec.Utils.is_i32b_array_opaque (2 * v $FIELD_MAX) 
            (Seq.index ${re} (v index)).f_values /\
        Spec.Utils.is_i32b 4190208 $zeta_00 /\
        Spec.Utils.is_i32b 4190208 $zeta_01
    "#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.modifies1_32 ${re} ${re}_future $index /\
        Spec.Utils.is_i32b_array_opaque (4 * v $FIELD_MAX)
            (Seq.index ${re}_future (v $index)).f_values /\
        unit_fe_post_inv_l1 (Seq.index ${re} (v $index)).f_values
                        (Seq.index ${re}_future (v $index)).f_values $zeta_00 $zeta_01
     "#))]
    fn round(
        re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT],
        index: usize,
        zeta_00: i32,
        zeta_01: i32,
    ) {
        hax_lib::fstar!(
            "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
        );
        simd_unit_invert_ntt_at_layer_1(&mut re[index], zeta_00, zeta_01);
        hax_lib::fstar!("reveal_opaque (`%unit_fe_post_inv_l1) unit_fe_post_inv_l1");
    }

    #[cfg(hax)]
    let orig_re = re.clone();

    round(re, 0, 3839961, -3628969);
    round(re, 1, -3881060, -3019102);
    round(re, 2, -1439742, -812732);
    round(re, 3, -1584928, 1285669);
    round(re, 4, 1341330, 1315589);
    round(re, 5, -177440, -2409325);
    round(re, 6, -1851402, 3159746);
    round(re, 7, -3553272, 189548);
    round(re, 8, -1316856, 759969);
    round(re, 9, -210977, 2389356);
    round(re, 10, -3249728, 1653064);
    round(re, 11, -8578, -3724342);
    round(re, 12, 3958618, 904516);
    round(re, 13, -1100098, 44288);
    round(re, 14, 3097992, 508951);
    round(re, 15, 264944, -3343383);
    round(re, 16, -1430430, 1852771);
    round(re, 17, 1349076, -381987);
    round(re, 18, -1308169, -22981);
    round(re, 19, -1228525, -671102);
    round(re, 20, -2477047, -411027);
    round(re, 21, -3693493, -2967645);
    round(re, 22, 2715295, 2147896);
    round(re, 23, -983419, 3412210);
    round(re, 24, 126922, -3632928);
    round(re, 25, -3157330, -3190144);
    round(re, 26, -1000202, -4083598);
    round(re, 27, 1939314, -1257611);
    round(re, 28, -1585221, 2176455);
    round(re, 29, 3475950, -1452451);
    round(re, 30, -3041255, -3677745);
    round(re, 31, -1528703, -3930395);
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
lemma_inv_l1_driver_compose (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${orig_re}) (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re})
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (4 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (8 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 2) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn invert_ntt_at_layer_2(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[inline(always)]
    #[hax_lib::fstar::options("--z3rlimit 100")]
    #[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
    #[hax_lib::requires(fstar!(r#"
        v index < v $SIMD_UNITS_IN_RING_ELEMENT /\
        Spec.Utils.is_i32b_array_opaque (4 * v $FIELD_MAX) 
            (Seq.index ${re} (v index)).f_values /\
        Spec.Utils.is_i32b 4190208 $zeta1
    "#))]
    #[hax_lib::ensures(|_| fstar!(r#"
        Spec.Utils.modifies1_32 ${re} ${re}_future $index /\
        Spec.Utils.is_i32b_array_opaque (8 * v $FIELD_MAX)
            (Seq.index ${re}_future (v $index)).f_values /\
        unit_fe_post_inv_l2 (Seq.index ${re} (v $index)).f_values
                        (Seq.index ${re}_future (v $index)).f_values $zeta1
     "#))]
    fn round(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT], index: usize, zeta1: i32) {
        hax_lib::fstar!(
            "reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) (Spec.Utils.is_i32b_array_opaque)"
        );
        simd_unit_invert_ntt_at_layer_2(&mut re[index], zeta1);
        hax_lib::fstar!("reveal_opaque (`%unit_fe_post_inv_l2) unit_fe_post_inv_l2");
    }

    #[cfg(hax)]
    let orig_re = re.clone();

    round(re, 0, -2797779);
    round(re, 1, 2071892);
    round(re, 2, -2556880);
    round(re, 3, 3900724);
    round(re, 4, 3881043);
    round(re, 5, 954230);
    round(re, 6, 531354);
    round(re, 7, 811944);
    round(re, 8, 3699596);
    round(re, 9, -1600420);
    round(re, 10, -2140649);
    round(re, 11, 3507263);
    round(re, 12, -3821735);
    round(re, 13, 3505694);
    round(re, 14, -1643818);
    round(re, 15, -1699267);
    round(re, 16, -539299);
    round(re, 17, 2348700);
    round(re, 18, -300467);
    round(re, 19, 3539968);
    round(re, 20, -2867647);
    round(re, 21, 3574422);
    round(re, 22, -3043716);
    round(re, 23, -3861115);
    round(re, 24, 3915439);
    round(re, 25, -2537516);
    round(re, 26, -3592148);
    round(re, 27, -1661693);
    round(re, 28, 3530437);
    round(re, 29, 3077325);
    round(re, 30, 95776);
    round(re, 31, 2706023);
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
lemma_inv_l2_driver_compose (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${orig_re}) (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re})
"#);
}

#[inline(always)]
#[hax_lib::fstar::before(
    r#"
let layer_bound_factor (step_by:usize) : n:nat{n <= 128} =
    match step_by with
    | MkInt 1 -> 8
    | MkInt 2 -> 16
    | MkInt 4 -> 32
    | MkInt 8 -> 64
    | MkInt 16 -> 128
    | _ -> 128"#
)]
#[hax_lib::fstar::before(r#"
(* ---- INVERSE cross-chunk GS-FE atom + bridge lemmas (Phase C).
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
"#)]
#[hax_lib::fstar::before(r#"
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
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 3) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re orig_re in
    let fut = Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r (31 - u / 2)) in
    Libcrux_ml_dsa.Simd.Portable.Ntt.forall32_elim_1d (fun u -> (u % 2 == 0) ==>
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
"#)]
#[hax_lib::fstar::before(r#"
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
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 4) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re orig_re in
    let fut = Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r (15 - u / 4)) in
    Libcrux_ml_dsa.Simd.Portable.Ntt.forall32_elim_1d (fun u -> (u % 4 < 2) ==>
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
"#)]
#[hax_lib::fstar::before(r#"
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
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 5) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re orig_re in
    let fut = Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r (7 - u / 8)) in
    Libcrux_ml_dsa.Simd.Portable.Ntt.forall32_elim_1d (fun u -> (u % 8 < 4) ==>
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
"#)]
#[hax_lib::fstar::before(r#"
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
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 6) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re orig_re in
    let fut = Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r (3 - u / 16)) in
    Libcrux_ml_dsa.Simd.Portable.Ntt.forall32_elim_1d (fun u -> (u % 16 < 8) ==>
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
"#)]
#[hax_lib::fstar::before(r#"
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
          (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re orig_re) in
           let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re re) in
           let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 7) in
           forall (i: nat). i < 256 ==>
             (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417))
  = let orig = Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re orig_re in
    let fut = Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re re in
    let zm (u: nat{u < 32}) : (z: i32{Spec.Utils.is_i32b 4190208 z}) =
        mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1) in
    Libcrux_ml_dsa.Simd.Portable.Ntt.forall32_elim_1d (fun u -> (u % 32 < 16) ==>
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
"#)]
#[hax_lib::fstar::options("--z3rlimit 600 --split_queries always")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    (v $STEP_BY > 0) /\
    (v $OFFSET + v $STEP_BY < v $SIMD_UNITS_IN_RING_ELEMENT) /\
    (v $OFFSET + 2 * v $STEP_BY <= v $SIMD_UNITS_IN_RING_ELEMENT) /\
    (Spec.Utils.forall32 (fun i -> (i >= v $OFFSET /\ i < (v $OFFSET + 2 * v $STEP_BY)) ==>
              Spec.Utils.is_i32b_array_opaque 
                ((layer_bound_factor $STEP_BY) * v $FIELD_MAX)
                (Seq.index ${re} i).f_values)) /\
    Spec.Utils.is_i32b 4190208 $ZETA
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Spec.Utils.modifies_range_32 ${re} ${re}_future $OFFSET (${OFFSET + STEP_BY + STEP_BY}) /\
    (Spec.Utils.forall32 (fun i -> (i >= v $OFFSET /\ i < (v $OFFSET + 2 * v $STEP_BY)) ==>
              Spec.Utils.is_i32b_array_opaque
                (2 * (layer_bound_factor $STEP_BY) * v $FIELD_MAX)
                (Seq.index ${re}_future i).f_values)) /\
    (Spec.Utils.forall32 (fun u -> (u >= v $OFFSET /\ u < v $OFFSET + v $STEP_BY) ==>
              unit_fe_post_inv_cross (Seq.index ${re} u).f_values
                                     (Seq.index ${re} (u + v $STEP_BY)).f_values
                                     (Seq.index ${re}_future u).f_values
                                     (Seq.index ${re}_future (u + v $STEP_BY)).f_values
                                     $ZETA))
"#))]
fn outer_3_plus<const OFFSET: usize, const STEP_BY: usize, const ZETA: i32>(
    re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT],
) {
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
                    (2 * (layer_bound_factor $STEP_BY) * v $FIELD_MAX)
                    (Seq.index ${re} i).f_values)) /\
            (Spec.Utils.forall32 (fun u -> (u >= v $OFFSET /\ u < v $j) ==>
                unit_fe_post_inv_cross (Seq.index $orig_re u).f_values
                                       (Seq.index $orig_re (u + v $STEP_BY)).f_values
                                       (Seq.index ${re} u).f_values
                                       (Seq.index ${re} (u + v $STEP_BY)).f_values
                                       $ZETA))
        "#
        ));

        let rej = re[j];
        let rejs = re[j + STEP_BY];
        arithmetic::add(&mut re[j], &rejs);
        arithmetic::subtract(&mut re[j + STEP_BY], &rej);
        #[cfg(hax)]
        let tmp = re[j + STEP_BY];
        arithmetic::montgomery_multiply_by_constant(&mut re[j + STEP_BY], ZETA);

        hax_lib::fstar!("Spec.Utils.is_i32b_array_larger
            (v $FIELD_MAX) (2 * (layer_bound_factor $STEP_BY) * v $FIELD_MAX) (Seq.index re (v j + v v_STEP_BY)).f_values");
        // Discharge the cross-unit inverse FE atom via the clean bridge lemma:
        // ci_lo/ci_hi = orig_re[j]/[j+STEP_BY] (== re[j]/[j+STEP_BY] at iter start, frame);
        // co_lo/co_hi = re[j]/[j+STEP_BY]; tmp = re[j+STEP_BY] after subtract (= ci_hi - ci_lo).
        // add post: co_lo = ci_lo + ci_hi; sub post: tmp = ci_hi - ci_lo;
        // mmbc post: co_hi = mont_mul(tmp, ZETA) and the mod-q relation.
        hax_lib::fstar!(
            r#"lemma_round_inv_cross_intro
                 (Seq.index $orig_re (v $j)).f_values
                 (Seq.index $orig_re (v $j + v $STEP_BY)).f_values
                 (Seq.index ${re} (v $j)).f_values
                 (Seq.index ${re} (v $j + v $STEP_BY)).f_values
                 ${tmp}.f_values
                 $ZETA"#
        );
    }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --using_facts_from '* -Hacspec_ml_dsa.createi_lemma'")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (8 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (16 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 3) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn invert_ntt_at_layer_3(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 8; // 1 << LAYER;
    const STEP_BY: usize = 1; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 280005>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 4010497>(re);
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -19422>(re);
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1757237>(re);
    outer_3_plus::<{ (4 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -3277672>(re);
    outer_3_plus::<{ (5 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1399561>(re);
    outer_3_plus::<{ (6 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -3859737>(re);
    outer_3_plus::<{ (7 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2118186>(re);
    outer_3_plus::<{ (8 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2108549>(re);
    outer_3_plus::<{ (9 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2619752>(re);
    outer_3_plus::<{ (10 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1119584>(re);
    outer_3_plus::<{ (11 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -549488>(re);
    outer_3_plus::<{ (12 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3585928>(re);
    outer_3_plus::<{ (13 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -1079900>(re);
    outer_3_plus::<{ (14 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1024112>(re);
    outer_3_plus::<{ (15 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2725464>(re);

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
// Flat-asserts: discharge each even-u inverse cross atom (orig_re vs final re) in
// ISOLATION (one frame each) so the compose lemma's forall32 precondition is
// assembled from 16 ground facts instead of a forall32-of-forall32 cascade.
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 0).f_values (Seq.index ${orig_re} 1).f_values (Seq.index ${re} 0).f_values (Seq.index ${re} 1).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 31)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 2).f_values (Seq.index ${orig_re} 3).f_values (Seq.index ${re} 2).f_values (Seq.index ${re} 3).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 30)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 4).f_values (Seq.index ${orig_re} 5).f_values (Seq.index ${re} 4).f_values (Seq.index ${re} 5).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 29)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 6).f_values (Seq.index ${orig_re} 7).f_values (Seq.index ${re} 6).f_values (Seq.index ${re} 7).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 28)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 8).f_values (Seq.index ${orig_re} 9).f_values (Seq.index ${re} 8).f_values (Seq.index ${re} 9).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 27)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 10).f_values (Seq.index ${orig_re} 11).f_values (Seq.index ${re} 10).f_values (Seq.index ${re} 11).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 26)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 12).f_values (Seq.index ${orig_re} 13).f_values (Seq.index ${re} 12).f_values (Seq.index ${re} 13).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 25)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 14).f_values (Seq.index ${orig_re} 15).f_values (Seq.index ${re} 14).f_values (Seq.index ${re} 15).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 24)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 16).f_values (Seq.index ${orig_re} 17).f_values (Seq.index ${re} 16).f_values (Seq.index ${re} 17).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 23)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 18).f_values (Seq.index ${orig_re} 19).f_values (Seq.index ${re} 18).f_values (Seq.index ${re} 19).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 22)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 20).f_values (Seq.index ${orig_re} 21).f_values (Seq.index ${re} 20).f_values (Seq.index ${re} 21).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 21)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 22).f_values (Seq.index ${orig_re} 23).f_values (Seq.index ${re} 22).f_values (Seq.index ${re} 23).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 20)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 24).f_values (Seq.index ${orig_re} 25).f_values (Seq.index ${re} 24).f_values (Seq.index ${re} 25).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 19)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 26).f_values (Seq.index ${orig_re} 27).f_values (Seq.index ${re} 26).f_values (Seq.index ${re} 27).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 18)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 28).f_values (Seq.index ${orig_re} 29).f_values (Seq.index ${re} 28).f_values (Seq.index ${re} 29).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 17)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 30).f_values (Seq.index ${orig_re} 31).f_values (Seq.index ${re} 30).f_values (Seq.index ${re} 31).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 16)));
lemma_inv_l3_cross_driver_compose ${orig_re} ${re}
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --using_facts_from '* -Hacspec_ml_dsa.createi_lemma'")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (16 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (32 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 4) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn invert_ntt_at_layer_4(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 16; // 1 << LAYER;
    const STEP_BY: usize = 2; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2680103>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3111497>(re);
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2884855>(re);
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 3119733>(re);
    outer_3_plus::<{ (4 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2091905>(re);
    outer_3_plus::<{ (5 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -359251>(re);
    outer_3_plus::<{ (6 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 2353451>(re);
    outer_3_plus::<{ (7 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 1826347>(re);

    hax_lib::fstar!(r#"
assert_norm (Spec.MLDSA.NttConstants.zeta_r 8 == 1826347);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 9 == 2353451);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 10 == (-359251));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 11 == (-2091905));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 12 == 3119733);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 13 == (-2884855));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 14 == 3111497);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 15 == 2680103);
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 0).f_values (Seq.index ${orig_re} 2).f_values (Seq.index ${re} 0).f_values (Seq.index ${re} 2).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 15)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 1).f_values (Seq.index ${orig_re} 3).f_values (Seq.index ${re} 1).f_values (Seq.index ${re} 3).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 15)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 4).f_values (Seq.index ${orig_re} 6).f_values (Seq.index ${re} 4).f_values (Seq.index ${re} 6).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 14)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 5).f_values (Seq.index ${orig_re} 7).f_values (Seq.index ${re} 5).f_values (Seq.index ${re} 7).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 14)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 8).f_values (Seq.index ${orig_re} 10).f_values (Seq.index ${re} 8).f_values (Seq.index ${re} 10).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 13)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 9).f_values (Seq.index ${orig_re} 11).f_values (Seq.index ${re} 9).f_values (Seq.index ${re} 11).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 13)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 12).f_values (Seq.index ${orig_re} 14).f_values (Seq.index ${re} 12).f_values (Seq.index ${re} 14).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 12)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 13).f_values (Seq.index ${orig_re} 15).f_values (Seq.index ${re} 13).f_values (Seq.index ${re} 15).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 12)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 16).f_values (Seq.index ${orig_re} 18).f_values (Seq.index ${re} 16).f_values (Seq.index ${re} 18).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 11)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 17).f_values (Seq.index ${orig_re} 19).f_values (Seq.index ${re} 17).f_values (Seq.index ${re} 19).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 11)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 20).f_values (Seq.index ${orig_re} 22).f_values (Seq.index ${re} 20).f_values (Seq.index ${re} 22).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 10)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 21).f_values (Seq.index ${orig_re} 23).f_values (Seq.index ${re} 21).f_values (Seq.index ${re} 23).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 10)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 24).f_values (Seq.index ${orig_re} 26).f_values (Seq.index ${re} 24).f_values (Seq.index ${re} 26).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 9)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 25).f_values (Seq.index ${orig_re} 27).f_values (Seq.index ${re} 25).f_values (Seq.index ${re} 27).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 9)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 28).f_values (Seq.index ${orig_re} 30).f_values (Seq.index ${re} 28).f_values (Seq.index ${re} 30).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 8)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 29).f_values (Seq.index ${orig_re} 31).f_values (Seq.index ${re} 29).f_values (Seq.index ${re} 31).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 8)));
lemma_inv_l4_cross_driver_compose ${orig_re} ${re}
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --using_facts_from '* -Hacspec_ml_dsa.createi_lemma'")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (32 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (64 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 5) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn invert_ntt_at_layer_5(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 32; // 1 << LAYER;
    const STEP_BY: usize = 4; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 466468>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -876248>(re);
    outer_3_plus::<{ (2 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -777960>(re);
    outer_3_plus::<{ (3 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 237124>(re);

    hax_lib::fstar!(r#"
assert_norm (Spec.MLDSA.NttConstants.zeta_r 4 == 237124);
assert_norm (Spec.MLDSA.NttConstants.zeta_r 5 == (-777960));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 6 == (-876248));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 7 == 466468);
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 0).f_values (Seq.index ${orig_re} 4).f_values (Seq.index ${re} 0).f_values (Seq.index ${re} 4).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 7)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 1).f_values (Seq.index ${orig_re} 5).f_values (Seq.index ${re} 1).f_values (Seq.index ${re} 5).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 7)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 2).f_values (Seq.index ${orig_re} 6).f_values (Seq.index ${re} 2).f_values (Seq.index ${re} 6).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 7)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 3).f_values (Seq.index ${orig_re} 7).f_values (Seq.index ${re} 3).f_values (Seq.index ${re} 7).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 7)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 8).f_values (Seq.index ${orig_re} 12).f_values (Seq.index ${re} 8).f_values (Seq.index ${re} 12).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 6)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 9).f_values (Seq.index ${orig_re} 13).f_values (Seq.index ${re} 9).f_values (Seq.index ${re} 13).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 6)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 10).f_values (Seq.index ${orig_re} 14).f_values (Seq.index ${re} 10).f_values (Seq.index ${re} 14).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 6)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 11).f_values (Seq.index ${orig_re} 15).f_values (Seq.index ${re} 11).f_values (Seq.index ${re} 15).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 6)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 16).f_values (Seq.index ${orig_re} 20).f_values (Seq.index ${re} 16).f_values (Seq.index ${re} 20).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 5)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 17).f_values (Seq.index ${orig_re} 21).f_values (Seq.index ${re} 17).f_values (Seq.index ${re} 21).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 5)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 18).f_values (Seq.index ${orig_re} 22).f_values (Seq.index ${re} 18).f_values (Seq.index ${re} 22).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 5)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 19).f_values (Seq.index ${orig_re} 23).f_values (Seq.index ${re} 19).f_values (Seq.index ${re} 23).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 5)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 24).f_values (Seq.index ${orig_re} 28).f_values (Seq.index ${re} 24).f_values (Seq.index ${re} 28).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 4)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 25).f_values (Seq.index ${orig_re} 29).f_values (Seq.index ${re} 25).f_values (Seq.index ${re} 29).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 4)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 26).f_values (Seq.index ${orig_re} 30).f_values (Seq.index ${re} 26).f_values (Seq.index ${re} 30).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 4)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 27).f_values (Seq.index ${orig_re} 31).f_values (Seq.index ${re} 27).f_values (Seq.index ${re} 31).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 4)));
lemma_inv_l5_cross_driver_compose ${orig_re} ${re}
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --using_facts_from '* -Hacspec_ml_dsa.createi_lemma'")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (64 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (128 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 6) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn invert_ntt_at_layer_6(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 64; // 1 << LAYER;
    const STEP_BY: usize = 8; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -518909>(re);
    outer_3_plus::<{ (1 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, -2608894>(re);

    hax_lib::fstar!(r#"
assert_norm (Spec.MLDSA.NttConstants.zeta_r 2 == (-2608894));
assert_norm (Spec.MLDSA.NttConstants.zeta_r 3 == (-518909));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 0).f_values (Seq.index ${orig_re} 8).f_values (Seq.index ${re} 0).f_values (Seq.index ${re} 8).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 3)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 1).f_values (Seq.index ${orig_re} 9).f_values (Seq.index ${re} 1).f_values (Seq.index ${re} 9).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 3)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 2).f_values (Seq.index ${orig_re} 10).f_values (Seq.index ${re} 2).f_values (Seq.index ${re} 10).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 3)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 3).f_values (Seq.index ${orig_re} 11).f_values (Seq.index ${re} 3).f_values (Seq.index ${re} 11).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 3)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 4).f_values (Seq.index ${orig_re} 12).f_values (Seq.index ${re} 4).f_values (Seq.index ${re} 12).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 3)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 5).f_values (Seq.index ${orig_re} 13).f_values (Seq.index ${re} 5).f_values (Seq.index ${re} 13).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 3)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 6).f_values (Seq.index ${orig_re} 14).f_values (Seq.index ${re} 6).f_values (Seq.index ${re} 14).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 3)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 7).f_values (Seq.index ${orig_re} 15).f_values (Seq.index ${re} 7).f_values (Seq.index ${re} 15).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 3)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 16).f_values (Seq.index ${orig_re} 24).f_values (Seq.index ${re} 16).f_values (Seq.index ${re} 24).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 2)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 17).f_values (Seq.index ${orig_re} 25).f_values (Seq.index ${re} 17).f_values (Seq.index ${re} 25).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 2)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 18).f_values (Seq.index ${orig_re} 26).f_values (Seq.index ${re} 18).f_values (Seq.index ${re} 26).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 2)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 19).f_values (Seq.index ${orig_re} 27).f_values (Seq.index ${re} 19).f_values (Seq.index ${re} 27).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 2)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 20).f_values (Seq.index ${orig_re} 28).f_values (Seq.index ${re} 20).f_values (Seq.index ${re} 28).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 2)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 21).f_values (Seq.index ${orig_re} 29).f_values (Seq.index ${re} 21).f_values (Seq.index ${re} 29).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 2)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 22).f_values (Seq.index ${orig_re} 30).f_values (Seq.index ${re} 22).f_values (Seq.index ${re} 30).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 2)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 23).f_values (Seq.index ${orig_re} 31).f_values (Seq.index ${re} 23).f_values (Seq.index ${re} 31).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 2)));
lemma_inv_l6_cross_driver_compose ${orig_re} ${re}
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always --using_facts_from '* -Hacspec_ml_dsa.createi_lemma'")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (128 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (256 * v $FIELD_MAX) ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}_future) in
     let spec = Hacspec_ml_dsa.Ntt.intt_layer in_flat (mk_usize 7) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 == (v (Seq.index spec i)) % 8380417)
"#) )]
fn invert_ntt_at_layer_7(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    const STEP: usize = 128; // 1 << LAYER;
    const STEP_BY: usize = 16; // step / COEFFICIENTS_IN_SIMD_UNIT;

    #[cfg(hax)]
    let orig_re = re.clone();

    outer_3_plus::<{ (0 * STEP * 2) / COEFFICIENTS_IN_SIMD_UNIT }, STEP_BY, 25847>(re);

    hax_lib::fstar!(r#"
assert_norm (Spec.MLDSA.NttConstants.zeta_r 1 == 25847);
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 0).f_values (Seq.index ${orig_re} 16).f_values (Seq.index ${re} 0).f_values (Seq.index ${re} 16).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 1).f_values (Seq.index ${orig_re} 17).f_values (Seq.index ${re} 1).f_values (Seq.index ${re} 17).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 2).f_values (Seq.index ${orig_re} 18).f_values (Seq.index ${re} 2).f_values (Seq.index ${re} 18).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 3).f_values (Seq.index ${orig_re} 19).f_values (Seq.index ${re} 3).f_values (Seq.index ${re} 19).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 4).f_values (Seq.index ${orig_re} 20).f_values (Seq.index ${re} 4).f_values (Seq.index ${re} 20).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 5).f_values (Seq.index ${orig_re} 21).f_values (Seq.index ${re} 5).f_values (Seq.index ${re} 21).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 6).f_values (Seq.index ${orig_re} 22).f_values (Seq.index ${re} 6).f_values (Seq.index ${re} 22).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 7).f_values (Seq.index ${orig_re} 23).f_values (Seq.index ${re} 7).f_values (Seq.index ${re} 23).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 8).f_values (Seq.index ${orig_re} 24).f_values (Seq.index ${re} 8).f_values (Seq.index ${re} 24).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 9).f_values (Seq.index ${orig_re} 25).f_values (Seq.index ${re} 9).f_values (Seq.index ${re} 25).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 10).f_values (Seq.index ${orig_re} 26).f_values (Seq.index ${re} 10).f_values (Seq.index ${re} 26).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 11).f_values (Seq.index ${orig_re} 27).f_values (Seq.index ${re} 11).f_values (Seq.index ${re} 27).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 12).f_values (Seq.index ${orig_re} 28).f_values (Seq.index ${re} 12).f_values (Seq.index ${re} 28).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 13).f_values (Seq.index ${orig_re} 29).f_values (Seq.index ${re} 13).f_values (Seq.index ${re} 29).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 14).f_values (Seq.index ${orig_re} 30).f_values (Seq.index ${re} 14).f_values (Seq.index ${re} 30).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
assert (unit_fe_post_inv_cross (Seq.index ${orig_re} 15).f_values (Seq.index ${orig_re} 31).f_values (Seq.index ${re} 15).f_values (Seq.index ${re} 31).f_values (mk_i32 (Spec.MLDSA.NttConstants.zeta_r 1)));
lemma_inv_l7_cross_driver_compose ${orig_re} ${re}
"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
// Phase E scaling-bridge lemmas (validated in scratch).
#[hax_lib::fstar::before(r#"
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

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --z3refresh"
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

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100 --z3refresh"
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
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re orig_re) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re fut_re) in
     forall (j:nat). j < 256 ==>
       (v (Seq.index out_flat j)) % 8380417 == (16382 * v (Seq.index in_flat j)) % 8380417))
  = let ci = Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re orig_re in
    let co = Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re fut_re in
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
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (256 * v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial 4211177 ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}_future) in
     forall (j:nat). j < 256 ==>
       (v (Seq.index out_flat j)) % 8380417 == (16382 * v (Seq.index in_flat j)) % 8380417)
"#) )]
fn scale_montgomery(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[cfg(hax)]
    let orig = re.clone();
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial) (Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (256 * v $FIELD_MAX) ${re})"#);
    for i in 0..re.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"
            (forall (k:nat).
              k < v $i ==>
              Spec.Utils.is_i32b_array_opaque 4211177
                (Seq.index $re k).f_values /\
              chunk_scaled (Seq.index ${orig} k).f_values (Seq.index $re k).f_values) /\
            (forall (k:nat).
              (k >= v $i /\ k < 32) ==>
              Spec.Utils.is_i32b_array_opaque (256 * v $FIELD_MAX)
                (Seq.index $re k).f_values /\
              (Seq.index $re k) == (Seq.index ${orig} k)))
        "#
        ));
        // After invert_ntt_at_layer, elements are of the form a * MONTGOMERY_R^{-1}
        // we multiply by (MONTGOMERY_R^2) * (1/2^8) mod Q = 41,978 to both:
        //
        // - Divide the elements by 256 and
        // - Convert the elements form montgomery domain to the standard domain.
        arithmetic::montgomery_multiply_by_constant(&mut re[i], 41_978);
        // Tight 4211177 output bound: the input chunk orig[i] is bounded by
        // 256*FIELD_MAX (k>=i invariant clause) and montgomery's per-lane post
        // gives re[i] = mont_mul(orig[i], 41978); the tight-bound lemma reduces
        // 256*FIELD_MAX to the centered 4211177.
        hax_lib::fstar!(r#"lemma_scale_chunk_tight_bound (Seq.index ${orig} (v $i)).f_values (Seq.index $re (v $i)).f_values"#);
        // montgomery's per-lane (Spec.MLDSA.Math.mod_q ...) post -> the opaque
        // chunk_scaled atom (input chunk is orig[i] by the k>=i clause).
        hax_lib::fstar!(r#"lemma_establish_chunk_scaled (Seq.index ${orig} (v $i)).f_values (Seq.index $re (v $i)).f_values"#);
    }
    hax_lib::fstar!(r#"reveal_opaque (`%Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial) (Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial 4211177 ${re})"#);
    // Lift the per-chunk 16382-scaling (loop post) to the flat-poly view.
    hax_lib::fstar!(r#"lemma_scale_driver ${orig} ${re}"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::fstar::before(r#"
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

(* ---- Phase E: scaling wrapper.  out ≡ to_mont(intt in) (mod q), with
   to_mont x = mod_q(R·x), R = 2^32 mod q = 4193792.  The impl stays in the
   Montgomery domain (mont_mul by 41978 = R·256^{-1}), so it is off the clean
   intt by R. *)
let to_mont (p: t_Array i32 (mk_usize 256)) : t_Array i32 (mk_usize 256) =
  Hacspec_ml_dsa.createi #i32 (mk_usize 256) #(usize -> i32)
    (fun i -> Hacspec_ml_dsa.Arithmetic.mod_q (mk_i64 4193792 *! (cast (p.[i] <: i32) <: i64)))

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
(* STANDALONE clean-context arithmetic: a == mod_q(8347681*b) ==> mod_q(R*a) ≡ 16382*b (mod q) *)
let lemma_scale_arith (a b : i32) : Lemma
  (requires a == Hacspec_ml_dsa.Arithmetic.mod_q (mk_i64 8347681 *! (cast b <: i64)))
  (ensures (v (Hacspec_ml_dsa.Arithmetic.mod_q (mk_i64 4193792 *! (cast a <: i64)))) % 8380417
           == (16382 * v b) % 8380417)
  = Hacspec_ml_dsa.Commute.Chunk.lemma_mod_q_v (mk_i64 4193792 *! (cast a <: i64));
    Hacspec_ml_dsa.Commute.Chunk.lemma_mod_q_v (mk_i64 8347681 *! (cast b <: i64));
    FStar.Math.Lemmas.lemma_mod_mul_distr_r 4193792 (v a) 8380417;
    FStar.Math.Lemmas.lemma_mod_mul_distr_r 4193792 (8347681 * v b) 8380417;
    assert_norm ((4193792 * 8347681) % 8380417 == 16382);
    FStar.Math.Lemmas.lemma_mod_mul_distr_r 16382 (v b) 8380417
#pop-options

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
"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial (v $FIELD_MAX) ${re}
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Simd.Portable.Ntt.is_i32b_polynomial 4211177 ${re}_future /\
    (let in_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}) in
     let out_flat = Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}_future) in
     forall (i: nat). i < 256 ==>
       (v (Seq.index out_flat i)) % 8380417 ==
       (v (Seq.index (to_mont (Hacspec_ml_dsa.Ntt.intt in_flat)) i)) % 8380417)
"#) )]
pub(crate) fn invert_ntt_montgomery(re: &mut [Coefficients; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[cfg(hax)]
    let s0 = re.clone();
    invert_ntt_at_layer_0(re);
    #[cfg(hax)]
    let s1 = re.clone();
    invert_ntt_at_layer_1(re);
    #[cfg(hax)]
    let s2 = re.clone();
    invert_ntt_at_layer_2(re);
    #[cfg(hax)]
    let s3 = re.clone();
    invert_ntt_at_layer_3(re);
    #[cfg(hax)]
    let s4 = re.clone();
    invert_ntt_at_layer_4(re);
    #[cfg(hax)]
    let s5 = re.clone();
    invert_ntt_at_layer_5(re);
    #[cfg(hax)]
    let s6 = re.clone();
    invert_ntt_at_layer_6(re);
    #[cfg(hax)]
    let s7 = re.clone();
    invert_ntt_at_layer_7(re);
    #[cfg(hax)]
    let s8 = re.clone();
    hax_lib::fstar!(r#"
lemma_intt_compose_8
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${s0}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${s1}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${s2}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${s3}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${s4}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${s5}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${s6}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${s7}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${s8}))
"#);
    scale_montgomery(re);
    // out ≡ 16382·s8 (scale_montgomery) and s8 ≡ intt_unscaled(s0) (compose)
    // ⟹ out ≡ to_mont(intt s0) (mod q).
    hax_lib::fstar!(r#"
lemma_invert_top
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${s0}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${s8}))
  (Hacspec_ml_dsa.Commute.Chunk.simd_units_to_array (Libcrux_ml_dsa.Simd.Portable.Ntt.chunks_of_re ${re}))
"#);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ntt::reduce, polynomial::PolynomialRingElement, simd::traits::FIELD_MODULUS};

    #[test]
    fn inv_ntt_unreduced_max() {
        let mut re = PolynomialRingElement::<crate::simd::portable::PortableSIMDUnit>::zero();
        for simd_unit in re.simd_units.iter_mut() {
            for i in 0..8 {
                simd_unit.values[i] = FIELD_MODULUS + (FIELD_MODULUS / 1024) + 6;
            }
        }
        let _ = core::hint::black_box(invert_ntt_montgomery(&mut re.simd_units));
    }

    #[test]
    #[should_panic]
    fn inv_ntt_unreduced_panic() {
        let mut re1 = PolynomialRingElement::<crate::simd::portable::PortableSIMDUnit>::zero();
        for simd_unit in re1.simd_units.iter_mut() {
            for i in 0..8 {
                simd_unit.values[i] = FIELD_MODULUS + (FIELD_MODULUS / 1024) + 7;
            }
        }
        core::hint::black_box(invert_ntt_montgomery(&mut re1.simd_units)); // In debug mode this will panic since the intermediate values overflow.

        let mut re2 = PolynomialRingElement::<crate::simd::portable::PortableSIMDUnit>::zero();
        for simd_unit in re2.simd_units.iter_mut() {
            for i in 0..8 {
                simd_unit.values[i] = FIELD_MODULUS + (FIELD_MODULUS / 1024) + 7;
            }
        }
        reduce(&mut re2);
        core::hint::black_box(invert_ntt_montgomery(&mut re2.simd_units));

        // In release mode, one of the checks below will panic, since
        // the intermediate values silently overflowed, producing an
        // incorrect result.
        for (i, simd_unit) in re2.simd_units.iter().enumerate() {
            for (j, reference_coeff) in simd_unit.values.iter().enumerate() {
                assert_eq!(*reference_coeff, re1.simd_units[i].values[j])
            }
        }
    }

    #[test]
    fn inv_ntt_reduced() {
        let mut re = PolynomialRingElement::<crate::simd::portable::PortableSIMDUnit>::zero();
        for simd_unit in re.simd_units.iter_mut() {
            for i in 0..8 {
                simd_unit.values[i] = FIELD_MODULUS + (FIELD_MODULUS / 1024) + 7;
            }
        }
        reduce(&mut re);
        let _ = core::hint::black_box(invert_ntt_montgomery(&mut re.simd_units));
    }

    #[test]
    fn inv_ntt_reduced_large() {
        let mut re = PolynomialRingElement::<crate::simd::portable::PortableSIMDUnit>::zero();
        for simd_unit in re.simd_units.iter_mut() {
            for i in 0..8 {
                simd_unit.values[i] = FIELD_MODULUS * 8;
            }
        }
        reduce(&mut re);
        let _ = core::hint::black_box(invert_ntt_montgomery(&mut re.simd_units));
    }
}

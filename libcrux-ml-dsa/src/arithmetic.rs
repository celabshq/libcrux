use crate::{
    constants::{Gamma2, COEFFICIENTS_IN_RING_ELEMENT},
    polynomial::PolynomialRingElement,
    simd::traits::Operations,
};

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"v $bound > 0 /\
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $vector"#))]
pub(crate) fn vector_infinity_norm_exceeds<SIMDUnit: Operations>(
    vector: &[PolynomialRingElement<SIMDUnit>],
    bound: i32,
) -> bool {
    let mut result = false;
    for i in 0..vector.len() {
        // Bridge the slice-level FIELD_MAX bound to the per-row poly bound (and unfold
        // it) so infinity_norm_exceeds' per-lane forall precondition discharges.
        hax_lib::fstar!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup (mk_usize 8380416) $vector (v $i);
               reveal_opaque (`%Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly) (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416) (Seq.index $vector (v $i)))"#
        );
        result = result || vector[i].infinity_norm_exceeds(bound);
    }
    result
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"v $SHIFT_BY == 13 /\
        (forall i. forall j.
            v (Seq.index (i0._super_i2.f_repr (Seq.index re.f_simd_units i)) j) >= 0 /\
            v (Seq.index (i0._super_i2.f_repr (Seq.index re.f_simd_units i)) j) <= 261631)"#))]
#[hax_lib::ensures(|_| fstar!(r#"
        (forall (i:nat). i < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${crate::simd::traits::specs::FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index ${re}_future.f_simd_units i)))"#))]
pub(crate) fn shift_left_then_reduce<SIMDUnit: Operations, const SHIFT_BY: i32>(
    re: &mut PolynomialRingElement<SIMDUnit>,
) {
    #[cfg(hax)]
    let old_re = re.clone();

    for i in 0..re.simd_units.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v i <= 32 /\
              (forall (j:nat). j >= v i /\ j < 32 ==>
                  Seq.index re.f_simd_units j == Seq.index old_re.f_simd_units j) /\
              (forall (j:nat). j < v i ==>
                  Spec.Utils.is_i32b_array_opaque (v ${crate::simd::traits::specs::FIELD_MAX})
                      (i0._super_i2.f_repr (Seq.index re.f_simd_units j)))"#
        ));

        SIMDUnit::shift_left_then_reduce::<SHIFT_BY>(&mut re.simd_units[i]);
        hax_lib::fstar!(r#"
          let lane_post (j:nat{j < 8}) :
            Lemma (Spec.Utils.is_i32b 8380416
                     (Seq.index (i0._super_i2.f_repr (Seq.index ${re}.f_simd_units (v ${i}))) j)) =
            Libcrux_ml_dsa.Simd.Traits.Specs.lemma_shift_left_then_reduce_lane_lookup
              (Seq.index (i0._super_i2.f_repr (Seq.index ${old_re}.f_simd_units (v ${i}))) j)
              (Seq.index (i0._super_i2.f_repr (Seq.index ${re}.f_simd_units (v ${i}))) j)
          in
          Classical.forall_intro lane_post;
          reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) Spec.Utils.is_i32b_array_opaque
        "#);
    }
}

// Pre/post opacified: pre is `is_bounded_poly_slice FIELD_MAX` (was bare
// double-forall on per-simd-unit FIELD_MAX); t0 post is
// `is_bounded_poly_slice (pow2 12)` (closed form, was bare double-forall);
// t1 post is `is_lane_range_poly_slice 0 1023` (was bare triple-forall +
// `forall8`).  All three forms reuse existing opaque atoms in
// polynomial.rs::spec — no new predicates.  Body remains admitted.
#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"${t0.len()} == ${t1.len()} /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice
        (mk_usize 8380416) $t0"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${t0}_future == Seq.length t0 /\
    Seq.length ${t1}_future == Seq.length t1 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice
        (mk_usize 4096) ${t0}_future /\
    Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice
        (mk_usize 0) (mk_usize 1023) ${t1}_future"#))]
#[hax_lib::fstar::verification_status(panic_free)]
pub(crate) fn power2round_vector<SIMDUnit: Operations>(
    t0: &mut [PolynomialRingElement<SIMDUnit>],
    t1: &mut [PolynomialRingElement<SIMDUnit>],
) {
    // ADMIT: hax cannot extract simultaneous &mut t0[i] / &mut t1[i] borrows in a
    // loop body in a way that supports a loop invariant. Body proof deferred until
    // hax upstream supports this pattern.
    hax_lib::fstar!("admit ()");
    for i in 0..t0.len() {
        power2round_one_ring_element::<SIMDUnit>(&mut t0[i], &mut t1[i]);
    }
}

#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    (forall (j:nat). j < 32 ==>
      Spec.Utils.is_i32b_array_opaque
        (v ${crate::simd::traits::specs::FIELD_MAX})
        (i0._super_i2.f_repr (Seq.index t0.f_simd_units j)))"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    (forall (j:nat). j < 32 ==>
      Spec.Utils.is_i32b_array_opaque (pow2 12)
        (i0._super_i2.f_repr (Seq.index ${t0}_future.f_simd_units j)) /\
      Spec.Utils.forall8 (fun (k:nat{k < 8}) ->
        let t1j = Seq.index ${t1}_future.Libcrux_ml_dsa.Polynomial.f_simd_units j in
        v (Seq.index (i0._super_i2.f_repr t1j) k) >= 0 /\
        v (Seq.index (i0._super_i2.f_repr t1j) k) < pow2 10))"#))]
fn power2round_one_ring_element<SIMDUnit: Operations>(
    t0: &mut PolynomialRingElement<SIMDUnit>,
    t1: &mut PolynomialRingElement<SIMDUnit>,
) {
    for j in 0..t0.simd_units.len() {
        hax_lib::loop_invariant!(|j: usize| fstar!(
            r#"v j <= 32 /\
              (forall (k:nat{k < 32}). k >= v j ==>
                Spec.Utils.is_i32b_array_opaque
                  (v ${crate::simd::traits::specs::FIELD_MAX})
                  (i0._super_i2.f_repr (Seq.index t0.f_simd_units k))) /\
              (forall (k:nat{k < 32}). k < v j ==>
                Spec.Utils.is_i32b_array_opaque (pow2 12)
                  (i0._super_i2.f_repr (Seq.index t0.f_simd_units k)) /\
                Spec.Utils.forall8 (fun (m:nat{m < 8}) ->
                  let t1k = Seq.index t1.Libcrux_ml_dsa.Polynomial.f_simd_units k in
                  v (Seq.index (i0._super_i2.f_repr t1k) m) >= 0 /\
                  v (Seq.index (i0._super_i2.f_repr t1k) m) < pow2 10))"#
        ));
        SIMDUnit::power2round(&mut t0.simd_units[j], &mut t1.simd_units[j]);
    }
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/ 
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
         ${t.len()} == dimension /\
         ${low.len()} == dimension /\
         ${high.len()} == dimension /\
         Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $t"#))]
pub(crate) fn decompose_vector<SIMDUnit: Operations>(
    dimension: usize,
    gamma2: Gamma2,
    t: &[PolynomialRingElement<SIMDUnit>],
    low: &mut [PolynomialRingElement<SIMDUnit>],
    high: &mut [PolynomialRingElement<SIMDUnit>],
) {
    for i in 0..dimension {
        hax_lib::loop_invariant!(|i: usize| low.len() == dimension && high.len() == dimension);

        for j in 0..low[0].simd_units.len() {
            hax_lib::loop_invariant!(|i: usize| low.len() == dimension && high.len() == dimension);

            // Bridge the slice-level FIELD_MAX bound on t down to the per-lane bound
            // that decompose's precondition needs on t[i].simd_units[j].
            hax_lib::fstar!(
                r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup (mk_usize 8380416) $t (v $i);
                   Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_lookup (mk_usize 8380416) (Seq.index $t (v $i)) (v $j)"#
            );

            SIMDUnit::decompose(
                gamma2,
                &t[i].simd_units[j],
                &mut low[i].simd_units[j],
                &mut high[i].simd_units[j],
            );
        }
    }
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
         ${low.len()} == ${high.len()} /\
         ${low.len()} == ${hint.len()} /\
         v (${low.len()}) <= 8 /\
         Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $low /\
         Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $high"#))]
pub(crate) fn make_hint<SIMDUnit: Operations>(
    low: &[PolynomialRingElement<SIMDUnit>],
    high: &[PolynomialRingElement<SIMDUnit>],
    gamma2: i32,
    hint: &mut [[i32; COEFFICIENTS_IN_RING_ELEMENT]],
) -> usize {
    let mut true_hints = 0;
    let mut hint_simd = PolynomialRingElement::<SIMDUnit>::zero();

    for i in 0..low.len() {
        hax_lib::loop_invariant!(|i: usize| true_hints <= 256 * i && hint.len() == low.len());

        for j in 0..hint_simd.simd_units.len() {
            hax_lib::loop_invariant!(|j: usize| true_hints <= 256 * i + 8 * j);

            // Bridge the slice-level FIELD_MAX bound down to the per-lane bound that
            // compute_hint's precondition needs: slice -> per-row poly -> per-lane.
            hax_lib::fstar!(
                r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup (mk_usize 8380416) $low (v $i);
                   Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup (mk_usize 8380416) $high (v $i);
                   Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_lookup (mk_usize 8380416) (Seq.index $low (v $i)) (v $j);
                   Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_lookup (mk_usize 8380416) (Seq.index $high (v $i)) (v $j)"#
            );

            let one_hints_count = SIMDUnit::compute_hint(
                &low[i].simd_units[j],
                &high[i].simd_units[j],
                gamma2,
                &mut hint_simd.simd_units[j],
            );

            true_hints += one_hints_count;
        }

        hint[i] = hint_simd.to_i32_array();
    }

    true_hints
}

#[inline(always)]
#[hax_lib::fstar::before(r#"let use_hint_bound (gamma2:i32) : usize = if v gamma2 = v Libcrux_ml_dsa.Constants.v_GAMMA2_V95_232_ then mk_usize 44 else mk_usize 16"#)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
         ${hint.len()} == ${re_vector.len()} /\
         v (${hint.len()}) <= 8 /\
         (forall i. forall j.
            (v (Seq.index (Seq.index ${hint} i) j) == 0 \/
             v (Seq.index (Seq.index ${hint} i) j) == 1)) /\
         Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) ${re_vector}}"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${re_vector}_future == Seq.length re_vector /\
    (forall (i:nat). i < Seq.length ${re_vector}_future ==>
      (forall (j:nat). j < 32 ==>
        ((v $gamma2 == v ${crate::constants::GAMMA2_V95_232} ==>
            Spec.Utils.is_i32b_array_opaque 44
              (i0._super_i2.f_repr
                (Seq.index (Seq.index ${re_vector}_future i).Libcrux_ml_dsa.Polynomial.f_simd_units j))) /\
         (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} ==>
            Spec.Utils.is_i32b_array_opaque 16
              (i0._super_i2.f_repr
                (Seq.index (Seq.index ${re_vector}_future i).Libcrux_ml_dsa.Polynomial.f_simd_units j))))))"#))]
pub(crate) fn use_hint<SIMDUnit: Operations>(
    gamma2: Gamma2,
    hint: &[[i32; COEFFICIENTS_IN_RING_ELEMENT]],
    re_vector: &mut [PolynomialRingElement<SIMDUnit>],
) {
    #[cfg(hax)]
    let old_rv: &[PolynomialRingElement<SIMDUnit>] = re_vector.to_vec().as_slice();
    // Bridge the per-(i,j) FIELD_MAX requires to is_bounded_poly_slice on the
    // entry snapshot old_rv, and seed the (empty) processed range.
    hax_lib::fstar!(r#"
        let _:Prims.unit =
          let aux (k:nat{k < Seq.length old_rv}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                     (mk_usize 8380416) (Seq.index old_rv k)) =
            assert (Seq.index old_rv k == Seq.index $re_vector k);
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              (mk_usize 8380416) $re_vector k
          in Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
          (mk_usize 8380416) old_rv;
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
          (use_hint_bound $gamma2) (mk_usize 0) (mk_usize 0) $re_vector"#);
    for i in 0..re_vector.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(r#"
            v ${i} <= Seq.length $re_vector /\
            Seq.length $re_vector == Seq.length old_rv /\
            Seq.length $re_vector == Seq.length $hint /\
            Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
              (use_hint_bound $gamma2) (mk_usize 0) ${i} $re_vector /\
            (forall (k:nat). v ${i} <= k /\ k < Seq.length $re_vector ==>
              Seq.index $re_vector k == Seq.index old_rv k) /\
            Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice
              (mk_usize 8380416) old_rv"#));
        // re_vector[i] == old_rv[i] (tail frame) and old_rv[i] is FIELD_MAX-bounded
        // (slice lookup), so re_vector[i] is FIELD_MAX-bounded for the inner loop.
        hax_lib::fstar!(r#"
            assert (Seq.index $re_vector (v ${i}) == Seq.index old_rv (v ${i}));
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              (mk_usize 8380416) old_rv (v ${i})"#);
        let mut tmp = PolynomialRingElement::zero();
        PolynomialRingElement::<SIMDUnit>::from_i32_array(&hint[i], &mut tmp);

        // Bridge: from_i32_array gives `f_repr tmp.simd_units[kk] == hint[i][kk*8..(kk+1)*8]`,
        // and `hint[i]` is binary (function pre), so each tmp simd-unit is a binary array.
        hax_lib::fstar!(r#"
            let aux (kk:nat{kk < 32}) : Lemma
                (Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque
                   (i0._super_i2.f_repr (Seq.index ${tmp}.f_simd_units kk))) =
              let r = i0._super_i2.f_repr (Seq.index ${tmp}.f_simd_units kk) in
              introduce forall (m:nat{m < 8}). (v (Seq.index r m) == 0 \/ v (Seq.index r m) == 1)
              with (Seq.lemma_index_slice (Seq.index $hint (v ${i})) (kk * 8) ((kk + 1) * 8) m);
              Libcrux_ml_dsa.Simd.Traits.Specs.lemma_is_binary_array_8_intro r
            in Classical.forall_intro aux"#);

        for j in 0..re_vector[0].simd_units.len() {
            hax_lib::loop_invariant!(|j: usize| fstar!(r#"
                v ${j} <= 32 /\
                Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                  (mk_usize 8380416) (Seq.index $re_vector (v ${i})) /\
                (forall (jj:nat). jj < v ${j} ==>
                  Spec.Utils.is_i32b_array_opaque (v (use_hint_bound $gamma2))
                    (i0._super_i2.f_repr (Seq.index ${tmp}.f_simd_units jj))) /\
                (forall (jj:nat). v ${j} <= jj /\ jj < 32 ==>
                  Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque
                    (i0._super_i2.f_repr (Seq.index ${tmp}.f_simd_units jj)))"#));
            // Bridge: is_bounded_poly FIELD_MAX re_vector[i] (inv) gives the
            // per-lane FIELD_MAX bound on re_vector[i].simd_units[j] that the
            // use_hint trait pre requires (explicit lookup, not the flaky SMTPat).
            hax_lib::fstar!(r#"
                Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_lookup
                  (mk_usize 8380416) (Seq.index $re_vector (v ${i})) (v ${j})"#);
            SIMDUnit::use_hint(gamma2, &re_vector[i].simd_units[j], &mut tmp.simd_units[j]);
        }
        // After inner loop: all 32 tmp simd-units are is_i32b_array_opaque b_g; lift to is_bounded_poly b_g tmp.
        hax_lib::fstar!(r#"
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_intro
              (use_hint_bound $gamma2) ${tmp}"#);
        // Snapshot pre-update re_vector so the carryover extend lemma can name arr_old.
        #[cfg(hax)]
        let iter_start: &[PolynomialRingElement<SIMDUnit>] = re_vector.to_vec().as_slice();
        re_vector[i] = tmp;
        // Re-establish the processed range at i+1 via the standalone extend lemma
        // (verified in clean context, avoiding cascade pollution here).
        hax_lib::fstar!(r#"
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_extend_after_update
              (use_hint_bound $gamma2) ${i} iter_start $re_vector"#);
    }
    // Bridge the final processed range to the per-(i,j) gamma2-conditional ensures.
    hax_lib::fstar!(r#"
        let aux (k:nat{k < Seq.length ${re_vector}}) :
          Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                   (use_hint_bound $gamma2) (Seq.index $re_vector k)) =
          Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
            (use_hint_bound $gamma2) (mk_usize 0) (Core_models.Slice.impl__len $re_vector) $re_vector k
        in Classical.forall_intro aux"#);
}

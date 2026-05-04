use crate::{
    constants::{Gamma2, COEFFICIENTS_IN_RING_ELEMENT},
    polynomial::PolynomialRingElement,
    simd::traits::Operations,
};

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"v $bound > 0 /\ 
        (forall i. forall j. Spec.Utils.is_i32b_array_opaque 
            (v ${crate::simd::traits::specs::FIELD_MAX}) 
            (i0._super_i2.f_repr (Seq.index (Seq.index vector i).f_simd_units j)))"#))]
pub(crate) fn vector_infinity_norm_exceeds<SIMDUnit: Operations>(
    vector: &[PolynomialRingElement<SIMDUnit>],
    bound: i32,
) -> bool {
    let mut result = false;
    for i in 0..vector.len() {
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

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"${t0.len()} == ${t1.len()} /\
    (forall (i:nat). i < Seq.length t0 ==>
      (forall (j:nat). j < 32 ==>
        Spec.Utils.is_i32b_array_opaque
          (v ${crate::simd::traits::specs::FIELD_MAX})
          (i0._super_i2.f_repr (Seq.index (Seq.index t0 i).f_simd_units j))))"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${t0}_future == Seq.length t0 /\
    Seq.length ${t1}_future == Seq.length t1 /\
    (forall (i:nat). i < Seq.length ${t0}_future ==>
      (forall (j:nat). j < 32 ==>
        Spec.Utils.is_i32b_array_opaque (pow2 12)
          (i0._super_i2.f_repr (Seq.index (Seq.index ${t0}_future i).f_simd_units j)) /\
        Spec.Utils.forall8 (fun (k:nat{k < 8}) ->
          let t1ij = Seq.index (Seq.index ${t1}_future i).Libcrux_ml_dsa.Polynomial.f_simd_units j in
          v (Seq.index (i0._super_i2.f_repr t1ij) k) >= 0 /\
          v (Seq.index (i0._super_i2.f_repr t1ij) k) < pow2 10)))"#))]
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
         (forall i. forall j. 
            Spec.Utils.is_i32b_array_opaque 
            (v ${crate::simd::traits::specs::FIELD_MAX}) 
            (i0._super_i2.f_repr (Seq.index (Seq.index t i).f_simd_units j)))"#))]
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
         (forall (i:nat). i < Seq.length low ==>
           (forall (j:nat). j < 32 ==>
             Spec.Utils.is_i32b_array_opaque (v ${crate::simd::traits::specs::FIELD_MAX})
               (i0._super_i2.f_repr (Seq.index (Seq.index low i).f_simd_units j)) /\
             Spec.Utils.is_i32b_array_opaque (v ${crate::simd::traits::specs::FIELD_MAX})
               (i0._super_i2.f_repr (Seq.index (Seq.index high i).f_simd_units j))))"#))]
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
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
         ${hint.len()} == ${re_vector.len()} /\
         v (${hint.len()}) <= 8 /\
         (forall i. forall j.
            (v (Seq.index (Seq.index ${hint} i) j) == 0 \/
             v (Seq.index (Seq.index ${hint} i) j) == 1)) /\
         (forall i. forall j.
            Spec.Utils.is_i32b_array_opaque
            (v ${crate::simd::traits::specs::FIELD_MAX})
            (i0._super_i2.f_repr (Seq.index (Seq.index re_vector i).f_simd_units j)))"#))]
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
    hax_lib::fstar!("admit ()");
    for i in 0..re_vector.len() {
        let mut tmp = PolynomialRingElement::zero();
        PolynomialRingElement::<SIMDUnit>::from_i32_array(&hint[i], &mut tmp);

        for j in 0..re_vector[0].simd_units.len() {
            SIMDUnit::use_hint(gamma2, &re_vector[i].simd_units[j], &mut tmp.simd_units[j]);
        }
        re_vector[i] = tmp;
    }
}

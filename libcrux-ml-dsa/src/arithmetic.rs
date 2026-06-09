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
/// CAUTION: This function must only be called with inputs for
/// which it is safe to leak the index of a violating coefficient.
///
/// For all norm checks during ML-DSA signature generation it is
/// safe to leak the index of a violating coefficient.
pub(crate) fn vector_infinity_norm_exceeds<SIMDUnit: Operations>(
    vector: &[PolynomialRingElement<SIMDUnit>],
    bound: i32,
) -> bool {
    let mut result = false;
    for i in 0..vector.len() {
        // XXX: We can't use the non-short-circuiting core::ops::BitOr
        // here, because of an issue in hax-lib v0.3.6.
        // (cf. https://github.com/cryspen/libcrux/issues/1437)
        //
        // Using the short-circuiting OR here is safe, even if it
        // leaks the index of a coefficient that violates the norm
        // check. See
        // `PolynomialRingElemen<SIMDUnit>::infinity_norm_exceeds` for
        // more details.
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
pub(crate) fn shift_left_then_reduce<SIMDUnit: Operations, const SHIFT_BY: i32>(
    re: &mut PolynomialRingElement<SIMDUnit>,
) {
    #[cfg(hax)]
    let old_re = re.clone();

    for i in 0..re.simd_units.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"
            forall j. j >= v i ==> Seq.index re.f_simd_units j == Seq.index old_re.f_simd_units j"#
        ));

        SIMDUnit::shift_left_then_reduce::<SHIFT_BY>(&mut re.simd_units[i]);
    }
}

#[inline(always)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::requires(fstar!(r#"${t.len()} == ${t1.len()} /\
    (forall i. forall j. 
        Spec.Utils.is_i32b_array_opaque 
        (v ${crate::simd::traits::specs::FIELD_MAX}) 
        (i0._super_i2.f_repr (Seq.index (Seq.index t i).f_simd_units j)))"#))]
pub(crate) fn power2round_vector<SIMDUnit: Operations>(
    t: &mut [PolynomialRingElement<SIMDUnit>],
    t1: &mut [PolynomialRingElement<SIMDUnit>],
) {
    #[cfg(hax)]
    let (old_t, old_t1) = { (t.to_vec(), t1.to_vec()) };

    #[cfg(hax)]
    use hax_lib::prop::*;

    for i in 0..t.len() {
        hax_lib::loop_invariant!(|i: usize| Prop::from(
            t.len() == old_t.len() && t1.len() == old_t1.len()
        )
        .and(forall(|j: usize| implies(
            j >= i && j < old_t.len(),
            fstar!(r#"${t[j]} == ${old_t[j]} /\ ${t1[j]} == ${old_t1[j]}"#)
        ))));

        for j in 0..t[i].simd_units.len() {
            hax_lib::loop_invariant!(|j: usize| Prop::from(
                t.len() == old_t.len() && t1.len() == old_t1.len()
            )
            .and(forall(|j: usize| implies(
                j > i && j < old_t.len(),
                fstar!(r#"${t[j]} == ${old_t[j]} /\ ${t1[j]} == ${old_t1[j]}"#)
            )))
            .and(forall(|k: usize| implies(
                k >= j && k < crate::simd::traits::SIMD_UNITS_IN_RING_ELEMENT,
                fstar!(
                    r#"
                        ${t[i].simd_units[k]} == ${old_t[i].simd_units[k]} /\
                        ${t1[i].simd_units[k]} == ${old_t1[i].simd_units[k]}
                    "#
                )
            ))));

            SIMDUnit::power2round(&mut t[i].simd_units[j], &mut t1[i].simd_units[j]);
        }
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
#[hax_lib::requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/ 
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
         ${low.len()} == ${high.len()} /\ 
         ${low.len()} == ${hint.len()} /\
         v (${low.len()}) <= 8"#))]
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
pub(crate) fn use_hint<SIMDUnit: Operations>(
    gamma2: Gamma2,
    hint: &[[i32; COEFFICIENTS_IN_RING_ELEMENT]],
    re_vector: &mut [PolynomialRingElement<SIMDUnit>],
) {
    #[cfg(hax)]
    let old_re_vector = re_vector.to_vec();

    for i in 0..re_vector.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"
            ${re_vector.len()} == ${hint.len()} /\
            (forall j. j >= v i ==> 
                (Seq.index re_vector j == Seq.index old_re_vector j))
            "#
        ));

        let mut tmp = PolynomialRingElement::zero();
        PolynomialRingElement::<SIMDUnit>::from_i32_array(&hint[i], &mut tmp);

        for j in 0..re_vector[0].simd_units.len() {
            hax_lib::loop_invariant!(|j: usize| fstar!(
                r#"
                ${re_vector.len()} == ${hint.len()} /\
                (forall j. j > v i ==> 
                    (Seq.index re_vector j == Seq.index old_re_vector j)) /\
                (forall k. k >= v j ==> 
                    (Seq.index (Seq.index re_vector (v i)).f_simd_units k ==
                     Seq.index (Seq.index old_re_vector (v i)).f_simd_units k))
                "#
            ));

            SIMDUnit::use_hint(gamma2, &re_vector[i].simd_units[j], &mut tmp.simd_units[j]);
        }
        re_vector[i] = tmp;
    }
}

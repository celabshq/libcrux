use crate::{
    arithmetic::shift_left_then_reduce,
    constants::BITS_IN_LOWER_PART_OF_T,
    ntt::{invert_ntt_montgomery, ntt, ntt_multiply_montgomery, reduce},
    polynomial::PolynomialRingElement,
    simd::traits::Operations,
};

#[cfg(hax)]
extern crate alloc;
#[cfg(hax)]
use crate::polynomial::spec;
#[cfg(hax)]
use crate::simd::traits::specs::*;

/// Bound-additive variant of `PolynomialRingElement::add` whose pre/post are
/// stated in `is_bounded_poly` form. Mirrors the ML-KEM `add_to_ring_element`
/// recipe: the per-simd-unit ↔ poly-level bridge is contained here so the
/// matrix loop bodies compose at the polynomial level instead of carrying the
/// `forall j:nat. j < 32 ==> is_i32b_array_opaque ...` quantifier through
/// every iteration. `rhs` is fixed at `FIELD_MAX = 8380416` to match the
/// post-condition of `ntt_multiply_montgomery` (the only kind of summand
/// passed in). Lives in `matrix.rs` rather than `impl PolynomialRingElement`
/// to avoid a `Polynomial → Polynomial.Spec → Polynomial` extraction cycle.
#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    v $_bound + 8380416 <= 2147483647 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly $_bound $myself /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416) $rhs"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
        ($_bound +! mk_usize 8380416) ${myself}_future"#))]
fn add_to_ring_element<SIMDUnit: Operations>(
    myself: &mut PolynomialRingElement<SIMDUnit>,
    rhs: &PolynomialRingElement<SIMDUnit>,
    _bound: usize,
) {
    PolynomialRingElement::<SIMDUnit>::add_bounded(myself, _bound, rhs, 8380416);
    hax_lib::fstar!(r#"
      Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_intro
        ($_bound +! mk_usize 8380416) $myself
    "#);
}

// Not inlining this makes key generation 3x slower for avx2. Only `inline` this
// function costs 30% performance too.
/// Compute InvertNTT(Â ◦ ŝ₁) + s₂
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(fstar!(r#"
    Seq.length $a_as_ntt == v $rows_in_a * v $columns_in_a /\
    Seq.length $s1_ntt == v $columns_in_a /\
    Seq.length $s1_s2 == v $columns_in_a + v $rows_in_a /\
    Seq.length $result == v $rows_in_a /\
    v $rows_in_a <= 8 /\
    v $columns_in_a <= 7 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $a_as_ntt /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $s1_ntt /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $s1_s2
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${a_as_ntt}_future == Seq.length $a_as_ntt /\
    Seq.length ${result}_future == Seq.length $result /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 16760832) ${result}_future
"#))]
pub(crate) fn compute_as1_plus_s2<SIMDUnit: Operations>(
    rows_in_a: usize,
    columns_in_a: usize,
    a_as_ntt: &mut [PolynomialRingElement<SIMDUnit>],
    s1_ntt: &[PolynomialRingElement<SIMDUnit>],
    s1_s2: &[PolynomialRingElement<SIMDUnit>],
    result: &mut [PolynomialRingElement<SIMDUnit>],
) {
    hax_lib::fstar!("admit ()");
    for i in 0..rows_in_a {
        for j in 0..columns_in_a {
            ntt_multiply_montgomery::<SIMDUnit>(&mut a_as_ntt[i * columns_in_a + j], &s1_ntt[j]);
            PolynomialRingElement::add(&mut result[i], &mut a_as_ntt[i * columns_in_a + j]);
        }
    }

    for i in 0..result.len() {
        // We do a Barrett reduction here, since the absolute value of
        // `columns_in_a` additions might be as large as `columns_in_a
        // * FIELD_MODULUS`, and `invert_ntt_montgomery` expects
        // coefficients of size at most `FIELD_MODULUS`.
        reduce(&mut result[i]);
        invert_ntt_montgomery::<SIMDUnit>(&mut result[i]);
        PolynomialRingElement::add(&mut result[i], &s1_s2[columns_in_a + i]);
    }
}

/// Compute InvertNTT(Â ◦ ŷ)
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(fstar!(r#"
    Seq.length $matrix == v $rows_in_a * v $columns_in_a /\
    Seq.length $mask == v $columns_in_a /\
    Seq.length $result == v $rows_in_a /\
    v $rows_in_a <= 8 /\
    v $columns_in_a <= 7 /\
    (forall (k:nat). k < v $rows_in_a * v $columns_in_a ==>
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416)
            (Seq.index $matrix k)) /\
    (forall (k:nat). k < v $columns_in_a ==>
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416)
            (Seq.index $mask k))
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${result}_future == Seq.length $result /\
    (forall (k:nat). k < v $rows_in_a ==>
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416)
            (Seq.index ${result}_future k))
"#))]
pub(crate) fn compute_matrix_x_mask<SIMDUnit: Operations>(
    rows_in_a: usize,
    columns_in_a: usize,
    matrix: &[PolynomialRingElement<SIMDUnit>],
    mask: &[PolynomialRingElement<SIMDUnit>],
    result: &mut [PolynomialRingElement<SIMDUnit>],
) {
    for i in 0..rows_in_a {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v $i <= v $rows_in_a /\
              Seq.length $result == v $rows_in_a /\
              (forall (k:nat). k < v $i ==>
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416)
                      (Seq.index $result k))"#
        ));

        // Snapshot before zeroing — used as the frame anchor inside the
        // inner loop and by reduce/invert_ntt_montgomery (which only mutate
        // result[i]).
        #[cfg(hax)]
        let old_result: &[PolynomialRingElement<SIMDUnit>] = result.to_vec().as_slice();

        result[i] = PolynomialRingElement::<SIMDUnit>::zero();
        // Lift `zero`'s per-lane `is_i32b_array_opaque 0` post to
        // `is_bounded_poly 0 result[i]` so the inner loop inv fires at j=0.
        hax_lib::fstar!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_intro
                 (mk_usize 0) (Seq.index $result (v $i))"#
        );

        for j in 0..columns_in_a {
            hax_lib::loop_invariant!(|j: usize| fstar!(
                r#"v $j <= v $columns_in_a /\
                  v $i < v $rows_in_a /\
                  Seq.length $result == v $rows_in_a /\
                  (forall (k:nat). k < v $rows_in_a /\ k <> v $i ==>
                      Seq.index $result k == Seq.index old_result k) /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                      ($j *! mk_usize 8380416) (Seq.index $result (v $i))"#
            ));

            // We could make `matrix` mutable here and avoid copying.
            // But that would require sampling the matrix multiple times.
            // That's not worth it.
            let mut product = mask[j];
            ntt_multiply_montgomery::<SIMDUnit>(&mut product, &matrix[i * columns_in_a + j]);
            add_to_ring_element::<SIMDUnit>(&mut result[i], &product, j * 8380416);
            // `add_to_ring_element` post is `is_bounded_poly (j *! FM +! FM)`;
            // distributivity_add_left bridges to `(j +! 1) *! FM` for the
            // inner inv at j+1 (same usize value, opacity stops congruence
            // from firing without the explicit lemma instance).
            hax_lib::fstar!(
                r#"assert (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                            ($j *! mk_usize 8380416 +! mk_usize 8380416)
                            (Seq.index $result (v $i)));
                   Math.Lemmas.distributivity_add_left (v $j) 1 8380416"#
            );
        }

        // After inner loop: is_bounded_poly (cols * FIELD_MAX) result[i].
        // reduce wants is_bounded_poly 2_143_289_343 — weaken via monotonicity.
        hax_lib::fstar!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher
                 ($columns_in_a *! mk_usize 8380416) (mk_usize 2143289343)
                 (Seq.index $result (v $i))"#
        );
        // We do a Barrett reduction here, since the absolute value of
        // `columns_in_a` additions might be as large as `columns_in_a
        // * FIELD_MODULUS`, and `invert_ntt_montgomery` expects
        // coefficients of size at most `FIELD_MODULUS`.
        reduce::<SIMDUnit>(&mut result[i]);
        invert_ntt_montgomery::<SIMDUnit>(&mut result[i]);
        // invert_ntt_montgomery post is is_bounded_poly 4_211_177;
        // weaken to FIELD_MAX for the outer invariant.
        hax_lib::fstar!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher
                 (mk_usize 4211177) (mk_usize 8380416)
                 (Seq.index $result (v $i))"#
        );
        // Re-establish the outer invariant at i+1: spell each piece out so
        // Z3 doesn't have to chain frame + Leibniz + final lemma_higher all
        // at once at the function-exit subtyping check.
        hax_lib::fstar!(
            r#"assert (forall (k:nat). k < v $i ==>
                   Seq.index $result k == Seq.index old_result k);
               assert (forall (k:nat). k < v $i ==>
                   Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416)
                       (Seq.index $result k));
               assert (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416)
                   (Seq.index $result (v $i)));
               assert (forall (k:nat). k < v $i + 1 ==>
                   Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416)
                       (Seq.index $result k))"#
        );
    }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --split_queries always")]
#[hax_lib::requires(fstar!(r#"
    (forall (k:nat). k < Seq.length $vector ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index $vector k).f_simd_units j)))) /\
    (forall (j:nat). j < 32 ==>
        Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
            (i0._super_i2.f_repr (Seq.index ring_element.f_simd_units j)))
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${vector}_future == Seq.length $vector /\
    (forall (k:nat{k < Seq.length ${vector}_future}).
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index ${vector}_future k).f_simd_units j))))
"#))]
pub(crate) fn vector_times_ring_element<SIMDUnit: Operations>(
    vector: &mut [PolynomialRingElement<SIMDUnit>],
    ring_element: &PolynomialRingElement<SIMDUnit>,
) {
    #[cfg(hax)]
    let e_vector_orig: &[PolynomialRingElement<SIMDUnit>] = vector.to_vec().as_slice();
    // ntt.rs now requires is_bounded_poly form; lift the per-lane FIELD_MAX
    // pre on `ring_element` (immutable, so once is enough).
    hax_lib::fstar!(
        r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_intro
             (mk_usize 8380416) $ring_element"#
    );
    for i in 0..vector.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v i <= Seq.length vector /\
              Seq.length vector == Seq.length e_vector_orig /\
              (forall (j:nat). j < 32 ==>
                  Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                      (i0._super_i2.f_repr (Seq.index ring_element.f_simd_units j))) /\
              (forall (k:nat). k < v i ==>
                  (forall (j:nat). j < 32 ==>
                      Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                          (i0._super_i2.f_repr (Seq.index (Seq.index vector k).f_simd_units j)))) /\
              (forall (k:nat). v i <= k /\ k < Seq.length vector ==>
                  Seq.index vector k == Seq.index e_vector_orig k)"#
        ));
        ntt_multiply_montgomery(&mut vector[i], ring_element);
        invert_ntt_montgomery(&mut vector[i]);
        // invert_ntt_montgomery's new post is `is_bounded_poly 4_211_177`;
        // weaken to FIELD_MAX, then explicitly extract per-lane form for the
        // loop invariant (SMTPat lemma_is_bounded_poly_lookup fires via the
        // explicit forall_intro across the 32 lanes).
        hax_lib::fstar!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher
                 (mk_usize 4211177) (mk_usize 8380416)
                 (Seq.index $vector (v $i));
               let lemma_lift (j:nat{j < 32}) :
                 Lemma (Spec.Utils.is_i32b_array_opaque
                          (v Libcrux_ml_dsa.Simd.Traits.Specs.v_FIELD_MAX)
                          (i0._super_i2.f_repr
                             (Seq.index (Seq.index $vector (v $i)).f_simd_units j))) =
                 Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_lookup
                   (mk_usize 8380416) (Seq.index $vector (v $i)) j
               in
               Classical.forall_intro lemma_lift"#
        );
    }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning")]
#[hax_lib::requires(fstar!(r#"
    Seq.length $lhs >= v $dimension /\
    Seq.length $rhs >= v $dimension /\
    (forall (k:nat). k < v $dimension ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index $lhs k).f_simd_units j)) /\
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index $rhs k).f_simd_units j))))
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${lhs}_future == Seq.length $lhs /\
    (forall (k:nat{k < Seq.length ${lhs}_future}). k < v $dimension ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque 16760832
                (i0._super_i2.f_repr (Seq.index (Seq.index ${lhs}_future k).f_simd_units j))))
"#))]
pub(crate) fn add_vectors<SIMDUnit: Operations>(
    dimension: usize,
    lhs: &mut [PolynomialRingElement<SIMDUnit>],
    rhs: &[PolynomialRingElement<SIMDUnit>],
) {
    #[cfg(hax)]
    let e_lhs_orig: &[PolynomialRingElement<SIMDUnit>] = lhs.to_vec().as_slice();
    for i in 0..dimension {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v i <= v dimension /\
              Seq.length lhs == Seq.length e_lhs_orig /\
              Seq.length lhs >= v dimension /\
              (forall (k:nat). k < v i ==>
                  (forall (j:nat). j < 32 ==>
                      Spec.Utils.is_i32b_array_opaque 16760832
                          (i0._super_i2.f_repr (Seq.index (Seq.index lhs k).f_simd_units j)))) /\
              (forall (k:nat). v i <= k /\ k < Seq.length lhs ==>
                  Seq.index lhs k == Seq.index e_lhs_orig k)"#
        ));
        PolynomialRingElement::<SIMDUnit>::add_bounded(&mut lhs[i], 8380416, &rhs[i], 8380416);
    }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning")]
#[hax_lib::requires(fstar!(r#"
    Seq.length $lhs >= v $dimension /\
    Seq.length $rhs >= v $dimension /\
    (forall (k:nat). k < v $dimension ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index $lhs k).f_simd_units j)) /\
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index $rhs k).f_simd_units j))))
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${lhs}_future == Seq.length $lhs /\
    (forall (k:nat{k < Seq.length ${lhs}_future}). k < v $dimension ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque 16760832
                (i0._super_i2.f_repr (Seq.index (Seq.index ${lhs}_future k).f_simd_units j))))
"#))]
pub(crate) fn subtract_vectors<SIMDUnit: Operations>(
    dimension: usize,
    lhs: &mut [PolynomialRingElement<SIMDUnit>],
    rhs: &[PolynomialRingElement<SIMDUnit>],
) {
    #[cfg(hax)]
    let e_lhs_orig: &[PolynomialRingElement<SIMDUnit>] = lhs.to_vec().as_slice();
    for i in 0..dimension {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v i <= v dimension /\
              Seq.length lhs == Seq.length e_lhs_orig /\
              Seq.length lhs >= v dimension /\
              (forall (k:nat). k < v i ==>
                  (forall (j:nat). j < 32 ==>
                      Spec.Utils.is_i32b_array_opaque 16760832
                          (i0._super_i2.f_repr (Seq.index (Seq.index lhs k).f_simd_units j)))) /\
              (forall (k:nat). v i <= k /\ k < Seq.length lhs ==>
                  Seq.index lhs k == Seq.index e_lhs_orig k)"#
        ));
        PolynomialRingElement::<SIMDUnit>::subtract_bounded(&mut lhs[i], 8380416, &rhs[i], 8380416);
    }
}

/// Compute InvertNTT(Â ◦ ẑ - ĉ ◦ NTT(t₁2ᵈ))
#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    Seq.length $matrix >= v $rows_in_a * v $columns_in_a /\
    Seq.length $signer_response >= v $columns_in_a /\
    Seq.length $t1 >= v $rows_in_a /\
    v $columns_in_a <= 7 /\
    (forall (j:nat). j < 32 ==>
        Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
            (i0._super_i2.f_repr (Seq.index verifier_challenge_as_ntt.f_simd_units j))) /\
    (forall (k:nat). k < Seq.length $matrix ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index $matrix k).f_simd_units j)))) /\
    (forall (k:nat). k < Seq.length $signer_response ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index $signer_response k).f_simd_units j)))) /\
    (forall (k:nat). k < Seq.length $t1 ==>
        (forall (j:nat). j < 32 ==>
            (forall (i: nat). i < 8 ==>
                v (Seq.index (i0._super_i2.f_repr (Seq.index (Seq.index $t1 k).f_simd_units j)) i) >= 0 /\
                v (Seq.index (i0._super_i2.f_repr (Seq.index (Seq.index $t1 k).f_simd_units j)) i) <= 261631)))
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${t1}_future == Seq.length $t1
"#))]
pub(crate) fn compute_w_approx<SIMDUnit: Operations>(
    rows_in_a: usize,
    columns_in_a: usize,
    matrix: &[PolynomialRingElement<SIMDUnit>],
    signer_response: &[PolynomialRingElement<SIMDUnit>],
    verifier_challenge_as_ntt: &PolynomialRingElement<SIMDUnit>,
    t1: &mut [PolynomialRingElement<SIMDUnit>],
) {
    hax_lib::fstar!("admit ()");
    for i in 0..rows_in_a {
        let mut inner_result = PolynomialRingElement::<SIMDUnit>::zero();
        for j in 0..columns_in_a {
            let mut product = matrix[i * columns_in_a + j];
            ntt_multiply_montgomery(&mut product, &signer_response[j]);
            PolynomialRingElement::<SIMDUnit>::add(&mut inner_result, &product);
        }

        shift_left_then_reduce::<SIMDUnit, { BITS_IN_LOWER_PART_OF_T as i32 }>(&mut t1[i]);
        // shift_left_then_reduce post: is_i32b FIELD_MAX.  ntt pre is now
        // NTT_BASE_BOUND = FIELD_MAX (widened from FIELD_MID by below-trait
        // Option C, ml-dsa-proofs commit 686543e33), so the chain composes
        // directly — no intermediate reduce needed.
        ntt(&mut t1[i]);
        ntt_multiply_montgomery(&mut t1[i], verifier_challenge_as_ntt);
        PolynomialRingElement::<SIMDUnit>::subtract(&mut inner_result, &t1[i]);
        t1[i] = inner_result;
        // We do a Barrett reduction here, since the absolute value of
        // `columns_in_a` additions might be as large as `columns_in_a
        // * FIELD_MODULUS`, and `invert_ntt_montgomery` expects
        // coefficients of size at most `FIELD_MODULUS`.
        reduce(&mut t1[i]);
        invert_ntt_montgomery(&mut t1[i]);
    }
}

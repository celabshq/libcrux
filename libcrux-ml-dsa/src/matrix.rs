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
use crate::simd::traits::specs::*;

// Not inlining this makes key generation 3x slower for avx2. Only `inline` this
// function costs 30% performance too.
/// Compute InvertNTT(Â ◦ ŝ₁) + s₂
#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    Seq.length $a_as_ntt >= v $rows_in_a * v $columns_in_a /\
    Seq.length $s1_ntt >= v $columns_in_a /\
    Seq.length $s1_s2 >= v $columns_in_a + v $rows_in_a /\
    Seq.length $result >= v $rows_in_a /\
    v $columns_in_a <= 7 /\
    (forall (k:nat). k < Seq.length $s1_ntt ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index $s1_ntt k).f_simd_units j)))) /\
    (forall (k:nat). k < Seq.length $s1_s2 ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index $s1_s2 k).f_simd_units j)))) /\
    (forall (k:nat). k < Seq.length $result ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index $result k).f_simd_units j))))
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${result}_future == Seq.length $result /\
    (forall (k:nat{k < Seq.length ${result}_future}). k < v $rows_in_a ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque 16760832
                (i0._super_i2.f_repr (Seq.index (Seq.index ${result}_future k).f_simd_units j))))
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
#[hax_lib::requires(fstar!(r#"
    Seq.length $matrix >= v $rows_in_a * v $columns_in_a /\
    Seq.length $mask >= v $columns_in_a /\
    Seq.length $result >= v $rows_in_a /\
    v $columns_in_a <= 7 /\
    (forall (k:nat). k < Seq.length $matrix ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index $matrix k).f_simd_units j)))) /\
    (forall (k:nat). k < Seq.length $result ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index $result k).f_simd_units j))))
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${result}_future == Seq.length $result /\
    (forall (k:nat{k < Seq.length ${result}_future}). k < v $rows_in_a ==>
        (forall (j:nat). j < 32 ==>
            Spec.Utils.is_i32b_array_opaque (v ${FIELD_MAX})
                (i0._super_i2.f_repr (Seq.index (Seq.index ${result}_future k).f_simd_units j))))
"#))]
pub(crate) fn compute_matrix_x_mask<SIMDUnit: Operations>(
    rows_in_a: usize,
    columns_in_a: usize,
    matrix: &[PolynomialRingElement<SIMDUnit>],
    mask: &[PolynomialRingElement<SIMDUnit>],
    result: &mut [PolynomialRingElement<SIMDUnit>],
) {
    // Body admit — the recipe (Polynomial::add_bounded with j*FIELD_MAX
    // ghost bound, snapshot-based frame, nested loop_invariants) was tried
    // but query 106-121 time out at rlimit 800 due to quantifier explosion
    // in the nested-loop SMT search.  Same shape as compute_as1_plus_s2.
    hax_lib::fstar!("admit ()");
    for i in 0..rows_in_a {
        for j in 0..columns_in_a {
            // We could make `matrix` mutable here and avoid copying.
            // But that would require sampling the matrix multiple times.
            // That's not worth it.
            let mut product = mask[j];
            ntt_multiply_montgomery(&mut product, &matrix[i * columns_in_a + j]);
            PolynomialRingElement::<SIMDUnit>::add(&mut result[i], &product);
        }
        // We do a Barrett reduction here, since the absolute value of
        // `columns_in_a` additions might be as large as `columns_in_a
        // * FIELD_MODULUS`, and `invert_ntt_montgomery` expects
        // coefficients of size at most `FIELD_MODULUS`.
        reduce(&mut result[i]);
        invert_ntt_montgomery(&mut result[i]);
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
    hax_lib::fstar!("admit ()");
    #[cfg(hax)]
    let e_vector_orig: &[PolynomialRingElement<SIMDUnit>] = vector.to_vec().as_slice();
    for i in 0..vector.len() {
        ntt_multiply_montgomery(&mut vector[i], ring_element);
        invert_ntt_montgomery(&mut vector[i]);
    }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --split_queries always")]
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
#[hax_lib::fstar::options("--z3rlimit 800 --split_queries always")]
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

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
/// every iteration. The `_rhs_bound` ghost parameter (runtime-erased)
/// lets callers thread the actual `rhs` bound through the chain — typical
/// values: `8380416` (FIELD_MAX) for `ntt_multiply_montgomery` outputs,
/// `4` for ETA-bounded `s2` entries.
/// Lives in `matrix.rs` rather than `impl PolynomialRingElement`
/// to avoid a `Polynomial → Polynomial.Spec → Polynomial` extraction cycle.
#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    v $_bound + v $_rhs_bound <= 2147483647 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly $_bound $myself /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly $_rhs_bound $rhs"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
        ($_bound +! $_rhs_bound) ${myself}_future"#))]
fn add_to_ring_element<SIMDUnit: Operations>(
    myself: &mut PolynomialRingElement<SIMDUnit>,
    rhs: &PolynomialRingElement<SIMDUnit>,
    _bound: usize,
    _rhs_bound: usize,
) {
    PolynomialRingElement::<SIMDUnit>::add_bounded(myself, _bound, rhs, _rhs_bound);
    hax_lib::fstar!(r#"
      Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_intro
        ($_bound +! $_rhs_bound) $myself
    "#);
}

/// `subtract` analog of `add_to_ring_element`. Same recipe: lifts the
/// per-simd-unit `subtract_bounded` post to the polynomial level.
/// `rhs` is fixed at `FIELD_MAX = 8380416` to match the post of
/// `ntt_multiply_montgomery` (the only summand in `compute_w_approx`).
#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    v $_bound + 8380416 <= 2147483647 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly $_bound $myself /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416) $rhs"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
        ($_bound +! mk_usize 8380416) ${myself}_future"#))]
fn subtract_to_ring_element<SIMDUnit: Operations>(
    myself: &mut PolynomialRingElement<SIMDUnit>,
    rhs: &PolynomialRingElement<SIMDUnit>,
    _bound: usize,
) {
    PolynomialRingElement::<SIMDUnit>::subtract_bounded(myself, _bound, rhs, 8380416);
    hax_lib::fstar!(r#"
      Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_intro
        ($_bound +! mk_usize 8380416) $myself
    "#);
}

// Not inlining this makes key generation 3x slower for avx2. Only `inline` this
// function costs 30% performance too.
//
// Split into two helpers to keep each F* function-level WP tractable:
// `compute_as1_plus_s2` does the NTT-domain accumulation (mutates both
// `a_as_ntt` and `result`, returns tuple at the F* level), and
// `inv_ntt_and_add_s2` does the second pass (Barrett reduce → InvNTT →
// add s2-half), single-mutable like `compute_matrix_x_mask`.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(fstar!(r#"
    Seq.length $a_as_ntt == v $rows_in_a * v $columns_in_a /\
    Seq.length $s1_ntt == v $columns_in_a /\
    Seq.length $result == v $rows_in_a /\
    v $rows_in_a <= 8 /\
    v $columns_in_a <= 7 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $a_as_ntt /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) $s1_ntt /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 0) $result
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${a_as_ntt}_future == Seq.length $a_as_ntt /\
    Seq.length ${result}_future == Seq.length $result /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice
        ($columns_in_a *! mk_usize 8380416) ${result}_future
"#))]
fn ntt_dot_accumulate<SIMDUnit: Operations>(
    rows_in_a: usize,
    columns_in_a: usize,
    a_as_ntt: &mut [PolynomialRingElement<SIMDUnit>],
    s1_ntt: &[PolynomialRingElement<SIMDUnit>],
    result: &mut [PolynomialRingElement<SIMDUnit>],
) {
    // Bootstrap outer-fold initial invariant from is_bounded_poly_slice 0
    // precondition: lift to per-element via Classical.forall_intro of the
    // slice lookup_lemma, then build the two range predicates.
    hax_lib::fstar!(
        r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length ${result}}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 0)
                     (Seq.index $result k)) =
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              (mk_usize 0) $result k
          in
          Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
          (mk_usize 0) (mk_usize 0) $rows_in_a $result;
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
          ($columns_in_a *! mk_usize 8380416)
          (mk_usize 0) (mk_usize 0) $result"#
    );
    for i in 0..rows_in_a {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v $i <= v $rows_in_a /\
              Seq.length $a_as_ntt == v $rows_in_a * v $columns_in_a /\
              Seq.length $s1_ntt == v $columns_in_a /\
              Seq.length $result == v $rows_in_a /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $a_as_ntt /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) $s1_ntt /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                  ($columns_in_a *! mk_usize 8380416)
                  (mk_usize 0) $i $result /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                  (mk_usize 0) $i $rows_in_a $result"#
        ));
        // Bridge outer-inv frame [i, rows) at bound 0 into inner-fold initial
        // state's needed [i+1, rows) frame.
        hax_lib::fstar!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
                 (mk_usize 0) ($i +! mk_usize 1) $rows_in_a $result"#
        );
        for j in 0..columns_in_a {
            hax_lib::loop_invariant!(|j: usize| fstar!(
                r#"v $j <= v $columns_in_a /\
                  v $i < v $rows_in_a /\
                  Seq.length $a_as_ntt == v $rows_in_a * v $columns_in_a /\
                  Seq.length $result == v $rows_in_a /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $a_as_ntt /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) $s1_ntt /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                      ($columns_in_a *! mk_usize 8380416)
                      (mk_usize 0) $i $result /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                      ($j *! mk_usize 8380416)
                      (Seq.index $result (v $i)) /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                      (mk_usize 0) ($i +! mk_usize 1) $rows_in_a $result"#
            ));
            // a_as_ntt[i*cols+j] is FIELD_MAX (matrix slice); weaken to
            // NTT_OUTPUT_BOUND for ntt_multiply's widened (both-operands) pre.
            // s1_ntt[j] is the NTT_OUTPUT_BOUND rhs (the ntt'd s1).
            hax_lib::fstar!(
                r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
                     (mk_usize 8380416) $a_as_ntt (v $i * v $columns_in_a + v $j);
                   Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher
                     (mk_usize 8380416) (mk_usize 75423744)
                     (Seq.index $a_as_ntt (v $i * v $columns_in_a + v $j))"#
            );
            ntt_multiply_montgomery::<SIMDUnit>(&mut a_as_ntt[i * columns_in_a + j], &s1_ntt[j]);
            // Use the bound-aware wrapper so the per-lane add precondition is
            // discharged inside add_to_ring_element rather than at the call site.
            // Bound (j*FM) reflects the inner inv at iter j; post is ((j+1)*FM).
            add_to_ring_element::<SIMDUnit>(
                &mut result[i],
                &a_as_ntt[i * columns_in_a + j],
                j * 8380416,
                8380416,
            );
            // Re-establish is_bounded_poly_slice for a_as_ntt (one entry was
            // updated by ntt_multiply, others unchanged) and frame the [0,i)
            // and [i+1, rows) ranges of result that the inner body didn't touch.
            hax_lib::fstar!(
                r#"
                let _:Prims.unit =
                  let aux (k: nat{k < Seq.length ${a_as_ntt}}) :
                    Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                             (mk_usize 8380416) (Seq.index $a_as_ntt k)) = () in
                  Classical.forall_intro aux;
                  Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
                    (mk_usize 8380416) $a_as_ntt
                in
                Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
                  ($columns_in_a *! mk_usize 8380416)
                  (mk_usize 0) $i $result;
                Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
                  (mk_usize 0) ($i +! mk_usize 1) $rows_in_a $result"#
            );
        }
        hax_lib::fstar!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
                 ($columns_in_a *! mk_usize 8380416)
                 (mk_usize 0) ($i +! mk_usize 1) $result"#
        );
    }
    // Bridge outer range to slice form for the post.
    hax_lib::fstar!(
        r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length ${result}}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                     ($columns_in_a *! mk_usize 8380416) (Seq.index $result k)) =
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
              ($columns_in_a *! mk_usize 8380416) (mk_usize 0) $rows_in_a $result k
          in
          Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
          ($columns_in_a *! mk_usize 8380416) $result"#
    );
}

/// Per-row Barrett reduce → InvNTT → add s2[i]. Mirrors
/// `compute_matrix_x_mask`'s single-mutable shape.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(fstar!(r#"
    Seq.length $result == v $rows_in_a /\
    Seq.length $s2 == v $rows_in_a /\
    v $rows_in_a <= 8 /\
    v $columns_in_a <= 7 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 4) $s2 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice
        ($columns_in_a *! mk_usize 8380416) $result
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${result}_future == Seq.length $result /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) ${result}_future
"#))]
fn inv_ntt_and_add_s2<SIMDUnit: Operations>(
    rows_in_a: usize,
    columns_in_a: usize,
    s2: &[PolynomialRingElement<SIMDUnit>],
    result: &mut [PolynomialRingElement<SIMDUnit>],
) {
    // Bootstrap initial invariant from is_bounded_poly_slice (cols*FM) result
    // precondition: lift to per-element + build the two range predicates.
    hax_lib::fstar!(
        r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length ${result}}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                     ($columns_in_a *! mk_usize 8380416) (Seq.index $result k)) =
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              ($columns_in_a *! mk_usize 8380416) $result k
          in
          Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
          ($columns_in_a *! mk_usize 8380416) (mk_usize 0) $rows_in_a $result;
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
          (mk_usize 8380416) (mk_usize 0) (mk_usize 0) $result"#
    );
    for i in 0..result.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v $i <= Seq.length $result /\
              Seq.length $result == v $rows_in_a /\
              Seq.length $s2 == v $rows_in_a /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 4) $s2 /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                  (mk_usize 8380416) (mk_usize 0) $i $result /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                  ($columns_in_a *! mk_usize 8380416)
                  $i $rows_in_a $result"#
        ));
        // Lookup result[i] from inv's range, then weaken (cols*FM) → 2_143_289_343
        // for reduce's pre.
        hax_lib::fstar!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
                 ($columns_in_a *! mk_usize 8380416)
                 $i $rows_in_a $result (v $i);
               Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher
                 ($columns_in_a *! mk_usize 8380416) (mk_usize 2143289343)
                 (Seq.index $result (v $i))"#
        );
        // We do a Barrett reduction here, since the absolute value of
        // `columns_in_a` additions might be as large as `columns_in_a
        // * FIELD_MODULUS`, and `invert_ntt_montgomery` expects
        // coefficients of size at most `FIELD_MODULUS`.
        reduce(&mut result[i]);
        invert_ntt_montgomery::<SIMDUnit>(&mut result[i]);
        // Bound-aware add: post is is_bounded_poly (4_211_177 + 4 = 4_211_181).
        // s2 is sampled in [0, ETA] with ETA ≤ 4, so s2[i] fits is_bounded_poly 4.
        add_to_ring_element::<SIMDUnit>(&mut result[i], &s2[i], 4_211_177, 4);
        // Lift 4_211_181 → 8_380_416 (FIELD_MAX) for the inv carryover; frame
        // [i+1, rows) at bound (cols*FM) (untouched entries).
        hax_lib::fstar!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher
                 (mk_usize 4211177 +! mk_usize 4) (mk_usize 8380416)
                 (Seq.index $result (v $i));
               Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
                 (mk_usize 8380416) (mk_usize 0) ($i +! mk_usize 1) $result;
               Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
                 ($columns_in_a *! mk_usize 8380416)
                 ($i +! mk_usize 1) $rows_in_a $result"#
        );
    }
    // Bridge outer range (FIELD_MAX over [0, rows)) to slice form for the post.
    hax_lib::fstar!(
        r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length ${result}}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416)
                     (Seq.index $result k)) =
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
              (mk_usize 8380416) (mk_usize 0) $rows_in_a $result k
          in
          Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
          (mk_usize 8380416) $result"#
    );
}

/// Compute InvertNTT(Â ◦ ŝ₁) + s₂
#[inline(always)]
#[hax_lib::requires(fstar!(r#"
    Seq.length $a_as_ntt == v $rows_in_a * v $columns_in_a /\
    Seq.length $s1_ntt == v $columns_in_a /\
    Seq.length $s1_s2 == v $columns_in_a + v $rows_in_a /\
    Seq.length $result == v $rows_in_a /\
    v $rows_in_a <= 8 /\
    v $columns_in_a <= 7 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $a_as_ntt /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) $s1_ntt /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 4) $s1_s2 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 0) $result
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${a_as_ntt}_future == Seq.length $a_as_ntt /\
    Seq.length ${result}_future == Seq.length $result /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) ${result}_future
"#))]
pub(crate) fn compute_as1_plus_s2<SIMDUnit: Operations>(
    rows_in_a: usize,
    columns_in_a: usize,
    a_as_ntt: &mut [PolynomialRingElement<SIMDUnit>],
    s1_ntt: &[PolynomialRingElement<SIMDUnit>],
    s1_s2: &[PolynomialRingElement<SIMDUnit>],
    result: &mut [PolynomialRingElement<SIMDUnit>],
) {
    ntt_dot_accumulate::<SIMDUnit>(rows_in_a, columns_in_a, a_as_ntt, s1_ntt, result);
    let s2 = &s1_s2[columns_in_a..];
    // Slice-projection bridge: every entry of s2 is also an entry of s1_s2
    // (at offset `columns_in_a + k`), so s2 inherits the ETA bound (≤ 4).
    hax_lib::fstar!(
        r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length ${s2}}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 4)
                     (Seq.index $s2 k)) =
            assert (Seq.index $s2 k == Seq.index $s1_s2 (v $columns_in_a + k));
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              (mk_usize 4) $s1_s2 (v $columns_in_a + k)
          in
          Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
          (mk_usize 4) $s2"#
    );
    inv_ntt_and_add_s2::<SIMDUnit>(rows_in_a, columns_in_a, s2, result);
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
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 75423744)
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
    // Establish the widened matrix forall ONCE in clean context (function
    // entry), via the per-entry monotonicity lemma.  Carrying it as a single
    // invariant fact (instead of re-deriving per inner-loop iteration) keeps
    // the outer-invariant re-establishment query from saturating: each
    // per-iteration `lemma_is_bounded_poly_higher` call would otherwise leave a
    // lingering `is_bounded_poly 75423744` opaque atom that pollutes sub-query.
    hax_lib::fstar!(
        r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length ${matrix}}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 75423744)
                     (Seq.index $matrix k)) =
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher
              (mk_usize 8380416) (mk_usize 75423744) (Seq.index $matrix k)
          in
          Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
          (mk_usize 75423744) $matrix;
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
          (mk_usize 8380416) (mk_usize 0) (mk_usize 0) $result"#
    );
    for i in 0..rows_in_a {
        // Outer carryover carried as the opaque `is_bounded_poly_range` atom
        // over [0, i) rather than a bare `forall k < i`.  The bare forall's
        // re-establishment query (after the index-`i` update) bails with
        // "incomplete quantifiers" under `--split_queries always`; the opaque
        // range + standalone `lemma_is_bounded_poly_range_extend_after_update`
        // (verified in clean context) closes it.  Mirrors `compute_w_approx`.
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v $i <= v $rows_in_a /\
              Seq.length $result == v $rows_in_a /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) $matrix /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                  (mk_usize 8380416) (mk_usize 0) $i $result"#
        ));

        // Snapshot before zeroing — used as the frame anchor inside the
        // inner loop and by reduce/invert_ntt_montgomery (which only mutate
        // result[i]).  It is also the iter-start anchor for the outer-inv
        // carryover-extension lemma at the end of the body.
        #[cfg(hax)]
        let old_result: &[PolynomialRingElement<SIMDUnit>] = result.to_vec().as_slice();
        // Carry the outer-inv opaque range over [0, i) from `result` to the
        // `old_result` snapshot (they are element-wise equal here), so the
        // inner-loop invariant and the end-of-body extension lemma can name it.
        hax_lib::fstar!(
            r#"
            let _:Prims.unit =
              let aux (k: nat{k < v $i /\ k < Seq.length old_result}) :
                Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                         (mk_usize 8380416) (Seq.index old_result k)) =
                assert (Seq.index old_result k == Seq.index $result k);
                Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
                  (mk_usize 8380416) (mk_usize 0) $i $result k
              in
              Classical.forall_intro aux
            in
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
              (mk_usize 8380416) (mk_usize 0) $i old_result"#
        );

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
                  Seq.length old_result == v $rows_in_a /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) $matrix /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                      (mk_usize 8380416) (mk_usize 0) $i old_result /\
                  (forall (k:nat). k < v $rows_in_a /\ k <> v $i ==>
                      Seq.index $result k == Seq.index old_result k) /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                      ($j *! mk_usize 8380416) (Seq.index $result (v $i))"#
            ));

            // We could make `matrix` mutable here and avoid copying.
            // But that would require sampling the matrix multiple times.
            // That's not worth it.
            let mut product = mask[j];
            // product (= mask[j]) is the NTT_OUTPUT_BOUND lhs; matrix[i*cols+j] (rhs)
            // is NTT_OUTPUT_BOUND via the opaque is_bounded_poly_slice atom carried in
            // the invariant — slice lookup (no bare-forall, so no context pollution of
            // the outer re-establishment query).
            hax_lib::fstar!(
                r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
                     (mk_usize 75423744) $matrix (v $i * v $columns_in_a + v $j)"#
            );
            ntt_multiply_montgomery::<SIMDUnit>(&mut product, &matrix[i * columns_in_a + j]);
            add_to_ring_element::<SIMDUnit>(&mut result[i], &product, j * 8380416, 8380416);
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
        // Re-establish the outer carryover at i+1 via the standalone lemma,
        // verified in clean context (polynomial.rs::spec), so the trivial
        // `k = v $i` and Seq-frame reasoning is not polluted by this
        // function's heavy ambient context (the old bare-forall
        // re-establishment bailed with "incomplete quantifiers" under
        // `--split_queries always`).  The body only mutated `result[v i]`;
        // the frame + range over `old_result` come from the inner-loop inv.
        hax_lib::fstar!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_extend_after_update
                 (mk_usize 8380416) $i old_result $result"#
        );
    }
    // After the outer loop: is_bounded_poly_range 8380416 0 rows_in_a result.
    // Bridge to the bare-forall ensures.
    hax_lib::fstar!(
        r#"
        let _:Prims.unit =
          let aux (k: nat{k < v $rows_in_a /\ k < Seq.length ${result}}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416)
                     (Seq.index $result k)) =
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
              (mk_usize 8380416) (mk_usize 0) $rows_in_a $result k
          in
          Classical.forall_intro aux
        in
        ()"#
    );
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning --split_queries always")]
#[hax_lib::requires(fstar!(r#"
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $vector /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 75423744) $ring_element
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${vector}_future == Seq.length $vector /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) ${vector}_future
"#))]
pub(crate) fn vector_times_ring_element<SIMDUnit: Operations>(
    vector: &mut [PolynomialRingElement<SIMDUnit>],
    ring_element: &PolynomialRingElement<SIMDUnit>,
) {
    #[cfg(hax)]
    let e_vector_orig: &[PolynomialRingElement<SIMDUnit>] = vector.to_vec().as_slice();
    // Carry the function-pre slice pred to the entry snapshot + seed empty range.
    hax_lib::fstar!(r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length e_vector_orig}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                     (mk_usize 8380416) (Seq.index e_vector_orig k)) =
            assert (Seq.index e_vector_orig k == Seq.index $vector k);
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              (mk_usize 8380416) $vector k
          in
          Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
          (mk_usize 8380416) e_vector_orig;
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
          (mk_usize 8380416) (mk_usize 0) (mk_usize 0) $vector"#);
    for i in 0..vector.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v $i <= Seq.length $vector /\
              Seq.length $vector == Seq.length e_vector_orig /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 75423744) $ring_element /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) e_vector_orig /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                  (mk_usize 8380416) (mk_usize 0) $i $vector /\
              (forall (k:nat). v $i <= k /\ k < Seq.length $vector ==>
                  Seq.index $vector k == Seq.index e_vector_orig k)"#
        ));
        #[cfg(hax)]
        let iter_start: &[PolynomialRingElement<SIMDUnit>] = vector.to_vec().as_slice();
        // vector[i] == e_vector_orig[i] (tail frame) and e_vector_orig is FIELD_MAX-bounded
        // (slice lookup), giving the FIELD_MAX bound ntt_multiply_montgomery needs on vector[i].
        hax_lib::fstar!(r#"
            assert (Seq.index $vector (v $i) == Seq.index e_vector_orig (v $i));
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              (mk_usize 8380416) e_vector_orig (v $i);
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher
              (mk_usize 8380416) (mk_usize 75423744) (Seq.index $vector (v $i))"#);
        ntt_multiply_montgomery(&mut vector[i], ring_element);
        invert_ntt_montgomery(&mut vector[i]);
        // invert_ntt_montgomery post is is_bounded_poly 4_211_177; weaken to FIELD_MAX,
        // then extend the processed range to i+1 via the standalone lemma.
        hax_lib::fstar!(r#"
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher
              (mk_usize 4211177) (mk_usize 8380416) (Seq.index $vector (v $i));
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_extend_after_update
              (mk_usize 8380416) $i iter_start $vector"#);
    }
    // Bridge the final processed range to slice form for the post.
    hax_lib::fstar!(r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length ${vector}}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 8380416)
                     (Seq.index $vector k)) =
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
              (mk_usize 8380416) (mk_usize 0) (Core_models.Slice.impl__len $vector) $vector k
          in
          Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
          (mk_usize 8380416) $vector"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning --split_queries always")]
#[hax_lib::requires(fstar!(r#"
    Seq.length $lhs >= v $dimension /\
    Seq.length $rhs >= v $dimension /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $lhs /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $rhs
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${lhs}_future == Seq.length $lhs /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 16760832) ${lhs}_future
"#))]
pub(crate) fn add_vectors<SIMDUnit: Operations>(
    dimension: usize,
    lhs: &mut [PolynomialRingElement<SIMDUnit>],
    rhs: &[PolynomialRingElement<SIMDUnit>],
) {
    #[cfg(hax)]
    let e_lhs_orig: &[PolynomialRingElement<SIMDUnit>] = lhs.to_vec().as_slice();
    hax_lib::fstar!(r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length e_lhs_orig}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                     (mk_usize 8380416) (Seq.index e_lhs_orig k)) =
            assert (Seq.index e_lhs_orig k == Seq.index $lhs k);
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              (mk_usize 8380416) $lhs k
          in Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
          (mk_usize 8380416) e_lhs_orig;
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
          (mk_usize 16760832) (mk_usize 0) (mk_usize 0) $lhs"#);
    for i in 0..dimension {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v $i <= v $dimension /\
              Seq.length $lhs == Seq.length e_lhs_orig /\
              Seq.length $lhs >= v $dimension /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) e_lhs_orig /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $rhs /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                  (mk_usize 16760832) (mk_usize 0) $i $lhs /\
              (forall (k:nat). v $i <= k /\ k < Seq.length $lhs ==>
                  Seq.index $lhs k == Seq.index e_lhs_orig k)"#
        ));
        #[cfg(hax)]
        let iter_start: &[PolynomialRingElement<SIMDUnit>] = lhs.to_vec().as_slice();
        hax_lib::fstar!(r#"
            assert (Seq.index $lhs (v $i) == Seq.index e_lhs_orig (v $i));
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              (mk_usize 8380416) e_lhs_orig (v $i);
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              (mk_usize 8380416) $rhs (v $i)"#);
        add_to_ring_element::<SIMDUnit>(&mut lhs[i], &rhs[i], 8380416, 8380416);
        hax_lib::fstar!(r#"
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_extend_after_update
              (mk_usize 16760832) $i iter_start $lhs"#);
    }
    hax_lib::fstar!(r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length ${lhs}}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 16760832)
                     (Seq.index $lhs k)) =
            if k < v $dimension then
              Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
                (mk_usize 16760832) (mk_usize 0) $dimension $lhs k
            else begin
              assert (Seq.index $lhs k == Seq.index e_lhs_orig k);
              Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
                (mk_usize 8380416) e_lhs_orig k;
              Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher
                (mk_usize 8380416) (mk_usize 16760832) (Seq.index $lhs k)
            end
          in Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
          (mk_usize 16760832) $lhs"#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning --split_queries always")]
#[hax_lib::requires(fstar!(r#"
    Seq.length $lhs >= v $dimension /\
    Seq.length $rhs >= v $dimension /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $lhs /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $rhs
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${lhs}_future == Seq.length $lhs /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 16760832) ${lhs}_future
"#))]
pub(crate) fn subtract_vectors<SIMDUnit: Operations>(
    dimension: usize,
    lhs: &mut [PolynomialRingElement<SIMDUnit>],
    rhs: &[PolynomialRingElement<SIMDUnit>],
) {
    #[cfg(hax)]
    let e_lhs_orig: &[PolynomialRingElement<SIMDUnit>] = lhs.to_vec().as_slice();
    hax_lib::fstar!(r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length e_lhs_orig}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                     (mk_usize 8380416) (Seq.index e_lhs_orig k)) =
            assert (Seq.index e_lhs_orig k == Seq.index $lhs k);
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              (mk_usize 8380416) $lhs k
          in Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
          (mk_usize 8380416) e_lhs_orig;
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
          (mk_usize 16760832) (mk_usize 0) (mk_usize 0) $lhs"#);
    for i in 0..dimension {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v $i <= v $dimension /\
              Seq.length $lhs == Seq.length e_lhs_orig /\
              Seq.length $lhs >= v $dimension /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) e_lhs_orig /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $rhs /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                  (mk_usize 16760832) (mk_usize 0) $i $lhs /\
              (forall (k:nat). v $i <= k /\ k < Seq.length $lhs ==>
                  Seq.index $lhs k == Seq.index e_lhs_orig k)"#
        ));
        #[cfg(hax)]
        let iter_start: &[PolynomialRingElement<SIMDUnit>] = lhs.to_vec().as_slice();
        hax_lib::fstar!(r#"
            assert (Seq.index $lhs (v $i) == Seq.index e_lhs_orig (v $i));
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              (mk_usize 8380416) e_lhs_orig (v $i);
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              (mk_usize 8380416) $rhs (v $i)"#);
        subtract_to_ring_element::<SIMDUnit>(&mut lhs[i], &rhs[i], 8380416);
        hax_lib::fstar!(r#"
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_extend_after_update
              (mk_usize 16760832) $i iter_start $lhs"#);
    }
    hax_lib::fstar!(r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length ${lhs}}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 16760832)
                     (Seq.index $lhs k)) =
            if k < v $dimension then
              Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
                (mk_usize 16760832) (mk_usize 0) $dimension $lhs k
            else begin
              assert (Seq.index $lhs k == Seq.index e_lhs_orig k);
              Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
                (mk_usize 8380416) e_lhs_orig k;
              Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher
                (mk_usize 8380416) (mk_usize 16760832) (Seq.index $lhs k)
            end
          in Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
          (mk_usize 16760832) $lhs"#);
}

/// Per-row composition step for `compute_w_approx`: take a single t1 entry
/// (T1-decoded, per-lane in `[0, 261631]`) and an `inner_result` polynomial
/// (the matrix-row · signer-response dot product, bounded by `cols*FIELD_MAX`),
/// and produce the row's contribution to w_approx in `t1_entry`.
///
/// Factored out so the chain of mutations on `t1_entry` happens at the
/// single-polynomial level (not slice level).  This avoids 6+ sequential
/// `Seq.upd t1 i ...` operations re-typing through the outer fold's body
/// refinement, which is the cascade source we identified via SMT profiling.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning")]
#[hax_lib::requires(fstar!(r#"
    v $columns_in_a <= 7 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly (mk_usize 0) (mk_usize 261631) $t1_entry /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
        ($columns_in_a *! mk_usize 8380416) $inner_result /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 75423744) $verifier_challenge_as_ntt
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 4211177) ${t1_entry}_future
"#))]
fn compose_w_approx_per_row<SIMDUnit: Operations>(
    t1_entry: &mut PolynomialRingElement<SIMDUnit>,
    mut inner_result: PolynomialRingElement<SIMDUnit>,
    verifier_challenge_as_ntt: &PolynomialRingElement<SIMDUnit>,
    columns_in_a: usize,
) {
    shift_left_then_reduce::<SIMDUnit, { BITS_IN_LOWER_PART_OF_T as i32 }>(t1_entry);
    // shift_left_then_reduce post: per-lane is_i32b_array_opaque FIELD_MAX.
    // Lift to is_bounded_poly form.
    hax_lib::fstar!(
        r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_intro
             (mk_usize 8380416) $t1_entry"#
    );

    ntt(t1_entry);
    ntt_multiply_montgomery(t1_entry, verifier_challenge_as_ntt);
    // t1_entry is_bounded_poly FIELD_MAX.

    subtract_to_ring_element::<SIMDUnit>(&mut inner_result, t1_entry, columns_in_a * 8380416);
    // inner_result is_bounded_poly ((cols+1)*FIELD_MAX).

    *t1_entry = inner_result;
    // Weaken (cols+1)*FIELD_MAX → 2_143_289_343 for reduce's pre.
    hax_lib::fstar!(
        r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher
             ($columns_in_a *! mk_usize 8380416 +! mk_usize 8380416)
             (mk_usize 2143289343)
             $t1_entry"#
    );

    reduce(t1_entry);
    invert_ntt_montgomery(t1_entry);
    // t1_entry is_bounded_poly 4_211_177 (post of invert_ntt_montgomery).
}

/// Compute InvertNTT(Â ◦ ẑ - ĉ ◦ NTT(t₁2ᵈ))
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning --split_queries always")]
#[hax_lib::requires(fstar!(r#"
    Seq.length $matrix == v $rows_in_a * v $columns_in_a /\
    Seq.length $signer_response == v $columns_in_a /\
    Seq.length $t1 == v $rows_in_a /\
    v $rows_in_a <= 8 /\
    v $columns_in_a <= 7 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $matrix /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) $signer_response /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 75423744) $verifier_challenge_as_ntt /\
    Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice (mk_usize 0) (mk_usize 261631) $t1
"#))]
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${t1}_future == Seq.length $t1 /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 4211177) ${t1}_future
"#))]
pub(crate) fn compute_w_approx<SIMDUnit: Operations>(
    rows_in_a: usize,
    columns_in_a: usize,
    matrix: &[PolynomialRingElement<SIMDUnit>],
    signer_response: &[PolynomialRingElement<SIMDUnit>],
    verifier_challenge_as_ntt: &PolynomialRingElement<SIMDUnit>,
    t1: &mut [PolynomialRingElement<SIMDUnit>],
) {
    // Snapshot t1 so the outer-loop frame can carry "tail unchanged from
    // function-entry" (the per-lane non-negative pre needed by
    // shift_left_then_reduce).  The pre on old_t1 is wrapped in the
    // opaque is_lane_range_poly_slice predicate to keep the inv lean.
    #[cfg(hax)]
    let old_t1: &[PolynomialRingElement<SIMDUnit>] = t1.to_vec().as_slice();
    // Bridge: is_lane_range_poly_slice 0 261631 t1 (function pre) carries to
    // old_t1 because old_t1 == t1 elementwise.  Via Classical.forall_intro
    // of the per-element lookup, then the slice intro lemma.
    hax_lib::fstar!(
        r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length old_t1}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly
                     (mk_usize 0) (mk_usize 261631) (Seq.index old_t1 k)) =
            assert (Seq.index old_t1 k == Seq.index $t1 k);
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_lane_range_poly_slice_lookup
              (mk_usize 0) (mk_usize 261631) $t1 k
          in
          Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_lane_range_poly_slice_intro
          (mk_usize 0) (mk_usize 261631) old_t1;
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
          (mk_usize 4211177) (mk_usize 0) (mk_usize 0) $t1
        "#
    );
    for i in 0..rows_in_a {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v $i <= v $rows_in_a /\
              Seq.length $matrix == v $rows_in_a * v $columns_in_a /\
              Seq.length $signer_response == v $columns_in_a /\
              Seq.length $t1 == v $rows_in_a /\
              Seq.length old_t1 == v $rows_in_a /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $matrix /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) $signer_response /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 75423744) $verifier_challenge_as_ntt /\
              Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                  (mk_usize 4211177) (mk_usize 0) $i $t1 /\
              (forall (k:nat). v $i <= k /\ k < Seq.length $t1 ==>
                  Seq.index $t1 k == Seq.index old_t1 k) /\
              Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice (mk_usize 0) (mk_usize 261631) old_t1"#
        ));

        // Iter-start snapshot of t1: needed so that the post-body
        // inv re-establishment can bridge from
        //   `is_bounded_poly_range 4_211_177 0 i iter_start_t1` (from inv)
        // to
        //   `is_bounded_poly_range 4_211_177 0 i t1_after_body`
        // when t1_after_body == update_at_usize iter_start_t1 (v i) (...)
        // (i.e., body only mutated index v i).
        #[cfg(hax)]
        let iter_start_t1: &[PolynomialRingElement<SIMDUnit>] = t1.to_vec().as_slice();

        let mut inner_result = PolynomialRingElement::<SIMDUnit>::zero();
        // Lift `zero`'s per-lane `is_i32b_array_opaque 0` post to
        // `is_bounded_poly 0 inner_result`.
        hax_lib::fstar!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_intro
                 (mk_usize 0) $inner_result"#
        );

        for j in 0..columns_in_a {
            hax_lib::loop_invariant!(|j: usize| fstar!(
                r#"v $j <= v $columns_in_a /\
                  v $i < v $rows_in_a /\
                  Seq.length $matrix == v $rows_in_a * v $columns_in_a /\
                  Seq.length $signer_response == v $columns_in_a /\
                  Seq.length $t1 == v $rows_in_a /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $matrix /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 75423744) $signer_response /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 75423744) $verifier_challenge_as_ntt /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                      ($j *! mk_usize 8380416) $inner_result /\
                  (forall (k:nat). v $i <= k /\ k < Seq.length $t1 ==>
                      Seq.index $t1 k == Seq.index old_t1 k) /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                      (mk_usize 4211177) (mk_usize 0) $i $t1 /\
                  Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice (mk_usize 0) (mk_usize 261631) old_t1"#
            ));

            let mut product = matrix[i * columns_in_a + j];
            // matrix[i*cols+j] is_bounded_poly FIELD_MAX (via slice SMTPat);
            // weaken FIELD_MAX -> NTT_OUTPUT_BOUND for ntt_multiply's widened
            // (both-operands) pre.  signer_response[j] is the NTT_OUTPUT_BOUND rhs.
            hax_lib::fstar!(
                r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_higher
                     (mk_usize 8380416) (mk_usize 75423744) $product"#
            );
            ntt_multiply_montgomery::<SIMDUnit>(&mut product, &signer_response[j]);
            // post: product is_bounded_poly FIELD_MAX.
            add_to_ring_element::<SIMDUnit>(&mut inner_result, &product, j * 8380416, 8380416);
            // post: inner_result is_bounded_poly ((j+1)*FIELD_MAX); bridge via
            // distributivity_add_left so the inner inv at j+1 fires.
            hax_lib::fstar!(
                r#"assert (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                            ($j *! mk_usize 8380416 +! mk_usize 8380416) $inner_result);
                   Math.Lemmas.distributivity_add_left (v $j) 1 8380416"#
            );
        }
        // After inner: is_bounded_poly (cols * FIELD_MAX) inner_result.

        // The outer frame says `t1[i] == old_t1[i]`; the outer inv says
        // `is_lane_range_poly_slice 0 261631 old_t1`.  Anchor the chain so
        // the dual-SMTPat lookup fires on `(is_lane_range_poly_slice ...,
        // Seq.index old_t1 (v $i))` → `is_lane_range_poly 0 261631 (t1[v $i])`,
        // which is the helper's pre.
        hax_lib::fstar!(
            r#"assert (Seq.index $t1 (v $i) == Seq.index old_t1 (v $i))"#
        );
        compose_w_approx_per_row::<SIMDUnit>(
            &mut t1[i],
            inner_result,
            verifier_challenge_as_ntt,
            columns_in_a,
        );
        // Helper post: is_bounded_poly 4_211_177 t1[i].  Anchor the bridge.
        hax_lib::fstar!(
            r#"assert (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                         (mk_usize 4211177) (Seq.index $t1 (v $i)))"#
        );

        // Re-establish the outer carryover at i+1 via standalone lemma.
        // The lemma is verified in clean context (in polynomial.rs) so the
        // trivial `k = v $i` and Seq-frame assertions don't get polluted by
        // compute_w_approx's heavy ambient context.
        hax_lib::fstar!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_extend_after_update
                 (mk_usize 4211177) $i iter_start_t1 $t1"#
        );
    }
    // After outer: is_bounded_poly_range 4_211_177 0 rows_in_a t1.  Bridge
    // to slice form for the post.
    hax_lib::fstar!(
        r#"
        let _:Prims.unit =
          let aux (k: nat{k < Seq.length ${t1}}) :
            Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 4211177)
                     (Seq.index $t1 k)) =
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
              (mk_usize 4211177) (mk_usize 0) $rows_in_a $t1 k
          in
          Classical.forall_intro aux
        in
        Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
          (mk_usize 4211177) $t1"#
    );
}

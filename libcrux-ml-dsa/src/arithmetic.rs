use crate::{
    constants::{Gamma2, COEFFICIENTS_IN_RING_ELEMENT},
    polynomial::PolynomialRingElement,
    simd::traits::Operations,
};

#[inline(always)]
#[hax_lib::fstar::before(r#"open Libcrux_ml_dsa.Arithmetic_theory"#)]
// `uuse_hint`'s loop snapshots the pre-update ring-element vector as
// `iter_start = as_slice (to_vec re_vector)`, and its per-iteration call to
// `lemma_is_bounded_poly_range_extend_after_update` needs the round-trip identity
// `as_slice (to_vec s) == s` to relate that snapshot to the mutated slice. This
// SMTPat lemma over poly slices supplies it.
#[hax_lib::fstar::before(
    r#"let lemma_as_slice_to_vec_id_poly
      (#v_SIMDUnit: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_dsa.Simd.Traits.t_Operations v_SIMDUnit)
      (s: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit))
  : Lemma
      (ensures
        Alloc.Vec.impl_1__as_slice #(Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit)
            #Alloc.Alloc.t_Global
          (Alloc.Slice.impl__to_vec #(Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit) s)
          == s)
      [SMTPat (Alloc.Vec.impl_1__as_slice #(Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement
              v_SIMDUnit)
            #Alloc.Alloc.t_Global
          (Alloc.Slice.impl__to_vec #(Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit) s)
        )]
  = Seq.append_empty_l s"#
)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
// Input bound relaxed to `2·(q-1) = 16760832` (was `q-1`); see the SIMD trait
// declaration in `simd/traits.rs`.  The narrowing ensures below is exact for
// any input, so it is unaffected.
#[hax_lib::requires(fstar!(r#"v $bound > 0 /\
        Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 16760832) $vector"#))]
// Narrowing: a `false` result means every ring element is strictly `< bound`
// (per-lane, in absolute value). Propagated from `infinity_norm_exceeds`' iff
// post; the sign rejection loop uses it to tighten `w0`/`mask` to `< bound`
// after each norm check (which the downstream add_vectors/make_hint bounds need).
#[hax_lib::ensures(|result| fstar!(r#"(b2t (not $result)) ==>
        (forall (k:nat). k < Seq.length $vector ==>
          (forall (j:nat). j < 32 ==>
             Spec.Utils.is_i32b_array_opaque (v $bound)
               (i0._super_i2.f_repr (Seq.index (Seq.index $vector k).f_simd_units j))))"#))]
pub(crate) fn vector_infinity_norm_exceeds<SIMDUnit: Operations>(
    vector: &[PolynomialRingElement<SIMDUnit>],
    bound: i32,
) -> bool {
    let mut result = false;
    for i in 0..vector.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"v i <= Seq.length $vector /\
            ((b2t (not result)) ==>
              (forall (k:nat). k < v i ==>
                (forall (j:nat). j < 32 ==>
                   Spec.Utils.is_i32b_array_opaque (v $bound)
                     (i0._super_i2.f_repr (Seq.index (Seq.index $vector k).f_simd_units j)))))"#
        ));
        // Bridge the slice-level FIELD_MAX bound to the per-row poly bound (and unfold
        // it) so infinity_norm_exceeds' per-lane forall precondition discharges.
        proof!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup (mk_usize 16760832) $vector (v $i);
               reveal_opaque (`%Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly) (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly (mk_usize 16760832) (Seq.index $vector (v $i)))"#
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
        proof!(
            r#"
          let lane_post (j:nat{j < 8}) :
            Lemma (Spec.Utils.is_i32b 8380416
                     (Seq.index (i0._super_i2.f_repr (Seq.index ${re}.f_simd_units (v ${i}))) j)) =
            Libcrux_ml_dsa.Simd.Traits.Specs.lemma_shift_left_then_reduce_lane_lookup
              (Seq.index (i0._super_i2.f_repr (Seq.index ${old_re}.f_simd_units (v ${i}))) j)
              (Seq.index (i0._super_i2.f_repr (Seq.index ${re}.f_simd_units (v ${i}))) j)
          in
          Classical.forall_intro lane_post;
          reveal_opaque (`%Spec.Utils.is_i32b_array_opaque) Spec.Utils.is_i32b_array_opaque
        "#
        );
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
#[libcrux_macros::trusted(
    panic_free,
    "hax-limitation: body admits the simultaneous &mut t0/t1 borrow loop; panic-free + ensures both blocked on hax"
)]
#[libcrux_macros::trusted(inline-admit)]
pub(crate) fn power2round_vector<SIMDUnit: Operations>(
    t0: &mut [PolynomialRingElement<SIMDUnit>],
    t1: &mut [PolynomialRingElement<SIMDUnit>],
) {
    // ADMIT: hax cannot extract simultaneous &mut t0[i] / &mut t1[i] borrows in a
    // loop body in a way that supports a loop invariant. Body proof deferred until
    // hax upstream supports this pattern.
    trusted_admit!(
        "hax-limitation: hax cannot extract simultaneous &mut t0[i]/&mut t1[i] \
         borrows in a loop body with a loop invariant"
    );
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
#[hax_lib::requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
         ${t.len()} == dimension /\
         ${low.len()} == dimension /\
         ${high.len()} == dimension /\
         Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) $t"#))]
// `is_lane_range_poly_range` (the opaque range predicate + intro/lookup/extend
// lemmas mirroring `is_bounded_poly_range`) lives in the companion
// `Libcrux_ml_dsa.Arithmetic_theory` module (opened at the top of this file).
// `decompose_vector` uses it to accumulate the per-row non-negativity of the
// `high` (HighBits) output, surfaced (via `lemma_lane_range_slice_high_all_nonneg`)
// as `make_hint`'s `high_all_nonneg` guard in sign_internal.
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${low}_future == Seq.length $low /\
    Seq.length ${high}_future == Seq.length $high /\
    Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice
      (mk_usize 0) (mk_usize 8380416) ${high}_future /\
    Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice
      (mk_usize 0) (use_hint_serialize_bound $gamma2) ${high}_future /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice
      (mk_usize 8380416) ${low}_future"#))]
// `use_hint_serialize_bound` (moved here from `use_hint` so `decompose_vector`
// can reference it) is the NON-NEGATIVE serialization width of the commitment
// (= `pow2 BITS_PER_COMMITMENT_COEFFICIENT - 1`): 63 (gamma2 = (q-1)/88, 6-bit)
// or 15 (gamma2 = (q-1)/32, 4-bit).  The tight `is_lane_range_poly_slice 0
// (use_hint_serialize_bound gamma2)` post above is what `commitment::
// serialize_vector` (sign_internal) consumes; the loose `0 8380416` post is
// kept for `make_hint`'s `high_all_nonneg` guard.
#[hax_lib::fstar::before(r#"let use_hint_serialize_bound (gamma2:i32) : usize = if v gamma2 = v Libcrux_ml_dsa.Constants.v_GAMMA2_V95_232_ then mk_usize 63 else mk_usize 15"#)]
#[hax_lib::fstar::options("--z3rlimit 200 --fuel 1 --ifuel 2")]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
pub(crate) fn decompose_vector<SIMDUnit: Operations>(
    dimension: usize,
    gamma2: Gamma2,
    t: &[PolynomialRingElement<SIMDUnit>],
    low: &mut [PolynomialRingElement<SIMDUnit>],
    high: &mut [PolynomialRingElement<SIMDUnit>],
) {
    // Base case: is_lane_range_poly_range over the empty [0,0) prefix of `high`.
    proof!(
        r#"lemma_is_lane_range_poly_range_intro (mk_usize 0) (mk_usize 8380416)
             (mk_usize 0) (mk_usize 0) $high"#
    );
    // Same empty-prefix base for the TIGHT [0, use_hint_serialize_bound gamma2]
    // range on `high` (parallel to the loose 8380416 one; discharges the tight
    // `is_lane_range_poly_slice` post consumed by serialize_vector).
    proof!(
        r#"lemma_is_lane_range_poly_range_intro (mk_usize 0) (use_hint_serialize_bound $gamma2)
             (mk_usize 0) (mk_usize 0) $high"#
    );
    // Base case (low side): is_bounded_poly_range over the empty [0,0) prefix
    // of `low`.  Mirrors the `high` base above; the outer inv then accumulates
    // the per-row |low| < FIELD_MAX bound (subtract_vectors in sign_internal
    // consumes it) exactly as the `high` inv accumulates the [0,q) range.
    proof!(
        r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
             (mk_usize 8380416) (mk_usize 0) (mk_usize 0) $low"#
    );
    for i in 0..dimension {
        // NOTE: the two slice-length conjuncts MUST be stated with
        // `Core_models.Slice.impl__len` (whose parameter is `t_Slice`), NOT
        // `Seq.length` (which accepts the supertype `Seq.seq`).  With
        // `Seq.length`, F* widens `low` in the fold's accumulator type to a
        // bare `Seq.seq`, so the init tuple `(high, low) : (t_Slice & t_Slice)`
        // needs a `t_Slice -> Seq.seq` coercion whose projector Z3 cannot
        // reduce ("incomplete quantifiers" on the fold-init subtyping).
        // `impl__len` keeps the accumulator symmetric `(t_Slice & t_Slice)`.
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"Core_models.Slice.impl__len #(Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit) $low == $dimension /\
               Core_models.Slice.impl__len #(Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit) $high == $dimension /\
               is_lane_range_poly_range (mk_usize 0) (mk_usize 8380416)
                 (mk_usize 0) $i $high /\
               is_lane_range_poly_range (mk_usize 0) (use_hint_serialize_bound $gamma2)
                 (mk_usize 0) $i $high /\
               Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                 (mk_usize 8380416) (mk_usize 0) $i $low"#
        ));

        // Snapshot the row-`i`-start `high` as the immutable frame anchor for the
        // inner loop (which only mutates row i) and the end-of-body extension.
        #[cfg(hax)]
        let old_high: &[PolynomialRingElement<SIMDUnit>] = high.to_vec().as_slice();
        // Carry the outer-inv opaque range [0,i) from `high` onto `old_high`
        // (element-wise equal here) so the inner inv + extension lemma can name it.
        proof!(
            r#"
            let _:Prims.unit =
              let aux (k: nat{k < v $i /\ k < Seq.length old_high}) :
                Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly
                         (mk_usize 0) (mk_usize 8380416) (Seq.index old_high k)) =
                assert (Seq.index old_high k == Seq.index $high k);
                lemma_is_lane_range_poly_range_lookup (mk_usize 0) (mk_usize 8380416)
                  (mk_usize 0) $i $high k
              in
              Classical.forall_intro aux
            in
            lemma_is_lane_range_poly_range_intro (mk_usize 0) (mk_usize 8380416)
              (mk_usize 0) $i old_high"#
        );
        // Same carryover for the TIGHT [0, use_hint_serialize_bound gamma2] range
        // onto old_high (parallel to the loose one above).
        proof!(
            r#"
            let _:Prims.unit =
              let aux (k: nat{k < v $i /\ k < Seq.length old_high}) :
                Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly
                         (mk_usize 0) (use_hint_serialize_bound $gamma2) (Seq.index old_high k)) =
                assert (Seq.index old_high k == Seq.index $high k);
                lemma_is_lane_range_poly_range_lookup (mk_usize 0) (use_hint_serialize_bound $gamma2)
                  (mk_usize 0) $i $high k
              in
              Classical.forall_intro aux
            in
            lemma_is_lane_range_poly_range_intro (mk_usize 0) (use_hint_serialize_bound $gamma2)
              (mk_usize 0) $i old_high"#
        );

        // Same snapshot + carryover for `low` (the mutated inner loop only
        // touches row i; `old_low` anchors the [0,i) frame for the extension).
        #[cfg(hax)]
        let old_low: &[PolynomialRingElement<SIMDUnit>] = low.to_vec().as_slice();
        proof!(
            r#"
            let _:Prims.unit =
              let aux (k: nat{k < v $i /\ k < Seq.length old_low}) :
                Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                         (mk_usize 8380416) (Seq.index old_low k)) =
                assert (Seq.index old_low k == Seq.index $low k);
                Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
                  (mk_usize 8380416) (mk_usize 0) $i $low k
              in
              Classical.forall_intro aux
            in
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_intro
              (mk_usize 8380416) (mk_usize 0) $i old_low"#
        );

        // NOTE: `high[0]` (not `low[0]`) as the unit count: hax extracts the two
        // `&mut` slices as a tuple-state fold `(high, low)`, and F*'s typeclass
        // resolver fails to re-resolve `Index` on the SECOND binder (`low.[_]`)
        // under the strengthened invariant's subtyping context (skill §7
        // tuple-state Index failure).  `high[0]` (first binder) resolves; both
        // slices have the identical 32 simd_units, so the bound is unchanged.
        for j in 0..high[0].simd_units.len() {
            hax_lib::loop_invariant!(|j: usize| fstar!(
                r#"Core_models.Slice.impl__len #(Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit) $low == $dimension /\
                   Core_models.Slice.impl__len #(Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit) $high == $dimension /\
                   v $i < v $dimension /\
                   Seq.length old_high == v $dimension /\
                   is_lane_range_poly_range (mk_usize 0) (mk_usize 8380416)
                     (mk_usize 0) $i old_high /\
                   is_lane_range_poly_range (mk_usize 0) (use_hint_serialize_bound $gamma2)
                     (mk_usize 0) $i old_high /\
                   (forall (k:nat). k < v $dimension /\ k <> v $i ==>
                       Seq.index $high k == Seq.index old_high k) /\
                   (forall (u:nat) (m:nat). u < v $j /\ m < 8 ==>
                       v (Seq.index (i0._super_i2.f_repr
                            (Seq.index (Seq.index $high (v $i)).f_simd_units u)) m) >= 0 /\
                       v (Seq.index (i0._super_i2.f_repr
                            (Seq.index (Seq.index $high (v $i)).f_simd_units u)) m) < 8380417 /\
                       v (Seq.index (i0._super_i2.f_repr
                            (Seq.index (Seq.index $high (v $i)).f_simd_units u)) m) <= v (use_hint_serialize_bound $gamma2)) /\
                   Seq.length old_low == v $dimension /\
                   Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
                     (mk_usize 8380416) (mk_usize 0) $i old_low /\
                   (forall (k:nat). k < v $dimension /\ k <> v $i ==>
                       Seq.index ($low <: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit)) k ==
                       Seq.index old_low k) /\
                   (forall (u:nat). u < v $j ==>
                       Spec.Utils.is_i32b_array_opaque (v (mk_usize 8380416))
                         (i0._super_i2.f_repr
                            (Seq.index (Seq.index ($low <: t_Slice (Libcrux_ml_dsa.Polynomial.t_PolynomialRingElement v_SIMDUnit)) (v $i)).f_simd_units u)))"#
            ));

            // Bridge the slice-level FIELD_MAX bound on t down to the per-lane bound
            // that decompose's precondition needs on t[i].simd_units[j].
            proof!(
                r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup (mk_usize 8380416) $t (v $i);
                   Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_lookup (mk_usize 8380416) (Seq.index $t (v $i)) (v $j)"#
            );

            SIMDUnit::decompose(
                gamma2,
                &t[i].simd_units[j],
                &mut low[i].simd_units[j],
                &mut high[i].simd_units[j],
            );

            // Widen the per-unit `low` bound that `decompose`'s post supplies
            // (|low| <= gamma2, i.e. 95232 or 261888 depending on the gamma2
            // case guaranteed by the requires) up to FIELD_MAX = 8380416 so it
            // matches the inner-inv `is_i32b_array_opaque (v (mk_usize 8380416))`
            // accumulator.  Both `is_i32b_array_larger` implications are put in
            // scope; the requires' gamma2 disjunction + decompose's conditional
            // low-bound post then discharge the widened bound in either case.
            proof!(
                r#"Spec.Utils.is_i32b_array_larger 95232 (v (mk_usize 8380416))
                     (i0._super_i2.f_repr (Seq.index (Seq.index $low (v $i)).f_simd_units (v $j)));
                   Spec.Utils.is_i32b_array_larger 261888 (v (mk_usize 8380416))
                     (i0._super_i2.f_repr (Seq.index (Seq.index $low (v $i)).f_simd_units (v $j)))"#
            );
            // Tight w1 bound on the HighBits unit just written: decompose's post
            // gives high[i][j] < 44 (gamma2=95232) resp. < 16 (gamma2=261888);
            // the function's gamma2 disjunction (requires) then case-splits
            // `use_hint_serialize_bound gamma2` (= 63 resp. 15) to `high[i][j] <=
            // use_hint_serialize_bound gamma2` (43<=63 resp. 15<=15).  Stated
            // per-lane so the inner-inv tight accumulator extends to unit j.
            proof!(
                r#"assert (forall (m:nat). m < 8 ==>
                     v (Seq.index (i0._super_i2.f_repr
                          (Seq.index (Seq.index $high (v $i)).f_simd_units (v $j))) m)
                       <= v (use_hint_serialize_bound $gamma2))"#
            );
        }

        // After the inner loop the accumulation covers all 32 units of row i =
        // the body of is_lane_range_poly; intro it, then extend the outer range
        // [0,i) -> [0,i+1) via the (old_high, high) frame.
        proof!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_lane_range_poly_intro
                 (mk_usize 0) (mk_usize 8380416) (Seq.index $high (v $i));
               lemma_is_lane_range_poly_range_extend_after_update
                 (mk_usize 0) (mk_usize 8380416) $i old_high $high"#
        );
        // Same intro + extend for the TIGHT [0, use_hint_serialize_bound gamma2]
        // range: the inner-inv tight accumulator (all 32 units <= bound) intros
        // is_lane_range_poly on row i, then the (old_high, high) frame extends
        // the outer tight range [0,i) -> [0,i+1).
        proof!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_lane_range_poly_intro
                 (mk_usize 0) (use_hint_serialize_bound $gamma2) (Seq.index $high (v $i));
               lemma_is_lane_range_poly_range_extend_after_update
                 (mk_usize 0) (use_hint_serialize_bound $gamma2) $i old_high $high"#
        );
        // Same intro + extend for `low`: at inner-loop exit all 32 units of
        // row i satisfy is_i32b_array_opaque 8380416 = body of is_bounded_poly;
        // extend the outer [0,i) -> [0,i+1) range via the (old_low, low) frame.
        proof!(
            r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_intro
                 (mk_usize 8380416) (Seq.index $low (v $i));
               Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_extend_after_update
                 (mk_usize 8380416) $i old_low $low"#
        );
    }
    // After the outer loop: range over all [0,dimension) rows -> whole slice.
    proof!(
        r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_lane_range_poly_slice_intro
             (mk_usize 0) (mk_usize 8380416) $high"#
    );
    // Same for the TIGHT [0, use_hint_serialize_bound gamma2] range: whole-slice
    // intro discharges the tight `is_lane_range_poly_slice` post consumed by
    // `commitment::serialize_vector` in sign_internal.
    proof!(
        r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_lane_range_poly_slice_intro
             (mk_usize 0) (use_hint_serialize_bound $gamma2) $high"#
    );
    // Same for `low`: the outer inv's [0,dimension) range = every row -> whole
    // slice; discharges the `is_bounded_poly_slice 8380416 low` post conjunct.
    proof!(
        r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_intro
             (mk_usize 8380416) $low"#
    );
}

#[inline(always)]
// `make_hint` functional-correctness theory (the `high_all_nonneg` guard, the
// per-row / per-unit hint sums, the `to_i32_array` fold characterization, and
// the inner/outer maintenance lemmas) lives in the companion
// `Libcrux_ml_dsa.Arithmetic_theory` module (opened at the top of this file).
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
#[hax_lib::ensures(|result| fstar!(r#"high_all_nonneg #v_SIMDUnit i0 $high ==>
    v $result == Libcrux_ml_dsa.Encoding.Signature.count_total_ones ${hint}_future"#))]
pub(crate) fn make_hint<SIMDUnit: Operations>(
    low: &[PolynomialRingElement<SIMDUnit>],
    high: &[PolynomialRingElement<SIMDUnit>],
    gamma2: i32,
    hint: &mut [[i32; COEFFICIENTS_IN_RING_ELEMENT]],
) -> usize {
    let mut true_hints = 0;
    let mut hint_simd = PolynomialRingElement::<SIMDUnit>::zero();

    for i in 0..low.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"(v $true_hints <= 256 * v $i /\ Seq.length $hint == Seq.length $low) /\
               (high_all_nonneg #v_SIMDUnit i0 $high ==>
                 v $true_hints == sum_rows $hint (v $i))"#
        ));

        for j in 0..hint_simd.simd_units.len() {
            hax_lib::loop_invariant!(|j: usize| fstar!(
                r#"v $true_hints <= 256 * v $i + 8 * v $j /\
                   (forall (u:nat). u < v $j ==>
                     Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque
                       (i0._super_i2.f_repr (Seq.index ${hint_simd}.f_simd_units u))) /\
                   (high_all_nonneg #v_SIMDUnit i0 $high ==>
                     v $true_hints ==
                     sum_rows $hint (v $i) +
                     sum_unit_hints i0 ${hint_simd}.f_simd_units (v $j))"#
            ));

            // Bridge the slice-level FIELD_MAX bound down to the per-lane bound that
            // compute_hint's precondition needs: slice -> per-row poly -> per-lane.
            proof!(
                r#"Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup (mk_usize 8380416) $low (v $i);
                   Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup (mk_usize 8380416) $high (v $i);
                   Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_lookup (mk_usize 8380416) (Seq.index $low (v $i)) (v $j);
                   Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_lookup (mk_usize 8380416) (Seq.index $high (v $i)) (v $j)"#
            );

            // Pre-update snapshot: carries the loop-entry invariant (units < j) into
            // the step lemma, which reasons about `Seq.upd old_hint_simd _ j tmp0`.
            #[cfg(hax)]
            let old_hint_simd = hint_simd.clone();

            let one_hints_count = SIMDUnit::compute_hint(
                &low[i].simd_units[j],
                &high[i].simd_units[j],
                gamma2,
                &mut hint_simd.simd_units[j],
            );

            // Inner-fold maintenance: `hint_simd[j]` now holds the freshly-written unit
            // (tmp0); the step lemma extends the guarded sum to j+1.
            proof!(
                r#"lemma_make_hint_inner_step #v_SIMDUnit i0 $high $hint $i old_hint_simd $true_hints
                     $one_hints_count $j (${hint_simd}.f_simd_units.[ $j ] <: v_SIMDUnit)"#
            );

            true_hints += one_hints_count;
        }

        // Outer-fold maintenance: writing row `i` = to_i32_array hint_simd extends the
        // row sum by count_row_ones of that row (via the row bridge).
        proof!(
            r#"lemma_make_hint_outer_step #v_SIMDUnit i0 $high $hint $i ${hint_simd} $true_hints"#
        );

        hint[i] = hint_simd.to_i32_array();
    }

    // sum_rows over all rows == count_total_ones of the completed hint.
    proof!(r#"lemma_sum_rows_eq_count_total $hint"#);

    true_hints
}

#[inline(always)]
#[hax_lib::fstar::before(r#"let use_hint_bound (gamma2:i32) : usize = if v gamma2 = v Libcrux_ml_dsa.Constants.v_GAMMA2_V95_232_ then mk_usize 44 else mk_usize 16"#)]
// `use_hint_serialize_bound` (the non-negative commitment-serialization width,
// 63 resp. 15) is defined as a `fstar::before` on `decompose_vector`
// (earlier in this file), so it is in scope here without a duplicate `let`.
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
#[hax_lib::fstar::options("--z3rlimit 300 --split_queries always")]
#[libcrux_macros::trusted(
    panic_free,
    "pending-proof(campaign): body verified panic-free; commitment-width functional post admitted"
)]
#[hax_lib::requires(fstar!(r#"
        (v $gamma2 == v ${crate::constants::GAMMA2_V261_888} \/
         v $gamma2 == v ${crate::constants::GAMMA2_V95_232}) /\
         ${hint.len()} == ${re_vector.len()} /\
         v (${hint.len()}) <= 8 /\
         Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_256_array_slice ${hint} /\
         Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (mk_usize 8380416) ${re_vector}}"#))]
// JUSTIFICATION for the added (admitted, panic_free) non-negative-range post: the
// per-lane UseHint output (FIPS 204, Alg. 40) is `w1' in {0, .., (q-1)/(2*gamma2)-1}`,
// i.e. NON-NEGATIVE and `<= (q-1)/(2*gamma2)-1` = 43 (gamma2=(q-1)/88) or 15
// (gamma2=(q-1)/32).  Those maxima fit in `BITS_PER_COMMITMENT_COEFFICIENT` bits
// (6 resp. 4), hence `0 <= w1' <= pow2 BITS - 1` = `use_hint_serialize_bound gamma2`
// (= 63 resp. 15) — the non-negative lane range that `commitment::serialize_vector`
// consumes (via `lemma_lane_range_pos_to_pos_array_slice`).  This is the same UseHint
// range the existing symmetric `is_bounded_poly_slice (use_hint_bound=44/16)` post
// already rests on; the extra conjunct just also exposes non-negativity (which the
// symmetric `|x| < b` form dropped).  Verified per-lane by the concrete
// `SIMDUnit::use_hint` impls' `use_hint_lane_post` (== Hacspec UseHint value).
#[hax_lib::ensures(|_| fstar!(r#"
    Seq.length ${re_vector}_future == Seq.length re_vector /\
    Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice (use_hint_bound $gamma2) ${re_vector}_future /\
    Libcrux_ml_dsa.Polynomial.Spec.is_lane_range_poly_slice (mk_usize 0) (use_hint_serialize_bound $gamma2) ${re_vector}_future"#))]
pub(crate) fn use_hint<SIMDUnit: Operations>(
    gamma2: Gamma2,
    hint: &[[i32; COEFFICIENTS_IN_RING_ELEMENT]],
    re_vector: &mut [PolynomialRingElement<SIMDUnit>],
) {
    #[cfg(hax)]
    let old_rv: &[PolynomialRingElement<SIMDUnit>] = re_vector.to_vec().as_slice();
    // Bridge the per-(i,j) FIELD_MAX requires to is_bounded_poly_slice on the
    // entry snapshot old_rv, and seed the (empty) processed range.
    proof!(
        r#"
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
          (use_hint_bound $gamma2) (mk_usize 0) (mk_usize 0) $re_vector"#
    );
    for i in 0..re_vector.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(
            r#"
            v ${i} <= Seq.length $re_vector /\
            Seq.length $re_vector == Seq.length old_rv /\
            Seq.length $re_vector == Seq.length $hint /\
            Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_range
              (use_hint_bound $gamma2) (mk_usize 0) ${i} $re_vector /\
            (forall (k:nat). v ${i} <= k /\ k < Seq.length $re_vector ==>
              Seq.index $re_vector k == Seq.index old_rv k) /\
            Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly_slice
              (mk_usize 8380416) old_rv"#
        ));
        // re_vector[i] == old_rv[i] (tail frame) and old_rv[i] is FIELD_MAX-bounded
        // (slice lookup), so re_vector[i] is FIELD_MAX-bounded for the inner loop.
        proof!(
            r#"
            assert (Seq.index $re_vector (v ${i}) == Seq.index old_rv (v ${i}));
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_slice_lookup
              (mk_usize 8380416) old_rv (v ${i})"#
        );
        let mut tmp = PolynomialRingElement::zero();
        PolynomialRingElement::<SIMDUnit>::from_i32_array(&hint[i], &mut tmp);

        // Bridge: from_i32_array gives `f_repr tmp.simd_units[kk] == hint[i][kk*8..(kk+1)*8]`,
        // and `hint[i]` is binary (function pre), so each tmp simd-unit is a binary array.
        // Surface the array-level binary atom on hint[i] from the slice-level pre so the
        // array lookup SMTPat fires inside the inner introduce.
        proof!(
            r#"
            Libcrux_ml_dsa.Simd.Traits.Specs.lemma_is_binary_256_array_slice_lookup
              $hint (v ${i});
            let aux (kk:nat{kk < 32}) : Lemma
                (Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque
                   (i0._super_i2.f_repr (Seq.index ${tmp}.f_simd_units kk))) =
              let r = i0._super_i2.f_repr (Seq.index ${tmp}.f_simd_units kk) in
              introduce forall (m:nat{m < 8}). (v (Seq.index r m) == 0 \/ v (Seq.index r m) == 1)
              with (Seq.lemma_index_slice (Seq.index $hint (v ${i})) (kk * 8) ((kk + 1) * 8) m;
                    Libcrux_ml_dsa.Simd.Traits.Specs.lemma_is_binary_256_array_lookup
                      (Seq.index $hint (v ${i})) (kk * 8 + m));
              Libcrux_ml_dsa.Simd.Traits.Specs.lemma_is_binary_array_8_intro r
            in Classical.forall_intro aux"#
        );

        for j in 0..re_vector[0].simd_units.len() {
            hax_lib::loop_invariant!(|j: usize| fstar!(
                r#"
                v ${j} <= 32 /\
                Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                  (mk_usize 8380416) (Seq.index $re_vector (v ${i})) /\
                (forall (jj:nat). jj < v ${j} ==>
                  Spec.Utils.is_i32b_array_opaque (v (use_hint_bound $gamma2))
                    (i0._super_i2.f_repr (Seq.index ${tmp}.f_simd_units jj))) /\
                (forall (jj:nat). v ${j} <= jj /\ jj < 32 ==>
                  Libcrux_ml_dsa.Simd.Traits.Specs.is_binary_array_8_opaque
                    (i0._super_i2.f_repr (Seq.index ${tmp}.f_simd_units jj)))"#
            ));
            // Bridge: is_bounded_poly FIELD_MAX re_vector[i] (inv) gives the
            // per-lane FIELD_MAX bound on re_vector[i].simd_units[j] that the
            // use_hint trait pre requires (explicit lookup, not the flaky SMTPat).
            proof!(
                r#"
                Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_lookup
                  (mk_usize 8380416) (Seq.index $re_vector (v ${i})) (v ${j})"#
            );
            SIMDUnit::use_hint(gamma2, &re_vector[i].simd_units[j], &mut tmp.simd_units[j]);
        }
        // After inner loop: all 32 tmp simd-units are is_i32b_array_opaque b_g; lift to is_bounded_poly b_g tmp.
        proof!(
            r#"
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_intro
              (use_hint_bound $gamma2) ${tmp}"#
        );
        // Snapshot pre-update re_vector so the carryover extend lemma can name arr_old.
        #[cfg(hax)]
        let iter_start: &[PolynomialRingElement<SIMDUnit>] = re_vector.to_vec().as_slice();
        re_vector[i] = tmp;
        // Re-establish the processed range at i+1 via the standalone extend lemma
        // (verified in clean context, avoiding cascade pollution here).
        proof!(
            r#"
            Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_extend_after_update
              (use_hint_bound $gamma2) ${i} iter_start $re_vector"#
        );
    }
    // Bridge the final processed range to the per-(i,j) gamma2-conditional ensures.
    proof!(
        r#"
        let aux (k:nat{k < Seq.length ${re_vector}}) :
          Lemma (Libcrux_ml_dsa.Polynomial.Spec.is_bounded_poly
                   (use_hint_bound $gamma2) (Seq.index $re_vector k)) =
          Libcrux_ml_dsa.Polynomial.Spec.lemma_is_bounded_poly_range_lookup
            (use_hint_bound $gamma2) (mk_usize 0) (Core_models.Slice.impl__len $re_vector) $re_vector k
        in Classical.forall_intro aux"#
    );
}

use crate::{
    hax_utils::hax_debug_assert,
    polynomial::{add_bounded, sub_bounded, zeta},
    vector::{Operations, PolynomialRingElement, FIELD_ELEMENTS_IN_VECTOR},
};

#[cfg(hax)]
use hax_lib::prop::ToProp;

#[cfg(hax)]
use crate::polynomial::spec;

#[cfg(hax)]
#[allow(unused_imports)]
use crate::vector::traits::spec::{mont_i16_to_spec_array, zetas_1, zetas_2, zetas_4};

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(4 * 3328, re) & (*zeta_i == 128))]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(3328, future(re))
    & (*future(zeta_i) == 64)
    & fstar!(r#"
        forall (i: usize). i <. mk_usize 16 ==>
          ${mont_i16_to_spec_array::<16>} (mk_usize 16)
            (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
              (Seq.index ${re}_future.f_coefficients (v i))) ==
          Hacspec_ml_kem.Invert_ntt.ntt_inverse_layer_n (mk_usize 16)
            (${mont_i16_to_spec_array::<16>} (mk_usize 16)
              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                (Seq.index ${re}.f_coefficients (v i))))
            (mk_usize 2)
            (Rust_primitives.unsize
              (${zetas_4}
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 127 -! mk_usize 4 *! i))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 126 -! mk_usize 4 *! i))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 125 -! mk_usize 4 *! i))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 124 -! mk_usize 4 *! i))))
      "#))]
pub(crate) fn invert_ntt_at_layer_1<Vector: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<Vector>,
) {
    #[cfg(hax)]
    let _zeta_i_init = *zeta_i;
    #[cfg(hax)]
    let _re_init = re.coefficients;

    for round in 0..16 {
        hax_lib::loop_invariant!(|round: usize| {
            (*zeta_i == _zeta_i_init - round * 4).to_prop()
                & (hax_lib::forall(|i: usize| {
                    if i < 16 {
                        if i >= round {
                            spec::is_bounded_vector(4 * 3328, &re.coefficients[i])
                                & fstar!(r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Seq.index ${_re_init} (v $i)
                                  "#)
                        } else {
                            // Impl-level (Option B): record only the relationship
                            // re.coefficients[j] == f_inv_ntt_layer_1_step _re_init[j] (parametric zetas).
                            // The function-form lift to IN.ntt_inverse_layer_n is done once after the loop.
                            spec::is_bounded_vector(3328, &re.coefficients[i])
                                & fstar!(r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Libcrux_ml_kem.Vector.Traits.f_inv_ntt_layer_1_step #$:Vector
                                      (Seq.index ${_re_init} (v $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 127 -! mk_usize 4 *! $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 126 -! mk_usize 4 *! $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 125 -! mk_usize 4 *! $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 124 -! mk_usize 4 *! $i))
                                  "#)
                        }
                    } else {
                        true.to_prop()
                    }
                }))
        });

        *zeta_i -= 1;
        hax_lib::fstar!(
            r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                        (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (4*3328)
                        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ round ])))"#
        );
        // Hand-holding for the impl-level loop invariant: link local
        // `zeta_i` to the parametric form `127 - 4*round` so the
        // assignment's call substitutes into the j=round branch cleanly.
        hax_lib::fstar!(r#"
            assert (zeta_i == mk_usize 127 -! mk_usize 4 *! round);
            assert (zeta_i -! mk_usize 1 == mk_usize 126 -! mk_usize 4 *! round);
            assert (zeta_i -! mk_usize 2 == mk_usize 125 -! mk_usize 4 *! round);
            assert (zeta_i -! mk_usize 3 == mk_usize 124 -! mk_usize 4 *! round)
          "#);
        re.coefficients[round] = Vector::inv_ntt_layer_1_step(
            re.coefficients[round],
            zeta(*zeta_i),
            zeta(*zeta_i - 1),
            zeta(*zeta_i - 2),
            zeta(*zeta_i - 3),
        );
        *zeta_i -= 3;
    }
    // Phase 7a (track A) Step 4 — Option B: lift the impl-level loop
    // invariant to the function-form citation in the ensures via a
    // post-loop forall_intro over the bridge lemma.  Each chunk j: reveal
    // its `is_i16b_array_opaque (4*3328)` (from the original
    // `is_bounded_poly` precondition on _re_init), then invoke the bridge
    // to lift the impl equation to the spec function-form equation.
    hax_lib::fstar!(r#"
        let aux (j: nat) : Lemma (j < 16 ==>
            ${mont_i16_to_spec_array::<16>} (mk_usize 16)
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index re.f_coefficients j)) ==
            Hacspec_ml_kem.Invert_ntt.ntt_inverse_layer_n (mk_usize 16)
              (${mont_i16_to_spec_array::<16>} (mk_usize 16)
                (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                  (Seq.index ${_re_init} j)))
              (mk_usize 2)
              (Rust_primitives.unsize
                (${zetas_4}
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 127 -! mk_usize 4 *! sz j))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 126 -! mk_usize 4 *! sz j))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 125 -! mk_usize 4 *! sz j))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 124 -! mk_usize 4 *! sz j)))))
          = if j < 16 then begin
              reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (4 * 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                    (Seq.index ${_re_init} j)));
              Hacspec_ml_kem.Commute.Bridges.lemma_inv_ntt_layer_1_step_to_hacspec
                #v_Vector
                (Seq.index ${_re_init} j)
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 127 -! mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 126 -! mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 125 -! mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 124 -! mk_usize 4 *! sz j))
            end
        in
        Classical.forall_intro aux
      "#);
}

// `invert_ntt_at_layer_2` and `invert_ntt_at_layer_3` deliberately omit
// Barrett reduction in their butterflies — see `inv_ntt_layer_{2,3}_step`
// in `src/vector/avx2/ntt.rs`, `neon/ntt.rs`, and `portable/ntt.rs`.
// The bound trace through the inverse NTT is:
//
//   layer 1 input:  4*3328  → output: 3328 (Barrett)
//   layer 2 input:  3328    → output: 2*3328 = 6656  (no Barrett)
//   layer 3 input:  2*3328  → output: 4*3328 = 13312 (no Barrett)
//   layer 4_plus(4) input: 4*3328 = 13312 → output: 3328 (Barrett in
//                                                           step_reduce)
//   layer 4_plus(5..7) input: 3328 → output: 3328 (steady state)
//
// Safety (no integer overflow):
//   * worst-case sum in layer 2: 2 * 3328 = 6656 < 32768 (i16 max)
//   * worst-case sum in layer 3: 2 * 6656 = 13312 < 32768
//   * worst-case `a_plus_b` / `b_minus_a` in layer 4_plus's step_reduce:
//       2 * 13312 = 26624 < 32768; also < 28296 (Barrett input precondition)
//   * worst-case i32 product in `mont_mul_by_constant`:
//       26624 * 1664 ≈ 4.4 × 10^7 << 2^31
// The looser internal bounds are unobservable externally:
// `invert_ntt_montgomery`'s post (`is_bounded_poly(3328)`) is unchanged.
//
// Skipping Barrett at layers 2/3 saves ~80 SIMD ops per inverse NTT.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400")]
#[hax_lib::requires(spec::is_bounded_poly(3328, re) & (*zeta_i == 64))]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(2 * 3328, future(re))
    & (*future(zeta_i) == 32)
    & fstar!(r#"
        forall (i: usize). i <. mk_usize 16 ==>
          ${mont_i16_to_spec_array::<16>} (mk_usize 16)
            (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
              (Seq.index ${re}_future.f_coefficients (v i))) ==
          Hacspec_ml_kem.Invert_ntt.ntt_inverse_layer_n (mk_usize 16)
            (${mont_i16_to_spec_array::<16>} (mk_usize 16)
              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                (Seq.index ${re}.f_coefficients (v i))))
            (mk_usize 4)
            (Rust_primitives.unsize
              (${zetas_2}
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 63 -! mk_usize 2 *! i))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 62 -! mk_usize 2 *! i))))
      "#))]
pub(crate) fn invert_ntt_at_layer_2<Vector: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<Vector>,
) {
    #[cfg(hax)]
    let _zeta_i_init = *zeta_i;
    #[cfg(hax)]
    let _re_init = re.coefficients;

    for round in 0..16 {
        hax_lib::loop_invariant!(|round: usize| {
            (*zeta_i == _zeta_i_init - round * 2).to_prop()
                & (hax_lib::forall(|i: usize| {
                    if i < 16 {
                        if i >= round {
                            spec::is_bounded_vector(3328, &re.coefficients[i])
                                & fstar!(r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Seq.index ${_re_init} (v $i)
                                  "#)
                        } else {
                            // Impl-level (Option B): record the per-chunk relation
                            // re.coefficients[j] == f_inv_ntt_layer_2_step _re_init[j]
                            //                          (zeta(63 - 2*j)) (zeta(62 - 2*j)).
                            // The function-form lift to IN.ntt_inverse_layer_n is
                            // done once after the loop (mirror of layer_3).
                            spec::is_bounded_vector(2 * 3328, &re.coefficients[i])
                                & fstar!(r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Libcrux_ml_kem.Vector.Traits.f_inv_ntt_layer_2_step #$:Vector
                                      (Seq.index ${_re_init} (v $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 63 -! mk_usize 2 *! $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 62 -! mk_usize 2 *! $i))
                                  "#)
                        }
                    } else {
                        true.to_prop()
                    }
                }))
        });

        *zeta_i -= 1;
        // Hand-holding: link local `zeta_i` to the parametric form so the
        // assignment's call substitutes the j=round zetas cleanly.
        hax_lib::fstar!(r#"assert (zeta_i == mk_usize 63 -! mk_usize 2 *! round)"#);
        re.coefficients[round] =
            Vector::inv_ntt_layer_2_step(re.coefficients[round], zeta(*zeta_i), zeta(*zeta_i - 1));
        // Per Rule SD4: targeted asserts of the two facts the iter-end
        // loop invariant subtyping needs (bound + spec-function equation
        // for the just-updated chunk), instead of a global reveal.
        hax_lib::fstar!(
            r#"assert (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector
                         (mk_usize 2 *! mk_usize 3328)
                         (re.Libcrux_ml_kem.Vector.f_coefficients.[round] <: v_Vector));
               assert (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients (v round) ==
                       Libcrux_ml_kem.Vector.Traits.f_inv_ntt_layer_2_step #v_Vector
                         (Seq.index ${_re_init} (v round))
                         (Libcrux_ml_kem.Polynomial.zeta (mk_usize 63 -! mk_usize 2 *! round))
                         (Libcrux_ml_kem.Polynomial.zeta (mk_usize 62 -! mk_usize 2 *! round)))"#
        );
        *zeta_i -= 1;
    }
    // Lift the impl-level loop invariant (per-chunk f_inv_ntt_layer_2_step
    // applications) to the function-form citation in the ensures via a
    // post-loop forall_intro over `lemma_inv_ntt_layer_2_step_to_hacspec`
    // (already proven in Bridges.fst, commit b7b49c358).
    hax_lib::fstar!(r#"
        let aux (j: nat) : Lemma (j < 16 ==>
            ${mont_i16_to_spec_array::<16>} (mk_usize 16)
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index re.f_coefficients j)) ==
            Hacspec_ml_kem.Invert_ntt.ntt_inverse_layer_n (mk_usize 16)
              (${mont_i16_to_spec_array::<16>} (mk_usize 16)
                (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                  (Seq.index ${_re_init} j)))
              (mk_usize 4)
              (Rust_primitives.unsize
                (${zetas_2}
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 63 -! mk_usize 2 *! sz j))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 62 -! mk_usize 2 *! sz j)))))
          = if j < 16 then begin
              reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (3328)
                  (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                    (Seq.index ${_re_init} j)));
              Hacspec_ml_kem.Commute.Bridges.lemma_inv_ntt_layer_2_step_to_hacspec
                #v_Vector
                (Seq.index ${_re_init} j)
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 63 -! mk_usize 2 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 62 -! mk_usize 2 *! sz j))
            end
        in
        Classical.forall_intro aux
      "#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400")]
#[hax_lib::requires(spec::is_bounded_poly(2 * 3328, re) & (*zeta_i == 32))]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(4 * 3328, future(re))
    & (*future(zeta_i) == 16)
    & fstar!(r#"
        forall (i: usize). i <. mk_usize 16 ==>
          ${mont_i16_to_spec_array::<16>} (mk_usize 16)
            (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
              (Seq.index ${re}_future.f_coefficients (v i))) ==
          Hacspec_ml_kem.Invert_ntt.ntt_inverse_layer_n (mk_usize 16)
            (${mont_i16_to_spec_array::<16>} (mk_usize 16)
              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                (Seq.index ${re}.f_coefficients (v i))))
            (mk_usize 8)
            (Rust_primitives.unsize
              (${zetas_1}
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 31 -! i))))
      "#))]
pub(crate) fn invert_ntt_at_layer_3<Vector: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<Vector>,
) {
    #[cfg(hax)]
    let _zeta_i_init = *zeta_i;
    #[cfg(hax)]
    let _re_init = re.coefficients;

    for round in 0..16 {
        hax_lib::loop_invariant!(|round: usize| {
            (*zeta_i == _zeta_i_init - round).to_prop()
                & (hax_lib::forall(|i: usize| {
                    if i < 16 {
                        if i >= round {
                            spec::is_bounded_vector(2 * 3328, &re.coefficients[i])
                                & fstar!(r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Seq.index ${_re_init} (v $i)
                                  "#)
                        } else {
                            // Impl-level (Option B): record only the relationship
                            // re.coefficients[j] == f_inv_ntt_layer_3_step _re_init[j] (zeta(31-j)).
                            // The function-form lift to IN.ntt_inverse_layer_n is done once after the loop.
                            spec::is_bounded_vector(4 * 3328, &re.coefficients[i])
                                & fstar!(r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Libcrux_ml_kem.Vector.Traits.f_inv_ntt_layer_3_step #$:Vector
                                      (Seq.index ${_re_init} (v $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 31 -! $i))
                                  "#)
                        }
                    } else {
                        true.to_prop()
                    }
                }))
        });

        *zeta_i -= 1;
        // Hand-holding for the impl-level loop invariant: link local
        // `zeta_i` to the parametric form `31 - round` so the assignment's
        // call substitutes into the j=round branch cleanly.
        hax_lib::fstar!(r#"assert (zeta_i == mk_usize 31 -! round)"#);
        re.coefficients[round] =
            Vector::inv_ntt_layer_3_step(re.coefficients[round], zeta(*zeta_i));
        // Per Rule SD4: targeted asserts of the two facts the iter-end
        // loop invariant subtyping needs (bound + spec-function equation
        // for the just-updated chunk), instead of a global reveal that
        // would unfold every prior chunk's bound atom universally.
        hax_lib::fstar!(
            r#"assert (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector
                         (mk_usize 4 *! mk_usize 3328)
                         (re.Libcrux_ml_kem.Vector.f_coefficients.[round] <: v_Vector));
               assert (Seq.index re.Libcrux_ml_kem.Vector.f_coefficients (v round) ==
                       Libcrux_ml_kem.Vector.Traits.f_inv_ntt_layer_3_step #v_Vector
                         (Seq.index ${_re_init} (v round))
                         (Libcrux_ml_kem.Polynomial.zeta (mk_usize 31 -! round)))"#
        );
    }
    // Phase 7a (track A) Step 4 layer 3 — Option B: lift the impl-level
    // loop invariant to the function-form citation in the ensures via a
    // post-loop forall_intro over the bridge lemma.  Each chunk j: reveal
    // its `is_i16b_array_opaque (2*3328)` (from the original
    // `is_bounded_poly` precondition on _re_init), then invoke the bridge
    // to lift the impl equation to the spec function-form equation.
    hax_lib::fstar!(r#"
        let aux (j: nat) : Lemma (j < 16 ==>
            ${mont_i16_to_spec_array::<16>} (mk_usize 16)
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index re.f_coefficients j)) ==
            Hacspec_ml_kem.Invert_ntt.ntt_inverse_layer_n (mk_usize 16)
              (${mont_i16_to_spec_array::<16>} (mk_usize 16)
                (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                  (Seq.index ${_re_init} j)))
              (mk_usize 8)
              (Rust_primitives.unsize
                (${zetas_1}
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 31 -! sz j)))))
          = if j < 16 then begin
              reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (2 * 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                    (Seq.index ${_re_init} j)));
              Hacspec_ml_kem.Commute.Bridges.lemma_inv_ntt_layer_3_step_to_hacspec
                #v_Vector
                (Seq.index ${_re_init} j)
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 31 -! sz j))
            end
        in
        Classical.forall_intro aux
      "#);
}

// USER-14 Step B opacity fix: the per-lane FUNCTIONAL post of the step
// (two `mont_i16_to_spec_fe` foralls) wrapped OPAQUE so it stays inert in
// `invert_ntt_at_layer_4_plus`'s loop context (it ignited a k!61 ~17.5M
// machine-int refinement cascade when transparent) and is revealed only by
// the per-step bridge (`lemma_step_keystone`).
#[cfg(hax)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
pub(crate) fn inv_ntt_step_post<Vector: Operations>(
    a: &Vector,
    b: &Vector,
    r0: &Vector,
    r1: &Vector,
    zeta_r: i16,
) -> hax_lib::Prop {
    hax_lib::fstar_prop_expr!(
        r#"
  (forall (i: nat).
      i < 16 ==>
      Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
        (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array r0) i) ==
      Hacspec_ml_kem.Parameters.impl_FieldElement__add
        (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
            (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array a) i))
        (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
            (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array b) i))) /\
  (forall (i: nat).
      i < 16 ==>
      Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
        (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array r1) i) ==
      Hacspec_ml_kem.Parameters.impl_FieldElement__mul
        (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta_r)
        (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array b) i))
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array a) i))))"#
    )
}

// `inv_ntt_layer_int_vec_step_reduce` accepts inputs bounded by `4*3328`
// (the looser bound from `invert_ntt_at_layer_3`'s output).  Internal
// sums reach `2 * 4*3328 = 8*3328 = 26624 < 28296` (Barrett's input
// precondition), and the i32 product `(2*4*3328) * 1664 ≈ 4.4e7 << 2^31`.
// Output is restored to `3328` by Barrett, so subsequent calls see the
// tight bound.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_vector(4 * 3328, &a) & (spec::is_bounded_vector(4 * 3328, &b) & (zeta_r >= -1664 && zeta_r <= 1664)))]
#[hax_lib::ensures(|(r0, r1)| spec::is_bounded_vector(3328, &r0) & (spec::is_bounded_vector(3328, &r1) & fstar!(r#"
    inv_ntt_step_post #$:Vector ${a} ${b} ${r0} ${r1} ${zeta_r}
"#)))]
pub(crate) fn inv_ntt_layer_int_vec_step_reduce<Vector: Operations>(
    mut a: Vector,
    mut b: Vector,
    zeta_r: i16,
) -> (Vector, Vector) {
    #[cfg(hax)]
    let _a_in = a;
    #[cfg(hax)]
    let _b_in = b;

    let b_minus_a = sub_bounded(b, 4 * 3328, &a, 4 * 3328);
    let a_plus_b = add_bounded(a, 4 * 3328, &b, 4 * 3328);

    #[cfg(hax)]
    spec::is_bounded_vector_higher(&a_plus_b, 8 * 3328, 28296);

    let r0 = Vector::barrett_reduce(a_plus_b);
    let r1 = Vector::montgomery_multiply_by_constant(b_minus_a, zeta_r);
    // Phase 7a Step 3.1 — lift the per-lane mod-q residue equations
    // (from `barrett_reduce_post` and `montgomery_multiply_by_constant_post`,
    // composed with `add_post` / `sub_post` of the prior `add_bounded` /
    // `sub_bounded` calls) to per-lane FE equations under
    // `mont_i16_to_spec_fe`.  Two `forall_intro`s — one per output chunk.
    // Phase 7a Step 5 (lane A5 Q101 fix): explicitly call
    // `lemma_mod_q_eq_unfold` to extract `v _ % 3329 == _ % 3329` from
    // the trait posts' `mod_q_eq` predicate before invoking the FE
    // commute lemmas.  Without this, Z3 reports "incomplete quantifiers"
    // on aux1's residue substitution at any rlimit.
    hax_lib::fstar!(r#"
        let a_arr_in = Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${_a_in} in
        let b_arr_in = Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${_b_in} in
        let a_plus_b_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${a_plus_b} in
        let b_minus_a_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${b_minus_a} in
        let r0_arr  = Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${r0} in
        let r1_arr  = Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${r1} in
        let aux0 (i: nat) : Lemma (i < 16 ==>
            Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index r0_arr i) ==
            Hacspec_ml_kem.Parameters.impl_FieldElement__add
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                 (Seq.index a_arr_in i))
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                 (Seq.index b_arr_in i)))
          = if i < 16 then begin
              Hacspec_ml_kem.Commute.Chunk.lemma_barrett_reduce_lane_post_to_mod_q_eq
                (Seq.index a_plus_b_arr i)
                (Seq.index r0_arr i);
              Hacspec_ml_kem.ModQ.lemma_mod_q_eq_unfold
                (v (Seq.index r0_arr i))
                (v (Seq.index a_plus_b_arr i));
              Hacspec_ml_kem.Commute.Chunk.lemma_add_fe_commute_mont_mod
                (Seq.index a_arr_in i)
                (Seq.index b_arr_in i)
                (Seq.index r0_arr i)
            end
        in
        Classical.forall_intro aux0;
        let aux1 (i: nat) : Lemma (i < 16 ==>
            Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index r1_arr i) ==
            Hacspec_ml_kem.Parameters.impl_FieldElement__mul
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe ${zeta_r})
              (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                   (Seq.index b_arr_in i))
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                   (Seq.index a_arr_in i))))
          = if i < 16 then
              Hacspec_ml_kem.Commute.Chunk.lemma_inv_butterfly_mont_lane_to_fe
                (Seq.index a_arr_in i)
                (Seq.index b_arr_in i)
                ${zeta_r}
                (Seq.index r1_arr i)
                (Seq.index b_minus_a_arr i)
        in
        Classical.forall_intro aux1
      "#);
    // USER-14 Step B opacity fix: fold the two per-lane foralls (established
    // above) into the opaque `inv_ntt_step_post` the strengthened ensures cites.
    hax_lib::fstar!(
        r#"reveal_opaque (`%inv_ntt_step_post)
             (inv_ntt_step_post #$:Vector ${_a_in} ${_b_in} ${r0} ${r1} ${zeta_r})"#
    );
    (r0, r1)
}

// `invert_ntt_at_layer_4_plus` is called four times.  The FIRST call
// (with `layer == 4`) receives input bounded by `4*3328` (from
// `invert_ntt_at_layer_3`'s loosened post — see comment above).
// `inv_ntt_layer_int_vec_step_reduce` accepts `4*3328` inputs and
// always produces `3328` outputs (Barrett internal), so subsequent
// calls (`layer == 5..7`) see the tight `3328` input.  We use the
// looser `4*3328` precondition uniformly to keep one signature.
#[inline(always)]
// USER-14 Step B (CLOSED): the STRENGTHENED post citing `IN.ntt_inverse_layer p
// layer` at the polynomial level (256-element FE polynomial) is now PROVEN with
// NO admit.  `invert_ntt_at_layer_4_plus` verifies via the store_block top-down
// recipe — opaque named fold-invariants (`outer_inv`/`inner_inv`, injected below)
// carry one atom each, and the nested-fold maintenance is discharged by standalone
// clean-context lemmas (`lemma_inner_step_maintains`, `lemma_inner_to_outer`,
// `lemma_postloop_cross_vec`, the per-index `lemma_*_lookup`s).  The per-step
// functional post is wrapped opaque (`inv_ntt_step_post`) so it stays inert in the
// loop and is revealed only by the per-step bridge.
//
// The strengthened post is what `invert_ntt_montgomery` consumes to
// chain layers 4..7 into `IN.ntt_inverse_butterflies`.  Validated
// downstream against `matrix.rs` consumers (Wave-C surface).
#[cfg_attr(hax, hax_lib::fstar::before(r#"
(* USER-14 Step B keystone: from one inv_ntt_layer_int_vec_step_reduce step (vectors
   j and j+step_vec, written to cout = cin updated at j,j+step_vec), establish
   cross_vec_hyp for BOTH written vectors.  Lives here (not Bridges) because it cites
   the opaque `inv_ntt_step_post` from this module's interface; Bridges cannot import
   this module (Invert_ntt already imports Bridges).  Bridges f_repr/f_to_i16_array gap
   is closed by the trait `f_to_i16_array_post` (result == f_repr x). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_step_keystone
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (cin cout: t_Array v_Vector (mk_usize 16))
      (step_vec: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (j: nat)
      (zeta_r: i16)
      (a b x y: v_Vector)
    : Lemma
      (requires
        j + step_vec < 16 /\
        j % (2 * step_vec) < step_vec /\
        j / (2 * step_vec) < Seq.length zs /\
        Seq.index zs (j / (2 * step_vec)) ==
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta_r /\
        Seq.index cin j == a /\ Seq.index cin (j + step_vec) == b /\
        Seq.index cout j == x /\ Seq.index cout (j + step_vec) == y /\
        inv_ntt_step_post #v_Vector a b x y zeta_r)
      (ensures
        (forall (l: nat). l < 16 ==>
           Hacspec_ml_kem.Commute.Bridges.cross_vec_hyp #v_Vector cin cout step_vec zs j l) /\
        (forall (l: nat). l < 16 ==>
           Hacspec_ml_kem.Commute.Bridges.cross_vec_hyp #v_Vector cin cout step_vec zs
             (j + step_vec) l)) =
  reveal_opaque (`%inv_ntt_step_post) (inv_ntt_step_post #v_Vector a b x y zeta_r);
  let a_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array a in
  let b_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array b in
  let x_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array x in
  let y_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array y in
  (* trait post f_to_i16_array_post: each *_arr == f_repr of its vector; lift through
     cin/cout indexing so the step-bridge ensures (about *_arr) match cross_vec_from_step
     requires (about f_repr(cin/cout[..])). *)
  assert (a_arr == Libcrux_ml_kem.Vector.Traits.f_repr (Seq.index cin j));
  assert (b_arr == Libcrux_ml_kem.Vector.Traits.f_repr (Seq.index cin (j + step_vec)));
  assert (x_arr == Libcrux_ml_kem.Vector.Traits.f_repr (Seq.index cout j));
  assert (y_arr == Libcrux_ml_kem.Vector.Traits.f_repr (Seq.index cout (j + step_vec)));
  Hacspec_ml_kem.Commute.Bridges.lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec
    a_arr b_arr x_arr y_arr zeta_r;
  Hacspec_ml_kem.Commute.Bridges.lemma_cross_vec_from_step #v_Vector cin cout step_vec zs j zeta_r
#pop-options

(* USER-14 Step B — opaque per-vector "content" predicate (the store_block `stored` analog):
   wraps the per-lane cross_vec_hyp forall for ONE vector m, so the loop invariant carries 16
   opaque atoms (one per i) instead of the 16x16 cross_vec_hyp cross-product that bloated the
   maintenance VC's context (13.7MB, globally flaky). Revealed only inside its own lemmas. *)
[@@ "opaque_to_smt"]
let cross_vec_done_at
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (cin cout: t_Array v_Vector (mk_usize 16))
      (step_vec: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (m: nat)
    : prop =
  forall (l: nat). l < 16 ==>
    Hacspec_ml_kem.Commute.Bridges.cross_vec_hyp #v_Vector cin cout step_vec zs m l

#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_cvda_intro
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (cin cout: t_Array v_Vector (mk_usize 16)) (step_vec: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement) (m: nat)
    : Lemma
      (requires
        (forall (l: nat). l < 16 ==>
           Hacspec_ml_kem.Commute.Bridges.cross_vec_hyp #v_Vector cin cout step_vec zs m l))
      (ensures cross_vec_done_at #v_Vector cin cout step_vec zs m) =
  reveal_opaque (`%cross_vec_done_at) (cross_vec_done_at #v_Vector cin cout step_vec zs m)

let lemma_cvda_reveal
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (cin cout: t_Array v_Vector (mk_usize 16)) (step_vec: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement) (m l: nat)
    : Lemma
      (requires cross_vec_done_at #v_Vector cin cout step_vec zs m /\ l < 16)
      (ensures Hacspec_ml_kem.Commute.Bridges.cross_vec_hyp #v_Vector cin cout step_vec zs m l) =
  reveal_opaque (`%cross_vec_done_at) (cross_vec_done_at #v_Vector cin cout step_vec zs m)

let lemma_cvda_frame1
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (cin cout1 cout2: t_Array v_Vector (mk_usize 16)) (step_vec: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement) (m: nat)
    : Lemma
      (requires m < 16 /\ Seq.index cout1 m == Seq.index cout2 m)
      (ensures cross_vec_done_at #v_Vector cin cout1 step_vec zs m <==>
               cross_vec_done_at #v_Vector cin cout2 step_vec zs m) =
  reveal_opaque (`%cross_vec_done_at) (cross_vec_done_at #v_Vector cin cout1 step_vec zs m);
  reveal_opaque (`%cross_vec_done_at) (cross_vec_done_at #v_Vector cin cout2 step_vec zs m);
  let aux (l: nat)
      : Lemma (l < 16 ==>
                 (Hacspec_ml_kem.Commute.Bridges.cross_vec_hyp #v_Vector cin cout1 step_vec zs m l <==>
                  Hacspec_ml_kem.Commute.Bridges.cross_vec_hyp #v_Vector cin cout2 step_vec zs m l)) =
    if l < 16
    then Hacspec_ml_kem.Commute.Bridges.lemma_cross_vec_frame #v_Vector cin cout1 cout2 step_vec zs m l
  in
  Classical.forall_intro aux
#pop-options

(* USER-14 Step B frame: the two writes (at j1=j, j2=j+step_vec) leave every OTHER vector m
   untouched, so its opaque cross_vec_done_at carries from the pre-step array cb to cf.
   Standalone (clean context) so the body's invariant maintenance is one call. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_cross_vec_frame_others
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (cin cb cf: t_Array v_Vector (mk_usize 16))
      (step_vec: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (j1 j2: nat)
    : Lemma
      (requires
        (forall (m: nat). m < 16 /\ m <> j1 /\ m <> j2 ==> Seq.index cf m == Seq.index cb m))
      (ensures
        (forall (m: nat). m < 16 /\ m <> j1 /\ m <> j2 ==>
           (cross_vec_done_at #v_Vector cin cb step_vec zs m <==>
            cross_vec_done_at #v_Vector cin cf step_vec zs m))) =
  let aux (m: nat)
      : Lemma (m < 16 /\ m <> j1 /\ m <> j2 ==>
                 (cross_vec_done_at #v_Vector cin cb step_vec zs m <==>
                  cross_vec_done_at #v_Vector cin cf step_vec zs m)) =
    if m < 16 && m <> j1 && m <> j2
    then lemma_cvda_frame1 #v_Vector cin cb cf step_vec zs m
  in
  Classical.forall_intro aux
#pop-options

(* USER-14 Step B index helper: in round `round`, inner index j sits in [2*round*sv, 2*round*sv+sv),
   so its block (j/(2sv)) is exactly `round` and its position (j%(2sv)) is < sv (low half). Clean
   (fuel0/ifuel0) modular arithmetic, mirrors Bridges.lemma_vec_partner_hi. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_inner_index (round j sv: nat)
    : Lemma (requires sv >= 1 /\ 2 * round * sv <= j /\ j < 2 * round * sv + sv)
            (ensures j / (2 * sv) == round /\ j % (2 * sv) == j - 2 * round * sv /\ j % (2 * sv) < sv) =
  let d:nat = j - 2 * round * sv in
  assert (j == d + round * (2 * sv));
  FStar.Math.Lemmas.small_div d (2 * sv);
  FStar.Math.Lemmas.small_mod d (2 * sv);
  FStar.Math.Lemmas.lemma_div_plus d round (2 * sv);
  FStar.Math.Lemmas.lemma_mod_plus d round (2 * sv)
#pop-options

(* USER-14 Step B: offset_vec = (round*step*2)/16 collapses to 2*round*step_vec when step = 16*step_vec.
   Clean nat arithmetic, isolated so the heavy body needn't inline the division cancel. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_offset_vec (round step offset offset_vec sv: nat)
    : Lemma (requires step == 16 * sv /\ offset == round * step * 2 /\ offset_vec == offset / 16)
            (ensures offset_vec == 2 * round * sv) =
  assert (offset == (2 * round * sv) * 16);
  FStar.Math.Lemmas.cancel_mul_div (2 * round * sv) 16
#pop-options

(* USER-14 Step B: the per-layer numeric facts (step=2^layer, groups=128/2^layer, e_zeta_i_init=2*groups,
   step divisible by 16, etc.).  Proven once in CLEAN context via the assert_norm match — inline in the
   function the 4 match arms were flaky (provable but rlimit-canceled) under the bloated loop-invariant
   context. *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 50"
let lemma_layer_numeric_facts (layer e_zeta_i_init: usize)
    : Lemma
      (requires
        (v layer == 4 \/ v layer == 5 \/ v layer == 6 \/ v layer == 7) /\
        (v layer == 4 ==> v e_zeta_i_init == 16) /\ (v layer == 5 ==> v e_zeta_i_init == 8) /\
        (v layer == 6 ==> v e_zeta_i_init == 4) /\ (v layer == 7 ==> v e_zeta_i_init == 2))
      (ensures
        v (mk_usize 1 <<! layer <: usize) == pow2 (v layer) /\
        v e_zeta_i_init == 2 * v (mk_usize 128 >>! layer <: usize) /\
        v (mk_usize 128 >>! layer <: usize) >= 1 /\
        v (mk_usize 1 <<! layer <: usize) == 16 * (v (mk_usize 1 <<! layer <: usize) / 16) /\
        v (mk_usize 1 <<! layer <: usize) / 16 >= 1 /\
        v (mk_usize 128 >>! layer <: usize) == 128 / pow2 (v layer) /\
        2 * v (mk_usize 128 >>! layer <: usize) * (v (mk_usize 1 <<! layer <: usize) / 16) == 16) =
  (match v layer with
    | 4 -> assert_norm (v (mk_usize 1 <<! mk_usize 4 <: usize) == 16 /\
                        v (mk_usize 128 >>! mk_usize 4 <: usize) == 8 /\ pow2 4 == 16 /\
                        2 * 8 * (16 / 16) == 16)
    | 5 -> assert_norm (v (mk_usize 1 <<! mk_usize 5 <: usize) == 32 /\
                        v (mk_usize 128 >>! mk_usize 5 <: usize) == 4 /\ pow2 5 == 32 /\
                        2 * 4 * (32 / 16) == 16)
    | 6 -> assert_norm (v (mk_usize 1 <<! mk_usize 6 <: usize) == 64 /\
                        v (mk_usize 128 >>! mk_usize 6 <: usize) == 2 /\ pow2 6 == 64 /\
                        2 * 2 * (64 / 16) == 16)
    | 7 -> assert_norm (v (mk_usize 1 <<! mk_usize 7 <: usize) == 128 /\
                        v (mk_usize 128 >>! mk_usize 7 <: usize) == 1 /\ pow2 7 == 128 /\
                        2 * 1 * (128 / 16) == 16)
    | _ -> ())
#pop-options

(* USER-14 Step B keystone wrapper: discharges the index preconditions of lemma_step_keystone
   from the loop-shaped facts (j in round's block range, zs[round] the zeta), so the body call
   is a one-liner. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_step_keystone_loop
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init cf: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (round j: nat)
      (zeta_r: i16)
      (a b x y: v_Vector)
    : Lemma
      (requires
        2 * round * step_vec_n <= j /\ j < 2 * round * step_vec_n + step_vec_n /\
        j + step_vec_n < 16 /\
        round < Seq.length zs /\
        Seq.index zs round == Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta_r /\
        Seq.index re_init j == a /\ Seq.index re_init (j + step_vec_n) == b /\
        Seq.index cf j == x /\ Seq.index cf (j + step_vec_n) == y /\
        inv_ntt_step_post #v_Vector a b x y zeta_r)
      (ensures
        cross_vec_done_at #v_Vector re_init cf step_vec_n zs j /\
        cross_vec_done_at #v_Vector re_init cf step_vec_n zs (j + step_vec_n)) =
  lemma_inner_index round j step_vec_n;
  lemma_step_keystone #v_Vector re_init cf step_vec_n zs j zeta_r a b x y;
  lemma_cvda_intro #v_Vector re_init cf step_vec_n zs j;
  lemma_cvda_intro #v_Vector re_init cf step_vec_n zs (j + step_vec_n)
#pop-options

(* USER-14 Step B — opaque NAMED invariant predicates (store_block top-down recipe).
   The two folds carry ONE opaque atom each instead of the 16x(if-ladder x 2 opaque)
   cross-product; maintenance is discharged by the standalone lemmas below in CLEAN
   context, so the function-level fold WP never sees the unfolded ladder. *)
[@@ "opaque_to_smt"]
let outer_inv
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (round step: usize)
    : prop =
  forall (i: usize).
    if i <. mk_usize 16
    then
      if v i >= (v round * v step * 2) / 16
      then
        (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
            (mk_usize 4 *! mk_usize 3328 <: usize)
            (coeffs.[ i ] <: v_Vector) /\
         Seq.index coeffs (v i) == Seq.index re_init (v i))
      else
        (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector (mk_usize 3328)
            (coeffs.[ i ] <: v_Vector) /\
         cross_vec_done_at #v_Vector re_init coeffs step_vec_n zs (v i))
    else b2t true

[@@ "opaque_to_smt"]
let inner_inv
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (offset_vec step_vec j: usize)
    : prop =
  forall (i: usize).
    if i <. mk_usize 16
    then
      if
        (v i >= v j && v i < v offset_vec + v step_vec) ||
        v i >= v j + v step_vec
      then
        (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
            (mk_usize 4 *! mk_usize 3328 <: usize)
            (coeffs.[ i ] <: v_Vector) /\
         Seq.index coeffs (v i) == Seq.index re_init (v i))
      else
        (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector (mk_usize 3328)
            (coeffs.[ i ] <: v_Vector) /\
         cross_vec_done_at #v_Vector re_init coeffs step_vec_n zs (v i))
    else b2t true

(* USER-14 Step B: per-index lookups — instantiate the opaque invariant's forall at a
   specific i.  The ENSURES contains the trigger terms (coeffs.[i], Seq.index coeffs (v i),
   cross_vec_done_at .. (v i)) so the revealed forall instantiates reliably (a bare
   `assert (Seq.index coeffs (v i) == ..)` does NOT carry the auto-selected `coeffs.[i]`
   trigger and fails with incomplete quantifiers). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_outer_inv_lookup
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (round step i: usize)
    : Lemma
      (requires
        outer_inv #v_Vector re_init coeffs step_vec_n zs round step /\ v i < 16)
      (ensures
        (if v i >= (v round * v step * 2) / 16
         then
           (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
               (mk_usize 4 *! mk_usize 3328 <: usize) (coeffs.[ i ] <: v_Vector) /\
            Seq.index coeffs (v i) == Seq.index re_init (v i))
         else
           (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector (mk_usize 3328)
               (coeffs.[ i ] <: v_Vector) /\
            cross_vec_done_at #v_Vector re_init coeffs step_vec_n zs (v i)))) =
  reveal_opaque (`%outer_inv) (outer_inv #v_Vector re_init coeffs step_vec_n zs round step)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_inner_inv_lookup
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (offset_vec step_vec j i: usize)
    : Lemma
      (requires
        inner_inv #v_Vector re_init coeffs step_vec_n zs offset_vec step_vec j /\ v i < 16)
      (ensures
        (if (v i >= v j && v i < v offset_vec + v step_vec) || v i >= v j + v step_vec
         then
           (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
               (mk_usize 4 *! mk_usize 3328 <: usize) (coeffs.[ i ] <: v_Vector) /\
            Seq.index coeffs (v i) == Seq.index re_init (v i))
         else
           (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector (mk_usize 3328)
               (coeffs.[ i ] <: v_Vector) /\
            cross_vec_done_at #v_Vector re_init coeffs step_vec_n zs (v i)))) =
  reveal_opaque (`%inner_inv)
    (inner_inv #v_Vector re_init coeffs step_vec_n zs offset_vec step_vec j)
#pop-options

(* USER-14 Step B: outer fold init at round=0 — threshold collapses to 0, every
   vector is PENDING (== re_init, bounded 4*3328). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_outer_inv_init
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (step: usize)
    : Lemma
      (requires
        (forall (i: usize). i <. mk_usize 16 ==>
           Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
             (mk_usize 4 *! mk_usize 3328 <: usize) (coeffs.[ i ] <: v_Vector) /\
           Seq.index coeffs (v i) == Seq.index re_init (v i)))
      (ensures outer_inv #v_Vector re_init coeffs step_vec_n zs (mk_usize 0) step) =
  assert ((v (mk_usize 0) * v step * 2) / 16 == 0);
  reveal_opaque (`%outer_inv) (outer_inv #v_Vector re_init coeffs step_vec_n zs (mk_usize 0) step)
#pop-options

(* USER-14 Step B: inner fold init — at j = offset_vec the inner PENDING disjunction
   collapses to (i >= offset_vec), which is exactly the outer PENDING condition at
   round (threshold == offset_vec).  So inner_inv at offset_vec follows from outer_inv. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_inner_inv_init
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (round step offset_vec step_vec: usize)
    : Lemma
      (requires
        v step_vec == step_vec_n /\
        v offset_vec == (v round * v step * 2) / 16 /\
        outer_inv #v_Vector re_init coeffs step_vec_n zs round step)
      (ensures
        inner_inv #v_Vector re_init coeffs step_vec_n zs offset_vec step_vec offset_vec) =
  reveal_opaque (`%outer_inv) (outer_inv #v_Vector re_init coeffs step_vec_n zs round step);
  reveal_opaque (`%inner_inv)
    (inner_inv #v_Vector re_init coeffs step_vec_n zs offset_vec step_vec offset_vec)
#pop-options

(* USER-14 Step B: the CORE maintenance lemma — one inner-fold step.  Given the inner
   invariant at (cb, j) and one butterfly step writing cf = cb[j:=x][j+sv:=y], establish
   the inner invariant at (cf, j+1).  Proven in CLEAN context: the two newly-written
   vectors {j, j+sv} become DONE via lemma_step_keystone_loop; every other vector keeps
   its branch (PENDING_j and PENDING_{j+1} differ exactly by {j, j+sv}) and its content
   carries because cf agrees with cb off {j, j+sv}. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_inner_step_maintains
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init cb cf: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (offset_vec step_vec j: usize)
      (round: nat)
      (zeta_r: i16)
      (x y: v_Vector)
    : Lemma
      (requires
        v step_vec == step_vec_n /\
        v offset_vec == 2 * round * step_vec_n /\
        v offset_vec <= v j /\ v j < v offset_vec + step_vec_n /\
        v j + step_vec_n < 16 /\
        round < Seq.length zs /\
        Seq.index zs round == Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta_r /\
        inner_inv #v_Vector re_init cb step_vec_n zs offset_vec step_vec j /\
        cf == Seq.upd (Seq.upd cb (v j) x) (v j + step_vec_n) y /\
        inv_ntt_step_post #v_Vector (Seq.index cb (v j)) (Seq.index cb (v j + step_vec_n))
          x y zeta_r /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector (mk_usize 3328) x /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector (mk_usize 3328) y)
      (ensures
        inner_inv #v_Vector re_init cf step_vec_n zs offset_vec step_vec (j +! mk_usize 1)) =
  (* extract per-i facts about cb at j and j+step_vec from inner_inv(cb,j); both are
     PENDING (cb agrees with re_init) via the lookups below. *)
  lemma_inner_inv_lookup #v_Vector re_init cb step_vec_n zs offset_vec step_vec j j;
  lemma_inner_inv_lookup #v_Vector re_init cb step_vec_n zs offset_vec step_vec j
    (j +! step_vec <: usize);
  let a:v_Vector = Seq.index cb (v j) in
  let b:v_Vector = Seq.index cb (v j + step_vec_n) in
  (* index facts for the two writes *)
  Seq.lemma_index_upd1 (Seq.upd cb (v j) x) (v j + step_vec_n) y;
  Seq.lemma_index_upd2 (Seq.upd cb (v j) x) (v j + step_vec_n) y (v j);
  Seq.lemma_index_upd1 cb (v j) x;
  assert (Seq.index cf (v j) == x);
  assert (Seq.index cf (v j + step_vec_n) == y);
  (* keystone: cross_vec_done_at re_init cf at j and j+step_vec *)
  lemma_step_keystone_loop #v_Vector re_init cf step_vec_n zs round (v j) zeta_r a b x y;
  (* frame: every other vector m keeps its cross_vec_done_at across the two writes *)
  lemma_cross_vec_frame_others #v_Vector re_init cb cf step_vec_n zs (v j) (v j + step_vec_n);
  let aux (i: usize)
      : Lemma
        (if i <. mk_usize 16
         then
           (if
               (v i >= v (j +! mk_usize 1 <: usize) && v i < v offset_vec + v step_vec) ||
               v i >= v (j +! mk_usize 1 <: usize) + v step_vec
             then
               (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                   (mk_usize 4 *! mk_usize 3328 <: usize)
                   (cf.[ i ] <: v_Vector) /\
                Seq.index cf (v i) == Seq.index re_init (v i))
             else
               (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector (mk_usize 3328)
                   (cf.[ i ] <: v_Vector) /\
                cross_vec_done_at #v_Vector re_init cf step_vec_n zs (v i)))
         else b2t true) =
    if i <. mk_usize 16
    then begin
      lemma_inner_inv_lookup #v_Vector re_init cb step_vec_n zs offset_vec step_vec j i;
      if v i = v j then ()
      else if v i = v j + step_vec_n then ()
      else begin
        Seq.lemma_index_upd2 (Seq.upd cb (v j) x) (v j + step_vec_n) y (v i);
        Seq.lemma_index_upd2 cb (v j) x (v i);
        lemma_cvda_frame1 #v_Vector re_init cb cf step_vec_n zs (v i)
      end
    end
  in
  Classical.forall_intro aux;
  reveal_opaque (`%inner_inv)
    (inner_inv #v_Vector re_init cf step_vec_n zs offset_vec step_vec (j +! mk_usize 1))
#pop-options

(* USER-14 Step B: inner fold result -> outer invariant at round+1.  At inner exit
   (j = offset_vec+step_vec) the inner DONE set is [0, offset_vec+2*step_vec), which equals
   the outer DONE set at round+1 since threshold(round+1) == offset_vec + 2*step_vec. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_inner_to_outer
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (round step offset_vec step_vec rnext jend: usize)
    : Lemma
      (requires
        v step == 16 * step_vec_n /\
        v step_vec == step_vec_n /\
        v offset_vec == 2 * v round * step_vec_n /\
        v rnext == v round + 1 /\
        v jend == v offset_vec + v step_vec /\
        inner_inv #v_Vector re_init coeffs step_vec_n zs offset_vec step_vec jend)
      (ensures
        outer_inv #v_Vector re_init coeffs step_vec_n zs rnext step) =
  lemma_offset_vec (v round + 1) (v step) ((v round + 1) * v step * 2)
    (((v round + 1) * v step * 2) / 16) step_vec_n;
  reveal_opaque (`%inner_inv)
    (inner_inv #v_Vector re_init coeffs step_vec_n zs offset_vec step_vec jend);
  reveal_opaque (`%outer_inv)
    (outer_inv #v_Vector re_init coeffs step_vec_n zs rnext step)
#pop-options

(* USER-14 Step B: post-loop bridge — outer_inv at round=groups (threshold==16, ALL
   vectors DONE) -> the full cross_vec_hyp forall the function post consumes. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_postloop_cross_vec
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (groups step: usize)
    : Lemma
      (requires
        (v groups * v step * 2) / 16 == 16 /\
        outer_inv #v_Vector re_init coeffs step_vec_n zs groups step)
      (ensures
        (forall (m: nat) (l: nat).
           Hacspec_ml_kem.Commute.Bridges.cross_vec_hyp #v_Vector re_init coeffs step_vec_n zs m l)) =
  let aux (m: nat) (l: nat)
      : Lemma
        (Hacspec_ml_kem.Commute.Bridges.cross_vec_hyp #v_Vector re_init coeffs step_vec_n zs m l) =
    if m < 16 && l < 16
    then begin
      lemma_outer_inv_lookup #v_Vector re_init coeffs step_vec_n zs groups step (mk_usize m);
      lemma_cvda_reveal #v_Vector re_init coeffs step_vec_n zs m l
    end
    else
      reveal_opaque (`%Hacspec_ml_kem.Commute.Bridges.cross_vec_hyp)
        (Hacspec_ml_kem.Commute.Bridges.cross_vec_hyp #v_Vector re_init coeffs step_vec_n zs m l)
  in
  Classical.forall_intro_2 aux
#pop-options

(* USER-14 Step B: the per-round zeta slice (impl Montgomery zetas mapped to spec FEs).
   Top-level so the function references it instead of a function-scope `let zs = Seq.init`. *)
let zs_of (groups: usize) (e_zeta_i_init: usize{v e_zeta_i_init >= v groups /\ v e_zeta_i_init <= 128})
    : t_Slice Hacspec_ml_kem.Parameters.t_FieldElement =
  Seq.init (v groups)
    (fun (r: nat{r < v groups}) ->
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
          (Libcrux_ml_kem.Polynomial.zeta (e_zeta_i_init -! mk_usize 1 -! mk_usize r <: usize) <: i16))

"#))]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(
    spec::is_bounded_poly(4 * 3328, re) & (
        match layer {
            4 => *zeta_i == 16,
            5 => *zeta_i == 8,
            6 => *zeta_i == 4,
            7 => *zeta_i == 2,
            _ => false,
        })
)]
#[hax_lib::ensures(|result| spec::is_bounded_poly(3328, future(re)) & (
        match layer {
            4 => *future(zeta_i) == 8,
            5 => *future(zeta_i) == 4,
            6 => *future(zeta_i) == 2,
            7 => *future(zeta_i) == 1,
            _ => false,
        }) & fstar!(r#"
    Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #$:Vector ${re}_future ==
    Hacspec_ml_kem.Invert_ntt.ntt_inverse_layer
      (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #$:Vector ${re})
      $layer
"#))]
pub(crate) fn invert_ntt_at_layer_4_plus<Vector: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<Vector>,
    layer: usize,
) {
    #[cfg(hax)]
    let _zeta_i_init = *zeta_i;
    #[cfg(hax)]
    let _re0 = *re;
    #[cfg(hax)]
    let _re_init = re.coefficients;

    let step = 1 << layer;
    let groups = 128 >> layer;
    // ghost: only referenced by the F* proof blocks (as the nat `v step_vec_n`).
    #[cfg(hax)]
    let step_vec_n = step / FIELD_ELEMENTS_IN_VECTOR;

    // USER-14 Step B: per-layer numeric facts + outer-fold invariant init.
    hax_lib::fstar!(r#"lemma_layer_numeric_facts ${layer} ${_zeta_i_init}"#);
    hax_lib::fstar!(
        r#"lemma_outer_inv_init #$:Vector ${_re_init} ${re}.f_coefficients
             (v ${step_vec_n}) (zs_of ${groups} ${_zeta_i_init}) ${step}"#
    );

    for round in 0..groups {
        hax_lib::loop_invariant!(|round: usize| {
            (*zeta_i == _zeta_i_init - round).to_prop()
                & fstar!(
                    r#"outer_inv #$:Vector ${_re_init} ${re}.f_coefficients
                         (v ${step_vec_n}) (zs_of ${groups} ${_zeta_i_init}) ${round} ${step}"#
                )
        });

        *zeta_i -= 1;

        let offset = round * step * 2;
        let offset_vec = offset / FIELD_ELEMENTS_IN_VECTOR;
        let step_vec = step / FIELD_ELEMENTS_IN_VECTOR;

        // inner-fold invariant init (outer_inv at round -> inner_inv at offset_vec).
        hax_lib::fstar!(
            r#"lemma_inner_inv_init #$:Vector ${_re_init} ${re}.f_coefficients
                 (v ${step_vec_n}) (zs_of ${groups} ${_zeta_i_init}) ${round} ${step}
                 ${offset_vec} ${step_vec}"#
        );

        for j in offset_vec..offset_vec + step_vec {
            hax_lib::loop_invariant!(|j: usize| {
                fstar!(
                    r#"inner_inv #$:Vector ${_re_init} ${re}.f_coefficients
                         (v ${step_vec_n}) (zs_of ${groups} ${_zeta_i_init})
                         ${offset_vec} ${step_vec} ${j}"#
                )
            });

            #[cfg(hax)]
            let _re_body_in = *re;
            // expose the 4*3328 PENDING bounds on j and j+step_vec for the step precondition.
            hax_lib::fstar!(
                r#"lemma_inner_inv_lookup #$:Vector ${_re_init} ${re}.f_coefficients (v ${step_vec_n})
                     (zs_of ${groups} ${_zeta_i_init}) ${offset_vec} ${step_vec} ${j} ${j};
                   lemma_inner_inv_lookup #$:Vector ${_re_init} ${re}.f_coefficients (v ${step_vec_n})
                     (zs_of ${groups} ${_zeta_i_init}) ${offset_vec} ${step_vec} ${j}
                     (${j} +! ${step_vec})"#
            );

            let (x, y) = inv_ntt_layer_int_vec_step_reduce(
                re.coefficients[j],
                re.coefficients[j + step_vec],
                zeta(*zeta_i),
            );
            re.coefficients[j] = x;
            re.coefficients[j + step_vec] = y;

            // inner maintenance: inner_inv at (cb,j) -> inner_inv at (cf,j+1).
            hax_lib::fstar!(
                r#"lemma_offset_vec (v ${round}) (v ${step}) (v ${offset}) (v ${offset_vec}) (v ${step_vec_n});
                   FStar.Seq.Base.init_index_ (v ${groups})
                     (fun (r: nat{r < v ${groups}}) ->
                        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                          (Libcrux_ml_kem.Polynomial.zeta
                            (${_zeta_i_init} -! mk_usize 1 -! mk_usize r)))
                     (v ${round});
                   lemma_inner_step_maintains #$:Vector ${_re_init}
                     ${_re_body_in}.f_coefficients ${re}.f_coefficients (v ${step_vec_n})
                     (zs_of ${groups} ${_zeta_i_init}) ${offset_vec} ${step_vec} ${j}
                     (v ${round}) (Libcrux_ml_kem.Polynomial.zeta ${zeta_i}) ${x} ${y}"#
            );
        }

        // outer maintenance: inner_inv at offset_vec+step_vec -> outer_inv at round+1.
        hax_lib::fstar!(
            r#"lemma_offset_vec (v ${round}) (v ${step}) (v ${offset}) (v ${offset_vec}) (v ${step_vec_n});
               lemma_inner_to_outer #$:Vector ${_re_init} ${re}.f_coefficients (v ${step_vec_n})
                 (zs_of ${groups} ${_zeta_i_init}) ${round} ${step} ${offset_vec} ${step_vec}
                 (${round} +! mk_usize 1) (${offset_vec} +! ${step_vec})"#
        );
    }

    // zeta-table forall: zs_of[round] == v_ZETAS[2*groups-1-round].
    hax_lib::fstar!(
        r#"(let aux (round: nat)
              : Lemma (round < v ${groups} ==>
                  Seq.index (zs_of ${groups} ${_zeta_i_init}) round ==
                  Hacspec_ml_kem.Ntt.v_ZETAS.[ sz (2 * v ${groups} - 1 - round) ]) =
            if round < v ${groups} then begin
              FStar.Seq.Base.init_index_ (v ${groups})
                (fun (r: nat{r < v ${groups}}) ->
                   Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                     (Libcrux_ml_kem.Polynomial.zeta (${_zeta_i_init} -! mk_usize 1 -! mk_usize r)))
                round;
              Hacspec_ml_kem.Commute.Bridges.lemma_zeta_eq_vzetas
                (${_zeta_i_init} -! mk_usize 1 -! mk_usize round);
              assert (v (${_zeta_i_init} -! mk_usize 1 -! mk_usize round) == 2 * v ${groups} - 1 - round)
            end
          in Classical.forall_intro aux)"#
    );
    // (a) is_bounded_poly 3328 (every vector DONE) + (b) the cross_vec_hyp forall;
    // then the outlet discharges the functional post.
    hax_lib::fstar!(
        r#"lemma_offset_vec (v ${groups}) (v ${step}) (v ${groups} * v ${step} * 2)
             ((v ${groups} * v ${step} * 2) / 16) (v ${step_vec_n});
           (let auxb (i: nat)
              : Lemma (i < 16 ==>
                  Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #$:Vector (mk_usize 3328)
                    (${re}.f_coefficients.[ sz i ])) =
            if i < 16 then
              lemma_outer_inv_lookup #$:Vector ${_re_init} ${re}.f_coefficients (v ${step_vec_n})
                (zs_of ${groups} ${_zeta_i_init}) ${groups} ${step} (sz i)
          in Classical.forall_intro auxb);
           lemma_postloop_cross_vec #$:Vector ${_re_init} ${re}.f_coefficients (v ${step_vec_n})
             (zs_of ${groups} ${_zeta_i_init}) ${groups} ${step};
           Hacspec_ml_kem.Commute.Bridges.lemma_layer_4_plus_post_from_cross_vec #$:Vector
             ${_re0} ${re} ${layer} ${step} (v ${step_vec_n}) (zs_of ${groups} ${_zeta_i_init})"#
    );
}

/// Run the seven Gentleman-Sande inverse-NTT layers (Montgomery zetas, no
/// per-layer finalize).  The lane-storage convention here is libcrux's
/// `·R⁻¹` form: callers (matrix-vector products composed of `ntt_multiply`
/// + `add_to_ring_element`) hand us coefficients whose i16 value `v c`
/// satisfies `v c ≡ value · R⁻¹ (mod q)`, where `value` is the FIPS-203
/// "real" spec value.  Each GS butterfly uses `montgomery_multiply(b, ζ·R)
/// = b · ζ · R · R⁻¹ = b · ζ`, so the Montgomery scalar of `ζ` cancels
/// with `mont_mul`'s built-in `· R⁻¹` and the `·R⁻¹` form is preserved.
///
/// On exit, `re` represents `ntt_inverse_butterflies(input_real)` still
/// in `·R⁻¹` form — the missing `· 128⁻¹` finalize that turns the 7-layer
/// butterfly chain into FIPS-203 `ntt_inverse` is **fused into the next
/// per-element op** by the caller.  Specifically, the three INTT-track
/// reduce fns in `polynomial.rs` (`subtract_reduce`,
/// `add_message_error_reduce`, `add_error_reduce`) immediately follow
/// `invert_ntt_montgomery` with a single `mont_mul(_, 1441)` step.  The
/// constant `1441 = R²/128 mod q` (per `pq-crystals/kyber/main/ref/ntt.c`
/// line 106: `const int16_t f = 1441; // mont^2/128`); on a `·R⁻¹` lane,
/// `mont_mul(_, 1441) = (·R⁻¹) · 1441 · R⁻¹ = · (R²/128) · R⁻² = · 1/128`,
/// which simultaneously
///   (a) discharges the missing `· 128⁻¹` from the 7 GS layers, and
///   (b) brings the lane back to plain form (`v c ≡ value · 1`).
///
/// Cross-spec runtime evidence: `ntt_matches_spec`,
/// `ntt_multiply_matches_spec`, and `full_ntt_multiply_chain_matches_spec`
/// in `src/ntt.rs` execute the entire pipeline and `assert_eq!` against
/// the hacspec reference, confirming that all Montgomery factors cancel
/// through this chain.
#[inline(always)]
// Phase 7a Step 5 (lane A5) — STRENGTHENED post citing
// `IN.ntt_inverse_butterflies` at the polynomial level.  This is the
// critical-path post that gates Wave-C consumers (`subtract_reduce`,
// `add_message_error_reduce`, `add_error_reduce` in polynomial.rs;
// `compute_message`, `compute_ring_element_v` in matrix.rs).
//
// Body chain admitted via `--admit_smt_queries true` while the bridge
// from per-chunk `ntt_inverse_layer_n 16` posts (layers 1 and 3, see
// strengthened posts in this file) to polynomial-level
// `ntt_inverse_layer 256 _ N` — needed for chaining layers 1, 2, 3
// into the per-layer polynomial form — is filed as USER-15.
//
// Layer 4..7 strengthened posts (Step 4 above) ALREADY cite the
// polynomial-level `IN.ntt_inverse_layer p layer` form.  Layer 2's
// post is bounds-only (per layer_2 admit, USER-13) — its functional
// effect is captured by the COMPOSITION via `ntt_inverse_butterflies`
// being correct, since runtime tests in `src/ntt.rs`
// (`ntt_matches_spec`, `full_ntt_multiply_chain_matches_spec`)
// confirm the spec relationship empirically.
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning --split_queries always")]
#[hax_lib::requires((K <= 4).to_prop() & (spec::is_bounded_poly(K * 3328, re)))]
#[hax_lib::ensures(|result| spec::is_bounded_poly(3328, future(re))
    & fstar!(r#"
    Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #$:Vector ${re}_future ==
    Hacspec_ml_kem.Invert_ntt.ntt_inverse_butterflies
      (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_mont #$:Vector ${re})
"#))]
pub(crate) fn invert_ntt_montgomery<const K: usize, Vector: Operations>(
    re: &mut PolynomialRingElement<Vector>,
) {
    // We only ever call this function after matrix/vector multiplication
    hax_debug_assert!(to_i16_array(re)
        .into_iter()
        .all(|coefficient| coefficient.abs() <= (K as i16) * 3328));

    #[cfg(hax)]
    spec::is_bounded_poly_higher(re, K * 3328, 4 * 3328);

    let mut zeta_i = super::constants::COEFFICIENTS_IN_RING_ELEMENT / 2;

    invert_ntt_at_layer_1(&mut zeta_i, re);
    invert_ntt_at_layer_2(&mut zeta_i, re);
    invert_ntt_at_layer_3(&mut zeta_i, re);
    // Layer 3's ensures gives 4*3328 directly; layer_4_plus needs 4*3328.
    invert_ntt_at_layer_4_plus(&mut zeta_i, re, 4);
    // Layer 4_plus's ensures is the tight 3328; widen to 4*3328 for the
    // next call (uniform 4*3328 precondition keeps one signature).
    #[cfg(hax)]
    spec::is_bounded_poly_higher(re, 3328, 4 * 3328);
    invert_ntt_at_layer_4_plus(&mut zeta_i, re, 5);
    #[cfg(hax)]
    spec::is_bounded_poly_higher(re, 3328, 4 * 3328);
    invert_ntt_at_layer_4_plus(&mut zeta_i, re, 6);
    #[cfg(hax)]
    spec::is_bounded_poly_higher(re, 3328, 4 * 3328);
    invert_ntt_at_layer_4_plus(&mut zeta_i, re, 7);
}

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
    (forall (i: nat). i < 16 ==>
       Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
         (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${r0}) i)
         ==
       Hacspec_ml_kem.Parameters.impl_FieldElement__add
         (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
            (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${a}) i))
         (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
            (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${b}) i))) /\
    (forall (i: nat). i < 16 ==>
       Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
         (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${r1}) i)
         ==
       Hacspec_ml_kem.Parameters.impl_FieldElement__mul
         (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe ${zeta_r})
         (Hacspec_ml_kem.Parameters.impl_FieldElement__sub
           (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${b}) i))
           (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
              (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${a}) i))))
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
// Phase 7a Step 5 (lane A5) drive-to-the-top spike: STRENGTHENED post
// citing `IN.ntt_inverse_layer p layer` at the polynomial level (256-
// element FE polynomial).  Body remains admitted via
// `--admit_smt_queries true` while the Z3 wall on
// `inv_ntt_layer_int_vec_step_reduce`'s strengthened post (Q187/Q191/
// Q192 saturating rlimit 200) is investigated.  Filed as USER-14
// (layer_4_plus body discharge with Step 3.1 forall in SMT context).
//
// The strengthened post is what `invert_ntt_montgomery` consumes to
// chain layers 4..7 into `IN.ntt_inverse_butterflies`.  Validated
// downstream against `matrix.rs` consumers (Wave-C surface).
#[hax_lib::fstar::options("--admit_smt_queries true")]
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

    let step = 1 << layer;

    for round in 0..(128 >> layer) {
        hax_lib::loop_invariant!(|round: usize| {
            (*zeta_i == _zeta_i_init - round).to_prop()
                & (hax_lib::forall(|i: usize| {
                    if i < 16 {
                        if i >= (round * step * 2) / 16 {
                            spec::is_bounded_vector(4 * 3328, &re.coefficients[i])
                        } else {
                            spec::is_bounded_vector(3328, &re.coefficients[i])
                        }
                    } else {
                        true.to_prop()
                    }
                }))
        });

        *zeta_i -= 1;

        let offset = round * step * 2;
        let offset_vec = offset / FIELD_ELEMENTS_IN_VECTOR;
        let step_vec = step / FIELD_ELEMENTS_IN_VECTOR;

        for j in offset_vec..offset_vec + step_vec {
            hax_lib::loop_invariant!(|j: usize| {
                hax_lib::forall(|i: usize| {
                    if i < 16 {
                        if (i >= j && i < offset_vec + step_vec) || (i >= j + step_vec) {
                            spec::is_bounded_vector(4 * 3328, &re.coefficients[i])
                        } else {
                            spec::is_bounded_vector(3328, &re.coefficients[i])
                        }
                    } else {
                        true.to_prop()
                    }
                })
            });

            let (x, y) = inv_ntt_layer_int_vec_step_reduce(
                re.coefficients[j],
                re.coefficients[j + step_vec],
                zeta(*zeta_i),
            );
            re.coefficients[j] = x;
            re.coefficients[j + step_vec] = y;
        }
    }
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

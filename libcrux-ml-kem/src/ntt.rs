use crate::{
    hax_utils::hax_debug_assert,
    polynomial::{
        add_bounded, multiply_by_constant_bounded, poly_barrett_reduce, sub_bounded, zeta,
        PolynomialRingElement, VECTORS_IN_RING_ELEMENT,
    },
    vector::Operations,
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
#[hax_lib::requires(spec::is_bounded_poly(_initial_coefficient_bound, re) & (*zeta_i == 63 && _initial_coefficient_bound == 7 * 3328))]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(_initial_coefficient_bound+3328, future(re))
    & (*future(zeta_i) == 127)
    & fstar!(r#"
        forall (i: usize). v i < 16 ==>
          Hacspec_ml_kem.Commute.Ntt_bridge.pv_post #$:Vector
            ${re}.f_coefficients ${re}_future.f_coefficients (mk_usize 2)
            (${zetas_4}
              (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! i))
              (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! i))
              (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! i))
              (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! i)))
            (v i)
      "#))]
pub(crate) fn ntt_at_layer_1<Vector: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<Vector>,
    _initial_coefficient_bound: usize, // This can be used for specifying the range of values allowed in re
) {
    let _zeta_i_init = *zeta_i;
    #[cfg(hax)]
    let _re_init = re.coefficients;

    for round in 0..16 {
        hax_lib::loop_invariant!(|round: usize| {
            (*zeta_i == _zeta_i_init + round * 4).to_prop()
                & (hax_lib::forall(|i: usize| {
                    if i < 16 {
                        if i >= round {
                            spec::is_bounded_vector(_initial_coefficient_bound, &re.coefficients[i])
                                & fstar!(
                                    r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Seq.index ${_re_init} (v $i)
                                  "#
                                )
                        } else {
                            // Impl-level (Option B): record only the relationship
                            // re.coefficients[j] == f_ntt_layer_1_step _re_init[j] (parametric zetas).
                            // Function-form lift to N.ntt_layer_n is done once after the loop.
                            spec::is_bounded_vector(
                                _initial_coefficient_bound + 3328,
                                &re.coefficients[i],
                            ) & fstar!(
                                r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Libcrux_ml_kem.Vector.Traits.f_ntt_layer_1_step #$:Vector
                                      (Seq.index ${_re_init} (v $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! $i))
                                  "#
                            )
                        }
                    } else {
                        true.to_prop()
                    }
                }))
        });
        *zeta_i += 1;
        hax_lib::fstar!(
            r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                        (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7*3328)
                        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ round ])))"#
        );
        // Hand-holding for the impl-level loop invariant: link local
        // `zeta_i` (just incremented to _zeta_i_init + 4*round + 1 = 64 + 4*round)
        // to the parametric form so the assignment substitutes cleanly.
        hax_lib::fstar!(
            r#"
            assert (zeta_i == mk_usize 64 +! mk_usize 4 *! round);
            assert (zeta_i +! mk_usize 1 == mk_usize 65 +! mk_usize 4 *! round);
            assert (zeta_i +! mk_usize 2 == mk_usize 66 +! mk_usize 4 *! round);
            assert (zeta_i +! mk_usize 3 == mk_usize 67 +! mk_usize 4 *! round)
          "#
        );

        re.coefficients[round] = Vector::ntt_layer_1_step(
            re.coefficients[round],
            zeta(*zeta_i),
            zeta(*zeta_i + 1),
            zeta(*zeta_i + 2),
            zeta(*zeta_i + 3),
        );
        *zeta_i += 3;
    }
    // Phase 7a (track A) Step 4 forward — Option B: lift the impl-level
    // loop invariant to function-form citation in the ensures via a
    // post-loop forall_intro over the bridge lemma.  Each chunk j: reveal
    // its `is_i16b_array_opaque (7*3328)` (from the original `is_bounded_poly`
    // precondition on _re_init), then invoke the bridge to lift the impl
    // equation to the spec function-form equation.
    hax_lib::fstar!(
        r#"
        let aux (j: nat) : Lemma (j < 16 ==>
            Hacspec_ml_kem.Commute.Ntt_bridge.pv_post #v_Vector
              ${_re_init} re.f_coefficients (mk_usize 2)
              (${zetas_4}
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! sz j)))
              j)
          = if j < 16 then begin
              reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (7 * 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                    (Seq.index ${_re_init} j)));
              Hacspec_ml_kem.Commute.Bridges.lemma_ntt_layer_1_step_to_hacspec
                #v_Vector
                (Seq.index ${_re_init} j)
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! sz j));
              Hacspec_ml_kem.Commute.Ntt_bridge.pv_post_intro #v_Vector
                ${_re_init} re.f_coefficients (mk_usize 2)
                (${zetas_4}
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! sz j))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! sz j))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! sz j))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! sz j)))
                j
            end
        in
        Classical.forall_intro aux
      "#
    );
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(_initial_coefficient_bound, re) & (*zeta_i == 31 && _initial_coefficient_bound == 6 * 3328))]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(_initial_coefficient_bound+3328, future(re))
    & (*future(zeta_i) == 63)
    & fstar!(r#"
        forall (i: usize). v i < 16 ==>
          Hacspec_ml_kem.Commute.Ntt_bridge.pv_post #$:Vector
            ${re}.f_coefficients ${re}_future.f_coefficients (mk_usize 4)
            (${zetas_2}
              (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! i))
              (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! i)))
            (v i)
      "#))]
pub(crate) fn ntt_at_layer_2<Vector: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<Vector>,
    _initial_coefficient_bound: usize, // This can be used for specifying the range of values allowed in re
) {
    let _zeta_i_init = *zeta_i;
    #[cfg(hax)]
    let _re_init = re.coefficients;

    for round in 0..16 {
        hax_lib::loop_invariant!(|round: usize| {
            (*zeta_i == _zeta_i_init + round * 2).to_prop()
                & (hax_lib::forall(|i: usize| {
                    if i < 16 {
                        if i >= round {
                            spec::is_bounded_vector(_initial_coefficient_bound, &re.coefficients[i])
                                & fstar!(
                                    r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Seq.index ${_re_init} (v $i)
                                  "#
                                )
                        } else {
                            // Impl-level (Option B): record only the relationship
                            // re.coefficients[j] == f_ntt_layer_2_step _re_init[j] (parametric zetas).
                            // Function-form lift to N.ntt_layer_n is done once after the loop.
                            spec::is_bounded_vector(
                                _initial_coefficient_bound + 3328,
                                &re.coefficients[i],
                            ) & fstar!(
                                r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Libcrux_ml_kem.Vector.Traits.f_ntt_layer_2_step #$:Vector
                                      (Seq.index ${_re_init} (v $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! $i))
                                  "#
                            )
                        }
                    } else {
                        true.to_prop()
                    }
                }))
        });
        *zeta_i += 1;
        hax_lib::fstar!(
            r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                        (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6*3328)
                        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ round ])))"#
        );
        // Hand-holding for the impl-level loop invariant: link local
        // `zeta_i` (just incremented to _zeta_i_init + 2*round + 1 = 32 + 2*round)
        // to the parametric form so the assignment substitutes cleanly.
        hax_lib::fstar!(
            r#"
            assert (zeta_i == mk_usize 32 +! mk_usize 2 *! round);
            assert (zeta_i +! mk_usize 1 == mk_usize 33 +! mk_usize 2 *! round)
          "#
        );

        re.coefficients[round] =
            Vector::ntt_layer_2_step(re.coefficients[round], zeta(*zeta_i), zeta(*zeta_i + 1));
        *zeta_i += 1;
    }
    // Phase 7b — Option B: lift the impl-level loop invariant to
    // function-form citation in the ensures via a post-loop forall_intro
    // over the bridge lemma `lemma_ntt_layer_2_step_to_hacspec`.
    hax_lib::fstar!(
        r#"
        let aux (j: nat) : Lemma (j < 16 ==>
            Hacspec_ml_kem.Commute.Ntt_bridge.pv_post #v_Vector
              ${_re_init} re.f_coefficients (mk_usize 4)
              (${zetas_2}
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! sz j)))
              j)
          = if j < 16 then begin
              reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6 * 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                    (Seq.index ${_re_init} j)));
              Hacspec_ml_kem.Commute.Bridges.lemma_ntt_layer_2_step_to_hacspec
                #v_Vector
                (Seq.index ${_re_init} j)
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! sz j));
              Hacspec_ml_kem.Commute.Ntt_bridge.pv_post_intro #v_Vector
                ${_re_init} re.f_coefficients (mk_usize 4)
                (${zetas_2}
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! sz j))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! sz j)))
                j
            end
        in
        Classical.forall_intro aux
      "#
    );
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(_initial_coefficient_bound, re) & (*zeta_i == 15 && _initial_coefficient_bound == 5 * 3328))]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(_initial_coefficient_bound+3328, future(re))
    & (*future(zeta_i) == 31)
    & fstar!(r#"
        forall (i: usize). v i < 16 ==>
          Hacspec_ml_kem.Commute.Ntt_bridge.pv_post #$:Vector
            ${re}.f_coefficients ${re}_future.f_coefficients (mk_usize 8)
            (${zetas_1}
              (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! i)))
            (v i)
      "#))]
pub(crate) fn ntt_at_layer_3<Vector: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<Vector>,
    _initial_coefficient_bound: usize, // This can be used for specifying the range of values allowed in re
) {
    let _zeta_i_init = *zeta_i;
    #[cfg(hax)]
    let _re_init = re.coefficients;

    for round in 0..16 {
        hax_lib::loop_invariant!(|round: usize| {
            (*zeta_i == _zeta_i_init + round).to_prop()
                & (hax_lib::forall(|i: usize| {
                    if i < 16 {
                        if i >= round {
                            spec::is_bounded_vector(_initial_coefficient_bound, &re.coefficients[i])
                                & fstar!(
                                    r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Seq.index ${_re_init} (v $i)
                                  "#
                                )
                        } else {
                            // Impl-level (Option B): record only the relationship
                            // re.coefficients[j] == f_ntt_layer_3_step _re_init[j] (single zeta).
                            // Function-form lift to N.ntt_layer_n is done once after the loop.
                            spec::is_bounded_vector(
                                _initial_coefficient_bound + 3328,
                                &re.coefficients[i],
                            ) & fstar!(
                                r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Libcrux_ml_kem.Vector.Traits.f_ntt_layer_3_step #$:Vector
                                      (Seq.index ${_re_init} (v $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! $i))
                                  "#
                            )
                        }
                    } else {
                        true.to_prop()
                    }
                }))
        });
        *zeta_i += 1;
        hax_lib::fstar!(
            r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                        (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (5*3328)
                        (Libcrux_ml_kem.Vector.Traits.f_to_i16_array (re.f_coefficients.[ round ])))"#
        );
        // Hand-holding: link local `zeta_i` (just incremented to
        // _zeta_i_init + round + 1 = 16 + round) to the parametric form.
        hax_lib::fstar!(
            r#"
            assert (zeta_i == mk_usize 16 +! round)
          "#
        );

        re.coefficients[round] = Vector::ntt_layer_3_step(re.coefficients[round], zeta(*zeta_i));
    }
    // Phase 7b — Option B: lift the impl-level loop invariant to
    // function-form citation in the ensures via a post-loop forall_intro
    // over the bridge lemma `lemma_ntt_layer_3_step_to_hacspec`.
    hax_lib::fstar!(
        r#"
        let aux (j: nat) : Lemma (j < 16 ==>
            Hacspec_ml_kem.Commute.Ntt_bridge.pv_post #v_Vector
              ${_re_init} re.f_coefficients (mk_usize 8)
              (${zetas_1}
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! sz j)))
              j)
          = if j < 16 then begin
              reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (5 * 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                    (Seq.index ${_re_init} j)));
              Hacspec_ml_kem.Commute.Bridges.lemma_ntt_layer_3_step_to_hacspec
                #v_Vector
                (Seq.index ${_re_init} j)
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! sz j));
              Hacspec_ml_kem.Commute.Ntt_bridge.pv_post_intro #v_Vector
                ${_re_init} re.f_coefficients (mk_usize 8)
                (${zetas_1}
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! sz j)))
                j
            end
        in
        Classical.forall_intro aux
      "#
    );
}

// F-B: opaque per-step forward butterfly functional post (MONT form).  Mirror of
// the inverse `inv_ntt_step_post`, but with the Cooley-Tukey butterfly:
//   mont(x[i]) == add (mont a[i]) (mul (mont zeta_r) (mont b[i]))   (butterfly._1)
//   mont(y[i]) == sub (mont a[i]) (mul (mont zeta_r) (mont b[i]))   (butterfly._2)
// Wrapped opaque so it stays inert in `ntt_at_layer_4_plus`'s loop context and is
// revealed only by the per-step bridge `lemma_step_keystone_fwd`.
#[cfg(hax)]
#[hax_lib::fstar::before(r#"[@@ "opaque_to_smt"]"#)]
pub(crate) fn ntt_step_post<Vector: Operations>(
    a: &Vector,
    b: &Vector,
    x: &Vector,
    y: &Vector,
    zeta_r: i16,
) -> hax_lib::Prop {
    hax_lib::fstar_prop_expr!(
        r#"
  (forall (i: nat).
      i < 16 ==>
      Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
        (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array x) i) ==
      Hacspec_ml_kem.Parameters.impl_FieldElement__add
        (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
            (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array a) i))
        (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta_r)
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array b) i)))) /\
  (forall (i: nat).
      i < 16 ==>
      Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
        (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array y) i) ==
      Hacspec_ml_kem.Parameters.impl_FieldElement__sub
        (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
            (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array a) i))
        (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta_r)
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                (Seq.index (Libcrux_ml_kem.Vector.Traits.f_to_i16_array b) i))))"#
    )
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_vector(_initial_coefficient_bound, &a) & (zeta_r >= -1664 && zeta_r <= 1664 && _initial_coefficient_bound <= 5 * 3328))]
#[hax_lib::ensures(|(r0, r1)| spec::is_bounded_vector(_initial_coefficient_bound+3328, &r0) & (spec::is_bounded_vector(_initial_coefficient_bound+3328, &r1) & fstar!(r#"
    ntt_step_post #$:Vector ${a} ${b} ${r0} ${r1} ${zeta_r}
"#)))]
fn ntt_layer_int_vec_step<Vector: Operations>(
    mut a: Vector,
    mut b: Vector,
    zeta_r: i16,
    _initial_coefficient_bound: usize, // This can be used for specifying the range of values allowed in re
) -> (Vector, Vector) {
    #[cfg(hax)]
    let _a_in = a;
    #[cfg(hax)]
    let _b_in = b;

    let t = Vector::montgomery_multiply_by_constant(b, zeta_r);
    b = sub_bounded(a, _initial_coefficient_bound, &t, 3328);
    a = add_bounded(a, _initial_coefficient_bound, &t, 3328);

    // Lift the per-lane mod-q residue equations (from
    // `montgomery_multiply_by_constant_post` composed with `add_post`/`sub_post`)
    // to per-lane FE equations under `mont_i16_to_spec_fe`.  No barrett (simpler
    // than the inverse).  Two `forall_intro`s — one per output.
    hax_lib::fstar!(
        r#"
        let a_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${_a_in} in
        let b_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${_b_in} in
        let t_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${t} in
        let x_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${a} in
        let y_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array ${b} in
        let aux0 (i: nat) : Lemma (i < 16 ==>
            Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index x_arr i) ==
            Hacspec_ml_kem.Parameters.impl_FieldElement__add
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a_arr i))
              (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe ${zeta_r})
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index b_arr i))))
          = if i < 16 then begin
              Hacspec_ml_kem.Commute.Chunk.lemma_montgomery_multiply_lane_post_to_mod_q_eq
                (Seq.index b_arr i) ${zeta_r} (Seq.index t_arr i);
              Hacspec_ml_kem.ModQ.lemma_mod_q_eq_unfold
                (v (Seq.index t_arr i)) (v (Seq.index b_arr i) * v ${zeta_r} * 169);
              Hacspec_ml_kem.Commute.Chunk.lemma_mont_mul_fe_commute_mont_mont
                ${zeta_r} (Seq.index b_arr i) (Seq.index t_arr i);
              Hacspec_ml_kem.Commute.Chunk.lemma_add_fe_commute_mont
                (Seq.index a_arr i) (Seq.index t_arr i) (Seq.index x_arr i)
            end
        in
        Classical.forall_intro aux0;
        let aux1 (i: nat) : Lemma (i < 16 ==>
            Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index y_arr i) ==
            Hacspec_ml_kem.Parameters.impl_FieldElement__sub
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a_arr i))
              (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe ${zeta_r})
                (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index b_arr i))))
          = if i < 16 then begin
              Hacspec_ml_kem.Commute.Chunk.lemma_montgomery_multiply_lane_post_to_mod_q_eq
                (Seq.index b_arr i) ${zeta_r} (Seq.index t_arr i);
              Hacspec_ml_kem.ModQ.lemma_mod_q_eq_unfold
                (v (Seq.index t_arr i)) (v (Seq.index b_arr i) * v ${zeta_r} * 169);
              Hacspec_ml_kem.Commute.Chunk.lemma_mont_mul_fe_commute_mont_mont
                ${zeta_r} (Seq.index b_arr i) (Seq.index t_arr i);
              Hacspec_ml_kem.Commute.Chunk.lemma_sub_fe_commute_mont
                (Seq.index a_arr i) (Seq.index t_arr i) (Seq.index y_arr i)
            end
        in
        Classical.forall_intro aux1
      "#
    );
    // Fold the two per-lane foralls into the opaque `ntt_step_post`.
    hax_lib::fstar!(
        r#"reveal_opaque (`%ntt_step_post)
             (ntt_step_post #$:Vector ${_a_in} ${_b_in} ${a} ${b} ${zeta_r})"#
    );
    (a, b)
}

#[cfg_attr(hax, hax_lib::fstar::before(r#"
(* ===== Forward layer-4+ cross-vector scaffold (mirror of the inverse
   Invert_ntt.fst USER-14 Step B keystone, with the forward butterfly atom
   `cross_vec_hyp_fwd` and the bound parameterized by e_initial_coefficient_bound). ===== *)

(* Keystone: from one ntt_layer_int_vec_step (vectors j and j+step_vec, written to
   cout = cin updated at j,j+step_vec), establish cross_vec_hyp_fwd for BOTH written
   vectors.  Lives here (not Ntt_bridge) because it cites the module-local opaque
   `ntt_step_post`.  Mirror of Invert_ntt.lemma_step_keystone. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_step_keystone_fwd
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
        ntt_step_post #v_Vector a b x y zeta_r)
      (ensures
        (forall (l: nat). l < 16 ==>
           Hacspec_ml_kem.Commute.Ntt_bridge.cross_vec_hyp_fwd #v_Vector cin cout step_vec zs j l) /\
        (forall (l: nat). l < 16 ==>
           Hacspec_ml_kem.Commute.Ntt_bridge.cross_vec_hyp_fwd #v_Vector cin cout step_vec zs
             (j + step_vec) l)) =
  reveal_opaque (`%ntt_step_post) (ntt_step_post #v_Vector a b x y zeta_r);
  let a_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array a in
  let b_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array b in
  let x_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array x in
  let y_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array y in
  assert (a_arr == Libcrux_ml_kem.Vector.Traits.f_repr (Seq.index cin j));
  assert (b_arr == Libcrux_ml_kem.Vector.Traits.f_repr (Seq.index cin (j + step_vec)));
  assert (x_arr == Libcrux_ml_kem.Vector.Traits.f_repr (Seq.index cout j));
  assert (y_arr == Libcrux_ml_kem.Vector.Traits.f_repr (Seq.index cout (j + step_vec)));
  Hacspec_ml_kem.Commute.Ntt_bridge.lemma_cross_vec_from_step_fwd #v_Vector cin cout step_vec zs j
    zeta_r
#pop-options

(* opaque per-vector "content" predicate (forward); wraps the per-lane
   cross_vec_hyp_fwd forall for ONE vector m.  Mirror of cross_vec_done_at. *)
[@@ "opaque_to_smt"]
let cross_vec_done_at_fwd
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
    Hacspec_ml_kem.Commute.Ntt_bridge.cross_vec_hyp_fwd #v_Vector cin cout step_vec zs m l

#push-options "--fuel 0 --ifuel 1 --z3rlimit 50"
let lemma_cvda_fwd_intro
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (cin cout: t_Array v_Vector (mk_usize 16)) (step_vec: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement) (m: nat)
    : Lemma
      (requires
        (forall (l: nat). l < 16 ==>
           Hacspec_ml_kem.Commute.Ntt_bridge.cross_vec_hyp_fwd #v_Vector cin cout step_vec zs m l))
      (ensures cross_vec_done_at_fwd #v_Vector cin cout step_vec zs m) =
  reveal_opaque (`%cross_vec_done_at_fwd) (cross_vec_done_at_fwd #v_Vector cin cout step_vec zs m)

let lemma_cvda_fwd_reveal
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (cin cout: t_Array v_Vector (mk_usize 16)) (step_vec: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement) (m l: nat)
    : Lemma
      (requires cross_vec_done_at_fwd #v_Vector cin cout step_vec zs m /\ l < 16)
      (ensures Hacspec_ml_kem.Commute.Ntt_bridge.cross_vec_hyp_fwd #v_Vector cin cout step_vec zs m l) =
  reveal_opaque (`%cross_vec_done_at_fwd) (cross_vec_done_at_fwd #v_Vector cin cout step_vec zs m)

let lemma_cvda_fwd_frame1
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()] i0: Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (cin cout1 cout2: t_Array v_Vector (mk_usize 16)) (step_vec: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement) (m: nat)
    : Lemma
      (requires m < 16 /\ Seq.index cout1 m == Seq.index cout2 m)
      (ensures cross_vec_done_at_fwd #v_Vector cin cout1 step_vec zs m <==>
               cross_vec_done_at_fwd #v_Vector cin cout2 step_vec zs m) =
  reveal_opaque (`%cross_vec_done_at_fwd) (cross_vec_done_at_fwd #v_Vector cin cout1 step_vec zs m);
  reveal_opaque (`%cross_vec_done_at_fwd) (cross_vec_done_at_fwd #v_Vector cin cout2 step_vec zs m);
  let aux (l: nat)
      : Lemma (l < 16 ==>
                 (Hacspec_ml_kem.Commute.Ntt_bridge.cross_vec_hyp_fwd #v_Vector cin cout1 step_vec zs m l <==>
                  Hacspec_ml_kem.Commute.Ntt_bridge.cross_vec_hyp_fwd #v_Vector cin cout2 step_vec zs m l)) =
    if l < 16
    then Hacspec_ml_kem.Commute.Ntt_bridge.lemma_cross_vec_frame_fwd #v_Vector cin cout1 cout2 step_vec zs m l
  in
  Classical.forall_intro aux
#pop-options

(* the two writes (at j1=j, j2=j+step_vec) leave every OTHER vector m untouched, so its
   opaque cross_vec_done_at_fwd carries from cb to cf.  Mirror of lemma_cross_vec_frame_others. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_cross_vec_frame_others_fwd
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
           (cross_vec_done_at_fwd #v_Vector cin cb step_vec zs m <==>
            cross_vec_done_at_fwd #v_Vector cin cf step_vec zs m))) =
  let aux (m: nat)
      : Lemma (m < 16 /\ m <> j1 /\ m <> j2 ==>
                 (cross_vec_done_at_fwd #v_Vector cin cb step_vec zs m <==>
                  cross_vec_done_at_fwd #v_Vector cin cf step_vec zs m)) =
    if m < 16 && m <> j1 && m <> j2
    then lemma_cvda_fwd_frame1 #v_Vector cin cb cf step_vec zs m
  in
  Classical.forall_intro aux
#pop-options

(* index helper: in round `round`, inner index j sits in [2*round*sv, 2*round*sv+sv).
   Copied verbatim from Invert_ntt.lemma_inner_index. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_inner_index_fwd (round j sv: nat)
    : Lemma (requires sv >= 1 /\ 2 * round * sv <= j /\ j < 2 * round * sv + sv)
            (ensures j / (2 * sv) == round /\ j % (2 * sv) == j - 2 * round * sv /\ j % (2 * sv) < sv) =
  let d:nat = j - 2 * round * sv in
  assert (j == d + round * (2 * sv));
  FStar.Math.Lemmas.small_div d (2 * sv);
  FStar.Math.Lemmas.small_mod d (2 * sv);
  FStar.Math.Lemmas.lemma_div_plus d round (2 * sv);
  FStar.Math.Lemmas.lemma_mod_plus d round (2 * sv)
#pop-options

(* nonlinear closure for lemma_layer_4_plus_to_poly_step's `(len(zs)*2)*len == 256`
   requires: from len == 16*k and 2*groups*k == 16, derive (groups*2)*len == 256. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_groups_len_256 (groups len k: nat)
    : Lemma (requires len == 16 * k /\ 2 * groups * k == 16)
            (ensures (groups * 2) * len == 256) =
  assert ((groups * 2) * len == (groups * 2) * (16 * k));
  assert ((groups * 2) * (16 * k) == 16 * (2 * groups * k))
#pop-options

(* offset_vec = (round*step*2)/16 collapses to 2*round*step_vec when step = 16*step_vec.
   Copied verbatim from Invert_ntt.lemma_offset_vec. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_offset_vec_fwd (round step offset offset_vec sv: nat)
    : Lemma (requires step == 16 * sv /\ offset == round * step * 2 /\ offset_vec == offset / 16)
            (ensures offset_vec == 2 * round * sv) =
  assert (offset == (2 * round * sv) * 16);
  FStar.Math.Lemmas.cancel_mul_div (2 * round * sv) 16
#pop-options

(* per-layer numeric facts; FORWARD variant: e_zeta_i_init == groups - 1 per layer
   (L7 g=1 init=0; L6 g=2 init=1; L5 g=4 init=3; L4 g=8 init=7). *)
#push-options "--fuel 1 --ifuel 1 --z3rlimit 50"
let lemma_layer_numeric_facts_fwd (layer e_zeta_i_init: usize)
    : Lemma
      (requires
        (v layer == 4 \/ v layer == 5 \/ v layer == 6 \/ v layer == 7) /\
        (v layer == 4 ==> v e_zeta_i_init == 7) /\ (v layer == 5 ==> v e_zeta_i_init == 3) /\
        (v layer == 6 ==> v e_zeta_i_init == 1) /\ (v layer == 7 ==> v e_zeta_i_init == 0))
      (ensures
        v (mk_usize 1 <<! layer <: usize) == pow2 (v layer) /\
        v e_zeta_i_init + 1 == v (mk_usize 128 >>! layer <: usize) /\
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

(* keystone wrapper: discharges index preconditions of lemma_step_keystone_fwd from the
   loop-shaped facts.  Mirror of Invert_ntt.lemma_step_keystone_loop. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_step_keystone_loop_fwd
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
        ntt_step_post #v_Vector a b x y zeta_r)
      (ensures
        cross_vec_done_at_fwd #v_Vector re_init cf step_vec_n zs j /\
        cross_vec_done_at_fwd #v_Vector re_init cf step_vec_n zs (j + step_vec_n)) =
  lemma_inner_index_fwd round j step_vec_n;
  lemma_step_keystone_fwd #v_Vector re_init cf step_vec_n zs j zeta_r a b x y;
  lemma_cvda_fwd_intro #v_Vector re_init cf step_vec_n zs j;
  lemma_cvda_fwd_intro #v_Vector re_init cf step_vec_n zs (j + step_vec_n)
#pop-options

(* opaque NAMED invariant predicates (forward).  Bound parameterized by
   e_initial_coefficient_bound: PENDING = bound (== re_init), DONE = bound+3328 (butterflied). *)
[@@ "opaque_to_smt"]
let outer_inv_fwd
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (e_initial_coefficient_bound: usize{v e_initial_coefficient_bound + 3328 < 65536})
      (round step: usize)
    : prop =
  forall (i: usize).
    if i <. mk_usize 16
    then
      if v i >= (v round * v step * 2) / 16
      then
        (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
            e_initial_coefficient_bound
            (coeffs.[ i ] <: v_Vector) /\
         Seq.index coeffs (v i) == Seq.index re_init (v i))
      else
        (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
            (e_initial_coefficient_bound +! mk_usize 3328 <: usize)
            (coeffs.[ i ] <: v_Vector) /\
         cross_vec_done_at_fwd #v_Vector re_init coeffs step_vec_n zs (v i))
    else b2t true

[@@ "opaque_to_smt"]
let inner_inv_fwd
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (e_initial_coefficient_bound: usize{v e_initial_coefficient_bound + 3328 < 65536})
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
            e_initial_coefficient_bound
            (coeffs.[ i ] <: v_Vector) /\
         Seq.index coeffs (v i) == Seq.index re_init (v i))
      else
        (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
            (e_initial_coefficient_bound +! mk_usize 3328 <: usize)
            (coeffs.[ i ] <: v_Vector) /\
         cross_vec_done_at_fwd #v_Vector re_init coeffs step_vec_n zs (v i))
    else b2t true

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_outer_inv_fwd_lookup
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (e_initial_coefficient_bound: usize{v e_initial_coefficient_bound + 3328 < 65536})
      (round step i: usize)
    : Lemma
      (requires
        outer_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound round step /\
        v i < 16)
      (ensures
        (if v i >= (v round * v step * 2) / 16
         then
           (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
               e_initial_coefficient_bound (coeffs.[ i ] <: v_Vector) /\
            Seq.index coeffs (v i) == Seq.index re_init (v i))
         else
           (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
               (e_initial_coefficient_bound +! mk_usize 3328 <: usize) (coeffs.[ i ] <: v_Vector) /\
            cross_vec_done_at_fwd #v_Vector re_init coeffs step_vec_n zs (v i)))) =
  reveal_opaque (`%outer_inv_fwd)
    (outer_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound round step)
#pop-options

#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_inner_inv_fwd_lookup
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (e_initial_coefficient_bound: usize{v e_initial_coefficient_bound + 3328 < 65536})
      (offset_vec step_vec j i: usize)
    : Lemma
      (requires
        inner_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound offset_vec
          step_vec j /\ v i < 16)
      (ensures
        (if (v i >= v j && v i < v offset_vec + v step_vec) || v i >= v j + v step_vec
         then
           (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
               e_initial_coefficient_bound (coeffs.[ i ] <: v_Vector) /\
            Seq.index coeffs (v i) == Seq.index re_init (v i))
         else
           (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
               (e_initial_coefficient_bound +! mk_usize 3328 <: usize) (coeffs.[ i ] <: v_Vector) /\
            cross_vec_done_at_fwd #v_Vector re_init coeffs step_vec_n zs (v i)))) =
  reveal_opaque (`%inner_inv_fwd)
    (inner_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound offset_vec
        step_vec j)
#pop-options

(* outer fold init at round=0 — threshold collapses to 0, every vector PENDING (== re_init). *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 100"
let lemma_outer_inv_fwd_init
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (e_initial_coefficient_bound: usize{v e_initial_coefficient_bound + 3328 < 65536})
      (step: usize)
    : Lemma
      (requires
        (forall (i: usize). i <. mk_usize 16 ==>
           Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
             e_initial_coefficient_bound (coeffs.[ i ] <: v_Vector) /\
           Seq.index coeffs (v i) == Seq.index re_init (v i)))
      (ensures outer_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound
                 (mk_usize 0) step) =
  assert ((v (mk_usize 0) * v step * 2) / 16 == 0);
  reveal_opaque (`%outer_inv_fwd)
    (outer_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound (mk_usize 0) step)
#pop-options

(* inner fold init — at j = offset_vec, inner PENDING collapses to (i >= offset_vec) = outer PENDING. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_inner_inv_fwd_init
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (e_initial_coefficient_bound: usize{v e_initial_coefficient_bound + 3328 < 65536})
      (round step offset_vec step_vec: usize)
    : Lemma
      (requires
        v step_vec == step_vec_n /\
        v offset_vec == (v round * v step * 2) / 16 /\
        outer_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound round step)
      (ensures
        inner_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound offset_vec
          step_vec offset_vec) =
  reveal_opaque (`%outer_inv_fwd)
    (outer_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound round step);
  reveal_opaque (`%inner_inv_fwd)
    (inner_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound offset_vec
        step_vec offset_vec)
#pop-options

(* the CORE maintenance lemma — one inner-fold step.  Mirror of lemma_inner_step_maintains. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_inner_step_maintains_fwd
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init cb cf: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (e_initial_coefficient_bound: usize{v e_initial_coefficient_bound + 3328 < 65536})
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
        inner_inv_fwd #v_Vector re_init cb step_vec_n zs e_initial_coefficient_bound offset_vec
          step_vec j /\
        cf == Seq.upd (Seq.upd cb (v j) x) (v j + step_vec_n) y /\
        ntt_step_post #v_Vector (Seq.index cb (v j)) (Seq.index cb (v j + step_vec_n))
          x y zeta_r /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
          (e_initial_coefficient_bound +! mk_usize 3328 <: usize) x /\
        Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
          (e_initial_coefficient_bound +! mk_usize 3328 <: usize) y)
      (ensures
        inner_inv_fwd #v_Vector re_init cf step_vec_n zs e_initial_coefficient_bound offset_vec
          step_vec (j +! mk_usize 1)) =
  lemma_inner_inv_fwd_lookup #v_Vector re_init cb step_vec_n zs e_initial_coefficient_bound offset_vec
    step_vec j j;
  lemma_inner_inv_fwd_lookup #v_Vector re_init cb step_vec_n zs e_initial_coefficient_bound offset_vec
    step_vec j (j +! step_vec <: usize);
  let a:v_Vector = Seq.index cb (v j) in
  let b:v_Vector = Seq.index cb (v j + step_vec_n) in
  Seq.lemma_index_upd1 (Seq.upd cb (v j) x) (v j + step_vec_n) y;
  Seq.lemma_index_upd2 (Seq.upd cb (v j) x) (v j + step_vec_n) y (v j);
  Seq.lemma_index_upd1 cb (v j) x;
  assert (Seq.index cf (v j) == x);
  assert (Seq.index cf (v j + step_vec_n) == y);
  lemma_step_keystone_loop_fwd #v_Vector re_init cf step_vec_n zs round (v j) zeta_r a b x y;
  lemma_cross_vec_frame_others_fwd #v_Vector re_init cb cf step_vec_n zs (v j) (v j + step_vec_n);
  let aux (i: usize)
      : Lemma
        (if i <. mk_usize 16
         then
           (if
               (v i >= v (j +! mk_usize 1 <: usize) && v i < v offset_vec + v step_vec) ||
               v i >= v (j +! mk_usize 1 <: usize) + v step_vec
             then
               (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                   e_initial_coefficient_bound
                   (cf.[ i ] <: v_Vector) /\
                Seq.index cf (v i) == Seq.index re_init (v i))
             else
               (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #v_Vector
                   (e_initial_coefficient_bound +! mk_usize 3328 <: usize)
                   (cf.[ i ] <: v_Vector) /\
                cross_vec_done_at_fwd #v_Vector re_init cf step_vec_n zs (v i)))
         else b2t true) =
    if i <. mk_usize 16
    then begin
      lemma_inner_inv_fwd_lookup #v_Vector re_init cb step_vec_n zs e_initial_coefficient_bound
        offset_vec step_vec j i;
      if v i = v j then ()
      else if v i = v j + step_vec_n then ()
      else begin
        Seq.lemma_index_upd2 (Seq.upd cb (v j) x) (v j + step_vec_n) y (v i);
        Seq.lemma_index_upd2 cb (v j) x (v i);
        lemma_cvda_fwd_frame1 #v_Vector re_init cb cf step_vec_n zs (v i)
      end
    end
  in
  Classical.forall_intro aux;
  reveal_opaque (`%inner_inv_fwd)
    (inner_inv_fwd #v_Vector re_init cf step_vec_n zs e_initial_coefficient_bound offset_vec step_vec
        (j +! mk_usize 1))
#pop-options

(* inner fold result -> outer invariant at round+1.  Mirror of lemma_inner_to_outer. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_inner_to_outer_fwd
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (e_initial_coefficient_bound: usize{v e_initial_coefficient_bound + 3328 < 65536})
      (round step offset_vec step_vec rnext jend: usize)
    : Lemma
      (requires
        v step == 16 * step_vec_n /\
        v step_vec == step_vec_n /\
        v offset_vec == 2 * v round * step_vec_n /\
        v rnext == v round + 1 /\
        v jend == v offset_vec + v step_vec /\
        inner_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound offset_vec
          step_vec jend)
      (ensures
        outer_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound rnext step) =
  lemma_offset_vec_fwd (v round + 1) (v step) ((v round + 1) * v step * 2)
    (((v round + 1) * v step * 2) / 16) step_vec_n;
  reveal_opaque (`%inner_inv_fwd)
    (inner_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound offset_vec
        step_vec jend);
  reveal_opaque (`%outer_inv_fwd)
    (outer_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound rnext step)
#pop-options

(* post-loop bridge — outer_inv at round=groups (ALL vectors DONE) -> full cross_vec_hyp_fwd forall. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 200 --split_queries always"
let lemma_postloop_cross_vec_fwd
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (re_init coeffs: t_Array v_Vector (mk_usize 16))
      (step_vec_n: pos)
      (zs: t_Slice Hacspec_ml_kem.Parameters.t_FieldElement)
      (e_initial_coefficient_bound: usize{v e_initial_coefficient_bound + 3328 < 65536})
      (groups step: usize)
    : Lemma
      (requires
        (v groups * v step * 2) / 16 == 16 /\
        outer_inv_fwd #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound groups step)
      (ensures
        (forall (m: nat) (l: nat).
           Hacspec_ml_kem.Commute.Ntt_bridge.cross_vec_hyp_fwd #v_Vector re_init coeffs step_vec_n zs m l)) =
  let aux (m: nat) (l: nat)
      : Lemma
        (Hacspec_ml_kem.Commute.Ntt_bridge.cross_vec_hyp_fwd #v_Vector re_init coeffs step_vec_n zs m l) =
    if m < 16 && l < 16
    then begin
      lemma_outer_inv_fwd_lookup #v_Vector re_init coeffs step_vec_n zs e_initial_coefficient_bound
        groups step (mk_usize m);
      lemma_cvda_fwd_reveal #v_Vector re_init coeffs step_vec_n zs m l
    end
    else
      reveal_opaque (`%Hacspec_ml_kem.Commute.Ntt_bridge.cross_vec_hyp_fwd)
        (Hacspec_ml_kem.Commute.Ntt_bridge.cross_vec_hyp_fwd #v_Vector re_init coeffs step_vec_n zs m l)
  in
  Classical.forall_intro_2 aux
#pop-options

(* per-round zeta slice (impl Montgomery zetas mapped to spec FEs); FORWARD = ASCENDING:
   zs[r] = mont(zeta(e_zeta_i_init + 1 + r)). *)
let zs_of_fwd (groups: usize)
    (e_zeta_i_init: usize{v e_zeta_i_init + v groups <= 127})
    : t_Slice Hacspec_ml_kem.Parameters.t_FieldElement =
  Seq.init (v groups)
    (fun (r: nat{r < v groups}) ->
        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
          (Libcrux_ml_kem.Polynomial.zeta (e_zeta_i_init +! mk_usize 1 +! mk_usize r <: usize) <: i16))
"#))]
#[hax_lib::fstar::options("--z3rlimit 300 --ext context_pruning --split_queries always")]
#[hax_lib::requires(
    spec::is_bounded_poly(_initial_coefficient_bound, re) & (
        _initial_coefficient_bound <= 5 * 3328 &&
        match layer {
            4 => *zeta_i == 7,
            5 => *zeta_i == 3,
            6 => *zeta_i == 1,
            7 => *zeta_i == 0,
            _ => false,
        })
)]
#[hax_lib::ensures(|result| spec::is_bounded_poly(_initial_coefficient_bound+3328, future(re)) & (
        match layer {
            4 => *future(zeta_i) == 15,
            5 => *future(zeta_i) == 7,
            6 => *future(zeta_i) == 3,
            7 => *future(zeta_i) == 1,
            _ => false,
        }) & fstar!(r#"
    Hacspec_ml_kem.Commute.Ntt_bridge.poly_step #$:Vector ${re} ${re}_future ${layer}
"#))]
#[inline(always)]
pub(crate) fn ntt_at_layer_4_plus<Vector: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<Vector>,
    layer: usize,
    _initial_coefficient_bound: usize, // This can be used for specifying the range of values allowed in re
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
    let step_vec_n = step / 16;

    // F-B: per-layer numeric facts + outer-fold invariant init.
    hax_lib::fstar!(r#"lemma_layer_numeric_facts_fwd ${layer} ${_zeta_i_init}"#);
    hax_lib::fstar!(
        r#"lemma_outer_inv_fwd_init #$:Vector ${_re_init} ${re}.f_coefficients
             (v ${step_vec_n}) (zs_of_fwd ${groups} ${_zeta_i_init})
             ${_initial_coefficient_bound} ${step}"#
    );

    for round in 0..groups {
        hax_lib::loop_invariant!(|round: usize| {
            (*zeta_i == _zeta_i_init + round).to_prop()
                & fstar!(
                    r#"outer_inv_fwd #$:Vector ${_re_init} ${re}.f_coefficients
                         (v ${step_vec_n}) (zs_of_fwd ${groups} ${_zeta_i_init})
                         ${_initial_coefficient_bound} ${round} ${step}"#
                )
        });

        *zeta_i += 1;

        let offset = round * step * 2;
        let offset_vec = offset / 16;
        let step_vec = step / 16;

        // inner-fold invariant init (outer_inv at round -> inner_inv at offset_vec).
        hax_lib::fstar!(
            r#"lemma_inner_inv_fwd_init #$:Vector ${_re_init} ${re}.f_coefficients
                 (v ${step_vec_n}) (zs_of_fwd ${groups} ${_zeta_i_init})
                 ${_initial_coefficient_bound} ${round} ${step} ${offset_vec} ${step_vec}"#
        );

        for j in offset_vec..offset_vec + step_vec {
            hax_lib::loop_invariant!(|j: usize| {
                fstar!(
                    r#"inner_inv_fwd #$:Vector ${_re_init} ${re}.f_coefficients
                         (v ${step_vec_n}) (zs_of_fwd ${groups} ${_zeta_i_init})
                         ${_initial_coefficient_bound} ${offset_vec} ${step_vec} ${j}"#
                )
            });

            #[cfg(hax)]
            let _re_body_in = *re;
            // expose the PENDING bounds on j and j+step_vec for the step precondition.
            hax_lib::fstar!(
                r#"lemma_inner_inv_fwd_lookup #$:Vector ${_re_init} ${re}.f_coefficients (v ${step_vec_n})
                     (zs_of_fwd ${groups} ${_zeta_i_init}) ${_initial_coefficient_bound} ${offset_vec} ${step_vec} ${j} ${j};
                   lemma_inner_inv_fwd_lookup #$:Vector ${_re_init} ${re}.f_coefficients (v ${step_vec_n})
                     (zs_of_fwd ${groups} ${_zeta_i_init}) ${_initial_coefficient_bound} ${offset_vec} ${step_vec} ${j}
                     (${j} +! ${step_vec})"#
            );

            let (x, y) = ntt_layer_int_vec_step(
                re.coefficients[j],
                re.coefficients[j + step_vec],
                zeta(*zeta_i),
                _initial_coefficient_bound,
            );
            re.coefficients[j] = x;
            re.coefficients[j + step_vec] = y;

            // inner maintenance: inner_inv at (cb,j) -> inner_inv at (cf,j+1).
            hax_lib::fstar!(
                r#"lemma_offset_vec_fwd (v ${round}) (v ${step}) (v ${offset}) (v ${offset_vec}) (v ${step_vec_n});
                   FStar.Seq.Base.init_index_ (v ${groups})
                     (fun (r: nat{r < v ${groups}}) ->
                        Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                          (Libcrux_ml_kem.Polynomial.zeta
                            (${_zeta_i_init} +! mk_usize 1 +! mk_usize r)))
                     (v ${round});
                   lemma_inner_step_maintains_fwd #$:Vector ${_re_init}
                     ${_re_body_in}.f_coefficients ${re}.f_coefficients (v ${step_vec_n})
                     (zs_of_fwd ${groups} ${_zeta_i_init}) ${_initial_coefficient_bound} ${offset_vec} ${step_vec} ${j}
                     (v ${round}) (Libcrux_ml_kem.Polynomial.zeta ${zeta_i}) ${x} ${y}"#
            );
        }

        // outer maintenance: inner_inv at offset_vec+step_vec -> outer_inv at round+1.
        hax_lib::fstar!(
            r#"lemma_offset_vec_fwd (v ${round}) (v ${step}) (v ${offset}) (v ${offset_vec}) (v ${step_vec_n});
               lemma_inner_to_outer_fwd #$:Vector ${_re_init} ${re}.f_coefficients (v ${step_vec_n})
                 (zs_of_fwd ${groups} ${_zeta_i_init}) ${_initial_coefficient_bound} ${round} ${step} ${offset_vec} ${step_vec}
                 (${round} +! mk_usize 1) (${offset_vec} +! ${step_vec})"#
        );
    }

    // zeta-table forall (ASCENDING): zs_of_fwd[round] == v_ZETAS[groups+round].
    hax_lib::fstar!(
        r#"(let aux (round: nat)
              : Lemma (round < v ${groups} ==>
                  Seq.index (zs_of_fwd ${groups} ${_zeta_i_init}) round ==
                  Hacspec_ml_kem.Ntt.v_ZETAS.[ sz (v ${groups} + round) ]) =
            if round < v ${groups} then begin
              FStar.Seq.Base.init_index_ (v ${groups})
                (fun (r: nat{r < v ${groups}}) ->
                   Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                     (Libcrux_ml_kem.Polynomial.zeta (${_zeta_i_init} +! mk_usize 1 +! mk_usize r)))
                round;
              Hacspec_ml_kem.Commute.Bridges.lemma_zeta_eq_vzetas
                (${_zeta_i_init} +! mk_usize 1 +! mk_usize round);
              assert (v (${_zeta_i_init} +! mk_usize 1 +! mk_usize round) == v ${groups} + round)
            end
          in Classical.forall_intro aux)"#
    );
    // (a) is_bounded_poly (bound+3328) (every vector DONE) + (b) the cross_vec_hyp_fwd
    // forall; then the bridge keystone discharges the PLAIN poly_step post.
    hax_lib::fstar!(
        r#"lemma_offset_vec_fwd (v ${groups}) (v ${step}) (v ${groups} * v ${step} * 2)
             ((v ${groups} * v ${step} * 2) / 16) (v ${step_vec_n});
           (let auxb (i: nat)
              : Lemma (i < 16 ==>
                  Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #$:Vector
                    (${_initial_coefficient_bound} +! mk_usize 3328) (${re}.f_coefficients.[ sz i ])) =
            if i < 16 then
              lemma_outer_inv_fwd_lookup #$:Vector ${_re_init} ${re}.f_coefficients (v ${step_vec_n})
                (zs_of_fwd ${groups} ${_zeta_i_init}) ${_initial_coefficient_bound} ${groups} ${step} (sz i)
          in Classical.forall_intro auxb);
           lemma_postloop_cross_vec_fwd #$:Vector ${_re_init} ${re}.f_coefficients (v ${step_vec_n})
             (zs_of_fwd ${groups} ${_zeta_i_init}) ${_initial_coefficient_bound} ${groups} ${step};
           FStar.Seq.Base.lemma_init_len (v ${groups})
             (fun (r: nat{r < v ${groups}}) ->
                Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                  (Libcrux_ml_kem.Polynomial.zeta (${_zeta_i_init} +! mk_usize 1 +! mk_usize r)));
           lemma_groups_len_256 (v ${groups}) (v ${step}) (v ${step} / 16);
           Hacspec_ml_kem.Commute.Ntt_bridge.lemma_layer_4_plus_to_poly_step #$:Vector
             ${_re0} ${re} ${layer} ${step} (v ${step_vec_n}) (zs_of_fwd ${groups} ${_zeta_i_init})"#
    );
}

// Layer-7 forward NTT functional post (`poly_step`).  The layer-7 fold is structurally
// `ntt_at_layer_4_plus(layer=7)`'s inner fold (groups=1, step_vec=8) but uses the plain
// `multiply_by_constant_bounded(b, -1600)` butterfly instead of the generic montgomery
// `ntt_layer_int_vec_step`.  `-1600 == zeta(1) mod q` (`(v(zeta 1)*169)%3329 == 1729 == -1600`,
// via `Ntt_bridge.lemma_zeta1_val`), so the plain multiply discharges the same Montgomery-form
// `ntt_step_post` the forward inner-fold scaffold consumes.
#[cfg_attr(
    hax,
    hax_lib::fstar::before(
        r#"
(* int core: (v(zeta 1)*169)%q == 1729 (== -1600 mod q) bridges plain mult by -1600 to
   the montgomery-mont butterfly precondition. *)
#push-options "--fuel 0 --ifuel 0 --z3rlimit 100"
let lemma_l7_mul_core (zz bb: int)
  : Lemma (requires (zz * 169) % 3329 == 1729)
          (ensures (bb * (-1600)) % 3329 == (zz * bb * 169) % 3329)
  = let q : pos = 3329 in
    FStar.Math.Lemmas.lemma_mod_mul_distr_l (zz * 169) bb q;
    assert (zz * bb * 169 == (zz * 169) * bb);
    assert ((((zz * 169) % q) * bb) % q == (1729 * bb) % q);
    FStar.Math.Lemmas.lemma_mod_plus ((-1600) * bb) bb q;
    assert ((-1600) * bb + bb * q == 1729 * bb);
    assert (bb * (-1600) == (-1600) * bb)
#pop-options

(* establish ntt_step_post for the layer-7 butterfly from the leaf-op posts:
   t = mult_by_const(b, -1600), x = add(a, t), y = sub(a, t).  Mirror of
   ntt_layer_int_vec_step's aux0/aux1, with the plain-multiply core for the t step. *)
#push-options "--fuel 0 --ifuel 1 --z3rlimit 300 --split_queries always"
let lemma_layer7_step_post
      (#v_Vector: Type0)
      (#[FStar.Tactics.Typeclasses.tcresolve ()]
          i0:
          Libcrux_ml_kem.Vector.Traits.t_Operations v_Vector)
      (a b t x y: v_Vector)
    : Lemma
      (requires
        Libcrux_ml_kem.Vector.Traits.Spec.multiply_by_constant_post
          (Libcrux_ml_kem.Vector.Traits.f_repr b) (mk_i16 (-1600))
          (Libcrux_ml_kem.Vector.Traits.f_repr t) /\
        Libcrux_ml_kem.Vector.Traits.Spec.add_post
          (Libcrux_ml_kem.Vector.Traits.f_repr a) (Libcrux_ml_kem.Vector.Traits.f_repr t)
          (Libcrux_ml_kem.Vector.Traits.f_repr x) /\
        Libcrux_ml_kem.Vector.Traits.Spec.sub_post
          (Libcrux_ml_kem.Vector.Traits.f_repr a) (Libcrux_ml_kem.Vector.Traits.f_repr t)
          (Libcrux_ml_kem.Vector.Traits.f_repr y))
      (ensures
        ntt_step_post #v_Vector a b x y (Libcrux_ml_kem.Polynomial.zeta (mk_usize 1)))
  = let zeta_r = Libcrux_ml_kem.Polynomial.zeta (mk_usize 1) in
    let a_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array a in
    let b_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array b in
    let t_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array t in
    let x_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array x in
    let y_arr = Libcrux_ml_kem.Vector.Traits.f_to_i16_array y in
    assert (a_arr == Libcrux_ml_kem.Vector.Traits.f_repr a);
    assert (b_arr == Libcrux_ml_kem.Vector.Traits.f_repr b);
    assert (t_arr == Libcrux_ml_kem.Vector.Traits.f_repr t);
    assert (x_arr == Libcrux_ml_kem.Vector.Traits.f_repr x);
    assert (y_arr == Libcrux_ml_kem.Vector.Traits.f_repr y);
    Hacspec_ml_kem.Commute.Ntt_bridge.lemma_zeta1_val ();
    assert ((v zeta_r * 169) % 3329 == 1729);
    let aux0 (i: nat)
        : Lemma
        (i < 16 ==>
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index x_arr i) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__add
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a_arr i))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
               (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta_r)
               (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index b_arr i)))) =
      if i < 16 then begin
        lemma_l7_mul_core (v zeta_r) (v (Seq.index b_arr i));
        Hacspec_ml_kem.Commute.Chunk.lemma_mont_mul_fe_commute_mont_mont zeta_r
          (Seq.index b_arr i) (Seq.index t_arr i);
        Hacspec_ml_kem.Commute.Chunk.lemma_add_fe_commute_mont
          (Seq.index a_arr i) (Seq.index t_arr i) (Seq.index x_arr i)
      end
    in
    Classical.forall_intro aux0;
    let aux1 (i: nat)
        : Lemma
        (i < 16 ==>
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index y_arr i) ==
          Hacspec_ml_kem.Parameters.impl_FieldElement__sub
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index a_arr i))
            (Hacspec_ml_kem.Parameters.impl_FieldElement__mul
               (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe zeta_r)
               (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe (Seq.index b_arr i)))) =
      if i < 16 then begin
        lemma_l7_mul_core (v zeta_r) (v (Seq.index b_arr i));
        Hacspec_ml_kem.Commute.Chunk.lemma_mont_mul_fe_commute_mont_mont zeta_r
          (Seq.index b_arr i) (Seq.index t_arr i);
        Hacspec_ml_kem.Commute.Chunk.lemma_sub_fe_commute_mont
          (Seq.index a_arr i) (Seq.index t_arr i) (Seq.index y_arr i)
      end
    in
    Classical.forall_intro aux1;
    reveal_opaque (`%ntt_step_post) (ntt_step_post #v_Vector a b x y zeta_r)
#pop-options
"#
    )
)]
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning")]
#[hax_lib::fstar::options("--z3rlimit 300 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(3, re))]
#[hax_lib::ensures(|result| spec::is_bounded_poly(4803, future(re)) & fstar!(r#"
    Hacspec_ml_kem.Commute.Ntt_bridge.poly_step #$:Vector ${re} ${re}_future (mk_usize 7)"#))]
pub(crate) fn ntt_at_layer_7<Vector: Operations>(re: &mut PolynomialRingElement<Vector>) {
    #[cfg(hax)]
    let _re0 = *re;
    #[cfg(hax)]
    let _re_init = re.coefficients;
    let step = VECTORS_IN_RING_ELEMENT / 2;

    // Layer 7 has a single butterfly group (groups=1, round=0, offset_vec=0, step_vec=8,
    // coefficient-step=128).  Weaken the input bound 3 -> 1475 (so the scaffold's DONE bound
    // `1475+3328 == 4803` matches the plain-multiply butterfly output) and init inner_inv_fwd.
    hax_lib::fstar!(
        r#"(let auxw (i: usize)
              : Lemma (i <. mk_usize 16 ==>
                  Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #$:Vector (mk_usize 1475)
                    (${re}.f_coefficients.[ i ])) =
            if i <. mk_usize 16 then
              Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector_higher #$:Vector
                (${re}.f_coefficients.[ i ]) (mk_usize 3) (mk_usize 1475)
          in Classical.forall_intro auxw);
          lemma_outer_inv_fwd_init #$:Vector ${_re_init} ${re}.f_coefficients
            8 (zs_of_fwd (mk_usize 1) (mk_usize 0)) (mk_usize 1475) (mk_usize 128);
          lemma_inner_inv_fwd_init #$:Vector ${_re_init} ${re}.f_coefficients
            8 (zs_of_fwd (mk_usize 1) (mk_usize 0)) (mk_usize 1475) (mk_usize 0) (mk_usize 128)
            (mk_usize 0) ${step}"#
    );

    for j in 0..step {
        hax_lib::loop_invariant!(|j: usize| {
            hax_lib::forall(|i: usize| {
                if i < 16 {
                    if (i >= j && i < step) || (i >= j + step) {
                        spec::is_bounded_vector(3, &re.coefficients[i])
                    } else {
                        spec::is_bounded_vector(4803, &re.coefficients[i])
                    }
                } else {
                    true.to_prop()
                }
            }) & fstar!(
                r#"inner_inv_fwd #$:Vector ${_re_init} ${re}.f_coefficients 8
                     (zs_of_fwd (mk_usize 1) (mk_usize 0)) (mk_usize 1475) (mk_usize 0) (mk_usize 8) ${j}"#
            )
        });

        // Help Z3 compute the bound from multiply_by_constant_bounded's ensures.
        // `abs_i16` is an abstract `val`, so assert_norm can't compute it; cite the
        // pre-existing library axiom `Spec.Utils.impl_i16__abs_value` instead.
        hax_lib::fstar!(r#"Spec.Utils.impl_i16__abs_value (mk_i16 (-1600))"#);
        #[cfg(hax)]
        let _re_body_in = *re;
        let t = multiply_by_constant_bounded(re.coefficients[j + step], 3, -1600);
        hax_lib::fstar!(
            r#"assert (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #$:Vector (mk_usize 4800) $t)"#
        );
        // Precompute the butterfly outputs from the (unmodified) input lane re[j], then write
        // re[j] then re[j+step] (matching ntt_at_layer_4_plus's write order = the maintains lemma).
        let y = sub_bounded(re.coefficients[j], 3, &t, 4800);
        let x = add_bounded(re.coefficients[j], 3, &t, 4800);
        re.coefficients[j] = x;
        re.coefficients[j + step] = y;
        hax_lib::fstar!(
            r#"FStar.Seq.Base.init_index_ (v (mk_usize 1))
                 (fun (r: nat{r < v (mk_usize 1)}) ->
                    Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 0 +! mk_usize 1 +! mk_usize r)))
                 (v (mk_usize 0));
               lemma_layer7_step_post #$:Vector
                 (${_re_body_in}.f_coefficients.[ ${j} ])
                 (${_re_body_in}.f_coefficients.[ ${j} +! ${step} ])
                 $t $x $y;
               lemma_inner_step_maintains_fwd #$:Vector ${_re_init}
                 ${_re_body_in}.f_coefficients ${re}.f_coefficients 8
                 (zs_of_fwd (mk_usize 1) (mk_usize 0)) (mk_usize 1475) (mk_usize 0) (mk_usize 8) ${j}
                 (v (mk_usize 0)) (Libcrux_ml_kem.Polynomial.zeta (mk_usize 1)) $x $y"#
        );
    }

    // groups=1: the single inner fold == the whole layer.  Lift to outer_inv at round=1
    // (all vectors DONE), bridge to the cross_vec_hyp_fwd forall + the ascending-zeta
    // correspondence, then discharge the PLAIN poly_step for layer 7.
    hax_lib::fstar!(
        r#"lemma_inner_to_outer_fwd #$:Vector ${_re_init} ${re}.f_coefficients 8
             (zs_of_fwd (mk_usize 1) (mk_usize 0)) (mk_usize 1475) (mk_usize 0) (mk_usize 128) (mk_usize 0)
             (mk_usize 8) (mk_usize 1) (mk_usize 0 +! mk_usize 8);
           lemma_postloop_cross_vec_fwd #$:Vector ${_re_init} ${re}.f_coefficients 8
             (zs_of_fwd (mk_usize 1) (mk_usize 0)) (mk_usize 1475) (mk_usize 1) (mk_usize 128);
           (let aux (round: nat)
              : Lemma (round < v (mk_usize 1) ==>
                  Seq.index (zs_of_fwd (mk_usize 1) (mk_usize 0)) round ==
                  Hacspec_ml_kem.Ntt.v_ZETAS.[ sz (v (mk_usize 1) + round) ]) =
            if round < v (mk_usize 1) then begin
              FStar.Seq.Base.init_index_ (v (mk_usize 1))
                (fun (r: nat{r < v (mk_usize 1)}) ->
                   Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_fe
                     (Libcrux_ml_kem.Polynomial.zeta (mk_usize 0 +! mk_usize 1 +! mk_usize r)))
                round;
              Hacspec_ml_kem.Commute.Bridges.lemma_zeta_eq_vzetas (mk_usize 0 +! mk_usize 1 +! mk_usize round);
              assert (v (mk_usize 0 +! mk_usize 1 +! mk_usize round) == v (mk_usize 1) + round)
            end in Classical.forall_intro aux);
           Hacspec_ml_kem.Commute.Ntt_bridge.lemma_layer_4_plus_to_poly_step #$:Vector
             ${_re0} ${re} (mk_usize 7) (mk_usize 128) 8 (zs_of_fwd (mk_usize 1) (mk_usize 0))"#
    );
}

/// Forward NTT of a CBD-sampled polynomial.
///
/// **Scaling**: input lane `v c ≡ α (mod q)` is in **plain** form (sample
/// output is plain — small CBD ints, no Montgomery scaling). NTT preserves
/// the input scaling: zetas are stored in Mont form (`v ζ_M ≡ ζ · R mod q`),
/// and each butterfly does `mont_mul(b, ζ_M) = b · ζ · R · R⁻¹ = b · ζ`
/// — the `· R` of zeta cancels with `mont_mul`'s built-in `· R⁻¹` factor.
/// So output is plain too.
///
/// Cross-spec runtime evidence: `ntt_matches_spec` test in this file
/// `assert_eq!`s `lift_poly(impl_after_ntt) == hacspec_ntt(plain_input)`,
/// confirming the impl preserves plain form through the full forward NTT.
// Forward NTT functional post (mirror of `ntt_vector_u`): the 7 per-layer `poly_step`
// atoms compose into the polynomial-level `Hacspec_ml_kem.Ntt.ntt` equality via
// `lemma_compose_7`.  Layer 7's `poly_step` comes from `ntt_at_layer_7`'s strengthened
// post; layers 6/5/4 from `ntt_at_layer_4_plus`; layers 3/2/1 from
// `lemma_layer{1,2,3}_to_poly_step`; barrett value-preservation via the free
// `poly_barrett_reduce`'s post + `lemma_poly_barrett_reduce_id`.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(3, re))]
#[hax_lib::ensures(|result| spec::is_bounded_poly(3328, future(re)) & fstar!(r#"
    Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector ${re}_future ==
    Hacspec_ml_kem.Ntt.ntt (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector $re)"#))]
pub(crate) fn ntt_binomially_sampled_ring_element<Vector: Operations>(
    re: &mut PolynomialRingElement<Vector>,
) {
    hax_debug_assert!(to_i16_array(re)
        .into_iter()
        .all(|coefficient| coefficient.abs() <= 3));

    #[cfg(hax)]
    let re0 = *re;
    // Pre-bind + anchor the barrett bound 28296 in CLEAN context (early ghost use) so its
    // usize range-check is range-checked here, not floated down into the polluted post-layer
    // quantifier context (where it saturates).  Mirrors ntt_vector_u.
    #[cfg(hax)]
    let bnd28296: usize = 28296;
    hax_lib::fstar!(r#"assert (v ${bnd28296} == 28296)"#);
    // Due to the small coefficient bound, we can skip the first round of
    // Montgomery reductions (plain-multiply layer-7 butterfly): poly_step re0 re1 7.
    ntt_at_layer_7(re);
    #[cfg(hax)]
    let re1 = *re;

    let mut zeta_i = 1;

    #[cfg(hax)]
    spec::is_bounded_poly_higher(re, 4803, 2 * 3328);

    ntt_at_layer_4_plus(&mut zeta_i, re, 6, 2 * 3328);
    #[cfg(hax)]
    let re2 = *re;
    ntt_at_layer_4_plus(&mut zeta_i, re, 5, 3 * 3328);
    #[cfg(hax)]
    let re3 = *re;
    ntt_at_layer_4_plus(&mut zeta_i, re, 4, 4 * 3328);
    #[cfg(hax)]
    let re4 = *re;
    ntt_at_layer_3(&mut zeta_i, re, 5 * 3328);
    #[cfg(hax)]
    let re5 = *re;
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Ntt_bridge.lemma_layer3_to_poly_step #$:Vector ${re4} ${re5}"#
    );
    ntt_at_layer_2(&mut zeta_i, re, 6 * 3328);
    #[cfg(hax)]
    let re6 = *re;
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Ntt_bridge.lemma_layer2_to_poly_step #$:Vector ${re5} ${re6}"#
    );
    ntt_at_layer_1(&mut zeta_i, re, 7 * 3328);
    #[cfg(hax)]
    let re7 = *re;
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Ntt_bridge.lemma_layer1_to_poly_step #$:Vector ${re6} ${re7}"#
    );

    // compose the 7 poly_step atoms into the N.ntt equality (driver post).
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Ntt_bridge.lemma_compose_7 #$:Vector ${re0} ${re1} ${re2} ${re3} ${re4} ${re5} ${re6} ${re7}"#
    );

    #[cfg(hax)]
    spec::is_bounded_poly_higher(re, 8 * 3328, bnd28296);

    // Method wrapper carries a bounds-only post; the bridge `lemma_impl__poly_barrett_reduce_spec`
    // exposes the same value-preservation as the free `poly_barrett_reduce`, then
    // `lemma_poly_barrett_reduce_id` (barrett is the plain identity) discharges the N.ntt equality.
    re.poly_barrett_reduce();
    hax_lib::fstar!(
        r#"Libcrux_ml_kem.Polynomial.lemma_impl__poly_barrett_reduce_spec #$:Vector ${re7};
           Hacspec_ml_kem.Commute.Chunk.lemma_poly_barrett_reduce_id
             (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector ${re7})"#
    );
}

/// Forward NTT of a decompressed ciphertext-u polynomial.  Same scaling
/// invariant as `ntt_binomially_sampled_ring_element`: input plain
/// (decompress output), output plain (NTT preserves form because Mont-form
/// zetas cancel with `mont_mul`'s `·R⁻¹`).
#[inline(always)]
// Forward NTT driver, functional post via the Ntt_bridge composition (mirror of
// invert_ntt_montgomery).  The 7 per-layer `poly_step` atoms compose into the
// polynomial-level `Hacspec_ml_kem.Ntt.ntt` equality via `lemma_compose_7`.
// F-B CLOSED (0 admits): layers 4-7 `poly_step` come from `ntt_at_layer_4_plus`'s
// strengthened post (forward cross-vector keystone scaffold in this module +
// `Ntt_bridge.lemma_layer_4_plus_to_poly_step`); layers 1-3 via
// `lemma_layer{1,2,3}_to_poly_step`; barrett value-preservation via the
// strengthened `impl__poly_barrett_reduce` post + `lemma_poly_barrett_reduce_id`;
// the 28296 bound via `is_bounded_poly_higher`.
#[hax_lib::fstar::options("--z3rlimit 400 --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(3328, re))]
#[hax_lib::ensures(|result| spec::is_bounded_poly(3328, future(re)) & fstar!(r#"
    Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector ${re}_future ==
    Hacspec_ml_kem.Ntt.ntt (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector $re)"#))]
pub(crate) fn ntt_vector_u<const VECTOR_U_COMPRESSION_FACTOR: usize, Vector: Operations>(
    re: &mut PolynomialRingElement<Vector>,
) {
    hax_debug_assert!(to_i16_array(re)
        .into_iter()
        .all(|coefficient| coefficient.abs() <= 3328));

    #[cfg(hax)]
    let re0 = *re;
    // Pre-bind + anchor the barrett bound 28296 in CLEAN context (early ghost
    // use) so its usize range-check is range-checked here, not floated down
    // into the polluted post-layer quantifier context (where it saturates).
    #[cfg(hax)]
    let bnd28296: usize = 28296;
    hax_lib::fstar!(r#"assert (v ${bnd28296} == 28296)"#);
    let mut zeta_i = 0;

    // layers 7,6,5,4: poly_step comes directly from ntt_at_layer_4_plus's
    // strengthened post (F-B), so no per-layer assume is needed.
    ntt_at_layer_4_plus(&mut zeta_i, re, 7, 3328);
    #[cfg(hax)]
    let re1 = *re;
    ntt_at_layer_4_plus(&mut zeta_i, re, 6, 2 * 3328);
    #[cfg(hax)]
    let re2 = *re;
    ntt_at_layer_4_plus(&mut zeta_i, re, 5, 3 * 3328);
    #[cfg(hax)]
    let re3 = *re;
    ntt_at_layer_4_plus(&mut zeta_i, re, 4, 4 * 3328);
    #[cfg(hax)]
    let re4 = *re;
    ntt_at_layer_3(&mut zeta_i, re, 5 * 3328);
    #[cfg(hax)]
    let re5 = *re;
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Ntt_bridge.lemma_layer3_to_poly_step #$:Vector ${re4} ${re5}"#
    );
    ntt_at_layer_2(&mut zeta_i, re, 6 * 3328);
    #[cfg(hax)]
    let re6 = *re;
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Ntt_bridge.lemma_layer2_to_poly_step #$:Vector ${re5} ${re6}"#
    );
    ntt_at_layer_1(&mut zeta_i, re, 7 * 3328);
    #[cfg(hax)]
    let re7 = *re;
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Ntt_bridge.lemma_layer1_to_poly_step #$:Vector ${re6} ${re7}"#
    );

    // compose the 7 poly_step atoms into the N.ntt equality (driver post).
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Ntt_bridge.lemma_compose_7 #$:Vector ${re0} ${re1} ${re2} ${re3} ${re4} ${re5} ${re6} ${re7}"#
    );

    // 28296 bound for the barrett precondition (re is bounded 8*3328 after layer 1).
    #[cfg(hax)]
    spec::is_bounded_poly_higher(re, 8 * 3328, bnd28296);

    // Call the free `poly_barrett_reduce` (whose post already carries the
    // value-preservation `to_spec_poly_plain result == HP.poly_barrett_reduce (...)`)
    // rather than the bounds-only `PolynomialRingElement::poly_barrett_reduce` method.
    poly_barrett_reduce(re);
    // barrett is the plain identity (`poly_barrett_reduce p == p`), so
    // `to_spec_poly_plain` survives the reduce — discharges the driver's N.ntt equality.
    hax_lib::fstar!(
        r#"Hacspec_ml_kem.Commute.Chunk.lemma_poly_barrett_reduce_id
             (Hacspec_ml_kem.Commute.Chunk.to_spec_poly_plain #$:Vector ${re7})"#
    );
}

#[cfg(test)]
mod cross_spec_tests {
    use crate::polynomial::cross_spec_tests::{lift_poly, lift_poly_montgomery, unlift_poly};
    use crate::vector::portable::PortableVector;
    use hacspec_ml_kem::parameters::{self as spec, FieldElement, Polynomial};

    /// Forward NTT: impl matches spec.
    ///
    /// Each NTT butterfly does montgomery_multiply(b, zeta*R) = b*zeta*R*R^{-1} = b*zeta,
    /// so the Montgomery factors cancel and the output is in plain form.
    #[test]
    fn ntt_matches_spec() {
        for seed in [0u16, 42, 255, 1000] {
            let spec_poly: Polynomial = spec::createi(|i| {
                FieldElement::new(
                    ((i as u16).wrapping_mul(seed).wrapping_add(7)) % spec::FIELD_MODULUS,
                )
            });

            let mut impl_poly = unlift_poly(&spec_poly);
            super::ntt_vector_u::<10, PortableVector>(&mut impl_poly);

            let spec_ntt = hacspec_ml_kem::ntt::vector_ntt([spec_poly])[0];

            assert_eq!(
                lift_poly(&impl_poly),
                spec_ntt,
                "NTT mismatch for seed={seed}"
            );
        }
    }

    /// NTT multiply: impl matches spec (accounting for Montgomery reduction).
    ///
    /// The impl's ntt_multiply does Montgomery multiplication internally,
    /// so the result has an R^{-1} factor relative to the spec.
    #[test]
    fn ntt_multiply_matches_spec() {
        let spec_a: Polynomial =
            spec::createi(|i| FieldElement::new((i as u16 * 7 + 3) % spec::FIELD_MODULUS));
        let spec_b: Polynomial =
            spec::createi(|i| FieldElement::new((i as u16 * 13 + 100) % spec::FIELD_MODULUS));

        let spec_a_ntt = hacspec_ml_kem::ntt::vector_ntt([spec_a])[0];
        let spec_b_ntt = hacspec_ml_kem::ntt::vector_ntt([spec_b])[0];
        let spec_product = hacspec_ml_kem::ntt::multiply_ntts(&spec_a_ntt, &spec_b_ntt);

        let mut impl_a = unlift_poly(&spec_a);
        let mut impl_b = unlift_poly(&spec_b);
        super::ntt_vector_u::<10, PortableVector>(&mut impl_a);
        super::ntt_vector_u::<10, PortableVector>(&mut impl_b);
        let impl_product = impl_a.ntt_multiply(&impl_b);

        // The impl ntt_multiply uses Montgomery reduction, so each pair
        // of the base-case multiply has a factor of R^{-1} relative to spec.
        // lift_poly_montgomery divides by R, so we need spec / R^2 for comparison.
        // Alternatively, multiply impl result by R to get spec result.
        const MONT_R: u32 = 2285; // 2^16 mod 3329
        let lifted: Polynomial = spec::createi(|i| {
            let c = lift_poly(&impl_product)[i];
            FieldElement::new((c.val as u32 * MONT_R % 3329) as u16)
        });

        assert_eq!(lifted, spec_product, "NTT multiply mismatch");
    }

    /// Full chain: NTT -> multiply -> inverse NTT -> Montgomery conversion
    /// should match spec's NTT -> multiply -> inverse NTT.
    ///
    /// Verifies all Montgomery factors cancel through the full pipeline.
    #[test]
    fn full_ntt_multiply_chain_matches_spec() {
        let spec_a: Polynomial =
            spec::createi(|i| FieldElement::new((i as u16 * 7 + 3) % spec::FIELD_MODULUS));
        let spec_b: Polynomial =
            spec::createi(|i| FieldElement::new((i as u16 * 13 + 100) % spec::FIELD_MODULUS));

        // Spec chain: NTT -> multiply -> inverse NTT
        let spec_a_ntt = hacspec_ml_kem::ntt::vector_ntt([spec_a])[0];
        let spec_b_ntt = hacspec_ml_kem::ntt::vector_ntt([spec_b])[0];
        let spec_product_ntt = hacspec_ml_kem::ntt::multiply_ntts(&spec_a_ntt, &spec_b_ntt);
        let spec_product = hacspec_ml_kem::invert_ntt::ntt_inverse(spec_product_ntt);

        // Impl chain: NTT -> multiply -> inverse NTT -> Montgomery-to-standard
        let mut impl_a = unlift_poly(&spec_a);
        let mut impl_b = unlift_poly(&spec_b);
        super::ntt_vector_u::<10, PortableVector>(&mut impl_a);
        super::ntt_vector_u::<10, PortableVector>(&mut impl_b);
        let mut impl_product = impl_a.ntt_multiply(&impl_b);

        crate::invert_ntt::invert_ntt_montgomery::<3, PortableVector>(&mut impl_product);

        // Montgomery-to-standard: multiply by 1441 and Barrett reduce.
        // 1441 combines R^{-1} and 128^{-1} (the missing inv-NTT scale factor).
        for i in 0..16 {
            impl_product.coefficients[i] =
                crate::vector::Operations::montgomery_multiply_by_constant(
                    impl_product.coefficients[i],
                    1441,
                );
            impl_product.coefficients[i] =
                crate::vector::Operations::barrett_reduce(impl_product.coefficients[i]);
        }

        assert_eq!(
            lift_poly(&impl_product),
            spec_product,
            "Full NTT multiply chain: Montgomery factors did not cancel"
        );
    }
}

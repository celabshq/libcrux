use crate::{
    hax_utils::hax_debug_assert,
    polynomial::{
        add_bounded, multiply_by_constant_bounded, sub_bounded, zeta, PolynomialRingElement,
        VECTORS_IN_RING_ELEMENT,
    },
    vector::Operations,
};

#[cfg(hax)]
use hax_lib::prop::ToProp;

#[cfg(hax)]
use crate::polynomial::spec;

#[cfg(hax)]
#[allow(unused_imports)]
use crate::vector::traits::spec::{zetas_1, zetas_2, zetas_4};

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(_initial_coefficient_bound, re) & (*zeta_i == 63 && _initial_coefficient_bound == 7 * 3328))]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(_initial_coefficient_bound+3328, future(re))
    & (*future(zeta_i) == 127)
    & fstar!(r#"
        forall (i: usize). i <. mk_usize 16 ==>
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_array
            (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
              (Seq.index ${re}_future.f_coefficients (v i))) ==
          Hacspec_ml_kem.Ntt.ntt_layer_n (mk_usize 16)
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_array
              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                (Seq.index ${re}.f_coefficients (v i))))
            (mk_usize 2)
            (Rust_primitives.unsize
              (${zetas_4}
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! i))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! i))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! i))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! i))))
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
                                & fstar!(r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Seq.index ${_re_init} (v $i)
                                  "#)
                        } else {
                            // Impl-level (Option B): record only the relationship
                            // re.coefficients[j] == f_ntt_layer_1_step _re_init[j] (parametric zetas).
                            // Function-form lift to N.ntt_layer_n is done once after the loop.
                            spec::is_bounded_vector(
                                _initial_coefficient_bound + 3328,
                                &re.coefficients[i],
                            )
                                & fstar!(r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Libcrux_ml_kem.Vector.Traits.f_ntt_layer_1_step #$:Vector
                                      (Seq.index ${_re_init} (v $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! $i))
                                  "#)
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
        hax_lib::fstar!(r#"
            assert (zeta_i == mk_usize 64 +! mk_usize 4 *! round);
            assert (zeta_i +! mk_usize 1 == mk_usize 65 +! mk_usize 4 *! round);
            assert (zeta_i +! mk_usize 2 == mk_usize 66 +! mk_usize 4 *! round);
            assert (zeta_i +! mk_usize 3 == mk_usize 67 +! mk_usize 4 *! round)
          "#);

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
    hax_lib::fstar!(r#"
        let aux (j: nat) : Lemma (j < 16 ==>
            Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_array
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index re.f_coefficients j)) ==
            Hacspec_ml_kem.Ntt.ntt_layer_n (mk_usize 16)
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_array
                (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                  (Seq.index ${_re_init} j)))
              (mk_usize 2)
              (Rust_primitives.unsize
                (${zetas_4}
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 64 +! mk_usize 4 *! sz j))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 65 +! mk_usize 4 *! sz j))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 66 +! mk_usize 4 *! sz j))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! sz j)))))
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
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 67 +! mk_usize 4 *! sz j))
            end
        in
        Classical.forall_intro aux
      "#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(_initial_coefficient_bound, re) & (*zeta_i == 31 && _initial_coefficient_bound == 6 * 3328))]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(_initial_coefficient_bound+3328, future(re))
    & (*future(zeta_i) == 63)
    & fstar!(r#"
        forall (i: usize). i <. mk_usize 16 ==>
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_array
            (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
              (Seq.index ${re}_future.f_coefficients (v i))) ==
          Hacspec_ml_kem.Ntt.ntt_layer_n (mk_usize 16)
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_array
              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                (Seq.index ${re}.f_coefficients (v i))))
            (mk_usize 4)
            (Rust_primitives.unsize
              (${zetas_2}
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! i))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! i))))
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
                                & fstar!(r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Seq.index ${_re_init} (v $i)
                                  "#)
                        } else {
                            // Impl-level (Option B): record only the relationship
                            // re.coefficients[j] == f_ntt_layer_2_step _re_init[j] (parametric zetas).
                            // Function-form lift to N.ntt_layer_n is done once after the loop.
                            spec::is_bounded_vector(
                                _initial_coefficient_bound + 3328,
                                &re.coefficients[i],
                            )
                                & fstar!(r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Libcrux_ml_kem.Vector.Traits.f_ntt_layer_2_step #$:Vector
                                      (Seq.index ${_re_init} (v $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! $i))
                                  "#)
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
        hax_lib::fstar!(r#"
            assert (zeta_i == mk_usize 32 +! mk_usize 2 *! round);
            assert (zeta_i +! mk_usize 1 == mk_usize 33 +! mk_usize 2 *! round)
          "#);

        re.coefficients[round] =
            Vector::ntt_layer_2_step(re.coefficients[round], zeta(*zeta_i), zeta(*zeta_i + 1));
        *zeta_i += 1;
    }
    // Phase 7b — Option B: lift the impl-level loop invariant to
    // function-form citation in the ensures via a post-loop forall_intro
    // over the bridge lemma `lemma_ntt_layer_2_step_to_hacspec`.
    hax_lib::fstar!(r#"
        let aux (j: nat) : Lemma (j < 16 ==>
            Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_array
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index re.f_coefficients j)) ==
            Hacspec_ml_kem.Ntt.ntt_layer_n (mk_usize 16)
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_array
                (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                  (Seq.index ${_re_init} j)))
              (mk_usize 4)
              (Rust_primitives.unsize
                (${zetas_2}
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! sz j))
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! sz j)))))
          = if j < 16 then begin
              reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (6 * 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                    (Seq.index ${_re_init} j)));
              Hacspec_ml_kem.Commute.Bridges.lemma_ntt_layer_2_step_to_hacspec
                #v_Vector
                (Seq.index ${_re_init} j)
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 32 +! mk_usize 2 *! sz j))
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 33 +! mk_usize 2 *! sz j))
            end
        in
        Classical.forall_intro aux
      "#);
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 800 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(_initial_coefficient_bound, re) & (*zeta_i == 15 && _initial_coefficient_bound == 5 * 3328))]
#[hax_lib::ensures(|result|
    spec::is_bounded_poly(_initial_coefficient_bound+3328, future(re))
    & (*future(zeta_i) == 31)
    & fstar!(r#"
        forall (i: usize). i <. mk_usize 16 ==>
          Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_array
            (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
              (Seq.index ${re}_future.f_coefficients (v i))) ==
          Hacspec_ml_kem.Ntt.ntt_layer_n (mk_usize 16)
            (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_array
              (Libcrux_ml_kem.Vector.Traits.f_repr #$:Vector
                (Seq.index ${re}.f_coefficients (v i))))
            (mk_usize 8)
            (Rust_primitives.unsize
              (${zetas_1}
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! i))))
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
                                & fstar!(r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Seq.index ${_re_init} (v $i)
                                  "#)
                        } else {
                            // Impl-level (Option B): record only the relationship
                            // re.coefficients[j] == f_ntt_layer_3_step _re_init[j] (single zeta).
                            // Function-form lift to N.ntt_layer_n is done once after the loop.
                            spec::is_bounded_vector(
                                _initial_coefficient_bound + 3328,
                                &re.coefficients[i],
                            )
                                & fstar!(r#"
                                    Seq.index ${re}.f_coefficients (v $i) ==
                                    Libcrux_ml_kem.Vector.Traits.f_ntt_layer_3_step #$:Vector
                                      (Seq.index ${_re_init} (v $i))
                                      (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! $i))
                                  "#)
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
        hax_lib::fstar!(r#"
            assert (zeta_i == mk_usize 16 +! round)
          "#);

        re.coefficients[round] = Vector::ntt_layer_3_step(re.coefficients[round], zeta(*zeta_i));
    }
    // Phase 7b — Option B: lift the impl-level loop invariant to
    // function-form citation in the ensures via a post-loop forall_intro
    // over the bridge lemma `lemma_ntt_layer_3_step_to_hacspec`.
    hax_lib::fstar!(r#"
        let aux (j: nat) : Lemma (j < 16 ==>
            Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_array
              (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                (Seq.index re.f_coefficients j)) ==
            Hacspec_ml_kem.Ntt.ntt_layer_n (mk_usize 16)
              (Libcrux_ml_kem.Vector.Traits.Spec.mont_i16_to_spec_array
                (Libcrux_ml_kem.Vector.Traits.f_repr #v_Vector
                  (Seq.index ${_re_init} j)))
              (mk_usize 8)
              (Rust_primitives.unsize
                (${zetas_1}
                  (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! sz j)))))
          = if j < 16 then begin
              reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque)
                (Libcrux_ml_kem.Vector.Traits.Spec.is_i16b_array_opaque (5 * 3328)
                  (Libcrux_ml_kem.Vector.Traits.f_to_i16_array
                    (Seq.index ${_re_init} j)));
              Hacspec_ml_kem.Commute.Bridges.lemma_ntt_layer_3_step_to_hacspec
                #v_Vector
                (Seq.index ${_re_init} j)
                (Libcrux_ml_kem.Polynomial.zeta (mk_usize 16 +! sz j))
            end
        in
        Classical.forall_intro aux
      "#);
}

#[inline(always)]
#[hax_lib::requires(spec::is_bounded_vector(_initial_coefficient_bound, &a) & (zeta_r >= -1664 && zeta_r <= 1664 && _initial_coefficient_bound <= 5 * 3328))]
#[hax_lib::ensures(|(r0, r1)| spec::is_bounded_vector(_initial_coefficient_bound+3328, &r0) & (spec::is_bounded_vector(_initial_coefficient_bound+3328, &r1)))]
fn ntt_layer_int_vec_step<Vector: Operations>(
    mut a: Vector,
    mut b: Vector,
    zeta_r: i16,
    _initial_coefficient_bound: usize, // This can be used for specifying the range of values allowed in re
) -> (Vector, Vector) {
    let t = Vector::montgomery_multiply_by_constant(b, zeta_r);
    b = sub_bounded(a, _initial_coefficient_bound, &t, 3328);
    a = add_bounded(a, _initial_coefficient_bound, &t, 3328);
    (a, b)
}

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
        })
)]
#[inline(always)]
pub(crate) fn ntt_at_layer_4_plus<Vector: Operations>(
    zeta_i: &mut usize,
    re: &mut PolynomialRingElement<Vector>,
    layer: usize,
    _initial_coefficient_bound: usize, // This can be used for specifying the range of values allowed in re
) {
    let step = 1 << layer;

    #[cfg(hax)]
    let _zeta_i_init = *zeta_i;

    for round in 0..(128 >> layer) {
        hax_lib::loop_invariant!(|round: usize| {
            (*zeta_i == _zeta_i_init + round).to_prop()
                & (hax_lib::forall(|i: usize| {
                    if i < 16 {
                        if i >= (round * step * 2) / 16 {
                            spec::is_bounded_vector(_initial_coefficient_bound, &re.coefficients[i])
                        } else {
                            spec::is_bounded_vector(
                                _initial_coefficient_bound + 3328,
                                &re.coefficients[i],
                            )
                        }
                    } else {
                        true.to_prop()
                    }
                }))
        });
        *zeta_i += 1;

        let offset = round * step * 2;
        let offset_vec = offset / 16; //FIELD_ELEMENTS_IN_VECTOR;
        let step_vec = step / 16; //FIELD_ELEMENTS_IN_VECTOR;

        for j in offset_vec..offset_vec + step_vec {
            hax_lib::loop_invariant!(|j: usize| {
                hax_lib::forall(|i: usize| {
                    if i < 16 {
                        if (i >= j && i < offset_vec + step_vec) || (i >= j + step_vec) {
                            spec::is_bounded_vector(_initial_coefficient_bound, &re.coefficients[i])
                        } else {
                            spec::is_bounded_vector(
                                _initial_coefficient_bound + 3328,
                                &re.coefficients[i],
                            )
                        }
                    } else {
                        true.to_prop()
                    }
                })
            });
            let (x, y) = ntt_layer_int_vec_step(
                re.coefficients[j],
                re.coefficients[j + step_vec],
                zeta(*zeta_i),
                _initial_coefficient_bound,
            );
            re.coefficients[j] = x;
            re.coefficients[j + step_vec] = y;
        }
    }
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning")]
#[hax_lib::fstar::options("--z3rlimit 300 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(3, re))]
#[hax_lib::ensures(|result| spec::is_bounded_poly(4803, future(re)))]
pub(crate) fn ntt_at_layer_7<Vector: Operations>(re: &mut PolynomialRingElement<Vector>) {
    let step = VECTORS_IN_RING_ELEMENT / 2;
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
            })
        });

        // Help Z3 compute the bound from multiply_by_constant_bounded's ensures
        hax_lib::fstar!(r#"assume (v (Core_models.Num.impl_i16__abs (mk_i16 (-1600))) == 1600)"#);
        let t = multiply_by_constant_bounded(re.coefficients[j + step], 3, -1600);
        hax_lib::fstar!(
            r#"assert (Libcrux_ml_kem.Polynomial.Spec.is_bounded_vector #$:Vector (mk_usize 4800) $t)"#
        );
        re.coefficients[j + step] = sub_bounded(re.coefficients[j], 3, &t, 4800);
        re.coefficients[j] = add_bounded(re.coefficients[j], 3, &t, 4800);
    }
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
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 200")]
#[hax_lib::requires(spec::is_bounded_poly(3, re))]
#[hax_lib::ensures(|result| spec::is_bounded_poly(3328, future(re)))]
// #[hax_lib::ensures(|_| fstar!(r#"
//     Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector ${re}_future ==
//     Spec.MLKEM.poly_ntt (Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector $re) /\
//     Libcrux_ml_kem.Polynomial.is_bounded_poly #$:Vector 3328 ${re}_future"#)]
pub(crate) fn ntt_binomially_sampled_ring_element<Vector: Operations>(
    re: &mut PolynomialRingElement<Vector>,
) {
    hax_debug_assert!(to_i16_array(re)
        .into_iter()
        .all(|coefficient| coefficient.abs() <= 3));

    // Due to the small coefficient bound, we can skip the first round of
    // Montgomery reductions.
    ntt_at_layer_7(re);

    let mut zeta_i = 1;

    #[cfg(hax)]
    spec::is_bounded_poly_higher(re, 4803, 2 * 3328);

    ntt_at_layer_4_plus(&mut zeta_i, re, 6, 2 * 3328);
    ntt_at_layer_4_plus(&mut zeta_i, re, 5, 3 * 3328);
    ntt_at_layer_4_plus(&mut zeta_i, re, 4, 4 * 3328);
    ntt_at_layer_3(&mut zeta_i, re, 5 * 3328);
    ntt_at_layer_2(&mut zeta_i, re, 6 * 3328);
    ntt_at_layer_1(&mut zeta_i, re, 7 * 3328);

    #[cfg(hax)]
    spec::is_bounded_poly_higher(re, 8 * 3328, 28296);

    re.poly_barrett_reduce()
}

/// Forward NTT of a decompressed ciphertext-u polynomial.  Same scaling
/// invariant as `ntt_binomially_sampled_ring_element`: input plain
/// (decompress output), output plain (NTT preserves form because Mont-form
/// zetas cancel with `mont_mul`'s `·R⁻¹`).
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300 --ext context_pruning --split_queries always")]
#[hax_lib::requires(spec::is_bounded_poly(3328, re))]
#[hax_lib::ensures(|result| spec::is_bounded_poly(3328, future(re)))]
// #[hax_lib::ensures(|_| fstar!(r#"Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector ${re}_future ==
//     Spec.MLKEM.poly_ntt (Libcrux_ml_kem.Polynomial.to_spec_poly_t #$:Vector $re)"#))]
pub(crate) fn ntt_vector_u<const VECTOR_U_COMPRESSION_FACTOR: usize, Vector: Operations>(
    re: &mut PolynomialRingElement<Vector>,
) {
    hax_debug_assert!(to_i16_array(re)
        .into_iter()
        .all(|coefficient| coefficient.abs() <= 3328));

    let mut zeta_i = 0;

    ntt_at_layer_4_plus(&mut zeta_i, re, 7, 3328);
    ntt_at_layer_4_plus(&mut zeta_i, re, 6, 2 * 3328);
    ntt_at_layer_4_plus(&mut zeta_i, re, 5, 3 * 3328);
    ntt_at_layer_4_plus(&mut zeta_i, re, 4, 4 * 3328);
    ntt_at_layer_3(&mut zeta_i, re, 5 * 3328);
    ntt_at_layer_2(&mut zeta_i, re, 6 * 3328);
    ntt_at_layer_1(&mut zeta_i, re, 7 * 3328);

    #[cfg(hax)]
    spec::is_bounded_poly_higher(re, 8 * 3328, 28296);

    re.poly_barrett_reduce()
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

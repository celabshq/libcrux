use crate::{
    hash_functions::Hash, invert_ntt::invert_ntt_montgomery, polynomial::PolynomialRingElement,
    sampling::sample_from_xof, vector::Operations,
};

#[cfg(hax)]
use hax_lib::prop::ToProp;

#[cfg(hax)]
use crate::polynomial::spec;

// TODO: The F* proof of sample_matrix_A is flaky (Z3 uses ~393/400 rlimit).
// The loop invariant for the inner j-loop should be simplified or the proof
// restructured so it verifies reliably. Currently in SLOW_MODULES (admitted by default).
#[inline(always)]
#[allow(non_snake_case)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options("--z3rlimit 400 --ext context_pruning --split_queries always")]
#[hax_lib::requires(K <= 4)]
#[hax_lib::ensures(|res| hax_lib::forall(|i:usize| hax_lib::implies(i < K,
                            hax_lib::forall(|j:usize| hax_lib::implies(j < K,
                                spec::is_bounded_poly(3328, &(future(A_transpose)[i][j]))))))
    & (match hacspec_ml_kem::matrix::sample_matrix_A::<K>(&seed[..32], transpose) {
        Ok(sampled_matrix) =>
            crate::vector::spec::matrix_to_spec(&future(A_transpose)) == sampled_matrix,
        Err(_) => true,
    }).to_prop())]
pub(crate) fn sample_matrix_A<const K: usize, Vector: Operations, Hasher: Hash<K>>(
    A_transpose: &mut [[PolynomialRingElement<Vector>; K]; K],
    seed: &[u8; 34],
    transpose: bool,
) {
    for i in 0..K {
        hax_lib::loop_invariant!(|i: usize| if transpose {
            hax_lib::forall(|k: usize| {
                hax_lib::implies(
                    k < K,
                    hax_lib::forall(|l: usize| {
                        hax_lib::implies(
                            l < K,
                            if k < i {
                                spec::is_bounded_poly(3328, &A_transpose[l][k])
                            } else {
                                true.to_prop()
                            },
                        )
                    }),
                )
            })
        } else {
            hax_lib::forall(|k: usize| {
                hax_lib::implies(
                    k < K,
                    hax_lib::forall(|l: usize| {
                        hax_lib::implies(
                            l < K,
                            if k < i {
                                spec::is_bounded_poly(3328, &A_transpose[k][l])
                            } else {
                                true.to_prop()
                            },
                        )
                    }),
                )
            })
        });

        let mut seeds = [seed.clone(); K];
        for j in 0..K {
            seeds[j][32] = i as u8;
            seeds[j][33] = j as u8;
        }
        let sampled = sample_from_xof::<K, Vector, Hasher>(&seeds);
        for j in 0..K {
            hax_lib::loop_invariant!(|j: usize| if transpose {
                hax_lib::forall(|k: usize| {
                    hax_lib::implies(
                        k < K,
                        hax_lib::forall(|l: usize| {
                            hax_lib::implies(
                                l < K,
                                if k < i || (k == i && l < j) {
                                    spec::is_bounded_poly(3328, &A_transpose[l][k])
                                } else {
                                    true.to_prop()
                                },
                            )
                        }),
                    )
                })
            } else {
                hax_lib::forall(|k: usize| {
                    hax_lib::implies(
                        k < K,
                        hax_lib::forall(|l: usize| {
                            hax_lib::implies(
                                l < K,
                                if k < i || (k == i && l < j) {
                                    spec::is_bounded_poly(3328, &A_transpose[k][l])
                                } else {
                                    true.to_prop()
                                },
                            )
                        }),
                    )
                })
            });
            // A[i][j] = A_transpose[j][i]
            if transpose {
                A_transpose[j][i] = sampled[j];
            } else {
                A_transpose[i][j] = sampled[j];
            }
        }
    }
}

/// The following functions compute various expressions involving
/// vectors and matrices. The computation of these expressions has been
/// abstracted away into these functions in order to save on loop iterations.

/// Compute v − InverseNTT(sᵀ ◦ NTT(u))
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --fuel 1 --ifuel 1 --ext context_pruning --using_facts_from '* -Hacspec_ml_kem.Parameters.createi_lemma -Libcrux_ml_kem.Polynomial.Spec'")]
#[hax_lib::requires((K <= 4).to_prop() & (
        spec::is_bounded_poly(4095, v) & (
            hax_lib::forall(|i:usize| hax_lib::implies(i < K,
                spec::is_bounded_poly(3328, &secret_as_ntt[i]) & (
                    spec::is_bounded_poly(3328, &u_as_ntt[i])
))))))]
#[hax_lib::ensures(|result| spec::is_bounded_poly(3328, &result)
    & (crate::vector::spec::poly_to_spec(&result)
        == hacspec_ml_kem::matrix::compute_message::<K>(
            &crate::vector::spec::poly_to_spec(&v),
            &crate::vector::spec::vector_to_spec(&secret_as_ntt),
            &crate::vector::spec::vector_to_spec(&u_as_ntt))).to_prop())]
pub(crate) fn compute_message<const K: usize, Vector: Operations>(
    v: &PolynomialRingElement<Vector>,
    secret_as_ntt: &[PolynomialRingElement<Vector>; K],
    u_as_ntt: &[PolynomialRingElement<Vector>; K],
) -> PolynomialRingElement<Vector> {
    // Ghost spec target (functional postcondition value).  Kept behind the opaque `vdot_done`
    // atom in the loop invariant (see Compute_dot_bridge) so the createi-heavy spec terms never
    // enter the loop-body VCs.
    #[cfg(hax)]
    let target = hacspec_ml_kem::matrix::compute_message::<K>(
        &crate::vector::spec::poly_to_spec(&v),
        &crate::vector::spec::vector_to_spec(&secret_as_ntt),
        &crate::vector::spec::vector_to_spec(&u_as_ntt),
    );

    let mut result = PolynomialRingElement::<Vector>::ZERO();

    hax_lib::assert_prop!(spec::is_bounded_poly(0, &result));
    hax_lib::fstar!(r#"Hacspec_ml_kem.Commute.Compute_dot_bridge.lemma_vdot_base
        ${secret_as_ntt} ${u_as_ntt} ${result}"#);

    for i in 0..K {
        hax_lib::loop_invariant!(|i: usize| spec::is_bounded_poly(i * 3328, &result)
            & fstar!(r#"Hacspec_ml_kem.Commute.Compute_dot_bridge.vdot_done
                ${result} ${secret_as_ntt} ${u_as_ntt} (Rust_primitives.Integers.v $i)"#));

        hax_lib::fstar!(r#"assert (Rust_primitives.Integers.v $i < Rust_primitives.Integers.v v_K);
            assert (Rust_primitives.Integers.v v_K <= 4)"#);
        #[cfg(hax)]
        let acc_old = result;
        let product = secret_as_ntt[i].ntt_multiply(&u_as_ntt[i]);
        result.add_to_ring_element(&product, i * 3328);
        hax_lib::fstar!(r#"Hacspec_ml_kem.Commute.Compute_dot_bridge.lemma_vdot_step_full
            ${secret_as_ntt} ${u_as_ntt} $i ${acc_old} ${product} ${result} ($i *! mk_usize 3328)"#);
    }

    #[cfg(hax)]
    let t_pre = result;
    invert_ntt_montgomery::<K, Vector>(&mut result);
    #[cfg(hax)]
    let re_future = result;
    result = v.subtract_reduce(result);
    hax_lib::fstar!(r#"Hacspec_ml_kem.Commute.Compute_dot_bridge.lemma_message_done_finalize
        ${v} ${secret_as_ntt} ${u_as_ntt} ${t_pre} ${re_future} ${result}"#);

    result
}

/// Compute InverseNTT(tᵀ ◦ r̂) + e₂ + message
#[inline(always)]
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning")]
#[hax_lib::requires((K <= 4).to_prop() & (
        spec::is_bounded_poly(3328, message) & (
            spec::is_bounded_poly(3328, error_2) & (
                hax_lib::forall(|i:usize| hax_lib::implies(i < K,
                    spec::is_bounded_poly(3328, &t_as_ntt[i]) & (
                        spec::is_bounded_poly(3328, &r_as_ntt[i])
)))))))]
#[hax_lib::ensures(|result| spec::is_bounded_poly(3328, &result)
    & (crate::vector::spec::poly_to_spec(&result)
        == hacspec_ml_kem::matrix::compute_ring_element_v::<K>(
            &crate::vector::spec::vector_to_spec(&t_as_ntt),
            &crate::vector::spec::vector_to_spec(&r_as_ntt),
            &crate::vector::spec::poly_to_spec(&error_2),
            &crate::vector::spec::poly_to_spec(&message))).to_prop())]
pub(crate) fn compute_ring_element_v<const K: usize, Vector: Operations>(
    t_as_ntt: &[PolynomialRingElement<Vector>; K],
    r_as_ntt: &[PolynomialRingElement<Vector>; K],
    error_2: &PolynomialRingElement<Vector>,
    message: &PolynomialRingElement<Vector>,
) -> PolynomialRingElement<Vector> {
    let mut result = PolynomialRingElement::<Vector>::ZERO();

    for i in 0..K {
        hax_lib::loop_invariant!(|i: usize| spec::is_bounded_poly(i * 3328, &result));

        let product = t_as_ntt[i].ntt_multiply(&r_as_ntt[i]);
        result.add_to_ring_element(&product, i * 3328);
    }

    invert_ntt_montgomery::<K, Vector>(&mut result);
    result = error_2.add_message_error_reduce(message, result);

    result
}

/// Compute u := InvertNTT(Aᵀ ◦ r̂) + e₁
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 400 --fuel 1 --ifuel 1 --ext context_pruning --using_facts_from '* -Hacspec_ml_kem.Parameters.createi_lemma -Libcrux_ml_kem.Polynomial.Spec'")]
#[hax_lib::requires((K <= 4).to_prop() & (
        hax_lib::forall(|i:usize| hax_lib::implies(i < K,
            spec::is_bounded_poly(7, &error_1[i]) & (
                spec::is_bounded_poly(3328, &r_as_ntt[i]) & (
                    hax_lib::forall(|j:usize| hax_lib::implies(j < K,
                        spec::is_bounded_poly(3328, &a_as_ntt[i][j])))))))))]
#[hax_lib::ensures(|result| hax_lib::forall(|i:usize| hax_lib::implies(i < K,
                                spec::is_bounded_poly(3328, &result[i])))
    & (crate::vector::spec::vector_to_spec(&result)
        == hacspec_ml_kem::matrix::compute_vector_u::<K>(
            &crate::vector::spec::matrix_to_spec(&a_as_ntt),
            &crate::vector::spec::vector_to_spec(&r_as_ntt),
            &crate::vector::spec::vector_to_spec(&error_1))).to_prop())]
pub(crate) fn compute_vector_u<const K: usize, Vector: Operations>(
    a_as_ntt: &[[PolynomialRingElement<Vector>; K]; K],
    r_as_ntt: &[PolynomialRingElement<Vector>; K],
    error_1: &[PolynomialRingElement<Vector>; K],
) -> [PolynomialRingElement<Vector>; K] {
    // Ghost spec target (functional postcondition value).  Kept behind opaque `row_done` atoms in
    // the loop invariants (see Matrix_bridge / Compute_u_bridge) so the createi-heavy spec terms
    // never enter the loop-body VCs.
    #[cfg(hax)]
    let target = hacspec_ml_kem::matrix::compute_vector_u::<K>(
        &crate::vector::spec::matrix_to_spec(&a_as_ntt),
        &crate::vector::spec::vector_to_spec(&r_as_ntt),
        &crate::vector::spec::vector_to_spec(&error_1),
    );

    let mut result = core::array::from_fn(|_i| PolynomialRingElement::<Vector>::ZERO());

    for i in 0..K {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| {
            if j < K {
                if j < i {
                    spec::is_bounded_poly(3328, &result[j])
                } else {
                    spec::is_bounded_poly(0, &result[j])
                }
            } else {
                true.to_prop()
            }
        }) & fstar!(r#"(forall (j: nat). j < v $i /\ j < v v_K ==>
            Hacspec_ml_kem.Commute.Matrix_bridge.row_done (Seq.index ${result} j) ${target} j)"#));

        #[cfg(hax)]
        let _result = result;

        hax_lib::fstar!(r#"Hacspec_ml_kem.Commute.Matrix_bridge.lemma_inner_done_base
            ${a_as_ntt} ${r_as_ntt} $i (${result}.[ $i ])"#);

        for j in 0..K {
            hax_lib::loop_invariant!(|j: usize| spec::is_bounded_poly(j * 3328, &result[i])
                & (hax_lib::forall(|k: usize| {
                    hax_lib::implies(k < K && k != i, hax_lib::eq(&result[k], &_result[k]))
                }))
                & (hax_lib::forall(|k: usize| hax_lib::implies(
                    k < i,
                    spec::is_bounded_poly(3328, &result[k])
                )))
                & fstar!(r#"(forall (k: nat). k < v $i /\ k < v v_K ==>
                    Hacspec_ml_kem.Commute.Matrix_bridge.row_done (Seq.index ${result} k) ${target} k)"#)
                & fstar!(r#"Hacspec_ml_kem.Commute.Matrix_bridge.inner_done
                    (${result}.[ $i ]) ${a_as_ntt} ${r_as_ntt} $i (v $j)"#));

            hax_lib::fstar!(r#"assert (v $j < v v_K); assert (v v_K <= 4)"#);
            #[cfg(hax)]
            let tt_old = result;
            let product = a_as_ntt[i][j].ntt_multiply(&r_as_ntt[j]);
            result[i].add_to_ring_element(&product, j * 3328);
            hax_lib::fstar!(r#"Hacspec_ml_kem.Commute.Matrix_bridge.lemma_inner_step_full
                ${a_as_ntt} ${r_as_ntt} $i $j ${tt_old} ${target}"#);
        }

        #[cfg(hax)]
        let t_pre = result[i];
        invert_ntt_montgomery::<K, Vector>(&mut result[i]);
        #[cfg(hax)]
        let re_future = result[i];
        result[i].add_error_reduce(&error_1[i]);
        hax_lib::fstar!(r#"Hacspec_ml_kem.Commute.Compute_u_bridge.lemma_u_row_done_finalize
            ${a_as_ntt} ${r_as_ntt} ${error_1} ${t_pre} ${re_future} (${result}.[ $i ]) $i ${target}"#);
        // Frame for the outer-invariant maintenance: invert_ntt + add_error_reduce only touch
        // result[i], so all other rows equal the iteration-start snapshot; this lets the bound
        // (for the from_fn-ZERO future rows) and row_done carry without a full re-derivation.
        hax_lib::fstar!(r#"assert (forall (k: nat). k < v v_K /\ k <> v $i ==>
            Seq.index ${result} k == Seq.index ${_result} k)"#);
    }
    hax_lib::fstar!(r#"Hacspec_ml_kem.Commute.Matrix_bridge.lemma_rows_assemble ${result} ${target}"#);
    result
}

/// Compute Â ◦ ŝ + ê
#[inline(always)]
#[allow(non_snake_case)]
#[hax_lib::fstar::options("--z3rlimit 400 --fuel 1 --ifuel 1 --ext context_pruning --using_facts_from '* -Hacspec_ml_kem.Parameters.createi_lemma -Libcrux_ml_kem.Polynomial.Spec'")]
#[hax_lib::requires((K <= 4).to_prop() & (
        hax_lib::forall(|i:usize| hax_lib::implies(i < K,
            spec::is_bounded_poly(3328, &error_as_ntt[i]) & (
                spec::is_bounded_poly(3328, &s_as_ntt[i]) & (
                    hax_lib::forall(|j:usize| hax_lib::implies(j < K,
                        spec::is_bounded_poly(3328, &matrix_A[i][j])))))))))]
#[hax_lib::ensures(|result| hax_lib::forall(|i:usize| hax_lib::implies(i < K,
                                spec::is_bounded_poly(3328, &future(t_as_ntt)[i])))
    & (crate::vector::spec::vector_to_spec(&future(t_as_ntt))
        == hacspec_ml_kem::matrix::compute_As_plus_e::<K>(
            // NOTE (2026-06-07): the impl computes t[i] = Σⱼ matrix_A[i][j]·s[j]
            // (row i of matrix_A), whereas the hacspec `multiply_matrix_by_column`
            // indexes `matrix[j][i]`. `matrix_to_spec` is index-preserving, so the
            // spec must receive the TRANSPOSE for the post to be true. This matches
            // keygen, which samples A with transpose=true and whose own post
            // (ind_cpa.rs generate_keypair_unpacked) relates via `transpose(&A_as_ntt)`.
            &hacspec_ml_kem::matrix::transpose(&crate::vector::spec::matrix_to_spec(&matrix_A)),
            &crate::vector::spec::vector_to_spec(&s_as_ntt),
            &crate::vector::spec::vector_to_spec(&error_as_ntt))).to_prop())]
pub(crate) fn compute_As_plus_e<const K: usize, Vector: Operations>(
    t_as_ntt: &mut [PolynomialRingElement<Vector>; K],
    matrix_A: &[[PolynomialRingElement<Vector>; K]; K],
    s_as_ntt: &[PolynomialRingElement<Vector>; K],
    error_as_ntt: &[PolynomialRingElement<Vector>; K],
) {
    // Ghost spec value of the result (the function's functional postcondition target).  Kept
    // behind opaque `row_done`/`inner_done` atoms in the loop invariants (see Matrix_bridge) so
    // the createi-heavy spec terms never enter the loop-body VCs.
    #[cfg(hax)]
    let target = hacspec_ml_kem::matrix::compute_As_plus_e::<K>(
        &hacspec_ml_kem::matrix::transpose(&crate::vector::spec::matrix_to_spec(&matrix_A)),
        &crate::vector::spec::vector_to_spec(&s_as_ntt),
        &crate::vector::spec::vector_to_spec(&error_as_ntt),
    );

    for i in 0..K {
        hax_lib::loop_invariant!(|i: usize| hax_lib::forall(|j: usize| hax_lib::implies(
            j < i,
            spec::is_bounded_poly(3328, &t_as_ntt[j])
        )) & fstar!(r#"(forall (j: nat). j < v $i /\ j < v v_K ==>
            Hacspec_ml_kem.Commute.Matrix_bridge.row_done (Seq.index ${t_as_ntt} j) ${target} j)"#));

        t_as_ntt[i] = PolynomialRingElement::<Vector>::ZERO();
        hax_lib::fstar!(r#"Hacspec_ml_kem.Commute.Matrix_bridge.lemma_inner_done_base
            ${matrix_A} ${s_as_ntt} $i (${t_as_ntt}.[ $i ])"#);

        for j in 0..K {
            hax_lib::loop_invariant!(|j: usize| spec::is_bounded_poly(j * 3328, &t_as_ntt[i])
                & (hax_lib::forall(|k: usize| hax_lib::implies(
                    k < i,
                    spec::is_bounded_poly(3328, &t_as_ntt[k])
                )))
                & fstar!(r#"(forall (k: nat). k < v $i /\ k < v v_K ==>
                    Hacspec_ml_kem.Commute.Matrix_bridge.row_done (Seq.index ${t_as_ntt} k) ${target} k)"#)
                & fstar!(r#"Hacspec_ml_kem.Commute.Matrix_bridge.inner_done
                    (${t_as_ntt}.[ $i ]) ${matrix_A} ${s_as_ntt} $i (v $j)"#));

            hax_lib::fstar!(r#"assert (v $j < v v_K); assert (v v_K <= 4)"#);
            #[cfg(hax)]
            let tt_old = *t_as_ntt;
            let product = matrix_A[i][j].ntt_multiply(&s_as_ntt[j]);
            t_as_ntt[i].add_to_ring_element(&product, j * 3328);
            hax_lib::fstar!(r#"Hacspec_ml_kem.Commute.Matrix_bridge.lemma_inner_step_full
                ${matrix_A} ${s_as_ntt} $i $j ${tt_old} ${target}"#);
        }

        #[cfg(hax)]
        let t_pre = t_as_ntt[i];
        t_as_ntt[i].add_standard_error_reduce(&error_as_ntt[i]);
        hax_lib::fstar!(r#"Hacspec_ml_kem.Commute.Matrix_bridge.lemma_row_done_finalize
            ${matrix_A} ${s_as_ntt} ${error_as_ntt} ${t_pre} (${t_as_ntt}.[ $i ]) $i ${target}"#);
    }
    hax_lib::fstar!(r#"Hacspec_ml_kem.Commute.Matrix_bridge.lemma_rows_assemble ${t_as_ntt} ${target}"#);
}

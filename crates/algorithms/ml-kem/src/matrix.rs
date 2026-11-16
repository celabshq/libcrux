use crate::{
    hash_functions::Hash, helper::cloop, invert_ntt::invert_ntt_montgomery,
    polynomial::PolynomialRingElement, sampling::sample_from_xof, vector::Operations,
};

#[inline(always)]
#[allow(non_snake_case)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(panic_free))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K"#)))]
#[cfg_attr(hax, hax_lib::ensures(|res|
    fstar!(r#"let (matrix_A, valid) = Spec.MLKEM.sample_matrix_A_ntt (Seq.slice $seed 0 32) in
        valid ==> (
        if $transpose then Libcrux_ml_kem.Polynomial.to_spec_matrix_t ${A_transpose}_future == matrix_A
        else Libcrux_ml_kem.Polynomial.to_spec_matrix_t ${A_transpose}_future == Spec.MLKEM.matrix_transpose matrix_A)"#)
))]
pub(crate) fn sample_matrix_A<const K: usize, Vector: Operations, Hasher: Hash<K>>(
    A_transpose: &mut [[PolynomialRingElement<Vector>; K]; K],
    seed: &[u8; 34],
    transpose: bool,
) {
    for i in 0..K {
        let mut seeds = [seed.clone(); K];
        for j in 0..K {
            seeds[j][32] = i as u8;
            seeds[j][33] = j as u8;
        }
        let sampled = sample_from_xof::<K, Vector, Hasher>(&seeds);
        cloop! {
            for (j, sample) in sampled.into_iter().enumerate() {
                // A[i][j] = A_transpose[j][i]
                if transpose {
                    A_transpose[j][i] = sample;
                } else {
                    A_transpose[i][j] = sample;
                }
            }
        }
    }
    ()
}

/// The following functions compute various expressions involving
/// vectors and matrices. The computation of these expressions has been
/// abstracted away into these functions in order to save on loop iterations.

/// Compute v − InverseNTT(sᵀ ◦ NTT(u))
#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(lax))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K"#)))]
#[cfg_attr(hax, hax_lib::ensures(|res|
    fstar!(r#"let open Libcrux_ml_kem.Polynomial in
        let secret_spec = to_spec_vector_t $secret_as_ntt in
        let u_spec = to_spec_vector_t $u_as_ntt in
        let v_spec = to_spec_poly_t $v in
        to_spec_poly_t $res ==
            Spec.MLKEM.(poly_sub v_spec (poly_inv_ntt (vector_dot_product_ntt #$K secret_spec u_spec))) /\
        Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 $res"#)
))]
pub(crate) fn compute_message<const K: usize, Vector: Operations>(
    v: &PolynomialRingElement<Vector>,
    secret_as_ntt: &[PolynomialRingElement<Vector>; K],
    u_as_ntt: &[PolynomialRingElement<Vector>; K],
) -> PolynomialRingElement<Vector> {
    let mut result = PolynomialRingElement::<Vector>::ZERO();

    for i in 0..K {
        let product = secret_as_ntt[i].ntt_multiply(&u_as_ntt[i]);
        result.add_to_ring_element::<K>(&product);
    }

    invert_ntt_montgomery::<K, Vector>(&mut result);
    result = v.subtract_reduce(result);

    result
}

/// Compute InverseNTT(tᵀ ◦ r̂) + e₂ + message
#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(lax))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K"#)))]
#[cfg_attr(hax, hax_lib::ensures(|res|
    fstar!(r#"let open Libcrux_ml_kem.Polynomial in
        let tt_spec = to_spec_vector_t $t_as_ntt in
        let r_spec = to_spec_vector_t $r_as_ntt in
        let e2_spec = to_spec_poly_t $error_2 in
        let m_spec = to_spec_poly_t $message in
        let res_spec = to_spec_poly_t $res in
        res_spec == Spec.MLKEM.(poly_add (poly_add (vector_dot_product_ntt #$K tt_spec r_spec) e2_spec) m_spec) /\
        Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 $res"#)
))]
pub(crate) fn compute_ring_element_v<const K: usize, Vector: Operations>(
    t_as_ntt: &[PolynomialRingElement<Vector>; K],
    r_as_ntt: &[PolynomialRingElement<Vector>; K],
    error_2: &PolynomialRingElement<Vector>,
    message: &PolynomialRingElement<Vector>,
) -> PolynomialRingElement<Vector> {
    let mut result = PolynomialRingElement::<Vector>::ZERO();

    for i in 0..K {
        let product = t_as_ntt[i].ntt_multiply(&r_as_ntt[i]);
        result.add_to_ring_element::<K>(&product);
    }

    invert_ntt_montgomery::<K, Vector>(&mut result);
    result = error_2.add_message_error_reduce(message, result);

    result
}

/// Compute u := InvertNTT(Aᵀ ◦ r̂) + e₁
#[inline(always)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(lax))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"
    Spec.MLKEM.is_rank $K /\
    (forall (i:nat). i < v $K ==>
        Libcrux_ml_kem.Polynomial.is_bounded_poly 7 (Seq.index ${error_1} i))"#)))]
#[cfg_attr(hax, hax_lib::ensures(|res|
    fstar!(r#"let open Libcrux_ml_kem.Polynomial in
        let a_spec = to_spec_matrix_t $a_as_ntt in
        let r_spec = to_spec_vector_t $r_as_ntt in
        let e_spec = to_spec_vector_t $error_1 in
        let res_spec = to_spec_vector_t $res in
        res_spec == Spec.MLKEM.(vector_add (vector_inv_ntt (matrix_vector_mul_ntt a_spec r_spec)) e_spec) /\
        (forall (i:nat). i < v $K ==>
            Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 (Seq.index $res i))"#)
))]
pub(crate) fn compute_vector_u<const K: usize, Vector: Operations>(
    a_as_ntt: &[[PolynomialRingElement<Vector>; K]; K],
    r_as_ntt: &[PolynomialRingElement<Vector>; K],
    error_1: &[PolynomialRingElement<Vector>; K],
) -> [PolynomialRingElement<Vector>; K] {
    let mut result = core::array::from_fn(|_i| PolynomialRingElement::<Vector>::ZERO());

    cloop! {
        for (i, row) in a_as_ntt.iter().enumerate() {
            cloop! {
                for (j, a_element) in row.iter().enumerate() {
                    let product = a_element.ntt_multiply(&r_as_ntt[j]);
                    result[i].add_to_ring_element::<K>(&product);
                }
            }

            invert_ntt_montgomery::<K, Vector>(&mut result[i]);
            result[i].add_error_reduce(&error_1[i]);
        }
    }

    result
}

/// Compute Â ◦ ŝ + ê
#[inline(always)]
#[allow(non_snake_case)]
#[cfg_attr(hax, hax_lib::fstar::verification_status(lax))]
#[cfg_attr(hax, hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K"#)))]
#[cfg_attr(hax, hax_lib::ensures(|res|
    fstar!(r#"let open Libcrux_ml_kem.Polynomial in
        to_spec_vector_t ${t_as_ntt}_future =
             Spec.MLKEM.compute_As_plus_e_ntt
               (to_spec_matrix_t $matrix_A) 
               (to_spec_vector_t $s_as_ntt) 
               (to_spec_vector_t $error_as_ntt) /\
        (forall (i: nat). i < v $K ==>
            Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 (Seq.index ${t_as_ntt}_future i))"#)
))]
pub(crate) fn compute_As_plus_e<const K: usize, Vector: Operations>(
    t_as_ntt: &mut [PolynomialRingElement<Vector>; K],
    matrix_A: &[[PolynomialRingElement<Vector>; K]; K],
    s_as_ntt: &[PolynomialRingElement<Vector>; K],
    error_as_ntt: &[PolynomialRingElement<Vector>; K],
) {
    cloop! {
        for (i, row) in matrix_A.iter().enumerate() {
            // This may be externally provided memory. Ensure that `t_as_ntt`
            // is all 0.
            t_as_ntt[i] = PolynomialRingElement::<Vector>::ZERO();
            cloop! {
                for (j, matrix_element) in row.iter().enumerate() {
                    let product = matrix_element.ntt_multiply(&s_as_ntt[j]);
                    t_as_ntt[i].add_to_ring_element::<K>(&product);
                }
            }
            t_as_ntt[i].add_standard_error_reduce(&error_as_ntt[i]);
        }
    };
    ()
}

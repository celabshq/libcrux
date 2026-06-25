use crate::{
    invert_ntt::ntt_inverse,
    ntt::multiply_ntts,
    parameters::{hash_functions::*, *},
    sampling::BadRejectionSamplingRandomnessError,
};

/// N.B.: According to the NIST FIPS 203 standard (Page 9, Line 519), a matrix is
/// a set of column vectors.
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
///
pub fn add_polynomials(p1: &Polynomial, p2: &Polynomial) -> Polynomial {
    createi(|j| {
        FieldElement::new(((p1[j].val as u32 + p2[j].val as u32) % FIELD_MODULUS as u32) as u16)
    })
}

pub fn sub_polynomials(p1: &Polynomial, p2: &Polynomial) -> Polynomial {
    createi(|j| {
        FieldElement::new(
            ((p1[j].val as u32 + FIELD_MODULUS as u32 - p2[j].val as u32) % FIELD_MODULUS as u32)
                as u16,
        )
    })
}

pub fn add_vectors<const RANK: usize>(v1: &Vector<RANK>, v2: &Vector<RANK>) -> Vector<RANK> {
    createi(|i| add_polynomials(&v1[i], &v2[i]))
}

pub fn multiply_matrix_by_column<const RANK: usize>(
    matrix: &Matrix<RANK>,
    vector: &Vector<RANK>,
) -> Vector<RANK> {
    createi(|i| {
        let mut result = [FieldElement::new(0); 256];
        for j in 0..RANK {
            let product = multiply_ntts(&matrix[j][i], &vector[j]);
            result = add_polynomials(&result, &product);
        }
        result
    })
}

pub fn multiply_vectors<const RANK: usize>(v1: &Vector<RANK>, v2: &Vector<RANK>) -> Polynomial {
    let mut result = [FieldElement::new(0); 256];
    for j in 0..RANK {
        let product = multiply_ntts(&v1[j], &v2[j]);
        result = add_polynomials(&result, &product);
    }
    result
}

pub fn transpose<const RANK: usize>(matrix: &Matrix<RANK>) -> Matrix<RANK> {
    createi(|i| createi(|j| matrix[j][i]))
}

// ── Decomposed operations matching the implementation's matrix.rs ──

/// Sample the matrix A from a seed. Corresponds to `sample_matrix_A` in the implementation.
///
/// When `transpose` is true, A_transpose[j][i] = sampled(i, j).
/// When `transpose` is false, A_transpose[i][j] = sampled(i, j).
#[allow(non_snake_case)]
#[hax_lib::requires(seed_for_A.len() == 32  && RANK <= 4)]
pub fn sample_matrix_A<const RANK: usize>(
    seed_for_A: &[u8],
    transpose: bool,
) -> Result<Matrix<RANK>, BadRejectionSamplingRandomnessError> {
    let mut A_as_ntt: Matrix<RANK> = [[[FieldElement::new(0); 256]; RANK]; RANK];
    let mut xof_input = [0u8; 34];
    xof_input[..32].copy_from_slice(seed_for_A);
    for i in 0..RANK {
        for j in 0..RANK {
            xof_input[32] = i as u8;
            xof_input[33] = j as u8;
            let xof_bytes: [u8; REJECTION_SAMPLING_SEED_SIZE] = XOF(&xof_input);
            let sampled = crate::sampling::sample_ntt::<70, 560, 840, 6720>(xof_bytes)?;
            if transpose {
                A_as_ntt[j][i] = sampled;
            } else {
                A_as_ntt[i][j] = sampled;
            }
        }
    }
    Ok(A_as_ntt)
}

/// Compute v − InverseNTT(sᵀ ◦ NTT(u)).
/// Corresponds to `compute_message` in the implementation's `matrix.rs`.
///
/// Used in K-PKE.Decrypt (Algorithm 15) to recover the message:
///   w ← v - NTT⁻¹(ŝᵀ ◦ NTT(u))
pub fn compute_message<const RANK: usize>(
    v: &Polynomial,
    secret_as_ntt: &Vector<RANK>,
    u_as_ntt: &Vector<RANK>,
) -> Polynomial {
    let inner_product = multiply_vectors(secret_as_ntt, u_as_ntt);
    let inner_product_inv = ntt_inverse(inner_product);
    sub_polynomials(v, &inner_product_inv)
}

/// Compute InverseNTT(tᵀ ◦ r̂) + e₂ + message.
/// Corresponds to `compute_ring_element_v` in the implementation's `matrix.rs`.
///
/// Used in K-PKE.Encrypt (Algorithm 14):
///   v ← NTT⁻¹(t̂ᵀ ◦ r̂) + e₂ + μ
pub fn compute_ring_element_v<const RANK: usize>(
    t_as_ntt: &Vector<RANK>,
    r_as_ntt: &Vector<RANK>,
    error_2: &Polynomial,
    message: &Polynomial,
) -> Polynomial {
    let inner_product = multiply_vectors(t_as_ntt, r_as_ntt);
    let inner_product_inv = ntt_inverse(inner_product);
    add_polynomials(&add_polynomials(&inner_product_inv, error_2), message)
}

/// Compute u := InvertNTT(Aᵀ ◦ r̂) + e₁.
/// Corresponds to `compute_vector_u` in the implementation's `matrix.rs`.
///
/// Used in K-PKE.Encrypt (Algorithm 14):
///   u ← NTT⁻¹(Âᵀ ◦ r̂) + e₁
pub fn compute_vector_u<const RANK: usize>(
    a_as_ntt: &Matrix<RANK>,
    r_as_ntt: &Vector<RANK>,
    error_1: &Vector<RANK>,
) -> Vector<RANK> {
    let a_transpose = transpose(a_as_ntt);
    let product = multiply_matrix_by_column(&a_transpose, r_as_ntt);
    let product_inv: Vector<RANK> = createi(|i| ntt_inverse(product[i]));
    add_vectors(&product_inv, error_1)
}

/// Compute t̂ := Â ◦ ŝ + ê.
/// Corresponds to `compute_As_plus_e` in the implementation's `matrix.rs`.
///
/// Used in K-PKE.KeyGen (Algorithm 13):
///   t̂ ← Â◦ŝ + ê
#[allow(non_snake_case)]
pub fn compute_As_plus_e<const RANK: usize>(
    a_as_ntt: &Matrix<RANK>,
    s_as_ntt: &Vector<RANK>,
    error_as_ntt: &Vector<RANK>,
) -> Vector<RANK> {
    let product = multiply_matrix_by_column(a_as_ntt, s_as_ntt);
    add_vectors(&product, error_as_ntt)
}

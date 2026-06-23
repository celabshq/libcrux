use crate::{
    matrix::{
        compute_As_plus_e, compute_message, compute_ring_element_v, compute_vector_u,
        sample_matrix_A,
    },
    ntt::vector_ntt,
    parameters::{hash_functions::*, *},
    sampling::{sample_poly_cbd, BadRejectionSamplingRandomnessError},
    serialize::{
        compress_then_serialize_message, compress_then_serialize_u, compress_then_serialize_v,
        deserialize_ring_elements_reduced, deserialize_then_decompress_message,
        deserialize_then_decompress_u, deserialize_then_decompress_v, serialize_secret_key,
    },
};

/// Helper to sample a polynomial from CBD with dynamic eta.
#[hax_lib::requires(eta == 2 || eta == 3)]
fn sample_secret(eta: usize, prf_input: &[u8; 33]) -> Polynomial {
    match eta {
        2 => {
            let out: [u8; 128] = PRF(prf_input);
            sample_poly_cbd::<128, 1024>(2, &out)
        }
        3 => {
            let out: [u8; 192] = PRF(prf_input);
            sample_poly_cbd::<192, 1536>(3, &out)
        }
        _ => panic!("unsupported eta={}", eta),
    }
}

#[hax_lib::requires(N1 > 0 && N == N1 - 1)]
fn concat_byte<const N: usize, const N1: usize>(a: &[u8; N], b: u8) -> [u8; N1] {
    let mut result = [0u8; N1];
    result[..N].copy_from_slice(a);
    result[N] = b;
    result
}

/// FIPS 203 inner loop: sample `RANK` polynomials from CBD_η using
/// PRF_η(seed ‖ domain_separator + i) for i ∈ {0, …, RANK-1}.
///
/// Captures the "for i ∈ {0, …, k-1}: v[i] ← SamplePolyCBD_η(PRF_η(seed, N))"
/// pattern that appears in K-PKE.KeyGen (Alg. 13) for s/e and in
/// K-PKE.Encrypt (Alg. 14) for r/e₁.
#[hax_lib::requires(
    seed.len() == 32
    && (eta == 2 || eta == 3)
    && RANK <= 4
    && (domain_separator as usize) + RANK < 256
)]
pub fn sample_vector_cbd<const RANK: usize>(
    eta: usize,
    seed: &[u8],
    domain_separator: u8,
) -> Vector<RANK> {
    createi(|i| {
        let prf_input: [u8; 33] =
            concat_byte::<32, 33>(seed.try_into().unwrap(), domain_separator + i as u8);
        sample_secret(eta, &prf_input)
    })
}

/// `sample_vector_cbd` followed by NTT.  Captures the "ŝ ← NTT(s)" /
/// "ê ← NTT(e)" / "r̂ ← NTT(r)" steps in K-PKE.{KeyGen,Encrypt}.
#[hax_lib::requires(
    seed.len() == 32
    && (eta == 2 || eta == 3)
    && RANK <= 4
    && (domain_separator as usize) + RANK < 256
)]
pub fn sample_vector_cbd_then_ntt<const RANK: usize>(
    eta: usize,
    seed: &[u8],
    domain_separator: u8,
) -> Vector<RANK> {
    vector_ntt(sample_vector_cbd::<RANK>(eta, seed, domain_separator))
}

/// Unpacked output of `generate_keypair_unpacked`.  Each tuple slot
/// matches the corresponding field on the libcrux `IndCpa{Public,Private}KeyUnpacked`:
/// `(secret_as_ntt, t_as_ntt, A_as_ntt, seed_for_A)`.
///
/// `A_as_ntt` is the raw sample-matrix output (`sample_matrix_A(seed, false)`);
/// callers that want the libcrux-impl convention `A[j][i] = sampled(i,j)` apply
/// `matrix::transpose` after.
pub type IndCpaKeypairUnpacked<const RANK: usize> =
    (Vector<RANK>, Vector<RANK>, Matrix<RANK>, [u8; 32]);

/// Algorithm 13: K-PKE.KeyGen — unpacked variant.  Returns the four
/// components separately instead of the serialized `(ek, dk)` byte
/// pair.  The packed `generate_keypair` is a thin serialization
/// wrapper around this function.
#[allow(non_snake_case)]
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && (params.eta1 == 2 || params.eta1 == 3)
    && key_generation_seed.len() == 32
)]
pub fn generate_keypair_unpacked<const RANK: usize>(
    params: &MlKemParams,
    key_generation_seed: &[u8],
) -> Result<IndCpaKeypairUnpacked<RANK>, BadRejectionSamplingRandomnessError> {
    hax_lib::debug_assert!(key_generation_seed.len() == 32);

    // (ρ,σ) ← G(d ‖ k)
    let mut g_input = [0u8; 33];
    g_input[..32].copy_from_slice(key_generation_seed);
    g_input[32] = RANK as u8;
    let hashed = G(&g_input);
    let (seed_for_A_slice, seed_for_secret_and_error) = hashed.split_at(32);

    // Â[i,j] ← SampleNTT(XOF(ρ, i, j))
    let A_as_ntt: Matrix<RANK> = sample_matrix_A(seed_for_A_slice, false)?;

    // s[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(σ,N)) ; ŝ ← NTT(s)
    let secret_as_ntt =
        sample_vector_cbd_then_ntt::<RANK>(params.eta1, seed_for_secret_and_error, 0);

    // e[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(σ,N)) ; ê ← NTT(e)
    let error_as_ntt =
        sample_vector_cbd_then_ntt::<RANK>(params.eta1, seed_for_secret_and_error, RANK as u8);

    // t̂ ← Â◦ŝ + ê
    let t_as_ntt = compute_As_plus_e(&A_as_ntt, &secret_as_ntt, &error_as_ntt);

    let mut seed_for_A = [0u8; 32];
    seed_for_A.copy_from_slice(seed_for_A_slice);

    Ok((secret_as_ntt, t_as_ntt, A_as_ntt, seed_for_A))
}

/// Algorithm 13: K-PKE.KeyGen
///
/// Generates an encryption key and a corresponding decryption key.
///
/// ```plaintext
/// Output: encryption key ekₚₖₑ ∈ 𝔹^{384k+32}.
/// Output: decryption key dkₚₖₑ ∈ 𝔹^{384k}.
///
/// d ←$ B
/// (ρ,σ) ← G(d)
/// N ← 0
/// for (i ← 0; i < k; i++)
///     for(j ← 0; j < k; j++)
///         Â[i,j] ← SampleNTT(XOF(ρ, j, i))
///     end for
/// end for
/// for(i ← 0; i < k; i++)
///     s[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(σ,N))
///     N ← N + 1
/// end for
/// for(i ← 0; i < k; i++)
///     e[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(σ,N))
///     N ← N + 1
/// end for
/// ŝ ← NTT(s)
/// ê ← NTT(e)
/// t̂ ← Â◦ŝ + ê
/// ekₚₖₑ ← ByteEncode₁₂(t̂) ‖ ρ
/// dkₚₖₑ ← ByteEncode₁₂(ŝ)
/// ```
#[allow(non_snake_case)]
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32
    && DK_PKE_SIZE == RANK * BYTES_PER_RING_ELEMENT
    && (params.eta1 == 2 || params.eta1 == 3)
    && key_generation_seed.len() == 32
)]
pub fn generate_keypair<const RANK: usize, const EK_SIZE: usize, const DK_PKE_SIZE: usize>(
    params: &MlKemParams,
    key_generation_seed: &[u8],
) -> Result<([u8; EK_SIZE], [u8; DK_PKE_SIZE]), BadRejectionSamplingRandomnessError> {
    hax_lib::debug_assert!(
        EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32
            && DK_PKE_SIZE == RANK * BYTES_PER_RING_ELEMENT
            && key_generation_seed.len() == 32
    );

    let (secret_as_ntt, t_as_ntt, _A_as_ntt, seed_for_A) =
        generate_keypair_unpacked::<RANK>(params, key_generation_seed)?;

    // ekₚₖₑ ← ByteEncode₁₂(t̂) ‖ ρ
    let t_encoded: [u8; DK_PKE_SIZE] = serialize_secret_key::<RANK, DK_PKE_SIZE>(&t_as_ntt);
    let mut ek = [0u8; EK_SIZE];
    ek[..DK_PKE_SIZE].copy_from_slice(&t_encoded);
    ek[DK_PKE_SIZE..].copy_from_slice(&seed_for_A);

    // dkₚₖₑ ← ByteEncode₁₂(ŝ)
    let dk: [u8; DK_PKE_SIZE] = serialize_secret_key::<RANK, DK_PKE_SIZE>(&secret_as_ntt);

    Ok((ek, dk))
}

/// Algorithm 14: K-PKE.Encrypt
///
/// Uses the encryption key to encrypt a plaintext message using the randomness r.
///
/// ```plaintext
/// Input: encryption key ekₚₖₑ ∈ 𝔹^{384k+32}.
/// Input: message m ∈ 𝔹^{32}.
/// Input: encryption randomness r ∈ 𝔹^{32}.
/// Output: ciphertext c ∈ 𝔹^{32(dᵤk + dᵥ)}.
///
/// N ← 0
/// t̂ ← ByteDecode₁₂(ekₚₖₑ[0:384k])
/// ρ ← ekₚₖₑ[384k: 384k + 32]
/// for (i ← 0; i < k; i++)
///     for(j ← 0; j < k; j++)
///         Â[i,j] ← SampleNTT(XOF(ρ, j, i))
///     end for
/// end for
/// for(i ← 0; i < k; i++)
///     r[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(r,N))
///     N ← N + 1
/// end for
/// for(i ← 0; i < k; i++)
///     e₁[i] ← SamplePolyCBD_{η₂}(PRF_{η₂}(r,N))
///     N ← N + 1
/// end for
/// e₂ ← SamplePolyCBD_{η₂}(PRF_{η₂}(r,N))
/// r̂ ← NTT(r)
/// u ← NTT⁻¹(Âᵀ ◦ r̂) + e₁
/// μ ← Decompress₁(ByteDecode₁(m))
/// v ← NTT⁻¹(t̂ᵀ ◦ r̂) + e₂ + μ
/// c₁ ← ByteEncode_{dᵤ}(Compress_{dᵤ}(u))
/// c₂ ← ByteEncode_{dᵥ}(Compress_{dᵥ}(v))
/// return c ← (c₁ ‖ c₂)
/// ```
#[allow(non_snake_case)]
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
    && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
    && CT_SIZE == U_SIZE + V_SIZE
    && ek.len() == RANK * BYTES_PER_RING_ELEMENT + 32
    && (params.eta1 == 2 || params.eta1 == 3)
    && (params.eta2 == 2 || params.eta2 == 3)
    && randomness.len() == 32
)]
pub fn encrypt<
    const RANK: usize,
    const U_SIZE: usize,
    const V_SIZE: usize,
    const CT_SIZE: usize,
>(
    params: &MlKemParams,
    ek: &[u8],
    message: &[u8; 32],
    randomness: &[u8],
) -> Result<[u8; CT_SIZE], BadRejectionSamplingRandomnessError> {
    hax_lib::debug_assert!(
        U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
            && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
            && CT_SIZE == U_SIZE + V_SIZE
            && ek.len() == RANK * BYTES_PER_RING_ELEMENT + 32
            && randomness.len() == 32
    );

    let t_encoded_size = params.t_as_ntt_encoded_size();

    // t̂ ← ByteDecode₁₂(ekₚₖₑ[0:384k])
    let t_as_ntt: Vector<RANK> = deserialize_ring_elements_reduced::<RANK>(&ek[..t_encoded_size]);

    // ρ ← ekₚₖₑ[384k: 384k + 32]
    let seed_for_A = &ek[t_encoded_size..];

    // Â[i,j] ← SampleNTT(XOF(ρ, j, i))
    let A_as_ntt: Matrix<RANK> = sample_matrix_A(seed_for_A, false)?;

    encrypt_unpacked::<RANK, U_SIZE, V_SIZE, CT_SIZE>(
        params, &t_as_ntt, &A_as_ntt, message, randomness,
    )
}

/// K-PKE.Encrypt — unpacked variant.  Skips the
/// `ByteDecode₁₂(ek)` / `sample_matrix_A(seed_for_A)` decoding step
/// and consumes the already-decoded `t_as_ntt` and `A_as_ntt`
/// directly.  The packed `encrypt` is a thin decoding wrapper.
#[allow(non_snake_case)]
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
    && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
    && CT_SIZE == U_SIZE + V_SIZE
    && (params.eta1 == 2 || params.eta1 == 3)
    && (params.eta2 == 2 || params.eta2 == 3)
    && randomness.len() == 32
)]
pub fn encrypt_unpacked<
    const RANK: usize,
    const U_SIZE: usize,
    const V_SIZE: usize,
    const CT_SIZE: usize,
>(
    params: &MlKemParams,
    t_as_ntt: &Vector<RANK>,
    A_as_ntt: &Matrix<RANK>,
    message: &[u8; 32],
    randomness: &[u8],
) -> Result<[u8; CT_SIZE], BadRejectionSamplingRandomnessError> {
    hax_lib::debug_assert!(
        U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
            && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
            && CT_SIZE == U_SIZE + V_SIZE
            && randomness.len() == 32
    );

    // r[i] ← SamplePolyCBD_{η₁}(PRF_{η₁}(r,N)) ; r̂ ← NTT(r)
    let r_as_ntt = sample_vector_cbd_then_ntt::<RANK>(params.eta1, randomness, 0);

    // e₁[i] ← SamplePolyCBD_{η₂}(PRF_{η₂}(r,N))
    let error_1 = sample_vector_cbd::<RANK>(params.eta2, randomness, RANK as u8);

    // e₂ ← SamplePolyCBD_{η₂}(PRF_{η₂}(r,N))
    let mut prf_input = [0u8; 33];
    prf_input[..32].copy_from_slice(randomness);
    prf_input[32] = (RANK * 2) as u8;
    let error_2 = sample_secret(params.eta2, &prf_input);

    // u ← NTT⁻¹(Âᵀ ◦ r̂) + e₁
    let u = compute_vector_u(A_as_ntt, &r_as_ntt, &error_1);

    // μ ← Decompress₁(ByteDecode₁(m))
    let message_as_ring_element = deserialize_then_decompress_message(message);

    // v ← NTT⁻¹(t̂ᵀ ◦ r̂) + e₂ + μ
    let v = compute_ring_element_v(t_as_ntt, &r_as_ntt, &error_2, &message_as_ring_element);

    // c₁ ← ByteEncode_{dᵤ}(Compress_{dᵤ}(u))
    let c1: [u8; U_SIZE] = compress_then_serialize_u::<RANK, U_SIZE>(&u, params.du);

    // c₂ ← ByteEncode_{dᵥ}(Compress_{dᵥ}(v))
    let c2: [u8; V_SIZE] = compress_then_serialize_v::<V_SIZE>(&v, params.dv);

    // c ← (c₁ ‖ c₂)
    let mut c = [0u8; CT_SIZE];
    c[..U_SIZE].copy_from_slice(&c1);
    c[U_SIZE..].copy_from_slice(&c2);

    Ok(c)
}

/// Algorithm 15: K-PKE.Decrypt
///
/// Uses the decryption key to decrypt a ciphertext.
///
/// ```plaintext
/// Input: decryption key dkₚₖₑ ∈ 𝔹^{384k}.
/// Input: ciphertext c ∈ 𝔹^{32(dᵤk + dᵥ)}.
/// Output: message m ∈ 𝔹^{32}.
///
/// c₁ ← c[0 : 32dᵤk]
/// c₂ ← c[32dᵤk : 32(dᵤk + dᵥ)]
/// u ← Decompress_{dᵤ}(ByteDecode_{dᵤ}(c₁))
/// v ← Decompress_{dᵥ}(ByteDecode_{dᵥ}(c₂))
/// ŝ ← ByteDecode₁₂(dkₚₖₑ)
/// w ← v - NTT⁻¹(ŝᵀ ◦ NTT(u))
/// m ← ByteEncode₁(Compress₁(w))
/// return m
/// ```
#[allow(non_snake_case)]
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && dk.len() == RANK * BYTES_PER_RING_ELEMENT
    && ciphertext.len() == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du + COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
)]
pub fn decrypt<const RANK: usize>(params: &MlKemParams, dk: &[u8], ciphertext: &[u8]) -> [u8; 32] {
    hax_lib::debug_assert!(
        dk.len() == RANK * BYTES_PER_RING_ELEMENT
            && ciphertext.len()
                == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du
                    + COEFFICIENTS_IN_RING_ELEMENT * params.dv)
                    / 8
    );

    // ŝ ← ByteDecode₁₂(dkₚₖₑ)
    let secret_as_ntt: Vector<RANK> = deserialize_ring_elements_reduced::<RANK>(dk);

    decrypt_unpacked::<RANK>(params, &secret_as_ntt, ciphertext)
}

/// K-PKE.Decrypt — unpacked variant.  Skips the
/// `ByteDecode₁₂(dk)` decoding step and consumes the already-decoded
/// `secret_as_ntt` directly.  The packed `decrypt` is a thin
/// decoding wrapper.
#[allow(non_snake_case)]
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && ciphertext.len() == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du + COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
)]
pub fn decrypt_unpacked<const RANK: usize>(
    params: &MlKemParams,
    secret_as_ntt: &Vector<RANK>,
    ciphertext: &[u8],
) -> [u8; 32] {
    hax_lib::debug_assert!(
        ciphertext.len()
            == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du
                + COEFFICIENTS_IN_RING_ELEMENT * params.dv)
                / 8
    );
    let u_encoded_size = params.u_encoded_size();

    // u ← Decompress_{dᵤ}(ByteDecode_{dᵤ}(c₁))
    let u: Vector<RANK> =
        deserialize_then_decompress_u::<RANK>(&ciphertext[0..u_encoded_size], params.du);

    // v ← Decompress_{dᵥ}(ByteDecode_{dᵥ}(c₂))
    let v = deserialize_then_decompress_v(&ciphertext[u_encoded_size..], params.dv);

    // w ← v - NTT⁻¹(ŝᵀ ◦ NTT(u))
    let u_as_ntt = vector_ntt(u);
    let w = compute_message(&v, secret_as_ntt, &u_as_ntt);

    // m ← ByteEncode₁(Compress₁(w))
    compress_then_serialize_message(w)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let seed = [42u8; 32];
        let (ek, dk) = generate_keypair::<3, { ML_KEM_768_EK_SIZE }, { ML_KEM_768_DK_PKE_SIZE }>(
            &ML_KEM_768,
            &seed,
        )
        .unwrap();

        let message = [0xABu8; 32];
        let randomness = [0xCDu8; 32];
        let ciphertext = encrypt::<
            3,
            { ML_KEM_768_U_SIZE },
            { ML_KEM_768_V_SIZE },
            { ML_KEM_768_CT_SIZE },
        >(&ML_KEM_768, &ek, &message, &randomness)
        .unwrap();
        let decrypted = decrypt::<3>(&ML_KEM_768, &dk, &ciphertext);

        assert_eq!(decrypted, message);
    }

    #[test]
    fn encrypt_deterministic() {
        let seed = [1u8; 32];
        let (ek, _dk) = generate_keypair::<3, { ML_KEM_768_EK_SIZE }, { ML_KEM_768_DK_PKE_SIZE }>(
            &ML_KEM_768,
            &seed,
        )
        .unwrap();

        let message = [0x55u8; 32];
        let randomness = [0x77u8; 32];
        let c1 =
            encrypt::<3, { ML_KEM_768_U_SIZE }, { ML_KEM_768_V_SIZE }, { ML_KEM_768_CT_SIZE }>(
                &ML_KEM_768,
                &ek,
                &message,
                &randomness,
            )
            .unwrap();
        let c2 =
            encrypt::<3, { ML_KEM_768_U_SIZE }, { ML_KEM_768_V_SIZE }, { ML_KEM_768_CT_SIZE }>(
                &ML_KEM_768,
                &ek,
                &message,
                &randomness,
            )
            .unwrap();

        assert_eq!(c1, c2);
    }

    #[test]
    fn decrypt_with_wrong_key() {
        let seed1 = [1u8; 32];
        let seed2 = [2u8; 32];
        let (ek, _dk1) = generate_keypair::<3, { ML_KEM_768_EK_SIZE }, { ML_KEM_768_DK_PKE_SIZE }>(
            &ML_KEM_768,
            &seed1,
        )
        .unwrap();
        let (_ek2, dk2) =
            generate_keypair::<3, { ML_KEM_768_EK_SIZE }, { ML_KEM_768_DK_PKE_SIZE }>(
                &ML_KEM_768,
                &seed2,
            )
            .unwrap();

        let message = [0xAAu8; 32];
        let randomness = [0xBBu8; 32];
        let ciphertext = encrypt::<
            3,
            { ML_KEM_768_U_SIZE },
            { ML_KEM_768_V_SIZE },
            { ML_KEM_768_CT_SIZE },
        >(&ML_KEM_768, &ek, &message, &randomness)
        .unwrap();
        let decrypted = decrypt::<3>(&ML_KEM_768, &dk2, &ciphertext);

        assert_ne!(decrypted, message);
    }
}

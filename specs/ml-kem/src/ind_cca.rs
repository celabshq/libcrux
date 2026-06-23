use crate::matrix::transpose;
use crate::parameters::hash_functions::*;
use crate::parameters::*;
use crate::sampling::BadRejectionSamplingRandomnessError;
use crate::serialize::serialize_public_key;
use crate::{ind_cpa, serialize};

/// Algorithm 16: ML-KEM.KeyGen_internal
///
/// ```plaintext
/// Input: d ∈ 𝔹³², z ∈ 𝔹³².
/// Output: encapsulation key ek ∈ 𝔹^{384k+32}.
/// Output: decapsulation key dk ∈ 𝔹^{768k+96}.
///
/// (ekₚₖₑ, dkₚₖₑ) ← K-PKE.KeyGen(d)
/// ek ← ekₚₖₑ
/// dk ← (dkₚₖₑ ‖ ek ‖ H(ek) ‖ z)
/// return (ek, dk)
/// ```
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32
    && DK_PKE_SIZE == RANK * BYTES_PER_RING_ELEMENT
    && DK_SIZE == DK_PKE_SIZE + EK_SIZE + H_DIGEST_SIZE + 32
    && (params.eta1 == 2 || params.eta1 == 3)
)]
fn keygen_internal<
    const RANK: usize,
    const EK_SIZE: usize,
    const DK_PKE_SIZE: usize,
    const DK_SIZE: usize,
>(
    params: &MlKemParams,
    d: &[u8; 32],
    z: &[u8; 32],
) -> Result<([u8; EK_SIZE], [u8; DK_SIZE]), BadRejectionSamplingRandomnessError> {
    hax_lib::debug_assert!(
        EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32
            && DK_PKE_SIZE == RANK * BYTES_PER_RING_ELEMENT
            && DK_SIZE == DK_PKE_SIZE + EK_SIZE + H_DIGEST_SIZE + 32
    );
    let (ek, dk_pke) = ind_cpa::generate_keypair::<RANK, EK_SIZE, DK_PKE_SIZE>(params, d)?;

    // dk ← (dkₚₖₑ ‖ ek ‖ H(ek) ‖ z)
    let mut dk = [0u8; DK_SIZE];
    dk[..DK_PKE_SIZE].copy_from_slice(&dk_pke);
    dk[DK_PKE_SIZE..DK_PKE_SIZE + EK_SIZE].copy_from_slice(&ek);
    dk[DK_PKE_SIZE + EK_SIZE..DK_PKE_SIZE + EK_SIZE + H_DIGEST_SIZE].copy_from_slice(&H(&ek));
    dk[DK_PKE_SIZE + EK_SIZE + H_DIGEST_SIZE..].copy_from_slice(z);

    Ok((ek, dk))
}

/// Algorithm 17: ML-KEM.Encaps_internal
///
/// ```plaintext
/// Input: encapsulation key ek ∈ 𝔹^{384k+32}.
/// Input: m ∈ 𝔹³².
/// Output: shared key K ∈ 𝔹³².
/// Output: ciphertext c ∈ 𝔹^{32(dᵤk+dᵥ)}.
///
/// (K, r) ← G(m ‖ H(ek))
/// c ← K-PKE.Encrypt(ek, m, r)
/// return (K, c)
/// ```
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
    && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
    && CT_SIZE == U_SIZE + V_SIZE
    && ek.len() == RANK * BYTES_PER_RING_ELEMENT + 32
    && (params.eta1 == 2 || params.eta1 == 3)
    && (params.eta2 == 2 || params.eta2 == 3)
)]
fn encaps_internal<
    const RANK: usize,
    const U_SIZE: usize,
    const V_SIZE: usize,
    const CT_SIZE: usize,
>(
    params: &MlKemParams,
    ek: &[u8],
    m: &[u8; 32],
) -> Result<([u8; 32], [u8; CT_SIZE]), BadRejectionSamplingRandomnessError> {
    hax_lib::debug_assert!(
        U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
            && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
            && CT_SIZE == U_SIZE + V_SIZE
            && ek.len() == RANK * BYTES_PER_RING_ELEMENT + 32
    );
    // (K, r) ← G(m ‖ H(ek))
    let mut to_hash = [0u8; 64];
    to_hash[..32].copy_from_slice(m);
    to_hash[32..].copy_from_slice(&H(ek));
    let hashed = G(&to_hash);
    let (shared_secret, pseudorandomness) = hashed.split_at(32);

    let r: [u8; 32] = pseudorandomness[..32].try_into().unwrap();

    // c ← K-PKE.Encrypt(ek, m, r)
    let c = ind_cpa::encrypt::<RANK, U_SIZE, V_SIZE, CT_SIZE>(params, ek, m, &r)?;

    let mut k = [0u8; 32];
    k.copy_from_slice(shared_secret);
    Ok((k, c))
}

/// Algorithm 18: ML-KEM.Decaps_internal
///
/// ```plaintext
/// Input: decapsulation key dk ∈ 𝔹^{768k+96}.
/// Input: ciphertext c ∈ 𝔹^{32(dᵤk+dᵥ)}.
/// Output: shared key K ∈ 𝔹³².
///
/// dkₚₖₑ ← dk[0 : 384k]
/// ekₚₖₑ ← dk[384k : 768k + 32]
/// h ← dk[768k + 32 : 768k + 64]
/// z ← dk[768k + 64 : 768k + 96]
/// m′ ← K-PKE.Decrypt(dkₚₖₑ, c)
/// (K′, r′) ← G(m′ ‖ h)
/// K̃ ← J(z ‖ c)
/// c′ ← K-PKE.Encrypt(ekₚₖₑ, m′, r′)
/// if c ≠ c′ then
///     K′ ← K̃
/// end if
/// return K′
/// ```
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32
    && DK_PKE_SIZE == RANK * BYTES_PER_RING_ELEMENT
    && DK_SIZE == DK_PKE_SIZE + EK_SIZE + H_DIGEST_SIZE + 32
    && U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
    && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
    && CT_SIZE == U_SIZE + V_SIZE
    && J_INPUT_SIZE == 32 + CT_SIZE
    && (params.eta1 == 2 || params.eta1 == 3)
    && (params.eta2 == 2 || params.eta2 == 3)
)]
fn decaps_internal<
    const RANK: usize,
    const EK_SIZE: usize,
    const DK_SIZE: usize,
    const DK_PKE_SIZE: usize,
    const U_SIZE: usize,
    const V_SIZE: usize,
    const CT_SIZE: usize,
    const J_INPUT_SIZE: usize,
>(
    params: &MlKemParams,
    dk: &[u8; DK_SIZE],
    c: &[u8; CT_SIZE],
) -> Result<[u8; 32], BadRejectionSamplingRandomnessError> {
    hax_lib::debug_assert!(
        EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32
            && DK_PKE_SIZE == RANK * BYTES_PER_RING_ELEMENT
            && DK_SIZE == DK_PKE_SIZE + EK_SIZE + H_DIGEST_SIZE + 32
            && U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
            && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
            && CT_SIZE == U_SIZE + V_SIZE
            && J_INPUT_SIZE == 32 + CT_SIZE
    );
    // dkₚₖₑ ← dk[0 : 384k]
    let dk_pke = &dk[..DK_PKE_SIZE];

    // ekₚₖₑ ← dk[384k : 768k + 32]
    let ek = &dk[DK_PKE_SIZE..DK_PKE_SIZE + EK_SIZE];

    // h ← dk[768k + 32 : 768k + 64]
    let h = &dk[DK_PKE_SIZE + EK_SIZE..DK_PKE_SIZE + EK_SIZE + H_DIGEST_SIZE];

    // z ← dk[768k + 64 : 768k + 96]
    let z = &dk[DK_PKE_SIZE + EK_SIZE + H_DIGEST_SIZE..];

    // m′ ← K-PKE.Decrypt(dkₚₖₑ, c)
    let m_prime = ind_cpa::decrypt::<RANK>(params, dk_pke, c);

    // (K′, r′) ← G(m′ ‖ h)
    let mut to_hash = [0u8; 64];
    to_hash[..32].copy_from_slice(&m_prime);
    to_hash[32..].copy_from_slice(h);
    let hashed = G(&to_hash);
    let (success_shared_secret, pseudorandomness) = hashed.split_at(32);

    let r_prime: [u8; 32] = pseudorandomness[..32].try_into().unwrap();

    // K̃ ← J(z ‖ c)
    let mut j_input = [0u8; J_INPUT_SIZE];
    j_input[..32].copy_from_slice(z);
    j_input[32..].copy_from_slice(c);
    let rejection_shared_secret: [u8; 32] = J(&j_input);

    // c′ ← K-PKE.Encrypt(ekₚₖₑ, m′, r′)
    let c_prime =
        ind_cpa::encrypt::<RANK, U_SIZE, V_SIZE, CT_SIZE>(params, ek, &m_prime, &r_prime)?;

    // if c ≠ c′ then K′ ← K̃
    if c[..] == c_prime[..] {
        let mut k = [0u8; 32];
        k.copy_from_slice(success_shared_secret);
        Ok(k)
    } else {
        Ok(rejection_shared_secret)
    }
}

/// Algorithm 19: ML-KEM.KeyGen
///
/// Generates an encapsulation key and a corresponding decapsulation key.
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32
    && DK_PKE_SIZE == RANK * BYTES_PER_RING_ELEMENT
    && DK_SIZE == DK_PKE_SIZE + EK_SIZE + H_DIGEST_SIZE + 32
    && (params.eta1 == 2 || params.eta1 == 3)
)]
pub fn generate_keypair<
    const RANK: usize,
    const EK_SIZE: usize,
    const DK_SIZE: usize,
    const DK_PKE_SIZE: usize,
>(
    params: &MlKemParams,
    randomness: &[u8; 64],
) -> Result<([u8; EK_SIZE], [u8; DK_SIZE]), BadRejectionSamplingRandomnessError> {
    hax_lib::debug_assert!(
        EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32
            && DK_PKE_SIZE == RANK * BYTES_PER_RING_ELEMENT
            && DK_SIZE == DK_PKE_SIZE + EK_SIZE + H_DIGEST_SIZE + 32
    );
    let d: &[u8; 32] = randomness[..32].try_into().unwrap();
    let z: &[u8; 32] = randomness[32..].try_into().unwrap();
    keygen_internal::<RANK, EK_SIZE, DK_PKE_SIZE, DK_SIZE>(params, d, z)
}

/// Modulus check for encapsulation key validation (FIPS 203 Section 7.2).
///
/// Verifies that ByteEncode₁₂(ByteDecode₁₂(ek[..384k])) == ek[..384k].
#[hax_lib::requires(
    params.rank <= 4
    && EK_SIZE == params.rank * BYTES_PER_RING_ELEMENT + 32
)]
pub fn public_key_modulus_check<const EK_SIZE: usize>(
    params: &MlKemParams,
    ek: &[u8; EK_SIZE],
) -> bool {
    let t_size = params.t_as_ntt_encoded_size();
    let encoded_ring_elements = &ek[..t_size];
    let mut valid = true;
    // Decode and re-encode; the round-trip should be identity for valid keys
    for chunk in encoded_ring_elements.chunks_exact(BYTES_PER_RING_ELEMENT) {
        let decoded =
            serialize::byte_decode::<{ 32 * 12 }, { 256 * 12 }>(chunk.try_into().unwrap(), 12);
        let re_encoded = serialize::byte_encode::<{ 32 * 12 }, { 256 * 12 }>(decoded, 12);
        if chunk != re_encoded.as_slice() {
            valid = false;
        }
    }
    valid
}

/// Algorithm 20: ML-KEM.Encaps
///
/// Uses the encapsulation key to generate a shared key and ciphertext.
/// Includes modulus check on ek per FIPS 203 Section 7.2.
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32
    && U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
    && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
    && CT_SIZE == U_SIZE + V_SIZE
    && (params.eta1 == 2 || params.eta1 == 3)
    && (params.eta2 == 2 || params.eta2 == 3)
)]
pub fn encapsulate<
    const RANK: usize,
    const EK_SIZE: usize,
    const U_SIZE: usize,
    const V_SIZE: usize,
    const CT_SIZE: usize,
>(
    params: &MlKemParams,
    ek: &[u8; EK_SIZE],
    m: &[u8; 32],
) -> Result<([u8; 32], [u8; CT_SIZE]), BadRejectionSamplingRandomnessError> {
    hax_lib::debug_assert!(
        EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32
            && U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
            && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
            && CT_SIZE == U_SIZE + V_SIZE
    );
    // Modulus check
    hax_lib::debug_assert!(
        public_key_modulus_check(params, ek),
        "encapsulation key modulus check failed"
    );

    encaps_internal::<RANK, U_SIZE, V_SIZE, CT_SIZE>(params, ek, m)
}

/// Algorithm 21: ML-KEM.Decaps
///
/// Uses the decapsulation key to produce a shared key from a ciphertext.
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32
    && DK_PKE_SIZE == RANK * BYTES_PER_RING_ELEMENT
    && DK_SIZE == DK_PKE_SIZE + EK_SIZE + H_DIGEST_SIZE + 32
    && U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
    && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
    && CT_SIZE == U_SIZE + V_SIZE
    && J_INPUT_SIZE == 32 + CT_SIZE
    && (params.eta1 == 2 || params.eta1 == 3)
    && (params.eta2 == 2 || params.eta2 == 3)
)]
pub fn decapsulate<
    const RANK: usize,
    const EK_SIZE: usize,
    const DK_SIZE: usize,
    const DK_PKE_SIZE: usize,
    const U_SIZE: usize,
    const V_SIZE: usize,
    const CT_SIZE: usize,
    const J_INPUT_SIZE: usize,
>(
    params: &MlKemParams,
    dk: &[u8; DK_SIZE],
    c: &[u8; CT_SIZE],
) -> Result<[u8; 32], BadRejectionSamplingRandomnessError> {
    hax_lib::debug_assert!(
        EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32
            && DK_PKE_SIZE == RANK * BYTES_PER_RING_ELEMENT
            && DK_SIZE == DK_PKE_SIZE + EK_SIZE + H_DIGEST_SIZE + 32
            && U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
            && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
            && CT_SIZE == U_SIZE + V_SIZE
            && J_INPUT_SIZE == 32 + CT_SIZE
    );
    decaps_internal::<RANK, EK_SIZE, DK_SIZE, DK_PKE_SIZE, U_SIZE, V_SIZE, CT_SIZE, J_INPUT_SIZE>(
        params, dk, c,
    )
}

// ── Unpacked-API CCA helpers (P5) ────────────────────────────────────
//
// Convention: `m_A` is in **libcrux convention** (transposed form,
// `A_transposed[j][i] = sampled(i, j)`), matching the `A` field on
// `IndCpaPublicKeyUnpacked` in the libcrux impl.  The helpers transpose
// internally before delegating to the raw-form `ind_cpa::*_unpacked`
// helpers.

/// Tuple returned by `ind_cca_unpack_generate_keypair`.  Components in
/// libcrux-impl order:
/// `(secret_as_ntt, t_as_ntt, m_A_transposed, seed_for_A, public_key_hash, implicit_rejection_value)`.
pub type IndCcaUnpackedKeyPair<const RANK: usize> = (
    Vector<RANK>,
    Vector<RANK>,
    Matrix<RANK>,
    [u8; 32],
    [u8; 32],
    [u8; 32],
);

/// ML-KEM.KeyGen — unpacked variant.  Returns the components used by
/// `MlKemKeyPairUnpacked` in the libcrux impl, instead of the
/// serialized `(ek, dk)` pair.
#[allow(non_snake_case)]
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32
    && (params.eta1 == 2 || params.eta1 == 3)
)]
pub fn ind_cca_unpack_generate_keypair<const RANK: usize, const EK_SIZE: usize>(
    params: &MlKemParams,
    randomness: &[u8; 64],
) -> Result<IndCcaUnpackedKeyPair<RANK>, BadRejectionSamplingRandomnessError> {
    hax_lib::debug_assert!(EK_SIZE == RANK * BYTES_PER_RING_ELEMENT + 32);

    let d: &[u8; 32] = randomness[..32].try_into().unwrap();
    let z: &[u8; 32] = randomness[32..].try_into().unwrap();

    let (secret_as_ntt, t_as_ntt, A_as_ntt, seed_for_A) =
        ind_cpa::generate_keypair_unpacked::<RANK>(params, d)?;

    // Libcrux stores A in transposed form on `IndCpaPublicKeyUnpacked.A`.
    let m_A: Matrix<RANK> = transpose(&A_as_ntt);

    // public_key_hash = H(serialize_public_key(t_as_ntt, seed_for_A))
    let ek: [u8; EK_SIZE] = serialize_public_key::<RANK, EK_SIZE>(&t_as_ntt, &seed_for_A);
    let public_key_hash: [u8; 32] = H(&ek);

    let mut implicit_rejection_value = [0u8; 32];
    implicit_rejection_value.copy_from_slice(z);

    Ok((
        secret_as_ntt,
        t_as_ntt,
        m_A,
        seed_for_A,
        public_key_hash,
        implicit_rejection_value,
    ))
}

/// ML-KEM.Encaps — unpacked variant.  Skips the `H(ek)` and
/// `ByteDecode₁₂(ek)` decoding steps; consumes the precomputed
/// `public_key_hash`, `t_as_ntt`, and `m_A` (libcrux-transposed form)
/// directly.
#[allow(non_snake_case)]
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
    && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
    && CT_SIZE == U_SIZE + V_SIZE
    && (params.eta1 == 2 || params.eta1 == 3)
    && (params.eta2 == 2 || params.eta2 == 3)
)]
pub fn ind_cca_unpack_encapsulate<
    const RANK: usize,
    const U_SIZE: usize,
    const V_SIZE: usize,
    const CT_SIZE: usize,
>(
    params: &MlKemParams,
    public_key_hash: &[u8; 32],
    t_as_ntt: &Vector<RANK>,
    m_A: &Matrix<RANK>,
    randomness: &[u8; 32],
) -> Result<([u8; 32], [u8; CT_SIZE]), BadRejectionSamplingRandomnessError> {
    // (K, r) ← G(m ‖ H(ek))
    let mut to_hash = [0u8; 64];
    to_hash[..32].copy_from_slice(randomness);
    to_hash[32..].copy_from_slice(public_key_hash);
    let hashed = G(&to_hash);
    let (shared_secret, pseudorandomness) = hashed.split_at(32);

    // Un-transpose to raw form for ind_cpa::encrypt_unpacked.
    let A_as_ntt: Matrix<RANK> = transpose(m_A);

    let c = ind_cpa::encrypt_unpacked::<RANK, U_SIZE, V_SIZE, CT_SIZE>(
        params,
        t_as_ntt,
        &A_as_ntt,
        randomness,
        pseudorandomness,
    )?;

    let mut k = [0u8; 32];
    k.copy_from_slice(shared_secret);
    Ok((k, c))
}

/// ML-KEM.Decaps — unpacked variant.  Skips the `dk` deconstruction
/// (`dkₚₖₑ`, `ekₚₖₑ`, `h`, `z`) and the `ByteDecode₁₂(dkₚₖₑ)` step;
/// consumes the components precomputed by `ind_cca_unpack_generate_keypair`
/// directly.  Re-encrypts via `ind_cpa::encrypt_unpacked` to check the
/// FO transform's c == c′ condition.
#[allow(non_snake_case)]
#[hax_lib::requires(
    RANK <= 4 && params.rank == RANK
    && U_SIZE == (RANK * COEFFICIENTS_IN_RING_ELEMENT * params.du) / 8
    && V_SIZE == (COEFFICIENTS_IN_RING_ELEMENT * params.dv) / 8
    && CT_SIZE == U_SIZE + V_SIZE
    && J_INPUT_SIZE == 32 + CT_SIZE
    && (params.eta1 == 2 || params.eta1 == 3)
    && (params.eta2 == 2 || params.eta2 == 3)
)]
pub fn ind_cca_unpack_decapsulate<
    const RANK: usize,
    const U_SIZE: usize,
    const V_SIZE: usize,
    const CT_SIZE: usize,
    const J_INPUT_SIZE: usize,
>(
    params: &MlKemParams,
    public_key_hash: &[u8; 32],
    implicit_rejection_value: &[u8; 32],
    ciphertext: &[u8; CT_SIZE],
    secret_as_ntt: &Vector<RANK>,
    t_as_ntt: &Vector<RANK>,
    m_A: &Matrix<RANK>,
) -> Result<[u8; 32], BadRejectionSamplingRandomnessError> {
    // m′ ← K-PKE.Decrypt(secret_as_ntt, c)
    let m_prime = ind_cpa::decrypt_unpacked::<RANK>(params, secret_as_ntt, ciphertext);

    // (K′, r′) ← G(m′ ‖ h)
    let mut to_hash = [0u8; 64];
    to_hash[..32].copy_from_slice(&m_prime);
    to_hash[32..].copy_from_slice(public_key_hash);
    let hashed = G(&to_hash);
    let (success_shared_secret, pseudorandomness) = hashed.split_at(32);

    // K̃ ← J(z ‖ c)
    let mut j_input = [0u8; J_INPUT_SIZE];
    j_input[..32].copy_from_slice(implicit_rejection_value);
    j_input[32..].copy_from_slice(ciphertext);
    let rejection_shared_secret: [u8; 32] = J(&j_input);

    // c′ ← K-PKE.Encrypt_unpacked(t_as_ntt, A, m′, r′)
    let A_as_ntt: Matrix<RANK> = transpose(m_A);
    let c_prime = ind_cpa::encrypt_unpacked::<RANK, U_SIZE, V_SIZE, CT_SIZE>(
        params,
        t_as_ntt,
        &A_as_ntt,
        &m_prime,
        pseudorandomness,
    )?;

    // if c ≠ c′ then K′ ← K̃
    if ciphertext[..] == c_prime[..] {
        let mut k = [0u8; 32];
        k.copy_from_slice(success_shared_secret);
        Ok(k)
    } else {
        Ok(rejection_shared_secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameters::hash_functions::J;

    #[test]
    fn keygen_encaps_decaps_consistency() {
        let randomness = [42u8; 64];
        let (ek, dk) = generate_keypair::<
            3,
            { ML_KEM_768_EK_SIZE },
            { ML_KEM_768_DK_SIZE },
            { ML_KEM_768_DK_PKE_SIZE },
        >(&ML_KEM_768, &randomness)
        .unwrap();

        let m = [0xABu8; 32];
        let (shared_secret, ciphertext) = encapsulate::<
            3,
            { ML_KEM_768_EK_SIZE },
            { ML_KEM_768_U_SIZE },
            { ML_KEM_768_V_SIZE },
            { ML_KEM_768_CT_SIZE },
        >(&ML_KEM_768, &ek, &m)
        .unwrap();

        let shared_secret_decapsulated = decapsulate::<
            3,
            { ML_KEM_768_EK_SIZE },
            { ML_KEM_768_DK_SIZE },
            { ML_KEM_768_DK_PKE_SIZE },
            { ML_KEM_768_U_SIZE },
            { ML_KEM_768_V_SIZE },
            { ML_KEM_768_CT_SIZE },
            { ML_KEM_768_J_INPUT_SIZE },
        >(&ML_KEM_768, &dk, &ciphertext)
        .unwrap();

        assert_eq!(shared_secret, shared_secret_decapsulated);
    }

    #[test]
    fn modified_ciphertext_implicit_rejection() {
        let randomness = [1u8; 64];
        let (ek, dk) = generate_keypair::<
            3,
            { ML_KEM_768_EK_SIZE },
            { ML_KEM_768_DK_SIZE },
            { ML_KEM_768_DK_PKE_SIZE },
        >(&ML_KEM_768, &randomness)
        .unwrap();

        let m = [0x55u8; 32];
        let (shared_secret, mut ciphertext) = encapsulate::<
            3,
            { ML_KEM_768_EK_SIZE },
            { ML_KEM_768_U_SIZE },
            { ML_KEM_768_V_SIZE },
            { ML_KEM_768_CT_SIZE },
        >(&ML_KEM_768, &ek, &m)
        .unwrap();

        // Tamper with ciphertext
        ciphertext[0] ^= 0xFF;

        let shared_secret_decapsulated = decapsulate::<
            3,
            { ML_KEM_768_EK_SIZE },
            { ML_KEM_768_DK_SIZE },
            { ML_KEM_768_DK_PKE_SIZE },
            { ML_KEM_768_U_SIZE },
            { ML_KEM_768_V_SIZE },
            { ML_KEM_768_CT_SIZE },
            { ML_KEM_768_J_INPUT_SIZE },
        >(&ML_KEM_768, &dk, &ciphertext)
        .unwrap();

        assert_ne!(shared_secret, shared_secret_decapsulated);

        // Verify implicit rejection: K̃ = J(z ‖ c)
        let z = &dk[ML_KEM_768_DK_PKE_SIZE + ML_KEM_768_EK_SIZE + H_DIGEST_SIZE..];
        let mut j_input = [0u8; ML_KEM_768_J_INPUT_SIZE];
        j_input[..32].copy_from_slice(z);
        j_input[32..].copy_from_slice(&ciphertext);
        let expected_rejection: [u8; 32] = J(&j_input);
        assert_eq!(shared_secret_decapsulated, expected_rejection);
    }

    #[test]
    fn modified_secret_key() {
        let randomness = [3u8; 64];
        let (ek, mut dk) = generate_keypair::<
            3,
            { ML_KEM_768_EK_SIZE },
            { ML_KEM_768_DK_SIZE },
            { ML_KEM_768_DK_PKE_SIZE },
        >(&ML_KEM_768, &randomness)
        .unwrap();

        let m = [0x77u8; 32];
        let (shared_secret, ciphertext) = encapsulate::<
            3,
            { ML_KEM_768_EK_SIZE },
            { ML_KEM_768_U_SIZE },
            { ML_KEM_768_V_SIZE },
            { ML_KEM_768_CT_SIZE },
        >(&ML_KEM_768, &ek, &m)
        .unwrap();

        // Tamper with the secret key (not the z portion)
        dk[0] ^= 0xFF;

        let shared_secret_decapsulated = decapsulate::<
            3,
            { ML_KEM_768_EK_SIZE },
            { ML_KEM_768_DK_SIZE },
            { ML_KEM_768_DK_PKE_SIZE },
            { ML_KEM_768_U_SIZE },
            { ML_KEM_768_V_SIZE },
            { ML_KEM_768_CT_SIZE },
            { ML_KEM_768_J_INPUT_SIZE },
        >(&ML_KEM_768, &dk, &ciphertext)
        .unwrap();
        assert_ne!(shared_secret, shared_secret_decapsulated);
    }
}

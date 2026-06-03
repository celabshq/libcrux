#![cfg(feature = "incremental")]

//! # Incremental API.
//!
//! **WARNING:** This API is not standard compliant and may lead to insecure
//! usage. Use at your own risk.
#[allow(unused_imports)]
use hax_lib::prop::ToProp;

use crate::{
    constants::BITS_PER_RING_ELEMENT,
    hash_functions::Hash,
    ind_cca::{unpacked::MlKemPrivateKeyUnpacked, validate_public_key},
    ind_cpa::{self, unpacked::IndCpaPrivateKeyUnpacked},
    matrix::sample_matrix_A,
    mlkem::impl_incr_platform,
    polynomial::{vec_len_bytes, PolynomialRingElement},
    utils::into_padded_array,
    variant, vector, SHARED_SECRET_SIZE,
};

use super::{
    unpacked::{encaps_prepare, MlKemKeyPairUnpacked, MlKemPublicKeyUnpacked},
    MlKemSharedSecret, Operations, KEY_GENERATION_SEED_SIZE,
};

/// Key and state types.
pub mod types;
pub(crate) use types::*;

// Platform instantiations

#[cfg(feature = "simd256")]
pub(crate) mod avx2 {
    use super::*;

    impl_incr_platform!(
        vector::SIMD256Vector,
        crate::hash_functions::avx2::Simd256Hash,
        unsafe,
        #[cfg_attr(not(hax), target_feature(enable = "avx2"))],
        #[allow(unsafe_code)]
    );
}
#[cfg(all(feature = "simd128", not(hax)))]
pub(crate) mod neon {
    use super::*;

    impl_incr_platform!(
        vector::SIMD128Vector,
        crate::hash_functions::neon::Simd128Hash
    );
}
pub(crate) mod portable {
    use super::*;

    impl_incr_platform!(
        vector::portable::PortableVector,
        crate::hash_functions::portable::PortableHash<K>
    );
}

/// Multiplexing incremental API.
///
/// Note that this requires alloc support and is not `no_std` compatible
pub(crate) mod multiplexing;

/// Generate a key pair for incremental encapsulation.
///
/// This generates a regular unpacked key pair [`MlKemKeyPairUnpacked`].
/// The two parts of the public key can be extracted with [`pk1`] and [`pk2`].
///
/// To [`decapsulate`], the entire key pair is used again.
#[inline(always)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && ETA1 == hacspec_ml_kem::parameters::eta1(K)
    && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
    && PUBLIC_KEY_SIZE == hacspec_ml_kem::parameters::cpa_public_key_size(K)
)]
pub(crate) fn generate_keypair<
    const K: usize,
    const CPA_PRIVATE_KEY_SIZE: usize,
    const PRIVATE_KEY_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    randomness: [u8; KEY_GENERATION_SEED_SIZE],
) -> MlKemKeyPairUnpacked<K, Vector> {
    // Generate unpacked key pair.
    let mut kp = MlKemKeyPairUnpacked::new();
    super::unpacked::generate_keypair::<
        K,
        CPA_PRIVATE_KEY_SIZE,
        PRIVATE_KEY_SIZE,
        PUBLIC_KEY_SIZE,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        Vector,
        Hasher,
        variant::MlKem,
    >(randomness, &mut kp);

    kp
}

/// Generate a key pair for incremental encapsulation.
///
/// This generates a regular key pair and writes
/// it into the `key_pair` output bytes.
///
/// The public keys can be extracted from the bytes.
#[inline(always)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && ETA1 == hacspec_ml_kem::parameters::eta1(K)
    && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
    && PUBLIC_KEY_SIZE == hacspec_ml_kem::parameters::cpa_public_key_size(K)
)]
pub(crate) fn generate_keypair_compressed<
    const K: usize,
    const PK2_LEN: usize,
    const CPA_PRIVATE_KEY_SIZE: usize,
    const PRIVATE_KEY_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    const KEYPAIR_LEN: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    randomness: [u8; KEY_GENERATION_SEED_SIZE],
    key_pair: &mut [u8; KEYPAIR_LEN],
) {
    // Generate unpacked key pair.
    let mut kp = MlKemKeyPairUnpacked::new();
    super::unpacked::generate_keypair::<
        K,
        CPA_PRIVATE_KEY_SIZE,
        PRIVATE_KEY_SIZE,
        PUBLIC_KEY_SIZE,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        Vector,
        Hasher,
        variant::MlKem,
    >(randomness, &mut kp);

    let kp = KeyPair::<K, PK2_LEN, Vector>::from(kp);
    kp.to_bytes_compressed::<KEYPAIR_LEN, CPA_PRIVATE_KEY_SIZE>(key_pair);
}

/// Generate a key pair for incremental encapsulation.
///
/// This generates a regular unpacked key pair [`MlKemKeyPairUnpacked`] and writes
/// it into the `key_pair` output bytes.
///
/// The public keys can be extracted from the bytes TODO.
#[inline(always)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && ETA1 == hacspec_ml_kem::parameters::eta1(K)
    && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
    && PUBLIC_KEY_SIZE == hacspec_ml_kem::parameters::cpa_public_key_size(K)
    && PK2_LEN <= 1536
    && key_pair.len() >= 64 + PK2_LEN + K * 512 + 32 + K * K * 512
)]
#[hax_lib::ensures(|result|
    result.is_ok() && future(key_pair).len() == key_pair.len())]
pub(crate) fn generate_keypair_serialized<
    const K: usize,
    const PK2_LEN: usize,
    const CPA_PRIVATE_KEY_SIZE: usize,
    const PRIVATE_KEY_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    randomness: [u8; KEY_GENERATION_SEED_SIZE],
    key_pair: &mut [u8],
) -> Result<(), Error> {
    // Generate unpacked key pair.
    let mut kp = MlKemKeyPairUnpacked::new();
    super::unpacked::generate_keypair::<
        K,
        CPA_PRIVATE_KEY_SIZE,
        PRIVATE_KEY_SIZE,
        PUBLIC_KEY_SIZE,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        Vector,
        Hasher,
        variant::MlKem,
    >(randomness, &mut kp);

    let kp = KeyPair::<K, PK2_LEN, Vector>::from(kp);
    kp.to_bytes(key_pair)
}

#[inline(always)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && ETA1 == hacspec_ml_kem::parameters::eta1(K)
    && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
    && ETA2 == hacspec_ml_kem::parameters::eta2(K)
    && ETA2_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta2_randomness_size(K)
    && C1_SIZE == hacspec_ml_kem::parameters::c1_size(K)
    && VECTOR_U_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_u_compression_factor(K)
    && VECTOR_U_BLOCK_LEN == hacspec_ml_kem::parameters::c1_block_size(K)
)]
#[hax_lib::ensures(|(_ct1, state, _ss)|
    crate::polynomial::spec::is_bounded_polynomial_vector(3328, &state.r_as_ntt)
    & crate::polynomial::spec::is_bounded_poly(3328, &state.error2)
)]
pub(crate) fn encapsulate1<
    const K: usize,
    const CIPHERTEXT_SIZE: usize,
    const C1_SIZE: usize,
    const VECTOR_U_COMPRESSION_FACTOR: usize,
    const VECTOR_U_BLOCK_LEN: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    pk1: &PublicKey1,
    randomness: [u8; SHARED_SECRET_SIZE],
) -> (Ciphertext1<C1_SIZE>, EncapsState<K, Vector>, [u8; 32]) {
    let hashed = encaps_prepare::<K, Hasher>(&randomness, &pk1.hash);
    let (shared_secret, pseudorandomness) = hashed.split_at(SHARED_SECRET_SIZE);

    // Rebuild the matrix A from the seed
    let mut matrix = [[PolynomialRingElement::<Vector>::ZERO(); K]; K];
    sample_matrix_A::<K, Vector, Hasher>(&mut matrix, &into_padded_array(&pk1.seed), false);

    // Fold sample_matrix_A's nested per-element forall ensures into the
    // opaque `is_bounded_polynomial_matrix 3328 matrix` atom required by
    // encrypt_c1 (same producer-site folding as in ind_cpa.rs).
    hax_lib::fstar!(
        r#"let folded (i: usize{ v i < v $K }) : Lemma
            (Libcrux_ml_kem.Polynomial.Spec.is_bounded_polynomial_vector $K #$:Vector (sz 3328)
                (${matrix}.[ i ] <: t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement $:Vector) $K)) =
            Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_vector_intro $K #$:Vector
                (${matrix}.[ i ] <: t_Array (Libcrux_ml_kem.Vector.t_PolynomialRingElement $:Vector) $K)
                (sz 3328)
        in
        Classical.forall_intro folded;
        Libcrux_ml_kem.Polynomial.Spec.lemma_is_bounded_polynomial_matrix_intro $K #$:Vector
            $matrix (sz 3328)"#
    );

    let mut ciphertext = [0u8; C1_SIZE];
    let (r_as_ntt, error2) = ind_cpa::encrypt_c1::<
        K,
        C1_SIZE,
        VECTOR_U_COMPRESSION_FACTOR,
        VECTOR_U_BLOCK_LEN,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        ETA2,
        ETA2_RANDOMNESS_SIZE,
        Vector,
        Hasher,
    >(&pseudorandomness, &matrix, &mut ciphertext);

    let state = EncapsState {
        randomness,
        r_as_ntt,
        error2,
    };
    (
        Ciphertext1 { value: ciphertext },
        state,
        shared_secret.try_into().unwrap(),
    )
}

#[inline(always)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && ETA1 == hacspec_ml_kem::parameters::eta1(K)
    && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
    && ETA2 == hacspec_ml_kem::parameters::eta2(K)
    && ETA2_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta2_randomness_size(K)
    && C1_SIZE == hacspec_ml_kem::parameters::c1_size(K)
    && VECTOR_U_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_u_compression_factor(K)
    && VECTOR_U_BLOCK_LEN == hacspec_ml_kem::parameters::c1_block_size(K)
    && state.len() >= K * 512 + 512 + 32
    && shared_secret.len() >= 32
)]
#[hax_lib::ensures(|result|
    result.is_ok()
    && future(state).len() == state.len()
    && future(shared_secret).len() == shared_secret.len())]
pub(crate) fn encapsulate1_serialized<
    const K: usize,
    const CIPHERTEXT_SIZE: usize,
    const C1_SIZE: usize,
    const VECTOR_U_COMPRESSION_FACTOR: usize,
    const VECTOR_U_BLOCK_LEN: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    pk1: &PublicKey1,
    randomness: [u8; SHARED_SECRET_SIZE],
    state: &mut [u8],
    shared_secret: &mut [u8],
) -> Result<Ciphertext1<C1_SIZE>, Error> {
    debug_assert!(shared_secret.len() >= SHARED_SECRET_SIZE);
    if shared_secret.len() < SHARED_SECRET_SIZE {
        return Err(Error::InvalidOutputLength);
    }

    let (ct1, encaps_state, ss) = encapsulate1::<
        K,
        CIPHERTEXT_SIZE,
        C1_SIZE,
        VECTOR_U_COMPRESSION_FACTOR,
        VECTOR_U_BLOCK_LEN,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        ETA2,
        ETA2_RANDOMNESS_SIZE,
        Vector,
        Hasher,
    >(pk1, randomness);

    // Write out the state
    encaps_state.to_bytes(state)?;
    shared_secret[..SHARED_SECRET_SIZE].copy_from_slice(&ss);

    // Return the ciphertext
    Ok(ct1)
}

/// Check that the pk1 and pk2 parts are consistent.
#[inline(always)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && PK_LEN == hacspec_ml_kem::parameters::cpa_public_key_size(K)
)]
pub(crate) fn validate_pk<
    const K: usize,
    const PK_LEN: usize,
    Hasher: Hash<K>,
    Vector: Operations,
>(
    pk1: &PublicKey1,
    pk2: &[u8],
) -> Result<(), Error> {
    let pk2_len = K * BITS_PER_RING_ELEMENT / 8;
    if pk2.len() != pk2_len {
        return Err(Error::InvalidInputLength);
    }

    validate_pk_parts::<K, PK_LEN, Hasher, Vector>(&pk1.seed, &pk1.hash, pk2)
}

/// Check that the pk1 and pk2 parts are consistent.
#[inline(always)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && PK_LEN == hacspec_ml_kem::parameters::cpa_public_key_size(K)
)]
pub(crate) fn validate_pk_bytes<
    const K: usize,
    const PK_LEN: usize,
    Hasher: Hash<K>,
    Vector: Operations,
>(
    pk1: &[u8],
    pk2: &[u8],
) -> Result<(), Error> {
    let pk2_len = K * BITS_PER_RING_ELEMENT / 8;
    if pk1.len() != 64 || pk2.len() != pk2_len {
        return Err(Error::InvalidInputLength);
    }

    validate_pk_parts::<K, PK_LEN, Hasher, Vector>(&pk1[0..32], &pk1[32..], pk2)
}

#[inline(always)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && PK_LEN == hacspec_ml_kem::parameters::cpa_public_key_size(K)
    && pk1_seed.len() == 32
    && pk1_hash.len() == 32
    && pk2.len() == K * BITS_PER_RING_ELEMENT / 8
)]
fn validate_pk_parts<const K: usize, const PK_LEN: usize, Hasher: Hash<K>, Vector: Operations>(
    pk1_seed: &[u8],
    pk1_hash: &[u8],
    pk2: &[u8],
) -> Result<(), Error> {
    // Build the full public key: t || 𝜌
    let mut pk = [0u8; PK_LEN];
    let pk2_len = K * BITS_PER_RING_ELEMENT / 8;
    debug_assert!(pk2_len == pk2.len());

    pk[0..pk2_len].copy_from_slice(&pk2);
    pk[pk2_len..].copy_from_slice(&pk1_seed);

    let hash = Hasher::H(&pk);
    if hash != pk1_hash {
        return Err(Error::InvalidPublicKey);
    }

    // Check the domain of t
    if !validate_public_key::<K, PK_LEN, Vector>(&pk) {
        return Err(Error::InvalidPublicKey);
    }

    Ok(())
}

#[inline(always)]
#[hax_lib::requires(
    (hacspec_ml_kem::parameters::is_rank(K)
    && PK2_LEN == hacspec_ml_kem::parameters::cpa_private_key_size(K)
    && C2_SIZE == hacspec_ml_kem::parameters::c2_size(K)
    && VECTOR_V_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_v_compression_factor(K))
        .to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &state.r_as_ntt)
    & crate::polynomial::spec::is_bounded_poly(3328, &state.error2)
)]
pub(crate) fn encapsulate2<
    const K: usize,
    const PK2_LEN: usize,
    const C2_SIZE: usize,
    const VECTOR_V_COMPRESSION_FACTOR: usize,
    Vector: Operations,
>(
    state: &EncapsState<K, Vector>,
    pk2: &PublicKey2<PK2_LEN>,
) -> Ciphertext2<C2_SIZE> {
    let mut ciphertext = [0u8; C2_SIZE];
    let t_as_ntt = pk2.deserialize();

    ind_cpa::encrypt_c2::<K, VECTOR_V_COMPRESSION_FACTOR, C2_SIZE, Vector>(
        &t_as_ntt,
        &state.r_as_ntt,
        &state.error2,
        &state.randomness,
        &mut ciphertext,
    );

    Ciphertext2 { value: ciphertext }
}

/// Encapsulate the second part of the ciphertext from a serialized state.
///
/// The state bytes are validated on decode: [`Error::InvalidInput`] is
/// returned if any decoded coefficient is out of field range.
#[inline(always)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && PK2_LEN == hacspec_ml_kem::parameters::cpa_private_key_size(K)
    && C2_SIZE == hacspec_ml_kem::parameters::c2_size(K)
    && VECTOR_V_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_v_compression_factor(K)
    && STATE_LEN >= K * 512 + 512 + 32
)]
pub(crate) fn encapsulate2_serialized<
    const K: usize,
    const PK2_LEN: usize,
    const C2_SIZE: usize,
    const VECTOR_V_COMPRESSION_FACTOR: usize,
    const STATE_LEN: usize,
    Vector: Operations,
>(
    state: &[u8; STATE_LEN],
    public_key_part: &PublicKey2<PK2_LEN>,
) -> Result<Ciphertext2<C2_SIZE>, Error> {
    let state = EncapsState::try_from_bytes(state)?;

    Ok(encapsulate2::<
        K,
        PK2_LEN,
        C2_SIZE,
        VECTOR_V_COMPRESSION_FACTOR,
        Vector,
    >(&state, public_key_part))
}

#[inline(always)]
#[hax_lib::requires(
    (hacspec_ml_kem::parameters::is_rank(K)
    && ETA1 == hacspec_ml_kem::parameters::eta1(K)
    && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
    && ETA2 == hacspec_ml_kem::parameters::eta2(K)
    && ETA2_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta2_randomness_size(K)
    && C1_SIZE == hacspec_ml_kem::parameters::c1_size(K)
    && C2_SIZE == hacspec_ml_kem::parameters::c2_size(K)
    && VECTOR_U_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_u_compression_factor(K)
    && VECTOR_V_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_v_compression_factor(K)
    && C1_BLOCK_SIZE == hacspec_ml_kem::parameters::c1_block_size(K)
    && CIPHERTEXT_SIZE == hacspec_ml_kem::parameters::cpa_ciphertext_size(K)
    && IMPLICIT_REJECTION_HASH_INPUT_SIZE
        == hacspec_ml_kem::parameters::implicit_rejection_hash_input_size(K))
        .to_prop()
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &private_key.private_key.ind_cpa_private_key.secret_as_ntt)
    & crate::polynomial::spec::is_bounded_polynomial_matrix(3328, &private_key.public_key.ind_cpa_public_key.A)
    & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &private_key.public_key.ind_cpa_public_key.t_as_ntt)
)]
pub(crate) fn decapsulate<
    const K: usize,
    const SECRET_KEY_SIZE: usize,
    const CPA_SECRET_KEY_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const CIPHERTEXT_SIZE: usize,
    const T_AS_NTT_ENCODED_SIZE: usize,
    const C1_SIZE: usize,
    const C2_SIZE: usize,
    const VECTOR_U_COMPRESSION_FACTOR: usize,
    const VECTOR_V_COMPRESSION_FACTOR: usize,
    const C1_BLOCK_SIZE: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
    const IMPLICIT_REJECTION_HASH_INPUT_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    private_key: &MlKemKeyPairUnpacked<K, Vector>,
    ciphertext1: &Ciphertext1<C1_SIZE>,
    ciphertext2: &Ciphertext2<C2_SIZE>,
) -> MlKemSharedSecret {
    let mut ciphertext = [0u8; CIPHERTEXT_SIZE];
    ciphertext[..C1_SIZE].copy_from_slice(&ciphertext1.value);
    ciphertext[C1_SIZE..].copy_from_slice(&ciphertext2.value);
    crate::ind_cca::unpacked::decapsulate::<
        K,
        SECRET_KEY_SIZE,
        CPA_SECRET_KEY_SIZE,
        PUBLIC_KEY_SIZE,
        CIPHERTEXT_SIZE,
        T_AS_NTT_ENCODED_SIZE,
        C1_SIZE,
        C2_SIZE,
        VECTOR_U_COMPRESSION_FACTOR,
        VECTOR_V_COMPRESSION_FACTOR,
        C1_BLOCK_SIZE,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        ETA2,
        ETA2_RANDOMNESS_SIZE,
        IMPLICIT_REJECTION_HASH_INPUT_SIZE,
        Vector,
        Hasher,
    >(private_key, &ciphertext.into())
}

/// Decapsulate with a serialized incremental key pair.
///
/// The key bytes are validated on decode: [`Error::InvalidInput`] is
/// returned if any decoded coefficient is out of field range.
#[inline(always)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && PK2_LEN == hacspec_ml_kem::parameters::cpa_private_key_size(K)
    && ETA1 == hacspec_ml_kem::parameters::eta1(K)
    && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
    && ETA2 == hacspec_ml_kem::parameters::eta2(K)
    && ETA2_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta2_randomness_size(K)
    && C1_SIZE == hacspec_ml_kem::parameters::c1_size(K)
    && C2_SIZE == hacspec_ml_kem::parameters::c2_size(K)
    && VECTOR_U_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_u_compression_factor(K)
    && VECTOR_V_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_v_compression_factor(K)
    && C1_BLOCK_SIZE == hacspec_ml_kem::parameters::c1_block_size(K)
    && CIPHERTEXT_SIZE == hacspec_ml_kem::parameters::cpa_ciphertext_size(K)
    && IMPLICIT_REJECTION_HASH_INPUT_SIZE
        == hacspec_ml_kem::parameters::implicit_rejection_hash_input_size(K)
    && key.len() >= 64 + PK2_LEN + K * 512 + 32 + K * K * 512
)]
pub(crate) fn decapsulate_incremental_key<
    const K: usize,
    const PK2_LEN: usize,
    const SECRET_KEY_SIZE: usize,
    const CPA_SECRET_KEY_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const CIPHERTEXT_SIZE: usize,
    const T_AS_NTT_ENCODED_SIZE: usize,
    const C1_SIZE: usize,
    const C2_SIZE: usize,
    const VECTOR_U_COMPRESSION_FACTOR: usize,
    const VECTOR_V_COMPRESSION_FACTOR: usize,
    const C1_BLOCK_SIZE: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
    const IMPLICIT_REJECTION_HASH_INPUT_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    key: &[u8],
    ciphertext1: &Ciphertext1<C1_SIZE>,
    ciphertext2: &Ciphertext2<C2_SIZE>,
) -> Result<MlKemSharedSecret, Error> {
    // Build an unpacked key pair from the input bytes.
    let key_pair: KeyPair<K, PK2_LEN, Vector> = KeyPair::from_bytes(key)?;
    let key_pair = key_pair.into_unpacked();

    let mut ciphertext = [0u8; CIPHERTEXT_SIZE];
    ciphertext[..C1_SIZE].copy_from_slice(&ciphertext1.value);
    ciphertext[C1_SIZE..].copy_from_slice(&ciphertext2.value);

    Ok(crate::ind_cca::unpacked::decapsulate::<
        K,
        SECRET_KEY_SIZE,
        CPA_SECRET_KEY_SIZE,
        PUBLIC_KEY_SIZE,
        CIPHERTEXT_SIZE,
        T_AS_NTT_ENCODED_SIZE,
        C1_SIZE,
        C2_SIZE,
        VECTOR_U_COMPRESSION_FACTOR,
        VECTOR_V_COMPRESSION_FACTOR,
        C1_BLOCK_SIZE,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        ETA2,
        ETA2_RANDOMNESS_SIZE,
        IMPLICIT_REJECTION_HASH_INPUT_SIZE,
        Vector,
        Hasher,
    >(&key_pair, &ciphertext.into()))
}

#[inline(always)]
#[hax_lib::requires(
    hacspec_ml_kem::parameters::is_rank(K)
    && SECRET_KEY_SIZE == hacspec_ml_kem::parameters::cca_private_key_size(K)
    && CPA_SECRET_KEY_SIZE == hacspec_ml_kem::parameters::cpa_private_key_size(K)
    && PUBLIC_KEY_SIZE == hacspec_ml_kem::parameters::cpa_public_key_size(K)
    && CIPHERTEXT_SIZE == hacspec_ml_kem::parameters::cpa_ciphertext_size(K)
    && T_AS_NTT_ENCODED_SIZE == hacspec_ml_kem::parameters::t_as_ntt_encoded_size(K)
    && C1_SIZE == hacspec_ml_kem::parameters::c1_size(K)
    && C2_SIZE == hacspec_ml_kem::parameters::c2_size(K)
    && VECTOR_U_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_u_compression_factor(K)
    && VECTOR_V_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_v_compression_factor(K)
    && C1_BLOCK_SIZE == hacspec_ml_kem::parameters::c1_block_size(K)
    && ETA1 == hacspec_ml_kem::parameters::eta1(K)
    && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
    && ETA2 == hacspec_ml_kem::parameters::eta2(K)
    && ETA2_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta2_randomness_size(K)
    && IMPLICIT_REJECTION_HASH_INPUT_SIZE == hacspec_ml_kem::parameters::implicit_rejection_hash_input_size(K)
)]
pub(crate) fn decapsulate_compressed_key<
    const K: usize,
    const PK2_LEN: usize,
    const SECRET_KEY_SIZE: usize,
    const CPA_SECRET_KEY_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const CIPHERTEXT_SIZE: usize,
    const T_AS_NTT_ENCODED_SIZE: usize,
    const C1_SIZE: usize,
    const C2_SIZE: usize,
    const VECTOR_U_COMPRESSION_FACTOR: usize,
    const VECTOR_V_COMPRESSION_FACTOR: usize,
    const C1_BLOCK_SIZE: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
    const IMPLICIT_REJECTION_HASH_INPUT_SIZE: usize,
    Vector: Operations,
    Hasher: Hash<K>,
>(
    private_key: &[u8; SECRET_KEY_SIZE],
    ciphertext1: &Ciphertext1<C1_SIZE>,
    ciphertext2: &Ciphertext2<C2_SIZE>,
) -> MlKemSharedSecret {
    let mut ciphertext = [0u8; CIPHERTEXT_SIZE];
    ciphertext[..C1_SIZE].copy_from_slice(&ciphertext1.value);
    ciphertext[C1_SIZE..].copy_from_slice(&ciphertext2.value);

    crate::ind_cca::decapsulate::<
        K,
        SECRET_KEY_SIZE,
        CPA_SECRET_KEY_SIZE,
        PUBLIC_KEY_SIZE,
        CIPHERTEXT_SIZE,
        T_AS_NTT_ENCODED_SIZE,
        C1_SIZE,
        C2_SIZE,
        VECTOR_U_COMPRESSION_FACTOR,
        VECTOR_V_COMPRESSION_FACTOR,
        C1_BLOCK_SIZE,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        ETA2,
        ETA2_RANDOMNESS_SIZE,
        IMPLICIT_REJECTION_HASH_INPUT_SIZE,
        Vector,
        Hasher,
        variant::MlKem,
    >(&private_key.into(), &ciphertext.into())
}

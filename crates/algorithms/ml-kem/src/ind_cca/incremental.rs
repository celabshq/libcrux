#![cfg(feature = "incremental")]

//! # Incremental API.
//!
//! **WARNING:** This API is not standard compliant and may lead to insecure
//! usage. Use at your own risk.
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
#[cfg(feature = "simd128")]
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

#[inline(always)]
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
) -> Ciphertext2<C2_SIZE> {
    let state = EncapsState::from_bytes(state);

    encapsulate2::<K, PK2_LEN, C2_SIZE, VECTOR_V_COMPRESSION_FACTOR, Vector>(
        &state,
        public_key_part,
    )
}

#[inline(always)]
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

#[inline(always)]
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
    >(&key_pair.into(), &ciphertext.into()))
}

#[inline(always)]
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

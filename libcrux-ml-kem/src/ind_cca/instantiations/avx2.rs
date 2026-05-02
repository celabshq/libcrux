use crate::{
    MlKemCiphertext, MlKemKeyPair, MlKemPrivateKey, MlKemPublicKey, MlKemSharedSecret,
    KEY_GENERATION_SEED_SIZE, SHARED_SECRET_SIZE,
};

#[allow(unsafe_code)]
/// Portable generate key pair.
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $CPA_PRIVATE_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_private_key_size $K /\
    $PRIVATE_KEY_SIZE == Hacspec_ml_kem.Parameters.cca_private_key_size $K /\
    $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K /\
    $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
    $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K"#))]
unsafe fn generate_keypair_avx2<
    const K: usize,
    const CPA_PRIVATE_KEY_SIZE: usize,
    const PRIVATE_KEY_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
>(
    randomness: &[u8; KEY_GENERATION_SEED_SIZE],
) -> MlKemKeyPair<PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE> {
    crate::ind_cca::generate_keypair::<
        K,
        CPA_PRIVATE_KEY_SIZE,
        PRIVATE_KEY_SIZE,
        PUBLIC_KEY_SIZE,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        crate::vector::SIMD256Vector,
        crate::hash_functions::avx2::Simd256Hash,
        crate::variant::MlKem,
    >(randomness)
}

#[allow(unsafe_code)]
#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $CPA_PRIVATE_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_private_key_size $K /\
    $PRIVATE_KEY_SIZE == Hacspec_ml_kem.Parameters.cca_private_key_size $K /\
    $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K /\
    $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
    $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K"#))]
pub(crate) fn generate_keypair<
    const K: usize,
    const CPA_PRIVATE_KEY_SIZE: usize,
    const PRIVATE_KEY_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
>(
    randomness: &[u8; KEY_GENERATION_SEED_SIZE],
) -> MlKemKeyPair<PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE> {
    unsafe {
        generate_keypair_avx2::<
            K,
            CPA_PRIVATE_KEY_SIZE,
            PRIVATE_KEY_SIZE,
            PUBLIC_KEY_SIZE,
            ETA1,
            ETA1_RANDOMNESS_SIZE,
        >(randomness)
    }
}

#[allow(unsafe_code)]
#[cfg(feature = "kyber")]
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
unsafe fn kyber_generate_keypair_avx2<
    const K: usize,
    const CPA_PRIVATE_KEY_SIZE: usize,
    const PRIVATE_KEY_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
>(
    randomness: &[u8; KEY_GENERATION_SEED_SIZE],
) -> MlKemKeyPair<PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE> {
    crate::ind_cca::generate_keypair::<
        K,
        CPA_PRIVATE_KEY_SIZE,
        PRIVATE_KEY_SIZE,
        PUBLIC_KEY_SIZE,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        crate::vector::SIMD256Vector,
        crate::hash_functions::avx2::Simd256Hash,
        crate::variant::Kyber,
    >(randomness)
}

#[allow(unsafe_code)]
#[cfg(feature = "kyber")]
pub(crate) fn kyber_generate_keypair<
    const K: usize,
    const CPA_PRIVATE_KEY_SIZE: usize,
    const PRIVATE_KEY_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
>(
    randomness: &[u8; KEY_GENERATION_SEED_SIZE],
) -> MlKemKeyPair<PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE> {
    unsafe {
        kyber_generate_keypair_avx2::<
            K,
            CPA_PRIVATE_KEY_SIZE,
            PRIVATE_KEY_SIZE,
            PUBLIC_KEY_SIZE,
            ETA1,
            ETA1_RANDOMNESS_SIZE,
        >(randomness)
    }
}

#[allow(unsafe_code)]
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K"#))]
unsafe fn validate_public_key_avx2<const K: usize, const PUBLIC_KEY_SIZE: usize>(
    public_key: &[u8; PUBLIC_KEY_SIZE],
) -> bool {
    crate::ind_cca::validate_public_key::<K, PUBLIC_KEY_SIZE, crate::vector::SIMD256Vector>(
        public_key,
    )
}

#[allow(unsafe_code)]
#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K"#))]
pub(crate) fn validate_public_key<const K: usize, const PUBLIC_KEY_SIZE: usize>(
    public_key: &[u8; PUBLIC_KEY_SIZE],
) -> bool {
    unsafe { validate_public_key_avx2::<K, PUBLIC_KEY_SIZE>(public_key) }
}

#[allow(unsafe_code)]
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $SECRET_KEY_SIZE == Hacspec_ml_kem.Parameters.cca_private_key_size $K /\
    $CIPHERTEXT_SIZE == Hacspec_ml_kem.Parameters.cpa_ciphertext_size $K"#))]
unsafe fn validate_private_key_avx2<
    const K: usize,
    const SECRET_KEY_SIZE: usize,
    const CIPHERTEXT_SIZE: usize,
>(
    private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
    ciphertext: &MlKemCiphertext<CIPHERTEXT_SIZE>,
) -> bool {
    crate::ind_cca::validate_private_key::<
        K,
        SECRET_KEY_SIZE,
        CIPHERTEXT_SIZE,
        crate::hash_functions::avx2::Simd256Hash,
    >(private_key, ciphertext)
}

#[allow(unsafe_code)]
#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $SECRET_KEY_SIZE == Hacspec_ml_kem.Parameters.cca_private_key_size $K /\
    $CIPHERTEXT_SIZE == Hacspec_ml_kem.Parameters.cpa_ciphertext_size $K"#))]
pub(crate) fn validate_private_key<
    const K: usize,
    const SECRET_KEY_SIZE: usize,
    const CIPHERTEXT_SIZE: usize,
>(
    private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
    ciphertext: &MlKemCiphertext<CIPHERTEXT_SIZE>,
) -> bool {
    unsafe {
        validate_private_key_avx2::<K, SECRET_KEY_SIZE, CIPHERTEXT_SIZE>(private_key, ciphertext)
    }
}

/// Private key validation
#[inline(always)]
#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $SECRET_KEY_SIZE == Hacspec_ml_kem.Parameters.cca_private_key_size $K"#))]
pub(crate) fn validate_private_key_only<const K: usize, const SECRET_KEY_SIZE: usize>(
    private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
) -> bool {
    crate::ind_cca::validate_private_key_only::<
        K,
        SECRET_KEY_SIZE,
        crate::hash_functions::avx2::Simd256Hash,
    >(private_key)
}

#[allow(unsafe_code)]
#[cfg(feature = "kyber")]
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
unsafe fn kyber_encapsulate_avx2<
    const K: usize,
    const CIPHERTEXT_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const T_AS_NTT_ENCODED_SIZE: usize,
    const C1_SIZE: usize,
    const C2_SIZE: usize,
    const VECTOR_U_COMPRESSION_FACTOR: usize,
    const VECTOR_V_COMPRESSION_FACTOR: usize,
    const VECTOR_U_BLOCK_LEN: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
>(
    public_key: &MlKemPublicKey<PUBLIC_KEY_SIZE>,
    randomness: &[u8; SHARED_SECRET_SIZE],
) -> (MlKemCiphertext<CIPHERTEXT_SIZE>, MlKemSharedSecret) {
    crate::ind_cca::encapsulate::<
        K,
        CIPHERTEXT_SIZE,
        PUBLIC_KEY_SIZE,
        T_AS_NTT_ENCODED_SIZE,
        C1_SIZE,
        C2_SIZE,
        VECTOR_U_COMPRESSION_FACTOR,
        VECTOR_V_COMPRESSION_FACTOR,
        VECTOR_U_BLOCK_LEN,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        ETA2,
        ETA2_RANDOMNESS_SIZE,
        crate::vector::SIMD256Vector,
        crate::hash_functions::avx2::Simd256Hash,
        crate::variant::Kyber,
    >(public_key, randomness)
}

#[allow(unsafe_code)]
#[cfg(feature = "kyber")]
pub(crate) fn kyber_encapsulate<
    const K: usize,
    const CIPHERTEXT_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const T_AS_NTT_ENCODED_SIZE: usize,
    const C1_SIZE: usize,
    const C2_SIZE: usize,
    const VECTOR_U_COMPRESSION_FACTOR: usize,
    const VECTOR_V_COMPRESSION_FACTOR: usize,
    const VECTOR_U_BLOCK_LEN: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
>(
    public_key: &MlKemPublicKey<PUBLIC_KEY_SIZE>,
    randomness: &[u8; SHARED_SECRET_SIZE],
) -> (MlKemCiphertext<CIPHERTEXT_SIZE>, MlKemSharedSecret) {
    unsafe {
        kyber_encapsulate_avx2::<
            K,
            CIPHERTEXT_SIZE,
            PUBLIC_KEY_SIZE,
            T_AS_NTT_ENCODED_SIZE,
            C1_SIZE,
            C2_SIZE,
            VECTOR_U_COMPRESSION_FACTOR,
            VECTOR_V_COMPRESSION_FACTOR,
            VECTOR_U_BLOCK_LEN,
            ETA1,
            ETA1_RANDOMNESS_SIZE,
            ETA2,
            ETA2_RANDOMNESS_SIZE,
        >(public_key, randomness)
    }
}

#[allow(unsafe_code)]
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $CIPHERTEXT_SIZE == Hacspec_ml_kem.Parameters.cpa_ciphertext_size $K /\
    $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K /\
    $T_AS_NTT_ENCODED_SIZE == Hacspec_ml_kem.Parameters.tt_as_ntt_encoded_size $K /\
    $C1_SIZE == Hacspec_ml_kem.Parameters.c1_size $K /\
    $C2_SIZE == Hacspec_ml_kem.Parameters.c2_size $K /\
    $VECTOR_U_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_u_compression_factor  $K /\
    $VECTOR_V_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_v_compression_factor  $K /\
    $VECTOR_U_BLOCK_LEN == Hacspec_ml_kem.Parameters.c1_block_size $K /\
    $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
    $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K /\
    $ETA2 == Hacspec_ml_kem.Parameters.eta2 $K /\
    $ETA2_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta2_randomness_size $K"#))]
unsafe fn encapsulate_avx2<
    const K: usize,
    const CIPHERTEXT_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const T_AS_NTT_ENCODED_SIZE: usize,
    const C1_SIZE: usize,
    const C2_SIZE: usize,
    const VECTOR_U_COMPRESSION_FACTOR: usize,
    const VECTOR_V_COMPRESSION_FACTOR: usize,
    const VECTOR_U_BLOCK_LEN: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
>(
    public_key: &MlKemPublicKey<PUBLIC_KEY_SIZE>,
    randomness: &[u8; SHARED_SECRET_SIZE],
) -> (MlKemCiphertext<CIPHERTEXT_SIZE>, MlKemSharedSecret) {
    crate::ind_cca::encapsulate::<
        K,
        CIPHERTEXT_SIZE,
        PUBLIC_KEY_SIZE,
        T_AS_NTT_ENCODED_SIZE,
        C1_SIZE,
        C2_SIZE,
        VECTOR_U_COMPRESSION_FACTOR,
        VECTOR_V_COMPRESSION_FACTOR,
        VECTOR_U_BLOCK_LEN,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        ETA2,
        ETA2_RANDOMNESS_SIZE,
        crate::vector::SIMD256Vector,
        crate::hash_functions::avx2::Simd256Hash,
        crate::variant::MlKem,
    >(public_key, randomness)
}

#[allow(unsafe_code)]
#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $CIPHERTEXT_SIZE == Hacspec_ml_kem.Parameters.cpa_ciphertext_size $K /\
    $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K /\
    $T_AS_NTT_ENCODED_SIZE == Hacspec_ml_kem.Parameters.tt_as_ntt_encoded_size $K /\
    $C1_SIZE == Hacspec_ml_kem.Parameters.c1_size $K /\
    $C2_SIZE == Hacspec_ml_kem.Parameters.c2_size $K /\
    $VECTOR_U_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_u_compression_factor  $K /\
    $VECTOR_V_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_v_compression_factor  $K /\
    $VECTOR_U_BLOCK_LEN == Hacspec_ml_kem.Parameters.c1_block_size $K /\
    $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
    $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K /\
    $ETA2 == Hacspec_ml_kem.Parameters.eta2 $K /\
    $ETA2_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta2_randomness_size $K"#))]
pub(crate) fn encapsulate<
    const K: usize,
    const CIPHERTEXT_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const T_AS_NTT_ENCODED_SIZE: usize,
    const C1_SIZE: usize,
    const C2_SIZE: usize,
    const VECTOR_U_COMPRESSION_FACTOR: usize,
    const VECTOR_V_COMPRESSION_FACTOR: usize,
    const VECTOR_U_BLOCK_LEN: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
    const ETA2: usize,
    const ETA2_RANDOMNESS_SIZE: usize,
>(
    public_key: &MlKemPublicKey<PUBLIC_KEY_SIZE>,
    randomness: &[u8; SHARED_SECRET_SIZE],
) -> (MlKemCiphertext<CIPHERTEXT_SIZE>, MlKemSharedSecret) {
    unsafe {
        encapsulate_avx2::<
            K,
            CIPHERTEXT_SIZE,
            PUBLIC_KEY_SIZE,
            T_AS_NTT_ENCODED_SIZE,
            C1_SIZE,
            C2_SIZE,
            VECTOR_U_COMPRESSION_FACTOR,
            VECTOR_V_COMPRESSION_FACTOR,
            VECTOR_U_BLOCK_LEN,
            ETA1,
            ETA1_RANDOMNESS_SIZE,
            ETA2,
            ETA2_RANDOMNESS_SIZE,
        >(public_key, randomness)
    }
}

#[allow(unsafe_code)]
#[cfg(feature = "kyber")]
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
unsafe fn kyber_decapsulate_avx2<
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
>(
    private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
    ciphertext: &MlKemCiphertext<CIPHERTEXT_SIZE>,
) -> MlKemSharedSecret {
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
        crate::vector::SIMD256Vector,
        crate::hash_functions::avx2::Simd256Hash,
        crate::variant::Kyber,
    >(private_key, ciphertext)
}

#[allow(unsafe_code)]
#[cfg(feature = "kyber")]
pub fn kyber_decapsulate<
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
>(
    private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
    ciphertext: &MlKemCiphertext<CIPHERTEXT_SIZE>,
) -> MlKemSharedSecret {
    unsafe {
        kyber_decapsulate_avx2::<
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
        >(private_key, ciphertext)
    }
}

#[allow(unsafe_code)]
#[cfg_attr(not(hax), target_feature(enable = "avx2"))]
#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $SECRET_KEY_SIZE == Hacspec_ml_kem.Parameters.cca_private_key_size $K /\
    $CPA_SECRET_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_private_key_size $K /\
    $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K /\
    $CIPHERTEXT_SIZE == Hacspec_ml_kem.Parameters.cpa_ciphertext_size $K /\
    $T_AS_NTT_ENCODED_SIZE == Hacspec_ml_kem.Parameters.tt_as_ntt_encoded_size $K /\
    $C1_SIZE == Hacspec_ml_kem.Parameters.c1_size $K /\
    $C2_SIZE == Hacspec_ml_kem.Parameters.c2_size $K /\
    $VECTOR_U_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_u_compression_factor  $K /\
    $VECTOR_V_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_v_compression_factor  $K /\
    $C1_BLOCK_SIZE == Hacspec_ml_kem.Parameters.c1_block_size $K /\
    $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
    $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K /\
    $ETA2 == Hacspec_ml_kem.Parameters.eta2 $K /\
    $ETA2_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta2_randomness_size $K /\
    $IMPLICIT_REJECTION_HASH_INPUT_SIZE == Hacspec_ml_kem.Parameters.implicit_rejection_hash_input_size $K"#))]
unsafe fn decapsulate_avx2<
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
>(
    private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
    ciphertext: &MlKemCiphertext<CIPHERTEXT_SIZE>,
) -> MlKemSharedSecret {
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
        crate::vector::SIMD256Vector,
        crate::hash_functions::avx2::Simd256Hash,
        crate::variant::MlKem,
    >(private_key, ciphertext)
}

#[allow(unsafe_code)]
#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $SECRET_KEY_SIZE == Hacspec_ml_kem.Parameters.cca_private_key_size $K /\
    $CPA_SECRET_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_private_key_size $K /\
    $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K /\
    $CIPHERTEXT_SIZE == Hacspec_ml_kem.Parameters.cpa_ciphertext_size $K /\
    $T_AS_NTT_ENCODED_SIZE == Hacspec_ml_kem.Parameters.tt_as_ntt_encoded_size $K /\
    $C1_SIZE == Hacspec_ml_kem.Parameters.c1_size $K /\
    $C2_SIZE == Hacspec_ml_kem.Parameters.c2_size $K /\
    $VECTOR_U_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_u_compression_factor  $K /\
    $VECTOR_V_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_v_compression_factor  $K /\
    $C1_BLOCK_SIZE == Hacspec_ml_kem.Parameters.c1_block_size $K /\
    $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
    $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K /\
    $ETA2 == Hacspec_ml_kem.Parameters.eta2 $K /\
    $ETA2_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta2_randomness_size $K /\
    $IMPLICIT_REJECTION_HASH_INPUT_SIZE == Hacspec_ml_kem.Parameters.implicit_rejection_hash_input_size $K"#))]
pub fn decapsulate<
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
>(
    private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
    ciphertext: &MlKemCiphertext<CIPHERTEXT_SIZE>,
) -> MlKemSharedSecret {
    unsafe {
        decapsulate_avx2::<
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
        >(private_key, ciphertext)
    }
}

/// Unpacked API
pub(crate) mod unpacked {
    use super::*;

    pub(crate) type MlKemKeyPairUnpacked<const K: usize> =
        crate::ind_cca::unpacked::MlKemKeyPairUnpacked<K, crate::vector::SIMD256Vector>;
    pub(crate) type MlKemPublicKeyUnpacked<const K: usize> =
        crate::ind_cca::unpacked::MlKemPublicKeyUnpacked<K, crate::vector::SIMD256Vector>;

    /// Get the unpacked public key.
    #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
    #[allow(unsafe_code)]
    #[hax_lib::requires(
        fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
        $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K /\
        $T_AS_NTT_ENCODED_SIZE == Hacspec_ml_kem.Parameters.tt_as_ntt_encoded_size $K"#)
    )]
    unsafe fn unpack_public_key_avx2<
        const K: usize,
        const T_AS_NTT_ENCODED_SIZE: usize,
        const PUBLIC_KEY_SIZE: usize,
    >(
        public_key: &MlKemPublicKey<PUBLIC_KEY_SIZE>,
        unpacked_public_key: &mut MlKemPublicKeyUnpacked<K>,
    ) {
        crate::ind_cca::unpacked::unpack_public_key::<
            K,
            T_AS_NTT_ENCODED_SIZE,
            PUBLIC_KEY_SIZE,
            crate::hash_functions::avx2::Simd256Hash,
            crate::vector::SIMD256Vector,
        >(public_key, unpacked_public_key)
    }

    /// Get the unpacked public key.
    #[allow(unsafe_code)]
    #[hax_lib::requires(
        fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
        $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K /\
        $T_AS_NTT_ENCODED_SIZE == Hacspec_ml_kem.Parameters.tt_as_ntt_encoded_size $K"#)
    )]
    pub(crate) fn unpack_public_key<
        const K: usize,
        const T_AS_NTT_ENCODED_SIZE: usize,
        const PUBLIC_KEY_SIZE: usize,
    >(
        public_key: &MlKemPublicKey<PUBLIC_KEY_SIZE>,
        unpacked_public_key: &mut MlKemPublicKeyUnpacked<K>,
    ) {
        unsafe {
            unpack_public_key_avx2::<K, T_AS_NTT_ENCODED_SIZE, PUBLIC_KEY_SIZE>(
                public_key,
                unpacked_public_key,
            )
        }
    }

    /// Take a serialized private key and generate an unpacked key pair from it.
    #[inline(always)]
    #[hax_lib::requires(
        fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
                v_SECRET_KEY_SIZE == Hacspec_ml_kem.Parameters.cca_private_key_size v_K /\
                v_CPA_SECRET_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_private_key_size v_K /\
                v_PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size v_K /\
                v_T_AS_NTT_ENCODED_SIZE == Hacspec_ml_kem.Parameters.tt_as_ntt_encoded_size v_K"#))]
    pub(crate) fn keypair_from_private_key<
        const K: usize,
        const SECRET_KEY_SIZE: usize,
        const CPA_SECRET_KEY_SIZE: usize,
        const PUBLIC_KEY_SIZE: usize,
        const T_AS_NTT_ENCODED_SIZE: usize,
    >(
        private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
        key_pair: &mut MlKemKeyPairUnpacked<K>,
    ) {
        crate::ind_cca::unpacked::keys_from_private_key::<
            K,
            SECRET_KEY_SIZE,
            CPA_SECRET_KEY_SIZE,
            PUBLIC_KEY_SIZE,
            T_AS_NTT_ENCODED_SIZE,
            crate::vector::SIMD256Vector,
        >(private_key, key_pair);
    }

    #[allow(unsafe_code)]
    #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
    #[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
        $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K /\
        $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
        $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K"#))]
    unsafe fn generate_keypair_avx2<
        const K: usize,
        const CPA_PRIVATE_KEY_SIZE: usize,
        const PRIVATE_KEY_SIZE: usize,
        const PUBLIC_KEY_SIZE: usize,
        const ETA1: usize,
        const ETA1_RANDOMNESS_SIZE: usize,
    >(
        randomness: [u8; KEY_GENERATION_SEED_SIZE],
        out: &mut MlKemKeyPairUnpacked<K>,
    ) {
        crate::ind_cca::unpacked::generate_keypair::<
            K,
            CPA_PRIVATE_KEY_SIZE,
            PRIVATE_KEY_SIZE,
            PUBLIC_KEY_SIZE,
            ETA1,
            ETA1_RANDOMNESS_SIZE,
            crate::vector::SIMD256Vector,
            crate::hash_functions::avx2::Simd256Hash,
            crate::variant::MlKem,
        >(randomness, out)
    }

    /// Generate a key pair
    #[allow(unsafe_code)]
    #[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
        $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K /\
        $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
        $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K"#))]
    pub(crate) fn generate_keypair<
        const K: usize,
        const CPA_PRIVATE_KEY_SIZE: usize,
        const PRIVATE_KEY_SIZE: usize,
        const PUBLIC_KEY_SIZE: usize,
        const ETA1: usize,
        const ETA1_RANDOMNESS_SIZE: usize,
    >(
        randomness: [u8; KEY_GENERATION_SEED_SIZE],
        out: &mut MlKemKeyPairUnpacked<K>,
    ) {
        unsafe {
            generate_keypair_avx2::<
                K,
                CPA_PRIVATE_KEY_SIZE,
                PRIVATE_KEY_SIZE,
                PUBLIC_KEY_SIZE,
                ETA1,
                ETA1_RANDOMNESS_SIZE,
            >(randomness, out)
        }
    }

    #[allow(unsafe_code)]
    #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
    #[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
        $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
        $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K /\
        $ETA2 == Hacspec_ml_kem.Parameters.eta2 $K /\
        $ETA2_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta2_randomness_size $K /\
        $C1_SIZE == Hacspec_ml_kem.Parameters.c1_size $K /\
        $C2_SIZE == Hacspec_ml_kem.Parameters.c2_size $K /\
        $VECTOR_U_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_u_compression_factor $K /\
        $VECTOR_V_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_v_compression_factor $K /\
        $VECTOR_U_BLOCK_LEN == Hacspec_ml_kem.Parameters.c1_block_size $K /\
        $CIPHERTEXT_SIZE == Hacspec_ml_kem.Parameters.cpa_ciphertext_size $K"#))]
    unsafe fn encapsulate_avx2<
        const K: usize,
        const CIPHERTEXT_SIZE: usize,
        const PUBLIC_KEY_SIZE: usize,
        const T_AS_NTT_ENCODED_SIZE: usize,
        const C1_SIZE: usize,
        const C2_SIZE: usize,
        const VECTOR_U_COMPRESSION_FACTOR: usize,
        const VECTOR_V_COMPRESSION_FACTOR: usize,
        const VECTOR_U_BLOCK_LEN: usize,
        const ETA1: usize,
        const ETA1_RANDOMNESS_SIZE: usize,
        const ETA2: usize,
        const ETA2_RANDOMNESS_SIZE: usize,
    >(
        public_key: &MlKemPublicKeyUnpacked<K>,
        randomness: &[u8; SHARED_SECRET_SIZE],
    ) -> (MlKemCiphertext<CIPHERTEXT_SIZE>, MlKemSharedSecret) {
        crate::ind_cca::unpacked::encapsulate::<
            K,
            CIPHERTEXT_SIZE,
            PUBLIC_KEY_SIZE,
            T_AS_NTT_ENCODED_SIZE,
            C1_SIZE,
            C2_SIZE,
            VECTOR_U_COMPRESSION_FACTOR,
            VECTOR_V_COMPRESSION_FACTOR,
            VECTOR_U_BLOCK_LEN,
            ETA1,
            ETA1_RANDOMNESS_SIZE,
            ETA2,
            ETA2_RANDOMNESS_SIZE,
            crate::vector::SIMD256Vector,
            crate::hash_functions::avx2::Simd256Hash,
        >(public_key, randomness)
    }

    /// Unpacked encapsulate
    #[allow(unsafe_code)]
    #[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
        $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
        $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K /\
        $ETA2 == Hacspec_ml_kem.Parameters.eta2 $K /\
        $ETA2_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta2_randomness_size $K /\
        $C1_SIZE == Hacspec_ml_kem.Parameters.c1_size $K /\
        $C2_SIZE == Hacspec_ml_kem.Parameters.c2_size $K /\
        $VECTOR_U_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_u_compression_factor $K /\
        $VECTOR_V_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_v_compression_factor $K /\
        $VECTOR_U_BLOCK_LEN == Hacspec_ml_kem.Parameters.c1_block_size $K /\
        $CIPHERTEXT_SIZE == Hacspec_ml_kem.Parameters.cpa_ciphertext_size $K"#))]
    pub(crate) fn encapsulate<
        const K: usize,
        const CIPHERTEXT_SIZE: usize,
        const PUBLIC_KEY_SIZE: usize,
        const T_AS_NTT_ENCODED_SIZE: usize,
        const C1_SIZE: usize,
        const C2_SIZE: usize,
        const VECTOR_U_COMPRESSION_FACTOR: usize,
        const VECTOR_V_COMPRESSION_FACTOR: usize,
        const VECTOR_U_BLOCK_LEN: usize,
        const ETA1: usize,
        const ETA1_RANDOMNESS_SIZE: usize,
        const ETA2: usize,
        const ETA2_RANDOMNESS_SIZE: usize,
    >(
        public_key: &MlKemPublicKeyUnpacked<K>,
        randomness: &[u8; SHARED_SECRET_SIZE],
    ) -> (MlKemCiphertext<CIPHERTEXT_SIZE>, MlKemSharedSecret) {
        unsafe {
            encapsulate_avx2::<
                K,
                CIPHERTEXT_SIZE,
                PUBLIC_KEY_SIZE,
                T_AS_NTT_ENCODED_SIZE,
                C1_SIZE,
                C2_SIZE,
                VECTOR_U_COMPRESSION_FACTOR,
                VECTOR_V_COMPRESSION_FACTOR,
                VECTOR_U_BLOCK_LEN,
                ETA1,
                ETA1_RANDOMNESS_SIZE,
                ETA2,
                ETA2_RANDOMNESS_SIZE,
            >(public_key, randomness)
        }
    }

    #[cfg_attr(not(hax), target_feature(enable = "avx2"))]
    #[allow(unsafe_code)]
    #[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
        $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
        $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K /\
        $ETA2 == Hacspec_ml_kem.Parameters.eta2 $K /\
        $ETA2_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta2_randomness_size $K /\
        $C1_SIZE == Hacspec_ml_kem.Parameters.c1_size $K /\
        $C2_SIZE == Hacspec_ml_kem.Parameters.c2_size $K /\
        $VECTOR_U_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_u_compression_factor $K /\
        $VECTOR_V_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_v_compression_factor $K /\
        $C1_BLOCK_SIZE == Hacspec_ml_kem.Parameters.c1_block_size $K /\
        $CIPHERTEXT_SIZE == Hacspec_ml_kem.Parameters.cpa_ciphertext_size $K /\
        $IMPLICIT_REJECTION_HASH_INPUT_SIZE == Hacspec_ml_kem.Parameters.implicit_rejection_hash_input_size $K"#))]
    unsafe fn decapsulate_avx2<
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
    >(
        key_pair: &MlKemKeyPairUnpacked<K>,
        ciphertext: &MlKemCiphertext<CIPHERTEXT_SIZE>,
    ) -> MlKemSharedSecret {
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
            crate::vector::SIMD256Vector,
            crate::hash_functions::avx2::Simd256Hash,
        >(key_pair, ciphertext)
    }

    /// Unpacked decapsulate
    #[allow(unsafe_code)]
    #[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
        $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
        $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K /\
        $ETA2 == Hacspec_ml_kem.Parameters.eta2 $K /\
        $ETA2_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta2_randomness_size $K /\
        $C1_SIZE == Hacspec_ml_kem.Parameters.c1_size $K /\
        $C2_SIZE == Hacspec_ml_kem.Parameters.c2_size $K /\
        $VECTOR_U_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_u_compression_factor $K /\
        $VECTOR_V_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_v_compression_factor $K /\
        $C1_BLOCK_SIZE == Hacspec_ml_kem.Parameters.c1_block_size $K /\
        $CIPHERTEXT_SIZE == Hacspec_ml_kem.Parameters.cpa_ciphertext_size $K /\
        $IMPLICIT_REJECTION_HASH_INPUT_SIZE == Hacspec_ml_kem.Parameters.implicit_rejection_hash_input_size $K"#))]
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
    >(
        key_pair: &MlKemKeyPairUnpacked<K>,
        ciphertext: &MlKemCiphertext<CIPHERTEXT_SIZE>,
    ) -> MlKemSharedSecret {
        unsafe {
            decapsulate_avx2::<
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
            >(key_pair, ciphertext)
        }
    }
}

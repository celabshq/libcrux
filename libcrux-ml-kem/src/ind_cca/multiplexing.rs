use super::*;

// For the case where we didn't compile with the simd128/simd256 features but
// have a CPU that has it and thus tries to call the simd128/simd256 version,
// we fall back to the portable version in this case.

#[cfg(feature = "simd256")]
use instantiations::avx2::{
    decapsulate as decapsulate_avx2, encapsulate as encapsulate_avx2,
    generate_keypair as generate_keypair_avx2,
};

#[cfg(all(feature = "simd128", not(hax)))]
use instantiations::neon::{
    decapsulate as decapsulate_neon, encapsulate as encapsulate_neon,
    generate_keypair as generate_keypair_neon,
};

#[cfg(not(feature = "simd256"))]
use instantiations::portable::{
    decapsulate as decapsulate_avx2, encapsulate as encapsulate_avx2,
    generate_keypair as generate_keypair_avx2,
};

#[cfg(any(not(feature = "simd128"), hax))]
use instantiations::portable::{
    decapsulate as decapsulate_neon, encapsulate as encapsulate_neon,
    generate_keypair as generate_keypair_neon,
};

#[cfg(all(feature = "simd256", feature = "kyber"))]
use instantiations::avx2::{
    kyber_decapsulate as kyber_decapsulate_avx2, kyber_encapsulate as kyber_encapsulate_avx2,
    kyber_generate_keypair as kyber_generate_keypair_avx2,
};

#[cfg(all(feature = "simd128", feature = "kyber", not(hax)))]
use instantiations::neon::{
    kyber_decapsulate as kyber_decapsulate_neon, kyber_encapsulate as kyber_encapsulate_neon,
    kyber_generate_keypair as kyber_generate_keypair_neon,
};

#[cfg(all(not(feature = "simd256"), feature = "kyber"))]
use instantiations::portable::{
    kyber_decapsulate as kyber_decapsulate_avx2, kyber_encapsulate as kyber_encapsulate_avx2,
    kyber_generate_keypair as kyber_generate_keypair_avx2,
};

#[cfg(all(any(not(feature = "simd128"), hax), feature = "kyber"))]
use instantiations::portable::{
    kyber_decapsulate as kyber_decapsulate_neon, kyber_encapsulate as kyber_encapsulate_neon,
    kyber_generate_keypair as kyber_generate_keypair_neon,
};

#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K"#))]
#[inline(always)]
pub(crate) fn validate_public_key<const K: usize, const PUBLIC_KEY_SIZE: usize>(
    public_key: &[u8; PUBLIC_KEY_SIZE],
) -> bool {
    instantiations::portable::validate_public_key::<K, PUBLIC_KEY_SIZE>(public_key)
}

#[inline(always)]
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
    instantiations::portable::validate_private_key::<K, SECRET_KEY_SIZE, CIPHERTEXT_SIZE>(
        private_key,
        ciphertext,
    )
}

#[cfg(feature = "kyber")]
pub(crate) fn kyber_generate_keypair<
    const K: usize,
    const CPA_PRIVATE_KEY_SIZE: usize,
    const PRIVATE_KEY_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
    const ETA1: usize,
    const ETA1_RANDOMNESS_SIZE: usize,
>(
    randomness: [u8; KEY_GENERATION_SEED_SIZE],
) -> MlKemKeyPair<PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE> {
    // Runtime feature detection.
    if libcrux_platform::simd256_support() {
        kyber_generate_keypair_avx2::<
            K,
            CPA_PRIVATE_KEY_SIZE,
            PRIVATE_KEY_SIZE,
            PUBLIC_KEY_SIZE,
            ETA1,
            ETA1_RANDOMNESS_SIZE,
        >(&randomness)
    } else if libcrux_platform::simd128_support() {
        kyber_generate_keypair_neon::<
            K,
            CPA_PRIVATE_KEY_SIZE,
            PRIVATE_KEY_SIZE,
            PUBLIC_KEY_SIZE,
            ETA1,
            ETA1_RANDOMNESS_SIZE,
        >(&randomness)
    } else {
        instantiations::portable::kyber_generate_keypair::<
            K,
            CPA_PRIVATE_KEY_SIZE,
            PRIVATE_KEY_SIZE,
            PUBLIC_KEY_SIZE,
            ETA1,
            ETA1_RANDOMNESS_SIZE,
        >(&randomness)
    }
}

#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $CPA_PRIVATE_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_private_key_size $K /\
    $PRIVATE_KEY_SIZE == Hacspec_ml_kem.Parameters.cca_private_key_size $K /\
    $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K /\
    $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
    $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K"#))]
#[hax_lib::ensures(|result|
    match hacspec_ml_kem::generate_keypair::<K, PUBLIC_KEY_SIZE, PRIVATE_KEY_SIZE, CPA_PRIVATE_KEY_SIZE>(
        &hacspec_ml_kem::parameters::rank_to_params(K),
        randomness,
    ) {
        Ok((ek, dk)) => result.pk.value == ek && result.sk.value == dk,
        Err(_) => true,
    }
)]
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
    // Runtime feature detection.
    if libcrux_platform::simd256_support() {
        generate_keypair_avx2::<
            K,
            CPA_PRIVATE_KEY_SIZE,
            PRIVATE_KEY_SIZE,
            PUBLIC_KEY_SIZE,
            ETA1,
            ETA1_RANDOMNESS_SIZE,
        >(randomness)
    } else if libcrux_platform::simd128_support() {
        generate_keypair_neon::<
            K,
            CPA_PRIVATE_KEY_SIZE,
            PRIVATE_KEY_SIZE,
            PUBLIC_KEY_SIZE,
            ETA1,
            ETA1_RANDOMNESS_SIZE,
        >(randomness)
    } else {
        instantiations::portable::generate_keypair::<
            K,
            CPA_PRIVATE_KEY_SIZE,
            PRIVATE_KEY_SIZE,
            PUBLIC_KEY_SIZE,
            ETA1,
            ETA1_RANDOMNESS_SIZE,
        >(randomness)
    }
}

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
    randomness: [u8; SHARED_SECRET_SIZE],
) -> (MlKemCiphertext<CIPHERTEXT_SIZE>, MlKemSharedSecret) {
    if libcrux_platform::simd256_support() {
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
        >(public_key, &randomness)
    } else if libcrux_platform::simd128_support() {
        kyber_encapsulate_neon::<
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
        >(public_key, &randomness)
    } else {
        instantiations::portable::kyber_encapsulate::<
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
        >(public_key, &randomness)
    }
}

#[hax_lib::requires(fstar!(r#"Hacspec_ml_kem.Parameters.is_rank $K /\
    $CIPHERTEXT_SIZE == Hacspec_ml_kem.Parameters.cpa_ciphertext_size $K /\
    $PUBLIC_KEY_SIZE == Hacspec_ml_kem.Parameters.cpa_public_key_size $K /\
    $T_AS_NTT_ENCODED_SIZE == Hacspec_ml_kem.Parameters.tt_as_ntt_encoded_size $K /\
    $C1_SIZE == Hacspec_ml_kem.Parameters.c1_size $K /\
    $C2_SIZE == Hacspec_ml_kem.Parameters.c2_size $K /\
    $VECTOR_U_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_u_compression_factor  $K /\
    $VECTOR_V_COMPRESSION_FACTOR == Hacspec_ml_kem.Parameters.vector_v_compression_factor  $K /\
    $C1_BLOCK_SIZE == Hacspec_ml_kem.Parameters.c1_block_size $K /\
    $ETA1 == Hacspec_ml_kem.Parameters.eta1 $K /\
    $ETA1_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta1_randomness_size $K /\
    $ETA2 == Hacspec_ml_kem.Parameters.eta2 $K /\
    $ETA2_RANDOMNESS_SIZE == Hacspec_ml_kem.Parameters.eta2_randomness_size $K"#))]
#[hax_lib::ensures(|result|
    match hacspec_ml_kem::encapsulate::<K, PUBLIC_KEY_SIZE, C1_SIZE, C2_SIZE, CIPHERTEXT_SIZE>(
        &hacspec_ml_kem::parameters::rank_to_params(K),
        &public_key.value,
        randomness,
    ) {
        Ok((shared, ciphertext)) => result.0.value == ciphertext && result.1 == shared,
        Err(_) => true,
    }
)]
pub(crate) fn encapsulate<
    const K: usize,
    const CIPHERTEXT_SIZE: usize,
    const PUBLIC_KEY_SIZE: usize,
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
>(
    public_key: &MlKemPublicKey<PUBLIC_KEY_SIZE>,
    randomness: &[u8; SHARED_SECRET_SIZE],
) -> (MlKemCiphertext<CIPHERTEXT_SIZE>, MlKemSharedSecret) {
    if libcrux_platform::simd256_support() {
        encapsulate_avx2::<
            K,
            CIPHERTEXT_SIZE,
            PUBLIC_KEY_SIZE,
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
        >(public_key, randomness)
    } else if libcrux_platform::simd128_support() {
        encapsulate_neon::<
            K,
            CIPHERTEXT_SIZE,
            PUBLIC_KEY_SIZE,
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
        >(public_key, randomness)
    } else {
        instantiations::portable::encapsulate::<
            K,
            CIPHERTEXT_SIZE,
            PUBLIC_KEY_SIZE,
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
        >(public_key, randomness)
    }
}

#[cfg(feature = "kyber")]
pub(crate) fn kyber_decapsulate<
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
    if libcrux_platform::simd256_support() {
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
    } else if libcrux_platform::simd128_support() {
        kyber_decapsulate_neon::<
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
    } else {
        instantiations::portable::kyber_decapsulate::<
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
#[hax_lib::ensures(|result|
    match hacspec_ml_kem::decapsulate::<K, PUBLIC_KEY_SIZE, SECRET_KEY_SIZE, CPA_SECRET_KEY_SIZE, C1_SIZE, C2_SIZE, CIPHERTEXT_SIZE, IMPLICIT_REJECTION_HASH_INPUT_SIZE>(
        &hacspec_ml_kem::parameters::rank_to_params(K),
        &private_key.value,
        &ciphertext.value,
    ) {
        Ok(expected) => result == expected,
        Err(_) => true,
    }
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
>(
    private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
    ciphertext: &MlKemCiphertext<CIPHERTEXT_SIZE>,
) -> MlKemSharedSecret {
    if libcrux_platform::simd256_support() {
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
    } else if libcrux_platform::simd128_support() {
        decapsulate_neon::<
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
    } else {
        instantiations::portable::decapsulate::<
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

use crate::{
    constant_time_ops::compare_ciphertexts_select_shared_secret_in_constant_time,
    constants::{
        ranked_bytes_per_ring_element, CPA_PKE_KEY_GENERATION_SEED_SIZE, H_DIGEST_SIZE,
        SHARED_SECRET_SIZE,
    },
    hash_functions::Hash,
    ind_cpa::serialize_public_key,
    serialize::deserialize_ring_elements_reduced_out,
    types::*,
    utils::into_padded_array,
    variant::*,
    vector::Operations,
};

/// Seed size for key generation
pub const KEY_GENERATION_SEED_SIZE: usize = CPA_PKE_KEY_GENERATION_SEED_SIZE + SHARED_SECRET_SIZE;

/// Seed size for encapsulation
pub const ENCAPS_SEED_SIZE: usize = SHARED_SECRET_SIZE;

// TODO: We should make this an actual type as opposed to alias so we can enforce
// some checks at the type level. This is being tracked in:
// https://github.com/cryspen/libcrux/issues/123
/// An ML-KEM shared secret.
///
/// A byte array of size [`SHARED_SECRET_SIZE`].
pub type MlKemSharedSecret = [u8; SHARED_SECRET_SIZE];

/// This module instantiates the functions in this file and multiplexes between
/// different implementations at runtime.
#[cfg(not(eurydice))]
pub(crate) mod multiplexing;

/// This module instantiates the functions in this file for each platform.
/// To use these, runtime checks must be performed before calling them.
pub(crate) mod instantiations;

/// This module implements an incremental API for ML-KEM.
#[cfg(not(eurydice))]
pub(crate) mod incremental;

/// Serialize the secret key.

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
    $SERIALIZED_KEY_LEN == Spec.MLKEM.v_CCA_PRIVATE_KEY_SIZE $K /\
    ${private_key.len()} == Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE $K /\
    ${public_key.len()} == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K /\
    ${implicit_rejection_value.len()} == Spec.MLKEM.v_SHARED_SECRET_SIZE"#))]
#[hax_lib::ensures(|result| fstar!(r#"${serialized}_future == Seq.append $private_key (
                                              Seq.append $public_key (
                                              Seq.append (Spec.Utils.v_H $public_key) 
                                                  $implicit_rejection_value))"#))]
fn serialize_kem_secret_key_mut<
    const K: usize,
    const SERIALIZED_KEY_LEN: usize,
    Hasher: Hash<K>,
>(
    private_key: &[u8],
    public_key: &[u8],
    implicit_rejection_value: &[u8],
    serialized: &mut [u8; SERIALIZED_KEY_LEN],
) {
    let mut pointer = 0;
    serialized[pointer..pointer + private_key.len()].copy_from_slice(private_key);
    pointer += private_key.len();
    serialized[pointer..pointer + public_key.len()].copy_from_slice(public_key);
    pointer += public_key.len();
    serialized[pointer..pointer + H_DIGEST_SIZE].copy_from_slice(&Hasher::H(public_key));
    pointer += H_DIGEST_SIZE;
    serialized[pointer..pointer + implicit_rejection_value.len()]
        .copy_from_slice(implicit_rejection_value);

    hax_lib::fstar!(
   "let open Spec.Utils in
    assert (Seq.slice serialized 0 (v #usize_inttype (Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE $K)) `Seq.equal` $private_key);
    assert (Seq.slice serialized (v #usize_inttype (Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE $K))
                            (v #usize_inttype (Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE $K +! Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K)) `Seq.equal` $public_key);
    assert (Seq.slice serialized (v #usize_inttype (Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE $K +!
                                            Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K))
                            (v #usize_inttype (Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE $K +!
                                            Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K +!
                                            Libcrux_ml_kem.Constants.v_H_DIGEST_SIZE))
            `Seq.equal` Libcrux_ml_kem.Hash_functions.f_H #$:Hasher #$K $public_key);
    assert (Seq.slice serialized (v #usize_inttype (Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE $K +!
                                            Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K +!
                                            Libcrux_ml_kem.Constants.v_H_DIGEST_SIZE))
                            (v #usize_inttype (Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE $K +!
                                            Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K +!
                                            Libcrux_ml_kem.Constants.v_H_DIGEST_SIZE +!
                                            Spec.MLKEM.v_SHARED_SECRET_SIZE))
            == $implicit_rejection_value);
    lemma_slice_append_4 serialized $private_key $public_key (Libcrux_ml_kem.Hash_functions.f_H #$:Hasher #$K $public_key) $implicit_rejection_value"
    );
}

#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
    $SERIALIZED_KEY_LEN == Spec.MLKEM.v_CCA_PRIVATE_KEY_SIZE $K /\
    ${private_key.len()} == Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE $K /\
    ${public_key.len()} == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K /\
    ${implicit_rejection_value.len()} == Spec.MLKEM.v_SHARED_SECRET_SIZE"#))]
#[hax_lib::ensures(|result| fstar!(r#"$result == Seq.append $private_key (
                                              Seq.append $public_key (
                                              Seq.append (Spec.Utils.v_H $public_key) 
                                                  $implicit_rejection_value))"#))]
fn serialize_kem_secret_key<const K: usize, const SERIALIZED_KEY_LEN: usize, Hasher: Hash<K>>(
    private_key: &[u8],
    public_key: &[u8],
    implicit_rejection_value: &[u8],
) -> [u8; SERIALIZED_KEY_LEN] {
    let mut out = [0u8; SERIALIZED_KEY_LEN];

    serialize_kem_secret_key_mut::<K, SERIALIZED_KEY_LEN, Hasher>(
        private_key,
        public_key,
        implicit_rejection_value,
        &mut out,
    );
    out
}

/// Validate an ML-KEM public key.
///
/// This implements the Modulus check in 7.2 2.
/// Note that the size check in 7.2 1 is covered by the `PUBLIC_KEY_SIZE` in the
/// `public_key` type.
#[inline(always)]
#[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
    $PUBLIC_KEY_SIZE == Spec.MLKEM.v_CCA_PUBLIC_KEY_SIZE $K"#))]
pub(crate) fn validate_public_key<
    const K: usize,
    const PUBLIC_KEY_SIZE: usize,
    Vector: Operations,
>(
    public_key: &[u8; PUBLIC_KEY_SIZE],
) -> bool {
    let deserialized_pk = deserialize_ring_elements_reduced_out::<K, Vector>(
        &public_key[..ranked_bytes_per_ring_element(K)],
    );
    let public_key_serialized = serialize_public_key::<K, PUBLIC_KEY_SIZE, Vector>(
        &deserialized_pk,
        &public_key[ranked_bytes_per_ring_element(K)..],
    );

    *public_key == public_key_serialized
}

/// Validate an ML-KEM private key.
///
/// This implements the Hash check in 7.3 3.
/// Note that the size checks in 7.2 1 and 2 are covered by the `SECRET_KEY_SIZE`
/// and `CIPHERTEXT_SIZE` in the `private_key` and `ciphertext` types.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
    $SECRET_KEY_SIZE == Spec.MLKEM.v_CCA_PRIVATE_KEY_SIZE $K /\
    $CIPHERTEXT_SIZE == Spec.MLKEM.v_CPA_CIPHERTEXT_SIZE $K"#))]
pub(crate) fn validate_private_key<
    const K: usize,
    const SECRET_KEY_SIZE: usize,
    const CIPHERTEXT_SIZE: usize,
    Hasher: Hash<K>,
>(
    private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
    _ciphertext: &MlKemCiphertext<CIPHERTEXT_SIZE>,
) -> bool {
    validate_private_key_only::<K, SECRET_KEY_SIZE, Hasher>(private_key)
}

/// Validate an ML-KEM private key.
///
/// This implements the Hash check in 7.3 3.
#[inline(always)]
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
    $SECRET_KEY_SIZE == Spec.MLKEM.v_CCA_PRIVATE_KEY_SIZE $K"#))]
pub(crate) fn validate_private_key_only<
    const K: usize,
    const SECRET_KEY_SIZE: usize,
    Hasher: Hash<K>,
>(
    private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
) -> bool {
    // Eurydice can't access values directly on the types. We need to go to the
    // `value` directly.

    let t = Hasher::H(&private_key.value[384 * K..768 * K + 32]);
    let expected = &private_key.value[768 * K + 32..768 * K + 64];
    t == expected
}

/// Packed API
///
/// Generate a key pair.
///
/// Depending on the `Vector` and `Hasher` used, this requires different hardware
/// features
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
    $CPA_PRIVATE_KEY_SIZE == Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE $K /\
    $PRIVATE_KEY_SIZE == Spec.MLKEM.v_CCA_PRIVATE_KEY_SIZE $K /\
    $PUBLIC_KEY_SIZE == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K /\
    $ETA1 == Spec.MLKEM.v_ETA1 $K /\
    $ETA1_RANDOMNESS_SIZE == Spec.MLKEM.v_ETA1_RANDOMNESS_SIZE $K"#))]
#[hax_lib::ensures(|result| fstar!(r#"let (expected, valid) = Spec.MLKEM.ind_cca_generate_keypair $K $randomness in
                                    valid ==> (${result}.f_sk.f_value, ${result}.f_pk.f_value) == expected"#))]
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
    Scheme: Variant,
>(
    randomness: &[u8; KEY_GENERATION_SEED_SIZE],
) -> MlKemKeyPair<PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE> {
    let ind_cpa_keypair_randomness = &randomness[0..CPA_PKE_KEY_GENERATION_SEED_SIZE];
    let implicit_rejection_value = &randomness[CPA_PKE_KEY_GENERATION_SEED_SIZE..];

    let (ind_cpa_private_key, public_key) = crate::ind_cpa::generate_keypair::<
        K,
        CPA_PRIVATE_KEY_SIZE,
        PUBLIC_KEY_SIZE,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
        Vector,
        Hasher,
        Scheme,
    >(ind_cpa_keypair_randomness);

    let secret_key_serialized = serialize_kem_secret_key::<K, PRIVATE_KEY_SIZE, Hasher>(
        &ind_cpa_private_key,
        &public_key,
        implicit_rejection_value,
    );
    let private_key: MlKemPrivateKey<PRIVATE_KEY_SIZE> =
        MlKemPrivateKey::from(secret_key_serialized);

    MlKemKeyPair::from(private_key, MlKemPublicKey::from(public_key))
}

#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
    $CIPHERTEXT_SIZE == Spec.MLKEM.v_CPA_CIPHERTEXT_SIZE $K /\
    $PUBLIC_KEY_SIZE == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K /\
    $T_AS_NTT_ENCODED_SIZE == Spec.MLKEM.v_T_AS_NTT_ENCODED_SIZE $K /\
    $C1_SIZE == Spec.MLKEM.v_C1_SIZE $K /\
    $C2_SIZE == Spec.MLKEM.v_C2_SIZE $K /\
    $VECTOR_U_COMPRESSION_FACTOR == Spec.MLKEM.v_VECTOR_U_COMPRESSION_FACTOR  $K /\
    $VECTOR_V_COMPRESSION_FACTOR == Spec.MLKEM.v_VECTOR_V_COMPRESSION_FACTOR  $K /\
    $C1_BLOCK_SIZE == Spec.MLKEM.v_C1_BLOCK_SIZE $K /\
    $ETA1 == Spec.MLKEM.v_ETA1 $K /\
    $ETA1_RANDOMNESS_SIZE == Spec.MLKEM.v_ETA1_RANDOMNESS_SIZE $K /\
    $ETA2 == Spec.MLKEM.v_ETA2 $K /\
    $ETA2_RANDOMNESS_SIZE == Spec.MLKEM.v_ETA2_RANDOMNESS_SIZE $K"#))]
#[hax_lib::ensures(|result| fstar!(r#"let (expected, valid) = Spec.MLKEM.ind_cca_encapsulate $K ${public_key}.f_value $randomness in
                                    valid ==> (${result}._1.f_value, ${result}._2) == expected"#))]
#[inline(always)]
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
    Vector: Operations,
    Hasher: Hash<K>,
    Scheme: Variant,
>(
    public_key: &MlKemPublicKey<PUBLIC_KEY_SIZE>,
    randomness: &[u8; SHARED_SECRET_SIZE],
) -> (MlKemCiphertext<CIPHERTEXT_SIZE>, MlKemSharedSecret) {
    let randomness = Scheme::entropy_preprocess::<K, Hasher>(randomness);
    let mut to_hash: [u8; 2 * H_DIGEST_SIZE] = into_padded_array(&randomness);

    hax_lib::fstar!(r#"eq_intro (Seq.slice $to_hash 0 32) $randomness"#);
    to_hash[H_DIGEST_SIZE..].copy_from_slice(&Hasher::H(public_key.as_slice()));

    hax_lib::fstar!(
        "assert (Seq.slice to_hash 0 (v $H_DIGEST_SIZE) == $randomness);
        lemma_slice_append $to_hash $randomness (Spec.Utils.v_H ${public_key}.f_value);
        assert ($to_hash == concat $randomness (Spec.Utils.v_H ${public_key}.f_value))"
    );
    let hashed = Hasher::G(&to_hash);
    let (shared_secret, pseudorandomness) = hashed.split_at(SHARED_SECRET_SIZE);

    let ciphertext = crate::ind_cpa::encrypt::<
        K,
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
        Vector,
        Hasher,
    >(public_key.as_slice(), &randomness, pseudorandomness);

    (
        MlKemCiphertext::from(ciphertext),
        Scheme::kdf::<K, CIPHERTEXT_SIZE, Hasher>(shared_secret, &ciphertext),
    )
}

/// This code verifies on some machines, runs out of memory on others
#[hax_lib::fstar::verification_status(panic_free)]
#[hax_lib::fstar::options("--z3rlimit 500")]
#[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
    $SECRET_KEY_SIZE == Spec.MLKEM.v_CCA_PRIVATE_KEY_SIZE $K /\
    $CPA_SECRET_KEY_SIZE == Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE $K /\
    $PUBLIC_KEY_SIZE == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K /\
    $CIPHERTEXT_SIZE == Spec.MLKEM.v_CPA_CIPHERTEXT_SIZE $K /\
    $T_AS_NTT_ENCODED_SIZE == Spec.MLKEM.v_T_AS_NTT_ENCODED_SIZE $K /\
    $C1_SIZE == Spec.MLKEM.v_C1_SIZE $K /\
    $C2_SIZE == Spec.MLKEM.v_C2_SIZE $K /\
    $VECTOR_U_COMPRESSION_FACTOR == Spec.MLKEM.v_VECTOR_U_COMPRESSION_FACTOR  $K /\
    $VECTOR_V_COMPRESSION_FACTOR == Spec.MLKEM.v_VECTOR_V_COMPRESSION_FACTOR  $K /\
    $C1_BLOCK_SIZE == Spec.MLKEM.v_C1_BLOCK_SIZE $K /\
    $ETA1 == Spec.MLKEM.v_ETA1 $K /\
    $ETA1_RANDOMNESS_SIZE == Spec.MLKEM.v_ETA1_RANDOMNESS_SIZE $K /\
    $ETA2 == Spec.MLKEM.v_ETA2 $K /\
    $ETA2_RANDOMNESS_SIZE == Spec.MLKEM.v_ETA2_RANDOMNESS_SIZE $K /\
    $IMPLICIT_REJECTION_HASH_INPUT_SIZE == Spec.MLKEM.v_IMPLICIT_REJECTION_HASH_INPUT_SIZE $K"#))]
#[hax_lib::ensures(|result| fstar!(r#"let (expected, valid) = Spec.MLKEM.ind_cca_decapsulate $K ${private_key}.f_value ${ciphertext}.f_value in
                                    valid ==> $result == expected"#))]
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
    Scheme: Variant,
>(
    private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
    ciphertext: &MlKemCiphertext<CIPHERTEXT_SIZE>,
) -> MlKemSharedSecret {
    hax_lib::fstar!(
        r#"assert (v $CIPHERTEXT_SIZE == v $IMPLICIT_REJECTION_HASH_INPUT_SIZE - v $SHARED_SECRET_SIZE)"#
    );
    let (ind_cpa_secret_key, ind_cpa_public_key, ind_cpa_public_key_hash, implicit_rejection_value) =
        unpack_private_key::<CPA_SECRET_KEY_SIZE, PUBLIC_KEY_SIZE>(&private_key.value);

    hax_lib::fstar!(
        r#"assert ($ind_cpa_secret_key == slice ${private_key}.f_value (sz 0) $CPA_SECRET_KEY_SIZE);
        assert ($ind_cpa_public_key == slice ${private_key}.f_value $CPA_SECRET_KEY_SIZE ($CPA_SECRET_KEY_SIZE +! $PUBLIC_KEY_SIZE));
        assert ($ind_cpa_public_key_hash == slice ${private_key}.f_value ($CPA_SECRET_KEY_SIZE +! $PUBLIC_KEY_SIZE)
            ($CPA_SECRET_KEY_SIZE +! $PUBLIC_KEY_SIZE +! Spec.MLKEM.v_H_DIGEST_SIZE));
        assert ($implicit_rejection_value == slice ${private_key}.f_value ($CPA_SECRET_KEY_SIZE +! $PUBLIC_KEY_SIZE +! Spec.MLKEM.v_H_DIGEST_SIZE)
            (length ${private_key}.f_value))"#
    );
    let decrypted = crate::ind_cpa::decrypt::<
        K,
        CIPHERTEXT_SIZE,
        C1_SIZE,
        VECTOR_U_COMPRESSION_FACTOR,
        VECTOR_V_COMPRESSION_FACTOR,
        Vector,
    >(ind_cpa_secret_key, &ciphertext.value);

    let mut to_hash: [u8; SHARED_SECRET_SIZE + H_DIGEST_SIZE] = into_padded_array(&decrypted);
    hax_lib::fstar!(r#"eq_intro (Seq.slice $to_hash 0 32) $decrypted"#);
    to_hash[SHARED_SECRET_SIZE..].copy_from_slice(ind_cpa_public_key_hash);

    hax_lib::fstar!(
        r#"lemma_slice_append to_hash $decrypted $ind_cpa_public_key_hash;
        assert ($decrypted == Spec.MLKEM.ind_cpa_decrypt $K $ind_cpa_secret_key ${ciphertext}.f_value);
        assert ($to_hash == concat $decrypted $ind_cpa_public_key_hash)"#
    );
    let hashed = Hasher::G(&to_hash);
    let (shared_secret, pseudorandomness) = hashed.split_at(SHARED_SECRET_SIZE);

    hax_lib::fstar!(
        r#"assert (($shared_secret , $pseudorandomness) == split $hashed $SHARED_SECRET_SIZE);
        assert (length $implicit_rejection_value = $SECRET_KEY_SIZE -! $CPA_SECRET_KEY_SIZE -! $PUBLIC_KEY_SIZE -! $H_DIGEST_SIZE);
        assert (length $implicit_rejection_value = Spec.MLKEM.v_SHARED_SECRET_SIZE);
        assert (Spec.MLKEM.v_SHARED_SECRET_SIZE <=. Spec.MLKEM.v_IMPLICIT_REJECTION_HASH_INPUT_SIZE $K)"#
    );
    let mut to_hash: [u8; IMPLICIT_REJECTION_HASH_INPUT_SIZE] =
        into_padded_array(implicit_rejection_value);
    hax_lib::fstar!(r#"eq_intro (Seq.slice $to_hash 0 32) $implicit_rejection_value"#);
    to_hash[SHARED_SECRET_SIZE..].copy_from_slice(ciphertext.as_ref());
    hax_lib::fstar!(
        "assert_norm (pow2 32 == 0x100000000);
        assert (v (sz 32) < pow2 32);
        assert (i1.f_PRF_pre (sz 32) $to_hash);
        lemma_slice_append $to_hash $implicit_rejection_value ${ciphertext}.f_value"
    );
    let implicit_rejection_shared_secret: [u8; SHARED_SECRET_SIZE] = Hasher::PRF(&to_hash);

    hax_lib::fstar!(
        "assert ($implicit_rejection_shared_secret == Spec.Utils.v_PRF (sz 32) $to_hash);
        assert (Seq.length $ind_cpa_public_key == v $PUBLIC_KEY_SIZE)"
    );
    let expected_ciphertext = crate::ind_cpa::encrypt::<
        K,
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
        Vector,
        Hasher,
    >(ind_cpa_public_key, &decrypted, pseudorandomness);

    let implicit_rejection_shared_secret = Scheme::kdf::<K, CIPHERTEXT_SIZE, Hasher>(
        &implicit_rejection_shared_secret,
        ciphertext.as_slice(),
    );
    let shared_secret =
        Scheme::kdf::<K, CIPHERTEXT_SIZE, Hasher>(shared_secret, ciphertext.as_slice());

    compare_ciphertexts_select_shared_secret_in_constant_time(
        ciphertext.as_ref(),
        &expected_ciphertext,
        &shared_secret,
        &implicit_rejection_shared_secret,
    )
}

/// Types for the unpacked API.
pub(crate) mod unpacked {
    use core::array::from_fn;

    use super::*;
    use crate::{
        constant_time_ops::{
            compare_ciphertexts_in_constant_time, select_shared_secret_in_constant_time,
        },
        hash_functions::portable::PortableHash,
        ind_cpa::{self, generate_keypair_unpacked, serialize_public_key_mut, unpacked::*},
        matrix::sample_matrix_A,
        polynomial::PolynomialRingElement,
        serialize::deserialize_ring_elements_reduced,
        vector::traits::Operations,
    };

    /// An unpacked ML-KEM IND-CCA Private Key
    pub struct MlKemPrivateKeyUnpacked<const K: usize, Vector: Operations> {
        pub(crate) ind_cpa_private_key: IndCpaPrivateKeyUnpacked<K, Vector>,
        pub(crate) implicit_rejection_value: [u8; 32],
    }

    /// An unpacked ML-KEM IND-CCA Private Key
    #[derive(Clone)]
    pub struct MlKemPublicKeyUnpacked<const K: usize, Vector: Operations> {
        pub(crate) ind_cpa_public_key: IndCpaPublicKeyUnpacked<K, Vector>,
        pub(crate) public_key_hash: [u8; 32],
    }

    /// An unpacked ML-KEM KeyPair
    pub struct MlKemKeyPairUnpacked<const K: usize, Vector: Operations> {
        pub private_key: MlKemPrivateKeyUnpacked<K, Vector>,
        pub public_key: MlKemPublicKeyUnpacked<K, Vector>,
    }

    /// Generate an unpacked key from a serialized key.
    #[hax_lib::requires(
        fstar!(r#"Spec.MLKEM.is_rank $K /\
        $PUBLIC_KEY_SIZE == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K /\
        $T_AS_NTT_ENCODED_SIZE == Spec.MLKEM.v_T_AS_NTT_ENCODED_SIZE $K"#)
    )]
    #[hax_lib::ensures(|result| {
        let unpacked_public_key_future = future(unpacked_public_key);
        {fstar!(r#"let (public_key_hash, (seed, (deserialized_pk, (matrix_A, valid)))) =
            Spec.MLKEM.ind_cca_unpack_public_key $K ${public_key}.f_value in (valid ==>
            Libcrux_ml_kem.Polynomial.to_spec_matrix_t #$K #$:Vector ${unpacked_public_key_future.ind_cpa_public_key.A} == matrix_A) /\
        Libcrux_ml_kem.Polynomial.to_spec_vector_t #$K #$:Vector ${unpacked_public_key_future.ind_cpa_public_key.t_as_ntt} == deserialized_pk /\
        ${unpacked_public_key_future.ind_cpa_public_key.seed_for_A} == seed /\
        ${unpacked_public_key_future.public_key_hash} == public_key_hash"#)}})
    ]
    #[inline(always)]
    pub(crate) fn unpack_public_key<
        const K: usize,
        const T_AS_NTT_ENCODED_SIZE: usize,
        const PUBLIC_KEY_SIZE: usize,
        Hasher: Hash<K>,
        Vector: Operations,
    >(
        public_key: &MlKemPublicKey<PUBLIC_KEY_SIZE>,
        unpacked_public_key: &mut MlKemPublicKeyUnpacked<K, Vector>,
    ) {
        deserialize_ring_elements_reduced::<K, Vector>(
            &public_key.value[..T_AS_NTT_ENCODED_SIZE],
            &mut unpacked_public_key.ind_cpa_public_key.t_as_ntt,
        );
        hax_lib::fstar!(
            r#"let (_, seed) = split ${public_key}.f_value (Spec.MLKEM.v_T_AS_NTT_ENCODED_SIZE $K) in
            eq_intro (Libcrux_ml_kem.Utils.into_padded_array (sz 32) seed) seed;
            eq_intro (Seq.slice (Libcrux_ml_kem.Utils.into_padded_array (sz 34) seed) 0 32) seed"#
        );
        unpacked_public_key.ind_cpa_public_key.seed_for_A =
            into_padded_array(&public_key.value[T_AS_NTT_ENCODED_SIZE..]);
        sample_matrix_A::<K, Vector, Hasher>(
            &mut unpacked_public_key.ind_cpa_public_key.A,
            &into_padded_array(&public_key.value[T_AS_NTT_ENCODED_SIZE..]),
            false,
        );
        unpacked_public_key.public_key_hash = Hasher::H(public_key.as_slice());
    }

    #[hax_lib::attributes]
    impl<const K: usize, Vector: Operations> MlKemPublicKeyUnpacked<K, Vector> {
        /// Get the serialized public key.
        #[inline(always)]
        #[requires(fstar!(r#"let ${self_} = self in
        Spec.MLKEM.is_rank $K /\
            $PUBLIC_KEY_SIZE == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K /\
            (forall (i:nat). i < v $K ==>
                Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 (Seq.index 
                    ${self_.ind_cpa_public_key.t_as_ntt} i))"#))]
        #[ensures(|_|
            fstar!(r#"let ${self_} = self in            
            ${serialized}_future.f_value == 
                Seq.append (Spec.MLKEM.vector_encode_12 #$K
                    (Libcrux_ml_kem.Polynomial.to_spec_vector_t #$K #$:Vector
                        ${self_.ind_cpa_public_key.t_as_ntt}))
                ${self_.ind_cpa_public_key.seed_for_A})"#)
        )]
        pub fn serialized_mut<const PUBLIC_KEY_SIZE: usize>(
            &self,
            serialized: &mut MlKemPublicKey<PUBLIC_KEY_SIZE>,
        ) {
            serialize_public_key_mut::<K, PUBLIC_KEY_SIZE, Vector>(
                &self.ind_cpa_public_key.t_as_ntt,
                &self.ind_cpa_public_key.seed_for_A,
                &mut serialized.value,
            );
        }

        /// Get the serialized public key.
        #[inline(always)]
        #[requires(fstar!(r#"let ${self_} = self in
        Spec.MLKEM.is_rank $K /\
            $PUBLIC_KEY_SIZE == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K /\
            (forall (i:nat). i < v $K ==>
                Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 (Seq.index
                    ${self_.ind_cpa_public_key.t_as_ntt} i))"#))]
        #[ensures(|res|
            fstar!(r#"let ${self_} = self in
            ${res.value} == Seq.append (Spec.MLKEM.vector_encode_12 #$K
                            (Libcrux_ml_kem.Polynomial.to_spec_vector_t #$K #$:Vector
                                ${self_.ind_cpa_public_key.t_as_ntt}))
                        ${self_.ind_cpa_public_key.seed_for_A})"#)
        )]
        pub fn serialized<const PUBLIC_KEY_SIZE: usize>(&self) -> MlKemPublicKey<PUBLIC_KEY_SIZE> {
            MlKemPublicKey::from(serialize_public_key::<K, PUBLIC_KEY_SIZE, Vector>(
                &self.ind_cpa_public_key.t_as_ntt,
                &self.ind_cpa_public_key.seed_for_A,
            ))
        }
    }

    impl<const K: usize, Vector: Operations> Default for MlKemPublicKeyUnpacked<K, Vector> {
        #[inline(always)]
        fn default() -> Self {
            Self {
                ind_cpa_public_key: IndCpaPublicKeyUnpacked::default(),
                public_key_hash: [0u8; 32],
            }
        }
    }

    /// Take a serialized private key and generate an unpacked key pair from it.
    #[inline(always)]
    #[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
           v_SECRET_KEY_SIZE == Spec.MLKEM.v_CCA_PRIVATE_KEY_SIZE v_K /\
           v_CPA_SECRET_KEY_SIZE == Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE v_K /\
           v_PUBLIC_KEY_SIZE == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE v_K /\
           v_T_AS_NTT_ENCODED_SIZE == Spec.MLKEM.v_T_AS_NTT_ENCODED_SIZE v_K"#))]
    pub fn keys_from_private_key<
        const K: usize,
        const SECRET_KEY_SIZE: usize,
        const CPA_SECRET_KEY_SIZE: usize,
        const PUBLIC_KEY_SIZE: usize,
        const T_AS_NTT_ENCODED_SIZE: usize,
        Vector: Operations,
    >(
        private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
        key_pair: &mut MlKemKeyPairUnpacked<K, Vector>,
    ) {
        let (
            ind_cpa_secret_key,
            ind_cpa_public_key,
            ind_cpa_public_key_hash,
            implicit_rejection_value,
        ) = unpack_private_key::<CPA_SECRET_KEY_SIZE, PUBLIC_KEY_SIZE>(&private_key.value);

        ind_cpa::deserialize_vector::<K, Vector>(
            ind_cpa_secret_key,
            &mut key_pair.private_key.ind_cpa_private_key.secret_as_ntt,
        );
        ind_cpa::build_unpacked_public_key_mut::<K, T_AS_NTT_ENCODED_SIZE, Vector, PortableHash<K>>(
            ind_cpa_public_key,
            &mut key_pair.public_key.ind_cpa_public_key,
        );
        key_pair
            .public_key
            .public_key_hash
            .copy_from_slice(ind_cpa_public_key_hash);
        key_pair
            .private_key
            .implicit_rejection_value
            .copy_from_slice(implicit_rejection_value);
        key_pair
            .public_key
            .ind_cpa_public_key
            .seed_for_A
            .copy_from_slice(&ind_cpa_public_key[T_AS_NTT_ENCODED_SIZE..]);
    }

    #[hax_lib::attributes]
    impl<const K: usize, Vector: Operations> MlKemKeyPairUnpacked<K, Vector> {
        /// Create a new empty unpacked key pair.
        #[inline(always)]
        pub fn new() -> Self {
            Self::default()
        }

        /// Take a serialized private key and generate an unpacked key pair from it.
        #[inline(always)]
        #[requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
           v_SECRET_KEY_SIZE == Spec.MLKEM.v_CCA_PRIVATE_KEY_SIZE v_K /\
           v_CPA_SECRET_KEY_SIZE == Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE v_K /\
           v_PUBLIC_KEY_SIZE == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE v_K /\
           v_T_AS_NTT_ENCODED_SIZE == Spec.MLKEM.v_T_AS_NTT_ENCODED_SIZE v_K)"#))]
        pub fn from_private_key<
            const SECRET_KEY_SIZE: usize,
            const CPA_SECRET_KEY_SIZE: usize,
            const PUBLIC_KEY_SIZE: usize,
            const T_AS_NTT_ENCODED_SIZE: usize,
        >(
            private_key: &MlKemPrivateKey<SECRET_KEY_SIZE>,
        ) -> Self {
            let mut out = Self::default();
            keys_from_private_key::<
                K,
                SECRET_KEY_SIZE,
                CPA_SECRET_KEY_SIZE,
                PUBLIC_KEY_SIZE,
                T_AS_NTT_ENCODED_SIZE,
                Vector,
            >(private_key, &mut out);
            out
        }

        /// Get the serialized public key.
        #[inline(always)]
        #[requires(fstar!(r#"let ${self_} = self in
        Spec.MLKEM.is_rank $K /\
            $PUBLIC_KEY_SIZE == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K /\
            (forall (i:nat). i < v $K ==>
                Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 (Seq.index 
                    ${self_.public_key.ind_cpa_public_key.t_as_ntt} i))"#))]
        #[ensures(|_|
            fstar!(r#"let ${self_} = self in
            ${serialized}_future.f_value == 
                Seq.append (Spec.MLKEM.vector_encode_12 #$K
                    (Libcrux_ml_kem.Polynomial.to_spec_vector_t #$K #$:Vector
                        ${self_.public_key.ind_cpa_public_key.t_as_ntt}))
                ${self_.public_key.ind_cpa_public_key.seed_for_A})"#)
        )]
        pub fn serialized_public_key_mut<const PUBLIC_KEY_SIZE: usize>(
            &self,
            serialized: &mut MlKemPublicKey<PUBLIC_KEY_SIZE>,
        ) {
            self.public_key
                .serialized_mut::<PUBLIC_KEY_SIZE>(serialized)
        }

        /// Get the serialized public key.
        #[inline(always)]
        #[requires(fstar!(r#"let ${self_} = self in
        Spec.MLKEM.is_rank $K /\
            $PUBLIC_KEY_SIZE == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K /\
            (forall (i:nat). i < v $K ==>
                Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 (Seq.index
                    ${self_.public_key.ind_cpa_public_key.t_as_ntt} i))"#))]
        #[ensures(|res|
            fstar!(r#"let ${self_} = self in
            ${res}.f_value == Seq.append (Spec.MLKEM.vector_encode_12 #$K
                            (Libcrux_ml_kem.Polynomial.to_spec_vector_t #$K #$:Vector
                                ${self_.public_key.ind_cpa_public_key.t_as_ntt}))
                        ${self_.public_key.ind_cpa_public_key.seed_for_A})"#)
        )]
        pub fn serialized_public_key<const PUBLIC_KEY_SIZE: usize>(
            &self,
        ) -> MlKemPublicKey<PUBLIC_KEY_SIZE> {
            self.public_key.serialized::<PUBLIC_KEY_SIZE>()
        }

        /// Get the serialized public key.
        #[inline(always)]
        pub fn public_key(&self) -> &MlKemPublicKeyUnpacked<K, Vector> {
            &self.public_key
        }

        /// Get the serialized public key.
        #[inline(always)]
        pub fn private_key(&self) -> &MlKemPrivateKeyUnpacked<K, Vector> {
            &self.private_key
        }

        /// Get the serialized private key.
        #[inline(always)]
        #[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
            $PRIVATE_KEY_SIZE == Spec.MLKEM.v_CCA_PRIVATE_KEY_SIZE $K /\
            $CPA_PRIVATE_KEY_SIZE == Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE $K /\
            $PUBLIC_KEY_SIZE == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K"#))]
        pub fn serialized_private_key_mut<
            const CPA_PRIVATE_KEY_SIZE: usize,
            const PRIVATE_KEY_SIZE: usize,
            const PUBLIC_KEY_SIZE: usize,
        >(
            &self,
            serialized: &mut MlKemPrivateKey<PRIVATE_KEY_SIZE>,
        ) {
            let (ind_cpa_private_key, ind_cpa_public_key) = ind_cpa::serialize_unpacked_secret_key::<
                K,
                CPA_PRIVATE_KEY_SIZE,
                PUBLIC_KEY_SIZE,
                Vector,
            >(
                &self.public_key.ind_cpa_public_key,
                &self.private_key.ind_cpa_private_key,
            );

            serialize_kem_secret_key_mut::<K, PRIVATE_KEY_SIZE, PortableHash<K>>(
                &ind_cpa_private_key,
                &ind_cpa_public_key,
                &self.private_key.implicit_rejection_value,
                &mut serialized.value,
            );
        }

        /// Get the serialized private key.
        #[inline(always)]
        #[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
            $PRIVATE_KEY_SIZE == Spec.MLKEM.v_CCA_PRIVATE_KEY_SIZE $K /\
            $CPA_PRIVATE_KEY_SIZE == Spec.MLKEM.v_CPA_PRIVATE_KEY_SIZE $K /\
            $PUBLIC_KEY_SIZE == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K"#))]
        pub fn serialized_private_key<
            const CPA_PRIVATE_KEY_SIZE: usize,
            const PRIVATE_KEY_SIZE: usize,
            const PUBLIC_KEY_SIZE: usize,
        >(
            &self,
        ) -> MlKemPrivateKey<PRIVATE_KEY_SIZE> {
            let mut sk = MlKemPrivateKey::default();
            self.serialized_private_key_mut::<CPA_PRIVATE_KEY_SIZE, PRIVATE_KEY_SIZE, PUBLIC_KEY_SIZE>(&mut sk);
            sk
        }
    }

    impl<const K: usize, Vector: Operations> Default for MlKemKeyPairUnpacked<K, Vector> {
        #[inline(always)]
        fn default() -> Self {
            Self {
                private_key: MlKemPrivateKeyUnpacked {
                    ind_cpa_private_key: IndCpaPrivateKeyUnpacked::default(),
                    implicit_rejection_value: [0u8; 32],
                },
                public_key: MlKemPublicKeyUnpacked::default(),
            }
        }
    }

    #[hax_lib::fstar::options("--z3rlimit 200")]
    #[hax_lib::fstar::before(r#"[@ "opaque_to_smt"]"#)]
    #[hax_lib::ensures(|result|
        fstar!(r#"forall (i: nat). i < v $K ==>
            (forall (j: nat). j < v $K ==>
                Seq.index (Seq.index $result i) j ==
                    Seq.index (Seq.index $ind_cpa_a j) i)"#))
    ]
    fn transpose_a<const K: usize, Vector: Operations>(
        ind_cpa_a: [[PolynomialRingElement<Vector>; K]; K],
    ) -> [[PolynomialRingElement<Vector>; K]; K] {
        // We need to un-transpose the A_transpose matrix provided by IND-CPA
        //  We would like to write the following but it is not supported by Eurydice yet.
        //  https://github.com/AeneasVerif/eurydice/issues/39
        //
        //    let A = from_fn(|i| {
        //        from_fn(|j| A_transpose[j][i])
        //    });

        #[allow(non_snake_case)]
        let mut A = from_fn(|_i| from_fn(|_j| PolynomialRingElement::<Vector>::ZERO()));
        for i in 0..K {
            hax_lib::loop_invariant!(|i: usize| {
                fstar!(
                    r#"forall (j: nat). j < v $i ==>
            (forall (k: nat). k < v $K ==>
                Seq.index (Seq.index $A j) k ==
                    Seq.index (Seq.index $ind_cpa_a k) j)"#
                )
            });
            let _a_i = A;
            for j in 0..K {
                hax_lib::loop_invariant!(|j: usize| {
                    fstar!(
                        r#"(forall (k: nat). k < v $i ==>
                    Seq.index $A k == Seq.index $_a_i k) /\
                (forall (k: nat). k < v $j ==>
                  Seq.index (Seq.index $A (v $i)) k ==
                    Seq.index (Seq.index $ind_cpa_a k) (v $i))"#
                    )
                });
                A[i][j] = ind_cpa_a[j][i].clone();
            }
        }
        A
    }

    /// Generate Unpacked Keys
    #[inline(always)]
    #[hax_lib::fstar::options("--z3rlimit 300 --ext context_pruning --split_queries always")]
    #[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
        $ETA1_RANDOMNESS_SIZE == Spec.MLKEM.v_ETA1_RANDOMNESS_SIZE $K /\
        $ETA1 == Spec.MLKEM.v_ETA1 $K /\
        $PUBLIC_KEY_SIZE == Spec.MLKEM.v_CPA_PUBLIC_KEY_SIZE $K"#))]
    #[hax_lib::ensures(|result|
        fstar!(r#"let ((m_A, public_key_hash), implicit_rejection_value), valid =
            Spec.MLKEM.ind_cca_unpack_generate_keypair $K $randomness in
        valid ==> Libcrux_ml_kem.Polynomial.to_spec_matrix_t #$K #$:Vector
            ${out}_future.f_public_key.f_ind_cpa_public_key.f_A == m_A /\
        ${out}_future.f_public_key.f_public_key_hash == public_key_hash /\
        ${out}_future.f_private_key.f_implicit_rejection_value == implicit_rejection_value"#))
    ]
    pub(crate) fn generate_keypair<
        const K: usize,
        const CPA_PRIVATE_KEY_SIZE: usize,
        const PRIVATE_KEY_SIZE: usize,
        const PUBLIC_KEY_SIZE: usize,
        const ETA1: usize,
        const ETA1_RANDOMNESS_SIZE: usize,
        Vector: Operations,
        Hasher: Hash<K>,
        Scheme: Variant,
    >(
        randomness: [u8; KEY_GENERATION_SEED_SIZE],
        out: &mut MlKemKeyPairUnpacked<K, Vector>,
    ) {
        let ind_cpa_keypair_randomness = &randomness[0..CPA_PKE_KEY_GENERATION_SEED_SIZE];
        let implicit_rejection_value = &randomness[CPA_PKE_KEY_GENERATION_SEED_SIZE..];

        generate_keypair_unpacked::<K, ETA1, ETA1_RANDOMNESS_SIZE, Vector, Hasher, Scheme>(
            ind_cpa_keypair_randomness,
            &mut out.private_key.ind_cpa_private_key,
            &mut out.public_key.ind_cpa_public_key,
        );

        #[allow(non_snake_case)]
        let A = transpose_a::<K, Vector>(out.public_key.ind_cpa_public_key.A);
        hax_lib::fstar!(
            r#"let (ind_cpa_keypair_randomness, _) = split $randomness Spec.MLKEM.v_CPA_KEY_GENERATION_SEED_SIZE in
        let ((((_, _), matrix_A_as_ntt), _), sufficient_randomness) =
            Spec.MLKEM.ind_cpa_generate_keypair_unpacked $K ind_cpa_keypair_randomness in
        let m_v_A = Libcrux_ml_kem.Polynomial.to_spec_matrix_t #$K #$:Vector $A in
        let m_f_A = Libcrux_ml_kem.Polynomial.to_spec_matrix_t #$K #$:Vector out.f_public_key.f_ind_cpa_public_key.f_A in
        let m_A:Spec.MLKEM.matrix $K = createi $K (Spec.MLKEM.matrix_A_as_ntt_i matrix_A_as_ntt) in
        assert (forall (i: nat). i < v $K ==>
            (forall (j: nat). j < v $K ==>
            Seq.index (Seq.index m_v_A i) j ==
                Seq.index (Seq.index m_f_A j) i));
        let lemma_aux (i: nat{ i < v $K }) : Lemma
            (sufficient_randomness ==> Seq.index m_v_A i == Seq.index m_A i) =
            if sufficient_randomness then
            eq_intro (Seq.index m_v_A i) (Seq.index m_A i)
        in
        Classical.forall_intro lemma_aux;
        if sufficient_randomness then
            eq_intro m_A m_v_A"#
        );
        out.public_key.ind_cpa_public_key.A = A;

        let pk_serialized = serialize_public_key::<K, PUBLIC_KEY_SIZE, Vector>(
            &out.public_key.ind_cpa_public_key.t_as_ntt,
            &out.public_key.ind_cpa_public_key.seed_for_A,
        );
        out.public_key.public_key_hash = Hasher::H(&pk_serialized);
        out.private_key.implicit_rejection_value = implicit_rejection_value.try_into().unwrap();
    }

    // Encapsulate with Unpacked Public Key
    #[inline(always)]
    #[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
        $ETA1 == Spec.MLKEM.v_ETA1 $K /\
        $ETA1_RANDOMNESS_SIZE == Spec.MLKEM.v_ETA1_RANDOMNESS_SIZE $K /\
        $ETA2 == Spec.MLKEM.v_ETA2 $K /\
        $ETA2_RANDOMNESS_SIZE == Spec.MLKEM.v_ETA2_RANDOMNESS_SIZE $K /\
        $C1_SIZE == Spec.MLKEM.v_C1_SIZE $K /\
        $C2_SIZE == Spec.MLKEM.v_C2_SIZE $K /\
        $VECTOR_U_COMPRESSION_FACTOR == Spec.MLKEM.v_VECTOR_U_COMPRESSION_FACTOR $K /\
        $VECTOR_V_COMPRESSION_FACTOR == Spec.MLKEM.v_VECTOR_V_COMPRESSION_FACTOR $K /\
        $VECTOR_U_BLOCK_LEN == Spec.MLKEM.v_C1_BLOCK_SIZE $K /\
        $CIPHERTEXT_SIZE == Spec.MLKEM.v_CPA_CIPHERTEXT_SIZE $K"#))]
    #[hax_lib::ensures(|(ciphertext_result, shared_secret_array)|
        fstar!(r#"let (ciphertext, shared_secret) =
            Spec.MLKEM.ind_cca_unpack_encapsulate $K ${public_key}.f_public_key_hash
            (Libcrux_ml_kem.Polynomial.to_spec_vector_t #$K #$:Vector ${public_key.ind_cpa_public_key.t_as_ntt})
            (Libcrux_ml_kem.Polynomial.to_spec_matrix_t #$K #$:Vector ${public_key.ind_cpa_public_key.A})
            $randomness in
        ${ciphertext_result}.f_value == ciphertext /\
        $shared_secret_array == shared_secret"#))
    ]
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
        Vector: Operations,
        Hasher: Hash<K>,
    >(
        public_key: &MlKemPublicKeyUnpacked<K, Vector>,
        randomness: &[u8; SHARED_SECRET_SIZE],
    ) -> (MlKemCiphertext<CIPHERTEXT_SIZE>, MlKemSharedSecret) {
        let hashed = encaps_prepare::<K, Hasher>(randomness, &public_key.public_key_hash);
        let (shared_secret, pseudorandomness) = hashed.split_at(SHARED_SECRET_SIZE);

        let ciphertext = ind_cpa::encrypt_unpacked::<
            K,
            CIPHERTEXT_SIZE,
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
            Vector,
            Hasher,
        >(
            &public_key.ind_cpa_public_key,
            &randomness,
            pseudorandomness,
        );
        let mut shared_secret_array = [0u8; SHARED_SECRET_SIZE];
        shared_secret_array.copy_from_slice(shared_secret);
        (MlKemCiphertext::from(ciphertext), shared_secret_array)
    }

    #[hax_lib::requires(randomness.len() == 32 && pk_hash.len() == 32)]
    #[hax_lib::ensures(|result| fstar!("result == Spec.Utils.v_G (concat randomness pk_hash)"))]
    pub(crate) fn encaps_prepare<const K: usize, Hasher: Hash<K>>(
        randomness: &[u8],
        pk_hash: &[u8],
    ) -> [u8; 64] {
        hax_lib::fstar!(
            "eq_intro (Seq.slice (
            Libcrux_ml_kem.Utils.into_padded_array (sz 64) $randomness) 0 32) $randomness"
        );
        let mut to_hash: [u8; 2 * H_DIGEST_SIZE] = into_padded_array(randomness);
        to_hash[H_DIGEST_SIZE..].copy_from_slice(pk_hash);
        hax_lib::fstar!(
            "eq_intro $to_hash (
            concat $randomness $pk_hash)"
        );

        Hasher::G(&to_hash)
    }

    // Decapsulate with Unpacked Private Key
    #[inline(always)]
    #[hax_lib::fstar::options("--z3rlimit 200 --ext context_pruning")]
    #[hax_lib::requires(fstar!(r#"Spec.MLKEM.is_rank $K /\
        $ETA1 == Spec.MLKEM.v_ETA1 $K /\
        $ETA1_RANDOMNESS_SIZE == Spec.MLKEM.v_ETA1_RANDOMNESS_SIZE $K /\
        $ETA2 == Spec.MLKEM.v_ETA2 $K /\
        $ETA2_RANDOMNESS_SIZE == Spec.MLKEM.v_ETA2_RANDOMNESS_SIZE $K /\
        $C1_SIZE == Spec.MLKEM.v_C1_SIZE $K /\
        $C2_SIZE == Spec.MLKEM.v_C2_SIZE $K /\
        $VECTOR_U_COMPRESSION_FACTOR == Spec.MLKEM.v_VECTOR_U_COMPRESSION_FACTOR $K /\
        $VECTOR_V_COMPRESSION_FACTOR == Spec.MLKEM.v_VECTOR_V_COMPRESSION_FACTOR $K /\
        $C1_BLOCK_SIZE == Spec.MLKEM.v_C1_BLOCK_SIZE $K /\
        $CIPHERTEXT_SIZE == Spec.MLKEM.v_CPA_CIPHERTEXT_SIZE $K /\
        $IMPLICIT_REJECTION_HASH_INPUT_SIZE == Spec.MLKEM.v_IMPLICIT_REJECTION_HASH_INPUT_SIZE $K"#))]
    #[hax_lib::ensures(|result|
        fstar!(r#"$result ==
            Spec.MLKEM.ind_cca_unpack_decapsulate $K ${key_pair.public_key.public_key_hash}
            ${key_pair.private_key.implicit_rejection_value}
            ${ciphertext.value}
            (Libcrux_ml_kem.Polynomial.to_spec_vector_t #$K #$:Vector ${key_pair.private_key.ind_cpa_private_key.secret_as_ntt})
            (Libcrux_ml_kem.Polynomial.to_spec_vector_t #$K #$:Vector ${key_pair.public_key.ind_cpa_public_key.t_as_ntt})
            (Libcrux_ml_kem.Polynomial.to_spec_matrix_t #$K #$:Vector ${key_pair.public_key.ind_cpa_public_key.A})"#))
    ]
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
        key_pair: &MlKemKeyPairUnpacked<K, Vector>,
        ciphertext: &MlKemCiphertext<CIPHERTEXT_SIZE>,
    ) -> MlKemSharedSecret {
        hax_lib::fstar!(
            r#"assert (v $IMPLICIT_REJECTION_HASH_INPUT_SIZE == 32 + v (Spec.MLKEM.v_CPA_CIPHERTEXT_SIZE $K));
        assert (v (Spec.MLKEM.v_C1_SIZE $K +! Spec.MLKEM.v_C2_SIZE $K) == v (Spec.MLKEM.v_C1_SIZE $K) + v (Spec.MLKEM.v_C2_SIZE $K));
        assert (v (Spec.MLKEM.v_C1_SIZE $K) == v (Spec.MLKEM.v_C1_BLOCK_SIZE $K) * v $K);
        assert (v (Spec.MLKEM.v_C1_BLOCK_SIZE $K)  == 32 * v (Spec.MLKEM.v_VECTOR_U_COMPRESSION_FACTOR $K));
        assert (v (Spec.MLKEM.v_C2_SIZE $K) == 32 * v (Spec.MLKEM.v_VECTOR_V_COMPRESSION_FACTOR $K))"#
        );
        let decrypted = ind_cpa::decrypt_unpacked::<
            K,
            CIPHERTEXT_SIZE,
            C1_SIZE,
            VECTOR_U_COMPRESSION_FACTOR,
            VECTOR_V_COMPRESSION_FACTOR,
            Vector,
        >(&key_pair.private_key.ind_cpa_private_key, &ciphertext.value);

        let mut to_hash: [u8; SHARED_SECRET_SIZE + H_DIGEST_SIZE] = into_padded_array(&decrypted);
        hax_lib::fstar!(r#"eq_intro (Seq.slice $to_hash 0 32) $decrypted"#);
        to_hash[SHARED_SECRET_SIZE..].copy_from_slice(&key_pair.public_key.public_key_hash);
        hax_lib::fstar!(
            r#"lemma_slice_append $to_hash $decrypted ${key_pair}.f_public_key.f_public_key_hash"#
        );

        let hashed = Hasher::G(&to_hash);
        let (shared_secret, pseudorandomness) = hashed.split_at(SHARED_SECRET_SIZE);

        let mut to_hash: [u8; IMPLICIT_REJECTION_HASH_INPUT_SIZE] =
            into_padded_array(&key_pair.private_key.implicit_rejection_value);
        hax_lib::fstar!(
            "eq_intro
            (Seq.slice $to_hash 0 32) ${key_pair}.f_private_key.f_implicit_rejection_value"
        );
        to_hash[SHARED_SECRET_SIZE..].copy_from_slice(ciphertext.as_ref());
        hax_lib::fstar!(
            "lemma_slice_append $to_hash ${key_pair}.f_private_key.f_implicit_rejection_value ${ciphertext}.f_value"
        );
        let implicit_rejection_shared_secret: [u8; SHARED_SECRET_SIZE] = Hasher::PRF(&to_hash);

        let expected_ciphertext = ind_cpa::encrypt_unpacked::<
            K,
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
            Vector,
            Hasher,
        >(
            &key_pair.public_key.ind_cpa_public_key,
            &decrypted,
            pseudorandomness,
        );

        let selector =
            compare_ciphertexts_in_constant_time(ciphertext.as_ref(), &expected_ciphertext);

        select_shared_secret_in_constant_time(
            shared_secret,
            &implicit_rejection_shared_secret,
            selector,
        )
    }
}

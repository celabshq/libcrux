//! ML-KEM 768

use super::{constants::*, ind_cca::*, types::*, *};

const RANK: usize = 3;
#[cfg(any(feature = "incremental", eurydice))]
const RANKED_BYTES_PER_RING_ELEMENT: usize = RANK * BITS_PER_RING_ELEMENT / 8;
const T_AS_NTT_ENCODED_SIZE: usize =
    (RANK * COEFFICIENTS_IN_RING_ELEMENT * BITS_PER_COEFFICIENT) / 8;
const VECTOR_U_COMPRESSION_FACTOR: usize = 10;
// [hax]: hacspec/hacspec-v2#27 stealing error
// block_len::<VECTOR_U_COMPRESSION_FACTOR>()
const C1_BLOCK_SIZE: usize = (COEFFICIENTS_IN_RING_ELEMENT * VECTOR_U_COMPRESSION_FACTOR) / 8;
// [hax]: hacspec/hacspec-v2#27 stealing error
//  serialized_len::<RANK, C1_BLOCK_SIZE>();
const C1_SIZE: usize = C1_BLOCK_SIZE * RANK;
const VECTOR_V_COMPRESSION_FACTOR: usize = 4;
// [hax]: hacspec/hacspec-v2#27 stealing error
//  block_len::<VECTOR_V_COMPRESSION_FACTOR>()
const C2_SIZE: usize = (COEFFICIENTS_IN_RING_ELEMENT * VECTOR_V_COMPRESSION_FACTOR) / 8;
const CPA_PKE_SECRET_KEY_SIZE: usize =
    (RANK * COEFFICIENTS_IN_RING_ELEMENT * BITS_PER_COEFFICIENT) / 8;
pub(crate) const CPA_PKE_PUBLIC_KEY_SIZE: usize = T_AS_NTT_ENCODED_SIZE + 32;
// These two are used in the hybrid kem. This could probably be improved.
const CPA_PKE_CIPHERTEXT_SIZE: usize = C1_SIZE + C2_SIZE;
const SECRET_KEY_SIZE: usize =
    CPA_PKE_SECRET_KEY_SIZE + CPA_PKE_PUBLIC_KEY_SIZE + H_DIGEST_SIZE + SHARED_SECRET_SIZE;

const ETA1: usize = 2;
const ETA1_RANDOMNESS_SIZE: usize = ETA1 * 64;
const ETA2: usize = 2;
const ETA2_RANDOMNESS_SIZE: usize = ETA2 * 64;

const IMPLICIT_REJECTION_HASH_INPUT_SIZE: usize = SHARED_SECRET_SIZE + CPA_PKE_CIPHERTEXT_SIZE;

/// The ML-KEM 768 algorithms
pub struct MlKem768;

#[cfg(not(eurydice))]
crate::impl_kem_trait!(
    MlKem768,
    MlKem768PublicKey,
    MlKem768PrivateKey,
    MlKem768Ciphertext
);

// Provide the (packed) PQCP APIs
#[cfg(feature = "pqcp")]
crate::pqcp::pqcp_api!("use libcrux_ml_kem::mlkem768::pqcp::*;", MlKem768, " 768 ");

/// An ML-KEM 768 Ciphertext
pub type MlKem768Ciphertext = MlKemCiphertext<CPA_PKE_CIPHERTEXT_SIZE>;
/// An ML-KEM 768 Private key
pub type MlKem768PrivateKey = MlKemPrivateKey<SECRET_KEY_SIZE>;
/// An ML-KEM 768 Public key
pub type MlKem768PublicKey = MlKemPublicKey<CPA_PKE_PUBLIC_KEY_SIZE>;
/// An ML-KEM 768 Key pair
pub type MlKem768KeyPair = MlKemKeyPair<SECRET_KEY_SIZE, CPA_PKE_PUBLIC_KEY_SIZE>;

// Instantiate the different functions.
macro_rules! instantiate {
    ($modp:ident, $p:path, $doc:expr) => {
        #[doc = $doc]
        pub mod $modp {
            use super::*;
            use $p as p;

            /// Validate a public key.
            ///
            /// Returns `true` if valid, and `false` otherwise.
            pub fn validate_public_key(public_key: &MlKem768PublicKey) -> bool {
                p::validate_public_key::<
                    RANK,
                    CPA_PKE_PUBLIC_KEY_SIZE,
                >(&public_key.value)
            }

            /// Validate a private key.
            ///
            /// Returns `true` if valid, and `false` otherwise.
            pub fn validate_private_key(
                private_key: &MlKem768PrivateKey,
                ciphertext: &MlKem768Ciphertext,
            ) -> bool {
                p::validate_private_key::<
                    RANK,
                    SECRET_KEY_SIZE,
                    CPA_PKE_CIPHERTEXT_SIZE,
                >(private_key, ciphertext)
            }

            /// Validate the private key only.
            ///
            /// Returns `true` if valid, and `false` otherwise.
            pub fn validate_private_key_only(
                private_key: &MlKem768PrivateKey,
            ) -> bool {
                p::validate_private_key_only::<
                    RANK,
                    SECRET_KEY_SIZE,
                >(private_key)
            }

            /// Generate ML-KEM 768 Key Pair
            pub fn generate_key_pair(
                randomness: [u8; KEY_GENERATION_SEED_SIZE],
            ) -> MlKem768KeyPair {
                p::generate_keypair::<
                    RANK,
                    CPA_PKE_SECRET_KEY_SIZE,
                    SECRET_KEY_SIZE,
                    CPA_PKE_PUBLIC_KEY_SIZE,
                    ETA1,
                    ETA1_RANDOMNESS_SIZE,
                >(&randomness)
            }

            /// Generate Kyber 768 Key Pair
            #[cfg(feature = "kyber")]
            #[cfg_attr(docsrs, doc(cfg(feature = "kyber")))]
            pub fn kyber_generate_key_pair(
                randomness: [u8; KEY_GENERATION_SEED_SIZE],
            ) -> MlKem768KeyPair {
                p::kyber_generate_keypair::<
                    RANK,
                    CPA_PKE_SECRET_KEY_SIZE,
                    SECRET_KEY_SIZE,
                    CPA_PKE_PUBLIC_KEY_SIZE,
                    ETA1,
                    ETA1_RANDOMNESS_SIZE,
                >(&randomness)
            }

            /// Encapsulate ML-KEM 768
            ///
            /// Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
            /// The input is a reference to an [`MlKem768PublicKey`] and [`SHARED_SECRET_SIZE`]
            /// bytes of `randomness`.
            pub fn encapsulate(
                public_key: &MlKem768PublicKey,
                randomness: [u8; SHARED_SECRET_SIZE],
            ) -> (MlKem768Ciphertext, MlKemSharedSecret) {
                p::encapsulate::<
                    RANK,
                    CPA_PKE_CIPHERTEXT_SIZE,
                    CPA_PKE_PUBLIC_KEY_SIZE,
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
                >(public_key, &randomness)
            }

            /// Encapsulate Kyber 768
            ///
            /// Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
            /// The input is a reference to an [`MlKem768PublicKey`] and [`SHARED_SECRET_SIZE`]
            /// bytes of `randomness`.
            #[cfg(feature = "kyber")]
            #[cfg_attr(docsrs, doc(cfg(feature = "kyber")))]
            pub fn kyber_encapsulate(
                public_key: &MlKem768PublicKey,
                randomness: [u8; SHARED_SECRET_SIZE],
            ) -> (MlKem768Ciphertext, MlKemSharedSecret) {
                p::kyber_encapsulate::<
                    RANK,
                    CPA_PKE_CIPHERTEXT_SIZE,
                    CPA_PKE_PUBLIC_KEY_SIZE,
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
                >(public_key, &randomness)
            }

            /// Decapsulate ML-KEM 768
            ///
            /// Generates an [`MlKemSharedSecret`].
            /// The input is a reference to an [`MlKem768PrivateKey`] and an [`MlKem768Ciphertext`].
            pub fn decapsulate(
                private_key: &MlKem768PrivateKey,
                ciphertext: &MlKem768Ciphertext,
            ) -> MlKemSharedSecret {
                p::decapsulate::<
                    RANK,
                    SECRET_KEY_SIZE,
                    CPA_PKE_SECRET_KEY_SIZE,
                    CPA_PKE_PUBLIC_KEY_SIZE,
                    CPA_PKE_CIPHERTEXT_SIZE,
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

            /// Decapsulate Kyber 768
            ///
            /// Generates an [`MlKemSharedSecret`].
            /// The input is a reference to an [`MlKem768PrivateKey`] and an [`MlKem768Ciphertext`].
            #[cfg(feature = "kyber")]
            #[cfg_attr(docsrs, doc(cfg(feature = "kyber")))]
            pub fn kyber_decapsulate(
                private_key: &MlKem768PrivateKey,
                ciphertext: &MlKem768Ciphertext,
            ) -> MlKemSharedSecret {
                p::kyber_decapsulate::<
                    RANK,
                    SECRET_KEY_SIZE,
                    CPA_PKE_SECRET_KEY_SIZE,
                    CPA_PKE_PUBLIC_KEY_SIZE,
                    CPA_PKE_CIPHERTEXT_SIZE,
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

            /// Unpacked APIs that don't use serialized keys.
            pub mod unpacked {
                use super::*;

                /// An Unpacked ML-KEM 768 Public key
                pub type MlKem768PublicKeyUnpacked = p::unpacked::MlKemPublicKeyUnpacked<RANK>;

                /// Am Unpacked ML-KEM 768 Key pair
                pub type MlKem768KeyPairUnpacked = p::unpacked::MlKemKeyPairUnpacked<RANK>;

                #[cfg(feature = "pqcp")]
                crate::pqcp::pqcp_unpacked_api!(
                    MlKem768KeyPairUnpacked,
                    MlKem768PublicKeyUnpacked,
                    MlKem768PrivateKey,
                    MlKem768PublicKey,
                    MlKem768Ciphertext,
                    " 768 "
                );

                /// Create a new, empty unpacked key.
                pub fn init_key_pair() -> MlKem768KeyPairUnpacked {
                    MlKem768KeyPairUnpacked::default()
                }

                /// Create a new, empty unpacked public key.
                pub fn init_public_key() -> MlKem768PublicKeyUnpacked {
                    MlKem768PublicKeyUnpacked::default()
                }

                /// Get the serialized public key.
                #[hax_lib::requires(fstar!(r#"forall (i:nat). i < 3 ==>
                    Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 (Seq.index 
                        ${public_key.ind_cpa_public_key.t_as_ntt} i)"#))]
                pub fn serialized_public_key(public_key: &MlKem768PublicKeyUnpacked, serialized : &mut MlKem768PublicKey) {
                    public_key.serialized_mut::<CPA_PKE_PUBLIC_KEY_SIZE>(serialized);
                }

                /// Get the serialized private key.
                pub fn key_pair_serialized_private_key(key_pair: &MlKem768KeyPairUnpacked) -> MlKem768PrivateKey {
                    key_pair.serialized_private_key::<CPA_PKE_SECRET_KEY_SIZE, SECRET_KEY_SIZE, CPA_PKE_PUBLIC_KEY_SIZE>()
                }

                /// Get the serialized private key.
                pub fn key_pair_serialized_private_key_mut(key_pair: &MlKem768KeyPairUnpacked, serialized: &mut MlKem768PrivateKey) {
                    key_pair.serialized_private_key_mut::<CPA_PKE_SECRET_KEY_SIZE, SECRET_KEY_SIZE, CPA_PKE_PUBLIC_KEY_SIZE>(serialized);
                }

                /// Get the serialized public key.
                #[hax_lib::requires(fstar!(r#"(forall (i:nat). i < 3 ==>
                        Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 (Seq.index 
                            ${key_pair.public_key.ind_cpa_public_key.t_as_ntt} i))"#))]
                pub fn key_pair_serialized_public_key_mut(key_pair: &MlKem768KeyPairUnpacked, serialized: &mut MlKem768PublicKey) {
                    key_pair.serialized_public_key_mut::<CPA_PKE_PUBLIC_KEY_SIZE>(serialized);
                }

                /// Get the serialized public key.
                #[hax_lib::requires(fstar!(r#"forall (i:nat). i < 3 ==>
                    Libcrux_ml_kem.Polynomial.is_bounded_poly 3328 (Seq.index 
                        ${key_pair.public_key.ind_cpa_public_key.t_as_ntt} i)"#))]
                pub fn key_pair_serialized_public_key(key_pair: &MlKem768KeyPairUnpacked) ->MlKem768PublicKey {
                    key_pair.serialized_public_key::<CPA_PKE_PUBLIC_KEY_SIZE>()
                }

                /// Get an unpacked key from a private key.
                pub fn key_pair_from_private_mut(private_key: &MlKem768PrivateKey, key_pair: &mut MlKem768KeyPairUnpacked) {
                    p::unpacked::keypair_from_private_key::<RANK, SECRET_KEY_SIZE, CPA_PKE_SECRET_KEY_SIZE, CPA_PKE_PUBLIC_KEY_SIZE, T_AS_NTT_ENCODED_SIZE>(private_key, key_pair);
                }

                /// Get the unpacked public key.
                pub fn public_key(key_pair: &MlKem768KeyPairUnpacked, pk: &mut MlKem768PublicKeyUnpacked) {
                    *pk = (*key_pair.public_key()).clone();
                }

                /// Get the unpacked public key.
                pub fn unpacked_public_key(
                    public_key: &MlKem768PublicKey,
                    unpacked_public_key: &mut MlKem768PublicKeyUnpacked
                ) {
                    p::unpacked::unpack_public_key::<
                        RANK,
                        T_AS_NTT_ENCODED_SIZE,
                        CPA_PKE_PUBLIC_KEY_SIZE,
                    >(public_key, unpacked_public_key)
                }

                /// Generate ML-KEM 768 Key Pair in "unpacked" form.
                pub fn generate_key_pair(
                    randomness: [u8; KEY_GENERATION_SEED_SIZE]
                ) ->  MlKem768KeyPairUnpacked {
                    let mut key_pair = MlKem768KeyPairUnpacked::default();
                    generate_key_pair_mut(randomness, &mut key_pair);
                    key_pair
                }

                /// Generate ML-KEM 768 Key Pair in "unpacked" form.
                pub fn generate_key_pair_mut(
                    randomness: [u8; KEY_GENERATION_SEED_SIZE],
                    key_pair: &mut MlKem768KeyPairUnpacked,
                ) {
                    p::unpacked::generate_keypair::<
                        RANK,
                        CPA_PKE_SECRET_KEY_SIZE,
                        SECRET_KEY_SIZE,
                        CPA_PKE_PUBLIC_KEY_SIZE,
                        ETA1,
                        ETA1_RANDOMNESS_SIZE,
                    >(randomness, key_pair);
                }

                /// Encapsulate ML-KEM 768 (unpacked)
                ///
                /// Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
                /// The input is a reference to an unpacked public key of type [`MlKem768PublicKeyUnpacked`],
                /// the SHA3-256 hash of this public key, and [`SHARED_SECRET_SIZE`] bytes of `randomness`.
                #[cfg_attr(
                    hax,
                    hax_lib::fstar::before(
                        interface,
                        "
                let _ =
                (* This module has implicit dependencies, here we make them explicit. *)
                (* The implicit dependencies arise from typeclasses instances. *)
                let open Libcrux_ml_kem.Vector.Portable in
                let open Libcrux_ml_kem.Vector.Neon in
                ()"
                    )
                )]
                pub fn encapsulate(
                    public_key: &MlKem768PublicKeyUnpacked,
                    randomness: [u8; SHARED_SECRET_SIZE],
                ) -> (MlKem768Ciphertext, MlKemSharedSecret) {
                    p::unpacked::encapsulate::<
                        RANK,
                        CPA_PKE_CIPHERTEXT_SIZE,
                        CPA_PKE_PUBLIC_KEY_SIZE,
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
                    >(public_key, &randomness)
                }

                /// Decapsulate ML-KEM 768 (unpacked)
                ///
                /// Generates an [`MlKemSharedSecret`].
                /// The input is a reference to an unpacked key pair of type [`MlKem768KeyPairUnpacked`]
                /// and an [`MlKem768Ciphertext`].
                pub fn decapsulate(
                    private_key: &MlKem768KeyPairUnpacked,
                    ciphertext: &MlKem768Ciphertext,
                ) -> MlKemSharedSecret {
                    p::unpacked::decapsulate::<
                        RANK,
                        SECRET_KEY_SIZE,
                        CPA_PKE_SECRET_KEY_SIZE,
                        CPA_PKE_PUBLIC_KEY_SIZE,
                        CPA_PKE_CIPHERTEXT_SIZE,
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
        }
    };
}

// Instantiations

instantiate! {portable, ind_cca::instantiations::portable, "Portable ML-KEM 768"}
#[cfg(feature = "simd256")]
instantiate! {avx2, ind_cca::instantiations::avx2, "AVX2 Optimised ML-KEM 768"}
#[cfg(feature = "simd128")]
instantiate! {neon, ind_cca::instantiations::neon, "Neon Optimised ML-KEM 768"}

/// Validate a public key.
///
/// Returns `true` if valid, and `false` otherwise.
#[cfg(not(eurydice))]
pub fn validate_public_key(public_key: &MlKem768PublicKey) -> bool {
    multiplexing::validate_public_key::<RANK, CPA_PKE_PUBLIC_KEY_SIZE>(&public_key.value)
}

/// Validate a private key.
///
/// Returns `true` if valid, and `false` otherwise.
#[cfg(not(eurydice))]
pub fn validate_private_key(
    private_key: &MlKem768PrivateKey,
    ciphertext: &MlKem768Ciphertext,
) -> bool {
    multiplexing::validate_private_key::<RANK, SECRET_KEY_SIZE, CPA_PKE_CIPHERTEXT_SIZE>(
        private_key,
        ciphertext,
    )
}

/// Generate ML-KEM 768 Key Pair
///
/// Generate an ML-KEM key pair. The input is a byte array of size
/// [`KEY_GENERATION_SEED_SIZE`].
///
/// This function returns an [`MlKem768KeyPair`].
#[cfg(not(eurydice))]
pub fn generate_key_pair(randomness: [u8; KEY_GENERATION_SEED_SIZE]) -> MlKem768KeyPair {
    multiplexing::generate_keypair::<
        RANK,
        CPA_PKE_SECRET_KEY_SIZE,
        SECRET_KEY_SIZE,
        CPA_PKE_PUBLIC_KEY_SIZE,
        ETA1,
        ETA1_RANDOMNESS_SIZE,
    >(&randomness)
}

/// Encapsulate ML-KEM 768
///
/// Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
/// The input is a reference to an [`MlKem768PublicKey`] and [`SHARED_SECRET_SIZE`]
/// bytes of `randomness`.
#[cfg(not(eurydice))]
pub fn encapsulate(
    public_key: &MlKem768PublicKey,
    randomness: [u8; SHARED_SECRET_SIZE],
) -> (MlKem768Ciphertext, MlKemSharedSecret) {
    multiplexing::encapsulate::<
        RANK,
        CPA_PKE_CIPHERTEXT_SIZE,
        CPA_PKE_PUBLIC_KEY_SIZE,
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
    >(public_key, &randomness)
}

/// Decapsulate ML-KEM 768
///
/// Generates an [`MlKemSharedSecret`].
/// The input is a reference to an [`MlKem768PrivateKey`] and an [`MlKem768Ciphertext`].
#[cfg(not(eurydice))]
pub fn decapsulate(
    private_key: &MlKem768PrivateKey,
    ciphertext: &MlKem768Ciphertext,
) -> MlKemSharedSecret {
    multiplexing::decapsulate::<
        RANK,
        SECRET_KEY_SIZE,
        CPA_PKE_SECRET_KEY_SIZE,
        CPA_PKE_PUBLIC_KEY_SIZE,
        CPA_PKE_CIPHERTEXT_SIZE,
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

/// Randomized APIs
///
/// The functions in this module are equivalent to the one in the main module,
/// but sample their own randomness, provided a random number generator that
/// implements `CryptoRng`.
///
/// Decapsulation is not provided in this module as it does not require randomness.
#[cfg(all(not(eurydice), feature = "rand"))]
pub mod rand {
    use super::{
        MlKem768Ciphertext, MlKem768KeyPair, MlKem768PublicKey, MlKemSharedSecret,
        KEY_GENERATION_SEED_SIZE, SHARED_SECRET_SIZE,
    };
    use ::rand::CryptoRng;

    /// Generate ML-KEM 768 Key Pair
    ///
    /// The random number generator `rng` needs to implement `CryptoRng`
    /// to sample the required randomness internally.
    ///
    /// This function returns an [`MlKem768KeyPair`].
    pub fn generate_key_pair(rng: &mut impl CryptoRng) -> MlKem768KeyPair {
        let mut randomness = [0u8; KEY_GENERATION_SEED_SIZE];
        rng.fill_bytes(&mut randomness);

        super::generate_key_pair(randomness)
    }

    /// Encapsulate ML-KEM 768
    ///
    /// Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
    /// The input is a reference to an [`MlKem768PublicKey`].
    /// The random number generator `rng` needs to implement `CryptoRng`
    /// to sample the required randomness internally.
    pub fn encapsulate(
        public_key: &MlKem768PublicKey,
        rng: &mut impl CryptoRng,
    ) -> (MlKem768Ciphertext, MlKemSharedSecret) {
        let mut randomness = [0u8; SHARED_SECRET_SIZE];
        rng.fill_bytes(&mut randomness);

        super::encapsulate(public_key, randomness)
    }
}

#[cfg(all(not(eurydice), feature = "kyber"))]
pub(crate) mod kyber {
    use super::*;

    /// The Kyber 768 algorithms
    pub struct Kyber768;

    crate::impl_kem_trait!(
        Kyber768,
        MlKem768PublicKey,
        MlKem768PrivateKey,
        MlKem768Ciphertext
    );

    /// Generate Kyber 768 Key Pair
    ///
    /// Generate a Kyber key pair. The input is a byte array of size
    /// [`KEY_GENERATION_SEED_SIZE`].
    ///
    /// This function returns an [`MlKem768KeyPair`].
    pub fn generate_key_pair(randomness: [u8; KEY_GENERATION_SEED_SIZE]) -> MlKem768KeyPair {
        multiplexing::kyber_generate_keypair::<
            RANK,
            CPA_PKE_SECRET_KEY_SIZE,
            SECRET_KEY_SIZE,
            CPA_PKE_PUBLIC_KEY_SIZE,
            ETA1,
            ETA1_RANDOMNESS_SIZE,
        >(randomness)
    }

    /// Encapsulate Kyber 768
    ///
    /// Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
    /// The input is a reference to an [`MlKem768PublicKey`] and [`SHARED_SECRET_SIZE`]
    /// bytes of `randomness`.
    pub fn encapsulate(
        public_key: &MlKem768PublicKey,
        randomness: [u8; SHARED_SECRET_SIZE],
    ) -> (MlKem768Ciphertext, MlKemSharedSecret) {
        multiplexing::kyber_encapsulate::<
            RANK,
            CPA_PKE_CIPHERTEXT_SIZE,
            CPA_PKE_PUBLIC_KEY_SIZE,
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

    /// Decapsulate ML-KEM 768
    ///
    /// Generates an [`MlKemSharedSecret`].
    /// The input is a reference to an [`MlKem768PrivateKey`] and an [`MlKem768Ciphertext`].
    pub fn decapsulate(
        private_key: &MlKem768PrivateKey,
        ciphertext: &MlKem768Ciphertext,
    ) -> MlKemSharedSecret {
        multiplexing::kyber_decapsulate::<
            RANK,
            SECRET_KEY_SIZE,
            CPA_PKE_SECRET_KEY_SIZE,
            CPA_PKE_PUBLIC_KEY_SIZE,
            CPA_PKE_CIPHERTEXT_SIZE,
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

/// # Incremental API.
///
/// **NOTE:** This is a non-standard API. Use with caution!
///
/// ## Serialized keys
/// ```
/// #[cfg(feature = "rand")]
/// {
/// use libcrux_ml_kem::mlkem768::incremental::*;
///
/// // USE ONLY CRYPTOGRAPHICALLY SECURE RANDOMNESS OR `generate`
/// let randomness = [0x13; 64];
/// let key_pair = KeyPairBytes::from_seed(randomness);
///
/// // Get pk1 and pk2 to send to the other party.
/// let pk1 = key_pair.pk1();
/// let pk2 = key_pair.pk2();
///
/// // On the receiver, encapsulate to the public keys.
/// // Check the public key for consistency.
/// assert!(validate_pk_bytes(pk1, pk2).is_ok());
///
/// let mut encaps_state = [0u8; encaps_state_len()];
/// let mut encaps_shared_secret = [0u8; shared_secret_size()];
/// let randomness = [0xAF; 32];
/// let ct1 = encapsulate1(
///     pk1,
///     randomness,
///     &mut encaps_state,
///     &mut encaps_shared_secret,
/// )
/// .unwrap();
///
/// let ct2 = encapsulate2(&encaps_state, &pk2);
///
/// // Decapsulate the shared secret after receiving ct1 and ct2.
/// let shared_secret = decapsulate_incremental_key(key_pair.as_ref(), &ct1, &ct2).unwrap();
///
/// assert_eq!(shared_secret, encaps_shared_secret);
/// }
/// ```
///
/// ## Compressed keys and randomness
/// ```
/// #[cfg(feature = "rand")]
/// {
/// use libcrux_ml_kem::mlkem768::incremental::*;
///
/// // Use a n RNG that is safe to use for cryptography.
/// // THIS ONE IS NOT!
/// let mut rng = ::rand::rng();
///
/// let key_pair = KeyPairCompressedBytes::generate(&mut rng);
///
/// // Get pk1 and pk2 to send to the other party.
/// let pk1 = key_pair.pk1();
/// let pk2 = key_pair.pk2();
///
/// // On the receiver, encapsulate to the public keys.
/// // Check the public key for consistency.
/// assert!(validate_pk_bytes(pk1, pk2).is_ok());
///
/// let mut encaps_state = [0u8; encaps_state_len()];
/// let mut encaps_shared_secret = [0u8; shared_secret_size()];
/// let ct1 = rand::encapsulate1(
///     pk1,
///     &mut rng,
///     &mut encaps_state,
///     &mut encaps_shared_secret,
/// )
/// .unwrap();
///
/// let ct2 = encapsulate2(&encaps_state, pk2);
///
/// // Decapsulate the shared secret after receiving ct1 and ct2.
/// let shared_secret = decapsulate_compressed_key(key_pair.sk(), &ct1, &ct2);
///
/// assert_eq!(shared_secret, encaps_shared_secret);
/// }
/// ```
#[cfg(all(not(eurydice), feature = "incremental"))]
pub mod incremental {
    use crate::mlkem::impl_incr_key_size;

    impl_incr_key_size!();
}

#[cfg(test)]
mod tests {
    use rand::{rngs::OsRng, TryRngCore};

    use super::{
        mlkem768::{generate_key_pair, validate_public_key},
        KEY_GENERATION_SEED_SIZE,
    };

    #[test]
    fn pk_validation() {
        let mut randomness = [0u8; KEY_GENERATION_SEED_SIZE];
        OsRng.try_fill_bytes(&mut randomness).unwrap();

        let key_pair = generate_key_pair(randomness);
        assert!(validate_public_key(&key_pair.pk));
    }
}

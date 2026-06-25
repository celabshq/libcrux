#![cfg(feature = "incremental")]

//! Top level entry points for ML-KEM
//!
//! For now this is only used for the incremental API.

// This should be replaced with a nicer proc macro later.

macro_rules! impl_incr_key_size {
    () => {
        use crate::ind_cca::incremental::multiplexing;

        use self::incremental::types::{self, Error};
        pub use self::incremental::types::{PublicKey1, PublicKey2};

        use super::*;
        use ind_cca::incremental;

        /// Ciphertext 1
        pub type Ciphertext1 = types::Ciphertext1<C1_SIZE>;

        /// Ciphertext 2
        pub type Ciphertext2 = types::Ciphertext2<C2_SIZE>;

        /// Get the size of the first public key in bytes.
        #[hax_lib::ensures(|result| result == 64)]
        pub const fn pk1_len() -> usize {
            PublicKey1::len()
        }

        /// Get the size of the second public key in bytes.
        #[hax_lib::ensures(|result| result == RANKED_BYTES_PER_RING_ELEMENT)]
        pub const fn pk2_len() -> usize {
            RANKED_BYTES_PER_RING_ELEMENT
        }

        /// The size of a compressed key pair in bytes.
        pub const COMPRESSED_KEYPAIR_LEN: usize = SECRET_KEY_SIZE;

        /// The size of the key pair in bytes.
        pub const fn key_pair_len() -> usize {
            // Because const generics are too limited, we compute it here from scratch.

            // PK1
            pk1_len()
            // PK2
            + pk2_len()
            // SK
            + RANK * 16 * 32 + 32
            // Matrix
            + RANK * RANK * 16 * 32
        }


        /// The size of the compressed key pair in bytes.
        pub const fn key_pair_compressed_len() -> usize {
            COMPRESSED_KEYPAIR_LEN
        }

        /// The size of the encaps state in bytes.
        pub const fn encaps_state_len() -> usize {
            // Because const generics are too limited, we compute it here from scratch.

            // r_as_ntt
            RANK * 16 * 32
            // error2
            + 16 * 32
            // randomness
            + 32
        }

        /// The size of the shared secret.
        pub const fn shared_secret_size() -> usize {
            SHARED_SECRET_SIZE
        }

        /// Functions in this module require an allocator to use [`Box`].
        ///
        /// Instead of serializing keys and state, the functions in this module return
        /// the platform dependent keys and state types for immediate use.
        #[cfg(feature = "alloc")]
        pub mod alloc {
            use super::*;
            use super::incremental::types::alloc::{State, Keys};
            use ::alloc::boxed::Box;

            /// Generate a new key pair for incremental encapsulation.
            pub fn generate_key_pair(randomness: [u8; KEY_GENERATION_SEED_SIZE]) -> Box<dyn Keys> {
                multiplexing::alloc::generate_keypair::<
                    RANK,
                    CPA_PKE_SECRET_KEY_SIZE,
                    SECRET_KEY_SIZE,
                    CPA_PKE_PUBLIC_KEY_SIZE,
                    RANKED_BYTES_PER_RING_ELEMENT,
                    ETA1,
                    ETA1_RANDOMNESS_SIZE,
                >(randomness)
            }

            /// Encapsulate the first part of the ciphertext.
            pub fn encapsulate1(
                public_key_part: &PublicKey1,
                randomness: [u8; SHARED_SECRET_SIZE],
            ) -> (Ciphertext1, Box<dyn State>, [u8; SHARED_SECRET_SIZE]) {
                multiplexing::alloc::encapsulate1::<
                    RANK,
                    CPA_PKE_CIPHERTEXT_SIZE,
                    C1_SIZE,
                    VECTOR_U_COMPRESSION_FACTOR,
                    C1_BLOCK_SIZE,
                    ETA1,
                    ETA1_RANDOMNESS_SIZE,
                    ETA2,
                    ETA2_RANDOMNESS_SIZE,
                >(public_key_part, randomness)
            }

            /// Encapsulate the second part of the ciphertext.
            ///
            /// The second part of the public key is passed in as byte slice.
            /// [`Error::InvalidInputLength`] is returned if `public_key_part` is too
            /// short.
            pub fn encapsulate2(state: &dyn State, public_key_part: &[u8]) -> Result<Ciphertext2, Error> {
                multiplexing::alloc::encapsulate2::<RANK, RANKED_BYTES_PER_RING_ELEMENT, C2_SIZE, VECTOR_V_COMPRESSION_FACTOR>(
                    state,
                    public_key_part,
                )
            }

            /// Decapsulate incremental ciphertexts.
            pub fn decapsulate(
                private_key: &dyn Keys,
                ciphertext1: &Ciphertext1,
                ciphertext2: &Ciphertext2,
            ) -> MlKemSharedSecret {
                multiplexing::alloc::decapsulate::<
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
                >(private_key, ciphertext1, ciphertext2)
            }
        }

        /// An encoded, incremental key pair.
        pub struct KeyPairBytes {
            value: [u8; key_pair_len()]
        }

        #[cfg(all(not(eurydice), feature = "rand"))]
        use ::rand::{CryptoRng, RngCore};

        impl KeyPairBytes {
            /// Generate a new key pair.
            /// This uses unpacked keys and does not compress the keys.
            pub fn from_seed(randomness: [u8; KEY_GENERATION_SEED_SIZE]) -> Self {
                let mut out = Self {
                    value: [0u8; key_pair_len()]
                };
                generate_key_pair(randomness, &mut out.value).unwrap();
                out
            }

            /// Generate a new key pair.
            /// This uses unpacked keys and does not compress the keys.
            #[cfg(all(not(eurydice), feature = "rand"))]
            pub fn generate(rng: &mut (impl RngCore + CryptoRng)) -> Self {
                let mut randomness = [0u8; KEY_GENERATION_SEED_SIZE];
                rng.fill_bytes(&mut randomness);
                let mut out = Self {
                    value: [0u8; key_pair_len()]
                };
                generate_key_pair(randomness, &mut out.value).unwrap();
                out
            }

            /// Get the raw bytes.
            pub fn to_bytes(self) -> [u8; key_pair_len()] {
                self.value
            }

            /// Get the PK1 bytes from the serialized key pair bytes
            pub fn pk1(&self) -> &[u8; pk1_len()] {
                // The unwrap here is ok because that's exactly what we take
                // and we know that `self.value` is long enough.
                <&[u8; pk1_len()]>::try_from(&self.value[0..pk1_len()]).unwrap()
            }

            /// Get the PK2 bytes from the serialized key pair bytes
            pub fn pk2(&self) -> &[u8; pk2_len()] {
                // The unwrap here is ok because that's exactly what we take
                // and we know that `self.value` is long enough.
                <&[u8; pk2_len()]>::try_from(&self.value[pk1_len()..pk1_len() + pk2_len()]).unwrap()
            }
        }

        impl AsRef<[u8]> for KeyPairBytes {
            fn as_ref(&self) -> &[u8] {
                &self.value
            }
        }

        /// Generate a key pair and write it into `key_pair`.
        /// This uses unpacked keys and does not compress the keys.
        ///
        /// `key_pair.len()` must be of size `key_pair_len()`.
        /// The function returns an error if this is not the case.
        #[hax_lib::ensures(|result|
            hax_lib::implies(
                key_pair.len() >= 64 + RANKED_BYTES_PER_RING_ELEMENT + RANK * 512 + 32 + RANK * RANK * 512,
                result.is_ok(),
            ) & (future(key_pair).len() == key_pair.len()))]
        pub fn generate_key_pair(randomness: [u8; KEY_GENERATION_SEED_SIZE], key_pair: &mut [u8]) -> Result<(), Error> {
            multiplexing::generate_keypair::<
                RANK,
                RANKED_BYTES_PER_RING_ELEMENT,
                CPA_PKE_SECRET_KEY_SIZE,
                SECRET_KEY_SIZE,
                CPA_PKE_PUBLIC_KEY_SIZE,
                RANKED_BYTES_PER_RING_ELEMENT,
                ETA1,
                ETA1_RANDOMNESS_SIZE,
            >(randomness, key_pair)
        }
        /// An encoded, compressed, incremental key pair.
        ///
        /// Layout: dk | (t | ⍴) | H(ek) | z
        pub struct KeyPairCompressedBytes {
            value: [u8; key_pair_compressed_len()]
        }

        impl KeyPairCompressedBytes {
            /// Generate a new key pair.
            /// This uses unpacked keys and does not compress the keys.
            pub fn from_seed(randomness: [u8; KEY_GENERATION_SEED_SIZE]) -> Self {
                let mut out = Self {
                    value: [0u8; key_pair_compressed_len()]
                };
                generate_key_pair_compressed(randomness, &mut out.value);
                out
            }

            /// Generate a new key pair.
            /// This uses unpacked keys and does not compress the keys.
            #[cfg(all(not(eurydice), feature = "rand"))]
            pub fn generate(rng: &mut (impl RngCore + CryptoRng)) -> Self {
                let mut randomness = [0u8; KEY_GENERATION_SEED_SIZE];
                rng.fill_bytes(&mut randomness);
                let mut out = Self {
                    value: [0u8; key_pair_compressed_len()]
                };
                generate_key_pair_compressed(randomness, &mut out.value);
                out
            }

            /// Get the raw bytes.
            pub fn to_bytes(self) -> [u8; key_pair_compressed_len()] {
                self.value
            }

            /// Get the PK1 bytes from the serialized key pair bytes
            pub fn pk1(&self) -> &[u8; pk1_len()] {
                // The unwrap here is ok because that's exactly what we take
                // and we know that `self.value` is long enough.
                const START: usize = 2 *  RANKED_BYTES_PER_RING_ELEMENT;
                <&[u8; pk1_len()]>::try_from(&self.value[START..START + pk1_len()]).unwrap()
            }

            /// Get the PK2 bytes from the serialized key pair bytes
            pub fn pk2(&self) -> &[u8; pk2_len()] {
                // The unwrap here is ok because that's exactly what we take
                // and we know that `self.value` is long enough.
                const START: usize = RANKED_BYTES_PER_RING_ELEMENT;
                <&[u8; pk2_len()]>::try_from(&self.value[START..START + pk2_len()]).unwrap()
            }

            /// Get the serialized private for decapsulation.
            pub fn sk(&self) -> &[u8; SECRET_KEY_SIZE] {
                &self.value
            }
        }

        impl AsRef<[u8]> for KeyPairCompressedBytes {
            fn as_ref(&self) -> &[u8] {
                &self.value
            }
        }

        /// Generate a key pair and write it into `key_pair`.
        /// This compresses the keys.
        pub fn generate_key_pair_compressed(randomness: [u8; KEY_GENERATION_SEED_SIZE], key_pair: &mut [u8; COMPRESSED_KEYPAIR_LEN]) {
            multiplexing::generate_keypair_compressed::<
                RANK,
                RANKED_BYTES_PER_RING_ELEMENT,
                CPA_PKE_SECRET_KEY_SIZE,
                SECRET_KEY_SIZE,
                CPA_PKE_PUBLIC_KEY_SIZE,
                RANKED_BYTES_PER_RING_ELEMENT,
                ETA1,
                ETA1_RANDOMNESS_SIZE,
                COMPRESSED_KEYPAIR_LEN,
            >(randomness, key_pair)
        }

        /// Get the PK1 bytes from the serialized key pair bytes
        pub fn pk1(
            keypair: &[u8; key_pair_len()],
        ) -> &[u8] {
            &keypair[0..pk1_len()]
        }

        /// Get the PK2 bytes from the serialized key pair bytes
        pub fn pk2(
            keypair: &[u8; key_pair_len()],
        ) -> &[u8] {
            &keypair[pk1_len()..pk1_len() + pk2_len()]
        }

        /// Validate that the two parts `pk1` and `pk2` are consistent.
        pub fn validate_pk(
            pk1: &PublicKey1,
            pk2: &[u8],
        ) -> Result<(), Error> {
            multiplexing::validate_pk::<RANK,  CPA_PKE_PUBLIC_KEY_SIZE>(pk1, pk2)
        }

        /// Validate that the two parts `pk1` and `pk2` are consistent.
        pub fn validate_pk_bytes(
            pk1: &[u8],
            pk2: &[u8],
        ) -> Result<(), Error> {
            multiplexing::validate_pk_bytes::<RANK,  CPA_PKE_PUBLIC_KEY_SIZE>(pk1, pk2)
        }

        /// Encapsulate the first part of the ciphertext.
        ///
        /// Returns an [`Error`] if the provided input or output don't have
        /// the appropriate sizes.
        ///
        /// `state` must be at least `encaps_state_len()` bytes and
        /// `shared_secret` at least `shared_secret_size()` bytes long.
        #[hax_lib::requires(
            state.len() >= RANK * 512 + 512 + 32
            && shared_secret.len() >= 32
        )]
        pub fn encapsulate1(
            pk1: &[u8],
            randomness: [u8; SHARED_SECRET_SIZE],
            state: &mut [u8],
            shared_secret: &mut [u8],
        ) -> Result<Ciphertext1, Error> {
            let public_key_part = PublicKey1::try_from(&pk1 as &[u8])?;

            multiplexing::encapsulate1::<
                RANK,
                CPA_PKE_CIPHERTEXT_SIZE,
                C1_SIZE,
                VECTOR_U_COMPRESSION_FACTOR,
                C1_BLOCK_SIZE,
                ETA1,
                ETA1_RANDOMNESS_SIZE,
                ETA2,
                ETA2_RANDOMNESS_SIZE,
            >(&public_key_part, randomness, state, shared_secret)
        }

        /// Encapsulate the first part of the ciphertext.
        ///
        /// Returns an [`Error`] if the provided input or output don't have
        /// the appropriate sizes.
        #[cfg(feature = "rand")]
        pub mod rand {
            use super::*;
            use ::rand::TryRngCore;

            /// Encapsulate the first part of the ciphertext.
            ///
            /// Returns an [`Error`] if the provided input or output don't have
            /// the appropriate sizes.
            ///
            /// `state` must be at least `encaps_state_len()` bytes and
            /// `shared_secret` at least `shared_secret_size()` bytes long.
            #[hax_lib::requires(
                state.len() >= RANK * 512 + 512 + 32
                && shared_secret.len() >= 32
            )]
            pub fn encapsulate1(
                pk1: &[u8],
                rng: &mut impl CryptoRng,
                state: &mut [u8],
                shared_secret: &mut [u8],
            ) -> Result<Ciphertext1, Error> {
                let public_key_part = PublicKey1::try_from(&pk1 as &[u8])?;
                let mut randomness = [0u8; SHARED_SECRET_SIZE];
                rng.try_fill_bytes(&mut randomness).map_err(|_| Error::InsufficientRandomness)?;

                multiplexing::encapsulate1::<
                    RANK,
                    CPA_PKE_CIPHERTEXT_SIZE,
                    C1_SIZE,
                    VECTOR_U_COMPRESSION_FACTOR,
                    C1_BLOCK_SIZE,
                    ETA1,
                    ETA1_RANDOMNESS_SIZE,
                    ETA2,
                    ETA2_RANDOMNESS_SIZE,
                >(&public_key_part, randomness, state, shared_secret)
            }
        }

        /// Encapsulate the second part of the ciphertext.
        ///
        /// The second part of the public key is passed in as byte slice.
        /// [`Error::InvalidInput`] is returned if the state bytes fail
        /// validation (a decoded coefficient is out of field range).
        pub fn encapsulate2(state: &[u8; encaps_state_len()], public_key_part: &[u8; pk2_len()]) -> Result<Ciphertext2, Error> {
            multiplexing::encapsulate2::<RANK, RANKED_BYTES_PER_RING_ELEMENT, C2_SIZE, VECTOR_V_COMPRESSION_FACTOR, {encaps_state_len()}>(state, public_key_part)
        }

        /// Decapsulate incremental ciphertexts.
        ///
        /// `private_key` must be at least `key_pair_len()` bytes long.
        /// [`Error::InvalidInput`] is returned if the key bytes fail
        /// validation (a decoded coefficient is out of field range).
        #[hax_lib::requires(
            private_key.len() >= 64 + RANKED_BYTES_PER_RING_ELEMENT + RANK * 512 + 32 + RANK * RANK * 512
        )]
        pub fn decapsulate_incremental_key(
            private_key: &[u8],
            ciphertext1: &Ciphertext1,
            ciphertext2: &Ciphertext2,
        ) -> Result<MlKemSharedSecret, Error> {
            multiplexing::decapsulate::<
                RANK,
                RANKED_BYTES_PER_RING_ELEMENT,
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
            >(private_key, ciphertext1, ciphertext2)
        }

        /// Decapsulate incremental ciphertexts.
        pub fn decapsulate_compressed_key(
            private_key: &[u8; SECRET_KEY_SIZE],
            ciphertext1: &Ciphertext1,
            ciphertext2: &Ciphertext2,
        ) -> MlKemSharedSecret {
            multiplexing::decapsulate_compressed::<
                RANK,
                RANKED_BYTES_PER_RING_ELEMENT,
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
            >(private_key, ciphertext1, ciphertext2)
        }
    };
}
pub(crate) use impl_incr_key_size;

macro_rules! impl_incr_platform {
    ($vector:path, $hash:path $(, $unsafe:ident)? $(, #[$meta:meta])*) => {
        #[allow(unused_imports)]
        use hax_lib::prop::ToProp;

        /// Downcast [`Keys`] to a platform dependent [`MlKemKeyPairUnpacked`].
        ///
        /// **PANICS** is the cast fails
        #[cfg(feature = "alloc")]
        pub(super) fn as_keypair<const K: usize>(
            k: &dyn core::any::Any,
        ) -> &MlKemKeyPairUnpacked<K, $vector> {
            k.downcast_ref().unwrap()
        }

        /// Downcast [`State`] to a  platform dependent [`EncapsState`].
        ///
        /// **PANICS** is the cast fails
        #[cfg(feature = "alloc")]
        pub(super) fn as_state<const K: usize>(s: &dyn core::any::Any) -> &EncapsState<K, $vector> {
            s.downcast_ref().unwrap()
        }

        $(#[$meta])*
        #[hax_lib::requires(
            hacspec_ml_kem::parameters::is_rank(K)
            && ETA1 == hacspec_ml_kem::parameters::eta1(K)
            && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
            && PUBLIC_KEY_SIZE == hacspec_ml_kem::parameters::cpa_public_key_size(K)
        )]
        pub(crate) $($unsafe)? fn generate_keypair<
            const K: usize,
            const CPA_PRIVATE_KEY_SIZE: usize,
            const PRIVATE_KEY_SIZE: usize,
            const PUBLIC_KEY_SIZE: usize,
            const BYTES_PER_RING_ELEMENT: usize,
            const ETA1: usize,
            const ETA1_RANDOMNESS_SIZE: usize,
        >(
            randomness: [u8; KEY_GENERATION_SEED_SIZE],
        ) -> MlKemKeyPairUnpacked<K, $vector> {
            super::generate_keypair::<
                K,
                CPA_PRIVATE_KEY_SIZE,
                PRIVATE_KEY_SIZE,
                PUBLIC_KEY_SIZE,
                ETA1,
                ETA1_RANDOMNESS_SIZE,
                $vector,
                $hash,
            >(randomness)
        }

        $(#[$meta])*
        #[hax_lib::requires(
            hacspec_ml_kem::parameters::is_rank(K)
            && ETA1 == hacspec_ml_kem::parameters::eta1(K)
            && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
            && PUBLIC_KEY_SIZE == hacspec_ml_kem::parameters::cpa_public_key_size(K)
            && PK2_LEN <= 1536
        )]
        #[hax_lib::ensures(|result|
            hax_lib::implies(
                key_pair.len() >= 64 + PK2_LEN + K * 512 + 32 + K * K * 512,
                result.is_ok(),
            ) & (future(key_pair).len() == key_pair.len()))]
        pub(crate) $($unsafe)? fn generate_keypair_serialized<
            const K: usize,
            const PK2_LEN: usize,
            const CPA_PRIVATE_KEY_SIZE: usize,
            const PRIVATE_KEY_SIZE: usize,
            const PUBLIC_KEY_SIZE: usize,
            const BYTES_PER_RING_ELEMENT: usize,
            const ETA1: usize,
            const ETA1_RANDOMNESS_SIZE: usize,
        >(
            randomness: [u8; KEY_GENERATION_SEED_SIZE],
            key_pair: &mut [u8],
        ) -> Result<(), Error> {
            if key_pair.len() < KeyPair::<K, PK2_LEN, $vector>::num_bytes() {
                return Err(Error::InvalidOutputLength);
            }

            super::generate_keypair_serialized::<
                K,
                PK2_LEN,
                CPA_PRIVATE_KEY_SIZE,
                PRIVATE_KEY_SIZE,
                PUBLIC_KEY_SIZE,
                ETA1,
                ETA1_RANDOMNESS_SIZE,
                $vector,
                $hash,
            >(randomness, key_pair)
        }

        $(#[$meta])*
        #[hax_lib::requires(
            (hacspec_ml_kem::parameters::is_rank(K)
            && ETA1 == hacspec_ml_kem::parameters::eta1(K)
            && ETA1_RANDOMNESS_SIZE == hacspec_ml_kem::parameters::eta1_randomness_size(K)
            && PUBLIC_KEY_SIZE == hacspec_ml_kem::parameters::cpa_public_key_size(K)
            && CPA_PRIVATE_KEY_SIZE == hacspec_ml_kem::parameters::ranked_bytes_per_ring_element(K)).to_prop()
            & fstar!(r#"v $COMPRESSED_KEYPAIR_LEN >= v $CPA_PRIVATE_KEY_SIZE + v $PK2_LEN + 96"#)
        )]
        pub(crate) $($unsafe)? fn generate_keypair_compressed<
            const K: usize,
            const PK2_LEN: usize,
            const CPA_PRIVATE_KEY_SIZE: usize,
            const PRIVATE_KEY_SIZE: usize,
            const PUBLIC_KEY_SIZE: usize,
            const BYTES_PER_RING_ELEMENT: usize,
            const ETA1: usize,
            const ETA1_RANDOMNESS_SIZE: usize,
            const COMPRESSED_KEYPAIR_LEN: usize,
        >(
            randomness: [u8; KEY_GENERATION_SEED_SIZE],
            key_pair: &mut [u8; COMPRESSED_KEYPAIR_LEN],
        ) {
            super::generate_keypair_compressed::<
                K,
                PK2_LEN,
                CPA_PRIVATE_KEY_SIZE,
                PRIVATE_KEY_SIZE,
                PUBLIC_KEY_SIZE,
                ETA1,
                ETA1_RANDOMNESS_SIZE,
                COMPRESSED_KEYPAIR_LEN,
                $vector,
                $hash,
            >(randomness, key_pair)
        }

        $(#[$meta])*
        #[hax_lib::requires(
            hacspec_ml_kem::parameters::is_rank(K)
            && PK_LEN == hacspec_ml_kem::parameters::cpa_public_key_size(K)
        )]
        pub(crate) $($unsafe)? fn validate_pk<const K: usize, const PK_LEN: usize>(
            pk1: &PublicKey1,
            pk2: &[u8],
        ) -> Result<(), Error> {
            super::validate_pk::<K, PK_LEN, $hash, $vector>(pk1, pk2)
        }

        $(#[$meta])*
        #[hax_lib::requires(
            hacspec_ml_kem::parameters::is_rank(K)
            && PK_LEN == hacspec_ml_kem::parameters::cpa_public_key_size(K)
        )]
        pub(crate) $($unsafe)? fn validate_pk_bytes<const K: usize, const PK_LEN: usize>(
            pk1: &[u8],
            pk2: &[u8],
        ) -> Result<(), Error> {
            super::validate_pk_bytes::<K, PK_LEN, $hash, $vector>(pk1, pk2)
        }

        $(#[$meta])*
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
        pub(crate) $($unsafe)? fn encapsulate1<
            const K: usize,
            const CIPHERTEXT_SIZE: usize,
            const C1_SIZE: usize,
            const VECTOR_U_COMPRESSION_FACTOR: usize,
            const VECTOR_U_BLOCK_LEN: usize,
            const ETA1: usize,
            const ETA1_RANDOMNESS_SIZE: usize,
            const ETA2: usize,
            const ETA2_RANDOMNESS_SIZE: usize,
        >(
            public_key_part: &PublicKey1,
            randomness: [u8; SHARED_SECRET_SIZE],
        ) -> (
            Ciphertext1<C1_SIZE>,
            EncapsState<K, $vector>,
            [u8; SHARED_SECRET_SIZE],
        ) {
            super::encapsulate1::<
                K,
                CIPHERTEXT_SIZE,
                C1_SIZE,
                VECTOR_U_COMPRESSION_FACTOR,
                VECTOR_U_BLOCK_LEN,
                ETA1,
                ETA1_RANDOMNESS_SIZE,
                ETA2,
                ETA2_RANDOMNESS_SIZE,
                $vector,
                $hash,
            >(public_key_part, randomness)
        }

        $(#[$meta])*
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
        pub(crate) $($unsafe)? fn encapsulate1_serialized<
            const K: usize,
            const CIPHERTEXT_SIZE: usize,
            const C1_SIZE: usize,
            const VECTOR_U_COMPRESSION_FACTOR: usize,
            const VECTOR_U_BLOCK_LEN: usize,
            const ETA1: usize,
            const ETA1_RANDOMNESS_SIZE: usize,
            const ETA2: usize,
            const ETA2_RANDOMNESS_SIZE: usize,
        >(
            public_key_part: &PublicKey1,
            randomness: [u8; SHARED_SECRET_SIZE],
            state: &mut [u8],
            shared_secret: &mut [u8],
        ) -> Result<Ciphertext1<C1_SIZE>, Error> {
            super::encapsulate1_serialized::<
                K,
                CIPHERTEXT_SIZE,
                C1_SIZE,
                VECTOR_U_COMPRESSION_FACTOR,
                VECTOR_U_BLOCK_LEN,
                ETA1,
                ETA1_RANDOMNESS_SIZE,
                ETA2,
                ETA2_RANDOMNESS_SIZE,
                $vector,
                $hash,
            >(public_key_part, randomness, state, shared_secret)
        }

        $(#[$meta])*
        #[hax_lib::requires(
            (hacspec_ml_kem::parameters::is_rank(K)
            && PK2_LEN == hacspec_ml_kem::parameters::cpa_private_key_size(K)
            && C2_SIZE == hacspec_ml_kem::parameters::c2_size(K)
            && VECTOR_V_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_v_compression_factor(K))
                .to_prop()
            & crate::polynomial::spec::is_bounded_polynomial_vector(3328, &state.r_as_ntt)
            & crate::polynomial::spec::is_bounded_poly(3328, &state.error2)
        )]
        pub(crate) $($unsafe)? fn encapsulate2<
            const K: usize,
            const PK2_LEN: usize,
            const C2_SIZE: usize,
            const VECTOR_V_COMPRESSION_FACTOR: usize,
        >(
            state: &EncapsState<K, $vector>,
            public_key_part: &PublicKey2<PK2_LEN>,
        ) -> Ciphertext2<C2_SIZE> {
            super::encapsulate2::<K, PK2_LEN, C2_SIZE, VECTOR_V_COMPRESSION_FACTOR, $vector>(
                state,
                public_key_part,
            )
        }

        $(#[$meta])*
        #[hax_lib::requires(
            hacspec_ml_kem::parameters::is_rank(K)
            && PK2_LEN == hacspec_ml_kem::parameters::cpa_private_key_size(K)
            && C2_SIZE == hacspec_ml_kem::parameters::c2_size(K)
            && VECTOR_V_COMPRESSION_FACTOR == hacspec_ml_kem::parameters::vector_v_compression_factor(K)
            && STATE_LEN >= K * 512 + 512 + 32
        )]
        pub(crate) $($unsafe)? fn encapsulate2_serialized<
            const K: usize,
            const PK2_LEN: usize,
            const C2_SIZE: usize,
            const VECTOR_V_COMPRESSION_FACTOR: usize,
            const STATE_LEN: usize,
        >(
            state: &[u8; STATE_LEN],
            public_key_part: &PublicKey2<PK2_LEN>,
        ) -> Result<Ciphertext2<C2_SIZE>, Error> {
            super::encapsulate2_serialized::<
                K,
                PK2_LEN,
                C2_SIZE,
                VECTOR_V_COMPRESSION_FACTOR,
                STATE_LEN,
                $vector,
            >(state, public_key_part)
        }

        $(#[$meta])*
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
        pub(crate) $($unsafe)? fn decapsulate<
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
            private_key: &MlKemKeyPairUnpacked<K, $vector>,
            ciphertext1: &Ciphertext1<C1_SIZE>,
            ciphertext2: &Ciphertext2<C2_SIZE>,
        ) -> MlKemSharedSecret {
            super::decapsulate::<
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
                $vector,
                $hash,
            >(private_key, ciphertext1, ciphertext2)
        }

        $(#[$meta])*
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
            && private_key.len() >= 64 + PK2_LEN + K * 512 + 32 + K * K * 512
        )]
        pub(crate) $($unsafe)? fn decapsulate_incremental_key<
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
        >(
            private_key: &[u8],
            ciphertext1: &Ciphertext1<C1_SIZE>,
            ciphertext2: &Ciphertext2<C2_SIZE>,
        ) -> Result<MlKemSharedSecret, Error> {
            super::decapsulate_incremental_key::<
                K,
                PK2_LEN,
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
                $vector,
                $hash,
            >(private_key, ciphertext1, ciphertext2)
        }

        $(#[$meta])*
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
        pub(crate) $($unsafe)? fn decapsulate_compressed_key<
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
        >(
            private_key: &[u8; SECRET_KEY_SIZE],
            ciphertext1: &Ciphertext1<C1_SIZE>,
            ciphertext2: &Ciphertext2<C2_SIZE>,
        ) -> MlKemSharedSecret {
            super::decapsulate_compressed_key::<
                K,
                PK2_LEN,
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
                $vector,
                $hash,
            >(private_key, ciphertext1, ciphertext2)
        }
    };
}
pub(crate) use impl_incr_platform;

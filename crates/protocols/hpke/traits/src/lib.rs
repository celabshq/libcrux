#![doc = include_str!("../Readme.md")]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::string::String;
use alloc::vec::Vec;

use error::Error;
use types::{AeadAlgorithm, KemAlgorithm};

pub mod error;
pub mod types;

// re-export trait
pub use rand::{CryptoRng, Rng, TryCryptoRng, TryRng};

/// The [`HpkeCrypto`] trait defines the necessary cryptographic functions used
/// in the HPKE implementation.
pub trait HpkeCrypto: core::fmt::Debug + Send + Sync {
    /// The PRNG implementation used by this provider's operations.
    type HpkePrng: TryCryptoRng;

    /// The name of the implementation.
    fn name() -> String;

    /// Returns an error if the KDF algorithm is not supported by this crypto provider.
    fn supports_kdf(alg: types::KdfAlgorithm) -> Result<(), Error>;

    /// Returns an error if the KEM algorithm is not supported by this crypto provider.
    fn supports_kem(alg: types::KemAlgorithm) -> Result<(), Error>;

    /// Returns an error if the AEAD algorithm is not supported by this crypto provider.
    fn supports_aead(alg: types::AeadAlgorithm) -> Result<(), Error>;

    /// Get the length of the output digest.
    #[inline(always)]
    fn kdf_digest_length(alg: types::KdfAlgorithm) -> usize {
        match alg {
            types::KdfAlgorithm::HkdfSha256 => 32,
            types::KdfAlgorithm::HkdfSha384 => 48,
            types::KdfAlgorithm::HkdfSha512 => 64,
            // `Nh` for the SHAKE KDFs per draft-ietf-hpke-pq Table 1.
            types::KdfAlgorithm::Shake128 => 32,
            types::KdfAlgorithm::Shake256 => 64,
            types::KdfAlgorithm::TurboShake128 => 32,
            types::KdfAlgorithm::TurboShake256 => 64,
        }
    }

    /// KDF Extract (two-stage KDFs only).
    fn kdf_extract(
        alg: types::TwoStageKdfAlgorithm,
        salt: &[u8],
        ikm: &[u8],
    ) -> Result<Vec<u8>, Error>;

    /// KDF Expand (two-stage KDFs only).
    fn kdf_expand(
        alg: types::TwoStageKdfAlgorithm,
        prk: &[u8],
        info: &[u8],
        output_size: usize,
    ) -> Result<Vec<u8>, Error>;

    /// KDF Derive (single-stage KDFs only): derive `l` bytes from `ikm`.
    fn kdf_derive(
        alg: types::SingleStageKdfAlgorithm,
        ikm: &[u8],
        l: usize,
    ) -> Result<Vec<u8>, Error>;

    /// Diffie-Hellman
    fn dh(alg: KemAlgorithm, pk: &[u8], sk: &[u8]) -> Result<Vec<u8>, Error>;

    /// Diffie-Hellman with the base (generate public key for secret key `sk`).
    fn secret_to_public(alg: KemAlgorithm, sk: &[u8]) -> Result<Vec<u8>, Error>;

    /// KEM key pair generation (encapsulation key, decapsulation key).
    fn kem_key_gen(
        alg: KemAlgorithm,
        prng: &mut Self::HpkePrng,
    ) -> Result<(Vec<u8>, Vec<u8>), Error>;

    /// KEM key pair generation (encapsulation key, decapsulation key) based
    /// on the `seed`.
    fn kem_key_gen_derand(alg: KemAlgorithm, seed: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Error>;

    /// KEM encapsulation to `pk_r` (shared secret, ciphertext).
    ///
    /// `hpke-rs` prefers [`HpkeCrypto::kem_encaps_derand`] and only calls this when that
    /// returns [`Error::UnsupportedKemOperation`].
    fn kem_encaps(
        alg: KemAlgorithm,
        pk_r: &[u8],
        prng: &mut Self::HpkePrng,
    ) -> Result<(Vec<u8>, Vec<u8>), Error>;

    /// Derandomized KEM encapsulation to `pk_r`, using the given
    /// `randomness` (see [`KemAlgorithm::encaps_randomness_len`]) instead of
    /// drawing from a PRNG. Only called for KEMs where
    /// `KemAlgorithm::encaps_randomness_len` returns `Some` (DH-based KEMs
    /// derive their own randomness generically and never reach this).
    fn kem_encaps_derand(
        alg: KemAlgorithm,
        pk_r: &[u8],
        randomness: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>), Error>;

    /// KEM decapsulation with `sk_r`.
    /// Returns the shared secret.
    fn kem_decaps(alg: KemAlgorithm, ct: &[u8], sk_r: &[u8]) -> Result<Vec<u8>, Error>;

    /// Validate a secret key for its correctness.
    fn dh_validate_sk(alg: KemAlgorithm, sk: &[u8]) -> Result<Vec<u8>, Error>;

    /// AEAD encrypt.
    fn aead_seal(
        alg: AeadAlgorithm,
        key: &[u8],
        nonce: &[u8],
        aad: &[u8],
        msg: &[u8],
    ) -> Result<Vec<u8>, Error>;

    /// AEAD decrypt.
    fn aead_open(
        alg: AeadAlgorithm,
        key: &[u8],
        nonce: &[u8],
        aad: &[u8],
        msg: &[u8],
    ) -> Result<Vec<u8>, Error>;

    /// Get key length for AEAD.
    ///
    /// Note that this function returns `0` for export only keys of unknown size.
    fn aead_key_length(alg: AeadAlgorithm) -> usize {
        match alg {
            AeadAlgorithm::Aes128Gcm => 16,
            AeadAlgorithm::Aes256Gcm => 32,
            AeadAlgorithm::ChaCha20Poly1305 => 32,
            AeadAlgorithm::HpkeExport => 0,
        }
    }

    /// Get key length for AEAD.
    ///
    /// Note that this function returns `0` for export only nonces of unknown size.
    fn aead_nonce_length(alg: AeadAlgorithm) -> usize {
        match alg {
            AeadAlgorithm::Aes128Gcm => 12,
            AeadAlgorithm::Aes256Gcm => 12,
            AeadAlgorithm::ChaCha20Poly1305 => 12,
            AeadAlgorithm::HpkeExport => 0,
        }
    }

    /// Get key length for AEAD.
    ///
    /// Note that this function returns `0` for export only tags of unknown size.
    fn aead_tag_length(alg: AeadAlgorithm) -> usize {
        match alg {
            AeadAlgorithm::Aes128Gcm => 16,
            AeadAlgorithm::Aes256Gcm => 16,
            AeadAlgorithm::ChaCha20Poly1305 => 16,
            AeadAlgorithm::HpkeExport => 0,
        }
    }
}

/// Implemented by the default HPKE providers — the ones that know how to construct their own
/// PRNG with no input from the caller (typically by drawing on system entropy). Used by
/// `Hpke::new`/`Hpke::try_new`. A provider parameterized over a caller-supplied Rng (e.g.
/// `HpkeLibcrux<R>` for some `R` other than the default) has no default and doesn't implement
/// this; use `Hpke::new_with_rng` for those instead.
pub trait HpkeDefaultPrng: HpkeCrypto {
    /// Construct the default PRNG, or `Err` if none is available in this build (e.g. no system
    /// RNG configured).
    fn try_prng() -> Result<Self::HpkePrng, Error>;
}

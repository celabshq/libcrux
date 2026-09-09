#![doc = include_str!("../Readme.md")]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

// The deprecated `draft-connolly-cfrg-hpke-mlkem` feature is superseded by
// `draft-ietf-hpke-pq`; the two use incompatible `DeriveKeyPair` / KDF semantics
// for the same ML-KEM code points and must not be enabled together.
#[cfg(all(
    feature = "draft-connolly-cfrg-hpke-mlkem",
    feature = "draft-ietf-hpke-pq"
))]
compile_error!(
    "enable only one of `draft-connolly-cfrg-hpke-mlkem` (deprecated) or `draft-ietf-hpke-pq`"
);

use alloc::{format, string::String, vec::Vec};
use core::fmt::Display;
use zeroize::Zeroize;

use hpke_rs_crypto::{
    error::Error,
    types::{
        AeadAlgorithm, KdfAlgorithm, KemAlgorithm, SingleStageKdfAlgorithm, TwoStageKdfAlgorithm,
    },
    HpkeCrypto, HpkeTestRng,
};

#[cfg(feature = "rustcrypto-p-curves")]
use p384::{
    elliptic_curve::{ecdh::diffie_hellman as p384diffie_hellman, sec1::ToSec1Point, Generate},
    PublicKey as P384PublicKey, SecretKey as P384SecretKey,
};
#[cfg(feature = "rustcrypto-p-curves")]
use p521::{
    elliptic_curve::ecdh::diffie_hellman as p521diffie_hellman, PublicKey as P521PublicKey,
    SecretKey as P521SecretKey,
};

use rand::{rngs::SysRng, Rng, SeedableRng, TryCryptoRng, TryRng};
use rand_core::UnwrapErr;

/// The Libcrux HPKE Provider
#[derive(Debug)]
pub struct HpkeLibcrux {}

/// The PRNG for the Libcrux Provider.
pub struct HpkeLibcruxPrng {
    #[cfg(feature = "deterministic-prng")]
    fake_rng: Vec<u8>,
    rng: rand_chacha::ChaCha20Rng,
}

impl HpkeLibcruxPrng {
    /// Construct a PRNG from a 32-byte seed.
    ///
    /// Useful on platforms without a system RNG (e.g. wasm), where the seed is
    /// gathered by the caller and passed in explicitly.
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self {
            #[cfg(feature = "deterministic-prng")]
            fake_rng: alloc::vec![],
            rng: rand_chacha::ChaCha20Rng::from_seed(seed),
        }
    }
}

impl Zeroize for HpkeLibcruxPrng {
    fn zeroize(&mut self) {
        // ChaCha20Rng doesn't implement zeroize and fake_rng is just for testing.
    }
}

impl HpkeCrypto for HpkeLibcrux {
    fn name() -> String {
        "Libcrux".into()
    }

    fn kdf_extract(alg: TwoStageKdfAlgorithm, salt: &[u8], ikm: &[u8]) -> Result<Vec<u8>, Error> {
        let alg = kdf_algorithm_to_libcrux_hkdf_algorithm(alg);
        let mut prk = alloc::vec![0u8; alg.hash_len()];
        libcrux_hkdf::extract(alg, &mut prk, salt, ikm)
            .map_err(|e| Error::CryptoLibraryError(format!("KDF extract error: {:?}", e)))?;
        Ok(prk)
    }

    fn kdf_expand(
        alg: TwoStageKdfAlgorithm,
        prk: &[u8],
        info: &[u8],
        output_size: usize,
    ) -> Result<Vec<u8>, Error> {
        let alg = kdf_algorithm_to_libcrux_hkdf_algorithm(alg);
        let mut okm = alloc::vec![0u8; output_size];
        libcrux_hkdf::expand(alg, &mut okm, prk, info)
            .map_err(|e| Error::CryptoLibraryError(format!("KDF expand error: {:?}", e)))?;
        Ok(okm)
    }

    fn kdf_derive(alg: SingleStageKdfAlgorithm, ikm: &[u8], l: usize) -> Result<Vec<u8>, Error> {
        // Single-stage (XOF) KDF: `Derive(ikm, L) = SHAKE(ikm, 8*L)`.
        // See draft-ietf-hpke-pq Section 5.
        Ok(shake_derive(alg, &[ikm], l))
    }

    fn dh(alg: KemAlgorithm, pk: &[u8], sk: &[u8]) -> Result<Vec<u8>, Error> {
        match alg {
            #[cfg(feature = "rustcrypto-p-curves")]
            KemAlgorithm::DhKemP384 => {
                let sk = P384SecretKey::from_slice(sk).map_err(|_| Error::KemInvalidSecretKey)?;
                let pk =
                    P384PublicKey::from_sec1_bytes(pk).map_err(|_| Error::KemInvalidPublicKey)?;
                Ok(p384diffie_hellman(sk.to_nonzero_scalar(), pk.as_affine())
                    .raw_secret_bytes()
                    .as_slice()
                    .into())
            }
            #[cfg(feature = "rustcrypto-p-curves")]
            KemAlgorithm::DhKemP521 => {
                let sk = P521SecretKey::from_slice(sk).map_err(|_| Error::KemInvalidSecretKey)?;
                let pk =
                    P521PublicKey::from_sec1_bytes(pk).map_err(|_| Error::KemInvalidPublicKey)?;
                Ok(p521diffie_hellman(sk.to_nonzero_scalar(), pk.as_affine())
                    .raw_secret_bytes()
                    .as_slice()
                    .into())
            }
            other => {
                let alg = kem_key_type_to_ecdh_alg(other)?;

                libcrux_ecdh::derive(alg, pk, sk)
                    .map_err(|e| Error::CryptoLibraryError(format!("ECDH derive error: {:?}", e)))
                    .map(|mut p| {
                        if alg == libcrux_ecdh::Algorithm::P256 {
                            p.truncate(32);
                            p
                        } else {
                            p
                        }
                    })
            }
        }
    }

    fn secret_to_public(alg: KemAlgorithm, sk: &[u8]) -> Result<Vec<u8>, Error> {
        match alg {
            #[cfg(feature = "rustcrypto-p-curves")]
            KemAlgorithm::DhKemP384 => {
                let sk = P384SecretKey::from_slice(sk).map_err(|_| Error::KemInvalidSecretKey)?;
                Ok(sk.public_key().to_sec1_point(false).as_bytes().into())
            }
            #[cfg(feature = "rustcrypto-p-curves")]
            KemAlgorithm::DhKemP521 => {
                let sk = P521SecretKey::from_slice(sk).map_err(|_| Error::KemInvalidSecretKey)?;
                Ok(sk.public_key().to_sec1_point(false).as_bytes().into())
            }
            other => {
                let alg = kem_key_type_to_ecdh_alg(other)?;
                kem_ecdh_secret_to_public(alg, sk)
            }
        }
    }

    fn kem_key_gen(
        alg: KemAlgorithm,
        prng: &mut Self::HpkePrng,
    ) -> Result<(Vec<u8>, Vec<u8>), Error> {
        match alg {
            #[cfg(any(
                feature = "draft-connolly-cfrg-hpke-mlkem",
                feature = "draft-ietf-hpke-pq"
            ))]
            KemAlgorithm::MlKem512 | KemAlgorithm::MlKem768 | KemAlgorithm::MlKem1024 => {
                let kem_alg = kem_key_type_to_libcrux_alg(alg)?;
                libcrux_kem::key_gen(kem_alg, prng)
                    .map(|(sk, pk)| (pk.encode(), sk.encode()))
                    .map_err(|e| Error::CryptoLibraryError(format!("KEM key gen error: {:?}", e)))
            }
            #[cfg(feature = "draft-ietf-hpke-pq")]
            KemAlgorithm::MlKem768P256 | KemAlgorithm::MlKem1024P384 => hybrid::key_gen(alg, prng),
            KemAlgorithm::XWingDraft06 => {
                let kem_alg = kem_key_type_to_libcrux_alg(alg)?;
                libcrux_kem::key_gen(kem_alg, prng)
                    .map(|(sk, pk)| (pk.encode(), sk.encode()))
                    .map_err(|e| Error::CryptoLibraryError(format!("KEM key gen error: {:?}", e)))
            }
            #[cfg(feature = "rustcrypto-p-curves")]
            KemAlgorithm::DhKemP384 => {
                let sk = P384SecretKey::generate_from_rng(&mut prng.rng);
                let pk = sk.public_key().to_sec1_point(false).as_bytes().into();
                let sk = sk.to_bytes().as_slice().into();
                Ok((pk, sk))
            }
            #[cfg(feature = "rustcrypto-p-curves")]
            KemAlgorithm::DhKemP521 => {
                let sk = P521SecretKey::generate_from_rng(&mut prng.rng);
                let pk = sk.public_key().to_sec1_point(false).as_bytes().into();
                let sk = sk.to_bytes().as_slice().into();
                Ok((pk, sk))
            }
            other_alg => {
                // ECDH only (libcrux curves)
                let ecdh_alg = kem_key_type_to_ecdh_alg(other_alg)?;
                let sk = libcrux_ecdh::generate_secret(ecdh_alg, prng).map_err(|e| {
                    Error::CryptoLibraryError(format!("KEM key gen error: {:?}", e))
                })?;

                let pk = kem_ecdh_secret_to_public(ecdh_alg, &sk)?;

                Ok((pk, sk))
            }
        }
    }

    fn kem_key_gen_derand(alg: KemAlgorithm, seed: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Error> {
        match alg {
            #[cfg(feature = "draft-ietf-hpke-pq")]
            KemAlgorithm::MlKem768P256 | KemAlgorithm::MlKem1024P384 => {
                hybrid::key_gen_derand(alg, seed)
            }
            // hpke-pq: the ML-KEM decapsulation key is the 64-byte `(d, z)` seed;
            // the public key is expanded from it and `kem_decaps` re-expands the
            // seed to recover the full decapsulation key.
            #[cfg(feature = "draft-ietf-hpke-pq")]
            KemAlgorithm::MlKem512 | KemAlgorithm::MlKem768 | KemAlgorithm::MlKem1024 => {
                let kem_alg = kem_key_type_to_libcrux_alg(alg)?;
                let (_sk, pk) = libcrux_kem::key_gen_derand(kem_alg, seed).map_err(|e| {
                    Error::CryptoLibraryError(format!("KEM key gen error: {:?}", e))
                })?;
                Ok((pk.encode(), seed.to_vec()))
            }
            #[cfg(feature = "rustcrypto-p-curves")]
            KemAlgorithm::DhKemP384 | KemAlgorithm::DhKemP521 => Err(Error::CryptoLibraryError(
                format!("This API should not be called with this algorithm."),
            )),

            _ => {
                let alg = kem_key_type_to_libcrux_alg(alg)?;
                libcrux_kem::key_gen_derand(alg, seed)
                    .map_err(|e| Error::CryptoLibraryError(format!("KEM key gen error: {:?}", e)))
                    .map(|(sk, pk)| (pk.encode(), sk.encode()))
            }
        }
    }

    fn kem_encaps(
        alg: KemAlgorithm,
        pk_r: &[u8],
        prng: &mut Self::HpkePrng,
    ) -> Result<(Vec<u8>, Vec<u8>), Error> {
        // For known-answer tests the encapsulation randomness is injected via the
        // test seed; run the PQ KEMs derandomized with it rather than drawing
        // from the RNG. We pull exactly the KEM's randomness length —
        // which, when the test seed is the vector's `ikmE`, is `ikmE` in order.
        #[cfg(feature = "deterministic-prng")]
        if let Some(n) = pq_encaps_randomness_len(alg) {
            let mut randomness = alloc::vec![0u8; n];
            prng.try_fill_test_bytes(&mut randomness)
                .map_err(|_| Error::InsufficientRandomness)?;
            return kem_encaps_derand(alg, pk_r, &randomness);
        }
        match alg {
            #[cfg(feature = "draft-ietf-hpke-pq")]
            KemAlgorithm::MlKem768P256 | KemAlgorithm::MlKem1024P384 => {
                hybrid::encaps(alg, pk_r, prng)
            }
            #[cfg(feature = "rustcrypto-p-curves")]
            KemAlgorithm::DhKemP384 | KemAlgorithm::DhKemP521 => Err(Error::CryptoLibraryError(
                format!("This API should not be called with this algorithm."),
            )),
            _ => {
                let alg = kem_key_type_to_libcrux_alg(alg)?;

                let pk = libcrux_kem::PublicKey::decode(alg, pk_r)
                    .map_err(|_| Error::KemInvalidPublicKey)?;
                pk.encapsulate(prng)
                    .map_err(|e| Error::CryptoLibraryError(format!("Encaps error {:?}", e)))
                    .map(|(ss, ct)| (ss.encode(), ct.encode()))
            }
        }
    }

    fn kem_decaps(alg: KemAlgorithm, ct: &[u8], sk_r: &[u8]) -> Result<Vec<u8>, Error> {
        match alg {
            #[cfg(feature = "draft-ietf-hpke-pq")]
            KemAlgorithm::MlKem768P256 | KemAlgorithm::MlKem1024P384 => {
                hybrid::decaps(alg, ct, sk_r)
            }
            // hpke-pq: `sk_r` is the 64-byte ML-KEM seed; re-derive the full
            // decapsulation key before decapsulating.
            #[cfg(feature = "draft-ietf-hpke-pq")]
            KemAlgorithm::MlKem512 | KemAlgorithm::MlKem768 | KemAlgorithm::MlKem1024 => {
                let kem_alg = kem_key_type_to_libcrux_alg(alg)?;
                let (sk, _pk) = libcrux_kem::key_gen_derand(kem_alg, sk_r).map_err(|e| {
                    Error::CryptoLibraryError(format!("KEM key gen error: {:?}", e))
                })?;
                let ct = libcrux_kem::Ct::decode(kem_alg, ct)
                    .map_err(|_| Error::KemInvalidCiphertext)?;
                ct.decapsulate(&sk)
                    .map_err(|e| Error::CryptoLibraryError(format!("Decaps error {:?}", e)))
                    .map(|ss| ss.encode())
            }
            #[cfg(feature = "rustcrypto-p-curves")]
            KemAlgorithm::DhKemP384 | KemAlgorithm::DhKemP521 => Err(Error::CryptoLibraryError(
                format!("This API should not be called with this algorithm."),
            )),
            _ => {
                let alg = kem_key_type_to_libcrux_alg(alg)?;

                let ct =
                    libcrux_kem::Ct::decode(alg, ct).map_err(|_| Error::KemInvalidCiphertext)?;
                let sk = libcrux_kem::PrivateKey::decode(alg, sk_r)
                    .map_err(|_| Error::KemInvalidSecretKey)?;
                ct.decapsulate(&sk)
                    .map_err(|e| Error::CryptoLibraryError(format!("Decaps error {:?}", e)))
                    .map(|ss| ss.encode())
            }
        }
    }

    fn dh_validate_sk(alg: KemAlgorithm, sk: &[u8]) -> Result<Vec<u8>, Error> {
        match alg {
            KemAlgorithm::DhKemP256 => libcrux_ecdh::p256::validate_scalar_slice(sk)
                .map_err(|e| Error::CryptoLibraryError(format!("ECDH invalid sk error: {:?}", e)))
                .map(|sk| sk.0.to_vec()),
            #[cfg(feature = "rustcrypto-p-curves")]
            KemAlgorithm::DhKemP384 => P384SecretKey::from_slice(sk)
                .map_err(|_| Error::KemInvalidSecretKey)
                .map(|_| sk.into()),
            #[cfg(feature = "rustcrypto-p-curves")]
            KemAlgorithm::DhKemP521 => P521SecretKey::from_slice(sk)
                .map_err(|_| Error::KemInvalidSecretKey)
                .map(|_| sk.into()),
            _ => Err(Error::UnknownKemAlgorithm),
        }
    }

    fn aead_seal(
        alg: AeadAlgorithm,
        key: &[u8],
        nonce: &[u8],
        aad: &[u8],
        msg: &[u8],
    ) -> Result<Vec<u8>, Error> {
        let alg = aead_alg(alg)?;

        use libcrux_traits::aead::typed_refs::Aead as _;

        // set up buffer for ctxt and tag
        let mut msg_ctx: Vec<u8> = alloc::vec![0; msg.len() + alg.tag_len()];
        let (ctxt, tag) = msg_ctx.split_at_mut(msg.len());

        // set up nonce
        let nonce = alg.new_nonce(nonce).map_err(|_| Error::AeadInvalidNonce)?;

        // set up key
        let key = alg
            .new_key(key)
            .map_err(|_| Error::CryptoLibraryError("AEAD invalid key length".into()))?;

        // set up tag
        let tag = alg
            .new_tag_mut(tag)
            .map_err(|_| Error::CryptoLibraryError("Invalid tag length".into()))?;

        key.encrypt(ctxt, tag, nonce, aad, msg)
            .map_err(|_| Error::CryptoLibraryError("Invalid configuration".into()))?;

        Ok(msg_ctx)
    }

    fn aead_open(
        alg: AeadAlgorithm,
        key: &[u8],
        nonce: &[u8],
        aad: &[u8],
        cipher_txt: &[u8],
    ) -> Result<Vec<u8>, Error> {
        let alg = aead_alg(alg)?;

        use libcrux_traits::aead::typed_refs::{Aead as _, DecryptError};

        if cipher_txt.len() < alg.tag_len() {
            return Err(Error::AeadInvalidCiphertext);
        }

        let boundary = cipher_txt.len() - alg.tag_len();

        // set up buffers for ptext, ctext, and tag
        let mut ptext = alloc::vec![0; boundary];
        let (ctext, tag) = cipher_txt.split_at(boundary);

        // set up nonce
        let nonce = alg.new_nonce(nonce).map_err(|_| Error::AeadInvalidNonce)?;

        // set up key
        let key = alg
            .new_key(key)
            .map_err(|_| Error::CryptoLibraryError("AEAD invalid key length".into()))?;

        // set up tag
        let tag = alg
            .new_tag(tag)
            .map_err(|_| Error::CryptoLibraryError("Invalid tag length".into()))?;

        key.decrypt(&mut ptext, nonce, aad, ctext, tag)
            .map_err(|e| match e {
                DecryptError::InvalidTag => Error::AeadOpenError,
                _ => Error::CryptoLibraryError("Invalid configuration".into()),
            })?;

        Ok(ptext)
    }

    type HpkePrng = HpkeLibcruxPrng;

    fn prng() -> Self::HpkePrng {
        #[cfg(feature = "deterministic-prng")]
        {
            let mut fake_rng = alloc::vec![0u8; 256];
            rand_chacha::ChaCha20Rng::from_rng(&mut UnwrapErr(SysRng)).fill_bytes(&mut fake_rng);
            HpkeLibcruxPrng {
                fake_rng,
                rng: rand_chacha::ChaCha20Rng::from_rng(&mut UnwrapErr(SysRng)),
            }
        }

        #[cfg(not(feature = "deterministic-prng"))]
        HpkeLibcruxPrng {
            rng: rand_chacha::ChaCha20Rng::from_rng(&mut UnwrapErr(SysRng)),
        }
    }

    /// Returns an error if the KDF algorithm is not supported by this crypto provider.
    fn supports_kdf(alg: KdfAlgorithm) -> Result<(), Error> {
        match alg {
            KdfAlgorithm::HkdfSha256 | KdfAlgorithm::HkdfSha384 | KdfAlgorithm::HkdfSha512 => {
                Ok(())
            }

            #[cfg(feature = "draft-ietf-hpke-pq")]
            KdfAlgorithm::Shake128 | KdfAlgorithm::Shake256 => Ok(()),

            #[cfg(not(feature = "draft-ietf-hpke-pq"))]
            KdfAlgorithm::Shake128 | KdfAlgorithm::Shake256 => Err(Error::UnknownKdfAlgorithm),

            KdfAlgorithm::TurboShake128 | KdfAlgorithm::TurboShake256 => {
                Err(Error::UnknownKdfAlgorithm)
            }
        }
    }

    /// Returns an error if the KEM algorithm is not supported by this crypto provider.
    fn supports_kem(alg: KemAlgorithm) -> Result<(), Error> {
        match alg {
            KemAlgorithm::DhKem25519 | KemAlgorithm::DhKemP256 | KemAlgorithm::XWingDraft06 => {
                Ok(())
            }

            #[cfg(feature = "rustcrypto-p-curves")]
            KemAlgorithm::DhKemP384 | KemAlgorithm::DhKemP521 => Ok(()),

            #[cfg(any(
                feature = "draft-connolly-cfrg-hpke-mlkem",
                feature = "draft-ietf-hpke-pq"
            ))]
            KemAlgorithm::MlKem512 | KemAlgorithm::MlKem768 | KemAlgorithm::MlKem1024 => Ok(()),

            #[cfg(feature = "draft-ietf-hpke-pq")]
            KemAlgorithm::MlKem768P256 => Ok(()),

            #[cfg(all(feature = "draft-ietf-hpke-pq", feature = "rustcrypto-p-curves"))]
            KemAlgorithm::MlKem1024P384 => Ok(()),
            _ => Err(Error::UnknownKemAlgorithm),
        }
    }

    /// Returns an error if the AEAD algorithm is not supported by this crypto provider.
    fn supports_aead(_alg: AeadAlgorithm) -> Result<(), Error> {
        Ok(())
    }
}

#[inline(always)]
fn kem_ecdh_secret_to_public(alg: libcrux_ecdh::Algorithm, sk: &[u8]) -> Result<Vec<u8>, Error> {
    libcrux_ecdh::secret_to_public(alg, sk)
        .map_err(|e| Error::CryptoLibraryError(format!("ECDH derive base error: {:?}", e)))
        .map(|p| {
            if alg == libcrux_ecdh::Algorithm::P256 {
                nist_format_uncompressed(p)
            } else {
                p
            }
        })
}

/// Prepend 0x04 for uncompressed NIST curve points.
#[inline(always)]
fn nist_format_uncompressed(mut pk: Vec<u8>) -> Vec<u8> {
    let mut tmp = Vec::with_capacity(pk.len() + 1);
    tmp.push(0x04);
    tmp.append(&mut pk);
    tmp
}

#[inline(always)]
fn kdf_algorithm_to_libcrux_hkdf_algorithm(alg: TwoStageKdfAlgorithm) -> libcrux_hkdf::Algorithm {
    match alg {
        TwoStageKdfAlgorithm::HkdfSha256 => libcrux_hkdf::Algorithm::Sha256,
        TwoStageKdfAlgorithm::HkdfSha384 => libcrux_hkdf::Algorithm::Sha384,
        TwoStageKdfAlgorithm::HkdfSha512 => libcrux_hkdf::Algorithm::Sha512,
    }
}

/// `SHAKE<size>.Derive(concat(inputs), L) = SHAKE<size>(concat(inputs), d = 8*L)`.
///
/// See draft-ietf-hpke-pq Section 5.
fn shake_derive(alg: SingleStageKdfAlgorithm, inputs: &[&[u8]], len: usize) -> Vec<u8> {
    let ikm = concat(inputs);
    let mut out = alloc::vec![0u8; len];
    match alg {
        SingleStageKdfAlgorithm::Shake128 => libcrux_sha3::shake128_ema(&mut out, &ikm),
        SingleStageKdfAlgorithm::Shake256 => libcrux_sha3::shake256_ema(&mut out, &ikm),
        SingleStageKdfAlgorithm::TurboShake128 | SingleStageKdfAlgorithm::TurboShake256 => {
            // Not supported yet
            unreachable!()
        }
    }
    out
}

#[inline(always)]
fn kem_key_type_to_libcrux_alg(alg: KemAlgorithm) -> Result<libcrux_kem::Algorithm, Error> {
    match alg {
        KemAlgorithm::DhKem25519 => Ok(libcrux_kem::Algorithm::X25519),
        KemAlgorithm::DhKemP256 => Ok(libcrux_kem::Algorithm::Secp256r1),
        #[cfg(any(
            feature = "draft-connolly-cfrg-hpke-mlkem",
            feature = "draft-ietf-hpke-pq"
        ))]
        KemAlgorithm::MlKem512 => Ok(libcrux_kem::Algorithm::MlKem512),
        #[cfg(any(
            feature = "draft-connolly-cfrg-hpke-mlkem",
            feature = "draft-ietf-hpke-pq"
        ))]
        KemAlgorithm::MlKem768 => Ok(libcrux_kem::Algorithm::MlKem768),
        #[cfg(any(
            feature = "draft-connolly-cfrg-hpke-mlkem",
            feature = "draft-ietf-hpke-pq"
        ))]
        KemAlgorithm::MlKem1024 => Ok(libcrux_kem::Algorithm::MlKem1024),
        KemAlgorithm::XWingDraft06 => Ok(libcrux_kem::Algorithm::XWingKemDraft06),
        _ => Err(Error::UnknownKemAlgorithm),
    }
}

/// The encapsulation-randomness length for the post-quantum KEMs (`N_random`),
/// or `None` for the DH-based KEMs (which inject via `Hpke::random`). Used only
/// by the deterministic test path.
#[cfg(feature = "deterministic-prng")]
#[inline]
fn pq_encaps_randomness_len(alg: KemAlgorithm) -> Option<usize> {
    match alg {
        KemAlgorithm::MlKem512 | KemAlgorithm::MlKem768 | KemAlgorithm::MlKem1024 => Some(32),
        KemAlgorithm::XWingDraft06 => Some(64),
        #[cfg(feature = "draft-ietf-hpke-pq")]
        KemAlgorithm::MlKem768P256 => Some(32 + 128),
        #[cfg(feature = "draft-ietf-hpke-pq")]
        KemAlgorithm::MlKem1024P384 => Some(32 + 48),
        _ => None,
    }
}

/// Derandomized encapsulation (test-only), using the supplied `randomness` as
/// the KEM's encapsulation randomness. Used by the known-answer tests so that
/// the sender-side `enc` matches the vectors.
#[cfg(feature = "deterministic-prng")]
#[inline]
fn kem_encaps_derand(
    alg: KemAlgorithm,
    pk_r: &[u8],
    randomness: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), Error> {
    match alg {
        #[cfg(feature = "draft-ietf-hpke-pq")]
        KemAlgorithm::MlKem768P256 | KemAlgorithm::MlKem1024P384 => {
            hybrid::encaps_derand(alg, pk_r, randomness)
        }
        _ => {
            let kem_alg = kem_key_type_to_libcrux_alg(alg)?;
            let pk = libcrux_kem::PublicKey::decode(kem_alg, pk_r)
                .map_err(|_| Error::KemInvalidPublicKey)?;
            pk.encapsulate_derand(randomness)
                .map_err(|e| Error::CryptoLibraryError(format!("Encaps error {:?}", e)))
                .map(|(ss, ct)| (ss.encode(), ct.encode()))
        }
    }
}

#[inline(always)]
fn kem_key_type_to_ecdh_alg(alg: KemAlgorithm) -> Result<libcrux_ecdh::Algorithm, Error> {
    match alg {
        KemAlgorithm::DhKem25519 => Ok(libcrux_ecdh::Algorithm::X25519),
        KemAlgorithm::DhKemP256 => Ok(libcrux_ecdh::Algorithm::P256),
        _ => Err(Error::UnknownKemAlgorithm),
    }
}

#[inline(always)]
fn aead_alg(alg_type: AeadAlgorithm) -> Result<libcrux_aead::Aead, Error> {
    match alg_type {
        AeadAlgorithm::ChaCha20Poly1305 => Ok(libcrux_aead::Aead::ChaCha20Poly1305),
        AeadAlgorithm::Aes128Gcm => Ok(libcrux_aead::Aead::AesGcm128),
        AeadAlgorithm::Aes256Gcm => Ok(libcrux_aead::Aead::AesGcm256),
        _ => Err(Error::UnknownAeadAlgorithm),
    }
}

#[inline(always)]
fn concat(values: &[&[u8]]) -> Vec<u8> {
    values.join(&[][..])
}

/// ML-KEM/ECDH hybrid KEMs (`MLKEM768-P256`, `MLKEM1024-P384`).
///
/// The authoritative reference is `draft-ietf-hpke-pq` (it defines the HPKE
/// integration, the code points, and the test vectors). These are the
/// `MLKEM768-P256` / `MLKEM1024-P384` instances of
/// `draft-irtf-cfrg-concrete-hybrid-kems`, built with the `CG` framework
/// (C2PRI Combiner with a nominal Group `T`) from `draft-irtf-cfrg-hybrid-kems`:
/// the combiner is `SHA3-256(ss_PQ || ss_T || ct_T || ek_T || label)`. See
/// * https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-05 (authoritative)
/// * https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-concrete-hybrid-kems-03
/// * https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-hybrid-kems-09
///
/// * The decapsulation key is the 32-byte seed; key generation and
///   decapsulation expand it with SHAKE256 into `seed_pq (64) || seed_t` and
///   re-derive the components.
/// * Rejection-sampled scalars and uncompressed SEC1 point encodings for NIST;
///   `ss_T` is the x-coordinate of the DH result.
/// * Wire formats: `ek = ek_PQ || ek_T`, `ct = ct_PQ || ct_T`.
#[cfg(feature = "draft-ietf-hpke-pq")]
mod hybrid {
    use super::*;

    /// The nominal group (traditional component) of a hybrid KEM.
    #[derive(Clone, Copy)]
    enum NominalGroup {
        P256,
        P384,
    }

    /// Per-instance hybrid KEM parameters.
    struct Params {
        /// The ML-KEM variant.
        ml_alg: libcrux_kem::Algorithm,

        /// The nominal group.
        curve: NominalGroup,

        /// Domain-separation label for the combiner.
        label: &'static [u8],

        /// Encoded ML-KEM encapsulation-key length.
        ml_ek_len: usize,

        /// Encoded ML-KEM ciphertext length.
        ml_ct_len: usize,

        /// The group's seed length (`T::SEED_SIZE`). The seed expanded by
        /// [`expand`] is `64` (ML-KEM `seed_pq`) plus this.
        group_seed_len: usize,
    }

    #[inline]
    const fn params(alg: KemAlgorithm) -> Result<Params, Error> {
        match alg {
            // P-256: ML-KEM seed (64) + group seed (128 = 4 rejection windows),
            // matching the P-256 nominal-group `Nseed` of 128 in
            // `draft-irtf-cfrg-concrete-hybrid-kems` §3.1. The encap randomness
            // is `32 + 128 = 160`, as in the `draft-ietf-hpke-pq` vectors
            // (resolving https://github.com/hpkewg/hpke-pq/issues/59).
            KemAlgorithm::MlKem768P256 => Ok(Params {
                ml_alg: libcrux_kem::Algorithm::MlKem768,
                curve: NominalGroup::P256,
                label: b"MLKEM768-P256",
                ml_ek_len: 1184,
                ml_ct_len: 1088,
                group_seed_len: 128,
            }),
            // P-384: ML-KEM seed (64) + group seed (48).
            KemAlgorithm::MlKem1024P384 => Ok(Params {
                ml_alg: libcrux_kem::Algorithm::MlKem1024,
                curve: NominalGroup::P384,
                label: b"MLKEM1024-P384",
                ml_ek_len: 1568,
                ml_ct_len: 1568,
                group_seed_len: 48,
            }),
            _ => Err(Error::UnknownKemAlgorithm),
        }
    }

    /// `ss = SHA3-256(ss_PQ || ss_T || ct_T || ek_T || label)`.
    #[inline]
    fn combine(p: &Params, ss_pq: &[u8], ss_t: &[u8], ct_t: &[u8], ek_t: &[u8]) -> Vec<u8> {
        let input = concat(&[ss_pq, ss_t, ct_t, ek_t, p.label]);
        libcrux_sha3::sha256(&input).to_vec()
    }

    #[inline]
    fn split_at_or(data: &[u8], n: usize, err: Error) -> Result<(&[u8], &[u8]), Error> {
        data.split_at_checked(n).ok_or(err)
    }

    // --- Nominal group operations, matching concrete-hybrid-kems. ---
    //
    // The traditional component of each hybrid is just the corresponding
    // `DhKem*` group, so these delegate to the provider's own ECDH / key
    // derivation paths (`dh_validate_sk`, `secret_to_public`, `dh`) rather than
    // re-implementing scalar validation, point derivation, and ECDH.

    #[inline]
    fn scalar_size(curve: NominalGroup) -> usize {
        match curve {
            NominalGroup::P256 => 32,
            NominalGroup::P384 => 48,
        }
    }

    /// The `DhKem*` code point for `curve`'s nominal group.
    #[inline]
    fn curve_kem_alg(curve: NominalGroup) -> KemAlgorithm {
        match curve {
            NominalGroup::P256 => KemAlgorithm::DhKemP256,
            NominalGroup::P384 => KemAlgorithm::DhKemP384,
        }
    }

    /// Return the canonical scalar bytes if `bytes` is a valid non-zero scalar.
    #[inline]
    fn validate_scalar(curve: NominalGroup, bytes: &[u8]) -> Option<Vec<u8>> {
        <HpkeLibcrux as HpkeCrypto>::dh_validate_sk(curve_kem_alg(curve), bytes).ok()
    }

    /// `random_scalar`: rejection-sample successive `SCALAR_SIZE` windows,
    /// returning the first valid scalar. Bounded by `seed.len() / SCALAR_SIZE`
    /// windows; the trailing partial window (if any) is ignored, matching
    /// concrete-hybrid-kems.
    #[inline]
    fn random_scalar(curve: NominalGroup, seed: &[u8]) -> Result<Vec<u8>, Error> {
        seed.chunks_exact(scalar_size(curve))
            .find_map(|window| validate_scalar(curve, window))
            .ok_or(Error::KemInvalidSecretKey)
    }

    /// `exp(generator, scalar)` — the public key, uncompressed SEC1.
    #[inline]
    fn base_pub(curve: NominalGroup, scalar: &[u8]) -> Result<Vec<u8>, Error> {
        <HpkeLibcrux as HpkeCrypto>::secret_to_public(curve_kem_alg(curve), scalar)
    }

    /// `element_to_shared_secret(exp(peer, scalar))` — the DH x-coordinate.
    #[inline]
    fn ecdh(curve: NominalGroup, scalar: &[u8], peer: &[u8]) -> Result<Vec<u8>, Error> {
        <HpkeLibcrux as HpkeCrypto>::dh(curve_kem_alg(curve), peer, scalar)
    }

    /// The component key material expanded from a hybrid seed.
    struct Expanded {
        ek_pq: Vec<u8>,
        seed_pq: [u8; 64],
        dk_t: Vec<u8>,
        ek_t: Vec<u8>,
    }

    /// Expand the seed into the component encapsulation key (`ek_PQ || ek_T`),
    /// the ML-KEM `seed_pq`, and the group scalar `dk_T`.
    #[inline]
    fn expand(p: &Params, seed: &[u8]) -> Result<Expanded, Error> {
        let material = shake_derive(
            SingleStageKdfAlgorithm::Shake256,
            &[seed],
            64 + p.group_seed_len,
        );
        let mut seed_pq = [0u8; 64];
        seed_pq.copy_from_slice(&material[..64]);
        let seed_t = &material[64..];

        let (_dk_pq, ek_pq) = libcrux_kem::key_gen_derand(p.ml_alg, &seed_pq)
            .map(|(sk, pk)| (sk, pk.encode()))
            .map_err(|e| Error::CryptoLibraryError(format!("KEM key gen error: {:?}", e)))?;

        let dk_t = random_scalar(p.curve, seed_t)?;
        let ek_t = base_pub(p.curve, &dk_t)?;
        Ok(Expanded {
            ek_pq,
            seed_pq,
            dk_t,
            ek_t,
        })
    }

    #[inline]
    pub(super) fn key_gen(
        alg: KemAlgorithm,
        prng: &mut HpkeLibcruxPrng,
    ) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let mut seed = alloc::vec![0u8; 32];
        prng.try_fill_bytes(&mut seed)
            .map_err(|_| Error::InsufficientRandomness)?;
        key_gen_derand(alg, &seed)
    }

    #[inline]
    /// Deterministic key generation: the 32-byte seed is the decapsulation key.
    pub(super) fn key_gen_derand(
        alg: KemAlgorithm,
        seed: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let p = params(alg)?;
        let e = expand(&p, seed)?;
        Ok((concat(&[&e.ek_pq, &e.ek_t]), seed.to_vec()))
    }

    #[inline]
    pub(super) fn encaps(
        alg: KemAlgorithm,
        pk_r: &[u8],
        prng: &mut HpkeLibcruxPrng,
    ) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let p = params(alg)?;
        let (ek_pq, ek_t) = split_at_or(pk_r, p.ml_ek_len, Error::KemInvalidPublicKey)?;

        // Post-quantum encapsulation (draws ML-KEM randomness first).
        let ml_pk = libcrux_kem::PublicKey::decode(p.ml_alg, ek_pq)
            .map_err(|_| Error::KemInvalidPublicKey)?;
        let (ss_pq, ct_pq) = ml_pk
            .encapsulate(prng)
            .map(|(ss, ct)| (ss.encode(), ct.encode()))
            .map_err(|e| Error::CryptoLibraryError(format!("Encaps error {:?}", e)))?;

        // Traditional encapsulation: ephemeral scalar from `T::SEED_SIZE` bytes.
        let mut seed_e = alloc::vec![0u8; p.group_seed_len];
        prng.try_fill_bytes(&mut seed_e)
            .map_err(|_| Error::InsufficientRandomness)?;
        let sk_e = random_scalar(p.curve, &seed_e)?;
        let ct_t = base_pub(p.curve, &sk_e)?;
        let ss_t = ecdh(p.curve, &sk_e, ek_t)?;

        let ss = combine(&p, &ss_pq, &ss_t, &ct_t, ek_t);
        Ok((ss, concat(&[&ct_pq, &ct_t])))
    }

    /// Derandomized encapsulation: `randomness = randomness_PQ (32) || seed_T`.
    #[cfg(feature = "deterministic-prng")]
    #[inline]
    pub(super) fn encaps_derand(
        alg: KemAlgorithm,
        pk_r: &[u8],
        randomness: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>), Error> {
        let p = params(alg)?;
        let (ek_pq, ek_t) = split_at_or(pk_r, p.ml_ek_len, Error::KemInvalidPublicKey)?;
        let (rand_pq, seed_e) = split_at_or(randomness, 32, Error::InsufficientRandomness)?;

        let ml_pk = libcrux_kem::PublicKey::decode(p.ml_alg, ek_pq)
            .map_err(|_| Error::KemInvalidPublicKey)?;
        let (ss_pq, ct_pq) = ml_pk
            .encapsulate_derand(rand_pq)
            .map(|(ss, ct)| (ss.encode(), ct.encode()))
            .map_err(|e| Error::CryptoLibraryError(format!("Encaps error {:?}", e)))?;

        let sk_e = random_scalar(p.curve, seed_e)?;
        let ct_t = base_pub(p.curve, &sk_e)?;
        let ss_t = ecdh(p.curve, &sk_e, ek_t)?;

        let ss = combine(&p, &ss_pq, &ss_t, &ct_t, ek_t);
        Ok((ss, concat(&[&ct_pq, &ct_t])))
    }

    #[inline]
    pub(super) fn decaps(alg: KemAlgorithm, ct: &[u8], sk_r: &[u8]) -> Result<Vec<u8>, Error> {
        let p = params(alg)?;
        let (ct_pq, ct_t) = split_at_or(ct, p.ml_ct_len, Error::KemInvalidCiphertext)?;

        // Re-expand the seed (`sk_r`) into the component keys.
        let e = expand(&p, sk_r)?;

        let (dk_pq, _ek_pq) = libcrux_kem::key_gen_derand(p.ml_alg, &e.seed_pq)
            .map_err(|err| Error::CryptoLibraryError(format!("KEM key gen error: {:?}", err)))?;
        let ct_pq =
            libcrux_kem::Ct::decode(p.ml_alg, ct_pq).map_err(|_| Error::KemInvalidCiphertext)?;
        let ss_pq = ct_pq
            .decapsulate(&dk_pq)
            .map(|ss| ss.encode())
            .map_err(|err| Error::CryptoLibraryError(format!("Decaps error {:?}", err)))?;

        let ss_t = ecdh(p.curve, &e.dk_t, ct_t)?;
        Ok(combine(&p, &ss_pq, &ss_t, ct_t, &e.ek_t))
    }
}

impl TryCryptoRng for HpkeLibcruxPrng {}

impl TryRng for HpkeLibcruxPrng {
    // TODO: Make use of fallible drbg.
    type Error = core::convert::Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.rng.next_u32())
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.rng.next_u64())
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        self.rng.fill_bytes(dst);
        Ok(())
    }
}

impl HpkeTestRng for HpkeLibcruxPrng {
    type Error = Error;

    #[cfg(feature = "deterministic-prng")]
    fn try_fill_test_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        // Here we fake our randomness for testing.
        if dest.len() > self.fake_rng.len() {
            return Err(Error::InsufficientRandomness);
        }
        dest.clone_from_slice(&self.fake_rng.split_off(self.fake_rng.len() - dest.len()));
        Ok(())
    }

    #[cfg(not(feature = "deterministic-prng"))]
    fn try_fill_test_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        use hpke_rs_crypto::Rng;

        self.fill_bytes(dest);
        Ok(())
    }

    #[cfg(feature = "deterministic-prng")]
    fn seed(&mut self, seed: &[u8]) {
        self.fake_rng = seed.to_vec();
    }
    #[cfg(not(feature = "deterministic-prng"))]
    fn seed(&mut self, _: &[u8]) {}
}

impl Display for HpkeLibcrux {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", Self::name())
    }
}

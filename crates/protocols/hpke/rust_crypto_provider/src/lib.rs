#![doc = include_str!("../Readme.md")]
#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::fmt::Display;
use rand::{rngs::SysRng, Rng};
use rand_core::{SeedableRng, UnwrapErr};
use zeroize::Zeroize;

use hpke_rs_crypto::{
    error::Error,
    types::{
        AeadAlgorithm, KdfAlgorithm, KemAlgorithm, SingleStageKdfAlgorithm, TwoStageKdfAlgorithm,
    },
    HpkeCrypto, HpkeTestRng,
};
use p256::{
    elliptic_curve::ecdh::diffie_hellman as p256diffie_hellman, PublicKey as p256PublicKey,
    SecretKey as p256SecretKey,
};

use k256::{
    elliptic_curve::{ecdh::diffie_hellman as k256diffie_hellman, sec1::ToEncodedPoint},
    PublicKey as k256PublicKey, SecretKey as k256SecretKey,
};

use p384::{
    elliptic_curve::ecdh::diffie_hellman as p384diffie_hellman, PublicKey as p384PublicKey,
    SecretKey as p384SecretKey,
};

use p521::{
    elliptic_curve::ecdh::diffie_hellman as p521diffie_hellman, PublicKey as p521PublicKey,
    SecretKey as p521SecretKey,
};

use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519StaticSecret};

mod aead;
mod hkdf;
// XXX: These are broken and pre-releases. Disabling them until they are stable.
#[cfg(feature = "experimental")]
mod pq_kem;
use crate::hkdf::*;
use crate::{aead::*, rand_shim::RandShim};
mod rand_shim;

/// The Rust Crypto HPKE Provider
#[derive(Debug)]
pub struct HpkeRustCrypto {}

/// The PRNG for the Rust Crypto Provider.
pub struct HpkeRustCryptoPrng {
    rng: rand_chacha::ChaCha20Rng,
    #[cfg(feature = "deterministic-prng")]
    fake_rng: Vec<u8>,
}

impl Zeroize for HpkeRustCryptoPrng {
    fn zeroize(&mut self) {
        // ChaCha20Rng doesn't implement zeroize and fake_rng is just for testing.
    }
}

impl HpkeCrypto for HpkeRustCrypto {
    fn name() -> String {
        "RustCrypto".into()
    }

    fn kdf_extract(alg: TwoStageKdfAlgorithm, salt: &[u8], ikm: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(match alg {
            TwoStageKdfAlgorithm::HkdfSha256 => sha256_extract(salt, ikm),
            TwoStageKdfAlgorithm::HkdfSha384 => sha384_extract(salt, ikm),
            TwoStageKdfAlgorithm::HkdfSha512 => sha512_extract(salt, ikm),
        })
    }

    fn kdf_expand(
        alg: TwoStageKdfAlgorithm,
        prk: &[u8],
        info: &[u8],
        output_size: usize,
    ) -> Result<Vec<u8>, Error> {
        match alg {
            TwoStageKdfAlgorithm::HkdfSha256 => sha256_expand(prk, info, output_size),
            TwoStageKdfAlgorithm::HkdfSha384 => sha384_expand(prk, info, output_size),
            TwoStageKdfAlgorithm::HkdfSha512 => sha512_expand(prk, info, output_size),
        }
    }

    fn kdf_derive(_alg: SingleStageKdfAlgorithm, _ikm: &[u8], _l: usize) -> Result<Vec<u8>, Error> {
        // The RustCrypto provider does not implement the single-stage (SHAKE)
        // KDFs of draft-ietf-hpke-pq.
        Err(Error::UnknownKdfAlgorithm)
    }

    fn dh(alg: KemAlgorithm, pk: &[u8], sk: &[u8]) -> Result<Vec<u8>, Error> {
        use subtle::ConstantTimeEq;
        match alg {
            KemAlgorithm::DhKem25519 => {
                if sk.len() != 32 {
                    return Err(Error::KemInvalidSecretKey);
                }
                if pk.len() != 32 {
                    return Err(Error::KemInvalidPublicKey);
                }
                assert!(pk.len() == 32);
                assert!(sk.len() == 32);
                let sk_array: [u8; 32] = sk.try_into().map_err(|_| Error::KemInvalidSecretKey)?;
                let pk_array: [u8; 32] = pk.try_into().map_err(|_| Error::KemInvalidPublicKey)?;
                let sk = X25519StaticSecret::from(sk_array);
                let shared_secret = sk
                    .diffie_hellman(&X25519PublicKey::from(pk_array))
                    .as_bytes()
                    .to_vec();

                if shared_secret.ct_eq(&[0u8; 32]).into() {
                    return Err(Error::KemInvalidPublicKey);
                }
                Ok(shared_secret)
            }
            KemAlgorithm::DhKemP256 => {
                let sk = p256SecretKey::from_slice(sk).map_err(|_| Error::KemInvalidSecretKey)?;
                let pk =
                    p256PublicKey::from_sec1_bytes(pk).map_err(|_| Error::KemInvalidPublicKey)?;
                Ok(p256diffie_hellman(sk.to_nonzero_scalar(), pk.as_affine())
                    .raw_secret_bytes()
                    .as_slice()
                    .into())
            }
            KemAlgorithm::DhKemP384 => {
                let sk = p384SecretKey::from_slice(sk).map_err(|_| Error::KemInvalidSecretKey)?;
                let pk =
                    p384PublicKey::from_sec1_bytes(pk).map_err(|_| Error::KemInvalidPublicKey)?;
                Ok(p384diffie_hellman(sk.to_nonzero_scalar(), pk.as_affine())
                    .raw_secret_bytes()
                    .as_slice()
                    .into())
            }
            KemAlgorithm::DhKemP521 => {
                let sk = p521SecretKey::from_slice(sk).map_err(|_| Error::KemInvalidSecretKey)?;
                let pk =
                    p521PublicKey::from_sec1_bytes(pk).map_err(|_| Error::KemInvalidPublicKey)?;
                Ok(p521diffie_hellman(sk.to_nonzero_scalar(), pk.as_affine())
                    .raw_secret_bytes()
                    .as_slice()
                    .into())
            }
            KemAlgorithm::DhKemK256 => {
                let sk = k256SecretKey::from_slice(sk).map_err(|_| Error::KemInvalidSecretKey)?;
                let pk =
                    k256PublicKey::from_sec1_bytes(pk).map_err(|_| Error::KemInvalidPublicKey)?;
                Ok(k256diffie_hellman(sk.to_nonzero_scalar(), pk.as_affine())
                    .raw_secret_bytes()
                    .as_slice()
                    .into())
            }
            _ => Err(Error::UnknownKemAlgorithm),
        }
    }

    fn kem_key_gen_derand(_alg: KemAlgorithm, _seed: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Error> {
        // XXX: These are broken and pre-releases. Disabling them until they are stable.
        #[cfg(feature = "experimental")]
        return pq_kem::kem_key_gen_derand(_alg, _seed);

        #[cfg(not(feature = "experimental"))]
        Err(Error::UnsupportedKemOperation)
    }

    fn kem_encaps(
        _alg: KemAlgorithm,
        _pk_r: &[u8],
        _prng: &mut Self::HpkePrng,
    ) -> Result<(Vec<u8>, Vec<u8>), Error> {
        // XXX: These are broken and pre-releases. Disabling them until they are stable.
        #[cfg(feature = "experimental")]
        return pq_kem::kem_encaps(_alg, _pk_r, _prng);

        #[cfg(not(feature = "experimental"))]
        Err(Error::UnsupportedKemOperation)
    }

    fn kem_decaps(_alg: KemAlgorithm, _ct: &[u8], _sk_r: &[u8]) -> Result<Vec<u8>, Error> {
        // XXX: These are broken and pre-releases. Disabling them until they are stable.
        #[cfg(feature = "experimental")]
        return pq_kem::kem_decaps(_alg, _ct, _sk_r);

        #[cfg(not(feature = "experimental"))]
        Err(Error::UnsupportedKemOperation)
    }

    fn secret_to_public(alg: KemAlgorithm, sk: &[u8]) -> Result<Vec<u8>, Error> {
        match alg {
            KemAlgorithm::DhKem25519 => {
                if sk.len() != 32 {
                    return Err(Error::KemInvalidSecretKey);
                }
                assert!(sk.len() == 32);
                let sk_array: [u8; 32] = sk.try_into().map_err(|_| Error::KemInvalidSecretKey)?;
                let sk = X25519StaticSecret::from(sk_array);
                Ok(X25519PublicKey::from(&sk).as_bytes().to_vec())
            }
            KemAlgorithm::DhKemP256 => {
                let sk = p256SecretKey::from_slice(sk).map_err(|_| Error::KemInvalidSecretKey)?;
                Ok(sk.public_key().to_encoded_point(false).as_bytes().into())
            }
            KemAlgorithm::DhKemP384 => {
                let sk = p384SecretKey::from_slice(sk).map_err(|_| Error::KemInvalidSecretKey)?;
                Ok(sk.public_key().to_encoded_point(false).as_bytes().into())
            }
            KemAlgorithm::DhKemP521 => {
                let sk = p521SecretKey::from_slice(sk).map_err(|_| Error::KemInvalidSecretKey)?;
                Ok(sk.public_key().to_encoded_point(false).as_bytes().into())
            }
            KemAlgorithm::DhKemK256 => {
                let sk = k256SecretKey::from_slice(sk).map_err(|_| Error::KemInvalidSecretKey)?;
                Ok(sk.public_key().to_encoded_point(false).as_bytes().into())
            }
            _ => Err(Error::UnsupportedKemOperation),
        }
    }

    fn kem_key_gen(
        alg: KemAlgorithm,
        prng: &mut Self::HpkePrng,
    ) -> Result<(Vec<u8>, Vec<u8>), Error> {
        match alg {
            KemAlgorithm::DhKem25519 => {
                let rng = &mut prng.rng;
                let sk = X25519StaticSecret::random_from_rng(RandShim(rng));
                let pk = X25519PublicKey::from(&sk).as_bytes().to_vec();
                let sk = sk.to_bytes().to_vec();
                Ok((pk, sk))
            }
            KemAlgorithm::DhKemP256 => {
                let rng = &mut prng.rng;
                let sk = p256SecretKey::random(&mut RandShim(rng));
                let pk = sk.public_key().to_encoded_point(false).as_bytes().into();
                let sk = sk.to_bytes().as_slice().into();
                Ok((pk, sk))
            }
            KemAlgorithm::DhKemP384 => {
                let rng = &mut prng.rng;
                let sk = p384SecretKey::random(&mut RandShim(rng));
                let pk = sk.public_key().to_encoded_point(false).as_bytes().into();
                let sk = sk.to_bytes().as_slice().into();
                Ok((pk, sk))
            }
            KemAlgorithm::DhKemP521 => {
                let rng = &mut prng.rng;
                let sk = p521SecretKey::random(&mut RandShim(rng));
                let pk = sk.public_key().to_encoded_point(false).as_bytes().into();
                let sk = sk.to_bytes().as_slice().into();
                Ok((pk, sk))
            }
            KemAlgorithm::DhKemK256 => {
                let rng = &mut prng.rng;
                let sk = k256SecretKey::random(&mut RandShim(rng));
                let pk = sk.public_key().to_encoded_point(false).as_bytes().into();
                let sk = sk.to_bytes().as_slice().into();
                Ok((pk, sk))
            }
            // XXX: These are broken and pre-releases. Disabling them until they
            //      are stable.
            #[allow(deprecated)]
            #[cfg(feature = "experimental")]
            KemAlgorithm::XWingDraft06
            | KemAlgorithm::XWingDraft06Obsolete
            | KemAlgorithm::MlKem768
            | KemAlgorithm::MlKem1024 => pq_kem::kem_key_gen(alg, prng),
            _ => Err(Error::UnknownKemAlgorithm),
        }
    }

    fn dh_validate_sk(alg: KemAlgorithm, sk: &[u8]) -> Result<Vec<u8>, Error> {
        match alg {
            KemAlgorithm::DhKemP256 => p256SecretKey::from_slice(sk)
                .map_err(|_| Error::KemInvalidSecretKey)
                .map(|_| sk.into()),
            KemAlgorithm::DhKemP384 => p384SecretKey::from_slice(sk)
                .map_err(|_| Error::KemInvalidSecretKey)
                .map(|_| sk.into()),
            KemAlgorithm::DhKemP521 => p521SecretKey::from_slice(sk)
                .map_err(|_| Error::KemInvalidSecretKey)
                .map(|_| sk.into()),
            KemAlgorithm::DhKemK256 => k256SecretKey::from_slice(sk)
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
        match alg {
            AeadAlgorithm::Aes128Gcm => aes128_seal(key, nonce, aad, msg),
            AeadAlgorithm::Aes256Gcm => aes256_seal(key, nonce, aad, msg),
            AeadAlgorithm::ChaCha20Poly1305 => chacha_seal(key, nonce, aad, msg),
            AeadAlgorithm::HpkeExport => Err(Error::UnknownAeadAlgorithm),
        }
    }

    fn aead_open(
        alg: AeadAlgorithm,
        key: &[u8],
        nonce: &[u8],
        aad: &[u8],
        msg: &[u8],
    ) -> Result<Vec<u8>, Error> {
        match alg {
            AeadAlgorithm::Aes128Gcm => aes128_open(alg, key, nonce, aad, msg),
            AeadAlgorithm::Aes256Gcm => aes256_open(alg, key, nonce, aad, msg),
            AeadAlgorithm::ChaCha20Poly1305 => chacha_open(alg, key, nonce, aad, msg),
            AeadAlgorithm::HpkeExport => Err(Error::UnknownAeadAlgorithm),
        }
    }

    type HpkePrng = HpkeRustCryptoPrng;

    fn prng() -> Self::HpkePrng {
        let rng = rand_chacha::ChaCha20Rng::from_rng(&mut UnwrapErr(SysRng));

        #[cfg(feature = "deterministic-prng")]
        {
            use rand::Rng;

            let mut fake_rng = alloc::vec![0u8; 256];
            let mut rng = rng;
            rng.fill_bytes(&mut fake_rng);

            HpkeRustCryptoPrng { fake_rng, rng }
        }
        #[cfg(not(feature = "deterministic-prng"))]
        HpkeRustCryptoPrng { rng }
    }

    /// Returns an error if the KDF algorithm is not supported by this crypto provider.
    fn supports_kdf(alg: KdfAlgorithm) -> Result<(), Error> {
        match alg {
            KdfAlgorithm::HkdfSha256 | KdfAlgorithm::HkdfSha384 | KdfAlgorithm::HkdfSha512 => {
                Ok(())
            }
            // The SHAKE KDFs (draft-ietf-hpke-pq) are not supported here yet.
            KdfAlgorithm::TurboShake128
            | KdfAlgorithm::TurboShake256
            | KdfAlgorithm::Shake128
            | KdfAlgorithm::Shake256 => Err(Error::UnknownKdfAlgorithm),
        }
    }

    /// Returns an error if the KEM algorithm is not supported by this crypto provider.
    fn supports_kem(alg: KemAlgorithm) -> Result<(), Error> {
        match alg {
            KemAlgorithm::DhKem25519
            | KemAlgorithm::DhKemP256
            | KemAlgorithm::DhKemK256
            | KemAlgorithm::DhKemP384
            | KemAlgorithm::DhKemP521 => Ok(()),
            // XXX: These are broken and pre-releases. Disabling them until they are stable.
            #[cfg(feature = "experimental")]
            KemAlgorithm::XWingDraft06 | KemAlgorithm::MlKem768 | KemAlgorithm::MlKem1024 => Ok(()),
            _ => Err(Error::UnknownKemAlgorithm),
        }
    }

    /// Returns an error if the AEAD algorithm is not supported by this crypto provider.
    fn supports_aead(alg: AeadAlgorithm) -> Result<(), Error> {
        match alg {
            AeadAlgorithm::Aes128Gcm
            | AeadAlgorithm::Aes256Gcm
            | AeadAlgorithm::ChaCha20Poly1305
            | AeadAlgorithm::HpkeExport => Ok(()),
        }
    }
}

impl rand_core::TryRng for HpkeRustCryptoPrng {
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

impl rand_core::TryCryptoRng for HpkeRustCryptoPrng {}

impl HpkeTestRng for HpkeRustCryptoPrng {
    #[cfg(feature = "deterministic-prng")]
    fn try_fill_test_bytes(&mut self, dest: &mut [u8]) -> Result<(), HpkeTestRngError> {
        // Here we fake our randomness for testing.
        if dest.len() > self.fake_rng.len() {
            return Err(HpkeTestRngError::InsufficientRandomness);
        }
        dest.clone_from_slice(&self.fake_rng.split_off(self.fake_rng.len() - dest.len()));
        Ok(())
    }

    #[cfg(feature = "deterministic-prng")]
    fn seed(&mut self, seed: &[u8]) {
        self.fake_rng = seed.to_vec();
    }

    #[cfg(not(feature = "deterministic-prng"))]
    fn try_fill_test_bytes(&mut self, dest: &mut [u8]) -> Result<(), HpkeTestRngError> {
        use rand::Rng;
        self.rng.fill_bytes(dest);
        Ok(())
    }

    #[cfg(not(feature = "deterministic-prng"))]
    fn seed(&mut self, _: &[u8]) {}

    type Error = HpkeTestRngError;
}

impl Display for HpkeRustCrypto {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", Self::name())
    }
}

#[derive(Debug)]
pub enum HpkeTestRngError {
    InsufficientRandomness,
}

impl Display for HpkeTestRngError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HpkeTestRngError::InsufficientRandomness => write!(f, "Insufficient randomness"),
        }
    }
}

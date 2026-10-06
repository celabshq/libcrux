use alloc::{vec, vec::Vec};

use hpke_rs_crypto::{error::Error, types::KemAlgorithm, HpkeCrypto, TryRng};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{dh_kem, util, Hpke};

/// A KEM private key wrapper.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct PrivateKey(pub(crate) Vec<u8>);
pub(crate) type PublicKey = Vec<u8>;

#[inline(always)]
fn ciphersuite(alg: KemAlgorithm) -> Vec<u8> {
    util::concat(&[b"KEM", &(alg as u16).to_be_bytes()])
}

pub(crate) fn encaps<Crypto: HpkeCrypto>(
    hpke: &mut Hpke<Crypto>,
    pk_r: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let alg = hpke.kem_id;
    match alg {
        KemAlgorithm::DhKemP256
        | KemAlgorithm::DhKemK256
        | KemAlgorithm::DhKemP384
        | KemAlgorithm::DhKemP521
        | KemAlgorithm::DhKem25519
        | KemAlgorithm::DhKem448 => {
            let randomness = hpke
                .random(alg.private_key_len())
                .map_err(|_| Error::InsufficientRandomness)?;
            dh_kem::encaps::<Crypto>(alg, pk_r, &ciphersuite(alg), &randomness)
        }
        #[allow(deprecated)]
        KemAlgorithm::XWingDraft06
        | KemAlgorithm::XWingDraft06Obsolete
        | KemAlgorithm::MlKem512
        | KemAlgorithm::MlKem768
        | KemAlgorithm::MlKem1024
        | KemAlgorithm::MlKem768P256
        | KemAlgorithm::MlKem1024P384 => {
            // Derandomized, same as the DH-KEM arm above: `Hpke::random`
            // supplies the encapsulation randomness (drawn from the prng in
            // `Hpke`, whatever it's backed by; see `HpkeCrypto::HpkePrng`),
            // and `kem_encaps_derand` is a pure function of it.
            let n = alg
                .encaps_randomness_len()
                .ok_or(Error::UnknownKemAlgorithm)?;
            let randomness = hpke.random(n).map_err(|_| Error::InsufficientRandomness)?;
            match Crypto::kem_encaps_derand(alg, pk_r, &randomness) {
                // Providers without derandomized encapsulation.
                Err(Error::UnsupportedKemOperation) => {
                    Crypto::kem_encaps(alg, pk_r, &mut hpke.prng)
                }
                r => r,
            }
        }
    }
}

pub(crate) fn decaps<Crypto: HpkeCrypto>(
    alg: KemAlgorithm,
    enc: &[u8],
    sk_r: &[u8],
) -> Result<Vec<u8>, Error> {
    match alg {
        KemAlgorithm::DhKemP256
        | KemAlgorithm::DhKemK256
        | KemAlgorithm::DhKemP384
        | KemAlgorithm::DhKemP521
        | KemAlgorithm::DhKem25519
        | KemAlgorithm::DhKem448 => dh_kem::decaps::<Crypto>(alg, enc, sk_r, &ciphersuite(alg)),
        #[allow(deprecated)]
        KemAlgorithm::XWingDraft06
        | KemAlgorithm::XWingDraft06Obsolete
        | KemAlgorithm::MlKem512
        | KemAlgorithm::MlKem768
        | KemAlgorithm::MlKem1024
        | KemAlgorithm::MlKem768P256
        | KemAlgorithm::MlKem1024P384 => Crypto::kem_decaps(alg, enc, sk_r),
    }
}

pub(crate) fn auth_encaps<Crypto: HpkeCrypto>(
    hpke: &mut Hpke<Crypto>,
    pk_r: &[u8],
    sk_s: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let alg = hpke.kem_id;
    match alg {
        KemAlgorithm::DhKemP256
        | KemAlgorithm::DhKemK256
        | KemAlgorithm::DhKemP384
        | KemAlgorithm::DhKemP521
        | KemAlgorithm::DhKem25519
        | KemAlgorithm::DhKem448 => {
            let randomness = hpke
                .random(alg.private_key_len())
                .map_err(|_| Error::InsufficientRandomness)?;
            dh_kem::auth_encaps::<Crypto>(alg, pk_r, sk_s, &ciphersuite(alg), &randomness)
        }
        #[allow(deprecated)]
        KemAlgorithm::XWingDraft06
        | KemAlgorithm::XWingDraft06Obsolete
        | KemAlgorithm::MlKem512
        | KemAlgorithm::MlKem768
        | KemAlgorithm::MlKem1024
        | KemAlgorithm::MlKem768P256
        | KemAlgorithm::MlKem1024P384 => Err(Error::UnsupportedKemOperation),
    }
}

pub(crate) fn auth_decaps<Crypto: HpkeCrypto>(
    alg: KemAlgorithm,
    enc: &[u8],
    sk_r: &[u8],
    pk_s: &[u8],
) -> Result<Vec<u8>, Error> {
    match alg {
        KemAlgorithm::DhKemP256
        | KemAlgorithm::DhKemK256
        | KemAlgorithm::DhKemP384
        | KemAlgorithm::DhKemP521
        | KemAlgorithm::DhKem25519
        | KemAlgorithm::DhKem448 => {
            dh_kem::auth_decaps::<Crypto>(alg, enc, sk_r, pk_s, &ciphersuite(alg))
        }
        #[allow(deprecated)]
        KemAlgorithm::XWingDraft06
        | KemAlgorithm::XWingDraft06Obsolete
        | KemAlgorithm::MlKem512
        | KemAlgorithm::MlKem768
        | KemAlgorithm::MlKem1024
        | KemAlgorithm::MlKem768P256
        | KemAlgorithm::MlKem1024P384 => Err(Error::UnsupportedKemOperation),
    }
}

/// Returns (private, public)
pub(crate) fn key_gen<Crypto: HpkeCrypto>(
    alg: KemAlgorithm,
    prng: &mut Crypto::HpkePrng,
) -> Result<(PrivateKey, Vec<u8>), Error> {
    match alg {
        // For ECDH based keys, we generate a completely fresh key.
        KemAlgorithm::DhKemP256
        | KemAlgorithm::DhKemK256
        | KemAlgorithm::DhKemP384
        | KemAlgorithm::DhKemP521
        | KemAlgorithm::DhKem25519
        | KemAlgorithm::DhKem448 => dh_kem::key_gen::<Crypto>(alg, prng),
        #[allow(deprecated)]
        KemAlgorithm::XWingDraft06
        | KemAlgorithm::XWingDraft06Obsolete
        | KemAlgorithm::MlKem512
        | KemAlgorithm::MlKem768
        | KemAlgorithm::MlKem1024
        | KemAlgorithm::MlKem768P256
        | KemAlgorithm::MlKem1024P384 => {
            let mut seed = zeroize::Zeroizing::new(vec![0u8; alg.private_key_len()]);
            prng.try_fill_bytes(&mut seed)
                .map_err(|_| Error::InsufficientRandomness)?;
            let (pk, sk) = derive_key_pair::<Crypto>(alg, &seed)?;
            Ok((sk, pk))
        }
    }
}

/// Derive key pair from the input key material `ikm`.
///
/// Returns (PublicKey, PrivateKey).
pub(crate) fn derive_key_pair<Crypto: HpkeCrypto>(
    alg: KemAlgorithm,
    ikm: &[u8],
) -> Result<(PublicKey, PrivateKey), Error> {
    match alg {
        KemAlgorithm::DhKemP256
        | KemAlgorithm::DhKemK256
        | KemAlgorithm::DhKemP384
        | KemAlgorithm::DhKemP521
        | KemAlgorithm::DhKem25519
        | KemAlgorithm::DhKem448 => dh_kem::derive_key_pair::<Crypto>(alg, &ciphersuite(alg), ikm),
        #[allow(deprecated)]
        KemAlgorithm::XWingDraft06 | KemAlgorithm::XWingDraft06Obsolete => {
            let seed = pq_derive_keypair_seed(alg, ikm, 32)?;
            Crypto::kem_key_gen_derand(alg, &seed).map(|(ek, dk)| (ek, PrivateKey(dk)))
        }
        KemAlgorithm::MlKem512 | KemAlgorithm::MlKem768 | KemAlgorithm::MlKem1024 => {
            let seed = pq_derive_keypair_seed(alg, ikm, 64)?;
            Crypto::kem_key_gen_derand(alg, &seed).map(|(ek, dk)| (ek, PrivateKey(dk)))
        }
        KemAlgorithm::MlKem768P256 | KemAlgorithm::MlKem1024P384 => {
            // The hybrid `DeriveKeyPair` produces a 32-byte seed that the
            // crypto provider expands into the component key material.
            let seed = pq_derive_keypair_seed(alg, ikm, 32)?;
            Crypto::kem_key_gen_derand(alg, &seed).map(|(ek, dk)| (ek, PrivateKey(dk)))
        }
    }
}

/// Derive the `DeriveKeyPair` seed of length `len` for a post-quantum KEM.
///
/// This is `SHAKE256.LabeledDerive(ikm, "DeriveKeyPair", "", len)` with the KEM
/// `suite_id` (`"KEM" || kem_id`), as specified by `draft-ietf-hpke-pq`. The
/// deprecated `draft-connolly-cfrg-hpke-mlkem` instead uses the unlabeled
/// `SHAKE256(ikm, len)`. Either way SHAKE256 is fixed by the KEM (independent of
/// the HPKE KDF), so it is computed directly with `libcrux_sha3` rather than via
/// the crypto provider's KDF — that way it works for any provider whose
/// ML-KEM/X-Wing KEMs route through here.
fn pq_derive_keypair_seed(alg: KemAlgorithm, ikm: &[u8], len: usize) -> Result<Vec<u8>, Error> {
    #[cfg(feature = "draft-connolly-cfrg-hpke-mlkem")]
    let _ = alg;

    // The deprecated `draft-connolly-cfrg-hpke-mlkem` derives the seed from the
    // unlabeled `ikm`; every other (current) configuration labels it first with the
    // `SHAKE256.LabeledDerive(ikm, "DeriveKeyPair", "", len)` input:
    // `ikm ‖ "HPKE-v1" ‖ suite_id ‖ I2OSP(len(label),2) ‖ label ‖ I2OSP(L,2)`.
    #[cfg(not(feature = "draft-connolly-cfrg-hpke-mlkem"))]
    let ikm = &util::concat(&[
        ikm,
        crate::kdf::HPKE_VERSION,
        &ciphersuite(alg),
        &crate::kdf::length_prefixed(b"DeriveKeyPair"),
        &(len as u16).to_be_bytes(),
    ]);

    Ok(match len {
        32 => libcrux_sha3::shake256::<32>(ikm).to_vec(),
        64 => libcrux_sha3::shake256::<64>(ikm).to_vec(),
        _ => return Err(Error::InsufficientRandomness),
    })
}

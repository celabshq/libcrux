//! ML-KEM/ECDH hybrid KEMs `MLKEM768-P256` and `MLKEM1024-P384`.
//!
//! Implements the instances of draft-irtf-cfrg-concrete-hybrid-kems as used by
//! <https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-05>. Outputs are
//! byte-compatible with the `hybrid` module of the libcrux provider
//! (`hpke-rs-libcrux`).
//!
//! The decapsulation key is the 32-byte seed. Encapsulation keys are
//! `ek_PQ || ek_T` and ciphertexts are `ct_PQ || ct_T`, with `ek_T` and `ct_T`
//! as uncompressed SEC1 points.

use alloc::{vec, vec::Vec};

use hpke_rs_crypto::{error::Error, types::KemAlgorithm, HpkeCrypto};
use ml_kem::{Decapsulate, KeyExport, TryKeyInit};
use rand_core::Rng;
use sha3::{Digest, Sha3_256};
use shake::{ExtendableOutput, Shake256, Update, XofReader};

use crate::{HpkeRustCrypto, HpkeRustCryptoPrng};

/// Length of the hybrid seed, which is also the decapsulation key.
const SEED_LEN: usize = 32;

/// Length of the ML-KEM encapsulation randomness at the start of the hybrid
/// encapsulation randomness.
const ML_KEM_RANDOMNESS_LEN: usize = 32;

#[derive(Clone, Copy)]
enum MlKem {
    MlKem768,
    MlKem1024,
}

struct Params {
    ml_kem: MlKem,

    /// The `DhKem*` algorithm of the nominal group, used for its scalar
    /// validation, base point multiplication and ECDH.
    group: KemAlgorithm,

    /// Domain-separation label of the combiner.
    label: &'static [u8],

    ml_ek_len: usize,
    ml_ct_len: usize,

    /// `Nseed` of the nominal group, the length of `seed_T` in the expanded
    /// seed and of `seed_E` in the encapsulation randomness.
    group_seed_len: usize,

    /// Length of an uncompressed SEC1 point (`ek_T`, `ct_T`).
    element_len: usize,
}

const fn params(alg: KemAlgorithm) -> Result<Params, Error> {
    match alg {
        KemAlgorithm::MlKem768P256 => Ok(Params {
            ml_kem: MlKem::MlKem768,
            group: KemAlgorithm::DhKemP256,
            label: b"MLKEM768-P256",
            ml_ek_len: 1184,
            ml_ct_len: 1088,
            group_seed_len: 128,
            element_len: 65,
        }),
        KemAlgorithm::MlKem1024P384 => Ok(Params {
            ml_kem: MlKem::MlKem1024,
            group: KemAlgorithm::DhKemP384,
            label: b"MLKEM1024-P384",
            ml_ek_len: 1568,
            ml_ct_len: 1568,
            group_seed_len: 48,
            element_len: 97,
        }),
        _ => Err(Error::UnknownKemAlgorithm),
    }
}

/// Splits `data` into its first `first_len` bytes and the rest.
///
/// # Errors
/// Returns `err` unless `data` is exactly `first_len + second_len` bytes long.
fn split_exact(
    data: &[u8],
    first_len: usize,
    second_len: usize,
    err: Error,
) -> Result<(&[u8], &[u8]), Error> {
    match data.split_at_checked(first_len) {
        Some((first, second)) if second.len() == second_len => Ok((first, second)),
        _ => Err(err),
    }
}

/// `ss = SHA3-256(ss_PQ || ss_T || ct_T || ek_T || label)`.
fn combine(p: &Params, ss_pq: &[u8], ss_t: &[u8], ct_t: &[u8], ek_t: &[u8]) -> Vec<u8> {
    Sha3_256::new()
        .chain_update(ss_pq)
        .chain_update(ss_t)
        .chain_update(ct_t)
        .chain_update(ek_t)
        .chain_update(p.label)
        .finalize()
        .to_vec()
}

/// Returns the first scalar-sized window of `seed` that is a valid nonzero
/// scalar. A trailing partial window is ignored.
///
/// # Errors
/// Returns [`Error::KemInvalidSecretKey`] if no window is a valid scalar.
fn random_scalar(p: &Params, seed: &[u8]) -> Result<Vec<u8>, Error> {
    seed.chunks_exact(p.group.private_key_len())
        .find_map(|window| <HpkeRustCrypto as HpkeCrypto>::dh_validate_sk(p.group, window).ok())
        .ok_or(Error::KemInvalidSecretKey)
}

/// The component keys derived from a hybrid seed.
struct Expanded {
    seed_pq: ml_kem::Seed,
    dk_t: Vec<u8>,
    ek_t: Vec<u8>,
}

/// Expands `seed` with `SHAKE256(seed, 64 + group_seed_len)` into the ML-KEM
/// seed and the nominal group key pair.
///
/// # Errors
/// Returns [`Error::KemInvalidSecretKey`] if `seed` isn't 32 bytes long or no
/// group seed window is a valid scalar.
fn expand(p: &Params, seed: &[u8]) -> Result<Expanded, Error> {
    if seed.len() != SEED_LEN {
        return Err(Error::KemInvalidSecretKey);
    }
    let mut shake = Shake256::default();
    shake.update(seed);
    let mut reader = shake.finalize_xof();
    let mut seed_pq = ml_kem::Seed::default();
    reader.read(&mut seed_pq);
    let mut seed_t = vec![0u8; p.group_seed_len];
    reader.read(&mut seed_t);

    let dk_t = random_scalar(p, &seed_t)?;
    let ek_t = <HpkeRustCrypto as HpkeCrypto>::secret_to_public(p.group, &dk_t)?;
    Ok(Expanded {
        seed_pq,
        dk_t,
        ek_t,
    })
}

fn ml_kem_encapsulation_key(ml_kem: MlKem, seed_pq: ml_kem::Seed) -> Vec<u8> {
    match ml_kem {
        MlKem::MlKem768 => ml_kem::DecapsulationKey768::from_seed(seed_pq)
            .encapsulation_key()
            .to_bytes()
            .to_vec(),
        MlKem::MlKem1024 => ml_kem::DecapsulationKey1024::from_seed(seed_pq)
            .encapsulation_key()
            .to_bytes()
            .to_vec(),
    }
}

/// Returns `(shared_secret, ciphertext)`.
fn ml_kem_encaps(
    ml_kem: MlKem,
    ek: &[u8],
    randomness: &ml_kem::B32,
) -> Result<(Vec<u8>, Vec<u8>), Error> {
    match ml_kem {
        MlKem::MlKem768 => {
            let ek = ml_kem::EncapsulationKey768::new_from_slice(ek)
                .map_err(|_| Error::KemInvalidPublicKey)?;
            let (ct, ss) = ek.encapsulate_deterministic(randomness);
            Ok((ss.to_vec(), ct.to_vec()))
        }
        MlKem::MlKem1024 => {
            let ek = ml_kem::EncapsulationKey1024::new_from_slice(ek)
                .map_err(|_| Error::KemInvalidPublicKey)?;
            let (ct, ss) = ek.encapsulate_deterministic(randomness);
            Ok((ss.to_vec(), ct.to_vec()))
        }
    }
}

fn ml_kem_decaps(ml_kem: MlKem, seed_pq: ml_kem::Seed, ct: &[u8]) -> Result<Vec<u8>, Error> {
    match ml_kem {
        MlKem::MlKem768 => {
            let ct = ml_kem::ml_kem_768::Ciphertext::try_from(ct)
                .map_err(|_| Error::KemInvalidCiphertext)?;
            let dk = ml_kem::DecapsulationKey768::from_seed(seed_pq);
            Ok(dk.decapsulate(&ct).to_vec())
        }
        MlKem::MlKem1024 => {
            let ct = ml_kem::ml_kem_1024::Ciphertext::try_from(ct)
                .map_err(|_| Error::KemInvalidCiphertext)?;
            let dk = ml_kem::DecapsulationKey1024::from_seed(seed_pq);
            Ok(dk.decapsulate(&ct).to_vec())
        }
    }
}

/// Generates a key pair from a random 32-byte seed.
/// Returns `(encapsulation_key, decapsulation_key)`.
pub(crate) fn key_gen(
    alg: KemAlgorithm,
    prng: &mut HpkeRustCryptoPrng,
) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let mut seed = [0u8; SEED_LEN];
    prng.fill_bytes(&mut seed);
    key_gen_derand(alg, &seed)
}

/// Derives the key pair for the 32-byte `seed`, which is also the
/// decapsulation key. Returns `(encapsulation_key, decapsulation_key)`.
pub(crate) fn key_gen_derand(alg: KemAlgorithm, seed: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let p = params(alg)?;
    let expanded = expand(&p, seed)?;
    let mut ek = ml_kem_encapsulation_key(p.ml_kem, expanded.seed_pq);
    ek.extend_from_slice(&expanded.ek_t);
    Ok((ek, seed.to_vec()))
}

/// Encapsulates to `pk_r` with fresh randomness.
/// Returns `(shared_secret, ciphertext)`.
pub(crate) fn encaps(
    alg: KemAlgorithm,
    pk_r: &[u8],
    prng: &mut HpkeRustCryptoPrng,
) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let len = alg
        .encaps_randomness_len()
        .ok_or(Error::UnknownKemAlgorithm)?;
    let mut randomness = vec![0u8; len];
    prng.fill_bytes(&mut randomness);
    encaps_derand(alg, pk_r, &randomness)
}

/// Encapsulates to `pk_r` with `randomness = r_PQ (32) || seed_E`.
/// Returns `(shared_secret, ciphertext)`.
pub(crate) fn encaps_derand(
    alg: KemAlgorithm,
    pk_r: &[u8],
    randomness: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let p = params(alg)?;
    let (ek_pq, ek_t) = split_exact(pk_r, p.ml_ek_len, p.element_len, Error::KemInvalidPublicKey)?;
    let (r_pq, seed_e) = split_exact(
        randomness,
        ML_KEM_RANDOMNESS_LEN,
        p.group_seed_len,
        Error::InsufficientRandomness,
    )?;
    let r_pq = ml_kem::B32::try_from(r_pq).map_err(|_| Error::InsufficientRandomness)?;

    let (ss_pq, ct_pq) = ml_kem_encaps(p.ml_kem, ek_pq, &r_pq)?;

    let sk_e = random_scalar(&p, seed_e)?;
    let ct_t = <HpkeRustCrypto as HpkeCrypto>::secret_to_public(p.group, &sk_e)?;
    let ss_t = <HpkeRustCrypto as HpkeCrypto>::dh(p.group, ek_t, &sk_e)?;

    let ss = combine(&p, &ss_pq, &ss_t, &ct_t, ek_t);
    let mut ct = ct_pq;
    ct.extend_from_slice(&ct_t);
    Ok((ss, ct))
}

/// Decapsulates `ct` with the 32-byte seed `sk_r`. Returns the shared secret.
pub(crate) fn decaps(alg: KemAlgorithm, ct: &[u8], sk_r: &[u8]) -> Result<Vec<u8>, Error> {
    let p = params(alg)?;
    let (ct_pq, ct_t) = split_exact(ct, p.ml_ct_len, p.element_len, Error::KemInvalidCiphertext)?;
    let expanded = expand(&p, sk_r)?;

    let ss_pq = ml_kem_decaps(p.ml_kem, expanded.seed_pq, ct_pq)?;
    let ss_t = <HpkeRustCrypto as HpkeCrypto>::dh(p.group, ct_t, &expanded.dk_t)
        .map_err(|_| Error::KemInvalidCiphertext)?;

    Ok(combine(&p, &ss_pq, &ss_t, ct_t, &expanded.ek_t))
}

//! DH KEM as described in §4.1. DH-Based KEM.

use alloc::{string::ToString, vec::Vec};

use hpke_rs_crypto::{
    error::Error,
    types::{KdfAlgorithm, KemAlgorithm, TwoStageKdfAlgorithm},
    HpkeCrypto,
};

use crate::util::{concat, SecretBytes};
use crate::{
    kdf::{labeled_expand, labeled_extract},
    kem::*,
};

/// The two-stage KDF for a DH-based KEM. DH KEMs always pair with an HKDF KDF,
/// so the conversion is infallible in practice.
#[inline]
fn dh_kdf(alg: KemAlgorithm) -> Result<TwoStageKdfAlgorithm, Error> {
    TwoStageKdfAlgorithm::try_from(KdfAlgorithm::from(alg))
}

fn extract_and_expand<Crypto: HpkeCrypto>(
    alg: KemAlgorithm,
    dh: &[u8],
    kem_context: &[u8],
    suite_id: &[u8],
) -> Result<SecretBytes, Error> {
    let kdf = dh_kdf(alg)?;
    let prk = labeled_extract::<Crypto>(kdf, &[], suite_id, "eae_prk", dh)?;
    labeled_expand::<Crypto>(
        kdf,
        &prk,
        suite_id,
        "shared_secret",
        kem_context,
        alg.shared_secret_len(),
    )
    .map(SecretBytes::from)
}

/// Serialize public key.
/// This is an identity function for X25519.
/// Because P256 public keys are already encoded before it is the identity
/// function here as well.
#[inline(always)]
pub(super) fn serialize(pk: &[u8]) -> Vec<u8> {
    pk.to_vec()
}

#[inline(always)]
pub(super) fn deserialize(enc: &[u8]) -> Vec<u8> {
    enc.to_vec()
}

/// Return (private, public)
pub(super) fn key_gen<Crypto: HpkeCrypto>(
    alg: KemAlgorithm,
    prng: &mut Crypto::HpkePrng,
) -> Result<(PrivateKey, Vec<u8>), Error> {
    let (pk, sk) = Crypto::kem_key_gen(alg, prng)?;
    Ok((PrivateKey(sk), pk))
}

pub(super) fn derive_key_pair<Crypto: HpkeCrypto>(
    alg: KemAlgorithm,
    suite_id: &[u8],
    ikm: &[u8],
) -> Result<(PublicKey, PrivateKey), Error> {
    let kdf = dh_kdf(alg)?;
    let dkp_prk = labeled_extract::<Crypto>(kdf, &[], suite_id, "dkp_prk", ikm)?;

    let sk = match alg {
        KemAlgorithm::DhKem25519 => PrivateKey(labeled_expand::<Crypto>(
            kdf,
            &dkp_prk,
            suite_id,
            "sk",
            &[],
            alg.private_key_len(),
        )?),
        KemAlgorithm::DhKemP256
        | KemAlgorithm::DhKemP384
        | KemAlgorithm::DhKemP521
        | KemAlgorithm::DhKemK256 => {
            // RFC 9180 §7.1.3: bitmask is 0x01 for P-521, 0xFF otherwise
            let bitmask: u8 = match alg {
                KemAlgorithm::DhKemP521 => 0x01,
                _ => 0xFF,
            };
            let mut ctr = 0u8;
            // Do rejection sampling trying to find a valid key.
            // It is expected that there aren't too many iterations and that
            // the loop will always terminate.
            loop {
                let candidate = labeled_expand::<Crypto>(
                    kdf,
                    &dkp_prk,
                    suite_id,
                    "candidate",
                    &ctr.to_be_bytes(),
                    alg.private_key_len(),
                );
                if let Ok(sk) = candidate {
                    let mut sk = PrivateKey(sk);
                    sk.0[0] &= bitmask;
                    if let Ok(sk) = Crypto::dh_validate_sk(alg, &sk.0) {
                        break PrivateKey(sk);
                    }
                }
                if ctr == u8::MAX {
                    // If we get here we lost. This should never happen.
                    return Err(Error::CryptoLibraryError(
                        "Unable to generate a valid private key".to_string(),
                    ));
                }
                ctr += 1;
            }
        }
        _ => {
            return Err(Error::UnknownKemAlgorithm);
        }
    };
    Ok((Crypto::secret_to_public(alg, &sk.0)?, sk))
}

pub(super) fn encaps<Crypto: HpkeCrypto>(
    alg: KemAlgorithm,
    pk_r: &[u8],
    suite_id: &[u8],
    randomness: &[u8],
) -> Result<(SecretBytes, Vec<u8>), Error> {
    debug_assert_eq!(randomness.len(), alg.private_key_len());
    let (pk_e, sk_e) = derive_key_pair::<Crypto>(alg, suite_id, randomness)?;
    let dh = SecretBytes::from(Crypto::dh(alg, pk_r, &sk_e.0)?);
    let enc = serialize(&pk_e);

    let pk_rm = serialize(pk_r);
    let kem_context = concat(&[&enc, &pk_rm]);

    let zz = extract_and_expand::<Crypto>(alg, &dh, &kem_context, suite_id)?;
    Ok((zz, enc))
}

pub(super) fn decaps<Crypto: HpkeCrypto>(
    alg: KemAlgorithm,
    enc: &[u8],
    sk_r: &[u8],
    suite_id: &[u8],
) -> Result<SecretBytes, Error> {
    let pk_e = deserialize(enc);
    let dh = SecretBytes::from(Crypto::dh(alg, &pk_e, sk_r)?);

    let pk_rm = serialize(&Crypto::secret_to_public(alg, sk_r)?);
    let kem_context = concat(&[enc, &pk_rm]);

    extract_and_expand::<Crypto>(alg, &dh, &kem_context, suite_id)
}

pub(super) fn auth_encaps<Crypto: HpkeCrypto>(
    alg: KemAlgorithm,
    pk_r: &[u8],
    sk_s: &[u8],
    suite_id: &[u8],
    randomness: &[u8],
) -> Result<(SecretBytes, Vec<u8>), Error> {
    debug_assert_eq!(randomness.len(), alg.private_key_len());
    let (pk_e, sk_e) = derive_key_pair::<Crypto>(alg, suite_id, randomness)?;
    let dh_e = SecretBytes::from(Crypto::dh(alg, pk_r, &sk_e.0)?);
    let dh_s = SecretBytes::from(Crypto::dh(alg, pk_r, sk_s)?);
    let dh = SecretBytes::from(concat(&[&dh_e, &dh_s]));

    let enc = serialize(&pk_e);
    let pk_rm = serialize(pk_r);
    let pk_sm = serialize(&Crypto::secret_to_public(alg, sk_s)?);

    let kem_context = concat(&[&enc, &pk_rm, &pk_sm]);

    let zz = extract_and_expand::<Crypto>(alg, &dh, &kem_context, suite_id)?;
    Ok((zz, enc))
}

pub(super) fn auth_decaps<Crypto: HpkeCrypto>(
    alg: KemAlgorithm,
    enc: &[u8],
    sk_r: &[u8],
    pk_s: &[u8],
    suite_id: &[u8],
) -> Result<SecretBytes, Error> {
    let pk_e = deserialize(enc);
    let dh_e = SecretBytes::from(Crypto::dh(alg, &pk_e, sk_r)?);
    let dh_s = SecretBytes::from(Crypto::dh(alg, pk_s, sk_r)?);
    let dh = SecretBytes::from(concat(&[&dh_e, &dh_s]));

    let pk_rm = serialize(&Crypto::secret_to_public(alg, sk_r)?);
    let pk_sm = serialize(pk_s);
    let kem_context = concat(&[enc, &pk_rm, &pk_sm]);

    extract_and_expand::<Crypto>(alg, &dh, &kem_context, suite_id)
}

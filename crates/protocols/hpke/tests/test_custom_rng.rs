//! Demonstrates bringing your own Rng to the libcrux HPKE provider, instead
//! of the default HMAC-DRBG: `HpkeLibcrux<R>` works for any `R: TryCryptoRng`,
//! so a caller passes their own Rng straight into `Hpke::new_with_rng` — no
//! wrapper type, no library-specific trait to implement.

extern crate hpke_rs as hpke;

use hpke::hpke_types::*;
use hpke::prelude::*;
use hpke_rs_crypto::TryCryptoRng;
use hpke_rs_libcrux::{HpkeLibcrux, HpkeLibcruxPrng};
use rand::{rngs::SysRng, SeedableRng};
use rand_chacha::ChaCha20Rng;

fn seal_open_round_trip<R: TryCryptoRng + 'static>(rng: R) {
    let mut hpke = Hpke::<HpkeLibcrux<R>>::new_with_rng(
        HpkeMode::Base,
        KemAlgorithm::DhKem25519,
        KdfAlgorithm::HkdfSha256,
        AeadAlgorithm::ChaCha20Poly1305,
        rng,
    );

    let (sk_r, pk_r) = hpke.generate_key_pair().unwrap().into_keys();
    let info = b"custom rng test info";
    let aad = b"custom rng test aad";
    let plaintext = b"custom rng test plaintext";

    let (enc, ctxt) = hpke
        .seal(&pk_r, info, aad, plaintext, None, None, None)
        .unwrap();
    let ptxt = hpke
        .open(&enc, &sk_r, info, aad, &ctxt, None, None, None)
        .unwrap();

    assert_eq!(ptxt, plaintext);
}

#[test]
fn custom_rng_chacha20() {
    seal_open_round_trip(ChaCha20Rng::from_seed([7u8; 32]));
}

#[test]
fn custom_rng_sys_rng() {
    seal_open_round_trip(SysRng);
}

fn seeded_sender_enc(seed: [u8; 32], pk_r: &HpkePublicKey) -> Vec<u8> {
    let mut hpke = Hpke::<HpkeLibcrux>::new_with_rng(
        HpkeMode::Base,
        KemAlgorithm::DhKem25519,
        KdfAlgorithm::HkdfSha256,
        AeadAlgorithm::ChaCha20Poly1305,
        HpkeLibcruxPrng::from_seed(seed),
    );
    let (enc, _) = hpke.setup_sender(pk_r, b"info", None, None, None).unwrap();
    enc
}

/// All randomness comes from the caller's PRNG: the same seed gives the same `enc`.
#[test]
fn from_seed_is_deterministic() {
    let mut hpke = Hpke::<HpkeLibcrux>::new(
        HpkeMode::Base,
        KemAlgorithm::DhKem25519,
        KdfAlgorithm::HkdfSha256,
        AeadAlgorithm::ChaCha20Poly1305,
    );
    let (_sk_r, pk_r) = hpke.generate_key_pair().unwrap().into_keys();

    assert_eq!(
        seeded_sender_enc([1u8; 32], &pk_r),
        seeded_sender_enc([1u8; 32], &pk_r)
    );
    assert_ne!(
        seeded_sender_enc([1u8; 32], &pk_r),
        seeded_sender_enc([2u8; 32], &pk_r)
    );
}

/// A clone gets a freshly seeded PRNG rather than a copy of the original's state.
#[test]
fn clone_uses_fresh_prng() {
    let mut hpke = Hpke::<HpkeLibcrux>::new_with_rng(
        HpkeMode::Base,
        KemAlgorithm::DhKem25519,
        KdfAlgorithm::HkdfSha256,
        AeadAlgorithm::ChaCha20Poly1305,
        HpkeLibcruxPrng::from_seed([3u8; 32]),
    );
    let mut clone = hpke.clone();
    let (_sk_r, pk_r) = hpke.generate_key_pair().unwrap().into_keys();

    let (enc, _) = hpke.setup_sender(&pk_r, b"info", None, None, None).unwrap();
    let (enc_clone, _) = clone
        .setup_sender(&pk_r, b"info", None, None, None)
        .unwrap();
    assert_ne!(enc, enc_clone);
}

//! Tests the RustCrypto provider's hybrid KEMs MLKEM768-P256 and
//! MLKEM1024-P384 against the draft-ietf-hpke-pq test vectors and, with the
//! `draft-ietf-hpke-pq` feature, against the libcrux provider.

// The deprecated `draft-connolly-cfrg-hpke-mlkem` feature changes the
// `DeriveKeyPair` of every post-quantum KEM to an unlabeled SHAKE256, which
// doesn't match the draft-ietf-hpke-pq vectors.
#![cfg(all(
    feature = "rustcrypto",
    feature = "experimental",
    not(feature = "draft-connolly-cfrg-hpke-mlkem")
))]

extern crate hpke_rs as hpke;

use std::fs::File;
use std::io::BufReader;

use hpke::prelude::*;
use hpke::test_util::hex_to_bytes;
use hpke_rs_crypto::error::Error;
use hpke_rs_crypto::types::{AeadAlgorithm, KdfAlgorithm, KemAlgorithm};
use hpke_rs_crypto::{HpkeCrypto, HpkeDefaultPrng};
use hpke_rs_rust_crypto::HpkeRustCrypto;
use serde::Deserialize;

#[derive(Deserialize)]
struct HpkeTestVector {
    mode: u8,
    kem_id: u16,
    kdf_id: u16,
    aead_id: u16,
    info: String,
    #[serde(rename = "ikmR")]
    ikm_r: String,
    #[serde(rename = "ikmE")]
    ikm_e: String,
    #[serde(rename = "skRm")]
    sk_rm: String,
    #[serde(rename = "pkRm")]
    pk_rm: String,
    enc: String,
    shared_secret: String,
    key: String,
    base_nonce: String,
    exporter_secret: String,
    encryptions: Vec<CiphertextKat>,
}

#[derive(Deserialize)]
struct CiphertextKat {
    aad: String,
    ct: String,
    pt: String,
}

fn check_kem(kem: KemAlgorithm, test: &HpkeTestVector) {
    let sk_rm = hex_to_bytes(&test.sk_rm);
    let pk_rm = hex_to_bytes(&test.pk_rm);
    let enc = hex_to_bytes(&test.enc);
    let shared_secret = hex_to_bytes(&test.shared_secret);

    let key_pair = HpkeRustCrypto::kem_key_gen_derand(kem, &sk_rm).expect("kem_key_gen_derand");
    assert_eq!(key_pair, (pk_rm.clone(), sk_rm.clone()));

    let encapsulation = HpkeRustCrypto::kem_encaps_derand(kem, &pk_rm, &hex_to_bytes(&test.ikm_e))
        .expect("kem_encaps_derand");
    assert_eq!(encapsulation, (shared_secret.clone(), enc.clone()));
    let decapsulated = HpkeRustCrypto::kem_decaps(kem, &enc, &sk_rm).expect("kem_decaps");
    assert_eq!(decapsulated, shared_secret);
}

fn check_hpke(kem: KemAlgorithm, kdf: KdfAlgorithm, test: &HpkeTestVector) {
    let mode = HpkeMode::try_from(test.mode).expect("unknown mode");
    let aead = AeadAlgorithm::try_from(test.aead_id).expect("unknown AEAD");
    let hpke = Hpke::<HpkeRustCrypto>::new(mode, kem, kdf, aead);

    let (sk_r, pk_r) = hpke
        .derive_key_pair(&hex_to_bytes(&test.ikm_r))
        .expect("derive_key_pair")
        .into_keys();
    assert_eq!(sk_r.as_slice(), hex_to_bytes(&test.sk_rm));
    assert_eq!(pk_r.as_slice(), hex_to_bytes(&test.pk_rm));

    let mut context = hpke
        .setup_receiver(
            &hex_to_bytes(&test.enc),
            &sk_r,
            &hex_to_bytes(&test.info),
            None,
            None,
            None,
        )
        .expect("setup_receiver");
    assert_eq!(context.key(), hex_to_bytes(&test.key));
    assert_eq!(context.nonce(), hex_to_bytes(&test.base_nonce));
    assert_eq!(
        context.exporter_secret(),
        hex_to_bytes(&test.exporter_secret)
    );

    let encryption = &test.encryptions[0];
    let pt = context
        .open(
            &hex_to_bytes(&encryption.aad),
            &hex_to_bytes(&encryption.ct),
        )
        .expect("open");
    assert_eq!(pt, hex_to_bytes(&encryption.pt));
}

#[test]
fn hybrid_vectors() {
    let file = File::open("tests/test_vectors_hpke_pq.json").expect("open vector file");
    let tests: Vec<HpkeTestVector> =
        serde_json::from_reader(BufReader::new(file)).expect("parse vector file");

    let mut kem_checked = Vec::new();
    let mut hpke_checked = Vec::new();
    for test in &tests {
        let Ok(kem) = KemAlgorithm::try_from(test.kem_id) else {
            continue;
        };
        if !matches!(
            kem,
            KemAlgorithm::MlKem768P256 | KemAlgorithm::MlKem1024P384
        ) {
            continue;
        }
        let kdf = KdfAlgorithm::try_from(test.kdf_id).expect("unknown KDF");
        check_kem(kem, test);
        kem_checked.push((kem, kdf));
        // The provider has no SHAKE KDFs, so the HPKE-level checks skip
        // (0x50, SHAKE128).
        if HpkeRustCrypto::supports_kdf(kdf).is_ok() {
            check_hpke(kem, kdf, test);
            hpke_checked.push((kem, kdf));
        }
    }

    assert_eq!(
        kem_checked,
        [
            (KemAlgorithm::MlKem768P256, KdfAlgorithm::HkdfSha256),
            (KemAlgorithm::MlKem1024P384, KdfAlgorithm::HkdfSha384),
            (KemAlgorithm::MlKem768P256, KdfAlgorithm::Shake128),
        ]
    );
    assert_eq!(
        hpke_checked,
        [
            (KemAlgorithm::MlKem768P256, KdfAlgorithm::HkdfSha256),
            (KemAlgorithm::MlKem1024P384, KdfAlgorithm::HkdfSha384),
        ]
    );
}

const HYBRID_KEMS: [KemAlgorithm; 2] = [KemAlgorithm::MlKem768P256, KemAlgorithm::MlKem1024P384];

#[test]
fn randomized_key_gen_and_encaps_round_trip() {
    let mut prng = HpkeRustCrypto::try_prng().expect("prng");
    for kem in HYBRID_KEMS {
        let (pk, sk) = HpkeRustCrypto::kem_key_gen(kem, &mut prng).expect("kem_key_gen");
        assert_eq!(sk.len(), kem.private_key_len());
        assert_eq!(
            HpkeRustCrypto::kem_key_gen_derand(kem, &sk).expect("derand"),
            (pk.clone(), sk.clone())
        );

        let (ss, ct) = HpkeRustCrypto::kem_encaps(kem, &pk, &mut prng).expect("kem_encaps");
        assert_eq!(
            HpkeRustCrypto::kem_decaps(kem, &ct, &sk).expect("kem_decaps"),
            ss
        );
    }
}

#[test]
fn malformed_inputs_return_specific_errors() {
    let mut prng = HpkeRustCrypto::try_prng().expect("prng");
    for kem in HYBRID_KEMS {
        let (pk, sk) = HpkeRustCrypto::kem_key_gen(kem, &mut prng).expect("kem_key_gen");
        let randomness = vec![0x42; kem.encaps_randomness_len().expect("randomness length")];
        let (_ss, ct) = HpkeRustCrypto::kem_encaps_derand(kem, &pk, &randomness).expect("encaps");

        let short_seed = &sk[..sk.len() - 1];
        assert!(matches!(
            HpkeRustCrypto::kem_key_gen_derand(kem, short_seed),
            Err(Error::KemInvalidSecretKey)
        ));
        assert!(matches!(
            HpkeRustCrypto::kem_decaps(kem, &ct, short_seed),
            Err(Error::KemInvalidSecretKey)
        ));

        let mut off_curve_pk = pk.clone();
        *off_curve_pk.last_mut().expect("pk isn't empty") ^= 1;
        assert!(matches!(
            HpkeRustCrypto::kem_encaps_derand(kem, &off_curve_pk, &randomness),
            Err(Error::KemInvalidPublicKey)
        ));
        assert!(matches!(
            HpkeRustCrypto::kem_encaps_derand(kem, &pk[..pk.len() - 1], &randomness),
            Err(Error::KemInvalidPublicKey)
        ));
        assert!(matches!(
            HpkeRustCrypto::kem_encaps_derand(kem, &pk, &randomness[1..]),
            Err(Error::InsufficientRandomness)
        ));

        let mut off_curve_ct = ct.clone();
        *off_curve_ct.last_mut().expect("ct isn't empty") ^= 1;
        assert!(matches!(
            HpkeRustCrypto::kem_decaps(kem, &off_curve_ct, &sk),
            Err(Error::KemInvalidCiphertext)
        ));
        assert!(matches!(
            HpkeRustCrypto::kem_decaps(kem, &ct[..ct.len() - 1], &sk),
            Err(Error::KemInvalidCiphertext)
        ));
    }
}

#[cfg(feature = "draft-ietf-hpke-pq")]
mod libcrux {
    use super::*;

    use hpke_rs_libcrux::HpkeLibcrux;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    type RustCrypto = HpkeRustCrypto;
    type Libcrux = HpkeLibcrux;

    struct Hybrid {
        kem: KemAlgorithm,
        /// Length of an uncompressed SEC1 point (`ek_T`, `ct_T`).
        element_len: usize,
    }

    const HYBRIDS: &[Hybrid] = &[
        Hybrid {
            kem: KemAlgorithm::MlKem768P256,
            element_len: 65,
        },
        Hybrid {
            kem: KemAlgorithm::MlKem1024P384,
            element_len: 97,
        },
    ];

    /// Length of the ML-KEM randomness `r_PQ`. The group seed `seed_E` follows
    /// it in the encapsulation randomness.
    const ML_KEM_RANDOMNESS_LEN: usize = 32;

    const P256_SCALAR_LEN: usize = 32;

    fn rng() -> ChaCha20Rng {
        ChaCha20Rng::from_seed([0x5a; 32])
    }

    fn random_bytes(rng: &mut ChaCha20Rng, len: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; len];
        rng.fill_bytes(&mut bytes);
        bytes
    }

    fn encaps_randomness(rng: &mut ChaCha20Rng, kem: KemAlgorithm) -> Vec<u8> {
        let len = kem
            .encaps_randomness_len()
            .expect("hybrid KEMs take encapsulation randomness");
        random_bytes(rng, len)
    }

    /// Returns `(pk, sk)`.
    fn key_pair(rng: &mut ChaCha20Rng, kem: KemAlgorithm) -> (Vec<u8>, Vec<u8>) {
        RustCrypto::kem_key_gen_derand(kem, &random_bytes(rng, kem.private_key_len()))
            .expect("key gen")
    }

    fn assert_both_reject<T>(
        kem: KemAlgorithm,
        case: &str,
        rust_crypto: Result<T, Error>,
        libcrux: Result<T, Error>,
    ) {
        assert!(
            rust_crypto.is_err(),
            "RustCrypto accepted {case} for {kem:?}"
        );
        assert!(libcrux.is_err(), "libcrux accepted {case} for {kem:?}");
    }

    #[test]
    fn same_seed_gives_same_key_pair() {
        let mut rng = rng();
        for hybrid in HYBRIDS {
            let kem = hybrid.kem;
            for _ in 0..8 {
                let seed = random_bytes(&mut rng, kem.private_key_len());
                let rust_crypto =
                    RustCrypto::kem_key_gen_derand(kem, &seed).expect("RustCrypto key gen");
                let libcrux = Libcrux::kem_key_gen_derand(kem, &seed).expect("libcrux key gen");
                assert_eq!(rust_crypto, libcrux, "key pair mismatch for {kem:?}");
            }
        }
    }

    #[test]
    fn same_randomness_gives_same_encapsulation() {
        let mut rng = rng();
        for hybrid in HYBRIDS {
            let kem = hybrid.kem;
            for _ in 0..4 {
                let (pk, sk) = key_pair(&mut rng, kem);
                let randomness = encaps_randomness(&mut rng, kem);

                let rust_crypto = RustCrypto::kem_encaps_derand(kem, &pk, &randomness)
                    .expect("RustCrypto encaps");
                let libcrux =
                    Libcrux::kem_encaps_derand(kem, &pk, &randomness).expect("libcrux encaps");
                assert_eq!(rust_crypto, libcrux, "encapsulation mismatch for {kem:?}");

                let (ss, ct) = rust_crypto;
                let rust_crypto = RustCrypto::kem_decaps(kem, &ct, &sk).expect("RustCrypto decaps");
                let libcrux = Libcrux::kem_decaps(kem, &ct, &sk).expect("libcrux decaps");
                assert_eq!(
                    rust_crypto, ss,
                    "RustCrypto decapsulation mismatch for {kem:?}"
                );
                assert_eq!(libcrux, ss, "libcrux decapsulation mismatch for {kem:?}");
            }
        }
    }

    const SUITES: &[(KemAlgorithm, KdfAlgorithm, AeadAlgorithm)] = &[
        (
            KemAlgorithm::MlKem768P256,
            KdfAlgorithm::HkdfSha256,
            AeadAlgorithm::Aes128Gcm,
        ),
        (
            KemAlgorithm::MlKem768P256,
            KdfAlgorithm::HkdfSha384,
            AeadAlgorithm::Aes256Gcm,
        ),
        (
            KemAlgorithm::MlKem1024P384,
            KdfAlgorithm::HkdfSha256,
            AeadAlgorithm::Aes128Gcm,
        ),
        (
            KemAlgorithm::MlKem1024P384,
            KdfAlgorithm::HkdfSha384,
            AeadAlgorithm::Aes256Gcm,
        ),
    ];

    /// Seals to a key pair the receiver's provider generates and opens with
    /// the receiver's provider.
    fn seal_open<Sender, Receiver>(kem: KemAlgorithm, kdf: KdfAlgorithm, aead: AeadAlgorithm)
    where
        Sender: HpkeDefaultPrng + 'static,
        Receiver: HpkeDefaultPrng + 'static,
    {
        let info = b"cross-provider info";
        let aad = b"cross-provider aad";
        let msg = b"cross-provider message";

        let mut receiver = Hpke::<Receiver>::new(HpkeMode::Base, kem, kdf, aead);
        let key_pair = receiver.generate_key_pair().expect("key gen");

        let mut sender = Hpke::<Sender>::new(HpkeMode::Base, kem, kdf, aead);
        let (enc, ct) = sender
            .seal(key_pair.public_key(), info, aad, msg, None, None, None)
            .expect("seal");
        let pt = receiver
            .open(
                &enc,
                key_pair.private_key(),
                info,
                aad,
                &ct,
                None,
                None,
                None,
            )
            .expect("open");
        assert_eq!(
            pt,
            msg,
            "{} to {} failed for {kem:?}/{kdf:?}/{aead:?}",
            Sender::name(),
            Receiver::name()
        );
    }

    #[test]
    fn seal_open_across_providers() {
        for &(kem, kdf, aead) in SUITES {
            seal_open::<RustCrypto, Libcrux>(kem, kdf, aead);
            seal_open::<Libcrux, RustCrypto>(kem, kdf, aead);
        }
    }

    #[test]
    fn invalid_first_scalar_window_is_skipped_for_p256() {
        let kem = KemAlgorithm::MlKem768P256;
        let mut rng = rng();
        let (pk, _sk) = key_pair(&mut rng, kem);
        let randomness = encaps_randomness(&mut rng, kem);

        let first_window_start = ML_KEM_RANDOMNESS_LEN;
        let second_window_start = first_window_start + P256_SCALAR_LEN;

        // 0xff..ff exceeds the group order, so the second window becomes sk_E.
        let mut skipping = randomness.clone();
        skipping[first_window_start..second_window_start].fill(0xff);

        // The same r_PQ, with the second window copied into the first one.
        let mut direct = randomness;
        direct.copy_within(
            second_window_start..second_window_start + P256_SCALAR_LEN,
            first_window_start,
        );

        let rust_crypto = RustCrypto::kem_encaps_derand(kem, &pk, &skipping).expect("RustCrypto");
        let libcrux = Libcrux::kem_encaps_derand(kem, &pk, &skipping).expect("libcrux");
        assert_eq!(rust_crypto, libcrux);
        let direct = RustCrypto::kem_encaps_derand(kem, &pk, &direct).expect("RustCrypto direct");
        assert_eq!(rust_crypto, direct);
    }

    #[test]
    fn all_invalid_scalar_windows_fail_encapsulation() {
        let mut rng = rng();
        for hybrid in HYBRIDS {
            let kem = hybrid.kem;
            let (pk, _sk) = key_pair(&mut rng, kem);
            // P-256 has four scalar windows, P-384 only one.
            let mut randomness = encaps_randomness(&mut rng, kem);
            randomness[ML_KEM_RANDOMNESS_LEN..].fill(0xff);

            assert_both_reject(
                kem,
                "all-0xff seed_E",
                RustCrypto::kem_encaps_derand(kem, &pk, &randomness),
                Libcrux::kem_encaps_derand(kem, &pk, &randomness),
            );
        }
    }

    #[test]
    fn malformed_encapsulation_inputs_are_rejected() {
        let mut rng = rng();
        for hybrid in HYBRIDS {
            let kem = hybrid.kem;
            let (pk, _sk) = key_pair(&mut rng, kem);
            let randomness = encaps_randomness(&mut rng, kem);

            let truncated_pk = &pk[..pk.len() - 1];
            assert_both_reject(
                kem,
                "truncated pk_r",
                RustCrypto::kem_encaps_derand(kem, truncated_pk, &randomness),
                Libcrux::kem_encaps_derand(kem, truncated_pk, &randomness),
            );

            // libcrux accepts the following inputs for at least one of the
            // KEMs because it doesn't check their exact length: an overlong
            // pk_r (MLKEM768-P256), randomness one byte short (MLKEM768-P256)
            // and randomness one byte long (both). Only RustCrypto is checked.
            let overlong_pk = [pk.as_slice(), &[0]].concat();
            let short_randomness = &randomness[..randomness.len() - 1];
            let long_randomness = [randomness.as_slice(), &[0]].concat();
            for (case, pk, randomness) in [
                (
                    "overlong pk_r",
                    overlong_pk.as_slice(),
                    randomness.as_slice(),
                ),
                ("short randomness", pk.as_slice(), short_randomness),
                ("long randomness", pk.as_slice(), long_randomness.as_slice()),
            ] {
                assert!(
                    RustCrypto::kem_encaps_derand(kem, pk, randomness).is_err(),
                    "RustCrypto accepted {case} for {kem:?}"
                );
            }
        }
    }

    #[test]
    fn malformed_decapsulation_inputs_are_rejected() {
        let mut rng = rng();
        for hybrid in HYBRIDS {
            let kem = hybrid.kem;
            let (pk, sk) = key_pair(&mut rng, kem);
            let randomness = encaps_randomness(&mut rng, kem);
            let (_ss, ct) = RustCrypto::kem_encaps_derand(kem, &pk, &randomness).expect("encaps");

            let truncated_ct = &ct[..ct.len() - 1];
            assert_both_reject(
                kem,
                "truncated ct",
                RustCrypto::kem_decaps(kem, truncated_ct, &sk),
                Libcrux::kem_decaps(kem, truncated_ct, &sk),
            );

            let mut off_curve_ct = ct.clone();
            *off_curve_ct.last_mut().expect("ct isn't empty") ^= 1;
            assert_both_reject(
                kem,
                "off-curve ct_T",
                RustCrypto::kem_decaps(kem, &off_curve_ct, &sk),
                Libcrux::kem_decaps(kem, &off_curve_ct, &sk),
            );

            // ct_T = 0x04 || x || y becomes (0x02 | parity of y) || x.
            let (ct_pq, ct_t) = ct.split_at(ct.len() - hybrid.element_len);
            let coordinate_len = (hybrid.element_len - 1) / 2;
            let y_parity = ct_t[hybrid.element_len - 1] & 1;
            let compressed_ct = [ct_pq, &[0x02 | y_parity], &ct_t[1..=coordinate_len]].concat();

            // libcrux accepts the following inputs for at least one of the
            // KEMs: an overlong ct (MLKEM768-P256), sk_r of 31 or 33 bytes
            // (both, it expands a seed of any length) and a compressed ct_T
            // (both). Only RustCrypto is checked.
            let overlong_ct = [ct.as_slice(), &[0]].concat();
            let short_sk = &sk[..sk.len() - 1];
            let long_sk = [sk.as_slice(), &[0]].concat();
            for (case, ct, sk) in [
                ("overlong ct", overlong_ct.as_slice(), sk.as_slice()),
                ("compressed ct_T", compressed_ct.as_slice(), sk.as_slice()),
                ("truncated sk_r", ct.as_slice(), short_sk),
                ("overlong sk_r", ct.as_slice(), long_sk.as_slice()),
            ] {
                assert!(
                    RustCrypto::kem_decaps(kem, ct, sk).is_err(),
                    "RustCrypto accepted {case} for {kem:?}"
                );
            }
        }
    }
}

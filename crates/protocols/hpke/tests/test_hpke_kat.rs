extern crate hpke_rs as hpke;

use hpke_rs_rust_crypto::HpkeRustCrypto;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{self, Deserialize, Serialize};
use std::convert::TryInto;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

use hpke::prelude::*;
use hpke::test_util::{hex_to_bytes, hex_to_bytes_option, vec_to_option_slice};
use hpke_rs_crypto::{types::*, HpkeCrypto};
use hpke_rs_libcrux::HpkeLibcrux;

/// A single HPKE known-answer test vector.
///
/// Some fields are optional to accommodate the different test vector formats we
/// consume: the newer vectors (e.g. `test_vectors2.json`) omit the ephemeral key
/// pair (`skEm`/`pkEm`) and the intermediate `key_schedule_context`/`secret`
/// values, and add a `suite_id`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct HpkeTestVector {
    mode: u8,
    kem_id: u16,
    kdf_id: u16,
    aead_id: u16,
    info: String,
    ikmR: String,
    ikmS: Option<String>,
    ikmE: String,
    skRm: String,
    skSm: Option<String>,
    // Ephemeral key material is absent from the post-quantum vectors (the
    // encapsulation randomness is not expressed as an ephemeral key pair there).
    skEm: Option<String>,
    psk: Option<String>,
    psk_id: Option<String>,
    pkRm: String,
    pkSm: Option<String>,
    pkEm: Option<String>,
    enc: String,
    shared_secret: String,
    suite_id: Option<String>,
    key_schedule_context: Option<String>,
    secret: Option<String>,
    key: String,
    base_nonce: String,
    exporter_secret: String,
    encryptions: Vec<CiphertextKAT>,
    exports: Vec<ExportsKAT>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct CiphertextKAT {
    aad: String,
    ct: String,
    nonce: String,
    pt: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct ExportsKAT {
    exporter_context: String,
    L: usize,
    exported_value: String,
}

/// Run the known-answer tests for all `tests` supported by the `Crypto` backend,
/// and return the `KemAlgorithm` of every vector that was actually executed
/// (vectors skipped because the backend doesn't support the ciphersuite yield
/// `None`). The caller uses this to assert exactly which suites ran.
fn kat<Crypto: HpkeCrypto + 'static>(tests: Vec<HpkeTestVector>) -> Vec<KemAlgorithm> {
    // Replace into_par_iter() with into_iter() to run tests sequentially.
    tests
        .into_par_iter()
        .filter_map(|test| {
            println!(
                "Testing mode {:?} with ciphersuite {:?}_{:?}_{:?}",
                test.mode, test.kem_id, test.kdf_id, test.aead_id
            );
            let mode: HpkeMode = test.mode.try_into().unwrap();
            // Algorithm identifiers this build doesn't know are simply skipped.
            let (Ok(kem_id), Ok(kdf_id), Ok(aead_id)): (
                Result<KemAlgorithm, _>,
                Result<KdfAlgorithm, _>,
                Result<AeadAlgorithm, _>,
            ) = (
                test.kem_id.try_into(),
                test.kdf_id.try_into(),
                test.aead_id.try_into(),
            ) else {
                return None;
            };

            if Crypto::supports_kem(kem_id).is_err() {
                log::trace!(
                    " > KEM {:?} not implemented yet for {}",
                    kem_id,
                    Crypto::name()
                );
                return None;
            }

            // All KDFs and AEADs are supported (when the KEM is supported).
            assert!(Crypto::supports_aead(aead_id).is_ok());
            assert!(Crypto::supports_kdf(kdf_id).is_ok());

            log::trace!(
                "Testing mode {:?} with ciphersuite {:?}_{:?}_{:?}",
                mode,
                kem_id,
                kdf_id,
                aead_id
            );

            // Init HPKE with the given mode and ciphersuite.
            let mut hpke = Hpke::<Crypto>::new(mode, kem_id, kdf_id, aead_id);

            // Set up sender and receiver.
            let pk_rm = HpkePublicKey::new(hex_to_bytes(&test.pkRm));
            let sk_rm = HpkePrivateKey::new(hex_to_bytes(&test.skRm));
            // Ephemeral key pair is only present in the classical (RFC 9180) vectors.
            let ephemeral_keys = match (&test.pkEm, &test.skEm) {
                (Some(pk), Some(sk)) => Some((
                    HpkePublicKey::new(hex_to_bytes(pk)),
                    HpkePrivateKey::new(hex_to_bytes(sk)),
                )),
                _ => None,
            };
            let pk_sm = hex_to_bytes_option(test.pkSm);
            let pk_sm = if pk_sm.is_empty() {
                None
            } else {
                Some(HpkePublicKey::new(pk_sm))
            };
            let pk_sm = pk_sm.as_ref();
            let sk_sm = hex_to_bytes_option(test.skSm);
            let sk_sm = if sk_sm.is_empty() {
                None
            } else {
                Some(HpkePrivateKey::new(sk_sm))
            };
            let sk_sm = sk_sm.as_ref();
            let info = hex_to_bytes(&test.info);
            let psk = hex_to_bytes_option(test.psk);
            let psk = vec_to_option_slice(&psk);
            let psk_id = hex_to_bytes_option(test.psk_id);
            let psk_id = vec_to_option_slice(&psk_id);
            let shared_secret = hex_to_bytes(&test.shared_secret);
            let key = hex_to_bytes(&test.key);
            let nonce = hex_to_bytes(&test.base_nonce);
            let exporter_secret = hex_to_bytes(&test.exporter_secret);

            // Input key material.
            let ikm_r = hex_to_bytes(&test.ikmR);
            let ikm_e = hex_to_bytes(&test.ikmE);
            let ikm_s = hex_to_bytes_option(test.ikmS);

            // Use internal `key_schedule` function for KAT.
            let mut direct_ctx = hpke
                .key_schedule(
                    &shared_secret,
                    &info,
                    psk.unwrap_or_default(),
                    psk_id.unwrap_or_default(),
                )
                .unwrap_or_else(|e| {
                    panic!("key_schedule failed for {kem_id:?}_{kdf_id:?}_{aead_id:?}: {e:?}")
                });

            // Check setup info
            // Note that key and nonce are empty for exporter only key derivation.
            assert_eq!(direct_ctx.key(), key);
            assert_eq!(direct_ctx.nonce(), nonce);
            assert_eq!(direct_ctx.exporter_secret(), exporter_secret);
            assert_eq!(direct_ctx.sequence_number(), 0);

            // Test key pair derivation.
            let (my_sk_r, my_pk_r) = hpke.derive_key_pair(&ikm_r).unwrap().into_keys();
            assert_eq!(sk_rm, my_sk_r);
            assert_eq!(pk_rm, my_pk_r);
            if let Some((pk_em, sk_em)) = &ephemeral_keys {
                let (my_sk_e, my_pk_e) = hpke.derive_key_pair(&ikm_e).unwrap().into_keys();
                assert_eq!(sk_em, &my_sk_e);
                assert_eq!(pk_em, &my_pk_e);
            }
            if let (Some(sk_sm), Some(pk_sm)) = (sk_sm, pk_sm) {
                let (my_sk_s, my_pk_s) = hpke.derive_key_pair(&ikm_s).unwrap().into_keys();
                assert_eq!(sk_sm, &my_sk_s);
                assert_eq!(pk_sm, &my_pk_s);
            }

            // Setup KAT receiver.
            let kat_enc = hex_to_bytes(&test.enc);
            let mut receiver_context_kat = hpke
                .setup_receiver(&kat_enc, &sk_rm, &info, psk, psk_id, pk_sm)
                .unwrap();

            // Setup sender and receiver with KAT randomness.
            // We first have to inject the randomness (ikmE).

            // Inject `ikmE` to check the sender-side `enc`. DH-based KEMs derive the
            // ephemeral from `Hpke::random`; the PQ KEMs run derandomized
            // from the injected seed. Either way `enc` must match the vector.
            #[cfg(feature = "hpke-test-prng")]
            {
                log::trace!("Testing with known ikmE ...");
                let mut hpke_sender = Hpke::<Crypto>::new(mode, kem_id, kdf_id, aead_id);
                // This only works when seeding the PRNG with ikmE.
                hpke_sender.seed(&ikm_e).expect("Error injecting ikm_e");
                let (enc, _sender_context_kat) = hpke_sender
                    .setup_sender(&pk_rm, &info, psk, psk_id, sk_sm)
                    .unwrap();
                let receiver_context = hpke
                    .setup_receiver(&enc, &sk_rm, &info, psk, psk_id, pk_sm)
                    .unwrap();
                assert_eq!(enc, kat_enc);
                assert_eq!(receiver_context.key(), receiver_context_kat.key());
                assert_eq!(receiver_context.nonce(), receiver_context_kat.nonce());
                assert_eq!(
                    receiver_context.exporter_secret(),
                    receiver_context_kat.exporter_secret()
                );
                receiver_context_kat = receiver_context;
                assert_eq!(receiver_context_kat.key(), key);
                assert_eq!(receiver_context_kat.nonce(), nonce);
                assert_eq!(receiver_context_kat.exporter_secret(), exporter_secret);
                assert_eq!(receiver_context_kat.sequence_number(), 0);
            }

            // Setup sender and receiver for self tests.
            let (enc, mut sender_context) = hpke
                .setup_sender(&pk_rm, &info, psk, psk_id, sk_sm)
                .unwrap();
            let mut receiver_context = hpke
                .setup_receiver(&enc, &sk_rm, &info, psk, psk_id, pk_sm)
                .unwrap();

            // Encrypt
            log::trace!(
                "Testing encryptions for mode {:?} with ciphersuite {:?}_{:?}_{:?}",
                mode,
                kem_id,
                kdf_id,
                aead_id
            );
            for encryption in test.encryptions.iter() {
                let aad = hex_to_bytes(&encryption.aad);
                let ptxt = hex_to_bytes(&encryption.pt);
                let ctxt_kat = hex_to_bytes(&encryption.ct);

                // Test context API self-test
                let ctxt_out = sender_context.seal(&aad, &ptxt).unwrap();
                let ptxt_out = receiver_context.open(&aad, &ctxt_out).unwrap();
                assert_eq!(ptxt_out, ptxt);

                // Test KAT receiver context open
                let ptxt_out = receiver_context_kat.open(&aad, &ctxt_kat).unwrap();
                assert_eq!(ptxt_out, ptxt);

                // Test KAT seal on direct_ctx
                let ct = direct_ctx.seal(&aad, &ptxt).unwrap();
                assert_eq!(ctxt_kat, ct);
            }

            // Test the single-shot API once per vector. This path runs a full KEM
            // setup_sender/setup_receiver (an encapsulation + decapsulation), so it
            // is by far the most expensive operation here; running it for every one
            // of the (up to 257) encryptions added no coverage over the per-message
            // KAT checks above, which already byte-compare every ciphertext.
            if let Some(encryption) = test.encryptions.first() {
                let aad = hex_to_bytes(&encryption.aad);
                let ptxt = hex_to_bytes(&encryption.pt);
                // Cloning the Hpke object renews the test PRNG.
                let mut hpke = hpke.clone();
                let (enc, ct) = hpke
                    .seal(&pk_rm, &info, &aad, &ptxt, psk, psk_id, sk_sm)
                    .unwrap();
                let ptxt_out = hpke
                    .open(&enc, &sk_rm, &info, &aad, &ct, psk, psk_id, pk_sm)
                    .unwrap();
                assert_eq!(ptxt_out, ptxt);
            }

            // Test KAT on direct_ctx for exporters
            log::trace!(
                "Testing exporter for mode {:?} with ciphersuite {:?}_{:?}_{:?}",
                mode,
                kem_id,
                kdf_id,
                aead_id
            );
            for export in test.exports.iter() {
                let export_context = hex_to_bytes(&export.exporter_context);
                let export_value = hex_to_bytes(&export.exported_value);
                let length = export.L;

                let exported_secret = direct_ctx.export(&export_context, length).unwrap();
                assert_eq!(export_value, exported_secret);
            }

            Some(kem_id)
        })
        .collect()
}

#[test]
fn kats_rust_crypto() {
    // `test_vectors2.json` is the newer vector format and includes p384,
    // `test_vectors_k256.json` holds the secp256k1 suites,
    // which is not standardized only the RustCrypto backend implements.
    let files = &[
        "tests/test_vectors.json",
        "tests/test_vectors2.json",
        "tests/test_vectors_k256.json",
    ];

    // The exact set of KEMs the RustCrypto backend must exercise. These files
    // carry no ML-KEM / X-Wing vectors, so `experimental` adds no KAT coverage
    // here; K256 runs regardless of it.
    let expected_kems = &[
        KemAlgorithm::DhKem25519,
        KemAlgorithm::DhKemP256,
        KemAlgorithm::DhKemP384,
        KemAlgorithm::DhKemP521,
        KemAlgorithm::DhKemK256,
    ];

    run::<HpkeRustCrypto>(files, expected_kems);
}

#[test]
fn kats_libcrux() {
    #[allow(unused_mut)]
    let mut files = vec!["tests/test_vectors.json", "tests/test_vectors2.json"];

    // `test_vectors_hpke_pq.json` is vendored from
    // <https://github.com/hpkewg/hpke-pq/blob/main/test-vectors.json>
    // (draft-ietf-hpke-pq). Only the libcrux provider implements these suites,
    // and only under the `draft-ietf-hpke-pq` feature. Unsupported suites within
    // the file (TurboSHAKE, X448, …) are skipped automatically. libcrux does not
    // implement secp256k1, so it is deliberately not handed `test_vectors_k256`.
    #[cfg(feature = "draft-ietf-hpke-pq")]
    files.push("tests/test_vectors_hpke_pq.json");

    // The exact set of KEMs the libcrux backend must exercise, per feature. Note
    // `draft-connolly-cfrg-hpke-mlkem` adds no entries: no ML-KEM vectors ship
    // outside the `draft-ietf-hpke-pq` file, so that feature has no KAT coverage.
    #[allow(unused_mut)]
    let mut expected_kems = vec![KemAlgorithm::DhKem25519, KemAlgorithm::DhKemP256];
    #[cfg(feature = "libcrux-rustcrypto-p-curves")]
    expected_kems.extend([KemAlgorithm::DhKemP384, KemAlgorithm::DhKemP521]);
    #[cfg(feature = "draft-ietf-hpke-pq")]
    expected_kems.extend([
        KemAlgorithm::XWingDraft06,
        KemAlgorithm::MlKem512,
        KemAlgorithm::MlKem768,
        KemAlgorithm::MlKem1024,
        KemAlgorithm::MlKem768P256,
    ]);
    // The ML-KEM-1024 / P-384 hybrid needs the P-384 curve, gated behind p-curves.
    #[cfg(all(
        feature = "draft-ietf-hpke-pq",
        feature = "libcrux-rustcrypto-p-curves"
    ))]
    expected_kems.push(KemAlgorithm::MlKem1024P384);

    run::<HpkeLibcrux>(&files, &expected_kems);
}

/// Run the KAT for every file and assert that the set of KEMs actually exercised
/// is exactly `expected_kems` — the concrete list of suites this backend is
/// declared to support in this build.
///
/// Because the expectation is a hand-declared list rather than something derived
/// from `Crypto::supports_*`, a regression that silently drops (or unexpectedly
/// gains) support for a KEM trips the assertion instead of passing quietly. Every
/// file handed to a backend must also run at least one vector, so a backend is
/// never given a file whose suites it can't run.
fn run<Crypto: HpkeCrypto + 'static>(files: &[&str], expected_kems: &[KemAlgorithm]) {
    let _ = pretty_env_logger::try_init();

    let mut ran_kems: Vec<KemAlgorithm> = Vec::new();
    for &path in files {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => panic!("Couldn't open file {}.", path),
        };
        let reader = BufReader::new(file);
        let tests: Vec<HpkeTestVector> = match serde_json::from_reader(reader) {
            Ok(r) => r,
            Err(e) => panic!("Error reading file.\n{:?}", e),
        };

        // Run the actual KAT; `kat` returns the KEM of every vector it exercised.
        let now = Instant::now();
        let executed = kat::<Crypto>(tests.clone());
        let time = now.elapsed();

        // Every file must contribute something, so a backend is never handed a
        // file it can't run any of (e.g. libcrux + secp256k1 vectors).
        assert!(
            !executed.is_empty(),
            "No KAT vectors ran for {} in {}",
            Crypto::name(),
            path
        );

        ran_kems.extend(executed);

        log::info!(
            "Test vectors with {} took: {}s",
            Crypto::name(),
            time.as_secs()
        );
    }

    // The set of KEMs that ran must be exactly the declared set. Compare as sets:
    // a KEM appears once per vector above and across multiple files.
    let ran_set = distinct(&ran_kems);
    let missing: Vec<_> = expected_kems
        .iter()
        .filter(|k| !ran_set.contains(k))
        .collect();
    let unexpected: Vec<_> = ran_set
        .iter()
        .filter(|k| !expected_kems.contains(k))
        .collect();
    assert!(
        missing.is_empty() && unexpected.is_empty(),
        "{}: KEM coverage mismatch — expected {:?}, ran {:?} \
         (missing {:?}, unexpected {:?})",
        Crypto::name(),
        expected_kems,
        ran_set,
        missing,
        unexpected
    );
}

/// The distinct values of `kems`, preserving first-seen order. `KemAlgorithm` is
/// only `PartialEq`, so we can't lean on a `HashSet`/`BTreeSet` here.
fn distinct(kems: &[KemAlgorithm]) -> Vec<KemAlgorithm> {
    let mut out: Vec<KemAlgorithm> = Vec::new();
    for &kem in kems {
        if !out.contains(&kem) {
            out.push(kem);
        }
    }
    out
}

#[cfg(feature = "serialization")]
#[cfg(feature = "hazmat")]
#[test]
fn test_serialization() {
    use hpke::HpkeKeyPair;

    // XXX: Make these individual tests.
    for mode in 0u8..4 {
        let hpke_mode = HpkeMode::try_from(mode).unwrap();
        for aead_mode in 1u16..4 {
            let aead_mode = AeadAlgorithm::try_from(aead_mode).unwrap();
            for kdf_mode in 1u16..4 {
                let kdf_mode = KdfAlgorithm::try_from(kdf_mode).unwrap();
                for &kem_mode in &[0x10u16, 0x20] {
                    let kem_mode = KemAlgorithm::try_from(kem_mode).unwrap();

                    let mut hpke =
                        Hpke::<HpkeRustCrypto>::new(hpke_mode, kem_mode, kdf_mode, aead_mode);

                    // JSON: Public, Private, KeyPair
                    let key_pair = hpke.generate_key_pair().unwrap();

                    let serialized_key_pair = serde_json::to_string(&key_pair).unwrap();
                    let deserialized_key_pair: HpkeKeyPair =
                        serde_json::from_str(&serialized_key_pair).unwrap();

                    let (sk, pk) = key_pair.into_keys();

                    let serialized_sk = serde_json::to_string(&sk).unwrap();
                    let deserialized_sk: HpkePrivateKey =
                        serde_json::from_str(&serialized_sk).unwrap();
                    let serialized_pk = serde_json::to_string(&pk).unwrap();
                    let deserialized_pk: HpkePublicKey =
                        serde_json::from_str(&serialized_pk).unwrap();

                    let (des_sk, des_pk) = deserialized_key_pair.into_keys();

                    assert_eq!(pk, des_pk);
                    assert_eq!(pk, deserialized_pk);
                    assert_eq!(sk.as_slice(), des_sk.as_slice());
                    assert_eq!(sk.as_slice(), deserialized_sk.as_slice());
                }
            }
        }
    }
}

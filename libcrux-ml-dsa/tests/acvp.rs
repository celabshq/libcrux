#![cfg(feature = "acvp")]

use libcrux_kats::acvp::mldsa::keygen_schema::*;
use libcrux_kats::acvp::mldsa::sign_schema::*;
use libcrux_kats::acvp::mldsa::verify_schema::*;
use libcrux_kats::acvp::schema_common::*;

#[test]
fn keygen() {
    use libcrux_kats::acvp::mldsa::KeyGenTests;

    let KeyGenTests { prompts, results } = KeyGenTests::load();
    assert_eq!(prompts.algorithm, "ML-DSA");
    assert_eq!(prompts.revision, "FIPS204");
    assert_covers(&prompts, &results);

    let mut tally = Tally::new();
    for kat in prompts.testGroups {
        let parameter_set = kat.parameterSet;
        eprintln!("{parameter_set}");

        for test in kat.tests {
            keygen_inner(test, &results, kat.tgId, &parameter_set);
            tally.add(&parameter_set);
        }
    }
    tally.assert_counts([("ML-DSA-44", 25), ("ML-DSA-65", 25), ("ML-DSA-87", 25)]);
}

#[inline(never)]
#[allow(non_snake_case)]
fn keygen_inner(
    test: KeyGenPrompt,
    results: &Results<ResultKeyGenTestGroup>,
    tgId: usize,
    parameter_set: &String,
) {
    use libcrux_ml_dsa::*;
    eprintln!("  {}", test.tcId);
    #[inline(never)]
    fn check<const VK_LEN: usize, const SK_LEN: usize>(
        keys: MLDSAKeyPair<VK_LEN, SK_LEN>,
        result: &KeyGenResult,
    ) {
        assert_eq!(result.pk, keys.verification_key.as_slice());
        assert_eq!(result.sk, keys.signing_key.as_slice());
    }

    let expected_result = results.find_expected_result(tgId, test.tcId);

    match parameter_set.as_str() {
        "ML-DSA-44" => check(ml_dsa_44::generate_key_pair(test.seed), expected_result),

        "ML-DSA-65" => check(ml_dsa_65::generate_key_pair(test.seed), expected_result),

        "ML-DSA-87" => check(ml_dsa_87::generate_key_pair(test.seed), expected_result),
        p => panic!("unexpected parameterSet {p}"),
    }
}

#[test]
fn siggen() {
    use libcrux_kats::acvp::mldsa::SigGenTests;

    let SigGenTests { prompts, results } = SigGenTests::load();
    assert_eq!(prompts.algorithm, "ML-DSA");
    assert_eq!(prompts.revision, "FIPS204");
    assert_covers(&prompts, &results);

    run_siggen(prompts, &results).assert_counts([
        ("internal", 90),
        ("external pure", 90),
        ("external preHash SHAKE-128", 9),
        ("skipped: externalMu", 90),
        ("skipped: preHash other than SHAKE-128", 81),
    ]);
}

/// Run the sigGen tests, returning the test cases run and skipped per category. Categories of
/// groups with a `keyFormat` are prefixed with it.
fn run_siggen(
    prompts: Prompts<SigGenPromptTestGroup>,
    results: &Results<ResultSigGenTestGroup>,
) -> Tally {
    let mut tally = Tally::new();
    for kat in prompts.testGroups {
        let parameter_set = kat.parameterSet;
        let key_format = kat
            .keyFormat
            .as_deref()
            .map_or(String::new(), |f| format!("{f} "));
        eprintln!("{parameter_set}");

        for mut test in kat.tests {
            let category = if kat.signatureInterface == "internal" {
                if kat.externalMu {
                    // NOTE: this API is currently not supported
                    "skipped: externalMu"
                } else {
                    "internal"
                }
            } else if kat.preHash.as_deref() == Some("preHash") {
                if test.hashAlg.as_deref() == Some("SHAKE-128") {
                    "external preHash SHAKE-128"
                } else {
                    "skipped: preHash other than SHAKE-128"
                }
            } else {
                assert_eq!(kat.preHash.as_deref(), Some("pure"));
                "external pure"
            };
            tally.add(format!("{key_format}{category}"));
            if category.starts_with("skipped") {
                continue;
            }

            match kat.keyFormat.as_deref() {
                Some("seed") => test.sk = signing_key_from_seed(&parameter_set, &test.seed),
                Some("expanded") | None => assert!(!test.sk.is_empty()),
                f => panic!("unexpected keyFormat {f:?}"),
            }
            match category {
                "internal" => siggen_inner_internal(test, results, kat.tgId, &parameter_set),
                "external preHash SHAKE-128" => {
                    siggen_inner_external_prehash(test, results, kat.tgId, &parameter_set)
                }
                _ => siggen_inner_external(test, results, kat.tgId, &parameter_set),
            }
        }
    }
    tally
}

#[inline(never)]
#[allow(non_snake_case)]
fn siggen_inner_internal(
    test: SigGenTest,
    results: &Results<ResultSigGenTestGroup>,
    tgId: usize,
    parameter_set: &String,
) {
    use libcrux_ml_dsa::*;
    eprintln!("  {}", test.tcId);

    let Randomness(rnd) = test.rnd.unwrap_or(Randomness([0u8; 32]));

    let expected_result = results.find_expected_result(tgId, test.tcId);

    match parameter_set.as_str() {
        "ML-DSA-44" => {
            let signature = ml_dsa_44::sign_internal(
                &MLDSASigningKey::new(test.sk.try_into().unwrap()),
                &test.message,
                rnd,
            )
            .unwrap();
            assert_eq!(signature.as_slice(), expected_result.signature);
        }

        "ML-DSA-65" => {
            let signature = ml_dsa_65::sign_internal(
                &MLDSASigningKey::new(test.sk.try_into().unwrap()),
                &test.message,
                rnd,
            )
            .unwrap();
            assert_eq!(signature.as_slice(), expected_result.signature);
        }

        "ML-DSA-87" => {
            let signature = ml_dsa_87::sign_internal(
                &MLDSASigningKey::new(test.sk.try_into().unwrap()),
                &test.message,
                rnd,
            )
            .unwrap();
            assert_eq!(signature.as_slice(), expected_result.signature);
        }
        p => panic!("unexpected parameterSet {p}"),
    }
}

macro_rules! siggen_test {
    ($name:ident, $sign:ident) => {
        #[inline(never)]
        #[allow(non_snake_case)]
        fn $name(
            test: SigGenTest,
            results: &Results<ResultSigGenTestGroup>,
            tgId: usize,
            parameter_set: &String,
        ) {
            use libcrux_ml_dsa::*;

            eprintln!("  {}", test.tcId);

            let Randomness(rnd) = test.rnd.unwrap_or(Randomness([0u8; 32]));

            let expected_result = results.find_expected_result(tgId, test.tcId);

            match parameter_set.as_str() {
                "ML-DSA-44" => {
                    let signature = ml_dsa_44::$sign(
                        &MLDSASigningKey::new(test.sk.try_into().unwrap()),
                        &test.message,
                        &test.context,
                        rnd,
                    )
                    .unwrap();
                    assert_eq!(signature.as_slice(), expected_result.signature);
                }

                "ML-DSA-65" => {
                    let signature = ml_dsa_65::$sign(
                        &MLDSASigningKey::new(test.sk.try_into().unwrap()),
                        &test.message,
                        &test.context,
                        rnd,
                    )
                    .unwrap();
                    assert_eq!(signature.as_slice(), expected_result.signature);
                }

                "ML-DSA-87" => {
                    let signature = ml_dsa_87::$sign(
                        &MLDSASigningKey::new(test.sk.try_into().unwrap()),
                        &test.message,
                        &test.context,
                        rnd,
                    )
                    .unwrap();
                    assert_eq!(signature.as_slice(), expected_result.signature);
                }
                p => panic!("unexpected parameterSet {p}"),
            }
        }
    };
}
siggen_test!(siggen_inner_external, sign);
siggen_test!(siggen_inner_external_prehash, sign_pre_hashed_shake128);

#[test]
fn sigver() {
    use libcrux_kats::acvp::mldsa::SigVerTests;

    let SigVerTests { prompts, results } = SigVerTests::load();
    assert_eq!(prompts.algorithm, "ML-DSA");
    assert_eq!(prompts.revision, "FIPS204");
    assert_covers(&prompts, &results);

    let mut tally = Tally::new();
    for kat in prompts.testGroups {
        let parameter_set = kat.parameterSet;
        eprintln!("{parameter_set}");

        for test in kat.tests {
            if kat.signatureInterface == "internal" {
                if kat.externalMu {
                    // NOTE: this API is currently not supported
                    tally.add("skipped: externalMu");
                    continue;
                }
                sigver_inner_internal(test, &results, kat.tgId, &parameter_set);
                tally.add("internal");
            } else {
                let pre_hash = kat.preHash.as_deref() == Some("preHash");
                if !pre_hash {
                    assert_eq!(kat.preHash.as_deref(), Some("pure"));
                } else if test.hashAlg.as_deref() != Some("SHAKE-128") {
                    tally.add("skipped: preHash other than SHAKE-128");
                    continue;
                }
                sigver_inner_external(test, &results, kat.tgId, &parameter_set, pre_hash);
                tally.add(if pre_hash {
                    "external preHash SHAKE-128"
                } else {
                    "external pure"
                });
            }
        }
    }
    tally.assert_counts([
        ("internal", 45),
        ("external pure", 45),
        ("external preHash SHAKE-128", 3),
        ("skipped: externalMu", 45),
        ("skipped: preHash other than SHAKE-128", 42),
    ]);
}

#[inline(never)]
#[allow(non_snake_case)]
fn sigver_inner_external(
    test: SigVerTest,
    results: &Results<ResultSigVerTestGroup>,
    tgId: usize,
    parameter_set: &String,
    pre_hash: bool,
) {
    use libcrux_ml_dsa::*;
    eprintln!("  {}", test.tcId);
    let expected_result = results.find_expected_result(tgId, test.tcId);

    let pk = test.pk.to_owned();

    match parameter_set.as_str() {
        "ML-DSA-44" => {
            let verify = match pre_hash {
                false => ml_dsa_44::verify,
                true => ml_dsa_44::verify_pre_hashed_shake128,
            };
            let valid = verify(
                &MLDSAVerificationKey::new(pk.try_into().unwrap()),
                &test.message,
                &test.context,
                &MLDSASignature::new(test.signature.try_into().unwrap()),
            );
            assert_eq!(valid.is_ok(), expected_result.testPassed);
        }

        "ML-DSA-65" => {
            let verify = match pre_hash {
                false => ml_dsa_65::verify,
                true => ml_dsa_65::verify_pre_hashed_shake128,
            };
            let valid = verify(
                &MLDSAVerificationKey::new(pk.try_into().unwrap()),
                &test.message,
                &test.context,
                &MLDSASignature::new(test.signature.try_into().unwrap()),
            );
            assert_eq!(valid.is_ok(), expected_result.testPassed);
        }

        "ML-DSA-87" => {
            let verify = match pre_hash {
                false => ml_dsa_87::verify,
                true => ml_dsa_87::verify_pre_hashed_shake128,
            };
            let valid = verify(
                &MLDSAVerificationKey::new(pk.try_into().unwrap()),
                &test.message,
                &test.context,
                &MLDSASignature::new(test.signature.try_into().unwrap()),
            );
            assert_eq!(valid.is_ok(), expected_result.testPassed);
        }
        p => panic!("unexpected parameterSet {p}"),
    }
}

#[inline(never)]
#[allow(non_snake_case)]
fn sigver_inner_internal(
    test: SigVerTest,
    results: &Results<ResultSigVerTestGroup>,
    tgId: usize,
    parameter_set: &String,
) {
    use libcrux_ml_dsa::*;
    eprintln!("  {}", test.tcId);
    let expected_result = results.find_expected_result(tgId, test.tcId);

    let pk = test.pk.to_owned();

    match parameter_set.as_str() {
        "ML-DSA-44" => {
            let valid = ml_dsa_44::verify_internal(
                &MLDSAVerificationKey::new(pk.try_into().unwrap()),
                &test.message,
                &MLDSASignature::new(test.signature.try_into().unwrap()),
            );
            assert_eq!(valid.is_ok(), expected_result.testPassed);
        }

        "ML-DSA-65" => {
            let valid = ml_dsa_65::verify_internal(
                &MLDSAVerificationKey::new(pk.try_into().unwrap()),
                &test.message,
                &MLDSASignature::new(test.signature.try_into().unwrap()),
            );
            assert_eq!(valid.is_ok(), expected_result.testPassed);
        }

        "ML-DSA-87" => {
            let valid = ml_dsa_87::verify_internal(
                &MLDSAVerificationKey::new(pk.try_into().unwrap()),
                &test.message,
                &MLDSASignature::new(test.signature.try_into().unwrap()),
            );
            assert_eq!(valid.is_ok(), expected_result.testPassed);
        }
        p => panic!("unexpected parameterSet {p}"),
    }
}

/// The expanded signing key for the key generation seed.
fn signing_key_from_seed(parameter_set: &str, seed: &[u8]) -> Vec<u8> {
    use libcrux_ml_dsa::*;

    let seed: [u8; 32] = seed.try_into().unwrap();
    match parameter_set {
        "ML-DSA-44" => ml_dsa_44::generate_key_pair(seed)
            .signing_key
            .as_slice()
            .to_vec(),
        "ML-DSA-65" => ml_dsa_65::generate_key_pair(seed)
            .signing_key
            .as_slice()
            .to_vec(),
        "ML-DSA-87" => ml_dsa_87::generate_key_pair(seed)
            .signing_key
            .as_slice()
            .to_vec(),
        p => panic!("unexpected parameterSet {p}"),
    }
}

/// `ML-DSA-sigGen-FIPS204-tr1`, which also gives signing keys as the key generation seed.
#[test]
fn siggen_tr1() {
    use libcrux_kats::acvp::mldsa::SigGenTests;

    let SigGenTests { prompts, results } = SigGenTests::load_tr1();
    assert_eq!(prompts.algorithm, "ML-DSA");
    assert_eq!(prompts.revision, "FIPS204-tr1");
    assert_covers(&prompts, &results);

    run_siggen(prompts, &results).assert_counts([
        ("expanded internal", 90),
        ("seed internal", 90),
        ("expanded external pure", 90),
        ("seed external pure", 90),
        ("expanded external preHash SHAKE-128", 8),
        ("seed external preHash SHAKE-128", 6),
        ("expanded skipped: externalMu", 90),
        ("seed skipped: externalMu", 90),
        ("expanded skipped: preHash other than SHAKE-128", 82),
        ("seed skipped: preHash other than SHAKE-128", 84),
    ]);
}

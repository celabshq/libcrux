#![cfg(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024",))]

use libcrux_kats::acvp::{
    mlkem::{encap_decap_schema::*, keygen_schema::*, *},
    schema_common::{assert_covers, Tally, TestIds},
};

const PARAMETER_SETS: [&str; 3] = ["ML-KEM-512", "ML-KEM-768", "ML-KEM-1024"];

/// Whether the parameter set is enabled by its feature.
fn enabled(parameter_set: &str) -> bool {
    match parameter_set {
        "ML-KEM-512" => cfg!(feature = "mlkem512"),
        "ML-KEM-768" => cfg!(feature = "mlkem768"),
        "ML-KEM-1024" => cfg!(feature = "mlkem1024"),
        p => panic!("unexpected parameterSet {p}"),
    }
}

/// The expected test cases per category `"{parameter set} {category}"` for each enabled
/// parameter set, and all of them as `"{parameter set} disabled"` for disabled ones.
fn expected_counts(per_parameter_set: &[(&str, usize)]) -> Vec<(String, usize)> {
    PARAMETER_SETS
        .iter()
        .flat_map(|p| {
            if enabled(p) {
                per_parameter_set
                    .iter()
                    .map(|(category, n)| (format!("{p} {category}"), *n))
                    .collect()
            } else {
                let n = per_parameter_set.iter().map(|(_, n)| n).sum();
                vec![(format!("{p} disabled"), n)]
            }
        })
        .collect()
}

#[test]
fn keygen() {
    use libcrux_ml_kem::*;

    let KeyGenTests { prompts, results } = KeyGenTests::load();
    assert_eq!(prompts.algorithm, "ML-KEM");
    assert_eq!(prompts.revision, "FIPS203");
    assert_covers(&prompts, &results);

    let mut tally = Tally::new();
    for kat in prompts.testGroups {
        let parameter_set = kat.parameterSet;
        eprintln!("{parameter_set}");
        if !enabled(&parameter_set) {
            tally.add_n(format!("{parameter_set} disabled"), kat.tests.len());
            continue;
        }

        for test in kat.tests {
            eprintln!("  {}", test.tcId);
            let mut seed = [0u8; 64];
            seed[..32].copy_from_slice(&test.d);
            seed[32..].copy_from_slice(&test.z);

            fn check<const EK_LEN: usize, const DK_LEN: usize>(
                keys: MlKemKeyPair<DK_LEN, EK_LEN>,
                result: &KeyGenResult,
            ) {
                assert_eq!(result.ek, keys.pk());
                assert_eq!(result.dk, keys.sk());
            }

            let expected_result = results.find_expected_result(kat.tgId, test.tcId);

            match parameter_set.as_str() {
                #[cfg(feature = "mlkem512")]
                "ML-KEM-512" => check(mlkem512::generate_key_pair(seed), expected_result),
                #[cfg(feature = "mlkem768")]
                "ML-KEM-768" => check(mlkem768::generate_key_pair(seed), expected_result),
                #[cfg(feature = "mlkem1024")]
                "ML-KEM-1024" => check(mlkem1024::generate_key_pair(seed), expected_result),
                p => panic!("unexpected parameterSet {p}"),
            }
            tally.add(format!("{parameter_set} keyGen"));
        }
    }
    tally.assert_counts(expected_counts(&[("keyGen", 25)]));
}

#[test]
fn encap_decap() {
    use libcrux_ml_kem::*;

    let EncapDecapTests { prompts, results } = EncapDecapTests::load();
    assert_eq!(prompts.algorithm, "ML-KEM");
    assert_eq!(prompts.revision, "FIPS203-tr1");
    assert_covers(&prompts, &results);

    let mut tally = Tally::new();
    for kat in prompts.testGroups {
        let parameter_set = kat.parameterSet.clone();
        eprintln!("{parameter_set}");
        if !enabled(&parameter_set) {
            tally.add_n(format!("{parameter_set} disabled"), kat.test_ids().len());
            continue;
        }
        let function = kat.tests.function();

        match kat.tests {
            EncapDecapTestPrompts::EncapTests { tests } => {
                for test in tests {
                    let expected_result = results.find_expected_result(kat.tgId, test.tcId);

                    let EncapDecapResult::EncapResult { c, k, .. } = expected_result else {
                        unreachable!()
                    };

                    let ek = test.ek;
                    let randomness = test.m;
                    match parameter_set.as_str() {
                        #[cfg(feature = "mlkem512")]
                        "ML-KEM-512" => {
                            let (actual_ct, actual_k) = mlkem512::encapsulate(
                                &mlkem512::MlKem512PublicKey::try_from(ek.as_slice()).unwrap(),
                                randomness,
                            );
                            assert_eq!(actual_ct.as_ref(), c);
                            assert_eq!(actual_k.as_ref(), k);
                        }
                        #[cfg(feature = "mlkem768")]
                        "ML-KEM-768" => {
                            let (actual_ct, actual_k) = mlkem768::encapsulate(
                                &mlkem768::MlKem768PublicKey::try_from(ek.as_slice()).unwrap(),
                                randomness,
                            );
                            assert_eq!(actual_ct.as_ref(), c);
                            assert_eq!(actual_k.as_ref(), k);
                        }
                        #[cfg(feature = "mlkem1024")]
                        "ML-KEM-1024" => {
                            let (actual_ct, actual_k) = mlkem1024::encapsulate(
                                &mlkem1024::MlKem1024PublicKey::try_from(ek.as_slice()).unwrap(),
                                randomness,
                            );
                            assert_eq!(actual_ct.as_ref(), c);
                            assert_eq!(actual_k.as_ref(), k);
                        }
                        p => panic!("unexpected parameterSet {p}"),
                    }
                    tally.add(format!("{parameter_set} {function}"));
                }
            }
            EncapDecapTestPrompts::DecapTests { tests } => {
                for test in tests {
                    let expected_result = results.find_expected_result(kat.tgId, test.tcId);

                    let EncapDecapResult::DecapResult { k, .. } = expected_result else {
                        unreachable!()
                    };

                    assert_eq!(
                        kat.keyFormat.as_deref(),
                        Some(test.key.key_format()),
                        "tgId: {}, tcId: {}",
                        kat.tgId,
                        test.tcId
                    );
                    let c = test.c;
                    let seed = |d: &[u8; 32], z: &[u8; 32]| {
                        let mut seed = [0u8; 64];
                        seed[..32].copy_from_slice(d);
                        seed[32..].copy_from_slice(z);
                        seed
                    };
                    match parameter_set.as_str() {
                        #[cfg(feature = "mlkem512")]
                        "ML-KEM-512" => {
                            let dk = match &test.key {
                                DecapKey::Expanded { dk } => {
                                    mlkem512::MlKem512PrivateKey::try_from(dk.as_slice()).unwrap()
                                }
                                DecapKey::Seed { d, z } => mlkem512::generate_key_pair(seed(d, z))
                                    .private_key()
                                    .clone(),
                            };
                            let c = mlkem512::MlKem512Ciphertext::try_from(c.as_slice()).unwrap();
                            let actual_k = mlkem512::decapsulate(&dk, &c);
                            assert_eq!(actual_k.as_ref(), k);
                        }
                        #[cfg(feature = "mlkem768")]
                        "ML-KEM-768" => {
                            let dk = match &test.key {
                                DecapKey::Expanded { dk } => {
                                    mlkem768::MlKem768PrivateKey::try_from(dk.as_slice()).unwrap()
                                }
                                DecapKey::Seed { d, z } => mlkem768::generate_key_pair(seed(d, z))
                                    .private_key()
                                    .clone(),
                            };
                            let c = mlkem768::MlKem768Ciphertext::try_from(c.as_slice()).unwrap();
                            let actual_k = mlkem768::decapsulate(&dk, &c);
                            assert_eq!(actual_k.as_ref(), k);
                        }
                        #[cfg(feature = "mlkem1024")]
                        "ML-KEM-1024" => {
                            let dk = match &test.key {
                                DecapKey::Expanded { dk } => {
                                    mlkem1024::MlKem1024PrivateKey::try_from(dk.as_slice()).unwrap()
                                }
                                DecapKey::Seed { d, z } => mlkem1024::generate_key_pair(seed(d, z))
                                    .private_key()
                                    .clone(),
                            };
                            let c = mlkem1024::MlKem1024Ciphertext::try_from(c.as_slice()).unwrap();
                            let actual_k = mlkem1024::decapsulate(&dk, &c);
                            assert_eq!(actual_k.as_ref(), k);
                        }
                        p => panic!("unexpected parameterSet {p}"),
                    }
                    tally.add(format!(
                        "{parameter_set} {function} {}",
                        test.key.key_format()
                    ));
                }
            }
            EncapDecapTestPrompts::EncapKeyCheck { tests } => {
                for test in tests {
                    assert_eq!(kat.keyFormat, None);
                    let succeeded = match parameter_set.as_str() {
                        #[cfg(feature = "mlkem512")]
                        "ML-KEM-512" => mlkem512::MlKem512PublicKey::try_from(test.ek.as_slice())
                            .map(|ek| mlkem512::validate_public_key(&ek))
                            .unwrap_or(false),
                        #[cfg(feature = "mlkem768")]
                        "ML-KEM-768" => mlkem768::MlKem768PublicKey::try_from(test.ek.as_slice())
                            .map(|ek| mlkem768::validate_public_key(&ek))
                            .unwrap_or(false),
                        #[cfg(feature = "mlkem1024")]
                        "ML-KEM-1024" => {
                            mlkem1024::MlKem1024PublicKey::try_from(test.ek.as_slice())
                                .map(|ek| mlkem1024::validate_public_key(&ek))
                                .unwrap_or(false)
                        }
                        p => panic!("unexpected parameterSet {p}"),
                    };

                    // get result
                    let EncapDecapResult::KeyCheckResult { testPassed, .. } =
                        results.find_expected_result(kat.tgId, test.tcId)
                    else {
                        unreachable!();
                    };

                    assert_eq!(
                        succeeded, *testPassed,
                        "tgId: {}, tcId: {}",
                        kat.tgId, test.tcId
                    );
                    tally.add(format!(
                        "{parameter_set} {function} {}",
                        if succeeded { "valid" } else { "invalid" }
                    ));
                }
            }
            EncapDecapTestPrompts::DecapKeyCheck { tests } => {
                for test in tests {
                    assert_eq!(kat.keyFormat.as_deref(), Some("expanded"));
                    let succeeded = match parameter_set.as_str() {
                        #[cfg(feature = "mlkem512")]
                        "ML-KEM-512" => mlkem512::MlKem512PrivateKey::try_from(test.dk.as_slice())
                            .map(|dk| mlkem512::portable::validate_private_key_only(&dk))
                            .unwrap_or(false),
                        #[cfg(feature = "mlkem768")]
                        "ML-KEM-768" => mlkem768::MlKem768PrivateKey::try_from(test.dk.as_slice())
                            .map(|dk| mlkem768::portable::validate_private_key_only(&dk))
                            .unwrap_or(false),
                        #[cfg(feature = "mlkem1024")]
                        "ML-KEM-1024" => {
                            mlkem1024::MlKem1024PrivateKey::try_from(test.dk.as_slice())
                                .map(|dk| mlkem1024::portable::validate_private_key_only(&dk))
                                .unwrap_or(false)
                        }
                        p => panic!("unexpected parameterSet {p}"),
                    };

                    // get result
                    let EncapDecapResult::KeyCheckResult { testPassed, .. } =
                        results.find_expected_result(kat.tgId, test.tcId)
                    else {
                        unreachable!();
                    };

                    assert_eq!(
                        succeeded, *testPassed,
                        "tgId: {}, tcId: {}",
                        kat.tgId, test.tcId
                    );
                    tally.add(format!(
                        "{parameter_set} {function} {}",
                        if succeeded { "valid" } else { "invalid" }
                    ));
                }
            }
        }
    }
    tally.assert_counts(expected_counts(&[
        ("encapsulation", 25),
        ("decapsulation seed", 10),
        ("decapsulation expanded", 10),
        ("encapsulationKeyCheck valid", 5),
        ("encapsulationKeyCheck invalid", 5),
        ("decapsulationKeyCheck valid", 5),
        ("decapsulationKeyCheck invalid", 5),
    ]));
}

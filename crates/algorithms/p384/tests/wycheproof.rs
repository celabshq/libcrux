use libcrux_kats::wycheproof::{ecdh, TestResult};

fn pad_slice_to_arr(b: &[u8]) -> [u8; 48] {
    let mut out = [0u8; 48];
    out[48 - b.len()..].copy_from_slice(b);
    out
}

#[test]
fn ecdh_secp384r1() {
    let test_set = ecdh::TestSet::load_secp384r1_ecpoint();
    let mut tests_run = 0;

    for test_group in test_set.test_groups {
        for test in &test_group.tests {
            // The ecpoint format gives us sec 1 encoded public keys as hex strings and private keys
            // encoded as hex formatted big integers

            // strip leading 0 bytes
            let sk = test
                .private_key
                .iter()
                .position(|b| *b != 0)
                .map_or(test.private_key.as_slice(), |pos| &test.private_key[pos..]);

            assert!(
                sk.len() <= 48,
                "0 prefix stripped sk is larger than 48 bytes, tc_id: {}",
                test.tc_id
            );

            // sk is a 48-byte big endian big integer, so we pad the lower bytes with 0
            let sk_bytes = pad_slice_to_arr(sk);

            if test.public_key.len() == 97 {
                let decode_result = libcrux_p384::PublicKey::try_from(test.public_key.as_slice());

                match decode_result {
                    Ok(pk) => {
                        let mut re_encoded = [0u8; 97];
                        pk.to_uncompressed(&mut re_encoded);
                        assert_eq!(
                            re_encoded.as_slice(),
                            test.public_key.as_slice(),
                            "tc_id: {}, re-encoding an uncompressed point did not reproduce the input",
                            test.tc_id
                        );
                    }
                    Err(_) => {
                        assert_eq!(
                            TestResult::Invalid,
                            test.result,
                            "tc_id: {}, test has invalid uncompressed point but test result is {:?}",
                            test.tc_id,
                            test.result
                        );
                        tests_run += 1;
                        continue;
                    }
                }
            } else if test.public_key.len() == 49 {
                let decode_result = libcrux_p384::PublicKey::try_from(test.public_key.as_slice());

                match decode_result {
                    Ok(pk) => {
                        let mut re_encoded = [0u8; 49];
                        pk.to_compressed(&mut re_encoded);
                        assert_eq!(
                            re_encoded.as_slice(),
                            test.public_key.as_slice(),
                            "tc_id: {}, re-encoding a compressed point did not reproduce the input",
                            test.tc_id
                        );
                    }
                    Err(_) => {
                        assert_eq!(
                            TestResult::Invalid,
                            test.result,
                            "tc_id: {}, test has invalid compressed point but test result is {:?}",
                            test.tc_id,
                            test.result
                        );
                        tests_run += 1;
                        continue;
                    }
                }
            } else {
                assert_eq!(
                    TestResult::Invalid,
                    test.result,
                    "tc_id: {}, public key has invalid size {}, but test result is {:?}",
                    test.tc_id,
                    test.public_key.len(),
                    test.result
                );
                assert!(
                    test.flags.contains(&"InvalidEncoding".to_string()),
                    "tc_id: {}, public key is invalid but test does not contain InvalidEncoding flag ",
                    test.tc_id
                );
                tests_run += 1;
                continue;
            }

            let result = libcrux_p384::derive_ecdh(&sk_bytes, &test.public_key);
            match test.result {
                // XXX: In the future, wycheproof might add acceptable test cases which we (want to) reject.
                // This needs to be split then.
                TestResult::Valid | TestResult::Acceptable => {
                    assert!(
                        result.is_ok(),
                        "tc_id {}: expected success or acceptable but ECDH failed",
                        test.tc_id,
                    );
                    let result = result.unwrap();
                    assert_eq!(
                        test.shared_secret,
                        result.as_ref(),
                        "tc_id {}: shared secret mismatch",
                        test.tc_id,
                    );
                }
                TestResult::Invalid => {
                    assert!(
                        result.is_err(),
                        "tc_id: {}, expected invalid test but ECDH derive succeeded",
                        test.tc_id
                    );
                }
            }
            tests_run += 1;
        }
    }

    assert_eq!(
        test_set.number_of_tests, tests_run,
        "invalid number of tests run"
    );
}

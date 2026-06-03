use libcrux_kats::wycheproof::mlkem::schema::*;
use libcrux_kats::wycheproof::mlkem::TestGroupType;

use libcrux_ml_kem::{MlKemCiphertext, MlKemPrivateKey, MlKemPublicKey};

macro_rules! wycheproof_test {
    ($name:ident, $parameter_set:expr, $module:path) => {
        mod $name {
            use super::*;
            use libcrux_kats::wycheproof::TestResult;

            #[test]
            fn keygen_and_decaps() {
                use $module::*;

                let katfile = MlKemTests::load($parameter_set, TestGroupType::MlKemTest);

                for test_group in katfile.keygen_and_decaps_tests() {
                    for test in &test_group.tests {
                        let Ok(seed) = test.seed.clone().try_into() else {
                            assert_eq!(test.result, TestResult::Invalid);
                            continue;
                        };

                        // generate key pair
                        let key_pair = generate_key_pair(seed);

                        // convert ciphertext
                        let Ok(ciphertext) = <[u8; _]>::try_from(test.ciphertext.clone()) else {
                            assert_eq!(test.result, TestResult::Invalid);
                            continue;
                        };
                        let ciphertext: MlKemCiphertext<_> = ciphertext.into();

                        // validate keys
                        assert!(validate_private_key(key_pair.private_key(), &ciphertext));
                        assert!(validate_public_key(key_pair.public_key()));

                        // compare encapsulation key
                        assert_eq!(key_pair.public_key().as_ref(), test.encapsulation_key);

                        // decapsulate
                        let shared_secret_from_decapsulate =
                            decapsulate(key_pair.private_key(), &ciphertext);

                        // compare shared secret
                        assert_eq!(shared_secret_from_decapsulate, test.shared_secret.as_ref());

                        // assert result is valid
                        assert_eq!(test.result, TestResult::Valid);
                    }
                }
            }

            #[test]
            fn encaps() {
                use $module::*;
                let katfile = MlKemTests::load($parameter_set, TestGroupType::MlKemEncapsTest);

                for test_group in katfile.encaps_tests() {
                    for test in &test_group.tests {
                        // convert to encapsulation key
                        let Ok(bytes) = <[u8; _]>::try_from(test.encapsulation_key.clone()) else {
                            assert_eq!(test.result, TestResult::Invalid);
                            continue;
                        };
                        let encapsulation_key: MlKemPublicKey<_> = bytes.into();

                        let is_valid = validate_public_key(&encapsulation_key);
                        match test.result {
                            TestResult::Invalid => assert!(
                                !is_valid,
                                "tc_id {} failed. invalid public key passes key validation",
                                test.tc_id,
                            ),
                            TestResult::Valid => assert!(
                                is_valid,
                                "tc_id {} failed. valid public key rejected by key validation",
                                test.tc_id,
                            ),
                            TestResult::Acceptable => {
                                unreachable!("TestResult::Acceptable not part of test vectors")
                            }
                        }
                    }
                }
            }

            #[test]
            fn semi_expanded_decaps() {
                use $module::*;

                let katfile =
                    MlKemTests::load($parameter_set, TestGroupType::MlKemDecapsValidationTest);

                for test_group in katfile.semi_expanded_decaps_tests() {
                    for test in &test_group.tests {
                        // convert private key
                        let Ok(bytes) = <[u8; _]>::try_from(test.decapsulation_key.clone()) else {
                            assert_eq!(test.result, TestResult::Invalid);
                            continue;
                        };
                        let private_key: MlKemPrivateKey<_> = bytes.into();

                        // convert ciphertext
                        let Ok(ciphertext) = <[u8; _]>::try_from(test.ciphertext.clone()) else {
                            assert_eq!(test.result, TestResult::Invalid);
                            continue;
                        };
                        let ciphertext: MlKemCiphertext<_> = ciphertext.into();

                        // validate keys
                        let is_valid = validate_private_key(&private_key, &ciphertext);
                        match test.result {
                            TestResult::Invalid => assert!(
                                !is_valid,
                                "tc_id {} failed. invalid private key passes key validation",
                                test.tc_id,
                            ),
                            TestResult::Valid => assert!(
                                is_valid,
                                "tc_id {} failed. valid private key rejected by key validation",
                                test.tc_id,
                            ),
                            TestResult::Acceptable => {
                                unreachable!("TestResult::Acceptable not part of test vectors")
                            }
                        }
                    }
                }
            }
        }
    };
}

// multiplexing API
#[cfg(feature = "mlkem512")]
wycheproof_test!(ml_kem_512, ParameterSet::MlKem512, libcrux_ml_kem::mlkem512);

// multiplexing API
#[cfg(feature = "mlkem768")]
wycheproof_test!(ml_kem_768, ParameterSet::MlKem768, libcrux_ml_kem::mlkem768);

// multiplexing API
#[cfg(feature = "mlkem1024")]
wycheproof_test!(
    ml_kem_1024,
    ParameterSet::MlKem1024,
    libcrux_ml_kem::mlkem1024
);

// portable
#[cfg(feature = "mlkem512")]
wycheproof_test!(
    ml_kem_512_portable,
    ParameterSet::MlKem512,
    libcrux_ml_kem::mlkem512::portable
);

// portable
#[cfg(feature = "mlkem768")]
wycheproof_test!(
    ml_kem_768_portable,
    ParameterSet::MlKem768,
    libcrux_ml_kem::mlkem768::portable
);

// portable
#[cfg(feature = "mlkem1024")]
wycheproof_test!(
    ml_kem_1024_portable,
    ParameterSet::MlKem1024,
    libcrux_ml_kem::mlkem1024::portable
);

use libcrux_kats::wycheproof::mlkem::schema::*;

use libcrux_ml_kem::{MlKemCiphertext, MlKemPublicKey};

macro_rules! wycheproof_test {
    ($name:ident, $parameter_set:expr, $module:path) => {
        mod $name {
            use super::*;

            #[test]
            fn keygen_and_decaps() {
                use $module::*;

                let katfile = MlKemTests::load($parameter_set);

                for test_group in katfile.keygen_and_decaps_tests() {
                    for test in &test_group.tests {
                        // generate key pair
                        let key_pair = generate_key_pair(test.seed.clone());

                        // convert ciphertext
                        let ciphertext: MlKemCiphertext<_> =
                            <[u8; _]>::try_from(test.ciphertext.clone()).unwrap().into();

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
                        assert_eq!(test.result, MlKemResult::Valid);
                    }
                }
            }
            #[test]
            fn encaps() {
                use $module::*;
                let katfile = MlKemTests::load($parameter_set);

                for test_group in katfile.encaps_tests() {
                    for test in &test_group.tests {
                        // all tests have an Invalid results and a `ModulusOverflow` flag
                        assert_eq!(test.result, MlKemResult::Invalid);
                        assert_eq!(test.flags, vec![Flag::ModulusOverflow]);
                        // convert to encapsulation key
                        let bytes: [u8; _] = test
                            .encapsulation_key
                            .clone()
                            .try_into()
                            .expect("invalid length");
                        let encapsulation_key: MlKemPublicKey<_> = bytes.into();

                        // validate the key (should fail for ModulusOverflow cases)
                        assert!(!validate_public_key(&encapsulation_key));
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

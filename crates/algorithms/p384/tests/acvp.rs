use libcrux_kats::acvp::{
    ec::{pad, uncompressed_point},
    kas_ecc::KasEccSscTests,
    schema_common::{assert_covers, Tally},
};
use libcrux_p384::{PrivateKey, PublicKey};

const FIELD_LEN: usize = 48;

fn private_key(d: &[u8]) -> [u8; FIELD_LEN] {
    pad(d, FIELD_LEN).unwrap().try_into().unwrap()
}

fn uncompressed(x: &[u8], y: &[u8]) -> Vec<u8> {
    uncompressed_point(x, y, FIELD_LEN).unwrap()
}

fn public_key_bytes(sk: &[u8; FIELD_LEN]) -> [u8; 2 * FIELD_LEN + 1] {
    let pk = PublicKey::from(&PrivateKey::try_from(sk.as_slice()).unwrap());
    let mut out = [0u8; 2 * FIELD_LEN + 1];
    pk.to_uncompressed(&mut out);
    out
}

#[test]
fn kas_ecc_ssc_p384() {
    let tests = KasEccSscTests::p384();
    assert_eq!(tests.prompts.algorithm, "KAS-ECC-SSC");
    assert_eq!(tests.prompts.revision, "Sp800-56Ar3");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.domainParameterGenerationMode, "P-384");
        let (iut_ephemeral, server_ephemeral) = (group.iut_ephemeral(), group.server_ephemeral());

        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            let (server_x, server_y) = test.server_public_key(server_ephemeral);
            let server_public_key = uncompressed(server_x, server_y);
            match group.testType.as_str() {
                "AFT" => {
                    // The IUT key pair is chosen by the IUT, the projection contains the one
                    // the expected results were computed with.
                    let projection = tests.find_projection(group.tgId, test.tcId);
                    let sk = private_key(projection.iut_private_key(iut_ephemeral));

                    let (x, y) = expected.iut_public_key(iut_ephemeral);
                    assert_eq!(
                        public_key_bytes(&sk).as_slice(),
                        uncompressed(x, y),
                        "tcId {}",
                        test.tcId
                    );

                    let z = libcrux_p384::derive_ecdh(&sk, &server_public_key).unwrap();
                    assert_eq!(
                        z.as_ref().as_slice(),
                        pad(expected.z.as_ref().unwrap(), FIELD_LEN).unwrap(),
                        "tcId {}",
                        test.tcId
                    );
                    tally.add(format!("AFT {}", group.scheme));
                }
                "VAL" => {
                    let sk = private_key(test.iut_private_key(iut_ephemeral));
                    let (x, y) = test.iut_public_key(iut_ephemeral);

                    let passed = public_key_bytes(&sk).as_slice() == uncompressed(x, y)
                        && libcrux_p384::derive_ecdh(&sk, &server_public_key).is_ok_and(|z| {
                            z.as_ref().as_slice() == pad(&test.z, FIELD_LEN).unwrap()
                        });
                    assert_eq!(Some(passed), expected.testPassed, "tcId {}", test.tcId);
                    tally.add(format!(
                        "VAL {} {}",
                        group.scheme,
                        if passed { "pass" } else { "fail" }
                    ));
                }
                t => panic!("unexpected testType {t}"),
            }
        }
    }
    tally.assert_counts([
        ("AFT ephemeralUnified", 5),
        ("AFT onePassDh", 5),
        ("VAL ephemeralUnified pass", 4),
        ("VAL ephemeralUnified fail", 1),
        ("VAL onePassDh pass", 4),
        ("VAL onePassDh fail", 1),
    ]);
}

#[test]
fn ecdsa_keyver_p384() {
    let tests = libcrux_kats::acvp::ecdsa::KeyVerTests::p384();
    assert_eq!(tests.prompts.algorithm, "ECDSA");
    assert_eq!(tests.prompts.mode, "keyVer");
    assert_eq!(tests.prompts.revision, "FIPS186-5");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.curve, "P-384");
        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            let valid = uncompressed_point(&test.qx, &test.qy, FIELD_LEN)
                .is_some_and(|pk| PublicKey::from_uncompressed(&pk).is_ok());
            assert_eq!(valid, expected.testPassed, "tcId {}", test.tcId);
            tally.add(if valid { "valid" } else { "invalid" });
        }
    }
    tally.assert_counts([("valid", 4), ("invalid", 8)]);
}

/// The IUT chooses its own key pairs, so the expected results of the (sample) vector set are
/// only examples. Check that they are consistent with the public key derivation, and with the
/// `rand` feature also that [`PrivateKey::generate`] produces valid key pairs.
#[test]
fn ecdsa_keygen_p384() {
    let tests = libcrux_kats::acvp::ecdsa::KeyGenTests::p384();
    assert_eq!(tests.prompts.algorithm, "ECDSA");
    assert_eq!(tests.prompts.mode, "keyGen");
    assert_eq!(tests.prompts.revision, "FIPS186-5");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.curve, "P-384");
        assert_eq!(group.secretGenerationMode, "testing candidates");
        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            assert_eq!(
                public_key_bytes(&private_key(&expected.d)).as_slice(),
                uncompressed(&expected.qx, &expected.qy),
                "tcId {}",
                test.tcId
            );

            #[cfg(feature = "rand")]
            {
                let sk = PrivateKey::generate(&mut rand::rng()).unwrap();
                // A valid private key 0 < d < n, and a valid public key Q = dG.
                assert!(PrivateKey::try_from(sk.as_ref().as_slice()).is_ok());
                let pk = public_key_bytes(sk.as_ref());
                assert!(PublicKey::from_uncompressed(&pk).is_ok());
            }
            tally.add(&group.testType);
        }
    }
    tally.assert_counts([("AFT", 3)]);
}

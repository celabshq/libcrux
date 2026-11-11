use libcrux_curve25519::{DK_LEN, EK_LEN, SS_LEN};
use libcrux_kats::acvp::{
    schema_common::{assert_covers, Tally},
    xecdh::{KeyGenTests, KeyVerTests, SscTests},
};

fn public_key(sk: &[u8]) -> [u8; EK_LEN] {
    let mut pk = [0u8; EK_LEN];
    libcrux_curve25519::secret_to_public(&mut pk, sk.try_into().unwrap());
    pk
}

/// The IUT chooses its own key pairs, so the expected results of the (sample) vector set are
/// only examples. Check that they are consistent with the public key derivation.
///
/// Key generation itself is not tested: libcrux-curve25519 has no key generation API.
#[test]
fn xecdh_keygen_curve25519() {
    let tests = KeyGenTests::curve25519();
    assert_eq!(tests.prompts.algorithm, "XECDH");
    assert_eq!(tests.prompts.mode, "keyGen");
    assert_eq!(tests.prompts.revision, "RFC7748");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.curve, "Curve25519");
        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            assert_eq!(
                public_key(&expected.privateKey),
                expected.publicKey.as_slice(),
                "tcId {}",
                test.tcId
            );
            tally.add(&group.testType);
        }
    }
    tally.assert_counts([("AFT", 10)]);
}

/// For X25519, public key validation only checks the length. Any 32 byte string, including one
/// with the most significant bit set, is a valid public key (RFC 7748, Section 5).
#[test]
fn xecdh_keyver_curve25519() {
    let tests = KeyVerTests::curve25519();
    assert_eq!(tests.prompts.algorithm, "XECDH");
    assert_eq!(tests.prompts.mode, "keyVer");
    assert_eq!(tests.prompts.revision, "RFC7748");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.curve, "Curve25519");
        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            let valid = <&[u8; EK_LEN]>::try_from(test.publicKey.as_slice()).is_ok();
            assert_eq!(valid, expected.testPassed, "tcId {}", test.tcId);
            tally.add(if valid { "valid" } else { "invalid" });
        }
    }
    tally.assert_counts([("valid", 8), ("invalid", 8)]);
}

#[test]
fn xecdh_ssc_curve25519() {
    let tests = SscTests::curve25519();
    assert_eq!(tests.prompts.algorithm, "XECDH");
    assert_eq!(tests.prompts.mode, "SSC");
    assert_eq!(tests.prompts.revision, "RFC7748");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.curve, "Curve25519");
        assert_eq!(group.testType, "AFT");
        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            // The IUT key pair is chosen by the IUT, the projection contains the one the
            // expected results were computed with.
            let sk: &[u8; DK_LEN] = tests
                .find_projection(group.tgId, test.tcId)
                .privateIut
                .as_slice()
                .try_into()
                .unwrap();
            assert_eq!(
                public_key(sk),
                expected.publicIut.as_slice(),
                "tcId {}",
                test.tcId
            );

            let pk: &[u8; EK_LEN] = test.publicServer.as_slice().try_into().unwrap();
            let mut z = [0u8; SS_LEN];
            assert!(libcrux_curve25519::ecdh(&mut z, pk, sk).is_ok());
            assert_eq!(z.as_slice(), expected.z, "tcId {}", test.tcId);
            tally.add(&group.testType);
        }
    }
    tally.assert_counts([("AFT", 25)]);
}

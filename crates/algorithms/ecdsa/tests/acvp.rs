use libcrux_ecdsa::{
    p256::{PrivateKey, PublicKey, Signature},
    DigestAlgorithm,
};
use libcrux_kats::acvp::{
    ec::{pad, raw_point},
    ecdsa::{KeyGenTests, KeyVerTests, SigGenTests, SigVerTests},
    schema_common::{assert_covers, Tally},
};

const FIELD_LEN: usize = 32;

fn digest(hash_alg: &str) -> DigestAlgorithm {
    match hash_alg {
        "SHA2-256" => DigestAlgorithm::Sha256,
        "SHA2-384" => DigestAlgorithm::Sha384,
        "SHA2-512" => DigestAlgorithm::Sha512,
        h => panic!("unexpected hashAlg {h}"),
    }
}

/// The public key `(x, y)`, or `None` if it is invalid.
fn public_key(x: &[u8], y: &[u8]) -> Option<PublicKey> {
    PublicKey::try_from(raw_point(x, y, FIELD_LEN)?.as_slice()).ok()
}

/// Whether `(r, s)` is a valid signature on `message` for the public key `(qx, qy)`.
fn verify(hash: DigestAlgorithm, message: &[u8], qx: &[u8], qy: &[u8], r: &[u8], s: &[u8]) -> bool {
    let (Some(pk), Some(r), Some(s)) = (public_key(qx, qy), pad(r, FIELD_LEN), pad(s, FIELD_LEN))
    else {
        return false;
    };
    let signature = Signature::from_raw(r.try_into().unwrap(), s.try_into().unwrap());
    libcrux_ecdsa::p256::verify(hash, message, &signature, &pk).is_ok()
}

#[test]
fn ecdsa_keyver_p256() {
    let tests = KeyVerTests::p256();
    assert_eq!(tests.prompts.algorithm, "ECDSA");
    assert_eq!(tests.prompts.mode, "keyVer");
    assert_eq!(tests.prompts.revision, "FIPS186-5");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.curve, "P-256");
        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            let valid = public_key(&test.qx, &test.qy).is_some();
            assert_eq!(valid, expected.testPassed, "tcId {}", test.tcId);
            tally.add(if valid { "valid" } else { "invalid" });
        }
    }
    tally.assert_counts([("valid", 4), ("invalid", 8)]);
}

/// The IUT chooses its own key pairs, so the expected results of the (sample) vector set are
/// only examples. Check that they are consistent with the public key derivation, and with the
/// `rand` feature (enabled by default) also that key generation produces valid key pairs.
#[test]
fn ecdsa_keygen_p256() {
    let tests = KeyGenTests::p256();
    assert_eq!(tests.prompts.algorithm, "ECDSA");
    assert_eq!(tests.prompts.mode, "keyGen");
    assert_eq!(tests.prompts.revision, "FIPS186-5");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.curve, "P-256");
        assert_eq!(group.secretGenerationMode, "testing candidates");
        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            let d: [u8; FIELD_LEN] = pad(&expected.d, FIELD_LEN).unwrap().try_into().unwrap();
            let pk = PrivateKey::try_from(&d).unwrap().public_key().unwrap();
            assert_eq!(
                Some(pk.0.to_vec()),
                raw_point(&expected.qx, &expected.qy, FIELD_LEN),
                "tcId {}",
                test.tcId
            );

            #[cfg(feature = "rand")]
            {
                let (sk, pk) =
                    libcrux_ecdsa::p256::rand::generate_key_pair(&mut rand::rngs::SysRng).unwrap();
                let sk: &[u8; FIELD_LEN] = sk.as_ref();
                assert!(PrivateKey::try_from(sk).is_ok());
                assert!(PublicKey::try_from(&pk.0).is_ok());
            }
            tally.add(&group.testType);
        }
    }
    tally.assert_counts([("AFT", 3)]);
}

#[test]
fn ecdsa_sigver_p256() {
    let tests = SigVerTests::p256();
    assert_eq!(tests.prompts.algorithm, "ECDSA");
    assert_eq!(tests.prompts.mode, "sigVer");
    assert_eq!(tests.prompts.revision, "FIPS186-5");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.curve, "P-256");
        let hash = digest(&group.hashAlg);
        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            let valid = verify(hash, &test.message, &test.qx, &test.qy, &test.r, &test.s);
            assert_eq!(valid, expected.testPassed, "tcId {}", test.tcId);
            tally.add(format!(
                "{} {}",
                group.hashAlg,
                if valid { "valid" } else { "invalid" }
            ));
        }
    }
    tally.assert_counts([
        ("SHA2-256 valid", 3),
        ("SHA2-256 invalid", 18),
        ("SHA2-384 valid", 3),
        ("SHA2-384 invalid", 18),
        ("SHA2-512 valid", 3),
        ("SHA2-512 invalid", 18),
    ]);
}

/// Signatures are randomized and the IUT chooses its own key pair per test group, so the
/// expected results of the (sample) vector set are only examples. Check that they verify, and
/// with the `rand` feature (enabled by default) also that signatures of the messages verify.
#[test]
fn ecdsa_siggen_p256() {
    let tests = SigGenTests::p256();
    assert_eq!(tests.prompts.algorithm, "ECDSA");
    assert_eq!(tests.prompts.mode, "sigGen");
    assert_eq!(tests.prompts.revision, "FIPS186-5");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.curve, "P-256");
        assert!(!group.componentTest);
        let hash = digest(&group.hashAlg);
        let expected_group = tests.results.find_group(group.tgId);

        #[cfg(feature = "rand")]
        let (sk, pk) =
            libcrux_ecdsa::p256::rand::generate_key_pair(&mut rand::rngs::SysRng).unwrap();

        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            assert!(
                verify(
                    hash,
                    &test.message,
                    &expected_group.qx,
                    &expected_group.qy,
                    &expected.r,
                    &expected.s,
                ),
                "tcId {}",
                test.tcId
            );

            #[cfg(feature = "rand")]
            {
                let signature = libcrux_ecdsa::p256::rand::sign(
                    hash,
                    &test.message,
                    &sk,
                    &mut rand::rngs::SysRng,
                )
                .unwrap();
                let (r, s) = signature.as_bytes();
                let (qx, qy) = pk.0.split_at(FIELD_LEN);
                assert!(
                    verify(hash, &test.message, qx, qy, r, s),
                    "tcId {}",
                    test.tcId
                );
            }
            tally.add(&group.hashAlg);
        }
    }
    tally.assert_counts([("SHA2-256", 10), ("SHA2-384", 10), ("SHA2-512", 10)]);
}

use libcrux_kats::acvp::{
    eddsa::{KeyGenTests, SigGenTests, SigVerTests},
    schema_common::{assert_covers, Tally},
};

fn public_key(d: &[u8]) -> [u8; 32] {
    let mut q = [0u8; 32];
    libcrux_ed25519::secret_to_public(&mut q, d.try_into().unwrap());
    q
}

/// The IUT chooses its own key pairs, so the expected results of the (sample) vector set are
/// only examples. Check that they are consistent with the public key derivation, and with the
/// `rand` feature also that key generation produces consistent key pairs.
#[test]
fn eddsa_keygen_ed25519() {
    let tests = KeyGenTests::ed25519();
    assert_eq!(tests.prompts.algorithm, "EDDSA");
    assert_eq!(tests.prompts.mode, "keyGen");
    assert_eq!(tests.prompts.revision, "1.0");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.curve, "ED-25519");
        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            assert_eq!(
                public_key(&expected.d),
                expected.q.as_slice(),
                "tcId {}",
                test.tcId
            );

            #[cfg(feature = "rand")]
            {
                let (sk, pk) = libcrux_ed25519::generate_key_pair(&mut rand::rng()).unwrap();
                assert_eq!(&public_key(sk.as_ref()), pk.as_ref());
            }
            tally.add(&group.testType);
        }
    }
    tally.assert_counts([("AFT", 3)]);
}

/// Ed25519 signatures are deterministic, so the signatures with the key pair of each group (from
/// the internal projection) must match.
#[test]
fn eddsa_siggen_ed25519() {
    let tests = SigGenTests::ed25519();
    assert_eq!(tests.prompts.algorithm, "EDDSA");
    assert_eq!(tests.prompts.mode, "sigGen");
    assert_eq!(tests.prompts.revision, "1.0");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.curve, "ED-25519");
        assert!(!group.preHash);
        assert!(["AFT", "BFT"].contains(&group.testType.as_str()));

        let projection = tests.find_projection(group.tgId);
        let d: &[u8; 32] = projection.d.as_slice().try_into().unwrap();
        assert_eq!(
            public_key(d),
            projection.q.as_slice(),
            "tgId {}",
            group.tgId
        );
        assert_eq!(
            projection.q,
            tests.results.find_group(group.tgId).q,
            "tgId {}",
            group.tgId
        );

        for test in &group.tests {
            assert!(test.context.is_empty());
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            let signature = libcrux_ed25519::sign(&test.message, d).unwrap();
            assert_eq!(
                signature.as_slice(),
                expected.signature,
                "tcId {}",
                test.tcId
            );
            tally.add(&group.testType);
        }
    }
    tally.assert_counts([("AFT", 10), ("BFT", 32)]);
}

#[test]
fn eddsa_sigver_ed25519() {
    let tests = SigVerTests::ed25519();
    assert_eq!(tests.prompts.algorithm, "EDDSA");
    assert_eq!(tests.prompts.mode, "sigVer");
    assert_eq!(tests.prompts.revision, "1.0");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.curve, "ED-25519");
        assert!(!group.preHash);
        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            let valid = match (
                <&[u8; 32]>::try_from(test.q.as_slice()),
                <&[u8; 64]>::try_from(test.signature.as_slice()),
            ) {
                (Ok(q), Ok(signature)) => {
                    libcrux_ed25519::verify(&test.message, q, signature).is_ok()
                }
                _ => false,
            };
            assert_eq!(valid, expected.testPassed, "tcId {}", test.tcId);
            tally.add(if valid { "valid" } else { "invalid" });
        }
    }
    tally.assert_counts([("valid", 3), ("invalid", 12)]);
}

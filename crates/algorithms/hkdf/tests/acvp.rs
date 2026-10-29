use libcrux_hkdf::Algorithm;
use libcrux_kats::acvp::{
    kda::HkdfTests,
    schema_common::{assert_covers, Tally},
};
use libcrux_secrets::{ClassifyRef, ClassifyRefMut};

fn algorithm(hmac_alg: &str) -> Algorithm {
    match hmac_alg {
        "SHA2-256" => Algorithm::Sha256,
        "SHA2-384" => Algorithm::Sha384,
        "SHA2-512" => Algorithm::Sha512,
        "SHA3-224" => Algorithm::Sha3_224,
        "SHA3-256" => Algorithm::Sha3_256,
        "SHA3-384" => Algorithm::Sha3_384,
        "SHA3-512" => Algorithm::Sha3_512,
        a => panic!("unexpected hmacAlg {a}"),
    }
}

#[test]
fn hkdf() {
    let tests = HkdfTests::sha2_sha3();
    assert_eq!(tests.prompts.algorithm, "KDA");
    assert_eq!(tests.prompts.mode, "HKDF");
    assert_eq!(tests.prompts.revision, "Sp800-56Cr2");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert!(!group.usesHybridSharedSecret);
        assert!(!group.multiExpansion);
        let config = &group.kdfConfiguration;
        assert_eq!(config.fixedInfoEncoding, "concatenation");

        for test in &group.tests {
            let param = &test.kdfParameter;
            assert_eq!(param.l, config.l);
            assert_eq!(param.salt.len() * 8, config.saltLen);
            assert_eq!(param.z.len() * 8, group.zLength);

            let mut dkm = vec![0u8; param.l / 8];
            libcrux_hkdf::hkdf(
                algorithm(&config.hmacAlg),
                dkm.as_mut_slice().classify_ref_mut(),
                param.salt.classify_ref(),
                param.z.classify_ref(),
                &test.fixed_info(&config.fixedInfoPattern),
            )
            .unwrap();

            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            match group.testType.as_str() {
                "AFT" => assert_eq!(Some(&dkm), expected.dkm.as_ref(), "tcId {}", test.tcId),
                "VAL" => assert_eq!(
                    test.dkm.as_ref() == Some(&dkm),
                    expected.testPassed.unwrap(),
                    "tcId {}",
                    test.tcId
                ),
                t => panic!("unexpected testType {t}"),
            }
            tally.add(&group.testType);
        }
    }
    tally.assert_counts([("AFT", 350), ("VAL", 350)]);
}

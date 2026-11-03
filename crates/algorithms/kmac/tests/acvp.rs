use libcrux_kats::acvp::{
    kmac::KmacTests,
    schema_common::{assert_covers, Tally},
};

/// `kmac(tag, key, data, customization)`.
type KmacFn = for<'a> fn(&'a mut [u8], &[u8], &[u8], &[u8]) -> &'a [u8];

fn kmac(tests: KmacTests, algorithm: &str, kmac: KmacFn) {
    assert_eq!(tests.prompts.algorithm, algorithm);
    assert_eq!(tests.prompts.revision, "1.0");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert!(!group.xof, "only KMAC without XOF is registered");
        for test in &group.tests {
            assert_eq!(test.keyLen, test.key.len() * 8);
            assert_eq!(test.msgLen, test.msg.len() * 8);
            assert_eq!(test.macLen % 8, 0);

            let mut mac = vec![0u8; test.macLen / 8];
            kmac(&mut mac, &test.key, &test.msg, group.customization(test));

            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            match group.testType.as_str() {
                "AFT" => assert_eq!(Some(mac), expected.mac, "tcId {}", test.tcId),
                "MVT" => assert_eq!(
                    mac == test.mac,
                    expected.testPassed.unwrap(),
                    "tcId {}",
                    test.tcId
                ),
                t => panic!("unexpected testType {t}"),
            }
            tally.add(&group.testType);
        }
    }
    tally.assert_counts([("AFT", 200), ("MVT", 200)]);
}

#[test]
fn kmac_128() {
    kmac(KmacTests::kmac_128(), "KMAC-128", libcrux_kmac::kmac_128);
}

#[test]
fn kmac_256() {
    kmac(KmacTests::kmac_256(), "KMAC-256", libcrux_kmac::kmac_256);
}

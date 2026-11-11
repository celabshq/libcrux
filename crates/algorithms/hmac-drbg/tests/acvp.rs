use libcrux_hmac_drbg::{HmacDrbgSha256, HmacDrbgSha384, HmacDrbgSha512};
use libcrux_kats::acvp::{
    drbg::HmacDrbgTests,
    schema_common::{assert_covers, Tally},
};

/// Instantiate a `$drbg`, then run the `reSeed` and `generate` operations of `$test` in order.
///
/// Evaluates to the output of the last `generate` call.
macro_rules! run {
    ($drbg:ty, $test:expr, $returned_bytes:expr) => {{
        let test = $test;
        let mut drbg = <$drbg>::new(&test.entropyInput, &test.nonce, &test.persoString).unwrap();

        let mut out = vec![0u8; $returned_bytes];
        for op in &test.otherInput {
            match op.intendedUse.as_str() {
                "reSeed" => drbg.reseed(&op.entropyInput, &op.additionalInput).unwrap(),
                "generate" => {
                    // Without prediction resistance, generate takes no entropy input.
                    assert!(op.entropyInput.is_empty());
                    drbg.generate(&mut out, &op.additionalInput).unwrap();
                }
                u => panic!("unexpected intendedUse {u}"),
            }
        }
        out
    }};
}

fn hmac_drbg<const N: usize>(tests: HmacDrbgTests, expected_counts: [(&str, usize); N]) {
    assert_eq!(tests.prompts.algorithm, "hmacDRBG");
    assert_eq!(tests.prompts.revision, "SP800-90Ar1");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.testType, "AFT");
        assert!(!group.derFunc);
        assert!(!group.predResistance);
        assert_eq!(group.returnedBitsLen % 8, 0);

        for test in &group.tests {
            assert_eq!(test.entropyInput.len() * 8, group.entropyInputLen);
            assert_eq!(test.nonce.len() * 8, group.nonceLen);
            assert_eq!(test.persoString.len() * 8, group.persoStringLen);
            for op in &test.otherInput {
                assert_eq!(op.additionalInput.len() * 8, group.additionalInputLen);
                if op.intendedUse == "reSeed" {
                    assert_eq!(op.entropyInput.len() * 8, group.entropyInputLen);
                }
            }
            assert_eq!(
                test.otherInput.iter().any(|op| op.intendedUse == "reSeed"),
                group.reSeed,
                "tcId {}",
                test.tcId
            );

            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            let returned_bytes = group.returnedBitsLen / 8;
            let out = match group.mode.as_str() {
                "SHA2-256" => run!(HmacDrbgSha256, test, returned_bytes),
                "SHA2-384" => run!(HmacDrbgSha384, test, returned_bytes),
                "SHA2-512" => run!(HmacDrbgSha512, test, returned_bytes),
                m => panic!("unexpected mode {m}"),
            };
            assert_eq!(out, expected.returnedBits, "tcId {}", test.tcId);
            tally.add(&group.mode);
        }
    }
    tally.assert_counts(expected_counts);
}

#[test]
fn hmac_drbg_sha2_256_sha2_384() {
    hmac_drbg(
        HmacDrbgTests::sha2_256_sha2_384(),
        [("SHA2-256", 30), ("SHA2-384", 30)],
    );
}

#[test]
fn hmac_drbg_sha2_512() {
    hmac_drbg(HmacDrbgTests::sha2_512(), [("SHA2-512", 30)]);
}

use libcrux_hmac::{
    HmacSha256, HmacSha384, HmacSha3_224, HmacSha3_256, HmacSha3_384, HmacSha3_512, HmacSha512,
    HmacState,
};
use libcrux_kats::acvp::{
    kbkdf::{
        schema::{KbkdfPrompt, KbkdfResult},
        FeedbackHmacTests,
    },
    kda::{
        schema::{TwoStepPrompt, TwoStepResult},
        TwoStepFeedbackHmacTests,
    },
};
use libcrux_nist_kdf::{feedback, two_step};

fn check<const OUTLEN: usize, H: HmacState<OUTLEN>>(test: &KbkdfPrompt, expected: &KbkdfResult) {
    let mut k_out = vec![0; expected.keyOut.len()];
    feedback::kdf::<OUTLEN, H>(&mut k_out, &test.keyIn, &test.iv, &[&expected.fixedData]).unwrap();
    assert_eq!(k_out, expected.keyOut, "tcId {}", test.tcId);
}

#[test]
fn feedback_hmac() {
    let FeedbackHmacTests { prompts, results } = FeedbackHmacTests::load();
    assert_eq!(prompts.algorithm, "KDF");
    assert_eq!(results.algorithm, "KDF");

    let mut tested = 0;
    for group in prompts.testGroups {
        assert_eq!(group.testType, "AFT");
        assert_eq!(group.kdfMode, "feedback");
        assert_eq!(group.counterLength, 32);
        assert_eq!(group.counterLocation, "before fixed data");

        // The KDF only outputs whole bytes.
        if group.keyOutLength % 8 != 0 {
            continue;
        }

        let check = match group.macMode.as_str() {
            "HMAC-SHA2-256" => check::<32, HmacSha256>,
            "HMAC-SHA2-384" => check::<48, HmacSha384>,
            "HMAC-SHA2-512" => check::<64, HmacSha512>,
            "HMAC-SHA3-224" => check::<28, HmacSha3_224>,
            "HMAC-SHA3-256" => check::<32, HmacSha3_256>,
            "HMAC-SHA3-384" => check::<48, HmacSha3_384>,
            "HMAC-SHA3-512" => check::<64, HmacSha3_512>,
            // Not supported by libcrux-hmac.
            "HMAC-SHA-1" | "HMAC-SHA2-224" | "HMAC-SHA2-512/224" | "HMAC-SHA2-512/256" => continue,
            mac => panic!("unexpected macMode {mac}"),
        };

        for test in &group.tests {
            let expected = results.find_expected_result(group.tgId, test.tcId);
            assert_eq!(expected.keyOut.len() * 8, group.keyOutLength);
            assert_eq!(test.iv.is_empty(), group.zeroLengthIv);
            check(test, expected);
            tested += 1;
        }
    }
    // Guard against silently skipping all tests.
    assert_eq!(tested, 100);
}

/// `two_step::kdf` with the HMAC fixed by the caller.
type TwoStepKdf = fn(&mut [u8], &[u8], &[u8], &[u8], &[&[u8]]);

fn two_step_kdf<const OUTLEN: usize, H: HmacState<OUTLEN>>(
    k_out: &mut [u8],
    shared_secret: &[u8],
    salt: &[u8],
    iv: &[u8],
    fixed_info: &[&[u8]],
) {
    two_step::kdf::<OUTLEN, H>(k_out, shared_secret, salt, iv, fixed_info).unwrap();
}

fn two_step_kdf_for(mac: &str) -> TwoStepKdf {
    match mac {
        "HMAC-SHA2-256" => two_step_kdf::<32, HmacSha256>,
        "HMAC-SHA2-384" => two_step_kdf::<48, HmacSha384>,
        "HMAC-SHA2-512" => two_step_kdf::<64, HmacSha512>,
        "HMAC-SHA3-224" => two_step_kdf::<28, HmacSha3_224>,
        "HMAC-SHA3-256" => two_step_kdf::<32, HmacSha3_256>,
        "HMAC-SHA3-384" => two_step_kdf::<48, HmacSha3_384>,
        "HMAC-SHA3-512" => two_step_kdf::<64, HmacSha3_512>,
        mac => panic!("unexpected macMode {mac}"),
    }
}

/// Compare the derived keying material against the expected result.
///
/// For `AFT` tests, `dkms` must equal the expected result. For `VAL` tests,
/// `dkms` must equal the dkm(s) in the prompt iff the test is expected to pass.
fn check_two_step(
    test_type: &str,
    tc_id: usize,
    dkms: &[Vec<u8>],
    prompt_dkms: Option<&[Vec<u8>]>,
    expected_dkms: Option<&[Vec<u8>]>,
    expected: &TwoStepResult,
) {
    match test_type {
        "AFT" => assert_eq!(Some(dkms), expected_dkms, "tcId {tc_id}"),
        "VAL" => {
            let prompt_dkms = prompt_dkms.expect("VAL test must contain the dkm");
            let passed = expected.testPassed.expect("VAL test must have a result");
            assert_eq!(dkms == prompt_dkms, passed, "tcId {tc_id}");
        }
        t => panic!("unexpected testType {t}"),
    }
}

#[test]
fn two_step_hmac() {
    let mut tested = 0;
    for TwoStepFeedbackHmacTests { prompts, results } in TwoStepFeedbackHmacTests::load_all() {
        assert_eq!(prompts.algorithm, "KDA");
        assert_eq!(prompts.mode, "TwoStep");
        assert_eq!(prompts.revision, "Sp800-56Cr2");

        for group in prompts.testGroups {
            assert!(!group.usesHybridSharedSecret);

            for test in &group.tests {
                match test {
                    TwoStepPrompt::Single(test) => {
                        let config = group.kdfConfiguration.as_ref().unwrap();
                        assert!(!group.multiExpansion);
                        assert_eq!(config.kdfMode, "feedback");
                        assert_eq!(config.counterLen, 32);
                        assert_eq!(config.counterLocation, "before fixed data");
                        assert_eq!(config.fixedInfoPattern, "uPartyInfo||vPartyInfo||l");
                        assert_eq!(config.fixedInfoEncoding, "concatenation");

                        let param = &test.kdfParameter;
                        assert_eq!(param.l, config.l);
                        assert_eq!(param.iv.len() * 8, config.ivLen);
                        assert_eq!(param.salt.len() * 8, config.saltLen);
                        assert_eq!(param.z.len() * 8, group.zLength);

                        // uPartyInfo || vPartyInfo || l, with l as a 32-bit big endian integer.
                        let u = test.fixedInfoPartyU.encode();
                        let v = test.fixedInfoPartyV.encode();
                        let l = u32::try_from(param.l).unwrap().to_be_bytes();

                        let mut dkm = vec![0; param.l / 8];
                        two_step_kdf_for(&config.macMode)(
                            &mut dkm,
                            &param.z,
                            &param.salt,
                            &param.iv,
                            &[&u, &v, &l],
                        );

                        let expected = results.find_expected_result(group.tgId, test.tcId);
                        check_two_step(
                            &group.testType,
                            test.tcId,
                            &[dkm],
                            test.dkm.as_ref().map(core::slice::from_ref),
                            expected.dkm.as_ref().map(core::slice::from_ref),
                            expected,
                        );
                    }
                    TwoStepPrompt::MultiExpansion(test) => {
                        let config = group.kdfMultiExpansionConfiguration.as_ref().unwrap();
                        assert!(group.multiExpansion);

                        let param = &test.kdfMultiExpansionParameter;
                        assert_eq!(param.kdfMode, "feedback");
                        assert_eq!(param.counterLen, 32);
                        assert_eq!(param.counterLocation, "before fixed data");
                        assert_eq!(param.macMode, config.macMode);
                        assert_eq!(param.iv.len() * 8, config.ivLen);
                        assert_eq!(param.salt.len() * 8, config.saltLen);
                        assert_eq!(param.z.len() * 8, group.zLength);

                        let kdf = two_step_kdf_for(&param.macMode);
                        let dkms: Vec<_> = param
                            .iterationParameters
                            .iter()
                            .map(|iteration| {
                                let mut dkm = vec![0; iteration.l / 8];
                                kdf(
                                    &mut dkm,
                                    &param.z,
                                    &param.salt,
                                    &param.iv,
                                    &[&iteration.fixedInfo],
                                );
                                dkm
                            })
                            .collect();

                        let expected = results.find_expected_result(group.tgId, test.tcId);
                        check_two_step(
                            &group.testType,
                            test.tcId,
                            &dkms,
                            test.dkms.as_deref(),
                            expected.dkms.as_deref(),
                            expected,
                        );
                    }
                }
                tested += 1;
            }
        }
    }
    // Guard against silently skipping tests.
    assert_eq!(tested, 3 * 2800);
}

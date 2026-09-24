//! Wycheproof tests for HMAC-SHA2 (256, 384, 512).

use libcrux_hkdf::{hkdf, Algorithm, ExpandError};
use libcrux_kats::wycheproof::{
    hkdf::{HashAlgorithm, HkdfTests, Test},
    schema_common::TestResult,
};
use libcrux_secrets::{ClassifyRef, ClassifyRefMut};

fn run(tests: &HkdfTests, alg: Algorithm) {
    let mut test_groups_run = 0;
    for group in &tests.test_groups {
        let mut tests_run = 0;
        for t in &group.tests {
            check(t, alg);
            tests_run += 1;
        }

        debug_assert_ne!(tests_run, 0);
        test_groups_run += 1;
    }

    debug_assert_ne!(test_groups_run, 0);
}

fn check(t: &Test, alg: Algorithm) {
    let mut okm = vec![0u8; t.size];
    let result = hkdf(
        alg,
        okm.as_mut_slice().classify_ref_mut(),
        t.salt.as_slice().classify_ref(),
        t.ikm.as_slice().classify_ref(),
        &t.info,
    );

    match t.result {
        TestResult::Valid => {
            result.unwrap_or_else(|_| panic!("tcId={} should be valid", t.tc_id));
            assert_eq!(
                okm.as_slice(),
                t.okm.as_slice(),
                "tcId={} should be valid",
                t.tc_id
            );
        }
        TestResult::Invalid => {
            assert_eq!(
                result,
                Err(ExpandError::OutputTooLong),
                "tcId={} should be invalid",
                t.tc_id
            );
        }
        TestResult::Acceptable => {} // skip
    }
}

#[test]
fn wycheproof_sha256() {
    run(&HkdfTests::load(HashAlgorithm::Sha256), Algorithm::Sha256);
}

#[test]
fn wycheproof_sha384() {
    run(&HkdfTests::load(HashAlgorithm::Sha384), Algorithm::Sha384);
}

#[test]
fn wycheproof_sha512() {
    run(&HkdfTests::load(HashAlgorithm::Sha512), Algorithm::Sha512);
}

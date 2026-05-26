use libcrux_hmac::{hmac, Algorithm, HmacSha256, HmacSha384, HmacSha512, HmacState};
use libcrux_kats::wycheproof::{
    hmac::{HashAlgorithm, HmacTests, Test},
    schema_common::TestResult,
};

fn run<const OUTLEN: usize, Digest: HmacState<OUTLEN>>(
    tests: &HmacTests,
    alg: Algorithm,
    dst: &mut [u8; OUTLEN],
) {
    let mut test_groups_run = 0;
    for group in &tests.test_groups {
        let mut tests_run = 0;
        for t in &group.tests {
            // Incremental API
            let mut h = Digest::new(&t.key).unwrap();
            h.update(&t.msg).unwrap();
            h.finalize(dst);

            // tag_size may be truncated; compare only the prefix
            let tag_bytes = (group.tag_size / 8) as usize;
            let computed = &dst[..tag_bytes];

            check(t, computed);

            // Single shot API
            let computed = hmac(alg, &t.key, &t.msg, None);
            let computed = &computed[..tag_bytes];
            check(t, computed);
            tests_run += 1;
        }

        debug_assert_ne!(tests_run, 0);
        test_groups_run += 1;
    }

    debug_assert_ne!(test_groups_run, 0);
}

fn check(t: &Test, computed: &[u8]) {
    match t.result {
        TestResult::Valid => assert_eq!(
            computed,
            t.tag.as_slice(),
            "tcId={} should be valid",
            t.tc_id
        ),
        TestResult::Invalid => assert_ne!(
            computed,
            t.tag.as_slice(),
            "tcId={} should be invalid",
            t.tc_id
        ),
        TestResult::Acceptable => {} // skip
    }
}

#[test]
fn wycheproof_sha256() {
    run::<32, HmacSha256>(
        &HmacTests::load(HashAlgorithm::Sha256),
        Algorithm::Sha256,
        &mut [0u8; 32],
    );
}

#[test]
fn wycheproof_sha384() {
    run::<48, HmacSha384>(
        &HmacTests::load(HashAlgorithm::Sha384),
        Algorithm::Sha384,
        &mut [0u8; 48],
    );
}

#[test]
fn wycheproof_sha512() {
    run::<64, HmacSha512>(
        &HmacTests::load(HashAlgorithm::Sha512),
        Algorithm::Sha512,
        &mut [0u8; 64],
    );
}

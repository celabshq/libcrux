use libcrux_hmac::{
    hmac, Algorithm, HmacSha256, HmacSha384, HmacSha3_224, HmacSha3_256, HmacSha3_384,
    HmacSha3_512, HmacSha512, HmacState,
};
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
            // tag_size may be truncated; compare only the prefix
            let tag_bytes = (group.tag_size / 8) as usize;

            // Incremental API, fed in a single update call
            let mut h = Digest::new(&t.key).unwrap();
            h.update(&t.msg).unwrap();
            h.finalize(dst);
            check(t, &dst[..tag_bytes]);

            // Incremental API, fed in multiple update calls to exercise streaming
            let mut h = Digest::new(&t.key).unwrap();
            let split = t.msg.len() / 2;
            let (first, second) = t.msg.split_at(split);
            h.update(first).unwrap();
            h.update(second).unwrap();
            h.finalize(dst);
            check(t, &dst[..tag_bytes]);
            // tag_size may be truncated; compare only the prefix
            let tag_bytes = (group.tag_size / 8) as usize;
            let computed = &mut dst[..tag_bytes];

            check(t, computed);

            // Single shot API
            hmac(alg, &t.key, &t.msg, computed);
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

#[test]
fn wycheproof_sha3_224() {
    run::<28, HmacSha3_224>(
        &HmacTests::load(HashAlgorithm::Sha3_224),
        Algorithm::Sha3_224,
        &mut [0u8; 28],
    );
}

#[test]
fn wycheproof_sha3_256() {
    run::<32, HmacSha3_256>(
        &HmacTests::load(HashAlgorithm::Sha3_256),
        Algorithm::Sha3_256,
        &mut [0u8; 32],
    );
}

#[test]
fn wycheproof_sha3_384() {
    run::<48, HmacSha3_384>(
        &HmacTests::load(HashAlgorithm::Sha3_384),
        Algorithm::Sha3_384,
        &mut [0u8; 48],
    );
}

#[test]
fn wycheproof_sha3_512() {
    run::<64, HmacSha3_512>(
        &HmacTests::load(HashAlgorithm::Sha3_512),
        Algorithm::Sha3_512,
        &mut [0u8; 64],
    );
}

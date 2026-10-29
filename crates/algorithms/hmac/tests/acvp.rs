use libcrux_hmac::{
    hmac, Algorithm, HmacSha256, HmacSha384, HmacSha3_224, HmacSha3_256, HmacSha3_384,
    HmacSha3_512, HmacSha512, HmacState,
};
use libcrux_kats::acvp::{
    hmac::HmacTests,
    schema_common::{assert_covers, Tally},
};

fn run<const OUTLEN: usize, Mac: HmacState<OUTLEN>>(
    tests: HmacTests,
    algorithm: &str,
    alg: Algorithm,
) {
    assert_eq!(tests.prompts.algorithm, algorithm);
    assert_eq!(tests.prompts.revision, "2.0");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.testType, "AFT");
        for test in &group.tests {
            assert_eq!(test.keyLen, test.key.len() * 8);
            assert_eq!(test.msgLen, test.msg.len() * 8);
            assert_eq!(
                test.macLen,
                OUTLEN * 8,
                "only untruncated MACs are registered"
            );

            let expected = tests.results.find_expected_result(group.tgId, test.tcId);

            // Incremental API, as used by 3KEM and the HMAC_DRBG.
            let mut tag = [0u8; OUTLEN];
            let mut state = Mac::new(&test.key).unwrap();
            state.update(&test.msg).unwrap();
            state.finalize(&mut tag);
            assert_eq!(tag.as_slice(), expected.mac, "tcId {}", test.tcId);

            // Single shot API.
            let mut tag = [0u8; OUTLEN];
            hmac(alg, &test.key, &test.msg, &mut tag);
            assert_eq!(tag.as_slice(), expected.mac, "tcId {}", test.tcId);

            tally.add(&group.testType);
        }
    }
    tally.assert_counts([("AFT", 150)]);
}

#[test]
fn hmac_sha2_256() {
    run::<32, HmacSha256>(
        HmacTests::hmac_sha2_256(),
        "HMAC-SHA2-256",
        Algorithm::Sha256,
    );
}

#[test]
fn hmac_sha2_384() {
    run::<48, HmacSha384>(
        HmacTests::hmac_sha2_384(),
        "HMAC-SHA2-384",
        Algorithm::Sha384,
    );
}

#[test]
fn hmac_sha3_256() {
    run::<32, HmacSha3_256>(
        HmacTests::hmac_sha3_256(),
        "HMAC-SHA3-256",
        Algorithm::Sha3_256,
    );
}

#[test]
fn hmac_sha3_384() {
    run::<48, HmacSha3_384>(
        HmacTests::hmac_sha3_384(),
        "HMAC-SHA3-384",
        Algorithm::Sha3_384,
    );
}

#[test]
fn hmac_sha2_512() {
    run::<64, HmacSha512>(
        HmacTests::hmac_sha2_512(),
        "HMAC-SHA2-512",
        Algorithm::Sha512,
    );
}

#[test]
fn hmac_sha3_224() {
    run::<28, HmacSha3_224>(
        HmacTests::hmac_sha3_224(),
        "HMAC-SHA3-224",
        Algorithm::Sha3_224,
    );
}

#[test]
fn hmac_sha3_512() {
    run::<64, HmacSha3_512>(
        HmacTests::hmac_sha3_512(),
        "HMAC-SHA3-512",
        Algorithm::Sha3_512,
    );
}

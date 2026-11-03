use libcrux_kats::acvp::{
    hash::sha2_mct,
    schema_common::{assert_covers, Tally},
    sha2::HashTests,
};

/// `hash(msg)` into a digest of the algorithm's size.
type HashFn = fn(&[u8]) -> Vec<u8>;

fn hash_sha256(msg: &[u8]) -> Vec<u8> {
    let mut hasher = libcrux_sha2::Sha256Hasher::new();
    hasher.update(msg).unwrap();
    hasher.finish_to_owned().to_vec()
}

fn hash_sha384(msg: &[u8]) -> Vec<u8> {
    let mut hasher = libcrux_sha2::Sha384Hasher::new();
    hasher.update(msg).unwrap();
    hasher.finish_to_owned().to_vec()
}

/// The number of AFT test cases of `algorithm`.
fn aft_count(algorithm: &str) -> usize {
    match algorithm {
        "SHA2-224" | "SHA2-256" => 512,
        "SHA2-384" | "SHA2-512" => 1024,
        a => panic!("unexpected algorithm {a}"),
    }
}

/// SHA-2 AFT and MCT. The LDT is run separately in [`sha2_ldt`].
fn sha2(tests: HashTests, algorithm: &str, hash: HashFn) {
    assert_eq!(tests.prompts.algorithm, algorithm);
    assert_eq!(tests.prompts.revision, "1.0");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            match group.testType.as_str() {
                "AFT" => {
                    assert_eq!(test.len, test.msg.len() * 8);
                    assert_eq!(Some(hash(&test.msg)), expected.md, "tcId {}", test.tcId);
                }
                "MCT" => {
                    assert_eq!(group.mctVersion.as_deref(), Some("standard"));
                    assert_eq!(
                        sha2_mct(&test.msg, hash),
                        expected.resultsArray,
                        "tcId {}",
                        test.tcId
                    );
                }
                "LDT" => {
                    tally.add("LDT (skipped, see sha2_ldt)");
                    continue;
                }
                t => panic!("unexpected testType {t}"),
            }
            tally.add(&group.testType);
        }
    }
    tally.assert_counts([
        ("AFT", aft_count(algorithm)),
        ("MCT", 1),
        ("LDT (skipped, see sha2_ldt)", 4),
    ]);
}

#[test]
fn sha2_256() {
    sha2(HashTests::sha2_256(), "SHA2-256", |m| {
        libcrux_sha2::sha256(m).to_vec()
    });
}

#[test]
fn sha2_256_hasher() {
    sha2(HashTests::sha2_256(), "SHA2-256", hash_sha256);
}

#[test]
fn sha2_384() {
    sha2(HashTests::sha2_384(), "SHA2-384", |m| {
        libcrux_sha2::sha384(m).to_vec()
    });
}

#[test]
fn sha2_384_hasher() {
    sha2(HashTests::sha2_384(), "SHA2-384", hash_sha384);
}

/// SHA-2 LDT: hashing messages of 1 to 8 GiB.
///
/// The message is fed to the incremental hasher in chunks of the repeated content.
macro_rules! sha2_ldt {
    ($name:ident, $tests:expr, $hasher:ty) => {
        #[test]
        #[ignore = "hashes up to 8 GiB per test case"]
        fn $name() {
            let tests = $tests;
            let mut tested = 0;
            for group in tests
                .prompts
                .testGroups
                .iter()
                .filter(|g| g.testType == "LDT")
            {
                for test in &group.tests {
                    let mut hasher = <$hasher>::new();
                    test.largeMsg
                        .as_ref()
                        .unwrap()
                        .for_each_chunk(|chunk| hasher.update(chunk).unwrap());

                    let expected = tests.results.find_expected_result(group.tgId, test.tcId);
                    assert_eq!(
                        Some(hasher.finish_to_owned().to_vec()),
                        expected.md,
                        "tcId {}",
                        test.tcId
                    );
                    tested += 1;
                }
            }
            assert_eq!(tested, 4);
        }
    };
}

sha2_ldt!(
    sha2_256_ldt,
    HashTests::sha2_256(),
    libcrux_sha2::Sha256Hasher
);
sha2_ldt!(
    sha2_384_ldt,
    HashTests::sha2_384(),
    libcrux_sha2::Sha384Hasher
);

fn hash_sha224(msg: &[u8]) -> Vec<u8> {
    let mut hasher = libcrux_sha2::Sha224Hasher::new();
    hasher.update(msg).unwrap();
    hasher.finish_to_owned().to_vec()
}

fn hash_sha512(msg: &[u8]) -> Vec<u8> {
    let mut hasher = libcrux_sha2::Sha512Hasher::new();
    hasher.update(msg).unwrap();
    hasher.finish_to_owned().to_vec()
}

#[test]
fn sha2_224() {
    sha2(HashTests::sha2_224(), "SHA2-224", |m| {
        libcrux_sha2::sha224(m).to_vec()
    });
}

#[test]
fn sha2_224_hasher() {
    sha2(HashTests::sha2_224(), "SHA2-224", hash_sha224);
}

#[test]
fn sha2_512() {
    sha2(HashTests::sha2_512(), "SHA2-512", |m| {
        libcrux_sha2::sha512(m).to_vec()
    });
}

#[test]
fn sha2_512_hasher() {
    sha2(HashTests::sha2_512(), "SHA2-512", hash_sha512);
}

sha2_ldt!(
    sha2_224_ldt,
    HashTests::sha2_224(),
    libcrux_sha2::Sha224Hasher
);
sha2_ldt!(
    sha2_512_ldt,
    HashTests::sha2_512(),
    libcrux_sha2::Sha512Hasher
);

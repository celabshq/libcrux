use libcrux_kats::acvp::{
    hash::{sha3_mct, shake_mct},
    schema_common::{assert_covers, Tally},
    sha3::HashTests,
};

/// `hash(digest, msg)` for the output length `digest.len()`.
type HashFn = fn(&mut [u8], &[u8]);

fn hash_sha3_256(digest: &mut [u8], msg: &[u8]) {
    let mut hasher = libcrux_sha3::Sha3_256Hasher::new();
    hasher.update(msg).unwrap();
    digest.copy_from_slice(&hasher.finish_to_owned());
}

fn hash_sha3_384(digest: &mut [u8], msg: &[u8]) {
    let mut hasher = libcrux_sha3::Sha3_384Hasher::new();
    hasher.update(msg).unwrap();
    digest.copy_from_slice(&hasher.finish_to_owned());
}

fn hash_sha3_512(digest: &mut [u8], msg: &[u8]) {
    let mut hasher = libcrux_sha3::Sha3_512Hasher::new();
    hasher.update(msg).unwrap();
    digest.copy_from_slice(&hasher.finish_to_owned());
}

#[cfg(feature = "simd128")]
fn neon_shake256(digest: &mut [u8], msg: &[u8]) {
    let mut dummy = vec![0u8; digest.len()];
    libcrux_sha3::neon::x2::shake256(msg, msg, digest, &mut dummy);
}

#[cfg(feature = "simd256")]
fn avx2_shake256(digest: &mut [u8], msg: &[u8]) {
    let mut dummy = [
        vec![0u8; digest.len()],
        vec![0u8; digest.len()],
        vec![0u8; digest.len()],
    ];
    let [d1, d2, d3] = &mut dummy;
    libcrux_sha3::avx2::x4::shake256(msg, msg, msg, msg, digest, d1, d2, d3);
}

/// SHA-3 AFT and MCT. The LDT is run separately in [`sha3_ldt`].
fn sha3(tests: HashTests, algorithm: &str, digest_len: usize, hash: HashFn) {
    assert_eq!(tests.prompts.algorithm, algorithm);
    assert_eq!(tests.prompts.revision, "2.0");
    assert_covers(&tests.prompts, &tests.results);

    let hash = |msg: &[u8]| {
        let mut md = vec![0u8; digest_len];
        hash(&mut md, msg);
        md
    };

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
                        sha3_mct(&test.msg, hash),
                        expected.resultsArray,
                        "tcId {}",
                        test.tcId
                    );
                }
                "LDT" => {
                    tally.add("LDT (skipped, see sha3_ldt)");
                    continue;
                }
                t => panic!("unexpected testType {t}"),
            }
            tally.add(&group.testType);
        }
    }
    let aft = match algorithm {
        "SHA3-224" => 246,
        "SHA3-256" => 238,
        "SHA3-384" => 206,
        "SHA3-512" => 174,
        a => panic!("unexpected algorithm {a}"),
    };
    tally.assert_counts([("AFT", aft), ("MCT", 1), ("LDT (skipped, see sha3_ldt)", 4)]);
}

/// SHAKE AFT, VOT and MCT.
fn shake(tests: HashTests, algorithm: &str, xof: HashFn) {
    assert_eq!(tests.prompts.algorithm, algorithm);
    assert_eq!(tests.prompts.revision, "1.0");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            match group.testType.as_str() {
                "AFT" | "VOT" => {
                    assert_eq!(test.len, test.msg.len() * 8);
                    let out_len = test.outLen.unwrap();
                    assert_eq!(out_len % 8, 0);
                    let mut md = vec![0u8; out_len / 8];
                    xof(&mut md, &test.msg);
                    assert_eq!(Some(md), expected.md, "tcId {}", test.tcId);
                    assert_eq!(Some(out_len), expected.outLen, "tcId {}", test.tcId);
                }
                "MCT" => {
                    assert_eq!(group.mctVersion.as_deref(), Some("standard"));
                    assert_eq!(
                        shake_mct(
                            &test.msg,
                            group.minOutLen.unwrap(),
                            group.maxOutLen.unwrap(),
                            xof
                        ),
                        expected.resultsArray,
                        "tcId {}",
                        test.tcId
                    );
                }
                t => panic!("unexpected testType {t}"),
            }
            tally.add(&group.testType);
        }
    }
    let aft = match algorithm {
        "SHAKE-128" => 216,
        "SHAKE-256" => 195,
        a => panic!("unexpected algorithm {a}"),
    };
    tally.assert_counts([("AFT", aft), ("VOT", 512), ("MCT", 1)]);
}

#[test]
fn sha3_256() {
    sha3(HashTests::sha3_256(), "SHA3-256", 32, |d, m| {
        libcrux_sha3::sha256_ema(d, m)
    });
}

#[test]
fn sha3_256_portable() {
    sha3(
        HashTests::sha3_256(),
        "SHA3-256",
        32,
        libcrux_sha3::portable::sha256,
    );
}

#[test]
fn sha3_256_hasher() {
    sha3(HashTests::sha3_256(), "SHA3-256", 32, hash_sha3_256);
}

#[cfg(feature = "simd128")]
#[test]
fn sha3_256_neon() {
    sha3(
        HashTests::sha3_256(),
        "SHA3-256",
        32,
        libcrux_sha3::neon::sha256,
    );
}

#[test]
fn sha3_384() {
    sha3(HashTests::sha3_384(), "SHA3-384", 48, |d, m| {
        libcrux_sha3::sha384_ema(d, m)
    });
}

#[test]
fn sha3_384_portable() {
    sha3(
        HashTests::sha3_384(),
        "SHA3-384",
        48,
        libcrux_sha3::portable::sha384,
    );
}

#[test]
fn sha3_384_hasher() {
    sha3(HashTests::sha3_384(), "SHA3-384", 48, hash_sha3_384);
}

#[cfg(feature = "simd128")]
#[test]
fn sha3_384_neon() {
    sha3(
        HashTests::sha3_384(),
        "SHA3-384",
        48,
        libcrux_sha3::neon::sha384,
    );
}

#[test]
fn sha3_512() {
    sha3(HashTests::sha3_512(), "SHA3-512", 64, |d, m| {
        libcrux_sha3::sha512_ema(d, m)
    });
}

#[test]
fn sha3_512_portable() {
    sha3(
        HashTests::sha3_512(),
        "SHA3-512",
        64,
        libcrux_sha3::portable::sha512,
    );
}

#[test]
fn sha3_512_hasher() {
    sha3(HashTests::sha3_512(), "SHA3-512", 64, hash_sha3_512);
}

#[cfg(feature = "simd128")]
#[test]
fn sha3_512_neon() {
    sha3(
        HashTests::sha3_512(),
        "SHA3-512",
        64,
        libcrux_sha3::neon::sha512,
    );
}

#[test]
fn shake128() {
    shake(
        HashTests::shake128(),
        "SHAKE-128",
        libcrux_sha3::shake128_ema,
    );
}

#[test]
fn shake128_portable() {
    shake(
        HashTests::shake128(),
        "SHAKE-128",
        libcrux_sha3::portable::shake128,
    );
}

#[test]
fn shake256() {
    shake(
        HashTests::shake256(),
        "SHAKE-256",
        libcrux_sha3::shake256_ema,
    );
}

#[test]
fn shake256_portable() {
    shake(
        HashTests::shake256(),
        "SHAKE-256",
        libcrux_sha3::portable::shake256,
    );
}

#[cfg(feature = "simd128")]
#[test]
fn shake256_neon() {
    shake(HashTests::shake256(), "SHAKE-256", neon_shake256);
}

#[cfg(feature = "simd256")]
#[test]
fn shake256_avx2() {
    shake(HashTests::shake256(), "SHAKE-256", avx2_shake256);
}

/// SHA-3 LDT: hashing messages of 1 to 8 GiB.
///
/// The message is fed to the incremental hasher in chunks of the repeated content.
macro_rules! sha3_ldt {
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

sha3_ldt!(
    sha3_256_ldt,
    HashTests::sha3_256(),
    libcrux_sha3::Sha3_256Hasher
);
sha3_ldt!(
    sha3_384_ldt,
    HashTests::sha3_384(),
    libcrux_sha3::Sha3_384Hasher
);
sha3_ldt!(
    sha3_512_ldt,
    HashTests::sha3_512(),
    libcrux_sha3::Sha3_512Hasher
);

fn hash_sha3_224(digest: &mut [u8], msg: &[u8]) {
    let mut hasher = libcrux_sha3::Sha3_224Hasher::new();
    hasher.update(msg).unwrap();
    digest.copy_from_slice(&hasher.finish_to_owned());
}

#[test]
fn sha3_224() {
    sha3(HashTests::sha3_224(), "SHA3-224", 28, |d, m| {
        libcrux_sha3::sha224_ema(d, m)
    });
}

#[test]
fn sha3_224_portable() {
    sha3(
        HashTests::sha3_224(),
        "SHA3-224",
        28,
        libcrux_sha3::portable::sha224,
    );
}

#[test]
fn sha3_224_hasher() {
    sha3(HashTests::sha3_224(), "SHA3-224", 28, hash_sha3_224);
}

#[cfg(feature = "simd128")]
#[test]
fn sha3_224_neon() {
    sha3(
        HashTests::sha3_224(),
        "SHA3-224",
        28,
        libcrux_sha3::neon::sha224,
    );
}

sha3_ldt!(
    sha3_224_ldt,
    HashTests::sha3_224(),
    libcrux_sha3::Sha3_224Hasher
);

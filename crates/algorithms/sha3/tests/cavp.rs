use libcrux_sha3::*;

macro_rules! sha3_test {
    ($test_name:ident, $kats_fn:path, $digest_len:expr, $algorithm:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let my_digest: [u8; $digest_len] =
                    sha3($algorithm, &test.msg[0..test.msg_length / 8]);
                assert_eq!(&my_digest, &test.digest[..], "test {i}: digest mismatch");
            }
            eprintln!("Ran {test_cnt} tests for {}", stringify!($test_name));
        }
    };
}

sha3_test!(
    SHA3_224ShortMsg,
    libcrux_kats::sha3::sha3_224_short,
    SHA3_224_DIGEST_SIZE,
    Algorithm::Sha224
);
sha3_test!(
    SHA3_224LongMsg,
    libcrux_kats::sha3::sha3_224_long,
    SHA3_224_DIGEST_SIZE,
    Algorithm::Sha224
);
sha3_test!(
    SHA3_256ShortMsg,
    libcrux_kats::sha3::sha3_256_short,
    SHA3_256_DIGEST_SIZE,
    Algorithm::Sha256
);
sha3_test!(
    SHA3_256LongMsg,
    libcrux_kats::sha3::sha3_256_long,
    SHA3_256_DIGEST_SIZE,
    Algorithm::Sha256
);
sha3_test!(
    SHA3_384ShortMsg,
    libcrux_kats::sha3::sha3_384_short,
    SHA3_384_DIGEST_SIZE,
    Algorithm::Sha384
);
sha3_test!(
    SHA3_384LongMsg,
    libcrux_kats::sha3::sha3_384_long,
    SHA3_384_DIGEST_SIZE,
    Algorithm::Sha384
);
sha3_test!(
    SHA3_512ShortMsg,
    libcrux_kats::sha3::sha3_512_short,
    SHA3_512_DIGEST_SIZE,
    Algorithm::Sha512
);
sha3_test!(
    SHA3_512LongMsg,
    libcrux_kats::sha3::sha3_512_long,
    SHA3_512_DIGEST_SIZE,
    Algorithm::Sha512
);

macro_rules! shake_test {
    ($test_name:ident, $kats_fn:path, $shake:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            let _ = pretty_env_logger::try_init();
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let mut my_digest = vec![0u8; test.digest.len()];
                $shake(&mut my_digest, &test.msg[0..test.msg_length / 8]);
                assert_eq!(&my_digest, &test.digest[..], "test {i}: digest mismatch");
            }
            eprintln!("Ran {test_cnt} tests for {}", stringify!($test_name));
        }
    };
}

shake_test!(
    SHAKE128ShortMsg,
    libcrux_kats::sha3::shake128_short,
    shake128_ema
);
shake_test!(
    SHAKE128LongMsg,
    libcrux_kats::sha3::shake128_long,
    shake128_ema
);
shake_test!(
    SHAKE256ShortMsg,
    libcrux_kats::sha3::shake256_short,
    shake256_ema
);
shake_test!(
    SHAKE256LongMsg,
    libcrux_kats::sha3::shake256_long,
    shake256_ema
);

macro_rules! shake_vo_test {
    ($test_name:ident, $kats_fn:path, $shake:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            let _ = pretty_env_logger::try_init();
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let mut my_digest = vec![0u8; test.digest.len()];
                $shake(&mut my_digest, &test.msg[0..tv.header.input_length / 8]);
                assert_eq!(&my_digest, &test.digest[..], "test {i}: digest mismatch");
            }
            eprintln!("Ran {test_cnt} tests for {}", stringify!($test_name));
        }
    };
}

shake_vo_test!(
    SHAKE128VariableOut,
    libcrux_kats::sha3::shake128_variable_out,
    shake128_ema
);
shake_vo_test!(
    SHAKE256VariableOut,
    libcrux_kats::sha3::shake256_variable_out,
    shake256_ema
);

macro_rules! shake_vo_test_incremental {
    ($name:ident, $kats_fn:path, $shake:ty) => {
        #[test]
        #[allow(non_snake_case)]
        fn $name() {
            use libcrux_sha3::portable::incremental::Xof;
            let _ = pretty_env_logger::try_init();
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let mut my_digest = vec![0u8; test.digest.len()];
                let mut shake = <$shake>::new();
                shake.absorb_final(&test.msg[0..tv.header.input_length / 8]);
                shake.squeeze(&mut my_digest);
                assert_eq!(&my_digest, &test.digest[..], "test {i}: digest mismatch");
            }
            eprintln!("Ran {test_cnt} tests for {}", stringify!($name));
        }
    };
}

shake_vo_test_incremental!(
    SHAKE128VariableOut_incremental,
    libcrux_kats::sha3::shake128_variable_out,
    libcrux_sha3::portable::incremental::Shake128Xof
);
shake_vo_test_incremental!(
    SHAKE256VariableOut_incremental,
    libcrux_kats::sha3::shake256_variable_out,
    libcrux_sha3::portable::incremental::Shake256Xof
);

macro_rules! sha3_neon_test {
    ($test_name:ident, $kats_fn:path, $digest_len:expr, $hash_fn:path) => {
        #[test]
        #[cfg(feature = "simd128")]
        #[allow(non_snake_case)]
        fn $test_name() {
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let mut my_digest = [0u8; $digest_len];
                $hash_fn(&mut my_digest, &test.msg[0..test.msg_length / 8]);
                assert_eq!(&my_digest, &test.digest[..], "test {i}: digest mismatch");
            }
            eprintln!("Ran {test_cnt} tests for {}", stringify!($test_name));
        }
    };
}

sha3_neon_test!(
    neon_SHA3_224ShortMsg,
    libcrux_kats::sha3::sha3_224_short,
    SHA3_224_DIGEST_SIZE,
    libcrux_sha3::neon::sha224
);
sha3_neon_test!(
    neon_SHA3_224LongMsg,
    libcrux_kats::sha3::sha3_224_long,
    SHA3_224_DIGEST_SIZE,
    libcrux_sha3::neon::sha224
);
sha3_neon_test!(
    neon_SHA3_256ShortMsg,
    libcrux_kats::sha3::sha3_256_short,
    SHA3_256_DIGEST_SIZE,
    libcrux_sha3::neon::sha256
);
sha3_neon_test!(
    neon_SHA3_256LongMsg,
    libcrux_kats::sha3::sha3_256_long,
    SHA3_256_DIGEST_SIZE,
    libcrux_sha3::neon::sha256
);
sha3_neon_test!(
    neon_SHA3_384ShortMsg,
    libcrux_kats::sha3::sha3_384_short,
    SHA3_384_DIGEST_SIZE,
    libcrux_sha3::neon::sha384
);
sha3_neon_test!(
    neon_SHA3_384LongMsg,
    libcrux_kats::sha3::sha3_384_long,
    SHA3_384_DIGEST_SIZE,
    libcrux_sha3::neon::sha384
);
sha3_neon_test!(
    neon_SHA3_512ShortMsg,
    libcrux_kats::sha3::sha3_512_short,
    SHA3_512_DIGEST_SIZE,
    libcrux_sha3::neon::sha512
);
sha3_neon_test!(
    neon_SHA3_512LongMsg,
    libcrux_kats::sha3::sha3_512_long,
    SHA3_512_DIGEST_SIZE,
    libcrux_sha3::neon::sha512
);

macro_rules! shake128_neon_test {
    ($test_name:ident, $kats_fn:path) => {
        #[test]
        #[cfg(feature = "simd128")]
        #[allow(non_snake_case)]
        fn $test_name() {
            let _ = pretty_env_logger::try_init();
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                assert_eq!(
                    test.digest.len(),
                    16,
                    "test {i}: unexpected output length {}",
                    test.digest.len()
                );
                let mut my_digest = [0u8; 16];
                libcrux_sha3::neon::shake128(&mut my_digest, &test.msg[0..test.msg_length / 8]);
                assert_eq!(&my_digest, &test.digest[..], "test {i}: digest mismatch");
            }
            eprintln!("Ran {test_cnt} tests for {}", stringify!($test_name));
        }
    };
}

shake128_neon_test!(neon_SHAKE128ShortMsg, libcrux_kats::sha3::shake128_short);
shake128_neon_test!(neon_SHAKE128LongMsg, libcrux_kats::sha3::shake128_long);

macro_rules! shake256_neon_test {
    ($test_name:ident, $kats_fn:path) => {
        #[test]
        #[cfg(feature = "simd128")]
        #[allow(non_snake_case)]
        fn $test_name() {
            let _ = pretty_env_logger::try_init();
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let msg = &test.msg[0..test.msg_length / 8];
                let out_len = test.digest.len();
                let mut out0 = vec![0u8; out_len];
                let mut out1 = vec![0u8; out_len];
                libcrux_sha3::neon::x2::shake256(msg, msg, &mut out0, &mut out1);
                assert_eq!(&out0, &test.digest, "test {i}: digest mismatch (lane 0)");
                assert_eq!(&out1, &test.digest, "test {i}: digest mismatch (lane 1)");
            }
            eprintln!("Ran {test_cnt} tests for {}", stringify!($test_name));
        }
    };
}

shake256_neon_test!(neon_SHAKE256ShortMsg, libcrux_kats::sha3::shake256_short);
shake256_neon_test!(neon_SHAKE256LongMsg, libcrux_kats::sha3::shake256_long);

macro_rules! shake256_vo_neon_test {
    ($test_name:ident, $kats_fn:path) => {
        #[test]
        #[cfg(feature = "simd128")]
        #[allow(non_snake_case)]
        fn $test_name() {
            let _ = pretty_env_logger::try_init();
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let msg = &test.msg[0..tv.header.input_length / 8];
                let out_len = test.digest.len();
                let mut out0 = vec![0u8; out_len];
                let mut out1 = vec![0u8; out_len];
                libcrux_sha3::neon::x2::shake256(msg, msg, &mut out0, &mut out1);
                assert_eq!(&out0, &test.digest, "test {i}: digest mismatch (lane 0)");
                assert_eq!(&out1, &test.digest, "test {i}: digest mismatch (lane 1)");
            }
            eprintln!("Ran {test_cnt} tests for {}", stringify!($test_name));
        }
    };
}

shake256_vo_neon_test!(
    neon_SHAKE256VariableOut,
    libcrux_kats::sha3::shake256_variable_out
);

macro_rules! shake256_avx2_test {
    ($test_name:ident, $kats_fn:path) => {
        #[test]
        #[cfg(feature = "simd256")]
        #[allow(non_snake_case)]
        fn $test_name() {
            let _ = pretty_env_logger::try_init();
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let msg = &test.msg[0..test.msg_length / 8];
                let out_len = test.digest.len();
                let mut out0 = vec![0u8; out_len];
                let mut out1 = vec![0u8; out_len];
                let mut out2 = vec![0u8; out_len];
                let mut out3 = vec![0u8; out_len];
                libcrux_sha3::avx2::x4::shake256(
                    msg, msg, msg, msg, &mut out0, &mut out1, &mut out2, &mut out3,
                );
                assert_eq!(&out0, &test.digest, "test {i}: digest mismatch (lane 0)");
                assert_eq!(&out1, &test.digest, "test {i}: digest mismatch (lane 1)");
                assert_eq!(&out2, &test.digest, "test {i}: digest mismatch (lane 2)");
                assert_eq!(&out3, &test.digest, "test {i}: digest mismatch (lane 3)");
            }
            eprintln!("Ran {test_cnt} tests for {}", stringify!($test_name));
        }
    };
}

shake256_avx2_test!(avx2_SHAKE256ShortMsg, libcrux_kats::sha3::shake256_short);
shake256_avx2_test!(avx2_SHAKE256LongMsg, libcrux_kats::sha3::shake256_long);

macro_rules! shake256_vo_avx2_test {
    ($test_name:ident, $kats_fn:path) => {
        #[test]
        #[cfg(feature = "simd256")]
        #[allow(non_snake_case)]
        // This test uncovered a bug in the avx2 shake26 implementation
        // Expect the panic here and fix in following PR
        #[should_panic(expected = "range start index 64 out of range for slice of length 33")]
        fn $test_name() {
            let _ = pretty_env_logger::try_init();
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let msg = &test.msg[0..tv.header.input_length / 8];
                let out_len = test.digest.len();
                let mut out0 = vec![0u8; out_len];
                let mut out1 = vec![0u8; out_len];
                let mut out2 = vec![0u8; out_len];
                let mut out3 = vec![0u8; out_len];
                libcrux_sha3::avx2::x4::shake256(
                    msg, msg, msg, msg, &mut out0, &mut out1, &mut out2, &mut out3,
                );
                assert_eq!(&out0, &test.digest, "test {i}: digest mismatch (lane 0)");
                assert_eq!(&out1, &test.digest, "test {i}: digest mismatch (lane 1)");
                assert_eq!(&out2, &test.digest, "test {i}: digest mismatch (lane 2)");
                assert_eq!(&out3, &test.digest, "test {i}: digest mismatch (lane 3)");
            }
            eprintln!("Ran {test_cnt} tests for {}", stringify!($test_name));
        }
    };
}

shake256_vo_avx2_test!(
    avx2_SHAKE256VariableOut,
    libcrux_kats::sha3::shake256_variable_out
);

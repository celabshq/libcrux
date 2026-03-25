use cavp::*;
use libcrux_sha3::*;

macro_rules! sha3_test {
    ($file:ident, $digest_len:expr, $algorithm:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $file() {
            let file = "tests/tv/".to_string();
            let file = file + stringify!($file) + ".rsp";
            let tv: TestVector<cavp::Sha3> = read_file(&file).unwrap();

            let mut c = 0;
            for test in &tv.tests {
                eprintln!("test {c}");
                c += 1;
                let my_digest: [u8; $digest_len] =
                    sha3($algorithm, &test.msg[0..test.msg_length / 8]);
                assert_eq!(&my_digest, &test.digest[..]);
            }
        }
    };
}

sha3_test!(SHA3_224LongMsg, SHA3_224_DIGEST_SIZE, Algorithm::Sha224);
sha3_test!(SHA3_256LongMsg, SHA3_256_DIGEST_SIZE, Algorithm::Sha256);
sha3_test!(SHA3_384LongMsg, SHA3_384_DIGEST_SIZE, Algorithm::Sha384);
sha3_test!(SHA3_512LongMsg, SHA3_512_DIGEST_SIZE, Algorithm::Sha512);

sha3_test!(SHA3_224ShortMsg, SHA3_224_DIGEST_SIZE, Algorithm::Sha224);
sha3_test!(SHA3_256ShortMsg, SHA3_256_DIGEST_SIZE, Algorithm::Sha256);
sha3_test!(SHA3_384ShortMsg, SHA3_384_DIGEST_SIZE, Algorithm::Sha384);
sha3_test!(SHA3_512ShortMsg, SHA3_512_DIGEST_SIZE, Algorithm::Sha512);

macro_rules! shake_test {
    ($file:ident, $shake:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $file() {
            let _ = pretty_env_logger::try_init();
            let file = "tests/tv/".to_string();
            let file = file + stringify!($file) + ".rsp";
            let tv: TestVector<cavp::ShakeMsg> = read_file(&file).unwrap();

            let mut c = 0;
            for test in &tv.tests {
                eprintln!("test {c}");
                c += 1;
                let mut my_digest = vec![0u8; test.digest.len()];
                $shake(&mut my_digest, &test.msg[0..test.msg_length / 8]);
                assert_eq!(&my_digest, &test.digest[..]);
            }
        }
    };
}

shake_test!(SHAKE128LongMsg, shake128_ema);
shake_test!(SHAKE128ShortMsg, shake128_ema);

shake_test!(SHAKE256LongMsg, shake256_ema);
shake_test!(SHAKE256ShortMsg, shake256_ema);

macro_rules! shake_vo_test {
    ($file:ident, $shake:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $file() {
            let _ = pretty_env_logger::try_init();
            let file = "tests/tv/".to_string();
            let file = file + stringify!($file) + ".rsp";
            let tv: TestVector<cavp::ShakeVariableOut> = read_file(&file).unwrap();

            let mut c = 0;
            for test in &tv.tests {
                eprintln!("test {c}");
                c += 1;
                let mut my_digest = vec![0u8; test.digest.len()];
                $shake(&mut my_digest, &test.msg[0..tv.header.input_length / 8]);
                assert_eq!(&my_digest, &test.digest[..]);
            }
        }
    };
}
shake_vo_test!(SHAKE128VariableOut, shake128_ema);
shake_vo_test!(SHAKE256VariableOut, shake256_ema);

macro_rules! shake_vo_test_incremental {
    ($name:ident, $file:ident, $shake:ty) => {
        #[test]
        #[allow(non_snake_case)]
        fn $name() {
            use libcrux_sha3::portable::incremental::Xof;
            let _ = pretty_env_logger::try_init();
            let file = "tests/tv/".to_string();
            let file = file + stringify!($file) + ".rsp";
            let tv: TestVector<cavp::ShakeVariableOut> = read_file(&file).unwrap();

            let mut c = 0;
            for test in &tv.tests {
                eprintln!("test {c}");
                c += 1;
                let mut my_digest = vec![0u8; test.digest.len()];
                let mut shake = <$shake>::new();
                shake.absorb_final(&test.msg[0..tv.header.input_length / 8]);
                shake.squeeze(&mut my_digest);
                assert_eq!(&my_digest, &test.digest[..]);
            }
        }
    };
}
shake_vo_test_incremental!(
    SHAKE128VariableOut_incremental,
    SHAKE128VariableOut,
    libcrux_sha3::portable::incremental::Shake128Xof
);
shake_vo_test_incremental!(
    SHAKE256VariableOut_incremental,
    SHAKE256VariableOut,
    libcrux_sha3::portable::incremental::Shake256Xof
);

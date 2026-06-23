/// CAVP (Cryptographic Algorithm Validation Program) tests.
/// Ported from ../../crates/algorithms/sha3/tests/cavp.rs
use hacspec_sha3::*;

macro_rules! sha3_test {
    ($name:ident, $kats_fn:path, $hash_fn:ident) => {
        #[test]
        #[allow(non_snake_case)]
        fn $name() {
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let digest = $hash_fn(&test.msg[0..test.msg_length / 8]);
                assert_eq!(&digest[..], &test.digest[..], "test {i}: digest mismatch");
            }
        }
    };
}

sha3_test!(
    SHA3_224ShortMsg,
    libcrux_kats::sha3::sha3_224_short,
    sha3_224
);
sha3_test!(SHA3_224LongMsg, libcrux_kats::sha3::sha3_224_long, sha3_224);
sha3_test!(
    SHA3_256ShortMsg,
    libcrux_kats::sha3::sha3_256_short,
    sha3_256
);
sha3_test!(SHA3_256LongMsg, libcrux_kats::sha3::sha3_256_long, sha3_256);
sha3_test!(
    SHA3_384ShortMsg,
    libcrux_kats::sha3::sha3_384_short,
    sha3_384
);
sha3_test!(SHA3_384LongMsg, libcrux_kats::sha3::sha3_384_long, sha3_384);
sha3_test!(
    SHA3_512ShortMsg,
    libcrux_kats::sha3::sha3_512_short,
    sha3_512
);
sha3_test!(SHA3_512LongMsg, libcrux_kats::sha3::sha3_512_long, sha3_512);

// SHAKE fixed-output tests
macro_rules! shake_test {
    ($name:ident, $kats_fn:path, $shake_fn:ident, $out_len:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $name() {
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let digest = $shake_fn::<$out_len>(&test.msg[0..test.msg_length / 8]);
                assert_eq!(&digest[..], &test.digest[..], "test {i}: digest mismatch");
            }
        }
    };
}

shake_test!(
    SHAKE128ShortMsg,
    libcrux_kats::sha3::shake128_short,
    shake128,
    16
);
shake_test!(
    SHAKE128LongMsg,
    libcrux_kats::sha3::shake128_long,
    shake128,
    16
);
shake_test!(
    SHAKE256ShortMsg,
    libcrux_kats::sha3::shake256_short,
    shake256,
    32
);
shake_test!(
    SHAKE256LongMsg,
    libcrux_kats::sha3::shake256_long,
    shake256,
    32
);

// SHAKE variable-output tests
macro_rules! shake_vo_test {
    ($name:ident, $kats_fn:path, $shake_fn:ident, $max_out:expr) => {
        #[test]
        #[allow(non_snake_case)]
        fn $name() {
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let full_output = $shake_fn::<$max_out>(&test.msg[0..tv.header.input_length / 8]);
                let expected_len = test.digest.len();
                assert_eq!(
                    &full_output[..expected_len],
                    &test.digest[..],
                    "test {i}: digest mismatch (output_len={expected_len} bytes)",
                );
            }
        }
    };
}

shake_vo_test!(
    SHAKE128VariableOut,
    libcrux_kats::sha3::shake128_variable_out,
    shake128,
    140
);
shake_vo_test!(
    SHAKE256VariableOut,
    libcrux_kats::sha3::shake256_variable_out,
    shake256,
    250
);

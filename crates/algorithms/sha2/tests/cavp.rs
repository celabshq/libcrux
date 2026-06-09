macro_rules! sha2_test {
    ($test_name:ident, $kats_fn:path, $hash_fn:path) => {
        #[test]
        #[allow(non_snake_case)]
        fn $test_name() {
            let tv = $kats_fn();
            let test_cnt = tv.tests.len();
            assert!(test_cnt > 0, "Empty test vector file");
            for (i, test) in tv.tests.iter().enumerate() {
                let digest = $hash_fn(&test.msg[0..test.msg_length / 8]);
                assert_eq!(&digest[..], &test.digest[..], "test {i}: digest mismatch");
            }
            eprintln!("Ran {test_cnt} tests for {}", stringify!($test_name));
        }
    };
}

sha2_test!(
    SHA224ShortMsg,
    libcrux_kats::sha2::sha224_short,
    libcrux_sha2::sha224
);
sha2_test!(
    SHA224LongMsg,
    libcrux_kats::sha2::sha224_long,
    libcrux_sha2::sha224
);

sha2_test!(
    SHA256ShortMsg,
    libcrux_kats::sha2::sha256_short,
    libcrux_sha2::sha256
);
sha2_test!(
    SHA256LongMsg,
    libcrux_kats::sha2::sha256_long,
    libcrux_sha2::sha256
);

sha2_test!(
    SHA384ShortMsg,
    libcrux_kats::sha2::sha384_short,
    libcrux_sha2::sha384
);
sha2_test!(
    SHA384LongMsg,
    libcrux_kats::sha2::sha384_long,
    libcrux_sha2::sha384
);

sha2_test!(
    SHA512ShortMsg,
    libcrux_kats::sha2::sha512_short,
    libcrux_sha2::sha512
);
sha2_test!(
    SHA512LongMsg,
    libcrux_kats::sha2::sha512_long,
    libcrux_sha2::sha512
);

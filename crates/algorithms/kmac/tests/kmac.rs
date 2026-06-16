#[test]
fn run_wycheproof() {
    use libcrux_kats::wycheproof::kmac::{Algorithm, KmacResult, KmacTests};

    for (algorithm, kmac) in [
        (
            Algorithm::Kmac128,
            libcrux_kmac::kmac_128 as for<'a> fn(&'a mut [u8], &[u8], &[u8], &[u8]) -> &'a [u8],
        ),
        (
            Algorithm::Kmac256,
            libcrux_kmac::kmac_256 as for<'a> fn(&'a mut [u8], &[u8], &[u8], &[u8]) -> &'a [u8],
        ),
    ] {
        let test_set = KmacTests::load(algorithm);

        println!("Test Set {}", test_set.algorithm);
        for test_group in &test_set.test_groups {
            let tag_size = test_group.tag_size;
            for (i, test) in test_group.tests.iter().enumerate() {
                let comment = &test.comment;
                println!("Test {i}: {comment}");
                let mut tag = vec![0u8; (tag_size / 8) as usize];
                let result = kmac(&mut tag, &test.key, &test.msg, &[]);

                match test.result {
                    KmacResult::Valid => assert_eq!(result, test.tag.as_slice()),
                    KmacResult::Invalid => assert_ne!(result, test.tag.as_slice()),
                    KmacResult::Acceptable => unreachable!(),
                }
            }
        }
    }
}

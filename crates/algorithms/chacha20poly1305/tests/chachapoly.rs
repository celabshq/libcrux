use libcrux_chacha20poly1305::{AeadError, KEY_LEN, TAG_LEN};
use libcrux_kats::wycheproof::{aead, TestResult};
use rand_core::UnwrapErr;

fn randbuf<const N: usize>(rng: &mut impl rand_core::Rng) -> [u8; N] {
    let mut buf = [0; N];
    rng.fill_bytes(&mut buf);
    buf
}

/// Generic function to instantiate wycheproof tests for either
/// chacha20poly1305 or xchacha20poly1305
fn wycheproof_test<EncF, DecF, const NONCE_LEN: usize>(
    test_set: aead::TestSet,
    encrypt: EncF,
    decrypt: DecF,
) where
    EncF: for<'a> Fn(
        &[u8; KEY_LEN],
        &[u8],
        &'a mut [u8],
        &[u8],
        &[u8; NONCE_LEN],
    ) -> Result<(&'a [u8], &'a [u8; TAG_LEN]), AeadError>,
    DecF: for<'a> Fn(
        &[u8; KEY_LEN],
        &'a mut [u8],
        &[u8],
        &[u8],
        &[u8; NONCE_LEN],
    ) -> Result<&'a [u8], AeadError>,
{
    let mut tests_run = 0;

    for test_group in test_set.test_groups {
        for test in &test_group.tests {
            let Ok(nonce) = <&[u8; NONCE_LEN]>::try_from(test.nonce.as_slice()) else {
                assert_eq!(TestResult::Invalid, test.result, "tc_id {}", test.tc_id);
                assert!(
                    test.flags.contains(&"InvalidNonceSize".to_owned()),
                    "tc_id: {}",
                    test.tc_id
                );
                tests_run += 1;
                continue;
            };
            let Ok(key) = <&[u8; 32]>::try_from(test.key.as_slice()) else {
                panic!("tc_id: {} has invalid key size", test.tc_id)
            };

            match test.result {
                TestResult::Valid => {
                    let mut ctxt_tag = vec![0; test.ct.len() + TAG_LEN];
                    // check that we encrypt to the expected ciphertext
                    let (ctxt, tag) = match encrypt(key, &test.pt, &mut ctxt_tag, &test.aad, nonce)
                    {
                        Ok((c, t)) => (c, t),
                        Err(err) => {
                            panic!(
                                "Unexpected encryption failure in tc_id {}: {err}",
                                test.tc_id
                            );
                        }
                    };
                    assert_eq!(
                        test.ct, ctxt,
                        "ciphertext mismatch tc_id {}: {}",
                        test.tc_id, test.comment
                    );
                    assert_eq!(
                        test.tag, tag,
                        "tag mismatch tc_id {}: {}",
                        test.tc_id, test.comment
                    );

                    // check that we can decrypt to the original plaintext
                    let mut decrypted = vec![0; test.pt.len()];
                    match decrypt(key, &mut decrypted, &ctxt_tag, &test.aad, nonce) {
                        Ok(ptxt) => {
                            assert_eq!(test.pt, ptxt);
                            assert_eq!(test.pt, decrypted);
                        }
                        Err(err) => {
                            panic!(
                                "Unexpected decryption failure in tc_id {}: {err}",
                                test.tc_id
                            );
                        }
                    };
                }
                TestResult::Invalid => {
                    // Invalid test cases have either wrong Nonce sizes (caught above),
                    // or an invalid ciphertext+tag
                    let mut decrypted = vec![0; test.pt.len()];
                    let invalid_ct_and_tag = [&test.ct[..], &test.tag[..]].concat();
                    match decrypt(key, &mut decrypted, &invalid_ct_and_tag, &test.aad, nonce) {
                        Ok(ptxt) => {
                            panic!(
                                "Unexpected decryption success in tc_id {}: {}. Decrypted plaintext: {:?}",
                                test.tc_id, test.comment, ptxt
                            );
                        }
                        Err(err) => {
                            assert!(
                                matches!(err, AeadError::InvalidCiphertext),
                                "decryption failed with wrong error"
                            );
                        }
                    };
                }
                TestResult::Acceptable => unreachable!("not present in chacha test vectors"),
            }
            tests_run += 1;
        }
    }

    assert_eq!(test_set.number_of_tests, tests_run);
    println!("Ran {tests_run} ChaCha20-Poly1305 tests");
}

#[test]
fn wycheproof_chacha20_poly1305() {
    let test_set = aead::TestSet::load_chacha20_poly1305();
    wycheproof_test(
        test_set,
        libcrux_chacha20poly1305::encrypt,
        libcrux_chacha20poly1305::decrypt,
    );
}

#[test]
fn wycheproof_xchacha20_poly1305() {
    let test_set = aead::TestSet::load_xchacha20_poly1305();
    wycheproof_test(
        test_set,
        libcrux_chacha20poly1305::xchacha20_poly1305::encrypt,
        libcrux_chacha20poly1305::xchacha20_poly1305::decrypt,
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn chachapoly_self_test() {
    let ptxt = b"hacspec rulez";
    let aad = b"associated data" as &[u8];
    let key = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ];
    let nonce = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    let mut ctxt = [0; 29];

    libcrux_chacha20poly1305::encrypt(&key, ptxt, &mut ctxt, aad, &nonce).unwrap();

    let mut ptxt_rx = [0; 13];

    assert!(libcrux_chacha20poly1305::decrypt(&key, &mut ptxt_rx, &ctxt, aad, &nonce).is_ok());
    assert_eq!(ptxt, &ptxt_rx);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[test]
fn chachapoly_self_test_rand() {
    let msg = b"hacspec rulez";
    let aad = b"associated data" as &[u8];

    let mut rng = UnwrapErr(rand::rngs::SysRng);

    let key: [u8; 32] = randbuf(&mut rng);
    let nonce: [u8; 12] = randbuf(&mut rng);

    let mut ctxt = [0; 29];
    let mut ptxt = [0; 13];

    libcrux_chacha20poly1305::encrypt(&key, msg, &mut ctxt, aad, &nonce).unwrap();
    assert!(libcrux_chacha20poly1305::decrypt(&key, &mut ptxt, &ctxt, aad, &nonce).is_ok());

    assert_eq!(msg, &ptxt);
}

#[test]
fn chachapoly_test_invalid_buffer_lengths() {
    let msg = b"hacspec rulez";
    let aad = b"associated data" as &[u8];

    let mut rng = UnwrapErr(rand::rngs::SysRng);

    let key: [u8; 32] = randbuf(&mut rng);
    let nonce: [u8; 12] = randbuf(&mut rng);

    // test that calling with incorrect-length buffers returns error: non-detached
    let err = libcrux_chacha20poly1305::encrypt(&key, msg, &mut [], aad, &nonce).unwrap_err();
    assert!(matches!(
        err,
        libcrux_chacha20poly1305::AeadError::CiphertextTooShort
    ));

    // test that calling with incorrect-length buffers returns error: detached
    let err =
        libcrux_chacha20poly1305::encrypt_detached(&key, msg, &mut [], &mut [0; 16], aad, &nonce)
            .unwrap_err();
    assert!(matches!(
        err,
        libcrux_chacha20poly1305::AeadError::CiphertextTooShort
    ));

    // encrypt correctly
    let mut ctxt = [0; 29];
    libcrux_chacha20poly1305::encrypt(&key, msg, &mut ctxt, aad, &nonce).unwrap();

    // encrypt correctly on ciphertext buffer longer than `ptxt.len() + TAG_LEN`
    let mut long_ctxt = [0; 30];
    libcrux_chacha20poly1305::encrypt(&key, msg, &mut long_ctxt, aad, &nonce).unwrap();

    let mut decrypted = [0u8; 13];
    assert!(
        libcrux_chacha20poly1305::decrypt(&key, &mut decrypted, &long_ctxt[..29], aad, &nonce)
            .is_ok()
    );
    assert_eq!(decrypted, *msg);

    // test that calling with incorrect-length buffers returns error: non-detached
    let err = libcrux_chacha20poly1305::decrypt(&key, &mut [], &ctxt, aad, &nonce).unwrap_err();
    assert!(matches!(
        err,
        libcrux_chacha20poly1305::AeadError::PlaintextTooShort
    ));

    // test that calling with incorrect-length buffers returns error: detached
    let err =
        libcrux_chacha20poly1305::decrypt_detached(&key, &mut [], &ctxt, &mut [0; 16], aad, &nonce)
            .unwrap_err();
    assert!(matches!(
        err,
        libcrux_chacha20poly1305::AeadError::PlaintextTooShort
    ));
}

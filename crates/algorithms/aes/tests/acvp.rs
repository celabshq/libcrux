use libcrux_kats::acvp::{
    aes::GcmTests,
    schema_common::{assert_covers, Tally},
};

fn gcm<Cipher: libcrux_aes::Aead>(tests: GcmTests, key_len: usize, cipher: Cipher) {
    assert_eq!(tests.prompts.algorithm, "ACVP-AES-GCM");
    assert_eq!(tests.prompts.revision, "1.0");
    assert_covers(&tests.prompts, &tests.results);

    let mut tally = Tally::new();
    for group in &tests.prompts.testGroups {
        assert_eq!(group.testType, "AFT");
        assert_eq!(group.keyLen, key_len);
        assert_eq!(group.ivLen, 96);
        assert_eq!(group.ivGen, "external");
        assert_eq!(group.ivGenMode.as_deref(), Some("8.2.2"));
        assert_eq!(group.tagLen, 128);

        for test in &group.tests {
            let expected = tests.results.find_expected_result(group.tgId, test.tcId);
            let key = cipher.new_key(&test.key).unwrap();
            let nonce = cipher.new_nonce(&test.iv).unwrap();

            match group.direction.as_str() {
                "encrypt" => {
                    let mut ct = vec![0u8; test.pt.len()];
                    let mut tag_bytes = vec![0u8; cipher.tag_len()];
                    let tag = cipher.new_tag_mut(&mut tag_bytes).unwrap();
                    key.encrypt(&mut ct, tag, nonce, &test.aad, &test.pt)
                        .unwrap();
                    assert_eq!(Some(ct), expected.ct, "tcId {}", test.tcId);
                    assert_eq!(Some(tag_bytes), expected.tag, "tcId {}", test.tcId);
                    tally.add("encrypt");
                }
                "decrypt" => {
                    let mut pt = vec![0u8; test.ct.len()];
                    let tag = cipher.new_tag(&test.tag).unwrap();
                    let result = key.decrypt(&mut pt, nonce, &test.aad, &test.ct, tag);
                    let passed = expected.testPassed.unwrap_or(true);
                    assert_eq!(result.is_ok(), passed, "tcId {}", test.tcId);
                    if passed {
                        assert_eq!(Some(pt), expected.pt, "tcId {}", test.tcId);
                        tally.add("decrypt");
                    } else {
                        assert_eq!(expected.pt, None, "tcId {}", test.tcId);
                        tally.add("decrypt failure");
                    }
                }
                d => panic!("unexpected direction {d}"),
            }
        }
    }
    let decrypt_failures = match key_len {
        128 => 25,
        256 => 27,
        k => panic!("unexpected key length {k}"),
    };
    tally.assert_counts([
        ("encrypt", 90),
        ("decrypt", 90 - decrypt_failures),
        ("decrypt failure", decrypt_failures),
    ]);
}

#[test]
fn aesgcm128() {
    gcm(GcmTests::aes128_iv96_tag128(), 128, libcrux_aes::AesGcm128);
}

#[test]
fn aesgcm128_portable() {
    gcm(
        GcmTests::aes128_iv96_tag128(),
        128,
        libcrux_aes::aes_gcm_128::portable::PortableAesGcm128,
    );
}

#[cfg(feature = "simd128")]
#[test]
fn aesgcm128_neon() {
    gcm(
        GcmTests::aes128_iv96_tag128(),
        128,
        libcrux_aes::aes_gcm_128::neon::NeonAesGcm128,
    );
}

#[cfg(feature = "simd256")]
#[test]
fn aesgcm128_x64() {
    gcm(
        GcmTests::aes128_iv96_tag128(),
        128,
        libcrux_aes::aes_gcm_128::x64::X64AesGcm128,
    );
}

#[test]
fn aesgcm256() {
    gcm(GcmTests::aes256_iv96_tag128(), 256, libcrux_aes::AesGcm256);
}

#[test]
fn aesgcm256_portable() {
    gcm(
        GcmTests::aes256_iv96_tag128(),
        256,
        libcrux_aes::aes_gcm_256::portable::PortableAesGcm256,
    );
}

#[cfg(feature = "simd128")]
#[test]
fn aesgcm256_neon() {
    gcm(
        GcmTests::aes256_iv96_tag128(),
        256,
        libcrux_aes::aes_gcm_256::neon::NeonAesGcm256,
    );
}

#[cfg(feature = "simd256")]
#[test]
fn aesgcm256_x64() {
    gcm(
        GcmTests::aes256_iv96_tag128(),
        256,
        libcrux_aes::aes_gcm_256::x64::X64AesGcm256,
    );
}

/// The AES block cipher, as the prerequisite for AES-GCM.
#[cfg(feature = "acvp")]
mod ecb {
    use libcrux_aes::acvp::*;
    use libcrux_kats::acvp::{
        aes::{ecb_mct, EcbTests},
        schema_common::{assert_covers, Tally},
    };

    fn ecb<const KEY_LEN: usize>(tests: EcbTests, encrypt: fn(&[u8; KEY_LEN], &mut [u8; 16])) {
        assert_eq!(tests.prompts.algorithm, "ACVP-AES-ECB");
        assert_eq!(tests.prompts.revision, "1.0");
        assert_covers(&tests.prompts, &tests.results);

        let mut tally = Tally::new();
        for group in &tests.prompts.testGroups {
            assert_eq!(group.direction, "encrypt");
            assert_eq!(group.keyLen, KEY_LEN * 8);

            for test in &group.tests {
                let expected = tests.results.find_expected_result(group.tgId, test.tcId);
                match group.testType.as_str() {
                    "AFT" => {
                        let key: [u8; KEY_LEN] = test.key.as_slice().try_into().unwrap();
                        // Includes multi-block messages.
                        assert_eq!(test.pt.len() % 16, 0);
                        let mut ct = test.pt.clone();
                        for block in ct.as_chunks_mut::<16>().0 {
                            encrypt(&key, block);
                        }
                        assert_eq!(Some(ct), expected.ct, "tcId {}", test.tcId);
                    }
                    "MCT" => assert_eq!(
                        ecb_mct(&test.key, &test.pt, encrypt),
                        expected.resultsArray,
                        "tcId {}",
                        test.tcId
                    ),
                    t => panic!("unexpected testType {t}"),
                }
                tally.add(&group.testType);
            }
        }
        let aft = match KEY_LEN {
            16 => 294,
            32 => 415,
            k => panic!("unexpected key length {k}"),
        };
        tally.assert_counts([("AFT", aft), ("MCT", 1)]);
    }

    #[test]
    fn aes128_ecb_portable() {
        ecb(EcbTests::aes128_encrypt(), portable_aes128_encrypt_block);
    }

    #[cfg(feature = "simd128")]
    #[test]
    fn aes128_ecb_neon() {
        ecb(EcbTests::aes128_encrypt(), neon_aes128_encrypt_block);
    }

    #[cfg(feature = "simd256")]
    #[test]
    fn aes128_ecb_x64() {
        ecb(EcbTests::aes128_encrypt(), x64_aes128_encrypt_block);
    }

    #[test]
    fn aes256_ecb_portable() {
        ecb(EcbTests::aes256_encrypt(), portable_aes256_encrypt_block);
    }

    #[cfg(feature = "simd128")]
    #[test]
    fn aes256_ecb_neon() {
        ecb(EcbTests::aes256_encrypt(), neon_aes256_encrypt_block);
    }

    #[cfg(feature = "simd256")]
    #[test]
    fn aes256_ecb_x64() {
        ecb(EcbTests::aes256_encrypt(), x64_aes256_encrypt_block);
    }
}

mod ccm {
    use libcrux_aes::{aes_ccm_128, aes_ccm_256, Aead};
    use libcrux_kats::acvp::{
        aes::{ccm_schema::CcmPromptTestGroup, CcmTests},
        schema_common::{assert_covers, Tally},
    };

    /// Run the tests of group `g` with `cipher`. Ciphertexts in ACVP include the tag.
    fn ccm_group<Cipher: Aead>(
        cipher: Cipher,
        tests: &CcmTests,
        g: &CcmPromptTestGroup,
        tally: &mut Tally,
    ) {
        assert_eq!(g.testType, "AFT");
        assert_eq!(g.ivLen, 96);
        assert_eq!(g.tagLen, cipher.tag_len() * 8);

        for test in &g.tests {
            let expected = tests.results.find_expected_result(g.tgId, test.tcId);
            let key = cipher.new_key(&test.key).unwrap();
            let nonce = cipher.new_nonce(&test.iv).unwrap();
            match g.direction.as_str() {
                "encrypt" => {
                    let mut ct = vec![0u8; test.pt.len()];
                    let mut tag = vec![0u8; cipher.tag_len()];
                    key.encrypt(
                        &mut ct,
                        cipher.new_tag_mut(&mut tag).unwrap(),
                        nonce,
                        &test.aad,
                        &test.pt,
                    )
                    .unwrap();
                    assert_eq!(Some([ct, tag].concat()), expected.ct, "tcId {}", test.tcId);
                    tally.add("encrypt");
                }
                "decrypt" => {
                    let (ct, tag) = test.ct.split_at(test.ct.len() - cipher.tag_len());
                    let mut pt = vec![0u8; ct.len()];
                    let result =
                        key.decrypt(&mut pt, nonce, &test.aad, ct, cipher.new_tag(tag).unwrap());
                    let passed = expected.testPassed.unwrap_or(true);
                    assert_eq!(result.is_ok(), passed, "tcId {}", test.tcId);
                    if passed {
                        assert_eq!(Some(pt), expected.pt, "tcId {}", test.tcId);
                        tally.add("decrypt");
                    } else {
                        assert_eq!(expected.pt, None, "tcId {}", test.tcId);
                        tally.add("decrypt failure");
                    }
                }
                d => panic!("unexpected direction {d}"),
            }
        }
    }

    /// Run all tests with the AES-CCM implementations for 128 and 256 bit keys, with 128 and
    /// 64 bit (short) tags.
    macro_rules! ccm {
        ($c128:expr, $c128_short:expr, $c256:expr, $c256_short:expr) => {{
            let tests = CcmTests::aes128_256_iv96();
            assert_eq!(tests.prompts.algorithm, "ACVP-AES-CCM");
            assert_eq!(tests.prompts.revision, "1.0");
            assert_covers(&tests.prompts, &tests.results);

            let mut tally = Tally::new();
            for g in &tests.prompts.testGroups {
                match (g.keyLen, g.tagLen) {
                    (128, 128) => ccm_group($c128, &tests, g, &mut tally),
                    (128, 64) => ccm_group($c128_short, &tests, g, &mut tally),
                    (256, 128) => ccm_group($c256, &tests, g, &mut tally),
                    (256, 64) => ccm_group($c256_short, &tests, g, &mut tally),
                    p => panic!("unexpected key and tag length {p:?}"),
                };
            }
            tally.assert_counts([("encrypt", 1400), ("decrypt", 123), ("decrypt failure", 37)]);
        }};
    }

    #[test]
    fn aesccm() {
        ccm!(
            libcrux_aes::AesCcm128,
            aes_ccm_128::short_tag::AesCcm128ShortTag,
            libcrux_aes::AesCcm256,
            aes_ccm_256::short_tag::AesCcm256ShortTag
        );
    }

    #[test]
    fn aesccm_portable() {
        ccm!(
            aes_ccm_128::portable::PortableAesCcm128,
            aes_ccm_128::short_tag::portable::PortableAesCcm128ShortTag,
            aes_ccm_256::portable::PortableAesCcm256,
            aes_ccm_256::short_tag::portable::PortableAesCcm256ShortTag
        );
    }

    #[cfg(feature = "simd128")]
    #[test]
    fn aesccm_neon() {
        ccm!(
            aes_ccm_128::neon::NeonAesCcm128,
            aes_ccm_128::short_tag::neon::NeonAesCcm128ShortTag,
            aes_ccm_256::neon::NeonAesCcm256,
            aes_ccm_256::short_tag::neon::NeonAesCcm256ShortTag
        );
    }

    #[cfg(feature = "simd256")]
    #[test]
    fn aesccm_x64() {
        ccm!(
            aes_ccm_128::x64::X64AesCcm128,
            aes_ccm_128::short_tag::x64::X64AesCcm128ShortTag,
            aes_ccm_256::x64::X64AesCcm256,
            aes_ccm_256::short_tag::x64::X64AesCcm256ShortTag
        );
    }
}

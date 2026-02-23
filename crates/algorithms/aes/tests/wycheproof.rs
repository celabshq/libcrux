use wycheproof::{
    aead::{Test, TestName},
    TestResult,
};

fn run<Cipher: libcrux_aes::Aead>(test: &Test, cipher: Cipher) {
    let mut ciphertext = vec![0u8; test.pt.len()];
    let mut plaintext = vec![0u8; test.pt.len()];
    let mut tag_bytes = vec![0u8; cipher.tag_len()];

    let key = cipher.new_key(&test.key).unwrap();
    let nonce = cipher.new_nonce(&test.nonce).unwrap();
    let tag = cipher.new_tag_mut(&mut tag_bytes).unwrap();

    key.encrypt(&mut ciphertext, tag, nonce, &test.aad, &test.pt)
        .unwrap();

    let tag = cipher.new_tag(&tag_bytes).unwrap();
    key.decrypt(&mut plaintext, nonce, &test.aad, &ciphertext, tag)
        .unwrap();

    assert_eq!(plaintext.as_slice(), test.pt.as_slice());

    if test.result == TestResult::Valid {
        assert_eq!(test.ct.as_slice(), &ciphertext);
        assert_eq!(test.tag.as_slice(), tag.as_ref());
    } else {
        let ct_ok = test.ct.as_slice() == ciphertext;
        let tag_ok = test.tag.as_slice() == tag.as_ref();
        assert!(!ct_ok || !tag_ok);
    }
}

fn test_variant(cipher: impl libcrux_aes::Aead, test_name: TestName) {
    let test_set = wycheproof::aead::TestSet::load(test_name).unwrap();

    // Ensure we ran some tests.
    let mut tested = false;

    for test_group in test_set.test_groups {
        println!(
            "* Group key size:{} tag size:{} nonce size:{}",
            test_group.key_size, test_group.tag_size, test_group.nonce_size,
        );

        if test_group.nonce_size != cipher.nonce_len() * 8 {
            println!("  Skipping unsupported nonce size");
            continue;
        }

        if test_group.tag_size != cipher.tag_len() * 8 {
            println!("  Skipping unsupported tag size");
            continue;
        }

        if test_group.key_size == cipher.key_len() * 8 {
            for test in test_group.tests {
                run(&test, cipher);
                tested = true;
            }
        }
    }

    assert!(tested, "No tests were run.")
}

// XXX: Probably could use a macro to make below more concise.

#[test]
fn aesgcm128() {
    // Multiplexing
    test_variant(libcrux_aes::AesGcm128, wycheproof::aead::TestName::AesGcm);
}

#[test]
fn aesgcm128_portable() {
    test_variant(
        libcrux_aes::aes_gcm_128::portable::PortableAesGcm128,
        wycheproof::aead::TestName::AesGcm,
    );
}

#[cfg(feature = "simd128")]
#[test]
fn aesgcm128_neon() {
    test_variant(
        libcrux_aes::aes_gcm_128::neon::NeonAesGcm128,
        wycheproof::aead::TestName::AesGcm,
    );
}

#[cfg(feature = "simd256")]
#[test]
fn aesgcm128_x64() {
    test_variant(
        libcrux_aes::aes_gcm_128::x64::X64AesGcm128,
        wycheproof::aead::TestName::AesGcm,
    );
}

#[test]
fn aesgcm256() {
    // Multiplexing
    test_variant(libcrux_aes::AesGcm256, wycheproof::aead::TestName::AesGcm);
}

#[test]
fn aesgcm256_portable() {
    test_variant(
        libcrux_aes::aes_gcm_256::portable::PortableAesGcm256,
        wycheproof::aead::TestName::AesGcm,
    );
}

#[cfg(feature = "simd128")]
#[test]
fn aesgcm256_neon() {
    test_variant(
        libcrux_aes::aes_gcm_256::neon::NeonAesGcm256,
        wycheproof::aead::TestName::AesGcm,
    );
}

#[cfg(feature = "simd256")]
#[test]
fn aesgcm256_x64() {
    test_variant(
        libcrux_aes::aes_gcm_256::x64::X64AesGcm256,
        wycheproof::aead::TestName::AesGcm,
    );
}

#[test]
fn aesccm128() {
    // Multiplexing
    test_variant(libcrux_aes::AesCcm128, wycheproof::aead::TestName::AesCcm);
}

#[test]
fn aesccm128_portable() {
    test_variant(
        libcrux_aes::aes_ccm_128::portable::PortableAesCcm128,
        wycheproof::aead::TestName::AesCcm,
    );
}

#[cfg(feature = "simd128")]
#[test]
fn aesccm128_neon() {
    test_variant(
        libcrux_aes::aes_ccm_128::neon::NeonAesCcm128,
        wycheproof::aead::TestName::AesCcm,
    );
}

#[cfg(feature = "simd256")]
#[test]
fn aesccm128_x64() {
    test_variant(
        libcrux_aes::aes_ccm_128::x64::X64AesCcm128,
        wycheproof::aead::TestName::AesCcm,
    );
}

#[test]
fn aesccm256() {
    // Multiplexing
    test_variant(libcrux_aes::AesCcm256, wycheproof::aead::TestName::AesCcm);
}

#[test]
fn aesccm256_portable() {
    test_variant(
        libcrux_aes::aes_ccm_256::portable::PortableAesCcm256,
        wycheproof::aead::TestName::AesCcm,
    );
}

#[cfg(feature = "simd128")]
#[test]
fn aesccm256_neon() {
    test_variant(
        libcrux_aes::aes_ccm_256::neon::NeonAesCcm256,
        wycheproof::aead::TestName::AesCcm,
    );
}

#[cfg(feature = "simd256")]
#[test]
fn aesccm256_x64() {
    test_variant(
        libcrux_aes::aes_ccm_256::x64::X64AesCcm256,
        wycheproof::aead::TestName::AesCcm,
    );
}

#[test]
fn aesccm128_short_tag() {
    // Multiplexing
    test_variant(
        libcrux_aes::aes_ccm_128::short_tag::AesCcm128ShortTag,
        wycheproof::aead::TestName::AesCcm,
    );
}

#[test]
fn aesccm128_portable_short_tag() {
    test_variant(
        libcrux_aes::aes_ccm_128::short_tag::portable::PortableAesCcm128ShortTag,
        wycheproof::aead::TestName::AesCcm,
    );
}

#[cfg(feature = "simd128")]
#[test]
fn aesccm128_neon_short_tag() {
    test_variant(
        libcrux_aes::aes_ccm_128::short_tag::neon::NeonAesCcm128ShortTag,
        wycheproof::aead::TestName::AesCcm,
    );
}

#[cfg(feature = "simd256")]
#[test]
fn aesccm128_x64_short_tag() {
    test_variant(
        libcrux_aes::aes_ccm_128::short_tag::x64::X64AesCcm128ShortTag,
        wycheproof::aead::TestName::AesCcm,
    );
}

#[test]
fn aesccm256_short_tag() {
    // Multiplexing
    test_variant(
        libcrux_aes::aes_ccm_256::short_tag::AesCcm256ShortTag,
        wycheproof::aead::TestName::AesCcm,
    );
}

#[test]
fn aesccm256_portable_short_tag() {
    test_variant(
        libcrux_aes::aes_ccm_256::short_tag::portable::PortableAesCcm256ShortTag,
        wycheproof::aead::TestName::AesCcm,
    );
}

#[cfg(feature = "simd128")]
#[test]
fn aesccm256_neon_short_tag() {
    test_variant(
        libcrux_aes::aes_ccm_256::short_tag::neon::NeonAesCcm256ShortTag,
        wycheproof::aead::TestName::AesCcm,
    );
}

#[cfg(feature = "simd256")]
#[test]
fn aesccm256_x64_short_tag() {
    test_variant(
        libcrux_aes::aes_ccm_256::short_tag::x64::X64AesCcm256ShortTag,
        wycheproof::aead::TestName::AesCcm,
    );
}

use libcrux_aes::{
    aes_ccm_128::{Key as Ccm128Key, Nonce as Ccm128Nonce, Tag as Ccm128Tag},
    aes_gcm_128::{Key as Gcm128Key, Nonce as Gcm128Nonce, Tag as Gcm128Tag},
    Aead, AeadConsts, AesGcm128,
};

// tests that an error is returned if ptxt.len() != ctxt.len()
#[test]
fn non_matching_lengths() {
    use libcrux_aes::AeadConsts as _;

    let k: Gcm128Key = [0; AesGcm128::KEY_LEN].into();
    let nonce: Gcm128Nonce = [0; AesGcm128::NONCE_LEN].into();
    let mut tag: Gcm128Tag = [0; AesGcm128::TAG_LEN].into();

    let pt = vec![0; 12];

    k.encrypt(&mut [0; 43], &mut tag, &nonce, b"", &pt)
        .unwrap_err();
}

// tests that an error is returned if ptxt is too long
// NOTE: this test is not applicable for pointer widths less than 64.
#[test]
#[cfg(target_pointer_width = "64")]
fn ptxt_too_long() {
    use libcrux_aes::AeadConsts as _;
    use libcrux_traits::aead::arrayref::{DecryptError, EncryptError};

    let k: Gcm128Key = [0; AesGcm128::KEY_LEN].into();
    let nonce: Gcm128Nonce = [0; AesGcm128::NONCE_LEN].into();
    let mut tag: Gcm128Tag = [0; AesGcm128::TAG_LEN].into();

    // unsafely create a slice that is too long
    let pt: &mut [u8] =
        unsafe { std::slice::from_raw_parts_mut(8 as *mut u8, u32::MAX as usize * 16) };

    // check that encryption returns error
    let e = k.encrypt(&mut [], &mut tag, &nonce, b"", &pt).unwrap_err();
    assert_eq!(e, EncryptError::PlaintextTooLong);

    // check that decryption returns error
    let e = k.decrypt(pt, &nonce, b"", &mut [], &tag).unwrap_err();
    assert_eq!(e, DecryptError::PlaintextTooLong);
}

#[test]
fn ccm_two_byte_aad_len_encoding() {
    use libcrux_aes::{AeadConsts as _, AesCcm128};

    let k: Ccm128Key = [0; AesCcm128::KEY_LEN].into();
    let nonce: Ccm128Nonce = [0; AesCcm128::NONCE_LEN].into();
    let mut tag: Ccm128Tag = [0; AesCcm128::TAG_LEN].into();

    // unsafely create a slice that is too long
    let aad = vec![8; 512];

    let pt = [42u8; 1];
    let mut ct = [0u8; 1];
    k.encrypt(&mut ct, &mut tag, &nonce, &aad, &pt).unwrap();

    let mut pt_decrypted = [0u8; 1];
    k.decrypt(&mut pt_decrypted, &nonce, &aad, &ct, &tag)
        .unwrap();

    assert_eq!(pt, pt_decrypted);
}

#[test]
#[ignore] // This is a really slow test, we ignore it on CI.
fn ccm_six_byte_aad_len_encoding() {
    use libcrux_aes::{AeadConsts as _, AesCcm128};

    let k: Ccm128Key = [0; AesCcm128::KEY_LEN].into();
    let nonce: Ccm128Nonce = [0; AesCcm128::NONCE_LEN].into();
    let mut tag: Ccm128Tag = [0; AesCcm128::TAG_LEN].into();

    // 2^16 - 2^8 - 1 = 65279
    let aad = vec![8; 65279];

    let pt = [42u8; 1];
    let mut ct = [0u8; 1];
    k.encrypt(&mut ct, &mut tag, &nonce, &aad, &pt).unwrap();

    let mut pt_decrypted = [0u8; 1];
    k.decrypt(&mut pt_decrypted, &nonce, &aad, &ct, &tag)
        .unwrap();

    assert_eq!(pt, pt_decrypted);
}

#[test]
#[ignore] // This is a really slow test, we ignore it on CI.
#[cfg(target_pointer_width = "64")]
fn ccm_ten_byte_aad_len_encoding() {
    use libcrux_aes::{AeadConsts as _, AesCcm128};

    let k: Ccm128Key = [0; AesCcm128::KEY_LEN].into();
    let nonce: Ccm128Nonce = [0; AesCcm128::NONCE_LEN].into();
    let mut tag: Ccm128Tag = [0; AesCcm128::TAG_LEN].into();

    let aad = vec![8; u32::MAX as usize + 1];

    let pt = [42u8; 1];
    let mut ct = [0u8; 1];
    k.encrypt(&mut ct, &mut tag, &nonce, &aad, &pt).unwrap();

    let mut pt_decrypted = [0u8; 1];
    k.decrypt(&mut pt_decrypted, &nonce, &aad, &ct, &tag)
        .unwrap();

    assert_eq!(pt, pt_decrypted);
}

#[test]
fn ccm_nist_kat() {
    use libcrux_aes::aes_ccm_128::short_tag::{AesCcm128ShortTag, Key, Nonce, Tag};

    let k: [u8; AesCcm128ShortTag::KEY_LEN] = hex::decode("404142434445464748494a4b4c4d4e4f")
        .unwrap()
        .try_into()
        .unwrap();
    let k: Key = k.into();
    let nonce: [u8; AesCcm128ShortTag::NONCE_LEN] = hex::decode("101112131415161718191a1b")
        .unwrap()
        .try_into()
        .unwrap();
    let nonce: Nonce = nonce.into();

    let mut tag: Tag = [0; AesCcm128ShortTag::TAG_LEN].into();

    let aad = hex::decode("000102030405060708090a0b0c0d0e0f10111213").unwrap();
    let pt = hex::decode("202122232425262728292a2b2c2d2e2f3031323334353637").unwrap();
    let mut ct = pt.clone();
    k.encrypt(&mut ct, &mut tag, &nonce, &aad, &pt).unwrap();

    let ct_expected =
        hex::decode("e3b201a9f5b71a7a9b1ceaeccd97e70b6176aad9a4428aa5484392fbc1b09951").unwrap();

    assert_eq!(ct, ct_expected[..ct_expected.len() - 8]);
    assert_eq!(tag.as_ref(), &ct_expected[ct_expected.len() - 8..]);
}

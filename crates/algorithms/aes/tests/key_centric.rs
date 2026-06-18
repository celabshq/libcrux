use libcrux_aes::{
    aes_gcm_128::{Key, Nonce, Tag},
    AesGcm128,
};

#[test]
fn test_key_centric_owned() {
    use libcrux_aes::AeadConsts as _;

    let k: Key = [0; AesGcm128::KEY_LEN].into();
    let nonce: Nonce = [0; AesGcm128::NONCE_LEN].into();
    let mut tag: Tag = [0; AesGcm128::TAG_LEN].into();

    let pt = b"the quick brown fox jumps over the lazy dog";
    let mut ct = [0; 43];
    let mut pt_out = [0; 43];

    k.encrypt(&mut ct, &mut tag, &nonce, b"", pt).unwrap();
    k.decrypt(&mut pt_out, &nonce, b"", &ct, &tag).unwrap();
    assert_eq!(pt, &pt_out);
}

#[test]
fn test_key_centric_refs() {
    use libcrux_aes::{Aead as _, AeadConsts as _};

    let algo = AesGcm128;

    let mut tag_bytes = [0; AesGcm128::TAG_LEN];
    let key = algo.new_key(&[0; AesGcm128::KEY_LEN]).unwrap();
    let tag = algo.new_tag_mut(&mut tag_bytes).unwrap();
    let nonce = algo.new_nonce(&[0; AesGcm128::NONCE_LEN]).unwrap();

    let pt = b"the quick brown fox jumps over the lazy dog";
    let mut ct = [0; 43];
    let mut pt_out = [0; 43];

    key.encrypt(&mut ct, tag, nonce, b"", pt).unwrap();
    let tag = algo.new_tag(&tag_bytes).unwrap();
    key.decrypt(&mut pt_out, nonce, b"", &ct, tag).unwrap();
    assert_eq!(pt, &pt_out);
}

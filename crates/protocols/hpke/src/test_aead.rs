use hpke_rs_crypto::{types::AeadAlgorithm, HpkeCrypto};
use hpke_rs_libcrux::HpkeLibcrux;
use hpke_rs_rust_crypto::HpkeRustCrypto;

#[test]
fn test_aes_gcm_128_self() {
    let key = [
        0x5b, 0x96, 0x04, 0xfe, 0x14, 0xea, 0xdb, 0xa9, 0x31, 0xb0, 0xcc, 0xf3, 0x48, 0x43, 0xda,
        0xb9,
    ];
    let nonce = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
    ];
    let aad = [0x03, 0x04, 0x05];
    let msg = b"test message";

    // test rust crypto provider
    let ctxt =
        HpkeRustCrypto::aead_seal(AeadAlgorithm::Aes128Gcm, &key, &nonce, &aad, msg).unwrap();
    let ptxt =
        HpkeRustCrypto::aead_open(AeadAlgorithm::Aes128Gcm, &key, &nonce, &aad, &ctxt).unwrap();
    assert_eq!(&ptxt, msg);

    // test libcrux crypto provider
    let ctxt = <HpkeLibcrux>::aead_seal(AeadAlgorithm::Aes128Gcm, &key, &nonce, &aad, msg).unwrap();
    let ptxt =
        <HpkeLibcrux>::aead_open(AeadAlgorithm::Aes128Gcm, &key, &nonce, &aad, &ctxt).unwrap();
    assert_eq!(&ptxt, msg);
}

#[test]
fn test_aes_gcm_256_self() {
    let key = [
        0x5b, 0x96, 0x04, 0xfe, 0x14, 0xea, 0xdb, 0xa9, 0x31, 0xb0, 0xcc, 0xf3, 0x48, 0x43, 0xda,
        0xb9, 0x5b, 0x96, 0x04, 0xfe, 0x14, 0xea, 0xdb, 0xa9, 0x31, 0xb0, 0xcc, 0xf3, 0x48, 0x43,
        0xda, 0xb9,
    ];
    let nonce = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
    ];
    let aad = [0x03, 0x04, 0x05];
    let msg = b"test message";

    // test rust crypto provider
    let ctxt =
        HpkeRustCrypto::aead_seal(AeadAlgorithm::Aes256Gcm, &key, &nonce, &aad, msg).unwrap();
    let ptxt =
        HpkeRustCrypto::aead_open(AeadAlgorithm::Aes256Gcm, &key, &nonce, &aad, &ctxt).unwrap();
    assert_eq!(&ptxt, msg);

    // test libcrux crypto provider
    let ctxt = <HpkeLibcrux>::aead_seal(AeadAlgorithm::Aes256Gcm, &key, &nonce, &aad, msg).unwrap();
    let ptxt =
        <HpkeLibcrux>::aead_open(AeadAlgorithm::Aes256Gcm, &key, &nonce, &aad, &ctxt).unwrap();
    assert_eq!(&ptxt, msg);
}

#[test]
fn test_chacha20poly1305_self() {
    let key = [
        0x5b, 0x96, 0x04, 0xfe, 0x14, 0xea, 0xdb, 0xa9, 0x31, 0xb0, 0xcc, 0xf3, 0x48, 0x43, 0xda,
        0xb9, 0x5b, 0x96, 0x04, 0xfe, 0x14, 0xea, 0xdb, 0xa9, 0x31, 0xb0, 0xcc, 0xf3, 0x48, 0x43,
        0xda, 0xb9,
    ];
    let nonce = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
    ];
    let aad = [0x03, 0x04, 0x05];
    let msg = b"test message";

    // test rust crypto provider
    let ctxt = HpkeRustCrypto::aead_seal(AeadAlgorithm::ChaCha20Poly1305, &key, &nonce, &aad, msg)
        .unwrap();
    let ptxt =
        HpkeRustCrypto::aead_open(AeadAlgorithm::ChaCha20Poly1305, &key, &nonce, &aad, &ctxt)
            .unwrap();
    assert_eq!(&ptxt, msg);

    // test libcrux provider
    let ctxt =
        <HpkeLibcrux>::aead_seal(AeadAlgorithm::ChaCha20Poly1305, &key, &nonce, &aad, msg).unwrap();
    let ptxt = <HpkeLibcrux>::aead_open(AeadAlgorithm::ChaCha20Poly1305, &key, &nonce, &aad, &ctxt)
        .unwrap();
    assert_eq!(&ptxt, msg);
}

// Tests error handling for keys of incorrect length
#[test]
fn test_chacha20poly1305_self_error() {
    let correct_length_key = [
        0x5b, 0x96, 0x04, 0xfe, 0x14, 0xea, 0xdb, 0xa9, 0x31, 0xb0, 0xcc, 0xf3, 0x48, 0x43, 0xda,
        0xb9, 0x5b, 0x96, 0x04, 0xfe, 0x14, 0xea, 0xdb, 0xa9, 0x31, 0xb0, 0xcc, 0xf3, 0x48, 0x43,
        0xda, 0xb9,
    ];
    let incorrect_length_key = [
        0x5b, 0x96, 0x04, 0xfe, 0x14, 0xea, 0xdb, 0xa9, 0x31, 0xb0, 0xcc, 0xf3, 0x48, 0x43, 0xda,
        0xb9,
    ];
    let nonce = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
    ];
    let aad = [0x03, 0x04, 0x05];
    let msg = b"test message";
    HpkeRustCrypto::aead_seal(
        AeadAlgorithm::ChaCha20Poly1305,
        &incorrect_length_key,
        &nonce,
        &aad,
        msg,
    )
    .expect_err("Should fail due to incorrect key length");
    let ctxt = HpkeRustCrypto::aead_seal(
        AeadAlgorithm::ChaCha20Poly1305,
        &correct_length_key,
        &nonce,
        &aad,
        msg,
    )
    .unwrap();
    HpkeRustCrypto::aead_open(
        AeadAlgorithm::ChaCha20Poly1305,
        &incorrect_length_key,
        &nonce,
        &aad,
        &ctxt,
    )
    .expect_err("Should fail due to incorrect key length");
}

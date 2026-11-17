use ctgrind_test::{classify, declassify};
use libcrux_p384::{PrivateKey, PublicKey};
use std::hint::black_box;

/// Parsing a secret private key, which checks 0 < sk < n.
#[inline(never)]
fn test_private_key_validation() {
    let sk_bytes = black_box([0x42u8; 48]);
    classify(&sk_bytes);

    let ok = PrivateKey::try_from(&sk_bytes[..]).is_ok();
    declassify(&ok);
    println!("p384 private key valid: {ok}");
}

#[inline(never)]
fn test_secret_to_public() {
    let sk = PrivateKey::try_from(&black_box([0x42u8; 48])[..]).unwrap();
    classify(sk.as_ref());

    let pk = PublicKey::from(&sk);
    let mut pk_bytes = [0u8; 97];
    pk.to_uncompressed(&mut pk_bytes);
    declassify(&pk_bytes);
    println!("p384 public key: {:02x?}", &pk_bytes[..4]);
}

#[inline(never)]
fn test_ecdh() {
    let their_sk = PrivateKey::try_from(&black_box([0x24u8; 48])[..]).unwrap();
    let their_pk = PublicKey::from(&their_sk);

    let sk = PrivateKey::try_from(&black_box([0x42u8; 48])[..]).unwrap();
    classify(sk.as_ref());

    let ss = their_pk.ecdh(&sk);
    declassify(ss.as_ref());
    println!("p384 ecdh: {:02x?}", &ss.as_ref()[..4]);
}

pub fn main() {
    test_private_key_validation();
    test_secret_to_public();
    test_ecdh();
}

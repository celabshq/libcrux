#![cfg(feature = "rand")]

use libcrux_ed25519::{generate_key_pair, sign, verify};

#[test]
fn self_test() {
    let mut rng = rand::rng();

    let (sk, vk) = generate_key_pair(&mut rng).unwrap();
    let msg = b"the message to be signed";

    let signature = sign(msg, sk.as_ref()).unwrap();

    let verified = verify(msg, vk.as_ref(), &signature);
    assert!(verified.is_ok());
}

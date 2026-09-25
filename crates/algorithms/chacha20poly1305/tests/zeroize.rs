#![cfg(feature = "zeroize")]

use libcrux_chacha20poly1305::{Key, KEY_LEN};
use libcrux_secrets::{Classify, DeclassifyRef};
use zeroize::Zeroize;

#[test]
fn key_zeroize_clears_bytes() {
    let mut key = Key::from([0xAAu8; KEY_LEN].classify());
    assert_eq!(key.as_ref().declassify_ref(), &[0xAAu8; KEY_LEN]);

    key.zeroize();

    assert_eq!(key.as_ref().declassify_ref(), &[0u8; KEY_LEN]);
}

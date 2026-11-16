use libcrux_secrets::{Classify, ClassifyRef, DeclassifyRef};

use super::arrayref::*;

pub fn simple<
    const KEY_LEN: usize,
    const TAG_LEN: usize,
    const NONCE_LEN: usize,
    T: Aead<KEY_LEN, TAG_LEN, NONCE_LEN>,
>() {
    let key = [1.classify(); KEY_LEN];
    let nonce = [2.classify(); NONCE_LEN];
    let plaintext = b"abcdefgh".classify_ref();
    let aad = b"ijklmnop";

    let mut ciphertext = [0; 8];
    let mut tag = [0.classify(); TAG_LEN];
    T::encrypt(&mut ciphertext, &mut tag, &key, &nonce, aad, plaintext)
        .expect("Aead::encrypt failed");

    let mut decrypted_plaintext = [0.classify(); 8];
    T::decrypt(
        &mut decrypted_plaintext,
        &key,
        &nonce,
        aad,
        &ciphertext,
        &tag,
    )
    .expect("Aead::decrypt failed");

    assert_eq!(
        plaintext.declassify_ref(),
        decrypted_plaintext.declassify_ref()
    );
}

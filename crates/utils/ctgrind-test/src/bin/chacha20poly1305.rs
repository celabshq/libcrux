use ctgrind_test::{classify, declassify};
use std::hint::black_box;

#[inline(never)]
fn test_poly1305_mac() {
    let key = black_box([0x42u8; 32]);
    let msg = black_box([0x17u8; 100]);
    classify(&key);

    let mut tag = [0u8; 16];
    assert!(libcrux_poly1305::mac(&key, &msg, &mut tag).is_ok());
    declassify(&tag);
    println!("poly1305 tag: {:02x?}", &tag[..4]);
}

/// Decryption with a secret key. The tag check result is declassified in the crate.
#[inline(never)]
fn test_decrypt(valid_tag: bool) {
    let key = black_box([0x42u8; 32]);
    let nonce = black_box([0x24u8; 12]);
    let aad = black_box([0x17u8; 13]);
    let msg = black_box([0x71u8; 64]);
    let mut ctxt = [0u8; 64 + 16];
    libcrux_chacha20poly1305::encrypt(&key, &msg, &mut ctxt, &aad, &nonce).unwrap();
    if !valid_tag {
        ctxt[64] ^= 1;
    }
    classify(&key);

    let mut ptxt = [0u8; 64];
    let ok = libcrux_chacha20poly1305::decrypt(&key, &mut ptxt, &ctxt, &aad, &nonce).is_ok();
    declassify(&ptxt);
    println!("chacha20poly1305 decrypt (valid tag: {valid_tag}): {ok}");
}

/// The tag comparison inside `decrypt`, with a secret computed tag.
#[cfg(valgrind_ct_test)]
#[inline(never)]
fn test_tag_eq(valid_tag: bool) {
    let computed_tag = black_box([0x42u8; 16]);
    let mut tag = black_box([0x42u8; 16]);
    if !valid_tag {
        tag[15] ^= 1;
    }
    classify(&computed_tag);

    let eq = libcrux_chacha20poly1305::ct_test::tag_eq(&computed_tag, &tag);
    declassify(&eq);
    println!("chacha20poly1305 tag_eq (valid tag: {valid_tag}): {eq}");
}

pub fn main() {
    test_poly1305_mac();
    test_decrypt(true);
    test_decrypt(false);
    #[cfg(valgrind_ct_test)]
    {
        test_tag_eq(true);
        test_tag_eq(false);
    }
}

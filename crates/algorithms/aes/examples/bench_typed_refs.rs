// structs for typed_owned
use libcrux_aes::{Aead as _, AeadConsts as _, AesGcm128};

fn main() {
    const PAYLOAD_SIZE: usize = 3045;

    let algo = AesGcm128;

    let mut tag_bytes = [0u8; AesGcm128::TAG_LEN];

    let key = algo.new_key(&[0x16; AesGcm128::KEY_LEN]).unwrap();
    let nonce = algo.new_nonce(&[0x12; AesGcm128::NONCE_LEN]).unwrap();

    let aad = [0xff; 32];
    let plaintext = [0xab; PAYLOAD_SIZE];

    let mut ciphertext = vec![0; PAYLOAD_SIZE];

    for _ in 0..10000 {
        let tag = algo.new_tag_mut(&mut tag_bytes).unwrap();
        key.encrypt(&mut ciphertext, tag, nonce, &aad, &plaintext)
            .unwrap();
    }
}

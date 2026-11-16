#![no_main]

use libcrux_ml_kem::{mlkem768, ENCAPS_SEED_SIZE, KEY_GENERATION_SEED_SIZE};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < KEY_GENERATION_SEED_SIZE + ENCAPS_SEED_SIZE {
        // Not enough entropy
        return;
    }

    let mut randomness = [0u8; KEY_GENERATION_SEED_SIZE];
    randomness.copy_from_slice(&data[..KEY_GENERATION_SEED_SIZE]);

    let key_pair = mlkem768::generate_key_pair(randomness);

    let mut randomness = [0u8; ENCAPS_SEED_SIZE];
    randomness.copy_from_slice(
        &data[KEY_GENERATION_SEED_SIZE..KEY_GENERATION_SEED_SIZE + ENCAPS_SEED_SIZE],
    );

    let (ct, _ss) = mlkem768::encapsulate(key_pair.public_key(), randomness);

    let _ = core::hint::black_box(mlkem768::decapsulate(key_pair.private_key(), &ct));
});

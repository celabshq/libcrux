#![no_main]

use libcrux_ml_kem::{mlkem768, KEY_GENERATION_SEED_SIZE};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < KEY_GENERATION_SEED_SIZE {
        // We need enough entropy.
        return;
    }
    let mut randomness = [0u8; KEY_GENERATION_SEED_SIZE];
    randomness.copy_from_slice(&data[..KEY_GENERATION_SEED_SIZE]);
    let _ = core::hint::black_box(mlkem768::generate_key_pair(randomness));
});

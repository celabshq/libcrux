use libcrux_ml_kem::{mlkem768, KEY_GENERATION_SEED_SIZE};
use rand::{rngs::OsRng, TryRngCore};

fn main() {
    let mut randomness = [0u8; KEY_GENERATION_SEED_SIZE];
    for _ in 0..100_000 {
        OsRng.try_fill_bytes(&mut randomness).unwrap();
        let _ = mlkem768::generate_key_pair(randomness);
    }
}

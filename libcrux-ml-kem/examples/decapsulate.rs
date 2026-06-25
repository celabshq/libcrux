use libcrux_ml_kem::{mlkem768, ENCAPS_SEED_SIZE, KEY_GENERATION_SEED_SIZE};
use rand::{rngs::OsRng, TryRngCore};

fn main() {
    let mut randomness = [0u8; KEY_GENERATION_SEED_SIZE];
    OsRng.try_fill_bytes(&mut randomness).unwrap();

    let key_pair = mlkem768::generate_key_pair(randomness);
    let mut randomness = [0u8; ENCAPS_SEED_SIZE];
    OsRng.try_fill_bytes(&mut randomness).unwrap();
    let (ct, _ss) = mlkem768::encapsulate(key_pair.public_key(), randomness);

    for _ in 0..100_000 {
        let _ = mlkem768::decapsulate(key_pair.private_key(), &ct);
    }
}

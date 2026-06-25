use libcrux_ml_dsa::ml_dsa_65;
use rand::{rngs::OsRng, TryRngCore};

fn random_array<const L: usize>() -> [u8; L] {
    let mut rng = OsRng;
    let mut seed = [0; L];
    rng.try_fill_bytes(&mut seed).unwrap();
    seed
}

fn main() {
    let key_generation_seed = random_array();
    let signing_randomness = random_array();
    let message = random_array::<1023>();

    let keypair = ml_dsa_65::generate_key_pair(key_generation_seed);

    for _i in 0..10_000 {
        let _ = ml_dsa_65::sign(&keypair.signing_key, &message, b"", signing_randomness);
    }
}

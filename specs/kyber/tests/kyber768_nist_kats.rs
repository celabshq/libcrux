extern crate hacspec_kyber;

use std::{fs::File, io::BufReader};

use hacspec_kyber::generate_keypair;
use libcrux::algorithms::sha3;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize)]
struct Kyber768NISTKAT {
    #[serde(with = "hex::serde")]
    key_generation_seed: [u8; 64],

    #[serde(with = "hex::serde")]
    sha3_256_hash_of_public_key: [u8; 32],

    #[serde(with = "hex::serde")]
    sha3_256_hash_of_secret_key: [u8; 32],

    #[serde(with = "hex::serde")]
    encapsulation_seed: [u8; 32],

    #[serde(with = "hex::serde")]
    sha3_256_hash_of_ciphertext: [u8; 32],

    #[serde(with = "hex::serde")]
    shared_secret: [u8; 32],
}

#[test]
fn kyber768_known_answer_tests() {
    let katfile = File::open("tests/kyber768_nist_kats.json").expect("Could not open KAT file.");
    let reader = BufReader::new(katfile);

    let nist_kats: Vec<Kyber768NISTKAT> =
        serde_json::from_reader(reader).expect("Could not deserialize KAT file.");

    for kat in nist_kats {
        let key_pair = generate_keypair(kat.key_generation_seed).unwrap();

        let public_key_hash = sha3::sha256(key_pair.pk());
        for i in 0..public_key_hash.len() {
            assert_eq!(public_key_hash[i], kat.sha3_256_hash_of_public_key[i]);
        }

        let secret_key_hash = sha3::sha256(key_pair.sk());
        for i in 0..secret_key_hash.len() {
            assert_eq!(secret_key_hash[i], kat.sha3_256_hash_of_secret_key[i]);
        }

        let (ciphertext, shared_secret) =
            hacspec_kyber::encapsulate(key_pair.pk().clone(), kat.encapsulation_seed).unwrap();

        let ciphertext_hash = sha3::sha256(&ciphertext);
        for i in 0..ciphertext_hash.len() {
            assert_eq!(ciphertext_hash[i], kat.sha3_256_hash_of_ciphertext[i]);
        }

        for i in 0..shared_secret.len() {
            assert_eq!(shared_secret[i], kat.shared_secret[i]);
        }

        let shared_secret_from_decapsulate = hacspec_kyber::decapsulate(ciphertext, *key_pair.sk());
        for i in 0..shared_secret.len() {
            assert_eq!(shared_secret_from_decapsulate[i], shared_secret[i]);
        }
    }
}

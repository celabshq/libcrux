//! Fuzz the responder's handshake message parser.
//!
//! The responder is the only party that accepts messages from unknown, untrusted
//! initiators over the network.  Feed arbitrary bytes in place of the first
//! initiator message and verify that no panic or undefined behaviour occurs.
//!
//! The ciphersuite `X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256` is
//! chosen because it exercises all three parser branches: outer DH, PQ KEM
//! decapsulation, and ML-DSA signature verification.
#![no_main]

use std::sync::LazyLock;

use libcrux_ml_kem::mlkem768;
use libcrux_psq::{
    handshake::{
        builders::{CiphersuiteBuilder, PrincipalBuilder},
        ciphersuites::CiphersuiteName,
        types::DHKeyPair,
    },
    Channel,
};
use libfuzzer_sys::fuzz_target;

struct Setup {
    responder_x25519_keys: DHKeyPair,
    responder_mlkem_keys: mlkem768::MlKem768KeyPair,
}

static SETUP: LazyLock<Setup> = LazyLock::new(|| {
    let mut rng = rand::rng();
    Setup {
        responder_x25519_keys: DHKeyPair::new(&mut rng),
        responder_mlkem_keys: mlkem768::rand::generate_key_pair(&mut rng),
    }
});

fuzz_target!(|data: &[u8]| {
    let setup = &*SETUP;

    let cname = CiphersuiteName::X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256;
    let Ok(responder_ciphersuite) = CiphersuiteBuilder::new(cname)
        .longterm_x25519_keys(&setup.responder_x25519_keys)
        .longterm_mlkem_encapsulation_key(setup.responder_mlkem_keys.public_key())
        .longterm_mlkem_decapsulation_key(setup.responder_mlkem_keys.private_key())
        .build_responder_ciphersuite()
    else {
        return;
    };

    let Ok(mut responder) =
        PrincipalBuilder::new(rand::rng()).build_responder(responder_ciphersuite)
    else {
        return;
    };

    let mut payload_buf = vec![0u8; data.len().saturating_add(64)];
    let _ = responder.read_message(data, &mut payload_buf);
});

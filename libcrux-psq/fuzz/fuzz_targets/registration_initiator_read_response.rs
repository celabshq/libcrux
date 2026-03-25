//! Fuzz the registration-initiator's response parser.
//!
//! A registration initiator sends a first message and then reads the
//! responder's reply to complete the handshake.  Feed arbitrary bytes as that
//! reply to exercise the response-parsing and key-derivation code paths.
//!
//! `write_message` is called first to advance the initiator into the correct
//! state; only the subsequent `read_message` call receives the fuzz input.
#![no_main]

use std::sync::LazyLock;

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
    initiator_x25519_keys: DHKeyPair,
}

static SETUP: LazyLock<Setup> = LazyLock::new(|| {
    let mut rng = rand::rng();
    Setup {
        responder_x25519_keys: DHKeyPair::new(&mut rng),
        initiator_x25519_keys: DHKeyPair::new(&mut rng),
    }
});

fuzz_target!(|data: &[u8]| {
    let setup = &*SETUP;

    // X25519-auth, no PQ KEM — cheap write_message so iteration rate stays high.
    let cname = CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256;
    let Ok(initiator_ciphersuite) = CiphersuiteBuilder::new(cname)
        .longterm_x25519_keys(&setup.initiator_x25519_keys)
        .peer_longterm_x25519_pk(&setup.responder_x25519_keys.pk)
        .build_initiator_ciphersuite()
    else {
        return;
    };

    let Ok(mut initiator) =
        PrincipalBuilder::new(rand::rng()).build_registration_initiator(initiator_ciphersuite)
    else {
        return;
    };

    // Advance the initiator to the "awaiting response" state.
    let mut msg_buf = vec![0u8; 4096];
    if initiator.write_message(b"register", &mut msg_buf).is_err() {
        return;
    }

    let mut payload_buf = vec![0u8; data.len().saturating_add(64)];
    let _ = initiator.read_message(data, &mut payload_buf);
});

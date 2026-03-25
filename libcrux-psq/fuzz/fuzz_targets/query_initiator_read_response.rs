//! Fuzz the query-initiator's response parser.
//!
//! A query initiator contacts a known responder without long-term registration.
//! After sending the first message it reads the responder's reply.  Feed
//! arbitrary bytes as that reply to verify robustness of the parser.
//!
//! `write_message` is called with a fixed payload to advance the initiator into
//! the "awaiting response" state before the fuzz input is exercised.
#![no_main]

use std::sync::LazyLock;

use libcrux_psq::{
    handshake::{builders::PrincipalBuilder, types::DHKeyPair},
    Channel,
};
use libfuzzer_sys::fuzz_target;

struct Setup {
    responder_x25519_keys: DHKeyPair,
}

static SETUP: LazyLock<Setup> = LazyLock::new(|| Setup {
    responder_x25519_keys: DHKeyPair::new(&mut rand::rng()),
});

fuzz_target!(|data: &[u8]| {
    let setup = &*SETUP;

    let Ok(mut initiator) =
        PrincipalBuilder::new(rand::rng()).build_query_initiator(&setup.responder_x25519_keys.pk)
    else {
        return;
    };

    // Advance the initiator to the "awaiting response" state.
    let mut msg_buf = vec![0u8; 4096];
    if initiator.write_message(b"query", &mut msg_buf).is_err() {
        return;
    }

    let mut payload_buf = vec![0u8; data.len().saturating_add(64)];
    let _ = initiator.read_message(data, &mut payload_buf);
});

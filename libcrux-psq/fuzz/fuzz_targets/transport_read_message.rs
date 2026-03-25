//! Fuzz `Transport::read_message`.
//!
//! After a completed registration handshake both parties derive a transport
//! channel.  Feed arbitrary bytes as an incoming transport message to verify
//! that the AEAD-decryption and framing code cannot be crashed by malformed
//! ciphertexts.
//!
//! The session is established once in a `LazyLock` and then serialized so that
//! each fuzzing iteration gets a fresh transport (with reset nonce counter) by
//! deserializing the stored bytes cheaply.
#![no_main]

use std::sync::LazyLock;

use libcrux_psq::{
    handshake::{
        builders::{CiphersuiteBuilder, PrincipalBuilder},
        ciphersuites::CiphersuiteName,
        types::{Authenticator, DHKeyPair, DHPublicKey},
    },
    session::{Session, SessionBinding},
    Channel, IntoSession,
};
use libfuzzer_sys::fuzz_target;

struct TransportSetup {
    /// Serialized initiator session — deserialized fresh every iteration.
    session_bytes: Vec<u8>,
    /// Owned authenticator kept alive so `SessionBinding` can borrow it.
    initiator_auth: Authenticator,
    /// Owned responder ECDH public key kept alive for the same reason.
    responder_ecdh_pk: DHPublicKey,
}

static TRANSPORT_SETUP: LazyLock<TransportSetup> = LazyLock::new(|| {
    let mut rng = rand::rng();
    let responder_x25519_keys = DHKeyPair::new(&mut rng);
    let initiator_x25519_keys = DHKeyPair::new(&mut rng);

    let cname = CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256;

    let initiator_ciphersuite = CiphersuiteBuilder::new(cname)
        .longterm_x25519_keys(&initiator_x25519_keys)
        .peer_longterm_x25519_pk(&responder_x25519_keys.pk)
        .build_initiator_ciphersuite()
        .expect("build initiator ciphersuite");

    let mut initiator = PrincipalBuilder::new(rand::rng())
        .build_registration_initiator(initiator_ciphersuite)
        .expect("build initiator");

    let responder_ciphersuite = CiphersuiteBuilder::new(cname)
        .longterm_x25519_keys(&responder_x25519_keys)
        .build_responder_ciphersuite()
        .expect("build responder ciphersuite");

    let mut responder = PrincipalBuilder::new(rand::rng())
        .build_responder(responder_ciphersuite)
        .expect("build responder");

    // Complete the two-round handshake.
    let mut msg_buf = vec![0u8; 4096];
    let mut payload_buf = vec![0u8; 4096];

    let len_i = initiator
        .write_message(b"init", &mut msg_buf)
        .expect("initiator write");
    responder
        .read_message(&msg_buf[..len_i], &mut payload_buf)
        .expect("responder read");
    let len_r = responder
        .write_message(b"resp", &mut msg_buf)
        .expect("responder write");
    initiator
        .read_message(&msg_buf[..len_r], &mut payload_buf)
        .expect("initiator read");

    let i_session = initiator.into_session().expect("into_session");

    // Store owned binding components so they outlive the LazyLock init closure.
    let initiator_auth = Authenticator::Dh(initiator_x25519_keys.pk.clone());
    let responder_ecdh_pk = responder_x25519_keys.pk.clone();

    let mut session_storage = vec![0u8; 4096];
    let session_len = i_session
        .serialize(
            &mut session_storage,
            SessionBinding {
                initiator_authenticator: &initiator_auth,
                responder_ecdh_pk: &responder_ecdh_pk,
                responder_pq_pk: None,
            },
        )
        .expect("serialize session");
    session_storage.truncate(session_len);

    TransportSetup {
        session_bytes: session_storage,
        initiator_auth,
        responder_ecdh_pk,
    }
});

fuzz_target!(|data: &[u8]| {
    let setup = &*TRANSPORT_SETUP;

    let Ok(mut session) = Session::deserialize(
        &setup.session_bytes,
        SessionBinding {
            initiator_authenticator: &setup.initiator_auth,
            responder_ecdh_pk: &setup.responder_ecdh_pk,
            responder_pq_pk: None,
        },
    ) else {
        return;
    };

    let Ok(mut transport) = session.transport_channel() else {
        return;
    };

    let mut payload_buf = vec![0u8; data.len().saturating_add(64)];
    let _ = transport.read_message(data, &mut payload_buf);
});

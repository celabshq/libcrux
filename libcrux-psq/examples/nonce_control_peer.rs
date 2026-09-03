//! Interop test helper process.
//!
//! This binary plays one side (`initiator` or `responder`) of a PSQ
//! registration handshake followed by one transport message round trip,
//! exchanging wire messages with its peer over stdin/stdout using a
//! trivial length-prefixed framing.
//!
//! It is built and run twice by `tests/nonce_control_interop.rs` -- once
//! with feature `nonce-control` enabled, once without -- to verify that
//! the two builds interoperate. See that test for details.
//!
//! Not meant to be run manually.

use std::io::{Read, Write};

use libcrux_psq::{
    handshake::{builders::*, ciphersuites::*, types::*},
    session::Transport,
    Channel, IntoSession,
};

const CIPHERSUITE: CiphersuiteName =
    CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256;

const CONTEXT: &[u8] = b"nonce-control interop test";
const OUTER_AAD: &[u8] = b"outer aad";
const INNER_AAD: &[u8] = b"inner aad";
const RESPONDER_AAD: &[u8] = b"responder aad";

const REGISTRATION_PAYLOAD_INITIATOR: &[u8] = b"registration payload from initiator";
const REGISTRATION_PAYLOAD_RESPONDER: &[u8] = b"registration payload from responder";
const APP_DATA_INITIATOR: &[u8] = b"transport payload from initiator";
const APP_DATA_RESPONDER: &[u8] = b"transport payload from responder";

// The nonce value the default (non-`nonce-control`) build ends up using for
// the first message sent/received on a freshly derived transport key: the
// nonce starts at all-zero and is incremented once before the first AEAD
// operation. A build using `nonce-control` has to replicate this manually
// to interoperate with a peer that does not use `nonce-control`.
#[cfg(feature = "nonce-control")]
const ALIGNED_NONCE: [u8; 12] = {
    let mut nonce = [0u8; 12];
    nonce[11] = 1;
    nonce
};

#[cfg(feature = "nonce-control")]
fn align_sender_nonce(channel: &mut Transport) {
    channel.set_sender_nonce(&ALIGNED_NONCE);
}
#[cfg(not(feature = "nonce-control"))]
fn align_sender_nonce(_channel: &mut Transport) {}

#[cfg(feature = "nonce-control")]
fn align_receiver_nonce(channel: &mut Transport) {
    channel.set_receiver_nonce(&ALIGNED_NONCE);
}
#[cfg(not(feature = "nonce-control"))]
fn align_receiver_nonce(_channel: &mut Transport) {}

fn send_frame(out: &mut impl Write, data: &[u8]) {
    out.write_all(&(data.len() as u32).to_be_bytes())
        .expect("write frame length");
    out.write_all(data).expect("write frame body");
    out.flush().expect("flush frame");
}

fn recv_frame(inp: &mut impl Read) -> Vec<u8> {
    let mut len_buf = [0u8; 4];
    inp.read_exact(&mut len_buf).expect("read frame length");
    let len = u32::from_be_bytes(len_buf) as usize;
    let mut buf = vec![0u8; len];
    inp.read_exact(&mut buf).expect("read frame body");
    buf
}

fn run_initiator(stdin: &mut impl Read, stdout: &mut impl Write) {
    let responder_pk_bytes = recv_frame(stdin);
    let responder_pk_bytes: [u8; 32] = responder_pk_bytes
        .as_slice()
        .try_into()
        .expect("responder public key must be 32 bytes");
    let responder_pk = DHPublicKey::from_bytes(&responder_pk_bytes);

    let mut rng = rand::rng();
    let initiator_keys = DHKeyPair::new(&mut rng);

    let initiator_ciphersuite = CiphersuiteBuilder::new(CIPHERSUITE)
        .longterm_x25519_keys(&initiator_keys)
        .peer_longterm_x25519_pk(&responder_pk)
        .build_initiator_ciphersuite()
        .expect("build initiator ciphersuite");

    let mut initiator = PrincipalBuilder::new(rand::rng())
        .outer_aad(OUTER_AAD)
        .inner_aad(INNER_AAD)
        .context(CONTEXT)
        .build_registration_initiator(initiator_ciphersuite)
        .expect("build registration initiator");

    let mut msg_buf = vec![0u8; 8192];
    let mut payload_buf = vec![0u8; 4096];

    let len = initiator
        .write_message(REGISTRATION_PAYLOAD_INITIATOR, &mut msg_buf)
        .expect("write handshake message 1");
    send_frame(stdout, &msg_buf[..len]);

    let msg2 = recv_frame(stdin);
    let (_, len_payload) = initiator
        .read_message(&msg2, &mut payload_buf)
        .expect("read handshake message 2");
    assert_eq!(
        &payload_buf[..len_payload],
        REGISTRATION_PAYLOAD_RESPONDER,
        "unexpected registration response payload"
    );

    assert!(initiator.is_handshake_finished());
    let mut session = initiator.into_session().expect("derive session");
    let mut channel = session
        .transport_channel()
        .expect("derive transport channel");

    align_sender_nonce(&mut channel);
    let len = channel
        .write_message(APP_DATA_INITIATOR, &mut msg_buf)
        .expect("write transport message");
    send_frame(stdout, &msg_buf[..len]);

    align_receiver_nonce(&mut channel);
    let reply = recv_frame(stdin);
    let (_, len_payload) = channel
        .read_message(&reply, &mut payload_buf)
        .expect("read transport reply");
    assert_eq!(
        &payload_buf[..len_payload],
        APP_DATA_RESPONDER,
        "unexpected transport reply payload"
    );
}

fn run_responder(stdin: &mut impl Read, stdout: &mut impl Write) {
    let mut rng = rand::rng();
    let responder_keys = DHKeyPair::new(&mut rng);

    // Publish our long-term public key out of band, so the initiator can
    // build its ciphersuite.
    send_frame(stdout, responder_keys.pk.as_ref());

    let responder_ciphersuite = CiphersuiteBuilder::new(CIPHERSUITE)
        .longterm_x25519_keys(&responder_keys)
        .build_responder_ciphersuite()
        .expect("build responder ciphersuite");

    let mut responder = PrincipalBuilder::new(rand::rng())
        .context(CONTEXT)
        .outer_aad(RESPONDER_AAD)
        .recent_keys_upper_bound(30)
        .build_responder(responder_ciphersuite)
        .expect("build responder");

    let mut msg_buf = vec![0u8; 8192];
    let mut payload_buf = vec![0u8; 4096];

    let msg1 = recv_frame(stdin);
    let (_, len_payload) = responder
        .read_message(&msg1, &mut payload_buf)
        .expect("read handshake message 1");
    assert_eq!(
        &payload_buf[..len_payload],
        REGISTRATION_PAYLOAD_INITIATOR,
        "unexpected registration payload"
    );

    let len = responder
        .write_message(REGISTRATION_PAYLOAD_RESPONDER, &mut msg_buf)
        .expect("write handshake message 2");
    send_frame(stdout, &msg_buf[..len]);

    assert!(responder.is_handshake_finished());
    let mut session = responder.into_session().expect("derive session");
    let mut channel = session
        .transport_channel()
        .expect("derive transport channel");

    align_receiver_nonce(&mut channel);
    let msg = recv_frame(stdin);
    let (_, len_payload) = channel
        .read_message(&msg, &mut payload_buf)
        .expect("read transport message");
    assert_eq!(
        &payload_buf[..len_payload],
        APP_DATA_INITIATOR,
        "unexpected transport payload"
    );

    align_sender_nonce(&mut channel);
    let len = channel
        .write_message(APP_DATA_RESPONDER, &mut msg_buf)
        .expect("write transport reply");
    send_frame(stdout, &msg_buf[..len]);
}

fn main() {
    let role = std::env::args()
        .nth(1)
        .expect("usage: nonce_control_peer <initiator|responder>");

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    match role.as_str() {
        "initiator" => run_initiator(&mut stdin, &mut stdout),
        "responder" => run_responder(&mut stdin, &mut stdout),
        other => panic!("unknown role: {other}"),
    }

    eprintln!("{role}: OK");
}

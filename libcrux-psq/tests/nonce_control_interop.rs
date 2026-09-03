//! Verifies that a build of `libcrux-psq` with feature `nonce-control`
//! enabled interoperates on the wire with a build without it.
//!
//! `nonce-control` changes how AEAD nonces are managed (see
//! `src/aead.rs`), which is why this needs to be tested across two
//! actually differently-compiled builds rather than within a single test
//! process: Cargo only ever compiles one variant of a crate for a given
//! feature set, so a single process cannot link one copy of
//! `libcrux-psq` with `nonce-control` and another without.
//!
//! Instead, this test builds the `nonce_control_peer` example twice --
//! once with `nonce-control`, once without -- and runs a full
//! registration handshake plus one transport message round trip between
//! the two resulting processes, piping messages between their
//! stdin/stdout.

use std::{
    env,
    io::{Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    thread,
};

fn build_peer(features: &[&str]) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let variant = if features.is_empty() { "off" } else { "on" };
    let target_dir = env::temp_dir()
        .join("libcrux-psq-nonce-control-interop")
        .join(variant);

    let mut cmd = Command::new(&cargo);
    cmd.arg("build")
        .arg("--manifest-path")
        .arg(format!("{manifest_dir}/Cargo.toml"))
        .arg("--example")
        .arg("nonce_control_peer")
        .arg("--target-dir")
        .arg(&target_dir);
    for feature in features {
        cmd.arg("--features").arg(feature);
    }

    let status = cmd.status().expect("failed to invoke `cargo build`");
    assert!(
        status.success(),
        "building nonce_control_peer example (features: {features:?}) failed"
    );

    target_dir
        .join("debug")
        .join("examples")
        .join("nonce_control_peer")
}

/// Relay bytes from `from` to `to` on a background thread until EOF or a
/// write error (the peer process having exited).
fn relay(mut from: impl Read + Send + 'static, mut to: impl Write + Send + 'static) {
    thread::spawn(move || {
        let mut buf = [0u8; 4096];
        loop {
            match from.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    if to.write_all(&buf[..n]).is_err() {
                        break;
                    }
                    let _ = to.flush();
                }
            }
        }
    });
}

/// Runs a handshake between an initiator built with `initiator_features`
/// and a responder built with `responder_features`, asserting that both
/// sides complete successfully.
fn run_interop(initiator_features: &[&str], responder_features: &[&str]) {
    let initiator_bin = build_peer(initiator_features);
    let responder_bin = build_peer(responder_features);

    let mut initiator = Command::new(&initiator_bin)
        .arg("initiator")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to spawn initiator process");

    let mut responder = Command::new(&responder_bin)
        .arg("responder")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to spawn responder process");

    let initiator_stdout = initiator.stdout.take().unwrap();
    let initiator_stdin = initiator.stdin.take().unwrap();
    let responder_stdout = responder.stdout.take().unwrap();
    let responder_stdin = responder.stdin.take().unwrap();

    relay(initiator_stdout, responder_stdin);
    relay(responder_stdout, initiator_stdin);

    let initiator_status = initiator.wait().expect("initiator process did not exit");
    let responder_status = responder.wait().expect("responder process did not exit");

    assert!(
        initiator_status.success(),
        "initiator process (features: {initiator_features:?}) failed"
    );
    assert!(
        responder_status.success(),
        "responder process (features: {responder_features:?}) failed"
    );
}

#[test]
fn nonce_control_initiator_interoperates_with_default_responder() {
    run_interop(&["nonce-control"], &[]);
}

#[test]
fn default_initiator_interoperates_with_nonce_control_responder() {
    run_interop(&[], &["nonce-control"]);
}

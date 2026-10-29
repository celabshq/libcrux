//! EdDSA (Ed25519, pure mode): key generation, signature generation and verification.
//!
//! Public key validation (keyVer) is not implemented, as libcrux has no API for it.

use libcrux_kats::acvp::{
    eddsa::schema::{
        KeyGenPromptTestGroup, KeyGenResult, SigGenPromptTestGroup, SigGenResult,
        SigGenResultTestGroup, SigVerPromptTestGroup, SigVerResult,
    },
    schema_common::Prompts,
};
use serde_json::Value;

use crate::{answer, not_implemented, not_implemented_group, parse, respond_implemented, response};

/// Why libcrux does not implement EdDSA on `curve` (with `pre_hash`), or `None` if it does.
fn unimplemented(curve: &str, pre_hash: bool) -> Option<&'static str> {
    match (curve, pre_hash) {
        ("ED-25519", false) => None,
        ("ED-25519", true) => Some("Ed25519ph"),
        ("ED-448", false) => Some("Ed448"),
        ("ED-448", true) => Some("Ed448ph"),
        (c, _) => panic!("unexpected curve {c}"),
    }
}

fn generate_key_pair() -> ([u8; 32], [u8; 32]) {
    let (sk, pk) = libcrux_ed25519::generate_key_pair(&mut rand::rng()).unwrap();
    (*sk.as_ref(), *pk.as_ref())
}

pub(crate) fn keygen(prompt: Value) -> Value {
    let prompts: Prompts<KeyGenPromptTestGroup> = parse(prompt);

    respond_implemented(&prompts, |g, t| {
        if let Some(reason) = unimplemented(&g.curve, false) {
            return not_implemented(reason);
        }
        let (d, q) = generate_key_pair();
        Some(KeyGenResult {
            tcId: t.tcId,
            d: d.to_vec(),
            q: q.to_vec(),
        })
    })
}

/// One key pair per test group, as the response contains the public key `q` per group.
pub(crate) fn siggen(prompt: Value) -> Value {
    let prompts: Prompts<SigGenPromptTestGroup> = parse(prompt);

    let test_groups = prompts
        .testGroups
        .iter()
        .filter_map(|g| {
            if let Some(reason) = unimplemented(&g.curve, g.preHash) {
                not_implemented_group::<_, SigGenResult>(g.tgId, &g.tests, reason);
                return None;
            }
            let (d, q) = generate_key_pair();
            Some(SigGenResultTestGroup {
                tgId: g.tgId,
                q: q.to_vec(),
                tests: answer(g.tgId, &g.tests, |t| {
                    assert!(t.context.is_empty(), "Ed25519 does not support a context");
                    SigGenResult {
                        tcId: t.tcId,
                        signature: libcrux_ed25519::sign(&t.message, &d).unwrap().to_vec(),
                    }
                }),
            })
        })
        .collect();
    response(&prompts, test_groups)
}

pub(crate) fn sigver(prompt: Value) -> Value {
    let prompts: Prompts<SigVerPromptTestGroup> = parse(prompt);

    respond_implemented(&prompts, |g, t| {
        if let Some(reason) = unimplemented(&g.curve, g.preHash) {
            return not_implemented(reason);
        }
        let valid = match (
            <&[u8; 32]>::try_from(t.q.as_slice()),
            <&[u8; 64]>::try_from(t.signature.as_slice()),
        ) {
            (Ok(q), Ok(signature)) => libcrux_ed25519::verify(&t.message, q, signature).is_ok(),
            _ => false,
        };
        Some(SigVerResult {
            tcId: t.tcId,
            testPassed: valid,
        })
    })
}

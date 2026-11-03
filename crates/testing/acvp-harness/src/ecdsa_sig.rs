//! ECDSA (FIPS 186-5) signature generation and verification on P-256 with SHA2-256, SHA2-384
//! and SHA2-512.

use libcrux_ecdsa::{
    p256::{PublicKey, Signature},
    DigestAlgorithm,
};
use libcrux_kats::acvp::{
    ec::{pad, raw_point},
    ecdsa::schema::{
        SigGenPromptTestGroup, SigGenResult, SigGenResultTestGroup, SigVerPromptTestGroup,
        SigVerResult,
    },
    schema_common::Prompts,
};
use serde_json::Value;

use crate::{
    answer, curves, not_implemented, not_implemented_group, parse, respond_implemented, response,
    within,
};

const FIELD_LEN: usize = 32;

/// The digest for ECDSA on `curve` with `hash_alg`, or why libcrux does not implement it.
fn digest(curve: &str, hash_alg: &str) -> Result<DigestAlgorithm, String> {
    if curve != "P-256" {
        // Rejects unknown curves; libcrux only implements ECDSA on P-256.
        let _ = curves::curve(curve);
        return Err(format!("curve {curve}"));
    }
    match hash_alg {
        "SHA2-256" => Ok(DigestAlgorithm::Sha256),
        "SHA2-384" => Ok(DigestAlgorithm::Sha384),
        "SHA2-512" => Ok(DigestAlgorithm::Sha512),
        "SHA2-224" | "SHA2-512/224" | "SHA2-512/256" | "SHA3-224" | "SHA3-256" | "SHA3-384"
        | "SHA3-512" | "SHAKE-128" | "SHAKE-256" => Err(format!("hashAlg {hash_alg}")),
        h => panic!("unexpected hashAlg {h}"),
    }
}

/// The digest for the signature generation of group `g`, or why libcrux does not implement it.
fn siggen_digest(g: &SigGenPromptTestGroup) -> Result<DigestAlgorithm, String> {
    let digest = digest(&g.curve, &g.hashAlg)?;
    if g.componentTest {
        return Err("componentTest (signing a digest)".to_string());
    }
    match g.conformance.as_deref() {
        None | Some("") => Ok(digest),
        Some("SP800-106") => Err("SP800-106 randomized hashing".to_string()),
        Some(c) => panic!("unexpected conformance {c}"),
    }
}

/// One key pair per test group, which is part of the response.
pub(crate) fn siggen(prompt: Value) -> Value {
    let prompts: Prompts<SigGenPromptTestGroup> = parse(prompt);

    let test_groups = prompts
        .testGroups
        .iter()
        .filter_map(|g| {
            let hash = match within(g.tgId, None, || siggen_digest(g)) {
                Ok(hash) => hash,
                Err(reason) => {
                    not_implemented_group::<_, SigGenResult>(g.tgId, &g.tests, reason);
                    return None;
                }
            };
            let (sk, pk) = within(g.tgId, None, || {
                libcrux_ecdsa::p256::rand::generate_key_pair(&mut rand::rng()).unwrap()
            });
            Some(SigGenResultTestGroup {
                tgId: g.tgId,
                qx: pk.0[..FIELD_LEN].to_vec(),
                qy: pk.0[FIELD_LEN..].to_vec(),
                tests: answer(g.tgId, &g.tests, |t| {
                    let signature =
                        libcrux_ecdsa::p256::rand::sign(hash, &t.message, &sk, &mut rand::rng())
                            .unwrap();
                    let (r, s) = signature.as_bytes();
                    SigGenResult {
                        tcId: t.tcId,
                        r: r.to_vec(),
                        s: s.to_vec(),
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
        let hash = match digest(&g.curve, &g.hashAlg) {
            Ok(hash) => hash,
            Err(reason) => return not_implemented(reason),
        };
        let valid = (|| {
            let q = raw_point(&t.qx, &t.qy, FIELD_LEN)?;
            let r = pad(&t.r, FIELD_LEN)?.try_into().ok()?;
            let s = pad(&t.s, FIELD_LEN)?.try_into().ok()?;
            let pk = PublicKey::try_from(q.as_slice()).ok()?;
            let signature = Signature::from_raw(r, s);
            Some(libcrux_ecdsa::p256::verify(hash, &t.message, &signature, &pk).is_ok())
        })()
        .unwrap_or(false);
        Some(SigVerResult {
            tcId: t.tcId,
            testPassed: valid,
        })
    })
}

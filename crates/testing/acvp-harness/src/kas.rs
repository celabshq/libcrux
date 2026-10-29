//! KAS-ECC-SSC (SP 800-56Ar3) on P-256 and P-384, with all schemes but the MQV ones.
//!
//! The shared secret is `Z = Ze || Zs`. `Ze` is the ECDH of the ephemeral key of each party, or
//! its static key if it has no ephemeral one, if any party has an ephemeral key. `Zs` is the ECDH
//! of the static keys, if both parties have one.

use libcrux_kats::acvp::{
    ec::pad,
    kas_ecc::schema::{KasPrompt, KasPromptTestGroup, KasResult},
    schema_common::Prompts,
};
use serde_json::Value;

use crate::{
    curves::{curve, Curve},
    not_implemented, parse,
};

/// Whether a party contributes an `(ephemeral, static)` key pair.
type Keys = (bool, bool);

/// The keys of the IUT and of the server, or `None` for the MQV schemes.
fn keys(scheme: &str, role: &str) -> Option<(Keys, Keys)> {
    let (initiator, responder) = match scheme {
        "ephemeralUnified" => ((true, false), (true, false)),
        "fullUnified" => ((true, true), (true, true)),
        "onePassDh" => ((true, false), (false, true)),
        "onePassUnified" => ((true, true), (false, true)),
        "staticUnified" => ((false, true), (false, true)),
        "fullMqv" | "onePassMqv" => return None,
        s => panic!("unexpected scheme {s}"),
    };
    match role {
        "initiator" => Some((initiator, responder)),
        "responder" => Some((responder, initiator)),
        r => panic!("unexpected kasRole {r}"),
    }
}

/// The public key `(x, y)` if `present`, checking that the prompt has it exactly then.
fn public_key<'a>(
    present: bool,
    x: &'a [u8],
    y: &'a [u8],
    what: &str,
) -> Option<(&'a [u8], &'a [u8])> {
    if present {
        assert!(!x.is_empty() && !y.is_empty(), "missing {what}");
        Some((x, y))
    } else {
        assert!(x.is_empty() && y.is_empty(), "unexpected {what}");
        None
    }
}

/// The server public keys `(ephemeral, static)` of `t`.
type ServerKeys<'a> = (Option<(&'a [u8], &'a [u8])>, Option<(&'a [u8], &'a [u8])>);

fn server_keys(server: Keys, t: &KasPrompt) -> ServerKeys<'_> {
    (
        public_key(
            server.0,
            &t.ephemeralPublicServerX,
            &t.ephemeralPublicServerY,
            "ephemeral server key",
        ),
        public_key(
            server.1,
            &t.staticPublicServerX,
            &t.staticPublicServerY,
            "static server key",
        ),
    )
}

/// `Z = Ze || Zs` for the IUT private keys `(ephemeral, static)`, or `None` if a key is invalid.
fn shared_secret(
    curve: &Curve,
    iut: (Option<&[u8]>, Option<&[u8]>),
    server: ServerKeys,
) -> Option<Vec<u8>> {
    let mut z = Vec::new();
    if iut.0.is_some() || server.0.is_some() {
        let d = iut.0.or(iut.1)?;
        let (x, y) = server.0.or(server.1)?;
        z.extend((curve.ecdh)(d, x, y)?);
    }
    if let (Some(d), Some((x, y))) = (iut.1, server.1) {
        z.extend((curve.ecdh)(d, x, y)?);
    }
    Some(z)
}

/// Whether `(x, y)` is the public key of `d`.
fn is_key_pair(curve: &Curve, d: &[u8], x: &[u8], y: &[u8]) -> bool {
    let len = curve.field_len;
    (|| {
        let (px, py) = (curve.public_key)(d)?;
        Some(pad(x, len)? == px && pad(y, len)? == py)
    })()
    .unwrap_or(false)
}

pub(crate) fn respond(prompt: Value) -> Value {
    // With a `hashFunctionZ`, the IUT answers with the hash of `z`, which is not supported.
    for g in prompt["testGroups"].as_array().into_iter().flatten() {
        let hash = &g["hashFunctionZ"];
        assert!(
            hash.is_null() || hash == "none",
            "tgId {}: hashFunctionZ {hash} is not supported",
            g["tgId"]
        );
    }
    let prompts: Prompts<KasPromptTestGroup> = parse(prompt);

    crate::respond_implemented(&prompts, |g, t| {
        let Some((iut, server)) = keys(&g.scheme, &g.kasRole) else {
            return not_implemented(&g.scheme);
        };
        let Some(curve) = curve(&g.domainParameterGenerationMode) else {
            return not_implemented(format!("curve {}", g.domainParameterGenerationMode));
        };
        let server = server_keys(server, t);
        Some(match g.testType.as_str() {
            "AFT" => {
                let ephemeral = iut.0.then(curve.generate);
                let static_ = iut.1.then(curve.generate);
                let z = shared_secret(
                    &curve,
                    (
                        ephemeral.as_ref().map(|k| k.0.as_slice()),
                        static_.as_ref().map(|k| k.0.as_slice()),
                    ),
                    server,
                )
                .expect("invalid server public key");
                let (ephemeral_x, ephemeral_y) = ephemeral.map(|(_, x, y)| (x, y)).unzip();
                let (static_x, static_y) = static_.map(|(_, x, y)| (x, y)).unzip();
                KasResult {
                    tcId: t.tcId,
                    ephemeralPublicIutX: ephemeral_x,
                    ephemeralPublicIutY: ephemeral_y,
                    staticPublicIutX: static_x,
                    staticPublicIutY: static_y,
                    z: Some(z),
                    testPassed: None,
                }
            }
            "VAL" => {
                assert!(!t.z.is_empty(), "missing z");
                fn private_key<'a>(
                    curve: &Curve,
                    present: bool,
                    d: &'a [u8],
                    x: &[u8],
                    y: &[u8],
                    what: &str,
                ) -> Option<(&'a [u8], bool)> {
                    public_key(present, x, y, what)?;
                    Some((d, is_key_pair(curve, d, x, y)))
                }
                let ephemeral = private_key(
                    &curve,
                    iut.0,
                    &t.ephemeralPrivateIut,
                    &t.ephemeralPublicIutX,
                    &t.ephemeralPublicIutY,
                    "ephemeral IUT key",
                );
                let static_ = private_key(
                    &curve,
                    iut.1,
                    &t.staticPrivateIut,
                    &t.staticPublicIutX,
                    &t.staticPublicIutY,
                    "static IUT key",
                );
                let keys_valid = [ephemeral, static_].iter().flatten().all(|k| k.1);
                let z = shared_secret(
                    &curve,
                    (ephemeral.map(|k| k.0), static_.map(|k| k.0)),
                    server,
                );
                let passed =
                    keys_valid && z.is_some_and(|z| pad(&t.z, z.len()).is_some_and(|tz| tz == z));
                KasResult {
                    tcId: t.tcId,
                    ephemeralPublicIutX: None,
                    ephemeralPublicIutY: None,
                    staticPublicIutX: None,
                    staticPublicIutY: None,
                    z: None,
                    testPassed: Some(passed),
                }
            }
            tt => panic!("unexpected testType {tt}"),
        })
    })
}

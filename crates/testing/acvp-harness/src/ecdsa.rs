//! ECDSA (FIPS 186-5) key generation and public key validation on P-256 and P-384.

use libcrux_kats::acvp::{
    ecdsa::schema::{KeyGenPromptTestGroup, KeyGenResult, KeyVerPromptTestGroup, KeyVerResult},
    schema_common::Prompts,
};
use serde_json::Value;

use crate::{curves::curve, not_implemented, parse, respond_implemented};

/// The key generation of libcrux uses rejection sampling ("testing candidates"). P-384 follows
/// SP 800-56A r3, Section 5.6.1.2.2 (`d = c + 1` for a candidate `c <= n - 2`). P-256 samples
/// `d` in `[1, n - 1]` directly, which gives the same distribution, but is not literally that
/// procedure.
pub(crate) fn keygen(prompt: Value) -> Value {
    let prompts: Prompts<KeyGenPromptTestGroup> = parse(prompt);

    respond_implemented(&prompts, |g, t| {
        let Some(curve) = curve(&g.curve) else {
            return not_implemented(format!("curve {}", g.curve));
        };
        match g.secretGenerationMode.as_str() {
            "testing candidates" => (),
            "extra bits" => return not_implemented("secretGenerationMode extra bits"),
            m => panic!("unexpected secretGenerationMode {m}"),
        }
        let (d, qx, qy) = (curve.ecdsa_generate)();
        Some(KeyGenResult {
            tcId: t.tcId,
            d,
            qx,
            qy,
        })
    })
}

pub(crate) fn keyver(prompt: Value) -> Value {
    let prompts: Prompts<KeyVerPromptTestGroup> = parse(prompt);

    respond_implemented(&prompts, |g, t| {
        let Some(curve) = curve(&g.curve) else {
            return not_implemented(format!("curve {}", g.curve));
        };
        Some(KeyVerResult {
            tcId: t.tcId,
            testPassed: (curve.validate)(&t.qx, &t.qy),
        })
    })
}

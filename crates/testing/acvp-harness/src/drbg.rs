//! HMAC_DRBG (SP 800-90Ar1) with SHA2-256, SHA2-384 and SHA2-512, with and without prediction
//! resistance.

use libcrux_hmac_drbg::{HmacDrbgSha256, HmacDrbgSha384, HmacDrbgSha512};
use libcrux_kats::acvp::{
    drbg::schema::{DrbgPromptTestGroup, DrbgResult},
    schema_common::Prompts,
};
use serde_json::Value;

use crate::{not_implemented, parse};

/// Instantiate a `$drbg`, then run the `reSeed` and `generate` operations of `$test` in order.
///
/// Evaluates to the output of the last `generate` call.
macro_rules! run {
    ($drbg:ty, $test:expr, $pred_resistance:expr, $returned_bytes:expr) => {{
        let test = $test;
        let mut drbg = <$drbg>::new(&test.entropyInput, &test.nonce, &test.persoString).unwrap();

        let mut out = vec![0u8; $returned_bytes];
        for op in &test.otherInput {
            match op.intendedUse.as_str() {
                "reSeed" => drbg.reseed(&op.entropyInput, &op.additionalInput).unwrap(),
                // With prediction resistance, generate first reseeds with fresh entropy and
                // the additional input, then generates without additional input
                // (SP 800-90A, Section 9.3.1).
                "generate" if $pred_resistance => {
                    drbg.reseed(&op.entropyInput, &op.additionalInput).unwrap();
                    drbg.generate(&mut out, &[]).unwrap();
                }
                "generate" => {
                    assert!(op.entropyInput.is_empty());
                    drbg.generate(&mut out, &op.additionalInput).unwrap();
                }
                u => panic!("unexpected intendedUse {u}"),
            }
        }
        out
    }};
}

pub(crate) fn respond(prompt: Value) -> Value {
    let prompts: Prompts<DrbgPromptTestGroup> = parse(prompt);

    crate::respond_implemented(&prompts, |g, t| {
        assert!(!g.derFunc);
        assert_eq!(g.returnedBitsLen % 8, 0);
        let returned_bytes = g.returnedBitsLen / 8;
        let pr = g.predResistance;
        let returned_bits = match g.mode.as_str() {
            "SHA2-256" => run!(HmacDrbgSha256, t, pr, returned_bytes),
            "SHA2-384" => run!(HmacDrbgSha384, t, pr, returned_bytes),
            "SHA2-512" => run!(HmacDrbgSha512, t, pr, returned_bytes),
            "SHA-1" | "SHA2-224" | "SHA2-512/224" | "SHA2-512/256" | "SHA3-224" | "SHA3-256"
            | "SHA3-384" | "SHA3-512" => return not_implemented(format!("mode {}", g.mode)),
            m => panic!("unexpected mode {m}"),
        };
        Some(DrbgResult {
            tcId: t.tcId,
            returnedBits: returned_bits,
        })
    })
}

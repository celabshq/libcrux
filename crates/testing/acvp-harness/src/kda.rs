//! SP 800-56Cr2 two-step KDA with an HMAC and a feedback mode expansion with a 32-bit counter
//! before the fixed data.

use libcrux_hmac::{
    HmacSha256, HmacSha384, HmacSha3_224, HmacSha3_256, HmacSha3_384, HmacSha3_512, HmacSha512,
    HmacState,
};
use libcrux_kats::acvp::{
    kda::schema::{TwoStepPrompt, TwoStepPromptTestGroup, TwoStepResult},
    schema_common::{Prompts, TestCase},
};
use libcrux_nist_kdf::two_step;
use serde_json::Value;

use crate::{not_implemented, parse};

/// The KDF for an expansion with `mac` in `kdf_mode`, with a `counter_len`-bit counter at
/// `counter_location`, or why libcrux does not implement it.
fn expansion(
    kdf_mode: &str,
    mac: &str,
    counter_len: usize,
    counter_location: &str,
) -> Result<TwoStepKdf, String> {
    if kdf_mode != "feedback" {
        return Err(format!("kdfMode {kdf_mode}"));
    }
    if counter_len != 32 || counter_location != "before fixed data" {
        return Err(format!("{counter_len}-bit counter {counter_location}"));
    }
    two_step_kdf_for(mac).ok_or_else(|| format!("macMode {mac}"))
}

/// The shared secret `z`, or `z || t` with a hybrid shared secret.
fn shared_secret(hybrid: bool, z: &[u8], t: &[u8]) -> Vec<u8> {
    assert_eq!(
        hybrid,
        !t.is_empty(),
        "t must be given exactly for a hybrid secret"
    );
    [z, t].concat()
}

/// `two_step::kdf(k_out, z, salt, iv, fixed_info)` with the HMAC fixed.
type TwoStepKdf = fn(&mut [u8], &[u8], &[u8], &[u8], &[&[u8]]);

fn two_step_kdf<const OUTLEN: usize, H: HmacState<OUTLEN>>(
    k_out: &mut [u8],
    z: &[u8],
    salt: &[u8],
    iv: &[u8],
    fixed_info: &[&[u8]],
) {
    two_step::kdf::<OUTLEN, H>(k_out, z, salt, iv, fixed_info).unwrap();
}

fn two_step_kdf_for(mac: &str) -> Option<TwoStepKdf> {
    Some(match mac {
        "HMAC-SHA2-256" => two_step_kdf::<32, HmacSha256>,
        "HMAC-SHA2-384" => two_step_kdf::<48, HmacSha384>,
        "HMAC-SHA2-512" => two_step_kdf::<64, HmacSha512>,
        "HMAC-SHA3-224" => two_step_kdf::<28, HmacSha3_224>,
        "HMAC-SHA3-256" => two_step_kdf::<32, HmacSha3_256>,
        "HMAC-SHA3-384" => two_step_kdf::<48, HmacSha3_384>,
        "HMAC-SHA3-512" => two_step_kdf::<64, HmacSha3_512>,
        "HMAC-SHA-1" | "HMAC-SHA2-224" | "HMAC-SHA2-512/224" | "HMAC-SHA2-512/256"
        | "CMAC-AES128" | "CMAC-AES192" | "CMAC-AES256" => return None,
        mac => panic!("unexpected macMode {mac}"),
    })
}

/// The length `l` in bytes.
fn bytes(l: usize) -> usize {
    assert_eq!(l % 8, 0, "only byte-oriented output lengths");
    l / 8
}

pub(crate) fn respond(prompt: Value) -> Value {
    let prompts: Prompts<TwoStepPromptTestGroup> = parse(prompt);

    crate::respond_implemented(&prompts, |g, t| {
        let mut result = TwoStepResult {
            tcId: t.tc_id(),
            dkm: None,
            dkms: None,
            testPassed: None,
        };
        match t {
            TwoStepPrompt::Single(t) => {
                let config = g.kdfConfiguration.as_ref().unwrap();
                let kdf = match expansion(
                    &config.kdfMode,
                    &config.macMode,
                    config.counterLen,
                    &config.counterLocation,
                ) {
                    Ok(kdf) => kdf,
                    Err(reason) => return not_implemented(reason),
                };
                assert_eq!(config.fixedInfoEncoding, "concatenation");

                let param = &t.kdfParameter;
                let z = shared_secret(g.usesHybridSharedSecret, &param.z, &param.t);
                let mut dkm = vec![0; bytes(param.l)];
                kdf(
                    &mut dkm,
                    &z,
                    &param.salt,
                    &param.iv,
                    &[&t.fixed_info(&config.fixedInfoPattern)],
                );
                match g.testType.as_str() {
                    "AFT" => result.dkm = Some(dkm),
                    "VAL" => result.testPassed = Some(t.dkm.as_ref() == Some(&dkm)),
                    tt => panic!("unexpected testType {tt}"),
                }
            }
            TwoStepPrompt::MultiExpansion(t) => {
                let param = &t.kdfMultiExpansionParameter;
                let kdf = match expansion(
                    &param.kdfMode,
                    &param.macMode,
                    param.counterLen,
                    &param.counterLocation,
                ) {
                    Ok(kdf) => kdf,
                    Err(reason) => return not_implemented(reason),
                };
                let z = shared_secret(g.usesHybridSharedSecret, &param.z, &param.t);
                let dkms: Vec<Vec<u8>> = param
                    .iterationParameters
                    .iter()
                    .map(|it| {
                        let mut dkm = vec![0; bytes(it.l)];
                        kdf(&mut dkm, &z, &param.salt, &param.iv, &[&it.fixedInfo]);
                        dkm
                    })
                    .collect();
                match g.testType.as_str() {
                    "AFT" => result.dkms = Some(dkms),
                    "VAL" => result.testPassed = Some(t.dkms.as_ref() == Some(&dkms)),
                    tt => panic!("unexpected testType {tt}"),
                }
            }
        }
        Some(result)
    })
}

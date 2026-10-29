//! SP 800-108 KBKDF (`KDF` revision 1.0) in feedback mode with an HMAC and a 32-bit counter
//! before the fixed data.

use libcrux_hmac::{
    HmacSha256, HmacSha384, HmacSha3_224, HmacSha3_256, HmacSha3_384, HmacSha3_512, HmacSha512,
    HmacState,
};
use libcrux_kats::acvp::{
    kbkdf::schema::{KbkdfPromptTestGroup, KbkdfResult},
    schema_common::Prompts,
};
use libcrux_nist_kdf::feedback;
use serde_json::Value;

use crate::{not_implemented, parse};

/// `kdf(k_out, k_in, iv, fixed_data)`.
type Kdf = fn(&mut [u8], &[u8], &[u8], &[u8]);

fn kdf<const OUTLEN: usize, H: HmacState<OUTLEN>>(
    k_out: &mut [u8],
    k_in: &[u8],
    iv: &[u8],
    fixed_data: &[u8],
) {
    feedback::kdf::<OUTLEN, H>(k_out, k_in, iv, &[fixed_data]).unwrap();
}

fn kdf_for(mac: &str) -> Option<Kdf> {
    Some(match mac {
        "HMAC-SHA2-256" => kdf::<32, HmacSha256>,
        "HMAC-SHA2-384" => kdf::<48, HmacSha384>,
        "HMAC-SHA2-512" => kdf::<64, HmacSha512>,
        "HMAC-SHA3-224" => kdf::<28, HmacSha3_224>,
        "HMAC-SHA3-256" => kdf::<32, HmacSha3_256>,
        "HMAC-SHA3-384" => kdf::<48, HmacSha3_384>,
        "HMAC-SHA3-512" => kdf::<64, HmacSha3_512>,
        _ => return None,
    })
}

/// The fixed data is chosen by the IUT and returned with the derived key.
pub(crate) fn respond(prompt: Value) -> Value {
    let prompts: Prompts<KbkdfPromptTestGroup> = parse(prompt);

    crate::respond_implemented(&prompts, |g, t| {
        assert_eq!(g.testType, "AFT");
        if g.kdfMode != "feedback" {
            return not_implemented(format!("kdfMode {}", g.kdfMode));
        }
        if g.counterLength == 0 {
            return not_implemented("no counter");
        }
        if g.counterLength != 32 || g.counterLocation != "before fixed data" {
            return not_implemented(format!(
                "{}-bit counter {}",
                g.counterLength, g.counterLocation
            ));
        }
        let Some(kdf) = kdf_for(&g.macMode) else {
            return not_implemented(format!("macMode {}", g.macMode));
        };
        if g.keyOutLength % 8 != 0 {
            return not_implemented("keyOutLength not a multiple of 8 bits");
        }
        let fixed_data: [u8; 32] = rand::random();
        let mut key_out = vec![0u8; g.keyOutLength / 8];
        kdf(&mut key_out, &t.keyIn, &t.iv, &fixed_data);
        Some(KbkdfResult {
            tcId: t.tcId,
            fixedData: fixed_data.to_vec(),
            keyOut: key_out,
        })
    })
}

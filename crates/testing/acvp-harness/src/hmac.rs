//! HMAC with SHA-2 and SHA-3: full-length MACs through the incremental API, truncated ones
//! through [`libcrux_hmac::hmac`].

use libcrux_hmac::{
    Algorithm, HmacSha256, HmacSha384, HmacSha3_224, HmacSha3_256, HmacSha3_384, HmacSha3_512,
    HmacSha512, HmacState,
};
use libcrux_kats::acvp::{
    hmac::schema::{HmacPromptTestGroup, HmacResult},
    schema_common::Prompts,
};
use serde_json::Value;

use crate::{not_implemented, parse};

fn mac<const OUTLEN: usize, Mac: HmacState<OUTLEN>>(key: &[u8], msg: &[u8]) -> Vec<u8> {
    let mut tag = [0u8; OUTLEN];
    let mut state = Mac::new(key).unwrap();
    state.update(msg).unwrap();
    state.finalize(&mut tag);
    tag.to_vec()
}

/// `mac(key, msg)`.
type MacFn = fn(&[u8], &[u8]) -> Vec<u8>;

pub(crate) fn respond(prompt: Value, algorithm: &str) -> Value {
    let (mac, alg, out_len): (MacFn, Algorithm, usize) = match algorithm {
        "HMAC-SHA2-256" => (mac::<32, HmacSha256>, Algorithm::Sha256, 32),
        "HMAC-SHA2-384" => (mac::<48, HmacSha384>, Algorithm::Sha384, 48),
        "HMAC-SHA2-512" => (mac::<64, HmacSha512>, Algorithm::Sha512, 64),
        "HMAC-SHA3-224" => (mac::<28, HmacSha3_224>, Algorithm::Sha3_224, 28),
        "HMAC-SHA3-256" => (mac::<32, HmacSha3_256>, Algorithm::Sha3_256, 32),
        "HMAC-SHA3-384" => (mac::<48, HmacSha3_384>, Algorithm::Sha3_384, 48),
        "HMAC-SHA3-512" => (mac::<64, HmacSha3_512>, Algorithm::Sha3_512, 64),
        a => panic!("unexpected algorithm {a}"),
    };
    let prompts: Prompts<HmacPromptTestGroup> = parse(prompt);

    crate::respond_implemented(&prompts, |g, t| {
        assert_eq!(g.testType, "AFT");
        assert_eq!(t.keyLen, t.key.len() * 8, "only byte-oriented keys");
        assert_eq!(
            t.msgLen.div_ceil(8),
            t.msg.len(),
            "msgLen does not match msg"
        );
        if t.msgLen % 8 != 0 {
            return not_implemented("bit-oriented message");
        }
        assert!(
            t.macLen % 8 == 0 && t.macLen <= out_len * 8,
            "unexpected macLen {}",
            t.macLen
        );
        let mac = if t.macLen == out_len * 8 {
            mac(&t.key, &t.msg)
        } else {
            let mut tag = vec![0u8; t.macLen / 8];
            libcrux_hmac::hmac(alg, &t.key, &t.msg, &mut tag);
            tag
        };
        Some(HmacResult { tcId: t.tcId, mac })
    })
}

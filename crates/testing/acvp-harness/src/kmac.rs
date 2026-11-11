//! KMAC-128 and KMAC-256 (SP 800-185), without XOF.

use libcrux_kats::acvp::{
    kmac::schema::{KmacPromptTestGroup, KmacResult},
    schema_common::Prompts,
};
use serde_json::Value;

use crate::parse;

/// `kmac(tag, key, data, customization)`.
type KmacFn = for<'a> fn(&'a mut [u8], &[u8], &[u8], &[u8]) -> &'a [u8];

pub(crate) fn respond(prompt: Value, algorithm: &str) -> Value {
    let kmac: KmacFn = match algorithm {
        "KMAC-128" => libcrux_kmac::kmac_128,
        "KMAC-256" => libcrux_kmac::kmac_256,
        a => panic!("unexpected algorithm {a}"),
    };
    let prompts: Prompts<KmacPromptTestGroup> = parse(prompt);

    crate::respond_implemented(&prompts, |g, t| {
        if g.xof {
            return crate::not_implemented("XOF");
        }
        assert_eq!(t.keyLen, t.key.len() * 8, "only byte-oriented keys");
        assert_eq!(t.msgLen, t.msg.len() * 8, "only byte-oriented messages");
        assert_eq!(t.macLen % 8, 0, "only byte-oriented MACs");
        let mut mac = vec![0u8; t.macLen / 8];
        kmac(&mut mac, &t.key, &t.msg, g.customization(t));
        let (mac, passed) = match g.testType.as_str() {
            "AFT" => (Some(mac), None),
            "MVT" => (None, Some(mac == t.mac)),
            tt => panic!("unexpected testType {tt}"),
        };
        Some(KmacResult {
            tcId: t.tcId,
            mac,
            testPassed: passed,
        })
    })
}

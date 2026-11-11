//! HKDF (`KDA` `HKDF` `Sp800-56Cr2`) with SHA-2 and SHA-3, with single and multiple
//! expansions and hybrid shared secrets `z || t`.

use libcrux_hkdf::Algorithm;
use libcrux_kats::acvp::{
    kda::{
        hkdf_schema::{
            HkdfAnyPromptTestGroup, HkdfMultiExpansionPromptTestGroup, HkdfPromptTestGroup,
        },
        schema::TwoStepResult,
    },
    schema_common::{Prompts, TestGroupResults},
};
use libcrux_secrets::{ClassifyRef, ClassifyRefMut};
use serde_json::Value;

use crate::{answer_implemented, not_implemented, parse, response};

/// The shared secret `z`, or `z || t` with a hybrid shared secret.
fn shared_secret(hybrid: bool, z: &[u8], t: &[u8]) -> Vec<u8> {
    assert_eq!(
        hybrid,
        !t.is_empty(),
        "t must be given exactly for a hybrid secret"
    );
    [z, t].concat()
}

/// The result of an `AFT` or `VAL` test.
fn result(
    test_type: &str,
    tc_id: usize,
    dkm: Option<Vec<u8>>,
    dkms: Option<Vec<Vec<u8>>>,
    passed: impl FnOnce() -> bool,
) -> TwoStepResult {
    let mut result = TwoStepResult {
        tcId: tc_id,
        dkm: None,
        dkms: None,
        testPassed: None,
    };
    match test_type {
        "AFT" => (result.dkm, result.dkms) = (dkm, dkms),
        "VAL" => result.testPassed = Some(passed()),
        tt => panic!("unexpected testType {tt}"),
    }
    result
}

fn single(g: &HkdfPromptTestGroup) -> Vec<TwoStepResult> {
    let config = &g.kdfConfiguration;
    answer_implemented(g.tgId, &g.tests, |t| {
        let Some(alg) = algorithm(&config.hmacAlg) else {
            return not_implemented(format!("hmacAlg {}", config.hmacAlg));
        };
        assert_eq!(config.fixedInfoEncoding, "concatenation");
        let param = &t.kdfParameter;
        assert_eq!(param.l % 8, 0, "only byte-oriented output lengths");
        let ikm = shared_secret(g.usesHybridSharedSecret, &param.z, &param.t);
        let mut dkm = vec![0u8; param.l / 8];
        libcrux_hkdf::hkdf(
            alg,
            dkm.as_mut_slice().classify_ref_mut(),
            param.salt.classify_ref(),
            ikm.classify_ref(),
            &t.fixed_info(&config.fixedInfoPattern),
        )
        .unwrap();
        Some(result(&g.testType, t.tcId, Some(dkm.clone()), None, || {
            t.dkm.as_ref() == Some(&dkm)
        }))
    })
}

/// One extraction, then one expansion per iteration.
fn multi_expansion(g: &HkdfMultiExpansionPromptTestGroup) -> Vec<TwoStepResult> {
    answer_implemented(g.tgId, &g.tests, |t| {
        let param = &t.kdfMultiExpansionParameter;
        let Some(alg) = algorithm(&param.hmacAlg) else {
            return not_implemented(format!("hmacAlg {}", param.hmacAlg));
        };
        let ikm = shared_secret(g.usesHybridSharedSecret, &param.z, &param.t);
        let mut prk = vec![0u8; alg.hash_len()];
        libcrux_hkdf::extract(
            alg,
            prk.as_mut_slice().classify_ref_mut(),
            param.salt.classify_ref(),
            ikm.classify_ref(),
        )
        .unwrap();
        let dkms: Vec<Vec<u8>> = param
            .iterationParameters
            .iter()
            .map(|it| {
                assert_eq!(it.l % 8, 0, "only byte-oriented output lengths");
                let mut dkm = vec![0u8; it.l / 8];
                libcrux_hkdf::expand(
                    alg,
                    dkm.as_mut_slice().classify_ref_mut(),
                    prk.classify_ref(),
                    &it.fixedInfo,
                )
                .unwrap();
                dkm
            })
            .collect();
        Some(result(
            &g.testType,
            t.tcId,
            None,
            Some(dkms.clone()),
            || t.dkms.as_ref() == Some(&dkms),
        ))
    })
}

fn algorithm(hmac_alg: &str) -> Option<Algorithm> {
    Some(match hmac_alg {
        "SHA2-256" => Algorithm::Sha256,
        "SHA2-384" => Algorithm::Sha384,
        "SHA2-512" => Algorithm::Sha512,
        "SHA3-224" => Algorithm::Sha3_224,
        "SHA3-256" => Algorithm::Sha3_256,
        "SHA3-384" => Algorithm::Sha3_384,
        "SHA3-512" => Algorithm::Sha3_512,
        "SHA-1" | "SHA2-224" | "SHA2-512/224" | "SHA2-512/256" => return None,
        a => panic!("unexpected hmacAlg {a}"),
    })
}

pub(crate) fn respond(prompt: Value) -> Value {
    let prompts: Prompts<HkdfAnyPromptTestGroup> = parse(prompt);

    let test_groups: Vec<_> = prompts
        .testGroups
        .iter()
        .map(|g| match g {
            HkdfAnyPromptTestGroup::Single(g) => {
                assert!(!g.multiExpansion);
                TestGroupResults {
                    tgId: g.tgId,
                    tests: single(g),
                }
            }
            HkdfAnyPromptTestGroup::MultiExpansion(g) => {
                assert!(g.multiExpansion);
                TestGroupResults {
                    tgId: g.tgId,
                    tests: multi_expansion(g),
                }
            }
        })
        .filter(|g| !g.tests.is_empty())
        .collect();
    response(&prompts, test_groups)
}

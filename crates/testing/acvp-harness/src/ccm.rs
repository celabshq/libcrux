//! AES-CCM with 128 and 256 bit keys and 64 (short) and 128 bit tags.

use libcrux_aes::{aes_ccm_128, aes_ccm_256, Aead};
use libcrux_kats::acvp::{
    aes::ccm_schema::{CcmPrompt, CcmPromptTestGroup, CcmResult},
    schema_common::{Prompts, TestGroupResults},
};
use serde_json::Value;

use crate::{
    answer_implemented, not_implemented, not_implemented_group, parse, response, unsupported,
    Backend,
};

/// Answer the test `t` of group `g` with `cipher`. Ciphertexts in ACVP include the tag.
fn ccm_group<Cipher: Aead>(cipher: Cipher, g: &CcmPromptTestGroup) -> TestGroupResults<CcmResult> {
    TestGroupResults {
        tgId: g.tgId,
        tests: answer_implemented(g.tgId, &g.tests, |t| ccm_test(&cipher, g, t)),
    }
}

fn ccm_test<Cipher: Aead>(
    cipher: &Cipher,
    g: &CcmPromptTestGroup,
    t: &CcmPrompt,
) -> Option<CcmResult> {
    assert_eq!(g.testType, "AFT");
    if g.ivLen != 96 {
        return not_implemented(format!("ivLen {}", g.ivLen));
    }
    assert_eq!(g.payloadLen % 8, 0, "only byte-oriented payloads");
    assert_eq!(t.aad.len() * 8, g.aadLen, "only byte-oriented AAD");
    let key = cipher.new_key(&t.key).unwrap();
    let nonce = cipher.new_nonce(&t.iv).unwrap();
    let mut result = CcmResult {
        tcId: t.tcId,
        ct: None,
        pt: None,
        testPassed: None,
    };
    match g.direction.as_str() {
        "encrypt" => {
            assert_eq!(t.pt.len() * 8, g.payloadLen);
            let mut ct = vec![0u8; t.pt.len()];
            let mut tag = vec![0u8; cipher.tag_len()];
            key.encrypt(
                &mut ct,
                cipher.new_tag_mut(&mut tag).unwrap(),
                nonce,
                &t.aad,
                &t.pt,
            )
            .unwrap();
            result.ct = Some([ct, tag].concat());
        }
        "decrypt" => {
            let ct_len =
                t.ct.len()
                    .checked_sub(cipher.tag_len())
                    .expect("the ciphertext is shorter than the tag");
            assert_eq!(ct_len * 8, g.payloadLen);
            let (ct, tag) = t.ct.split_at(ct_len);
            let mut pt = vec![0u8; ct.len()];
            let tag = cipher.new_tag(tag).unwrap();
            match key.decrypt(&mut pt, nonce, &t.aad, ct, tag) {
                Ok(()) => result.pt = Some(pt),
                Err(_) => result.testPassed = Some(false),
            }
        }
        d => panic!("unexpected direction {d}"),
    }
    Some(result)
}

/// Answer the group `$g` with the implementation for its key and tag length.
macro_rules! ccm_for {
    ($g:expr, $c128:expr, $c128_short:expr, $c256:expr, $c256_short:expr) => {
        match ($g.keyLen, $g.tagLen) {
            (128, 128) => ccm_group($c128, $g),
            (128, 64) => ccm_group($c128_short, $g),
            (256, 128) => ccm_group($c256, $g),
            (256, 64) => ccm_group($c256_short, $g),
            (192, _) => not_implemented_group($g.tgId, &$g.tests, "AES-192"),
            (128 | 256, t) => not_implemented_group($g.tgId, &$g.tests, format!("tagLen {t}")),
            (k, _) => panic!("unexpected key length {k}"),
        }
    };
}

pub(crate) fn ccm(prompt: Value, backend: Backend) -> Value {
    let prompts: Prompts<CcmPromptTestGroup> = parse(prompt);
    let test_groups = prompts
        .testGroups
        .iter()
        .map(|g| match backend {
            Backend::Default => ccm_for!(
                g,
                libcrux_aes::AesCcm128,
                aes_ccm_128::short_tag::AesCcm128ShortTag,
                libcrux_aes::AesCcm256,
                aes_ccm_256::short_tag::AesCcm256ShortTag
            ),
            Backend::Portable => ccm_for!(
                g,
                aes_ccm_128::portable::PortableAesCcm128,
                aes_ccm_128::short_tag::portable::PortableAesCcm128ShortTag,
                aes_ccm_256::portable::PortableAesCcm256,
                aes_ccm_256::short_tag::portable::PortableAesCcm256ShortTag
            ),
            #[cfg(feature = "simd128")]
            Backend::Neon => ccm_for!(
                g,
                aes_ccm_128::neon::NeonAesCcm128,
                aes_ccm_128::short_tag::neon::NeonAesCcm128ShortTag,
                aes_ccm_256::neon::NeonAesCcm256,
                aes_ccm_256::short_tag::neon::NeonAesCcm256ShortTag
            ),
            #[cfg(feature = "simd256")]
            Backend::Avx2 => ccm_for!(
                g,
                aes_ccm_128::x64::X64AesCcm128,
                aes_ccm_128::short_tag::x64::X64AesCcm128ShortTag,
                aes_ccm_256::x64::X64AesCcm256,
                aes_ccm_256::short_tag::x64::X64AesCcm256ShortTag
            ),
            #[allow(unreachable_patterns)]
            _ => unsupported("ACVP-AES-CCM", backend),
        })
        .filter(|g| !g.tests.is_empty())
        .collect();
    response(&prompts, test_groups)
}

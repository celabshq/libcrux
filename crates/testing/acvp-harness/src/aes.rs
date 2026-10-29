//! AES-ECB (the block cipher, encryption only) and AES-GCM, with 128 and 256 bit keys.

use libcrux_aes::Aead;
use libcrux_kats::acvp::{
    aes::{
        ecb_mct,
        ecb_schema::{EcbPrompt, EcbPromptTestGroup, EcbResult},
        gcm_schema::{GcmPrompt, GcmPromptTestGroup, GcmResult},
    },
    schema_common::{Prompts, TestGroupResults},
};
use serde_json::Value;

use crate::{
    answer_implemented, not_implemented, not_implemented_group, parse, respond_implemented,
    response, unsupported, Backend,
};

fn ecb_test<const KEY_LEN: usize>(
    g: &EcbPromptTestGroup,
    t: &EcbPrompt,
    encrypt: fn(&[u8; KEY_LEN], &mut [u8; 16]),
) -> EcbResult {
    match g.testType.as_str() {
        "AFT" => {
            let key: [u8; KEY_LEN] = t.key.as_slice().try_into().expect("wrong key length");
            let mut ct = t.pt.clone();
            let (blocks, rest) = ct.as_chunks_mut::<16>();
            assert!(
                rest.is_empty(),
                "the plaintext is not a multiple of the block size"
            );
            for block in blocks {
                encrypt(&key, block);
            }
            EcbResult {
                tcId: t.tcId,
                ct: Some(ct),
                resultsArray: Vec::new(),
            }
        }
        "MCT" => EcbResult {
            tcId: t.tcId,
            ct: None,
            resultsArray: ecb_mct::<KEY_LEN>(&t.key, &t.pt, encrypt),
        },
        tt => panic!("unexpected testType {tt}"),
    }
}

pub(crate) fn ecb(prompt: Value, backend: Backend) -> Value {
    use libcrux_aes::acvp::*;

    type Encrypt128 = fn(&[u8; 16], &mut [u8; 16]);
    type Encrypt256 = fn(&[u8; 32], &mut [u8; 16]);
    let (encrypt128, encrypt256): (Encrypt128, Encrypt256) = match backend {
        Backend::Default | Backend::Portable => {
            (portable_aes128_encrypt_block, portable_aes256_encrypt_block)
        }
        #[cfg(feature = "simd128")]
        Backend::Neon => (neon_aes128_encrypt_block, neon_aes256_encrypt_block),
        #[cfg(feature = "simd256")]
        Backend::Avx2 => (x64_aes128_encrypt_block, x64_aes256_encrypt_block),
        #[allow(unreachable_patterns)]
        _ => unsupported("ACVP-AES-ECB", backend),
    };
    let prompts: Prompts<EcbPromptTestGroup> = parse(prompt);

    respond_implemented(&prompts, |g, t| {
        match g.direction.as_str() {
            "encrypt" => (),
            "decrypt" => return not_implemented("decrypt"),
            d => panic!("unexpected direction {d}"),
        }
        Some(match g.keyLen {
            128 => ecb_test(g, t, encrypt128),
            256 => ecb_test(g, t, encrypt256),
            192 => return not_implemented("AES-192"),
            k => panic!("unexpected key length {k}"),
        })
    })
}

fn gcm_group<Cipher: Aead>(cipher: Cipher, g: &GcmPromptTestGroup) -> TestGroupResults<GcmResult> {
    TestGroupResults {
        tgId: g.tgId,
        tests: answer_implemented(g.tgId, &g.tests, |t| gcm_test(&cipher, g, t)),
    }
}

fn gcm_test<Cipher: Aead>(
    cipher: &Cipher,
    g: &GcmPromptTestGroup,
    t: &GcmPrompt,
) -> Option<GcmResult> {
    assert_eq!(g.testType, "AFT");
    match g.ivGen.as_str() {
        "external" => (),
        "internal" => return not_implemented("internal IV generation"),
        i => panic!("unexpected ivGen {i}"),
    }
    if g.ivLen != 96 {
        return not_implemented(format!("ivLen {}", g.ivLen));
    }
    if g.tagLen != 128 {
        return not_implemented(format!("tagLen {}", g.tagLen));
    }
    assert_eq!(t.aad.len() * 8, g.aadLen, "only byte-oriented AAD");
    let key = cipher.new_key(&t.key).unwrap();
    let nonce = cipher.new_nonce(&t.iv).unwrap();
    let mut result = GcmResult {
        tcId: t.tcId,
        ct: None,
        tag: None,
        pt: None,
        testPassed: None,
    };
    match g.direction.as_str() {
        "encrypt" => {
            assert_eq!(t.pt.len() * 8, g.payloadLen, "only byte-oriented payloads");
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
            (result.ct, result.tag) = (Some(ct), Some(tag));
        }
        "decrypt" => {
            assert_eq!(t.ct.len() * 8, g.payloadLen, "only byte-oriented payloads");
            let mut pt = vec![0u8; t.ct.len()];
            let tag = cipher.new_tag(&t.tag).unwrap();
            match key.decrypt(&mut pt, nonce, &t.aad, &t.ct, tag) {
                Ok(()) => result.pt = Some(pt),
                Err(_) => result.testPassed = Some(false),
            }
        }
        d => panic!("unexpected direction {d}"),
    }
    Some(result)
}

/// Answer the group `$g` with `$c128` or `$c256`, depending on the key length.
macro_rules! gcm_for {
    ($g:expr, $c128:expr, $c256:expr) => {
        match $g.keyLen {
            128 => gcm_group($c128, $g),
            256 => gcm_group($c256, $g),
            192 => not_implemented_group($g.tgId, &$g.tests, "AES-192"),
            k => panic!("unexpected key length {k}"),
        }
    };
}

pub(crate) fn gcm(prompt: Value, backend: Backend) -> Value {
    use libcrux_aes::{aes_gcm_128, aes_gcm_256};

    let prompts: Prompts<GcmPromptTestGroup> = parse(prompt);
    let test_groups = prompts
        .testGroups
        .iter()
        .map(|g| match backend {
            Backend::Default => gcm_for!(g, libcrux_aes::AesGcm128, libcrux_aes::AesGcm256),
            Backend::Portable => gcm_for!(
                g,
                aes_gcm_128::portable::PortableAesGcm128,
                aes_gcm_256::portable::PortableAesGcm256
            ),
            #[cfg(feature = "simd128")]
            Backend::Neon => gcm_for!(
                g,
                aes_gcm_128::neon::NeonAesGcm128,
                aes_gcm_256::neon::NeonAesGcm256
            ),
            #[cfg(feature = "simd256")]
            Backend::Avx2 => gcm_for!(
                g,
                aes_gcm_128::x64::X64AesGcm128,
                aes_gcm_256::x64::X64AesGcm256
            ),
            #[allow(unreachable_patterns)]
            _ => unsupported("ACVP-AES-GCM", backend),
        })
        .filter(|g| !g.tests.is_empty())
        .collect();
    response(&prompts, test_groups)
}

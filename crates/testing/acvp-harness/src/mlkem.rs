//! ML-KEM (FIPS 203).

use libcrux_kats::acvp::{
    mlkem::{
        encap_decap_schema::{
            DecapKey, EncapDecapPromptTestGroup, EncapDecapResult, EncapDecapTestPrompts,
        },
        keygen_schema::{KeyGenPromptTestGroup, KeyGenResult},
    },
    schema_common::{Prompts, TestGroupResults},
};
use libcrux_ml_kem::{mlkem1024, mlkem512, mlkem768};
use serde_json::Value;

use crate::{answer, parse, respond, response, unsupported, within, Backend};

/// `(ek, dk)` or `(c, k)`.
type Pair = (Vec<u8>, Vec<u8>);

/// The ML-KEM functions of one parameter set and backend, on byte slices.
struct MlKem {
    /// `(ek, dk)` from the seed `d || z`.
    keygen: fn([u8; 64]) -> Pair,
    /// `(c, k)`, or `None` if `ek` has the wrong length.
    encaps: fn(&[u8], [u8; 32]) -> Option<Pair>,
    /// `k`, or `None` if `dk` or `c` have the wrong length.
    decaps: fn(&[u8], &[u8]) -> Option<Vec<u8>>,
    check_ek: fn(&[u8]) -> bool,
    check_dk: fn(&[u8]) -> bool,
}

/// [`MlKem`] for the functions in module `$functions`, with the types of `$m` and the
/// decapsulation key check of `$check`.
macro_rules! ml_kem {
    ($m:ident, $pk:ident, $sk:ident, $ct:ident, $($functions:ident)::+, $($check:ident)::+) => {
        MlKem {
            keygen: |seed| {
                let kp = $($functions)::+::generate_key_pair(seed);
                (kp.pk().to_vec(), kp.sk().to_vec())
            },
            encaps: |ek, m| {
                let ek = $m::$pk::try_from(ek).ok()?;
                let (c, k) = $($functions)::+::encapsulate(&ek, m);
                Some((c.as_ref().to_vec(), k.as_ref().to_vec()))
            },
            decaps: |dk, c| {
                let dk = $m::$sk::try_from(dk).ok()?;
                let c = $m::$ct::try_from(c).ok()?;
                Some($($functions)::+::decapsulate(&dk, &c).as_ref().to_vec())
            },
            check_ek: |ek| {
                $m::$pk::try_from(ek)
                    .map(|ek| $($functions)::+::validate_public_key(&ek))
                    .unwrap_or(false)
            },
            check_dk: |dk| {
                $m::$sk::try_from(dk)
                    .map(|dk| $($check)::+::validate_private_key_only(&dk))
                    .unwrap_or(false)
            },
        }
    };
}

macro_rules! ml_kem_for {
    ($m:ident, $pk:ident, $sk:ident, $ct:ident, $backend:expr) => {
        match $backend {
            Backend::Default => MlKem {
                // The multiplexed API has no `validate_private_key_only`, and
                // `validate_private_key` only uses the type of the ciphertext.
                check_dk: |dk| {
                    $m::$sk::try_from(dk)
                        .map(|dk| $m::validate_private_key(&dk, &$m::$ct::default()))
                        .unwrap_or(false)
                },
                ..ml_kem!($m, $pk, $sk, $ct, $m, $m::portable)
            },
            Backend::Portable => ml_kem!($m, $pk, $sk, $ct, $m::portable, $m::portable),
            #[cfg(feature = "simd128")]
            Backend::Neon => ml_kem!($m, $pk, $sk, $ct, $m::neon, $m::neon),
            #[cfg(feature = "simd256")]
            Backend::Avx2 => ml_kem!($m, $pk, $sk, $ct, $m::avx2, $m::avx2),
            #[allow(unreachable_patterns)]
            _ => unsupported("ML-KEM", $backend),
        }
    };
}

fn ml_kem(parameter_set: &str, backend: Backend) -> MlKem {
    match parameter_set {
        "ML-KEM-512" => ml_kem_for!(
            mlkem512,
            MlKem512PublicKey,
            MlKem512PrivateKey,
            MlKem512Ciphertext,
            backend
        ),
        "ML-KEM-768" => ml_kem_for!(
            mlkem768,
            MlKem768PublicKey,
            MlKem768PrivateKey,
            MlKem768Ciphertext,
            backend
        ),
        "ML-KEM-1024" => ml_kem_for!(
            mlkem1024,
            MlKem1024PublicKey,
            MlKem1024PrivateKey,
            MlKem1024Ciphertext,
            backend
        ),
        p => panic!("unsupported parameter set {p}"),
    }
}

fn seed(d: &[u8; 32], z: &[u8; 32]) -> [u8; 64] {
    let mut seed = [0u8; 64];
    seed[..32].copy_from_slice(d);
    seed[32..].copy_from_slice(z);
    seed
}

pub(crate) fn keygen(prompt: Value, backend: Backend) -> Value {
    let prompts: Prompts<KeyGenPromptTestGroup> = parse(prompt);

    respond(&prompts, |g, t| {
        let (ek, dk) = (ml_kem(&g.parameterSet, backend).keygen)(seed(&t.d, &t.z));
        KeyGenResult {
            tcId: t.tcId,
            ek,
            dk,
        }
    })
}

pub(crate) fn encap_decap(prompt: Value, backend: Backend) -> Value {
    let prompts: Prompts<EncapDecapPromptTestGroup> = parse(prompt);

    let test_groups = prompts
        .testGroups
        .iter()
        .map(|g| {
            let ml_kem = within(g.tgId, None, || ml_kem(&g.parameterSet, backend));
            let tests = match &g.tests {
                EncapDecapTestPrompts::EncapTests { tests } => answer(g.tgId, tests, |t| {
                    let (c, k) = (ml_kem.encaps)(&t.ek, t.m).expect("invalid ek");
                    EncapDecapResult::EncapResult { tcId: t.tcId, c, k }
                }),
                EncapDecapTestPrompts::DecapTests { tests } => answer(g.tgId, tests, |t| {
                    assert_eq!(
                        g.keyFormat.as_deref().unwrap_or("expanded"),
                        t.key.key_format(),
                        "keyFormat"
                    );
                    let dk = match &t.key {
                        DecapKey::Expanded { dk } => dk.clone(),
                        DecapKey::Seed { d, z } => (ml_kem.keygen)(seed(d, z)).1,
                    };
                    let k = (ml_kem.decaps)(&dk, &t.c).expect("invalid dk or c");
                    EncapDecapResult::DecapResult { tcId: t.tcId, k }
                }),
                EncapDecapTestPrompts::EncapKeyCheck { tests } => {
                    answer(g.tgId, tests, |t| EncapDecapResult::KeyCheckResult {
                        tcId: t.tcId,
                        testPassed: (ml_kem.check_ek)(&t.ek),
                    })
                }
                EncapDecapTestPrompts::DecapKeyCheck { tests } => {
                    answer(g.tgId, tests, |t| EncapDecapResult::KeyCheckResult {
                        tcId: t.tcId,
                        testPassed: (ml_kem.check_dk)(&t.dk),
                    })
                }
            };
            TestGroupResults {
                tgId: g.tgId,
                tests,
            }
        })
        .collect();
    response(&prompts, test_groups)
}

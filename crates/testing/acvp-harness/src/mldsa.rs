//! ML-DSA (FIPS 204): keyGen, sigGen (including `FIPS204-tr1` seed keys) and sigVer.
//!
//! Supported are the internal interface without external mu, and the external interface with
//! pure signatures and HashML-DSA with SHAKE-128. Other test cases are not implemented.

use libcrux_kats::acvp::{
    mldsa::{
        keygen_schema::{KeyGenPromptTestGroup, KeyGenResult},
        sign_schema::{SigGenPromptTestGroup, SigGenResult},
        verify_schema::{SigVerPromptTestGroup, SigVerResult},
    },
    schema_common::Prompts,
};
use libcrux_ml_dsa::{ml_dsa_44, ml_dsa_65, ml_dsa_87};
use serde_json::Value;

use crate::{not_implemented, parse, respond, respond_implemented, unsupported, Backend};

/// `(vk, sk)` from the seed.
type KeyGen = fn([u8; 32]) -> (Vec<u8>, Vec<u8>);
/// `sign(sk, message, randomness)`, or `None` on an error.
type SignInternal = fn(&[u8], &[u8], [u8; 32]) -> Option<Vec<u8>>;
/// `sign(sk, message, context, randomness)`, or `None` on an error.
type Sign = fn(&[u8], &[u8], &[u8], [u8; 32]) -> Option<Vec<u8>>;
/// `verify(vk, message, signature)`.
type VerifyInternal = fn(&[u8], &[u8], &[u8]) -> bool;
/// `verify(vk, message, context, signature)`.
type Verify = fn(&[u8], &[u8], &[u8], &[u8]) -> bool;

/// The ML-DSA functions of one parameter set and backend, on byte slices.
struct MlDsa {
    keygen: KeyGen,
    sign_internal: SignInternal,
    sign: Sign,
    sign_pre_hashed_shake128: Sign,
    verify_internal: VerifyInternal,
    verify: Verify,
    verify_pre_hashed_shake128: Verify,
}

/// [`MlDsa`] for the functions in module `$functions`.
macro_rules! ml_dsa {
    ($($functions:ident)::+) => {{
        use libcrux_ml_dsa::{MLDSASignature, MLDSASigningKey, MLDSAVerificationKey};
        MlDsa {
            keygen: |seed| {
                let kp = $($functions)::+::generate_key_pair(seed);
                (
                    kp.verification_key.as_slice().to_vec(),
                    kp.signing_key.as_slice().to_vec(),
                )
            },
            sign_internal: |sk, m, rnd| {
                let sk = MLDSASigningKey::new(sk.try_into().ok()?);
                let sig = $($functions)::+::sign_internal(&sk, m, rnd).ok()?;
                Some(sig.as_slice().to_vec())
            },
            sign: |sk, m, ctx, rnd| {
                let sk = MLDSASigningKey::new(sk.try_into().ok()?);
                let sig = $($functions)::+::sign(&sk, m, ctx, rnd).ok()?;
                Some(sig.as_slice().to_vec())
            },
            sign_pre_hashed_shake128: |sk, m, ctx, rnd| {
                let sk = MLDSASigningKey::new(sk.try_into().ok()?);
                let sig = $($functions)::+::sign_pre_hashed_shake128(&sk, m, ctx, rnd).ok()?;
                Some(sig.as_slice().to_vec())
            },
            verify_internal: |vk, m, sig| {
                let (Ok(vk), Ok(sig)) = (vk.try_into(), sig.try_into()) else {
                    return false;
                };
                let (vk, sig) = (MLDSAVerificationKey::new(vk), MLDSASignature::new(sig));
                $($functions)::+::verify_internal(&vk, m, &sig).is_ok()
            },
            verify: |vk, m, ctx, sig| {
                let (Ok(vk), Ok(sig)) = (vk.try_into(), sig.try_into()) else {
                    return false;
                };
                let (vk, sig) = (MLDSAVerificationKey::new(vk), MLDSASignature::new(sig));
                $($functions)::+::verify(&vk, m, ctx, &sig).is_ok()
            },
            verify_pre_hashed_shake128: |vk, m, ctx, sig| {
                let (Ok(vk), Ok(sig)) = (vk.try_into(), sig.try_into()) else {
                    return false;
                };
                let (vk, sig) = (MLDSAVerificationKey::new(vk), MLDSASignature::new(sig));
                $($functions)::+::verify_pre_hashed_shake128(&vk, m, ctx, &sig).is_ok()
            },
        }
    }};
}

macro_rules! ml_dsa_for {
    ($m:ident, $backend:expr) => {
        match $backend {
            Backend::Default => ml_dsa!($m),
            Backend::Portable => ml_dsa!($m::portable),
            #[cfg(feature = "simd128")]
            Backend::Neon => ml_dsa!($m::neon),
            #[cfg(feature = "simd256")]
            Backend::Avx2 => ml_dsa!($m::avx2),
            #[allow(unreachable_patterns)]
            _ => unsupported("ML-DSA", $backend),
        }
    };
}

fn ml_dsa(parameter_set: &str, backend: Backend) -> MlDsa {
    match parameter_set {
        "ML-DSA-44" => ml_dsa_for!(ml_dsa_44, backend),
        "ML-DSA-65" => ml_dsa_for!(ml_dsa_65, backend),
        "ML-DSA-87" => ml_dsa_for!(ml_dsa_87, backend),
        p => panic!("unsupported parameter set {p}"),
    }
}

pub(crate) fn keygen(prompt: Value, backend: Backend) -> Value {
    let prompts: Prompts<KeyGenPromptTestGroup> = parse(prompt);

    respond(&prompts, |g, t| {
        let (pk, sk) = (ml_dsa(&g.parameterSet, backend).keygen)(t.seed);
        KeyGenResult {
            tcId: t.tcId,
            pk,
            sk,
        }
    })
}

pub(crate) fn siggen(prompt: Value, backend: Backend) -> Value {
    let prompts: Prompts<SigGenPromptTestGroup> = parse(prompt);

    respond_implemented(&prompts, |g, t| {
        let ml_dsa = ml_dsa(&g.parameterSet, backend);
        if g.externalMu {
            return not_implemented("externalMu");
        }
        let sk = match g.keyFormat.as_deref() {
            None | Some("expanded") => t.sk.clone(),
            Some("seed") => (ml_dsa.keygen)(t.seed.as_slice().try_into().unwrap()).1,
            f => panic!("unexpected keyFormat {f:?}"),
        };
        assert_eq!(
            g.deterministic,
            t.rnd.is_none(),
            "rnd must be given exactly for non-deterministic signing"
        );
        // Deterministic signing uses an all-zero randomness.
        let rnd = t.rnd.as_ref().map_or([0u8; 32], |r| r.0);
        let signature = match (g.signatureInterface.as_str(), g.preHash.as_deref()) {
            ("internal", _) => (ml_dsa.sign_internal)(&sk, &t.message, rnd),
            ("external", Some("pure")) => (ml_dsa.sign)(&sk, &t.message, &t.context, rnd),
            ("external", Some("preHash")) => match t.hashAlg.as_deref() {
                Some("SHAKE-128") => {
                    (ml_dsa.sign_pre_hashed_shake128)(&sk, &t.message, &t.context, rnd)
                }
                h => return not_implemented(format!("preHash with {}", h.unwrap_or("?"))),
            },
            (i, p) => panic!("unexpected signatureInterface {i} with preHash {p:?}"),
        }
        .expect("signing failed");
        Some(SigGenResult {
            tcId: t.tcId,
            signature,
        })
    })
}

pub(crate) fn sigver(prompt: Value, backend: Backend) -> Value {
    let prompts: Prompts<SigVerPromptTestGroup> = parse(prompt);

    respond_implemented(&prompts, |g, t| {
        let ml_dsa = ml_dsa(&g.parameterSet, backend);
        if g.externalMu {
            return not_implemented("externalMu");
        }
        let passed = match (g.signatureInterface.as_str(), g.preHash.as_deref()) {
            ("internal", _) => (ml_dsa.verify_internal)(&t.pk, &t.message, &t.signature),
            ("external", Some("pure")) => {
                (ml_dsa.verify)(&t.pk, &t.message, &t.context, &t.signature)
            }
            ("external", Some("preHash")) => match t.hashAlg.as_deref() {
                Some("SHAKE-128") => {
                    (ml_dsa.verify_pre_hashed_shake128)(&t.pk, &t.message, &t.context, &t.signature)
                }
                h => return not_implemented(format!("preHash with {}", h.unwrap_or("?"))),
            },
            (i, p) => panic!("unexpected signatureInterface {i} with preHash {p:?}"),
        };
        Some(SigVerResult {
            tcId: t.tcId,
            testPassed: passed,
        })
    })
}

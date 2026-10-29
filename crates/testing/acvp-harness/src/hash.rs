//! SHA-2, SHA-3 and SHAKE.

use libcrux_kats::acvp::{
    hash::{
        schema::{HashPromptTestGroup, HashResult, LargeMessage},
        sha2_mct, sha3_mct, shake_mct,
    },
    schema_common::Prompts,
};
use serde_json::Value;

use crate::{not_implemented, parse, respond_implemented, unsupported, Backend};

type HashFn = fn(&[u8]) -> Vec<u8>;
type XofFn = fn(&mut [u8], &[u8]);

fn sha_fn(algorithm: &str, backend: Backend) -> HashFn {
    match (algorithm, backend) {
        // SHA-2 has a single implementation.
        ("SHA2-224", _) => |m| libcrux_sha2::sha224(m).to_vec(),
        ("SHA2-256", _) => |m| libcrux_sha2::sha256(m).to_vec(),
        ("SHA2-384", _) => |m| libcrux_sha2::sha384(m).to_vec(),
        ("SHA2-512", _) => |m| libcrux_sha2::sha512(m).to_vec(),

        ("SHA3-224", Backend::Default) => |m| libcrux_sha3::sha224(m).to_vec(),

        ("SHA3-256", Backend::Default) => |m| libcrux_sha3::sha256(m).to_vec(),
        ("SHA3-384", Backend::Default) => |m| libcrux_sha3::sha384(m).to_vec(),
        ("SHA3-512", Backend::Default) => |m| libcrux_sha3::sha512(m).to_vec(),
        ("SHA3-224", Backend::Portable) => |m| {
            let mut d = vec![0; 28];
            libcrux_sha3::portable::sha224(&mut d, m);
            d
        },
        ("SHA3-256", Backend::Portable) => |m| {
            let mut d = vec![0; 32];
            libcrux_sha3::portable::sha256(&mut d, m);
            d
        },
        ("SHA3-384", Backend::Portable) => |m| {
            let mut d = vec![0; 48];
            libcrux_sha3::portable::sha384(&mut d, m);
            d
        },
        ("SHA3-512", Backend::Portable) => |m| {
            let mut d = vec![0; 64];
            libcrux_sha3::portable::sha512(&mut d, m);
            d
        },
        #[cfg(feature = "simd128")]
        ("SHA3-224", Backend::Neon) => |m| {
            let mut d = vec![0; 28];
            libcrux_sha3::neon::sha224(&mut d, m);
            d
        },
        #[cfg(feature = "simd128")]
        ("SHA3-256", Backend::Neon) => |m| {
            let mut d = vec![0; 32];
            libcrux_sha3::neon::sha256(&mut d, m);
            d
        },
        #[cfg(feature = "simd128")]
        ("SHA3-384", Backend::Neon) => |m| {
            let mut d = vec![0; 48];
            libcrux_sha3::neon::sha384(&mut d, m);
            d
        },
        #[cfg(feature = "simd128")]
        ("SHA3-512", Backend::Neon) => |m| {
            let mut d = vec![0; 64];
            libcrux_sha3::neon::sha512(&mut d, m);
            d
        },
        _ => unsupported(algorithm, backend),
    }
}

fn shake_fn(algorithm: &str, backend: Backend) -> XofFn {
    match (algorithm, backend) {
        ("SHAKE-128", Backend::Default) => libcrux_sha3::shake128_ema,
        ("SHAKE-256", Backend::Default) => libcrux_sha3::shake256_ema,
        ("SHAKE-128", Backend::Portable) => libcrux_sha3::portable::shake128,
        ("SHAKE-256", Backend::Portable) => libcrux_sha3::portable::shake256,
        #[cfg(feature = "simd128")]
        ("SHAKE-256", Backend::Neon) => |d, m| {
            let mut dummy = vec![0u8; d.len()];
            libcrux_sha3::neon::x2::shake256(m, m, d, &mut dummy);
        },
        #[cfg(feature = "simd256")]
        ("SHAKE-256", Backend::Avx2) => |d, m| {
            let len = d.len();
            let (mut d1, mut d2, mut d3) = (vec![0u8; len], vec![0u8; len], vec![0u8; len]);
            libcrux_sha3::avx2::x4::shake256(m, m, m, m, d, &mut d1, &mut d2, &mut d3);
        },
        _ => unsupported(algorithm, backend),
    }
}

/// The expanded message of a large data test, which must be hashed with a single call, or
/// `None` if it is too large to allocate.
fn large_message(large: &LargeMessage) -> Option<Vec<u8>> {
    if large.fullLength > 8 << 30 {
        return not_implemented(format!("LDT {} GiB", large.fullLength >> 33));
    }
    Some(large.expand())
}

/// SHA-2 and SHA-3: AFT, MCT and LDT.
pub(crate) fn sha(prompt: Value, algorithm: &str, backend: Backend) -> Value {
    let hash = sha_fn(algorithm, backend);
    let sha2 = algorithm.starts_with("SHA2");
    let prompts: Prompts<HashPromptTestGroup> = parse(prompt);

    respond_implemented(&prompts, |g, t| {
        let result = |md| HashResult {
            tcId: t.tcId,
            md: Some(md),
            outLen: None,
            resultsArray: Vec::new(),
        };
        Some(match g.testType.as_str() {
            "AFT" => result(hash(byte_message(t.len, &t.msg)?)),
            "MCT" => {
                assert_eq!(g.mctVersion.as_deref(), Some("standard"));
                HashResult {
                    tcId: t.tcId,
                    md: None,
                    outLen: None,
                    resultsArray: if sha2 {
                        sha2_mct(byte_message(t.len, &t.msg)?, hash)
                    } else {
                        sha3_mct(byte_message(t.len, &t.msg)?, hash)
                    },
                }
            }
            "LDT" => result(hash(&large_message(t.largeMsg.as_ref().unwrap())?)),
            tt => panic!("unexpected testType {tt}"),
        })
    })
}

/// The message `msg` of `len` bits, or `None` if it is not a whole number of bytes.
fn byte_message(len: usize, msg: &[u8]) -> Option<&[u8]> {
    assert_eq!(len.div_ceil(8), msg.len(), "len does not match msg");
    if !len.is_multiple_of(8) {
        return not_implemented("bit-oriented message");
    }
    Some(msg)
}

/// SHAKE: AFT, VOT and MCT.
pub(crate) fn shake(prompt: Value, algorithm: &str, backend: Backend) -> Value {
    let xof = shake_fn(algorithm, backend);
    let prompts: Prompts<HashPromptTestGroup> = parse(prompt);

    respond_implemented(&prompts, |g, t| {
        Some(match g.testType.as_str() {
            "AFT" | "VOT" => {
                let msg = byte_message(t.len, &t.msg)?;
                let out_len = t.outLen.unwrap();
                if out_len % 8 != 0 {
                    return not_implemented("bit-oriented output");
                }
                let mut md = vec![0u8; out_len / 8];
                xof(&mut md, msg);
                HashResult {
                    tcId: t.tcId,
                    md: Some(md),
                    outLen: Some(out_len),
                    resultsArray: Vec::new(),
                }
            }
            "MCT" => {
                assert_eq!(g.mctVersion.as_deref(), Some("standard"));
                let (min, max) = (g.minOutLen.unwrap(), g.maxOutLen.unwrap());
                if min % 8 != 0 || max % 8 != 0 {
                    return not_implemented("bit-oriented output");
                }
                HashResult {
                    tcId: t.tcId,
                    md: None,
                    outLen: None,
                    resultsArray: shake_mct(byte_message(t.len, &t.msg)?, min, max, xof),
                }
            }
            tt => panic!("unexpected testType {tt}"),
        })
    })
}

/// CAVP (Cryptographic Algorithm Validation Program) tests.
/// Ported from ../../crates/algorithms/sha3/tests/cavp.rs
///
/// Reads NIST .rsp test vector files and validates our SHA-3/SHAKE implementation
/// against each test case.
use hacspec_sha3::*;
use std::fs;
use std::path::Path;

// ---------------------------------------------------------------------------
// Simple .rsp file parser (replaces the external `cavp` crate dependency)
// ---------------------------------------------------------------------------

struct Sha3TestCase {
    msg_length_bits: usize,
    msg: Vec<u8>,
    digest: Vec<u8>,
}

fn parse_sha3_rsp(path: &Path) -> Vec<Sha3TestCase> {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    let mut tests = Vec::new();
    let mut len: usize = 0;
    let mut msg: Vec<u8> = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
            continue;
        }
        if let Some(val) = line.strip_prefix("Len = ") {
            len = val.trim().parse().unwrap();
        } else if let Some(val) = line.strip_prefix("Msg = ") {
            msg = hex::decode(val.trim()).unwrap();
        } else if let Some(val) = line.strip_prefix("MD = ") {
            let digest = hex::decode(val.trim()).unwrap();
            tests.push(Sha3TestCase {
                msg_length_bits: len,
                msg: msg.clone(),
                digest,
            });
        }
    }
    tests
}

struct ShakeTestCase {
    msg_length_bits: usize,
    msg: Vec<u8>,
    output: Vec<u8>,
}

fn parse_shake_rsp(path: &Path) -> Vec<ShakeTestCase> {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    let mut tests = Vec::new();
    let mut len: usize = 0;
    let mut msg: Vec<u8> = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
            continue;
        }
        if let Some(val) = line.strip_prefix("Len = ") {
            len = val.trim().parse().unwrap();
        } else if let Some(val) = line.strip_prefix("Msg = ") {
            msg = hex::decode(val.trim()).unwrap();
        } else if let Some(val) = line.strip_prefix("Output = ") {
            let output = hex::decode(val.trim()).unwrap();
            tests.push(ShakeTestCase {
                msg_length_bits: len,
                msg: msg.clone(),
                output,
            });
        }
    }
    tests
}

struct ShakeVariableOutTestCase {
    msg: Vec<u8>,
    output: Vec<u8>,
}

fn parse_shake_variable_out_rsp(path: &Path) -> (usize, Vec<ShakeVariableOutTestCase>) {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    let mut tests = Vec::new();
    let mut input_length_bits: usize = 0;
    let mut msg: Vec<u8> = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // Parse header fields like [Input Length = 128]
        if line.starts_with('[') && line.ends_with(']') {
            let inner = &line[1..line.len() - 1];
            if let Some(val) = inner.strip_prefix("Input Length = ") {
                input_length_bits = val.trim().parse().unwrap();
            }
            continue;
        }
        if line.starts_with("COUNT") || line.starts_with("Outputlen") {
            // We don't need these — output length is implicit in the expected output
            continue;
        }
        if let Some(val) = line.strip_prefix("Msg = ") {
            msg = hex::decode(val.trim()).unwrap();
        } else if let Some(val) = line.strip_prefix("Output = ") {
            let output = hex::decode(val.trim()).unwrap();
            tests.push(ShakeVariableOutTestCase {
                msg: msg.clone(),
                output,
            });
        }
    }
    (input_length_bits, tests)
}

// ---------------------------------------------------------------------------
// Path to the test vector files (shared with reference implementation)
// ---------------------------------------------------------------------------

fn tv_path(name: &str) -> std::path::PathBuf {
    // From specs/sha3/ to crates/algorithms/sha3/tests/tv/
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/algorithms/sha3/tests/tv")
        .join(name)
}

// ---------------------------------------------------------------------------
// SHA3 CAVP tests
// ---------------------------------------------------------------------------

macro_rules! sha3_cavp_test {
    ($name:ident, $file:expr, $hash_fn:ident, $digest_len:expr) => {
        #[test]
        fn $name() {
            let tests = parse_sha3_rsp(&tv_path($file));
            assert!(!tests.is_empty(), "no test cases found");
            for (i, tc) in tests.iter().enumerate() {
                let msg = &tc.msg[..tc.msg_length_bits / 8];
                let digest = $hash_fn(msg);
                assert_eq!(
                    &digest[..],
                    &tc.digest[..],
                    "test case {i} failed (msg_len={} bits)",
                    tc.msg_length_bits
                );
            }
        }
    };
}

sha3_cavp_test!(sha3_224_short_msg, "SHA3_224ShortMsg.rsp", sha3_224, 28);
sha3_cavp_test!(sha3_224_long_msg, "SHA3_224LongMsg.rsp", sha3_224, 28);
sha3_cavp_test!(sha3_256_short_msg, "SHA3_256ShortMsg.rsp", sha3_256, 32);
sha3_cavp_test!(sha3_256_long_msg, "SHA3_256LongMsg.rsp", sha3_256, 32);
sha3_cavp_test!(sha3_384_short_msg, "SHA3_384ShortMsg.rsp", sha3_384, 48);
sha3_cavp_test!(sha3_384_long_msg, "SHA3_384LongMsg.rsp", sha3_384, 48);
sha3_cavp_test!(sha3_512_short_msg, "SHA3_512ShortMsg.rsp", sha3_512, 64);
sha3_cavp_test!(sha3_512_long_msg, "SHA3_512LongMsg.rsp", sha3_512, 64);

// ---------------------------------------------------------------------------
// SHAKE CAVP tests (short/long message, fixed output length)
// ---------------------------------------------------------------------------

// SHAKE128 ShortMsg/LongMsg: [Outputlen = 128] → 16 bytes
#[test]
fn shake128_short_msg() {
    let tests = parse_shake_rsp(&tv_path("SHAKE128ShortMsg.rsp"));
    assert!(!tests.is_empty());
    for (i, tc) in tests.iter().enumerate() {
        let msg = &tc.msg[..tc.msg_length_bits / 8];
        let digest = shake128::<16>(msg);
        assert_eq!(
            &digest[..],
            &tc.output[..],
            "test case {i} failed (msg_len={} bits)",
            tc.msg_length_bits
        );
    }
}

#[test]
fn shake128_long_msg() {
    let tests = parse_shake_rsp(&tv_path("SHAKE128LongMsg.rsp"));
    assert!(!tests.is_empty());
    for (i, tc) in tests.iter().enumerate() {
        let msg = &tc.msg[..tc.msg_length_bits / 8];
        let digest = shake128::<16>(msg);
        assert_eq!(
            &digest[..],
            &tc.output[..],
            "test case {i} failed (msg_len={} bits)",
            tc.msg_length_bits
        );
    }
}

// SHAKE256 ShortMsg/LongMsg: [Outputlen = 256] → 32 bytes
#[test]
fn shake256_short_msg() {
    let tests = parse_shake_rsp(&tv_path("SHAKE256ShortMsg.rsp"));
    assert!(!tests.is_empty());
    for (i, tc) in tests.iter().enumerate() {
        let msg = &tc.msg[..tc.msg_length_bits / 8];
        let digest = shake256::<32>(msg);
        assert_eq!(
            &digest[..],
            &tc.output[..],
            "test case {i} failed (msg_len={} bits)",
            tc.msg_length_bits
        );
    }
}

#[test]
fn shake256_long_msg() {
    let tests = parse_shake_rsp(&tv_path("SHAKE256LongMsg.rsp"));
    assert!(!tests.is_empty());
    for (i, tc) in tests.iter().enumerate() {
        let msg = &tc.msg[..tc.msg_length_bits / 8];
        let digest = shake256::<32>(msg);
        assert_eq!(
            &digest[..],
            &tc.output[..],
            "test case {i} failed (msg_len={} bits)",
            tc.msg_length_bits
        );
    }
}

// ---------------------------------------------------------------------------
// SHAKE Variable Output Length CAVP tests
//
// These tests have variable output lengths per test case. Since our API uses
// const generics, we compute a max-size output and compare the prefix.
// SHAKE is an XOF so the first N bytes of shake(msg, K) match shake(msg, N)
// for any K >= N.
// ---------------------------------------------------------------------------

// SHAKE128 VariableOut: max output = 1120 bits = 140 bytes
#[test]
fn shake128_variable_out() {
    let (input_length_bits, tests) =
        parse_shake_variable_out_rsp(&tv_path("SHAKE128VariableOut.rsp"));
    assert!(!tests.is_empty());
    for (i, tc) in tests.iter().enumerate() {
        let msg = &tc.msg[..input_length_bits / 8];
        let full_output = shake128::<140>(msg);
        let expected_len = tc.output.len();
        assert_eq!(
            &full_output[..expected_len],
            &tc.output[..],
            "test case {i} failed (output_len={expected_len} bytes)",
        );
    }
}

// SHAKE256 VariableOut: max output = 2000 bits = 250 bytes
#[test]
fn shake256_variable_out() {
    let (input_length_bits, tests) =
        parse_shake_variable_out_rsp(&tv_path("SHAKE256VariableOut.rsp"));
    assert!(!tests.is_empty());
    for (i, tc) in tests.iter().enumerate() {
        let msg = &tc.msg[..input_length_bits / 8];
        let full_output = shake256::<250>(msg);
        let expected_len = tc.output.len();
        assert_eq!(
            &full_output[..expected_len],
            &tc.output[..],
            "test case {i} failed (output_len={expected_len} bytes)",
        );
    }
}

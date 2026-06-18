use std::any::type_name_of_val;

use libcrux_kats::wycheproof::{ecdh, ecdsa, TestResult};
use libcrux_p256::{
    compressed_to_raw, ecdh_api::EcdhSlice, ecdsa_verif_p256_sha2, ecdsa_verif_p256_sha512,
    uncompressed_to_raw,
};

fn pad_slice_to_arr(b: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[32 - b.len()..].copy_from_slice(b);
    out
}

#[test]
fn ecdh_secp256r1() {
    let test_set = ecdh::TestSet::load_secp256r1_ecpoint();
    let mut tests_run = 0;

    for test_group in test_set.test_groups {
        for test in &test_group.tests {
            // The ecpoint format gives us sec 1 encoded public keys as hex strings and private keys
            // encoded as hex formatted big integers

            // strip leading 0 bytes
            let sk = test
                .private_key
                .iter()
                .position(|b| *b != 0)
                .map_or(test.private_key.as_slice(), |pos| &test.private_key[pos..]);

            assert!(
                sk.len() <= 32,
                "0 prefix stripped sk is larger than 32 bytes, tc_id: {}",
                test.tc_id
            );
            // sk is a 32-byte big endian big integer, so we pad the lower bytes with 0
            let sk_bytes = pad_slice_to_arr(sk);

            let mut pk_bytes = [0; 64];
            if test.public_key.len() == 65 {
                assert!(
                    uncompressed_to_raw(&test.public_key, &mut pk_bytes),
                    "tc_id: {}, invalid uncompressed public key",
                    test.tc_id
                );
            } else if test.public_key.len() == 33 {
                let valid = compressed_to_raw(&test.public_key, &mut pk_bytes);
                if !valid {
                    assert_eq!(
                        TestResult::Invalid,
                        test.result,
                        "tc_id: {}, test has invalid compressed point but test result is {:?}",
                        test.tc_id,
                        test.result
                    );
                    tests_run += 1;
                    continue;
                }
            } else {
                assert_eq!(
                    TestResult::Invalid,
                    test.result,
                    "tc_id: {}, public key has invalid size {}, but test result is {:?}",
                    test.tc_id,
                    test.public_key.len(),
                    test.result
                );
                assert!(
                    test.flags.contains(&"InvalidEncoding".to_string()),
                    "tc_id: {}, public key is invalid but test does not contain InvalidEncoding flag ",
                    test.tc_id
                );
                tests_run += 1;
                continue;
            }

            let mut derived = [0u8; 64];
            let result = libcrux_p256::P256::derive_ecdh(&mut derived, &pk_bytes, &sk_bytes);

            match test.result {
                // XXX: In the future, wycheproof might add acceptable test cases which we (want to) reject.
                // This needs to be split then.
                TestResult::Valid | TestResult::Acceptable => {
                    assert!(
                        result.is_ok(),
                        "tc_id {}: expected success or acceptable but ECDH failed",
                        test.tc_id,
                    );
                    // Wycheproof shared secret is just the X coordinate (first 32 bytes)
                    assert_eq!(
                        test.shared_secret,
                        derived[..32],
                        "tc_id {}: shared secret mismatch",
                        test.tc_id,
                    );
                }
                TestResult::Invalid => {
                    assert!(
                        result.is_err(),
                        "tc_id: {}, expected invalid test but ECDH derive succeeded",
                        test.tc_id
                    );
                }
            }
            tests_run += 1;
        }
    }

    assert_eq!(
        test_set.number_of_tests, tests_run,
        "invalid number of tests run"
    );
    println!("Ran {tests_run} ecdh_secp256r1_ecpoint tests",);
}

/// A P-256 Signature
#[derive(Clone, Default)]
struct Signature {
    r: [u8; 32],
    s: [u8; 32],
}

/// ASN.1 DER parser for ECDSA signatures.
/// Returns None if the signature is malformed.
fn decode_signature(sig: &[u8]) -> Option<Signature> {
    use der::{asn1::UintRef, Decode, Reader};
    // Adapted from https://docs.rs/ecdsa/0.16.9/src/ecdsa/der.rs.html#357-370
    fn decode_der_rust_crypto(der_bytes: &[u8]) -> der::Result<(UintRef<'_>, UintRef<'_>)> {
        let mut reader = der::SliceReader::new(der_bytes)?;
        let header = der::Header::decode(&mut reader)?;
        header.tag().assert_eq(der::Tag::Sequence)?;

        let (r, s) = reader.read_nested(header.length(), |reader| {
            let r = UintRef::decode(reader)?;
            let s = UintRef::decode(reader)?;
            Ok::<_, der::Error>((r, s))
        })?;
        reader.finish()?;
        Ok((r, s))
    }
    let (r, s) = decode_der_rust_crypto(sig).ok()?;
    if r.as_bytes().len() > 32 || s.as_bytes().len() > 32 {
        return None;
    }
    Some(Signature {
        r: pad_slice_to_arr(r.as_bytes()),
        s: pad_slice_to_arr(s.as_bytes()),
    })
}

/// Generic ecdsa_secp256r1_sha test function
///
/// The [`ecdsa::TestSet`] and signature verification function must correspond to each other.
fn ecdsa_secp256r1_sha_test<F>(test_set: ecdsa::TestSet, verify: F)
where
    F: Fn(u32, &[u8], &[u8], &[u8], &[u8]) -> bool,
{
    let mut tests_run = 0;
    let mut decoding_sig_failed = 0;

    for test_group in test_set.test_groups {
        let mut pk_bytes = [0; 64];
        if test_group.key.key.len() == 65 {
            assert!(
                uncompressed_to_raw(&test_group.key.key, &mut pk_bytes),
                "test group with invalid uncompressed public key. Key (DER): {}",
                test_group.public_key_der
            );
        } else {
            panic!(
                "test group with invalid public key length. Key (DER): {}",
                test_group.public_key_der
            );
        }
        for test in &test_group.tests {
            let Some(sig) = decode_signature(&test.sig) else {
                let contains_expected_flag = test.flags.iter().any(|flag| {
                    // Test cases with these flags can fail the decoding
                    // Unfortunately, there are also test cases which have this flags, which **should** decode,
                    // but then fail verification. There currently doesn't seem to be a good way to distinguish
                    // between these.
                    [
                        "BerEncodedSignature",
                        "InvalidEncoding",
                        "MissingZero",
                        "ModifiedSignature",
                        "InvalidTypesInSignature",
                        "RangeCheck",
                        "ModifiedInteger",
                        "IntegerOverflow",
                        "InvalidSignature",
                        "ArithmeticError",
                    ]
                    .contains(&flag.as_str())
                });
                decoding_sig_failed += 1;
                assert_eq!(
                    TestResult::Invalid,
                    test.result,
                    "tc_id: {}, signature decoding failed but test is valid",
                    test.tc_id
                );
                assert!(
                    contains_expected_flag,
                    "tc_id: {}, decoding signature failed, but test contained unexpected flags",
                    test.tc_id
                );
                tests_run += 1;
                continue;
            };

            let valid = verify(test.msg.len() as u32, &test.msg, &pk_bytes, &sig.r, &sig.s);

            match test.result {
                TestResult::Valid => {
                    assert!(
                        valid,
                        "tc_id: {}, test result is valid but verify failed",
                        test.tc_id
                    );
                }
                TestResult::Invalid => {
                    assert!(
                        !valid,
                        "tc_id: {}, test result is invalid but verify succeeded",
                        test.tc_id
                    );
                }
                TestResult::Acceptable => {
                    unreachable!("not present in test set")
                }
            }

            tests_run += 1;
        }
    }

    assert_ne!(
        test_set.number_of_tests, decoding_sig_failed,
        "invalid signature decode function"
    );
    assert_eq!(
        test_set.number_of_tests, tests_run,
        "invalid number of tests run"
    );
    println!("Ran {tests_run} {} tests", type_name_of_val(&verify),);
}

#[test]
fn ecdsa_secp256r1_sha256() {
    let test_set = ecdsa::TestSet::load_secp256r1_sha256();
    ecdsa_secp256r1_sha_test(test_set, ecdsa_verif_p256_sha2);
}

#[test]
fn ecdsa_secp256r1_sha512() {
    let test_set = ecdsa::TestSet::load_secp256r1_sha512();
    ecdsa_secp256r1_sha_test(test_set, ecdsa_verif_p256_sha512);
}

//! Wycheproof ECDSA P-256 known answer tests.
//!
//! Signatures in the test vectors are DER-encoded, so every case also exercises
//! [`Signature::from_der`] and, for anything that decodes, [`Signature::to_der`].

use libcrux_ecdsa::{
    p256::{self, PublicKey, Signature},
    DigestAlgorithm, Error,
};
use libcrux_kats::wycheproof::{ecdsa, TestResult};

/// Flags marking vectors that a strict DER decoder may legitimately reject.
///
/// Vectors carrying only *other* flags encode a well-formed signature that must
/// decode and then fail verification — `PointDuplication` and `Untruncatedhash`
/// are the notable ones.
const MAY_FAIL_TO_DECODE: &[&str] = &[
    "ArithmeticError",
    "BerEncodedSignature",
    "IntegerOverflow",
    "InvalidEncoding",
    "InvalidSignature",
    "InvalidTypesInSignature",
    "MissingZero",
    "ModifiedInteger",
    "ModifiedSignature",
    "RangeCheck",
];

/// Flags marking encodings that a strict DER decoder must never accept: BER
/// long-form and indefinite lengths, wrong tags, redundant leading zero bytes,
/// and integers that do not fit in 32 bytes.
///
/// This is the dual of [`MAY_FAIL_TO_DECODE`] and the assertion that catches the
/// decoder becoming *lax*. `RangeCheck` is deliberately absent: those vectors
/// (`r ± n`, `r + 256 * n`) are rejected today only because those particular
/// values need 33 bytes or are negative, not because the encoding is malformed.
/// A vector with `r == n` would be canonical DER and must be caught by the
/// range check in verification instead.
///
/// If a refreshed Wycheproof file trips this assertion, decide whether the new
/// vector really is a non-canonical *encoding* (then the decoder has a bug) or a
/// semantic problem that happens to carry an encoding flag (then drop the flag
/// from this list).
const MUST_FAIL_TO_DECODE: &[&str] = &[
    "BerEncodedSignature",
    "IntegerOverflow",
    "InvalidEncoding",
    "InvalidTypesInSignature",
    "MissingZero",
];

fn has_flag(test: &ecdsa::Test, flags: &[&str]) -> bool {
    test.flags.iter().any(|flag| flags.contains(&flag.as_str()))
}

fn wycheproof_ecdsa_p256(test_set: ecdsa::TestSet, hash: DigestAlgorithm) {
    let mut tests_run = 0;
    let mut decoding_sig_failed = 0;

    for test_group in &test_set.test_groups {
        // Uncompressed point, `04 || X || Y`.
        let pk = PublicKey::try_from(test_group.key.key.as_slice()).unwrap_or_else(|e| {
            panic!(
                "test group with invalid public key ({e:?}). Key (DER): {}",
                test_group.public_key_der
            )
        });

        for test in &test_group.tests {
            let signature = match Signature::from_der(&test.sig) {
                Ok(signature) => signature,
                Err(_) => {
                    assert_eq!(
                        TestResult::Invalid,
                        test.result,
                        "tc_id {}: signature failed to decode but the test is not invalid",
                        test.tc_id
                    );
                    assert!(
                        has_flag(test, MAY_FAIL_TO_DECODE),
                        "tc_id {}: signature failed to decode for an unexpected reason: {:?}",
                        test.tc_id,
                        test.flags
                    );
                    decoding_sig_failed += 1;
                    tests_run += 1;
                    continue;
                }
            };

            assert!(
                !has_flag(test, MUST_FAIL_TO_DECODE),
                "tc_id {}: non-canonical encoding was accepted: {:?}",
                test.tc_id,
                test.flags
            );

            // Anything the strict decoder accepts is already canonical DER, so
            // re-encoding it must reproduce the input byte for byte.
            assert_eq!(
                signature.to_der().as_bytes(),
                test.sig.as_slice(),
                "tc_id {}: re-encoding the signature did not reproduce the input",
                test.tc_id
            );

            match (p256::verify(hash, &test.msg, &signature, &pk), &test.result) {
                (Ok(()), TestResult::Valid) => {}
                (Err(Error::InvalidSignature), TestResult::Invalid) => {}
                (result, expected) => panic!(
                    "tc_id {}: verify returned {result:?} but the test result is {expected:?}",
                    test.tc_id
                ),
            }

            tests_run += 1;
        }
    }

    assert_eq!(test_set.number_of_tests, tests_run, "did not run all tests");
    // Guard against a decoder that accepts everything; the converse (one that
    // rejects everything) is caught by the `TestResult::Invalid` assertion above.
    assert!(
        decoding_sig_failed > 0,
        "no signature was rejected while decoding"
    );
    println!("Ran {tests_run} tests, {decoding_sig_failed} of which were rejected while decoding");
}

#[test]
fn ecdsa_secp256r1_sha256() {
    wycheproof_ecdsa_p256(
        ecdsa::TestSet::load_secp256r1_sha256(),
        DigestAlgorithm::Sha256,
    );
}

#[test]
fn ecdsa_secp256r1_sha512() {
    wycheproof_ecdsa_p256(
        ecdsa::TestSet::load_secp256r1_sha512(),
        DigestAlgorithm::Sha512,
    );
}

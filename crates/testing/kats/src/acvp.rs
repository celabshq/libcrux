//! ACVP Known Answer Tests
//!
//! Each vector set is loaded into a struct with the `prompts` (`prompt.json`) and the expected
//! `results` (`expectedResults.json`), and for sample sets also the `projection`
//! (`internalProjection.json`). [`schema_common`] has the lookup and coverage helpers shared by
//! all of them.

/// Load the vector set in `acvp/$directory/$variant` into `Self { prompts, results }`, or with
/// `projection` into `Self { prompts, results, projection }`.
#[allow(unused_macros)]
macro_rules! load {
    (@file $directory:literal, $variant:literal, $file:literal) => {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/acvp/",
            $directory,
            "/",
            $variant,
            "/",
            $file
        )))
        .expect(concat!(
            "Could not deserialize ",
            $directory,
            "/",
            $variant,
            "/",
            $file
        ))
    };
    ($directory:literal, $variant:literal) => {
        Self {
            prompts: load!(@file $directory, $variant, "prompt.json"),
            results: load!(@file $directory, $variant, "expectedResults.json"),
        }
    };
    ($directory:literal, $variant:literal, projection) => {
        Self {
            prompts: load!(@file $directory, $variant, "prompt.json"),
            results: load!(@file $directory, $variant, "expectedResults.json"),
            projection: load!(@file $directory, $variant, "internalProjection.json"),
        }
    };
}

/// Implement [`schema_common::TestCase`] for test case structs with a `tcId`.
#[allow(unused_macros)]
macro_rules! impl_test_case {
    ($($case:ty),* $(,)?) => {$(
        impl $crate::acvp::schema_common::TestCase for $case {
            fn tc_id(&self) -> usize {
                self.tcId
            }
        }
    )*};
}

/// Implement [`schema_common::TestGroup`] for test group structs with a `tgId` and `tests`.
#[allow(unused_macros)]
macro_rules! impl_test_group {
    ($($group:ty => $case:ty),* $(,)?) => {$(
        impl $crate::acvp::schema_common::TestGroup for $group {
            type Test = $case;

            fn tg_id(&self) -> usize {
                self.tgId
            }

            fn tests(&self) -> &[$case] {
                &self.tests
            }
        }
    )*};
}

pub mod schema_common;

/// Helpers for elliptic curve points and scalars given as big-endian integers.
pub mod ec;

#[cfg(feature = "mldsa")]
pub mod mldsa;

#[cfg(feature = "mlkem")]
pub mod mlkem;

#[cfg(feature = "kbkdf")]
pub mod kbkdf;

#[cfg(feature = "kda")]
pub mod kda;

#[cfg(feature = "kmac")]
pub mod kmac;

#[cfg(any(feature = "sha2", feature = "sha3"))]
pub mod hash;

#[cfg(feature = "sha2")]
pub mod sha2;

#[cfg(feature = "sha3")]
pub mod sha3;

#[cfg(feature = "hmac")]
pub mod hmac;

#[cfg(feature = "aes")]
pub mod aes;

#[cfg(feature = "kas-ecc")]
pub mod kas_ecc;

#[cfg(feature = "ecdsa")]
pub mod ecdsa;

#[cfg(feature = "drbg")]
pub mod drbg;

#[cfg(feature = "eddsa")]
pub mod eddsa;

#[cfg(feature = "xecdh")]
pub mod xecdh;

#[cfg(test)]
mod tests {
    #![allow(unused_macros)]

    use super::schema_common::assert_covers;

    /// Check that the vector set parses, that the prompts and expected results cover the same
    /// test cases, and that serializing the expected results reproduces `expectedResults.json`.
    macro_rules! check {
        ($tests:expr, $directory:literal, $variant:literal) => {{
            let tests = $tests;
            assert_covers(&tests.prompts, &tests.results);
            let expected: serde_json::Value = serde_json::from_str(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/acvp/",
                $directory,
                "/",
                $variant,
                "/expectedResults.json"
            )))
            .unwrap();
            assert_eq!(serde_json::to_value(&tests.results).unwrap(), expected);
        }};
    }

    #[cfg(feature = "aes")]
    #[test]
    fn aes() {
        use super::aes::*;
        check!(EcbTests::aes128_encrypt(), "aes-ecb-1_0", "aes-128-encrypt");
        check!(EcbTests::aes256_encrypt(), "aes-ecb-1_0", "aes-256-encrypt");
        check!(
            GcmTests::aes128_iv96_tag128(),
            "aes-gcm-1_0",
            "aes-128-iv96-tag128"
        );
        check!(
            GcmTests::aes256_iv96_tag128(),
            "aes-gcm-1_0",
            "aes-256-iv96-tag128"
        );
        check!(
            CcmTests::aes128_256_iv96(),
            "aes-ccm-1_0",
            "aes-128-256-iv96"
        );
    }

    #[cfg(feature = "drbg")]
    #[test]
    fn drbg() {
        use super::drbg::HmacDrbgTests;
        check!(
            HmacDrbgTests::sha2_256_sha2_384(),
            "hmac-drbg-sp800-90ar1",
            "sha2-256-sha2-384"
        );
        check!(
            HmacDrbgTests::sha2_512(),
            "hmac-drbg-sp800-90ar1",
            "sha2-512"
        );
    }

    #[cfg(feature = "ecdsa")]
    #[test]
    fn ecdsa() {
        use super::ecdsa::*;
        check!(KeyGenTests::p256(), "ecdsa-keygen-fips186-5", "p-256");
        check!(KeyGenTests::p384(), "ecdsa-keygen-fips186-5", "p-384");
        check!(KeyVerTests::p256(), "ecdsa-keyver-fips186-5", "p-256");
        check!(KeyVerTests::p384(), "ecdsa-keyver-fips186-5", "p-384");
        check!(SigGenTests::p256(), "ecdsa-siggen-fips186-5", "p-256");
        check!(SigVerTests::p256(), "ecdsa-sigver-fips186-5", "p-256");
    }

    #[cfg(feature = "eddsa")]
    #[test]
    fn eddsa() {
        use super::eddsa::*;
        check!(KeyGenTests::ed25519(), "eddsa-1_0", "keygen-ed25519");
        check!(SigGenTests::ed25519(), "eddsa-1_0", "siggen-ed25519");
        let tests = SigGenTests::ed25519();
        assert_covers(&tests.prompts, &tests.projection);
        check!(SigVerTests::ed25519(), "eddsa-1_0", "sigver-ed25519");
    }

    #[cfg(feature = "hmac")]
    #[test]
    fn hmac() {
        use super::hmac::HmacTests;
        check!(HmacTests::hmac_sha2_256(), "hmac-sha2-2_0", "hmac-sha2-256");
        check!(HmacTests::hmac_sha2_384(), "hmac-sha2-2_0", "hmac-sha2-384");
        check!(HmacTests::hmac_sha2_512(), "hmac-sha2-2_0", "hmac-sha2-512");
        check!(HmacTests::hmac_sha3_224(), "hmac-sha3-2_0", "hmac-sha3-224");
        check!(HmacTests::hmac_sha3_256(), "hmac-sha3-2_0", "hmac-sha3-256");
        check!(HmacTests::hmac_sha3_384(), "hmac-sha3-2_0", "hmac-sha3-384");
        check!(HmacTests::hmac_sha3_512(), "hmac-sha3-2_0", "hmac-sha3-512");
    }

    #[cfg(feature = "kas-ecc")]
    #[test]
    fn kas_ecc() {
        use super::kas_ecc::KasEccSscTests;
        for tests in [KasEccSscTests::p256(), KasEccSscTests::p384()] {
            assert_covers(&tests.prompts, &tests.projection);
        }
        check!(KasEccSscTests::p256(), "kas-ecc-ssc-sp800-56ar3", "p-256");
        check!(KasEccSscTests::p384(), "kas-ecc-ssc-sp800-56ar3", "p-384");
    }

    #[cfg(feature = "kbkdf")]
    #[test]
    fn kbkdf() {
        use super::kbkdf::FeedbackHmacTests;
        check!(
            FeedbackHmacTests::load(),
            "kdf-1_0",
            "feedback-hmac-ctr32-before-fixed"
        );
    }

    #[cfg(feature = "kda")]
    #[test]
    fn kda() {
        use super::kda::*;
        check!(HkdfTests::sha2_sha3(), "kda-hkdf-sp800-56cr2", "sha2-sha3");
        let [l136, l512, l1000] = TwoStepFeedbackHmacTests::load_all()
            .try_into()
            .ok()
            .unwrap();
        check!(
            l136,
            "kda-twostep-sp800-56cr2",
            "feedback-hmac-ctr32-before-fixed-l136"
        );
        check!(
            l512,
            "kda-twostep-sp800-56cr2",
            "feedback-hmac-ctr32-before-fixed-l512"
        );
        check!(
            l1000,
            "kda-twostep-sp800-56cr2",
            "feedback-hmac-ctr32-before-fixed-l1000"
        );
        let [combine, dhkdf, nho256, nho384] = TwoStepFeedbackHmacTests::load_3kem()
            .try_into()
            .ok()
            .unwrap();
        check!(combine, "kda-twostep-sp800-56cr2", "3kem-combine-sha3-256");
        check!(
            dhkdf,
            "kda-twostep-sp800-56cr2",
            "3kem-dhkdf-combine-sha3-384"
        );
        check!(nho256, "kda-twostep-sp800-56cr2", "3kem-nho-sha3-256");
        check!(nho384, "kda-twostep-sp800-56cr2", "3kem-nho-sha3-384");
    }

    #[cfg(feature = "kmac")]
    #[test]
    fn kmac() {
        use super::kmac::KmacTests;
        check!(KmacTests::kmac_128(), "kmac-1_0", "kmac-128");
        check!(KmacTests::kmac_256(), "kmac-1_0", "kmac-256");
    }

    #[cfg(feature = "mldsa")]
    #[test]
    fn mldsa() {
        use super::mldsa::*;
        check!(KeyGenTests::load(), "mldsa-1_1_0_43", "keygen");
        check!(SigGenTests::load(), "mldsa-1_1_0_43", "siggen");
        check!(SigGenTests::load_tr1(), "mldsa-1_1_0_43", "siggen-tr1");
        check!(SigVerTests::load(), "mldsa-1_1_0_43", "sigver");
    }

    #[cfg(feature = "mlkem")]
    #[test]
    fn mlkem() {
        use super::mlkem::*;
        check!(KeyGenTests::load(), "mlkem-1_1_0_43", "keygen");
        check!(EncapDecapTests::load(), "mlkem-1_1_0_43", "encap-decap");
    }

    #[cfg(feature = "sha2")]
    #[test]
    fn sha2() {
        use super::sha2::HashTests;
        check!(HashTests::sha2_224(), "sha2-1_0", "sha2-224");
        check!(HashTests::sha2_256(), "sha2-1_0", "sha2-256");
        check!(HashTests::sha2_384(), "sha2-1_0", "sha2-384");
        check!(HashTests::sha2_512(), "sha2-1_0", "sha2-512");
    }

    #[cfg(feature = "sha3")]
    #[test]
    fn sha3() {
        use super::sha3::HashTests;
        check!(HashTests::sha3_224(), "sha3-2_0", "sha3-224");
        check!(HashTests::sha3_256(), "sha3-2_0", "sha3-256");
        check!(HashTests::sha3_384(), "sha3-2_0", "sha3-384");
        check!(HashTests::sha3_512(), "sha3-2_0", "sha3-512");
        check!(HashTests::shake128(), "shake-1_0", "shake-128");
        check!(HashTests::shake256(), "shake-1_0", "shake-256");
    }

    #[cfg(feature = "xecdh")]
    #[test]
    fn xecdh() {
        use super::xecdh::*;
        check!(
            KeyGenTests::curve25519(),
            "xecdh-rfc7748",
            "keygen-curve25519"
        );
        check!(
            KeyVerTests::curve25519(),
            "xecdh-rfc7748",
            "keyver-curve25519"
        );
        check!(SscTests::curve25519(), "xecdh-rfc7748", "ssc-curve25519");
        let tests = SscTests::curve25519();
        assert_covers(&tests.prompts, &tests.projection);
    }
}

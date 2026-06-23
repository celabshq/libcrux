//! Spec-internal property tests for `hacspec_ml_kem` that are *not* already
//! covered by the inline `#[cfg(test)]` unit tests in `src/`.
//!
//! These exercise the public spec API without any reference to a concrete
//! implementation. Roundtrip / range properties that the in-module unit tests
//! already cover (byte_encode/decode, compress/decompress, NTT roundtrip and
//! multiplication, IND-CPA encrypt/decrypt, rejection sampling of zeros) are
//! intentionally not duplicated here.
//!
//! Spec ↔ impl byte equality lives on the impl side
//! (`libcrux-ml-kem/tests/cross_spec*.rs`).

mod serialization {
    use hacspec_ml_kem::serialize::*;

    /// `compress_then_serialize_message` ∘ `deserialize_then_decompress_message`
    /// is the identity on 32-byte messages (the 1-bit message encoding).
    #[test]
    fn message_serialize_roundtrip() {
        let msg_bytes = [0xABu8; 32];
        let poly = deserialize_then_decompress_message(&msg_bytes);
        let reencoded = compress_then_serialize_message(poly);
        assert_eq!(msg_bytes, reencoded);
    }
}

mod sampling_tests {
    use hacspec_ml_kem::parameters::*;
    use hacspec_ml_kem::sampling::*;

    /// CBD with eta=2: all coefficients should be in {0, 1, 2, 3327, 3328}.
    #[test]
    fn cbd_eta2_range() {
        let bytes = [0x55u8; 128]; // deterministic pattern
        let poly = sample_poly_cbd::<128, 1024>(2, &bytes);
        for (i, coeff) in poly.iter().enumerate() {
            assert!(
                coeff.val <= 2 || coeff.val >= FIELD_MODULUS - 2,
                "CBD eta=2 coefficient {} out of range: {}",
                i,
                coeff.val
            );
        }
    }

    /// CBD with eta=3: all coefficients should be in {0,1,2,3, 3326,3327,3328}.
    #[test]
    fn cbd_eta3_range() {
        let bytes = [0xAAu8; 192]; // deterministic pattern
        let poly = sample_poly_cbd::<192, 1536>(3, &bytes);
        for (i, coeff) in poly.iter().enumerate() {
            assert!(
                coeff.val <= 3 || coeff.val >= FIELD_MODULUS - 3,
                "CBD eta=3 coefficient {} out of range: {}",
                i,
                coeff.val
            );
        }
    }
}

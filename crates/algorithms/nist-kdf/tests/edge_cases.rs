//! Edge cases that are not covered by the ACVP test vectors.
//!
//! The expected outputs were computed with an independent implementation using
//! Python's `hmac` module.

use libcrux_hmac::{HmacSha256, HmacSha512};
use libcrux_nist_kdf::{feedback, two_step};

const K_IN: [u8; 32] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
];
const IV: [u8; 32] = [
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f,
];
const SALT: [u8; 16] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f,
];
const Z: [u8; 32] = [
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
    0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f,
];

#[test]
fn feedback_empty_fixed_info() {
    let expected = hex::decode(
        "b3eb0fcaabe78408c0763bba8071f092d006195b6a5f7d1f7073c3c9516adefb6f3eff3c0c3bd2f4",
    )
    .unwrap();

    // No fixed info slices at all.
    let mut k_out = [0; 40];
    feedback::kdf::<32, HmacSha256>(&mut k_out, &K_IN, &IV, &[]).unwrap();
    assert_eq!(k_out.as_slice(), expected);

    // A single empty fixed info slice.
    let mut k_out = [0; 40];
    feedback::kdf::<32, HmacSha256>(&mut k_out, &K_IN, &IV, &[&[]]).unwrap();
    assert_eq!(k_out.as_slice(), expected);
}

#[test]
fn two_step_empty_fixed_info() {
    let expected = hex::decode(
        "ea6e859704e11ebf53f5e5123ae08af98a4244d4c9901fdb76cecf8a509c0917ed674ef29095cb00",
    )
    .unwrap();

    // No fixed info slices at all.
    let mut k_out = [0; 40];
    two_step::kdf::<32, HmacSha256>(&mut k_out, &Z, &SALT, &IV, &[]).unwrap();
    assert_eq!(k_out.as_slice(), expected);

    // A single empty fixed info slice.
    let mut k_out = [0; 40];
    two_step::kdf::<32, HmacSha256>(&mut k_out, &Z, &SALT, &IV, &[&[]]).unwrap();
    assert_eq!(k_out.as_slice(), expected);
}

#[test]
fn feedback_empty_output() {
    let mut k_out = [];
    assert!(feedback::kdf::<32, HmacSha256>(&mut k_out, &K_IN, &IV, &[b"info"]).is_ok());
}

#[test]
fn two_step_empty_output() {
    // SP 800-56Cr2 requires L to be a positive integer.
    let mut k_out = [];
    assert!(two_step::kdf::<32, HmacSha256>(&mut k_out, &Z, &SALT, &IV, &[b"info"]).is_err());
}

#[test]
fn two_step_empty_salt_is_default_salt() {
    // SP 800-56Cr2 specifies an all-zero default salt with the length of a single
    // hash input block. Because HMAC zero-pads short keys, an empty salt is equivalent.
    let mut k_out_empty = [0; 40];
    let mut k_out_default = [0; 40];
    two_step::kdf::<32, HmacSha256>(&mut k_out_empty, &Z, &[], &IV, &[b"info"]).unwrap();
    two_step::kdf::<32, HmacSha256>(&mut k_out_default, &Z, &[0; 64], &IV, &[b"info"]).unwrap();
    assert_eq!(k_out_empty, k_out_default);

    let mut k_out_empty = [0; 80];
    let mut k_out_default = [0; 80];
    two_step::kdf::<64, HmacSha512>(&mut k_out_empty, &Z, &[], &IV, &[b"info"]).unwrap();
    two_step::kdf::<64, HmacSha512>(&mut k_out_default, &Z, &[0; 128], &IV, &[b"info"]).unwrap();
    assert_eq!(k_out_empty, k_out_default);
}

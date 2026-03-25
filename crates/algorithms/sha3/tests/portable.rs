mod test_vectors;

// Portable implementation tests
use crate::test_vectors::{DIGEST_LEN, DIGEST_LEN_SHAKE256, STRING_LEN, STRING_LEN_SHAKE256};
use libcrux_sha3::portable::incremental::Xof;
use libcrux_sha3::portable::{incremental, sha224, sha256, sha384, sha512, shake128, shake256};

#[test]
fn sha3_224() {
    let mut digest = [0u8; 28];

    sha224(&mut digest, test_vectors::EMPTY);
    assert_eq!(hex::encode(digest), test_vectors::sha3_224::EMPTY);

    sha224(&mut digest, test_vectors::HELLO);
    assert_eq!(hex::encode(digest), test_vectors::sha3_224::HELLO);

    sha224(&mut digest, test_vectors::STAR0);
    assert_eq!(hex::encode(digest), test_vectors::sha3_224::STAR0);
}

#[test]
fn sha3_256() {
    let mut digest = [0u8; 32];

    sha256(&mut digest, test_vectors::EMPTY);
    assert_eq!(hex::encode(digest), test_vectors::sha3_256::EMPTY);

    sha256(&mut digest, test_vectors::HELLO);
    assert_eq!(hex::encode(digest), test_vectors::sha3_256::HELLO);

    sha256(&mut digest, test_vectors::STAR0);
    assert_eq!(hex::encode(digest), test_vectors::sha3_256::STAR0);
}

#[test]
fn sha3_384() {
    let mut digest = [0u8; 48];

    sha384(&mut digest, test_vectors::EMPTY);
    assert_eq!(hex::encode(digest), test_vectors::sha3_384::EMPTY);

    sha384(&mut digest, test_vectors::HELLO);
    assert_eq!(hex::encode(digest), test_vectors::sha3_384::HELLO);

    sha384(&mut digest, test_vectors::STAR0);
    assert_eq!(hex::encode(digest), test_vectors::sha3_384::STAR0);
}

#[test]
fn sha3_512() {
    let mut digest = [0u8; 64];

    sha512(&mut digest, test_vectors::EMPTY);
    assert_eq!(hex::encode(digest), test_vectors::sha3_512::EMPTY);

    sha512(&mut digest, test_vectors::HELLO);
    assert_eq!(hex::encode(digest), test_vectors::sha3_512::HELLO);

    sha512(&mut digest, test_vectors::STAR0);
    assert_eq!(hex::encode(digest), test_vectors::sha3_512::STAR0);
}

#[test]
fn sha3_shake128() {
    let mut digest = [0u8; DIGEST_LEN];

    shake128(&mut digest, test_vectors::EMPTY);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake128::EMPTY_FIVE_BLOCKS[..STRING_LEN]
    );

    shake128(&mut digest, test_vectors::HELLO);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake128::HELLO_FIVE_BLOCKS[..STRING_LEN]
    );
    let mut digest = [0u8; 53];

    shake128(&mut digest, test_vectors::STAR0);
    assert_eq!(
        hex::encode(digest),
        test_vectors::shake128::STAR0_FIVE_BLOCKS[..53 * 2]
    );
}

#[test]
fn sha3_shake256() {
    let mut digest = [0u8; DIGEST_LEN];

    shake256(&mut digest, test_vectors::EMPTY);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake256::EMPTY_FIVE_BLOCKS[..STRING_LEN]
    );

    shake256(&mut digest, test_vectors::HELLO);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake256::HELLO_FIVE_BLOCKS[..STRING_LEN]
    );

    let mut digest = [0u8; 71];

    shake256(&mut digest, test_vectors::STAR0);
    assert_eq!(
        hex::encode(digest),
        test_vectors::shake256::STAR0_FIVE_BLOCKS[..71 * 2]
    );
}

#[test]
fn sha3_shake128_incremental() {
    // Test squeezing 1 block (168 bytes)
    let mut state = incremental::shake128_init();
    incremental::shake128_absorb_final(&mut state, test_vectors::HELLO);

    // Test squeezing next block (168 bytes)
    let mut digest = [0u8; DIGEST_LEN * 4];
    incremental::shake128_squeeze_next_block(&mut state, &mut digest);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake128::HELLO_FIVE_BLOCKS[336..672]
    );

    // ---

    // Test squeezing 3 blocks (504 bytes)
    state = incremental::shake128_init();
    incremental::shake128_absorb_final(&mut state, test_vectors::HELLO);

    let mut digest = [0u8; DIGEST_LEN * 12];
    incremental::shake128_squeeze_first_three_blocks(&mut state, &mut digest);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake128::HELLO_FIVE_BLOCKS[..STRING_LEN * 12]
    );

    // ---

    // Test squeezing 5 blocks (840 bytes)
    state = incremental::shake128_init();
    incremental::shake128_absorb_final(&mut state, test_vectors::HELLO);

    let mut digest = [0u8; DIGEST_LEN * 20];
    incremental::shake128_squeeze_first_five_blocks(&mut state, &mut digest);
    assert_eq!(
        hex::encode(digest),
        test_vectors::shake128::HELLO_FIVE_BLOCKS
    );
}

#[test]
fn sha3_shake256_incremental() {
    // Test squeezing 1 block (136 bytes for SHAKE256, not 168)
    let mut state = incremental::shake256_init();
    incremental::shake256_absorb_final(&mut state, test_vectors::HELLO);

    let mut digest = [0u8; DIGEST_LEN_SHAKE256];
    incremental::shake256_squeeze_first_block(&mut state, &mut digest);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake256::HELLO_FIVE_BLOCKS[..STRING_LEN_SHAKE256]
    );

    // Test squeezing next block (136 bytes)
    incremental::shake256_squeeze_next_block(&mut state, &mut digest);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake256::HELLO_FIVE_BLOCKS
            [DIGEST_LEN_SHAKE256 * 2..DIGEST_LEN_SHAKE256 * 4]
    );
}

#[test]
fn sha3_shake128_absorb() {
    let mut state = incremental::Shake128Xof::new();
    state.absorb_final(b"Hello, ");

    let mut digest = [0u8; 32];
    state.squeeze(&mut digest);
    let expected = "62dac7f538d3c56e66a1e0ccda69f4b6c8f6269572ad9312c7a04a2228b474a5";
    assert_eq!(hex::encode(digest), expected);

    // ---

    state = incremental::Shake128Xof::new();
    state.absorb(b"Hello, ");
    state.absorb_final(b"World!");

    state.squeeze(&mut digest);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake128::HELLO_FIVE_BLOCKS[..64]
    );

    // ---

    state = incremental::Shake128Xof::new();
    state.absorb(b"Hello, ");
    state.absorb_final(&[]);

    state.squeeze(&mut digest);
    assert_eq!(hex::encode(digest), expected);
}

#[test]
fn sha3_shake256_absorb() {
    let mut state = incremental::Shake256Xof::new();
    state.absorb_final(b"Hello, ");

    let mut digest = [0u8; 32];
    state.squeeze(&mut digest);
    let expected = "018680a686f24f889fe4613dba0058ea1b035b7270a8c26b363f42557bbd991a";
    assert_eq!(hex::encode(digest), expected);

    // ---

    state = incremental::Shake256Xof::new();
    state.absorb(b"Hello, ");
    state.absorb_final(b"World!");

    state.squeeze(&mut digest);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake256::HELLO_FIVE_BLOCKS[..64]
    );

    // ---

    state = incremental::Shake256Xof::new();
    state.absorb(b"Hello, ");
    state.absorb_final(&[]);

    state.squeeze(&mut digest);
    assert_eq!(hex::encode(digest), expected);
}

// === Regression tests ===

/// Regression test for XOF squeeze bug: when squeeze is called with a buffer
/// larger than RATE bytes, the first output block was skipped due to an extra
/// keccakf1600() call before the first extraction. This test requests 200 bytes
/// (> SHAKE128 RATE of 168) in a single squeeze call and compares against the
/// known one-shot output.
#[test]
fn bug1_xof_squeeze_skips_first_block_shake128() {
    // One-shot: known correct output
    let mut expected = [0u8; 200];
    shake128(&mut expected, test_vectors::HELLO);

    // Streaming XOF: was buggy when out_len > RATE (168 for SHAKE128)
    let mut state = incremental::Shake128Xof::new();
    state.absorb_final(test_vectors::HELLO);
    let mut actual = [0u8; 200];
    state.squeeze(&mut actual);

    assert_eq!(actual, expected);
}

/// Same regression test for SHAKE256 (RATE = 136).
#[test]
fn bug1_xof_squeeze_skips_first_block_shake256() {
    let mut expected = [0u8; 200];
    shake256(&mut expected, test_vectors::HELLO);

    let mut state = incremental::Shake256Xof::new();
    state.absorb_final(test_vectors::HELLO);
    let mut actual = [0u8; 200];
    state.squeeze(&mut actual);

    assert_eq!(actual, expected);
}

/// Regression test: multiple squeeze calls should produce the same output as
/// a single large squeeze (i.e., streaming squeeze is consistent).
#[test]
fn bug1_xof_squeeze_multi_call_consistency() {
    // Single large squeeze
    let mut state1 = incremental::Shake128Xof::new();
    state1.absorb_final(test_vectors::HELLO);
    let mut single = [0u8; 504]; // 3 * RATE(168)
    state1.squeeze(&mut single);

    // Multiple small squeezes
    let mut state2 = incremental::Shake128Xof::new();
    state2.absorb_final(test_vectors::HELLO);
    let mut multi = [0u8; 504];
    state2.squeeze(&mut multi[0..168]);
    state2.squeeze(&mut multi[168..336]);
    state2.squeeze(&mut multi[336..504]);

    assert_eq!(single, multi);
}

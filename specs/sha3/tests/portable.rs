/// Tests ported from ../../crates/algorithms/sha3/tests/portable.rs
/// Tests the spec against the same hardcoded test vectors used by the reference implementation.
mod test_vectors;

use hacspec_sha3::*;

const DIGEST_LEN: usize = 42;
const STRING_LEN: usize = DIGEST_LEN * 2;

#[test]
fn test_sha3_224() {
    assert_eq!(
        hex::encode(sha3_224(test_vectors::EMPTY)),
        test_vectors::sha3_224::EMPTY
    );
    assert_eq!(
        hex::encode(sha3_224(test_vectors::HELLO)),
        test_vectors::sha3_224::HELLO
    );
    assert_eq!(
        hex::encode(sha3_224(test_vectors::STAR0)),
        test_vectors::sha3_224::STAR0
    );
}

#[test]
fn test_sha3_256() {
    assert_eq!(
        hex::encode(sha3_256(test_vectors::EMPTY)),
        test_vectors::sha3_256::EMPTY
    );
    assert_eq!(
        hex::encode(sha3_256(test_vectors::HELLO)),
        test_vectors::sha3_256::HELLO
    );
    assert_eq!(
        hex::encode(sha3_256(test_vectors::STAR0)),
        test_vectors::sha3_256::STAR0
    );
}

#[test]
fn test_sha3_384() {
    assert_eq!(
        hex::encode(sha3_384(test_vectors::EMPTY)),
        test_vectors::sha3_384::EMPTY
    );
    assert_eq!(
        hex::encode(sha3_384(test_vectors::HELLO)),
        test_vectors::sha3_384::HELLO
    );
    assert_eq!(
        hex::encode(sha3_384(test_vectors::STAR0)),
        test_vectors::sha3_384::STAR0
    );
}

#[test]
fn test_sha3_512() {
    assert_eq!(
        hex::encode(sha3_512(test_vectors::EMPTY)),
        test_vectors::sha3_512::EMPTY
    );
    assert_eq!(
        hex::encode(sha3_512(test_vectors::HELLO)),
        test_vectors::sha3_512::HELLO
    );
    assert_eq!(
        hex::encode(sha3_512(test_vectors::STAR0)),
        test_vectors::sha3_512::STAR0
    );
}

#[test]
fn test_shake128() {
    // Test with 42-byte output (DIGEST_LEN)
    let digest = shake128::<DIGEST_LEN>(test_vectors::EMPTY);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake128::EMPTY_FIVE_BLOCKS[..STRING_LEN]
    );

    let digest = shake128::<DIGEST_LEN>(test_vectors::HELLO);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake128::HELLO_FIVE_BLOCKS[..STRING_LEN]
    );

    // Test with 53-byte output
    let digest = shake128::<53>(test_vectors::STAR0);
    assert_eq!(
        hex::encode(digest),
        test_vectors::shake128::STAR0_FIVE_BLOCKS[..53 * 2]
    );
}

#[test]
fn test_shake256() {
    // Test with 42-byte output (DIGEST_LEN)
    let digest = shake256::<DIGEST_LEN>(test_vectors::EMPTY);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake256::EMPTY_FIVE_BLOCKS[..STRING_LEN]
    );

    let digest = shake256::<DIGEST_LEN>(test_vectors::HELLO);
    assert_eq!(
        hex::encode(digest),
        &test_vectors::shake256::HELLO_FIVE_BLOCKS[..STRING_LEN]
    );

    // Test with 71-byte output
    let digest = shake256::<71>(test_vectors::STAR0);
    assert_eq!(
        hex::encode(digest),
        test_vectors::shake256::STAR0_FIVE_BLOCKS[..71 * 2]
    );
}

// Multi-block squeeze tests: verify we produce the full 5-block outputs correctly.

#[test]
fn shake128_five_blocks_empty() {
    // 5 blocks of SHAKE128 = 5 * 168 = 840 bytes
    let digest = shake128::<840>(test_vectors::EMPTY);
    assert_eq!(
        hex::encode(digest),
        test_vectors::shake128::EMPTY_FIVE_BLOCKS
    );
}

#[test]
fn shake128_five_blocks_hello() {
    let digest = shake128::<840>(test_vectors::HELLO);
    assert_eq!(
        hex::encode(digest),
        test_vectors::shake128::HELLO_FIVE_BLOCKS
    );
}

#[test]
fn shake128_five_blocks_star0() {
    let digest = shake128::<840>(test_vectors::STAR0);
    assert_eq!(
        hex::encode(digest),
        test_vectors::shake128::STAR0_FIVE_BLOCKS
    );
}

#[test]
fn shake256_five_blocks_empty() {
    // 5 blocks of SHAKE256 = 5 * 136 = 680 bytes
    let digest = shake256::<680>(test_vectors::EMPTY);
    assert_eq!(
        hex::encode(digest),
        test_vectors::shake256::EMPTY_FIVE_BLOCKS
    );
}

#[test]
fn shake256_five_blocks_hello() {
    let digest = shake256::<680>(test_vectors::HELLO);
    assert_eq!(
        hex::encode(digest),
        test_vectors::shake256::HELLO_FIVE_BLOCKS
    );
}

#[test]
fn shake256_five_blocks_star0() {
    let digest = shake256::<680>(test_vectors::STAR0);
    assert_eq!(
        hex::encode(digest),
        test_vectors::shake256::STAR0_FIVE_BLOCKS
    );
}

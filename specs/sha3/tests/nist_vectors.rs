use hacspec_sha3::*;

/// Helper to decode a hex string to a byte vector.
fn hex_to_bytes(s: &str) -> Vec<u8> {
    hex::decode(s).expect("valid hex")
}

// ============================================================
// SHA3-224 NIST vectors
// ============================================================

#[test]
fn sha3_224_empty() {
    let expected = hex_to_bytes("6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7");
    assert_eq!(sha3_224(b"").to_vec(), expected);
}

#[test]
fn sha3_224_abc() {
    let expected = hex_to_bytes("e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf");
    assert_eq!(sha3_224(b"abc").to_vec(), expected);
}

#[test]
fn sha3_224_448bit() {
    // "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"
    let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
    let expected = hex_to_bytes("8a24108b154ada21c9fd5574494479ba5c7e7ab76ef264ead0fcce33");
    assert_eq!(sha3_224(msg).to_vec(), expected);
}

// ============================================================
// SHA3-256 NIST vectors
// ============================================================

#[test]
fn sha3_256_empty() {
    let expected = hex_to_bytes("a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a");
    assert_eq!(sha3_256(b"").to_vec(), expected);
}

#[test]
fn sha3_256_abc() {
    let expected = hex_to_bytes("3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532");
    assert_eq!(sha3_256(b"abc").to_vec(), expected);
}

#[test]
fn sha3_256_448bit() {
    let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
    let expected = hex_to_bytes("41c0dba2a9d6240849100376a8235e2c82e1b9998a999e21db32dd97496d3376");
    assert_eq!(sha3_256(msg).to_vec(), expected);
}

#[test]
fn sha3_256_896bit() {
    // "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"
    let msg = b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";
    let expected = hex_to_bytes("916f6061fe879741ca6469b43971dfdb28b1a32dc36cb3254e812be27aad1d18");
    assert_eq!(sha3_256(msg).to_vec(), expected);
}

// ============================================================
// SHA3-384 NIST vectors
// ============================================================

#[test]
fn sha3_384_empty() {
    let expected = hex_to_bytes(
        "0c63a75b845e4f7d01107d852e4c2485c51a50aaaa94fc61995e71bbee983a2ac3713831264adb47fb6bd1e058d5f004",
    );
    assert_eq!(sha3_384(b"").to_vec(), expected);
}

#[test]
fn sha3_384_abc() {
    let expected = hex_to_bytes(
        "ec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e49be4b298d88cea927ac7f539f1edf228376d25",
    );
    assert_eq!(sha3_384(b"abc").to_vec(), expected);
}

// ============================================================
// SHA3-512 NIST vectors
// ============================================================

#[test]
fn sha3_512_empty() {
    let expected = hex_to_bytes(
        "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26",
    );
    assert_eq!(sha3_512(b"").to_vec(), expected);
}

#[test]
fn sha3_512_abc() {
    let expected = hex_to_bytes(
        "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0",
    );
    assert_eq!(sha3_512(b"abc").to_vec(), expected);
}

// ============================================================
// SHAKE128 NIST vectors
// ============================================================

#[test]
fn shake128_empty_32() {
    let expected = hex_to_bytes("7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26");
    assert_eq!(shake128::<32>(b"").to_vec(), expected);
}

#[test]
fn shake128_abc_32() {
    let expected = hex_to_bytes("5881092dd818bf5cf8a3ddb793fbcba74097d5c526a6d35f97b83351940f2cc8");
    assert_eq!(shake128::<32>(b"abc").to_vec(), expected);
}

// ============================================================
// SHAKE256 NIST vectors
// ============================================================

#[test]
fn shake256_empty_32() {
    let expected = hex_to_bytes("46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762f");
    assert_eq!(shake256::<32>(b"").to_vec(), expected);
}

#[test]
fn shake256_abc_32() {
    let expected = hex_to_bytes("483366601360a8771c6863080cc4114d8db44530f8f1e1ee4f94ea37e78b5739");
    assert_eq!(shake256::<32>(b"abc").to_vec(), expected);
}

// ============================================================
// SHAKE with longer output (squeeze multiple blocks)
// ============================================================

#[test]
fn shake256_empty_64() {
    let expected = hex_to_bytes(
        "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762fd75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be",
    );
    assert_eq!(shake256::<64>(b"").to_vec(), expected);
}

// ============================================================
// Padding boundary tests
// ============================================================

#[test]
fn sha3_256_rate_minus_1() {
    // Message of length rate-1 = 135 bytes: padding fits in the last byte
    let msg = vec![0xABu8; 135];
    let result = sha3_256(&msg);
    assert_eq!(result.len(), 32);
}

#[test]
fn sha3_256_exact_rate() {
    // Message of length rate = 136 bytes: forces an extra block for padding
    let msg = vec![0xABu8; 136];
    let result = sha3_256(&msg);
    assert_eq!(result.len(), 32);
}

#[test]
fn sha3_256_rate_plus_1() {
    // Message of length rate+1 = 137 bytes
    let msg = vec![0xABu8; 137];
    let result = sha3_256(&msg);
    assert_eq!(result.len(), 32);
}

// ============================================================
// Proptest: determinism and output length
// ============================================================

use proptest::prelude::*;

proptest! {
    #[test]
    fn sha3_256_deterministic(msg in proptest::collection::vec(any::<u8>(), 0..512)) {
        let h1 = sha3_256(&msg);
        let h2 = sha3_256(&msg);
        prop_assert_eq!(h1, h2);
    }

    #[test]
    fn sha3_512_output_length(msg in proptest::collection::vec(any::<u8>(), 0..512)) {
        let h = sha3_512(&msg);
        prop_assert_eq!(h.len(), 64);
    }

    #[test]
    fn shake128_output_length(msg in proptest::collection::vec(any::<u8>(), 0..256)) {
        let h = shake128::<16>(&msg);
        prop_assert_eq!(h.len(), 16);
    }

    #[test]
    fn shake256_output_length(msg in proptest::collection::vec(any::<u8>(), 0..256)) {
        let h = shake256::<48>(&msg);
        prop_assert_eq!(h.len(), 48);
    }
}

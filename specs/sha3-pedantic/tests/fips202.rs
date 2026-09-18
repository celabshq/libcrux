//! Tests that check this spec against the document it transcribes, and
//! against the byte-level spec it sits next to.

use hacspec_sha3_pedantic as pedantic;
use pedantic::bits::{b2h, h2b, h2b_full};
use pedantic::sponge::pad10_star_1;
use pedantic::state_array::StateArray;

// ---------------------------------------------------------------------------
// The Standard's own worked examples
// ---------------------------------------------------------------------------

/// Appendix B.1, Table 5: `h2b(0xA32E, 14) = 1100 0101 0111 01`.
#[test]
fn h2b_table_5() {
    let bits = h2b(&[0xA3, 0x2E], 14);
    let expected: Vec<bool> = "11000101011101".chars().map(|c| c == '1').collect();
    assert_eq!(bits, expected);
}

/// `b2h` inverts `h2b` on byte-aligned strings (Appendix B.1).
#[test]
fn b2h_inverts_h2b() {
    let bytes: Vec<u8> = (0u8..=255).collect();
    assert_eq!(b2h(&h2b_full(&bytes)), bytes);
}

/// Algorithm 9: `m + len(pad10*1(x, m))` is a positive multiple of `x`, the
/// padding starts and ends with a `1`, and everything between is `0`.
#[test]
fn pad10_star_1_is_well_formed() {
    for x in [8usize, 136, 168, 1088, 1152] {
        for m in 0..(3 * x) {
            let p = pad10_star_1(x, m);
            assert!((m + p.len()).is_multiple_of(x) && m + p.len() > 0);
            assert!(p.len() >= 2);
            assert!(p[0] && p[p.len() - 1]);
            assert!(p[1..p.len() - 1].iter().all(|b| !b));
        }
    }
}

/// Appendix B.2, Table 6: for byte-aligned messages the appended bytes are
/// `0x86` / `0x0680` / `0x06 .. 0x80` for the hash functions and
/// `0x9F` / `0x1F80` / `0x1F .. 0x80` for the XOFs.
#[test]
fn padding_bytes_table_6() {
    // The bits appended to M are the domain suffix followed by pad10*1 over
    // the suffixed message.
    fn appended(suffix: &[bool], rate_bits: usize, message_bytes: usize) -> Vec<u8> {
        let m = 8 * message_bytes;
        let mut bits = suffix.to_vec();
        bits.extend(pad10_star_1(rate_bits, m + suffix.len()));
        b2h(&bits)
    }

    let r = 1088; // SHA3-256 / SHAKE256 rate; irrelevant here beyond r/8 = 136
    let q_of = |bytes: usize| (r / 8) - (bytes % (r / 8));

    for (suffix, one, two, first) in [
        (
            &pedantic::sha3::HASH_SUFFIX[..],
            0x86u8,
            [0x06u8, 0x80u8],
            0x06u8,
        ),
        (
            &pedantic::sha3::XOF_SUFFIX[..],
            0x9Fu8,
            [0x1Fu8, 0x80u8],
            0x1Fu8,
        ),
    ] {
        // q = 1
        let bytes = (r / 8) - 1;
        assert_eq!(q_of(bytes), 1);
        assert_eq!(appended(suffix, r, bytes), vec![one]);

        // q = 2
        let bytes = (r / 8) - 2;
        assert_eq!(q_of(bytes), 2);
        assert_eq!(appended(suffix, r, bytes), two.to_vec());

        // q > 2
        let bytes = 0;
        let q = q_of(bytes);
        assert!(q > 2);
        let mut expected = vec![first];
        expected.extend(std::iter::repeat_n(0x00, q - 2));
        expected.push(0x80);
        assert_eq!(appended(suffix, r, bytes), expected);
    }
}

// ---------------------------------------------------------------------------
// Known answers
// ---------------------------------------------------------------------------

#[test]
fn digests_of_the_empty_message() {
    assert_eq!(
        hex::encode(pedantic::bytes::sha3_224(b"")),
        "6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7"
    );
    assert_eq!(
        hex::encode(pedantic::bytes::sha3_256(b"")),
        "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a"
    );
    assert_eq!(
        hex::encode(pedantic::bytes::sha3_384(b"")),
        "0c63a75b845e4f7d01107d852e4c2485c51a50aaaa94fc61995e71bbee983a2ac3713831264adb47fb6bd1e058d5f004"
    );
    assert_eq!(
        hex::encode(pedantic::bytes::sha3_512(b"")),
        "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26"
    );
    assert_eq!(
        hex::encode(pedantic::bytes::shake128(b"", 32)),
        "7f9c2ba4e88f827d616045507605853ed73b8093f6efbc88eb1a6eacfa66ef26"
    );
    assert_eq!(
        hex::encode(pedantic::bytes::shake256(b"", 32)),
        "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762f"
    );
}

/// "abc", the other vector everyone knows.
#[test]
fn digest_of_abc() {
    assert_eq!(
        hex::encode(pedantic::bytes::sha3_256(b"abc")),
        "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"
    );
}

// ---------------------------------------------------------------------------
// Agreement with the byte-level spec next door
// ---------------------------------------------------------------------------

/// Lengths around the rate boundaries, where padding behaviour changes:
/// `r/8` is 144, 136, 104, 72 bytes for SHA3-224/256/384/512 and 168/136 for
/// SHAKE128/256.
fn interesting_lengths() -> Vec<usize> {
    let mut lengths = vec![0usize, 1, 2, 7, 8, 63, 64, 65];
    for rate_bytes in [72usize, 104, 136, 144, 168] {
        for delta in [2usize, 1] {
            lengths.push(rate_bytes - delta);
        }
        lengths.push(rate_bytes);
        lengths.push(rate_bytes + 1);
        lengths.push(2 * rate_bytes);
        lengths.push(2 * rate_bytes + 1);
    }
    lengths.sort_unstable();
    lengths.dedup();
    lengths
}

fn message(len: usize) -> Vec<u8> {
    // Deterministic, and not all-equal bytes.
    (0..len)
        .map(|i| (i as u8).wrapping_mul(37).wrapping_add(11))
        .collect()
}

#[test]
fn agrees_with_hacspec_sha3() {
    for len in interesting_lengths() {
        let m = message(len);
        assert_eq!(
            pedantic::bytes::sha3_224(&m),
            hacspec_sha3::sha3_224(&m),
            "SHA3-224 at length {len}"
        );
        assert_eq!(
            pedantic::bytes::sha3_256(&m),
            hacspec_sha3::sha3_256(&m),
            "SHA3-256 at length {len}"
        );
        assert_eq!(
            pedantic::bytes::sha3_384(&m),
            hacspec_sha3::sha3_384(&m),
            "SHA3-384 at length {len}"
        );
        assert_eq!(
            pedantic::bytes::sha3_512(&m),
            hacspec_sha3::sha3_512(&m),
            "SHA3-512 at length {len}"
        );
        assert_eq!(
            pedantic::bytes::shake128(&m, 32)[..],
            hacspec_sha3::shake128::<32>(&m)[..],
            "SHAKE128 at length {len}"
        );
        assert_eq!(
            pedantic::bytes::shake256(&m, 32)[..],
            hacspec_sha3::shake256::<32>(&m)[..],
            "SHAKE256 at length {len}"
        );
    }
}

/// A XOF is squeezed across several blocks: 200 bytes needs two SHAKE128
/// squeezes (rate 168 bytes) and two SHAKE256 squeezes (rate 136 bytes).
#[test]
fn xof_output_longer_than_the_rate() {
    let m = message(50);
    assert_eq!(
        pedantic::bytes::shake128(&m, 200)[..],
        hacspec_sha3::shake128::<200>(&m)[..]
    );
    assert_eq!(
        pedantic::bytes::shake256(&m, 200)[..],
        hacspec_sha3::shake256::<200>(&m)[..]
    );
}

// ---------------------------------------------------------------------------
// Structural properties from Sec. 3
// ---------------------------------------------------------------------------

/// Sec. 3.1.2 and 3.1.3 are inverse to each other.
#[test]
fn state_array_conversions_round_trip() {
    let bits: Vec<bool> = (0..1600).map(|i| (i * 7 + 3) % 5 == 0).collect();
    assert_eq!(StateArray::<64>::from_bits(&bits).to_bits(), bits);
}

/// Sec. 3.4: `KECCAK-f[b] = KECCAK-p[b, 12 + 2l]`, and the rounds of
/// `KECCAK-p[b, n_r]` are the *last* `n_r` rounds of `KECCAK-f[b]`.
#[test]
fn keccak_p_is_the_tail_of_keccak_f() {
    use pedantic::keccak_p::{keccak_f, keccak_p, rnd};

    let s: Vec<bool> = (0..1600).map(|i| i % 3 == 0).collect();
    assert_eq!(keccak_f::<64>(&s), keccak_p::<64>(&s, 24));

    // The last 19 rounds of KECCAK-f[1600], applied by hand, are KECCAK-p[1600, 19].
    let mut a = StateArray::<64>::from_bits(&s);
    for i_r in (24 - 19)..24 {
        a = rnd(&a, i_r as i64);
    }
    assert_eq!(a.to_bits(), keccak_p::<64>(&s, 19));
}

/// Table 2: the offsets `ρ` produces by its `(x, y)` walk are the tabulated
/// ones. The table is indexed with `x` across and `y` down, as printed.
#[test]
fn rho_offsets_match_table_2() {
    // Same as in the spec itself: the (x, y) indices are the point here.
    #![allow(clippy::needless_range_loop)]

    use pedantic::step_mappings::rho;

    // Table 2 of the Standard, transcribed as printed there: the columns run
    // x = 3, 4, 0, 1, 2 and the rows y = 2, 1, 0, 4, 3 (the labelling
    // convention of Figure 2), and the offsets are the unreduced
    // (t+1)(t+2)/2, so they exceed w = 64.
    //
    //        x = 3  x = 4  x = 0  x = 1  x = 2
    // y = 2    153    231      3     10    171
    // y = 1     55    276     36    300      6
    // y = 0     28     91      0      1    190
    // y = 4    120     78    210     66    253
    // y = 3     21    136    105     45     15
    //
    // Re-indexed here as table[x][y]:
    let table = [
        [0usize, 36, 3, 105, 210], // x = 0
        [1, 300, 10, 45, 66],      // x = 1
        [190, 6, 171, 15, 253],    // x = 2
        [28, 55, 153, 21, 120],    // x = 3
        [91, 276, 231, 136, 78],   // x = 4
    ];

    // Put a single 1 at z = 0 in every lane; after ρ that bit sits at the
    // lane's offset.
    let mut a = StateArray::<64>::zero();
    for x in 0..5 {
        for y in 0..5 {
            a.a[x][y][0] = true;
        }
    }
    let out = rho(&a);
    for x in 0..5 {
        for y in 0..5 {
            let offset = table[x][y] % 64;
            for z in 0..64 {
                assert_eq!(out.a[x][y][z], z == offset, "lane ({x}, {y}) bit {z}");
            }
        }
    }
}

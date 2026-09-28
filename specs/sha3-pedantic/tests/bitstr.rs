//! `BitStr` against a naive `Vec<bool>` reference.
//!
//! `bits::BitStr` is opaque to the extraction, so its Lean model in
//! `Assumptions/FunsExternal.lean` is hand-written and the correspondence
//! between that model and the bit-packed Rust below it is assumed rather than
//! proved. These tests are what stands in for the proof: every operation the
//! model names is checked against the obvious `Vec<bool>` implementation of
//! the same Sec. 2.3 notation, on lengths that exercise the byte boundaries
//! the packing has and the model does not.

use hacspec_sha3_pedantic::bits::{b2h, h2b, h2b_full, BitStr};
use hacspec_sha3_pedantic::nat::Nat;
use proptest::prelude::*;

/// The reference: one bit per `bool`, exactly as the model reads it.
fn reference_bits(bytes: &[u8]) -> Vec<bool> {
    let mut out = Vec::new();
    for b in bytes {
        for j in 0..8 {
            out.push((b >> j) & 1 == 1);
        }
    }
    out
}

/// Lengths that straddle the packing's byte boundaries.
fn interesting_lens() -> Vec<usize> {
    vec![0, 1, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 200]
}

#[test]
fn from_bytes_matches_reference() {
    for len in interesting_lens() {
        let bytes: Vec<u8> = (0..len).map(|i| (i * 37 + 11) as u8).collect();
        let s = BitStr::from_bytes(&bytes);
        let r = reference_bits(&bytes);
        assert_eq!(s.len(), Nat::from_usize(r.len()), "len at {len}");
        assert_eq!(s.to_bits(), r, "bits at {len}");
    }
}

#[test]
fn to_bytes_inverts_from_bytes() {
    for len in interesting_lens() {
        let bytes: Vec<u8> = (0..len).map(|i| (i * 89 + 3) as u8).collect();
        assert_eq!(BitStr::from_bytes(&bytes).to_bytes(), bytes, "at {len}");
        assert_eq!(b2h(&h2b_full(&bytes)), bytes, "via h2b/b2h at {len}");
    }
}

#[test]
fn from_bits_to_bits_roundtrip() {
    for len in interesting_lens() {
        let bits: Vec<bool> = (0..len).map(|i| i % 3 == 0).collect();
        assert_eq!(BitStr::from_bits(&bits).to_bits(), bits, "at {len}");
    }
}

/// `b2h` pads with zeros to a whole byte — Algorithm 11 — so a truncated
/// string must not leave stale bits behind in the final byte.
#[test]
fn trunc_zeroes_the_tail() {
    let all_ones = BitStr::from_bytes(&[0xFF; 4]);
    for s in 0..=32u64 {
        let t = all_ones.trunc(Nat::new(s));
        assert_eq!(t.len(), Nat::new(s));
        let expected_bytes = (s as usize).div_ceil(8);
        assert_eq!(t.to_bytes().len(), expected_bytes, "byte count at {s}");
        if s % 8 != 0 {
            let last = *t.to_bytes().last().unwrap();
            assert_eq!(last >> (s % 8), 0, "stale bits above bit {} ", s % 8);
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 256, ..ProptestConfig::default() })]

    /// `len`, `bit`, `concat`, `trunc` and `slice` against the reference.
    #[test]
    fn ops_match_reference(
        xs in prop::collection::vec(any::<u8>(), 0..64),
        ys in prop::collection::vec(any::<u8>(), 0..64),
        k in 0usize..512,
    ) {
        let (x, y) = (BitStr::from_bytes(&xs), BitStr::from_bytes(&ys));
        let (rx, ry) = (reference_bits(&xs), reference_bits(&ys));

        prop_assert_eq!(x.len(), Nat::from_usize(rx.len()));
        for i in 0..rx.len() {
            prop_assert_eq!(x.bit(Nat::from_usize(i)), rx[i], "bit {}", i);
        }

        let mut rcat = rx.clone();
        rcat.extend(&ry);
        prop_assert_eq!(x.concat(&y).to_bits(), rcat.clone());

        let s = k % (rcat.len() + 1);
        prop_assert_eq!(x.concat(&y).trunc(Nat::from_usize(s)).to_bits(), rcat[..s].to_vec());

        if !rx.is_empty() {
            let from = k % rx.len();
            let n = (k * 7) % (rx.len() - from + 1);
            prop_assert_eq!(
                x.slice(Nat::from_usize(from), Nat::from_usize(n)).to_bits(),
                rx[from..from + n].to_vec()
            );
        }
    }

    /// `zeros` and `h2b` at an explicit `n` (Algorithm 10).
    #[test]
    fn zeros_and_h2b_match_reference(
        n in 0u64..300,
        bytes in prop::collection::vec(any::<u8>(), 0..40),
    ) {
        prop_assert_eq!(BitStr::zeros(Nat::new(n)).to_bits(), vec![false; n as usize]);

        let avail = 8 * bytes.len() as u64;
        let take = if avail == 0 { 0 } else { n % (avail + 1) };
        prop_assert_eq!(
            h2b(&bytes, Nat::new(take)).to_bits(),
            reference_bits(&bytes)[..take as usize].to_vec()
        );
    }
}

//! The state and the state array — FIPS 202, Sec. 3.1.
//!
//! The state of `KECCAK-p[b, n_r]` is `b` bits. With `w = b/25` and
//! `l = log2(w)`, it is represented either as a `b`-bit string `S` or as a
//! 5-by-5-by-`w` array `A` of bits, indexed by triples `(x, y, z)` with
//! `0 ≤ x < 5`, `0 ≤ y < 5`, `0 ≤ z < w` (Table 1 lists the seven widths).

use crate::bits::{Bit, BitString};

/// A 5-by-5-by-`W` array of bits, `A[x, y, z]`.
///
/// `W` is the lane size `w` from Table 1; `b = 25·W`.
// `Debug` is for Rust callers only: the derived `fmt` does not extract (the
// generated instance has a shape the Lean core model does not match).
#[cfg_attr(not(hax_backend_lean), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct StateArray<const W: usize> {
    /// `a[x][y][z]` is `A[x, y, z]`.
    pub a: [[[Bit; W]; 5]; 5],
}

impl<const W: usize> StateArray<W> {
    /// `w` — the lane size, `b/25`.
    pub const W: usize = W;

    /// `b` — the width of the permutation, `25w`.
    pub const B: usize = 25 * W;

    /// `l = log2(w)` — Table 1, which lists the seven admissible widths:
    ///
    /// | b | 25 | 50 | 100 | 200 | 400 | 800 | 1600 |
    /// |---|---|---|---|---|---|---|---|
    /// | w | 1 | 2 | 4 | 8 | 16 | 32 | 64 |
    /// | l | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
    pub const L: usize = match W {
        1 => 0,
        2 => 1,
        4 => 2,
        8 => 3,
        16 => 4,
        32 => 5,
        64 => 6,
        _ => panic!("w must be one of the seven widths of Table 1"),
    };

    /// The all-zero state array, `0^b` seen as an array.
    pub fn zero() -> Self {
        StateArray {
            a: [[[false; W]; 5]; 5],
        }
    }

    /// Sec. 3.1.2: converting strings to state arrays,
    /// `A[x, y, z] = S[w(5y + x) + z]`.
    pub fn from_bits(s: &[Bit]) -> StateArray<W> {
        assert!(s.len() == 25 * W, "Sec. 3.1.2 expects a string of b bits");
        let mut out = Self::zero();
        for x in 0..5 {
            for y in 0..5 {
                for z in 0..W {
                    out.a[x][y][z] = s[W * (5 * y + x) + z];
                }
            }
        }
        out
    }

    /// Sec. 3.1.3: converting state arrays to strings.
    ///
    /// `Lane(i, j) = A[i, j, 0] || A[i, j, 1] || … || A[i, j, w-1]`,
    /// `Plane(j) = Lane(0, j) || Lane(1, j) || … || Lane(4, j)`, and
    /// `S = Plane(0) || Plane(1) || … || Plane(4)`.
    pub fn to_bits(&self) -> BitString {
        let mut s = Vec::with_capacity(Self::B);
        for y in 0..5 {
            for x in 0..5 {
                for z in 0..W {
                    s.push(self.a[x][y][z]);
                }
            }
        }
        s
    }
}

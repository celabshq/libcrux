//! Bit strings and the operations on them — FIPS 202, Sec. 2.3 and App. B.1.
//!
//! The Standard's objects are bit strings of arbitrary length, so this module
//! has two layers.
//!
//! [`BitStr`] is the arbitrary-length one: messages, the padded message `P`,
//! and the output `Z`. It is **opaque to the extraction** — the Lean side does
//! not see the representation below, but a hand-written model in
//! `Assumptions/TypesExternal.lean` that reads it as a plain list of bits, of
//! any length whatsoever. That is the point of the type: the Standard puts no
//! bound on `len(M)`, and neither should the specification.
//!
//! The representation here is bit-packed, one bit per bit rather than the byte
//! per bit a `Vec<bool>` would spend, and the length is a `u64` rather than a
//! `usize`. Both choices are about reach:
//!
//! * `isize::MAX` bytes of packed storage is `8 * isize::MAX` bits, which
//!   exceeds `u64::MAX` on 32- and 64-bit targets alike, so the `len` field is
//!   the only limit left and no chunked representation would buy anything;
//! * a `u64` length keeps the ceiling off the target's word size. With a
//!   `usize` length, `8 * m.len()` — the bit count of a byte-aligned message —
//!   overflows a 32-bit target at 512 MB, which is exactly the bound the
//!   proofs used to carry.
//!
//! The fixed-width layer is the free functions below, on `&[Bit]`. The state
//! of the permutation is `b` bits for a `b` in Table 1, at most 1600, so
//! nothing there needs to reach past a `usize` and the plain slice stays.
//!
//! Where a condition the Standard states is checked, it is a plain `assert!`,
//! which extracts to a `massert` in the generated Lean: `Trunc_s` needs
//! `s <= len(X)`, `h2b` needs `n <= 8m`.

/// A single bit.
pub type Bit = bool;

/// A bit string of `b` bits for a fixed, small `b` — the permutation state and
/// the blocks cut out of `P`. Length is carried by the vector, matching
/// `len(S)`.
pub type BitString = Vec<Bit>;

/// A bit string of arbitrary length — the Standard's `M`, `N`, `P` and `Z`.
///
/// Opaque to the extraction: the Lean model is a list of bits with no length
/// bound at all, which is the object FIPS 202 talks about. The fields here are
/// how it is realised in Rust, and are not what the proofs reason about.
#[cfg_attr(hax, hax_lib::opaque)]
#[cfg_attr(not(hax_backend_lean), derive(Debug))]
// `PartialEq`/`Eq` are for the tests only, and a derived `Clone` would be an
// impl whose body reads the representation -- which the extraction must not
// see. Both are kept away from the hax build; `Clone` is hand-written below so
// the opaque bodies can still use it.
#[cfg_attr(not(hax), derive(PartialEq, Eq))]
pub struct BitStr {
    /// Bit `i` is bit `i % 8` of `bytes[i / 8]`, least significant first — the
    /// same order App. B.1 reads a byte in.
    bytes: Vec<u8>,
    /// `len(S)`, in bits. Not `8 * bytes.len()`: the last byte is partial
    /// whenever `len % 8 != 0`.
    len: u64,
}

impl Clone for BitStr {
    #[cfg_attr(hax, hax_lib::opaque)]
    fn clone(&self) -> BitStr {
        BitStr {
            bytes: self.bytes.clone(),
            len: self.len,
        }
    }
}

impl BitStr {
    /// The empty string.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn empty() -> BitStr {
        BitStr {
            bytes: Vec::new(),
            len: 0,
        }
    }

    /// `len(S)` — Sec. 2.3.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn len(&self) -> u64 {
        self.len
    }

    /// Whether `len(S) = 0`.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// `S[i]` — Sec. 2.3, the bits of a string indexed from zero.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn bit(&self, i: u64) -> Bit {
        assert!(i < self.len, "index past the end of the string");
        let byte = self.bytes[(i / 8) as usize];
        (byte >> (i % 8)) & 1u8 == 1u8
    }

    /// Append one bit in place. Private: the operations the Standard names are
    /// the pure ones below. This is how they stay linear rather than
    /// quadratic — the type is opaque, so the body is ours to write.
    #[cfg_attr(hax, hax_lib::opaque)]
    fn push_mut(&mut self, b: Bit) {
        if self.len % 8 == 0 {
            self.bytes.push(0u8);
        }
        if b {
            let i = (self.len / 8) as usize;
            self.bytes[i] |= 1u8 << (self.len % 8);
        }
        self.len += 1;
    }

    /// `0^n` — Sec. 2.3: the string of `n` zero bits.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn zeros(n: u64) -> BitStr {
        let full = n / 8 + if n % 8 == 0 { 0 } else { 1 };
        let mut bytes: Vec<u8> = Vec::new();
        for _ in 0..full {
            bytes.push(0u8);
        }
        BitStr { bytes, len: n }
    }

    /// The string of the bits of `b`, in order.
    ///
    /// The bridge up from the fixed-width layer: a `b`-bit block of the state,
    /// or one of the suffixes of Sec. 6.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn from_bits(b: &[Bit]) -> BitStr {
        let mut out = BitStr::empty();
        for i in 0..b.len() {
            out.push_mut(b[i]);
        }
        out
    }

    /// The bits of this string, in order.
    ///
    /// The bridge back down, used where the length is known to be small — a
    /// block of `r < b` bits. Panics if the string is longer than a `usize`
    /// can count, which by construction it never is at those call sites.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn to_bits(&self) -> BitString {
        assert!(
            self.len <= usize::MAX as u64,
            "string too long to hand to the fixed-width layer"
        );
        let mut out: BitString = Vec::new();
        for i in 0..self.len {
            out.push(self.bit(i));
        }
        out
    }

    /// `X || Y` — Sec. 2.3: concatenation.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn concat(&self, y: &BitStr) -> BitStr {
        let mut out = self.clone();
        for i in 0..y.len() {
            out.push_mut(y.bit(i));
        }
        out
    }

    /// `Trunc_s(X)` — Sec. 2.3: the string of the first `s` bits of `X`.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn trunc(&self, s: u64) -> BitStr {
        assert!(s <= self.len, "Trunc_s needs s <= len(X)");
        let mut out = BitStr::empty();
        for i in 0..s {
            out.push_mut(self.bit(i));
        }
        out
    }

    /// The `n` bits of `X` from `from` — the `r`-bit blocks `P_i` of Algorithm
    /// 8, which the Standard names by slicing rather than by an operation of
    /// its own.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn slice(&self, from: u64, n: u64) -> BitStr {
        assert!(from + n <= self.len, "slice past the end of the string");
        let mut out = BitStr::empty();
        for i in 0..n {
            out.push_mut(self.bit(from + i));
        }
        out
    }

    /// Algorithm 10 at its maximum `n`: the `8m` bits of the bytes `h`.
    ///
    /// A primitive rather than `h2b(h, 8 * h.len())`, so that the bit count of
    /// a byte string is never formed as a `usize`. That multiplication is what
    /// bounded the specification to 512 MB on a 32-bit target.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn from_bytes(h: &[u8]) -> BitStr {
        BitStr {
            bytes: h.to_vec(),
            len: 8 * (h.len() as u64),
        }
    }

    /// Algorithm 11: `b2h(S)`, the bytes of `S || 0^(-len(S) mod 8)`.
    ///
    /// The packed representation already *is* that string of bytes: every bit
    /// above `len` in the final byte was pushed as a zero and never set.
    #[cfg_attr(hax, hax_lib::opaque)]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}

/// `Trunc_s(X)` for the fixed-width layer — Sec. 2.3.
pub fn trunc(x: &[Bit], s: usize) -> BitString {
    assert!(s <= x.len(), "Trunc_s needs s <= len(X)");
    let mut out: BitString = Vec::new();
    for i in 0..s {
        out.push(x[i]);
    }
    out
}

/// `0^n` for the fixed-width layer — Sec. 2.3.
pub fn zeros(n: usize) -> BitString {
    let mut out: BitString = Vec::new();
    for _ in 0..n {
        out.push(false);
    }
    out
}

/// `X || Y` for the fixed-width layer — Sec. 2.3.
pub fn concat(x: &[Bit], y: &[Bit]) -> BitString {
    let mut out: BitString = Vec::new();
    for i in 0..x.len() {
        out.push(x[i]);
    }
    for i in 0..y.len() {
        out.push(y[i]);
    }
    out
}

/// `X ⊕ Y` for two strings of the same length — the bitwise XOR that Step 6 of
/// Algorithm 8 applies to the state.
pub fn xor(x: &[Bit], y: &[Bit]) -> BitString {
    assert!(
        x.len() == y.len(),
        "the xor of Sec. 2.3 takes two strings of the same length"
    );
    let mut out: BitString = Vec::new();
    for i in 0..x.len() {
        out.push(x[i] ^ y[i]);
    }
    out
}

/// Algorithm 10: `h2b(H, n)` — hexadecimal string to bit string.
///
/// This takes the bytes directly (`H` parsed as in Step 2a) rather than a
/// string of hexadecimal digits.
pub fn h2b(h: &[u8], n: u64) -> BitStr {
    assert!(n <= 8 * (h.len() as u64), "Algorithm 10 requires n <= 8m");
    BitStr::from_bytes(h).trunc(n)
}

/// `h2b(H)` with `n` at its maximum, `8m` — Appendix B.1.
pub fn h2b_full(h: &[u8]) -> BitStr {
    BitStr::from_bytes(h)
}

/// Algorithm 11: `b2h(S)` — bit string to hexadecimal string.
///
/// `T = S || 0^(-n mod 8)`, then `h_i = Σ_j b_ij · 2^j` over `b_ij = T[8i + j]`.
/// Returned as the bytes `h_0 … h_(m-1)`.
pub fn b2h(s: &BitStr) -> Vec<u8> {
    s.to_bytes()
}

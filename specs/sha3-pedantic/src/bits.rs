//! Bit strings, and the conversions between them and hexadecimal/byte
//! strings — FIPS 202, Sec. 2.3 and Appendix B.1.
//!
//! The Standard works on *bit strings* throughout. A bit string of length
//! `n` is indexed `S[0] … S[n-1]`, and `S = S[0] || S[1] || … || S[n-1]`.
//!
//! The helpers here are written with `push` in explicit loops rather than with
//! `Vec::extend`/`to_vec`, which the Lean core model does not cover. The
//! "Input:" conditions of the Algorithms are plain `assert!`s, which extract to
//! `massert` in the generated Lean.

/// A single bit. FIPS 202 writes bits as `0`/`1` and combines them with
/// `⊕` (XOR) and `·` (AND, "integer multiplication" in Sec. 3.2.4).
pub type Bit = bool;

/// A bit string. Length is carried by the vector, matching `len(S)`.
pub type BitString = Vec<Bit>;

/// `Trunc_s(X)` — Sec. 2.3: the string of the first `s` bits of `X`.
pub fn trunc(x: &[Bit], s: usize) -> BitString {
    assert!(s <= x.len(), "Trunc_s needs s <= len(X)");
    let mut out: BitString = Vec::new();
    for i in 0..s {
        out.push(x[i]);
    }
    out
}

/// `0^n` — Sec. 2.3: the string of `n` zero bits.
pub fn zeros(n: usize) -> BitString {
    let mut out: BitString = Vec::new();
    for _ in 0..n {
        out.push(false);
    }
    out
}

/// `X || Y` — Sec. 2.3: concatenation.
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
/// Algorithm 8 applies to the state and a padded block.
pub fn xor(x: &[Bit], y: &[Bit]) -> BitString {
    assert!(
        x.len() == y.len(),
        "XOR is pointwise, so the lengths must agree"
    );
    let mut out: BitString = Vec::new();
    for i in 0..x.len() {
        out.push(x[i] ^ y[i]);
    }
    out
}

/// Algorithm 10: `h2b(H, n)` — hexadecimal string to bit string.
///
/// Byte `h_i` becomes the eight bits `b_i0 … b_i7` with
/// `h_i = Σ_j b_ij · 2^j`, laid down as `T[8i + j] = b_ij`; the result is
/// `Trunc_n(T)`. So within a byte the *least* significant bit comes first.
///
/// This takes the bytes directly (`H` parsed as in Step 2a) rather than a
/// string of hexadecimal digits.
pub fn h2b(h: &[u8], n: usize) -> BitString {
    assert!(n <= 8 * h.len(), "Algorithm 10 requires n <= 8m");
    let mut t: BitString = Vec::new();
    for i in 0..h.len() {
        let byte = h[i];
        for j in 0..8 {
            t.push((byte >> j) & 1u8 == 1u8);
        }
    }
    trunc(&t, n)
}

/// `h2b(H)` with `n` at its maximum, `8m` — Appendix B.1.
pub fn h2b_full(h: &[u8]) -> BitString {
    h2b(h, 8 * h.len())
}

/// Algorithm 11: `b2h(S)` — bit string to hexadecimal string.
///
/// `T = S || 0^(-n mod 8)`, then `h_i = Σ_j b_ij · 2^j` over `b_ij = T[8i + j]`.
/// Returned as the bytes `h_0 … h_(m-1)`.
pub fn b2h(s: &[Bit]) -> Vec<u8> {
    let n = s.len();
    let t = concat(s, &zeros((8 - n % 8) % 8));
    let m = t.len() / 8;
    let mut h: Vec<u8> = Vec::new();
    for i in 0..m {
        let mut byte = 0u8;
        for j in 0..8 {
            if t[8 * i + j] {
                byte |= 1u8 << j;
            }
        }
        h.push(byte);
    }
    h
}

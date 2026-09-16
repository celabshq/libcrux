//! The sponge construction, multi-rate padding and `KECCAK[c]` —
//! FIPS 202, Sec. 4, 5.1 and 5.2.

use crate::bits::{concat, trunc, xor, zeros, Bit, BitString};
use crate::keccak_p::keccak_p;

/// Algorithm 9: `pad10*1(x, m)`.
///
/// Returns `P = 1 || 0^j || 1` where `j = (-m - 2) mod x`, so that `m + len(P)`
/// is a positive multiple of `x`.
pub fn pad10_star_1(x: usize, m: usize) -> BitString {
    let x_i = x as i64;
    let j = (((-(m as i64) - 2) % x_i) + x_i) % x_i;
    let mut p: BitString = Vec::new();
    p.push(true);
    for _ in 0..j {
        p.push(false);
    }
    p.push(true);
    p
}

/// The components of `SPONGE[f, pad, r]` — Sec. 4.
///
/// The construction is parameterised by "an underlying function on
/// fixed-length strings, denoted by `f`" and "a padding rule, denoted by
/// `pad`"; the width `b` "is determined by the choice of `f`". A trait carries
/// exactly those three, and instantiating the sponge means implementing it.
pub trait Components {
    /// `f`, mapping `b`-bit strings to `b`-bit strings.
    fn f(&self, s: &[Bit]) -> BitString;

    /// `pad(x, m)`, the padding rule.
    fn pad(&self, x: usize, m: usize) -> BitString;
}

/// Algorithm 8: `SPONGE[f, pad, r](N, d)`.
///
/// `components` carries the `f` and `pad` of `SPONGE[f, pad, r]`. It is a value rather
/// than only a type parameter on purpose: a trait whose type parameter is
/// fixed by the turbofish alone loses its instance argument at every call site
/// the extraction lifts out of this function, loops included.
pub fn sponge<C: Components>(components: &C, b: usize, r: usize, n: &[Bit], d: usize) -> BitString {
    // 1. Let P = N || pad(r, len(N)).
    let p = concat(n, &components.pad(r, n.len()));
    // 2. Let n = len(P)/r.
    let blocks = p.len() / r;
    // 3. Let c = b - r.
    let c = b - r;
    // 4. Let P_0, … , P_{n-1} be the r-bit blocks of P.
    // 5. Let S = 0^b.
    let mut s = zeros(b);
    // 6. For i from 0 to n-1, let S = f(S ⊕ (P_i || 0^c)).
    for i in 0..blocks {
        let block = concat(&p[i * r..(i + 1) * r], &zeros(c));
        s = components.f(&xor(&s, &block));
    }
    // 7. Let Z be the empty string.
    let mut z: BitString = Vec::new();
    loop {
        // 8. Let Z = Z || Trunc_r(S).
        let head = trunc(&s, r);
        z = concat(&z, &head);
        // 9. If d ≤ |Z|, then return Trunc_d(Z); else continue.
        if d <= z.len() {
            return trunc(&z, d);
        }
        // 10. Let S = f(S), and continue with Step 8.
        s = components.f(&s);
    }
}

/// `b = 1600` for the KECCAK functions — Sec. 5.2.
pub const B: usize = 1600;

/// Sec. 5.2: `KECCAK[c]` is the sponge with `KECCAK-p[1600, 24]` as `f` and
/// `pad10*1` as the padding rule.
pub struct Keccak1600;

impl Components for Keccak1600 {
    fn f(&self, s: &[Bit]) -> BitString {
        keccak_p::<64>(s, 24)
    }

    fn pad(&self, x: usize, m: usize) -> BitString {
        pad10_star_1(x, m)
    }
}

/// Sec. 5.2: `KECCAK[c](N, d) = SPONGE[KECCAK-p[1600, 24], pad10*1, 1600-c](N, d)`.
pub fn keccak_c(c: usize, n: &[Bit], d: usize) -> BitString {
    sponge(&Keccak1600, B, B - c, n, d)
}

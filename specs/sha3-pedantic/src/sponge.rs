//! The sponge construction, multi-rate padding and `KECCAK[c]` —
//! FIPS 202, Sec. 4, 5.1 and 5.2.

use crate::bits::{concat, trunc, xor, zeros, Bit, BitString};
use crate::keccak_p::keccak_p;

/// Algorithm 9: `pad10*1(x, m)`.
///
/// Returns `P = 1 || 0^j || 1` where `j = (-m - 2) mod x`, so that `m + len(P)`
/// is a positive multiple of `x`.
pub fn pad10_star_1(x: usize, m: usize) -> BitString {
    assert!(x > 0, "Algorithm 9 takes a positive x");
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

/// The function and the padding rule of `SPONGE[f, pad, r]` — Sec. 4.
///
/// Of the construction's three components, "an underlying function on
/// fixed-length strings, denoted by `f`", "a padding rule, denoted by `pad`"
/// and the rate `r`, this trait carries the first two; `r` is the third
/// parameter of [`Sponge`]. The width `b` is not a component of its own: Sec. 4
/// says it "is determined by the choice of `f`", so it comes along with `f` as
/// an associated constant.
pub trait Components {
    /// `b`, the width of `f` — Sec. 4: "The width `b` is determined by the
    /// choice of `f`."
    const B: usize;

    /// `f`, mapping `b`-bit strings to `b`-bit strings.
    fn f(&self, s: &[Bit]) -> BitString;

    /// `pad(x, m)`, the padding rule.
    fn pad(&self, x: usize, m: usize) -> BitString;
}

/// `SPONGE[f, pad, r]` — the sponge function that Sec. 4 builds from its three
/// components. Applying it to `(N, d)` is [`Sponge::apply`], Algorithm 8.
///
/// The components travel as a value rather than only as a type parameter on
/// purpose: a trait whose type parameter is fixed by the turbofish alone loses
/// its instance argument at every call site the extraction lifts out of a
/// function, loops included. They are held by value, not by reference: a
/// `&'a C` field read inside a loop is an internal error in aeneas (a nested
/// borrow). Both are concessions to the toolchain; the width alone would be an
/// associated constant either way.
pub struct Sponge<C: Components> {
    /// `f` and `pad`, and with them `b`.
    components: C,
    /// `r`, the rate.
    r: usize,
}

impl<C: Components> Sponge<C> {
    /// `SPONGE[f, pad, r]`: fix the three components.
    pub fn new(components: C, r: usize) -> Self {
        // Sec. 4: "The rate r is a positive integer that is strictly less than
        // the width b."
        assert!(r > 0 && r < C::B, "Sec. 4 takes 0 < r < b");
        Sponge { components, r }
    }

    /// Algorithm 8: `SPONGE[f, pad, r](N, d)`.
    pub fn apply(&self, n: &[Bit], d: usize) -> BitString {
        let components = &self.components;
        let r = self.r;
        // 1. Let P = N || pad(r, len(N)).
        let p = concat(n, &components.pad(r, n.len()));
        // 2. Let n = len(P)/r.
        let blocks = p.len() / r;
        // 3. Let c = b - r.
        let c = C::B - r;
        // 4. Let P_0, … , P_{n-1} be the r-bit blocks of P.
        // 5. Let S = 0^b.
        let mut s = zeros(C::B);
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
}

/// `b = 1600`, the width that `KECCAK[c]` restricts the KECCAK family to —
/// Sec. 5.2.
///
/// The family itself is defined for every `r + c` in Table 1; `KECCAK[c]` is
/// the case `b = 1600`, in which `r` is determined by the choice of `c`.
pub const B: usize = 1600;

/// Sec. 5.2: `KECCAK[c]` is the sponge with `KECCAK-p[1600, 24]` as `f` and
/// `pad10*1` as the padding rule.
pub struct Keccak1600;

impl Components for Keccak1600 {
    const B: usize = B;

    fn f(&self, s: &[Bit]) -> BitString {
        keccak_p::<64>(s, 24)
    }

    fn pad(&self, x: usize, m: usize) -> BitString {
        pad10_star_1(x, m)
    }
}

/// Sec. 5.2: `KECCAK[c](N, d) = SPONGE[KECCAK-p[1600, 24], pad10*1, 1600-c](N, d)`.
pub fn keccak_c(c: usize, n: &[Bit], d: usize) -> BitString {
    Sponge::new(Keccak1600, B - c).apply(n, d)
}

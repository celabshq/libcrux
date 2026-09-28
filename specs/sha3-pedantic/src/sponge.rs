//! The sponge construction, multi-rate padding and `KECCAK[c]` —
//! FIPS 202, Sec. 4, 5.1 and 5.2.

use crate::bits::{concat, trunc, xor, zeros, Bit, BitStr, BitString};
use crate::keccak_p::keccak_p;
use crate::nat::Nat;

/// Algorithm 9: `pad10*1(x, m)`.
///
/// Returns `P = 1 || 0^j || 1` where `j = (-m - 2) mod x`, so that `m + len(P)`
/// is a positive multiple of `x`.
pub fn pad10_star_1(x: Nat, m: Nat) -> BitStr {
    assert!(x > Nat::new(0), "Algorithm 9 takes a positive x");
    // `j = (-m - 2) mod x`. Written so that it never leaves the nonnegative
    // integers: `-m - 2 ≡ x - ((m + 2) mod x) (mod x)`, and reducing `m` first
    // keeps the sum small. The Standard's form would need a signed type, which
    // a length here is not — the subtraction below is the one place this crate
    // relies on `Nat`'s subtraction, and `(m % x + 2) % x < x` is what makes it
    // total.
    let j = (x - ((m % x + Nat::new(2)) % x)) % x;
    let one = BitStr::from_bits(&[true]);
    one.concat(&BitStr::zeros(j)).concat(&one)
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
    fn pad(&self, x: Nat, m: Nat) -> BitStr;
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
    r: Nat,
}

impl<C: Components> Sponge<C> {
    /// `SPONGE[f, pad, r]`: fix the three components.
    pub fn new(components: C, r: Nat) -> Self {
        // Sec. 4: "The rate r is a positive integer that is strictly less than
        // the width b."
        assert!(
            r > Nat::new(0) && r < Nat::from_usize(C::B),
            "Sec. 4 takes 0 < r < b"
        );
        Sponge { components, r }
    }

    /// Algorithm 8: `SPONGE[f, pad, r](N, d)`.
    pub fn apply(&self, n: &BitStr, d: Nat) -> BitStr {
        let components = &self.components;
        let r = self.r;
        // 1. Let P = N || pad(r, len(N)).
        let p = n.concat(&components.pad(r, n.len()));
        // 2. Let n = len(P)/r.
        let blocks = p.len() / r;
        // 3. Let c = b - r.
        //    `r < b ≤ 1600`, so the rate goes back down to a `usize` here
        //    without question; it is a `Nat` on the bit-string side because
        //    the lengths it is compared against are.
        let c = C::B - r.to_usize();
        // 4. Let P_0, … , P_{n-1} be the r-bit blocks of P.
        // 5. Let S = 0^b.
        let mut s = zeros(C::B);
        // 6. For i from 0 to n-1, let S = f(S ⊕ (P_i || 0^c)).
        let mut i = Nat::new(0);
        while i < blocks {
            let block = concat(&p.slice(i * r, r).to_bits(), &zeros(c));
            s = components.f(&xor(&s, &block));
            i = i + Nat::new(1);
        }
        // 7. Let Z be the empty string.
        let mut z = BitStr::empty();
        loop {
            // 8. Let Z = Z || Trunc_r(S).
            let head = trunc(&s, r.to_usize());
            z = z.concat(&BitStr::from_bits(&head));
            // 9. If d ≤ |Z|, then return Trunc_d(Z); else continue.
            if d <= z.len() {
                return z.trunc(d);
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

    fn pad(&self, x: Nat, m: Nat) -> BitStr {
        pad10_star_1(x, m)
    }
}

/// Sec. 5.2: `KECCAK[c](N, d) = SPONGE[KECCAK-p[1600, 24], pad10*1, 1600-c](N, d)`.
pub fn keccak_c(c: usize, n: &BitStr, d: Nat) -> BitStr {
    Sponge::new(Keccak1600, Nat::from_usize(B - c)).apply(n, d)
}

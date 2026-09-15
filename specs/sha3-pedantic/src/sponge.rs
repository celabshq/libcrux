//! The sponge construction, multi-rate padding and `KECCAK[c]` —
//! FIPS 202, Sec. 4, 5.1 and 5.2.

use crate::bits::{concat, trunc, zeros, Bit, BitString};
use crate::keccak_p::keccak_p;

/// Algorithm 9: `pad10*1(x, m)`.
///
/// Returns `P = 1 || 0^j || 1` where `j = (-m - 2) mod x`, so that `m + len(P)`
/// is a positive multiple of `x`.
pub fn pad10_star_1(x: usize, m: usize) -> BitString {
    let x_i = x as i64;
    let j = (((-(m as i64) - 2) % x_i) + x_i) % x_i;
    let mut p = vec![true];
    p.extend(zeros(j as usize));
    p.push(true);
    p
}

/// Algorithm 8: `SPONGE[f, pad, r](N, d)`.
///
/// `f` maps `b`-bit strings to `b`-bit strings, `r < b` is the rate, and `pad`
/// is the padding rule. Kept generic in `f` and `pad`, as the Standard is.
pub fn sponge<F, P>(f: F, pad: P, r: usize, b: usize, n: &[Bit], d: usize) -> BitString
where
    F: Fn(&[Bit]) -> BitString,
    P: Fn(usize, usize) -> BitString,
{
    // 1. Let P = N || pad(r, len(N)).
    let p = concat(n, &pad(r, n.len()));
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
        let xored: BitString = s.iter().zip(block.iter()).map(|(a, b)| a ^ b).collect();
        s = f(&xored);
    }
    // 7. Let Z be the empty string.
    let mut z: BitString = Vec::new();
    loop {
        // 8. Let Z = Z || Trunc_r(S).
        z.extend(trunc(&s, r));
        // 9. If d ≤ |Z|, then return Trunc_d(Z); else continue.
        if d <= z.len() {
            return trunc(&z, d);
        }
        // 10. Let S = f(S), and continue with Step 8.
        s = f(&s);
    }
}

/// Sec. 5.2: `KECCAK[c](N, d) = SPONGE[KECCAK-p[1600, 24], pad10*1, 1600-c](N, d)`.
pub fn keccak_c(c: usize, n: &[Bit], d: usize) -> BitString {
    const B: usize = 1600;
    sponge(|s| keccak_p::<64>(s, 24), pad10_star_1, B - c, B, n, d)
}

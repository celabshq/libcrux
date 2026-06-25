/// Arithmetic helper functions — FIPS 204, Section 7.4.
///
/// Operations on elements of Z_q where q = 8380417.
use crate::parameters::{D, Q};

/// Reduce a to [0, q-1].
#[inline]
pub(crate) fn mod_q(a: i64) -> i32 {
    let r = (a % Q as i64) as i32;
    if r < 0 {
        r + Q
    } else {
        r
    }
}

/// Centered modular reduction: returns r in [-(m/2), m/2).
#[inline]
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(m > 0)]
pub(crate) fn mod_pm(a: i32, m: i32) -> i32 {
    let a64 = a as i64;
    let m64 = m as i64;
    let r = ((a64 % m64) + m64) % m64;
    let r = r as i32;
    if r > m / 2 {
        r - m
    } else {
        r
    }
}

/// Power2Round(r) — FIPS 204, Algorithm 35.
///
/// Decomposes r into (r1, r0) such that r ≡ r1·2^d + r0 (mod q).
#[hax_lib::requires(r >= 0 && r < Q)]
pub(crate) fn power2round(r: i32) -> (i32, i32) {
    let r_plus = r % Q;
    let r_plus = if r_plus < 0 { r_plus + Q } else { r_plus };
    let two_d = 1i32 << D;
    let r0 = mod_pm(r_plus, two_d);
    let r1 = (r_plus - r0) / two_d;
    (r1, r0)
}

/// Decompose(r) — FIPS 204, Algorithm 36.
///
/// Decomposes r into (r1, r0) such that r ≡ r1·(2·gamma2) + r0 (mod q).
/// Special case: when the straightforward rounding would give r1 = (q-1)/(2·gamma2),
/// returns (0, r0 - 1) instead.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(r >= 0 && r < Q && gamma2 > 0 && gamma2 < Q / 2)]
pub(crate) fn decompose(r: i32, gamma2: i32) -> (i32, i32) {
    let r_plus = r % Q;
    let r_plus = if r_plus < 0 { r_plus + Q } else { r_plus };
    let alpha = 2 * gamma2;
    let r0 = mod_pm(r_plus, alpha);
    if r_plus - r0 == Q - 1 {
        (0, r0 - 1)
    } else {
        let r1 = (r_plus - r0) / alpha;
        (r1, r0)
    }
}

/// HighBits(r) — FIPS 204, Algorithm 37.
///
/// Returns r1 from Decompose(r).
#[hax_lib::requires(r >= 0 && r < Q && gamma2 > 0 && gamma2 < Q / 2)]
pub(crate) fn high_bits(r: i32, gamma2: i32) -> i32 {
    decompose(r, gamma2).0
}

/// LowBits(r) — FIPS 204, Algorithm 38.
///
/// Returns r0 from Decompose(r).
#[hax_lib::requires(r >= 0 && r < Q && gamma2 > 0 && gamma2 < Q / 2)]
pub(crate) fn low_bits(r: i32, gamma2: i32) -> i32 {
    decompose(r, gamma2).1
}

/// MakeHint(z, r) — FIPS 204, Algorithm 39.
///
/// Returns true if adding z to r changes the high bits.
#[hax_lib::requires(r >= 0 && r < Q && gamma2 > 0 && gamma2 < Q / 2)]
pub(crate) fn make_hint(z: i32, r: i32, gamma2: i32) -> bool {
    let r1 = high_bits(r, gamma2);
    let v1 = high_bits(mod_q(r as i64 + z as i64), gamma2);
    r1 != v1
}

/// UseHint(h, r) — FIPS 204, Algorithm 40.
///
/// Returns the high bits of r, adjusted according to hint h.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(r >= 0 && r < Q && gamma2 > 0 && gamma2 < Q / 2)]
pub(crate) fn use_hint(hint: bool, r: i32, gamma2: i32) -> i32 {
    let m = (Q - 1) / (2 * gamma2);
    let (r1, r0) = decompose(r, gamma2);
    if hint && r0 > 0 {
        (r1 + 1) % m
    } else if hint && r0 <= 0 {
        ((r1 - 1) % m + m) % m
    } else {
        r1
    }
}

/// Infinity norm of a single coefficient (absolute value, centered).
#[inline]
#[hax_lib::fstar::options("--z3rlimit 150")]
pub(crate) fn coeff_norm(a: i32) -> i32 {
    let a_mod = (((a as i64 % Q as i64) + Q as i64) % Q as i64) as i32;
    if a_mod > Q / 2 {
        Q - a_mod
    } else {
        a_mod
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power2round_basic() {
        let (r1, r0) = power2round(0);
        assert_eq!((r1, r0), (0, 0));
        let (r1, r0) = power2round(8192); // 2^13
        assert_eq!(r1 * (1 << D) + r0, 8192);
    }

    #[test]
    fn decompose_basic() {
        let gamma2 = (Q - 1) / 88; // 95232
        let (r1, r0) = decompose(0, gamma2);
        assert_eq!(r1 * 2 * gamma2 + r0, 0);
    }

    #[test]
    fn high_low_bits_roundtrip() {
        let gamma2 = (Q - 1) / 32;
        for r in [0, 1, 100, Q / 2, Q - 1] {
            let r1 = high_bits(r, gamma2);
            let r0 = low_bits(r, gamma2);
            let reconstructed =
                ((r1 as i64 * 2 * gamma2 as i64 + r0 as i64) % Q as i64 + Q as i64) % Q as i64;
            assert_eq!(reconstructed as i32, r, "failed for r={r}");
        }
    }

    #[test]
    fn use_hint_no_hint() {
        let gamma2 = (Q - 1) / 32;
        for r in [0, 1, 100, Q / 2, Q - 1] {
            assert_eq!(use_hint(false, r, gamma2), high_bits(r, gamma2));
        }
    }
}

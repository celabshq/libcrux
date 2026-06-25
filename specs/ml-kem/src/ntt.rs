use crate::parameters::*;

const ZETA: FieldElement = FieldElement::new(17);

/// Montgomery constant R = 2^16 mod q.
/// In the implementation, coefficients are stored in Montgomery form (a * R mod q).
/// In the spec, we use plain modular arithmetic, so R is conceptually 1.
/// This constant documents the correspondence.
#[allow(dead_code)]
pub const MONTGOMERY_R: i32 = 1; // Identity in the spec

/// In the implementation, zetas are pre-multiplied by Montgomery R.
/// In the spec, ZETAS are plain values, so ZETAS_TIMES_MONTGOMERY_R == ZETAS.
/// This alias documents the correspondence with the implementation's `ZETAS_TIMES_MONTGOMERY_R`.
#[allow(dead_code)]
pub const ZETAS_TIMES_MONTGOMERY_R: [FieldElement; 128] = ZETAS;

/// Montgomery domain conversion: identity in the spec.
///
/// In the implementation, `to_standard_domain(a)` converts from Montgomery form
/// by computing `a * MONTGOMERY_R_INV mod q`. Since the spec uses plain arithmetic
/// (effectively MONTGOMERY_R = 1), this is an identity operation.
///
/// Documenting this correspondence enables function-by-function verification by
/// showing that the implementation's Montgomery conversions compose to identity.
#[allow(dead_code)]
pub fn to_standard_domain(a: FieldElement) -> FieldElement {
    a
}

/// Montgomery multiplication: identity wrapper in the spec.
///
/// In the implementation, `montgomery_multiply_by_constant(a, c)` computes
/// `a * c * R^{-1} mod q`. In the spec, this simplifies to `a * c mod q` since R = 1.
#[allow(dead_code)]
pub fn montgomery_multiply_by_constant(a: FieldElement, c: FieldElement) -> FieldElement {
    FieldElement::new(((a.val as u32 * c.val as u32) % FIELD_MODULUS as u32) as u16)
}

/// Convert a field element to its unsigned representative in [0, q).
/// Corresponds to `to_unsigned_field_modulus` / `Vector::to_unsigned_representative`
/// in the implementation.
///
/// In the spec, field elements are already non-negative after reduction, so this
/// is a plain modular reduction.
pub fn to_unsigned_field_modulus(a: FieldElement) -> FieldElement {
    FieldElement::new(a.val % FIELD_MODULUS) // already unsigned, just reduce
}

fn bit_rev_7(x: usize) -> usize {
    let mut result = 0;
    for i in 0..7 {
        if (x >> i) & 1 == 1 {
            result |= 1 << (6 - i);
        }
    }
    result
}

/// Use the Cooley–Tukey butterfly to compute an in-place NTT representation
/// of a `Polynomial`.
///
/// Given a `Polynomial` `f`, the NTT representation `f^` is:
///
/// ```plaintext
/// f^ := (f mod(X² - ζ^(2*BitRev₇(0) + 1), ..., f mod (X² − ζ^(2·BitRev₇(127) + 1))
/// ```
///
/// This function implements <strong>Algorithm 8</strong> of the NIST FIPS 203 standard, which
/// is reproduced below:
///
/// ```plaintext
/// Input: array f ∈ ℤ₂₅₆.
/// Output: array fˆ ∈ ℤ₂₅₆.
///
/// fˆ ← f
/// k ← 1
/// for (len ← 128; len ≥ 2; len ← len/2)
///     for (start ← 0; start < 256; start ← start + 2·len)
///         zeta ← ζ^(BitRev₇(k)) mod q
///         k ← k + 1
///         for (j ← start; j < start + len; j++)
///             t ← zeta·fˆ[j+len]
///             fˆ[j+len] ← fˆ[j] − t
///             fˆ[j] ← fˆ[j] + t
///         end for
///     end for
/// end for
/// return fˆ
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
pub const ZETAS: [FieldElement; 128] = [
    FieldElement::new(1),
    FieldElement::new(1729),
    FieldElement::new(2580),
    FieldElement::new(3289),
    FieldElement::new(2642),
    FieldElement::new(630),
    FieldElement::new(1897),
    FieldElement::new(848),
    FieldElement::new(1062),
    FieldElement::new(1919),
    FieldElement::new(193),
    FieldElement::new(797),
    FieldElement::new(2786),
    FieldElement::new(3260),
    FieldElement::new(569),
    FieldElement::new(1746),
    FieldElement::new(296),
    FieldElement::new(2447),
    FieldElement::new(1339),
    FieldElement::new(1476),
    FieldElement::new(3046),
    FieldElement::new(56),
    FieldElement::new(2240),
    FieldElement::new(1333),
    FieldElement::new(1426),
    FieldElement::new(2094),
    FieldElement::new(535),
    FieldElement::new(2882),
    FieldElement::new(2393),
    FieldElement::new(2879),
    FieldElement::new(1974),
    FieldElement::new(821),
    FieldElement::new(289),
    FieldElement::new(331),
    FieldElement::new(3253),
    FieldElement::new(1756),
    FieldElement::new(1197),
    FieldElement::new(2304),
    FieldElement::new(2277),
    FieldElement::new(2055),
    FieldElement::new(650),
    FieldElement::new(1977),
    FieldElement::new(2513),
    FieldElement::new(632),
    FieldElement::new(2865),
    FieldElement::new(33),
    FieldElement::new(1320),
    FieldElement::new(1915),
    FieldElement::new(2319),
    FieldElement::new(1435),
    FieldElement::new(807),
    FieldElement::new(452),
    FieldElement::new(1438),
    FieldElement::new(2868),
    FieldElement::new(1534),
    FieldElement::new(2402),
    FieldElement::new(2647),
    FieldElement::new(2617),
    FieldElement::new(1481),
    FieldElement::new(648),
    FieldElement::new(2474),
    FieldElement::new(3110),
    FieldElement::new(1227),
    FieldElement::new(910),
    FieldElement::new(17),
    FieldElement::new(2761),
    FieldElement::new(583),
    FieldElement::new(2649),
    FieldElement::new(1637),
    FieldElement::new(723),
    FieldElement::new(2288),
    FieldElement::new(1100),
    FieldElement::new(1409),
    FieldElement::new(2662),
    FieldElement::new(3281),
    FieldElement::new(233),
    FieldElement::new(756),
    FieldElement::new(2156),
    FieldElement::new(3015),
    FieldElement::new(3050),
    FieldElement::new(1703),
    FieldElement::new(1651),
    FieldElement::new(2789),
    FieldElement::new(1789),
    FieldElement::new(1847),
    FieldElement::new(952),
    FieldElement::new(1461),
    FieldElement::new(2687),
    FieldElement::new(939),
    FieldElement::new(2308),
    FieldElement::new(2437),
    FieldElement::new(2388),
    FieldElement::new(733),
    FieldElement::new(2337),
    FieldElement::new(268),
    FieldElement::new(641),
    FieldElement::new(1584),
    FieldElement::new(2298),
    FieldElement::new(2037),
    FieldElement::new(3220),
    FieldElement::new(375),
    FieldElement::new(2549),
    FieldElement::new(2090),
    FieldElement::new(1645),
    FieldElement::new(1063),
    FieldElement::new(319),
    FieldElement::new(2773),
    FieldElement::new(757),
    FieldElement::new(2099),
    FieldElement::new(561),
    FieldElement::new(2466),
    FieldElement::new(2594),
    FieldElement::new(2804),
    FieldElement::new(1092),
    FieldElement::new(403),
    FieldElement::new(1026),
    FieldElement::new(1143),
    FieldElement::new(2150),
    FieldElement::new(2775),
    FieldElement::new(886),
    FieldElement::new(1722),
    FieldElement::new(1212),
    FieldElement::new(1874),
    FieldElement::new(1029),
    FieldElement::new(2110),
    FieldElement::new(2935),
    FieldElement::new(885),
    FieldElement::new(2154),
];

#[hax_lib::requires(i < 128)]
pub fn get_zeta(i: usize) -> FieldElement {
    ZETAS[i]
}

/// Cooley–Tukey butterfly: `(a, b, ζ) ↦ (a + ζ·b, a − ζ·b)`.
/// Used in the forward NTT.
pub fn butterfly(
    zeta: FieldElement,
    a: FieldElement,
    b: FieldElement,
) -> (FieldElement, FieldElement) {
    let t = zeta.mul(b);
    (a.add(t), a.sub(t))
}

/// One layer of the NTT, generic over the array length `N`.
///
/// The layer is characterised by `(len, groups)` where `groups = zetas.len()`
/// and `N = 2·groups·len`.  Each of the `groups` butterfly groups spans `2·len`
/// consecutive coefficients, uses one zeta, and runs `len` independent
/// butterflies.
///
/// This is FIPS 203 Algorithm 8 lines 3-8 applied once at butterfly half-size
/// `len`.  The within-chunk case (N = 16, len ∈ {2, 4, 8}) corresponds to the
/// trait's `ntt_layer_{1,2,3}_step`; the full-polynomial case (N = 256,
/// len = 2^layer) is what `ntt_layer` below instantiates.
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(
    len >= 1 && len < 1024 && zetas.len() < 1024 && zetas.len() * 2 * len == N
)]
pub fn ntt_layer_n<const N: usize>(
    p: [FieldElement; N],
    len: usize,
    zetas: &[FieldElement],
) -> [FieldElement; N] {
    createi(|i| {
        let group = i / (2 * len);
        let idx = i % (2 * len);
        if idx < len {
            butterfly(zetas[group], p[i], p[i + len]).0
        } else {
            butterfly(zetas[group], p[i - len], p[i]).1
        }
    })
}

/// One layer of the 256-coefficient NTT.  Thin wrapper over `ntt_layer_n`
/// that selects the zeta slice for this layer out of the global `ZETAS`
/// table.
///
/// Follows FIPS 203 Algorithm 8.  Butterfly half-size `len = 2^layer`,
/// groups = `128 / len`, zetas used = `ZETAS[groups .. 2·groups]`.
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(layer >= 1 && layer <= 7)]
fn ntt_layer(p: Polynomial, layer: usize) -> Polynomial {
    let len = 1 << layer;
    let groups = 128 / len;
    ntt_layer_n(p, len, &ZETAS[groups..2 * groups])
}

fn ntt(p: Polynomial) -> Polynomial {
    let p = ntt_layer(p, 7);
    let p = ntt_layer(p, 6);
    let p = ntt_layer(p, 5);
    let p = ntt_layer(p, 4);
    let p = ntt_layer(p, 3);
    let p = ntt_layer(p, 2);
    let p = ntt_layer(p, 1);
    p
}

/// Compute the product of two `KyberBinomial`s with respect to the
/// modulus `X² - zeta`.
///
/// This function implements <strong>Algorithm 11</strong> of the NIST FIPS 203 standard, which
/// is reproduced below:
///
/// ```plaintext
/// Input:  a₀, a₁, b₀, b₁ ∈ ℤq.
/// Input: γ ∈ ℤq.
/// Output: c₀, c₁ ∈ ℤq.
///
/// c₀ ← a₀·b₀ + a₁·b₁·γ
/// c₁ ← a₀·b₁ + a₁·b₀
/// return c₀, c₁
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
#[hax_lib::fstar::options("--z3rlimit 150")]
fn base_case_multiply_even(
    a0: FieldElement,
    a1: FieldElement,
    b0: FieldElement,
    b1: FieldElement,
    zeta: FieldElement,
) -> FieldElement {
    // c₀ = a₀·b₀ + a₁·b₁·ζ
    a0.mul(b0).add(a1.mul(b1).mul(zeta))
}

fn base_case_multiply_odd(
    a0: FieldElement,
    a1: FieldElement,
    b0: FieldElement,
    b1: FieldElement,
) -> FieldElement {
    // c₁ = a₀·b₁ + a₁·b₀
    a0.mul(b1).add(a1.mul(b0))
}

/// Given two `Polynomial`s in their NTT representations,
/// compute their product. Given two polynomials in the NTT domain `f^` and `ĵ`,
/// the `iᵗʰ` coefficient of the product `k̂` is determined by the calculation:
///
/// ```plaintext
/// ĥ[2·i] + ĥ[2·i + 1]X = (f^[2·i] + f^[2·i + 1]X)·(ĝ[2·i] + ĝ[2·i + 1]X) mod (X² - ζ^(2·BitRev₇(i) + 1))
/// ```
///
/// This function implements <strong>Algorithm 10</strong> of the NIST FIPS 203 standard, which
/// is reproduced below:
///
/// ```plaintext
/// Input: Two arrays fˆ ∈ ℤ₂₅₆ and ĝ ∈ ℤ₂₅₆.
/// Output: An array ĥ ∈ ℤq.
///
/// for(i ← 0; i < 128; i++)
///     (ĥ[2i], ĥ[2i+1]) ← BaseCaseMultiply(fˆ[2i], fˆ[2i+1], ĝ[2i], ĝ[2i+1], ζ^(2·BitRev₇(i) + 1))
/// end for
/// return ĥ
/// ```
///
/// The NIST FIPS 203 standard can be found at
/// <https://csrc.nist.gov/pubs/fips/203/ipd>.
/// Pointwise polynomial multiplication in the NTT domain, generic over
/// the array length `N`.
///
/// The input is two NTT-domain arrays of `N` coefficients and an array of
/// `N/4` zetas.  Consecutive 4-coefficient groups are treated as two
/// quadratic polynomials multiplied modulo `X² − ζ`: the first pair uses
/// `+ζ`, the second pair uses `−ζ`.  This is the trait-compatible
/// restriction of FIPS 203 Algorithm 10.
///
/// When instantiated at N=256 with `zetas = ZETAS[64..128]` this is the
/// full `multiply_ntts` below.  When instantiated at N=16 with 4 zetas
/// this is the trait's `ntt_multiply(lhs, rhs, z0, z1, z2, z3)`.
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::requires(zetas.len() < 1024 && zetas.len() * 4 == N)]
pub fn ntt_multiply_n<const N: usize>(
    p1: &[FieldElement; N],
    p2: &[FieldElement; N],
    zetas: &[FieldElement],
) -> [FieldElement; N] {
    createi(|i| {
        let group = i / 4;
        let zeta = if i % 4 < 2 {
            zetas[group]
        } else {
            zetas[group].neg()
        };
        if i % 2 == 0 {
            base_case_multiply_even(p1[i], p1[i + 1], p2[i], p2[i + 1], zeta)
        } else {
            base_case_multiply_odd(p1[i - 1], p1[i], p2[i - 1], p2[i])
        }
    })
}

pub fn multiply_ntts(p1: &Polynomial, p2: &Polynomial) -> Polynomial {
    ntt_multiply_n(p1, p2, &ZETAS[64..128])
}

pub fn vector_ntt<const RANK: usize>(vector: Vector<RANK>) -> Vector<RANK> {
    createi(|i| ntt(vector[i]))
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    use crate::{
        compress::tests::arb_ring_element, invert_ntt::ntt_inverse, parameters::FIELD_MODULUS,
    };

    const Q: i32 = FIELD_MODULUS as i32;

    fn mod_q(x: i32) -> FieldElement {
        FieldElement::new(x.rem_euclid(Q) as u16)
    }

    fn mod_pow(base: i32, exp: u32, modulus: i32) -> i32 {
        let mut result: i64 = 1;
        let mut b: i64 = (base as i64).rem_euclid(modulus as i64);
        let mut e = exp;
        while e > 0 {
            if e % 2 == 1 {
                result = (result * b).rem_euclid(modulus as i64);
            }
            b = (b * b).rem_euclid(modulus as i64);
            e /= 2;
        }
        result as i32
    }

    /// Reference NTT implementing FIPS 203 Algorithm 8 directly with loops.
    fn ref_ntt(f: &Polynomial) -> Polynomial {
        let mut fhat = *f;
        let mut k: usize = 1;
        let mut len: usize = 128;
        while len >= 2 {
            let mut start = 0;
            while start < 256 {
                let zeta = get_zeta(k);
                k += 1;
                for j in start..(start + len) {
                    let t = (zeta.val as i32 * fhat[j + len].val as i32).rem_euclid(Q);
                    fhat[j + len] = mod_q(fhat[j].val as i32 - t);
                    fhat[j] = mod_q(fhat[j].val as i32 + t);
                }
                start += 2 * len;
            }
            len /= 2;
        }
        fhat
    }

    const INVERSE_OF_128: FieldElement = FieldElement::new(3303);

    /// Reference inverse NTT implementing FIPS 203 Algorithm 9 directly with loops.
    fn ref_ntt_inverse(fhat: &Polynomial) -> Polynomial {
        let mut f = *fhat;
        let mut k: usize = 127;
        let mut len: usize = 2;
        while len <= 128 {
            let mut start = 0;
            while start < 256 {
                let zeta = get_zeta(k);
                k -= 1;
                for j in start..(start + len) {
                    let t = f[j];
                    f[j] = mod_q(t.val as i32 + f[j + len].val as i32);
                    f[j + len] = mod_q(zeta.val as i32 * (f[j + len].val as i32 - t.val as i32));
                }
                start += 2 * len;
            }
            len *= 2;
        }
        let inv128 = INVERSE_OF_128.val as i32;
        for coeff in f.iter_mut() {
            *coeff = mod_q(coeff.val as i32 * inv128);
        }
        f
    }

    /// Multiply polynomials in Z_q[X]/(X^256+1) using schoolbook, reducing mod q.
    fn poly_mul_schoolbook(a: &Polynomial, b: &Polynomial) -> Polynomial {
        let mut result = [0i64; 256];
        for i in 0..256 {
            for j in 0..256 {
                let prod = a[i].val as i64 * b[j].val as i64;
                if i + j < 256 {
                    result[i + j] += prod;
                } else {
                    result[i + j - 256] -= prod;
                }
            }
        }
        createi(|i| FieldElement::new((result[i].rem_euclid(Q as i64)) as u16))
    }

    #[test]
    fn seven_bit_reverse() {
        assert_eq!(64, bit_rev_7(1));
        assert_eq!(127, bit_rev_7(255));
        assert_eq!(78, bit_rev_7(185));
    }

    #[test]
    fn zetas_are_correct() {
        for i in 0..128 {
            let expected = mod_pow(ZETA.val as i32, bit_rev_7(i) as u32, Q);
            assert_eq!(
                get_zeta(i).val as i32,
                expected,
                "get_zeta({}] = {} but expected 17^BitRev7({}) = {}",
                i,
                get_zeta(i).val,
                i,
                expected
            );
        }
    }

    #[test]
    fn ntt_of_zero_is_zero() {
        let zero = [FieldElement::new(0); 256];
        assert_eq!(ntt(zero), zero);
    }

    #[test]
    fn ntt_matches_reference() {
        // Test with a simple input: f[0] = 1, rest zero (the constant polynomial 1)
        let mut f = [FieldElement::new(0); 256];
        f[0] = FieldElement::new(1);
        let ntt_result = ntt(f);
        let ref_result = ref_ntt(&f);
        assert_eq!(
            ntt_result, ref_result,
            "NTT mismatch for constant polynomial 1"
        );

        // Test with f[1] = 1 (the polynomial X)
        let mut f = [FieldElement::new(0); 256];
        f[1] = FieldElement::new(1);
        let ntt_result = ntt(f);
        let ref_result = ref_ntt(&f);
        assert_eq!(ntt_result, ref_result, "NTT mismatch for polynomial X");

        // Test with a more complex polynomial
        let f: Polynomial = createi(|i| FieldElement::new((i as u16 * 7 + 3) % FIELD_MODULUS));
        let ntt_result = ntt(f);
        let ref_result = ref_ntt(&f);
        assert_eq!(
            ntt_result, ref_result,
            "NTT mismatch for complex polynomial"
        );
    }

    #[test]
    fn ntt_inverse_matches_reference() {
        let mut fhat = [FieldElement::new(0); 256];
        fhat[0] = FieldElement::new(1);
        let inv_result = ntt_inverse(fhat);
        let ref_result = ref_ntt_inverse(&fhat);
        assert_eq!(
            inv_result, ref_result,
            "Inverse NTT mismatch for unit input"
        );

        let fhat: Polynomial = createi(|i| FieldElement::new((i as u16 * 13 + 5) % FIELD_MODULUS));
        let inv_result = ntt_inverse(fhat);
        let ref_result = ref_ntt_inverse(&fhat);
        assert_eq!(
            inv_result, ref_result,
            "Inverse NTT mismatch for complex input"
        );
    }

    #[test]
    fn base_case_multiply_known_values() {
        // (a0 + a1*X) * (b0 + b1*X) mod (X^2 - zeta) = c0 + c1*X
        // c0 = a0*b0 + a1*b1*zeta
        // c1 = a0*b1 + a1*b0
        let a0 = FieldElement::new(5);
        let a1 = FieldElement::new(3);
        let b0 = FieldElement::new(7);
        let b1 = FieldElement::new(2);
        let zeta = FieldElement::new(17);

        // c0 = 5*7 + 3*2*17 = 35 + 102 = 137
        let c0 = base_case_multiply_even(a0, a1, b0, b1, zeta);
        assert_eq!(c0, FieldElement::new(137));

        // c1 = 5*2 + 3*7 = 10 + 21 = 31
        let c1 = base_case_multiply_odd(a0, a1, b0, b1);
        assert_eq!(c1, FieldElement::new(31));
    }

    #[test]
    fn base_case_multiply_reduces_mod_q() {
        let a0 = FieldElement::new(3000);
        let a1 = FieldElement::new(3000);
        let b0 = FieldElement::new(3000);
        let b1 = FieldElement::new(3000);
        let zeta = FieldElement::new(1729);

        let c0 = base_case_multiply_even(a0, a1, b0, b1, zeta);
        assert!(c0.val < FIELD_MODULUS, "c0 = {} not in [0, q)", c0.val);

        let c1 = base_case_multiply_odd(a0, a1, b0, b1);
        assert!(c1.val < FIELD_MODULUS, "c1 = {} not in [0, q)", c1.val);
    }

    #[test]
    fn multiply_ntts_known_vector() {
        // Multiply NTT(1) * NTT(X) and verify result
        let mut f = [FieldElement::new(0); 256];
        f[0] = FieldElement::new(1);
        let mut g = [FieldElement::new(0); 256];
        g[1] = FieldElement::new(1);

        let f_ntt = ntt(f);
        let g_ntt = ntt(g);
        let product_ntt = multiply_ntts(&f_ntt, &g_ntt);
        let product = ntt_inverse(product_ntt);

        // f*g = 1*X = X, so product should be [0, 1, 0, 0, ...]
        let mut expected = [FieldElement::new(0); 256];
        expected[1] = FieldElement::new(1);
        assert_eq!(product, expected, "1 * X should equal X");
    }

    #[test]
    fn ntt_multiply_corresponds_to_polynomial_multiply() {
        // f = 1 + 2X + 3X^2
        let mut f = [FieldElement::new(0); 256];
        f[0] = FieldElement::new(1);
        f[1] = FieldElement::new(2);
        f[2] = FieldElement::new(3);

        // g = 4 + 5X
        let mut g = [FieldElement::new(0); 256];
        g[0] = FieldElement::new(4);
        g[1] = FieldElement::new(5);

        // f*g = 4 + 13X + 22X^2 + 15X^3 (schoolbook)
        let expected = poly_mul_schoolbook(&f, &g);

        let f_ntt = ntt(f);
        let g_ntt = ntt(g);
        let product_ntt = multiply_ntts(&f_ntt, &g_ntt);
        let product = ntt_inverse(product_ntt);

        assert_eq!(
            product, expected,
            "NTT multiplication should correspond to polynomial multiplication"
        );
    }

    #[test]
    fn ntt_multiply_with_reduction() {
        // Test multiplication where X^256 + 1 reduction matters
        // f = X^200, g = X^100
        // f*g = X^300 = X^300 mod (X^256+1) = -X^44
        let mut f = [FieldElement::new(0); 256];
        f[200] = FieldElement::new(1);
        let mut g = [FieldElement::new(0); 256];
        g[100] = FieldElement::new(1);

        let expected = poly_mul_schoolbook(&f, &g);
        // expected[44] should be -1 mod q = 3328
        assert_eq!(expected[44], FieldElement::new(FIELD_MODULUS - 1));

        let f_ntt = ntt(f);
        let g_ntt = ntt(g);
        let product_ntt = multiply_ntts(&f_ntt, &g_ntt);
        let product = ntt_inverse(product_ntt);

        assert_eq!(product, expected);
    }

    proptest! {
        #[test]
        fn to_ntt_and_back(ring_element in arb_ring_element(12)) {
            assert_eq!(ring_element, ntt_inverse(ntt(ring_element)));
        }

        #[test]
        fn ntt_matches_reference_proptest(ring_element in arb_ring_element(12)) {
            assert_eq!(ntt(ring_element), ref_ntt(&ring_element));
        }

        #[test]
        fn ntt_inverse_matches_reference_proptest(ring_element in arb_ring_element(12)) {
            assert_eq!(ntt_inverse(ring_element), ref_ntt_inverse(&ring_element));
        }

        #[test]
        fn ntt_multiply_is_poly_multiply(f_elem in arb_ring_element(8), g_elem in arb_ring_element(8)) {
            let expected = poly_mul_schoolbook(&f_elem, &g_elem);
            let f_ntt = ntt(f_elem);
            let g_ntt = ntt(g_elem);
            let product = ntt_inverse(multiply_ntts(&f_ntt, &g_ntt));
            assert_eq!(product, expected);
        }
    }
}

use crate::arithmetic::mod_q;
use crate::createi;
/// NTT and NTT⁻¹ — FIPS 204, Section 7.5 (Algorithms 41–43).
///
/// The NTT operates on polynomials in R_q = Z_q[X]/(X^256+1).
/// ζ = 1753 is a primitive 512th root of unity in Z_q.
/// The pre-computed zetas table stores ζ^BitRev8(k) mod q for k = 0..255.
use crate::parameters::{Polynomial, Q};

/// Pre-computed zetas: zetas[k] = ζ^BitRev8(k) mod q — FIPS 204, Appendix B.
pub(crate) const ZETAS: [i32; 256] = [
    1, 4808194, 3765607, 3761513, 5178923, 5496691, 5234739, 5178987, 7778734, 3542485, 2682288,
    2129892, 3764867, 7375178, 557458, 7159240, 5010068, 4317364, 2663378, 6705802, 4855975,
    7946292, 676590, 7044481, 5152541, 1714295, 2453983, 1460718, 7737789, 4795319, 2815639,
    2283733, 3602218, 3182878, 2740543, 4793971, 5269599, 2101410, 3704823, 1159875, 394148,
    928749, 1095468, 4874037, 2071829, 4361428, 3241972, 2156050, 3415069, 1759347, 7562881,
    4805951, 3756790, 6444618, 6663429, 4430364, 5483103, 3192354, 556856, 3870317, 2917338,
    1853806, 3345963, 1858416, 3073009, 1277625, 5744944, 3852015, 4183372, 5157610, 5258977,
    8106357, 2508980, 2028118, 1937570, 4564692, 2811291, 5396636, 7270901, 4158088, 1528066,
    482649, 1148858, 5418153, 7814814, 169688, 2462444, 5046034, 4213992, 4892034, 1987814,
    5183169, 1736313, 235407, 5130263, 3258457, 5801164, 1787943, 5989328, 6125690, 3482206,
    4197502, 7080401, 6018354, 7062739, 2461387, 3035980, 621164, 3901472, 7153756, 2925816,
    3374250, 1356448, 5604662, 2683270, 5601629, 4912752, 2312838, 7727142, 7921254, 348812,
    8052569, 1011223, 6026202, 4561790, 6458164, 6143691, 1744507, 1753, 6444997, 5720892, 6924527,
    2660408, 6600190, 8321269, 2772600, 1182243, 87208, 636927, 4415111, 4423672, 6084020, 5095502,
    4663471, 8352605, 822541, 1009365, 5926272, 6400920, 1596822, 4423473, 4620952, 6695264,
    4969849, 2678278, 4611469, 4829411, 635956, 8129971, 5925040, 4234153, 6607829, 2192938,
    6653329, 2387513, 4768667, 8111961, 5199961, 3747250, 2296099, 1239911, 4541938, 3195676,
    2642980, 1254190, 8368000, 2998219, 141835, 8291116, 2513018, 7025525, 613238, 7070156,
    6161950, 7921677, 6458423, 4040196, 4908348, 2039144, 6500539, 7561656, 6201452, 6757063,
    2105286, 6006015, 6346610, 586241, 7200804, 527981, 5637006, 6903432, 1994046, 2491325,
    6987258, 507927, 7192532, 7655613, 6545891, 5346675, 8041997, 2647994, 3009748, 5767564,
    4148469, 749577, 4357667, 3980599, 2569011, 6764887, 1723229, 1665318, 2028038, 1163598,
    5011144, 3994671, 8368538, 7009900, 3020393, 3363542, 214880, 545376, 7609976, 3105558,
    7277073, 508145, 7826699, 860144, 3430436, 140244, 6866265, 6195333, 3123762, 2358373, 6187330,
    5365997, 6663603, 2926054, 7987710, 8077412, 3531229, 4405932, 4606686, 1900052, 7598542,
    1054478, 7648983,
];

/// BitRev8(m) — FIPS 204, Algorithm 43.
///
/// Reverses the bits of an 8-bit integer.
#[hax_lib::requires(m < 256)]
pub(crate) fn bit_rev_8(m: usize) -> usize {
    let mut r = 0usize;
    let mut v = m;
    for _i in 0..8 {
        r = (r << 1) | (v & 1);
        v >>= 1;
    }
    r
}

/// Single Cooley-Tukey butterfly layer of the NTT — FIPS 204, Algorithm 41.
///
/// `layer` is the bit-shift exponent: len = 1 << layer.
/// For NTT, layers are applied from 7 down to 0 (len = 128, 64, ..., 1).
/// The zeta index for block `round` at this layer is `128/len + round`.
#[hax_lib::fstar::options("--split_queries always --z3rlimit 80")]
#[hax_lib::requires(layer <= 7)]
fn ntt_layer(p: Polynomial, layer: usize) -> Polynomial {
    let len = 1 << layer;
    let k = 128 / len; // base zeta index for this layer
    createi(|i| {
        let round = i / (2 * len);
        let idx = i % (2 * len);
        let z = ZETAS[round + k] as i64;
        if idx < len {
            let t = mod_q(z * p[i + len] as i64);
            mod_q(p[i] as i64 + t as i64)
        } else {
            let t = mod_q(z * p[i] as i64);
            mod_q(p[i - len] as i64 - t as i64)
        }
    })
}

/// NTT(w) — FIPS 204, Algorithm 41.
///
/// Computes the Number Theoretic Transform of polynomial w.
pub(crate) fn ntt(w: Polynomial) -> Polynomial {
    let p = ntt_layer(w, 7);
    let p = ntt_layer(p, 6);
    let p = ntt_layer(p, 5);
    let p = ntt_layer(p, 4);
    let p = ntt_layer(p, 3);
    let p = ntt_layer(p, 2);
    let p = ntt_layer(p, 1);
    let p = ntt_layer(p, 0);
    p
}

/// Single Gentleman-Sande butterfly layer of the inverse NTT — FIPS 204, Algorithm 42.
///
/// `layer` is the bit-shift exponent: len = 1 << layer.
/// For INTT, layers are applied from 0 up to 7 (len = 1, 2, ..., 128).
/// The zeta index for block `round` at this layer is `256/len - 1 - round`.
#[hax_lib::fstar::options("--split_queries always --z3rlimit 80")]
#[hax_lib::requires(layer <= 7)]
fn intt_layer(p: Polynomial, layer: usize) -> Polynomial {
    let len = 1 << layer;
    let k = (256 / len) - 1; // base zeta index for this layer
    createi(|i| {
        let round = i / (2 * len);
        let idx = i % (2 * len);
        if idx < len {
            mod_q(p[i] as i64 + p[i + len] as i64)
        } else {
            // -ζ mod q = q - ζ (for ζ in [0, q-1])
            let z = (Q as i64 - ZETAS[k - round] as i64) % Q as i64;
            mod_q(z * (p[i - len] as i64 - p[i] as i64))
        }
    })
}

/// Multiply all coefficients by 256⁻¹ mod q = 8347681.
fn reduce_polynomial(p: Polynomial) -> Polynomial {
    let f = 8347681i64;
    createi(|i| mod_q(f * p[i] as i64))
}

/// NTT⁻¹(ŵ) — FIPS 204, Algorithm 42.
///
/// Computes the inverse Number Theoretic Transform.
pub(crate) fn intt(w_hat: Polynomial) -> Polynomial {
    let p = intt_layer(w_hat, 0);
    let p = intt_layer(p, 1);
    let p = intt_layer(p, 2);
    let p = intt_layer(p, 3);
    let p = intt_layer(p, 4);
    let p = intt_layer(p, 5);
    let p = intt_layer(p, 6);
    let p = intt_layer(p, 7);
    reduce_polynomial(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zetas_spot_check() {
        // zetas[0] = ζ^BitRev8(0) = ζ^0 = 1
        assert_eq!(ZETAS[0], 1);
        // zetas[128] = ζ^BitRev8(128) = ζ^1 = 1753
        assert_eq!(ZETAS[128], 1753);
    }

    #[test]
    fn zetas_computed() {
        // Verify the table by computing ζ^BitRev8(k) mod q
        fn pow_mod(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
            let mut result = 1i64;
            base %= modulus;
            while exp > 0 {
                if exp & 1 == 1 {
                    result = result * base % modulus;
                }
                exp >>= 1;
                base = base * base % modulus;
            }
            result
        }
        for k in 0..256 {
            let br = bit_rev_8(k);
            let expected = pow_mod(1753, br as i64, Q as i64) as i32;
            assert_eq!(ZETAS[k], expected, "mismatch at k={k}");
        }
    }

    #[test]
    fn bit_rev_8_basic() {
        assert_eq!(bit_rev_8(0), 0);
        assert_eq!(bit_rev_8(1), 128);
        assert_eq!(bit_rev_8(128), 1);
        assert_eq!(bit_rev_8(255), 255);
    }

    #[test]
    fn ntt_intt_roundtrip() {
        let mut p = [0i32; 256];
        p[0] = 1;
        p[1] = 2;
        p[255] = 100;
        let p_hat = ntt(p);
        let p_back = intt(p_hat);
        for i in 0..256 {
            assert_eq!(p[i], p_back[i], "mismatch at index {i}");
        }
    }

    #[test]
    fn inverse_of_256() {
        // Verify: 256 * 8347681 ≡ 1 (mod q)
        assert_eq!((256i64 * 8347681i64) % Q as i64, 1);
    }
}

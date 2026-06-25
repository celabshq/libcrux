/// ML-DSA parameters — FIPS 204, Section 4.

/// Field modulus q = 2^23 - 2^13 + 1.
pub(crate) const Q: i32 = 8380417;

/// Number of dropped bits from t — FIPS 204, Table 1.
pub(crate) const D: usize = 13;

/// Primitive 512th root of unity in Z_q.
pub(crate) const ZETA: i32 = 1753;

/// Coefficients per polynomial.
pub(crate) const N: usize = 256;

/// A polynomial in R_q = Z_q[X]/(X^256 + 1), represented as 256 coefficients.
/// Coefficients are in the range [0, q-1] unless otherwise noted.
pub(crate) type Polynomial = [i32; 256];

/// A zero polynomial.
pub(crate) const ZERO_POLY: Polynomial = [0i32; 256];

/// ML-DSA parameter set — FIPS 204, Table 1.
///
/// Field-level refinements (`#[hax_lib::refine(...)]`) bound each
/// parameter to the union of values that the three FIPS 204 standard
/// parameter sets (ML-DSA-44/65/87) take.  Construction sites for
/// `ML_DSA_{44,65,87}` below discharge these refinements once; callers
/// of any function taking `&MlDsaParams` then get the bounds for free
/// without repeating them in every `#[hax_lib::requires]`.
#[hax_lib::attributes]
pub struct MlDsaParams {
    /// Matrix row dimension.
    #[hax_lib::refine(k <= 8)]
    pub k: usize,
    /// Matrix column dimension.
    #[hax_lib::refine(l <= 7)]
    pub l: usize,
    /// Number of ±1 coefficients in c — collision strength.
    #[hax_lib::refine(tau <= 64)]
    pub tau: usize,
    /// Collision strength of c̃ in bits.
    #[hax_lib::refine(lambda <= 256)]
    pub lambda: usize,
    /// Coefficient range for y.
    #[hax_lib::refine(gamma1 == (1i32 << 17) || gamma1 == (1i32 << 19))]
    pub gamma1: i32,
    /// Low-order rounding range.
    #[hax_lib::refine(gamma2 == (Q - 1) / 88 || gamma2 == (Q - 1) / 32)]
    pub gamma2: i32,
    /// Secret key coefficient bound.
    #[hax_lib::refine(eta == 2 || eta == 4)]
    pub eta: usize,
    /// Maximum number of 1s in the hint h.
    #[hax_lib::refine(omega <= 80)]
    pub omega: usize,
    /// Signing bound: τ · η.
    #[hax_lib::refine(beta >= 0)]
    pub beta: i32,
}

/// Public key size in bytes: 32 + 32·k·(bitlen(q-1) - d).
#[hax_lib::requires(params.k <= 16)]
pub fn pk_size(params: &MlDsaParams) -> usize {
    32 + 32 * params.k * 10 // bitlen(q-1) - d = 23 - 13 = 10
}

/// Private key size in bytes.
#[hax_lib::requires(params.k <= 16 && params.l <= 16 && params.eta <= 8)]
pub fn sk_size(params: &MlDsaParams) -> usize {
    let eta_bits = if params.eta == 2 { 3 } else { 4 };
    32 + 32 + 64 + 32 * (params.l + params.k) * eta_bits + 32 * params.k * D
}

/// Signature size in bytes.
#[hax_lib::requires(params.k <= 16 && params.l <= 16 && params.lambda <= 1024 && params.omega <= 256)]
pub fn sig_size(params: &MlDsaParams) -> usize {
    let gamma1_bits = if params.gamma1 == (1 << 17) { 18 } else { 20 };
    params.lambda / 4 + 32 * params.l * gamma1_bits + params.omega + params.k
}

/// ML-DSA-44 — FIPS 204, Table 1.
pub const ML_DSA_44: MlDsaParams = MlDsaParams {
    k: 4,
    l: 4,
    tau: 39,
    lambda: 128,
    gamma1: 1 << 17,
    gamma2: (Q - 1) / 88,
    eta: 2,
    omega: 80,
    beta: 39 * 2,
};

/// ML-DSA-65 — FIPS 204, Table 1.
pub const ML_DSA_65: MlDsaParams = MlDsaParams {
    k: 6,
    l: 5,
    tau: 49,
    lambda: 192,
    gamma1: 1 << 19,
    gamma2: (Q - 1) / 32,
    eta: 4,
    omega: 55,
    beta: 49 * 4,
};

/// ML-DSA-87 — FIPS 204, Table 1.
pub const ML_DSA_87: MlDsaParams = MlDsaParams {
    k: 8,
    l: 7,
    tau: 60,
    lambda: 256,
    gamma1: 1 << 19,
    gamma2: (Q - 1) / 32,
    eta: 2,
    omega: 75,
    beta: 60 * 2,
};

// Per-parameter-set sizes in bytes (FIPS 204, Table 2).
pub const ML_DSA_44_PK_SIZE: usize = 1312;
pub const ML_DSA_44_SK_SIZE: usize = 2560;
pub const ML_DSA_44_SIG_SIZE: usize = 2420;
pub const ML_DSA_65_PK_SIZE: usize = 1952;
pub const ML_DSA_65_SK_SIZE: usize = 4032;
pub const ML_DSA_65_SIG_SIZE: usize = 3309;
pub const ML_DSA_87_PK_SIZE: usize = 2592;
pub const ML_DSA_87_SK_SIZE: usize = 4896;
pub const ML_DSA_87_SIG_SIZE: usize = 4627;

// w1 encoded sizes: K * bytes_per_poly.
// ML-DSA-44: gamma2=(Q-1)/88, w1_max=43, 6 bits, 192 bytes/poly, K=4 → 768
pub const ML_DSA_44_W1_SIZE: usize = 768;
// ML-DSA-65: gamma2=(Q-1)/32, w1_max=15, 4 bits, 128 bytes/poly, K=6 → 768
pub const ML_DSA_65_W1_SIZE: usize = 768;
// ML-DSA-87: gamma2=(Q-1)/32, w1_max=15, 4 bits, 128 bytes/poly, K=8 → 1024
pub const ML_DSA_87_W1_SIZE: usize = 1024;

// c_tilde lengths: lambda / 4.
pub const ML_DSA_44_C_TILDE_LEN: usize = 32;
pub const ML_DSA_65_C_TILDE_LEN: usize = 48;
pub const ML_DSA_87_C_TILDE_LEN: usize = 64;

/// bitlen(n): number of bits needed to represent n.
#[hax_lib::fstar::options("--z3rlimit 150")]
#[hax_lib::ensures(|result| result <= 64usize)]
pub(crate) fn bitlen(n: usize) -> usize {
    let mut bits = 0usize;
    let mut v = n;
    for _i in 0usize..64usize {
        hax_lib::loop_invariant!(|_i: usize| bits <= _i);
        if v > 0 {
            bits += 1;
            v >>= 1;
        }
    }
    bits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_sizes() {
        // Table 2: Sizes (in bytes) of keys and signatures
        assert_eq!(pk_size(&ML_DSA_44), 1312);
        assert_eq!(pk_size(&ML_DSA_65), 1952);
        assert_eq!(pk_size(&ML_DSA_87), 2592);
        assert_eq!(sk_size(&ML_DSA_44), 2560);
        assert_eq!(sk_size(&ML_DSA_65), 4032);
        assert_eq!(sk_size(&ML_DSA_87), 4896);
        assert_eq!(sig_size(&ML_DSA_44), 2420);
        assert_eq!(sig_size(&ML_DSA_65), 3309);
        assert_eq!(sig_size(&ML_DSA_87), 4627);
    }

    #[test]
    fn field_modulus() {
        assert_eq!(Q, (1 << 23) - (1 << 13) + 1);
    }

    #[test]
    fn gamma2_values() {
        assert_eq!((Q - 1) / 88, 95232);
        assert_eq!((Q - 1) / 32, 261888);
    }
}

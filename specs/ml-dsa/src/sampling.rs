use crate::createi;
use crate::encoding::{coeff_from_half_byte, coeff_from_three_bytes};
use crate::hash_functions::h;
/// Pseudorandom sampling — FIPS 204, Section 7.3 (Algorithms 29–34).
use crate::parameters::{Polynomial, Q, ZERO_POLY};

/// Concatenate a 32-byte seed with two single bytes → [u8; 34].
fn concat_2bytes(a: &[u8; 32], b: u8, c: u8) -> [u8; 34] {
    let mut result = [0u8; 34];
    result[..32].copy_from_slice(a);
    result[32] = b;
    result[33] = c;
    result
}

/// Concatenate a 64-byte seed with a u16 in little-endian → [u8; 66].
fn concat_u16_le(a: &[u8; 64], val: u16) -> [u8; 66] {
    let mut result = [0u8; 66];
    result[..64].copy_from_slice(a);
    result[64] = val as u8;
    result[65] = (val >> 8) as u8;
    result
}

/// Maximum number of bytes the spec's `sample_in_ball` consumes from
/// the SHAKE-256 stream of ρ.
///
/// FIPS 204 Algorithm 29 specifies an *unbounded* SHAKE-256 stream:
/// the rejection-sampling inner loop keeps squeezing until success.
/// A reference impl (e.g. `libcrux-ml-dsa::sample::sample_challenge_ring_element`)
/// follows that contract via block-by-block `state.squeeze_next_block()`.
/// This spec approximates the unbounded stream with a single fixed
/// buffer of `SAMPLE_IN_BALL_BUDGET` bytes so the F* extraction is a
/// total function.
///
/// The expected byte consumption for the standard parameter sets is
/// small: for τ ≤ 60 (largest), each outer iteration's expected number
/// of bytes is ≈ 256 / (256 − τ) ≈ 1.27, so total expected consumption
/// is ≈ τ · 1.27 + 8 ≈ 84 bytes.  The probability of exceeding this
/// budget is < 2⁻¹²⁸ (Chernoff bound, see FIPS 204 §6.2.2).
///
/// **Spec ↔ impl equivalence theorem (informal)**: an impl that
/// consumes the SHAKE-256 stream block-by-block is equivalent to this
/// spec on every ρ for which the spec returns `Ok(_)` (i.e., for which
/// rejection sampling completes within `SAMPLE_IN_BALL_BUDGET` bytes).
/// The probabilistically-impossible `Err(SampleInBallExhausted)` arm
/// captures the only place where the spec departs from FIPS 204.
pub const SAMPLE_IN_BALL_BUDGET: usize = 1024;

/// SampleInBall(ρ) — FIPS 204, Algorithm 29.
///
/// Samples a polynomial c ∈ R with coefficients in {-1, 0, 1} and
/// Hamming weight exactly τ, using a Fisher-Yates shuffle over the
/// SHAKE-256 stream of ρ.  See `SAMPLE_IN_BALL_BUDGET` for the
/// finite-buffer approximation and its impl-equivalence semantics.
#[hax_lib::fstar::options("--z3rlimit 400")]
#[hax_lib::requires(tau <= 64)]
pub(crate) fn sample_in_ball(
    rho: &[u8],
    tau: usize,
) -> Result<Polynomial, crate::error::MlDsaError> {
    let buf: [u8; SAMPLE_IN_BALL_BUDGET] = h(rho);
    let mut c = ZERO_POLY;

    let signs = u64::from_le_bytes([
        buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
    ]);

    let mut byte_offset = 8usize;
    let mut exhausted = false;

    for counter in 0..tau {
        if !exhausted {
            let i = 256 - tau + counter;
            // Scan forward through buf for the first byte j ≤ i.
            // The inner cap matches the outer-buf cap so this loop is
            // guaranteed to either find a byte or observe exhaustion.
            let mut j = 0usize;
            let mut found = false;
            for _scan in 0..SAMPLE_IN_BALL_BUDGET {
                hax_lib::loop_invariant!(|_scan: usize| !found || j <= i);
                if !found && byte_offset < SAMPLE_IN_BALL_BUDGET {
                    let candidate = buf[byte_offset] as usize;
                    byte_offset += 1;
                    if candidate <= i {
                        j = candidate;
                        found = true;
                    }
                }
            }
            if found {
                // Fisher-Yates swap
                c[i] = c[j];
                let sign_bit = (signs >> counter) & 1;
                c[j] = if sign_bit == 1 { Q - 1 } else { 1 };
            } else {
                exhausted = true;
            }
        }
    }

    if exhausted {
        Err(crate::error::MlDsaError::SampleInBallExhausted)
    } else {
        Ok(c)
    }
}

/// RejNTTPoly(ρ) — FIPS 204, Algorithm 30.
///
/// Samples a polynomial in T_q (NTT domain) using rejection sampling
/// from SHAKE128 output.
#[hax_lib::fstar::options("--z3rlimit 300")]
pub(crate) fn rej_ntt_poly(seed: &[u8]) -> Polynomial {
    let buf: [u8; 1024] = crate::hash_functions::g(seed);
    let mut a = ZERO_POLY;
    let mut j = 0usize;
    for i in 0..(1024 / 3) {
        if j < 256 {
            if let Some(coeff) = coeff_from_three_bytes(buf[3 * i], buf[3 * i + 1], buf[3 * i + 2])
            {
                a[j] = coeff;
                j += 1;
            }
        }
    }
    a
}

/// RejBoundedPoly(ρ) — FIPS 204, Algorithm 31.
///
/// Samples a polynomial with coefficients in [-η, η] using rejection
/// sampling from SHAKE256 output.
#[hax_lib::fstar::options("--z3rlimit 300")]
pub(crate) fn rej_bounded_poly(seed: &[u8], eta: usize) -> Polynomial {
    let buf: [u8; 512] = h(seed);
    let mut a = ZERO_POLY;
    let mut j = 0usize;
    for i in 0..512 {
        if j < 256 {
            let z0 = coeff_from_half_byte(buf[i] & 0x0F, eta);
            if let Some(c) = z0 {
                a[j] = c;
                j += 1;
            }
        }
        if j < 256 {
            let z1 = coeff_from_half_byte(buf[i] >> 4, eta);
            if let Some(c) = z1 {
                a[j] = c;
                j += 1;
            }
        }
    }
    a
}

/// ExpandA(ρ) — FIPS 204, Algorithm 32.
///
/// Samples a k × ℓ matrix Â of polynomials in T_q from a 32-byte seed ρ.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(K <= 8 && L <= 8)]
pub(crate) fn expand_a<const K: usize, const L: usize>(rho: &[u8; 32]) -> [[Polynomial; L]; K] {
    createi(|r| {
        createi(|s| {
            let seed = concat_2bytes(rho, s as u8, r as u8);
            rej_ntt_poly(&seed)
        })
    })
}

/// ExpandS(ρ') — FIPS 204, Algorithm 33.
///
/// Samples vectors s1 ∈ R^ℓ and s2 ∈ R^k with coefficients in [-η, η].
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(K <= 8 && L <= 8)]
pub(crate) fn expand_s<const K: usize, const L: usize>(
    rho_prime: &[u8; 64],
    eta: usize,
) -> ([Polynomial; L], [Polynomial; K]) {
    let s1: [Polynomial; L] = createi(|r| {
        let seed = concat_u16_le(rho_prime, r as u16);
        rej_bounded_poly(&seed, eta)
    });
    let s2: [Polynomial; K] = createi(|r| {
        let idx = (r + L) as u16;
        let seed = concat_u16_le(rho_prime, idx);
        rej_bounded_poly(&seed, eta)
    });
    (s1, s2)
}

/// ExpandMask(ρ'', κ) — FIPS 204, Algorithm 34.
///
/// Samples a vector y ∈ R^ℓ with coefficients in [-γ1+1, γ1].
#[hax_lib::fstar::options("--z3rlimit 400")]
#[hax_lib::requires(
    L <= 256 && kappa <= 65535 - L
    && (gamma1 == (1i32 << 17) || gamma1 == (1i32 << 19))
)]
pub(crate) fn expand_mask<const L: usize>(
    rho_pp: &[u8; 64],
    kappa: usize,
    gamma1: i32,
) -> [Polynomial; L] {
    createi(|r| {
        let idx = (kappa + r) as u16;
        let seed = concat_u16_le(rho_pp, idx);
        if gamma1 == (1 << 17) {
            let buf: [u8; 576] = h(&seed);
            crate::encoding::bit_unpack(&buf, gamma1 as usize - 1, gamma1 as usize)
        } else {
            let buf: [u8; 640] = h(&seed);
            crate::encoding::bit_unpack(&buf, gamma1 as usize - 1, gamma1 as usize)
        }
    })
}

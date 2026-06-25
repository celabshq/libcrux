use crate::createi;
/// Encoding and decoding of ML-DSA keys, signatures, and polynomials.
/// FIPS 204, Sections 7.1–7.2 (Algorithms 9–28).
use crate::parameters::{bitlen, MlDsaParams, Polynomial, D, Q, ZERO_POLY};
use hax_lib::int::ToInt;

// ========================================================================
// Bit-packing — Algorithms 16–21
// ========================================================================

/// SimpleBitPack(w, b) — Algorithm 16.
///
/// Packs polynomial w (coefficients in [0, b]) into BYTES bytes.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(BYTES == 32 * bitlen(b) && bitlen(b) <= 23)]
pub(crate) fn simple_bit_pack<const BYTES: usize>(w: &Polynomial, b: usize) -> [u8; BYTES] {
    let bits = bitlen(b);
    let mut out = [0u8; BYTES];
    for i in 0usize..256usize {
        hax_lib::loop_invariant!(|i: usize| i <= 256usize);
        let val = w[i] as u32;
        for bit in 0usize..bits {
            hax_lib::loop_invariant!(|bit: usize| bit <= bits);
            let pos = i * bits + bit;
            if (val >> bit) & 1 == 1 {
                out[pos / 8] |= 1 << (pos % 8);
            }
        }
    }
    out
}

/// BitPack(w, a, b) — Algorithm 17.
///
/// Packs polynomial w (coefficients in [-a, b]) into BYTES bytes.
/// Stores b - w_i for each coefficient.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(a.to_int() + b.to_int() <= 16777216usize.to_int() && BYTES == 32 * bitlen(a + b) && bitlen(a + b) <= 23)]
pub(crate) fn bit_pack<const BYTES: usize>(w: &Polynomial, a: usize, b: usize) -> [u8; BYTES] {
    let bits = bitlen(a + b);
    let mut out = [0u8; BYTES];
    for i in 0usize..256usize {
        hax_lib::loop_invariant!(|i: usize| i <= 256usize);
        let val = (b as i64 - w[i] as i64) as u32;
        for bit in 0usize..bits {
            hax_lib::loop_invariant!(|bit: usize| bit <= bits);
            let pos = i * bits + bit;
            if (val >> bit) & 1 == 1 {
                out[pos / 8] |= 1 << (pos % 8);
            }
        }
    }
    out
}

/// SimpleBitUnpack(v, b) — Algorithm 18.
///
/// Unpacks bytes into a polynomial with coefficients in [0, 2^c - 1] where c = bitlen(b).
#[hax_lib::fstar::options("--z3rlimit 300")]
pub(crate) fn simple_bit_unpack(v: &[u8], b: usize) -> Polynomial {
    let bits = bitlen(b);
    let mut w = ZERO_POLY;
    for i in 0usize..256usize {
        hax_lib::loop_invariant!(|i: usize| i <= 256usize);
        let mut val = 0u32;
        for bit in 0usize..24usize {
            hax_lib::loop_invariant!(|bit: usize| bit <= 24usize);
            if bit < bits {
                let pos = i * bits + bit;
                if pos / 8 < v.len() && (v[pos / 8] >> (pos % 8)) & 1 == 1 {
                    val |= 1 << bit;
                }
            }
        }
        w[i] = val as i32;
    }
    w
}

/// BitUnpack(v, a, b) — Algorithm 19.
///
/// Unpacks bytes into a polynomial with coefficients in [b - 2^c + 1, b]
/// where c = bitlen(a + b). When a + b + 1 is a power of 2, coefficients are in [-a, b].
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(a.to_int() + b.to_int() <= 16777216usize.to_int())]
pub(crate) fn bit_unpack(v: &[u8], a: usize, b: usize) -> Polynomial {
    let bits = bitlen(a + b);
    let mut w = ZERO_POLY;
    for i in 0usize..256usize {
        hax_lib::loop_invariant!(|i: usize| i <= 256usize);
        let mut val = 0u32;
        for bit in 0usize..24usize {
            hax_lib::loop_invariant!(|bit: usize| bit <= 24usize);
            if bit < bits {
                let pos = i * bits + bit;
                if pos / 8 < v.len() && (v[pos / 8] >> (pos % 8)) & 1 == 1 {
                    val |= 1 << bit;
                }
            }
        }
        w[i] = (b as i64 - val as i64) as i32;
    }
    w
}

// ========================================================================
// Hint encoding — Algorithms 20–21
// ========================================================================

/// HintBitPack(h) — Algorithm 20.
///
/// Encodes hint vector h (k polynomials with binary coefficients,
/// collectively at most ω nonzero) into ω + k bytes.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(K <= 8 && omega <= 256 && omega + K == HINT_BYTES)]
pub(crate) fn hint_bit_pack<const K: usize, const HINT_BYTES: usize>(
    h: &[[bool; 256]; K],
    omega: usize,
) -> [u8; HINT_BYTES] {
    let mut y = [0u8; HINT_BYTES];
    let mut index = 0usize;
    for i in 0usize..K {
        for j in 0usize..256usize {
            if h[i][j] && index < omega {
                y[index] = j as u8;
                index += 1;
            }
        }
        y[omega + i] = index as u8;
    }
    y
}

/// HintBitUnpack(y, omega) — Algorithm 21.
///
/// Decodes hint vector from ω + k bytes. Returns None if malformed.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(K <= 8 && omega <= 256 && omega + K <= y.len())]
pub(crate) fn hint_bit_unpack<const K: usize>(y: &[u8], omega: usize) -> Option<[[bool; 256]; K]> {
    let mut h = [[false; 256]; K];
    let mut index = 0usize;
    let mut valid = true;
    for i in 0..K {
        if valid {
            let end = y[omega + i] as usize;
            if end < index || end > omega {
                valid = false;
            } else {
                let first = index;
                for _scan in 0..256 {
                    if valid && index < end {
                        if index > first && y[index - 1] >= y[index] {
                            valid = false;
                        } else {
                            h[i][y[index] as usize] = true;
                            index += 1;
                        }
                    }
                }
            }
        }
    }
    // Check remaining bytes are zero
    for i in 0..256 {
        if valid && i >= index && i < omega {
            if y[i] != 0 {
                valid = false;
            }
        }
    }
    if valid {
        Some(h)
    } else {
        None
    }
}

// ========================================================================
// Key and signature encoding — Algorithms 22–28
// ========================================================================

/// pkEncode(ρ, t1) — Algorithm 22.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(K <= 8 && PK_SIZE == 32 + 320 * K)]
pub(crate) fn pk_encode<const K: usize, const PK_SIZE: usize>(
    rho: &[u8; 32],
    t1: &[Polynomial; K],
) -> [u8; PK_SIZE] {
    let mut pk = [0u8; PK_SIZE];
    pk[..32].copy_from_slice(rho);
    for i in 0usize..K {
        hax_lib::fstar!("assert_norm(${bitlen} (sz 1023) == sz 10)");
        let encoded: [u8; 320] = simple_bit_pack::<320>(&t1[i], 1023);
        pk[32 + i * 320..32 + (i + 1) * 320].copy_from_slice(&encoded);
    }
    pk
}

/// pkDecode(pk) — Algorithm 23.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(K <= 8 && pk.len() >= 32 + 320 * K)]
pub(crate) fn pk_decode<const K: usize>(pk: &[u8]) -> ([u8; 32], [Polynomial; K]) {
    let mut rho = [0u8; 32];
    rho.copy_from_slice(&pk[..32]);
    let t1: [Polynomial; K] = createi(|i| {
        let start = 32 + i * 320;
        simple_bit_unpack(&pk[start..start + 320], 1023)
    });
    (rho, t1)
}

/// skEncode(ρ, K, tr, s1, s2, t0) — Algorithm 24.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(K <= 8 && L <= 8 && SK_SIZE >= 128 + (L + K) * 32 * (if params.eta == 2 { 3 } else { 4 }) + K * 416)]
pub(crate) fn sk_encode<const K: usize, const L: usize, const SK_SIZE: usize>(
    rho: &[u8; 32],
    key: &[u8; 32],
    tr: &[u8; 64],
    s1: &[Polynomial; L],
    s2: &[Polynomial; K],
    t0: &[Polynomial; K],
    params: &MlDsaParams,
) -> [u8; SK_SIZE] {
    let eta = params.eta;
    let mut sk = [0u8; SK_SIZE];
    sk[..32].copy_from_slice(rho);
    sk[32..64].copy_from_slice(key);
    sk[64..128].copy_from_slice(tr);
    let mut offset = 128;
    // eta=2 → bitlen(4)=3 → 32*3=96 bytes; eta=4 → bitlen(8)=4 → 32*4=128 bytes
    hax_lib::fstar!("assert_norm(${bitlen} (sz 4) == sz 3)");
    hax_lib::fstar!("assert_norm(${bitlen} (sz 8) == sz 4)");
    for i in 0..L {
        hax_lib::loop_invariant!(|i: usize| offset == 128 + i * (bitlen(2 * eta) * 32));
        if eta == 2 {
            let encoded: [u8; 96] = bit_pack::<96>(&s1[i], eta, eta);
            sk[offset..offset + 96].copy_from_slice(&encoded);
            offset += 96;
        } else {
            let encoded: [u8; 128] = bit_pack::<128>(&s1[i], eta, eta);
            sk[offset..offset + 128].copy_from_slice(&encoded);
            offset += 128;
        }
    }
    for i in 0..K {
        hax_lib::loop_invariant!(
            |i: usize| offset == 128 + L * (bitlen(2 * eta) * 32) + i * (bitlen(2 * eta) * 32)
        );
        if eta == 2 {
            let encoded: [u8; 96] = bit_pack::<96>(&s2[i], eta, eta);
            sk[offset..offset + 96].copy_from_slice(&encoded);
            offset += 96;
        } else {
            let encoded: [u8; 128] = bit_pack::<128>(&s2[i], eta, eta);
            sk[offset..offset + 128].copy_from_slice(&encoded);
            offset += 128;
        }
    }
    // t0: bit_pack with a = 2^(d-1) - 1 = 4095, b = 2^(d-1) = 4096
    // bitlen(4095 + 4096) = bitlen(8191) = 13 → 32*13 = 416 bytes
    hax_lib::fstar!("assert_norm(${bitlen} (sz (4095 + 4096)) == sz 13)");
    for i in 0..K {
        hax_lib::loop_invariant!(
            |i: usize| offset == 128 + (L + K) * (bitlen(2 * eta) * 32) + i * 416
        );
        let encoded: [u8; 416] = bit_pack::<416>(&t0[i], (1 << (D - 1)) - 1, 1 << (D - 1));
        sk[offset..offset + 416].copy_from_slice(&encoded);
        offset += 416;
    }
    sk
}

/// skDecode(sk) — Algorithm 25.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(K <= 8 && L <= 8 && sk.len() >= 128 + (L + K) * 32 * (if params.eta == 2 { 3 } else { 4 }) + K * 416)]
pub(crate) fn sk_decode<const K: usize, const L: usize>(
    sk: &[u8],
    params: &MlDsaParams,
) -> (
    [u8; 32],
    [u8; 32],
    [u8; 64],
    [Polynomial; L],
    [Polynomial; K],
    [Polynomial; K],
) {
    let eta = params.eta;
    hax_lib::fstar!("assert_norm(${bitlen} (sz 4) == sz 3)");
    hax_lib::fstar!("assert_norm(${bitlen} (sz 8) == sz 4)");
    let eta_bytes = 32 * bitlen(2 * eta);

    let mut rho = [0u8; 32];
    rho.copy_from_slice(&sk[..32]);
    let mut key = [0u8; 32];
    key.copy_from_slice(&sk[32..64]);
    let mut tr = [0u8; 64];
    tr.copy_from_slice(&sk[64..128]);

    let s1: [Polynomial; L] = createi(|i| {
        let offset = 128 + i * eta_bytes;
        bit_unpack(&sk[offset..offset + eta_bytes], eta, eta)
    });
    let s2: [Polynomial; K] = createi(|i| {
        let offset = 128 + L * eta_bytes + i * eta_bytes;
        bit_unpack(&sk[offset..offset + eta_bytes], eta, eta)
    });

    let d_minus_1 = D - 1;
    hax_lib::fstar!("assert_norm(${bitlen} (sz (pow2 12 - 1 + pow2 12)) == sz 13)");
    let t0_bytes = 32 * bitlen((1 << d_minus_1) - 1 + (1 << d_minus_1));
    let t0: [Polynomial; K] = createi(|i| {
        let offset = 128 + (L + K) * eta_bytes + i * t0_bytes;
        bit_unpack(
            &sk[offset..offset + t0_bytes],
            (1 << d_minus_1) - 1,
            1 << d_minus_1,
        )
    });
    (rho, key, tr, s1, s2, t0)
}

/// sigEncode(c̃, z, h) — Algorithm 26.
///
/// The hint-pack buffer-size selection branches on the three FIPS 204
/// parameter sets (`(γ1, ω, K) ∈ {(2¹⁷, 80, 4), (2¹⁹, 55, 6), (2¹⁹, 75, 8)}`),
/// so the precondition must enumerate them.
#[hax_lib::fstar::options("--z3rlimit 400")]
#[hax_lib::requires(
    L <= 8
    && c_tilde.len() >= params.lambda / 4
    && (
        (params.gamma1 == (1i32 << 17) && params.omega == 80 && K == 4) ||
        (params.gamma1 == (1i32 << 19) && params.omega == 55 && K == 6) ||
        (params.gamma1 == (1i32 << 19) && params.omega == 75 && K == 8)
    )
    && SIG_SIZE >= params.lambda / 4 + L * 32 * (if params.gamma1 == (1i32 << 17) { 18 } else { 20 }) + params.omega + K
)]
pub(crate) fn sig_encode<const K: usize, const L: usize, const SIG_SIZE: usize>(
    c_tilde: &[u8],
    z: &[Polynomial; L],
    h: &[[bool; 256]; K],
    params: &MlDsaParams,
) -> [u8; SIG_SIZE] {
    let gamma1 = params.gamma1 as usize;
    let c_tilde_len = params.lambda / 4;
    let mut sigma = [0u8; SIG_SIZE];
    sigma[..c_tilde_len].copy_from_slice(&c_tilde[..c_tilde_len]);
    let mut offset = c_tilde_len;
    // gamma1=2^17 → bitlen(2^17-1+2^17)=bitlen(2^18-1)=18 → 32*18=576
    // gamma1=2^19 → bitlen(2^19-1+2^19)=bitlen(2^20-1)=20 → 32*20=640
    hax_lib::fstar!("assert_norm(${bitlen} (sz (pow2 17 - 1 + pow2 17)) == sz 18)");
    hax_lib::fstar!("assert_norm(${bitlen} (sz (pow2 19 - 1 + pow2 19)) == sz 20)");
    for i in 0..L {
        hax_lib::loop_invariant!(
            |i: usize| offset == c_tilde_len + i * (bitlen(gamma1 - 1 + gamma1) * 32)
        );
        if gamma1 == (1 << 17) {
            let encoded: [u8; 576] = bit_pack::<576>(&z[i], gamma1 - 1, gamma1);
            sigma[offset..offset + 576].copy_from_slice(&encoded);
            offset += 576;
        } else {
            let encoded: [u8; 640] = bit_pack::<640>(&z[i], gamma1 - 1, gamma1);
            sigma[offset..offset + 640].copy_from_slice(&encoded);
            offset += 640;
        }
    }
    // hint: omega + K bytes
    let hint_bytes = params.omega + K;
    if gamma1 == (1 << 17) {
        // ML-DSA-44: omega=80, K=4 → 84 bytes
        let hint_enc: [u8; 84] = hint_bit_pack::<K, 84>(h, params.omega);
        sigma[offset..offset + hint_bytes].copy_from_slice(&hint_enc[..hint_bytes]);
    } else if params.omega == 55 {
        // ML-DSA-65: omega=55, K=6 → 61 bytes
        let hint_enc: [u8; 61] = hint_bit_pack::<K, 61>(h, params.omega);
        sigma[offset..offset + hint_bytes].copy_from_slice(&hint_enc[..hint_bytes]);
    } else {
        // ML-DSA-87: omega=75, K=8 → 83 bytes
        let hint_enc: [u8; 83] = hint_bit_pack::<K, 83>(h, params.omega);
        sigma[offset..offset + hint_bytes].copy_from_slice(&hint_enc[..hint_bytes]);
    }
    sigma
}

/// sigDecode(σ) — Algorithm 27.
///
/// Returns (c̃, z, h) or None if the hint is malformed.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(
    K <= 8 && L <= 8 && C_TILDE_LEN <= 64
    && sigma.len() >= C_TILDE_LEN + L * 32 * (if params.gamma1 == (1i32 << 17) { 18 } else { 20 }) + params.omega + K
)]
pub(crate) fn sig_decode<const K: usize, const L: usize, const C_TILDE_LEN: usize>(
    sigma: &[u8],
    params: &MlDsaParams,
) -> Option<([u8; C_TILDE_LEN], [Polynomial; L], [[bool; 256]; K])> {
    let gamma1 = params.gamma1 as usize;
    hax_lib::fstar!("assert_norm(${bitlen} (sz (pow2 17 - 1 + pow2 17)) == sz 18)");
    hax_lib::fstar!("assert_norm(${bitlen} (sz (pow2 19 - 1 + pow2 19)) == sz 20)");
    let z_bytes = 32 * bitlen(gamma1 - 1 + gamma1);

    let mut c_tilde = [0u8; C_TILDE_LEN];
    c_tilde.copy_from_slice(&sigma[..C_TILDE_LEN]);
    let z: [Polynomial; L] = createi(|i| {
        let offset = C_TILDE_LEN + i * z_bytes;
        bit_unpack(&sigma[offset..offset + z_bytes], gamma1 - 1, gamma1)
    });
    let hint_offset = C_TILDE_LEN + L * z_bytes;
    let h = hint_bit_unpack::<K>(&sigma[hint_offset..], params.omega)?;
    Some((c_tilde, z, h))
}

/// w1Encode(w1) — Algorithm 28.
///
/// Encodes the high-bits vector w1 into bytes for hashing.
#[hax_lib::fstar::options("--z3rlimit 300")]
#[hax_lib::requires(
    K <= 8 && W1_BYTES >= K * 32 * (if params.gamma2 == (Q - 1) / 88 { 6 } else { 4 })
)]
pub(crate) fn w1_encode<const K: usize, const W1_BYTES: usize>(
    w1: &[Polynomial; K],
    params: &MlDsaParams,
) -> [u8; W1_BYTES] {
    let mut encoded = [0u8; W1_BYTES];
    // gamma2=(Q-1)/88 → w1_max=43, bitlen(43)=6, 32*6=192 bytes/poly
    // gamma2=(Q-1)/32 → w1_max=15, bitlen(15)=4, 32*4=128 bytes/poly
    hax_lib::fstar!("assert_norm(${bitlen} (sz 43) == sz 6)");
    hax_lib::fstar!("assert_norm(${bitlen} (sz 15) == sz 4)");
    let w1_max = ((Q - 1) / (2 * params.gamma2) - 1) as usize;
    let bytes_per_poly = 32 * bitlen(w1_max);
    for i in 0..K {
        hax_lib::loop_invariant!(|i: usize| i <= K);
        if params.gamma2 == (Q - 1) / 88 {
            let packed: [u8; 192] = simple_bit_pack::<192>(&w1[i], w1_max);
            encoded[i * bytes_per_poly..(i + 1) * bytes_per_poly].copy_from_slice(&packed);
        } else {
            let packed: [u8; 128] = simple_bit_pack::<128>(&w1[i], w1_max);
            encoded[i * bytes_per_poly..(i + 1) * bytes_per_poly].copy_from_slice(&packed);
        }
    }
    encoded
}

// ========================================================================
// CoeffFromThreeBytes, CoeffFromHalfByte — Algorithms 14–15
// ========================================================================

/// CoeffFromThreeBytes(b0, b1, b2) — Algorithm 14.
///
/// Returns an element of {0, 1, ..., q-1} or None (rejection).
pub(crate) fn coeff_from_three_bytes(b0: u8, b1: u8, b2: u8) -> Option<i32> {
    let b2p = if b2 > 127 { b2 - 128 } else { b2 };
    let z = ((b2p as i32) << 16) | ((b1 as i32) << 8) | (b0 as i32);
    if z < Q {
        Some(z)
    } else {
        None
    }
}

/// CoeffFromHalfByte(b, η) — Algorithm 15.
///
/// Returns an element of {-η, ..., η} or None (rejection).
pub(crate) fn coeff_from_half_byte(b: u8, eta: usize) -> Option<i32> {
    if eta == 2 && (b as usize) < 15 {
        Some(2 - (b % 5) as i32)
    } else if eta == 4 && (b as usize) < 9 {
        Some(4 - b as i32)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_bit_pack_unpack_roundtrip() {
        let mut w = ZERO_POLY;
        w[0] = 1023; // max for 10-bit
        w[1] = 0;
        w[255] = 512;
        let packed: [u8; 320] = simple_bit_pack::<320>(&w, 1023);
        let unpacked = simple_bit_unpack(&packed, 1023);
        for i in 0..256 {
            assert_eq!(w[i], unpacked[i], "mismatch at {i}");
        }
    }

    #[test]
    fn bit_pack_unpack_roundtrip() {
        let mut w = ZERO_POLY;
        w[0] = 2;
        w[1] = -2;
        w[2] = 0;
        w[3] = 1;
        let packed: [u8; 96] = bit_pack::<96>(&w, 2, 2);
        let unpacked = bit_unpack(&packed, 2, 2);
        for i in 0..256 {
            assert_eq!(w[i], unpacked[i], "mismatch at {i}");
        }
    }

    #[test]
    fn coeff_from_three_bytes_basic() {
        assert_eq!(coeff_from_three_bytes(0, 0, 0), Some(0));
        assert_eq!(coeff_from_three_bytes(1, 0, 0), Some(1));
        assert_eq!(coeff_from_three_bytes(0x00, 0xE0, 0x7F), Some(Q - 1));
        assert_eq!(coeff_from_three_bytes(0, 0, 128), Some(0));
        assert_eq!(coeff_from_three_bytes(0x01, 0xE0, 0x7F), None);
    }

    #[test]
    fn coeff_from_half_byte_eta2() {
        assert_eq!(coeff_from_half_byte(0, 2), Some(2));
        assert_eq!(coeff_from_half_byte(4, 2), Some(-2));
        assert_eq!(coeff_from_half_byte(14, 2), Some(2 - 14 % 5));
        assert_eq!(coeff_from_half_byte(15, 2), None);
    }
}

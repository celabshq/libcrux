// This file adapted from https://github.com/AU-COBRA/AUCurves/blob/d4159c1b8a91f2f832761c72f00caa8fb59ec472/safegcd-rs/src/safegcd_p384.rs
//! Bernstein-Yang divstep modular inversion mod NIST P-384
//! = 2^384 − 2^128 − 2^96 + 2^32 − 1.
//!
//! Uses 7×62-bit signed limbs (6 u64 input).  Iteration count: 15 outer
//! × 59 = 885 divsteps (paper bound for 384-bit prime: 15 outer iters).
//!
//! KAT via `x · x^{-1} ≡ 1 (mod p)` using schoolbook mul-mod.

use crate::field::safegcd::{self as sg, ModInfo, Signed62};

/// P-384 prime in 6×u64 little-endian.
pub(super) const P384_P_6X64: [u64; 6] = [
    0x0000_0000_FFFF_FFFF,
    0xFFFF_FFFF_0000_0000,
    0xFFFF_FFFF_FFFF_FFFE,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
];

pub(super) const P384: ModInfo<7> = sg::modinfo_from_6u64(P384_P_6X64, 15);

/// `x^{-1} mod p384` on 6×u64 saturated input.
#[inline(never)]
pub(super) fn p384_invert_divstep_sat(out: &mut [u64; 6], x: &[u64; 6]) {
    let x_lim: Signed62<7> = sg::from_saturated_6(x);
    let inv = sg::invert(x_lim, &P384);
    *out = sg::to_saturated_6(&inv);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mul_mod_p_6(a: &[u64; 6], b: &[u64; 6], p: &[u64; 6]) -> [u64; 6] {
        let mut prod = [0u64; 12];
        for i in 0..6 {
            let mut carry: u128 = 0;
            for j in 0..6 {
                let v = (a[i] as u128) * (b[j] as u128) + (prod[i + j] as u128) + carry;
                prod[i + j] = v as u64;
                carry = v >> 64;
            }
            prod[i + 6] = carry as u64;
        }
        const N: usize = 12;
        let mut p_ext = [0u64; N];
        p_ext[..6].copy_from_slice(p);
        let mut rem = [0u64; N];
        for bit in (0..(N * 64)).rev() {
            let mut carry: u64 = 0;
            for limb in rem.iter_mut() {
                let new_carry = *limb >> 63;
                *limb = (*limb << 1) | carry;
                carry = new_carry;
            }
            let limb = bit / 64;
            let lbit = bit % 64;
            rem[0] |= (prod[limb] >> lbit) & 1;
            let mut ge = true;
            for k in (0..N).rev() {
                if rem[k] > p_ext[k] {
                    ge = true;
                    break;
                }
                if rem[k] < p_ext[k] {
                    ge = false;
                    break;
                }
            }
            if ge {
                let mut borrow: i128 = 0;
                for k in 0..N {
                    let v = (rem[k] as i128) - (p_ext[k] as i128) - borrow;
                    rem[k] = v as u64;
                    borrow = if v < 0 { 1 } else { 0 };
                }
            }
        }
        let mut out = [0u64; 6];
        out.copy_from_slice(&rem[..6]);
        out
    }

    fn is_zero(x: &[u64; 6]) -> bool {
        x.iter().all(|&v| v == 0)
    }

    #[test]
    fn p384_invert_times_self_is_one() {
        let inputs: [[u64; 6]; 4] = [
            [1, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 0],
            [
                0x0123_4567_89ab_cdef,
                0xfedc_ba98_7654_3210,
                0xdead_beef,
                0x1,
                0x0,
                0x0,
            ],
            // p - 1
            [
                P384_P_6X64[0] - 1,
                P384_P_6X64[1],
                P384_P_6X64[2],
                P384_P_6X64[3],
                P384_P_6X64[4],
                P384_P_6X64[5],
            ],
        ];
        for (i, x) in inputs.iter().enumerate() {
            let mut xi = [0u64; 6];
            p384_invert_divstep_sat(&mut xi, x);
            assert!(!is_zero(&xi), "P-384 invert(x) = 0 for input {}", i);
            let prod = mul_mod_p_6(x, &xi, &P384_P_6X64);
            let one = [1u64, 0, 0, 0, 0, 0];
            assert_eq!(prod, one, "P-384: x · x^-1 != 1 for input {}", i);
        }
    }
}

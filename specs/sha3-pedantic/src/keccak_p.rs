//! `KECCAK-p[b, n_r]` and `KECCAK-f[b]` — FIPS 202, Sec. 3.3 and 3.4.

use crate::bits::{Bit, BitString};
use crate::state_array::StateArray;
use crate::step_mappings::{chi, iota, pi, rho, theta};

/// Sec. 3.3: `Rnd(A, i_r) = ι(χ(π(ρ(θ(A)))), i_r)`.
pub fn rnd<const W: usize>(a: &StateArray<W>, i_r: i64) -> StateArray<W> {
    iota(&chi(&pi(&rho(&theta(a)))), i_r)
}

/// Algorithm 7: `KECCAK-p[b, n_r](S)`.
///
/// `b` is fixed by `W` (`b = 25W`); `n_r` is the number of rounds.
pub fn keccak_p<const W: usize>(s: &[Bit], n_r: usize) -> BitString {
    // Sec. 3: the permutation "is defined for any b in {25, 50, 100, 200, 400,
    // 800, 1600} and any positive integer n_r"; b is fixed by W, and Table 1 is
    // checked by `StateArray::<W>::L`.
    assert!(n_r > 0, "Algorithm 7 takes a positive number of rounds");
    // 1. Convert S into a state array, A (Sec. 3.1.2).
    let mut a = StateArray::<W>::from_bits(s);

    // 2. For i_r from 12 + 2l - n_r to 12 + 2l - 1, let A = Rnd(A, i_r).
    let last = 12 + 2 * StateArray::<W>::L as i64 - 1;
    let first = last - n_r as i64 + 1;
    for i_r in first..=last {
        a = rnd(&a, i_r);
    }

    // 3. Convert A into a string S′ of length b (Sec. 3.1.3).
    // 4. Return S′.
    a.to_bits()
}

/// Sec. 3.4: `KECCAK-f[b] = KECCAK-p[b, 12 + 2l]`.
pub fn keccak_f<const W: usize>(s: &[Bit]) -> BitString {
    keccak_p::<W>(s, 12 + 2 * StateArray::<W>::L)
}

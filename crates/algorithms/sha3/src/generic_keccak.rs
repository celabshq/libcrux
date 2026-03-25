//! The generic SHA3 implementation that uses portable or platform specific
//! sub-routines.

use core::ops::Index;

use crate::traits::*;

#[cfg(hax)]
use crate::proof_utils::{slices_same_len, valid_rate};

#[cfg(hax)]
use hax_lib::{constructors::from_bool, int::ToInt};

/// A generic Xof API.
pub(crate) mod xof;

/// Constants in SHA3.
mod constants;
use constants::*;

/// Simd128 specific implementations.
///
/// We need to work around hax limitations and therefore need to implement `squeeze`
/// separately for each platform.
#[cfg(feature = "simd128")]
pub(crate) mod simd128;

/// Simd256 specific implementations.
#[cfg(feature = "simd256")]
pub(crate) mod simd256;

/// Portable specific implementations.
pub(crate) mod portable;

#[derive(Copy, Clone)]
pub(crate) struct KeccakState<const N: usize, T: KeccakItem<N>> {
    pub(crate) st: [T; 25],
}

#[hax_lib::attributes]
impl<const N: usize, T: KeccakItem<N>> KeccakState<N, T> {
    /// Create a new Shake128 x4 state.
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self {
            st: [T::zero(); 25],
        }
    }

    /// Set element `[i, j] = v`.
    #[hax_lib::requires(i < 5 && j < 5)]
    fn set(&mut self, i: usize, j: usize, v: T) {
        set_ij(&mut self.st, i, j, v);
    }

    #[inline(always)]
    fn theta(&mut self) -> [T; 5] {
        let c: [T; 5] = [
            T::xor5(
                self[(0, 0)],
                self[(1, 0)],
                self[(2, 0)],
                self[(3, 0)],
                self[(4, 0)],
            ),
            T::xor5(
                self[(0, 1)],
                self[(1, 1)],
                self[(2, 1)],
                self[(3, 1)],
                self[(4, 1)],
            ),
            T::xor5(
                self[(0, 2)],
                self[(1, 2)],
                self[(2, 2)],
                self[(3, 2)],
                self[(4, 2)],
            ),
            T::xor5(
                self[(0, 3)],
                self[(1, 3)],
                self[(2, 3)],
                self[(3, 3)],
                self[(4, 3)],
            ),
            T::xor5(
                self[(0, 4)],
                self[(1, 4)],
                self[(2, 4)],
                self[(3, 4)],
                self[(4, 4)],
            ),
        ];
        #[allow(clippy::identity_op)]
        [
            T::rotate_left1_and_xor(c[(0 + 4) % 5], c[(0 + 1) % 5]),
            T::rotate_left1_and_xor(c[(1 + 4) % 5], c[(1 + 1) % 5]),
            T::rotate_left1_and_xor(c[(2 + 4) % 5], c[(2 + 1) % 5]),
            T::rotate_left1_and_xor(c[(3 + 4) % 5], c[(3 + 1) % 5]),
            T::rotate_left1_and_xor(c[(4 + 4) % 5], c[(4 + 1) % 5]),
        ]
    }

    // Splitting rho and pi into multiple functions is needed for formal verification in F*.
    // Having to many manipulations of the keccak state in a single function causes the
    // verification condition explode exponentially and never completes.
    #[inline(always)]
    fn rho_0(&mut self, t: [T; 5]) {
        self.set(0, 0, T::xor(self[(0, 0)], t[0]));
        self.set(1, 0, T::xor_and_rotate::<36, 28>(self[(1, 0)], t[0]));
        self.set(2, 0, T::xor_and_rotate::<3, 61>(self[(2, 0)], t[0]));
        self.set(3, 0, T::xor_and_rotate::<41, 23>(self[(3, 0)], t[0]));
        self.set(4, 0, T::xor_and_rotate::<18, 46>(self[(4, 0)], t[0]));
    }

    #[inline(always)]
    fn rho_1(&mut self, t: [T; 5]) {
        self.set(0, 1, T::xor_and_rotate::<1, 63>(self[(0, 1)], t[1]));
        self.set(1, 1, T::xor_and_rotate::<44, 20>(self[(1, 1)], t[1]));
        self.set(2, 1, T::xor_and_rotate::<10, 54>(self[(2, 1)], t[1]));
        self.set(3, 1, T::xor_and_rotate::<45, 19>(self[(3, 1)], t[1]));
        self.set(4, 1, T::xor_and_rotate::<2, 62>(self[(4, 1)], t[1]));
    }

    #[inline(always)]
    fn rho_2(&mut self, t: [T; 5]) {
        self.set(0, 2, T::xor_and_rotate::<62, 2>(self[(0, 2)], t[2]));
        self.set(1, 2, T::xor_and_rotate::<6, 58>(self[(1, 2)], t[2]));
        self.set(2, 2, T::xor_and_rotate::<43, 21>(self[(2, 2)], t[2]));
        self.set(3, 2, T::xor_and_rotate::<15, 49>(self[(3, 2)], t[2]));
        self.set(4, 2, T::xor_and_rotate::<61, 3>(self[(4, 2)], t[2]));
    }

    #[inline(always)]
    fn rho_3(&mut self, t: [T; 5]) {
        self.set(0, 3, T::xor_and_rotate::<28, 36>(self[(0, 3)], t[3]));
        self.set(1, 3, T::xor_and_rotate::<55, 9>(self[(1, 3)], t[3]));
        self.set(2, 3, T::xor_and_rotate::<25, 39>(self[(2, 3)], t[3]));
        self.set(3, 3, T::xor_and_rotate::<21, 43>(self[(3, 3)], t[3]));
        self.set(4, 3, T::xor_and_rotate::<56, 8>(self[(4, 3)], t[3]));
    }

    #[inline(always)]
    fn rho_4(&mut self, t: [T; 5]) {
        self.set(0, 4, T::xor_and_rotate::<27, 37>(self[(0, 4)], t[4]));
        self.set(1, 4, T::xor_and_rotate::<20, 44>(self[(1, 4)], t[4]));
        self.set(2, 4, T::xor_and_rotate::<39, 25>(self[(2, 4)], t[4]));
        self.set(3, 4, T::xor_and_rotate::<8, 56>(self[(3, 4)], t[4]));
        self.set(4, 4, T::xor_and_rotate::<14, 50>(self[(4, 4)], t[4]));
    }

    #[inline(always)]
    fn rho(&mut self, t: [T; 5]) {
        self.rho_0(t);
        self.rho_1(t);
        self.rho_2(t);
        self.rho_3(t);
        self.rho_4(t);
    }

    #[inline(always)]
    fn pi_0(&mut self, old: KeccakState<N, T>) {
        self.set(1, 0, old[(0, 3)]);
        self.set(2, 0, old[(0, 1)]);
        self.set(3, 0, old[(0, 4)]);
        self.set(4, 0, old[(0, 2)]);
    }

    #[inline(always)]
    fn pi_1(&mut self, old: KeccakState<N, T>) {
        self.set(0, 1, old[(1, 1)]);
        self.set(1, 1, old[(1, 4)]);
        self.set(2, 1, old[(1, 2)]);
        self.set(3, 1, old[(1, 0)]);
        self.set(4, 1, old[(1, 3)]);
    }

    #[inline(always)]
    fn pi_2(&mut self, old: KeccakState<N, T>) {
        self.set(0, 2, old[(2, 2)]);
        self.set(1, 2, old[(2, 0)]);
        self.set(2, 2, old[(2, 3)]);
        self.set(3, 2, old[(2, 1)]);
        self.set(4, 2, old[(2, 4)]);
    }

    #[inline(always)]
    fn pi_3(&mut self, old: KeccakState<N, T>) {
        self.set(0, 3, old[(3, 3)]);
        self.set(1, 3, old[(3, 1)]);
        self.set(2, 3, old[(3, 4)]);
        self.set(3, 3, old[(3, 2)]);
        self.set(4, 3, old[(3, 0)]);
    }

    #[inline(always)]
    fn pi_4(&mut self, old: KeccakState<N, T>) {
        self.set(0, 4, old[(4, 4)]);
        self.set(1, 4, old[(4, 2)]);
        self.set(2, 4, old[(4, 0)]);
        self.set(3, 4, old[(4, 3)]);
        self.set(4, 4, old[(4, 1)]);
    }

    #[inline(always)]
    fn pi(&mut self) {
        let old: KeccakState<N, T> = *self;

        self.pi_0(old);
        self.pi_1(old);
        self.pi_2(old);
        self.pi_3(old);
        self.pi_4(old);
    }

    #[inline(always)]
    fn chi(&mut self) {
        let old = *self;

        #[allow(clippy::needless_range_loop)]
        for i in 0..5 {
            for j in 0..5 {
                self.set(
                    i,
                    j,
                    T::and_not_xor(self[(i, j)], old[(i, (j + 2) % 5)], old[(i, (j + 1) % 5)]),
                );
            }
        }
    }

    #[inline(always)]
    #[hax_lib::requires(i < ROUNDCONSTANTS.len())]
    fn iota(&mut self, i: usize) {
        self.set(0, 0, T::xor_constant(self[(0, 0)], ROUNDCONSTANTS[i]));
    }

    #[inline(always)]
    fn keccakf1600(&mut self) {
        for i in 0..24 {
            let t = self.theta();
            self.rho(t);
            self.pi();
            self.chi();
            self.iota(i);
        }
    }

    #[inline(always)]
    #[hax_lib::requires(
      from_bool(
        N != 0 &&
        valid_rate(RATE) &&
        start.to_int() + RATE.to_int() <= input[0].len().to_int()
      ).and(
        slices_same_len(input)
      )
    )]
    fn absorb_block<const RATE: usize>(&mut self, input: &[&[u8]; N], start: usize)
    where
        Self: Absorb<N>,
    {
        #[cfg(not(any(eurydice, hax)))]
        debug_assert!(input.iter().all(|buf| buf.len() == input[0].len()));

        self.load_block::<RATE>(input, start);
        self.keccakf1600()
    }

    #[inline(always)]
    #[hax_lib::requires(
      from_bool(
        N != 0 &&
        valid_rate(RATE) &&
        len < RATE &&
        start.to_int() + len.to_int() <= input[0].len().to_int()
      ).and(
        slices_same_len(input)
      )
    )]
    pub(crate) fn absorb_final<const RATE: usize, const DELIM: u8>(
        &mut self,
        input: &[&[u8]; N],
        start: usize,
        len: usize,
    ) where
        Self: Absorb<N>,
    {
        #[cfg(not(eurydice))]
        debug_assert!(N > 0 && len < RATE);

        #[cfg(not(any(eurydice, hax)))]
        debug_assert!(input.iter().all(|buf| buf.len() == input[0].len()));

        self.load_last::<RATE, DELIM>(input, start, len);
        self.keccakf1600()
    }
}

#[hax_lib::attributes]
impl<const N: usize, T: KeccakItem<N>> Index<(usize, usize)> for KeccakState<N, T> {
    type Output = T;

    /// Get element `[i, j]`.
    #[hax_lib::requires(index.0 < 5 && index.1 < 5)]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        get_ij(&self.st, index.0, index.1)
    }
}

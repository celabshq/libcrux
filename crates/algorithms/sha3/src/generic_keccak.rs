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
pub(crate) mod constants;
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

/// Cross-spec tests: compare each permutation step against the hacspec spec.
#[cfg(test)]
mod cross_spec_tests {
    use super::*;
    use hacspec_sha3::keccak_f as spec_kf;
    use hacspec_sha3::sponge as spec_sponge;

    fn wrap(st: [u64; 25]) -> KeccakState<1, u64> {
        KeccakState { st }
    }

    /// Non-trivial test state: keccak-f applied to state where lane 0 = 1.
    fn test_state() -> [u64; 25] {
        let mut st = [0u64; 25];
        st[0] = 1;
        spec_kf::keccak_f(st)
    }

    // -- Layer 1: Constants --

    #[test]
    fn round_constants_match() {
        assert_eq!(ROUNDCONSTANTS, spec_kf::ROUND_CONSTANTS);
    }

    // -- Layer 2: Permutation steps --

    #[test]
    fn theta_rho_matches_spec() {
        for &st in &[[0u64; 25], test_state()] {
            let mut s = wrap(st);
            let t = s.theta();
            s.rho(t);
            assert_eq!(s.st, spec_kf::rho(spec_kf::theta(st)));
        }
    }

    #[test]
    fn pi_matches_spec() {
        let st = test_state();
        let mut s = wrap(st);
        s.pi();
        assert_eq!(s.st, spec_kf::pi(st));
    }

    #[test]
    fn chi_matches_spec() {
        let st = test_state();
        let mut s = wrap(st);
        s.chi();
        assert_eq!(s.st, spec_kf::chi(st));
    }

    #[test]
    fn iota_matches_spec() {
        for round in 0..24 {
            let st = test_state();
            let mut s = wrap(st);
            s.iota(round);
            assert_eq!(s.st, spec_kf::iota(st, round));
        }
    }

    #[test]
    fn keccak_f_matches_spec() {
        for &st in &[[0u64; 25], test_state()] {
            let mut s = wrap(st);
            s.keccakf1600();
            assert_eq!(s.st, spec_kf::keccak_f(st));
        }
    }

    #[test]
    fn single_round_matches_spec() {
        let st = test_state();
        let spec = spec_kf::iota(
            spec_kf::chi(spec_kf::pi(spec_kf::rho(spec_kf::theta(st)))),
            0,
        );
        let mut s = wrap(st);
        let t = s.theta();
        s.rho(t);
        s.pi();
        s.chi();
        s.iota(0);
        assert_eq!(s.st, spec);
    }

    // -- Layer 3: Sponge helpers --

    #[test]
    fn load_block_matches_spec() {
        let block = [0xABu8; 200];
        let mut impl_st = [0u64; 25];
        crate::simd::portable::load_block::<136>(&mut impl_st, &block, 0);
        let spec_st = spec_sponge::xor_block_into_state([0u64; 25], &block, 136);
        assert_eq!(impl_st, spec_st);
    }

    #[test]
    fn load_block_with_offset() {
        let mut data = [0u8; 400];
        for (i, b) in data.iter_mut().enumerate() {
            *b = (i & 0xFF) as u8;
        }
        let mut impl_st = [0u64; 25];
        crate::simd::portable::load_block::<136>(&mut impl_st, &data, 136);
        let spec_st = spec_sponge::xor_block_into_state([0u64; 25], &data[136..272], 136);
        assert_eq!(impl_st, spec_st);
    }

    #[test]
    fn store_block_matches_spec() {
        let state = test_state();
        let mut impl_out = [0u8; 200];
        crate::simd::portable::store_block::<136>(&state, &mut impl_out, 0, 136);
        let mut spec_out = [0u8; 200];
        spec_out = spec_sponge::squeeze_state(&state, spec_out, 0, 136);
        assert_eq!(impl_out[..136], spec_out[..136]);
    }

    #[test]
    fn load_last_matches_spec_padding() {
        // Test various last-block sizes for SHA3-256 (rate=136, delim=0x06)
        let rate = 136usize;
        for len in [0, 1, 7, 8, 9, 15, 16, 17, 64, 100, 135] {
            let mut msg = [0u8; 135];
            for i in 0..len {
                msg[i] = (i & 0xFF) as u8;
            }
            let mut impl_st = [0u64; 25];
            crate::simd::portable::load_last::<136, 0x06>(&mut impl_st, &msg[..len], 0, len);

            let mut last_block = [0u8; 200];
            last_block[..len].copy_from_slice(&msg[..len]);
            last_block[len] = 0x06;
            last_block[rate - 1] |= 0x80;
            let spec_st = spec_sponge::xor_block_into_state([0u64; 25], &last_block, rate);
            assert_eq!(impl_st, spec_st, "load_last mismatch at len={len}");
        }
    }

    #[test]
    fn load_block_all_rates() {
        // Test load_block for every rate used by SHA3/SHAKE variants
        let data = [0xCDu8; 200];
        macro_rules! test_rate {
            ($rate:expr) => {
                let mut impl_st = [0u64; 25];
                crate::simd::portable::load_block::<$rate>(&mut impl_st, &data, 0);
                let spec_st = spec_sponge::xor_block_into_state([0u64; 25], &data, $rate);
                assert_eq!(impl_st, spec_st, "load_block mismatch at rate={}", $rate);
            };
        }
        test_rate!(72); // SHA3-512
        test_rate!(104); // SHA3-384
        test_rate!(136); // SHA3-256 / SHAKE256
        test_rate!(144); // SHA3-224
        test_rate!(168); // SHAKE128
    }

    #[test]
    fn store_block_partial_len() {
        // Test squeeze with various lengths (not just the full rate)
        let state = test_state();
        for &len in &[8, 16, 64, 72, 104, 136] {
            let mut impl_out = [0u8; 200];
            crate::simd::portable::store_block::<136>(&state, &mut impl_out, 0, len);
            let mut spec_out = [0u8; 200];
            spec_out = spec_sponge::squeeze_state(&state, spec_out, 0, len);
            assert_eq!(
                impl_out[..len],
                spec_out[..len],
                "store_block mismatch at len={len}"
            );
        }
    }

    #[test]
    fn store_block_with_offset() {
        let state = test_state();
        let len = 72;
        let offset = 64;
        let mut impl_out = [0u8; 200];
        crate::simd::portable::store_block::<136>(&state, &mut impl_out, offset, len);
        let mut spec_out = [0u8; 200];
        spec_out = spec_sponge::squeeze_state(&state, spec_out, offset, len);
        assert_eq!(
            impl_out[offset..offset + len],
            spec_out[offset..offset + len]
        );
    }

    // -- Layer 4: Top-level absorb / squeeze split --
    //
    // These tests pin down the refactor of `keccak1` into
    // `absorb` + `squeeze`: each phase is compared against the spec
    // independently.  They also confirm that the two-line
    // `keccak1 = squeeze(absorb(...))` matches the monolithic spec
    // `keccak` for every SHA-3 / SHAKE variant.

    #[test]
    fn absorb_matches_spec() {
        // For each rate/delim, compare impl absorb's state against the
        // spec absorb's state on a few message sizes that exercise
        // zero, partial, and multi-block absorb paths.
        fn check<const RATE: usize, const DELIM: u8>(msgs: &[&[u8]]) {
            for msg in msgs {
                let impl_s: KeccakState<1, u64> =
                    crate::generic_keccak::portable::absorb::<RATE, DELIM>(msg);
                let spec_s = spec_sponge::absorb(RATE, DELIM, msg);
                assert_eq!(
                    impl_s.st,
                    spec_s,
                    "absorb mismatch at RATE={RATE}, DELIM={DELIM:#x}, len={}",
                    msg.len()
                );
            }
        }

        let empty: &[u8] = &[];
        let short: &[u8] = b"hello world";
        let mut rate_minus_one = [0u8; 135];
        for (i, b) in rate_minus_one.iter_mut().enumerate() {
            *b = (i & 0xFF) as u8;
        }
        let mut multi_block = [0u8; 200];
        for (i, b) in multi_block.iter_mut().enumerate() {
            *b = (i & 0xFF) as u8;
        }
        let msgs: &[&[u8]] = &[empty, short, &rate_minus_one, &multi_block];

        check::<144, 0x06>(msgs); // SHA3-224
        check::<136, 0x06>(msgs); // SHA3-256
        check::<104, 0x06>(msgs); // SHA3-384
        check::<72, 0x06>(msgs); // SHA3-512
        check::<168, 0x1f>(msgs); // SHAKE128
        check::<136, 0x1f>(msgs); // SHAKE256
    }

    #[test]
    fn squeeze_matches_spec() {
        // Start from a state produced by impl absorb (which we just
        // showed matches the spec) and confirm that squeeze produces
        // the same bytes as the spec squeeze for output lengths that
        // exercise every branch (zero blocks, exact blocks, tail).
        fn check<const RATE: usize, const DELIM: u8, const OUT: usize>(msg: &[u8]) {
            let impl_s = crate::generic_keccak::portable::absorb::<RATE, DELIM>(msg);
            let spec_s = spec_sponge::absorb(RATE, DELIM, msg);
            debug_assert_eq!(impl_s.st, spec_s);

            let mut impl_out = [0u8; OUT];
            crate::generic_keccak::portable::squeeze::<RATE>(impl_s, &mut impl_out);
            let spec_out = spec_sponge::squeeze::<OUT>(spec_s, RATE);
            assert_eq!(
                impl_out,
                spec_out,
                "squeeze mismatch at RATE={RATE}, DELIM={DELIM:#x}, OUT={OUT}, msg.len()={}",
                msg.len()
            );
        }

        let msg: &[u8] = b"the quick brown fox jumps over the lazy dog";

        // Hash variants: OUT < RATE (single-squeeze path)
        check::<144, 0x06, 28>(msg); // SHA3-224
        check::<136, 0x06, 32>(msg); // SHA3-256
        check::<104, 0x06, 48>(msg); // SHA3-384
        check::<72, 0x06, 64>(msg); // SHA3-512
                                    // SHAKE: OUT exercising loop and tail
        check::<168, 0x1f, 16>(msg); // short shake128
        check::<168, 0x1f, 200>(msg); // multi-block shake128 (loop + tail)
        check::<136, 0x1f, 64>(msg); // short shake256
        check::<136, 0x1f, 300>(msg); // multi-block shake256 (loop + tail)
    }

    #[test]
    fn keccak1_matches_squeeze_of_absorb() {
        // `keccak1` must be observationally equal to
        // `squeeze(absorb(...))`, both on the impl side.  This pins
        // the two-line rewrite of `keccak1`.
        fn check<const RATE: usize, const DELIM: u8, const OUT: usize>(msg: &[u8]) {
            let mut via_keccak1 = [0u8; OUT];
            crate::generic_keccak::portable::keccak1::<RATE, DELIM>(msg, &mut via_keccak1);

            let s = crate::generic_keccak::portable::absorb::<RATE, DELIM>(msg);
            let mut via_split = [0u8; OUT];
            crate::generic_keccak::portable::squeeze::<RATE>(s, &mut via_split);

            assert_eq!(
                via_keccak1, via_split,
                "keccak1 != squeeze(absorb) at RATE={RATE}, DELIM={DELIM:#x}, OUT={OUT}",
            );
        }

        let msg: &[u8] = b"refactor sanity check";
        check::<136, 0x06, 32>(msg); // SHA3-256
        check::<168, 0x1f, 64>(msg); // SHAKE128
        check::<168, 0x1f, 200>(msg); // SHAKE128, multi-block + tail
    }
}

/// NEON to_spec tests: verify that each permutation step on KeccakState<2, uint64x2_t>
/// operates lane-wise, i.e. extracting lane l from the SIMD result equals the scalar
/// spec step applied to lane l of the input.  This validates the `to_spec` commutativity
/// property that the F* generalization proof is built on.
#[cfg(all(test, feature = "simd128"))]
mod neon_to_spec_tests {
    use super::*;
    use hacspec_sha3::keccak_f as spec_kf;
    use libcrux_intrinsics::arm64::*;

    /// Extract lane `l` (0 or 1) from a KeccakState<2, uint64x2_t> → [u64; 25]
    fn extract_lane(state: &KeccakState<2, _uint64x2_t>, lane: usize) -> [u64; 25] {
        assert!(lane < 2);
        let mut out = [0u64; 25];
        for i in 0..25 {
            let mut tmp = [0u64; 2];
            _vst1q_u64(&mut tmp, state.st[i]);
            out[i] = tmp[lane];
        }
        out
    }

    /// to_spec: KeccakState<2, uint64x2_t> → [[u64; 25]; 2]
    fn to_spec(state: &KeccakState<2, _uint64x2_t>) -> [[u64; 25]; 2] {
        [extract_lane(state, 0), extract_lane(state, 1)]
    }

    /// Pack two scalar [u64; 25] states into a KeccakState<2, uint64x2_t>
    fn from_spec(lanes: [[u64; 25]; 2]) -> KeccakState<2, _uint64x2_t> {
        let mut st = [_vdupq_n_u64(0); 25];
        for i in 0..25 {
            let arr = [lanes[0][i], lanes[1][i]];
            st[i] = _vld1q_u64(&arr);
        }
        KeccakState { st }
    }

    /// Two distinct non-trivial test states for packing into lanes.
    fn test_states() -> [[u64; 25]; 2] {
        let mut st0 = [0u64; 25];
        st0[0] = 1;
        let st0 = spec_kf::keccak_f(st0);

        let mut st1 = [0u64; 25];
        st1[0] = 0xDEAD_BEEF_CAFE_BABEu64;
        let st1 = spec_kf::keccak_f(st1);

        [st0, st1]
    }

    #[test]
    fn to_spec_roundtrip() {
        let lanes = test_states();
        let packed = from_spec(lanes);
        let extracted = to_spec(&packed);
        assert_eq!(extracted, lanes, "to_spec(from_spec(x)) != x");
    }

    #[test]
    fn neon_theta_rho_to_spec() {
        let lanes = test_states();
        let mut s = from_spec(lanes);
        let t = s.theta();
        s.rho(t);
        let result = to_spec(&s);
        for l in 0..2 {
            let spec = spec_kf::rho(spec_kf::theta(lanes[l]));
            assert_eq!(result[l], spec, "theta+rho mismatch on lane {l}");
        }
    }

    #[test]
    fn neon_pi_to_spec() {
        let lanes = test_states();
        let mut s = from_spec(lanes);
        s.pi();
        let result = to_spec(&s);
        for l in 0..2 {
            let spec = spec_kf::pi(lanes[l]);
            assert_eq!(result[l], spec, "pi mismatch on lane {l}");
        }
    }

    #[test]
    fn neon_chi_to_spec() {
        let lanes = test_states();
        let mut s = from_spec(lanes);
        s.chi();
        let result = to_spec(&s);
        for l in 0..2 {
            let spec = spec_kf::chi(lanes[l]);
            assert_eq!(result[l], spec, "chi mismatch on lane {l}");
        }
    }

    #[test]
    fn neon_iota_to_spec() {
        for round in 0..24 {
            let lanes = test_states();
            let mut s = from_spec(lanes);
            s.iota(round);
            let result = to_spec(&s);
            for l in 0..2 {
                let spec = spec_kf::iota(lanes[l], round);
                assert_eq!(result[l], spec, "iota mismatch on lane {l}, round {round}");
            }
        }
    }

    #[test]
    fn neon_single_round_to_spec() {
        let lanes = test_states();
        let mut s = from_spec(lanes);
        let t = s.theta();
        s.rho(t);
        s.pi();
        s.chi();
        s.iota(0);
        let result = to_spec(&s);
        for l in 0..2 {
            let spec = spec_kf::iota(
                spec_kf::chi(spec_kf::pi(spec_kf::rho(spec_kf::theta(lanes[l])))),
                0,
            );
            assert_eq!(result[l], spec, "single round mismatch on lane {l}");
        }
    }

    #[test]
    fn neon_keccakf1600_to_spec() {
        let lanes = test_states();
        let mut s = from_spec(lanes);
        s.keccakf1600();
        let result = to_spec(&s);
        for l in 0..2 {
            let spec = spec_kf::keccak_f(lanes[l]);
            assert_eq!(result[l], spec, "keccakf1600 mismatch on lane {l}");
        }
    }

    #[test]
    fn neon_keccakf1600_zero_state() {
        let lanes = [[0u64; 25]; 2];
        let mut s = from_spec(lanes);
        s.keccakf1600();
        let result = to_spec(&s);
        let spec = spec_kf::keccak_f([0u64; 25]);
        for l in 0..2 {
            assert_eq!(
                result[l], spec,
                "keccakf1600 zero-state mismatch on lane {l}"
            );
        }
    }

    #[test]
    fn neon_keccakf1600_iterated() {
        // Apply keccakf1600 multiple times; verify lanes stay independent
        let mut lanes = test_states();
        let mut s = from_spec(lanes);
        for _ in 0..5 {
            s.keccakf1600();
            lanes[0] = spec_kf::keccak_f(lanes[0]);
            lanes[1] = spec_kf::keccak_f(lanes[1]);
            let result = to_spec(&s);
            assert_eq!(result[0], lanes[0], "iterated keccakf1600 lane 0 diverged");
            assert_eq!(result[1], lanes[1], "iterated keccakf1600 lane 1 diverged");
        }
    }
}

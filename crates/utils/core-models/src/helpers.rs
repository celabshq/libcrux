#[cfg(test)]
pub mod test {
    use crate::abstractions::{bit::Bit, bitvec::BitVec, funarr::FunArray};
    use rand::prelude::*;

    /// Helper trait to generate random values
    pub trait HasRandom {
        fn random() -> Self;
    }
    macro_rules! mk_has_random {
        ($($ty:ty),*) => {
            $(impl HasRandom for $ty {
                fn random() -> Self {
                    let mut rng = rand::rng();
                    rng.random()
                }
            })*
        };
    }

    mk_has_random!(bool);
    mk_has_random!(i8, i16, i32, i64, i128);
    mk_has_random!(u8, u16, u32, u64, u128);

    impl HasRandom for isize {
        fn random() -> Self {
            i128::random() as isize
        }
    }
    impl HasRandom for usize {
        fn random() -> Self {
            i128::random() as usize
        }
    }

    impl HasRandom for Bit {
        fn random() -> Self {
            crate::abstractions::bit::Bit::from(bool::random())
        }
    }
    impl<const N: u64> HasRandom for BitVec<N> {
        fn random() -> Self {
            Self::from_fn(|_| Bit::random())
        }
    }

    impl<const N: u64, T: HasRandom> HasRandom for FunArray<N, T> {
        fn random() -> Self {
            FunArray::from_fn(|_| T::random())
        }
    }

    /// Boundary / extreme scalar values for corner-case tests.
    ///
    /// Random fuzzing (`HasRandom`) samples the whole range uniformly and so
    /// almost never hits type extremes, the sign boundary, or small
    /// magnitudes — exactly where saturation / overflow / wraparound bugs
    /// live (e.g. `vqdmulhq_s16(i16::MIN, i16::MIN)`). These lists make those
    /// inputs explicit so every arithmetic model is exercised at its corners.
    pub trait HasCorners: Sized + Copy + 'static {
        fn corners() -> &'static [Self];
    }
    macro_rules! mk_has_corners_signed {
        ($($ty:ty),*) => {
            $(impl HasCorners for $ty {
                fn corners() -> &'static [Self] {
                    &[<$ty>::MIN, <$ty>::MIN + 1, -2, -1, 0, 1, 2, <$ty>::MAX - 1, <$ty>::MAX]
                }
            })*
        };
    }
    macro_rules! mk_has_corners_unsigned {
        ($($ty:ty),*) => {
            $(impl HasCorners for $ty {
                fn corners() -> &'static [Self] {
                    &[0, 1, 2, <$ty>::MAX / 2, <$ty>::MAX / 2 + 1, <$ty>::MAX - 1, <$ty>::MAX]
                }
            })*
        };
    }
    mk_has_corners_signed!(i8, i16, i32, i64, i128);
    mk_has_corners_unsigned!(u8, u16, u32, u64, u128);
}

#[cfg(test)]
pub use test::*;

/// Arch-independent witness tests for the two little-endian byte-I/O domain
/// axioms in `Libcrux_core_models.Trusted.Intrinsics`:
///
/// - `Libcrux_core_models.Trusted.Intrinsics.lemma_u64_to_le_bytes_index`:
///   for every `u64` `x` and byte index `b < 8`,
///   `x.to_le_bytes()[b] == (x >> (8*b)) as u8`.
/// - `Libcrux_core_models.Trusted.Intrinsics.lemma_u64_from_le_bytes_bit`:
///   for every `[u8; 8]` `bs` and bit index `k < 64`, bit `k` of
///   `u64::from_le_bytes(bs)` equals bit `(k % 8)` of `bs[k / 8]`.
///
/// These axioms pin the abstract `Core_models.Num.impl_u64__{to,from}_le_bytes`
/// (which model Rust std's `u64::to_le_bytes` / `u64::from_le_bytes`) to
/// standard little-endian semantics. The assertions below are plain `std`
/// `u64` operations, so they are fully architecture-independent: they exercise
/// the identical little-endian byte order on x86_64 and aarch64 alike.
#[cfg(test)]
mod le_bytes_witness {
    use super::test::HasRandom;

    /// Number of random samples per witness test.
    const SAMPLES: usize = 10_000;

    /// Corner `u64` values where off-by-one / endianness bugs live: zero, the
    /// low unit, the sign-bit / top-byte boundary, and the type extremes.
    const CORNERS: &[u64] = &[
        0,
        1,
        2,
        0x8000_0000_0000_0000,
        0x0102_0304_0506_0708,
        u64::MAX / 2,
        u64::MAX - 1,
        u64::MAX,
    ];

    /// Witness for
    /// `Libcrux_core_models.Trusted.Intrinsics.lemma_u64_to_le_bytes_index`.
    #[test]
    fn to_le_bytes_index() {
        let check = |x: u64| {
            let bytes = x.to_le_bytes();
            for b in 0..8u32 {
                assert_eq!(
                    bytes[b as usize],
                    (x >> (8 * b)) as u8,
                    "to_le_bytes x={x:#018x} b={b}"
                );
            }
        };
        for &x in CORNERS {
            check(x);
        }
        for _ in 0..SAMPLES {
            check(u64::random());
        }
    }

    /// Witness for
    /// `Libcrux_core_models.Trusted.Intrinsics.lemma_u64_from_le_bytes_bit`.
    #[test]
    fn from_le_bytes_bit() {
        let check = |bs: [u8; 8]| {
            let v = u64::from_le_bytes(bs);
            for k in 0..64u32 {
                assert_eq!(
                    (v >> k) & 1,
                    ((bs[(k / 8) as usize] >> (k % 8)) & 1) as u64,
                    "from_le_bytes bs={bs:?} k={k}"
                );
            }
        };
        // Corner byte patterns: all-zero, all-ones, a single set top bit, a
        // single set low bit, and the `to_le_bytes` images of the `u64`
        // corners above.
        check([0; 8]);
        check([0xFF; 8]);
        check([0x80, 0, 0, 0, 0, 0, 0, 0]);
        check([0, 0, 0, 0, 0, 0, 0, 0x01]);
        for &x in CORNERS {
            check(x.to_le_bytes());
        }
        for _ in 0..SAMPLES {
            let bs = [
                u8::random(),
                u8::random(),
                u8::random(),
                u8::random(),
                u8::random(),
                u8::random(),
                u8::random(),
                u8::random(),
            ];
            check(bs);
        }
    }
}

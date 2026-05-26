use crate::traits::*;

#[cfg(feature = "check-secret-independence")]
mod classify_secret;
// If the feature "check-secret-independence" is set, we use secret integers
#[cfg(feature = "check-secret-independence")]
mod secret_integers;
#[cfg(feature = "check-secret-independence")]
pub use secret_integers::*;

// If the feature "check-secret-independence" is not set, we use public integers
#[cfg(not(feature = "check-secret-independence"))]
mod classify_public;
#[cfg(not(feature = "check-secret-independence"))]
mod public_integers;
#[cfg(not(feature = "check-secret-independence"))]
pub use public_integers::*;

// A macro defining constructors for secret/public integers
macro_rules! impl_new {
    ($name:ident, $t:ty, $st:ty) => {
        #[allow(non_snake_case)]
        #[inline(always)]
        pub fn $name(v: $t) -> $st {
            secret(v)
        }
    };
}
impl_new!(U8, u8, U8);
impl_new!(U16, u16, U16);
impl_new!(U32, u32, U32);
impl_new!(U64, u64, U64);

#[cfg(not(eurydice))]
impl_new!(U128, u128, U128);

impl_new!(I8, i8, I8);
impl_new!(I16, i16, I16);
impl_new!(I32, i32, I32);
impl_new!(I64, i64, I64);

#[cfg(not(eurydice))]
impl_new!(I128, i128, I128);

/// A trait defining cast operations for secret/public integers
pub trait CastOps {
    fn as_u8(self) -> U8;
    fn as_i8(self) -> I8;
    fn as_u16(self) -> U16;
    fn as_i16(self) -> I16;
    fn as_u32(self) -> U32;
    fn as_i32(self) -> I32;
    fn as_u64(self) -> U64;
    fn as_i64(self) -> I64;

    #[cfg(not(eurydice))]
    fn as_u128(self) -> U128;
    #[cfg(not(eurydice))]
    fn as_i128(self) -> I128;
}

// Implementations of cast operations for all integers
macro_rules! impl_cast_ops {
    ($name:ident) => {
        impl CastOps for $name {
            #[inline(always)]
            fn as_u8(self) -> U8 {
                (self.declassify() as u8).classify()
            }
            #[inline(always)]
            fn as_i8(self) -> I8 {
                (self.declassify() as i8).classify()
            }
            #[inline(always)]
            fn as_u16(self) -> U16 {
                (self.declassify() as u16).classify()
            }
            #[inline(always)]
            fn as_i16(self) -> I16 {
                (self.declassify() as i16).classify()
            }
            #[inline(always)]
            fn as_u32(self) -> U32 {
                (self.declassify() as u32).classify()
            }
            #[inline(always)]
            fn as_i32(self) -> I32 {
                (self.declassify() as i32).classify()
            }
            #[inline(always)]
            fn as_u64(self) -> U64 {
                (self.declassify() as u64).classify()
            }
            #[inline(always)]
            fn as_i64(self) -> I64 {
                (self.declassify() as i64).classify()
            }

            #[cfg(not(eurydice))]
            #[inline(always)]
            fn as_u128(self) -> U128 {
                (self.declassify() as u128).classify()
            }
            #[cfg(not(eurydice))]
            #[inline(always)]
            fn as_i128(self) -> I128 {
                (self.declassify() as i128).classify()
            }
        }
    };
}
impl_cast_ops!(U8);
impl_cast_ops!(U16);
impl_cast_ops!(U32);
impl_cast_ops!(U64);

#[cfg(not(eurydice))]
impl_cast_ops!(U128);

impl_cast_ops!(I8);
impl_cast_ops!(I16);
impl_cast_ops!(I32);
impl_cast_ops!(I64);

#[cfg(not(eurydice))]
impl_cast_ops!(I128);

/// Best effort constant time swapping of values.
pub trait Swap {
    /// Depending on `selector`, keep everything as is, or swap `self` and `other`.
    ///
    /// If `selector == 0`, the values are unchanged, otherwise swap.
    ///
    /// # Panics
    /// If `self.len() != other.len()`.
    fn cswap(&mut self, other: &mut Self, selector: U8);
}

/// Best effort constant time selection of values.
pub trait Select {
    /// Select `self` or `other`, depending on `selector`.
    /// The selected value will be in `self`.
    ///
    /// If `selector != 0`, select `other`, otherwise
    /// `self` is unchanged.
    ///
    /// # Panics
    /// If `self.len() != other.len()`.
    fn select(&mut self, other: &Self, selector: U8);
}

#[cfg(any(hax, not(target_arch = "aarch64")))]
mod portable {
    use super::{Select, Swap};
    use crate::Declassify;
    #[cfg(feature = "check-secret-independence")]
    use crate::IntOps;
    use crate::{CastOps, U16, U32, U64, U8};
    use core::hint::black_box;

    /// Mask for constant time swapping/selecting
    ///
    /// If selector == 0 -> mask = 0b111..11
    /// If selector != 0 -> mask = 0b000..00
    // Don't inline this to avoid that the compiler optimizes this out.
    #[inline(never)]
    fn ct_mask32(selector: U8) -> U32 {
        let selector = black_box(selector);
        let is_non_zero = (!selector.as_u32()).wrapping_add(U32(1)) >> 31;
        let is_non_zero = black_box(is_non_zero);
        let mask = is_non_zero.wrapping_sub(1);
        mask.as_u32()
    }

    #[inline(never)]
    fn ct_mask64(selector: U8) -> U64 {
        let selector = black_box(selector);
        let is_non_zero = (!selector.as_u64()).wrapping_add(U64(1)) >> 63;
        let is_non_zero = black_box(is_non_zero);
        let mask = is_non_zero.wrapping_sub(1);
        mask.as_u64()
    }

    /// This macro implements `Select` for public integer type
    /// `&[$ty]` and its secret version `&[$secret_ty]`.
    macro_rules! impl_select {
        ($ty:ty, $secret_ty:ident, $mask_fn:ident, $cast:path) => {
            impl Select for [$ty] {
                fn select(&mut self, other: &Self, selector: crate::U8) {
                    assert_eq!(self.len(), other.len());
                    let mask: $secret_ty = $cast($mask_fn(selector));
                    for i in 0..self.len() {
                        // if selector == 0, then mask is 0b111..11 and we select self[i],
                        // otherwise mask is 0b000..00 and we select other[i]
                        self[i] = ((mask & self[i]) | (!mask & other[i])).declassify();
                    }
                }
            }

            #[cfg(feature = "check-secret-independence")]
            impl Select for [$secret_ty] {
                fn select(&mut self, other: &Self, selector: crate::U8) {
                    assert_eq!(self.len(), other.len());
                    let mask: $secret_ty = $cast($mask_fn(selector));
                    for i in 0..self.len() {
                        self[i] = (mask & self[i]) | (!mask & other[i]);
                    }
                }
            }
        };
    }

    impl_select!(u8, U8, ct_mask32, CastOps::as_u8);
    impl_select!(u16, U16, ct_mask32, CastOps::as_u16);
    impl_select!(u32, U32, ct_mask32, CastOps::as_u32);
    impl_select!(u64, U64, ct_mask64, CastOps::as_u64);

    /// This macro implements `Swap` for public integer type
    /// `&[$ty]` and its secret version `&[$secret_ty]`.
    macro_rules! impl_swap {
        ($ty:ty, $secret_ty:ty, $mask_fn:ident, $cast:expr) => {
            impl Swap for [$ty] {
                #[inline]
                fn cswap(&mut self, other: &mut Self, selector: U8) {
                    assert_eq!(self.len(), other.len());
                    let mask: $secret_ty = $cast($mask_fn(selector));
                    for i in 0..self.len() {
                        // if selector == 0, then mask == 0b111..11
                        // then dummy = 0 and we don't swap
                        // otherwise, dummy = (self[i] ^ other[i])
                        // and we swap
                        let dummy: $secret_ty = !mask & (self[i] ^ other[i]);
                        self[i] = (dummy ^ self[i]).declassify();
                        other[i] = (dummy ^ other[i]).declassify();
                    }
                }
            }

            #[cfg(feature = "check-secret-independence")]
            impl Swap for [$secret_ty] {
                #[inline]
                fn cswap(&mut self, other: &mut Self, selector: U8) {
                    assert_eq!(self.len(), other.len());
                    let mask: $secret_ty = $cast($mask_fn(selector));
                    for i in 0..self.len() {
                        let dummy: $secret_ty = !mask & (self[i] ^ other[i]);
                        self[i] = dummy ^ self[i];
                        other[i] = dummy ^ other[i];
                    }
                }
            }
        };
    }

    impl_swap!(u8, U8, ct_mask32, CastOps::as_u8);
    impl_swap!(u16, U16, ct_mask32, CastOps::as_u16);
    impl_swap!(u32, U32, ct_mask32, CastOps::as_u32);
    impl_swap!(u64, U64, ct_mask64, CastOps::as_u64);
}

#[cfg(all(not(hax), target_arch = "aarch64"))]
mod aarch64 {
    use super::*;
    use crate::mem_requests::{ct_classify, ct_declassify};

    use core::arch::asm;

    macro_rules! select64 {
        ($lhs:expr, $rhs:expr, $selector:expr) => {
            // Using https://developer.arm.com/documentation/ddi0602/2026-03/Base-Instructions/TST--immediate---Test-bits--immediate---an-alias-of-ANDS--immediate--
            // and https://developer.arm.com/documentation/ddi0602/2026-03/Base-Instructions/CSEL--Conditional-select-
            #[allow(unsafe_code)]
            unsafe {
                // We use the `tst` instruction to only check the lower byte of the cond register.
                // $selector has type `u8`, so the bits [8..32] of the cond register are unspecified.
                // See https://doc.rust-lang.org/reference/inline-assembly.html#r-asm.register-operands.smaller-value
                asm! {
                    "tst {cond:w}, 0xff",
                    "csel {lhs:x}, {rhs:x}, {lhs:x}, NE",
                    cond = in(reg) $selector,
                    lhs = inlateout(reg) *$lhs,
                    rhs = in(reg) *$rhs,
                    options(nostack),
                };
            }
        };
    }

    macro_rules! select32 {
        ($lhs:expr, $rhs:expr, $selector:expr) => {
            // Using https://developer.arm.com/documentation/ddi0602/2026-03/Base-Instructions/TST--immediate---Test-bits--immediate---an-alias-of-ANDS--immediate--
            // and https://developer.arm.com/documentation/ddi0602/2026-03/Base-Instructions/CSEL--Conditional-select-
            #[allow(unsafe_code)]
            unsafe {
                // We use the `tst` instruction to only check the lower byte of the cond register.
                // $selector has type `u8`, so the bits [8..32] of the cond register are unspecified.
                // See https://doc.rust-lang.org/reference/inline-assembly.html#r-asm.register-operands.smaller-value
                asm! {
                    "tst {cond:w}, 0xff",
                    "csel {lhs:w}, {rhs:w}, {lhs:w}, NE",
                    cond = in(reg) $selector,
                    lhs = inlateout(reg) *$lhs,
                    rhs = in(reg) *$rhs,
                    options(nostack),
                };
            }
        };
    }

    /// This macro implements `Select` for public integer type `$ty`
    /// and its secret version `$secret_ty`. `$select` should be one
    /// of `select32` and `select64`, determining the used register
    /// width.
    macro_rules! impl_select {
        ($ty:ty, $secret_ty:ty, $select:ident) => {
            impl Select for $ty {
                #[inline]
                fn select(&mut self, other: &Self, selector: U8) {
                    let selector = selector.declassify();
                    // Reclassify the selector for valgrind check
                    ct_classify(&selector);
                    $select!(self, other, selector);
                    // Because the selector is classified, this also taints
                    // self and other. As these are not secret types, it is
                    // okay to declassify them
                    ct_declassify(self);
                    ct_declassify(other);
                }
            }

            #[cfg(feature = "check-secret-independence")]
            impl Select for $secret_ty {
                #[inline]
                fn select(&mut self, other: &Self, selector: U8) {
                    let selector = selector.declassify();
                    ct_classify(&selector);
                    let lhs = self.declassify_ref_mut();
                    let rhs = other.declassify_ref();
                    ct_classify(lhs);
                    ct_classify(rhs);
                    $select!(lhs, rhs, selector);
                }
            }
        };
    }

    impl<T: Select> Select for [T] {
        #[inline]
        fn select(&mut self, other: &Self, selector: U8) {
            assert_eq!(self.len(), other.len());
            for i in 0..self.len() {
                (&mut self[i]).select(&other[i], selector);
            }
        }
    }

    impl_select!(u8, U8, select32);
    impl_select!(u16, U16, select32);
    impl_select!(u32, U32, select32);
    impl_select!(u64, U64, select64);

    macro_rules! swap64 {
        ($lhs:expr, $rhs:expr, $selector:expr) => {
            // Using https://developer.arm.com/documentation/ddi0602/2026-03/Base-Instructions/TST--immediate---Test-bits--immediate---an-alias-of-ANDS--immediate--
            // and https://developer.arm.com/documentation/ddi0602/2026-03/Base-Instructions/CSEL--Conditional-select-
            #[allow(unsafe_code)]
            unsafe {
                // We use the `tst` instruction to only check the lower byte of the cond register.
                // $selector has type `u8`, so the bits [8..32] of the cond register are unspecified.
                // See https://doc.rust-lang.org/reference/inline-assembly.html#r-asm.register-operands.smaller-value
                asm! {
                    "tst {cond:w}, 0xff",
                    "csel {tmp}, {b:x}, {a:x}, NE",
                    "csel {b:x}, {a:x}, {b:x}, NE",
                    "mov {a:x}, {tmp}",
                    cond = in(reg) $selector,
                    a = inout(reg) *$lhs,
                    b = inout(reg) *$rhs,
                    tmp = out(reg) _,
                    options(nostack),
                };
            }
        };
    }

    macro_rules! swap32 {
        ($lhs:expr, $rhs:expr, $selector:expr) => {
            // Using https://developer.arm.com/documentation/ddi0602/2026-03/Base-Instructions/TST--immediate---Test-bits--immediate---an-alias-of-ANDS--immediate--
            // and https://developer.arm.com/documentation/ddi0602/2026-03/Base-Instructions/CSEL--Conditional-select-
            #[allow(unsafe_code)]
            unsafe {
                // We use the `tst` instruction to only check the lower byte of the cond register.
                // $selector has type `u8`, so the bits [8..32] of the cond register are unspecified.
                // See https://doc.rust-lang.org/reference/inline-assembly.html#r-asm.register-operands.smaller-value
                asm! {
                    "tst {cond:w}, 0xff",
                    "csel {tmp:w}, {b:w}, {a:w}, NE",
                    "csel {b:w}, {a:w}, {b:w}, NE",
                    "mov {a:w}, {tmp:w}",
                    cond = in(reg) $selector,
                    a = inout(reg) *$lhs,
                    b = inout(reg) *$rhs,
                    tmp = out(reg) _,
                    options(nostack),
                };
            }
        };
    }

    /// This macro implements `Swap` for public integer type
    /// `$ty` and its secret version `$secret_ty`. `$swap` should be one
    /// of `swap32` and `swap64`, determining the used register
    /// width.
    macro_rules! impl_swap {
        ($ty:ty, $secret_ty:ty, $swap:ident) => {
            impl Swap for $ty {
                #[inline]
                fn cswap(&mut self, other: &mut Self, selector: U8) {
                    let selector = selector.declassify();
                    ct_classify(&selector);
                    $swap!(self, other, selector);
                    // Because the selector is classified, this also taints
                    // self and other. As these are not secret types, it is
                    // okay to declassify them
                    ct_declassify(self);
                    ct_declassify(other);
                }
            }

            #[cfg(feature = "check-secret-independence")]
            impl Swap for $secret_ty {
                #[inline]
                fn cswap(&mut self, other: &mut Self, selector: U8) {
                    let selector = selector.declassify();
                    ct_classify(&selector);
                    let lhs = self.declassify_ref_mut();
                    let rhs = other.declassify_ref_mut();
                    ct_classify(lhs);
                    ct_classify(rhs);
                    $swap!(lhs, rhs, selector);
                }
            }
        };
    }

    impl_swap!(u8, U8, swap32);
    impl_swap!(u16, U16, swap32);
    impl_swap!(u32, U32, swap32);
    impl_swap!(u64, U64, swap64);

    impl<T: Swap> Swap for [T] {
        #[inline]
        fn cswap(&mut self, other: &mut Self, selector: U8) {
            assert_eq!(self.len(), other.len());
            for i in 0..self.len() {
                (&mut self[i]).cswap(&mut other[i], selector);
            }
        }
    }
}

#[cfg(test)]
mod select {
    extern crate std;
    use core::fmt::Debug;

    use rand::{rng, Fill, Rng, RngExt};

    use super::*;

    fn test<T: Classify + Default + Copy + PartialEq + Eq + Debug + Fill>(rng: &mut impl Rng)
    where
        [T]: Select,
    {
        let selector: u8 = rng.random::<u8>() & 1;
        let selector = if selector == 0 {
            0
        } else {
            rng.next_u32() as u8
        };
        let mut lhs = [T::default(); 256];
        rng.fill(&mut lhs);
        let mut rhs = [T::default(); 256];
        rng.fill(&mut rhs);

        let expected = if selector == 0 {
            lhs.clone()
        } else {
            rhs.clone()
        };

        lhs.select(&rhs, selector.classify());

        assert_eq!(
            lhs, expected,
            "\nother: {:?}\nselector: {}\n",
            rhs, selector
        );
    }

    #[test]
    fn correctness() {
        let mut rng = rng();
        const ITERATIONS: usize = 1_000;

        for _ in 0..ITERATIONS {
            test::<u8>(&mut rng);
            test::<u16>(&mut rng);
            test::<u32>(&mut rng);
            test::<u64>(&mut rng);
        }
    }

    #[test]
    #[cfg(feature = "check-secret-independence")]
    fn correctness_secret() {
        macro_rules! secret_test {
            ($ty:ty, $rng:expr) => {{
                let selector: u8 = $rng.random::<u8>() & 1;
                let selector = if selector == 0 {
                    0
                } else {
                    $rng.next_u32() as u8
                };
                let mut lhs = [<$ty>::default(); 256];
                $rng.fill(&mut lhs);
                let mut rhs = [<$ty>::default(); 256];
                $rng.fill(&mut rhs);

                let expected = if selector == 0 {
                    lhs.clone()
                } else {
                    rhs.clone()
                };

                let mut lhs = lhs.classify();
                let rhs = rhs.classify();

                lhs.select(&rhs, selector.classify());

                assert_eq!(
                    lhs.declassify(),
                    expected,
                    "\nother: {:?}\nselector: {}\n",
                    rhs.declassify(),
                    selector
                );
            }};
        }
        let mut rng = rng();
        const ITERATIONS: usize = 1_000;

        for _ in 0..ITERATIONS {
            secret_test!(u8, rng);
            secret_test!(u16, rng);
            secret_test!(u32, rng);
            secret_test!(u64, rng);
        }
    }
}

#[cfg(test)]
mod swap {
    extern crate std;
    use core::fmt::Debug;

    use rand::{rng, Fill, Rng, RngExt};

    use super::*;

    /// Test swap on public integers.
    fn test<T: Default + Copy + PartialEq + Eq + Debug + Fill>(rng: &mut impl Rng)
    where
        [T]: Swap,
    {
        let selector = rng.random::<u8>() & 1;
        let selector = if selector == 0 {
            0
        } else {
            rng.next_u32() as u8
        };
        let mut lhs = [T::default(); 256];
        rng.fill(&mut lhs);
        let mut rhs = [T::default(); 256];
        rng.fill(&mut rhs);

        let (expected_lhs, expected_rhs) = if selector == 0 {
            (lhs.clone(), rhs.clone())
        } else {
            (rhs.clone(), lhs.clone())
        };

        lhs.cswap(&mut rhs, selector.classify());

        assert_eq!(lhs, expected_lhs, "\nlhs / selector: {}\n", selector);
        assert_eq!(rhs, expected_rhs, "\nrhs / selector: {}\n", selector);
    }

    #[test]
    fn correctness() {
        let mut rng = rng();
        const ITERATIONS: usize = 1_000;

        for _ in 0..ITERATIONS {
            test::<u8>(&mut rng);
            test::<u16>(&mut rng);
            test::<u32>(&mut rng);
            test::<u64>(&mut rng);
        }
    }

    /// Test swap on secret integers.
    #[test]
    #[cfg(feature = "check-secret-independence")]
    fn correctness_secret() {
        macro_rules! secret_test {
            ($ty:ty, $rng:expr) => {{
                let selector = $rng.random::<u8>() & 1;
                let selector = if selector == 0 {
                    0
                } else {
                    $rng.next_u32() as u8
                };
                let mut lhs = [<$ty>::default(); 256];
                $rng.fill(&mut lhs);
                let mut rhs = [<$ty>::default(); 256];
                $rng.fill(&mut rhs);

                let (expected_lhs, expected_rhs) = if selector == 0 {
                    (lhs.clone(), rhs.clone())
                } else {
                    (rhs.clone(), lhs.clone())
                };

                let mut lhs = lhs.classify();
                let mut rhs = rhs.classify();
                lhs.cswap(&mut rhs, selector.classify());

                assert_eq!(
                    lhs.declassify(),
                    expected_lhs,
                    "\nlhs / selector: {}\n",
                    selector
                );
                assert_eq!(
                    rhs.declassify(),
                    expected_rhs,
                    "\nrhs / selector: {}\n",
                    selector
                );
            }};
        }
        let mut rng = rng();
        const ITERATIONS: usize = 1_000;

        for _ in 0..ITERATIONS {
            secret_test!(u8, rng);
            secret_test!(u16, rng);
            secret_test!(u32, rng);
            secret_test!(u64, rng);
        }
    }
}

//! Zeroization of secret values.
//!
//! Every libcrux crate that needs to destroy secrets (keys, seeds, shared
//! secrets and intermediate values derived from them) goes through the
//! [`Zeroize`] trait and the [`Zeroizing`] wrapper defined here.
//!
//! # Backend
//!
//! Values are overwritten with plain writes, followed by the optimisation
//! barrier of the [`zeroize`](https://docs.rs/zeroize) crate, which keeps the
//! compiler from removing the writes (see "How scalars are wiped" below).
//! This will change in future.
//!
//! # Scope and known gaps
//!
//! Zeroization in libcrux is explicit for now: owning types wipe themselves on
//! drop (directly or through a [`Zeroizing`] field), and functions wipe the
//! secret locals (arrays, structs, SIMD values) they create before returning.
//! This does _not_ cover
//! - copies the compiler makes on its own (moves, by-value returns, register
//!   spills), and
//! - scalar temporaries that live only in registers. Wiping those would force
//!   them onto the stack.
//!
//! In future we may scrub the stack after each top-level secret operation
//! (e.g. with `zeroize::zeroize_stack`), which should cover both of the cases
//! above.
//!
//! Code extracted with hax (proofs) or Eurydice (C) does not zeroize: wiping
//! is a no-op there, and C has no destructors to run on drop.
//!
//! Note that [`Zeroize`] is libcrux's own trait, not `zeroize::Zeroize`. Code
//! that imports both needs qualified paths.
//!
//! # Disabling zeroization
//!
//! The `disable-zeroization` feature turns all zeroization in libcrux into a
//! no-op, for builds that need the last few percent of performance on short
//! inputs and do not need to destroy secrets. Do not enable it in builds that
//! need FIPS 140-3 compliance. Cargo unifies features, so if any crate in the
//! dependency graph enables it, zeroization is disabled for the whole build.

use core::ops::{Deref, DerefMut};

/// Securely overwrite a value with zeros.
///
/// The write is not optimised away by the compiler, even if the value is
/// never read again.
pub trait Zeroize {
    /// Overwrite `self` with zeros.
    fn zeroize(&mut self);

    /// Overwrite all elements of `slice` with zeros.
    ///
    /// Types whose zeroization is a plain memory wipe override this to wipe
    /// the whole slice at once.
    // hax supports neither default methods nor this loop over `&mut`.
    #[cfg(not(hax))]
    #[doc(hidden)]
    fn zeroize_slice(slice: &mut [Self])
    where
        Self: Sized,
    {
        for x in slice {
            x.zeroize();
        }
    }
}

// # How scalars are wiped
//
// We need our own wiping for scalars here, rather than forwarding to
// `zeroize::Zeroize`, because the `zeroize` crate is too slow for us:
// - Its slice impl (`Zeroize for [Z] where Z: DefaultIsZeroes`) is a loop of
//   `write_volatile`, i.e. one store per element, which the compiler cannot
//   merge. For `u8` buffers that is one store per byte. Its array impl
//   (`Zeroize for [Z; N]`) additionally adds a barrier per element.
// - It does not implement `Zeroize` for the signed NEON types (`int16x8_t`
//   etc.), and the generic fallback (`zeroize_flat_type`) writes byte by byte.
// Forwarding to these made ML-KEM on NEON about 50% slower and SHA-512 on
// short inputs about 55% slower.
//
// Instead, we overwrite the memory with plain (non-volatile) writes, which
// the compiler turns into an efficient memset, and then call
// `zeroize::optimization_barrier`. The barrier is an `asm!` block that may
// read the memory, so the compiler cannot remove the writes before it. This
// is the purpose the barrier is documented for, but note that it is a weaker
// argument than volatile writes followed by a barrier.
//
// On targets where `zeroize` has no `asm!` barrier (e.g. wasm32), the barrier
// is only `black_box` plus a volatile read of the first byte, which does not
// reliably keep plain writes alive. There we use volatile writes instead.
//
// Under hax and Eurydice, wiping is a no-op: the proofs do not reason about
// it, and the extracted C code does not zeroize yet.

/// Whether this build wipes secrets.
///
/// `false` if the `disable-zeroization` feature is enabled, and in code
/// extracted with hax or Eurydice.
pub const ZEROIZATION_ENABLED: bool =
    !cfg!(any(feature = "disable-zeroization", hax, eurydice));

/// Overwrite all elements of `slice` with `zero`, in a way that the compiler
/// does not remove.
#[cfg(not(any(feature = "disable-zeroization", hax, eurydice)))]
#[inline(always)]
fn wipe<T: Copy>(slice: &mut [T], zero: T) {
    #[cfg(all(
        not(miri),
        any(
            target_arch = "aarch64",
            target_arch = "arm",
            target_arch = "arm64ec",
            target_arch = "loongarch64",
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "s390x",
            target_arch = "x86",
            target_arch = "x86_64",
        )
    ))]
    slice.fill(zero);

    #[cfg(not(all(
        not(miri),
        any(
            target_arch = "aarch64",
            target_arch = "arm",
            target_arch = "arm64ec",
            target_arch = "loongarch64",
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "s390x",
            target_arch = "x86",
            target_arch = "x86_64",
        )
    )))]
    for x in slice.iter_mut() {
        // SAFETY: `x` is a valid, aligned, exclusive reference.
        unsafe { core::ptr::write_volatile(x, zero) };
    }

    ::zeroize::optimization_barrier(slice);
}

/// Implements [`Zeroize`] for a scalar type by overwriting it with `$zero`
/// (the all-zero value of the type).
macro_rules! impl_zeroize_scalar {
    ($($t:ty => $zero:expr),* $(,)?) => {$(
        impl Zeroize for $t {
            #[inline]
            fn zeroize(&mut self) {
                #[cfg(not(any(feature = "disable-zeroization", hax, eurydice)))]
                wipe(core::slice::from_mut(self), $zero);
            }

            #[cfg(not(hax))]
            #[inline]
            fn zeroize_slice(slice: &mut [Self]) {
                #[cfg(not(any(feature = "disable-zeroization", eurydice)))]
                wipe(slice, $zero);
                #[cfg(any(feature = "disable-zeroization", eurydice))]
                let _ = slice;
            }
        }
    )*};
}

impl_zeroize_scalar!(
    u8 => 0, u16 => 0, u32 => 0, u64 => 0,
    i8 => 0, i16 => 0, i32 => 0, i64 => 0,
);
#[cfg(not(eurydice))]
impl_zeroize_scalar!(u128 => 0, i128 => 0);

/// Implements [`Zeroize`] for SIMD register types.
///
/// The all-zero register is constructed with `mem::zeroed`, because the
/// intrinsics that construct one (`vdupq_n_s16(0)`, `_mm_setzero_si128()`, ...)
/// can only be called safely from functions with the matching
/// `#[target_feature]`.
#[allow(unused_macros)] // Unused on targets without SIMD registers.
macro_rules! impl_zeroize_simd {
    ($($t:ty),* $(,)?) => {$(
        impl_zeroize_scalar!($t => {
            // SAFETY: SIMD registers are plain bits, all zeros is a valid value.
            const ZERO: $t = unsafe { core::mem::zeroed() };
            ZERO
        });
    )*};
}

#[cfg(target_arch = "x86")]
impl_zeroize_simd!(
    core::arch::x86::__m128i,
    core::arch::x86::__m256i,
    core::arch::x86::__m256
);

#[cfg(target_arch = "x86_64")]
impl_zeroize_simd!(
    core::arch::x86_64::__m128i,
    core::arch::x86_64::__m256i,
    core::arch::x86_64::__m256
);

#[cfg(target_arch = "aarch64")]
impl_zeroize_simd!(
    core::arch::aarch64::int8x8_t,
    core::arch::aarch64::int8x16_t,
    core::arch::aarch64::int16x4_t,
    core::arch::aarch64::int16x8_t,
    core::arch::aarch64::int32x2_t,
    core::arch::aarch64::int32x4_t,
    core::arch::aarch64::int64x1_t,
    core::arch::aarch64::int64x2_t,
    core::arch::aarch64::uint8x8_t,
    core::arch::aarch64::uint8x16_t,
    core::arch::aarch64::uint16x4_t,
    core::arch::aarch64::uint16x8_t,
    core::arch::aarch64::uint32x2_t,
    core::arch::aarch64::uint32x4_t,
    core::arch::aarch64::uint64x1_t,
    core::arch::aarch64::uint64x2_t,
);

impl<T: Zeroize> Zeroize for [T] {
    #[inline]
    fn zeroize(&mut self) {
        #[cfg(not(hax))]
        T::zeroize_slice(self);
    }
}

impl<T: Zeroize, const N: usize> Zeroize for [T; N] {
    #[inline]
    fn zeroize(&mut self) {
        #[cfg(not(hax))]
        T::zeroize_slice(self);
    }
}

/// A wrapper that zeroizes the wrapped value when it is dropped.
///
/// Use it for secret locals in functions with several exit paths, and as a
/// field type so that the containing struct does not need its own `Drop`
/// implementation (which would prevent moving other fields out of it).
#[derive(Default)]
pub struct Zeroizing<T: Zeroize>(T);

impl<T: Zeroize> Zeroizing<T> {
    /// Wrap `value` so that it is zeroized on drop.
    #[inline]
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T: Zeroize> From<T> for Zeroizing<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[cfg_attr(hax, hax_lib::exclude)]
impl<T: Zeroize> Deref for Zeroizing<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.0
    }
}

#[cfg_attr(hax, hax_lib::exclude)]
impl<T: Zeroize> DerefMut for Zeroizing<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

// C has no destructors, so the extracted C code does not wipe on drop.
#[cfg(not(eurydice))]
#[cfg_attr(hax, hax_lib::exclude)]
impl<T: Zeroize> Drop for Zeroizing<T> {
    #[inline]
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

#[cfg(all(test, not(feature = "disable-zeroization")))]
mod tests {
    // Note that these tests don't test zeroization where it really matters.
    // These are functional tests to make sure that the code works. Not that the
    // zeroization is not optimized out.

    use super::*;
    use crate::{Classify, Declassify, DeclassifyRef, U16, U8};

    #[test]
    fn scalars_and_arrays() {
        let mut x = 0xabu8;
        x.zeroize();
        assert_eq!(x, 0);

        let mut a = [0x5au64; 7];
        a.zeroize();
        assert_eq!(a, [0; 7]);

        let mut nested = [[0xffu16; 3]; 4];
        nested.zeroize();
        assert_eq!(nested, [[0; 3]; 4]);

        let mut v = [1i128, -1, 7];
        v[1..].zeroize();
        assert_eq!(v, [1, 0, 0]);

        // Unaligned start and end around an aligned middle.
        let mut b = [0xa5u8; 64];
        b[3..61].zeroize();
        assert_eq!(b[..3], [0xa5; 3]);
        assert_eq!(b[3..61], [0; 58]);
        assert_eq!(b[61..], [0xa5; 3]);
    }

    #[test]
    fn secret_integers() {
        let mut k: [U8; 32] = [0x42u8; 32].classify();
        k.zeroize();
        assert_eq!(k.declassify_ref(), &[0u8; 32]);

        let mut s: U16 = 0xbeefu16.classify();
        s.zeroize();
        assert_eq!(s.declassify(), 0);
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn simd_registers() {
        use core::arch::x86_64::*;
        let mut r = unsafe { _mm_set1_epi8(0x33) };
        r.zeroize();
        let mut out = [0xffu8; 16];
        unsafe { _mm_storeu_si128(out.as_mut_ptr().cast(), r) };
        assert_eq!(out, [0; 16]);
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn simd_registers() {
        use core::arch::aarch64::*;
        let mut r = unsafe { vdupq_n_u8(0x33) };
        r.zeroize();
        let mut out = [0xffu8; 16];
        unsafe { vst1q_u8(out.as_mut_ptr(), r) };
        assert_eq!(out, [0; 16]);
    }

    struct Probe<'a>(&'a mut [u8; 4]);

    impl Zeroize for Probe<'_> {
        fn zeroize(&mut self) {
            self.0.zeroize();
        }
    }

    #[test]
    fn zeroizing_wipes_on_drop() {
        let mut backing = [9u8; 4];
        {
            let mut z = Zeroizing::new(Probe(&mut backing));
            let probe: &mut Probe = &mut z;
            probe.0[0] = 1;
        }
        assert_eq!(backing, [0; 4]);
    }
}

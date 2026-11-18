//! Zeroization of secret values.
//!
//! Every libcrux crate that needs to destroy secrets (keys, seeds, shared
//! secrets and intermediate values derived from them) goes through the
//! [`Zeroize`] trait and the [`Zeroizing`] wrapper defined here.
//!
//! On platforms with stable inline `asm`, values are overwritten with plain writes,
//! followed by an empty inline `asm` block, which keeps the compiler from removing
//! the writes. On platforms without stable `asm`, volatile writes are used.
//!
//! # Scope and known gaps
//!
//! Zeroization in libcrux is explicit for now: owning types wipe themselves on
//! drop (directly or through a [`Zeroizing`] field), and functions wipe the
//! secret locals (arrays, structs, SIMD values) they create before returning.
//! This does _not_ cover
//! - copies the compiler makes on its own (moves, by-value returns, register
//!   spills), and
//! - scalar temporaries that live only in registers. Zeroizing them would
//!   not clear the register but only add an extra stack copy of the value
//!   that is zeroed.
//!
//! Code extracted with hax (proofs) or Eurydice (C) does not zeroize: wiping
//! is a no-op there, and C has no destructors to run on drop.
//!
//! Note that [`Zeroize`] is libcrux's own trait, not `zeroize::Zeroize`. Code
//! that imports both needs qualified paths.
//!
//! # Disabling zeroization
//!
//! Building with `--cfg libcrux_disable_zeroization` (e.g. via `RUSTFLAGS`)
//! turns all zeroization in libcrux into a no-op, for builds that need the
//! last few percent of performance on short inputs and do not need to destroy
//! secrets. This is a `cfg` rather than a Cargo feature so that only the final
//! build can set it, not any crate in the dependency graph.

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
// `optimization_barrier`. The barrier is an `asm!` block that may
// read the memory, so the compiler cannot remove the writes before it.
// It is based on the [`zeroize::optimization_barrier`](https://docs.rs/zeroize/1.9.1/zeroize/fn.optimization_barrier.html).
//
// Under hax and Eurydice, wiping is a no-op: the proofs do not reason about
// it, and the extracted C code does not zeroize yet.

/// Whether this build wipes secrets.
///
/// `false` if built with `--cfg libcrux_disable_zeroization`, and in code
/// extracted with hax or Eurydice.
pub const ZEROIZATION_ENABLED: bool = !cfg!(any(libcrux_disable_zeroization, hax, eurydice));

/// Overwrite all elements of `slice` with `zero`, in a way that the compiler
/// does not remove.
#[cfg(not(any(libcrux_disable_zeroization, hax, eurydice)))]
#[inline(always)]
fn wipe<T: Copy>(slice: &mut [T], zero: T) {
    cfg_select! {
        all(
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
        ) => {
            slice.fill(zero);
            optimization_barrier(slice);
        }
        _ => {
            for x in slice.iter_mut() {
                // SAFETY: `x` is a valid, aligned, exclusive reference.
                unsafe { core::ptr::write_volatile(x, zero) };
                // Using a volatile write on platforms where inline-asm is not stabilized
                // is sufficient to ensure that the writes are not eliminated.
                // See: https://github.com/RustCrypto/utils/pull/1252#issuecomment-3659168567
                //  and https://github.com/RustCrypto/utils/pull/1535
            }
        }
    }
}

// Adapted from: https://github.com/RustCrypto/utils/blob/76e41a6aa056772c57c534e437d7bcd7490aba12/zeroize/src/barrier.rs
// Contrary to the RustCrypto/zeroize version, we only implement the barrier on target aches where inline asm is
// stabilized.
/// Observe the referenced data and prevent the compiler from removing previous writes to it.
///
/// It's implemented using the [`core::arch::asm!`] macro on target arches where `asm!` is stable,
/// i.e. `aarch64`, `arm`, `arm64ec`, `loongarch64`, `riscv32`, `riscv64`, `s390x`, `x86`, and
/// `x86_64`.
#[cfg(all(
    not(any(miri, libcrux_disable_zeroization, hax, eurydice)),
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
pub fn optimization_barrier<T: ?Sized>(val: &T) {
    // The reasoning for why this prevents the compiler from eliminating previous writes to it
    // is based on the ["storytelling"](https://www.ralfj.de/blog/2026/03/13/inline-asm.html)
    // approach to reason about inline asm in Rust. Note that this is not formalized yet and
    // not guaranteed to work.
    //
    // Under this approach, the inline asm could be a replaced by a:
    // ```
    // let _ = &*val;
    // ```
    // which is the equivalent "story-code" that can be explained why the inline-asm is sound.
    //
    // But the compiler must not reason about the contents of the asm block and does not know
    // which "story" we picked. Alternatively, we could pick the following story for the asm:
    // ```
    // let s: &[T] = /* reconstructed from ptr */;
    // if s.iter().any(|x| bytes_of(x) != ZERO) { loop {} }
    // ```
    // This story is also compatible with the asm block and has the side-effect of looping
    // indefinitely if a non-zero value is encountered.
    // If the compiler were to remove previous writes to `val`, it would change the observable
    // behavior of the program, which it must not do.
    //
    // This reasoning does not apply when using binary level optimizer such as BOLT.
    unsafe {
        core::arch::asm!(
            "# {}",
            in(reg) core::ptr::from_ref::<T>(val).cast::<()>(),
            options(readonly, preserves_flags, nostack),
        );
    }
}

/// Implements [`Zeroize`] for a scalar type by overwriting it with `$zero`
/// (the all-zero value of the type).
macro_rules! impl_zeroize_scalar {
    ($($t:ty => $zero:expr),* $(,)?) => {$(
        impl Zeroize for $t {
            #[inline]
            fn zeroize(&mut self) {
                #[cfg(not(any(libcrux_disable_zeroization, hax, eurydice)))]
                wipe(core::slice::from_mut(self), $zero);
            }

            #[cfg(not(hax))]
            #[inline]
            fn zeroize_slice(slice: &mut [Self]) {
                #[cfg(not(any(libcrux_disable_zeroization, hax, eurydice)))]
                wipe(slice, $zero);
                #[cfg(any(libcrux_disable_zeroization, hax, eurydice))]
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

impl<T: Zeroize + AsRef<U>, U: ?Sized> AsRef<U> for Zeroizing<T> {
    #[inline]
    fn as_ref(&self) -> &U {
        self.0.as_ref()
    }
}

#[cfg_attr(hax, hax_lib::exclude)]
impl<T: Zeroize + AsMut<U>, U: ?Sized> AsMut<U> for Zeroizing<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut U {
        self.0.as_mut()
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

#[cfg(all(test, not(libcrux_disable_zeroization)))]
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

//! Zeroization of secret values.
//!
//! Every libcrux crate that needs to destroy secrets (keys, seeds, shared
//! secrets and intermediate values derived from them) goes through the
//! [`Zeroize`] trait and the [`Zeroizing`] wrapper defined here.
//!
//! We use volatile writes to zero the memory. These writes are guaranteed
//! to not be removed by the compiler.
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
#[cfg_attr(hax, hax_lib::attributes)]
pub trait Zeroize {
    /// Overwrite `self` with zeros.
    // Make the precondition visible to callers in F*. Without it, the
    // instances are abstract and callers cannot prove `f_zeroize_pre`.
    #[cfg_attr(hax, hax_lib::requires(true))]
    fn zeroize(&mut self);

    /// Overwrite all elements of `slice` with zeros.
    ///
    /// Types whose zeroization is a plain memory wipe override this to wipe
    /// the whole slice at once.
    // hax supports neither default methods nor this loop over `&mut`.
    #[cfg(not(any(hax, eurydice)))]
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
// Instead, we overwrite the memory with volatile writes, specialized on
// the size of the element to overwrite for better codegen.
//
// Under hax and Eurydice, wiping is a no-op: the proofs do not reason about
// it, and the extracted C code does not zeroize yet.

/// Whether this build wipes secrets.
///
/// `false` if built with `--cfg libcrux_disable_zeroization`, and in code
/// extracted with hax or Eurydice.
pub const ZEROIZATION_ENABLED: bool = !cfg!(any(libcrux_disable_zeroization, hax, eurydice));

/// Fill a slice with a repeated value, preserving its type and alignment.
///
/// This function overwrites `dst` using volatile writes with `src`. These
/// writes are guaranteed to not be removed by the compiler (there can be
/// other optimizations that e.g. lead to copies of the value in the stack).
///
/// To get acceptable performance, the function is internally specialized on the
/// `size_of` T and uses a chunked volatile writes. The straight-forward
/// implementation of looping over a slice and overwriting each element leads
/// to bad code-gen when zeroizing values such `[u8; 64]`, where 64 one byte
/// stores would be emitted.
///
/// The size dispatch and chunking are very likely to be optimized out.
///
/// Adapted from <https://github.com/RustCrypto/utils/blob/77d1a3e6c2e8c28f49189258abb5d4405c0f1330/zeroize/src/bulk.rs#L62-L94>
#[inline(always)]
#[cfg(not(any(libcrux_disable_zeroization, hax, eurydice)))]
fn volatile_fill_slice<T: Copy>(dst: &mut [T], src: T) {
    // Target 16-byte chunks
    match size_of::<T>() {
        0 => {}
        1 => volatile_fill_chunked::<_, 16>(dst, src),
        2 => volatile_fill_chunked::<_, 8>(dst, src),
        4 => volatile_fill_chunked::<_, 4>(dst, src),
        8 => volatile_fill_chunked::<_, 2>(dst, src),
        _ => volatile_fill_iterative(dst, src),
    }
}

/// Prepare the repeated pattern and handle the tail independently of the bulk backend.
#[inline(always)]
#[cfg(not(any(libcrux_disable_zeroization, hax, eurydice)))]
fn volatile_fill_chunked<T: Copy, const N: usize>(dst: &mut [T], src: T) {
    let (chunks, tail) = dst.as_chunks_mut::<N>();

    if !chunks.is_empty() {
        volatile_fill_iterative(chunks, [src; N]);
    }

    volatile_fill_iterative(tail, src);
}

/// Write individual elements or complete chunks with the same volatile primitive.
#[inline(always)]
#[cfg(not(any(libcrux_disable_zeroization, hax, eurydice)))]
fn volatile_fill_iterative<T: Copy>(dst: &mut [T], src: T) {
    for item in dst {
        // SAFETY: `x` is a valid, aligned, exclusive reference.
        unsafe { core::ptr::write_volatile(item, src) }
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
                volatile_fill_slice(core::slice::from_mut(self), $zero);
            }

            #[cfg(not(any(hax, eurydice)))]
            #[inline]
            fn zeroize_slice(slice: &mut [Self]) {
                #[cfg(not(libcrux_disable_zeroization))]
                volatile_fill_slice(slice, $zero);
                #[cfg(libcrux_disable_zeroization)]
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
///
/// Not implemented under Eurydice: wiping is a no-op there, and the C
/// functions would need the intrinsics headers wherever they end up.
#[allow(unused_macros)] // Unused on targets without SIMD registers.
macro_rules! impl_zeroize_simd {
    ($($t:ty),* $(,)?) => {$(
        #[cfg(not(eurydice))]
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
        #[cfg(not(any(hax, eurydice)))]
        T::zeroize_slice(self);
    }
}

impl<T: Zeroize, const N: usize> Zeroize for [T; N] {
    #[inline]
    fn zeroize(&mut self) {
        #[cfg(not(any(hax, eurydice)))]
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

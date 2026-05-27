use core::fmt;

use super::*;

impl<const OUTLEN: usize, Alg: HmacAlgorithm<OUTLEN>> fmt::Debug for HmacDrbg<OUTLEN, Alg> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("HmacDrbg");
        s.field("key", &"<redacted>")
            .field("v", &"<redacted>")
            .field("reseed_counter", &self.reseed_counter);

        #[cfg(feature = "health-tests")]
        s.field("poisoned", &self.health.poisoned());

        s.finish()
    }
}

impl<const OUTLEN: usize, Alg: HmacAlgorithm<OUTLEN>> Drop for HmacDrbg<OUTLEN, Alg> {
    #[allow(unsafe_code)]
    fn drop(&mut self) {
        // Overwrite key material with zeros. Using write_volatile prevents the
        // compiler from optimising away the writes as "dead stores".
        for b in self.key.iter_mut() {
            unsafe { core::ptr::write_volatile(b, 0) };
        }
        for b in self.v.iter_mut() {
            unsafe { core::ptr::write_volatile(b, 0) };
        }
        atomic::compiler_fence(atomic::Ordering::SeqCst);
        // HealthState::Drop zeroes its own output-derived bytes automatically.
    }
}

pub(super) mod private {
    pub trait Sealed {}
}

impl private::Sealed for HmacSha256 {}
impl private::Sealed for HmacSha384 {}
impl private::Sealed for HmacSha512 {}

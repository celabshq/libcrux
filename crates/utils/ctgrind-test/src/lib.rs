//! Helpers shared by the constant-time test binaries.

use crabgrind::memcheck::{self, MemState};
use std::ffi::c_void;

/// Mark `val` as secret. Valgrind reports any branch or memory access that depends on it.
pub fn classify<T: ?Sized>(val: &T) {
    let _ = memcheck::mark_memory(
        val as *const T as *const c_void,
        size_of_val(val),
        MemState::Undefined,
    );
}

/// Mark `val` as public again, e.g. a result that is revealed anyway.
pub fn declassify<T: ?Sized>(val: &T) {
    let _ = memcheck::mark_memory(
        val as *const T as *const c_void,
        size_of_val(val),
        MemState::Defined,
    );
}

/// A deterministic RNG whose output is marked as secret.
pub struct SecretRng(pub u8);

impl rand_core::TryRng for SecretRng {
    type Error = core::convert::Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let mut bytes = [0u8; 4];
        self.try_fill_bytes(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let mut bytes = [0u8; 8];
        self.try_fill_bytes(&mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        for byte in dst.iter_mut() {
            *byte = self.0;
            self.0 = self.0.wrapping_add(1);
        }
        classify(dst);
        Ok(())
    }
}

impl rand_core::TryCryptoRng for SecretRng {}

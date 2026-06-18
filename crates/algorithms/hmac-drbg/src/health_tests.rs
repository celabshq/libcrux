// ---------------------------------------------------------------------------
// Continuous health-test parameters (feature = "health-tests")
// Sourced from NIST SP 800-90B §4.4, Table 4 (8-bit samples, α = 2^{-20}).
// Conservative minimum-entropy assumption H_min = 1 bit/byte is used so that
// the tests catch only truly catastrophic failures while keeping the false-
// positive rate at or below 2^{-20} per test.
// ---------------------------------------------------------------------------

use crate::{HmacAlgorithm, HmacDrbg, APT_C, APT_W, RCT_C, STARTUP_MONOBIT_MIN_BYTES};

/// AIS 31 startup/online tests applied to entropy input at instantiation and reseed.
///
/// Three independent tests, each catching a different failure mode in the entropy source:
///
/// 1. **Non-constant** (total failure): all bytes in `entropy` must not be identical.
///    Catches a completely stuck or zeroed PTRNG.
///
/// 2. **Monobit**: when `entropy.len() >= STARTUP_MONOBIT_MIN_BYTES`, the fraction
///    of set bits must be within `[25 %, 75 %]`. A highly skewed bit distribution
///    indicates severe bias.
///
/// 3. **Run-length** (same cut off [`RCT_C`] as the continuous RCT): no run of ≥
///    `RCT_C` identical bytes. Catches a partially stuck byte lane.
///
/// Returns `true` when any test fails; the caller should return
/// `Err(Error::HealthCheckFailed)` and not use the entropy.
pub(crate) fn startup_test(entropy: &[u8], min_len: usize) -> bool {
    if entropy.len() < min_len.max(1) {
        return true;
    }

    // 1. Non-constant (total failure)
    let first = entropy[0];
    if entropy.iter().all(|&b| b == first) {
        return true;
    }

    // 2. Monobit — skip if the sample is too short to be meaningful
    if entropy.len() >= STARTUP_MONOBIT_MIN_BYTES {
        let ones: u32 = entropy.iter().map(|&b| b.count_ones()).sum();
        let total = (entropy.len() * 8) as u32;
        // Require 25 % ≤ ones/total ≤ 75 %:
        //   ones * 4 < total  ↔  ones < total/4  (< 25 %)
        //   ones * 4 > total * 3  ↔  ones > total*3/4  (> 75 %)
        if ones * 4 < total || ones * 4 > total * 3 {
            return true;
        }
    }

    // 3. Run-length (same cut off as continuous RCT)
    let mut run_byte = entropy[0];
    let mut run_len: u8 = 1;
    for &byte in &entropy[1..] {
        if byte == run_byte {
            run_len = run_len.saturating_add(1);
            if run_len >= RCT_C {
                return true;
            }
        } else {
            run_byte = byte;
            run_len = 1;
        }
    }

    false
}

/// All mutable state needed by the continuous health tests.
///
/// Collected into a single struct so that [`HmacDrbg`] carries exactly one
/// `#[cfg(feature = "health-tests")]` field instead of eight.
pub(crate) struct HealthState<const OUTLEN: usize> {
    /// Last generated V block; used for the inter-call consecutive-block test.
    prev_block: [u8; OUTLEN],
    /// Whether `prev_block` holds a valid value (false after instantiate/reseed).
    prev_block_valid: bool,
    /// Set after any failure; all further DRBG operations return `HealthCheckFailed`.
    poisoned: bool,
    /// RCT: last byte seen in the output stream; meaningful only when `rct_run_len > 0`.
    rct_last_byte: u8,
    /// RCT: current run length of identical bytes; 0 means the stream has not started.
    rct_run_len: u8,
    /// APT: bytes consumed in the current W-byte window; 0 means a fresh window.
    apt_window_pos: u16,
    /// APT: first byte of the current window (the reference value being counted).
    apt_first_byte: u8,
    /// APT: recurrences of `apt_first_byte` in this window, excluding the first byte.
    apt_count: u16,
}

impl<const OUTLEN: usize> HealthState<OUTLEN> {
    pub(crate) fn new() -> Self {
        Self {
            prev_block: [0u8; OUTLEN],
            prev_block_valid: false,
            poisoned: false,
            rct_last_byte: 0,
            rct_run_len: 0,
            apt_window_pos: 0,
            apt_first_byte: 0,
            apt_count: 0,
        }
    }

    /// Reset stream-continuity state after a reseed.
    ///
    /// The V value has changed, so the previous generated block is no longer
    /// a meaningful reference. RCT and APT counters are restarted too.
    pub(crate) fn reset(&mut self) {
        self.prev_block = [0u8; OUTLEN];
        self.prev_block_valid = false;
        self.rct_run_len = 0;
        self.apt_window_pos = 0;
    }

    /// Zeroize bytes that hold output-derived values.
    ///
    /// Called by [`HmacDrbg::zeroize_and_poison`] on failure and by [`Drop`].
    #[allow(unsafe_code)]
    pub(crate) fn zeroize(&mut self) {
        use core::sync::atomic;

        for b in self.prev_block.iter_mut() {
            unsafe { core::ptr::write_volatile(b, 0) };
        }
        unsafe { core::ptr::write_volatile(&mut self.rct_last_byte, 0) };
        unsafe { core::ptr::write_volatile(&mut self.apt_first_byte, 0) };
        atomic::compiler_fence(atomic::Ordering::SeqCst);
    }

    /// Block-level tests run on each generated V block.
    ///
    /// Neither NIST 800-90C  nor AIS 20/31 seem to give clear guidance here. What we do here are
    /// reasonable checks that we can extend later.
    ///
    /// Returns `true` if a test failed (caller must call `zeroize_and_poison` and
    /// return `Error::HealthCheckFailed`).  On success the internal `prev_block` is
    /// updated.
    pub(crate) fn check_block(
        &mut self,
        new_block: &[u8; OUTLEN],
        v_before: &[u8; OUTLEN],
    ) -> bool {
        if new_block == v_before {
            return true;
        }
        if self.prev_block_valid && new_block == &self.prev_block {
            return true;
        }
        self.prev_block = *new_block;
        self.prev_block_valid = true;
        false
    }

    /// Byte-level tests applied to every byte of a generated V block.
    ///
    /// - **RCT** (SP 800-90B §4.4.1): detects runs of ≥ [`RCT_C`] identical bytes.
    /// - **APT** (SP 800-90B §4.4.2): detects high-proportion recurrence within a
    ///   [`APT_W`]-byte window.
    ///
    /// Returns `true` if a test failed (caller must call `zeroize_and_poison` and
    /// return `Error::HealthCheckFailed`).  State is updated on every call.
    pub(crate) fn run_byte_checks(&mut self, data: &[u8]) -> bool {
        for &byte in data {
            // --- Repetition Count Test ---
            if self.rct_run_len == 0 || byte != self.rct_last_byte {
                self.rct_last_byte = byte;
                self.rct_run_len = 1;
            } else {
                self.rct_run_len = self.rct_run_len.saturating_add(1);
                if self.rct_run_len >= RCT_C {
                    return true;
                }
            }

            // --- Adaptive Proportion Test ---
            if self.apt_window_pos == 0 {
                // Start a new window.
                self.apt_first_byte = byte;
                self.apt_count = 0;
                self.apt_window_pos = 1;
            } else {
                if byte == self.apt_first_byte {
                    self.apt_count += 1;
                    if self.apt_count > APT_C {
                        return true;
                    }
                }
                self.apt_window_pos += 1;
                if self.apt_window_pos >= APT_W {
                    self.apt_window_pos = 0; // begin the next window
                }
            }
        }
        false
    }

    pub(crate) fn poisoned(&self) -> bool {
        self.poisoned
    }

    pub(crate) fn set_poisoned(&mut self) {
        self.poisoned = true;
    }
}

impl<const OUTLEN: usize> Drop for HealthState<OUTLEN> {
    fn drop(&mut self) {
        self.zeroize();
    }
}

#[allow(private_bounds)]
impl<const OUTLEN: usize, Alg: HmacAlgorithm<OUTLEN>> HmacDrbg<OUTLEN, Alg> {
    // -----------------------------------------------------------------------
    // Health-test helpers (feature = "health-tests")
    // -----------------------------------------------------------------------

    /// Zeroize key and V, then mark this instance as poisoned.
    ///
    /// Called on any health-test failure.  After this all operations return
    /// `Error::HealthCheckFailed`.  [`HealthState::Drop`] zeroes health-specific
    /// bytes independently.
    #[allow(unsafe_code)]
    pub(super) fn zeroize_and_poison(&mut self) {
        for b in self.key.iter_mut() {
            unsafe { core::ptr::write_volatile(b, 0) };
        }
        for b in self.v.iter_mut() {
            unsafe { core::ptr::write_volatile(b, 0) };
        }
        self.health.zeroize();
        self.health.set_poisoned();
    }

    /// Returns `true` if a health-test failure has been detected and the
    /// instance is permanently unusable.
    pub fn is_poisoned(&self) -> bool {
        self.health.poisoned()
    }

    /// Poison this instance directly (for testing error-path behaviour).
    #[cfg(any(test, feature = "testing"))]
    pub fn poison_for_testing(&mut self) {
        self.health.set_poisoned();
    }
}

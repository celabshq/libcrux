//! NIST SP 800-90A Rev 1 §10.1.2 HMAC_DRBG
//!
//! Supports HMAC-SHA-256, HMAC-SHA-384, and HMAC-SHA-512.
//!
//! # Example
//! ```
//! use libcrux_hmac_drbg::HmacDrbgSha256;
//!
//! // In production obtain entropy from the OS (e.g. `new_from_sys_rng`).
//! let entropy = {
//!     let mut e = [0u8; 32];
//!     for (i, b) in e.iter_mut().enumerate() { *b = if i % 2 == 0 { 0x55 } else { 0xaa }; }
//!     e
//! };
//! let nonce = [0u8; 16];
//! // global domain separation, e.g. app/protocol
//! let personalization = b"myapp.v0";
//! let mut drbg = HmacDrbgSha256::new(&entropy, &nonce, personalization).unwrap();
//! let mut output = [0u8; 32];
//! // per-generate domain separation, e.g. usage of generated entropy inside protocol
//! let additional_input = b"gen-ephemeral";
//! drbg.generate(&mut output, additional_input).unwrap();
//! ```
#![no_std]
#![deny(unsafe_code)]
#![deny(
    missing_docs,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_html_tags,
    rustdoc::private_doc_tests
)]

use core::{marker::PhantomData, sync::atomic};

use libcrux_hmac::HmacState;

#[cfg(test)]
mod tests;

/// Health tests.
/// Enabled with the `health-tests` feature.
#[cfg(feature = "health-tests")]
mod health_tests;
#[cfg(feature = "health-tests")]
use health_tests::*;

/// rand::TryCryptoRng / SeedableRng integration (feature = "rand")
#[cfg(feature = "rand")]
mod rand;
#[cfg(all(feature = "rand", not(feature = "health-tests")))]
pub use rand::{HmacDrbgRng, HmacSha256DrbgRng, HmacSha384DrbgRng, HmacSha512DrbgRng};
#[cfg(feature = "rand")]
pub use rand::{HmacDrbgSeed, TryReseedableRng};

mod errors;
pub use errors::*;

mod utils;

mod hmac;
use hmac::{HmacAlgorithm, HmacSha256, HmacSha384, HmacSha512};

/// Maximum number of generate calls before a reseed is required (SP 800-90A §8.4 Table 1, §10.1 Table 2).
///
/// After this many calls to [`HmacDrbg::generate`], the next call returns
/// [`GenerateError::ReseedRequired`]. Use [`HmacDrbg::needs_reseed`] to check proactively.
pub const RESEED_INTERVAL: u64 = 1 << 48;

/// Maximum number of bytes that can be requested in a single [`HmacDrbg::generate`] call
/// (SP 800-90A §10.1 Tables 1 and 2).
///
/// Requesting more returns [`GenerateError::RequestInvalid`].
pub const MAX_GENERATE_BYTES: usize = 65_536;

/// Minimum entropy input length in bytes (SP 800-90A Table 2: security strength).
///
/// For all supported variants this is 32 bytes (256 bits), regardless of the hash length.
pub const MIN_ENTROPY_BYTES: usize = 32;

// == Constants for health checks ==
// We want these to always be defined, not just when the feature is set.
// That is why we define them here instead of in the module.

/// Repetition Count Test cutoff (SP 800-90B §4.4.1, H_min=1, α=2^{-20}).
/// C = ⌈20 / H_min⌉ + 1 = 21.  A run of ≥ 21 identical bytes signals failure.
pub const RCT_C: u8 = 21;

/// Minimum entropy length in bytes for the startup monobit test to be applied.
pub const STARTUP_MONOBIT_MIN_BYTES: usize = 8;

/// Adaptive Proportion Test window size (SP 800-90B §4.4.2).
pub const APT_W: u16 = 512;

/// Adaptive Proportion Test cut off (SP 800-90B §4.4.2, Table 4, H_min=1, α=2^{-20}).
/// At most 325 recurrences of the first byte are allowed within a 512-byte window.
pub const APT_C: u16 = 325;

// ---------------------------------------------------------------------------
// HMAC_DRBG state
// ---------------------------------------------------------------------------

/// HMAC_DRBG instantiation (SP 800-90A Rev 1 §10.1.2).
///
/// `OUTLEN` is the HMAC output length in bytes (32, 48, or 64).
/// Use the type aliases [`HmacDrbgSha256`], [`HmacDrbgSha384`], [`HmacDrbgSha512`].
#[allow(private_bounds)]
pub struct HmacDrbg<const OUTLEN: usize, Alg: HmacAlgorithm<OUTLEN>> {
    _alg: PhantomData<Alg>,
    key: [u8; OUTLEN],
    v: [u8; OUTLEN],
    reseed_counter: u64,

    /// All continuous health-test state, present only with feature `health-tests`.
    #[cfg(feature = "health-tests")]
    health: HealthState<OUTLEN>,
}

/// HMAC-DRBG instantiated with HMAC-SHA-256.
pub type HmacDrbgSha256 = HmacDrbg<32, HmacSha256>;

/// HMAC-DRBG instantiated with HMAC-SHA-384.
pub type HmacDrbgSha384 = HmacDrbg<48, HmacSha384>;

/// HMAC-DRBG instantiated with HMAC-SHA-512.
pub type HmacDrbgSha512 = HmacDrbg<64, HmacSha512>;

#[allow(private_bounds)]
impl<const OUTLEN: usize, Alg: HmacAlgorithm<OUTLEN>> HmacDrbg<OUTLEN, Alg> {
    // -----------------------------------------------------------------------
    // §10.1.2.2  HMAC_DRBG_Update
    // -----------------------------------------------------------------------

    /// HMAC_DRBG_Update (§10.1.2.2).
    ///
    /// `provided_parts` is a slice of data fragments that are logically
    /// concatenated to form the provided_data argument.  Pass `&[]` when
    /// there is no additional data (equivalent to `provided_data = Null`
    /// in the spec).
    fn update(&mut self, provided_parts: &[&[u8]]) -> Result<(), UpdateError> {
        let has_data = provided_parts.iter().any(|s| !s.is_empty());
        let mut new_v = [0u8; OUTLEN];

        // K = HMAC(K, V || 0x00 [|| provided_data])
        {
            let mut h = Alg::new_hmac(&self.key)?;
            h.update(&self.v)?;
            h.update(&[0x00u8])?;
            if has_data {
                for &part in provided_parts {
                    h.update(part)?;
                }
            }
            h.finalize(&mut self.key);
        }

        // V = HMAC(K, V)
        Alg::hmac(&mut new_v, &self.key, &self.v)?;
        self.v = new_v;

        if has_data {
            // K = HMAC(K, V || 0x01 || provided_data)
            let mut h = Alg::new_hmac(&self.key)?;
            h.update(&self.v)?;
            h.update(&[0x01u8])?;
            for &part in provided_parts {
                h.update(part)?;
            }
            h.finalize(&mut self.key);

            // V = HMAC(K, V)
            Alg::hmac(&mut new_v, &self.key, &self.v)?;
            self.v = new_v;
        }

        Ok(())
    }

    // -----------------------------------------------------------------------
    // §10.1.2.3  Instantiate
    // -----------------------------------------------------------------------

    /// Instantiate an HMAC_DRBG from explicit entropy material (SP 800-90A §10.1.2.3).
    ///
    /// - `entropy_input`: must contain at least [`MIN_ENTROPY_BYTES`] bytes of entropy.
    /// - `nonce`: a unique value (technically a value that is not more likely to repeat than a
    ///   a random value of length OUTLEN/2)
    /// - `personalization_string`: optional application-specific context (may be empty).
    ///
    /// Returns [`InstantiateError::InputTooLarge`] if the combined length of the three
    /// inputs exceeds the maximum input size of the underlying hash function.
    //
    // #hax: requires true
    // #hax: ensures result.is_ok() ==> result.unwrap().reseed_counter == 1
    pub fn new(
        entropy_input: &[u8],
        nonce: &[u8],
        personalization_string: &[u8],
    ) -> Result<Self, InstantiateError> {
        // AIS 31 startup test: validate entropy source before using it.
        #[cfg(feature = "health-tests")]
        if startup_test(entropy_input, MIN_ENTROPY_BYTES) {
            return Err(InstantiateError::HealthCheckFailed);
        }

        #[cfg(not(feature = "health-tests"))]
        if entropy_input.len() < MIN_ENTROPY_BYTES {
            return Err(InstantiateError::InsufficientEntropy);
        }

        let mut drbg = Self {
            _alg: PhantomData,
            key: [0x00u8; OUTLEN],
            v: [0x01u8; OUTLEN],
            reseed_counter: 1,
            #[cfg(feature = "health-tests")]
            health: HealthState::new(),
        };

        // seed_material = entropy_input || nonce || personalization_string
        drbg.update(&[entropy_input, nonce, personalization_string])?;

        Ok(drbg)
    }

    // -----------------------------------------------------------------------
    // §10.1.2.4  Reseed
    // -----------------------------------------------------------------------

    /// Reseed the DRBG with fresh entropy (SP 800-90A §10.1.2.4).
    ///
    /// - `entropy_input`: fresh entropy from a live source (at least [`MIN_ENTROPY_BYTES`]).
    /// - `additional_input`: optional caller-supplied data mixed into the reseed
    ///   (pass `&[]` for none).
    ///
    /// Resets the reseed counter to 1 on success.
    //
    // #hax: requires true
    // #hax: ensures result.is_ok() ==> self.reseed_counter == 1
    pub fn reseed(
        &mut self,
        entropy_input: &[u8],
        additional_input: &[u8],
    ) -> Result<(), ReseedError> {
        #[cfg(feature = "health-tests")]
        if self.health.poisoned() {
            return Err(ReseedError::HealthCheckFailed);
        }

        // AIS 31 startup test: validate entropy source before reseeding.
        #[cfg(feature = "health-tests")]
        if startup_test(entropy_input, MIN_ENTROPY_BYTES) {
            return Err(ReseedError::HealthCheckFailed);
        }

        // seed_material = entropy_input || additional_input
        self.update(&[entropy_input, additional_input])?;
        self.reseed_counter = 1;

        // V has been refreshed; reset the health-test stream state.
        #[cfg(feature = "health-tests")]
        self.health.reset();

        Ok(())
    }

    // -----------------------------------------------------------------------
    // §10.1.2.5  Generate
    // -----------------------------------------------------------------------

    /// Generate pseudorandom bytes into `output` (§10.1.2.5).
    ///
    /// `additional_input` is optional; pass `None` or `Some(&[])` for none.
    /// Returns `Err(ReseedRequired)` if the reseed counter has been exhausted.
    ///
    /// With feature `health-tests`, four continuous tests run on every call:
    /// - **Intra-call** (NIST 800-90C §9.3.3): `V = HMAC(K, V_old)` must satisfy
    ///   `V ≠ V_old`; equality signals a catastrophic HMAC fixed-point.
    /// - **Inter-call / T4** (AIS 20/31): each new block must differ from the last
    ///   block of the previous `generate` call.
    /// - **RCT** (SP 800-90B §4.4.1): no run of ≥ [`RCT_C`] identical bytes.
    /// - **APT** (SP 800-90B §4.4.2): proportion of any byte value within a
    ///   [`APT_W`]-byte window must not exceed [`APT_C`].
    ///
    /// On any failure the instance is poisoned and `Error::HealthCheckFailed` is
    /// returned.
    //
    // #hax: requires output.len() > 0
    // #hax: requires output.len() <= MAX_GENERATE_BYTES
    // #hax: requires true   // additional_input has no length constraint
    // #hax: requires self.reseed_counter <= RESEED_INTERVAL
    // #hax: ensures result.is_ok() ==> self.reseed_counter == old(self.reseed_counter) + 1
    pub fn generate(
        &mut self,
        output: &mut [u8],
        additional_input: &[u8],
    ) -> Result<(), GenerateError> {
        #[cfg(feature = "health-tests")]
        if self.health.poisoned() {
            return Err(GenerateError::HealthCheckFailed);
        }

        if self.reseed_counter > RESEED_INTERVAL {
            return Err(GenerateError::ReseedRequired);
        }
        if output.is_empty() {
            return Err(GenerateError::RequestInvalid);
        }
        if output.len() > MAX_GENERATE_BYTES {
            return Err(GenerateError::RequestInvalid);
        }

        // Update state with additional_input (might be empty).
        if !additional_input.is_empty() {
            self.update(&[additional_input])?;
        }

        // Generate output: repeatedly compute V = HMAC(K, V) and copy into output.
        let mut written = 0;
        while written < output.len() {
            let mut tmp = [0u8; OUTLEN];
            #[cfg(feature = "health-tests")]
            let v_before = self.v;

            Alg::hmac(&mut tmp, &self.key, &self.v)?;

            #[cfg(feature = "health-tests")]
            {
                if self.health.check_block(&tmp, &v_before) || self.health.run_byte_checks(&tmp) {
                    self.zeroize_and_poison();
                    return Err(GenerateError::HealthCheckFailed);
                }
            }

            self.v = tmp;

            let remaining = output.len() - written;
            let chunk = remaining.min(OUTLEN);
            output[written..written + chunk].copy_from_slice(&self.v[..chunk]);
            written += chunk;
        }

        // Final update (with or without additional_input).
        self.update(&[additional_input])?;
        self.reseed_counter += 1;
        Ok(())
    }

    /// Returns `true` if `generate` would immediately return `Err(ReseedRequired)`.
    ///
    /// Callers can use this to proactively schedule a reseed before the counter
    /// is exhausted rather than discovering it on the next `generate` call.
    //
    // #hax: ensures result == (self.reseed_counter > RESEED_INTERVAL)
    pub fn needs_reseed(&self) -> bool {
        self.reseed_counter > RESEED_INTERVAL
    }

    /// Returns the current reseed counter.
    ///
    /// The counter starts at 1 after instantiation or reseed and increments by 1 on each
    /// successful [`generate`] call. When it exceeds [`RESEED_INTERVAL`],
    /// further generation is refused until a reseed.
    ///
    /// [`generate`]: Self::generate
    //
    // #hax: ensures result == self.reseed_counter
    pub fn reseed_counter(&self) -> u64 {
        self.reseed_counter
    }

    /// Force-set the reseed counter (for testing error paths).
    #[cfg(any(test, feature = "testing"))]
    pub fn set_reseed_counter(&mut self, v: u64) {
        self.reseed_counter = v;
    }
}

#[cfg(feature = "rand")]
#[allow(private_bounds)]
impl<const OUTLEN: usize, Alg: HmacAlgorithm<OUTLEN>> HmacDrbg<OUTLEN, Alg> {
    /// Instantiate from a [`rand::TryCryptoRng`], drawing entropy and nonce internally.
    ///
    /// Draws `OUTLEN` bytes of entropy and `OUTLEN` bytes of nonce from `rng`
    /// (conservative; the spec minimum for the nonce is `OUTLEN / 2`).
    ///
    /// Returns [`InstantiateFromRngError::RngError`] if the source RNG fails, or
    /// an instantiation error if the seed material is rejected.
    //
    // #hax: requires personalization_string.len() <= MAX_SEED_BYTES - 2 * OUTLEN
    // #hax: ensures result.is_ok() ==> result.unwrap().reseed_counter == 1
    pub fn new_from_rng<R: rand::TryCryptoRng>(
        rng: &mut R,
        personalization_string: &[u8],
    ) -> Result<Self, InstantiateFromRngError> {
        // XXX: Why is this not MIN_ENTROPY_BYTES??
        let mut entropy = [0u8; OUTLEN];

        let mut nonce = [0u8; OUTLEN];
        rng.try_fill_bytes(&mut entropy)
            .map_err(|_| InstantiateFromRngError::RngError)?;
        rng.try_fill_bytes(&mut nonce)
            .map_err(|_| InstantiateFromRngError::RngError)?;
        Self::new(&entropy, &nonce, personalization_string).map_err(|error| match error {
            InstantiateError::InputTooLarge => InstantiateFromRngError::InputTooLarge,

            #[cfg(feature = "health-tests")]
            InstantiateError::HealthCheckFailed => InstantiateFromRngError::HealthCheckFailed,

            // we provide OUTLEN bytes of entropy, which is enough (>= MIN_ENTROPY_BYTES)
            // XXX: Why is this not MIN_ENTROPY_BYTES??
            #[cfg(not(feature = "health-tests"))]
            InstantiateError::InsufficientEntropy => unreachable!(),
        })
    }

    /// Instantiate from the system RNG ([`rand::rngs::SysRng`]).
    //
    // #hax: requires personalization_string.len() <= MAX_SEED_BYTES - 2 * OUTLEN
    // #hax: ensures result.is_ok() ==> result.unwrap().reseed_counter == 1
    pub fn new_from_sys_rng(
        personalization_string: &[u8],
    ) -> Result<Self, InstantiateFromRngError> {
        Self::new_from_rng(&mut rand::rngs::SysRng, personalization_string)
    }

    /// Reseed from a [`CryptoRng`] (generates entropy internally).
    ///
    /// [`CryptoRng`]: ::rand::CryptoRng
    //
    // #hax: requires additional_input.len() <= MAX_SEED_BYTES - OUTLEN
    // #hax: ensures result.is_ok() ==> self.reseed_counter == 1
    pub fn reseed_from_rng<R: rand::TryCryptoRng>(
        &mut self,
        rng: &mut R,
        additional_input: &[u8],
    ) -> Result<(), ReseedFromRngError<R::Error>> {
        // XXX: Why is this not MIN_ENTROPY_BYTES??
        let mut entropy = [0u8; OUTLEN];

        rng.try_fill_bytes(&mut entropy)
            .map_err(ReseedFromRngError::RngError)?;
        self.reseed(&entropy, additional_input)
            .map_err(|error| match error {
                ReseedError::InputTooLarge => ReseedFromRngError::InputTooLarge,

                #[cfg(feature = "health-tests")]
                ReseedError::HealthCheckFailed => ReseedFromRngError::HealthCheckFailed,

                // we provide OUTLEN bytes of entropy, which is enough (>= MIN_ENTROPY_BYTES)
                // XXX: Why is this not MIN_ENTROPY_BYTES??
                #[cfg(not(feature = "health-tests"))]
                ReseedError::InsufficientEntropy => unreachable!(),
            })
    }
}

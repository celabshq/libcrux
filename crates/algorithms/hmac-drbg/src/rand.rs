use core::marker::PhantomData;

use ::rand::SeedableRng;
pub(crate) use ::rand::{rngs, TryCryptoRng};

#[cfg(not(feature = "health-tests"))]
pub use ::rand::CryptoRng;

use crate::{GenerateError, HmacAlgorithm, HmacDrbg, ReseedError, MAX_GENERATE_BYTES};

#[cfg(feature = "health-tests")]
use super::health_tests::HealthState;

// ---------------------------------------------------------------------------
// Seed type
// ---------------------------------------------------------------------------

/// Seed buffer for [`SeedableRng`] on [`HmacDrbg`].
///
/// A newtype over `[u8; N]` with a manual [`Default`] impl, since
/// `[u8; N]: Default` is only available for N ≤ 32 in the current toolchain.
#[derive(Clone)]
pub struct HmacDrbgSeed<const N: usize>(pub [u8; N]);

impl<const N: usize> Default for HmacDrbgSeed<N> {
    fn default() -> Self {
        Self([0u8; N])
    }
}

impl<const N: usize> AsMut<[u8]> for HmacDrbgSeed<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl<const N: usize> AsRef<[u8]> for HmacDrbgSeed<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const OUTLEN: usize, Alg: HmacAlgorithm<OUTLEN>> rand::TryRng for HmacDrbg<OUTLEN, Alg> {
    type Error = GenerateError;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let mut buf = [0u8; 4];
        self.generate(&mut buf, &[])?;
        Ok(u32::from_le_bytes(buf))
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let mut buf = [0u8; 8];
        self.generate(&mut buf, &[])?;
        Ok(u64::from_le_bytes(buf))
    }

    /// Fill `dst` with pseudorandom bytes.
    ///
    /// Requests larger than [`MAX_GENERATE_BYTES`] are split into chunks so
    /// that arbitrarily-sized buffers can be filled without error.
    //
    // #hax: requires self.reseed_counter + (dst.len() / MAX_GENERATE_BYTES) as u64 + 1 <= RESEED_INTERVAL
    // #hax: ensures result.is_ok() ==> dst is fully written
    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        let mut written = 0;
        while written < dst.len() {
            let chunk = (dst.len() - written).min(MAX_GENERATE_BYTES);
            self.generate(&mut dst[written..written + chunk], &[])?;
            written += chunk;
        }
        Ok(())
    }
}

impl<const OUTLEN: usize, Alg: HmacAlgorithm<OUTLEN>> rand::TryCryptoRng for HmacDrbg<OUTLEN, Alg> {}

/// [`SeedableRng`] implementation for [`HmacDrbg`].
///
/// The seed type is [`HmacDrbgSeed<OUTLEN>`] — 32 bytes for SHA-256,
/// 48 for SHA-384, 64 for SHA-512.
///
/// # Method summary
///
/// | Method | Entropy source | Nonce | Health tests |
/// |--------|---------------|-------|--------------|
/// | [`from_seed`] | caller-supplied (`OUTLEN` B) | none | bypassed |
/// | [`from_rng`] | `OUTLEN` B from `rng` | `OUTLEN` B from `rng` | applied |
/// | [`try_from_rng`] | `OUTLEN` B from `rng` | `OUTLEN` B from `rng` | applied |
///
/// [`from_seed`]: SeedableRng::from_seed
/// [`from_rng`]: SeedableRng::from_rng
/// [`try_from_rng`]: SeedableRng::try_from_rng
///
/// ## `from_seed` and startup health tests
///
/// [`SeedableRng::from_seed`] intentionally bypasses the AIS 31 startup
/// health tests that [`HmacDrbg::new`] applies to the entropy input.
/// `SeedableRng` is a *deterministic* interface: the caller supplies an exact
/// seed, which may include all-zero test vectors.  The continuous health tests
/// that run during [`HmacDrbg::generate`] are still active.
///
/// ## `from_rng` / `try_from_rng` and startup health tests
///
/// [`SeedableRng::from_rng`] and [`SeedableRng::try_from_rng`] draw `OUTLEN`
/// bytes of entropy **and** `OUTLEN` bytes of nonce from the supplied RNG and
/// pass them through [`HmacDrbg::new`], so the AIS 31 startup health tests are
/// applied when the `health-tests` feature is enabled.
///
/// Panics if `new()` fails (a catastrophic event indicating a broken entropy
/// source).
#[allow(private_bounds)]
impl<const OUTLEN: usize, Alg: HmacAlgorithm<OUTLEN>> SeedableRng for HmacDrbg<OUTLEN, Alg> {
    type Seed = HmacDrbgSeed<OUTLEN>;

    // #hax: requires true   // seed is always [u8; OUTLEN]; update() cannot fail for inputs of this size
    // #hax: ensures result.reseed_counter == 1
    fn from_seed(seed: Self::Seed) -> Self {
        let mut drbg = Self {
            _alg: PhantomData,
            key: [0x00u8; OUTLEN],
            v: [0x01u8; OUTLEN],
            reseed_counter: 1,
            #[cfg(feature = "health-tests")]
            health: HealthState::new(),
        };
        // seed is [u8; OUTLEN]; update() with a single non-empty slice always succeeds.
        drbg.update(&[&seed.0])
            .expect("HMAC_DRBG_Update with a seed of length OUTLEN cannot fail");
        drbg
    }

    /// Instantiate from an infallible RNG, drawing entropy and a nonce.
    ///
    /// Consumes `2 × OUTLEN` bytes from `rng` (`OUTLEN` entropy + `OUTLEN`
    /// nonce) and instantiates through [`HmacDrbg::new`].  Unlike
    /// [`from_seed`], this path applies the AIS 31 startup health tests when
    /// the `health-tests` feature is enabled.
    ///
    /// Panics if [`HmacDrbg::new`] fails (catastrophic entropy-source
    /// failure; should never occur with a correctly operating RNG).
    ///
    /// [`from_seed`]: SeedableRng::from_seed
    // #hax: requires true   // entropy=[u8;OUTLEN], nonce=[u8;OUTLEN]: 2*OUTLEN <= MAX_SEED_BYTES always holds
    // #hax: ensures result.reseed_counter == 1
    fn from_rng<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let mut entropy = [0u8; OUTLEN];
        let mut nonce = [0u8; OUTLEN];
        rng.fill_bytes(&mut entropy);
        rng.fill_bytes(&mut nonce);
        Self::new(&entropy, &nonce, &[])
            .expect("HMAC_DRBG_Instantiate from RNG failed: entropy source produced invalid output")
    }

    /// Instantiate from a fallible RNG, drawing entropy and a nonce.
    ///
    /// Consumes `2 × OUTLEN` bytes from `rng` (`OUTLEN` entropy + `OUTLEN`
    /// nonce) and instantiates through [`HmacDrbg::new`].  Unlike
    /// [`from_seed`], this path applies the AIS 31 startup health tests when
    /// the `health-tests` feature is enabled.
    ///
    /// Returns `Err(rng_error)` if `rng` fails.
    ///
    /// # Warning — trait limitation
    ///
    /// The [`SeedableRng`] trait fixes the error type of this method to
    /// `R::Error` (the RNG's own error type).  It is therefore **impossible**
    /// to forward
    #[cfg_attr(
        feature = "health-tests",
        doc = "[`InstantiateError::HealthCheckFailed`]"
    )]
    #[cfg_attr(
        not(feature = "health-tests"),
        doc = "`InstantiateError::HealthCheckFailed`"
    )]
    /// — or any other
    /// [`InstantiateError`] variant — through this return type without an
    /// `R::Error: From<Error>` bound, which cannot be added to a trait method
    /// override.
    ///
    /// As a consequence, if `feature = "health-tests"` is enabled and the
    /// entropy source fails the AIS 31 startup test, this method **panics**
    /// rather than returning an error.  If you need non-panicking, fully
    /// error-propagating instantiation, use [`HmacDrbg::new_from_rng`]
    /// directly; it returns `Result<Self, Error>` and propagates all failure
    /// modes.
    ///
    /// [`from_seed`]: SeedableRng::from_seed
    /// [`InstantiateError`]: crate::errors::InstantiateError
    #[cfg_attr(
        feature = "health-tests",
        doc = "[`InstantiateError::HealthCheckFailed`]: crate::errors::InstantiateError::HealthCheckFailed"
    )]
    // #hax: requires true   // entropy=[u8;OUTLEN], nonce=[u8;OUTLEN]: 2*OUTLEN <= MAX_SEED_BYTES always holds
    // #hax: ensures result.is_ok() ==> result.unwrap().reseed_counter == 1
    fn try_from_rng<R: rand::TryRng + ?Sized>(rng: &mut R) -> Result<Self, R::Error> {
        let mut entropy = [0u8; OUTLEN];
        let mut nonce = [0u8; OUTLEN];
        rng.try_fill_bytes(&mut entropy)?;
        rng.try_fill_bytes(&mut nonce)?;
        Ok(Self::new(&entropy, &nonce, &[]).expect(
            "HMAC_DRBG_Instantiate from RNG failed: entropy source produced invalid output",
        ))
    }
}

// ---------------------------------------------------------------------------
// Reseedable Rng Trait
// ---------------------------------------------------------------------------

/// Extension trait for an RNG that supports explicit reseeding with caller-supplied entropy.
///
/// This is separate from [`SeedableRng`] because reseeding an already-running
/// DRBG takes entropy + additional input, whereas `SeedableRng` only covers initial
/// construction.
pub trait TryReseedableRng: rand::TryRng {
    /// The seed/entropy type accepted by [`reseed`].
    ///
    /// [`reseed`]: Self::reseed
    ///
    /// For [`HmacDrbg`] this is [`HmacDrbgSeed<OUTLEN>`].
    type Entropy: AsRef<[u8]>;

    /// The error that the reseeding operation returns.
    type ReseedError: core::error::Error;

    /// The method that reseeds the RNG with the given entropy and additional input
    fn reseed(
        &mut self,
        entropy_input: &Self::Entropy,
        additional_input: &[u8],
    ) -> Result<(), Self::ReseedError>;
}

impl<const OUTLEN: usize, Alg: HmacAlgorithm<OUTLEN>> TryReseedableRng for HmacDrbg<OUTLEN, Alg> {
    type Entropy = HmacDrbgSeed<OUTLEN>;

    type ReseedError = ReseedError;

    fn reseed(
        &mut self,
        entropy: &Self::Entropy,
        additional_input: &[u8],
    ) -> Result<(), Self::ReseedError> {
        self.reseed(entropy.as_ref(), additional_input)
    }
}

// ---------------------------------------------------------------------------
// HmacDrbgRng — auto-reseeding, infallible CryptoRng wrapper
// ---------------------------------------------------------------------------

/// Auto-reseeding, infallible HMAC-DRBG RNG using HMAC-SHA-256.
#[cfg(not(feature = "health-tests"))]
pub type HmacSha256DrbgRng<ReseedRng> = HmacDrbgRng<32, crate::hmac::HmacSha256, ReseedRng>;
/// Auto-reseeding, infallible HMAC-DRBG RNG using HMAC-SHA-384.
#[cfg(not(feature = "health-tests"))]
pub type HmacSha384DrbgRng<ReseedRng> = HmacDrbgRng<48, crate::hmac::HmacSha384, ReseedRng>;
/// Auto-reseeding, infallible HMAC-DRBG RNG using HMAC-SHA-512.
#[cfg(not(feature = "health-tests"))]
pub type HmacSha512DrbgRng<ReseedRng> = HmacDrbgRng<64, crate::hmac::HmacSha512, ReseedRng>;

/// Auto-reseeding, infallible [`rand::CryptoRng`] wrapper around [`HmacDrbg`].
///
/// Unlike [`HmacDrbg`] (which implements the fallible [`rand::TryCryptoRng`]),
/// `HmacDrbgRng` implements the infallible [`rand::CryptoRng`] by automatically
/// reseeding from an inner `ReseedRng` when needed.
///
/// Only available without `feature = "health-tests"`, because health-test failures
/// cannot be expressed through the infallible `CryptoRng` interface.
///
/// Use the type aliases [`HmacSha256DrbgRng`], [`HmacSha384DrbgRng`],
/// [`HmacSha512DrbgRng`] for convenience.
#[cfg(not(feature = "health-tests"))]
#[allow(private_bounds)]
pub struct HmacDrbgRng<const OUTLEN: usize, Hmac: HmacAlgorithm<OUTLEN>, ReseedRng: CryptoRng> {
    drbg: HmacDrbg<OUTLEN, Hmac>,
    rng: ReseedRng,
}

#[cfg(not(feature = "health-tests"))]
#[allow(private_bounds)]
impl<const OUTLEN: usize, Hmac: HmacAlgorithm<OUTLEN>, ReseedRng: CryptoRng>
    HmacDrbgRng<OUTLEN, Hmac, ReseedRng>
{
    /// Instantiate a new `HmacDrbgRng`, sampling entropy and nonce from the `ReseedRng`.
    ///
    /// `personalization` is a 32-byte application-specific label mixed into the
    /// initial seed material (use `&[0u8; 32]` if not needed). The `rng` is kept
    /// for automatic reseeding on subsequent generate calls.
    pub fn new(mut rng: ReseedRng, personalization: &[u8; 32]) -> Self {
        let mut init_entropy = [0u8; crate::MIN_ENTROPY_BYTES];
        let mut nonce = [0u8; 32];
        rng.fill_bytes(&mut init_entropy);
        rng.fill_bytes(&mut nonce);

        Self::new_from_seed(rng, &init_entropy, &nonce, personalization)
    }

    /// Instantiates a new `HmacDrbgRng` from explicit entropy and nonce.
    ///
    /// The `ReseedRng` is only used for automatic reseeding, not for initial
    /// seeding.
    pub fn new_from_seed(
        rng: ReseedRng,
        entropy: &[u8; crate::MIN_ENTROPY_BYTES],
        nonce: &[u8; 32],
        personalization: &[u8; 32],
    ) -> Self {
        let drbg = match HmacDrbg::new(entropy, nonce, personalization) {
            Ok(drbg) => drbg,
            Err(error) => match error {
                // We know how much data we inject and it's fine
                crate::InstantiateError::InputTooLarge => unreachable!(),
                // the provided entropy is MIN_ENTROPY_BYTES, so it is enough
                crate::InstantiateError::InsufficientEntropy => unreachable!(),
            },
        };

        Self { drbg, rng }
    }

    /// Somewhat safely generates a bit of randomness:
    ///  - ensures we don't pass in too much additional_input (none, in fact)
    ///  - reseeds if needed
    ///
    /// The caller MUST ensure that they don't ask for too much data. Ideally enforce this using
    /// hax.
    ///
    ///
    //hax: requires dst.len() < crate::MAX_GENERATE_BYTES
    fn safe_generate_small(&mut self, dst: &mut [u8]) {
        if self.drbg.needs_reseed() {
            match self.drbg.reseed_from_rng(&mut self.rng, &[]) {
                Ok(()) => (),
                // we know how much data we put in and it's fine
                Err(crate::ReseedFromRngError::InputTooLarge) => unreachable!(),

                // this Rng is infallible, so this can't happen
                Err(crate::ReseedFromRngError::RngError(error)) => match error {},
            }
        }

        match self.drbg.generate(dst, &[]) {
            Ok(()) => (),
            // we just ensured that no reseed is required
            Err(crate::GenerateError::ReseedRequired) => unreachable!(),
            // We know how much data we request and it's fine
            Err(crate::GenerateError::RequestInvalid) => unreachable!(),
            // Again, the input size is safe.
            Err(crate::GenerateError::InputTooLarge) => unreachable!(),
        }
    }
}

#[cfg(not(feature = "health-tests"))]
impl<const OUTLEN: usize, Hmac: HmacAlgorithm<OUTLEN>, ReseedRng: CryptoRng> rand::TryRng
    for HmacDrbgRng<OUTLEN, Hmac, ReseedRng>
{
    type Error = core::convert::Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let mut out = [0u8; 4];
        self.safe_generate_small(&mut out);
        Ok(u32::from_le_bytes(out))
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let mut out = [0u8; 8];
        self.safe_generate_small(&mut out);
        Ok(u64::from_le_bytes(out))
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        let (chunks, rest): (&mut [[u8; MAX_GENERATE_BYTES]], _) = dst.as_chunks_mut();

        for chunk in chunks {
            self.safe_generate_small(chunk);
        }

        self.safe_generate_small(rest);

        Ok(())
    }
}

#[cfg(not(feature = "health-tests"))]
impl<const OUTLEN: usize, Hmac: HmacAlgorithm<OUTLEN>, ReseedRng: CryptoRng> rand::TryCryptoRng
    for HmacDrbgRng<OUTLEN, Hmac, ReseedRng>
{
}

/// Ensure that HmacDrbgRng implements CryptoRng
#[cfg(not(feature = "health-tests"))]
fn _assert_crypto_rng<const OUTLEN: usize, Hmac: HmacAlgorithm<OUTLEN>, ReseedRng: CryptoRng>() {
    fn must_impl<T: CryptoRng>() {}
    must_impl::<HmacDrbgRng<OUTLEN, Hmac, ReseedRng>>();
}

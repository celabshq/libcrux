use core::{error::Error, fmt};

#[cfg(doc)]
use crate::{MAX_GENERATE_BYTES, RESEED_INTERVAL};

/// The Error returned by the Update operation of HMAC-DRBG.
#[derive(Debug, PartialEq)]
pub enum UpdateError {
    /// The combined seed material exceeds the internal limit.
    InputTooLarge,
}

/// The Error returned by the Reseed operation of HMAC-DRBG.
#[derive(Debug, PartialEq)]
pub enum ReseedError {
    /// The combined seed material exceeds the internal limit.
    InputTooLarge,

    /// The entropy provided is insufficient.
    #[cfg(not(feature = "health-tests"))]
    InsufficientEntropy,

    /// A continuous health test detected a catastrophic internal failure
    /// (feature `health-tests`). The DRBG is permanently poisoned; discard
    /// the instance. This should never occur in correct operation.
    ///
    /// Note that this error can only occur when enabling the feature.
    #[cfg(feature = "health-tests")]
    HealthCheckFailed,
}

/// The Error returned by the Instantiate operation of HMAC-DRBG.
#[derive(Debug, PartialEq)]
pub enum InstantiateError {
    /// The combined seed material exceeds the internal limit.
    InputTooLarge,

    /// The entropy provided is insufficient.
    #[cfg(not(feature = "health-tests"))]
    InsufficientEntropy,

    /// A continuous health test detected a catastrophic internal failure
    /// (feature `health-tests`). The DRBG is permanently poisoned; discard
    /// the instance. This should never occur in correct operation.
    ///
    /// Note that this error can only occur when enabling the feature.
    #[cfg(feature = "health-tests")]
    HealthCheckFailed,
}

/// The Error returned by the Instantiate operation of HMAC-DRBG, when used with an Rng.
#[cfg(feature = "rand")]
#[derive(Debug, PartialEq)]
pub enum InstantiateFromRngError {
    /// The combined seed material exceeds the internal limit.
    InputTooLarge,

    /// The Rng used for seeding failed.
    RngError,

    /// A continuous health test detected a catastrophic internal failure
    /// (feature `health-tests`). The DRBG is permanently poisoned; discard
    /// the instance. This should never occur in correct operation.
    ///
    /// Note that this error can only occur when enabling the feature.
    #[cfg(feature = "health-tests")]
    HealthCheckFailed,
}

/// The Error returned by the Reseed operation of HMAC-DRBG.ha, when used with an Rng
#[cfg(feature = "rand")]
#[derive(Debug, PartialEq)]
pub enum ReseedFromRngError<RngError> {
    /// The combined seed material exceeds the internal limit.
    InputTooLarge,

    /// The Rng used for seeding failed.
    RngError(RngError),

    /// A continuous health test detected a catastrophic internal failure
    /// (feature `health-tests`). The DRBG is permanently poisoned; discard
    /// the instance. This should never occur in correct operation.
    ///
    /// Note that this error can only occur when enabling the feature.
    #[cfg(feature = "health-tests")]
    HealthCheckFailed,
}

/// Errors returned by [`HmacDrbg::generate`].
///
/// [`HmacDrbg::generate`]: crate::HmacDrbg::generate
#[derive(Debug, PartialEq)]
pub enum GenerateError {
    /// The reseed counter has exceeded [`RESEED_INTERVAL`]; call `reseed` before generating.
    ///
    /// [`RESEED_INTERVAL`]: crate::RESEED_INTERVAL
    ReseedRequired,

    /// The requested output length is zero or exceeds [`MAX_GENERATE_BYTES`].
    ///
    /// [`MAX_GENERATE_BYTES`]: crate::MAX_GENERATE_BYTES
    RequestInvalid,

    /// The combined seed material exceeds the internal limit.
    InputTooLarge,

    /// A continuous health test detected a catastrophic internal failure
    /// (feature `health-tests`). The DRBG is permanently poisoned; discard
    /// the instance. This should never occur in correct operation.
    ///
    /// Note that this error can only occur when enabling the feature.
    #[cfg(feature = "health-tests")]
    HealthCheckFailed,
}

impl Error for GenerateError {}
impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateError::ReseedRequired => f.write_str("reseed required before next generate"),
            GenerateError::RequestInvalid => f.write_str("generate request size out of range"),
            GenerateError::InputTooLarge => {
                f.write_str("seed material exceeds maximum allowed size")
            }
            #[cfg(feature = "health-tests")]
            GenerateError::HealthCheckFailed => {
                f.write_str("continuous health test failed: DRBG is permanently poisoned")
            }
        }
    }
}

impl Error for ReseedError {}
impl fmt::Display for ReseedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ReseedError::InputTooLarge => f.write_str("seed material exceeds maximum allowed size"),

            #[cfg(not(feature = "health-tests"))]
            ReseedError::InsufficientEntropy => f.write_str("provided entropy is insufficient"),

            #[cfg(feature = "health-tests")]
            ReseedError::HealthCheckFailed => {
                f.write_str("continuous health test failed: DRBG is permanently poisoned")
            }
        }
    }
}

impl Error for InstantiateError {}
impl fmt::Display for InstantiateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstantiateError::InputTooLarge => {
                f.write_str("seed material exceeds maximum allowed size")
            }
            #[cfg(not(feature = "health-tests"))]
            InstantiateError::InsufficientEntropy => {
                f.write_str("provided entropy is insufficient")
            }
            #[cfg(feature = "health-tests")]
            InstantiateError::HealthCheckFailed => {
                f.write_str("continuous health test failed: DRBG is permanently poisoned")
            }
        }
    }
}

#[cfg(feature = "rand")]
impl Error for InstantiateFromRngError {}
#[cfg(feature = "rand")]
impl fmt::Display for InstantiateFromRngError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstantiateFromRngError::InputTooLarge => {
                f.write_str("seed material exceeds maximum allowed size")
            }
            InstantiateFromRngError::RngError => f.write_str("RNG source failed"),
            #[cfg(feature = "health-tests")]
            InstantiateFromRngError::HealthCheckFailed => {
                f.write_str("continuous health test failed: DRBG is permanently poisoned")
            }
        }
    }
}

impl Error for UpdateError {}
impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateError::InputTooLarge => f.write_str("seed material exceeds maximum allowed size"),
        }
    }
}

impl From<UpdateError> for InstantiateError {
    fn from(error: UpdateError) -> Self {
        match error {
            UpdateError::InputTooLarge => InstantiateError::InputTooLarge,
        }
    }
}

impl From<UpdateError> for ReseedError {
    fn from(error: UpdateError) -> Self {
        match error {
            UpdateError::InputTooLarge => ReseedError::InputTooLarge,
        }
    }
}

impl From<UpdateError> for GenerateError {
    fn from(_: UpdateError) -> Self {
        Self::InputTooLarge
    }
}

impl From<super::hmac::Error> for GenerateError {
    fn from(err: super::hmac::Error) -> Self {
        match err {
            super::hmac::Error::InputTooLarge => GenerateError::InputTooLarge,
        }
    }
}

impl From<super::hmac::Error> for UpdateError {
    fn from(err: super::hmac::Error) -> Self {
        match err {
            super::hmac::Error::InputTooLarge => UpdateError::InputTooLarge,
        }
    }
}

impl From<libcrux_hmac::Error> for UpdateError {
    fn from(error: libcrux_hmac::Error) -> Self {
        match error {
            libcrux_hmac::Error::InvalidInputLength => UpdateError::InputTooLarge,
        }
    }
}

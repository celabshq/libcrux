//! # P-384 Private Keys
//!
//! Private keys are field elements in Fp which are bounded by the
//! curve order of P-384.

#[cfg(feature = "rand")]
use rand::TryCryptoRng;

#[cfg(feature = "rand")]
use crate::RandomnessError;
use crate::{
    constants::{FP_NUM_BYTES, NIST_P384_CURVE_ORDER_BE_BYTES},
    util::{be_bytes_lt, be_bytes_nonzero},
};

/// A P-384 private key.
pub struct PrivateKey(
    /// This is a big-endian encoding of an integer x with 0 < x < NIST_P384_CURVE_ORDER.
    pub(crate) [u8; FP_NUM_BYTES],
);

impl AsRef<[u8; FP_NUM_BYTES]> for PrivateKey {
    fn as_ref(&self) -> &[u8; FP_NUM_BYTES] {
        &self.0
    }
}

#[cfg(feature = "rand")]
/// How many times to retry sampling a valid scalar during key generation.
const SCALAR_REJ_SAMPLING_BOUND: usize = 5;

impl PrivateKey {
    #[cfg(feature = "rand")]
    /// Generate a fresh private key.
    ///
    /// Performs rejection sampling internally, and may return an error if
    /// rejection sampling does not succeed in a particular number of
    /// attempts, indicating a major failure of randomness generation.
    pub fn generate(rng: &mut impl TryCryptoRng) -> Result<Self, RandomnessError> {
        let mut bytes = [0u8; FP_NUM_BYTES];
        let mut attempts = 0;

        while attempts < SCALAR_REJ_SAMPLING_BOUND {
            rng.try_fill_bytes(&mut bytes)
                .map_err(|_| RandomnessError)?;
            if let Ok(key) = Self::try_from(bytes.as_slice()) {
                return Ok(key);
            }
            attempts += 1;
        }
        Err(RandomnessError)
    }
}

impl TryFrom<&[u8]> for PrivateKey {
    type Error = crate::EcdhError;

    /// A valid SEC1 encoding of a P-384 private key is a 48 byte
    /// big-endian integer that is less than the P-384 curve order.
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let value: [u8; FP_NUM_BYTES] = value
            .try_into()
            .map_err(|_| Self::Error::InvalidPrivateKey)?;
        if core::hint::black_box(be_bytes_nonzero(&value))
            & core::hint::black_box(be_bytes_lt(&value, &NIST_P384_CURVE_ORDER_BE_BYTES))
        {
            Ok(Self(value))
        } else {
            Err(Self::Error::InvalidPrivateKey)
        }
    }
}

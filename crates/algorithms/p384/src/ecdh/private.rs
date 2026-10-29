//! # P-384 Private Keys
//!
//! Private keys are field elements in Fp which are bounded by the
//! curve order of P-384.

#[cfg(feature = "rand")]
use rand::TryCryptoRng;

#[cfg(feature = "rand")]
use crate::{
    constants::NIST_P384_CURVE_ORDER_MINUS_ONE_BE_BYTES, util::be_bytes_add_one, RandomnessError,
};
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
    /// Uses the rejection sampling method of FIPS 186-5, Appendix A.2.2: a
    /// candidate `c` of 384 random bits is rejected if `c > n - 2`, and the
    /// private key is `d = c + 1`.
    ///
    /// Returns an error if rejection sampling does not succeed in a
    /// particular number of attempts, indicating a major failure of
    /// randomness generation.
    pub fn generate(rng: &mut impl TryCryptoRng) -> Result<Self, RandomnessError> {
        let mut c = [0u8; FP_NUM_BYTES];
        let mut attempts = 0;

        while attempts < SCALAR_REJ_SAMPLING_BOUND {
            rng.try_fill_bytes(&mut c).map_err(|_| RandomnessError)?;
            // Accept if c <= n - 2. The comparison is constant time (see
            // `be_bytes_lt`) and reveals only its result. Whether a candidate
            // is accepted, and so the number of attempts, is public: each
            // candidate is an independent draw from the CSPRNG, so the
            // rejected ones say nothing about the accepted one, which is
            // uniform on [0, n - 2] whatever the number of attempts.
            // `be_bytes_add_one` is branch-free, so the value of the accepted
            // key does not leak.
            if be_bytes_lt(&c, &NIST_P384_CURVE_ORDER_MINUS_ONE_BE_BYTES) {
                be_bytes_add_one(&mut c);
                return Ok(Self(c));
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

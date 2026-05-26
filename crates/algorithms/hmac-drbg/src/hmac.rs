use libcrux_hmac::{hmac_slices, HmacState};

use super::utils;

/// The Error returned by the Update operation of HMAC-DRBG.
pub enum Error {
    /// The combined seed material exceeds the internal limit.
    InputTooLarge,
}

// ---------------------------------------------------------------------------
// Algorithm trait
// ---------------------------------------------------------------------------

/// Trait implemented by the three marker types below.
/// Associates a const `OUTLEN` with the HMAC computation for that hash.
pub(super) trait HmacAlgorithm<const OUTLEN: usize>: utils::private::Sealed {
    type State: HmacState<OUTLEN>;

    /// Single shot HMAC.
    ///
    /// Returns an [`InputTooLarge`] when the input is too long.
    fn hmac(dst: &mut [u8; OUTLEN], key: &[u8], data: &[u8]) -> Result<(), Error> {
        hmac_slices::<OUTLEN, Self::State>(dst, key, &[data]).map_err(|_| Error::InputTooLarge)
    }

    /// New HMAC streaming state.
    ///
    /// Returns the [`Self::State`] or an [`InputTooLarge`] if the `key` is too long.
    fn new_hmac(key: &[u8]) -> Result<Self::State, Error> {
        Self::State::new(key).map_err(|_| Error::InputTooLarge)
    }
}

/// Marker type selecting HMAC-SHA-256.
#[derive(Debug)]
pub struct HmacSha256;

/// Marker type selecting HMAC-SHA-384.
#[derive(Debug)]
pub struct HmacSha384;

/// Marker type selecting HMAC-SHA-512.
#[derive(Debug)]
pub struct HmacSha512;

impl HmacAlgorithm<32> for HmacSha256 {
    type State = libcrux_hmac::HmacSha256;
}

impl HmacAlgorithm<48> for HmacSha384 {
    type State = libcrux_hmac::HmacSha384;
}

impl HmacAlgorithm<64> for HmacSha512 {
    type State = libcrux_hmac::HmacSha512;
}

#![no_std]

pub use libcrux_hacl_rs::curve25519_51 as hacl;

mod impl_hacl;

pub mod ecdh_api;

pub use impl_hacl::{ecdh, secret_to_public};
use libcrux_secrets::{DeclassifyRef, DeclassifyRefMut, U8};

/// The length of Curve25519 secret keys.
pub const DK_LEN: usize = 32;

/// The length of Curve25519 public keys.
pub const EK_LEN: usize = 32;

/// The length of Curve25519 shared keys.
pub const SS_LEN: usize = 32;

/// Indicates that an error occurred
pub struct Error;

/// This trait is implemented by the backing implementations.
/// Only used for ensuring implementations follow the same interface, not really consumed.
#[allow(dead_code)]
trait Curve25519 {
    /// Computes a public key from a secret key.
    fn secret_to_public(pk: &mut [u8; EK_LEN], sk: &[u8; DK_LEN]);

    /// Computes the scalar multiplication between the provided public and secret keys. Returns an
    /// error if the result is 0.
    fn ecdh(out: &mut [u8; SS_LEN], pk: &[u8; EK_LEN], sk: &[u8; DK_LEN]) -> Result<(), Error>;
}

pub struct X25519;

impl libcrux_traits::kem::arrayref::Kem<DK_LEN, EK_LEN, EK_LEN, SS_LEN, DK_LEN, DK_LEN> for X25519 {
    type KeyGenError = libcrux_traits::kem::arrayref::KeyGenError;
    type EncapsError = libcrux_traits::kem::arrayref::EncapsError;
    type DecapsError = libcrux_traits::kem::arrayref::DecapsError;

    fn keygen(
        ek: &mut [u8; DK_LEN],
        dk: &mut [U8; EK_LEN],
        rand: &[U8; DK_LEN],
    ) -> Result<(), Self::KeyGenError> {
        dk.copy_from_slice(rand);
        clamp(dk.declassify_ref_mut());
        secret_to_public(ek, dk.declassify_ref());
        Ok(())
    }

    fn encaps(
        ct: &mut [u8; EK_LEN],
        ss: &mut [U8; SS_LEN],
        ek: &[u8; EK_LEN],
        rand: &[U8; DK_LEN],
    ) -> Result<(), Self::EncapsError> {
        let mut eph_dk = *rand;
        clamp(eph_dk.declassify_ref_mut());
        secret_to_public(ct, eph_dk.declassify_ref());

        ecdh(ss.declassify_ref_mut(), ek, eph_dk.declassify_ref())
            .map_err(|_| libcrux_traits::kem::arrayref::EncapsError::Unknown)
    }

    fn decaps(
        ss: &mut [U8; SS_LEN],
        ct: &[u8; DK_LEN],
        dk: &[U8; EK_LEN],
    ) -> Result<(), Self::DecapsError> {
        ecdh(ss.declassify_ref_mut(), ct, dk.declassify_ref())
            .map_err(|_| libcrux_traits::kem::arrayref::DecapsError::Unknown)
    }
}

libcrux_traits::kem::slice::impl_trait!(X25519 => EK_LEN, DK_LEN, EK_LEN, EK_LEN, DK_LEN, DK_LEN);

/// Clamp a scalar.
pub fn clamp(scalar: &mut [u8; DK_LEN]) {
    // We clamp the key already to make sure it can't be misused.
    scalar[0] &= 248u8;
    scalar[31] &= 127u8;
    scalar[31] |= 64u8;
}

/// Test whether a scalar is already clamped, error indicating that it is not.
pub fn is_clamped(scalar: &[u8; DK_LEN]) -> Result<(), Error> {
    if scalar[0] & 7 != 0 || scalar[31] & 128 != 0 || scalar[31] & 64 != 64 {
        Err(Error)
    } else {
        Ok(())
    }
}

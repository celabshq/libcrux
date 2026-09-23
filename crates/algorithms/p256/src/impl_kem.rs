use libcrux_secrets::{DeclassifyRef, DeclassifyRefMut, U8};
use libcrux_traits::kem::arrayref::*;

use super::{POINT_LEN, SCALAR_LEN};

// The sorts of data that the Kem trait needs
const EK_LEN: usize = POINT_LEN;
const CT_LEN: usize = POINT_LEN;
const SS_LEN: usize = POINT_LEN;

const DK_LEN: usize = SCALAR_LEN;
const RAND_KEYGEN_LEN: usize = SCALAR_LEN;
const RAND_ENCAPS_LEN: usize = SCALAR_LEN;

impl Kem<EK_LEN, DK_LEN, CT_LEN, SS_LEN, RAND_KEYGEN_LEN, RAND_ENCAPS_LEN> for super::P256 {
    type KeyGenError = KeyGenError;
    type EncapsError = EncapsError;
    type DecapsError = DecapsError;

    fn keygen(
        ek: &mut [u8; EK_LEN],
        dk: &mut [U8; DK_LEN],
        rand: &[U8; RAND_KEYGEN_LEN],
    ) -> Result<(), Self::KeyGenError> {
        if !super::p256::validate_private_key(rand.as_slice().declassify_ref()) {
            return Err(KeyGenError::InvalidRandomness);
        }

        dk.copy_from_slice(rand.as_slice());
        if !super::p256::dh_initiator(ek, dk.declassify_ref_mut()) {
            return Err(KeyGenError::Unknown);
        }

        Ok(())
    }

    fn encaps(
        ct: &mut [u8; CT_LEN],
        ss: &mut [U8; SS_LEN],
        ek: &[u8; EK_LEN],
        rand: &[U8; RAND_ENCAPS_LEN],
    ) -> Result<(), Self::EncapsError> {
        if !super::p256::validate_public_key(ek) {
            return Err(EncapsError::InvalidEncapsKey);
        }

        if !super::p256::validate_private_key(rand.as_slice().declassify_ref()) {
            return Err(EncapsError::InvalidRandomness);
        }

        if !super::p256::dh_responder(ss.declassify_ref_mut(), ek, rand.declassify_ref()) {
            return Err(EncapsError::Unknown);
        }

        if !super::p256::dh_initiator(ct, rand.declassify_ref()) {
            return Err(EncapsError::Unknown);
        }

        Ok(())
    }

    fn decaps(
        ss: &mut [U8; SS_LEN],
        ct: &[u8; CT_LEN],
        dk: &[U8; DK_LEN],
    ) -> Result<(), Self::DecapsError> {
        if !super::p256::validate_public_key(ct) {
            return Err(DecapsError::InvalidCiphertext);
        }

        if !super::p256::validate_private_key(dk.as_slice().declassify_ref()) {
            return Err(DecapsError::InvalidDecapsKey);
        }

        if !super::p256::dh_responder(ss.declassify_ref_mut(), ct, dk.declassify_ref()) {
            return Err(DecapsError::Unknown);
        }

        Ok(())
    }
}

libcrux_traits::kem::slice::impl_trait!(super::P256 => EK_LEN, DK_LEN, CT_LEN, SS_LEN, RAND_KEYGEN_LEN, RAND_ENCAPS_LEN);

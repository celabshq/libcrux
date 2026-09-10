use super::{POINT_LEN, SCALAR_LEN};

pub const RAND_LEN: usize = SCALAR_LEN;
pub const SECRET_LEN: usize = SCALAR_LEN;
pub const PUBLIC_LEN: usize = POINT_LEN;

pub use libcrux_traits::ecdh::{arrayref::EcdhArrayref, owned::EcdhOwned, slice::EcdhSlice};

use libcrux_secrets::{Declassify, DeclassifyRef, DeclassifyRefMut, U8};

impl libcrux_traits::ecdh::arrayref::EcdhArrayref<RAND_LEN, SECRET_LEN, PUBLIC_LEN>
    for super::P256
{
    fn generate_secret(
        secret: &mut [U8; SECRET_LEN],
        rand: &[U8; RAND_LEN],
    ) -> Result<(), libcrux_traits::ecdh::arrayref::GenerateSecretError> {
        secret.copy_from_slice(rand);
        <Self as libcrux_traits::ecdh::arrayref::EcdhArrayref<RAND_LEN, SECRET_LEN, PUBLIC_LEN>>::validate_secret(secret).map_err(|_| libcrux_traits::ecdh::arrayref::GenerateSecretError::Unknown)
    }

    fn secret_to_public(
        public: &mut [u8; PUBLIC_LEN],
        secret: &[U8; SECRET_LEN],
    ) -> Result<(), libcrux_traits::ecdh::arrayref::SecretToPublicError> {
        // `crate::p256::dh_initiator` returns false if `secret` is not valid
        crate::p256::dh_initiator(public, secret.declassify_ref().as_slice())
            .then_some(())
            .ok_or(libcrux_traits::ecdh::arrayref::SecretToPublicError::InvalidSecret)
    }

    fn derive_ecdh(
        derived: &mut [U8; PUBLIC_LEN],
        public: &[u8; PUBLIC_LEN],
        secret: &[U8; SECRET_LEN],
    ) -> Result<(), libcrux_traits::ecdh::arrayref::DeriveError> {
        // `crate::p256::dh_responder` checks the validity of `public` and `private`
        crate::p256::dh_responder(
            derived.declassify_ref_mut().as_mut_slice(),
            public.as_ref(),
            secret.declassify_ref().as_slice(),
        )
        .then_some(())
        .ok_or(libcrux_traits::ecdh::arrayref::DeriveError::Unknown)
    }

    fn validate_secret(
        secret: &[U8; SECRET_LEN],
    ) -> Result<(), libcrux_traits::ecdh::arrayref::ValidateSecretError> {
        let mut all_zero = U8(0u8);
        for i in 0..SECRET_LEN {
            all_zero |= secret[i];
        }

        (all_zero.declassify() != 0)
            .then_some(())
            .ok_or(libcrux_traits::ecdh::arrayref::ValidateSecretError::InvalidSecret)?;

        crate::p256::validate_private_key(secret.declassify_ref().as_slice())
            .then_some(())
            .ok_or(libcrux_traits::ecdh::arrayref::ValidateSecretError::InvalidSecret)
    }
}

libcrux_traits::ecdh::slice::impl_ecdh_slice_trait!(super::P256 => RAND_LEN, SECRET_LEN, PUBLIC_LEN);

//! This module contains the trait and related errors for a KEM that takes array references as
//! arguments and returns values as arrays.

use libcrux_secrets::{Classify, U8};

use super::arrayref;

pub use arrayref::{DecapsError, EncapsError, KeyGenError};

/// A Key Encapsulation Mechanismd (KEM) that returns values instead of writing the results to
/// `&mut` arguments.
pub trait Kem<
    const EK_LEN: usize,
    const DK_LEN: usize,
    const CT_LEN: usize,
    const SS_LEN: usize,
    const RAND_KEYGEN_LEN: usize,
    const RAND_ENCAPS_LEN: usize,
>
{
    /// Generate a pair of encapsulation and decapsulation keys.
    /// It is the responsibility of the caller to ensure  that the `rand` argument is actually
    /// random.
    fn keygen(rand: &[U8; RAND_KEYGEN_LEN]) -> Result<([U8; DK_LEN], [u8; EK_LEN]), KeyGenError>;

    /// Encapsulate a shared secret towards a given encapsulation key.
    /// It is the responsibility of the caller to ensure  that the `rand` argument is actually
    /// random.
    fn encaps(
        ek: &[u8; EK_LEN],
        rand: &[U8; RAND_ENCAPS_LEN],
    ) -> Result<([U8; SS_LEN], [u8; CT_LEN]), EncapsError>;

    /// Decapsulate a shared secret.
    fn decaps(ct: &[u8; CT_LEN], dk: &[U8; DK_LEN]) -> Result<[U8; SS_LEN], DecapsError>;
}

impl<
        const EK_LEN: usize,
        const DK_LEN: usize,
        const CT_LEN: usize,
        const SS_LEN: usize,
        const RAND_KEYGEN_LEN: usize,
        const RAND_ENCAPS_LEN: usize,
        T: arrayref::Kem<EK_LEN, DK_LEN, CT_LEN, SS_LEN, RAND_KEYGEN_LEN, RAND_ENCAPS_LEN>,
    > Kem<EK_LEN, DK_LEN, CT_LEN, SS_LEN, RAND_KEYGEN_LEN, RAND_ENCAPS_LEN> for T
{
    fn keygen(rand: &[U8; RAND_KEYGEN_LEN]) -> Result<([U8; DK_LEN], [u8; EK_LEN]), KeyGenError> {
        let mut dk = [0u8.classify(); DK_LEN];
        let mut ek = [0u8; EK_LEN];

        <Self as arrayref::Kem<
            EK_LEN,
            DK_LEN,
            CT_LEN,
            SS_LEN,
            RAND_KEYGEN_LEN,
            RAND_ENCAPS_LEN,
        >>::keygen(&mut ek, &mut dk, rand)?;

        Ok((dk, ek))
    }

    fn encaps(
        ek: &[u8; EK_LEN],
        rand: &[U8; RAND_ENCAPS_LEN],
    ) -> Result<([U8; SS_LEN], [u8; CT_LEN]), EncapsError> {
        let mut ss = [0u8.classify(); SS_LEN];
        let mut ct = [0u8; CT_LEN];

        <Self as arrayref::Kem<
            EK_LEN,
            DK_LEN,
            CT_LEN,
            SS_LEN,
            RAND_KEYGEN_LEN,
            RAND_ENCAPS_LEN,
        >>::encaps(&mut ct, &mut ss, ek, rand)?;

        Ok((ss, ct))
    }

    fn decaps(ct: &[u8; CT_LEN], dk: &[U8; DK_LEN]) -> Result<[U8; SS_LEN], DecapsError> {
        let mut ss = [0u8.classify(); SS_LEN];

        <Self as arrayref::Kem<
            EK_LEN,
            DK_LEN,
            CT_LEN,
            SS_LEN,
            RAND_KEYGEN_LEN,
            RAND_ENCAPS_LEN,
        >>::decaps(&mut ss, ct, dk)?;

        Ok(ss)
    }
}

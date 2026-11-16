//! This module contains the trait and related errors for a KEM that takes slices as
//! arguments and writes the results to mutable slices..

use libcrux_secrets::U8;

use super::arrayref;

/// A Key Encapsulation Mechanismd (KEM). This trait takes slices as arguments.
pub trait Kem {
    /// Generate a pair of encapsulation and decapsulation keys.
    /// It is the responsibility of the caller to ensure  that the `rand` argument is actually
    /// random.
    fn keygen(ek: &mut [u8], dk: &mut [U8], rand: &[U8]) -> Result<(), KeyGenError>;

    /// Encapsulate a shared secret towards a given encapsulation key.
    /// It is the responsibility of the caller to ensure  that the `rand` argument is actually
    /// random.
    fn encaps(ct: &mut [u8], ss: &mut [U8], ek: &[u8], rand: &[U8]) -> Result<(), EncapsError>;

    /// Decapsulate a shared secret.
    fn decaps(ss: &mut [U8], ct: &[u8], dk: &[U8]) -> Result<(), DecapsError>;
}

/// Error generating key with provided randomness
#[derive(Debug)]
pub enum KeyGenError {
    /// Error generating key with provided randomness
    InvalidRandomness,
    /// The provided randomness has the wrong length
    InvalidRandomnessLength,
    /// The provided encapsulation key has the wrong length
    InvalidEncapsKeyLength,
    /// The provided decapulation key has the wrong length
    InvalidDecapsKeyLength,
    /// An unknown error occurred
    Unknown,
}

/// Error indicating that encapsulating failed
#[derive(Debug)]
pub enum EncapsError {
    /// Encapsulation key is invalid
    InvalidEncapsKey,
    /// Error encapsulating key with provided randomness
    InvalidRandomness,
    /// The provided randomness has the wrong length
    InvalidRandomnessLength,
    /// The provided encapsulation key has the wrong length
    InvalidEncapsKeyLength,
    /// The provided ciphertext has the wrong length
    InvalidCiphertextLength,
    /// The provided shared secret has the wrong length
    InvalidSharedSecretLength,
    /// An unknown error occurred
    Unknown,
}

/// Error indicating that decapsulating failed
#[derive(Debug)]
pub enum DecapsError {
    /// Ciphertext key is invalid
    InvalidCiphertext,
    /// Decapsulation key is invalid
    InvalidDecapsKey,
    /// The provided decapulation key has the wrong length
    InvalidDecapsKeyLength,
    /// The provided ciphertext has the wrong length
    InvalidCiphertextLength,
    /// The provided shared secret has the wrong length
    InvalidSharedSecretLength,
    /// An unknown error occurred
    Unknown,
}

impl From<arrayref::KeyGenError> for KeyGenError {
    fn from(value: arrayref::KeyGenError) -> Self {
        match value {
            arrayref::KeyGenError::InvalidRandomness => KeyGenError::InvalidRandomness,
            arrayref::KeyGenError::Unknown => KeyGenError::Unknown,
        }
    }
}

impl From<arrayref::EncapsError> for EncapsError {
    fn from(value: arrayref::EncapsError) -> Self {
        match value {
            arrayref::EncapsError::InvalidEncapsKey => EncapsError::InvalidEncapsKey,
            arrayref::EncapsError::InvalidRandomness => EncapsError::InvalidRandomness,
            arrayref::EncapsError::Unknown => EncapsError::Unknown,
        }
    }
}

impl From<arrayref::DecapsError> for DecapsError {
    fn from(value: arrayref::DecapsError) -> Self {
        match value {
            arrayref::DecapsError::InvalidCiphertext => DecapsError::InvalidCiphertext,
            arrayref::DecapsError::InvalidDecapsKey => DecapsError::InvalidDecapsKey,
            arrayref::DecapsError::Unknown => DecapsError::Unknown,
        }
    }
}

#[macro_export]
/// Implements [`Kem`] for any [`arrayref::Kem`]
macro_rules! impl_trait {
    ($type:ty => $ek:expr, $dk:expr, $ct:expr, $ss:expr, $rand_kg:expr, $rand_encaps:expr) => {
        impl $crate::kem::slice::Kem for $type {
            fn keygen(ek: &mut [u8], dk: &mut [$crate::libcrux_secrets::U8], rand: &[$crate::libcrux_secrets::U8]) -> Result<(), $crate::kem::slice::KeyGenError> {
                let ek : &mut [u8; $ek] = ek
                    .try_into()
                    .map_err(|_| $crate::kem::slice::KeyGenError::InvalidEncapsKeyLength)?;
                let dk : &mut [$crate::libcrux_secrets::U8; $dk] = dk
                    .try_into()
                    .map_err(|_| $crate::kem::slice::KeyGenError::InvalidDecapsKeyLength)?;
                let rand : &[$crate::libcrux_secrets::U8; $rand_kg] = rand
                    .try_into()
                    .map_err(|_| $crate::kem::slice::KeyGenError::InvalidRandomnessLength)?;

                <$type as $crate::kem::arrayref::Kem<$ek, $dk, $ct, $ss, $rand_kg, $rand_encaps>>::keygen(ek, dk, rand).map_err($crate::kem::slice::KeyGenError::from)
            }

            fn encaps(ct: &mut [u8], ss: &mut [$crate::libcrux_secrets::U8], ek: &[u8], rand: &[$crate::libcrux_secrets::U8]) -> Result<(), $crate::kem::slice::EncapsError>{
                let ct : &mut [u8; $ct] = ct
                    .try_into()
                    .map_err(|_| $crate::kem::slice::EncapsError::InvalidCiphertextLength)?;
                let ss : &mut [$crate::libcrux_secrets::U8; $ss] = ss
                    .try_into()
                    .map_err(|_| $crate::kem::slice::EncapsError::InvalidSharedSecretLength)?;
                let ek : & [u8; $ek] = ek
                    .try_into()
                    .map_err(|_| $crate::kem::slice::EncapsError::InvalidEncapsKeyLength)?;
                let rand : &[$crate::libcrux_secrets::U8; $rand_encaps] = rand
                    .try_into()
                    .map_err(|_| $crate::kem::slice::EncapsError::InvalidRandomnessLength)?;


                <$type as $crate::kem::arrayref::Kem<$ek, $dk, $ct, $ss, $rand_kg, $rand_encaps>>::encaps(ct, ss, ek,rand).map_err($crate::kem::slice::EncapsError::from)
            }

            fn decaps(ss: &mut [$crate::libcrux_secrets::U8], ct: &[u8], dk: &[$crate::libcrux_secrets::U8]) -> Result<(), $crate::kem::slice::DecapsError> {
                let ss : &mut [$crate::libcrux_secrets::U8; $ss] = ss
                    .try_into()
                    .map_err(|_| $crate::kem::slice::DecapsError::InvalidSharedSecretLength)?;
                let ct : &[u8; $ct] = ct
                    .try_into()
                    .map_err(|_| $crate::kem::slice::DecapsError::InvalidCiphertextLength)?;
                let dk : &[$crate::libcrux_secrets::U8; $dk] = dk
                    .try_into()
                    .map_err(|_| $crate::kem::slice::DecapsError::InvalidDecapsKeyLength)?;

                <$type as $crate::kem::arrayref::Kem<$ek, $dk, $ct, $ss, $rand_kg, $rand_encaps>>::decaps(ss, ct, dk).map_err($crate::kem::slice::DecapsError::from)
            }

        }

        #[cfg(test)]
        #[test]
        fn simple_kem_test(){
            $crate::kem::tests::simple::<$ek, $dk, $ct, $ss, $rand_kg, $rand_encaps, $type>();
        }
    };

}

pub use impl_trait;

impl core::fmt::Display for KeyGenError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            KeyGenError::InvalidRandomness => "error generating key with provided randomness",
            KeyGenError::Unknown => "an unknown error occurred",
            KeyGenError::InvalidRandomnessLength => "the provided randomness has the wrong length",
            KeyGenError::InvalidEncapsKeyLength => {
                "the provided encapsulation key has the wrong length"
            }
            KeyGenError::InvalidDecapsKeyLength => {
                "the provided decapsulation key has the wrong length"
            }
        };

        f.write_str(text)
    }
}

impl core::fmt::Display for EncapsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            EncapsError::InvalidEncapsKey => "encapsulation key is invalid",
            EncapsError::InvalidRandomness => "error generating key with provided randomness",
            EncapsError::Unknown => "an unknown error occurred",
            EncapsError::InvalidRandomnessLength => "the provided randomness has the wrong length",
            EncapsError::InvalidEncapsKeyLength => {
                "the provided encapsulation key has the wrong length"
            }
            EncapsError::InvalidCiphertextLength => "the provided ciphertext has the wrong length",
            EncapsError::InvalidSharedSecretLength => {
                "the provided shared secret has the wrong length"
            }
        };

        f.write_str(text)
    }
}

impl core::fmt::Display for DecapsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let text = match self {
            DecapsError::InvalidDecapsKey => "encapsulation key is invalid",
            DecapsError::InvalidCiphertext => "ciphertext is invalid",
            DecapsError::Unknown => "an unknown error occurred",
            DecapsError::InvalidCiphertextLength => "the provided ciphertext has the wrong length",
            DecapsError::InvalidSharedSecretLength => {
                "the provided shared secret has the wrong length"
            }
            DecapsError::InvalidDecapsKeyLength => {
                "the provided decapsulation key has the wrong length"
            }
        };

        f.write_str(text)
    }
}

#[cfg(feature = "error-in-core")]
/// Here we implement the Error trait. This has only been added to core relatively recently, so we
/// are hiding that behind a feature.
mod error_in_core {
    impl core::error::Error for super::KeyGenError {}
    impl core::error::Error for super::EncapsError {}
    impl core::error::Error for super::DecapsError {}
}

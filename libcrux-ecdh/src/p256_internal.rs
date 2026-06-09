use alloc::format;

use rand::{CryptoRng, Rng};

use super::Error;
// P256 we only have in HACL
use crate::hacl::p256;

pub struct PrivateKey(pub [u8; 32]);

#[cfg(feature = "codec")]
use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

#[cfg(feature = "codec")]
extern crate std;

#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(TlsSize, TlsSerialize, TlsDeserialize))]
pub struct PublicKey(pub [u8; 64]);

/// Output of a scalar multiplication between a public key and a secret key.
///
/// This value is NOT (!) safe for use as a key and needs to be processed in a round of key
/// derivation, to ensure both that the output is uniformly random and that unkown key share
/// attacks can not happen.
#[cfg_attr(feature = "codec", derive(TlsSize, TlsSerialize, TlsDeserialize))]
pub struct SharedSecret(pub [u8; 64]);

impl From<&[u8; 64]> for PublicKey {
    fn from(value: &[u8; 64]) -> Self {
        Self(*value)
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().map_err(|_| Error::InvalidPoint)?))
    }
}

impl From<&[u8; 32]> for PrivateKey {
    fn from(value: &[u8; 32]) -> Self {
        Self(*value)
    }
}

impl TryFrom<&[u8]> for PrivateKey {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().map_err(|_| Error::InvalidScalar)?))
    }
}

impl From<&[u8; 64]> for SharedSecret {
    fn from(value: &[u8; 64]) -> Self {
        Self(*value)
    }
}

impl TryFrom<&[u8]> for SharedSecret {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().map_err(|_| Error::InvalidScalar)?))
    }
}

impl AsRef<[u8]> for PrivateKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for SharedSecret {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for PrivateKey {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsRef<[u8; 64]> for PublicKey {
    fn as_ref(&self) -> &[u8; 64] {
        &self.0
    }
}

impl AsRef<[u8; 64]> for SharedSecret {
    fn as_ref(&self) -> &[u8; 64] {
        &self.0
    }
}

pub(super) fn derive(p: &PublicKey, s: &PrivateKey) -> Result<SharedSecret, Error> {
    // We assume that the private key has been validated.
    p256::ecdh(s, p)
        .map_err(|e| Error::Custom(format!("HACL Error {:?}", e)))
        .map(SharedSecret)
}

/// Compute the public key, corresponding to the private key `s`.
pub fn secret_to_public(s: &PrivateKey) -> Result<PublicKey, Error> {
    p256::validate_scalar(s).map_err(|e| Error::Custom(format!("HACL Error {:?}", e)))?;
    p256::secret_to_public(s)
        .map_err(|e| Error::Custom(format!("HACL Error {:?}", e)))
        .map(PublicKey)
}

pub fn validate_scalar(s: &PrivateKey) -> Result<(), Error> {
    p256::validate_scalar(s).map_err(|e| e.into())
}

#[allow(unused)]
pub fn validate_point(p: &PublicKey) -> Result<(), Error> {
    p256::validate_point(p).map_err(|e| e.into())
}

pub(crate) fn prepare_public_key(public_key: &[u8]) -> Result<PublicKey, Error> {
    if public_key.is_empty() {
        return Err(Error::InvalidPoint);
    }

    // Parse the public key.
    let pk = if let Ok(pk) = p256::uncompressed_to_coordinates(public_key) {
        pk
    } else {
        // Might be uncompressed
        if public_key.len() == 33 {
            p256::compressed_to_coordinates(public_key).map_err(|_| Error::InvalidPoint)?
        } else {
            // Might be a simple concatenation
            public_key.try_into().map_err(|_| Error::InvalidPoint)?
        }
    };
    let pk = PublicKey(pk);

    p256::validate_point(&pk)
        .map(|()| pk)
        .map_err(|_| Error::InvalidPoint)
}

/// Generate a new p256 secret (scalar)
pub fn generate_secret(rng: &mut (impl CryptoRng + Rng)) -> Result<PrivateKey, Error> {
    const LIMIT: usize = 100;
    for _ in 0..LIMIT {
        let mut out = [0u8; 32];
        rng.try_fill_bytes(&mut out)
            .map_err(|_| Error::KeyGenError)?;

        let out = PrivateKey(out);
        if validate_scalar(&out).is_ok() {
            return Ok(out);
        }
    }
    Err(Error::KeyGenError)
}

/// Generate a new P256 key pair
pub fn key_gen(rng: &mut (impl CryptoRng + Rng)) -> Result<(PrivateKey, PublicKey), Error> {
    let sk = generate_secret(rng)?;
    let pk = secret_to_public(&sk)?;
    Ok((sk, pk))
}

// XXX: This could be simplified with the pure Rust version now.

use alloc::format;

use rand::CryptoRng;

use super::Error;

pub struct PrivateKey(pub [u8; 32]);

#[cfg(feature = "codec")]
use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

#[cfg(feature = "codec")]
extern crate std;

#[derive(Debug)]
#[cfg_attr(feature = "codec", derive(TlsSize, TlsSerialize, TlsDeserialize))]
pub struct PublicKey(pub [u8; 32]);

/// Output of a scalar multiplication between a public key and a secret key.
///
/// This value is NOT (!) safe for use as a key and needs to be processed in a round of key
/// derivation, to ensure both that the output is uniformly random and that unkown key share
/// attacks can not happen.
#[cfg_attr(feature = "codec", derive(TlsSize, TlsSerialize, TlsDeserialize))]
pub struct SharedSecret(pub [u8; 32]);

impl From<&[u8; 32]> for PublicKey {
    fn from(value: &[u8; 32]) -> Self {
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
        let mut out = Self(*value);
        clamp(&mut out.0);
        out
    }
}

impl TryFrom<&[u8]> for PrivateKey {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut out = Self(value.try_into().map_err(|_| Error::InvalidScalar)?);
        clamp(&mut out.0);
        Ok(out)
    }
}

impl From<&[u8; 32]> for SharedSecret {
    fn from(value: &[u8; 32]) -> Self {
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

impl AsRef<[u8; 32]> for PublicKey {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for SharedSecret {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

pub fn derive(p: &PublicKey, s: &PrivateKey) -> Result<SharedSecret, Error> {
    // Use the portable HACL implementation.
    use crate::hacl::curve25519;

    curve25519::ecdh(s, p)
        .map_err(|e| Error::Custom(format!("HACL Error {:?}", e)))
        .map(SharedSecret)
}

/// Compute the public key, corresponding to the private key `s`.
pub fn secret_to_public(s: &PrivateKey) -> Result<PublicKey, Error> {
    // Use the portable HACL implementation.
    use crate::hacl::curve25519;

    Ok(PublicKey(curve25519::secret_to_public(s)))
}

/// Generate a new x25519 secret.
pub fn generate_secret(rng: &mut impl CryptoRng) -> Result<PrivateKey, Error> {
    const LIMIT: usize = 100;
    for _ in 0..LIMIT {
        let mut out = [0u8; 32];
        rng.try_fill_bytes(&mut out)
            .map_err(|_| Error::KeyGenError)?;

        // We don't want a 0 key.
        if out.iter().all(|&b| b == 0) {
            continue;
        }

        clamp(&mut out);
        return Ok(PrivateKey(out));
    }

    Err(Error::KeyGenError)
}

/// Clamp a scalar.
fn clamp(scalar: &mut [u8; 32]) {
    // We clamp the key already to make sure it can't be misused.
    scalar[0] &= 248u8;
    scalar[31] &= 127u8;
    scalar[31] |= 64u8;
}

/// Generate a new P256 key pair
pub fn key_gen(rng: &mut impl CryptoRng) -> Result<(PrivateKey, PublicKey), Error> {
    let sk = generate_secret(rng)?;
    let pk = secret_to_public(&sk)?;
    Ok((sk, pk))
}

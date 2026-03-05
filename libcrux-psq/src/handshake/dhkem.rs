//! Diffie-Hellman KEM type wrappers
//!
//! This module provides wrappers around KEM types, assuming a DH-KEM
//! style API.
use libcrux_ecdh::{secret_to_public, Algorithm};
use rand::CryptoRng;
use tls_codec::{TlsDeserialize, TlsDeserializeBytes, TlsSerialize, TlsSerializeBytes, TlsSize};

use crate::handshake::{
    types::{Authenticator, ProvideAuthenticator},
    HandshakeError as Error,
};

#[derive(TlsSerializeBytes, TlsSize)]
/// A wrapper around a KEM shared secret.
pub struct DHSharedSecret(Vec<u8>);

impl AsRef<[u8]> for DHSharedSecret {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

#[derive(
    Eq,
    Debug,
    Hash,
    PartialEq,
    Clone,
    Copy,
    TlsDeserializeBytes,
    TlsSerializeBytes,
    TlsSize,
    TlsSerialize,
    TlsDeserialize,
)]
/// A wrapper around a KEM public key.
pub struct DHPublicKey([u8; 32]);

impl ProvideAuthenticator for DHPublicKey {
    fn authenticator(&self) -> super::types::Authenticator {
        Authenticator::Dh(*self)
    }
}

impl AsRef<[u8]> for DHPublicKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

/// Import a Diffie-Hellman public key from raw bytes.
impl DHPublicKey {
    pub fn from_bytes(value: &[u8; 32]) -> Self {
        Self(*value)
    }
}

/// A wrapper around a KEM private key.
pub struct DHPrivateKey(Vec<u8>);

impl AsRef<[u8]> for DHPrivateKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl DHSharedSecret {
    /// Derive a shared secret, DH-KEM style.
    pub(crate) fn derive(sk: &DHPrivateKey, pk: &DHPublicKey) -> Result<DHSharedSecret, Error> {
        Ok(DHSharedSecret(
            libcrux_ecdh::derive(Algorithm::X25519, pk.0, &sk.0).map_err(|_| Error::CryptoError)?,
        ))
    }
}

impl DHPrivateKey {
    /// Creates a new KEM private key.
    pub fn new(rng: &mut impl CryptoRng) -> Self {
        Self(
            libcrux_ecdh::generate_secret(libcrux_ecdh::Algorithm::X25519, rng)
                .expect("Insufficient Randomness"),
        )
    }

    /// Compute the KEM public key from the KEM private key.
    pub fn to_public(&self) -> DHPublicKey {
        DHPublicKey(
            secret_to_public(libcrux_ecdh::Algorithm::X25519, &self.0)
                .expect("secret key is honestly generated X25519 key")
                .try_into()
                .expect("secret key is honestly generated X25519 key"),
        )
    }

    /// Import a Diffie-Hellman private key from raw bytes.
    pub fn from_bytes(value: &[u8; 32]) -> Result<Self, Error> {
        // Test whether the key is already clamped to make sure it can't be misused.
        if libcrux_ecdh::validate_scalar(libcrux_ecdh::Algorithm::X25519, value).is_err() {
            Err(Error::InvalidDHSecret)
        } else {
            Ok(Self(Vec::from(value)))
        }
    }

    /// Perform a Diffie-Hellman exchange between `self` and `remote_public_key`
    /// to produce a `DHSharedSecret`
    pub fn diffie_hellman(&self, remote_public_key: &DHPublicKey) -> Result<DHSharedSecret, Error> {
        DHSharedSecret::derive(self, remote_public_key)
    }
}

/// A wrapper for Diffie-Hellman key pairs.
pub struct DHKeyPair {
    /// The Diffie-Hellman private key
    pub(crate) sk: DHPrivateKey,
    /// The Diffie-Hellman public key
    pub pk: DHPublicKey,
}

impl ProvideAuthenticator for DHKeyPair {
    fn authenticator(&self) -> super::types::Authenticator {
        self.pk.authenticator()
    }
}

impl DHKeyPair {
    /// Generate a fresh Diffie-Hellman key pair.
    pub fn new(rng: &mut impl CryptoRng) -> Self {
        let sk = DHPrivateKey::new(rng);
        let pk = sk.to_public();
        Self { sk, pk }
    }

    /// Provide reference to the Diffie-Hellman private key.
    pub fn sk(&self) -> &DHPrivateKey {
        &self.sk
    }
}

impl From<DHPrivateKey> for DHKeyPair {
    /// Derive a Diffie-Hellman public key from a private key
    /// and build a wrapper around both.
    fn from(sk: DHPrivateKey) -> Self {
        Self {
            pk: sk.to_public(),
            sk,
        }
    }
}

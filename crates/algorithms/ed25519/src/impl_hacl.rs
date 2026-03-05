#[cfg(feature = "codec")]
use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

#[cfg(feature = "codec")]
extern crate std;

#[cfg(feature = "codec")]
use std::format;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    SigningError,
    InvalidSignature,
    KeyGen,
}

/// An Ed25519 public, verification key
#[derive(Default, Clone, Copy)]
#[cfg_attr(feature = "codec", derive(TlsSerialize, TlsDeserialize, TlsSize))]
pub struct VerificationKey {
    value: [u8; 32],
}

impl VerificationKey {
    /// Get the raw bytes
    pub fn into_bytes(self) -> [u8; 32] {
        self.value
    }

    /// Build a new verification key
    pub fn from_bytes(value: [u8; 32]) -> Self {
        Self { value }
    }
}

impl AsRef<[u8; 32]> for VerificationKey {
    fn as_ref(&self) -> &[u8; 32] {
        &self.value
    }
}

/// An Ed25519 private, signing  key
#[derive(Default)]
pub struct SigningKey {
    value: [u8; 32],
}

impl SigningKey {
    /// Get the raw bytes
    pub fn into_bytes(self) -> [u8; 32] {
        self.value
    }

    /// Build a new signing key
    pub fn from_bytes(value: [u8; 32]) -> Self {
        Self { value }
    }
}

impl AsRef<[u8; 32]> for SigningKey {
    fn as_ref(&self) -> &[u8; 32] {
        &self.value
    }
}

/// The hacl implementation requires that
/// - the private key is a 32 byte buffer
/// - the signature is a 64 byte buffer,
/// - the payload buffer is not shorter than payload_len.
///
/// We enforce the first two using types, and the latter by using `payload.len()` and `payload_len`.
/// This has the caveat that `payload_len` must be <= u32::MAX, so we return an error if that is
/// not the case.
#[inline(always)]
pub fn sign(payload: &[u8], private_key: &[u8; 32]) -> Result<[u8; 64], Error> {
    let mut signature = [0u8; 64];
    crate::hacl::ed25519::sign(
        &mut signature,
        private_key,
        payload.len().try_into().map_err(|_| Error::SigningError)?,
        payload,
    );

    Ok(signature)
}

/// The hacl implementation requires that
/// - the public key is a 32 byte buffer
/// - the signature is a 64 byte buffer,
/// - the payload buffer is not shorter than payload_len.
///
/// We enforce the first two using types, and the latter by using `payload.len()` and `payload_len`.
/// This has the caveat that `payload_len` must be <= u32::MAX, so we return an error if that is
/// not the case.
#[inline(always)]
pub fn verify(payload: &[u8], public_key: &[u8; 32], signature: &[u8; 64]) -> Result<(), Error> {
    if crate::hacl::ed25519::verify(
        public_key,
        payload.len().try_into().map_err(|_| Error::SigningError)?,
        payload,
        signature,
    ) {
        Ok(())
    } else {
        Err(Error::InvalidSignature)
    }
}

/// Compute the public point for the given secret key `sk`.
/// The hacl implementation requires that these are both 32 byte buffers, which we enforce through
/// types.
#[inline(always)]
pub fn secret_to_public(pk: &mut [u8; 32], sk: &[u8; 32]) {
    crate::hacl::ed25519::secret_to_public(pk, sk)
}

#[cfg(feature = "rand")]
pub fn generate_key_pair(
    rng: &mut impl rand_core::CryptoRng,
) -> Result<(SigningKey, VerificationKey), Error> {
    use rand_core::TryRngCore;

    const LIMIT: usize = 100;
    let mut sk = [0u8; 32];
    let mut pk = [0u8; 32];

    for _ in 0..LIMIT {
        rng.try_fill_bytes(&mut sk).map_err(|_| Error::KeyGen)?;

        // We don't want a 0 key.
        if sk.iter().all(|&b| b == 0) {
            sk = [0u8; 32];
            continue;
        }

        break;
    }

    secret_to_public(&mut pk, &sk);

    Ok((SigningKey { value: sk }, VerificationKey { value: pk }))
}

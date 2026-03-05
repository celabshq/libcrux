use std::ops::Deref;

use libcrux_ed25519::VerificationKey as Ed25519VerificationKey;
use libcrux_kem::{MlKem768Ciphertext, MlKem768PrivateKey, MlKem768PublicKey};
use libcrux_ml_dsa::ml_dsa_65::{MLDSA65Signature, MLDSA65VerificationKey};
use libcrux_ml_kem::MlKemSharedSecret;
use tls_codec::{TlsDeserialize, TlsSerialize, TlsSerializeBytes, TlsSize};

#[cfg(feature = "classic-mceliece")]
use crate::classic_mceliece::{Ciphertext, PublicKey, SecretKey, SharedSecret};
use crate::handshake::{
    ciphersuite::{initiator::Auth, PSQ_MLDSA_CONTEXT},
    dhkem::DHPublicKey,
    transcript::Transcript,
    HandshakeError,
};

#[derive(Clone, TlsSize, TlsDeserialize, TlsSerialize)]
#[repr(u8)]
/// A value that is used to authenticate the initiator.
///
/// We assume the responder can validate the authenticator out-of-band.
pub enum Authenticator {
    /// A Diffie-Hellman public key.
    Dh(DHPublicKey),
    /// A signature verification key.
    Sig(SignatureVerificationKey),
}

/// An object that can provide an initiator authenticator.
pub trait ProvideAuthenticator {
    /// Provide the initiator authenticator contained in this object.
    fn authenticator(&self) -> Authenticator;
}

impl ProvideAuthenticator for Ed25519VerificationKey {
    fn authenticator(&self) -> Authenticator {
        Authenticator::Sig(SignatureVerificationKey::Ed25519(*self))
    }
}

impl ProvideAuthenticator for MLDSA65VerificationKey {
    fn authenticator(&self) -> Authenticator {
        Authenticator::Sig(SignatureVerificationKey::MlDsa65(Box::new(self.clone())))
    }
}

impl From<Auth<'_>> for Authenticator {
    fn from(value: Auth<'_>) -> Self {
        match value {
            Auth::DH(dhkey_pair) => Authenticator::Dh(dhkey_pair.pk),
            Auth::Sig(sig_auth) => Authenticator::Sig(sig_auth.into()),
        }
    }
}

impl ProvideAuthenticator for SignatureVerificationKey {
    fn authenticator(&self) -> Authenticator {
        match &self {
            SignatureVerificationKey::Ed25519(verification_key) => verification_key.authenticator(),
            SignatureVerificationKey::MlDsa65(mldsaverification_key) => {
                mldsaverification_key.deref().authenticator()
            }
        }
    }
}

#[derive(TlsSize, TlsDeserialize, TlsSerialize, Clone)]
#[repr(u8)]
/// A signature verification key.
pub enum SignatureVerificationKey {
    /// A verification key for the Ed25519 signature scheme.
    Ed25519(Ed25519VerificationKey),
    /// A verification key for the ML-DSA 65 signature scheme.
    MlDsa65(Box<MLDSA65VerificationKey>),
}

pub enum SignatureType {
    Ed25519,
    MlDsa,
}

impl SignatureVerificationKey {
    pub(crate) fn verify(
        &self,
        tx1: &Transcript,
        signature: &Signature,
    ) -> Result<(), HandshakeError> {
        use tls_codec::SerializeBytes;
        let payload = tx1.tls_serialize().map_err(HandshakeError::Serialize)?;
        match (self, signature) {
            (SignatureVerificationKey::Ed25519(verification_key), Signature::Ed25519(sig)) => {
                libcrux_ed25519::verify(&payload, verification_key.as_ref(), sig)
                    .map_err(|e| e.into())
            }

            (
                SignatureVerificationKey::MlDsa65(mldsaverification_key),
                Signature::MlDsa65(mldsasignature),
            ) => libcrux_ml_dsa::ml_dsa_65::verify(
                mldsaverification_key,
                &payload,
                PSQ_MLDSA_CONTEXT,
                mldsasignature,
            )
            .map_err(|e| e.into()),
            (SignatureVerificationKey::Ed25519(_), Signature::MlDsa65(_))
            | (SignatureVerificationKey::MlDsa65(_), Signature::Ed25519(_)) => {
                Err(HandshakeError::UnsupportedCiphersuite)
            }
        }
    }
}

#[derive(TlsSize, TlsDeserialize, TlsSerialize)]
#[repr(u8)]
/// A digital signature.
pub enum Signature {
    /// An Ed25519 signature.
    Ed25519([u8; 64]),
    /// An ML-DSA 65 signature.
    MlDsa65(Box<MLDSA65Signature>),
}

#[derive(TlsSize, TlsDeserialize, TlsSerialize)]
#[repr(u8)]
/// A ciphertext encapsulating a post-quantum shared secret.
pub enum PQCiphertext {
    /// An ML-KEM 768 ciphertext.
    MlKem(Box<MlKem768Ciphertext>) = 1,
    #[cfg(feature = "classic-mceliece")]
    /// A Classic McEliece 460896f ciphertext.
    CMC(Box<Ciphertext>) = 2,
    #[cfg(not(feature = "classic-mceliece"))]
    /// Dummy variant (use feature `classic-mceliece` to enable Classic McEliece)
    CMC = 3,
}

#[derive(TlsSize, TlsSerialize)]
#[repr(u8)]
/// An encapsulation key for a post-quantum key encapsulation mechanism.
pub enum PQEncapsulationKey<'a> {
    /// An ML-KEM 768 encapsulation key.
    MlKem(&'a MlKem768PublicKey) = 1,
    #[cfg(feature = "classic-mceliece")]
    /// A Classic McEliece 460896f encapsulation key.
    CMC(&'a PublicKey) = 2,
    #[cfg(not(feature = "classic-mceliece"))]
    /// Dummy variant (use feature `classic-mceliece` to enable Classic McEliece)
    CMC = 3,
}

impl<'a> From<&'a MlKem768PublicKey> for PQEncapsulationKey<'a> {
    fn from(value: &'a MlKem768PublicKey) -> Self {
        Self::MlKem(value)
    }
}

#[cfg(feature = "classic-mceliece")]
impl<'a> From<&'a PublicKey> for PQEncapsulationKey<'a> {
    fn from(value: &'a PublicKey) -> Self {
        Self::CMC(value)
    }
}

#[derive(TlsSize, TlsSerializeBytes)]
#[repr(u8)]
/// A shared secret generated by a post-quantum key encapsulation mechanism.
pub enum PQSharedSecret {
    /// Indicates the absense of a PQ shared secret.
    None,
    /// An ML-KEM 768 shared secret.
    MlKem(MlKemSharedSecret),
    #[cfg(feature = "classic-mceliece")]
    /// A Classic McEliece 460896f shared secret.
    CMC(SharedSecret<'static>),
    #[cfg(not(feature = "classic-mceliece"))]
    /// Dummy variant (use feature `classic-mceliece` to enable Classic McEliece)
    CMC,
}

/// A decapsulation key for a post-quantum key encapsulation mechanism.
pub enum PQDecapsulationKey {
    /// Indicates the absense of a PQ decapsulation key.
    None,
    /// An ML-KEM 768 decapsulation key.
    MlKem(Box<MlKem768PrivateKey>),
    #[cfg(feature = "classic-mceliece")]
    /// A Classic McEliece 460896f decapsulation key.
    CMC(Box<SecretKey>),
    #[cfg(not(feature = "classic-mceliece"))]
    /// Dummy variant (use feature `classic-mceliece` to enable Classic McEliece)
    CMC,
}

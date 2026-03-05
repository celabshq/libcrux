//! # The PSQ Handshake
//!
//! The PSQ handshake consists of one message from the initiator to the
//! responder, and one response from the responder. It supports two modes:
//! - Query mode: A stateless, classically secure protocol which allows
//!   the initiator to send one query payload to the responder, which in
//!   turn sends one query response payload to the initiator.
//! - Registration mode: An (optionally) HNDL-secure protocol, which allows the
//!   initiator to register its long-term classical public key with the
//!   responder in a long-term session. The initiator's long-term public
//!   key is transmitted encrypted under the responders long-term public
//!   key, and thus not revealed to an eavesdropping attacker.

#![allow(missing_docs)]

#[derive(Debug, PartialEq)]
pub enum HandshakeError {
    Serialize(tls_codec::Error),
    Deserialize(tls_codec::Error),
    CryptoError,
    InitiatorState,
    ResponderState,
    TransportState,
    OutputBufferShort,
    PayloadTooLong,
    ChannelError,
    UnsupportedCiphersuite,
    Storage,
    OtherError,
    IdentifierMismatch,
    InvalidMessage,
    InvalidDHSecret,
}

impl From<libcrux_ed25519::Error> for HandshakeError {
    fn from(_value: libcrux_ed25519::Error) -> Self {
        Self::CryptoError
    }
}

impl From<libcrux_ml_dsa::SigningError> for HandshakeError {
    fn from(_value: libcrux_ml_dsa::SigningError) -> Self {
        Self::CryptoError
    }
}

impl From<libcrux_ml_dsa::VerificationError> for HandshakeError {
    fn from(_value: libcrux_ml_dsa::VerificationError) -> Self {
        Self::CryptoError
    }
}

impl From<AEADError> for HandshakeError {
    fn from(value: AEADError) -> Self {
        match value {
            AEADError::CryptoError => HandshakeError::CryptoError,
            AEADError::Serialize(error) => HandshakeError::Serialize(error),
            AEADError::Deserialize(error) => HandshakeError::Deserialize(error),
            AEADError::KeyExpired => unreachable!("Attempt to re-use an expired key during handshake. This indicates a fatal bug in the handshake protocol, please submit a bug report at https://github.com/cryspen/libcrux/."), // this really should not happen and indicates a fatal bug in the handshake protocol
        }
    }
}

use std::ops::Deref;

use tls_codec::{TlsDeserialize, TlsSerialize, TlsSerializeBytes, TlsSize, VLByteSlice, VLBytes};
use transcript::Transcript;

use crate::{
    aead::{AEADError, AEADKeyNonce, AeadType},
    handshake::{
        ciphersuite::{types::PQSharedSecret, CiphersuiteName},
        dhkem::{DHPrivateKey, DHPublicKey, DHSharedSecret},
        types::{Authenticator, ProvideAuthenticator, Signature, SignatureVerificationKey},
    },
};

pub(crate) mod dhkem;
pub(crate) mod initiator;
// pub mod pqkem;
pub(crate) mod responder;
pub(crate) mod transcript;

pub(crate) mod builder;
pub(crate) mod ciphersuite;

pub(crate) struct ToTransportState {
    pub(crate) tx2: Transcript,
    pub(crate) k2: AEADKeyNonce,
    pub(crate) initiator_authenticator: Option<Authenticator>,
    pub(crate) pq: bool,
}

#[derive(TlsDeserialize, TlsSize)]
/// A PSQ handshake message.
pub(crate) struct HandshakeMessage {
    /// A Diffie-Hellman KEM public key
    pk: DHPublicKey,
    /// The AEAD-encrypted message payload
    ciphertext: VLBytes,
    /// AEAD tag authenticating the ciphertext and any AAD
    tag: [u8; 16],
    /// Associated data, covered by the AEAD message authentication tag
    aad: VLBytes,
    /// The handshake ciphersuite for this message
    ciphersuite: CiphersuiteName,
}

#[derive(TlsDeserialize, TlsSize)]
#[repr(u8)]
pub(crate) enum AuthMessage {
    Dh(DHPublicKey),
    Sig {
        vk: Box<SignatureVerificationKey>,
        signature: Box<Signature>,
    },
}

impl From<&AuthMessage> for Authenticator {
    fn from(value: &AuthMessage) -> Self {
        match value {
            AuthMessage::Dh(dhpublic_key) => dhpublic_key.authenticator(),
            AuthMessage::Sig { vk, signature: _ } => vk.deref().authenticator(),
        }
    }
}

#[derive(TlsDeserialize, TlsSize)]
/// A PSQ inner message
pub(crate) struct InnerMessage {
    /// The AEAD-encrypted message payload
    ciphertext: VLBytes,
    /// AEAD tag authenticating the ciphertext and any AAD
    tag: [u8; 16],
    /// Associated data, covered by the AEAD message authentication tag
    aad: VLBytes,
    /// An optional post-quantum key encapsulation
    pq_encapsulation: VLBytes,
    /// The initiator authenticator
    auth: AuthMessage,
}

#[derive(TlsSerialize, TlsSize)]
#[repr(u8)]
pub(crate) enum AuthMessageOut<'a> {
    Dh(&'a DHPublicKey),
    Sig {
        vk: &'a SignatureVerificationKey,
        signature: &'a Signature,
    },
}

#[derive(TlsSerialize, TlsSize)]
/// A PSQ inner message (serialization helper)
pub(crate) struct InnerMessageOut<'a> {
    ciphertext: VLByteSlice<'a>,
    tag: [u8; 16], // XXX: implement Serialize for &[T; N]
    aad: VLByteSlice<'a>,
    pq_encapsulation: VLByteSlice<'a>,
    auth: AuthMessageOut<'a>,
}

#[derive(TlsSerialize, TlsSize)]
/// A PSQ handshake message. (Serialization helper)
pub(crate) struct HandshakeMessageOut<'a> {
    pk: &'a DHPublicKey,
    ciphertext: VLByteSlice<'a>,
    tag: [u8; 16], // XXX: implement Serialize for &[T; N]
    aad: VLByteSlice<'a>,
    ciphersuite: CiphersuiteName,
}

pub(crate) fn write_output(payload: &[u8], out: &mut [u8]) -> Result<usize, HandshakeError> {
    let payload_len = payload.len();
    if out.len() < payload_len {
        return Err(HandshakeError::OutputBufferShort);
    }
    out[..payload_len].copy_from_slice(payload);
    Ok(payload_len)
}

// K0 = KDF(g^xs, tx0)
pub(super) fn derive_k0(
    peer_pk: &DHPublicKey,
    own_pk: &DHPublicKey,
    own_sk: &DHPrivateKey,
    ctx: &[u8],
    is_responder: bool,
    aead_type: AeadType,
) -> Result<(Transcript, AEADKeyNonce), HandshakeError> {
    #[derive(TlsSerializeBytes, TlsSize)]
    struct K0Ikm<'a> {
        g_xs: &'a DHSharedSecret,
    }

    let tx0 = if is_responder {
        transcript::tx0(ctx, own_pk, peer_pk)?
    } else {
        transcript::tx0(ctx, peer_pk, own_pk)?
    };
    let ikm = K0Ikm {
        g_xs: &DHSharedSecret::derive(own_sk, peer_pk)?,
    };

    Ok((tx0, AEADKeyNonce::new(&ikm, &tx0, aead_type)?))
}

// K1 = KDF(K0 | g^cs | [SS], tx1)
pub(super) fn derive_k1_dh(
    k0: &AEADKeyNonce,
    own_longterm_key: &DHPrivateKey,
    peer_longterm_pk: &DHPublicKey,
    pq_shared_secret: Option<PQSharedSecret>,
    tx1: &Transcript,
    aead_type: AeadType,
) -> Result<AEADKeyNonce, HandshakeError> {
    #[derive(TlsSerializeBytes, TlsSize)]
    struct K1IkmDh<'a> {
        k0: &'a AEADKeyNonce,
        ecdh_shared_secret: &'a DHSharedSecret,
        pq_shared_secret: Option<PQSharedSecret>,
    }

    let ecdh_shared_secret = DHSharedSecret::derive(own_longterm_key, peer_longterm_pk)?;
    let ikm = K1IkmDh {
        k0,
        ecdh_shared_secret: &ecdh_shared_secret,
        pq_shared_secret,
    };

    Ok(AEADKeyNonce::new(&ikm, &tx1, aead_type)?)
}

// K1 = KDF(K0 | [SS], tx1 | sig)
pub(super) fn derive_k1_sig(
    k0: &AEADKeyNonce,
    pq_shared_secret: Option<PQSharedSecret>,
    tx1: &Transcript,
    signature: &Signature,
    aead_type: AeadType,
) -> Result<AEADKeyNonce, HandshakeError> {
    #[derive(TlsSerializeBytes, TlsSize)]
    struct K1IkmSig<'a> {
        k0: &'a AEADKeyNonce,
        pq_shared_secret: Option<PQSharedSecret>,
    }
    #[derive(TlsSerializeBytes, TlsSize)]
    struct K1InfoSig<'a> {
        tx1: &'a Transcript,
        signature_vec: Vec<u8>,
    }

    // XXX: This is not great.
    let signature_vec = match signature {
        Signature::Ed25519(sig) => sig.to_vec(),
        Signature::MlDsa65(mldsasignature) => mldsasignature.deref().as_ref().to_vec(),
    };

    Ok(AEADKeyNonce::new(
        &K1IkmSig {
            k0,
            pq_shared_secret,
        },
        &K1InfoSig { tx1, signature_vec },
        aead_type,
    )?)
}

#[derive(TlsSerializeBytes, TlsSize)]
struct K2IkmQuery<'a> {
    k0: &'a AEADKeyNonce,
    g_xs: &'a DHSharedSecret,
    g_xy: &'a DHSharedSecret,
}

#[derive(TlsSerializeBytes, TlsSize)]
struct K2IkmRegistrationDh<'a, 'b> {
    k1: &'a AEADKeyNonce,
    g_cy: &'b DHSharedSecret,
    g_xy: &'b DHSharedSecret,
}

#[derive(TlsSerializeBytes, TlsSize)]
struct K2IkmRegistrationSig<'a, 'b> {
    k1: &'a AEADKeyNonce,
    g_xy: &'b DHSharedSecret,
}

pub mod builders {
    #[derive(Debug, PartialEq)]
    pub enum BuilderError {
        CiphersuiteBuilderState,
        PrincipalBuilderState,
        UnsupportedCiphersuite,
    }

    #[doc(inline)]
    pub use crate::handshake::builder::PrincipalBuilder;
    #[doc(inline)]
    pub use crate::handshake::ciphersuite::builder::CiphersuiteBuilder;

    #[doc(inline)]
    pub use crate::handshake::ciphersuite::initiator::InitiatorCiphersuite;

    #[doc(inline)]
    pub use crate::handshake::ciphersuite::responder::ResponderCiphersuite;
}

pub mod types {
    #[doc(inline)]
    pub use crate::handshake::ciphersuite::types::*;
    #[doc(inline)]
    pub use crate::handshake::dhkem::{DHKeyPair, DHPrivateKey, DHPublicKey};

    #[doc(inline)]
    pub use crate::handshake::ciphersuite::initiator::SigningKeyPair;
}

pub mod ciphersuites {
    #[doc(inline)]
    pub use crate::handshake::ciphersuite::CiphersuiteName;
}

#[doc(inline)]
pub use initiator::query::QueryInitiator;

#[doc(inline)]
pub use initiator::registration::RegistrationInitiator;

#[doc(inline)]
pub use responder::Responder;

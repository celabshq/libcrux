use std::io::Cursor;

use rand::CryptoRng;

use tls_codec::{
    Deserialize, Serialize, SerializeBytes, Size, TlsDeserialize, TlsSerialize, TlsSize,
    VLByteSlice, VLBytes,
};

use crate::protocol::write_output;

use super::{
    dhkem::{DHKeyPair, DHPublicKey},
    initiator::{QueryInitiator, RegistrationInitiator},
    keys::{derive_i2r_channel_key, derive_r2i_channel_key, derive_session_key, AEADKey},
    pqkem::{PQKeyPair, PQPublicKey},
    responder::Responder,
    session::{SessionKey, SESSION_ID_LENGTH},
    transcript::Transcript,
};

pub(crate) fn serialize_error(e: tls_codec::Error) -> Error {
    Error::Serialize(e)
}

#[derive(Debug)]
pub enum Error {
    BuilderState,
    Serialize(tls_codec::Error),
    Deserialize(tls_codec::Error),
    CryptoError,
    InitiatorState,
    ResponderState,
    TransportState,
    OutputBufferShort,
    PayloadTooLong,
    ChannelError,
    Storage,
    OtherError,
    IdentifierMismatch,
}

#[derive(Debug)]
pub(crate) struct ToTransportState {
    pub(crate) tx2: Transcript,
    pub(crate) k2: AEADKey,
    pub(crate) initiator_ecdh_pk: Option<DHPublicKey>,
}

pub(crate) const PK_BINDER_LEN: usize = 8;

#[derive(TlsSerialize, TlsDeserialize, TlsSize)]
#[repr(u8)]
pub(crate) enum SessionPrincipal {
    Initiator,
    Responder,
}

#[derive(TlsSerialize, TlsDeserialize, TlsSize)]
/// A long-term session derived from the final handshake state.
///
/// Allows the creation of up to `u64::MAX` distinct, bi-directional
/// secure `Channel`s between initiator and responder, via
/// `Session::channel`.
///
/// The `Session` can be stored using `Session::serialize` and loaded
/// using `Session::deserialize`, which expects references to the same
/// public keys that were used in session creation to succeed.
///
/// **Warning**: Session state must only be stored and loaded using
/// `Session::serialize` and `Session::deserialize`.  While `Session`
/// implements `tls_codec::{Serialize, Deserialize}`, the associated
/// methods should not be called directly, since they do not consume the
/// `Session`. This opens up the possibility of continual use of a session
/// that has also been serialized. If the serialized session is then
/// deserialized, the deserialized version is stale and using it to
/// re-derive `Channel`s will result in nonce re-use with the potential
/// for loss of confidentiality.
pub struct Session {
    pub(crate) principal: SessionPrincipal,
    pub(crate) session_key: SessionKey,
    pub(crate) pk_binder: [u8; PK_BINDER_LEN],
    pub(crate) channel_counter: u64,
}

// pkBinder = KDF(skCS, g^c | g^s | [pkS])
fn derive_pk_binder(
    key: &SessionKey,
    initiator_ecdh_pk: &DHPublicKey,
    responder_ecdh_pk: &DHPublicKey,
    responder_pq_pk: Option<&PQPublicKey>,
) -> Result<[u8; PK_BINDER_LEN], Error> {
    #[derive(TlsSerialize, TlsSize)]
    struct PkBinderInfo<'a> {
        initiator_ecdh_pk: &'a DHPublicKey,
        responder_ecdh_pk: &'a DHPublicKey,
        responder_pq_pk: Option<&'a PQPublicKey>,
    }

    let info = PkBinderInfo {
        initiator_ecdh_pk,
        responder_ecdh_pk,
        responder_pq_pk,
    };
    let mut info_buf = vec![0u8; info.tls_serialized_len()];
    info.tls_serialize(&mut &mut info_buf[..])
        .map_err(serialize_error)?;

    let mut pk_binder = [0; PK_BINDER_LEN];
    libcrux_hkdf::sha2_256::hkdf(
        &mut pk_binder,
        &[],
        &key.key.tls_serialize_bytes().map_err(serialize_error)?,
        &info_buf,
    )
    .map_err(|_| Error::CryptoError)?;

    Ok(
        pk_binder.try_into().map_err(|_| Error::CryptoError)?, // We don't expect this to fail, unless HDKF gave us the wrong output length
    )
}

impl Session {
    pub(crate) fn new(
        tx2: Transcript,
        k2: AEADKey,
        initiator_ecdh_pk: &DHPublicKey,
        responder_ecdh_pk: &DHPublicKey,
        responder_pq_pk: Option<&PQPublicKey>,
        is_initiator: bool,
    ) -> Result<Self, Error> {
        let session_key = derive_session_key(k2, tx2)?;
        let pk_binder = derive_pk_binder(
            &session_key,
            initiator_ecdh_pk,
            responder_ecdh_pk,
            responder_pq_pk,
        )?;
        Ok(Self {
            principal: if is_initiator {
                SessionPrincipal::Initiator
            } else {
                SessionPrincipal::Responder
            },
            session_key,
            pk_binder,
            channel_counter: 0,
        })
    }

    // WARN: tls_serialize should not be called directly, since it does
    // not consume `Session`. This opens the possibility for nonce
    // re-use by deserializing a stale `Session` since the original
    // could be used after serialization.
    pub fn serialize(self, out: &mut [u8]) -> Result<usize, Error> {
        self.tls_serialize(&mut &mut out[..])
            .map_err(serialize_error)
    }

    // XXX: Use `tls_codec::conditional_deserializable` to implement
    // the validation.
    pub fn deserialize(
        bytes: &[u8],
        initiator_ecdh_pk: &DHPublicKey,
        responder_ecdh_pk: &DHPublicKey,
        responder_pq_pk: Option<&PQPublicKey>,
    ) -> Result<Self, Error> {
        let session =
            Session::tls_deserialize(&mut Cursor::new(bytes)).map_err(|e| Error::Deserialize(e))?;

        if derive_pk_binder(
            &session.session_key,
            initiator_ecdh_pk,
            responder_ecdh_pk,
            responder_pq_pk,
        )? == session.pk_binder
        {
            Ok(session)
        } else {
            Err(Error::Storage)
        }
    }

    pub fn channel(&mut self) -> Result<Channel, Error> {
        let channel = Channel::new(&self, matches!(self.principal, SessionPrincipal::Initiator))?;
        self.channel_counter = self
            .channel_counter
            .checked_add(1)
            .ok_or(Error::ChannelError)?;
        Ok(channel)
    }

    pub fn identifier(&self) -> &[u8; SESSION_ID_LENGTH] {
        &self.session_key.identifier
    }
}

#[derive(TlsSerialize, TlsSize)]
struct TransportMessageOut<'a> {
    nonce: u64,
    ciphertext: VLByteSlice<'a>,
    tag: [u8; 16],
}

#[derive(TlsDeserialize, TlsSize)]
struct TransportMessage {
    nonce: u64,
    ciphertext: VLBytes,
    tag: [u8; 16],
}

/// A secure channel derived from a long-term session.
///
/// Each channel derived from a session is identified by a monotonically
/// increasing `u64`. The channel identifier is included in every
/// `TransportMessage` sent on the channel.
///
/// Receiving a `TransportMessage` without matching channel identifier
/// results in an error.
pub struct Channel {
    send_key: AEADKey,
    recv_key: AEADKey,
    identifier: u64,
}

impl Channel {
    fn new(session: &Session, is_initiator: bool) -> Result<Self, Error> {
        if is_initiator {
            Ok(Self {
                send_key: derive_i2r_channel_key(session)?,
                recv_key: derive_r2i_channel_key(session)?,
                identifier: session.channel_counter,
            })
        } else {
            Ok(Self {
                send_key: derive_r2i_channel_key(session)?,
                recv_key: derive_i2r_channel_key(session)?,
                identifier: session.channel_counter,
            })
        }
    }

    pub fn identifier(&self) -> u64 {
        self.identifier
    }
}

impl Protocol for Channel {
    fn write_message(&mut self, payload: &[u8], out: &mut [u8]) -> Result<usize, Error> {
        // We match the maximum payload length of Noise.
        if payload.len() > 65535 {
            return Err(Error::PayloadTooLong);
        }
        let mut ciphertext = vec![0u8; payload.len()];
        let tag = self.send_key.encrypt(payload, &[], &mut ciphertext)?;
        let message = TransportMessageOut {
            nonce: self.identifier,
            ciphertext: VLByteSlice(ciphertext.as_ref()),
            tag,
        };

        message
            .tls_serialize(&mut &mut out[..])
            .map_err(|e| Error::Serialize(e))
    }

    fn read_message(&mut self, message: &[u8], out: &mut [u8]) -> Result<(usize, usize), Error> {
        let message = TransportMessage::tls_deserialize(&mut Cursor::new(message))
            .map_err(|e| Error::Deserialize(e))?;

        if self.identifier != message.nonce {
            return Err(Error::IdentifierMismatch);
        }

        let bytes_deserialized = message.tls_serialized_len();

        let payload = self
            .recv_key
            .decrypt(message.ciphertext.as_slice(), &message.tag, &[])?;

        let out_bytes_written = write_output(&payload, out)?;

        Ok((bytes_deserialized, out_bytes_written))
    }
}

pub trait IntoSession {
    fn into_session(self) -> Result<Session, Error>;
    fn is_handshake_finished(&self) -> bool;
}

pub trait Protocol {
    /// Write a handshake message to `out` to drive the handshake forward.
    ///
    /// The message may include a `payload`. Returns the number of
    /// bytes written to `out`.  If the internal state is not ready to
    /// write a message, nothing is written to `out` and `Ok(0)` is
    /// returned.
    fn write_message(&mut self, payload: &[u8], out: &mut [u8]) -> Result<usize, Error>;

    /// Reads the bytes in `message` as input to the handshake, and
    /// writes any payload bytes to `payload`.
    ///
    /// Returns a pair of `(bytes_deserialized, bytes_written)`, where
    /// `bytes_deserialized` is the number of bytes read from
    /// `message` and `bytes_written` is the number of bytes written
    /// to `payload`. If the internal state is not ready to read a
    /// message, nothing is written to `payload` and `Ok((0,0))` is
    /// returned.
    fn read_message(&mut self, message: &[u8], payload: &mut [u8])
        -> Result<(usize, usize), Error>;
}

pub struct Builder<'a, Rng: CryptoRng> {
    rng: Rng,
    context: &'a [u8],
    inner_aad: &'a [u8],
    outer_aad: &'a [u8],
    longterm_ecdh_keys: Option<&'a DHKeyPair>,
    longterm_pq_keys: Option<&'a PQKeyPair>,
    peer_longterm_ecdh_pk: Option<&'a DHPublicKey>,
    peer_longterm_pq_pk: Option<&'a PQPublicKey>,
    recent_keys_upper_bound: Option<usize>,
}

impl<'a, Rng: CryptoRng> Builder<'a, Rng> {
    /// Create a new builder.
    pub fn new(rng: Rng) -> Self {
        Self {
            rng,
            context: &[],
            inner_aad: &[],
            outer_aad: &[],
            longterm_ecdh_keys: None,
            longterm_pq_keys: None,
            peer_longterm_ecdh_pk: None,
            peer_longterm_pq_pk: None,
            recent_keys_upper_bound: None,
        }
    }

    // properties

    /// Set the context.
    pub fn context(mut self, context: &'a [u8]) -> Self {
        self.context = context;
        self
    }

    /// Set the inner AAD.
    pub fn inner_aad(mut self, inner_aad: &'a [u8]) -> Self {
        self.inner_aad = inner_aad;
        self
    }

    /// Set the outer AAD.
    pub fn outer_aad(mut self, outer_aad: &'a [u8]) -> Self {
        self.outer_aad = outer_aad;
        self
    }

    /// Set the long-term ECDH key pair.
    pub fn longterm_ecdh_keys(mut self, longterm_ecdh_keys: &'a DHKeyPair) -> Self {
        self.longterm_ecdh_keys = Some(longterm_ecdh_keys);
        self
    }

    /// Set the long-term PQ key pair.
    pub fn longterm_pq_keys(mut self, longterm_pq_keys: &'a PQKeyPair) -> Self {
        self.longterm_pq_keys = Some(longterm_pq_keys);
        self
    }

    /// Set the peer's long-term ECDH public key.
    pub fn peer_longterm_ecdh_pk(mut self, peer_longterm_ecdh_pk: &'a DHPublicKey) -> Self {
        self.peer_longterm_ecdh_pk = Some(peer_longterm_ecdh_pk);
        self
    }

    /// Set the peer's long-term PQ public key.
    pub fn peer_longterm_pq_pk(mut self, peer_longterm_pq_pk: &'a PQPublicKey) -> Self {
        self.peer_longterm_pq_pk = Some(peer_longterm_pq_pk);
        self
    }

    /// Set the maximum number of recent keys stored for DoS protection.
    pub fn recent_keys_upper_bound(mut self, recent_keys_upper_bound: usize) -> Self {
        self.recent_keys_upper_bound = Some(recent_keys_upper_bound);
        self
    }

    // builders

    /// Build a new [`QueryInitiator`].
    ///
    /// This requires that a `responder_ecdh_pk` is set.
    /// It also uses the `context` and `outer_aad`.
    pub fn build_query_initiator(self) -> Result<QueryInitiator<'a>, Error> {
        let Some(responder_ecdh_pk) = self.peer_longterm_ecdh_pk else {
            return Err(Error::BuilderState);
        };

        QueryInitiator::new(responder_ecdh_pk, self.context, self.outer_aad, self.rng)
    }

    /// Build a new [`RegistrationInitiator`].
    ///
    /// This requires that a `longterm_ecdh_keys` and a `peer_longterm_ecdh_pk`
    /// is set.
    /// It also uses the `context`, `inner_aad`, `outer_aad`, and
    /// `peer_longterm_pq_pk`.
    pub fn build_registration_initiator(self) -> Result<RegistrationInitiator<'a, Rng>, Error> {
        let Some(longterm_ecdh_keys) = self.longterm_ecdh_keys else {
            return Err(Error::BuilderState);
        };

        let Some(peer_longterm_ecdh_pk) = self.peer_longterm_ecdh_pk else {
            return Err(Error::BuilderState);
        };

        RegistrationInitiator::new(
            longterm_ecdh_keys,
            peer_longterm_ecdh_pk,
            self.peer_longterm_pq_pk,
            self.context,
            self.inner_aad,
            self.outer_aad,
            self.rng,
        )
    }

    /// Build a new [`Responder`].
    ///
    /// This requires that a `longterm_ecdh_keys`, and `recent_keys_upper_bound` is set.
    /// It also uses the `context`, `outer_aad`, and `longterm_pq_keys`.
    pub fn build_responder(self) -> Result<Responder<'a, Rng>, Error> {
        let Some(longterm_ecdh_keys) = self.longterm_ecdh_keys else {
            return Err(Error::BuilderState);
        };

        let Some(recent_keys_upper_bound) = self.recent_keys_upper_bound else {
            return Err(Error::BuilderState);
        };

        Ok(Responder::new(
            longterm_ecdh_keys,
            self.longterm_pq_keys,
            self.context,
            self.outer_aad,
            recent_keys_upper_bound,
            self.rng,
        ))
    }
}

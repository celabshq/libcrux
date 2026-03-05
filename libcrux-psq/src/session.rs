//! # Longterm Session
//!
//! This module implements long-term sessions derived from PSQ handshakes.

mod session_key;
mod transport;

use std::io::Cursor;

use session_key::{derive_session_key, SessionKey, SESSION_ID_LENGTH};
use tls_codec::{
    Deserialize, Serialize, SerializeBytes, Size, TlsDeserialize, TlsSerialize, TlsSize,
};
pub use transport::Transport;

use crate::{
    aead::{AEADError, AEADKeyNonce, AeadType},
    handshake::{
        ciphersuite::types::PQEncapsulationKey, dhkem::DHPublicKey, transcript::Transcript,
        types::Authenticator,
    },
    session::session_key::derive_import_key,
};

/// Session related errors
#[derive(Debug, PartialEq)]
pub enum SessionError {
    /// An error during session creation
    IntoSession,
    /// An error during serialization using TLSCodec
    Serialize(tls_codec::Error),
    /// An error during deserialization using TLSCodec
    Deserialize(tls_codec::Error),
    /// The given channel payload exceeded the maximum length
    PayloadTooLong(usize),
    /// An error in an underlying cryptographic primitive
    CryptoError,
    /// An error in storing or loading a session state
    Storage,
    /// The maxmium number of derivable channels has been reached
    ReachedMaxChannels,
    /// A channel message contains an inappropriate channel identifier
    IdentifierMismatch,
    /// The given payload exceeds the available output buffer
    OutputBufferShort,
    /// An error arising during the import of an external secret
    Import,
}

impl From<AEADError> for SessionError {
    fn from(value: AEADError) -> Self {
        match value {
            AEADError::CryptoError | AEADError::KeyExpired => SessionError::CryptoError,
            AEADError::Serialize(error) => SessionError::Serialize(error),
            AEADError::Deserialize(error) => SessionError::Deserialize(error),
        }
    }
}
/// The length of the public key binder in bytes.
pub(crate) const PK_BINDER_LEN: usize = 8;

#[derive(TlsSerialize, TlsDeserialize, TlsSize)]
#[repr(u8)]
/// The holder of a `Session` struct.
///
/// Note that this refers only to the roles during the initial handshake,
/// as either peer can initiate a secure channel from a session.
pub(crate) enum SessionPrincipal {
    /// The handshake initiator
    Initiator,
    /// The handshake responder
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
    /// Which handshake party holds the `Session`
    pub(crate) principal: SessionPrincipal,
    /// The long-term session key derived from the handshake
    pub(crate) session_key: SessionKey,
    /// Binds this `Session` to the long-term public key material of both
    /// parties that was used during the handshake
    pub(crate) pk_binder: Option<[u8; PK_BINDER_LEN]>,
    /// An increasing counter of derived secure channels
    pub(crate) channel_counter: u64,
    pub(crate) aead_type: AeadType,
    pub(crate) transcript: Transcript,
}

// pkBinder = KDF(skCS, g^c | g^s | [pkS])
fn derive_pk_binder(
    key: &SessionKey,
    initiator_authenticator: &Authenticator,
    responder_ecdh_pk: &DHPublicKey,
    responder_pq_pk: &Option<PQEncapsulationKey>,
) -> Result<[u8; PK_BINDER_LEN], SessionError> {
    #[derive(TlsSerialize, TlsSize)]
    struct PkBinderInfo<'a> {
        initiator_authenticator: &'a Authenticator,
        responder_ecdh_pk: &'a DHPublicKey,
        responder_pq_pk: &'a Option<PQEncapsulationKey<'a>>,
    }

    let info = PkBinderInfo {
        initiator_authenticator,
        responder_ecdh_pk,
        responder_pq_pk,
    };
    let mut info_buf = vec![0u8; info.tls_serialized_len()];
    info.tls_serialize(&mut &mut info_buf[..])
        .map_err(SessionError::Serialize)?;

    let mut binder = [0u8; PK_BINDER_LEN];

    libcrux_hkdf::sha2_256::hkdf(
        &mut binder,
        &[],
        &SerializeBytes::tls_serialize(&key.key).map_err(SessionError::Serialize)?,
        &info_buf,
    )
    .map_err(|_| SessionError::CryptoError)?;

    Ok(binder)
}

/// Wraps public key material that is bound to a session.
pub struct SessionBinding<'a> {
    /// The initiator's authenticator value, i.e. a long-term DH public value or signature verification key.
    pub initiator_authenticator: &'a Authenticator,
    /// The responder's long term DH public value.
    pub responder_ecdh_pk: &'a DHPublicKey,
    /// The responder's long term PQ-KEM public key (if any).
    pub responder_pq_pk: Option<PQEncapsulationKey<'a>>,
}

impl Session {
    /// Create a new `Session`.
    ///
    /// This will derive the long-term session key, and optionally compute a binder tying
    /// the session key to any long-term public key material that was used during the
    /// handshake.
    pub(crate) fn new<'a>(
        tx2: Transcript,
        k2: AEADKeyNonce,
        session_binding: Option<SessionBinding<'a>>,
        is_initiator: bool,
        aead_type: AeadType,
    ) -> Result<Self, SessionError> {
        let session_key = derive_session_key(k2, &tx2, aead_type)?;

        let pk_binder = session_binding
            .map(|session_binding| {
                derive_pk_binder(
                    &session_key,
                    session_binding.initiator_authenticator,
                    session_binding.responder_ecdh_pk,
                    &session_binding.responder_pq_pk,
                )
            })
            .transpose()?;

        Ok(Self {
            principal: if is_initiator {
                SessionPrincipal::Initiator
            } else {
                SessionPrincipal::Responder
            },
            session_key,
            pk_binder,
            channel_counter: 0,
            aead_type,
            transcript: tx2,
        })
    }

    /// Import a secret, replacing the main session secret
    ///
    /// A secret `psk` that is at least 32 bytes long can be imported
    /// into the session, replacing the original main session secret
    /// `K_S` with a fresh secret derived from the `psk` and `K_S`.
    /// If a public key binding is provided it must be the same as for
    /// the original session, but the new session will derive a fresh
    /// session ID and update the running transcript `tx` for future
    /// imports.
    ///
    /// In detail:
    /// ```text
    /// K_import = KDF(K_S || psk, "secret import")
    /// tx' = Hash(tx || session_ID)
    ///
    /// // From here: treat K_import as though it was the outcome of a handshake
    /// K_S' = KDF(K_import, "session secret" | tx')
    /// session_ID' = KDF(K_S', "shared key id")
    /// ```
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Initiator and responder finish the handshake resulting in
    /// // `initiator_session` and `responder_session` both bound to public
    /// // keys in `session_binding`.
    ///
    /// // WARN: In real usage, `psk` should be at least 32 bytes of high
    /// // entropy randomness.
    /// let psk = [0xab; 32];
    ///
    /// // Re-key the initiator session, providing the old session
    /// // binding. This sustains the binding to these public keys.
    /// // `session_binding`, if provided must match the binding of the
    /// // original session.
    /// let initiator_session = initiator_session.import(psk.as_slice(), session_binding).unwrap();
    ///
    /// // Re-key the responder session, providing the old session
    /// // binding. This sustains the binding to these public keys.
    /// // `session_binding`, if provided must match the binding of the
    /// // original session.
    /// let responder_session = initiator_session.import(psk.as_slice(), session_binding).unwrap();
    ///
    /// // [.. If `psk` was the same on both sides, you can now derive
    /// // transport channels from the re-keyed session as before ..]
    ///
    /// // WARN: In real usage, `another_psk` should be at least 32 bytes of high
    /// // entropy randomness.
    /// let another_psk = [0xcd; 32];
    ///
    /// // Re-key the initiator session, stripping the binding to the original
    /// // handshake public keys. Once the binding has been stripped it cannot
    /// // be re-established without performing a fresh handshake. Exercise
    /// // with caution to avoid session misbinding attacks.
    /// let unbound_initiator_session = initiator_session.import(another_psk.as_slice(), None).unwrap();
    ///
    /// // Re-key the responder session, stripping the binding to the original
    /// // handshake public keys. Once the binding has been stripped it cannot
    /// // be re-established without performing a fresh handshake. Exercise
    /// // with caution to avoid session misbinding attacks.
    /// let unbound_responder_session = responder_session.import(another_psk.as_slice(), None).unwrap();
    ///
    /// // [.. If `psk` was the same on both sides, you can now derive
    /// // transport channels from the re-keyed session as before ..]
    /// ```
    pub fn import<'a>(
        self,
        psk: &[u8],
        session_binding: impl Into<Option<SessionBinding<'a>>>,
    ) -> Result<Self, SessionError> {
        // We require that the psk is at least 32 bytes long.
        if psk.len() < 32 {
            return Err(SessionError::Import);
        }

        let session_binding = session_binding.into();

        match (self.pk_binder, &session_binding) {
            // No binder was present, no new binder was provided.
            (None, None) => (),
            // No binder was present, but a new binder was provided => We disallow re-binding a session after the binding has been lost.
            (None, Some(_)) => return Err(SessionError::Import),
            // Some binder was present and no other binder was provided => This removes the binding from the session for good
            (Some(_), None) => (),
            // Some binder was present and a binder was provided for
            // validation => The new session will be bound to the same
            // keys as the original, if the binder is valid
            (
                Some(pk_binder),
                Some(SessionBinding {
                    initiator_authenticator,
                    responder_ecdh_pk,
                    responder_pq_pk,
                }),
            ) => {
                if derive_pk_binder(
                    &self.session_key,
                    initiator_authenticator,
                    responder_ecdh_pk,
                    responder_pq_pk,
                )? != pk_binder
                {
                    return Err(SessionError::Import);
                }
            }
        };

        let transcript =
            Transcript::add_hash::<3>(Some(&self.transcript), self.session_key.identifier)
                .map_err(|_| SessionError::Import)?;

        let import_key = derive_import_key(self.session_key.key, psk, self.aead_type)?;

        Self::new(
            transcript,
            import_key,
            session_binding,
            matches!(self.principal, SessionPrincipal::Initiator),
            self.aead_type,
        )
    }

    /// Serializes the session state for storage.
    ///
    /// We require the caller to input the public keys (if any) that
    /// were used to create the session, in order to enforce they have
    /// access to all keys necessary to deserialize the session later
    /// on.
    ///
    /// WARN: `tls_serialize`
    /// should not be called directly, since it does not consume
    /// `Session`. This opens the possibility for nonce re-use by
    /// deserializing a stale `Session` since the original could be
    /// used after serialization.
    pub fn serialize<'a>(
        self,
        out: &mut [u8],
        session_binding: impl Into<Option<SessionBinding<'a>>>,
    ) -> Result<usize, SessionError> {
        let session_binding = session_binding.into();
        match (self.pk_binder, session_binding) {
            (None, None) => self
                .tls_serialize(&mut &mut out[..])
                .map_err(SessionError::Serialize),
            (None, Some(_)) | (Some(_), None) => Err(SessionError::Storage),
            (
                Some(pk_binder),
                Some(SessionBinding {
                    initiator_authenticator,
                    responder_ecdh_pk,
                    responder_pq_pk,
                }),
            ) => {
                if derive_pk_binder(
                    &self.session_key,
                    initiator_authenticator,
                    responder_ecdh_pk,
                    &responder_pq_pk,
                )? != pk_binder
                {
                    Err(SessionError::Storage)
                } else {
                    self.tls_serialize(&mut &mut out[..])
                        .map_err(SessionError::Serialize)
                }
            }
        }
    }

    /// Export a secret derived from the main session key.
    ///
    /// Derives a secret `K` from the main session key as
    /// `K = KDF(K_session, context || "PSQ secret export")`.
    pub fn export_secret(&self, context: &[u8], out: &mut [u8]) -> Result<(), SessionError> {
        use tls_codec::TlsSerializeBytes;
        const PSQ_EXPORT_CONTEXT: &[u8; 17] = b"PSQ secret export";
        #[derive(TlsSerializeBytes, TlsSize)]
        struct ExportInfo<'a> {
            context: &'a [u8],
            separator: [u8; 17],
        }

        libcrux_hkdf::sha2_256::hkdf(
            out,
            b"",
            self.session_key.key.as_ref(),
            &ExportInfo {
                context,
                separator: *PSQ_EXPORT_CONTEXT,
            }
            .tls_serialize()
            .map_err(SessionError::Serialize)?,
        )
        .map_err(|_| SessionError::CryptoError)
    }

    /// Deserialize a session state.
    ///
    /// If the session was bound to a set of public keys, those same public keys must be provided to validate the binding on deserialization.
    // XXX: Use `tls_codec::conditional_deserializable` to implement
    // the validation.
    pub fn deserialize<'a>(
        bytes: &[u8],
        session_binding: impl Into<Option<SessionBinding<'a>>>,
    ) -> Result<Self, SessionError> {
        let session_binding = session_binding.into();
        let mut session =
            Session::tls_deserialize(&mut Cursor::new(bytes)).map_err(SessionError::Deserialize)?;
        // The main session key is immediately expired since it
        // should not be used for transport encryption directly.
        session.session_key.key.expire();
        let session = session;

        match (session.pk_binder, session_binding) {
            // No binder was expected and none was provided.
            (None, None) => Ok(session),
            // No binder was expected, but a binder was provided =>
            // Error to signal that this session is not bound to the
            // provided binder.
            (None, Some(_)) => Err(SessionError::Storage),
            // Some binder was expected but none was provided.
            (Some(_), None) => Err(SessionError::Storage),
            // Some binder was expected and a binder was provided =>
            // Deserialization is valid, if binder is valid.
            (Some(pk_binder), Some(provided_binding)) => {
                if derive_pk_binder(
                    &session.session_key,
                    provided_binding.initiator_authenticator,
                    provided_binding.responder_ecdh_pk,
                    &provided_binding.responder_pq_pk,
                )? == pk_binder
                {
                    Ok(session)
                } else {
                    Err(SessionError::Storage)
                }
            }
        }
    }

    /// Derive a new secure transport channel from the session state.
    ///
    /// The new transport channel allows both peers to send and receive
    /// messages AEAD encrypted under a fresh channel key derived from the
    /// long-term session key.
    pub fn transport_channel(&mut self) -> Result<Transport, SessionError> {
        let channel = Transport::new(self, matches!(self.principal, SessionPrincipal::Initiator))?;
        self.channel_counter = self
            .channel_counter
            .checked_add(1)
            .ok_or(SessionError::ReachedMaxChannels)?;
        Ok(channel)
    }

    /// Output the channel identifier.
    pub fn identifier(&self) -> &[u8; SESSION_ID_LENGTH] {
        &self.session_key.identifier
    }
}

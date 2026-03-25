use std::{collections::VecDeque, io::Cursor, mem::take};

use rand::CryptoRng;
use tls_codec::{
    Deserialize, Serialize, Size, TlsDeserialize, TlsSerialize, TlsSize, VLByteSlice, VLBytes,
};

use super::{
    derive_k0, derive_k1_dh,
    dhkem::{DHPrivateKey, DHPublicKey, DHSharedSecret},
    initiator::InitiatorInnerPayload,
    transcript::{tx2, Transcript},
    write_output, HandshakeError as Error, HandshakeMessage, HandshakeMessageOut, K2IkmQuery,
    K2IkmRegistrationDh, ToTransportState,
};
use crate::{
    aead::{AEADKeyNonce, AeadType},
    handshake::{
        ciphersuite::{responder::ResponderCiphersuite, CiphersuiteName},
        derive_k1_sig,
        dhkem::DHKeyPair,
        transcript::verify_tx1,
        types::Authenticator,
        InnerMessage, K2IkmRegistrationSig,
    },
    session::{Session, SessionBinding, SessionError},
    traits::{Channel, IntoSession},
};

struct MessageCiphertext {
    ciphertext: Vec<u8>,
    tag: [u8; 16],
    ciphersuite: CiphersuiteName,
}

#[derive(TlsDeserialize, TlsSize)]
#[repr(u8)]
pub(crate) enum InitiatorOuterPayload {
    Query(VLBytes),
    Registration(InnerMessage),
}

#[derive(Debug)]
pub(crate) struct RespondQueryState {
    pub(crate) tx0: Transcript,
    pub(crate) k0: AEADKeyNonce,
    pub(crate) initiator_ephemeral_ecdh_pk: DHPublicKey,
}

pub(crate) struct RespondRegistrationState {
    pub(crate) tx1: Transcript,
    pub(crate) k1: AEADKeyNonce,
    pub(crate) initiator_ephemeral_ecdh_pk: DHPublicKey,
    pub(crate) initiator_authenticator: Authenticator,
    pub(crate) pq: bool,
}

#[derive(Default)]
pub(crate) enum ResponderState {
    #[default]
    InProgress, // A placeholder while computing the next state
    Initial,
    RespondQuery(Box<RespondQueryState>),
    RespondRegistration(Box<RespondRegistrationState>),
    ToTransport(Box<ToTransportState>),
}

pub struct Responder<'a, Rng: CryptoRng> {
    pub(crate) state: ResponderState,
    ciphersuite: ResponderCiphersuite<'a>,
    working_ciphersuite: Option<CiphersuiteName>,
    recent_keys: VecDeque<DHPublicKey>,
    recent_keys_upper_bound: usize,
    context: &'a [u8],
    aad: &'a [u8],
    rng: Rng,
}

#[derive(TlsDeserialize, TlsSize)]
pub struct ResponderQueryPayload(pub VLBytes);

#[derive(TlsSerialize, TlsSize)]
pub struct ResponderQueryPayloadOut<'a>(VLByteSlice<'a>);

#[derive(TlsDeserialize, TlsSize)]
pub struct ResponderRegistrationPayload(pub VLBytes);

#[derive(TlsSerialize, TlsSize)]
pub struct ResponderRegistrationPayloadOut<'a>(VLByteSlice<'a>);

impl<'a, Rng: CryptoRng> Responder<'a, Rng> {
    /// Returns the most recent initiator authenticator for out-of-band
    /// verification, if any.
    ///
    /// A responder in it's initial state or a responder that has processed an
    /// initiator query message returns `None`, as it does not have an
    /// initiator authenticator.
    ///
    /// A responder that has processed a registration initiator's first
    /// message will respond with the authenticator included in the message by
    /// the initiator:
    ///
    /// - For DH-based authentication, this will be the long-term ECDH public
    ///   key `pk_I` provided by the initiator. If the authenticator continues
    ///   the handshake, `pk_I` will be part of the session key derivation.
    /// - For signature-based authentication, this will be the signature
    ///   verification key `vk_I` provided by the initiator. At this point of
    ///   the handshake the initiator has provided a valid signature of the
    ///   running handshake transcript (`tx1`) under `vk_I`. If the
    ///   authenticator continues the handshake, `vk_I` will be part of the
    ///   session key derivation.
    pub fn initiator_authenticator(&self) -> Option<Authenticator> {
        match &self.state {
            ResponderState::InProgress
            | ResponderState::Initial
            | ResponderState::RespondQuery(_) => None,
            ResponderState::RespondRegistration(state) => {
                Some(state.initiator_authenticator.clone())
            }
            ResponderState::ToTransport(state) => state.initiator_authenticator.clone(),
        }
    }

    /// Abort an in-progress handshake.
    ///
    /// At any point in the handshake, the responder state can be
    /// reset to abort the handshake.
    pub fn abort_handshake(&mut self) {
        self.state = ResponderState::Initial {};
        self.working_ciphersuite = None;
    }

    pub(crate) fn new(
        ciphersuite: ResponderCiphersuite<'a>,
        context: &'a [u8],
        aad: &'a [u8],
        recent_keys_upper_bound: usize,
        rng: Rng,
    ) -> Self {
        Self {
            state: ResponderState::Initial {},
            ciphersuite,
            working_ciphersuite: None,
            context,
            aad,
            rng,
            recent_keys: VecDeque::with_capacity(recent_keys_upper_bound),
            recent_keys_upper_bound,
        }
    }

    fn derive_query_key(
        &self,
        tx0: &Transcript,
        k0: &AEADKeyNonce,
        responder_ephemeral_ecdh_pk: &DHPublicKey,
        responder_ephemeral_ecdh_sk: &DHPrivateKey,
        initiator_ephemeral_ecdh_pk: &DHPublicKey,
    ) -> Result<(Transcript, AEADKeyNonce), Error> {
        let tx2 = tx2(tx0, responder_ephemeral_ecdh_pk)?;
        let k2 = derive_k2_query_responder(
            k0,
            initiator_ephemeral_ecdh_pk,
            responder_ephemeral_ecdh_sk,
            &self.ciphersuite.kex.sk,
            &tx2,
            AeadType::ChaCha20Poly1305,
        )?;

        Ok((tx2, k2))
    }

    fn decrypt_outer_message(
        &self,
        initiator_outer_message: &HandshakeMessage,
    ) -> Result<(InitiatorOuterPayload, Transcript, AEADKeyNonce), Error> {
        let (tx0, mut k0) = derive_k0(
            &initiator_outer_message.pk,
            &self.ciphersuite.kex.pk,
            &self.ciphersuite.kex.sk,
            self.context,
            true,
            self.ciphersuite.aead_type(),
        )?;

        let initiator_payload: InitiatorOuterPayload = k0.handshake_decrypt(
            initiator_outer_message.ciphertext.as_slice(),
            &initiator_outer_message.tag,
            initiator_outer_message.aad.as_slice(),
        )?;

        Ok((initiator_payload, tx0, k0))
    }

    fn decrypt_inner_message(
        &self,
        tx0: &Transcript,
        k0: &AEADKeyNonce,
        initiator_inner_message: &InnerMessage,
    ) -> Result<
        (
            InitiatorInnerPayload,
            Transcript,
            AEADKeyNonce,
            Authenticator,
            bool,
        ),
        Error,
    > {
        let Some(working_ciphersuite) = self.working_ciphersuite else {
            return Err(Error::ResponderState);
        };

        let pq_encapsulation_deserialized = working_ciphersuite
            .deserialize_encapsulation(initiator_inner_message.pq_encapsulation.as_ref())?;

        let (authenticator, tx1) = verify_tx1(
            tx0,
            &initiator_inner_message.auth,
            if pq_encapsulation_deserialized.is_some() {
                self.ciphersuite.pq.encapsulation_key()
            } else {
                None
            },
            initiator_inner_message.pq_encapsulation.as_slice(),
        )?;

        let pq_shared_secret = pq_encapsulation_deserialized
            .as_ref()
            .as_ref()
            .map(|enc| self.ciphersuite.pq_decapsulate(enc))
            .transpose()?;

        let mut k1 = match &initiator_inner_message.auth {
            super::AuthMessage::Dh(dhpublic_key) => derive_k1_dh(
                k0,
                &self.ciphersuite.kex.sk,
                dhpublic_key,
                pq_shared_secret,
                &tx1,
                self.ciphersuite.aead_type(),
            ),
            super::AuthMessage::Sig { vk: _, signature } => derive_k1_sig(
                k0,
                pq_shared_secret,
                &tx1,
                signature,
                self.ciphersuite.aead_type(),
            ),
        }?;

        let inner_payload: InitiatorInnerPayload = k1.handshake_decrypt(
            initiator_inner_message.ciphertext.as_slice(),
            &initiator_inner_message.tag,
            initiator_inner_message.aad.as_slice(),
        )?;

        Ok((
            inner_payload,
            tx1,
            k1,
            authenticator,
            pq_encapsulation_deserialized.is_some(),
        ))
    }

    /// Compute registration response and set state to `ToTransport`.
    fn registration(
        &mut self,
        payload: &[u8],
        responder_ephemeral_ecdh_keys: &DHKeyPair,
        state: RespondRegistrationState,
    ) -> Result<MessageCiphertext, Error> {
        let tx2 = tx2(&state.tx1, &responder_ephemeral_ecdh_keys.pk)?;
        let mut k2 = match &state.initiator_authenticator {
            Authenticator::Dh(dhpublic_key) => derive_k2_registration_responder_dh(
                &state.k1,
                &tx2,
                dhpublic_key,
                &state.initiator_ephemeral_ecdh_pk,
                &responder_ephemeral_ecdh_keys.sk,
                self.ciphersuite.aead_type(),
            )?,
            Authenticator::Sig(_) => derive_k2_registration_responder_sig(
                &state.k1,
                &tx2,
                &state.initiator_ephemeral_ecdh_pk,
                &responder_ephemeral_ecdh_keys.sk,
                self.ciphersuite.aead_type(),
            )?,
        };

        let outer_payload = ResponderRegistrationPayloadOut(VLByteSlice(payload));
        let (ciphertext, tag) = k2.handshake_encrypt(&outer_payload, self.aad)?;

        let Some(working_ciphersuite) = self.working_ciphersuite else {
            return Err(Error::ResponderState);
        };

        self.state = ResponderState::ToTransport(
            ToTransportState {
                tx2,
                k2,
                initiator_authenticator: Some(state.initiator_authenticator),
                pq: state.pq,
            }
            .into(),
        );

        Ok(MessageCiphertext {
            ciphertext,
            tag,
            ciphersuite: working_ciphersuite,
        })
    }

    /// Compute query response and reset state to `Initial`.
    fn query(
        &mut self,
        payload: &[u8],
        responder_ephemeral_ecdh_keys: &DHKeyPair,
        state: RespondQueryState,
    ) -> Result<MessageCiphertext, Error> {
        let (_tx2, mut k2) = self.derive_query_key(
            &state.tx0,
            &state.k0,
            &responder_ephemeral_ecdh_keys.pk,
            &responder_ephemeral_ecdh_keys.sk,
            &state.initiator_ephemeral_ecdh_pk,
        )?;

        let outer_payload = ResponderQueryPayloadOut(VLByteSlice(payload));
        let (ciphertext, tag) = k2.handshake_encrypt(&outer_payload, self.aad)?;

        self.state = ResponderState::Initial;

        Ok(MessageCiphertext {
            ciphertext,
            tag,
            ciphersuite: CiphersuiteName::query_ciphersuite(),
        })
    }

    /// Compute response message elements and update responder state.
    fn prepare_message_contents(
        &mut self,
        payload: &[u8],
    ) -> Result<(DHPublicKey, MessageCiphertext), Error> {
        let state = take(&mut self.state);
        let responder_ephemeral_ecdh_keys = DHKeyPair::new(&mut self.rng);

        let message_contents = match state {
            ResponderState::RespondQuery(respond_query_state) => self.query(
                payload,
                &responder_ephemeral_ecdh_keys,
                *respond_query_state,
            )?,
            ResponderState::RespondRegistration(respond_registration_state) => self.registration(
                payload,
                &responder_ephemeral_ecdh_keys,
                *respond_registration_state,
            )?,
            _ => return Err(Error::ResponderState),
        };

        Ok((responder_ephemeral_ecdh_keys.pk, message_contents))
    }

    /// Check whether `pk` is contained in the set of most recently
    /// seen public keys.
    fn check_rate_limit(&mut self, pk: &DHPublicKey) -> Result<(), Error> {
        if self.recent_keys.contains(pk) {
            return Err(Error::RateLimit);
        } else {
            if self.recent_keys.len() == self.recent_keys_upper_bound {
                self.recent_keys.pop_back();
            }
            self.recent_keys.push_front(pk.clone());
        }
        Ok(())
    }

    /// Read message payload and update responder state.
    fn read_message_contents(
        &mut self,
        initiator_outer_message: &HandshakeMessage,
    ) -> Result<Vec<u8>, Error> {
        // Check that the ephemeral key was not in the most recent keys.
        self.check_rate_limit(&initiator_outer_message.pk)?;

        // Set the working ciphersuite for this handshake.
        self.working_ciphersuite = Some(
            initiator_outer_message
                .ciphersuite
                .coerce_compatible(&self.ciphersuite)?,
        );

        // Decrypt the outer message payload.
        let (initiator_outer_payload, tx0, k0) =
            self.decrypt_outer_message(&initiator_outer_message)?;

        match initiator_outer_payload {
            InitiatorOuterPayload::Query(initiator_query_payload) => {
                // We're ready to respond to the query message.
                self.state = ResponderState::RespondQuery(
                    RespondQueryState {
                        tx0,
                        k0,
                        initiator_ephemeral_ecdh_pk: initiator_outer_message.pk,
                    }
                    .into(),
                );
                Ok(initiator_query_payload.into())
            }

            InitiatorOuterPayload::Registration(initiator_inner_message) => {
                // Decrypt the inner message payload.
                match self.decrypt_inner_message(&tx0, &k0, &initiator_inner_message) {
                    Ok((initiator_inner_payload, tx1, k1, initiator_authenticator, pq)) => {
                        // We're ready to respond to the registration message.
                        self.state = ResponderState::RespondRegistration(
                            RespondRegistrationState {
                                tx1,
                                k1,
                                initiator_ephemeral_ecdh_pk: initiator_outer_message.pk,
                                initiator_authenticator,
                                pq,
                            }
                            .into(),
                        );
                        Ok(initiator_inner_payload.0.into())
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
}

impl<'a, Rng: CryptoRng> Channel<Error, HandshakeMessage> for Responder<'a, Rng> {
    fn write_message(&mut self, payload: &[u8], out: &mut [u8]) -> Result<usize, Error> {
        let (responder_ephemeral_ecdh_pk, message_contents) =
            self.prepare_message_contents(payload)?;

        let out_msg = HandshakeMessageOut {
            pk: &responder_ephemeral_ecdh_pk,
            ciphertext: VLByteSlice(&message_contents.ciphertext),
            tag: message_contents.tag,
            aad: VLByteSlice(self.aad),
            ciphersuite: message_contents.ciphersuite,
        };

        let bytes_serialized = out_msg
            .tls_serialize(&mut &mut out[..])
            .map_err(Error::Serialize)?;

        Ok(bytes_serialized)
    }

    fn read_message(
        &mut self,
        message_bytes: &[u8],
        out: &mut [u8],
    ) -> Result<(usize, usize), Error> {
        if !matches!(self.state, ResponderState::Initial) {
            return Err(Error::ResponderState);
        }

        // Deserialize the outer message.
        let initiator_outer_message =
            HandshakeMessage::tls_deserialize(&mut Cursor::new(&message_bytes))
                .map_err(Error::Deserialize)?;
        let bytes_deserialized = initiator_outer_message.tls_serialized_len();

        let inner_message_payload = self.read_message_contents(&initiator_outer_message)?;
        let out_bytes_written = write_output(&inner_message_payload, out)?;
        Ok((bytes_deserialized, out_bytes_written))
    }

    fn write_message_external_encoding(
        &mut self,
        payload: &[u8],
    ) -> Result<HandshakeMessage, Error> {
        let (responder_ephemeral_ecdh_pk, message_contents) =
            self.prepare_message_contents(payload)?;

        Ok(HandshakeMessage {
            pk: responder_ephemeral_ecdh_pk,
            ciphertext: message_contents.ciphertext,
            tag: message_contents.tag,
            aad: self.aad.to_vec(),
            ciphersuite: message_contents.ciphersuite,
        })
    }

    fn read_message_external_encoding(
        &mut self,
        message: &HandshakeMessage,
    ) -> Result<Vec<u8>, Error> {
        if !matches!(self.state, ResponderState::Initial) {
            return Err(Error::ResponderState);
        }

        self.read_message_contents(message)
    }
}

impl<'a, Rng: CryptoRng> IntoSession for Responder<'a, Rng> {
    fn into_session(self) -> Result<Session, SessionError> {
        let ResponderState::ToTransport(mut state) = self.state else {
            return Err(SessionError::IntoSession);
        };

        let Some(initiator_authenticator) = take(&mut state.initiator_authenticator) else {
            return Err(SessionError::IntoSession);
        };

        Session::new(
            state.tx2,
            state.k2,
            Some(SessionBinding {
                initiator_authenticator: &initiator_authenticator,
                responder_ecdh_pk: &self.ciphersuite.kex.pk,
                responder_pq_pk: if state.pq {
                    self.ciphersuite.own_pq_encapsulation_key()
                } else {
                    None
                },
            }),
            false,
            self.ciphersuite.aead_type(),
        )
    }

    fn is_handshake_finished(&self) -> bool {
        matches!(self.state, ResponderState::ToTransport { .. })
    }
}

// K2 = KDF(K1 | g^cy | g^xy, tx2)
pub(super) fn derive_k2_registration_responder_dh(
    k1: &AEADKeyNonce,
    tx2: &Transcript,
    initiator_longterm_pk: &DHPublicKey,
    initiator_ephemeral_pk: &DHPublicKey,
    responder_ephemeral_sk: &DHPrivateKey,
    aead_type: AeadType,
) -> Result<AEADKeyNonce, Error> {
    let responder_ikm = K2IkmRegistrationDh {
        k1,
        g_cy: &DHSharedSecret::derive(responder_ephemeral_sk, initiator_longterm_pk)?,
        g_xy: &DHSharedSecret::derive(responder_ephemeral_sk, initiator_ephemeral_pk)?,
    };

    Ok(AEADKeyNonce::new(&responder_ikm, tx2, aead_type)?)
}

// K2 = KDF(K1 | g^xy, tx2)
pub(super) fn derive_k2_registration_responder_sig(
    k1: &AEADKeyNonce,
    tx2: &Transcript,
    initiator_ephemeral_pk: &DHPublicKey,
    responder_ephemeral_sk: &DHPrivateKey,
    aead_type: AeadType,
) -> Result<AEADKeyNonce, Error> {
    let responder_ikm = K2IkmRegistrationSig {
        k1,
        g_xy: &DHSharedSecret::derive(responder_ephemeral_sk, initiator_ephemeral_pk)?,
    };

    Ok(AEADKeyNonce::new(&responder_ikm, tx2, aead_type)?)
}

// K2 = KDF(K0 | g^xs | g^xy, tx2)
pub(super) fn derive_k2_query_responder(
    k0: &AEADKeyNonce,
    initiator_ephemeral_ecdh_pk: &DHPublicKey,
    responder_ephemeral_ecdh_sk: &DHPrivateKey,
    responder_longterm_ecdh_sk: &DHPrivateKey,
    tx2: &Transcript,
    aead_type: AeadType,
) -> Result<AEADKeyNonce, Error> {
    let responder_ikm = K2IkmQuery {
        k0,
        g_xs: &DHSharedSecret::derive(responder_longterm_ecdh_sk, initiator_ephemeral_ecdh_pk)?,
        g_xy: &DHSharedSecret::derive(responder_ephemeral_ecdh_sk, initiator_ephemeral_ecdh_pk)?,
    };

    Ok(AEADKeyNonce::new(&responder_ikm, tx2, aead_type)?)
}

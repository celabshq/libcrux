use std::{io::Cursor, mem::take};

use rand::CryptoRng;
use tls_codec::{Deserialize, Serialize, Size, VLByteSlice};

use crate::{
    aead::{AEADKeyNonce, AeadType},
    handshake::{
        ciphersuite::{initiator::InitiatorCiphersuite, traits::CiphersuiteBase},
        derive_k0, derive_k1_dh, derive_k1_sig,
        dhkem::{DHKeyPair, DHPrivateKey, DHPublicKey, DHSharedSecret},
        responder::ResponderRegistrationPayload,
        transcript::{sign_tx1, tx2, Transcript},
        write_output, AuthMessageOut, HandshakeError as Error, HandshakeMessage,
        HandshakeMessageOut, InnerMessageOut, K2IkmRegistrationDh, K2IkmRegistrationSig,
        ToTransportState,
    },
    session::{Session, SessionBinding, SessionError},
    traits::{Channel, IntoSession},
};

use super::{InitiatorInnerPayloadOut, InitiatorOuterPayloadOut};

pub struct RegistrationInitiator<'a, Rng: CryptoRng> {
    ciphersuite: InitiatorCiphersuite<'a>,
    inner_aad: &'a [u8],
    outer_aad: &'a [u8],
    rng: Rng,
    state: RegistrationInitiatorState,
}

pub(crate) struct InitialState {
    initiator_ephemeral_keys: DHKeyPair,
    tx0: Transcript,
    k0: AEADKeyNonce,
}

pub(crate) struct WaitingState {
    initiator_ephemeral_ecdh_sk: DHPrivateKey,
    tx1: Transcript,
    k1: AEADKeyNonce,
}

#[derive(Default)]
pub(crate) enum RegistrationInitiatorState {
    #[default]
    InProgress, // A placeholder while computing the next state
    Initial(Box<InitialState>),
    Waiting(Box<WaitingState>),
    ToTransport(Box<ToTransportState>),
}

impl<'a, Rng: CryptoRng> RegistrationInitiator<'a, Rng> {
    /// Create a new [`RegistrationInitiator`].
    pub(crate) fn new(
        ciphersuite: InitiatorCiphersuite<'a>,
        ctx: &[u8],
        inner_aad: &'a [u8],
        outer_aad: &'a [u8],
        mut rng: Rng,
    ) -> Result<Self, Error> {
        let initiator_ephemeral_keys = DHKeyPair::new(&mut rng);

        let (tx0, k0) = derive_k0(
            ciphersuite.kex,
            &initiator_ephemeral_keys.pk,
            &initiator_ephemeral_keys.sk,
            ctx,
            false,
            ciphersuite.aead_type(),
        )?;

        let state = RegistrationInitiatorState::Initial(
            InitialState {
                tx0,
                k0,
                initiator_ephemeral_keys,
            }
            .into(),
        );

        Ok(Self {
            ciphersuite,
            inner_aad,
            outer_aad,
            rng,
            state,
        })
    }
}

impl<'a, Rng: CryptoRng> Channel<Error> for RegistrationInitiator<'a, Rng> {
    fn write_message(&mut self, payload: &[u8], out: &mut [u8]) -> Result<usize, Error> {
        let RegistrationInitiatorState::Initial(mut state) = take(&mut self.state) else {
            // If we're not in the initial state, we write nothing
            return Ok(0);
        };

        let (pq_encapsulation, pq_shared_secret) =
            self.ciphersuite.pq_encapsulate(&mut self.rng)?;

        let pq_encapsulation_serialized = pq_encapsulation
            .tls_serialize_detached()
            .map_err(|_e| Error::OutputBufferShort)?;

        let (signature, tx1) = sign_tx1(
            &state.tx0,
            self.ciphersuite.auth,
            self.ciphersuite.pq.into(),
            &pq_encapsulation_serialized,
            &mut self.rng,
        )?;

        let (k1, outer_ciphertext, outer_tag) = match self.ciphersuite.auth {
            crate::handshake::ciphersuite::initiator::Auth::DH(dhkey_pair) => {
                let mut k1 = derive_k1_dh(
                    &state.k0,
                    &dhkey_pair.sk,
                    self.ciphersuite.kex,
                    pq_shared_secret,
                    &tx1,
                    self.ciphersuite.aead_type(),
                )?;

                let inner_payload = InitiatorInnerPayloadOut(VLByteSlice(payload));
                let (inner_ciphertext, inner_tag) =
                    k1.handshake_encrypt(&inner_payload, self.inner_aad)?;

                let outer_payload = InitiatorOuterPayloadOut::Registration(InnerMessageOut {
                    auth: AuthMessageOut::Dh(&dhkey_pair.pk),
                    ciphertext: VLByteSlice(&inner_ciphertext),
                    tag: inner_tag,
                    aad: VLByteSlice(self.inner_aad),
                    pq_encapsulation: VLByteSlice(&pq_encapsulation_serialized),
                });
                let (outer_ciphertext, outer_tag) =
                    state.k0.handshake_encrypt(&outer_payload, self.outer_aad)?;

                (k1, outer_ciphertext, outer_tag)
            }
            crate::handshake::ciphersuite::initiator::Auth::Sig(sig_auth) => {
                let mut k1 = derive_k1_sig(
                    &state.k0,
                    pq_shared_secret,
                    &tx1,
                    signature.as_ref().expect("signature based initiator authentication produces a signature, or propagates the signing error before"),
                    self.ciphersuite.aead_type(),
                )?;

                let inner_payload = InitiatorInnerPayloadOut(VLByteSlice(payload));
                let (inner_ciphertext, inner_tag) =
                    k1.handshake_encrypt(&inner_payload, self.inner_aad)?;

                let outer_payload = InitiatorOuterPayloadOut::Registration(InnerMessageOut {
                    auth: AuthMessageOut::Sig {
                        vk: &sig_auth.into(),
                        signature: signature.as_ref().expect("signature based initiator authentication produces a signature, or propagates the signing error before"),
                    },
                    ciphertext: VLByteSlice(&inner_ciphertext),
                    tag: inner_tag,
                    aad: VLByteSlice(self.inner_aad),
                    pq_encapsulation: VLByteSlice(&pq_encapsulation_serialized),
                });
                let (outer_ciphertext, outer_tag) =
                    state.k0.handshake_encrypt(&outer_payload, self.outer_aad)?;
                (k1, outer_ciphertext, outer_tag)
            }
        };

        let msg = HandshakeMessageOut {
            pk: &state.initiator_ephemeral_keys.pk,
            ciphertext: VLByteSlice(&outer_ciphertext),
            tag: outer_tag,
            aad: VLByteSlice(self.outer_aad),
            ciphersuite: self.ciphersuite.name(),
        };

        let out_bytes_written = msg
            .tls_serialize(&mut &mut out[..])
            .map_err(Error::Serialize)?;

        self.state = RegistrationInitiatorState::Waiting(
            WaitingState {
                initiator_ephemeral_ecdh_sk: state.initiator_ephemeral_keys.sk,
                tx1,
                k1,
            }
            .into(),
        );

        Ok(out_bytes_written)
    }

    fn read_message(
        &mut self,
        message_bytes: &[u8],
        out: &mut [u8],
    ) -> Result<(usize, usize), Error> {
        let RegistrationInitiatorState::Waiting(state) = take(&mut self.state) else {
            // If we're not in the waiting state, we do nothing.
            return Ok((0, 0));
        };

        // Deserialize the message.
        let responder_msg = HandshakeMessage::tls_deserialize(&mut Cursor::new(&message_bytes))
            .map_err(Error::Deserialize)?;
        let bytes_deserialized = responder_msg.tls_serialized_len();

        // Derive K2
        let tx2 = tx2(&state.tx1, &responder_msg.pk)?;
        let mut k2 = match self.ciphersuite.auth {
            crate::handshake::ciphersuite::initiator::Auth::DH(dhkey_pair) => {
                derive_k2_registration_initiator_dh(
                    &state.k1,
                    &tx2,
                    &dhkey_pair.sk,
                    &state.initiator_ephemeral_ecdh_sk,
                    &responder_msg.pk,
                    self.ciphersuite.aead_type,
                )?
            }
            crate::handshake::ciphersuite::initiator::Auth::Sig(_) => {
                derive_k2_registration_initiator_sig(
                    &state.k1,
                    &tx2,
                    &state.initiator_ephemeral_ecdh_sk,
                    &responder_msg.pk,
                    self.ciphersuite.aead_type,
                )?
            }
        };

        // Decrypt Payload
        let registration_response: ResponderRegistrationPayload = k2.handshake_decrypt(
            responder_msg.ciphertext.as_slice(),
            &responder_msg.tag,
            responder_msg.aad.as_slice(),
        )?;

        let out_bytes_written = write_output(registration_response.0.as_slice(), out)?;

        self.state = RegistrationInitiatorState::ToTransport(
            ToTransportState {
                tx2,
                k2,
                initiator_authenticator: None,
                pq: self.ciphersuite.is_pq(),
            }
            .into(),
        );

        Ok((bytes_deserialized, out_bytes_written))
    }
}

impl<'a, Rng: CryptoRng> IntoSession for RegistrationInitiator<'a, Rng> {
    fn into_session(self) -> Result<Session, SessionError> {
        let RegistrationInitiatorState::ToTransport(state) = self.state else {
            return Err(SessionError::IntoSession);
        };

        Session::new(
            state.tx2,
            state.k2,
            Some(SessionBinding {
                initiator_authenticator: &self.ciphersuite.auth.into(),
                responder_ecdh_pk: self.ciphersuite.peer_ecdh_encapsulation_key(),
                responder_pq_pk: self.ciphersuite.peer_pq_encapsulation_key(),
            }),
            true,
            self.ciphersuite.aead_type(),
        )
    }

    fn is_handshake_finished(&self) -> bool {
        matches!(self.state, RegistrationInitiatorState::ToTransport(_))
    }
}

// K2 = KDF(K1 | g^cy | g^xy, tx2)
pub(super) fn derive_k2_registration_initiator_dh(
    k1: &AEADKeyNonce,
    tx2: &Transcript,
    initiator_longterm_sk: &DHPrivateKey,
    initiator_ephemeral_sk: &DHPrivateKey,
    responder_ephemeral_pk: &DHPublicKey,
    aead_type: AeadType,
) -> Result<AEADKeyNonce, Error> {
    let responder_ikm = K2IkmRegistrationDh {
        k1,
        g_cy: &DHSharedSecret::derive(initiator_longterm_sk, responder_ephemeral_pk)?,
        g_xy: &DHSharedSecret::derive(initiator_ephemeral_sk, responder_ephemeral_pk)?,
    };

    AEADKeyNonce::new(&responder_ikm, tx2, aead_type).map_err(|e| e.into())
}

// K2 = KDF(K1 | g^xy, tx2)
pub(super) fn derive_k2_registration_initiator_sig(
    k1: &AEADKeyNonce,
    tx2: &Transcript,
    initiator_ephemeral_sk: &DHPrivateKey,
    responder_ephemeral_pk: &DHPublicKey,
    aead_type: AeadType,
) -> Result<AEADKeyNonce, Error> {
    let responder_ikm = K2IkmRegistrationSig {
        k1,
        g_xy: &DHSharedSecret::derive(initiator_ephemeral_sk, responder_ephemeral_pk)?,
    };

    AEADKeyNonce::new(&responder_ikm, tx2, aead_type).map_err(|e| e.into())
}

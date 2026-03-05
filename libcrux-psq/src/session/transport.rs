//! # Secure Transport Channels
use std::io::Cursor;

use tls_codec::{
    Deserialize, Serialize, SerializeBytes, Size, TlsDeserialize, TlsSerialize, TlsSerializeBytes,
    TlsSize, VLByteSlice, VLBytes,
};

#[cfg(feature = "nonce-control")]
use crate::aead::NONCE_LEN;
use crate::{aead::AEADKeyNonce, traits::Channel};

use super::{Session, SessionError as Error};

#[derive(TlsSerialize, TlsSize)]
/// A message to be sent on the transport channel. (Serialization helper)
struct TransportMessageOut<'a> {
    /// The channel identifier
    channel_identifier: u64,
    /// AEAD ciphertext containing the message payload
    ciphertext: VLByteSlice<'a>,
    /// AEAD message authentication tag over the cipertext.
    tag: [u8; 16],
}

#[derive(TlsDeserialize, TlsSize)]
/// A message to be sent on the transport channel.
struct TransportMessage {
    /// The channel identifier
    channel_identifier: u64,
    /// AEAD ciphertext containing the message payload
    ciphertext: VLBytes,
    /// AEAD message authentication tag over the cipertext.
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
///
/// To prevent de-syncing of states between sender and receiver a
/// failed decryption (e.g. due to transmission errors) will leave the
/// receiver nonce unchanged, meaning the decrypting party can ask for
/// retransmission of the faulty ciphertext and re-attempt decryption.
pub struct Transport {
    /// Key used for AEAD-encrypting messages to be sent
    send_key: AEADKeyNonce,
    /// Key used for AEAD-decrypting received messages
    recv_key: AEADKeyNonce,
    /// Identifier sent with each message on this channel. Stays constant
    /// during the lifetime of the channel
    channel_identifier: u64,
}

impl Transport {
    /// Create a new channel from a `Session`.
    ///
    /// The `is_initiator` argument decides which derived key is used for
    /// sending and which for receiving of messages.
    pub(crate) fn new(session: &Session, is_initiator: bool) -> Result<Self, Error> {
        if is_initiator {
            Ok(Self {
                send_key: derive_i2r_channel_key(session)?,
                recv_key: derive_r2i_channel_key(session)?,
                channel_identifier: session.channel_counter,
            })
        } else {
            Ok(Self {
                send_key: derive_r2i_channel_key(session)?,
                recv_key: derive_i2r_channel_key(session)?,
                channel_identifier: session.channel_counter,
            })
        }
    }

    /// Returns the channel identifier.
    pub fn identifier(&self) -> u64 {
        self.channel_identifier
    }

    /// Set the nonce used for encrypting outgoing messages.
    #[cfg(feature = "nonce-control")]
    pub fn set_sender_nonce(&mut self, nonce: &[u8; NONCE_LEN]) {
        self.send_key.set_nonce(nonce);
    }

    /// Set the nonce used for decrypting incoming messages.
    #[cfg(feature = "nonce-control")]
    pub fn set_receiver_nonce(&mut self, nonce: &[u8; NONCE_LEN]) {
        self.recv_key.set_nonce(nonce);
    }

    /// Get the current nonce used for encrypting outgoing messages.
    #[cfg(feature = "nonce-control")]
    pub fn sender_nonce(&self) -> &[u8; NONCE_LEN] {
        self.send_key.nonce()
    }

    /// Get the current nonce used for decrypting incoming messages.
    #[cfg(feature = "nonce-control")]
    pub fn receiver_nonce(&self) -> &[u8; NONCE_LEN] {
        self.send_key.nonce()
    }
}

impl Channel<Error> for Transport {
    fn write_message(&mut self, payload: &[u8], out: &mut [u8]) -> Result<usize, Error> {
        // We match the maximum payload length of Noise.
        if payload.len() > 65535 {
            return Err(Error::PayloadTooLong(payload.len()));
        }
        let mut ciphertext = vec![0u8; payload.len()];
        let tag = self.send_key.encrypt(payload, &[], &mut ciphertext)?;
        let message = TransportMessageOut {
            channel_identifier: self.channel_identifier,
            ciphertext: VLByteSlice(ciphertext.as_ref()),
            tag,
        };

        message
            .tls_serialize(&mut &mut out[..])
            .map_err(Error::Serialize)
    }

    fn read_message(&mut self, message: &[u8], out: &mut [u8]) -> Result<(usize, usize), Error> {
        let message = TransportMessage::tls_deserialize(&mut Cursor::new(message))
            .map_err(Error::Deserialize)?;

        if self.channel_identifier != message.channel_identifier {
            return Err(Error::IdentifierMismatch);
        }

        if out.len() < message.ciphertext.as_slice().len() {
            return Err(Error::OutputBufferShort);
        }

        let bytes_deserialized = message.tls_serialized_len();

        self.recv_key.decrypt_out(
            message.ciphertext.as_slice(),
            &message.tag,
            &[],
            &mut out[..message.ciphertext.as_slice().len()],
        )?;

        let out_bytes_written = message.ciphertext.as_slice().len();

        Ok((bytes_deserialized, out_bytes_written))
    }
}

const I2R_CHANNEL_KEY_LABEL: &[u8] = b"i2r channel key";
const R2I_CHANNEL_KEY_LABEL: &[u8] = b"r2i channel key";

// skChanneli2r = KDF(skCS, "i2r channel key" | pk_binder | channel_counter)
// skChannelr2i = KDF(skCS, "r2i channel key" | pk_binder | channel_counter)
fn derive_channel_key<const IS_INITIATOR: bool>(session: &Session) -> Result<AEADKeyNonce, Error> {
    #[derive(TlsSerializeBytes, TlsSize)]
    struct ChannelKeyInfo<'a> {
        domain_separator: &'static [u8],
        pk_binder: Option<&'a [u8]>,
        counter: u64,
    }

    AEADKeyNonce::new(
        &session.session_key.key,
        &ChannelKeyInfo {
            domain_separator: if IS_INITIATOR {
                I2R_CHANNEL_KEY_LABEL
            } else {
                R2I_CHANNEL_KEY_LABEL
            },
            pk_binder: session
                .pk_binder
                .as_ref()
                .map(|pk_binder| pk_binder.as_slice()),
            counter: session.channel_counter,
        }
        .tls_serialize()
        .map_err(Error::Serialize)?,
        session.aead_type,
    )
    .map_err(|e| e.into())
}

pub(super) fn derive_i2r_channel_key(session: &Session) -> Result<AEADKeyNonce, Error> {
    derive_channel_key::<true>(session)
}

pub(super) fn derive_r2i_channel_key(session: &Session) -> Result<AEADKeyNonce, Error> {
    derive_channel_key::<false>(session)
}

use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize, VLByteSlice, VLBytes};

use crate::handshake::InnerMessageOut;

pub mod query;
pub mod registration;

#[derive(TlsSerialize, TlsSize)]
#[repr(u8)]
pub(crate) enum InitiatorOuterPayloadOut<'a> {
    Query(VLByteSlice<'a>),
    Registration(InnerMessageOut<'a>),
}

#[derive(TlsDeserialize, TlsSize)]
pub(crate) struct InitiatorInnerPayload(pub VLBytes);

#[derive(TlsSerialize, TlsSize)]
pub(crate) struct InitiatorInnerPayloadOut<'a>(pub VLByteSlice<'a>);

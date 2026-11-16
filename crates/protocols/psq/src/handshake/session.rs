use tls_codec::{TlsDeserialize, TlsSerialize, TlsSize};

use super::keys::AEADKey;

/// The length of a session ID in bytes.
pub const SESSION_ID_LENGTH: usize = 32;

/// The length of a sessin key in bytes.
pub const SESSION_KEY_LENGTH: usize = 32;

#[derive(TlsSerialize, TlsDeserialize, TlsSize)]
pub struct SessionKey {
    pub(crate) identifier: [u8; SESSION_ID_LENGTH],
    pub(crate) key: AEADKey,
}

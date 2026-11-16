//! # Ciphersuite Traits
//!
//! This module defines common behaviour for all initiator and responder ciphersuites.

use tls_codec::{Deserialize, Serialize, SerializeBytes, Size};

use crate::handshake::ciphersuite::CiphersuiteName;

/// Functionality and associated types that are shared in common
/// between initiator and responde ciphersuites.
pub(crate) trait CiphersuiteBase {
    type Ciphertext: Serialize + Deserialize + Size;
    type EncapsulationKeyRef: Serialize + Size;
    type SharedSecret: SerializeBytes + Size;

    fn name(&self) -> CiphersuiteName;
}

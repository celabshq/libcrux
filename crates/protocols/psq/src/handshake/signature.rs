use tls_codec::{TlsDeserializeBytes, TlsSerializeBytes, TlsSize};

#[derive(Debug, Clone, PartialEq, TlsSerializeBytes, TlsDeserializeBytes, TlsSize)]
#[repr(u8)]
pub(crate) enum Signature {
    Ed25519(Vec<u8>),
    MlDsa(Vec<u8>),
}
#[derive(Debug, Clone, PartialEq, TlsSerializeBytes, TlsDeserializeBytes, TlsSize)]
#[repr(u8)]
pub(crate) enum VerificationKey {
    Ed25519(Vec<u8>),
    MlDsa(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub(crate) enum SigningKey {
    Ed25519(Vec<u8>),
    MlDsa(Vec<u8>),
}

pub(crate) type CredentialKeyPair = (SigningKey, VerificationKey);

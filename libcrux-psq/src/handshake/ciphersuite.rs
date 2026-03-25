use std::io::Cursor;

use tls_codec::{Deserialize, TlsDeserialize, TlsSerialize, TlsSize};

use crate::handshake::{
    ciphersuite::{responder::ResponderCiphersuite, traits::CiphersuiteBase, types::PQCiphertext},
    HandshakeError,
};
pub(crate) const PSQ_MLDSA_CONTEXT: &[u8] = b"PSQ";

pub mod builder;
pub mod initiator;
pub mod responder;
pub mod traits;
pub mod types;

#[derive(TlsSize, TlsSerialize, TlsDeserialize, Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
/// Available ciphersuites for use in the PSQ handshake.
///
/// The name assigns different algorithms according to the following mask:
/// ```ignore
/// OUTER_PQKEM_AUTH_AEAD_KDF
/// ```
///
/// where
/// - `OUTER` is the elliptic curve Diffie-Hellman key exchange used for the
///   outer messsage layer. Supported curves at this point are: `X25519`
/// - `PQKEM` is the PQ-KEM used in the inner message. Supported PQ-KEMs at this
///   point are: `MLKEM768`, `CLASSICMCELIECE` (using feature
///   `classic-mceliece`) and `NONE` (indicating no PQ-KEM will be used)
/// - `AUTH` is the method of initiator authentication used in the inner
///   message. Supported authentication methods at this point are authentication
///   via the initiator's long-term `X25519` public key, or authentication via a
///   signature under the initiator's long-term signing key. Supported signature
///   schemes at this point are: `MLDSA65` and `ED25519`. Initiator long term
///   public and signing keys are assumed to be available to responder
///   out-of-band.
/// - `AEAD` is the AEAD used for encrypting message payloads. Supported AEADs
///   at this point are: `CHACHA20POLY1305` and `AESGCM128`.
/// - `KDF` is the key derivation function used to derive AEAD keys. Supported
///   KDFs at this point are `HKDFSHA256`.
pub enum CiphersuiteName {
    /// Use X25519 for the outer key exchange, no PQ-KEM, X25519 for
    /// DH-based initiator autentication, Chacha20Poly1305 as payload
    /// AEAD and SHA-256 for HKDF. This is the only ciphersuite
    /// supported in Query mode, where no client authentication takes
    /// place.
    X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256,
    /// Use X25519 for the outer key exchange, ML-KEM 768 for PQ-KEM,
    /// X25519 for DH-based initiator autentication, Chacha20Poly1305
    /// as payload AEAD and SHA-256 for HKDF.
    X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256,
    /// Use X25519 for the outer key exchange, Classic McEliece for
    /// PQ-KEM, X25519 for DH-based initiator autentication,
    /// Chacha20Poly1305 as payload AEAD and SHA-256 for
    /// HKDF. (requires feature `classic-mceliece`)
    X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256,
    /// Use X25519 for the outer key exchange, no PQ-KEM, X25519 for
    /// DH-based initiator autentication, AesGcm128 as payload AEAD
    /// and SHA-256 for HKDF.
    X25519_NONE_X25519_AESGCM128_HKDFSHA256,
    /// Use X25519 for the outer key exchange, ML-KEM 768 for PQ-KEM,
    /// X25519 for DH-based initiator autentication, AesGcm128 as
    /// payload AEAD and SHA-256 for HKDF.
    X25519_MLKEM768_X25519_AESGCM128_HKDFSHA256,
    /// Use X25519 for the outer key exchange, Classic McEliece for
    /// PQ-KEM, X25519 for DH-based initiator autentication, AesGcm128
    /// as payload AEAD and SHA-256 for HKDF. (requires feature
    /// `classic-mceliece`)
    X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256,

    /// Use X25519 for the outer key exchange, no PQ-KEM, Ed25519 for
    /// signature-based initiator autentication, Chacha20Poly1305 as
    /// payload AEAD and SHA-256 for HKDF. This is the only
    /// ciphersuite supported in Query mode, where no client
    /// authentication takes place.
    X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256,
    /// Use X25519 for the outer key exchange, ML-KEM 768 for PQ-KEM,
    /// Ed25519 for signature-based initiator autentication,
    /// Chacha20Poly1305 as payload AEAD and SHA-256 for HKDF.
    X25519_MLKEM768_ED25519_CHACHA20POLY1305_HKDFSHA256,
    /// Use X25519 for the outer key exchange, Classic McEliece for
    /// PQ-KEM, Ed25519 for signature-based initiator autentication,
    /// Chacha20Poly1305 as payload AEAD and SHA-256 for
    /// HKDF. (requires feature `classic-mceliece`)
    X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256,
    /// Use X25519 for the outer key exchange, no PQ-KEM, Ed25519 for
    /// signature-based initiator autentication, AesGcm128 as payload AEAD
    /// and SHA-256 for HKDF.
    X25519_NONE_ED25519_AESGCM128_HKDFSHA256,
    /// Use X25519 for the outer key exchange, ML-KEM 768 for PQ-KEM,
    /// Ed25519 for signature-based initiator autentication, AesGcm128 as
    /// payload AEAD and SHA-256 for HKDF.
    X25519_MLKEM768_ED25519_AESGCM128_HKDFSHA256,
    /// Use X25519 for the outer key exchange, Classic McEliece for
    /// PQ-KEM, Ed25519 for signature-based initiator autentication,
    /// AesGcm128 as payload AEAD and SHA-256 for HKDF. (requires
    /// feature `classic-mceliece`)
    X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256,

    /// Use X25519 for the outer key exchange, no PQ-KEM, ML-DSA 65
    /// for signature-based initiator autentication, Chacha20Poly1305
    /// as payload AEAD and SHA-256 for HKDF. This is the only
    /// ciphersuite supported in Query mode, where no client
    /// authentication takes place.
    X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256,
    /// Use X25519 for the outer key exchange, ML-KEM 768 for PQ-KEM,
    /// ML-DSA 65 for signature-based initiator autentication,
    /// Chacha20Poly1305 as payload AEAD and SHA-256 for HKDF.
    X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256,
    /// Use X25519 for the outer key exchange, Classic McEliece for
    /// PQ-KEM, ML-DSA 65 for signature-based initiator autentication,
    /// Chacha20Poly1305 as payload AEAD and SHA-256 for
    /// HKDF. (requires feature `classic-mceliece`)
    X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256,
    /// Use X25519 for the outer key exchange, no PQ-KEM, ML-DSA 65
    /// for signature-based initiator autentication, AesGcm128 as
    /// payload AEAD and SHA-256 for HKDF.
    X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256,
    /// Use X25519 for the outer key exchange, ML-KEM 768 for PQ-KEM,
    /// ML-DSA 65 for signature-based initiator autentication,
    /// AesGcm128 as payload AEAD and SHA-256 for HKDF.
    X25519_MLKEM768_MLDSA65_AESGCM128_HKDFSHA256,
    /// Use X25519 for the outer key exchange, Classic McEliece for
    /// PQ-KEM, ML-DSA 65 for signature-based initiator autentication,
    /// AesGcm128 as payload AEAD and SHA-256 for HKDF. (requires
    /// feature `classic-mceliece`)
    X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256,
}

impl CiphersuiteName {
    /// Returns the default ciphersuite used for query mode.
    pub(crate) fn query_ciphersuite() -> Self {
        Self::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256
    }

    /// Coerce the responder ciphersuite into a compatible working
    /// ciphersuite for a given handshake.
    ///
    /// A compatible ciphersuite exists, if the PQ KEMs of both ciphersuites
    /// agree, or if the initiator side does not specify a PQ KEM. If the
    /// initiator side specifies a PQ KEM and it does not match the responder
    /// side, there is no compatible ciphersuite.
    ///
    /// If a compatible ciphersuite exists, it will match the initiator
    /// ciphersuite, i.e. a PQ responder ciphersuite can be coerced into a
    /// compatible non-PQ ciphersuite.
    pub(crate) fn coerce_compatible(
        &self,
        responder_ciphersuite: &ResponderCiphersuite,
    ) -> Result<CiphersuiteName, HandshakeError> {
        match (self, responder_ciphersuite.name()) {
            (CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256, _) => {
                Ok(CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256)
            }
            (CiphersuiteName::X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256, _) => {
                Ok(CiphersuiteName::X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256)
            }
            (CiphersuiteName::X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256, _) => {
                Ok(CiphersuiteName::X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256)
            }
            (CiphersuiteName::X25519_NONE_X25519_AESGCM128_HKDFSHA256, _) => {
                Ok(CiphersuiteName::X25519_NONE_X25519_AESGCM128_HKDFSHA256)
            }
            (CiphersuiteName::X25519_NONE_ED25519_AESGCM128_HKDFSHA256, _) => {
                Ok(CiphersuiteName::X25519_NONE_ED25519_AESGCM128_HKDFSHA256)
            }
            (CiphersuiteName::X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256, _) => {
                Ok(CiphersuiteName::X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256)
            }

            (x, y) if *x == y => Ok(y).into(),
            _ => Err(HandshakeError::UnsupportedCiphersuite),
        }
    }

    pub(crate) fn deserialize_encapsulation(
        &self,
        bytes: &[u8],
    ) -> Result<Option<PQCiphertext>, HandshakeError> {
        match self {
            CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256 => Ok(None),

            CiphersuiteName::X25519_MLKEM768_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256 => {
                let enc = Option::<PQCiphertext>::tls_deserialize(&mut Cursor::new(bytes))
                    .map_err(HandshakeError::Deserialize)?;

                Ok(enc)
            }

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256 => {
                use std::io::Cursor;

                let enc = Option::<PQCiphertext>::tls_deserialize(&mut Cursor::new(bytes))
                    .map_err(HandshakeError::Deserialize)?;
                Ok(enc)
            }

            #[cfg(not(feature = "classic-mceliece"))]
            CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256 => {
                Err(HandshakeError::UnsupportedCiphersuite)
            }
        }
    }
}

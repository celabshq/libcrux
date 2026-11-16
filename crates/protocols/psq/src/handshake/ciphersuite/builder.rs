//! # Ciphersuite Builder
//!
//! This module provides a builder pattern for PSQ ciphersuites.

use libcrux_kem::{MlKem768PrivateKey, MlKem768PublicKey};

#[cfg(feature = "classic-mceliece")]
use crate::classic_mceliece::{PublicKey, SecretKey};
use crate::handshake::{
    builders::BuilderError,
    ciphersuite::{
        initiator::{InitiatorCiphersuite, PqKemPublicKey},
        responder::{PqKemKeyPair, ResponderCiphersuite},
        CiphersuiteName,
    },
    dhkem::{DHKeyPair, DHPublicKey},
};
use crate::{aead::AeadType, handshake::ciphersuite::initiator::SigningKeyPair};

/// A builder for PSQ handshake ciphersuites.
pub struct CiphersuiteBuilder<'a> {
    name: CiphersuiteName,
    longterm_x25519_keys: Option<&'a DHKeyPair>,
    longterm_mlkem_encapsulation_key: Option<&'a MlKem768PublicKey>,
    longterm_mlkem_decapsulation_key: Option<&'a MlKem768PrivateKey>,
    longterm_signing_keys: Option<SigningKeyPair<'a>>,
    peer_longterm_x25519_pk: Option<&'a DHPublicKey>,
    peer_longterm_mlkem_pk: Option<&'a MlKem768PublicKey>,
    #[cfg(feature = "classic-mceliece")]
    longterm_cmc_encapsulation_key: Option<&'a PublicKey>,
    #[cfg(feature = "classic-mceliece")]
    longterm_cmc_decapsulation_key: Option<&'a SecretKey>,
    #[cfg(feature = "classic-mceliece")]
    peer_longterm_cmc_pk: Option<&'a PublicKey>,
}

impl<'a> CiphersuiteBuilder<'a> {
    /// Start building a new ciphersuite.
    pub fn new(name: CiphersuiteName) -> Self {
        Self {
            name,
            longterm_x25519_keys: None,
            longterm_mlkem_encapsulation_key: None,
            longterm_mlkem_decapsulation_key: None,
            longterm_signing_keys: None,
            peer_longterm_x25519_pk: None,
            peer_longterm_mlkem_pk: None,
            #[cfg(feature = "classic-mceliece")]
            longterm_cmc_encapsulation_key: None,
            #[cfg(feature = "classic-mceliece")]
            longterm_cmc_decapsulation_key: None,
            #[cfg(feature = "classic-mceliece")]
            peer_longterm_cmc_pk: None,
        }
    }

    /// Provide a principal's long-term ECDH key pair.
    pub fn longterm_x25519_keys(mut self, keypair: &'a DHKeyPair) -> Self {
        self.longterm_x25519_keys = Some(keypair);
        self
    }

    /// Provide a principal's long-term ML-KEM encapsulation key.
    pub fn longterm_mlkem_encapsulation_key(
        mut self,
        encapsulation_key: &'a MlKem768PublicKey,
    ) -> Self {
        self.longterm_mlkem_encapsulation_key = Some(encapsulation_key);
        self
    }

    /// Provide a principal's long-term ML-KEM decapsulation key.
    pub fn longterm_mlkem_decapsulation_key(
        mut self,
        decapsulation_key: &'a MlKem768PrivateKey,
    ) -> Self {
        self.longterm_mlkem_decapsulation_key = Some(decapsulation_key);
        self
    }

    /// Provide a principal's long-term signing keys.
    pub fn longterm_signing_keys(mut self, signing_keys: SigningKeyPair<'a>) -> Self {
        self.longterm_signing_keys = Some(signing_keys);
        self
    }

    /// Provide a peer's long-term ECDH public key.
    pub fn peer_longterm_x25519_pk(mut self, x25519_pk: &'a DHPublicKey) -> Self {
        self.peer_longterm_x25519_pk = Some(x25519_pk);
        self
    }

    /// Provide a peer's long-term ML-KEM encapsulation key.
    pub fn peer_longterm_mlkem_pk(mut self, encapsulation_key: &'a MlKem768PublicKey) -> Self {
        self.peer_longterm_mlkem_pk = Some(encapsulation_key);
        self
    }

    #[cfg(feature = "classic-mceliece")]
    /// Provide a principal's long-term Classic McEliece encapsulation key.
    pub fn longterm_cmc_encapsulation_key(mut self, encapsulation_key: &'a PublicKey) -> Self {
        self.longterm_cmc_encapsulation_key = Some(encapsulation_key);
        self
    }

    #[cfg(feature = "classic-mceliece")]
    /// Provide a principal's long-term Classic McEliece decapsulation key.
    pub fn longterm_cmc_decapsulation_key(mut self, decapsulation_key: &'a SecretKey) -> Self {
        self.longterm_cmc_decapsulation_key = Some(decapsulation_key);
        self
    }

    #[cfg(feature = "classic-mceliece")]
    /// Provide a peers's long-term Classic McEliece encapsulation key.
    pub fn peer_longterm_cmc_pk(mut self, encapsulation_key: &'a PublicKey) -> Self {
        self.peer_longterm_cmc_pk = Some(encapsulation_key);
        self
    }

    /// Finish building an [`InitiatorCiphersuite`].
    pub fn build_initiator_ciphersuite(self) -> Result<InitiatorCiphersuite<'a>, BuilderError> {
        let Some(kex) = self.peer_longterm_x25519_pk else {
            return Err(BuilderError::CiphersuiteBuilderState);
        };

        let pq = match self.name {
            CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256 => PqKemPublicKey::None,

            CiphersuiteName::X25519_MLKEM768_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256 => {
                let Some(pq) = self.peer_longterm_mlkem_pk else {
                    return Err(BuilderError::CiphersuiteBuilderState);
                };
                pq.into()
            }

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256 => {
                let Some(pq) = self.peer_longterm_cmc_pk else {
                    return Err(BuilderError::CiphersuiteBuilderState);
                };
                pq.into()
            }

            #[cfg(not(feature = "classic-mceliece"))]
            CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256 => {
                return Err(BuilderError::UnsupportedCiphersuite);
            }
        };

        let auth = match self.name {
            CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_AESGCM128_HKDFSHA256 => {
                let Some(auth) = self.longterm_x25519_keys else {
                    return Err(BuilderError::CiphersuiteBuilderState);
                };
                auth.into()
            }

            CiphersuiteName::X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_AESGCM128_HKDFSHA256 => {
                let Some(sig_keys) = self.longterm_signing_keys else {
                    return Err(BuilderError::CiphersuiteBuilderState);
                };
                if !matches!(sig_keys, SigningKeyPair::Ed25519(_, _)) {
                    return Err(BuilderError::CiphersuiteBuilderState);
                }
                sig_keys.into()
            }

            CiphersuiteName::X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_AESGCM128_HKDFSHA256 => {
                let Some(sig_keys) = self.longterm_signing_keys else {
                    return Err(BuilderError::CiphersuiteBuilderState);
                };
                if !matches!(sig_keys, SigningKeyPair::MlDsa65(_, _)) {
                    return Err(BuilderError::CiphersuiteBuilderState);
                }
                sig_keys.into()
            }

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256 => {
                let Some(auth) = self.longterm_x25519_keys else {
                    return Err(BuilderError::CiphersuiteBuilderState);
                };
                auth.into()
            }

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256 => {
                let Some(sig_keys) = self.longterm_signing_keys else {
                    return Err(BuilderError::CiphersuiteBuilderState);
                };
                if !matches!(sig_keys, SigningKeyPair::Ed25519(_, _)) {
                    return Err(BuilderError::CiphersuiteBuilderState);
                }
                sig_keys.into()
            }

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256 => {
                let Some(sig_keys) = self.longterm_signing_keys else {
                    return Err(BuilderError::CiphersuiteBuilderState);
                };
                if !matches!(sig_keys, SigningKeyPair::MlDsa65(_, _)) {
                    return Err(BuilderError::CiphersuiteBuilderState);
                }
                sig_keys.into()
            }

            #[cfg(not(feature = "classic-mceliece"))]
            CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256 => {
                return Err(BuilderError::UnsupportedCiphersuite);
            }
        };

        let aead_type = match self.name {
            CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256 => {
                AeadType::ChaCha20Poly1305
            }

            CiphersuiteName::X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_NONE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_AESGCM128_HKDFSHA256 => AeadType::AesGcm128,

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256 => {
                AeadType::ChaCha20Poly1305
            }

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256 => {
                AeadType::AesGcm128
            }

            #[cfg(not(feature = "classic-mceliece"))]
            CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256 => {
                return Err(BuilderError::UnsupportedCiphersuite);
            }
        };

        Ok(InitiatorCiphersuite {
            name: self.name,
            kex,
            pq,
            auth,
            aead_type,
        })
    }

    /// Finish building an [`InitiatorCiphersuite`].
    pub fn build_responder_ciphersuite(self) -> Result<ResponderCiphersuite<'a>, BuilderError> {
        let Some(kex) = self.longterm_x25519_keys else {
            return Err(BuilderError::CiphersuiteBuilderState);
        };

        let pq = match self.name {
            CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256 => PqKemKeyPair::None,

            CiphersuiteName::X25519_MLKEM768_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256 => {
                let Some(pqpk) = self.longterm_mlkem_encapsulation_key else {
                    return Err(BuilderError::CiphersuiteBuilderState);
                };
                let Some(pqsk) = self.longterm_mlkem_decapsulation_key else {
                    return Err(BuilderError::CiphersuiteBuilderState);
                };
                (pqsk, pqpk).into()
            }

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256 => {
                let Some(pqpk) = self.longterm_cmc_encapsulation_key else {
                    return Err(BuilderError::CiphersuiteBuilderState);
                };
                let Some(pqsk) = self.longterm_cmc_decapsulation_key else {
                    return Err(BuilderError::CiphersuiteBuilderState);
                };
                (pqsk, pqpk).into()
            }

            #[cfg(not(feature = "classic-mceliece"))]
            CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256 => {
                return Err(BuilderError::UnsupportedCiphersuite);
            }
        };

        let aead_type = match self.name {
            CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256 => {
                AeadType::ChaCha20Poly1305
            }

            CiphersuiteName::X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_NONE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_AESGCM128_HKDFSHA256 => AeadType::AesGcm128,

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256 => {
                AeadType::ChaCha20Poly1305
            }

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256 => {
                AeadType::AesGcm128
            }

            #[cfg(not(feature = "classic-mceliece"))]
            CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256 => {
                return Err(BuilderError::UnsupportedCiphersuite);
            }
        };

        Ok(ResponderCiphersuite {
            name: self.name,
            kex,
            pq,
            aead_type,
        })
    }
}

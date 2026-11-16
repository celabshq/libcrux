use libcrux_kem::{MlKem768PrivateKey, MlKem768PublicKey};

#[cfg(feature = "classic-mceliece")]
use crate::classic_mceliece::{PublicKey, SecretKey};
use crate::{
    aead::AeadType,
    handshake::{
        ciphersuite::{
            traits::CiphersuiteBase,
            types::{PQCiphertext, PQEncapsulationKey, PQSharedSecret},
            CiphersuiteName,
        },
        dhkem::DHKeyPair,
        HandshakeError,
    },
};

pub(crate) enum PqKemKeyPair<'a> {
    None,
    MlKem(&'a MlKem768PrivateKey, &'a MlKem768PublicKey),
    #[cfg(feature = "classic-mceliece")]
    Cmc(&'a SecretKey, &'a PublicKey),
    #[cfg(not(feature = "classic-mceliece"))]
    #[allow(unused)]
    Cmc(
        core::marker::PhantomData<&'a [u8]>,
        core::marker::PhantomData<&'a [u8]>,
    ),
}

impl<'a> PqKemKeyPair<'a> {
    pub(crate) fn encapsulation_key(&self) -> Option<PQEncapsulationKey<'a>> {
        match self {
            PqKemKeyPair::None => None,
            PqKemKeyPair::MlKem(_, ml_kem_public_key) => {
                Some(PQEncapsulationKey::MlKem(ml_kem_public_key))
            }
            #[cfg(feature = "classic-mceliece")]
            PqKemKeyPair::Cmc(_, public_key) => Some(PQEncapsulationKey::CMC(public_key)),
            #[cfg(not(feature = "classic-mceliece"))]
            PqKemKeyPair::Cmc(_, _) => {
                // We can never reach this because the ciphersuite can only be constructed with the feature turned on.
                unreachable!("unsupported ciphersuite")
            }
        }
    }
}

impl<'a> From<(&'a MlKem768PrivateKey, &'a MlKem768PublicKey)> for PqKemKeyPair<'a> {
    fn from(value: (&'a MlKem768PrivateKey, &'a MlKem768PublicKey)) -> Self {
        PqKemKeyPair::MlKem(value.0, value.1)
    }
}

#[cfg(feature = "classic-mceliece")]
impl<'a> From<(&'a SecretKey, &'a PublicKey)> for PqKemKeyPair<'a> {
    fn from(value: (&'a SecretKey, &'a PublicKey)) -> Self {
        PqKemKeyPair::Cmc(value.0, value.1)
    }
}

pub struct ResponderCiphersuite<'a> {
    pub(crate) name: CiphersuiteName,
    pub(crate) kex: &'a DHKeyPair,
    pub(crate) pq: PqKemKeyPair<'a>,
    pub(crate) aead_type: AeadType,
}

impl<'a> CiphersuiteBase for ResponderCiphersuite<'a> {
    type Ciphertext = PQCiphertext;
    type EncapsulationKeyRef = PQEncapsulationKey<'a>;
    type SharedSecret = PQSharedSecret;

    fn name(&self) -> CiphersuiteName {
        self.name
    }
}

impl<'a> ResponderCiphersuite<'a> {
    pub(crate) fn aead_type(&self) -> AeadType {
        self.aead_type
    }

    pub(crate) fn own_pq_encapsulation_key(
        &self,
    ) -> Option<<Self as CiphersuiteBase>::EncapsulationKeyRef> {
        match self.pq {
            PqKemKeyPair::None => None,
            PqKemKeyPair::MlKem(_, ml_kem_public_key) => {
                Some(PQEncapsulationKey::MlKem(ml_kem_public_key))
            }
            #[cfg(feature = "classic-mceliece")]
            PqKemKeyPair::Cmc(_, public_key) => Some(PQEncapsulationKey::CMC(public_key)),
            #[cfg(not(feature = "classic-mceliece"))]
            PqKemKeyPair::Cmc(_, _) => {
                // We can never reach this because the ciphersuite can only be constructed with the feature turned on.
                unreachable!("unsupported ciphersuite")
            }
        }
    }

    pub(crate) fn pq_decapsulate(
        &self,
        ciphertext: &<Self as CiphersuiteBase>::Ciphertext,
    ) -> Result<<Self as CiphersuiteBase>::SharedSecret, HandshakeError> {
        match self.pq {
            PqKemKeyPair::None => Err(HandshakeError::UnsupportedCiphersuite),
            PqKemKeyPair::MlKem(ml_kem_private_key, _) => {
                let PQCiphertext::MlKem(inner_ctxt) = ciphertext else {
                    return Err(HandshakeError::CryptoError);
                };
                let shared_secret =
                    libcrux_ml_kem::mlkem768::decapsulate(ml_kem_private_key, inner_ctxt);

                Ok(PQSharedSecret::MlKem(shared_secret))
            }
            #[cfg(feature = "classic-mceliece")]
            PqKemKeyPair::Cmc(secret_key, _) => {
                use crate::classic_mceliece::ClassicMcEliece;
                use libcrux_traits::kem::KEM;

                let PQCiphertext::CMC(inner_ctxt) = ciphertext else {
                    return Err(HandshakeError::CryptoError);
                };
                let shared_secret =
                    <ClassicMcEliece as KEM>::decapsulate(&secret_key.0, inner_ctxt)
                        .map_err(|_| HandshakeError::CryptoError)?;
                Ok(PQSharedSecret::CMC(shared_secret))
            }
            #[cfg(not(feature = "classic-mceliece"))]
            PqKemKeyPair::Cmc(_, _) => {
                // We can never reach this because the ciphersuite can only be constructed with the feature turned on.
                unreachable!("unsupported ciphersuite")
            }
        }
    }
}

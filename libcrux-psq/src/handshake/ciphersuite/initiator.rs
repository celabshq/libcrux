use libcrux_ed25519::{SigningKey as Ed25519SigningKey, VerificationKey as Ed25519VerificationKey};
use libcrux_ml_dsa::ml_dsa_65::{MLDSA65KeyPair, MLDSA65SigningKey, MLDSA65VerificationKey};
use rand::CryptoRng;
use tls_codec::SerializeBytes;

#[cfg(feature = "classic-mceliece")]
use crate::classic_mceliece::PublicKey;
use crate::{
    aead::AeadType,
    handshake::{
        ciphersuite::{
            traits::CiphersuiteBase,
            types::{PQCiphertext, PQEncapsulationKey, PQSharedSecret},
            CiphersuiteName, PSQ_MLDSA_CONTEXT,
        },
        dhkem::{DHKeyPair, DHPublicKey},
        transcript::Transcript,
        types::{Signature, SignatureVerificationKey},
        HandshakeError,
    },
};

#[derive(Clone, Copy)]
pub(crate) enum PqKemPublicKey<'a> {
    None,
    MlKem(&'a libcrux_ml_kem::mlkem768::MlKem768PublicKey),
    #[cfg(feature = "classic-mceliece")]
    Cmc(&'a PublicKey),
    #[cfg(not(feature = "classic-mceliece"))]
    #[allow(unused)]
    Cmc(std::marker::PhantomData<&'a [u8]>),
}

impl<'a> From<&'a libcrux_ml_kem::mlkem768::MlKem768PublicKey> for PqKemPublicKey<'a> {
    fn from(value: &'a libcrux_ml_kem::mlkem768::MlKem768PublicKey) -> Self {
        PqKemPublicKey::MlKem(value)
    }
}

#[cfg(feature = "classic-mceliece")]
impl<'a> From<&'a PublicKey> for PqKemPublicKey<'a> {
    fn from(value: &'a PublicKey) -> Self {
        PqKemPublicKey::Cmc(value)
    }
}

#[derive(Clone, Copy)]
/// A pair of long-term signing and verification keys.
pub enum SigningKeyPair<'a> {
    /// A key pair for the Ed25519 signature scheme.
    Ed25519(&'a Ed25519SigningKey, &'a Ed25519VerificationKey),
    /// A key pair for the ML-DSA 65 signature scheme.
    MlDsa65(&'a MLDSA65SigningKey, &'a MLDSA65VerificationKey),
}

impl<'a> From<&'a MLDSA65KeyPair> for SigningKeyPair<'a> {
    fn from(value: &'a MLDSA65KeyPair) -> Self {
        Self::MlDsa65(&value.signing_key, &value.verification_key)
    }
}

impl<'a> From<&'a (Ed25519SigningKey, Ed25519VerificationKey)> for SigningKeyPair<'a> {
    fn from(value: &'a (Ed25519SigningKey, Ed25519VerificationKey)) -> Self {
        Self::Ed25519(&value.0, &value.1)
    }
}

impl<'a> SigningKeyPair<'a> {
    // ProVerif: EUF-CMA signing. `extern__sign(sk, msg)` is one-way; the matching
    // `extern__sig_verify` reduc (psq_crypto.pvl) accepts it only under
    // `vk_of(sk)`. We project the inner signing scalar from the SigningKeyPair so
    // the verification key the peer recovers (`SignatureVerificationKey` carries
    // `vk = vk_of(sk)`, set up by the role) binds to the same `sk`.
    #[cfg_attr(
        feature = "hax-pv",
        hax_lib::proverif::replace_body(
            "let libcrux_psq__handshake__ciphersuite__initiator__SigningKeyPair__Ed25519(sk, vk) = self in \
               libcrux_psq__handshake__ciphersuite__types__Signature__Ed25519(extern__sign(sk, tx)) \
             else let libcrux_psq__handshake__ciphersuite__initiator__SigningKeyPair__MlDsa65(sk, vk) = self in \
               libcrux_psq__handshake__ciphersuite__types__Signature__MlDsa65(extern__sign(sk, tx)) \
             else bitstring_err()"
        )
    )]
    pub(crate) fn sign(
        &self,
        rng: &mut impl CryptoRng,
        tx: &Transcript,
    ) -> Result<Signature, HandshakeError> {
        let payload = tx.tls_serialize().map_err(HandshakeError::Serialize)?;
        match self {
            SigningKeyPair::Ed25519(signing_key, _) => {
                let sig = libcrux_ed25519::sign(&payload, signing_key.as_ref())?;
                Ok(Signature::Ed25519(sig))
            }
            SigningKeyPair::MlDsa65(mldsasigning_key, _) => {
                let mut randomness = [0u8; libcrux_ml_dsa::SIGNING_RANDOMNESS_SIZE];
                rng.fill_bytes(&mut randomness);
                let sig = libcrux_ml_dsa::ml_dsa_65::sign(
                    mldsasigning_key,
                    &payload,
                    PSQ_MLDSA_CONTEXT,
                    randomness,
                )?;
                Ok(Signature::MlDsa65(Box::new(sig)))
            }
        }
    }
}

impl From<SigningKeyPair<'_>> for SignatureVerificationKey {
    fn from(value: SigningKeyPair<'_>) -> Self {
        match value {
            SigningKeyPair::Ed25519(_, verification_key) => {
                SignatureVerificationKey::Ed25519(*verification_key)
            }
            SigningKeyPair::MlDsa65(_, mldsaverification_key) => {
                SignatureVerificationKey::MlDsa65(Box::new(mldsaverification_key.clone()))
            }
        }
    }
}

impl<'a> From<PqKemPublicKey<'a>> for Option<PQEncapsulationKey<'a>> {
    fn from(value: PqKemPublicKey<'a>) -> Self {
        match value {
            PqKemPublicKey::None => None,
            PqKemPublicKey::MlKem(ml_kem_public_key) => {
                Some(PQEncapsulationKey::MlKem(ml_kem_public_key))
            }
            #[cfg(feature = "classic-mceliece")]
            PqKemPublicKey::Cmc(public_key) => Some(PQEncapsulationKey::CMC(public_key)),
            #[cfg(not(feature = "classic-mceliece"))]
            PqKemPublicKey::Cmc(_) => {
                // We can never reach this because the ciphersuite can only be constructed with the feature turned on.
                unreachable!("unsupported ciphersuite")
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Auth<'a> {
    DH(&'a DHKeyPair),
    Sig(SigningKeyPair<'a>),
}

impl<'a> From<&'a DHKeyPair> for Auth<'a> {
    fn from(value: &'a DHKeyPair) -> Self {
        Auth::DH(value)
    }
}

impl<'a> From<SigningKeyPair<'a>> for Auth<'a> {
    fn from(value: SigningKeyPair<'a>) -> Self {
        Auth::Sig(value)
    }
}

pub struct InitiatorCiphersuite<'a> {
    pub(crate) name: CiphersuiteName,
    pub(crate) kex: &'a DHPublicKey,
    pub(crate) pq: PqKemPublicKey<'a>,
    pub(crate) auth: Auth<'a>,
    pub(crate) aead_type: AeadType,
}

impl<'a> CiphersuiteBase for InitiatorCiphersuite<'a> {
    type Ciphertext = PQCiphertext;
    type EncapsulationKeyRef = PQEncapsulationKey<'a>;
    type SharedSecret = PQSharedSecret;

    fn name(&self) -> CiphersuiteName {
        self.name
    }
}

pub(crate) type PQOptionPair<A, B> = (Option<A>, Option<B>);

impl<'a> InitiatorCiphersuite<'a> {
    pub(crate) fn aead_type(&self) -> AeadType {
        self.aead_type
    }

    pub(crate) fn peer_pq_encapsulation_key(
        &self,
    ) -> Option<<Self as CiphersuiteBase>::EncapsulationKeyRef> {
        match self.pq {
            PqKemPublicKey::None => None,
            PqKemPublicKey::MlKem(ml_kem_public_key) => {
                Some(PQEncapsulationKey::MlKem(ml_kem_public_key))
            }
            #[cfg(feature = "classic-mceliece")]
            PqKemPublicKey::Cmc(public_key) => Some(PQEncapsulationKey::CMC(public_key)),
            #[cfg(not(feature = "classic-mceliece"))]
            PqKemPublicKey::Cmc(_) => {
                // We can never reach this because the ciphersuite can only be constructed with the feature turned on.
                unreachable!("unsupported ciphersuite")
            }
        }
    }

    pub(crate) fn peer_ecdh_encapsulation_key(&self) -> &DHPublicKey {
        self.kex
    }

    pub(crate) fn is_pq(&self) -> bool {
        !matches!(self.pq, PqKemPublicKey::None)
    }

    pub(crate) fn pq_encapsulate(
        &self,
        rng: &mut impl CryptoRng,
    ) -> Result<
        PQOptionPair<
            <Self as CiphersuiteBase>::Ciphertext,
            <Self as CiphersuiteBase>::SharedSecret,
        >,
        HandshakeError,
    > {
        match self.pq {
            PqKemPublicKey::None => Ok((None, None)),
            PqKemPublicKey::MlKem(ml_kem_public_key) => {
                let mut rand = [0u8; libcrux_ml_kem::ENCAPS_SEED_SIZE];
                rng.fill_bytes(&mut rand);
                let (ct, ss) = libcrux_ml_kem::mlkem768::encapsulate(ml_kem_public_key, rand);

                Ok((
                    Some(PQCiphertext::MlKem(Box::new(ct))),
                    Some(PQSharedSecret::MlKem(ss)),
                ))
            }
            #[cfg(feature = "classic-mceliece")]
            PqKemPublicKey::Cmc(public_key) => {
                use crate::classic_mceliece::ClassicMcEliece;
                use libcrux_traits::kem::KEM;

                let (ss, ct) = <ClassicMcEliece as KEM>::encapsulate(public_key, rng)
                    .map_err(|_| HandshakeError::CryptoError)?;

                Ok((
                    Some(PQCiphertext::CMC(Box::new(ct))),
                    Some(PQSharedSecret::CMC(ss)),
                ))
            }
            #[cfg(not(feature = "classic-mceliece"))]
            PqKemPublicKey::Cmc(_) => {
                // We can never reach this because the ciphersuite can only be constructed with the feature turned on.
                unreachable!("unsupported ciphersuite")
            }
        }
    }
}

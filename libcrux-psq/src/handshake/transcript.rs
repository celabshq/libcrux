use rand::CryptoRng;
use tls_codec::{
    Serialize, SerializeBytes, TlsDeserialize, TlsSerialize, TlsSerializeBytes, TlsSize,
};

pub const TX0_DOMAIN_SEP: u8 = 0;
pub const TX1_DOMAIN_SEP: u8 = 1;
pub const TX2_DOMAIN_SEP: u8 = 2;

use super::dhkem::DHPublicKey;
use crate::handshake::{
    ciphersuite::{initiator::Auth, types::PQEncapsulationKey},
    types::{Authenticator, Signature},
    AuthMessage, HandshakeError as Error,
};
use libcrux_sha2::{Digest, SHA256_LENGTH};

/// The initial transcript hash.
#[derive(Debug, Default, Clone, Copy, TlsSerializeBytes, TlsSerialize, TlsDeserialize, TlsSize)]
pub struct Transcript([u8; SHA256_LENGTH]);

impl Transcript {
    fn new(initial_input: impl Serialize) -> Result<Self, Error> {
        Self::add_hash::<TX0_DOMAIN_SEP>(None, initial_input)
    }

    // ProVerif: running transcript hash. `extern__hash` is one-way and
    // injective in the term algebra; the chained previous transcript plus the
    // (distinctly-typed) step input bind each step distinctly — the const
    // domain separator can't be interpolated into the replace_body text and is
    // redundant given the chaining. Avoids the stateful SHA-256 hasher
    // (new/update/finish) the &mut analysis can't model.
    #[cfg_attr(
        feature = "hax-pv",
        hax_lib::proverif::replace_body("extern__hash(old_transcript, input)")
    )]
    pub(crate) fn add_hash<const DOMAIN_SEPARATOR: u8>(
        old_transcript: Option<&Transcript>,
        input: impl Serialize,
    ) -> Result<Transcript, Error> {
        let mut hasher = libcrux_sha2::Sha256::new();
        hasher.update(&[DOMAIN_SEPARATOR]);
        hasher.update(
            <Option<&Transcript> as SerializeBytes>::tls_serialize(&old_transcript)
                .map_err(Error::Serialize)?
                .as_slice(),
        );
        hasher.update(
            input
                .tls_serialize_detached()
                .map_err(Error::Serialize)?
                .as_slice(),
        );

        let mut digest = [0u8; 32];
        hasher.finish(&mut digest);
        Ok(Transcript(digest))
    }
}

impl AsRef<[u8]> for Transcript {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

// tx0 = hash(0 | ctx | g^s | g^x)
pub(crate) fn tx0(
    context: &[u8],
    responder_pk: &DHPublicKey,
    initiator_pk: &DHPublicKey,
) -> Result<Transcript, Error> {
    #[derive(TlsSerialize, TlsSize)]
    struct Transcript0Inputs<'a> {
        context: &'a [u8],
        responder_pk: &'a DHPublicKey,
        initiator_pk: &'a DHPublicKey,
    }

    Transcript::new(&Transcript0Inputs {
        context,
        responder_pk,
        initiator_pk,
    })
}

// tx1 = hash(1 | tx0 | g^c | [pkS] | [encap(pkS, SS)])
// or
// tx1 = hash(1 | tx0 | vk | [pkS] | [encap(pkS, SS)])
pub(crate) fn tx1(
    tx0: &Transcript,
    initiator_authenticator: &Authenticator,
    responder_pq_pk: Option<PQEncapsulationKey>,
    pq_encaps: &[u8],
) -> Result<Transcript, Error> {
    #[derive(TlsSerialize, TlsSize)]
    struct Transcript1InputsDh<'a> {
        initiator_authenticator: &'a Authenticator,
        responder_pq_pk: Option<PQEncapsulationKey<'a>>,
        pq_encaps: &'a [u8],
    }

    Transcript::add_hash::<TX1_DOMAIN_SEP>(
        Some(tx0),
        Transcript1InputsDh {
            initiator_authenticator,
            pq_encaps,
            responder_pq_pk,
        },
    )
}

pub(crate) fn sign_tx1<'a>(
    tx0: &Transcript,
    authenticator: Auth<'a>,
    responder_pq_pk: Option<PQEncapsulationKey>,
    pq_encaps: &[u8],
    rng: &mut impl CryptoRng,
) -> Result<(Option<Signature>, Transcript), Error> {
    let tx1 = tx1(
        tx0,
        &Authenticator::from(authenticator),
        responder_pq_pk,
        pq_encaps,
    )?;

    match authenticator {
        Auth::DH(_) => Ok((None, tx1)),
        Auth::Sig(sig_key_pair) => Ok((Some(sig_key_pair.sign(rng, &tx1)?), tx1)),
    }
}

pub(crate) fn verify_tx1(
    tx0: &Transcript,
    initiator_authenticator: &AuthMessage,
    responder_pq_pk: Option<PQEncapsulationKey>,
    pq_encaps: &[u8],
) -> Result<(Authenticator, Transcript), Error> {
    let authenticator = Authenticator::from(initiator_authenticator);
    let tx1 = tx1(tx0, &authenticator, responder_pq_pk, pq_encaps)?;
    match initiator_authenticator {
        AuthMessage::Dh(_dhpublic_key) => Ok((authenticator, tx1)),
        AuthMessage::Sig { vk, signature } => {
            // verify signature
            vk.verify(&tx1, signature)?;

            Ok((authenticator, tx1))
        }
    }
}

// Registration Mode: tx2 = hash(2 | tx1 | g^y)
// Query Mode:        tx2 = hash(2 | tx0 | g^y)
pub(crate) fn tx2(
    prev_tx: &Transcript,
    responder_ephemeral_pk: &DHPublicKey,
) -> Result<Transcript, Error> {
    Transcript::add_hash::<TX2_DOMAIN_SEP>(Some(prev_tx), responder_ephemeral_pk)
}

use libcrux_ml_dsa::ml_dsa_65::MLDSA65KeyPair;
use libcrux_ml_kem::mlkem768::MlKem768KeyPair;
#[cfg(feature = "classic-mceliece")]
use libcrux_psq::classic_mceliece::KeyPair;
use libcrux_psq::{
    handshake::{builders::*, ciphersuites::*, types::*, HandshakeError},
    session::{Session, SessionBinding, SessionError},
    Channel, IntoSession,
};
use rand::Rng;

struct CommonSetup {
    pub responder_mlkem_keys: MlKem768KeyPair,
    #[cfg(feature = "classic-mceliece")]
    pub responder_cmc_keys: KeyPair,
    pub responder_x25519_keys: DHKeyPair,
    pub initiator_x25519_keys: DHKeyPair,
    pub initiator_mldsa_keys: MLDSA65KeyPair,
    pub initiator_ed25519_keys: (
        libcrux_ed25519::SigningKey,
        libcrux_ed25519::VerificationKey,
    ),
}

use std::sync::LazyLock;

static SETUP: LazyLock<CommonSetup> = LazyLock::new(|| CommonSetup::new());

#[derive(Debug, PartialEq)]
pub enum TestError {
    Handshake(HandshakeError),
    Session(SessionError),
    Builder(BuilderError),
}

impl From<HandshakeError> for TestError {
    fn from(value: HandshakeError) -> Self {
        TestError::Handshake(value)
    }
}

impl From<BuilderError> for TestError {
    fn from(value: BuilderError) -> Self {
        TestError::Builder(value)
    }
}

impl From<SessionError> for TestError {
    fn from(value: SessionError) -> Self {
        TestError::Session(value)
    }
}

impl CommonSetup {
    fn pq_encapsulation_key(&self, name: CiphersuiteName) -> Option<PQEncapsulationKey<'_>> {
        match name {
            CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256 => None,

            CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_AESGCM128_HKDFSHA256 => Some(
                PQEncapsulationKey::MlKem(self.responder_mlkem_keys.public_key()),
            ),

            CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256 => {
                #[cfg(feature = "classic-mceliece")]
                return Some(PQEncapsulationKey::CMC(&self.responder_cmc_keys.pk));
                #[cfg(not(feature = "classic-mceliece"))]
                panic!("unsupported ciphersuite")
            }
        }
    }

    fn initiator_authenticator(&self, name: CiphersuiteName) -> Authenticator {
        match name {
            CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_AESGCM128_HKDFSHA256 => {
                Authenticator::Dh(self.initiator_x25519_keys.pk.clone())
            }

            CiphersuiteName::X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_AESGCM128_HKDFSHA256 => Authenticator::Sig(
                SignatureVerificationKey::Ed25519(self.initiator_ed25519_keys.1),
            ),

            CiphersuiteName::X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_AESGCM128_HKDFSHA256 => {
                Authenticator::Sig(SignatureVerificationKey::MlDsa65(Box::new(
                    self.initiator_mldsa_keys.verification_key.clone(),
                )))
            }

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256 => {
                Authenticator::Dh(self.initiator_x25519_keys.pk.clone())
            }

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256 => {
                Authenticator::Sig(SignatureVerificationKey::Ed25519(
                    self.initiator_ed25519_keys.1,
                ))
            }

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256 => {
                Authenticator::Sig(SignatureVerificationKey::MlDsa65(Box::new(
                    self.initiator_mldsa_keys.verification_key.clone(),
                )))
            }

            #[cfg(not(feature = "classic-mceliece"))]
            CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256 => {
                panic!("unsupported ciphersuite")
            }
        }
    }

    fn initiator_signature_keys(&self, name: CiphersuiteName) -> Option<SigningKeyPair<'_>> {
        match name {
            CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_X25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_X25519_AESGCM128_HKDFSHA256 => None,

            CiphersuiteName::X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_ED25519_AESGCM128_HKDFSHA256 => {
                Some(SigningKeyPair::from(&self.initiator_ed25519_keys))
            }

            CiphersuiteName::X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_MLKEM768_MLDSA65_AESGCM128_HKDFSHA256 => {
                Some(SigningKeyPair::from(&self.initiator_mldsa_keys))
            }

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256 => None,

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256 => {
                Some(SigningKeyPair::from(&self.initiator_ed25519_keys))
            }

            #[cfg(feature = "classic-mceliece")]
            CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256 => {
                Some(SigningKeyPair::from(&self.initiator_mldsa_keys))
            }

            #[cfg(not(feature = "classic-mceliece"))]
            CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256
            | CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256 => {
                panic!("unsupported ciphersuite");
            }
        }
    }

    fn new() -> Self {
        let mut rng = rand::rng();
        let responder_mlkem_keys = libcrux_ml_kem::mlkem768::rand::generate_key_pair(&mut rng);
        #[cfg(feature = "classic-mceliece")]
        let responder_cmc_keys =
            libcrux_psq::classic_mceliece::KeyPair::generate_key_pair(&mut rng);

        let responder_x25519_keys = DHKeyPair::new(&mut rng);
        let initiator_x25519_keys = DHKeyPair::new(&mut rng);

        let mut rand = [0u8; libcrux_ml_dsa::KEY_GENERATION_RANDOMNESS_SIZE];
        rng.fill(&mut rand);
        let initiator_mldsa_keys = libcrux_ml_dsa::ml_dsa_65::generate_key_pair(rand);

        let initiator_ed25519_keys = libcrux_ed25519::generate_key_pair(&mut rng).unwrap();

        CommonSetup {
            responder_mlkem_keys,
            #[cfg(feature = "classic-mceliece")]
            responder_cmc_keys,
            responder_x25519_keys,
            initiator_x25519_keys,
            initiator_mldsa_keys,
            initiator_ed25519_keys,
        }
    }
}

fn registration_no_encoding(
    setup: &CommonSetup,
    initiator_ciphersuite_id: CiphersuiteName,
    responder_ciphersuite_id: CiphersuiteName,
) -> Result<(), TestError> {
    let ctx = b"Test Context";
    let aad_initiator_outer = b"Test Data I Outer";
    let aad_initiator_inner = b"Test Data I Inner";
    let aad_responder = b"Test Data R";

    // Setup initiator
    // We add everything here to construct any ciphersuite.
    #[allow(unused_mut)] // we need it mutable for the CMC case
    let mut initiator_cbuilder = CiphersuiteBuilder::new(initiator_ciphersuite_id)
        .longterm_x25519_keys(&setup.initiator_x25519_keys)
        .peer_longterm_x25519_pk(&setup.responder_x25519_keys.pk)
        .peer_longterm_mlkem_pk(setup.responder_mlkem_keys.public_key());

    if let Some(signature_key_pair) = setup.initiator_signature_keys(initiator_ciphersuite_id) {
        initiator_cbuilder = initiator_cbuilder.longterm_signing_keys(signature_key_pair);
    }

    #[cfg(feature = "classic-mceliece")]
    {
        initiator_cbuilder = initiator_cbuilder.peer_longterm_cmc_pk(&setup.responder_cmc_keys.pk);
    }
    let initiator_ciphersuite = initiator_cbuilder.build_initiator_ciphersuite().unwrap();

    let mut initiator = PrincipalBuilder::new(rand::rng())
        .outer_aad(aad_initiator_outer)
        .inner_aad(aad_initiator_inner)
        .context(ctx)
        .build_registration_initiator(initiator_ciphersuite)
        .unwrap();

    // Setup responder
    #[allow(unused_mut)] // we need it mutable for the CMC case
    let mut responder_cbuilder = CiphersuiteBuilder::new(responder_ciphersuite_id)
        .longterm_x25519_keys(&setup.responder_x25519_keys)
        .longterm_mlkem_encapsulation_key(setup.responder_mlkem_keys.public_key())
        .longterm_mlkem_decapsulation_key(setup.responder_mlkem_keys.private_key());

    #[cfg(feature = "classic-mceliece")]
    {
        responder_cbuilder = responder_cbuilder
            .longterm_cmc_encapsulation_key(&setup.responder_cmc_keys.pk)
            .longterm_cmc_decapsulation_key(&setup.responder_cmc_keys.sk);
    }
    let responder_ciphersuite = responder_cbuilder.build_responder_ciphersuite().unwrap();

    let mut responder = PrincipalBuilder::new(rand::rng())
        .context(ctx)
        .outer_aad(aad_responder)
        .recent_keys_upper_bound(30)
        .build_responder(responder_ciphersuite)
        .unwrap();

    // Send first message
    let registration_payload_initiator = b"Registration_init";
    let sent_intiator_message = initiator
        .write_message_external_encoding(registration_payload_initiator)
        .unwrap();

    // Read first message
    let received_initiator_message = responder
        .read_message_external_encoding(&sent_intiator_message)
        .unwrap();

    assert_eq!(
        registration_payload_initiator.to_vec(),
        received_initiator_message
    );

    // Get the authenticator out here, so we can deserialize the session later.
    let Some(initiator_authenticator) = responder.initiator_authenticator() else {
        panic!("No initiator authenticator found")
    };

    // Respond
    let registration_payload_responder = b"Registration_respond";
    let sent_response = responder
        .write_message_external_encoding(registration_payload_responder)
        .unwrap();

    // Finalize on registration initiator
    let received_response = initiator
        .read_message_external_encoding(&sent_response)
        .unwrap();

    assert_eq!(registration_payload_responder.to_vec(), received_response);

    // Ready for transport mode
    assert!(initiator.is_handshake_finished());
    assert!(responder.is_handshake_finished());

    let i_transport = initiator.into_session().unwrap();
    let r_transport = responder.into_session().unwrap();

    // test serialization, deserialization
    let mut session_storage = vec![0u8; 4096];
    i_transport
        .serialize(
            &mut session_storage,
            SessionBinding {
                initiator_authenticator: &setup.initiator_authenticator(initiator_ciphersuite_id),
                responder_ecdh_pk: &setup.responder_x25519_keys.pk,
                responder_pq_pk: setup.pq_encapsulation_key(initiator_ciphersuite_id),
            },
        )
        .unwrap();
    let mut i_transport = Session::deserialize(
        &session_storage,
        SessionBinding {
            initiator_authenticator: &setup.initiator_authenticator(initiator_ciphersuite_id),
            responder_ecdh_pk: &setup.responder_x25519_keys.pk,
            responder_pq_pk: setup.pq_encapsulation_key(initiator_ciphersuite_id),
        },
    )
    .unwrap();

    r_transport
        .serialize(
            &mut session_storage,
            SessionBinding {
                initiator_authenticator: &initiator_authenticator,
                responder_ecdh_pk: &setup.responder_x25519_keys.pk,
                responder_pq_pk: setup.pq_encapsulation_key(initiator_ciphersuite_id),
            },
        )
        .unwrap();
    let mut r_transport = Session::deserialize(
        &session_storage,
        SessionBinding {
            initiator_authenticator: &initiator_authenticator,
            responder_ecdh_pk: &setup.responder_x25519_keys.pk,
            responder_pq_pk: setup.pq_encapsulation_key(initiator_ciphersuite_id),
        },
    )
    .unwrap();

    let mut channel_i = i_transport.transport_channel().unwrap();
    let mut channel_r = r_transport.transport_channel().unwrap();

    assert_eq!(channel_i.identifier(), channel_r.identifier());

    let app_data_i = b"Derived session hey".as_slice();
    let app_data_r = b"Derived session ho".as_slice();

    let sent_message = channel_i
        .write_message_external_encoding(app_data_i)
        .unwrap();

    let received_message = channel_r
        .read_message_external_encoding(&sent_message)
        .unwrap();

    assert_eq!(app_data_i.to_vec(), received_message);

    let sent_message = channel_r
        .write_message_external_encoding(app_data_r)
        .unwrap();

    let received_message = channel_i
        .read_message_external_encoding(&sent_message)
        .unwrap();
    assert_eq!(app_data_r.to_vec(), received_message);

    Ok(())
}

fn registration(
    setup: &CommonSetup,
    initiator_ciphersuite_id: CiphersuiteName,
    responder_ciphersuite_id: CiphersuiteName,
) -> Result<(), TestError> {
    let ctx = b"Test Context";
    let aad_initiator_outer = b"Test Data I Outer";
    let aad_initiator_inner = b"Test Data I Inner";
    let aad_responder = b"Test Data R";

    let mut msg_channel = vec![0u8; 8192];
    let mut payload_buf_responder = vec![0u8; 4096];
    let mut payload_buf_initiator = vec![0u8; 4096];

    // Setup initiator
    // We add everything here to construct any ciphersuite.
    #[allow(unused_mut)] // we need it mutable for the CMC case
    let mut initiator_cbuilder = CiphersuiteBuilder::new(initiator_ciphersuite_id)
        .longterm_x25519_keys(&setup.initiator_x25519_keys)
        .peer_longterm_x25519_pk(&setup.responder_x25519_keys.pk)
        .peer_longterm_mlkem_pk(setup.responder_mlkem_keys.public_key());

    if let Some(signature_key_pair) = setup.initiator_signature_keys(initiator_ciphersuite_id) {
        initiator_cbuilder = initiator_cbuilder.longterm_signing_keys(signature_key_pair);
    }

    #[cfg(feature = "classic-mceliece")]
    {
        initiator_cbuilder = initiator_cbuilder.peer_longterm_cmc_pk(&setup.responder_cmc_keys.pk);
    }
    let initiator_ciphersuite = initiator_cbuilder.build_initiator_ciphersuite().unwrap();

    let mut initiator = PrincipalBuilder::new(rand::rng())
        .outer_aad(aad_initiator_outer)
        .inner_aad(aad_initiator_inner)
        .context(ctx)
        .build_registration_initiator(initiator_ciphersuite)
        .unwrap();

    // Setup responder
    #[allow(unused_mut)] // we need it mutable for the CMC case
    let mut responder_cbuilder = CiphersuiteBuilder::new(responder_ciphersuite_id)
        .longterm_x25519_keys(&setup.responder_x25519_keys)
        .longterm_mlkem_encapsulation_key(setup.responder_mlkem_keys.public_key())
        .longterm_mlkem_decapsulation_key(setup.responder_mlkem_keys.private_key());

    #[cfg(feature = "classic-mceliece")]
    {
        responder_cbuilder = responder_cbuilder
            .longterm_cmc_encapsulation_key(&setup.responder_cmc_keys.pk)
            .longterm_cmc_decapsulation_key(&setup.responder_cmc_keys.sk);
    }
    let responder_ciphersuite = responder_cbuilder.build_responder_ciphersuite().unwrap();

    let mut responder = PrincipalBuilder::new(rand::rng())
        .context(ctx)
        .outer_aad(aad_responder)
        .recent_keys_upper_bound(30)
        .build_responder(responder_ciphersuite)
        .unwrap();

    // Send first message
    let registration_payload_initiator = b"Registration_init";
    let len_i = initiator
        .write_message(registration_payload_initiator, &mut msg_channel)
        .unwrap();

    // Read first message
    let (len_r_deserialized, len_r_payload) = responder
        .read_message(&msg_channel, &mut payload_buf_responder)
        .unwrap();

    // We read the same amount of data.
    assert_eq!(len_r_deserialized, len_i);
    assert_eq!(len_r_payload, registration_payload_initiator.len());
    assert_eq!(
        &payload_buf_responder[0..len_r_payload],
        registration_payload_initiator
    );

    // Get the authenticator out here, so we can deserialize the session later.
    let Some(initiator_authenticator) = responder.initiator_authenticator() else {
        panic!("No initiator authenticator found")
    };

    // Respond
    let registration_payload_responder = b"Registration_respond";
    let len_r = responder
        .write_message(registration_payload_responder, &mut msg_channel)
        .unwrap();

    // Finalize on registration initiator
    let (len_i_deserialized, len_i_payload) = initiator
        .read_message(&msg_channel, &mut payload_buf_initiator)
        .unwrap();

    // We read the same amount of data.
    assert_eq!(len_r, len_i_deserialized);
    assert_eq!(registration_payload_responder.len(), len_i_payload);
    assert_eq!(
        &payload_buf_initiator[0..len_i_payload],
        registration_payload_responder
    );

    // Ready for transport mode
    assert!(initiator.is_handshake_finished());
    assert!(responder.is_handshake_finished());

    let i_transport = initiator.into_session().unwrap();
    let r_transport = responder.into_session().unwrap();

    // test serialization, deserialization
    let mut session_storage = vec![0u8; 4096];
    i_transport
        .serialize(
            &mut session_storage,
            SessionBinding {
                initiator_authenticator: &setup.initiator_authenticator(initiator_ciphersuite_id),
                responder_ecdh_pk: &setup.responder_x25519_keys.pk,
                responder_pq_pk: setup.pq_encapsulation_key(initiator_ciphersuite_id),
            },
        )
        .unwrap();
    let mut i_transport = Session::deserialize(
        &session_storage,
        SessionBinding {
            initiator_authenticator: &setup.initiator_authenticator(initiator_ciphersuite_id),
            responder_ecdh_pk: &setup.responder_x25519_keys.pk,
            responder_pq_pk: setup.pq_encapsulation_key(initiator_ciphersuite_id),
        },
    )
    .unwrap();

    r_transport
        .serialize(
            &mut session_storage,
            SessionBinding {
                initiator_authenticator: &initiator_authenticator,
                responder_ecdh_pk: &setup.responder_x25519_keys.pk,
                responder_pq_pk: setup.pq_encapsulation_key(initiator_ciphersuite_id),
            },
        )
        .unwrap();
    let mut r_transport = Session::deserialize(
        &session_storage,
        SessionBinding {
            initiator_authenticator: &initiator_authenticator,
            responder_ecdh_pk: &setup.responder_x25519_keys.pk,
            responder_pq_pk: setup.pq_encapsulation_key(initiator_ciphersuite_id),
        },
    )
    .unwrap();

    let mut channel_i = i_transport.transport_channel().unwrap();
    let mut channel_r = r_transport.transport_channel().unwrap();

    assert_eq!(channel_i.identifier(), channel_r.identifier());

    let app_data_i = b"Derived session hey".as_slice();
    let app_data_r = b"Derived session ho".as_slice();

    let len_i = channel_i
        .write_message(app_data_i, &mut msg_channel)
        .unwrap();

    let (len_r_deserialized, len_r_payload) = channel_r
        .read_message(&msg_channel, &mut payload_buf_responder)
        .unwrap();

    // We read the same amount of data.
    assert_eq!(len_r_deserialized, len_i);
    assert_eq!(len_r_payload, app_data_i.len());
    assert_eq!(&payload_buf_responder[0..len_r_payload], app_data_i);

    let len_r = channel_r
        .write_message(app_data_r, &mut msg_channel)
        .unwrap();

    let (len_i_deserialized, len_i_payload) = channel_i
        .read_message(&msg_channel, &mut payload_buf_initiator)
        .unwrap();

    assert_eq!(len_r, len_i_deserialized);
    assert_eq!(app_data_r.len(), len_i_payload);
    assert_eq!(&payload_buf_initiator[0..len_i_payload], app_data_r);

    Ok(())
}

#[test]
fn compatibility_matching_ciphersuites() {
    eprintln!("Testing symmetric ciphersuites");
    macro_rules! symmetric_compat {
        ($suite:expr) => {
            eprintln!("Testing {} against itself", stringify!($suite));
            assert!(registration(&*SETUP, $suite, $suite,).is_ok());
            assert!(registration_no_encoding(&*SETUP, $suite, $suite,).is_ok());
        };
        ($suite:expr, $($other_suite:expr),*) => {
            eprintln!("Testing {} against itself", stringify!($suite));
            assert!(registration(&*SETUP, $suite, $suite).is_ok());
            assert!(registration_no_encoding(&*SETUP, $suite, $suite).is_ok());

            symmetric_compat!($($other_suite),+);
        };
    }

    symmetric_compat!(
        CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_NONE_X25519_AESGCM128_HKDFSHA256,
        CiphersuiteName::X25519_MLKEM768_X25519_AESGCM128_HKDFSHA256,
        CiphersuiteName::X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_MLKEM768_ED25519_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_NONE_ED25519_AESGCM128_HKDFSHA256,
        CiphersuiteName::X25519_MLKEM768_ED25519_AESGCM128_HKDFSHA256,
        CiphersuiteName::X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256,
        CiphersuiteName::X25519_MLKEM768_MLDSA65_AESGCM128_HKDFSHA256
    );
    #[cfg(feature = "classic-mceliece")]
    symmetric_compat!(
        CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256,
        CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256,
        CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
    );
}

#[test]
fn compatible_ciphersuites_asymmetric_mlkem() {
    eprintln!("Testing ciphersuite compatibility");
    macro_rules! asymmetric_compat {
        (($suite_none:expr, $suite_mlkem:expr, $suite_cmc:expr)) => {
            eprintln!("Testing {} against {} (ML-KEM)", stringify!($suite_none), stringify!($suite_mlkem));
            assert!(registration(&*SETUP, $suite_none, $suite_mlkem).is_ok());
            assert!(registration_no_encoding(&*SETUP, $suite_none, $suite_mlkem).is_ok());
            #[cfg(feature = "classic-mceliece")]
            {
                eprintln!("Testing {} against {} (CLASSIC-MCELIECE)", stringify!($suite_none), stringify!($suite_cmc));
                assert!(registration(&*SETUP, $suite_none, $suite_cmc).is_ok());
                assert!(registration_no_encoding(&*SETUP, $suite_none, $suite_cmc).is_ok());
            }
        };
        (($suite_none:expr, $suite_mlkem:expr, $suite_cmc:expr), $(($other_suite_none:expr, $other_suite_mlkem:expr, $other_suite_cmc:expr)),*) => {
            eprintln!("Testing {} against {} (ML-KEM)", stringify!($suite_none), stringify!($suite_mlkem));
            assert!(registration(&*SETUP, $suite_none, $suite_mlkem).is_ok());
            assert!(registration_no_encoding(&*SETUP, $suite_none, $suite_mlkem).is_ok());
            #[cfg(feature = "classic-mceliece")]
            {
                eprintln!("Testing {} against {} (CLASSIC-MCELIECE)", stringify!($suite_none), stringify!($suite_cmc));
                assert!(registration(&*SETUP, $suite_none, $suite_cmc).is_ok());
                assert!(registration_no_encoding(&*SETUP, $suite_none, $suite_cmc).is_ok());
            }

            asymmetric_compat!($(($other_suite_none, $other_suite_mlkem, $other_suite_cmc)),+);
        };
    }

    asymmetric_compat!(
        (
            CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256,
            CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256,
            CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256
        ),
        (
            CiphersuiteName::X25519_NONE_ED25519_CHACHA20POLY1305_HKDFSHA256,
            CiphersuiteName::X25519_MLKEM768_ED25519_CHACHA20POLY1305_HKDFSHA256,
            CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256
        ),
        (
            CiphersuiteName::X25519_NONE_MLDSA65_CHACHA20POLY1305_HKDFSHA256,
            CiphersuiteName::X25519_MLKEM768_MLDSA65_CHACHA20POLY1305_HKDFSHA256,
            CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256
        ),
        (
            CiphersuiteName::X25519_NONE_X25519_AESGCM128_HKDFSHA256,
            CiphersuiteName::X25519_MLKEM768_X25519_AESGCM128_HKDFSHA256,
            CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256
        ),
        (
            CiphersuiteName::X25519_NONE_ED25519_AESGCM128_HKDFSHA256,
            CiphersuiteName::X25519_MLKEM768_ED25519_AESGCM128_HKDFSHA256,
            CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256
        ),
        (
            CiphersuiteName::X25519_NONE_MLDSA65_AESGCM128_HKDFSHA256,
            CiphersuiteName::X25519_MLKEM768_MLDSA65_AESGCM128_HKDFSHA256,
            CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
        )
    );
}

#[test]
#[cfg(not(feature = "classic-mceliece"))]
fn unsupported_classic_mceliece() {
    // Trying to build an initiator or responder with a CMC suite should fail.
    eprintln!("Testing unsupported Classic McEliece suites will error");
    macro_rules! unsupported_cmc {
        ($suite:expr) => {
            let initiator_result = CiphersuiteBuilder::new($suite).build_initiator_ciphersuite();
            assert!(
                initiator_result.is_err(),
            );
            let responder_result = CiphersuiteBuilder::new($suite).build_responder_ciphersuite();
            assert!(
                responder_result.is_err(),
            );
        };
        ($suite:expr, $($other_suite:expr),*) => {
            let initiator_result = CiphersuiteBuilder::new($suite).build_initiator_ciphersuite();
            assert!(
                initiator_result.is_err(),
            );
            let responder_result = CiphersuiteBuilder::new($suite).build_initiator_ciphersuite();
            assert!(
                responder_result.is_err(),
            );

            unsupported_cmc!($($other_suite),+);
        };
    }

    unsupported_cmc!(
        CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_CLASSICMCELIECE_X25519_AESGCM128_HKDFSHA256,
        CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_CLASSICMCELIECE_ED25519_AESGCM128_HKDFSHA256,
        CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_CLASSICMCELIECE_MLDSA65_AESGCM128_HKDFSHA256
    );
}

fn query_no_encoding(setup: &CommonSetup, responder_ciphersuite_id: CiphersuiteName) {
    let ctx = b"Test Context";
    let aad_initiator = b"Test Data I";
    let aad_responder = b"Test Data R";

    // Setup initiator
    let mut initiator = PrincipalBuilder::new(rand::rng())
        .outer_aad(aad_initiator)
        .context(ctx)
        .build_query_initiator(&setup.responder_x25519_keys.pk)
        .unwrap();

    // Setup responder
    #[allow(unused_mut)] // we need it mutable for the CMC case
    let mut responder_cbuilder = CiphersuiteBuilder::new(responder_ciphersuite_id)
        .longterm_x25519_keys(&setup.responder_x25519_keys)
        .longterm_mlkem_encapsulation_key(setup.responder_mlkem_keys.public_key())
        .longterm_mlkem_decapsulation_key(setup.responder_mlkem_keys.private_key());

    #[cfg(feature = "classic-mceliece")]
    {
        responder_cbuilder = responder_cbuilder
            .longterm_cmc_encapsulation_key(&setup.responder_cmc_keys.pk)
            .longterm_cmc_decapsulation_key(&setup.responder_cmc_keys.sk);
    }
    let responder_ciphersuite = responder_cbuilder.build_responder_ciphersuite().unwrap();

    let mut responder = PrincipalBuilder::new(rand::rng())
        .context(ctx)
        .outer_aad(aad_responder)
        .recent_keys_upper_bound(30)
        .build_responder(responder_ciphersuite)
        .unwrap();

    // Send first message
    let query_payload_initiator = b"Query_init";
    let sent_initiator_message = initiator
        .write_message_external_encoding(query_payload_initiator)
        .unwrap();

    // Read first message
    let received_initiator_message = responder
        .read_message_external_encoding(&sent_initiator_message)
        .unwrap();

    assert_eq!(received_initiator_message, query_payload_initiator.to_vec());

    // Respond
    let query_payload_responder = b"Query_respond";
    let sent_response = responder
        .write_message_external_encoding(query_payload_responder)
        .unwrap();

    // Finalize on query initiator
    let received_response = initiator
        .read_message_external_encoding(&sent_response)
        .unwrap();

    assert_eq!(received_response, query_payload_responder.to_vec());
}

fn query(setup: &CommonSetup, responder_ciphersuite_id: CiphersuiteName) {
    let ctx = b"Test Context";
    let aad_initiator = b"Test Data I";
    let aad_responder = b"Test Data R";

    let mut msg_channel = vec![0u8; 4096];
    let mut payload_buf_responder = vec![0u8; 4096];
    let mut payload_buf_initiator = vec![0u8; 4096];

    // Setup initiator
    let mut initiator = PrincipalBuilder::new(rand::rng())
        .outer_aad(aad_initiator)
        .context(ctx)
        .build_query_initiator(&setup.responder_x25519_keys.pk)
        .unwrap();

    // Setup responder
    #[allow(unused_mut)] // we need it mutable for the CMC case
    let mut responder_cbuilder = CiphersuiteBuilder::new(responder_ciphersuite_id)
        .longterm_x25519_keys(&setup.responder_x25519_keys)
        .longterm_mlkem_encapsulation_key(setup.responder_mlkem_keys.public_key())
        .longterm_mlkem_decapsulation_key(setup.responder_mlkem_keys.private_key());

    #[cfg(feature = "classic-mceliece")]
    {
        responder_cbuilder = responder_cbuilder
            .longterm_cmc_encapsulation_key(&setup.responder_cmc_keys.pk)
            .longterm_cmc_decapsulation_key(&setup.responder_cmc_keys.sk);
    }
    let responder_ciphersuite = responder_cbuilder.build_responder_ciphersuite().unwrap();

    let mut responder = PrincipalBuilder::new(rand::rng())
        .context(ctx)
        .outer_aad(aad_responder)
        .recent_keys_upper_bound(30)
        .build_responder(responder_ciphersuite)
        .unwrap();

    // Send first message
    let query_payload_initiator = b"Query_init";
    let len_i = initiator
        .write_message(query_payload_initiator, &mut msg_channel)
        .unwrap();

    // Read first message
    let (len_r_deserialized, len_r_payload) = responder
        .read_message(&msg_channel, &mut payload_buf_responder)
        .unwrap();

    // We read the same amount of data.
    assert_eq!(len_r_deserialized, len_i);
    assert_eq!(len_r_payload, b"Query init".len());
    assert_eq!(
        &payload_buf_responder[0..len_r_payload],
        query_payload_initiator
    );

    // Respond
    let query_payload_responder = b"Query_respond";
    let len_r = responder
        .write_message(query_payload_responder, &mut msg_channel)
        .unwrap();

    // Finalize on query initiator
    let (len_i_deserialized, len_i_payload) = initiator
        .read_message(&msg_channel, &mut payload_buf_initiator)
        .unwrap();

    // We read the same amount of data.
    assert_eq!(len_r, len_i_deserialized);
    assert_eq!(query_payload_responder.len(), len_i_payload);
    assert_eq!(
        &payload_buf_initiator[0..len_i_payload],
        query_payload_responder
    );
}

#[test]
fn compatibility_query() {
    query(
        &*SETUP,
        CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256,
    );
    query(
        &*SETUP,
        CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256,
    );
    #[cfg(feature = "classic-mceliece")]
    query(
        &*SETUP,
        CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256,
    );
    query_no_encoding(
        &*SETUP,
        CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256,
    );
    query_no_encoding(
        &*SETUP,
        CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256,
    );
    #[cfg(feature = "classic-mceliece")]
    query_no_encoding(
        &*SETUP,
        CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256,
    );
}

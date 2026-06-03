//! # PQ-PSK
//!
//! This crate implements a protocol for establishing and mutually
//! registering a PQ-PSK between an initiator and a responder.
//!
//! ```rust
//! use libcrux_ml_kem::mlkem768::MlKem768KeyPair;
//! use libcrux_psq::{
//!     handshake::{builders::*, ciphersuites::*, types::*, HandshakeError},
//!     session::{Session, SessionBinding, SessionError},
//!     Channel, IntoSession,
//! };
//! use rand::Rng;
//! use tls_codec::{Deserialize, Serialize};
//!
//! let mut rng = rand::rng();
//!
//! // External setup: Responder keys
//! let responder_mlkem_keys = libcrux_ml_kem::mlkem768::rand::generate_key_pair(&mut rng);
//!
//! let responder_ecdh_keys = DHKeyPair::new(&mut rng);
//!
//! // External setup: Initiator keys
//! let initiator_ecdh_keys = DHKeyPair::new(&mut rng);
//!
//! let ctx = b"Test Context";
//! let aad_initiator_query = b"Test Data Initiator Query";
//! let aad_initiator_outer = b"Test Data I Outer";
//! let aad_initiator_inner = b"Test Data I Inner";
//! let aad_responder = b"Test Data R";
//!
//! let mut msg_channel = vec![0u8; 4096];
//! let mut payload_buf_responder = vec![0u8; 4096];
//! let mut payload_buf_initiator = vec![0u8; 4096];
//!
//! let responder_ciphersuite_id =
//!     CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256;
//!
//! // Setup query initiator
//! let mut query_initiator = PrincipalBuilder::new(rand::rng())
//!     .outer_aad(aad_initiator_query)
//!     .context(ctx)
//!     .build_query_initiator(&responder_ecdh_keys.pk)
//!     .unwrap();
//!
//! // Setup responder
//! let mut responder_ciphersuite = CiphersuiteBuilder::new(responder_ciphersuite_id)
//!     .longterm_x25519_keys(&responder_ecdh_keys)
//!     .longterm_mlkem_encapsulation_key(responder_mlkem_keys.public_key())
//!     .longterm_mlkem_decapsulation_key(responder_mlkem_keys.private_key())
//!     .build_responder_ciphersuite()
//!     .unwrap();
//!
//! let mut responder = PrincipalBuilder::new(rand::rng())
//!     .context(ctx)
//!     .outer_aad(aad_responder)
//!     .recent_keys_upper_bound(30)
//!     .build_responder(responder_ciphersuite)
//!     .unwrap();
//!
//! // Query the responder for its ciphersuite
//! let query_payload_initiator = b"Query_init"; // This message is application defined
//! let len_i = query_initiator
//!     .write_message(query_payload_initiator, &mut msg_channel)
//!     .unwrap();
//!
//! // Read first message at the responder
//! let (len_r_deserialized, len_r_payload) = responder
//!     .read_message(&msg_channel, &mut payload_buf_responder)
//!     .unwrap();
//!
//! // Respond
//! let query_payload_responder = responder_ciphersuite_id.tls_serialize_detached().unwrap();
//! let len_r = responder
//!     .write_message(&query_payload_responder, &mut msg_channel)
//!     .unwrap();
//!
//! // Finalize on query initiator
//! let (len_i_deserialized, len_i_payload) = query_initiator
//!     .read_message(&msg_channel, &mut payload_buf_initiator)
//!     .unwrap();
//!
//! let responder_ciphersuite_id_received =
//!     CiphersuiteName::tls_deserialize(&mut std::io::Cursor::new(&payload_buf_initiator))
//!         .unwrap();
//!
//! assert_eq!(responder_ciphersuite_id, responder_ciphersuite_id_received);
//!
//! // Setup Registration initiator with received ciphersuite
//! let initiator_ciphersuite = CiphersuiteBuilder::new(responder_ciphersuite_id_received)
//!     .longterm_x25519_keys(&initiator_ecdh_keys)
//!     .peer_longterm_x25519_pk(&responder_ecdh_keys.pk)
//!     .peer_longterm_mlkem_pk(responder_mlkem_keys.public_key())
//!     .build_initiator_ciphersuite()
//!     .unwrap();
//!
//! let mut initiator = PrincipalBuilder::new(rand::rng())
//!     .outer_aad(aad_initiator_outer)
//!     .inner_aad(aad_initiator_inner)
//!     .context(ctx)
//!     .build_registration_initiator(initiator_ciphersuite)
//!     .unwrap();
//!
//! // Send first message
//! let registration_payload_initiator = b"Registration_init";
//! let len_i = initiator
//!     .write_message(registration_payload_initiator, &mut msg_channel)
//!     .unwrap();
//!
//! // Read first message
//! let (len_r_deserialized, len_r_payload) = responder
//!     .read_message(&msg_channel, &mut payload_buf_responder)
//!     .unwrap();
//!
//! // Respond
//! let registration_payload_responder = b"Registration_respond";
//! let len_r = responder
//!     .write_message(registration_payload_responder, &mut msg_channel)
//!     .unwrap();
//!
//! // Finalize on registration initiator
//! let (len_i_deserialized, len_i_payload) = initiator
//!     .read_message(&msg_channel, &mut payload_buf_initiator)
//!     .unwrap();
//!
//! // Ready for transport mode
//! assert!(initiator.is_handshake_finished());
//! assert!(responder.is_handshake_finished());
//!
//! let i_transport = initiator.into_session().unwrap();
//! let mut r_transport = responder.into_session().unwrap();
//!
//! // test serialization, deserialization
//! let mut session_storage = vec![0u8; 4096];
//! i_transport
//!     .serialize(
//!         &mut session_storage,
//!         SessionBinding {
//!             initiator_authenticator: &(&initiator_ecdh_keys.pk).authenticator(),
//!             responder_ecdh_pk: &responder_ecdh_keys.pk,
//!             responder_pq_pk: Some(responder_mlkem_keys.public_key().into()),
//!         },
//!     )
//!     .unwrap();
//!
//! let mut i_transport = Session::deserialize(
//!     &session_storage,
//!     SessionBinding {
//!         initiator_authenticator: &(&initiator_ecdh_keys.pk).authenticator(),
//!         responder_ecdh_pk: &responder_ecdh_keys.pk,
//!         responder_pq_pk: Some(responder_mlkem_keys.public_key().into()),
//!     },
//! )
//! .unwrap();
//!
//! let mut channel_i = i_transport.transport_channel().unwrap();
//! let mut channel_r = r_transport.transport_channel().unwrap();
//!
//! assert_eq!(channel_i.identifier(), channel_r.identifier());
//!
//! let app_data_i = b"Derived session hey".as_slice();
//! let app_data_r = b"Derived session ho".as_slice();
//!
//! let len_i = channel_i
//!     .write_message(app_data_i, &mut msg_channel)
//!     .unwrap();
//!
//! let (len_r_deserialized, len_r_payload) = channel_r
//!     .read_message(&msg_channel, &mut payload_buf_responder)
//!     .unwrap();
//!
//! let len_r = channel_r
//!     .write_message(app_data_r, &mut msg_channel)
//!     .unwrap();
//!
//! let (len_i_deserialized, len_i_payload) = channel_i
//!     .read_message(&msg_channel, &mut payload_buf_initiator)
//!     .unwrap();
//! ```

#![warn(missing_docs)]

mod aead;
pub mod handshake;
pub mod session;
mod traits;

#[doc(inline)]
pub use traits::{Channel, IntoSession};

#[cfg(feature = "classic-mceliece")]
pub mod classic_mceliece;

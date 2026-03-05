use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use libcrux_ml_kem::mlkem768::MlKem768KeyPair;
#[cfg(feature = "classic-mceliece")]
use libcrux_psq::classic_mceliece::KeyPair;
use libcrux_psq::{
    handshake::{
        builders::{CiphersuiteBuilder, PrincipalBuilder},
        ciphersuites::CiphersuiteName,
        types::DHKeyPair,
        QueryInitiator, RegistrationInitiator, Responder,
    },
    Channel, IntoSession,
};
use rand::CryptoRng;

pub fn randombytes(n: usize) -> Vec<u8> {
    use rand::rngs::OsRng;
    use rand::TryRngCore;

    let mut bytes = vec![0u8; n];
    OsRng.try_fill_bytes(&mut bytes).unwrap();
    bytes
}

struct CommonSetup {
    pub responder_mlkem_keys: MlKem768KeyPair,
    #[cfg(feature = "classic-mceliece")]
    pub responder_cmc_keys: KeyPair,
    pub responder_ecdh_keys: DHKeyPair,
    pub initiator_ecdh_keys: DHKeyPair,
}

impl CommonSetup {
    fn new() -> Self {
        let mut rng = rand::rng();
        let responder_mlkem_keys = libcrux_ml_kem::mlkem768::rand::generate_key_pair(&mut rng);
        #[cfg(feature = "classic-mceliece")]
        let responder_cmc_keys =
            libcrux_psq::classic_mceliece::KeyPair::generate_key_pair(&mut rng);

        let responder_ecdh_keys = DHKeyPair::new(&mut rng);
        let initiator_ecdh_keys = DHKeyPair::new(&mut rng);

        CommonSetup {
            responder_mlkem_keys,
            #[cfg(feature = "classic-mceliece")]
            responder_cmc_keys,
            responder_ecdh_keys,
            initiator_ecdh_keys,
        }
    }
}

fn query<const PQ: bool>(c: &mut Criterion) {
    let ciphersuite = if PQ {
        CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256
    } else {
        CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256
    };
    let ctx = b"Test Context";
    let aad_initiator = b"Test Data I";
    let aad_responder = b"Test Data R";

    // External setup
    let setup = CommonSetup::new();

    // Setup initiator
    let mut initiator =
        query_initiator(rand::rng(), ctx, aad_initiator, &setup.responder_ecdh_keys);
    c.bench_function(&format!("[Query] Initiator setup"), |b| {
        b.iter_batched(
            || rand::rng(),
            |rng| {
                initiator = query_initiator(rng, ctx, aad_initiator, &setup.responder_ecdh_keys);
            },
            BatchSize::SmallInput,
        )
    });

    // Setup responder
    let mut responder = build_responder(&setup, ctx, aad_responder, ciphersuite);
    c.bench_function(&format!("[Query] Responder setup {ciphersuite:?}"), |b| {
        b.iter_batched(
            || rand::rng(),
            |_rng| {
                responder = build_responder(&setup, ctx, aad_responder, ciphersuite);
            },
            BatchSize::SmallInput,
        )
    });

    // Setup for sending messages.

    // Send first message
    c.bench_function(
        &format!("[Query] Initiator send query {ciphersuite:?}"),
        |b| {
            b.iter_batched_ref(
                || {
                    let msg_channel = vec![0u8; 4096];

                    let initiator = query_initiator(
                        rand::rng(),
                        ctx,
                        aad_initiator,
                        &setup.responder_ecdh_keys,
                    );

                    (initiator, msg_channel)
                },
                |(initiator, msg_channel)| {
                    let query_payload_initiator = b"Query_init";
                    let _len_i = initiator
                        .write_message(query_payload_initiator, msg_channel)
                        .unwrap();
                },
                BatchSize::SmallInput,
            )
        },
    );

    // Read first message
    c.bench_function(
        &format!("[Query] Responder read message {ciphersuite:?}"),
        |b| {
            b.iter_batched_ref(
                || {
                    let mut msg_channel = vec![0u8; 4096];
                    let payload_buf_responder = vec![0u8; 4096];

                    let mut initiator = query_initiator(
                        rand::rng(),
                        ctx,
                        aad_initiator,
                        &setup.responder_ecdh_keys,
                    );

                    let query_payload_initiator = b"Query_init";
                    let _len_i = initiator
                        .write_message(query_payload_initiator, &mut msg_channel)
                        .unwrap();

                    let responder = build_responder(&setup, ctx, aad_responder, ciphersuite);
                    (responder, msg_channel, payload_buf_responder)
                },
                |(responder, msg_channel, payload_buf_responder)| {
                    let (_len_r_deserialized, _len_r_payload) = responder
                        .read_message(msg_channel, payload_buf_responder)
                        .unwrap();
                },
                BatchSize::SmallInput,
            )
        },
    );

    // Respond
    c.bench_function(&format!("[Query] Responder respond {ciphersuite:?}"), |b| {
        b.iter_batched_ref(
            || {
                let mut msg_channel = vec![0u8; 4096];
                let mut payload_buf_responder = vec![0u8; 4096];

                let mut initiator =
                    query_initiator(rand::rng(), ctx, aad_initiator, &setup.responder_ecdh_keys);

                let query_payload_initiator = b"Query_init";
                let _len_i = initiator
                    .write_message(query_payload_initiator, &mut msg_channel)
                    .unwrap();

                let mut responder = build_responder(&setup, ctx, aad_responder, ciphersuite);
                let (_len_r_deserialized, _len_r_payload) = responder
                    .read_message(&msg_channel, &mut payload_buf_responder)
                    .unwrap();

                (responder, msg_channel)
            },
            |(responder, msg_channel)| {
                let query_payload_responder = b"Query_respond";
                let _len_r = responder
                    .write_message(query_payload_responder, msg_channel)
                    .unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    // Finalize on query initiator
    c.bench_function(
        &format!("[Query] Finalize initiator {ciphersuite:?}"),
        |b| {
            b.iter_batched_ref(
                || {
                    let mut msg_channel = vec![0u8; 4096];
                    let mut payload_buf_responder = vec![0u8; 4096];
                    let payload_buf_initiator = vec![0u8; 4096];

                    let mut initiator = query_initiator(
                        rand::rng(),
                        ctx,
                        aad_initiator,
                        &setup.responder_ecdh_keys,
                    );

                    let query_payload_initiator = b"Query_init";
                    let _len_i = initiator
                        .write_message(query_payload_initiator, &mut msg_channel)
                        .unwrap();

                    let mut responder = build_responder(&setup, ctx, aad_responder, ciphersuite);
                    let (_len_r_deserialized, _len_r_payload) = responder
                        .read_message(&msg_channel, &mut payload_buf_responder)
                        .unwrap();

                    let query_payload_responder = b"Query_respond";
                    let _len_r = responder
                        .write_message(query_payload_responder, &mut msg_channel)
                        .unwrap();

                    (initiator, msg_channel, payload_buf_initiator)
                },
                |(initiator, msg_channel, payload_buf_initiator)| {
                    let (_len_i_deserialized, _len_i_payload) = initiator
                        .read_message(msg_channel, payload_buf_initiator)
                        .unwrap();
                },
                BatchSize::SmallInput,
            )
        },
    );
}

fn registration(c: &mut Criterion, suite_i: CiphersuiteName, suite_r: CiphersuiteName) {
    let ctx = b"Test Context";
    let aad_initiator_outer = b"Test Data I Outer";
    let aad_initiator_inner = b"Test Data I Inner";
    let aad_responder = b"Test Data R";

    // External setup
    let setup = CommonSetup::new();

    c.bench_function(&format!("[Registration] Initiator setup"), |b| {
        b.iter_batched(
            || rand::rng(),
            |_rng| {
                let _initiator = registration_initiator(
                    &setup,
                    ctx,
                    aad_initiator_outer,
                    aad_initiator_inner,
                    suite_i,
                );
            },
            BatchSize::SmallInput,
        )
    });

    // Setup responder
    let mut responder = build_responder(&setup, ctx, aad_responder, suite_r);

    c.bench_function(
        &format!("[Registration] Responder setup ; Initiator({suite_i:?}) Responder({suite_r:?})"),
        |b| {
            b.iter_batched(
                || rand::rng(),
                |_rng| {
                    responder = build_responder(&setup, ctx, aad_responder, suite_r);
                },
                BatchSize::SmallInput,
            )
        },
    );

    // Setup for sending messages.

    // Send first message
    c.bench_function(
        &format!("[Registration] Initiator send registration ; Initiator({suite_i:?}) Responder({suite_r:?})"),
        |b| {
            b.iter_batched_ref(
                || {
                    let msg_channel = vec![0u8; 4096];

                    let initiator = registration_initiator(
                        &setup,
                        ctx,
                        aad_initiator_outer,
                        aad_initiator_inner,
                        suite_i,
                    );

                    (initiator, msg_channel)
                },
                |(initiator, msg_channel)| {
                    let registration_payload_initiator = b"Registration_init";
                    let _len_i = initiator
                        .write_message(registration_payload_initiator, msg_channel)
                        .unwrap();
                },
                BatchSize::SmallInput,
            )
        },
    );

    // Read first message
    c.bench_function(
        &format!(
            "[Registration] Responder read message ; Initiator({suite_i:?}) Responder({suite_r:?})"
        ),
        |b| {
            b.iter_batched_ref(
                || {
                    let mut msg_channel = vec![0u8; 4096];
                    let payload_buf_responder = vec![0u8; 4096];

                    let mut initiator = registration_initiator(
                        &setup,
                        ctx,
                        aad_initiator_outer,
                        aad_initiator_inner,
                        suite_i,
                    );

                    let registration_payload_initiator = b"Registration_init";
                    let _len_i = initiator
                        .write_message(registration_payload_initiator, &mut msg_channel)
                        .unwrap();

                    let responder = build_responder(&setup, ctx, aad_responder, suite_r);

                    (responder, msg_channel, payload_buf_responder)
                },
                |(responder, msg_channel, payload_buf_responder)| {
                    let (_len_r_deserialized, _len_r_payload) = responder
                        .read_message(msg_channel, payload_buf_responder)
                        .unwrap();
                },
                BatchSize::SmallInput,
            )
        },
    );

    // Respond
    c.bench_function(
        &format!(
            "[Registration] Responder respond ; Initiator({suite_i:?}) Responder({suite_r:?})"
        ),
        |b| {
            b.iter_batched_ref(
                || {
                    let mut msg_channel = vec![0u8; 4096];
                    let mut payload_buf_responder = vec![0u8; 4096];

                    let mut initiator = registration_initiator(
                        &setup,
                        ctx,
                        aad_initiator_outer,
                        aad_initiator_inner,
                        suite_i,
                    );

                    let registration_payload_initiator = b"Registration_init";
                    let _len_i = initiator
                        .write_message(registration_payload_initiator, &mut msg_channel)
                        .unwrap();

                    let mut responder = build_responder(&setup, ctx, aad_responder, suite_r);

                    let (_len_r_deserialized, _len_r_payload) = responder
                        .read_message(&msg_channel, &mut payload_buf_responder)
                        .unwrap();

                    (responder, msg_channel)
                },
                |(responder, msg_channel)| {
                    let registration_payload_responder = b"Registration_respond";
                    let _len_r = responder
                        .write_message(registration_payload_responder, msg_channel)
                        .unwrap();
                },
                BatchSize::SmallInput,
            )
        },
    );

    // Finalize on registration initiator
    c.bench_function(
        &format!(
            "[Registration] Finalize initiator ; Initiator({suite_i:?}) Responder({suite_r:?})"
        ),
        |b| {
            b.iter_batched_ref(
                || {
                    let mut msg_channel = vec![0u8; 4096];
                    let mut payload_buf_responder = vec![0u8; 4096];
                    let payload_buf_initiator = vec![0u8; 4096];

                    let mut initiator = registration_initiator(
                        &setup,
                        ctx,
                        aad_initiator_outer,
                        aad_initiator_inner,
                        suite_i,
                    );

                    let registration_payload_initiator = b"Registration_init";
                    let _len_i = initiator
                        .write_message(registration_payload_initiator, &mut msg_channel)
                        .unwrap();

                    let mut responder = build_responder(&setup, ctx, aad_responder, suite_r);

                    let (_len_r_deserialized, _len_r_payload) = responder
                        .read_message(&msg_channel, &mut payload_buf_responder)
                        .unwrap();

                    let registration_payload_responder = b"Registration_respond";
                    let _len_r = responder
                        .write_message(registration_payload_responder, &mut msg_channel)
                        .unwrap();

                    (initiator, msg_channel, payload_buf_initiator)
                },
                |(initiator, msg_channel, payload_buf_initiator)| {
                    let (_len_i_deserialized, _len_i_payload) = initiator
                        .read_message(msg_channel, payload_buf_initiator)
                        .unwrap();
                },
                BatchSize::SmallInput,
            )
        },
    );

    // IntoTransport transform Initiator
    c.bench_function(
        &format!("[Registration] IntoTransport Responder ; Initiator({suite_i:?}) Responder({suite_r:?})"),
        |b| {
            b.iter_batched(
                || {
                    let mut msg_channel = vec![0u8; 4096];
                    let mut payload_buf_responder = vec![0u8; 4096];

                    let mut initiator = registration_initiator(
                        &setup,
                        ctx,
                        aad_initiator_outer,
                        aad_initiator_inner,
                        suite_i,
                    );

                    let registration_payload_initiator = b"Registration_init";
                    let _len_i = initiator
                        .write_message(registration_payload_initiator, &mut msg_channel)
                        .unwrap();

                    let mut responder = build_responder(&setup, ctx, aad_responder, suite_r);

                    let (_len_r_deserialized, _len_r_payload) = responder
                        .read_message(&msg_channel, &mut payload_buf_responder)
                        .unwrap();

                    let registration_payload_responder = b"Registration_respond";
                    let _len_r = responder
                        .write_message(registration_payload_responder, &mut msg_channel)
                        .unwrap();

                    responder
                },
                |responder| {
                    let _ = responder.into_session();
                },
                BatchSize::SmallInput,
            )
        },
    );

    // IntoTransport transform Initiator
    c.bench_function(
        &format!("[Registration] IntoTransport Initiator ; Initiator({suite_i:?}) Responder({suite_r:?})"),
        |b| {
            b.iter_batched(
                || {
                    let mut msg_channel = vec![0u8; 4096];
                    let mut payload_buf_responder = vec![0u8; 4096];
                    let mut payload_buf_initiator = vec![0u8; 4096];

                    let mut initiator = registration_initiator(
                        &setup,
                        ctx,
                        aad_initiator_outer,
                        aad_initiator_inner,
                        suite_i,
                    );

                    let registration_payload_initiator = b"Registration_init";
                    let _len_i = initiator
                        .write_message(registration_payload_initiator, &mut msg_channel)
                        .unwrap();

                    let mut responder = build_responder(&setup, ctx, aad_responder, suite_r);

                    let (_len_r_deserialized, _len_r_payload) = responder
                        .read_message(&msg_channel, &mut payload_buf_responder)
                        .unwrap();

                    let registration_payload_responder = b"Registration_respond";
                    let _len_r = responder
                        .write_message(registration_payload_responder, &mut msg_channel)
                        .unwrap();

                    let (_len_i_deserialized, _len_i_payload) = initiator
                        .read_message(&msg_channel, &mut payload_buf_initiator)
                        .unwrap();

                    initiator
                },
                |initiator| {
                    let _ = initiator.into_session();
                },
                BatchSize::SmallInput,
            )
        },
    );

    // Transport write message
    c.bench_function(
        &format!("[Registration] Transport Write ; Initiator({suite_i:?}) Responder({suite_r:?})"),
        |b| {
            b.iter_batched_ref(
                || {
                    let mut msg_channel = vec![0u8; 5050];
                    let mut payload_buf_responder = vec![0u8; 4096];
                    let mut payload_buf_initiator = vec![0u8; 4096];

                    let mut initiator = registration_initiator(
                        &setup,
                        ctx,
                        aad_initiator_outer,
                        aad_initiator_inner,
                        suite_i,
                    );

                    let registration_payload_initiator = b"Registration_init";
                    let _len_i = initiator
                        .write_message(registration_payload_initiator, &mut msg_channel)
                        .unwrap();

                    let mut responder = build_responder(&setup, ctx, aad_responder, suite_r);

                    let (_len_r_deserialized, _len_r_payload) = responder
                        .read_message(&msg_channel, &mut payload_buf_responder)
                        .unwrap();

                    let registration_payload_responder = b"Registration_respond";
                    let _len_r = responder
                        .write_message(registration_payload_responder, &mut msg_channel)
                        .unwrap();

                    let (_len_i_deserialized, _len_i_payload) = initiator
                        .read_message(&msg_channel, &mut payload_buf_initiator)
                        .unwrap();

                    let mut initiator = initiator.into_session().unwrap();
                    let channel = initiator.transport_channel().unwrap();
                    let payload = randombytes(4096);
                    (channel, msg_channel, payload)
                },
                |(channel, msg_channel, payload)| {
                    let _ = channel.write_message(payload, msg_channel).unwrap();
                },
                BatchSize::SmallInput,
            )
        },
    );

    // Transport read message
    c.bench_function(
        &format!("[Registration] Transport Read ; Initiator({suite_i:?}) Responder({suite_r:?})"),
        |b| {
            b.iter_batched_ref(
                || {
                    let mut msg_channel = vec![0u8; 5050];
                    let mut payload_buf_responder = vec![0u8; 4096];
                    let mut payload_buf_initiator = vec![0u8; 4096];

                    let mut initiator = registration_initiator(
                        &setup,
                        ctx,
                        aad_initiator_outer,
                        aad_initiator_inner,
                        suite_i,
                    );

                    let registration_payload_initiator = b"Registration_init";
                    let _len_i = initiator
                        .write_message(registration_payload_initiator, &mut msg_channel)
                        .unwrap();

                    let mut responder = build_responder(&setup, ctx, aad_responder, suite_r);

                    let (_len_r_deserialized, _len_r_payload) = responder
                        .read_message(&msg_channel, &mut payload_buf_responder)
                        .unwrap();

                    let registration_payload_responder = b"Registration_respond";
                    let _len_r = responder
                        .write_message(registration_payload_responder, &mut msg_channel)
                        .unwrap();

                    let (_len_i_deserialized, _len_i_payload) = initiator
                        .read_message(&msg_channel, &mut payload_buf_initiator)
                        .unwrap();

                    let mut initiator = initiator.into_session().unwrap();

                    let mut initiator_channel = initiator.transport_channel().unwrap();

                    let _ = initiator_channel
                        .write_message(&randombytes(4096), &mut msg_channel)
                        .unwrap();

                    let mut responder = responder.into_session().unwrap();
                    let responder_channel = responder.transport_channel().unwrap();
                    (responder_channel, msg_channel, payload_buf_responder)
                },
                |(responder_channel, msg_channel, payload_buf_responder)| {
                    let _ = responder_channel
                        .read_message(msg_channel, payload_buf_responder)
                        .unwrap();
                },
                BatchSize::SmallInput,
            )
        },
    );
}

#[inline(always)]
fn build_responder<'a>(
    setup: &'a CommonSetup,
    ctx: &'a [u8],
    aad_responder: &'a [u8],
    ciphersuite_id: CiphersuiteName,
) -> Responder<'a, impl CryptoRng> {
    // Setup responder
    #[allow(unused_mut)] // we need it mutable for the CMC case
    let mut responder_cbuilder = CiphersuiteBuilder::new(ciphersuite_id)
        .longterm_x25519_keys(&setup.responder_ecdh_keys)
        .longterm_mlkem_encapsulation_key(setup.responder_mlkem_keys.public_key())
        .longterm_mlkem_decapsulation_key(setup.responder_mlkem_keys.private_key());

    #[cfg(feature = "classic-mceliece")]
    {
        responder_cbuilder = responder_cbuilder
            .longterm_cmc_encapsulation_key(&setup.responder_cmc_keys.pk)
            .longterm_cmc_decapsulation_key(&setup.responder_cmc_keys.sk);
    }
    let responder_ciphersuite = responder_cbuilder.build_responder_ciphersuite().unwrap();

    PrincipalBuilder::new(rand::rng())
        .context(ctx)
        .outer_aad(aad_responder)
        .recent_keys_upper_bound(30)
        .build_responder(responder_ciphersuite)
        .unwrap()
}

#[inline(always)]
fn query_initiator<'a>(
    rng: impl CryptoRng,
    ctx: &'a [u8],
    aad_initiator: &'a [u8],
    responder_ecdh_keys: &'a DHKeyPair,
) -> QueryInitiator<'a> {
    PrincipalBuilder::new(rng)
        .outer_aad(aad_initiator)
        .context(ctx)
        .build_query_initiator(&responder_ecdh_keys.pk)
        .unwrap()
}

#[inline(always)]
fn registration_initiator<'a>(
    setup: &'a CommonSetup,
    ctx: &'a [u8],
    aad_initiator_outer: &'a [u8],
    aad_initiator_inner: &'a [u8],
    ciphersuite_id: CiphersuiteName,
) -> RegistrationInitiator<'a, impl CryptoRng> {
    #[allow(unused_mut)] // we need it mutable for the CMC case
    let mut initiator_cbuilder = CiphersuiteBuilder::new(ciphersuite_id)
        .longterm_x25519_keys(&setup.initiator_ecdh_keys)
        .peer_longterm_x25519_pk(&setup.responder_ecdh_keys.pk)
        .peer_longterm_mlkem_pk(setup.responder_mlkem_keys.public_key());

    #[cfg(feature = "classic-mceliece")]
    {
        initiator_cbuilder = initiator_cbuilder.peer_longterm_cmc_pk(&setup.responder_cmc_keys.pk);
    }
    let initiator_ciphersuite = initiator_cbuilder.build_initiator_ciphersuite().unwrap();

    PrincipalBuilder::new(rand::rng())
        .outer_aad(aad_initiator_outer)
        .inner_aad(aad_initiator_inner)
        .context(ctx)
        .build_registration_initiator(initiator_ciphersuite)
        .unwrap()
}

pub fn protocol(c: &mut Criterion) {
    // PSQv2 query protocol
    query::<true>(c);
    query::<false>(c);
    // PSQv2 registration protocol
    registration(
        c,
        CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256,
    );
    registration(
        c,
        CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256,
    );
    registration(
        c,
        CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_MLKEM768_X25519_CHACHA20POLY1305_HKDFSHA256,
    );
    #[cfg(feature = "classic-mceliece")]
    registration(
        c,
        CiphersuiteName::X25519_NONE_X25519_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256,
    );
    #[cfg(feature = "classic-mceliece")]
    registration(
        c,
        CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256,
        CiphersuiteName::X25519_CLASSICMCELIECE_X25519_CHACHA20POLY1305_HKDFSHA256,
    );
}

criterion_group!(benches, protocol);
criterion_main!(benches);

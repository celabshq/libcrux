#![allow(non_snake_case)]
use std::io::Write as _;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use der::Decode as _;
use pkcs1::DecodeRsaPrivateKey;
use rand_core_old::OsRng;
use ring::signature::KeyPair;

const MSG_STR: &str = "the test message is 32 byte long";
const MSG: &[u8; 32] = b"the test message is 32 byte long";
const SALT: &[u8] = b"this is the test salt";

// With the openssl_keygen function above we'd sometimes get keys that are one byte too short.
// This function sidesteps that issue by rejecting such keys and resampling on rejection.
fn keygen(bits: usize) -> (Vec<u8>, openssl::rsa::Rsa<openssl::pkey::Private>) {
    for _ in 0..10 {
        let (der, key) = openssl_keygen(bits as u32);

        if parse_der_works(&der, bits) {
            return (der, key);
        }
    }

    panic!("couldn't generate a key that works well")
}

fn openssl_keygen(bits: u32) -> (Vec<u8>, openssl::rsa::Rsa<openssl::pkey::Private>) {
    let k =
        openssl::rsa::Rsa::generate_with_e(bits, &openssl::bn::BigNum::from_u32(0x10001).unwrap())
            .unwrap();
    let der = k.private_key_to_der().unwrap();

    (der, k)
}

fn parse_der_libcrux(sk_der: &[u8], _bits: usize) -> libcrux_rsa::VarLenPrivateKey<'_> {
    let mut decoder = der::SliceReader::new(sk_der).unwrap();
    let rsa_priv_key = pkcs1::RsaPrivateKey::decode(&mut decoder).unwrap();

    if !matches!(rsa_priv_key.public_exponent.as_bytes(), [1, 0, 1]) {
        panic!("unexpected modulus, expected 0x010001");
    }

    let n = rsa_priv_key.modulus.as_bytes();
    let n = trim_leading_zeroes(n);

    let d = rsa_priv_key.private_exponent.as_bytes();
    let d = trim_leading_zeroes(d);

    libcrux_rsa::VarLenPrivateKey::from_components(n, d)
        .map_err(|err| (err, format!("n.len={}, d.len={}", n.len(), d.len())))
        .unwrap()
}

// With the openssl_keygen function above we'd sometimes get keys that are one byte too short.
// This function checks whether the key size matches.
fn parse_der_works(sk_der: &[u8], bits: usize) -> bool {
    let mut decoder = der::SliceReader::new(sk_der).unwrap();
    let rsa_priv_key = pkcs1::RsaPrivateKey::decode(&mut decoder).unwrap();

    if !matches!(rsa_priv_key.public_exponent.as_bytes(), [1, 0, 1]) {
        panic!("unexpected modulus, expected 0x010001");
    }

    let n = rsa_priv_key.modulus.as_bytes();
    let n = trim_leading_zeroes(n);

    let d = rsa_priv_key.private_exponent.as_bytes();
    let d = trim_leading_zeroes(d);

    n.len() == bits / 8 && d.len() == bits / 8
}

fn trim_leading_zeroes(mut buf: &[u8]) -> &[u8] {
    while let Some(leading) = buf.first() {
        if *leading == 0 {
            buf = &buf[1..];
        } else {
            break;
        }
    }
    buf
}

macro_rules! verify {
    ($fun_name:ident, $bench_name:literal, $bits:literal, $bytes:literal) => {
        fn $fun_name(c: &mut Criterion) {
            let bits = $bits;
            print!("Benchmarking RSA Verify with {bits} bits");

            // Comparing libcrux performance for different payload sizes and other implementations.
            let mut group = c.benchmark_group($bench_name);

            // We can't do key generation each benchmark function's setup phase, because some of them
            // use references to the key material for cryptographic operations, and then the key would
            // be owned by the setup closure, which has returned by then.
            let (der_, sk) = keygen($bits);
            let der = der_.as_slice();

            println!("  libcrux@{bits}...");
            group.bench_function("libcrux", |b| {
                b.iter_batched(
                    || {
                        let sk = parse_der_libcrux(&der, bits);
                        let mut sig = [0; $bytes];
                        libcrux_rsa::sign_varlen(
                            libcrux_rsa::DigestAlgorithm::Sha2_256,
                            &sk,
                            MSG,
                            SALT,
                            &mut sig,
                        )
                        .unwrap();

                        (sk, sig)
                    },
                    |(sk, sig)| {
                        libcrux_rsa::verify_varlen(
                            libcrux_rsa::DigestAlgorithm::Sha2_256,
                            sk.pk(),
                            MSG,
                            SALT.len() as u32,
                            &sig,
                        )
                        .unwrap();
                    },
                    BatchSize::SmallInput,
                )
            });

            if bits <= 4096 {
                println!("  RustCrypto@{bits}...");

                group.bench_function("RustCrypto", |b| {
                    b.iter_batched(
                        || {
                            let sk = rsa::RsaPrivateKey::from_pkcs1_der(&der).unwrap();

                            let sig = sk
                                .sign_with_rng(
                                    &mut OsRng,
                                    rsa::Pss::new::<rsa::sha2::Sha256>(),
                                    MSG,
                                )
                                .unwrap();

                            let pk = sk.to_public_key();
                            (pk, sig)
                        },
                        |(pk, sig)| {
                            pk.verify(rsa::Pss::new::<rsa::sha2::Sha256>(), MSG, &sig)
                                .map_err(Option::Some)
                                .unwrap()
                        },
                        BatchSize::SmallInput,
                    )
                });

                println!("  Ring@{bits}...");

                group.bench_function("Ring", |b| {
                    b.iter_batched(
                        || {
                            let key_pair = ring::rsa::KeyPair::from_der(der).unwrap();
                            let mut sig = [0; $bytes];
                            key_pair
                                .sign(
                                    &ring::signature::RSA_PSS_SHA256,
                                    &ring::rand::SystemRandom::new(),
                                    MSG,
                                    &mut sig,
                                )
                                .unwrap();

                            let pk = key_pair.public_key();

                            let pk_comp = ring::rsa::PublicKeyComponents::<Vec<u8>>::from(pk);
                            (pk_comp, sig)
                        },
                        |(pk, sig)| {
                            pk.verify(&ring::signature::RSA_PSS_2048_8192_SHA256, MSG, &sig)
                                .unwrap()
                        },
                        BatchSize::SmallInput,
                    )
                });
            }

            println!("  OpenSSL@{bits}...");

            group.bench_function("OpenSSL", |b| {
                b.iter_batched(
                    || {
                        let mut sig = [0; $bytes];
                        let sk = openssl::pkey::PKey::from_rsa(sk.clone()).unwrap();
                        let mut signer =
                            openssl::sign::Signer::new(openssl::hash::MessageDigest::sha256(), &sk)
                                .unwrap();
                        write!(&mut signer, "{MSG_STR}").unwrap();
                        signer.sign(&mut sig).unwrap();

                        (sk, sig)
                    },
                    |(sk, sig)| {
                        let mut verifier = openssl::sign::Verifier::new(
                            openssl::hash::MessageDigest::sha256(),
                            sk.as_ref(),
                        )
                        .unwrap();
                        write!(&mut verifier, "{MSG_STR}").unwrap();
                        verifier.verify(&sig).unwrap();
                    },
                    BatchSize::SmallInput,
                )
            });
        }
    };
}

macro_rules! sign {
    ($fun_name:ident, $bench_name:literal, $bits:literal, $bytes:literal) => {
        fn $fun_name(c: &mut Criterion) {
            let bits = $bits;
            const BYTES: usize = $bytes;
            // Comparing libcrux performance for different payload sizes and other implementations.
            let mut group = c.benchmark_group($bench_name);

            let (der_, sk) = keygen(bits);
            let der = der_.as_slice();

            group.bench_function("libcrux", |b| {
                b.iter_batched(
                    || parse_der_libcrux(der, bits),
                    |sk| {
                        let mut sig = [0; BYTES];
                        libcrux_rsa::sign_varlen(
                            libcrux_rsa::DigestAlgorithm::Sha2_256,
                            &sk,
                            MSG,
                            SALT,
                            &mut sig,
                        )
                        .unwrap();
                    },
                    BatchSize::SmallInput,
                )
            });

            if bits <= 4096 {
                group.bench_function("RustCrypto", |b| {
                    b.iter_batched(
                        || rsa::RsaPrivateKey::from_pkcs1_der(&der).unwrap(),
                        |sk| {
                            sk.sign_with_rng(&mut OsRng, rsa::Pss::new::<rsa::sha2::Sha256>(), MSG)
                                .unwrap();
                        },
                        BatchSize::SmallInput,
                    )
                });

                group.bench_function("Ring", |b| {
                    b.iter_batched(
                        || ring::rsa::KeyPair::from_der(der).unwrap(),
                        |sk| {
                            let mut sig = [0; BYTES];
                            sk.sign(
                                &ring::signature::RSA_PSS_SHA256,
                                &ring::rand::SystemRandom::new(),
                                MSG,
                                &mut sig,
                            )
                            .unwrap();
                        },
                        BatchSize::SmallInput,
                    )
                });
            }

            group.bench_function("OpenSSL", |b| {
                b.iter_batched(
                    || openssl::pkey::PKey::from_rsa(sk.clone()).unwrap(),
                    |sk| {
                        let mut sigbuf = [0; BYTES];
                        let mut signer =
                            openssl::sign::Signer::new(openssl::hash::MessageDigest::sha256(), &sk)
                                .unwrap();
                        write!(&mut signer, "{MSG_STR}").unwrap();
                        signer.sign(&mut sigbuf).unwrap();
                    },
                    BatchSize::SmallInput,
                )
            });
        }
    };
}

verify!(verify_2048, "rsa/verify/2048", 2048, 256);
verify!(verify_3072, "rsa/verify/3072", 3072, 384);
verify!(verify_4096, "rsa/verify/4096", 4096, 512);
verify!(verify_6144, "rsa/verify/6144", 6144, 768);
verify!(verify_8192, "rsa/verify/8192", 8192, 1024);

sign!(sign_2048, "rsa/sign/2048", 2048, 256);
sign!(sign_3072, "rsa/sign/3072", 3072, 384);
sign!(sign_4096, "rsa/sign/4096", 4096, 512);
sign!(sign_6144, "rsa/sign/6144", 6144, 768);
sign!(sign_8192, "rsa/sign/8192", 8192, 1024);

criterion_group!(
    benches,
    sign_2048,
    sign_3072,
    sign_4096,
    sign_6144,
    sign_8192,
    verify_2048,
    verify_3072,
    verify_4096,
    verify_6144,
    verify_8192
);
//criterion_group!(benches, sign, verify_2048, verify_4096);
criterion_main!(benches);

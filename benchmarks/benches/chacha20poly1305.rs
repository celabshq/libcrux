use chacha20poly1305::{AeadCore, AeadInPlace, KeyInit};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

use libcrux_chacha20poly1305::*;

use benchmarks::util::*;
use rand::RngCore;
use ring::aead::UnboundKey;

fn randbuf<const LEN: usize>(drbg: &mut impl RngCore) -> [u8; LEN] {
    let mut buf = [0; LEN];
    drbg.fill_bytes(&mut buf);
    buf
}

// Comparing libcrux performance for different payload sizes and other implementations.
fn comparisons_encrypt(c: &mut Criterion) {
    const PAYLOAD_SIZES: [usize; 1] = [1024 * 1024 * 10];

    let mut drbg = rand::rng(); // not a crypto rng
    let mut group = c.benchmark_group("ChaCha20Poly1305 Encrypt");

    for payload_size in PAYLOAD_SIZES.iter() {
        group.throughput(Throughput::Bytes(*payload_size as u64));

        group.bench_with_input(
            BenchmarkId::new("libcrux", fmt(*payload_size)),
            payload_size,
            |b, payload_size| {
                b.iter_batched(
                    || {
                        let key = randbuf(&mut drbg);
                        let nonce = randbuf(&mut drbg);
                        let ptxt = randombytes(*payload_size);
                        let ctxt = vec![0; *payload_size];
                        let aad = randombytes(1_000);
                        (ptxt, ctxt, nonce, aad, key)
                    },
                    |(ptxt, mut ctxt, nonce, aad, key)| {
                        let _tag = encrypt(&key, &ptxt, &mut ctxt, &aad, &nonce);
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Ring", fmt(*payload_size)),
            payload_size,
            |b, payload_size| {
                b.iter_batched(
                    || {
                        let cipher = &ring::aead::CHACHA20_POLY1305;
                        let key = UnboundKey::new(cipher, &randombytes(32)).unwrap();
                        let key = ring::aead::LessSafeKey::new(key);
                        let data = randombytes(*payload_size);

                        let aad = ring::aead::Aad::from(randombytes(1_000));
                        let nonce =
                            ring::aead::Nonce::try_assume_unique_for_key(&randombytes(12)).unwrap();
                        (data, key, nonce, aad)
                    },
                    |(mut data, key, nonce, aad)| {
                        let _tag = key
                            .seal_in_place_separate_tag(nonce, aad, &mut data)
                            .unwrap();
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("RustCrypto", fmt(*payload_size)),
            payload_size,
            |b, payload_size| {
                use rand_core_old::OsRng;
                b.iter_batched(
                    || {
                        let mut rng = OsRng;
                        let key = chacha20poly1305::ChaCha20Poly1305::generate_key(&mut rng);
                        let cipher = chacha20poly1305::ChaCha20Poly1305::new(&key);
                        let nonce = chacha20poly1305::ChaCha20Poly1305::generate_nonce(&mut rng); // 96-bits; unique per message
                        let aad = randombytes(1_000);
                        let data = randombytes(*payload_size);
                        (data, cipher, nonce, aad)
                    },
                    |(mut data, cipher, nonce, aad)| {
                        cipher.encrypt_in_place(&nonce, &aad, &mut data).unwrap();
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        #[cfg(all(not(windows), not(target_arch = "wasm32"), not(target_arch = "x86")))]
        group.bench_with_input(
            BenchmarkId::new("OpenSSL", fmt(*payload_size)),
            payload_size,
            |b, payload_size| {
                use openssl::symm::{encrypt_aead, Cipher};

                b.iter_batched(
                    || {
                        let cipher = Cipher::chacha20_poly1305();
                        let key = randombytes(32);
                        let nonce = randombytes(12);
                        let aad = randombytes(1_000);
                        let data = randombytes(*payload_size);

                        (data, cipher, key, nonce, aad)
                    },
                    |(data, cipher, key, nonce, aad)| {
                        let mut tag = [0u8; 16];
                        let _ciphertext =
                            encrypt_aead(cipher, &key, Some(&nonce), &aad, &data, &mut tag);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

// Comparing libcrux performance for different payload sizes and other implementations.
fn comparisons_decrypt(c: &mut Criterion) {
    const PAYLOAD_SIZES: [usize; 1] = [1024 * 1024 * 10];

    let mut drbg = rand::rng(); // not a crypto rng
    let mut group = c.benchmark_group("ChaCha20Poly1305 Decrypt");

    for payload_size in PAYLOAD_SIZES.iter() {
        group.throughput(Throughput::Bytes(*payload_size as u64));

        group.bench_with_input(
            BenchmarkId::new("libcrux", fmt(*payload_size)),
            payload_size,
            |b, payload_size| {
                let payload_size = *payload_size;

                b.iter_batched(
                    || {
                        let key = randbuf(&mut drbg);
                        let nonce_enc = randbuf(&mut drbg);
                        let nonce = nonce_enc;
                        let ptxt = randombytes(payload_size);
                        let mut ctxt = vec![0; payload_size + TAG_LEN];
                        let aad = randombytes(1_000);

                        let (ctxt_got, tag) =
                            encrypt(&key, &ptxt, &mut ctxt, &aad, &nonce).unwrap();
                        assert_eq!(payload_size, ctxt_got.len());
                        assert_eq!(TAG_LEN, tag.len());

                        (key, nonce, ptxt, ctxt, aad)
                    },
                    |(key, nonce, mut ptxt, ctxt, aad)| {
                        let _r = decrypt(&key, &mut ptxt, &ctxt, &aad, &nonce);
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Ring", fmt(*payload_size)),
            payload_size,
            |b, payload_size| {
                b.iter_batched(
                    || {
                        let cipher = &ring::aead::CHACHA20_POLY1305;
                        let key = UnboundKey::new(cipher, &randombytes(32)).unwrap();
                        let key = ring::aead::LessSafeKey::new(key);
                        let mut data = randombytes(*payload_size);

                        let a = randombytes(1_000);
                        let aad = ring::aead::Aad::from(a.clone());
                        let n = randombytes(12);
                        let nonce = ring::aead::Nonce::try_assume_unique_for_key(&n).unwrap();

                        let tag = key
                            .seal_in_place_separate_tag(nonce, aad, &mut data)
                            .unwrap();
                        data.extend_from_slice(tag.as_ref());
                        (
                            data,
                            key,
                            ring::aead::Nonce::try_assume_unique_for_key(&n).unwrap(),
                            ring::aead::Aad::from(a),
                        )
                    },
                    |(mut data, key, nonce, aad)| {
                        let _r = key.open_in_place(nonce, aad, &mut data).unwrap();
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("RustCrypto", fmt(*payload_size)),
            payload_size,
            |b, payload_size| {
                // Using older version of traits as required by library
                use rand_core_old::OsRng;
                b.iter_batched(
                    || {
                        let key = chacha20poly1305::ChaCha20Poly1305::generate_key(&mut OsRng);
                        let cipher = chacha20poly1305::ChaCha20Poly1305::new(&key);
                        let nonce = chacha20poly1305::ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
                        let aad = randombytes(1_000);
                        let mut data = randombytes(*payload_size);

                        cipher.encrypt_in_place(&nonce, &aad, &mut data).unwrap();
                        (data, cipher, nonce, aad)
                    },
                    |(mut data, cipher, nonce, aad)| {
                        cipher.decrypt_in_place(&nonce, &aad, &mut data).unwrap();
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        #[cfg(all(not(windows), not(target_arch = "wasm32"), not(target_arch = "x86")))]
        group.bench_with_input(
            BenchmarkId::new("OpenSSL", fmt(*payload_size)),
            payload_size,
            |b, payload_size| {
                use openssl::symm::{decrypt_aead, encrypt_aead, Cipher};

                b.iter_batched(
                    || {
                        let cipher = Cipher::chacha20_poly1305();
                        let key = randombytes(32);
                        let nonce = randombytes(12);
                        let aad = randombytes(1_000);
                        let data = randombytes(*payload_size);

                        let mut tag = [0u8; 16];
                        let ciphertext =
                            encrypt_aead(cipher, &key, Some(&nonce), &aad, &data, &mut tag)
                                .unwrap();

                        (ciphertext, cipher, key, nonce, aad, tag)
                    },
                    |(ciphertext, cipher, key, nonce, aad, tag)| {
                        let _r = decrypt_aead(cipher, &key, Some(&nonce), &aad, &ciphertext, &tag)
                            .unwrap();
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

fn benchmarks(c: &mut Criterion) {
    comparisons_encrypt(c);
    comparisons_decrypt(c)
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);

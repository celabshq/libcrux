#![allow(non_snake_case)]
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

pub fn randombytes<const LEN: usize>() -> [u8; LEN] {
    let mut bytes = [0u8; LEN];
    rand::rng().fill_bytes(&mut bytes);
    bytes
}

pub fn randombytes_vec(n: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; n];
    rand::rng().fill_bytes(&mut bytes);
    bytes
}

pub fn fmt(x: usize) -> String {
    let base = (x as f64).log(1024f64).floor() as usize;
    let suffix = ["", "KB", "MB", "GB"];
    format!("{} {}", x >> (10 * base), suffix[base])
}

macro_rules! impl_comp {
    ($fun:ident, $keylen:expr, $portable:ident, $neon:ident, $intel:ident, $rustcrypto_fun:expr) => {
        // Comparing libcrux performance for different payload sizes and other implementations.
        fn $fun(c: &mut Criterion) {
            const PAYLOAD_SIZES: [usize; 3] = [128, 1024, 1024 * 1024 * 10];

            let mut group = c.benchmark_group(stringify!($fun).replace("_", " "));

            for payload_size in PAYLOAD_SIZES.iter() {
                group.throughput(Throughput::Bytes(*payload_size as u64));

                group.bench_with_input(
                    BenchmarkId::new("libcrux", fmt(*payload_size)),
                    payload_size,
                    |b, payload_size| {
                        b.iter_batched(
                            || {
                                (
                                    randombytes::<$keylen>(),
                                    randombytes::<12>(),
                                    randombytes::<32>(),
                                    randombytes_vec(*payload_size),
                                )
                            },
                            |(key, nonce, aad, payload)| {
                                let mut ciphertext = vec![0; *payload_size];
                                use libcrux_aes::$fun::portable::{$portable, Key, Nonce, Tag};

                                let k: Key<$portable> = key.into();
                                let nonce: Nonce<$portable> = nonce.into();
                                let mut tag: Tag<$portable> = [0; libcrux_aes::TAG_LEN].into();

                                k.encrypt(&mut ciphertext, &mut tag, &nonce, &aad, &payload)
                                    .unwrap();
                            },
                            BatchSize::SmallInput,
                        )
                    },
                );

                #[cfg(all(target_arch = "aarch64", target_feature = "aes"))]
                group.bench_with_input(
                    BenchmarkId::new("neon-aes-clmul", fmt(*payload_size)),
                    payload_size,
                    |b, payload_size| {
                        b.iter_batched(
                            || {
                                (
                                    randombytes::<$keylen>(),
                                    randombytes::<12>(),
                                    randombytes::<32>(),
                                    randombytes_vec(*payload_size),
                                )
                            },
                            |(key, nonce, aad, payload)| {
                                let mut ciphertext = vec![0; *payload_size];
                                use libcrux_aes::$fun::neon::{$neon, Key, Nonce, Tag};

                                let k: Key<$neon> = key.into();
                                let nonce: Nonce<$neon> = nonce.into();
                                let mut tag: Tag<$neon> = [0; libcrux_aes::TAG_LEN].into();

                                k.encrypt(&mut ciphertext, &mut tag, &nonce, &aad, &payload)
                                    .unwrap();
                            },
                            BatchSize::SmallInput,
                        )
                    },
                );

                #[cfg(all(target_arch = "x86_64"))] // ENABLE: target_feature="aes"
                group.bench_with_input(
                    BenchmarkId::new("intel-aes-clmul", fmt(*payload_size)),
                    payload_size,
                    |b, payload_size| {
                        b.iter_batched(
                            || {
                                (
                                    randombytes::<$keylen>(),
                                    randombytes::<12>(),
                                    randombytes::<32>(),
                                    randombytes_vec(*payload_size),
                                )
                            },
                            |(key, nonce, aad, payload)| {
                                let mut ciphertext = vec![0; *payload_size];
                                use libcrux_aes::$fun::x64::{$intel, Key, Nonce, Tag};

                                let k: Key<$intel> = key.into();
                                let nonce: Nonce<$intel> = nonce.into();
                                let mut tag: Tag<$intel> = [0; libcrux_aes::TAG_LEN].into();

                                k.encrypt(&mut ciphertext, &mut tag, &nonce, &aad, &payload)
                                    .unwrap();
                            },
                            BatchSize::SmallInput,
                        )
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new("rust-crypto", fmt(*payload_size)),
                    payload_size,
                    |b, payload_size| {
                        b.iter_batched(
                            || {
                                (
                                    randombytes::<$keylen>(),
                                    randombytes::<12>(),
                                    randombytes::<32>(),
                                    randombytes_vec(*payload_size),
                                )
                            },
                            |(key, nonce, aad, payload)| {
                                let mut ciphertext = vec![0; *payload_size];
                                let mut tag = [0u8; 16];
                                $rustcrypto_fun(
                                    key.as_slice(),
                                    nonce.as_slice(),
                                    &aad,
                                    &payload,
                                    &mut ciphertext,
                                    &mut tag,
                                );
                            },
                            BatchSize::SmallInput,
                        )
                    },
                );
            }
        }
    };
}

use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes128Gcm, Aes256Gcm,
};
use rand::RngCore;

fn rustcrypto_aes128_gcm_encrypt(
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    msg: &[u8],
    ciphertext: &mut [u8],
    tag: &mut [u8],
) {
    let cipher = Aes128Gcm::new(key.into());
    let ctxt = cipher.encrypt(nonce.into(), Payload { msg, aad }).unwrap();
    ciphertext.copy_from_slice(&ctxt[0..msg.len()]);
    tag.copy_from_slice(&ctxt[msg.len()..]);
}

// XXX: We could work with the traits here, but this is quicker for now.
fn rustcrypto_aes256_gcm_encrypt(
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    msg: &[u8],
    ciphertext: &mut [u8],
    tag: &mut [u8],
) {
    let cipher = Aes256Gcm::new(key.into());
    let ctxt = cipher.encrypt(nonce.into(), Payload { msg, aad }).unwrap();
    ciphertext.copy_from_slice(&ctxt[0..msg.len()]);
    tag.copy_from_slice(&ctxt[msg.len()..]);
}

impl_comp!(
    aes_gcm_128,
    16,
    PortableAesGcm128,
    NeonAesGcm128,
    X64AesGcm128,
    rustcrypto_aes128_gcm_encrypt
);
impl_comp!(
    aes_gcm_256,
    32,
    PortableAesGcm256,
    NeonAesGcm256,
    X64AesGcm256,
    rustcrypto_aes256_gcm_encrypt
);

fn benchmarks(c: &mut Criterion) {
    aes_gcm_128(c);
    aes_gcm_256(c);
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);

#![allow(non_snake_case)]
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};

use benchmarks::util::*;

macro_rules! impl_comp {
    ($fun:ident, $libcrux:expr, $ring:expr, $rust_crypto:ty, $openssl:expr) => {
        // Comparing libcrux performance for different payload sizes and other implementations.
        fn $fun(c: &mut Criterion) {
            const PAYLOAD_SIZES: [usize; 5] = [100, 1024, 2048, 4096, 8192];

            let mut group = c.benchmark_group(stringify!($fun).replace("_", " "));

            for payload_size in PAYLOAD_SIZES.iter() {
                group.throughput(Throughput::Bytes(*payload_size as u64));

                group.bench_with_input(
                    BenchmarkId::new("libcrux", fmt(*payload_size)),
                    payload_size,
                    |b, payload_size| {
                        b.iter_batched(
                            || randombytes(*payload_size),
                            |payload| $libcrux(&payload),
                            BatchSize::SmallInput,
                        )
                    },
                );

                if let Some(ring) = $ring {
                    group.bench_with_input(
                        BenchmarkId::new("Ring", fmt(*payload_size)),
                        payload_size,
                        |b, payload_size| {
                            b.iter_batched(
                                || randombytes(*payload_size),
                                |payload| ring::digest::digest(ring, &payload),
                                BatchSize::SmallInput,
                            )
                        },
                    );
                }

                group.bench_with_input(
                    BenchmarkId::new("RustCrypto", fmt(*payload_size)),
                    payload_size,
                    |b, payload_size| {
                        use sha2::Digest;

                        b.iter_batched(
                            || randombytes(*payload_size),
                            |payload| {
                                let mut hasher = <$rust_crypto>::new();
                                hasher.update(&payload);
                                hasher.finalize()
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
                        use openssl::hash::*;

                        b.iter_batched(
                            || randombytes(*payload_size),
                            |payload| hash($openssl, &payload),
                            BatchSize::SmallInput,
                        )
                    },
                );
            }
        }
    };
}

impl_comp!(
    Sha2_224,
    libcrux_sha2::sha224,
    None,
    sha2::Sha224,
    MessageDigest::sha224()
);
impl_comp!(
    Sha2_256,
    libcrux_sha2::sha256,
    Some(&ring::digest::SHA256),
    sha2::Sha256,
    MessageDigest::sha256()
);
impl_comp!(
    Sha2_384,
    libcrux_sha2::sha384,
    Some(&ring::digest::SHA384),
    sha2::Sha384,
    MessageDigest::sha384()
);
impl_comp!(
    Sha2_512,
    libcrux_sha2::sha512,
    Some(&ring::digest::SHA512),
    sha2::Sha512,
    MessageDigest::sha512()
);

fn benchmarks(c: &mut Criterion) {
    Sha2_224(c);
    Sha2_256(c);
    Sha2_384(c);
    Sha2_512(c);
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);

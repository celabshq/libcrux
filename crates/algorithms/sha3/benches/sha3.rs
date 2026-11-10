#![allow(non_snake_case)]
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use libcrux_sha3::{portable::incremental::Xof, *};

pub fn randombytes(n: usize) -> Vec<u8> {
    use rand::{rngs::SysRng, TryRng};

    let mut bytes = vec![0u8; n];
    SysRng.try_fill_bytes(&mut bytes).unwrap();
    bytes
}

pub fn fmt(x: usize) -> String {
    let base = (x as f64).log(1024f64).floor() as usize;
    let suffix = ["", "KB", "MB", "GB"];
    format!("{} {}", x >> (10 * base), suffix[base])
}

macro_rules! impl_comp {
    ($fun:ident, $libcrux:expr, $rust_fun:ident) => {
        // Comparing libcrux performance for different payload sizes and other implementations.
        fn $fun(c: &mut Criterion) {
            const PAYLOAD_SIZES: [usize; 3] = [128, 1024, 1024 * 1024 * 10];

            let mut group = c.benchmark_group(stringify!($fun).replace("_", " "));

            for payload_size in PAYLOAD_SIZES.iter() {
                group.throughput(Throughput::Bytes(*payload_size as u64));

                group.bench_with_input(
                    BenchmarkId::new("portable", fmt(*payload_size)),
                    payload_size,
                    |b, payload_size| {
                        b.iter_batched_ref(
                            || randombytes(*payload_size),
                            |payload| {
                                let _d: [u8; digest_size($libcrux)] =
                                    core::hint::black_box(hash($libcrux, payload));
                            },
                            BatchSize::SmallInput,
                        )
                    },
                );

                #[cfg(all(feature = "simd128", target_arch = "aarch64"))]
                group.bench_with_input(
                    BenchmarkId::new("simd128", fmt(*payload_size)),
                    payload_size,
                    |b, payload_size| {
                        b.iter_batched_ref(
                            || randombytes(*payload_size),
                            |payload| {
                                let mut digest = [0u8; digest_size($libcrux)];
                                neon::$rust_fun(&mut digest, payload);
                                core::hint::black_box(digest);
                            },
                            BatchSize::SmallInput,
                        )
                    },
                );
            }
        }
    };
}

impl_comp!(Sha3_224, Algorithm::Sha224, sha224);
impl_comp!(Sha3_256, Algorithm::Sha256, sha256);
impl_comp!(Sha3_384, Algorithm::Sha384, sha384);
impl_comp!(Sha3_512, Algorithm::Sha512, sha512);

fn shake256x4(c: &mut Criterion) {
    let mut group = c.benchmark_group("shake256/x4");

    #[cfg(all(feature = "simd256", target_arch = "x86_64"))]
    group.bench_function("avx2", |b| {
        b.iter_batched_ref(
            || {
                (
                    randombytes(33),
                    randombytes(33),
                    randombytes(33),
                    randombytes(33),
                )
            },
            |(payload0, payload1, payload2, payload3)| {
                let mut digest0 = [0u8; 128];
                let mut digest1 = [0u8; 128];
                let mut digest2 = [0u8; 128];
                let mut digest3 = [0u8; 128];
                avx2::x4::shake256(
                    payload0,
                    payload1,
                    payload2,
                    payload3,
                    &mut digest0,
                    &mut digest1,
                    &mut digest2,
                    &mut digest3,
                );
                core::hint::black_box((digest0, digest1, digest2, digest3));
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("portable", |b| {
        b.iter_batched_ref(
            || {
                (
                    randombytes(33),
                    randombytes(33),
                    randombytes(33),
                    randombytes(33),
                )
            },
            |(payload0, payload1, payload2, payload3)| {
                let mut digest0 = [0u8; 128];
                let mut digest1 = [0u8; 128];
                let mut digest2 = [0u8; 128];
                let mut digest3 = [0u8; 128];
                portable::shake256(&mut digest0, payload0);
                portable::shake256(&mut digest1, payload1);
                portable::shake256(&mut digest2, payload2);
                portable::shake256(&mut digest3, payload3);
                core::hint::black_box((digest0, digest1, digest2, digest3));
            },
            BatchSize::SmallInput,
        )
    });
}

fn shake256(c: &mut Criterion) {
    let mut group = c.benchmark_group("shake256");

    group.bench_function("portable", |b| {
        b.iter_batched_ref(
            || randombytes(33),
            |payload| {
                let mut digest = [0u8; 128];
                portable::shake256(&mut digest, payload);
                core::hint::black_box(digest);
            },
            BatchSize::SmallInput,
        )
    });
}

fn xof(c: &mut Criterion) {
    let mut group = c.benchmark_group("shake256/xof");

    group.bench_function("portable", |b| {
        b.iter_batched_ref(
            || randombytes(33),
            |payload| {
                let mut digest = [0u8; 128];
                let mut shake = portable::incremental::Shake256Xof::new();
                shake.absorb(payload);
                shake.absorb_final(&[]);
                shake.squeeze(&mut digest);
                core::hint::black_box(digest);
            },
            BatchSize::SmallInput,
        )
    });
}

fn benchmarks(c: &mut Criterion) {
    shake256x4(c);
    Sha3_224(c);
    Sha3_256(c);
    Sha3_384(c);
    Sha3_512(c);
    xof(c);
    shake256(c);
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);

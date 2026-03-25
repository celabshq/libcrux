use std::time::Duration;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use libcrux::primitives::{kem, kem::Algorithm};
use rand_core::{OsRng, RngCore, TryRngCore};

pub fn comparisons_key_generation(c: &mut Criterion) {
    let mut os_rng = OsRng;
    let mut rng = os_rng.unwrap_mut();
    let mut group = c.benchmark_group("Kyber768 Key Generation");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("libcrux portable (external random)", |b| {
        let mut seed = [0; 64];
        rng.fill_bytes(&mut seed);
        b.iter(|| {
            let _kp = libcrux_kem::deterministic::mlkem768_generate_keypair_derand(seed);
        })
    });

    // FIXME: This has not yet landed in `libcrux-ml-kem`
    // group.bench_function("libcrux portable unpacked (external random)", |b| {
    //     b.iter(|| {
    //         let mut seed = [0; 64];
    //         rng.try_fill_bytes(&mut seed).unwrap();
    //         let _tuple = libcrux_ml_kem::mlkem768::generate_key_pair_unpacked(seed);
    //     })
    // });

    group.bench_function("libcrux portable (HACL-DRBG)", |b| {
        b.iter(|| {
            let (_secret_key, _public_key) = kem::key_gen(Algorithm::MlKem768, &mut rng).unwrap();
        })
    });

    group.bench_function("libcrux portable (OsRng)", |b| {
        b.iter(|| {
            let (_secret_key, _public_key) = kem::key_gen(Algorithm::MlKem768, &mut rng).unwrap();
        })
    });

    group.bench_function("pqclean reference implementation", |b| {
        b.iter(|| {
            let (_public_key, _secret_key) = pqcrypto_kyber::kyber768::keypair();
        })
    });

    #[cfg(all(not(target_os = "windows"), target_arch = "x86_64"))]
    group.bench_function("libjade kyber avx2", |b| {
        b.iter(|| {
            let mut seed = [0; 64];
            rng.try_fill_bytes(&mut seed).unwrap();
            let mut public_key = [0u8; 1184];
            let mut secret_key = [0u8; 2400];
            unsafe {
                libjade_sys::jade_kem_kyber_kyber768_amd64_avx2_keypair_derand(
                    public_key.as_mut_ptr(),
                    secret_key.as_mut_ptr(),
                    seed.as_ptr(),
                )
            };
        })
    });
}

pub fn comparisons_pk_validation(c: &mut Criterion) {
    let mut rng = OsRng;
    let mut group = c.benchmark_group("Kyber768 PK Validation");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("libcrux portable", |b| {
        let mut seed = [0; 64];
        rng.try_fill_bytes(&mut seed).unwrap();
        b.iter_batched(
            || libcrux_kem::deterministic::mlkem768_generate_keypair_derand(seed),
            |key_pair| {
                let _valid = libcrux_kem::ml_kem768_validate_public_key(&key_pair.into_parts().1);
            },
            BatchSize::SmallInput,
        )
    });
}

pub fn comparisons_encapsulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Kyber768 Encapsulation");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("libcrux portable (external random)", |b| {
        let mut seed1 = [0; 64];
        OsRng.try_fill_bytes(&mut seed1).unwrap();
        let mut seed2 = [0; 32];
        OsRng.try_fill_bytes(&mut seed2).unwrap();
        b.iter_batched(
            || libcrux_kem::deterministic::mlkem768_generate_keypair_derand(seed1),
            |keypair| {
                let (_shared_secret, _ciphertext) =
                    libcrux_kem::deterministic::mlkem768_encapsulate_derand(
                        &keypair.public_key(),
                        seed2,
                    );
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("libcrux portable", |b| {
        b.iter_batched(
            || {
                let mut os_rng = OsRng;
                let mut rng = os_rng.unwrap_mut();
                let (_secret_key, public_key) =
                    libcrux_kem::key_gen(Algorithm::MlKem768, &mut rng).unwrap();

                (os_rng, public_key)
            },
            |(mut os_rng, public_key)| {
                let mut rng = os_rng.unwrap_mut();
                let (_shared_secret, _ciphertext) = public_key.encapsulate(&mut rng).unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("libcrux portable OsRng", |b| {
        b.iter_batched(
            || {
                let mut os_rng = OsRng;
                let mut drbg = os_rng.unwrap_mut();
                let (_secret_key, public_key) =
                    libcrux_kem::key_gen(Algorithm::MlKem768, &mut drbg).unwrap();

                public_key
            },
            |public_key| {
                let mut os_rng = OsRng;
                let mut rng = os_rng.unwrap_mut();
                let (_shared_secret, _ciphertext) = public_key.encapsulate(&mut rng).unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("pqclean reference implementation", |b| {
        b.iter_batched(
            || {
                let (public_key, _secret_key) = pqcrypto_kyber::kyber768::keypair();

                public_key
            },
            |public_key| {
                let (_shared_secret, _ciphertext) =
                    pqcrypto_kyber::kyber768::encapsulate(&public_key);
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(not(target_os = "windows"), target_arch = "x86_64"))]
    group.bench_function("libjade kyber avx2", |b| {
        b.iter_batched(
            || {
                let mut rng = OsRng;
                let mut seed = [0; 64];
                rng.try_fill_bytes(&mut seed).unwrap();
                let mut public_key = [0u8; 1184];
                let mut secret_key = [0u8; 2400];
                unsafe {
                    libjade_sys::jade_kem_kyber_kyber768_amd64_avx2_keypair_derand(
                        public_key.as_mut_ptr(),
                        secret_key.as_mut_ptr(),
                        seed.as_ptr(),
                    )
                };

                (rng, public_key)
            },
            |(mut rng, public_key)| {
                let mut seed = [0; 32];
                rng.try_fill_bytes(&mut seed).unwrap();

                let mut ciphertext = [0u8; 1088];
                let mut shared_secret = [0u8; 32];

                unsafe {
                    libjade_sys::jade_kem_kyber_kyber768_amd64_avx2_enc_derand(
                        ciphertext.as_mut_ptr(),
                        shared_secret.as_mut_ptr(),
                        public_key.as_ptr(),
                        seed.as_ptr(),
                    );
                }
            },
            BatchSize::SmallInput,
        )
    });
}

pub fn comparisons_decapsulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Kyber768 Decapsulation");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("libcrux portable", |b| {
        b.iter_batched(
            || {
                let mut os_rng = OsRng;
                let mut rng = os_rng.unwrap_mut();
                let (secret_key, public_key) =
                    libcrux_kem::key_gen(Algorithm::MlKem768, &mut rng).unwrap();
                let (_shared_secret, ciphertext) = public_key.encapsulate(&mut rng).unwrap();
                (secret_key, ciphertext)
            },
            |(secret_key, ciphertext)| {
                let _shared_secret = ciphertext.decapsulate(&secret_key);
            },
            BatchSize::SmallInput,
        )
    });

    // FIXME: This has not yet landed in `libcrux-ml-kem`.
    // group.bench_function("libcrux portable unpacked", |b| {
    //     b.iter_batched(
    //         || {
    //             let mut seed = [0; 64];
    //             OsRng.try_fill_bytes(&mut seed).unwrap();
    //             let (sk_state, pubkey) = libcrux_ml_kem::mlkem768::generate_key_pair_unpacked(seed);

    //             let mut rand = [0; 32];
    //             OsRng.try_fill_bytes(&mut rand).unwrap();
    //             let (ciphertext, _) = libcrux_ml_kem::mlkem768::encapsulate(&pubkey, rand);
    //             (sk_state, ciphertext)
    //         },
    //         |(sk_state, ciphertext)| {
    //             let _shared_secret =
    //                 libcrux_ml_kem::mlkem768::decapsulate_unpacked(&sk_state, &ciphertext);
    //         },
    //         BatchSize::SmallInput,
    //     )
    // });

    group.bench_function("pqclean reference implementation", |b| {
        b.iter_batched(
            || {
                let (public_key, secret_key) = pqcrypto_kyber::kyber768::keypair();
                let (_shared_secret, ciphertext) =
                    pqcrypto_kyber::kyber768::encapsulate(&public_key);

                (ciphertext, secret_key)
            },
            |(ciphertext, secret_key)| {
                let _shared_secret =
                    pqcrypto_kyber::kyber768::decapsulate(&ciphertext, &secret_key);
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(not(target_os = "windows"), target_arch = "x86_64"))]
    group.bench_function("libjade kyber avx2", |b| {
        b.iter_batched(
            || {
                let mut rng = OsRng;
                let mut seed = [0; 64];
                rng.try_fill_bytes(&mut seed).unwrap();
                let mut public_key = [0u8; 1184];
                let mut secret_key = [0u8; 2400];
                unsafe {
                    libjade_sys::jade_kem_kyber_kyber768_amd64_avx2_keypair_derand(
                        public_key.as_mut_ptr(),
                        secret_key.as_mut_ptr(),
                        seed.as_ptr(),
                    )
                };

                let mut seed = [0; 32];
                rng.try_fill_bytes(&mut seed).unwrap();

                let mut ciphertext = [0u8; 1088];
                let mut shared_secret = [0u8; 32];

                unsafe {
                    libjade_sys::jade_kem_kyber_kyber768_amd64_avx2_enc_derand(
                        ciphertext.as_mut_ptr(),
                        shared_secret.as_mut_ptr(),
                        public_key.as_ptr(),
                        seed.as_ptr(),
                    );
                }

                (secret_key, ciphertext)
            },
            |(secret_key, ciphertext)| {
                let mut shared_secret = [0u8; 32];
                unsafe {
                    libjade_sys::jade_kem_kyber_kyber768_amd64_avx2_dec(
                        shared_secret.as_mut_ptr(),
                        ciphertext.as_ptr(),
                        secret_key.as_ptr(),
                    );
                }
            },
            BatchSize::SmallInput,
        )
    });
}

pub fn comparisons(c: &mut Criterion) {
    comparisons_pk_validation(c);
    comparisons_key_generation(c);
    comparisons_encapsulation(c);
    comparisons_decapsulation(c);
}

criterion_group!(benches, comparisons);
criterion_main!(benches);

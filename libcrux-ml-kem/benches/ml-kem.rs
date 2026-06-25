use std::hint::black_box;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::{rngs::OsRng, RngCore, TryRngCore};

use libcrux_ml_kem::{mlkem1024, mlkem512, mlkem768};

macro_rules! init {
    ($version:path, $bench:expr, $c:expr) => {{
        let version = stringify!($version);
        let key_size = version
            .strip_prefix("mlkem")
            .expect("crate name does not begin with 'mlkem'");
        let mut group = $c.benchmark_group(format!(
            "category=ML-KEM,keySize={},name={}",
            key_size, $bench
        ));
        group.measurement_time(Duration::from_secs(10));

        use $version as version;

        fun!("portable", version::portable, group);
        fun_unpacked!("portable", version::portable::unpacked, group);

        #[cfg(feature = "incremental")]
        fun_incremental!("multiplexing", version::incremental, group);

        #[cfg(feature = "simd128")]
        {
            fun!("neon", version::neon, group);
            fun_unpacked!("neon", version::neon::unpacked, group);
        }

        #[cfg(feature = "simd256")]
        {
            fun!("avx2", version::avx2, group);
            fun_unpacked!("avx2", version::avx2::unpacked, group);
        }
    }};
}

pub fn key_generation(c: &mut Criterion) {
    let mut rng = OsRng;

    macro_rules! fun {
        ($name:expr, $p:path, $group:expr) => {
            $group.bench_function(format!("platform={},api=external random", $name), |b| {
                use $p as p;

                let mut seed = [0; 64];
                rng.try_fill_bytes(&mut seed).unwrap();
                b.iter(|| {
                    let _kp = core::hint::black_box(p::generate_key_pair(seed));
                })
            });
        };
    }

    macro_rules! fun_unpacked {
        ($name:expr, $p:path, $group:expr) => {
            $group.bench_function(
                format!("platform={},api=unpacked (external random)", $name),
                |b| {
                    use $p as p;
                    let mut seed = [0; 64];
                    rng.try_fill_bytes(&mut seed).unwrap();
                    b.iter(|| {
                        let mut kp = p::init_key_pair();
                        p::generate_key_pair_mut(seed, &mut kp);
                    })
                },
            );
        };
    }

    init!(mlkem512, "Key Generation", c);
    init!(mlkem768, "Key Generation", c);
    init!(mlkem1024, "Key Generation", c);
}

pub fn pk_validation(c: &mut Criterion) {
    let mut rng = OsRng;

    macro_rules! fun {
        ($name:expr, $p:path, $group:expr) => {
            $group.bench_function(format!("platform={}", $name), |b| {
                use $p as p;

                let mut seed = [0; 64];
                rng.try_fill_bytes(&mut seed).unwrap();
                b.iter_batched(
                    || {
                        let keypair = p::generate_key_pair(seed);
                        keypair
                    },
                    |keypair| {
                        let _valid = black_box(p::validate_public_key(keypair.public_key()));
                    },
                    BatchSize::SmallInput,
                )
            });
        };
    }

    macro_rules! fun_unpacked {
        ($name:expr, $p:path, $group:expr) => {
            // We don't do anything here.
        };
    }

    #[cfg(feature = "incremental")]
    macro_rules! fun_incremental {
        ($name:expr, $p:path, $group:expr) => {
            $group.bench_function(format!("platform={},api=incremental", $name), |b| {
                use $p::*;

                b.iter_batched(
                    || {
                        let mut rng = OsRng;
                        KeyPairCompressedBytes::generate(&mut rng)
                    },
                    |keypair| {
                        let _valid = black_box(validate_pk_bytes(keypair.pk1(), keypair.pk2()));
                    },
                    BatchSize::SmallInput,
                )
            });
        };
    }

    init!(mlkem512, "PK Validation", c);
    init!(mlkem768, "PK Validation", c);
    init!(mlkem1024, "PK Validation", c);
}

pub fn encapsulation(c: &mut Criterion) {
    macro_rules! fun {
        ($name:expr, $p:path, $group:expr) => {
            $group.bench_function(format!("platform={},api=external random", $name), |b| {
                use $p as p;
                let mut seed1 = [0; 64];
                OsRng.try_fill_bytes(&mut seed1).unwrap();
                let mut seed2 = [0; 32];
                OsRng.try_fill_bytes(&mut seed2).unwrap();
                b.iter_batched(
                    || p::generate_key_pair(seed1),
                    |keypair| {
                        let (_shared_secret, _ciphertext) =
                            black_box(p::encapsulate(keypair.public_key(), seed2));
                    },
                    BatchSize::SmallInput,
                )
            });
        };
    }

    macro_rules! fun_unpacked {
        ($name:expr, $p:path, $group:expr) => {
            $group.bench_function(
                format!("platform={},api=unpacked (external random)", $name),
                |b| {
                    use $p as p;

                    let mut seed1 = [0; 64];
                    ::rand::rng().fill_bytes(&mut seed1);
                    let mut seed2 = [0; 32];
                    ::rand::rng().fill_bytes(&mut seed2);

                    b.iter_batched(
                        || {
                            let mut kp = p::init_key_pair();
                            p::generate_key_pair_mut(seed1, &mut kp);
                            kp
                        },
                        |keypair| {
                            let (_shared_secret, _ciphertext) =
                                black_box(p::encapsulate(&keypair.public_key, seed2));
                        },
                        BatchSize::SmallInput,
                    )
                },
            );
        };
    }

    #[cfg(feature = "incremental")]
    macro_rules! fun_incremental {
        ($name:expr, $p:path, $group:expr) => {
            $group.bench_function(
                format!("platform={},api=incremental (external random)", $name),
                |b| {
                    use $p::*;

                    let mut seed1 = [0; 64];
                    OsRng.fill_bytes(&mut seed1);
                    let mut seed2 = [0; 32];
                    OsRng.fill_bytes(&mut seed2);

                    b.iter_batched(
                        || KeyPairCompressedBytes::from_seed(seed1),
                        |keypair| {
                            let mut encaps_state = [0u8; encaps_state_len()];
                            let mut encaps_shared_secret = [0u8; shared_secret_size()];

                            let _ct1 = encapsulate1(
                                keypair.pk1(),
                                seed2,
                                &mut encaps_state,
                                &mut encaps_shared_secret,
                            )
                            .unwrap();

                            let _ct2 = encapsulate2(&encaps_state, keypair.pk2());
                        },
                        BatchSize::SmallInput,
                    )
                },
            );

            $group.bench_function(format!("platform={},api=incremental", $name), |b| {
                use $p::*;

                b.iter_batched(
                    || {
                        let mut rng = OsRng;
                        KeyPairCompressedBytes::generate(&mut rng)
                    },
                    |keypair| {
                        let mut rng = OsRng;
                        let mut encaps_state = [0u8; encaps_state_len()];
                        let mut encaps_shared_secret = [0u8; shared_secret_size()];

                        let _ct1 = rand::encapsulate1(
                            keypair.pk1(),
                            &mut rng,
                            &mut encaps_state,
                            &mut encaps_shared_secret,
                        )
                        .unwrap();

                        let _ct2 = encapsulate2(&encaps_state, keypair.pk2());
                    },
                    BatchSize::SmallInput,
                )
            });
        };
    }

    init!(mlkem512, "Encapsulation", c);
    init!(mlkem768, "Encapsulation", c);
    init!(mlkem1024, "Encapsulation", c);
}

pub fn decapsulation(c: &mut Criterion) {
    macro_rules! fun {
        ($name:expr, $p:path, $group:expr) => {
            $group.bench_function(format!("platform={},", $name), |b| {
                use $p as p;
                let mut seed1 = [0; 64];
                OsRng.try_fill_bytes(&mut seed1).unwrap();
                let mut seed2 = [0; 32];
                OsRng.try_fill_bytes(&mut seed2).unwrap();
                b.iter_batched(
                    || {
                        let keypair = p::generate_key_pair(seed1);
                        let (ciphertext, _shared_secret) =
                            p::encapsulate(keypair.public_key(), seed2);
                        (keypair, ciphertext)
                    },
                    |(keypair, ciphertext)| {
                        let _shared_secret =
                            black_box(p::decapsulate(keypair.private_key(), &ciphertext));
                    },
                    BatchSize::SmallInput,
                )
            });
        };
    }

    macro_rules! fun_unpacked {
        ($name:expr, $p:path, $group:expr) => {
            $group.bench_function(format!("platform={},api=unpacked", $name), |b| {
                use $p as p;
                let mut seed1 = [0; 64];
                OsRng.try_fill_bytes(&mut seed1).unwrap();
                let mut seed2 = [0; 32];
                OsRng.try_fill_bytes(&mut seed2).unwrap();
                b.iter_batched(
                    || {
                        let mut keypair = p::init_key_pair();
                        p::generate_key_pair_mut(seed1, &mut keypair);
                        let (ciphertext, _shared_secret) =
                            p::encapsulate(&keypair.public_key, seed2);
                        (keypair, ciphertext)
                    },
                    |(keypair, ciphertext)| {
                        let _shared_secret = black_box(p::decapsulate(&keypair, &ciphertext));
                    },
                    BatchSize::SmallInput,
                )
            });
        };
    }

    #[cfg(feature = "incremental")]
    macro_rules! fun_incremental {
        ($name:expr, $p:path, $group:expr) => {
            $group.bench_function(format!("platform={},api=incremental", $name), |b| {
                use $p::*;

                let mut seed1 = [0; 64];
                OsRng.fill_bytes(&mut seed1);
                let mut seed2 = [0; 32];
                OsRng.fill_bytes(&mut seed2);

                b.iter_batched(
                    || {
                        let keypair = KeyPairCompressedBytes::from_seed(seed1);

                        let mut encaps_state = [0u8; encaps_state_len()];
                        let mut encaps_shared_secret = [0u8; shared_secret_size()];

                        let ct1 = encapsulate1(
                            keypair.pk1(),
                            seed2,
                            &mut encaps_state,
                            &mut encaps_shared_secret,
                        )
                        .unwrap();

                        let ct2 = encapsulate2(&encaps_state, keypair.pk2());

                        (ct1, ct2, keypair)
                    },
                    |(ct1, ct2, keypair)| {
                        let _shared_secret = decapsulate_compressed_key(keypair.sk(), &ct1, &ct2);
                    },
                    BatchSize::SmallInput,
                )
            });
        };
    }

    init!(mlkem512, "Decapsulation", c);
    init!(mlkem768, "Decapsulation", c);
    init!(mlkem1024, "Decapsulation", c);
}

pub fn comparisons(c: &mut Criterion) {
    key_generation(c);
    encapsulation(c);
    decapsulation(c);
    pk_validation(c);
}

criterion_group!(benches, comparisons);
criterion_main!(benches);

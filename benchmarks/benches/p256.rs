use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn derive(c: &mut Criterion) {
    // Comparing libcrux performance for different payload sizes and other implementations.
    let mut group = c.benchmark_group("P256 derive");

    group.bench_function("libcrux", |b| {
        b.iter_batched(
            || {
                use rand::rngs::SysRng;
                use rand_core::UnwrapErr;
                let mut rng = UnwrapErr(SysRng);
                let (_, pk1) =
                    libcrux_ecdh::key_gen(libcrux_ecdh::Algorithm::P256, &mut rng).unwrap();
                let (sk2, _) =
                    libcrux_ecdh::key_gen(libcrux_ecdh::Algorithm::P256, &mut rng).unwrap();
                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let _zz = libcrux_ecdh::derive(libcrux_ecdh::Algorithm::P256, &pk1, &sk2).unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Ring", |b| {
        use ring::{agreement, rand::SystemRandom};

        b.iter_batched(
            || {
                let rng = SystemRandom::new();
                let sk1 =
                    agreement::EphemeralPrivateKey::generate(&agreement::ECDH_P256, &rng).unwrap();
                let pk1 = sk1.compute_public_key().unwrap();
                let sk2 =
                    agreement::EphemeralPrivateKey::generate(&agreement::ECDH_P256, &rng).unwrap();

                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let _zz: Result<Vec<u8>, ring::error::Unspecified> = agreement::agree_ephemeral(
                    sk2,
                    &agreement::UnparsedPublicKey::new(&agreement::ECDH_P256, pk1),
                    |k| Ok(k.to_vec()),
                )
                .unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("RustCrypto", |b| {
        use p256::{ecdh::EphemeralSecret, PublicKey};
        // Using older version of `rand_core` traits as required by `p256`
        use rand_core_old::OsRng;

        b.iter_batched(
            || {
                let sk1 = EphemeralSecret::random(&mut OsRng);
                let pk1 = PublicKey::from(&sk1);
                let sk2 = EphemeralSecret::random(&mut OsRng);
                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let _zz = sk2.diffie_hellman(&pk1);
            },
            BatchSize::SmallInput,
        )
    });
}

fn secret_to_public(c: &mut Criterion) {
    // Comparing libcrux performance for different payload sizes and other implementations.
    let mut group = c.benchmark_group("P256 secret to public");
    use rand::rngs::SysRng;
    use rand_core::UnwrapErr;
    let mut rng = UnwrapErr(SysRng);

    group.bench_function("libcrux", |b| {
        b.iter_batched(
            || {
                let (sk, _) =
                    libcrux_ecdh::key_gen(libcrux_ecdh::Algorithm::P256, &mut rng).unwrap();
                sk
            },
            |sk| {
                let _pk =
                    libcrux_ecdh::secret_to_public(libcrux_ecdh::Algorithm::P256, &sk).unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Ring", |b| {
        use ring::{agreement, rand::SystemRandom};

        b.iter_batched(
            || {
                let rng = SystemRandom::new();
                let sk =
                    agreement::EphemeralPrivateKey::generate(&agreement::ECDH_P256, &rng).unwrap();

                sk
            },
            |sk| {
                let _pk = sk.compute_public_key().unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("RustCrypto", |b| {
        use p256::{ecdh::EphemeralSecret, PublicKey};
        // Using older version of `rand_core` traits as required by `p256`
        use rand_core_old::OsRng;

        b.iter_batched(
            || {
                let sk = EphemeralSecret::random(&mut OsRng);
                sk
            },
            |sk| {
                let _pk = PublicKey::from(&sk);
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, derive, secret_to_public);
criterion_main!(benches);

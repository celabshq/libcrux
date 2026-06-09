use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand_core::UnwrapErr;

fn derive(c: &mut Criterion) {
    // Comparing libcrux performance for different payload sizes and other implementations.
    let mut group = c.benchmark_group("x25519/derive");

    group.bench_function("libcrux", |b| {
        b.iter_batched(
            || {
                use rand::rngs::SysRng;
                let mut rng = UnwrapErr(SysRng);
                let (_, pk1) = libcrux_ecdh::x25519_key_gen(&mut rng).unwrap();
                let sk2 = libcrux_ecdh::x25519_generate_secret(&mut rng).unwrap();

                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let _zz =
                    libcrux_ecdh::derive(libcrux_ecdh::Algorithm::X25519, &pk1, &sk2).unwrap();
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
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
                let pk1 = sk1.compute_public_key().unwrap();
                let sk2 =
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();

                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let _zz: Result<Vec<u8>, ring::error::Unspecified> = agreement::agree_ephemeral(
                    sk2,
                    &agreement::UnparsedPublicKey::new(&agreement::X25519, pk1),
                    |k| Ok(k.to_vec()),
                )
                .unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(not(windows), not(target_arch = "wasm32"), not(target_arch = "x86")))]
    group.bench_function("OpenSSL", |b| {
        use openssl::{
            derive::Deriver,
            pkey::{Id, PKey},
            pkey_ctx::PkeyCtx,
        };

        b.iter_batched(
            || {
                let mut ctx = PkeyCtx::new_id(Id::X25519).unwrap();
                ctx.keygen_init().unwrap();
                let sk1 = ctx.keygen().unwrap();
                let pk1 = sk1.raw_public_key().unwrap();

                ctx.keygen_init().unwrap();
                let sk2 = ctx.keygen().unwrap();
                let pk1 = PKey::public_key_from_raw_bytes(&pk1, Id::X25519).unwrap();
                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let mut deriver = Deriver::new(&sk2).unwrap();
                deriver.set_peer(&pk1).unwrap();
                let _zz = deriver.derive_to_vec().unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Dalek", |b| {
        use rand_core_old::OsRng;
        use x25519_dalek::{EphemeralSecret, PublicKey};

        b.iter_batched(
            || {
                let sk1 = EphemeralSecret::random_from_rng(OsRng);
                let pk1 = PublicKey::from(&sk1);
                let sk2 = EphemeralSecret::random_from_rng(OsRng);
                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let _zz = sk2.diffie_hellman(&pk1);
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Dalek Ristretto", |b| {
        use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
        use rand::{rngs::SysRng, TryRng};

        b.iter_batched(
            || {
                let mut sk1_b = [0u8; 32];
                SysRng.try_fill_bytes(&mut sk1_b).unwrap();
                let sk1 = Scalar::from_bytes_mod_order(sk1_b);
                let pk1 = RistrettoPoint::mul_base(&sk1);
                let mut sk2_b = [0u8; 32];
                SysRng.try_fill_bytes(&mut sk2_b).unwrap();
                let sk2 = Scalar::from_bytes_mod_order(sk2_b);
                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let _zz = pk1 * sk2;
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(target_arch = "x86_64", target_os = "linux", crypto_lib25519))]
    group.bench_function("lib25519", |b| {
        b.iter_batched(
            || {
                let mut sk1 = [0u8; 32];
                let mut pk1 = [0u8; 32];
                unsafe {
                    lib25519::lib25519_dh_x25519_keypair(pk1.as_mut_ptr(), sk1.as_mut_ptr());
                }
                let sk2 = randombytes(32);
                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let mut zz = [0u8; 32];
                unsafe {
                    lib25519::lib25519_dh_x25519(zz.as_mut_ptr(), pk1.as_ptr(), sk2.as_ptr());
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn secret_to_public(c: &mut Criterion) {
    // Comparing libcrux performance for different payload sizes and other implementations.
    let mut group = c.benchmark_group("x25519/secret to public");

    group.bench_function("libcrux", |b| {
        b.iter_batched(
            || {
                use rand::rngs::SysRng;
                let mut rng = UnwrapErr(SysRng);
                let sk = libcrux_ecdh::x25519_generate_secret(&mut rng).unwrap();
                sk
            },
            |sk| {
                let _pk =
                    libcrux_ecdh::secret_to_public(libcrux_ecdh::Algorithm::X25519, &sk).unwrap();
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
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();

                sk
            },
            |sk| {
                let _pk = sk.compute_public_key().unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(not(windows), not(target_arch = "wasm32"), not(target_arch = "x86")))]
    group.bench_function("OpenSSL", |b| {
        use openssl::{pkey::Id, pkey_ctx::PkeyCtx};

        b.iter_batched(
            || {},
            |()| {
                let mut ctx = PkeyCtx::new_id(Id::X25519).unwrap();
                ctx.keygen_init().unwrap();
                let sk1 = ctx.keygen().unwrap();
                let _pk1 = sk1.raw_public_key().unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Dalek", |b| {
        // Use older version of `rand_core` as required by library
        use rand_core_old::OsRng;
        use x25519_dalek::{EphemeralSecret, PublicKey};

        b.iter_batched(
            || {
                let sk = EphemeralSecret::random_from_rng(OsRng);
                sk
            },
            |sk| {
                let _pk = PublicKey::from(&sk);
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Dalek Ristretto", |b| {
        use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
        use rand::{rngs::SysRng, TryRng};

        b.iter_batched(
            || {
                let mut sk_b = [0u8; 32];
                SysRng.try_fill_bytes(&mut sk_b).unwrap();
                let sk = Scalar::from_bytes_mod_order(sk_b);
                sk
            },
            |sk| {
                let _pk = RistrettoPoint::mul_base(&sk);
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(target_arch = "x86_64", target_os = "linux", crypto_lib25519))]
    group.bench_function("lib25519", |b| {
        b.iter_batched(
            || {},
            |_| {
                let mut sk = [0u8; 32];
                let mut pk = [0u8; 32];
                unsafe {
                    lib25519::lib25519_dh_x25519_keypair(pk.as_mut_ptr(), sk.as_mut_ptr());
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn nym_outfox_create(c: &mut Criterion) {
    // Comparing libcrux performance for different payload sizes and other implementations.
    let mut group = c.benchmark_group("x25519/nym outfox create");

    group.bench_function("libcrux", |b| {
        b.iter_batched(
            || {
                use rand::rngs::SysRng;
                let mut rng = UnwrapErr(SysRng);

                let (_, pk1) = libcrux_ecdh::x25519_key_gen(&mut rng).unwrap();

                let sk2a = libcrux_ecdh::x25519_generate_secret(&mut rng).unwrap();
                let sk2b = libcrux_ecdh::x25519_generate_secret(&mut rng).unwrap();
                let sk2c = libcrux_ecdh::x25519_generate_secret(&mut rng).unwrap();
                let sk2d = libcrux_ecdh::x25519_generate_secret(&mut rng).unwrap();

                (pk1, sk2a, sk2b, sk2c, sk2d)
            },
            |(pk1, sk2a, sk2b, sk2c, sk2d)| {
                let _pk2a =
                    libcrux_ecdh::secret_to_public(libcrux_ecdh::Algorithm::X25519, &sk2a).unwrap();
                let _pk2b =
                    libcrux_ecdh::secret_to_public(libcrux_ecdh::Algorithm::X25519, &sk2b).unwrap();
                let _pk2c =
                    libcrux_ecdh::secret_to_public(libcrux_ecdh::Algorithm::X25519, &sk2c).unwrap();
                let _pk2d =
                    libcrux_ecdh::secret_to_public(libcrux_ecdh::Algorithm::X25519, &sk2d).unwrap();
                let _zza =
                    libcrux_ecdh::derive(libcrux_ecdh::Algorithm::X25519, &pk1, &sk2a).unwrap();
                let _zzb =
                    libcrux_ecdh::derive(libcrux_ecdh::Algorithm::X25519, &pk1, &sk2b).unwrap();
                let _zzc =
                    libcrux_ecdh::derive(libcrux_ecdh::Algorithm::X25519, &pk1, &sk2c).unwrap();
                let _zzd =
                    libcrux_ecdh::derive(libcrux_ecdh::Algorithm::X25519, &pk1, &sk2d).unwrap();
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
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
                let pk1 = sk1.compute_public_key().unwrap();
                let sk2a =
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();

                let sk2b =
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();

                let sk2c =
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();

                let sk2d =
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();

                (pk1, sk2a, sk2b, sk2c, sk2d)
            },
            |(pk1, sk2a, sk2b, sk2c, sk2d)| {
                let _pk2a = sk2a.compute_public_key().unwrap();
                let _pk2b = sk2b.compute_public_key().unwrap();
                let _pk2c = sk2c.compute_public_key().unwrap();
                let _pk2d = sk2d.compute_public_key().unwrap();
                let _zza: Result<Vec<u8>, ring::error::Unspecified> = agreement::agree_ephemeral(
                    sk2a,
                    &agreement::UnparsedPublicKey::new(&agreement::X25519, pk1.clone()),
                    |k| Ok(k.to_vec()),
                )
                .unwrap();
                let _zzb: Result<Vec<u8>, ring::error::Unspecified> = agreement::agree_ephemeral(
                    sk2b,
                    &agreement::UnparsedPublicKey::new(&agreement::X25519, pk1.clone()),
                    |k| Ok(k.to_vec()),
                )
                .unwrap();
                let _zzc: Result<Vec<u8>, ring::error::Unspecified> = agreement::agree_ephemeral(
                    sk2c,
                    &agreement::UnparsedPublicKey::new(&agreement::X25519, pk1.clone()),
                    |k| Ok(k.to_vec()),
                )
                .unwrap();
                let _zzd: Result<Vec<u8>, ring::error::Unspecified> = agreement::agree_ephemeral(
                    sk2d,
                    &agreement::UnparsedPublicKey::new(&agreement::X25519, pk1.clone()),
                    |k| Ok(k.to_vec()),
                )
                .unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(not(windows), not(target_arch = "wasm32"), not(target_arch = "x86")))]
    group.bench_function("OpenSSL", |b| {
        use openssl::{
            derive::Deriver,
            pkey::{Id, PKey},
            pkey_ctx::PkeyCtx,
        };

        b.iter_batched(
            || {
                let mut ctx = PkeyCtx::new_id(Id::X25519).unwrap();
                ctx.keygen_init().unwrap();
                let pkey = ctx.keygen().unwrap();
                let pk1 = pkey.raw_public_key().unwrap();
                let pk1 = PKey::public_key_from_raw_bytes(&pk1, Id::X25519).unwrap();

                pk1
            },
            |pk1| {
                let mut ctx = PkeyCtx::new_id(Id::X25519).unwrap();
                ctx.keygen_init().unwrap();
                let sk2a = ctx.keygen().unwrap();

                ctx.keygen_init().unwrap();
                let sk2b = ctx.keygen().unwrap();

                ctx.keygen_init().unwrap();
                let sk2c = ctx.keygen().unwrap();

                ctx.keygen_init().unwrap();
                let sk2d = ctx.keygen().unwrap();

                let mut deriver1 = Deriver::new(&sk2a).unwrap();
                deriver1.set_peer(&pk1).unwrap();
                let _zz1 = deriver1.derive_to_vec().unwrap();
                let mut deriver2 = Deriver::new(&sk2b).unwrap();
                deriver2.set_peer(&pk1).unwrap();
                let _zz2 = deriver2.derive_to_vec().unwrap();
                let mut deriver3 = Deriver::new(&sk2c).unwrap();
                deriver3.set_peer(&pk1).unwrap();
                let _zz3 = deriver3.derive_to_vec().unwrap();
                let mut deriver4 = Deriver::new(&sk2d).unwrap();
                deriver4.set_peer(&pk1).unwrap();
                let _zz4 = deriver4.derive_to_vec().unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Dalek", |b| {
        // use older version of `rand_core` as required by library
        use rand_core_old::OsRng;
        use x25519_dalek::{EphemeralSecret, PublicKey};

        b.iter_batched(
            || {
                let sk1 = EphemeralSecret::random_from_rng(OsRng);
                let pk1 = PublicKey::from(&sk1);
                let sk2a = EphemeralSecret::random_from_rng(OsRng);
                let sk2b = EphemeralSecret::random_from_rng(OsRng);
                let sk2c = EphemeralSecret::random_from_rng(OsRng);
                let sk2d = EphemeralSecret::random_from_rng(OsRng);
                (pk1, sk2a, sk2b, sk2c, sk2d)
            },
            |(pk1, sk2a, sk2b, sk2c, sk2d)| {
                let _pk2a = PublicKey::from(&sk2a);
                let _pk2b = PublicKey::from(&sk2b);
                let _pk2c = PublicKey::from(&sk2c);
                let _pk2d = PublicKey::from(&sk2d);
                let _zza = sk2a.diffie_hellman(&pk1);
                let _zzb = sk2b.diffie_hellman(&pk1);
                let _zzc = sk2c.diffie_hellman(&pk1);
                let _zzd = sk2d.diffie_hellman(&pk1);
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Dalek Ristretto", |b| {
        use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
        use rand::{rngs::SysRng, TryRng};

        b.iter_batched(
            || {
                let mut sk1_b = [0u8; 32];
                SysRng.try_fill_bytes(&mut sk1_b).unwrap();
                let sk1 = Scalar::from_bytes_mod_order(sk1_b);
                let pk1 = RistrettoPoint::mul_base(&sk1);
                let mut sk2_b = [0u8; 32];
                SysRng.try_fill_bytes(&mut sk2_b).unwrap();
                let sk2a = Scalar::from_bytes_mod_order(sk2_b);
                let sk2b = Scalar::from_bytes_mod_order(sk2_b);
                let sk2c = Scalar::from_bytes_mod_order(sk2_b);
                let sk2d = Scalar::from_bytes_mod_order(sk2_b);
                (pk1, sk2a, sk2b, sk2c, sk2d)
            },
            |(pk1, sk2a, sk2b, sk2c, sk2d)| {
                let _pk2a = RistrettoPoint::mul_base(&sk2a);
                let _pk2b = RistrettoPoint::mul_base(&sk2b);
                let _pk2c = RistrettoPoint::mul_base(&sk2c);
                let _pk2d = RistrettoPoint::mul_base(&sk2d);
                let _zza = pk1 * sk2a;
                let _zzb = pk1 * sk2b;
                let _zzc = pk1 * sk2c;
                let _zzd = pk1 * sk2d;
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(target_arch = "x86_64", target_os = "linux", crypto_lib25519))]
    group.bench_function("lib25519", |b| {
        b.iter_batched(
            || {
                let mut sk1 = [0u8; 32];
                let mut pk1 = [0u8; 32];
                unsafe {
                    lib25519::lib25519_dh_x25519_keypair(pk1.as_mut_ptr(), sk1.as_mut_ptr());
                }
                let sk2a = randombytes(32);
                let sk2b = randombytes(32);
                let sk2c = randombytes(32);
                let sk2d = randombytes(32);
                (pk1, sk2a, sk2b, sk2c, sk2d)
            },
            |(pk1, sk2a, sk2b, sk2c, sk2d)| {
                let mut pks = [0u8; 32 * 8];
                let mut sks = [0u8; 32 * 8];
                let mut zzs = [0u8; 32 * 8];
                pks[..32].copy_from_slice(&pk1);
                pks[32..64].copy_from_slice(&pk1);
                pks[64..96].copy_from_slice(&pk1);
                pks[96..128].copy_from_slice(&pk1);
                pks[128..160].copy_from_slice(&pk1);
                pks[160..192].copy_from_slice(&pk1);
                pks[192..224].copy_from_slice(&pk1);
                pks[224..].copy_from_slice(&pk1);
                sks[..32].copy_from_slice(&sk2a);
                sks[32..64].copy_from_slice(&sk2b);
                sks[64..96].copy_from_slice(&sk2c);
                sks[96..128].copy_from_slice(&sk2d);
                sks[128..160].copy_from_slice(&sk2a);
                sks[160..192].copy_from_slice(&sk2b);
                sks[192..224].copy_from_slice(&sk2c);
                sks[224..].copy_from_slice(&sk2d);
                unsafe {
                    lib25519::lib25519_nPbatch_montgomery25519(
                        zzs.as_mut_ptr(),
                        pks.as_ptr(),
                        sks.as_ptr(),
                        8,
                    );
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn nym_outfox_process(c: &mut Criterion) {
    // Comparing libcrux performance for different payload sizes and other implementations.
    let mut group = c.benchmark_group("x25519/nym outfox process");

    group.bench_function("libcrux", |b| {
        b.iter_batched(
            || {
                use rand::rngs::SysRng;
                let mut rng = UnwrapErr(SysRng);

                let (_, pk1) = libcrux_ecdh::x25519_key_gen(&mut rng).unwrap();
                let sk2 = libcrux_ecdh::x25519_generate_secret(&mut rng).unwrap();

                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let _zz =
                    libcrux_ecdh::derive(libcrux_ecdh::Algorithm::X25519, &pk1, &sk2).unwrap();
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
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
                let pk1 = sk1.compute_public_key().unwrap();
                let sk2 =
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();

                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let _zz: Result<Vec<u8>, ring::error::Unspecified> = agreement::agree_ephemeral(
                    sk2,
                    &agreement::UnparsedPublicKey::new(&agreement::X25519, pk1),
                    |k| Ok(k.to_vec()),
                )
                .unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(not(windows), not(target_arch = "wasm32"), not(target_arch = "x86")))]
    group.bench_function("OpenSSL", |b| {
        use openssl::{
            derive::Deriver,
            pkey::{Id, PKey},
            pkey_ctx::PkeyCtx,
        };

        b.iter_batched(
            || {
                let mut ctx = PkeyCtx::new_id(Id::X25519).unwrap();
                ctx.keygen_init().unwrap();
                let sk1 = ctx.keygen().unwrap();
                let pk1 = sk1.raw_public_key().unwrap();
                let pk1 = PKey::public_key_from_raw_bytes(&pk1, Id::X25519).unwrap();

                let sk2 = PKey::generate_x25519().unwrap();

                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let mut deriver = Deriver::new(&sk2).unwrap();
                deriver.set_peer(&pk1).unwrap();
                let _zz = deriver.derive_to_vec().unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Dalek", |b| {
        // use older version of `rand_core` as required by library
        use rand_core_old::OsRng;
        use x25519_dalek::{EphemeralSecret, PublicKey};

        b.iter_batched(
            || {
                let sk1 = EphemeralSecret::random_from_rng(OsRng);
                let pk1 = PublicKey::from(&sk1);
                let sk2 = EphemeralSecret::random_from_rng(OsRng);
                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let _zz = sk2.diffie_hellman(&pk1);
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Dalek Ristretto", |b| {
        use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
        use rand::{rngs::SysRng, TryRng};

        b.iter_batched(
            || {
                let mut sk1_b = [0u8; 32];
                SysRng.try_fill_bytes(&mut sk1_b).unwrap();
                let sk1 = Scalar::from_bytes_mod_order(sk1_b);
                let pk1 = RistrettoPoint::mul_base(&sk1);
                let mut sk2_b = [0u8; 32];
                SysRng.try_fill_bytes(&mut sk2_b).unwrap();
                let sk2 = Scalar::from_bytes_mod_order(sk2_b);
                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let _zz = pk1 * sk2;
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(target_arch = "x86_64", target_os = "linux", crypto_lib25519))]
    group.bench_function("lib25519", |b| {
        b.iter_batched(
            || {
                let mut sk1 = [0u8; 32];
                let mut pk1 = [0u8; 32];
                unsafe {
                    lib25519::lib25519_dh_x25519_keypair(pk1.as_mut_ptr(), sk1.as_mut_ptr());
                }
                let sk2 = randombytes(32);
                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let mut zz = [0u8; 32];
                unsafe {
                    lib25519::lib25519_dh_x25519(zz.as_mut_ptr(), pk1.as_ptr(), sk2.as_ptr());
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn nym_sphinx_create(c: &mut Criterion) {
    // Comparing libcrux performance for different payload sizes and other implementations.
    let mut group = c.benchmark_group("x25519/nym sphinx create");

    group.bench_function("libcrux", |b| {
        b.iter_batched(
            || {
                use rand::rngs::SysRng;
                let mut rng = UnwrapErr(SysRng);
                let (_, pk1) = libcrux_ecdh::x25519_key_gen(&mut rng).unwrap();
                let sk2 = libcrux_ecdh::x25519_generate_secret(&mut rng).unwrap();

                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let _pk2a =
                    libcrux_ecdh::secret_to_public(libcrux_ecdh::Algorithm::X25519, &sk2).unwrap();
                let zza =
                    libcrux_ecdh::derive(libcrux_ecdh::Algorithm::X25519, &pk1, &sk2).unwrap();
                let _pk2b =
                    libcrux_ecdh::secret_to_public(libcrux_ecdh::Algorithm::X25519, &zza).unwrap();
                let zzb =
                    libcrux_ecdh::derive(libcrux_ecdh::Algorithm::X25519, &pk1, &zza).unwrap();
                let _pk2c =
                    libcrux_ecdh::secret_to_public(libcrux_ecdh::Algorithm::X25519, &zzb).unwrap();
                let zzc =
                    libcrux_ecdh::derive(libcrux_ecdh::Algorithm::X25519, &pk1, &zzb).unwrap();
                let _pk2d =
                    libcrux_ecdh::secret_to_public(libcrux_ecdh::Algorithm::X25519, &zzc).unwrap();
                let _zzd =
                    libcrux_ecdh::derive(libcrux_ecdh::Algorithm::X25519, &pk1, &zzc).unwrap();
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
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
                let pk1 = sk1.compute_public_key().unwrap();
                let sk2a =
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();

                let sk2b =
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();

                let sk2c =
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();

                let sk2d =
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();

                (pk1, sk2a, sk2b, sk2c, sk2d)
            },
            |(pk1, sk2a, sk2b, sk2c, sk2d)| {
                let _pk2a = sk2a.compute_public_key().unwrap();
                let _zza: Result<Vec<u8>, ring::error::Unspecified> = agreement::agree_ephemeral(
                    sk2a,
                    &agreement::UnparsedPublicKey::new(&agreement::X25519, pk1.clone()),
                    |k| Ok(k.to_vec()),
                )
                .unwrap();
                let _pk2b = sk2b.compute_public_key().unwrap();
                let _zzb: Result<Vec<u8>, ring::error::Unspecified> = agreement::agree_ephemeral(
                    sk2b,
                    &agreement::UnparsedPublicKey::new(&agreement::X25519, pk1.clone()),
                    |k| Ok(k.to_vec()),
                )
                .unwrap();
                let _pk2c = sk2c.compute_public_key().unwrap();
                let _zzc: Result<Vec<u8>, ring::error::Unspecified> = agreement::agree_ephemeral(
                    sk2c,
                    &agreement::UnparsedPublicKey::new(&agreement::X25519, pk1.clone()),
                    |k| Ok(k.to_vec()),
                )
                .unwrap();
                let _pk2d = sk2d.compute_public_key().unwrap();
                let _zzd: Result<Vec<u8>, ring::error::Unspecified> = agreement::agree_ephemeral(
                    sk2d,
                    &agreement::UnparsedPublicKey::new(&agreement::X25519, pk1.clone()),
                    |k| Ok(k.to_vec()),
                )
                .unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(not(windows), not(target_arch = "wasm32"), not(target_arch = "x86")))]
    group.bench_function("OpenSSL", |b| {
        use openssl::{
            derive::Deriver,
            pkey::{Id, PKey},
            pkey_ctx::PkeyCtx,
        };

        b.iter_batched(
            || {
                let mut ctx = PkeyCtx::new_id(Id::X25519).unwrap();
                ctx.keygen_init().unwrap();
                let sk1 = ctx.keygen().unwrap();
                let pk1 = sk1.raw_public_key().unwrap();
                let pk1 = PKey::public_key_from_raw_bytes(&pk1, Id::X25519).unwrap();

                pk1
            },
            |pk1| {
                let mut ctx = PkeyCtx::new_id(Id::X25519).unwrap();
                ctx.keygen_init().unwrap();
                let sk2a = ctx.keygen().unwrap();

                let mut deriver1 = Deriver::new(&sk2a).unwrap();
                deriver1.set_peer(&pk1).unwrap();
                let _zz1 = deriver1.derive_to_vec().unwrap();

                ctx.keygen_init().unwrap();
                let sk2b = ctx.keygen().unwrap();

                let mut deriver2 = Deriver::new(&sk2b).unwrap();
                deriver2.set_peer(&pk1).unwrap();
                let _zz2 = deriver2.derive_to_vec().unwrap();

                ctx.keygen_init().unwrap();
                let sk2c = ctx.keygen().unwrap();

                let mut deriver3 = Deriver::new(&sk2c).unwrap();
                deriver3.set_peer(&pk1).unwrap();
                let _zz3 = deriver3.derive_to_vec().unwrap();

                ctx.keygen_init().unwrap();
                let sk2d = ctx.keygen().unwrap();

                let mut deriver4 = Deriver::new(&sk2d).unwrap();
                deriver4.set_peer(&pk1).unwrap();
                let _zz4 = deriver4.derive_to_vec().unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Dalek", |b| {
        // Use older version of `rand_core` as required by library
        use rand_core_old::OsRng;
        use x25519_dalek::{EphemeralSecret, PublicKey};

        b.iter_batched(
            || {
                let sk1 = EphemeralSecret::random_from_rng(OsRng);
                let pk1 = PublicKey::from(&sk1);
                let sk2 = EphemeralSecret::random_from_rng(OsRng);
                (pk1, sk2)
            },
            |(pk1, sk2a)| {
                let _pk2a = PublicKey::from(&sk2a);
                let _zza = sk2a.diffie_hellman(&pk1);
                let sk2b = EphemeralSecret::random_from_rng(OsRng);
                let _pk2b = PublicKey::from(&sk2b);
                let _zza = sk2b.diffie_hellman(&pk1);
                let sk2c = EphemeralSecret::random_from_rng(OsRng);
                let _pk2c = PublicKey::from(&sk2c);
                let _zzc = sk2c.diffie_hellman(&pk1);
                let sk2d = EphemeralSecret::random_from_rng(OsRng);
                let _pk2d = PublicKey::from(&sk2d);
                let _zzd = sk2d.diffie_hellman(&pk1);
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Dalek Ristretto", |b| {
        use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
        use rand::{rngs::SysRng, TryRng};

        b.iter_batched(
            || {
                let mut sk1_b = [0u8; 32];
                SysRng.try_fill_bytes(&mut sk1_b).unwrap();
                let sk1 = Scalar::from_bytes_mod_order(sk1_b);
                let pk1 = RistrettoPoint::mul_base(&sk1);
                let mut sk2_b = [0u8; 32];
                SysRng.try_fill_bytes(&mut sk2_b).unwrap();
                let sk2a = Scalar::from_bytes_mod_order(sk2_b);
                (pk1, sk2a)
            },
            |(pk1, sk2a)| {
                let _pk2a = RistrettoPoint::mul_base(&sk2a);
                let zza = (pk1 * sk2a).compress().to_bytes();
                let sk2b = Scalar::from_bytes_mod_order(zza);
                let _pk2b = RistrettoPoint::mul_base(&sk2b);
                let zzb = (pk1 * sk2b).compress().to_bytes();
                let sk2c = Scalar::from_bytes_mod_order(zzb);
                let _pk2c = RistrettoPoint::mul_base(&sk2c);
                let zzc = (pk1 * sk2c).compress().to_bytes();
                let sk2d = Scalar::from_bytes_mod_order(zzc);
                let _pk2d = RistrettoPoint::mul_base(&sk2d);
                let _zzd = pk1 * sk2d;
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(target_arch = "x86_64", target_os = "linux", crypto_lib25519))]
    group.bench_function("lib25519", |b| {
        b.iter_batched(
            || {
                let mut sk1 = [0u8; 32];
                let mut pk1 = [0u8; 32];
                unsafe {
                    lib25519::lib25519_dh_x25519_keypair(pk1.as_mut_ptr(), sk1.as_mut_ptr());
                }
                let sk2 = randombytes(32);
                sk2
            },
            |sk2| {
                let mut zz1 = [0u8; 32];
                let mut zz2 = [0u8; 32];
                let mut zz3 = [0u8; 32];
                let mut zz4 = [0u8; 32];
                let mut pk1 = [0u8; 32];
                let mut pk2 = [0u8; 32];
                let mut pk3 = [0u8; 32];
                let mut pk4 = [0u8; 32];
                unsafe {
                    lib25519::lib25519_nG_montgomery25519(pk1.as_mut_ptr(), sk2.as_ptr());
                    lib25519::lib25519_dh_x25519(zz1.as_mut_ptr(), pk1.as_ptr(), sk2.as_ptr());
                }
                unsafe {
                    lib25519::lib25519_nG_montgomery25519(pk2.as_mut_ptr(), zz1.as_ptr());
                    lib25519::lib25519_dh_x25519(zz2.as_mut_ptr(), pk1.as_ptr(), zz1.as_ptr());
                }
                unsafe {
                    lib25519::lib25519_nG_montgomery25519(pk3.as_mut_ptr(), zz3.as_ptr());
                    lib25519::lib25519_dh_x25519(zz3.as_mut_ptr(), pk1.as_ptr(), zz2.as_ptr());
                }
                unsafe {
                    lib25519::lib25519_nG_montgomery25519(pk4.as_mut_ptr(), zz3.as_ptr());
                    lib25519::lib25519_dh_x25519(zz4.as_mut_ptr(), pk1.as_ptr(), zz3.as_ptr());
                }
            },
            BatchSize::SmallInput,
        )
    });
}

fn nym_sphinx_process(c: &mut Criterion) {
    // Comparing libcrux performance for different payload sizes and other implementations.
    let mut group = c.benchmark_group("x25519/nym sphinx process");

    group.bench_function("libcrux", |b| {
        b.iter_batched(
            || {
                use rand::rngs::SysRng;
                let mut rng = UnwrapErr(SysRng);
                let (_, pk1) = libcrux_ecdh::x25519_key_gen(&mut rng).unwrap();
                let sk2 = libcrux_ecdh::x25519_generate_secret(&mut rng).unwrap();

                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let _zz1 =
                    libcrux_ecdh::derive(libcrux_ecdh::Algorithm::X25519, &pk1, &sk2).unwrap();
                let _zz2 =
                    libcrux_ecdh::derive(libcrux_ecdh::Algorithm::X25519, &pk1, &sk2).unwrap();
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
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
                let pk1 = sk1.compute_public_key().unwrap();
                let sk2 =
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
                let pk2 = sk2.compute_public_key().unwrap();

                let sk3 =
                    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();

                (pk1, pk2, sk2, sk3)
            },
            |(pk1, pk2, sk2, sk3)| {
                let _zz: Result<Vec<u8>, ring::error::Unspecified> = agreement::agree_ephemeral(
                    sk2,
                    &agreement::UnparsedPublicKey::new(&agreement::X25519, pk1),
                    |k| Ok(k.to_vec()),
                )
                .unwrap();

                let _zz2: Result<Vec<u8>, ring::error::Unspecified> = agreement::agree_ephemeral(
                    sk3,
                    &agreement::UnparsedPublicKey::new(&agreement::X25519, pk2),
                    |k| Ok(k.to_vec()),
                )
                .unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(not(windows), not(target_arch = "wasm32"), not(target_arch = "x86")))]
    group.bench_function("OpenSSL", |b| {
        use openssl::{
            derive::Deriver,
            pkey::{Id, PKey},
            pkey_ctx::PkeyCtx,
        };

        b.iter_batched(
            || {
                let mut ctx = PkeyCtx::new_id(Id::X25519).unwrap();
                ctx.keygen_init().unwrap();
                let sk1 = ctx.keygen().unwrap();
                let pk1 = sk1.raw_public_key().unwrap();
                let pk1 = PKey::public_key_from_raw_bytes(&pk1, Id::X25519).unwrap();

                ctx.keygen_init().unwrap();
                let sk2 = ctx.keygen().unwrap();
                let pk2 = sk2.raw_public_key().unwrap();
                let pk2 = PKey::public_key_from_raw_bytes(&pk2, Id::X25519).unwrap();

                (sk1, pk1, sk2, pk2)
            },
            |(sk1, pk1, sk2, pk2)| {
                let mut deriver = Deriver::new(&sk2).unwrap();
                deriver.set_peer(&pk1).unwrap();
                let _zz1 = deriver.derive_to_vec().unwrap();
                let mut deriver = Deriver::new(&sk1).unwrap();
                deriver.set_peer(&pk2).unwrap();
                let _zz2 = deriver.derive_to_vec().unwrap();
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Dalek", |b| {
        use rand_core_old::OsRng;
        use x25519_dalek::{EphemeralSecret, PublicKey};

        b.iter_batched(
            || {
                let rng = OsRng;
                let sk1 = EphemeralSecret::random_from_rng(rng);
                let pk1 = PublicKey::from(&sk1);
                let sk2 = EphemeralSecret::random_from_rng(rng);
                let pk2 = PublicKey::from(&sk2);
                (sk1, pk1, sk2, pk2)
            },
            |(sk1, pk1, sk2, pk2)| {
                let _zz = sk2.diffie_hellman(&pk1);
                let _zz2 = sk1.diffie_hellman(&pk2);
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("Dalek Ristretto", |b| {
        use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
        use rand::{rngs::SysRng, TryRng};

        b.iter_batched(
            || {
                let mut sk1_b = [0u8; 32];
                SysRng.try_fill_bytes(&mut sk1_b).unwrap();
                let sk1 = Scalar::from_bytes_mod_order(sk1_b);
                let pk1 = RistrettoPoint::mul_base(&sk1);
                let mut sk2_b = [0u8; 32];
                SysRng.try_fill_bytes(&mut sk2_b).unwrap();
                let sk2 = Scalar::from_bytes_mod_order(sk2_b);
                let pk2 = RistrettoPoint::mul_base(&sk2);
                (sk1, pk1, sk2, pk2)
            },
            |(sk1, pk1, sk2, pk2)| {
                let _zz = pk1 * sk2;
                let _zz2 = pk2 * sk1;
            },
            BatchSize::SmallInput,
        )
    });

    #[cfg(all(target_arch = "x86_64", target_os = "linux", crypto_lib25519))]
    group.bench_function("lib25519", |b| {
        b.iter_batched(
            || {
                let mut sk1 = [0u8; 32];
                let mut pk1 = [0u8; 32];
                unsafe {
                    lib25519::lib25519_dh_x25519_keypair(pk1.as_mut_ptr(), sk1.as_mut_ptr());
                }
                let sk2 = randombytes(32);
                (pk1, sk2)
            },
            |(pk1, sk2)| {
                let mut zz = [0u8; 32];
                let mut zz2 = [0u8; 32];
                unsafe {
                    lib25519::lib25519_dh_x25519(zz.as_mut_ptr(), pk1.as_ptr(), sk2.as_ptr());
                    lib25519::lib25519_dh_x25519(zz2.as_mut_ptr(), pk1.as_ptr(), sk2.as_ptr());
                }
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    derive,
    secret_to_public,
    nym_outfox_create,
    nym_outfox_process,
    nym_sphinx_create,
    nym_sphinx_process
);
criterion_main!(benches);

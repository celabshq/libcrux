use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::{rngs::OsRng, TryRngCore};

use libcrux_ml_dsa::ml_dsa_65;

pub fn comparisons_key_generation(c: &mut Criterion) {
    let mut rng = OsRng;
    let mut group = c.benchmark_group("ML-DSA-65 Key Generation");
    group.measurement_time(Duration::from_secs(10));

    let mut randomness = [0; 32];
    rng.try_fill_bytes(&mut randomness).unwrap();

    group.bench_function("libcrux (external random)", move |b| {
        b.iter(|| {
            let _ = ml_dsa_65::generate_key_pair(randomness);
        })
    });

    #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
    group.bench_function("pqclean (internal random)", move |b| {
        b.iter(|| {
            let (_, _) = pqcrypto_mldsa::mldsa65::keypair();
        })
    });
}

pub fn comparisons_signing(c: &mut Criterion) {
    let mut rng = OsRng;
    let mut group = c.benchmark_group("ML-DSA-65 Signing");
    group.measurement_time(Duration::from_secs(10));

    let mut message = [0u8; 511];
    rng.try_fill_bytes(&mut message).unwrap();

    let mut randomness = [0; 32];
    rng.try_fill_bytes(&mut randomness).unwrap();
    let keypair = ml_dsa_65::generate_key_pair(randomness);

    rng.try_fill_bytes(&mut randomness).unwrap();

    group.bench_function("libcrux (external random)", move |b| {
        b.iter(|| {
            let _ = ml_dsa_65::sign(&keypair.signing_key, &message, b"", randomness);
        })
    });

    #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
    {
        let (_, sk) = pqcrypto_mldsa::mldsa65::keypair();
        group.bench_function("pqclean (internal random)", move |b| {
            b.iter(|| {
                let _ = pqcrypto_mldsa::mldsa65::detached_sign(&message, &sk);
            })
        });
    }
}

pub fn comparisons_verification(c: &mut Criterion) {
    let mut rng = OsRng;
    let mut group = c.benchmark_group("ML-DSA-65 Verification");
    group.measurement_time(Duration::from_secs(10));

    let mut message = [0u8; 511];
    rng.try_fill_bytes(&mut message).unwrap();

    let mut randomness = [0; 32];
    rng.try_fill_bytes(&mut randomness).unwrap();
    let keypair = ml_dsa_65::generate_key_pair(randomness);

    rng.try_fill_bytes(&mut randomness).unwrap();
    let signature = ml_dsa_65::sign(&keypair.signing_key, &message, b"", randomness).unwrap();

    group.bench_function("libcrux", move |b| {
        b.iter(|| {
            let _ =
                ml_dsa_65::verify(&keypair.verification_key, &message, b"", &signature).unwrap();
        })
    });

    #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
    {
        let (vk, sk) = pqcrypto_mldsa::mldsa65::keypair();
        let signature = pqcrypto_mldsa::mldsa65::detached_sign(&message, &sk);

        group.bench_function("pqclean", move |b| {
            b.iter(|| {
                let _ =
                    pqcrypto_mldsa::mldsa65::verify_detached_signature(&signature, &message, &vk)
                        .unwrap();
            })
        });
    }
}

pub fn comparisons(c: &mut Criterion) {
    comparisons_key_generation(c);
    comparisons_signing(c);
    comparisons_verification(c);
}

criterion_group!(benches, comparisons);
criterion_main!(benches);

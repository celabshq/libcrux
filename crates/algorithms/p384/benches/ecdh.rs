use criterion::{criterion_group, criterion_main, Criterion};
use libcrux_p384::{PrivateKey, PublicKey};

// A valid (secret key, public key) pair taken from the Wycheproof
// secp384r1 ECDH test vectors (tcId 1).
const SK: &str = "766e61425b2da9f846c09fc3564b93a6f8603b7392c785165bf20da948c49fd1fb1dee4edd64356b9f21c588b75dfd81";
const PK: &str = "04790a6e059ef9a5940163183d4a7809135d29791643fc43a2f17ee8bf677ab84f791b64a6be15969ffa012dd9185d8796d9b954baa8a75e82df711b3b56eadff6b0f668c3b26b4b1aeb308a1fcc1c680d329a6705025f1c98a0b5e5bfcb163caa";

fn ecdh_static(c: &mut Criterion) {
    let sk_bytes = hex::decode(SK).unwrap();
    let pk_bytes = hex::decode(PK).unwrap();

    c.bench_function("P-384 ECDH", |b| {
        b.iter(|| {
            core::hint::black_box(
                libcrux_p384::derive_ecdh(
                    core::hint::black_box(&sk_bytes),
                    core::hint::black_box(&pk_bytes),
                )
                .unwrap(),
            )
        })
    });
}

fn ecdh_variable_pk_uncompressed(c: &mut Criterion) {
    c.bench_function("P-384 ECDH Variable PK (Uncompressed)", |b| {
        b.iter_batched(
            || {
                let mut rng = rand::rng();

                let sk = PrivateKey::generate(&mut rng).unwrap();
                let mut pk_bytes = [0u8; 97];
                let sk_i = PrivateKey::generate(&mut rng).unwrap();
                PublicKey::from(&sk_i).to_uncompressed(&mut pk_bytes);

                (sk, pk_bytes)
            },
            |(sk, pk)| {
                libcrux_p384::derive_ecdh(
                    core::hint::black_box(sk.as_ref()),
                    core::hint::black_box(&pk),
                )
                .unwrap();
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn ecdh_variable_pk_compressed(c: &mut Criterion) {
    c.bench_function("P-384 ECDH Variable PK (Compressed)", |b| {
        b.iter_batched(
            || {
                let mut rng = rand::rng();

                let sk = PrivateKey::generate(&mut rng).unwrap();
                let mut pk_bytes = [0u8; 49];
                let sk_i = PrivateKey::generate(&mut rng).unwrap();
                PublicKey::from(&sk_i).to_compressed(&mut pk_bytes);

                (sk, pk_bytes)
            },
            |(sk, pk)| {
                libcrux_p384::derive_ecdh(
                    core::hint::black_box(sk.as_ref()),
                    core::hint::black_box(&pk),
                )
                .unwrap();
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::new(15,0));
    targets = ecdh_static,
    ecdh_variable_pk_compressed,
    ecdh_variable_pk_uncompressed
);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use hpke_rs_crypto::{types::*, HpkeCrypto, HpkeDefaultPrng};
use hpke_rs_rust_crypto::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(&format!("x25519 Derive"), |b| {
        b.iter_batched(
            || {
                let (pk, sk) = HpkeRustCrypto::kem_key_gen(
                    KemAlgorithm::DhKem25519,
                    &mut HpkeRustCrypto::try_prng().unwrap(),
                )
                .unwrap();
                (sk.clone(), pk.clone())
            },
            |(sk, pk)| {
                let _ = HpkeRustCrypto::dh(KemAlgorithm::DhKem25519, &pk, &sk);
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function(&format!("x25519 Derive Base"), |b| {
        b.iter_batched(
            || {
                let (_pk, sk) = HpkeRustCrypto::kem_key_gen(
                    KemAlgorithm::DhKem25519,
                    &mut HpkeRustCrypto::try_prng().unwrap(),
                )
                .unwrap();
                sk.clone()
            },
            |sk| {
                let _pk = HpkeRustCrypto::secret_to_public(KemAlgorithm::DhKem25519, &sk).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);

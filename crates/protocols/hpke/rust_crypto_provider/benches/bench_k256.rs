use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use hpke_rs_crypto::{types::KemAlgorithm, HpkeCrypto, HpkeDefaultPrng};
use hpke_rs_rust_crypto::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(&format!("K256 Derive"), |b| {
        b.iter_batched(
            || {
                let (pk, sk) = HpkeRustCrypto::kem_key_gen(
                    KemAlgorithm::DhKemK256,
                    &mut HpkeRustCrypto::try_prng().unwrap(),
                )
                .unwrap();
                (sk.clone(), pk.clone())
            },
            |(sk, pk)| {
                let _ = HpkeRustCrypto::dh(KemAlgorithm::DhKemK256, &pk, &sk);
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function(&format!("K256 Derive Base"), |b| {
        b.iter_batched(
            || {
                let (_pk, sk) = HpkeRustCrypto::kem_key_gen(
                    KemAlgorithm::DhKemK256,
                    &mut HpkeRustCrypto::try_prng().unwrap(),
                )
                .unwrap();
                sk.clone()
            },
            |sk| {
                let _pk = HpkeRustCrypto::secret_to_public(KemAlgorithm::DhKemK256, &sk).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);

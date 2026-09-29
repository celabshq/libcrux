use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use hpke_rs_crypto::{types::*, HpkeCrypto};
use hpke_rs_libcrux::*;
use rand::TryRng;

type DefaultLibcrux = HpkeLibcrux<HpkeLibcruxPrng>;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(&format!("HKDF SHA256 Extract"), |b| {
        b.iter_batched(
            || {
                let mut salt = vec![0u8; 77];
                let mut ikm = vec![0u8; 32];
                rand::rngs::SysRng.try_fill_bytes(&mut salt).unwrap();
                rand::rngs::SysRng.try_fill_bytes(&mut ikm).unwrap();
                (salt.clone(), ikm.clone())
            },
            |(salt, ikm)| {
                let _ = DefaultLibcrux::kdf_extract(TwoStageKdfAlgorithm::HkdfSha256, &salt, &ikm);
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function(&format!("HKDF SHA256 Expand"), |b| {
        b.iter_batched(
            || {
                let mut info = vec![0u8; 77];
                let mut prk = vec![0u8; 32];
                rand::rngs::SysRng.try_fill_bytes(&mut info).unwrap();
                rand::rngs::SysRng.try_fill_bytes(&mut prk).unwrap();
                (prk.clone(), info.clone())
            },
            |(prk, info)| {
                let _ =
                    DefaultLibcrux::kdf_expand(TwoStageKdfAlgorithm::HkdfSha256, &prk, &info, 32);
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);

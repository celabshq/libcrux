//! Benchmarks for classic (DH-based) HPKE ciphersuites.
//!
//! Run with: `cargo bench --bench bench --features libcrux,rustcrypto`

use criterion::{criterion_group, criterion_main, Criterion};
use hpke_rs::prelude::*;
use hpke_rs_crypto::{
    types::{KdfAlgorithm, KemAlgorithm},
    HpkeDefaultPrng,
};
use hpke_rs_libcrux::HpkeLibcrux;
use hpke_rs_rust_crypto::*;

mod common;
use common::{bench_suite, AEAD_IDS};

const MODES: [Mode; 4] = [
    HpkeMode::Base,
    HpkeMode::Auth,
    HpkeMode::Psk,
    HpkeMode::AuthPsk,
];
const KDF_IDS: [KdfAlgorithm; 3] = [
    KdfAlgorithm::HkdfSha256,
    KdfAlgorithm::HkdfSha384,
    KdfAlgorithm::HkdfSha512,
];
const KEM_IDS: [KemAlgorithm; 6] = [
    KemAlgorithm::DhKemP256,
    KemAlgorithm::DhKemK256,
    KemAlgorithm::DhKemP384,
    KemAlgorithm::DhKemP521,
    KemAlgorithm::DhKem25519,
    KemAlgorithm::DhKem448,
];

fn benchmark_classic<Crypto: HpkeDefaultPrng + 'static>(c: &mut Criterion) {
    for hpke_mode in MODES {
        for aead_mode in AEAD_IDS {
            if Crypto::supports_aead(aead_mode).is_err() {
                continue;
            }
            for kdf_mode in KDF_IDS {
                if Crypto::supports_kdf(kdf_mode).is_err() {
                    continue;
                }
                for kem_mode in KEM_IDS {
                    if Crypto::supports_kem(kem_mode).is_err() {
                        continue;
                    }
                    // Classic suites do not separately time key generation.
                    bench_suite::<Crypto>(c, hpke_mode, kem_mode, kdf_mode, aead_mode, false);
                }
            }
        }
    }
}

fn bench_libcrux(c: &mut Criterion) {
    benchmark_classic::<HpkeLibcrux>(c);
}

fn bench_rust_crypto(c: &mut Criterion) {
    benchmark_classic::<HpkeRustCrypto>(c);
}

criterion_group!(benches, bench_libcrux, bench_rust_crypto,);
criterion_main!(benches);

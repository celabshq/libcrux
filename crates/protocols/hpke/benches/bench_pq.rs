//! Benchmarks for post-quantum HPKE ciphersuites (draft-ietf-hpke-pq).
//!
//! Run with: `cargo bench --bench bench_pq --features draft-ietf-hpke-pq,libcrux`

use criterion::{criterion_group, criterion_main, Criterion};
use hpke_rs::prelude::*;
use hpke_rs_crypto::{
    types::{KdfAlgorithm, KemAlgorithm},
    HpkeDefaultPrng,
};
use hpke_rs_libcrux::HpkeLibcrux;

mod common;
use common::{bench_suite, AEAD_IDS};

/// One two-stage (HKDF) and one single-stage (SHAKE) KDF, to cover both
/// key-schedule shapes for the post-quantum suites.
const PQ_KDF_IDS: [KdfAlgorithm; 2] = [KdfAlgorithm::HkdfSha256, KdfAlgorithm::Shake256];

/// Post-quantum KEMs from `draft-ietf-hpke-pq`.
const PQ_KEM_IDS: &[KemAlgorithm] = &[
    KemAlgorithm::MlKem512,
    KemAlgorithm::MlKem768,
    KemAlgorithm::MlKem1024,
    KemAlgorithm::MlKem768P256,
    #[cfg(feature = "libcrux-rustcrypto-p-curves")]
    KemAlgorithm::MlKem1024P384,
    KemAlgorithm::XWingDraft06,
];

/// Benchmark the post-quantum ciphersuites.
///
/// Restricted to `Base` mode: the PQ KEMs return `UnsupportedKemOperation` for the
/// `Auth`/`AuthPsk` modes, and PSK is out of scope here. Key generation is timed in
/// addition to the usual operations.
fn benchmark_post_quantum<Crypto: HpkeDefaultPrng + 'static>(c: &mut Criterion) {
    for aead_mode in AEAD_IDS {
        if Crypto::supports_aead(aead_mode).is_err() {
            continue;
        }
        for kdf_mode in PQ_KDF_IDS {
            if Crypto::supports_kdf(kdf_mode).is_err() {
                continue;
            }
            for &kem_mode in PQ_KEM_IDS {
                if Crypto::supports_kem(kem_mode).is_err() {
                    continue;
                }
                bench_suite::<Crypto>(c, HpkeMode::Base, kem_mode, kdf_mode, aead_mode, true);
            }
        }
    }
}

fn bench_pq(c: &mut Criterion) {
    benchmark_post_quantum::<HpkeLibcrux>(c);
}

criterion_group!(benches, bench_pq,);
criterion_main!(benches);

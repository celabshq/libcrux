use libcrux_ml_dsa::{
    ml_dsa_87::{self, MLDSA87KeyPair, MLDSA87Signature},
    KEY_GENERATION_RANDOMNESS_SIZE, SIGNING_RANDOMNESS_SIZE,
};

mod bench_utils;

fn main() {
    bench_group_libcrux!(
        "87",
        "portable",
        ml_dsa_87::portable,
        MLDSA87KeyPair,
        MLDSA87Signature
    );
    #[cfg(feature = "simd128")]
    bench_group_libcrux!(
        "87",
        "neon",
        ml_dsa_87::neon,
        MLDSA87KeyPair,
        MLDSA87Signature
    );
    #[cfg(feature = "simd256")]
    bench_group_libcrux!(
        "87",
        "avx2",
        ml_dsa_87::avx2,
        MLDSA87KeyPair,
        MLDSA87Signature
    );

    #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
    bench_group_pqclean!("87", mldsa87);
}

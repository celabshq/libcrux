use rand::TryRngCore;

#[allow(unused)]
pub(crate) fn random_array<const L: usize>() -> [u8; L] {
    let mut rng = rand::rngs::OsRng;
    let mut seed = [0; L];
    rng.try_fill_bytes(&mut seed).unwrap();
    seed
}

#[allow(unused)]
pub(crate) fn print_time(label: &str, d: std::time::Duration) {
    let micros = d.as_micros();
    let time = if micros < MILLI_PER_ITERATION_THRESHOLD {
        format!("{} μs/iter", micros / ITERATIONS as u128)
    } else if micros < SECOND_PER_ITERATION_THRESHOLD {
        format!(
            "{:.2} ms/iter",
            (micros as f64 / (MICROS_PER_MILLI * ITERATIONS as f64))
        )
    } else {
        format!(
            "{:.2}s/iter",
            (micros as f64 / (MICROS_PER_SECOND * ITERATIONS as f64))
        )
    };
    let space = if label.len() < 6 {
        "\t\t".to_string()
    } else {
        "\t".to_string()
    };

    println!("{label} ... bench:{space}{time}");
}

pub(crate) const ITERATIONS: usize = 10_000;
#[allow(unused)]
pub(crate) const WARMUP_ITERATIONS: usize = 1_000;

pub(crate) const MICROS_PER_MILLI: f64 = 1_000.0;
pub(crate) const MICROS_PER_SECOND: f64 = 1_000_000.0;
pub(crate) const MILLI_PER_ITERATION_THRESHOLD: u128 = 1_000 * ITERATIONS as u128;
pub(crate) const SECOND_PER_ITERATION_THRESHOLD: u128 = 1_000_000 * ITERATIONS as u128;

// A benchmarking macro to avoid copying memory and skewing the results.
#[macro_export]
macro_rules! bench {
    ($implementation:literal, $fun_label:literal, $hardware:literal, $keysize:literal, $input:expr, $setup:expr, $routine:expr) => {{
        let mut time = std::time::Duration::ZERO;

        // Warmup
        for _ in 0..bench_utils::WARMUP_ITERATIONS {
            let input = $setup($input);
            let _ = $routine(input);
        }

        // Benchmark
        for _ in 0..bench_utils::ITERATIONS {
            let input = $setup($input);

            let start = std::time::Instant::now();
            let _ = core::hint::black_box($routine(input));
            let end = std::time::Instant::now();

            time += end.duration_since(start);
        }
        bench_utils::print_time(
            &format!(
                "test implementation={} ML-DSA,keySize={},label={},hardware={}",
                $implementation, $keysize, $fun_label, $hardware
            ),
            time,
        );
    }};
}

#[macro_export]
macro_rules! bench_group_libcrux {
    ($keysize:literal, $hardware:literal, $mod:path, $keypair_t:ident, $signature_t:ident) => {{
        use $mod as p;
        bench!(
            "libcrux",
            "KeyGen",
            $hardware,
            $keysize,
            (),
            |()| {
                let key_generation_seed: [u8; KEY_GENERATION_RANDOMNESS_SIZE] =
                    bench_utils::random_array();
                key_generation_seed
            },
            |key_generation_seed: [u8; KEY_GENERATION_RANDOMNESS_SIZE]| {
                p::generate_key_pair(key_generation_seed)
            }
        );

        bench!(
            "libcrux",
            "Sign",
            $hardware,
            $keysize,
            (),
            |()| {
                let key_generation_seed: [u8; KEY_GENERATION_RANDOMNESS_SIZE] =
                    bench_utils::random_array();
                let signing_randomness: [u8; SIGNING_RANDOMNESS_SIZE] = bench_utils::random_array();
                let message = bench_utils::random_array::<1023>();
                let keypair = p::generate_key_pair(key_generation_seed);

                (keypair, message, signing_randomness)
            },
            |(keypair, message, signing_randomness): (
                $keypair_t,
                [u8; 1023],
                [u8; SIGNING_RANDOMNESS_SIZE]
            )| { p::sign(&keypair.signing_key, &message, b"", signing_randomness) }
        );

        bench!(
            "libcrux",
            "Verify",
            $hardware,
            $keysize,
            (),
            |()| {
                let key_generation_seed: [u8; KEY_GENERATION_RANDOMNESS_SIZE] =
                    bench_utils::random_array();
                let signing_randomness: [u8; SIGNING_RANDOMNESS_SIZE] = bench_utils::random_array();
                let message = bench_utils::random_array::<1023>();
                let keypair = p::generate_key_pair(key_generation_seed);
                let signature =
                    p::sign(&keypair.signing_key, &message, b"", signing_randomness).unwrap();
                (keypair, message, signature)
            },
            |(keypair, message, signature): ($keypair_t, [u8; 1023], $signature_t)| {
                p::verify(&keypair.verification_key, &message, b"", &signature).unwrap()
            }
        );

        println!("");
    }};
}

#[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
#[macro_export]
macro_rules! bench_group_pqclean {
    ($variant:literal, $mod:ident) => {{
        bench!("pqclean", "KeyGen", "auto", $variant, (), |()| {}, |()| {
            pqcrypto_mldsa::$mod::keypair()
        });
        bench!(
            "pqclean",
            "Sign",
            "auto",
            $variant,
            (),
            |()| {
                let (_, sk) = pqcrypto_mldsa::$mod::keypair();
                let message = bench_utils::random_array::<1023>();
                (sk, message)
            },
            |(sk, message): (pqcrypto_mldsa::$mod::SecretKey, [u8; 1023])| {
                let _ = pqcrypto_mldsa::$mod::detached_sign(&message, &sk);
            }
        );
        bench!(
            "pqclean",
            "Verify",
            "auto",
            $variant,
            (),
            |()| {
                let (vk, sk) = pqcrypto_mldsa::$mod::keypair();
                let message = bench_utils::random_array::<1023>();
                let signature = pqcrypto_mldsa::$mod::detached_sign(&message, &sk);
                (vk, message, signature)
            },
            |(vk, message, signature): (
                pqcrypto_mldsa::$mod::PublicKey,
                [u8; 1023],
                pqcrypto_mldsa::$mod::DetachedSignature
            )| {
                let _ = pqcrypto_mldsa::$mod::verify_detached_signature(&signature, &message, &vk)
                    .unwrap();
            }
        );
    }};
}

/// Cross-spec tests: verify top-level SHA-3 / SHAKE functions against the
/// hacspec specification.  Lower-level tests (permutation steps, sponge
/// helpers) live as unit tests inside `src/generic_keccak.rs`.

/// Generate test inputs at boundary sizes around the given rate.
/// Returns empty, 1 byte, sub-rate, rate-1, rate, rate+1, 2*rate-1, 2*rate,
/// 2*rate+1, and a large multi-block input.
fn boundary_inputs(rate: usize) -> Vec<Vec<u8>> {
    [
        0,
        1,
        8,
        rate / 2,
        rate - 1,
        rate,
        rate + 1,
        2 * rate - 1,
        2 * rate,
        2 * rate + 1,
        5 * rate + 7,
    ]
    .into_iter()
    .map(|len| (0..len).map(|i| (i & 0xFF) as u8).collect())
    .collect()
}

// =========================================================================
// SHA3 hash functions — boundary input sizes for each rate
// =========================================================================

#[test]
fn sha3_224_boundary() {
    // rate = 144
    for input in boundary_inputs(144) {
        let spec = hacspec_sha3::sha3_224(&input);
        let mut out = [0u8; 28];
        libcrux_sha3::portable::sha224(&mut out, &input);
        assert_eq!(out, spec, "sha3_224 mismatch at len {}", input.len());
    }
}

#[test]
fn sha3_256_boundary() {
    // rate = 136
    for input in boundary_inputs(136) {
        let spec = hacspec_sha3::sha3_256(&input);
        let mut out = [0u8; 32];
        libcrux_sha3::portable::sha256(&mut out, &input);
        assert_eq!(out, spec, "sha3_256 mismatch at len {}", input.len());
    }
}

#[test]
fn sha3_384_boundary() {
    // rate = 104
    for input in boundary_inputs(104) {
        let spec = hacspec_sha3::sha3_384(&input);
        let mut out = [0u8; 48];
        libcrux_sha3::portable::sha384(&mut out, &input);
        assert_eq!(out, spec, "sha3_384 mismatch at len {}", input.len());
    }
}

#[test]
fn sha3_512_boundary() {
    // rate = 72
    for input in boundary_inputs(72) {
        let spec = hacspec_sha3::sha3_512(&input);
        let mut out = [0u8; 64];
        libcrux_sha3::portable::sha512(&mut out, &input);
        assert_eq!(out, spec, "sha3_512 mismatch at len {}", input.len());
    }
}

// =========================================================================
// SHAKE XOFs — boundary input sizes AND boundary output sizes
// =========================================================================

#[test]
fn shake128_boundary_inputs() {
    // rate = 168
    for input in boundary_inputs(168) {
        let spec = hacspec_sha3::shake128::<64>(&input);
        let mut out = [0u8; 64];
        libcrux_sha3::portable::shake128(&mut out, &input);
        assert_eq!(out, spec, "shake128 input mismatch at len {}", input.len());
    }
}

#[test]
fn shake256_boundary_inputs() {
    // rate = 136
    for input in boundary_inputs(136) {
        let spec = hacspec_sha3::shake256::<64>(&input);
        let mut out = [0u8; 64];
        libcrux_sha3::portable::shake256(&mut out, &input);
        assert_eq!(out, spec, "shake256 input mismatch at len {}", input.len());
    }
}

#[test]
fn shake128_boundary_outputs() {
    let input = b"boundary output test";
    let rate = 168;
    let spec_long = hacspec_sha3::shake128::<1024>(input);
    // Test output lengths around the rate boundary (squeeze multi-block)
    for &out_len in &[
        1,
        8,
        rate / 2,
        rate - 1,
        rate,
        rate + 1,
        2 * rate,
        3 * rate + 7,
    ] {
        let mut out = [0u8; 1024];
        libcrux_sha3::portable::shake128(&mut out[..out_len], input);
        assert_eq!(
            out[..out_len],
            spec_long[..out_len],
            "shake128 output mismatch at out_len {out_len}"
        );
    }
}

#[test]
fn shake256_boundary_outputs() {
    let input = b"boundary output test";
    let rate = 136;
    let spec_long = hacspec_sha3::shake256::<1024>(input);
    for &out_len in &[
        1,
        8,
        rate / 2,
        rate - 1,
        rate,
        rate + 1,
        2 * rate,
        3 * rate + 7,
    ] {
        let mut out = [0u8; 1024];
        libcrux_sha3::portable::shake256(&mut out[..out_len], input);
        assert_eq!(
            out[..out_len],
            spec_long[..out_len],
            "shake256 output mismatch at out_len {out_len}"
        );
    }
}

// =========================================================================
// Squeeze structure — exercises the three branches of the split squeeze
// (output_blocks == 0 / rem == 0 / rem != 0), the F* proof's structural
// claim in Phase 16.  See specs/sha3/src/sponge.rs keccak().
// =========================================================================

/// For a given SHAKE rate, pick output lengths that fall into each of the
/// three structural branches.
fn squeeze_structure_lengths(rate: usize) -> Vec<(usize, &'static str)> {
    vec![
        // output_blocks == 0 (output_len < rate)
        (1, "zero-blocks: len=1"),
        (rate / 2, "zero-blocks: len=rate/2"),
        (rate - 1, "zero-blocks: len=rate-1"),
        // output_blocks >= 1, output_rem == 0 (exact multiple of rate)
        (rate, "exact: len=rate"),
        (2 * rate, "exact: len=2*rate"),
        (3 * rate, "exact: len=3*rate"),
        // output_blocks >= 1, output_rem != 0 (multiple + nonzero remainder)
        (rate + 1, "rem: len=rate+1"),
        (2 * rate + 7, "rem: len=2*rate+7"),
        (3 * rate + (rate - 1), "rem: len=3*rate+rate-1"),
    ]
}

#[test]
fn shake128_squeeze_structure() {
    let input = b"squeeze structure test";
    let rate = 168;
    // Spec call emits the longest output; slice to compare for each case.
    let spec_long = hacspec_sha3::shake128::<2048>(input);
    for (out_len, label) in squeeze_structure_lengths(rate) {
        assert!(out_len <= 2048);
        let mut out = [0u8; 2048];
        libcrux_sha3::portable::shake128(&mut out[..out_len], input);
        assert_eq!(
            out[..out_len],
            spec_long[..out_len],
            "shake128 structure mismatch ({label})"
        );
    }
}

#[test]
fn shake256_squeeze_structure() {
    let input = b"squeeze structure test";
    let rate = 136;
    let spec_long = hacspec_sha3::shake256::<2048>(input);
    for (out_len, label) in squeeze_structure_lengths(rate) {
        assert!(out_len <= 2048);
        let mut out = [0u8; 2048];
        libcrux_sha3::portable::shake256(&mut out[..out_len], input);
        assert_eq!(
            out[..out_len],
            spec_long[..out_len],
            "shake256 structure mismatch ({label})"
        );
    }
}

// =========================================================================
// EMA variants match return variants
// =========================================================================

#[test]
fn ema_variants_match_return_variants() {
    let input = b"test ema vs return";

    let ret = libcrux_sha3::sha224(input);
    let mut ema = [0u8; 28];
    libcrux_sha3::sha224_ema(&mut ema, input);
    assert_eq!(ret, ema, "sha224");

    let ret = libcrux_sha3::sha256(input);
    let mut ema = [0u8; 32];
    libcrux_sha3::sha256_ema(&mut ema, input);
    assert_eq!(ret, ema, "sha256");

    let ret = libcrux_sha3::sha384(input);
    let mut ema = [0u8; 48];
    libcrux_sha3::sha384_ema(&mut ema, input);
    assert_eq!(ret, ema, "sha384");

    let ret = libcrux_sha3::sha512(input);
    let mut ema = [0u8; 64];
    libcrux_sha3::sha512_ema(&mut ema, input);
    assert_eq!(ret, ema, "sha512");

    let ret = libcrux_sha3::shake128::<64>(input);
    let mut ema = [0u8; 64];
    libcrux_sha3::shake128_ema(&mut ema, input);
    assert_eq!(ret, ema, "shake128");

    let ret = libcrux_sha3::shake256::<64>(input);
    let mut ema = [0u8; 64];
    libcrux_sha3::shake256_ema(&mut ema, input);
    assert_eq!(ret, ema, "shake256");
}

// =========================================================================
// Generic hash dispatch
// =========================================================================

#[test]
fn hash_dispatch_matches_direct() {
    let input = b"dispatch test";
    assert_eq!(
        libcrux_sha3::hash::<28>(libcrux_sha3::Algorithm::Sha224, input),
        libcrux_sha3::sha224(input)
    );
    assert_eq!(
        libcrux_sha3::hash::<32>(libcrux_sha3::Algorithm::Sha256, input),
        libcrux_sha3::sha256(input)
    );
    assert_eq!(
        libcrux_sha3::hash::<48>(libcrux_sha3::Algorithm::Sha384, input),
        libcrux_sha3::sha384(input)
    );
    assert_eq!(
        libcrux_sha3::hash::<64>(libcrux_sha3::Algorithm::Sha512, input),
        libcrux_sha3::sha512(input)
    );
}

// =========================================================================
// NEON (simd128) — cross-spec for all hash functions
// =========================================================================

#[cfg(feature = "simd128")]
mod neon_cross_spec {
    use super::boundary_inputs;

    #[test]
    fn sha3_224_boundary() {
        for input in boundary_inputs(144) {
            let spec = hacspec_sha3::sha3_224(&input);
            let mut out = [0u8; 28];
            libcrux_sha3::neon::sha224(&mut out, &input);
            assert_eq!(out, spec, "neon sha3_224 mismatch at len {}", input.len());
        }
    }

    #[test]
    fn sha3_256_boundary() {
        for input in boundary_inputs(136) {
            let spec = hacspec_sha3::sha3_256(&input);
            let mut out = [0u8; 32];
            libcrux_sha3::neon::sha256(&mut out, &input);
            assert_eq!(out, spec, "neon sha3_256 mismatch at len {}", input.len());
        }
    }

    #[test]
    fn sha3_384_boundary() {
        for input in boundary_inputs(104) {
            let spec = hacspec_sha3::sha3_384(&input);
            let mut out = [0u8; 48];
            libcrux_sha3::neon::sha384(&mut out, &input);
            assert_eq!(out, spec, "neon sha3_384 mismatch at len {}", input.len());
        }
    }

    #[test]
    fn sha3_512_boundary() {
        for input in boundary_inputs(72) {
            let spec = hacspec_sha3::sha3_512(&input);
            let mut out = [0u8; 64];
            libcrux_sha3::neon::sha512(&mut out, &input);
            assert_eq!(out, spec, "neon sha3_512 mismatch at len {}", input.len());
        }
    }

    #[test]
    fn shake128_boundary() {
        for input in boundary_inputs(168) {
            let spec = hacspec_sha3::shake128::<64>(&input);
            let mut out = [0u8; 64];
            libcrux_sha3::neon::shake128(&mut out, &input);
            assert_eq!(out, spec, "neon shake128 mismatch at len {}", input.len());
        }
    }

    #[test]
    fn shake256_boundary() {
        for input in boundary_inputs(136) {
            let spec = hacspec_sha3::shake256::<64>(&input);
            let mut out = [0u8; 64];
            libcrux_sha3::neon::shake256(&mut out, &input);
            assert_eq!(out, spec, "neon shake256 mismatch at len {}", input.len());
        }
    }

    #[test]
    fn neon_x2_shake256_matches_spec() {
        for input in boundary_inputs(136) {
            let spec = hacspec_sha3::shake256::<64>(&input);
            let mut out0 = [0u8; 64];
            let mut out1 = [0u8; 64];
            libcrux_sha3::neon::x2::shake256(&input, &input, &mut out0, &mut out1);
            assert_eq!(
                out0,
                spec,
                "neon x2 shake256 lane0 mismatch at len {}",
                input.len()
            );
            assert_eq!(
                out1,
                spec,
                "neon x2 shake256 lane1 mismatch at len {}",
                input.len()
            );
        }
    }
}

// =========================================================================
// AVX2 (simd256) — cross-spec via x4 incremental API
// =========================================================================

#[cfg(feature = "simd256")]
mod avx2_cross_spec {
    #[test]
    fn avx2_x4_shake256_matches_spec() {
        let inputs: [&[u8]; 4] = [b"alpha", b"beta!", b"gamma", b"delta"];
        let mut state = libcrux_sha3::avx2::x4::incremental::init();
        libcrux_sha3::avx2::x4::incremental::shake256_absorb_final(
            &mut state, inputs[0], inputs[1], inputs[2], inputs[3],
        );
        let mut out0 = [0u8; 136];
        let mut out1 = [0u8; 136];
        let mut out2 = [0u8; 136];
        let mut out3 = [0u8; 136];
        libcrux_sha3::avx2::x4::incremental::shake256_squeeze_first_block(
            &mut state, &mut out0, &mut out1, &mut out2, &mut out3,
        );
        for (i, (out, input)) in [out0, out1, out2, out3]
            .iter()
            .zip(inputs.iter())
            .enumerate()
        {
            let spec = hacspec_sha3::shake256::<136>(input);
            assert_eq!(&out[..], &spec[..], "avx2 x4 shake256 lane {i} mismatch");
        }
    }

    #[test]
    fn avx2_x4_shake128_matches_spec() {
        let inputs: [&[u8]; 4] = [b"oneone", b"twotwo", b"three!", b"four!!"];
        let mut state = libcrux_sha3::avx2::x4::incremental::init();
        libcrux_sha3::avx2::x4::incremental::shake128_absorb_final(
            &mut state, inputs[0], inputs[1], inputs[2], inputs[3],
        );
        // Squeeze 3 blocks (3 * 168 = 504 bytes)
        let mut out0 = [0u8; 504];
        let mut out1 = [0u8; 504];
        let mut out2 = [0u8; 504];
        let mut out3 = [0u8; 504];
        libcrux_sha3::avx2::x4::incremental::shake128_squeeze_first_three_blocks(
            &mut state, &mut out0, &mut out1, &mut out2, &mut out3,
        );
        for (i, (out, input)) in [out0, out1, out2, out3]
            .iter()
            .zip(inputs.iter())
            .enumerate()
        {
            let spec = hacspec_sha3::shake128::<504>(input);
            assert_eq!(&out[..], &spec[..], "avx2 x4 shake128 lane {i} mismatch");
        }
    }

    #[test]
    fn avx2_x4_shake256_multi_squeeze_matches_spec() {
        let input = b"multi-squeeze test";
        let spec = hacspec_sha3::shake256::<408>(input);

        let mut state = libcrux_sha3::avx2::x4::incremental::init();
        libcrux_sha3::avx2::x4::incremental::shake256_absorb_final(
            &mut state, input, input, input, input,
        );

        // First block: 136 bytes
        let mut b1 = [0u8; 136];
        let mut d1 = [0u8; 136];
        let mut d2 = [0u8; 136];
        let mut d3 = [0u8; 136];
        libcrux_sha3::avx2::x4::incremental::shake256_squeeze_first_block(
            &mut state, &mut b1, &mut d1, &mut d2, &mut d3,
        );
        assert_eq!(&b1[..], &spec[..136], "first block");

        // Second block: 136 bytes
        let mut b2 = [0u8; 136];
        libcrux_sha3::avx2::x4::incremental::shake256_squeeze_next_block(
            &mut state, &mut b2, &mut d1, &mut d2, &mut d3,
        );
        assert_eq!(&b2[..], &spec[136..272], "second block");

        // Third block: 136 bytes
        let mut b3 = [0u8; 136];
        libcrux_sha3::avx2::x4::incremental::shake256_squeeze_next_block(
            &mut state, &mut b3, &mut d1, &mut d2, &mut d3,
        );
        assert_eq!(&b3[..], &spec[272..408], "third block");
    }
}

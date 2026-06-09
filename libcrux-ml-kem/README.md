# ML-KEM

This crate implements all three ML-KEM ([FIPS 203](https://csrc.nist.gov/pubs/fips/203/final)) variants 512, 768, and 1024.
For each algorithm, it inlcudes a portable Rust implementation, as well as SIMD implementations for Intel AVX2 and AArch64 Neon platforms.

## Module structure
The modules

- `mlkem512`
- `mlkem768`
- `mlkem1024`

implement the three parameter sets for ML-KEM defined in FIPS 203. 

Each module provides the following API:
- `generate_key_pair`: to generate an ML-KEM key pair,
- `encapsulate`: to encapsulate a shared secret towards a given ML-KEM public key,
- `decapsulate`: to decapsulate a shared secret from a ciphertext using an ML-KEM private key,
- `validate_public_key`: to perform validation of public keys as required by FIPS 203 prior to encapsulation,
- `validate_private_key`: to perform validation of private keys and ciphertexts as required by FIPS 203 prior to decapsulation.

### Portable and Optimized Implementations
The crate provides portable, as well as AVX2- and NEON-optimized
implementations of the above API. By defautl, the crate's `build.rs`
will include the portable implementation and one of the optimized
implementations in the build, according to the value of
`CARGO_CFG_TARGET_ARCH`.

In addition, the above functions perform CPU feature detection at
runtime to ensure the most efficient implementation for the given
platform is selected.

It is recommended to rely on the automatic feature detection, but specific
builds can be forced by setting environment variables,
`LIBCRUX_ENABLE_SIMD128=1` or `LIBCRUX_ENABLE_SIMD256=1`.

Usage example:
```rust
 #[cfg(feature = "mlkem768")]
 {
     use rand::{rngs::SysRng, TryRng};

     // Ensure you use good randomness.
     // It is not recommended to use OsRng directly!
     // Instead it is highly encouraged to use RNGs like NISTs DRBG to account for
     // bad system entropy.
     fn random_array<const L: usize>() -> [u8; L] {
         let mut rng = SysRng;
         let mut seed = [0; L];
         rng.try_fill_bytes(&mut seed).unwrap();
         seed
     }

     use libcrux_ml_kem::*;

     // This example uses ML-KEM 768. The other variants can be used the same way.

     // Generate a key pair.
     let key_pair = {
        let randomness = random_array();
        mlkem768::generate_key_pair(randomness)
     };

     // Encapsulating a shared secret to a public key.
     let (ciphertext, shared_secret) = {
        let randomness = random_array();
        mlkem768::encapsulate(key_pair.public_key(), randomness)
     };

     // Decapsulating a shared secret with a private key.
     let shared_secret_decapsulated = mlkem768::decapsulate(key_pair.private_key(), &ciphertext);

     assert_eq!(shared_secret_decapsulated, shared_secret);
 }
```

### Unpacked APIs
The default KEM API described above operates on serialized keys,
i.e. `encapsulate` will take a serialized ML-KEM public key as input
and `decapsulate` will take a serialized ML-KEM private key as input,
and these must be validated before use with `validate_public_key`
and `validate_private_key` for FIPS 203 compliance.

In addition, in each parameter set module, (e.g. `mlkem768`) the crate
provides an API for working with "unpacked" keys, which have already
been deserialized. For some applications it may thus be advantageous
to validate key material once, then deserialized into unpacked
representation once, and to use the the already validated and
deserialized form from then on.

The unpacked APIs are platform dependent, so they can be found in
submodules `mlkem768::portable::unpacked`, `mlkem768::avx2::unpacked`,
`mlkem768::neon::unpacked`, depending on which of these platform
specific modules are part of the build in question.

### PQ Code Package Common APIs
As part of the PQ code package project (PQCP), the crate also
implements the common API on packed keys outlined in that project for each parameter
set in the respective sub-module `pqcp`:

- `crypto_kem_keypair_derand`: To generate an ML-KEM key pair, provided external randonmess.
- `crypto_kem_keypair` (with feature `rand`): To generate an ML-KEM key pair, sampling randomness internally.
- `crypto_kem_enc_derand`: To encapsulate a shared secret, provided external randomness.
- `crypto_kem_enc` (with feature `rand`): To encapsulate a shared secret, sampling randomness internally.
- `crypto_kem_dec`: To decapsulate a shared secret.

Additionally, the above API is available for each platform-dependent unpacked key type, with function names suffixed by `_struct` and the following functions are provided for working with unpacked keys:

- `crypto_kem_marshal_pk`: For serializing unpacked encapsulation keys.
- `crypto_kem_marhsal_sk`: For serializing unpacked decapsulation keys.
- `crypto_kem_parse_pk`: For deserializing an unpacked encapsulation key. Performs key validation.
- `crypto_kem_parse_sk`: For deserializing an unpacked decapsulation key. Performs key validation.
- `crypto_kem_pk_from_seed`: For generating an unpacked decapsulation key from a seed.
- `crypto_kem_sk_from_seed`: For deriving an unpacked encapsulation key from an unpacked decapsulation key.

The API is available through feature `pqcp`.

### Crate Features
The crate provides the following features.

Default features:
- `mlkem512`, `mlkem768` & `mlkem1024`: These can be used to select
  individual parameter sets. By default, all parameter sets are
  included.
- `rand`: Whereas the default APIs for `generate_key_pair` and
  `encapsulate` expect external provision of random bytes, this
  feature enables randomized versions of these APIs (in submodules
  `mlkem512::rand`, `mlkem768::rand`, `mlkem1024::rand`) which take an
  `&mut impl rand_core::CryptoRng` argument to generate the required
  randomness internally.
- `default-no-std` & `std`: Disabling default feature `std` provides
  `no_std` support. For convenience `default-no-std` collects all default
  features except `std`.
  
Additional features:
- `kyber`: Provides access to an, as yet, unverified implementation of
  Kyber as submitted in Round 3 of the NIST PQ competition. The Kyber
  APIs follow the general structure of the ML-KEM APIs.
- `check-secret-independence`: All operations on ring elements in the
  portable implementation use the integer types of the
  [`libcrux-secrets`](https://crates.io/crates/libcrux-secrets) crate under the hood. That crate allows checking a
  program operating on these types for secret independence at compile
  time. Enabling the `check-secret-independence` feature switches on
  this compile-time checking of secret independence. By default, the
  integer types of `libcrux-secrets` transparently fall back on Rust's
  standard integer types.
- `simd128`, `simd256`: These features force a compilation for NEON or
  AVX2 targets, respectively, as discussed above.
- `incremental`: An experimental API, which allows for incremental
  encapsulation.
- `pqcp`: See above.
- `codec`: Use this feature to enable (de-)serialization of (packed)
  public keys and ciphertexts using
  [`tls_codec`](https://crates.io/crates/tls_codec).

## Secret Independence
As outlined in the description of the `check-secret-independence`
feature above, we leverage the Rust type system to ensure that secret
values are not used in operations that are known to have
data-dependent execution time. While the implementation of constant time
operations is best-effort, as there are no guarantees from the
compiler, we follow established constant-time patterns and validate
constant-time code via inspection of the generated assembly
instructions.

## Verification
![verified]

The portable and AVX2 code for field arithmetic, NTT polynomial arithmetic, serialization, and the generic code for high-level algorithms
is formally verified using [hax](https://cryspen.com/hax) and  [F*](https://fstar-lang.org).

Please refer to [this file](proofs/verification_status.md) for detail on the verification of this crate.

[verified]: ../.assets/verified-brightgreen.svg


# ML-DSA

This crate implements all three ML-DSA ([FIPS 204](https://csrc.nist.gov/pubs/fips/204/final)) variants 44, 65, and 87, and includes
both a portable implementation and an optimized SIMD implementation for Intel AVX2-enabled platforms.

## Verification
![verified]

The portable and AVX2 code for field arithmetic, NTT polynomial arithmetic, and serialization is formally verified using [hax](https://cryspen.com/hax) and
 [F*](https://fstar-lang.org).

## Usage

```Rust
 use rand::{rngs::OsRng, RngCore};

 // Ensure you use good randomness.
 // It is not recommended to use OsRng directly!
 // Instead it is highly encouraged to use RNGs like NISTs DRBG to account for
 // bad system entropy.
 fn random_array<const L: usize>() -> [u8; L] {
     let mut rng = OsRng;
     let mut seed = [0; L];
     rng.try_fill_bytes(&mut seed).unwrap();
     seed
 }

 use libcrux_ml_dsa::*;

 // This example uses ML-DSA-65. The other variants can be used the same way.

 // Generate a key pair.
 let randomness = random_array();
 let key_pair = ml_dsa_65::generate_key_pair(randomness);

 // Generate a random message.
 let message = random_array::<1024>();

 // Sign this random message
 let randomness = random_array();
 let signature = ml_dsa_65::sign(key_pair.signing_key, &message, randomness);

 // Verify the signature and assert that it is indeed valid
 assert!(ml_dsa_65::verify(key_pair.verification_key, &message, signature).is_ok());
```

[verified]: ../.assets/verified-brightgreen.svg

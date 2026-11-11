# KATs

This crate provides KAT test vectors for:
- ML-DSA (wycheproof)
    - sign (`noseed`)
    - verify
- ML-DSA (acvp)
    - keygen
    - sign, and sign `FIPS204-tr1` (`siggen-tr1`, with seed keys)
    - verify
- HKDF SP 800-56Cr2 KDA (acvp, generated)
    - SHA2-256/384/512 and SHA3-224/256/384/512, fixed info `uPartyInfo||vPartyInfo||l`
- KMAC-128/256 (acvp, generated)
    - AFT, MVT, without XOF
- SP 800-108 KBKDF (acvp)
    - feedback mode, HMAC, 32-bit counter before the fixed data
- SP 800-56Cr2 two-step KDA (acvp, generated)
    - HMAC, feedback mode expansion, 32-bit counter before the fixed data
    - `3kem-*`: the parameters used by 3KEM (fixed info `label||00||uPartyInfo||vPartyInfo||l`, empty IV)
- ML-KEM (acvp)
    - keygen
    - encap-decap
- SHA3-224/256/384/512 (acvp, generated)
    - AFT, MCT, LDT
- SHAKE-128/256 (acvp, generated)
    - AFT, MCT, VOT
- HMAC-SHA3-224/256/384/512 (acvp, generated)
- AES (acvp, generated)
    - AES-128/256-ECB encrypt: AFT, MCT
    - AES-128/256-GCM: 96-bit IV, 128-bit tag
    - AES-128/256-CCM: 96-bit nonce, 64 and 128-bit tags
- KAS-ECC-SSC SP 800-56Ar3 (acvp, generated)
    - P-256 and P-384, `ephemeralUnified` and `onePassDh`
- ECDSA FIPS 186-5 (acvp, generated)
    - P-256 and P-384 keyGen (testing candidates, sample) and keyVer
    - P-256 sigGen (sample) and sigVer with SHA2-256/384/512
- SHA2-224/256/384/512 (acvp, generated)
    - AFT, MCT, LDT
- HMAC-SHA2-256/384/512 (acvp, generated)
- HMAC_DRBG SP 800-90Ar1 (acvp, generated)
    - SHA2-256/384/512, reseed, no prediction resistance
- EdDSA (acvp, generated)
    - Ed25519 (pure) keyGen (sample), sigGen (sample) and sigVer
- XECDH RFC 7748 (acvp, generated)
    - Curve25519 keyGen (sample), keyVer and SSC (sample)
- ML-KEM (wycheproof)
    - keygen/decaps
    - encaps
- p256 (wycheproof)
    - ECDH
    - ECDSA
- poly1305 (boringssl)
- SHA-2 (NIST)

⚠️ NOTE: This crate serves as an internal testing dependency for other `libcrux`
crates, and is not intended to be used directly.

## ACVP helpers

Besides the vector sets, `acvp` provides helpers for tests and the ACVP harness that are
independent of the primitive, which is passed in as a closure:

- `schema_common`: `find_group`/`find_test` lookups for any test group type, `assert_covers` to
  check that the prompts and expected results contain the same test cases, and `Tally` to assert
  exact numbers of tested and skipped test cases per category.
- Monte Carlo test drivers (`hash::{sha2_mct, sha3_mct, shake_mct}`, `aes::ecb_mct`), the LDT
  message expansion (`LargeMessage::{expand, for_each_chunk}`), the KDA fixed info
  (`SingleExpansionPrompt::fixed_info`), the KAS key selection by scheme and role, and
  big-endian padding of curve points and scalars (`ec`).
- The result structs implement `Serialize`, with upper case hex, so that responses can be
  written in the format of `expectedResults.json`.

## Source

The JSON files for wycheproof were taken from `https://github.com/C2SP/wycheproof`
* As of commit [6d9d6de30f02e229dfc160323722c3ddac866181](https://github.com/C2SP/wycheproof/tree/6d9d6de30f02e229dfc160323722c3ddac866181)

The JSON files for ACVP ML-KEM and ML-DSA were taken from `https://github.com/usnistgov/ACVP-Server`
* As of commit [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d) (release v1.1.0.43)
* ML-KEM encap-decap from `ML-KEM-encapDecap-FIPS203-tr1` and ML-DSA `siggen-tr1` from `ML-DSA-sigGen-FIPS204-tr1`; all others from the sets without the `-tr1` suffix

The JSON files for ACVP SP 800-108 KBKDF were taken from `https://github.com/usnistgov/ACVP-Server` (`gen-val/json-files/KDF-1.0`)
* As of commit [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d)
* Filtered to the test groups listed in `src/acvp/kbkdf.rs`

The JSON files for ACVP SP 800-56Cr2 two-step KDA, SHA2, SHA3, SHAKE, HMAC-SHA2, HMAC-SHA3, AES-ECB, AES-GCM, AES-CCM, KAS-ECC-SSC, ECDSA keyGen/keyVer/sigGen/sigVer, EdDSA, XECDH, HMAC_DRBG, HKDF and KMAC were generated locally with the `GenValAppRunner` of `https://github.com/usnistgov/ACVP-Server`
* As of commit [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d)
* Using the `registration.json` next to each `prompt.json`
* The `prompt.json`, `expectedResults.json` and, for KAS-ECC-SSC, `internalProjection.json` files are minified
* KAS-ECC-SSC is generated as a sample, because the AFT expected results depend on the IUT key pair, which is only contained in the `internalProjection.json`

The JSON files for ACVP EdDSA and XECDH were generated the same way. EdDSA sigGen and XECDH SSC are generated as samples, and include the `internalProjection.json` with the IUT key pairs.

To regenerate a vector set from its registration (see `../Readme.md`):

```sh
cargo xtask acvp generate crates/testing/kats/acvp/sha3-2_0/sha3-256/registration.json
# KAS-ECC-SSC also needs the internalProjection.json
cargo xtask acvp generate --keep-projection crates/testing/kats/acvp/kas-ecc-ssc-sp800-56ar3/p-256/registration.json
```

The generated vectors are random, so regenerating changes them.

The RSP files for SHA-2 were taken from NISTs [Cryptographic Algorithm Validation Program](https://csrc.nist.gov/projects/cryptographic-algorithm-validation-program/secure-hashing) (accessed 05.05.2026).

The test vectors for poly1305 are taken from [boringssl](https://github.com/google/boringssl/blob/2a8e86174536b735a777a56897c7949d33bd46a6/crypto/poly1305/poly1305_tests.txt).

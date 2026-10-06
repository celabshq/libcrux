# KATs

This crate provides KAT test vectors for:
- ML-DSA (wycheproof)
    - sign (`noseed`)
    - verify
- ML-DSA (acvp)
    - keygen
    - sign
    - verify
- SP 800-108 KBKDF (acvp)
    - feedback mode, HMAC, 32-bit counter before the fixed data
- SP 800-56Cr2 two-step KDA (acvp, generated)
    - HMAC, feedback mode expansion, 32-bit counter before the fixed data
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

## Source

The JSON files for wycheproof were taken from `https://github.com/C2SP/wycheproof`
* As of commit [6d9d6de30f02e229dfc160323722c3ddac866181](https://github.com/C2SP/wycheproof/tree/6d9d6de30f02e229dfc160323722c3ddac866181)

The JSON files for ACVP ML-KEM and ML-DSA were taken from `https://github.com/usnistgov/ACVP-Server`
* As of commit [112690e8484dba7077709a05b1f3af58ddefdd5d](https://github.com/usnistgov/ACVP-Server/commit/112690e8484dba7077709a05b1f3af58ddefdd5d)

The JSON files for ACVP SP 800-108 KBKDF were taken from `https://github.com/usnistgov/ACVP-Server` (`gen-val/json-files/KDF-1.0`)
* As of commit [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d)
* Filtered to the test groups listed in `src/acvp/kbkdf.rs`

The JSON files for ACVP SP 800-56Cr2 two-step KDA were generated locally with the `GenValAppRunner` of `https://github.com/usnistgov/ACVP-Server`
* As of commit [975de31eb83d87039ec88934fdc47d8c312b892d](https://github.com/usnistgov/ACVP-Server/commit/975de31eb83d87039ec88934fdc47d8c312b892d)
* Using the `registration.json` next to each `prompt.json`

The RSP files for SHA-2 were taken from NISTs [Cryptographic Algorithm Validation Program](https://csrc.nist.gov/projects/cryptographic-algorithm-validation-program/secure-hashing) (accessed 05.05.2026).

The test vectors for poly1305 are taken from [boringssl](https://github.com/google/boringssl/blob/2a8e86174536b735a777a56897c7949d33bd46a6/crypto/poly1305/poly1305_tests.txt).

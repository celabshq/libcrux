# Testing

This directory contains the test infrastructure that is shared between the libcrux crates:

- [`kats`](kats): `libcrux-kats`, known answer test vectors and their parsers.
- [`acvp-harness`](acvp-harness): `acvp-harness`, which answers ACVP vector sets with libcrux.

Both are internal testing tools and are not published.

## Known answer tests (`libcrux-kats`)

The algorithm crates depend on `libcrux-kats` as a dev-dependency and enable the features for
the vectors they need, e.g. `libcrux-kats = { workspace = true, features = ["sha3"] }`. The
vectors are embedded with `include_str!`, so tests do not read files at runtime.

| Source                                           | Directory                                        | Module                                      | Algorithms                                                                                                |
| ------------------------------------------------ | ------------------------------------------------ | ------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| [Wycheproof](https://github.com/C2SP/wycheproof) | `kats/wycheproof`                                | `wycheproof`                                | ML-KEM, ML-DSA, P-256 ECDH and ECDSA, P-384 ECDH, HMAC, HKDF, KMAC, ChaCha20-Poly1305, XChaCha20-Poly1305 |
| NIST CAVP response files                         | `kats/tv/{sha2,sha3,drbg}`                       | `sha2`, `sha3`, `cavp`                      | SHA-2, SHA-3, SHAKE, HMAC_DRBG                                                                            |
| BoringSSL                                        | `kats/src/poly1305.rs`                           | `poly1305`                                  | Poly1305                                                                                                  |
| NIST ACVP, downloaded                            | `kats/acvp/{mlkem,mldsa}-*`, `kats/acvp/kdf-1_0` | `acvp::mlkem`, `acvp::mldsa`, `acvp::kbkdf` | ML-KEM, ML-DSA, SP 800-108 KBKDF                                                                          |
| NIST ACVP, generated                             | `kats/acvp/*` (with a `registration.json`)       | `acvp::*`                                   | see below                                                                                                 |

The exact upstream versions are listed in [`kats/Readme.md`](kats/Readme.md).

Wycheproof vectors are edge cases and known-bad inputs. CAVP and ACVP vectors are the NIST
validation tests. The ACVP tests are what the CAVP certificates are based on, see below.

## ACVP

NIST issues CAVP algorithm certificates through ACVP, the Automated Cryptographic Validation
Protocol. A validation lab registers the capabilities of an implementation with the ACVP
server, downloads vector sets (`prompt.json`), and submits the answers of the implementation,
which the server validates.

libcrux tests this in two ways, both based on the open source
[ACVP-Server](https://github.com/usnistgov/ACVP-Server). Its gen/val tools generate and validate
vector sets locally, without the web API, and run in the Docker image in
[`.docker/acvp-server`](../../.docker/acvp-server).

### The ACVP-Server image

```sh
cargo xtask acvp image
```

builds the gen/val tools of the ACVP-Server, at the commit pinned in the Dockerfile, into the
image `ghcr.io/celabshq/libcrux-acvp-server:<tag>`, where `<tag>` is the git tree hash of
`.docker/acvp-server` (`git rev-parse HEAD:.docker/acvp-server`), or `dirty` if the directory has
uncommitted changes. The other `acvp` tasks pull that image, and build it if it is not published
or the directory is dirty. `--image` selects a different image. The container runs as the
calling user, so the files it writes are not owned by root. Its entrypoint has three commands:

- `generate <registration.json>...`: generate a vector set next to each registration
  (`prompt.json`, `expectedResults.json`, `internalProjection.json`).
- `validate (<internalProjection.json> <response.json>)...`: validate responses and write a
  `validation.json` next to them.
- `check <registration.json>...`: check registrations.

### File-based tests

The vector sets in `kats/acvp` are run by regular `cargo test`s in the algorithm crates, usually
in `tests/acvp.rs`, against the expected results. They run on every backend of an algorithm
(portable, NEON, AVX2) as separate tests.

The static files published in the ACVP-Server repository are examples that mostly do not cover
the parameters libcrux supports. Most vector sets are therefore generated from a
`registration.json` that describes what libcrux supports, which is stored next to them:

```sh
cargo xtask acvp generate kats/acvp/sha3-2_0/sha3-256/registration.json
```

Some vector sets are generated as samples (`"isSample": true`), because only samples contain
all expected results. For KAS-ECC-SSC, the implementation chooses its own key pair in the AFT
tests, so the expected results do not contain the private keys. These are in the
`internalProjection.json`, which is kept with `--keep-projection`. For ECDSA keyGen, the tests
check the sample key pairs and that the key generation produces valid key pairs; the actual
key generation answers can only be validated by the server.

| Vector sets                                                                                   | Tests                                                                              |
| --------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| SHA2-224/256/384/512, SHA3-224/256/384/512, SHAKE-128/256                                     | `sha2/tests/acvp.rs`, `sha3/tests/acvp.rs` (the large data tests are `#[ignore]`d) |
| HMAC-SHA2-256/384/512, HMAC-SHA3-224/256/384/512                                              | `hmac/tests/acvp.rs`                                                               |
| AES-128/256 ECB, GCM and CCM                                                                  | `aes/tests/acvp.rs` (ECB needs `--features acvp`)                                  |
| ML-KEM keyGen, encapDecap                                                                     | `libcrux-ml-kem/tests/acvp.rs`                                                     |
| ML-DSA keyGen, sigGen (including `FIPS204-tr1` seed keys), sigVer                             | `libcrux-ml-dsa/tests/acvp.rs` (needs `--features acvp`)                           |
| SP 800-108 KBKDF, SP 800-56C two-step KDA                                                     | `nist-kdf/tests/acvp.rs`                                                           |
| HKDF (`KDA` `HKDF`) with SHA2-256/384/512 and SHA3-224/256/384/512                            | `hkdf/tests/acvp.rs`                                                               |
| KMAC-128/256 (without XOF)                                                                    | `kmac/tests/acvp.rs`                                                               |
| KAS-ECC-SSC P-256, ECDSA keyGen/keyVer/sigGen/sigVer P-256                                    | `libcrux-ecdh/tests/acvp.rs`, `ecdsa/tests/acvp.rs`                                |
| KAS-ECC-SSC P-384, ECDSA keyGen/keyVer P-384                                                  | `p384/tests/acvp.rs`                                                               |
| HMAC_DRBG SHA2-256/384/512                                                                    | `hmac-drbg/tests/acvp.rs`                                                          |
| EdDSA Ed25519 keyGen, sigGen, sigVer (no keyVer, as libcrux has no public key validation API) | `ed25519/tests/acvp.rs`                                                            |
| XECDH Curve25519 keyGen, keyVer, SSC                                                          | `curve25519/tests/acvp.rs`                                                         |

The large data tests (LDT) of SHA-2 and SHA-3 hash messages of 1 to 8 GiB. They are separate
`#[ignore]`d tests (`sha2_*_ldt`, `sha3_*_ldt`), so they do not run with a plain `cargo test` or in
CI. Run them in release mode with:

```sh
cargo test -p libcrux-sha2 --release --test acvp -- --ignored
cargo test -p libcrux-sha3 --release --test acvp -- --ignored
```

### Validation against the ACVP server

```sh
cargo xtask acvp run
```

runs the same loop as a validation, with fresh vector sets and without access to the expected
results:

1. Generate a vector set for each registration in
   [`acvp-harness/registrations`](acvp-harness/registrations). These are not samples, so the
   answers are only in the server's `internalProjection.json`.
2. Answer each vector set with `acvp-harness`, once per backend.
3. Validate the answers with the server, and fail unless every vector set passes.

The output is written to `target/acvp`: the vector sets in `vectors/`, and the responses and
validations per backend in `<backend>/`.

By default, the portable implementation, the platform-multiplexed API (`default`), and the
hardware-specific implementation of the host (`neon` on aarch64, `avx2` on x86_64) are tested.
Use `--backend` (repeatable) to select backends, and pass registrations to run only those:

```sh
cargo xtask acvp run --backend portable aes-gcm
```

Algorithms without an implementation for a backend are reported as skipped, e.g. SHAKE-128 on
NEON. A skipped vector set on the `portable` or `default` backend is a failure, as those implement
every algorithm. Algorithms with a single implementation are answered the same way for every
backend.

The registrations ask for every option ACVP offers within each algorithm family libcrux
implements (key, IV, tag and output lengths, bit-oriented lengths, curves, hash functions,
directions, schemes and roles, large data tests, ...), and also register sibling algorithms that
libcrux does not implement, e.g. SHA-1, SHA2-512/256, HMAC-SHA2-224, deterministic ECDSA, EdDSA
`keyVer` or KDA OneStep. The harness leaves the test cases libcrux cannot answer out of its
response and lists them, with the reason, in `notImplemented.json`; a vector set of an algorithm
the harness has no module for is answered this way as a whole. They are removed from the
projection before validation, and listed as not implemented at the end of the run. A vector set
without any implemented test case is reported as not implemented without validation.

The harness can also be run on its own, e.g. on a vector set from a lab:

```sh
cargo run --release -p libcrux-acvp-harness -- --backend portable prompt.json response.json
```

The prompt is either a vector set, or a vector set in the format of the ACVP web API
(`[{"acvVersion": ..}, vector set]`), which is answered in the same format. The harness exits with
code 2 if the algorithm is not implemented for the backend, and with code 1 on any other error,
e.g. an unknown algorithm or a failed test case (reported with its `vsId`, `tgId` and `tcId`).

This does not use the ACVP web API (authentication, test sessions, downloading vector sets and
uploading responses), which is how a lab runs the validation against NIST's demo and
production servers. The vector set and response files are the formats of that protocol.

### Not covered (yet)

Algorithms in libcrux that ACVP can test, but that have no ACVP tests:

| Algorithm                                     | Crate                   | ACVP                                 | Note               |
| --------------------------------------------- | ----------------------- | ------------------------------------ | ------------------ |
| RSA-PSS sign and verify with SHA2-256/384/512 | `crates/algorithms/rsa` | `RSA` `sigGen`, `sigVer` (FIPS186-5) | Rsa is legacy only |

Parts of algorithms with ACVP tests that are not tested:

| What                                                           | Why                                                          |
| -------------------------------------------------------------- | ------------------------------------------------------------ |
| Options and algorithm variants that libcrux does not implement | listed by `cargo xtask acvp run` under "Not implemented"     |

`cargo xtask acvp run` lists every test case the server generated that libcrux cannot answer,
with the reason:

| Vector set | Not implemented |
| --- | --- |
| SHA-1, SHA2-512/224, SHA2-512/256 | the whole algorithm |
| SHA2-224/256/384/512, SHA3-224/256/384/512 | bit-oriented messages; LDT of 2, 4 and 8 GiB |
| SHAKE-128, SHAKE-256 | bit-oriented messages and outputs |
| HMAC-SHA-1, HMAC-SHA2-224, HMAC-SHA2-512/224, HMAC-SHA2-512/256 | the whole algorithm |
| HMAC-SHA2-256/384/512, HMAC-SHA3-224/256/384/512 | bit-oriented messages |
| AES-ECB | AES-192; decryption |
| AES-GCM | AES-192; IV lengths other than 96 bits; tag lengths other than 128 bits; internal IV generation |
| AES-CCM | AES-192; IV lengths other than 96 bits; tag lengths other than 64 and 128 bits |
| ECDSA `keyGen`, `keyVer` | P-224, P-521, B-* and K-* curves; `keyGen` with "extra bits" |
| ECDSA `sigGen`, `sigVer` | curves other than P-256; hash functions other than SHA2-256/384/512; `sigGen` component test and SP800-106 randomized hashing |
| DetECDSA `sigGen` | the whole algorithm |
| EdDSA `keyGen`, `sigGen`, `sigVer` | Ed448; Ed25519ph and Ed448ph |
| EdDSA `keyVer` | the whole mode (no public key validation API) |
| XECDH `keyGen`, `keyVer`, `SSC` | Curve448 |
| hmacDRBG | SHA-1, SHA2-224, SHA2-512/224, SHA2-512/256 and SHA3 modes |
| KAS-ECC-SSC | curves other than P-256 and P-384; `fullMqv`, `onePassMqv` |
| KDA HKDF | SHA-1, SHA2-224, SHA2-512/224, SHA2-512/256 |
| KDA OneStep, OneStepNoCounter | the whole mode |
| KBKDF | counter and double pipeline modes; feedback without a counter or with another counter length or location; CMAC and HMAC-SHA-1/SHA2-224/SHA2-512/224/SHA2-512/256; output lengths that are not whole bytes |
| KDF KMAC (SP 800-108r1) | the whole mode |
| KMAC-128, KMAC-256 | KMAC as an XOF |
| ML-DSA `sigGen`, `sigVer` | external mu; HashML-DSA with hash functions other than SHAKE-128 |

Options that are not registered, so the run does not list them:

| What | Why |
| --- | --- |
| KDA TwoStep beyond the four 3KEM registrations | the ACVP server runs out of memory generating a full TwoStep vector set |
| ECDSA `sigVer` on the B-* and K-* curves, and with SP800-106 | generating these vector sets did not finish within 19 minutes |
| KAS-ECC-SSC with `hashFunctionZ` | not registered yet; the harness rejects it |
| KAS-ECC CDH-Component | not registered yet |

ChaCha20-Poly1305, Poly1305, BLAKE2, and the protocols built from the algorithms
(e.g. PSQ) have no ACVP tests, as ACVP does not cover them; they are tested with the other
known answer tests above.

### CI

- The algorithm crates' workflows run the file-based tests, including on the portable
  implementation (`LIBCRUX_DISABLE_SIMD128=1 LIBCRUX_DISABLE_SIMD256=1`).
- [`acvp.yml`](../../.github/workflows/acvp.yml) runs `cargo xtask acvp run` on x86_64 and
  aarch64 Linux.
- [`docker-acvp-server.yml`](../../.github/workflows/docker-acvp-server.yml) publishes the
  ACVP-Server image to `ghcr.io/<repository>-acvp-server` when `.docker/acvp-server` changes on
  `main`, tagged with the git tree hash of that directory. `cargo xtask acvp` pulls the image
  with that tag, in CI and locally, so it is only built when it has not been published yet.

# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- [#1539](https://github.com/celabshq/libcrux/pull/1539): Support for the
  post-quantum and PQ/T-hybrid algorithms of
  [draft-ietf-hpke-pq](https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-04),
  in the **libcrux provider only**, behind the new `draft-ietf-hpke-pq` feature.
  Validated against the draft's published test vectors.
  - Single-stage KDFs `KdfAlgorithm::Shake128 = 0x0010` and
    `KdfAlgorithm::Shake256 = 0x0011`, with the corresponding one-stage HPKE
    key schedule (`KeySchedule`, `Export`, `DeriveKeyPair`).
  - ML-KEM-512 (`0x0040`), ML-KEM-768 (`0x0041`), and ML-KEM-1024 (`0x0042`) as
    HPKE KEMs.
  - The PQ/T-hybrid KEMs `KemAlgorithm::MlKem768P256 = 0x0050`,
    `KemAlgorithm::MlKem1024P384 = 0x0051`, and `MLKEM768-X25519` (`0x647a`,
    via X-Wing draft-06), per
    [draft-irtf-cfrg-concrete-hybrid-kems-03](https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-concrete-hybrid-kems-03).

### Fixed

- [#xxxx](https://github.com/celabshq/libcrux/pull/xxxx): Return `KemInvalidCiphertext` error instead of `AeadInvalidCiphertext` on KEM decapsulation failure

## [0.7.0] (2026-07-15)

### Added

- [#147](https://github.com/celabshq/hpke-rs/pull/147): Add P384 and P521 DHKEM support for the libcrux provider via RustCrypto crates guarded behind the `libcrux-rustcrypto-p-curves` feature flag.
- [#146](https://github.com/celabshq/hpke-rs/pull/146): Add support for ML-KEM768 and ML-KEM1024 gated behind the `draft-connolly-cfrg-hpke-mlkem` feature flag.

### Changed

- [#1534](https://github.com/celabshq/libcrux/pull/1534): Update dependencies: `hpke-rs-crypto`, `libcrux-traits`, `libcrux-aead`, `libcrux-hkdf`, `libcrux-kem`

## [0.6.1] - 2026-02-20

### Changed
- [#129](https://github.com/cryspen/hpke-rs/pull/129): Update libcrux dependencies
- [#122](https://github.com/cryspen/hpke-rs/pull/122): Update rand dependencies

## [0.5.1] - 2026-02-02

- [#114](https://github.com/celabshq/libcrux/pull/114): Update dependencies `libcrux-ecdh`, `libcrux-aead`, `libcrux-sha3`, `libcrux-kem`, `libcrux-hkdf`, `libcrux-traits`

## [0.5.0] - 2025-12-16

- [#105](https://github.com/cryspen/hpke-rs/pull/105) Update dependencies

## 0.4.0 - 2025-12-01

- [#98](https://github.com/cryspen/hpke-rs/pull/98): add support for AES-GCM to the provider

## 0.3.0 - 2025-07-01

* initial release

*Please disregard any previous versions.*

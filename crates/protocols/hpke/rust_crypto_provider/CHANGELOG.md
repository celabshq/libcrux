# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- [#1602](https://github.com/celabshq/libcrux/pull/1602): Update the RustCrypto dependencies: `p256`, `k256` and `p384` to `0.14`, `x25519-dalek` to `3`, `chacha20poly1305` to `0.11`, `aes-gcm` to `0.11`, and the (experimental) `x-wing` and `ml-kem` to their `0.1`/`0.3` releases.

## [0.7.0] (2026-07-15)

### Changed

- [#1534](https://github.com/celabshq/libcrux/pull/1534): Update dependency `hpke-rs-crypto`

## [0.6.1] - 2026-03-20

### Changed

- [#137](https://github.com/cryspen/hpke-rs/pull/137): Move unstable X-Wing and ML-KEM ciphersuites behind the `experimental` feature.

## [0.6.0] - 2026-02-13

### Fixed
- [#124](https://github.com/cryspen/hpke-rs/pull/124): Error out on x25519 0 keys

### Added
- [#127](https://github.com/cryspen/hpke-rs/pull/127): Add support for ML-KEM-768 (`KemAlgorithm::MlKem768 = 0x0041`) and ML-KEM-1024 (`KemAlgorithm::MlKem1024 = 0x0042`) to the RustCrypto provider, and add X-Wing support to the RustCrypto provider using the `x-wing` and `ml-kem` crates.

## [0.4.0] - 2025-12-16

- [#105](https://github.com/cryspen/hpke-rs/pull/105) Update dependencies

## [0.3.0] - 2025-07-01

- [#72](https://github.com/cryspen/hpke-rs/pull/72):
  -  use new hpke-rs-crypto trait API

## [0.2.0] - 2023-12-01

- [#59](https://github.com/franziskuskiefer/hpke-rs/pull/59): hpke-rs-rust-crypto: make deterministic-prng enable the std feature
- [#58](https://github.com/franziskuskiefer/hpke-rs/pull/58): no-std-ify hpke-rs-rust-crypto (some more)
- [#57](https://github.com/franziskuskiefer/hpke-rs/pull/57): switch from x25519-dalek-ng to x25519-dalek
- [#53](https://github.com/franziskuskiefer/hpke-rs/pull/53): rm getrandom dep
- [#50](https://github.com/franziskuskiefer/hpke-rs/pull/50): no-std-ify hpke-rs-crypto
- [#49](https://github.com/franziskuskiefer/hpke-rs/pull/49): hpke-rs-crypto: make serde opt-in
- [#48](https://github.com/franziskuskiefer/hpke-rs/pull/48): no-std-ify hpke-rs-rust-crypto
- [#47](https://github.com/franziskuskiefer/hpke-rs/pull/47): hpks-rs-crypto: simplify Cargo.toml

## [0.1.3] - 2023-11-21

- Updated TLS codec dependency

## [0.1.2] - 2023-03-04

### Changed

- Update dependencies

## 0.1.1 (2022-02-24)

- initial release

_Please disregard any previous versions._

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] (2026-10-07)

### Added

- [#1539](https://github.com/celabshq/libcrux/pull/1539): Add the `draft-ietf-hpke-pq`
  algorithm identifiers. New KEMs `KemAlgorithm::MlKem512 = 0x0040`,
  `MlKem768P256 = 0x0050`, and `MlKem1024P384 = 0x0051`; new single-stage KDFs
  `KdfAlgorithm::Shake128 = 0x0010`, `Shake256 = 0x0011`, `TurboShake128 = 0x0012`,
  and `TurboShake256 = 0x0013`. Repoint the ML-KEM / hybrid KEM doc references to
  `draft-ietf-hpke-pq` (authoritative) and `draft-irtf-cfrg-concrete-hybrid-kems`.
  Split KDF handling into single-stage (`SingleStageKdfAlgorithm`) and two-stage
  (`TwoStageKdfAlgorithm`) families: `HpkeCrypto::kdf_extract`/`kdf_expand` now take a
  `TwoStageKdfAlgorithm`, and a new `kdf_derive` method covers the single-stage KDFs.

## [0.7.0] (2026-07-15)

### Changed

- [#154](https://github.com/celabshq/hpke-rs/pull/154): `HpkeCrypto::HpkePrng` now has `rand` v0.10 bound `CryptoRng`, instead of `rand` v0.9 bound `RngCore + CryptoRng`


## [0.6.1] - 2026-03-20

## [0.4.0] - 2025-12-16

- [#103](https://github.com/cryspen/hpke-rs/pull/103): Add correct code point for `XWingDraft06` ciphersuite and move old code point to `XWingDraft06Hpke`.

## [0.3.0] - 2025-07-01

- [#72](https://github.com/cryspen/hpke-rs/pull/72):
  -  redesign `HpkeCrypto` trait to support X-Wing KEM
  -  upgrade rand dependency from 0.8 -> 0.9

## [0.2.0] - 2023-12-01

- [#53](https://github.com/franziskuskiefer/hpke-rs/pull/53): rm getrandom dep
- [#50](https://github.com/franziskuskiefer/hpke-rs/pull/50): no-std-ify hpke-rs-crypto
- [#49](https://github.com/franziskuskiefer/hpke-rs/pull/49): hpke-rs-crypto: make serde opt-in
- [#47](https://github.com/franziskuskiefer/hpke-rs/pull/47): hpks-rs-crypto: simplify Cargo.toml

## [0.1.3] - 2023-11-21

- Updated TLS codec dependency

## [0.1.2] - 2023-03-04

### Changed

- Update dependencies

## 0.1.1 (2022-02-24)

- initial release

_Please disregard any previous versions._

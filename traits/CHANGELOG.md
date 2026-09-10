# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- [#1584](https://github.com/celabshq/libcrux/pull/1584): Added a `KEMError::InsufficientRandomness`
  variant for KEM implementations that sample randomness from a fallible RNG
- [#1584](https://github.com/celabshq/libcrux/pull/1584): (Breaking) Widened the RNG bound on
  `kem::KEM::generate_key_pair` and `kem::KEM::encapsulate` from `CryptoRng` to `TryCryptoRng` to
  support fallible RNGs

## [0.0.8] (2026-07-15)

### Changed

- [#1534](https://github.com/celabshq/libcrux/pull/1534): Update dependency `libcrux-secrets`
- [#1493](https://github.com/celabshq/libcrux/pull/1493): `finish` methods on digest traits consume the hasher

## [0.0.7] (2026-05-13)

### Changed

- [#1385](https://github.com/celabshq/libcrux/pull/1385): Update trait bounds to use `rand` version 0.10

## [0.0.6] (2026-02-12)

### Changed

- [#1309](https://github.com/celabshq/libcrux/pull/1309): Allow `no_alloc` support & add minimal `README.md`

## [0.0.5] (2026-01-22)

- [#1297](https://github.com/celabshq/libcrux/pull/1297): Update dependencies
  
## [0.0.4] (2025-11-05)

- [#1190](https://github.com/celabshq/libcrux/pull/1190): Use secret types in kem traits
- [#1187](https://github.com/celabshq/libcrux/pull/1187): Add provided method `generate_pair` to ECDH traits
- [#1184](https://github.com/celabshq/libcrux/pull/1184): Make aead use secrets and add keygen functions
- [#1181](https://github.com/celabshq/libcrux/pull/1181): Basic ECDH traits
- [#1155](https://github.com/celabshq/libcrux/pull/1155): Add API for initializing digest state
- [#1115](https://github.com/celabshq/libcrux/pull/1115): AEAD: Key-Centric API with Multiplexing Support
- [#1078](https://github.com/celabshq/libcrux/pull/1078): Digest traits
- [#1057](https://github.com/celabshq/libcrux/pull/1057): Add AEAD traits
- [#1053](https://github.com/celabshq/libcrux/pull/1053): Add and implement KEM traits

## [0.0.3] (2025-06-30)

- [#922](https://github.com/celabshq/libcrux/pull/922): Make crate `no_std`-compatible by depending on `rand` without the `std` feature enabled

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.6] (2026-02-12)

### Changed

- [#1309](https://github.com/cryspen/libcrux/pull/1309): Allow `no_alloc` support & add minimal `README.md`

## [0.0.5] (2026-01-22)

- [#1297](https://github.com/cryspen/libcrux/pull/1297): Update dependencies
  
## [0.0.4] (2025-11-05)

- [#1190](https://github.com/cryspen/libcrux/pull/1190): Use secret types in kem traits
- [#1187](https://github.com/cryspen/libcrux/pull/1187): Add provided method `generate_pair` to ECDH traits
- [#1184](https://github.com/cryspen/libcrux/pull/1184): Make aead use secrets and add keygen functions
- [#1181](https://github.com/cryspen/libcrux/pull/1181): Basic ECDH traits
- [#1155](https://github.com/cryspen/libcrux/pull/1155): Add API for initializing digest state
- [#1115](https://github.com/cryspen/libcrux/pull/1115): AEAD: Key-Centric API with Multiplexing Support
- [#1078](https://github.com/cryspen/libcrux/pull/1078): Digest traits
- [#1057](https://github.com/cryspen/libcrux/pull/1057): Add AEAD traits
- [#1053](https://github.com/cryspen/libcrux/pull/1053): Add and implement KEM traits

## [0.0.3] (2025-06-30)

- [#922](https://github.com/cryspen/libcrux/pull/922): Make crate `no_std`-compatible by depending on `rand` without the `std` feature enabled

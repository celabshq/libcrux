# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.6] (2026-02-12)

### Changed

- [#1324](https://github.com/cryspen/libcrux/pull/1324): Update dependencies: `libcrux-curve25519`, `libcrux-ecdh`, `libcrux-ml-kem`, `libcrux-traits`, `libcrux-p256`, `libcrux-sha3`

## [0.0.5] (2026-01-22)

- [#1297](https://github.com/cryspen/libcrux/pull/1297): Update dependencies
- [#1280](https://github.com/cryspen/libcrux/pull/1280): Update dependencies `libcrux-sha3`, `libcrux-ml-kem`

## [0.0.4] (2025-11-05)

- [#1053](https://github.com/cryspen/libcrux/pull/1053): Add and implement KEM traits

## [0.0.3] (2025-06-30)

- [#920](https://github.com/cryspen/libcrux/pull/920):
  - Drop support for XWingKyberDraft02, XWingKemDraft02, and X25519Kyber768Draft00
  - Add support for XWingKemDraft06
  - Add key gen and encaps derandomized functions
- [#922](https://github.com/cryspen/libcrux/pull/922): Make `no_std` optional using default `std` feature

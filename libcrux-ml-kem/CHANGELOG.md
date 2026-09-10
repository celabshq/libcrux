# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- [#1584](https://github.com/celabshq/libcrux/pull/1584): Widened the RNG bound on `pqcp::crypto_kem_keypair`, `pqcp::crypto_kem_enc`,
  `pqcp::crypto_kem_enc_struct`, and `incremental::rand::encapsulate1` from
  `CryptoRng` to `TryCryptoRng` to support fallible RNGs
- [#1584](https://github.com/celabshq/libcrux/pull/1584): (Breaking) Widened the RNG bound on
  `incremental::KeyPairCompressedBytes::generate` from `CryptoRng` to `TryCryptoRng`; it now
  returns `Result<Self, Error>` instead of `Self`, matching `incremental::KeyPairBytes::generate`
- [#1584](https://github.com/celabshq/libcrux/pull/1584): (Breaking) Widened the RNG bound on
  `pqcp::crypto_kem_keypair_struct` from `CryptoRng` to `TryCryptoRng`; it now returns
  `Result<(), PQCPError>` instead of `()`, matching `pqcp::crypto_kem_keypair` and
  `pqcp::crypto_kem_enc_struct`
- [#1584](https://github.com/celabshq/libcrux/pull/1584): (Breaking) Widened the RNG bound on
  `mlkem512::rand::generate_key_pair`, `mlkem512::rand::encapsulate`,
  `mlkem768::rand::generate_key_pair`, `mlkem768::rand::encapsulate`,
  `mlkem1024::rand::generate_key_pair`, and `mlkem1024::rand::encapsulate` from `CryptoRng` to
  `TryCryptoRng`; they now return `Result<_, libcrux_traits::kem::KEMError>` instead of the bare
  value

### Fixed

- [#1584](https://github.com/celabshq/libcrux/pull/1584): Fixed a pre-existing build failure when combining the `incremental` and
  `rand` features, caused by `RngCore`/`TryRngCore` no longer being
  re-exported from `rand`'s crate root as of `rand` 0.10

## [0.0.10] (2026-07-15)

### Changed

- [#1534](https://github.com/celabshq/libcrux/pull/1534): Update dependencies: `libcrux-secrets`, `libcrux-traits`, `libcrux-sha3`, `libcrux-intrinsics`

## [0.0.9] (2026-05-13)

### Changed

- [#1434](https://github.com/celabshq/libcrux/pull/1434): Update dependencies: `libcrux-intrinsics`
- [#1433](https://github.com/celabshq/libcrux/pull/1433): Update dependencies: `libcrux-traits`, `libcrux-sha3`
- [#1385](https://github.com/celabshq/libcrux/pull/1385): Update trait bounds to use `rand` version 0.10

## [0.0.8] (2026-03-19)

### Changed

- [#1368](https://github.com/celabshq/libcrux/pull/1368): Update dependencies: `libcrux-sha3`

## [0.0.7] (2026-02-12)

### Changed

- [#1324](https://github.com/celabshq/libcrux/pull/1324): Update dependencies: `libcrux-sha3`, `libcrux-traits`, `libcrux-intrinsics`
- [#1242](https://github.com/celabshq/libcrux/pull/1242): In tests, use the latest versions of ACVP KATs exported by `libcrux-kats`.

## [0.0.6] (2026-01-22)

- [#1285](https://github.com/celabshq/libcrux/pull/1285): Update `hax-lib` dependency

## [0.0.5] (2026-01-12)

- [#1280](https://github.com/celabshq/libcrux/pull/1280): Update dependencies `libcrux-platform`, `libcrux-sha3`
- [#1276](https://github.com/celabshq/libcrux/pull/1276): (Breaking) Fix incorrect portable state serialization in incremental API (@GuuJiang)

## [0.0.4] (2025-11-05)

- [#1190](https://github.com/celabshq/libcrux/pull/1190): Use secret types in kem traits
- [#1165](https://github.com/celabshq/libcrux/pull/1165): PQCP APIs
- [#1053](https://github.com/celabshq/libcrux/pull/1053): Add and implement KEM traits

## [0.0.3] (2025-06-30)

- [#922](https://github.com/celabshq/libcrux/pull/922): Feature-gate `rand/std` feature behind the `std` feature in this crate

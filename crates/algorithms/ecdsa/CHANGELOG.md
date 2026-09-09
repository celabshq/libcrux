# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- [#XXXX](https://github.com/celabshq/libcrux/pull/XXXX): `PrivateKey`/nonce scalar validation now rejects over-length input instead of silently truncating it to its last 32 bytes

## [0.0.8] (2026-07-15)

### Changed

- [#1534](https://github.com/celabshq/libcrux/pull/1534): Update dependencies: `libcrux-sha2`, `libcrux-p256`

## [0.0.7] (2026-05-13)

### Changed

- [#1433](https://github.com/celabshq/libcrux/pull/1433): Update dependencies: `libcrux-traits`, `libcrux-sha2`
- [#1385](https://github.com/celabshq/libcrux/pull/1385): Dropped `Rng` bounds on `rand` feature

## [0.0.6] (2026-02-12)

### Changed

- [#1324](https://github.com/celabshq/libcrux/pull/1324): Update dependencies: `libcrux-p256`, `libcrux-sha2`

## [0.0.5] (2026-01-26)

- [#1297](https://github.com/celabshq/libcrux/pull/1297): Update dependencies

## [0.0.4] (2025-11-05)

- [#1061](https://github.com/celabshq/libcrux/pull/1061): Add `std` feature gate for `libcrux-ecdsa`
- [#1060](https://github.com/celabshq/libcrux/pull/1060): Fixes for `libcrux-ecdsa` with `--no-default-features`

## [0.0.3] (2025-06-30)

- [#922](https://github.com/celabshq/libcrux/pull/922): Upgrade dependencies for `libcrux-sha2` and `libcrux-p256`

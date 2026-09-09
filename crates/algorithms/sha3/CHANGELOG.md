# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- [#XXXX](https://github.com/celabshq/libcrux/pull/XXXX): `hash::<LEN>` now enforces `LEN == digest_size(algorithm)` and the payload-length precondition with a release-mode `assert!` instead of a `debug_assert!`, so a mismatch panics instead of silently returning a truncated/wrong-algorithm digest

## [0.0.10] (2026-07-15)

### Fixed

- [#1493](https://github.com/celabshq/libcrux/pull/1493): Provide instantiable `libcrux-traits` incremental hashing API
- [#1456](https://github.com/celabshq/libcrux/pull/1456): Fix out of bounds indexing in avx2 SHAKE-256 implementation
- [#1389](https://github.com/celabshq/libcrux/pull/1389): Fix partial Block output of incremental XOF API

### Changed

- [#1534](https://github.com/celabshq/libcrux/pull/1534): Update dependency: `libcrux-traits`, `libcrux-intrinsics`
- [#1454](https://github.com/celabshq/libcrux/pull/1454): `debug_assert` that generic `LEN` matches algorithm in `hash` function
- [#1493](https://github.com/celabshq/libcrux/pull/1493): `finish` methods on digest traits consume the hasher


### Added

- [#1292](https://github.com/celabshq/libcrux/pull/1292): Add portable implementations of CSHAKE-128 and CSHAKE-256 (https://github.com/kraemv)

## [0.0.9] (2026-05-13)

### Changed

- [#1434](https://github.com/celabshq/libcrux/pull/1434): Update dependencies: `libcrux-intrinsics`
- [#1433](https://github.com/celabshq/libcrux/pull/1433): Update dependencies: `libcrux-traits`

## [0.0.8] (2026-03-19)

### Fixed

- [#1352](https://github.com/celabshq/libcrux/pull/1352): Fix SHA-3 XOF squeeze and AVX2 store function

## [0.0.7] (2026-02-12)

### Changed

- [#1324](https://github.com/celabshq/libcrux/pull/1324): Update dependencies: `libcrux-traits`, `libcrux-intrinsics`

## [0.0.6] (2026-01-22)

- [#1285](https://github.com/celabshq/libcrux/pull/1285): Update `hax-lib` dependency

## [0.0.5] (2026-01-12)

- [#1280](https://github.com/celabshq/libcrux/pull/1280): Update dependency `libcrux-platform`
- [#1238](https://github.com/celabshq/libcrux/pull/1237): Remove digest type aliases

## [0.0.4] (2025-11-05)

- [#1078](https://github.com/celabshq/libcrux/pull/1078): Digest traits

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.7] (2026-10-07)

### Changed

- [#1551](https://github.com/celabshq/libcrux/pull/1551) Update crabgrind to 0.3.1

## [0.0.6] (2026-07-15)

### Fixed

- [#1461](https://github.com/celabshq/libcrux/pull/1461): Fix incorrect cmp in aarch64 select/swap implementation

### Changed

- [#1499](https://github.com/celabshq/libcrux/pull/1499): Replace `IntOps` and `EncodeOps` traits with inherent methods on `Secret` (https://github.com/Arul-Sujith)
- [#1446](https://github.com/celabshq/libcrux/pull/1446): Remove const qualifier of secret types constructors
- [#1462](https://github.com/celabshq/libcrux/pull/1462): More robust casts instead of transmutes when checking secret independence
- [#1484](https://github.com/celabshq/libcrux/pull/1484): seal scalar trait and synchronize De-/Classify trait impls for public/secret types

### Added

- [#1446](https://github.com/celabshq/libcrux/pull/1446): Integrate valgrind requests when cfg `valgrind_ct_test` is set

## [0.0.5] (2026-01-22)

### Added

- [#1284](https://github.com/celabshq/libcrux/pull/1284): Add wrapping negation to `IntOps` trait

### Changed

- [#1285](https://github.com/celabshq/libcrux/pull/1285): Update `hax-lib` dependency

## [0.0.4] (2025-11-05)

### Added
- [#1210](https://github.com/celabshq/libcrux/pull/1210): Implement (de-)classification of immutable slices for `cfg(hax)`
- [#1135](https://github.com/celabshq/libcrux/pull/1135): also impl conversion for public
- [#1095](https://github.com/celabshq/libcrux/pull/1095): Implement negation for secret integers
- [#1094](https://github.com/celabshq/libcrux/pull/1094): Add constant time swap and select

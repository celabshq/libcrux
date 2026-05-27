# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Fixed

- [#1460](https://github.com/cryspen/libcrux/issues/1460): Fix incorrect cmp in aarch64 select/swap implementation

### Changed

- [#1446](https://github.com/cryspen/libcrux/pull/1446): Remove const qualifier of secret types constructors
- [#1462](https://github.com/cryspen/libcrux/pull/1462): More robust casts instead of transmutes when checking secret independence

### Added

- [#1446](https://github.com/cryspen/libcrux/pull/1446): Integrate valgrind requests when cfg `valgrind_ct_test` is set

## [0.0.5] (2026-01-22)

### Added

- [#1284](https://github.com/cryspen/libcrux/pull/1284): Add wrapping negation to `IntOps` trait

### Changed

- [#1285](https://github.com/cryspen/libcrux/pull/1285): Update `hax-lib` dependency

## [0.0.4] (2025-11-05)

### Added
- [#1210](https://github.com/cryspen/libcrux/pull/1210): Implement (de-)classification of immutable slices for `cfg(hax)`
- [#1135](https://github.com/cryspen/libcrux/pull/1135): also impl conversion for public
- [#1095](https://github.com/cryspen/libcrux/pull/1095): Implement negation for secret integers
- [#1094](https://github.com/cryspen/libcrux/pull/1094): Add constant time swap and select

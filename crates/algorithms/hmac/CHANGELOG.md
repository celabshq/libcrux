# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- [#XXXX](https://github.com/celabshq/libcrux/pull/XXXX): `hmac()` now panics with a clear message instead of silently returning a tag shorter than the requested `tag_length` when `tag_length` exceeds the algorithm's native tag length

## [0.0.8] (2026-07-15)

### Added

- [#1382](https://github.com/celabshq/libcrux/pull/1382): Add an incremental API

### Changed

- [#1534](https://github.com/celabshq/libcrux/pull/1534): Update dependencies: `libcrux-traits`, `libcrux-sha2`


## [0.0.7] (2026-05-13)

### Removed

- [#1391](https://github.com/celabshq/libcrux/pull/1391): Remove support for HMAC-SHA1

### Changed

- [#1434](https://github.com/celabshq/libcrux/pull/1434): Update dependencies: `libcrux-hacl-rs`
- [#1433](https://github.com/celabshq/libcrux/pull/1433): Update dependencies: `libcrux-sha2`

## [0.0.6] (2026-02-12)

### Changed

- [#1324](https://github.com/celabshq/libcrux/pull/1324): Update dependencies: `libcrux-sha2`

## [0.0.5] (2026-01-22)

- [#1297](https://github.com/celabshq/libcrux/pull/1297): Update dependencies
  
## [0.0.4] (2025-11-05)

## [0.0.3] (2025-06-30)

- [#922](https://github.com/celabshq/libcrux/pull/922): Make crate `no_std`-compatible by upgrading to latest, `no_std`-compatible version of `libcrux-sha2` dependency.
- [#993](https://github.com/celabshq/libcrux/pull/993): Update dependencies `hacl-rs` and `macros`

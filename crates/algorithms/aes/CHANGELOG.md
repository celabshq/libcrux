# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- [#1474](https://github.com/cryspen/libcrux/pull/1474): Add support for AES-CCM

### Fixed

- [#1474](https://github.com/cryspen/libcrux/pull/1474): Enforce limits on plaintext and AAD lengths for AES-GCM, according to RFC 5116.

### Changed

- [#1474](https://github.com/cryspen/libcrux/pull/1474): Change crate name from `libcrux-aesgcm` to `libcrux-aes`

## [0.0.8] (2026-05-13)

### Changed

- [#1434](https://github.com/cryspen/libcrux/pull/1434): Update dependencies: `libcrux-intrinsics`
- [#1433](https://github.com/cryspen/libcrux/pull/1433): Update dependencies: `libcrux-traits`

### Removed

- [#1385](https://github.com/cryspen/libcrux/pull/1385): Remove empty cargo feature `rand`

## [0.0.7] (2026-02-12)

### Changed

- [#1324](https://github.com/cryspen/libcrux/pull/1324): Update dependencies: `libcrux-traits`, `libcrux-intrinsics`

## [0.0.6] (2026-01-22)

- [#1297](https://github.com/cryspen/libcrux/pull/1297): Update dependencies
  
## [0.0.5] (2026-01-12)

- [#1280](https://github.com/cryspen/libcrux/pull/1280): Update dependency `libcrux-platform`

## [0.0.4] (2025-11-05)

- [#1185](https://github.com/cryspen/libcrux/pull/1185): Initial version of `libcrux-aesgcm`

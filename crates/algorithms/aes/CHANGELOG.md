# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- [#26](https://github.com/cryspen/libcrux-evit/pull/26): Add support for AES-CCM

### Fixed

- [#26](https://github.com/cryspen/libcrux-evit/pull/26): Enforce limits on plaintext and AAD lengths for AES-GCM, according to RFC 5116.

### Changed

- [#26](https://github.com/cryspen/libcrux-evit/pull/26): Change crate name from `libcrux-aesgcm` to `libcrux-aes`

## [0.0.7] (2026-02-12)

### Changed

- [#1324](https://github.com/cryspen/libcrux/pull/1324): Update dependencies: `libcrux-traits`, `libcrux-intrinsics`

## [0.0.6] (2026-01-22)

- [#1297](https://github.com/cryspen/libcrux/pull/1297): Update dependencies
  
## [0.0.5] (2026-01-12)

- [#1280](https://github.com/cryspen/libcrux/pull/1280): Update dependency `libcrux-platform`

## [0.0.4] (2025-11-05)

- [#1185](https://github.com/cryspen/libcrux/pull/1185): Initial version of `libcrux-aesgcm`

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Zeroize `PrivateKey` and `SharedSecret` on drop, and the intermediate values of
  scalar multiplication and field inversion.

### Changed

- `PrivateKey` and `SharedSecret` now implement `Drop`, so their fields can no
  longer be moved out.

### Changed

- `PrivateKey::generate` follows SP 800-56A r3, Section 5.6.1.2.2 (Key Pair Generation by
  Testing Candidates): candidates `c > n - 2` are rejected and the private key is `c + 1`.
  The same random input now yields a different key.

## [0.0.1]

### Added

- [#1601](https://github.com/celabshq/libcrux/pull/1601): Initial version of this crate.

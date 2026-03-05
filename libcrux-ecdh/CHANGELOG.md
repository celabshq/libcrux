# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.6] (2026-02-12)

### Changed

- [#1324](https://github.com/cryspen/libcrux/pull/1324): Update dependencies: `libcrux-curve25519`, `libcrux-p256`
- [#1301](https://github.com/cryspen/libcrux/pull/1301): Check length and clamping in X25519 secret validation. This is a breaking change since errors are now raised on unclamped X25519 secrets or inputs of the wrong length

## [0.0.5] (2026-01-22)

- [#1297](https://github.com/cryspen/libcrux/pull/1297): Update dependencies
  
## [0.0.4] (2025-11-05)

## [0.0.3] (2025-06-30)

-  [#920](https://github.com/cryspen/libcrux/pull/920): Export `p256_secret_to_public()` and `x25519_secret_to_public()`
-  [#922](https://github.com/cryspen/libcrux/pull/922): Make `no_std` optional using default `std` feature


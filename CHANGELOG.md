# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Fixed

- (libcrux-secrets) [#1460](https://github.com/cryspen/libcrux/issues/1460): Fix incorrect cmp in aarch64 select/swap implementation 

### Changed

- (libcrux-secrets) [#1446](https://github.com/cryspen/libcrux/pull/1446): Remove const qualifier of secret types constructors
- (libcrux-secrets) [#1462](https://github.com/cryspen/libcrux/pull/1462): More robust casts instead of transmutes when checking secret independence

### Added

- (libcrux-secrets) [#1446](https://github.com/cryspen/libcrux/pull/1446): Integrate valgrind requests when cfg `valgrind_ct_test` is set

## [0.0.4] (2026-05-13)

### Fixed

- (libcrux-ml-dsa) [#1398](https://github.com/cryspen/libcrux/pull/1398): Fix incorrect AVX2 use_hint implementation
- (libcrux-ml-dsa) [#1395](https://github.com/cryspen/libcrux/pull/1395): Fully reduce iNTT inputs on AVX2
- (libcrux-chacha20poly1305) [#1386](https://github.com/cryspen/libcrux/pull/1386): Fix potential panic in `libcrux_chacha20poly1305::encrypt` (reported by @fg0x0)

### Removed

- (libcrux-hmac) [#1391](https://github.com/cryspen/libcrux/pull/1391): Remove support for HMAC-SHA1

### Changed

- [#1434](https://github.com/cryspen/libcrux/pull/1434): Update dependencies: `libcrux-hacl-rs`, `libcrux-poly1305`, `libcrux-curve25519`
- [#1433](https://github.com/cryspen/libcrux/pull/1433): Update dependencies: `libcrux-traits`, `libcrux-ed25519`, `libcrux-ml-kem`, `libcrux-kem`, `libcrux-aesgcm`, `libcrux-blake2`, `libcrux-chacha20poly1305`, `libcrux-p256`, `libcrux-curve25519`, `libcrux-sha3`, `libcrux-sha2`, `libcrux-hmac`, `libcrux-hkdf`, `libcrux-rsa`, `libcrux-ecdsa`, `libcrux-ecdh`, `libcrux-digest`, `libcrux-psq`, `libcrux-ml-dsa`, `libcrux-aead`
- [#1412](https://github.com/cryspen/libcrux/pull/1412): Update dependencies: `libcrux-aead`, `libcrux-aesgcm`, `libcrux-chacha20poly1305`, `libcrux-ecdsa`, `libcrux-hkdf`, `libcrux-hmac`, `libcrux-kem`, `libcrux-ml-dsa`, `libcrux-psq`, `libcrux-ecdh`, `libcrux-hpke-rs`
- (libcrux-ecdh) [#1385](https://github.com/cryspen/libcrux/pull/1385): Update RNG trait bounds on key generation functions from rand v0.9 `Rng` trait to rand v0.10 `Rng` trait
- (libcrux-ecdsa) [#1385](https://github.com/cryspen/libcrux/pull/1385): Dropped `Rng` bounds on `rand` feature
- (libcrux-aesgcm) [#1385](https://github.com/cryspen/libcrux/pull/1385): Remove empty cargo feature `rand`

## [0.0.3] (2026-03-19)

### Changed

- [#1368](https://github.com/cryspen/libcrux/pull/1368): Update dependencies: `libcrux-sha3`, `libcrux-ed25519`, `libcrux-ml-dsa`, `libcrux-poly1305`, `libcrux-ml-kem`, `libcrux-kem`, `libcrux-digest`, `libcrux-chacha20poly1305`, `libcrux-aead`, `libcrux-psq`
- [#1259](https://github.com/cryspen/libcrux/pull/1259): Make this crate a pure Rust re-exporter of the `libcrux` sub-crates

## [0.0.2-alpha.3] (2024-07-23)
## [0.0.1] (2023-06-13)

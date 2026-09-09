# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- (libcrux-hmac-drbg) [#1558](https://github.com/celabshq/libcrux/pull/1558): fix panic in reseeding wrappers fill_bytes method
- (libcrux-psq) [#XXXX](https://github.com/celabshq/libcrux/pull/XXXX): Compare the session-resumption `pk_binder` in constant time, closing a timing side channel; return an error instead of panicking on an unexpected AEAD key-reuse during the handshake; clamp `recent_keys_upper_bound(0)` to `1` so it can no longer silently disable the responder's rate-limit cache
- (libcrux-kem) [#XXXX](https://github.com/celabshq/libcrux/pull/XXXX): Reject invalid/short seed lengths in `PublicKey::encapsulate_derand` without panicking
- (libcrux-sha3) [#XXXX](https://github.com/celabshq/libcrux/pull/XXXX): `hash::<LEN>` now enforces its length preconditions with a release-mode `assert!` instead of a `debug_assert!`, preventing a silently truncated/wrong-algorithm digest
- (libcrux-hmac) [#XXXX](https://github.com/celabshq/libcrux/pull/XXXX): `hmac()` now panics with a clear message instead of silently returning a shorter tag than requested
- (libcrux-ecdsa) [#XXXX](https://github.com/celabshq/libcrux/pull/XXXX): scalar validation now rejects over-length input instead of silently truncating it
- (libcrux-platform) [#XXXX](https://github.com/celabshq/libcrux/pull/XXXX): Fix a data race in x86 CPU feature detection
- (libcrux-ml-dsa) [#XXXX](https://github.com/celabshq/libcrux/pull/XXXX): `generate_key_pair`'s output-buffer length check is now enforced in release builds

### Changed

- (libcrux-macros) [#1550](https://github.com/celabshq/libcrux/pull/1550): Update syn dependency to 3.0
- (libcrux-secrets) [#1551](https://github.com/celabshq/libcrux/pull/1551) Update crabgrind to 0.3.1
- (libcrux-hmac-drbg) [#1558](https://github.com/celabshq/libcrux/pull/1558): rename `GenerateError::RequestInvalid` to `GenerateError::OutputTooLarge

### Added

- (hpke-rs) [#1539](https://github.com/celabshq/libcrux/pull/1539): Support for the
  post-quantum and PQ/T-hybrid algorithms of
  [draft-ietf-hpke-pq](https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-04),
  in the **libcrux provider only**, behind the new `draft-ietf-hpke-pq` feature.


## [0.0.5] (2026-07-15)

### Fixed

- (libcrux-sha3) [#1493](https://github.com/celabshq/libcrux/pull/1493): Provide instantiable `libcrux-traits` incremental hashing API
- (libcrux-secrets) [#1460](https://github.com/celabshq/libcrux/pull/1461): Fix incorrect cmp in aarch64 select/swap implementation
- (libcrux-sha3) [#1456](https://github.com/celabshq/libcrux/pull/1456): Fix out of bounds indexing in avx2 SHAKE-256 implementation
- (libcrux-sha3) [#1389](https://github.com/celabshq/libcrux/pull/1389): Fix partial Block output of incremental XOF API
- (libcrux-aes) [#1474](https://github.com/celabshq/libcrux/pull/1474): Enforce limits on plaintext and AAD lengths for AES-GCM, according to RFC 5116.
- (libcrux-aes) [#1528](https://github.com/celabshq/libcrux/pull/1528): Use constant time comparison for AES-GCM and AES-CCM tag check

### Changed

- [#1534](https://github.com/celabshq/libcrux/pull/XYZ): Update dependencies: `libcrux-blake2`, `libcrux-sha3`, `libcrux-sha2`, `libcrux-curve25519`, `libcrux-aes`, `libcrux-chacha20poly1305`, `libcrux-ml-dsa`, `libcrux-ml-kem`, `libcrux-rsa`, `libcrux-ed25519`, `libcrux-digest`, `libcrux-hmac`, `libcrux-p256`, `libcrux-aead`, `libcrux-hkdf`, `libcrux-ecdsa`, `libcrux-ecdh`, `libcrux-kem`, `libcrux-psq`, `hpke-rs`
- [#1505](https://github.com/celabshq/libcrux/pull/1505): Update dependency: `hpke-rs`
- (libcrux-traits, libcrux-sha3, libcrux-sha2, libcrux-blake2) [#1493](https://github.com/celabshq/libcrux/pull/1493): `finish` methods on digest traits consume the hasher
- (libcrux-secrets) [#1446](https://github.com/celabshq/libcrux/pull/1446): Remove const qualifier of secret types constructors
- (libcrux-secrets) [#1462](https://github.com/celabshq/libcrux/pull/1462): More robust casts instead of transmutes when checking secret independence
- (libcrux-sha3) [1454](https://github.com/celabshq/libcrux/pull/1454): `debug_assert` that generic `LEN` matches algorithm in `hash` function
- (libcrux-secrets) [#1484](https://github.com/celabshq/libcrux/pull/1484): seal scalar trait and synchronize De-/Classify trait impls for public/secret types
- (libcrux-secrets) [#1499](https://github.com/celabshq/libcrux/pull/1499): Replace `IntOps` and `EncodeOps` traits with inherent methods on `Secret` (https://github.com/Arul-Sujith)

### Added

- [#1474](https://github.com/celabshq/libcrux/pull/1474): Add support for AES-CCM from `libcrux-aes`, rename feature `aes_gcm` to `aes_aead`
- [#1382](https://github.com/celabshq/libcrux/pull/1382): Add support for HMAC-DRBG
- (libcrux-secrets) [#1446](https://github.com/celabshq/libcrux/pull/1446): Integrate valgrind requests when cfg `valgrind_ct_test` is set
- (libcrux-ml-dsa) [#1457](https://github.com/celabshq/libcrux/pull/1457): Add dependency on libcrux-secrets for optional valgrind integration
- (libcrux-kmac) [#1292](https://github.com/celabshq/libcrux/pull/1292): Add support for KMAC (https://github.com/kraemv)
- (libcrux-sha3) [#1292](https://github.com/celabshq/libcrux/pull/1292): Add support for incremental CShake (https://github.com/kraemv)
- (libcrux-hmac) [#1382](https://github.com/celabshq/libcrux/pull/1382): Add an incremental API

## [0.0.4] (2026-05-13)

### Fixed

- (libcrux-ml-dsa) [#1398](https://github.com/celabshq/libcrux/pull/1398): Fix incorrect AVX2 use_hint implementation
- (libcrux-ml-dsa) [#1395](https://github.com/celabshq/libcrux/pull/1395): Fully reduce iNTT inputs on AVX2
- (libcrux-chacha20poly1305) [#1386](https://github.com/celabshq/libcrux/pull/1386): Fix potential panic in `libcrux_chacha20poly1305::encrypt` (reported by @fg0x0)

### Removed

- (libcrux-hmac) [#1391](https://github.com/celabshq/libcrux/pull/1391): Remove support for HMAC-SHA1

### Changed

- [#1434](https://github.com/celabshq/libcrux/pull/1434): Update dependencies: `libcrux-hacl-rs`, `libcrux-poly1305`, `libcrux-curve25519`
- [#1433](https://github.com/celabshq/libcrux/pull/1433): Update dependencies: `libcrux-traits`, `libcrux-ed25519`, `libcrux-ml-kem`, `libcrux-kem`, `libcrux-aesgcm`, `libcrux-blake2`, `libcrux-chacha20poly1305`, `libcrux-p256`, `libcrux-curve25519`, `libcrux-sha3`, `libcrux-sha2`, `libcrux-hmac`, `libcrux-hkdf`, `libcrux-rsa`, `libcrux-ecdsa`, `libcrux-ecdh`, `libcrux-digest`, `libcrux-psq`, `libcrux-ml-dsa`, `libcrux-aead`
- [#1412](https://github.com/celabshq/libcrux/pull/1412): Update dependencies: `libcrux-aead`, `libcrux-aesgcm`, `libcrux-chacha20poly1305`, `libcrux-ecdsa`, `libcrux-hkdf`, `libcrux-hmac`, `libcrux-kem`, `libcrux-ml-dsa`, `libcrux-psq`, `libcrux-ecdh`, `libcrux-hpke-rs`
- (libcrux-ecdh) [#1385](https://github.com/celabshq/libcrux/pull/1385): Update RNG trait bounds on key generation functions from rand v0.9 `Rng` trait to rand v0.10 `Rng` trait
- (libcrux-ecdsa) [#1385](https://github.com/celabshq/libcrux/pull/1385): Dropped `Rng` bounds on `rand` feature
- (libcrux-aesgcm) [#1385](https://github.com/celabshq/libcrux/pull/1385): Remove empty cargo feature `rand`

## [0.0.3] (2026-03-19)

### Changed

- [#1368](https://github.com/celabshq/libcrux/pull/1368): Update dependencies: `libcrux-sha3`, `libcrux-ed25519`, `libcrux-ml-dsa`, `libcrux-poly1305`, `libcrux-ml-kem`, `libcrux-kem`, `libcrux-digest`, `libcrux-chacha20poly1305`, `libcrux-aead`, `libcrux-psq`
- [#1259](https://github.com/celabshq/libcrux/pull/1259): Make this crate a pure Rust re-exporter of the `libcrux` sub-crates

## [0.0.2-alpha.3] (2024-07-23)
## [0.0.1] (2023-06-13)

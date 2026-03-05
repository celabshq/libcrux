# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added 

- [#1339](https://github.com/cryspen/libcrux/pull/1339): Expose `Transport` struct and add external nonce-control feature

### Changed

- [#1339](https://github.com/cryspen/libcrux/pull/1339): Leave receiver nonce unchanged on decryption failure

## [0.0.7] (2026-02-12)

### Added

- [#1307](https://github.com/cryspen/libcrux/pull/1307): Expose additional functionalities on the DHKEM (https://github.com/jstuczyn)
- [#1298](https://github.com/cryspen/libcrux/pull/1298): Propagate import/export functions from CMC crate (https://github.com/georgio)

### Fixed

- [#1319](https://github.com/cryspen/libcrux/pull/1319): Propagate AEADError instead of panicking
- [#1301](https://github.com/cryspen/libcrux/pull/1301): Fix broken clamping check for imported X25519 secret keys

### Changed

- [#1324](https://github.com/cryspen/libcrux/pull/1324): Update dependencies: `libcrux-traits`, `libcrux-ecdh`, `libcrux-ml-kem`, `libcrux-ed25519`, `libcrux-kem`, `libcrux-chacha20poly1305`, `libcrux-aesgcm`, `libcrux-sha2`, `libcrux-hmac` `libcrux-ml-dsa`, `libcrux-hkdf`

## [0.0.6] (2026-01-22)

- [#1294](https://github.com/cryspen/libcrux/pull/1294): Allow import of X25519 private and public keys (https://github.com/georgio)
- [#1278](https://github.com/cryspen/libcrux/pull/1278): Allow import of secrets into sessions. (Breaking, since session binding to principal public keys is now optional.)
- [#1280](https://github.com/cryspen/libcrux/pull/1280): Update dependencies `libcrux-sha3`, `libcrux-ml-kem`, `libcrux-ml-dsa`
- [#1248](https://github.com/cryspen/libcrux/pull/1248):
    - Add signature-based authentication (breaking because ciphersuite IDs have changed)
    - Add AES-GCM 128 ciphersuite support
    - Add secret export API to `Session`
    - Add support for retrieving initiator authenticator mid-handshake from Responder
    - Update `Session` deserialization API to accept initiator authenticator instead of initiator DH public key only (breaking)
    - Update `Session` serialization API to require initiator authenticator and responder public keys as inputs to enforce serializer has access to all information needed to deserialize

## [0.0.5] (2025-11-05)

- [#1108](https://github.com/cryspen/libcrux/pull/1108): Allow using Classic McEliece for PQ KEM
- [#1091](https://github.com/cryspen/libcrux/pull/1091): Allow derivation of an Unregistered PSK
- [#1081](https://github.com/cryspen/libcrux/pull/1081): Session storage
- [#1079](https://github.com/cryspen/libcrux/pull/1079): Use TLSCodec for Serialization/Deserialization in PSQv1
- [#1048](https://github.com/cryspen/libcrux/pull/1048): PSQv2

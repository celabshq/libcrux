# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] (2026-10-06)

### Added

- [#1539](https://github.com/celabshq/libcrux/pull/1539): Support for the
  post-quantum and PQ/T-hybrid algorithms of
  [draft-ietf-hpke-pq](https://datatracker.ietf.org/doc/html/draft-ietf-hpke-pq-04),
  in the **libcrux provider only**, behind the new `draft-ietf-hpke-pq` feature.
  Validated against the draft's published test vectors.
  - Single-stage KDFs `KdfAlgorithm::Shake128 = 0x0010` and
    `KdfAlgorithm::Shake256 = 0x0011`, with the corresponding one-stage HPKE
    key schedule (`KeySchedule`, `Export`, `DeriveKeyPair`).
  - ML-KEM-512 (`0x0040`), ML-KEM-768 (`0x0041`), and ML-KEM-1024 (`0x0042`) as
    HPKE KEMs.
  - The PQ/T-hybrid KEMs `KemAlgorithm::MlKem768P256 = 0x0050`,
    `KemAlgorithm::MlKem1024P384 = 0x0051`, and `MLKEM768-X25519` (`0x647a`,
    via X-Wing draft-06), per
    [draft-irtf-cfrg-concrete-hybrid-kems-03](https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-concrete-hybrid-kems-03).
    
### Changed

- (hpke-rs, hpke-rs-libcrux, hpke-rs-crypto) [#1612](https://github.com/celabshq/libcrux/pull/1612): Caller-provided randomness. `HpkeLibcrux<R>` accepts any `TryCryptoRng` via `Hpke::new_with_rng`, and all HPKE randomness is drawn from it. `HpkeLibcruxPrng` is now opaque (`try_new`, `try_from_seed`, and a panicking `from_seed`). The system RNG sits behind the provider's `sys-rng` feature, which the `hpke-rs` `libcrux` feature enables; `libcrux-no-sys-rng` gives the provider without it. `HpkeCrypto::prng` and `HpkeTestRng` are replaced by `HpkeDefaultPrng::try_prng`, and the `deterministic-prng`/`hpke-test-prng` features are removed.

### Deprecated

- [#1539](https://github.com/celabshq/libcrux/pull/1539): The `draft-connolly-cfrg-hpke-mlkem` feature is deprecated in favour of
  `draft-ietf-hpke-pq` (it emits a build warning, and the two features cannot be
  enabled at the same time).

## [0.7.0] (2026-07-15)

### Changed

- [#1534](https://github.com/celabshq/libcrux/pull/1534): Update dependencies: `hpke-rs-crypto`, `hpke-rs-rust-crypto`, `libcrux-sha3`, `hpke-rs-libcrux`
- [#147](https://github.com/celabshq/hpke-rs/pull/147): Add P384 and P521 DHKEM support for the libcrux provider via RustCrypto crates guarded behind the `libcrux-rustcrypto-p-curves` feature flag.
- [#146](https://github.com/celabshq/hpke-rs/pull/146): Add support for ML-KEM768 and ML-KEM1024 gated behind the `draft-connolly-cfrg-hpke-mlkem` feature flag.

## [0.6.1] - 2026-03-20

- Update crypto providers

## [0.6.0] - 2026-02-13

### Added

- [#127](https://github.com/cryspen/hpke-rs/pull/127): Add support for ML-KEM-768 (`KemAlgorithm::MlKem768 = 0x0041`) and ML-KEM-1024 (`KemAlgorithm::MlKem1024 = 0x0042`) to the RustCrypto provider, and add X-Wing support to the RustCrypto provider using the `x-wing` and `ml-kem` crates.

### Fixed

- [#127](https://github.com/cryspen/hpke-rs/pull/127): Fix `KemAlgorithm::TryFrom<u16>` mapping where `0x004D` incorrectly resolved to `XWingDraft06` instead of `XWingDraft06Obsolete`.
- [#123](https://github.com/cryspen/hpke-rs/pull/123): Fix potential overflow in context counter and switch to use u64.
- [#128](https://github.com/cryspen/hpke-rs/pull/128): Return errors when trying to use open/seal with export only ciphersuite and when using kdf export with an output that's too long (instead of truncating it)

### Changed
- [#128](https://github.com/cryspen/hpke-rs/pull/128): Added zeroize for Context and ephemeral private kem keys

## [0.5.1] - 2026-02-02

- [#114](https://github.com/celabshq/libcrux/pull/114): Update dependency `libcrux-sha3`

## [0.5.0] - 2025-12-16

- [#103](https://github.com/cryspen/hpke-rs/pull/103) Breaking: Use correct algorithm ID for XWing (`KemAlgorithm::XWingDraft06`) and deprecate old one (still available as `KemAlgorithm::XWingDraft06Obsolete`).

## [0.4.0] - 2025-12-01

- Updated dependencies

## [0.3.0] - 2025-07-01

- [#98](https://github.com/cryspen/hpke-rs/pull/98): add support for AES-GCM to the Libcrux provider

- [#77]():
  - `rustcrypto` and `libcrux` features expose the corresponding crypto providers
  - trait types are re-exported as `hpke_types` for convenience
- [#72](https://github.com/cryspen/hpke-rs/pull/72):
  -  add support for X-Wing KEM
  -  upgrade rand dependency from 0.8 -> 0.9
  -  replace Evercrypt provider with Libcrux provider
- [#66](https://github.com/franziskuskiefer/hpke-rs/pull/66): add support for secp256k1 curve. This adds `DhKemK256 = 0x0016` to the `KemAlgorithms`

## [0.2.0] - 2023-12-01

- [#59](https://github.com/franziskuskiefer/hpke-rs/pull/59): hpke-rs-rust-crypto: make deterministic-prng enable the std feature
- [#56](https://github.com/franziskuskiefer/hpke-rs/pull/56): CI: check no-std support
- [#58](https://github.com/franziskuskiefer/hpke-rs/pull/58): no-std-ify hpke-rs-rust-crypto (some more)
- [#57](https://github.com/franziskuskiefer/hpke-rs/pull/57): switch from x25519-dalek-ng to x25519-dalek
- [#55](https://github.com/franziskuskiefer/hpke-rs/pull/55): rm `RwLock` from `Hpke` and no-std-ify the `hpke-rs` library
- [#53](https://github.com/franziskuskiefer/hpke-rs/pull/53): rm getrandom dep
- [#50](https://github.com/franziskuskiefer/hpke-rs/pull/50): no-std-ify hpke-rs-crypto
- [#49](https://github.com/franziskuskiefer/hpke-rs/pull/49): hpke-rs-crypto: make serde opt-in
- [#48](https://github.com/franziskuskiefer/hpke-rs/pull/48): no-std-ify hpke-rs-rust-crypto
- [#47](https://github.com/franziskuskiefer/hpke-rs/pull/47): hpks-rs-crypto: simplify Cargo.toml

## [0.1.2] - 2023-11-21

- Updated TLS codec dependency
- [#44](https://github.com/franziskuskiefer/hpke-rs/pull/44): implement std::error::Error for HpkeError

## [0.1.1] - 2023-06-22

### Changed

- Updated crypto providers
- Use variable length TLS encoding as required by TLS for serialization

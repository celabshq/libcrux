# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.2] (2026-10-07)

### Fixed

- [#1558](https://github.com/celabshq/libcrux/pull/1558): fix panic in reseeding wrappers fill_bytes method

### Changed

- [#1612](https://github.com/celabshq/libcrux/pull/1612): `HmacDrbg::new_from_sys_rng` is now behind the new `sys_rng` feature; enabling `rand` alone no longer pulls in `rand/sys_rng`, so platforms without a system RNG (e.g. wasm) can use the `rand` integration.
- [#1558](https://github.com/celabshq/libcrux/pull/1558): rename `GenerateError::RequestInvalid` to `GenerateError::OutputTooLarge`


## [0.0.1] (2026-07-15)

- [#1382](https://github.com/celabshq/libcrux/pull/1382): Initial version of this crate

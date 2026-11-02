# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.7] (Unreleased)

### Changed

- [#XXX](https://github.com/celabshq/libcrux/pull/XXX): **breaking** Remove unused code: the `bignum::{bignum32, bignum64, bignum256, bignum256_32, bignum4096, bignum4096_32}` modules, the unused 32-bit and 64-bit functions in `bignum::{base, bignum_base}`, the `fstar::{uint16, uint32}` modules, `fstar::uint8::gte_mask`, and the unused `fstar::uint128` functions

## [0.0.6] (2026-10-07)

### Changed

- [#1601](https://github.com/celabshq/libcrux/pull/1601): **breaking** Introduce feature `alloc` that gates any heap allocations

## [0.0.5] (2026-05-13)

### Changed

- [#1391](https://github.com/celabshq/libcrux/pull/1391): Remove some unnecessary allocations

## [0.0.4] (2025-11-05)

## [0.0.3] (2025-06-30)

- [#993](https://github.com/celabshq/libcrux/pull/993): Update dependency `macros`

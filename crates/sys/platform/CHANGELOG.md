# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- [#1591](https://github.com/celabshq/libcrux/pull/1591): Avoid data races in x86 feature detection
- [#1591](https://github.com/celabshq/libcrux/pull/1591): Handle feature detection on Intel SGX and CPUs without cpuid
- [#1592](https://github.com/celabshq/libcrux/pull/1592): Check xgetbv output for AVX2 availability


## [0.0.3] (2026-01-12)

- [#1265](https://github.com/celabshq/libcrux/pull/1265): Simplify AArch64 feature checks, use atomics instead of static mut (@jrose-signal)

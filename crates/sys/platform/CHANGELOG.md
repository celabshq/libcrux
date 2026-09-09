# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- [#XXXX](https://github.com/celabshq/libcrux/pull/XXXX): Fix a data race in x86 CPU feature detection where two threads could concurrently write `CPU_ID` before either observed initialization as complete

## [0.0.3] (2026-01-12)

- [#1265](https://github.com/celabshq/libcrux/pull/1265): Simplify AArch64 feature checks, use atomics instead of static mut (@jrose-signal)

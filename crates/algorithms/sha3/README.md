# Libcrux SHA3

![verified]

This crate implements [SHA3] (FIPS 202).

It provides 
- a portable implementation
- an AVX2 optimised implementation
- a Neon optimised implementation

## `no_std` support

This crate supports `no_std` targets and is free of heap allocations.

## Verification

The Rust source is verified with hax (<https://github.com/cryspen/hax>), which
extracts it to F\* (<https://fstar-lang.org>); the portable, AVX2, and Neon
backends are covered. The per-function status (runtime safety or functional
correctness against the Hacspec specification) is in
[`proofs/verification_status.md`](proofs/verification_status.md); the F\* proofs
are under [`proofs/fstar/`](proofs/fstar/). See [`PROOFS.md`](../../../PROOFS.md)
at the repository root for the index across crates.

[SHA3]: https://csrc.nist.gov/pubs/fips/202/final
[verified]: ../../../.assets/verified-brightgreen.svg

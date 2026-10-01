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
backends are covered. The one-shot hashing APIs are proven functionally correct
against the Hacspec specification; the incremental APIs are proven panic-free.

See [`proofs/README.md`](proofs/README.md) for the proofs: where the main
correctness theorems live, the spec ← equivalence layering, the per-function
verification status, and the coverage boundaries.

[SHA3]: https://csrc.nist.gov/pubs/fips/202/final
[verified]: ../../../.assets/verified-brightgreen.svg

# Libcrux SHA3

![pre-verification]

This crate implements [SHA3] (FIPS 202).

It provides 
- a portable implementation
- an AVX2 optimised implementation
- a Neon optimised implementation

## `no_std` support

This crate supports `no_std` targets and is free of heap allocations.

[SHA3]: https://csrc.nist.gov/pubs/fips/202/final
[pre-verification]: ../../../.assets/pre_verification-orange.svg

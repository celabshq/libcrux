# Libcrux SHA3

![pre-verification]

This crate implements [SHA3] (FIPS 202).

It provides 
- a portable implementation
- an AVX2 optimised implementation
- a Neon optimised implementation

## `no_std` support

This crate supports `no_std` targets and is free of heap allocations.

## Zeroization

One-shot hashes wipe their internal state before returning. The incremental
and XOF states (`Shake128Xof`, `Shake256Xof`, `CShake128`/`CShake256`, the
digest-trait types, and the `neon::x2` and `avx2::x4` incremental
`KeccakState`s) are wiped on drop.

`portable::KeccakState` is `Copy`, so it cannot wipe itself on drop. If you
absorb secret data into it, call `libcrux_secrets::Zeroize::zeroize` on it
when done. Outputs written to your buffers are yours to wipe.

Not wiped:

- The per-block copies of the input made while absorbing. The caller still
  holds the input, and wiping them made hashing up to 20% slower.
- Temporaries inside the Keccak permutation, and copies the compiler makes on
  its own (see `libcrux-secrets`).

The C code extracted from this crate does not zeroize yet.

[SHA3]: https://csrc.nist.gov/pubs/fips/202/final
[pre-verification]: ../../../.assets/pre_verification-orange.svg

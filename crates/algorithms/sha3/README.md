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

The Keccak state is wiped at the end of every one-shot hash (except NEON x2,
see below), together with the padded last input block and the buffers used to
write the output. The incremental and XOF states (`Shake128Xof`,
`Shake256Xof`, `CShake128`/`CShake256`, the digest-trait types and the x2/x4
incremental `KeccakState`s) are wiped on drop.

`portable::KeccakState` is `Copy` and cannot wipe itself. Callers that absorb
secret data into it must call `libcrux_secrets::Zeroize::zeroize` on it when
done. The hash outputs written to caller buffers are the caller's to wipe.

Temporaries inside the Keccak permutation, and copies the compiler makes on its
own, are not wiped (see `libcrux-secrets`).

The per-block copies of the input that `load_block` makes before absorbing a
block are not wiped. The caller still holds the input, and wiping these copies
once per block made hashing up to 20% slower. They are in:

- `src/simd/portable.rs`, `load_block`: `state_flat`
- `src/simd/arm64.rs`, `load_block`: `u`
- `src/simd/avx2.rs`, `load_block`: `u8s` (both of them)

The Keccak state of the one-shot NEON x2 functions (`keccak2` in
`src/generic_keccak/simd128.rs`) is not wiped either. Wiping it takes its
address, which keeps the compiler from holding the state in registers, and made
hashing up to ~20% slower on large inputs. Without the wipe the state mostly
lives in registers; spills to the stack are not covered (see
`libcrux-secrets`).

The C code extracted from this crate does not zeroize yet.

[SHA3]: https://csrc.nist.gov/pubs/fips/202/final
[pre-verification]: ../../../.assets/pre_verification-orange.svg

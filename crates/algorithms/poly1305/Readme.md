# Poly1305

## `no_std` support

This crate supports `no_std` targets, but requires the presence of a global allocator.

## Verification
![verified-hacl]

This crate contains safe Rust that was compiled from verified C
originating in the [HACL* project](https://hacl-star.github.io).

> The code for [these] algorithms is formally verified using the F*
> verification framework for memory safety, functional correctness, and
> secret independence (resistance to some types of timing
> side-channels).
-- [The HACL* repository](https://github.com/hacl-star/hacl-star?tab=readme-ov-file#a-high-assurance-cryptographic-library)

For more details on the compilation from C to Rust, please refer to
["Compiling C to Safe Rust,
Formalized"](https://arxiv.org/abs/2412.15042) by Aymeric Fromherz and
Jonathan Protzenko.

[verified-hacl]: ../../../.assets/verified-hacl-rs-brightgreen.svg

# Digest

This crate provides a usable interface to `libcrux-sha2`, `libcrux-sha3` and `libcrux-blake2`.

## Verification

### Blake2 and Sha2
![verified-hacl]

`libcrux-blake2` and `libcrux-sha2` contain safe Rust that was compiled from verified C
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

### Sha3
![pre-verification]

**NOTE:** The Sha3 implementation has not been formally verified yet.

[verified-hacl]: ../../../.assets/verified-hacl-rs-brightgreen.svg
[pre-verification]: ../../../.assets/pre_verification-orange.svg

# AEAD

This crate provides a usable interface to `libcrux-chacha20poly1305` and `libcrux-aesgcm`.

## Verification

### `libcrux-chacha20poly1305`
![verified-hacl]

`libcrux-chacha20poly1305` contains safe Rust that was compiled from verified C
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

**NOTE:** The XChacha20Poly1305 wrapper has not been formally verified yet.

### `libcrux-aesgcm`
![pre-verification]

The implementations of AES-GCM 128 and AES-GCM 256 have not been formally verified yet.

[verified-hacl]: ../../../.assets/verified-hacl-rs-brightgreen.svg
[pre-verification]: ../../../.assets/pre_verification-orange.svg

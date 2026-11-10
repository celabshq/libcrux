# HMAC

This crate implements HMAC on SHA 2 (except for SHA 224) and SHA 3.

## `no_std` support

This crate supports `no_std` targets.

## Zeroization

The padded key, `K ^ ipad`, `K ^ opad` and the intermediate hash states and
inner hash values are wiped before returning (one-shot API) or when the
streaming state is dropped. Tags written to caller buffers are the caller's to
wipe.

The zeroization calls in the HACL\*-derived code were added by hand and are
not covered by the HACL\* proofs.

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

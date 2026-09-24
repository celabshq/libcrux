# NIST Curve P-384

This crate implements ECDH for NIST curve P-384. It is based on P-384
base field arithmetic from [Bas Spitter's fork of
fiat-crypto][fiat-crypto] and [AU-Curves][aucurves].

⚠️ NOTE: This crate serves as an internal dependency to other `libcrux`
crates and SHOULD NOT be used directly.

## `no_std` support

This crate supports `no_std` targets, and is free of heap allocations.

## Secret Independence
The code in this crate aims to be secret-independent on the source code level.

## Verification
Please refer to the documentation provided in the
[fiat-crypto][fiat-crypto] and [AU-Curves][aucurves] repositories for
details on the verification of the vendored code.

[fiat-crypto]: https://github.com/spitters/fiat-crypto
[aucurves]: https://github.com/AU-COBRA/AUCurves

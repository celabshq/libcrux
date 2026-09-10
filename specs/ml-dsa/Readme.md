# Hacspec-style ML-DSA specification

This is a hacspec-style Rust implementation of ML-DSA, closely following FIPS-204. Its purpose
is to serve as a reference implementation for verifying functional correctness of more efficient
implementations.

## Extraction via hax into Lean

Prerequisites:
* [Lean](https://lean-lang.org/install/)
* [cargo](https://rust-lang.org/tools/install/)
* [cargo-binstall](https://github.com/cargo-bins/cargo-binstall#installation)
* (hax, charon, aeneas will be downloaded automatically)

To extract the Lean code, run the following in the `specs` directory:
```
cargo hax extract hacspec-ml-dsa
```

To type check the extracted Lean run the following in the `specs/ml-dsa/proofs/hacspec-ml-dsa/lean` directory:
```
lake exe cache get
lake build
```
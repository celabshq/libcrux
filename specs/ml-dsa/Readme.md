# Hacspec-style ML-DSA specification

This is a hacspec-style Rust implementation of ML-DSA, closely following FIPS-204. Its purpose
is to serve as a reference implementation for verifying functional correctness of more efficient
implementations.

## Extraction via hax into Lean

Prerequisites:
* [Lean](https://lean-lang.org/install/)
* [cargo](https://rust-lang.org/tools/install/)
* hax 0.4.0 on `PATH` (`cargo hax`); charon and aeneas are downloaded
  automatically on first use

To extract the Lean code, run the following in the `specs` directory:
```
cargo hax extract hacspec-ml-dsa
```

To type check the extracted Lean run the following in the `specs/ml-dsa/proofs/hacspec-ml-dsa/lean` directory:
```
lake exe cache get
lake build
```
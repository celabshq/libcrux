# Hacspec-style ML-KEM specification

This is a hacspec-style Rust implementation of ML-KEM, closely following FIPS-203. Its purpose
is to serve as a reference implementation for verifying functional correctness of more efficient
implementations.

## Extraction via hax into Lean

Prerequisites:
* hax 0.4.0 (https://github.com/cryspen/hax)
* Lean (https://lean-lang.org/install/)

To extract the Lean code, run the following in the `specs` directory:
```
cargo hax extract hacspec-ml-kem
```

To type check the extracted Lean run the following in the `specs/ml-kem/proofs/lean` directory:
```
lake exe cache get
lake build
```
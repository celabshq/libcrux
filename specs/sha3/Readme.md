# Hacspec-style SHA-3 specification

This is a hacspec-style Rust implementation of SHA-3, closely following FIPS-202. Its purpose
is to serve as a reference implementation for verifying functional correctness of more efficient
implementations.

## Extraction via hax

### F*

Prerequisites:
* Hax 0.3.6 (https://github.com/cryspen/hax/tree/87ba96831ecfeb7dbb54efcf97036fbc5f25bc71)
* F* 2026/03/24
  (https://github.com/FStarLang/FStar/releases/tag/v2026.03.24)

Run `hax_fstar.sh extract` to produce the F* files, and `hax_fstar.sh prove` to type-check them.

### Lean

Prerequisites:
* hax 0.4.0 (https://github.com/cryspen/hax)
* Lean (https://lean-lang.org/install/)

To extract the Lean code, run the following in the `specs` directory:
```
cargo hax extract hacspec-sha3
```

To type check the extracted Lean run the following in the `specs/sha3/proofs/lean` directory:
```
lake exe cache get
lake build
```
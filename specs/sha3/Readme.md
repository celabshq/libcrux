# Hacspec-style SHA-3 specification

This is a hacspec-style Rust implementation of SHA-3, closely following FIPS-202. Its purpose
is to serve as a reference implementation for verifying functional correctness of more efficient
implementations.

## Extraction via HAX

### F*

Prerequisites:
* Hax 0.3.6 (https://github.com/cryspen/hax/tree/87ba96831ecfeb7dbb54efcf97036fbc5f25bc71)
* F* 2026/03/24
  (https://github.com/FStarLang/FStar/releases/tag/v2026.03.24)

Run `hax_fstar.sh extract` to produce the F* files, and `hax_fstar.sh prove` to type-check them.

### Lean

Prerequisites:
* Hax from `hax-evit`, commit `1f85fc1`
  https://github.com/cryspen/hax-evit/tree/1f85fc13b9967080cc657863e2000ba5d4aa8647
  (This is a not publicly available yet.)
* Aeneas `8d2077c`
  (https://github.com/cryspen/aeneas/releases/tag/nightly-2026.06.04)

Run `hax_aeneas.py` to extract. Run `cd proofs/aeneas-lean && lake update && lake build` to
type-check.
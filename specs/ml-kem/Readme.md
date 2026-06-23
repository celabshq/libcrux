# Hacspec-style ML-KEM specification

This is a hacspec-style Rust implementation of ML-KEM, closely following FIPS 203. Its purpose
is to serve as a reference implementation for verifying functional correctness of more efficient
implementations.

**Do not use this implementation for other applications! Besides being slow, it may be vulnerable to side-channel attacks.**

## Extraction via HAX

### Lean

Prerequisites:
* Hax from `hax-evit`, commit `1f85fc1`
  https://github.com/cryspen/hax-evit/tree/ffdf432705d409b62ec025d253a340234b59766f
  (This is a not publicly available yet.)
* Aeneas `8d2077c`
  (https://github.com/cryspen/aeneas/releases/tag/nightly-2026.06.04)

Run `hax_aeneas.py` to extract.
Depending on the aeneas binary you have, you may have to run
`SKIP_VERSION_CHECK=1 hax_aeneas.py` instead.
Run `cd proofs/aeneas-lean && lake update && lake build` to
type-check.
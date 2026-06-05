# ML-KEM — Formal Verification

This directory holds the [F\*](https://www.fstar-lang.org/) verification of the
`libcrux-ml-kem` implementation of ML-KEM (FIPS 203). The proofs are produced
from the Rust source by [hax](https://github.com/hacspec/hax), which extracts
the annotated Rust into F\*; F\* then discharges the proof obligations against a
hacspec-style reference specification.

```
src/*.rs  ──hax──▶  proofs/fstar/extraction/*.fst(i)  ──F*/Z3──▶  verified
   ▲ #[hax_lib::requires/ensures/...]      │ spec: proofs/fstar/spec, specs/ml-kem
   └────────────────────────────────────── └ make check/<Module>.fst
```

## Top-level theorems

The verification establishes two classes of theorem for the optimized
implementation (all three backends — Portable, AVX2, Neon):

1. **Memory & panic safety.** The full public API
   (`mlkem{512,768,1024}::{generate_key_pair, encapsulate, decapsulate}` and the
   incremental API) and essentially all internal functions are proven **free of
   panics and arithmetic overflow**, and to respect every callee precondition.
   In F\* terms each such function carries a discharged `requires`/`ensures`
   contract (or, at minimum, `verification_status(panic_free)`). Decode entry
   points (`KeyPair::from_bytes`, `EncapsState::try_from_bytes`) additionally
   bounds-check every decoded coefficient and return `Error::InvalidInput`
   rather than trusting their input.

2. **Functional correctness of the arithmetic core.** The number-theoretic
   transform (forward and inverse, all layers), Montgomery and Barrett
   reduction, coefficient (de)serialization, (de)compression, and the
   binomial/rejection sampling carry F\* postconditions that tie the
   bit-twiddling SIMD code to the mathematical reference spec
   (`Hacspec_ml_kem.*`, `Spec.Utils.*`) **modulo q = 3329** — i.e. the
   vectorized code computes the same field-element arithmetic as the spec.

3. **Cross-backend equivalence.** Portable, AVX2, and Neon each implement the
   same `Libcrux_ml_kem.Vector.Traits.t_Operations` trait contract, so the
   per-operation specs above hold uniformly across all three SIMD backends, and
   the generic ML-KEM layer is verified once against that trait.

These are component-level functional-correctness + safety theorems, not a single
end-to-end IND-CCA security proof (that is out of scope for this tree).

## Verification state

The authoritative, auto-generated tally lives in
[`ml_kem_verification_status.md`](./ml_kem_verification_status.md) (regenerate
with `generate_verification_status.py`). Headline as of the last run:

| Metric | Count | % |
| --- | --- | --- |
| Total functions | 978 | |
| **Panic-safe** (panic-free + spec-bearing) | 954 | **97.5%** |
| &nbsp;&nbsp;— cites high-level hacspec | 116 | 11.9% |
| &nbsp;&nbsp;— interval/bounds ensures | 68 | 7.0% |
| &nbsp;&nbsp;— other non-trivial ensures | 250 | 25.6% |
| &nbsp;&nbsp;— panic-free only | 520 | 53.2% |
| Lax (admitted) | 5 | 0.5% |
| Unverified (not extracted) | 19 | 1.9% |

Per backend, the SIMD `Vector` trait and its primitives carry **0 lax and 0
unverified** on all three (Portable, AVX2, Neon): every NTT-layer trait method
(`op_{,inv_}ntt_layer_{1,2,3}_step`, `op_ntt_multiply`), arithmetic primitive,
and rejection sampler is functionally verified (Neon's `rej_sample` delegates to
the verified portable scalar sampler).

Known remaining gaps (see the status file for the live list): 5 admitted
(`lax`) — the generic rejection-sampling helpers and a few incremental-API
`From`-instance bodies (a hax trait-precondition limitation) — and 19 not
extracted to F\* (`pqcp` and `lib` glue), plus a prefix-form `serialize_vector`
contract used by `to_bytes_compressed`.

## Reproducing the results

Toolchain (pinned): F\* `2026.03.24`, Z3 `4.13.3`, `cargo-hax` `0.3.7`.

```sh
# 1. Extract Rust → F* (regenerates proofs/fstar/extraction/*.fst(i))
cd libcrux-ml-kem
./hax.py extract

# 2. Verify everything with F* (uses .fstar-cache/ for incremental checking)
cd proofs/fstar/extraction
make all                                   # full crate
make check/Libcrux_ml_kem.Vector.Neon.fst  # a single module

# 3. Regenerate the status table
cd ../../..            # back to libcrux-ml-kem/
python3 proofs/generate_verification_status.py

# 4. Run the implementation test suite
cargo test --features simd128              # Neon  (on aarch64)
cargo test --features simd256              # AVX2  (on x86-64)
cargo test                                 # Portable
```

`make all` from a warm `.fstar-cache` re-checks the whole crate in a few
minutes; a cold run is longer. A module verifies cleanly when F\* prints
`Verified module: <M>` / `All verification conditions discharged successfully`.

## Layout

| Path | Contents |
| --- | --- |
| `fstar/extraction/` | hax-extracted `.fst`/`.fsti` + the `Makefile` (the proofs) |
| `fstar/spec/`, `../../specs/ml-kem/` | the hacspec reference spec + commute lemmas |
| `ml_kem_verification_status.md` | auto-generated per-function proof-tier tally |
| `generate_verification_status.py` / `.sh`, `verification_status.config.json` | status generator |

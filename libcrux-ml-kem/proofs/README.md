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

The verification establishes the following for the optimized implementation
(all three backends — Portable, AVX2, Neon):

1. **End-to-end functional correctness of the public KEM API.** For every
   parameter set (ML-KEM-512/768/1024), the public `generate_key_pair`,
   `encapsulate`, and `decapsulate` are proven to return **byte-for-byte the
   same result as the `hacspec_ml_kem` executable reference specification** (a
   FIPS-203 transcription — the spec the NIST KATs run against). The F\*
   `ensures` clauses cite `Hacspec_ml_kem.Ind_cca.{generate_keypair,
   encapsulate, decapsulate}`:
   - `generate_key_pair(seed)` ⇒ `pk.value == ek ∧ sk.value == dk`,
     where `(ek, dk) = generate_keypair(seed)`;
   - `encapsulate(pk, r)` ⇒ `ct.value == ciphertext ∧ ss == shared`,
     where `(shared, ciphertext) = encapsulate(pk.value, r)`;
   - `decapsulate(sk, ct)` ⇒ `ss == decapsulate(sk.value, ct.value)`.

   The same postconditions back the generic `Ind_cca.{generate_keypair,
   encapsulate, decapsulate}` layer (proven once, generic over the SIMD
   backend) and the unpacked key-API variants. The reference returns a
   `Result`; equality is asserted on the `Ok` case (the negligible
   rejection-sampling failure branch is unconstrained).

2. **Memory & panic safety.** The full public API (incl. the incremental API)
   and essentially all internal functions are proven **free of panics and
   arithmetic overflow**, and to respect every callee precondition. Decode
   entry points (`KeyPair::from_bytes`, `EncapsState::try_from_bytes`)
   additionally bounds-check every decoded coefficient and return
   `Error::InvalidInput` rather than trusting their input.

3. **Functional correctness of the arithmetic core.** The number-theoretic
   transform (forward and inverse, all layers), Montgomery and Barrett
   reduction, coefficient (de)serialization, (de)compression, and the
   binomial/rejection sampling carry F\* postconditions that tie the
   bit-twiddling SIMD code to the mathematical reference spec
   (`Hacspec_ml_kem.*`, `Spec.Utils.*`) **modulo q = 3329**.

4. **Cross-backend equivalence.** Portable, AVX2, and Neon each implement the
   same `Libcrux_ml_kem.Vector.Traits.t_Operations` trait contract, so the
   per-operation specs above hold uniformly across all three SIMD backends, and
   the generic ML-KEM layer is verified once against that trait.

Together these give end-to-end functional correctness — the optimized KEM
computes exactly the FIPS-203 reference — plus memory and panic safety. They
are not an IND-CCA *security* proof (the reference's cryptographic security is
out of scope for this tree).

## Verification state

The authoritative, auto-generated tally lives in
[`ml_kem_verification_status.md`](./ml_kem_verification_status.md) (regenerate
with `generate_verification_status.py`). Headline as of the last run:

| Metric | Count | % |
| --- | --- | --- |
| Total functions | 963 | |
| **Panic-safe** (panic-free + spec-bearing) | 957 | **99.4%** |
| &nbsp;&nbsp;— cites high-level hacspec | 212 | 22.0% |
| &nbsp;&nbsp;— interval/bounds ensures | 54 | 5.6% |
| &nbsp;&nbsp;— other non-trivial ensures | 274 | 28.5% |
| &nbsp;&nbsp;— panic-free only | 417 | 43.3% |
| Lax (admitted) | 3 | 0.3% |
| Unverified (not extracted) | 3 | 0.3% |

The NIST/PQCP C-ABI shim (`src/pqcp/`, the `crypto_kem_*` boundary functions)
is **excluded** from this tally as out-of-scope FFI/boundary code; see
`_excluded_modules` in `verification_status.config.json`.

Per backend, the SIMD `Vector` trait and its primitives carry **0 lax and 0
unverified** on all three (Portable, AVX2, Neon): every NTT-layer trait method
(`op_{,inv_}ntt_layer_{1,2,3}_step`, `op_ntt_multiply`), arithmetic primitive,
and rejection sampler is functionally verified (Neon's `rej_sample` delegates to
the verified portable scalar sampler).

Known remaining gaps (see the status file for the live list): 3 admitted
(`lax`) and 3 not extracted to F\*.

- **`lax`** — (1) `sampling::sample_from_xof`: its rejection-sampling
  `while !done` loop has no statically decreasing measure, so it is not
  provably terminating, and `panic_free` does not exempt termination. The
  bounded inner helper it drives, `sample_from_uniform_distribution_next`, **is**
  fully verified (it establishes the per-coefficient
  `≤ COEFFICIENTS_IN_RING_ELEMENT` bound). (2–3) two incremental-API
  `From`-instance bodies (`From<&MlKemPublicKeyUnpacked> for PublicKey2`,
  `From<KeyPair> for MlKemKeyPairUnpacked`), blocked by a hax limitation that
  forces trivial preconditions on core-trait (`From`) impls.
- **not extracted** — the three functions in `src/lib.rs` (crate-level glue,
  filtered out of extraction by hax).

`KeyPair::to_bytes_compressed` is panic-free-verified modulo one `assume` (a
`From`-instance `sk`-equality F\* won't reduce through the typeclass
projector); see the status file's "hax limitations" finding.

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

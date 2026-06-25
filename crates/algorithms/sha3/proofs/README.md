# SHA-3 proofs

Machine-checked **functional-correctness** proofs (F*, via [hax](https://github.com/hacspec/hax))
for this crate's SHA-3 / SHAKE implementations. Every public hashing API is proven to compute
exactly what the Hacspec specification (the `hacspec_sha3` crate) says it should.

**Status:** zero admits, 0 unverified functions, 269 / 269 functions panic-safe — see
[`verification_status.md`](verification_status.md). Trust reduces to the F* toolchain plus the
per-lane SIMD intrinsic axioms in `Libcrux_intrinsics.{Arm64,Avx2}_extract` (the only external
assumptions; noted at the relevant lemmas).

## Where the main theorems live

The top-level correctness theorems — one per algorithm, per backend — are in three clearly-named
files under `fstar/equivalence/`. Start here:

| File | Backend | Theorems | Guarantee (per algorithm) |
| --- | --- | --- | --- |
| `EquivImplSpec.Correctness.Portable.fst` | Portable (N=1) | `lemma_{sha224,sha256,sha384,sha512,shake128,shake256}_portable` | `Libcrux_sha3.Portable.<algo> digest data == Hacspec_sha3.Sha3.<algo> data` |
| `EquivImplSpec.Correctness.Neon.fst` | Neon / Arm64 (N=2) | `lemma_{sha224,sha256,sha384,sha512,shake128,shake256}_arm64` | `Libcrux_sha3.Neon.<algo> digest data == Hacspec_sha3.Sha3.<algo> data` (lane-0 of the 2-way driver) |
| `EquivImplSpec.Correctness.Avx2.fst` | AVX2 x4 (N=4) | `lemma_shake256_x4_avx2` | `Libcrux_sha3.Avx2.X4.shake256` output lane `l` `== Hacspec_sha3.Sha3.shake256 (data[l])` |

The spec hashers (`sha3_224_/256_/384_/512_`, `shake128/256`) are defined in
`Hacspec_sha3.Sha3` (in the `hacspec_sha3` crate, `specs/sha3/`).

### Why the backends don't have identical theorem sets

The default top-level API (`libcrux_sha3::sha256` etc., and `hash<LEN>`) dispatches to **`portable`
on every platform** (`lib.rs`); `neon` and `avx2` are separate, feature-gated (`simd128`/`simd256`),
opt-in `pub mod`s that a caller invokes directly. Those two modules made *different* choices about
what to expose, so they prove different theorems:

- **Portable** — full single-buffer SHA-3 hasher (`sha224/256/384/512` + `shake128/256`); the backend
  behind the default API → six one-shot correctness theorems.
- **Neon** — also exposes single-buffer `sha224/256/384/512` + `shake128/256`, but implemented by
  running the **2-way** driver `keccak2(&[data,data], digest, &mut dummy)` and discarding lane 1
  → six one-shot correctness theorems (lane-0 specialisations).
- **AVX2** — `pub mod x4` only: a *4-way-parallel SHAKE engine* (used by ML-KEM / ML-DSA). Its sole
  one-shot entry point is `shake256_x4` (proven by `lemma_shake256_x4_avx2`); it has **no
  `sha224/256/384/512` functions at all** — so there is no `lemma_sha256_avx2` because there is no
  `avx2::sha256` to be about. (NEON provides single-buffer hashers by wasting 1 of 2 lanes; the AVX2
  module simply never added the analogous 4-way-waste-3 wrappers. That asymmetry is a *library API*
  choice, not a verification gap — the proofs cover exactly the functions that exist.)

**Known coverage gap (uniform across backends):** the *incremental* APIs — AVX2's
`shake{128,256}_absorb_final` / `*_squeeze_*`, and the corresponding Portable/Neon incremental
paths — are currently proven **panic-free with state-machine invariants only, not spec-equivalent**
to `Hacspec_sha3`. Closing that (an incremental-sponge ≡ spec refinement) is the genuine remaining
functional-correctness work; it is not specific to AVX2.

## Architecture (spec ← equivalence layers)

```
Hacspec_sha3.{Sha3, Sponge, Keccak_f}                 -- the spec (specs/sha3/, extracted)
        ▲
EquivImplSpec.Keccakf.{Generic,Portable,Arm64,Avx2}   -- keccak-f permutation ≡ spec, per lane
        ▲                                                 (Generic = backend-agnostic core;
                                                            ChiFold/SpecRounds = step helpers)
EquivImplSpec.Sponge.{Generic.*, Portable.*, Arm64.*, Avx2.*}
        ▲                                                 -- absorb / squeeze / block steps ≡ spec
                                                            (Steps, Driver, SqueezeDriver, SqueezeAPI)
EquivImplSpec.Correctness.{Portable, Neon, Avx2}      -- THE MAIN THEOREMS (this README's table)
```

Each correctness theorem reduces, via the per-backend `keccak1/2/4` driver lemma
(`= absorb · squeeze`), down through the sponge layer to the lane-wise keccak-f equivalence
`lemma_keccakf1600_{portable,arm64,avx2}` and ultimately to the spec.

Supporting modules: `Proof_Utils.{Lemmas,NatFold,FoldRange}` (fold/range/index helpers),
`fstar/stubs/Spec.Utils.fst[i]` (small stubs). The implementation-side proof obligations
(loop invariants, `ensures`) live inline in the extracted impl under `fstar/extraction/`.

## Building / re-verifying

```sh
# verify the equivalence proofs (and, transitively, their deps)
make -C crates/algorithms/sha3/proofs/fstar/equivalence
# verify a single module
make -C crates/algorithms/sha3/proofs/fstar/equivalence check/EquivImplSpec.Correctness.Portable.fst
```

Regenerate the status report after a build:
`python3 proofs/generate_verification_status.py --root . --config proofs/verification_status.config.json --output proofs/verification_status.md`

## Notes for future readers

- **Campaign history** (per-sprint agent prompts, status logs, milestones) has been moved out of the
  way into [`campaign-history/`](campaign-history/); it is historical and not needed to read or
  rebuild the proofs.
- **`Hacspec_sha3.Sponge.Lemmas.fst`** (in `fstar/equivalence/`) is a *hand-written, implementation-
  side* helper despite living in the `Hacspec_sha3` namespace — it is part of these proofs, not the
  spec crate.
- **The spec extraction** (`Hacspec_sha3.*` under `specs/sha3/proofs/fstar/extraction/`) is generated
  from `specs/sha3/src` and lives with the spec crate by design (it is regenerated by `hax.sh` and
  also feeds the Lean/aeneas backend); the equivalence build picks it up automatically.
- Internal module names still use the historical `EquivImplSpec.Sponge.*` / `.Keccakf.*` scheme;
  these are referenced by name in `fstar!` blocks in the Rust sources, so they are intentionally
  left unrenamed (only the top-level theorem files were renamed for discoverability).

# SHA-3 admit/assume inventory — 2026-05-24 (post Track B-2 follow-up)

Worktree: `/Users/karthik/libcrux-sha3-track-b-2`
Branch: `sha3-proofs-track-b-2`
Diff vs base (`c9f2e4f09`): 2 files modified (Sponge.Arm64.fst,
Sponge.Avx2.fst) — closes 5 byte_eq admits.

## Headline

**3 load-bearing items** remain in the SHA-3 verification surface
(down from 5 at `c9f2e4f09`, down from 8 at `d083da011`):

1. `Libcrux_sha3.Simd.Avx2.Store.fst:165` — body `admit ()` in
   `store_block` (source: `crates/algorithms/sha3/src/simd/avx2/store.rs:74`).
2. `EquivImplSpec.Sponge.Arm64.Driver.fst:111` — `assume val lemma_squeeze2_arm64`.
3. `EquivImplSpec.Sponge.Avx2.API.fst:87`     — `assume val lemma_squeeze4_avx2`.

All other SHA-3 modules verify with **no admits, no `assume val`s,
no `--admit_smt_queries true`** under `make check/EquivImplSpec.Sponge.{Arm64,Avx2}.API.fst`.

## Dependency chain

```
                  ┌──────────────────────────┐
                  │ Avx2.Store.fst:165       │
                  │ body admit in store_block│
                  └────────────┬─────────────┘
                               │ blocks (per-byte ensures unproven)
                               ▼
            ┌──────────────────────────────────────┐
            │ EquivImplSpec.Sponge.Avx2.fst        │
            │ avx2_sc_store_block (now verified —  │
            │ consumes store_block's ensures, but  │
            │ that ensures is unproved upstream)   │
            └────────────┬─────────────────────────┘
                         │ blocks
                         ▼
            ┌────────────────────────────────────┐
            │ Sponge.Avx2.API.fst:87             │
            │ assume val lemma_squeeze4_avx2     │
            └────────────────────────────────────┘

(Arm64 path: Simd.Arm64.fst's store_block ALREADY VERIFIED;
 the remaining assume on Sponge.Arm64.Driver.fst:111 is the
 driver-level squeeze2 lemma that needs writing — mechanical,
 mirrors the verified path.)
```

## Per-item detail

### 1. `Libcrux_sha3.Simd.Avx2.Store.fst:165` — `store_block` body admit

**Source**: `crates/algorithms/sha3/src/simd/avx2/store.rs:74` (hax-inserted `hax_lib::fstar!("admit()")` as scaffolding).
**Spec gap**: per-byte lane-wise `to_le_bytes` ensures on `out{0..3}_future`.
**Closure plan**: mirror the now-verified Arm64 `store_block` proof,
which uses the loop-invariant + per-byte equality pattern already
established. Estimated effort: medium (the Arm64 mirror exists; AVX2
needs 4-lane seed pattern from §7 fstar-for-libcrux SKILL.md
"Per-lane `get_lane_u64` seeds" template — same recipe that closed
the 5 byte_eq admits this session).

### 2. `EquivImplSpec.Sponge.Arm64.Driver.fst:111` — `assume val lemma_squeeze2_arm64`

**Statement**: per-lane driver-level squeeze2 at N=2.
`Simd128.squeeze2 rate s out0 out1 ≡ Hacspec_sha3.Sponge.squeeze outlen (extract_lane l s.f_st) rate` for each lane `l`.
**Closure plan**: documented in `BRIEF_squeeze_steps.md` — convert
`assume val` to `let` with one inline-ensures pass over the Steps
lemma (Option B from the squeeze2 post-mortem). Mechanical replace —
all infrastructure already in place since the Arm64 `store_block` proof landed.
Estimated effort: low (one session).

### 3. `EquivImplSpec.Sponge.Avx2.API.fst:87` — `assume val lemma_squeeze4_avx2`

**Statement**: N=4 mirror of (2) over `Simd256.squeeze4`.
**Closure plan**: identical recipe to (2), but **blocked on (1)** —
the per-lane Steps lemma consumes `avx2_sc_store_block`'s ensures,
which is verified at the Sponge layer but ultimately bottoms out in
`Avx2.Store.store_block`'s body admit.
Estimated effort: low (mirror of A2) once (1) closes.

## Comment-only mentions (NOT load-bearing)

These show up in grep but are inside `(* *)` comment blocks:

- `EquivImplSpec.Keccakf.Avx2.fst:117` — comment referencing the
  upstream `Core_models.Num.fst:493` `assume val` for
  `lemma_shl_xor_shr_is_rotate_left`. Out of SHA-3 scope.
- `Libcrux_sha3.Generic_keccak.Simd256.fst:283` — doc comment in a
  module header.

## Closure ordering recommendation

```
(1) Avx2.Store.store_block body admit
    └─→ (3) lemma_squeeze4_avx2     (unblocks once 1 closes)

(2) lemma_squeeze2_arm64            (independent; can start now)
```

(2) is sprint-low-hanging-fruit and should land first; (1) is the
remaining structural work and (3) falls out mechanically.

## What changed this session

- 2 byte_eq `admit ()`s closed (`Sponge.Arm64.fst:124`,
  `Sponge.Avx2.fst:125`).
- 3 inline-byte_eq cliff-rlimit-800 admits closed in
  `lemma_sq_lane_avx2_eq_squeeze_state` via 4-lane seed pattern
  (now passes at 588/800).
- Net SHA-3 unverified surface: **−5 from `c9f2e4f09`, −2 from
  `d083da011`** (the latter delta nets the 2 cliff admits added
  between the two snapshots).

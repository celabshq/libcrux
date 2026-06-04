# Plan: close the arm64 `squeeze2` proof via strategic opaque predicates

**Date:** 2026-05-30
**Strategy:** the same opaque-range-predicate / top-down recipe that closed AVX2
`store_block` (see `feedback_opaque_predicate_store_proof` memory + skill §7),
lifted **one level up** — from per-byte store layout to the per-lane
output-prefix ↔ `Hacspec_sha3.Sponge.squeeze` equality.

---

## 1. The exact admit

`src/generic_keccak/simd128.rs:120` (arm64 / NEON backend, `_uint64x2_t`, N=2):

```rust
#[hax_lib::fstar::options("--z3rlimit 300 --admit_smt_queries true")]   // ← THE admit
pub(crate) fn squeeze2<const RATE: usize>(
    mut s: KeccakState<2, _uint64x2_t>, out0: &mut [u8], out1: &mut [u8],
) { ... }
```

This is the **driver** `squeeze2` (the free fn, no `start`/`len`), not the per-block
trait method. Its `ensures` (lines 104–119) is the **full functional spec**, both lanes:

```
v outlen < MAX-200 ==>
  out0_future == Hacspec_sha3.Sponge.squeeze outlen (extract_lane (mk_usize 2) lc_arm64 s.st 0) RATE
  /\ out1_future == Hacspec_sha3.Sponge.squeeze outlen (extract_lane (mk_usize 2) lc_arm64 s.st 1) RATE
```

`--admit_smt_queries true` admits the entire body VC, so this functional spec is
**asserted but unproven**. It is the *only* admit in the squeeze2 chain.

### Why closing this one admit closes everything

`EquivImplSpec.Sponge.Arm64.Driver.fst:111` `lemma_squeeze2_arm64` is **already a real
`let`** (the comment at line 42 calling it `assume val` is stale) with an **empty
proof body**:
```fstar
= let _ = Libcrux_sha3.Generic_keccak.Simd128.squeeze2 rate s out0 out1 in ()
```
It merely brings the driver's `ensures` into context. So:
`squeeze2 admit` → `lemma_squeeze2_arm64` → `lemma_keccak2_arm64` → Neon hashers.
Discharge the one admit (or migrate its proof to the equivalence layer, see §3) and
the whole arm64 functional-squeeze chain is closed with **no further equivalence-layer
work**.

`lemma_squeeze4_avx2` (`EquivImplSpec.Sponge.Avx2.API.fst:87`, still `assume val`) is
the N=4 twin and is the **follow-on** target — same recipe, four lanes, four output
buffers. Do arm64/N=2 first; avx2/N=4 reuses the design verbatim.

---

## 2. Assets already in hand (do NOT re-prove)

| Asset | Location | Status |
|---|---|---|
| arm64 `store_block` per-byte post (`get_lane_u64(s[(j-start)/8], lane).to_le_bytes()[(j-start)%8]`) | `src/simd/arm64/store.rs:36` | **verified** |
| `lemma_squeeze_block_arm64` (one full-rate block ↔ `squeeze_state`) | `Arm64.Steps.fst:137` | **proven** |
| `lemma_squeeze_last_arm64` (partial trailing block) | `Arm64.Steps.fst:179` | **proven** |
| `lemma_squeeze_one_step_arm64` (keccakf ; block, per-byte prefix extension) | `Arm64.Steps.fst:243` | **proven** |
| `arm64_sc_store_block` / `_load_block` / `_load_last` (NEON↔scalar bridges) | `EquivImplSpec.Sponge.Arm64.fst:150/386/487` | **proven `let`s** |
| Portable `squeeze`/`squeeze_blocks`/`squeeze_last` (the structural template) | `src/generic_keccak/portable.rs:356/256/96` | **verified inline** |
| Portable fold-range→`squeeze_blocks` bridge (`lemma_fold_range_step`) | `EquivImplSpec.Sponge.Portable.SqueezeAPI.fst` | **proven** |
| Spec lemmas `lemma_squeeze_blocks_tail` / `_unfold` / `iterate_keccak_f` | `Hacspec_sha3.Sponge.Lemmas` | **proven (spec layer)** |

`lemma_squeeze_one_step_arm64`'s current ensures is *already* the exposed per-byte
prefix form: `outputs_pre[l][k] == squeeze..[k] for k<i*rate` ⟹ `outX'[k] == squeeze..[k]
for k<(i+1)*rate`, plus `s' == iterate_keccak_f i lane_st_init`. **This is the per-step
engine** — the opaque predicate just wraps its pre/post foralls.

---

## 3. THE architectural decision — resolve via spike FIRST (Phase 0)

The functional proof references `EquivImplSpec.Keccakf.Generic.extract_lane` +
`EquivImplSpec.Keccakf.Arm64.lc_arm64` (already a non-cyclic dep — the Rust `ensures`
*already* names them) but the per-block→spec bridge (`lemma_squeeze_block_arm64`) lives
in `EquivImplSpec.Sponge.Arm64.Steps`. Two hosting options; the spike decides which:

- **Path A — inline in the Rust driver body.** Replace `--admit_smt_queries true` with a
  real `loop_invariant!` + `fstar!` proof, citing the `Arm64.Steps` step lemmas
  directly. **Viable iff `Arm64.Steps` (and its dep `EquivImplSpec.Sponge.Arm64`) do
  NOT transitively depend on `Generic_keccak.Simd128`** (the driver's module). Evidence
  it *might* be viable: `Arm64.Steps` references the driver `squeeze2` only in a
  *comment* (line 224); its code cites `SA.arm64_sc_store_block`, which is about the
  per-block `store_block` in `Libcrux_sha3.Simd.Arm64`, not the driver. This mirrors how
  Portable proves `squeeze` inline. **Spike:** add a throwaway `let _ = EquivImplSpec.
  Sponge.Arm64.Steps.lemma_squeeze_one_step_arm64 in ...` reference (or a trivial
  `fstar!` cite) inside the driver and run `fstar_build` — if the dep graph accepts it
  (no cycle error), Path A is open.

- **Path B — equivalence-layer proof of `lemma_squeeze2_arm64`.** If A cycles: revert
  the Rust driver `squeeze2` to **length-only** ensures (drop the functional spec AND
  `--admit_smt_queries true`; it then verifies for real, trivially), and give
  `lemma_squeeze2_arm64` a **real proof body** that reasons about the *extracted* driver
  fold via `Proof_Utils.FoldRange.lemma_fold_range_step` (exactly as Portable
  `SqueezeAPI` does), using the step lemmas + opaque predicate. This is the
  cycle-safe path and the more likely one.

The **opaque-predicate design (§4) is identical** under either path — only the host
module changes.

---

## 4. The strategic opaque predicate (the core of the plan)

The Z3-killer is the **two-lane × all-output-bytes** per-byte `forall` dragged through
the block loop's invariant and step VC. Seal it exactly like `stored`/`modifies_range`
sealed the store cross-product.

### Predicate (equivalence-layer F*; or `hax_lib::fstar::before` if Path A)

```fstar
[@@ "opaque_to_smt"]
let squeezed_upto
      (lane_st_init: <spec state type>) (out: Seq.seq u8)
      (rate outlen: usize) (hi: nat) : prop =
  forall (k: nat). (k < hi /\ k < Seq.length out) ==>
    Seq.index out k
    == Seq.index (Hacspec_sha3.Sponge.squeeze outlen lane_st_init rate <: Seq.seq u8) k
```

`squeezed_upto … hi` = "output prefix `[0,hi)` already equals the squeeze spec." One
predicate per lane; the loop carries two instances (`out0`/lane0, `out1`/lane1).

### Confined-reveal lemmas (mirror the store_block frame-lemma kit)

- `lemma_squeezed_upto_zero` — `squeezed_upto _ _ _ _ 0` (base seed; trivial).
- `lemma_squeezed_upto_extend` — `squeezed_upto .. lo  /\  (forall k. lo<=k<hi ==> out[k]==spec[k])  ==>  squeezed_upto .. hi`. **The only place the bare forall is touched** — `reveal_opaque` confined here.
- `lemma_squeezed_upto_full` — `squeezed_upto .. outlen  /\  Seq.length out == outlen  ==>  out == Hacspec_sha3.Sponge.squeeze outlen lane_st_init rate` (Seq.equal extensionality; the single reveal at the end that discharges the driver's full-Seq ensures).

### Restate the step lemma to speak the predicate (keeps the forall confined)

Wrap `lemma_squeeze_one_step_arm64` (or add a thin `_op` variant) so its **pre/post are
in `squeezed_upto` terms**, not the raw forall:
```
requires:  squeezed_upto (extract_lane s_pre l) outX rate outlen (i*rate)
            /\ s_pre.f_st == iterate_keccak_f (i-1) lane_st_init  /\ window bounds
ensures:   squeezed_upto (extract_lane s_post l) outX' rate outlen ((i+1)*rate)
            /\ s_post.f_st == iterate_keccak_f i lane_st_init
```
Body = call the existing proven `lemma_squeeze_one_step_arm64` (gives the raw window
forall) then `lemma_squeezed_upto_extend`. The exposed cross-product never escapes this
lemma's body — every loop-step VC sees only opaque `squeezed_upto`.

---

## 5. Proof skeleton (top-down, mirrors Portable `squeeze`)

1. **Block loop** (`blocks` full rate-blocks). Invariant, **per lane**:
   `squeezed_upto (extract_lane s_init l) outX rate outlen (i*rate)` +
   `s.f_st == iterate_keccak_f i s_init` + `out0.len()/out1.len()` preserved.
   - Base (i=1 after `squeeze_first_block`): `squeezed_upto .. rate` from
     `lemma_squeeze_block_arm64` + `lemma_squeezed_upto_extend` off the `_zero` seed.
   - Step (i→i+1, after `keccakf1600` + per-block method): the restated step lemma ×2 lanes.
2. **Tail** (`last < outlen`, partial block): `lemma_squeeze_last_arm64` + a
   `squeezed_upto` extend to `outlen` (partial-window variant of the extend lemma).
3. **Close**: `lemma_squeezed_upto_full` ×2 lanes ⟹ the two full-Seq equalities =
   the driver `ensures`. Single reveal point.

Each loop-step VC manipulates only opaque `squeezed_upto` + `iterate_keccak_f` — the
per-byte work is confined to the window inside the step lemma. This is what prevents the
saturation that the `--admit_smt_queries true` was papering over.

---

## 6. Files

- `src/generic_keccak/simd128.rs` — driver `squeeze2`. Path A: replace the admit with
  `loop_invariant!` + `fstar!` proof. Path B: revert ensures to length-only, drop the
  admit.
- **Path B host** (likely): `EquivImplSpec.Sponge.Arm64.Driver.fst` (or a new
  `EquivImplSpec.Sponge.Arm64.SqueezeDriver.fst`) — define `squeezed_upto` + the 3
  frame lemmas + restated step, give `lemma_squeeze2_arm64` its real body. These are
  **manual** `.fst`s (not hax-extracted) — edit directly, note they're untracked.
- `proofs/fstar/extraction/Libcrux_sha3.Generic_keccak.Simd128.fst` — re-extracted from
  the Rust change.

---

## 7. Verification protocol (per skill Phase 8 + memory)

- Inner loop: `fstar_typecheck` / `--admit_except` per function (the file is large —
  use the curl + `kill -0` wait loop, full build exceeds the 60 s MCP cap).
- **`--admit_except` hides sibling + own-VC obligations** → always finish with a full
  `fstar_build check/<Module>.fst` with no `--admit_except`.
- **Profile before any negative result** (`smtprofiling` on the failing `.smt2`) —
  saturation (`used X.000`) vs missing-fact vs quantifier cascade need different fixes
  (memory `feedback_smtprofile_before_negative`).
- Behavioural gate: arm64 is native NEON — `cargo test` (sha3 arm64/neon paths) stays
  green. No `--target x86_64` needed (that's avx2 only).
- **Admit-count gate**: net `admit()` / `--admit_smt_queries true` / `[@@ admitted]` in
  touched files must end **≤ baseline − 1** (the simd128.rs:120 admit removed; nothing
  new introduced). Path B must NOT leave `lemma_squeeze2_arm64`'s body empty-reading a
  reverted ensures — it gets a real proof.
- **Per-function budget**: 30–60 min; past that, `fstar_note(level="cliff")` + stop.
- **Do NOT commit** (standing constraint this campaign).
- Max 4 concurrent fstar/z3 per agent (memory `feedback_max_4_fstar_per_agent`).

## 8. Known pitfalls (carried from store_block)

- Don't resurrect old per-attempt helpers (`lemma_lane_chain_*` etc.) on faith — every
  prior squeeze2 attempt that leaned on exposed foralls cliffed. The opacity is the point.
- Fully-qualify identifiers in `fstar!` strings (no `open`s in extracted modules).
- `rlimit` cap 800 / 400-with-`split_queries`; bumping above hides structure — if it
  saturates, restructure (more opacity / smaller window), don't bump.
- `lemma_squeezed_upto_extend`'s ensures forms `i*rate`, `(i+1)*rate` arithmetic — needs
  no-overflow bounds in `requires` (the `v outlen < MAX-200` slack covers it; thread it).

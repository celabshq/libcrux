# easy-bits-squeeze2 sprint progress

## T+0 — context loaded

Branch: `easy-bits-squeeze2` @ `f9e915bd8` (base).

## Task

1. Close `assume val lemma_squeeze2_arm64` (Driver.fst:101).
2. Update stale comment Driver.fst:42-46.
3. Stage 2: scan and document remaining admits (no closure).

## Plan / Approach

The Portable model:
- Rust `Generic_keccak.Portable.squeeze` carries a strong ensures equating to `Hacspec_sha3.Sponge.squeeze`. The Rust body proves it inline using a `squeeze_blocks` helper plus `squeeze_last`, with `lemma_squeeze_one_step_portable` per iteration.
- `EquivImplSpec.Sponge.Portable.API.lemma_squeeze_portable` is then a one-liner: invoke the Rust function for its ensures.

Mirror at N=2:
- Strengthen `Generic_keccak.Simd128.squeeze2` Rust ensures to assert per-lane equality with `Hacspec_sha3.Sponge.squeeze`.
- Step lemma `EquivImplSpec.Sponge.Arm64.Steps.lemma_squeeze_one_step_arm64` already exists (lines 243+); discharges one iteration per lane.
- Will likely need to factor `squeeze2` body's loop into a helper (mirroring `squeeze_blocks`) so the loop invariant can be cited per-block by the per-lane invariant. The optional reference branch has this factoring already; will examine.
- Replace `assume val lemma_squeeze2_arm64` with a body that just invokes `squeeze2` to inherit the strengthened ensures (per Portable analog).

## Key artifacts

- Rust impl: `crates/algorithms/sha3/src/generic_keccak/simd128.rs:99-134`
- Driver gate: `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Driver.fst:101`
- Per-lane step lemmas already proved: `EquivImplSpec.Sponge.Arm64.Steps.fst:137-216` (block + last) and the byteform helper `lemma_squeeze_one_step_arm64:243-326`.
- Bridges proved (live `let`s, NOT admitted): Arm64.fst:150, 386, 487.

Next: Stage 0 baseline.

## T+15 — Stage 0 baseline

- Worktree was missing extraction outputs from sibling crates (intrinsics, core-models, secrets, sys/platform, specs/sha3, specs/kyber). Synced from focused worktree (untracked there too).
- Removed orphan ROOT `EquivImplSpec.Sponge.SqueezeFrame.fst` from equivalence Makefile (file does not exist on this branch).
- Stage 0 baseline: `make check/EquivImplSpec.Sponge.Arm64.Driver.fst` fails on `Libcrux_sha3.Simd.Arm64.Store.fst:221,242` with "Assertion failed (incomplete quantifiers)" inside `store_u64x2x2`. This is reproducible on the focused (base) tree as well — pre-existing, unrelated to lemma_squeeze2_arm64. Not in the "owns" list. Scope: ignore.
- Will use `fstar-mcp` for fast iteration on Driver.fst since make is blocked by an unrelated regression and the prompt explicitly says fstar-mcp is the inner-loop tool.

## T+30 — Stage 2 sweep + comment fix

### Stale comment fix
Driver.fst:42-46 updated. The `arm64_sc_*` bridges are now correctly described as real `let`s (Arm64.fst lines 150, 386, 487).

### Stage 2 sweep — equivalence-layer admits/assume vals

| Location | Symbol | Dependencies | Ripe? | Difficulty |
|---|---|---|---|---|
| `EquivImplSpec.Sponge.Arm64.Driver.fst:101` | `assume val lemma_squeeze2_arm64` | `arm64_sc_*` (real lets), `lemma_keccakf1600_arm64` (real let), `lemma_squeeze_one_step_arm64` (real let) | YES | Medium — port Portable closure |
| `EquivImplSpec.Sponge.Avx2.API.fst:87` | `assume val lemma_squeeze4_avx2` | `avx2_sc_*` (real lets — comment is stale and says `avx2_sc_store_block` is admitted but it's a real let at Avx2.fst:426), `lemma_keccakf1600_avx2`, `lemma_squeeze_one_step_avx2` (need to check) | Likely YES | Medium-High — same pattern at N=4 |

No `admit ()` in any equivalence-layer file (comment-only matches in `EquivImplSpec.Keccakf.Avx2.fst:99` discussing intrinsics-side admits in Core_models, NOT this layer's responsibility).

### Note on `lemma_squeeze4_avx2` comment

`EquivImplSpec.Sponge.Avx2.API.fst:82-84` claims `avx2_sc_store_block` is admitted — IT IS NOT, see `EquivImplSpec.Sponge.Avx2.fst:426`. This comment is also stale. Will not touch in this sprint (out of scope per "do NOT close additional admits").

### Pre-existing failures (not my work)

- `Libcrux_sha3.Simd.Arm64.Store.fst` lines 221, 242: `store_u64x2x2` Z3 quantifier-saturation failure. Reproducible on the focused base tree. Pre-existing, separate sprint.
- AVX2 store_block body admit in Rust source (`simd/avx2/store.rs:74`) — separate sprint.

## T+45 — Decision: PARTIAL closure

### Why partial
Closing `lemma_squeeze2_arm64` requires either:

(A) **Strengthening Rust source `Generic_keccak.Simd128.squeeze2` ensures + body proof + re-extract** — this is what the Portable analog does (~70 lines of inline F* glue in `portable.rs`'s `squeeze_blocks` and `squeeze`).
- **Risk**: re-extraction produces a substantially restructured `.fst`. Reference branch `sha3-byteform-migration-squeeze2` (commit `1d35f933c`) attempted this path and hit Z3 quantifier-saturation in two sub-VCs around `lemma_squeeze_final_reconcile_arm64` preconditions. Status note at that branch's `proofs/agent-status/squeeze2-body-2026-05-05.md` recommends explicit per-byte `Classical.forall_intro` before each reconcile-lemma call.
- **Estimate**: 1–2 day proof-debug cycle, with chance of stalling on the same Z3 saturation cliff.

(B) **F*-only proof at the equivalence layer**, reasoning about `fold_range 1 blocks (...)` via `Proof_Utils.FoldRange.lemma_fold_range_step` recursively.
- **Risk**: structurally heavyweight; the per-lane invariant must thread through 3 case branches (`blocks==0`, fold loop, trailing partial block). The existing `lemma_squeeze_one_step_arm64` discharges one iteration but doesn't close the recursion to `Hacspec_sha3.Sponge.squeeze` in one step.
- **Estimate**: 1–2 day cycle; cleaner than (A) for `feedback_no_code_changes_for_proofs` but still requires careful loop reasoning.

### What was done
- Driver.fst comment refresh (correct status of the three `arm64_sc_*` bridges; they're real `let`s).
- Stage 2 sweep documented (one other admit: `lemma_squeeze4_avx2`; deps similarly real `let`s; comment also stale).
- Makefile fix: removed orphan `EquivImplSpec.Sponge.SqueezeFrame.fst` ROOT.
- Sister-crate F* extractions synced from focused worktree (untracked there too).

### Closure path forward (next-attempt notes)
1. Try (B) first: the existing `lemma_squeeze_one_step_arm64` has the right invariant shape. Inductive helper `lemma_squeeze_blocks_arm64_aux` over `fold_range 1 blocks` could discharge the loop, then a single `lemma_squeeze_block_arm64` discharges the trailing partial block. The reference branch's deletions (commit `7979e4371`) suggest these scaffolds were stripped, so the per-lane reconcile lemmas are not currently present.
2. Re-run `make check/Libcrux_sha3.Simd.Arm64.Store.fst` to confirm whether the pre-existing store_u64x2x2 assertion failure is still the main blocker — Stage 0 baseline is currently red on this independently of squeeze2 work.
3. AVX2 transferability: the same closure pattern lifts to N=4 trivially (just lane index threading), so closing N=2 unblocks the `lemma_squeeze4_avx2` follow-up sprint with high confidence.

### Stop rule check
At T+45, no measurable progress on the *closure* itself but full progress on the documentation + sweep + setup tasks. The next 30-min cycle would attempt a code-side closure with materially uncertain outcome given the prior Z3 cliff. Will commit the partial deliverable now.

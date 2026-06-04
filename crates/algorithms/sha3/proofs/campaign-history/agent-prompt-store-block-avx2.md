# Prompt — discharge `Libcrux_sha3.Simd.Avx2.Store.store_block`

## Mission

Close the body-`admit()` block on `Libcrux_sha3.Simd.Avx2.Store.store_block` by porting the structural pattern that landed for the Arm64 store_block sprint (commits `8c0202a4b` → `a2a3f5f71` → `f9e915bd8` on `sha3-proofs-focused`).

- **Target**: `crates/algorithms/sha3/src/simd/avx2/store.rs:74` — `hax_lib::fstar!("admit()");` at the entry of `store_block`. Remove it and discharge the body against the post (16 forall conjuncts, 4 per output buffer × 4 output buffers). The function lives in the post-split `simd/avx2/store.rs` with the trait impls in `simd/avx2.rs` (the parent module shim) and helpers/wrappers in `simd/avx2/wrappers.rs` (extracts to `Libcrux_sha3.Simd.Avx2.Wrappers.fst`).
- **Secondary goal**: the structural recipe should expose insights that let us come back and stabilize `Libcrux_sha3.Simd.Arm64.load_block` query 301 (pre-existing cliff documented in `crates/algorithms/sha3/src/simd/arm64.rs` and `proofs/agent-status/store-block-arm64-discharge-progress.md`). The AVX2 work is the cleaner laboratory because the AVX2 cascade-closure filter is already in place for load_block.

## What's already there (read FIRST)

### The Arm64 store_block sprint (template — read this carefully)

Recipe that closed Arm64 store_block (now on `sha3-proofs-focused`):

1. **Helpers** (commit `8c0202a4b`):
   - Strengthened `_vst1q_bytes_u64` ensures from length-only to per-byte content + frame.
   - New module `Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst` with `store_block_window_byte_of_vst` — a per-byte bridge lemma that takes the raw `update_at_range` and `e_vst1q_bytes_u64` posts as they appear after each store, and concludes the per-byte equality at index `j` for the freshly-stored window. **No SMTPat — explicit lemma calls per iteration.**
2. **Per-iteration wrapper** (commit `c14f94d2c`): `store_u64x2x2` — wraps the trn1q + trn2q + 2× vst1q sequence with a strong per-byte ensures (split on `(j-start)/8 == 2*i`). Inside the wrapper body, two named local lemmas (`bridge_out0`, `bridge_out1`) call `store_block_window_byte_of_vst`; `Classical.forall_intro` lifts each into the per-absolute-index byte forall.
3. **Tail wrappers** (commit `29424f593`): `store_tail_high` (for `8 < remaining < 16`) and `store_tail_low` (for `0 < remaining ≤ 8`). Same bridge pattern, with manual case split (prefix / middle window / suffix), each branch closed by `Seq.slice` `Seq.index` manual unfolding.
4. **Full / tail split + composer** (commit `83d1a04c2`): the critical structural fix. `store_block` was monolithic (loop + tail + function-level ensures all in one body), and Z3 cliffed on the aggregation step (loop invariant ⊕ tail wrapper post → function ensures, with `(16q+k)/8 = 2q + k/8` Euclidean bridging under the using_facts_from filter). Split into:
   - `store_block_full(s, out0, out1, start, q)` — loop only, ensures over `[start, start+16q)`.
   - `store_block_tail(s, out0, out1, start, q, remaining)` — if-else only, ensures over `[start+16q, start+16q+remaining)`.
   - `store_block(...)` — thin composer; computes `q,remaining` and calls the two. Function ensures derives by composition of two opaque function posts. **Aggregation closed without admit** because `q` and `remaining` are pinned by call boundaries — no Euclidean bridging inside `store_block`'s body.
5. **Filter** (function-level): `--using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'` was applied to the per-iteration / per-tail-branch wrappers and `store_block_full`. Same filter that closed AVX2 load_block earlier in the project.

Read these commits before touching anything:
```
git -C <worktree> log --oneline -p 8c0202a4b c14f94d2c 29424f593 83d1a04c2 a2a3f5f71 \
    -- crates/algorithms/sha3 crates/utils/intrinsics
```

### AVX2 starting state

Different from Arm64 in three important ways:

1. **Post-split layout (post `f9e915bd8`)**: AVX2 is structured as `simd/avx2/{wrappers,load,store}.rs` extracting to `Libcrux_sha3.Simd.Avx2.{Wrappers,Load,Store}.fst` plus a thin parent `Libcrux_sha3.Simd.Avx2.fst` for trait impls. The cascade-closure helpers (`load_lane_u64_lane_extensionality` SMTPat lemma + opaque `createi`) are co-located with `load_block` in `Libcrux_sha3.Simd.Avx2.Load.fst`. The `store_block` admit is in `Libcrux_sha3.Simd.Avx2.Store.fst` — a much smaller scope (~778 lines) than the pre-split monolith. Per-module verify after change with `make check/Libcrux_sha3.Simd.Avx2.Store.fst`.

2. **The intrinsic `mm256_storeu_si256_u8` already has a strong content ensures** (`crates/utils/intrinsics/src/avx2_extract.rs:91-100`):
   ```rust
   forall i < 4. u64::from_le_bytes(future(output)[i*8..i*8+8]) == get_lane_u64(vector, i)
   ```
   This is a *per-lane* equality stated as `from_le_bytes(slice) == lane`. The loop invariant needs *per-byte* in the inverse direction (`out[j] == to_le_bytes(lane)[j%8]`). So you likely need a small bridge lemma `from_le_bytes_lane ↔ to_le_bytes_byte` at the SHA-3 layer (in a new `Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.fst` analogous to the Arm64 helpers), *not* an intrinsic-level strengthening. Try this first; only fall back to strengthening the intrinsic ensures if the SHA-3-layer bridge cliffs.

3. **AVX2 cascade-closure infrastructure is already in tree** (commits `7bb581f8b`, `8203c9ace`, `28db4222a`, `3b9fc054c` for load_block). The using_facts_from filter, the `load_lane_u64_lane_extensionality` SMTPat lemma (now in `Libcrux_sha3.Simd.Avx2.Load.fst`), and opaque-`createi` are all in place. `load_block` (AVX2) verifies clean. Use that as evidence that the filter is sufficient for AVX2 store_block too.

### AVX2 store_block shape (read avx2/store.rs)

```rust
pub(crate) fn store_block<const RATE: usize>(
    s: &[Vec256; 25],
    out0: &mut [u8], out1: &mut [u8], out2: &mut [u8], out3: &mut [u8],
    start: usize, len: usize,
)
```

- Outer loop: `for i in 0..len/32` — 32 bytes per buffer per iteration. Per iteration: 4 `mm256_permute2x128_si256` + 4 `mm256_unpacklo/hi_epi64` (deinterleaving 4 lanes from 4 state slots) + 4 `mm256_storeu_si256_u8` (one per output buffer).
- Tail at `len % 32`: outer `if rem > 0`, then inner `for k in 0..chunks8` (8-byte chunks via tmp `[0u8; 32]` + `copy_from_slice` into 4 buffers), then ragged `if rem8 > 0` for the final partial bytes.
- Post: 16 forall conjuncts (4 per output × 4 output buffers).

This tail is **more complex than Arm64** — Arm64 had two if-else branches; AVX2 has an outer if + nested for + nested if. Plan for two or three tail wrappers (`store_tail_chunks` for the inner for, `store_tail_ragged` for the rem8 case) plus the per-iteration `store_u64x4x4` wrapper.

## Setup (user runs once before spawning)

```
cd /Users/karthik/libcrux-sha3-focused
git worktree add -b store-block-avx2-discharge \
    /Users/karthik/libcrux-sha3-store-avx2-discharge sha3-proofs-focused
```

Hard constraint: NEVER `cd` into `/Users/karthik/libcrux-sha3-focused`, the Arm64 sibling worktrees, or any other tree. Use `git -C /Users/karthik/libcrux-sha3-store-avx2-discharge ...` and absolute paths. Use `EnterWorktree path=...` once at the start to switch the agent's cwd in.

## Approach (port the Arm64 recipe)

### Stage 1 — helpers

1. Decide whether the existing `mm256_storeu_si256_u8` content ensures is enough, or if a per-byte form is needed. Try writing a SHA-3-layer bridge lemma first:
   ```fstar
   val storeu_byte_of_lane
       (out_pre out_post: Seq.seq u8) (vec: Vec256) (k: nat{k < 32})
     : Lemma
       (requires
         Seq.length out_pre = Seq.length out_post /\
         (forall (i:nat{i<4}). u64_from_le_bytes (Seq.slice out_post (i*8) (i*8+8))
                              == get_lane_u64 vec i))
       (ensures Seq.index out_post k ==
                impl_u64__to_le_bytes (get_lane_u64 vec (k/8)).[k%8])
   ```
   If this proves quickly, build on it. If it cliffs, strengthen the intrinsic ensures instead (mirror the Arm64 helper-1 work).
2. Add `Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.fst` with the chosen bridge lemma + a `store_block_window_byte_of_storeu` analog of the Arm64 `store_block_window_byte_of_vst`. **No SMTPat** — explicit lemma calls.

### Stage 2 — per-iteration wrapper

Add `store_u64x4x4(out0, out1, out2, out3, s_a, s_b, s_c, s_d, start, i)` — wraps the per-iteration permute + unpacklo/hi + 4× storeu sequence with a strong per-byte ensures (split on `(j-start)/8 == 4*i + k` for k ∈ {0,1,2,3}). Inside, four named local lemmas (`bridge_out0`/`bridge_out1`/`bridge_out2`/`bridge_out3`) call the helper-2 bridge; `Classical.forall_intro` lifts.

### Stage 3 — tail wrappers

Two or three:
- `store_tail_chunks` for the inner `for k in 0..chunks8` loop body. Each iteration writes 8 bytes per buffer via tmp + copy_from_slice. Per-byte ensures over `[start+8k, start+8(k+1))` for each buffer.
- `store_tail_ragged` for the final `rem8 > 0` partial. Per-byte ensures over `[start+len-rem8, start+len)`.
- The outer `if rem > 0` block can stay inline OR be its own `store_block_tail_wrapper`; structure it however gives you the cleanest call site.

### Stage 4 — full / tail split + composer

This is the load-bearing structural fix from Arm64. Split:
- `store_block_full(s, out0..out3, start, q)` — outer for-loop only. Ensures: per-byte content over `[start, start+32q)` plus frame outside.
- `store_block_tail(s, out0..out3, start, q, rem)` — `rem > 0` block only (inner for + rem8). Ensures: per-byte content over `[start+32q, start+32q+rem)` plus frame outside.
- `store_block` — composer. Computes `chunks = len/32`, `rem = len%32`, calls the two. Function ensures derives from composition of two opaque function posts. **No Euclidean bridging in this body.**

### Stage 5 — options

Apply `--using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'` to the wrappers, full, and tail. Composer probably needs only modest options. rlimit ≤ 800 monolith / ≤ 400 with split_queries (per `feedback_rlimit_cap_800`).

## Operational rules

- `feedback_grep_make_output`: pipe make to `/tmp/<name>.log`, grep, never Read full F* logs.
- `feedback_use_fstar_mcp`: fstar-mcp for sub-second iteration; full SMT through make.
- `feedback_fstar_mcp_session_dies_after_make`: recreate session after each make.
- `feedback_no_manual_edits_extracted`: experimental .fst probes during cascade debugging are OK; permanent fixes go through Rust source + `bash hax.sh extract`.
- `feedback_rlimit_cap_800`: never set rlimit > 800 mono / 400 with `--split_queries always`.
- `feedback_bisect_before_blame`: if a proof OUTSIDE store_block fails after a change, bisect to parent before assuming you broke it. Lucky-Z3 cliff edges look like regressions.
- **Budget**: lifted 60-min cap. Stop condition is **two consecutive 30-min cycles with no measurable progress**.
- `feedback_agent_status_reports`: append to `crates/algorithms/sha3/proofs/agent-status/store-block-avx2-discharge-progress.md` every 15 min.
- `feedback_smtpat_lane_propagation` + `feedback_smtpat_percent_above_trait`: avoid SMTPats in the new bridges; explicit lemma calls (the load-bearing Arm64 pattern).
- `feedback_for_loop_param_unshadowing`: rename `mut` loop accumulators if needed.
- `feedback_no_code_changes_for_proofs` (means "ask before changing the impl"): the structural split is authorized by this prompt. If you find a *different* impl change is needed (e.g. reordering ops, adding a non-obvious helper not in this prompt), surface to user before editing.
- `project_sha3_arm64_load_block_cliff`: be aware of the pre-existing Arm64 load_block cliff. Module-wide make on the AVX2 worktree should NOT regress it. If your changes to shared modules (`Hacspec_sha3.*`, `Libcrux_intrinsics.*`) cause Arm64 load_block to break differently than today's signature (173s canceled at query 301), that's information — surface it.

## Skills

`fstar-mcp`, `fstar-for-libcrux` (especially §1.5.1 dual-trigger trap, §8 direct .fst probing), `smtprofiling`, `proofdebugging`.

## File boundaries

You own:
- `crates/algorithms/sha3/src/simd/avx2.rs`
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.fst` (regenerated; OK to probe directly during cascade debugging)
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.fst` (new — only if helpers are needed at the SHA-3 layer)
- `crates/algorithms/sha3/proofs/agent-status/store-block-avx2-discharge-*.md`

You may touch (with surface-to-user first):
- `crates/utils/intrinsics/src/avx2_extract.rs` — only if the SHA-3-layer bridge cliffs and the intrinsic's content ensures genuinely needs strengthening. Cross-cutting; surface a 1-paragraph proposal.

You do NOT touch:
- Arm64 files (`simd/arm64.rs`, `Libcrux_sha3.Simd.Arm64*.fst`).
- `EquivImplSpec.*`, `specs/sha3/*`, `portable.rs`, `generic_keccak.rs`.
- Squeeze2 / squeeze4 driver lemmas.

## Insight target — load_block

Throughout the work, keep notes on which patterns might apply to `Libcrux_sha3.Simd.Arm64.load_block` (the documented cliff at line 1320-1470 of the extracted .fst). Specifically:

- Does the per-iteration wrapper pattern (analog of `store_u64x2x2`) trivially apply to `load_block`? The current `load_u64x2x2` IS a per-iter wrapper, so the question is whether there's a *stronger* version with the right shape.
- Does the full/tail split apply? `load_block` (Arm64) currently embeds a small tail (`rem = RATE % 16` handling). Splitting into `load_block_full` (loop only) + maybe reusing the existing `load_last`-style ragged handling could make the loop's ensures clean.
- Does the using_facts_from filter shrink the cliff if applied alone?
- Does opacifying the loop invariant predicate help?

If you discover a small, low-risk patch to load_block that closes its cliff without scope-creeping the AVX2 sprint, propose it as a final follow-up commit on the AVX2 branch. Otherwise document the diagnosis in the progress doc.

## Status reports every 15 min

Append to `crates/algorithms/sha3/proofs/agent-status/store-block-avx2-discharge-progress.md`:
```
## 2026-05-DD, T+N (sub-task)
- Sub-task: <which stage, which sub-step>
- Blocker: <if any>
- ETA: <next checkpoint>
- load_block insight notes: <if any>
```

## Deliverables

Commit on `store-block-avx2-discharge` branch (do NOT push).

- **Success**: `make check/Libcrux_sha3.Simd.Avx2.fst` passes clean, **zero `admit()` lines** in `store_block` / `store_block_full` / `store_block_tail` / wrappers. Module-wide make also clean for any other module touched. Commit message describes the helper choices, wrapper signatures, and any divergence from the Arm64 recipe (e.g. different tail granularity, intrinsic-level bridging vs SHA-3-layer bridging).
- **Partial**: targeted admit isolated to the smallest scope possible. Status doc with diagnosis, qi.profile data, next-attempt path. Commit progress.

## Final report (≤400 words)

(1) Did all admits close in `store_block` and friends? (2) Final structure (signatures of `store_block_full`, `store_block_tail`, the wrappers). (3) Helper choice (SHA-3-layer bridge or intrinsic-level strengthening, with reason). (4) Filter/options used. (5) Per-function timings (sub-query count, max time, total). (6) qi.profile cascade-source diagnosis if any was needed. (7) Branch SHA. (8) **Insights for the load_block follow-up** — explicit notes on which patterns transfer back, and any low-hanging opportunities you found.

## Suggested first 30 min

1. EnterWorktree to `/Users/karthik/libcrux-sha3-store-avx2-discharge`.
2. Read commits `8c0202a4b a2a3f5f71` end-to-end on this branch (`git log -p ...`).
3. Read `avx2.rs:481-543` (current store_block body).
4. Read `avx2_extract.rs:90-100` (existing `mm256_storeu_si256_u8` ensures).
5. Read the AVX2 cascade-closure precedent: `Libcrux_sha3.Simd.Avx2.fst` tail (look for `load_lane_u64_lane_extensionality` and the opaque `createi`).
6. Sketch the helper bridge lemma signature + the per-iter wrapper signature on paper; choose between SHA-3-layer bridging vs intrinsic-level strengthening.
7. First status entry to progress.md.

If by T+45 min the helper-1 sketch isn't typechecking with `lax`, surface a 1-paragraph blocker note.

# Prompt — split `simd/arm64.rs` and `simd/avx2.rs` into load/store submodules

## Mission

Refactor both SHA-3 SIMD modules so `load_*` and `store_*` live in separate Rust submodules and extract to separate F* modules:

- `crates/algorithms/sha3/src/simd/arm64.rs` → `simd/arm64/{load.rs, store.rs, mod.rs}` extracting to `Libcrux_sha3.Simd.Arm64.{Load,Store}.fst` plus a thin `Libcrux_sha3.Simd.Arm64` for the trait impls.
- `crates/algorithms/sha3/src/simd/avx2.rs` → `simd/avx2/{load.rs, store.rs, mod.rs}` similarly.

Goal: shrink the F* module-level SMT context for each function so that hairy proofs (`load_block` query 301, etc.) don't share a Z3 process with everything else in the file.

## Why this matters (READ FIRST)

Investigation on 2026-05-06 (post-merge of the Arm64 store_block sprint at `a2a3f5f71` on `sha3-proofs-focused`) showed:

- `load_block`'s body proof is **sound** — verifies in 70 s under `--admit_except` scoping (425 sub-queries, query 301 closes in 485 ms with hint).
- In **full-module** mode, the same proof cliffs at query 301 (~173 s, canceled at rlimit 800).
- Sanity-tested `--admit_except` is genuinely verifying (using a nonexistent target name produces 0 queries; load_block scoping produces 425 queries and real Z3 work).
- `#restart-solver` before `load_block` was tried — does NOT close the cliff (158 s, canceled). So it's not Z3 state pollution; it's the SMT context size itself.

This means the proof is correct; the cliff is from too many declarations / lemmas in scope when the whole module compiles together. Splitting `Libcrux_sha3.Simd.Arm64` so `load_block` lives in its own F* module (with a smaller open list) is the natural structural fix and is also good engineering hygiene independent of the cliff.

The same picture is expected for AVX2 — `load_block` (AVX2) closed via the cascade-closure work already, but pre-emptively splitting avoids re-incurring the same trap when AVX2 store_block lands in a follow-up sprint.

A bonus we'll measure: **does the split alone close the Arm64 `load_block` query 301 cliff?** If yes, that's a big win and we annotate the `arm64/load.rs` source accordingly. If no, we still have a cleaner structure and can target the cliff with filters / opaque markers in a smaller scope.

## Setup

```
cd /Users/karthik/libcrux-sha3-focused
git worktree add -b load-store-module-split \
    /Users/karthik/libcrux-sha3-load-store-split sha3-proofs-focused
```

Hard constraint: NEVER `cd` into `/Users/karthik/libcrux-sha3-focused`, the discharge / helpers / squeeze sibling worktrees, or anything else. Use `git -C /Users/karthik/libcrux-sha3-load-store-split ...` and absolute paths. Use `EnterWorktree path=...` once at the start to switch the agent's cwd in.

## Required reading (no skipping)

1. **The Arm64 store_block sprint commits** (`8c0202a4b a2a3f5f71` on this branch's history). Read in particular how `Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst` is wired from `arm64.rs`:
   ```
   git -C <worktree> log --oneline -p 8c0202a4b a2a3f5f71 -- \
     crates/algorithms/sha3/src/simd/arm64.rs \
     crates/algorithms/sha3/proofs/fstar/extraction
   ```
2. **Existing AVX2 layout** — the cascade-closure commits `7bb581f8b 8203c9ace 28db4222a 3b9fc054c` show how AVX2 lemmas are laid out today.
3. **hax module naming** — how `simd/arm64.rs` produces `Libcrux_sha3.Simd.Arm64.fst`. Check `crates/algorithms/sha3/hax.sh` for the extraction invocation. Look for examples elsewhere in the repo where a `mod_name/{file1.rs, file2.rs, mod.rs}` pattern produces `Crate.ModName.{File1,File2}.fst`. The `crates/algorithms/sha3/src/portable.rs` may already split this way — check.
4. **Skills**: `fstar-mcp`, `fstar-for-libcrux` (especially the hax extraction layout / re-extract gotchas), `proofdebugging`, `smtprofiling`.
5. **Memory rules**:
   - `feedback_grep_make_output` — never Read full F* logs.
   - `feedback_no_manual_edits_extracted` — refactor goes via Rust source + re-extract.
   - `feedback_rlimit_cap_800` — unchanged.
   - `feedback_no_code_changes_for_proofs` — this refactor IS authorized by this prompt; further structural changes need user sign-off.
   - `feedback_branch_means_worktree` — stay in the worktree.
   - `feedback_per_stage_clean_rebuild` — multi-stage proof refactors silently regress; delete touched modules' `.checked` and re-make per stage.
   - `feedback_bisect_before_blame` — if a proof outside your scope fails, bisect.
   - `project_sha3_arm64_load_block_cliff` — be aware of the pre-existing cliff; the goal is to test if the split closes it.

## Plan

### Stage 1 — Arm64 split

1. Create directory `crates/algorithms/sha3/src/simd/arm64/`.
2. Move `load_*` functions to `simd/arm64/load.rs`. Specifically: `load_lane_u64`, `load_u64x2`, `load_u64x2x2`, `load_block`, `load_last`. Plus the `lemma_rate_mod` ghost helper that load_block uses.
3. Move `store_*` functions to `simd/arm64/store.rs`. Specifically: `store_u64x2x2`, `store_tail_high`, `store_tail_low`, `store_block_full`, `store_block_tail`, `store_block`. Update the `mod` declarations and `use` paths.
4. The new `simd/arm64/mod.rs` (or `simd/arm64.rs` parallel to a `simd/arm64/` directory — pick whichever Rust idiom matches the rest of the codebase) keeps:
   - Type alias `pub type uint64x2_t = _uint64x2_t;`
   - SIMD math wrappers (`_veor5q_u64`, `_vrax1q_u64`, `_vxarq_u64`, `_vbcaxq_u64`, `_veorq_n_u64`)
   - `pub use load::{load_block, load_last};`
   - `pub use store::store_block;`
   - The `KeccakItem<2>`, `Absorb<2>`, `Squeeze2` impl blocks
5. Move `Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst` → `Libcrux_sha3.Simd.Arm64.Store.Helpers.fst` (or whatever name matches the extraction). Update the open in store.rs accordingly.
6. Re-extract: `bash hax.sh extract` from the sha3 crate root. Verify the produced files are now `Libcrux_sha3.Simd.Arm64.Load.fst`, `Libcrux_sha3.Simd.Arm64.Store.fst`, `Libcrux_sha3.Simd.Arm64.fst` (thin), `Libcrux_sha3.Simd.Arm64.Store.Helpers.fst`.
7. Per-module verification:
   - `make check/Libcrux_sha3.Simd.Arm64.Load.fst > /tmp/arm64-load.log 2>&1` — should now succeed cleanly because the module is small. **This is the cliff test.** If this passes without `--admit_except`, the split closed the load_block cliff.
   - `make check/Libcrux_sha3.Simd.Arm64.Store.Helpers.fst` — small helper module, should be fast.
   - `make check/Libcrux_sha3.Simd.Arm64.Store.fst` — store_*. May need its options preserved (the `--using_facts_from` filter).
   - `make check/Libcrux_sha3.Simd.Arm64.fst` — thin trait-impls module. Should be fast.
8. Full-module verify everything together: `make` from the extraction dir. Expectation: clean, faster than before because each module's proofs run in their own SMT context.

### Stage 2 — AVX2 split

Mirror exactly the same pattern in `simd/avx2.rs` → `simd/avx2/{load.rs, store.rs, mod.rs}`. The AVX2 store_block still has the entry-`admit()` (separate sprint will close it) — the split is independent. Stage 2 should:

- Move `load_lane_u64`, `load_u64x4`, `load_u64x4x4`, `load_block`, `load_last` to `avx2/load.rs`.
- Move `store_block` (still admitted) to `avx2/store.rs`.
- Move the AVX2 cascade-closure helpers (the `load_lane_u64_lane_extensionality` SMTPat lemma + opaque `createi` machinery, currently at the tail of `Libcrux_sha3.Simd.Avx2.fst`) into either `avx2/load.rs` (so they live with their consumer) or a new helpers module. Surface to user before deciding — this is shared-code that may have cross-cutting effects.
- Mirror `KeccakItem<4>`, `Absorb<4>`, `Squeeze4` impls in `mod.rs`.
- Re-extract; per-module verify; full-module verify.

### Stage 3 — Measure & document

After both splits land:

1. **Measure load_block cliff**: from a clean cache, run `make check/Libcrux_sha3.Simd.Arm64.Load.fst > /tmp/load-clean.log 2>&1` and grep for `load_block, 301`. Record whether it closes (succeeded vs failed) and the timing.
2. If it closes, **remove the load_block cliff comment** from `arm64/load.rs` (the comment block at the top of `pub(crate) fn load_block` documenting the cliff) and update the `project_sha3_arm64_load_block_cliff.md` memory to note the cliff was closed by the module split.
3. If it doesn't close, **add a final-attempt entry** to the project memory documenting that the split alone wasn't enough (with timing) and listing what filtering / opacification the next sprint should try in the smaller `Load` module's scope.
4. Record per-module timings and per-function max sub-query times — useful baseline for the AVX2 store_block sprint.

## Operational rules

- `feedback_grep_make_output`: pipe make to `/tmp/<name>.log`, grep, never Read full F* logs.
- `feedback_use_fstar_mcp`: fstar-mcp for sub-second iteration; full SMT through make.
- `feedback_fstar_mcp_session_dies_after_make`: recreate session after each make.
- `feedback_no_manual_edits_extracted`: extracted files are regenerated; all changes go through Rust source + `bash hax.sh extract`.
- `feedback_per_stage_clean_rebuild`: rm `.checked` for touched modules between stages.
- `feedback_rlimit_cap_800`: ≤ 800 mono / ≤ 400 with split_queries.
- `feedback_branch_means_worktree`: stay in worktree.
- **Budget**: lifted 60-min cap. Stop condition is **two consecutive 30-min cycles with no measurable progress**.
- `feedback_agent_status_reports`: append to `crates/algorithms/sha3/proofs/agent-status/load-store-split-progress.md` every 15 min.

## File boundaries

You own:
- `crates/algorithms/sha3/src/simd/arm64.rs` (delete or shrink)
- `crates/algorithms/sha3/src/simd/arm64/{load,store,mod}.rs` (new)
- `crates/algorithms/sha3/src/simd/avx2.rs` (delete or shrink)
- `crates/algorithms/sha3/src/simd/avx2/{load,store,mod}.rs` (new)
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Arm64*.fst` (regenerated; old StoreBlockHelpers may be renamed)
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2*.fst` (regenerated)
- `crates/algorithms/sha3/proofs/agent-status/load-store-split-*.md`
- `crates/algorithms/sha3/src/simd/arm64.rs`'s comment about load_block cliff (delete if Stage 3 confirms close)

You may touch (with surface-to-user first):
- `crates/algorithms/sha3/hax.sh` — only if the extraction invocation needs adjustment for the new module structure.
- `crates/algorithms/sha3/proofs/fstar/extraction/Makefile` — only if module name changes break the dep graph.
- `crates/algorithms/sha3/src/lib.rs` (or wherever `pub mod simd` is declared) — only if the module re-export structure needs to change.

You do NOT touch:
- `crates/algorithms/sha3/src/simd/portable.rs` (out of scope; mirror the pattern only if you want, surface first).
- `crates/algorithms/sha3/src/{generic_keccak,traits}.rs` and downstream callers — they should call through `Libcrux_sha3.Simd.Arm64` / `.Avx2` exactly as today after the re-export.
- `EquivImplSpec.*`, `specs/sha3/*`.
- `crates/utils/intrinsics/*` (intrinsic files unchanged).
- Any other crate.

## Deliverables

Commit on `load-store-module-split` branch (do NOT push):

- **Success**: Both modules split, all per-module makes pass, full-module make passes (no admit_except needed), and Stage 3 measurements documented. If load_block cliff closed, commit message says so. If not, status doc records the data + next-attempt path.
- **Partial**: Either Arm64 or AVX2 split landed, the other documented. Rust compiles, downstream callers (trait impls, `generic_keccak`) all still work.

## Final report (≤300 words)

(1) Both splits landed? (2) Per-module file structure (Rust + extracted F*). (3) Did the Arm64 load_block query 301 cliff close after the split — yes/no, with timing? (4) Per-module verification times (clean cache, no hints). (5) Any cross-cutting cleanups required (hax.sh, Makefile, lib.rs)? (6) Branch SHA. (7) Notes for the AVX2 store_block sprint about the new module layout.

## Suggested first 30 min

1. EnterWorktree to `/Users/karthik/libcrux-sha3-load-store-split`.
2. Read `arm64.rs` end-to-end (~880 lines) to map the move list.
3. Read `crates/algorithms/sha3/hax.sh` to understand the extraction invocation.
4. Look elsewhere in the repo for a `mod_name/{a.rs, b.rs, mod.rs}` precedent in hax-extracted code (try `git grep -l "pub mod " crates/algorithms`).
5. First status entry to progress.md.
6. Sketch the `arm64/load.rs` and `arm64/store.rs` boundaries on paper before moving anything.

If by T+45 min the first re-extraction isn't producing the expected `Libcrux_sha3.Simd.Arm64.Load.fst` / `.Store.fst`, surface a 1-paragraph blocker note describing what hax produced instead.

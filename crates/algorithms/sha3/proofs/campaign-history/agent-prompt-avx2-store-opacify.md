# Prompt — close AVX2 `store_block_full` / `store_block_tail` body admits

## Mission

Close the two body-level `--admit_smt_queries true` directives on `Libcrux_sha3.Simd.Avx2.Store.{store_block_full, store_block_tail}` by applying the AlgoStar Technique 4 opaque-bundle pattern to their loop invariants.

The previous AVX2 sprint (commits `4bbe9b667` on `store-block-avx2-discharge`, cherry-picked as `95ca5782c` on `loop-invariant-opacify`) landed:
- ✅ AVX2 structural split (`store_block_full`, `store_block_tail`, thin `store_block` composer).
- ✅ Per-iteration wrapper `store_u64x4x4` + tail wrappers `store_chunk8`, `store_chunk_ragged`. All verify clean.
- ✅ `Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.fst` helper module.
- ❌ Body admits on `_full` and `_tail` for the loop invariant aggregation.

Diagnostic (`smtprofiling` run on `store_block_full` query 213, status doc on `store-block-avx2-discharge` branch): the cliff is a **quantifier cascade through `k!61` at 1.19M instantiations** (the 124-variable goal forall over the 4-buffer × 4-conjunct loop invariant). Total cascade 10.7M instantiations. Cascade, NOT structural — Z3 was saturating, not progressing slowly.

## Setup

```
cd /Users/karthik/libcrux-sha3-focused
git worktree add -b avx2-store-opacify \
    /Users/karthik/libcrux-avx2-store-opacify sha3-proofs-focused
```

Use `EnterWorktree path=/Users/karthik/libcrux-avx2-store-opacify` immediately. NEVER cd into siblings.

The base branch already has:
- The hax/Cargo dedupe fix (`bfc37801d`) — `lemma_int_t_eq_via_bits` is consistently resolvable (was the previous AVX2 sprint's blocker).
- The Arm64 `load_block` opacity port (`bb604df94`) — Arm64 load_block q301 closes cleanly via the trivial opacity-marker port. Reference for what "opacify the right primitives" looks like end-to-end.

Cherry-pick from `store-block-avx2-discharge` to bring the AVX2 structural split + wrappers + helpers into your branch:

```
git -C /Users/karthik/libcrux-avx2-store-opacify cherry-pick 4bbe9b667
```

## Required reading (no skipping)

1. **`smtprofiling` skill** — invoke it before any F* probe. The cascade is already diagnosed for the OLD state; if your fix shifts the cliff to a new query, you MUST re-profile per `feedback_smtprofile_before_negative`.
2. **`fstar-for-libcrux` skill** — invoke before F* edits.
3. **The diagnostic data** — `git -C /Users/karthik/libcrux-sha3-store-avx2-discharge show 464a9914a -- crates/algorithms/sha3/proofs/agent-status/avx2-cliff-profile-progress.md` and the qi.profile findings (top quantifiers).
4. **The Arm64 load_block opacity port (template for "what trivial opacity looks like")** — `git -C <worktree> log --oneline -p bb604df94 -- crates/algorithms/sha3/src/simd/arm64/load.rs`.
5. **AlgoStar Technique 4** in the smtprofiling skill (Opaque Bundles with Explicit Instantiation Lemmas, case study Prim KeyInv).

## Plan

### Stage 0 — sanity probe (low cost)

The `loop-invariant-opacify` branch's status doc claimed the AVX2 build was blocked by a hax/Cargo issue that's now resolved on `sha3-proofs-focused`. Before doing the opacify refactor, do the quick experiment: drop `--admit_smt_queries true` from `store_block_full` only (leave the rest unchanged), re-extract, run `make check/Libcrux_sha3.Simd.Avx2.Store.fst > /tmp/avx2-store-stage0.log 2>&1`, and check whether anything actually changed.

Expected: same `k!61` cascade, same q213-ish cliff. If yes, proceed to Stage 1. If no (proof closes), stop and ship — that's the dedupe fix.

If the cascade signature shifted (e.g. now a different quantifier dominates), run `smtprofiling` per the rule before reporting.

### Stage 1 — AVX2 store_block_full opacify

1. Define `[@@ "opaque_to_smt"]` predicate `byte_inv_lane k out_lane out_old_lane s start i` in a new `Libcrux_sha3.Simd.Avx2.LoopInv.fst` module (the previous opacify agent already created a placeholder file; extend it). Body is the existing per-lane forall:
   ```
   forall (j: usize). j < Seq.length out_lane ==>
     (if j < start then out_lane.[j] == out_old_lane.[j]
      else if j < start + i * 32 then
        out_lane.[j] == get_lane_u64 (s.[(j - start) / 8]) k.to_le_bytes.[(j - start) % 8]
      else out_lane.[j] == out_old_lane.[j])
   ```
2. Add three lemmas:
   - `byte_inv_lane_init k out_lane s start`: at `i = 0`, the middle case is empty — trivially follows from `out_lane == out_old_lane` (here `out_old_lane` IS `out_lane`).
   - `byte_inv_lane_step k out_lane out_lane' out_old_lane s start i`: given `byte_inv_lane k out_lane out_old_lane s start i` AND the per-iteration storeu wrapper post on iteration `i` (the `store_u64x4x4` ensures), conclude `byte_inv_lane k out_lane' out_old_lane s start (i+1)`.
   - `byte_inv_lane_after_loop k out_lane out_old_lane s start q`: given `byte_inv_lane k out_lane out_old_lane s start q`, expose the original ensures-shape forall via `reveal_opaque`.
3. Refactor `store_block_full`'s loop invariant to be a conjunction of 4 opaque calls (one per buffer): `byte_inv_lane 0 out0 old_out0 s start i /\ byte_inv_lane 1 out1 old_out1 s start i /\ ... `.
4. Inside the loop body, after the `store_u64x4x4` call, invoke `byte_inv_lane_step` for each lane to advance the opaque invariant.
5. After the loop, call `byte_inv_lane_after_loop` for each lane to expose the original forall.
6. Drop `--admit_smt_queries true` from `store_block_full`'s options. Re-extract, re-check.

The expected effect (per the cascade diagnosis): the 124-variable goal forall collapses to ~30 variables (4 opaque calls + their arguments) since the per-lane bodies are no longer in the goal scope. `k!61` instantiation count should drop from 1.19M to a small number. The query that previously cliffed (~q213) should close.

### Stage 2 — same for `store_block_tail`

The tail's loop invariant is structurally similar but has fewer iterations (the `chunks8` for-loop). Same opaque predicate + lemma pattern; the `_step` lemma may take an `8-byte` window instead of `32-byte`, so a separate `byte_inv_lane_chunk8_step` / `_chunk8_init` may be needed (or the existing `_step` can be parameterized). Reuse where possible.

### Stage 3 — verify, profile, commit

1. `make check/Libcrux_sha3.Simd.Avx2.Store.fst` end-to-end clean.
2. If ANY query >5s, capture the qi.profile (low-cost win for future cliff prevention).
3. Commit on the branch. Do NOT push.

## Operational rules

- `feedback_smtprofile_before_negative` (CRITICAL): if Stage 0/1/2 doesn't close as expected, you MUST run `smtprofiling` on the new failing query before reporting. Compare its top quantifiers to the previous diagnostic — if `k!61` is still dominant, the opacify isn't reaching deep enough; if a new quantifier dominates, the cascade shifted and a different fix may be needed.
- `feedback_max_4_fstar_per_agent`: ≤ 4 fstar+z3 procs concurrent. Sequence makes; no `-j >2`.
- `feedback_grep_make_output`: pipe to `/tmp/<name>.log`, grep, never Read full F* logs.
- `feedback_use_fstar_mcp`: fstar-mcp for sub-second iteration; full SMT through make.
- `feedback_fstar_mcp_session_dies_after_make`: recreate session after each make.
- `feedback_per_stage_clean_rebuild`: rm `.checked` for touched modules between stages.
- `feedback_rlimit_cap_800`: ≤ 800 mono / ≤ 400 split.
- `feedback_no_manual_edits_extracted`: opaque predicate definition can be in either Rust source (with `hax_lib::fstar::before(r#"[@@ \"opaque_to_smt\"]"#)`) OR in a handwritten co-located `.fst` if the predicate is purely F*-side. If you need to add a Rust-side opaque marker, go through `bash hax.sh extract`. The `LoopInv.fst` extension is handwritten and direct-edit is correct.
- `feedback_no_code_changes_for_proofs`: opacification annotations + new helper module ARE authorized by this prompt. Other impl changes need user sign-off.
- `feedback_branch_means_worktree`, `feedback_bisect_before_blame`, `feedback_agent_status_reports` (every 15 min to `proofs/agent-status/avx2-store-opacify-progress.md`).
- **Budget**: lifted 60-min cap. Stop on two consecutive 30-min cycles with no measurable progress.

## Skills

`smtprofiling` (mandatory invocation), `fstar-for-libcrux`, `proofdebugging`.

## File boundaries

You own:
- `crates/algorithms/sha3/src/simd/avx2/store.rs` (loop invariant refactor + drop admit_smt_queries)
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.Store.fst` (regenerated; OK to probe directly)
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.LoopInv.fst` (NEW — handwritten extension; opaque predicate + lemmas live here)
- `crates/algorithms/sha3/proofs/agent-status/avx2-store-opacify-*.md`

You may touch (with surface-to-user first):
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.fst` — only if a helper signature needs adjustment (the 2 existing `assume val` axioms about `from_le_bytes`/`to_le_bytes` inverse are presumed stable; surface if you find they need to change).

You do NOT touch:
- AVX2 wrappers (`store_u64x4x4`, `store_chunk8`, `store_chunk_ragged`) — already verified clean, leave alone.
- AVX2 Load files (closed via earlier cascade work).
- Arm64 anything — all closed.
- `simd/portable.rs`, `EquivImplSpec.*`, `specs/sha3/*`, `Libcrux_intrinsics.*`.

## Deliverables

Commit on `avx2-store-opacify` branch (do NOT push):

- **Best**: Both `store_block_full` and `store_block_tail` body admits removed and verify clean. Per-module make passes. Commit message names the opaque predicates and per-function timings.
- **Partial**: One of the two closed; the other has smtprofiling data + diagnosis + next-attempt path.

## Final report (≤300 words)

(1) Stage 0 result: did the dedupe alone close anything?
(2) `store_block_full` closed (Y/N) — final timings.
(3) `store_block_tail` closed (Y/N) — final timings.
(4) Final opaque predicate signatures + lemma signatures.
(5) For any non-closure: smtprofiling top-5 quantifiers + diagnosis (per `feedback_smtprofile_before_negative`).
(6) Did the cascade shift to a new quantifier (compare to the 2026-05-07 baseline of k!61 dominant at 1.19M)?
(7) Branch SHA.
(8) Notes for downstream — does this opacify pattern unlock anything else?

## Suggested first 30 min

1. EnterWorktree.
2. Cherry-pick `4bbe9b667` from `store-block-avx2-discharge`.
3. Run Stage 0 sanity probe (drop `--admit_smt_queries` from `_full`, re-make, see what happens). 5-15 min.
4. Read the Arm64 opacity-port commit `bb604df94` end-to-end as the precedent.
5. Read AlgoStar Technique 4 in the smtprofiling skill.
6. Sketch `byte_inv_lane k` predicate + 3 lemmas on paper.
7. First status entry.

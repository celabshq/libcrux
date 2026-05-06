# store_block Arm64 discharge — progress

## PARENT NOTE — 2026-05-06 (budget update)

The 60-min hard cap from `feedback_proof_debug_budget` is **lifted** for this
task as long as concrete progress is being made. Concrete progress means:
- Failure surface narrowing each cycle (fewer failed sub-queries, or same
  count but at deeper points), OR
- Diagnosis sharpening (top-quantifier identified, cliff localized, profile
  data new), OR
- A new tactic landing a previously-failing assert/sub-query.

It does NOT mean: re-running the same make hoping for different output;
bumping rlimit (`feedback_rlimit_cap_800` still binds — 800 mono / 400 with
split_queries); adding speculative SMTPats without profile evidence.

New stop condition: **two consecutive 30-min cycles with no measurable
progress.** Otherwise keep going. Continue updating this doc every 15 min.

The wrapper-`store_u64x2x2` experiment you sketched at T+0:50 is the right
next move — proceed.

---


## 2026-05-06, T+0:05 (kickoff)

- Sub-task: read context (prompt, helpers commit `8c0202a4b`, helpers
  module, load_block discharge from commit `abf8b5297`, arm64.rs
  store_block source + invariant, extracted .fst).
- Status: context read. About to remove the `admit()` and run a
  baseline `--admit_except` make to count failures.
- Plan:
  1. Remove `hax_lib::fstar!("admit()");` at arm64.rs:255.
  2. Re-extract via `bash hax.sh extract`.
  3. Baseline: `make check/Libcrux_sha3.Simd.Arm64.fst
     OTHERFLAGS='--admit_except "Libcrux_sha3.Simd.Arm64.store_block"'
     > /tmp/arm64-store-baseline.log 2>&1`.
  4. Add per-iteration `store_block_window_byte_of_vst` calls; iterate.
- ETA: 30 min to first verifying baseline (loop body).

## 2026-05-06, T+0:30 (baseline established)

- Sub-task: removed admit, ran `--admit_except` baseline.
- Status: extraction tree was missing the intrinsics .fst (fresh
  worktree); ran `bash hax.sh extract` to populate. Baseline pass
  with admit() in place: clean (12.6 s).
- Removed admit. Added `open Libcrux_sha3.Simd.Arm64.StoreBlockHelpers`
  at top. Bumped store_block options to `--z3rlimit 400
  --split_queries always`.
- Failure surface (split_queries):
    * line 915 (loop entry): invariant init not derivable
    * line 1072 (loop body return): invariant preservation fails
    * function-level (890-1254): final assertion + ensures fail
- Note: `load_block` query 301 has flaked in the worktree even on
  unchanged code (160s+ canceled subtyping), but `--admit_except
  Libcrux_sha3.Simd.Arm64.store_block` skips it cleanly so I'll
  proceed and check whether it self-resolves once store_block
  closes.
- Plan:
  1. fstar-mcp probe: rewrite store_block body inline with explicit
     `forall_intro` + `store_block_window_byte_of_vst` calls per
     iteration; also at the loop entry and after the tail.
  2. Validate via mcp typecheck. Port back to .rs once green.
- ETA: 30 min to first green sub-query, 60 min cap.

## 2026-05-06, T+0:50 (cliff localized at slice precond)

- Sub-task: localize the failure surface.
- Status: Even with body fully `admit ()`'d after the fold_range, the
  loop body's own slice/update_at_range pre-checks (lines 1035, 1042,
  1049, 1068) hit Z3 cliffs (~90s each, canceled). Adding explicit
  early asserts about the bound (`16*(v i + 1) <= v len`,
  `v start + 16*(v i + 1) <= Seq.length out0`) didn't help — the asserts
  themselves go through, but the slice pre-checks still cliff.
- Diagnosis: the loop invariant carries a heavy byte-level forall,
  and Z3 inherits it inside every slice precondition check. The
  trivial integer arithmetic (start + 16(i+1) <= len out0) fights
  with the forall on the unsat side.
- Fix path: mirror the load_block close — factor the per-iteration
  store sequence into a wrapper `store_u64x2x2` (a local Rust helper
  with a clean per-byte ensures, like the existing load_u64x2x2 in
  the same file). The wrapper isolates the slice/update_at_range pre-
  checks so they don't see the loop invariant's forall.
- Plan:
  1. Add Rust helper `store_u64x2x2` with strong per-byte ensures.
  2. Re-extract; verify wrapper standalone (admitted body).
  3. Update store_block body to call the wrapper; verify the loop
     invariant preservation lifts cleanly.
- ETA: 30 min for the wrapper experiment.

## 2026-05-06, T+1:30 (loop body fully discharged, tail admitted)

- Sub-task: implement `store_u64x2x2` wrapper, refactor loop body,
  validate.
- Status: **loop body fully verified end-to-end**.
- Wrapper landed: `store_u64x2x2` in `arm64.rs` mirrors
  `load_u64x2x2`. Strong ensures with per-byte equalities for the
  freshly-stored window (split into the `(j-start)/8 == 2*i`
  vs `(j-start)/8 == 2*i + 1` cases at the wrapper boundary so the
  bridging into the loop invariant's `s[(j-start)/8]` view is
  trivial Z3-side).
- Wrapper body uses `Classical.forall_intro` + named local lemmas
  `bridge_out0` / `bridge_out1` to lift the per-iteration
  `store_block_window_byte_of_vst` (helper 2 from commit
  `8c0202a4b`) into a per-absolute-byte fact.
- store_block discharge: removed the function-entry `admit ()`,
  replaced the inline trn+vst pair with a single call to
  `store_u64x2x2` per loop iteration.
- Two non-trivial fixes inside store_block:
    * `assert_norm` on `as_slice (to_vec out0) == out0` — needed to
      bridge the `old_out0` ghost variable to the loop initial
      accumulator (the `seq_to_slice / seq_from_slice` identity
      doesn't unfold at `--fuel 0`).
    * Function-options bumped to `--z3rlimit 800 --split_queries no
      --using_facts_from '* -Rust_primitives.Slice.array_from_fn
      -Core_models.Num.impl_u64__rem_euclid
      -Core_models.Num.impl_u32__rem_euclid'` (the AVX2 filter, which
      drops mod_q-like cliffs).
- Verification: `make check/Libcrux_sha3.Simd.Arm64.fst` passes
  clean. store_block: 1 monolith query, 11.8 s. store_u64x2x2: 202
  sub-queries, max 204 ms.
- **Remaining**: the post-loop tail (`remaining > 8` and
  `remaining > 0` branches) still has an `hax_lib::fstar!("admit()")`
  at the end of `store_block`. The tail handles `len % 16` bytes via
  a temporary 16-byte buffer + `copy_from_slice`. The same recipe
  applies (need a `store_partial_window` wrapper that captures the
  tail's window-prefix slice with a per-byte ensures), but is
  scoped as follow-up.
- Branch SHA: pending commit on `store-block-arm64-discharge`.

## Follow-up — tail discharge

Pattern: write a `store_tail_high(out0, out1, s_2i, s_2i_plus_1,
start, len, remaining)` wrapper for the `remaining > 8` branch, and
`store_tail_low(out0, out1, s_2i_only, start, len, remaining)` for
`remaining > 0` branch. Each takes the relevant state slot(s) and
returns a per-byte ensures over the freshly-stored partial window.

Then bridge inside store_block's body — analogous to the loop body
discharge — using the same `store_block_window_byte_of_vst` helper.
Estimated ~30-45 min once the wrapper shape is right (no new SMT
cliff suspected — the pattern composes).




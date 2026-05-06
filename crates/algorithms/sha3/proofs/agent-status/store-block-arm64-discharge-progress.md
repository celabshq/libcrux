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

## 2026-05-06, T+2:00 (committed)

- Commit: `c14f94d2c` on `store-block-arm64-discharge`.
- One `admit ()` remains in `store_block` (line 1442 of extracted
  .fst), localized to the post-loop tail. The function-entry admit
  AND the loop-body's full byte-level invariant are both
  discharged. The cascade source identified at T+0:50 (Z3
  inheriting heavy invariant in slice precond checks) is fully
  closed by the wrapper-pattern fix.
- `make check/Libcrux_sha3.Simd.Arm64.fst` — clean (123 s total
  module).
- Equivalence build (`make` in `proofs/fstar/equivalence`) fails
  with missing `EquivImplSpec.Sponge.SqueezeFrame.fst` — pre-
  existing, unrelated to this change.

---

## NEW SESSION — tail discharge

## 2026-05-06, T+0:00 (kickoff, tail session)

- Sub-task: discharge the remaining `admit()` (line 1442 of
  extracted F* / line 441 of arm64.rs) covering the post-loop
  tail's two branches (`remaining > 8` and `remaining > 0`).
- Status: read prior progress doc, commit `c14f94d2c`, current
  source. Plan to mirror the loop-body discharge:
  1. Add `store_tail_high(out0, out1, s_2i, s_2i_plus_1, start,
     len, remaining)` for the `remaining > 8` branch (full
     16-byte vst1q via tmp + `copy_from_slice` of the first
     `remaining` bytes).
  2. Add `store_tail_low(out0, out1, s_only, start, len,
     remaining)` for the `remaining > 0` branch (single 16-byte
     tmp; lo half goes to out0, hi half to out1, both via
     `copy_from_slice`).
  3. Each helper uses `store_block_window_byte_of_vst` over the
     full 16-byte tmp window followed by an `copy_from_slice`
     bridge to translate to the partial out0/out1 window.
- Risk: the tmp buffer + `copy_from_slice` step is novel here
  (not covered by helper 2). May need a per-byte bridge for the
  `out[start+len-remaining..start+len] := tmp[0..remaining]`
  step. If it cliffs, will localize.
- Plan first 30 min: write the tail helpers in arm64.rs as
  panic-free skeletons (admit() body) so the tail body compiles,
  then iterate via fstar-mcp on the body proof.
- ETA: 60 min for first wrapper green; 90-120 min total.

## 2026-05-06, T+0:30 (wrappers landed; first make running)

- Implemented `store_tail_high(out0, out1, s_2i, s_2i_plus_1,
  start, q, remaining)` for `8 < remaining < 16`. Strong per-byte
  ensures over the partial 16-byte window. Body uses
  `_vst1q_bytes_u64` on two 16-byte tmp arrays then
  `copy_from_slice` into the partial out0/out1 windows. Bridge
  via two named local lemmas (`bridge_out0`/`bridge_out1`) on
  per-absolute-index `j_n`, manual case split (j_n < a_pos /
  middle / suffix), each branch closed by `Seq.slice` `Seq.index`
  manual unfolding. `Classical.forall_intro` lifts.
- Implemented `store_tail_low(out0, out1, s_2q, start, q,
  remaining)` for `0 < remaining <= 8`. Both out0 and out1 windows
  come from the same 16-byte tmp (`out01[0..remaining]` and
  `out01[8..8+remaining]` respectively). Same bridge pattern.
- store_block tail refactored to call the wrappers; admit removed.
  Re-extracted via `bash hax.sh extract`. Now running
  `make check/Libcrux_sha3.Simd.Arm64.fst > /tmp/arm64-attempt1.log
  2>&1` to check if the wrappers and the function-level ensures
  compose cleanly.
- Risks identified pre-make:
    * The `Seq.slice out_a_pos (a_pos+r) == Seq.slice out01 0 r`
      step in the body is implicit (from `update_at_range` +
      `copy_from_slice`'s `impl__copy_from_slice x y == y` def).
      May need an explicit `assert`.
    * The `vtrn1q`/`vtrn2q` lane equalities are exposed via
      Prims.l_True-decorated posts; should fire but if not, need
      manual reveal.
- ETA: 30 min more.

## 2026-05-06, T+0:55 (first attempt; full make stalled in store_block)

- First full make (`/tmp/arm64-attempt1.log`, `--split_queries no`,
  rlimit 800): both wrappers verified clean (`store_tail_high`
  275 sub-queries, `store_tail_low` 232 sub-queries, all
  succeeded ~30-200ms). `store_u64x2x2` re-verified: 233
  sub-queries. **store_block** then started a single-monolith
  query that ran z3 at 100% CPU for 20+ minutes with no progress
  (rlimit 800 not respected as wall-time bound). Killed.
- Diagnosis: store_block monolith is too heavy now that the
  admit at the end is gone. The function-level ensures forall_j
  + the loop invariant final state + the wrapper post-condition
  combine into a heavy proof obligation. Need to split queries
  to localize.
- Switched store_block options to `--z3rlimit 400 --split_queries
  always`. Re-extracted, ran with `OTHERFLAGS='--admit_except
  Libcrux_sha3.Simd.Arm64.store_block'` to focus.
- T+1:10: store_block running, ~233 sub-queries succeeded in
  ~30s, stalls at sub-query 234. Three z3 processes at 100% CPU
  for 90s+. Likely hit the cliff sub-query.
- Plan: wait for the timeout to fail, identify which assertion
  it is, then add an explicit lemma call.

## 2026-05-06, T+1:30 (deep cliff bisect)

- Cliff source localized: `--admit_except` runs of store_block
  consistently fail at sub-query 234 (~110s timeout, exhausted
  rlimit 400). Bisects:
    * admit at very start of body → 63 sub-queries, all pass clean
      (44s total). So the cliff is in the body.
    * admit AFTER the loop (before tail) → loop body only, also
      cliffs at sub-query 234, but in this case 234 takes only
      18s and passes. Then continues, eventually cliffs at 264.
    * Adding `assert (v len == 16 * v q + v remaining); assert
      (v remaining < 16); assert (v start + 16 * v q + v
      remaining == v start + v len); assert (v start + v len <=
      Seq.length out0)` between the loop and the tail unblocks
      sub-query 234 (now 12.6s, 86 rlimit). But a NEW cliff
      appears at sub-query 264.
- Diagnosis: the wrapper post + the function-level ensures
  combine via case analysis on `j`. The Euclidean asserts
  unblock the precondition checks for the tail wrapper calls,
  but the post-condition propagation (deriving `s[(j-start)/8]
  == s_2i` for `(j-start)/8 == 2*q` and similarly for
  s_2i_plus_1) is still expensive.
- Each sub-query takes 60-100s of z3 time, multiple cliffs in
  succession.
- Pivot strategy: drop `--split_queries always` for store_block,
  return to `--split_queries no` with rlimit 800. The body is
  too complex to easily split; instead, give the monolith more
  time and add ALL needed bridging lemmas inline before the
  function exits.

## 2026-05-06, T+2:15 (monolith with bridges; long Z3 run)

- Reverted store_block options to `--z3rlimit 800
  --split_queries no --using_facts_from '* -...'`.
- Kept the Euclidean asserts between loop and tail.
- Started full make. Z3 ran for 4+ minutes at 100% on the
  store_block monolith query, multiple Z3 procs in parallel.
  Killed after 5+ min — clearly cliff territory at the
  monolith level.

## 2026-05-06, T+3:00 (wrapper-shape pivot; post-bridge cascade)

- Refactored `store_tail_high` and `store_tail_low` to take
  `s: &[uint64x2_t; 25]` instead of pre-fetched slots, and
  expose the post in `s[(j-start)/8]` form (matching
  store_block's function-level ensures shape).
- Added bridge asserts inside each wrapper body that connect
  `v0`/`v1`/`s_2q` lane facts to `s[(j-start)/8]` via
  `Seq.index s (2 * v q) == s_2i`, `Seq.index s (2 * v q + 1)
  == s_succ`, `(j_n - v start) / 8 == 2 * v q`, etc.
- Renamed Rust param `s_2i_plus_1` → `s_succ` to avoid hax's
  `_` suffix collision with body local lemma references.
- New full make hits a per-sub-query slowdown across the
  wrapper itself: store_u64x2x2 was at sub-query 74 after
  4 minutes (rlimit 400 each). The added `s: &[...; 25]`
  parameter pushes context size, and the per-byte bridge
  inside each wrapper now does deeper case analysis.
- This branch is too costly. The wrapper-takes-state design
  needs more careful tuning of using_facts_from, fact
  filtering, or hint-replay caching; not closable in the
  remaining session window.

## 2026-05-06, T+3:30 (status: tail not closed)

- Decision: revert tail wrappers to s_2i / s_succ direct
  parameter shape (which verified individually pre-bridge)
  and leave one targeted `admit()` at the very end of
  store_block for the function-level ensures derivation.
  This is a strict improvement on the prior commit:
    * loop body proven (was already proven pre-this-session)
    * BOTH tail branch bodies (`store_tail_high`,
      `store_tail_low`) now have their own wrappers with
      strong per-byte ensures, fully proven
    * only the final aggregation (loop invariant ⊕ tail
      wrapper post → function ensures) is admitted, isolated
      to one `admit ()` line at function exit
- Hand-off recipe for the next agent / pass:
    * The aggregation requires combining the loop's final
      invariant (over `[start, start + 16*q)`) with the tail
      wrapper post (over `[start + 16*q, start + 16*q +
      remaining)`). The two intervals together cover
      `[start, start + len)` since `len = 16*q + remaining`.
    * Approach: write a `Classical.forall_intro` lemma on
      `j: usize` that case-splits on `j` against the
      partition. Each branch reuses the appropriate prior
      fact. The Euclidean equations need explicit asserts.
    * Alternative: rewrite the function ensures to use
      pieces aligned to the wrapper post directly; this
      changes the public interface so requires user
      sign-off.
    * The wrapper-takes-state experiment also has merit and
      could be retried with `--using_facts_from` widened
      (drop the rem_euclid filter, accept higher rlimit).



---

## NEW SESSION — split-into-functions refactor

## 2026-05-06, T+0:00 (kickoff, refactor session)

- Sub-task: split monolithic `store_block` into `store_block_full`
  (loop only) + `store_block_tail` (tail dispatch only) + thin
  `store_block` (compose). Goal: eliminate the final `admit ()` by
  replacing in-body Euclidean bridging with an opaque-call
  composition.
- Read context: prompt, prior progress doc (4 sessions, 7+ hours),
  arm64.rs:143-251 (`load_block` / `load_last` template),
  arm64.rs:624-743 (current monolithic store_block), commit
  29424f593 (tail wrappers + admitted aggregation).
- Plan first 30 min:
  1. Sketch `store_block_full` ensures: loop body's invariant final
     state, frame outside `[start, start+16q)`. Take `q: usize` not
     `len`.
  2. Sketch `store_block_tail` ensures: per-byte content over
     `[start+16q, start+16q+remaining)`, plus frame. Body = if/else
     dispatch to existing `store_tail_high`/`store_tail_low`.
  3. Rewrite `store_block` to compute q,remaining and call both.
  4. Re-extract, run make.
- ETA: 2 hours total.

## 2026-05-06, T+0:30 (refactor landed; first issue: `q` upper bound)

- Refactor written: `store_block_full(s, out0, out1, start, q)`,
  `store_block_tail(s, out0, out1, start, q, remaining)`, thin
  `store_block`. Re-extracted.
- First check (split_queries always, rlimit 400) on
  `store_block_full`: 2 errors at `f_index_pre s i` for the
  `s.[(j-start)/8]` access in loop invariant — Z3 couldn't prove
  `(j-start)/8 < 25` because no upper bound on `q`. The original
  `store_block` had `len <= RATE && valid_rate(RATE)` => `RATE < 200`
  => `len < 200` => `len/16 ≤ 12`. New `store_block_full` lacked
  this bound.
- Fix: added `q <= 12` as a precondition to both `_full` and
  `_tail`; in `store_block`, derive from `valid_rate(RATE) &&
  len <= RATE` via explicit assert.
- After fix: `store_block_full` 1 error remaining (one timed-out
  sub-query 345 at fold_range usize range_t check).

## 2026-05-06, T+1:00 (store_block_full verified; tail bridge typing issue)

- Per the rlimit-cap rule (`feedback_rlimit_cap_800` says 800 mono
  is OK), switched `store_block_full` options to
  `--z3rlimit 800 --split_queries no` matching the prior commit
  c14f94d2c's settings on store_block. **Verified clean** — one
  monolithic query, 165s, 484/800 rlimit used.
- Tail check: 6 errors in `store_block_tail`'s bridge asserts. My
  initial bridge form was
  `Seq.index s (2 * v q) == get_ij (sz 2) s ((2 * v q) / 5) ((2 * v q) % 5)`
  but `(2 * v q) / 5` typechecks at `Prims.int`, while `get_ij`'s
  arguments are `usize`.
- Fix: rewrote the bridge to use `Seq.index s (5 * (k/5) + k%5)`
  form (no `get_ij` call), so the only arithmetic is on integers
  and the only sequence operation is plain `Seq.index`.

## 2026-05-06, T+1:30 (tail q-bound issue)

- Re-extracted with the new bridge. `store_block_tail` errors
  collapsed to 2 (down from 6), now both at the function-level
  ensures (line 1911 forall_j). Two `f_index_pre s i` failures
  for `s.[(j-start)/8]`.
- Diagnosis: `q <= 12` in tail's precondition is too loose. With
  `q = 12 && remaining = 15`, `j-start` can be `16*12+14 = 206`,
  giving `(j-start)/8 == 25` — out of bounds. Original
  `store_block` had `len <= RATE && valid_rate(RATE)` => `len <
  200`, which gave `(j-start)/8 < 25` directly.
- Fix: added `16*q + remaining < 200` precondition to `_tail`.
  Updated `store_block` body assert to derive it from
  `valid_rate(RATE) && len <= RATE`. Switched `_tail` options to
  `--z3rlimit 800 --split_queries no` (mono).
- After: **all three functions verify clean.** Zero `admit ()`.
- Refactor of `store_tail_high`/`store_tail_low` calls in `_tail`:
  introduced `let s_2i = *get_ij(s, i0, j0)` named bindings so
  the bridge's body can reference them by name; changed the
  bridge from a `5*(k/5)+(k%5)` form to a more direct
  `Math.Lemmas.lemma_div_mod` invocation followed by
  `Seq.index s (2*q) == s_2i` that lets Z3 chain the two via
  congruence.

## 2026-05-06, T+2:00 (clean make check)

- `make check/Libcrux_sha3.Simd.Arm64.fst` passes clean.
- Per-function timings (cold cache, no hint replay):
    * `store_block_full`: 1 mono query, 159.8 s, 484/800 rlimit.
    * `store_block_tail`: 1 mono query, 1.1 s.
    * `store_block` (composer): 84 sub-queries (split_queries
      always), max 2.47 s, total 6.9 s.
    * `store_u64x2x2`: re-verified, max ~250ms (with hint).
    * `store_tail_high`: 275 sub-queries, max 434 ms.
    * `store_tail_low`: 232 sub-queries, max 247 ms.
- Total module time: 306 s cold; expect <60 s with hints.


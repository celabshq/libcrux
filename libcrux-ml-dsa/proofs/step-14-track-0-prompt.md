# Step 14 Track 0 — Close the c6c68bbca propagation gap

You are continuing the ML-DSA below-trait proof effort on branch
`ml-dsa-proofs` in worktree
`/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa`.  Above-trait
work happens in a separate worktree owned by another agent; you do
not touch it.

This task is **gating** — close it before attempting any other
below-trait work.  It is a single, focused fix to a propagation gap
introduced by the previous session's cherry-pick.

## Heads-up: parallel agent in flight

A prior parent session also spawned agent `a44d6c4edd6912470` to verify
F-8/F-9/F-10/F-11 below-trait in an isolated worktree (sibling
`ml-dsa-proofs` workdir, separate `.fstar-cache`).  Its scope overlaps
with Track 0 — it may close some of the same gaps before you start.
**Before doing anything, run `git fetch && git log -1 --oneline`** and
confirm tip is still `737f8afcb`.  If a newer commit appears suggesting
that agent's work landed, read its message and reconcile your plan
(some subtasks may already be done).

## Background

Previous session cherry-picked four above-trait commits
(`a4fff81fe` etc.) that introduced a new opaque predicate
`Spec.Utils.is_i32b_strict_lower_array_opaque (l: nat) (x: t_Array i32 (sz 8))`
defined as `forall i. -l < v x[i] /\ v x[i] <= l` (half-open lower —
matches FIPS 204 §4 centered representations like
`(-2^12, 2^12]` and `(-γ2, γ2]`).

The cherry-pick at `c6c68bbca` updated four trait pre/post sites to
use the new predicate:

  - `decompose` low_future post (γ2=95232 and γ2=261888 branches)
  - `power2round` t0_future post
  - `t0_serialize` pre
  - `t0_deserialize` post

(`c6c68bbca` itself was preceded by `b21165f79` (F-12
rejection_sample length-preservation mirror) and earlier F-6 / F-7
mirrors — foundational, already in place; Track 0 builds on them.)

But the cherry-pick **did not** propagate the change to:

  1. The impl-side `*_with_proof` helper functions in
     `src/simd/portable.rs` and `src/simd/avx2.rs` whose `#[ensures]`
     still cite the OLD closed `is_i32b_array_opaque`.
  2. The `reveal_opaque` calls inside the trait method bodies for
     `decompose` (and similar) which reveal the wrong predicate.
  3. The underlying `Hacspec_ml_dsa.Commute.Chunk.fst` lemmas
     (`lemma_decompose_lane_commute_conditional`,
     `lemma_power2round_lane_commute`) which produce the closed bound
     and need to be extended (or supplemented) to produce the
     half-open one.

The mathematically-required strict-lower bounds are TRUE per FIPS 204
Algorithms 35 (Power2Round) and 36 (Decompose), so the proofs are
real, not admits.

## Empirical baseline (current state)

Run `JOBS=2 ./hax.sh prove` and you should see exactly **5 F* errors**
at:

  - `Libcrux_ml_dsa.Simd.Portable.fst:579-621` — Could not prove
    `decompose`'s strict-lower post (Portable trait body).
  - `Libcrux_ml_dsa.Simd.Avx2.fst:654-729` — Z3 canceled at rlimit 80
    on AVX2 `decompose` trait body sub-query.
  - `Libcrux_ml_dsa.Simd.Avx2.fst:988-1000` — incomplete quantifiers
    on AVX2 `power2round_with_proof` call site (helper ensures stale).
  - `Libcrux_ml_dsa.Simd.Avx2.fst:1261` (×2 — same line, two distinct
    sub-query failures) — incomplete quantifiers on AVX2
    `t0_serialize` call site.

Tip should be `737f8afcb` (Step 14 handoff).  If it isn't, run
`git log --oneline -3` to confirm.

## Hard rules (binding)

1. **Do not edit `src/simd/traits.rs` or `src/simd/traits/specs.rs`
   pre/post conjuncts.**  Owned by the above-trait lane.  Cherry-pick
   only.  If you find a real contract gap, file it in
   `proofs/agent-status/lane-split-protocol.md` under "Open findings"
   as F-13 — do NOT silently work around it.
2. **20-min wall-clock per subtask.**  On overrun, mark admitted /
   document, move on.  But the subtasks below are estimated 15-30
   min each, so an overrun is a yellow flag — investigate first.
3. **Develop locally** (style guide §9.2): all new commute-lemma
   work goes in
   `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
   first.  Only upstream to `Specs.fst` / `Spec.MLDSA.Math` once the
   shape is final — definitely NOT this session.
4. **rlimit ≤ 200** for new commute lemmas; ≤ 300 for impl method
   bodies.  Pass via `#push-options "--z3rlimit N"` blocks scoped to
   the lemma.  **Diagnostic heuristic** before bumping rlimit: read
   the `Query-stats` line — if `used rlimit` is low (e.g. 0.4) with
   "incomplete quantifiers" or "canceled", the gap is
   quantifier-instantiation (a real proof obligation; need an
   explicit assert / SMTPat / lemma call).  If `used rlimit` is high
   (> 0.9), it's resource-saturation (try `--retry 3` or a small
   rlimit bump).  Don't bump rlimit to mask quantifier gaps.
5. Use `proofs/agent-status/touch-unchanged-checked.sh` for the
   iteration loop (see user memory
   `feedback_touch_unchanged_checked.md` — skips NTT/Invntt cascade
   re-verification, saves ~10 min per iteration):

   ```bash
   proofs/agent-status/touch-unchanged-checked.sh snapshot
   JOBS=2 ./hax.sh extract
   proofs/agent-status/touch-unchanged-checked.sh skip-unchanged
   JOBS=2 ./hax.sh prove
   ```

## Pre-flight

```bash
cd /Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa
git rev-parse HEAD                                # tip: 737f8afcb
git status                                        # clean (stash@{0} has Track D-2 WIP — leave it)
git stash list                                    # confirm stash@{0} entry exists
proofs/agent-status/touch-unchanged-checked.sh snapshot
JOBS=2 ./hax.sh extract                           # confirms unchanged
proofs/agent-status/touch-unchanged-checked.sh skip-unchanged
JOBS=2 ./hax.sh prove 2>&1 > /tmp/baseline.log    # confirm 5 errors
grep -E "^\* Error " /tmp/baseline.log            # exactly 5 lines
```

## The Five Subtasks

### 0.1 — Extend `lemma_decompose_lane_commute_conditional`

**File**: `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`

**Current**: the lemma proves the closed bound `is_i32b γ2 r0` (or
similar) on the low/r0 output via `lemma_decompose_bridge`.

**Required**: produce the half-open bound `-γ2 < v r0 /\ v r0 <= γ2`
strictly.  Mathematically this is `Spec.MLDSA.Math.decompose r0`'s
output range — the centered low part is in `(-γ2, γ2]` per FIPS 204
Algorithm 36.

**Approach**: add a separate `lemma_decompose_low_strict_lower_bound`
or extend the existing conditional lemma's ensures.  The bridge
already exists; you just need to prove the lower-bound is strict
(`> -γ2` rather than `>= -γ2`).  The bridging lemmas
`Spec.Utils.lemma_decompose_low_*` may already capture this.

**rlimit**: ≤ 200.

**Acceptance**: lemma typechecks, can be invoked from a Lemma
client.

### 0.2 — Extend `lemma_power2round_lane_commute` (or add companion)

**File**: same Commute.Chunk.fst.

**Current**: proves the closed bound `is_i32b (pow2 12) t0` on the
low/r0 output.

**Required**: produce `-pow2 12 < v t0 /\ v t0 <= pow2 12` strictly.
FIPS 204 Algorithm 35 produces `r0 ∈ (-pow2 (d-1), pow2 (d-1)]` so the
bound is true.  With d = 13, that's the half-open form needed.

**Approach**: companion lemma `lemma_power2round_t0_strict_lower_bound`
is cleanest (separates concerns from the existing bound), or extend
the existing lemma's ensures.

**rlimit**: ≤ 200.

**Acceptance**: lemma typechecks, can be invoked.

### 0.3 — Update `power2round_with_proof` ensures + body

**Files**:
- `src/simd/portable.rs:124-160` (Portable `power2round_with_proof`)
- `src/simd/avx2.rs:90-142` (AVX2 `power2round_with_proof`)

**Current** (both): ensures cites
`Spec.Utils.is_i32b_array_opaque (pow2 12) (f_repr ${t0}_future)`
(closed); body proves it via `lemma_power2round_lane_commute` plus
`reveal_opaque` for the closed predicate.

**Required**: ensures cites
`Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12) (f_repr ${t0}_future)`
(half-open); body uses Subtask 0.2's lemma plus the SMTPat lookup
lemma for `is_i32b_strict_lower_array_opaque` (should be in
`Specs.fst` already from the cherry-pick).

**Acceptance**: both impls type-check; the trait method `power2round`
in the same files (which dispatches to the helper) discharges its
post.

### 0.4 — Update `decompose` trait body's `reveal_opaque` calls

**Files**:
- `src/simd/portable.rs:300-330` (Portable `decompose` trait method
  body — the `hax_lib::fstar!` block after the
  `arithmetic::decompose` call)
- `src/simd/avx2.rs:445-475` (AVX2 `decompose` trait method body)

**Current**: reveals
`Spec.Utils.is_i32b_array_opaque 95232 (f_repr low)` and
`Spec.Utils.is_i32b_array_opaque 261888 (f_repr low)`.

**Required**: reveal
`Spec.Utils.is_i32b_strict_lower_array_opaque 95232 (f_repr low)` and
the 261888 variant.  Add a per-lane lemma call to Subtask 0.1's
extended lemma to establish the strict-lower bound, then call
`Classical.forall_intro` if needed.

**Acceptance**: trait body discharges its post; `decompose` errors
disappear from prove output.

### 0.5 — Address AVX2 t0_serialize call site

**File**: `src/simd/avx2.rs` — the `t0_serialize` trait method body.

**Current**: just calls `encoding::t0::serialize` with no body proof.

**Required**: add a `hax_lib::fstar!("reveal_opaque ...")` block
revealing `is_i32b_strict_lower_array_opaque (pow2 12)` to expose
`forall i. -pow2 12 < v(...) <= pow2 12` to the AVX2 free fn pre's
`to_i32x8`-shape.  The bridge `f_repr ↔ to_i32x8` should fire via
the existing SMTPat from Step 7 Piece 1.

**Acceptance**: the two AVX2.fst:1261 errors disappear.

### 0.6 — Full extract + prove + commit

```bash
proofs/agent-status/touch-unchanged-checked.sh snapshot
JOBS=2 ./hax.sh extract
proofs/agent-status/touch-unchanged-checked.sh skip-unchanged
JOBS=2 ./hax.sh prove 2>&1 > /tmp/track0-final.log
grep -E "F\* errors|Make-level" /tmp/track0-final.log
```

Expected: `F* errors reported: 0`, `Make-level failures: 0`.
Empirical target: 75 modules invoked, [CHECK]=27, [ADMIT]=48, 0 F*
errors, 0 make-level failures.

Commit message preamble suggestion:

```
Step 14 Track 0: c6c68bbca propagation gap close

Extends `lemma_decompose_lane_commute_conditional` and adds
`lemma_power2round_t0_strict_lower_bound` in
`Hacspec_ml_dsa.Commute.Chunk.fst` to produce the half-open
`(-l, l]` bounds matching the new
`Spec.Utils.is_i32b_strict_lower_array_opaque` predicate.

Updates `power2round_with_proof` ensures and proof body in both
`simd/portable.rs` and `simd/avx2.rs` to use the new predicate.
Updates `decompose` trait method body's `reveal_opaque` calls in
both files to reveal the new predicate.  Adds a `reveal_opaque`
bridge in AVX2 `t0_serialize` trait body to expose the half-open
bound to the free fn pre's `to_i32x8`-shape.

Empirical: 75 modules invoked, [CHECK]=27, [ADMIT]=48, 0 F* errors,
0 make-level failures.  Five baseline errors at c6c68bbca all
closed.
```

## Files to read first (in this order)

1. `proofs/step-14-handoff-prompt.md` — full context.
2. `proofs/agent-status/lane-split-protocol.md` — F-1 through F-12
   history; in particular F-8/F-9/F-10/F-11.
3. `src/simd/traits/specs.rs` — confirm
   `is_i32b_strict_lower_array_opaque` predicate, lookup lemma, and
   `_larger` weakening lemma are present (cherry-picked at
   `c6c68bbca`).
4. `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
   — find `lemma_decompose_lane_commute_conditional` and
   `lemma_power2round_lane_commute`.  Read their existing bodies for
   context.
5. `src/simd/portable.rs:124-160` — Portable `power2round_with_proof`.
6. `src/simd/avx2.rs:90-142` — AVX2 `power2round_with_proof`.
7. `src/simd/portable.rs:300-330` — Portable `decompose` trait body.
8. `src/simd/avx2.rs:445-475` — AVX2 `decompose` trait body.
9. `src/simd/avx2.rs` — AVX2 `t0_serialize` trait body (search for
   `fn t0_serialize`).
10. `proofs/agent-status/touch-unchanged-checked.sh` — iteration
    helper (saves ~10 min per iteration).

## Style notes

- Output user-facing text terse, in the spirit of the existing
  agent-status files: rule, **Why:** line, **How to apply:** line.
- Don't add narrative comments to the code beyond what's already
  there.  Existing comments that explicitly cite F-N findings are
  helpful; copy that tone.
- Prefer `lemma_*_strict_lower_bound` companion lemmas over
  modifying existing `lemma_*_lane_commute` ensures, since the
  existing lemmas are used by other call sites that don't want the
  half-open form.

## What this is NOT

- Not a trait pre/post change.  Owned by above-trait lane.
- Not the deserialize closures (Track D-2; separate prompt).
- Not Track B's bit-trick proof (`lemma_decompose_spec_eq_decompose`)
  — defer to USER-lane sprint.
- Not the BitVecEq extension for gamma1 offset packing — its own
  session.

## End-of-session expectations

- 5 baseline errors all closed.
- 0 F* errors / 0 make-level failures.
- One commit on `ml-dsa-proofs` with the changes (preamble above).
- Track D-2 prompt is the next focused task — direct the user to
  `proofs/step-14-track-d-2-prompt.md` as the natural follow-on.

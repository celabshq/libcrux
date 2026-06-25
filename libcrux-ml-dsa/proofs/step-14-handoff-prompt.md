# Step 14 — Handoff after Step 13 close + c6c68bbca propagation gap

Resuming the ML-DSA below-trait proof lane on branch `ml-dsa-proofs`
in `/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa`.

## What landed in Step 13

Many commits between `52657214a` (Step 13 handoff prompt) and
`c6c68bbca` (current tip).  Highlights:

- `027fc57b5` Track A close + Track D-1 t1/error close + F-3 mirror
  (5 admits removed: Track A lemma, Portable t1/error, AVX2 t1/error
  trait bodies).
- `efc4a1fc6` Cherry-pick F-6 (t0_serialize trait pre swap to centered
  `is_i32b_array_opaque (pow2 12)`).
- `8b439d957` Cherry-pick F-7 (commitment/gamma1/t0 *_serialize trait
  pres tightened to `pow2 d - 1`).
- `3cf230cbe` Track A-deferred — AVX2 gamma1_serialize trait body
  close (1 admit removed).
- `a30402f11` F-6/F-7 mirror — drop Portable commitment / Portable
  gamma1 / Portable t0 admits, file F-8/F-9/F-10/F-11.
- `b21165f79` Below-trait F-12 mirror — rejection_sample_* impl posts
  add length-preservation.
- `c6c68bbca` Below-trait F-8/F-9/F-10/F-11 mirror — half-open
  `(-l, l]` predicate batch.  **This commit has a propagation gap; see
  below.**

Empirical at `a30402f11` (post-F-6/F-7 mirror): **75 modules invoked,
[CHECK]=27, [ADMIT]=48, 0 F* errors, 0 make-level failures**.

## Critical: c6c68bbca propagation gap

Commit `c6c68bbca` updated the trait pres/posts for F-8/F-9/F-10/F-11
to use the new `Spec.Utils.is_i32b_strict_lower_array_opaque (l: nat)`
predicate (= `forall i. -l < v x[i] /\ v x[i] <= l`, half-open
lower).  But it did NOT propagate the change down to the impl-side
helper functions or the `reveal_opaque` calls inside trait method
bodies.

**Concrete gaps** (all in `src/simd/{portable,avx2}.rs`):

1. **`power2round_with_proof` ensures**:
   - Current: `Spec.Utils.is_i32b_array_opaque (pow2 12) t0_future`
     (closed)
   - Required: `Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12)
     t0_future` (half-open) to discharge the new trait post.
   - Lines: `portable.rs:127`, `avx2.rs:94`.

2. **`decompose` trait method body's `reveal_opaque` calls**:
   - Current: reveals `is_i32b_array_opaque 95232 low` and
     `is_i32b_array_opaque 261888 low`.
   - Required: reveal the strict-lower variants since the post now
     uses them.
   - Lines: `portable.rs:314,320`, `avx2.rs:458,464`.

3. **`Hacspec_ml_dsa.Commute.Chunk.lemma_decompose_lane_commute_conditional`**:
   - Current produces the closed bound on r0 (low value).
   - Required: produce the half-open `(-γ2, γ2]` bound (matching FIPS
     204 Algorithm 36 mathematically).
   - Estimated: extending the existing `lemma_decompose_bridge` to
     drop the `>= -γ2` boundary case (since `-γ2` is only achievable
     by the `r' - r1 * 2γ2 = -γ2` boundary which Hacspec's centered
     `mod_pm` excludes).

4. **`Hacspec_ml_dsa.Commute.Chunk.lemma_power2round_t1_bound` /
   `lemma_power2round_lane_commute`**:
   - Need a half-open variant or a new `lemma_power2round_t0_strict_lower_bound`
     that establishes `-pow2 12 < v t0_future <= pow2 12` strictly.
   - FIPS 204 Algorithm 35 produces `r0 ∈ (-pow2 (d-1), pow2 (d-1)]`
     so the bound is mathematically true.

5. **AVX2 `decompose` trait body**: at `Avx2.fst:654` Z3 cancels
   trying to prove the trait post.  Same gap as above propagating.

**Empirical baseline at `c6c68bbca`** (verified Step 14 entry):
**5 F* errors, 0 make-level failures**.  Specifically:
- `Libcrux_ml_dsa.Simd.Portable.fst:579-621` — Could not prove
  `decompose`'s strict-lower post-condition (Portable trait body).
- `Libcrux_ml_dsa.Simd.Avx2.fst:654-729` — Z3 canceled at rlimit 80
  on AVX2 `decompose` trait body sub-query.
- `Libcrux_ml_dsa.Simd.Avx2.fst:988-1000` — incomplete quantifiers on
  AVX2 `power2round_with_proof` call site (the helper ensures uses
  closed `is_i32b_array_opaque (pow2 12)` while the trait post wants
  half-open).
- `Libcrux_ml_dsa.Simd.Avx2.fst:1261` (×2) — incomplete quantifiers
  on AVX2 `t0_serialize` call site.  The `is_i32b_strict_lower_array_opaque`
  trait pre needs an explicit `reveal_opaque` in the trait body to
  expose the half-open bound to the AVX2 free fn pre's `to_i32x8`-shape
  unfolding.

**Recommendation**: the c6c68bbca propagation gap should be closed
**before** any further below-trait deserialize work.  Open question:
is this fix above-trait-owned (since the original F-9/F-11 lemma
enhancements logically belong with their predicate), or below-trait?
**Recommendation: below-trait**, because:
- The `Hacspec_ml_dsa.Commute.Chunk.fst` lemmas live in the below-trait
  worktree (style guide §9.2 — "develop locally first").
- The `*_with_proof` helpers in `simd/{portable,avx2}.rs` are
  below-trait code.
- The fix is a one-day proof effort (extend two commute lemmas + swap
  the predicates in the helper ensures + reveal calls).

## Hard rules (binding, inherited)

1. **Do not edit `src/simd/traits.rs` or `src/simd/traits/specs.rs`
   pre/post conjuncts.**  Owned by the above-trait lane.  Cherry-pick
   only.  Trait sha is `c6c68bbca` (this lane) ↔ `a4fff81fe`
   (above-trait).
2. **Mismatches go into `lane-split-protocol.md`** under "Open
   findings".  Resolved findings: F-1, F-2, F-3, F-4, F-12.  Open: F-5,
   F-6, F-7, F-8, F-9, F-10, F-11 (cherry-picked but propagation gap
   still open below-trait).
3. **20-min wall-clock per method per impl.**  On overrun, mark
   admitted, document, move on.
4. **Develop locally, upstream specs once** (style guide §9.2): new
   spec/math lemmas go in
   `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
   first.
5. **rlimit ≤ 200** for new commute lemmas; ≤ 300 for impl method
   bodies.

## Pre-flight

```bash
cd /Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa
git rev-parse HEAD                                # tip: c6c68bbca
git status                                        # working tree may have stashes
git stash list                                    # check for in-flight Track D-2 t1_deserialize work
proofs/agent-status/touch-unchanged-checked.sh snapshot
JOBS=2 ./hax.sh extract                           # confirms unchanged
proofs/agent-status/touch-unchanged-checked.sh skip-unchanged
JOBS=2 ./hax.sh prove 2>&1 > /tmp/prove-baseline.log  # confirm errors
grep -E "^\* Error |F\* errors|Make-level" /tmp/prove-baseline.log
```

Expected: ≥ 2 F* errors at the locations above, 0 make-level failures.

## Tracks (priority order)

### Track 0 — c6c68bbca propagation gap close (~2 hours)

This is the **gating** task — do NOT attempt any other Track until
this is closed.

**Subtask 0.1**: extend `lemma_decompose_lane_commute_conditional` in
`Hacspec_ml_dsa.Commute.Chunk.fst` to also produce the strict-lower
`(-γ2, γ2]` bound on the low/r0 output.  Mathematically true (FIPS 204
mod_pm semantics), should be a small addition to the existing proof.

**Subtask 0.2**: extend `lemma_power2round_lane_commute` (or add a
companion `lemma_power2round_t0_strict_lower_bound`) for the
`(-pow2 12, pow2 12]` bound on t0_future.

**Subtask 0.3**: update `power2round_with_proof` ensures in
`portable.rs:127` and `avx2.rs:94` to use
`is_i32b_strict_lower_array_opaque (pow2 12)`.  Update the proof body
to call the new commute lemma (Subtask 0.2).

**Subtask 0.4**: update `decompose` trait method body's
`reveal_opaque` calls in `portable.rs:314,320` and `avx2.rs:458,464`
from `is_i32b_array_opaque` to `is_i32b_strict_lower_array_opaque`.
Update the body's commute lemma call (or follow-up reveal) to chain
through Subtask 0.1.

**Subtask 0.5**: full extract + prove.  Confirm 0 errors.  Empirical
baseline target: 75 modules invoked, [CHECK]=27, [ADMIT]=48, 0 F*
errors, 0 make-level failures (same as `a30402f11`).

**Acceptance**: `JOBS=2 ./hax.sh prove` exits with 0 errors.

### Track D-2 — Deserialize trait bodies (~3-4 hours after Track 0)

Once Track 0 is closed, resume Track D-2 from the Step 13 handoff:

**D-2 priority**:
1. **t1_deserialize** (~1 hr) — Portable + AVX2.  WIP in
   `git stash@{0}` from the previous session.  The Portable proof
   needs an SMT bit-vec hint on `(x & (pow2 10 - 1))` to derive
   `< pow2 10`.  Look at `i32_bit_zero_lemma_to_lt_pow2_n_weak` in
   `fstar-helpers/fstar-bitvec/`.  AVX2 free fn ensures uses
   `j % 32 > 10 ==> Bit_Zero` — needs strengthening to `>= 10` to
   give value < pow2 10 (or add a separate value-bound clause).
2. **error_deserialize** (~45 min) — Portable + AVX2.  Trait post is
   per-eta `forall i. -eta ≤ v out[i] ≤ eta`.  Body is mask + ETA - x
   form; bound provable.
3. **t0_deserialize** (~1 hr) — Portable + AVX2.  Trait post is
   `is_i32b_strict_lower_array_opaque (pow2 12)`.  Need free fn
   ensures matching the strict-lower form.  Body is mask + GAMMA1 -
   coefficient form.
4. **gamma1_deserialize** (~1 hr) — Portable + AVX2.  Trait post is
   `is_i32b_array_opaque (pow2 d)` (closed; F-N follow-up may tighten
   to strict-lower for round-trip symmetry).  AVX2 has strong
   existing post; needs bridge.  Portable needs new ensures.

### Track B math (USER lane, deferred)

`lemma_decompose_spec_eq_decompose` in `Commute.Chunk.fst` — single
math admit pending the bit-trick proof for magic constants 11275
(γ2=95232) and 1025 (γ2=261888).  ~150-200 lines.  Better suited to a
focused USER-lane sprint than a tail of Step 14.

### Track C (stretch)

AVX2 `compute_hint` / `use_hint` / `decompose` impl bodies — needs
strengthening of AVX2 free-fn posts via `Spec.Intrinsics` SMTPats.
Substantial — likely 200+ lines of new SMTPat axioms.  Skip unless
Track 0 + D-2 finish well within budget.

## What this is NOT

- Not a trait pre/post change.  Owned by above-trait lane.
- Not a `Specs.fst` edit.  Cherry-pick only.
- Not Track B's bit-trick proof — defer to USER lane.
- Not the BitVecEq extension for gamma1 offset packing — its own
  session.

## End-of-session expectations

- Track 0 fully closed (c6c68bbca propagation gap).
- At least 1-2 deserialize methods closed (t1 + error preferred).
- 0 F* errors / 0 make-level failures, baseline at or above 27 [CHECK].
- `MLDSA_STATUS.md` and `outstanding-admits.md` synced with progress.

## Files to read first

1. `libcrux-ml-dsa/MLDSA_STATUS.md` — current baseline + per-method
   status table.
2. `libcrux-ml-dsa/proofs/agent-status/lane-split-protocol.md` —
   F-1 through F-12 history; resolution status.  Note F-8/F-9/F-10/F-11
   are cherry-picked but propagation incomplete.
3. `libcrux-ml-dsa/proofs/outstanding-admits.md` — "Active admits"
   section.
4. `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
   — `lemma_decompose_lane_commute_conditional` and
   `lemma_power2round_lane_commute` are the lemmas needing the
   strict-lower bound additions (Track 0 subtasks 0.1/0.2).
5. `libcrux-ml-dsa/src/simd/traits/specs.rs` — `is_i32b_strict_lower_array_opaque`
   predicate added by `c6c68bbca` cherry-pick.  Lookup/intro lemmas
   should be there.
6. `libcrux-ml-dsa/src/simd/portable.rs:124-160` — Portable
   `power2round_with_proof` body (Track 0 subtask 0.3 site).
7. `libcrux-ml-dsa/src/simd/avx2.rs:90-142` — AVX2 `power2round_with_proof`
   body (Track 0 subtask 0.3 site).
8. `libcrux-ml-dsa/src/simd/portable.rs:300-330` — Portable
   `decompose` trait body's reveal calls (Track 0 subtask 0.4 site).
9. `libcrux-ml-dsa/src/simd/avx2.rs:445-475` — AVX2 `decompose` trait
   body's reveal calls (Track 0 subtask 0.4 site).
10. `libcrux-ml-dsa/proofs/agent-status/touch-unchanged-checked.sh` —
    USE THIS for the iteration loop (saves ~10 min per iteration).
11. `git stash@{0}` — Track D-2 t1_deserialize WIP from the previous
    session.  Pop after Track 0 lands.
12. `git log --oneline -10` for recent context.

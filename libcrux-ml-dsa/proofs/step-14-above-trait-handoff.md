# Above-trait Step 14 fresh-session handoff

You are continuing the libcrux ML-DSA above-trait verification on
branch `ml-dsa-above-trait`, worktree `~/libcrux-ml-dsa-above-trait/`.

Verify state:

```
cd ~/libcrux-ml-dsa-above-trait && git log -1 --oneline
```

Should show **`341a93d4d above-trait: F-13 — revert F-11 decompose
strict-lower (math is FALSE)`**.  If not, run `git fetch --all` and
`git checkout ml-dsa-above-trait`.

The sibling below-trait worktree at `~/libcrux-ml-dsa-proofs/` is on
branch `ml-dsa-proofs` (read-only reference for cross-lane diffing).

## Recent above-trait commits (since 22460d2b5 baseline)

```
341a93d4d above-trait: F-13 — revert F-11 decompose strict-lower (math is FALSE)
4bcc40005 above-trait: generate_key_pair length-preservation ensures
a4fff81fe above-trait: F-8/F-9/F-10/F-11 half-open (-l, l] predicate batch
216cbf17e above-trait: rejection_sample_* trait posts add length-preservation
926009f33 above-trait: Hash_functions Xof length-preservation ensures
01bc041ab above-trait: lane-split-protocol — mark F-6 / F-7 RESOLVED
2fd479fc3 above-trait: F-7 tighten *_serialize trait pres to pow2 d - 1
040d17c87 above-trait: F-6 t0_serialize trait pre swap to centered i32b
56075c4ee above-trait: outstanding-admits — diagnose 2026-04-29 closure attempt
a3a1c59bf above-trait: Polynomial::zero gains is_i32b_array_opaque 0 ensures
9c2fd2523 above-trait: arithmetic::shift_left_then_reduce gains FIELD_MAX ensures
09d0743d3 above-trait: Sample.fst close 5 of 10 body admits
```

## Open above-trait work (priority order)

### 1. Sample.fst remaining 5 admits — IN-PROGRESS ATTEMPT EXISTS

`src/sample.rs` has 5 body admits remaining:
- `sample_up_to_four_ring_elements_flat`
- `sample_four_error_ring_elements`
- `sample_mask_ring_element`
- `sample_mask_vector`
- `sample_challenge_ring_element`

A prior agent (`ab0453d4e1ab9d850`) attempted closure but hit the
Anthropic rate limit before the F\* `make` finished.  Its source edits
are saved at `/tmp/sample-retry-attempt.patch` (216 lines, modifies
`libcrux-ml-dsa/src/sample.rs` only — adds pre/post + loop_invariants
to the 5 functions).  The agent's `AGENT_STATUS.md` (in
`/Users/karthik/libcrux/.claude/worktrees/agent-ab0453d4e1ab9d850/`)
reported "all admits removed; pres/posts/loop_invariants in place;
ETA 30-90 min for the make".  No verification ran to completion, so
the edits are **untrusted**.

**Two options for this session:**

**Option A (audit + verify)**: apply `/tmp/sample-retry-attempt.patch`,
run `cd libcrux-ml-dsa && ./hax.sh extract && cd
proofs/fstar/extraction && make
$(git rev-parse --show-toplevel)/.fstar-cache/checked/Libcrux_ml_dsa.Sample.fst.checked`,
audit any failures.  Risk: edits may have proof gaps that surface
deep in the make.

**Option B (clean restart)**: discard the patch, follow the recipe
below from scratch.  Probably faster than auditing.

**Recipe** (from Agent B's worked example, commit `09d0743d3`):

1. Convert any remaining `cloop! { ... }` loops to plain `for i in
   0..N { ... }` so `loop_invariant!` attaches.
2. Pre: input lengths, divisor non-zero where used, alignment with
   chunk size.
3. Post: length-preservation on output buffers + bounds-only on
   sampled coefficients (intentional per `outstanding-admits.md` —
   full bound-on-rejection-sample is below-trait's job).
4. Xof posts give `Seq.length state.out_future == BLOCK_SIZE` (or
   similar literal); use these to discharge slicing operations
   (`randomness[..N]`, `state.absorb_final(seed)`,
   `state.squeeze(&mut buf)`).
5. **20-min budget per function.**  If a body proof doesn't close in
   20 min, restore the admit, document in
   `proofs/outstanding-admits.md`, move on.

Hash_functions Xof methods already have length-preservation ensures
(`926009f33`).  `Spec.Utils.is_i32b_array_opaque`,
`is_pos_array_opaque`, `is_i32b_strict_lower_array_opaque`,
`Polynomial.Spec.is_bounded_poly` all available.

### 2. F-13 follow-up cherry-pick coordination

The `decompose` portion of F-11 was reverted (`341a93d4d`) because the
strict-lower bound is mathematically FALSE — FIPS 204 Algorithm 36's
special-case adjustment `r0 ← r0 - 1` allows `r0 = -γ2`.  Below-trait
filed F-13 in `proofs/agent-status/lane-split-protocol.md` and is
currently working through Track 0; the revert here unblocks the
below-trait closed-bound proof body.

No action needed on this lane unless below-trait files a new
finding.  But: confirm `341a93d4d` is reflected in
`lane-split-protocol.md` here (it should be — it was committed
together).

### 3. Matrix.fst three wrappers — DEFERRED

`compute_as1_plus_s2`, `compute_matrix_x_mask`, `compute_w_approx`
in `src/matrix.rs` still body-admitted.  Agent A's prior attempt
landed scaffolding (`shift_left_then_reduce` polynomial ensures,
`Polynomial::zero` ensures) but didn't close the wrappers; pre-existing
Z3 flake on `add_vectors`/`subtract_vectors` query 56 (rlimit 800,
~125s) blocks debugging.  Skip in this session unless explicitly
prioritized.

### 4. F-4 — `compute_hint_lane_post` real spec gap

`lemma_compute_hint_lane_commute_conditional` is `admit ()`'d because
Spec `compute_one_hint` and Hacspec `make_hint` diverge at `low =
-γ2, high ≠ 0`.  Real spec divergence — needs an above-trait decision
before any below-trait work proceeds (cite Spec form directly, or
strengthen Hacspec).  Not urgent; defer to a discussion with the
below-trait owner.

### 5. F-13 candidate (`make_hint` Z3 failures)

Pre-existing 3 errors in `Arithmetic.fst` at lines 620 / 661 / 662
(`true_hints +! one_hints_count` usize-overflow in inner loop).
Independent of half-open work.  Lower priority.

## Below-trait state (read-only context)

Below-trait branch tip: `a5d7e58cb` (Track 0 prompt enrichment) plus
in-progress Track 0 work in working tree (uncommitted; another session
owns it).  The fresh below-trait session is closing the `c6c68bbca`
propagation gap and adapting to the F-13 revert.

## Hard rules

1. **Do not edit `src/simd/portable*` or `src/simd/avx2*`** — owned by
   below-trait.  This is the lane-split protocol.
2. **Trait pre/post changes in `src/simd/traits.rs` are above-trait
   territory** — that's this session.  Document each in
   `proofs/agent-status/lane-split-protocol.md` so below-trait can
   cherry-pick.
3. **20-min wall-clock budget per proof attempt.**  Admit + document
   on overrun.
4. **Spawned subagents must write status reports every 15 minutes** to
   `<worktree>/AGENT_STATUS.md` — see user memory
   `feedback_agent_status_reports.md`.  Required per recent loss of
   3 agent-hours to Anthropic-rate-limit silent failure.

## Useful pointers

- `proofs/outstanding-admits.md` — current admit inventory, recipes,
  design decisions.
- `MLDSA_STATUS.md` — module-level CHECK/ADMIT status, Phase tracker.
- `proofs/agent-status/lane-split-protocol.md` — F-1 through F-13
  history.

## User memory (read for context)

- `~/.claude/projects/-Users-karthik-libcrux/memory/feedback_lane_split.md`
- `~/.claude/projects/-Users-karthik-libcrux/memory/feedback_no_cache_nuke.md`
- `~/.claude/projects/-Users-karthik-libcrux/memory/feedback_use_fstar_mcp.md`
- `~/.claude/projects/-Users-karthik-libcrux/memory/feedback_hax_modulo_euclidean.md`
- `~/.claude/projects/-Users-karthik-libcrux/memory/feedback_agent_status_reports.md`

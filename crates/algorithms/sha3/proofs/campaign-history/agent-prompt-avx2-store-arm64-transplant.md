# Prompt — close AVX2 `store_block` via Arm64 transplant

## Mission

Close the `hax_lib::fstar!("admit()")` admit on AVX2 `store_block`
(`crates/algorithms/sha3/src/simd/avx2/store.rs:74`) by **mirroring the
Arm64 proof pattern** that landed cleanly in commits `c14f94d2c` and
`83d1a04c2`: factored wrapper with strong per-byte ensures + bridge
lemmas in `StoreBlockHelpers.fst` + terse 4-conjunct per-byte loop
invariant, all under the same hax options that worked for Arm64.

This is a **transplant** sprint, not a research sprint. The proof shape
is known to work (Arm64 ships it). The risk is in the AVX2-specific
deinterleave helpers; everything else is mechanical.

## Why this strategy (not opacify)

The previous attempt (`avx2-store-opacify`, deleted 2026-05-08) tried
AlgoStar Technique 4 — opaque predicates over the loop invariant. It
broke the q213 k!61 cascade in `store_block_full` but left two new
admits in the bridge lemmas (`byte_inv_full_step_from_wrapper_post`
unprovable as stated; signature surgery rejected as too fragile) plus a
new admit on the composer. Net 1 cliff broken vs. 3 admits added —
agreed-as-failed.

The diagnosis from `proofs/squeeze-cascade-profile.md` and the prior
agent's smtprofiling: Z3 saturated NOT on the output forall shape (which
opacify hid) but on the *input* reasoning — which deinterleaved lane
(`mm256_permute2x128` + `mm256_unpacklo/hi`) maps to which output
buffer. Hiding the conclusion didn't help.

Arm64 does NOT opacify. Its wrapper `store_u64x2x2` has a strong
per-byte ensures (src/simd/arm64/store.rs:36-57) and the loop invariant
is 4 forall conjuncts. Z3 chews it directly at rlimit 800. Same shape
should work for AVX2.

## Setup

```
cd /Users/karthik/libcrux-sha3-focused
git worktree add -b avx2-store-arm64-transplant \
    /Users/karthik/libcrux-avx2-store-transplant sha3-proofs-focused
```

Use `EnterWorktree path=/Users/karthik/libcrux-avx2-store-transplant`
immediately. NEVER cd into siblings.

Cherry-pick the AVX2 structural split (3 wrappers + helpers module are
already verified clean):

```
git -C /Users/karthik/libcrux-avx2-store-transplant cherry-pick 4bbe9b667
```

After cherry-pick: `store_block_full` and `store_block_tail` exist with
`--admit_smt_queries true` on each. Wrappers `store_u64x4x4`,
`store_chunk8`, `store_chunk_ragged` are clean.

## Required reading (no skipping)

1. **`fstar-for-libcrux` skill** — invoke before F* edits.
2. **Arm64 `store_block_full` proof — the template**:
   - `crates/algorithms/sha3/src/simd/arm64/store.rs:400-671` (function
     definition, hax options, loop invariant, ensures).
   - `crates/algorithms/sha3/src/simd/arm64/store.rs:31-131`
     (`store_u64x2x2` wrapper — STRONG per-byte ensures + bridge body).
   - `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst`
     (the bridge lemmas: `store_block_window_byte_of_vst` and friends).
3. **AVX2 `load_block` cascade closure (commit `7bb581f8b`,
   `8203c9ace`, `28db4222a`, `3b9fc054c`)** — established the SMTPat
   pattern for AVX2 lane reasoning. Your deinterleave-semantics helpers
   should follow this style.
4. **The deleted opacify attempt's diagnostic data**:
   `git -C /Users/karthik/libcrux-sha3-store-avx2-discharge show 464a9914a -- crates/algorithms/sha3/proofs/agent-status/avx2-cliff-profile-progress.md`
   — k!61 dominant @ 1.19M instantiations on q213. This is the cascade
   you need to break, but the strategy is *factor it into the wrapper*,
   not opacify the goal.

## Plan

### Stage 0 — sanity probe (5 min)

Drop `--admit_smt_queries true` from `store_block_full`'s
`hax::fstar::options` (in `simd/avx2/store.rs`), re-extract, run
`make check/Libcrux_sha3.Simd.Avx2.Store.fst > /tmp/avx2-stage0.log 2>&1`,
grep for cliff. Expected: same q213-equivalent cliff at rlimit 400. If
no cliff, ship and stop. (Sanity probe only — the deduped main is on
this branch but the opacify sprint already confirmed cliff persists.)

### Stage 1 — strengthen `store_u64x4x4` wrapper ensures (~60-90 min)

The wrapper currently lives in `simd/avx2/wrappers.rs` (per cherry-pick
of `4bbe9b667`). Its current ensures is in case-split form (one branch
per `(j-start-32i)/8 ∈ {0,1,2,3}` against `s_0..s_3`). Replace with
**direct per-byte form** matching Arm64's `store_u64x2x2`:

```
ensures (forall (K: nat{K < 4}) (j: nat).
  start <= j /\ j < start + 32 ==>
    Seq.index out_K j ==
    Seq.index
      (Core_models.Num.impl_u64__to_le_bytes
         (get_lane_u64 (Seq.index s ((j - start) / 8)) (mk_usize K)))
      ((j - start) % 8))
```

The deinterleave ops (`mm256_permute2x128_si256` +
`mm256_unpacklo_epi64` / `mm256_unpackhi_epi64`) need to be discharged
INSIDE the wrapper. The body proof is sized O(4) per iteration — Z3
should handle it at rlimit 400 with the right helpers in scope.

Add helpers in a new or existing `Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.fst`:
- `lemma_permute2x128_then_unpacklo_lane_lo` and `_hi` — relate the
  permute+unpack output lanes to the input vector lanes. Tight SMTPat.
  These are the AVX2 analogue of Arm64's vtrn1/vtrn2 lemmas.
- `lemma_storeu_si256_per_byte` — relate `mm256_storeu_si256_u8`'s
  output to per-byte `to_le_bytes` extraction from the input vector.
  May already be in `StoreBlockHelpers.fst` from the cherry-pick;
  strengthen if needed.

Verify wrapper alone: `make check/Libcrux_sha3.Simd.Avx2.Wrappers.fst`
clean. Expected per-iteration cost: O(4) sub-queries × ~50ms each.

### Stage 2 — `store_block_full` body discharge (~60 min)

With the strong wrapper ensures from Stage 1, the loop invariant should
be 4 per-byte forall conjuncts (one per output buffer):

```
loop_invariant (forall (j: nat).
  start <= j /\ j < start + 32 * i ==>
    Seq.index out_K j ==
    Seq.index
      (Core_models.Num.impl_u64__to_le_bytes
         (get_lane_u64 (Seq.index s ((j - start) / 8)) (mk_usize K)))
      ((j - start) % 8))
```

(× 4 buffers, plus length/frame conjuncts.)

After each `store_u64x4x4` call in the loop body, raw F* call to a
helper that extends the invariant from `i` to `i+1` (Arm64 pattern,
src/simd/arm64/store.rs:78-130):

```
hax_lib::fstar!(r#"
  Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.byte_window_extend_x4
    s start i ${out0}_old ${out0} ...;
  Classical.forall_intro (Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.byte_window_extend_x4_forall_lemma s start i ${out0}_old ${out0} ...)
"#);
```

(adapt to actual hax syntax; see Arm64 store.rs:78-130 for the verbatim
form).

Drop `--admit_smt_queries true` from `store_block_full`'s
`hax::fstar::options`. Use Arm64's main loop options:
`--z3rlimit 800 --split_queries no --z3refresh --using_facts_from '*
-Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid
-Core_models.Num.impl_u32__rem_euclid'`.

Re-extract, `make check/Libcrux_sha3.Simd.Avx2.Store.fst`. Target: clean
verification, max sub-query <2s (Arm64 max is ~1s).

### Stage 3 — `store_block_tail` (~45 min)

Same pattern, scaled to the chunks8 (8-byte windows) + ragged tail.
Mirror Arm64's `store_tail_high` / `store_tail_low` (arm64/store.rs:150-398).
Helpers go in `StoreBlockHelpers.fst`.

### Stage 4 — composer `store_block` (~15 min)

Once both halves are clean with old-form per-byte ensures, the
composer's WP between them should not cliff. Drop the admit; if Z3
struggles, factor a length/start arithmetic lemma.

### Stage 5 — verify & commit

`make check/Libcrux_sha3.Simd.Avx2.Store.fst` end-to-end clean.
Optional: full `make` of the SHA-3 tree to catch downstream regressions.
If any query >5s, capture qi.profile (cheap future-cliff insurance).
Commit on the branch. Do NOT push.

## Operational rules (non-negotiable, from auto-memory)

- `feedback_smtprofile_before_negative` — no negative report without
  qi.profile. The k!61 cascade baseline is on q213 of `store_block_full`
  pre-Stage-1.
- `feedback_max_4_fstar_per_agent` — ≤4 concurrent fstar+z3.
- `feedback_grep_make_output` — pipe to /tmp logs and grep. Never Read
  full F* logs. The opacify attempt's third agent died on a stream-idle
  timeout from a large Read.
- `feedback_use_fstar_mcp` — fstar-mcp `typecheck_buffer` for
  sub-second iteration.
- `feedback_fstar_mcp_session_dies_after_make` — recreate session
  after each make.
- `feedback_per_stage_clean_rebuild` — between stages, `rm` only the
  touched modules' `.checked` and re-make.
- `feedback_rlimit_cap_800` — ≤800 mono / ≤400 split. Arm64's rlimit
  800 with `--split_queries no` is the budget; do not exceed.
- `feedback_no_manual_edits_extracted` — ALL spec/proof changes go
  through Rust source + `bash hax.sh extract`. The opacify attempt's
  patch-script approach is FORBIDDEN here. The handwritten
  `Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.fst` is fine; everything
  else must be hax-generated.
- `feedback_no_code_changes_for_proofs` — annotation/proof scaffolding
  changes ARE authorized; semantic impl changes (e.g. reshuffling
  permute2x128 calls to make the proof easier) need user sign-off.
- `feedback_branch_means_worktree` — already enforced by the
  `git worktree add` setup above.
- `feedback_bisect_before_blame` — if a previously-passing module
  regresses, run the same make at the parent commit before claiming
  regression.
- `feedback_agent_status_reports` — every 15 min to
  `crates/algorithms/sha3/proofs/agent-status/avx2-store-arm64-transplant-progress.md`:
  timestamp, stage, sub-task, blocker, ETA.

## File boundaries

You own:
- `crates/algorithms/sha3/src/simd/avx2/store.rs` (loop invariants,
  ensures, drop admits).
- `crates/algorithms/sha3/src/simd/avx2/wrappers.rs` (strengthen
  `store_u64x4x4` ensures; do NOT change wrapper body semantics).
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.fst`
  (handwritten — direct edit allowed; the file already exists with 2
  `assume val` axioms about `from_le_bytes`/`to_le_bytes` inverse,
  which are presumed stable).
- `crates/algorithms/sha3/proofs/agent-status/avx2-store-arm64-transplant-*.md`.

You may touch (with surface-to-user first):
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.Store.fst`
  — only for probing during fstar-mcp loops; the canonical source is
  `store.rs` + `bash hax.sh extract`.

You do NOT touch:
- AVX2 Load files (closed via `7bb581f8b` / `8203c9ace` / `28db4222a` /
  `3b9fc054c`).
- Arm64 anything (the template — read-only reference).
- `simd/portable.rs`, `EquivImplSpec.*`, `specs/sha3/*`,
  `Libcrux_intrinsics.*`.

## Budget

60-min cap LIFTED. Stop on two consecutive 30-min cycles with no
measurable progress. Each stage commits before moving on so a timeout
leaves stable state.

Realistic estimate: Stage 1 (60-90m) + Stage 2 (60m) + Stage 3 (45m) +
Stage 4 (15m) + Stage 5 (30m) = 4-5 hours focused. The Arm64 sprint
took ~6 hours including helper iteration; AVX2 should be similar
because the deinterleave helpers may need 2-3 SMTPat tweaks.

## Skills

`fstar-for-libcrux` (mandatory before F* edits), `proofdebugging`
(when a stage fails), `smtprofiling` (mandatory before any negative
result; baseline k!61 @ 1.19M on q213 of pre-Stage-1 `store_block_full`).

## Deliverables

Commit on `avx2-store-arm64-transplant` branch (do NOT push):

- **Best**: `store_block_full`, `store_block_tail`, and `store_block`
  composer all admit-free; full `Libcrux_sha3.Simd.Avx2.Store` module
  verifies clean. Per-stage commits. Helpers documented in
  `StoreBlockHelpers.fst`. No new admits anywhere.
- **Partial**: Stage 1 (strong wrapper ensures) + Stage 2
  (`store_block_full`) closed; Stage 3/4 deferred. Per-stage commits +
  smtprofiling diagnosis if any stage cliffed.

## Final report (≤300 words)

(1) Stage 0 result: dedupe-only sanity, did anything close trivially?
(2) Stage 1: wrapper ensures strengthened? Helper lemmas added?
    Per-iteration cost. Wrappers.fst clean Y/N.
(3) Stage 2: `store_block_full` closed Y/N. Final timings, max sub-query.
(4) Stage 3: `store_block_tail` closed Y/N.
(5) Stage 4: composer admit removed Y/N.
(6) Stage 5: full module make clean? End-to-end timing.
(7) New helper lemma signatures + which call sites use them.
(8) For any non-closure: smtprofiling top-5 quantifiers + diagnosis
    (per `feedback_smtprofile_before_negative`). Compare to k!61
    baseline @ 1.19M on pre-Stage-1 q213.
(9) Branch SHA.
(10) Notes for downstream — does the deinterleave-helpers pattern
     unlock anything else (e.g., AVX2 Squeeze)?

## Suggested first 30 min

1. EnterWorktree.
2. Cherry-pick `4bbe9b667`.
3. Read Arm64 `store_u64x2x2` end-to-end (arm64/store.rs:31-131) — this
   is your template.
4. Read Arm64 `StoreBlockHelpers.fst` end-to-end — these are your
   bridge-lemma model.
5. Stage 0 sanity probe (5 min).
6. Sketch the strong `store_u64x4x4` ensures + the deinterleave helper
   lemmas on paper before touching code.
7. First status entry.

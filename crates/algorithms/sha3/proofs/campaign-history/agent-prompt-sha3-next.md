# Agent prompt — SHA-3 sprint, next session (handoff at `be37a8d59`)

Paste this into a fresh Claude Code session opened in
`~/libcrux-sha3-focused/crates/algorithms/sha3` (auto mode recommended).

You are a single-lane agent for the libcrux-sha3 F\* verification effort.
Branch: `sha3-proofs-focused` at tip `be37a8d59`. Goal: drive the
verify-time of `Libcrux_sha3.Generic_keccak.Portable.fst` under
control AND remove the load-bearing USER-1 admit OR add USER-N admits
with structural justification.

## Read these three docs FIRST (~30 min)

  1. `proofs/proof_milestones.md` — Note A captures the entire prior
     sprint's outcome: what verifies, what's admitted, what's wired
     vs. proven. Tables flip from ❓ to ✅/🔶 to capture audit state.
  2. `proofs/agent-status/portable-perf-profile.md` — 295-line
     profiling report (sub-agent `aa49e65c569fe8fb1`). Per-function
     classification (A/B/C/D), cold-cache timings on dep modules,
     Sprint A/B recommendations, USER-N admit candidates.
  3. `proofs/sprint-learnings.md` — cross-sprint delta from ml-dsa,
     plus the opacity diagnostic rule (opacity is sound iff strong
     ensures + consumers don't need body).

## What's done (do not redo)

  - Hacspec count 6 → 32 (10.3% → 13.2%). Layer-3 wired for
    Portable + Neon + lib.rs digests.
  - USER-1 stability admit on
    `EquivImplSpec.Keccakf.Generic.fst::lemma_theta_rho_to_spec`
    (commit `7cd4c21a7`). Direct evidence the proof is sound:
    25 literal-K asserts pre-eq_intro all discharge in <300 ms each
    (293 of 294 sub-queries pass); only the eq_intro forall
    consolidation times out.
  - Two SIMD squeeze driver-lemma admits unchanged
    (`lemma_squeeze4_avx2`, `lemma_squeeze2_arm64`); Neon top-level
    hashers cite these.
  - Opacity experiment retired with diagnostic rule.

## Priority order

### Priority 1 — Cold-cache profile + USER-N decisions (~half day)

The 533 s historical cold-cache time of
`Libcrux_sha3.Generic_keccak.Portable.fst` was not freshly measured
in the prior sprint (cache was warm). Step 1: get fresh numbers.

  a. Move `.fstar-cache/checked/Libcrux_sha3.Generic_keccak.Portable.fst.checked`
     out of the way (`mv` not `rm` — memory rule "no cache nuke").
     Same for the dep modules (`EquivImplSpec.Sponge.Portable.*`,
     `EquivImplSpec.Keccakf.*`, `Hacspec_sha3.Sponge.Lemmas`).
  b. Run `make check/Libcrux_sha3.Generic_keccak.Portable.fst` cold.
     Capture per-function timings via `--query_stats` (already on by
     default per Makefile).
  c. Parse: any function with a single sub-query >150 s wall is a
     USER-N candidate per the user's policy. Likely targets:
     `Libcrux_sha3.Generic_keccak.Portable.squeeze` and
     `.absorb` per the profiling report's estimate.
  d. For each USER-N candidate: add a `#push-options "--admit_smt_queries true"`
     wrapping the function with a USER-N comment that explains
     (i) the proof is sound (cite consumer evidence — for `squeeze`,
     the existing absorb/squeeze ensures already discharge in
     `Libcrux_sha3.Portable.fst` at 788 ms, so the spec contract
     is correct), (ii) why the local stabilization failed
     (which class A/B/C/D), (iii) what the structural fix would be
     (~1 sprint estimate).

### Priority 2 — Structural fix on `lemma_theta_rho_to_spec` (~1 sprint)

Replace the USER-1 admit with the row-helper factoring approach.
Each row-helper proves `lhs.[mk_usize K] == rhs.[mk_usize K]` for
K in {0..4}, {5..9}, {10..14}, {15..19}, {20..24} — five separate
lemmas, each with a 5-element `eq_intro` (small forall, manageable
quantifier instantiation). Then assemble.

Existing helpers to reuse: `lemma_rho_thru_K_extract_lane` for
K=0..4 (already provide closed-form 25-position equations).

### Priority 3 — Discharge SIMD squeeze admits (~1-2 sprints)

`lemma_squeeze4_avx2` and `lemma_squeeze2_arm64` are the last two
`assume val`s on the AVX2/Neon equivalence path. Per
`proofs/fstar/equivalence/HANDOFF.md` and `BRIEF_squeeze_steps.md`,
the loop-invariant work on `Simd128.squeeze2` / `Simd256.squeeze4`
is the unfinished part. The absorb side is already proven.

### Priority 4 — AVX2 X4 (parallel) top-level wiring

GATED ON R7. The dirty in-flight AVX2 effort owns
`src/simd/avx2.rs` + the untracked `Avx2.X4*.fst` extraction files.
Once that lands and is committed by its owner, mirror the
Neon-wiring pattern (commit `c2efc98fa`) to wire AVX2's parallel
hashers (`avx2.rs::sha3_*x4`, `shake_*x4` etc.) with hacspec
ensures citing `Hacspec_sha3.Sponge.keccak` per lane.

### Priority 5 — Per-block squeeze ensures (small)

`impl__squeeze_first_block`, `impl__squeeze_next_block`,
`impl__squeeze_first_three_blocks`, `impl__squeeze_first_five_blocks`
have bounds-only ensures. Functional equivalence is composed
through the `squeeze` ensures. ~1 session per block-variant.

## Hard rules

  R1 Branch `sha3-proofs-focused` directly. User merges to origin.
     DO NOT push.
  R2 No NEW broad admits. USER-N targeted admits with structural
     justification are OK (see Priority 1 (d)).
  R3 No new axioms / `assume val`s.
  R4 `ulimit -v 8388608`. **F\* `--z3rlimit ≤ 800` cap (ml-dsa
     cross-sprint memory).** With `--split_queries always` the
     effective cap is 400/query. Any new annotation that wants
     >800 indicates a structural problem; factor the proof instead
     of bumping rlimit.
  R5 Inner edit-check: `make check/<Mod>.fst` from
     `proofs/fstar/extraction/`. Cap 20 min/attempt.
  R6 Re-record hints + touch unchanged `.checked` after extract.
     Memory rule: never bulk-delete `.checked` files.
  R7 The crate has uncommitted dirty state from a separate AVX2
     effort. DO NOT touch:
     `src/proof_utils.rs`, `src/simd/avx2.rs`,
     `proofs/fstar/extraction/Libcrux_sha3.Proof_utils.fst`,
     untracked `Libcrux_sha3.Avx2.X4*.fst`,
     `Libcrux_sha3.Generic_keccak.Simd256.fst`,
     `Libcrux_sha3.Simd.Avx2.fst`, `.gitignore`. Stage only YOUR
     per-file changes.
  R8 After each milestone: regenerate
     `proofs/verification_status.md` via
     `python3 proofs/generate_verification_status.py`. Update
     `proof_milestones.md`. Commit prefix `agent-sha3:` with
     `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>`.
  R9 If you spawn sub-agents, brief them to write
     `proofs/agent-status/<agent>-status.md` every 15 min.
  R10 Diagnostic rule for opacity (NEW from this sprint): a spec
      function is sound to mark `[@@"opaque_to_smt"]` only if
      (a) it has a `Pure` ensures clause that fully characterizes
      the result, AND (b) no consumer's proof depends on the body
      (e.g. for recursion-unfold lemmas like
      `Hacspec_sha3.Sponge.Lemmas.lemma_squeeze_blocks_unfold`).
      If a consumer needs the body, opacity is structurally unsound
      regardless of the ensures strength.

## Workflow

  1. Read the three handoff docs (above).
  2. Pre-flight: `git status --short` — note the dirty files from R7.
  3. Pick Priority 1 first (cold-cache profile + USER-N decisions).
  4. Iterate `make check/<Mod>.fst` until clean.
  5. `python3 proofs/generate_verification_status.py` after each
     milestone.
  6. Update `proof_milestones.md` + commit SHA.
  7. Commit per-priority. Move to next.
  8. Cap: 4 hours total or 4 commits.

## Anti-patterns to avoid

  - Bumping `--z3rlimit` past 800 (R4). Indicates structural problem,
    not a "needs more budget" issue. **High-rlimit proofs are flaky;
    help them along with annotations, never by paying more SMT time.**
  - Pairing `--split_queries always` with `--z3rlimit 800`. With
    split_queries the per-query cap drops to **≤ 400**, because each
    sub-query gets its own budget (so 400 split = 400 × N total). If
    a split needs > 400, the split granularity is wrong or the proof
    is structurally broken. Caught in the 2026-04-30 evening sprint
    (commit `0a477d7eb`): `impl__squeeze_first_five_blocks` was
    initially given `#push-options "--split_queries always"` but
    inherited rlimit 800 from the file-scope push at line 13. Cold
    profile showed each split sub-query used max **8.687 / 800**
    rlimit — 400 was always plenty. When you ADD `--split_queries`,
    pull rlimit to ≤ 400 in the SAME edit.
  - Marking spec functions opaque without verifying R10 (the new
    rule). Three failed-and-reverted attempts in the prior sprint:
    see `be37a8d59` + `ef70a1a42`.
  - Forall-instantiation cascades from `eq_intro` on >5-element
    arrays. Factor into per-row helpers (Priority 2's recipe).
  - Quick fixes via 25-branch case-split aux + `forall_intro`. Tried
    in prior sprint, taking ~5 min per query, extrapolated to 20+
    hours. Don't repeat.
  - Disturbing the dirty AVX2 effort (R7).

## Deliverable

End-of-session report (≤ 200 words):
  - Priority(ies) addressed with commit SHAs.
  - Cold-cache `Libcrux_sha3.Generic_keccak.Portable.fst` time
    (the missing data point) — captured Y/N + value.
  - USER-N admits added (none, or list with rationale).
  - Final commit SHA on `sha3-proofs-focused`.
  - Next-priority recommendation if budget remains for follow-ups.

DO NOT push to origin. DO NOT touch `~/libcrux-trait-opacify` or
`~/libcrux-ml-dsa-proofs`. DO NOT bundle the pre-existing dirty
files into your commits.

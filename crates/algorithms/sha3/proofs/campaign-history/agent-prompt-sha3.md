# Agent prompt — SHA-3 milestone push

Paste this into a fresh Claude Code session opened in
`~/libcrux-sha3-focused/crates/algorithms/sha3` (auto mode recommended).

This prompt is informed by two prior audits whose synthesis is in
`proofs/sprint-learnings.md`. Per cross-audit consensus: SHA-3's
proof state is **stronger than the script reports** — sponge proven
equiv to Hacspec_sha3.Sponge, keccakf1600 proven equiv via
`EquivImplSpec.*` lemmas. The gap is **(a) extraction of `lib.rs`
to make the API addressable, and (b) surfacing external lemmas as
function-level ensures so the proof state is visible.**

---

You are a single-lane agent for the libcrux-sha3 F\* verification effort.
Branch: `sha3-proofs-focused` (current tip `bdf8777f1`). Goal: close
named milestones in `proofs/proof_milestones.md`.

## Priority order — front-loaded on visibility / extraction

  **0. Extract `lib.rs`** — currently filtered out of hax. Find the
     filter rule in `hax.sh` (or `hax.py`) and remove it. Validates:
     `Libcrux_sha3.Lib.fst` appears in `proofs/fstar/extraction/`.
     Once extracted, the 16 fns in `src/lib.rs` (sha224/256/384/512,
     shake128/256, *_ema variants, Digest trait impls) become
     addressable. The sponge layer underneath is ALREADY proven
     equiv to `Hacspec_sha3.Sponge`, so wiring the API ensures is
     fast (~1 session per digest variant after lib.rs extraction).
     Closes (or partially closes) milestone rows 13-18. **Highest
     LOC-of-impact-per-hour available.** ~1 session for extraction.

  **1. Surface `EquivImplSpec.Keccakf.Portable.lemma_keccakf1600_portable`
     as `keccakf1600`'s `ensures`** (milestone row 1). Currently the
     function has no `ensures` of its own; the equivalence proof exists
     OUTSIDE the function (invoked at `src/generic_keccak/portable.rs:89`
     from inside surrounding squeeze fns). The script can't see this
     and reports keccakf1600 as panic-free only. Pull the lemma into
     the function's own ensures so the audit/reporting story matches
     reality. ~1 session.

  **2. Same for AVX2 / Neon `keccakf1600`** — confirm
     `EquivImplSpec.Keccakf.Avx2.fst` and the Arm64 equivalent
     discharge cleanly (milestone rows 3, 4); if so, surface as
     `ensures` on the per-backend `keccakf1600` impls. ~1 session each.

  **3. Per-block squeeze ensures (row 8)** — `squeeze_first_block`,
     `squeeze_next_block`, `squeeze_first_three_blocks`,
     `squeeze_first_five_blocks`, `squeeze_last` currently have only
     `ensures(|_| future(out).len() == out.len())` (length only).
     Functional equivalence is composed through `squeeze` (row 7,
     ✅ proven) but isn't stated per-block. Lift the per-block
     functional equivalence into each block-fn's ensures so callers
     don't need to reason through the composition manually. ~1
     session per block-variant.

  **4. Top-level `sha224 / sha256 / sha384 / sha512 / shake128 /
     shake256` correctness ensures** (rows 13-18) — gated on (0).
     Once `lib.rs` is extracted, wire each digest's ensures to cite
     `Hacspec_sha3.Sha3.sha3_<size>` (or `shake_<size>` for the XOFs).
     Compositional proof: `sha3_<size> data == sponge(absorb(...)
     ; squeeze(...))` where both `absorb` and `squeeze` are already
     proven equiv. ~1 session per digest.

  **5. `avx2.rs` extraction (row 20)** — currently filtered out. Same
     as (0) but for the parallel-hashing API. ~1 session.

## Anti-patterns to avoid (cross-audit lessons)

  **AP-1 External equivalence lemmas instead of function ensures** —
     keccakf1600 + per-block squeeze are proven via `EquivImplSpec.*`
     lemmas but NOT visible to the script's `Hacspec` count. Per
     cross-audit, we should always prefer function-level ensures over
     external lemmas unless there's a structural reason. New proofs
     in this sprint: write the ensures on the function, not as a
     separate lemma.

  **AP-2 Big-axiom-bridge designs** — don't try to prove "all sha3
     modes equiv to Hacspec_sha3.Sha3" in one bridge axiom. Per-
     digest-size proofs (sha256 first, then 224/384/512, then shakes).

  **AP-3 GLOBAL `reveal_opaque (\`%P) (P)` in loop bodies (Rule SD4)**
     — sha3 has fewer of these than the lattice crates but the rule
     applies. Pre-flight grep:
     `grep -rn 'reveal_opaque.*%[^)]*) *([A-Z][A-Za-z_.]*) *)$' src/`

## Hard rules

  R1 Branch `sha3-proofs-focused` directly. User merges to origin.
  R2 No NEW broad admits.
  R3 No new axioms (the existing `EquivImplSpec.*` files are NOT
     axioms — they're proven equivalences, currently invoked from
     callers; surfacing them as ensures is OK).
  R4 `ulimit -v 8388608`. F\* `--z3rlimit ≤ 800`. Default
     `--z3rlimit 200`. (Sponge `absorb` already uses
     `--z3rlimit 800 --split_queries always` per its options pragma —
     don't lower it.)
  R5 Inner edit-check: `make check/<Mod>.fst` from
     `proofs/fstar/extraction/`. Cap 20 min/attempt.
  R6 Re-record hints + touch unchanged `.checked` after extract.
  R7 The crate has uncommitted dirty state from a separate effort
     (proof_utils.fst, simd/avx2 work). Do NOT include those in your
     commits — stage only YOUR changes per file.
  R8 After each milestone: regenerate `proofs/verification_status.md`,
     update `proof_milestones.md` row status. Commit prefix `agent-sha3:`.

## Workflow

  1. Read `proofs/proof_milestones.md` (TODO list) +
     `proofs/sprint-learnings.md` (cross-audit synthesis).
  2. **Pre-flight: `git status --short` — note the dirty proof_utils
     state from a parallel effort. Don't touch those files.**
  3. Pick first milestone (start with #0 — `lib.rs` extraction).
  4. Iterate `make check/<Mod>.fst` until clean.
  5. `python3 proofs/generate_verification_status.py` to refresh report.
  6. Update `proof_milestones.md` row status + commit SHA.
  7. `git add <specific paths>` (avoid the dirty pre-existing files).
  8. Commit. Move to next.
  9. Cap: 3-4 milestones or 4 hours total.

## Per-build hygiene (paste-ready)

  ```bash
  cd ~/libcrux-sha3-focused/crates/algorithms/sha3
  # The crate root contains src/, proofs/, ..., but it's nested.
  # Per-extraction snapshot:
  cd proofs/fstar/extraction
  find . -maxdepth 1 \( -name "Libcrux_sha3*.fst" -o -name "Libcrux_sha3*.fsti" \) | sort | xargs shasum > /tmp/pre.sha
  cd ../../..
  # check actual extract command: hax.sh or hax.py
  ./hax.sh extract 2>&1 || python3 hax.py extract 2>&1
  cd proofs/fstar/extraction
  find . -maxdepth 1 \( -name "Libcrux_sha3*.fst" -o -name "Libcrux_sha3*.fsti" \) | sort | xargs shasum > /tmp/post.sha
  diff /tmp/pre.sha /tmp/post.sha > /tmp/diff.sha
  CACHE=$(git rev-parse --show-toplevel)/.fstar-cache/checked
  for f in $(find . -maxdepth 1 \( -name "Libcrux_sha3*.fst" -o -name "Libcrux_sha3*.fsti" \)); do
    base=$(basename $f); chk=$CACHE/$base.checked
    if ! grep -qF "$base" /tmp/diff.sha && [ -f "$chk" ]; then touch "$chk"; fi
  done
  ```

## Deliverable

End-of-session report (≤ 200 words):
  - Milestones closed (✅) with commit SHAs.
  - `lib.rs` extraction status (the gating item) — landed Y/N + which
    digest fns now have hacspec ensures.
  - Surfaced lemmas — list the functions whose ensures now cite
    `EquivImplSpec.*` or `Hacspec_sha3.*`.
  - Final commit SHA on `sha3-proofs-focused`.
  - Next-priority recommendation.

DO NOT push to origin. DO NOT touch `~/libcrux-trait-opacify` or
`~/libcrux-ml-dsa-proofs`. DO NOT bundle the pre-existing dirty
proof_utils.fst / simd/avx2 work into your commits.

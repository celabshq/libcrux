# Prompt — close low-hanging admits/assume vals before the AVX2 store_block sprint

## Mission

Close the small, independent proof gates that don't require the structural-decomposition treatment that AVX2 store_block (and Arm64 load_block) need. Specifically:

1. **`assume val lemma_squeeze2_arm64`** in `EquivImplSpec.Sponge.Arm64.Driver.fst:101` — port the Portable squeeze closure template (commits / pattern referenced inline in `EquivImplSpec.Sponge.Portable.SqueezeAPI.fst:70-81`) to N=2.
2. **Stale comments** in `EquivImplSpec.Sponge.Arm64.Driver.fst:42-46` claiming `arm64_sc_load_block` / `arm64_sc_load_last` / `arm64_sc_store_block` are admitted — they are real `let`s now (in `EquivImplSpec.Sponge.Arm64.fst:150 / 386 / 487`). Update the comment.
3. **Status sweep**: identify any other equivalence-layer `assume val` / `admit ()` whose dependencies are now real proofs and which are similarly closeable. Surface them as a final-report addendum, do NOT close them in this sprint (avoid scope creep).

Goal: shrink the equivalence-layer admit count from 2 to 1 (`lemma_squeeze4_avx2` remains as the natural next-step candidate, scheduled separately) and document anything else that's "ripe."

## Why now (READ FIRST)

After the load/store module split landed at `f9e915bd8` on `sha3-proofs-focused`, an inventory of remaining admits/assume vals showed:

```
crates/algorithms/sha3/src/simd/avx2/store.rs:74          # AVX2 store_block body — separate sprint
EquivImplSpec.Sponge.Avx2.API.fst:87                       # assume val lemma_squeeze4_avx2 — N=4, harder
EquivImplSpec.Sponge.Arm64.Driver.fst:101                  # assume val lemma_squeeze2_arm64 — N=2, EASY
```

`lemma_squeeze2_arm64`'s assumption chain in the comment cites three NEON bridges (`arm64_sc_load_block`, `arm64_sc_load_last`, `arm64_sc_store_block`) that the Driver claims are also admitted. **They are not** — all three are real `let`s in `EquivImplSpec.Sponge.Arm64.fst` (lines 150, 386, 487). So the only thing standing between us and a closed `lemma_squeeze2_arm64` is the fold_range / squeeze-loop argument over `Libcrux_sha3.Generic_keccak.Simd128.squeeze2` — which is structurally identical to the closure achieved for the Portable side via the post-2026-04-25 squeeze refactor described in `EquivImplSpec.Sponge.Portable.SqueezeAPI.fst:70-81`.

The Portable closure had two parts:
- Lockstep induction pushed into the Rust-side `squeeze`'s ensures (`Libcrux_sha3.Generic_keccak.Portable.squeeze`'s post asserts equality with `EquivImplSpec.Sponge.Portable.Steps.portable_squeeze_composed`).
- Reconciliation of `portable_squeeze_composed` with `Hacspec_sha3.Sponge.squeeze` done at the equivalence layer in `EquivImplSpec.Sponge.Portable.API.lemma_squeeze_portable`.

Port both halves to N=2 for the Arm64/Simd128 case.

## Setup

```
cd /Users/karthik/libcrux-sha3-focused
git worktree add -b easy-bits-squeeze2 \
    /Users/karthik/libcrux-sha3-easy-bits-squeeze2 sha3-proofs-focused
```

Hard constraint: NEVER `cd` into `/Users/karthik/libcrux-sha3-focused`, sibling worktrees, etc. Use `git -C /Users/karthik/libcrux-sha3-easy-bits-squeeze2 ...` and absolute paths. Use `EnterWorktree path=...` once at the start.

## Required reading (no skipping)

1. **The Portable template (the entire closure pattern you're porting):**
   ```
   crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Portable.API.fst       # lemma_squeeze_portable (the actual proof)
   crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Portable.SqueezeAPI.fst  # bridge lemmas chained on top
   crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Portable.Steps.fst     # portable_squeeze_composed
   crates/algorithms/sha3/src/generic_keccak/portable.rs                                       # Rust-side ensures shape (look for `squeeze` and what its post asserts)
   ```
2. **The N=2 target setup:**
   ```
   crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Driver.fst       # the assume val
   crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.fst              # the (real) per-lane bridges
   crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Steps.fst        # likely needs a `simd128_squeeze_composed` analog
   crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.API.fst          # callers of lemma_keccak2_arm64 (which composes squeeze2)
   crates/algorithms/sha3/src/generic_keccak/simd128.rs                                        # Rust-side squeeze2 — current ensures is bounds-only
   ```
3. **Skills**: `fstar-mcp`, `fstar-for-libcrux`, `proofdebugging`, `smtprofiling`, `specreview`.
4. **Memory rules**:
   - `feedback_grep_make_output` — pipe make to log + grep, never read full F* logs.
   - `feedback_no_manual_edits_extracted` — Rust source + re-extract; equivalence-layer `.fst` files are handwritten (not extracted) so direct edits ARE the right way there.
   - `feedback_rlimit_cap_800` — ≤ 800 mono / ≤ 400 with split_queries.
   - `feedback_no_code_changes_for_proofs` — strengthening the Rust-side ensures of `Generic_keccak.Simd128.squeeze2` IS authorized by this prompt (mirror of Portable.squeeze). Anything beyond that, surface first.
   - `feedback_branch_means_worktree`, `feedback_per_stage_clean_rebuild`, `feedback_bisect_before_blame`, `feedback_agent_status_reports`, `feedback_use_fstar_mcp`, `feedback_fstar_mcp_session_dies_after_make`.
   - `project_sha3_arm64_load_block_cliff` — stay clear of load_block; it has a separate sprint queued.
   - **Budget**: lifted 60-min cap. Stop condition is **two consecutive 30-min cycles with no measurable progress**.
5. **Hand-off / context**:
   - `proofs/fstar/equivalence/HANDOFF.md` (look for the squeeze section + the post-2026-04-25 squeeze refactor narrative).

## Plan

### Stage 0 — sanity: re-verify split lands clean

`make check/Libcrux_sha3.Simd.Arm64.fst > /tmp/baseline.log 2>&1` and equivalent for `Avx2`. Confirm the base build is clean (modulo the known load_block cliff and AVX2 store_block admit). This is the diff-baseline.

### Stage 1 — port the Portable squeeze closure to N=2

1. **Strengthen `Libcrux_sha3.Generic_keccak.Simd128.squeeze2`'s Rust-side ensures.** Currently `crates/algorithms/sha3/src/generic_keccak/simd128.rs:99-105`:
   ```rust
   #[hax_lib::ensures(|_| future(out0).len() == out0.len() && future(out1).len() == out1.len())]
   ```
   Mirror Portable: assert equality with a new `simd128_squeeze_composed` (analog of `portable_squeeze_composed`). The composed function is defined in a new `EquivImplSpec.Sponge.Arm64.Steps.fst` section (or co-located if simpler). It chains:
   - `s.squeeze2::<RATE>(out0, out1, 0, RATE)` for the first block,
   - the for-loop body `keccakf1600 ; squeeze2(...)` repeated `blocks - 1` times,
   - the partial last `keccakf1600 ; squeeze2(.., last, outlen - last)`,
   into a single fold_range / `squeeze_blocks_*` shape that the equivalence layer can reconcile with `Hacspec_sha3.Sponge.squeeze`.
   The fold_range → `squeeze_blocks` bridge for one lane should be discharged inline (use the existing `Hacspec_sha3.Sponge.Lemmas.lemma_squeeze_blocks_*` already cited by the Portable template).
2. **Re-extract** via `bash hax.sh extract` from the sha3 crate root.
3. **Add `lemma_squeeze2_arm64` proof** in `EquivImplSpec.Sponge.Arm64.Driver.fst` (or wherever the Portable analog lives). Replace the `assume val` with a `let`. Body should be a per-lane induction analogous to `lemma_squeeze_portable`, citing:
   - The strengthened `squeeze2` ensures (gives you the lockstep equality vs `simd128_squeeze_composed` for free).
   - `arm64_sc_load_block` / `arm64_sc_load_last` / `arm64_sc_store_block` (already proved).
   - `lemma_keccakf1600_arm64` (already proved per `EquivImplSpec.Keccakf.Arm64`).
4. Verify per-module: `make check/EquivImplSpec.Sponge.Arm64.Driver.fst > /tmp/squeeze2.log 2>&1`, grep errors. Then dependents: `EquivImplSpec.Sponge.Arm64.API.fst` (which currently re-exports the assume val).
5. **Update the stale comment** in `EquivImplSpec.Sponge.Arm64.Driver.fst:42-46` — the `arm64_sc_*` bridges are real `let`s, not assumed.

### Stage 2 — sweep and document

1. After lemma_squeeze2_arm64 lands, scan the equivalence dir for any other `assume val` / `admit ()` whose dependency chain is now closed. Quick filter:
   ```
   grep -rnE 'assume val|admit ?\(\)' crates/algorithms/sha3/proofs/fstar/equivalence/*.fst
   ```
2. For each result, check whether its cited dependencies in nearby comments are now real `let`s. **Do NOT close additional admits in this sprint** — the goal is documentation. Add a final-report addendum listing each: location, dependency status, estimated difficulty if closed.
3. Update `HANDOFF.md` (or the relevant tracking doc) with the post-sprint admit count.

## Operational rules

- `feedback_grep_make_output`: pipe make to `/tmp/<name>.log`, grep for `^\* Error|All verification|TOTAL TIME`.
- `feedback_use_fstar_mcp`: fstar-mcp for sub-second iteration; full SMT through make.
- `feedback_fstar_mcp_session_dies_after_make`: recreate session after each make.
- `feedback_no_manual_edits_extracted`: handwritten equivalence-layer .fst files are direct-edit territory; extracted files (via hax) only via Rust source + re-extract.
- `feedback_rlimit_cap_800`: ≤ 800 mono / ≤ 400 with split_queries.
- `feedback_per_stage_clean_rebuild`: rm `.checked` for touched modules between stages.
- `feedback_branch_means_worktree`: stay in worktree.
- **Budget**: lifted 60-min cap. Stop condition is two consecutive 30-min cycles with no measurable progress.
- `feedback_agent_status_reports`: append to `crates/algorithms/sha3/proofs/agent-status/easy-bits-progress.md` every 15 min.
- `feedback_smtpat_lane_propagation` + `feedback_smtpat_percent_above_trait`: avoid SMTPats; explicit lemma calls.
- `feedback_bisect_before_blame`: if a proof outside scope fails after a change, bisect to parent before assuming you broke it.
- `feedback_drive_to_top_spike`: validate spec via consumer before discharging bodies — this is exactly the Portable template's strategy (push lockstep into Rust ensures via spec, reconcile spec at API).

## File boundaries

You own:
- `crates/algorithms/sha3/src/generic_keccak/simd128.rs` (squeeze2 ensures strengthening)
- `crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Generic_keccak.Simd128.fst` (regenerated; OK to probe directly)
- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Driver.fst` (close the assume val + fix stale comment)
- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Steps.fst` (likely needs `simd128_squeeze_composed`)
- `crates/algorithms/sha3/proofs/agent-status/easy-bits-*.md`

You may touch (with surface-to-user first):
- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.fst` — only if a small additive helper is needed; surface the proposed addition first.
- `crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.API.fst` — only if the re-export of `lemma_squeeze2_arm64` needs a signature touch (it shouldn't — closing the assume val is signature-preserving).

You do NOT touch:
- AVX2 files, EquivImplSpec.Sponge.Avx2.* (separate sprints).
- `simd/arm64/load.rs`, `simd/avx2/load.rs` (load_block cliff sprint owns these).
- `Hacspec_sha3.*`, `specs/sha3/*` (spec is canonical).
- `Libcrux_intrinsics.*`.

If the squeeze2 closure requires a Hacspec-side change (e.g., a new `lemma_squeeze_blocks_*` shape), STOP and surface to user.

## Deliverables

Commit on `easy-bits-squeeze2` branch (do NOT push):

- **Success**: `assume val lemma_squeeze2_arm64` replaced by a real `let`, `make check/EquivImplSpec.Sponge.Arm64.Driver.fst` passes clean, downstream `EquivImplSpec.Sponge.Arm64.API.fst` and `EquivImplSpec.Keccakf.Arm64.fst` re-verify clean. Commit message names the proof technique (Portable template port) and per-module timings.
- **Partial**: status doc with the proof obligation reduced (e.g., to one ground sub-fact), and the next-attempt path. The spec-side strengthening (Rust-side ensures of `squeeze2`) lands either way as a strict improvement.

## Final report (≤300 words)

(1) `lemma_squeeze2_arm64` closed (Y/N)? (2) If Y: summary of the proof technique, per-module timings, max sub-query time. (3) If N: precisely where the cliff/gap is, with qi.profile diagnosis if applicable. (4) Stale Driver.fst comment updated (Y/N)? (5) Stage 2 sweep — list of remaining admits/assume vals + ripe-or-not + estimated difficulty. (6) Branch SHA. (7) Notes that may help the AVX2 sprint (lemma_squeeze4_avx2 is the same pattern at N=4 — does the technique transfer?).

## Suggested first 30 min

1. EnterWorktree to `/Users/karthik/libcrux-sha3-easy-bits-squeeze2`.
2. Read the Portable closure top-to-bottom: `EquivImplSpec.Sponge.Portable.{API,SqueezeAPI,Steps}.fst` + `Generic_keccak.Portable.fst` (extracted).
3. Read the N=2 target: `EquivImplSpec.Sponge.Arm64.{Driver,fst}` + `Generic_keccak.Simd128.fst` (extracted).
4. Sketch on paper: what is `simd128_squeeze_composed`? What does the strengthened `squeeze2` ensures look like? What induction does `lemma_squeeze2_arm64` do?
5. First status entry to progress.md.
6. Stage 0 baseline make.

If by T+45 min the Portable template doesn't transfer cleanly to N=2 (e.g., you discover the per-lane projection lemma at the equivalence layer doesn't compose with the strengthened ensures), surface a 1-paragraph blocker note describing the divergence.

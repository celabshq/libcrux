# SHA-3 admit investigation — deep dive + closure plan

You're picking up a verified-cryptography campaign (libcrux SHA-3). Goal **this session**: investigate the 5 remaining load-bearing admits in depth, understand the cross-backend asymmetries (especially Arm64-proves-but-Avx2-admits), and produce a prescriptive closure plan. **You are NOT closing any admit this session** — investigate and plan only. Implementation is a separate sprint.

## Where things stand

- Repo: `/Users/karthik/libcrux-sha3-proofs`, branch `sha3-proofs-focused`, HEAD `d7ac1bddb`.
- Toolchain: F\* **2026.03.24** (NOT nightly-2026-04-12 — that has unrelated regressions on this tree). hax `integer-lemmas` @ `952bee04` with `hax-engine` 0.3.6 pinned at `~/hax/engine`. Build requires `OTHERFLAGS="--z3rlimit_factor 4"`.
- Baseline `make -C crates/algorithms/sha3/proofs/fstar` verifies clean modulo the 5 admits below.

## Required reading (in this order)

1. `crates/algorithms/sha3/proofs/sha3-sprint-todo.md` — campaign plan. The 2026-05-23 STATUS DELTA section and §"Parallel-branch inventory" are recent and load-bearing context. TL;DR has the 5-admit table with locations.
2. `crates/algorithms/sha3/proofs/agent-status/RESUME-PROMPT-2026-05-23.md` — prior session's findings on the e-matching cliff and what was tried that didn't work. Don't repeat negative experiments.
3. Invoke the **`fstar-for-libcrux`** skill before touching any `.fst`. The skill's §1.5 (regression detection), §1.5.1 (layered cascades), and §7 ("rlimit bump is a smell") are directly applicable.

## The 5 admits (current state)

1. `EquivImplSpec.Sponge.Arm64.Driver.fst:111` — `assume val lemma_squeeze2_arm64` (Arm64 squeeze2). Building-block lemma `lemma_squeeze_one_step_arm64` IS proven at `EquivImplSpec.Sponge.Arm64.Steps.fst:243`.
2. `EquivImplSpec.Sponge.Avx2.API.fst:87` — `assume val lemma_squeeze4_avx2` (Avx2 mirror of #1). **No analogous `lemma_squeeze_one_step_avx2` exists** — `EquivImplSpec.Sponge.Avx2.Steps.fst` has a gap here. **Asymmetry to explain.**
3. `crates/algorithms/sha3/src/simd/avx2/store.rs:74` — `hax_lib::fstar!("admit()")` → `Libcrux_sha3.Simd.Avx2.Store.fst:165`. **Arm64 store_block IS fully proven** (`Libcrux_sha3.Simd.Arm64.Store.fst:1204` plus `_full`/`_tail` split + `StoreBlockHelpers.fst`). **Asymmetry to explain.** Substantial WIP scaffolding for the Avx2 closure lives unmerged in `~/libcrux-sha3-store-avx2-discharge` (HEAD `464a9914a`, 3 commits ahead) and `~/libcrux-loop-inv-opacify` (HEAD `c62edb033`, has `LoopInv.fst`) — read those before re-creating anything.
4. `EquivImplSpec.Sponge.Arm64.fst:124` — `admit ();` at start of inner `byte_eq` lemma in `lemma_load_block_eq_xor_block_into_state_arm64`. Used 59/1600 rlimit (3.7%) — e-matching bound, NOT budget-bound. Confirmed structural at `--z3rlimit_factor 16` (same 59.228 rlimit used).
5. `EquivImplSpec.Sponge.Avx2.fst:125` — exact AVX2 mirror of #4. 73/1600 rlimit. Same cliff family as the 2026-05-05 load_block one (which WAS closed via `[@@ "opaque_to_smt"]` on `Hacspec_sha3.createi` — see commit `7bb581f8b` and `~/.claude/skills/fstar-for-libcrux/SKILL.md §1.5` worked example). The createi opacification is in place (`specs/sha3/src/lib.rs:10`), so this is a *different* cliff downstream of that fix — the `extract_lane (load_block …)` consumer side rather than `load_block`'s body.

## What I want from this session

For each of the 5 admits, deliverables:

1. **Why does it fail right now?** Don't guess — get evidence. Per skill §1.5, use `smtprofiling` skill: `--log_queries --z3refresh --query_stats` on the failing function, find the failing query's `.smt2`, run `smt.qi.profile=true` on it, aggregate `[quantifier_instances]`. The dominant quantifier (often `k!N` anonymous or a named `Tm_refine_<hash>`) is the cascade source. Report it with instance count.

2. **Why does the *other* backend's analogue work?** For admits 2/3/5 specifically, the Arm64 sibling proves where the Avx2 admits. Compare: SMT options, push-options rlimit, helper lemmas in scope, SMTPat triggers, intrinsic-level postconditions (`crates/utils/intrinsics/src/avx2_extract.rs` vs the Arm64 NEON equivalents). The Arm64-passes/Avx2-admits asymmetry is the single biggest clue in this codebase — exploit it.

3. **Prescriptive closure plan.** For each admit, name the specific fix shape (opacity on X, SMTPat narrowing on Y, structural factor of Z) per the skill's playbook. Estimate effort. Flag dependencies (some admits will fall to the same root cause; ordering matters).

4. **Cross-admit synthesis.** Are 4 and 5 the same root cause manifesting twice? Is 3 actually solvable using the scaffolding in `store-block-avx2-discharge` + `LoopInv.fst` (read those branches before answering — note their `--admit_smt_queries true` blockers and the hax-checkout include drift mentioned in `c62edb033`'s commit message). Recommend sprint ordering, taking dependencies into account.

## Process discipline (skill rules — read before doing anything)

- **fstar-for-libcrux §2 Rule 1**: never `Read` a full F\* make log. Always grep for errors. Logs go to `/tmp/`.
- **§2 Rules 2–3**: inner loop is fstar-mcp (`typecheck_buffer` lax, then full; `lookup_symbol` for signatures). Outer loop is `make`. fstar-mcp session dies after every `make` — recreate.
- **§2 Rule 8**: per-fn budget is 30–60 min. If a single admit's diagnostic is taking longer, document the blocker and move on. This is investigation, not closure — don't go down rabbit holes.
- **No hints recorded**: when running `make` with admits in place, use `ENABLE_HINTS="--use_hints"` (no `--record_hints`) so the cache doesn't get polluted with admit-bearing hints. Per the resume-prompt's "Hard-won lesson #1".
- **Don't add admits** to chase a profile. Admit-mode in this session is read-only — the 5 admits already in place are what you investigate, not a license to add more.
- **Don't edit Rust source** unless explicitly required by a diagnostic experiment, and even then only as a temporary probe per skill §8 "Debugging proofs in F\* directly".

## What to produce

A new file `crates/algorithms/sha3/proofs/agent-status/admit-investigation-YYYY-MM-DD.md` (use today's date) with one section per admit, structured as:

```
## Admit N: <name> at <file:line>

### Failure shape
<from qi.profile + query stats>

### Asymmetry with sibling backend (if applicable)
<what the working backend does differently>

### Root cause hypothesis
<the cascade source you found>

### Closure plan
<specific fix shape, effort, dependencies>
```

Plus a final §"Sprint ordering" with the recommended sequence + rationale.

Commit the investigation note at the end. Don't modify `sha3-sprint-todo.md` — that's the next sprint's job after this investigation lands.

## Final note

Treat the parallel branches (`store-block-avx2-discharge`, `loop-invariant-opacify`, `sha3-byteform-migration-squeeze2`) as **valuable prior art** — they have profiling data, structural split designs, and a `LoopInv.fst` predicate stack that you do NOT need to re-derive. Their authors gave up *not* because the work is wrong but because of either (a) the residual cascade they didn't have time to fix or (b) an environmental blocker (hax-checkout drift). Inherit what you can; don't waste cycles re-creating.

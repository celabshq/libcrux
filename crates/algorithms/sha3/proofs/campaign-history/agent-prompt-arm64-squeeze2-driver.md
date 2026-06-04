# Fresh-session prompt — close the arm64 `squeeze2` driver proof (opaque-predicate strategy)

> Paste this into a fresh Claude Code session opened in
> `/Users/karthik/libcrux-sha3-proofs/crates/algorithms/sha3`.
> Read the skill `fstar-for-libcrux` and the memory file
> `feedback_opaque_predicate_store_proof` before writing any F*.

---

## Goal

Close the **one** admit gating the entire arm64 (NEON, N=2) functional-squeeze proof:
`--admit_smt_queries true` at **`src/generic_keccak/simd128.rs:120`** on the driver
`squeeze2`. Use **strategic opaque predicates** (the recipe that closed AVX2
`store_block`) to tame the two-lane × all-output-bytes cross-product that has cliffed
every prior attempt and that the admit is currently papering over.

Full design is in **`proofs/agent-status/arm64-squeeze2-plan-2026-05-30.md`** — read it
first; it has the predicate definition, frame lemmas, proof skeleton, and the
architectural spike.

## Why this is tractable now

Everything below `squeeze2` is **already proven** — do NOT re-prove them:
`lemma_squeeze_block_arm64`, `lemma_squeeze_last_arm64`, **`lemma_squeeze_one_step_arm64`**
(the per-iteration keccakf+block engine, `Arm64.Steps.fst:243`), the `arm64_sc_*` NEON
bridges (`EquivImplSpec.Sponge.Arm64.fst`), and arm64 `store_block`'s per-byte post
(`src/simd/arm64/store.rs:36`). The **Portable** `squeeze`/`squeeze_blocks`/`squeeze_last`
(`src/generic_keccak/portable.rs:356/256/96`) + `Portable.SqueezeAPI`'s
`lemma_fold_range_step` bridge are the exact structural template. Your job is only the
**loop-shape composition**, with opacity keeping it out of Z3's saturation zone.

## Plan of attack

1. **Phase 0 — resolve the host (spike, do this FIRST).** Determine whether the proof
   can live inline in the Rust driver body (Path A) or must go in the equivalence layer
   (Path B), per §3 of the plan. Test by referencing an `Arm64.Steps` lemma from the
   driver's module and running `fstar_build` to see if the dep graph cycles. Path B
   (equivalence-layer proof of `lemma_squeeze2_arm64` via the extracted fold) is the
   likely cycle-safe answer; revert the Rust ensures to length-only there.
2. **Define the opaque predicate `squeezed_upto` + 3 confined-reveal frame lemmas**
   (`_zero`, `_extend`, `_full`) per §4. Mirror `stored`/`modifies_range` +
   `lemma_stored_*` from `src/simd/avx2/store.rs`.
3. **Restate `lemma_squeeze_one_step_arm64` into `squeezed_upto` terms** (thin `_op`
   wrapper, reveal confined to its body) so every loop-step VC sees only the opaque pred.
4. **Assemble top-down** per §5: block-loop invariant (per lane: `squeezed_upto ..
   (i*rate)` + `iterate_keccak_f i s_init` + length), tail via `lemma_squeeze_last_arm64`,
   close with `lemma_squeezed_upto_full` ×2 lanes.
5. **Full module build, no `--admit_except`** (it hides own-VCs); admit-count gate
   (net −1); `cargo test` (native NEON; no `--target x86_64`) stays green.

## Hard rules (from memory + skill — non-negotiable)

- **No code changes to satisfy proofs**: never add ops to the impl; preserve impl,
  adapt proofs / spec. (`feedback_no_code_changes_for_proofs`)
- **`smtprofiling` BEFORE reporting any cliff/blocker/negative** — saturation vs cascade
  vs missing-fact are indistinguishable without `qi.profile`. (`feedback_smtprofile_before_negative`)
- **rlimit cap 800 (400 with `--split_queries always`)** — never exceed; saturation
  means restructure (more opacity / smaller window), not bump. (`feedback_rlimit_cap_800`)
- **`--admit_except` hides sibling AND own-VC obligations** → always finish with a full
  `fstar_build check/<Module>.fst`. (`feedback_opaque_predicate_store_proof`)
- Use the **fstar-proxy MCP** (`fstar_typecheck` interactive, `fstar_build` batch) — never
  shell `make`. (`feedback_use_fstar_mcp`) Large files exceed the 60 s cap: use the curl
  + `kill -0` wait-loop for full builds.
- **Never touch `.checked` mtimes / bulk-delete cache**; let hax/make manage staleness.
  (`feedback_no_checked_tampering`, `feedback_no_cache_nuke`)
- **Never hand-edit extracted `Hacspec_*` / generated `.fst`** as a spec fix — change
  Rust + re-extract. Manual `.fsti`/equivalence `.fst`s are fair game (note they're
  untracked). (`feedback_no_manual_edits_extracted`)
- **Per-function budget 30–60 min**; past that `fstar_note(level="cliff", ...)` + stop +
  surface to user. (`feedback_proof_debug_budget`, `feedback_fstar_errors_ask_user`)
- **Max 4 concurrent fstar/z3 processes.** (`feedback_max_4_fstar_per_agent`)
- **Do NOT commit anything** — this campaign may rebase; user commits explicitly.
- **Status report every ~15 min** if run as a long agent (sub-task / blocker / ETA).

## Definition of done

- `src/generic_keccak/simd128.rs:120` admit removed; driver `squeeze2` either proves its
  functional ensures for real (Path A) or carries length-only ensures with the functional
  spec proven in `lemma_squeeze2_arm64` (Path B).
- `lemma_squeeze2_arm64` is a real proof (not an empty ensures-reader off an admitted
  driver).
- Full `fstar_build` of the touched module(s) green, no `--admit_except`, admit-count
  net −1, `cargo test` green.
- Then (separate follow-on): the N=4 twin `lemma_squeeze4_avx2`
  (`EquivImplSpec.Sponge.Avx2.API.fst:87`, still `assume val`) reusing this exact design.

# Resume prompt — SIMD intrinsics trust-base sprint

Copy the block below into a fresh Claude Code session. The plan it references
is committed at `crates/utils/core-models/INTRINSICS-TRUST-PLAN.md` on branch
`libcrux-ml-kem-proofs` (commit `e86e29488` and three preceding retrospective
cherry-picks `2d5539845`, `7f5804c00`, `79405cf15`).

---

```
Continue the SIMD intrinsics trust-base sprint.

Working directory: /Users/karthik/libcrux-trait-opacify
Branch: libcrux-ml-kem-proofs (do not switch branches)
Plan: crates/utils/core-models/INTRINSICS-TRUST-PLAN.md (read first, fully)
Auto mode: active — execute autonomously, minimize interruptions, prefer
action over planning.

Goal: drive the trust-ladder metric from today's roughly (28%, 26%, 100%, 0%,
0%) to (100%, 100%, 100%, 100%, 0%) on T1 = 193 libcrux-used intrinsics
(99 AVX2 in crates/utils/intrinsics/src/avx2.rs + 94 NEON in arm64.rs).
D5 (F*-discharged-not-admitted) stays at 0% — that's the deferred unification
plan, do NOT execute it this sprint.

START by integrating the three "Deferred refinements" at the bottom of the
plan, in this order:

1. Restructure the six-step plan into Phase A (serial setup) → Phase B
   (max-parallel, up to 5 isolated git worktrees: B1 wt-avx2-defined,
   B2 wt-avx2-handwritten, B3 wt-neon-defined, B4 wt-neon-handwritten,
   B5 wt-cross-validate) → Phase C (serial close: prune + CI invariant).
   Use `Agent` with `isolation: "worktree"` for each Phase B lane. Specify
   per-lane file ownership (writes), inputs (read-only), pass condition,
   merge protocol, and 15-min status report cadence per
   feedback_agent_status_reports. If A2 file restructure is too risky for
   the current proof state (F* qualified-name churn), collapse Phase B
   to 3 lanes (AVX2 backfill, NEON port, cross-validate).

2. Extend libcrux-ml-kem/proofs/retrospective-methodology.md to add
   D6 (TCB strength: D6.1 model coverage, D6.2 test coverage, D6.3
   F* spec coverage, D6.4 audit consistency, D6.5 F* spec proven) and
   W5 (TCB validation: W5a model LOC, W5b mk! count, W5c audit-script LOC).
   Update libcrux-ml-kem/proofs/initial-retrospective.md with the
   2026-05-02 baseline TCB strength numbers from the plan's table.

3. The lineage-cleanup note in the plan is informational; no action needed
   beyond confirming trait-opacify stays uncommitted-to.

THEN execute Phase A1 (audit script). Steps 2-6 of the original plan map
onto Phase B and C.

Hard constraints from the plan's "Explicit deferrals" section — these
files must NOT be touched structurally this sprint:
- crates/utils/intrinsics/src/{avx2,arm64}_extract.rs (keep ensures verbatim)
- fstar-helpers/fstar-bitvec/* (defer entirely)
- libcrux-ml-dsa/proofs/fstar/spec/Spec.Intrinsics.fsti (audit only)
- ml-kem's --cfg pre_core_models (keep)
- libcrux-ml-kem/proofs/simd-model-unification-plan.md execution (deferred)

Source for body ports during Phases B1-B4:
~/verify-rust-std/testable-simd-models/ (Cryspen-authored Apache-2.0 superset
with full NEON + broader AVX2). Add attribution headers when porting.

Memory entries that apply: feedback_no_cache_nuke, feedback_use_fstar_mcp,
feedback_no_manual_edits_extracted, feedback_extraction_first,
feedback_track_fstar_perf, feedback_agent_status_reports,
feedback_proof_debug_budget.

Deliverables when sprint completes:
1. crates/utils/core-models/scripts/intrinsics-audit.{rs,py} — audit script.
2. A trust index doc (location TBD; either tracked under core-models with a
   gitignore exception, or under libcrux-ml-kem/proofs/) refreshable by the
   audit script.
3. crates/utils/core-models/scripts/cross-validate.rs + a findings doc
   listing every F*-spec ↔ tested-Rust-body mismatch discovered.
4. CI invariant gating future PRs (D1=D2=D4=100% on T1).
5. cargo test -p core-models passes on x86_64 AND aarch64-apple-darwin.
6. cargo build of libcrux-ml-kem and libcrux-ml-dsa, plus their F*
   extraction, pass without regression.

If you spawn long-running agents, brief them to emit a status report every
15 min (sub-task, blocker, ETA) per feedback_agent_status_reports. Per
feedback_proof_debug_budget, 30-60 min hard cap per single intrinsic before
marking as follow-up and moving on.

Note: the stale modification to libcrux-ml-kem/src/ind_cpa.rs in the working
tree is from a parallel sprint — leave it alone.
```

---

## Quick reference

- **Plan**: `crates/utils/core-models/INTRINSICS-TRUST-PLAN.md`
- **Retrospective methodology** (to extend): `libcrux-ml-kem/proofs/retrospective-methodology.md`
- **Initial retrospective** (to extend): `libcrux-ml-kem/proofs/initial-retrospective.md`
- **Unification plan** (deferred — do NOT execute): `libcrux-ml-kem/proofs/simd-model-unification-plan.md`
- **Port source**: `~/verify-rust-std/testable-simd-models/`
- **Branch**: `libcrux-ml-kem-proofs` in `/Users/karthik/libcrux-trait-opacify`
- **Most recent commit**: `e86e29488 agent-mlkem: SIMD intrinsics trust-base sprint plan (v1)`

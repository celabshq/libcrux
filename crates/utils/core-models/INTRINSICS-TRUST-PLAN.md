# SIMD Intrinsics Trust-Base Sprint — Execution Plan

**Status**: ready to execute.
**Branch**: `libcrux-ml-kem-proofs` in `/Users/karthik/libcrux-trait-opacify`.
**Companion docs**:
- `libcrux-ml-kem/proofs/simd-model-unification-plan.md` (long-term unification, **deferred** until ml-kem trait layer stabilizes — this sprint deliberately does NOT execute that plan).
- `libcrux-ml-kem/proofs/retrospective-methodology.md` and `initial-retrospective.md` on the `trait-opacify` branch (prior measurement work).

## Problem statement

All three libcrux verification efforts (ml-kem, ml-dsa, sha-3) rely on SIMD intrinsic models that are currently **pure axioms**: F* `val` declarations with admitted lane-form `ensures` clauses or `SMTPat` lemmas, with no executable counterpart and no differential test against the real CPU. The Trusted Computing Base of every published proof rests on this unvalidated assumption layer.

Five overlapping models exist:
- **Model A** — `crates/utils/intrinsics/src/{arm64,avx2}_extract.rs` — hand-written F*/Lean axioms via `#[hax_lib::fstar::replace(...)]`. Used by ml-kem AVX2 (via `--cfg pre_core_models`), ml-kem NEON, ml-dsa NEON, sha-3 NEON.
- **Model A1** — `fstar-helpers/fstar-bitvec/BitVec.Intrinsics.fsti` — F*-only bodies referenced by Model A includes. ~30 intrinsics.
- **Model B** — `crates/utils/core-models/` — Rust models, hax-extracted to F*. Has `mk!` randomized differential tests against `core::arch::x86_64::*`. Used by ml-dsa AVX2.
- **Model C** — `libcrux-ml-dsa/proofs/fstar/spec/Spec.Intrinsics.fsti` — 62 hand-written F* SMTPat lemmas. Pure axioms layered on Model B.
- **Model 0** — `crates/utils/intrinsics/src/{arm64,avx2}.rs` — production Rust impl, uses `core::arch` with `#[cfg(hax)]` redirect to core-models.

External reference: `~/verify-rust-std/testable-simd-models/` is a Cryspen-authored superset of core-models with broader AVX2 coverage (1873 vs 1522 LOC) AND a complete NEON model (872 LOC + 218 `mk!` tests). No hax annotations.

## Sprint goal: narrow scope to libcrux usage

Instead of expanding core-models to match testable-simd-models, **shrink core-models to exactly the set libcrux uses**, in preparation for eventually moving core-models out of libcrux into its own repository.

The contract is `T1` = the set of `pub fn` in `crates/utils/intrinsics/src/{avx2,arm64}.rs`:
- `T1_avx2 = 99` intrinsics.
- `T1_arm64 = 94` intrinsics.
- `T1 = 193` total.

After the sprint, every intrinsic in `T1` has:
1. A concrete Rust body in core-models (`D1 = 100%`).
2. A `mk!` randomized differential test against the real CPU (`D2 = 100%`).
3. A consumer-facing F* spec (ensures clause or SMTPat) — already 100% today.
4. An audit consistency check that the F* spec is empirically consistent with core-models' tested body, run on random inputs (`D4 = 100%`).

`D5` (F* spec discharged not admitted) stays at 0% — that's the deferred unification plan.

## Trust ladder

| Level | Definition |
|---|---|
| L0 | F* spec only (ensures or SMTPat lemma). No Rust model, no test. Pure axiom. |
| L1 | F* spec + Rust model body in core-models. Body untested. |
| L2 | L1 + `mk!` randomized differential test against real CPU. |
| L3 | L2 + cross-validation script: F* spec predicate evaluated against core-models' tested body on random inputs. |
| L4 | L3 + F* obligation discharged in F* via core-models extraction + lift lemma + bit-vec↔int-vec bridge. **Deferred.** |

Today: ~26% of T1 at L2, the rest at L0. After sprint: 100% of T1 at L3.

## Six-step sprint plan

### Step 1 — Audit script (1-2 days)

**Owner of artifact**: `crates/utils/core-models/scripts/intrinsics-audit.rs` (or `.py`).

Implementation: `syn`-based scanner that reads:
- `crates/utils/intrinsics/src/{avx2,arm64}.rs` → T1 = list of `pub fn` names.
- `crates/utils/core-models/src/core_arch/x86.rs`, `core_arch/x86/interpretations.rs`, `core_arch/arm/*.rs` (when added) → T2 = list with sub-flags `has-body` / `has-mk_lift_lemma!` / `has-mk!`.
- `libcrux-ml-dsa/proofs/fstar/spec/Spec.Intrinsics.fsti` → T3 = list of intrinsics referenced in lemma statements (regex `I\.mm…` or similar).
- `crates/utils/intrinsics/src/{avx2,arm64}_extract.rs` → list of intrinsics with their `#[hax_lib::ensures(...)]` clauses (the consumer-facing post-conditions).

**Output**: `crates/utils/core-models/proofs/intrinsics-trust-index.md` with:
- Per-intrinsic CSV: `name, in_T1, has_body, has_lift_lemma, has_mk_test, has_extract_ensures, has_specintrinsics_lemma, L_level`.
- Summary header with D1, D2, D3, D4 percentages.
- Three difference sets: `T1 \ T2` (gaps to fill), `T2 \ T1` (candidates to drop), `T3 \ T1` (orphan SMTPat lemmas).

**Pass condition**: script runs cleanly, produces the index file, and the index file's CSV has 193 rows under `in_T1=true`.

### Step 2 — AVX2 backfill (1-2 weeks)

For every intrinsic flagged "in T1, opaque-only or missing body" in the audit:

1. Port a body from testable-simd-models if it exists there (preferred — already tested), else from upstream `stdarch` (for `defined` intrinsics — copy/adapt body from `core::arch::x86::avx2::*`), else hand-model from Intel docs (for `extern` LLVM-leaf intrinsics — these go in a `*_handwritten.rs` file).
2. Drop `#[hax_lib::opaque]` if the body is now concrete and hax-extractable; keep it for `extern` leaves.
3. Add `mk_lift_lemma!` postulating equality between upstream and the integer-vector composition (if going through int-vec) or directly between upstream and the bit-vector body.
4. Add `mk!(_mm256_X(…));` test invocation in `crates/utils/core-models/src/core_arch/x86/interpretations.rs::tests`.
5. Run `cargo test -p core-models` and confirm the new test passes.

**Reorganization** (recommended but optional): split `core_arch/x86.rs` along testable-simd-models' lines: `models/{sse2,ssse3,avx,avx2}.rs` for `defined` intrinsics + `models/{…}_handwritten.rs` for `extern` LLVM-leaves. Cleaner, but only do this if the audit shows the gap is large enough to justify file restructuring.

**Pass condition**: D1_avx2 = 100%, D2_avx2 = 100%. `cargo test -p core-models` passes on x86_64.

### Step 3 — NEON port (2-3 weeks)

`core-models` has zero NEON today. Port from testable-simd-models' `arm_shared/models/neon.rs` (872 LOC) + `arm_shared/tests/neon.rs` (218 `mk!` invocations), but **only the 94 intrinsics in `T1_arm64`**.

1. Create `crates/utils/core-models/src/core_arch/arm/{mod.rs, neon.rs, neon_handwritten.rs}`.
2. Reconcile const-generic: testable-simd-models uses `BitVec<const N: u32>`, core-models uses `BitVec<const N: u64>`. Port to core-models' `u64` to match the existing `Spec.Intrinsics.fsti` references.
3. For each `T1_arm64` intrinsic, copy the body from testable-simd-models (or adapt from upstream `core::arch::aarch64`).
4. Add hax annotations consistent with core-models' x86 module: `#[hax_lib::opaque]` for `extern` leaves, concrete for `defined` intrinsics.
5. Add `mk_lift_lemma!` invocations.
6. Add `mk!` tests for all 94, gated on `target_arch = aarch64`. Wire CI to run on Apple Silicon GitHub Actions runners (single-line workflow change).
7. Update `crates/utils/intrinsics/src/arm64.rs` to add `#[cfg(hax)] pub use core_models::arch::arm::*;` (mirror existing AVX2 pattern).

**Do NOT touch** `crates/utils/intrinsics/src/arm64_extract.rs` structurally. The hand-axiom `ensures` clauses there stay verbatim — they're the spec ml-kem/ml-dsa/sha-3 NEON proofs cite. core-models is built up alongside, not as a replacement.

**Pass condition**: D1_arm64 = 100%, D2_arm64 = 100%. `cargo test -p core-models` passes on `aarch64-apple-darwin` runner.

### Step 4 — Prune (1-2 days)

After Steps 2-3, run the audit script again. Anything in `T2 \ T1`:
- If it's an opaque val with no body, **drop**. Removes axiom surface.
- If it's a concrete body that no libcrux call site uses, **drop** unless it's a useful supporting helper (e.g., conversion fn).
- If `T3 \ T1` is non-empty, drop the orphan SMTPat lemma in `Spec.Intrinsics.fsti` — *but only if* `cargo build -p libcrux-ml-dsa` and `make verify` in `libcrux-ml-dsa/proofs/fstar/extraction` still pass. Verify each drop individually.

**Pass condition**: |T2| = |T1| = 193 (modulo unavoidable supporting helpers). |T3 \ T1| = 0.

### Step 5 — Cross-validation script (3-5 days)

**Artifact**: `crates/utils/core-models/scripts/cross-validate.rs` or extend the audit script.

For each intrinsic in T1, parse the `#[hax_lib::ensures(|result| fstar!("..."))]` clause from `_extract.rs` AND the corresponding SMTPat lemma from `Spec.Intrinsics.fsti` (if present). The clauses are F* expressions; we need a Rust evaluator for the lane-form predicate sub-language they use:
- `vec256_as_i16x16`, `vec128_as_i16x8`, `vec128_as_u8x16`, etc. — lane decomposition. Implementable as `BitVec::to_i16x16` etc. in Rust.
- `Seq.create N v`, `Seq.index seq i` — sequences. Implementable as `Vec<T>`.
- `Spec.Utils.map2 op` — pointwise binary op.
- `Spec.Utils.map_array f` — pointwise unary op.
- `forall (i:nat{i<N}). P i` — universal quantification. Implementable as bounded loop over `0..N`.
- Pointwise lane predicates: `get_lane_X result i == ...`. Pointwise check.

Most ensures clauses use one of ~10 patterns. Build a small Rust DSL that parses each pattern and emits an evaluator closure. For each intrinsic:
1. Generate 10,000 random inputs.
2. Compute LHS via real `core::arch::*::intrinsic(input)`.
3. Compute RHS via the parsed predicate evaluated on `input` and `LHS`.
4. Assert pass; record FAIL with input/expected/got.

**Output**: `crates/utils/core-models/proofs/intrinsics-cross-validation-findings.md` listing every mismatch. Each one becomes a PR or issue.

**Coverage scope**: this script must cover every `T1` intrinsic that has an ensures clause OR a SMTPat lemma. For intrinsics that have neither (rare — only the lowest-level loads/stores), fall back to "model is at L2, no L3 audit possible."

**Pass condition**: D4 = 100% (or D4 < 100% with the missing intrinsics enumerated as out-of-scope).

### Step 6 — CI invariant (1 day)

`cargo xtask check-intrinsics-parity` (or a dedicated script):
1. Re-run Steps 1 and 5.
2. Fail if D1 < 100%, D2 < 100%, or D4 < 100% on T1.
3. Fail if any new `pub fn` is added to `crates/utils/intrinsics/src/{avx2,arm64}.rs` without a matching body + `mk!` + audit-passing predicate in core-models.

Wire into GitHub Actions on x86_64 + aarch64-apple-darwin matrix.

**Pass condition**: PR that adds an intrinsic without modeling it fails CI; PR that breaks an existing audit fails CI.

## Explicit deferrals (do NOT touch this sprint)

| File / Task | Why deferred |
|---|---|
| `crates/utils/intrinsics/src/avx2_extract.rs` (structural) | ml-kem proofs cite its ensures + BitVec.Intrinsics-mediated bodies; replacing them requires the unification plan's bit-vec↔int-vec bridge work. |
| `crates/utils/intrinsics/src/arm64_extract.rs` (structural) | Same reason. The hand-axiom ensures stay; core-models grows in parallel. |
| `fstar-helpers/fstar-bitvec/BitVec.Intrinsics.fsti` and friends | Slated for retirement in unification Phase 3. Cross-validation script bypasses these by testing consumer-facing lemmas end-to-end against real CPU. |
| `Tactics.GetBit.fst:42-46` `delta_namespace` update | Phase 1.5 of unification. |
| `libcrux-ml-dsa/proofs/fstar/spec/Spec.Intrinsics.fsti` (rewrite) | 62 SMTPat axioms are load-bearing for ml-dsa proof state. Audit only (Steps 1, 5). Don't restructure. |
| Dropping `--cfg pre_core_models` from ml-kem | Unification Phase 3. |
| `mm_movemask_epi8_bv`, `mm256_concat_pairs_n`, `mm256_madd_epi16_specialized'`, `saturate8` port to core-models | Unification Phase 2. |
| Dead `call_native_intrinsic` hook in BitVec.Intrinsics.fsti:402-415 | Harmless; removing forces re-extraction. Leave alone. |

## Reportable deliverables

1. **`crates/utils/core-models/proofs/intrinsics-trust-index.md`** — refreshed by audit script. Five-dimension trust metric, per-intrinsic CSV.
2. **`crates/utils/core-models/proofs/intrinsics-cross-validation-findings.md`** — every mismatch found between F* spec and tested Rust body, with input/expected/got. Each finding is a security artifact: bug in spec, bug in model, bug in upstream, or spec ambiguity.
3. **CI parity check** — locks the trust index at 100% going forward.

## Estimated total effort

| Task | Effort |
|---|---:|
| Step 1: audit script | 1-2 days |
| Step 2: AVX2 backfill | 1-2 weeks |
| Step 3: NEON port | 2-3 weeks |
| Step 4: prune | 1-2 days |
| Step 5: cross-validation script | 3-5 days |
| Step 6: CI invariant | 1 day |
| **Total** | **~5-6 weeks** |

Steps 2 and 3 can run in parallel after Step 1 completes. Step 5 can run in parallel with Steps 2-4 as long as the audit script (Step 1) is producing a valid trust index.

## Key references

- `crates/utils/intrinsics/src/{avx2,arm64}.rs` — T1 (libcrux surface).
- `crates/utils/intrinsics/src/{avx2,arm64}_extract.rs` — F* axiom site (do not touch).
- `crates/utils/core-models/src/core_arch/x86.rs` + `interpretations.rs` — Model B home, Step 2 work.
- `crates/utils/core-models/src/core_arch/arm/` — Step 3 destination (does not exist yet).
- `~/verify-rust-std/testable-simd-models/src/core_arch/{x86,arm_shared}/` — port source.
- `libcrux-ml-dsa/proofs/fstar/spec/Spec.Intrinsics.fsti` — Model C, audit-only.
- `fstar-helpers/fstar-bitvec/` — Model A1, deferred entirely.
- `libcrux-ml-kem/proofs/simd-model-unification-plan.md` — long-term plan, do NOT execute this sprint.
- `libcrux-ml-kem/proofs/retrospective-methodology.md` — measurement framework (W1-W4, D1-D5). Will be extended with D6 (TCB strength) + W5 (TCB validation) — see "Deferred refinements" below.

## Deferred refinements (next session)

The plan above is committed in v1 form. The following improvements were
discussed but not yet integrated; pick them up at the start of the next
session before spawning execution agents.

### 1. Restructure plan for maximum parallelism with isolated worktrees

The current plan has six numbered steps that are mostly serial. Restructure
into three phases (A serial setup → B max-parallel lanes → C serial close)
where Phase B has up to 5 isolated git worktrees running concurrently:

- **Phase A (serial, 1-3 days)**:
  - A1: audit script (single worktree, branch `intrinsics-audit`, owns `crates/utils/core-models/scripts/`).
  - A2 (optional but enables wider B-fanout): file restructure of `core_arch/x86.rs` along testable-simd-models' `models/{sse2,ssse3,avx,avx2}.rs` + `models/*_handwritten.rs` lines, with hax `replace` annotations preserving F\* qualified names so `Spec.Intrinsics.fsti` doesn't break.
- **Phase B (max parallel, 1-3 weeks)**:
  - B1: AVX2 *defined* intrinsics → `wt-avx2-defined`, owns `models/avx2.rs` + matching tests.
  - B2: AVX2 *extern* intrinsics → `wt-avx2-handwritten`, owns `models/avx2_handwritten.rs` + tests.
  - B3: NEON *defined* intrinsics → `wt-neon-defined`, owns new `core_arch/arm/models/neon.rs` + tests.
  - B4: NEON *extern* intrinsics → `wt-neon-handwritten`, owns new `core_arch/arm/models/neon_handwritten.rs` + tests.
  - B5: cross-validation script → `wt-cross-validate`, owns `crates/utils/core-models/scripts/cross-validate.rs` + a findings markdown.
- **Phase C (serial, 1-3 days)**:
  - C1: prune T2\T1 + T3\T1 → `wt-prune`.
  - C2: CI invariant + workflow → `wt-ci`.

Each lane spec must include: worktree path, branch name, file ownership (writes),
file inputs (read-only), pass condition, status-report cadence (every 15 min per
`feedback_agent_status_reports`), and merge protocol (target branch, ordering).

If A2 is skipped (avoid F\*-name churn risk during proof effort), Phase B
collapses to 3 lanes: AVX2 backfill (single lane, sequential edits to `x86.rs`),
NEON port (single lane, fresh files, no conflict), cross-validate (independent
lane, scripts only).

### 2. Extend retrospective measurement framework

Update `libcrux-ml-kem/proofs/retrospective-methodology.md` to add:

- **D6 — TCB strength** (5 sub-percentages over T1 = 193 libcrux-used intrinsics):
  - D6.1 Rust-model coverage (% T1 with concrete body in core-models).
  - D6.2 Test coverage (% T1 with `mk!` randomized test against real CPU).
  - D6.3 F\* spec coverage (% T1 with consumer-facing `ensures` or SMTPat).
  - D6.4 Audit consistency (% T1 where F\* spec is mechanically consistent with tested model).
  - D6.5 F\* spec proven (% T1 where F\* obligation is discharged not admitted; deferred to unification plan).
- **W5 — TCB validation** (work axis, alongside W1 Specs / W2 Proofs / W3 Annotations / W4 Tests):
  - W5a: Rust intrinsic-model LOC (`crates/utils/core-models/src/core_arch/`).
  - W5b: differential-test count (`mk!` invocations).
  - W5c: audit-script LOC (`crates/utils/core-models/scripts/`).

Update `libcrux-ml-kem/proofs/initial-retrospective.md` to add the **TCB strength baseline**
(2026-05-02 snapshot, pre-sprint):

| Dimension | Today | Target after sprint |
|---|---:|---:|
| D6.1 Rust-model coverage | ~28% (≈55/193) | 100% |
| D6.2 Test coverage | ~26% (≈50/193) | 100% |
| D6.3 F\* spec coverage | ~100% (193/193 via `_extract.rs` ensures + Spec.Intrinsics.fsti SMTPats) | 100% |
| D6.4 Audit consistency | 0% | 100% |
| D6.5 F\* spec proven | 0% | 0% (deferred) |

These numbers are reproducible by re-running the audit script in Step 1.

### 3. Lineage cleanup notes (already executed)

- The `trait-opacify` branch is **obsolete** as of 2026-05-02. Spec.MLKEM removal
  was completed independently on `libcrux-ml-kem-proofs` via a different route.
- The `Hacspec_ml_kem.{Math,Encode,Sample,Cca,Cpa,NttSpec,Instances}.fst` modules
  added on `trait-opacify` (~408 LOC across commits 304a196fb..4e0093ff7) are
  **discarded** — placed at the wrong path (`libcrux-ml-kem/proofs/fstar/commute/`
  rather than `specs/ml-kem/proofs/fstar/commute/`) and the underlying citation
  migration was achieved differently on the canonical branch.
- Three retrospective commits (`b20b09862`, `daeffd891`, `7f549b318`) WERE
  cherry-picked from `trait-opacify` to `libcrux-ml-kem-proofs` on 2026-05-02
  to bring the retrospective files forward. The remaining 18 proof commits
  on trait-opacify are not carried forward.

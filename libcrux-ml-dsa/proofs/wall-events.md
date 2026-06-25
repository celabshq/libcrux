# Wall events

Cross-cutting audit log: cross-branch checks, scope rulings, and other
events worth recording outside the per-sprint logs.

## 2026-05-02 — Class B sprint outcome + Montgomery sprint plan

**Trigger**: Class B parallel-agent sprint
(`handoff-2026-05-01-class-b-bounds.md`) closed partial.

**Outcome**:

* **Chain 1** (sample.rs / samplex4.rs ensures-only) ✅ landed at
  tip `140702ee2`.  Three commits, F* clean, cargo test 20/20.
* **Chain 3** (matrix / arithmetic body proofs) ❌ blocked by
  hax slice-bounds tactic limitations on dual-mutable-slice
  patterns.  Reverted; only diagnostic status log committed
  (`2054c9f15`).
* **`generate_key_pair` panic_free flip**: deferred.

**Cause analysis**: `power2round_vector` and
`compute_as1_plus_s2` have body shapes the hax slice-bounds
tactic doesn't handle (dual `&mut [T]` access; multi-step
length-equality combination in nested loops).  These are
**hax tooling limitations**, not proof-design problems.

**Strategic pivot**: tighten the upstream
`Operations::invert_ntt_montgomery` trait post (currently
`is_i32b FIELD_MAX`, ~2× looser than empirical
`q/2 + 13` per saturation probe).  After tightening, the
consumer body proofs become simpler and dodge the tactic
edge cases.

**New artefacts**:

* `proofs/handoff-2026-05-02-mont-bound-foundation.md` —
  two-sprint plan (collaborative spec foundation, then
  autonomous propagation).
* `proofs/agent-status/agent-trait-pattern-audit-2026-05-02.md`
  — read-only audit of all 27 `Operations` trait methods
  against the 3-part pattern (5/27 compliant, 8 drive-by
  candidates, 10 follow-up candidates).
* `proofs/agent-status/agent-arith-bound-status.md` — full
  Chain 3 diagnostic.
* New memory rule: `feedback_proof_debug_budget.md` —
  30–60 min hard cap per function on proof-debug.

**Empirical findings** (committed in worktree branches as
auxiliary regression tests):

* `tests/montgomery_mul_const_probe.rs` — sweep of
  `montgomery_multiply_by_constant(_, 41_978)` over
  `[-256·FIELD_MAX, 256·FIELD_MAX]`.  Max output
  `4_211_051` (= q/2 + ~21k), matching analytic bound
  `|result| ≤ q/2 + ⌈|value|/2³²⌉ + 1` from ML-KEM's
  documented Montgomery bound at
  `libcrux-ml-kem/src/vector/portable/arithmetic.rs:343-348`.
* `tests/power2round_boundary_probe.rs` — `power2round_element`
  matches FIPS 204 exactly inside `|t| < q`; deviates by
  ±1023 in `t1` for `t ∈ [q, q+4]` (mod-q reconstruction
  still holds).  Confirms the trait pre is genuinely
  binding, not a tightening.

**Branch state**: `ml-dsa-proofs` at `2054c9f15`, 8 commits
ahead of origin, working tree clean.  Not pushed.

---

## 2026-05-01 — cross-branch-audit: rust-spec ml-dsa commits

**Trigger**: agent message in `/tmp/agent-msg-ml-dsa.md` flagged three
ml-dsa-tagged commits on the `rust-spec` branch (pre-fork, 2026-03-18)
as candidate cherry-picks — specifically `8b7a38189` "ml-dsa with some
opaques", on the hypothesis that it might contain opacity scaffolding
analogous to `Hacspec_ml_kem.ModQ` that the current `Hacspec_ml_dsa`
layer is missing.

**Commits inspected**:
- `8b7a38189` — "ml-dsa with some opaques" (Mar 18, 1985 LOC across 15 files under `specs/ml-dsa/`).
- `ed88c27d8` — "ml-dsa" (Mar 18, polish pass, ~900 LOC delta).
- `6a651a62c` — "ml-dsa" (Mar 18 — Mar 23, third revision).

**Method**: grep each commit's `specs/ml-dsa/` content for
`opaque_to_smt`, `ModQ`, `hax_lib::`, `fstar!`, `[@@`, `opaque`,
`reveal_opaque`. Sampled `arithmetic.rs` and `polynomial.rs` raw
content directly.

**Result**: 0 / 0 / 0 matches across all three commits. The "opaques"
in the commit message refers to Rust `pub(crate)` API visibility, NOT
F* `opaque_to_smt` trait-opacity. Files are vanilla Rust without any
F* annotations.

**Decision**: no cherry-pick. The current `Hacspec_ml_dsa` layer
(4,318 LOC, trait-opacity, `opaque_to_smt` lane-post predicates,
`Spec.MLDSA.{Math,Ntt}` interface) is strictly more advanced than the
rust-spec prototype. Per AP-1 (no big-axiom-bridge), wholesale-import
of an older spec would regress the current verification effort.

**Branch posture**: leave `rust-spec` untouched as a historical
reference. Do not pursue further audits on this branch unless the
content gap is specific and concrete.

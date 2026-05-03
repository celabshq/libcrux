# Stream 0 Status — 2026-05-03

## U-task 5: op_ntt_layer_1_step — BLOCKED (known Z3 issue)

**File**: `libcrux-ml-kem/proofs/fstar/extraction/Libcrux_ml_kem.Vector.Portable.fst` (untracked, gitignored)
**Function**: `op_ntt_layer_1_step`
**Status**: retained `--admit_smt_queries true`

### What was tried this session

Three approaches:
1. `p_layer_1` lambda + `assert (forall4 p_layer_1)`: Individual branch asserts succeed (1.8–3.5 s each), but the final `assert (forall4 p_layer_1)` (query 61) hits rlimit 200 after 37 s.
2. Direct `assert (ntt_layer_1_step_branch_post b ...)` without lambda: Query 25 fails at rlimit 200 after 39 s.
3. `p_layer_1 b` assert followed immediately by `assert (branch_post b ...)`: Same failure on the `branch_post` assertion.

### Root cause

Confirmed by `Commute.Chunk.fst` lines 1189–1208 (already documented):

> "Per-branch wrappers were attempted here (Phase 6 follow-up agent A2) but Z3 still hangs even with `b` literal: revealing `ntt_layer_1_step_branch_post` exposes the if-ladder `let z = if b = 0 then zeta0 else if b = 1 then ...`, and Z3 case-splits even when the outer `b` is a literal."

The `4 * b` multiplication in `ntt_layer_1_step_branch_post`'s body triggers Z3's QI+LIA solver interaction. The reveal axiom instantiation for concrete `b` requires evaluating the nested ite (zeta selection) and linear arithmetic (`4*b`, `4*b+2`, etc.) simultaneously. Z3 saturates even at rlimit 200.

Layer 2 (`op_ntt_layer_2_step`) works because its branch_post uses conditional arithmetic (`base + off`) without multiplication — pure EUF evaluation, no LIA cross-layer.

### Fix paths (future work)

1. **Rewrite `ntt_layer_1_step_branch_post`** to accept zeta as a parameter (caller selects, no if-ladder in body). Invasive — requires Traits.Spec changes + downstream cascade.
2. **Use F* tactics**: `assert_norm` / `Tactics.compute` to eagerly reduce the if-ladder before SMT. Not tried yet; would require tactic syntax in an extraction file.

### Session effort

~60 min on U-task 5, hitting the documented hard cap. Moved on to U-tasks 4 and fstar-mcp fixes.

# Sprint 2026-06-02 — Portable NTT layer-1 admits closed (rollup)

## Result
The **two `--admit_smt_queries true` admits in `vector/portable.rs`** are closed,
fully verified (functional FE-algebra ensures, not just panic-free):
- `op_ntt_layer_1_step`   (was portable.rs:447, lax since Phase-6 2026-04-27)
- `op_inv_ntt_layer_1_step` (was portable.rs:686, "admitted by analogy")

`Portable` backend now has **0 lax** functions (was 2). The two moved Lax→Math.

## Builds (all green, no --admit_except)
- `check/Hacspec_ml_kem.Commute.Chunk.fst`: exit 0, 277 s (8 new per-branch lemmas).
- `check/Libcrux_ml_kem.Vector.Portable.fst`: exit 0, 122 s (whole module, real VCs
  for both wrappers + all 62 siblings). Tainted `--admit_except` .checked removed first.

## What changed (3 git-tracked files)
1. `specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Chunk.fst` (+92 lines,
   purely additive): 8 concrete-`b` lemmas
   `lemma_{,inv_}ntt_layer_1_step_branch_{0,1,2,3}`. Each:
   `reveal_opaque` the opaque `{,inv_}ntt_layer_1_step_branch_post` at the literal-`b`
   application + one call to the pre-existing `lemma_{,inv_}ntt_layer_1_butterfly_to_fe`.
   Options `--z3rlimit 100 --fuel 0 --ifuel 1`. No SMTPat → no e-matching pollution.
2. `libcrux-ml-kem/src/vector/portable.rs`: both wrappers — dropped the admit, set
   `--z3rlimit 400 --fuel 0 --ifuel 1 --split_queries always`, replaced the local
   `p_layer_1`/`p_inv_1` predicate-with-conditional + 8 `lemma_butterfly_pair_commute`
   + 4 `assert (p_layer_X b)` with 4 branch-lemma calls + `forall4`. Stale failure-
   narrative comments removed.
3. `libcrux-ml-kem/proofs/fstar/extraction/Libcrux_ml_kem.Vector.Portable.fst`:
   re-extracted (`python3 hax.py extract`), only this .fst changed.

## Why the old approach cliffed (root cause, confirmed by git history)
Layer-1's per-branch predicate body has a **4-way zeta `if`-ladder**
(`if b=0 then zeta0 .. else zeta3`). Asserting `p_layer_1 b` inline made Z3
case-split the ladder for every one of the 16 butterfly FE facts in scope → one
sub-query ran >10 min at rlimit 800 (2026-04-27). Layers 2/3 verify inline because
they have 2-way / single-zeta dispatch. Fixing `b` to a literal in a standalone
lemma collapses the ladder to one branch in a clean SMT context (no cross-branch
fact pollution) — the recipe in `feedback_layer2_branch_post_z3_unlock`. The
`_butterfly_to_fe` helper that does the actual Mont/FE arithmetic already existed.

## Phase-8 regression gate
- portable.rs: `--admit_smt_queries`/`admit ()`/`lax` count 2 → **0**.
- Chunk.fst: `= admit ()` count 2 → 2 (both pre-existing, far above the insertion;
  diff adds no admit/assume). Net admits **−2**.
- `verification_status.md` regenerated: Portable lax 2→0, Math 45→47.

## No downstream rebuild needed
Both wrappers' trait posts (`ntt_layer_1_step_post` / `inv_ntt_layer_1_step_post`)
are unchanged — only the proof bodies changed. In-module trait-dispatch call sites
were re-verified by the full Portable build. External consumers see only the
(unchanged) post. Chunk.fst additions carry no SMTPat, so no e-matching regression.

## Not committed
3 staged files awaiting user OK. Suggested message:
"agent-mlkem: close portable NTT layer-1 admits (op_{,inv_}ntt_layer_1_step, 0 admits)".

## Remaining NTT-leaf SIMD lax (not in this sprint's scope)
- AVX2 `vector/avx2/ntt.rs`: `inv_ntt_layer_1_step` (line ~204), `ntt_multiply`
  (~338) — both lax with NO ensures (need spec design first).
- AVX2 `vector/avx2.rs:1057` `op_serialize_1` (serialize, not NTT).
- Neon backend: all-lax by policy.
- Generic `ntt.rs:564` `ntt_vector_u` `--admit_smt_queries true` (forward-NTT driver;
  closed on branch `agent/fwd-ntt-mirror-inverse` @ e6fde3497, not merged into this branch).

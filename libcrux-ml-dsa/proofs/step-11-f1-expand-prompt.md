# Step 11 — F-1 expand: prove use_hint math + apply template to decompose / compute_hint

Resuming the ML-DSA below-trait proof lane on branch `ml-dsa-proofs` in
`/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa`.  Step 10 closed
in three commits:
- `202686c56` Track A — impl posts hardened to discharge cherry-picked
  array-level conjuncts (incl. new `lemma_power2round_t1_bound`).
- `f07720f2b` Track B — 5 non-trivial impl bodies extracted to
  one-line `*_with_proof` wrappers.  AVX2 `impl_1` 46s/637q → 4.5s/1q.
- `ef99e756c` F-2 sync (canonical `decompose` post shape per above-trait
  `36fe89b18`) + F-1 use_hint Portable scaffolding (paired commute
  lemmas with `admit ()` math bodies; structural shape final).

Tip: `ef99e756c`.  Empirical baseline: **98 invoked / [CHECK]=40 /
[ADMIT]=58 / 0 errors / 0 make-level failures**.

Read in this order before doing anything else:

1. `libcrux-ml-dsa/MLDSA_STATUS.md` — current baseline + branch state.
2. `libcrux-ml-dsa/proofs/agent-status/lane-split-protocol.md` — F-1
   and F-2 are RESOLVED; the verdict is option (d) (no contract change;
   pair unconditional bound lemma with conditional equation lemma).
3. `libcrux-ml-dsa/proofs/outstanding-admits.md` — see "F-1 use_hint
   Portable impl close — scaffolding only" for the two `admit ()`
   placeholders to discharge.
4. `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
   — bottom of file: `lemma_use_one_hint_bound` and
   `lemma_use_hint_lane_commute_conditional` (both `admit()` bodies).
5. `libcrux-ml-dsa/proofs/agent-status/fstar-perf-top20.md` — Track B
   snapshot.  AVX2 `impl_1` no longer top-5; Portable
   `reduce_with_proof` at 18.86s with rlimit-sat on the unsplit query.
6. `libcrux-ml-dsa/proofs/agent-status/touch-unchanged-checked.sh` —
   USE THIS for the iteration loop:
   ```
   proofs/agent-status/touch-unchanged-checked.sh snapshot
   ./hax.sh extract
   proofs/agent-status/touch-unchanged-checked.sh skip-unchanged
   ./hax.sh prove
   ```
   Saves ~10 min per iteration vs. cold-cache NTT/Invntt cascade.
7. `git log --oneline -5` for recent context.

## Hard rules (binding, inherited)

1. **Do not edit `src/simd/traits.rs` or `src/simd/traits/specs.rs`
   pre/post conjuncts.**  Owned by the above-trait lane.  Cherry-pick
   only.  Trait sha is `36fe89b18`; F-2 is closed unless a new
   stuck-query reappears.
2. **Mismatches go into `lane-split-protocol.md`** under "Open
   findings" (numbered F-3, F-4, …).  Do not silently fix on the impl
   side.
3. **20-min wall-clock per method per impl.**  On overrun, mark
   admitted, document, move on.
4. **Develop locally, upstream specs once** (style guide §9.2): new
   spec/math lemmas go in
   `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
   first; `Spec.MLDSA.Math.*` and `Specs.fst` are read-only on this
   branch.
5. **rlimit ≤ 200** for new commute lemmas; ≤ 300 for impl method
   bodies.  If higher needed, factor.

## Tracks (priority order)

### Track 1 — Replace `admit ()` in the use_hint commute lemmas

Two `admit ()` bodies in `Hacspec_ml_dsa.Commute.Chunk.fst`:

#### 1a. `lemma_use_one_hint_bound` (unconditional bound)

Goal: prove `Spec.MLDSA.Math.use_one_hint (v g) (v r) (v hint) ∈
[0, 4190208 / v g)` for any i32 input.

Spec.MLDSA.Math.use_one_hint (line 104 of
`libcrux-ml-dsa/proofs/fstar/spec/Spec.MLDSA.Math.fst`):
```
let use_one_hint (g r hint) =
  let r0, r1, _ = decompose g r in
  if hint = 0 then r1
  else (if r0 > 0 then (r1 + 1) % (4190208 / g)
        else (r1 - 1) % (4190208 / g))
```

For `hint = 1`: result is `(r1 ± 1) % (4190208/g)` which is
unconditionally in `[0, 4190208/g)` since F\*'s `int %` returns
non-negative values strictly less than the modulus.  A single
`FStar.Math.Lemmas.lemma_mod_lt` should suffice.

For `hint = 0`: result is `r1` from `Spec.MLDSA.Math.decompose`.
Need a sub-lemma analogous to `lemma_power2round_t1_bound` (already
proven in Track A) showing `r1 ∈ [0, 4190208/g)` for any input.
The proof structure mirrors `power2round_t1_bound`:
- `r_q = r % q ∈ [0, q-1]`
- `r_g = mod_p r_q (2g) ∈ (-g, g]`
- `r1 = (r_q - r_g) / (2g)` (or `0` in the `r_q - r_g = q-1`
  special case)
- Case-split on `r_q % (2g) > g`.  In one branch `r1 = r_q / (2g)`
  ≤ (q-1)/(2g); in the other `r1 = r_q/(2g) + 1`, but the corner
  case `r1 = q/(2g)` requires `r_q = q-1` which has `r_q % (2g) = 0`,
  contradiction.  Same edge-case argument as `power2round_t1_bound`.

Estimate: ~30 lines + 30 lines.  Try `--z3rlimit 200`, `--ifuel 1`.

#### 1b. `lemma_use_hint_lane_commute_conditional` (conditional equation)

Under hypothesis `v input ∈ [0, q) /\ (v hint == 0 \/ v hint == 1)`,
prove `Spec.MLDSA.Math.use_one_hint (v gamma2) (v input) (v hint) ==
v (Hacspec_ml_dsa.Arithmetic.uuse_hint (v hint = 1) input gamma2)`.

Both unfold to "decompose r → (r0, r1), then case on hint".  Two
divergences:
1. `Spec.MLDSA.Math.decompose` returns `(r0, r1, bool)` (int triple);
   `Hacspec_ml_dsa.Arithmetic.decompose` returns `(r1, r0)` (i32
   pair).  Need a sub-lemma bridging the two under `v r ∈ [0, q)`
   (Spec uses `mod_p`, Hacspec uses `mod_pm` — slightly different but
   compute the same value when `r ∈ [0, q)`).
2. For `hint = 1, r0 ≤ 0`: Spec computes `(r1 - 1) % (4190208/g)` (int
   `%`, always non-negative); Hacspec computes `(((r1 - 1) %! m) + m) %! m`
   (i32 `%!`, signed; the `+ m` then `%! m` re-corrects).  Both
   produce the same value but proof needs to bridge the two flavors.

Estimate: ~50-80 lines.  Try `--z3rlimit 300`, factor sub-lemmas.

#### Acceptance for Track 1

- Both `admit ()` calls in `Commute.Chunk.fst` replaced with real
  proofs.
- Portable `Operations::use_hint` impl body still verifies (no body
  change needed).
- Empirical baseline still `98/40/58/0`.

### Track 2 — Apply paired-lemma template to `decompose` + `compute_hint` Portable

Mirror the use_hint Portable scaffolding (commit `ef99e756c`) for
the two remaining methods whose body is currently a top-level
`admit ()`:

| Method | Underlying free fn | Bound to discharge | Conditional equation |
|---|---|---|---|
| `decompose` | `arithmetic::decompose` post: per-lane `(r0, r1) == Spec.MLDSA.Math.decompose ...` | gamma2-conditional `is_i32b_array_opaque 95232 (resp. 261888) low_future` + `... 44 (resp. 16) high_future` | `decompose_lane_post`: under `v input ∈ [0, q)`, `v low == v r0 /\ v high == v r1` from `Hacspec.decompose` |
| `compute_hint` | `arithmetic::compute_hint` post: per-lane `if Hacspec.make_hint then v hint == 1 else v hint == 0` | `is_binary_array_8_opaque hint_future` (always 0 or 1) | `compute_hint_lane_post`: same shape |

For each:
1. Add `lemma_<method>_bound` (unconditional) and
   `lemma_<method>_lane_commute_conditional` (`introduce ... with hyp.`)
   to `Commute.Chunk.fst`.  Bodies can start `admit ()`; Track 1's
   math may already provide reusable pieces (decompose r1 bound is
   the same lemma as use_one_hint's hint=0 case).
2. Replace the impl body's top-level `admit()` with paired
   `Classical.forall_intro` calls + opaque reveals (mirror
   the portable.rs `use_hint` block from `ef99e756c`).
3. Run extract+prove with the touch-unchanged trick.

Acceptance: Portable `decompose` and `compute_hint` impl bodies are
no longer single `admit()`s; structurally complete with named
commute-lemma calls.  Math admits remain in `Commute.Chunk.fst`
lemma bodies (acceptable for this step; future track replaces them).

### Track 3 (stretch, only if 1+2 finish in budget) — AVX2 use_hint / decompose / compute_hint

These are blocked by the AVX2 `arithmetic::*` free fns being under
`#push-options "--admit_smt_queries true"` (e.g. `uuse_hint` at
`Libcrux_ml_dsa.Simd.Avx2.Arithmetic.fst:545-547`).  To close AVX2
impl bodies you need to:
1. Strengthen the AVX2 free fn post (drop `admit_smt_queries`, add
   per-lane `to_i32x8 ... == Spec.MLDSA.Math.use_one_hint ...`
   conjunct).  Mirrors the AVX2 `add`/`subtract`/`reduce` work in
   Steps 7-8.
2. Then apply the F-1 paired-lemma template (Track 2 shape).

Pre-budgeted as substantial.  Skip if Track 1 + Track 2 take the
full session.

### Track 4 (further stretch) — Step 9.6 AVX2 montgomery_multiply

Self-contained, no F-1 dependency.  Needs
`lemma_mont_mul_bound_and_mod_q` in `Commute.Chunk.fst` (~70-line
ML-KEM-style proof; mirror `lemma_mont_mul_red_i16_int` in
`libcrux-ml-kem/proofs/fstar/spec/Spec.Utils.fst:505` with q=8380417,
R⁻¹=8265825, shift=32).  Then bridge AVX2 free-fn post to trait post.
See Task #4 in the previous session's tasks.

## Pre-flight

```bash
cd /Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa
git rev-parse HEAD                                # tip: ef99e756c
git status                                        # should be clean
proofs/agent-status/touch-unchanged-checked.sh snapshot
JOBS=2 ./hax.sh extract
proofs/agent-status/touch-unchanged-checked.sh skip-unchanged
JOBS=2 ./hax.sh prove 2>&1 | tail -22             # baseline 98/40/58/0
proofs/agent-status/extract-fstar-perf.sh 20      # post-baseline perf
```

## What success looks like

- End-of-session baseline still `0 errors / 0 make-level failures`.
- Track 1: both `admit ()` bodies in `Commute.Chunk.fst` lemmas
  replaced; Portable `use_hint` impl body still verifies.
- Track 2: Portable `decompose` and `compute_hint` impl bodies
  upgraded from single `admit()` to paired-lemma scaffolding (math
  in lemma bodies may still admit for now).
- Track 3 (stretch): one or more of AVX2 use_hint/decompose/compute_hint
  closes via free-fn post strengthening + F-1 template.
- New perf snapshot in `fstar-perf-top20.md` if anything moves.

## What this is NOT

- Not a trait pre/post change.  Owned by above-trait lane.
- Not a `Specs.fst` edit.  Cherry-pick only.
- Not a `Spec.MLDSA.Math` rewrite.  All bridging lemmas go in
  `Commute.Chunk.fst` first per style guide §9.2.
- Not a Portable `reduce_with_proof` perf fix.  The 18.86s with
  rlimit-sat on the unsplit query is a known issue (Track B
  snapshot); future work, not blocking.

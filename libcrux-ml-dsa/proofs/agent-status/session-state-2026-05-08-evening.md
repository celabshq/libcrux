# Session state — 2026-05-08 evening

Handoff snapshot for a fresh agent session.  Written to summarize a long
working session that produced an audit + multiple performance experiments
+ a Track B refactor on AVX2.  Branch: `ml-dsa-proofs`.  HEAD at session
start: `9b5b75b4b`.

## Confirmed wins kept (uncommitted; user will commit if final prove clean)

1. **`Operations::montgomery_multiply` post: dropped third clause**
   (audit Phase A item 10 — the bare `forall i<8. ... == Spec.MLDSA.Math.mont_mul ...`).
   Real abstraction win.  k!61 cascade source dropped 16% (100K→85K
   instances).  q522 of `Avx2.impl_1` got 35× faster (6.6s→185ms).
   Cliff at q1 persisted but for unrelated reasons (monolithic VC, addressed below).
   Sites edited: `src/simd/traits.rs`, `src/simd/avx2.rs` (impl),
   `src/simd/portable.rs` (impl).  Free-fn posts (avx2/arithmetic.rs,
   portable/arithmetic.rs, portable.rs::montgomery_multiply_with_proof)
   retain mont_mul — they live below the trait boundary.

2. **`Ml_dsa_generic::generate_key_pair` body re-admitted** + dropped its
   `requires(signing_key.len() == SIGNING_KEY_SIZE && ...)` clause.
   Restores the cold-cache build to clean state.  The keygen-cone
   opacification scaffolding (commits `c4fe50bd3` → `9b5b75b4b`) stays
   in tree as future scaffolding.  Site: `src/ml_dsa_generic.rs:54-100`.

3. **Track B refactor on `impl Operations for AVX2SIMDUnit`** (applied
   by spawned agent, status doc `proofs/agent-status/agent-track-b-avx2-status.md`).
   8 non-trivial methods extracted into `*_with_proof` free functions
   matching the Portable convention.  Cold-cache prove clean
   (85/85 modules, 0 errors).  Avx2.impl_1 dropped 11.8s → 9.06s
   (with `--split_queries always`); +1.93s for the new
   `decompose_with_proof` top-level VC.  Net slight improvement +
   abstraction uniformity with Portable.  Sites edited:
   `src/simd/avx2.rs` extensively (look for `_with_proof` free-fn
   definitions + dispatcher one-liners in the impl block).

4. **`Spec.Utils.forall8` / `forall32` uniformity** in trait pre/post +
   matching impl posts + Track B `*_with_proof` free fns.  Replaces
   `forall (i:nat). i < N ==> P i` with `forallN (fun i -> P i)` (a
   transparent macro that unfolds to N-way conjunction).  Sites edited:
   - `src/simd/traits.rs`: 7 sites (shift_left_then_reduce pre, t1_serialize pre,
     ntt pre+post, invert_ntt_montgomery pre+post, reduce pre+post)
   - `src/simd/avx2.rs`: 5 sites mirrored in impl + with_proof free fns
   - `src/simd/portable.rs`: 5 sites mirrored similarly

   Verdict pending the in-flight prove.  See "Open verdict" below.

## Confirmed reverts (uncommitted; rolled back during session)

1. **`--split_queries always`** on `impl Operations for AVX2SIMDUnit` and
   `impl Operations for Coefficients` and `reduce_with_proof`.  Worked
   pre-forall8/32 (Avx2 57.4s→11.8s, Portable 39.5s→0.52s,
   reduce_with_proof 17.8s→1.7s — see Snapshot 2026-05-08b).  Reverted
   when forall8/32 cleanup landed because forall8/32 + `--split_queries always`
   composes badly: each conjunct of the N-way conjunction becomes a
   separate sub-query, 32× more queries, 3-30× slowdown observed.

2. **`mont_mul` clause already-applied edits to free-fn `*_with_proof` posts
   converted to `forall8`** (kept; not a revert) — these stay since
   below-trait posts can carry the mont_mul clause.

## Open verdict (pending in-flight prove)

The current uncommitted state has:
- Track B for AVX2 (kept)
- mont_mul clause drop (kept)
- forall8/32 cleanup uniformly applied across trait + both impls + all `_with_proof` free fns (kept)
- `--split_queries always` REMOVED from all 3 sites (reverted)

Cold-cache prove launched at session end:
`/tmp/forall-no-split-prove.log` — running as of session-state file write.

**Expected outcome** (based on prior data):
- impl_1 totals likely between 11.8s (split-only) and 38s (forall+split):
  the forall8/32 conjunction + non-split path may handle it as one
  monolithic VC.  May regress relative to bare-forall+split (the
  pre-cleanup Snapshot 2026-05-08b shape).
- 3 above-trait consumers (`Encoding.T1::serialize`, `Encoding.Error::deserialize_to_vector_then_ntt`,
  `Ntt::reduce`) may stay rlimit-sat (they were rlimit-sat under
  forall+split).  Or may improve if non-split lets Z3 batch-prove.

**If the prove fails**: the user wants to commit current state; "fail"
likely means "verifies but with regressions".  We will commit anyway
and the next session decides next steps.

**If the prove succeeds**: commit current state.

## What was NOT done (open items for next session)

### Tier 1 (post-experiment-driven, not in original audit)

1. **3 hard-fail-without-hints functions** still hint-reliant:
   - `Simd.Portable.Encoding.Gamma1::deserialize_when_gamma1_is_2_pow_17_`
   - `Matrix::compute_matrix_x_mask`
   - `Simd.Portable.Encoding.T0::deserialize`
   All already use `--split_queries always` per their existing fstar::options.
   Different remediation needed: factor lemmas, smaller per-conjunct
   queries, possibly poly-level opacity in encoding posts.
   Reference: `proofs/agent-status/hint-deletion-experiment-2026-05-08.md`.

2. **Un-admit `generate_key_pair` body** + re-prove.  See if q60
   cliff resurfaces post-Track-B.  If clean, the audit's items 25-27
   (ntt/invert_ntt/reduce poly-forall opacity) are not needed.  If
   q60 cliff persists, profile (per audit Phase E.1 recipe) to identify
   the new dominant Skolem.

### Tier 2 (audit cleanup — abstraction wins, may not be perf wins)

3. **`rejection_sample_*` posts** still use bare
   `forall (i:nat{i < Seq.length out_future}). i < v $result ==> ...`
   (variable-length slice prefix; can't use forall8/32).  Audit items
   13–15: introduce `rejection_sample_count_post (out: t_Slice i32)
   (count: usize) (lo hi: i32) : prop` opaque pred.  Three call sites
   in `traits.rs` + matching impls.

4. **`reduce_lane_post` / `montgomery_multiply_lane_post` /
   `shift_left_then_reduce_lane_post` raw `%`/`*` in bodies** (audit
   items 4, 8, 9).  Introduce `mod_q_eq` opaque pred (per
   `trait-correctness-post-design-draft.md`).  Cleanup-tier; the
   lookup lemmas are manual-call-only (no SMTPats), so leak risk is
   currently zero.

### Tier 3 (audit Phase A items demoted post-experiments)

5. **Items 25–27** (`ntt`/`invert_ntt`/`reduce` trait poly-forall
   opacity) — the audit's headline candidate.  Demoted because the
   Avx2.impl_1 cliff they were hypothesized to cause is fixed by
   split_queries+Track B (when applied).  Re-evaluate after un-admit
   (item 2 above).

### Tier 4 (other open items, separate sprints)

6. **7 in-spec `assume()` clauses** in `Hacspec_ml_dsa.Ml_dsa.fst` —
   Sprint B work.
7. **`lemma_mont_red_mod_q`** and **`lemma_decompose_spec_eq_decompose`**
   admits — separate user-lane sprints.

## Performance snapshots in this branch

- `proofs/agent-status/fstar-perf-top20.md` — historical perf table.
  Most recent snapshots:
  - **Snapshot 2026-05-08** (cold-cache baseline before remediation)
  - **Snapshot 2026-05-08b** (after `--split_queries always` on
    impl_1 + reduce_with_proof) — the headline Track-B-companion win
    on Portable.
  - Track B for AVX2 + forall8/32 + no-split snapshot will be
    Snapshot 2026-05-08c (TBD; the in-flight prove will produce data
    for it; the next session should update the file with that
    snapshot).

## Other docs added this session

- `proofs/agent-status/abstraction-boundary-audit-2026-05-07.md` —
  the master audit doc.  27 trait methods + 16 predicates + 22
  SMTPat lemmas + 103 above-trait foralls audited.  Top-15 risk
  register + k!63 cascade hypothesis (partially refuted by
  experiments).

- `proofs/agent-status/qi-baseline-2026-05-08.md` — qi.profile
  baseline of 6 hot/borderline queries.  Pre-Track-B; will be
  outdated for Avx2.impl_1 (Track B changed query structure).

- `proofs/agent-status/hint-deletion-experiment-2026-05-08.md` —
  comprehensive no-hints fingerprint with 3 hard-fail functions.

- `proofs/agent-status/agent-track-b-avx2-status.md` — the spawned
  agent's status doc; details of which methods were extracted vs
  skipped.

- `~/.claude/skills/fstar-for-libcrux/SKILL.md` — Track B section
  consolidated with cross-backend evidence (ml-kem `op_*` + ml-dsa
  `_with_proof`) and the `--split_queries always` companion rule.
  This was edited by another agent during the session.

## Key learnings from this session

1. **`--split_queries always` is a powerful win against monolithic
   impl-block VCs** (557 → 4 queries on Portable.impl_1 with Track B
   already in place; -79% on Avx2.impl_1 even without Track B).

2. **`forall8`/`forall32` macros DO NOT compose with
   `--split_queries always`.**  Each macro unfolds to an N-way
   conjunction; with split_queries, each conjunct is a separate
   sub-query.  3-30× slowdown observed.  Choose ONE: bare-forall +
   split_queries (current Snapshot 2026-05-08b), or forall8/32 +
   monolithic (current uncommitted).

3. **Cold-cache full prove vs targeted clearing**: F* / make handles
   staleness incrementally per `feedback_no_cache_nuke`.  Manual
   `rm -f *.checked` is overkill except when you specifically need
   apples-to-apples timings vs a saved snapshot.  Default: just run
   `JOBS=4 ./hax.sh prove` and let make figure out what's stale.

4. **The audit's #1 cliff hypothesis (trait poly-foralls →
   consumer cascades) was partially wrong.**  The Avx2.impl_1 cliff
   was a monolithic-VC issue, not opacity per se.  `--split_queries
   always` alone fixed it without any opacity changes.  This means
   the audit's Phase A items 25-27 (ntt/invert_ntt/reduce poly-forall
   opacity) are demoted — only re-promote if un-admit-keygen test
   fails.

5. **Three functions hard-fail without hints**, regardless of
   Track B / split_queries / forall8/32: `T0::deserialize`,
   `compute_matrix_x_mask`, `Gamma1::deserialize_when_gamma1_is_2_pow_17_`.
   These have intrinsic structural complexity (per-byte assertion
   chains, large per-iteration ambient context).  Need lemma-factoring
   work, separate from this session's experiments.

6. **Don't trust a stale background log without rerunning.**  An
   intermediate measurement that showed Track B regressed AVX2 was
   stale-state pollution (other worktrees doing parallel work).
   Fresh cold-cache rerun showed Track B was ~neutral on AVX2.

## Working tree state at session end

```bash
$ git status libcrux-ml-dsa/src/
# (uncommitted edits)
M libcrux-ml-dsa/src/ml_dsa_generic.rs           # admit + drop requires
M libcrux-ml-dsa/src/simd/avx2.rs                # Track B + mont_mul drop + forall8/32 + revert split_queries
M libcrux-ml-dsa/src/simd/portable.rs            # mont_mul drop + forall8/32 + revert split_queries
M libcrux-ml-dsa/src/simd/traits.rs              # mont_mul drop + forall8/32

$ git status libcrux-ml-dsa/proofs/
M libcrux-ml-dsa/proofs/verification_status.md   # auto-regenerated; admits +1
M libcrux-ml-dsa/verification_result.txt         # latest cold-cache prove
?? libcrux-ml-dsa/proofs/agent-status/abstraction-boundary-audit-2026-05-07.md
?? libcrux-ml-dsa/proofs/agent-status/agent-track-b-avx2-status.md
?? libcrux-ml-dsa/proofs/agent-status/hint-deletion-experiment-2026-05-08.md
?? libcrux-ml-dsa/proofs/agent-status/qi-baseline-2026-05-08.md
?? libcrux-ml-dsa/proofs/agent-status/session-state-2026-05-08-evening.md  # this file
?? libcrux-ml-dsa/proofs/agent-status/agent-asymmetric-opaque-experiment.md  # pre-session
?? libcrux-ml-dsa/proofs/agent-status/agent-bridge-cascade-experiment.md  # pre-session
?? libcrux-ml-dsa/proofs/agent-status/power2round-refactor-decision.md  # pre-session
?? libcrux-ml-dsa/proofs/agent-status/trait-correctness-post-design-draft.md  # pre-session
```

`fstar-perf-top20.md` was updated with Snapshot 2026-05-08b but the
post-Track-B + forall8/32 snapshot needs to be appended by the next
session once the in-flight prove completes.

## Backups in `/tmp/`

- `/tmp/hints-backup-2026-05-08/` — original `.hints` from session start
  (before hint-deletion experiment)
- `/tmp/hints-backup-iter3-2026-05-08/` — hints after iter3 of hint experiment

These are preserved across session boundaries (system /tmp, not
session-scoped).  Restore by `cp -f /tmp/hints-backup-2026-05-08/*.hints
/Users/karthik/libcrux-ml-dsa-proofs/.fstar-cache/hints/` if needed.

## Resume checklist

To pick up:

1. Read this file (you're doing it).
2. Read `proofs/agent-status/abstraction-boundary-audit-2026-05-07.md`
   sections 1 (TL;DR) and Phase E (risk register + remediation).
3. Read `proofs/agent-status/fstar-perf-top20.md` Snapshot 2026-05-08b
   (and 2026-05-08c when written).
4. Read `proofs/agent-status/hint-deletion-experiment-2026-05-08.md`
   Cross-cut summary section.
5. `git log -10 ml-dsa-proofs` to see what was committed.
6. `git diff` against HEAD if anything is uncommitted.
7. Decide: continue Tier 1 work (un-admit + 3 hard-fails) or Tier 2
   cleanup (rejection_sample, mod_q_eq).

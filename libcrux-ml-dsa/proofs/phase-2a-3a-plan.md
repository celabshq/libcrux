# Phase 2A + 3A — Parallel Execution Plan

**Branch**: `ml-dsa-proofs` (resume from commit `7be6b31b1`)
**Status of Phase 1**: ✅ done — see commit `7be6b31b1` and `MLDSA_STATUS.md`.
**Goal of this checkpoint**: launch Phase 2A (portable arithmetic impl proofs) and Phase 3A wave (i) (AVX2 arithmetic impl proofs) in parallel via subagents, with crash-safe worktree isolation, and merge each wave back as a separate commit.

---

## What was just completed (Phase 1)

13 of 21 Operations trait methods have strong, hacspec-anchored posts at the
trait surface (T=✅ in `MLDSA_STATUS.md`):

- `add`, `subtract`, `zero`, `from/to_coefficient_array` — already strong
  before Phase 1; not touched.
- `infinity_norm_exceeds`, `decompose`, `compute_hint`, `use_hint`,
  `montgomery_multiply`, `shift_left_then_reduce`, `power2round`,
  `error_deserialize`, `t1_deserialize`, `reduce` — new per-lane
  `[@@ "opaque_to_smt"]` posts citing `Hacspec_ml_dsa.Arithmetic.*` /
  `Hacspec_ml_dsa.Encoding.*`.

8 methods have length/bound-only posts (T=🟡), with admit docs in
`outstanding-admits.md`: rejection_sample × 3, gamma1 ser/deser,
commitment_serialize, error_serialize, t0 ser/deser, t1_serialize.

2 methods (`ntt`, `invert_ntt_montgomery`) keep bounds-only posts
(USER-lane admit; per-poly Tier-3 chain).

Verification baseline: 1 F* error (pre-existing
`Libcrux_core_models.Abstractions.Funarr` Error 92), all Phase-1-touched
modules either Checked or cleanly Admitted.

---

## What Phase 2A and 3A do

Phase 2 and 3 land **impl-side proofs**: actual `reveal_opaque` +
`Classical.forall_intro` blocks inside the `impl Operations for X`
methods (and the underlying primitive functions) so F* checks the impl
satisfies the (now strong) trait post.

Both 2A and 3A are mechanical: the lane post + lookup/intro lemmas
already exist in `Libcrux_ml_dsa.Simd.Traits.Specs` (Phase 1).

### Phase 2A scope (single agent)

**Files to edit** (only these):
- `libcrux-ml-dsa/src/simd/portable/arithmetic.rs` — primitive impls.
- `libcrux-ml-dsa/src/simd/portable.rs` — the `impl Operations` delegations,
  if any need `verification_status` adjustments.

**Methods covered**: 4
- `add`
- `subtract`
- `infinity_norm_exceeds`
- `reduce` (delegates to `shift_left_then_reduce::<0>` in a 32-iteration loop)

**Pattern per method**:
1. Read the trait post in `traits.rs` to confirm what conjuncts are
   demanded.
2. Inside the impl body, add `hax_lib::fstar!(r#"reveal_opaque (\`%X_lane_post) ...; let aux (j: nat{j < 8}) : Lemma (...) = ... in Classical.forall_intro aux"#)`.
3. For methods that fold over 32 SIMD units (`reduce`), add a loop
   invariant tracking the per-vector post.
4. Re-extract; run F* on `Libcrux_ml_dsa.Simd.Portable.Arithmetic.fst`.

**Lane posts to cite** (from `Libcrux_ml_dsa.Simd.Traits.Specs`):
- `add_post` / `sub_post` (already opaque from Phase 0; `reveal_opaque`
  unfolds to per-lane integer equality).
- `infinity_norm_exceeds_post` — opaque atom; equivalent to
  `b2t result <==> exists i. v (Hacspec_ml_dsa.Arithmetic.coeff_norm simd_unit.[i]) >= v bound`.
- `reduce_lane_post` — opaque atom; equivalent to
  `0 <= v result < 8380417 /\ (v result) % 8380417 == (v input) % 8380417`.
  Lookup lemma `lemma_reduce_lane_lookup` has dual-trigger SMTPat.

### Phase 3A wave (i) scope (single agent)

**Files to edit** (only these):
- `libcrux-ml-dsa/src/simd/avx2/arithmetic.rs` — primitive impls.
- `libcrux-ml-dsa/src/simd/avx2.rs` — `impl Operations` delegations if
  needed.

**Methods covered**: 4 (the 3A wave (i) row of `sprint-plan.md`)
- `add`
- `subtract`
- `montgomery_multiply`
- `reduce`

NOT in this wave (skip — they are 3A waves ii/iii/iv or 3C):
- `power2round`, `shift_left_then_reduce` (wave ii)
- `compute_hint`, `use_hint`, `decompose` (wave 3C — already lax)
- All encoding methods (waves iii, iv)

**Pattern per method**: same reveal_opaque + Classical.forall_intro,
but with the AVX2 intrinsic-spec bridge layer:
1. Use `vec256_as_i32x8` to lift Vec256 to `t_Array i32 (mk_usize 8)`.
2. Reveal the lane post.
3. Discharge per-lane equation via `Libcrux_intrinsics.Avx2.*` post on
   `mm256_add_epi32` etc.

**Risk**: `montgomery_multiply` may need a Tier-1 commute lemma (analog
to ML-KEM's `Hacspec_ml_kem.Commute.Chunk.lemma_montgomery_*`). If the
20-min budget runs out, mark the impl `verification_status(lax)` and add
to `outstanding-admits.md` as a 3A wave-(i) carry-over.

---

## Coordinator pre-flight (before launching agents)

1. `cd libcrux-ml-dsa && git rev-parse HEAD` — confirm tip is `7be6b31b1`.
2. `git status` — confirm clean working tree.
3. Read `src/simd/portable/arithmetic.rs` and `src/simd/avx2/arithmetic.rs`
   end-to-end so you understand the existing proof scaffolding (push/pop
   options, helper lemmas, bounds-only posts). About 10 minutes.
4. Read `libcrux-ml-kem/src/vector/portable.rs` and
   `libcrux-ml-kem/src/vector/avx2.rs` for the analog reveal_opaque
   pattern in ML-KEM. ~10 minutes.
5. Skim `proofs/proof-style-guide.md` §3.4–3.6 for the
   `Classical.forall_intro` template.

---

## Launching the agents

Use the harness `Agent` tool. Both calls go in **one message** so they
run truly concurrently. Both use:
- `subagent_type: "general-purpose"`
- `isolation: "worktree"`  ← **critical for crash safety**
- `run_in_background: true`  ← coordinator notified on completion;
  no polling

### Agent A briefing (Phase 2A)

```
description: "Phase 2A portable arithmetic proofs"

prompt:
  You are completing Phase 2A of the ML-DSA proof sprint on branch
  ml-dsa-proofs (tip 7be6b31b1). Phase 1 added strong trait posts; your
  job is to discharge those posts inside the portable impl.

  Read first:
  - libcrux-ml-dsa/MLDSA_STATUS.md (status snapshot)
  - libcrux-ml-dsa/proofs/sprint-plan.md §Phase 2 (your wave is 2A)
  - libcrux-ml-dsa/proofs/proof-style-guide.md §3 (the reveal_opaque +
    Classical.forall_intro pattern)
  - libcrux-ml-dsa/src/simd/traits.rs (lines 60–188; trait posts you
    must satisfy)
  - libcrux-ml-dsa/src/simd/traits/specs.rs (lane post predicates and
    their dual-trigger lookup / named-intro lemmas — already in place)
  - libcrux-ml-kem/src/vector/portable.rs (analog template for the
    pattern)

  Scope (touch only these files):
  - libcrux-ml-dsa/src/simd/portable/arithmetic.rs
  - libcrux-ml-dsa/src/simd/portable.rs (only if a delegation needs
    verification_status adjustment)

  Methods to discharge (4):
  - add
  - subtract
  - infinity_norm_exceeds
  - reduce  (delegates to shift_left_then_reduce::<0> over 32 SIMD units)

  Hard rules:
  1. Do NOT edit libcrux-ml-dsa/src/simd/traits.rs or
     libcrux-ml-dsa/src/simd/traits/specs.rs.  Pre-conditions never
     change; post-conditions never change.
  2. Do NOT add new lemmas to traits/specs.rs. If a per-element commute
     lemma is needed, place it in
     specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst
     (new file, fine to create).
  3. 20-minute wall-clock per method.  On overrun: add
     #[hax_lib::fstar::verification_status(lax)] on the impl method,
     append a one-paragraph entry to
     libcrux-ml-dsa/proofs/outstanding-admits.md, move on.
  4. Cite Hacspec_ml_dsa.* only — never Spec.MLDSA.Math or
     Spec.MLDSA.Ntt for new code.

  Status tracking (mandatory):
  - At start, write to /tmp/phase-2a-status.md a header line plus a
    PENDING line per method.
  - After each method, update its line to PROVED, ADMITTED (with
    one-line reason), or SKIPPED.
  - At finish, write a final summary line.

  Build verification:
  - Run ./hax.sh extract from libcrux-ml-dsa/ after edits.
  - Run ./hax.sh prove from libcrux-ml-dsa/. Expect at minimum the
    pre-existing Funarr Error 92 baseline (out of scope) plus any new
    admits you intentionally added. Anything else is a regression.

  Commit cadence:
  - One commit at the end of the wave, on the worktree branch.
  - Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

  Return as your final tool-result output:
  - Methods proved vs admitted (with reasons).
  - Final F* prove summary (the verification_result.txt summary block).
  - Any helper lemmas added (file + names).
```

### Agent B briefing (Phase 3A wave i)

```
description: "Phase 3A wave (i) AVX2 arithmetic proofs"

prompt:
  You are completing Phase 3A wave (i) of the ML-DSA proof sprint on
  branch ml-dsa-proofs (tip 7be6b31b1). Phase 1 added strong trait
  posts; your job is to discharge those posts inside the AVX2 impl.

  Read first:
  - libcrux-ml-dsa/MLDSA_STATUS.md
  - libcrux-ml-dsa/proofs/sprint-plan.md §Phase 3 row 3A
  - libcrux-ml-dsa/proofs/proof-style-guide.md §3
  - libcrux-ml-dsa/src/simd/traits.rs (trait posts you must satisfy)
  - libcrux-ml-dsa/src/simd/traits/specs.rs (lane post predicates and
    lookup / named-intro lemmas — already in place)
  - libcrux-ml-kem/src/vector/avx2.rs (analog template, AVX2 bridge
    pattern)
  - The AVX2 intrinsic posts in libcrux-intrinsics extraction (already
    on F* include path).

  Scope (touch only these files):
  - libcrux-ml-dsa/src/simd/avx2/arithmetic.rs
  - libcrux-ml-dsa/src/simd/avx2.rs (only if a delegation needs
    verification_status adjustment)

  Methods to discharge (4 — wave i only):
  - add
  - subtract
  - montgomery_multiply
  - reduce

  Methods explicitly OUT OF SCOPE for this wave:
  - power2round, shift_left_then_reduce (wave ii)
  - compute_hint, use_hint, decompose (wave 3C — already lax)
  - All encoding methods (waves iii, iv)

  Same hard rules as Agent A (see phase-2a briefing): no edits to
  traits.rs or traits/specs.rs; per-element commute lemmas go in
  specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst;
  20-min budget per method; cite Hacspec_ml_dsa.* only.

  Status tracking (mandatory): write to /tmp/phase-3a-status.md the
  same way Agent A writes to phase-2a-status.md.

  Pre-known risks:
  - montgomery_multiply may need a Tier-1 commute lemma (analog of
    ML-KEM's Hacspec_ml_kem.Commute.Chunk.lemma_montgomery_*). If the
    20-min budget runs out, lax + admit doc is the right move.
  - Two AVX2 methods (compute_hint, use_hint) already have
    verification_status(lax) on their primitive impl — DO NOT remove
    those; they are not part of this wave.

  Build verification: same as Agent A — ./hax.sh extract && ./hax.sh
  prove from libcrux-ml-dsa/.

  Commit cadence: one commit at the end of the wave.

  Return: same structured summary as Agent A.
```

---

## During the parallel window (coordinator)

Don't poll — the harness notifies on completion. Use the time to:
- Pre-draft `MLDSA_STATUS.md` row updates for the 8 methods.
- Read `Hacspec_ml_kem.Commute.Chunk.fst` (if it exists) for the
  Tier-1 commute lemma template both agents may need.
- Skim `libcrux-ml-kem/proofs/manual-proof-targets.md` for the USER-2
  / USER-4 templates that NTT/AVX2-NTT will eventually borrow.

---

## After both agents return (coordinator checklist)

For each agent in turn:
1. `Read /tmp/phase-2a-status.md` (resp. `phase-3a-status.md`) for the
   ground-truth method-by-method outcome.
2. Inspect the agent's worktree:
   - The harness reports the worktree path on completion.
   - `git -C <worktree-path> log -1` to see the commit.
   - `git -C <worktree-path> diff ml-dsa-proofs..HEAD --stat`.
3. Read the actual file edits with `Read`. Trust no summary blindly.
4. If the diff is clean and the agent's claimed prove summary matches
   what you see in the worktree's `verification_result.txt`,
   cherry-pick onto `ml-dsa-proofs`:
   ```
   git cherry-pick <agent-commit-sha>
   ```
5. If the diff has issues, salvage the good parts:
   ```
   git -C <worktree-path> diff ml-dsa-proofs..HEAD -- <good-files> | git apply
   ```

After both waves are merged onto `ml-dsa-proofs`:
6. From `libcrux-ml-dsa/`: `./hax.sh extract && ./hax.sh prove`.
7. Confirm only the Funarr Error 92 baseline plus any documented
   intentional admits.
8. Update `MLDSA_STATUS.md` per-method `P` (portable) and `A` (AVX2)
   columns: ✅ for proved, 🟡 for admitted-with-doc.
9. If new admits, ensure they are documented in
   `proofs/outstanding-admits.md`.
10. Update `verification_result.txt` (regenerated by `./hax.sh prove`).

Final commit on `ml-dsa-proofs`:
```
git commit -m "Phase 2A + 3A: portable + AVX2 arithmetic impl proofs

(summary of methods proved, methods admitted, any commute lemmas added)

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

If both waves cleanly cherry-picked, this final commit is just the
docs update — fine.

---

## Crash recovery procedure

| Failure mode | Recovery |
|---|---|
| Agent's tool result returned with error | Read its worktree's `/tmp/phase-?a-status.md` for partial progress; cherry-pick the methods that succeeded. |
| Agent silent past 110 min | `Bash: ls /tmp/phase-?a-status.md` to see how far it got; manually merge / abandon as needed. |
| Agent crashed before any commit | Worktree filesystem still has its edits; salvage via `git -C <worktree> diff ml-dsa-proofs > /tmp/salvage.patch`, review, `git apply` on main. |
| F* cache corruption in worktree | Worktree-local; doesn't affect main branch. Just ignore that agent's result; relaunch on a fresh worktree. |
| Two agents collided on a shared lemma name in `Commute.Chunk.fst` | Caught at merge step; pick one, rename the other, re-run prove. Briefings tell them not to extend `traits/specs.rs` so name collision is constrained to the new commute file. |
| Cherry-pick conflict on `verification_result.txt` | Discard the agent's version; regenerate from main after both merge by running `./hax.sh prove`. |

---

## Branch hygiene

- `ml-dsa-proofs` is the integration branch. Worktree branches are
  short-lived; cherry-pick onto `ml-dsa-proofs` and clean up.
- Don't rebase or force-push `ml-dsa-proofs`.
- One commit per wave on `ml-dsa-proofs` keeps history linear and
  cherry-pickable for backports.

---

## Sequel: when 2A + 3A are merged

Phase 2 still has waves 2B (5 methods), 2C (1), 2D (5 ser/deser pairs),
2E (3 rejection variants), 2F (full NTT — pre-budgeted partial admit),
2G (invntt — depends on 2F). Phase 3 still has 3A waves ii/iii/iv,
3B, 3C (likely admit), 3D, 3E (pre-budgeted admits).

Recommendation for the next handoff: run **2D + 2E + 3A wave (iii) + 3A
wave (iv)** in parallel — all four are encoding/sampling, file-disjoint,
mechanical. Or alternatively go to **2B + 3A wave (ii)** which are
arithmetic continuation. Pick based on whether commute lemmas added in
2A/3A are reusable.

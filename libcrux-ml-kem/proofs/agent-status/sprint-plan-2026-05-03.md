# ML-KEM proof sprint plan — 2026-05-03

> **⚡ REQUIRED READING before any work:**
> `~/.claude/skills/fstar-for-libcrux/README.md`
> This skill captures the libcrux-specific F* workflow, token discipline rules,
> fstar-mcp usage guide, fsti cascade protocol, and common failure modes.
> It supersedes ad-hoc guidance in earlier sprint plans.

**Tip on entry:** `d916e9563` (is_bounded_polynomial_vector threaded through decapsulate chain; all Mlkem*.Unpacked.fstis updated on disk).

**Backdrop:** See `proofs/agent-status/proof-status-audit-2026-05-03.md` for full surface inventory and the answers to Q1/Q2/Q3.

**Two milestones:**
- **A** — All BELOW-trait fns fully verified w.r.t. Hacspec; all ABOVE-trait fns up to `ind_cca` are panic_free.
- **B** — All ABOVE-trait fns up to `ind_cca` fully verified w.r.t. Hacspec.

This plan targets **Milestone A in 1 week** with a coherent off-ramp into a 4–8 week effort for **Milestone B**.

---

## Capacity model

- Claude usage resets in ~24h; Day 1 runs on a tight Claude budget. Days 2–5 have abundant agent capacity, so we run 3 parallel streams.
- Two roles: **U** (user — high-judgment, Z3-massaging, spec design) and **A** (agent — replication, plumbing, propagation).
- Per-fn budget on agent work is 60 min; the user can blow past this for genuinely hard fns.

---

## Token / usage discipline (READ BEFORE SPAWNING AGENTS)

Symptom we are guarding against: prior multi-agent sprints burned a 5-hour usage allowance in ~1 hour. Every agent briefing in this plan MUST enforce these rules verbatim — they are NOT defaults agents will discover on their own:

1. **Never `Read` a full F* `make` log into context.** Redirect every `make` invocation to a per-stream log file and grep for errors only. Boilerplate to paste into agent prompts:
   ```bash
   make check/<Module>.fst > /tmp/make-<stream>.log 2>&1
   grep -nE '(error|Error|Failed|Cannot|^\(' /tmp/make-<stream>.log | head -50
   # only widen with sed -n 'M,Np' or grep -B/-A around a specific error line
   ```
   Dominant token sink — empirically responsible for the 5x burn rate. Codified in `feedback_grep_make_output`.

2. **Inner loop is fstar-mcp `typecheck_buffer`, not `make`.** Sub-second vs 50-100s. Reserve `make` for end-of-task validation only. Agent must recreate the mcp session after any `make` (per `feedback_fstar_mcp_session_dies_after_make`).

3. **Each agent gets a self-contained briefing.** No "explore the repo to understand X" — provide exact `file:line` targets, the lighthouse pattern to replicate, and the expected diff shape. Re-exploration by 5 parallel agents is the second-largest token sink.

4. **Status reports are 3 lines max:** `sub-task / blocker / ETA`. Per `feedback_agent_status_reports`. Agents narrating their reasoning is pure token cost.

5. **Hard 60-min per-fn budget, enforced via `ScheduleWakeup`.** If an agent has not closed a function by T+60min, it must mark FOLLOW-UP and stop — not loop. Per `feedback_proof_debug_budget`.

6. **Parallelism only when worktrees are file-disjoint.** Two agents touching the same `.fst` file or the same Rust module collide on `.checked` invalidation and re-do each other's work. Streams must be partitioned by file before spawn — per `feedback_branch_means_worktree`.

7. **Read-only exploration → `Explore` agent, not `general-purpose`.** Smaller toolset, no edit-check loop, ~5x cheaper for "where is X / which lemma do I need" questions.

8. **Plan-then-execute split.** For each Stream, ONE Plan/Explore agent produces the punch-list (file:line, lighthouse pattern, expected diff). Then ONE executor agent works the list. Avoids 3 parallel agents independently re-discovering the same context.

If we hit the burn rate again mid-sprint, the first thing to audit is rule #1 compliance in the spawned agent prompts.

---

## What agents reliably do (A) vs what the user should own (U)

| Agents reliably | User should own |
|---|---|
| Replicating a proof pattern across siblings (lighthouse → 3 follow-ons) | Designing new spec-side lemmas where no shape exists |
| Adding ensures/requires/loop-invariant plumbing | Restructuring proofs to escape Z3 quantifier saturation |
| Caller-site bound propagation (multi-file mechanical) | Long-standing USER-tagged blockers (USER-12, USER-13, etc.) |
| Removing `ADMIT_MODULES` entries once the chain is real | Choosing between full-proof-first vs panic-free-first per fn |
| Trait carve-outs when scoped + verified per impl | Reviewing trait carve-outs for soundness |
| Re-extraction + make-check iteration | Z3-massaging with `--split_queries`, `--ext context_pruning`, rlimit tuning |

---

## Day 1 — User-driven (minimal agent usage)

Goal: land in-flight work, set up Days 2–5 by clearing the design-heavy items the user is best at.

### U-tasks (priority order)

1. **Review + merge Stream 1 (`agent-mlkem/phase-f-stream1-ind_cca`)** into `libcrux-ml-kem-proofs`. Should be a fast-forward + run of `make check` on the merged tip. Stream 1 dropped `Libcrux_ml_kem.Ind_cca.Unpacked.fst` from `ADMIT_MODULES` — this is the headline Lane E close.
2. **Review + merge Stream 2 (`agent-mlkem/phase-f-stream2-ind_cpa` at `e7eb32780`)**. Stream 2 result: lighthouse `deserialize_vector` flipped lax → panic_free; other 3 stayed lax with structural blockers (see U-tasks 6 + 7 below).
3. **Verify the `noeq` fix (`d51105087`)** propagated — the `Libcrux_ml_kem.Vector.Neon.Vector_type.fsti` parse error should no longer trigger after fresh re-extraction. Spot-check by running `make check/Libcrux_ml_kem.Mlkem512.Portable.Unpacked.fst` (one of the previously-broken modules).
4. **Draft Phase C bridge lemma signature** — the bridge from `byte_encode 12` over a vector to `serialize_secret_key` (per Phase D residual notes). This is fresh Hacspec-side design; agents struggle with new design. Write the lemma's val signature in `specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Chunk.fst` (or a new sibling); body can be `admit ()` initially and discharged later. Goal: unblock 4 cascade-lax fns (Day 2 agent work).
5. **USER-12 NTT layer 1 attempt** — `op_ntt_layer_1_step` in `Libcrux_ml_kem.Vector.Portable.fst` has been Z3-saturating since Phase 6. The branch-helper refactor pattern is documented in `feedback_layer2_branch_post_z3_unlock`: 4 per-branch concrete-`b` helpers + per-lane wrapper + `--split_queries always` on the per-vector composition. This is exactly the kind of Z3-massaging where agent budget gets blown; user-owning it.
6. **Family A inductive unfolding lemma** (Stream 2 residual) — `serialize_vector` and `compress_then_serialize_u` in `src/ind_cpa.rs` stayed lax because the spec target is `Hax.Folds.fold_range` with `True` post; no `createi_lemma` analogue exists for `Seq.slice (serialize_secret_key K T v) (j*B) ((j+1)*B) == byte_encode v[j]`. Need an inductive unfolding lemma in `Hacspec_ml_kem.Serialize.fst` (or a sibling commute module) that asserts this slice-equality for all `j < K`. Once the lemma lands, both Family A fns flip mechanically (Day 2 agent task).
7. **`ntt_vector_u` functional ensure restoration** (Stream 2 residual) — `deserialize_then_decompress_u` in `src/ind_cpa.rs` stayed lax because `ntt_vector_u`'s functional ensure is commented out at `src/ntt.rs:560-561`. The loop invariant for `deserialize_then_decompress_u` cannot be maintained across the in-place NTT call without it. Restore the ensure (uncomment) and verify `ntt_vector_u` against it; this is spec-side wiring + may surface a small ensures discharge problem. Once landed, `deserialize_then_decompress_u` flips mechanically (Day 2 agent task).

### A-tasks (Day 1) — only if absolutely needed

- One read-only Explore agent to draft the `compress_then_serialize_message` Q2 cancel diagnosis (per Makefile line 49 comment) — only spawn if we have agent budget left after merging Streams 1+2.

### Day 1 exit criteria

- Streams 1+2 merged to `libcrux-ml-kem-proofs`.
- noeq fix verified.
- Phase C bridge lemma signature drafted (body admitted).
- USER-12 either landed or precise blocker documented.
- Decisions made on which Days 2–5 agent streams to spawn.

---

## Day 2 — Agent surge (parallel streams)

Now usage has reset. Spawn 3 parallel agent streams.

### A-tasks (parallel)

- **Stream 2.1 — Pattern-2 cluster (3 fns):** `encrypt_c1`, `encrypt_c2`, `sample_vector_cbd_then_ntt` in `src/ind_cpa.rs`. Each has a specific callee precondition gap; the bound chain we built makes them tractable. Worktree: a fresh `agent-mlkem/phase-f-stream2_1-pattern2`.
- **Stream 2.2 — Phase C cascade replay (4 fns):** Once the user lands the bridge lemma signature on Day 1, spawn an agent to wire `serialize_public_key{,_mut}`, `serialize_unpacked_secret_key`, packed `generate_keypair` and packed `encapsulate` to use the bridge. This is mechanical propagation if the lemma sig is right.
- **Stream 2.3 — Below-trait audit + admit_smt_queries removal:** Sweep `src/vector/portable/*.rs` for `--admit_smt_queries true` push-options that are now stale (Phase 2 opacity work was removed many of these per Makefile comments). Identify which can be dropped.

### U-tasks (Day 2 — light)

- Oversight: review per-fn FOLLOW-UPs as agents finish; arbitrate stay-lax vs more-effort.
- If USER-12 didn't land Day 1, retry with a fresh perspective.

### Day 2 exit criteria

- Pattern-2 cluster: 2-3 of 3 flipped (stretch all 3).
- Phase C cascade: 3-4 of 4 flipped via the bridge lemma.
- Below-trait Portable: residual `admit_smt_queries` count down.

---

## Day 3 — Below-trait close

### A-tasks

- **Stream 3.1 — Portable Compress + Serialize residuals.** Any remaining `admit ()` calls in trait wrappers get real proofs. Pattern: similar to the lemma_bounded_i16_array_intro work for decompress_ciphertext_coefficient on Day -2.
- **Stream 3.2 — Forward NTT layers 4–7 below-trait.** These are panic_free; check if full Hacspec-equivalence is available cheaply (Q3 candidates).

### U-tasks

- USER-13 / similar long-standing tags if any remain.
- Spec design: if the Phase C bridge body needs more lemmas to land, write them.

### Day 3 exit criteria

- Below-trait Portable fully verified end-to-end (Milestone A's "below-trait verified" half).

---

## Day 4 — Milestone A close + Milestone B start

### A-tasks

- **Stream 4.1 — Stragglers from Days 1-3.** Anything that didn't fully close.
- **Stream 4.2 — Milestone B easy fns.** Per Q3 audit, target: `invert_ntt_at_layer_1`, `deserialize_then_decompress_ring_element_v` (already done), small unpacked-API helpers — fns whose loop invariants already establish per-iteration spec equality so panic_free → fully-verified is cheap.

### U-tasks

- Final Milestone A clean make on `libcrux-ml-kem-proofs` mainline.
- Tag `milestone-A-close-2026-05-XX`.

### Day 4 exit criteria

- **Milestone A done.** All below-trait fns verified; all above-trait up to `ind_cca` panic_free.
- ~5 above-trait fns started on full Hacspec ensures.

---

## Day 5 — Milestone B push (`ind_cpa` slice)

### A-tasks

- **Stream 5.1 — `ind_cpa` full Hacspec ensures.** Walk the panic_free fns in `src/ind_cpa.rs` adding `result == Hacspec_ml_kem.ind_cpa.<fn>(...)` ensures and verifying.

### U-tasks

- Spec-design checkpoint: if there's a fn where the Hacspec spec doesn't have a matching shape, decide whether to add the shape (Spec.MLKEM avoidance per `feedback_avoid_spec_mlkem`) or defer.
- Final clean make pass; document any deferred Milestone B fns.

### Day 5 exit criteria

- `ind_cpa` fully verified w.r.t. Hacspec for the panic_free subset that has Hacspec spec functions.
- `ind_cca` Milestone B deferred to longer plan (needs unpacked-API spec design first).

---

## What's OUT OF SCOPE for this 1 week

| Item | Where it goes |
|---|---|
| **Neon to panic_free/verified** | Separate 1-week sprint (per Q1 audit, ~4-8 sessions including SIMD intrinsic models). |
| **Incremental API** | Already in ADMIT_MODULES; not on Milestone A or B critical path. |
| **Full Milestone B for `ind_cca`** | Needs unpacked-API spec design (Hacspec `ind_cca_unpack_*` shapes). 4-8 week plan. |
| **AVX2 ntt_multiply / inv_ntt_layer_1 admits** | Documented C4e Layer-0.5 admits. Below-trait blockers; tackle in below-trait sweep if cheap, else defer. |

---

## 4-8 week plan — Full Milestone B + Neon

After Milestone A closes, the longer effort:

### Weeks 2-3 — Milestone B for `ind_cca`

Owner: U for spec design, A for proof replication.

- **Week 2 (U-heavy):** Design Hacspec `ind_cca_unpack_encapsulate` and `ind_cca_unpack_decapsulate` spec functions in `specs/ml-kem/`. Write commute lemmas connecting the unpacked-API impl to the spec.
- **Week 3 (A-surge):** Replicate full ensures across `unpacked::encapsulate` and `unpacked::decapsulate`, plus `ind_cca::encapsulate` and `ind_cca::decapsulate` (packed). Wire through callers.

### Weeks 4-5 — Neon panic_free

Owner: U for SIMD intrinsic model design, A for body proofs.

- **Week 4 (U-heavy):** Build SIMD intrinsic models in `crates/utils/intrinsics` for the Neon ops we use. Trust-base sprint pattern from `INTRINSICS-TRUST-PLAN.md`.
- **Week 5 (A-surge):** Verify each Neon op against the intrinsic model. Drop Neon entries from `ADMIT_MODULES` one by one.

### Weeks 6-7 — Neon full Hacspec ensures (stretch)

Same as Portable's verification trajectory but for Neon. Mostly mechanical replication once panic_free done.

### Week 8 — Hardening + audit

- Tighten the user-axiom set (per Q2 audit, ~1280 words). Document each axiom precisely.
- Run a final security audit for soundness of trait carve-outs.
- Publish proof status milestone in repo.

---

## Risks

1. **Phase C bridge lemma** (Day 1 U-task) — fresh design. If the bridge requires more than one lemma, slip ½ day into Day 2. Mitigation: if the bridge is more complex than expected, defer the cascade replay and have agents work on Pattern-2 + below-trait residuals in parallel; the bridge can land Day 3.
2. **USER-12 NTT layer 1** (Day 1 U-task) — outstanding for many sessions. The branch-helper refactor pattern is documented but unverified. Mitigation: if not landed by end of Day 1, defer to Day 3 along with USER-13 cluster.
3. **`compress_then_serialize_message` Q2 cancel** — single Z3 query that doesn't terminate. May surface during Day 3 below-trait sweep. Mitigation: re-add to `ADMIT_MODULES` if necessary, document precise blocker.
4. **noeq-style extractor regressions** — re-extraction may surface other latent issues not caught at HEAD. Mitigation: each agent's first action should be a clean `make check` on the merged base before editing.

---

## Status pings

- Per-stream status: `proofs/agent-status/stream<N>-status.md` (15-min cadence per `feedback_agent_status_reports`).
- Daily roll-up: append to `proofs/agent-status/sprint-2026-05-03-rollup.md` at end-of-day.

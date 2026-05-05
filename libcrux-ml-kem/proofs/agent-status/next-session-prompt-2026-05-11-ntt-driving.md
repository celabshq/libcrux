# Next-session prompt — drive forward+inverse NTT to full hacspec

**Branch:** `libcrux-ml-kem-proofs` (or fresh worktree, see Pre-session step 1)
**Tip on entry:** `f2bb7c7ca` (or later — sprint 2026-05-09 rollup + 2026-05-10 prompt)

**Mostly autonomous, with user reserved for two specific calls.**  Goal
is to close the highest-value *full hacspec* milestones in
`proofs/proof_milestones.md` rows 1, 2, 6, 7, 8, 9 (forward NTT layers,
top-level NTT, inverse NTT layer 4+, Montgomery driver, ntt_multiply).

## Autonomy split + interleave plan

The bridge infrastructure in
`Hacspec_ml_kem.Commute.Bridges.fst` is far more complete than the
2026-05-09 rollup suggested (see "What's already there" below).  Most
of this work is mechanical integration; only a few pieces genuinely
need the user.  Plan: **the agent runs the autonomous slice while
parked on a user-required item, so the user's reply unblocks
maximum-already-built work, not a fresh start.**

### Agent-autonomous lanes — start these regardless of user availability

**A. USER-15 unfold lemma in `Bridges.fst`** (~30-60 min, no
dependencies).  Authoring `lemma_ntt_inverse_butterflies_unfold` (or
named to fit the file's convention): a definitional Lemma stating that
`IN.ntt_inverse_butterflies p == IN.ntt_inverse_layer (… (1)) (2)` for
the 7-call chain.  Provable from the hacspec definition without any
impl-side dependency.  Land independently of USER-14.

**B. USER-15 close in `invert_ntt_montgomery`** (~30 min after lane A).
With the unfold lemma proven, drop `verification_status(panic_free)`
on `invert_ntt_montgomery`, add a `hax_lib::fstar!` block invoking the
unfold lemma, and verify the body composes against the inherited
admitted-ensures of `invert_ntt_at_layer_4_plus` (ensures already cite
`IN.ntt_inverse_layer` even though the body is admitted — consumers
treat this as a known fact).  This closes USER-15 *without waiting on
USER-14 body discharge.*  Milestone row 7 → ✅.

**C. `ntt_vector_u` panic-free + bound-only ensures** (~30-45 min).
The 2026-05-09 attempt failed because explicit `panic_free` interfered
with bound propagation through subsequent calls.  The fix: remove
`verification_status(panic_free)`, drop the functional Hacspec ensures
conjunct (keep only `is_bounded_poly(3328, future(re))`), use options
`--z3rlimit 200` (no `context_pruning`, no `split_queries`).  This is
exactly the proven sibling `ntt_binomially_sampled_ring_element`
pattern.  Milestone row 1 partial improvement.

**D. Wiring scaffold for forward `ntt_at_layer_4_plus` ensures** —
*conditional on user input on `Hacspec_ml_kem.Ntt.ntt_layer_n` shape*
(see lane E).  If the spec is already finalized, this becomes a
pattern-port from inverse layer_4_plus and is autonomous.  Otherwise
parked.

### User-required lanes — ping and continue elsewhere

**E. Forward NTT layer_4_plus spec citation** (per milestone row 1:
"spec design needed").  Verify whether
`Hacspec_ml_kem.Ntt.ntt_layer_n` (or equivalent multi-step forward
layer spec) exists in `specs/ml-kem/src/ntt.rs` /
`Hacspec_ml_kem.Ntt.fst`.  If yes, autonomous (lane D).  If no, ping
user with the specific question:
> "Forward layer_4_plus needs `Hacspec_ml_kem.Ntt.ntt_layer_n` (or
> equivalent).  I see `IN.ntt_inverse_layer` for inverse — should I
> mirror this name/shape for forward, or do you have a different spec
> design in mind?"

**F. USER-14 body discharge — when `panic_free` flip stalls.**  First
attempt: drop `--admit_smt_queries true`, add
`verification_status(panic_free)` + `--z3rlimit 400 --ext context_pruning
--split_queries always`, integrate `lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec`
into the inner-loop body.  This sprint already hit rlimit-400 cap at
queries 188 and 204 (canceled at the cap).  When it stalls again, ping
user with the specific question:
> "USER-14 layer_4_plus inner-loop maintenance hit rlimit-400 cap
> [again].  My default is to restructure into 4 concrete-`layer` fns
> (`invert_ntt_at_layer_{4,5,6,7}` instead of one parameterized).
> Approve, or want me to try a helper-with-`layer`-param first?"

**G. `ntt_at_layer_7` novel design.**  Structurally distinct (single-
zeta, between-chunk butterfly).  No `_lane_bridge` analog.  If reached
this session, ping user with:
> "ntt_at_layer_7 has no template — needs new bridge design.  Punt to
> a separate session, or want me to draft a strawman design for
> review?"

**H. `ntt_multiply` spec citation.**  Verify
`Hacspec_ml_kem.Ntt.multiply_ntts` exists.  If yes, lane is autonomous
(define trait post + per-fn proof).  If no, ping user.

### Recommended execution order (interleave-aware)

1. **First 60 min:** lanes A + C in parallel (different files, no
   conflict).  Both land cleanly.
2. **Next 30 min:** lane B (uses lane A).  USER-15 → ✅.
3. **Probe lane E** (does the forward spec exist?).  If yes → start
   lane D.  If no → ping user, continue to step 4.
4. **Attempt lane F** (USER-14 first attempt).  Expected to stall at
   rlimit cap.  When it stalls, ping user, continue to step 5.
5. **While parked on E and F:** check lane H spec citation, write up
   the milestone-doc updates for the closed lanes (A, B, C), refresh
   `proofs/agent-status/fstar-perf-top20.md`.
6. **On user reply:** execute the chosen path immediately; the
   prerequisite work is already done.

### Defaults that DO NOT need user input

- `_to_hacspec` lemma placement → `Bridges.fst`, mirror existing family.
- Forward+inverse layer_4_plus pairing → close inverse first, then
  pattern-port forward.  Don't ping for sequencing.
- USER-15 unfold lemma placement → `Bridges.fst`.
- Whether to land lanes A/B/C before USER-14 closes → yes, always;
  they don't depend on USER-14 body.

## Branch hygiene — mandatory

The user's parallel work runs in `/Users/karthik/libcrux-trait-opacify`.
Per `feedback_branch_means_worktree`:

```bash
git -C /Users/karthik/libcrux-trait-opacify worktree add \
    /Users/karthik/libcrux-ntt-full-hacspec \
    -b agent-mlkem-ntt-full-hacspec-2026-05-11
cd /Users/karthik/libcrux-ntt-full-hacspec/libcrux-ml-kem
```

All work here.  **Do NOT touch the shared worktree.**  When the session
closes, user merges/cherry-picks back.  If the worktree directory
already exists, prompt user before reusing.

## What's already there — surprisingly complete

Read `proofs/proof_milestones.md` Layer 1 table first.  Summary of the
already-shipped bridge infrastructure (in
`specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Bridges.fst`):

**Forward NTT polynomial-level bridges (PROVEN):**
- `lemma_ntt_layer_1_step_to_hacspec` (line 150)
- `lemma_ntt_layer_2_step_to_hacspec` (line 984) — uses 4 per-branch
  helpers at lines 800/836/872/908 (the "branch_post 4-way refactor"
  pattern).
- `lemma_ntt_layer_3_step_to_hacspec` (line 1129)

**Inverse NTT polynomial-level bridges (PROVEN):**
- `lemma_inv_ntt_layer_1_step_to_hacspec` (line 252)
- `lemma_inv_ntt_layer_2_step_to_hacspec` (line 702) — uses the same
  4 per-branch helper structure.
- `lemma_inv_ntt_layer_3_step_to_hacspec` (line 411)

**Per-int-vec-step bridge (PROVEN, key for layer_4_plus):**
- `lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec` (line 1173) —
  this is the building block that lifts a single
  `inv_ntt_layer_int_vec_step_reduce` call's per-pair commute to the
  polynomial-level claim used by layer_4_plus.

**Already-wired sites (impl-side `lemma_*_to_hacspec` calls):**
- `src/ntt.rs:233, 253, 340, 359` — forward layer_2, layer_3 wired in
  the impl (`ntt_at_layer_2`, `ntt_at_layer_3`).
- `src/invert_ntt.rs:253, 274, 388` — inverse layer_2, layer_3 wired
  in the impl.

**Currently still admitted:**
- `invert_ntt_at_layer_4_plus` (USER-14, row 6).
- `invert_ntt_montgomery` (USER-15, row 7).
- `ntt_at_layer_4_plus` (forward, no claim yet — row 1 partial).
- `ntt_at_layer_7` (forward, single-zeta between-chunk, novel design).
- `ntt_vector_u` (above-traits driver — flips trivially once layer_4+
  closes).

## Recommended ordering — driven by milestone doc

The milestone doc's "Next-priority order" (lines 99-113) is the
authoritative ordering.  Adapted with this sprint's context:

### Order 1: USER-15 — `invert_ntt_montgomery` body discharge (~1 session)

**Currently:** `panic_free` (this sprint, commit `2b0f159d1`) — body
verifies, ensures admitted via panic_free.

**To close:** drop `verification_status(panic_free)`; the body is just
`invert_ntt_at_layer_{1,2,3}` + 4× `invert_ntt_at_layer_4_plus` +
`is_bounded_poly_higher` widenings.  Once layers 1, 2, 3 already-have
hacspec ensures (they do, see milestone rows 3, 4, 5), and layer_4_plus
closes (Order 2 below), the chain composes:

```
ntt_inverse_butterflies = layer_1 ∘ layer_2 ∘ layer_3 ∘
                          layer_4_plus(4) ∘ layer_4_plus(5) ∘
                          layer_4_plus(6) ∘ layer_4_plus(7)
```

**What's needed:** a `Hacspec_ml_kem.Invert_ntt.ntt_inverse_butterflies`
unfolding lemma that exposes this 7-call chain definitionally.  The spec
exists; the lemma is mechanical.  ~30 min to write + 30 min to verify.

**Blocker:** USER-14 must land first (else layer_4_plus still has body
admit and the chain doesn't compose).

**Decision point for user:** wire the unfold lemma directly in
`src/invert_ntt.rs` body (via `hax_lib::fstar!` block) vs. add to
`Hacspec_ml_kem.Commute.Bridges.fst` as a reusable lemma.  Recommend
the latter for symmetry with the existing `_to_hacspec` family.

### Order 2: USER-14 — `invert_ntt_at_layer_4_plus` body discharge (~1-2 sessions)

**Currently:** `--admit_smt_queries true` (`src/invert_ntt.rs:528`).
This sprint's attempt to flip directly to `panic_free` failed — inner-loop
maintenance VC saturated rlimit 400 cap (queries 188 / 204 canceled at
the cap with 157 s and 200 s wall).

**Path to close:**

The body has nested `for round` × `for j` loops with a rich `is_bounded_vector`
loop invariant.  Each inner-loop body calls
`inv_ntt_layer_int_vec_step_reduce` whose per-pair commute is captured
by `lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec`
(`Bridges.fst:1173`, **already proven**).

**The bridge to write:** a polynomial-level lemma that lifts the
per-iteration claim to the loop invariant's hacspec form.  Pattern:

```fstar
(* Polynomial-level form: layer_4_plus 16 iterations of layer step
   over chunks j ∈ [0,8), j+8 = the matched pair *)
let lemma_inv_ntt_at_layer_4_plus_iter_to_hacspec
    (#vV: Type0) {| Operations vV |}
    (re_in re_out: t_PolynomialRingElement vV)
    (zeta_i: usize) (layer round: nat)
    (* ...preconditions matching layer_4_plus's loop invariant... *)
  : Lemma (ensures
      (* post-iteration: to_spec_poly_mont re_out matches
         IN.ntt_inverse_layer applied to to_spec_poly_mont re_in *))
  = (* unfolds via 16 lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec
       calls + Seq.lemma_eq_intro *)
```

**Open user-decision:** layer_4_plus has 4 layers (4, 5, 6, 7) that
share a single function body via the `match layer { 4 => ..., ... }`
pre/post.  Z3 case-splits on `layer` and the inner-loop maintenance
explodes.  Two options:

**(a)** Restructure as 4 separate functions
`invert_ntt_at_layer_{4,5,6,7}` each with concrete `layer` (drops the
case-split).  Cleaner but ~4× boilerplate.

**(b)** Keep one function but wrap the loop body in a helper that takes
`layer` as a concrete parameter, deferring the case-split out of the
inner-loop VC.  Cleaner code, but Z3 may still struggle.

**Recommend:** start with (a) for `layer == 4` only (the first invocation
in `invert_ntt_montgomery`'s body), prove it cleanly, then try (b) for
the remaining 5/6/7 invocations.  This stages the risk.

### Order 3: Forward NTT row 1 — `ntt_at_layer_4_plus` + `ntt_at_layer_7`

**Currently:** bounds-only ensures, no hacspec claim.

**Distance:** the bridges
`lemma_ntt_layer_n_16_<2*len>_lane` for `len > 8` don't exist for forward
yet (the comment in `Chunk.fst:2356-2390` describes the pattern: each
layer needs a `lemma_ntt_layer_n_16_<2*len>_lane` createi unfold + a
`zetas_<groups>_lane` zetas unfold + a per-lane bridge).

**`ntt_at_layer_7`** is structurally novel (single-zeta between-chunk
butterfly) — needs its own bridge design.  Defer.

**Recommended scope this session:** just `ntt_at_layer_4_plus` body
discharge + ensures upgrade, mirroring the inverse layer_4_plus pattern
(Order 2).  USER-14 (inverse) and forward layer_4_plus share most of
the lifting infrastructure — close them as a pair.

### Order 4: Above-traits drivers — `ntt_vector_u`, top-level `ntt`

Once forward layer_4_plus closes:
- `ntt_vector_u` (`src/ntt.rs:564`) drops `--admit_smt_queries true`
  and gets full hacspec functional ensures (the chain
  `ntt_at_layer_4+×4 → layer_3 → layer_2 → layer_1` already exists in
  the body).
- Top-level `ntt` in `polynomial.rs` (milestone row 2) gets its hacspec
  ensures and verifies via composition.

### Order 5: `ntt_multiply` (milestone row 8)

Separate workstream — needs `Hacspec_ml_kem.Ntt.multiply_ntts` spec
citation in the trait + per-fn proof.  ~1 sprint.

## Z3 walls already encountered + diagnostics

From the 2026-05-09 sprint log
(`proofs/agent-status/sprint-2026-05-09-rollup.md` → "What deferred"):

- `invert_ntt_at_layer_4_plus` flipped directly to `panic_free` with
  `--z3rlimit 400 --ext context_pruning --split_queries always`:
  query 188 timed out at 157 s using 400/400 rlimit (canceled), query
  204 same at 199 s.  Conclusion: inner-loop invariant maintenance is
  the bottleneck; the per-pair facts can't be aggregated in one VC.

- `ntt_vector_u` direct `panic_free` flip (no functional ensures
  retained): subtyping fails on layer_2/layer_1 calls' bound args
  `6*3328`/`7*3328`.  Sibling
  `ntt_binomially_sampled_ring_element` verifies the same chain
  *without* `panic_free` (uses default verify-ensures with bound-only
  ensures).  Hypothesis: `panic_free` interferes with bound
  propagation through call site preconditions.  Worth diagnosing —
  may apply to the layer_4+ wrapper too.

**Diagnostic recipe for new walls** (per `feedback` skill `smtprofiling`):

```bash
# In src/<file>.rs, on the failing fn, add:
#   #[hax_lib::fstar::options("--z3rlimit 400 --query_stats --log_queries --z3refresh --split_queries always")]
# Then:
python3 hax.py extract && cd proofs/fstar/extraction && rm -f .depend
make check/Libcrux_ml_kem.<Module>.fst > /tmp/qs.log 2>&1
# After fail:
ls queries-Libcrux_ml_kem.<Module>-*.smt2 | tail
z3 smt.qi.profile=true -smt2 queries-...-<failing-N>.smt2 \
    | grep -A 1 "qi.profile" | head -30
# Top quantifier triggers identify the saturating axiom.
```

Find proven-pattern templates in
`Bridges.fst`:152-200 (`lemma_ntt_layer_1_step_to_hacspec`),
`Bridges.fst`:984-1022 (`lemma_ntt_layer_2_step_to_hacspec` — the 4-way
refactor pattern in action).

## Read first (non-negotiable)

1. **`proofs/proof_milestones.md`** — Layer 1 table, "Next-priority
   order" section (lines 99-113), and "What the count does NOT reflect"
   (lines 91-97).
2. **`proofs/agent-status/sprint-2026-05-09-rollup.md`** — context for
   why USER-14 / USER-15 are still open and what was tried.
3. **`MEMORY.md`** entries:
   - `feedback_layer2_branch_post_z3_unlock` — the 4-way per-branch
     helper recipe for branch_post wrappers (already used by layer_2).
   - `feedback_drive_to_top_spike` — strengthen post chain top-down with
     `--admit_smt_queries true` on bodies first; validate spec via
     consumer propagation BEFORE discharging bodies.
   - `feedback_proof_debug_budget` — 30-60 min hard cap per fn.
   - `feedback_rlimit_cap_800` — never bump past 800 / 400-with-split;
     restructure instead.
   - `feedback_branch_means_worktree`.
   - `feedback_use_fstar_mcp` — sub-second iteration vs. 50-100s `make`.
   - `feedback_track_fstar_perf` — refresh `fstar-perf-top20.md` after
     full builds.
4. **`Hacspec_ml_kem.Commute.Bridges.fst`** — skim the existing proven
   `_to_hacspec` lemmas as templates.  In particular,
   `lemma_inv_ntt_layer_2_step_to_hacspec` (line 702) demonstrates the
   4-way per-branch composition pattern.

## When to ping the user (consolidated from "Autonomy split")

The agent-autonomous lanes (A, B, C) require no user contact.  Ping
only when blocked on user-required lanes (E, F, G, H above), and
*always* keep an autonomous lane running while parked.

If you find yourself needing to write a new `lemma_ntt_layer_n_16_<N>_lane`
for a layer not already covered, that's a sign you've drifted off the
recommended order — ping the user before adding new infrastructure.

## Stage acceptance + commit hygiene

Per closed user-task:
- `make check/Libcrux_ml_kem.<Module>.fst rc=0`.
- `bash proofs/generate_verification_status.sh` shows the affected row
  no longer has lax / has higher tier (Hacspec column).
- Update `proofs/proof_milestones.md` row status from 🔶 → ✅ with the
  closing commit hash.

Suggested commits:
- `agent-mlkem: USER-14 close — invert_ntt_at_layer_4_plus body discharges via lemma_inv_ntt_layer_int_vec_step_reduce_to_hacspec lift`
- `agent-mlkem: USER-15 close — invert_ntt_montgomery functional ensures`
- `agent-mlkem: forward NTT layer_4_plus + ntt_at_layer_7 hacspec ensures`
  (if tackled this session)
- `agent-mlkem: ntt_vector_u functional ensures (bounded by row 8 forward chain)`

Final session rollup:
- `agent-mlkem: sprint 2026-05-11 rollup — NTT full-hacspec drive`

## Pre-session checklist

- [ ] Worktree created at `/Users/karthik/libcrux-ntt-full-hacspec`,
      branch `agent-mlkem-ntt-full-hacspec-2026-05-11`, tip
      `f2bb7c7ca` or later.  Confirm `pwd` and `git rev-parse HEAD`
      before any edits.
- [ ] Read `proofs/proof_milestones.md` Layer 1 table + Next-priority
      order.
- [ ] Read `proofs/agent-status/sprint-2026-05-09-rollup.md` "What
      deferred" section.
- [ ] Skim `Hacspec_ml_kem.Commute.Bridges.fst` — the existing 12+
      proven `_to_hacspec` lemmas + `_lane_bridge` helpers.
- [ ] Confirm full-tree build is at known baseline (1 pre-existing
      `Hash_functions.fst` Error 47 only) before starting.

## Status reports (per `feedback_agent_status_reports`)

Every ~15 min, append a 4-line update to
`proofs/agent-status/sprint-2026-05-11-status.md`:
- Active lane (A/B/C/D/E/F/G/H from "Autonomy split").
- F* state (verifying / failing / new wall / parked).
- If parked: which user-required lane, what question is queued.
- Next autonomous lane to pick up.

When pinging the user: write the question to the status doc *and*
output it to chat, then immediately switch to the next autonomous
lane — do not idle waiting.

End-of-session: write
`proofs/agent-status/sprint-2026-05-11-rollup.md` summarizing each
landed lane + closing commits + open user pings.  Update
`proofs/proof_milestones.md` row statuses.

## Out-of-scope / explicit non-goals

- AVX2/serialize lax sites — separate sprint, see
  `next-session-prompt-2026-05-10-avx2-serialize-closure.md`.
- `to_bytes`/`from_bytes` hax-lib slice modeling.
- Portable `op_(inv_)ntt_layer_1_step` 4-way branch refactor — separate
  user-task; not on the critical path for full NTT hacspec.
- `mlkem*.rs` extraction (milestone row 19) — separate sprint.
- Top-level KEM API hacspec citations (rows 20-28) — gated on row 19.

## Key file paths quick reference

- `proofs/proof_milestones.md` — authoritative milestone tracker.
- `specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Bridges.fst` —
  polynomial-level NTT bridges (already proven for layers 1, 2, 3
  forward + inverse + per-int-vec-step).
- `specs/ml-kem/proofs/fstar/commute/Hacspec_ml_kem.Commute.Chunk.fst` —
  per-chunk butterfly commute lemmas + the
  `lemma_ntt_layer_n_16_<N>_lane` createi unfolds.
- `src/ntt.rs` — forward NTT impl; layers 2, 3 already wired
  (lines 233, 253, 340, 359).  `ntt_at_layer_4_plus` (line 406) +
  `ntt_at_layer_7` (line 475) need the spec hookup + body discharge.
- `src/invert_ntt.rs` — inverse NTT impl; layers 1, 2, 3 already wired
  (lines 253, 274, 388).  `invert_ntt_at_layer_4_plus` (line 552) +
  `invert_ntt_montgomery` (line 666) are USER-14 / USER-15.
- `proofs/fstar/extraction/Makefile` — full-make gate.

## Session vibe

Mostly autonomous.  Run lanes A/B/C/D in parallel where independent;
ping user only on lanes E/F/G/H per "When to ping the user".
**Never idle while parked** — there is always another autonomous lane
ready.  Use `fstar-mcp` typecheck for sub-second iteration during proof
drafting; switch to full `make check/<Module>.fst` only at stage
acceptance.  Pipe make output to `/tmp/*.log` and grep — never `Read`
the full log.

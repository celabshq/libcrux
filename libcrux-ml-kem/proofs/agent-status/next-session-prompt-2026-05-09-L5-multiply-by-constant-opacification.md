# Next-session prompt — L5 `multiply_by_constant_post` opacification

**Goal:** Hide the bare non-linear arithmetic `v r[i] == v vec[i] * v c` from
above-trait consumers by wrapping it in a lane-form opaque atom.

**Branch tip on entry:** `05b772da6` ("cold-baseline perf snapshot 2026-05-08")
or later. Verify via `git log -1 --oneline`.

**Worktree:** create a fresh one — per `feedback_branch_means_worktree`:

```bash
git -C /Users/karthik/libcrux-trait-opacify worktree add \
    /Users/karthik/libcrux-multiply-by-constant-opaque \
    -b agent-mlkem-L5-multiply-by-constant-opaque-2026-05-09
cd /Users/karthik/libcrux-multiply-by-constant-opaque
```

**Session constraint:** `make -k -j2` (NOT `-j4`) — `python3 hax.py prove`
hardcodes `-j4`, so for full proves bypass it via direct
`make -k -j2 -C libcrux-ml-kem/proofs/fstar/extraction/`.

## Read first (non-negotiable)

1. **`~/.claude/skills/fstar-for-libcrux/SKILL.md`** — Rules 1, 5, 7
   especially.
2. `MEMORY.md` — `feedback_no_cache_nuke`, `feedback_panic_free_vs_lax`,
   `feedback_smtpat_percent_above_trait`, `feedback_proof_debug_budget`.
3. **`libcrux-ml-kem/proofs/agent-status/audit-2026-05-08-trait-boundary.md`**
   — the audit defining L5 (sections §1.1, §4). Read the priority
   matrix in §"Recommended cleanup order".
4. **`libcrux-ml-kem/proofs/agent-status/sprint-2026-05-13-rollup.md`**
   — worked example for the same recipe applied to loadu/storeu Sites 1–4.

## Why this sprint (sharp framing)

L5 is the **only** entry in the audit where the trait post exposes
**bare non-linear arithmetic** (`v vec[i] * v c`) without an opaque
wrapper. Every other "leaky" post either is linear (L1–L4, L6, L9, L10)
or wraps non-linear content under `mod_q_eq` (L7, L8). Z3's NRA
decision procedure WILL run on L5's bare form at any consumer that
mentions `multiply_by_constant_post`; it does NOT run on the others
unless the wrapping opaque is revealed.

L5 is therefore **the highest-perf-leverage single boundary fix**.
L7/L8 cleanup is abstraction-quality only; L5 is abstraction + perf.

## Scope (in / out)

**IN scope:**
- Define `multiply_by_constant_lane_post` opaque in `Vector.Traits.Spec`.
- Rewrite `multiply_by_constant_pre` and `multiply_by_constant_post` in
  `vector/traits.rs::spec` to use the lane atom.
- Update each backend's `impl Operations::multiply_by_constant` proof
  in `vector/{avx2,portable}.rs` (and Neon if not in `ADMIT_MODULES` —
  it likely is).
- Update consumers across `polynomial.rs`, `serialize.rs`, `ind_cpa.rs`,
  `Hacspec_ml_kem.Commute.{Chunk,Bridges}.fst`. Expect ~5–10 reveal
  sites total based on call-frequency in the codebase.

**OUT of scope (separate sprints):**
- L7 `barrett_reduce_post`, L8 `montgomery_multiply_by_constant_post`
  — abstraction-only fixes, queued as separate sessions.
- L1–L4 (add/sub/negate add/sub) — linear, abstraction-only, follow-on.
- L6, L9, L10 — small abstraction follow-ons.
- `is_bounded_poly` opacity — deferred, touches more callers.
- The 2 cold-baseline failures (`Types.Index_impls.fst:18`,
  `Vector.Portable.fst:1008` post-Sprint-B) — separate quick-fix
  sprint.

## Current shape (what L5 looks like today)

`libcrux-ml-kem/src/vector/traits.rs` lines 677–695:

```rust
pub(crate) fn multiply_by_constant_pre(vec: &[i16; 16], c: i16) -> hax_lib::Prop {
    hax_lib::fstar_prop_expr!(
        r#"forall i.
            is_intb (pow2 15 - 1) (v (Seq.index ${vec} i) * v $c)"#
    )
}

pub(crate) fn multiply_by_constant_post(
    vec: &[i16; 16],
    c: i16,
    result: &[i16; 16],
) -> hax_lib::Prop {
    hax_lib::fstar_prop_expr!(
        r#"forall i.
            v (Seq.index ${result} i) ==
            v (Seq.index ${vec} i) * v $c /\
            is_intb (pow2 15 - 1) (v (Seq.index ${result} i))"#
    )
}
```

Both contain bare `v vec[i] * v c`. The post even surfaces an
equation `v r[i] == v vec[i] * v c` — directly invitable into Z3's NRA
e-graph at any consumer site.

## Target shape (recipe)

### Step 1: Define `multiply_by_constant_lane_post` opaque

In `vector/traits.rs::spec`'s `hax_lib::fstar::before` block (around
line 169 where `bounded_i16_array` is defined), add:

```fstar
[@@ "opaque_to_smt"]
let multiply_by_constant_lane_post (vec_i c result_i: i16) : prop =
  v result_i == v vec_i * v c /\
  is_intb (pow2 15 - 1) (v result_i)

(* SMTPat-fired per-index unfolding (consume direction).  Dual-trigger
   pattern: fires only when Z3 has BOTH the indexed access on `result`
   AND the opaque atom on the same lane in its e-graph. *)
let lemma_multiply_by_constant_lane_post_lookup
    (vec_i c result_i: i16)
    : Lemma (requires multiply_by_constant_lane_post vec_i c result_i)
            (ensures  v result_i == v vec_i * v c /\
                      is_intb (pow2 15 - 1) (v result_i))
            [SMTPat (multiply_by_constant_lane_post vec_i c result_i)] =
  reveal_opaque (`%multiply_by_constant_lane_post)
                (multiply_by_constant_lane_post vec_i c result_i)
```

(SMTPat single-pattern is acceptable here because `multiply_by_constant_lane_post`
is a fresh predicate with no other call sites — only consumers that explicitly
have it in scope will trigger.)

### Step 2: Rewrite trait pre/post

Same file, replace the `multiply_by_constant_pre` / `_post` bodies
with:

```rust
pub(crate) fn multiply_by_constant_pre(vec: &[i16; 16], c: i16) -> hax_lib::Prop {
    hax_lib::fstar_prop_expr!(
        r#"forall i.
            is_intb (pow2 15 - 1) (v (Seq.index ${vec} i) * v $c)"#
    )
}
```

(Pre stays — the pre is a sufficiency condition; it's not the consumer-leak
problem. The post is what hurts.)

```rust
pub(crate) fn multiply_by_constant_post(
    vec: &[i16; 16],
    c: i16,
    result: &[i16; 16],
) -> hax_lib::Prop {
    hax_lib::fstar_prop_expr!(
        r#"Spec.Utils.forall16 (fun (i: nat{i < 16}) ->
             multiply_by_constant_lane_post
               (Seq.index ${vec} i) ${c} (Seq.index ${result} i))"#
    )
}
```

(Optional refinement: also fix L5's `pre` similarly with a
`multiply_by_constant_lane_pre` opaque if you want to fully seal both
directions. Lower priority — the pre's `forall i. is_intb (...) (v[i] * v c)`
is something callers ESTABLISH, not consume; they typically prove it from
`is_i16b_array_opaque` + a single `c`-bound. Re-shaping it is more
invasive. Defer to a follow-up.)

### Step 3: Update backend impls

Three backends to check (`avx2.rs`, `portable.rs`, `neon` likely admitted):

```bash
grep -nE "fn multiply_by_constant\b|spec::multiply_by_constant_post" \
  libcrux-ml-kem/src/vector/{avx2,portable,neon}.rs \
  libcrux-ml-kem/src/vector/avx2/arithmetic.rs \
  libcrux-ml-kem/src/vector/portable/arithmetic.rs 2>/dev/null
```

For each backend's `impl Operations::multiply_by_constant`, the body
already discharges the per-lane equation. Add a `hax_lib::fstar!()` block
that reveals the new opaque atom:

```rust
fn multiply_by_constant(vec: Self, c: i16) -> Self {
    let result = arithmetic::multiply_by_constant(vec, c);
    hax_lib::fstar!(
        r#"reveal_opaque (`%Libcrux_ml_kem.Vector.Traits.Spec.multiply_by_constant_lane_post)
                          (Libcrux_ml_kem.Vector.Traits.Spec.multiply_by_constant_lane_post)"#
    );
    result
}
```

Validate per-backend:

```bash
make -k -j2 -C libcrux-ml-kem/proofs/fstar/extraction/ \
  check/Libcrux_ml_kem.Vector.Avx2.fst > /tmp/v_avx2.log 2>&1
echo rc=$?
grep -nE 'Error 19|^\* Error' /tmp/v_avx2.log | head
```

### Step 4: Update consumers

Find direct consumers of `multiply_by_constant_post`:

```bash
grep -rnE "multiply_by_constant_post" \
  libcrux-ml-kem/src libcrux-ml-kem/proofs/fstar specs/ml-kem/proofs/fstar 2>/dev/null \
  | grep -v "vector/traits.rs\|\.checked\|queries-"
```

Expect:
- Polynomial-level functions (`polynomial.rs`) that compose multiply with
  add/sub. They likely consume the per-i forall directly. After the
  rewrite, they need a `reveal_opaque (`%multiply_by_constant_lane_post)`
  if they need the per-i equation, OR can leave it sealed and rely on
  bound-only reasoning.
- Commute lemmas (`Hacspec_ml_kem.Commute.{Chunk,Bridges}.fst`) that
  bridge the equation to the FE-algebra level. These DO need the
  unfolded equation — add `reveal_opaque` per consumer site.

For each consumer that previously took `forall i. v r[i] == v vec[i] * v c`
as a usable hypothesis, the rewrite will surface a
`Spec.Utils.forall16 (fun i -> multiply_by_constant_lane_post …)`. Consumers
need to either:
1. `reveal_opaque (`%multiply_by_constant_lane_post)` to recover the
   raw equation (lazy compatibility — works but doesn't reduce the
   leak in that consumer), OR
2. Refactor to consume the lane post directly via the SMTPat'd lookup
   lemma (reduces NRA work in that consumer).

Start with (1) for compatibility; defer (2) to per-consumer sprints.

### Step 5: Per-stage validation

After each step, validate via the tightest scope possible:

```bash
# After Step 1: spec module compiles
make -k -j2 -C libcrux-ml-kem/proofs/fstar/extraction/ \
  check/Libcrux_ml_kem.Vector.Traits.Spec.fst > /tmp/v_spec.log 2>&1

# After Step 2: traits module compiles
make -k -j2 -C libcrux-ml-kem/proofs/fstar/extraction/ \
  check/Libcrux_ml_kem.Vector.Traits.fst > /tmp/v_traits.log 2>&1

# After Step 3 (per backend):
make -k -j2 -C libcrux-ml-kem/proofs/fstar/extraction/ \
  check/Libcrux_ml_kem.Vector.{Avx2,Portable}.fst > /tmp/v_backend.log 2>&1

# After Step 4: full prove
make -k -j2 -C libcrux-ml-kem/proofs/fstar/extraction/ > /tmp/v_full.log 2>&1
echo rc=$?
grep -cE '^\* Error 19' /tmp/v_full.log
```

Expected final state: `rc=0`, `0` Error 19s.

## Worked example reference

The loadu/storeu sprint (commits `dd1bbbf4b`, `8fd907a53`, `49e70d5d4`)
applied a structurally identical recipe to a different boundary. Key
takeaways from those commits:

- Spec changes in `crates/utils/intrinsics/src/avx2_extract.rs` and
  `vector/traits.rs::spec` are mechanical.
- Backend reveals per impl method are one-line additions.
- Consumer reveals in Commute.* are one-line additions per lemma.
- Test gate is `make -k -j2 -C proofs/fstar/extraction/ rc=0`.
- Iteration cycle: ~5–15 min per make round (cold) or sub-second (cached).

## Pre-existing context worth knowing

1. **`mm256_mullo_epi16_specialized4` recently corrected** (commit
   `1c9638f34`). Use of `mm256_mullo_epi16` in `multiply_by_constant`
   (avx2/arithmetic.rs:218) goes through this spec.
2. **`montgomery_multiply_by_constant`** is L8 and a sibling of L5 — it
   has the same shape but with `mod_q_eq` wrapping. Don't conflate
   — they're separate sprints.
3. **2 cold-baseline failures** exist (per
   `fstar-perf-top20.md` snapshot 2): `Types.Index_impls.fst:18` and
   `Vector.Portable.fst:1008`. Out of scope; if either fails after
   your changes for *new* reasons, that's a regression to track.
4. **Top 4 cold-baseline slow proofs** are
   `Vector.Portable.op_(inv_)ntt_layer_{2,3}_step` at rlimit 600/800
   fuel 1. They MAY get faster after L5 if their bodies use
   `multiply_by_constant`, MAY stay the same if they don't.

## Time budget

- Step 1 (spec): 30 min.
- Step 2 (trait rewrite): 15 min.
- Step 3 (3 backends, ~10 min each + validation): 1 hour.
- Step 4 (consumer reveals, expect 5–10 sites): 1 hour.
- Step 5 (full prove + iteration): 30–60 min.

**Total: ~3 hours.** Hard cap per `feedback_proof_debug_budget`: 4
hours. If at the cap, document the working state, mark blockers, and
stop.

## Acceptance criteria

- [ ] `multiply_by_constant_lane_post` defined opaque in
  `Vector.Traits.Spec`.
- [ ] `multiply_by_constant_post` rewritten to use the lane atom.
- [ ] All 3 backends' `impl Operations::multiply_by_constant` updated
  with `reveal_opaque` (Neon may already be in `ADMIT_MODULES` — check
  before editing).
- [ ] All direct consumers of the old shape updated (either reveal-based
  compatibility or refactored to lane-post consumption).
- [ ] `make -k -j2` from `proofs/fstar/extraction/` exits `rc=0` with
  `0` Error 19s.
- [ ] `git grep "v (Seq.index .*) \* v"` in `vector/traits.rs::spec`
  shows the multiply equation gone (other linear leaks may remain — those
  are L1–L4, separate).

## Optional follow-on (within session if time permits)

If acceptance is met within budget, run a fresh perf-top20 snapshot
(`bash proofs/generate_verification_status.sh` plus the awk parse from
`sprint-2026-05-13-rollup.md`-era pipeline) to confirm the
NRA-cost-at-consumers thesis. Look for time drops at:
- `Polynomial.subtract_reduce` (item 21, currently 9.5 s).
- `Polynomial.ntt_multiply` (item 16, currently 4.8 s).
- `Vector.Portable.Ntt.ntt_multiply{,_binomials}` (items 13, 16).

Append the new snapshot to `fstar-perf-top20.md` as Snapshot 3.

## Out-of-band escalation

If during the sprint:
- The lane-form opaque is rejected by a backend's WP composition →
  the recipe in §3 isn't right; surface to user before deeper fixes.
- A consumer needs MORE than `reveal_opaque` to recover (e.g. the lane
  form has different binders) → flag as L5-deferral candidate.
- A cold-baseline failure (Types.Index_impls or Portable.f_from_bytes)
  reproduces unexpectedly mid-sprint → likely environmental contention
  per `feedback_environmental_contention` (this session learned that
  parallel sha3/ml-dsa work degrades Z3 timing); pause and check `ps
  aux | grep fstar.exe`.

## Final commit + merge

Commits expected (one per logical change):

```
agent-mlkem: L5 multiply_by_constant — define lane-post opaque atom
agent-mlkem: L5 multiply_by_constant — rewrite trait post via forall16
agent-mlkem: L5 multiply_by_constant — backend impl reveals
agent-mlkem: L5 multiply_by_constant — consumer reveals (n sites)
[optional] agent-mlkem: L5 perf re-snapshot — Snapshot 3 in fstar-perf-top20
```

Final fast-forward merge into `libcrux-ml-kem-proofs` from the parent
worktree once full prove rc=0:

```bash
cd /Users/karthik/libcrux-trait-opacify
git checkout libcrux-ml-kem/verification_result.txt  # discard transient
git merge --ff-only agent-mlkem-L5-multiply-by-constant-opaque-2026-05-09
git worktree remove --force /Users/karthik/libcrux-multiply-by-constant-opaque
git branch -d agent-mlkem-L5-multiply-by-constant-opaque-2026-05-09
```

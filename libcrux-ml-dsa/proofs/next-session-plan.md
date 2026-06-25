# ML-DSA Sprint — Next-Session Plan

**Branch**: `ml-dsa-proofs`
**Tip after this session**: see `MLDSA_STATUS.md` for the current SHA.
**Sprint plan**: [`sprint-plan.md`](sprint-plan.md) (waves 2A–4D).
**Style guide**: [`proof-style-guide.md`](proof-style-guide.md).
**Outstanding admits**: [`outstanding-admits.md`](outstanding-admits.md).

## Resume entrypoint (2026-04-28 audit + reduce-refactor session)

**Latest tip**: `3faaff641`. Empirical baseline: **97 modules invoked,
[CHECK]=41, [ADMIT]=56, 97 verified, 0 errors**. Was 25 errors / 52
verified / 60 invoked at the previous session's start.

This session's 15 commits, in order:

| SHA | Subject |
|---|---|
| `b8710dfb2` | 25-error → 0-error cleanup (Steps 1-4: INVNTT admits, Portable.Arithmetic u32-mask refactor, Sample cloop refactor, Step 3 AVX2-encoding admit-parity, Step 4 bounded_* bound tightening) |
| `c8b2a7ffd` | Step 5: lift `Simd.Portable.fst` + `Simd.Avx2.fst` from ADMIT to CHECK |
| `33c7b813a` | Step 5b: tighten Portable impl-method posts with `#[hax_lib::attributes]` + per-method `#[requires]/[ensures]` |
| `60f5a9fe9` | Step 5b/5c: AVX2 impl-method tightening + close `bounded_{add,sub}_pre` SMTPats with real proofs |
| `3276f1f44` | Doc refresh after Steps 1-5 |
| `32ba9d9b4` | Step 5c finish: close `bounded_{add,sub}_post` via `Classical.forall_intro` per-lane lemma |
| `e289170ea` | Audit: strengthen Operations trait pre-conditions to match Portable free-fn requires for panic freedom |
| `31975b030` | Verification snapshot confirming the trait pre-tightening propagates cleanly |
| `1943e7f6e` | Add `reduce` pre-condition + relax `shift_left_then_reduce` for SHIFT_BY=0 |
| `2c8e22798` | Docs: refresh status + outstanding-admits for the trait pre-condition audit |
| `3faaff641` | Refactor: dedicated `reduce` primitive (no shift) + drop disjunctive pre |

Major deliverables:

1. **All 25 starting errors closed**.
2. **`Simd.Portable.fst` and `Simd.Avx2.fst` lifted** from ADMIT to CHECK, with all 21 impl methods carrying the strong trait pre/post via `#[hax_lib::attributes]` + per-method `#[requires]/[ensures]`. Body admits remain on the methods whose underlying free fn proves a strictly weaker post than the trait — see `outstanding-admits.md`.
3. **All four `bounded_{add,sub}_{pre,post}` SMTPat-bridge lemmas** in `src/simd/traits/specs.rs` now have real proofs (was 4 admits with an unsound bound at session start).
4. **Operations trait pre-condition audit**: 13 methods strengthened to match the Portable free-fn requires for panic freedom — `use_hint`, all `rejection_sample_*`, all `gamma1_*`, `commitment_serialize`, all `error_*`, all `t0_*`, all `t1_*`, `reduce`. Bounds packaged in opaque-predicate form (reuse of `Spec.Utils.is_i32b_array_opaque`; new `is_binary_array_8_opaque` in specs.rs) following ML-KEM's `bounded_pos_i16_array` style. All Portable + AVX2 impls mirror.
5. **`reduce` / `shift_left_then_reduce` mismatch resolved structurally**: dedicated `arithmetic::reduce` free fn in both Portable and AVX2 (Barrett-reduce only, no shift), `Operations::reduce` calls it directly, `shift_left_then_reduce` keeps its `SHIFT_BY == 13`-only pre. AVX2 dedups via `shift_left_then_reduce` calling `reduce` after `mm256_slli_epi32`.

---

## Hard rule: every function we touch leaves in final form

**Going forward, do not remove a body admit without also bringing the
function's post to its final shape.** A "weak post + admit-removed"
pass is a foot-gun: it leaves the proof obligation indistinguishable
from before, but consumers think the function is done.

Concretely, when picking a function to work on:

1. **Strengthen the post** to its target form (canonical
   `Hacspec_ml_dsa.*` cite, full functional correctness, not just a
   bound). Mark the trait/free-fn posts as final — no admits.
2. **State the necessary intermediate lemmas** in the
   commute / spec module (`Hacspec_ml_dsa.Commute.Chunk.fst`,
   `Spec.Utils.fsti`, etc.) **before** touching the function body.
3. **Add loop invariants** that thread the per-iteration shape of the
   strong post through the function body. Don't accept a loop
   invariant that's just `True`.
4. **Discharge the body** with no `hax_lib::fstar!("admit ()")` and
   no `verification_status(panic_free)` on free functions — those are
   only allowed for pre-budgeted USER-lane items already documented
   in `outstanding-admits.md`.
5. **Run `./hax.sh prove` clean** before moving on. If you hit a
   20-minute wall on a single function, fully revert your edits to
   that function (so the next session starts from a known-good
   state), document the obstacle in `outstanding-admits.md`, and
   pick a different function.

The thin-wrapper rule on impl methods is unchanged: each impl method
mirrors the trait pre/post and forwards to a free function. The
strengthening work happens at the free-function level — once the free
function proves the strong post, the impl method auto-discharges
through the thin-wrapper pattern (no body admit needed).

## Recommended next-session work order

The order optimizes for the rule above: each item is "pick a method
or method group, finish it end-to-end, no admit residue." Cite
`Hacspec_ml_dsa.*` exclusively per Hard Rule #3.

### Step 6 — Create the commute scaffolding (~20 min)

`specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
does not exist yet. Per the hard rules, per-element commute lemmas
land here. Create the file with the right module header + Hacspec
imports; leave it empty otherwise. The next 4-5 functions that need
strengthening will populate it.

### Step 7 — `Operations::reduce` end-to-end (~1 hr)

The dedicated `arithmetic::reduce` free fn (Portable + AVX2) is in
place at `3faaff641` but the impl methods still have a body admit.
Target: prove the impls without admit.

- Portable: `arithmetic::reduce` already has the strong post
  (`is_i32b_array_opaque 8380416` + per-lane `mod_q` congruence).
  The impl method's post is `forall j i. reduce_lane_post …` per
  `Libcrux_ml_dsa.Simd.Traits.Specs`. Bridge: a single per-lane
  lemma in `Commute.Chunk.fst` that says `reduce_lane_post input
  result <==> (centered Barrett bound) ∧ (mod_q congruence)` — both
  shapes are already in spec form.
- AVX2: same target. The free `arithmetic::reduce` post is
  `barrett_red(input)` per i32x8 lane; needs a translation lemma to
  `reduce_lane_post (f_repr v).[i] (f_repr v_future).[i]`.

### Step 8 — `Operations::add` / `subtract` end-to-end (~1 hr)

These already have strong posts at the trait level (`add_post` /
`sub_post` over the `f_repr` view) and the underlying Portable free
fns prove exactly that. AVX2 needs the Vec256 ↔ f_repr translation
lemma. Should mirror Step 7's structure; pair them since the lemmas
are nearly identical.

### Step 9 — Per-method commute lemmas for the rest of the arithmetic
trait methods (~3-4 hr total, one method at a time)

`infinity_norm_exceeds`, `decompose`, `compute_hint`, `use_hint`,
`montgomery_multiply`, `shift_left_then_reduce`, `power2round`. Each
gets:
- Final post in `Hacspec_ml_dsa.*` cite (most are already in this
  shape — see `Simd.Traits.Specs.*_lane_post` predicates).
- Per-method translation lemma in `Commute.Chunk.fst` for the AVX2
  side (Vec256 ↔ f_repr).
- Free-fn bodies discharged. Portable's free fns currently cite
  `Spec.MLDSA.Math.*` (obsolete) — migrate to `Hacspec_ml_dsa.*` as
  part of finishing each method.

After Step 9 the Simd.Portable + Simd.Avx2 impl methods on
arithmetic primitives should all be admit-free. The encoding +
sample methods are deferred to later steps.

### Step 10 — `Libcrux_ml_dsa.Arithmetic.power2round_vector` /
`use_hint` end-to-end (~1 hr)

These wrappers in `src/arithmetic.rs` currently have body admits and
empty (`l_True`) posts. Target: state the proper per-vector post —
each `re_vector[i]` has `Hacspec_ml_dsa.Arithmetic.power2round`
applied to it (or `use_one_hint` for `use_hint`), the rest is
unchanged. Loop invariants thread the partial result through.

### Step 11 — Encoding methods, two waves

**Wave 11A** (~3-4 hr): extend `fstar-helpers/fstar-bitvec/BitVecEq.fst`
with offset-encoded `bit_pack` / `bit_unpack` variants needed for
`gamma1_*` and `t0_*` (each value `v` is packed as `b - v` over a
signed range — ML-KEM's existing helpers don't cover this).

**Wave 11B** (~6-8 hr): one method at a time, take each of
`gamma1_serialize`, `gamma1_deserialize`, `t0_serialize`,
`t0_deserialize`, `t1_serialize`, `t1_deserialize`,
`commitment_serialize`, `error_serialize`, `error_deserialize` and
finish it end-to-end:
- Trait post strengthened to `BitVecEq.int_t_array_bitwise_eq`
  against `Hacspec_ml_dsa.Encoding.{simple_bit_pack, bit_pack, …}`.
- Portable free fn's existing strong post (it already cites
  `bit_vec_of_int_t_array`) carried through.
- AVX2 free fn's strong post added (currently mid-body admit at
  prefix).
- Both impl methods discharge cleanly.

The mid-body admits in `src/simd/avx2/encoding/{gamma1,t0,t1,error}.rs`
get removed as we go.

### Step 12 — `Operations::ntt` / `invert_ntt_montgomery` per-poly post (USER lane)

Tier-3 chain of 8 layer-step lemmas with BitRev₈ zeta indexing in
`Hacspec_ml_dsa.Commute.Chunk.fst::lemma_ntt_full_commute`. Wait for
ML-KEM's analog (`libcrux-ml-kem/proofs/manual-proof-targets.md`
USER-2) to land as a template. Once `ntt` is finished, `invert_ntt`
and the AVX2 INVNTT layers are direct adaptations.

### Step 13 — Phase 4: spec migration & cross-spec test activation

Per `sprint-plan.md` Phase 4 (hours 44-48):
- Delete `Spec.MLDSA.Math`, `Spec.MLDSA.Ntt`, `Spec.Intrinsics`.
  Anything still citing them at this point is a Step 9/10/11 bug.
- Activate cross-spec tests per `MLDSA_STATUS.md` "Cross-spec test
  activation checklist" (~30 min).

Historical context (earlier 2026-04-27/28 Funarr-unblock session):

This doc was originally the resume entrypoint after the **2026-04-27/28
Funarr-unblock session**, which moved the empirical baseline from 11
modules verified (1 Funarr error) to **52 modules verified out of 60
invoked, 25 remaining errors**.

---

## What changed in the previous session

Five commits on `ml-dsa-proofs`:

| SHA | Subject |
|---|---|
| `f9a1798ac` | Phase 2A/3A coord: capture Phase-1 carryover findings |
| `04fd066f0` | Phase-1 rework: fix Specs.fst lemmas, relax over-strong posts, fix AVX2 reduce loop |
| `8ff59f350` | MLDSA_STATUS: record Phase-1 rework + identify Funarr as gating blocker |
| `42d4a3347` | **core-models: fix from_fn/BitVec::from_fn F\* extraction** (the unblock) |
| `1c827fab7` | Phase-1 traits.rs: fix Eta enum projection + Seq.length well-formedness |

Substantively:

1. **Phase-1 lemmas in `Simd.Traits.Specs.fst` repaired** —
   `lemma_decompose_lane_lookup`, `lemma_compute_hint_lane_lookup`, and
   `lemma_use_hint_lane_lookup` had their `gamma2 ∈ {95232, 261888}`
   constraint hoisted into `requires` so F\* can discharge the
   `Hacspec_ml_dsa.Arithmetic.{decompose,make_hint,uuse_hint}` precondition
   when typechecking the ensures.
2. **Three over-strong Phase-1 posts relaxed**:
   - `infinity_norm_exceeds_post` → raw signed abs (was centered `coeff_norm`).
   - `reduce_lane_post` → centered Barrett range (was unsigned `[0, q)`).
   - `montgomery_multiply_lane_post` → mathematical `int` arithmetic (was overflowing i64).
3. **AVX2 `Operations::reduce` impl bug fixed** — the body was reducing
   only 4 of 32 SIMD units (indices 0/8/16/24); replaced with a proper
   `for i in 0..simd_units.len()` loop matching the portable impl.
4. **Funarr/Bitvec `from_fn` F\* extraction unblocked** in core-models'
   Rust source (`crates/utils/core-models/src/abstractions/{funarr,bitvec}.rs`).
   The hax-generated F\* `replace` overrides for `FunArray::from_fn` and
   `BitVec::from_fn` declared a single implicit type slot (`#v_T`) but
   hax extracts call sites with two implicits (the impl-block's `T` plus
   the closure's auto-inferred `F: Fn(u64) -> T`). Added an `#_v_F: Type0`
   absorbing slot to both override signatures plus passed it explicitly
   at the in-replace call sites. Single-line change per file; persistent
   across `./hax.sh extract` runs.
5. **Trait-side regressions surfaced by the unblock** in `traits.rs`:
   `error_deserialize`'s post used `v $eta` against the `Eta` enum; the
   three `rejection_sample_*` posts had `Seq.index out_future i` without
   the `i < Seq.length out_future` constraint. Both fixed.

The four Phase-1-pre-existing `bounded_{add,sub}_{pre,post}` SMTPat-bridge
lemmas (in `traits/specs.rs:292-368`) are now `admit ()`. They are
documented in `outstanding-admits.md`. The `b1+b2 ≤ u32::MAX` constraint
is genuinely too loose to discharge `int_is_i32(lhs[i]+rhs[i])` (needs
`≤ i32::MAX`); a USER-lane fix tightens the constraint.

---

## Current empirical state (after 2026-04-28 cleanup pass)

Run `./hax.sh prove` from `libcrux-ml-dsa/`:

| Metric | Value |
|---|---|
| Modules invoked | 97 |
| `[CHECK]` mode (full F\* check) | 39 |
| `[ADMIT]` mode (`--admit_smt_queries true`) | 58 |
| Modules verified by F\* | **97** |
| F\* errors reported | **0** |
| Make-level failures | 0 |

The 25-error triage that follows is **historical** — it documents how each
of those errors was closed in the 2026-04-28 session. See the "Active
admits" section of [`outstanding-admits.md`](outstanding-admits.md) for
the per-function admit annotations introduced to close them; those are
the next session's targets if the goal is to lift the admits to real
proofs.

The 25 errors fall into 8 files. See [`outstanding-admits.md`](outstanding-admits.md) for full descriptions; below is the triage:

### Error triage by trait-boundary position

**Below the trait** (impl-side, prove the trait posts) — 6 files / 22 errors:

| File | Errors | Implements | Trait post strength | Disposition |
|---|---:|---|---|---|
| `Simd.Avx2.Invntt` | 15 | `invert_ntt_montgomery` | 🟡 bounds-only | **Pre-budgeted admit** (Wave 3E, USER lane). Add `#[hax_lib::fstar::verification_status::panic_free]` on the layer\_1/layer\_2 helpers; document in `outstanding-admits.md`. ~10 min. |
| `Simd.Avx2.Encoding.Gamma1` | 4 | `gamma1_{serialize,deserialize}` | 🟡 length-only | Wave 3A iv. Either admit at `panic_free` parity with the 🟡 trait post, or lift to bitvec post (deferred per existing admit doc). ~30 min if admitted. |
| `Simd.Avx2.Encoding.Error` | 1 | `error_{serialize,deserialize}` | 🟡 / **✅ strong** | Wave 3A iii. Single error — likely hits the relaxed `error_deserialize` post. Investigate first. ~20 min. |
| `Simd.Avx2.Encoding.T0` | 1 | `t0_{serialize,deserialize}` | 🟡 / 🟡 | Wave 3A iv. ~20 min. |
| `Simd.Avx2.Encoding.T1` | 1 | `t1_{serialize,deserialize}` | 🟡 / **✅ strong** | Wave 3A iv. ~20 min. |
| `Simd.Portable.Arithmetic` | 1 | line 738 in `infinity_norm_exceeds`'s body | (the post was relaxed in this session) | The body has an `assert (v normalized == abs (v coefficient))` that may need tweaking under the relaxed post. ~15 min. |

**Above the trait** (consumer side, *use* the trait posts) — 2 files / 2 errors:

| File | Errors | Uses trait methods | Strong-post reliance | Disposition |
|---|---:|---|---|---|
| `Libcrux_ml_dsa.Arithmetic` | 1 (Err 54) line 199 | likely `add`/`subtract`/`decompose`/`reduce` | **Yes** — depends on Phase-1 strong posts | Inspect — could be a small mismatch with the relaxed `reduce_lane_post` (centered vs unsigned). ~15 min. |
| `Libcrux_ml_dsa.Sample` | 1 (Err 228) line 1233 | `rejection_sample_*` | No (bounds-only) | The post was tightened in `1c827fab7` with the `Seq.length` constraint; consumer might need a small adjustment. ~15 min. |

### Strong-post dependence summary

- Strictly relying on Phase-1 strong posts: **2–4 of 25** (the above-trait `Arithmetic.fst:199`; the strong-post sub-methods buried in T1/Error files; potentially `Portable.Arithmetic:738`).
- Length/bitvec/bounds-only post side (still T=🟡, will become strong in waves 2D/3A iv): **7–9 of 25** (encoding modules + `Sample.fst`).
- Pre-budgeted INVNTT admit zone (USER-lane wave 3E, expected admit): **15 of 25**.

---

## Recommended next-up order

The order optimizes for "biggest reduction in error count per minute spent."

### Step 1 — INVNTT pre-budgeted admit (~10 min, kills 15 errors)

Add `#[hax_lib::fstar::verification_status::panic_free]` (or `admit()` in
the function body) to whichever helper inside `src/simd/avx2/invntt.rs`
houses the layer\_1 / layer\_2 4-zeta-parallel butterflies. The exact line
is reported by F\* as `Libcrux_ml_dsa.Simd.Avx2.Invntt.fst:894-941` —
inspect the extracted F\* to find which Rust function emits there.
Document in `outstanding-admits.md` with the existing pre-budgeted
admit entry. Re-run `./hax.sh prove`; expect error count to drop
from 25 → ~10.

### Step 2 — Three single-error files (~45 min total, kills 3 errors)

Each is a small Phase-1 leftover-rough-edge of the kind we resolved
already in `1c827fab7`. Investigate in order:

- **`Simd.Portable.Arithmetic.fst:738`** in `infinity_norm_exceeds` body.
  The session relaxed the post on this method (raw signed abs instead
  of `coeff_norm`); the impl's internal assertion `assert (v normalized
  == abs (v coefficient))` may now be either redundant or shaped
  slightly wrong. Likely a 1-line tweak in
  `src/simd/portable/arithmetic.rs:380-410`.
- **`Libcrux_ml_dsa.Arithmetic.fst:199`** Err 54. Likely a centered-vs-unsigned
  mismatch from the `reduce_lane_post` relaxation. Inspect what
  `Libcrux_ml_dsa.Arithmetic.*` does at that line.
- **`Libcrux_ml_dsa.Sample.fst:1233`** Err 228. Likely consumer-side
  adjustment for the new `Seq.length` constraint on rejection_sample
  posts; or it's one of the previously-broken `bounded_*` SMTPats now
  failing to fire.

After Step 2, expected error count: **~7 errors** (the 7 in AVX2
encoding modules).

### Step 3 — AVX2 encoding errors (~1.5–2 hr, kills remaining 7)

Two paths:

A. **Admit-parity path** (~30 min): bring the AVX2-side to the same
   `panic_free` admit posture the trait posts already are at (T=🟡 for
   all 7 encoding methods). Mark the AVX2 encoding free functions in
   `src/simd/avx2/encoding/{gamma1,t0,t1,error}.rs` as `panic_free`.
   This matches the trait-side strength and gets the count to **0
   errors** at the cost of leaving real proof work for waves 2D / 3A iv.

B. **Bitvec-strength path** (~1.5–2 hr): start wave 2D — strengthen
   the trait posts to use `BitVecEq.int_t_array_bitwise_eq` against
   `Hacspec_ml_dsa.Encoding.{simple_bit_pack,bit_pack,...}`, then
   discharge in both portable and AVX2 impls. This is the proper Phase
   2D / 3A iv work; do it when bitvec-helper extensions are ready in
   `fstar-helpers/fstar-bitvec/BitVecEq.fst`.

Recommendation: take path A for now; revisit B once ML-KEM's analogous
`serialize_post_N`/`deserialize_post_N` shapes have stabilized
(coordinate via `libcrux-ml-kem/proofs/manual-proof-targets.md`).

### Step 4 — `bounded_{add,sub}_{pre,post}` constraint tightening

In `src/simd/traits/specs.rs:292-368`, change `b1 + b2 <= 4294967295`
to `b1 + b2 <= 2147483647` (i32::MAX), then remove the four `admit ()`
bodies. Verify each `reveal_opaque` body discharges. ~20 min total.

This is **separate from the count-reduction goal** — these are SMTPat
helpers that fire automatically when downstream code uses
`add_pre`/`add_post`. Tightening the constraint fixes a soundness
concern (currently those lemmas admit a false claim).

### Step 5 — Wave proper proofs (Phase 2A/2B/3A waves)

After Steps 1–4 the count is 0 and the foundation is solid. Then begin
the actual wave proof work per `sprint-plan.md`:
- **Wave 2A** (portable add/subtract/infinity\_norm\_exceeds/reduce):
  the underlying free functions are already verified (CHECK mode); the
  thin-wrapper impl methods in `src/simd/portable.rs` need
  `#[requires]+#[ensures]` annotations matching the trait pre/post.
  Currently the impl-Operations file (`Simd.Portable.fst`) is in
  ADMIT mode; lifting it to CHECK mode is the wave-2A deliverable.
- **Wave 3A wave (i)** (AVX2 add/subtract/montgomery\_multiply/reduce):
  same shape on the AVX2 side. `Simd.Avx2.Arithmetic.fst` already
  passes; `Simd.Avx2.fst` (the impl-Operations file) is ADMIT-mode.

The thin-wrapper rule (per the previous session's guidance from the
user):

> impls do not contain any interesting code; each impl function is a
> one-line wrapper around a non-impl function with exactly the same
> pre- and post-conditions.

Allowed admit shapes: `#[hax_lib::fstar::verification_status::panic_free]`
**on the free function only** (never on impl methods), or
`hax_lib::fstar!("admit()")` in the body of the free function.
**`verification_status(lax)` is forbidden everywhere.**

---

## Pre-flight checklist for the next session

1. `cd libcrux-ml-dsa && git rev-parse HEAD` — confirm tip matches
   `MLDSA_STATUS.md`.
2. `git status` — clean working tree.
3. `./hax.sh prove 2>&1 | tail -22` — confirm 25 errors / 52 verified
   / 60 invoked baseline. If counts differ, something has drifted; do
   not start wave work until reconciled.
4. Read `outstanding-admits.md` "Active admits" section; the 4
   `bounded_*` lemmas are admitted with `admit ()` and surface in
   any reasoning about `add_pre`/`add_post` — keep this in mind.
5. Read `MLDSA_STATUS.md` per-method P/A columns for the strong-post
   coverage.

---

## Hard rules carried forward from previous sessions

1. **Pre-conditions** in `traits.rs` and `traits/specs.rs` may be
   conjunctively strengthened to match what the impls actually need
   for panic freedom (audit landed at `e289170ea`); never weakened in
   a way that would break a downstream caller's reliance. Posts may be
   conjunctively strengthened.
2. **Every function we touch leaves in final form.** Do not remove a
   body admit without simultaneously bringing the function's post to
   its target shape (canonical `Hacspec_ml_dsa.*` cite, full functional
   correctness, loop invariants threading the strong post). A
   "weak-post + admit-removed" pass is forbidden — it leaves the proof
   obligation unchanged but consumers see the function as done.
3. **20-minute wall-clock per proof attempt.** On overrun, **revert
   your edits to the function** so the next session starts from a
   known-good state, document the obstacle in `outstanding-admits.md`,
   and pick a different function. Do NOT leave a body admit and call
   it "in progress."
4. **Cite `Hacspec_ml_dsa.*` only** in new posts; never
   `Spec.MLDSA.Math` or `Spec.MLDSA.Ntt` (obsolete, deleted in Phase 4).
5. **Per-element commute lemmas** go in
   `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
   (create if absent — it doesn't yet exist; Step 6 in the work order).
6. **Never `verification_status(lax)`** anywhere. The only allowed
   admit shapes are `panic_free` on free functions and
   `hax_lib::fstar!("admit ()")` in free-function bodies, and only for
   pre-budgeted USER-lane items already documented in
   `outstanding-admits.md`. Anything outside that set is a Hard Rule
   #2 violation.
6. **impls are thin one-line wrappers** with `#[requires]+#[ensures]`
   identical to the underlying free function's; the free function
   carries the proof body / admit / panic\_free.

---

## Files to read in order at the start of the next session

1. `libcrux-ml-dsa/MLDSA_STATUS.md` (status snapshot).
2. `libcrux-ml-dsa/proofs/next-session-plan.md` (this file).
3. `libcrux-ml-dsa/proofs/outstanding-admits.md` (active admits).
4. `libcrux-ml-dsa/verification_result.txt` (current prove output).
5. Skim `libcrux-ml-dsa/proofs/sprint-plan.md` §Phase 2 and §Phase 3 if
   committing to wave-proper proof work.
6. `git log --oneline -10` for the last 10 commits' context.

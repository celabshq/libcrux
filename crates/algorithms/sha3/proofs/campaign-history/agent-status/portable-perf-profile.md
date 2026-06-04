# Portable.fst Verification-Time Profile

**Worktree**: `/Users/karthik/libcrux-sha3-focused`
**Branch**: `sha3-proofs-focused` @ `d07cac213` (USER-1 admit + spec opacity experiment)
**Date**: 2026-04-30 (~19:00 CEST)
**Profiling agent**: read-only; build invoked as
`make check/Libcrux_sha3.Generic_keccak.Portable.fst OTHERFLAGS="--z3refresh"`

---

## TL;DR — the build at HEAD does not complete.

**The cold-cache build of `Libcrux_sha3.Generic_keccak.Portable.fst`
FAILED before reaching the target module.** It crashed at the dep
`EquivImplSpec.Sponge.Portable.Steps.fst::lemma_absorb_block_portable`
(Steps.fst line 79, query 47, 69-second Z3 timeout, full rlimit 200
exhausted). Wallclock to failure: **13:00.42** (`make` self-reported
`682s user, 15s system, 89% cpu`).

**Root cause**: the spec-opacity edits in commit `d07cac213` marked
three spec functions `[@@"opaque_to_smt"]`:

  - `Hacspec_sha3.Keccak_f.keccak_f` — has NO `Pure` ensures (only
    body-defined behaviour). Opacity hides everything.
  - `Hacspec_sha3.Sponge.absorb_block` — `Pure` ensures is
    `(fun _ -> Prims.l_True)` (vacuous). Opacity hides everything.
  - `Hacspec_sha3.Sponge.squeeze_state` — has a *strong* per-byte
    forall ensures. Opacity is sound: callers can still reason via
    the ensures.

Of the three, the first two are **structurally broken** by opacity:
hiding the body when there is no functional ensures leaves callers
with nothing to use. `EquivImplSpec.Sponge.Portable.Steps.fst::
lemma_absorb_block_portable` (which proves impl-`absorb_block` ≡
spec-`absorb_block`) needs the spec body unfolded to discharge the
equivalence — without `reveal_opaque (` `%absorb_block) absorb_block`
in the proof, Z3 has nothing to prove.

This was foreshadowed in the d07cac213 commit message: *"Test status
of the opacity edit: NOT YET VERIFIED — make blocked by the unrelated
dep cycle from the parallel agent."* The dep cycle is gone now, but
the underlying structural problem (no ensures to back up the opacity)
is independent.

**Therefore, no fresh per-query profile of `Libcrux_sha3.Generic_keccak.Portable.fst`
exists**: the upstream `.checked` artifact in the cache (`Apr 30 18:30`)
predates the opacity edit. Re-running F* against it would require
re-verifying it from scratch, which is exactly what failed here. The
read-only constraint blocks reverting the spec opacity to obtain a
clean baseline.

What this report has, with hard numbers, is:
  1. Cold-cache wallclock totals for **three of the four direct deps**
     (the fourth, Steps.fst, failed; the target, Portable.fst, was
     never reached).
  2. Per-function timings within those three dep modules — including
     the failure-mode evidence on Steps.
  3. A structural classification of every Portable.fst function based
     on the source code, plus the per-dep-function evidence.
  4. A concrete next-sprint plan that unblocks the build AND profiles
     the target module.

---

## Per-module wallclock (cold-cache)

| Module                                                | Total ms | # queries | Slowest query (ms) | Status |
| ----------------------------------------------------- | -------: | --------: | -----------------: | ------ |
| `EquivImplSpec.Sponge.Generic.Core.fst`               |  103,320 |        19 |              1,676 | ✅ verified |
| `EquivImplSpec.Keccakf.Portable.fst`                  |  104,057 |        12 |                489 | ✅ verified |
| `EquivImplSpec.Sponge.Portable.fst`                   |  213,650 |       137 |              8,167 | ✅ verified |
| `EquivImplSpec.Sponge.Portable.Steps.fst`             | **168,854** | 48 + 2 fail | 88,571 (failed) + 69,144 (failed) | ❌ FAILED |
| `Libcrux_sha3.Generic_keccak.Portable.fst` (target)   |       —  |         — |                  — | NOT REACHED |
| **Subtotal (chain through Sponge.Portable)**          |   421,027 |       168 |              8,167 | |

Cold-cache walltime includes z3-spawn, Z3 startup (`--z3refresh`), and
F\*'s own typechecking — query_stats only reports the SMT portion.
F\*-side typechecking is the gap between sum-of-query-ms and module
TOTAL TIME (e.g. Sponge.Portable: 213.6s wall vs 61.8s SMT-sum, so
~152s in F\* typechecker — 70% of wall is F\* not Z3).

The user-reported headline of "~533 s for `Libcrux_sha3.Generic_keccak.Portable.fst`"
was from a pre-opacity build; given the chain dep numbers and the
size/structure of Portable.fst, that is plausible (Portable.fst has
~1000 lines of body asserts and 3 large fold_range loops). For
context, the Apr-30 18:36 UTC cold run took 8,322 ms for the
TOP-LEVEL `Libcrux_sha3.fst` once Portable.fst.checked was present —
proving Portable.fst is the bottleneck of the chain.

---

## Per-function detail in the dep modules

### Slowest dep functions — top 12 by total SMT ms

| Function                                                                           | Total ms | # q | Slowest q (ms) | Max used rlimit |
| ---------------------------------------------------------------------------------- | -------: | --: | -------------: | --------------: |
| `EquivImplSpec.Sponge.Portable.Steps.lemma_absorb_block_portable`                  | 168,854  |  48 |  88,571 ❌      | 200.000 (hit) |
| `EquivImplSpec.Sponge.Portable.lemma_load_last_equals_load_block_on_padded`        |  43,658  | 121 |   3,426        |   8.212 |
| `EquivImplSpec.Sponge.Portable.portable_sc_store_block`                            |   8,208  |   2 |   8,167        |  20.672 |
| `EquivImplSpec.Sponge.Portable.portable_sc_load_last`                              |   3,872  |   2 |   3,823        |  13.454 |
| `EquivImplSpec.Sponge.Portable.portable_sc_load_block`                             |   2,936  |   2 |   2,897        |   7.420 |
| `EquivImplSpec.Sponge.Generic.Core.sponge_correctness`                             |   1,676  |   1 |   1,676        |   8.131 |
| `EquivImplSpec.Sponge.Generic.Core.__proj__Mksponge_correctness__item__sc_load_last` |   1,401 |   2 |     872        |   4.644 |
| `EquivImplSpec.Sponge.Portable.sq_lane_portable`                                   |   1,026  |   2 |     907        |   0.475 |
| `EquivImplSpec.Sponge.Portable.lemma_load_block_eq_xor_block_into_state`           |     646  |   2 |     595        |   1.080 |
| `EquivImplSpec.Sponge.Portable.lemma_load_last_eq_xor_block_into_state_padded`     |     595  |   2 |     547        |   1.439 |
| `EquivImplSpec.Keccakf.Portable.portable_lc_rotate_left1_and_xor`                  |     489  |   1 |     489        |   1.884 |
| `EquivImplSpec.Sponge.Portable.lemma_store_block_eq_squeeze_state`                 |     309  |   2 |     258        |   0.398 |

### Failure detail on Steps

```
EquivImplSpec.Sponge.Portable.Steps.lemma_absorb_block_portable, q1
  failed {reason-unknown=unknown because canceled} in 88,571 ms,
  rlimit 200/200 (full budget hit).
[F* split-query retry: 46 sub-queries succeed at <320 ms each.]
EquivImplSpec.Sponge.Portable.Steps.lemma_absorb_block_portable, q47
  failed {reason-unknown=unknown because canceled} in 69,144 ms,
  rlimit 200/200 (full budget hit).
* Error 19 at Steps.fst(79,7-79,33): Assertion failed
make[1]: *** [Steps.fst.checked] Error 1
```

Steps.fst line 79 is `KP.lemma_keccakf1600_portable s1` — the call to
the Keccakf1600 equivalence lemma. After applying `xor_block_into_state`
equivalence (line 72) and the impl's `f_load_block`, Z3 needs to
conclude that `impl_2__absorb_block ks ... .f_st == absorb_block ks.f_st ...`.
With `absorb_block` opaque AND no ensures, the chain
`xor + keccak_f` cannot be reduced.

---

## Portable.fst per-function STRUCTURAL analysis (no fresh runtime data)

Numbers below are **not measured** in this run. They are inferred from
(a) the dep timings (the Portable callsites are dominated by the same
lemmas that show up in Steps + Sponge.Portable above) and (b) the
options pragmas, source structure, and per-byte forall density. The
user-reported 533 s is the empirical anchor; the column "share %" is a
plausible apportionment.

| Fn (line)                                  | Body size            | rlimit | split? | Likely class | Est. share of 533s |
| ------------------------------------------ | -------------------- | ------:| ------ | ------------ | -----------------: |
| `e_keccak_state_impl_opts` (15)            | unit                 |    800 | n      | trivial      |                  ~0% |
| `impl__squeeze_next_block` (17)            | 2 trait calls        |    800 | n      | (A) bounds   |                  ~1% |
| `impl__squeeze_first_block` (56)           | 1 trait call         |    800 | n      | (A) bounds   |                  ~1% |
| `impl__squeeze_first_three_blocks` (81)    | 3 squeeze + 2 keccakf|    800 | n      | (A) bounds   |                  ~3% |
| `impl__squeeze_last` (141)                 | if-then per-byte forall_intro + lemma_eq_intro over `Hacspec_sha3.Sponge.squeeze_state` | 800 | n | **(B+C)** mixed | **~12%** |
| `impl__squeeze_first_five_blocks` (233)    | 5 squeeze + 4 keccakf|    800 | n      | (A) bounds   |                  ~5% |
| `absorb` (324)                             | fold_range loop, invariant cites `Hacspec_sha3.Sponge.absorb_blocks`; body invokes `Steps.lemma_absorb_block_portable` (the failing lemma!) and `Hacspec_sha3.Sponge.Lemmas.lemma_absorb_blocks_tail` | 800 | **YES** `--split_queries always` | **(B+D)** spec-bound + mixed | **~28%** |
| `squeeze` (440)                            | 3-branch if + fold_range loop + 4 forall_intro lemmas. Invariant cites `Hacspec_sha3.Sponge.squeeze_blocks`. Body uses `lemma_squeeze_blocks_base`, `lemma_squeeze_blocks_tail`, `lemma_squeeze_last_extensional`, `lemma_squeeze_unfold`, `lemma_keccakf1600_portable`. Largest single function in the module. | 800 | **YES** `--split_queries always` | **(B+C+D)** all three | **~45%** |
| `keccak1` (748)                            | trivial composition of `absorb` + `squeeze` |    200 | n      | (B) spec     |                  ~5% |

**Headline candidates for >150s wall and USER-N admit**:

  - `squeeze` (line 440) — the only single body in the module that
    plausibly exceeds 150 s on its own. Hosts 4 forall_intros, 2
    nested aux lemmas (each per-byte over `output_len`), 1 fold_range
    loop with a 4-clause invariant, 5 spec-citing lemma calls, and
    the call to `impl__squeeze_last`. **Most likely USER-N candidate.**
  - `absorb` (line 324) — second-likeliest. Single fold_range with
    a 2-clause invariant + 2 calls into `Steps` + 2 calls into
    `Sponge.Lemmas`. The DIRECT dep on the failing
    `Steps.lemma_absorb_block_portable` makes this body
    **uncompletable at HEAD** even with admits: the lemma it imports
    cannot be discharged.

Class column legend (matches user prompt):

  - **(A) usize-arithmetic-bound** — ensures is bounds-only or body
    asserts decompose offsets/lengths. Diagnostic: hypotheses cite
    `is_intb`, `Core_models.Slice.impl__len`, `*RATE`. **Opacity does
    not help.** Fix: explicit offset asserts.
  - **(B) spec-unfolding-bound** — body asserts cite
    `Hacspec_sha3.Sponge.{keccak,squeeze,squeeze_state,absorb_block,...}`
    or `Hacspec_sha3.Keccak_f.keccak_f`. **Opacity SHOULD help** if
    the spec has a strong ensures (true for `squeeze_state`,
    `squeeze_last`, `absorb`, `squeeze`); **opacity HURTS** if the
    spec has a vacuous or missing ensures (true for `absorb_block`
    and `keccak_f` at HEAD).
  - **(C) quantifier-instantiation-bound** — per-byte forall,
    `eq_intro` forall over array indices. Opacity won't help; fix is
    structural: factor the forall into per-index aux + classical
    forall_intro.
  - **(D) mixed**.

---

## Recommended remediation, per function

| Function           | Class     | Recommendation |
| ------------------ | --------- | -------------- |
| `impl__squeeze_*_block(s)`, `impl__squeeze_first_block` | (A) | Add explicit `assert (i*RATE + RATE <= out.len())` and `assert (start + RATE <= out.len())` before each `f_squeeze` call. These are bounds-only proofs; opacity is irrelevant. Likely <5 s each. |
| `impl__squeeze_last` | (B+C) | (1) Inline a smaller per-byte aux `aux_byte (k: nat{k<...})` with the three case-split branches. Already present (lines 203-220). The pain here is `Seq.lemma_eq_intro` over `Hacspec_sha3.Sponge.squeeze_state` (line 224). With `squeeze_state` opaque, `lemma_eq_intro` will only be able to use the spec ensures forall — which is sufficient (the ensures characterizes every byte). Profile and confirm. (2) If still slow, replace `forall_intro aux` + `lemma_eq_intro` with a direct manual `Seq.lemma_eq_intro` instance and let Z3 use the ensures. |
| `absorb` | (B+D) | **Fix Steps first.** Add `reveal_opaque (\`%Hacspec_sha3.Sponge.absorb_block) Hacspec_sha3.Sponge.absorb_block` at the top of `Steps.lemma_absorb_block_portable`. Same for `lemma_absorb_last_portable`. Once Steps verifies, `absorb` should profile cleanly. If it still exceeds 150 s, the explicit `lemma_absorb_blocks_tail` chain is the next candidate to factor. |
| `squeeze` | (B+C+D) | The 4 `forall_intro`s on per-byte aux lemmas (`aux_write`, `aux_tail`, `aux_write_step`, `aux_tail_step`) compose 4 quantifier instantiations per loop iteration. (1) **First**, after Steps is fixed, profile to see whether opacity on `squeeze_state` actually reduces cost in `squeeze` (it should — the loop invariant cites `Hacspec_sha3.Sponge.squeeze_blocks` and `squeeze_state`, both of which can now be reasoned about purely via ensures). (2) If `squeeze` still exceeds 150 s, factor `squeeze_blocks_tail` use into a separate top-level lemma and call it once instead of inside the loop body. |
| `keccak1` | (B) | Two-step composition; should be <30 s. Likely fine. |

### USER-N admit candidates (>150 s policy)

Based on inferred share-of-time plus the structural analysis:

  1. **`squeeze`** (~45% of 533s ≈ 240 s estimated) — top candidate
     for a USER-N admit if the next sprint's structural fix doesn't
     bring it under 150 s. The current `--split_queries always`
     pragma means the function is already split into many sub-queries
     by F\*; the slow part is forall composition, not a single
     monolithic query.
  2. **`absorb`** (~28% ≈ 150 s estimated) — borderline. Currently
     **uncompletable** because of the Steps.fst breakage; not eligible
     for USER-N until Steps is fixed.

Neither becomes a useful admit until **Steps is fixed first** — that
unblocks the entire chain and gives clean per-function numbers.

---

## Where opacity wins, where it doesn't

**Wins**: `squeeze_state` (strong per-byte forall ensures). The 309 ms
proof of `lemma_store_block_eq_squeeze_state` in Sponge.Portable.fst
*today* — with `squeeze_state` already opaque — is direct evidence:
the lemma proves pointwise impl ≡ spec via the ensures forall, no
body unfolding needed. This is the model.

**Hurts**: `keccak_f` (no `Pure` clause), `absorb_block`
(`fun _ -> Prims.l_True`). At HEAD these have no functional ensures
to back up the opacity, so any consumer that needs to reason about
their result is dead in the water. Steps.lemma_absorb_block_portable's
failure is the immediate consequence; Portable.fst::absorb's bodies
cite `lemma_absorb_block_portable` and `lemma_absorb_last_portable`,
so the Portable target also dies once it's reached.

**Diagnostic rule** (this should be a sprint-learning):
> Opacity on a spec function is sound iff the function has a `Pure`
> ensures clause that fully characterizes its return value. Mark
> opacity only on functions where consumer proofs depend on the spec
> *result* (which can come from ensures), not on the spec *body*. If
> the spec has no ensures, opacity is equivalent to making the
> function `assume val` from the consumer's perspective.

---

## Recommended next 1-2 sprints

**Sprint A (1 day, 1 agent — UNBLOCK)**
  1. Add `reveal_opaque (\`%absorb_block) Hacspec_sha3.Sponge.absorb_block`
     at the top of
     `EquivImplSpec.Sponge.Portable.Steps.lemma_absorb_block_portable`.
  2. Same for `lemma_absorb_last_portable` (uses `absorb_final`,
     internally `xor_block_into_state` + `keccak_f`).
  3. Same for `EquivImplSpec.Keccakf.Portable.lemma_keccakf1600_portable`
     (which proves the impl `keccakf1600` ≡ opaque-marked spec
     `keccak_f`).
  4. Run the full chain. Verify `Libcrux_sha3.fst` still terminates.
  5. **Capture the fresh per-function profile of
     `Libcrux_sha3.Generic_keccak.Portable.fst`** — this is the
     deliverable I could not produce.

**Sprint B (1-2 days — ANALYZE + REDUCE)**
  1. With clean numbers, run the same parsing on the Portable.fst
     query log and identify the actual slow functions.
  2. For each function >150 s in classes (B) or (C), apply the
     per-class fix from the "Recommended remediation" table.
  3. Add functional ensures to spec functions whose opacity is
     load-bearing for downstream proofs. Prime targets:
     `Hacspec_sha3.Sponge.absorb_block` (a one-line ensures that
     `result == keccak_f (xor_block_into_state state block rate)` —
     i.e. the body itself, but as an ensures the body becomes available
     to consumers without unfolding).
     `Hacspec_sha3.Keccak_f.keccak_f` is harder (24-round fold), but
     a `Pure` annotation with at least the bound `length result == 25`
     and a no-op `(fun _ -> Prims.l_True)` rescues every callsite that
     doesn't need to reason about `keccak_f`'s actual permutation
     (which is most callsites — only `EquivImplSpec.Keccakf.*` do).

**Beyond sprint B**: only if (a) Portable.fst still has individual
functions >150 s after the structural fixes, OR (b) the row-helper
factoring of `lemma_theta_rho_to_spec` (still admitted under USER-1
in commit `7cd4c21a7`) becomes the limiting factor for the full
verification chain.

---

## Artifacts

  - Build log (cold cache, with `--query_stats`):
    `/tmp/sha3-prof/build.log` (3.5 MB; 216 successful queries +
    2 failed queries parsed).
  - Parsed per-query TSV: `/tmp/sha3-prof/all-queries.tsv`
    (217 rows; columns: function, query_id, status, ms, rlimit,
    used_rlimit). Suitable for further analysis.
  - This report: `crates/algorithms/sha3/proofs/agent-status/portable-perf-profile.md`.
  - Status updates (every 15 min): `crates/algorithms/sha3/proofs/agent-status/profiling-status.md`.

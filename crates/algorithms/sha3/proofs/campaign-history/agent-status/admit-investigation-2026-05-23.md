# SHA-3 admit investigation — 2026-05-23

Investigation-only session. Five load-bearing admits profiled / analysed against
the per-admit closure plan. **No admit closed this session.** Implementation is
a separate sprint; this note hands off prescriptive fixes and a sprint-ordering
recommendation.

- Branch: `sha3-proofs-focused`, HEAD `d7ac1bddb`.
- Toolchain: F\* 2026.03.24 + hax `integer-lemmas` @ `952bee04` + hax-engine 0.3.6.
- Opam switch `hax` (makefiles/hax.py handle internally; not switched globally).
- Baseline build: `make -C crates/algorithms/sha3/proofs/fstar` clean modulo the 5 admits with `OTHERFLAGS="--z3rlimit_factor 4"`.

The cross-cutting punchline up front: items **3, 4, 5 share one root cause** — Z3
e-matching cliffs from the AVX2 / Arm64 SIMD lane-projection chain
(`{arm64,avx2}_lane → get_lane_u64{x2,x4}`) firing per-`i × per-lane` ghost-call
cross-products. Items **1, 2** are a separate family: missing Rust-side ensures
strength on the `Generic_keccak.Simd{128,256}.squeeze{2,4}` drivers (bounds only,
no per-byte lockstep invariant).

The cliff/cascade family (3, 4, 5) is the **same `k!61` fingerprint** as the
2026-05-05 AVX2 `load_block` cliff that closed via `[@@ "opaque_to_smt"]` on
`Hacspec_sha3.createi` + `lemma_load_lane_u64_lane_extensionality` SMTPat
narrowing (skill §1.5 worked example). The 2026-05-05 fix was at the upstream
spec layer; admits 3/4/5 sit at **downstream consumer sites** where that fix
doesn't reach. They need their own opacity layer, scoped to the lane-projection
chain.

---

## 2026-05-24 UPDATE — empirical findings from Track A and Track B attempts

Track A (`~/libcrux-sha3-track-a`, branch `sha3-proofs-track-a`) attempted admit 1
closure. Track B (`~/libcrux-sha3-track-b`, branch `sha3-proofs-track-b`)
attempted admits 4 and 5. **Both tracks closed 0 admits**, but both produced
strong validation of the diagnosis below + concrete handoff recipes for the
next attempts. The closure-plan effort estimates in this note were too
optimistic — both Path 1/2 ("inline lockstep / squeeze_blocks2 cherry-pick"
for admits 1/2) and "Fix 1 alone" (opacity for admits 4/5) were called
mechanical/low-variance; empirically each is medium-variance structural.

### Track A findings (admit 1)

- Path 1 (inline lockstep replacing `assume val lemma_squeeze2_arm64`) is
  **structurally infeasible as stated**: `Simd128.squeeze2`'s function-level
  ensures is bounds-only, and its `fold_range` loop_invariant carries no
  byteform predicate. F\* has no path from the (bounds-only) upper-bound
  loop_invariant back to the per-byte equivalence the lemma needs.
- Path 2 / "Path Z" (cherry-pick `squeeze_blocks2` from
  `~/libcrux-sha3-squeeze2 @ 1d35f933c`) **lands the helper cleanly**
  (`squeeze_blocks2` verifies on its own at 4× rlimit), but `squeeze2`'s
  body still cliffs — Z3 saturates on the first sub-VC. Same shape the
  squeeze2 branch's `7979e4371` resolved via `--admit_smt_queries true`
  (a workaround Track A correctly refused per the no-new-admits rule).
- The forall_intro fix from `squeeze2-body-2026-05-05.md` with `()` aux
  bodies **was validated as insufficient**: Z3 enters the same saturation
  pattern. Open question: do aux bodies that explicitly invoke
  `lemma_sq_lane_arm64_eq_squeeze_state` (rather than `()`) discharge?
  That's the next experiment for admit 1.
- Track A's worktree state: clean baseline at fork point; status notes
  + rollup committed at HEAD `448a61bca`.

### Track B findings (admits 4 + 5)

- Diagnosis was **empirically correct**. Fix 1+2 (opacity on
  `KA.{arm64,avx2}_lane` + per-`i` `match` destructure replacing
  `Classical.forall_intro byte_eq`) collapsed `k!61` from 128,229
  instances → 30 instances. **4000× reduction.** Strongest evidence
  to date that the opacity approach is correct in principle.
- **But it unmasked the next-down cascades** (skill §1.5.1 "layered
  cascades" prediction realised):
  - `Core_models.Ops.Index.f_index` interp at **2196 instances** (was
    fully masked behind `k!61`).
  - `Rust_primitives.Hax.Folds.fsti` `forall` interp at **1094**, from
    `load_block`'s heavy ensures clause's universally-quantified body.
  - `Core_models.Default` / `Core_models.Convert` interps totalling
    **~3000** — unrelated typeclass machinery firing under the new
    e-matching surface.
- **Fix 1 alone regresses a previously-passing baseline lemma**:
  `lemma_sq_lane_avx2_eq_squeeze_state` (`Sponge.Avx2.fst:378+`) goes
  from <10 s baseline to **98 s saturating rlimit 600/600**. The
  opacity introduction shifts an e-matching path; the squeeze lemma
  has no companion fix in scope. **Closure requires threading
  `lemma_avx2_lane_unfold` reveal into the squeeze lemma's asserts.**
- The Fix 3 layer (multi-pattern SMTPat narrowing) didn't help over
  Fix 1+2 — the next cliff isn't `arm64_lane`-shaped.
- **Closure requires a 4th-layer structural refactor**: extract
  `byte_eq` from inside `lemma_load_block_eq_xor_block_into_state_{arm64,avx2}`
  into a **standalone top-level lemma** (e.g.
  `lemma_load_block_byte_eq_arm64`) whose `requires` packages the
  `load_block` ensures conjunct **at index `i` as a per-`i` hypothesis**
  — not as a free `load_block`-style forall. This is the standard
  "iter-start snapshot + standalone bridge lemma" pattern from skill
  §7 ("Iter-start snapshot + standalone bridge lemma for opaque
  carryover after Seq.upd"), adapted to the per-`i` byte-equality
  shape. Expected to suppress the `f_index` / Folds-forall instances
  and clear the path.
- Track B's worktree state: clean baseline; status + rollup committed
  at HEAD `b701bf5eb`. All proof code reverted to base (byte-identical
  to `d8f65def4`).

### Revised closure recipes

**Admit 1 (Arm64 squeeze driver)** — Path Z (resumable):

1. Re-cherry-pick `1d35f933c` from `~/libcrux-sha3-squeeze2` to bring
   `squeeze_blocks2` + 3 step lemmas + one-liner driver back.
2. In `Simd128.squeeze2`'s else branch (just before the two
   `lemma_squeeze_final_reconcile_arm64 ... {0,1}` calls), add four
   `Classical.forall_intro` calls (prefix × 2 lanes, trailing × 2 lanes).
   **Aux bodies must explicitly invoke
   `lemma_sq_lane_arm64_eq_squeeze_state`** on the relevant ranges to
   reify the byte-level facts before the `()` rewriter would have
   fired. Track A validated that `()` does NOT work; the explicit
   invocation is the next experiment.
3. If the aux bodies discharge: drop `--admit_smt_queries true` from
   `lemma_squeeze_final_reconcile_arm64`'s push-options (per the prior
   note, the lemma's body falls out cleanly once preconditions are
   met explicitly). The driver `lemma_squeeze2_arm64` is the existing
   one-liner.
4. If they don't: Path Z is structurally infeasible without admits.
   Fall back is much-larger-scope (e.g. moving byte-level reasoning
   inside the Arm64 instance's `f_squeeze2` trait method ensures).

**Admit 2 (AVX2 squeeze driver)** — depends on admit 1's closure
pattern landing. Adds porting `lemma_squeeze_one_step_arm64` →
`lemma_squeeze_one_step_avx2` (the gap in `Avx2.Steps.fst` — file
ends at line 171 with `lemma_squeeze_last_avx2`) + `squeeze_blocks2`
→ `squeeze_blocks4` + the 3 step lemmas Arm64→AVX2.

**Admits 4 + 5 (byte_eq mirrored pair)** — 5-step recipe (Track B
agent's rollup):

1. Re-apply Fix 1 (opacity on `KA.{arm64,avx2}_lane` in
   `EquivImplSpec.Keccakf.{Arm64,Avx2}.fst` + `reveal_opaque` /
   `*_lane_unfold` helpers at SMTPat-bridge sites in
   `EquivImplSpec.Sponge.{Arm64,Avx2}.fst`).
2. Thread `lemma_avx2_lane_unfold` reveal into
   `lemma_sq_lane_avx2_eq_squeeze_state` to fix the Fix-1-induced
   regression in `EquivImplSpec.Sponge.Avx2.fst:378+`.
3. **Extract `byte_eq` into a standalone top-level lemma**
   `lemma_load_block_byte_eq_arm64` (and mirror) whose `requires`
   take the `load_block` ensures conjunct at index `i` directly as
   a per-`i` hypothesis. Caller threads the conjunct from
   `load_block`'s ensures into the call site.
4. `--admit_except` on the new standalone lemma to isolate;
   `--using_facts_from '* -Core_models.Default -Core_models.Convert'`
   to suppress the 3000-instance typeclass contribution.
5. Mirror for AVX2 (admit 5).

**Admit 3 (AVX2 store_block)** — unchanged from original recipe
(`store-block-avx2-discharge` structural split +
`loop-invariant-opacify` `LoopInv.fst` transplant). The `k!61` cascade
collapse Track B validated (4000×) is direct evidence that the same
opacity attack will work on admit 3's loop-invariant — Track C
inherits Track B's positive result + the standalone-lemma-extraction
discipline.

### Revised sprint ordering

1. **Re-attempt Track B** with the 5-step recipe above. Highest
   confidence: diagnosis empirically validated, foundation
   (Keccakf opacity) closes cleanly in 2.4 s. Closes 2 of 5.
2. **Re-attempt Track A** with the explicit-lemma aux bodies. Pivots
   on the open empirical question; if aux bodies discharge, closes
   admit 1, then admit 2 mechanically. If not, Path Z is dead and we
   need a different angle (or admit-with-justification).
3. **Track C** (admit 3). Largest scaffolding pre-drafted; benefits
   from Tracks A/B's validated patterns.

The Track A and Track B status notes + rollups in their respective
worktrees contain the full operational details (memory peaks, exact
file:line targets, what was tried and reverted).

---

## Admit 1: `lemma_squeeze2_arm64` (`assume val`) at `EquivImplSpec.Sponge.Arm64.Driver.fst:111`

### Failure shape

Not an SMT failure — `assume val`. No query is fired; the lemma is taken on
faith by callers (`lemma_keccak2_arm64`, line 162, calls it twice per lane).

The proof gap is **structural**: `Libcrux_sha3.Generic_keccak.Simd128.squeeze2`
(`crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Generic_keccak.Simd128.fst:191`)
currently has a **bounds-only ensures**:

```fstar
(ensures fun (out0_future, out1_future) ->
  len out0_future = len out0 /\ len out1_future = len out1)
```

There is no per-byte invariant linking `out{0,1}_future` to
`Hacspec_sha3.Sponge.squeeze`. The driver lemma's ensures (per-lane equivalence
to scalar `Sponge.squeeze`) cannot be discharged by invoking `squeeze2` and
reading the ensures.

The per-lane lockstep induction has all its **building blocks already in
place** (`EquivImplSpec.Sponge.Arm64.Steps.lemma_squeeze_one_step_arm64` at
line 243). What's missing is either (a) the inline loop-invariant proof on
`squeeze2` that uses `lemma_squeeze_one_step_arm64` 2× per iteration, or
(b) a Rust-side ensures strengthening + `squeeze_blocks2` helper.

### Asymmetry with sibling backend

There is no analogous backend that closes this; the equivalent **Portable**
discharge — `lemma_squeeze_portable` (`EquivImplSpec.Sponge.Portable.API.fst:98`)
— is itself elevated to `--z3rlimit 800 --split_queries always` and is on the
"watch list" in `sha3-sprint-todo.md` (item 7 of Suggested sprint order). The
Portable path proves via inline lockstep induction over `Generic_keccak.Portable.squeeze`'s
loop body; the squeeze body has a strong-enough invariant scaffolded inline.

### Root cause hypothesis

`Generic_keccak.Simd128.squeeze2`'s Rust-side ensures is too weak. There's no
upstream cascade to fix; the work is **proof construction**, not cascade
defusing.

### Closure plan

Two paths, in increasing structural depth:

1. **Inline lockstep (recommended for first attempt; minimum-disruption).**
   Replace `assume val lemma_squeeze2_arm64` with a `let` body that calls
   `Libcrux_sha3.Generic_keccak.Simd128.squeeze2` (so its body unfolds into
   scope), inducts over `blocks`, applies `lemma_squeeze_one_step_arm64` 2×
   per iteration (once per lane), and concludes via byteform reconciliation.
   The trailing-partial-block case is handled by `lemma_squeeze_last_arm64`
   (`Sponge.Arm64.Steps.fst:179`). See `BRIEF_squeeze_steps.md` for the
   handoff sketch. Estimated effort: ~1 session (mechanical).

2. **Rust-side ensures strengthening via `squeeze_blocks2`.** Per the
   `sha3-byteform-migration-squeeze2` branch (HEAD `7979e4371`, worktree
   `~/libcrux-sha3-squeeze2`, 2 commits ahead). That branch factors squeeze2's
   multi-block loop into a `squeeze_blocks2` helper with a strong per-byte
   ensures against `Hacspec_sha3.Sponge.squeeze`'s byteform on
   `[0, output_blocks*RATE)`. The helper verifies cleanly (394 sub-queries,
   no admits per branch commit `1d35f933c`). The final commit on that branch
   stripped the squeeze2 inline proof scaffolding and re-admitted squeeze2's
   body (`--admit_smt_queries true`), but `squeeze_blocks2` is preserved.
   Reviving the stripped step lemmas (`lemma_squeeze_prefix_preserved_arm64`,
   `lemma_squeeze_trailing_byteform_arm64`, `lemma_squeeze_final_reconcile_arm64`)
   and removing the `--admit_smt_queries true` on squeeze2 closes the loop.
   The reconcile lemma's body is the only remaining open item.
   Estimated effort: ~1 session, given the branch as prior art.

Approach (1) keeps Rust source and `squeeze2`'s ensures untouched; approach (2)
strengthens the Rust ensures, which means the discharged lemma becomes a
one-liner. Approach (2) is structurally cleaner long-term; approach (1) is
shorter to ship.

### Dependencies / sprint ordering

- **No upstream dependency.** All ingredients (`lemma_squeeze_one_step_arm64`,
  `lemma_squeeze_last_arm64`, `lemma_squeeze_block_arm64`, etc.) are proven.
- Closing this **unblocks the Avx2 mirror (admit 2)** by establishing the
  pattern. Once the Arm64 closure shape is settled, porting to AVX2 is
  mechanical modulo the missing `lemma_squeeze_one_step_avx2` (see admit 2).

---

## Admit 2: `lemma_squeeze4_avx2` (`assume val`) at `EquivImplSpec.Sponge.Avx2.API.fst:87`

### Failure shape

Not an SMT failure — `assume val`. The AVX2 counterpart of admit 1.
`Libcrux_sha3.Generic_keccak.Simd256.squeeze4` has the same weak (bounds-only)
ensures as `squeeze2`.

### Asymmetry with sibling backend

**This is the canonical "Arm64-proves-but-Avx2-admits" sibling pair, except
Arm64 also admits it (admit 1).** The asymmetry is one layer deeper: the
**per-iteration step lemma** that supports the closure pattern exists for
Arm64 (`lemma_squeeze_one_step_arm64`, `Sponge.Arm64.Steps.fst:243`) but
**not** for AVX2 (`EquivImplSpec.Sponge.Avx2.Steps.fst` only has lemmas
through `lemma_squeeze_last_avx2` at line 171; no `lemma_squeeze_one_step_avx2`).

Mechanically the two backends use identical lane-projection structure
(`KA.{arm64,avx2}_lane` over the 25-element state array), so a port is shape-
preserving with `2 → 4` in the lane bound and the cor­responding intrinsic
typeclass. The Z3 budget at N=4 will likely be tighter; the AVX2 driver lemma
has 4× lanes' worth of lemma calls in its consumer (`lemma_keccak4_avx2`
calls it 4 times — `Avx2.API.fst:150–153`).

### Root cause hypothesis

Same as admit 1: weak Rust-side ensures on `squeeze4` + missing step lemma in
the equivalence Steps file. No upstream cascade.

### Closure plan

Two ordered sub-tasks:

1. **Port `lemma_squeeze_one_step_arm64` → `lemma_squeeze_one_step_avx2`** in
   `EquivImplSpec.Sponge.Avx2.Steps.fst`. The body is mechanical; the budget
   will scale roughly 2× (going from `lane < 2` to `lane < 4` doubles the
   per-`i` ghost-call cross-product feeding the SMT context). Watch for
   `--z3rlimit 400 --split_queries always` not being enough; may need
   `--using_facts_from '… -array_from_fn -rem_euclid'` per the AVX2
   `load_block` 2026-05-05 closure template (skill §1.5).
   Estimated effort: ~0.5–1 sprint.

2. **Discharge `lemma_squeeze4_avx2`** by inline lockstep (Approach 1 from
   admit 1) or by `squeeze_blocks4` Rust ensures (Approach 2). Same trade-offs;
   recommend doing (1) here once admit 1's pattern is settled.
   Estimated effort: ~1 sprint.

Total for admit 2: 1–2 sprints if admit 1 is closed first; longer if both
need to converge independently.

### Dependencies / sprint ordering

- **Depends on admit 1** for the closure pattern. Wait for that pattern to
  land, then port.
- The missing `lemma_squeeze_one_step_avx2` is also referenced obliquely by
  the existing `EquivImplSpec.Sponge.Avx2.lemma_load_block_eq_xor_block_into_state_avx2`
  proof shape (admit 5) — closing admit 5's cascade may inform how to bound
  the step lemma's SMT context.

---

## Admit 3: `Libcrux_sha3.Simd.Avx2.Store.store_block` body admit (`hax_lib::fstar!("admit()")`) at `Libcrux_sha3.Simd.Avx2.Store.fst:165` (Rust source: `crates/algorithms/sha3/src/simd/avx2/store.rs:74`)

### Failure shape

**This admit was profiled by the prior session on branch
`store-block-avx2-discharge` (HEAD `464a9914a`).** Profile data preserved at
`~/libcrux-sha3-store-avx2-discharge/crates/algorithms/sha3/proofs/agent-status/avx2-cliff-profile-progress.md`.
Reproducing the headline numbers here:

- Cliff queries in `store_block_full` (post-structural-split): q213, q240,
  q300 — each 78–86 s, all `failed {reason-unknown=unknown}`, all hit
  `rlimit 400.000 / 400`.
- qi.profile of q213 (84.3 s wall):
  - `quant-instantiations`: **10,768,398** (10.7 million)
  - `lazy-quant-instantiations`: 2,668,307
  - `missed-quant`: 5760 (cascade fingerprint)
- Top-1 quantifier: **`k!61` at 1,186,197 instantiations** (anonymous goal
  forall; corresponds to the `fold_range` loop_invariant body's 124-var outer
  forall — 4 buffers × byte-level forall conjunct, plus the arithmetic
  bridges, plus the `pure_post` continuation).
- Top-3 cascade tail: `Rust_primitives.Integers_pretyping_1eff91…` (358k),
  `projection_inverse_BoxBool_proj_0` (357k), `Prims_pretyping_f537159…`
  (328k) — all standard typing/refinement machinery re-firing per Skolemized
  goal term.

### Asymmetry with sibling backend

**Arm64 store_block IS fully proven.** Closed via the discharge sequence in
commits `c14f94d2c` / `29424f593` / `83d1a04c2` on `sha3-proofs-focused`.
The Arm64 proof template uses **four cooperating ingredients**, *none* of
which is in place on the AVX2 side at HEAD `d7ac1bddb`:

1. **`store_u64x2x2` per-iteration wrapper** (`Libcrux_sha3.Simd.Arm64.Store.fst:26`):
   factors the inner loop body's two stores into a single helper with strong
   per-byte ensures over a 16-byte window. Wrapper is verified at
   `--z3rlimit 400 --split_queries always` (line 13).
2. **`StoreBlockHelpers.fst`** (`Libcrux_sha3.Simd.Arm64.StoreBlockHelpers.fst`,
   597 lines on the Arm64 side): per-byte bridge lemmas
   (`store_block_window_byte`, `store_block_window_byte_of_vst`, etc.) that
   reify `update_at_range ∘ vst1q_bytes_u64` as a per-absolute-index byte
   equality. The byte-level loop invariant advances 16 bytes per iteration
   through these.
3. **`_full` / `_tail` structural split.** `store_block` is decomposed into
   `store_block_full` (multi-of-16-byte chunks) + `store_block_tail` (residual
   partial chunk). The `store_block` top-level becomes a one-line sequential
   composition. This splits the function-level WP into two manageable parts.
4. **`--using_facts_from '* -Rust_primitives.Slice.array_from_fn -Core_models.Num.impl_u64__rem_euclid -Core_models.Num.impl_u32__rem_euclid'`** on
   `store_block_full` / `_tail` (`Arm64.Store.fst:794, 1009`). Filters the
   same upstream cascade sources as the 2026-05-05 AVX2 `load_block` fix.

The AVX2 current state (`Libcrux_sha3.Simd.Avx2.Store.fst:165`) is: a single
700-line monolithic `store_block` with `let _:Prims.unit = admit () in`
gating the entire body. No wrapper, no helpers module, no structural split,
no `--using_facts_from` filter, `--z3rlimit 300` only.

**Why N=4 hurts where N=2 didn't (from profile data):** the goal forall has
~124 outer-bound vars at N=4 vs ~62 at N=2. Each Z3 case-split on bool-encoded
atoms (`HasType x bool`, `BoxBool ...`) re-skolemizes the int_t pretyping +
range-refinement chain. The cascade's positive-feedback factor scales
**superlinearly** with the number of universally bound vars in `k!61`'s body
(more patterns matched per Skolemization).

### Root cause hypothesis

The dominant `k!61` (1.19M instances) on the 124-var goal forall is the
direct manifestation of the loop_invariant being **inline** in `store_block_full`'s
ensures — all four per-buffer byte-level foralls (each citing
`update_at_range`, `get_lane_u64`, `to_le_bytes`, `(j-start)/8` arithmetic)
appear in one giant Z3 term, which the solver tries to e-match against
everything in scope.

### Closure plan

Recommended sequence (mirrors Arm64 template, with AVX2-specific scaffolding
already drafted in parallel branches):

1. **Inherit `~/libcrux-sha3-store-avx2-discharge`'s structural split.**
   That branch (HEAD `464a9914a`, 3 commits ahead of `sha3-proofs-focused`)
   has the AVX2 `store_block` already split into `store_block_full` /
   `store_block_tail` / composer, with 3 per-iter wrappers (`store_u64x4x4`,
   `store_chunk8`, `store_chunk_ragged`) and a 597-line
   `Libcrux_sha3.Simd.Avx2.StoreBlockHelpers.fst` axiom-bridge module.
   The structural skeleton is done; the bodies carry
   `--admit_smt_queries true`.
2. **Inherit `~/libcrux-loop-inv-opacify`'s `LoopInv.fst`** — a 170-line
   opaque per-lane byte invariant `byte_inv_full` + `_init` / `_step` /
   `_after_loop` lemmas (AlgoStar Technique 4, opaque bundles with explicit
   instantiation lemmas). This is **the predicate stack designed to break
   the `k!61` cascade** by compressing the 124-var goal forall to ~30 vars.
3. **Wire LoopInv into store_block_full's loop_invariant.** Replace each of
   the 4 per-buffer inline byte-level foralls with `byte_inv_full lane out_k …`.
   The wrapper post (`store_u64x4x4`) bridges to the step lemma; the
   `_after_loop` reveal re-exposes the function ensures shape.
4. **Apply the `--using_facts_from` filter** matching Arm64 store_block_full's.
5. **Re-profile post-fix.** Per skill §1.5.1 ("layered cascades"): the
   *next* cascade source likely surfaces. Be prepared to tighten one further
   SMTPat or add a second opacity marker.

**Open prerequisite** (gating items 1–3 above): the `loop-invariant-opacify`
branch hit a pre-existing hax-checkout include drift
(`Libcrux_intrinsics.Avx2_extract.fsti` references `lemma_int_t_eq_via_bits`,
which is in hax `952bee0` but not `ad110bf`; both checkouts coexist on the
include path and the older one wins). On the current `sha3-proofs-focused`
HEAD `d7ac1bddb`, this is **NOT a present blocker** — the baseline build is
clean. The blocker was specific to the `loop-invariant-opacify` worktree's
`.fstar-cache` state. So transplanting `LoopInv.fst` + the structural-split
work onto `sha3-proofs-focused` should *not* re-introduce the include drift.
Verify after transplant by confirming `make check/Libcrux_intrinsics.Avx2_extract.fsti`
remains green.

### Estimated effort

1–2 sprints, with prior art carrying ~60% of the work:
- Structural split + wrappers: already drafted in `store-block-avx2-discharge`.
- Opaque predicate stack: already drafted in `loop-invariant-opacify`.
- Remaining work: wiring, post-LoopInv-introduction profiling, and the
  inevitable next-cascade tightening (one or two SMTPat patterns).

### Dependencies / sprint ordering

- **No hard dependency** on other admits, but coordinates with admit 5 (AVX2
  byte_eq) on the lane-projection cascade family — see "Cross-admit synthesis"
  below.
- The 2026-05-05 AVX2 `load_block` closure (`createi` opacity + SMTPat
  narrowing + `--using_facts_from`) is **prior art**: the closing pattern is
  documented; what's missing here is the application to a different consumer
  shape (per-byte `update_at_range` chain rather than per-lane `load_lane_u64`).

---

## Admit 4: `lemma_load_block_eq_xor_block_into_state_arm64` inner `byte_eq` `admit ();` at `EquivImplSpec.Sponge.Arm64.fst:124`

### Failure shape

**Profiled this session.** Removed the `admit ();` and re-ran focused
`make check/EquivImplSpec.Sponge.Arm64.fst` at `--z3rlimit 1600 --log_queries
--query_stats --z3refresh`. Failure observed:

```
Query-stats (… lemma_load_block_eq_xor_block_into_state_arm64, 1)
  failed {reason-unknown=unknown because (incomplete quantifiers)}
  in 5835 milliseconds with fuel 0 and ifuel 1 and rlimit 1600
  (used rlimit 52.128)
```

- **Used rlimit 52/1600 (3.26%)** — confirms the e-matching cliff: not
  budget-bound.
- Failure mode `incomplete quantifiers` matches the `fold_range` /
  `extract_lane (load_block …)` cliff family.

Sub-query 1 corresponds to the inner `byte_eq` lemma body
(`Arm64.fst:113,3–139,43` per the query stats source-range tag) — the part
that calls `SP.lemma_subslice_bytes_eq` under a per-`i` Skolemized goal.

qi.profile (z3-4.13.3 `smt.qi.profile=true`, ~83 s wall) — top-10 quantifiers
by instantiation count:

```
   128229  k!61                            (anonymous goal forall — outer Skolemized i)
    48409  Prims_pretyping_f537159…        (Prims.bool pretype axiom)
    48386  bool_inversion                  (HasTypeFuel u x bool => is-BoxBool x)
    47605  projection_inverse_BoxBool_proj_0
    43390  l_quant_interp_2a41664…         (interpretation of an anonymous lambda)
    33752  constructor_distinct_BoxBool
    27691  refinement_interpretation_Tm_refine_9db1a9099a…
    27556  bool_typing
    18610  Rust_primitives.Integers_pretyping_1eff91…  (int_t pretype)
    17540  typing_Libcrux_intrinsics.Arm64_extract.get_lane_u64
```

**The cascade source is `k!61`**, which is the inner `byte_eq` lemma's
Skolemized `i` (Z3 names it `k!61` after Skolemizing the
`Classical.forall_intro` of the `forall (i: nat{i < 25})`). At 128k instances
in 5 s of solver time, every Z3 case-split re-fires the int/bool/refinement
typing chain and the `get_lane_u64` axiom — the trinity-of-cascade signature
(top instances on `k!N` Skolem + bool/Prims pretype + refinement-interp).

The standard cascade tail (`get_lane_u64` typing at 17.5k, `get_lane_u64x2`
equation at 14.2k) confirms the lane-projection chain is being instantiated
per-`i` × per-lane:

```fstar
(* inside byte_eq, repeated 2× for each of lb_state and state: *)
let _ = I.get_lane_u64 lb_state.[ii] (mk_usize 0) in
let _ = I.get_lane_u64 lb_state.[ii] (mk_usize 1) in
```

These four ghost calls, multiplied by the `KA.arm64_lane` SMTPat
(`lemma_arm64_lane_eq_get_lane_u64` at `Arm64.fst:78`, with trigger
`[SMTPat (KA.arm64_lane v l)]`), produce a fanned-out (state × ii × lane)
context that Z3 must e-match against per Skolemization of `k!61`.

### Asymmetry with sibling backend

**Mirrored pair with admit 5 (Avx2).** Neither SIMD backend proves this
byte_eq lemma — they're admitted identically. The sibling that **does**
prove the same shape is **Portable** (`EquivImplSpec.Sponge.Portable.fst:119`,
`lemma_load_block_eq_xor_block_into_state`). The Portable version verifies
clean at `--z3rlimit 200`:

```fstar
let byte_eq (i: nat{i < 25}) : Lemma (Seq.index lb i == Seq.index spec_state i) =
  let ii = mk_usize i in
  if v ii < v (rate /! mk_usize 8 <: usize)
  then lemma_subslice_bytes_eq blocks start rate ii
in Classical.forall_intro byte_eq;
Rust_primitives.Arrays.eq_intro lb spec_state
```

**The difference**: Portable state is `t_Array u64 25` directly — no lane
projection step. Arm64 / AVX2 state is `t_Array t_e_uint64x2_t 25` / `t_Array
t_Vec256 25`, requiring `KA.arm64_lane lb_state.[ii] l` → `I.get_lane_u64
lb_state.[ii] (mk_usize l)` rewriting per access. The byte_eq body has 4 ghost
calls (Arm64) or 8 ghost calls (Avx2) to fire the lane axiom — and that's
exactly what e-matches at 17k instances against the inner `k!61` Skolem.

### Root cause hypothesis

`KA.arm64_lane` is a non-opaque `let` (`EquivImplSpec.Keccakf.Arm64.fst:44`):

```fstar
let arm64_lane (v: I.t_e_uint64x2_t) (l: nat{l < 2}) : u64 =
  I.get_lane_u64x2 v l
```

The SMTPat-tagged bridge `lemma_arm64_lane_eq_get_lane_u64`
(`Arm64.fst:78–82`) carries:

```fstar
[SMTPat (KA.arm64_lane v l)]
```

— **single trigger, broad shape**: matches every occurrence of `KA.arm64_lane
… …` in the goal context. The byte_eq body's two `assert (lhs.[ii] ==
KA.arm64_lane lb_state.[ii] l)` lines (plus the implicit `arm64_lane` calls
inside `G.extract_lane`'s SMTPat unfolding) fire this lemma per-`i`. Each
fire rewrites to `I.get_lane_u64 v (mk_usize l)`, which then ignites the
`typing_…get_lane_u64` and `equation_…get_lane_u64x2` axioms.

### Closure plan

Three candidate fix shapes (in order of expected payoff and minimum
disruption):

1. **`[@@ "opaque_to_smt"]` on `KA.arm64_lane`** (mirror of the 2026-05-05
   `Hacspec_sha3.createi` opacification, skill §1.5 worked example). Add
   `reveal_opaque (`%KA.arm64_lane) (KA.arm64_lane …)` calls at the bridge
   lemma sites (`lemma_arm64_lane_eq_get_lane_u64` body, plus any in
   `EquivImplSpec.Keccakf.Generic.extract_lane`'s SMTPat trigger).
   Direct attack on the cascade source.
   Estimated effort: 1–2 sessions including downstream-cascade re-profiling.

2. **Per-`i` `match` destructure** instead of `Classical.forall_intro byte_eq`
   (skill §7 "Per-i match for forall over a SIMD chain"). Replace:

   ```fstar
   Classical.forall_intro byte_eq;
   Rust_primitives.Arrays.eq_intro lhs rhs
   ```

   with:

   ```fstar
   introduce forall (i: nat{i < 25}). Seq.index lhs i == Seq.index rhs i
     with (match i with
       | 0  -> byte_eq 0
       | 1  -> byte_eq 1
       …
       | 24 -> byte_eq 24);
   Rust_primitives.Arrays.eq_intro lhs rhs
   ```

   Each branch destructures the Skolemized `i` to a concrete ground term
   before Z3 sees the body, eliminating the `k!61` Skolem entirely. 25
   branches; ~80 ms per branch per the technique's prior validation on
   `Vector.Avx2.Serialize.serialize_1`.
   Estimated effort: 1 session (mechanical, with a small style cost in
   line count).

3. **Tighten `lemma_arm64_lane_eq_get_lane_u64`'s SMTPat to a 2-trigger**
   (skill §1.5.1 SMTPat hygiene). Replace the single trigger with a multi-
   pattern that requires *both* the `arm64_lane` term *and* a use of the
   corresponding `get_lane_u64` term, restricting the fan-out to bridging
   contexts only:

   ```fstar
   [SMTPat (KA.arm64_lane v l); SMTPat (I.get_lane_u64 v (mk_usize l))]
   ```

   Symmetric-specificity rule from §1.5.1: both triggers share `v` and `l`,
   so the cross-product is tight. May not fully resolve the cliff on its
   own (the cascade has multiple unrelated `arm64_lane` calls in scope that
   don't share a matching `get_lane_u64`), but it's a cheap experiment
   that, combined with (1) or (2), may suffice.
   Estimated effort: 0.5 session for the experiment; re-profile required.

Recommendation: try (1) first. The 2026-05-05 closure on `createi` is the
*exact* precedent. If a residual cascade surfaces, layer (2) on top per
§1.5.1 ("a cascade-fix often unmasks the next cascade").

### Dependencies / sprint ordering

- **Closes admit 5 by symmetry** — see admit 5 below. The qi.profile
  cite on the AVX2 mirror (`Sponge.Avx2.fst:125`) already confirms the
  same `incomplete quantifiers` cliff at `73/1600 = 4.6%` rlimit
  (per `sha3-sprint-todo.md §"2026-05-23 STATUS DELTA"`), and the byte_eq
  body shape is byte-identical modulo `2 → 4` lanes. The avx2_lane fix
  ports trivially.
- **Independent** of admits 1, 2 (different family).
- **Independent** of admit 3 at the body level, but **shares root cause
  family**: admit 3's `k!61` (1.19M instances) is the same cascade
  fingerprint, just at the function-level WP of `store_block_full`'s
  loop_invariant rather than at the inner-lemma level. Opacifying
  `KA.arm64_lane` / `KA.avx2_lane` may **also help** admit 3 if the
  loop_invariant's `get_lane_u64` references are within reach of the
  opacity.

---

## Admit 5: `lemma_load_block_eq_xor_block_into_state_avx2` inner `byte_eq` `admit ();` at `EquivImplSpec.Sponge.Avx2.fst:125`

### Failure shape

Not re-profiled this session (budget; admit 4's profile is the direct
sibling and the shape is byte-identical modulo lane count). Cited from
`sha3-sprint-todo.md §"2026-05-23 STATUS DELTA"`: failing query used
**73/1600 = 4.6% rlimit**, `incomplete quantifiers`. Same family as
admit 4 (52/1600 = 3.3%); the slight uptick at N=4 is from the extra
lane ghost calls in byte_eq's body (8 vs 4 — 4 lanes × 2 states).

The Avx2 byte_eq body (`Sponge.Avx2.fst:124–143`) is bit-identical to
Arm64's modulo:

- `KA.arm64_lane` → `KA.avx2_lane`
- `lane < 2` → `lane < 4`
- 4 ghost `I.get_lane_u64 … (mk_usize 0..3)` calls instead of 2
- An extra `assert (Seq.length blocks.[mk_usize l] == Seq.length blocks.[mk_usize 0])`
  in the inner-if branch (4-buffer equal-length precondition discharge).

### Asymmetry with sibling backend

**Mirrored with admit 4.** Same family. The "working backend" framing is
the same as admit 4: Portable proves at `--z3rlimit 200`; both SIMD
backends admit at much higher rlimit due to the lane-projection cascade.

### Root cause hypothesis

`KA.avx2_lane = I.get_lane_u64x4 v l` is the same non-opaque `let`
(`EquivImplSpec.Keccakf.Avx2.fst:58`). The bridge lemma in this file is
`EquivImplSpec.Sponge.Avx2.fst:80-ish` (parallel to Arm64's
`lemma_arm64_lane_eq_get_lane_u64`); same single-trigger SMTPat shape.

### Closure plan

Apply the **same fix as admit 4** with `arm64` → `avx2`, `2 → 4`. All
three candidate fixes (opacity on `KA.avx2_lane`, per-`i` match destructure,
multi-pattern SMTPat narrowing) port mechanically.

Estimated effort: 0.5–1 session **after** admit 4 is closed (the pattern
is established).

### Dependencies / sprint ordering

- **Depends on admit 4's pattern landing.** Do not work this in parallel —
  the asymmetry was the pattern, not the structural work.

---

## Cross-admit synthesis

### Are admits 4 and 5 the same root cause?

**Yes, with high confidence.** Same `incomplete quantifiers` mode, same
rlimit fraction (3.3% / 4.6%), byte-identical body shape modulo lane
count, same downstream cascade (`get_lane_u64{x2,x4}` typing/equation).
Both close on the same fix family (`KA.{arm64,avx2}_lane` opacity or
per-`i` match destructure).

### Are admits 3 and 4/5 the same root cause?

**Closely related, but distinct surfaces.** Both involve the lane-projection
chain (`get_lane_u64` / `get_lane_u64x4`). Both cascade through the same
typing infrastructure (k!N anonymous goal forall + Prims/Integers/Box-Bool
pretyping). But:

- **Admit 3 (store_block)** has the cliff at the **function-level WP** of
  `store_block_full`'s loop_invariant (the 124-var inline `forall (j: usize)`
  per buffer). 1.19M instances of k!61.
- **Admit 4/5 (byte_eq)** have the cliff at an **inner-lemma level**, with
  the Skolemized `i` of `Classical.forall_intro byte_eq`. 128k instances of
  k!61 — about 10× smaller than admit 3.

The opacification target is **different** for each: admit 3 needs
`byte_inv_full` (LoopInv.fst-style per-buffer opaque) to compress the
loop_invariant; admit 4/5 needs `KA.{arm64,avx2}_lane` opacity to compress
the per-`i` byte_eq body. Opacifying `KA.{arm64,avx2}_lane` for admit 4/5
**may incidentally help admit 3** (since the loop_invariant uses
`get_lane_u64` directly, not `arm64_lane`, the effect is partial — but the
SMTPat bridge `lemma_arm64_lane_eq_get_lane_u64` fires regardless and
would be less aggressive if its head were opaque).

### Is admit 3 closable using the parallel-branch scaffolding?

**Yes, with effort, modulo the historic toolchain blocker.** The two
relevant branches are:

- **`store-block-avx2-discharge` (~/libcrux-sha3-store-avx2-discharge HEAD
  `464a9914a`, 3 commits ahead)**: structural split + 3 wrappers +
  `StoreBlockHelpers.fst`. Bodies admitted (`--admit_smt_queries true`);
  `avx2-cliff-profile-progress.md` contains the qi.profile data cited
  above for admit 3.
- **`loop-invariant-opacify` (~/libcrux-loop-inv-opacify HEAD `c62edb033`,
  3 commits ahead)**: `LoopInv.fst` (170 lines) with `byte_inv_full`
  opaque + 3 instantiation lemmas. Ready to wire into the structural
  split.

The blocker noted in `loop-inv-opacify-final.md` (hax-checkout include
drift on `Libcrux_intrinsics.Avx2_extract.fsti`) was **specific to that
branch's `.fstar-cache` state**. The current `sha3-proofs-focused` HEAD
`d7ac1bddb` builds clean. Transplanting `LoopInv.fst` + the
structural-split work onto `sha3-proofs-focused` should not re-introduce
the include drift; verify by confirming
`make check/Libcrux_intrinsics.Avx2_extract.fsti` remains green
post-transplant.

### Are admits 1 and 2 the same root cause?

**Yes — weak Rust-side ensures on `Generic_keccak.Simd{128,256}.squeeze{2,4}`.**
Admit 1 has all building blocks in the equivalence layer
(`lemma_squeeze_one_step_arm64`). Admit 2 needs the building block ported
(`lemma_squeeze_one_step_avx2` does not exist).

### Recommended sprint ordering

Three independent tracks, ordered by impact/effort ratio:

**Track A (high impact, low effort) — admits 1 & 2: squeeze drivers**

1. **Admit 1** (`lemma_squeeze2_arm64`): inline lockstep closure using
   `lemma_squeeze_one_step_arm64`. Reference prior art:
   `sha3-byteform-migration-squeeze2` branch's `squeeze_blocks2` helper.
   Estimated: ~1 session.

2. **Admit 2** (`lemma_squeeze4_avx2`): port `lemma_squeeze_one_step_arm64`
   → `lemma_squeeze_one_step_avx2` in `EquivImplSpec.Sponge.Avx2.Steps.fst`
   first (the gap), then mirror admit 1's closure pattern.
   Estimated: ~1–2 sprints.

**Track B (medium impact, medium effort) — admits 4 & 5: byte_eq cliff**

3. **Admit 4** (`lemma_load_block_eq_xor_block_into_state_arm64` byte_eq):
   try `[@@ "opaque_to_smt"]` on `KA.arm64_lane` + `reveal_opaque` at
   bridge-lemma sites. Re-profile. If a residual cascade surfaces, layer
   the per-`i` match destructure on top.
   Estimated: ~1 session for the opacity attempt + 1 session for any
   layered fix.

4. **Admit 5** (mirror): port admit 4's fix mechanically.
   Estimated: ~0.5 session.

**Track C (lower-priority, high effort) — admit 3: AVX2 store_block**

5. **Admit 3** (`Libcrux_sha3.Simd.Avx2.Store.store_block` body): inherit
   `store-block-avx2-discharge`'s structural split + wrappers, inherit
   `loop-invariant-opacify`'s `LoopInv.fst` predicate stack, wire
   LoopInv into `store_block_full`'s loop_invariant, apply Arm64's
   `--using_facts_from` filter. Re-profile post-fix per skill §1.5.1.
   Estimated: 1–2 sprints (60% of work pre-drafted across branches).

**Why this ordering?**

- Tracks A and B are independent of each other; can run in parallel.
- Track B's `KA.{arm64,avx2}_lane` opacity may incidentally narrow Track
  C's cascade (Track C's profile shows `get_lane_u64` typing as a
  cascade-tail contributor), so doing B first slightly de-risks C.
- Track A has the highest impact per effort (closes 2 admits with no
  cascade-debugging budget). Track C has the largest absolute payoff
  (closes 1 admit but unlocks the AVX2 squeeze pipeline) but the highest
  variance (next-cascade-surfaces risk per §1.5.1).
- Tracks A and B together close 4 of 5 admits in ~3–5 sessions; Track
  C alone closes the remaining 1 in 1–2 sprints.

---

## Process notes

- Per the prompt and skill rules: this session was **investigation-only**.
  No admit was closed. The Arm64 file was temporarily edited (admit
  removed, profile flags added) for diagnostic purposes only; restored
  from `/tmp/Arm64.fst.bak` before commit.
- Generated `.smt2` files were cleaned up.
- Did NOT modify `sha3-sprint-todo.md` — that's the next sprint's job per
  the prompt.
- Did NOT pollute hint cache (no `--record_hints`).
- Diagnostic make ran with `--admit_except` initially (silent — issue
  with how the flag interacts with the dep-build phase), then with full
  check letting the byte_eq fail naturally to capture the cliff.
- qi.profile awk one-liner from the skill needed a field correction
  (`$2` for qid, `$4` for instance count given the F\* qi.profile output
  format `[quantifier_instances] <qid> :  N : … : … : … : …`).

## Files referenced

- `/Users/karthik/libcrux-sha3-proofs/crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.Driver.fst:111` — admit 1
- `/Users/karthik/libcrux-sha3-proofs/crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Avx2.API.fst:87` — admit 2
- `/Users/karthik/libcrux-sha3-proofs/crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.Store.fst:165` (Rust: `crates/algorithms/sha3/src/simd/avx2/store.rs:74`) — admit 3
- `/Users/karthik/libcrux-sha3-proofs/crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Arm64.fst:124` — admit 4
- `/Users/karthik/libcrux-sha3-proofs/crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Avx2.fst:125` — admit 5
- `/Users/karthik/libcrux-sha3-proofs/crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Keccakf.Arm64.fst:44` (`arm64_lane`) — root cause site
- `/Users/karthik/libcrux-sha3-proofs/crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Keccakf.Avx2.fst:58` (`avx2_lane`) — root cause site
- `/Users/karthik/libcrux-sha3-proofs/crates/algorithms/sha3/proofs/fstar/equivalence/EquivImplSpec.Sponge.Portable.fst:77` (`lemma_subslice_bytes_eq`) — working analogue for byte_eq family
- `/Users/karthik/libcrux-sha3-store-avx2-discharge/crates/algorithms/sha3/proofs/agent-status/avx2-cliff-profile-progress.md` — admit 3 qi.profile data (q213, k!61 1.19M)
- `/Users/karthik/libcrux-loop-inv-opacify/crates/algorithms/sha3/proofs/fstar/extraction/Libcrux_sha3.Simd.Avx2.LoopInv.fst` — prior art predicate stack for admit 3
- `/Users/karthik/libcrux-sha3-squeeze2/crates/algorithms/sha3/proofs/agent-status/squeeze2-body-2026-05-05.md` — prior art for admit 1

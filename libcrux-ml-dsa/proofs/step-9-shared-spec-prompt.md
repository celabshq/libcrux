# Step 9 — Shared-spec migration + Portable impl admits + trait-bounds audit

Resuming the ML-DSA proof sprint on branch `ml-dsa-proofs` in
`/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa`. Step 7
(`Operations::reduce` for both Portable and AVX2) and Step 8
(`Operations::add` / `Operations::subtract` for AVX2) closed in
prior sessions. Read in this order before doing anything else:

1. `libcrux-ml-dsa/MLDSA_STATUS.md` — current empirical baseline
   (98 invoked / [CHECK]=42 / [ADMIT]=56 / 0 errors).
2. `libcrux-ml-dsa/proofs/outstanding-admits.md` — focus on the
   `Libcrux_ml_dsa.Simd.Portable + Simd.Avx2 impl-Operations method
   body admits` entry. That's the umbrella ticket Step 9 chips at.
3. `git log --oneline -10` for recent context.

## Workflow: maximize caching (READ FIRST)

F* uses two independent caches; **both must be preserved**:

- `.fstar-cache/checked/*.checked` — per-module verification
  cache. If a module's source mtime is unchanged, F* skips the
  module entirely. This is the big cost saver.
- `.fstar-cache/hints/*.hints` — per-query SMT replay cache. Hints
  make individual queries faster but don't avoid module re-check.

**Hard rule: never `rm -rf .fstar-cache/checked/*` or any
broad `find .fstar-cache -delete`.** A full cache wipe forces a
10-15 min cold re-prove. F*'s caching is reliable; if you suspect
corruption, narrow the scope to one or two specific files. Avoid
deleting hint files entirely — they replay correctly even after
source edits.

### Iteration cycle

1. Edit the file(s).
2. Run `make /Users/karthik/libcrux-ml-dsa-proofs/.fstar-cache/checked/$TARGET.fst.checked`
   from `libcrux-ml-dsa/proofs/fstar/extraction/`. Make walks back
   through deps and re-checks only what's stale (file with newer
   mtime than its `.checked`). Warm: ~1-30 sec; cold (just your
   touched files): typically under 2 minutes.
3. Iterate — only step 2 each time, no cache clearing.
4. **Once per cohesive piece of work, before committing**, run
   `JOBS=2 ./hax.sh prove 2>&1 | tail -22` to verify the global
   baseline (98 invoked / [CHECK]=42 / [ADMIT]=56 / 0 errors).
   Warm cache means this is fast (~2-3 min — the ADMIT-mode
   modules always re-emit their tag, the CHECK-mode modules are
   cache hits).

### Useful targets

- `Libcrux_ml_dsa.Simd.Avx2.fst.checked` — AVX2 trait impl.
- `Libcrux_ml_dsa.Simd.Portable.fst.checked` — Portable trait impl.
- `Libcrux_ml_dsa.Simd.Avx2.Arithmetic.fst.checked` — AVX2 free fns.
- `Libcrux_ml_dsa.Simd.Portable.Arithmetic.fst.checked` — Portable free fns.
- `Spec.Intrinsics.fsti.checked` — intrinsic spec.
- For files OUTSIDE the local extraction dir (e.g.,
  `Hacspec_ml_dsa.Commute.Chunk.fst`): no make target — invoke
  `fstar.exe` directly with the same include paths the Makefile
  builds, OR check it indirectly by running a target that imports
  it.

### When you legitimately need to invalidate

- You edited a file that another module depends on and `make
  $TARGET.checked` claims "nothing to do": run `touch
  /Users/karthik/libcrux-ml-dsa-proofs/path/to/file.fst` to bump
  the mtime, then make again.
- You changed a `*.fsti` interface and want a fresh re-check:
  `rm` only that one specific `.checked` file.

## Architectural commitment (READ FIRST)

We are committing to a **two-tier proof architecture** for SIMD
arithmetic operations. All Step-9 work follows it.

### Tier 1 — shared integer spec

For each operation that has both a Portable and an AVX2
implementation, there is **ONE F* spec stated in integer
operations only**. No `mod_q`, no field-modulus opacity, no
`is_i32b` in the body of the spec — just integer add/sub/mul/shift
plus opaque arithmetic ops from `Spec.Intrinsics`. Both impls'
free-fn posts cite this same shared spec.

Examples (already in
`libcrux-ml-dsa/proofs/fstar/spec/Spec.MLDSA.Math.fst`):

```fstar
[@@ "opaque_to_smt"]
let barrett_red (x:i32) : i32 =
  let q = shift_right_opaque (add_mod_opaque x (shift_left (mk_i32 1) (mk_i32 22))) (mk_i32 23) in
  sub_mod_opaque x (mul_mod_opaque q v_FIELD_MODULUS)

[@@ "opaque_to_smt"]
let mont_mul (x:i32) (y:i32) : i32 = ...   (* integer ops *)

let decompose_spec (gamma2:i32) (r:i32) : (i32 & i32) = ...
let power2round (t:range_t I32) : (range_t I32 & range_t I32) = ...
let compute_one_hint (low high gamma2: range_t I32) : range_t I32 = ...
let use_one_hint (g:gamma2) (r:range_t I32) (hint:range_t I32) : range_t I32 = ...
```

These already exist. **Drop the "obsolete / scheduled for
deletion" banner from `Spec.MLDSA.Math.fst`**: it is the
shared-spec layer (Tier 1), not a relic. The eventual relocation
of the whole module to a hand-written file under
`specs/ml-dsa/proofs/fstar/commute/` (or a sibling) is fine for
cleanliness but is **not required for Step 9**.

### Tier 2 — commute lemmas

In `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`,
prove the hacspec-lift property for each shared spec — i.e., what
the integer-level spec means **after lifting modulo the prime**.
Each lift lemma is proved **once** and used by both the Portable
and AVX2 trait-impl proofs.

Existing lift lemmas:
- `lemma_reduce_lane_commute` (centered Barrett bound +
  raw mod-q congruence → `reduce_lane_post`).
- `lemma_barrett_red_bound_and_mod_q` (Step 7) — landed.
- `lemma_add_lane_commute` / `lemma_sub_lane_commute` (Step 8) —
  bridge `add_mod_opaque`/`sub_mod_opaque` to integer equality.

### Tier 1.5 — trait-post bounds completeness (parallel goal)

**While auditing each method's free-fn ↔ trait alignment, also
verify the trait pre/post in `src/simd/traits.rs` are complete
with respect to bounds.** The motivation: every module above
`traits.rs` (e.g., `Arithmetic.power2round_vector`, `Matrix.*`,
`Ml_dsa_generic.*`, `Sample.*`) calls `Operations::*` methods.
Each such caller has its own panic-freedom obligation that
depends on knowing bounds on the call's `_future` outputs. If
the trait post doesn't expose those bounds, the caller has to
admit panic-freedom even though the bound is provable.

Audit checklist per trait method:

- For every input that's bound-relevant, is the bound visible in
  the **pre** (typically as `is_i32b_array_opaque b ...`)?
- For every output the caller may operate on further, does the
  **post** state a concrete bound on `_future`? Examples:
  - `add` post should expose `is_i32b_array_opaque (b1+b2)`
    (b1, b2 from the pre via `bounded_add_post` SMTPat bridge —
    already wired up; verify each caller benefits).
  - `subtract` post should expose `is_i32b_array_opaque (b1+b2)`
    likewise.
  - `reduce` post: `is_i32b 8380416` per lane (carried via
    `reduce_lane_post` — verify it's revealed/usable downstream).
  - `montgomery_multiply` post: `is_i32b 8380416` per lane (carried
    via `montgomery_multiply_lane_post`).
  - `decompose` post: bounds on `low`/`high` (via
    `decompose_lane_post` — verify completeness).
  - `power2round` post: bounds on `t0_future`/`t1_future` (via
    `power2round_lane_post` — verify completeness).
  - `shift_left_then_reduce` post: `is_i32b 8380416` per lane
    (via `shift_left_then_reduce_lane_post`).
  - `infinity_norm_exceeds` post: when `result == false`, input
    bound tightens to `bound - 1` (verify implication form is
    consumable).
  - `to_coefficient_array`: length post (already there).
  - `compute_hint` / `use_hint`: hint output bounds.
- For each gap, add the missing conjunct to the trait post **and
  re-verify the impls** (the strengthened post may need a stronger
  proof body).

**Why this is a parallel lane.** Once the trait posts are
complete, a separate session can sweep modules-above-traits.rs to
drop their `admit ()` annotations and prove panic-freedom — that
work doesn't touch the impl-side proofs. So even if a Step-9
session only completes the trait-bounds audit (without finishing
all impl admits), it unblocks meaningful parallel work.

### Tier 3 — impl proofs (thin)

Each `Operations` impl method (Portable AND AVX2) becomes a thin
wrapper:

1. Optionally `_orig = simd_units.clone()` under `#[cfg(hax)]` if
   the post is mutating.
2. Call the underlying `arithmetic::*` free fn (which has the
   Tier-1 post).
3. If looped, carry the Tier-1 free-fn-post shape in
   `loop_invariant`.
4. After the loop (or directly), `Classical.forall_intro` over the
   per-lane index, invoking the Tier-2 commute lemma to land the
   trait post.

Step 7's `reduce` and Step 8's `add`/`subtract` are templates;
mirror them.

## Inventory: what's already aligned vs. what isn't

| Method | Tier 1 spec | Portable free-fn post (`src/simd/portable/arithmetic.rs`) | AVX2 free-fn post (`src/simd/avx2/arithmetic.rs`) | Aligned? |
|---|---|---|---|---|
| `reduce` | `barrett_red` | `mod_q` + bounds (line 485-489) | `barrett_red` | **NO** — Portable shortcuts |
| `add` | `add_mod` | `add_post` directly | `add_mod_opaque` (Step 8) | YES |
| `subtract` | `sub_mod` | `sub_post` directly | `sub_mod_opaque` (Step 8) | YES |
| `montgomery_multiply` | `mont_mul` | `mod_q (lhs*rhs*8265825)` (line 250-256) | `mont_mul` (per AVX2) | **NO** — Portable cites mod-q form |
| `shift_left_then_reduce` | `barrett_red(shift_left_opaque ...)` | (audit needed, line 448-456) | `barrett_red(shift_left_opaque ...)` | likely YES |
| `power2round` | `Spec.MLDSA.Math.power2round` | `power2round` (line 340-347) | `power2round` | YES |
| `decompose` | `decompose_spec` (gamma2→pair) | `Spec.MLDSA.Math.decompose (v gamma2) r` (line 754-767) | `decompose_spec` | **NO** — Portable cites the `decompose` (mod-q) variant; AVX2 cites `decompose_spec` (integer). Need to unify on `decompose_spec`. |
| `compute_hint` | `compute_one_hint` + sum | `compute_one_hint` per lane (line 525-535) | (lax — wave 3C) | **YES on Portable**, AVX2 has `lax` |
| `use_hint` | `use_one_hint` | `use_one_hint` per lane (line 803-810) | (admits) | **YES on Portable**, AVX2 admits |
| `infinity_norm_exceeds` | bounds-form | `is_i32b_array_opaque (bound-1)` | `is_i32b (bound-1)` per-lane | YES (no commute needed; trait post is bounds-only) |

So 4 misalignments to fix: `reduce`, `montgomery_multiply`,
`decompose` (Portable side cites the wrong shape), and the AVX2
side of `compute_hint` / `use_hint`.

## Bounds-audit checklist (run alongside each method)

For every method touched in Step 9, before declaring it done:

1. Open `src/simd/traits.rs`. Read the method's `#[requires]`
   and `#[ensures]`.
2. Open the underlying free fn (`src/simd/portable/arithmetic.rs`
   or `src/simd/avx2/arithmetic.rs`). Read its `#[requires]` /
   `#[ensures]`.
3. Identify any bound the free fn proves on `_future` outputs
   that the trait post does NOT currently expose to the caller.
4. Identify any bound the free fn requires on `_future` rhs/lhs
   inputs that the trait pre does NOT currently mandate.
5. For each gap: extend the trait post (or pre) — typically by
   adding a conjunct on `is_i32b_array_opaque` or a per-lane
   bound, packaged opaquely if needed.
6. If the strengthened trait post breaks any consumer (run
   `./hax.sh prove`), DON'T revert — track which consumers
   depend on the new conjunct in `outstanding-admits.md`'s
   resolution-pending list. Strengthening posts is monotone for
   correctness; consumer fallout means the consumer was using a
   different (weaker) form and may need its own pre/post update.
7. If a gap can't be closed because the free fn's body doesn't
   prove the needed bound, that's a separate finding — document
   it in `outstanding-admits.md` as a TODO for a future session.

This audit is part of "every function we touch leaves in final
form" (Hard Rule #2). A method's Step-9 work is incomplete until
the bounds are audited.

## Per-method work — Step 9 walks

### 1. shift_left_then_reduce (~20 min)

Both impls likely already cite `barrett_red(shift_left_opaque ...)`.
Audit Portable's free-fn post at
`src/simd/portable/arithmetic.rs:448-456`. If aligned, the impl
admits in `src/simd/portable.rs:152-155` and
`src/simd/avx2.rs:?` discharge with the same loop_invariant +
`Classical.forall_intro` template using `lemma_barrett_red_bound_and_mod_q`
plus a tiny `lemma_shift_left_then_reduce_lane_commute` (which
just reveals `shift_left_then_reduce_lane_post` opacity) — same
shape as `lemma_reduce_lane_commute`.

If misaligned, strengthen Portable's post to cite
`barrett_red(shift_left_opaque ...)` first.

### 2. montgomery_multiply (~30-40 min)

Misaligned. Steps:

- (Tier 1) Strengthen Portable's `arithmetic::montgomery_multiply`
  post (`src/simd/portable/arithmetic.rs:249-256`) to cite
  `Spec.MLDSA.Math.mont_mul` per lane, in addition to (or instead
  of) the current `mod_q` form. Tightening posts is monotone for
  consumers.
- (Tier 2) Add `lemma_mont_mul_bound_and_mod_q` to
  `Commute.Chunk.fst`: `Lemma (requires is_i32b_array_opaque 8380416 rhs)
  (ensures is_i32b 8380416 (mont_mul x y) /\ (v (mont_mul x y)) % 8380417 ==
  (v x * v y * 8265825) % 8380417)`. Reveal `mont_mul` opacity, push
  rlimit ≤ 200, factor q-bound if needed.
- (Tier 3) Discharge both impls' admits using the standard template.

### 3. power2round (~30 min)

Both likely already aligned on `Spec.MLDSA.Math.power2round`.
Add `lemma_power2round_lane_commute` to `Commute.Chunk.fst`
(reveals `power2round_lane_post` opacity, similar shape to
existing `lemma_reduce_lane_commute`). Discharge both impl
admits.

### 4. decompose (~30 min)

Portable cites `Spec.MLDSA.Math.decompose` (mod-q variant);
AVX2 cites `decompose_spec` (integer variant). Pick `decompose_spec`
as the Tier-1 spec (it's already integer-form, matches AVX2).
Strengthen Portable's `arithmetic::decompose` post
(`src/simd/portable/arithmetic.rs:754-767`) to cite `decompose_spec`.
Add `lemma_decompose_lane_commute` to `Commute.Chunk.fst`.
Discharge both impl admits.

(Optional cleanup: drop `Spec.MLDSA.Math.decompose` if no consumer
remains. Audit before deleting.)

### 5. compute_hint AVX2 (~30 min)

Portable already cites `Spec.MLDSA.Math.compute_one_hint`
per-lane. AVX2 free fn is currently `lax`
(`#[hax_lib::fstar::verification_status(lax)]` at
`src/simd/avx2/arithmetic.rs:286`). Drop the `lax`, write a real
post citing `compute_one_hint` per-lane plus a sum-of-popcounts
ensures. Add `lemma_compute_hint_lane_commute` to
`Commute.Chunk.fst`. Discharge AVX2's impl admit.

The sum-side of the post (`v $result == compute_hint hint_future`)
is non-trivial because `compute_hint` is `repeati`-folded. May
need a per-iteration accumulator-correspondence lemma; budget
extra. If the sum part overruns 20 min, REVERT to bounds-only
post and document — leave the per-lane part landed.

### 6. use_hint AVX2 (~30 min)

Same shape as compute_hint. Portable already cites
`use_one_hint` per-lane; AVX2 admits. Drop the AVX2 admit
(`src/simd/avx2/arithmetic.rs:?`), strengthen its post to cite
`use_one_hint` per-lane, add
`lemma_use_hint_lane_commute` (likely already exists in
`Specs.fst:lemma_use_hint_lane_lookup` — adapt or wrap), discharge
both impl admits.

### 7. infinity_norm_exceeds (~20 min)

Both already aligned. Just discharge the impl admits in
`portable.rs:69-72` and `avx2.rs:125-128` with the standard
template — no new commute lemma needed (the trait post
`infinity_norm_exceeds_post` is opaque-bounds-only; revealing it
plus the free-fn post is enough).

### Skip from this plan

- `ntt` / `invert_ntt_montgomery`: Tier-3 USER lane, pre-budgeted
  admits.
- All encoding methods (`gamma1_*`, `commitment_*`, `error_*`,
  `t0_*`, `t1_*`): different proof structure (BitVecEq), not
  shared-spec work. Phase 2D / 3A iv.
- All `rejection_sample_*`: different proof structure.

## §3 The Step-7 Portable proof (reference template)

From `libcrux-ml-dsa/src/simd/portable.rs:337-380` (committed at
`c91f0b413`, refined by Step 7 AVX2 at `aa51e5ef9`):

```rust
fn reduce(simd_units: &mut [Self; SIMD_UNITS_IN_RING_ELEMENT]) {
    #[cfg(hax)]
    let _orig = simd_units.clone();

    for i in 0..simd_units.len() {
        hax_lib::loop_invariant!(|i: usize| fstar!(r#"
            v $i <= 32 /\
            (forall (j:nat{j < 32}). j < v $i ==>
                <Tier-1 free-fn-post shape, per j>) /\
            (forall (j:nat{j < 32}). j >= v $i ==>
                Seq.index ${simd_units} j == Seq.index ${_orig} j)"#));

        arithmetic::reduce(&mut simd_units[i]);
    }

    hax_lib::fstar!(r#"
        let pf (j: nat{j < 32}) : Lemma
            (ensures Spec.Utils.forall8 (fun (k: nat{k < 8}) ->
                <trait-post predicate per j,k>)) =
            <reveal opacity for is_i32b_array_opaque>;
            let pfk (k: nat{k < 8}) : Lemma
                (ensures <trait-post per j,k>) =
                <Tier-2 commute lemma call>
            in
            Classical.forall_intro pfk
        in
        Classical.forall_intro pf
    "#);
}
```

Step 8 `add`/`subtract` (in `src/simd/avx2.rs:60-117`, agent
commit ?) is the non-looped variant — operations on a single
`simd_unit`, so no outer loop_invariant; just `Classical.forall_intro`
over `k<8`.

## Hard rules (binding)

1. **20-minute wall-clock per method per impl.** On overrun,
   REVERT that method+impl to the admit stub, document obstacle in
   `outstanding-admits.md`, move on.
2. **Every function we touch leaves in final form.** No
   "weak-post + admit-removed" passes.
3. **Tier 1 specs in `Spec.MLDSA.Math.fst`** (or
   `Hacspec_ml_dsa.*` if eventually relocated). Drop the
   "obsolete / scheduled for deletion" banner at the top of
   `Spec.MLDSA.Math.fst` — it's the shared-spec layer.
4. **Tier 2 commute lemmas in
   `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`.**
   Nowhere else.
5. **Never `verification_status(lax)`** in new work. The
   `compute_hint` AVX2 lax is the one place we're allowed to
   delete an existing lax (replacing with real proof).
6. **Impls are thin one-line wrappers** with `#[requires]+#[ensures]`
   identical to the underlying free fn's; the free fn carries the
   Tier-1 post, the impl carries the trait post.
7. **rlimit ≤ 200** for new commute lemmas. ≤ 300 for impl-method
   bodies. If higher needed, factor.

## Pre-flight

```
cd libcrux-ml-dsa
git rev-parse HEAD                    # tip from Step 8
git status                            # should be clean
JOBS=2 ./hax.sh prove 2>&1 | tail -22 # baseline 98/42/56/0 0 errors
```

Verify the baseline before starting any method.

## Sequencing recommendation (greedy easy-first)

1. Drop `Spec.MLDSA.Math.fst`'s obsolete banner (~5 min). Bumps
   nothing in the empirical baseline; signals architectural
   intent.
2. **infinity_norm_exceeds** (both impls, ~20 min total).
3. **shift_left_then_reduce** (both impls, ~20 min).
4. **power2round** (both impls, ~30 min).
5. **decompose** (both impls, ~30 min, includes Portable post
   refactor).
6. **montgomery_multiply** (both impls, ~30-40 min, includes
   Portable post refactor + new Tier-2 lemma).
7. **use_hint AVX2** (~30 min).
8. **compute_hint AVX2** (~30 min, may overrun on the sum part —
   revert to bounds-only post if so).

If steps 1-5 land in one session, that's 8 of the 16 Portable
admits + 5 AVX2 admits cleared. Steps 6-8 are stretch.

## Documentation cadence

After each method lands cleanly:
1. Update `proofs/outstanding-admits.md` to remove (or narrow) the
   resolved entry.
2. Bump per-method P/A column in `MLDSA_STATUS.md`.
3. Commit each method as its own discrete commit so partial
   success on a session-cap still ships the win.

## What success looks like

- End-of-session baseline unchanged: **98 invoked / [CHECK]=42 /
  [ADMIT]=56 / 0 errors**.
- 5+ Portable methods flipped from 🟡 → ✅ in `MLDSA_STATUS.md`.
- 3+ AVX2 methods flipped.
- `Spec.MLDSA.Math.fst` banner reframed: shared-spec layer, not
  obsolete.
- 1+ new commute lemma in `Hacspec_ml_dsa.Commute.Chunk.fst`.
- **Trait-bounds audit complete** for the methods touched: each
  method's trait pre/post correctly propagates bounds, with any
  gaps either fixed or documented in `outstanding-admits.md`
  with a clear "what bound is missing and why" note. This
  unblocks the parallel "drop admits in modules above traits.rs
  and prove panic-freedom" lane.

## What this is NOT

- Not a Hacspec migration. `Spec.MLDSA.Math` stays put. The Tier-1
  shared specs are hand-written F*, not generated from Rust
  hacspec. Phase-4's earlier framing as "delete Spec.MLDSA.Math"
  was wrong; it's recognized now as the Tier-1 layer.
- Not encoding/sampling work. Those use BitVecEq + bit-vector
  identities, different proof structure.
- Not NTT work. Tier-3 USER-lane.

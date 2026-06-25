# Step 10 — Below-trait sweep: post-cherry-pick audit + one-line-wrapper refactor

Resuming the ML-DSA below-trait proof lane on branch `ml-dsa-proofs` in
`/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa`.  Step 9 closed
`infinity_norm_exceeds`, `power2round`, `shift_left_then_reduce` for
both impls plus `montgomery_multiply` Portable.  Trait pre/post was
strengthened by the above-trait agent and cherry-picked into this
branch as `a331580ec`.  Read in this order before doing anything else:

1. `libcrux-ml-dsa/MLDSA_STATUS.md` — current empirical baseline
   (`71 invoked / [CHECK]=13 / [ADMIT]=58 / 0 errors`).
2. `libcrux-ml-dsa/proofs/agent-status/lane-split-protocol.md` — the
   below-trait / above-trait split.  Trait pre/post is owned by the
   above-trait agent; we do **not** edit `traits.rs` or
   `traits/specs.rs`.  Mismatches are flagged in this file under
   "Open findings".
3. `libcrux-ml-dsa/proofs/outstanding-admits.md` — admit ledger.
4. `libcrux-ml-dsa/proofs/agent-status/fstar-perf-top20.md` — slow
   functions to keep an eye on.
5. `git log --oneline -10` for recent context.  Tip is `af7f89f27`
   (lane-split commit).

## Hard rules (binding, inherited from Step 9)

1. **Do not edit `src/simd/traits.rs` or `src/simd/traits/specs.rs`
   pre/post conjuncts.**  Owned by the above-trait lane.  Cherry-pick
   only.
2. **Mismatches go into `lane-split-protocol.md`** under "Open
   findings", numbered F-N.  Do not silently fix on the impl side.
3. **20-min wall-clock per method per impl.**  On overrun, mark
   admitted, document, move on.
4. **Develop locally, upstream specs once** (style guide §9.2):
   new commute lemmas / props go in
   `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
   first; `Specs.fst` is read-only on this branch.
5. **rlimit ≤ 200** for new commute lemmas; ≤ 300 for impl method
   bodies.  If higher needed, factor.

## Two parallel tracks

### Track A — Audit already-discharged impls against the new trait posts

The cherry-pick `a331580ec` strengthened 7 trait posts and 2 trait
pres.  A full prove ran clean (0 errors), but the strengthened
conjuncts piggyback on existing proof bodies and may be fragile (one
extra normalisation hint, one rlimit nudge, and they break).  Audit
each method that already has a non-admit body to make sure the new
conjuncts are robustly discharged:

| Method | Status | New post conjuncts |
|---|---|---|
| `reduce` (Portable) | ✅ Step 7 | `is_i32b_array_opaque FIELD_MAX simd_units_future j` per-j |
| `reduce` (AVX2) | ✅ Step 7 | same |
| `power2round` (Portable) | ✅ Step 9.4 | `is_i32b_array_opaque (pow2 12) t0_future` + `t1_future ∈ [0, pow2 10)` per-lane |
| `power2round` (AVX2) | ✅ Step 9.4 | same |
| `infinity_norm_exceeds` (both) | ✅ Step 9.2 | unchanged by cherry-pick |
| `shift_left_then_reduce` (both) | ✅ Step 9.3 | unchanged by cherry-pick |
| `montgomery_multiply` Portable | ✅ Step 9.6 | unchanged by cherry-pick |

For each method where the cherry-pick added a conjunct (i.e., the
top 2 rows: `reduce`, `power2round`), do:

1. Read the existing proof body in `src/simd/portable.rs` /
   `src/simd/avx2.rs`.
2. Read the matching arithmetic free fn post in
   `src/simd/portable/arithmetic.rs` / `avx2/arithmetic.rs`.
3. Trace whether the new array-level bound conjunct in the trait
   post is discharged by the existing reveal-opaque calls + lane
   post, or whether a fresh `Classical.forall_intro` over `i<8`
   converting per-lane bounds to the `is_i32b_array_opaque` form
   is needed.
4. If not robustly discharged: add the missing `Classical.forall_intro`
   or `is_i32b_array_opaque` reveal explicitly.  Test via
   `make $TARGET.checked` (warm cache should be ~2 min per impl).

Acceptance: every method's post bound conjunct has a clear, named
proof step — no relying on Z3's automatic forall instantiation across
opaque predicate boundaries.

### Track B — One-line-wrapper refactor of impl methods

**Target shape:** every method in `impl Operations for Coefficients`
(`src/simd/portable.rs`) and `impl Operations for AVX2SIMDUnit`
(`src/simd/avx2.rs`) is a literal one-liner — no proof body.  All
proof work lives in a dedicated pre-impl function declared at module
scope.

#### Pattern

For each method with a non-trivial proof body today, introduce a
wrapper function:

```rust
// Above the impl block:
#[hax_lib::requires(fstar!(r#"...TRAIT_PRE..."#))]
#[hax_lib::ensures(|_| fstar!(r#"...TRAIT_POST..."#))]
pub(crate) fn power2round_with_proof(t0: &mut Coefficients, t1: &mut Coefficients) {
    #[cfg(hax)]
    let _orig_t0 = t0.clone();
    arithmetic::power2round(t0, t1);
    hax_lib::fstar!(r#"...lemma_power2round_lane_commute forall_intro..."#);
}

// Inside impl block:
#[requires(fstar!(r#"...TRAIT_PRE..."#))]
#[ensures(|_| fstar!(r#"...TRAIT_POST..."#))]
fn power2round(t0: &mut Coefficients, t1: &mut Coefficients) {
    power2round_with_proof(t0, t1)
}
```

#### Why

1. **Performance.**  Currently `Simd.{Portable,Avx2}.fst impl_1` is
   ranked #3-#4 in `fstar-perf-top20.md` — a single function-level VC
   across ~30 method bodies, ~10 s rlimit-saturated.  Splitting each
   method's proof into its own top-level function gives each method
   its own VC (and its own rlimit) — should drop impl_1's time
   substantially and stop the rlimit saturation.
2. **Independent iteration.**  When working on one method, the impl
   block's overall `--rlimit`/`--fuel` doesn't have to absorb that
   method's needs.  Per-method `#push-options` becomes natural on the
   wrapper function.
3. **Style consistency.**  Trait impls in this codebase already
   follow the pattern for trivial methods (`zero`,
   `from_coefficient_array`, `add`, `subtract` etc.).  The
   non-trivial Step-9 methods are the outliers.

#### Methods to convert

Currently non-one-liner (have inline proof bodies):

| Method | Files |
|---|---|
| `infinity_norm_exceeds` | `portable.rs`, `avx2.rs` |
| `montgomery_multiply` (Portable) | `portable.rs` |
| `shift_left_then_reduce` | `portable.rs`, `avx2.rs` |
| `power2round` | `portable.rs`, `avx2.rs` |
| `reduce` | `portable.rs`, `avx2.rs` |

10 wrapper functions to create across the two impl files.  Each
wrapper pulls the existing proof body verbatim; the impl method
becomes a one-line call.

#### Suggested ordering (greedy easiest-first)

1. `infinity_norm_exceeds` Portable — body has only `let result = ...; reveal_opaque(...)`, simplest extraction.
2. `infinity_norm_exceeds` AVX2 — same shape with extra `assert (forall ...)` for the `to_i32x8` bridge.
3. `shift_left_then_reduce` Portable / AVX2.
4. `power2round` Portable / AVX2.
5. `montgomery_multiply` Portable.
6. `reduce` Portable / AVX2 — biggest body (loop_invariant + after-loop pf).

Each wrapper extraction should be ~10 minutes; verify each one
locally via `make $CACHE_DIR/Libcrux_ml_dsa.Simd.{Portable,Avx2}.fst.checked`
(warm cache, sub-2-min after Track A's work).

#### Acceptance

- `Simd.{Portable,Avx2}.fst impl_1` Z3 time drops to <2 s each (vs.
  10 s today).  Confirm via `extract-fstar-perf.sh` on a fresh prove.
- Every impl method body fits on one line.
- All Step-9 closes still verify.

### Track C (after A and B) — Step 9 stretch goals from prior plan

Pick up the deferred Step 9 admits:
- Step 9.5 `decompose` (both impls).  Portable post refactor needed
  (`Spec.MLDSA.Math.decompose` triple → `decompose_spec` pair).  May
  surface a Tier-1.5 finding (same `[0, q)`-conditional gap as
  use_hint) — flag in `lane-split-protocol.md` if so.
- Step 9.6 (AVX2) `montgomery_multiply` AVX2.  Needs
  `lemma_mont_mul_bound_and_mod_q` in `Commute.Chunk.fst` (~70-line
  ML-KEM-style proof; mirror `lemma_mont_mul_red_i16_int` with
  q=8380417, R⁻¹=8265825, shift=32).
- Step 9.7 `use_hint` (both).  Currently flagged F-1 in
  `lane-split-protocol.md` — wait for above-trait agent to either
  tighten the pre or change the impl-side normalisation contract.
- Step 9.8 `compute_hint` AVX2.  Drop `verification_status(lax)`,
  write a real per-lane post citing `Spec.MLDSA.Math.compute_one_hint`,
  add bridge lemma, prove the body.

## Pre-flight

```bash
cd libcrux-ml-dsa
git rev-parse HEAD                    # tip: af7f89f27
git status                            # should be clean
JOBS=2 ./hax.sh prove 2>&1 | tail -22 # baseline 71/13/58/0
./proofs/agent-status/extract-fstar-perf.sh 20  # current top-20
```

## What success looks like

- End-of-session baseline still `0 errors / 0 make-level failures`.
- Track A: every cherry-picked post conjunct has an explicit named
  proof step (no Z3 luck).
- Track B: 5 method bodies extracted to wrapper functions; impl
  methods are one-liners.  `impl_1` drops out of the perf top-5.
- Track C (stretch): one or more of 9.5 / 9.6 AVX2 / 9.8 closes.
- New perf snapshot in `fstar-perf-top20.md`.

## What this is NOT

- Not a trait pre/post change.  Owned by above-trait lane.
- Not a `Specs.fst` edit.  Cherry-pick only.
- Not a `Spec.MLDSA.Math` rewrite.  Touch only inside per-method
  commute lemmas in `Commute.Chunk.fst`.

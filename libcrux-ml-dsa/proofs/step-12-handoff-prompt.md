# Step 12 — Handoff after Step 11 close

Resuming the ML-DSA below-trait proof lane on branch `ml-dsa-proofs`
in `/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa`.

Step 11 closed in **five commits**:
- `32ae8a9ce` Track 1 — F-1 use_hint commute lemmas (both admits → real
  proofs).  Sub-lemmas: `lemma_spec_decompose_r1_bound`,
  `lemma_mod_pm_eq_mod_p`, `lemma_decompose_bridge`.
- `77a87ce4b` Track 2 — Portable decompose + compute_hint paired-lemma
  scaffolding.  One math admit remains in
  `lemma_compute_hint_lane_commute_conditional` (FIPS 204 §3.6
  MakeHint correctness, ~60-100 lines).
- `709bd9059` Step 11 docs sync.
- `0ddff0d09` Track 4 — AVX2 montgomery_multiply scaffolded via
  `lemma_mont_mul_bound_and_mod_q` (admit body initially).
- `b2ebf58b9` Track 4 close — `lemma_mont_mul_bound_and_mod_q` admit
  replaced with real proof (~80-line ML-KEM-style i32/i64 Montgomery,
  cold ~330s, hint-cached <1s).
- `4f752dd37` AVX2 zero / from_coefficient_array / to_coefficient_array
  body admits replaced with real proofs (new `mm256_setzero_si256_lemma`
  + `mm256_loadu_si256_i32_lemma` SMTPat axioms in `Spec.Intrinsics.fsti`).

**Tip:** `4f752dd37`.  **Empirical baseline:**
**88 modules invoked / [CHECK]=30 / [ADMIT]=58 / 0 errors / 0 make-level
failures**.  AVX2 `impl_1` verifies in 5s @ rlimit 80 (used 8.0/80).

Read in this order before doing anything else:

1. `libcrux-ml-dsa/MLDSA_STATUS.md` — current baseline + per-method
   status table.  Methods now closed on both Portable and AVX2:
   add, subtract, infinity_norm_exceeds, reduce, montgomery_multiply,
   shift_left_then_reduce, power2round, plus all of zero,
   from_coefficient_array, to_coefficient_array.
2. `libcrux-ml-dsa/proofs/outstanding-admits.md` — see "Step 11 Track 2"
   (one math admit) and "Active admits" sections.
3. `libcrux-ml-dsa/proofs/agent-status/lane-split-protocol.md` — F-1
   and F-2 are RESOLVED; trait sha is `36fe89b18`.
4. `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
   — `lemma_compute_hint_lane_commute_conditional` body is the only
   `admit ()` left in this file.
5. `libcrux-ml-dsa/proofs/agent-status/touch-unchanged-checked.sh` —
   USE THIS for the iteration loop (saves ~10 min per iteration).
6. `git log --oneline -8` for recent context.

## Hard rules (binding, inherited)

1. **Do not edit `src/simd/traits.rs` or `src/simd/traits/specs.rs`
   pre/post conjuncts.**  Owned by the above-trait lane.  Cherry-pick
   only.  Trait sha is `36fe89b18`.
2. **Mismatches go into `lane-split-protocol.md`** under "Open
   findings" (numbered F-3, F-4, …).
3. **20-min wall-clock per method per impl.**  On overrun, mark
   admitted, document, move on.
4. **Develop locally, upstream specs once** (style guide §9.2): new
   spec/math lemmas go in
   `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
   first.
5. **rlimit ≤ 200** for new commute lemmas; ≤ 300 for impl method
   bodies.

## Tracks (priority order)

### Track 0 — `commitment_serialize/deserialize` quick wins (start here)

The `commitment` encoding free fns (`src/simd/portable/encoding/commitment.rs`)
already advertise `bit_vec_of_int_t_array` ensures clauses on
`serialize_4` / `serialize_6` (and analogous on the deserialize side
— check before starting), and Z3 verifies them.  But the trait
methods `commitment_serialize` (Portable `src/simd/portable.rs:569`,
AVX2 `src/simd/avx2.rs:545`) and `commitment_deserialize` still have
top-level `hax_lib::fstar!("admit ()")` bodies — pure threading work.

Pattern (mirror Track-4 mont_mul):
1. Pre-call: reveal opacity of any opaque pres/posts (e.g.
   `is_i32b_array_opaque (pow2 4)` for the simd_unit input).
2. Call `encoding::commitment::serialize` / `::deserialize`.
3. Post-call: thread the free fn's bit_vec post into the trait
   post.  The trait post citing
   `Hacspec_ml_dsa.Encoding.simple_bit_pack` is equivalent under
   `bit_vec_of_int_t_array`'s SMTPats — should fold automatically
   with `assert (Seq.equal ...)` if needed.

If `commitment::deserialize` lacks an `ensures`, add one in the
serialize template's mirror shape (`bit_vec_of_int_t_array` between
the bytes input and the simd_unit output).  Z3 should chew it.

**Scope:** 2 methods × 2 impls = 4 admit bodies.  ~30 min each, ~2
hours total.  Low-risk warm-up before the harder math.

**Acceptance:** all four `commitment_serialize`/`commitment_deserialize`
admits replaced.  Commit.

### Track A — `lemma_compute_hint_lane_commute_conditional` math close

The single math admit remaining in `Commute.Chunk.fst`.  Goal: under
`v gamma2 ∈ {95232, 261888}` and `v high ∈ [0, q)`:

```
Spec.MLDSA.Math.compute_one_hint (v low) (v high) (v gamma2) == 1
  <==>
Hacspec_ml_dsa.Arithmetic.make_hint low high gamma2 == true
```

Spec form (direct):
```
if low > gamma2 || low < -gamma2 || (low = -gamma2 && high <> 0)
then 1 else 0
```

Hacspec form (decompose-then-compare):
```
let r1 = high_bits high gamma2  // = 0 since high < q and decompose
                                // of small high gives r1 = 0
let v1 = high_bits (mod_q (high + low)) gamma2
r1 <>. v1
```

This is the **standard FIPS 204 §3.6 MakeHint correctness** theorem.
Estimated **60-100 lines** of math, structurally similar to
`lemma_spec_decompose_r1_bound` plus a case-split on the relationship
between `low`, `gamma2`, and the high-bits boundary.

Sub-lemma to prove first: `high_bits high gamma2 == 0` for `v high ∈
[0, m)` (where m = 4190208/g, the trait pre's bound on `v high`).
This follows from `lemma_decompose_bridge` plus
`lemma_spec_decompose_r1_bound`.  Then case-split:
- |low| > gamma2 ⟹ adding low changes the high-bits bucket.
- low ∈ [-gamma2, gamma2] ∧ low ≠ -gamma2 ⟹ stays in same bucket.
- low = -gamma2 corner case: depends on high.

### Track B — AVX2 `decompose` body close (post-strengthen path)

The AVX2 `decompose` free fn already has a useful per-lane post:
```
forall i. let (r0_s, r1_s) = Spec.MLDSA.Math.decompose_spec gamma2 (to_i32x8 r i) in
  to_i32x8 r0_future i = r0_s /\ to_i32x8 r1_future i = r1_s
```

**`decompose_spec` is the AVX2 IMPLEMENTATION mirror** (using the same
SIMD intrinsic shape), NOT the canonical `Spec.MLDSA.Math.decompose`
(which is int-level).  See `Spec.MLDSA.Math.fst:49-73`.

Sub-lemma needed: `lemma_decompose_spec_eq_decompose`:
```
under v r ∈ [0, q) and v gamma2 valid:
  let (r0_spec, r1_spec) = Spec.MLDSA.Math.decompose_spec gamma2 r in
  let (r0_int, r1_int, _) = Spec.MLDSA.Math.decompose (v gamma2) (v r) in
  v r0_spec == r0_int /\ v r1_spec == r1_int
```

Then mirror Track 4's structural approach: per-lane apply the bridge
+ existing `lemma_decompose_bound` + `lemma_decompose_lane_commute_conditional`.

Estimate: bridge lemma ~50 lines (mostly mechanical i32-level mirror),
impl body ~30 lines (mirror of Track 4 mont_mul template).

### Track C (stretch) — AVX2 `compute_hint` / `use_hint`

Both currently `verification_status(lax)` on the AVX2 free fn.  To
close, EITHER:

(C-i) Drop `lax` and prove the AVX2 free fn body.  Needs Spec.Intrinsics
   models for `mm256_blendv_epi32`, `mm256_sign_epi32`,
   `mm256_movemask_ps`, `mm256_castsi256_ps`, `vec256_blendv_epi32`,
   `mm256_abs_epi32`, `mm256_cmpgt_epi32`, `mm256_cmpeq_epi32`.  These
   are sign/blend/compare intrinsics.  Substantial — likely 200+ lines
   of new SMTPat axioms.

(C-ii) Accept `lax` and add a strengthened post via assume.  Less
   sound, but unblocks the trait impl.

Skip if Tracks A + B fill the budget.

### Track D — Phase 2D encoding methods (only after Track 0/A/B)

The encoding methods break into three buckets by blocker shape:

**Bucket (a) — Easy bridge** (covered by Track 0 above): `commitment_*`
free fns already advertise `bit_vec_of_int_t_array` posts; only the
trait body admits remain.  Pure threading work.

**Bucket (b) — Add ensures + bridge** (this Track D-1): `error_*`,
`t1_*`, `t0_*` (4 + 4 + 4 = 12 trait admit bodies across both impls,
plus the corresponding free-fn posts to add).  The free fns currently
have `requires` but no `ensures`.  Mirror the `commitment::serialize_4`
template: add `bit_vec_of_int_t_array` `ensures` clause citing the
appropriate width (error w=3/4, t1 w=10, t0 w=13).  Z3 should verify
each directly from the body; widths above 6 may need higher rlimit
than commitment uses.  Then thread to trait body using the Track 0
pattern.  Per-fn estimate: ~30 lines (free-fn ensures) + ~30 lines
(trait body bridge).

**Bucket (c) — Library + ensures + bridge** (Track D-2, blocked):
`gamma1_*` packs `GAMMA1 - coefficient` (signed→unsigned offset),
which `bit_vec_of_int_t_array` doesn't model.  Need either
(a) an `offset_bit_vec_of_int_t_array` variant in
`fstar-helpers/fstar-bitvec/BitVecEq.fst`, or (b) per-method ad-hoc
post phrased in terms of the offset.  Per-fn estimate: helper
extension ~40 lines + per-fn 30 + 30 lines (free-fn + trait).
**DO NOT attempt in Step 12** — start by extending BitVecEq, which
is its own session.

**Why commitment is verified but gamma1/t0/t1/error are not:**

`src/simd/portable/encoding/commitment.rs::serialize_4`/`serialize_6`
each carry a proper `#[hax_lib::ensures]` clause using
`bit_vec_of_int_t_array` from
`fstar-helpers/fstar-bitvec/BitVecEq.fst` — equating the input
coefficients' bit-vector to the output bytes' bit-vector.  Z3 verifies
the bit-equality directly from the body (no explicit proof needed).
By contrast, `gamma1.rs::serialize_when_gamma1_is_2_pow_{17,19}`,
`t0.rs::serialize`, `t1.rs::serialize`,
`error.rs::serialize_when_eta_is_{2,4}` all have **only `requires`
clauses, no `ensures`**.  Hence the trait method bodies have nothing
to thread through, and admit().

## Pre-flight

```bash
cd /Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa
git rev-parse HEAD                                # tip: 4f752dd37
git status                                        # should be clean
proofs/agent-status/touch-unchanged-checked.sh snapshot
JOBS=2 ./hax.sh extract
proofs/agent-status/touch-unchanged-checked.sh skip-unchanged
JOBS=2 ./hax.sh prove 2>&1 | tail -22             # baseline 88/30/58/0
```

## What success looks like

Recommended priority (warm-up → math → encoding stretch):

1. Track 0 (commitment quick wins): 4 trait admits replaced.  ✅
2. Track B (AVX2 decompose): impl body closed via
   decompose_spec-eq-decompose bridge.  ✅
3. Track A (compute_hint math): `lemma_compute_hint_lane_commute_conditional`
   body replaced with real proof.  ✅ → zero `admit ()` left in
   `Commute.Chunk.fst`.
4. Track D-1 (stretch): one or more of `error`/`t1`/`t0` free-fn
   ensures added + trait body bridged.

End-of-session baseline still `0 errors / 0 make-level failures`.
Tracks C and D-2 are explicitly NOT for Step 12.

Note on ordering: Track B is structurally easier than Track A
(mechanical mirror of Track 4 mont_mul vs novel FIPS 204 §3.6 math),
so doing Track 0 → B → A keeps risk monotonically increasing.  If A
overruns the 20-min budget, leave the admit and document; B and Track
0 work are independent and ship as separate commits.

## What this is NOT

- Not a trait pre/post change.  Owned by above-trait lane.
- Not a `Specs.fst` edit.  Cherry-pick only.
- Not Track C (AVX2 use_hint/compute_hint) — blocked on
  `verification_status(lax)` and substantial Spec.Intrinsics SIMD
  intrinsic models; future session.
- Not Track D-2 (gamma1 offset-pack) — blocked on BitVecEq library
  extension; future session.

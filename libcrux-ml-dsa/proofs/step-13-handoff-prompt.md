# Step 13 — Handoff after Step 12 close + F-3/F-4 cherry-pick

Resuming the ML-DSA below-trait proof lane on branch `ml-dsa-proofs`
in `/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa`.

## What landed in Step 12

Six commits (`4f752dd37` ← Step 11 close, then this lane's Step 12):

- `87a71ccc4` Track 0c — AVX2 `commitment_serialize` body close.  Free
  fn requires only `out.len()`; trait post is length-pres → drop the
  admit.  ~2s verify.
- `a9388d5a9` Lane-split-protocol findings F-3, F-4, F-5 filed.
- `937adc57b` Track B — AVX2 `decompose` body close via new
  `lemma_decompose_spec_eq_decompose` bridge in `Commute.Chunk.fst`.
  Bridge body is `admit ()` (bit-trick equivalence proof; ~150-200
  lines of interval analysis pending USER-lane).  Impl proof is
  real: case-split on `v r ∈ [0, q)`, chain bridge →
  `lemma_decompose_lane_commute_conditional`; outside [0,q) the lane
  post is vacuously true.
- `686543e33` Option C — `NTT_BASE_BOUND` widened from `FIELD_MID`
  (q/2 = 4190208) to `FIELD_MAX` (q-1 = 8380416), single-constant
  trait change.  Closes the `reduce → ntt` chain spec gap for
  above-trait `compute_w_approx`.  Internal NTT peak rises from 71M
  to 75M — still ~28× under i32::MAX.
- `cdb6e946e` Cherry-pick of above-trait `e2dc257d4` — F-3 and F-4
  resolution.  See "What changed in the trait contract" below.

Empirical baseline at `686543e33` (pre-cherry-pick): 75 modules
invoked, [CHECK]=27, [ADMIT]=48, 0 errors, 0 make-level failures.
The Step 13 entry baseline (post-`cdb6e946e`) is captured in this
session's `verification_result.txt` after the next full prove run —
**confirm before starting work**.

## What changed in the trait contract (cherry-pick `cdb6e946e`)

**F-3 fix** — encoding `*_serialize` trait pres switched from signed
`is_i32b_array_opaque b (f_repr simd_unit)` (allowed `|x| ≤ b`,
including negative) to non-negative `is_pos_array_opaque b (f_repr
simd_unit)` (forces `0 ≤ x ≤ b`).  Affects four trait method pres:
`commitment_serialize`, `gamma1_serialize`, `t0_serialize`,
`error_serialize`.

New predicate (mirror of ML-KEM's `bounded_pos_i16_array`) lives in
`src/simd/traits/specs.rs:79-97`:
```fstar
let is_pos_array (l: nat) (x: t_Array i32 (mk_usize 8)) : prop =
  forall (i: nat). i < 8 ==>
    v (Seq.index x i) >= 0 /\ v (Seq.index x i) <= l

[@@ "opaque_to_smt"]
let is_pos_array_opaque (l: nat) (x: t_Array i32 (mk_usize 8)) : prop =
  is_pos_array l x

let lemma_is_pos_array_lookup ...  (* SMTPat-driven on Seq.index *)
let lemma_is_pos_array_intro ...
```

The previously-misleading comment at specs.rs:64-69 (claiming
`is_i32b_array_opaque` was equivalent to a non-neg bound) is replaced
with accurate F-3 commentary.

Above-trait wrapper updates landed too (in `src/encoding/{commitment,
gamma1,t0,error}.rs`) — these include new `requires` clauses that
forward `is_pos_array_opaque` from caller to trait method, plus a
refactor of `serialize_vector` from `cloop!` to plain `for` with
`hax_lib::loop_invariant!`.

**F-4 fix** — `compute_hint_lane_post` (specs.rs:172-176) right-hand
side switched from `if Hacspec_ml_dsa.Arithmetic.make_hint low high
gamma2 then v hint == 1 else v hint == 0` to direct citation:
```fstar
v hint == Spec.MLDSA.Math.compute_one_hint (v low) (v high) (v gamma2)
```
Reason: the old form was provably false at `low = -gamma2, high != 0`
(Spec returns 1, Hacspec MakeHint via decompose's special-case
returns 0).

Lookup/intro lemmas updated in lockstep (specs.rs:178-198).

Trade-off (documented in the new comment): drops the cross-spec link
to FIPS 204; recoverable later if a Hacspec helper that mirrors the
optimized boundary handling lands.

## Hard rules (binding, inherited)

1. **Do not edit `src/simd/traits.rs` or `src/simd/traits/specs.rs`
   pre/post conjuncts.**  Owned by the above-trait lane.  Cherry-pick
   only.  Trait sha is `cdb6e946e` (this lane) ↔ `e2dc257d4`
   (above-trait).
2. **Mismatches go into `lane-split-protocol.md`** under "Open
   findings".  Resolved findings: F-1, F-2, F-3, F-4, F-5.
3. **20-min wall-clock per method per impl.**  On overrun, mark
   admitted, document, move on.
4. **Develop locally, upstream specs once** (style guide §9.2): new
   spec/math lemmas go in
   `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
   first.
5. **rlimit ≤ 200** for new commute lemmas; ≤ 300 for impl method
   bodies.

## Pre-flight

```bash
cd /Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa
git rev-parse HEAD                                # tip: cdb6e946e
git status                                        # clean
proofs/agent-status/touch-unchanged-checked.sh snapshot
JOBS=2 ./hax.sh extract                           # confirms unchanged
proofs/agent-status/touch-unchanged-checked.sh skip-unchanged
JOBS=2 ./hax.sh prove 2>&1 | tail -10             # confirm 0 errors
```

## Tracks (priority order)

### Track A — `lemma_compute_hint_lane_commute_conditional` close (~10 min)

The single math admit remaining in `Hacspec_ml_dsa.Commute.Chunk.fst`
just became **trivial** under F-4.

Old conclusion (`compute_hint_lane_post`) cited `make_hint`, requiring
the FIPS 204 §3.6 MakeHint correctness theorem (~60-100 lines, with a
genuine boundary inconsistency that made it unprovable as written).

**New conclusion** (after F-4):
```
v hint == Spec.MLDSA.Math.compute_one_hint (v low) (v high) (v gamma2)
```

The lemma's existing requires already gives:
```
v hint_future == Spec.MLDSA.Math.compute_one_hint (v low) (v high) (v gamma2)
```

So the lemma body collapses to:
```fstar
let lemma_compute_hint_lane_commute_conditional
    (gamma2 low high hint_future: i32)
    : Lemma
        (requires
          (v gamma2 == 95232 \/ v gamma2 == 261888) /\
          v hint_future == Spec.MLDSA.Math.compute_one_hint (v low) (v high) (v gamma2))
        (ensures TS.compute_hint_lane_post gamma2 low high hint_future)
  = reveal_opaque (`%TS.compute_hint_lane_post)
                  (TS.compute_hint_lane_post gamma2 low high hint_future)
```

That's it.  `introduce ... with hyp` block + `admit ()` body in the
old version (lines 583-598 of `Commute.Chunk.fst`) goes away.

**Acceptance:** `Hacspec_ml_dsa.Commute.Chunk.fst` has zero `admit
()` calls outside `lemma_decompose_spec_eq_decompose` (Track B
math).  Commit.

### Track 0a — Portable `commitment_serialize` body close (~30 min)

Now unblocked by F-3.  The trait pre is now `is_pos_array_opaque
(pow2 (Seq.length serialized)) (f_repr simd_unit)`, and the Portable
free fn `src/simd/portable/encoding/commitment.rs::serialize`
requires `forall i. bounded values[i] d` — the new `is_pos_array_opaque`
lookup SMTPat fires on `Seq.index x i`, giving exactly `0 ≤ v x[i] ≤
pow2 d`, which discharges `bounded`.

Pattern (mirror Track 0c):
```rust
fn commitment_serialize(simd_unit: &Coefficients, serialized: &mut [u8]) {
    encoding::commitment::serialize(simd_unit, serialized)
}
```

Just delete the `hax_lib::fstar!("admit ()")` line at
`src/simd/portable.rs:570`.  Z3 should fold the trait pre's
non-negative bound into the free fn's requires via the new SMTPat
lookup; the trait post (length-pres only) is the first conjunct of
the free fn's bit_vec ensures.

**Acceptance:** Portable `commitment_serialize` admit removed.
Commit.

### Track D-1 — `t0_serialize`, `error_serialize`, `t1_serialize` (~2 hrs)

Same shape as Track 0a:

- **`t0_serialize`** (Portable + AVX2): trait pre now
  `is_pos_array_opaque (pow2 13) (f_repr simd_unit)`.  Free fn
  `src/simd/portable/encoding/t0.rs::serialize` already has the
  `bounded` requires.  Need to ADD a `bit_vec_of_int_t_array` ensures
  to the Portable free fn (~30 lines, mirror commitment::serialize),
  then thread to trait body.  AVX2 free fn: same as commitment AVX2
  — likely just delete the trait body admit.

- **`error_serialize`** (Portable + AVX2): trait pre now
  `is_pos_array_opaque eta (f_repr simd_unit)` (wait — error uses
  `match eta with Eta_Two -> 2 | Eta_Four -> 4`, check the exact pre).
  `error::serialize_when_eta_is_2` and `serialize_when_eta_is_4` need
  ensures clauses; thread to trait body.

- **`t1_serialize`**: trait pre is already
  `forall i. v simd_unit.f_repr[i] >= 0 /\ < pow2 10` (non-negative
  per-lane form, escaped F-3 in the original — no need for
  `is_pos_array_opaque`).  Free fn `t1::serialize` already has
  `bounded` pre.  Just add ensures + thread.

Per fn: ~30 lines (free-fn ensures) + ~5 lines (trait body delete admit).

**Acceptance:** at least one of t0/error/t1 fully closed (both Portable
and AVX2 trait bodies admit-free).  Commit per method.

### Track A-deferred — `gamma1_serialize` / `gamma1_deserialize`

`gamma1` packs `GAMMA1 - coefficient` (signed→unsigned offset),
which `bit_vec_of_int_t_array` doesn't model directly.  Needs either
(a) an `offset_bit_vec_of_int_t_array` variant in
`fstar-helpers/fstar-bitvec/BitVecEq.fst`, or (b) per-method ad-hoc
post phrased in terms of the offset.  Per-fn estimate: helper
extension ~40 lines + per-fn 30 + 30 lines (free-fn + trait).
**DO NOT attempt in Step 13** — start by extending BitVecEq, which
is its own session.

The F-3 fix already updated `gamma1_serialize`'s trait pre to
`is_pos_array_opaque`, but the impl body still admits because the
free fn doesn't have a `bit_vec` ensures.  Adding an offset-aware
ensures is the pending work.

### Track B math — `lemma_decompose_spec_eq_decompose` close

The single math admit remaining in `Commute.Chunk.fst` after Track A
lands.  Goal: under `is_i32b 8380416 r` and valid gamma2:
```
let (r0_s_avx, r1_s_avx) = Spec.MLDSA.Math.decompose_spec gamma2 r in
let (r0_int, r1_int, _) = Spec.MLDSA.Math.decompose (v gamma2) (v r) in
v r0_s_avx == r0_int /\ v r1_s_avx == r1_int
```

`decompose_spec` (`Spec.MLDSA.Math.fst:49-73`) computes via SIMD bit
tricks: for gamma2=95232, `r1 = (((r' + 127)/128) * 11275 + 2^23) >>
24` masked to [0, 43]; for gamma2=261888, `r1 = (((r' + 127)/128) *
1025 + 2^21) >> 22` masked to [0, 15].  `r' = r mod q` (Euclidean,
handled via `if r < 0 then r + q else r`).

`Spec.MLDSA.Math.decompose` is the canonical formula `r1 = (r_q -
r_g) / (2*gamma2)` with the special case at `r_q - r_g = q-1`.

Estimated proof: ~150-200 lines of interval analysis on the magic
constants 11275 and 1025, with assert_norm for boundary checks.
Mirror of Track 4 mont_mul depth.  Better suited to a focused
USER-lane sprint than a tail of Step 13.

### Track C (stretch) — AVX2 `compute_hint` / `use_hint`

Both currently `verification_status(lax)` on the AVX2 free fn.  After
F-4, AVX2 `compute_hint` impl body's admit could in principle be
replaced via a per-lane bridge similar to Track B's decompose.  But
the AVX2 free fn body uses sign/blend/compare intrinsics
(`mm256_blendv_epi32`, `mm256_sign_epi32`, `mm256_movemask_ps`,
`mm256_castsi256_ps`, `vec256_blendv_epi32`, `mm256_abs_epi32`,
`mm256_cmpgt_epi32`, `mm256_cmpeq_epi32`) that lack
`Spec.Intrinsics` SMTPat models.  Substantial — likely 200+ lines of
new SMTPat axioms.  Skip if Tracks A + 0a + D-1 fill the budget.

## Recommended priority

1. **Track A close** (10 min) — trivial after F-4.
2. **Track 0a close** (30 min) — single delete after F-3.
3. **Track D-1: t1_serialize** (45 min) — easiest of the three (no
   F-3 needed, just bit_vec ensures + bridge).
4. **Track D-1: t0_serialize** (45 min) — F-3 unblocked, mirror
   commitment template.
5. **Track D-1: error_serialize** (45 min) — F-3 unblocked, similar
   shape with `match eta`.

Skip gamma1 (needs BitVecEq extension), Track B math (USER-lane), and
Track C (substantial Spec.Intrinsics work).

## What this is NOT

- Not a trait pre/post change.  Owned by above-trait lane.
- Not a `Specs.fst` edit.  Cherry-pick only.
- Not Track B's bit-trick proof — defer to USER lane.
- Not Track C — defer; needs Spec.Intrinsics SIMD intrinsic models.
- Not the BitVecEq extension for gamma1 offset packing — its own
  session.

## End-of-session expectations

- Track A admit replaced with one-line proof.
- Track 0a admit deleted.
- At least one of t0 / t1 / error _serialize fully closed (Portable +
  AVX2 trait bodies admit-free).
- 0 errors / 0 make-level failures, baseline at or above 27 [CHECK].
- `MLDSA_STATUS.md` and `outstanding-admits.md` synced with progress.

## Files to read first

1. `libcrux-ml-dsa/MLDSA_STATUS.md` — current baseline + per-method
   status table.
2. `libcrux-ml-dsa/proofs/agent-status/lane-split-protocol.md` —
   F-1 through F-5 history; resolution status.
3. `libcrux-ml-dsa/proofs/outstanding-admits.md` — "Active admits"
   section.
4. `specs/ml-dsa/proofs/fstar/commute/Hacspec_ml_dsa.Commute.Chunk.fst`
   — `lemma_compute_hint_lane_commute_conditional` body is the only
   `admit ()` left to close in this file (modulo Track B math).
5. `libcrux-ml-dsa/src/simd/traits/specs.rs:79-97` — new
   `is_pos_array_opaque` predicate + lookup/intro lemmas (use these
   in Track 0a / D-1 if Z3 needs help discharging the free fn `bounded`
   pre).
6. `libcrux-ml-dsa/src/simd/portable/encoding/commitment.rs` — Track
   0c template (free fn already has `bit_vec_of_int_t_array` ensures);
   mirror for t0 / error / t1.
7. `libcrux-ml-dsa/proofs/agent-status/touch-unchanged-checked.sh` —
   USE THIS for the iteration loop (saves ~10 min per iteration).
8. `git log --oneline -8` for recent context.

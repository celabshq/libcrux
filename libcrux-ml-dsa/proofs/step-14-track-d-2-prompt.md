# Step 14 Track D-2 — Deserialize trait body closures

You are continuing the ML-DSA below-trait proof effort on branch
`ml-dsa-proofs` in worktree
`/Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa`.

**Prerequisite**: Track 0 (c6c68bbca propagation gap close) MUST have
landed.  Run `JOBS=2 ./hax.sh prove` first; if it shows any F*
errors, switch to `proofs/step-14-track-0-prompt.md` first.

## Goal

Close the four `*_deserialize` trait method body admits — both
Portable and AVX2 — by adding ensures clauses to the underlying free
functions and threading them through to the trait bodies.  Each
deserialize is symmetric to its serialize counterpart, which has
already closed in Step 13.

## Current state of admits

In `src/simd/portable.rs` and `src/simd/avx2.rs`, all four trait
method bodies start with `hax_lib::fstar!("admit ()")`:

| Method | Trait post (after F-10 cherry-pick) |
|---|---|
| `t1_deserialize` | `forall8 (fun i -> 0 <= v out_future[i] /\ v out_future[i] < pow2 10)` |
| `error_deserialize` | per-eta forall8 — `eta=2 ==> -2 <= v <= 2`, `eta=4 ==> -4 <= v <= 4` |
| `t0_deserialize` | `is_i32b_strict_lower_array_opaque (pow2 12) (f_repr out_future)` |
| `gamma1_deserialize` | `is_i32b_array_opaque (pow2 d) (f_repr out_future)` (d ∈ {17, 19}) |

## Pre-flight

```bash
cd /Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa
git rev-parse HEAD                                # tip should be Track 0's commit
git status                                        # clean
git stash list                                    # may have stash@{0} = WIP t1_deserialize
proofs/agent-status/touch-unchanged-checked.sh snapshot
JOBS=2 ./hax.sh extract                           # confirms unchanged
proofs/agent-status/touch-unchanged-checked.sh skip-unchanged
JOBS=2 ./hax.sh prove 2>&1 > /tmp/baseline.log    # confirm 0 errors
grep -E "F\* errors|Make-level" /tmp/baseline.log
```

If the prove fails at this step, you are NOT on a clean Track 0
foundation — switch to the Track 0 prompt.

## Hard rules (binding, inherited)

1. **Do not edit `src/simd/traits.rs` or `src/simd/traits/specs.rs`
   pre/post conjuncts.**  Trait posts are owned by the above-trait
   lane.  If a deserialize trait post needs adjustment (e.g. to use
   the strict-lower predicate for round-trip symmetry with the
   half-open serialize pre), file as F-N in
   `lane-split-protocol.md` first.
2. **20-min wall-clock per method per impl.**  On overrun, mark
   admitted, document, move on.
3. **Develop locally** (style guide §9.2): any new spec lemmas go
   in `Hacspec_ml_dsa.Commute.Chunk.fst` first.
4. **rlimit ≤ 200** for new commute lemmas; ≤ 300 for impl method
   bodies.
5. Use `proofs/agent-status/touch-unchanged-checked.sh` for the
   iteration loop.
6. **Commit per method** (i.e. one commit each for t1, error, t0,
   gamma1 deserialize closures) — keeps cherry-pick-able and easy
   to revert if one method breaks.

## Recommended priority order

1. **t1_deserialize** (~1 hr) — simplest, smallest scope.  WIP may
   exist in `git stash@{0}` from a prior session — pop if useful.
2. **error_deserialize** (~45 min) — close in shape to error_serialize
   we just closed.
3. **t0_deserialize** (~1 hr) — needs strict-lower form ensures
   matching the new trait post.
4. **gamma1_deserialize** (~1 hr) — most complex, has offset packing.

You may finish any subset within budget; skip what overruns.

## Per-method spec sketches

### t1_deserialize (~1 hr)

**Trait post**: `forall i < 8. 0 <= v out[i] /\ v out[i] < pow2 10`.

**Portable** (`src/simd/portable/encoding/t1.rs::deserialize`):
- Body: `cloop! { for (i, bytes) in serialized.chunks_exact(5).enumerate() { ... values[4*i + j] = (some_combo) & mask; ... } }` where `mask = (1 << 10) - 1 = 1023`.
- The `& 1023` ensures `[0, 1024)`.
- Add `#[hax_lib::ensures(...)]` claiming the per-element bound.
- Strengthen `loop_invariant!` to track `forall j < 4*i. v values[j] in [0, pow2 10)`.
- Likely needs an SMT bit-vec hint:
  `BitVectors.bounded_pow2_minus_one_is_bounded` or
  `i32_logand_pow2_minus_one_lemma`.  Search
  `fstar-helpers/fstar-bitvec/` for the right one.

**AVX2** (`src/simd/avx2/encoding/t1.rs::deserialize`):
- Existing ensures has `forall j. j < 256 ==> j%32 > 10 ==> Bit_Zero`
  (note the strict `>`).  This gives `[0, pow2 11)`, which is too
  weak for the trait post (`< pow2 10`).
- Either: (a) tighten the existing ensures to `j%32 >= 10` (one
  character change), and the body's `& 1023` should prove it
  cleanly via `mm256_and_si256` SMTPat lemma; OR (b) add a separate
  `forall i < 8. 0 <= v (to_i32x8 out_future i) < pow2 10` ensures
  alongside the bit_vec one.
- (a) is cleaner.
- In the trait body, drop the admit; the trait post should
  discharge from the strengthened free fn ensures via the f_repr ↔
  to_i32x8 SMTPat bridge.

### error_deserialize (~45 min)

**Trait post**:
```
Spec.Utils.forall8 (fun (i: nat{i < 8}) ->
  ($eta == Eta_Two ==> v out[i] >= -2 /\ v out[i] <= 2) /\
  ($eta == Eta_Four ==> v out[i] >= -4 /\ v out[i] <= 4))
```

**Portable** (`src/simd/portable/encoding/error.rs::deserialize_when_eta_is_2/4`):
- Body for eta=2: `simd_unit.values[i] = ETA - ((byte & 7) as i32)` where `byte & 7 ∈ [0, 8)`, so `ETA - x ∈ (-6, 2] ⊂ [-6, 2]`.  Trait post wants `[-2, 2]` — body's lower bound is wrong.  Wait — `byte >> n & 7` uses 3 bits of byte.  For eta=2 case, masked-3-bit value is in `[0, 8)`.  Then `ETA - x ∈ (2-8, 2] = (-6, 2]`.  That's not `[-2, 2]` ✗.

  **Open question**: is the per-byte 3-bit decode actually producing
  values in `[0, 5)` (so `ETA - x ∈ [-2, 2]`)?  If FIPS 204 only
  generates valid encodings (i.e. `byte & 7 ∈ [0, 5)`), the function
  is partial.  Look at how upstream callers validate bytes; this
  may need a free fn pre `forall i. byte[i] / something < 5` to
  rule out invalid encodings.  Hacspec
  `Hacspec_ml_dsa.Encoding.bit_unpack` may have a similar pre.

  Filing this as an F-13 (open finding) is appropriate if you can't
  resolve in 20 min.

- Body for eta=4: `simd_unit.values[i] = ETA - ((byte & 0xF) as i32)`
  where `byte & 0xF ∈ [0, 16)`.  `ETA - x ∈ [-12, 4]` if x ∈ [0, 16].
  Same issue: trait post wants `[-4, 4]`, body's lower bound is -12.
  Same upstream-caller pre needed.

**AVX2** (`src/simd/avx2/encoding/error.rs::deserialize_when_eta_is_2/4`):
- Bodies are panic_free + admit (same situation — pre-existing
  active admit per `outstanding-admits.md`).
- The free fn ensures advertises a bit_vec-shape post which doesn't
  give the per-element value bound directly.  Bridge via
  `i32_bit_zero_lemma_to_lt_pow2_n_weak n` or similar.

If either Portable or AVX2 hits the partiality issue, skip this
method, file F-13, and move on to t0_deserialize.

### t0_deserialize (~1 hr)

**Trait post**: `Spec.Utils.is_i32b_strict_lower_array_opaque (pow2 12) (f_repr out_future)`
(half-open `(-pow2 12, pow2 12]`).

**Portable** (`src/simd/portable/encoding/t0.rs::deserialize`):
- Body: `simd_unit.values[i] = GAMMA1 - coefficient` where
  `coefficient = masked_combination & GAMMA1_TIMES_2_BITMASK` and
  `GAMMA1_TIMES_2_BITMASK = (GAMMA1 << 1) - 1 = pow2 13 - 1`.
- So `coefficient ∈ [0, pow2 13)`, hence
  `GAMMA1 - coefficient ∈ (GAMMA1 - pow2 13, GAMMA1]`.
- With `GAMMA1 = pow2 12 = 4096`, that's `(-4096, 4096]` — exactly
  the half-open form!
- Add ensures: `forall i. -pow2 12 < v values[i] /\ v values[i] <= pow2 12`.
- Strengthen `loop_invariant!`.
- Add an `is_i32b_strict_lower_array_opaque` intro lemma call at the
  end of the trait body to package into the opaque form.

**AVX2** (`src/simd/avx2/encoding/t0.rs::deserialize`):
- Existing ensures has the `deserialize_post` with bit_vec form.
- Need to derive the `is_i32b_strict_lower_array_opaque (pow2 12)
  (f_repr out_future)` from it.  The AVX2 body computes
  `*out = mm256_sub_epi32(mm256_set1_epi32(GAMMA1), *out)` after
  `deserialize_unsigned`.  Per-lane this is `GAMMA1 - coefficient`
  with the right bound.
- Likely needs a manual lemma:
  `forall i. v (to_i32x8 out_future i) ∈ (-pow2 12, pow2 12]`.

### gamma1_deserialize (~1 hr)

**Trait post**: `Spec.Utils.is_i32b_array_opaque (pow2 d) (f_repr out_future)`
where d ∈ {17, 19} = `gamma1_exponent`.

**Portable** (`src/simd/portable/encoding/gamma1.rs::deserialize_when_gamma1_is_2_pow_17/19`):
- Body: `simd_unit.values[i] = GAMMA1 - coefficient` where
  `coefficient & ((GAMMA1 << 1) - 1) ∈ [0, pow2 (d+1))`.
- Result: `GAMMA1 - coefficient ∈ (-GAMMA1, GAMMA1]` — same shape as t0.
- Note: trait post is the closed form `is_i32b_array_opaque`, NOT
  the strict-lower form.  This is an F-N follow-up (round-trip
  symmetry would want strict-lower) but for now mirror the closed
  form.
- Closed form `forall i. -pow2 d <= v values[i] /\ v values[i] <= pow2 d`
  is implied by `(-pow2 d, pow2 d]` — provable.

**AVX2** (`src/simd/avx2/encoding/gamma1.rs::deserialize`):
- Existing ensures uses `deserialize_post` with bit_vec form and
  `out_reverted` derivation.
- Need to derive the closed value bound.  Significantly more work —
  may need a fresh lemma in `Hacspec_ml_dsa.Commute.Chunk.fst`
  bridging the bit_vec form to the value bound.
- If the AVX2 closure overruns the 20-min budget, leave it as admit
  and document as a Track A-deferred follow-up.

## Iteration loop

For each method, the loop is:

```bash
# 1. Edit free fn ensures + trait body in src/simd/{portable,avx2}.rs
# 2. Re-extract + prove
proofs/agent-status/touch-unchanged-checked.sh snapshot
JOBS=2 ./hax.sh extract
proofs/agent-status/touch-unchanged-checked.sh skip-unchanged
JOBS=2 ./hax.sh prove 2>&1 > /tmp/d2-iter.log &
PID=$!
# Monitor; on done check errors:
grep -E "^\* Error " /tmp/d2-iter.log
# If pass, commit; else iterate.
```

Each method commit:

```
Step 14 Track D-2: <method>_deserialize close

Adds ensures clause(s) to <method>::deserialize{,_when_*} in
src/simd/portable/encoding/<file>.rs (and AVX2 free fn if needed).
Drops the trait body admit in src/simd/{portable,avx2}.rs.

<rationale: bound shape / source of bound / how it discharges>

Empirical: 75 modules invoked, [CHECK]=27, [ADMIT]=48 minus N where
N is the count of admits dropped this method, 0 F* errors,
0 make-level failures.
```

## End-of-session expectations

- t1_deserialize closed (Portable + AVX2): 2 admits removed.
- error_deserialize closed (or F-13 filed if partiality issue):
  ideally 2 admits removed.
- t0_deserialize: at least Portable closed (1 admit removed).  AVX2
  may need the SMTPat bridge — leave admit if blocked.
- gamma1_deserialize: at least Portable closed.  AVX2 likely admits
  pending BitVecEq extension.

Best case: 6 admits removed.  Realistic case: 4 admits removed
(Portable across all 4 methods).  Sync `MLDSA_STATUS.md` and
`outstanding-admits.md` at the end.

## Files to read first

1. `proofs/step-14-track-0-prompt.md` — confirm Track 0 closed first.
2. `proofs/agent-status/lane-split-protocol.md` — open findings,
   especially F-10/F-11 for context on the trait posts.
3. `proofs/outstanding-admits.md` — "Active admits" section,
   particularly the AVX2 encoding body admits and the trait body
   admits remaining.
4. `src/simd/portable/encoding/t1.rs` and `src/simd/avx2/encoding/t1.rs`
   — start here.
5. `src/simd/traits/specs.rs` — reference for the new
   `is_i32b_strict_lower_array_opaque` predicate and lookup/intro
   lemmas.
6. `src/simd/portable.rs` and `src/simd/avx2.rs` — trait method
   bodies.
7. The Step 13 commit
   `Step 13: Track A close + Track D-1 t1/error close + F-3 mirror`
   (sha `027fc57b5`) for the t1_serialize / error_serialize template
   (the symmetric close pattern).
8. `proofs/agent-status/touch-unchanged-checked.sh` — iteration
   helper.

## Style notes

- Output user-facing text terse.  Avoid trailing summaries; the
  diff and the prove-output speak for themselves.
- Don't add narrative comments to the code unless flagging an F-N
  finding.
- Match the commit-message style of the Step 13 closes (sha
  `027fc57b5`, `3cf230cbe`, `a30402f11`).

## What this is NOT

- Not a trait pre/post change.  Owned by above-trait lane.
- Not Track 0 (already done).
- Not Track B's bit-trick proof — defer to USER lane.
- Not the BitVecEq extension for gamma1 offset packing — its own
  session.  Where gamma1_deserialize hits BitVecEq complexity,
  leave the AVX2 admit and proceed.

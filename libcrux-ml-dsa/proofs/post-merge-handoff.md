# Post-merge handoff — single-lane ML-DSA proofs

**Branch**: `ml-dsa-proofs` (the `ml-dsa-above-trait` lane has been
merged in; tip after merge: `dd0fb629c`; tip after rlimit bumps:
`43c8d40dd`; tip after cherry-pick of PR 1348 closure: `9c83b0279`).

**Worktree**: `/Users/karthik/libcrux-ml-dsa-proofs/`.  The
`~/libcrux-ml-dsa-above-trait/` worktree should now be considered
**stale** — all future work happens here.  Sub-agent branches
(`agent-aa244240-above-trait`, `agent-ab0453d4-above-trait`,
`ml-dsa-above-trait-agent-a370d3b3`) hold only duplicates / older
states of work that's already merged here.

**Empirical baseline** (commit `43c8d40dd`, full re-prove):
- 99 modules invoked, [CHECK]=76, [ADMIT]=23, 0 F\* errors,
  0 make-level failures.
- All 23 admits are pre-budgeted (see "Pre-budgeted admits" below).

**Latest commit** (`9c83b0279`, incremental re-prove only — most
.checked files cached):
- 77 modules invoked, [CHECK]=58, [ADMIT]=19, 0 F\* errors.
- The cherry-pick closes the `Encoding.Signature.deserialize` body
  admit (PR 1348 in-place verification — replaces top-of-body
  `admit ()` with real `#[hax_lib::requires]` + per-loop invariants
  + `validate_hint_rows` / `write_hint_rows` helpers).

The lane-split protocol is no longer active.  Any agent in this
worktree may now edit any source file.  Trait pre/post changes
should be made directly here without a cherry-pick handshake.

## Quick start (any session)

```bash
cd /Users/karthik/libcrux-ml-dsa-proofs/libcrux-ml-dsa
git rev-parse HEAD                                # 9c83b0279 or descendant
proofs/agent-status/touch-unchanged-checked.sh snapshot
JOBS=2 ./hax.sh extract
proofs/agent-status/touch-unchanged-checked.sh skip-unchanged
JOBS=2 ./hax.sh prove 2>&1 > /tmp/baseline.log
grep -E "Modules invoked|F\* errors|^\* Error" /tmp/baseline.log
# Expect on a fresh-cache run: ~99 invoked, [CHECK]≈76, [ADMIT]≈23,
#   0 F* errors.
# Expect on an incremental run: fewer invoked (cached .checked files
#   skipped), 0 F* errors.
```

If the baseline isn't clean, **stop and diagnose** before editing.

## What's verified (76 CHECK modules)

Trait contract:
- `Libcrux_ml_dsa.Simd.Traits.{fsti,Specs.fst,fst}` — the
  `Operations` trait + spec helpers (`is_i32b_array_opaque`,
  `is_i32b_strict_lower_array_opaque`, `is_pos_array_opaque`,
  per-method `*_lane_post`, etc.).

Below-trait impls (was below-trait lane):
- `Simd.Portable.{fst, Vector_type, Sample, Arithmetic, Ntt, Invntt,
  Encoding.{Commitment,Error,Gamma1,T0,T1}}`.
- `Simd.Avx2.{fst, Vector_type, Arithmetic, Ntt, Invntt,
  Encoding.{Commitment,Error,Gamma1,T0,T1}}`.

Above-trait modules (was above-trait lane):
- `Constants`, `Types`.
- `Arithmetic` (with bumped rlimit on `make_hint`).
- `Polynomial`, `Polynomial.Spec`.
- `Ntt` (top-level, distinct from `Simd.*.Ntt`).
- `Encoding.{T0, T1, Commitment, Error, Gamma1, Verification_key,
  Signing_key, Signature}` — all closed.
- `Matrix` (with bumped rlimit on `add_vectors`).
- `Sample`, `Pre_hash`.
- `Hash_functions.{Shake128, Shake256, Portable, Simd256, Neon}`.
- `Ml_dsa_generic`, `Ml_dsa_generic.Ml_dsa_{44,65,87}_`,
  `Ml_dsa_generic.Multiplexing.{44,65,87}_`,
  `Ml_dsa_generic.Instantiations.{Portable,Avx2,Neon}.Ml_dsa_{44,65,87}_`.

Hacspec / spec:
- `Hacspec_ml_dsa.Commute.Chunk.fst` — per-lane commute lemmas
  bridging Hacspec semantics to per-lane post predicates.
- `Spec.Intrinsics.fst`, `Spec.MLDSA.Math.fst`, `Spec.MLDSA.Ntt.fst`.

## Pre-budgeted admits (20 modules, intentionally NOT verified yet)

Listed explicitly in `proofs/fstar/extraction/Makefile`'s
`ADMIT_MODULES` allowlist (mirroring `libcrux-ml-kem`'s structure
since 2026-04-30).  Every other `.fst` in the extraction tree is
verified by full SMT — newly-extracted files therefore default to
verified, forcing an explicit Makefile entry to admit.  Goal: drive
this list to zero.

User-facing API wrappers (12):
- `Libcrux_ml_dsa.Ml_dsa_{44,65,87}_.fst` and the
  `.{Avx2,Neon,Portable}` variants.

These are the public entry points (`generate_key_pair`, `sign`,
`verify`).  They thread through the verified core via
`Ml_dsa_generic.*` (which IS verified).  Closing them requires
top-level functional-correctness specs that don't exist yet.
**Stretch goal** for a future sprint.

Sample dispatchers (4):
- `Libcrux_ml_dsa.Samplex4.{Avx2,Neon,Portable}.fst`,
  `Libcrux_ml_dsa.Samplex4.fst` — X4-parallel matrix sampling.
  Requires trait-method panic-freedom on the underlying X4 Xof
  hash functions.

AVX2 rejection sample shuffle table + samplers (3):
- `Simd.Avx2.Rejection_sample.{Less_than_eta,Less_than_field_modulus,Shuffle_table}.fst`.
- The shuffle table is pure data, but its `is_bit_set` helper does
  `1 << bit_position` with `bit_position: u8` — F* needs a
  `bit_position < bits USIZE` refinement that's not in the extracted
  source.  Source-side fix: add a `requires(bit_position < 64)`.
- The samplers are admit-only pending bit-vec body proofs (similar
  shape to the Step 13 Track A AVX2 closures).

Spec dispatcher (1):
- `Libcrux_ml_dsa.Specs.Simd.Portable.Sample.fst` — internal spec
  helper.  `rejection_sample_*_post` references
  `Spec.MLDSA.Math.rejection_sample_field_modulus` whose
  randomness-length precondition is not yet bridged in the
  extracted post.

Recently-promoted (no longer admitted):
- `Libcrux_ml_dsa.Constants.Ml_dsa_{44,65,87}_.fst` — promoted
  2026-04-30 (Session B follow-up).  Pure const definitions; verify
  trivially with no source change.

## Outstanding admits (within VERIFIED modules)

Top-level body admits (function-level `hax_lib::fstar!("admit ()")`)
and `panic_free` annotations remain in CHECKed modules.  The
catalog is `proofs/outstanding-admits.md`.  Highlights:

**Ml_dsa_generic.* — 10+ functions body-admitted** (`sign_internal`,
`verify_internal`, `generate_key_pair_internal`, ...).  These are
the orchestrator — closing them is the largest remaining proof
effort.  Each carries strong wrapper pre/post (the surrounding
contracts hold), but the body's chain of calls is admitted.

**Sample.fst — 9 of 10 functions body-admit**.  Above-trait closed
5 of 10 in commit `09d0743d3`; the remaining are the rejection
sampling state machines + Xof-trait-dependent flows.

**Matrix.fst — 6 wrappers admit-bodied**:
`compute_as1_plus_s2`, `compute_matrix_x_mask`,
`vector_times_ring_element`, `add_vectors`, `subtract_vectors`,
`compute_w_approx`.  Strong pre/post; bodies admit.

**Encoding.* wrappers — admit bodies**:
`Signature.serialize`.
Closures already landed:
- `Encoding.T0.deserialize_to_vector_then_ntt` (`577a112cf`).
- `Encoding.Error.deserialize_to_vector_then_ntt` length-pres (`65940351b`).
- `Encoding.Signing_key.generate_serialized` (`8fa040756`).
- `Encoding.Verification_key.deserialize` (`743956689`).
- `Encoding.Signature.deserialize` (PR 1348, `9c83b0279`).
- `Encoding.Verification_key.generate_serialized` (Session B half).

**Arithmetic.fst** — `power2round_vector` body admit (hax IndexMut
quirk, see `outstanding-admits.md`).

**Hash_functions.{Portable,Simd256,Neon}** — Xof impls have
opaque admit bodies (5 modules verify their trait declarations
without body verification).

## Pre-budgeted "USER lane" admits (non-Z3 SIMD)

These are pre-known to require manual proof refactoring:
- `Simd.Portable.Ntt.{layer_*}` and `Simd.Avx2.Ntt.{layer_1,layer_2}`
  — wide SIMD layer Z3 timeouts.
- `Simd.{Portable,Avx2}.Invntt.{layer_3,layer_4}` — same.
- `Hacspec_ml_dsa.Commute.Chunk.lemma_ntt_full_commute` — Tier-3
  composition admit.

See `proofs/outstanding-admits.md` Pre-budgeted section.

## Recommended next sessions

### Session A: Close Ml_dsa_generic.* function bodies
Largest remaining proof effort.  Each function in
`src/ml_dsa_generic.rs` is body-admitted with strong wrapper
pre/post.  Closing requires threading the per-call posts through
the orchestrator's call chain.  Start with the smallest
(`generate_key_pair_internal`?), then `sign_internal`,
`verify_internal`.  Per-function 1-2hr each.  ~10 functions total.

### Session B: Close encoding wrappers
`src/encoding/{verification_key,signing_key,signature}.rs` body
admits.  These are mostly offset-arithmetic and copy_from_slice
threading.  Above-trait proved this is tractable (commits
`8fa040756`, `743956689`).

- ✅ `Verification_key.generate_serialized` — closed Session B.1.
  Pattern: mirror Signing_key.generate_serialized (loop_invariant
  carrying length + per-lane forall + offset arithmetic asserts).
  One subtlety: `RING_ELEMENT_OF_T1S_SIZE` extracts via the chain
  `BITS_IN_UPPER_PART_OF_T = FIELD_MODULUS_MINUS_ONE_BIT_LENGTH -
  BITS_IN_LOWER_PART_OF_T`; Z3 cannot fold this under `fuel 0`,
  so use `assert_norm (v $RING_ELEMENT_OF_T1S_SIZE == 320)`
  (rather than plain `assert`, which works for the simpler
  `RING_ELEMENT_OF_T0S_SIZE` chain).  rlimit 400, split_queries
  always.

- 🔵 `Signature.serialize` — DEFERRED to its own session (more
  complex than the 30-60 min estimate).  The hint-pack inner loop
  uses an unguarded `true_hints_seen` accumulator that the body
  must show stays `<= max_ones_in_hint`.  Caller (`sign_internal`)
  checks `if ones_in_hint > MAX_ONES_IN_HINT { skip }` before
  calling, but expressing this as a function precondition needs
  one of:
    (a) a recursive F* spec helper `count_total_ones` defined in a
        `hax_lib::fstar!` header block + bound `count_total_ones
        hint <= v $max_ones_in_hint` in the precondition + monotonicity
        across the i-loop;
    (b) take the `actual_ones_in_hint: usize` count as a NEW
        function parameter (caller already has it from `make_hint`
        return value), with precondition tying it to the spec count;
    (c) a defensive in-loop bound check `if hint[i][j] == 1 &&
        true_hints_seen < max_ones_in_hint` that subtly changes
        runtime semantics on out-of-spec inputs.
  PR 1348's `deserialize` closure shows the helper-split pattern
  needed here (split serialize into prefix + hint-write helper,
  the helper carrying the count bound).  Estimate: 2-3 hr.

### Session C: AVX2 Rejection_sample.{Less_than_eta,Less_than_field_modulus}
Promote the shuffle-table + samplers to CHECK.  Shape similar to
Step 13 Track A AVX2 closures.  Per-fn 1-2hr.

### Session D: Sample.fst remaining 5 admits
Above-trait did 5 of 10; finish the other 5.  Many are Xof-trait
flows — may need Hash_functions.Shake128.fst trait method
strengthening too.

### Session E: USER-lane pre-budgeted (NTT/Invntt SIMD layers)
Refactor each AVX2 layer body into 4 per-zeta sub-functions to
split Z3 obligations.  Long-running USER work; not for a 2-hour
session.  See `proofs/manual-proof-targets.md` for ML-KEM analog.

### Session F (one-shot): clean up duplicated lane-split-protocol
entries
The merge resolved conflicts by taking HEAD; some F-N entries
appear twice (open + resolved).  Mostly cosmetic but worth a
~30 min pass to dedupe.  Don't do this and Session A in the same
day — context-switch cost.

## Active F-N findings (open)

From `proofs/agent-status/lane-split-protocol.md`:
- **F-3** — encoding `*_serialize` trait pres signed-vs-non-negative
  mismatch.  Above-trait fix landed (`cdb6e946e`); below-trait
  mirror landed (`027fc57b5`).  Mostly resolved; some downstream
  cleanups remain.
- **F-4** — `compute_hint_lane_post` divergence.  Resolved by
  switching post to cite `compute_one_hint` directly.
- **F-5** — `reduce_lane_post` to FIELD_MID — open finding;
  decision pending (option (a) impl-side correction step vs option
  (b) accept looser `is_i32b 4194303` bound).
- **F-13** — `decompose` strict-lower bound RESOLVED (revert).
- **F-14** — `error_deserialize` trait post FIPS BitUnpack range
  RESOLVED (FIPS-correct asymmetric range).
- **F-15** — `Encoding.Error.deserialize_to_vector_then_ntt`
  length-pres + admit RESOLVED (above-trait).

The **F-N numbering is stable**; new findings should pick up at
F-16.  Both lanes' duplicated F-N text (F-12, F-8/9/10/11 batch
re-entered) was resolved by taking HEAD's text — some duplicate
entries remain in the doc and are flagged as a Session F cleanup.

## Sanity-check checklist for next session

```bash
# 1. Confirm tip (should be 9c83b0279 or a descendant):
git log --oneline -5

# 2. Confirm clean state:
git status

# 3. Snapshot + extract + skip-unchanged + prove:
proofs/agent-status/touch-unchanged-checked.sh snapshot
JOBS=2 ./hax.sh extract
proofs/agent-status/touch-unchanged-checked.sh skip-unchanged
JOBS=2 ./hax.sh prove 2>&1 > /tmp/baseline.log

# 4. Confirm baseline:
grep -E "Modules invoked|F\* errors" /tmp/baseline.log
# Expect on a fresh build (after touching all .checked):
#   Modules invoked:        99  ([CHECK]=76  [ADMIT]=23)
#   F* errors reported:     0
# Expect on an incremental build (most .checked cached):
#   ~50-77 invoked, 0 F* errors.
```

If the baseline drifts (CHECK count drops or errors appear), the
first task of the session is to find what regressed.  Likely
suspects: rlimit-fragile queries (the Step 13/14 work pushed
several modules close to their rlimit budgets — see
`proofs/agent-status/fstar-perf-top20.md`).

## Useful pointers

- `proofs/outstanding-admits.md` — admit catalog with diagnosis +
  mitigation per item.
- `proofs/agent-status/lane-split-protocol.md` — F-N findings ledger
  (still useful as historical record even though the lane split is
  no longer active).
- `proofs/agent-status/fstar-perf-top20.md` — Z3 hot-path tracking;
  refresh after any prove that touches NTT/Invntt to catch
  regressions.
- `proofs/agent-status/touch-unchanged-checked.sh` — incremental
  build helper; use snapshot/skip-unchanged for fast iteration.
- `MLDSA_STATUS.md` — high-level status; pre-merge per-lane
  history retained at the bottom for context.

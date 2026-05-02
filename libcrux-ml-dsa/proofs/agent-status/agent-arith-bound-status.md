# agent-arith-bound (Chain 3) — Class B sprint status

Worktree: `agent-ad87ffc7dac1c1efb`
Branch: `worktree-agent-ad87ffc7dac1c1efb` (parent merges to `ml-dsa-proofs`)
Starting tip: `140702ee2` (agent-mldsa: agent-ntt-bound DONE — Chain 1 merged)

## Plan

Two body proofs, single commit:

- `matrix.rs::compute_as1_plus_s2`: remove body admit; tighten declared
  post from `is_i32b 16760832` (≈ 2·FIELD_MAX) to `is_i32b FIELD_MAX`
  by appending a final Barrett `reduce` after the `add(s2)` step.
  Substitute `add` → `add_bounded` (zero runtime, spec-only) for
  the per-iteration ghost-bound chain.
- `arithmetic.rs::power2round_vector`: remove body admit; single
  inner-loop call to fully-spec'd `power2round_one_ring_element`
  chains directly via snapshot-on-suffix loop_invariant.

Parent-approved decisions (do not re-litigate):
1. Keep the in-function `reduce` insertion at end of second loop
   (matches PQClean `polyveck_caddq` step; benign-tightening
   confirmed empirically).
2. Keep post tightening from 16760832 → FIELD_MAX.
3. Keep `add → add_bounded` substitution.

## Wall events

### 2026-05-01T19:35Z — environment baseline + WIP applied

Worktree was at unrelated `abf60f5c7`; reset to `140702ee2`.
Applied stash@{0} (chain3-wip-pending-audit-2026-05-01) cleanly:
matrix.rs +87, arithmetic.rs +23. Diff matches expected.

Pre-existing F* spec breakage (per Chain 1's notes — survives a fresh
`cargo hax extract`):
- `Hacspec_ml_dsa.Parameters.fst`: drop `{Rust_primitives.Hax.dropped_body}`
  field refinements (hax-lib v0.3.6 doesn't yet emit a stub).
- `Hacspec_ml_dsa.Encoding.fst`, `Hacspec_ml_dsa.Ml_dsa.fst`:
  add `--admit_smt_queries true` push-options around size-arithmetic
  blocks where Z3 fails on usize-overflow.
Local-extraction-only patches (gitignored, not committed).

Sub-task: augment matrix.rs:21-30 comment with PQClean-parity / NTT-bound
follow-up / loop-invariant-constraint context per parent's spec.
ETA: +10 min.

### 2026-05-01T19:45Z — comment augmented + extraction green

matrix.rs comment block expanded with three new design notes:
(a) PQClean parity — flow matches `crypto_sign_keypair`'s
    matrix-vector → reduce → invntt → add s2 → caddq → power2round; the
    final `reduce` is libcrux's caddq equivalent.
(b) Bound-tightening follow-up — invntt's loose `is_i32b FIELD_MAX`
    post can be tightened to `is_i32b (q/2 + small)` via the piecewise
    montgomery_reduce_element pattern; once landed, the final
    `reduce` becomes removable.
(c) Loop-invariant constraint `v columns_in_a <= 7` made explicit.

`./hax.sh extract` succeeded (libcrux-ml-dsa) and
`cargo hax into fstar` succeeded (specs/ml-dsa).  No more dropped_body
refinements present in current extraction — Chain 1's pre-existing
breakage no longer applies.  Will check F* for residual issues
during the per-module verify pass.

Sub-task: F* check on Matrix.fst and Arithmetic.fst.
ETA: +20 min.

### 2026-05-01T20:30Z — F* check incomplete; structural issue surfaced (parent diagnostic)

Agent hit usage cap during F* verification phase (last entry above
indicated F* check was about to run). Parent ran the F* check directly
post-cap to assess state:

- `make check/Libcrux_ml_dsa.Arithmetic.fst` FAILS at
  `power2round_vector` query 67 with "Subtyping check failed" at
  Libcrux_ml_dsa.Arithmetic.fst:308:30-40.
- Line 308 is inside the `Rust_primitives.Hax.Folds.fold_range` loop
  bound argument typing the length argument to `Core_models.Slice.impl__len`.
- Bumped rlimit from 200 to 400 (max under `--split_queries always`):
  query 67 STILL fails with "canceled" after 31s. Reverted rlimit to 200.

Conclusion: NOT an rlimit issue; the loop_invariant on
`power2round_vector` (snapshot-based suffix-frame pattern) is missing a
key assertion that allows F* to discharge the loop bound subtype check.
Needs proof-side work: likely a `reveal_opaque` of `is_i32b_array_opaque`
inside the loop body, or a `Seq.length t0_orig == Seq.length t0`
invariant tightening, or a missing length assertion before the
`power2round_one_ring_element` call.

Matrix.fst not yet checked (would also need rlimit 800 attempt or proof
fix on compute_as1_plus_s2 first).

WIP state in worktree:
- src/matrix.rs and src/arithmetic.rs modified with body proofs +
  comment augmentation (NOT committed).
- src/arithmetic.rs reverted to rlimit 200 (original WIP).
- F* extraction is up-to-date for current source.

Next step (when usage resets): a proof-debug agent picks up here, with
fstar-mcp typecheck_buffer to iterate sub-second on
`power2round_vector`'s loop_invariant.


### 2026-05-02T00:00Z — Chain 3 sprint outcome: SOURCE UNCHANGED, body proofs blocked

After exhausting the per-function debug budget (per memory rule
`feedback_proof_debug_budget`), reverted both functions to their
original ad0632490 state. Chain 3 sprint produces **zero source
changes**.

**Why both body proofs failed to land:**

`power2round_vector`: dual `&mut [T]` slice access pattern
(`&mut t0[i], &mut t1[i]`) hits a hax slice-bounds tactic limitation.
Seven invariant variants tried:
1. Original 5-clause invariant with `t0_orig` snapshot → query 67
   subtype check times out at rlimit 400.
2. Simplified 3-clause invariant → "Tactic failed" at `t1.[i]`.
3. Added explicit `assert (v i < Seq.length t1)` → tactic ignores
   asserts (it's syntactic).
4. Added redundant inequality `v i <= Seq.length t1` to invariant →
   no effect.
5. Introduced `let dim = t0.len()` to mirror `add_vectors` shape →
   no effect.
6. Split borrows into two `let` bindings → hax issue #420 (DirectAndMut).
7. Changed function pre from `${t0.len()} == ${t1.len()}` to
   `Seq.length t0 == Seq.length t1` → no effect.

The hax tactic for slice access at `t1.[i]` does not combine
`Seq.length t0 == Seq.length t1` (loop_invariant) with `v i < v dim`
(loop bound). add_vectors works because its rhs check uses the
function pre `Seq.length rhs >= v dimension` directly without the
length-equality combinator. power2round_vector is the only fn in
keygen with two concurrent &mut slice accesses.

`compute_as1_plus_s2`: same hax slice-bounds tactic limitation in
the inner loop_invariant + a separate field-name resolution issue
at `(Seq.index result (v i)).f_simd_units` (fixed via inline type
ascription, but the broader proof still didn't close). When admitting
the body and keeping the post tightening (16760832 → FIELD_MAX) plus
the runtime reduce (PQClean parity) plus the add_bounded substitution,
side effects in F* extraction caused `add_vectors` (a previously-
verified function) to time out at rlimit 800. Reverting all source
changes restored Matrix.fst to verified state.

**Generate_key_pair panic_free flip: BLOCKED.** Cannot land the flip
this sprint because compute_as1_plus_s2's post stays at the loose
`is_i32b 16760832` (= 2*FIELD_MAX), which doesn't satisfy
power2round_vector's pre `is_i32b FIELD_MAX`. Both function bodies
must close (or both posts must be coherent with the rest of the
chain) before the flip can land.

**Net Chain 3 deliverables:**
- Independent review of the proposed source change (BENIGN-TIGHTENING
  verdict, KAT-safe, PQClean-parity confirmed).
- Empirical bounds on `montgomery_multiply_by_constant(_, 41_978)`
  documented (max output ≈ q/2 + 21k under input bound 256·FIELD_MAX;
  formula `|out| ≤ ⌈|val|/2³²⌉ + q/2` matches ML-KEM's documented
  Montgomery bound).
- This status log with diagnostic for both body proofs.
- TaskCreate user-followup items for both body proofs.

**Recommended user followups:**
1. (power2round_vector) Refactor `power2round_one_ring_element` to
   take t1 by value and return a tuple, eliminating the dual-mutable
   pattern; OR file hax issue describing the slice-bounds tactic gap.
2. (compute_as1_plus_s2) Same as power2round_vector OR widen
   `invert_ntt_montgomery`'s trait post via the piecewise Montgomery
   pattern (then the body's natural post is ~q/2 + ε, post-tightening
   becomes free, the final reduce becomes removable).
3. (panic_free flip) Wait for at least the compute_as1_plus_s2 post
   tightening before attempting the flip on `generate_key_pair`.


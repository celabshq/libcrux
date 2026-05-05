# Next-session prompt — finish `deserialize_5` outer bridge

**Branch tip:** start from the commit that lands sprint 2026-05-12.
**Worktree:** create a fresh one off that branch.

## What's already done (sprint 2026-05-12)

- `BitVec.Intrinsics.fsti::mm256_mullo_epi16_specialized4` spec **corrected**
  (was reversed; now `shift = 11 - ((k%2)*5 + (k/2)*2)`).
- Inner helper `deserialize_5_vec(c: Vec128) -> Vec256` is **fully verified**.
  - `[@@"opaque_to_smt"]`, single `assert_norm (BitVec.Utils.forall256 (…))`.
  - Spec is in terms of `c` bit positions (NOT bytes).
  - 105 queries pass, max rlimit 28/400.
- Outer `deserialize_5` is `verification_status(panic_free)` — body type-checks,
  ensures injected as `admit ()`.

## Sole remaining task: discharge outer ensures

The outer's body is:
```rust
let coefficients = mm_set_epi8(bytes[9] as i8, …, bytes[0] as i8);
deserialize_5_vec(coefficients)
```

Outer ensures (target):
```fstar
forall (i: nat{i < 256}).
  $result i = (if i % 16 >= 5 then 0
               else let j = (i / 16) * 5 + i % 16 in
                     bit_vec_of_int_t_array $bytes 8 j)
```

## What's available at the outer

- Helper's ensures (from val signature, opaque body):
  ```fstar
  result i = (if i%16 >= 5 then 0 else c (c_byte(i) * 8 + j(i)%8))
  ```
  where `c = coefficients`, the post-`mm_set_epi8` Vec128.
  The transform is closed-form arithmetic (see helper ensures in source).
- `mm_set_epi8` spec: `c (8*k + b) = bit_vec_of_int_t_array bytes 8
  (byte_map[k]*8 + b)` for k ∈ [0..15], b ∈ [0..7], where
  `byte_map = [0;1;1;2;2;3;3;4;5;6;6;7;7;8;8;9]` (from the source's
  argument order, `bytes[byte_map[k]]` is what lands in c byte k).

## What didn't work in 2026-05-12 (do not retry)

| Attempt | Result |
|---|---|
| Outer `assert_norm (forall256 (coefficients (…) = bit_vec_of_int_t_array bytes 8 (…)))` | 152s timeout, rlimit 400 |
| Same in 4 × `forall_n 64` quarters | All 4 timeout, ~18 min total |
| Pure arithmetic identity `byte_map_val * 8 + j%8 = (i/16)*5 + i%16` over `forall256` (no Vec128/bytes refs at all) | 105s timeout — the 16-arm if-cascade in `byte_map_val` over 256 conjuncts is itself too heavy |

## Recommended approach: per-byte lemma + Z3 instantiation

Establish 16 small lemmas, one per c-byte index, asserting:
```fstar
let lemma_c_byte_k (k: nat{k < 16}) (b: nat{b < 8})
  : Lemma (coefficients (8*k + b) =
           bit_vec_of_int_t_array bytes 8 (byte_map_for_k * 8 + b))
```

Each instance is a single bit equality at concrete (k, b).  Discharge via
`assert_norm` per (k, b) — 128 tiny normalization steps, each ground.

Then write the bridge as a regular Z3 `assert (forall i. …)` letting Z3
instantiate the per-byte facts.  Z3 has to:
- For each i with i%16<5, compute c_byte(i) and j(i) (both closed-form arithmetic).
- Match the per-byte lemma at concrete c_byte(i).
- Check arithmetic: `byte_map[c_byte(i)] * 8 + j(i)%8 = (i/16)*5 + i%16`.

This separates the "expensive" 16-byte case-split into 16 cheap lemmas
that get linked by Z3 via instantiation, rather than asking
`assert_norm` to expand all 16 cases × 256 i values.

### Alternative (if Z3 instantiation struggles)

Per-`(i/16)` match with 16 explicit arms, each doing `assert_norm
(forall_n 16 (fun b -> …))` at concrete k.  This was previously warned
against (see prior sprint's "Approaches that DO NOT work") for the
*symbolic-k* version.  But with EXPLICIT match on concrete k=0,1,…,15,
the closure substitutes concretely and the prior warning doesn't
apply.  Each forall_n 16 should reduce in <5s.  Total: ~16 SMT queries.

## Time budget

30 min for the per-byte lemma approach.  If it doesn't close in that
time, fall back to the per-lane match.  60 min total cap per
`feedback_proof_debug_budget`.

## Pre-session checklist

- [ ] Create fresh worktree.
- [ ] Confirm `git log` ancestry includes the sprint-2026-05-12 commits
      (`mm256_mullo_epi16_specialized4` fix + helper refactor).
- [ ] Confirm clean baseline: `make check/Libcrux_ml_kem.Vector.Avx2.Serialize.fst`
      rc=0.
- [ ] Read `proofs/agent-status/sprint-2026-05-12-rollup.md` for context.

## Stretch (separate, optional, ~30 min)

- storeu/loadu axiom upstream into `crates/utils/intrinsics/src/avx2_extract.rs`
  ensures (still pending from sprint 2026-05-11; see that prompt for details).
